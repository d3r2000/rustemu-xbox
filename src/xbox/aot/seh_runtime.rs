/// Wine-style SEH runtime frame registry.
///
/// Tracks _SEH_prolog / _SEH_epilog frames explicitly so prolog and epilog
/// are guaranteed to be exact inverses. Replaces ad-hoc stack math with
/// structured frame tracking.
///
/// Design from CLAUDE.md: "represent the exception-registration chain and
/// frame contract explicitly, make prolog and epilog exact inverses."
use std::cell::RefCell;

/// A single SEH frame record, pushed by prolog, popped by epilog.
#[derive(Debug, Clone)]
pub struct SehFrame {
    pub guest_esp_at_entry: u32, // ESP when _SEH_prolog was called
    pub ebp: u32,                // EBP set by prolog (frame pointer)
    pub old_seh: u32,            // previous fs:[0] value
    pub handler: u32,            // exception handler address
    pub scope_table: u32,        // scope table pointer
    pub frame_size: u32,         // dynamic stack allocation size
    pub caller_ret_addr: u32,    // return address to caller
    pub saved_ebx: u32,
    pub saved_esi: u32,
    pub saved_edi: u32,
}

/// Per-thread frame stack.
thread_local! {
    pub static FRAME_STACK: RefCell<Vec<SehFrame>> = RefCell::new(Vec::with_capacity(32));
    static TOTAL_PROLOG: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static TOTAL_EPILOG: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    /// The __except_handler3 address, read from _SEH_prolog's `push <handler>` instruction.
    static PROLOG_HANDLER: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Set the handler address for _SEH_prolog (lazy, only reads once per thread).
/// Reads the 4-byte handler address from `push <handler>` at prolog_guest_addr+1.
pub fn set_prolog_handler_lazy(r15: u64, prolog_guest_addr: u32) {
    PROLOG_HANDLER.with(|h| {
        if h.get() == 0 {
            let handler = unsafe { *((r15 + prolog_guest_addr as u64 + 1) as *const u32) };
            h.set(handler);
            crate::xbox::emulator::debug_log(&format!(
                "[SEH] _SEH_prolog handler = 0x{:08X} (read from guest 0x{:08X}+1)",
                handler, prolog_guest_addr
            ));
        }
    });
}

/// Push a new SEH frame (called from _SEH_prolog HLE handler).
///
/// At _SEH_prolog entry, guest stack is:
///   [ESP]   = return address (to caller, after the `call _SEH_prolog`)
///   [ESP+4] = scope_table (pushed by caller before the call)
///   [ESP+8] = frame_size  (pushed by caller before the call)
///
/// The caller's code before `call _SEH_prolog` looks like:
///   push <frame_size>
///   push <scope_table_addr>
///   call _SEH_prolog
///
/// _SEH_prolog does:
///   push 0x002B7750        ; handler
///   mov eax, fs:[0]        ; old SEH chain
///   push eax
///   mov fs:[0], esp        ; link new frame
///   mov eax, [esp+0x10]    ; scope_table (from caller's push)
///   mov [ebp+0x10], ebp... ; complex stack setup
///   lea ebp, [esp+0x10]    ; EBP = frame pointer
///   sub esp, <frame_size>  ; allocate local vars
///   push ebx, esi, edi     ; save callee-saved regs
///   mov [ebp-0x18], esp    ; saved ESP
///   ret                    ; return to caller (after call _SEH_prolog)
#[cfg(windows)]
pub fn push_frame(context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT, r15: u64) {
    let esp = context.R14 as u32;
    let mem = |addr: u32| -> u32 { unsafe { *((r15 + addr as u64) as *const u32) } };
    let write = |addr: u32, val: u32| unsafe {
        *((r15 + addr as u64) as *mut u32) = val;
    };

    // Replicate the EXACT x86 _SEH_prolog at 0x002B7B28 step by step.
    //
    // Entry state: caller did "push frame_size; push scope_table; call _SEH_prolog"
    // So stack is: [ESP]=ret_addr, [ESP+4]=scope_table, [ESP+8]=frame_size,
    //              [ESP+C]=caller_ret_addr, [ESP+10]=caller_arg0, ...
    //
    // Real x86 instructions:
    //   002b7b28: push 0x2B7750           ; handler
    //   002b7b2d: mov eax, fs:[0]         ; old SEH chain
    //   002b7b33: push eax                ; push old SEH
    //   002b7b34: mov fs:[0], esp         ; link SEH frame
    //   002b7b3b: mov eax, [esp+0x10]     ; eax = frame_size (at new offset)
    //   002b7b3f: mov [esp+0x10], ebp     ; OVERWRITE frame_size slot with old EBP
    //   002b7b43: lea ebp, [esp+0x10]     ; EBP = &old_EBP_slot
    //   002b7b47: sub esp, eax            ; allocate locals
    //   002b7b49: push ebx
    //   002b7b4a: push esi
    //   002b7b4b: push edi
    //   002b7b4c: mov eax, [ebp-8]        ; eax = ret_addr (original [ESP+0])
    //   002b7b4f: mov [ebp-0x18], esp     ; save ESP
    //   002b7b52: push eax                ; push ret_addr for the `ret` below
    //   002b7b53: mov eax, [ebp-4]        ; eax = scope_table (original [ESP+4])
    //   002b7b56: mov [ebp-4], -1         ; try_level = -1
    //   002b7b5d: mov [ebp-8], eax        ; move scope_table to [EBP-8]
    //   002b7b60: ret                     ; jump to ret_addr

    let ret_addr = mem(esp); // [ESP+0] = return address
    let scope_table = mem(esp + 4); // [ESP+4] = scope_table
    let frame_size = mem(esp + 8); // [ESP+8] = frame_size

    // fs:[0] maps to [R15 + 0x0C000000] (emitter adds FAKE_KPCR_BASE to fs: accesses)
    const FS_ZERO: u32 = 0x0C00_0000;
    let old_seh = mem(FS_ZERO); // fs:[0]
    let old_ebp = context.Rbp as u32;

    // Read the handler address from the caller's push instruction.
    // The caller does: push frame_size; push scope_table; call _SEH_prolog
    // But _SEH_prolog itself starts with: push <handler_addr>
    // The handler addr is the first PUSH immediate in _SEH_prolog.
    // We read it from [ESP+0x0C] — the caller_ret_addr slot holds the address
    // after `call _SEH_prolog`. The handler address was pushed INSIDE _SEH_prolog,
    // not by the caller. We need to read the actual push value from the guest code.
    //
    // Since we intercept at _SEH_prolog ENTRY (before push handler executes),
    // the handler addr isn't on the stack yet. Read it from the code bytes:
    // _SEH_prolog starts with: 68 xx xx xx xx (push imm32) → bytes at [guest_eip+1..5]
    // But we don't have guest_eip here. Use the scope_table's parent entry instead.
    // Actually simpler: read [ESP-4] or check what _SEH_prolog would push.
    // The push opcode is at the hook address. We stored the hook address in the OOVPA
    // match. For now, read the 4 bytes after the 0x68 push opcode at the prolog entry.
    // HACK: read from guest memory at (ret_addr - 5 - 18 + 1) = approximate.
    // Better: store the handler in a known location. For now use the scope_table
    // which points to a __except_handler3 reference.
    //
    // Read the handler address from _SEH_prolog's first instruction:
    // push <handler_addr> → opcode 0x68 followed by 4-byte handler address.
    // The _SEH_prolog guest address is stored in our OOVPA match. We read the
    // handler from the PROLOG_GUEST_ADDR + 1 (skip the 0x68 push opcode).
    let handler_addr = PROLOG_HANDLER.with(|h| h.get());

    // Simulate the pushes: push handler, push old_seh
    let mut sp = esp;
    sp -= 4;
    write(sp, handler_addr); // push handler (unused — VEH handles)
    sp -= 4;
    write(sp, old_seh); // push old SEH chain
    write(FS_ZERO, sp); // mov fs:[0], esp

    // Now stack is: [sp]=old_seh, [sp+4]=handler, [sp+8]=ret_addr,
    //               [sp+C]=scope_table, [sp+10]=frame_size, [sp+14]=caller_ret, ...

    // mov eax, [esp+0x10] → eax = frame_size
    let fsize = mem(sp + 0x10); // should == frame_size
                                // mov [esp+0x10], ebp → overwrite frame_size with old EBP
    write(sp + 0x10, old_ebp);
    // lea ebp, [esp+0x10]
    let ebp = sp + 0x10;

    // After lea ebp:
    //   [EBP-0x10] = old_seh       (sp+0)
    //   [EBP-0x0C] = handler       (sp+4)
    //   [EBP-0x08] = ret_addr      (sp+8)
    //   [EBP-0x04] = scope_table   (sp+C)
    //   [EBP+0x00] = old_EBP       (sp+10, was frame_size)
    //   [EBP+0x04] = caller_ret    (sp+14)
    //   [EBP+0x08] = caller_arg0   (sp+18) ← first arg from CALLER's caller
    //   [EBP+0x0C] = caller_arg1
    //   [EBP+0x10] = caller_arg2   ← this is what [ebp+0x10] reads as "count"

    // sub esp, eax (= frame_size)
    sp = ebp - 0x10; // current ESP after lea = ebp - 0x10
    sp = sp.wrapping_sub(fsize);

    // push ebx, esi, edi
    let saved_ebx = context.Rbx as u32;
    let saved_esi = context.Rsi as u32;
    let saved_edi = context.Rdi as u32;
    sp -= 4;
    write(sp, saved_ebx);
    sp -= 4;
    write(sp, saved_esi);
    sp -= 4;
    write(sp, saved_edi);

    // mov [ebp-0x18], esp
    write(ebp.wrapping_sub(0x18), sp);

    // Now the shuffle: the real prolog swaps ret_addr and scope_table
    // mov eax, [ebp-8]  → eax = ret_addr
    // push eax           → for the ret at end (we don't need this, we jump directly)
    // mov eax, [ebp-4]  → eax = scope_table
    // mov [ebp-4], -1    → try_level = -1
    // mov [ebp-8], eax   → scope_table moves to [ebp-8]
    write(ebp.wrapping_sub(4), 0xFFFF_FFFF); // [EBP-4] = try_level = -1
    write(ebp.wrapping_sub(8), scope_table); // [EBP-8] = scope_table (was ret_addr)

    let new_esp = sp;

    // Store frame record
    let frame = SehFrame {
        guest_esp_at_entry: esp,
        ebp,
        old_seh,
        handler: handler_addr,
        scope_table,
        frame_size,
        caller_ret_addr: ret_addr,
        saved_ebx,
        saved_esi,
        saved_edi,
    };

    FRAME_STACK.with(|fs| fs.borrow_mut().push(frame));
    let count = TOTAL_PROLOG.with(|c| {
        let v = c.get() + 1;
        c.set(v);
        v
    });

    // Update context
    context.Rbp = ebp as u64;
    context.R14 = new_esp as u64;

    if count <= 5 {
        crate::xbox::emulator::debug_log(&format!(
            "[SEH-WINE] PROLOG #{} EBP=0x{:08X} ESP=0x{:08X}→0x{:08X} frame=0x{:X} ret=0x{:08X}",
            count, ebp, esp, new_esp, frame_size, ret_addr
        ));
    }

    // R12 pop DISABLED — emit_call_rel no longer pushes to R12.

    // Return to caller (ret_addr = instruction after `call _SEH_prolog`)
    // The caller continues with EBP set up and locals allocated.
}

/// Pop the SEH frame (called from _SEH_epilog HLE handler).
///
/// _SEH_epilog does:
///   mov ecx, [ebp-10h]  ; old SEH chain
///   mov fs:[0], ecx     ; restore chain
///   pop ecx             ; pop return addr into ECX
///   pop edi, esi, ebx   ; restore callee-saved
///   leave               ; ESP=EBP, pop EBP
///   push ecx; ret       ; jump to return addr (push+ret = jmp ecx)
#[cfg(windows)]
pub fn pop_frame(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    r15: u64,
) -> u32 {
    let esp = context.R14 as u32;
    let ebp = context.Rbp as u32;
    let mem = |addr: u32| -> u32 { unsafe { *((r15 + addr as u64) as *const u32) } };
    let write = |addr: u32, val: u32| unsafe {
        *((r15 + addr as u64) as *mut u32) = val;
    };

    let count = TOTAL_EPILOG.with(|c| {
        let v = c.get() + 1;
        c.set(v);
        v
    });

    // Try to pop from frame stack for validation
    let frame = FRAME_STACK.with(|fs| fs.borrow_mut().pop());

    // 1. Restore SEH chain: fs:[0] = [EBP-0x10]
    const FS_ZERO: u32 = 0x0C00_0000;
    let cur_seh = mem(FS_ZERO);
    let old_seh = mem(ebp.wrapping_sub(0x10));
    if count <= 10 || count % 10000 == 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[SEH-EPILOG] #{} EBP=0x{:08X} [EBP-0x10]=0x{:08X} cur_fs0=0x{:08X} → writing 0x{:08X}",
            count,
            ebp,
            ebp.wrapping_sub(0x10),
            cur_seh,
            old_seh
        ));
    }
    write(FS_ZERO, old_seh);

    // 2. Pop return addr: ECX = [ESP], ESP += 4
    let ecx = mem(esp);
    let mut new_esp = esp + 4;

    // 3. Pop EDI, ESI, EBX
    context.Rdi = mem(new_esp) as u64;
    new_esp += 4;
    context.Rsi = mem(new_esp) as u64;
    new_esp += 4;
    context.Rbx = mem(new_esp) as u64;
    new_esp += 4;

    // 4. LEAVE: ESP = EBP, pop EBP
    new_esp = ebp;
    let new_ebp = mem(new_esp);
    new_esp += 4;
    context.Rbp = new_ebp as u64;

    // 5. Update ESP (push ecx; ret → equivalent to jmp ecx)
    context.R14 = new_esp as u64;
    context.Rcx = ecx as u64;

    // R12 pop DISABLED — emit_call_rel no longer pushes to R12.

    if count <= 5 {
        let matched = frame.as_ref().map(|f| f.ebp == ebp).unwrap_or(false);
        crate::xbox::emulator::debug_log(&format!(
            "[SEH-WINE] EPILOG #{} EBP=0x{:08X}→0x{:08X} ESP=0x{:08X}→0x{:08X} ret=0x{:08X} matched={}",
            count, ebp, new_ebp, esp, new_esp, ecx, matched
        ));
    }

    // Validate frame symmetry
    if let Some(f) = &frame {
        if f.ebp != ebp {
            static MISMATCH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = MISMATCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::emulator::debug_log(&format!(
                    "[SEH-WINE] MISMATCH #{}: prolog EBP=0x{:08X} epilog EBP=0x{:08X}",
                    n, f.ebp, ebp
                ));
            }
        }
    }

    // Return the jump target (ECX = shuffled return address)
    ecx
}

/// Get frame stack depth (for HUD telemetry).
pub fn frame_depth() -> usize {
    FRAME_STACK.with(|fs| fs.borrow().len())
}

/// Get total prolog/epilog counts.
pub fn counts() -> (u64, u64) {
    let p = TOTAL_PROLOG.with(|c| c.get());
    let e = TOTAL_EPILOG.with(|c| c.get());
    (p, e)
}
