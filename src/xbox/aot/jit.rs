//! Diagnostic JIT backend selector.
//!
//! This is intentionally not a whole-game JIT. The first supported mode is a
//! rescue-only diagnostic lane: AOT remains the primary executor, and this
//! module makes unresolved-target rescue compilation visible and selectable.

use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuBackend {
    Aot,
    JitDiagnostic,
}

impl CpuBackend {
    pub fn as_str(self) -> &'static str {
        match self {
            CpuBackend::Aot => "aot",
            CpuBackend::JitDiagnostic => "jit-diagnostic",
        }
    }

    pub fn rescue_jit_enabled(self) -> bool {
        matches!(self, CpuBackend::JitDiagnostic)
    }
}

static CPU_BACKEND: AtomicU8 = AtomicU8::new(CpuBackend::Aot as u8);
static RESCUE_CANDIDATE_LOGS: AtomicU32 = AtomicU32::new(0);
static RESCUE_RESULT_LOGS: AtomicU32 = AtomicU32::new(0);
static RET_MISS_LOGS: AtomicU32 = AtomicU32::new(0);
static RET_ZERO_LOGS: AtomicU32 = AtomicU32::new(0);

pub fn current() -> CpuBackend {
    match CPU_BACKEND.load(Ordering::Relaxed) {
        1 => CpuBackend::JitDiagnostic,
        _ => CpuBackend::Aot,
    }
}

pub fn configure(core_option: Option<&str>) -> CpuBackend {
    let env_value = std::env::var("RUSTEMU_CPU_BACKEND").ok();
    let (source, raw) = if let Some(value) = env_value.as_deref() {
        ("env", value)
    } else if let Some(value) = core_option {
        ("core-option", value)
    } else {
        ("default", "aot")
    };

    let backend = parse_backend(raw);
    CPU_BACKEND.store(backend as u8, Ordering::Relaxed);
    crate::xbox::emulator::debug_log(&format!(
        "[CPU-BACKEND] selected={} source={} raw='{}' mode={}",
        backend.as_str(),
        source,
        raw,
        if backend.rescue_jit_enabled() {
            "rescue-only diagnostic JIT"
        } else {
            "AOT"
        }
    ));
    backend
}

fn parse_backend(raw: &str) -> CpuBackend {
    let normalized = raw
        .trim()
        .to_ascii_lowercase()
        .replace('_', "-")
        .replace(' ', "-");
    match normalized.as_str() {
        "jit" | "jit-diagnostic" | "diagnostic-jit" | "rescue-jit" | "rescue-only-jit" => {
            CpuBackend::JitDiagnostic
        }
        _ => CpuBackend::Aot,
    }
}

pub fn note_unresolved_rescue_candidate(guest_addr: u32, esp: u32, eax: u32) {
    if !current().rescue_jit_enabled() {
        return;
    }
    let n = RESCUE_CANDIDATE_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[JIT-DIAG] unresolved rescue candidate target=0x{:08X} esp=0x{:08X} eax=0x{:08X} count={}",
            guest_addr,
            esp,
            eax,
            n + 1
        ));
    }
}

pub fn note_oovpa_rescue_candidate(pattern_name: &str, guest_addr: u32) {
    if !current().rescue_jit_enabled() {
        return;
    }
    crate::xbox::emulator::debug_log(&format!(
        "[JIT-DIAG] OOVPA rescue candidate pattern={} guest=0x{:08X}",
        pattern_name, guest_addr
    ));
}

pub fn note_rescue_result(guest_addr: u32, result: Option<u32>, decoded: usize, bytes: u32) {
    if !current().rescue_jit_enabled() {
        return;
    }
    let n = RESCUE_RESULT_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[JIT-DIAG] rescue result target=0x{:08X} host={} decoded={} bytes={} count={}",
            guest_addr,
            result
                .map(|host| format!("+0x{:X}", host))
                .unwrap_or_else(|| "none".to_string()),
            decoded,
            bytes,
            n + 1
        ));
    }
}

pub fn note_ret_miss(ret_site: u32, guest_target: u32, esp: u32, hash_lookups: u64) {
    if !current().rescue_jit_enabled() {
        return;
    }
    let n = RET_MISS_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[JIT-DIAG] RetFastMiss ret_site=0x{:08X} guest_target=0x{:08X} esp=0x{:08X} full_hash_pending=true hash_lookups={} count={}",
            ret_site,
            guest_target,
            esp,
            hash_lookups,
            n + 1
        ));
    }
}

pub fn note_ret_to_zero(dispatch_count: u64, kernel_calls: u64, esp: u32, eax: u32) {
    if !current().rescue_jit_enabled() {
        return;
    }
    let n = RET_ZERO_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[JIT-DIAG] RET_TO_ZERO dispatch={} kernel_calls={} esp=0x{:08X} eax=0x{:08X} count={}",
            dispatch_count,
            kernel_calls,
            esp,
            eax,
            n + 1
        ));
    }
}
