/// AOT trampoline — asm! entry/exit between host and guest code.
/// R13 = &RuntimeContext (GuestState at offset 0)
/// R15 = guest memory base, R14 = guest ESP, R12 = shadow stack pointer.
use crate::xbox::aot::runtime::RuntimeContext;

/// Enter guest code. Returns exit_reason when guest traps out.
///
/// RSP is switched to a dedicated host_stack buffer (8MB) during guest execution.
/// This isolates VEH exception dispatch stack usage from the OS thread stack.
/// On exit, RSP is restored to the OS thread stack.
///
/// # Safety
/// `entry_host` must point into an executable code buffer.
/// `ctx` must be a valid RuntimeContext with code_base, guest state, etc. initialized.
#[cfg(target_arch = "x86_64")]
pub unsafe fn enter_guest(ctx: &mut RuntimeContext, entry_host: *const u8) -> u32 {
    // Compute top of host_stack (stack grows down, so start at the end)
    // Align to 16 bytes for ABI compliance, minus 8 for the push alignment
    let host_stack_top: u64 =
        ((ctx.host_stack.as_ptr() as u64 + ctx.host_stack.len() as u64) & !0xF) - 8;

    core::arch::asm!(
        // ---- ENTER: save host state, switch stacks, load guest state ----
        // Save callee-saved regs on the OS thread stack
        "push rbx",
        "push rbp",

        // Save OS thread RSP (after pushes) for exit path restoration
        "mov [r13 + 0x00], rsp",           // GuestState.host_rsp = OS stack ptr

        // Use the OS thread stack directly (256MB via thread::Builder::stack_size).
        // Previously switched to a 32MB host_stack buffer which overflowed on
        // deeply nested D3D calls. The OS thread stack is large enough.
        // "mov rsp, r8",  // DISABLED — use OS stack instead

        // Set up exit label address and clear exit_reason
        "lea rax, [rip + 99f]",            // address of exit label
        "mov [r13 + 0x08], rax",           // exit_addr = &exit_label
        "mov dword ptr [r13 + 0x30], 0",   // exit_reason = RUNNING

        // Save entry address on host_stack BEFORE loading guest regs.
        "push {entry}",

        // Load pinned registers
        "mov r15, [r13 + 0x38]",           // R15 = guest memory base
        "mov r14d, [r13 + 0x20]",          // R14 = guest ESP
        "mov r12, [r13 + 0x40]",           // R12 = shadow stack pointer

        // Load guest GPRs (safe — entry address is on the stack)
        "mov eax, [r13 + 0x10]",
        "mov ecx, [r13 + 0x14]",
        "mov edx, [r13 + 0x18]",
        "mov ebx, [r13 + 0x1C]",
        "mov ebp, [r13 + 0x24]",
        "mov esi, [r13 + 0x28]",
        "mov edi, [r13 + 0x2C]",

        // Pop entry address and jump there (ret = pop + jmp)
        "ret",

        // ---- EXIT: save guest state, restore OS stack ----
        "99:",
        "mov [r13 + 0x10], eax",
        "mov [r13 + 0x14], ecx",
        "mov [r13 + 0x18], edx",
        "mov [r13 + 0x1C], ebx",
        "mov [r13 + 0x20], r14d",          // save guest ESP
        "mov [r13 + 0x24], ebp",
        "mov [r13 + 0x28], esi",
        "mov [r13 + 0x2C], edi",
        "mov [r13 + 0x40], r12",           // save shadow stack pointer

        // Restore OS thread RSP (points to pushed rbp, rbx on OS stack)
        "mov rsp, [r13 + 0x00]",
        "pop rbp",
        "pop rbx",

        entry = in(reg) entry_host,
        inlateout("r8") host_stack_top => _,
        inout("r13") (ctx as *mut RuntimeContext) => _,
        // All registers the guest code may use
        out("r12") _, out("r14") _, out("r15") _,
        out("rax") _, out("rcx") _, out("rdx") _,
        out("rsi") _, out("rdi") _,
        out("r9") _, out("r10") _, out("r11") _,
        // Win64 callee-saved XMMs (must be declared as clobbered)
        out("xmm6") _, out("xmm7") _, out("xmm8") _, out("xmm9") _,
        out("xmm10") _, out("xmm11") _, out("xmm12") _, out("xmm13") _,
        out("xmm14") _, out("xmm15") _,
    );

    ctx.guest.exit_reason
}
