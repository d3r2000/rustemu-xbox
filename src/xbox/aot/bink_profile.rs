//! BINK profiler — per-trap-type counters scoped to Spider-Man's BINK guest PC range.
//!
//! Codex 2026-04-26 plan, profiler step. Active alongside the
//! [BINK-TIMING] instrumentation that fires when RUSTEMU_NATIVE_BINK=1
//! plants Bink* hooks in TAP mode. The TAP timestamps tell us the wrapper
//! symptom (BinkDoFrame ~ 100-300 ms); these counters tell us the cause:
//!
//!   high interp        → AOT coverage gap inside BINK section, fix decoder
//!   high ret/ind       → dispatch overhead, JIT_HOT inline could help
//!   low traps high dt  → emitted-code quality, AOT emitter or selective HLE
//!   high av/fixup      → address-rewrite/fixup issue (R15 sign-ext storm)
//!
//! Counters are global atomics, scoped via the `is_in_bink()` predicate that
//! every increment site calls first. Total cost per non-BINK trap: one branch.
//! Total cost per BINK trap: one branch + one relaxed atomic add. Cheap.
//!
//! Spider-Man XDK 4134 BINK section ranges (from baseline log
//! spiderman_run_20260426_144924.log lines 27-37):
//!   BINK     0x0035C1C0..0x00372880  (91 840 bytes, main code)
//!   BINK32   0x00372880..0x00373B00
//!   BINK32A  0x00373B00..0x00375080
//!   BINK16   0x00375080..0x003763A0
//!   BINK4444 0x003763A0..0x00377940
//!   BINK5551 0x00377940..0x00378A80
//!   BINK16MX 0x00378A80..0x00378BC0
//!   BINK16X2 0x00378BC0..0x00379120
//!   BINK16M  0x00379120..0x00379320
//!   BINK32MX 0x00379320..0x003794E0
//!   BINK32X2 0x003794E0..0x00379A80
//!   BINK32M  0x00379A80..0x00379BCC
//!
//! All 12 code sections are contiguous; a single range check covers them.
//! BINKDATA at 0x007E8A20 is a separate data section, not code — excluded.

use std::sync::atomic::{AtomicU64, Ordering};

const BINK_LO: u32 = 0x0035_C1C0;
const BINK_HI: u32 = 0x0037_A000;

#[inline(always)]
pub fn is_in_bink(guest_pc: u32) -> bool {
    guest_pc >= BINK_LO && guest_pc < BINK_HI
}

pub static BINK_INTERP_ENTRIES: AtomicU64 = AtomicU64::new(0);
pub static BINK_RET_TRAPS: AtomicU64 = AtomicU64::new(0);
pub static BINK_IND_TRAPS: AtomicU64 = AtomicU64::new(0);
pub static BINK_SYS_TRAPS: AtomicU64 = AtomicU64::new(0);
pub static BINK_AV_FIXUPS: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
pub fn note_interp_entry(eip: u32) {
    if is_in_bink(eip) {
        BINK_INTERP_ENTRIES.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub fn note_ret_trap(guest_pc: u32) {
    if is_in_bink(guest_pc) {
        BINK_RET_TRAPS.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub fn note_ind_trap(guest_pc: u32) {
    if is_in_bink(guest_pc) {
        BINK_IND_TRAPS.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub fn note_sys_trap(guest_pc: u32) {
    if is_in_bink(guest_pc) {
        BINK_SYS_TRAPS.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub fn note_av_fixup(guest_pc: u32) {
    if is_in_bink(guest_pc) {
        BINK_AV_FIXUPS.fetch_add(1, Ordering::Relaxed);
    }
}

/// Snapshot all 5 counters in one shot. Order: [interp, ret, ind, sys, av].
/// Each counter is atomic; cross-counter consistency is best-effort (the next
/// trap may race in between). Acceptable for diagnostic deltas.
#[inline]
pub fn snapshot() -> [u64; 5] {
    [
        BINK_INTERP_ENTRIES.load(Ordering::Relaxed),
        BINK_RET_TRAPS.load(Ordering::Relaxed),
        BINK_IND_TRAPS.load(Ordering::Relaxed),
        BINK_SYS_TRAPS.load(Ordering::Relaxed),
        BINK_AV_FIXUPS.load(Ordering::Relaxed),
    ]
}
