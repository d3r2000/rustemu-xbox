/// Matched-transition discipline (Frida/Wine-style) — Phase 3 log-only
/// variant of the plan in docs/xbe_ctor_reference.md + CLAUDE.md design
/// references section.
///
/// # Purpose
///
/// Every call boundary (kernel dispatch, HLE hook, SEH prolog/epilog,
/// emitted CALL/RET) pushes a `TransitionFrame` on enter and pops + verifies
/// it on leave. If the R14 (guest ESP) delta between enter and leave
/// doesn't match the expected stdcall cleanup (`4 + argc*4`), we log a
/// loud `[TRANSITION-FAIL]` event.
///
/// Phase 3 is **log-only**. We never abort or alter execution. The
/// existing silent-recovery paths (stack scan, shadow-stack fallback, SEH
/// HLE bookkeeping) stay active. We just SEE every mismatch.
///
/// # Design references (CLAUDE.md)
///
/// - Frida `guminterceptor.c`: explicit enter/leave matched transition
///   frames. Rewrites live caller return address to `on_leave_trampoline`,
///   restores `next_hop` on leave.
/// - Wine `dlls/ntdll/unix/file.c` + `server/fd.c`: strict stdcall
///   discipline enforced at syscall-frame boundaries. Fail at the
///   boundary, don't silently recover.
///
/// # Why this matters for rustemu
///
/// Current VEH paths silently paper over 4-byte drift (stack scan
/// recovery, emit_ret_checked hash-lookup fallback, SEH HLE frame
/// bookkeeping). Accumulated drift across many calls eventually
/// corrupts a string or pointer silently — which is exactly how the
/// current Spider-Man RtlCreateHeap stall manifests: guest code faults
/// inside a ctor, VEH papers over it, process_heap is set but the heap
/// struct is never initialized.
///
/// Matched-transition logging surfaces those silent recoveries loudly,
/// so diagnostics tell us WHICH instruction caused the fault and WHICH
/// function silently swallowed it.
use std::cell::RefCell;

/// Kind of call boundary being tracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionKind {
    /// Kernel ordinal dispatch (via trampoline exit or inline handler).
    Kernel,
    /// OOVPA-matched HLE hook (D3D8, DSOUND, etc.).
    Hle,
    /// Manually planted hook (non-OOVPA).
    Manual,
    /// `_SEH_prolog` HLE call site (game frame setup).
    SehProlog,
    /// Emitted `emit_call_rel` push to R12 shadow stack.
    EmittedCall,
}

/// One record per in-flight call. Pushed by the enter path, popped and
/// verified by the leave path. Mismatch between `expected_r14_on_leave`
/// and the actual R14 at leave time = stdcall drift.
#[derive(Debug, Clone, Copy)]
pub struct TransitionFrame {
    pub kind: TransitionKind,
    /// Guest PC the call entered. For kernel/HLE, this is the HLE function
    /// entry (or the guest instruction that trapped). For EmittedCall, this
    /// is the guest target address.
    pub guest_entry_pc: u32,
    /// Guest return address that the caller expects to return to.
    pub guest_ret_addr: u32,
    /// R14 (guest ESP) at enter time, BEFORE any cleanup.
    pub entry_r14: u32,
    /// Expected R14 after stdcall cleanup: `entry_r14 + 4 + argc*4`.
    /// (+4 for the popped return address, +4 per stdcall arg.)
    pub expected_r14_on_leave: u32,
    /// Argument count used for the expected-cleanup calculation.
    pub argc: u8,
    /// Symbolic name for logging. Usually function name or ordinal mnemonic.
    /// Bounded-length so we don't allocate on the hot path.
    pub name: &'static str,
    /// Kernel ordinal, if kind == Kernel. Zero otherwise.
    pub ordinal: u16,
}

impl TransitionFrame {
    /// Construct a new frame. Does not push — use `push()` for that.
    #[must_use]
    pub fn new(
        kind: TransitionKind,
        name: &'static str,
        entry_r14: u32,
        guest_entry_pc: u32,
        guest_ret_addr: u32,
        argc: u8,
    ) -> Self {
        let expected = entry_r14
            .wrapping_add(4)
            .wrapping_add((argc as u32).wrapping_mul(4));
        Self {
            kind,
            guest_entry_pc,
            guest_ret_addr,
            entry_r14,
            expected_r14_on_leave: expected,
            argc,
            name,
            ordinal: 0,
        }
    }

    /// Shorthand for kernel transitions where we also know the ordinal.
    #[must_use]
    pub fn new_kernel(
        ordinal: u16,
        name: &'static str,
        entry_r14: u32,
        guest_entry_pc: u32,
        guest_ret_addr: u32,
        argc: u8,
    ) -> Self {
        let mut f = Self::new(
            TransitionKind::Kernel,
            name,
            entry_r14,
            guest_entry_pc,
            guest_ret_addr,
            argc,
        );
        f.ordinal = ordinal;
        f
    }

    /// Check whether the observed R14 at leave time matches the expected
    /// stdcall cleanup delta. Returns the signed delta from expected.
    /// Zero delta = clean. Non-zero = drift (sign indicates direction).
    #[must_use]
    pub fn delta_from_expected(&self, observed_r14: u32) -> i64 {
        (observed_r14 as i64) - (self.expected_r14_on_leave as i64)
    }
}

thread_local! {
    /// Per-thread transition stack. Each worker thread gets its own, so we
    /// can compare kernel<->user transitions without cross-thread locking.
    /// `RefCell` is fine because all pushes/pops happen on the owning
    /// thread from synchronous code paths (VEH handlers and dispatch loop).
    static TRANSITION_STACK: RefCell<Vec<TransitionFrame>> =
        const { RefCell::new(Vec::new()) };
}

/// Global event counter — how many transitions have been pushed in this
/// process since load. Used for rate-limiting the loud logs.
static PUSHED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Global mismatch counter — how many TRANSITION-FAIL events have fired.
static MISMATCHED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Global verified-clean counter — pops that matched exactly.
static VERIFIED_CLEAN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Push a transition frame onto the current thread's stack.
///
/// Safe to call from anywhere. The frame stays on the stack until
/// `pop_and_verify` pops it.
///
/// Log budget: to avoid flooding we log the first 100 pushes, then every
/// 10_000th push after that.
pub fn push(frame: TransitionFrame) {
    let n = PUSHED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 100 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[TRANSITION-PUSH #{}] kind={:?} name='{}' entry_R14=0x{:08X} expected_leave=0x{:08X} argc={} ret_addr=0x{:08X} ordinal={}",
            n, frame.kind, frame.name, frame.entry_r14,
            frame.expected_r14_on_leave, frame.argc, frame.guest_ret_addr,
            frame.ordinal
        ));
    }
    TRANSITION_STACK.with(|s| s.borrow_mut().push(frame));
}

/// Result of `pop_and_verify`.
#[derive(Debug, Clone, Copy)]
pub enum VerifyResult {
    /// No frame on stack (unexpected pop, possibly unbalanced).
    Underflow,
    /// Popped and R14 matched exactly.
    Clean {
        kind: TransitionKind,
        name: &'static str,
    },
    /// Popped but R14 was off by `delta` bytes.
    Drift {
        kind: TransitionKind,
        name: &'static str,
        expected_r14: u32,
        observed_r14: u32,
        delta: i64,
    },
    /// Popped but the kind doesn't match what caller expected. Caller
    /// can decide whether to re-push or abort.
    KindMismatch {
        expected_kind: TransitionKind,
        got: TransitionFrame,
    },
}

/// Pop the top frame and verify R14 matches its expected-cleanup value.
///
/// `expected_kind` is optional (use `None` to accept any). The verify
/// is performed regardless of kind; the kind check is advisory.
///
/// Log budget for mismatches: always log the first 200 TRANSITION-FAIL
/// events, then every 1000th (noise control).
pub fn pop_and_verify(expected_kind: Option<TransitionKind>, observed_r14: u32) -> VerifyResult {
    let frame = TRANSITION_STACK.with(|s| s.borrow_mut().pop());
    let frame = match frame {
        Some(f) => f,
        None => {
            // Unbalanced pop — caller called pop without a prior push.
            // Log the first few times this happens.
            let n = MISMATCHED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::emulator::debug_log(&format!(
                    "[TRANSITION-FAIL #{}] underflow — pop with empty stack, expected_kind={:?} observed_R14=0x{:08X}",
                    n, expected_kind, observed_r14
                ));
            }
            return VerifyResult::Underflow;
        }
    };

    if let Some(ek) = expected_kind {
        if ek != frame.kind {
            // Kind mismatch — caller's expectation differs from what's on
            // top. Don't re-push here (caller decides). Just report.
            let n = MISMATCHED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::emulator::debug_log(&format!(
                    "[TRANSITION-FAIL #{}] kind-mismatch — expected {:?}, popped {:?} name='{}'",
                    n, ek, frame.kind, frame.name
                ));
            }
            return VerifyResult::KindMismatch {
                expected_kind: ek,
                got: frame,
            };
        }
    }

    let delta = frame.delta_from_expected(observed_r14);
    if delta == 0 {
        VERIFIED_CLEAN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        VerifyResult::Clean {
            kind: frame.kind,
            name: frame.name,
        }
    } else {
        let n = MISMATCHED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 200 || n % 1000 == 0 {
            crate::xbox::emulator::debug_log(&format!(
                "[TRANSITION-FAIL #{}] DRIFT name='{}' kind={:?} expected_R14=0x{:08X} observed_R14=0x{:08X} delta={:+} argc={} ordinal={} entry_PC=0x{:08X} ret_addr=0x{:08X}",
                n, frame.name, frame.kind, frame.expected_r14_on_leave,
                observed_r14, delta, frame.argc, frame.ordinal,
                frame.guest_entry_pc, frame.guest_ret_addr
            ));
        }
        VerifyResult::Drift {
            kind: frame.kind,
            name: frame.name,
            expected_r14: frame.expected_r14_on_leave,
            observed_r14,
            delta,
        }
    }
}

/// Non-destructive peek at the top frame. Useful for VEH handlers to
/// decide whether a fault happened inside a tracked transition.
pub fn peek() -> Option<TransitionFrame> {
    TRANSITION_STACK.with(|s| s.borrow().last().copied())
}

/// Current stack depth on this thread. Useful as a diagnostic.
pub fn depth() -> usize {
    TRANSITION_STACK.with(|s| s.borrow().len())
}

/// Clear the stack on this thread. Call after an unrecoverable error
/// (AOT_EXIT_ERROR, RET_TO_ZERO, etc.) so stale frames don't trigger
/// phantom mismatches on the next call sequence.
pub fn clear_on_error() {
    TRANSITION_STACK.with(|s| {
        let cleared = s.borrow().len();
        if cleared > 0 {
            s.borrow_mut().clear();
            crate::xbox::emulator::debug_log(&format!(
                "[TRANSITION-CLEAR] cleared {} stale frames after error",
                cleared
            ));
        }
    });
}

/// Read current counters for HUD / diagnostic display.
/// Returns (pushes, verified_clean, mismatches).
#[must_use]
pub fn counters() -> (u64, u64, u64) {
    (
        PUSHED.load(std::sync::atomic::Ordering::Relaxed),
        VERIFIED_CLEAN.load(std::sync::atomic::Ordering::Relaxed),
        MISMATCHED.load(std::sync::atomic::Ordering::Relaxed),
    )
}

// ============================================================================
// Unit tests — verify discipline semantics without any emulator integration.
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Counter reset via stack clear. Tests can't reset PUSHED/MISMATCHED
    // statics, so we assert on deltas instead of absolute values.
    fn counters_delta() -> (u64, u64, u64) {
        let c = counters();
        (c.0, c.1, c.2)
    }

    // NOTE: cargo test runs tests in parallel by default and our global
    // counters (PUSHED / MISMATCHED / VERIFIED_CLEAN) are process-wide, so
    // we can't assert exact deltas. We rely on the VerifyResult enum
    // variant + thread_local stack depth instead — both of those ARE
    // isolated per-test because the stack is thread_local.

    #[test]
    fn matched_kernel_transition_is_clean() {
        clear_on_error();
        let frame = TransitionFrame::new_kernel(
            184,
            "NtAllocateVirtualMemory",
            0x1EFFFE00,
            0x002A_1234,
            0x002A_1238,
            5,
        );
        // 5 args → expected +4 ret + 5*4 = +24 bytes cleanup → 0x1EFFFE18
        assert_eq!(frame.expected_r14_on_leave, 0x1EFFFE18);
        push(frame);
        assert_eq!(depth(), 1);

        // Simulate guest returning with R14 advanced by exactly 24 bytes.
        let r = pop_and_verify(Some(TransitionKind::Kernel), 0x1EFFFE18);
        match r {
            VerifyResult::Clean { kind, name } => {
                assert_eq!(kind, TransitionKind::Kernel);
                assert_eq!(name, "NtAllocateVirtualMemory");
            }
            other => panic!("expected Clean, got {:?}", other),
        }
        assert_eq!(depth(), 0);
    }

    #[test]
    fn drift_by_four_is_detected() {
        clear_on_error();
        let frame = TransitionFrame::new(
            TransitionKind::Hle,
            "D3DDevice_Swap",
            0x1EFFFE00,
            0x002F_47D0,
            0x002A_1234,
            1,
        );
        // 1 arg → expected +4 ret + 4 = +8
        assert_eq!(frame.expected_r14_on_leave, 0x1EFFFE08);
        push(frame);
        // Simulate return with R14 only advanced by 4 (missed the arg).
        let r = pop_and_verify(Some(TransitionKind::Hle), 0x1EFFFE04);
        match r {
            VerifyResult::Drift {
                delta,
                expected_r14,
                observed_r14,
                ..
            } => {
                assert_eq!(delta, -4);
                assert_eq!(expected_r14, 0x1EFFFE08);
                assert_eq!(observed_r14, 0x1EFFFE04);
            }
            other => panic!("expected Drift(-4), got {:?}", other),
        }
    }

    #[test]
    fn drift_positive_is_also_detected() {
        clear_on_error();
        let frame = TransitionFrame::new(TransitionKind::Hle, "f", 0x1EFFFE00, 0x0, 0x0, 2);
        // 2 args → expected +12
        push(frame);
        // Simulate return with R14 over-advanced by 4 bytes.
        let r = pop_and_verify(Some(TransitionKind::Hle), 0x1EFFFE10);
        match r {
            VerifyResult::Drift { delta, .. } => assert_eq!(delta, 4),
            other => panic!("expected Drift(+4), got {:?}", other),
        }
    }

    #[test]
    fn nested_transitions_pop_in_reverse_order() {
        clear_on_error();
        let outer = TransitionFrame::new_kernel(
            255,
            "PsCreateSystemThreadEx",
            0x1EFFFE00,
            0x002B_0000,
            0x002B_0004,
            10,
        );
        let inner = TransitionFrame::new(
            TransitionKind::Hle,
            "inner_hle",
            outer.expected_r14_on_leave,
            0x002B_1000,
            0x002B_1004,
            2,
        );
        push(outer);
        push(inner);
        assert_eq!(depth(), 2);

        // Pop inner first (LIFO).
        let r = pop_and_verify(Some(TransitionKind::Hle), inner.expected_r14_on_leave);
        assert!(matches!(r, VerifyResult::Clean { .. }));
        assert_eq!(depth(), 1);

        // Now outer.
        let r = pop_and_verify(Some(TransitionKind::Kernel), outer.expected_r14_on_leave);
        assert!(matches!(r, VerifyResult::Clean { .. }));
        assert_eq!(depth(), 0);
    }

    #[test]
    fn underflow_on_pop_without_push() {
        clear_on_error();
        let r = pop_and_verify(Some(TransitionKind::Kernel), 0x1EFFFE00);
        assert!(matches!(r, VerifyResult::Underflow));
    }

    #[test]
    fn clear_on_error_discards_all_frames() {
        clear_on_error();
        for i in 0..5 {
            push(TransitionFrame::new_kernel(
                100 + i,
                "test",
                0x1EFFFE00,
                0x10000,
                0x10004,
                0,
            ));
        }
        assert_eq!(depth(), 5);
        clear_on_error();
        assert_eq!(depth(), 0);
    }

    #[test]
    fn kind_mismatch_does_not_reinsert() {
        clear_on_error();
        let frame = TransitionFrame::new(
            TransitionKind::Kernel,
            "kernel_fn",
            0x1EFFFE00,
            0x002A_0000,
            0x002A_0004,
            0,
        );
        push(frame);
        assert_eq!(depth(), 1);

        // Pop expecting HLE but top is Kernel.
        let r = pop_and_verify(Some(TransitionKind::Hle), 0x1EFFFE04);
        match r {
            VerifyResult::KindMismatch { got, .. } => {
                assert_eq!(got.kind, TransitionKind::Kernel);
            }
            other => panic!("expected KindMismatch, got {:?}", other),
        }
        // Frame was popped, not re-pushed. Caller is responsible.
        assert_eq!(depth(), 0);
    }

    #[test]
    fn delta_from_expected_computes_signed() {
        let frame = TransitionFrame::new(TransitionKind::Hle, "x", 0x1000, 0x100, 0x104, 0);
        assert_eq!(frame.expected_r14_on_leave, 0x1004);
        assert_eq!(frame.delta_from_expected(0x1004), 0);
        assert_eq!(frame.delta_from_expected(0x1008), 4);
        assert_eq!(frame.delta_from_expected(0x1000), -4);
    }

    #[test]
    fn stdcall_cleanup_formula_is_correct() {
        // stdcall: caller pushes args, callee pops ret+args on exit.
        // After the callee returns, caller's ESP is at entry_esp + 4 + argc*4.
        // 0 args → +4 (just ret)
        assert_eq!(
            TransitionFrame::new(TransitionKind::Kernel, "f", 0, 0, 0, 0).expected_r14_on_leave,
            4
        );
        // 5 args → +24
        assert_eq!(
            TransitionFrame::new(TransitionKind::Kernel, "f", 0, 0, 0, 5).expected_r14_on_leave,
            24
        );
        // 10 args → +44
        assert_eq!(
            TransitionFrame::new(TransitionKind::Kernel, "f", 0, 0, 0, 10).expected_r14_on_leave,
            44
        );
    }
}
