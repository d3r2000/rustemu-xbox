/// VEH DPC sub-handler — ISR/DPC injection + dispatch counter [L4-DPC-*]
/// Ported from AOT_VEH_DPC.cpp (C++ srcv1).
///
/// Implements the "Ping Utility" (heartbeat injector): at INT3 trap boundaries,
/// injects VBlank ISR and DPC calls into the guest execution flow. This drives
/// the game's frame loop — without it, games stall waiting for VBlank interrupts.
///
/// Architecture:
///   - ISR/DPC addresses captured by KeInitializeInterrupt/KeInitializeDpc
///   - Stored in global atomics (set by kernel dispatch, read by VEH)
///   - VEH_TryISRDPC called from orchestrator after every INT3 dispatch
///   - Synchronous injection: build stdcall frame on guest stack, redirect RIP
///   - Caller cleanup: save/restore all GPRs around injection (Pitfall #28)
use std::sync::{
    atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    OnceLock,
};

#[cfg(windows)]
use crate::xbox::aot::runtime::RuntimeContext;
#[cfg(windows)]
use crate::xbox::aot::veh::VehResult;

/// Diagnostic kill-switches for Spider-Man VBlank delivery.
///
/// 2026-04-26: Suppress direct VEH guest-ISR/DPC injection for Spider-Man.
/// The worker dispatch loop already emulates VBlank interrupt side effects and
/// fires the captured DPC at safe NtYield/KeSetEvent boundaries. Directly
/// redirecting guest RIP into the ISR during arbitrary INT3 traps can resume at
/// fragile RET/SEH boundaries and turn stack-frame values into branch targets.
///
/// Keep the worker-side scheduler callbacks enabled: those run from controlled
/// dispatch-loop boundaries and are the path that advances the game's frame FSM.
pub const SUPPRESS_VEH_GUEST_ISR_DPC_INJECTION: bool = true;
pub const SUPPRESS_WORKER_GUEST_DPC_CALLBACKS: bool = false;

pub fn suppress_guest_isr_dpc_injection() -> bool {
    SUPPRESS_WORKER_GUEST_DPC_CALLBACKS
}

pub fn suppress_veh_guest_isr_dpc_injection() -> bool {
    SUPPRESS_VEH_GUEST_ISR_DPC_INJECTION
}

// ============================================================================
// Global ISR/DPC state — set by kernel handlers, consumed by VEH
// ============================================================================

/// VBlank ISR guest address (from KeInitializeInterrupt, first call)
static ISR_ADDRESS: AtomicU32 = AtomicU32::new(0);
/// VBlank ISR ServiceContext (arg1 to ISR)
static ISR_CONTEXT: AtomicU32 = AtomicU32::new(0);

/// PGRAPH completion ISR guest address (from KeInitializeInterrupt, second call)
static PGRAPH_ISR_ADDRESS: AtomicU32 = AtomicU32::new(0);
/// PGRAPH ISR ServiceContext
static PGRAPH_ISR_CONTEXT: AtomicU32 = AtomicU32::new(0);

/// DPC routine guest address (from KeInitializeDpc)
static DPC_ROUTINE: AtomicU32 = AtomicU32::new(0);
/// DPC DeferredContext (arg2 to DPC)
static DPC_CONTEXT: AtomicU32 = AtomicU32::new(0);
/// DPC object pointer (PKDPC — passed as arg0 to DPC routine)
static DPC_OBJECT: AtomicU32 = AtomicU32::new(0);
/// Timer object pointer (from KeSetTimer — used to set SignalState before DPC fire)
static DPC_TIMER: AtomicU32 = AtomicU32::new(0);
/// True when a kernel timer should repeatedly deliver the captured DPC.
///
/// One-shot DPCs remain queued-only; this is for title frame timers such as
/// Spider-Man's NtSetTimerEx heartbeat, where the guest expects the OS to keep
/// firing the DPC after the first arm.
static DPC_TIMER_PERIODIC: AtomicBool = AtomicBool::new(false);

/// True when ISR has queued a DPC (set by KeInsertQueueDpc or ISR injection)
static DPC_PENDING: AtomicBool = AtomicBool::new(false);
/// KDPC object currently queued by KeInsertQueueDpc.
static DPC_QUEUED_OBJECT: AtomicU32 = AtomicU32::new(0);

/// ISR injection cooldown (counts down per INT3 dispatch)
static ISR_COOLDOWN: AtomicU32 = AtomicU32::new(0);

/// PGRAPH ISR cooldown (separate from VBlank, longer interval)
static PGRAPH_ISR_COOLDOWN: AtomicU32 = AtomicU32::new(0);

/// VBlank counter (incremented each ISR injection)
static VBLANK_COUNT: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// P0 kernel-signal-chain telemetry (added 2026-04-25)
//
// 5-counter snapshot of the ISR→DPC chain. Diagnoses where the chain breaks
// when WORKER-DPC count = 0 in a run. Snapshot is logged whenever any one
// counter hits a milestone (1, 10, 100, 1000, 10000) so we get a self-
// terminating, low-noise picture of the chain state.
//
// Reading the snapshot in debug.log:
//   isr_attempts  = how many times try_inject_isr entered
//   isr_injected  = how many times ISR actually fired (RIP redirected)
//   dpc_pend_set  = how many times set_dpc_pending() called (i.e. the game's
//                   injected ISR reached its `call KeInsertQueueDpc`)
//   dpc_attempts  = how many times try_inject_dpc entered
//   dpc_injected  = how many times DPC actually fired (RIP redirected)
//
// Diagnostic interpretation:
//   isr_a > 0 && isr_s == 0       → ISR injection setup bailing (CRT guard,
//                                    addr_hash miss, unsafe boundary)
//   isr_s > 0 && dpc_pend_set==0  → injected ISR runs but never reaches its
//                                    KeInsertQueueDpc call (early bail in
//                                    ISR body — disasm at 0x002FB6CD/0x6DE
//                                    has 2 zero-tests that could short-
//                                    circuit before the KIQDpc call)
//   dpc_pend_set > 0 && dpc_a==0  → DPC_PENDING set but worker dispatch never
//                                    checks (is_main_worker gate at line 587)
//   dpc_a > 0 && dpc_s == 0       → DPC injection setup bailing (CRT guard,
//                                    addr_hash miss, no resume trap)
//   dpc_s > 0 && FSM stuck at 0   → DPC fires but doesn't tick the FSM clock —
//                                    memory file's "App+0x108 is the clock"
//                                    was wrong (per agent C8). Hunt elsewhere.
// ============================================================================
pub static KS_ISR_ATTEMPTS: AtomicU64 = AtomicU64::new(0);
pub static KS_ISR_INJECTED: AtomicU64 = AtomicU64::new(0);
pub static KS_DPC_PENDING_SET: AtomicU64 = AtomicU64::new(0);
pub static KS_DPC_ATTEMPTS: AtomicU64 = AtomicU64::new(0);
pub static KS_DPC_INJECTED: AtomicU64 = AtomicU64::new(0);

fn ks_log_snapshot(label: &str) {
    crate::xbox::emulator::debug_log(&format!(
        "[KS-TELEM] {} isr_a={} isr_s={} dpc_pend_set={} dpc_a={} dpc_s={}",
        label,
        KS_ISR_ATTEMPTS.load(Ordering::Relaxed),
        KS_ISR_INJECTED.load(Ordering::Relaxed),
        KS_DPC_PENDING_SET.load(Ordering::Relaxed),
        KS_DPC_ATTEMPTS.load(Ordering::Relaxed),
        KS_DPC_INJECTED.load(Ordering::Relaxed),
    ));
}

fn ks_milestone(n: u64, label: &str) {
    if n == 1 || n == 10 || n == 100 || n == 1000 || n == 10000 {
        ks_log_snapshot(&format!("{}={}", label, n));
    }
}

// ============================================================================
// Caller-cleanup state for synchronous injection (Pitfall #28)
// ============================================================================

/// Host offset of the INT3 where injection happened (0 = no active injection)
static INJECT_RESUME_OFFSET: AtomicU32 = AtomicU32::new(0);

/// Saved R14 (guest ESP) before injection frame was built
static INJECT_SAVED_R14: AtomicU32 = AtomicU32::new(0);

/// Saved guest GPRs during injection (DPC/ISR clobbers everything)
#[cfg(windows)]
struct SavedGprs {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rflags: u64,
}

#[cfg(windows)]
static mut INJECT_SAVED_GPRS: SavedGprs = SavedGprs {
    rax: 0,
    rbx: 0,
    rcx: 0,
    rdx: 0,
    rsi: 0,
    rdi: 0,
    rbp: 0,
    rflags: 0,
};

// ============================================================================
// Matched-transition ISR trampoline (cherry-picked from JIT commit d6e8ccb)
// ============================================================================

/// Saved RIP at the INT3 boundary — the post-dispatch resume point Windows was
/// about to execute. Restored on cleanup so the main flow continues unchanged.
static INJECT_SAVED_RIP: AtomicU64 = AtomicU64::new(0);

/// Saved R12 (shadow stack top) before we pushed our cleanup sentinel.
static INJECT_SAVED_R12: AtomicU64 = AtomicU64::new(0);

/// ISR cleanup trap page — a 4KB RWX page filled with INT3 (0xCC) bytes,
/// allocated lazily on first ISR injection. We push its base address onto R12
/// so that when the ISR's emitted RET does `jmp r10`, execution lands on our
/// INT3 and VEH runs `handle_isr_cleanup`. 0 = not yet allocated.
static ISR_CLEANUP_TRAP: AtomicU64 = AtomicU64::new(0);
const ISR_CLEANUP_TRAP_SIZE: usize = 0x1000;

/// Guest address that ISR injection needed to jump to but couldn't find in
/// addr_hash — typically because the JIT hasn't compiled that block yet.
/// The worker dispatch loop polls this (via `take_pending_isr_compile`) and
/// force-compiles the block. Next INT3, ISR injection will succeed.
static PENDING_ISR_COMPILE: AtomicU32 = AtomicU32::new(0);

/// Worker dispatch loop drains this. Returns the guest address to compile,
/// or 0 if nothing pending.
pub fn take_pending_isr_compile() -> u32 {
    PENDING_ISR_COMPILE.swap(0, Ordering::Relaxed)
}

/// Lazily allocate the cleanup trap page.
#[cfg(windows)]
fn ensure_cleanup_trap() -> u64 {
    let cur = ISR_CLEANUP_TRAP.load(Ordering::Acquire);
    if cur != 0 {
        return cur;
    }
    use windows::Win32::System::Memory::{
        VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
    };
    let ptr = unsafe {
        VirtualAlloc(
            None,
            ISR_CLEANUP_TRAP_SIZE,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if ptr.is_null() {
        return 0;
    }
    // Fill with INT3 (0xCC)
    unsafe {
        std::ptr::write_bytes(ptr as *mut u8, 0xCC, ISR_CLEANUP_TRAP_SIZE);
    }
    ISR_CLEANUP_TRAP.store(ptr as u64, Ordering::Release);
    crate::xbox::emulator::debug_log(&format!(
        "[L4-ISR-TRAP] allocated cleanup trap page base=0x{:016X} size={}",
        ptr as u64, ISR_CLEANUP_TRAP_SIZE
    ));
    ptr as u64
}

/// True if `rip` falls within the ISR cleanup trap page.
pub fn is_cleanup_trap(rip: u64) -> bool {
    let base = ISR_CLEANUP_TRAP.load(Ordering::Acquire);
    base != 0 && rip >= base && rip < base + ISR_CLEANUP_TRAP_SIZE as u64
}

/// True while an ISR injection is in flight. Used by veh.rs to gate the
/// depth-anchor check so we only do R12 comparison when a real injection
/// could be landing.
pub fn is_injection_active() -> bool {
    INJECT_SAVED_RIP.load(Ordering::Acquire) != 0
}

/// Log which cleanup path fired (diagnostics — rate-limited).
pub fn log_cleanup_path(path: &str, rip: u64, r12: u64) {
    static CLEAN_PATH_LOG: AtomicU32 = AtomicU32::new(0);
    let n = CLEAN_PATH_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 20 {
        let saved_r12 = INJECT_SAVED_R12.load(Ordering::Acquire);
        crate::xbox::emulator::debug_log(&format!(
            "[L4-ISR-PATH] {} rip=0x{:016X} r12=0x{:016X} saved_r12=0x{:016X} delta={}",
            path,
            rip,
            r12,
            saved_r12,
            r12 as i64 - saved_r12 as i64
        ));
    }
}

/// Depth-anchor structural check: if an ISR injection is in flight and the
/// current R12 has risen back to (or above) the anchor we saved at inject
/// time, the ISR has fully unwound its own call frames and is attempting to
/// return past the injection boundary. Fire cleanup even if RIP didn't land
/// on the sentinel trap page (covers tail-jmp exits, direct-thunk returns,
/// and any other non-sentinel ISR-exit path).
pub fn should_depth_anchor_cleanup(cur_r12: u64) -> bool {
    let saved_rip = INJECT_SAVED_RIP.load(Ordering::Acquire);
    if saved_rip == 0 {
        return false; // no injection in flight
    }
    let saved_r12 = INJECT_SAVED_R12.load(Ordering::Acquire);
    if saved_r12 == 0 {
        return false;
    }
    // Full-descending shadow stack: PUSH decrements R12, POP increments it.
    // At inject we did saved_r12 -> (saved_r12 - 8) and wrote sentinel there.
    // When the ISR's final RET pops that entry, R12 becomes saved_r12 again.
    // Any value >= saved_r12 means the ISR has returned or overshot.
    cur_r12 >= saved_r12
}

/// Run cleanup when the ISR's emitted RET jumps to the sentinel trap page
/// (or when depth-anchor detects it unwound past the injection boundary).
/// Restores every guest register we saved at inject time and resumes at the
/// original post-dispatch RIP. Called from the VEH orchestrator when it sees
/// an INT3 inside the cleanup trap page or a matched-depth R12.
#[cfg(windows)]
pub fn handle_isr_cleanup(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> VehResult {
    let saved_r14 = INJECT_SAVED_R14.load(Ordering::Acquire);
    let saved_rip = INJECT_SAVED_RIP.load(Ordering::Acquire);
    let saved_r12 = INJECT_SAVED_R12.load(Ordering::Acquire);

    if saved_rip == 0 {
        // No injection in flight — stray trap, let the normal handler deal with it.
        return VehResult::NotHandled;
    }

    // Sprint 3 validation (audit-2026-04-23.md P1.5 "shadow stack desync"):
    // The audit claimed L4-ISR cleanup sentinel was not popped off R12. Agent
    // S7-A5 / S9-A5 / S9-A24 all proved current code is correct: ISR's emitted
    // RET (`mov r10,[r12]; add r12,8; jmp r10`) pops the sentinel before we
    // land on the cleanup INT3, so `context.R12 == saved_r12` is already true
    // at this point. The old audit claim predates commit d6e8ccb (matched-
    // transition discipline) which introduced the explicit one-push/one-pop
    // contract. We preserve the unconditional `R12 = saved_r12` restore as a
    // defensive anchor — but we now VALIDATE the incoming R12 so any future
    // regression (or a genuine audit-described bug in another code path)
    // shows up in the log instead of silently succeeding.
    let incoming_r12 = context.R12;
    let r12_delta: i64 = incoming_r12 as i64 - saved_r12 as i64;
    if r12_delta != 0 {
        static R12_DRIFT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = R12_DRIFT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 20 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[L4-ISR-R12-DRIFT] #{} incoming_r12=0x{:016X} saved_r12=0x{:016X} delta={} \
                 — restoring anchor; audit P1.5 'sentinel not popped' would show delta=-8, \
                 'double-pop' would show delta=+8, 'overshoot' would show delta>0 multiple of 8",
                n, incoming_r12, saved_r12, r12_delta
            ));
        }
    }

    // Restore pointed registers (unconditional anchor — safe for all paths:
    // sentinel-trap, depth-anchor, early-exit).
    context.R14 = saved_r14 as u64;
    context.R12 = saved_r12;
    context.Rip = saved_rip;

    unsafe {
        context.Rax = INJECT_SAVED_GPRS.rax;
        context.Rbx = INJECT_SAVED_GPRS.rbx;
        context.Rcx = INJECT_SAVED_GPRS.rcx;
        context.Rdx = INJECT_SAVED_GPRS.rdx;
        context.Rsi = INJECT_SAVED_GPRS.rsi;
        context.Rdi = INJECT_SAVED_GPRS.rdi;
        context.Rbp = INJECT_SAVED_GPRS.rbp;
        context.EFlags = INJECT_SAVED_GPRS.rflags as u32;
    }

    // Clear injection state so the next INT3 can inject again
    INJECT_RESUME_OFFSET.store(0, Ordering::Release);
    INJECT_SAVED_RIP.store(0, Ordering::Release);

    let count = CLEANUP_LOG_COUNT.fetch_add(1, Ordering::Relaxed);
    if count < 10 {
        crate::xbox::emulator::debug_log(&format!(
            "[L4-ISR-CLEAN] ISR returned — restored R14=0x{:08X} R12=0x{:016X} RIP=0x{:016X}",
            saved_r14, saved_r12, saved_rip
        ));
    }

    VehResult::Handled
}

/// Log counter for ISR/DPC injection (limit noise)
static ISR_LOG_COUNT: AtomicU32 = AtomicU32::new(0);
static DPC_LOG_COUNT: AtomicU32 = AtomicU32::new(0);

/// Guard: CRT boot must complete before ISR injection starts.
/// ISR injection during _initterm corrupts the execution flow — the ISR return
/// path can misinterpret the ServiceContext (0x80D134B0) as a code address,
/// producing bogus targets like 0x00D134B0 that crash the CRT init chain.
/// Set to true by the worker after the organic StartRoutine chain completes.
static CRT_BOOT_COMPLETE: AtomicBool = AtomicBool::new(false);

/// Guard: worker-fired DPC is active — VEH must NOT inject ISR/DPC.
/// Xbox DPCs are non-reentrant (XDK: "DPCs are non-reentrant and serialized").
/// Without this guard, VEH injects a second DPC frame on top of the worker-fired
/// DPC, causing stack drift (+116 bytes observed) and crash on next DPC fire.
static WORKER_DPC_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Signal that CRT boot is complete and ISR injection can begin.
/// Idempotent — only logs on the first call.
pub fn signal_crt_boot_complete() {
    if CRT_BOOT_COMPLETE.swap(true, Ordering::AcqRel) {
        return; // already signaled
    }
    crate::xbox::emulator::debug_log("[L4-DPC] CRT boot complete — ISR injection enabled");
}

/// Set by worker before firing a DPC, cleared after DPC returns.
/// Prevents VEH from injecting ISR/DPC during an active DPC (non-reentrant).
pub fn set_worker_dpc_active(active: bool) {
    WORKER_DPC_ACTIVE.store(active, Ordering::Release);
}
static CLEANUP_LOG_COUNT: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// Public API — called by kernel handlers to capture ISR/DPC state
// ============================================================================

/// Called from KeInitializeInterrupt to capture ISR address.
/// First call = VBlank ISR (primary heartbeat), subsequent = PGRAPH ISR.
/// USB HID + PGRAPH share the same ISR function with different contexts.
pub fn capture_isr(service_routine: u32, service_context: u32) {
    if ISR_ADDRESS.load(Ordering::Relaxed) == 0 {
        ISR_ADDRESS.store(service_routine, Ordering::Release);
        ISR_CONTEXT.store(service_context, Ordering::Release);
        crate::xbox::emulator::debug_log(&format!(
            "[L4-DPC] Captured VBlank ISR=0x{:08X} ctx=0x{:08X}",
            service_routine, service_context
        ));
    } else {
        PGRAPH_ISR_ADDRESS.store(service_routine, Ordering::Release);
        PGRAPH_ISR_CONTEXT.store(service_context, Ordering::Release);
        crate::xbox::emulator::debug_log(&format!(
            "[L4-DPC] Captured PGRAPH ISR=0x{:08X} ctx=0x{:08X}",
            service_routine, service_context
        ));
    }
}

/// Called from KeInitializeDpc to capture DPC routine address.
pub fn capture_dpc(dpc_object: u32, deferred_routine: u32, deferred_context: u32) {
    DPC_OBJECT.store(dpc_object, Ordering::Release);
    DPC_ROUTINE.store(deferred_routine, Ordering::Release);
    DPC_CONTEXT.store(deferred_context, Ordering::Release);
    crate::xbox::emulator::debug_log(&format!(
        "[L4-DPC] Captured DPC routine=0x{:08X} ctx=0x{:08X} obj=0x{:08X}",
        deferred_routine, deferred_context, dpc_object
    ));
}

/// Get captured ISR and DPC addresses (for worker re-entry after phase 0).
pub fn get_captured_routines() -> (u32, u32) {
    (
        ISR_ADDRESS.load(Ordering::Relaxed),
        DPC_ROUTINE.load(Ordering::Relaxed),
    )
}

/// Get ISR ServiceContext (for ISR injection call frame).
pub fn get_isr_context() -> u32 {
    ISR_CONTEXT.load(Ordering::Relaxed)
}

/// Get DPC DeferredContext (for DPC injection call frame).
pub fn get_dpc_context() -> u32 {
    DPC_CONTEXT.load(Ordering::Relaxed)
}

/// Get DPC object pointer (for DPC injection call frame).
pub fn get_dpc_object() -> u32 {
    DPC_OBJECT.load(Ordering::Relaxed)
}

/// Capture timer address from KeSetTimer (for SignalState before DPC fire).
pub fn capture_timer(timer_ptr: u32) {
    DPC_TIMER.store(timer_ptr, Ordering::Release);
}

/// Enable/disable periodic delivery for the captured timer DPC.
pub fn set_timer_periodic(active: bool) {
    DPC_TIMER_PERIODIC.store(active, Ordering::Release);
}

/// Check whether the captured timer should keep delivering its DPC.
pub fn is_timer_periodic() -> bool {
    DPC_TIMER_PERIODIC.load(Ordering::Acquire)
}

/// Get timer pointer (for setting SignalState before DPC fire).
pub fn get_dpc_timer() -> u32 {
    DPC_TIMER.load(Ordering::Relaxed)
}

fn mark_dpc_pending() {
    DPC_PENDING.store(true, Ordering::Release);
    let n = KS_DPC_PENDING_SET.fetch_add(1, Ordering::Relaxed) + 1;
    ks_milestone(n, "dpc_pend_set");
}

/// Called from internal timer/VBlank paths to signal a synthetic DPC pending.
pub fn set_dpc_pending() {
    mark_dpc_pending();
}

/// Queue a guest KDPC object. Returns true only when the DPC was not already
/// queued, matching KeInsertQueueDpc's BOOLEAN return contract.
pub fn queue_dpc_object(dpc_object: u32) -> bool {
    if dpc_object == 0 {
        mark_dpc_pending();
        return true;
    }

    match DPC_QUEUED_OBJECT.compare_exchange(0, dpc_object, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => {
            mark_dpc_pending();
            true
        }
        Err(existing) => existing != dpc_object && false,
    }
}

/// Pop the queued KDPC object when the scheduler consumes a pending DPC.
/// Returns `None` for synthetic/internal DPCs queued without a guest KDPC.
pub fn take_queued_dpc_object() -> Option<u32> {
    let queued = DPC_QUEUED_OBJECT.swap(0, Ordering::AcqRel);
    DPC_PENDING.store(false, Ordering::Release);
    if queued == 0 {
        None
    } else {
        Some(queued)
    }
}

/// Remove a queued guest KDPC object. Returns true if it was queued.
pub fn remove_dpc_object(dpc_object: u32) -> bool {
    let queued = DPC_QUEUED_OBJECT.load(Ordering::Acquire);
    if queued == 0 || (dpc_object != 0 && queued != dpc_object) {
        return false;
    }
    DPC_QUEUED_OBJECT.store(0, Ordering::Release);
    DPC_PENDING.store(false, Ordering::Release);
    true
}

/// Reset ISR cooldown (called from retro_run VBlank boundary).
pub fn reset_isr_cooldown() {
    ISR_COOLDOWN.store(0, Ordering::Relaxed);
}

/// Reset all DPC/ISR state (called on unload).
pub fn reset_all() {
    ISR_ADDRESS.store(0, Ordering::Relaxed);
    ISR_CONTEXT.store(0, Ordering::Relaxed);
    PGRAPH_ISR_ADDRESS.store(0, Ordering::Relaxed);
    PGRAPH_ISR_CONTEXT.store(0, Ordering::Relaxed);
    DPC_ROUTINE.store(0, Ordering::Relaxed);
    DPC_CONTEXT.store(0, Ordering::Relaxed);
    DPC_OBJECT.store(0, Ordering::Relaxed);
    DPC_QUEUED_OBJECT.store(0, Ordering::Relaxed);
    DPC_PENDING.store(false, Ordering::Relaxed);
    DPC_TIMER.store(0, Ordering::Relaxed);
    DPC_TIMER_PERIODIC.store(false, Ordering::Relaxed);
    ISR_COOLDOWN.store(0, Ordering::Relaxed);
    PGRAPH_ISR_COOLDOWN.store(0, Ordering::Relaxed);
    VBLANK_COUNT.store(0, Ordering::Relaxed);
    INJECT_RESUME_OFFSET.store(0, Ordering::Relaxed);
    ISR_LOG_COUNT.store(0, Ordering::Relaxed);
    DPC_LOG_COUNT.store(0, Ordering::Relaxed);
    CLEANUP_LOG_COUNT.store(0, Ordering::Relaxed);
    KS_ISR_ATTEMPTS.store(0, Ordering::Relaxed);
    KS_ISR_INJECTED.store(0, Ordering::Relaxed);
    KS_DPC_PENDING_SET.store(0, Ordering::Relaxed);
    KS_DPC_ATTEMPTS.store(0, Ordering::Relaxed);
    KS_DPC_INJECTED.store(0, Ordering::Relaxed);
    CRT_BOOT_COMPLETE.store(false, Ordering::Relaxed);
    WORKER_DPC_ACTIVE.store(false, Ordering::Relaxed);
}

/// Check if CRT boot is complete (ISR/DPC can fire).
pub fn is_crt_boot_complete() -> bool {
    CRT_BOOT_COMPLETE.load(Ordering::Acquire)
}

/// Get ISR address (for worker dispatch loop).
pub fn get_isr_address() -> u32 {
    ISR_ADDRESS.load(Ordering::Relaxed)
}

/// Check if DPC is pending.
pub fn is_dpc_pending() -> bool {
    DPC_PENDING.load(Ordering::Acquire)
}

/// Clear DPC pending flag (called after dispatch-loop fires DPC).
pub fn clear_dpc_pending() {
    let _ = take_queued_dpc_object();
}

/// Get DPC routine address (for worker dispatch loop).
pub fn get_dpc_routine() -> u32 {
    DPC_ROUTINE.load(Ordering::Relaxed)
}

/// SystemArgument1 value for worker/VEH-fired DPC frames.
///
/// Spider-Man's XDK D3D VBlank DPC at 0x002FBF10 starts with
/// `mov ebx, [esp+0x0c]` and then reads `[ebx]` as the NV2A MMIO base.
/// The real ISR/KeInsertQueueDpc path supplies the miniport as
/// SystemArgument1; our direct worker fire used zero, so the body read from
/// guest address 0 and returned without taking the PMC_INTR branch.
///
/// Default-off for now: enabling this proves the body sees PMC_INTR bit 24,
/// but the 2026-04-26 smoke showed Spider-Man frame presentation regressing
/// while the worker entered a very hot D3D loop. Keep it as an opt-in probe
/// until the presentation/yield side is fixed.
pub fn dpc_system_argument1(dpc_routine: u32, deferred_context: u32) -> u32 {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    let enabled = *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_DPC_SYSARG1")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    });
    if enabled && dpc_routine == 0x002F_BF10 {
        deferred_context
    } else {
        0
    }
}

/// Set NV2A shadow registers for VBlank ISR injection.
/// Called before firing ISR from the worker dispatch loop.
pub fn setup_vblank_shadow_regs() {
    // PMC_INTR (0x100): set ONLY PCRTC (bit 24) pending.
    // Do NOT set PGRAPH (bit 12) — the PGRAPH idle override returns 0 for all
    // PGRAPH reads (0xFD400000+), so the ISR would see PGRAPH pending in PMC_INTR
    // but PGRAPH_INTR=0, causing an infinite ack loop (write-1-to-clear mismatch).
    super::nv2a::shadow_write(0x0100, 0x0100_0000); // PCRTC only
                                                    // PMC_INTR_EN (0x140): master enable (bit 0) + PCRTC (bit 24)
    super::nv2a::shadow_write(0x0140, 0x0100_0001);
    // PCRTC_INTR (0x600100): VBlank pending (bit 0)
    super::nv2a::shadow_write(0x60_0100, 0x01);
}

/// Increment VBlank counter (called after ISR fires from dispatch loop).
pub fn increment_vblank() {
    VBLANK_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Check if ISR/DPC infrastructure has been captured (for diagnostics).
pub fn has_isr() -> bool {
    ISR_ADDRESS.load(Ordering::Relaxed) != 0
}
pub fn has_dpc() -> bool {
    DPC_ROUTINE.load(Ordering::Relaxed) != 0
}

// ============================================================================
// VEH integration — called from orchestrator at INT3 boundaries
// ============================================================================

/// Try ISR/DPC injection at the current INT3 boundary.
/// Returns Handled if injection happened (RIP was redirected), NotHandled otherwise.
///
/// Called from veh.rs orchestrator AFTER normal INT3 dispatch completes.
/// Matching C++ VEH_TryISRDPC().
#[cfg(windows)]
pub fn try_isr_dpc(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
    ctx: &mut RuntimeContext,
) -> VehResult {
    // Guard: don't nest injections (single-level save)
    if INJECT_RESUME_OFFSET.load(Ordering::Relaxed) != 0 {
        // Check if this IS the resume point (caller cleanup)
        if host_offset == INJECT_RESUME_OFFSET.load(Ordering::Relaxed) {
            return handle_cleanup(context);
        }
        return VehResult::NotHandled;
    }

    // Guard: worker-fired DPC is active — don't inject ISR/DPC on top of it.
    // Xbox DPCs are non-reentrant; injecting here causes stack drift and crash.
    if WORKER_DPC_ACTIVE.load(Ordering::Acquire) {
        return VehResult::NotHandled;
    }

    if suppress_veh_guest_isr_dpc_injection() {
        static SUPPRESS_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SUPPRESS_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 10 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[DPC-SUPPRESS] VEH guest ISR/DPC injection skipped at host_off=0x{:X} esp=0x{:08X}",
                host_offset,
                context.R14 as u32
            ));
        }
        return VehResult::NotHandled;
    }

    // THREAD-AWARE GUARD: never inject ISR/DPC on the main worker thread via VEH.
    // VEH injection on the main worker during kernel dispatch sets resume address
    // to kernel ordinals (0xFFFF0000+), causing BOGUS TARGET crashes.
    // The worker dispatch loop handles VBlank DPC delivery via
    // try_fire_isr_dpc_from_dispatch() at KeSetEvent/NtYieldExecution boundaries.
    let r14_check = context.R14 as u32;
    let is_main_worker = r14_check >= 0x1E00_0000 && r14_check < 0x1F00_0000;

    // 1. VBlank ISR injection (priority: checked first)
    //    Guard: no ISR during CRT boot — corrupts _initterm execution flow.
    let isr_addr = ISR_ADDRESS.load(Ordering::Acquire);
    let crt_done = CRT_BOOT_COMPLETE.load(Ordering::Acquire);
    if isr_addr != 0 && !crt_done {
        static ISR_BLOCK_LOG: AtomicU32 = AtomicU32::new(0);
        let cnt = ISR_BLOCK_LOG.fetch_add(1, Ordering::Relaxed);
        if cnt < 3 {
            crate::xbox::emulator::debug_log(&format!(
                "[L4-DPC] ISR BLOCKED by CRT guard (isr=0x{:08X}, crt_done=false, block#{})",
                isr_addr, cnt
            ));
        }
    }
    // VEH ISR injection RE-ENABLED 2026-04-20 (cherry-picked d6e8ccb matched-transition).
    // The previous disable was defensive after ISR injection corrupted the stack.
    // With the cleanup-trap sentinel + depth-anchor check now in place (see
    // ensure_cleanup_trap, should_depth_anchor_cleanup, handle_isr_cleanup), the
    // ISR's emitted RET is guaranteed to land on an INT3 we own, and cleanup
    // restores the pre-injection register snapshot cleanly.
    //
    // Thread gating: only main worker receives VBlank injection. Child threads
    // (background loaders, audio) still get VBlank side-effect emulation via
    // try_fire_isr_dpc_from_dispatch in worker.rs, but never actual ISR call.
    if is_main_worker && isr_addr != 0 && crt_done {
        let cooldown = ISR_COOLDOWN.load(Ordering::Relaxed);
        if cooldown > 0 {
            ISR_COOLDOWN.store(cooldown - 1, Ordering::Relaxed);
        } else {
            // Time to inject ISR
            if let Some(result) = try_inject_isr(context, host_offset, ctx, isr_addr) {
                return result;
            }
        }
    }

    // 2. PGRAPH ISR injection — also DISABLED (same reason as VBlank ISR)
    let pgraph_isr = PGRAPH_ISR_ADDRESS.load(Ordering::Acquire);
    if false && pgraph_isr != 0 && crt_done {
        let pgraph_cooldown = PGRAPH_ISR_COOLDOWN.load(Ordering::Relaxed);
        if pgraph_cooldown > 0 {
            PGRAPH_ISR_COOLDOWN.store(pgraph_cooldown - 1, Ordering::Relaxed);
        } else if ctx.mmio_count > 0 {
            // Only inject PGRAPH ISR after first MMIO access (GPU init must be done)
            if let Some(result) = try_inject_pgraph_isr(context, host_offset, ctx, pgraph_isr) {
                return result;
            }
        }
    }

    // 3. DPC injection (if ISR queued one, or direct DPC pending)
    //    THREAD-AWARE: Only inject DPC on child threads, NOT on the main worker.
    if DPC_PENDING.load(Ordering::Acquire) {
        if !is_main_worker {
            let dpc_routine = DPC_ROUTINE.load(Ordering::Relaxed);
            if dpc_routine != 0 {
                DPC_PENDING.store(false, Ordering::Release);
                if let Some(result) = try_inject_dpc(context, host_offset, ctx, dpc_routine) {
                    return result;
                }
            }
        }
    }

    VehResult::NotHandled
}

/// Handle caller cleanup after ISR/DPC returns to the injection point.
#[cfg(windows)]
fn handle_cleanup(context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT) -> VehResult {
    // Restore R14 (guest ESP) to pre-injection value
    let saved_r14 = INJECT_SAVED_R14.load(Ordering::Relaxed);
    context.R14 = saved_r14 as u64;

    // Restore all GPRs
    unsafe {
        context.Rax = INJECT_SAVED_GPRS.rax;
        context.Rbx = INJECT_SAVED_GPRS.rbx;
        context.Rcx = INJECT_SAVED_GPRS.rcx;
        context.Rdx = INJECT_SAVED_GPRS.rdx;
        context.Rsi = INJECT_SAVED_GPRS.rsi;
        context.Rdi = INJECT_SAVED_GPRS.rdi;
        context.Rbp = INJECT_SAVED_GPRS.rbp;
        context.EFlags = INJECT_SAVED_GPRS.rflags as u32;
    }

    // Clear injection state
    INJECT_RESUME_OFFSET.store(0, Ordering::Release);

    crate::rate_log!(
        20,
        "[L4-CLEANUP] Restored R14=0x{:08X} after ISR/DPC return",
        saved_r14
    );

    // Fall through to normal INT3 dispatch (return NotHandled so orchestrator
    // processes this INT3 normally)
    VehResult::NotHandled
}

/// Inject VBlank ISR: build 3-arg stdcall frame and redirect to ISR routine.
#[cfg(windows)]
fn try_inject_isr(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
    ctx: &mut RuntimeContext,
    isr_addr: u32,
) -> Option<VehResult> {
    // P0 telemetry: count every entry, regardless of guard outcome.
    let _n_isr_a = KS_ISR_ATTEMPTS.fetch_add(1, Ordering::Relaxed) + 1;
    ks_milestone(_n_isr_a, "isr_attempts");

    // HARD GUARD: never inject ISR during CRT boot, regardless of caller.
    let crt_state = CRT_BOOT_COMPLETE.load(Ordering::Acquire);
    // Log EVERY call (even blocked ones) to trace the leak
    static ISR_CALL_CTR: AtomicU32 = AtomicU32::new(0);
    let call_n = ISR_CALL_CTR.fetch_add(1, Ordering::Relaxed);
    if call_n < 20 {
        crate::xbox::emulator::debug_log(&format!(
            "[ISR-GUARD] try_inject_isr #{}: crt_done={} isr=0x{:08X} host_off=0x{:X}",
            call_n, crt_state, isr_addr, host_offset
        ));
    }
    if !crt_state {
        return None;
    }
    // Only inject ISR into the main worker thread (worker #0).
    // Background child threads (loading, audio) must not receive ISR injection —
    // the ISR manipulates the guest stack and can corrupt the child's execution.
    {
        use crate::xbox::emulator::WORKER_THREAD_ID;
        let main_tid = WORKER_THREAD_ID.load(Ordering::Relaxed);
        let current_tid = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };
        if main_tid != 0 && current_tid != main_tid {
            return None;
        }
    }
    // Re-entry handling (from JIT d6e8ccb): if the prior injection never cleaned up,
    // force-cleanup now before starting a new one. This prevents a permanently
    // stuck injection from blocking all future vblanks.
    if INJECT_SAVED_RIP.load(Ordering::Acquire) != 0 {
        static FORCE_LOG: AtomicU32 = AtomicU32::new(0);
        let n = FORCE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 10 {
            crate::xbox::emulator::debug_log(&format!(
                "[L4-ISR-FORCE] prior injection stale (R12=0x{:016X} saved_R12=0x{:016X} delta={}) — forcing cleanup then re-injecting",
                context.R12,
                INJECT_SAVED_R12.load(Ordering::Acquire),
                context.R12 as i64 - INJECT_SAVED_R12.load(Ordering::Acquire) as i64,
            ));
        }
        handle_isr_cleanup(context);
    }

    // Look up ISR routine in addr hash. If the JIT has not yet compiled the
    // ISR entry block, the lookup fails and we silently abort injection. Set
    // a pending-compile flag so the worker dispatch loop force-compiles that
    // block before the next INT3 — on the next attempt the lookup will hit.
    let isr_host = match ctx.addr_hash.lookup(isr_addr) {
        Some(h) => h,
        None => {
            let prev = PENDING_ISR_COMPILE.swap(isr_addr, Ordering::Relaxed);
            if prev != isr_addr {
                crate::xbox::emulator::debug_log(&format!(
                    "[L4-ISR-PEND] queuing force-compile of ISR=0x{:08X} (addr_hash miss)",
                    isr_addr
                ));
            }
            return None;
        }
    };

    // Find current guest address from trap table (for return address)
    let guest_addr = ctx
        .find_trap(host_offset)
        .map(|t| t.guest_addr)
        .unwrap_or(0);

    // Safety: only inject at real guest code boundaries. Kernel thunks live at
    // 0xFFFF0000+ordinal and are not valid return addresses — if we inject here
    // the ISR's RET will jump into ordinal space and crash.
    if guest_addr == 0 || guest_addr >= 0xFFFF_0000 || guest_addr < 0x0001_0000 {
        static SKIP_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SKIP_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 5 {
            crate::xbox::emulator::debug_log(&format!(
                "[L4-ISR-SKIP] unsafe boundary guest_addr=0x{:08X} (kernel/invalid) — skipping injection",
                guest_addr
            ));
        }
        return None;
    }

    // Allocate the cleanup trap page if this is the first injection.
    let cleanup_trap = ensure_cleanup_trap();
    if cleanup_trap == 0 {
        crate::xbox::emulator::debug_log("[L4-ISR-SKIP] cleanup trap allocation failed");
        return None;
    }

    let isr_context = ISR_CONTEXT.load(Ordering::Relaxed);
    let r15_base = ctx.guest_mem_base;

    // Save current state (GPRs, R14, R12, RIP)
    save_injection_state(context, host_offset);

    // Build __stdcall frame: 3 args (ret_addr, PKINTERRUPT=0, ServiceContext)
    // ISR prototype: BOOLEAN (*ISR)(PKINTERRUPT, PVOID ServiceContext)
    let esp = context.R14 as u32;
    let new_esp = esp.wrapping_sub(12); // 3 dwords

    unsafe {
        // [esp+0] = return address (current guest addr → will resume at this INT3)
        let ptr0 = r15_base.add(new_esp as usize) as *mut u32;
        *ptr0 = guest_addr;
        // [esp+4] = PKINTERRUPT (NULL — we don't have a real interrupt object)
        let ptr1 = r15_base.add(new_esp as usize + 4) as *mut u32;
        *ptr1 = 0;
        // [esp+8] = ServiceContext
        let ptr2 = r15_base.add(new_esp as usize + 8) as *mut u32;
        *ptr2 = isr_context;
    }

    context.R14 = new_esp as u64;

    // Push cleanup sentinel onto the shadow stack so the ISR's emitted RET
    // (`mov r10, [r12]; add r12, 8; jmp r10`) lands on our INT3 trap page.
    // Matched-transition discipline: we pushed exactly one entry; the ISR's
    // final RET pops exactly one. Cherry-picked from JIT commit d6e8ccb.
    let new_r12 = context.R12.wrapping_sub(8);
    unsafe {
        *(new_r12 as *mut u64) = cleanup_trap;
    }
    context.R12 = new_r12;

    // Set NV2A PMC interrupt shadow (VBlank pending).
    // Write to shadow registers, NOT guest memory (NV2A region is PAGE_NOACCESS!).
    // Guest ISR will read these via MMIO → VEH → shadow_read.
    {
        // PMC_INTR (0x100): set PCRTC (bit 24) + PGRAPH (bit 12) pending
        let pmc_intr = super::nv2a::shadow_read(0x0100);
        super::nv2a::shadow_write(0x0100, pmc_intr | 0x0100_1000);
        // PMC_INTR_EN (0x140): master enable (bit 0) + PCRTC (bit 24) + PGRAPH (bit 12)
        let pmc_en = super::nv2a::shadow_read(0x0140);
        super::nv2a::shadow_write(0x0140, pmc_en | 0x0100_1001);
        // PCRTC_INTR (0x600100): VBlank pending (bit 0)
        let pcrtc = super::nv2a::shadow_read(0x60_0100);
        super::nv2a::shadow_write(0x60_0100, pcrtc | 0x01);
        // PGRAPH_INTR (0x400100): completion pending (bit 0)
        let pgraph = super::nv2a::shadow_read(0x40_0100);
        super::nv2a::shadow_write(0x40_0100, pgraph | 0x01);
    }

    // After ISR returns, it will hit the INT3 at host_offset → cleanup
    // ISR is expected to call KeInsertQueueDpc → sets DPC_PENDING
    //
    // 2026-04-24 ATTEMPTED BRIDGE (REVERTED): tried setting DPC_PENDING=true
    // here so the next INT3 fires the captured DPC routine. Build was clean,
    // first ISR fired (vbl#1), but the subsequent DPC fire set
    // WORKER_DPC_ACTIVE=true and the DPC body didn't complete cleanly,
    // leaving WORKER_DPC_ACTIVE permanently set and blocking all further
    // ISR injections. Game ran for 1700 swaps with single-frame ISR
    // semantics (no clock tick), then exited. The DPC body has init
    // dependencies (CDevice fields, scene_mgr, etc.) that aren't met.
    //
    // Real fix needs: scene constructor running organically first, OR a
    // restricted "tick-only" DPC stub that just writes [App+0x108] and
    // returns, bypassing the full DPC body until init is complete.
    // Multi-session work — see memory/spiderman_legal_real_blocker_2026_04_24.md.

    // Set ISR cooldown based on phase.
    // Lowered 2026-04-20 after cherry-picking d6e8ccb: prior 50K value meant
    // only ~5 attempts per 80s. Real Xbox fires VBlank at 60Hz (~16ms apart).
    // With cooldown=1000 INT3s and ~50K INT3s/sec worker rate, we get ~50 ISR
    // attempts/sec which matches VBlank cadence closely enough.
    //
    // 2026-04-25: tested candidate β (cooldown = 50) — KS-TELEM showed isr_a
    // unchanged at 5/45s. Cooldown is NOT the rate-limiting factor for
    // Spider-Man; the orchestrator's try_isr_dpc() entry frequency is.
    // Reverting to original 500/1000 split. Wall-clock pacing (candidate α)
    // is also moot until that upstream invocation rate is understood.
    let cooldown = if ctx.mmio_count == 0 {
        500u32
    } else {
        1_000u32
    };
    ISR_COOLDOWN.store(cooldown, Ordering::Relaxed);

    VBLANK_COUNT.fetch_add(1, Ordering::Relaxed);

    // Redirect to ISR
    let code_base = ctx.code_base as u64;
    context.Rip = code_base + isr_host as u64;

    crate::rate_log!(
        10,
        "[L4-ISR-INJ] VBlank ISR=0x{:08X} ctx=0x{:08X} resume=0x{:08X} vbl#{}",
        isr_addr,
        isr_context,
        guest_addr,
        VBLANK_COUNT.load(Ordering::Relaxed)
    );

    let _n_isr_s = KS_ISR_INJECTED.fetch_add(1, Ordering::Relaxed) + 1;
    ks_milestone(_n_isr_s, "isr_injected");

    Some(VehResult::Handled)
}

/// Inject PGRAPH ISR: build 3-arg stdcall frame and redirect to PGRAPH ISR routine.
/// Similar to VBlank ISR but with separate cooldown and only fires after GPU init.
#[cfg(windows)]
fn try_inject_pgraph_isr(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
    ctx: &mut RuntimeContext,
    pgraph_isr_addr: u32,
) -> Option<VehResult> {
    if !CRT_BOOT_COMPLETE.load(Ordering::Acquire) {
        return None;
    }
    let pgraph_host = ctx.addr_hash.lookup(pgraph_isr_addr)?;
    let guest_addr = ctx
        .find_trap(host_offset)
        .map(|t| t.guest_addr)
        .unwrap_or(0);

    let pgraph_context = PGRAPH_ISR_CONTEXT.load(Ordering::Relaxed);
    let r15_base = ctx.guest_mem_base;

    save_injection_state(context, host_offset);

    // Build __stdcall frame: 3 args (ret_addr, PKINTERRUPT=0, ServiceContext)
    let esp = context.R14 as u32;
    let new_esp = esp.wrapping_sub(12);

    unsafe {
        let ptr0 = r15_base.add(new_esp as usize) as *mut u32;
        *ptr0 = guest_addr;
        let ptr1 = r15_base.add(new_esp as usize + 4) as *mut u32;
        *ptr1 = 0; // PKINTERRUPT = NULL
        let ptr2 = r15_base.add(new_esp as usize + 8) as *mut u32;
        *ptr2 = pgraph_context;
    }

    context.R14 = new_esp as u64;

    // Set PGRAPH completion pending in shadow registers
    {
        let pgraph = super::nv2a::shadow_read(0x40_0100);
        super::nv2a::shadow_write(0x40_0100, pgraph | 0x01);
        let pmc_intr = super::nv2a::shadow_read(0x0100);
        super::nv2a::shadow_write(0x0100, pmc_intr | 0x0000_1000); // PGRAPH bit 12
    }

    // Cooldown: 100K dispatches between PGRAPH ISRs (much less frequent than VBlank)
    PGRAPH_ISR_COOLDOWN.store(100_000, Ordering::Relaxed);

    let code_base = ctx.code_base as u64;
    context.Rip = code_base + pgraph_host as u64;

    crate::rate_log!(
        10,
        "[L4-PGRAPH-ISR] PGRAPH ISR=0x{:08X} ctx=0x{:08X} resume=0x{:08X}",
        pgraph_isr_addr,
        pgraph_context,
        guest_addr
    );

    Some(VehResult::Handled)
}

/// Inject DPC: build 5-arg stdcall frame and redirect to DPC routine.
#[cfg(windows)]
fn try_inject_dpc(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
    ctx: &mut RuntimeContext,
    dpc_routine: u32,
) -> Option<VehResult> {
    // P0 telemetry: count every entry, regardless of guard outcome.
    let _n_dpc_a = KS_DPC_ATTEMPTS.fetch_add(1, Ordering::Relaxed) + 1;
    ks_milestone(_n_dpc_a, "dpc_attempts");

    // HARD GUARD: never inject DPC during CRT boot
    if !CRT_BOOT_COMPLETE.load(Ordering::Acquire) {
        return None;
    }

    if (0x0001_0000..0x2000_0000).contains(&dpc_routine) {
        let mut bytes = [0u8; 16];
        unsafe {
            for (i, b) in bytes.iter_mut().enumerate() {
                *b = *ctx.guest_mem_base.add(dpc_routine as usize + i);
            }
        }
        if bytes.iter().all(|&b| b == 0) {
            DPC_PENDING.store(false, Ordering::Release);
            crate::xbox::aot::veh::veh_log(&format!(
                "[L4-DPC-SKIP-ZERO-CODE] routine=0x{:08X} bytes={:02X?}",
                dpc_routine, bytes
            ));
            return None;
        }
    }

    // Look up DPC routine in addr hash
    let dpc_host = ctx.addr_hash.lookup(dpc_routine)?;

    // Find current guest address for return — must be valid or we can't resume
    let guest_addr = match ctx.find_trap(host_offset) {
        Some(t) => t.guest_addr,
        None => {
            // Can't determine resume address — re-queue DPC for next opportunity
            DPC_PENDING.store(true, Ordering::Release);
            return None;
        }
    };

    let dpc_context = DPC_CONTEXT.load(Ordering::Relaxed);
    let dpc_object = DPC_OBJECT.load(Ordering::Relaxed);
    let r15_base = ctx.guest_mem_base;

    // Save current state
    save_injection_state(context, host_offset);

    // Set Timer SignalState before firing DPC — games may KeWaitForSingleObject(timer)
    // after KeSetTimer. The timer object must be signaled before the DPC runs.
    let timer_ptr = DPC_TIMER.load(Ordering::Relaxed);
    if timer_ptr != 0 && timer_ptr < 0x1000_0000 {
        unsafe {
            // KTIMER.Header.SignalState is at offset +4 (DISPATCHER_HEADER.SignalState)
            let signal_addr = r15_base.add(timer_ptr as usize + 4) as *mut u32;
            *signal_addr = 1; // Set timer to signaled
        }
        crate::rate_log!(
            10,
            "[L4-DPC] Timer 0x{:08X} SignalState=1 before DPC fire",
            timer_ptr
        );
    }

    // Build __stdcall frame: 5 args
    // DPC prototype: VOID (*DPC)(PKDPC, PVOID DeferredContext, PVOID SA1, PVOID SA2)
    let esp = context.R14 as u32;
    let new_esp = esp.wrapping_sub(20); // 5 dwords: ret + 4 args

    unsafe {
        let base = r15_base.add(new_esp as usize);
        *(base as *mut u32) = guest_addr; // [esp+0] = return address
        *(base.add(4) as *mut u32) = dpc_object; // [esp+4] = PKDPC
        *(base.add(8) as *mut u32) = dpc_context; // [esp+8] = DeferredContext
        *(base.add(12) as *mut u32) = dpc_system_argument1(dpc_routine, dpc_context); // [esp+12] = SystemArgument1
        *(base.add(16) as *mut u32) = 0; // [esp+16] = SystemArgument2
    }

    context.R14 = new_esp as u64;

    // Redirect to DPC routine
    let code_base = ctx.code_base as u64;
    context.Rip = code_base + dpc_host as u64;

    crate::rate_log!(
        20,
        "[L4-DPC-INJ] DPC=0x{:08X} ctx=0x{:08X} obj=0x{:08X} resume=0x{:08X}",
        dpc_routine,
        dpc_context,
        dpc_object,
        guest_addr
    );

    let _n_dpc_s = KS_DPC_INJECTED.fetch_add(1, Ordering::Relaxed) + 1;
    ks_milestone(_n_dpc_s, "dpc_injected");

    Some(VehResult::Handled)
}

/// Save GPRs + R14 before injection (Pitfall #28: guest DPC/ISR clobbers everything).
#[cfg(windows)]
fn save_injection_state(
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
) {
    INJECT_RESUME_OFFSET.store(host_offset, Ordering::Release);
    INJECT_SAVED_R14.store(context.R14 as u32, Ordering::Release);
    // Save Rip + R12 for matched-transition cleanup (from JIT d6e8ccb).
    INJECT_SAVED_RIP.store(context.Rip, Ordering::Release);
    INJECT_SAVED_R12.store(context.R12, Ordering::Release);

    unsafe {
        INJECT_SAVED_GPRS = SavedGprs {
            rax: context.Rax,
            rbx: context.Rbx,
            rcx: context.Rcx,
            rdx: context.Rdx,
            rsi: context.Rsi,
            rdi: context.Rdi,
            rbp: context.Rbp,
            rflags: context.EFlags as u64,
        };
    }
}
