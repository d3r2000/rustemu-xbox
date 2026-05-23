//! Guest-aware SEH dispatcher — a minimum port of Wine's
//! `dlls/msvcrt/except_i386.c` + `dlls/ntdll/exception.c` exception flow,
//! adapted to our emulated x86 guest running on a Rust x86-64 host.
//!
//! # Problem this solves
//!
//! Our VEH (veh_dispatch.rs) intercepts host-level exceptions (AVs, INT3s,
//! single-step, etc.) and either fixes them up (R15 sign-extension,
//! NV2A MMIO, kernel INT3s) or silently skips. That works for anything
//! where the emulator is the "authority" — but it is **wrong** for
//! guest-initiated exceptions that the game expects to catch via its own
//! `try/__except` blocks.
//!
//! Spider-Man's master init function (sub_002A52A3) wraps its boot in a
//! `try/except` whose scope table we've already reverse-engineered:
//!
//! ```text
//!   filter  = 0x002A540C  (returns 1 if App->[0x50] is set)
//!   handler = 0x002A542B  (cleans up global at 0x3F82C8, returns cleanly)
//! ```
//!
//! When the Frame ctor's THISCALL body (sub_0xD67E0) throws, Windows SEH
//! is supposed to walk `fs:[0]`, find this handler, call the filter with
//! `EXCEPTION_POINTERS*`, run the handler block, then return normally
//! from master init. Our VEH currently swallows the AV without ever
//! invoking the guest's filter or handler — so the game never gets a
//! chance to clean up and continue boot.
//!
//! # What this file provides
//!
//! Minimum subset of Wine's dispatcher, for the **old-style MSVC
//! `_except_handler3` ABI** that Spider-Man (XDK 4134, built with MSVC
//! 7.x) uses. C++ exceptions (`__CxxFrameHandler` / `cxx_function_descr`)
//! are NOT in scope here — the game uses the simpler scope-table form.
//!
//! # Current status: SKELETON ONLY
//!
//! Stage 1 of a 6-stage port. This file compiles and its types match
//! Wine's layout, but nothing dispatches yet. Integration with
//! `veh_dispatch` happens in Stage 4. See the todo list on the commit
//! that introduced this file for the full staged plan.

#![allow(dead_code)] // Skeleton — many items unused until Stage 4.

use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// Guest-side types — layout must match MSVC 7.x exactly.
//
// All fields are 32-bit little-endian. Offsets are identical to those in
// Wine's `dlls/msvcrt/except_i386.c` except where noted. These structs are
// intentionally NOT `#[repr(C)]` Rust types: the guest owns the memory,
// we only read/write through GuestMemory with explicit offsets. The
// `#[allow(non_camel_case_types)]` matches Windows/Wine conventions.
// ============================================================================

/// Mirrors `EXCEPTION_REGISTRATION_RECORD` (the link in the `fs:[0]` chain).
/// 8 bytes. Every `_SEH_prolog` pushes one of these onto the guest stack.
pub const SEH_REG_REC_PREV_OFFSET: u32 = 0x00; // prev EXCEPTION_REGISTRATION_RECORD*
pub const SEH_REG_REC_HANDLER_OFFSET: u32 = 0x04; // handler function (fn ptr)
pub const SEH_REG_REC_SIZE: u32 = 0x08;

/// Mirrors `MSVCRT_EXCEPTION_FRAME` from Wine except_i386.c:81-90.
/// Layout of the extended frame that `_SEH_prolog` actually pushes for
/// `_except_handler3`-using functions (like Spider-Man's master init).
/// 24 bytes in total.
pub const MSVCRT_EF_PREV_OFFSET: u32 = 0x00; // prev *EXCEPTION_REGISTRATION_RECORD
pub const MSVCRT_EF_HANDLER_OFFSET: u32 = 0x04; // = 0x002B7750 for Spider-Man (_except_handler3)
pub const MSVCRT_EF_SCOPETABLE_OFFSET: u32 = 0x08; // PSCOPETABLE
pub const MSVCRT_EF_TRYLEVEL_OFFSET: u32 = 0x0C; // int
pub const MSVCRT_EF_EBP_OFFSET: u32 = 0x10; // saved ebp
pub const MSVCRT_EF_XPOINTERS_OFFSET: u32 = 0x14; // PEXCEPTION_POINTERS
pub const MSVCRT_EF_SIZE: u32 = 0x18;

/// Mirrors `SCOPETABLE` entry — 12 bytes.
/// Spider-Man's scope table at 0x3AEB00 has exactly 1 entry:
/// { previousTryLevel=-1, filter=0x002A540C, handler=0x002A542B }.
pub const SCOPETABLE_PREV_TRY_LEVEL: u32 = 0x00; // int
pub const SCOPETABLE_FILTER_OFFSET: u32 = 0x04; // int (*)(PEXCEPTION_POINTERS)
pub const SCOPETABLE_HANDLER_OFFSET: u32 = 0x08; // void* (*)(void)
pub const SCOPETABLE_ENTRY_SIZE: u32 = 0x0C;

/// Mirrors `EXCEPTION_RECORD`. The dispatcher writes one of these into
/// guest memory before invoking filters. Minimum fields needed for
/// Spider-Man's filter (which only inspects its frame).
pub const ER_EXCEPTION_CODE_OFFSET: u32 = 0x00; // DWORD
pub const ER_EXCEPTION_FLAGS_OFFSET: u32 = 0x04; // DWORD (EH_NONCONTINUABLE | EH_UNWINDING etc.)
pub const ER_EXCEPTION_RECORD_OFFSET: u32 = 0x08; // PEXCEPTION_RECORD (nested, usually NULL)
pub const ER_EXCEPTION_ADDRESS_OFFSET: u32 = 0x0C; // PVOID (faulting guest PC)
pub const ER_NUMBER_PARAMETERS_OFFSET: u32 = 0x10; // DWORD
pub const ER_INFORMATION_ARRAY_OFFSET: u32 = 0x14; // ULONG_PTR[15]
pub const ER_SIZE: u32 = 0x50; // 80 bytes total

/// Mirrors `EXCEPTION_POINTERS` — just two pointers.
pub const EP_EXCEPTION_RECORD_OFFSET: u32 = 0x00; // PEXCEPTION_RECORD
pub const EP_CONTEXT_OFFSET: u32 = 0x04; // PCONTEXT
pub const EP_SIZE: u32 = 0x08;

/// `ExceptionFlags` bits that matter for our dispatch.
pub const EH_NONCONTINUABLE: u32 = 0x01;
pub const EH_UNWINDING: u32 = 0x02;
pub const EH_EXIT_UNWIND: u32 = 0x04;
pub const EH_STACK_INVALID: u32 = 0x08;
pub const EH_NESTED_CALL: u32 = 0x10;

/// Filter return codes (Windows/MSVC).
pub const EXCEPTION_EXECUTE_HANDLER: i32 = 1;
pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
pub const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

/// Sentinel for "top of the fs:[0] chain". Windows uses 0xFFFFFFFF.
pub const FS_CHAIN_END: u32 = 0xFFFF_FFFF;

/// Sentinel trylevel meaning "outermost / end of list".
pub const TRYLEVEL_END: i32 = -1;

// ============================================================================
// Dispatcher public API
// ============================================================================

/// Outcome of guest SEH dispatch — what the caller (veh_dispatch) should do
/// with the host CONTEXT after we return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchResult {
    /// No guest handler accepted — fall back to existing silent-recovery or
    /// crash path. Caller should preserve original behavior.
    NotHandled,

    /// A filter returned `EXCEPTION_CONTINUE_EXECUTION` — resume the guest
    /// at the same PC. Caller should let execution continue with the
    /// (possibly modified) CONTEXT.
    ContinueExecution,

    /// A filter returned `EXCEPTION_EXECUTE_HANDLER` — we've unwound to
    /// that frame and the caller's CONTEXT has been updated so the next
    /// guest instruction is the handler block. Caller should jump to it.
    Handled {
        /// Guest PC of the catch block we should resume at.
        handler_pc: u32,
        /// Guest ESP after unwinding to the handler frame.
        esp: u32,
        /// Guest EBP after unwinding.
        ebp: u32,
    },
}

/// A snapshot of one `MSVCRT_EXCEPTION_FRAME` entry in the guest `fs:[0]`
/// chain, with its guest address preserved so we can refer back to it
/// during unwind.
#[derive(Debug, Clone, Copy)]
pub struct SehFrameSnapshot {
    /// Guest address of this frame record itself (where the fs:[0] link
    /// points). Needed as the "frame ID" during RtlUnwind.
    pub frame_addr: u32,
    /// Previous frame address (next link in the chain).
    pub prev: u32,
    /// Handler function pointer (for MSVCRT frames, typically
    /// `_except_handler3` at 0x002B7750 in Spider-Man).
    pub handler: u32,
    /// Pointer to the scopetable (MSVCRT_EXCEPTION_FRAME only).
    pub scopetable: u32,
    /// Current try level (MSVCRT_EXCEPTION_FRAME only).
    pub trylevel: i32,
    /// Saved EBP (MSVCRT_EXCEPTION_FRAME only).
    pub ebp: u32,
}

/// Walk the guest `fs:[0]` SEH chain and collect snapshots of each frame.
/// Returns up to `max_frames` entries, stopping when we hit `FS_CHAIN_END`
/// (0xFFFFFFFF), a cycle, an address that's not readable, or the cap.
///
/// This treats EVERY frame as the MSVCRT extended form (24 bytes). That's
/// safe for Spider-Man because all its functions using SEH use
/// `_except_handler3` and therefore this layout. If we later emulate games
/// that mix frame kinds, we'd distinguish via the handler function pointer.
///
/// In our memory model, `fs:[0]` is emulated as guest address `0x00000000`
/// (see veh.rs `FAKE_TEB_BASE = 0`) — `_SEH_prolog` writes ESP to `[0]`
/// after pushing its frame, so reading `[0]` yields the top frame.
pub fn walk_chain(
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    max_frames: usize,
) -> Vec<SehFrameSnapshot> {
    CHAIN_WALKS.fetch_add(1, Ordering::Relaxed);

    let mut out = Vec::with_capacity(max_frames.min(16));
    let mut seen = std::collections::HashSet::<u32>::new();

    // fs:[0] is at guest address 0 in our model.
    let mut cur = memory.read_u32(0x0000_0000);

    while cur != FS_CHAIN_END && cur != 0 && out.len() < max_frames {
        if !seen.insert(cur) {
            // Cycle — bail.
            break;
        }
        // Bounds check: guest RAM is 0x00000000..0x20000000 (512MB).
        // Stack frames live in 0x00C00000..0x1F000000 typically.
        if cur >= 0x2000_0000 {
            break;
        }

        let prev = memory.read_u32(cur + MSVCRT_EF_PREV_OFFSET);
        let handler = memory.read_u32(cur + MSVCRT_EF_HANDLER_OFFSET);
        let scopetable = memory.read_u32(cur + MSVCRT_EF_SCOPETABLE_OFFSET);
        let trylevel = memory.read_u32(cur + MSVCRT_EF_TRYLEVEL_OFFSET) as i32;
        let ebp = memory.read_u32(cur + MSVCRT_EF_EBP_OFFSET);

        out.push(SehFrameSnapshot {
            frame_addr: cur,
            prev,
            handler,
            scopetable,
            trylevel,
            ebp,
        });

        cur = prev;
    }

    out
}

/// Result of evaluating a filter — mirrors the Windows filter return
/// convention (1 = handle, 0 = continue search, -1 = continue execution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterResult {
    /// `EXCEPTION_CONTINUE_SEARCH` — filter said "not me, try the next
    /// frame."
    ContinueSearch,
    /// `EXCEPTION_EXECUTE_HANDLER` — filter accepts the exception, the
    /// dispatcher should unwind and run the handler block.
    ExecuteHandler,
    /// `EXCEPTION_CONTINUE_EXECUTION` — filter says "fix it up and
    /// resume" (used e.g. by page-commit-on-demand handlers).
    ContinueExecution,
}

// ============================================================================
// Stage 5 helpers: unwind + guest-memory writes
// ============================================================================

/// `fs:[0]` in our model lives at guest address 0 (FAKE_TEB_BASE).
/// Matches the C++ v1 Xbox kernel convention.
const fn fs_zero_addr() -> u32 {
    0
}

fn write_u32_guest(memory: &crate::xbox::memory::guest_memory::GuestMemory, addr: u32, val: u32) {
    if addr < 0x2000_0000 {
        memory.write_u32(addr, val);
    }
}

/// Stage 5 — walk intermediate SEH frames between the current top and the
/// target and "unwind" them.
///
/// **Full Wine semantics** would invoke each intermediate frame's handler
/// with `EH_UNWINDING | EH_EXIT_UNWIND` flags so C++ destructors + __finally
/// blocks can run. That requires a call-guest-code-from-Rust primitive we
/// don't have yet (the guest code would recurse into VEH and so on).
///
/// **Pragmatic implementation (2026-04-21):** Log a warning for any
/// intermediate frame whose scopetable shows an active try-level (indicating
/// it may have __finally blocks that should run), then let the caller pop
/// them from the chain wholesale by rewriting fs:[0]. For Spider-Man and
/// most games, intermediate frames between a throw site and a catch rarely
/// have __finally — they're passive in the sense that skipping them is safe.
///
/// Returns the number of frames "unwound" (for diagnostics / UNWINDS_PERFORMED).
fn perform_unwind(
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    chain: &[SehFrameSnapshot],
    target_frame_addr: u32,
) -> u32 {
    let mut popped = 0u32;
    for frame in chain {
        if frame.frame_addr == target_frame_addr {
            break;
        }
        popped += 1;
        // Warn if the intermediate frame has an active try-level. In a full
        // Wine port we'd call its handler with EH_UNWINDING here.
        if frame.trylevel != TRYLEVEL_END && frame.handler == 0x002B_7750 {
            // Active _except_handler3 frame with an open __try — if any
            // enclosing scope has a __finally it won't run. Log once.
            static WARN_CTR: AtomicU64 = AtomicU64::new(0);
            let n = WARN_CTR.fetch_add(1, Ordering::Relaxed);
            if n < 8 {
                crate::xbox::emulator::debug_log(&format!(
                    "[SEH-UNWIND] skipping __finally invocation for frame 0x{:08X} \
                     (trylevel={}, handler=0x{:08X}) — call-guest-from-Rust not \
                     implemented. If destructor bugs appear, this is why.",
                    frame.frame_addr, frame.trylevel, frame.handler
                ));
            }
        }
        // Unlink visually — actual pop happens in the caller by rewriting
        // fs:[0] to the target's prev after this returns.
        let _ = memory; // reserved for future __finally invocation
    }
    popped
}

/// Raw-pointer entry for Stage 4 VEH integration. Used from veh.rs where
/// we have `r15_base: u64` rather than a `&GuestMemory`. Equivalent logic
/// to `evaluate_filter`, reads via unsafe raw-pointer deref (bounds
/// checked). Returns the same `FilterResult` values.
///
/// # Safety
///
/// `r15_base` must be the 4GB guest memory reservation base. The function
/// reads u32/u8 from `r15_base + guest_addr` for a few specific addresses
/// that are always in guest RAM (< 0x20000000).
pub unsafe fn evaluate_filter_raw(r15_base: u64, filter_pc: u32) -> FilterResult {
    FILTERS_CALLED.fetch_add(1, Ordering::Relaxed);

    #[inline(always)]
    unsafe fn rd_u32(r15: u64, guest_addr: u32) -> u32 {
        if guest_addr >= 0x2000_0000 {
            return 0;
        }
        unsafe { std::ptr::read_unaligned((r15 + guest_addr as u64) as *const u32) }
    }
    #[inline(always)]
    unsafe fn rd_u8(r15: u64, guest_addr: u32) -> u8 {
        if guest_addr >= 0x2000_0000 {
            return 0;
        }
        unsafe { *((r15 + guest_addr as u64) as *const u8) }
    }

    match filter_pc {
        // See the docstring of `evaluate_filter` for the Spider-Man
        // master-init filter assembly.
        0x002A_540C => {
            let app_ptr = unsafe { rd_u32(r15_base, 0x003F_5EB0) };
            if app_ptr == 0 || app_ptr >= 0x2000_0000 {
                return FilterResult::ContinueSearch;
            }
            let flag = unsafe { rd_u8(r15_base, app_ptr + 0x50) };
            if flag == 0 {
                FilterResult::ContinueSearch
            } else {
                FilterResult::ExecuteHandler
            }
        }
        _ => {
            static UNKNOWN_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = UNKNOWN_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 8 {
                crate::xbox::emulator::debug_log(&format!(
                    "[SEH-FILTER-RAW #{}] unknown filter at guest 0x{:08X} — CONTINUE_SEARCH",
                    n, filter_pc
                ));
            }
            FilterResult::ContinueSearch
        }
    }
}

/// Evaluate a filter function at the given guest PC.
///
/// # Why this emulates rather than re-enters the JIT
///
/// A fully general implementation would (a) push `EXCEPTION_POINTERS*` as
/// the filter arg on the guest stack, (b) push a sentinel return address,
/// (c) re-enter the trampoline at the filter's host code offset, (d) wait
/// for the guest `ret` to bounce off the sentinel (via our RET_TO_ZERO
/// exit), (e) capture EAX from the saved guest state, (f) restore the
/// pre-filter context. That's ~100 LOC of coroutine-style state machine
/// spanning VEH invocations and rigorous re-entrancy handling.
///
/// For Spider-Man specifically, every `_except_handler3`-using function
/// ends up with a small, static filter whose logic we can read statically
/// from the XBE. We pattern-match on the filter's entry PC and run an
/// equivalent Rust implementation. Unknown filters conservatively return
/// `ContinueSearch` so the exception propagates to an outer frame — that
/// matches "no handler accepted" behaviour and doesn't break anything.
///
/// This is acknowledged tech debt. A later pass can replace each
/// pattern-matched arm with a real trampoline call and drop the
/// whitelist. Until then this is enough to let Spider-Man boot.
pub fn evaluate_filter(
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    filter_pc: u32,
    _frame: &SehFrameSnapshot,
    _exception_record_addr: u32,
) -> FilterResult {
    FILTERS_CALLED.fetch_add(1, Ordering::Relaxed);

    match filter_pc {
        // Spider-Man master init filter at 0x002A540C:
        //     mov eax, [0x3f5eb0]      ; App ptr
        //     test eax, eax
        //     je return_zero
        //     mov cl, [eax + 0x50]     ; App->flag_0x50
        //     test al, al              ; note: al not cl — quirk, same sign bit
        //     je return_zero
        //     mov eax, 1; ret
        //   return_zero:
        //     xor eax, eax; ret
        0x002A_540C => {
            let app_ptr = memory.read_u32(0x003F_5EB0);
            if app_ptr == 0 || app_ptr >= 0x2000_0000 {
                return FilterResult::ContinueSearch;
            }
            let flag = memory.read_u8(app_ptr + 0x50);
            if flag == 0 {
                FilterResult::ContinueSearch
            } else {
                FilterResult::ExecuteHandler
            }
        }

        // Unknown filter — conservative default is CONTINUE_SEARCH so the
        // exception keeps walking the chain. Worst case the outermost
        // frame gets it and the OS default applies (which for us means
        // our existing VEH fallback). This is strictly no-worse than
        // pre-dispatcher behaviour.
        _ => {
            static UNKNOWN_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = UNKNOWN_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 8 {
                crate::xbox::emulator::debug_log(&format!(
                    "[SEH-FILTER #{}] unknown filter at guest 0x{:08X} — returning CONTINUE_SEARCH",
                    n, filter_pc
                ));
            }
            FilterResult::ContinueSearch
        }
    }
}

/// Given a scopetable pointer, read the `SCOPETABLE` entry at
/// `scopetable[index]` and return its (prev_try_level, filter, handler)
/// triple. Returns `None` for obviously-invalid scopetable pointers.
pub fn read_scopetable_entry(
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    scopetable: u32,
    index: u32,
) -> Option<(i32, u32, u32)> {
    if scopetable == 0 || scopetable >= 0x2000_0000 {
        return None;
    }
    let entry_addr = scopetable + index * SCOPETABLE_ENTRY_SIZE;
    // Rough bounds: scopetables live in .data / .rdata which is < 0x800000.
    if entry_addr < 0x0001_0000 || entry_addr >= 0x2000_0000 {
        return None;
    }
    let prev_try = memory.read_u32(entry_addr + SCOPETABLE_PREV_TRY_LEVEL) as i32;
    let filter = memory.read_u32(entry_addr + SCOPETABLE_FILTER_OFFSET);
    let handler = memory.read_u32(entry_addr + SCOPETABLE_HANDLER_OFFSET);
    Some((prev_try, filter, handler))
}

/// Debug: log the current SEH chain to the emulator log stream. Used as
/// a smoke test from VEH when an exception fires, to confirm our chain
/// reader matches the game's runtime state before we start invoking
/// filters.
pub fn log_chain(memory: &crate::xbox::memory::guest_memory::GuestMemory, tag: &str) {
    let frames = walk_chain(memory, 32);
    if frames.is_empty() {
        crate::xbox::emulator::debug_log(&format!("[SEH-CHAIN] {} (empty or unreadable)", tag));
        return;
    }
    crate::xbox::emulator::debug_log(&format!("[SEH-CHAIN] {} depth={}", tag, frames.len()));
    for (i, f) in frames.iter().enumerate() {
        // Only MSVCRT frames (scopetable != 0) have a meaningful trylevel.
        // For simple EXCEPTION_REGISTRATION_RECORD entries the scopetable
        // field just happens to land in whatever memory follows — but for
        // diagnostic clarity we report it and let the reader notice when
        // the handler isn't 0x002B7750.
        let first_scope = read_scopetable_entry(memory, f.scopetable, 0);
        let scope_str = match first_scope {
            Some((pt, filt, hnd)) => format!(
                " scope[0]=(prev_try={}, filter=0x{:08X}, handler=0x{:08X})",
                pt, filt, hnd
            ),
            None => String::new(),
        };
        crate::xbox::emulator::debug_log(&format!(
            "[SEH-CHAIN]   [{}] frame=0x{:08X} prev=0x{:08X} handler=0x{:08X} \
             scopetable=0x{:08X} trylevel={} ebp=0x{:08X}{}",
            i, f.frame_addr, f.prev, f.handler, f.scopetable, f.trylevel, f.ebp, scope_str
        ));
    }
}

/// Main entry point. Called from VEH when a guest instruction caused an
/// exception (access violation, divide-by-zero, explicit `int 3`, etc.)
/// and we want to give the guest a chance to catch it.
///
/// Stages 2-3 implemented: walks fs:[0] chain, evaluates each frame's
/// filter (via `evaluate_filter` which pattern-matches known filters),
/// and reports the outcome. Stage 5 (RtlUnwind) and Stage 6 (resume at
/// handler) still need to happen; for now an `ExecuteHandler` result
/// produces a `Handled{...}` return but the caller (VEH — Stage 4)
/// won't actually be wired yet.
pub fn dispatch_guest_exception(
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    exception_code: u32,
    fault_guest_pc: u32,
    fault_guest_esp: u32,
    _fault_info: &[u32],
) -> DispatchResult {
    DISPATCH_ATTEMPTS.fetch_add(1, Ordering::Relaxed);

    // Log only periodically to avoid drowning the log during floods.
    static LOG_CTR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = LOG_CTR.fetch_add(1, Ordering::Relaxed);
    let should_log = n < 8 || n.is_power_of_two();
    if should_log {
        crate::xbox::emulator::debug_log(&format!(
            "[SEH-DISPATCH #{}] code=0x{:08X} guest_pc=0x{:08X} guest_esp=0x{:08X}",
            n, exception_code, fault_guest_pc, fault_guest_esp
        ));
        log_chain(memory, &format!("at dispatch #{}", n));
    }

    // Walk the SEH chain. For each MSVCRT frame, look at its scopetable's
    // current try-level entry, run the filter, decide.
    let frames = walk_chain(memory, 32);
    for frame in &frames {
        // Only MSVCRT_EXCEPTION_FRAMEs have a valid scopetable. We detect
        // them by the known handler address. Spider-Man uses 0x002B7750
        // for _except_handler3. Other handler addresses (plain vectored,
        // custom) we skip for now.
        if frame.handler != 0x002B_7750 {
            continue;
        }
        // The current try-level decides which scopetable entry's filter
        // we should run. trylevel == TRYLEVEL_END means no active try.
        if frame.trylevel == TRYLEVEL_END {
            continue;
        }
        let Some((prev_try, filter_pc, handler_pc)) =
            read_scopetable_entry(memory, frame.scopetable, frame.trylevel as u32)
        else {
            continue;
        };

        let result = evaluate_filter(memory, filter_pc, frame, 0 /* er not yet built */);
        if should_log {
            crate::xbox::emulator::debug_log(&format!(
                "[SEH-DISPATCH #{}]   filter@0x{:08X} trylevel={} prev_try={} result={:?}",
                n, filter_pc, frame.trylevel, prev_try, result
            ));
        }
        match result {
            FilterResult::ContinueSearch => continue,
            FilterResult::ContinueExecution => {
                return DispatchResult::ContinueExecution;
            }
            FilterResult::ExecuteHandler => {
                HANDLERS_INVOKED.fetch_add(1, Ordering::Relaxed);

                // ---- Stage 5: RtlUnwind ----
                //
                // Between the current fs:[0] top and the target frame
                // there may be intermediate SEH frames that need their
                // handlers invoked with EH_UNWINDING | EH_EXIT_UNWIND so
                // they can release locks, run C++ destructors, close
                // handles, etc.
                //
                // Full Wine semantics require CALLING each intermediate
                // handler as guest code with a synthesized EXCEPTION_RECORD
                // where ExceptionCode = STATUS_UNWIND (0xC0000027), flags
                // include EH_UNWINDING. That's a "call-guest-from-Rust"
                // primitive we don't have yet (would need a
                // stack-switch-and-resume trampoline like kernel_trampoline
                // but with a pre-built frame).
                //
                // Pragmatic Stage 5: POP the intermediate frames from the
                // chain without invoking their handlers. This matches
                // what MSVC's __except does for frames that have no
                // `__finally` block — the runtime simply doesn't call
                // them. For frames that DO have __finally, a full
                // invocation is needed; we emit a warning log so those
                // cases become visible if they matter.
                //
                // Stage 6 responsibility (caller — VEH) is then just
                // to jump to handler_pc with the unwound ESP/EBP.
                perform_unwind(memory, &frames, frame.frame_addr);

                // Unlink the target frame and everything above by setting
                // fs:[0] to the target frame's PREV link. Now fs:[0] =
                // whatever frame the catch's enclosing code is in.
                let new_chain_top = frame.prev;
                write_u32_guest(memory, fs_zero_addr(), new_chain_top);

                // Also update the target frame's trylevel to the
                // enclosing level (`prev_try` from the scopetable),
                // so if the handler body re-throws within the same
                // function, we catch it at the right outer try level.
                // _SEH_prolog stores trylevel at EBP-4 by convention.
                let trylevel_slot = frame.ebp.wrapping_sub(4);
                write_u32_guest(memory, trylevel_slot, prev_try as u32);

                UNWINDS_PERFORMED.fetch_add(1, Ordering::Relaxed);

                if should_log {
                    crate::xbox::emulator::debug_log(&format!(
                        "[SEH-DISPATCH #{}] UNWIND complete → handler_pc=0x{:08X} \
                         chain_top 0x{:08X} → 0x{:08X} trylevel[EBP-4]={}",
                        n, handler_pc, frame.frame_addr, new_chain_top, prev_try
                    ));
                }

                // ---- Stage 6: Resume-at-handler ----
                //
                // Return the Handled result. Caller (VEH integration)
                // sets the host CONTEXT so:
                //   Rip = handler_pc
                //   R14 (guest ESP) = frame.frame_addr (the unwound SP)
                //   Rbp = frame.ebp
                //   Rax = 0 (per MSVC convention — cleared entry)
                // Then returns EXCEPTION_CONTINUE_EXECUTION from the VEH
                // handler and Windows resumes guest execution at the
                // handler block.
                return DispatchResult::Handled {
                    handler_pc,
                    esp: frame.frame_addr,
                    ebp: frame.ebp,
                };
            }
        }
    }

    DispatchResult::NotHandled
}

// ============================================================================
// Stats (for debug logging / HUD)
// ============================================================================

pub static DISPATCH_ATTEMPTS: AtomicU64 = AtomicU64::new(0);
pub static CHAIN_WALKS: AtomicU64 = AtomicU64::new(0);
pub static FILTERS_CALLED: AtomicU64 = AtomicU64::new(0);
pub static HANDLERS_INVOKED: AtomicU64 = AtomicU64::new(0);
pub static UNWINDS_PERFORMED: AtomicU64 = AtomicU64::new(0);

/// Human-readable snapshot of dispatcher activity for the HUD / periodic log.
pub fn stats() -> (u64, u64, u64, u64, u64) {
    (
        DISPATCH_ATTEMPTS.load(Ordering::Relaxed),
        CHAIN_WALKS.load(Ordering::Relaxed),
        FILTERS_CALLED.load(Ordering::Relaxed),
        HANDLERS_INVOKED.load(Ordering::Relaxed),
        UNWINDS_PERFORMED.load(Ordering::Relaxed),
    )
}

// ============================================================================
// Unit tests (structure-only — real dispatch tests land in Stage 3+)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_match_wine_layout() {
        // Wine's except_i386.c:81-90 defines MSVCRT_EXCEPTION_FRAME.
        // Verify we're byte-compatible.
        assert_eq!(MSVCRT_EF_PREV_OFFSET, 0x00);
        assert_eq!(MSVCRT_EF_HANDLER_OFFSET, 0x04);
        assert_eq!(MSVCRT_EF_SCOPETABLE_OFFSET, 0x08);
        assert_eq!(MSVCRT_EF_TRYLEVEL_OFFSET, 0x0C);
        assert_eq!(MSVCRT_EF_EBP_OFFSET, 0x10);
        assert_eq!(MSVCRT_EF_XPOINTERS_OFFSET, 0x14);
        assert_eq!(MSVCRT_EF_SIZE, 0x18);
    }

    #[test]
    fn scopetable_entry_size_matches_wine() {
        // Wine except_i386.c:74-79 — 3 dwords per entry.
        assert_eq!(SCOPETABLE_PREV_TRY_LEVEL, 0);
        assert_eq!(SCOPETABLE_FILTER_OFFSET, 4);
        assert_eq!(SCOPETABLE_HANDLER_OFFSET, 8);
        assert_eq!(SCOPETABLE_ENTRY_SIZE, 12);
    }

    #[test]
    fn exception_record_size_matches_windows() {
        // sizeof(EXCEPTION_RECORD) on 32-bit Windows = 80 bytes.
        assert_eq!(ER_SIZE, 80);
    }

    #[test]
    fn walk_chain_handles_empty_tib() {
        // Build a synthetic GuestMemory where fs:[0] (guest addr 0) is
        // FS_CHAIN_END. Chain walker should return an empty Vec.
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        mem.write_u32(0x0, FS_CHAIN_END);
        let frames = walk_chain(&mem, 32);
        assert!(frames.is_empty());
    }

    #[test]
    fn walk_chain_reads_single_frame() {
        // Plant one MSVCRT_EXCEPTION_FRAME at 0x1000 and point fs:[0] at it.
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        let frame_va = 0x1_0000u32;
        mem.write_u32(0x0, frame_va);
        mem.write_u32(frame_va + MSVCRT_EF_PREV_OFFSET, FS_CHAIN_END);
        mem.write_u32(frame_va + MSVCRT_EF_HANDLER_OFFSET, 0x002B_7750);
        mem.write_u32(frame_va + MSVCRT_EF_SCOPETABLE_OFFSET, 0x003A_EB00);
        mem.write_u32(frame_va + MSVCRT_EF_TRYLEVEL_OFFSET, 0xFFFF_FFFF); // -1 (END)
        mem.write_u32(frame_va + MSVCRT_EF_EBP_OFFSET, 0x1EFF_FFE0);
        let frames = walk_chain(&mem, 32);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].frame_addr, frame_va);
        assert_eq!(frames[0].prev, FS_CHAIN_END);
        assert_eq!(frames[0].handler, 0x002B_7750);
        assert_eq!(frames[0].scopetable, 0x003A_EB00);
        assert_eq!(frames[0].trylevel, -1);
        assert_eq!(frames[0].ebp, 0x1EFF_FFE0);
    }

    #[test]
    fn walk_chain_follows_prev_links() {
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        // Two frames: fs[0] -> 0x2000 -> 0x1000 -> END
        mem.write_u32(0x0, 0x2000);
        mem.write_u32(0x2000 + MSVCRT_EF_PREV_OFFSET, 0x1000);
        mem.write_u32(0x2000 + MSVCRT_EF_HANDLER_OFFSET, 0xAAAA);
        mem.write_u32(0x1000 + MSVCRT_EF_PREV_OFFSET, FS_CHAIN_END);
        mem.write_u32(0x1000 + MSVCRT_EF_HANDLER_OFFSET, 0xBBBB);
        let frames = walk_chain(&mem, 32);
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].frame_addr, 0x2000);
        assert_eq!(frames[0].handler, 0xAAAA);
        assert_eq!(frames[1].frame_addr, 0x1000);
        assert_eq!(frames[1].handler, 0xBBBB);
    }

    #[test]
    fn walk_chain_detects_cycle() {
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        // Self-cycle: fs[0] -> 0x1000 -> 0x1000 -> ...
        mem.write_u32(0x0, 0x1000);
        mem.write_u32(0x1000 + MSVCRT_EF_PREV_OFFSET, 0x1000);
        let frames = walk_chain(&mem, 32);
        assert_eq!(
            frames.len(),
            1,
            "cycle detection should stop after first visit"
        );
    }

    #[test]
    fn walk_chain_respects_max_frames() {
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        // Build a 5-frame chain, ask for max_frames=3
        mem.write_u32(0x0, 0x5000);
        mem.write_u32(0x5000 + MSVCRT_EF_PREV_OFFSET, 0x4000);
        mem.write_u32(0x4000 + MSVCRT_EF_PREV_OFFSET, 0x3000);
        mem.write_u32(0x3000 + MSVCRT_EF_PREV_OFFSET, 0x2000);
        mem.write_u32(0x2000 + MSVCRT_EF_PREV_OFFSET, 0x1000);
        mem.write_u32(0x1000 + MSVCRT_EF_PREV_OFFSET, FS_CHAIN_END);
        let frames = walk_chain(&mem, 3);
        assert_eq!(frames.len(), 3);
    }

    #[test]
    fn read_scopetable_entry_rejects_null_ptr() {
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        assert!(read_scopetable_entry(&mem, 0, 0).is_none());
        assert!(read_scopetable_entry(&mem, 0x2000_0001, 0).is_none());
    }

    #[test]
    fn read_scopetable_entry_decodes_spiderman_layout() {
        // Plant Spider-Man's known scopetable at 0x3AEB00:
        //   [0] prev_try=-1, filter=0x002A540C, handler=0x002A542B
        let mem = crate::xbox::memory::guest_memory::GuestMemory::new()
            .expect("guest memory alloc for test");
        let st = 0x003A_EB00u32;
        mem.write_u32(st + SCOPETABLE_PREV_TRY_LEVEL, 0xFFFF_FFFF);
        mem.write_u32(st + SCOPETABLE_FILTER_OFFSET, 0x002A_540C);
        mem.write_u32(st + SCOPETABLE_HANDLER_OFFSET, 0x002A_542B);
        let e = read_scopetable_entry(&mem, st, 0).expect("should decode");
        assert_eq!(e.0, -1);
        assert_eq!(e.1, 0x002A_540C);
        assert_eq!(e.2, 0x002A_542B);
    }
}
