#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::Fxc::{
    D3DCompile, D3DCOMPILE_ENABLE_STRICTNESS, D3DCOMPILE_OPTIMIZATION_LEVEL1,
};
/// D3D11 hardware rendering backend — port of D3D11Backend.cpp.
/// Offscreen RT → staging texture → CPU readback → display_buffer.
/// No swap chain — RetroArch owns the window.

#[cfg(windows)]
use windows::{
    core::Interface, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
};

use super::render_state::RenderStateTable;
use super::shaders;
use super::{GpuBackend, NV2AVertex, VertexSkinPayload};
#[cfg(windows)]
use std::collections::{HashMap, VecDeque};
#[cfg(windows)]
use std::sync::{Mutex, OnceLock};

mod command_processor;
mod graphics_system;
mod pipeline;
mod primitive_processor;
mod texture_cache;

use command_processor::{draw_index_in_ranges, parse_draw_ranges_env, parse_u32_list_env};
use graphics_system::{
    maybe_record_present_frame, sample_pixel_stats, sampled_argb_stats, write_d3d11_readback_bmp,
    D3D11RenderTargetSurface, PendingRenderTarget, PresentReadbackCache,
};
use primitive_processor::{
    expand_triangle_fan, expand_triangle_fan_skin, expand_xbox_quad_list,
    expand_xbox_quad_list_skin, map_prim_type, D3D11DrawVertex, RecentDrawSummary,
};

const D3D11_DYNAMIC_VERTEX_CAP: usize = 16_384;
pub const D3D11_HLE_RT_KEY: u32 = 0xFFFF_F110;
pub const D3D11_HLE_RT_SIZE: u32 = 0xFFFF_F111;
pub const D3D11_HLE_RT_FORMAT: u32 = 0xFFFF_F112;
pub const D3D11_HLE_RT_DATA: u32 = 0xFFFF_F113;
pub const D3D11_HLE_RT_PITCH: u32 = 0xFFFF_F114;
pub const D3D11_HLE_RT_COMMIT: u32 = 0xFFFF_F115;
const FFP_WORLD_INDEX: usize = 0;
const FFP_VIEW_INDEX: usize = 1;
const FFP_PROJECTION_INDEX: usize = 2;
const FFP_TRANSFORM_COUNT: usize = 3;
const FFP_IDENTITY_MATRIX: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

#[cfg(windows)]
#[derive(Clone)]
struct RecentVsConstantUpload {
    seq: u64,
    reg: usize,
    rows: usize,
    preview: Vec<[f32; 4]>,
}

#[cfg(windows)]
static RECENT_VS_CONSTANT_UPLOADS: OnceLock<Mutex<VecDeque<RecentVsConstantUpload>>> =
    OnceLock::new();
#[cfg(windows)]
static VS_CONSTANT_UPLOAD_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static SPIDEY_PETER_ONLY_SEEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(windows)]
static D3D11_PROF_DRAW_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_DRAW_TOTAL_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_DRAW_MAP_US: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_DRAW_GPU_US: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_DRAW_FLUSH_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_DRAW_FLUSH_CALLS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_READBACK_CALLS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_READBACK_TOTAL_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_READBACK_COPY_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_READBACK_MAP_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_READBACK_CPU_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_FRAME_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[cfg(windows)]
static D3D11_PROF_LAST_FRAME_US: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

#[cfg(windows)]
fn d3d11_frame_prof_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_FRAME_PROF")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn d3d11_flush_each_draw_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_FLUSH_EACH_DRAW")
            .map(|value| {
                let value = value.trim();
                !(value == "0"
                    || value.eq_ignore_ascii_case("false")
                    || value.eq_ignore_ascii_case("no")
                    || value.eq_ignore_ascii_case("off"))
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn d3d11_preflight_validate_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_PREFLIGHT_VALIDATE")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn d3d11_prof_now_us() -> u64 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    START
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_micros()
        .min(u128::from(u64::MAX)) as u64
}

#[cfg(windows)]
fn d3d11_prof_add_us(counter: &std::sync::atomic::AtomicU64, start: std::time::Instant) {
    counter.fetch_add(
        start.elapsed().as_micros().min(u128::from(u64::MAX)) as u64,
        std::sync::atomic::Ordering::Relaxed,
    );
}

#[cfg(windows)]
struct D3D11PerfGuard {
    start: std::time::Instant,
    micros: &'static std::sync::atomic::AtomicU64,
}

#[cfg(windows)]
impl D3D11PerfGuard {
    fn new(
        enabled: bool,
        calls: &'static std::sync::atomic::AtomicU64,
        micros: &'static std::sync::atomic::AtomicU64,
    ) -> Option<Self> {
        if enabled {
            calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(Self {
                start: std::time::Instant::now(),
                micros,
            })
        } else {
            None
        }
    }
}

#[cfg(windows)]
impl Drop for D3D11PerfGuard {
    fn drop(&mut self) {
        d3d11_prof_add_us(self.micros, self.start);
    }
}

#[cfg(windows)]
fn d3d11_log_frame_prof(source: &str, active_rt_key: u32, width: usize, height: usize) {
    if !d3d11_frame_prof_enabled() {
        return;
    }
    let n = D3D11_PROF_FRAME_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let now = d3d11_prof_now_us();
    let prev = D3D11_PROF_LAST_FRAME_US.swap(now, std::sync::atomic::Ordering::Relaxed);
    let dt_us = if prev == 0 {
        0
    } else {
        now.saturating_sub(prev)
    };
    let fps = if dt_us > 0 {
        1_000_000.0 / dt_us as f64
    } else {
        0.0
    };
    let take = |counter: &std::sync::atomic::AtomicU64| {
        counter.swap(0, std::sync::atomic::Ordering::Relaxed)
    };
    let draw_calls = take(&D3D11_PROF_DRAW_CALLS);
    let draw_total_us = take(&D3D11_PROF_DRAW_TOTAL_US);
    let draw_map_us = take(&D3D11_PROF_DRAW_MAP_US);
    let draw_gpu_us = take(&D3D11_PROF_DRAW_GPU_US);
    let draw_flush_us = take(&D3D11_PROF_DRAW_FLUSH_US);
    let draw_flush_calls = take(&D3D11_PROF_DRAW_FLUSH_CALLS);
    let read_calls = take(&D3D11_PROF_READBACK_CALLS);
    let read_total_us = take(&D3D11_PROF_READBACK_TOTAL_US);
    let read_copy_us = take(&D3D11_PROF_READBACK_COPY_US);
    let read_map_us = take(&D3D11_PROF_READBACK_MAP_US);
    let read_cpu_us = take(&D3D11_PROF_READBACK_CPU_US);
    crate::xbox::emulator::debug_log(&format!(
        "[D3D11-FRAME-PROF] #{} dt_us={} fps={:.2} source={} rt=0x{:08X} size={}x{} draw_calls={} draw_total_us={} draw_map_us={} draw_gpu_us={} draw_flush_calls={} draw_flush_us={} read_calls={} read_total_us={} read_copy_us={} read_map_us={} read_cpu_us={}",
        n,
        dt_us,
        fps,
        source,
        active_rt_key,
        width,
        height,
        draw_calls,
        draw_total_us,
        draw_map_us,
        draw_gpu_us,
        draw_flush_calls,
        draw_flush_us,
        read_calls,
        read_total_us,
        read_copy_us,
        read_map_us,
        read_cpu_us
    ));
}

#[cfg(windows)]
#[derive(Clone, Copy)]
struct PeterCandidateOverlayPoint {
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
    color: u32,
}

#[cfg(windows)]
fn recent_vs_constant_uploads() -> &'static Mutex<VecDeque<RecentVsConstantUpload>> {
    RECENT_VS_CONSTANT_UPLOADS.get_or_init(|| Mutex::new(VecDeque::with_capacity(128)))
}

#[cfg(windows)]
const FFP_VS_HLSL: &str = r#"
cbuffer RustemuFfpTransforms : register(b1) {
    row_major float4x4 ffp_world;
    row_major float4x4 ffp_view;
    row_major float4x4 ffp_projection;
};

struct VS_IN {
    float4 pos : POSITION;
    uint color : COLOR;
    float2 uv : TEXCOORD;
};

struct VS_OUT {
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
};

float4 unpack_argb(uint color) {
    return float4(
        float((color >> 16) & 0xFFu) / 255.0,
        float((color >> 8) & 0xFFu) / 255.0,
        float(color & 0xFFu) / 255.0,
        float((color >> 24) & 0xFFu) / 255.0
    );
}

VS_OUT main(VS_IN input) {
    VS_OUT output;
    float4 p = float4(input.pos.xyz, 1.0);
    p = mul(p, ffp_world);
    p = mul(p, ffp_view);
    output.pos = mul(p, ffp_projection);
    output.color = unpack_argb(input.color);
    output.uv = input.uv;
    return output;
}
"#;

// Derived from Cxbx-Reloaded (GPL-2.0), snapshot 585c49a:
// src/core/hle/D3D8/XbD3D8Types.h
// Rustemu keeps Xbox guest constants in c0..c191 and mirrors Cxbx-R's
// host-only extension registers for translated vertex shader helpers.
pub(crate) const X_D3DVS_CONSTREG_COUNT: usize = 192;
pub(crate) const CXBX_D3DVS_CONSTREG_VREGDEFAULTS_BASE: usize = X_D3DVS_CONSTREG_COUNT;
pub(crate) const CXBX_D3DVS_CONSTREG_VREGDEFAULTS_SIZE: usize = 16;
pub(crate) const CXBX_D3DVS_CONSTREG_VREGDEFAULTS_FLAG_BASE: usize =
    CXBX_D3DVS_CONSTREG_VREGDEFAULTS_BASE + CXBX_D3DVS_CONSTREG_VREGDEFAULTS_SIZE;
pub(crate) const CXBX_D3DVS_CONSTREG_VREGDEFAULTS_FLAG_SIZE: usize = 4;
pub(crate) const CXBX_D3DVS_SCREENSPACE_SCALE_BASE: usize =
    CXBX_D3DVS_CONSTREG_VREGDEFAULTS_FLAG_BASE + CXBX_D3DVS_CONSTREG_VREGDEFAULTS_FLAG_SIZE;
pub(crate) const CXBX_D3DVS_SCREENSPACE_OFFSET_BASE: usize = CXBX_D3DVS_SCREENSPACE_SCALE_BASE + 1;
pub(crate) const CXBX_D3DVS_TEXTURES_SCALE_BASE: usize = CXBX_D3DVS_SCREENSPACE_OFFSET_BASE + 1;
pub(crate) const CXBX_D3DVS_TEXTURES_SCALE_SIZE: usize = 4;
pub(crate) const CXBX_D3DVS_CONSTREG_FOGINFO: usize =
    CXBX_D3DVS_TEXTURES_SCALE_BASE + CXBX_D3DVS_TEXTURES_SCALE_SIZE;
pub(crate) const D3D11_VS_CONSTANT_COUNT: usize = CXBX_D3DVS_CONSTREG_FOGINFO + 1;
pub(crate) const D3D11_VS_SURFACE_SIZE_REG: usize = D3D11_VS_CONSTANT_COUNT;
pub(crate) const D3D11_VS_CLIP_RANGE_REG: usize = D3D11_VS_CONSTANT_COUNT + 1;
pub(crate) const D3D11_VS_CBUFFER_COUNT: usize = D3D11_VS_CONSTANT_COUNT + 2;

const RECENT_DRAW_COUNT: usize = 128;
#[cfg(windows)]
static SPIDEY_FIRST_CIRCLE_DRAW: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_CHIS22_DRAW_COUNT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_LAST_CHIS22_DRAW: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[cfg(windows)]
fn doom_d3d11_rt_readback_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        [
            "RUSTEMU_DOOM_D3D11_RT_READBACK",
            "RUSTEMU_D3D11_RT_READBACK",
        ]
        .iter()
        .any(|key| {
            std::env::var(key)
                .map(|value| {
                    let value = value.trim();
                    value == "1"
                        || value.eq_ignore_ascii_case("true")
                        || value.eq_ignore_ascii_case("yes")
                        || value.eq_ignore_ascii_case("on")
                })
                .unwrap_or(false)
        })
    })
}

#[cfg(windows)]
fn d3d11_honor_guest_viewport() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_HONOR_GUEST_VIEWPORT")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn d3d11_rt_local_viewport() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_RT_LOCAL_VIEWPORT")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn d3d11_debug_layer_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_D3D11_DEBUG_LAYER")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
pub(super) fn spidey_rt_trace_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_RT_TRACE")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn spidey_force_100d_solid_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_FORCE_100D_SOLID")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn spidey_force_peter_diffuse_white_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_FORCE_PETER_DIFFUSE_WHITE")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn peter_probe_json_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("RUSTEMU_PETER_PROBE_JSON").is_ok())
}

#[cfg(windows)]
fn peter_probe_json_limit() -> u32 {
    static LIMIT: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *LIMIT.get_or_init(|| {
        std::env::var("RUSTEMU_PETER_PROBE_JSON_LIMIT")
            .ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(8)
    })
}

#[cfg(windows)]
fn spidey_peter_outlier_probe_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_PETER_OUTLIER_PROBE")
            .or_else(|_| std::env::var("RUSTEMU_PETER_OUTLIER_PROBE"))
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn spidey_peter_outlier_draw_filter() -> Option<u32> {
    static DRAW: std::sync::OnceLock<Option<u32>> = std::sync::OnceLock::new();
    *DRAW.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_PETER_OUTLIER_DRAW")
            .or_else(|_| std::env::var("RUSTEMU_PETER_OUTLIER_DRAW"))
            .ok()
            .and_then(|value| value.trim().parse::<u32>().ok())
    })
}

#[cfg(windows)]
pub(super) fn spidey_peter_texel_probe_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_PETER_TEXEL_PROBE")
            .or_else(|_| std::env::var("RUSTEMU_PETER_TEXEL_PROBE"))
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or_else(|_| peter_probe_json_enabled())
    })
}

#[cfg(windows)]
fn peter_candidate_overlay_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_PETER_CANDIDATE_OVERLAY")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn peter_candidate_overlay_bases() -> Vec<i32> {
    static BASES: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
    BASES
        .get_or_init(|| {
            std::env::var("RUSTEMU_SPIDEY_PETER_CANDIDATE_BASES")
                .ok()
                .map(|raw| {
                    raw.split([',', ';', ' '])
                        .filter_map(|part| part.trim().parse::<i32>().ok())
                        .filter(|base| (-96..=95).contains(base))
                        .collect::<Vec<_>>()
                })
                .filter(|bases| !bases.is_empty())
                .unwrap_or_else(|| vec![-88, -82, -79, -85, -91, -92])
        })
        .clone()
}

#[cfg(windows)]
fn peter_candidate_overlay_color(slot: usize) -> u32 {
    const COLORS: [u32; 12] = [
        0xFFFF_2020,
        0xFF20_FF20,
        0xFF30_90FF,
        0xFFFF_E020,
        0xFFFF_40FF,
        0xFF20_FFFF,
        0xFFFF_9020,
        0xFF90_FF20,
        0xFF90_20FF,
        0xFFFF_FFFF,
        0xFF80_8080,
        0xFFFF_80C0,
    ];
    COLORS[slot % COLORS.len()]
}

#[cfg(windows)]
fn spidey_skin_raw4_probe_enabled() -> bool {
    std::env::var("RUSTEMU_SPIDEY_XGRPH_SKIN_RAW4_CANARY")
        .or_else(|_| std::env::var("RUSTEMU_SPIDEY_SKIN_RAW4_CANARY"))
        .or_else(|_| std::env::var("RUSTEMU_SPIDEY_PBYTE4_RAW"))
        .map(|value| {
            let value = value.trim();
            value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
                || value.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

#[cfg(windows)]
fn spidey_skin_a0_remap_probe_enabled() -> bool {
    std::env::var("RUSTEMU_SPIDEY_SKIN_A0_REMAP")
        .map(|value| {
            let value = value.trim();
            value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
                || value.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

#[cfg(windows)]
fn spidey_cxbxr_skin_c1_canary_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_CXBXR_SKIN_C1_CANARY")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn spidey_peter_only_render_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_PETER_ONLY_RENDER")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
fn spidey_peter_only_shader(handle: u32) -> bool {
    static HANDLES: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
    let handles = HANDLES.get_or_init(|| {
        let mut parsed = parse_u32_list_env("RUSTEMU_SPIDEY_PETER_ONLY_VS");
        if parsed.is_empty() {
            parsed.push(0x0000_100D);
        }
        parsed
    });
    handles.contains(&handle)
}

#[cfg(windows)]
pub(super) fn spidey_rt_diag_key(key: u32) -> bool {
    let key = key & !0x3;
    (0x03C0_0150..=0x03C0_0330).contains(&key) || key == 0x0123_1000
}

#[cfg(windows)]
fn spidey_describe_guest_vs(raw: u32) -> String {
    if raw == 0 {
        return "none".to_string();
    }
    if raw & 1 == 0 {
        return format!("shader-handle");
    }

    let pos = raw & super::fvf_decode::fvf::POSITION_MASK;
    let pos_name = match pos {
        super::fvf_decode::fvf::XYZ => "XYZ",
        super::fvf_decode::fvf::XYZRHW => "XYZRHW",
        super::fvf_decode::fvf::XYZB1 => "XYZB1",
        super::fvf_decode::fvf::XYZB2 => "XYZB2",
        super::fvf_decode::fvf::XYZB3 => "XYZB3",
        super::fvf_decode::fvf::XYZB4 => "XYZB4",
        _ => "unknown-pos",
    };
    let tex_count =
        (raw & super::fvf_decode::fvf::TEXCOUNT_MASK) >> super::fvf_decode::fvf::TEXCOUNT_SHIFT;
    match super::fvf_decode::decode_fvf(raw) {
        Some((_, stride)) => format!("fvf:{} tex{} stride{}", pos_name, tex_count, stride),
        None => format!("fvf:{} tex{} invalid", pos_name, tex_count),
    }
}

#[cfg(windows)]
fn spidey_format_vertex_sample(verts: &[NV2AVertex]) -> String {
    verts
        .iter()
        .take(4)
        .map(|v| {
            format!(
                "({:.3},{:.3},{:.3},w={:.3},c=0x{:08X},uv={:.3},{:.3})",
                v.x, v.y, v.z, v.w, v.color, v.u, v.v
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(windows)]
fn argb_fnv1a64(pixels: &[u32]) -> u64 {
    let mut hash = 0xCBF2_9CE4_8422_2325u64;
    for px in pixels {
        for byte in px.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x1000_0000_01B3);
        }
    }
    hash
}

pub struct D3D11Backend {
    width: i32,
    height: i32,
    #[cfg(windows)]
    device: Option<ID3D11Device>,
    #[cfg(windows)]
    ctx: Option<ID3D11DeviceContext>,
    #[cfg(windows)]
    rt_tex: Option<ID3D11Texture2D>,
    #[cfg(windows)]
    rtv: Option<ID3D11RenderTargetView>,
    #[cfg(windows)]
    ds_tex: Option<ID3D11Texture2D>,
    #[cfg(windows)]
    dsv: Option<ID3D11DepthStencilView>,
    #[cfg(windows)]
    staging_tex: Option<ID3D11Texture2D>,
    #[cfg(windows)]
    vs: Option<ID3D11VertexShader>,
    #[cfg(windows)]
    ffp_vs: Option<ID3D11VertexShader>,
    #[cfg(windows)]
    ps: Option<ID3D11PixelShader>,
    #[cfg(windows)]
    ps_textured: Option<ID3D11PixelShader>,
    #[cfg(windows)]
    ps_spidey_solid: Option<ID3D11PixelShader>,
    #[cfg(windows)]
    nv2a_psh_cache: HashMap<u64, ID3D11PixelShader>,
    #[cfg(windows)]
    layout: Option<ID3D11InputLayout>,
    #[cfg(windows)]
    vb: Option<ID3D11Buffer>,
    #[cfg(windows)]
    vs_cbuffer: Option<ID3D11Buffer>,
    #[cfg(windows)]
    ffp_cbuffer: Option<ID3D11Buffer>,
    #[cfg(windows)]
    ps_cbuffer: Option<ID3D11Buffer>,
    #[cfg(windows)]
    nv2a_vs_cache: HashMap<u32, ID3D11VertexShader>,
    #[cfg(windows)]
    nv2a_layout_cache: HashMap<u32, ID3D11InputLayout>,
    #[cfg(windows)]
    active_nv2a_vs: Option<u32>,
    #[cfg(windows)]
    rs: Option<ID3D11RasterizerState>,
    #[cfg(windows)]
    dss: Option<ID3D11DepthStencilState>,
    #[cfg(windows)]
    force_no_depth_dss: Option<ID3D11DepthStencilState>,
    #[cfg(windows)]
    force_opaque_bs: Option<ID3D11BlendState>,
    #[cfg(windows)]
    bs: Option<ID3D11BlendState>,
    #[cfg(windows)]
    info_queue: Option<ID3D11InfoQueue>,
    #[cfg(windows)]
    rt_cache: HashMap<u32, D3D11RenderTargetSurface>,
    #[cfg(windows)]
    pending_rt: PendingRenderTarget,
    #[cfg(windows)]
    active_rt_key: u32,
    #[cfg(windows)]
    backbuffer_rt_key: u32,
    #[cfg(windows)]
    backbuffer_rt_info: PendingRenderTarget,
    black_clear_without_draw_pending: bool,
    draws_since_clear: u32,
    readback_buf: Vec<u32>,
    readback_x: u32,
    readback_y: u32,
    readback_width: u32,
    readback_height: u32,
    present_cache: PresentReadbackCache,
    has_active_texture: bool,
    texture_v_flip: bool,
    active_tex_width: u32,
    active_tex_height: u32,
    active_tex_handle: u32,
    active_tex_format_code: u32,
    active_tex_source: &'static str,
    active_tex_byte_len: usize,
    active_tex_srv_bound: bool,
    active_tex_guest_raw: u32,
    active_tex_guest_norm: u32,
    active_tex_data: u32,
    active_tex_guest_format_code: u32,
    active_tex_swizzled_blocks: bool,
    active_tex_alpha_min: u8,
    active_tex_alpha_max: u8,
    active_tex_alpha_nonzero_sample: usize,
    active_tex_rgb_nonzero_sample: usize,
    active_tex_sample_count: usize,
    active_tex_probe_pixels: Vec<u32>,
    active_tex_probe_width: u32,
    active_tex_probe_height: u32,
    next_tex_handle: u32,
    texture_stage_states: [[u32; 32]; 4],
    ps_render_states: [u32; 137],
    ps_constants: [[f32; 4]; 32],
    ps_constant_written_mask: u32,
    ps_constants_dirty: bool,
    render_state: RenderStateTable,
    vs_constants: [[f32; 4]; D3D11_VS_CONSTANT_COUNT],
    vs_constants_dirty: bool,
    ffp_transforms: [[f32; 16]; FFP_TRANSFORM_COUNT],
    ffp_transform_mask: u8,
    recent_draws: [RecentDrawSummary; RECENT_DRAW_COUNT],
    recent_draw_cursor: usize,
    recent_draw_seen: usize,
    peter_candidate_overlay_points: Vec<PeterCandidateOverlayPoint>,
}

impl D3D11Backend {
    #[cfg(windows)]
    fn log_adapter_identity(device: &ID3D11Device) {
        let dxgi_device = match device.cast::<IDXGIDevice>() {
            Ok(device) => device,
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-ADAPTER] IDXGIDevice cast FAILED: {}",
                    e
                ));
                return;
            }
        };

        let adapter = match unsafe { dxgi_device.GetAdapter() } {
            Ok(adapter) => adapter,
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-ADAPTER] GetAdapter FAILED: {}",
                    e
                ));
                return;
            }
        };

        let desc = match unsafe { adapter.GetDesc() } {
            Ok(desc) => desc,
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!("[D3D11-ADAPTER] GetDesc FAILED: {}", e));
                return;
            }
        };

        let name_len = desc
            .Description
            .iter()
            .position(|&ch| ch == 0)
            .unwrap_or(desc.Description.len());
        let name = String::from_utf16_lossy(&desc.Description[..name_len]);
        let vendor = match desc.VendorId {
            0x10DE => "NVIDIA",
            0x8086 => "Intel",
            0x1002 | 0x1022 => "AMD",
            0x1414 => "Microsoft",
            _ => "Unknown",
        };

        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-ADAPTER] {} vendor={} vendor_id=0x{:04X} device_id=0x{:04X} subsys=0x{:08X} revision=0x{:X} dedicated_vram_mb={} dedicated_sys_mb={} shared_sys_mb={} luid=0x{:08X}{:08X}",
            name,
            vendor,
            desc.VendorId,
            desc.DeviceId,
            desc.SubSysId,
            desc.Revision,
            desc.DedicatedVideoMemory / (1024 * 1024),
            desc.DedicatedSystemMemory / (1024 * 1024),
            desc.SharedSystemMemory / (1024 * 1024),
            desc.AdapterLuid.HighPart as u32,
            desc.AdapterLuid.LowPart
        ));
    }

    pub fn new() -> Self {
        let mut vs_constants = [[0.0; 4]; D3D11_VS_CONSTANT_COUNT];
        for reg in CXBX_D3DVS_TEXTURES_SCALE_BASE
            ..CXBX_D3DVS_TEXTURES_SCALE_BASE + CXBX_D3DVS_TEXTURES_SCALE_SIZE
        {
            vs_constants[reg] = [1.0, 1.0, 1.0, 1.0];
        }
        Self {
            width: 0,
            height: 0,
            #[cfg(windows)]
            device: None,
            #[cfg(windows)]
            ctx: None,
            #[cfg(windows)]
            rt_tex: None,
            #[cfg(windows)]
            rtv: None,
            #[cfg(windows)]
            ds_tex: None,
            #[cfg(windows)]
            dsv: None,
            #[cfg(windows)]
            staging_tex: None,
            #[cfg(windows)]
            vs: None,
            #[cfg(windows)]
            ffp_vs: None,
            #[cfg(windows)]
            ps: None,
            #[cfg(windows)]
            ps_textured: None,
            #[cfg(windows)]
            ps_spidey_solid: None,
            #[cfg(windows)]
            nv2a_psh_cache: HashMap::new(),
            #[cfg(windows)]
            layout: None,
            #[cfg(windows)]
            vb: None,
            #[cfg(windows)]
            vs_cbuffer: None,
            #[cfg(windows)]
            ffp_cbuffer: None,
            #[cfg(windows)]
            ps_cbuffer: None,
            #[cfg(windows)]
            nv2a_vs_cache: HashMap::new(),
            #[cfg(windows)]
            nv2a_layout_cache: HashMap::new(),
            #[cfg(windows)]
            active_nv2a_vs: None,
            #[cfg(windows)]
            rs: None,
            #[cfg(windows)]
            dss: None,
            #[cfg(windows)]
            force_no_depth_dss: None,
            #[cfg(windows)]
            force_opaque_bs: None,
            #[cfg(windows)]
            bs: None,
            #[cfg(windows)]
            info_queue: None,
            #[cfg(windows)]
            rt_cache: HashMap::new(),
            #[cfg(windows)]
            pending_rt: PendingRenderTarget::default(),
            #[cfg(windows)]
            active_rt_key: 0,
            #[cfg(windows)]
            backbuffer_rt_key: 0,
            #[cfg(windows)]
            backbuffer_rt_info: PendingRenderTarget::default(),
            black_clear_without_draw_pending: false,
            draws_since_clear: 0,
            readback_buf: Vec::new(),
            readback_x: 0,
            readback_y: 0,
            readback_width: 0,
            readback_height: 0,
            present_cache: PresentReadbackCache::default(),
            has_active_texture: false,
            texture_v_flip: false,
            active_tex_width: 0,
            active_tex_height: 0,
            active_tex_handle: 0,
            active_tex_format_code: 0,
            active_tex_source: "none",
            active_tex_byte_len: 0,
            active_tex_srv_bound: false,
            active_tex_guest_raw: 0,
            active_tex_guest_norm: 0,
            active_tex_data: 0,
            active_tex_guest_format_code: 0,
            active_tex_swizzled_blocks: false,
            active_tex_alpha_min: 0,
            active_tex_alpha_max: 0,
            active_tex_alpha_nonzero_sample: 0,
            active_tex_rgb_nonzero_sample: 0,
            active_tex_sample_count: 0,
            active_tex_probe_pixels: Vec::new(),
            active_tex_probe_width: 0,
            active_tex_probe_height: 0,
            next_tex_handle: 1,
            texture_stage_states: super::default_texture_stage_states(),
            ps_render_states: [0; 137],
            ps_constants: [[0.0; 4]; 32],
            ps_constant_written_mask: 0,
            ps_constants_dirty: true,
            render_state: RenderStateTable::default(),
            vs_constants,
            vs_constants_dirty: true,
            ffp_transforms: [FFP_IDENTITY_MATRIX; FFP_TRANSFORM_COUNT],
            ffp_transform_mask: 0,
            recent_draws: [RecentDrawSummary::default(); RECENT_DRAW_COUNT],
            recent_draw_cursor: 0,
            recent_draw_seen: 0,
            peter_candidate_overlay_points: Vec::new(),
        }
    }

    fn is_pixel_shader_render_state(state: u32) -> bool {
        state <= 59 || state == 136
    }

    fn next_texture_handle(&mut self) -> u32 {
        let handle = self.next_tex_handle;
        self.next_tex_handle = self.next_tex_handle.wrapping_add(1);
        if self.next_tex_handle == 0 {
            self.next_tex_handle = 1;
        }
        handle
    }

    #[cfg(windows)]
    fn sample_active_texture_rgba(&self, u: f32, v: f32) -> Option<[u8; 4]> {
        if self.active_tex_probe_pixels.is_empty()
            || self.active_tex_probe_width == 0
            || self.active_tex_probe_height == 0
        {
            return None;
        }
        let x = (u.clamp(0.0, 1.0) * (self.active_tex_probe_width.saturating_sub(1)) as f32).round()
            as u32;
        let y = (v.clamp(0.0, 1.0) * (self.active_tex_probe_height.saturating_sub(1)) as f32)
            .round() as u32;
        let index = y
            .saturating_mul(self.active_tex_probe_width)
            .saturating_add(x) as usize;
        let argb = *self.active_tex_probe_pixels.get(index)?;
        Some([
            ((argb >> 16) & 0xFF) as u8,
            ((argb >> 8) & 0xFF) as u8,
            (argb & 0xFF) as u8,
            ((argb >> 24) & 0xFF) as u8,
        ])
    }

    #[cfg(windows)]
    fn log_spidey_peter_ps_state(&self, draw_index: u32, verts: &[NV2AVertex], prim_type: u32) {
        if self.active_nv2a_vs != Some(0x0000_100D) || !spidey_peter_texel_probe_enabled() {
            return;
        }
        static PS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = PS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if !(n < 128 || n.is_power_of_two()) {
            return;
        }
        let s0 = &self.texture_stage_states[0];
        let texel_summary = [
            ("face", 0.50f32, 0.18f32),
            ("shirt", 0.50f32, 0.45f32),
            ("jeans", 0.50f32, 0.75f32),
            ("hand", 0.25f32, 0.35f32),
        ]
        .iter()
        .map(
            |(name, u, v)| match self.sample_active_texture_rgba(*u, *v) {
                Some(rgba) => format!(
                    "{}@{:.2},{:.2}={:02X}{:02X}{:02X}{:02X}",
                    name, u, v, rgba[0], rgba[1], rgba[2], rgba[3]
                ),
                None => format!("{}@{:.2},{:.2}=none", name, u, v),
            },
        )
        .collect::<Vec<_>>()
        .join(" ");
        let (color_min, color_max, color_avg, color_first, color_rgb_nonzero, color_alpha_nonzero) =
            if verts.is_empty() {
                ([0u8; 4], [0u8; 4], [0.0f32; 4], 0u32, 0usize, 0usize)
            } else {
                let mut min = [u8::MAX; 4];
                let mut max = [u8::MIN; 4];
                let mut sum = [0u64; 4];
                let mut rgb_nonzero = 0usize;
                let mut alpha_nonzero = 0usize;
                for v in verts {
                    let c = [
                        ((v.color >> 16) & 0xFF) as u8,
                        ((v.color >> 8) & 0xFF) as u8,
                        (v.color & 0xFF) as u8,
                        ((v.color >> 24) & 0xFF) as u8,
                    ];
                    for i in 0..4 {
                        min[i] = min[i].min(c[i]);
                        max[i] = max[i].max(c[i]);
                        sum[i] += u64::from(c[i]);
                    }
                    rgb_nonzero += usize::from((v.color & 0x00FF_FFFF) != 0);
                    alpha_nonzero += usize::from((v.color & 0xFF00_0000) != 0);
                }
                let len = verts.len() as f32;
                (
                    min,
                    max,
                    [
                        sum[0] as f32 / len,
                        sum[1] as f32 / len,
                        sum[2] as f32 / len,
                        sum[3] as f32 / len,
                    ],
                    verts[0].color,
                    rgb_nonzero,
                    alpha_nonzero,
                )
            };
        let mut vertex_samples = Vec::new();
        if !verts.is_empty() {
            let sample_indices = [
                0usize,
                verts.len() / 4,
                verts.len() / 2,
                verts.len().saturating_sub(1),
            ];
            for (label, vi) in ["first", "q1", "mid", "last"].iter().zip(sample_indices) {
                if let Some(v) = verts.get(vi) {
                    let color = [
                        ((v.color >> 16) & 0xFF) as u8,
                        ((v.color >> 8) & 0xFF) as u8,
                        (v.color & 0xFF) as u8,
                        ((v.color >> 24) & 0xFF) as u8,
                    ];
                    let texel = self.sample_active_texture_rgba(v.u, v.v);
                    let prediction = texel.map(|rgba| {
                        [
                            ((u16::from(rgba[0]) * u16::from(color[0])) / 255) as u8,
                            ((u16::from(rgba[1]) * u16::from(color[1])) / 255) as u8,
                            ((u16::from(rgba[2]) * u16::from(color[2])) / 255) as u8,
                            ((u16::from(rgba[3]) * u16::from(color[3])) / 255) as u8,
                        ]
                    });
                    let texel_text = match texel {
                        Some(rgba) => {
                            format!(
                                "{:02X}{:02X}{:02X}{:02X}",
                                rgba[0], rgba[1], rgba[2], rgba[3]
                            )
                        }
                        None => "none".to_string(),
                    };
                    let pred_text = match prediction {
                        Some(rgba) => {
                            format!(
                                "{:02X}{:02X}{:02X}{:02X}",
                                rgba[0], rgba[1], rgba[2], rgba[3]
                            )
                        }
                        None => "none".to_string(),
                    };
                    vertex_samples.push(format!(
                        "{}#{} uv={:.4},{:.4} diffuse={:02X}{:02X}{:02X}{:02X} tex={} ps_textured={}",
                        label,
                        vi,
                        v.u,
                        v.v,
                        color[0],
                        color[1],
                        color[2],
                        color[3],
                        texel_text,
                        pred_text
                    ));
                }
            }
        }
        let vertex_sample_summary = vertex_samples.join(" ");
        crate::xbox::emulator::debug_log(&format!(
            "[PETER-PS-STATE] #{} draw={} verts={} prim={} vs=0x{:08X} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} upload_fmt=0x{:02X}/{} source={} size={}x{} bytes={} srv_bound={} decoded_probe={}x{} rgba=\"{}\" diffuse_first=0x{:08X} diffuse_min={:02X}{:02X}{:02X}{:02X} diffuse_max={:02X}{:02X}{:02X}{:02X} diffuse_avg={:.1},{:.1},{:.1},{:.1} diffuse_nonzero={}/{} diffuse_alpha={}/{} vertex_samples=\"{}\" colorop0=0x{:08X} alphaop0=0x{:08X} colorarg1=0x{:08X} colorarg2=0x{:08X} alphaarg1=0x{:08X} alphaarg2=0x{:08X} factor=0x{:08X} blend={} src={:?} dst={:?} alpha_test={} alpha_ref={} alpha_func={:?} z={} zwrite={} color_write=0x{:X} ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_out0=0x{:08X} ps_final=[0x{:08X},0x{:08X}] ps_texmodes=0x{:08X}",
            n,
            draw_index,
            verts.len(),
            prim_type,
            self.active_nv2a_vs.unwrap_or(0),
            self.active_tex_handle,
            self.active_tex_guest_raw,
            self.active_tex_guest_norm,
            self.active_tex_data,
            self.active_tex_guest_format_code,
            crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
            self.active_tex_format_code,
            self.active_texture_format_name(),
            self.active_tex_source,
            self.active_tex_width,
            self.active_tex_height,
            self.active_tex_byte_len,
            self.active_tex_srv_bound,
            self.active_tex_probe_width,
            self.active_tex_probe_height,
            texel_summary,
            color_first,
            color_min[0],
            color_min[1],
            color_min[2],
            color_min[3],
            color_max[0],
            color_max[1],
            color_max[2],
            color_max[3],
            color_avg[0],
            color_avg[1],
            color_avg[2],
            color_avg[3],
            color_rgb_nonzero,
            verts.len(),
            color_alpha_nonzero,
            verts.len(),
            vertex_sample_summary,
            s0[super::X_D3DTSS_COLOROP],
            s0[super::X_D3DTSS_ALPHAOP],
            s0[super::X_D3DTSS_COLORARG1],
            s0[super::X_D3DTSS_COLORARG2],
            s0[super::X_D3DTSS_ALPHAARG1],
            s0[super::X_D3DTSS_ALPHAARG2],
            self.render_state.texture_factor,
            self.render_state.alpha_blend_enable,
            self.render_state.src_blend,
            self.render_state.dest_blend,
            self.render_state.alpha_test_enable,
            self.render_state.alpha_ref,
            self.render_state.alpha_func,
            self.render_state.z_enable,
            self.render_state.z_write_enable,
            self.render_state.color_write_enable,
            self.ps_render_states[53],
            self.ps_render_states[34],
            self.ps_render_states[0],
            self.ps_render_states[45],
            self.ps_render_states[8],
            self.ps_render_states[9],
            self.ps_render_states[136],
        ));
    }

    #[cfg(windows)]
    fn log_spidey_pixel_lane_trace(&self, draw_index: u32, verts: &[NV2AVertex], prim_type: u32) {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let enabled = *ENABLED.get_or_init(|| {
            std::env::var("RUSTEMU_SPIDEY_PIXEL_LANE_TRACE")
                .map(|value| {
                    let value = value.trim();
                    value == "1"
                        || value.eq_ignore_ascii_case("true")
                        || value.eq_ignore_ascii_case("yes")
                        || value.eq_ignore_ascii_case("on")
                })
                .unwrap_or(false)
        });
        if !enabled {
            return;
        }

        let Some(active_vs) = self.active_nv2a_vs else {
            return;
        };
        if active_vs != 0x0000_100D && active_vs != 0x0000_1001 {
            return;
        }

        static PIXEL_LANE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = PIXEL_LANE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if !(n < 160 || n.is_power_of_two()) {
            return;
        }

        fn fmt_words(words: &[u32]) -> String {
            words
                .iter()
                .map(|value| format!("{:08X}", value))
                .collect::<Vec<_>>()
                .join(",")
        }

        fn fmt_argb(value: u32) -> String {
            let a = ((value >> 24) & 0xFF) as f32 / 255.0;
            let r = ((value >> 16) & 0xFF) as f32 / 255.0;
            let g = ((value >> 8) & 0xFF) as f32 / 255.0;
            let b = (value & 0xFF) as f32 / 255.0;
            format!("{:08X}=rgba({:.3},{:.3},{:.3},{:.3})", value, r, g, b, a)
        }

        let s0 = &self.texture_stage_states[0];
        let s1 = &self.texture_stage_states[1];
        let psh_state =
            super::nv2a_psh::PixelShaderState::from_render_states(&self.ps_render_states);
        let translated = super::nv2a_psh::translate_basic_modulate(psh_state);
        let (ps_path, ps_key) = if let Some(program) = translated {
            ("translated", program.key)
        } else if self.has_active_texture {
            ("fallback-textured", 0)
        } else {
            ("fallback-flat", 0)
        };

        let (color_min, color_max, color_first, rgb_nonzero, alpha_nonzero) = if verts.is_empty() {
            ([0u8; 4], [0u8; 4], 0u32, 0usize, 0usize)
        } else {
            let mut min = [u8::MAX; 4];
            let mut max = [u8::MIN; 4];
            let mut rgb = 0usize;
            let mut alpha = 0usize;
            for v in verts {
                let c = [
                    ((v.color >> 16) & 0xFF) as u8,
                    ((v.color >> 8) & 0xFF) as u8,
                    (v.color & 0xFF) as u8,
                    ((v.color >> 24) & 0xFF) as u8,
                ];
                for i in 0..4 {
                    min[i] = min[i].min(c[i]);
                    max[i] = max[i].max(c[i]);
                }
                rgb += usize::from((v.color & 0x00FF_FFFF) != 0);
                alpha += usize::from((v.color & 0xFF00_0000) != 0);
            }
            (min, max, verts[0].color, rgb, alpha)
        };

        let sample_summary = if verts.is_empty() {
            "none".to_string()
        } else {
            let sample_indices = [
                0usize,
                verts.len() / 4,
                verts.len() / 2,
                verts.len().saturating_sub(1),
            ];
            ["first", "q1", "mid", "last"]
                .iter()
                .zip(sample_indices)
                .filter_map(|(label, index)| {
                    let v = verts.get(index)?;
                    let texel = self.sample_active_texture_rgba(v.u, v.v);
                    let texel_text = texel
                        .map(|rgba| {
                            format!(
                                "{:02X}{:02X}{:02X}{:02X}",
                                rgba[0], rgba[1], rgba[2], rgba[3]
                            )
                        })
                        .unwrap_or_else(|| "none".to_string());
                    Some(format!(
                        "{}#{} uv={:.4},{:.4} diffuse=0x{:08X} tex={}",
                        label, index, v.u, v.v, v.color, texel_text
                    ))
                })
                .collect::<Vec<_>>()
                .join(" ")
        };

        crate::xbox::emulator::debug_log(&format!(
            "[SPIDEY-PIXEL-LANE] #{} draw={} vs=0x{:08X} verts={} prim={} ps_path={} ps_key=0x{:016X} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} upload_fmt=0x{:08X}/{} source={} size={}x{} bytes={} srv_bound={} probe={}x{} samples=\"{}\" diffuse_first=0x{:08X} diffuse_min={:02X}{:02X}{:02X}{:02X} diffuse_max={:02X}{:02X}{:02X}{:02X} rgb_nonzero={}/{} alpha_nonzero={}/{} tfactor={} tss0=color(op=0x{:08X},a0=0x{:08X},a1=0x{:08X},a2=0x{:08X}) alpha(op=0x{:08X},a0=0x{:08X},a1=0x{:08X},a2=0x{:08X}) result=0x{:08X} tc=0x{:08X} xform=0x{:08X} tss1=color(op=0x{:08X},a0=0x{:08X},a1=0x{:08X},a2=0x{:08X}) alpha(op=0x{:08X},a0=0x{:08X},a1=0x{:08X},a2=0x{:08X}) result=0x{:08X} tc=0x{:08X} xform=0x{:08X} combiner_count=0x{:08X} texmodes=0x{:08X} input_texture=0x{:08X} compare=0x{:08X} dot=0x{:08X} c0map=0x{:08X} c1map=0x{:08X} final_abcd=0x{:08X} final_efg=0x{:08X} final_c0={} final_c1={} rgb_inputs=[{}] alpha_inputs=[{}] rgb_outputs=[{}] alpha_outputs=[{}] const0=[{}] const1=[{}] blend={} src={:?} dst={:?} z={} zwrite={} color_write=0x{:X}",
            n,
            draw_index,
            active_vs,
            verts.len(),
            prim_type,
            ps_path,
            ps_key,
            self.active_tex_handle,
            self.active_tex_guest_raw,
            self.active_tex_guest_norm,
            self.active_tex_data,
            self.active_tex_guest_format_code,
            crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
            self.active_tex_format_code,
            self.active_texture_format_name(),
            self.active_tex_source,
            self.active_tex_width,
            self.active_tex_height,
            self.active_tex_byte_len,
            self.active_tex_srv_bound,
            self.active_tex_probe_width,
            self.active_tex_probe_height,
            sample_summary,
            color_first,
            color_min[0],
            color_min[1],
            color_min[2],
            color_min[3],
            color_max[0],
            color_max[1],
            color_max[2],
            color_max[3],
            rgb_nonzero,
            verts.len(),
            alpha_nonzero,
            verts.len(),
            fmt_argb(self.render_state.texture_factor),
            s0[super::X_D3DTSS_COLOROP],
            s0[super::X_D3DTSS_COLORARG0],
            s0[super::X_D3DTSS_COLORARG1],
            s0[super::X_D3DTSS_COLORARG2],
            s0[super::X_D3DTSS_ALPHAOP],
            s0[super::X_D3DTSS_ALPHAARG0],
            s0[super::X_D3DTSS_ALPHAARG1],
            s0[super::X_D3DTSS_ALPHAARG2],
            s0[super::X_D3DTSS_RESULTARG],
            s0[super::X_D3DTSS_TEXCOORDINDEX],
            s0[super::X_D3DTSS_TEXTURETRANSFORMFLAGS],
            s1[super::X_D3DTSS_COLOROP],
            s1[super::X_D3DTSS_COLORARG0],
            s1[super::X_D3DTSS_COLORARG1],
            s1[super::X_D3DTSS_COLORARG2],
            s1[super::X_D3DTSS_ALPHAOP],
            s1[super::X_D3DTSS_ALPHAARG0],
            s1[super::X_D3DTSS_ALPHAARG1],
            s1[super::X_D3DTSS_ALPHAARG2],
            s1[super::X_D3DTSS_RESULTARG],
            s1[super::X_D3DTSS_TEXCOORDINDEX],
            s1[super::X_D3DTSS_TEXTURETRANSFORMFLAGS],
            psh_state.combiner_count,
            psh_state.texture_modes,
            psh_state.input_texture,
            psh_state.compare_mode,
            psh_state.dot_mapping,
            psh_state.c0_mapping,
            psh_state.c1_mapping,
            psh_state.final_abcd,
            psh_state.final_efg,
            fmt_argb(psh_state.final_constant0),
            fmt_argb(psh_state.final_constant1),
            fmt_words(&psh_state.rgb_inputs),
            fmt_words(&psh_state.alpha_inputs),
            fmt_words(&psh_state.rgb_outputs),
            fmt_words(&psh_state.alpha_outputs),
            fmt_words(&psh_state.const0),
            fmt_words(&psh_state.const1),
            self.render_state.alpha_blend_enable,
            self.render_state.src_blend,
            self.render_state.dest_blend,
            self.render_state.z_enable,
            self.render_state.z_write_enable,
            self.render_state.color_write_enable,
        ));
    }

    fn set_vs_constant_if_changed(&mut self, reg: usize, value: [f32; 4]) -> bool {
        let Some(slot) = self.vs_constants.get_mut(reg) else {
            return false;
        };
        let changed = slot
            .iter()
            .zip(value.iter())
            .any(|(a, b)| (*a - *b).abs() > 1.0e-6);
        if changed {
            *slot = value;
        }
        changed
    }

    #[cfg(windows)]
    fn mark_vs_constants_dirty(&mut self) {
        self.vs_constants_dirty = true;
    }

    #[cfg(windows)]
    fn upload_vs_constants_if_dirty(&mut self) {
        if !self.vs_constants_dirty {
            return;
        }
        self.upload_vs_constants();
        self.vs_constants_dirty = false;
    }

    #[cfg(windows)]
    fn mark_ps_constants_dirty(&mut self) {
        self.ps_constants_dirty = true;
    }

    #[cfg(windows)]
    fn upload_ps_constants_if_dirty(&mut self) {
        if !self.ps_constants_dirty {
            return;
        }
        self.upload_ps_constants();
        self.ps_constants_dirty = false;
    }

    fn update_cxbxr_screenspace_constants(
        &mut self,
        draw_index: u32,
        rt_key: u32,
        rt_width: u32,
        rt_height: u32,
    ) -> bool {
        let target_w = if rt_width != 0 {
            rt_width
        } else {
            self.width.max(1) as u32
        }
        .max(1) as f32;
        let target_h = if rt_height != 0 {
            rt_height
        } else {
            self.height.max(1) as u32
        }
        .max(1) as f32;

        // Adapted from Cxbx-Reloaded's
        // CxbxUpdateHostViewPortOffsetAndScaleConstants. D3D11 does not need
        // the D3D9 half-pixel offset, and final clip-Z remap remains in HLSL.
        let scale = [target_w * 0.5, -target_h * 0.5, 1.0, 1.0];
        let offset = [target_w * 0.5, target_h * 0.5, 0.0, 0.0];

        let mut changed = false;
        changed |= self.set_vs_constant_if_changed(CXBX_D3DVS_SCREENSPACE_SCALE_BASE, scale);
        changed |= self.set_vs_constant_if_changed(CXBX_D3DVS_SCREENSPACE_OFFSET_BASE, offset);

        if changed {
            static CXBXR_CONST_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = CXBXR_CONST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CXBXR-SCREENSPACE-CONST] #{} draw={} rt=0x{:08X} {}x{} c{}=[{:.3},{:.3},{:.3},{:.3}] c{}=[{:.3},{:.3},{:.3},{:.3}]",
                    n,
                    draw_index,
                    rt_key,
                    target_w as u32,
                    target_h as u32,
                    CXBX_D3DVS_SCREENSPACE_SCALE_BASE,
                    scale[0],
                    scale[1],
                    scale[2],
                    scale[3],
                    CXBX_D3DVS_SCREENSPACE_OFFSET_BASE,
                    offset[0],
                    offset[1],
                    offset[2],
                    offset[3]
                ));
            }
        }
        changed
    }

    #[cfg(windows)]
    fn log_spidey_synth_rt_constants_diag(
        &self,
        draw_index: u32,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        rt_key: u32,
        rt_width: u32,
        rt_height: u32,
        rt_data: u32,
        rt_pitch: u32,
        vp_x: f32,
        vp_y: f32,
        vp_w: f32,
        vp_h: f32,
        vp_mode: &str,
    ) {
        if !crate::xbox::emulator::spidey_synth_training_enabled()
            || !(max_x > 320.0 && max_y > 240.0)
        {
            return;
        }
        static RT_CONST_DIAG_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = RT_CONST_DIAG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if !(n < 512 || n.is_power_of_two()) {
            return;
        }
        let c212 = self
            .vs_constants
            .get(CXBX_D3DVS_SCREENSPACE_SCALE_BASE)
            .copied()
            .unwrap_or([0.0; 4]);
        let c213 = self
            .vs_constants
            .get(CXBX_D3DVS_SCREENSPACE_OFFSET_BASE)
            .copied()
            .unwrap_or([0.0; 4]);
        crate::xbox::emulator::debug_log(&format!(
            "[B-DIAG-C212] #{} draw={} rt=0x{:08X} dims={}x{} data=0x{:08X} pitch={} viewport={} {:.0}x{:.0}+{:.0}+{:.0} scissor=untracked c212=[{:.3},{:.3},{:.3},{:.3}] c213=[{:.3},{:.3},{:.3},{:.3}] bbox=[{:.1},{:.1}..{:.1},{:.1}] active_vs=0x{:08X}",
            n,
            draw_index,
            rt_key,
            rt_width,
            rt_height,
            rt_data,
            rt_pitch,
            vp_mode,
            vp_w,
            vp_h,
            vp_x,
            vp_y,
            c212[0],
            c212[1],
            c212[2],
            c212[3],
            c213[0],
            c213[1],
            c213[2],
            c213[3],
            min_x,
            min_y,
            max_x,
            max_y,
            self.active_nv2a_vs.unwrap_or(0)
        ));
    }

    #[cfg(windows)]
    fn format_vs_const_rows(&self, regs: &[usize]) -> String {
        use std::fmt::Write as _;

        let mut out = String::new();
        for &reg in regs {
            let Some(c) = self.vs_constants.get(reg).copied() else {
                continue;
            };
            let _ = write!(
                out,
                " c{}=[{:.4},{:.4},{:.4},{:.4}]",
                reg, c[0], c[1], c[2], c[3]
            );
        }
        out
    }

    #[cfg(windows)]
    fn log_spidey_peter_vs_constants_diag(
        &self,
        draw_index: u32,
        prim_type: i32,
        verts: &[NV2AVertex],
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        rt_key: u32,
        rt_width: u32,
        rt_height: u32,
        rt_data: u32,
        rt_pitch: u32,
        vp_x: f32,
        vp_y: f32,
        vp_w: f32,
        vp_h: f32,
        vp_mode: &str,
    ) {
        if !crate::xbox::emulator::spidey_synth_training_enabled() {
            return;
        }

        let post_peterstu = crate::xbox::emulator::spidey_post_peterstu_vshader_key_seq() != 0;
        let active_vs = self.active_nv2a_vs.unwrap_or(0);
        let on_backbuffer = rt_key == self.backbuffer_rt_key || rt_width >= 640 || rt_height >= 480;
        let peter_candidate =
            post_peterstu && active_vs == 0x0000_1003 && on_backbuffer && max_x < 16.0;
        let compare_candidate = post_peterstu && active_vs != 0x0000_1003 && on_backbuffer;

        static BACKBUFFER_SAMPLE_DRAW: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let frame_sample = on_backbuffer
            .then(|| BACKBUFFER_SAMPLE_DRAW.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let frame_sample_candidate = frame_sample
            .map(|n| n < 128 || (n & 0x03ff) < 64)
            .unwrap_or(false);

        if !peter_candidate && !compare_candidate && !frame_sample_candidate {
            return;
        }

        static PETER_CONST_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        static COMPARE_CONST_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let frame_sample_selected = !peter_candidate && frame_sample_candidate;
        let (kind, n) = if peter_candidate {
            (
                "peter",
                PETER_CONST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            )
        } else if frame_sample_selected {
            ("frame_sample", frame_sample.unwrap_or(0))
        } else {
            (
                "compare",
                COMPARE_CONST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            )
        };
        if !frame_sample_selected && !(n < 16 || n.is_power_of_two()) {
            return;
        }

        let mut regs = Vec::with_capacity(52);
        regs.extend(0usize..=15);
        regs.extend(58usize..=63);
        regs.extend(96usize..=123);
        regs.push(CXBX_D3DVS_SCREENSPACE_SCALE_BASE);
        regs.push(CXBX_D3DVS_SCREENSPACE_OFFSET_BASE);
        let constants = self.format_vs_const_rows(&regs);
        let guest_vs_raw =
            crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(std::sync::atomic::Ordering::Relaxed);
        let stream_stride =
            crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.load(std::sync::atomic::Ordering::Relaxed);
        let projected = verts
            .first()
            .map(|v| {
                let p = [v.x, v.y, v.z, v.w];
                let dot = |c: [f32; 4]| -> f32 {
                    p[0] * c[0] + p[1] * c[1] + p[2] * c[2] + p[3] * c[3]
                };
                let clip = [
                    dot(self.vs_constants[4]),
                    dot(self.vs_constants[5]),
                    dot(self.vs_constants[6]),
                    dot(self.vs_constants[7]),
                ];
                let inv_w = if clip[3].abs() > 1.0e-6 {
                    1.0 / clip[3]
                } else {
                    0.0
                };
                let ndc_x = clip[0] * inv_w;
                let ndc_y = clip[1] * inv_w;
                let ndc_z = clip[2] * inv_w;
                let c58 = self.vs_constants[58];
                let c59 = self.vs_constants[59];
                let xbox_x = ndc_x * c58[0] + c59[0];
                let xbox_y = ndc_y * c58[1] + c59[1];
                let on_screen = ndc_x.is_finite()
                    && ndc_y.is_finite()
                    && ndc_z.is_finite()
                    && ndc_x.abs() <= 1.0
                    && ndc_y.abs() <= 1.0
                    && ndc_z >= 0.0
                    && ndc_z <= 1.0;
                format!(
                    " clip4=[{:.3},{:.3},{:.3},{:.3}] ndc=[{:.3},{:.3},{:.3}] xbox_xy=[{:.1},{:.1}] on_screen={}",
                    clip[0],
                    clip[1],
                    clip[2],
                    clip[3],
                    ndc_x,
                    ndc_y,
                    ndc_z,
                    xbox_x,
                    xbox_y,
                    on_screen as u8
                )
            })
            .unwrap_or_default();

        crate::xbox::emulator::debug_log(&format!(
            "[SPIDEY-PETER-VS-CONST] #{} kind={} draw={} active_vs=0x{:08X} guest_vs=0x{:08X} {} stream0_stride={} verts={} prim={} rt=0x{:08X} {}x{} data=0x{:08X} pitch={} viewport={} {:.0}x{:.0}+{:.0}+{:.0} bbox=[{:.1},{:.1}..{:.1},{:.1}] first=[{}]{}{}",
            n,
            kind,
            draw_index,
            active_vs,
            guest_vs_raw,
            spidey_describe_guest_vs(guest_vs_raw),
            stream_stride,
            verts.len(),
            prim_type,
            rt_key,
            rt_width,
            rt_height,
            rt_data,
            rt_pitch,
            vp_mode,
            vp_w,
            vp_h,
            vp_x,
            vp_y,
            min_x,
            min_y,
            max_x,
            max_y,
            spidey_format_vertex_sample(verts),
            projected,
            constants
        ));
    }

    #[cfg(windows)]
    fn log_spidey_100d_skin_eval(
        &self,
        draw_index: u32,
        prim_type: u32,
        verts: &[NV2AVertex],
        skin: &[VertexSkinPayload],
    ) {
        if self.active_nv2a_vs != Some(0x0000_100D) || verts.is_empty() || skin.is_empty() {
            return;
        }

        static SKIN_EVAL_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = SKIN_EVAL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let summary_log_enabled = n < 16 || n.is_power_of_two();
        let outlier_probe_enabled = spidey_peter_outlier_probe_enabled()
            && spidey_peter_outlier_draw_filter().map_or(true, |wanted| wanted == draw_index);
        if !summary_log_enabled && !outlier_probe_enabled {
            return;
        }

        let sample_index = skin
            .iter()
            .position(|p| {
                p.blend_indices.iter().any(|v| v.abs() > 0.0001)
                    || p.blend_weights.iter().any(|v| v.abs() > 0.0001)
            })
            .unwrap_or(0)
            .min(verts.len().saturating_sub(1));
        let v = verts[sample_index];
        let payload = skin[sample_index];
        let p = [v.x, v.y, v.z, 1.0f32];

        let dot4 = |row: [f32; 4]| -> f32 {
            p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
        };
        let raw4_formula = spidey_skin_raw4_probe_enabled();
        let a0_from_skin = |blend_index: f32| -> f32 {
            if raw4_formula {
                blend_index * 4.0 - 92.0
            } else {
                blend_index * self.vs_constants[1][0] + self.vs_constants[1][1]
            }
        };
        let a0_key = |a0: f32| -> i32 { (a0 + 0.001).floor() as i32 };
        let plain_index = |a0: f32, offset: i32| -> usize {
            let mut i = a0_key(a0) + offset;
            if i < 0 {
                i += 96;
            }
            i.clamp(0, 191) as usize
        };
        let spidey_index = |a0: f32, offset: i32| -> usize {
            let raw = a0_key(a0);
            let skin_numerator = raw + 92;
            if skin_numerator >= 0 && skin_numerator % 4 == 0 && (0..=2).contains(&offset) {
                let bone = skin_numerator / 4;
                if (0..48).contains(&bone) {
                    return (17 + bone * 3 + offset).clamp(0, 191) as usize;
                }
            }
            plain_index(a0, offset)
        };
        let hlsl_index = |a0: f32, offset: i32| -> Option<usize> {
            let i = a0_key(a0) + offset + 96;
            (0..192).contains(&i).then_some(i as usize)
        };
        let eval_with = |index_fn: &dyn Fn(f32, i32) -> usize| -> ([f32; 3], String) {
            let mut out = [0.0f32; 3];
            let mut lanes = Vec::new();
            for lane in 0..4 {
                let a0 = a0_from_skin(payload.blend_indices[lane]);
                let weight = payload.blend_weights[lane];
                let i0 = index_fn(a0, 0);
                let i1 = index_fn(a0, 1);
                let i2 = index_fn(a0, 2);
                let r = [
                    dot4(self.vs_constants[i0]),
                    dot4(self.vs_constants[i1]),
                    dot4(self.vs_constants[i2]),
                ];
                out[0] += r[0] * weight;
                out[1] += r[1] * weight;
                out[2] += r[2] * weight;
                lanes.push(format!(
                    "l{}:v5={:.5}:a0={:.1}:w={:.3}:c{}-{}-{}:r=[{:.3},{:.3},{:.3}]",
                    lane, payload.blend_indices[lane], a0, weight, i0, i1, i2, r[0], r[1], r[2]
                ));
            }
            (out, lanes.join(" "))
        };
        let eval_hlsl = || -> ([f32; 3], String) {
            let mut out = [0.0f32; 3];
            let mut lanes = Vec::new();
            for lane in 0..4 {
                let a0 = a0_from_skin(payload.blend_indices[lane]);
                let weight = payload.blend_weights[lane];
                let i0 = hlsl_index(a0, 0);
                let i1 = hlsl_index(a0, 1);
                let i2 = hlsl_index(a0, 2);
                let row0 = i0.map(|i| self.vs_constants[i]).unwrap_or([0.0; 4]);
                let row1 = i1.map(|i| self.vs_constants[i]).unwrap_or([0.0; 4]);
                let row2 = i2.map(|i| self.vs_constants[i]).unwrap_or([0.0; 4]);
                let r = [dot4(row0), dot4(row1), dot4(row2)];
                out[0] += r[0] * weight;
                out[1] += r[1] * weight;
                out[2] += r[2] * weight;
                lanes.push(format!(
                    "l{}:v5={:.5}:a0={:.1}:w={:.3}:c{}-{}-{}:r=[{:.3},{:.3},{:.3}]",
                    lane,
                    payload.blend_indices[lane],
                    a0,
                    weight,
                    i0.map(|i| i.to_string())
                        .unwrap_or_else(|| "oor".to_string()),
                    i1.map(|i| i.to_string())
                        .unwrap_or_else(|| "oor".to_string()),
                    i2.map(|i| i.to_string())
                        .unwrap_or_else(|| "oor".to_string()),
                    r[0],
                    r[1],
                    r[2]
                ));
            }
            (out, lanes.join(" "))
        };

        let (spidey_pos, spidey_lanes) = eval_with(&spidey_index);
        let (plain_pos, plain_lanes) = eval_with(&plain_index);
        let (hlsl_pos, hlsl_lanes) = eval_hlsl();
        let project_to_xbox = |pos: [f32; 3]| -> ([f32; 4], [f32; 3], [f32; 2], bool) {
            let p = [pos[0], pos[1], pos[2], 1.0f32];
            let dot_row = |row: [f32; 4]| -> f32 {
                p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
            };
            let clip = [
                dot_row(self.vs_constants[4]),
                dot_row(self.vs_constants[5]),
                dot_row(self.vs_constants[6]),
                dot_row(self.vs_constants[7]),
            ];
            let inv_w = if clip[3].abs() > 1.0e-6 {
                1.0 / clip[3]
            } else {
                f32::NAN
            };
            let ndc = [clip[0] * inv_w, clip[1] * inv_w, clip[2] * inv_w];
            let screen_scale = self.vs_constants[CXBX_D3DVS_SCREENSPACE_SCALE_BASE];
            let screen_offset = self.vs_constants[CXBX_D3DVS_SCREENSPACE_OFFSET_BASE];
            let screen = [
                ndc[0] * screen_scale[0] + screen_offset[0],
                ndc[1] * screen_scale[1] + screen_offset[1],
            ];
            let on_screen = screen[0].is_finite()
                && screen[1].is_finite()
                && (0.0..=640.0).contains(&screen[0])
                && (0.0..=480.0).contains(&screen[1]);
            (clip, ndc, screen, on_screen)
        };
        let (spidey_clip, spidey_ndc, spidey_screen, spidey_on) = project_to_xbox(spidey_pos);
        let (plain_clip, plain_ndc, plain_screen, plain_on) = project_to_xbox(plain_pos);
        let (hlsl_clip, hlsl_ndc, hlsl_screen, hlsl_on) = project_to_xbox(hlsl_pos);

        let vert_count = verts.len().min(skin.len());
        let mut skin_min = [f32::INFINITY; 3];
        let mut skin_max = [f32::NEG_INFINITY; 3];
        let mut clip_min = [f32::INFINITY; 4];
        let mut clip_max = [f32::NEG_INFINITY; 4];
        let mut ndc_min = [f32::INFINITY; 3];
        let mut ndc_max = [f32::NEG_INFINITY; 3];
        let mut screen_min = [f32::INFINITY; 2];
        let mut screen_max = [f32::NEG_INFINITY; 2];
        let mut finite_vertices = 0usize;
        let mut on_screen_vertices = 0usize;
        let mut left_vertices = 0usize;
        let mut right_vertices = 0usize;
        let mut above_vertices = 0usize;
        let mut below_vertices = 0usize;
        let mut nonfinite_vertices = 0usize;
        let mut const_oor = 0usize;
        let mut active_weight_lanes = 0usize;
        let mut a0_hist: [std::collections::BTreeMap<i32, usize>; 4] =
            std::array::from_fn(|_| std::collections::BTreeMap::new());
        let mut a0_weighted_hist: [std::collections::BTreeMap<i32, usize>; 4] =
            std::array::from_fn(|_| std::collections::BTreeMap::new());
        let mut outlier_screens = if outlier_probe_enabled {
            vec![None::<[f32; 2]>; vert_count]
        } else {
            Vec::new()
        };

        let dot4_for = |p: [f32; 4], row: [f32; 4]| -> f32 {
            p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
        };
        static OUTLIER_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        for i in 0..vert_count {
            let v = verts[i];
            let payload = skin[i];
            let p = [v.x, v.y, v.z, 1.0f32];
            let mut pos = [0.0f32; 3];
            let mut lane_details = if outlier_probe_enabled {
                Vec::with_capacity(4)
            } else {
                Vec::new()
            };
            for lane in 0..4 {
                let a0 = a0_from_skin(payload.blend_indices[lane]);
                let key = a0_key(a0);
                *a0_hist[lane].entry(key).or_insert(0) += 1;
                let weight = payload.blend_weights[lane];
                if weight.abs() > 1.0e-6 {
                    active_weight_lanes += 1;
                    *a0_weighted_hist[lane].entry(key).or_insert(0) += 1;
                }
                let rows = [hlsl_index(a0, 0), hlsl_index(a0, 1), hlsl_index(a0, 2)];
                let mut r = [0.0f32; 3];
                let mut row_values = [[0.0f32; 4]; 3];
                for component in 0..3 {
                    if let Some(index) = rows[component] {
                        row_values[component] = self.vs_constants[index];
                        r[component] = dot4_for(p, row_values[component]);
                    } else {
                        const_oor += 1;
                    }
                    pos[component] += r[component] * weight;
                }
                if outlier_probe_enabled {
                    lane_details.push(format!(
                        "l{}:v5={:.6}:w={:.4}:a0={:.3}:key={}:slots={}/{}/{}:r=[{:.4},{:.4},{:.4}]:c0=[{:.4},{:.4},{:.4},{:.4}]:c1=[{:.4},{:.4},{:.4},{:.4}]:c2=[{:.4},{:.4},{:.4},{:.4}]",
                        lane,
                        payload.blend_indices[lane],
                        weight,
                        a0,
                        key,
                        rows[0].map(|idx| idx.to_string()).unwrap_or_else(|| "oor".to_string()),
                        rows[1].map(|idx| idx.to_string()).unwrap_or_else(|| "oor".to_string()),
                        rows[2].map(|idx| idx.to_string()).unwrap_or_else(|| "oor".to_string()),
                        r[0],
                        r[1],
                        r[2],
                        row_values[0][0],
                        row_values[0][1],
                        row_values[0][2],
                        row_values[0][3],
                        row_values[1][0],
                        row_values[1][1],
                        row_values[1][2],
                        row_values[1][3],
                        row_values[2][0],
                        row_values[2][1],
                        row_values[2][2],
                        row_values[2][3],
                    ));
                }
            }

            let (clip, ndc, screen, on_screen) = project_to_xbox(pos);
            if outlier_probe_enabled {
                let clip_w = clip[3].abs().max(1.0e-6);
                let clip_outlier = clip[0].is_finite() && clip[0].abs() > clip_w * 4.0;
                let screen_outlier = screen[0].is_finite()
                    && screen[1].is_finite()
                    && (screen[0] < -64.0
                        || screen[0] > 704.0
                        || screen[1] < -64.0
                        || screen[1] > 544.0);
                if screen[0].is_finite() && screen[1].is_finite() {
                    outlier_screens[i] = Some(screen);
                }
                if clip_outlier || screen_outlier || !screen[0].is_finite() || !clip[0].is_finite()
                {
                    let on = OUTLIER_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if on < 256 {
                        crate::xbox::emulator::debug_log(&format!(
                            "[PETER-OUTLIER] #{} draw={} vi={} verts={} raw_pos=[{:.5},{:.5},{:.5},1] raw_v5=[{:.6},{:.6},{:.6},{:.6}] raw_v6=[{:.4},{:.4},{:.4},{:.4}] pos=[{:.5},{:.5},{:.5}] clip=[{:.5},{:.5},{:.5},{:.5}] ndc=[{:.5},{:.5},{:.5}] screen=[{:.2},{:.2}] clip_outlier={} screen_outlier={} c1=[{:.6},{:.6},{:.6},{:.6}] lanes=\"{}\"",
                            on,
                            draw_index,
                            i,
                            vert_count,
                            p[0],
                            p[1],
                            p[2],
                            payload.blend_indices[0],
                            payload.blend_indices[1],
                            payload.blend_indices[2],
                            payload.blend_indices[3],
                            payload.blend_weights[0],
                            payload.blend_weights[1],
                            payload.blend_weights[2],
                            payload.blend_weights[3],
                            pos[0],
                            pos[1],
                            pos[2],
                            clip[0],
                            clip[1],
                            clip[2],
                            clip[3],
                            ndc[0],
                            ndc[1],
                            ndc[2],
                            screen[0],
                            screen[1],
                            clip_outlier as u8,
                            screen_outlier as u8,
                            self.vs_constants[1][0],
                            self.vs_constants[1][1],
                            self.vs_constants[1][2],
                            self.vs_constants[1][3],
                            lane_details.join(" ")
                        ));
                    }
                }
            }
            let finite = pos.iter().all(|v| v.is_finite())
                && clip.iter().all(|v| v.is_finite())
                && ndc.iter().all(|v| v.is_finite())
                && screen.iter().all(|v| v.is_finite());
            if finite {
                finite_vertices += 1;
                for component in 0..3 {
                    skin_min[component] = skin_min[component].min(pos[component]);
                    skin_max[component] = skin_max[component].max(pos[component]);
                    ndc_min[component] = ndc_min[component].min(ndc[component]);
                    ndc_max[component] = ndc_max[component].max(ndc[component]);
                }
                for component in 0..4 {
                    clip_min[component] = clip_min[component].min(clip[component]);
                    clip_max[component] = clip_max[component].max(clip[component]);
                }
                for component in 0..2 {
                    screen_min[component] = screen_min[component].min(screen[component]);
                    screen_max[component] = screen_max[component].max(screen[component]);
                }
                if on_screen {
                    on_screen_vertices += 1;
                }
                if screen[0] < 0.0 {
                    left_vertices += 1;
                } else if screen[0] > 640.0 {
                    right_vertices += 1;
                }
                if screen[1] < 0.0 {
                    above_vertices += 1;
                } else if screen[1] > 480.0 {
                    below_vertices += 1;
                }
            } else {
                nonfinite_vertices += 1;
            }
        }

        if outlier_probe_enabled && vert_count >= 3 {
            static TRI_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let edge_len = |a: [f32; 2], b: [f32; 2]| -> f32 {
                let dx = a[0] - b[0];
                let dy = a[1] - b[1];
                (dx * dx + dy * dy).sqrt()
            };
            let scan_tri = |tri_index: usize, ia: usize, ib: usize, ic: usize| {
                let Some(a) = outlier_screens.get(ia).and_then(|v| *v) else {
                    return;
                };
                let Some(b) = outlier_screens.get(ib).and_then(|v| *v) else {
                    return;
                };
                let Some(c) = outlier_screens.get(ic).and_then(|v| *v) else {
                    return;
                };
                let e01 = edge_len(a, b);
                let e12 = edge_len(b, c);
                let e20 = edge_len(c, a);
                let max_edge = e01.max(e12).max(e20);
                let min_x = a[0].min(b[0]).min(c[0]);
                let max_x = a[0].max(b[0]).max(c[0]);
                let min_y = a[1].min(b[1]).min(c[1]);
                let max_y = a[1].max(b[1]).max(c[1]);
                let wide_span = max_x - min_x > 700.0 || max_y - min_y > 540.0;
                let huge_edge = max_edge > 700.0;
                if !(wide_span || huge_edge) {
                    return;
                }
                let tn = TRI_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if tn >= 128 {
                    return;
                }
                let va = skin[ia];
                let vb = skin[ib];
                let vc = skin[ic];
                crate::xbox::emulator::debug_log(&format!(
                    "[PETER-TRI-OUTLIER] #{} draw={} prim={} tri={} vi={}/{}/{} screen=[{:.1},{:.1}]/[{:.1},{:.1}]/[{:.1},{:.1}] edge=[{:.1},{:.1},{:.1}] bbox=[{:.1},{:.1}..{:.1},{:.1}] v5a=[{:.5},{:.5},{:.5},{:.5}] w6a=[{:.3},{:.3},{:.3},{:.3}] v5b=[{:.5},{:.5},{:.5},{:.5}] w6b=[{:.3},{:.3},{:.3},{:.3}] v5c=[{:.5},{:.5},{:.5},{:.5}] w6c=[{:.3},{:.3},{:.3},{:.3}] c1=[{:.6},{:.6},{:.6},{:.6}]",
                    tn,
                    draw_index,
                    prim_type,
                    tri_index,
                    ia,
                    ib,
                    ic,
                    a[0],
                    a[1],
                    b[0],
                    b[1],
                    c[0],
                    c[1],
                    e01,
                    e12,
                    e20,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    va.blend_indices[0],
                    va.blend_indices[1],
                    va.blend_indices[2],
                    va.blend_indices[3],
                    va.blend_weights[0],
                    va.blend_weights[1],
                    va.blend_weights[2],
                    va.blend_weights[3],
                    vb.blend_indices[0],
                    vb.blend_indices[1],
                    vb.blend_indices[2],
                    vb.blend_indices[3],
                    vb.blend_weights[0],
                    vb.blend_weights[1],
                    vb.blend_weights[2],
                    vb.blend_weights[3],
                    vc.blend_indices[0],
                    vc.blend_indices[1],
                    vc.blend_indices[2],
                    vc.blend_indices[3],
                    vc.blend_weights[0],
                    vc.blend_weights[1],
                    vc.blend_weights[2],
                    vc.blend_weights[3],
                    self.vs_constants[1][0],
                    self.vs_constants[1][1],
                    self.vs_constants[1][2],
                    self.vs_constants[1][3],
                ));
            };

            if prim_type == 6 {
                for tri in 0..vert_count.saturating_sub(2) {
                    scan_tri(tri, tri, tri + 1, tri + 2);
                }
            } else {
                for tri in 0..(vert_count / 3) {
                    let i = tri * 3;
                    scan_tri(tri, i, i + 1, i + 2);
                }
            }
        }

        if !summary_log_enabled {
            return;
        }

        let fmt_range3 = |min: [f32; 3], max: [f32; 3]| -> String {
            if min[0].is_infinite() {
                "empty".to_string()
            } else {
                format!(
                    "[{:.3},{:.3},{:.3}..{:.3},{:.3},{:.3}]",
                    min[0], min[1], min[2], max[0], max[1], max[2]
                )
            }
        };
        let fmt_range4 = |min: [f32; 4], max: [f32; 4]| -> String {
            if min[0].is_infinite() {
                "empty".to_string()
            } else {
                format!(
                    "[{:.3},{:.3},{:.3},{:.3}..{:.3},{:.3},{:.3},{:.3}]",
                    min[0], min[1], min[2], min[3], max[0], max[1], max[2], max[3]
                )
            }
        };
        let fmt_range2 = |min: [f32; 2], max: [f32; 2]| -> String {
            if min[0].is_infinite() {
                "empty".to_string()
            } else {
                format!("[{:.1},{:.1}..{:.1},{:.1}]", min[0], min[1], max[0], max[1])
            }
        };
        let fmt_hist = |map: &std::collections::BTreeMap<i32, usize>| -> String {
            let mut pairs = map
                .iter()
                .map(|(key, count)| (*count, *key))
                .collect::<Vec<_>>();
            pairs.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
            pairs
                .iter()
                .take(12)
                .map(|(count, key)| format!("{}:{}", key, count))
                .collect::<Vec<_>>()
                .join(",")
        };
        let a0_hist_summary = (0..4)
            .map(|lane| format!("l{}{{{}}}", lane, fmt_hist(&a0_hist[lane])))
            .collect::<Vec<_>>()
            .join(" ");
        let a0_weighted_summary = (0..4)
            .map(|lane| format!("l{}{{{}}}", lane, fmt_hist(&a0_weighted_hist[lane])))
            .collect::<Vec<_>>()
            .join(" ");
        crate::xbox::emulator::debug_log(&format!(
            "[SPIDEY-100D-BBOX] #{} draw={} prim={} verts={} finite={} nonfinite={} on={} left={} right={} above={} below={} active_weight_lanes={} const_oor={} skin_bbox={} clip_bbox={} ndc_bbox={} screen_bbox={} a0_all=\"{}\" a0_weighted=\"{}\" hlsl_sample_pos=[{:.3},{:.3},{:.3}] hlsl_sample_clip=[{:.3},{:.3},{:.3},{:.3}] hlsl_sample_ndc=[{:.3},{:.3},{:.3}] hlsl_sample_screen=[{:.1},{:.1}] hlsl_sample_on={} hlsl_lanes=\"{}\"",
            n,
            draw_index,
            prim_type,
            vert_count,
            finite_vertices,
            nonfinite_vertices,
            on_screen_vertices,
            left_vertices,
            right_vertices,
            above_vertices,
            below_vertices,
            active_weight_lanes,
            const_oor,
            fmt_range3(skin_min, skin_max),
            fmt_range4(clip_min, clip_max),
            fmt_range3(ndc_min, ndc_max),
            fmt_range2(screen_min, screen_max),
            a0_hist_summary,
            a0_weighted_summary,
            hlsl_pos[0],
            hlsl_pos[1],
            hlsl_pos[2],
            hlsl_clip[0],
            hlsl_clip[1],
            hlsl_clip[2],
            hlsl_clip[3],
            hlsl_ndc[0],
            hlsl_ndc[1],
            hlsl_ndc[2],
            hlsl_screen[0],
            hlsl_screen[1],
            hlsl_on as u8,
            hlsl_lanes
        ));
        let mut triplet_candidates = Vec::new();
        for start in 0usize..=94 {
            let row0 = self.vs_constants[start];
            let row1 = self.vs_constants[start + 1];
            let row2 = self.vs_constants[start + 2];
            let nonzero = row0
                .iter()
                .chain(row1.iter())
                .chain(row2.iter())
                .any(|v| v.abs() > 1.0e-6);
            if !nonzero {
                continue;
            }
            let pos = [dot4(row0), dot4(row1), dot4(row2)];
            let (clip, _ndc, screen, on_screen) = project_to_xbox(pos);
            if !screen[0].is_finite() || !screen[1].is_finite() {
                continue;
            }
            let dx = screen[0] - 320.0;
            let dy = screen[1] - 240.0;
            let score = dx * dx + dy * dy;
            triplet_candidates.push((score, start, pos, screen, clip[3], on_screen));
        }
        triplet_candidates
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let triplet_summary = triplet_candidates
            .iter()
            .take(8)
            .map(|(_score, start, pos, screen, clip_w, on_screen)| {
                format!(
                    "c{}-{}-{}:pos=[{:.3},{:.3},{:.3}]:screen=[{:.1},{:.1}]:w={:.3}:on={}",
                    start,
                    start + 1,
                    start + 2,
                    pos[0],
                    pos[1],
                    pos[2],
                    screen[0],
                    screen[1],
                    clip_w,
                    *on_screen as u8
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let c17 = self.vs_constants[17];
        let c18 = self.vs_constants[18];
        let c19 = self.vs_constants[19];
        crate::xbox::emulator::debug_log(&format!(
            "[SPIDEY-100D-SKIN-EVAL] #{} draw={} sample={} verts={} v0=[{:.4},{:.4},{:.4},1] v5=[{:.5},{:.5},{:.5},{:.5}] v6=[{:.3},{:.3},{:.3},{:.3}] spidey_pos=[{:.3},{:.3},{:.3}] spidey_clip=[{:.3},{:.3},{:.3},{:.3}] spidey_ndc=[{:.3},{:.3},{:.3}] spidey_screen=[{:.1},{:.1}] spidey_on={} plain_pos=[{:.3},{:.3},{:.3}] plain_clip=[{:.3},{:.3},{:.3},{:.3}] plain_ndc=[{:.3},{:.3},{:.3}] plain_screen=[{:.1},{:.1}] plain_on={} triplet_best=\"{}\" spidey_lanes=\"{}\" plain_lanes=\"{}\" c17=[{:.3},{:.3},{:.3},{:.3}] c18=[{:.3},{:.3},{:.3},{:.3}] c19=[{:.3},{:.3},{:.3},{:.3}]",
            n,
            draw_index,
            sample_index,
            verts.len(),
            p[0],
            p[1],
            p[2],
            payload.blend_indices[0],
            payload.blend_indices[1],
            payload.blend_indices[2],
            payload.blend_indices[3],
            payload.blend_weights[0],
            payload.blend_weights[1],
            payload.blend_weights[2],
            payload.blend_weights[3],
            spidey_pos[0],
            spidey_pos[1],
            spidey_pos[2],
            spidey_clip[0],
            spidey_clip[1],
            spidey_clip[2],
            spidey_clip[3],
            spidey_ndc[0],
            spidey_ndc[1],
            spidey_ndc[2],
            spidey_screen[0],
            spidey_screen[1],
            spidey_on as u8,
            plain_pos[0],
            plain_pos[1],
            plain_pos[2],
            plain_clip[0],
            plain_clip[1],
            plain_clip[2],
            plain_clip[3],
            plain_ndc[0],
            plain_ndc[1],
            plain_ndc[2],
            plain_screen[0],
            plain_screen[1],
            plain_on as u8,
            triplet_summary,
            spidey_lanes,
            plain_lanes,
            c17[0],
            c17[1],
            c17[2],
            c17[3],
            c18[0],
            c18[1],
            c18[2],
            c18[3],
            c19[0],
            c19[1],
            c19[2],
            c19[3],
        ));
    }

    #[cfg(windows)]
    fn queue_peter_candidate_overlay(
        &mut self,
        draw_index: u64,
        draw_verts: &[NV2AVertex],
        source_skin: Option<&[VertexSkinPayload]>,
    ) {
        if !peter_candidate_overlay_enabled() {
            return;
        }
        let Some(skin) = source_skin else {
            return;
        };
        let count = draw_verts.len().min(skin.len());
        if count == 0 {
            return;
        }

        let bases = peter_candidate_overlay_bases();
        if bases.is_empty() {
            return;
        }

        let dot4 = |p: [f32; 4], row: [f32; 4]| -> f32 {
            p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
        };
        let const_index = |a0: f32, offset: i32| -> Option<usize> {
            let signed = (a0 + 0.001).floor() as i32;
            let index = signed + offset + 96;
            (0..192).contains(&index).then_some(index as usize)
        };
        let project = |constants: &[[f32; 4]; D3D11_VS_CONSTANT_COUNT],
                       pos: [f32; 3]|
         -> Option<(i32, i32)> {
            let p = [pos[0], pos[1], pos[2], 1.0f32];
            let clip = [
                dot4(p, constants[4]),
                dot4(p, constants[5]),
                dot4(p, constants[6]),
                dot4(p, constants[7]),
            ];
            if !clip.iter().all(|v| v.is_finite()) || clip[3] <= 1.0e-6 {
                return None;
            }
            let ndc_x = clip[0] / clip[3];
            let ndc_y = clip[1] / clip[3];
            if !ndc_x.is_finite() || !ndc_y.is_finite() {
                return None;
            }
            let scale = constants[CXBX_D3DVS_SCREENSPACE_SCALE_BASE];
            let offset = constants[CXBX_D3DVS_SCREENSPACE_OFFSET_BASE];
            let screen_x = ndc_x * scale[0] + offset[0];
            let screen_y = ndc_y * scale[1] + offset[1];
            if !screen_x.is_finite()
                || !screen_y.is_finite()
                || screen_x < 0.0
                || screen_x >= 640.0
                || screen_y < 0.0
                || screen_y >= 480.0
            {
                return None;
            }
            Some((screen_x.round() as i32, screen_y.round() as i32))
        };

        static OVERLAY_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let mut per_base_hits = vec![0usize; bases.len()];
        let mut queued = 0usize;
        for (base_slot, base_signed) in bases.iter().enumerate() {
            let bias = *base_signed as f32 - 3.0;
            let color = peter_candidate_overlay_color(base_slot);
            let mut screens = vec![None::<(i32, i32)>; count];
            for i in 0..count {
                let v = draw_verts[i];
                let payload = skin[i];
                let p = [v.x, v.y, v.z, if v.w.is_finite() { v.w } else { 1.0 }];
                let mut pos = [0.0f32; 3];
                let mut contributed = false;
                for lane in 0..4 {
                    let weight = payload.blend_weights[lane];
                    if weight.abs() <= 1.0e-6 {
                        continue;
                    }
                    let a0 = payload.blend_indices[lane] * 765.0 + bias;
                    for component in 0..3 {
                        if let Some(row_index) = const_index(a0, component as i32) {
                            pos[component] += dot4(p, self.vs_constants[row_index]) * weight;
                            contributed = true;
                        }
                    }
                }
                if !contributed {
                    continue;
                }
                if let Some((x, y)) = project(&self.vs_constants, pos) {
                    screens[i] = Some((x, y));
                    self.peter_candidate_overlay_points
                        .push(PeterCandidateOverlayPoint {
                            x,
                            y,
                            x2: x,
                            y2: y,
                            color,
                        });
                    per_base_hits[base_slot] += 1;
                    queued += 1;
                }
            }
            for tri in 0..(count / 3) {
                let i = tri * 3;
                let Some(a) = screens[i] else { continue };
                let Some(b) = screens[i + 1] else { continue };
                let Some(c) = screens[i + 2] else { continue };
                for (p0, p1) in [(a, b), (b, c), (c, a)] {
                    self.peter_candidate_overlay_points
                        .push(PeterCandidateOverlayPoint {
                            x: p0.0,
                            y: p0.1,
                            x2: p1.0,
                            y2: p1.1,
                            color,
                        });
                }
            }
        }

        let n = OVERLAY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            let summary = bases
                .iter()
                .zip(per_base_hits.iter())
                .enumerate()
                .map(|(slot, (base, hits))| {
                    format!(
                        "{}:{}=#{:06X}",
                        base,
                        hits,
                        peter_candidate_overlay_color(slot) & 0x00FF_FFFF
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-PETER-CANDIDATE-OVERLAY] #{} draw={} verts={} queued={} {}",
                n, draw_index, count, queued, summary
            ));
        }
    }

    #[cfg(windows)]
    fn apply_peter_candidate_overlay(&mut self, width: usize, height: usize) {
        if self.peter_candidate_overlay_points.is_empty() {
            return;
        }
        let mut drawn = 0usize;
        for point in self.peter_candidate_overlay_points.drain(..) {
            if point.x == point.x2 && point.y == point.y2 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let x = point.x + dx;
                        let y = point.y + dy;
                        if x < 0 || y < 0 {
                            continue;
                        }
                        let x = x as usize;
                        let y = y as usize;
                        if x >= width || y >= height {
                            continue;
                        }
                        self.readback_buf[y * width + x] = point.color;
                        drawn += 1;
                    }
                }
            } else {
                let mut x0 = point.x;
                let mut y0 = point.y;
                let x1 = point.x2;
                let y1 = point.y2;
                let dx = (x1 - x0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };
                let dy = -(y1 - y0).abs();
                let sy = if y0 < y1 { 1 } else { -1 };
                let mut err = dx + dy;
                for _ in 0..2048 {
                    if x0 >= 0 && y0 >= 0 {
                        let x = x0 as usize;
                        let y = y0 as usize;
                        if x < width && y < height {
                            self.readback_buf[y * width + x] = point.color;
                            drawn += 1;
                        }
                    }
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    let e2 = err * 2;
                    if e2 >= dy {
                        err += dy;
                        x0 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y0 += sy;
                    }
                }
            }
        }

        let bases = peter_candidate_overlay_bases();
        for (slot, _) in bases.iter().enumerate() {
            let color = peter_candidate_overlay_color(slot);
            let x0 = 8 + slot * 18;
            for y in 8..20usize.min(height) {
                for x in x0..(x0 + 14).min(width) {
                    self.readback_buf[y * width + x] = color;
                }
            }
        }

        static APPLY_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = APPLY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-PETER-CANDIDATE-OVERLAY-APPLY] #{} pixels={} bases={:?}",
                n, drawn, bases
            ));
        }
    }

    #[cfg(windows)]
    fn maybe_dump_peter_probe_json(
        &self,
        draw_index: u32,
        verts: &[NV2AVertex],
        skin: &[VertexSkinPayload],
    ) {
        let active_vs = self.active_nv2a_vs.unwrap_or(0);
        let skinned_payload = skin.iter().take(128).any(|payload| {
            payload.blend_indices.iter().any(|v| v.abs() > 0.0001)
                || payload.blend_weights.iter().any(|v| v.abs() > 0.0001)
        });
        if !peter_probe_json_enabled()
            || active_vs == 0
            || !skinned_payload
            || verts.len().min(skin.len()) < 100
        {
            return;
        }

        static PETER_PROBE_JSON_WRITES: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let probe_index = PETER_PROBE_JSON_WRITES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if probe_index >= peter_probe_json_limit() {
            return;
        }

        let vert_count = verts.len().min(skin.len());
        let sample_count = 20usize.min(vert_count);
        let step = (vert_count / sample_count).max(1);
        let mut sampled = Vec::with_capacity(sample_count);
        for sample in 0..sample_count {
            let index = (sample * step).min(vert_count - 1);
            let v = verts[index];
            let payload = skin[index];
            sampled.push((
                index,
                [v.x, v.y, v.z, if v.w.is_finite() { v.w } else { 1.0 }],
                payload.blend_indices,
                payload.blend_weights,
            ));
        }

        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!(
            "  \"source\": \"rustemu active_vs=0x{:08X} draw {}\",\n",
            active_vs, draw_index
        ));
        json.push_str(&format!("  \"probe_index\": {},\n", probe_index));
        json.push_str(&format!("  \"active_vs\": \"0x{:08X}\",\n", active_vs));
        json.push_str(&format!("  \"draw_index\": {},\n", draw_index));
        json.push_str(&format!("  \"draw_vertices\": {},\n", vert_count));
        let raw4_formula = spidey_skin_raw4_probe_enabled();
        let remap_formula = spidey_skin_a0_remap_probe_enabled();
        json.push_str(&format!(
            "  \"active_skin_formula\": \"{}\",\n",
            if raw4_formula {
                "raw4: a0=v5*4-92"
            } else {
                "cxbxr: a0=v5*c[-95].x+c[-95].y"
            }
        ));
        json.push_str(&format!(
            "  \"active_const_index\": \"{}\",\n",
            if remap_formula {
                "relative_remap"
            } else {
                "direct_signed_plus_96"
            }
        ));
        json.push_str("  \"texture_state\": {\n");
        json.push_str(&format!(
            "    \"has_active_texture\": {},\n\
             \"v_flip\": {},\n\
             \"handle\": {},\n\
             \"guest_raw\": \"0x{:08X}\",\n\
             \"guest_norm\": \"0x{:08X}\",\n\
             \"data\": \"0x{:08X}\",\n\
             \"size\": [{}, {}],\n\
             \"format_code\": \"0x{:08X}\",\n\
             \"format_name\": \"{}\",\n\
             \"guest_format_code\": \"0x{:08X}\",\n\
             \"guest_format_name\": \"{}\",\n\
             \"source\": \"{}\",\n\
             \"byte_len\": {},\n\
             \"srv_bound\": {},\n\
             \"alpha_min\": {},\n\
             \"alpha_max\": {},\n\
             \"alpha_nonzero_sample\": {},\n\
             \"rgb_nonzero_sample\": {},\n\
             \"sample_count\": {}\n",
            self.has_active_texture,
            self.texture_v_flip,
            self.active_tex_handle,
            self.active_tex_guest_raw,
            self.active_tex_guest_norm,
            self.active_tex_data,
            self.active_tex_width,
            self.active_tex_height,
            self.active_tex_format_code,
            self.active_texture_format_name(),
            self.active_tex_guest_format_code,
            crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
            self.active_tex_source,
            self.active_tex_byte_len,
            self.active_tex_srv_bound,
            self.active_tex_alpha_min,
            self.active_tex_alpha_max,
            self.active_tex_alpha_nonzero_sample,
            self.active_tex_rgb_nonzero_sample,
            self.active_tex_sample_count
        ));
        json.push_str("  },\n");
        let s0 = &self.texture_stage_states[0];
        json.push_str(&format!(
            "  \"render_state\": {{\n\
             \"alpha_blend\": {},\n\
             \"src_blend\": \"{:?}\",\n\
             \"dest_blend\": \"{:?}\",\n\
             \"blend_op\": {},\n\
             \"alpha_test\": {},\n\
             \"alpha_ref\": {},\n\
             \"alpha_func\": \"{:?}\",\n\
             \"color_write\": {},\n\
             \"z_enable\": {},\n\
             \"z_write\": {},\n\
             \"z_func\": \"{:?}\",\n\
             \"texture_factor\": \"0x{:08X}\",\n\
             \"cull_mode\": \"{:?}\",\n\
             \"fill_mode\": \"{:?}\",\n\
             \"ps_combiner\": \"0x{:08X}\",\n\
             \"ps_rgb0\": \"0x{:08X}\",\n\
             \"ps_alpha0\": \"0x{:08X}\",\n\
             \"ps_texmodes\": \"0x{:08X}\",\n\
             \"tss0_color_op\": \"0x{:08X}\",\n\
             \"tss0_color_arg1\": \"0x{:08X}\",\n\
             \"tss0_color_arg2\": \"0x{:08X}\",\n\
             \"tss0_alpha_op\": \"0x{:08X}\",\n\
             \"tss0_alpha_arg1\": \"0x{:08X}\",\n\
             \"tss0_alpha_arg2\": \"0x{:08X}\",\n\
             \"tss0_address_u\": \"0x{:08X}\",\n\
             \"tss0_address_v\": \"0x{:08X}\"\n\
             }},\n",
            self.render_state.alpha_blend_enable,
            self.render_state.src_blend,
            self.render_state.dest_blend,
            self.render_state.blend_op,
            self.render_state.alpha_test_enable,
            self.render_state.alpha_ref,
            self.render_state.alpha_func,
            self.render_state.color_write_enable,
            self.render_state.z_enable,
            self.render_state.z_write_enable,
            self.render_state.z_func,
            self.render_state.texture_factor,
            self.render_state.cull_mode,
            self.render_state.fill_mode,
            self.ps_render_states[53],
            self.ps_render_states[34],
            self.ps_render_states[0],
            self.ps_render_states[136],
            s0[super::X_D3DTSS_COLOROP],
            s0[super::X_D3DTSS_COLORARG1],
            s0[super::X_D3DTSS_COLORARG2],
            s0[super::X_D3DTSS_ALPHAOP],
            s0[super::X_D3DTSS_ALPHAARG1],
            s0[super::X_D3DTSS_ALPHAARG2],
            s0[super::X_D3DTSS_ADDRESSU],
            s0[super::X_D3DTSS_ADDRESSV]
        ));
        json.push_str("  \"recent_draws\": [\n");
        let recent_count = self.recent_draw_seen.min(RECENT_DRAW_COUNT);
        let start = self.recent_draw_seen.saturating_sub(recent_count);
        for i in 0..recent_count {
            let logical = start + i;
            let draw = self.recent_draws[logical % RECENT_DRAW_COUNT];
            let comma = if i + 1 < recent_count { "," } else { "" };
            json.push_str(&format!(
                "    {{\"draw_index\":{},\"verts\":{},\"prim\":{},\"bbox\":[{:.3},{:.3},{:.3},{:.3}],\"uv\":[{:.6},{:.6},{:.6},{:.6}],\"first_color\":\"0x{:08X}\",\"alpha\":[{},{}],\"tex_handle\":{},\"tex_data\":\"0x{:08X}\",\"tex_size\":[{},{}],\"tex_format\":\"{}\",\"tex_source\":\"{}\",\"srv_bound\":{},\"active_vs\":\"0x{:08X}\",\"blend\":{},\"z_enable\":{},\"z_write\":{},\"color_write\":{},\"alpha_test\":{},\"alpha_ref\":{},\"rt_key\":\"0x{:08X}\"}}{}\n",
                draw.draw_index,
                draw.verts,
                draw.prim_type,
                draw.min_x,
                draw.min_y,
                draw.max_x,
                draw.max_y,
                draw.min_u,
                draw.min_v,
                draw.max_u,
                draw.max_v,
                draw.first_color,
                draw.min_alpha,
                draw.max_alpha,
                draw.tex_handle,
                draw.tex_data,
                draw.tex_width,
                draw.tex_height,
                crate::xbox::gpu::texture_format::format_name(draw.tex_format),
                draw.tex_source,
                draw.tex_srv_bound,
                draw.active_vs,
                draw.blend,
                draw.z_enable,
                draw.z_write,
                draw.color_write,
                draw.alpha_test,
                draw.alpha_ref,
                draw.rt_key,
                comma
            ));
        }
        json.push_str("  ],\n");
        json.push_str("  \"viewport_scale\": [320.0, -240.0],\n");
        json.push_str("  \"viewport_offset\": [320.0, 240.0],\n");
        json.push_str(
            "  \"ps2_reference_targets\": {\n\
             \"head\": [340, 210],\n\
             \"chest\": [340, 265],\n\
             \"hips\": [345, 295],\n\
             \"feet\": [345, 370],\n\
             \"right_hand\": [400, 265],\n\
             \"left_hand\": [320, 260],\n\
             \"bbox\": [[305, 210], [410, 380]]\n\
             },\n",
        );

        json.push_str("  \"c\": [\n");
        for i in 0..192usize {
            let row = self.vs_constants.get(i).copied().unwrap_or([0.0; 4]);
            let comma = if i + 1 < 192 { "," } else { "" };
            json.push_str(&format!(
                "    [{:.6}, {:.6}, {:.6}, {:.6}]{}\n",
                row[0], row[1], row[2], row[3], comma
            ));
        }
        json.push_str("  ],\n");

        let recent_uploads = if peter_probe_json_enabled() {
            recent_vs_constant_uploads()
                .lock()
                .map(|uploads| uploads.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        json.push_str("  \"recent_vs_constant_uploads\": [\n");
        for (upload_index, upload) in recent_uploads.iter().enumerate() {
            let upload_comma = if upload_index + 1 < recent_uploads.len() {
                ","
            } else {
                ""
            };
            json.push_str(&format!(
                "    {{\n\
                 \"seq\": {},\n\
                 \"reg\": {},\n\
                 \"rows\": {},\n\
                 \"end_reg\": {},\n\
                 \"preview\": [\n",
                upload.seq,
                upload.reg,
                upload.rows,
                upload.reg.saturating_add(upload.rows)
            ));
            for (row_index, row) in upload.preview.iter().enumerate() {
                let row_comma = if row_index + 1 < upload.preview.len() {
                    ","
                } else {
                    ""
                };
                json.push_str(&format!(
                    "      [{:.6}, {:.6}, {:.6}, {:.6}]{}\n",
                    row[0], row[1], row[2], row[3], row_comma
                ));
            }
            json.push_str(&format!("    ]\n    }}{}\n", upload_comma));
        }
        json.push_str("  ],\n");

        let dot4_for = |p: [f32; 4], row: [f32; 4]| -> f32 {
            p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
        };
        let a0_key = |a0: f32| -> i32 { (a0 + 0.001).floor() as i32 };
        let direct_const_index = |a0: f32, offset: i32| -> Option<usize> {
            let i = a0_key(a0) + offset + 96;
            (0..192).contains(&i).then_some(i as usize)
        };
        let remapped_const_index = |a0: f32, offset: i32| -> Option<usize> {
            let raw = a0_key(a0);
            let skin_numerator = raw + 92;
            if skin_numerator >= 0 && skin_numerator % 4 == 0 && (0..=2).contains(&offset) {
                let bone = skin_numerator / 4;
                if (0..48).contains(&bone) {
                    return Some((17 + bone * 3 + offset).clamp(0, 191) as usize);
                }
            }
            direct_const_index(a0, offset)
        };
        let const_index = |a0: f32, offset: i32| -> Option<usize> {
            if remap_formula {
                remapped_const_index(a0, offset)
            } else {
                direct_const_index(a0, offset)
            }
        };
        let project_to_xbox = |pos: [f32; 3]| -> ([f32; 4], [f32; 3], [f32; 2], bool, bool) {
            let p = [pos[0], pos[1], pos[2], 1.0f32];
            let dot_row = |row: [f32; 4]| -> f32 {
                p[0] * row[0] + p[1] * row[1] + p[2] * row[2] + p[3] * row[3]
            };
            let clip = [
                dot_row(self.vs_constants[4]),
                dot_row(self.vs_constants[5]),
                dot_row(self.vs_constants[6]),
                dot_row(self.vs_constants[7]),
            ];
            let inv_w = if clip[3].abs() > 1.0e-6 {
                1.0 / clip[3]
            } else {
                f32::NAN
            };
            let ndc = [clip[0] * inv_w, clip[1] * inv_w, clip[2] * inv_w];
            let screen_scale = self.vs_constants[CXBX_D3DVS_SCREENSPACE_SCALE_BASE];
            let screen_offset = self.vs_constants[CXBX_D3DVS_SCREENSPACE_OFFSET_BASE];
            let screen = [
                ndc[0] * screen_scale[0] + screen_offset[0],
                ndc[1] * screen_scale[1] + screen_offset[1],
            ];
            let finite = pos.iter().all(|v| v.is_finite())
                && clip.iter().all(|v| v.is_finite())
                && ndc.iter().all(|v| v.is_finite())
                && screen.iter().all(|v| v.is_finite());
            let near_camera = finite && clip[3] > 1.0e-6;
            let inside_viewport = near_camera
                && (0.0..=640.0).contains(&screen[0])
                && (0.0..=480.0).contains(&screen[1]);
            (clip, ndc, screen, near_camera, inside_viewport)
        };

        #[derive(Clone)]
        struct CorrelatedVertex {
            index: usize,
            v0: [f32; 4],
            v5: [f32; 4],
            v6: [f32; 4],
            a0: [f32; 4],
            rows: [[i32; 3]; 4],
            row_values: [[[f32; 4]; 3]; 4],
            pos: [f32; 3],
            clip: [f32; 4],
            ndc: [f32; 3],
            screen: [f32; 2],
            near_camera: bool,
            inside_viewport: bool,
            category: &'static str,
        }

        let mut correlated = Vec::with_capacity(vert_count);
        let mut on_hist = std::collections::BTreeMap::<i32, usize>::new();
        let mut off_hist = std::collections::BTreeMap::<i32, usize>::new();
        let mut extreme_hist = std::collections::BTreeMap::<i32, usize>::new();
        let mut row_hist = std::collections::BTreeMap::<i32, usize>::new();
        let mut row_oor = 0usize;

        for i in 0..vert_count {
            let v = verts[i];
            let payload = skin[i];
            let p = [v.x, v.y, v.z, if v.w.is_finite() { v.w } else { 1.0 }];
            let mut pos = [0.0f32; 3];
            let mut a0 = [0.0f32; 4];
            let mut rows = [[-1i32; 3]; 4];
            let mut row_values = [[[0.0f32; 4]; 3]; 4];
            for lane in 0..4 {
                a0[lane] = if raw4_formula {
                    payload.blend_indices[lane] * 4.0 - 92.0
                } else {
                    payload.blend_indices[lane] * self.vs_constants[1][0] + self.vs_constants[1][1]
                };
                let weight = payload.blend_weights[lane];
                if weight.abs() <= 1.0e-6 {
                    continue;
                }
                for component in 0..3 {
                    if let Some(row_index) = const_index(a0[lane], component as i32) {
                        rows[lane][component] = row_index as i32;
                        row_values[lane][component] = self.vs_constants[row_index];
                        *row_hist.entry(row_index as i32).or_insert(0) += 1;
                        pos[component] += dot4_for(p, self.vs_constants[row_index]) * weight;
                    } else {
                        row_oor += 1;
                    }
                }
            }
            let (clip, ndc, screen, near_camera, inside_viewport) = project_to_xbox(pos);
            let finite = pos.iter().all(|v| v.is_finite())
                && clip.iter().all(|v| v.is_finite())
                && ndc.iter().all(|v| v.is_finite())
                && screen.iter().all(|v| v.is_finite());
            let extreme = !finite
                || screen[0] < -640.0
                || screen[0] > 1280.0
                || screen[1] < -480.0
                || screen[1] > 960.0;
            let category = if inside_viewport {
                "on"
            } else if extreme {
                "extreme"
            } else {
                "off"
            };
            let hist = if inside_viewport {
                &mut on_hist
            } else if extreme {
                &mut extreme_hist
            } else {
                &mut off_hist
            };
            for lane in 0..4 {
                if payload.blend_weights[lane].abs() > 1.0e-6 {
                    *hist.entry(a0_key(a0[lane])).or_insert(0) += 1;
                }
            }
            correlated.push(CorrelatedVertex {
                index: i,
                v0: p,
                v5: payload.blend_indices,
                v6: payload.blend_weights,
                a0,
                rows,
                row_values,
                pos,
                clip,
                ndc,
                screen,
                near_camera,
                inside_viewport,
                category,
            });
        }

        let fmt_hist_json = |map: &std::collections::BTreeMap<i32, usize>| -> String {
            let mut pairs = map
                .iter()
                .map(|(key, count)| (*count, *key))
                .collect::<Vec<_>>();
            pairs.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
            pairs
                .iter()
                .take(20)
                .map(|(count, key)| format!("{{\"value\":{},\"count\":{}}}", key, count))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let on_count = correlated.iter().filter(|v| v.category == "on").count();
        let near_count = correlated.iter().filter(|v| v.near_camera).count();
        let off_count = correlated.iter().filter(|v| v.category == "off").count();
        let extreme_count = correlated
            .iter()
            .filter(|v| v.category == "extreme")
            .count();
        struct BaseSweep {
            base_signed: i32,
            bias: f32,
            on: usize,
            near: usize,
            off: usize,
            extreme: usize,
            row_oor: usize,
        }
        let mut base_sweep = Vec::<BaseSweep>::new();
        for base_signed in -96..=95 {
            let bias = base_signed as f32 - 3.0;
            let mut on = 0usize;
            let mut near = 0usize;
            let mut off = 0usize;
            let mut extreme = 0usize;
            let mut row_oor = 0usize;
            for i in 0..vert_count {
                let v = verts[i];
                let payload = skin[i];
                let p = [v.x, v.y, v.z, if v.w.is_finite() { v.w } else { 1.0 }];
                let mut pos = [0.0f32; 3];
                for lane in 0..4 {
                    let weight = payload.blend_weights[lane];
                    if weight.abs() <= 1.0e-6 {
                        continue;
                    }
                    let a0 = payload.blend_indices[lane] * 765.0 + bias;
                    for component in 0..3 {
                        if let Some(row_index) = direct_const_index(a0, component as i32) {
                            pos[component] += dot4_for(p, self.vs_constants[row_index]) * weight;
                        } else {
                            row_oor += 1;
                        }
                    }
                }
                let (_, _, screen, near_camera, inside_viewport) = project_to_xbox(pos);
                let finite =
                    pos.iter().all(|v| v.is_finite()) && screen.iter().all(|v| v.is_finite());
                if inside_viewport {
                    on += 1;
                } else if near_camera {
                    near += 1;
                    off += 1;
                } else if !finite
                    || screen[0] < -640.0
                    || screen[0] > 1280.0
                    || screen[1] < -480.0
                    || screen[1] > 960.0
                {
                    extreme += 1;
                } else {
                    off += 1;
                }
            }
            base_sweep.push(BaseSweep {
                base_signed,
                bias,
                on,
                near,
                off,
                extreme,
                row_oor,
            });
        }
        base_sweep.sort_by(|a, b| {
            b.on.cmp(&a.on)
                .then_with(|| b.near.cmp(&a.near))
                .then_with(|| a.extreme.cmp(&b.extreme))
                .then_with(|| a.row_oor.cmp(&b.row_oor))
        });
        let base_sweep_json = base_sweep
            .iter()
            .take(24)
            .map(|entry| {
                format!(
                    "{{\"base_signed\":{},\"base_host\":{},\"scale\":765.0,\"bias\":{:.3},\"on\":{},\"near\":{},\"off\":{},\"extreme\":{},\"row_oor\":{}}}",
                    entry.base_signed,
                    entry.base_signed + 96,
                    entry.bias,
                    entry.on,
                    entry.near,
                    entry.off,
                    entry.extreme,
                    entry.row_oor
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        json.push_str(&format!(
            "  \"vertex_bone_correlation\": {{\n\
             \"on_screen\": {},\n\
             \"near_camera\": {},\n\
             \"off_screen\": {},\n\
             \"extreme\": {},\n\
             \"row_oor\": {},\n\
             \"a0_on_top\": [{}],\n\
             \"a0_off_top\": [{}],\n\
             \"a0_extreme_top\": [{}],\n\
             \"rows_top\": [{}]\n\
             }},\n",
            on_count,
            near_count,
            off_count,
            extreme_count,
            row_oor,
            fmt_hist_json(&on_hist),
            fmt_hist_json(&off_hist),
            fmt_hist_json(&extreme_hist),
            fmt_hist_json(&row_hist)
        ));
        json.push_str(&format!("  \"base_sweep_top\": [{}],\n", base_sweep_json));

        let fmt_row_values_json = |rows: &[[[f32; 4]; 3]; 4]| -> String {
            let mut lanes = Vec::with_capacity(4);
            for lane in rows {
                let mut components = Vec::with_capacity(3);
                for row in lane {
                    components.push(format!(
                        "[{:.6},{:.6},{:.6},{:.6}]",
                        row[0], row[1], row[2], row[3]
                    ));
                }
                lanes.push(format!("[{}]", components.join(",")));
            }
            format!("[{}]", lanes.join(","))
        };

        let mut selected_indices = Vec::<usize>::new();
        for wanted in ["on", "off", "extreme"] {
            selected_indices.extend(
                correlated
                    .iter()
                    .enumerate()
                    .filter(|(_, vertex)| vertex.category == wanted)
                    .take(24)
                    .map(|(index, _)| index),
            );
        }
        if selected_indices.is_empty() {
            selected_indices.extend(0..correlated.len().min(64));
        }
        selected_indices.sort_unstable();
        selected_indices.dedup();

        json.push_str("  \"projected_vertices\": [\n");
        for (out_index, correlated_index) in selected_indices.iter().enumerate() {
            let vertex = &correlated[*correlated_index];
            let comma = if out_index + 1 < selected_indices.len() {
                ","
            } else {
                ""
            };
            json.push_str(&format!(
                "    {{\"i\":{},\"category\":\"{}\",\"near_camera\":{},\"inside_viewport\":{},\"screen\":[{:.3},{:.3}],\"ndc\":[{:.6},{:.6},{:.6}],\"clip\":[{:.6},{:.6},{:.6},{:.6}],\"pos\":[{:.6},{:.6},{:.6}],\"v0\":[{:.6},{:.6},{:.6},{:.6}],\"v5\":[{:.6},{:.6},{:.6},{:.6}],\"v6\":[{:.6},{:.6},{:.6},{:.6}],\"a0\":[{:.3},{:.3},{:.3},{:.3}],\"rows\":[[{},{},{}],[{},{},{}],[{},{},{}],[{},{},{}]],\"row_values\":{}}}{}\n",
                vertex.index,
                vertex.category,
                vertex.near_camera,
                vertex.inside_viewport,
                vertex.screen[0],
                vertex.screen[1],
                vertex.ndc[0],
                vertex.ndc[1],
                vertex.ndc[2],
                vertex.clip[0],
                vertex.clip[1],
                vertex.clip[2],
                vertex.clip[3],
                vertex.pos[0],
                vertex.pos[1],
                vertex.pos[2],
                vertex.v0[0],
                vertex.v0[1],
                vertex.v0[2],
                vertex.v0[3],
                vertex.v5[0],
                vertex.v5[1],
                vertex.v5[2],
                vertex.v5[3],
                vertex.v6[0],
                vertex.v6[1],
                vertex.v6[2],
                vertex.v6[3],
                vertex.a0[0],
                vertex.a0[1],
                vertex.a0[2],
                vertex.a0[3],
                vertex.rows[0][0],
                vertex.rows[0][1],
                vertex.rows[0][2],
                vertex.rows[1][0],
                vertex.rows[1][1],
                vertex.rows[1][2],
                vertex.rows[2][0],
                vertex.rows[2][1],
                vertex.rows[2][2],
                vertex.rows[3][0],
                vertex.rows[3][1],
                vertex.rows[3][2],
                fmt_row_values_json(&vertex.row_values),
                comma
            ));
        }
        json.push_str("  ],\n");

        json.push_str("  \"vertices\": [\n");
        for (sample, (index, v0, v5, v6)) in sampled.iter().enumerate() {
            let comma = if sample + 1 < sampled.len() { "," } else { "" };
            json.push_str(&format!(
                "    {{\n\
                 \"sample_index\": {},\n\
                 \"source_vertex_index\": {},\n\
                 \"v0\": [{:.6}, {:.6}, {:.6}, {:.6}],\n\
                 \"v5\": [{:.6}, {:.6}, {:.6}, {:.6}],\n\
                 \"v6\": [{:.6}, {:.6}, {:.6}, {:.6}],\n\
                 \"target_screen\": null\n\
                 }}{}\n",
                sample,
                index,
                v0[0],
                v0[1],
                v0[2],
                v0[3],
                v5[0],
                v5[1],
                v5[2],
                v5[3],
                v6[0],
                v6[1],
                v6[2],
                v6[3],
                comma
            ));
        }
        json.push_str("  ]\n");
        json.push_str("}\n");

        let path = format!(
            r"./peter_probe_{:02}_vs_{:08X}_draw_{:06}.json",
            probe_index, active_vs, draw_index
        );
        let latest_path = r"./peter_probe.json";
        let write_result =
            std::fs::write(&path, &json).and_then(|_| std::fs::write(latest_path, &json));
        match write_result {
            Ok(()) => crate::xbox::emulator::debug_log(&format!(
                "[PETER-PROBE-JSON] wrote {} draw={} verts={} samples={} bytes={}",
                path,
                draw_index,
                vert_count,
                sampled.len(),
                json.len()
            )),
            Err(err) => crate::xbox::emulator::debug_log(&format!(
                "[PETER-PROBE-JSON] failed to write {}: {}",
                path, err
            )),
        }
    }

    #[cfg(windows)]
    fn record_recent_draw(
        &mut self,
        draw_index: u32,
        verts: &[NV2AVertex],
        prim_type: i32,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        min_u: f32,
        min_v: f32,
        max_u: f32,
        max_v: f32,
    ) {
        let (first_color, min_alpha, max_alpha) = if verts.is_empty() {
            (0, 0, 0)
        } else {
            let (min_alpha, max_alpha) = verts.iter().fold((u8::MAX, u8::MIN), |(lo, hi), v| {
                let a = (v.color >> 24) as u8;
                (lo.min(a), hi.max(a))
            });
            (verts[0].color, min_alpha, max_alpha)
        };

        let summary = RecentDrawSummary {
            draw_index,
            verts: verts.len().min(u32::MAX as usize) as u32,
            prim_type,
            min_x,
            min_y,
            max_x,
            max_y,
            min_u,
            min_v,
            max_u,
            max_v,
            first_color,
            min_alpha,
            max_alpha,
            tex_handle: self.active_tex_handle,
            tex_guest_raw: self.active_tex_guest_raw,
            tex_guest_norm: self.active_tex_guest_norm,
            tex_data: self.active_tex_data,
            tex_width: self.active_tex_width,
            tex_height: self.active_tex_height,
            tex_format: self.active_tex_format_code,
            tex_guest_format: self.active_tex_guest_format_code,
            tex_source: self.active_tex_source,
            tex_srv_bound: self.active_tex_srv_bound,
            active_vs: self.active_nv2a_vs.unwrap_or(0),
            blend: self.render_state.alpha_blend_enable,
            z_enable: self.render_state.z_enable != 0,
            z_write: self.render_state.z_write_enable,
            color_write: self.render_state.color_write_enable,
            alpha_test: self.render_state.alpha_test_enable,
            alpha_ref: self.render_state.alpha_ref as u32,
            rt_key: self
                .debug_active_render_target()
                .map(|(key, _, _, _, _)| key)
                .unwrap_or(self.active_rt_key),
        };
        self.recent_draws[self.recent_draw_cursor] = summary;
        self.recent_draw_cursor = (self.recent_draw_cursor + 1) % RECENT_DRAW_COUNT;
        self.recent_draw_seen = self
            .recent_draw_seen
            .saturating_add(1)
            .min(RECENT_DRAW_COUNT);
    }

    #[cfg(windows)]
    fn dump_recent_draws_before_circle(&self, circle_draw_index: u32) {
        static RECENT_DUMPED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if RECENT_DUMPED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let count = self.recent_draw_seen.min(RECENT_DRAW_COUNT);
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-RECENT-BEFORE-CIRCLE-BEGIN] circle_draw={} count={} cursor={}",
            circle_draw_index, count, self.recent_draw_cursor
        ));
        if count == 0 {
            return;
        }

        let start = (self.recent_draw_cursor + RECENT_DRAW_COUNT - count) % RECENT_DRAW_COUNT;
        for i in 0..count {
            let idx = (start + i) % RECENT_DRAW_COUNT;
            let d = self.recent_draws[idx];
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RECENT-BEFORE-CIRCLE] rel={} draw={} verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3},{:.3}..{:.3},{:.3}] color=0x{:08X} alpha=[{}..{}] tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:08X}/{} guest_fmt=0x{:02X}/{} size={}x{} source={} srv={} vs=0x{:08X} blend={} z={} zwrite={} colorwrite=0x{:X} atest={} aref={} rt=0x{:08X}",
                i,
                d.draw_index,
                d.verts,
                d.prim_type,
                d.min_x,
                d.min_y,
                d.max_x,
                d.max_y,
                d.min_u,
                d.min_v,
                d.max_u,
                d.max_v,
                d.first_color,
                d.min_alpha,
                d.max_alpha,
                d.tex_handle,
                d.tex_guest_raw,
                d.tex_guest_norm,
                d.tex_data,
                d.tex_format,
                if d.tex_format == 0xFFFF_FFFE {
                    "ARG32(decoded)"
                } else {
                    crate::xbox::gpu::texture_format::format_name(d.tex_format)
                },
                d.tex_guest_format,
                crate::xbox::gpu::texture_format::format_name(d.tex_guest_format),
                d.tex_width,
                d.tex_height,
                d.tex_source,
                d.tex_srv_bound,
                d.active_vs,
                d.blend,
                d.z_enable,
                d.z_write,
                d.color_write,
                d.alpha_test,
                d.alpha_ref,
                d.rt_key
            ));
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-RECENT-BEFORE-CIRCLE-END] circle_draw={} count={}",
            circle_draw_index, count
        ));
    }

    #[cfg(windows)]
    fn log_spidey_select_circle_vs_probe(
        &self,
        draw_index: u32,
        prim_type: i32,
        vertex_count: usize,
    ) {
        if !self.active_texture_is_spidey_select_circle() {
            return;
        }
        static CIRCLE_VS_PROBE_LOGGED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if CIRCLE_VS_PROBE_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        let active_vs = self.active_nv2a_vs.unwrap_or(0);
        let guest_vs_raw =
            crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(std::sync::atomic::Ordering::Relaxed);
        let stream_stride =
            crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.load(std::sync::atomic::Ordering::Relaxed);
        let shader_hle_path = (0x0000_1000..0x0100_0000).contains(&guest_vs_raw);
        let guest_fvf = if !shader_hle_path && guest_vs_raw != 0 && (guest_vs_raw & 1) != 0 {
            guest_vs_raw
        } else {
            0
        };
        let fvf_xyzrhw = (guest_fvf & crate::xbox::gpu::fvf_decode::fvf::POSITION_MASK)
            == crate::xbox::gpu::fvf_decode::fvf::XYZRHW;
        let host_stride = std::mem::size_of::<D3D11DrawVertex>();
        let input_layout_span = 92usize; // last element offset 76 + float4 TEXCOORD1.
        let using_nv2a_layout = active_vs != 0 && self.nv2a_layout_cache.contains_key(&active_vs);
        let using_nv2a_vs = active_vs != 0 && self.nv2a_vs_cache.contains_key(&active_vs);
        let hlsl = if active_vs != 0 {
            crate::xbox::gpu::nv2a_vsh::hlsl_for_handle(active_vs).unwrap_or_default()
        } else {
            String::new()
        };
        let hlsl_path = if hlsl.is_empty() {
            String::from("none")
        } else {
            let path = format!(r"./spidey_circle_vs_{active_vs:08X}.hlsl");
            if let Err(e) = std::fs::write(&path, &hlsl) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CIRCLE-VS] failed to write {}: {}",
                    path, e
                ));
            }
            path
        };
        let mut escaped = hlsl.replace("\r\n", "\n").replace('\n', "\\n");
        let truncated = escaped.len() > 12000;
        if truncated {
            escaped.truncate(12000);
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-CIRCLE-VS] draw={} verts={} prim={} active_vs=0x{:08X} guest_vs_raw=0x{:08X} guest_fvf=0x{:08X} fvf_xyzrhw={} stream_stride={} host_stride={} input_layout_span={} using_nv2a_vs={} using_nv2a_layout={} hlsl_len={} hlsl_truncated={} hlsl_path={} hlsl=\"{}\"",
            draw_index,
            vertex_count,
            prim_type,
            active_vs,
            guest_vs_raw,
            guest_fvf,
            fvf_xyzrhw,
            stream_stride,
            host_stride,
            input_layout_span,
            using_nv2a_vs,
            using_nv2a_layout,
            hlsl.len(),
            truncated,
            hlsl_path,
            escaped
        ));
    }

    #[cfg(windows)]
    fn capture_spidey_select_circle_after_draw(
        &mut self,
        draw_index: u32,
        prim_type: i32,
        verts: &[NV2AVertex],
    ) {
        if !self.active_texture_is_spidey_select_circle() {
            return;
        }
        static CIRCLE_AFTER_DRAW_CAPTURED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if CIRCLE_AFTER_DRAW_CAPTURED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        let (min_x, min_y, max_x, max_y) = if verts.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            verts.iter().fold(
                (
                    f32::INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::NEG_INFINITY,
                ),
                |(min_x, min_y, max_x, max_y), v| {
                    (
                        min_x.min(v.x),
                        min_y.min(v.y),
                        max_x.max(v.x),
                        max_y.max(v.y),
                    )
                },
            )
        };
        let width = self.width.max(0) as usize;
        let height = self.height.max(0) as usize;
        let rt_key = self.active_rt_key;
        let pixels = self.readback_framebuffer().to_vec();
        let path = r"./spidey_select_circle_after_draw.bmp";
        write_d3d11_readback_bmp(path, width as u32, height as u32, &pixels);
        let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
            sampled_argb_stats(&pixels);
        let px = |x: usize, y: usize| -> u32 {
            if width == 0 || height == 0 {
                return 0;
            }
            pixels
                .get(
                    y.min(height - 1)
                        .saturating_mul(width)
                        .saturating_add(x.min(width - 1)),
                )
                .copied()
                .unwrap_or(0)
        };
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-CIRCLE-READBACK] draw={} prim={} verts={} bbox=[{:.1},{:.1}..{:.1},{:.1}] tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} rt=0x{:08X} out={}x{} nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} px_left_tile_center=0x{:08X} px_screen_center=0x{:08X} px_right_tile_center=0x{:08X} px_right_tile_edge=0x{:08X} path={}",
            draw_index,
            prim_type,
            verts.len(),
            min_x,
            min_y,
            max_x,
            max_y,
            self.active_tex_guest_raw,
            self.active_tex_guest_norm,
            self.active_tex_data,
            self.active_tex_guest_format_code,
            crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
            rt_key,
            width,
            height,
            sample_nonzero,
            sample_alpha,
            sample_rgb,
            first_px,
            mid_px,
            px(192, 240),
            px(320, 240),
            px(448, 240),
            px(575, 240),
            path
        ));
    }

    #[cfg(windows)]
    fn maybe_capture_doom_post_draw_rt(
        &mut self,
        draw_index: u32,
        prim_type: i32,
        draw_verts: &[NV2AVertex],
    ) {
        if !doom_d3d11_rt_readback_enabled() {
            return;
        }
        if prim_type != super::NV097_TRIANGLE_STRIP
            || draw_verts.len() != 4
            || self.active_tex_guest_norm != 0x03C0_01A0
            || self.active_tex_data != 0x0400_0000
            || self.active_tex_width != 512
            || self.active_tex_height != 256
            || self.active_tex_guest_format_code
                != crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
            || !self.has_active_texture
            || !self.active_tex_srv_bound
        {
            return;
        }

        let (min_x, min_y, max_x, max_y, min_u, min_v, max_u, max_v) = draw_verts.iter().fold(
            (
                f32::INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
            ),
            |(min_x, min_y, max_x, max_y, min_u, min_v, max_u, max_v), v| {
                (
                    min_x.min(v.x),
                    min_y.min(v.y),
                    max_x.max(v.x),
                    max_y.max(v.y),
                    min_u.min(v.u),
                    min_v.min(v.v),
                    max_u.max(v.u),
                    max_v.max(v.v),
                )
            },
        );
        let looks_like_doom_present_quad = min_x >= 30.0
            && min_x <= 50.0
            && max_x >= 590.0
            && max_x <= 610.0
            && min_y >= 20.0
            && min_y <= 40.0
            && max_y >= 440.0
            && max_y <= 460.0
            && min_u.abs() <= 0.01
            && min_v.abs() <= 0.01
            && (max_u - 0.625).abs() <= 0.01
            && (max_v - 0.78125).abs() <= 0.01;
        if !looks_like_doom_present_quad {
            return;
        }

        static DOOM_RT_AFTER_DRAW_MATCH: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        static DOOM_RT_AFTER_DRAW_MATCHES: std::sync::OnceLock<Vec<u32>> =
            std::sync::OnceLock::new();
        let match_seq = DOOM_RT_AFTER_DRAW_MATCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let capture_matches = DOOM_RT_AFTER_DRAW_MATCHES
            .get_or_init(|| parse_u32_list_env("RUSTEMU_DOOM_D3D11_RT_READBACK_MATCHES"));
        let should_capture = if capture_matches.is_empty() {
            match_seq == 0
        } else {
            capture_matches.contains(&match_seq)
        };
        if !should_capture {
            return;
        }

        let Some(ctx) = self.ctx.as_ref().cloned() else {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DOOM-RT-READBACK] match={} draw={} reason=ctx-null",
                match_seq, draw_index
            ));
            return;
        };
        let Some((rt, _rtv, _dsv, staging, rt_key)) = self.active_render_target_resources() else {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DOOM-RT-READBACK] match={} draw={} reason=rt-null active_rt=0x{:08X}",
                match_seq, draw_index, self.active_rt_key
            ));
            return;
        };

        unsafe {
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            rt.GetDesc(&mut desc);
            let width = desc.Width as usize;
            let height = desc.Height as usize;
            if width == 0 || height == 0 {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-DOOM-RT-READBACK] match={} draw={} reason=empty-rt rt=0x{:08X} {}x{}",
                    match_seq, draw_index, rt_key, width, height
                ));
                return;
            }

            ctx.CopyResource(&staging, &rt);
            ctx.Flush();

            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            if let Err(err) = ctx.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-DOOM-RT-READBACK] match={} draw={} reason=map-failed rt=0x{:08X} err={}",
                    match_seq, draw_index, rt_key, err
                ));
                return;
            }

            let mut pixels = vec![0xFF00_0000u32; width.saturating_mul(height)];
            let src = mapped.pData as *const u8;
            let dst = pixels.as_mut_ptr() as *mut u8;
            let row_bytes = width.saturating_mul(4);
            for y in 0..height {
                std::ptr::copy_nonoverlapping(
                    src.add(y.saturating_mul(mapped.RowPitch as usize)),
                    dst.add(y.saturating_mul(row_bytes)),
                    row_bytes,
                );
            }
            ctx.Unmap(&staging, 0);

            let full_path = format!(
                r"./doom_d3d11_rt_after_doom_quad_match{:05}_draw{:05}_rt{:08X}_{}x{}.bmp",
                match_seq, draw_index, rt_key, width, height
            );
            write_d3d11_readback_bmp(&full_path, width as u32, height as u32, &pixels);

            let crop_x = min_x.floor().max(0.0) as usize;
            let crop_y = min_y.floor().max(0.0) as usize;
            let crop_max_x = max_x.ceil().max(0.0) as usize;
            let crop_max_y = max_y.ceil().max(0.0) as usize;
            let crop_w = crop_max_x.min(width).saturating_sub(crop_x.min(width));
            let crop_h = crop_max_y.min(height).saturating_sub(crop_y.min(height));
            let mut crop = Vec::new();
            if crop_w > 0 && crop_h > 0 {
                crop.resize(crop_w * crop_h, 0xFF00_0000);
                for y in 0..crop_h {
                    let src_off = (crop_y + y) * width + crop_x;
                    let dst_off = y * crop_w;
                    crop[dst_off..dst_off + crop_w]
                        .copy_from_slice(&pixels[src_off..src_off + crop_w]);
                }
            }
            let crop_path = if crop.is_empty() {
                String::from("none")
            } else {
                let path = format!(
                    r"./doom_d3d11_rt_after_doom_quad_match{:05}_draw{:05}_rt{:08X}_drawrect{}x{}.bmp",
                    match_seq, draw_index, rt_key, crop_w, crop_h
                );
                write_d3d11_readback_bmp(&path, crop_w as u32, crop_h as u32, &crop);
                path
            };

            let full_rgb_nonblack = pixels.iter().filter(|px| (**px & 0x00FF_FFFF) != 0).count();
            let full_alpha_nonzero = pixels.iter().filter(|px| (**px & 0xFF00_0000) != 0).count();
            let crop_rgb_nonblack = crop.iter().filter(|px| (**px & 0x00FF_FFFF) != 0).count();
            let crop_alpha_nonzero = crop.iter().filter(|px| (**px & 0xFF00_0000) != 0).count();
            let (sample_rgb, sample_alpha, sample_count, first, mid, last) =
                sample_pixel_stats(&pixels);
            let (crop_sample_rgb, crop_sample_alpha, crop_sample_count, crop_first, crop_mid, _) =
                sample_pixel_stats(&crop);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DOOM-RT-READBACK] match={} draw={} prim={} verts={} rt=0x{:08X} rt_size={}x{} rowpitch={} full_hash=0x{:016X} crop_hash=0x{:016X} full_rgb_nonblack={}/{} full_alpha_nonzero={}/{} sample_rgb={}/{} sample_alpha={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X} crop={}x{} crop_rgb_nonblack={}/{} crop_alpha_nonzero={}/{} crop_sample_rgb={}/{} crop_sample_alpha={}/{} crop_first=0x{:08X} crop_mid=0x{:08X} tex=0x{:08X}->0x{:08X} data=0x{:08X} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.5},{:.5}..{:.5},{:.5}] full_path={} crop_path={}",
                match_seq,
                draw_index,
                prim_type,
                draw_verts.len(),
                rt_key,
                width,
                height,
                mapped.RowPitch,
                argb_fnv1a64(&pixels),
                argb_fnv1a64(&crop),
                full_rgb_nonblack,
                pixels.len(),
                full_alpha_nonzero,
                pixels.len(),
                sample_rgb,
                sample_count,
                sample_alpha,
                sample_count,
                first,
                mid,
                last,
                crop_w,
                crop_h,
                crop_rgb_nonblack,
                crop.len(),
                crop_alpha_nonzero,
                crop.len(),
                crop_sample_rgb,
                crop_sample_count,
                crop_sample_alpha,
                crop_sample_count,
                crop_first,
                crop_mid,
                self.active_tex_guest_raw,
                self.active_tex_guest_norm,
                self.active_tex_data,
                min_x,
                min_y,
                max_x,
                max_y,
                min_u,
                min_v,
                max_u,
                max_v,
                full_path,
                crop_path
            ));
        }
    }

    #[cfg(windows)]
    fn log_spidey_chis22_draw_probe(
        &self,
        draw_index: u32,
        prim_type: i32,
        verts: &[NV2AVertex],
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        min_u: f32,
        min_v: f32,
        max_u: f32,
        max_v: f32,
    ) {
        if !self.active_texture_is_spidey_chis22() {
            return;
        }

        static CHIS22_DRAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let seq = CHIS22_DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        SPIDEY_CHIS22_DRAW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        SPIDEY_LAST_CHIS22_DRAW.store(draw_index, std::sync::atomic::Ordering::Relaxed);
        let first_circle = SPIDEY_FIRST_CIRCLE_DRAW.load(std::sync::atomic::Ordering::Relaxed);
        let after_circle = first_circle != 0 && draw_index > first_circle;
        if !(seq < 96 || seq.is_power_of_two() || after_circle) {
            return;
        }

        let (min_a, max_a, rgb_nonzero, alpha_nonzero, first_color) = if verts.is_empty() {
            (0u8, 0u8, 0usize, 0usize, 0u32)
        } else {
            let first_color = verts[0].color;
            let (min_a, max_a, rgb, alpha) = verts.iter().fold(
                (u8::MAX, u8::MIN, 0usize, 0usize),
                |(lo, hi, rgb, alpha), v| {
                    let a = (v.color >> 24) as u8;
                    (
                        lo.min(a),
                        hi.max(a),
                        rgb + usize::from((v.color & 0x00FF_FFFF) != 0),
                        alpha + usize::from((v.color & 0xFF00_0000) != 0),
                    )
                },
            );
            (min_a, max_a, rgb, alpha, first_color)
        };
        let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
            self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-CHIS22-DRAW] #{} draw={} first_circle={} after_circle={} verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3},{:.3}..{:.3},{:.3}] tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:08X}/{} source={} srv={} color=0x{:08X} alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} blend={} z={} zwrite={} colorwrite=0x{:X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
            seq,
            draw_index,
            first_circle,
            after_circle,
            verts.len(),
            prim_type,
            min_x,
            min_y,
            max_x,
            max_y,
            min_u,
            min_v,
            max_u,
            max_v,
            self.active_tex_handle,
            self.active_tex_guest_raw,
            self.active_tex_guest_norm,
            self.active_tex_data,
            self.active_tex_format_code,
            self.active_texture_format_name(),
            self.active_tex_source,
            self.active_tex_srv_bound,
            first_color,
            min_a,
            max_a,
            rgb_nonzero,
            verts.len(),
            alpha_nonzero,
            verts.len(),
            self.render_state.alpha_blend_enable,
            self.render_state.z_enable,
            self.render_state.z_write_enable,
            self.render_state.color_write_enable,
            rt_key,
            rt_w,
            rt_h,
            rt_data,
            rt_pitch
        ));
    }

    /// Try to create a D3D11 device with the debug layer enabled.
    /// Returns Ok if successful. Fails silently if Graphics Tools
    /// (D3D11SDKLayers.dll) is not installed on the host.
    #[cfg(windows)]
    unsafe fn try_create_device_with_flags(
        flags: D3D11_CREATE_DEVICE_FLAG,
        device: &mut Option<ID3D11Device>,
        ctx: &mut Option<ID3D11DeviceContext>,
        feature_level: &mut D3D_FEATURE_LEVEL,
    ) -> windows::core::Result<()> {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            flags,
            None,
            D3D11_SDK_VERSION,
            Some(device),
            Some(feature_level),
            Some(ctx),
        )
    }

    /// Pump pending messages from the ID3D11InfoQueue into debug_log.
    /// Call each frame (or each Present-equivalent) after the debug layer is enabled.
    #[cfg(windows)]
    pub fn pump_debug_messages(&mut self) {
        let iq = match &self.info_queue {
            Some(q) => q,
            None => return,
        };
        unsafe {
            let count = iq.GetNumStoredMessages();
            if count == 0 {
                return;
            }
            // Cap messages per pump so we don't flood debug.log
            let cap = count.min(64);
            for i in 0..cap {
                // First call with null to get size
                let mut size: usize = 0;
                if iq.GetMessage(i, None, &mut size).is_err() || size == 0 {
                    continue;
                }
                let mut buf = vec![0u8; size];
                let msg_ptr = buf.as_mut_ptr() as *mut D3D11_MESSAGE;
                if iq.GetMessage(i, Some(msg_ptr), &mut size).is_err() {
                    continue;
                }
                let msg = &*msg_ptr;
                let sev = match msg.Severity {
                    D3D11_MESSAGE_SEVERITY_CORRUPTION => "CORRUPTION",
                    D3D11_MESSAGE_SEVERITY_ERROR => "ERROR",
                    D3D11_MESSAGE_SEVERITY_WARNING => "WARNING",
                    D3D11_MESSAGE_SEVERITY_INFO => "INFO",
                    _ => "MSG",
                };
                let desc = if !msg.pDescription.is_null() && msg.DescriptionByteLength > 0 {
                    let slice = std::slice::from_raw_parts(
                        msg.pDescription as *const u8,
                        msg.DescriptionByteLength.saturating_sub(1),
                    );
                    std::str::from_utf8(slice).unwrap_or("<non-utf8>")
                } else {
                    "<no description>"
                };
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11 {}] cat={} id={} {}",
                    sev, msg.Category.0, msg.ID.0, desc
                ));
            }
            iq.ClearStoredMessages();
        }
    }

    fn ffp_transform_index(state: u32) -> Option<usize> {
        match state {
            super::D3DTS_WORLD => Some(FFP_WORLD_INDEX),
            super::D3DTS_VIEW => Some(FFP_VIEW_INDEX),
            super::D3DTS_PROJECTION => Some(FFP_PROJECTION_INDEX),
            _ => None,
        }
    }

    fn matrix_is_identity(matrix: &[f32; 16]) -> bool {
        matrix
            .iter()
            .zip(FFP_IDENTITY_MATRIX.iter())
            .all(|(a, b)| (*a - *b).abs() <= 1.0e-5)
    }

    #[cfg(windows)]
    fn ffp_transform_chain_non_identity(&self) -> bool {
        self.ffp_transforms
            .iter()
            .any(|matrix| !Self::matrix_is_identity(matrix))
    }

    #[cfg(windows)]
    fn current_ffp_input_may_need_transform() -> bool {
        let raw =
            crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(std::sync::atomic::Ordering::Relaxed);
        let shader_hle_path = (0x0000_1000..0x0100_0000).contains(&raw);
        if raw != 0 && !shader_hle_path && (raw & 1) != 0 {
            return (raw & super::fvf_decode::fvf::POSITION_MASK) != super::fvf_decode::fvf::XYZRHW;
        }

        // Spider-Man's sample-RT compositor reaches us with raw shader state 0
        // and a 36-byte fixed-function stream. Smaller shader-zero streams are
        // usually true screen-space HUD quads and should keep the passthrough.
        raw == 0
            && crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.load(std::sync::atomic::Ordering::Relaxed)
                >= 36
    }

    #[cfg(windows)]
    fn ffp_transform_lane_active(&self) -> bool {
        self.active_nv2a_vs.is_none()
            && self.ffp_transform_chain_non_identity()
            && Self::current_ffp_input_may_need_transform()
    }

    #[cfg(windows)]
    fn ensure_ffp_transform_resources(&mut self) -> bool {
        if self.ffp_vs.is_some() && self.ffp_cbuffer.is_some() {
            return true;
        }
        let Some(device) = self.device.as_ref().cloned() else {
            return false;
        };

        if self.ffp_vs.is_none() {
            let mut code: Option<ID3DBlob> = None;
            let mut errors: Option<ID3DBlob> = None;
            let result = unsafe {
                D3DCompile(
                    FFP_VS_HLSL.as_ptr() as *const core::ffi::c_void,
                    FFP_VS_HLSL.len(),
                    windows::core::s!("rustemu_ffp_vs"),
                    None,
                    None,
                    windows::core::s!("main"),
                    windows::core::s!("vs_4_0"),
                    D3DCOMPILE_ENABLE_STRICTNESS | D3DCOMPILE_OPTIMIZATION_LEVEL1,
                    0,
                    &mut code,
                    Some(&mut errors),
                )
            };
            if let Err(e) = result {
                let err_text = errors
                    .as_ref()
                    .map(|blob| unsafe {
                        let bytes = std::slice::from_raw_parts(
                            blob.GetBufferPointer() as *const u8,
                            blob.GetBufferSize(),
                        );
                        String::from_utf8_lossy(bytes).trim().to_string()
                    })
                    .unwrap_or_default();
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-FFP-VS-COMPILE-FAIL] err={} {}",
                    e, err_text
                ));
                return false;
            }

            let Some(code) = code else {
                crate::xbox::emulator::debug_log("[D3D11-FFP-VS-COMPILE-FAIL] missing bytecode");
                return false;
            };
            let bytecode = unsafe {
                std::slice::from_raw_parts(
                    code.GetBufferPointer() as *const u8,
                    code.GetBufferSize(),
                )
            };
            let mut vs: Option<ID3D11VertexShader> = None;
            if let Err(e) = unsafe { device.CreateVertexShader(bytecode, None, Some(&mut vs)) } {
                crate::xbox::emulator::debug_log(&format!("[D3D11-FFP-VS-CREATE-FAIL] err={}", e));
                return false;
            }
            self.ffp_vs = vs;
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-FFP-VS-COMPILE] bytes={} hlsl_len={}",
                bytecode.len(),
                FFP_VS_HLSL.len()
            ));
        }

        if self.ffp_cbuffer.is_none() {
            let cb_desc = D3D11_BUFFER_DESC {
                ByteWidth: (FFP_TRANSFORM_COUNT * std::mem::size_of::<[f32; 16]>()) as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let mut cb: Option<ID3D11Buffer> = None;
            if let Err(e) = unsafe { device.CreateBuffer(&cb_desc, None, Some(&mut cb)) } {
                crate::xbox::emulator::debug_log(&format!("[D3D11-FFP-CBUFFER-FAIL] err={}", e));
                return false;
            }
            self.ffp_cbuffer = cb;
        }

        self.ffp_vs.is_some() && self.ffp_cbuffer.is_some()
    }

    #[cfg(windows)]
    fn upload_ffp_transforms(&self) {
        let (Some(ctx), Some(cb)) = (&self.ctx, &self.ffp_cbuffer) else {
            return;
        };
        unsafe {
            ctx.UpdateSubresource(
                cb,
                0,
                None,
                self.ffp_transforms.as_ptr() as *const core::ffi::c_void,
                0,
                0,
            );
            ctx.VSSetConstantBuffers(1, Some(&[Some(cb.clone())]));
        }
    }

    #[cfg(windows)]
    fn validate_draw_bindings(
        &mut self,
        label: &str,
        draw_index: u32,
        vertex_count: u32,
        prim_type: i32,
    ) -> bool {
        let Some(ctx) = self.ctx.as_ref().cloned() else {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-PREFLIGHT-FAIL] {} draw={} verts={} prim={} ctx=NULL",
                label, draw_index, vertex_count, prim_type
            ));
            return false;
        };

        unsafe {
            let mut vs: Option<ID3D11VertexShader> = None;
            let mut ps: Option<ID3D11PixelShader> = None;
            let mut rtvs: [Option<ID3D11RenderTargetView>; 1] = [None];
            let mut dsv: Option<ID3D11DepthStencilView> = None;

            ctx.VSGetShader(&mut vs, None, None);
            ctx.PSGetShader(&mut ps, None, None);
            ctx.OMGetRenderTargets(Some(&mut rtvs), Some(&mut dsv));

            let vs_bound = vs.is_some();
            let ps_bound = ps.is_some();
            let rtv_bound = rtvs[0].is_some();
            if vs_bound && ps_bound && rtv_bound {
                return true;
            }
            // Only reach here on binding failure — log it.
            let active_rt = self.debug_active_render_target();
            let (rt_key, rt_width, rt_height, rt_data, rt_pitch) =
                active_rt.unwrap_or((0, 0, 0, 0, 0));

            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-PREFLIGHT-FAIL] {} draw={} verts={} prim={} vs_bound={} ps_bound={} rtv_bound={} dsv_bound={} active_vs=0x{:08X} textured={} tex={} srv_bound={} rt_key=0x{:08X} rt={}x{} data=0x{:08X} pitch={}",
                label,
                draw_index,
                vertex_count,
                prim_type,
                vs_bound,
                ps_bound,
                rtv_bound,
                dsv.is_some(),
                self.active_nv2a_vs.unwrap_or(0),
                self.has_active_texture,
                self.active_tex_handle,
                self.active_tex_srv_bound,
                rt_key,
                rt_width,
                rt_height,
                rt_data,
                rt_pitch
            ));
            self.pump_debug_messages();
            false
        }
    }
}

impl GpuBackend for D3D11Backend {
    #[cfg(windows)]
    fn init(&mut self, width: i32, height: i32) -> bool {
        self.width = width;
        self.height = height;
        self.readback_buf = vec![0u32; (width * height) as usize];
        self.readback_x = 0;
        self.readback_y = 0;
        self.readback_width = width.max(0) as u32;
        self.readback_height = height.max(0) as u32;

        unsafe {
            // --- Create device (no swap chain, hardware) ---
            let mut device: Option<ID3D11Device> = None;
            let mut ctx: Option<ID3D11DeviceContext> = None;
            let mut feature_level = D3D_FEATURE_LEVEL_9_1;

            let debug_requested = d3d11_debug_layer_enabled();
            let mut debug_enabled = debug_requested;
            let hr = if debug_requested {
                // Opt-in only: the debug layer emits validation messages through
                // OutputDebugString, raising DBG_PRINTSERVICE_CODE exceptions
                // that our VEH sees. Keep validation available without making
                // normal gameplay pay the per-draw debug layer cost.
                let hr = Self::try_create_device_with_flags(
                    D3D11_CREATE_DEVICE_DEBUG,
                    &mut device,
                    &mut ctx,
                    &mut feature_level,
                );
                if hr.is_ok() {
                    crate::xbox::emulator::debug_log(
                        "[D3D11] Debug layer ENABLED by RUSTEMU_D3D11_DEBUG_LAYER",
                    );
                    hr
                } else {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11] Debug layer unavailable ({:?}) — retrying without DEBUG flag",
                        hr
                    ));
                    debug_enabled = false;
                    device = None;
                    ctx = None;
                    Self::try_create_device_with_flags(
                        D3D11_CREATE_DEVICE_FLAG(0),
                        &mut device,
                        &mut ctx,
                        &mut feature_level,
                    )
                }
            } else {
                Self::try_create_device_with_flags(
                    D3D11_CREATE_DEVICE_FLAG(0),
                    &mut device,
                    &mut ctx,
                    &mut feature_level,
                )
            };
            if hr.is_err() {
                crate::xbox::emulator::debug_log(&format!("[D3D11] CreateDevice FAILED: {:?}", hr));
                return false;
            }
            let device = device.unwrap();
            let ctx = ctx.unwrap();

            crate::xbox::emulator::debug_log(&format!(
                "[D3D11] Device created (feature level 0x{:X}, debug={})",
                feature_level.0, debug_enabled
            ));
            Self::log_adapter_identity(&device);

            // --- Hook up InfoQueue if debug layer enabled ---
            if debug_enabled {
                if let Ok(iq) = device.cast::<ID3D11InfoQueue>() {
                    // Capture corruption/error messages into debug.log for
                    // headless RetroArch runs instead of breaking before the
                    // message pump can record the offending resource.
                    let _ = iq.SetBreakOnSeverity(D3D11_MESSAGE_SEVERITY_CORRUPTION, false);
                    let _ = iq.SetBreakOnSeverity(D3D11_MESSAGE_SEVERITY_ERROR, false);
                    // Keep storage bounded; -1 = unlimited, but we pump each frame.
                    let _ = iq.SetMessageCountLimit(4096);
                    self.info_queue = Some(iq);
                    crate::xbox::emulator::debug_log(
                        "[D3D11] InfoQueue attached (logging ERROR/CORRUPTION)",
                    );
                } else {
                    crate::xbox::emulator::debug_log(
                        "[D3D11] InfoQueue cast FAILED — messages will not be pumped",
                    );
                }
            }

            // --- Render target (offscreen, BGRA) ---
            let rt_desc = D3D11_TEXTURE2D_DESC {
                Width: width as u32,
                Height: height as u32,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };
            let mut rt_tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&rt_desc, None, Some(&mut rt_tex)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] RT texture FAILED: {}", e));
                return false;
            }
            let rt_tex = rt_tex.unwrap();

            let mut rtv: Option<ID3D11RenderTargetView> = None;
            if let Err(e) = device.CreateRenderTargetView(&rt_tex, None, Some(&mut rtv)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] RTV FAILED: {}", e));
                return false;
            }
            let rtv = rtv.unwrap();

            // --- Depth/stencil ---
            let ds_desc = D3D11_TEXTURE2D_DESC {
                Width: width as u32,
                Height: height as u32,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };
            let mut ds_tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&ds_desc, None, Some(&mut ds_tex)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] DS texture FAILED: {}", e));
                return false;
            }
            let ds_tex = ds_tex.unwrap();

            let mut dsv: Option<ID3D11DepthStencilView> = None;
            if let Err(e) = device.CreateDepthStencilView(&ds_tex, None, Some(&mut dsv)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] DSV FAILED: {}", e));
                return false;
            }
            let dsv = dsv.unwrap();

            // --- Staging texture (CPU readback) ---
            let stag_desc = D3D11_TEXTURE2D_DESC {
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                ..rt_desc
            };
            let mut staging_tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&stag_desc, None, Some(&mut staging_tex)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] Staging FAILED: {}", e));
                return false;
            }
            let staging_tex = staging_tex.unwrap();

            // --- Shaders (from precompiled CSO blobs) ---
            let mut vs: Option<ID3D11VertexShader> = None;
            if let Err(e) = device.CreateVertexShader(shaders::VS_PASSTHROUGH, None, Some(&mut vs))
            {
                crate::xbox::emulator::debug_log(&format!("[D3D11] VS FAILED: {}", e));
                return false;
            }
            let vs = vs.unwrap();

            let mut ps: Option<ID3D11PixelShader> = None;
            if let Err(e) = device.CreatePixelShader(shaders::PS_PASSTHROUGH, None, Some(&mut ps)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] PS FAILED: {}", e));
                return false;
            }
            let ps = ps.unwrap();

            let mut ps_textured: Option<ID3D11PixelShader> = None;
            if let Err(e) =
                device.CreatePixelShader(shaders::PS_TEXTURED, None, Some(&mut ps_textured))
            {
                crate::xbox::emulator::debug_log(&format!("[D3D11] PS_TEXTURED FAILED: {}", e));
                return false;
            }
            if let Some(dynamic_ps) = Self::compile_textured_pixel_shader(&device) {
                ps_textured = Some(dynamic_ps);
            }
            let ps_spidey_solid = Self::compile_spidey_solid_pixel_shader(&device);

            // --- Input layout matching NV2AVertex ---
            let layout_desc = [
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("POSITION"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("COLOR"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32_UINT,
                    InputSlot: 0,
                    AlignedByteOffset: 16,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("TEXCOORD"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 20,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];
            let mut layout: Option<ID3D11InputLayout> = None;
            if let Err(e) =
                device.CreateInputLayout(&layout_desc, shaders::VS_PASSTHROUGH, Some(&mut layout))
            {
                crate::xbox::emulator::debug_log(&format!("[D3D11] InputLayout FAILED: {}", e));
                return false;
            }
            let layout = layout.unwrap();

            // --- Dynamic vertex buffer ---
            let vb_desc = D3D11_BUFFER_DESC {
                ByteWidth: (D3D11_DYNAMIC_VERTEX_CAP * std::mem::size_of::<D3D11DrawVertex>())
                    as u32,
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let mut vb: Option<ID3D11Buffer> = None;
            if let Err(e) = device.CreateBuffer(&vb_desc, None, Some(&mut vb)) {
                crate::xbox::emulator::debug_log(&format!("[D3D11] VB FAILED: {}", e));
                return false;
            }
            let vb = vb.unwrap();

            let vs_cb_desc = D3D11_BUFFER_DESC {
                ByteWidth: (D3D11_VS_CBUFFER_COUNT * std::mem::size_of::<[f32; 4]>()) as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let mut vs_cbuffer: Option<ID3D11Buffer> = None;
            if let Err(e) = device.CreateBuffer(&vs_cb_desc, None, Some(&mut vs_cbuffer)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11] VS constant buffer FAILED: {}",
                    e
                ));
                return false;
            }
            let vs_cbuffer = vs_cbuffer.unwrap();

            let ps_cb_desc = D3D11_BUFFER_DESC {
                ByteWidth: (46 * std::mem::size_of::<[f32; 4]>()) as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let mut ps_cbuffer: Option<ID3D11Buffer> = None;
            if let Err(e) = device.CreateBuffer(&ps_cb_desc, None, Some(&mut ps_cbuffer)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11] PS constant buffer FAILED: {}",
                    e
                ));
                return false;
            }
            let ps_cbuffer = ps_cbuffer.unwrap();

            // --- Rasterizer state (no culling) ---
            // 2026-04-24: DepthClipEnable=false. Spider-Man verts sometimes
            // carry z values far outside NDC [0,1] (e.g. z=1509 for HUD
            // quads where vertex format has no Z slot and our decoder
            // reads color bytes as Z). DepthClipEnable=true would clip
            // every such triangle pre-rasterization → zero output.
            let raster_desc = D3D11_RASTERIZER_DESC {
                FillMode: D3D11_FILL_SOLID,
                CullMode: D3D11_CULL_NONE,
                DepthClipEnable: false.into(),
                ..Default::default()
            };
            let mut rs: Option<ID3D11RasterizerState> = None;
            if device
                .CreateRasterizerState(&raster_desc, Some(&mut rs))
                .is_ok()
            {
                if let Some(ref rs) = rs {
                    ctx.RSSetState(rs);
                }
            }

            // --- Depth stencil state (always pass until we honour game Z) ---
            // 2026-04-24: depth test disabled with ALWAYS func. Xbox verts
            // often carry game-internal Z that isn't D3D11 NDC z in [0,1].
            // Combined with rasterizer DepthClipEnable=false, this lets
            // draws reach the RT without being killed by the depth stage.
            let ds_state_desc = D3D11_DEPTH_STENCIL_DESC {
                DepthEnable: false.into(),
                DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ZERO,
                DepthFunc: D3D11_COMPARISON_ALWAYS,
                ..Default::default()
            };
            let mut dss: Option<ID3D11DepthStencilState> = None;
            if device
                .CreateDepthStencilState(&ds_state_desc, Some(&mut dss))
                .is_ok()
            {
                if let Some(ref dss) = dss {
                    ctx.OMSetDepthStencilState(dss, 0);
                }
            }

            // --- Blend state ---
            // Seed D3D11 from Xbox D3D8 defaults: AlphaBlendEnable=false,
            // SrcBlend=ONE, DestBlend=ZERO. Later SetRenderState calls rebuild
            // this block, so legal/menu quads are not accidentally made fully
            // transparent by host-side always-on alpha blending.
            let mut blend_desc = D3D11_BLEND_DESC::default();
            let rs_default = self.render_state;
            blend_desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: rs_default.alpha_blend_enable.into(),
                SrcBlend: Self::d3d_blend_factor(rs_default.src_blend),
                DestBlend: Self::d3d_blend_factor(rs_default.dest_blend),
                BlendOp: Self::d3d_blend_op(rs_default.blend_op),
                SrcBlendAlpha: Self::d3d_blend_factor(rs_default.src_blend),
                DestBlendAlpha: Self::d3d_blend_factor(rs_default.dest_blend),
                BlendOpAlpha: Self::d3d_blend_op(rs_default.blend_op),
                RenderTargetWriteMask: Self::d3d_color_write_mask(rs_default.color_write_enable),
            };
            let mut bs: Option<ID3D11BlendState> = None;
            if device.CreateBlendState(&blend_desc, Some(&mut bs)).is_ok() {
                if let Some(ref bs) = bs {
                    ctx.OMSetBlendState(bs, None, 0xFFFF_FFFF);
                }
            }

            // --- Set pipeline state ---
            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), &dsv);

            let vp = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: width as f32,
                Height: height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };
            ctx.RSSetViewports(Some(&[vp]));
            ctx.VSSetShader(&vs, None);
            ctx.PSSetShader(&ps, None);
            ctx.IASetInputLayout(&layout);

            // Store state
            self.device = Some(device);
            self.ctx = Some(ctx);
            self.rt_tex = Some(rt_tex);
            self.rtv = Some(rtv);
            self.ds_tex = Some(ds_tex);
            self.dsv = Some(dsv);
            self.staging_tex = Some(staging_tex);
            self.vs = Some(vs);
            self.ps = Some(ps);
            self.ps_textured = ps_textured;
            self.ps_spidey_solid = ps_spidey_solid;
            self.layout = Some(layout);
            self.vb = Some(vb);
            self.vs_cbuffer = Some(vs_cbuffer);
            self.ps_cbuffer = Some(ps_cbuffer);
            self.rs = rs;
            self.dss = dss;
            self.bs = bs;
            self.mark_vs_constants_dirty();
            self.upload_vs_constants_if_dirty();
            self.mark_ps_constants_dirty();
            self.upload_ps_constants_if_dirty();
        }

        crate::xbox::emulator::debug_log(&format!(
            "[D3D11] Initialized ({}x{}), RT=B8G8R8A8, Staging ready",
            width, height
        ));
        true
    }

    #[cfg(not(windows))]
    fn init(&mut self, _width: i32, _height: i32) -> bool {
        false
    }

    fn shutdown(&mut self) {
        crate::xbox::emulator::debug_log("[D3D11] Shutdown");
        #[cfg(windows)]
        {
            self.active_rt_key = 0;
            self.backbuffer_rt_key = 0;
            self.backbuffer_rt_info = PendingRenderTarget::default();
            self.black_clear_without_draw_pending = false;
            self.draws_since_clear = 0;
            self.present_cache.clear();
            self.recent_draws = [RecentDrawSummary::default(); RECENT_DRAW_COUNT];
            self.recent_draw_cursor = 0;
            self.recent_draw_seen = 0;
            self.pending_rt = PendingRenderTarget::default();
            self.rt_cache.clear();
            self.active_nv2a_vs = None;
            self.nv2a_vs_cache.clear();
            self.nv2a_psh_cache.clear();
            self.ffp_transforms = [FFP_IDENTITY_MATRIX; FFP_TRANSFORM_COUNT];
            self.ffp_transform_mask = 0;
            self.ffp_cbuffer = None;
            self.vs_cbuffer = None;
            self.vb = None;
            self.layout = None;
            self.ps_textured = None;
            self.ps_spidey_solid = None;
            self.ps = None;
            self.ffp_vs = None;
            self.vs = None;
            self.bs = None;
            self.force_opaque_bs = None;
            self.force_no_depth_dss = None;
            self.dss = None;
            self.rs = None;
            self.staging_tex = None;
            self.dsv = None;
            self.ds_tex = None;
            self.rtv = None;
            self.rt_tex = None;
            self.ctx = None;
            self.device = None;
        }
    }

    #[cfg(windows)]
    fn draw_primitive(&mut self, verts: &[NV2AVertex], prim_type: i32) {
        let frame_prof = d3d11_frame_prof_enabled();
        let _draw_prof = D3D11PerfGuard::new(
            frame_prof,
            &D3D11_PROF_DRAW_CALLS,
            &D3D11_PROF_DRAW_TOTAL_US,
        );
        let (min_x, max_x, min_y, max_y) = verts.iter().fold(
            (
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ),
            |(min_x, max_x, min_y, max_y), v| {
                (
                    min_x.min(v.x),
                    max_x.max(v.x),
                    min_y.min(v.y),
                    max_y.max(v.y),
                )
            },
        );
        let (min_u, max_u, min_v, max_v) = verts.iter().fold(
            (
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ),
            |(min_u, max_u, min_v, max_v), v| {
                (
                    min_u.min(v.u),
                    max_u.max(v.u),
                    min_v.min(v.v),
                    max_v.max(v.v),
                )
            },
        );

        // Tripwire 2026-04-20: proves geometry reached the D3D11 backend.
        // Rate-limited so a render loop doesn't flood the log.
        let draw_index = {
            static TW_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            TW_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };
        static DISABLE_RAWPOS_FALLBACK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let disable_rawpos_fallback = *DISABLE_RAWPOS_FALLBACK.get_or_init(|| {
            crate::xbox::emulator::spidey_synth_training_enabled()
                || std::env::var("RUSTEMU_SPIDEY_DISABLE_RAWPOS_FALLBACK")
                    .map(|v| {
                        let v = v.trim();
                        v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                    })
                    .unwrap_or(false)
        });
        static FORCE_DEPTH_OFF: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let force_depth_off = *FORCE_DEPTH_OFF.get_or_init(|| {
            std::env::var("RUSTEMU_SPIDEY_FORCE_DEPTH_OFF")
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false)
        });
        static FORCE_DEPTH_OFF_AFTER: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
        let force_depth_off_after = *FORCE_DEPTH_OFF_AFTER.get_or_init(|| {
            std::env::var("RUSTEMU_SPIDEY_FORCE_DEPTH_OFF_AFTER")
                .ok()
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(0)
        });
        let force_depth_off_active = force_depth_off && draw_index >= force_depth_off_after;
        let host_rawpos_fallback = !disable_rawpos_fallback
            && self.active_nv2a_vs.is_some()
            && verts
                .iter()
                .all(|v| v.x.is_finite() && v.y.is_finite() && v.z.is_finite())
            && (min_x.abs().max(max_x.abs()) > 4.0 || min_y.abs().max(max_y.abs()) > 4.0);
        let rawpos_flag = if host_rawpos_fallback { 1.0 } else { 0.0 };
        if (self.vs_constants[191][0] - rawpos_flag).abs() > f32::EPSILON {
            self.vs_constants[191] = [rawpos_flag, 0.0, 0.0, 0.0];
            self.mark_vs_constants_dirty();
            static RAWPOS_GUARD_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RAWPOS_GUARD_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RAWPOS-GUARD] #{} draw={} active_vs=0x{:08X} rawpos_fallback={} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                    n,
                    draw_index,
                    self.active_nv2a_vs.unwrap_or(0),
                    host_rawpos_fallback,
                    min_x,
                    min_y,
                    max_x,
                    max_y
                ));
            }
        }
        {
            if draw_index < 16 || draw_index.is_power_of_two() {
                let (fx, fy, fz, fc) = verts
                    .first()
                    .map(|v| (v.x, v.y, v.z, v.color))
                    .unwrap_or((0.0, 0.0, 0.0, 0));
                let (min_a, max_a) = verts.iter().fold((u8::MAX, u8::MIN), |(lo, hi), v| {
                    let a = (v.color >> 24) as u8;
                    (lo.min(a), hi.max(a))
                });
                let all_verts: String = verts
                    .iter()
                    .map(|v| {
                        format!(
                            "({:.1},{:.1},{:.1};uv={:.3},{:.3})",
                            v.x, v.y, v.z, v.u, v.v
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-DRAW #{}] verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] all=[{}] color=0x{:08X} alpha=[{}..{}] uv=[{:.3}..{:.3},{:.3}..{:.3}] textured={} vflip={}",
                    draw_index,
                    verts.len(),
                    prim_type,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    all_verts,
                    fc,
                    min_a,
                    max_a,
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    self.has_active_texture,
                    self.texture_v_flip
                ));
                let _ = (fx, fy, fz);
            }
        }
        self.record_recent_draw(
            draw_index, verts, prim_type, min_x, min_y, max_x, max_y, min_u, min_v, max_u, max_v,
        );
        self.log_spidey_chis22_draw_probe(
            draw_index, prim_type, verts, min_x, min_y, max_x, max_y, min_u, min_v, max_u, max_v,
        );
        if self.active_texture_is_spidey_select_circle() {
            let first_circle_set = SPIDEY_FIRST_CIRCLE_DRAW
                .compare_exchange(
                    0,
                    draw_index,
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                )
                .is_ok();
            if first_circle_set {
                crate::xbox::aot::oovpa::oovpa_hle::note_spidey_menu_selector_seen(
                    "d3d11-circle-texture",
                    draw_index,
                );
                let chis22_count =
                    SPIDEY_CHIS22_DRAW_COUNT.load(std::sync::atomic::Ordering::Relaxed);
                let last_chis22 =
                    SPIDEY_LAST_CHIS22_DRAW.load(std::sync::atomic::Ordering::Relaxed);
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CHIS22-AT-FIRST-CIRCLE] circle_draw={} chis22_count={} last_chis22_draw={} gap={} title_patches={}",
                    draw_index,
                    chis22_count,
                    last_chis22,
                    draw_index.saturating_sub(last_chis22),
                    std::env::var_os("RUSTEMU_TITLE_PATCHES").is_some()
                        || std::env::var_os("RUSTEMU_SPIDEY_TITLE_PATCHES").is_some()
                        || std::env::var_os("RUSTEMU_SPIDEY_PATCHES").is_some()
                ));
            }
            static CIRCLE_DRAW_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let seq = CIRCLE_DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if seq < 128 || seq.is_power_of_two() {
                self.dump_recent_draws_before_circle(draw_index);
                self.log_spidey_select_circle_vs_probe(draw_index, prim_type, verts.len());
                let (min_a, max_a, rgb_nonzero, alpha_nonzero) = if verts.is_empty() {
                    (0u8, 0u8, 0usize, 0usize)
                } else {
                    verts.iter().fold(
                        (u8::MAX, u8::MIN, 0usize, 0usize),
                        |(lo, hi, rgb, alpha), v| {
                            let a = (v.color >> 24) as u8;
                            (
                                lo.min(a),
                                hi.max(a),
                                rgb + usize::from((v.color & 0x00FF_FFFF) != 0),
                                alpha + usize::from((v.color & 0xFF00_0000) != 0),
                            )
                        },
                    )
                };
                let s0 = &self.texture_stage_states[0];
                let active_vs = self.active_nv2a_vs.unwrap_or(0);
                let c4 = self.vs_constants[4];
                let c5 = self.vs_constants[5];
                let c6 = self.vs_constants[6];
                let c7 = self.vs_constants[7];
                let c58 = self.vs_constants[58];
                let c59 = self.vs_constants[59];
                let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
                    self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CIRCLE-DRAW] #{} draw={} verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3}..{:.3},{:.3}..{:.3}] tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} upload_fmt=0x{:08X}/{} source={} size={}x{} bytes={} srv_bound={} alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} color_write=0x{:X} z={} zwrite={} zfunc={:?} active_vs=0x{:08X} c4=[{:.3},{:.3},{:.3},{:.3}] c5=[{:.3},{:.3},{:.3},{:.3}] c6=[{:.3},{:.3},{:.3},{:.3}] c7=[{:.3},{:.3},{:.3},{:.3}] c58=[{:.3},{:.3},{:.3},{:.3}] c59=[{:.3},{:.3},{:.3},{:.3}] tss_alphakill=0x{:08X} tss_color=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) tss_alpha=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) tss_xform=0x{:08X} tss_tc=0x{:08X} ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_texmodes=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                    seq,
                    draw_index,
                    verts.len(),
                    prim_type,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.active_tex_guest_format_code,
                    crate::xbox::gpu::texture_format::format_name(
                        self.active_tex_guest_format_code
                    ),
                    self.active_tex_format_code,
                    self.active_texture_format_name(),
                    self.active_tex_source,
                    self.active_tex_width,
                    self.active_tex_height,
                    self.active_tex_byte_len,
                    self.active_tex_srv_bound,
                    min_a,
                    max_a,
                    rgb_nonzero,
                    verts.len(),
                    alpha_nonzero,
                    verts.len(),
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.blend_op,
                    self.render_state.alpha_test_enable,
                    self.render_state.alpha_ref,
                    self.render_state.alpha_func,
                    self.render_state.color_write_enable,
                    self.render_state.z_enable,
                    self.render_state.z_write_enable,
                    self.render_state.z_func,
                    active_vs,
                    c4[0],
                    c4[1],
                    c4[2],
                    c4[3],
                    c5[0],
                    c5[1],
                    c5[2],
                    c5[3],
                    c6[0],
                    c6[1],
                    c6[2],
                    c6[3],
                    c7[0],
                    c7[1],
                    c7[2],
                    c7[3],
                    c58[0],
                    c58[1],
                    c58[2],
                    c58[3],
                    c59[0],
                    c59[1],
                    c59[2],
                    c59[3],
                    s0[11],
                    s0[super::X_D3DTSS_COLOROP],
                    s0[super::X_D3DTSS_COLORARG0],
                    s0[super::X_D3DTSS_COLORARG1],
                    s0[super::X_D3DTSS_COLORARG2],
                    s0[super::X_D3DTSS_ALPHAOP],
                    s0[super::X_D3DTSS_ALPHAARG0],
                    s0[super::X_D3DTSS_ALPHAARG1],
                    s0[super::X_D3DTSS_ALPHAARG2],
                    s0[super::X_D3DTSS_TEXTURETRANSFORMFLAGS],
                    s0[super::X_D3DTSS_TEXCOORDINDEX],
                    self.ps_render_states[53],
                    self.ps_render_states[34],
                    self.ps_render_states[0],
                    self.ps_render_states[136],
                    rt_key,
                    rt_w,
                    rt_h,
                    rt_data,
                    rt_pitch
                ));
            }
        }
        let shader1001_focus = self.active_nv2a_vs == Some(0x0000_1001)
            || self.active_tex_guest_raw == 0x83C3_A630
            || self.active_tex_guest_norm == 0x03C3_A630
            || self.active_tex_guest_raw == 0x83C3_9D30
            || self.active_tex_guest_norm == 0x03C3_9D30;
        if shader1001_focus {
            let first_selector_set = SPIDEY_FIRST_CIRCLE_DRAW
                .compare_exchange(
                    0,
                    draw_index,
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                )
                .is_ok();
            if first_selector_set {
                crate::xbox::aot::oovpa::oovpa_hle::note_spidey_menu_selector_seen(
                    "d3d11-shader1001",
                    draw_index,
                );
                let chis22_count =
                    SPIDEY_CHIS22_DRAW_COUNT.load(std::sync::atomic::Ordering::Relaxed);
                let last_chis22 =
                    SPIDEY_LAST_CHIS22_DRAW.load(std::sync::atomic::Ordering::Relaxed);
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CHIS22-AT-FIRST-CIRCLE] source=shader1001 circle_draw={} chis22_count={} last_chis22_draw={} gap={} title_patches={}",
                    draw_index,
                    chis22_count,
                    last_chis22,
                    draw_index.saturating_sub(last_chis22),
                    std::env::var_os("RUSTEMU_TITLE_PATCHES").is_some()
                        || std::env::var_os("RUSTEMU_SPIDEY_TITLE_PATCHES").is_some()
                        || std::env::var_os("RUSTEMU_SPIDEY_PATCHES").is_some()
                ));
            }
            static SHADER1001_DRAW_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let seq = SHADER1001_DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if seq < 64 || seq.is_power_of_two() {
                let (
                    min_a,
                    max_a,
                    min_r,
                    max_r,
                    min_g,
                    max_g,
                    min_b,
                    max_b,
                    rgb_nonzero,
                    alpha_nonzero,
                    first_color,
                ) = if verts.is_empty() {
                    (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0usize, 0usize, 0u32)
                } else {
                    let first_color = verts[0].color;
                    let (min_a, max_a, min_r, max_r, min_g, max_g, min_b, max_b, rgb, alpha) =
                        verts.iter().fold(
                            (
                                u8::MAX,
                                u8::MIN,
                                u8::MAX,
                                u8::MIN,
                                u8::MAX,
                                u8::MIN,
                                u8::MAX,
                                u8::MIN,
                                0usize,
                                0usize,
                            ),
                            |(lo_a, hi_a, lo_r, hi_r, lo_g, hi_g, lo_b, hi_b, rgb, alpha), v| {
                                let a = (v.color >> 24) as u8;
                                let r = (v.color >> 16) as u8;
                                let g = (v.color >> 8) as u8;
                                let b = v.color as u8;
                                (
                                    lo_a.min(a),
                                    hi_a.max(a),
                                    lo_r.min(r),
                                    hi_r.max(r),
                                    lo_g.min(g),
                                    hi_g.max(g),
                                    lo_b.min(b),
                                    hi_b.max(b),
                                    rgb + usize::from((v.color & 0x00FF_FFFF) != 0),
                                    alpha + usize::from((v.color & 0xFF00_0000) != 0),
                                )
                            },
                        );
                    (
                        min_a,
                        max_a,
                        min_r,
                        max_r,
                        min_g,
                        max_g,
                        min_b,
                        max_b,
                        rgb,
                        alpha,
                        first_color,
                    )
                };
                let c58 = self.vs_constants[58];
                let c59 = self.vs_constants[59];
                let c4 = self.vs_constants[4];
                let c5 = self.vs_constants[5];
                let c6 = self.vs_constants[6];
                let c7 = self.vs_constants[7];
                let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
                    self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-SHADER1001-DRAW] #{} draw={} verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3}..{:.3},{:.3}..{:.3}] active_vs=0x{:08X} rawpos_fallback={} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} upload_fmt=0x{:08X}/{} source={} size={}x{} bytes={} srv_bound={} tex_alpha=[{}..{}] tex_alpha_nonzero={}/{} tex_rgb_nonzero={}/{} first_color=0x{:08X} vtx_a=[{}..{}] vtx_r=[{}..{}] vtx_g=[{}..{}] vtx_b=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} c4=[{:.3},{:.3},{:.3},{:.3}] c5=[{:.3},{:.3},{:.3},{:.3}] c6=[{:.3},{:.3},{:.3},{:.3}] c7=[{:.3},{:.3},{:.3},{:.3}] c58=[{:.3},{:.3},{:.3},{:.3}] c59=[{:.3},{:.3},{:.3},{:.3}] blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} color_write=0x{:X} z={} zwrite={} zfunc={:?} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                    seq,
                    draw_index,
                    verts.len(),
                    prim_type,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    self.active_nv2a_vs.unwrap_or(0),
                    host_rawpos_fallback,
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.active_tex_guest_format_code,
                    crate::xbox::gpu::texture_format::format_name(
                        self.active_tex_guest_format_code
                    ),
                    self.active_tex_format_code,
                    self.active_texture_format_name(),
                    self.active_tex_source,
                    self.active_tex_width,
                    self.active_tex_height,
                    self.active_tex_byte_len,
                    self.active_tex_srv_bound,
                    self.active_tex_alpha_min,
                    self.active_tex_alpha_max,
                    self.active_tex_alpha_nonzero_sample,
                    self.active_tex_sample_count,
                    self.active_tex_rgb_nonzero_sample,
                    self.active_tex_sample_count,
                    first_color,
                    min_a,
                    max_a,
                    min_r,
                    max_r,
                    min_g,
                    max_g,
                    min_b,
                    max_b,
                    rgb_nonzero,
                    verts.len(),
                    alpha_nonzero,
                    verts.len(),
                    c4[0],
                    c4[1],
                    c4[2],
                    c4[3],
                    c5[0],
                    c5[1],
                    c5[2],
                    c5[3],
                    c6[0],
                    c6[1],
                    c6[2],
                    c6[3],
                    c7[0],
                    c7[1],
                    c7[2],
                    c7[3],
                    c58[0],
                    c58[1],
                    c58[2],
                    c58[3],
                    c59[0],
                    c59[1],
                    c59[2],
                    c59[3],
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.blend_op,
                    self.render_state.alpha_test_enable,
                    self.render_state.alpha_ref,
                    self.render_state.alpha_func,
                    self.render_state.color_write_enable,
                    self.render_state.z_enable,
                    self.render_state.z_write_enable,
                    self.render_state.z_func,
                    rt_key,
                    rt_w,
                    rt_h,
                    rt_data,
                    rt_pitch
                ));
            }
        }
        let tile_w = max_x - min_x;
        let tile_h = max_y - min_y;
        let tile_candidate = self.has_active_texture
            && min_x.is_finite()
            && max_x.is_finite()
            && min_y.is_finite()
            && max_y.is_finite()
            && min_x >= -8.0
            && min_y >= -8.0
            && max_x <= self.width as f32 + 8.0
            && max_y <= self.height as f32 + 8.0
            && tile_w >= 64.0
            && tile_h >= 64.0
            && verts.len() <= 8;
        let title_top_right_candidate =
            tile_candidate && min_x < 640.0 && max_x > 384.0 && min_y < 260.0 && max_y > -8.0;
        let stage0_state = self.texture_stage_states[0];
        static TITLE_UI_TRACE_ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let title_ui_trace = *TITLE_UI_TRACE_ON
            .get_or_init(|| std::env::var_os("RUSTEMU_SPIDEY_TITLE_UI_TRACE").is_some());
        let title_ui_texture_size = matches!(
            (self.active_tex_width, self.active_tex_height),
            (512, 64)
                | (256, 64)
                | (128, 64)
                | (64, 64)
                | (64, 128)
                | (128, 128)
                | (256, 128)
                | (128, 256)
                | (256, 256)
        );
        let title_ui_band = min_x.is_finite()
            && max_x.is_finite()
            && min_y.is_finite()
            && max_y.is_finite()
            && (
                // 640x480 title layout: top logo sits above the menu panel,
                // prompts sit in the bottom strip, and START/arrows sit over
                // the center selector circle.
                max_y <= 145.0
                    || min_y >= 360.0
                    || (min_x <= 570.0 && max_x >= 300.0 && min_y <= 340.0 && max_y >= 120.0)
            );
        if title_ui_trace && self.has_active_texture && title_ui_texture_size && title_ui_band {
            static TITLE_UI_DRAW_N: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let ui_n = TITLE_UI_DRAW_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if ui_n < 512 || ui_n.is_power_of_two() {
                let (min_a, max_a, rgb_nonzero, alpha_nonzero, first_color) = if verts.is_empty() {
                    (0u8, 0u8, 0usize, 0usize, 0u32)
                } else {
                    let first_color = verts[0].color;
                    let (min_a, max_a, rgb, alpha) = verts.iter().fold(
                        (u8::MAX, u8::MIN, 0usize, 0usize),
                        |(lo, hi, rgb, alpha), v| {
                            let a = (v.color >> 24) as u8;
                            (
                                lo.min(a),
                                hi.max(a),
                                rgb + usize::from((v.color & 0x00FF_FFFF) != 0),
                                alpha + usize::from((v.color & 0xFF00_0000) != 0),
                            )
                        },
                    );
                    (min_a, max_a, rgb, alpha, first_color)
                };
                let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
                    self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TITLE-UI-DRAW] #{} draw={} verts={} prim={} bbox=[{:.1},{:.1}..{:.1},{:.1}] tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} upload_fmt=0x{:08X}/{} source={} tex_size={}x{} bytes={} srv_bound={} tex_alpha=[{}..{}] tex_alpha_nonzero={}/{} tex_rgb_nonzero={}/{} first_color=0x{:08X} vtx_alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} uv=[{:.3}..{:.3},{:.3}..{:.3}] blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} color_write=0x{:X} z={} zwrite={} zfunc={:?} active_vs=0x{:08X} tfactor=0x{:08X} tss_color=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) tss_alpha=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_texmodes=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                    ui_n,
                    draw_index,
                    verts.len(),
                    prim_type,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.active_tex_guest_format_code,
                    crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
                    self.active_tex_format_code,
                    self.active_texture_format_name(),
                    self.active_tex_source,
                    self.active_tex_width,
                    self.active_tex_height,
                    self.active_tex_byte_len,
                    self.active_tex_srv_bound,
                    self.active_tex_alpha_min,
                    self.active_tex_alpha_max,
                    self.active_tex_alpha_nonzero_sample,
                    self.active_tex_sample_count,
                    self.active_tex_rgb_nonzero_sample,
                    self.active_tex_sample_count,
                    first_color,
                    min_a,
                    max_a,
                    rgb_nonzero,
                    verts.len(),
                    alpha_nonzero,
                    verts.len(),
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.blend_op,
                    self.render_state.alpha_test_enable,
                    self.render_state.alpha_ref,
                    self.render_state.alpha_func,
                    self.render_state.color_write_enable,
                    self.render_state.z_enable,
                    self.render_state.z_write_enable,
                    self.render_state.z_func,
                    self.active_nv2a_vs.unwrap_or(0),
                    self.render_state.texture_factor,
                    stage0_state[super::X_D3DTSS_COLOROP],
                    stage0_state[super::X_D3DTSS_COLORARG0],
                    stage0_state[super::X_D3DTSS_COLORARG1],
                    stage0_state[super::X_D3DTSS_COLORARG2],
                    stage0_state[super::X_D3DTSS_ALPHAOP],
                    stage0_state[super::X_D3DTSS_ALPHAARG0],
                    stage0_state[super::X_D3DTSS_ALPHAARG1],
                    stage0_state[super::X_D3DTSS_ALPHAARG2],
                    self.ps_render_states[53],
                    self.ps_render_states[34],
                    self.ps_render_states[0],
                    self.ps_render_states[136],
                    rt_key,
                    rt_w,
                    rt_h,
                    rt_data,
                    rt_pitch
                ));
            }
        }
        if tile_candidate && draw_index >= 16_000 {
            static TILE_DIAG_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let tile_log_n = TILE_DIAG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if tile_log_n < 2048 {
                let zero_sized = self.active_tex_width == 0 || self.active_tex_height == 0;
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TILE-DIAG #{}] seq={} bbox=[{:.1},{:.1}..{:.1},{:.1}] size={:.1}x{:.1} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} fmt=0x{:08X}/{} source={} tex_size={}x{} bytes={} srv_null={} zero_size={} tex_alpha=[{}..{}] tex_alpha_nonzero={}/{} tex_rgb_nonzero={}/{} tfactor=0x{:08X} tss_alpha=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) combiner=0x{:08X} rgb0=0x{:08X} alpha0=0x{:08X} rgbout0=0x{:08X} final_abcd=0x{:08X} final_efg=0x{:08X} fc=[0x{:08X},0x{:08X}] texmodes=0x{:08X} uv=[{:.3}..{:.3},{:.3}..{:.3}] top_right={}",
                    draw_index,
                    tile_log_n,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    tile_w,
                    tile_h,
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.active_tex_guest_format_code,
                    crate::xbox::gpu::texture_format::format_name(
                        self.active_tex_guest_format_code
                    ),
                    self.active_tex_format_code,
                    self.active_texture_format_name(),
                    self.active_tex_source,
                    self.active_tex_width,
                    self.active_tex_height,
                    self.active_tex_byte_len,
                    !self.active_tex_srv_bound,
                    zero_sized,
                    self.active_tex_alpha_min,
                    self.active_tex_alpha_max,
                    self.active_tex_alpha_nonzero_sample,
                    self.active_tex_sample_count,
                    self.active_tex_rgb_nonzero_sample,
                    self.active_tex_sample_count,
                    self.render_state.texture_factor,
                    stage0_state[super::X_D3DTSS_ALPHAOP],
                    stage0_state[super::X_D3DTSS_ALPHAARG0],
                    stage0_state[super::X_D3DTSS_ALPHAARG1],
                    stage0_state[super::X_D3DTSS_ALPHAARG2],
                    self.ps_render_states[53],
                    self.ps_render_states[34],
                    self.ps_render_states[0],
                    self.ps_render_states[45],
                    self.ps_render_states[8],
                    self.ps_render_states[9],
                    self.ps_render_states[43],
                    self.ps_render_states[44],
                    self.ps_render_states[136],
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    title_top_right_candidate
                ));
            }
        }
        let region_overlap = min_x < 640.0 && max_x > 384.0 && min_y < 256.0 && max_y > 0.0;
        let region_inside = min_x >= 384.0 && max_x <= 640.0 && min_y >= 0.0 && max_y <= 256.0;
        let first_title_frame_window = (15_990..=16_140).contains(&draw_index);
        if first_title_frame_window || (region_overlap && draw_index >= 16_000) {
            static REGION_DIAG_N: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let region_seq = REGION_DIAG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if first_title_frame_window || region_seq < 2048 {
                let (min_a, max_a, rgb_nonzero, alpha_nonzero) =
                    verts
                        .iter()
                        .fold((u8::MAX, u8::MIN, 0usize, 0usize), |acc, v| {
                            let (min_a, max_a, rgb_nonzero, alpha_nonzero) = acc;
                            let a = (v.color >> 24) as u8;
                            (
                                min_a.min(a),
                                max_a.max(a),
                                rgb_nonzero + usize::from((v.color & 0x00FF_FFFF) != 0),
                                alpha_nonzero + usize::from((v.color & 0xFF00_0000) != 0),
                            )
                        });
                let first_color = verts.first().map(|v| v.color).unwrap_or(0);
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-REGION-DIAG #{}] seq={} bbox=[{:.1},{:.1}..{:.1},{:.1}] overlaps={} inside={} verts={} prim={} vsh=0x{:08X} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} textured={} srv_null={} tex_fmt=0x{:08X}/{} tex_source={} tex_size={}x{} first_color=0x{:08X} alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} blend={} src={:?} dst={:?} blend_op={} color_write=0x{:X} z_enable={} tfactor=0x{:08X} tss_alpha=0x{:08X}(args=0x{:08X},0x{:08X},0x{:08X}) combiner=0x{:08X} rgb0=0x{:08X} alpha0=0x{:08X} rgbout0=0x{:08X} final_abcd=0x{:08X} final_efg=0x{:08X} fc=[0x{:08X},0x{:08X}] texmodes=0x{:08X} uv=[{:.3}..{:.3},{:.3}..{:.3}]",
                    draw_index,
                    region_seq,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    region_overlap,
                    region_inside,
                    verts.len(),
                    prim_type,
                    self.active_nv2a_vs.unwrap_or(0),
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.active_tex_guest_format_code,
                    crate::xbox::gpu::texture_format::format_name(
                        self.active_tex_guest_format_code
                    ),
                    self.has_active_texture,
                    !self.active_tex_srv_bound,
                    self.active_tex_format_code,
                    self.active_texture_format_name(),
                    self.active_tex_source,
                    self.active_tex_width,
                    self.active_tex_height,
                    first_color,
                    min_a,
                    max_a,
                    rgb_nonzero,
                    verts.len(),
                    alpha_nonzero,
                    verts.len(),
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.blend_op,
                    self.render_state.color_write_enable,
                    self.render_state.z_enable,
                    self.render_state.texture_factor,
                    stage0_state[super::X_D3DTSS_ALPHAOP],
                    stage0_state[super::X_D3DTSS_ALPHAARG0],
                    stage0_state[super::X_D3DTSS_ALPHAARG1],
                    stage0_state[super::X_D3DTSS_ALPHAARG2],
                    self.ps_render_states[53],
                    self.ps_render_states[34],
                    self.ps_render_states[0],
                    self.ps_render_states[45],
                    self.ps_render_states[8],
                    self.ps_render_states[9],
                    self.ps_render_states[43],
                    self.ps_render_states[44],
                    self.ps_render_states[136],
                    min_u,
                    max_u,
                    min_v,
                    max_v
                ));
            }
        }
        static LOG_DRAW_RANGES: std::sync::OnceLock<Vec<(u32, u32)>> = std::sync::OnceLock::new();
        static DROP_DRAW_RANGES: std::sync::OnceLock<Vec<(u32, u32)>> = std::sync::OnceLock::new();
        static DROP_TEXTURES: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        static FOCUS_TEXTURES: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        let log_draw_range = draw_index_in_ranges(
            draw_index,
            LOG_DRAW_RANGES.get_or_init(|| parse_draw_ranges_env("RUSTEMU_SPIDEY_LOG_DRAW_RANGE")),
        );
        let drop_draw_range = draw_index_in_ranges(
            draw_index,
            DROP_DRAW_RANGES
                .get_or_init(|| parse_draw_ranges_env("RUSTEMU_SPIDEY_DROP_DRAW_RANGE")),
        );
        let drop_texture = {
            let drop_textures =
                DROP_TEXTURES.get_or_init(|| parse_u32_list_env("RUSTEMU_SPIDEY_DROP_TEX"));
            drop_textures.iter().any(|key| {
                *key == self.active_tex_guest_norm
                    || *key == self.active_tex_guest_raw
                    || *key == self.active_tex_data
            })
        };
        let focus_texture = {
            let focus_textures =
                FOCUS_TEXTURES.get_or_init(|| parse_u32_list_env("RUSTEMU_SPIDEY_FOCUS_TEX"));
            focus_textures.iter().any(|key| {
                *key == self.active_tex_guest_norm
                    || *key == self.active_tex_guest_raw
                    || *key == self.active_tex_data
            })
        };
        if log_draw_range || drop_draw_range || drop_texture || focus_texture {
            let first = verts.first().copied().unwrap_or_default();
            let (min_a, max_a, rgb_nonzero, alpha_nonzero) = verts.iter().fold(
                (u8::MAX, u8::MIN, 0usize, 0usize),
                |(lo, hi, rgb, alpha), v| {
                    let a = (v.color >> 24) as u8;
                    (
                        lo.min(a),
                        hi.max(a),
                        rgb + usize::from((v.color & 0x00FF_FFFF) != 0),
                        alpha + usize::from((v.color & 0xFF00_0000) != 0),
                    )
                },
            );
            let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
                self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DRAW-RANGE] draw={} action={} verts={} prim={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] first=({:.4},{:.4},{:.4},w={:.4},c=0x{:08X},uv={:.4},{:.4}) active_vs=0x{:08X} rawpos_fallback={} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} textured={} srv_bound={} fmt=0x{:02X}/{} upload_fmt=0x{:08X}/{} tex_source={} tex_size={}x{} tex_bytes={} alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} blend={} src={:?} dst={:?} blend_op={} alpha_test={} alpha_ref={} alpha_func={:?} color_write=0x{:X} z={} zwrite={} zfunc={:?} ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_out0=0x{:08X} ps_final=[0x{:08X},0x{:08X}] ps_texmodes=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                draw_index,
                if drop_texture {
                    "drop-tex"
                } else if drop_draw_range {
                    "drop"
                } else if focus_texture {
                    "focus-tex"
                } else {
                    "observe"
                },
                verts.len(),
                prim_type,
                min_x,
                min_y,
                verts.iter().fold(f32::INFINITY, |m, v| m.min(v.z)),
                max_x,
                max_y,
                verts.iter().fold(f32::NEG_INFINITY, |m, v| m.max(v.z)),
                first.x,
                first.y,
                first.z,
                first.w,
                first.color,
                first.u,
                first.v,
                self.active_nv2a_vs.unwrap_or(0),
                host_rawpos_fallback,
                self.active_tex_handle,
                self.active_tex_guest_raw,
                self.active_tex_guest_norm,
                self.active_tex_data,
                self.has_active_texture,
                self.active_tex_srv_bound,
                self.active_tex_guest_format_code,
                crate::xbox::gpu::texture_format::format_name(
                    self.active_tex_guest_format_code
                ),
                self.active_tex_format_code,
                self.active_texture_format_name(),
                self.active_tex_source,
                self.active_tex_width,
                self.active_tex_height,
                self.active_tex_byte_len,
                min_a,
                max_a,
                rgb_nonzero,
                verts.len(),
                alpha_nonzero,
                verts.len(),
                self.render_state.alpha_blend_enable,
                self.render_state.src_blend,
                self.render_state.dest_blend,
                self.render_state.blend_op,
                self.render_state.alpha_test_enable,
                self.render_state.alpha_ref,
                self.render_state.alpha_func,
                self.render_state.color_write_enable,
                self.render_state.z_enable,
                self.render_state.z_write_enable,
                self.render_state.z_func,
                self.ps_render_states[53],
                self.ps_render_states[34],
                self.ps_render_states[0],
                self.ps_render_states[45],
                self.ps_render_states[8],
                self.ps_render_states[9],
                self.ps_render_states[136],
                rt_key,
                rt_w,
                rt_h,
                rt_data,
                rt_pitch
            ));
        }
        if drop_draw_range || drop_texture {
            let _ = super::take_next_draw_skin_payload(verts.len());
            return;
        }
        let ctx = match self.ctx.as_ref().cloned() {
            Some(c) => c,
            None => return,
        };
        let vb = match self.vb.as_ref().cloned() {
            Some(b) => b,
            None => return,
        };
        let near_fullscreen = min_x <= 5.0
            && min_y <= 5.0
            && max_x >= (self.width as f32 - 5.0)
            && max_y >= (self.height as f32 - 5.0);
        let tex_w = self.active_tex_width as f32;
        let tex_h = self.active_tex_height as f32;
        let uv_span_u = max_u - min_u;
        let uv_span_v = max_v - min_v;
        let plausible_pixel_uvs = self.has_active_texture
            && self.active_tex_width > 0
            && self.active_tex_height > 0
            && (max_u > 2.0 || max_v > 2.0)
            && min_u >= -1.0
            && min_v >= -1.0
            && max_u <= tex_w + 1.0
            && max_v <= tex_h + 1.0
            && (near_fullscreen || uv_span_u > 8.0 || uv_span_v > 8.0);
        let pixel_space_uvs = plausible_pixel_uvs;
        if pixel_space_uvs {
            static PIXEL_UV_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PIXEL_UV_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 8 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-UV-NORM #{}] tex={}x{} uv=[{:.1}..{:.1},{:.1}..{:.1}]",
                    n, self.active_tex_width, self.active_tex_height, min_u, max_u, min_v, max_v
                ));
            }
        }
        let expanded_verts = if prim_type == super::NV097_QUAD_LIST {
            static QUAD_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = QUAD_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-QUADLIST] #{} in_verts={} quads={} dropped={} out_verts={}",
                    n,
                    verts.len(),
                    verts.len() / 4,
                    verts.len() % 4,
                    (verts.len() / 4) * 6
                ));
            }
            Some(expand_xbox_quad_list(verts))
        } else if prim_type == super::NV097_TRIANGLE_FAN {
            Some(expand_triangle_fan(verts))
        } else {
            None
        };
        let skin_payload = super::take_next_draw_skin_payload(verts.len());
        let expanded_skin = if prim_type == super::NV097_QUAD_LIST {
            Some(expand_xbox_quad_list_skin(&skin_payload))
        } else if prim_type == super::NV097_TRIANGLE_FAN {
            Some(expand_triangle_fan_skin(&skin_payload))
        } else {
            None
        };
        let source_verts: &[NV2AVertex] = expanded_verts.as_deref().unwrap_or(verts);
        let source_skin: &[VertexSkinPayload] = expanded_skin.as_deref().unwrap_or(&skin_payload);

        let adjusted_verts;
        let draw_verts: &[NV2AVertex] =
            if self.has_active_texture && (pixel_space_uvs || self.texture_v_flip) {
                adjusted_verts = source_verts
                    .iter()
                    .map(|v| {
                        let mut out = *v;
                        if pixel_space_uvs {
                            out.u /= self.active_tex_width as f32;
                            out.v /= self.active_tex_height as f32;
                        }
                        if self.texture_v_flip {
                            out.v = 1.0 - out.v;
                        }
                        out
                    })
                    .collect::<Vec<_>>();
                &adjusted_verts
            } else {
                source_verts
            };
        if spidey_peter_only_render_enabled() {
            let active_vs = self.active_nv2a_vs.unwrap_or(0);
            let is_peter_draw = spidey_peter_only_shader(active_vs);
            static PETER_ONLY_DROP_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            static PETER_ONLY_KEEP_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let peter_only_armed =
                SPIDEY_PETER_ONLY_SEEN.load(std::sync::atomic::Ordering::Relaxed);
            if !is_peter_draw && peter_only_armed {
                let n = PETER_ONLY_DROP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[SPIDEY-PETER-ONLY-DROP] #{} draw={} vs=0x{:08X} verts={} prim={} rt=0x{:08X}",
                        n,
                        draw_index,
                        active_vs,
                        draw_verts.len(),
                        prim_type,
                        self.active_rt_key
                    ));
                }
                return;
            }

            if is_peter_draw
                && !SPIDEY_PETER_ONLY_SEEN.swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                self.clear(0xFF00_0000);
            }
            if is_peter_draw {
                let n = PETER_ONLY_KEEP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[SPIDEY-PETER-ONLY-KEEP] #{} draw={} vs=0x{:08X} verts={} prim={} rt=0x{:08X} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                        n,
                        draw_index,
                        active_vs,
                        draw_verts.len(),
                        prim_type,
                        self.active_rt_key,
                        min_x,
                        min_y,
                        max_x,
                        max_y
                    ));
                }
            }
        }
        let count = draw_verts.len().min(D3D11_DYNAMIC_VERTEX_CAP);
        if count == 0 {
            return;
        }
        self.queue_peter_candidate_overlay(draw_index as u64, draw_verts, Some(source_skin));
        self.maybe_dump_peter_probe_json(draw_index, draw_verts, source_skin);
        self.log_spidey_peter_ps_state(draw_index, draw_verts, prim_type as u32);
        self.log_spidey_pixel_lane_trace(draw_index, draw_verts, prim_type as u32);
        self.log_spidey_100d_skin_eval(draw_index, prim_type as u32, draw_verts, source_skin);
        let ffp_transform_lane_active = self.ffp_transform_lane_active();
        let use_ffp_transform_lane =
            ffp_transform_lane_active && self.ensure_ffp_transform_resources();
        let screen_space_ui_depth_off = self.active_nv2a_vs.is_none()
            && !ffp_transform_lane_active
            && self.has_active_texture
            && self.render_state.alpha_blend_enable
            && !self.render_state.z_write_enable
            && min_x.is_finite()
            && max_x.is_finite()
            && min_y.is_finite()
            && max_y.is_finite()
            && max_x >= -1024.0
            && min_x <= self.width as f32 + 1024.0
            && max_y >= -1024.0
            && min_y <= self.height as f32 + 1024.0;
        let force_100d_solid =
            spidey_force_100d_solid_enabled() && self.active_nv2a_vs == Some(0x0000_100D);
        let force_peter_diffuse_white = if spidey_force_peter_diffuse_white_enabled() {
            static FORCE_PETER_WHITE_VS: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
            let forced_vs = FORCE_PETER_WHITE_VS
                .get_or_init(|| parse_u32_list_env("RUSTEMU_SPIDEY_FORCE_PETER_DIFFUSE_WHITE_VS"));
            match self.active_nv2a_vs {
                Some(vs) if forced_vs.is_empty() => vs == 0x0000_100D,
                Some(vs) => forced_vs.contains(&vs),
                None => false,
            }
        } else {
            false
        };
        let original_first_color = draw_verts.first().map(|v| v.color).unwrap_or(0);
        let mut upload_verts: Vec<D3D11DrawVertex> = draw_verts
            .iter()
            .copied()
            .zip(source_skin.iter().copied())
            .take(count)
            .map(|(v, skin)| D3D11DrawVertex::from_nv2a(v, skin))
            .collect();
        if force_100d_solid {
            for v in &mut upload_verts {
                v.color = 0xFFFF_00FF;
            }
            static FORCE_100D_SOLID_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = FORCE_100D_SOLID_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[SPIDEY-FORCE-100D-SOLID] #{} draw={} verts={} prim={} rt=0x{:08X} tex={} textured={} blend={} src={:?} dst={:?} colorwrite=0x{:X} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                    n,
                    draw_index,
                    count,
                    prim_type,
                    self.active_rt_key,
                    self.active_tex_handle,
                    self.has_active_texture,
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.color_write_enable,
                    min_x,
                    min_y,
                    max_x,
                    max_y
                ));
            }
        }
        if force_peter_diffuse_white {
            for v in &mut upload_verts {
                v.color = 0xFFFF_FFFF;
            }
            static FORCE_PETER_WHITE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = FORCE_PETER_WHITE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[SPIDEY-FORCE-PETER-DIFFUSE-WHITE] #{} draw={} vs=0x{:08X} verts={} prim={} first_color=0x{:08X} rt=0x{:08X} tex={} guest=0x{:08X}->0x{:08X} data=0x{:08X} textured={} blend={} src={:?} dst={:?} colorwrite=0x{:X} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                    n,
                    draw_index,
                    self.active_nv2a_vs.unwrap_or(0),
                    count,
                    prim_type,
                    original_first_color,
                    self.active_rt_key,
                    self.active_tex_handle,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    self.has_active_texture,
                    self.render_state.alpha_blend_enable,
                    self.render_state.src_blend,
                    self.render_state.dest_blend,
                    self.render_state.color_write_enable,
                    min_x,
                    min_y,
                    max_x,
                    max_y
                ));
            }
        }

        unsafe {
            // Some HLE paths touch D3D state outside the normal render pass.
            // Re-bind the complete draw-output state so queued guest draws are
            // replayed against the offscreen RT with permissive raster/depth/blend.
            self.bind_active_render_target();
            if let Some(ref rs) = self.rs {
                ctx.RSSetState(rs);
            }
            if let Some(ref dss) = self.dss {
                ctx.OMSetDepthStencilState(dss, 0);
            }
            let no_depth_reason = if force_depth_off_active {
                Some("env")
            } else if screen_space_ui_depth_off {
                Some("screen-ui")
            } else {
                None
            };
            if let Some(reason) = no_depth_reason {
                if self.force_no_depth_dss.is_none() {
                    if let Some(device) = self.device.as_ref() {
                        let ds_state_desc = D3D11_DEPTH_STENCIL_DESC {
                            DepthEnable: false.into(),
                            DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ZERO,
                            DepthFunc: D3D11_COMPARISON_ALWAYS,
                            StencilEnable: false.into(),
                            ..Default::default()
                        };
                        let mut dss: Option<ID3D11DepthStencilState> = None;
                        if device
                            .CreateDepthStencilState(&ds_state_desc, Some(&mut dss))
                            .is_ok()
                        {
                            self.force_no_depth_dss = dss;
                        }
                    }
                }
                if let Some(ref dss) = self.force_no_depth_dss {
                    ctx.OMSetDepthStencilState(dss, 0);
                }
                static FORCE_DEPTH_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = FORCE_DEPTH_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 8 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-FORCE-DEPTH-OFF] #{} reason={} draw={} active_rt=0x{:08X} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                        n, reason, draw_index, self.active_rt_key, min_x, min_y, max_x, max_y
                    ));
                }
            }
            if force_100d_solid {
                if self.force_opaque_bs.is_none() {
                    if let Some(device) = self.device.as_ref() {
                        let mut blend_desc = D3D11_BLEND_DESC::default();
                        blend_desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                            BlendEnable: false.into(),
                            SrcBlend: D3D11_BLEND_ONE,
                            DestBlend: D3D11_BLEND_ZERO,
                            BlendOp: D3D11_BLEND_OP_ADD,
                            SrcBlendAlpha: D3D11_BLEND_ONE,
                            DestBlendAlpha: D3D11_BLEND_ZERO,
                            BlendOpAlpha: D3D11_BLEND_OP_ADD,
                            RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
                        };
                        let mut bs: Option<ID3D11BlendState> = None;
                        if device.CreateBlendState(&blend_desc, Some(&mut bs)).is_ok() {
                            self.force_opaque_bs = bs;
                        }
                    }
                }
                if let Some(ref bs) = self.force_opaque_bs {
                    ctx.OMSetBlendState(bs, None, 0xFFFF_FFFF);
                }
            } else if let Some(ref bs) = self.bs {
                ctx.OMSetBlendState(bs, None, 0xFFFF_FFFF);
            }
            let (rt_key, rt_w, rt_h, rt_data, rt_pitch) =
                self.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
            let honor_guest_viewport = d3d11_honor_guest_viewport();
            let rt_local_viewport = d3d11_rt_local_viewport();
            let has_guest_viewport = self.readback_width > 0 && self.readback_height > 0;
            let guest_viewport_is_full = self.readback_x == 0
                && self.readback_y == 0
                && self.readback_width == self.width.max(0) as u32
                && self.readback_height == self.height.max(0) as u32;
            let drawing_non_backbuffer_rt =
                rt_key != 0 && rt_key != self.backbuffer_rt_key && rt_w > 0 && rt_h > 0;
            let (vp_x, vp_y, vp_w, vp_h, vp_mode) =
                if rt_local_viewport && drawing_non_backbuffer_rt {
                    if has_guest_viewport {
                        let target_w = rt_w.max(1);
                        let target_h = rt_h.max(1);
                        let vp_x = self.readback_x.min(target_w.saturating_sub(1));
                        let vp_y = self.readback_y.min(target_h.saturating_sub(1));
                        let vp_w = self
                            .readback_width
                            .max(1)
                            .min(target_w.saturating_sub(vp_x).max(1));
                        let vp_h = self
                            .readback_height
                            .max(1)
                            .min(target_h.saturating_sub(vp_y).max(1));
                        (
                            vp_x as f32,
                            vp_y as f32,
                            vp_w as f32,
                            vp_h as f32,
                            "rt-guest",
                        )
                    } else {
                        (0.0, 0.0, rt_w as f32, rt_h as f32, "rt-full")
                    }
                } else if honor_guest_viewport && has_guest_viewport {
                    (
                        self.readback_x as f32,
                        self.readback_y as f32,
                        self.readback_width.max(1) as f32,
                        self.readback_height.max(1) as f32,
                        "guest",
                    )
                } else {
                    (0.0, 0.0, self.width as f32, self.height as f32, "full")
                };
            if rt_local_viewport
                || honor_guest_viewport
                || (has_guest_viewport && !guest_viewport_is_full)
            {
                static DRAW_VP_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = DRAW_VP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 128 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-DRAW-VP] #{} draw={} mode={} honor={} rtlocal={} guest={}x{}+{}+{} host={:.0}x{:.0}+{:.0}+{:.0} active_vs=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                        n,
                        draw_index,
                        vp_mode,
                        honor_guest_viewport,
                        rt_local_viewport,
                        self.readback_width,
                        self.readback_height,
                        self.readback_x,
                        self.readback_y,
                        vp_w,
                        vp_h,
                        vp_x,
                        vp_y,
                        self.active_nv2a_vs.unwrap_or(0),
                        rt_key,
                        rt_w,
                        rt_h,
                        rt_data,
                        rt_pitch,
                        min_x,
                        min_y,
                        max_x,
                        max_y
                    ));
                }
            }
            let rt_trace = spidey_rt_trace_enabled();
            let drawing_offscreen_rt =
                rt_key != 0 && rt_key != self.backbuffer_rt_key && spidey_rt_diag_key(rt_key);
            let sampling_rt_texture = self.active_tex_source.starts_with("render-target")
                || spidey_rt_diag_key(self.active_tex_guest_norm)
                || spidey_rt_diag_key(self.active_tex_data);
            if rt_trace && (drawing_offscreen_rt || sampling_rt_texture) {
                static RT_DRAW_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = RT_DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 2048 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-RT-DRAW] #{} draw={} role={} verts={} prim={} active_vs=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={} viewport={}x{}+{}+{} host={:.0}x{:.0}+{:.0}+{:.0} tex_source={} tex=0x{:08X}->0x{:08X} tex_data=0x{:08X} tex_size={}x{} tex_fmt=0x{:08X}/{} srv={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3}..{:.3},{:.3}..{:.3}] blend={} z={} zwrite={} color_write=0x{:X}",
                        n,
                        draw_index,
                        if drawing_offscreen_rt && sampling_rt_texture {
                            "draw-rt+sample-rt"
                        } else if drawing_offscreen_rt {
                            "draw-rt"
                        } else {
                            "sample-rt"
                        },
                        verts.len(),
                        prim_type,
                        self.active_nv2a_vs.unwrap_or(0),
                        rt_key,
                        rt_w,
                        rt_h,
                        rt_data,
                        rt_pitch,
                        self.readback_width,
                        self.readback_height,
                        self.readback_x,
                        self.readback_y,
                        vp_w,
                        vp_h,
                        vp_x,
                        vp_y,
                        self.active_tex_source,
                        self.active_tex_guest_raw,
                        self.active_tex_guest_norm,
                        self.active_tex_data,
                        self.active_tex_width,
                        self.active_tex_height,
                        self.active_tex_format_code,
                        self.active_texture_format_name(),
                        self.active_tex_srv_bound,
                        min_x,
                        min_y,
                        max_x,
                        max_y,
                        min_u,
                        max_u,
                        min_v,
                        max_v,
                        self.render_state.alpha_blend_enable,
                        self.render_state.z_enable,
                        self.render_state.z_write_enable,
                        self.render_state.color_write_enable
                    ));
                    let active_sample_vs = self.active_nv2a_vs.unwrap_or(0);
                    if sampling_rt_texture
                        && (self.active_nv2a_vs.is_none()
                            || matches!(active_sample_vs, 0x0000_1002 | 0x0000_100B))
                    {
                        let guest_vs_raw = crate::xbox::aot::nv2a_pb::VERTEX_SHADER
                            .load(std::sync::atomic::Ordering::Relaxed);
                        let stream_stride = crate::xbox::aot::nv2a_pb::STREAM0_STRIDE
                            .load(std::sync::atomic::Ordering::Relaxed);
                        let slot = |idx: usize| -> (u32, u32, u32, u32) {
                            (
                                crate::xbox::aot::nv2a_pb::VS_SLOT_TYPE[idx]
                                    .load(std::sync::atomic::Ordering::Relaxed),
                                crate::xbox::aot::nv2a_pb::VS_SLOT_SIZE[idx]
                                    .load(std::sync::atomic::Ordering::Relaxed),
                                crate::xbox::aot::nv2a_pb::VS_SLOT_STRIDE[idx]
                                    .load(std::sync::atomic::Ordering::Relaxed),
                                crate::xbox::aot::nv2a_pb::VS_SLOT_OFFSET[idx]
                                    .load(std::sync::atomic::Ordering::Relaxed),
                            )
                        };
                        let s0 = slot(0);
                        let s1 = slot(1);
                        let s3 = slot(3);
                        let s4 = slot(4);
                        let s9 = slot(9);
                        let tss0 = &self.texture_stage_states[0];
                        let tss1 = &self.texture_stage_states[1];
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-RT-SAMPLE-SLOTS] #{} draw={} active_vs=0x{:08X} guest_vs=0x{:08X} {} stream0_stride={} slot0=type{} size{} stride{} off0x{:08X} slot1=type{} size{} stride{} off0x{:08X} slot3=type{} size{} stride{} off0x{:08X} slot4=type{} size{} stride{} off0x{:08X} slot9=type{} size{} stride{} off0x{:08X} tss0=addr{}/{} filt{}/{}/{} xform=0x{:X} texcoord=0x{:X} colorop={} alphaop={} tss1=addr{}/{} filt{}/{}/{} xform=0x{:X} texcoord=0x{:X} colorop={} alphaop={} rawpos={} screen_ui={} orig_first=[{}] draw_first=[{}]",
                            n,
                            draw_index,
                            active_sample_vs,
                            guest_vs_raw,
                            spidey_describe_guest_vs(guest_vs_raw),
                            stream_stride,
                            s0.0,
                            s0.1,
                            s0.2,
                            s0.3,
                            s1.0,
                            s1.1,
                            s1.2,
                            s1.3,
                            s3.0,
                            s3.1,
                            s3.2,
                            s3.3,
                            s4.0,
                            s4.1,
                            s4.2,
                            s4.3,
                            s9.0,
                            s9.1,
                            s9.2,
                            s9.3,
                            tss0[super::X_D3DTSS_ADDRESSU],
                            tss0[super::X_D3DTSS_ADDRESSV],
                            tss0[super::X_D3DTSS_MINFILTER],
                            tss0[super::X_D3DTSS_MAGFILTER],
                            tss0[super::X_D3DTSS_MIPFILTER],
                            tss0[super::X_D3DTSS_TEXTURETRANSFORMFLAGS],
                            tss0[super::X_D3DTSS_TEXCOORDINDEX],
                            tss0[super::X_D3DTSS_COLOROP],
                            tss0[super::X_D3DTSS_ALPHAOP],
                            tss1[super::X_D3DTSS_ADDRESSU],
                            tss1[super::X_D3DTSS_ADDRESSV],
                            tss1[super::X_D3DTSS_MINFILTER],
                            tss1[super::X_D3DTSS_MAGFILTER],
                            tss1[super::X_D3DTSS_MIPFILTER],
                            tss1[super::X_D3DTSS_TEXTURETRANSFORMFLAGS],
                            tss1[super::X_D3DTSS_TEXCOORDINDEX],
                            tss1[super::X_D3DTSS_COLOROP],
                            tss1[super::X_D3DTSS_ALPHAOP],
                            host_rawpos_fallback,
                            screen_space_ui_depth_off,
                            spidey_format_vertex_sample(verts),
                            spidey_format_vertex_sample(draw_verts)
                        ));
                    }
                }
            }
            if self.update_cxbxr_screenspace_constants(draw_index, rt_key, rt_w, rt_h) {
                self.mark_vs_constants_dirty();
            }
            crate::xbox::emulator::note_spidey_synth_d3d11_draw(
                draw_index as u64,
                self.active_nv2a_vs.unwrap_or(0),
                rt_key,
                rt_w,
                rt_h,
                min_x,
                min_y,
                max_x,
                max_y,
            );
            self.log_spidey_synth_rt_constants_diag(
                draw_index, min_x, min_y, max_x, max_y, rt_key, rt_w, rt_h, rt_data, rt_pitch,
                vp_x, vp_y, vp_w, vp_h, vp_mode,
            );
            self.log_spidey_peter_vs_constants_diag(
                draw_index, prim_type, verts, min_x, min_y, max_x, max_y, rt_key, rt_w, rt_h,
                rt_data, rt_pitch, vp_x, vp_y, vp_w, vp_h, vp_mode,
            );
            let vp = D3D11_VIEWPORT {
                TopLeftX: vp_x,
                TopLeftY: vp_y,
                Width: vp_w,
                Height: vp_h,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };
            ctx.RSSetViewports(Some(&[vp]));
            if let Some(handle) = self.active_nv2a_vs {
                if let Some(layout) = self.nv2a_layout_cache.get(&handle) {
                    ctx.IASetInputLayout(layout);
                } else if let Some(ref layout) = self.layout {
                    ctx.IASetInputLayout(layout);
                }
                if let Some(vs) = self.nv2a_vs_cache.get(&handle) {
                    ctx.VSSetShader(vs, None);
                    self.upload_vs_constants_if_dirty();
                } else if let Some(ref vs) = self.vs {
                    ctx.VSSetShader(vs, None);
                }
            } else if use_ffp_transform_lane {
                if let Some(ref layout) = self.layout {
                    ctx.IASetInputLayout(layout);
                }
                if let Some(ref vs) = self.ffp_vs {
                    ctx.VSSetShader(vs, None);
                    self.upload_ffp_transforms();
                    static FFP_BIND_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = FFP_BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 8 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-FFP-VS-BIND] #{} draw={} mask=0x{:X} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
                            n, draw_index, self.ffp_transform_mask, min_x, min_y, max_x, max_y
                        ));
                    }
                }
            } else if let Some(ref vs) = self.vs {
                if let Some(ref layout) = self.layout {
                    ctx.IASetInputLayout(layout);
                }
                ctx.VSSetShader(vs, None);
            }

            // Map dynamic VB
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            let map_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            if ctx
                .Map(&vb, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))
                .is_err()
            {
                return;
            }
            std::ptr::copy_nonoverlapping(
                upload_verts.as_ptr() as *const u8,
                mapped.pData as *mut u8,
                count * std::mem::size_of::<D3D11DrawVertex>(),
            );
            ctx.Unmap(&vb, 0);
            if let Some(start) = map_start {
                d3d11_prof_add_us(&D3D11_PROF_DRAW_MAP_US, start);
            }

            // Set VB and topology
            let stride = std::mem::size_of::<D3D11DrawVertex>() as u32;
            let offset = 0u32;
            ctx.IASetVertexBuffers(0, 1, Some(&Some(vb.clone())), Some(&stride), Some(&offset));
            ctx.IASetPrimitiveTopology(map_prim_type(prim_type));

            // Select PS based on active texture state. Always bind the
            // passthrough shader after SetTexture(NULL); otherwise D3D11 keeps
            // the previous textured PS and samples an unbound SRV.
            if force_100d_solid {
                if let Some(ps) = self.ps_spidey_solid.clone() {
                    self.upload_ps_constants_if_dirty();
                    ctx.PSSetShader(&ps, None);
                } else if let Some(ps) = self.ps.clone() {
                    self.upload_ps_constants_if_dirty();
                    ctx.PSSetShader(&ps, None);
                }
            } else if self.has_active_texture {
                let mut bound_translated = false;
                let psh_state =
                    super::nv2a_psh::PixelShaderState::from_render_states(&self.ps_render_states);
                if let Some(program) = super::nv2a_psh::translate_basic_modulate(psh_state) {
                    if !self.nv2a_psh_cache.contains_key(&program.key) {
                        if let Some(ps) = self.compile_nv2a_pixel_shader(program.key, &program.hlsl)
                        {
                            self.nv2a_psh_cache.insert(program.key, ps);
                        }
                    }
                    if let Some(ps) = self.nv2a_psh_cache.get(&program.key).cloned() {
                        self.upload_ps_constants_if_dirty();
                        ctx.PSSetShader(&ps, None);
                        bound_translated = true;
                    }
                }
                if !bound_translated {
                    if let Some(ps) = self.ps_textured.clone() {
                        self.upload_ps_constants_if_dirty();
                        ctx.PSSetShader(&ps, None);
                    }
                }
            } else if let Some(ps) = self.ps.clone() {
                self.upload_ps_constants_if_dirty();
                ctx.PSSetShader(&ps, None);
            }

            static PSH_BIND_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PSH_BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PSH-BIND] #{} textured={} combiner=0x{:08X} rgb0=0x{:08X} texmodes=0x{:08X}",
                    n,
                    self.has_active_texture,
                    self.ps_render_states[53],
                    self.ps_render_states[34],
                    self.ps_render_states[136]
                ));
            }

            // Draw
            self.pump_debug_messages();
            if d3d11_preflight_validate_enabled()
                && !self.validate_draw_bindings("pre-Draw", draw_index, count as u32, prim_type)
            {
                return;
            }
            let gpu_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            ctx.Draw(count as u32, 0);
            if let Some(start) = gpu_start {
                d3d11_prof_add_us(&D3D11_PROF_DRAW_GPU_US, start);
            }
            self.pump_debug_messages();
            if d3d11_flush_each_draw_enabled() {
                D3D11_PROF_DRAW_FLUSH_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let flush_start = if frame_prof {
                    Some(std::time::Instant::now())
                } else {
                    None
                };
                ctx.Flush();
                if let Some(start) = flush_start {
                    d3d11_prof_add_us(&D3D11_PROF_DRAW_FLUSH_US, start);
                }
                self.pump_debug_messages();
            }
        }
        self.maybe_capture_doom_post_draw_rt(draw_index, prim_type, draw_verts);
        self.draws_since_clear = self.draws_since_clear.saturating_add(1);
        self.black_clear_without_draw_pending = false;
        static CAPTURE_DRAW_LIST: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        let capture_draw_list = CAPTURE_DRAW_LIST.get_or_init(|| {
            let capture_draw_steps = std::env::var("RUSTEMU_SPIDEY_CAPTURE_DRAW_STEPS")
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false);
            let mut draws = Vec::new();
            if capture_draw_steps {
                draws.extend([
                    16_000, 18_000, 20_000, 24_000, 28_000, 32_000, 40_000, 48_000,
                ]);
            }
            if let Ok(raw) = std::env::var("RUSTEMU_SPIDEY_CAPTURE_DRAWS") {
                for part in raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace()) {
                    if let Ok(v) = part.trim().parse::<u32>() {
                        draws.push(v);
                    }
                }
            }
            if let Ok(raw) = std::env::var("RUSTEMU_SPIDEY_CAPTURE_DRAW_RANGE") {
                let nums: Vec<u32> = raw
                    .split(|c: char| c == ',' || c == ';' || c == ':' || c.is_whitespace())
                    .filter_map(|part| part.trim().parse::<u32>().ok())
                    .collect();
                if nums.len() >= 2 {
                    let start = nums[0].min(nums[1]);
                    let end = nums[0].max(nums[1]);
                    let step = nums.get(2).copied().unwrap_or(1).max(1);
                    let mut draw = start;
                    while draw <= end {
                        draws.push(draw);
                        match draw.checked_add(step) {
                            Some(next) if next > draw => draw = next,
                            _ => break,
                        }
                    }
                }
            }
            draws.sort_unstable();
            draws.dedup();
            draws
        });
        if capture_draw_list.binary_search(&draw_index).is_ok() {
            let pixels = self.readback_framebuffer().to_vec();
            let path = format!(r"./gate3_after_draw_{:05}.bmp", draw_index);
            write_d3d11_readback_bmp(&path, self.width as u32, self.height as u32, &pixels);
            let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
                sampled_argb_stats(&pixels);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DRAW-CAPTURE] draw={} path={} sample_nonzero={} sample_alpha={} sample_rgb={} first=0x{:08X} mid=0x{:08X} bbox=[{:.1},{:.1}..{:.1},{:.1}] tex={} textured={}",
                draw_index,
                path,
                sample_nonzero,
                sample_alpha,
                sample_rgb,
                first_px,
                mid_px,
                min_x,
                min_y,
                max_x,
                max_y,
                self.active_tex_handle,
                self.has_active_texture
            ));
        }
        self.capture_spidey_select_circle_after_draw(draw_index, prim_type, draw_verts);
    }

    #[cfg(not(windows))]
    fn draw_primitive(&mut self, _verts: &[NV2AVertex], _prim_type: i32) {}

    #[cfg(windows)]
    fn clear(&mut self, color: u32) {
        if spidey_peter_only_render_enabled()
            && SPIDEY_PETER_ONLY_SEEN.load(std::sync::atomic::Ordering::Relaxed)
        {
            static PETER_ONLY_CLEAR_DROP_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PETER_ONLY_CLEAR_DROP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[SPIDEY-PETER-ONLY-CLEAR-DROP] #{} color=0x{:08X}",
                    n, color
                ));
            }
            return;
        }
        self.clear_surface(color, true, true, true);
    }

    #[cfg(windows)]
    fn clear_surface(
        &mut self,
        color: u32,
        clear_color: bool,
        clear_depth: bool,
        clear_stencil: bool,
    ) {
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return,
        };
        let Some((_rt, rtv, dsv, _staging, key)) = self.active_render_target_resources() else {
            return;
        };

        let black_opaque_clear =
            clear_color && (color & 0x00FF_FFFF) == 0 && (color & 0xFF00_0000) != 0;
        let skip_repeated_black_clear = black_opaque_clear
            && self.black_clear_without_draw_pending
            && self.draws_since_clear == 0;
        if rtv.as_raw().is_null() || dsv.as_raw().is_null() {
            static NULL_CLEAR_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = NULL_CLEAR_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-CLEAR-SKIP-NULL-RT] #{} rt=0x{:08X} color=0x{:08X} rtv_null={} dsv_null={}",
                    n,
                    key,
                    color,
                    rtv.as_raw().is_null(),
                    dsv.as_raw().is_null()
                ));
            }
            return;
        }

        if skip_repeated_black_clear {
            static BLACK_SKIP_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BLACK_SKIP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-BLACK-CLEAR-SKIP] #{} rt=0x{:08X} color=0x{:08X} draws_since_clear={}",
                    n, key, color, self.draws_since_clear
                ));
            }
            return;
        }

        let mut did_clear = false;

        unsafe {
            if clear_color {
                // ARGB32 → float RGBA
                let rgba: [f32; 4] = [
                    ((color >> 16) & 0xFF) as f32 / 255.0,
                    ((color >> 8) & 0xFF) as f32 / 255.0,
                    (color & 0xFF) as f32 / 255.0,
                    ((color >> 24) & 0xFF) as f32 / 255.0,
                ];
                ctx.ClearRenderTargetView(&rtv, &rgba);
                did_clear = true;
            }

            let mut ds_flags = 0u32;
            if clear_depth {
                ds_flags |= D3D11_CLEAR_DEPTH.0 as u32;
            }
            if clear_stencil {
                ds_flags |= D3D11_CLEAR_STENCIL.0 as u32;
            }
            if ds_flags != 0 {
                ctx.ClearDepthStencilView(&dsv, ds_flags, 1.0, 0);
                did_clear = true;
            }

            // Force the clear to reach the GPU before any subsequent
            // CopyResource() in readback_framebuffer(). Without this,
            // D3D11's deferred immediate context can reorder the clear
            // behind the copy, producing stale (black) pixels in
            // readback. Confirmed by 25-agent Sprint 6C audit.
            if did_clear {
                ctx.Flush();
            }
        }
        if clear_color {
            self.draws_since_clear = 0;
            self.black_clear_without_draw_pending = black_opaque_clear;
            if self.black_clear_without_draw_pending {
                static BLACK_CLEAR_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = BLACK_CLEAR_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-BLACK-CLEAR-PENDING] #{} rt=0x{:08X} color=0x{:08X}",
                        n, key, color
                    ));
                }
            }
        }
    }

    #[cfg(not(windows))]
    fn clear(&mut self, _color: u32) {}

    #[cfg(windows)]
    fn set_texture(&mut self, stage: u32, width: u32, height: u32, pixels: &[u32]) {
        let _ = self.upload_argb_texture(
            stage,
            width,
            height,
            pixels,
            0xFFFF_FFFE,
            "argb",
            pixels.len() * std::mem::size_of::<u32>(),
        );
    }

    #[cfg(not(windows))]
    fn set_texture(&mut self, _stage: u32, _width: u32, _height: u32, _pixels: &[u32]) {}

    #[cfg(windows)]
    fn set_texture_raw(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        format_code: u32,
        bytes: &[u8],
    ) {
        D3D11Backend::set_texture_raw(self, stage, width, height, format_code, bytes);
    }

    #[cfg(not(windows))]
    fn set_texture_raw(
        &mut self,
        _stage: u32,
        _width: u32,
        _height: u32,
        _format_code: u32,
        _bytes: &[u8],
    ) {
    }

    fn bind_render_target_texture(&mut self, stage: u32, key: u32) -> bool {
        #[cfg(windows)]
        {
            if stage >= 4 || key == 0 {
                return false;
            }
            if key == self.active_rt_key {
                static RT_SELF_BIND_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = RT_SELF_BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() || spidey_rt_diag_key(key) {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-RT-TEX-HAZARD] #{} action=reject-active-rt stage={} key=0x{:08X} active_rt=0x{:08X}",
                        n, stage, key, self.active_rt_key
                    ));
                }
                return false;
            }
            let whiteout_rt_03c00150 = std::env::var("RUSTEMU_SPIDEY_WHITEOUT_RT_03C00150")
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false);
            if whiteout_rt_03c00150 && stage == 0 && key == 0x03C0_0150 && self.active_rt_key == 0 {
                let white = [0xFFFF_FFFFu32];
                if self.upload_argb_texture(stage, 1, 1, &white, 0xFFFF_FFFE, "rt-whiteout", 4) {
                    self.active_tex_guest_raw = key;
                    self.active_tex_guest_norm = key;
                    self.active_tex_data = key;
                    self.active_tex_guest_format_code = 0xFFFF_FFFE;
                    self.active_tex_swizzled_blocks = false;
                    static RT_WHITEOUT_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = RT_WHITEOUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-RT-WHITEOUT] #{} stage={} key=0x{:08X} active_rt=0x{:08X} bound=1x1-white",
                            n, stage, key, self.active_rt_key
                        ));
                    }
                    return true;
                }
            }
            self.bind_render_target_texture_impl(stage, key, "render-target", key, key)
        }

        #[cfg(not(windows))]
        {
            let _ = (stage, key);
            false
        }
    }

    fn bind_render_target_texture_by_data(&mut self, stage: u32, data_addr: u32) -> bool {
        #[cfg(windows)]
        {
            let Some((key, base, len)) = self.find_render_target_alias_by_data(data_addr) else {
                return false;
            };
            let bound = self.bind_render_target_texture_impl(
                stage,
                key,
                "render-target-alias",
                data_addr & !0x3,
                data_addr & !0x3,
            );
            if bound {
                static RT_ALIAS_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = RT_ALIAS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-RT-ALIAS] #{} stage={} data=0x{:08X} -> key=0x{:08X} range=[0x{:08X}..0x{:08X}) bytes={}",
                        n,
                        stage,
                        data_addr & !0x3,
                        key,
                        base,
                        (base as u64).saturating_add(len) as u32,
                        len
                    ));
                }
            }
            bound
        }

        #[cfg(not(windows))]
        {
            let _ = (stage, data_addr);
            false
        }
    }

    fn set_texture_debug_info(
        &mut self,
        stage: u32,
        tex_raw: u32,
        tex_norm: u32,
        data_addr: u32,
        format_code: u32,
    ) {
        if stage != 0 {
            return;
        }
        self.active_tex_guest_raw = tex_raw;
        self.active_tex_guest_norm = tex_norm;
        self.active_tex_data = data_addr;
        self.active_tex_guest_format_code = format_code;
        self.active_tex_swizzled_blocks = false;
    }

    fn set_texture_swizzle_hint(&mut self, stage: u32, swizzled_blocks: bool) {
        if stage != 0 {
            return;
        }
        self.active_tex_swizzled_blocks = swizzled_blocks;
    }

    fn clear_texture(&mut self, stage: u32) {
        // DEBUG STICKY TEXTURE (2026-04-24): when set to true, DO NOT reset
        // has_active_texture here — forces every subsequent draw through
        // PS_TEXTURED sampling whatever's bound at stage 0. Used once to
        // prove the D3D11 upload path works end-to-end (produced RED glyphs
        // when paired with TEST_PATTERN_ON_EMPTY in oovpa_hle.rs — result
        // verified, pipeline confirmed correct).
        //
        // Now OFF — organic behavior: SetTexture(NULL) resets the flag so
        // vertex-colored PS_PASSTHROUGH draws work correctly (forcing them
        // through PS_TEXTURED with no UVs would break the input layout).
        const STICKY_CHECKER_DEBUG: bool = false;
        #[cfg(windows)]
        unsafe {
            if let Some(ctx) = &self.ctx {
                // Even in sticky mode we still unbind this stage's SRV —
                // upstream test pattern has re-bound stage 0 already if
                // that's where the debug texture lives.
                if !STICKY_CHECKER_DEBUG {
                    ctx.PSSetShaderResources(stage, Some(&[None]));
                }
            }
            if !STICKY_CHECKER_DEBUG && stage == 0 {
                self.has_active_texture = false;
                self.texture_v_flip = false;
                self.active_tex_width = 0;
                self.active_tex_height = 0;
                self.active_tex_handle = 0;
                self.active_tex_format_code = 0;
                self.active_tex_source = "cleared";
                self.active_tex_byte_len = 0;
                self.active_tex_srv_bound = false;
                self.active_tex_guest_raw = 0;
                self.active_tex_guest_norm = 0;
                self.active_tex_data = 0;
                self.active_tex_guest_format_code = 0;
                self.active_tex_swizzled_blocks = false;
                self.active_tex_alpha_min = 0;
                self.active_tex_alpha_max = 0;
                self.active_tex_alpha_nonzero_sample = 0;
                self.active_tex_rgb_nonzero_sample = 0;
                self.active_tex_sample_count = 0;
            }
        }
        #[cfg(not(windows))]
        {
            let _ = stage;
        }
    }

    #[cfg(windows)]
    fn set_texture_stage_state(&mut self, stage: u32, state: u32, value: u32) {
        if stage >= 4 || state >= 32 {
            return;
        }
        self.texture_stage_states[stage as usize][state as usize] = value;
        if matches!(
            state as usize,
            super::X_D3DTSS_ADDRESSU
                | super::X_D3DTSS_ADDRESSV
                | super::X_D3DTSS_ADDRESSW
                | super::X_D3DTSS_MAGFILTER
                | super::X_D3DTSS_MINFILTER
                | super::X_D3DTSS_MIPFILTER
        ) {
            self.bind_sampler_for_stage(stage);
        }
        if stage == 0
            && matches!(
                state as usize,
                super::X_D3DTSS_COLOROP
                    | super::X_D3DTSS_COLORARG0
                    | super::X_D3DTSS_COLORARG1
                    | super::X_D3DTSS_COLORARG2
                    | super::X_D3DTSS_ALPHAOP
                    | super::X_D3DTSS_ALPHAARG0
                    | super::X_D3DTSS_ALPHAARG1
                    | super::X_D3DTSS_ALPHAARG2
            )
        {
            self.mark_ps_constants_dirty();
        }
    }

    #[cfg(windows)]
    fn resolve_scanout(&mut self, pcrtc_start: u32) -> bool {
        self.resolve_scanout_impl(pcrtc_start)
    }

    #[cfg(windows)]
    fn copy_backbuffer_to_display(&mut self, pcrtc_start: u32) -> bool {
        self.resolve_scanout(pcrtc_start)
    }

    #[cfg(windows)]
    fn bind_display_as_render_target(&mut self, pcrtc_start: u32) -> bool {
        self.bind_display_as_render_target_impl(pcrtc_start)
    }

    #[cfg(windows)]
    fn restore_backbuffer_render_target(&mut self) {
        self.restore_backbuffer_render_target_impl();
    }

    #[cfg(windows)]
    fn bind_default_render_target(&mut self) {
        self.bind_default_render_target_impl("default");
    }

    #[cfg(windows)]
    fn present_display(&mut self, pcrtc_start: u32) -> &[u32] {
        self.present_display_impl(pcrtc_start)
    }

    #[cfg(not(windows))]
    fn resolve_scanout(&mut self, _pcrtc_start: u32) -> bool {
        false
    }

    #[cfg(windows)]
    fn readback_framebuffer(&mut self) -> &[u32] {
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return &self.readback_buf,
        };
        let frame_prof = d3d11_frame_prof_enabled();
        let readback_start = if frame_prof {
            D3D11_PROF_READBACK_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(std::time::Instant::now())
        } else {
            None
        };
        let pcrtc_start = crate::xbox::aot::nv2a::shadow_read(0x60_0800);
        let force_backbuffer = std::env::var("RUSTEMU_READBACK_BACKBUFFER")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false);
        let prefer_scanout = std::env::var("RUSTEMU_READBACK_SCANOUT")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
            || std::env::var("RUSTEMU_PRESENT_SCANOUT")
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false);
        let scanout_rt = if force_backbuffer || !prefer_scanout {
            None
        } else {
            self.scanout_render_target_resources(pcrtc_start)
        };
        let readback_source = if scanout_rt.is_some() {
            "scanout"
        } else if force_backbuffer {
            "backbuffer"
        } else {
            "active"
        };
        let Some((rt, _rtv, _dsv, stag, active_rt_key)) = scanout_rt.or_else(|| {
            if force_backbuffer {
                self.backbuffer_render_target_resources()
            } else {
                self.active_render_target_resources()
            }
        }) else {
            return &self.readback_buf;
        };

        unsafe {
            let out_w = self.width.max(0) as usize;
            let out_h = self.height.max(0) as usize;
            if out_w == 0 || out_h == 0 {
                return &self.readback_buf;
            }

            // Copy RT -> staging
            let copy_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            ctx.CopyResource(&stag, &rt);
            if let Some(start) = copy_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_COPY_US, start);
            }

            // Map staging for CPU read
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            let map_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            if ctx
                .Map(&stag, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
                .is_err()
            {
                return &self.readback_buf;
            }
            if let Some(start) = map_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_MAP_US, start);
            }
            let cpu_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };

            let needed = out_w.saturating_mul(out_h);
            if self.readback_buf.len() != needed {
                self.readback_buf.resize(needed, 0xFF00_0000);
            }
            self.readback_buf.fill(0xFF00_0000);

            let mut rt_desc = D3D11_TEXTURE2D_DESC::default();
            rt.GetDesc(&mut rt_desc);
            let rt_w = rt_desc.Width as usize;
            let rt_h = rt_desc.Height as usize;

            // Presentation readback must copy the full host frame, not the
            // guest's current draw viewport. Spider-Man switches through
            // 256x256/254x254 offscreen passes during scene loads; using that
            // viewport here makes RetroArch present stale or partial frames.
            let copy_x = 0usize;
            let copy_y = 0usize;
            let copy_w = rt_w.min(out_w);
            let copy_h = rt_h.min(out_h);

            static READBACK_LOG_N: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let log_n = READBACK_LOG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let log_pcrtc_every_swap = std::env::var("RUSTEMU_LOG_PCRTC_EVERY_SWAP")
                .map(|v| v == "1")
                .unwrap_or(false)
                || std::env::var("RUSTEMU_TEST_AUTOPRESS_START").is_ok();
            if log_pcrtc_every_swap || log_n < 8 || log_n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PCRTC-START] swap={} pcrtc_start=0x{:08X} active_rt=0x{:08X} forced_backbuffer={} source={} guest_vp={}x{}+{}+{} present_copy={}x{}+{}+{}",
                    log_n,
                    pcrtc_start,
                    active_rt_key,
                    force_backbuffer,
                    readback_source,
                    self.readback_width,
                    self.readback_height,
                    self.readback_x,
                    self.readback_y,
                    copy_w,
                    copy_h,
                    copy_x,
                    copy_y
                ));
            }
            if log_n < 8 || log_n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-READBACK] #{} rt={}x{} out={}x{} active_rt=0x{:08X} forced_backbuffer={} source={} pcrtc_start=0x{:08X} guest_vp={}x{}+{}+{} present_copy={}x{}+{}+{} rowpitch={}",
                    log_n,
                    rt_w,
                    rt_h,
                    out_w,
                    out_h,
                    active_rt_key,
                    force_backbuffer,
                    readback_source,
                    pcrtc_start,
                    self.readback_width,
                    self.readback_height,
                    self.readback_x,
                    self.readback_y,
                    copy_w,
                    copy_h,
                    copy_x,
                    copy_y,
                    mapped.RowPitch
                ));
            }

            // Copy row-by-row. D3D11 may pad mapped rows, and the guest may
            // render to only the active viewport height (Spider-Man: 640x256).
            let src = mapped.pData as *const u8;
            let dst = self.readback_buf.as_mut_ptr() as *mut u8;
            let dst_pitch = out_w * 4;
            let row_bytes = copy_w * 4;
            if row_bytes > 0 && copy_h > 0 {
                for y in 0..copy_h {
                    std::ptr::copy_nonoverlapping(
                        src.add((copy_y + y) * mapped.RowPitch as usize + copy_x * 4),
                        dst.add((copy_y + y) * dst_pitch + copy_x * 4),
                        row_bytes,
                    );
                }
            }
            ctx.Unmap(&stag, 0);

            let (rgb_nonblack, alpha_nonzero, sampled, _, _, _) =
                sample_pixel_stats(&self.readback_buf);
            self.present_cache.reconcile_after_readback(
                &mut self.readback_buf,
                self.black_clear_without_draw_pending,
                self.draws_since_clear,
                readback_source,
                active_rt_key,
                rgb_nonblack,
                alpha_nonzero,
                sampled,
            );
            self.apply_peter_candidate_overlay(out_w, out_h);
            let (rgb_nonblack, alpha_nonzero, sampled, first, mid, last) =
                sample_pixel_stats(&self.readback_buf);
            if log_pcrtc_every_swap || log_n < 8 || log_n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-READBACK-STATS] #{} source={} active_rt=0x{:08X} rgb_nonblack={}/{} alpha_nonzero={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X}",
                    log_n,
                    readback_source,
                    active_rt_key,
                    rgb_nonblack,
                    sampled,
                    alpha_nonzero,
                    sampled,
                    first,
                    mid,
                    last
                ));
            }

            maybe_record_present_frame(
                readback_source,
                out_w as u32,
                out_h as u32,
                &self.readback_buf,
            );
            if let Some(start) = cpu_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_CPU_US, start);
            }
            if let Some(start) = readback_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_TOTAL_US, start);
                d3d11_log_frame_prof(readback_source, active_rt_key, out_w, out_h);
            }
        }

        // Pump D3D11 debug messages once per readback (= once per Swap).
        self.pump_debug_messages();

        // 2026-04-25: removed the host-composed legal-notice overlay (was
        // arming on a 512x512 DXT5 bind and memcpy-overlaying a pre-baked
        // PowerShell BMP onto the readback). That was a synthetic shortcut
        // that masked organic GPU output. The DXT routing fixes in
        // self-healing SetTexture (oovpa_hle.rs:4248-4297) and the BC pitch
        // fix below mean compressed textures now upload correctly; the next
        // organic step is to mint multiple texture handles for the stash's
        // 7 DDS chunks (font atlas + 6 logos) so the game's draw sequence
        // binds the right texture per quad.

        &self.readback_buf
    }

    #[cfg(windows)]
    fn present_framebuffer(&mut self) -> &[u32] {
        self.present_framebuffer_impl()
    }

    #[cfg(not(windows))]
    fn present_framebuffer(&mut self) -> &[u32] {
        &self.readback_buf
    }

    #[cfg(not(windows))]
    fn readback_framebuffer(&mut self) -> &[u32] {
        &self.readback_buf
    }

    #[cfg(windows)]
    fn set_viewport(&mut self, x: u32, y: u32, w: u32, h: u32, min_z: f32, max_z: f32) {
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return,
        };
        let out_w = self.width.max(0) as u32;
        let out_h = self.height.max(0) as u32;
        let clamped_x = x.min(out_w);
        let clamped_y = y.min(out_h);
        let clamped_w = w.min(out_w.saturating_sub(clamped_x));
        let clamped_h = h.min(out_h.saturating_sub(clamped_y));
        if self.readback_x != clamped_x
            || self.readback_y != clamped_y
            || self.readback_width != clamped_w
            || self.readback_height != clamped_h
        {
            static VIEWPORT_CHANGE_N: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = VIEWPORT_CHANGE_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-READBACK-VP] #{} {}x{}+{}+{} -> {}x{}+{}+{} z=[{:.3},{:.3}] active_rt=0x{:08X} backbuffer=0x{:08X}",
                n,
                self.readback_width,
                self.readback_height,
                self.readback_x,
                self.readback_y,
                clamped_w,
                clamped_h,
                clamped_x,
                clamped_y,
                min_z,
                max_z,
                self.active_rt_key,
                self.backbuffer_rt_key
            ));
        }
        self.readback_x = clamped_x;
        self.readback_y = clamped_y;
        self.readback_width = clamped_w;
        self.readback_height = clamped_h;
        let vp = D3D11_VIEWPORT {
            TopLeftX: x as f32,
            TopLeftY: y as f32,
            Width: w as f32,
            Height: h as f32,
            MinDepth: min_z,
            MaxDepth: max_z,
        };
        unsafe {
            ctx.RSSetViewports(Some(&[vp]));
        }
    }

    #[cfg(windows)]
    fn set_transform(&mut self, state: u32, matrix: &[f32; 16]) {
        let Some(index) = Self::ffp_transform_index(state) else {
            return;
        };
        self.ffp_transforms[index] = *matrix;
        self.ffp_transform_mask |= 1 << index;

        static FFP_TRANSFORM_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = FFP_TRANSFORM_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 12 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-FFP-TRANSFORM] #{} state={} mask=0x{:X} row0=[{:.4},{:.4},{:.4},{:.4}]",
                n, state, self.ffp_transform_mask, matrix[0], matrix[1], matrix[2], matrix[3]
            ));
        }
    }

    #[cfg(windows)]
    fn set_render_state(&mut self, state: u32, value: u32) {
        self.apply_render_state(state, value);
    }

    #[cfg(windows)]
    fn set_vs_constant(&mut self, reg: u32, data: &[f32]) {
        let reg = reg as usize;
        if reg >= X_D3DVS_CONSTREG_COUNT || data.is_empty() {
            return;
        }
        for (i, chunk) in data.chunks(4).enumerate() {
            let dst = reg + i;
            if dst >= X_D3DVS_CONSTREG_COUNT {
                break;
            }
            for (j, value) in chunk.iter().copied().enumerate().take(4) {
                if (self.vs_constants[dst][j] - value).abs() > f32::EPSILON {
                    self.vs_constants[dst][j] = value;
                    self.mark_vs_constants_dirty();
                }
            }
        }
        let row_count = data.chunks(4).count().max(1);
        if peter_probe_json_enabled() {
            let seq = VS_CONSTANT_UPLOAD_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let preview_rows = row_count.min(8);
            let mut preview = Vec::with_capacity(preview_rows);
            for row in 0..preview_rows {
                preview.push(
                    self.vs_constants
                        .get(reg.saturating_add(row))
                        .copied()
                        .unwrap_or([0.0; 4]),
                );
            }
            if let Ok(mut uploads) = recent_vs_constant_uploads().lock() {
                uploads.push_back(RecentVsConstantUpload {
                    seq,
                    reg,
                    rows: row_count,
                    preview,
                });
                while uploads.len() > 128 {
                    uploads.pop_front();
                }
            }
        }
        if (reg..reg + row_count).any(|r| (4..=20).contains(&r) || (48..=72).contains(&r)) {
            static D3D11_VS_SKIN_CONST_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = D3D11_VS_SKIN_CONST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                let r0 = self.vs_constants.get(reg).copied().unwrap_or([0.0; 4]);
                let r1 = self
                    .vs_constants
                    .get(reg.saturating_add(1))
                    .copied()
                    .unwrap_or([0.0; 4]);
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-VS-CONST-SKIN] #{} c{} rows={} c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}]",
                    n,
                    reg,
                    row_count,
                    reg,
                    r0[0],
                    r0[1],
                    r0[2],
                    r0[3],
                    reg + 1,
                    r1[0],
                    r1[1],
                    r1[2],
                    r1[3],
                ));
            }
        }
    }

    fn set_ps_constant(&mut self, reg: u32, data: &[f32]) {
        let reg = reg as usize;
        if reg >= self.ps_constants.len() || data.is_empty() {
            return;
        }
        for (i, chunk) in data.chunks(4).enumerate() {
            let dst = reg + i;
            if dst >= self.ps_constants.len() {
                break;
            }
            self.ps_constant_written_mask |= 1u32 << dst;
            for (j, value) in chunk.iter().copied().enumerate().take(4) {
                if (self.ps_constants[dst][j] - value).abs() > 1.0e-6 {
                    self.ps_constants[dst][j] = value;
                    self.mark_ps_constants_dirty();
                }
            }
        }

        static D3D11_PS_CONST_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = D3D11_PS_CONST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            let c0 = self.ps_constants.get(reg).copied().unwrap_or([0.0; 4]);
            let c1 = self
                .ps_constants
                .get(reg.saturating_add(1))
                .copied()
                .unwrap_or([0.0; 4]);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-PS-CONST] #{} c{} rows={} c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}]",
                n,
                reg,
                (data.len() + 3) / 4,
                reg,
                c0[0],
                c0[1],
                c0[2],
                c0[3],
                reg + 1,
                c1[0],
                c1[1],
                c1[2],
                c1[3],
            ));
        }
    }

    #[cfg(windows)]
    fn set_vertex_shader_hlsl(&mut self, handle: u32, hlsl: Option<&str>) {
        if self.ctx.is_none() {
            return;
        }
        if handle == 0 || hlsl.is_none() {
            self.active_nv2a_vs = None;
            unsafe {
                if let (Some(ctx), Some(vs)) = (&self.ctx, &self.vs) {
                    ctx.VSSetShader(vs, None);
                }
            }
            return;
        }

        if !self.nv2a_vs_cache.contains_key(&handle) {
            let Some((compiled, layout)) = self.compile_nv2a_vertex_shader(handle, hlsl.unwrap())
            else {
                return;
            };
            self.nv2a_vs_cache.insert(handle, compiled);
            self.nv2a_layout_cache.insert(handle, layout);
        }
        let bound_vs = self.nv2a_vs_cache.get(&handle).cloned();
        if let Some(vs) = bound_vs {
            self.active_nv2a_vs = Some(handle);
            if spidey_cxbxr_skin_c1_canary_enabled()
                && !spidey_skin_raw4_probe_enabled()
                && self.vs_constants[1] == [0.0, 1.0, 0.0, 0.0]
            {
                // Cxbx-R translates Spider-Man's stripped skinning operand as
                // c(-95).x/y. Its runtime log for the Peter draw uploads the
                // bone palette at signed c[-79], c[-76], ...; map normalized
                // PBYTE4 index 1/255 to c[-79] while this remains gated.
                let base_signed = std::env::var("RUSTEMU_SPIDEY_CXBXR_SKIN_C1_BASE_SIGNED")
                    .ok()
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(-79);
                let scale = std::env::var("RUSTEMU_SPIDEY_CXBXR_SKIN_C1_SCALE")
                    .ok()
                    .and_then(|value| value.trim().parse::<f32>().ok())
                    .unwrap_or(765.0);
                let bias = std::env::var("RUSTEMU_SPIDEY_CXBXR_SKIN_C1_BIAS")
                    .ok()
                    .and_then(|value| value.trim().parse::<f32>().ok())
                    .unwrap_or(base_signed as f32 - 3.0);
                self.vs_constants[1] = [scale, bias, 0.0, 0.0];
                self.mark_vs_constants_dirty();
                static C1_CANARY_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let k = C1_CANARY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if k < 16 || k.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-SPIDEY-CXBXR-SKIN-C1-CANARY] #{} handle=0x{:08X} base_signed={} c1=[{:.3},{:.3},0,0]",
                        k, handle, base_signed, scale, bias
                    ));
                }
            }
            self.upload_vs_constants_if_dirty();
            unsafe {
                if let Some(ctx) = &self.ctx {
                    ctx.VSSetShader(&vs, None);
                }
            }
            static BIND_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-VSH-BIND] #{} handle=0x{:08X}",
                    n, handle
                ));
            }
        }
    }

    fn black_clear_without_draw_pending(&self) -> bool {
        self.black_clear_without_draw_pending
    }

    fn name(&self) -> &'static str {
        "D3D11"
    }
}
