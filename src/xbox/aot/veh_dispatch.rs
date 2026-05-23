use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32 as DispatchAtomicU32;

/// Snapshot of original XBE .text section bytes (immune to runtime corruption).
/// Populated after XBE load. Used by VEH handlers to decode guest instructions
/// when live guest memory may have been overwritten through the RAM mirror.
static mut TEXT_SNAPSHOT: Option<TextSnapshot> = None;

pub struct TextSnapshot {
    pub base_va: u32,  // guest virtual address of snapshot start (e.g. 0x11000)
    pub data: Vec<u8>, // original bytes
}

/// Save a snapshot of guest .text bytes (call after XBE load, before execution).
pub fn save_text_snapshot(guest_base: *const u8, va_start: u32, size: u32) {
    let data =
        unsafe { std::slice::from_raw_parts(guest_base.add(va_start as usize), size as usize) }
            .to_vec();
    unsafe {
        TEXT_SNAPSHOT = Some(TextSnapshot {
            base_va: va_start,
            data,
        });
    }
    crate::xbox::emulator::debug_log(&format!(
        "[TEXT-SNAP] Saved {} bytes from 0x{:08X}",
        size, va_start
    ));
}

/// Read guest bytes — prefer snapshot if available, fall back to live memory.
pub fn read_guest_bytes(r15: u64, guest_addr: u32, len: usize) -> Vec<u8> {
    unsafe {
        if let Some(ref snap) = TEXT_SNAPSHOT {
            let off_start = guest_addr.wrapping_sub(snap.base_va) as usize;
            let off_end = off_start + len;
            if off_start < snap.data.len() && off_end <= snap.data.len() {
                return snap.data[off_start..off_end].to_vec();
            }
        }
        // Fall back to live memory
        std::slice::from_raw_parts((r15 + guest_addr as u64) as *const u8, len).to_vec()
    }
}

/// Global stack scan counter — readable from HUD.
static GLOBAL_STACK_SCANS: DispatchAtomicU32 = DispatchAtomicU32::new(0);
// Broad stack scans were useful while bringing up Spider-Man, but at title/Bink
// time they can resurrect stale call-return continuations and keep corrupted
// frames alive forever. Keep exact RET/hash and one-slot recovery paths, but do
// not search arbitrary stack depth for "any compiled address".
const ENABLE_BROAD_STACK_SCAN_RECOVERY: bool = false;
/// One-shot: heap diag already dumped for the 0x16F50 spin loop.
static HEAP_DIAG_DONE: AtomicBool = AtomicBool::new(false);
/// Spin detection for non-exec indirect calls: (guest_addr, target, count).
/// If the same guest address calls the same garbage target >100 times, force exit
/// to prevent infinite loops like _except_handler3 calling corrupted SEH filter pointers.
static IND_STUB_SPIN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static IND_STUB_SPIN_COUNT: DispatchAtomicU32 = DispatchAtomicU32::new(0);

/// Extra stack cleanup for non-executable indirect CALL stubs.
///
/// The indirect handler pushes a synthetic return address before resolving a
/// CALL. If the target is garbage/data, we synthesize an immediate successful
/// return and undo that return-address push. Some Spider-Man virtual methods
/// also have one already-pushed stack argument and use MSVC thiscall callee
/// cleanup (`ret 4`). When the target is non-executable, no real callee runs,
/// so the stub has to perform that argument cleanup too.
#[cfg(windows)]
fn non_exec_indirect_stub_arg_cleanup(guest_addr: u32) -> u32 {
    match guest_addr {
        // Spider-Man scene walker virtual calls. These are MSVC thiscall
        // methods with one stack argument; when the vtable target is corrupt
        // and we synthesize a return, mirror the callee's `ret 4` cleanup.
        0x000F_2AC1 | 0x000F_2B25 | 0x000F_2BA7 | 0x000F_2BD5 => 4,
        // Spider-Man active-scene update callbacks:
        //   0x000DD688: call dword ptr [[scene+0x50]+0x3C]
        //   0x000DD691: call dword ptr [[scene+0x54]+0x3C]
        //   0x0010C37D: call dword ptr [vtable+0x120]
        //
        // Each call site pushes one stack argument before a thiscall-style
        // virtual method. If the object/vtable is not wired yet and we no-op
        // the null target, mirror the missing callee's `ret 4` cleanup.
        0x000D_D688 | 0x000D_D691 | 0x0010_C37D => 4,
        // Spider-Man entity/scene optional callbacks reached after the
        // post-wait menu action. These vtable slots can be NULL while missing
        // optional entities are reported via GameAssert. The real methods are
        // callee-cleanup callbacks; if the emulator stubs a NULL target without
        // matching that cleanup, the next CRT heap RET pops stale arguments as
        // return addresses and falls into KeBugCheck(0xC0000144).
        0x0016_2140 => 4, // push edi; call [eax+0x100]
        0x0016_2164 => 8, // push 1; push 8; call [edx+0x104]
        0x0010_6B9A | 0x0010_6BCB | 0x0010_6C15 => 4,
        _ => 0,
    }
}

#[cfg(windows)]
fn non_exec_indirect_stub_allows_repeat(guest_addr: u32, target: u32) -> bool {
    match (guest_addr, target) {
        // Spider-Man scene update optional callback:
        //   0x000F144F: call dword ptr [[esi+0x0C]+0x40]
        //
        // On the post-start/menu path the object at scene+0x0C can be present
        // while its vtable is still NULL. Returning EAX=0 is the right no-op
        // behavior, but this callback legitimately fires once per frame while
        // the guest waits on loader/demo state. Do not let the generic garbage
        // function-pointer spin detector turn that into a fatal worker exit.
        (0x000F_144F, 0x0000_0000) | (0x000F_144F, 0x0000_0001) => true,
        // Spider-Man menu tail render callback:
        //   0x000F1380: call dword ptr [[esi+0x50]+0x40]
        //
        // This path is reached from sub_000F1230 after the menu state-3
        // handler is otherwise ready. In the current HLE frame pump the
        // optional object at scene+0x50 can exist without a populated vtable
        // method yet. The native caller has no stack arguments here, so a
        // zero-EAX no-op return is enough; do not classify the per-frame null
        // callback as a fatal spin.
        (0x000F_1380, 0x0000_0000) => true,
        // Spider-Man render-scene list walker. While the Rust bridge is using
        // an inert synthetic scene object, slot 0x38 is deliberately NULL so
        // the callback behaves as an empty per-frame hook. The caller has one
        // stack argument; cleanup is handled by non_exec_indirect_stub_arg_cleanup.
        (0x000F_2BA7, 0x0000_0000) => true,
        // Empty Spider-Man active-scene callbacks can fire every frame while
        // the menu scene is coming up. They are optional observer/update hooks,
        // so no-op them without tripping the generic spin bailout.
        (0x000D_D688, 0x0000_0000) | (0x000D_D691, 0x0000_0000) | (0x0010_C37D, 0x0000_0000) => {
            true
        }
        _ => false,
    }
}

#[cfg(windows)]
fn guest_addr_for_diag(addr: u32) -> bool {
    (addr < 0x2000_0000) || (addr >= 0x8000_0000 && addr < 0xA000_0000)
}

#[cfg(windows)]
fn diag_read_u32(r15: u64, addr: u32) -> u32 {
    if guest_addr_for_diag(addr) {
        unsafe { super::veh::guest_read_u32(r15, addr) }
    } else {
        0
    }
}

#[cfg(windows)]
fn diag_ascii(r15: u64, addr: u32, max_len: usize) -> String {
    if !guest_addr_for_diag(addr) {
        return String::new();
    }

    let mut out = String::new();
    for i in 0..max_len {
        let b = unsafe { super::veh::guest_read_u8(r15, addr.wrapping_add(i as u32)) };
        if b == 0 {
            break;
        }
        if b.is_ascii_graphic() || b == b' ' {
            out.push(b as char);
        } else {
            out.push('.');
        }
    }
    out
}

#[cfg(windows)]
fn log_spidey_scene_vcall(
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_addr: u32,
    target: u32,
    normalized: u32,
) {
    if !matches!(
        guest_addr,
        0x000F_2AC1
            | 0x000F_2B25
            | 0x000F_2BA7
            | 0x000F_2BD5
            | 0x000D_D688
            | 0x000D_D691
            | 0x0010_C37D
            | 0x000F_1380
            | 0x000F_144F
    ) {
        return;
    }

    static SCENE_VCALL_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = SCENE_VCALL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if !(n < 24 || n.is_power_of_two()) {
        return;
    }

    let r15 = context.R15;
    let ecx = context.Rcx as u32;
    let eax = context.Rax as u32;
    let ebx = context.Rbx as u32;
    let edx = context.Rdx as u32;
    let esi = context.Rsi as u32;
    let edi = context.Rdi as u32;
    let ebp = context.Rbp as u32;
    let esp = context.R14 as u32;

    let obj_vt = diag_read_u32(r15, ecx);
    let obj_slot38 = diag_read_u32(r15, obj_vt.wrapping_add(0x38));
    let eax_slot38 = diag_read_u32(r15, eax.wrapping_add(0x38));
    let ebx_word = diag_read_u32(r15, ebx);
    let esp0 = diag_read_u32(r15, esp);
    let esp4 = diag_read_u32(r15, esp.wrapping_add(4));
    let scene_38 = diag_read_u32(r15, esi.wrapping_add(0x38));
    let scene_40 = diag_read_u32(r15, esi.wrapping_add(0x40));
    let scene_44 = diag_read_u32(r15, esi.wrapping_add(0x44));
    let scene_50 = diag_read_u32(r15, esi.wrapping_add(0x50));
    let scene_54 = diag_read_u32(r15, esi.wrapping_add(0x54));
    let scene_40_vt = diag_read_u32(r15, scene_40);
    let scene_44_vt = diag_read_u32(r15, scene_44);
    let scene_50_vt = diag_read_u32(r15, scene_50);
    let scene_54_vt = diag_read_u32(r15, scene_54);
    let scene_40_slot120 = diag_read_u32(r15, scene_40_vt.wrapping_add(0x120));
    let scene_50_slot3c = diag_read_u32(r15, scene_50_vt.wrapping_add(0x3C));
    let scene_50_slot40 = diag_read_u32(r15, scene_50_vt.wrapping_add(0x40));
    let scene_54_slot3c = diag_read_u32(r15, scene_54_vt.wrapping_add(0x3C));
    let scene_54_slot40 = diag_read_u32(r15, scene_54_vt.wrapping_add(0x40));
    let scene_38_ascii = diag_ascii(r15, esi.wrapping_add(0x38), 48);
    let scene_root = diag_read_u32(r15, esi.wrapping_add(0x28));
    let root_144 = diag_read_u32(r15, scene_root.wrapping_add(0x144));
    let root_154 = diag_read_u32(r15, scene_root.wrapping_add(0x154));
    let root_15c = diag_read_u32(r15, scene_root.wrapping_add(0x15C));
    let root_1e4 = diag_read_u32(r15, scene_root.wrapping_add(0x1E4));
    let root_1e8 = diag_read_u32(r15, scene_root.wrapping_add(0x1E8));
    let scene_144 = diag_read_u32(r15, esi.wrapping_add(0x144));
    let scene_1e4 = diag_read_u32(r15, esi.wrapping_add(0x1E4));
    let scene_1e8 = diag_read_u32(r15, esi.wrapping_add(0x1E8));

    let ecx_ascii = diag_ascii(r15, ecx, 32);
    let edx_ascii = diag_ascii(r15, edx, 32);
    let ebx_word_ascii = diag_ascii(r15, ebx_word, 32);

    veh_log(&format!(
        "[SPIDEY-SCENE-VCALL] #{} site=0x{:08X} target=0x{:08X} normalized=0x{:08X} \
         eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X} esp=0x{:08X} \
         obj_vt=0x{:08X} obj_slot38=0x{:08X} eax_slot38=0x{:08X} ebx_word=0x{:08X} \
         stack=[0x{:08X},0x{:08X}] scene38=0x{:08X} scene38_ascii='{}' \
         scene40=0x{:08X}/vt=0x{:08X}/slot120=0x{:08X} scene44=0x{:08X}/vt=0x{:08X} \
         scene50=0x{:08X}/vt=0x{:08X}/slot3c=0x{:08X}/slot40=0x{:08X} \
         scene54=0x{:08X}/vt=0x{:08X}/slot3c=0x{:08X}/slot40=0x{:08X} \
         scene_root=0x{:08X} root144=0x{:08X} root154=0x{:08X} root15c=0x{:08X} root1e4=0x{:08X} root1e8=0x{:08X} \
         scene144=0x{:08X} scene1e4=0x{:08X} scene1e8=0x{:08X} \
         ecx_ascii='{}' edx_ascii='{}' ebx_word_ascii='{}'",
        n,
        guest_addr,
        target,
        normalized,
        eax,
        ecx,
        edx,
        ebx,
        esi,
        edi,
        ebp,
        esp,
        obj_vt,
        obj_slot38,
        eax_slot38,
        ebx_word,
        esp0,
        esp4,
        scene_38,
        scene_38_ascii,
        scene_40,
        scene_40_vt,
        scene_40_slot120,
        scene_44,
        scene_44_vt,
        scene_50,
        scene_50_vt,
        scene_50_slot3c,
        scene_50_slot40,
        scene_54,
        scene_54_vt,
        scene_54_slot3c,
        scene_54_slot40,
        scene_root,
        root_144,
        root_154,
        root_15c,
        root_1e4,
        root_1e8,
        scene_144,
        scene_1e4,
        scene_1e8,
        ecx_ascii,
        edx_ascii,
        ebx_word_ascii,
    ));
}

/// Get total stack scan count (for HUD display).
pub fn get_stack_scan_count() -> u32 {
    GLOBAL_STACK_SCANS.load(std::sync::atomic::Ordering::Relaxed)
}

/// VEH Dispatch sub-handler — INT3 trap switch [L3-ROUTE]
/// Routes INT3 breakpoints based on TrapType:
///   - Ret: pop guest stack → hash lookup fallback
///   - Indirect: indirect JMP/CALL → resolve target, kernel calls handled inline
///   - System: emitter-resolved kernel thunk → inline kernel dispatch
///   - Error: encode failure → error exit
///
/// Kernel calls are dispatched INLINE in VEH (matching C++ v1):
///   - CALL: args from [R14+0], R14 += arg_count*4, RIP = host_offset+1
///   - JMP:  ret from [R14+0], args from [R14+4], R14 += 4+arg_count*4, resolve ret

#[cfg(windows)]
use super::veh::{sync_guest_from_context, veh_log, VehResult};
#[cfg(windows)]
use crate::xbox::aot::runtime::{
    RuntimeContext, TrapType, AOT_EXIT_ERROR, AOT_EXIT_HALT, AOT_EXIT_KERNEL_CALL,
    AOT_EXIT_SPAWN_THREAD, AOT_EXIT_SYSTEM_TRAP, AOT_EXIT_THREAD_EXIT, AOT_EXIT_UNRESOLVED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuestBreakpointPolicy {
    ContinueOneByte,
    TerminalSystemTrap,
}

fn guest_breakpoint_policy(opcode: u8) -> GuestBreakpointPolicy {
    match opcode {
        0xCC => GuestBreakpointPolicy::TerminalSystemTrap,
        _ => GuestBreakpointPolicy::ContinueOneByte,
    }
}

/// Handle copied privileged x86 port I/O instructions that fault as
/// STATUS_PRIVILEGED_INSTRUCTION before they can reach the INT3 trap path.
#[cfg(windows)]
pub fn handle_privileged_in_out(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &RuntimeContext,
) -> VehResult {
    let rip = context.Rip as u64;
    let host_offset = ctx.host_offset_for_rip(rip);
    let guest_addr = ctx.addr_hash.reverse_lookup(host_offset);
    let opcode = unsafe { *(rip as *const u8) };
    let port = (context.Rdx & 0xFFFF) as u16;

    match opcode {
        0xEC => {
            // IN AL, DX. Return a safe zero byte while preserving the rest of RAX.
            context.Rax &= !0xFF;
            context.Rip = rip + 1;
            static LOG_N: DispatchAtomicU32 = DispatchAtomicU32::new(0);
            let n = LOG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                veh_log(&format!(
                    "[SYSTEM] IN AL,DX port=0x{:04X} -> 0 at guest 0x{:08X}",
                    port, guest_addr
                ));
            }
            VehResult::Handled
        }
        0xEE => {
            // OUT DX, AL. The Xbox hardware port write is ignored by this HLE path.
            let value = (context.Rax & 0xFF) as u8;
            context.Rip = rip + 1;
            static LOG_N: DispatchAtomicU32 = DispatchAtomicU32::new(0);
            let n = LOG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                veh_log(&format!(
                    "[SYSTEM] OUT DX,AL port=0x{:04X} value=0x{:02X} ignored at guest 0x{:08X}",
                    port, value, guest_addr
                ));
            }
            VehResult::Handled
        }
        _ => VehResult::NotHandled,
    }
}

/// Tight stack trace for USB OHCI init window.
/// Fires when ESP is on the worker stack (0x1EFFF000 range) and kernel_calls are 470-500.
/// Logs event kind, guest_pc, ESP, and top 6 stack dwords. Watches for 0x00D134B0.
#[cfg(windows)]
fn trace_stack_window(
    _label: &str,
    _guest_pc: u32,
    _context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    _ctx: &RuntimeContext,
    _extra: &str,
) {
    // Disabled — XMountMUA argc root cause fixed, stack trace noise removed
}

#[cfg(windows)]
fn try_handle_dsound_dynamic_stream_stub(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    code_base: u64,
    host_offset: u32,
    guest_addr: u32,
    target: u32,
) -> Option<VehResult> {
    const DSS_VTABLE_ADDR: u32 = 0x00EA_3100;
    const DSS_STUBS_ADDR: u32 = 0x00EA_3200;
    const DSS_STUBS_END: u32 = DSS_STUBS_ADDR + 0x80;

    if target < DSS_STUBS_ADDR || target >= DSS_STUBS_END || ((target - DSS_STUBS_ADDR) & 3) != 0 {
        return None;
    }

    let slot = (target - DSS_STUBS_ADDR) / 4;
    let esp = context.R14 as u32;
    let this = diag_read_u32(context.R15, esp.wrapping_add(4));
    let p_input = diag_read_u32(context.R15, esp.wrapping_add(8));
    let p_output = diag_read_u32(context.R15, esp.wrapping_add(12));
    let vtable = diag_read_u32(context.R15, this);

    let mut args = [0u32; 8];
    args[0] = this;
    args[1] = p_input;
    args[2] = p_output;
    let guest_mem = context.R15 as *mut u8;
    let (name, cleanup, result) = match slot {
        0 => (
            "AddRef",
            4,
            crate::xbox::apu::dsound::hle_addref(&args, guest_mem),
        ),
        1 => (
            "Release",
            4,
            crate::xbox::apu::dsound::hle_release(&args, guest_mem),
        ),
        // Spider-Man's XDK 4134 stream vtable routes Process through slot +0x10.
        4 => (
            "Process",
            12,
            crate::xbox::apu::dsound::hle_stream_process(&args, guest_mem),
        ),
        // Reached during stream teardown with only `this` pushed by the caller.
        6 => ("Teardown", 4, 0),
        _ => ("Noop", 4, 0),
    };

    let ret_esp = esp.wrapping_add(4).wrapping_add(cleanup);
    context.R14 = ret_esp as u64;
    ctx.guest.esp = ret_esp;
    context.Rax = result as u64;
    context.Rip = code_base + host_offset as u64 + 1;

    static DSOUND_DYN_STREAM_LOG: DispatchAtomicU32 = DispatchAtomicU32::new(0);
    let n = DSOUND_DYN_STREAM_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() || vtable != DSS_VTABLE_ADDR || !matches!(slot, 4) {
        veh_log(&format!(
            "[DSOUND-DYN-VT] #{} site=0x{:08X} target=0x{:08X} slot={} {} this=0x{:08X} vt=0x{:08X} \
             in=0x{:08X} out=0x{:08X} result=0x{:08X} cleanup={} esp 0x{:08X}->0x{:08X}",
            n,
            guest_addr,
            target,
            slot,
            name,
            this,
            vtable,
            p_input,
            p_output,
            result,
            cleanup,
            esp,
            ret_esp
        ));
    }

    Some(VehResult::Handled)
}

/// Handle an INT3 breakpoint in the main code buffer.
/// Looks up the trap info and routes based on trap type.
#[cfg(windows)]
pub fn handle_int3(
    rip: u64,
    code_base: u64,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
) -> VehResult {
    let host_offset = (rip - code_base) as u32;

    // DR0 hardware watchpoint removed — XMountMUA argc root cause fixed.

    let trap_info = ctx.find_trap(host_offset).copied();
    let trap = match trap_info {
        Some(t) => t,
        None => {
            veh_log(&format!(
                "[L3-ROUTE] UNKNOWN INT3 at host_offset=0x{:X} (RIP=0x{:016X})",
                host_offset, rip
            ));
            unsafe {
                sync_guest_from_context(ctx, context);
            }
            ctx.guest.exit_reason = AOT_EXIT_ERROR;
            ctx.guest.exit_guest_addr = 0xBAD10000 + host_offset;
            context.Rip = ctx.guest.exit_addr;
            return VehResult::Handled;
        }
    };

    // D3D section diagnostic
    if trap.guest_addr >= 0x002E_BDA0 && trap.guest_addr <= 0x0030_38F8 {
        crate::rate_log!(
            50,
            "[D3D-TRAP] g=0x{:08X} type={:?} cleanup={} host_off=0x{:X} R14=0x{:08X}",
            trap.guest_addr,
            trap.trap_type,
            trap.ret_cleanup,
            host_offset,
            context.R14 as u32
        );
    }

    unsafe {
        sync_guest_from_context(ctx, context);
    }

    match trap.trap_type {
        TrapType::Ret => {
            return handle_ret(code_base, host_offset, context, ctx, &trap);
        }

        TrapType::Indirect => {
            ctx.indirect_dispatches += 1;
            crate::xbox::aot::bink_profile::note_ind_trap(trap.guest_addr);
            // Periodic .text integrity check
            if ctx.indirect_dispatches % 500 == 0 {
                check_text_integrity(
                    context.R15,
                    ctx.indirect_dispatches as usize,
                    trap.guest_addr,
                    context.R14 as u32,
                );
            }
            let mut target = resolve_indirect_target_live(ctx, context, trap.guest_addr);
            // If live guest bytes are corrupted (zeroed by game's memset), the decoder
            // produces garbage. Fall back to scanning GPRs for a plausible target.
            if target == 0 || target == trap.guest_addr {
                // Guest bytes corrupted — decoder can't resolve the operand.
                // Scan GPRs for kernel thunk values (0xFFFF0000+ or 0x80000000|ord).
                // Only accept kernel addresses — code/heap addresses are too ambiguous.
                let gprs = [
                    context.Rsi as u32,
                    context.Rdi as u32,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rbx as u32,
                    context.Rbp as u32,
                ];
                for &r in &gprs {
                    if r >= 0xFFFF_0000 || (r >= 0x8000_0000 && r < 0x8000_0200) {
                        target = r;
                        break;
                    }
                }
            }
            // Use trap.ret_cleanup for is_call (set by emitter at compile time).
            // Don't read live guest bytes — they may be overwritten by game code.
            let is_call = trap.ret_cleanup == 1;

            let doom_column_probe_name = if is_call
                && super::oovpa::XBE_ENTRY.load(std::sync::atomic::Ordering::Relaxed) == 0x0003_E5AB
            {
                match trap.guest_addr {
                    0x0002_D530 => Some("DOOM_SPAN_CALL_INDIRECT_BEFORE"),
                    0x0002_F165 => Some("DOOM_COLUMN_CALL_INDIRECT_BEFORE"),
                    0x0003_9B10 => Some("DOOM_WALL_COLUMN_CALL_MID_BEFORE"),
                    0x0003_9BCF => Some("DOOM_WALL_COLUMN_CALL_TOP_BEFORE"),
                    0x0003_9C91 => Some("DOOM_WALL_COLUMN_CALL_BOTTOM_BEFORE"),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(probe_name) = doom_column_probe_name {
                if probe_name.starts_with("DOOM_SPAN_") {
                    crate::xbox::aot::oovpa::oovpa_hle::doom_span_call_tap(
                        probe_name,
                        context.R15 as *mut u8,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                    );
                } else {
                    crate::xbox::aot::oovpa::oovpa_hle::doom_column_call_tap(
                        probe_name,
                        context.R15 as *mut u8,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                    );
                }
            }

            // Indirect dispatch diagnostics
            if target == 0 {
                crate::rate_log!(
                    100,
                    "[ZERO-TARGET] guest 0x{:08X}: ESI=0x{:08X} EDI=0x{:08X} EBX=0x{:08X}",
                    trap.guest_addr,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    context.Rbx as u32
                );
            }
            crate::rate_log!(
                20,
                "[L3-ROUTE] INDIRECT {} at guest 0x{:08X} -> target 0x{:08X} esp=0x{:08X}",
                if is_call { "CALL" } else { "JMP" },
                trap.guest_addr,
                target,
                context.R14 as u32
            );

            record_event(
                trap.guest_addr,
                target,
                if is_call { 1 } else { 2 },
                ctx.guest.esp,
            );

            // Thunk repair: if target=0 and the guest instruction loads from the
            // kernel thunk table range, the game's runtime code overwrote the resolved
            // thunk entry. Repair from the ThunkTable snapshot.
            let target = if target == 0 {
                ctx.thunk_table
                    .repair_at_site(ctx.guest_mem_base as *const u8, trap.guest_addr)
                    .unwrap_or(target)
            } else {
                target
            };

            // Kernel call detection:
            // 1. Standard thunk: target >= 0xFFFF0000 (thunk table decrypted to 0xFFFF0000+ordinal)
            // 2. Mirror thunk: target = 0x80000000|ordinal (XPP section function pointers)
            //    These are kernel ordinals stored as 0x80000000|ord in the thunk/import table.
            //    The indirect call reads the pointer, gets 0x800000BB, which is ordinal 187.
            let is_kernel =
                target >= 0xFFFF_0000 || (target >= 0x8000_0000 && target < 0x8000_0200);
            if is_kernel {
                // Convert 0x80000000|ord to 0xFFFF0000+ord for the kernel handler
                let kernel_target = if target >= 0xFFFF_0000 {
                    target
                } else {
                    let ord = target & 0xFFFF;
                    0xFFFF_0000 + ord
                };
                // For CALL traps the INT3 replaced the guest CALL instruction, so
                // the CPU never pushed the architectural return address. Preserve it
                // explicitly for diagnostics/transition accounting and for any caller
                // that needs the guest continuation PC. Do not write this to the guest
                // stack: the safe dispatch loop resumes by host RIP for CALLs.
                let call_guest_ret_addr = if is_call {
                    trap.guest_addr
                        .wrapping_add(guest_instr_len(ctx, trap.guest_addr))
                } else {
                    0
                };
                return handle_kernel_exit(
                    context,
                    ctx,
                    code_base,
                    host_offset,
                    kernel_target,
                    is_call,
                    call_guest_ret_addr,
                );
            }

            // Non-kernel indirect: push ret addr for CALL, then try hash lookup
            if is_call {
                let ret_addr = trap.guest_addr + guest_instr_len(ctx, trap.guest_addr);
                // The Windows CONTEXT is the live source of truth inside VEH.
                // ctx.guest.esp is only synchronized when we leave the trampoline;
                // using it here can push the indirect-call return address onto a
                // stale guest stack and corrupt callee-saved restores.
                let new_esp = (context.R14 as u32).wrapping_sub(4);
                unsafe {
                    super::veh::guest_write_u32(context.R15, new_esp, ret_addr);
                }
                context.R14 = new_esp as u64;

                trace_stack_window(
                    "IND-CALL",
                    trap.guest_addr,
                    context,
                    ctx,
                    &format!("target=0x{:08X} pushed_ret=0x{:08X}", target, ret_addr),
                );

                // R12 shadow push DISABLED — emit_inline_guest_ret doesn't pop R12.
                // Guest stack has the return address; hash lookup handles dispatch.
            }

            // Try hash lookup for target
            let normalized = normalize_addr(target);
            if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                context.Rip = code_base + host_off as u64;
                return VehResult::Handled;
            }

            // Spider-Man post-start/menu guard. sub_002A45B0 ends with
            // `jmp dword ptr [edx+0x18]`; after MENU.xbs loads, this slot can
            // contain the poison float 0x3DCCCCCD. The next guest instruction
            // is the function's `ret` at 0x002A45F4, so treating the missing
            // callback as absent is the least invasive recovery.
            let target_in_exec = ctx
                .exec_ranges
                .iter()
                .any(|&(s, e)| normalized >= s && normalized < e);
            if !is_call && trap.guest_addr == 0x002A_45F1 && !target_in_exec {
                static SPIDEY_POSTFRAME_SKIP: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SPIDEY_POSTFRAME_SKIP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() {
                    let r15 = context.R15;
                    let rd = |addr: u32| -> u32 {
                        if addr != 0 && addr < 0x2000_0000 {
                            unsafe { super::veh::guest_read_u32(r15, addr) }
                        } else {
                            0
                        }
                    };
                    let obj = rd(0x0072_6F04);
                    let obj_vt = rd(obj);
                    let obj_cb04 = rd(obj_vt.wrapping_add(0x04));
                    let obj_cb18 = rd(obj_vt.wrapping_add(0x18));
                    let edx = context.Rdx as u32;
                    let edx_cb18 = rd(edx.wrapping_add(0x18));
                    let flags = rd(0x0072_6F00);
                    let prev_time = rd(0x0072_6EF8);
                    let threshold = rd(0x0072_6EFC);
                    let esp = context.R14 as u32;
                    let st0 = rd(esp);
                    let st4 = rd(esp.wrapping_add(4));
                    let st8 = rd(esp.wrapping_add(8));
                    veh_log(&format!(
                        "[SPIDEY-POSTFRAME-SKIP] #{} invalid tail-jmp target=0x{:08X} normalized=0x{:08X} \
                         obj=0x{:08X} vt=0x{:08X} cb04=0x{:08X} cb18=0x{:08X} edx=0x{:08X} edx+18=0x{:08X} \
                         flags_word=0x{:08X} time_bits=(0x{:08X},0x{:08X}) eax=0x{:08X} ecx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X} esp=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X}] -> fallthrough ret 0x002A45F4",
                        n,
                        target,
                        normalized,
                        obj,
                        obj_vt,
                        obj_cb04,
                        obj_cb18,
                        edx,
                        edx_cb18,
                        flags,
                        prev_time,
                        threshold,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rbx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        context.Rbp as u32,
                        esp,
                        st0,
                        st4,
                        st8
                    ));
                }

                let fallthrough = 0x002A_45F4;
                if let Some(host_off) = ctx.addr_hash.lookup(fallthrough) {
                    context.Rip = code_base + host_off as u64;
                    return VehResult::Handled;
                }
                ctx.guest.esp = context.R14 as u32;
                ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
                ctx.guest.exit_guest_addr = fallthrough;
                context.Rip = ctx.guest.exit_addr;
                return VehResult::Handled;
            }

            // Target not in compiled code.
            if is_call {
                // If target is in an executable section, exit for rescue compilation.
                // The guest stack already has the return address pushed (lines above),
                // so the callee can execute and RET normally after rescue_emit.
                let in_exec = ctx
                    .exec_ranges
                    .iter()
                    .any(|&(s, e)| normalized >= s && normalized < e);
                let in_data = ctx
                    .data_ranges
                    .iter()
                    .any(|&(s, e)| normalized >= s && normalized < e);
                if in_exec && !in_data {
                    crate::rate_log!(
                        30,
                        "[L3-RESCUE] CALL to uncompiled 0x{:08X} from guest 0x{:08X} — rescue",
                        normalized,
                        trap.guest_addr
                    );
                    ctx.guest.esp = context.R14 as u32;
                    ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
                    ctx.guest.exit_guest_addr = normalized;
                    context.Rip = ctx.guest.exit_addr;
                    return VehResult::Handled;
                }

                if trap.guest_addr == 0x0003_AC34 && target == 0 {
                    static SPIDEY_CB_NULL_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = SPIDEY_CB_NULL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 10 || n.is_power_of_two() {
                        let ret_addr = unsafe { *((context.R15 + context.R14) as *const u32) };
                        let upd2_object = unsafe { *((context.R15 + 0x003F_7D90) as *const u32) };
                        let upd2_vtable = if upd2_object != 0 && upd2_object < 0x2000_0000 {
                            unsafe { *((context.R15 + upd2_object as u64) as *const u32) }
                        } else {
                            0
                        };
                        let upd2_list_start = if upd2_object != 0 && upd2_object < 0x2000_0000 {
                            unsafe { *((context.R15 + upd2_object as u64 + 0x08) as *const u32) }
                        } else {
                            0
                        };
                        let upd2_list_end = if upd2_object != 0 && upd2_object < 0x2000_0000 {
                            unsafe { *((context.R15 + upd2_object as u64 + 0x0C) as *const u32) }
                        } else {
                            0
                        };
                        let first_entry = if upd2_list_start != 0 && upd2_list_start < 0x2000_0000 {
                            unsafe { *((context.R15 + upd2_list_start as u64) as *const u32) }
                        } else {
                            0
                        };
                        let first_vtable = if first_entry != 0 && first_entry < 0x2000_0000 {
                            unsafe { *((context.R15 + first_entry as u64) as *const u32) }
                        } else {
                            0
                        };
                        let first_slot20 = if first_vtable != 0 && first_vtable < 0x0100_0000 {
                            unsafe { *((context.R15 + first_vtable as u64 + 0x20) as *const u32) }
                        } else {
                            0
                        };
                        let entry_object = context.Rcx as u32;
                        let entry_vtable = if entry_object != 0 && entry_object < 0x2000_0000 {
                            unsafe { *((context.R15 + entry_object as u64) as *const u32) }
                        } else {
                            0
                        };
                        veh_log(&format!(
                            "[SPIDEY-CB-GUARD] 0x3AC34 null callback target skipped #{} \
                             ret=0x{:08X} ECX=0x{:08X} vt=0x{:08X} ESI=0x{:08X} EDI=0x{:08X} EBX=0x{:08X} upd2=0x{:08X} upd2_vt=0x{:08X} upd2_list=[0x{:08X}..0x{:08X}] first=0x{:08X} first_vt=0x{:08X} first_slot20=0x{:08X}",
                            n,
                            ret_addr,
                            context.Rcx as u32,
                            entry_vtable,
                            context.Rsi as u32,
                            context.Rdi as u32,
                            context.Rbx as u32,
                            upd2_object,
                            upd2_vtable,
                            upd2_list_start,
                            upd2_list_end,
                            first_entry,
                            first_vtable,
                            first_slot20
                        ));
                    }

                    let ret_esp = (context.R14 as u32).wrapping_add(4);
                    context.R14 = ret_esp as u64;
                    context.Rax = 0;
                    context.Rsi = (context.Rdi as u32).wrapping_sub(4) as u64;
                    context.Rip = code_base + host_offset as u64 + 1;
                    return VehResult::Handled;
                }

                // Non-executable target (e.g. uninitialized D3D vtable): stub it out.
                // Undo the CALL setup and return EAX=0 (S_OK).
                log_spidey_scene_vcall(context, trap.guest_addr, target, normalized);

                if let Some(result) = try_handle_dsound_dynamic_stream_stub(
                    context,
                    ctx,
                    code_base,
                    host_offset,
                    trap.guest_addr,
                    target,
                ) {
                    return result;
                }

                if trap.guest_addr == 0x0010_C37D && target == 0 && (context.Rsi as u32) == 0 {
                    let arg_cleanup = non_exec_indirect_stub_arg_cleanup(trap.guest_addr);
                    let ret_esp = (context.R14 as u32)
                        .wrapping_add(4)
                        .wrapping_add(arg_cleanup);
                    let epilogue = crate::xbox::emulator::normalize_guest_addr(0x0010_C3B9);
                    if let Some(host_off) = ctx.addr_hash.lookup(epilogue) {
                        static SPIDEY_10C37D_NULL_THIS_SKIP: DispatchAtomicU32 =
                            DispatchAtomicU32::new(0);
                        let n = SPIDEY_10C37D_NULL_THIS_SKIP
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 16 || n.is_power_of_two() {
                            veh_log(&format!(
                                "[SPIDEY-10C37D-NULL-THIS-SKIP] #{} target=0x{:08X} esp 0x{:08X}->0x{:08X} epilogue=0x0010C3B9",
                                n,
                                target,
                                context.R14 as u32,
                                ret_esp
                            ));
                        }
                        context.R14 = ret_esp as u64;
                        context.Rax = 0;
                        context.Rip = code_base + host_off as u64;
                        return VehResult::Handled;
                    }
                }

                // Spin detection: if the same (guest_addr, target) pair fires >100 times,
                // the caller is looping on a corrupt function pointer (e.g. _except_handler3
                // calling garbage SEH filter 0x00000007). Force exit instead of looping 49M times.
                if !non_exec_indirect_stub_allows_repeat(trap.guest_addr, target) {
                    let key = ((trap.guest_addr as u64) << 32) | (target as u64);
                    let prev_key = IND_STUB_SPIN.load(std::sync::atomic::Ordering::Relaxed);
                    if key == prev_key {
                        let count =
                            IND_STUB_SPIN_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if count >= 100 {
                            veh_log(&format!(
                                "[L3-STUB] SPIN BAILOUT: guest 0x{:08X} called garbage target 0x{:08X} {} times — forcing exit",
                                trap.guest_addr, target, count
                            ));
                            // Undo CALL and force worker exit
                            let arg_cleanup = non_exec_indirect_stub_arg_cleanup(trap.guest_addr);
                            let ret_esp = (context.R14 as u32)
                                .wrapping_add(4)
                                .wrapping_add(arg_cleanup);
                            context.R14 = ret_esp as u64;
                            ctx.guest.esp = ret_esp;
                            ctx.guest.exit_reason = AOT_EXIT_ERROR;
                            ctx.guest.exit_guest_addr = trap.guest_addr;
                            context.Rip = ctx.guest.exit_addr;
                            return VehResult::Handled;
                        }
                    } else {
                        IND_STUB_SPIN.store(key, std::sync::atomic::Ordering::Relaxed);
                        IND_STUB_SPIN_COUNT.store(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }

                let arg_cleanup = non_exec_indirect_stub_arg_cleanup(trap.guest_addr);
                let ret_esp = (context.R14 as u32)
                    .wrapping_add(4)
                    .wrapping_add(arg_cleanup);
                context.R14 = ret_esp as u64;

                context.Rax = 0; // EAX = 0 (S_OK / success stub)

                // Resume after the INT3 (skip the trap)
                context.Rip = code_base + host_offset as u64 + 1;

                trace_stack_window(
                    "IND-STUB",
                    trap.guest_addr,
                    context,
                    ctx,
                    &format!(
                        "STUBBED target=0x{:08X} EAX=0 cleanup={}",
                        target, arg_cleanup
                    ),
                );
                crate::rate_log!(
                    30,
                    "[L3-STUB] Stubbed CALL to 0x{:08X} from guest 0x{:08X} (returning EAX=0, non-exec, cleanup={})",
                    target,
                    trap.guest_addr,
                    arg_cleanup
                );

                // One-shot diagnostic: dump registers for vtable dispatch spin at 0x44E54
                if trap.guest_addr == 0x0004_4E54 && target == 0xBF80_0000 {
                    static DIAG_44E54: std::sync::atomic::AtomicBool =
                        std::sync::atomic::AtomicBool::new(false);
                    if !DIAG_44E54.swap(true, std::sync::atomic::Ordering::Relaxed) {
                        let mem_base = ctx.guest_mem_base;
                        let ecx = context.Rcx as u32;
                        let eax = context.Rax as u32;
                        let edi = context.Rdi as u32;
                        let esi = context.Rsi as u32;
                        let upd1 = unsafe { *(mem_base.add(0x3F87A8) as *const u32) };
                        let zp10 = unsafe { *(mem_base.add(0x10) as *const u32) };
                        let zp14 = unsafe { *(mem_base.add(0x14) as *const u32) };
                        // Read the object pointer and vtable
                        let obj_ptr = if edi < 0x2000_0000 {
                            unsafe { *(mem_base.add(edi as usize) as *const u32) }
                        } else {
                            0xDEAD
                        };
                        let vtable = if obj_ptr < 0x2000_0000 {
                            unsafe { *(mem_base.add(obj_ptr as usize) as *const u32) }
                        } else {
                            0xDEAD
                        };
                        let vt_slot1 = if vtable < 0x2000_0000 && vtable > 0 {
                            unsafe { *(mem_base.add(vtable as usize + 4) as *const u32) }
                        } else {
                            0xDEAD
                        };
                        veh_log(&format!(
                            "[DIAG-44E54] ECX=0x{:08X} EAX=0x{:08X} EDI=0x{:08X} ESI=0x{:08X} \
                             Upd1=[0x3F87A8]=0x{:08X} zp[0x10]=0x{:08X} zp[0x14]=0x{:08X} \
                             [EDI]=0x{:08X} [[EDI]]=0x{:08X} [vtable+4]=0x{:08X}",
                            ecx, eax, edi, esi, upd1, zp10, zp14, obj_ptr, vtable, vt_slot1
                        ));
                    }
                }

                // One-shot heap diagnostic for the 0x16F50 spin loop (null callback retry).
                // Dumps the NT heap state to understand why RtlAllocateHeap(0x8000) returns NULL.
                if trap.guest_addr == 0x0001_6F50
                    && target == 0
                    && !HEAP_DIAG_DONE.swap(true, std::sync::atomic::Ordering::Relaxed)
                {
                    let r15 = context.R15;
                    let rd =
                        |addr: u32| -> u32 { unsafe { super::veh::guest_read_u32(r15, addr) } };

                    // process_heap pointer at [0x7E18B0]
                    let heap = rd(0x7E_18B0);
                    // __active_heap at [0x7E1C4C]
                    let active_heap = rd(0x7E_1C4C);
                    // _crtheap at [0x7E1584]
                    let crtheap = rd(0x7E_1584);

                    veh_log(&format!(
                        "[HEAP-DIAG] spin at 0x16F50: process_heap=0x{:08X} _crtheap=0x{:08X} __active_heap={}",
                        heap, crtheap, active_heap
                    ));

                    if heap != 0 && heap < 0x2000_0000 {
                        // Heap fields (Xbox offsets: +8 from standard NT due to HEAP_ENTRY header)
                        let signature = rd(heap + 0x10);
                        let flags = rd(heap + 0x14);
                        let force_flags = rd(heap + 0x18);
                        let vm_threshold = rd(heap + 0x1C);
                        let total_free = rd(heap + 0x30);
                        let seg0_base = rd(heap + 0x60);
                        // FreeLists[0] at heap+0x180
                        let fl0_flink = rd(heap + 0x180);
                        let fl0_blink = rd(heap + 0x184);
                        let fl0_head = heap + 0x180;
                        let fl0_empty = fl0_flink == fl0_head;

                        veh_log(&format!(
                            "[HEAP-DIAG] sig=0x{:08X} flags=0x{:X} force=0x{:X} VMThreshold={} TotalFree={} seg0=0x{:08X}",
                            signature, flags, force_flags, vm_threshold, total_free, seg0_base
                        ));
                        veh_log(&format!(
                            "[HEAP-DIAG] FreeLists[0]: flink=0x{:08X} blink=0x{:08X} empty={}",
                            fl0_flink, fl0_blink, fl0_empty
                        ));

                        // If not empty, check the largest block (at Blink)
                        if !fl0_empty && fl0_blink > 8 {
                            let entry = fl0_blink - 8; // HEAP_ENTRY is 8 bytes before the list node
                            let size_units = rd(entry) & 0xFFFF; // Size in lower u16
                            let prev_size = (rd(entry) >> 16) & 0xFFFF;
                            let entry_flags = rd(entry + 4) & 0xFF;
                            veh_log(&format!(
                                "[HEAP-DIAG] Largest free: entry=0x{:08X} size={} units ({}B) prev={} flags=0x{:02X}",
                                entry, size_units, size_units as u64 * 16, prev_size, entry_flags
                            ));
                        }

                        // Walk first 4 entries of FreeLists[0]
                        let mut cursor = fl0_flink;
                        for i in 0..4u32 {
                            if cursor == fl0_head || cursor < 8 {
                                break;
                            }
                            let entry = cursor - 8;
                            let size_units = rd(entry) & 0xFFFF;
                            let next = rd(cursor);
                            veh_log(&format!(
                                "[HEAP-DIAG] FL0[{}]: entry=0x{:08X} size={} units ({}B) next=0x{:08X}",
                                i, entry, size_units, size_units as u64 * 16, next
                            ));
                            cursor = next;
                        }

                        // Check heap lock variable at heap+0x580
                        let lock_var = rd(heap + 0x580);
                        let lock_count = if lock_var != 0 && lock_var < 0x2000_0000 {
                            rd(lock_var + 0x10) as i32 // LockCount at CS+0x10
                        } else {
                            -99
                        };
                        veh_log(&format!(
                            "[HEAP-DIAG] LockVariable=0x{:08X} LockCount={}",
                            lock_var, lock_count
                        ));

                        // Required units for 32KB: (0x8010 + 0x1F) & ~0xF >> 4 = 0x802 = 2050
                        veh_log(&format!(
                            "[HEAP-DIAG] Need ~2050 units for 32KB. VMThreshold={} ({}=ok if >= 2050)",
                            vm_threshold, if vm_threshold >= 2050 { "OK" } else { "TOO LOW" }
                        ));

                        // Raw hex dump at heap base to verify structure offset assumptions
                        for row in 0..16u32 {
                            let off = row * 16;
                            let v0 = rd(heap + off);
                            let v1 = rd(heap + off + 4);
                            let v2 = rd(heap + off + 8);
                            let v3 = rd(heap + off + 12);
                            veh_log(&format!(
                                "[HEAP-DIAG] +0x{:03X}: {:08X} {:08X} {:08X} {:08X}",
                                off, v0, v1, v2, v3
                            ));
                        }
                        // Also dump at +0x180 (FreeLists[0])
                        for row in 0..4u32 {
                            let off = 0x180 + row * 16;
                            let v0 = rd(heap + off);
                            let v1 = rd(heap + off + 4);
                            let v2 = rd(heap + off + 8);
                            let v3 = rd(heap + off + 12);
                            veh_log(&format!(
                                "[HEAP-DIAG] FL+0x{:03X}: {:08X} {:08X} {:08X} {:08X}",
                                off, v0, v1, v2, v3
                            ));
                        }
                    }
                }

                return VehResult::Handled;
            }

            // JMP to unresolvable target — exit for dispatch loop
            veh_log(&format!(
                "[L3-JMP] UNRESOLVED JMP to {} from guest {} esp=0x{:08X}",
                crate::xbox::logging::fmt_guest_addr(normalized),
                crate::xbox::logging::fmt_guest_addr(trap.guest_addr),
                context.R14 as u32
            ));
            ctx.guest.esp = context.R14 as u32;
            ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
            ctx.guest.exit_guest_addr = normalized;
            context.Rip = ctx.guest.exit_addr;
            return VehResult::Handled;
        }

        TrapType::System => {
            ctx.system_traps += 1;
            crate::xbox::aot::bink_profile::note_sys_trap(trap.guest_addr);

            // Check if this is a resolved kernel call (guest_addr >= KERNEL_MAGIC_BASE)
            if trap.guest_addr >= 0xFFFF_0000 {
                let is_call = trap.ret_cleanup == 1; // emitter encodes is_call here
                let call_guest_ret_addr = if is_call {
                    trap.guest_addr
                        .wrapping_add(guest_instr_len(ctx, trap.guest_addr))
                } else {
                    0
                };
                return handle_kernel_exit(
                    context,
                    ctx,
                    code_base,
                    host_offset,
                    trap.guest_addr,
                    is_call,
                    call_guest_ret_addr,
                );
            }

            // Non-kernel system trap — emulate inline (RDTSC, CPUID, etc.)
            return handle_system_inline(context, ctx, code_base, host_offset, &trap);
        }

        TrapType::Error => {
            ctx.system_traps += 1;
            ctx.guest.exit_reason = AOT_EXIT_ERROR;
            ctx.guest.exit_guest_addr = trap.guest_addr;
            context.Rip = ctx.guest.exit_addr;
            return VehResult::Handled;
        }

        TrapType::Rescue => {
            // Encode failure rescue: decode original guest bytes and execute at runtime.
            // Ported from C++ AOT_VEH.cpp "Runtime rescue" path.
            ctx.rescue_count += 1;
            return handle_rescue(context, ctx, code_base, host_offset, &trap);
        }

        TrapType::StackTrace => {
            // P0 diagnostic probe — zero state change, skip INT3 and resume.
            let r14 = context.R14 as u32;
            let r15 = context.R15;
            let r12 = context.R12;
            let rbp = context.Rbp as u32;
            let rax = context.Rax as u32;
            let s0 = if r14 < 0x2000_0000 {
                unsafe { super::veh::guest_read_u32(r15, r14) }
            } else {
                0xDEAD
            };
            // CALL-BOUNDARY LOGGER (2026-04-22): for guest PCs in the
            // probed regions, log full state unconditionally (cap first
            // 300 events). Other StackTrace probes still rate-limited.
            let in_font_helper = (0x0002_6220..=0x0002_624B).contains(&trap.guest_addr) // legal font helper
                || (0x0002_CEBD..=0x0002_CEC2).contains(&trap.guest_addr); // font-loader caller
            let in_alloc_chain = matches!(
                trap.guest_addr,
                0x0004_2580
                    | 0x0004_259F
                    | 0x0004_25A4
                    | 0x0004_25BC
                    | 0x0004_25BD
                    | 0x002B_4FA0
                    | 0x002B_4FC1
                    | 0x002B_4FC6
                    | 0x002B_4FC7
                    | 0x002B_4FF2
                    | 0x002B_4FF3
                    | 0x002B_4FFD
                    | 0x002B_5002
                    | 0x002B_5004
            );
            let stack_has_font_ret = if in_alloc_chain {
                (0..16u32).any(|i| {
                    let a = r14.wrapping_add(i * 4);
                    if a < 0x2000_0000 {
                        let w = unsafe { super::veh::guest_read_u32(r15, a) };
                        w == 0x0002_622D || w == 0x0002_CEC2
                    } else {
                        false
                    }
                })
            } else {
                false
            };
            if in_font_helper || stack_has_font_ret {
                static FONT_CALL_TRACE: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = FONT_CALL_TRACE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 140 {
                    let shadow_top = if r12 >= 0x1000 && r12 < 0x7FFF_FFFF_FFFF {
                        unsafe { std::ptr::read_unaligned(r12 as *const u64) }
                    } else {
                        0
                    };
                    // Snapshot the alloc-failed callback at 0x003F5A48
                    // (null-callback hypothesis proof point).
                    let cb_0x3f5a48 = unsafe { super::veh::guest_read_u32(r15, 0x003F_5A48) };
                    let mut stack = [0u32; 9];
                    for i in 0..9 {
                        let a = r14.wrapping_sub(16).wrapping_add((i as u32) * 4);
                        if a < 0x2000_0000 {
                            stack[i] = unsafe { super::veh::guest_read_u32(r15, a) };
                        }
                    }
                    crate::xbox::emulator::debug_log(&format!(
                        "[CALL-BOUNDARY #{}] g=0x{:08X} ESP(R14)=0x{:08X} EBP=0x{:08X} EAX=0x{:08X} ECX=0x{:08X} ESI=0x{:08X} R12=0x{:016X} R12[0]=0x{:016X} [ESP]=0x{:08X} [R14-16..+16]={:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} [0x3F5A48]=0x{:08X}",
                        n, trap.guest_addr, r14, rbp, rax, context.Rcx as u32, context.Rsi as u32,
                        r12, shadow_top, s0,
                        stack[0], stack[1], stack[2], stack[3], stack[4],
                        stack[5], stack[6], stack[7], stack[8],
                        cb_0x3f5a48
                    ));
                }
            } else {
                crate::rate_log!(
                    2000,
                    "[P0-STACK] g=0x{:08X} R14=0x{:08X} [R14]=0x{:08X}",
                    trap.guest_addr,
                    r14,
                    s0
                );
            }
            context.Rip += 1;
            return VehResult::Handled;
        }

        TrapType::RetMiss => {
            // Inline RET fast-path miss — normal slow path (hash collision or empty slot).
            // R14 and R12 already adjusted by inline code. R10D has guest return address.
            // VEH does full hash probe and jumps on hit, stack scan on miss.
            ctx.ret_hash_lookups += 1;
            crate::xbox::aot::bink_profile::note_ret_trap(trap.guest_addr);
            // Periodic .text integrity check
            if ctx.ret_hash_lookups % 500 == 0 {
                check_text_integrity(
                    context.R15,
                    ctx.ret_hash_lookups as usize,
                    trap.guest_addr,
                    context.R14 as u32,
                );
            }
            let guest_ret = context.R10 as u32;
            crate::xbox::aot::jit::note_ret_miss(
                trap.guest_addr,
                guest_ret,
                context.R14 as u32,
                ctx.ret_hash_lookups,
            );

            // PHASE 7 · RET fast-path miss logger (log-only).
            // Every fast miss means the inline RET predictor did not find the
            // return target. This is not an AOT miss by itself: the full address
            // hash below often resolves it cleanly. Only RET-UNRES means a
            // plausible executable target was absent from the compiled map.
            // First 200 events always logged; then every 10000th.
            static RET_FAST_MISS_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let rm_n = RET_FAST_MISS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if rm_n < 200 || rm_n % 10_000 == 0 {
                // Peek top of shadow stack to see what R12 PREDICTED vs what
                // the guest actually wants. Null deref guard on R12 pointer.
                let r12 = context.R12;
                let shadow_top = if r12 >= 0x1000 && r12 < 0x7FFF_FFFF_FFFF {
                    unsafe {
                        // [r12] is the most recent shadow entry (host addr we expected to return to)
                        std::ptr::read_unaligned(r12 as *const u64)
                    }
                } else {
                    0
                };
                crate::xbox::emulator::debug_log(&format!(
                    "[RET-FASTMISS #{}] guest_RET_addr={} guest_target={} R14=0x{:08X} R12_top=0x{:016X} slow_path=full_hash_pending (hash_lookups_total={})",
                    rm_n,
                    crate::xbox::logging::fmt_guest_addr(trap.guest_addr),
                    crate::xbox::logging::fmt_guest_addr(guest_ret),
                    context.R14 as u32,
                    shadow_top, ctx.ret_hash_lookups
                ));
            }

            if guest_ret == 0 {
                // Count RET_TO_ZERO events — after too many, stop VEH-level recovery
                // and exit to dispatch loop. On real Xbox, main() never returns.
                static RET_ZERO_COUNT: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let rz_count = RET_ZERO_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                // Before RET_TO_ZERO, check if this is a spurious zero (function arg/local)
                // rather than the real stack bottom.
                //
                // 2026-04-27 Spider-Man: after START, the scene/loader worker runs on
                // lower worker stacks such as 0x1CF7xxxx / 0x1CFBxxxx, not only the
                // original 0x1E/0x1F main stack. The old recovery window only scanned
                // 0x1E00_0000..0x1F00_0000, so a transient zero popped on the loader
                // stack immediately became RET_TO_ZERO and killed scene init.
                //
                // Keep this narrow and RET-to-zero-only: do not broadly stack-scan all
                // bad returns. For zero returns, scan a small forward window for the
                // next real executable return address already present on the guest
                // stack, preserving guest registers and continuing organically.
                let r14 = context.R14 as u32;
                let r15 = context.R15;
                let spidey_worker_stack = super::oovpa::XBE_ENTRY
                    .load(std::sync::atomic::Ordering::Relaxed)
                    == 0x002A_9C38
                    && r14 >= 0x1C00_0000
                    && r14 < 0x2000_0000;
                if rz_count < 16 && spidey_worker_stack {
                    for offset in (0..256u32).step_by(4) {
                        let addr = r14.wrapping_add(offset);
                        if addr >= 0x2000_0000 {
                            break;
                        }
                        let candidate = unsafe { super::veh::guest_read_u32(r15, addr) };
                        let norm = normalize_addr(candidate);
                        if norm >= 0x0001_0000 && norm < 0x0080_0000 {
                            if let Some(host_off) = ctx.addr_hash.lookup(norm) {
                                static RET_ZERO_SPIDEY_RECOVER_LOG: std::sync::atomic::AtomicU32 =
                                    std::sync::atomic::AtomicU32::new(0);
                                let n = RET_ZERO_SPIDEY_RECOVER_LOG
                                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if n < 32 || n.is_power_of_two() {
                                    crate::xbox::emulator::debug_log(&format!(
                                        "[RET-ZERO-SPIDEY-RECOVER #{}] ret_site=0x{:08X} old_R14=0x{:08X} offset=+{} recovered=0x{:08X} new_R14=0x{:08X}",
                                        n,
                                        trap.guest_addr,
                                        r14,
                                        offset,
                                        norm,
                                        addr.wrapping_add(4)
                                    ));
                                }
                                context.R14 = addr.wrapping_add(4) as u64;
                                context.Rip = code_base + host_off as u64;
                                record_event(trap.guest_addr, norm, 6, addr.wrapping_add(4));
                                return VehResult::Handled;
                            }
                        }
                    }
                }

                if ENABLE_BROAD_STACK_SCAN_RECOVERY
                    && rz_count < 5
                    && r14 < 0x1F00_0000
                    && r14 > 0x1E00_0000
                {
                    for offset in (0..128u32).step_by(4) {
                        let addr = r14.wrapping_add(offset);
                        if addr >= 0x1F00_0000 {
                            break;
                        }
                        let candidate = unsafe { super::veh::guest_read_u32(r15, addr) };
                        let norm = normalize_addr(candidate);
                        if norm >= 0x10000 {
                            if let Some(host_off) = ctx.addr_hash.lookup(norm) {
                                context.R14 = (addr + 4) as u64;
                                context.Rip = code_base + host_off as u64;
                                record_event(trap.guest_addr, norm, 6, addr + 4);
                                return VehResult::Handled;
                            }
                        }
                    }
                }
                if rz_count < 10 {
                    dump_recent_activity("RET_TO_ZERO via RetMiss");
                }
                ctx.guest.esp = r14;
                ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_RET_TO_ZERO;
                ctx.guest.exit_guest_addr = 0;
                context.Rip = ctx.guest.exit_addr;
                return VehResult::Handled;
            }

            let normalized = normalize_addr(guest_ret);
            record_event(trap.guest_addr, normalized, 3, context.R14 as u32);

            // NOTE: Kernel ordinals (0xFFFF0000+) on the guest stack indicate
            // stack corruption — kernel addresses should never be RET targets.
            // The TrapType::Indirect handler is the correct path for kernel thunks.
            // If we see one here, just let it fall through to stack scan / UNRESOLVED.

            if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                let doom_column_ret_name = match normalized {
                    0x0002_D536 => Some("DOOM_SPAN_CALL_AFTER_RETMISS"),
                    0x0002_F16B => Some("DOOM_COLUMN_CALL_AFTER_RETMISS"),
                    0x0003_9B16 => Some("DOOM_WALL_COLUMN_CALL_MID_AFTER_RETMISS"),
                    0x0003_9BD5 => Some("DOOM_WALL_COLUMN_CALL_TOP_AFTER_RETMISS"),
                    0x0003_9C97 => Some("DOOM_WALL_COLUMN_CALL_BOTTOM_AFTER_RETMISS"),
                    _ => None,
                };
                if let Some(probe_name) = doom_column_ret_name {
                    if probe_name.starts_with("DOOM_SPAN_") {
                        crate::xbox::aot::oovpa::oovpa_hle::doom_span_call_tap(
                            probe_name,
                            context.R15 as *mut u8,
                            context.Rax as u32,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rsi as u32,
                            context.Rdi as u32,
                        );
                    } else {
                        crate::xbox::aot::oovpa::oovpa_hle::doom_column_call_tap(
                            probe_name,
                            context.R15 as *mut u8,
                            context.Rax as u32,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rsi as u32,
                            context.Rdi as u32,
                        );
                    }
                }
                static RET_FASTMISS_FULL_HIT_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n =
                    RET_FASTMISS_FULL_HIT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[RET-FASTMISS-FULL-HIT #{}] ret_site={} target={} host=+0x{:X} R14=0x{:08X} kind=fast_path_only",
                        n,
                        crate::xbox::logging::fmt_guest_addr(trap.guest_addr),
                        crate::xbox::logging::fmt_guest_addr(normalized),
                        host_off,
                        context.R14 as u32
                    ));
                }
                context.Rip = code_base + host_off as u64;
                return VehResult::Handled;
            }

            // If inline RET popped a stack/data pointer, but the next dword at the
            // now-current ESP is a compiled return address, the guest ESP was exactly
            // one slot low at RET entry. Recover before the broad stack scan so we
            // preserve EAX/ECX/etc. (the generic scan intentionally clobbers EAX).
            //
            // Low .data pointers are just as poisonous as high heap/stack pointers:
            // Spider-Man's XGetDeviceChanges writes globals at 0x003F59F4/98, and
            // the legal-splash guard can transiently pop 0x003F59F4 while the real
            // return address (0x0003AC37) is one slot higher.
            let target_is_data = ctx
                .data_ranges
                .iter()
                .any(|&(start, end)| normalized >= start && normalized < end);
            let target_is_exec = ctx
                .exec_ranges
                .iter()
                .any(|&(start, end)| normalized >= start && normalized < end);
            if normalized >= 0x0080_0000 || target_is_data || !target_is_exec {
                let r14 = context.R14 as u32;
                let r15 = context.R15;
                let candidate = unsafe { super::veh::guest_read_u32(r15, r14) };
                let candidate_norm = normalize_addr(candidate);
                if candidate_norm >= 0x10000 && candidate_norm < 0x0080_0000 {
                    if let Some(host_off) = ctx.addr_hash.lookup(candidate_norm) {
                        static RET_ONE_LOW_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = RET_ONE_LOW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 50 || n % 100 == 0 {
                            crate::xbox::emulator::debug_log(&format!(
                                "[RET-ONELOW-RECOVER #{}] ret_site=0x{:08X} bad_target=0x{:08X} recovered=0x{:08X} old_R14=0x{:08X} new_R14=0x{:08X} eax=0x{:08X}",
                                n,
                                trap.guest_addr,
                                normalized,
                                candidate_norm,
                                r14,
                                r14.wrapping_add(4),
                                context.Rax as u32
                            ));
                        }
                        context.R14 = r14.wrapping_add(4) as u64;
                        context.Rip = code_base + host_off as u64;
                        record_event(trap.guest_addr, candidate_norm, 5, r14.wrapping_add(4));
                        return VehResult::Handled;
                    }
                }
            }

            // Plausible executable XBE code address not in hash — exit to worker
            // for rescue_emit. Low .data/.rdata addresses can live in this same
            // numeric window; those must fall through to the stack scan below.
            if target_is_exec
                && !target_is_data
                && normalized >= 0x10000
                && normalized < 0x0080_0000
            {
                let r14 = context.R14 as u32;
                // PUSH-SITE PROBE (2026-04-22): capture the context when a
                // RET target falls through hash-lookup-miss to UNRESOLVED.
                // This is the path that produces our 0x00476920 crash.
                // Log the RET site (trap.guest_addr), the target, the raw
                // stack value at pre-RET ESP, and top 8 dwords of stack.
                static RET_UNRESOLVED_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = RET_UNRESOLVED_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // Log first 10 events always, then every 100th for tail.
                if n < 10 || (normalized == 0x0047_6920 && n < 100) {
                    let r15 = context.R15;
                    let mut stack = [0u32; 10];
                    for i in 0..10 {
                        let a = r14.wrapping_sub(4).wrapping_add((i as u32) * 4);
                        if a < 0x2000_0000 {
                            stack[i] = unsafe { super::veh::guest_read_u32(r15, a) };
                        }
                    }
                    crate::xbox::emulator::debug_log(&format!(
                        "[RET-UNRES #{}] kind=AOT_MISS ret_site={} target={} R14=0x{:08X} EAX=0x{:08X} EBP=0x{:08X} [R14-4..+32]={:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
                        n,
                        crate::xbox::logging::fmt_guest_addr(trap.guest_addr),
                        crate::xbox::logging::fmt_guest_addr(normalized),
                        r14, context.Rax as u32, context.Rbp as u32,
                        stack[0], stack[1], stack[2], stack[3], stack[4],
                        stack[5], stack[6], stack[7], stack[8], stack[9]
                    ));
                }
                ctx.guest.esp = r14;
                ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
                ctx.guest.exit_guest_addr = normalized;
                context.Rip = ctx.guest.exit_addr;
                return VehResult::Handled;
            }

            // Garbage address — stack scan fallback.
            // Scan up to 2048 bytes forward from current ESP for compiled addresses.
            // Upper bound must accommodate high stacks (worker at 0x1EFFFFXX).
            //
            // PHASE 8 LOUD LOGGING (2026-04-20): this fallback is the
            // "silent recovery" mechanism that can pick an SEH handler
            // pointer or a stale return address as the "next" RET target.
            // When that happens inside RtlCreateHeap, the function returns
            // early without writing the heap struct. Log EVERY scan that
            // hits so we can see exactly which silent-recovery is
            // corrupting the call chain.
            let r15 = context.R15;
            let r14 = context.R14 as u32;
            if ENABLE_BROAD_STACK_SCAN_RECOVERY {
                for offset in (0..2048u32).step_by(4) {
                    let addr = r14.wrapping_add(offset);
                    if addr > 0x2000_0000 {
                        break;
                    }
                    let candidate = unsafe { super::veh::guest_read_u32(r15, addr) };
                    let norm = normalize_addr(candidate);
                    if norm >= 0x10000 && norm < 0x0080_0000 {
                        if let Some(host_off) = ctx.addr_hash.lookup(norm) {
                            GLOBAL_STACK_SCANS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            // Loud log: always first 100, then every 100th.
                            static SCAN_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let sl_n = SCAN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if sl_n < 100 || sl_n % 100 == 0 {
                                crate::xbox::emulator::debug_log(&format!(
                                    "[STACK-SCAN-HIT #{}] ret_from=0x{:08X} orig_R14=0x{:08X} scanned_to_offset=+{} picked_target=0x{:08X} new_R14=0x{:08X} (SILENT RECOVERY — may jump into SEH handler or wrong scope)",
                                    sl_n, trap.guest_addr, r14, offset, norm, addr + 4
                                ));
                            }
                            context.R14 = (addr + 4) as u64;
                            context.Rip = code_base + host_off as u64;
                            record_event(trap.guest_addr, norm, 5, addr + 4);
                            return VehResult::Handled;
                        }
                    }
                }
            }

            // Unresolved
            ctx.guest.esp = r14;
            ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
            ctx.guest.exit_guest_addr = normalized;
            context.Rip = ctx.guest.exit_addr;
            return VehResult::Handled;
        }
    }
}

// ============================================================================
// RET handler — guest stack read + hash lookup, stack scan fallback
// ============================================================================

/// Handle a RET trap: read return address from guest stack, hash lookup,
/// stack scan fallback, or exit trampoline if unresolvable.
#[cfg(windows)]
fn handle_ret(
    code_base: u64,
    host_offset: u32,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    trap: &crate::xbox::aot::runtime::TrapInfo,
) -> VehResult {
    ctx.ret_hash_lookups += 1;
    let r15 = context.R15;
    let r14 = context.R14 as u32;

    // R12 pop DISABLED — emit_call_rel no longer pushes to R12, so nothing to pop.
    // GPS-RET disabled: RET dispatch uses emit_inline_guest_ret (guest stack hash lookup).

    // Read return address from guest stack
    let guest_ret = unsafe { super::veh::guest_read_u32(r15, r14) };
    let new_r14 = r14.wrapping_add(4).wrapping_add(trap.ret_cleanup as u32);

    let doom_column_ret_name = match guest_ret {
        0x0002_D536 => Some("DOOM_SPAN_CALL_AFTER_RET"),
        0x0002_F16B => Some("DOOM_COLUMN_CALL_AFTER_RET"),
        0x0003_9B16 => Some("DOOM_WALL_COLUMN_CALL_MID_AFTER_RET"),
        0x0003_9BD5 => Some("DOOM_WALL_COLUMN_CALL_TOP_AFTER_RET"),
        0x0003_9C97 => Some("DOOM_WALL_COLUMN_CALL_BOTTOM_AFTER_RET"),
        _ => None,
    };
    if let Some(probe_name) = doom_column_ret_name {
        if probe_name.starts_with("DOOM_SPAN_") {
            crate::xbox::aot::oovpa::oovpa_hle::doom_span_call_tap(
                probe_name,
                context.R15 as *mut u8,
                context.Rax as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            );
        } else {
            crate::xbox::aot::oovpa::oovpa_hle::doom_column_call_tap(
                probe_name,
                context.R15 as *mut u8,
                context.Rax as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            );
        }
    }

    // P0 CRASH SITE diagnostic
    if trap.guest_addr == 0x0029_C874 {
        crate::rate_log!(
            5,
            "[P0-CRASH] R14=0x{:08X} ret=0x{:08X} cleanup={} EBP=0x{:08X}",
            r14,
            guest_ret,
            trap.ret_cleanup,
            context.Rbp as u32
        );
    }

    // Zone RET diagnostic (P0 zone + D3D section + stdcall)
    let in_p0 = trap.guest_addr >= 0x0029_C400 && trap.guest_addr <= 0x0029_D000;
    let in_d3d = trap.guest_addr >= 0x002E_BDA0 && trap.guest_addr <= 0x0030_38F8;
    if in_p0 || in_d3d || trap.ret_cleanup > 0 {
        crate::rate_log!(
            100,
            "[{}-RET] g=0x{:08X} ret=0x{:08X} R14=0x{:08X}→0x{:08X} cleanup={}",
            if in_p0 {
                "P0"
            } else if in_d3d {
                "D3D"
            } else {
                "STD"
            },
            trap.guest_addr,
            guest_ret,
            r14,
            new_r14,
            trap.ret_cleanup
        );
    }

    context.R14 = new_r14 as u64;
    trace_stack_window(
        "RET",
        trap.guest_addr,
        context,
        ctx,
        &format!(
            "ret_to=0x{:08X} cleanup={} pre_esp=0x{:08X}",
            guest_ret, trap.ret_cleanup, r14
        ),
    );
    record_event(trap.guest_addr, guest_ret, 3, new_r14);

    // RET to 0 = exit
    if guest_ret == 0 {
        dump_recent_activity("RET_TO_ZERO");
        ctx.guest.esp = new_r14;
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_RET_TO_ZERO;
        ctx.guest.exit_guest_addr = 0;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    }

    // Mirror normalization + hash lookup
    let normalized = normalize_addr(guest_ret);
    if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
        context.Rip = code_base + host_off as u64;
        return VehResult::Handled;
    }

    // Stack scan fallback — find nearest valid return address.
    //
    // PHASE 8 LOUD LOGGING (2026-04-20): this is the second stack-scan
    // fallback (handle_ret path, vs the RetMiss path). Same "silent
    // recovery" problem — may pick an SEH handler pointer or stale
    // return address as the RET target.
    let r15 = context.R15;
    if ENABLE_BROAD_STACK_SCAN_RECOVERY {
        for offset in (0..2048u32).step_by(4) {
            let addr = new_r14.wrapping_add(offset);
            if addr > 0x2000_0000 {
                break;
            }
            let candidate = unsafe { super::veh::guest_read_u32(r15, addr) };
            let norm = normalize_addr(candidate);
            if norm >= 0x10000 && norm < 0x0080_0000 {
                if let Some(host_off) = ctx.addr_hash.lookup(norm) {
                    // Loud log: first 100, then every 100th.
                    static RET_SCAN_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let sl_n = RET_SCAN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if sl_n < 100 || sl_n % 100 == 0 {
                        let guest_func = ctx.addr_hash.reverse_lookup(host_offset);
                        crate::xbox::emulator::debug_log(&format!(
                            "[STACK-SCAN-HIT-L3 #{}] ret_from=0x{:08X} (guest_func~0x{:08X}) bad_ret=0x{:08X} scanned_to_offset=+{} picked_target=0x{:08X} new_R14=0x{:08X} (SILENT RECOVERY — may jump into SEH handler or wrong scope)",
                            sl_n, trap.guest_addr, guest_func, guest_ret, offset, norm, addr + 4
                        ));
                    }
                    context.R14 = (addr + 4) as u64;
                    context.Rip = code_base + host_off as u64;
                    record_event(trap.guest_addr, norm, 5, addr + 4);
                    GLOBAL_STACK_SCANS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let guest_func = ctx.addr_hash.reverse_lookup(host_offset);
                    crate::rate_log!(50, "[L3-RET] STACK SCAN: bad ret 0x{:08X} from guest=0x{:08X} found 0x{:08X} at ESP+{}",
                        guest_ret, guest_func, candidate, offset);
                    return VehResult::Handled;
                }
            }
        }
    }

    // Truly unresolvable — exit trampoline
    let guest_func = ctx.addr_hash.reverse_lookup(host_offset);
    crate::rate_log!(
        20,
        "[L3-RET] UNRESOLVED RET to {} from guest={} esp=0x{:08X}",
        crate::xbox::logging::fmt_guest_addr(normalized),
        crate::xbox::logging::fmt_guest_addr(guest_func),
        new_r14
    );
    ctx.guest.esp = new_r14;
    ctx.guest.exit_reason = AOT_EXIT_UNRESOLVED;
    ctx.guest.exit_guest_addr = normalized;
    context.Rip = ctx.guest.exit_addr;
    VehResult::Handled
}

// ============================================================================
// Kernel call exit — save state and exit trampoline for safe dispatch in Rust
// ============================================================================

/// Handle a kernel call by EXITING the trampoline. Dispatch happens in safe
/// Rust code (dispatch loop), not inside VEH where heap locks may be held.
///
/// VEH reads args, does stdcall cleanup, saves ordinal+args to ctx, then exits.
/// The dispatch loop calls kernel::dispatch_inline() and re-enters the trampoline.
#[cfg(windows)]
fn handle_kernel_exit(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    code_base: u64,
    host_offset: u32,
    target: u32,
    is_call: bool,
    call_guest_ret_addr: u32,
) -> VehResult {
    use crate::xbox::aot::runtime::{AOT_EXIT_ERROR, AOT_EXIT_KERNEL_CALL};
    use crate::xbox::kernel::{ordinals, KERNEL_MAGIC_BASE};

    let ordinal = target - KERNEL_MAGIC_BASE;
    if ordinal >= 0x8000 {
        let r14 = context.R14 as u32;
        let mut stack = String::new();
        for i in 0..8u32 {
            let addr = r14.wrapping_add(i * 4);
            let value = unsafe { super::veh::guest_read_u32(context.R15, addr) };
            stack.push_str(&format!(" [0x{:08X}]=0x{:08X}", addr, value));
        }
        veh_log(&format!(
            "[KERNEL-INVALID-TARGET] target=0x{:08X} ordinal={} is_call={} host_off=0x{:X} call_ret=0x{:08X} r14=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} stack:{}",
            target,
            ordinal,
            is_call,
            host_offset,
            call_guest_ret_addr,
            r14,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            stack
        ));
        ctx.guest.exit_reason = AOT_EXIT_ERROR;
        ctx.guest.exit_guest_addr = target;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    }
    record_event(0, ordinal, 0, context.R14 as u32);
    let arg_count = ordinals::arg_count(ordinal);
    let r14 = context.R14 as u32;

    crate::rate_log!(
        50,
        "[L2-SHADOW] KERN-ENTRY ord={} R12=0x{:X} R14=0x{:08X}",
        ordinal,
        context.R12,
        r14
    );
    let r15 = context.R15;

    if is_call && call_guest_ret_addr == 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[KERNEL-CALL-RETADDR-ZERO] ord={} {} host_off=0x{:X} esp=0x{:08X}",
            ordinal,
            crate::xbox::kernel::ordinals::name(ordinal),
            host_offset,
            r14
        ));
    }

    // For JMP (tail call): save return address from [R14+0] BEFORE cleanup
    let jmp_ret_addr = if !is_call {
        unsafe { super::veh::guest_read_u32(r15, r14) }
    } else {
        call_guest_ret_addr
    };

    // Read args from guest stack
    let arg_base = if is_call { 0u32 } else { 4 };
    let mut args = [0u32; 12];
    for i in 0..arg_count.min(12) {
        args[i as usize] = unsafe { super::veh::guest_read_u32(r15, r14 + arg_base + i * 4) };
    }

    // Diagnostic: raw stack dump for MmAllocateContiguousMemoryEx (ordinal 166)
    if ordinal == 166 {
        let mut stack_vals = [0u32; 8];
        for i in 0..8 {
            stack_vals[i] = unsafe { super::veh::guest_read_u32(r15, r14 + i as u32 * 4) };
        }
        crate::xbox::emulator::debug_log(&format!(
            "[MM-DIAG] ord=166 is_call={} arg_base={} R14=0x{:08X} stack=[{:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}]",
            is_call, arg_base, r14,
            stack_vals[0], stack_vals[1], stack_vals[2], stack_vals[3],
            stack_vals[4], stack_vals[5], stack_vals[6], stack_vals[7]
        ));
    }

    // ----------------------------------------------------------------------
    // PHASE 4 · Matched-transition push for VEH-path kernel calls.
    // Records entry R14 + argc + ordinal. The matching pop+verify happens
    // immediately after stdcall cleanup below. Log-only, zero behavioral
    // effect.
    // ----------------------------------------------------------------------
    {
        use crate::xbox::aot::transition;
        transition::push(transition::TransitionFrame::new_kernel(
            ordinal as u16,
            crate::xbox::kernel::ordinals::name(ordinal),
            r14,          // entry ESP before any cleanup
            0,            // guest_entry_pc unknown (VEH-dispatched)
            jmp_ret_addr, // CALL guest continuation or JMP caller ret addr
            arg_count as u8,
        ));
    }

    // Stdcall cleanup in R14 (guest ESP).
    // CALL: no guest ret_addr was pushed (INT3 replaced CALL). Pop args only.
    // JMP: pop caller's ret_addr + args.
    if is_call {
        context.R14 += (arg_count * 4) as u64;
    } else {
        context.R14 += (4 + arg_count * 4) as u64;
    }

    // ----------------------------------------------------------------------
    // PHASE 4 · Pop and verify R14 matches expected-cleanup.
    // For CALL: entry had no ret_addr on stack (INT3 replaced CALL), so
    //   expected is entry + argc*4 (no +4 for ret_addr). The TransitionFrame
    //   was constructed with argc which includes an implicit +4 — pass
    //   observed+4 to match.
    // For JMP: we popped BOTH args AND ret_addr, so observed already matches
    //   expected with no adjustment.
    // ----------------------------------------------------------------------
    {
        use crate::xbox::aot::transition;
        let observed = if is_call {
            (context.R14 as u32).wrapping_add(4)
        } else {
            context.R14 as u32
        };
        let _ = transition::pop_and_verify(Some(transition::TransitionKind::Kernel), observed);
    }

    // Stack trace: post-cleanup ESP
    {
        let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
        trace_stack_window(
            "KERN-EXIT",
            0, // guest_addr not available here
            context,
            ctx,
            &format!("ord={} argc={} pre_esp=0x{:08X}", ord_name, arg_count, r14),
        );
    }

    // Save kernel call details to RuntimeContext for dispatch loop
    ctx.kernel_ordinal = ordinal;
    ctx.kernel_args = args;
    ctx.kernel_arg_count = arg_count;
    ctx.kernel_is_call = is_call;
    ctx.kernel_jmp_ret_addr = jmp_ret_addr;
    // Host RIP to resume after kernel dispatch.
    //
    // For CALL (e.g. `call [0x381790]`): the INT3 replaced the CALL instruction.
    //   No emit_call_rel was involved, no shadow stack push. Resume at INT3+1.
    //
    // For JMP thunk (e.g. `call 0x2B1A3A` → `jmp [0x3818C4]`):
    //   emit_call_rel pushed the caller's return address onto R12 (shadow stack)
    //   and the guest stack before JMPing to the thunk. The JMP handler already
    //   popped jmp_ret_addr from the guest stack. We must also pop R12 and
    //   resume at the caller's return address, NOT at INT3+1 (which is dead
    //   code after the JMP thunk).
    ctx.kernel_r14_pre_cleanup = context.R14 as u32;
    if !is_call && jmp_ret_addr != 0 {
        // JMP thunk reached via emit_call_rel — resolve caller's return address
        if let Some(host_off) = ctx.addr_hash.lookup(jmp_ret_addr) {
            ctx.kernel_host_resume = code_base + host_off as u64;
            crate::rate_log!(
                10,
                "[KERNEL-JMP] Resolved jmp_ret=0x{:08X} -> host+0x{:X}",
                jmp_ret_addr,
                host_off
            );
        } else {
            ctx.kernel_host_resume = code_base + host_offset as u64 + 1;
        }
    } else {
        // Direct CALL — resume at INT3+1
        ctx.kernel_host_resume = code_base + host_offset as u64 + 1;
    }

    crate::rate_log!(
        5,
        "[L4-SHADOW] PRE-SYNC R12=0x{:X} resume=0x{:X}",
        context.R12,
        ctx.kernel_host_resume
    );

    // Sync guest registers but PRESERVE shadow_stack_top.
    // The trampoline exit will save R12 → shadow_stack_top.
    // If we overwrite it here, the trampoline's save is redundant
    // and any R12 changes from emit_call_rel CALLs are lost.
    // Frida-style invariant: R12 holds only guest call frames (from emit_call_rel).
    // Kernel/HLE dispatcher resumes stay in ctx.kernel_host_resume, never on R12.
    // This prevents double-entry consumption on the first RET after kernel return.

    let saved_shadow = ctx.guest.shadow_stack_top;
    unsafe { sync_guest_from_context(ctx, context) };
    ctx.guest.shadow_stack_top = saved_shadow;
    ctx.guest.exit_reason = AOT_EXIT_KERNEL_CALL;
    ctx.guest.exit_guest_addr = target;

    // Sprint 2 fix (audit-2026-04-23.md P0.2 — R15 drift at dispatch#5):
    // If an R15 sign-extension fixup was armed (R15 temporarily shifted
    // ±4GB with TF set by handle_r15_fixup), redirecting RIP below will
    // never reach handle_single_step that normally restores R15. The
    // dispatcher would then resume with a shifted R15, corrupting every
    // subsequent guest-memory read of the form `[R15+reg]` by 0x1_0000_0000
    // bytes. 7 R15-fixup events with +4294967296 delta were observed in
    // the latest Spider-Man run. clear_pending_fixup restores R15 to the
    // pre-shift original and clears the trap flag; no-op when no fixup
    // is pending.
    super::veh_fixup::clear_pending_fixup(context);

    context.Rip = ctx.guest.exit_addr;

    VehResult::Handled
}

// ============================================================================
// System instruction emulation (RDTSC, CPUID, RDMSR, WBINVD, CLI/STI)
// ============================================================================

/// Handle a non-kernel system trap inline: emulate the guest instruction.
/// Matches C++ v1 AOT_VEH_Dispatch.cpp system instruction handling.
#[cfg(windows)]
fn handle_system_inline(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    code_base: u64,
    host_offset: u32,
    trap: &crate::xbox::aot::runtime::TrapInfo,
) -> VehResult {
    let r15 = context.R15;
    let guest_addr = trap.guest_addr;

    // Read guest bytes at trap address to identify the instruction
    // Use snapshot to avoid reading corrupted live memory
    let gb_vec = read_guest_bytes(r15, guest_addr, 4);
    let gb = &gb_vec[..];

    if gb[0] == 0x0F && gb[1] == 0x31 {
        // RDTSC: Xbox CPU = 733.333 MHz
        // Use real wall-clock time scaled to 733MHz (matching C++ v1)
        use std::sync::atomic::{AtomicI64, Ordering};
        use windows::Win32::System::Performance::{
            QueryPerformanceCounter, QueryPerformanceFrequency,
        };

        static QPC_FREQ: AtomicI64 = AtomicI64::new(0);
        static QPC_START: AtomicI64 = AtomicI64::new(0);

        let mut freq = QPC_FREQ.load(Ordering::Relaxed);
        if freq == 0 {
            let mut f = 0i64;
            let mut s = 0i64;
            unsafe {
                let _ = QueryPerformanceFrequency(&mut f as *mut i64);
                let _ = QueryPerformanceCounter(&mut s as *mut i64);
            }
            QPC_FREQ.store(f, Ordering::Relaxed);
            QPC_START.store(s, Ordering::Relaxed);
            freq = f;
        }

        let mut now = 0i64;
        unsafe {
            let _ = QueryPerformanceCounter(&mut now as *mut i64);
        }
        let start = QPC_START.load(Ordering::Relaxed);
        let elapsed_sec = (now - start) as f64 / freq as f64;
        let tsc = (elapsed_sec * 733_333_333.0) as u64;

        context.Rax = (tsc & 0xFFFF_FFFF) as u64;
        context.Rdx = (tsc >> 32) as u64;

        if ctx.system_traps <= 5 {
            veh_log(&format!(
                "[SYSTEM] RDTSC at guest 0x{:08X} -> EDX:EAX = 0x{:08X}:{:08X}",
                guest_addr,
                (tsc >> 32) as u32,
                tsc as u32
            ));
        }
    } else if gb[0] == 0x0F && gb[1] == 0xA2 {
        // CPUID: emulate Xbox Pentium III / Celeron (matching C++ v1)
        let leaf = context.Rax as u32;
        match leaf {
            0 => {
                context.Rax = 2; // max leaf
                context.Rbx = 0x756E6547; // "Genu"
                context.Rcx = 0x6C65746E; // "ntel"
                context.Rdx = 0x49656E69; // "ineI"
            }
            1 => {
                context.Rax = 0x00000673; // Pentium III family 6 model 7 stepping 3
                context.Rbx = 0;
                context.Rcx = 0;
                context.Rdx = 0x0383FBFF; // features: SSE, MMX, etc.
            }
            _ => {
                context.Rax = 0;
                context.Rbx = 0;
                context.Rcx = 0;
                context.Rdx = 0;
            }
        }
        if ctx.system_traps <= 5 {
            veh_log(&format!(
                "[SYSTEM] CPUID leaf={} at guest 0x{:08X}",
                leaf, guest_addr
            ));
        }
    } else if gb[0] == 0x0F && gb[1] == 0x32 {
        // RDMSR: read model-specific register
        let msr = context.Rcx as u32;
        let (eax_val, edx_val) = if msr == 0x2A {
            (0x05C0_0000u32, 0u32) // bus ratio in bits 22-26
        } else {
            (0u32, 0u32)
        };
        context.Rax = eax_val as u64;
        context.Rdx = edx_val as u64;
        if ctx.system_traps <= 10 {
            veh_log(&format!(
                "[SYSTEM] RDMSR(0x{:X}) -> EAX=0x{:X} EDX=0x{:X} at guest 0x{:08X}",
                msr, eax_val, edx_val, guest_addr
            ));
        }
    } else if gb[0] == 0x0F && gb[1] == 0x09 {
        // WBINVD: Write Back and Invalidate Cache — NOP in emulator
        // D3D uses this for GPU DMA coherency; our memory model is coherent
        if ctx.system_traps <= 5 {
            veh_log(&format!(
                "[SYSTEM] WBINVD at guest 0x{:08X} (NOP)",
                guest_addr
            ));
        }
    } else if gb[0] == 0xFA {
        // CLI: clear interrupt flag — NOP in emulator
    } else if gb[0] == 0xFB {
        // STI: set interrupt flag — NOP in emulator
    } else if guest_breakpoint_policy(gb[0]) == GuestBreakpointPolicy::TerminalSystemTrap {
        // Guest INT3 is a breakpoint/assertion, not padding to silently skip.
        // Doom's I_Error path relies on this being terminal; continuing after it
        // falls through into code that assumes the error routine never returns.
        let esp = context.R14 as u32;
        let msg0 = diag_ascii(r15, esp, 192);
        let msg8 = diag_ascii(r15, esp.wrapping_add(8), 192);
        veh_log(&format!(
            "[SYSTEM] INT3 at guest 0x{:08X} esp=0x{:08X} stack='{}' stack+8='{}'",
            guest_addr, esp, msg0, msg8
        ));
        ctx.guest.exit_reason = AOT_EXIT_SYSTEM_TRAP;
        ctx.guest.exit_guest_addr = guest_addr;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    } else if gb[0] == 0xF4 {
        // HLT: halt — exit (guest panic)
        veh_log(&format!(
            "[SYSTEM] HLT at guest 0x{:08X} — guest panic",
            guest_addr
        ));
        ctx.guest.exit_reason = AOT_EXIT_HALT;
        ctx.guest.exit_guest_addr = guest_addr;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    } else if gb[0] == 0xCD {
        // INT imm8 — software interrupt, treat as trap exit
        let int_num = gb[1];
        veh_log(&format!(
            "[SYSTEM] INT 0x{:02X} at guest 0x{:08X}",
            int_num, guest_addr
        ));
        ctx.guest.exit_reason = AOT_EXIT_SYSTEM_TRAP;
        ctx.guest.exit_guest_addr = guest_addr;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    } else {
        // Unknown system instruction — exit
        // Diagnostic: verify R15 and actual memory state
        let ctx_base = ctx.guest.r15_base;
        let direct_byte = unsafe { *(ctx_base as *const u8).add(guest_addr as usize) };
        veh_log(&format!("[SYSTEM] UNKNOWN at guest 0x{:08X} bytes={:02X?} R15=0x{:X} ctx_base=0x{:X} direct_byte=0x{:02X}",
            guest_addr, &gb[..2], r15, ctx_base, direct_byte));
        ctx.guest.exit_reason = AOT_EXIT_SYSTEM_TRAP;
        ctx.guest.exit_guest_addr = guest_addr;
        context.Rip = ctx.guest.exit_addr;
        return VehResult::Handled;
    }

    // Skip past the INT3 (1 byte) — continue execution
    context.Rip = code_base + host_offset as u64 + 1;
    VehResult::Handled
}

#[cfg(test)]
mod tests {
    use super::{guest_breakpoint_policy, GuestBreakpointPolicy};

    #[test]
    fn infra_regression_guest_int3_is_terminal_not_a_nop() {
        assert_eq!(
            guest_breakpoint_policy(0xCC),
            GuestBreakpointPolicy::TerminalSystemTrap
        );
    }

    #[test]
    fn infra_regression_non_breakpoint_opcodes_are_not_classified_as_guest_breakpoints() {
        for opcode in [0x0F, 0xFA, 0xFB, 0xF4, 0xCD, 0x90] {
            assert_eq!(
                guest_breakpoint_policy(opcode),
                GuestBreakpointPolicy::ContinueOneByte
            );
        }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Mirror normalization: 0x80xxxxxx-0x9Fxxxxxx → 0x00xxxxxx-0x1Fxxxxxx
#[inline]
fn normalize_addr(addr: u32) -> u32 {
    if addr >= 0x8000_0000 && addr < 0xA000_0000 {
        addr & 0x1FFF_FFFF
    } else {
        addr
    }
}

/// Resolve the target of an indirect JMP/CALL using LIVE x64 register values.
/// Previous version read from ctx.guest which holds stale values from trampoline entry.
/// The live values are in the x64 CONTEXT (set by the CPU during exception handling).
#[cfg(windows)]
fn resolve_indirect_target_live(
    ctx: &RuntimeContext,
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_addr: u32,
) -> u32 {
    use iced_x86::{Decoder, DecoderOptions, Instruction, OpKind, Register};

    let mem_base = ctx.guest_mem_base;
    if mem_base.is_null() || guest_addr == 0 {
        return guest_addr;
    }

    let bytes = read_guest_bytes(context.R15, guest_addr, 15);

    let mut decoder = Decoder::with_ip(32, &bytes, guest_addr as u64, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    if !decoder.can_decode() {
        return guest_addr;
    }
    decoder.decode_out(&mut instr);
    if instr.is_invalid() {
        return guest_addr;
    }

    // Read register value from LIVE x64 context (lower 32 bits = guest register)
    let live_reg = |r: Register| -> u32 {
        match r {
            Register::EAX | Register::RAX => context.Rax as u32,
            Register::ECX | Register::RCX => context.Rcx as u32,
            Register::EDX | Register::RDX => context.Rdx as u32,
            Register::EBX | Register::RBX => context.Rbx as u32,
            Register::ESP | Register::RSP => context.R14 as u32, // R14 = guest ESP
            Register::EBP | Register::RBP => context.Rbp as u32,
            Register::ESI | Register::RSI => context.Rsi as u32,
            Register::EDI | Register::RDI => context.Rdi as u32,
            _ => 0,
        }
    };

    if instr.op0_kind() == OpKind::Register {
        return live_reg(instr.op_register(0));
    }

    if instr.op0_kind() == OpKind::Memory {
        let base = instr.memory_base();
        let index = instr.memory_index();
        let scale = instr.memory_index_scale() as u32;
        let disp = instr.memory_displacement32();

        let mut ea = disp;
        if base != Register::None {
            ea = ea.wrapping_add(live_reg(base));
        }
        if index != Register::None {
            ea = ea.wrapping_add(live_reg(index).wrapping_mul(scale));
        }

        let target = unsafe {
            let ptr = mem_base.add(ea as usize) as *const u32;
            *ptr
        };
        return target;
    }

    guest_addr
}

/// Check if the guest instruction at the given address is a CALL (vs JMP).
#[cfg(windows)]
fn is_guest_call(ctx: &RuntimeContext, guest_addr: u32) -> bool {
    use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic};
    let mem_base = ctx.guest_mem_base;
    if mem_base.is_null() || guest_addr == 0 {
        return false;
    }
    let bytes: Vec<u8> = (0..15u32)
        .map(|i| unsafe { *mem_base.add((guest_addr + i) as usize) })
        .collect();
    let mut decoder = Decoder::with_ip(32, &bytes, guest_addr as u64, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    if !decoder.can_decode() {
        return false;
    }
    decoder.decode_out(&mut instr);
    let result = instr.mnemonic() == Mnemonic::Call;
    // Log unexpected non-CALL for known call sites
    if !result && (guest_addr == 0x0004_2E59) {
        veh_log(&format!(
            "[IS_CALL-BUG] guest 0x{:08X}: mnemonic={:?} bytes={:02X?}",
            guest_addr,
            instr.mnemonic(),
            &bytes[..instr.len().min(8)]
        ));
    }
    result
}

/// Get the length of the guest instruction at the given address.
#[cfg(windows)]
fn guest_instr_len(ctx: &RuntimeContext, guest_addr: u32) -> u32 {
    use iced_x86::{Decoder, DecoderOptions, Instruction};
    let mem_base = ctx.guest_mem_base;
    if mem_base.is_null() || guest_addr == 0 {
        return 2;
    }
    let bytes: Vec<u8> = (0..15u32)
        .map(|i| unsafe { *mem_base.add((guest_addr + i) as usize) })
        .collect();
    let mut decoder = Decoder::with_ip(32, &bytes, guest_addr as u64, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    if !decoder.can_decode() {
        return 2;
    }
    decoder.decode_out(&mut instr);
    instr.len() as u32
}

/// Log details of the guest instruction at the given address for debugging.
#[cfg(windows)]
fn log_guest_instruction(ctx: &RuntimeContext, guest_addr: u32) {
    use iced_x86::{
        Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter, OpKind, Register,
    };

    let mem_base = ctx.guest_mem_base;
    if mem_base.is_null() || guest_addr == 0 {
        return;
    }

    let bytes: Vec<u8> = (0..15u32)
        .map(|i| unsafe { *mem_base.add((guest_addr + i) as usize) })
        .collect();

    let mut decoder = Decoder::with_ip(32, &bytes, guest_addr as u64, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    if !decoder.can_decode() {
        return;
    }
    decoder.decode_out(&mut instr);
    if instr.is_invalid() {
        return;
    }

    let mut formatter = IntelFormatter::new();
    let mut output = String::new();
    formatter.format(&instr, &mut output);

    if instr.op0_kind() == OpKind::Memory
        || (instr.op_count() >= 2 && instr.op_kind(1) == OpKind::Memory)
    {
        let (base_reg, idx_reg, scale, disp) = if instr.op0_kind() == OpKind::Memory {
            (
                instr.memory_base(),
                instr.memory_index(),
                instr.memory_index_scale(),
                instr.memory_displacement32(),
            )
        } else {
            (
                instr.memory_base(),
                instr.memory_index(),
                instr.memory_index_scale(),
                instr.memory_displacement32(),
            )
        };
        let mut ea = disp;
        if base_reg != Register::None {
            ea = ea.wrapping_add(guest_reg_value(&ctx.guest, base_reg));
        }
        if idx_reg != Register::None {
            ea = ea.wrapping_add(guest_reg_value(&ctx.guest, idx_reg).wrapping_mul(scale as u32));
        }
        let val = unsafe { *(mem_base.add(ea as usize) as *const u32) };
        veh_log(&format!(
            "  instr: {} | EA=0x{:08X} [EA]=0x{:08X} base={:?}=0x{:08X}",
            output,
            ea,
            val,
            base_reg,
            if base_reg != Register::None {
                guest_reg_value(&ctx.guest, base_reg)
            } else {
                0
            }
        ));
    } else {
        veh_log(&format!("  instr: {}", output));
    }
}

/// Read a guest register value from GuestState.
#[cfg(windows)]
fn guest_reg_value(guest: &crate::xbox::aot::runtime::GuestState, reg: iced_x86::Register) -> u32 {
    use iced_x86::Register::*;
    match reg {
        EAX | AX | AL => guest.eax,
        ECX | CX | CL => guest.ecx,
        EDX | DX | DL => guest.edx,
        EBX | BX | BL => guest.ebx,
        ESP | SP => guest.esp,
        EBP | BP => guest.ebp,
        ESI | SI => guest.esi,
        EDI | DI => guest.edi,
        _ => 0,
    }
}

// ============================================================================
// Rescue handler — emulate instructions that the emitter couldn't encode.
// Ported from C++ AOT_VEH.cpp runtime rescue path.
// ============================================================================

/// Handle a rescue trap: decode the original guest instruction and emulate it.
/// trap.ret_cleanup stores the guest instruction length.
/// trap.guest_addr is the guest PC of the failed instruction.
#[cfg(windows)]
fn handle_rescue(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    code_base: u64,
    host_offset: u32,
    trap: &crate::xbox::aot::runtime::TrapInfo,
) -> VehResult {
    let r15 = context.R15;
    let guest_addr = trap.guest_addr;
    let instr_len = trap.ret_cleanup as u32;

    // Decode the original guest bytes (use snapshot to avoid corruption)
    let guest_bytes_vec = read_guest_bytes(r15, guest_addr, instr_len.max(1) as usize);
    let guest_bytes = &guest_bytes_vec[..];

    let mut decoder = iced_x86::Decoder::with_ip(
        32,
        guest_bytes,
        guest_addr as u64,
        iced_x86::DecoderOptions::NONE,
    );
    let instr = decoder.decode();

    if instr.is_invalid() {
        if ctx.rescue_count <= 10 {
            veh_log(&format!(
                "[RESCUE] FAIL: invalid decode at guest 0x{:08X} bytes={:02X?}",
                guest_addr, guest_bytes
            ));
        }
        // Skip the INT3 and hope for the best
        context.Rip = code_base + host_offset as u64 + 1;
        return VehResult::Handled;
    }

    // Compute effective address for memory operands
    let ea = rescue_compute_ea(context, &instr);

    use iced_x86::Mnemonic;
    match instr.mnemonic() {
        Mnemonic::Inc => {
            // INC [mem] or INC reg
            if instr.op0_kind() == iced_x86::OpKind::Memory {
                let ptr = (r15 + ea as u64) as *mut u32;
                let val = unsafe { *ptr };
                unsafe {
                    *ptr = val.wrapping_add(1);
                }
                // Update EFLAGS for INC (ZF, SF, OF, PF, AF — NOT CF)
                rescue_update_flags_inc(context, val, val.wrapping_add(1));
            } else if instr.op0_kind() == iced_x86::OpKind::Register {
                let val = rescue_read_reg(context, instr.op_register(0));
                let new_val = val.wrapping_add(1);
                rescue_write_reg(context, instr.op_register(0), new_val);
                rescue_update_flags_inc(context, val, new_val);
            }
        }
        Mnemonic::Dec => {
            if instr.op0_kind() == iced_x86::OpKind::Memory {
                let ptr = (r15 + ea as u64) as *mut u32;
                let val = unsafe { *ptr };
                unsafe {
                    *ptr = val.wrapping_sub(1);
                }
                rescue_update_flags_dec(context, val, val.wrapping_sub(1));
            } else if instr.op0_kind() == iced_x86::OpKind::Register {
                let val = rescue_read_reg(context, instr.op_register(0));
                let new_val = val.wrapping_sub(1);
                rescue_write_reg(context, instr.op_register(0), new_val);
                rescue_update_flags_dec(context, val, new_val);
            }
        }
        Mnemonic::Push => {
            // PUSH [mem] or PUSH reg
            let val = if instr.op0_kind() == iced_x86::OpKind::Memory {
                unsafe { *((r15 + ea as u64) as *const u32) }
            } else if instr.op0_kind() == iced_x86::OpKind::Register {
                rescue_read_reg(context, instr.op_register(0))
            } else if instr.op0_kind() == iced_x86::OpKind::Immediate32
                || instr.op0_kind() == iced_x86::OpKind::Immediate32to64
            {
                instr.immediate32()
            } else if instr.op0_kind() == iced_x86::OpKind::Immediate8
                || instr.op0_kind() == iced_x86::OpKind::Immediate8to32
            {
                instr.immediate8to32() as u32
            } else {
                0
            };
            // Push: R14 -= 4, [R15+R14] = val
            let new_esp = (context.R14 as u32).wrapping_sub(4);
            unsafe {
                *((r15 + new_esp as u64) as *mut u32) = val;
            }
            context.R14 = new_esp as u64;
        }
        Mnemonic::Pop => {
            // POP [mem] or POP reg
            let esp = context.R14 as u32;
            let val = unsafe { *((r15 + esp as u64) as *const u32) };
            let new_esp = esp.wrapping_add(4);
            context.R14 = new_esp as u64;
            if instr.op0_kind() == iced_x86::OpKind::Memory {
                unsafe {
                    *((r15 + ea as u64) as *mut u32) = val;
                }
            } else if instr.op0_kind() == iced_x86::OpKind::Register {
                rescue_write_reg(context, instr.op_register(0), val);
            }
        }
        Mnemonic::Xlatb => {
            // AL = [EBX + AL]
            let al = (context.Rax & 0xFF) as u32;
            let ebx = context.Rbx as u32;
            let addr = ebx.wrapping_add(al);
            let val = unsafe { *((r15 + addr as u64) as *const u8) };
            context.Rax = (context.Rax & !0xFF) | val as u64;
        }
        Mnemonic::Mov => {
            // MOV r/m32, r32 or MOV r32, r/m32 — critical for FS:[0] SEH chain ops.
            // FS: prefix adds KPCR base (0x0C000000) to displacement.
            let fs_adj: u32 = if instr.segment_prefix() == iced_x86::Register::FS
                || instr.segment_prefix() == iced_x86::Register::GS
            {
                0x0C00_0000
            } else {
                0
            };
            let adj_ea = ea.wrapping_add(fs_adj);

            let op0 = instr.op0_kind();
            let op1 = instr.op1_kind();

            if op0 == iced_x86::OpKind::Memory && op1 == iced_x86::OpKind::Register {
                // MOV [mem], reg (e.g., mov fs:[0], esp)
                let val = rescue_read_reg(context, instr.op_register(1));
                let size = instr.memory_size();
                match size {
                    iced_x86::MemorySize::UInt32 | iced_x86::MemorySize::Int32 => unsafe {
                        *((r15 + adj_ea as u64) as *mut u32) = val;
                    },
                    iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => unsafe {
                        *((r15 + adj_ea as u64) as *mut u16) = val as u16;
                    },
                    iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => unsafe {
                        *((r15 + adj_ea as u64) as *mut u8) = val as u8;
                    },
                    _ => unsafe {
                        *((r15 + adj_ea as u64) as *mut u32) = val;
                    },
                }
                if fs_adj != 0 && ctx.rescue_count <= 20 {
                    veh_log(&format!(
                        "[RESCUE] MOV fs:[0x{:08X}], reg = 0x{:08X} at guest 0x{:08X}",
                        adj_ea, val, guest_addr
                    ));
                }
            } else if op0 == iced_x86::OpKind::Register && op1 == iced_x86::OpKind::Memory {
                // MOV reg, [mem] (e.g., mov eax, fs:[0])
                let size = instr.memory_size();
                let val: u32 = match size {
                    iced_x86::MemorySize::UInt32 | iced_x86::MemorySize::Int32 => unsafe {
                        *((r15 + adj_ea as u64) as *const u32)
                    },
                    iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => unsafe {
                        *((r15 + adj_ea as u64) as *const u16) as u32
                    },
                    iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => unsafe {
                        *((r15 + adj_ea as u64) as *const u8) as u32
                    },
                    _ => unsafe { *((r15 + adj_ea as u64) as *const u32) },
                };
                rescue_write_reg(context, instr.op_register(0), val);
                if fs_adj != 0 && ctx.rescue_count <= 20 {
                    veh_log(&format!(
                        "[RESCUE] MOV reg, fs:[0x{:08X}] = 0x{:08X} at guest 0x{:08X}",
                        adj_ea, val, guest_addr
                    ));
                }
            } else if op0 == iced_x86::OpKind::Register && op1 == iced_x86::OpKind::Register {
                // MOV reg, reg
                let val = rescue_read_reg(context, instr.op_register(1));
                rescue_write_reg(context, instr.op_register(0), val);
            } else if op0 == iced_x86::OpKind::Memory
                && (op1 == iced_x86::OpKind::Immediate32
                    || op1 == iced_x86::OpKind::Immediate32to64)
            {
                // MOV [mem], imm32
                let val = instr.immediate32();
                unsafe {
                    *((r15 + adj_ea as u64) as *mut u32) = val;
                }
            } else if op0 == iced_x86::OpKind::Register
                && (op1 == iced_x86::OpKind::Immediate32
                    || op1 == iced_x86::OpKind::Immediate32to64
                    || op1 == iced_x86::OpKind::Immediate8to32)
            {
                // MOV reg, imm32
                let val = if op1 == iced_x86::OpKind::Immediate8to32 {
                    instr.immediate8to32() as u32
                } else {
                    instr.immediate32()
                };
                rescue_write_reg(context, instr.op_register(0), val);
            } else {
                if ctx.rescue_count <= 20 {
                    let mut fmt = String::new();
                    let mut output = iced_x86::IntelFormatter::new();
                    let _ = iced_x86::Formatter::format(&mut output, &instr, &mut fmt);
                    veh_log(&format!(
                        "[RESCUE] MOV UNHANDLED: guest 0x{:08X} instr={}",
                        guest_addr, fmt
                    ));
                }
            }
        }
        _ => {
            // Unknown rescue type — log and skip
            if ctx.rescue_count <= 20 {
                let mut fmt = String::new();
                let mut output = iced_x86::IntelFormatter::new();
                let _ = iced_x86::Formatter::format(&mut output, &instr, &mut fmt);
                veh_log(&format!(
                    "[RESCUE] SKIP: guest 0x{:08X} instr={}",
                    guest_addr, fmt
                ));
            }
        }
    }

    // Advance past the INT3
    context.Rip = code_base + host_offset as u64 + 1;
    VehResult::Handled
}

/// Compute the effective address for a memory operand using guest registers.
#[cfg(windows)]
fn rescue_compute_ea(
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    instr: &iced_x86::Instruction,
) -> u32 {
    let base = instr.memory_base();
    let index = instr.memory_index();
    let scale = instr.memory_index_scale();
    let disp = instr.memory_displacement32();

    let mut ea = disp;
    if base != iced_x86::Register::None {
        ea = ea.wrapping_add(rescue_read_reg(context, base));
    }
    if index != iced_x86::Register::None {
        ea = ea.wrapping_add(rescue_read_reg(context, index).wrapping_mul(scale as u32));
    }
    ea
}

/// Read a 32-bit register value from the x64 CONTEXT, mapping guest regs.
/// ESP maps to R14 (guest ESP), not RSP (host stack).
#[cfg(windows)]
fn rescue_read_reg(
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    reg: iced_x86::Register,
) -> u32 {
    use iced_x86::Register::*;
    match reg {
        EAX | AX | AL => context.Rax as u32,
        ECX | CX | CL => context.Rcx as u32,
        EDX | DX | DL => context.Rdx as u32,
        EBX | BX | BL => context.Rbx as u32,
        ESP | SP => context.R14 as u32, // Guest ESP is in R14
        EBP | BP => context.Rbp as u32,
        ESI | SI => context.Rsi as u32,
        EDI | DI => context.Rdi as u32,
        _ => 0,
    }
}

/// Write a 32-bit value to a register in the x64 CONTEXT.
///
/// Thin wrapper around `veh::write_reg64` (E4 consolidation, 2026-04-22).
/// The previous independent implementation only handled the 32-bit form
/// of each register and used the same body for AL/AX/EAX — which was
/// effectively an EAX write for every width, matching veh::write_reg64's
/// pre-E4 bug. Routing through write_reg64 gives both paths correct
/// per-width semantics (8-bit / 16-bit preserve upper bits; 32-bit
/// zero-extends).
///
/// Only special case: guest ESP lives in R14, not context.Rsp.
#[cfg(windows)]
fn rescue_write_reg(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    reg: iced_x86::Register,
    val: u32,
) {
    use iced_x86::Register::*;
    match reg {
        // Guest ESP is pinned to R14 (not host RSP) per register contract.
        ESP | SP => context.R14 = val as u64,
        _ => super::veh::write_reg64(reg, context, val as u64),
    }
}

/// Update EFLAGS for INC (ZF, SF, OF, PF — CF unchanged).
#[cfg(windows)]
fn rescue_update_flags_inc(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    old: u32,
    new: u32,
) {
    let mut flags = context.EFlags;
    // ZF
    if new == 0 {
        flags |= 0x40;
    } else {
        flags &= !0x40;
    }
    // SF
    if (new as i32) < 0 {
        flags |= 0x80;
    } else {
        flags &= !0x80;
    }
    // OF: overflow if old was 0x7FFFFFFF (max positive → negative)
    if old == 0x7FFF_FFFF {
        flags |= 0x800;
    } else {
        flags &= !0x800;
    }
    context.EFlags = flags;
}

/// Update EFLAGS for DEC (ZF, SF, OF, PF — CF unchanged).
#[cfg(windows)]
fn rescue_update_flags_dec(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    old: u32,
    new: u32,
) {
    let mut flags = context.EFlags;
    // ZF
    if new == 0 {
        flags |= 0x40;
    } else {
        flags &= !0x40;
    }
    // SF
    if (new as i32) < 0 {
        flags |= 0x80;
    } else {
        flags &= !0x80;
    }
    // OF: overflow if old was 0x80000000 (min negative → positive)
    if old == 0x8000_0000 {
        flags |= 0x800;
    } else {
        flags &= !0x800;
    }
    context.EFlags = flags;
}

// ============================================================================
// Diagnostic ring buffer — tracks last 32 VEH events before exit
// ============================================================================

const RING_SIZE: usize = 32;

#[derive(Clone, Copy)]
struct VehEvent {
    guest_addr: u32,
    target: u32,    // kernel ordinal or indirect target
    event_type: u8, // 0=kernel, 1=indirect_call, 2=indirect_jmp, 3=ret, 4=ret_unresolved, 5=stack_scan
    esp: u32,
}

static mut RING_BUF: [VehEvent; RING_SIZE] = [VehEvent {
    guest_addr: 0,
    target: 0,
    event_type: 0,
    esp: 0,
}; RING_SIZE];
static RING_IDX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn record_event(guest_addr: u32, target: u32, event_type: u8, esp: u32) {
    let idx = RING_IDX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    unsafe {
        RING_BUF[idx % RING_SIZE] = VehEvent {
            guest_addr,
            target,
            event_type,
            esp,
        };
    }
}

/// Check .text integrity using R15 from VEH context. Call every N events.
fn check_text_integrity(r15: u64, event_idx: usize, guest_addr: u32, esp: u32) {
    if r15 == 0 {
        return;
    }
    let p1 = unsafe { *((r15 as usize + 0x11000) as *const u32) };
    let p2 = unsafe { *((r15 as usize + 0x20000) as *const u32) };
    let p3 = unsafe { *((r15 as usize + 0x100000) as *const u32) };
    if p1 == 0 && p2 == 0 && p3 == 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[TEXT-ZERO-VEH] .text zeroed at event ~{} g=0x{:08X} esp=0x{:08X}",
            event_idx, guest_addr, esp
        ));
    }
}

fn dump_recent_activity(reason: &str) {
    let total = RING_IDX.load(std::sync::atomic::Ordering::Relaxed);
    let count = total.min(RING_SIZE);
    let start = if total >= RING_SIZE {
        total % RING_SIZE
    } else {
        0
    };

    veh_log(&format!(
        "[DIAG] {} — last {} VEH events (total={}):",
        reason, count, total
    ));
    for i in 0..count {
        let idx = (start + i) % RING_SIZE;
        let ev = unsafe { RING_BUF[idx] };
        let type_str = match ev.event_type {
            0 => "KERNEL",
            1 => "IND_CALL",
            2 => "IND_JMP",
            3 => "RET",
            4 => "RET_UNRES",
            5 => "STACK_SCAN",
            _ => "???",
        };
        veh_log(&format!(
            "[DIAG]   #{}: {} guest={} target={} esp=0x{:08X}",
            total - count + i,
            type_str,
            crate::xbox::logging::fmt_guest_addr(ev.guest_addr),
            crate::xbox::logging::fmt_guest_addr(ev.target),
            ev.esp
        ));
    }
}

// repair_zeroed_thunk moved to ThunkTable::repair_at_site() in xbe.rs
