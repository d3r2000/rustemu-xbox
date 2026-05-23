/// VEH Orchestrator — entry guards, routing, crash handler, diagnostics [L2-AV-DROP]
///
/// 5-file VEH architecture (matching C++ AOT_VEH split):
///   veh.rs          — Orchestrator: entry guards, routing, crash handler [L2-AV-DROP]
///   veh_mmio.rs     — NV2A MMIO decode (GPU register read/write)        [NV2A-DATA]
///   veh_fixup.rs    — R15 fixups: sign-ext, overflow, string, RAM wrap  [L2-TAG]
///   veh_dispatch.rs — INT3 trap switch: RET, INDIRECT+KERNEL, SYSTEM    [L3-ROUTE]
///   veh_dpc.rs      — ISR/DPC injection + dispatch counter (stub)       [L4-DPC-*]
///
/// Sub-handlers return VehResult::Handled or VehResult::NotHandled.
/// The orchestrator translates Handled → EXCEPTION_CONTINUE_EXECUTION.

// ============================================================================
// rate_log! — replaces 20+ scattered static AtomicU32 + fetch_add patterns.
// Each call site gets its own counter. Format args are only evaluated when
// the counter is below the limit (avoids allocating dead strings).
// Returns the counter value so callers can do extra work conditionally.
// ============================================================================
/// Rate-limited VEH logging. Usage: `rate_log!(50, "[TAG] msg {}", val);`
#[macro_export]
macro_rules! rate_log {
    ($limit:expr, $($arg:tt)*) => {{
        static __RATE_CTR: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let __n = __RATE_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if __n < $limit {
            $crate::xbox::aot::veh::veh_log(&format!($($arg)*));
        }
        __n
    }};
}

// ============================================================================
// Hardware watchpoint helpers (DR0/DR7) for catching AOT-emitted writers.
// ============================================================================

/// One-shot arm of DR0 hardware watchpoint on the CURRENT thread, for writes
/// to `watch_host_addr` (4-byte aligned, 4-byte length). Writes that hit this
/// address will raise EXCEPTION_SINGLE_STEP with Dr6 bit 0 set, and the
/// orchestrator's EXCEPTION_SINGLE_STEP handler will log RIP and disarm.
#[cfg(windows)]
pub fn arm_hw_watchpoint(watch_host_addr: u64) {
    use windows::Win32::System::Diagnostics::Debug::*;
    use windows::Win32::System::Threading::GetCurrentThread;
    static ARMED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if ARMED.swap(true, std::sync::atomic::Ordering::AcqRel) {
        return; // already armed
    }
    unsafe {
        let h = GetCurrentThread();
        let mut ctx: CONTEXT = std::mem::zeroed();
        // CONTEXT_DEBUG_REGISTERS_AMD64 flag is 0x00100010 on x64.
        ctx.ContextFlags = CONTEXT_FLAGS(0x0010_0010);
        if GetThreadContext(h, &mut ctx).is_err() {
            ARMED.store(false, std::sync::atomic::Ordering::Release);
            return;
        }
        ctx.Dr0 = watch_host_addr;
        ctx.Dr1 = 0;
        ctx.Dr2 = 0;
        ctx.Dr3 = 0;
        // DR7 bits:
        //   L0=1 (bit 0)  — local enable DR0
        //   G0=1 (bit 1)  — global enable (ignored on NT but harmless)
        //   RW0=01 (bits 16-17) — break on data write only
        //   LEN0=11 (bits 18-19) — 4-byte length
        //   LE=1 (bit 8), GE=1 (bit 9) — local/global exact
        ctx.Dr7 = (1 << 0) | (1 << 1)     // L0, G0
                | (1 << 8) | (1 << 9)     // LE, GE
                | (0b01 << 16)            // RW0 = write-only
                | (0b11 << 18); // LEN0 = 4 bytes
        if SetThreadContext(h, &ctx).is_ok() {
            crate::xbox::emulator::debug_log(&format!(
                "[HW-WATCH] DR0 armed at host 0x{:016X} (Dr7=0x{:X})",
                watch_host_addr, ctx.Dr7
            ));
        } else {
            ARMED.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

/// Replace DR0 on the current thread even if another diagnostic armed it first.
/// Intended for narrow one-off probes where the caller has already filtered the
/// target and wants the next writer more than any older watchpoint.
#[cfg(windows)]
pub fn replace_hw_watchpoint(watch_host_addr: u64, tag: &str) {
    use windows::Win32::System::Diagnostics::Debug::*;
    use windows::Win32::System::Threading::GetCurrentThread;
    unsafe {
        let h = GetCurrentThread();
        let mut ctx: CONTEXT = std::mem::zeroed();
        ctx.ContextFlags = CONTEXT_FLAGS(0x0010_0010);
        if GetThreadContext(h, &mut ctx).is_err() {
            return;
        }
        ctx.Dr0 = watch_host_addr;
        ctx.Dr1 = 0;
        ctx.Dr2 = 0;
        ctx.Dr3 = 0;
        ctx.Dr7 = (1 << 0) | (1 << 1) | (1 << 8) | (1 << 9) | (0b01 << 16) | (0b11 << 18);
        if SetThreadContext(h, &ctx).is_ok() {
            crate::xbox::emulator::debug_log(&format!(
                "[HW-WATCH] DR0 replaced by {} at host 0x{:016X} (Dr7=0x{:X})",
                tag, watch_host_addr, ctx.Dr7
            ));
        }
    }
}

// ============================================================================
// Guest memory helpers — replace raw `*((r15 + offset as u64) as *const u32)`
// with named functions. Debug-mode bounds check catches wild offsets early.
// ============================================================================

/// Read a 32-bit value from guest memory at `[r15 + offset]`.
/// # Safety
/// Caller must ensure `offset` maps to committed guest memory.
#[inline(always)]
pub unsafe fn guest_read_u32(r15: u64, offset: u32) -> u32 {
    debug_assert!(
        offset < 0x4000_0000
            || (offset >= 0x8000_0000 && offset < 0xA000_0000)
            || (offset >= 0xFD00_0000 && offset < 0xFF00_0000),
        "guest_read_u32: suspicious offset 0x{:08X}",
        offset
    );
    *((r15 + offset as u64) as *const u32)
}

/// Read an 8-bit value from guest memory at `[r15 + offset]`.
#[inline(always)]
pub unsafe fn guest_read_u8(r15: u64, offset: u32) -> u8 {
    *((r15 + offset as u64) as *const u8)
}

/// Write a 32-bit value to guest memory at `[r15 + offset]`.
/// # Safety
/// Caller must ensure `offset` maps to committed, writable guest memory.
#[inline(always)]
pub unsafe fn guest_write_u32(r15: u64, offset: u32, val: u32) {
    *((r15 + offset as u64) as *mut u32) = val;
}

/// Read a 64-bit value from a host pointer (shadow stack, etc.).
/// # Safety
/// Caller must ensure `ptr` is valid and aligned.
#[inline(always)]
pub unsafe fn host_read_u64(ptr: u64) -> u64 {
    *(ptr as *const u64)
}

#[cfg(windows)]
use std::sync::atomic::{AtomicPtr, Ordering};

#[cfg(windows)]
use crate::xbox::aot::runtime::{RuntimeContext, AOT_EXIT_ERROR, AOT_EXIT_RET_TO_ZERO};

/// Sub-handler return type — prevents EXCEPTION_CONTINUE_EXECUTION/-1 confusion.
#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehResult {
    Handled,
    NotHandled,
}

/// Global fallback context (used by main thread and for install/remove lifecycle).
#[cfg(windows)]
static VEH_CONTEXT: AtomicPtr<RuntimeContext> = AtomicPtr::new(std::ptr::null_mut());

#[cfg(windows)]
static VEH_HANDLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Global VEH dispatch counter — counts ALL VEH handler entries across all threads.
/// Matches C++ g_veh_dispatch_count which counts every VEH event (kernel calls,
/// access violations, R15 fixups, MMIO, rescues — not just kernel calls).
#[cfg(windows)]
static VEH_DISPATCH_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Global stop flag — set by unload to force worker threads out of guest code.
/// Checked on every VEH entry. Matches C++ g_aot_stop_requested.
#[cfg(windows)]
static AOT_STOP_REQUESTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Request all guest threads to stop (called from unload before join).
pub fn request_stop() {
    #[cfg(windows)]
    AOT_STOP_REQUESTED.store(true, Ordering::Release);
}

/// Check if stop has been requested.
pub fn is_stop_requested() -> bool {
    AOT_STOP_REQUESTED.load(Ordering::Acquire)
}

/// Worker completion flag — set when worker's first dispatch exits with RET_TO_ZERO.
/// The main thread's NtWaitForSingleObject polls this to know when the worker is done.
#[cfg(windows)]
static WORKER_DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Signal that the worker has completed its initial run.
pub fn signal_worker_done() {
    #[cfg(windows)]
    {
        WORKER_DONE.store(true, Ordering::Release);
        // Also signal the Windows Event so NtWaitForSingleObject unblocks
        let h = crate::xbox::emulator::WORKER_DONE_EVENT.load(Ordering::Relaxed);
        if h != 0 {
            use windows::Win32::Foundation::HANDLE;
            use windows::Win32::System::Threading::SetEvent;
            unsafe {
                SetEvent(HANDLE(h as *mut _)).ok();
            }
            crate::xbox::emulator::debug_log(
                "[WORKER] Done event SIGNALED — main thread will unblock",
            );
        }
    }
}

/// Check if the worker has completed its initial run.
pub fn is_worker_done() -> bool {
    #[cfg(windows)]
    {
        WORKER_DONE.load(Ordering::Acquire)
    }
    #[cfg(not(windows))]
    {
        true
    }
}

/// Clear stop flag (called from install_veh at start).
pub fn clear_stop() {
    #[cfg(windows)]
    AOT_STOP_REQUESTED.store(false, Ordering::Release);
}

/// Read the global VEH dispatch count (for stage display).
pub fn take_veh_dispatch_count() -> u64 {
    #[cfg(windows)]
    {
        VEH_DISPATCH_COUNT.load(Ordering::Relaxed)
    }
    #[cfg(not(windows))]
    {
        0
    }
}

/// Increment VEH dispatch counter (called from orchestrator on every VEH entry).
#[cfg(windows)]
pub(crate) fn bump_veh_dispatch() {
    VEH_DISPATCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Per-thread RuntimeContext pointer for multi-thread VEH support.
#[cfg(windows)]
thread_local! {
    static THREAD_VEH_CTX: std::cell::Cell<*mut RuntimeContext> = const { std::cell::Cell::new(std::ptr::null_mut()) };
    /// Re-entrancy depth counter: allows up to 5 nested VEH handlers.
    static VEH_ACTIVE: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Set the per-thread VEH context. Call before entering guest code on any thread.
#[cfg(windows)]
pub fn set_thread_context(ctx: *mut RuntimeContext) {
    THREAD_VEH_CTX.with(|c| c.set(ctx));
}

/// Clear the per-thread VEH context. Call when a thread exits guest execution.
#[cfg(windows)]
pub fn clear_thread_context() {
    THREAD_VEH_CTX.with(|c| c.set(std::ptr::null_mut()));
}

/// Get the per-thread VEH context pointer (may be null).
#[cfg(windows)]
pub fn get_thread_context() -> *mut RuntimeContext {
    THREAD_VEH_CTX.with(|c| c.get())
}

// ============================================================================
// Trace ring buffer — last 128 VEH events, dumped on crash
// ============================================================================

/// One entry in the VEH trace ring — packed into two u64 atomics for lock-free access.
/// Word0: [kind:8][guest_pc:32][esp_lo24:24] = 64 bits
/// Word1: [eax:32][esi:32] = 64 bits
/// This is approximate (lossy) but zero-overhead.
use std::sync::atomic::AtomicU64;

const TRACE_RING_SIZE: usize = 128;
const TRACE_RING_MASK: usize = TRACE_RING_SIZE - 1;
static TRACE_RING_W0: [AtomicU64; TRACE_RING_SIZE] = {
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; TRACE_RING_SIZE]
};
static TRACE_RING_W1: [AtomicU64; TRACE_RING_SIZE] = {
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; TRACE_RING_SIZE]
};
static TRACE_RING_IDX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Record a trace entry — lock-free, ~2 atomic stores per call.
#[cfg(windows)]
#[inline(always)]
fn trace_record(
    kind: u8,
    guest_pc: u32,
    ctx: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    extra: u32,
) {
    let idx = TRACE_RING_IDX.fetch_add(1, std::sync::atomic::Ordering::Relaxed) & TRACE_RING_MASK;
    // Pack: kind(8) | guest_pc(32) | extra_lo24(24)
    let w0 = ((kind as u64) << 56) | ((guest_pc as u64) << 24) | ((extra as u64) & 0xFF_FFFF);
    // Pack: eax(32) | esi(32)
    let w1 = ((ctx.Rax as u64 & 0xFFFF_FFFF) << 32) | (ctx.Rsi as u64 & 0xFFFF_FFFF);
    TRACE_RING_W0[idx].store(w0, std::sync::atomic::Ordering::Relaxed);
    TRACE_RING_W1[idx].store(w1, std::sync::atomic::Ordering::Relaxed);

    // Hotspot histogram (2026-04-21 fix). For 'I' events (INT3 traps),
    // `guest_pc` is always 0 and `extra` is the host_offset of the trap
    // site — which is exactly what we want to histogram to find tight
    // loops. For 'A' events (AVs), `guest_pc` is the faulting guest PC.
    // Use whichever is non-zero.
    let hot_key = if guest_pc != 0 && guest_pc != 0xFFFF_FFFF {
        guest_pc
    } else {
        extra
    };
    if hot_key != 0 && hot_key != 0xFFFF_FFFF {
        let slot = ((hot_key >> 4) as usize) & (PC_HIST_BUCKETS - 1);
        PC_HIST[slot].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        PC_HIST_TOTAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

// Guest-PC hotspot histogram (2026-04-21, layer 5 diagnostic).
// Cumulative count of how many trace events recorded each guest-PC
// 16-byte bucket. Dumped on demand via pc_hist_dump(). Used to find
// tight loops in the worker during slow periods.
const PC_HIST_BUCKETS: usize = 4096;
static PC_HIST: [std::sync::atomic::AtomicU32; PC_HIST_BUCKETS] = {
    const Z: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    [Z; PC_HIST_BUCKETS]
};
static PC_HIST_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Dump the top-N guest-PC buckets from the histogram. Caller specifies
/// `top_n`. Bucket = guest_pc & ~0xF (16-byte aligned). Callable from
/// emulator.rs each N frames for periodic hotspot diagnosis.
#[cfg(windows)]
pub fn pc_hist_dump(top_n: usize) {
    let total = PC_HIST_TOTAL.load(std::sync::atomic::Ordering::Relaxed);
    if total == 0 {
        return;
    }
    let mut entries: Vec<(usize, u32)> = (0..PC_HIST_BUCKETS)
        .filter_map(|slot| {
            let c = PC_HIST[slot].load(std::sync::atomic::Ordering::Relaxed);
            if c > 0 {
                Some((slot, c))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    veh_log(&format!(
        "[PC-HIST] total={} unique_buckets={} top_{}:",
        total,
        entries.len(),
        top_n
    ));
    for (slot, hits) in entries.iter().take(top_n) {
        // Bucket × 16 gives the aligned address for this bucket. With the
        // 4096-bucket hash, collisions possible within a 64KB wrap.
        let addr = (*slot as u32) << 4;
        veh_log(&format!(
            "  bucket=0x{:04X} (guest_pc ≈ 0x{:08X}) hits={}",
            slot, addr, hits
        ));
    }
}

/// Dump the trace ring to debug.log (called on crash).
/// Also dumps Spider-Man pool globals at key addresses.
#[cfg(windows)]
fn trace_dump() {
    let total = TRACE_RING_IDX.load(std::sync::atomic::Ordering::Relaxed);
    if total == 0 {
        return;
    }
    let count = total.min(TRACE_RING_SIZE);
    let start = if total > TRACE_RING_SIZE {
        total - TRACE_RING_SIZE
    } else {
        0
    };
    veh_log(&format!(
        "=== TRACE RING DUMP ({} of {} events) ===",
        count, total
    ));
    for i in 0..count {
        let idx = (start + i) & TRACE_RING_MASK;
        let w0 = TRACE_RING_W0[idx].load(std::sync::atomic::Ordering::Relaxed);
        let w1 = TRACE_RING_W1[idx].load(std::sync::atomic::Ordering::Relaxed);
        let kind = (w0 >> 56) as u8 as char;
        let guest_pc = ((w0 >> 24) & 0xFFFF_FFFF) as u32;
        let extra = (w0 & 0xFF_FFFF) as u32;
        let eax = (w1 >> 32) as u32;
        let esi = w1 as u32;
        veh_log(&format!(
            "  [{}] {} g=0x{:08X} EAX=0x{:08X} ESI=0x{:08X} x=0x{:06X}",
            start + i,
            kind,
            guest_pc,
            eax,
            esi,
            extra
        ));
    }
    veh_log("=== END TRACE RING ===");
    // Dump Spider-Man pool globals
    let ctx_ptr = THREAD_VEH_CTX.with(|c| c.get());
    let ctx_ptr = if ctx_ptr.is_null() {
        VEH_CONTEXT.load(Ordering::Acquire)
    } else {
        ctx_ptr
    };
    if !ctx_ptr.is_null() {
        let ctx = unsafe { &*ctx_ptr };
        let r15 = ctx.guest_mem_base as u64;
        if r15 != 0 {
            unsafe {
                let pool_counter = *((r15 + 0x3F8C30) as *const u32);
                let pool_flag = *((r15 + 0x4B1DC4) as *const u32);
                let pool_desc0 = *((r15 + 0x3F8C3C) as *const u32);
                let pool_entry0 = *((r15 + 0x42EFBC) as *const u32);
                let buf_start = *((r15 + 0x43AFC0) as *const u32);
                veh_log(&format!(
                    "=== POOL STATE: counter=0x{:08X} flag=0x{:08X} desc[0]=0x{:08X} entry[0]=0x{:08X} buf=0x{:08X} ===",
                    pool_counter, pool_flag, pool_desc0, pool_entry0, buf_start
                ));
            }
        }
    }
}

/// Write a Windows minidump to disk. Called from VEH on crash/corruption.
#[cfg(windows)]
pub fn write_minidump(
    exception_pointers: *const windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS,
) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static DUMPED: AtomicBool = AtomicBool::new(false);
    if DUMPED.swap(true, Ordering::Relaxed) {
        return;
    } // one-shot

    use windows::core::*;
    use windows::Win32::Foundation::*;
    use windows::Win32::Storage::FileSystem::*;
    use windows::Win32::System::Diagnostics::Debug::*;
    use windows::Win32::System::Threading::*;

    let path = w!(r"./crash.dmp");
    let handle = unsafe {
        CreateFileW(
            path,
            GENERIC_WRITE.0,
            FILE_SHARE_NONE,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    };
    let handle = match handle {
        Ok(h) => h,
        Err(e) => {
            veh_log(&format!("[MINIDUMP] Failed to create crash.dmp: {}", e));
            return;
        }
    };

    let process = unsafe { GetCurrentProcess() };
    let pid = unsafe { GetCurrentProcessId() };

    let dump_type = MiniDumpNormal | MiniDumpWithThreadInfo;

    let exc_info = MINIDUMP_EXCEPTION_INFORMATION {
        ThreadId: unsafe { GetCurrentThreadId() },
        ExceptionPointers: exception_pointers as *mut _,
        ClientPointers: FALSE,
    };
    let exc_param: Option<*const MINIDUMP_EXCEPTION_INFORMATION> = if exception_pointers.is_null() {
        None
    } else {
        Some(&exc_info as *const _)
    };

    let result =
        unsafe { MiniDumpWriteDump(process, pid, handle, dump_type, exc_param, None, None) };

    unsafe {
        let _ = CloseHandle(handle);
    }

    match result {
        Ok(_) => veh_log("[MINIDUMP] Wrote crash.dmp"),
        Err(e) => veh_log(&format!("[MINIDUMP] MiniDumpWriteDump failed: {}", e)),
    }
}

#[cfg(windows)]
pub fn veh_log(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"./debug.log")
    {
        let _ = writeln!(f, "[VEH] {}", msg);
    }
}

#[cfg(windows)]
pub fn install_veh(ctx: &mut RuntimeContext) {
    use windows::Win32::System::Diagnostics::Debug::AddVectoredExceptionHandler;

    clear_stop(); // Reset stop flag for new session

    let ctx_ptr = ctx as *mut RuntimeContext;
    VEH_CONTEXT.store(ctx_ptr, Ordering::Release);
    set_thread_context(ctx_ptr);

    let handle = unsafe { AddVectoredExceptionHandler(1, Some(veh_handler)) };

    VEH_HANDLE.store(handle as usize, std::sync::atomic::Ordering::Release);
    veh_log(&format!("[L2-AV-DROP] VEH installed: handle={:?}", handle));
}

#[cfg(windows)]
pub fn remove_veh() {
    use windows::Win32::System::Diagnostics::Debug::RemoveVectoredExceptionHandler;

    let handle = VEH_HANDLE.swap(0, std::sync::atomic::Ordering::AcqRel);
    if handle != 0 {
        unsafe {
            let _ = RemoveVectoredExceptionHandler(handle as *mut std::ffi::c_void);
        }
        VEH_CONTEXT.store(std::ptr::null_mut(), Ordering::Release);
        veh_log("[L2-AV-DROP] VEH removed");
    }
}

// ============================================================================
// Register read/write helpers (shared by sub-handlers)
// ============================================================================

/// Read a 32-bit value from a register in the Windows CONTEXT.
#[cfg(windows)]
pub fn read_reg32(
    reg: iced_x86::Register,
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> u32 {
    use iced_x86::Register::*;
    match reg {
        EAX | RAX | AX | AL => context.Rax as u32,
        ECX | RCX | CX | CL => context.Rcx as u32,
        EDX | RDX | DX | DL => context.Rdx as u32,
        EBX | RBX | BX | BL => context.Rbx as u32,
        ESP | RSP => context.Rsp as u32,
        EBP | RBP | BP => context.Rbp as u32,
        ESI | RSI | SI | SIL => context.Rsi as u32,
        EDI | RDI | DI | DIL => context.Rdi as u32,
        R8D | R8 => context.R8 as u32,
        R9D | R9 => context.R9 as u32,
        R10D | R10 => context.R10 as u32,
        R11D | R11 => context.R11 as u32,
        _ => 0,
    }
}

/// Write a value to a register in the Windows CONTEXT with correct
/// per-width semantics (E4 fix, 2026-04-22).
///
/// Hardware semantics we match:
///   - 32-bit writes (EAX, ECX, ...): zero-extend to 64-bit parent (RAX).
///     This is what x86-64 does for `mov eax, X` / `mov r8d, X` etc.
///   - 16-bit writes (AX, CX, SI, DI, ...): preserve upper 48 bits of parent.
///     x86 `mov ax, X` does NOT touch bits 16..31 of EAX (or 16..63 of RAX).
///   - 8-bit low writes (AL, CL, SIL, DIL, ...): preserve upper 56 bits.
///   - 8-bit high writes (AH, CH, DH, BH): preserve all other bits; write
///     only bits 8..15 of the parent register.
///   - 64-bit writes (RAX, R8, ...): full write (rare in 32-bit-guest code).
///
/// Previous version wrote the full 64-bit `value` for every width,
/// clobbering upper bits of the parent register for 16-bit and 8-bit
/// destinations. Bug only fires when a narrow-width MMIO read or R15-wrap
/// read targets a subregister (e.g. `mov al, [0xFDxxxxxx]`) — rare in
/// current Spider-Man/Blue-series paths but architecturally a landmine.
#[cfg(windows)]
pub fn write_reg64(
    reg: iced_x86::Register,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    value: u64,
) {
    use iced_x86::Register::*;

    /// Preserve bits of `parent` outside the width-mask at the given shift,
    /// and deposit the low-width bits of `value` at that shift.
    #[inline]
    fn write_partial(parent: &mut u64, value: u64, width_mask: u64, shift: u32) {
        let full_mask = width_mask << shift;
        *parent = (*parent & !full_mask) | ((value & width_mask) << shift);
    }

    match reg {
        // 64-bit (full) — rare in 32-bit guest code but iced may surface RAX etc.
        RAX => context.Rax = value,
        RCX => context.Rcx = value,
        RDX => context.Rdx = value,
        RBX => context.Rbx = value,
        RBP => context.Rbp = value,
        RSI => context.Rsi = value,
        RDI => context.Rdi = value,
        R8 => context.R8 = value,
        R9 => context.R9 = value,
        R10 => context.R10 = value,
        R11 => context.R11 = value,

        // 32-bit — zero-extends upper 32 (matches x86-64 `mov r32` behaviour).
        EAX => context.Rax = value as u32 as u64,
        ECX => context.Rcx = value as u32 as u64,
        EDX => context.Rdx = value as u32 as u64,
        EBX => context.Rbx = value as u32 as u64,
        EBP => context.Rbp = value as u32 as u64,
        ESI => context.Rsi = value as u32 as u64,
        EDI => context.Rdi = value as u32 as u64,
        R8D => context.R8 = value as u32 as u64,
        R9D => context.R9 = value as u32 as u64,
        R10D => context.R10 = value as u32 as u64,
        R11D => context.R11 = value as u32 as u64,

        // 16-bit — preserves upper 48 bits of parent.
        AX => write_partial(&mut context.Rax, value, 0xFFFF, 0),
        CX => write_partial(&mut context.Rcx, value, 0xFFFF, 0),
        DX => write_partial(&mut context.Rdx, value, 0xFFFF, 0),
        BX => write_partial(&mut context.Rbx, value, 0xFFFF, 0),
        BP => write_partial(&mut context.Rbp, value, 0xFFFF, 0),
        SI => write_partial(&mut context.Rsi, value, 0xFFFF, 0),
        DI => write_partial(&mut context.Rdi, value, 0xFFFF, 0),

        // 8-bit low — preserves upper 56 bits of parent.
        AL => write_partial(&mut context.Rax, value, 0xFF, 0),
        CL => write_partial(&mut context.Rcx, value, 0xFF, 0),
        DL => write_partial(&mut context.Rdx, value, 0xFF, 0),
        BL => write_partial(&mut context.Rbx, value, 0xFF, 0),
        SIL => write_partial(&mut context.Rsi, value, 0xFF, 0),
        DIL => write_partial(&mut context.Rdi, value, 0xFF, 0),
        BPL => write_partial(&mut context.Rbp, value, 0xFF, 0),

        // 8-bit high — only bits 8..15 of parent.
        AH => write_partial(&mut context.Rax, value, 0xFF, 8),
        CH => write_partial(&mut context.Rcx, value, 0xFF, 8),
        DH => write_partial(&mut context.Rdx, value, 0xFF, 8),
        BH => write_partial(&mut context.Rbx, value, 0xFF, 8),

        _ => {}
    }
}

/// Sync guest GPRs from Windows CONTEXT back to GuestState.
#[cfg(windows)]
pub unsafe fn sync_guest_from_context(
    ctx: &mut RuntimeContext,
    wctx: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
) {
    ctx.guest.eax = wctx.Rax as u32;
    ctx.guest.ecx = wctx.Rcx as u32;
    ctx.guest.edx = wctx.Rdx as u32;
    ctx.guest.ebx = wctx.Rbx as u32;
    ctx.guest.esp = wctx.R14 as u32; // R14 = guest ESP
    ctx.guest.ebp = wctx.Rbp as u32;
    ctx.guest.esi = wctx.Rsi as u32;
    ctx.guest.edi = wctx.Rdi as u32;
    let old_top = ctx.guest.shadow_stack_top;
    ctx.guest.shadow_stack_top = wctx.R12; // R12 = shadow stack
    if old_top != wctx.R12 {
        crate::rate_log!(
            20,
            "[L4-SHADOW] SYNC old=0x{:X} new=0x{:X} delta={}",
            old_top,
            wctx.R12,
            old_top as i64 - wctx.R12 as i64
        );
    }
}

// ============================================================================
// Main VEH handler — orchestrator
// ============================================================================

#[cfg(windows)]
unsafe extern "system" fn veh_handler(
    exception_info: *mut windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS,
) -> i32 {
    use super::veh_dispatch;
    use super::veh_fixup;
    use super::veh_mmio;
    use windows::Win32::Foundation::{
        EXCEPTION_ACCESS_VIOLATION, EXCEPTION_BREAKPOINT, EXCEPTION_SINGLE_STEP,
    };

    const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
    const MS_VC_EXCEPTION: u32 = 0x406D1388; // SetThreadName exception — ignore

    let info = unsafe { &*exception_info };
    let record = unsafe { &*info.ExceptionRecord };
    let context = unsafe { &mut *info.ContextRecord };

    // Early-out for MSVC SetThreadName exception (not a real crash)
    if record.ExceptionCode.0 as u32 == MS_VC_EXCEPTION {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // Re-entrancy depth guard: allow up to 5 nested VEH handlers (normal for
    // compiled code that does R15 fixup → AV → fixup in sequence).
    // Beyond 5 deep, it's recursive and will overflow the stack.
    let depth = VEH_ACTIVE.with(|c| {
        let d = c.get();
        c.set(d + 1);
        d
    });
    struct VehGuard;
    impl Drop for VehGuard {
        fn drop(&mut self) {
            VEH_ACTIVE.with(|c| c.set(c.get().saturating_sub(1)));
        }
    }
    let _guard = VehGuard;
    if depth >= 3 {
        // Too deep — skip the faulting instruction and continue.
        // Returning CONTINUE_SEARCH would invoke Windows exception handlers
        // which use MORE stack, causing the overflow we're trying to prevent.
        let rip = context.Rip;
        let bytes = unsafe { std::slice::from_raw_parts(rip as *const u8, 15) };
        let mut dec = iced_x86::Decoder::with_ip(64, bytes, rip, iced_x86::DecoderOptions::NONE);
        if dec.can_decode() {
            let instr = dec.decode();
            context.Rip = rip + instr.len() as u64;
            // Zero destination register if it's a read
            if instr.op_count() >= 1 {
                if let iced_x86::OpKind::Register = instr.op_kind(0) {
                    super::veh::write_reg64(instr.op_register(0), context, 0);
                }
            }
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // Stack overflow: reset the guard page and exit the trampoline cleanly.
    // This happens when compiled guest code has deep call chains that
    // accumulate host stack frames (each emit_call_rel uses host RSP).
    if record.ExceptionCode.0 as u32 == 0xC000_00FD {
        // Stack overflow during guest execution. The compiled code or VEH
        // chain consumed too much host stack. Recovery: move RSP up by 64KB
        // (into valid stack space) and exit the trampoline.
        veh_log("[VEH] STACK_OVERFLOW — restoring safe state and exiting trampoline");
        // Find the RuntimeContext via R13 and force a trampoline exit
        let ctx_ptr = context.R13 as *mut crate::xbox::aot::runtime::RuntimeContext;
        if !ctx_ptr.is_null() {
            let ctx = unsafe { &mut *ctx_ptr };
            // Restore RSP to saved host_rsp (the OS thread stack before trampoline entry)
            let saved_host_rsp = ctx.guest.host_rsp;
            if saved_host_rsp > 0x10000 {
                context.Rsp = saved_host_rsp;
            }
            // Restore R14 (guest ESP) and R15 (guest memory base)
            context.R14 = 0x1EFF_E000u64; // WORKER_STACK_TOP - 0x2000
            context.R15 = ctx.guest.r15_base;
            context.R13 = ctx_ptr as u64;
            ctx.guest.esp = 0x1EFF_E000;
            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_ERROR;
            ctx.guest.exit_guest_addr = 0xDEAD_0001;
            context.Rip = ctx.guest.exit_addr;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // Bump global VEH dispatch counter on EVERY handler entry
    // (matches C++ g_veh_dispatch_count which counts all VEH events)
    bump_veh_dispatch();

    // Heap canary: detect the exact VEH event where the heap header gets zeroed.
    // Only active after CreateDevice has set _crtheap.
    // GUARD: only run on guest threads (R15 is only valid on guest threads).
    {
        static CANARY_ALIVE: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(true);
        static CANARY_LOGGED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        let is_guest_thread = !THREAD_VEH_CTX.with(|c| c.get()).is_null();
        if is_guest_thread
            && CANARY_ALIVE.load(std::sync::atomic::Ordering::Relaxed)
            && !CANARY_LOGGED.load(std::sync::atomic::Ordering::Relaxed)
        {
            let r15 = context.R15;
            let heap_ptr_addr = r15 + 0x007E_18B0u64;
            let heap = *(heap_ptr_addr as *const u32);
            if heap != 0 && heap < 0x2000_0000 {
                let sig8 = *((r15 + heap as u64 + 0x08) as *const u32);
                let sig16 = *((r15 + heap as u64 + 0x10) as *const u32);
                if sig8 != 0xEEFF_EEFF && sig16 != 0xEEFF_EEFF {
                    CANARY_LOGGED.store(true, std::sync::atomic::Ordering::Relaxed);
                    let ex_code = record.ExceptionCode.0 as u32;
                    let fault_addr = if record.NumberParameters >= 2 {
                        record.ExceptionInformation[1] as u64
                    } else {
                        0
                    };
                    veh_log(&format!(
                        "[HEAP-CANARY] DEAD! sig8=0x{:08X} sig16=0x{:08X} ex=0x{:08X} RIP=0x{:016X} fault=0x{:016X} R14=0x{:X} depth={}",
                        sig8, sig16, ex_code, context.Rip, fault_addr, context.R14, depth
                    ));
                    // Also dump the first 48 bytes of heap for context
                    for row in 0..3u32 {
                        let off = row * 16;
                        let v0 = *((r15 + heap as u64 + off as u64) as *const u32);
                        let v1 = *((r15 + heap as u64 + off as u64 + 4) as *const u32);
                        let v2 = *((r15 + heap as u64 + off as u64 + 8) as *const u32);
                        let v3 = *((r15 + heap as u64 + off as u64 + 12) as *const u32);
                        veh_log(&format!(
                            "[HEAP-CANARY] +0x{:03X}: {:08X} {:08X} {:08X} {:08X}",
                            off, v0, v1, v2, v3
                        ));
                    }
                }
            }
        }
    }

    // ---- Single-step trap: R15 fixup restoration [L2-TAG] + DR watchpoint ----
    if record.ExceptionCode == EXCEPTION_SINGLE_STEP {
        // DR0 hardware watchpoint. Dr6 bit 0 indicates DR0 fired.
        let dr6 = context.Dr6;
        if (dr6 & 0xF) != 0 {
            // Read written value BEFORE deciding whether to log — we want to
            // FILTER on value=0x00476920 (the smoking gun).
            let hit_slot = if (dr6 & (1 << 1)) != 0 {
                1
            } else if (dr6 & (1 << 2)) != 0 {
                2
            } else if (dr6 & (1 << 3)) != 0 {
                3
            } else {
                0
            };
            let watched_addr = match hit_slot {
                1 => context.Dr1,
                2 => context.Dr2,
                3 => context.Dr3,
                _ => context.Dr0,
            };
            let written_val = if watched_addr != 0 {
                unsafe { std::ptr::read_unaligned(watched_addr as *const u32) }
            } else {
                0
            };
            // Log every write when value looks suspicious (our crash target or
            // anything in 0x00475000-0x00478000 BSS zone). Also log first 5
            // events unconditionally for sanity.
            static HWBP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = HWBP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let is_suspicious = written_val == 0x0047_6920
                || (written_val >= 0x0047_5000 && written_val < 0x0047_8000);
            let verbose_watch = std::env::var_os("RUSTEMU_SPIDEY_WATCH_SCENE18C").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_WATCH_SCENE17F").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_GLOBAL_SCENE_WATCH").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_WATCH_GLOBAL_SCENE").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_SLOT1_WATCH").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_EVENT_SLOT_WATCH").is_some();
            if n < 5 || is_suspicious || verbose_watch {
                // Read instruction bytes at RIP for offline decode. Limit 16 bytes.
                let mut ins_bytes = [0u8; 16];
                let rip_ptr = context.Rip as *const u8;
                if !rip_ptr.is_null() {
                    unsafe {
                        std::ptr::copy_nonoverlapping(rip_ptr, ins_bytes.as_mut_ptr(), 16);
                    }
                }
                // Try reverse-lookup host_offset → guest PC via RuntimeContext.
                // Access via THREAD_VEH_CTX thread-local.
                let ctx_ptr = THREAD_VEH_CTX.with(|c| c.get());
                let (guest_pc, host_off, watched_guest, caller, caller2, watch_extra) = if !ctx_ptr
                    .is_null()
                {
                    let ctx = unsafe { &*ctx_ptr };
                    let cb = ctx.code_base as u64;
                    let gb = ctx.guest_mem_base as u64;
                    let valid_guest = |addr: u32| -> bool {
                        addr < 0x4000_0000
                            || (addr >= 0x8000_0000 && addr < 0xA000_0000)
                            || (addr >= 0xFD00_0000 && addr < 0xFF00_0000)
                    };
                    let safe_read_u32 = |addr: u32| -> u32 {
                        if valid_guest(addr) {
                            unsafe { std::ptr::read_unaligned((gb + addr as u64) as *const u32) }
                        } else {
                            0
                        }
                    };
                    let safe_read_u8 = |addr: u32| -> u8 {
                        if valid_guest(addr) {
                            unsafe { std::ptr::read_unaligned((gb + addr as u64) as *const u8) }
                        } else {
                            0
                        }
                    };
                    let read_guest_string = |addr: u32| -> String {
                        if !valid_guest(addr) {
                            return String::new();
                        }
                        let mut bytes = Vec::new();
                        for i in 0..48u32 {
                            let b = safe_read_u8(addr.wrapping_add(i));
                            if b == 0 {
                                break;
                            }
                            if b.is_ascii_graphic() || b == b' ' || b == b'\\' || b == b'/' {
                                bytes.push(b);
                            } else {
                                break;
                            }
                        }
                        String::from_utf8_lossy(&bytes).to_string()
                    };
                    let ebp = context.Rbp as u32;
                    let caller = safe_read_u32(ebp.wrapping_add(4));
                    let prev_ebp = safe_read_u32(ebp);
                    let caller2 = safe_read_u32(prev_ebp.wrapping_add(4));
                    let scene_name = read_guest_string(0x004B_C848);
                    let stash_name = read_guest_string(0x004B_C948);
                    let frame = safe_read_u32(0x004B_C630);
                    let frame_scene = safe_read_u32(frame.wrapping_add(0x18));
                    let scene_18c = safe_read_u8(frame_scene.wrapping_add(0x18C));
                    let scene_a0 = safe_read_u32(frame_scene.wrapping_add(0xA0));
                    let state_idx = safe_read_u32(scene_a0.wrapping_sub(0x10));
                    let state_arr = safe_read_u32(scene_a0.wrapping_sub(0x14));
                    let state_val = if state_idx < 0x1000 {
                        safe_read_u32(state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let xroot = safe_read_u32(0x004C_06B8);
                    let focus_mgr = safe_read_u32(xroot.wrapping_add(0x1A8));
                    let focus_head = safe_read_u32(focus_mgr.wrapping_add(0x10));
                    let focus_first = safe_read_u32(focus_head);
                    let focus_item = safe_read_u32(focus_first.wrapping_add(0x08));
                    let action_mgr = safe_read_u32(0x003F_7D90);
                    let action_active = safe_read_u32(action_mgr.wrapping_add(0x58));
                    let pad_cur = safe_read_u32(0x0037_9C00);
                    let pad_chg = safe_read_u32(0x0037_9C04);
                    let pad_prev = safe_read_u32(0x0037_9C08);
                    let mu_cur = safe_read_u32(0x0037_9C70);
                    let mu_chg = safe_read_u32(0x0037_9C74);
                    let mu_prev = safe_read_u32(0x0037_9C78);
                    let watch_extra = format!(
                        "scene='{}' stash='{}' frame_scene=0x{:08X} scene18c={} state_val={} xroot=0x{:08X} focus_mgr=0x{:08X} focus_item=0x{:08X} action_mgr=0x{:08X} action+58=0x{:08X} pad={:X}/{:X}/{:X} mu={:X}/{:X}/{:X}",
                        scene_name,
                        stash_name,
                        frame_scene,
                        scene_18c,
                        state_val,
                        xroot,
                        focus_mgr,
                        focus_item,
                        action_mgr,
                        action_active,
                        pad_cur,
                        pad_chg,
                        pad_prev,
                        mu_cur,
                        mu_chg,
                        mu_prev
                    );
                    let watched_guest =
                        if watched_addr >= gb && watched_addr.wrapping_sub(gb) <= 0xFFFF_FFFF {
                            watched_addr.wrapping_sub(gb) as u32
                        } else {
                            0
                        };
                    if context.Rip >= cb && context.Rip < cb + ctx.code_size as u64 {
                        let ho = (context.Rip - cb) as u32;
                        (
                            ctx.addr_hash.reverse_lookup(ho),
                            ho,
                            watched_guest,
                            caller,
                            caller2,
                            watch_extra,
                        )
                    } else {
                        (0, 0, watched_guest, caller, caller2, watch_extra)
                    }
                } else {
                    (0, 0, 0, 0, 0, String::new())
                };
                let hex = ins_bytes
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                crate::xbox::emulator::debug_log(&format!(
                    "[HW-WATCH #{}] {} DR{} guest_addr=0x{:08X} value=0x{:08X} RIP=0x{:016X} host_off=0x{:X} guest_pc~0x{:08X} caller=0x{:08X} caller2=0x{:08X} R14=0x{:08X} RBP=0x{:08X} RAX=0x{:016X} RSI=0x{:016X} {} bytes=[{}]",
                    n,
                    if is_suspicious { "SMOKING-GUN!" } else { "write" },
                    hit_slot,
                    watched_guest, written_val, context.Rip, host_off, guest_pc,
                    caller, caller2,
                    context.R14 as u32, context.Rbp as u32, context.Rax, context.Rsi,
                    watch_extra,
                    hex
                ));
            }
            // Clear Dr6 but leave Dr0/Dr7 armed.
            context.Dr6 = 0;
            context.EFlags |= 0x00010000; // RF: skip re-trap on same instr
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        if veh_fixup::handle_single_step(context) == VehResult::Handled {
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // ---- Entry guard: find thread context ----
    // ONLY use per-thread context. The VEH_CONTEXT global must NOT be used as
    // fallback — non-guest threads (RetroArch UI, D3D11 backend) would pick up
    // the guest thread's RuntimeContext and be incorrectly treated as guest threads.
    let ctx_ptr = THREAD_VEH_CTX.with(|c| c.get());
    if ctx_ptr.is_null() {
        // Non-guest thread exception — let Windows handle it.
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let ctx = unsafe { &mut *ctx_ptr };

    // Stop requested (unload) — force exit trampoline immediately
    if AOT_STOP_REQUESTED.load(Ordering::Acquire) && ctx.guest.exit_addr != 0 {
        sync_guest_from_context(ctx, context);
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_HALT;
        ctx.guest.exit_guest_addr = 0;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    let rip = context.Rip as u64;
    let code_base = ctx.code_base as u64;
    let code_end = code_base + ctx.code_size as u64;
    let in_main_code = rip >= code_base && rip < code_end;
    let in_our_code = in_main_code || ctx.is_our_code(rip);

    let region_end = if in_main_code {
        code_end
    } else {
        ctx.rescue_ranges
            .iter()
            .find(|&&(base, size)| {
                let b = base as u64;
                rip >= b && rip < b + size as u64
            })
            .map(|&(base, size)| base as u64 + size as u64)
            .unwrap_or(rip + 16)
    };

    // ---- Fast pass-through: AV outside our code ----
    if record.ExceptionCode == EXCEPTION_ACCESS_VIOLATION && !in_our_code {
        return handle_av_outside_code(rip, record, context, ctx);
    }

    // ---- Access Violation routing [L2-AV-DROP] ----
    if record.ExceptionCode == EXCEPTION_ACCESS_VIOLATION && in_our_code {
        return handle_av_in_code(
            rip,
            region_end,
            record,
            context,
            ctx,
            in_main_code,
            code_base,
        );
    }

    // ---- Catch-all for non-breakpoint exceptions in our code ----
    if record.ExceptionCode != EXCEPTION_BREAKPOINT && in_our_code {
        // Decode faulting instruction for diagnostics
        let bytes_len = 16.min((region_end - rip) as usize);
        let bytes_slice = unsafe { std::slice::from_raw_parts(rip as *const u8, bytes_len) };
        let mut dec =
            iced_x86::Decoder::with_ip(64, bytes_slice, rip, iced_x86::DecoderOptions::NONE);
        let decoded = if dec.can_decode() {
            Some(dec.decode())
        } else {
            None
        };
        let mut instr_str = String::new();
        if let Some(ref instr) = decoded {
            let mut fmt = iced_x86::IntelFormatter::new();
            iced_x86::Formatter::format(&mut fmt, instr, &mut instr_str);
        }
        let host_off = ctx.host_offset_for_rip(rip);
        let guest_pc = ctx.addr_hash.reverse_lookup(host_off);
        let exc_code = record.ExceptionCode.0 as u32;
        // For privileged IN/OUT port instructions, emulate the exact one-byte
        // operation before the generic unhandled-exception diagnostic.
        if exc_code == 0xC000_0096 {
            if super::veh_dispatch::handle_privileged_in_out(context, ctx) == VehResult::Handled {
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        veh_log(&format!(
            "[L2-AV-DROP] UNHANDLED 0x{:08X} at RIP=0x{:016X} guest=0x{:08X} R14=0x{:X} instr: {} bytes: {:02X?}",
            record.ExceptionCode.0 as u32, rip, guest_pc, context.R14,
            if instr_str.is_empty() { "??".to_string() } else { instr_str },
            &bytes_slice[..bytes_len.min(8)]
        ));
        // For integer divide-by-zero (0xC0000094/0xC0000095):
        // Skip the instruction, simulating division by 1: keep EAX (quotient = dividend),
        // set EDX = 0 (remainder = 0). Previous approach of EAX=EDX=0 corrupted game state
        // (e.g. Doom's ticdup=0 → G_Ticker divides by 0 → EAX=0 → gametic never advances).
        // Treating divisor-0 as divisor-1 preserves the dividend and lets init code proceed.
        if exc_code == 0xC000_0094 || exc_code == 0xC000_0095 {
            if let Some(ref instr) = decoded {
                let guest_pc = ctx.addr_hash.reverse_lookup(ctx.host_offset_for_rip(rip));
                // Diagnostic: read ticdup value for Doom's 0x18C41 div-by-zero
                static DIV_DIAG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = DIV_DIAG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 10 {
                    let gs_ptr = unsafe { *((context.R15 + 0x101BB8) as *const u32) };
                    let ticdup_addr = gs_ptr.wrapping_add(0x1F50);
                    let ticdup = if ticdup_addr < 0x1000_0000 {
                        unsafe { *((context.R15 + ticdup_addr as u64) as *const u32) }
                    } else {
                        0xDEAD
                    };
                    let word_1272 = if gs_ptr < 0x1000_0000 {
                        unsafe { *((context.R15 + gs_ptr as u64 + 0x1272) as *const u16) }
                    } else {
                        0xDEAD
                    };
                    veh_log(&format!("[VEH] DIV-BY-ZERO #{}: guest=0x{:08X} EAX=0x{:X} ECX=0x{:X} gs=0x{:08X} ticdup@1F50={} word@1272={}",
                        n, guest_pc, context.Rax, context.Rcx, gs_ptr, ticdup, word_1272));
                } else {
                    veh_log(&format!(
                        "[VEH] DIV-BY-ZERO: skip (div-by-1) at guest=0x{:08X} EAX=0x{:X}",
                        guest_pc, context.Rax
                    ));
                }
                context.Rip = rip + instr.len() as u64;
                // EAX stays as-is (quotient = dividend / 1 = dividend)
                context.Rdx = 0; // remainder = 0
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        // For other illegal/privileged instructions, skip the decoded host op.
        if exc_code == 0xC000_0096 {
            if let Some(ref instr) = decoded {
                context.Rip = rip + instr.len() as u64;
                context.Rax = 0;
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        sync_guest_from_context(ctx, context);
        ctx.guest.exit_reason = AOT_EXIT_ERROR;
        ctx.guest.exit_guest_addr = 0xBAD30000;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // ---- Exceptions OUTSIDE our code (RIP=0 → RET_TO_ZERO, or crash) ----
    if !in_our_code && !ctx_ptr.is_null() {
        // ---- ISR cleanup sentinel [L4-ISR-CLEAN] ----
        // Cherry-picked from JIT commit d6e8ccb. If the INT3 fired inside the
        // ISR cleanup trap page (outside our code buffer, which is why we check
        // here in the !in_our_code branch), the ISR we injected just RET'd
        // through our matched-transition sentinel. Run cleanup before anything
        // else touches state — we need to restore registers to the pre-injection
        // snapshot regardless of where the ISR ended up.
        // Also: depth-anchor check catches non-sentinel ISR exits (tail-jmp
        // returns, direct thunk returns) by comparing R12 against the saved
        // anchor we pushed at inject time.
        if record.ExceptionCode == EXCEPTION_BREAKPOINT {
            let via_sentinel = super::veh_dpc::is_cleanup_trap(rip);
            let via_anchor =
                !via_sentinel && super::veh_dpc::should_depth_anchor_cleanup(context.R12);
            if via_sentinel || via_anchor {
                let path = if via_sentinel { "SENTINEL" } else { "ANCHOR" };
                super::veh_dpc::log_cleanup_path(path, rip, context.R12);
                if super::veh_dpc::handle_isr_cleanup(context) == VehResult::Handled {
                    return EXCEPTION_CONTINUE_EXECUTION;
                }
            }
        }

        let result = handle_exception_outside_code(rip, record, context, ctx, ctx_ptr);
        if result != EXCEPTION_CONTINUE_SEARCH {
            return result;
        }
    }

    // ---- INT3 routing [L3-ROUTE] ----
    // DON'T reset null-deref streak here — INT3s from inline RET (RetMiss)
    // fire between consecutive null derefs, preventing the loop breaker.
    // The streak resets only when the loop breaker simulates RET.
    if record.ExceptionCode != EXCEPTION_BREAKPOINT {
        // STATUS_HEAP_CORRUPTION (0xC0000374): ntdll heap corruption detector.
        // This is fatal if unhandled — suppress to keep game running while investigating.
        if record.ExceptionCode.0 as u32 == 0xC000_0374 {
            static HEAP_CORRUPT_COUNT: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = HEAP_CORRUPT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[VEH] HEAP_CORRUPTION suppressed #{} at RIP=0x{:016X} — continuing",
                    n, rip
                ));
            }
            // Skip the exception by continuing — the heap is in an inconsistent state
            // but the game can often survive if it doesn't re-allocate from the bad bucket.
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        // Log unexpected non-breakpoint exceptions we can't handle
        veh_log(&format!(
            "[L2-AV-DROP] PASS-THROUGH: exc=0x{:08X} at RIP=0x{:016X} in_code={} ctx_null={}",
            record.ExceptionCode.0 as u32,
            rip,
            in_our_code,
            ctx_ptr.is_null()
        ));
        return EXCEPTION_CONTINUE_SEARCH;
    }

    if !in_our_code {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // Rescue code INT3 — check OOVPA hooks first, then exit as unresolved
    if !in_main_code {
        let host_offset = ctx.host_offset_for_rip(rip);
        if host_offset != 0xFFFF_FFFF {
            // Check OOVPA hooks in rescue pool
            if super::oovpa::veh_try_dispatch(host_offset, context, ctx, code_base) {
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        veh_log(&format!("[L3-ROUTE] RESCUE INT3 at RIP=0x{:016X}", rip));
        sync_guest_from_context(ctx, context);
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
        ctx.guest.exit_guest_addr = 0;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // Main code INT3 — route via dispatch sub-handler
    let host_offset = (rip - code_base) as u32;

    // Trace: record INT3 with host_offset only (no reverse_lookup — that's O(n))
    trace_record(b'I', 0, context, host_offset);

    // Check if this INT3 is the DPC/ISR cleanup point (Pitfall #28)
    // Must check BEFORE normal dispatch so we restore GPRs first.
    if super::veh_dpc::try_isr_dpc(context, host_offset, ctx) == VehResult::Handled {
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // Check OOVPA hooks before regular INT3 dispatch
    if super::oovpa::veh_try_dispatch(host_offset, context, ctx, code_base) {
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    if veh_dispatch::handle_int3(rip, code_base, context, ctx) == VehResult::Handled {
        // After successful INT3 dispatch, try ISR/DPC injection at this boundary
        // (ISR/DPC injection happens at INT3 boundaries — safe points in guest flow)
        // Note: context.Rip was already set by handle_int3, but if try_isr_dpc
        // injects, it will override Rip to the ISR/DPC target.
        let new_offset = (context.Rip - code_base) as u32;
        if super::veh_dpc::try_isr_dpc(context, new_offset, ctx) == VehResult::Handled {
            // ISR/DPC injection redirected execution
        }
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    crate::rate_log!(
        3,
        "[VEH-UNHANDLED] exc=0x{:08X} RIP=0x{:016X} in_code={} R14=0x{:X}",
        record.ExceptionCode.0 as u32,
        rip,
        in_our_code,
        context.R14
    );
    EXCEPTION_CONTINUE_SEARCH
}

// ============================================================================
// AV outside our code — MMIO from non-code, guest crash detection, pass-through.
// ============================================================================

#[cfg(windows)]
unsafe fn handle_av_outside_code(
    rip: u64,
    record: &windows::Win32::System::Diagnostics::Debug::EXCEPTION_RECORD,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
) -> i32 {
    use super::veh_mmio;
    const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

    let fault_addr = if record.NumberParameters >= 2 {
        record.ExceptionInformation[1] as u64
    } else {
        0
    };
    let r15_base = ctx.guest_mem_base as u64;
    let guest_offset = if fault_addr >= r15_base {
        (fault_addr - r15_base) as u32
    } else {
        0xFFFF_FFFF
    };

    // 1. NV2A MMIO range (0xFD000000-0xFE000000) — handle even from outside code buffer
    if guest_offset >= 0xFD00_0000 && guest_offset <= 0xFE00_0000 {
        let is_write = record.NumberParameters >= 1 && record.ExceptionInformation[0] == 1;
        let region_end = rip + 15; // max x64 instruction length
        if veh_mmio::handle_mmio(rip, region_end, context, guest_offset, is_write, ctx)
            == VehResult::Handled
        {
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }

    // 2. AV in Windows DLL code (RIP in high address range): Windows-internal exception.
    //    Non-guest thread: already handled above (THREAD_VEH_CTX null → CONTINUE_SEARCH).
    //    Guest thread at depth >= 1 (nested in VEH): skip the faulting instruction.
    //    We're inside our own VEH handler (rescue compiler or kernel handler called a
    //    Windows API that triggered an internal AV). Can't CONTINUE_SEARCH because
    //    JIT frames below KiUserExceptionDispatcher have no .pdata unwind info.
    //    Guest thread at depth 0 (direct): pass through — SEH can unwind Rust frames.
    if rip >= 0x7FF0_0000_0000 {
        let depth = VEH_ACTIVE.with(|c| c.get());
        if depth > 1 {
            // Nested exception: skip the faulting instruction to recover
            let bytes = unsafe { std::slice::from_raw_parts(rip as *const u8, 15) };
            let mut dec =
                iced_x86::Decoder::with_ip(64, bytes, rip, iced_x86::DecoderOptions::NONE);
            if dec.can_decode() {
                let instr = dec.decode();
                crate::rate_log!(50, "[VEH-PASSTHRU] OS AV depth={} skip: RIP=0x{:016X} fault=0x{:016X} — skip {} bytes",
                    depth, rip, fault_addr, instr.len());
                context.Rip = rip + instr.len() as u64;
                // Zero destination register for reads (return 0 to caller)
                if instr.op_count() >= 1 {
                    if let iced_x86::OpKind::Register = instr.op_kind(0) {
                        write_reg64(instr.op_register(0), context, 0);
                    }
                }
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        let fault_low = fault_addr as u32;
        let fault_looks_guest = fault_addr >= r15_base && fault_addr < r15_base + 0x1_0000_0000
            || fault_low < 0x2000_0000
            || (0x8000_0000..0xA000_0000).contains(&fault_low);
        if fault_looks_guest {
            let bytes = unsafe { std::slice::from_raw_parts(rip as *const u8, 15) };
            let mut dec =
                iced_x86::Decoder::with_ip(64, bytes, rip, iced_x86::DecoderOptions::NONE);
            if dec.can_decode() {
                let instr = dec.decode();
                crate::rate_log!(
                    20,
                    "[VEH-HOST-CRT-AV] host RIP=0x{:016X} fault=0x{:016X} low=0x{:08X} \
                     RCX=0x{:016X} RDX=0x{:016X} R8=0x{:016X} R9=0x{:016X} \
                     RSI=0x{:016X} RDI=0x{:016X} R14=0x{:08X} R15=0x{:016X} — skip {} bytes",
                    rip,
                    fault_addr,
                    fault_low,
                    context.Rcx,
                    context.Rdx,
                    context.R8,
                    context.R9,
                    context.Rsi,
                    context.Rdi,
                    context.R14 as u32,
                    context.R15,
                    instr.len()
                );
                context.Rip = rip + instr.len() as u64;
                if instr.op_count() >= 1 {
                    if let iced_x86::OpKind::Register = instr.op_kind(0) {
                        write_reg64(instr.op_register(0), context, 0);
                    }
                }
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        crate::rate_log!(100, "[VEH-PASSTHRU] OS AV on guest thread depth=0: RIP=0x{:016X} fault=0x{:016X} — passing to Windows SEH",
            rip, fault_addr);
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // 3. Non-MMIO AV on guest thread — check if R14 (guest ESP) is corrupted
    let r14_val = context.R14 as u32;
    let r14_looks_valid =
        r14_val < 0x2000_0000 || (r14_val >= 0x8000_0000 && r14_val < 0xA000_0000);

    if !r14_looks_valid {
        crate::rate_log!(10, "[VEH-PASSTHRU] CAUGHT guest crash: RIP=0x{:016X} fault=0x{:016X} R14=0x{:X} — forcing exit",
            rip, fault_addr, context.R14);
        ctx.guest.exit_reason = AOT_EXIT_ERROR;
        ctx.guest.exit_guest_addr = 0xDEAD_FFFF;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // 4. AV outside our code — force exit (worker dispatch loop handles re-entry).
    //    Can't pass through to Windows SEH: rescue-compiled garbage code causes host
    //    AVs that aren't from D3D11/system, and game SEH chain is invalid in x64.
    crate::rate_log!(5, "[VEH-PASSTHRU] AV not-our-code: RIP=0x{:016X} fault=0x{:016X} guest_off=0x{:08X} R14=0x{:X} — forcing exit",
        rip, fault_addr, guest_offset, context.R14);
    ctx.guest.exit_reason = AOT_EXIT_ERROR;
    ctx.guest.exit_guest_addr = 0xDEAD_FFFE;
    context.Rip = ctx.guest.exit_addr;
    EXCEPTION_CONTINUE_EXECUTION
}

// ============================================================================
// AV in our code — R15 fixup, MMIO, shadow stack, rep movsd, crash recovery.
// ============================================================================

/// Try to dispatch an exception through the guest's SEH chain.
/// Returns EXCEPTION_CONTINUE_EXECUTION if dispatched, EXCEPTION_CONTINUE_SEARCH if not.
/// Used for both AVs and non-AV exceptions (divide-by-zero, etc.).
#[cfg(windows)]
unsafe fn try_seh_dispatch(
    rip: u64,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    in_main_code: bool,
    code_base: u64,
    guest_addr: u32, // fault address for diagnostics (0 if not AV)
) -> i32 {
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
    const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

    let r15 = context.R15;
    // FS:[0] maps to [R15 + 0x0C000000] (FAKE_KPCR_BASE_ADDR in emitter.rs)
    const FS_ZERO_ADDR: u32 = 0x0C00_0000;
    // Safe read: check page is accessible before dereferencing
    let seh_addr = r15 + FS_ZERO_ADDR as u64;
    let seh_head = if seh_addr < r15 + 0x2000_0000 {
        *(seh_addr as *const u32)
    } else {
        0xFFFF_FFFF
    };

    crate::xbox::emulator::debug_log(&format!(
        "[SEH-TRY] guest_addr=0x{:08X} fs:[0]@0C000000=0x{:08X} R14=0x{:X}",
        guest_addr, seh_head, context.R14
    ));

    if seh_head == 0xFFFF_FFFF || seh_head == 0 || seh_head >= 0x2000_0000 {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    let mut frame_ptr = seh_head;
    for _walk in 0..16 {
        if frame_ptr == 0xFFFF_FFFF || frame_ptr == 0 || frame_ptr >= 0x2000_0000 {
            break;
        }
        let prev = guest_read_u32(r15, frame_ptr);
        let handler = guest_read_u32(r15, frame_ptr + 4);

        crate::xbox::emulator::debug_log(&format!(
            "[SEH-WALK] frame=0x{:08X} prev=0x{:08X} handler=0x{:08X}",
            frame_ptr, prev, handler
        ));

        if handler < 0x10000 || handler >= 0x0080_0000 {
            frame_ptr = prev;
            continue;
        }

        let scope_table = guest_read_u32(r15, frame_ptr + 8);
        let frame_ebp = frame_ptr.wrapping_add(0x10);
        let trylevel = guest_read_u32(r15, frame_ebp.wrapping_sub(4));

        crate::xbox::emulator::debug_log(&format!(
            "[SEH-WALK] scope_table=0x{:08X} frame_ebp=0x{:08X} trylevel={}",
            scope_table, frame_ebp, trylevel
        ));

        if scope_table < 0x10000 || scope_table >= 0x0080_0000 || trylevel > 100 {
            frame_ptr = prev;
            continue;
        }

        let mut level = trylevel;
        for _scope_walk in 0..32 {
            if level == 0xFFFF_FFFF {
                break;
            }
            let entry_addr = scope_table + level * 12;
            if entry_addr >= 0x0080_0000 {
                break;
            }
            let enclosing = guest_read_u32(r15, entry_addr) as i32;
            let filter = guest_read_u32(r15, entry_addr + 4);
            let except_handler = guest_read_u32(r15, entry_addr + 8);

            crate::xbox::emulator::debug_log(&format!(
                "[SEH-SCOPE] level={} enclosing={} filter=0x{:08X} handler=0x{:08X}",
                level, enclosing, filter, except_handler
            ));

            // Accept catch-all (filter=0xFFFFFFFF) OR any non-zero filter (filter function)
            // For non-catch-all, we can't evaluate the filter function — treat as catch-all
            if (filter == 0xFFFF_FFFF || filter >= 0x10000)
                && except_handler >= 0x10000
                && except_handler < 0x0080_0000
            {
                let host_offset = ctx.host_offset_for_rip(rip);
                let guest_pc = ctx.addr_hash.reverse_lookup(host_offset);

                veh_log(&format!(
                    "[SEH-DISPATCH] exc at guest=0x{:08X} fault=0x{:08X} → handler=0x{:08X} frame_ebp=0x{:08X} trylevel={}",
                    guest_pc, guest_addr, except_handler, frame_ebp, level
                ));

                guest_write_u32(r15, FS_ZERO_ADDR, prev);
                guest_write_u32(r15, frame_ebp.wrapping_sub(4), enclosing as u32);
                context.R14 = frame_ebp as u64;
                context.Rbp = frame_ebp as u64;
                context.Rax = 0;

                if let Some(host_off) = ctx.addr_hash.lookup(except_handler) {
                    context.Rip = ctx.code_base as u64 + host_off as u64;
                    return EXCEPTION_CONTINUE_EXECUTION;
                } else {
                    sync_guest_from_context(ctx, context);
                    ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
                    ctx.guest.exit_guest_addr = except_handler;
                    ctx.guest.esp = context.R14 as u32;
                    context.Rip = ctx.guest.exit_addr;
                    return EXCEPTION_CONTINUE_EXECUTION;
                }
            }
            level = enclosing as u32;
        }
        frame_ptr = prev;
    }

    EXCEPTION_CONTINUE_SEARCH
}

#[cfg(windows)]
#[allow(clippy::too_many_arguments)]
unsafe fn handle_av_in_code(
    rip: u64,
    region_end: u64,
    record: &windows::Win32::System::Diagnostics::Debug::EXCEPTION_RECORD,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    in_main_code: bool,
    code_base: u64,
) -> i32 {
    use super::{veh_fixup, veh_mmio};
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

    let fault_addr = if record.NumberParameters >= 2 {
        record.ExceptionInformation[1] as u64
    } else {
        0
    };
    let is_write = record.NumberParameters >= 1 && record.ExceptionInformation[0] == 1;
    let r15_base = ctx.guest_mem_base as u64;
    let guest_addr = fault_addr.wrapping_sub(r15_base) as u32;

    // 1. R15 fixup: sign-extension / overflow [L2-TAG]
    let offset64 = fault_addr.wrapping_sub(r15_base);
    if offset64 > 0xFFFF_FFFF {
        let host_offset = ctx.host_offset_for_rip(rip);
        let guest_pc = if veh_fixup::r15_fixup_profile_active() {
            ctx.addr_hash.reverse_lookup(host_offset)
        } else {
            0
        };
        if veh_fixup::handle_r15_fixup(
            fault_addr,
            r15_base,
            context,
            host_offset,
            guest_pc,
            &mut ctx.sign_ext_fixups,
        ) == VehResult::Handled
        {
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }

    // 2a. Write watchpoint on .text section (primary, cached mirror, uncached mirror)
    let is_code_region = (guest_addr >= 0x0001_1000 && guest_addr < 0x002E_BDB0)
        || (guest_addr >= 0x8001_1000 && guest_addr < 0x802E_BDB0)
        || (guest_addr >= 0xA001_1000 && guest_addr < 0xA02E_BDB0);
    if is_write && is_code_region {
        static TEXT_WP_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let wp_n = TEXT_WP_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // Write minidump on first .text corruption
        if wp_n == 0 {
            // Build EXCEPTION_POINTERS from our record+context for MiniDumpWriteDump
            let ep = windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS {
                ExceptionRecord: record as *const _ as *mut _,
                ContextRecord: context as *const _ as *mut _,
            };
            write_minidump(&ep as *const _);
        }
        if wp_n < 20 {
            let bytes_len = 16.min((region_end - rip) as usize);
            let bytes_slice = unsafe { std::slice::from_raw_parts(rip as *const u8, bytes_len) };
            let mut dec =
                iced_x86::Decoder::with_ip(64, bytes_slice, rip, iced_x86::DecoderOptions::NONE);
            if dec.can_decode() {
                let instr = dec.decode();
                let mut instr_str = String::new();
                let mut fmt = iced_x86::IntelFormatter::new();
                iced_x86::Formatter::format(&mut fmt, &instr, &mut instr_str);
                let host_off = ctx.host_offset_for_rip(rip);
                let guest_pc = ctx.addr_hash.reverse_lookup(host_off);
                veh_log(&format!(
                    "[CODE-WP] #{} WRITE 0x{:08X} from guest=0x{:08X} instr={} EAX=0x{:X} ECX=0x{:X} EDI=0x{:X} ESI=0x{:X} R14=0x{:X}",
                    wp_n, guest_addr, guest_pc, instr_str, context.Rax, context.Rcx, context.Rdi, context.Rsi, context.R14
                ));
            }
        }
        // Check if this is a rep stosd/rep stosb — if so, skip entirely (zero ECX, advance RIP)
        // instead of unprotecting. This prevents the 3.8GB zero flood from the NULL alloc bug.
        let bytes_len2 = 16.min((region_end - rip) as usize);
        let bytes2 = unsafe { std::slice::from_raw_parts(rip as *const u8, bytes_len2) };
        let mut dec2 = iced_x86::Decoder::with_ip(64, bytes2, rip, iced_x86::DecoderOptions::NONE);
        if dec2.can_decode() {
            let instr2 = dec2.decode();
            if instr2.has_rep_prefix() {
                // Skip the rep string op: set RCX=0 and advance RIP past it
                veh_log(&format!(
                    "[CODE-WP] SKIP rep string op at RIP=0x{:X} guest=0x{:08X} RCX={} RDI=0x{:X}",
                    rip, guest_addr, context.Rcx, context.Rdi
                ));
                context.Rcx = 0;
                context.Rip = rip + instr2.len() as u64;
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
        // For non-rep writes: unprotect the page so the write proceeds — one-shot watchpoint
        let page_base = (fault_addr & !0xFFF) as *const core::ffi::c_void;
        let mut old = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
        unsafe {
            let _ = windows::Win32::System::Memory::VirtualProtect(
                page_base,
                0x1000,
                windows::Win32::System::Memory::PAGE_READWRITE,
                &mut old,
            );
        }
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // 2. MMIO: NV2A GPU registers [NV2A-DATA]
    if veh_mmio::is_nv2a_mmio(guest_addr) {
        if veh_mmio::handle_mmio(rip, region_end, context, guest_addr, is_write, ctx)
            == VehResult::Handled
        {
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        veh_log(&format!(
            "[L2-AV-DROP] MMIO decode FAILED at guest=0x{:08X} RIP=0x{:016X}",
            guest_addr, rip
        ));
    }

    // 3. Shadow stack guard page hit — R12 overflow/underflow
    if ctx.shadow_stack.is_guard_fault(fault_addr) {
        ctx.sign_ext_fixups += 1;
        let is_ss_write = record.NumberParameters >= 2 && record.ExceptionInformation[0] == 1;
        if ctx.sign_ext_fixups <= 5 || (ctx.sign_ext_fixups % 1_000_000 == 0) {
            let host_off = ctx.host_offset_for_rip(rip);
            let guest_func = ctx.addr_hash.reverse_lookup(host_off);
            veh_log(&format!(
                "[L2-AV] SHADOW STACK {}: guest=0x{:08X} ESP=0x{:08X} (reset #{})",
                if is_ss_write { "OVERFLOW" } else { "UNDERFLOW" },
                guest_func,
                context.R14 as u32,
                ctx.sign_ext_fixups
            ));
            // One-shot: dump guest bytes and register state at the spin address
            if ctx.sign_ext_fixups == 1 {
                let gm = context.R15 as *const u8;
                let ga = guest_func;
                if ga > 0 && ga < 0x1000_0000 {
                    let mut bytes = String::with_capacity(96);
                    for i in 0u32..32 {
                        let b = unsafe { *gm.add((ga + i) as usize) };
                        bytes.push_str(&format!("{:02X} ", b));
                    }
                    veh_log(&format!(
                        "[L2-AV] SPIN-DIAG: guest=0x{:08X} bytes=[{}]",
                        ga,
                        bytes.trim()
                    ));
                    veh_log(&format!("[L2-AV] SPIN-DIAG: EAX=0x{:08X} ECX=0x{:08X} EDX=0x{:08X} EBX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X}",
                        context.Rax as u32, context.Rcx as u32, context.Rdx as u32,
                        context.Rbx as u32, context.Rsi as u32, context.Rdi as u32));
                    // Read pPut, threshold from device struct
                    let gpd = crate::xbox::aot::oovpa::G_PDEVICE_DYNAMIC
                        .load(std::sync::atomic::Ordering::Relaxed);
                    if gpd != 0 {
                        let dev_ptr = unsafe { *((gm as u64 + gpd as u64) as *const u32) };
                        if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                            let db = gm as u64 + dev_ptr as u64;
                            let p_put = unsafe { *((db + 0x00) as *const u32) };
                            let p_thr = unsafe { *((db + 0x04) as *const u32) };
                            let flags = unsafe { *((db + 0x08) as *const u32) };
                            veh_log(&format!("[L2-AV] SPIN-DIAG: dev=0x{:08X} pPut=0x{:08X} pThreshold=0x{:08X} flags=0x{:08X}",
                                dev_ptr, p_put, p_thr, flags));
                        }
                    }
                    // Hash table diagnostics: read the three game hash table objects
                    for &(name, addr) in &[
                        ("HT1", 0x6F2DD4u32),
                        ("HT2", 0x6F2E18u32),
                        ("HT3", 0x6F2E90u32),
                    ] {
                        let base = gm as u64 + addr as u64;
                        let f0 = unsafe { *((base) as *const u32) };
                        let f4 = unsafe { *((base + 4) as *const u32) }; // bucket ptr
                        let f10 = unsafe { *((base + 0x10) as *const u32) }; // count
                        veh_log(&format!("[L2-AV] SPIN-DIAG: {} @0x{:08X}: +0=0x{:08X} +4(buckets)=0x{:08X} +10(count)=0x{:08X}",
                            name, addr, f0, f4, f10));
                    }
                    // CRT heap handle check
                    let heap1 = unsafe { *((gm as u64 + 0x7E18B0) as *const u32) };
                    let heap2 = unsafe { *((gm as u64 + 0x7E1584) as *const u32) };
                    veh_log(&format!("[L2-AV] SPIN-DIAG: heap_handle1@7E18B0=0x{:08X} heap_handle2@7E1584=0x{:08X}", heap1, heap2));
                    // Zero page check: what's at address 0x28?
                    let zp28 = unsafe { *((gm as u64 + 0x28) as *const u32) };
                    veh_log(&format!(
                        "[L2-AV] SPIN-DIAG: [0x28]=0x{:08X} (zero page/TEB)",
                        zp28
                    ));
                }
            }
        }
        context.R12 = ctx.shadow_stack.top() as u64;
        // Break inline pushbuffer spin loops: sync GET=PUT in device struct.
        // D3D render state setters inline the space check (comparing PUT vs GET)
        // without calling the hooked MakeSpace/KickOff. The spin triggers millions
        // of shadow stack overflows. Syncing here breaks the spin immediately.
        crate::xbox::aot::oovpa::sync_pushbuffer_get_eq_put(context.R15 as *mut u8);
        if is_ss_write {
            return EXCEPTION_CONTINUE_EXECUTION;
        } else {
            let esp = context.R14 as u32;
            let ret_addr = guest_read_u32(context.R15, esp);
            context.R14 += 4;
            let normalized = crate::xbox::emulator::normalize_guest_addr(ret_addr);
            sync_guest_from_context(ctx, context);
            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
            ctx.guest.exit_guest_addr = normalized;
            ctx.guest.esp = context.R14 as u32;
            context.Rip = ctx.guest.exit_addr;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }

    // 4. Guest SEH dispatch — if the game registered an exception handler via
    //    __try/__except (_SEH_prolog), route the AV to the game's handler instead
    //    of doing crash recovery hacks. This is how real Xbox / Cxbx-R works.
    {
        let r15 = context.R15;
        // FS:[0] maps to guest address 0x0C000000 (FAKE_KPCR_BASE_ADDR in emitter.rs).
        // The AOT emitter adds 0x0C000000 to all FS: segment displacements.
        const FS_ZERO_ADDR: u32 = 0x0C00_0000;
        let seh_head = guest_read_u32(r15, FS_ZERO_ADDR);
        let host_off_diag = ctx.host_offset_for_rip(rip);
        let guest_pc_diag = ctx.addr_hash.reverse_lookup(host_off_diag);
        crate::rate_log!(
            20,
            "[SEH-CHECK] AV at guest=0x{:08X} fault=0x{:08X} fs:[0]@0C000000=0x{:08X}",
            guest_pc_diag,
            guest_addr,
            seh_head
        );
        if seh_head != 0xFFFF_FFFF && seh_head != 0 && seh_head < 0x2000_0000 {
            // Walk the SEH chain looking for a handler.
            // EXCEPTION_REGISTRATION_RECORD: { prev: u32, handler: u32 }
            // For MSVC __except_handler3, the frame also has:
            //   [frame+0x08] = scope_table, [frame+0x0C] = trylevel
            // The scope_table has entries: { enclosing_level, filter, handler }
            let mut frame_ptr = seh_head;
            let mut dispatched = false;

            for _walk in 0..16 {
                // max 16 frames to prevent infinite loop
                if frame_ptr == 0xFFFF_FFFF || frame_ptr == 0 || frame_ptr >= 0x2000_0000 {
                    break;
                }

                let prev = guest_read_u32(r15, frame_ptr);
                let handler = guest_read_u32(r15, frame_ptr + 4);

                crate::rate_log!(
                    20,
                    "[SEH-WALK] frame=0x{:08X} prev=0x{:08X} handler=0x{:08X}",
                    frame_ptr,
                    prev,
                    handler
                );

                // Validate handler is in executable guest code range
                if handler < 0x10000 || handler >= 0x0080_0000 {
                    frame_ptr = prev;
                    continue;
                }

                // Read the saved EBP for this frame. For _SEH_prolog frames,
                // EBP = frame_ptr + 0x10 (lea ebp,[esp+0x10] in _SEH_prolog).
                // The scope_table is at [frame_ptr + 0x08].
                let scope_table = guest_read_u32(r15, frame_ptr + 8);
                let frame_ebp = frame_ptr.wrapping_add(0x10);

                // Read trylevel from [EBP - 0x04]
                let trylevel = guest_read_u32(r15, frame_ebp.wrapping_sub(4));

                if scope_table < 0x10000 || scope_table >= 0x0080_0000 || trylevel > 100 {
                    frame_ptr = prev;
                    continue;
                }

                // Walk scope table entries: each is { enclosing_level: i32, filter: u32, handler: u32 }
                // Size = 12 bytes per entry. Find the entry for current trylevel.
                let mut level = trylevel;
                for _scope_walk in 0..32 {
                    if level == 0xFFFF_FFFF {
                        break;
                    }

                    let entry_addr = scope_table + level * 12;
                    if entry_addr >= 0x0080_0000 {
                        break;
                    }
                    let enclosing = guest_read_u32(r15, entry_addr) as i32;
                    let filter = guest_read_u32(r15, entry_addr + 4);
                    let except_handler = guest_read_u32(r15, entry_addr + 8);

                    // Evaluate the filter:
                    //   filter == 0xFFFFFFFF means __except(EXCEPTION_EXECUTE_HANDLER)
                    //     — i.e., catch-all. This is the common shorthand case.
                    //   filter != 0xFFFFFFFF means it's a real filter function
                    //     — we dispatch to `seh_dispatch::evaluate_filter_raw`
                    //     which pattern-matches known filter PCs (see Wine port
                    //     Stages 2-3).
                    // If EITHER returns "run the handler", we dispatch to it
                    // with the same unwind code path below.
                    let should_dispatch = except_handler >= 0x10000
                        && except_handler < 0x0080_0000
                        && (filter == 0xFFFF_FFFF
                            || matches!(
                                unsafe {
                                    crate::xbox::aot::seh_dispatch::evaluate_filter_raw(r15, filter)
                                },
                                crate::xbox::aot::seh_dispatch::FilterResult::ExecuteHandler
                            ));
                    if should_dispatch {
                        // Dispatch to this handler!
                        // Unwind: restore EBP, set ESP to frame, jump to handler
                        let host_offset = ctx.host_offset_for_rip(rip);
                        let guest_pc = ctx.addr_hash.reverse_lookup(host_offset);

                        veh_log(&format!(
                            "[SEH-DISPATCH] AV at guest=0x{:08X} fault=0x{:08X} → handler=0x{:08X} frame_ebp=0x{:08X} trylevel={}",
                            guest_pc, guest_addr, except_handler, frame_ebp, level
                        ));

                        // Restore SEH chain: fs:[0] = prev (unlink this frame)
                        guest_write_u32(r15, FS_ZERO_ADDR, prev);

                        // Set trylevel to enclosing level
                        guest_write_u32(r15, frame_ebp.wrapping_sub(4), enclosing as u32);

                        // Restore guest registers
                        context.R14 = frame_ebp.wrapping_sub(0x0C) as u64; // ESP at callee-saved regs
                        context.Rbp = frame_ebp as u64;

                        // Restore callee-saved from frame: [EBP-0x0C]=EDI, [EBP-0x08]=ESI, [EBP-0x04]=EBX...
                        // Actually _SEH_prolog stores: [EBP-0x18]=ESP, and callee-saves at bottom.
                        // The except handler expects: EBP = frame pointer, ESP = frame_ebp area.
                        // Set ESP to the frame start (after callee-saved regs are popped)
                        context.R14 = frame_ebp as u64; // ESP = EBP (standard frame)
                        context.Rax = 0; // EAX cleared (common convention)

                        // Jump to the except handler in compiled guest code
                        if let Some(host_off) = ctx.addr_hash.lookup(except_handler) {
                            context.Rip = ctx.code_base as u64 + host_off as u64;
                            dispatched = true;
                            break;
                        } else {
                            // Handler not in AOT cache — exit to dispatch loop
                            sync_guest_from_context(ctx, context);
                            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
                            ctx.guest.exit_guest_addr = except_handler;
                            ctx.guest.esp = context.R14 as u32;
                            context.Rip = ctx.guest.exit_addr;
                            dispatched = true;
                            break;
                        }
                    }

                    // Try enclosing scope
                    level = enclosing as u32;
                }

                if dispatched {
                    break;
                }
                frame_ptr = prev;
            }

            if dispatched {
                return EXCEPTION_CONTINUE_EXECUTION;
            }
            // No handler found — fall through to crash recovery
        }
    }

    // 5. Gap region fault (placeholder, not yet committed)
    {
        let guest_off = fault_addr.wrapping_sub(r15_base) as u32;
        if guest_off >= 0x2000_0000 && guest_off < 0x4000_0000 {
            crate::rate_log!(
                5,
                "[VEH-COMMIT] Fault in gap region: guest 0x{:08X}",
                guest_off
            );
        }
    }

    // 6. Crash handler — unhandled AV (no guest SEH handler found)
    let host_offset = ctx.host_offset_for_rip(rip);
    // Trace: record AV and dump ring on crash
    trace_record(
        b'A',
        guest_addr,
        context,
        fault_addr.wrapping_sub(r15_base) as u32,
    );
    trace_dump();
    handle_av_crash_recovery(
        rip,
        region_end,
        fault_addr,
        guest_addr,
        host_offset,
        record,
        context,
        ctx,
        in_main_code,
        code_base,
    )
}

// ============================================================================
// Exception outside our code — RET_TO_ZERO, crash in host, emulator thread AV.
// ============================================================================

#[cfg(windows)]
unsafe fn handle_exception_outside_code(
    rip: u64,
    record: &windows::Win32::System::Diagnostics::Debug::EXCEPTION_RECORD,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    ctx_ptr: *mut RuntimeContext,
) -> i32 {
    const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

    let thread_ctx = THREAD_VEH_CTX.with(|c| c.get());
    let is_emulator_thread = !thread_ctx.is_null();
    let r13_matches = context.R13 as *mut RuntimeContext == ctx_ptr;

    if !(r13_matches || is_emulator_thread) {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // RIP=0 → guest returned to address 0 (end of call chain)
    if rip == 0 {
        veh_log("[L2-AV-DROP] RET_TO_ZERO: guest returned to addr 0");
        sync_guest_from_context(ctx, context);
        ctx.guest.exit_reason = AOT_EXIT_RET_TO_ZERO;
        ctx.guest.exit_guest_addr = 0;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // R13 matches → we're in the trampoline but RIP is outside code buffer
    if r13_matches {
        let fault_addr = if record.NumberParameters >= 2 {
            record.ExceptionInformation[1] as u64
        } else {
            0
        };
        crate::rate_log!(10, "[L2-AV-DROP] CRASH OUTSIDE CODE: exc=0x{:08X} RIP=0x{:016X} fault=0x{:016X} R14=0x{:X}",
            record.ExceptionCode.0 as u32, rip, fault_addr, context.R14);
        sync_guest_from_context(ctx, context);
        ctx.guest.exit_reason = AOT_EXIT_ERROR;
        ctx.guest.exit_guest_addr = 0xDEAD_FFFF;
        context.Rip = ctx.guest.exit_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // Emulator thread but R13 doesn't match — in Rust code, not trampoline
    if is_emulator_thread
        && record.ExceptionCode == windows::Win32::Foundation::EXCEPTION_ACCESS_VIOLATION
    {
        let fault_addr = if record.NumberParameters >= 2 {
            record.ExceptionInformation[1] as u64
        } else {
            0
        };
        crate::rate_log!(
            5,
            "[L2-AV-DROP] AV IN EMULATOR THREAD (not trampoline): RIP=0x{:016X} fault=0x{:016X}",
            rip,
            fault_addr
        );
    }

    EXCEPTION_CONTINUE_SEARCH
}

// ============================================================================
// AV crash recovery — simulated RET → stack scan → exit.
// ============================================================================

#[cfg(windows)]
#[allow(clippy::too_many_arguments)]
unsafe fn handle_av_crash_recovery(
    rip: u64,
    region_end: u64,
    fault_addr: u64,
    guest_addr: u32,
    host_offset: u32,
    record: &windows::Win32::System::Diagnostics::Debug::EXCEPTION_RECORD,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut RuntimeContext,
    in_main_code: bool,
    code_base: u64,
) -> i32 {
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

    // AV-storm bailout (2026-04-21). If the same RIP faults >256 times
    // consecutively, give up and exit the trampoline cleanly with
    // AOT_EXIT_ERROR instead of looping forever. Observed on
    // test_blue_padded: after ESI corruption via (still-unidentified)
    // RET-discipline bug, the same instruction AVs for 23,000+ events
    // before our crash dump kicks in. That's dead time for both
    // debug.log and the user's eyes. Bailing out early lets the main
    // thread keep running (GPU backend, RetroArch frame loop) instead
    // of blocking behind the worker's infinite retry.
    //
    // Threshold 256 is empirical: a genuine page-commit-on-demand
    // fault can legitimately retry a few times while the commit
    // happens. 256 consecutive retries at the same RIP without
    // progress means no legitimate recovery is happening.
    {
        use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
        static LAST_FAULT_RIP: AtomicU64 = AtomicU64::new(0);
        static SAME_RIP_COUNT: AtomicU32 = AtomicU32::new(0);
        const STORM_THRESHOLD: u32 = 256;

        let prev = LAST_FAULT_RIP.swap(rip, Ordering::Relaxed);
        let count = if prev == rip {
            SAME_RIP_COUNT.fetch_add(1, Ordering::Relaxed) + 1
        } else {
            SAME_RIP_COUNT.store(1, Ordering::Relaxed);
            1
        };

        if count == STORM_THRESHOLD {
            veh_log(&format!(
                "[AV-STORM] RIP=0x{:016X} host_off=0x{:X} guest_addr=0x{:08X} \
                 fault=0x{:016X} — {} consecutive faults at same RIP, bailing out. \
                 EAX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X}",
                rip,
                host_offset,
                guest_addr,
                fault_addr,
                count,
                context.Rax as u32,
                context.Rsi as u32,
                context.Rdi as u32
            ));
            ctx.guest.exit_reason = AOT_EXIT_ERROR;
            ctx.guest.exit_addr = rip;
            // Exit the trampoline via the same mechanism the normal
            // crash path uses — set RIP to ctx.guest.exit_addr so the
            // dispatcher loop sees AOT_EXIT_ERROR next.
            context.Rip = rip; // caller will observe exit_reason != 0
                               // One log line is enough — don't keep spamming after bailout.
            return EXCEPTION_CONTINUE_EXECUTION;
        } else if count > STORM_THRESHOLD {
            // After threshold, silently stop retrying so we don't
            // flood the log.
            ctx.guest.exit_reason = AOT_EXIT_ERROR;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }

    // --- 32-bit address wrapping fix ---
    // Guest x86-32 does [idx*scale+disp] with 32-bit wrap, but x64 AOT computes
    // [R15+idx*scale+disp] with 64-bit math. When idx is large (e.g. 0xFFFFFFFF),
    // the 64-bit result overflows the 4GB guest reservation. Fix: recompute using
    // 32-bit arithmetic and emulate the access at the wrapped address.
    {
        let fault_offset = fault_addr.wrapping_sub(context.R15);
        if fault_offset > 0x1_0000_0000 {
            // Decode the faulting instruction
            let bytes_len = 16.min((region_end - rip) as usize);
            let bytes_slice = std::slice::from_raw_parts(rip as *const u8, bytes_len);
            let mut dec =
                iced_x86::Decoder::with_ip(64, bytes_slice, rip, iced_x86::DecoderOptions::NONE);
            if dec.can_decode() {
                let instr = dec.decode();
                // Find which operand is the memory access
                let (mem_op_idx, reg_op_idx) = if instr.op0_kind() == iced_x86::OpKind::Memory {
                    (0usize, 1usize)
                } else if instr.op_count() >= 2 && instr.op1_kind() == iced_x86::OpKind::Memory {
                    (1, 0)
                } else {
                    (usize::MAX, usize::MAX)
                };

                if mem_op_idx != usize::MAX {
                    // Compute wrapped 32-bit guest address
                    let index_reg = instr.memory_index();
                    let index_val = read_reg32(index_reg, context);
                    let scale = instr.memory_index_scale() as u32;
                    let disp = instr.memory_displacement32();
                    // Base register contribution (skip R15 — that's our guest memory base)
                    let base_reg = instr.memory_base();
                    let base_val = if base_reg == iced_x86::Register::R15 {
                        0u32
                    } else {
                        read_reg32(base_reg, context)
                    };
                    let wrapped = base_val
                        .wrapping_add(index_val.wrapping_mul(scale))
                        .wrapping_add(disp);

                    if (wrapped as u64) < 0x2000_0000 {
                        // within 512MB RAM
                        let host_ptr = context.R15 + wrapped as u64;
                        let is_read =
                            record.NumberParameters >= 1 && record.ExceptionInformation[0] == 0;
                        let is_write =
                            record.NumberParameters >= 1 && record.ExceptionInformation[0] == 1;

                        if is_read {
                            let value = *(host_ptr as *const u32);
                            let dst_reg = if mem_op_idx == 1 {
                                instr.op0_register()
                            } else {
                                instr.op1_register()
                            };
                            write_reg64(dst_reg, context, value as u64);
                            context.Rip = rip + instr.len() as u64;
                            crate::rate_log!(
                                20,
                                "[VEH-WRAP32] READ 0x{:08X} (wrapped from 0x{:X}) = 0x{:08X}",
                                wrapped,
                                fault_offset,
                                value
                            );
                            return EXCEPTION_CONTINUE_EXECUTION;
                        } else if is_write {
                            let src_val = if mem_op_idx == 0 {
                                // mem = reg: source is op1
                                read_reg32(instr.op1_register(), context)
                            } else {
                                read_reg32(instr.op0_register(), context)
                            };
                            *(host_ptr as *mut u32) = src_val;
                            context.Rip = rip + instr.len() as u64;
                            crate::rate_log!(
                                20,
                                "[VEH-WRAP32] WRITE 0x{:08X} (wrapped from 0x{:X}) = 0x{:08X}",
                                wrapped,
                                fault_offset,
                                src_val
                            );
                            return EXCEPTION_CONTINUE_EXECUTION;
                        }
                    }
                }
            }
        }
    }

    // --- Diagnostic dump ---
    let access_type = if record.NumberParameters >= 1 {
        match record.ExceptionInformation[0] {
            0 => "READ",
            1 => "WRITE",
            8 => "EXEC",
            _ => "?",
        }
    } else {
        "?"
    };

    let nearest_trap = ctx
        .trap_table
        .iter()
        .rev()
        .find(|t| t.host_offset < host_offset);
    let guest_pc = nearest_trap.map(|t| t.guest_addr).unwrap_or(0);

    // Decode faulting x64 instruction
    let mut faulting_instr_str = String::new();
    let bytes_len = 16.min((region_end - rip) as usize);
    let bytes_slice = std::slice::from_raw_parts(rip as *const u8, bytes_len);
    let mut dec = iced_x86::Decoder::with_ip(64, bytes_slice, rip, iced_x86::DecoderOptions::NONE);
    let decoded_ok = dec.can_decode();
    let fi = if decoded_ok { Some(dec.decode()) } else { None };
    if let Some(ref instr) = fi {
        let mut fmt = iced_x86::IntelFormatter::new();
        iced_x86::Formatter::format(&mut fmt, instr, &mut faulting_instr_str);
    }

    veh_log(&format!(
        "[L2-AV-DROP] CRASH: AV {} fault=0x{:016X} guest_off=0x{:08X} guest_pc=0x{:08X} instr: {}",
        access_type, fault_addr, guest_addr, guest_pc, faulting_instr_str
    ));
    veh_log(&format!(
        "  R14=0x{:08X} RBP=0x{:08X} RAX=0x{:08X} host_off=0x{:X}",
        context.R14 as u32, context.Rbp as u32, context.Rax as u32, host_offset
    ));
    // Full register dump for tracing bad pointers
    veh_log(&format!(
        "  EAX=0x{:08X} EBX=0x{:08X} ECX=0x{:08X} EDX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X}",
        context.Rax as u32,
        context.Rbx as u32,
        context.Rcx as u32,
        context.Rdx as u32,
        context.Rsi as u32,
        context.Rdi as u32
    ));
    // Dump stack args for the faulting function
    {
        let r14 = context.R14 as u32;
        let r15 = context.R15;
        let mut stack = [0u32; 12];
        for i in 0..12u32 {
            stack[i as usize] = guest_read_u32(r15, r14 + i * 4);
        }
        veh_log(&format!(
            "  Stack[0..11]: {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
            stack[0], stack[1], stack[2], stack[3], stack[4], stack[5],
            stack[6], stack[7], stack[8], stack[9], stack[10], stack[11]
        ));
    }

    // --- Recovery: simulate RET to exit the faulting function ---
    if let Some(ref instr) = fi {
        let instr_len = instr.len() as u64;
        context.Rax = 0;
        context.Rcx = 0;

        let fault_offset = fault_addr.wrapping_sub(context.R15) as u32;
        let guest_in_crt = guest_pc < 0x0010_0000;
        if fault_offset == 0xFFFFFFFF
            || fault_offset >= 0xF0000000
            || guest_in_crt
            || fault_offset >= 0x0800_0000
        {
            let r14 = context.R14 as u32;
            let r15 = context.R15;
            let ret_addr = guest_read_u32(r15, r14);
            let new_r14 = r14.wrapping_add(4);
            context.R14 = new_r14 as u64;
            context.Rax = 0;

            let normalized = if ret_addr >= 0x8000_0000 && ret_addr < 0xA000_0000 {
                ret_addr & 0x1FFF_FFFF
            } else {
                ret_addr
            };

            // Direct hash lookup
            if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                context.Rip = ctx.code_base as u64 + host_off as u64;
                crate::rate_log!(
                    10,
                    "[L2-AV-RET] guest=0x{:08X} simulated RET to 0x{:08X}",
                    guest_pc,
                    ret_addr
                );
                return EXCEPTION_CONTINUE_EXECUTION;
            }

            // Stack scan fallback
            for scan_off in (0..4096u32).step_by(4) {
                let scan_addr = new_r14.wrapping_add(scan_off);
                if scan_addr > 0x1000_0000 {
                    break;
                }
                let candidate = guest_read_u32(r15, scan_addr);
                let norm = if candidate >= 0x8000_0000 && candidate < 0xA000_0000 {
                    candidate & 0x1FFF_FFFF
                } else {
                    candidate
                };
                if norm >= 0x10000 && norm < 0x0080_0000 {
                    if let Some(host_off) = ctx.addr_hash.lookup(norm) {
                        context.R14 = (scan_addr + 4) as u64;
                        context.Rax = 0;
                        context.Rip = ctx.code_base as u64 + host_off as u64;
                        crate::rate_log!(
                            10,
                            "[L2-AV-SCAN] guest=0x{:08X} scanned to 0x{:08X} at ESP+{}",
                            guest_pc,
                            candidate,
                            scan_off
                        );
                        return EXCEPTION_CONTINUE_EXECUTION;
                    }
                }
            }

            // Stack scan failed — exit as RET_TO_ZERO
            sync_guest_from_context(ctx, context);
            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_RET_TO_ZERO;
            ctx.guest.exit_guest_addr = 0;
            context.Rip = ctx.guest.exit_addr;
            return EXCEPTION_CONTINUE_EXECUTION;
        }

        // Non-CRT: skip instruction
        context.Rip = rip + instr_len;
        return EXCEPTION_CONTINUE_EXECUTION;
    }

    // Can't decode — exit with error
    sync_guest_from_context(ctx, context);
    ctx.guest.exit_reason = AOT_EXIT_ERROR;
    ctx.guest.exit_guest_addr = 0xDEAD0000 + host_offset;
    context.Rip = ctx.guest.exit_addr;
    EXCEPTION_CONTINUE_EXECUTION
}

// Non-windows stubs
#[cfg(not(windows))]
pub fn install_veh(_ctx: &mut RuntimeContext) {}
#[cfg(not(windows))]
pub fn remove_veh() {}
#[cfg(not(windows))]
pub fn set_thread_context(_ctx: *mut RuntimeContext) {}
#[cfg(not(windows))]
pub fn clear_thread_context() {}
