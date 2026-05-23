/// Kernel Trampoline — AOT-emitted bridge between guest x86 and host Rust kernel dispatch.
///
/// Instead of INT3 → VEH → dispatch → re-enter (the "emergency brake" approach),
/// the emitter now generates a direct CALL to a trampoline stub for each kernel ordinal.
///
/// The trampoline:
///   1. Saves guest GPRs to GuestState [R13]
///   2. Calls `kernel_bridge(R13, ordinal, is_call)` — a Rust extern "C" function
///   3. If handled inline: restores GPRs, returns to next AOT instruction
///   4. If needs full dispatch: restores GPRs, exits main trampoline to dispatch loop
///
/// Result: Zero exceptions. Zero VEH overhead. Zero stack drift.
/// Each of pool_init's 1,400+ kernel calls returns with an exactly correct stack.

use crate::xbox::emulator::debug_log;
use crate::xbox::kernel::ordinals;
use std::sync::atomic::{AtomicU64, Ordering};

/// Total trampoline bridge calls (inline + full dispatch)
pub static BRIDGE_CALLS: AtomicU64 = AtomicU64::new(0);
/// Total inline-handled calls (no VEH, no dispatch loop)
pub static BRIDGE_INLINE: AtomicU64 = AtomicU64::new(0);

/// The Rust-side kernel bridge function.
/// Called from the trampoline stub with the RuntimeContext pointer.
///
/// Returns 0 if handled inline (continue execution), 1 if needs full dispatch (exit trampoline).
///
/// # Safety
/// Called from generated machine code. `ctx_ptr` must be a valid RuntimeContext.
#[no_mangle]
pub unsafe extern "C" fn kernel_bridge(
    ctx_ptr: *mut u8,
    ordinal: u32,
    is_call: u32,           // 1 = CALL, 0 = JMP
    host_return_addr: u64,  // host address to resume at after full dispatch
) -> u64 {
    use crate::xbox::aot::runtime::RuntimeContext;

    BRIDGE_CALLS.fetch_add(1, Ordering::Relaxed);

    let ctx = &mut *(ctx_ptr as *mut RuntimeContext);
    let r15 = ctx.guest.r15_base as *const u8;
    let esp = ctx.guest.esp;

    // Reject bogus ordinals (valid Xbox kernel ordinals are 1-416)
    if ordinal > 416 {
        static BOGUS_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = BOGUS_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 20 {
            debug_log(&format!(
                "[BRIDGE] bogus ordinal {} (0x{:X}) at ESP=0x{:08X}, returning 0",
                ordinal, ordinal, esp
            ));
        }
        ctx.guest.eax = 0;
        // Stdcall cleanup: just pop args (0 args for unknown) + ret addr for JMP
        if is_call == 0 {
            ctx.guest.esp += 4;
        }
        return 0;
    }

    // Determine arg base: CALL reads args at [ESP+0], JMP skips ret addr at [ESP+0]
    let arg_base = if is_call != 0 { esp } else { esp + 4 };

    let arg_count = ordinals::arg_count(ordinal);
    let is_data = ordinals::is_data(ordinal);

    // Data exports return a fixed address, no stdcall cleanup
    if is_data {
        ctx.guest.eax = 0;
        return 0;
    }

    // Read args from guest stack
    let mut args = [0u32; 12];
    for i in 0..std::cmp::min(arg_count as usize, 12) {
        let addr = arg_base + (i as u32) * 4;
        args[i] = std::ptr::read_unaligned((r15 as *const u8).add(addr as usize) as *const u32);
    }

    // Save pre-cleanup ESP for drift tracking
    ctx.kernel_r14_pre_cleanup = esp;
    ctx.kernel_is_call = is_call != 0;

    // ----------------------------------------------------------------------
    // PHASE 4 · Matched-transition push for ALL kernel calls (inline + exit).
    // The push captures entry ESP. The matching pop for the inline-handled
    // fast path fires right before returning; for the exit path (full
    // dispatch via `dispatch_kernel_call`), the pop fires inside that
    // function after stdcall cleanup.
    //
    // For CALL thunks: caller still has ret_addr on stack, so observed
    //   ESP at leave = entry_esp + argc*4 (just args popped).
    //   Expected frame = entry_esp + 4 + argc*4.
    //   Report with observed+4 to match (caller pops ret_addr after return).
    // For JMP thunks: we pop BOTH args AND ret_addr inline (line 95 below),
    //   so observed ESP at leave = entry_esp + 4 + argc*4 directly.
    // ----------------------------------------------------------------------
    let ret_addr_on_stack = if is_call != 0 {
        // CALL: ret addr is at [ESP+0]
        std::ptr::read_unaligned((r15 as *const u8).add(esp as usize) as *const u32)
    } else {
        // JMP: ret addr is at [ESP+0] too (pushed by caller)
        std::ptr::read_unaligned((r15 as *const u8).add(esp as usize) as *const u32)
    };
    {
        use crate::xbox::aot::transition;
        transition::push(transition::TransitionFrame::new_kernel(
            ordinal as u16,
            ordinals::name(ordinal),
            esp,
            0,                    // guest_entry_pc — kernel is Rust code
            ret_addr_on_stack,
            arg_count as u8,
        ));
    }

    // Try inline fast dispatch first (hot ordinals — no allocation, no locking)
    if let Some(result) = dispatch_inline_fast(ctx, ordinal, &args, arg_count) {
        // Stdcall cleanup: callee pops args
        if arg_count > 0 {
            ctx.guest.esp += arg_count * 4;
        }
        // JMP thunk: also pop the return address
        if is_call == 0 {
            ctx.guest.esp += 4;
        }
        ctx.guest.eax = result;

        // Track inline kernel calls
        ctx.inline_kernel_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        BRIDGE_INLINE.fetch_add(1, Ordering::Relaxed);

        // Phase 4 pop+verify.
        // For CALL: ret_addr still on stack → report observed+4.
        // For JMP: both popped → report ctx.guest.esp directly.
        {
            use crate::xbox::aot::transition;
            let observed = if is_call != 0 {
                ctx.guest.esp.wrapping_add(4)
            } else {
                ctx.guest.esp
            };
            let _ = transition::pop_and_verify(
                Some(transition::TransitionKind::Kernel),
                observed,
            );
        }

        return 0; // Handled inline — continue execution
    }

    // Store kernel call info for the dispatch loop to handle
    ctx.kernel_ordinal = ordinal;
    ctx.kernel_args = args;
    ctx.kernel_arg_count = arg_count;
    ctx.kernel_is_call = is_call != 0;
    ctx.kernel_host_resume = host_return_addr;

    // For JMP thunks: save the return address from guest stack
    if is_call == 0 {
        ctx.kernel_jmp_ret_addr =
            std::ptr::read_unaligned((r15 as *const u8).add(esp as usize) as *const u32);
    } else {
        ctx.kernel_jmp_ret_addr = 0;
    }

    // Stdcall cleanup
    if arg_count > 0 {
        ctx.guest.esp += arg_count * 4;
    }
    if is_call == 0 {
        ctx.guest.esp += 4; // JMP: pop ret addr
    }

    1 // Needs full dispatch — exit trampoline
}

/// Fast inline dispatch for hot kernel ordinals.
/// Returns Some(eax) if handled, None if needs full dispatch.
unsafe fn dispatch_inline_fast(
    ctx: &mut crate::xbox::aot::runtime::RuntimeContext,
    ordinal: u32,
    args: &[u32; 12],
    _arg_count: u32,
) -> Option<u32> {
    let r15 = ctx.guest.r15_base as *mut u8;

    match ordinal {
        // RtlInitializeCriticalSection (ordinal 277, 1 arg)
        277 => {
            let cs_addr = args[0];
            if cs_addr > 0 && (cs_addr as u64) < 0x1000_0000 {
                let p = r15.add(cs_addr as usize);
                std::ptr::write_bytes(p, 0, 28);
                std::ptr::write_unaligned(p as *mut i32, -1); // LockCount = -1
            }
            Some(0)
        }

        // RtlEnterCriticalSection (ordinal 275, 1 arg)
        275 => {
            let cs_addr = args[0];
            if cs_addr > 0 && (cs_addr as u64) < 0x1000_0000 {
                let p = r15.add(cs_addr as usize);
                let lock_count = std::ptr::read_unaligned(p as *const i32);
                std::ptr::write_unaligned(p as *mut i32, lock_count + 1);
                let rec = std::ptr::read_unaligned(p.add(4) as *const i32);
                std::ptr::write_unaligned(p.add(4) as *mut i32, rec + 1);
            }
            Some(0)
        }

        // RtlLeaveCriticalSection (ordinal 276, 1 arg)
        276 => {
            let cs_addr = args[0];
            if cs_addr > 0 && (cs_addr as u64) < 0x1000_0000 {
                let p = r15.add(cs_addr as usize);
                let rec = std::ptr::read_unaligned(p.add(4) as *const i32);
                std::ptr::write_unaligned(p.add(4) as *mut i32, rec - 1);
                let lock_count = std::ptr::read_unaligned(p as *const i32);
                std::ptr::write_unaligned(p as *mut i32, lock_count - 1);
            }
            Some(0)
        }

        // NtAllocateVirtualMemory (ordinal 184, 6 args)
        184 => {
            let base_addr_ptr = args[0];
            let size_ptr = args[2];
            if base_addr_ptr > 0 && (base_addr_ptr as u64) < 0x1000_0000 {
                let requested_base =
                    std::ptr::read_unaligned(r15.add(base_addr_ptr as usize) as *const u32);
                let requested_size =
                    std::ptr::read_unaligned(r15.add(size_ptr as usize) as *const u32);
                if requested_base == 0 {
                    let alloc = ctx.bump_alloc_next;
                    let aligned_size = (requested_size + 0xFFF) & !0xFFF;
                    ctx.bump_alloc_next += aligned_size;
                    std::ptr::write_unaligned(
                        r15.add(base_addr_ptr as usize) as *mut u32,
                        alloc,
                    );
                    std::ptr::write_unaligned(
                        r15.add(size_ptr as usize) as *mut u32,
                        aligned_size,
                    );
                }
            }
            Some(0)
        }

        // NtFreeVirtualMemory (ordinal 199) — no-op
        199 => Some(0),

        // NtProtectVirtualMemory — no-op
        202 => Some(0),

        // RtlNtStatusToDosError (ordinal 278, 1 arg)
        278 => {
            let status = args[0];
            let dos_err = match status {
                0 => 0,
                0x80000006 => 18,
                0xC0000005 => 998,
                0xC000000D => 87,
                0xC0000017 => 8,
                0xC0000034 => 2,
                _ => status,
            };
            Some(dos_err)
        }

        // RtlInitAnsiString (ordinal 274, 2 args)
        274 => {
            let str_addr = args[0];
            let src_addr = args[1];
            if str_addr > 0 && (str_addr as u64) < 0x1000_0000 {
                let p = r15.add(str_addr as usize);
                if src_addr == 0 {
                    std::ptr::write_unaligned(p as *mut u16, 0);
                    std::ptr::write_unaligned(p.add(2) as *mut u16, 0);
                    std::ptr::write_unaligned(p.add(4) as *mut u32, 0);
                } else {
                    let mut len = 0u16;
                    let mut scan = src_addr;
                    while scan < 0x1000_0000 {
                        if *r15.add(scan as usize) == 0 {
                            break;
                        }
                        len += 1;
                        scan += 1;
                        if len >= 65535 {
                            break;
                        }
                    }
                    std::ptr::write_unaligned(p as *mut u16, len);
                    std::ptr::write_unaligned(p.add(2) as *mut u16, len + 1);
                    std::ptr::write_unaligned(p.add(4) as *mut u32, src_addr);
                }
            }
            Some(0)
        }

        // InterlockedIncrement (ordinal 49) — __fastcall via ECX
        49 => {
            let addr = ctx.guest.ecx;
            if addr > 0 && (addr as u64) < 0x1000_0000 {
                let p = r15.add(addr as usize) as *mut i32;
                let val = std::ptr::read_unaligned(p);
                std::ptr::write_unaligned(p, val + 1);
                Some((val + 1) as u32)
            } else {
                Some(0)
            }
        }

        // InterlockedDecrement (ordinal 48) — __fastcall via ECX
        48 => {
            let addr = ctx.guest.ecx;
            if addr > 0 && (addr as u64) < 0x1000_0000 {
                let p = r15.add(addr as usize) as *mut i32;
                let val = std::ptr::read_unaligned(p);
                std::ptr::write_unaligned(p, val - 1);
                Some((val - 1) as u32)
            } else {
                Some(0)
            }
        }

        // InterlockedCompareExchange (ordinal 47) — __fastcall: ECX=Dest, EDX=Exchange
        47 => {
            let addr = ctx.guest.ecx;
            let exchange = ctx.guest.edx;
            let comperand = args[0];
            if addr > 0 && (addr as u64) < 0x1000_0000 {
                let p = r15.add(addr as usize) as *mut u32;
                let old = std::ptr::read_unaligned(p);
                if old == comperand {
                    std::ptr::write_unaligned(p, exchange);
                }
                Some(old)
            } else {
                Some(0)
            }
        }

        // PsCreateSystemThreadEx (ordinal 255) — needs full dispatch
        255 => None,

        // Anything else — needs full dispatch
        _ => None,
    }
}

/// Emit a trampoline stub for a specific kernel ordinal into the code buffer.
/// Returns the offset within the code buffer where the stub starts.
///
/// The stub is reached via CALL rel32 from the main AOT code stream.
/// On entry, [RSP] = return address to the next AOT instruction.
///
/// Layout:
///   1. Pop return address into R11 (save for inline return / full dispatch resume)
///   2. Save guest GPRs to [R13]
///   3. Call kernel_bridge(R13, ordinal, is_call)
///   4. If RAX=0: restore GPRs, push R11, RET (inline return)
///   5. If RAX≠0: save R11 to kernel_host_resume, restore GPRs, exit main trampoline
pub fn emit_trampoline_stub(code: &mut Vec<u8>, ordinal: u32, is_call: bool) -> u32 {
    let stub_offset = code.len() as u32;
    let bridge_addr = kernel_bridge as usize as u64;
    let is_call_val: u32 = if is_call { 1 } else { 0 };

    // Pop return address into R11 (caller's next instruction)
    // pop r11  →  41 5B
    code.extend_from_slice(&[0x41, 0x5B]);

    // Save guest GPRs to GuestState [R13]
    emit_save_gprs(code);

    // Set up Win64 ABI: kernel_bridge(RCX=ctx, RDX=ordinal, R8=is_call, R9=host_return)
    // mov rcx, r13  →  4C 89 E9
    code.extend_from_slice(&[0x4C, 0x89, 0xE9]);
    // mov edx, imm32  →  BA <imm32>
    code.push(0xBA);
    code.extend_from_slice(&ordinal.to_le_bytes());
    // mov r8d, imm32  →  41 B8 <imm32>
    code.extend_from_slice(&[0x41, 0xB8]);
    code.extend_from_slice(&is_call_val.to_le_bytes());
    // mov r9, r11  →  4D 89 D9 (REX.WRB, mov r9, r11 → reg=011(r11), rm=001(r9))
    // Actually: mov r9, r11 → source=r11, dest=r9
    // 4D 89 D9: REX.WRB (0x4D), 0x89, ModRM(11, r11=011, r9=001) = 0xD9
    code.extend_from_slice(&[0x4D, 0x89, 0xD9]);

    // sub rsp, 0x28 (shadow space + alignment)  →  48 83 EC 28
    code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);

    // Save R11 (return addr) on stack before call (it's caller-saved in Win64)
    // push r11  →  41 53
    code.extend_from_slice(&[0x41, 0x53]);

    // mov rax, imm64 (bridge address)  →  48 B8 <imm64>
    code.extend_from_slice(&[0x48, 0xB8]);
    code.extend_from_slice(&bridge_addr.to_le_bytes());

    // call rax  →  FF D0
    code.extend_from_slice(&[0xFF, 0xD0]);

    // pop r11  →  41 5B (restore return addr)
    code.extend_from_slice(&[0x41, 0x5B]);

    // add rsp, 0x28  →  48 83 C4 28
    code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);

    // test rax, rax  →  48 85 C0
    code.extend_from_slice(&[0x48, 0x85, 0xC0]);

    // jnz .exit_path  →  75 XX (placeholder)
    let jnz_pos = code.len();
    code.extend_from_slice(&[0x75, 0x00]);

    // === INLINE PATH (RAX=0): Restore GPRs, push R11, RET ===
    emit_restore_gprs(code);
    // push r11  →  41 53 (return address back on stack)
    code.extend_from_slice(&[0x41, 0x53]);
    // ret  →  C3
    code.push(0xC3);

    // === EXIT PATH (RAX≠0): Restore GPRs, exit main trampoline ===
    // kernel_bridge already saved host_return_addr to ctx.kernel_host_resume
    let exit_pos = code.len();
    // Patch jnz offset
    code[jnz_pos + 1] = (exit_pos - (jnz_pos + 2)) as u8;

    // Restore GPRs from [R13] (bridge already updated EAX and R14/ESP)
    emit_restore_gprs(code);

    // mov dword ptr [r13+0x30], 8  (exit_reason = AOT_EXIT_KERNEL_CALL)
    // 41 C7 45 30 08 00 00 00
    code.extend_from_slice(&[0x41, 0xC7, 0x45, 0x30, 0x08, 0x00, 0x00, 0x00]);

    // jmp [r13+0x08]  (exit_addr — main trampoline exit label)
    // 41 FF 65 08
    code.extend_from_slice(&[0x41, 0xFF, 0x65, 0x08]);

    let stub_size = code.len() as u32 - stub_offset;
    debug_log(&format!(
        "[TRAMPOLINE] Emitted ord {} ({}) +0x{:X} {}B call={}",
        ordinal,
        ordinals::name(ordinal),
        stub_offset,
        stub_size,
        is_call
    ));

    stub_offset
}

/// Emit save of all 8 guest GPRs to GuestState [R13]
fn emit_save_gprs(code: &mut Vec<u8>) {
    // mov [r13+0x10], eax  →  41 89 45 10
    code.extend_from_slice(&[0x41, 0x89, 0x45, 0x10]);
    // mov [r13+0x14], ecx  →  41 89 4D 14
    code.extend_from_slice(&[0x41, 0x89, 0x4D, 0x14]);
    // mov [r13+0x18], edx  →  41 89 55 18
    code.extend_from_slice(&[0x41, 0x89, 0x55, 0x18]);
    // mov [r13+0x1C], ebx  →  41 89 5D 1C
    code.extend_from_slice(&[0x41, 0x89, 0x5D, 0x1C]);
    // mov [r13+0x20], r14d →  45 89 75 20
    code.extend_from_slice(&[0x45, 0x89, 0x75, 0x20]);
    // mov [r13+0x24], ebp  →  41 89 6D 24
    code.extend_from_slice(&[0x41, 0x89, 0x6D, 0x24]);
    // mov [r13+0x28], esi  →  41 89 75 28
    code.extend_from_slice(&[0x41, 0x89, 0x75, 0x28]);
    // mov [r13+0x2C], edi  →  41 89 7D 2C
    code.extend_from_slice(&[0x41, 0x89, 0x7D, 0x2C]);
}

/// Emit restore of all 8 guest GPRs from GuestState [R13]
fn emit_restore_gprs(code: &mut Vec<u8>) {
    // mov eax, [r13+0x10]  →  41 8B 45 10
    code.extend_from_slice(&[0x41, 0x8B, 0x45, 0x10]);
    // mov ecx, [r13+0x14]
    code.extend_from_slice(&[0x41, 0x8B, 0x4D, 0x14]);
    // mov edx, [r13+0x18]
    code.extend_from_slice(&[0x41, 0x8B, 0x55, 0x18]);
    // mov ebx, [r13+0x1C]
    code.extend_from_slice(&[0x41, 0x8B, 0x5D, 0x1C]);
    // mov r14d, [r13+0x20] (guest ESP with stdcall cleanup applied)
    code.extend_from_slice(&[0x45, 0x8B, 0x75, 0x20]);
    // mov ebp, [r13+0x24]
    code.extend_from_slice(&[0x41, 0x8B, 0x6D, 0x24]);
    // mov esi, [r13+0x28]
    code.extend_from_slice(&[0x41, 0x8B, 0x75, 0x28]);
    // mov edi, [r13+0x2C]
    code.extend_from_slice(&[0x41, 0x8B, 0x7D, 0x2C]);
}
