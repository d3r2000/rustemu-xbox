pub mod bink_profile;
pub mod decoder;
pub mod emitter;
pub mod jit;
pub mod oovpa;
pub mod oovpa_d3d;
pub mod oovpa_patterns;
pub mod runtime;
pub mod trampoline;
pub mod veh;
pub mod veh_dispatch;
pub mod veh_dpc;
pub mod veh_fixup;
pub mod veh_mmio;
// Note: oovpa_hle is now a submodule of oovpa (oovpa::oovpa_hle)
pub mod micro_interp;
pub mod nv2a;
pub mod nv2a_pb; // spliced from JIT branch 2026-04-20 — pushbuffer parser
pub mod seh_dispatch; // Guest-aware exception dispatcher (Wine-style) — skeleton, Stage 1 of 6
pub mod seh_runtime;
pub mod snapshot_probe;
pub mod transition; // Frida/Wine-style matched-transition discipline (Phase 3, log-only)
