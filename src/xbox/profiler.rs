use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub enum CpuPhase {
    GuestExec,
    KernelDispatch,
    OovpaHle,
    ApuNotify,
    DebugLog,
}

impl CpuPhase {
    fn name(self) -> &'static str {
        match self {
            CpuPhase::GuestExec => "guest_exec",
            CpuPhase::KernelDispatch => "kernel_dispatch",
            CpuPhase::OovpaHle => "oovpa_hle",
            CpuPhase::ApuNotify => "apu_notify",
            CpuPhase::DebugLog => "debug_log",
        }
    }

    fn counters(self) -> (&'static AtomicU64, &'static AtomicU64) {
        match self {
            CpuPhase::GuestExec => (&GUEST_EXEC_CALLS, &GUEST_EXEC_NS),
            CpuPhase::KernelDispatch => (&KERNEL_DISPATCH_CALLS, &KERNEL_DISPATCH_NS),
            CpuPhase::OovpaHle => (&OOVPA_HLE_CALLS, &OOVPA_HLE_NS),
            CpuPhase::ApuNotify => (&APU_NOTIFY_CALLS, &APU_NOTIFY_NS),
            CpuPhase::DebugLog => (&DEBUG_LOG_CALLS, &DEBUG_LOG_NS),
        }
    }
}

pub struct CpuPhaseGuard {
    phase: CpuPhase,
    start: Instant,
}

pub struct KernelOrdinalGuard {
    ordinal: u32,
    start: Instant,
}

pub struct HleNameGuard {
    name: &'static str,
    start: Instant,
}

impl Drop for CpuPhaseGuard {
    fn drop(&mut self) {
        finish(self.phase, Some(self.start));
    }
}

impl Drop for KernelOrdinalGuard {
    fn drop(&mut self) {
        record_kernel_ordinal(self.ordinal, self.start);
    }
}

impl Drop for HleNameGuard {
    fn drop(&mut self) {
        record_hle_name(self.name, self.start);
    }
}

struct SubTimer {
    calls: AtomicU64,
    nanos: AtomicU64,
}

impl SubTimer {
    fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            nanos: AtomicU64::new(0),
        }
    }
}

struct HleNameSlot {
    hash: AtomicU64,
    calls: AtomicU64,
    nanos: AtomicU64,
    name: OnceLock<&'static str>,
}

impl HleNameSlot {
    fn new() -> Self {
        Self {
            hash: AtomicU64::new(0),
            calls: AtomicU64::new(0),
            nanos: AtomicU64::new(0),
            name: OnceLock::new(),
        }
    }
}

struct HleOverflowEntry {
    name: &'static str,
    calls: u64,
    nanos: u64,
}

#[derive(Copy, Clone)]
struct KeDelayMeta {
    caller: u32,
    interval_ptr: u32,
    interval_100ns: i64,
    requested_us: i64,
    wait_mode: u32,
    alertable: bool,
}

struct KeDelaySlot {
    key: AtomicU64,
    calls: AtomicU64,
    nanos: AtomicU64,
    meta: OnceLock<KeDelayMeta>,
}

impl KeDelaySlot {
    fn new() -> Self {
        Self {
            key: AtomicU64::new(0),
            calls: AtomicU64::new(0),
            nanos: AtomicU64::new(0),
            meta: OnceLock::new(),
        }
    }
}

struct KeDelayOverflowEntry {
    meta: KeDelayMeta,
    calls: u64,
    nanos: u64,
}

static GUEST_EXEC_CALLS: AtomicU64 = AtomicU64::new(0);
static GUEST_EXEC_NS: AtomicU64 = AtomicU64::new(0);
static KERNEL_DISPATCH_CALLS: AtomicU64 = AtomicU64::new(0);
static KERNEL_DISPATCH_NS: AtomicU64 = AtomicU64::new(0);
static OOVPA_HLE_CALLS: AtomicU64 = AtomicU64::new(0);
static OOVPA_HLE_NS: AtomicU64 = AtomicU64::new(0);
static APU_NOTIFY_CALLS: AtomicU64 = AtomicU64::new(0);
static APU_NOTIFY_NS: AtomicU64 = AtomicU64::new(0);
static DEBUG_LOG_CALLS: AtomicU64 = AtomicU64::new(0);
static DEBUG_LOG_NS: AtomicU64 = AtomicU64::new(0);
static PROFILE_EVENTS: AtomicU64 = AtomicU64::new(0);
static NEXT_DUMP_MS: AtomicU64 = AtomicU64::new(5_000);
static DUMPING: AtomicBool = AtomicBool::new(false);
const KERNEL_MAX_ORDINAL: usize = 512;
const HLE_NAME_BUCKETS: usize = 2048;
const HLE_NAME_PROBE_LIMIT: usize = 16;
const KE_DELAY_BUCKETS: usize = 1024;
const KE_DELAY_PROBE_LIMIT: usize = 16;

fn truthy_env(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            let value = value.trim();
            value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
                || value.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

pub fn cpu_frame_prof_active() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| truthy_env("RUSTEMU_CPU_FRAME_PROF"))
}

fn start_cell() -> &'static Mutex<Instant> {
    static START: OnceLock<Mutex<Instant>> = OnceLock::new();
    START.get_or_init(|| Mutex::new(Instant::now()))
}

fn kernel_timers() -> &'static [SubTimer] {
    static TIMERS: OnceLock<Vec<SubTimer>> = OnceLock::new();
    TIMERS
        .get_or_init(|| (0..KERNEL_MAX_ORDINAL).map(|_| SubTimer::new()).collect())
        .as_slice()
}

fn hle_name_slots() -> &'static [HleNameSlot] {
    static SLOTS: OnceLock<Vec<HleNameSlot>> = OnceLock::new();
    SLOTS
        .get_or_init(|| (0..HLE_NAME_BUCKETS).map(|_| HleNameSlot::new()).collect())
        .as_slice()
}

fn hle_overflow() -> &'static Mutex<Vec<HleOverflowEntry>> {
    static OVERFLOW: OnceLock<Mutex<Vec<HleOverflowEntry>>> = OnceLock::new();
    OVERFLOW.get_or_init(|| Mutex::new(Vec::new()))
}

fn ke_delay_slots() -> &'static [KeDelaySlot] {
    static SLOTS: OnceLock<Vec<KeDelaySlot>> = OnceLock::new();
    SLOTS
        .get_or_init(|| (0..KE_DELAY_BUCKETS).map(|_| KeDelaySlot::new()).collect())
        .as_slice()
}

fn ke_delay_overflow() -> &'static Mutex<Vec<KeDelayOverflowEntry>> {
    static OVERFLOW: OnceLock<Mutex<Vec<KeDelayOverflowEntry>>> = OnceLock::new();
    OVERFLOW.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn reset_cpu_frame_profile() {
    if !cpu_frame_prof_active() {
        return;
    }

    for phase in ALL_PHASES {
        let (calls, nanos) = phase.counters();
        calls.store(0, Ordering::Relaxed);
        nanos.store(0, Ordering::Relaxed);
    }
    for timer in kernel_timers() {
        timer.calls.store(0, Ordering::Relaxed);
        timer.nanos.store(0, Ordering::Relaxed);
    }
    for slot in hle_name_slots() {
        slot.calls.store(0, Ordering::Relaxed);
        slot.nanos.store(0, Ordering::Relaxed);
    }
    if let Ok(mut overflow) = hle_overflow().lock() {
        overflow.clear();
    }
    for slot in ke_delay_slots() {
        slot.calls.store(0, Ordering::Relaxed);
        slot.nanos.store(0, Ordering::Relaxed);
    }
    if let Ok(mut overflow) = ke_delay_overflow().lock() {
        overflow.clear();
    }
    PROFILE_EVENTS.store(0, Ordering::Relaxed);
    NEXT_DUMP_MS.store(5_000, Ordering::Relaxed);
    if let Ok(mut start) = start_cell().lock() {
        *start = Instant::now();
    }
}

#[inline(always)]
pub fn start(phase: CpuPhase) -> Option<Instant> {
    if cpu_frame_prof_active() {
        let _ = phase;
        Some(Instant::now())
    } else {
        None
    }
}

#[inline(always)]
pub fn finish(phase: CpuPhase, start: Option<Instant>) {
    if let Some(start) = start {
        record_phase(phase, start);
    }
}

#[inline(always)]
pub fn guard(phase: CpuPhase) -> Option<CpuPhaseGuard> {
    if cpu_frame_prof_active() {
        Some(CpuPhaseGuard {
            phase,
            start: Instant::now(),
        })
    } else {
        None
    }
}

#[inline(always)]
pub fn kernel_ordinal_guard(ordinal: u32) -> Option<KernelOrdinalGuard> {
    if cpu_frame_prof_active() {
        Some(KernelOrdinalGuard {
            ordinal,
            start: Instant::now(),
        })
    } else {
        None
    }
}

#[inline(always)]
pub fn hle_name_guard(name: &'static str) -> Option<HleNameGuard> {
    if cpu_frame_prof_active() {
        Some(HleNameGuard {
            name,
            start: Instant::now(),
        })
    } else {
        None
    }
}

#[inline(always)]
fn record_phase(phase: CpuPhase, start: Instant) {
    let ns = start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
    let (calls, nanos) = phase.counters();
    calls.fetch_add(1, Ordering::Relaxed);
    nanos.fetch_add(ns, Ordering::Relaxed);

    let events = PROFILE_EVENTS.fetch_add(1, Ordering::Relaxed) + 1;
    if events & 0x3ff == 0 {
        maybe_dump_periodic();
    }
}

#[inline(always)]
fn record_kernel_ordinal(ordinal: u32, start: Instant) {
    let Some(timer) = kernel_timers().get(ordinal as usize) else {
        return;
    };
    let ns = start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
    timer.calls.fetch_add(1, Ordering::Relaxed);
    timer.nanos.fetch_add(ns, Ordering::Relaxed);
}

#[inline(always)]
fn record_hle_name(name: &'static str, start: Instant) {
    let ns = start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
    let hash = hle_name_hash(name);
    let slots = hle_name_slots();
    let mut index = (hash as usize) & (HLE_NAME_BUCKETS - 1);

    for _ in 0..HLE_NAME_PROBE_LIMIT {
        let slot = &slots[index];
        let seen = slot.hash.load(Ordering::Relaxed);
        if seen == hash
            || (seen == 0
                && slot
                    .hash
                    .compare_exchange(0, hash, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok())
        {
            let _ = slot.name.set(name);
            slot.calls.fetch_add(1, Ordering::Relaxed);
            slot.nanos.fetch_add(ns, Ordering::Relaxed);
            return;
        }
        index = (index + 1) & (HLE_NAME_BUCKETS - 1);
    }

    if let Ok(mut overflow) = hle_overflow().lock() {
        if let Some(entry) = overflow.iter_mut().find(|entry| entry.name == name) {
            entry.calls = entry.calls.saturating_add(1);
            entry.nanos = entry.nanos.saturating_add(ns);
        } else {
            overflow.push(HleOverflowEntry {
                name,
                calls: 1,
                nanos: ns,
            });
        }
    }
}

pub fn record_ke_delay(
    caller: u32,
    interval_ptr: u32,
    interval_100ns: i64,
    wait_mode: u32,
    alertable: bool,
    start: Instant,
) {
    if !cpu_frame_prof_active() {
        return;
    }

    let ns = start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
    let meta = KeDelayMeta {
        caller,
        interval_ptr,
        interval_100ns,
        requested_us: requested_delay_us(interval_100ns),
        wait_mode,
        alertable,
    };
    let key = ke_delay_key(meta);
    let slots = ke_delay_slots();
    let mut index = (key as usize) & (KE_DELAY_BUCKETS - 1);

    for _ in 0..KE_DELAY_PROBE_LIMIT {
        let slot = &slots[index];
        let seen = slot.key.load(Ordering::Relaxed);
        if seen == key
            || (seen == 0
                && slot
                    .key
                    .compare_exchange(0, key, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok())
        {
            let _ = slot.meta.set(meta);
            slot.calls.fetch_add(1, Ordering::Relaxed);
            slot.nanos.fetch_add(ns, Ordering::Relaxed);
            return;
        }
        index = (index + 1) & (KE_DELAY_BUCKETS - 1);
    }

    if let Ok(mut overflow) = ke_delay_overflow().lock() {
        if let Some(entry) = overflow.iter_mut().find(|entry| {
            entry.meta.caller == meta.caller
                && entry.meta.interval_ptr == meta.interval_ptr
                && entry.meta.interval_100ns == meta.interval_100ns
                && entry.meta.wait_mode == meta.wait_mode
                && entry.meta.alertable == meta.alertable
        }) {
            entry.calls = entry.calls.saturating_add(1);
            entry.nanos = entry.nanos.saturating_add(ns);
        } else {
            overflow.push(KeDelayOverflowEntry {
                meta,
                calls: 1,
                nanos: ns,
            });
        }
    }
}

fn requested_delay_us(interval_100ns: i64) -> i64 {
    if interval_100ns < 0 {
        interval_100ns.saturating_neg() / 10
    } else {
        0
    }
}

fn ke_delay_key(meta: KeDelayMeta) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in meta.caller.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    for byte in meta.interval_ptr.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    for byte in meta.interval_100ns.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    for byte in meta.wait_mode.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash ^= u64::from(meta.alertable);
    hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    hash.max(1)
}

#[inline(always)]
fn hle_name_hash(name: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &byte in name.as_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash.max(1)
}

fn elapsed_ms() -> u64 {
    let Ok(start) = start_cell().lock() else {
        return 0;
    };
    start.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

fn maybe_dump_periodic() {
    let elapsed = elapsed_ms();
    let next = NEXT_DUMP_MS.load(Ordering::Relaxed);
    if elapsed < next {
        return;
    }

    if NEXT_DUMP_MS
        .compare_exchange(
            next,
            next.saturating_add(5_000),
            Ordering::Relaxed,
            Ordering::Relaxed,
        )
        .is_ok()
    {
        dump_cpu_frame_profile("periodic");
    }
}

pub fn dump_cpu_frame_profile(reason: &str) {
    if !cpu_frame_prof_active() {
        return;
    }
    if DUMPING.swap(true, Ordering::Relaxed) {
        return;
    }

    let elapsed = elapsed_ms();
    crate::xbox::emulator::debug_log(&format!(
        "[CPU-FRAME-PROF] reason={} elapsed_ms={} events={}",
        reason,
        elapsed,
        PROFILE_EVENTS.load(Ordering::Relaxed)
    ));

    let elapsed_ns = elapsed.saturating_mul(1_000_000);
    for phase in ALL_PHASES {
        let (calls_counter, nanos_counter) = phase.counters();
        let calls = calls_counter.load(Ordering::Relaxed);
        let nanos = nanos_counter.load(Ordering::Relaxed);
        if calls == 0 && nanos == 0 {
            continue;
        }

        let total_ms = nanos as f64 / 1_000_000.0;
        let avg_us = if calls == 0 {
            0.0
        } else {
            nanos as f64 / calls as f64 / 1_000.0
        };
        let per_sec = if elapsed == 0 {
            0.0
        } else {
            calls as f64 * 1000.0 / elapsed as f64
        };
        let pct_elapsed = if elapsed_ns == 0 {
            0.0
        } else {
            nanos as f64 * 100.0 / elapsed_ns as f64
        };

        crate::xbox::emulator::debug_log(&format!(
            "[CPU-FRAME-PROF] phase={} calls={} total_ms={:.3} avg_us={:.3} calls_per_s={:.2} pct_elapsed={:.2}",
            phase.name(),
            calls,
            total_ms,
            avg_us,
            per_sec,
            pct_elapsed
        ));
    }

    dump_kernel_hitlist(elapsed);
    dump_ke_delay_hitlist(elapsed);
    dump_hle_name_hitlist(elapsed);

    DUMPING.store(false, Ordering::Relaxed);
}

fn dump_kernel_hitlist(elapsed: u64) {
    let mut rows: Vec<(usize, u64, u64)> = kernel_timers()
        .iter()
        .enumerate()
        .filter_map(|(ordinal, timer)| {
            let calls = timer.calls.load(Ordering::Relaxed);
            let nanos = timer.nanos.load(Ordering::Relaxed);
            if calls == 0 && nanos == 0 {
                None
            } else {
                Some((ordinal, calls, nanos))
            }
        })
        .collect();
    rows.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.1.cmp(&a.1)));

    for (rank, (ordinal, calls, nanos)) in rows.into_iter().take(10).enumerate() {
        dump_subphase_row(
            "kernel",
            rank + 1,
            &format!(
                "{}:{}",
                ordinal,
                crate::xbox::kernel::ordinals::name(ordinal as u32)
            ),
            calls,
            nanos,
            elapsed,
        );
    }
}

fn dump_hle_name_hitlist(elapsed: u64) {
    let mut rows: Vec<(&'static str, u64, u64)> = hle_name_slots()
        .iter()
        .filter_map(|slot| {
            let calls = slot.calls.load(Ordering::Relaxed);
            let nanos = slot.nanos.load(Ordering::Relaxed);
            if calls == 0 && nanos == 0 {
                return None;
            }
            let name = slot.name.get().copied().unwrap_or("<unknown>");
            Some((name, calls, nanos))
        })
        .collect();

    if let Ok(overflow) = hle_overflow().lock() {
        rows.extend(
            overflow
                .iter()
                .filter(|entry| entry.calls != 0 || entry.nanos != 0)
                .map(|entry| (entry.name, entry.calls, entry.nanos)),
        );
    }

    rows.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.1.cmp(&a.1)));

    for (rank, (name, calls, nanos)) in rows.into_iter().take(12).enumerate() {
        dump_subphase_row("hle", rank + 1, name, calls, nanos, elapsed);
    }
}

fn dump_ke_delay_hitlist(elapsed: u64) {
    let mut rows: Vec<(KeDelayMeta, u64, u64)> = ke_delay_slots()
        .iter()
        .filter_map(|slot| {
            let calls = slot.calls.load(Ordering::Relaxed);
            let nanos = slot.nanos.load(Ordering::Relaxed);
            if calls == 0 && nanos == 0 {
                return None;
            }
            slot.meta.get().copied().map(|meta| (meta, calls, nanos))
        })
        .collect();

    if let Ok(overflow) = ke_delay_overflow().lock() {
        rows.extend(
            overflow
                .iter()
                .filter(|entry| entry.calls != 0 || entry.nanos != 0)
                .map(|entry| (entry.meta, entry.calls, entry.nanos)),
        );
    }

    rows.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.1.cmp(&a.1)));

    for (rank, (meta, calls, nanos)) in rows.into_iter().take(12).enumerate() {
        let total_ms = nanos as f64 / 1_000_000.0;
        let avg_elapsed_us = if calls == 0 {
            0.0
        } else {
            nanos as f64 / calls as f64 / 1_000.0
        };
        let requested_total_ms = if meta.requested_us > 0 {
            meta.requested_us as f64 * calls as f64 / 1_000.0
        } else {
            0.0
        };
        let oversleep_ms = total_ms - requested_total_ms;
        let per_sec = if elapsed == 0 {
            0.0
        } else {
            calls as f64 * 1000.0 / elapsed as f64
        };
        let pct_elapsed = if elapsed == 0 {
            0.0
        } else {
            total_ms * 100.0 / elapsed as f64
        };

        crate::xbox::emulator::debug_log(&format!(
            "[CPU-FRAME-PROF] sub=ke_delay rank={} caller=0x{:08X} interval_ptr=0x{:08X} alertable={} wait_mode={} interval_100ns={} requested_us={} calls={} total_ms={:.3} requested_total_ms={:.3} oversleep_ms={:.3} avg_elapsed_us={:.3} calls_per_s={:.2} pct_elapsed={:.2}",
            rank + 1,
            meta.caller,
            meta.interval_ptr,
            meta.alertable,
            meta.wait_mode,
            meta.interval_100ns,
            meta.requested_us,
            calls,
            total_ms,
            requested_total_ms,
            oversleep_ms,
            avg_elapsed_us,
            per_sec,
            pct_elapsed
        ));
    }
}

fn dump_subphase_row(kind: &str, rank: usize, name: &str, calls: u64, nanos: u64, elapsed: u64) {
    let total_ms = nanos as f64 / 1_000_000.0;
    let avg_us = if calls == 0 {
        0.0
    } else {
        nanos as f64 / calls as f64 / 1_000.0
    };
    let per_sec = if elapsed == 0 {
        0.0
    } else {
        calls as f64 * 1000.0 / elapsed as f64
    };
    let pct_elapsed = if elapsed == 0 {
        0.0
    } else {
        total_ms * 100.0 / elapsed as f64
    };

    crate::xbox::emulator::debug_log(&format!(
        "[CPU-FRAME-PROF] sub={} rank={} name={} calls={} total_ms={:.3} avg_us={:.3} calls_per_s={:.2} pct_elapsed={:.2}",
        kind, rank, name, calls, total_ms, avg_us, per_sec, pct_elapsed
    ));
}

const ALL_PHASES: &[CpuPhase] = &[
    CpuPhase::GuestExec,
    CpuPhase::KernelDispatch,
    CpuPhase::OovpaHle,
    CpuPhase::ApuNotify,
    CpuPhase::DebugLog,
];
