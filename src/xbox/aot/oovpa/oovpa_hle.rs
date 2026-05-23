use super::{
    read_dev_ptr, DEV_FRAME_CTR, DEV_PB_BASE, DEV_PB_GET, DEV_PB_LIMIT, DEV_PB_PUT, DEV_VSHADER,
    G_PDEVICE, G_PDEVICE_DYNAMIC, OOVPA_DRAW_CALLS, OOVPA_MMIO_COUNT, OOVPA_PB_COMMANDS,
    OOVPA_SWAP_COUNT, PB_DUMMY_BASE, PB_DUMMY_SIZE, STREAM0_STRIDE, STREAM0_VB_PTR, XDK_BUILD,
};
/// OOVPA HLE (High-Level Emulation) stubs — execute_hle(), execute_manual_hle(), and all helper functions.
/// Split from oovpa.rs for modularity.
use crate::xbox::emulator::debug_log;
use crate::xbox::gpu::{NV2AVertex, VertexSkinPayload};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const HLE_HEAP_POOL_BASE: u32 = 0x1000_0000;
const HLE_HEAP_POOL_END: u32 = 0x1C00_0000;
static HLE_HEAP_BUMP: AtomicU32 = AtomicU32::new(HLE_HEAP_POOL_BASE);
static HLE_HEAP_LOCK: Mutex<()> = Mutex::new(());
static HLE_HEAP_FREE_LIST: OnceLock<Mutex<Vec<(u32, u32)>>> = OnceLock::new();
static HLE_HEAP_ALLOC_SIZES: OnceLock<Mutex<Vec<(u32, u32)>>> = OnceLock::new();

const VS_CONSTANT_COUNT: usize = 192;
const X_D3DSCM_192CONSTANTS: u32 = 0x01;
const X_D3DSCM_NORESERVEDCONSTANTS: u32 = 0x10;
static VS_CONSTANTS: OnceLock<Mutex<[[f32; 4]; VS_CONSTANT_COUNT]>> = OnceLock::new();
static VS_WVP_CANDIDATE: OnceLock<Mutex<[[f32; 4]; 4]>> = OnceLock::new();
static VS_WVP_CANDIDATES: OnceLock<Mutex<Vec<MatrixCandidate>>> = OnceLock::new();
static VS_MATRIX_SEQ: AtomicU32 = AtomicU32::new(0);
static VS_VIEWPORT: OnceLock<Mutex<ShaderViewport>> = OnceLock::new();
static SHADER_CONSTANT_MODE: AtomicU32 = AtomicU32::new(X_D3DSCM_192CONSTANTS);
static CXBXR_RESERVED_VS_CONSTANTS: OnceLock<bool> = OnceLock::new();
static PENDING_SHADER_SOURCES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static VS_SHADER_INFOS: OnceLock<Mutex<Vec<VertexShaderInfo>>> = OnceLock::new();
static PS_RENDER_STATES: OnceLock<Mutex<[u32; 137]>> = OnceLock::new();
static PS_CONSTANTS: OnceLock<Mutex<[[f32; 4]; 32]>> = OnceLock::new();
static PS_RENDER_STATE_SEQ: AtomicU32 = AtomicU32::new(0);
static PS_MAPPING_VALID: AtomicBool = AtomicBool::new(false);
static CURRENT_PIXEL_SHADER_HANDLE: AtomicU32 = AtomicU32::new(0);
static TEXTURE_STAGE_STATES: OnceLock<Mutex<[[u32; 32]; 4]>> = OnceLock::new();
static TEXTURE_STAGE_STATE_SEQ: AtomicU32 = AtomicU32::new(0);
static LIGHT_ENABLE_MASK: AtomicU32 = AtomicU32::new(0);
static LIGHT_STATE_SEQ: AtomicU32 = AtomicU32::new(0);
static MATERIAL_STATE_SEQ: AtomicU32 = AtomicU32::new(0);
static CURRENT_STAGE0_TEXTURE: AtomicU32 = AtomicU32::new(0);
static CURRENT_STAGE0_TEXTURE_RAW: AtomicU32 = AtomicU32::new(0);
static CURRENT_STAGE0_TEXTURE_DATA: AtomicU32 = AtomicU32::new(0);
static CURRENT_STAGE0_TEXTURE_FORMAT: AtomicU32 = AtomicU32::new(0);
const SPIDEY_SELECT_CIRCLE_TEX_RAW: u32 = 0x83E0_8570;
const SPIDEY_SELECT_CIRCLE_TEX_NORM: u32 = 0x03E0_8570;
const SPIDEY_RED_FOCUS_TEX_RAW: u32 = 0x83E2_1930;
const SPIDEY_RED_FOCUS_TEX_NORM: u32 = 0x03E2_1930;
static SPIDEY_MENU_SELECTOR_SEEN: AtomicBool = AtomicBool::new(false);
static SPIDEY_VISIBLE_MENU_CONFIRM_STARTED: AtomicBool = AtomicBool::new(false);
static SPIDEY_MENU_READY_FOR_CONFIRM: AtomicBool = AtomicBool::new(false);
static SPIDEY_MENU_SELECTOR_PRESENT_CACHE: Mutex<Option<Vec<u32>>> = Mutex::new(None);
static SPIDEY_TITLE_UI_TEX_TRACE_N: AtomicU32 = AtomicU32::new(0);
static SPIDEY_MANUAL_CONFIRM_EDGES: AtomicU32 = AtomicU32::new(0);
static SPIDEY_MANUAL_CONFIRM_HELD: AtomicU32 = AtomicU32::new(0);
static SPIDEY_MANUAL_SCRIPT_ACTIVE: AtomicU32 = AtomicU32::new(0);
static SPIDEY_MANUAL_SCRIPT_PREV_CONFIRM: AtomicU32 = AtomicU32::new(0);
static SPIDEY_MANUAL_SCRIPT_ARM_LOG: AtomicU32 = AtomicU32::new(0);
static SPIDEY_START_HOLD_POLLS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_POST_WAIT_A_POLLS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_LEVEL_CONFIRM_POLLS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_LEVEL_CONFIRM_STARTED: AtomicU32 = AtomicU32::new(0);
static SPIDEY_LEVEL_CONFIRM_LAST_STATE: AtomicU32 = AtomicU32::new(u32::MAX);
static SPIDEY_LEVEL_CONFIRM_ORIGIN_READY_POLLS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ORIGINZ_SUSTAIN_POLLS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ORIGINZ_LAST_VALID_STATE: AtomicU32 = AtomicU32::new(u32::MAX);
static SPIDEY_ORIGINZ_STATE_FALLBACK_LOG: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ORIGINZ_STATE_POKE_LOG: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ORIGINZ_STATE4_040A_HLE_LOG: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ORIGINZ_STATE4_040A_WAIT_CLEAR_LOG: AtomicU32 = AtomicU32::new(0);

const X_D3DSWAP_DEFAULT: u32 = 0;
const X_D3DSWAP_COPY: u32 = 1;
const X_D3DSWAP_BYPASSCOPY: u32 = 2;
const X_D3DSWAP_FINISH: u32 = 4;
static XBOX_SWAP_COPY_PENDING: AtomicBool = AtomicBool::new(false);

pub(crate) fn note_spidey_menu_selector_seen(source: &str, draw_index: u32) {
    if !SPIDEY_MENU_SELECTOR_SEEN.swap(true, Ordering::Relaxed) {
        debug_log(&format!(
            "[SPIDEY-MENU-SELECTOR-SEEN] source={} draw={}",
            source, draw_index
        ));
    }
}

pub(crate) fn spidey_menu_selector_seen() -> bool {
    SPIDEY_MENU_SELECTOR_SEEN.load(Ordering::Relaxed)
}

pub(super) fn note_spidey_manual_confirm_edge() {
    let _ =
        SPIDEY_MANUAL_CONFIRM_EDGES.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |edges| {
            Some(edges.saturating_add(1).min(8))
        });
}

pub(super) fn take_spidey_manual_confirm_edge() -> bool {
    SPIDEY_MANUAL_CONFIRM_EDGES
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |edges| {
            if edges == 0 {
                None
            } else {
                Some(edges - 1)
            }
        })
        .is_ok()
}

pub(super) fn spidey_manual_confirm_held() -> bool {
    SPIDEY_MANUAL_CONFIRM_HELD.load(Ordering::Relaxed) != 0
}

fn spidey_vsh_call_trace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_VSH_CALL_TRACE"))
}

fn cpu_vs_xform_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_CPU_VS_XFORM")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    })
}

fn cpu_vs_flatten_z_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_CPU_VS_FLATTEN_Z")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
    })
}

fn cpu_vs_exact_dp4_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_CPU_VS_EXACT_DP4")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
    })
}

fn cpu_vs_clip_output_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_CPU_VS_CLIP_OUTPUT")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
    })
}

fn spidey_drop_wild_cpu_xform_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_DROP_WILD_CPU_XFORM")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
    })
}

fn spidey_isolate_drawidx_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_ISOLATE_DRAWIDX")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false)
    })
}

fn spidey_skin_draw_probe_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_SKIN_DRAW_PROBE"))
}

fn parse_u32_env_token(token: &str) -> Option<u32> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    if let Some(hex) = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        token.parse::<u32>().ok()
    }
}

fn spidey_focus_texture_list() -> &'static [u32] {
    static FOCUS_TEXTURES: OnceLock<Vec<u32>> = OnceLock::new();
    FOCUS_TEXTURES.get_or_init(|| {
        let mut out = vec![SPIDEY_RED_FOCUS_TEX_NORM, SPIDEY_RED_FOCUS_TEX_RAW];
        if let Ok(raw) = std::env::var("RUSTEMU_SPIDEY_FOCUS_TEX") {
            out.extend(
                raw.split(|c: char| c == ',' || c == ';' || c == ':' || c.is_whitespace())
                    .filter_map(parse_u32_env_token),
            );
        }
        out.sort_unstable();
        out.dedup();
        out
    })
}

fn spidey_focus_texture_match(stage: u32, tex_raw: u32, tex_norm: u32) -> bool {
    if stage != 0 {
        return false;
    }
    let focus = spidey_focus_texture_list();
    focus.binary_search(&tex_raw).is_ok() || focus.binary_search(&tex_norm).is_ok()
}

fn spidey_capture_hle_drawidx_list() -> &'static [u32] {
    static CAPTURES: OnceLock<Vec<u32>> = OnceLock::new();
    CAPTURES.get_or_init(|| {
        let Ok(raw) = std::env::var("RUSTEMU_SPIDEY_CAPTURE_HLE_DRAWIDX_RANGE") else {
            return Vec::new();
        };
        let nums: Vec<u32> = raw
            .split(|c: char| c == ',' || c == ';' || c == ':' || c.is_whitespace())
            .filter_map(parse_u32_env_token)
            .collect();
        if nums.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::new();
        if nums.len() >= 2 {
            let start = nums[0].min(nums[1]);
            let end = nums[0].max(nums[1]);
            let step = nums.get(2).copied().unwrap_or(1).max(1);
            let mut draw = start;
            while draw <= end && out.len() < 256 {
                out.push(draw);
                match draw.checked_add(step) {
                    Some(next) if next > draw => draw = next,
                    _ => break,
                }
            }
        } else {
            out.push(nums[0]);
        }
        out.sort_unstable();
        out.dedup();
        out
    })
}

fn spidey_capture_hle_drawidx_requested(draw_no: u32) -> bool {
    spidey_capture_hle_drawidx_list()
        .binary_search(&draw_no)
        .is_ok()
}

const CPU_VS_CLIP_PASSTHROUGH_HLSL: &str = r#"
struct VS_IN {
    float4 pos : POSITION;
    uint color : COLOR;
    float2 uv : TEXCOORD;
    float4 blend_indices : BLENDINDICES;
    float4 blend_weight : BLENDWEIGHT;
    float4 normal : NORMAL;
    float4 uv1 : TEXCOORD1;
};
struct VS_OUT {
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
    float4 normal_passthrough : TEXCOORD1;
    float4 uv1_passthrough : TEXCOORD2;
};
float4 unpack_color(uint argb) {
    return float4(
        (float)((argb >> 16) & 255u),
        (float)((argb >> 8) & 255u),
        (float)(argb & 255u),
        (float)((argb >> 24) & 255u)) / 255.0;
}
VS_OUT main(VS_IN input) {
    VS_OUT outp;
    outp.pos = input.pos;
    outp.color = unpack_color(input.color);
    outp.uv = input.uv;
    outp.normal_passthrough = input.normal;
    outp.uv1_passthrough = input.uv1;
    return outp;
}
"#;

#[derive(Clone, Copy)]
struct ShaderViewport {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    min_z: f32,
    max_z: f32,
}

#[derive(Clone, Copy)]
struct MatrixCandidate {
    seq: u32,
    src: u32,
    rows: [[f32; 4]; 4],
}

#[derive(Clone, Copy)]
enum TransformMode {
    RowClip,
    ColClip,
    RowScreen,
    ColScreen,
}

impl TransformMode {
    fn label(self) -> &'static str {
        match self {
            Self::RowClip => "row+clip",
            Self::ColClip => "col+clip",
            Self::RowScreen => "row+screen",
            Self::ColScreen => "col+screen",
        }
    }
}

#[derive(Clone, Copy)]
struct TransformStats {
    finite: usize,
    onscreen: usize,
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    score: f32,
}

impl TransformStats {
    fn width(self) -> f32 {
        (self.max_x - self.min_x).max(0.0)
    }

    fn height(self) -> f32 {
        (self.max_y - self.min_y).max(0.0)
    }

    fn area(self) -> f32 {
        self.width() * self.height()
    }
}

#[derive(Clone, Copy)]
struct SelectedTransform {
    matrix: MatrixCandidate,
    mode: TransformMode,
    viewport: ShaderViewport,
    stats: TransformStats,
}

#[derive(Clone)]
struct VertexShaderInfo {
    handle: u32,
    screenspace: bool,
    simple_dp4_o_pos: bool,
    passes_diffuse: bool,
    diffuse_mul_c0: bool,
    passes_tex0: bool,
    tex0_add_c0: bool,
    skinned: bool,
    decl: VertexDeclInfo,
    snippet: String,
}

#[derive(Clone, Copy, Default)]
struct VertexDeclElement {
    data_type: u8,
    offset: u32,
    size: u32,
}

#[derive(Clone, Copy, Default)]
struct VertexDeclInfo {
    stride: u32,
    v2: Option<VertexDeclElement>,
    v4: Option<VertexDeclElement>,
    v5: Option<VertexDeclElement>,
    v6: Option<VertexDeclElement>,
}

#[derive(Clone, Copy, Debug, Default)]
struct PixelShaderStateSnapshot {
    seq: u32,
    alpha_inputs0: u32,
    final_abcd: u32,
    final_efg: u32,
    rgb_inputs0: u32,
    compare_mode: u32,
    final_constant0: u32,
    final_constant1: u32,
    rgb_outputs0: u32,
    combiner_count: u32,
    ps_reserved: u32,
    dot_mapping: u32,
    input_texture: u32,
    texture_modes: u32,
}

#[derive(Clone, Copy)]
struct TextureStageSnapshot {
    seq: u32,
    states: [[u32; 32]; 4],
}

fn vs_constants() -> &'static Mutex<[[f32; 4]; VS_CONSTANT_COUNT]> {
    VS_CONSTANTS.get_or_init(|| Mutex::new([[0.0; 4]; VS_CONSTANT_COUNT]))
}

fn vs_wvp_candidate() -> &'static Mutex<[[f32; 4]; 4]> {
    VS_WVP_CANDIDATE.get_or_init(|| Mutex::new([[0.0; 4]; 4]))
}

fn vs_wvp_candidates() -> &'static Mutex<Vec<MatrixCandidate>> {
    VS_WVP_CANDIDATES.get_or_init(|| Mutex::new(Vec::new()))
}

fn ps_render_states() -> &'static Mutex<[u32; 137]> {
    PS_RENDER_STATES.get_or_init(|| Mutex::new([0; 137]))
}

fn ps_constants() -> &'static Mutex<[[f32; 4]; 32]> {
    PS_CONSTANTS.get_or_init(|| Mutex::new([[0.0; 4]; 32]))
}

fn texture_stage_states() -> &'static Mutex<[[u32; 32]; 4]> {
    TEXTURE_STAGE_STATES
        .get_or_init(|| Mutex::new(crate::xbox::gpu::default_texture_stage_states()))
}

fn shader_viewport() -> &'static Mutex<ShaderViewport> {
    VS_VIEWPORT.get_or_init(|| {
        Mutex::new(ShaderViewport {
            x: 0.0,
            y: 0.0,
            w: 640.0,
            h: 480.0,
            min_z: 0.0,
            max_z: 1.0,
        })
    })
}

fn shader_constant_mode() -> u32 {
    SHADER_CONSTANT_MODE.load(Ordering::Relaxed)
}

fn shader_constant_mode_uses_reserved_constants() -> bool {
    (shader_constant_mode() & X_D3DSCM_NORESERVEDCONSTANTS) == 0
}

fn cxbxr_reserved_vs_constants_enabled() -> bool {
    *CXBXR_RESERVED_VS_CONSTANTS.get_or_init(|| {
        crate::xbox::emulator::spidey_synth_training_enabled()
            || spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_C_HLE")
            || spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_HLE_CXBXR")
            || std::env::var("RUSTEMU_CXBXR_RESERVED_VS_CONSTANTS")
                .map(|v| {
                    let v = v.trim();
                    v == "1"
                        || v.eq_ignore_ascii_case("true")
                        || v.eq_ignore_ascii_case("yes")
                        || v.eq_ignore_ascii_case("on")
                })
                .unwrap_or(false)
    })
}

fn spidey_vs_const_source_probe_enabled() -> bool {
    use std::sync::OnceLock;
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        crate::xbox::emulator::spidey_synth_training_enabled()
            || std::env::var("RUSTEMU_SPIDEY_CONST_SOURCE_PROBE")
                .map(|v| {
                    let v = v.trim();
                    v == "1"
                        || v.eq_ignore_ascii_case("true")
                        || v.eq_ignore_ascii_case("yes")
                        || v.eq_ignore_ascii_case("on")
                })
                .unwrap_or(false)
    })
}

fn plausible_shader_constant_mode(mode: u32) -> bool {
    (mode & !(X_D3DSCM_192CONSTANTS | 0x02 | X_D3DSCM_NORESERVEDCONSTANTS)) == 0
}

pub(super) fn hle_note_shader_constant_mode(source: &str, mode: u32) {
    if !plausible_shader_constant_mode(mode) {
        static BAD_MODE_LOG: AtomicU32 = AtomicU32::new(0);
        let n = BAD_MODE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE] SetShaderConstantMode/{source}: ignored implausible mode=0x{mode:08X}"
            ));
        }
        return;
    }

    SHADER_CONSTANT_MODE.store(mode, Ordering::Relaxed);
    let noreserved = (mode & X_D3DSCM_NORESERVEDCONSTANTS) != 0;
    static SCM_NOTE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = SCM_NOTE_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE] SetShaderConstantMode/{source}: mode=0x{mode:08X} noreserved={}",
            noreserved as u32
        ));
    }

    if shader_constant_mode_uses_reserved_constants() {
        let vp = *shader_viewport().lock().unwrap_or_else(|e| e.into_inner());
        upload_reserved_screenspace_constants(vp);
    }
}

fn stack_arg0(context: &windows::Win32::System::Diagnostics::Debug::CONTEXT) -> Option<u32> {
    let arg = (context.R14 as u32).checked_add(4)?;
    if arg.checked_add(4)? > 0x2000_0000 {
        return None;
    }
    unsafe { Some(*((context.R15 + arg as u64) as *const u32)) }
}

fn is_pixel_shader_render_state(state: u32) -> bool {
    // XDK/Cxbx-R: X_D3DRS_PSALPHAINPUTS0..X_D3DRS_PSINPUTTEXTURE
    // occupy slots 0..56. X_D3DPIXELSHADERDEF appends software mapping
    // words at 57..59. PSTextureModes is stored out-of-range at slot 136.
    state <= 59 || state == 136
}

fn pixel_shader_render_state_name(state: u32) -> String {
    match state {
        0..=7 => format!("D3DRS_PSALPHAINPUTS{}", state),
        8 => "D3DRS_PSFINALCOMBINERINPUTSABCD".to_string(),
        9 => "D3DRS_PSFINALCOMBINERINPUTSEFG".to_string(),
        10..=17 => format!("D3DRS_PSCONSTANT0_{}", state - 10),
        18..=25 => format!("D3DRS_PSCONSTANT1_{}", state - 18),
        26..=33 => format!("D3DRS_PSALPHAOUTPUTS{}", state - 26),
        34..=41 => format!("D3DRS_PSRGBINPUTS{}", state - 34),
        42 => "D3DRS_PSCOMPAREMODE".to_string(),
        43 => "D3DRS_PSFINALCOMBINERCONSTANT0".to_string(),
        44 => "D3DRS_PSFINALCOMBINERCONSTANT1".to_string(),
        45..=52 => format!("D3DRS_PSRGBOUTPUTS{}", state - 45),
        53 => "D3DRS_PSCOMBINERCOUNT".to_string(),
        54 => "D3DRS_PSTEXTUREMODES_RESERVED".to_string(),
        55 => "D3DRS_PSDOTMAPPING".to_string(),
        56 => "D3DRS_PSINPUTTEXTURE".to_string(),
        57 => "D3DRS_PSC0MAPPING".to_string(),
        58 => "D3DRS_PSC1MAPPING".to_string(),
        59 => "D3DRS_PSFINALCOMBINERCONSTANTS".to_string(),
        136 => "D3DRS_PSTEXTUREMODES".to_string(),
        _ => format!("D3DRS_{}", state),
    }
}

pub(super) fn render_state_slot_from_fastcall_reg(reg: u32) -> u32 {
    crate::xbox::gpu::render_state::normalize_render_state_input(reg, 0).0
}

pub(super) fn hle_note_pixel_shader_render_state(source: &str, state: u32, value: u32) {
    if !is_pixel_shader_render_state(state) {
        return;
    }

    if matches!(state, 57 | 58 | 59) {
        PS_MAPPING_VALID.store(true, Ordering::Relaxed);
    }

    if let Ok(mut states) = ps_render_states().lock() {
        states[state as usize] = value;
    }
    let seq = PS_RENDER_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;

    static PS_RS_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PS_RS_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 256 || n.is_power_of_two() || (matches!(state, 53 | 136) && n % 256 == 0) {
        let tex_modes = if state == 136 {
            format!(
                " modes=[{},{},{},{}]",
                value & 0x1F,
                (value >> 5) & 0x1F,
                (value >> 10) & 0x1F,
                (value >> 15) & 0x1F
            )
        } else {
            String::new()
        };
        debug_log(&format!(
            "[PS-RS] #{} src={} state={} {} value=0x{:08X}{}",
            seq,
            source,
            state,
            pixel_shader_render_state_name(state),
            value,
            tex_modes
        ));
    }
}

pub(super) fn hle_apply_interpreter_render_state(name: &str, state: u32, value: u32) -> u32 {
    let (state, value) = crate::xbox::gpu::render_state::normalize_render_state_input(state, value);
    drain_draws_before_state_change(name);
    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_render_state(state, value);
    }
    let group = crate::xbox::gpu::render_state::apply_global(state, value);
    hle_note_pixel_shader_render_state(name, state, value);

    static INTERP_RS_LOG: AtomicU32 = AtomicU32::new(0);
    let n = INTERP_RS_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 24 && (is_pixel_shader_render_state(state) || group.is_some()) {
        debug_log(&format!(
            "[INTERP-HLE] {}: state={} val=0x{:08X} group={:?} (#{})",
            name, state, value, group, n
        ));
    }
    0
}

fn pixel_shader_state_snapshot() -> PixelShaderStateSnapshot {
    let states = ps_render_states().lock().unwrap_or_else(|e| e.into_inner());
    PixelShaderStateSnapshot {
        seq: PS_RENDER_STATE_SEQ.load(Ordering::Relaxed),
        alpha_inputs0: states[0],
        final_abcd: states[8],
        final_efg: states[9],
        rgb_inputs0: states[34],
        compare_mode: states[42],
        final_constant0: states[43],
        final_constant1: states[44],
        rgb_outputs0: states[45],
        combiner_count: states[53],
        ps_reserved: states[54],
        dot_mapping: states[55],
        input_texture: states[56],
        texture_modes: states[136],
    }
}

fn log_pixel_shader_draw_snapshot(tag: &str, draw_no: u32) {
    static PS_DRAW_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PS_DRAW_LOG.fetch_add(1, Ordering::Relaxed);
    if n >= 64 && !n.is_power_of_two() {
        return;
    }

    let ps = pixel_shader_state_snapshot();
    debug_log(&format!(
        "[PS-RS-DRAW] {}#{} seq={} combiner=0x{:08X} texmodes=0x{:08X} modes=[{},{},{},{}] rgb0=0x{:08X} alpha0=0x{:08X} out0=0x{:08X} final=[0x{:08X},0x{:08X}] fc=[0x{:08X},0x{:08X}] dot=0x{:08X} input=0x{:08X} compare=0x{:08X} reserved=0x{:08X}",
        tag,
        draw_no,
        ps.seq,
        ps.combiner_count,
        ps.texture_modes,
        ps.texture_modes & 0x1F,
        (ps.texture_modes >> 5) & 0x1F,
        (ps.texture_modes >> 10) & 0x1F,
        (ps.texture_modes >> 15) & 0x1F,
        ps.rgb_inputs0,
        ps.alpha_inputs0,
        ps.rgb_outputs0,
        ps.final_abcd,
        ps.final_efg,
        ps.final_constant0,
        ps.final_constant1,
        ps.dot_mapping,
        ps.input_texture,
        ps.compare_mode,
        ps.ps_reserved
    ));
}

fn texture_stage_state_name(state: u32) -> &'static str {
    match state {
        0 => "D3DTSS_ADDRESSU",
        1 => "D3DTSS_ADDRESSV",
        2 => "D3DTSS_ADDRESSW",
        3 => "D3DTSS_MAGFILTER",
        4 => "D3DTSS_MINFILTER",
        5 => "D3DTSS_MIPFILTER",
        6 => "D3DTSS_MIPMAPLODBIAS",
        7 => "D3DTSS_MAXMIPLEVEL",
        8 => "D3DTSS_MAXANISOTROPY",
        9 => "D3DTSS_COLORKEYOP",
        10 => "D3DTSS_COLORSIGN",
        11 => "D3DTSS_ALPHAKILL",
        12 => "D3DTSS_COLOROP",
        13 => "D3DTSS_COLORARG0",
        14 => "D3DTSS_COLORARG1",
        15 => "D3DTSS_COLORARG2",
        16 => "D3DTSS_ALPHAOP",
        17 => "D3DTSS_ALPHAARG0",
        18 => "D3DTSS_ALPHAARG1",
        19 => "D3DTSS_ALPHAARG2",
        20 => "D3DTSS_RESULTARG",
        21 => "D3DTSS_TEXTURETRANSFORMFLAGS",
        22 => "D3DTSS_BUMPENVMAT00",
        23 => "D3DTSS_BUMPENVMAT01",
        24 => "D3DTSS_BUMPENVMAT11",
        25 => "D3DTSS_BUMPENVMAT10",
        26 => "D3DTSS_BUMPENVLSCALE",
        27 => "D3DTSS_BUMPENVLOFFSET",
        28 => "D3DTSS_TEXCOORDINDEX",
        29 => "D3DTSS_BORDERCOLOR",
        30 => "D3DTSS_COLORKEYCOLOR",
        31 => "D3DTSS_UNSUPPORTED",
        _ => "D3DTSS_UNKNOWN",
    }
}

fn texture_op_name(value: u32) -> &'static str {
    match value {
        0 => "ZERO/UNSET",
        1 => "DISABLE",
        2 => "SELECTARG1",
        3 => "SELECTARG2",
        4 => "MODULATE",
        5 => "MODULATE2X",
        6 => "MODULATE4X",
        7 => "ADD",
        8 => "ADDSIGNED",
        9 => "ADDSIGNED2X",
        10 => "SUBTRACT",
        11 => "ADDSMOOTH",
        12 => "BLENDDIFFUSEALPHA",
        13 => "BLENDCURRENTALPHA",
        14 => "BLENDTEXTUREALPHA",
        15 => "BLENDFACTORALPHA",
        16 => "BLENDTEXTUREALPHAPM",
        17 => "PREMODULATE",
        18 => "MODULATEALPHA_ADDCOLOR",
        19 => "MODULATECOLOR_ADDALPHA",
        20 => "MODULATEINVALPHA_ADDCOLOR",
        21 => "MODULATEINVCOLOR_ADDALPHA",
        22 => "DOTPRODUCT3",
        23 => "MULTIPLYADD",
        24 => "LERP",
        25 => "BUMPENVMAP",
        26 => "BUMPENVMAPLUMINANCE",
        _ => "OP_UNKNOWN",
    }
}

fn texture_address_name(value: u32) -> &'static str {
    match value {
        0 => "ZERO/UNSET",
        1 => "WRAP",
        2 => "MIRROR",
        3 => "CLAMP",
        4 => "BORDER",
        5 => "CLAMPTOEDGE",
        _ => "ADDR_UNKNOWN",
    }
}

fn texture_filter_name(value: u32) -> &'static str {
    match value {
        0 => "NONE",
        1 => "POINT",
        2 => "LINEAR",
        3 => "ANISOTROPIC",
        4 => "QUINCUNX",
        5 => "GAUSSIANCUBIC",
        _ => "FILTER_UNKNOWN",
    }
}

fn texture_arg_name(value: u32) -> String {
    let base = match value & 0x7 {
        0 => "DIFFUSE",
        1 => "CURRENT",
        2 => "TEXTURE",
        3 => "TFACTOR",
        4 => "SPECULAR",
        5 => "TEMP",
        6 => "CONSTANT",
        _ => "ARG_UNKNOWN",
    };
    let mut s = base.to_string();
    if value & 0x10 != 0 {
        s.push_str("|COMPLEMENT");
    }
    if value & 0x20 != 0 {
        s.push_str("|ALPHAREPLICATE");
    }
    s
}

fn texture_stage_value_name(state: u32, value: u32) -> String {
    match state {
        0..=2 => texture_address_name(value).to_string(),
        3..=5 => texture_filter_name(value).to_string(),
        12 | 16 => texture_op_name(value).to_string(),
        13..=15 | 17..=20 => texture_arg_name(value),
        21 => match value {
            0 => "DISABLE".to_string(),
            1 => "COUNT1".to_string(),
            2 => "COUNT2".to_string(),
            3 => "COUNT3".to_string(),
            4 => "COUNT4".to_string(),
            v => format!("FLAGS_0x{v:08X}"),
        },
        28 => format!("TEXCOORD{}", value & 0xFFFF),
        _ => format!("0x{value:08X}"),
    }
}

fn should_log_texture_stage_state(stage: u32, state: u32, seq: u32) -> bool {
    const INTERESTING: &[u32] = &[
        0, 1, 2, 3, 4, 5, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 28, 29, 30,
    ];
    stage == 0 || seq <= 96 || INTERESTING.contains(&state) || seq.is_power_of_two()
}

pub(super) fn hle_note_texture_stage_state(
    source: &str,
    stage: u32,
    state: u32,
    value: u32,
) -> u32 {
    let seq = TEXTURE_STAGE_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    if stage < 4 && state < 32 {
        drain_draws_before_state_change(source);
        if let Ok(mut stages) = texture_stage_states().lock() {
            stages[stage as usize][state as usize] = value;
        }
        if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
            gpu.set_texture_stage_state(stage, state, value);
        }
    }

    if should_log_texture_stage_state(stage, state, seq) {
        debug_log(&format!(
            "[TSS] #{} src={} stage={} state={} {} value=0x{:08X} {}",
            seq,
            source,
            stage,
            state,
            texture_stage_state_name(state),
            value,
            texture_stage_value_name(state, value)
        ));
    }
    0
}

fn texture_stage_snapshot() -> TextureStageSnapshot {
    let states = texture_stage_states()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    TextureStageSnapshot {
        seq: TEXTURE_STAGE_STATE_SEQ.load(Ordering::Relaxed),
        states: *states,
    }
}

fn log_texture_stage_draw_snapshot(tag: &str, draw_no: u32) {
    static TSS_DRAW_LOG: AtomicU32 = AtomicU32::new(0);
    let n = TSS_DRAW_LOG.fetch_add(1, Ordering::Relaxed);
    if n >= 64 && !n.is_power_of_two() {
        return;
    }

    let tss = texture_stage_snapshot();
    let s0 = &tss.states[0];
    let s1 = &tss.states[1];
    debug_log(&format!(
        "[TSS-DRAW] {}#{} seq={} s0_addr={}/{} s0_filter={}/{}/{} s0_color={}({},{},{}) s0_alpha={}({},{},{}) s0_xform={} s0_tc={} s1_color={} s1_alpha={}",
        tag,
        draw_no,
        tss.seq,
        texture_address_name(s0[0]),
        texture_address_name(s0[1]),
        texture_filter_name(s0[3]),
        texture_filter_name(s0[4]),
        texture_filter_name(s0[5]),
        texture_op_name(s0[12]),
        texture_arg_name(s0[13]),
        texture_arg_name(s0[14]),
        texture_arg_name(s0[15]),
        texture_op_name(s0[16]),
        texture_arg_name(s0[17]),
        texture_arg_name(s0[18]),
        texture_arg_name(s0[19]),
        texture_stage_value_name(21, s0[21]),
        texture_stage_value_name(28, s0[28]),
        texture_op_name(s1[12]),
        texture_op_name(s1[16]),
    ));
}

fn is_spidey_select_circle_texture(tex_raw: u32, tex_norm: u32) -> bool {
    tex_raw == SPIDEY_SELECT_CIRCLE_TEX_RAW || tex_norm == SPIDEY_SELECT_CIRCLE_TEX_NORM
}

fn spidey_select_circle_stage0_active() -> bool {
    is_spidey_select_circle_texture(
        CURRENT_STAGE0_TEXTURE_RAW.load(Ordering::Relaxed),
        CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed),
    )
}

fn log_spidey_select_circle_bind(stage: u32, tex_raw: u32, tex_norm: u32, data: u32, fmt: u32) {
    if stage != 0 || !is_spidey_select_circle_texture(tex_raw, tex_norm) {
        return;
    }
    static CIRCLE_BIND_LOG: AtomicU32 = AtomicU32::new(0);
    let seq = CIRCLE_BIND_LOG.fetch_add(1, Ordering::Relaxed);
    if seq >= 64 && !seq.is_power_of_two() {
        return;
    }

    let tss = texture_stage_snapshot();
    let s0 = &tss.states[0];
    let ps = pixel_shader_state_snapshot();
    let rs = crate::xbox::gpu::render_state::GLOBAL_RENDER_STATE
        .lock()
        .map(|state| *state)
        .unwrap_or_default();

    debug_log(&format!(
        "[SPIDEY-CIRCLE-BIND] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} blend={} src={:?} dst={:?} alpha_test={} alpha_ref={} alpha_func={:?} tss_alphakill=0x{:08X} tss_color={}({},{},{}) tss_alpha={}({},{},{}) ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_texmodes=0x{:08X}",
        seq,
        stage,
        tex_raw,
        tex_norm,
        data,
        fmt,
        crate::xbox::gpu::texture_format::format_name(fmt),
        rs.alpha_blend_enable,
        rs.src_blend,
        rs.dest_blend,
        rs.alpha_test_enable,
        rs.alpha_ref,
        rs.alpha_func,
        s0[11],
        texture_op_name(s0[12]),
        texture_arg_name(s0[13]),
        texture_arg_name(s0[14]),
        texture_arg_name(s0[15]),
        texture_op_name(s0[16]),
        texture_arg_name(s0[17]),
        texture_arg_name(s0[18]),
        texture_arg_name(s0[19]),
        ps.combiner_count,
        ps.rgb_inputs0,
        ps.alpha_inputs0,
        ps.texture_modes
    ));
}

fn log_spidey_select_circle_draw_probe(
    tag: &str,
    draw_no: u32,
    prim_type: u32,
    verts: &[crate::xbox::gpu::NV2AVertex],
    shader: u32,
    note: &str,
) {
    if !spidey_select_circle_stage0_active() {
        return;
    }
    let selector_was_seen = SPIDEY_MENU_SELECTOR_SEEN.swap(true, Ordering::Relaxed);
    static CIRCLE_DRAW_LOG: AtomicU32 = AtomicU32::new(0);
    let seq = CIRCLE_DRAW_LOG.fetch_add(1, Ordering::Relaxed);
    if seq >= 128 && !seq.is_power_of_two() {
        return;
    }

    let (min_x, min_y, max_x, max_y, min_u, max_u, min_v, max_v, min_a, max_a, rgb_n, alpha_n) =
        if verts.is_empty() {
            (
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0u8, 0u8, 0usize, 0usize,
            )
        } else {
            verts.iter().fold(
                (
                    f32::INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    u8::MAX,
                    u8::MIN,
                    0usize,
                    0usize,
                ),
                |(
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    min_u,
                    max_u,
                    min_v,
                    max_v,
                    min_a,
                    max_a,
                    rgb_n,
                    alpha_n,
                ),
                 v| {
                    let a = (v.color >> 24) as u8;
                    (
                        min_x.min(v.x),
                        min_y.min(v.y),
                        max_x.max(v.x),
                        max_y.max(v.y),
                        min_u.min(v.u),
                        max_u.max(v.u),
                        min_v.min(v.v),
                        max_v.max(v.v),
                        min_a.min(a),
                        max_a.max(a),
                        rgb_n + usize::from((v.color & 0x00FF_FFFF) != 0),
                        alpha_n + usize::from((v.color & 0xFF00_0000) != 0),
                    )
                },
            )
        };
    let tss = texture_stage_snapshot();
    let s0 = &tss.states[0];
    let ps = pixel_shader_state_snapshot();
    let rs = crate::xbox::gpu::render_state::GLOBAL_RENDER_STATE
        .lock()
        .map(|state| *state)
        .unwrap_or_default();
    let tex_raw = CURRENT_STAGE0_TEXTURE_RAW.load(Ordering::Relaxed);
    let tex_norm = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    let tex_data = CURRENT_STAGE0_TEXTURE_DATA.load(Ordering::Relaxed);
    let tex_fmt = CURRENT_STAGE0_TEXTURE_FORMAT.load(Ordering::Relaxed);
    let fvf_raw = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
    let active_shader = if shader != 0 { shader } else { fvf_raw };
    let shader_hle_path = (0x0000_1000..0x0100_0000).contains(&active_shader);
    let guest_fvf = if !shader_hle_path && active_shader != 0 && (active_shader & 1) != 0 {
        active_shader
    } else {
        0
    };
    let fvf_xyzrhw = (guest_fvf & crate::xbox::gpu::fvf_decode::fvf::POSITION_MASK)
        == crate::xbox::gpu::fvf_decode::fvf::XYZRHW;
    let stream_stride = STREAM0_STRIDE.load(Ordering::Relaxed);
    let shader_info = if shader_hle_path {
        lookup_vertex_shader_info(active_shader)
    } else {
        None
    };
    let shader_kind = if shader_hle_path {
        "program"
    } else if guest_fvf != 0 {
        "fvf"
    } else {
        "none"
    };
    let decl_stride = shader_info
        .as_ref()
        .map(|info| info.decl.stride)
        .unwrap_or(0);
    let decl_v5 = shader_info
        .as_ref()
        .and_then(|info| info.decl.v5)
        .map(|e| format!("dt=0x{:02X}:off={}:sz={}", e.data_type, e.offset, e.size))
        .unwrap_or_else(|| "none".to_string());
    let decl_v6 = shader_info
        .as_ref()
        .and_then(|info| info.decl.v6)
        .map(|e| format!("dt=0x{:02X}:off={}:sz={}", e.data_type, e.offset, e.size))
        .unwrap_or_else(|| "none".to_string());

    debug_log(&format!(
        "[SPIDEY-CIRCLE-DRAW] #{} tag={} draw={} note={} selector_was_seen={} tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} prim={} verts={} shader=0x{:08X} shader_kind={} guest_vs_raw=0x{:08X} guest_fvf=0x{:08X} fvf_xyzrhw={} stream_stride={} decl_stride={} decl_screenspace={} simple_dp4={} skinned={} decl_v5={} decl_v6={} bbox=[{:.1},{:.1}..{:.1},{:.1}] uv=[{:.3}..{:.3},{:.3}..{:.3}] alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} color_write=0x{:X} z={} zwrite={} tss_seq={} tss_alphakill=0x{:08X} tss_color={}({},{},{}) tss_alpha={}({},{},{}) tss_xform={} tss_tc={} ps_seq={} ps_combiner=0x{:08X} ps_rgb0=0x{:08X} ps_alpha0=0x{:08X} ps_out0=0x{:08X} ps_final=[0x{:08X},0x{:08X}] ps_texmodes=0x{:08X}",
        seq,
        tag,
        draw_no,
        note,
        selector_was_seen,
        tex_raw,
        tex_norm,
        tex_data,
        tex_fmt,
        crate::xbox::gpu::texture_format::format_name(tex_fmt),
        prim_type,
        verts.len(),
        active_shader,
        shader_kind,
        fvf_raw,
        guest_fvf,
        fvf_xyzrhw,
        stream_stride,
        decl_stride,
        shader_info.as_ref().is_some_and(|info| info.screenspace),
        shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
        shader_info.as_ref().is_some_and(|info| info.skinned),
        decl_v5,
        decl_v6,
        min_x,
        min_y,
        max_x,
        max_y,
        min_u,
        max_u,
        min_v,
        max_v,
        min_a,
        max_a,
        rgb_n,
        verts.len(),
        alpha_n,
        verts.len(),
        rs.alpha_blend_enable,
        rs.src_blend,
        rs.dest_blend,
        rs.blend_op,
        rs.alpha_test_enable,
        rs.alpha_ref,
        rs.alpha_func,
        rs.color_write_enable,
        rs.z_enable,
        rs.z_write_enable,
        tss.seq,
        s0[11],
        texture_op_name(s0[12]),
        texture_arg_name(s0[13]),
        texture_arg_name(s0[14]),
        texture_arg_name(s0[15]),
        texture_op_name(s0[16]),
        texture_arg_name(s0[17]),
        texture_arg_name(s0[18]),
        texture_arg_name(s0[19]),
        texture_stage_value_name(21, s0[21]),
        texture_stage_value_name(28, s0[28]),
        ps.seq,
        ps.combiner_count,
        ps.rgb_inputs0,
        ps.alpha_inputs0,
        ps.rgb_outputs0,
        ps.final_abcd,
        ps.final_efg,
        ps.texture_modes
    ));
}

fn capture_spidey_select_circle_after_draw(
    backend: &mut dyn crate::xbox::gpu::GpuBackend,
    tag: &str,
    draw_no: u32,
    prim_type: u32,
    verts: &[crate::xbox::gpu::NV2AVertex],
) {
    if !spidey_select_circle_stage0_active() {
        return;
    }
    static CIRCLE_AFTER_DRAW_CAPTURED: AtomicBool = AtomicBool::new(false);
    if CIRCLE_AFTER_DRAW_CAPTURED.swap(true, Ordering::Relaxed) {
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

    let pixels = backend.readback_framebuffer().to_vec();
    SPIDEY_MENU_SELECTOR_SEEN.store(true, Ordering::Relaxed);
    {
        let mut cache = SPIDEY_MENU_SELECTOR_PRESENT_CACHE
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        *cache = Some(pixels.clone());
    }
    let path = r"./spidey_select_circle_after_draw.bmp";
    write_readback_bmp(path, 640, 480, &pixels);
    let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) = sampled_argb_stats(&pixels);
    let px = |x: usize, y: usize| -> u32 {
        pixels
            .get(y.saturating_mul(640).saturating_add(x))
            .copied()
            .unwrap_or(0)
    };
    debug_log(&format!(
        "[SPIDEY-CIRCLE-READBACK] tag={} draw={} prim={} verts={} bbox=[{:.1},{:.1}..{:.1},{:.1}] tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} px_left_tile_center=0x{:08X} px_screen_center=0x{:08X} px_right_tile_center=0x{:08X} px_right_tile_edge=0x{:08X} path={}",
        tag,
        draw_no,
        prim_type,
        verts.len(),
        min_x,
        min_y,
        max_x,
        max_y,
        CURRENT_STAGE0_TEXTURE_RAW.load(Ordering::Relaxed),
        CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed),
        CURRENT_STAGE0_TEXTURE_DATA.load(Ordering::Relaxed),
        CURRENT_STAGE0_TEXTURE_FORMAT.load(Ordering::Relaxed),
        crate::xbox::gpu::texture_format::format_name(
            CURRENT_STAGE0_TEXTURE_FORMAT.load(Ordering::Relaxed)
        ),
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

fn capture_spidey_focus_texture_after_draw(
    backend: &mut dyn crate::xbox::gpu::GpuBackend,
    tag: &str,
    draw_no: u32,
    prim_type: u32,
    shader: u32,
    verts: &[crate::xbox::gpu::NV2AVertex],
) {
    let tex_raw = CURRENT_STAGE0_TEXTURE_RAW.load(Ordering::Relaxed);
    let tex_norm = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    if !spidey_focus_texture_match(0, tex_raw, tex_norm) {
        return;
    }

    static FOCUS_AFTER_DRAW_CAPTURED: AtomicU32 = AtomicU32::new(0);
    let seq = FOCUS_AFTER_DRAW_CAPTURED.fetch_add(1, Ordering::Relaxed);
    if seq >= 16 {
        return;
    }

    let (
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        min_u,
        max_u,
        min_v,
        max_v,
        min_a,
        max_a,
        rgb_n,
        alpha_n,
    ) = if verts.is_empty() {
        (
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0u8, 0u8, 0usize, 0usize,
        )
    } else {
        verts.iter().fold(
            (
                f32::INFINITY,
                f32::INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
                u8::MAX,
                u8::MIN,
                0usize,
                0usize,
            ),
            |(
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
                min_u,
                max_u,
                min_v,
                max_v,
                min_a,
                max_a,
                rgb_n,
                alpha_n,
            ),
             v| {
                let a = (v.color >> 24) as u8;
                (
                    min_x.min(v.x),
                    min_y.min(v.y),
                    min_z.min(v.z),
                    max_x.max(v.x),
                    max_y.max(v.y),
                    max_z.max(v.z),
                    min_u.min(v.u),
                    max_u.max(v.u),
                    min_v.min(v.v),
                    max_v.max(v.v),
                    min_a.min(a),
                    max_a.max(a),
                    rgb_n + usize::from((v.color & 0x00FF_FFFF) != 0),
                    alpha_n + usize::from((v.color & 0xFF00_0000) != 0),
                )
            },
        )
    };

    let pixels = backend.readback_framebuffer().to_vec();
    let path = format!(
        r"./gate3_focus_tex_after_draw_{:02}_drawidx_{:05}_tex_{:08X}.bmp",
        seq, draw_no, tex_norm
    );
    write_readback_bmp(&path, 640, 480, &pixels);
    let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) = sampled_argb_stats(&pixels);
    let px = |x: usize, y: usize| -> u32 {
        pixels
            .get(y.saturating_mul(640).saturating_add(x))
            .copied()
            .unwrap_or(0)
    };
    let rs = crate::xbox::gpu::render_state::GLOBAL_RENDER_STATE
        .lock()
        .map(|state| *state)
        .unwrap_or_default();
    let tss = texture_stage_snapshot();
    let s0 = &tss.states[0];
    let ps = pixel_shader_state_snapshot();
    debug_log(&format!(
        "[HLE-DRAWIDX-FOCUS-TEX-CAPTURE] seq={} tag={} draw#{} shader=0x{:08X} tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} prim={} verts={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] uv=[{:.3}..{:.3},{:.3}..{:.3}] vtx_alpha=[{}..{}] rgb_nonzero={}/{} alpha_nonzero={}/{} fb_nonzero={}/1024 fb_alpha={}/1024 fb_rgb={}/1024 first=0x{:08X} mid=0x{:08X} px_left=0x{:08X} px_center=0x{:08X} px_right=0x{:08X} lighting={} ambient=0x{:08X} tfactor=0x{:08X} light_mask=0x{:08X} light_seq={} material_seq={} fog={} fog_color=0x{:08X} blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} z={} zwrite={} zfunc={:?} color_write=0x{:X} tss_color={}({},{},{}) tss_alpha={}({},{},{}) ps_combiner={} rgb0=0x{:08X} alpha0=0x{:08X} path={}",
        seq,
        tag,
        draw_no,
        shader,
        tex_raw,
        tex_norm,
        CURRENT_STAGE0_TEXTURE_DATA.load(Ordering::Relaxed),
        CURRENT_STAGE0_TEXTURE_FORMAT.load(Ordering::Relaxed),
        crate::xbox::gpu::texture_format::format_name(
            CURRENT_STAGE0_TEXTURE_FORMAT.load(Ordering::Relaxed)
        ),
        prim_type,
        verts.len(),
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        min_u,
        max_u,
        min_v,
        max_v,
        min_a,
        max_a,
        rgb_n,
        verts.len(),
        alpha_n,
        verts.len(),
        sample_nonzero,
        sample_alpha,
        sample_rgb,
        first_px,
        mid_px,
        px(160, 240),
        px(320, 240),
        px(480, 240),
        rs.lighting,
        rs.ambient,
        rs.texture_factor,
        LIGHT_ENABLE_MASK.load(Ordering::Relaxed),
        LIGHT_STATE_SEQ.load(Ordering::Relaxed),
        MATERIAL_STATE_SEQ.load(Ordering::Relaxed),
        rs.fog_enable,
        rs.fog_color,
        rs.alpha_blend_enable,
        rs.src_blend,
        rs.dest_blend,
        rs.blend_op,
        rs.alpha_test_enable,
        rs.alpha_ref,
        rs.alpha_func,
        rs.z_enable,
        rs.z_write_enable,
        rs.z_func,
        rs.color_write_enable,
        texture_op_name(s0[12]),
        texture_arg_name(s0[13]),
        texture_arg_name(s0[14]),
        texture_arg_name(s0[15]),
        texture_op_name(s0[16]),
        texture_arg_name(s0[17]),
        texture_arg_name(s0[18]),
        texture_arg_name(s0[19]),
        ps.combiner_count,
        ps.rgb_inputs0,
        ps.alpha_inputs0,
        path
    ));
}

fn load_pixel_shader_def(source: &str, def_addr: u32, guest_mem: *mut u8) -> bool {
    let Some(def) = normalize_guest_ram_ptr(def_addr) else {
        return false;
    };
    if def > 0x2000_0000u32.saturating_sub(0xF0) {
        return false;
    }

    let mut words = [0u32; 60];
    unsafe {
        let base = guest_mem.add(def as usize);
        for (i, word) in words.iter_mut().enumerate() {
            *word = std::ptr::read_unaligned(base.add(i * 4) as *const u32);
        }
    }

    {
        let mut states = ps_render_states().lock().unwrap_or_else(|e| e.into_inner());
        for i in 0..=59 {
            states[i] = words[i];
        }
        states[136] = words[54];
    }
    PS_MAPPING_VALID.store(true, Ordering::Relaxed);
    let seq = PS_RENDER_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;

    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        for (i, value) in words.iter().copied().enumerate().take(60) {
            gpu.set_render_state(i as u32, value);
        }
        gpu.set_render_state(136, words[54]);
    }

    static PS_SET_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PS_SET_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 24 || n.is_power_of_two() {
        let tex_modes = words[54];
        debug_log(&format!(
            "[PS-SET] #{} src={} def=0x{:08X} combiner=0x{:08X} texmodes=0x{:08X} modes=[{},{},{},{}] rgb0=0x{:08X} alpha0=0x{:08X} out0=0x{:08X} final=[0x{:08X},0x{:08X}] fc=[0x{:08X},0x{:08X}] dot=0x{:08X} input=0x{:08X} map=[c0=0x{:08X},c1=0x{:08X},fc=0x{:08X}]",
            seq,
            source,
            def,
            words[53],
            tex_modes,
            tex_modes & 0x1F,
            (tex_modes >> 5) & 0x1F,
            (tex_modes >> 10) & 0x1F,
            (tex_modes >> 15) & 0x1F,
            words[34],
            words[0],
            words[45],
            words[8],
            words[9],
            words[43],
            words[44],
            words[55],
            words[56],
            words[57],
            words[58],
            words[59]
        ));
    }
    true
}

pub(super) fn hle_set_pixel_shader(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let handle = args[0];
    drain_draws_before_state_change("SetPixelShader");
    CURRENT_PIXEL_SHADER_HANDLE.store(handle, Ordering::Relaxed);
    if handle == 0 {
        PS_MAPPING_VALID.store(false, Ordering::Relaxed);
        static PS_CLEAR_LOG: AtomicU32 = AtomicU32::new(0);
        let n = PS_CLEAR_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 8 {
            debug_log("[PS-SET] SetPixelShader(NULL)");
        }
        return 0;
    }

    let mut candidates = [0u32; 3];
    candidates[0] = handle;
    candidates[1] = handle;
    candidates[2] = handle.wrapping_add(0x0C);
    if let Some(h) = normalize_guest_ram_ptr(handle) {
        if h <= 0x2000_0000u32.saturating_sub(0x0C) {
            let ptr =
                unsafe { std::ptr::read_unaligned(guest_mem.add(h as usize + 0x08) as *const u32) };
            candidates[0] = ptr;
        }
    }

    for (idx, candidate) in candidates.iter().copied().enumerate() {
        if candidate != 0 && load_pixel_shader_def("SetPixelShader", candidate, guest_mem) {
            static PS_BIND_LOG: AtomicU32 = AtomicU32::new(0);
            let n = PS_BIND_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                debug_log(&format!(
                    "[PS-BIND] #{} handle=0x{:08X} candidate{}=0x{:08X}",
                    n, handle, idx, candidate
                ));
            }
            return 0;
        }
    }

    static PS_FAIL_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PS_FAIL_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        debug_log(&format!(
            "[PS-BIND] failed handle=0x{:08X} candidates=[0x{:08X},0x{:08X},0x{:08X}]",
            handle, candidates[0], candidates[1], candidates[2]
        ));
    }
    0
}

fn pending_shader_sources() -> &'static Mutex<Vec<String>> {
    PENDING_SHADER_SOURCES.get_or_init(|| Mutex::new(Vec::new()))
}

fn vs_shader_infos() -> &'static Mutex<Vec<VertexShaderInfo>> {
    VS_SHADER_INFOS.get_or_init(|| Mutex::new(Vec::new()))
}

fn record_pending_shader_source(source: String) {
    if source.is_empty() {
        return;
    }
    let mut pending = pending_shader_sources()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    pending.push(source);
    if pending.len() > 32 {
        pending.remove(0);
    }
}

fn take_pending_shader_source() -> Option<String> {
    let mut pending = pending_shader_sources()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if pending.is_empty() {
        None
    } else {
        Some(pending.remove(0))
    }
}

fn register_vertex_shader_info(
    handle: u32,
    source: Option<String>,
    decl: VertexDeclInfo,
) -> Option<VertexShaderInfo> {
    let source = source.unwrap_or_default();
    let lower = source.to_ascii_lowercase();
    let info = VertexShaderInfo {
        handle,
        screenspace: lower.contains("#pragma screenspace"),
        simple_dp4_o_pos: lower.contains("dp4 opos.x")
            && lower.contains("dp4 opos.y")
            && lower.contains("dp4 opos.z")
            && lower.contains("dp4 opos.w")
            && !lower.contains("a0.x"),
        passes_diffuse: lower.contains("od0") && lower.contains("v1"),
        diffuse_mul_c0: lower.contains("mul od0") && lower.contains("v1") && lower.contains("c[0]"),
        passes_tex0: lower.contains("ot0") && (lower.contains("v3") || lower.contains("v7")),
        tex0_add_c0: lower.contains("add ot0") && lower.contains("v3") && lower.contains("c[0]"),
        skinned: lower.contains("a0.x") || lower.contains("vsf_skin"),
        decl,
        snippet: source.chars().take(160).collect(),
    };
    let mut infos = vs_shader_infos().lock().unwrap_or_else(|e| e.into_inner());
    infos.retain(|old| old.handle != handle);
    infos.push(info.clone());
    if infos.len() > 64 {
        infos.remove(0);
    }
    Some(info)
}

fn lookup_vertex_shader_info(handle: u32) -> Option<VertexShaderInfo> {
    vs_shader_infos()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .rev()
        .find(|info| info.handle == handle)
        .cloned()
}

fn signed_norm(raw: u32, bits: u32) -> f32 {
    let sign_bit = 1u32 << (bits - 1);
    let mask = (1u32 << bits) - 1;
    let value = raw & mask;
    let signed = if (value & sign_bit) != 0 {
        (value as i32) - (1i32 << bits)
    } else {
        value as i32
    };
    (signed as f32 / (sign_bit - 1) as f32).clamp(-1.0, 1.0)
}

fn read_decl_vec4(guest_mem: *mut u8, base: u64, stride: u32, elem: VertexDeclElement) -> [f32; 4] {
    if elem.size == 0 || elem.offset >= stride {
        return [0.0; 4];
    }
    let ptr = base.saturating_add(elem.offset as u64);
    let end = ptr.saturating_add(elem.size as u64);
    let guest_base = guest_mem as u64;
    if ptr < guest_base || end > guest_base.saturating_add(0x2000_0000) {
        return [0.0; 4];
    }
    unsafe {
        match elem.data_type {
            0x42 => [
                std::ptr::read_unaligned(ptr as *const f32),
                std::ptr::read_unaligned((ptr + 4) as *const f32),
                std::ptr::read_unaligned((ptr + 8) as *const f32),
                std::ptr::read_unaligned((ptr + 12) as *const f32),
            ],
            0x32 => [
                std::ptr::read_unaligned(ptr as *const f32),
                std::ptr::read_unaligned((ptr + 4) as *const f32),
                std::ptr::read_unaligned((ptr + 8) as *const f32),
                1.0,
            ],
            0x22 => [
                std::ptr::read_unaligned(ptr as *const f32),
                std::ptr::read_unaligned((ptr + 4) as *const f32),
                0.0,
                1.0,
            ],
            0x12 => [std::ptr::read_unaligned(ptr as *const f32), 0.0, 0.0, 1.0],
            0x44 => [
                std::ptr::read_unaligned(ptr as *const u8) as f32 / 255.0,
                std::ptr::read_unaligned((ptr + 1) as *const u8) as f32 / 255.0,
                std::ptr::read_unaligned((ptr + 2) as *const u8) as f32 / 255.0,
                std::ptr::read_unaligned((ptr + 3) as *const u8) as f32 / 255.0,
            ],
            0x45 => [
                std::ptr::read_unaligned(ptr as *const i16) as f32,
                std::ptr::read_unaligned((ptr + 2) as *const i16) as f32,
                std::ptr::read_unaligned((ptr + 4) as *const i16) as f32,
                std::ptr::read_unaligned((ptr + 6) as *const i16) as f32,
            ],
            0x24 => [
                std::ptr::read_unaligned(ptr as *const u8) as f32 / 255.0,
                std::ptr::read_unaligned((ptr + 1) as *const u8) as f32 / 255.0,
                0.0,
                1.0,
            ],
            0x25 => [
                std::ptr::read_unaligned(ptr as *const i16) as f32,
                std::ptr::read_unaligned((ptr + 2) as *const i16) as f32,
                0.0,
                1.0,
            ],
            0x05 | 0x61 => [
                std::ptr::read_unaligned(ptr as *const u8) as f32,
                std::ptr::read_unaligned((ptr + 1) as *const u8) as f32,
                std::ptr::read_unaligned((ptr + 2) as *const u8) as f32,
                std::ptr::read_unaligned((ptr + 3) as *const u8) as f32,
            ],
            0x16 => {
                let packed = std::ptr::read_unaligned(ptr as *const u32);
                [
                    signed_norm(packed, 11),
                    signed_norm(packed >> 11, 11),
                    signed_norm(packed >> 22, 10),
                    1.0,
                ]
            }
            0x40 => {
                let color = std::ptr::read_unaligned(ptr as *const u32);
                [
                    ((color >> 16) & 0xFF) as f32 / 255.0,
                    ((color >> 8) & 0xFF) as f32 / 255.0,
                    (color & 0xFF) as f32 / 255.0,
                    ((color >> 24) & 0xFF) as f32 / 255.0,
                ]
            }
            _ => [0.0; 4],
        }
    }
}

fn read_decl_vec4_pbyte4_raw(
    guest_mem: *mut u8,
    base: u64,
    stride: u32,
    elem: VertexDeclElement,
) -> [f32; 4] {
    if elem.data_type != 0x44 || elem.size < 4 || elem.offset >= stride {
        return read_decl_vec4(guest_mem, base, stride, elem);
    }
    let ptr = base.saturating_add(elem.offset as u64);
    let guest_base = guest_mem as u64;
    if ptr < guest_base || ptr.saturating_add(4) > guest_base.saturating_add(0x2000_0000) {
        return [0.0; 4];
    }
    let scale = if spidey_pbyte4_normalize_enabled() {
        1.0 / 255.0
    } else {
        1.0
    };
    unsafe {
        [
            std::ptr::read_unaligned(ptr as *const u8) as f32 * scale,
            std::ptr::read_unaligned((ptr + 1) as *const u8) as f32 * scale,
            std::ptr::read_unaligned((ptr + 2) as *const u8) as f32 * scale,
            std::ptr::read_unaligned((ptr + 3) as *const u8) as f32 * scale,
        ]
    }
}

fn vertex_decl_elem_summary(elem: Option<VertexDeclElement>) -> String {
    elem.map(|e| format!("dt=0x{:02X}:off={}:sz={}", e.data_type, e.offset, e.size))
        .unwrap_or_else(|| "missing".to_string())
}

fn skin_a0_probe(blend_indices: [f32; 4]) -> String {
    let normalize = spidey_pbyte4_normalize_enabled();
    let constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    for (lane, value) in blend_indices.into_iter().enumerate() {
        let a0 = if normalize {
            (value * 1020.0) - 92.0
        } else {
            (value * 4.0) - 92.0
        };
        let raw = (a0.round() as i32).saturating_add(92);
        let bone = if raw >= 0 && raw % 4 == 0 {
            Some(raw / 4)
        } else {
            None
        };
        let row_base = bone
            .filter(|b| (0..48).contains(b))
            .map(|b| 17 + (b as usize * 3));
        let row0 = row_base
            .and_then(|idx| constants.get(idx).copied())
            .unwrap_or([0.0; 4]);
        out.push(format!(
            "lane{}:v={:.5}:a0={:.1}:bone={}:row_base={}:row0=[{:.3},{:.3},{:.3},{:.3}]",
            lane,
            value,
            a0,
            bone.map(|b| b.to_string())
                .unwrap_or_else(|| "-".to_string()),
            row_base
                .map(|idx| format!("c{}", idx))
                .unwrap_or_else(|| "-".to_string()),
            row0[0],
            row0[1],
            row0[2],
            row0[3],
        ));
    }
    out.join(" ")
}

#[allow(clippy::too_many_arguments)]
fn log_spidey_skin_draw_probe(
    guest_mem: *mut u8,
    draw_no: u32,
    shader: u32,
    prim_type: u32,
    index_count: u32,
    index_data_ptr: u32,
    vb_data: u32,
    stride: u32,
    shader_translated: bool,
    cpu_vs_xform: bool,
    shader_info: Option<&VertexShaderInfo>,
    shader_decl: VertexDeclInfo,
    first_indices: &[u32],
    min_index: u32,
    max_index: u32,
    skin_payload: &[VertexSkinPayload],
    finite_count: usize,
    bbox: (f32, f32, f32, f32, f32, f32),
) {
    if !spidey_skin_draw_probe_enabled() {
        return;
    }
    if stride != 0x2C && !shader_info.is_some_and(|info| info.skinned) {
        return;
    }

    static SKIN_DRAW_PROBE_N: AtomicU32 = AtomicU32::new(0);
    let probe_no = SKIN_DRAW_PROBE_N.fetch_add(1, Ordering::Relaxed);
    if probe_no >= 32 && !probe_no.is_power_of_two() {
        return;
    }

    let first = skin_payload.first().copied().unwrap_or_default();
    let sample = skin_payload
        .iter()
        .copied()
        .find(|p| {
            p.blend_indices.iter().any(|v| v.abs() > 0.0001)
                || p.blend_weights.iter().any(|v| v.abs() > 0.0001)
        })
        .unwrap_or(first);
    let hlsl = crate::xbox::gpu::nv2a_vsh::hlsl_for_handle(shader).unwrap_or_default();
    let hlsl_rel = hlsl.contains("nv2a_relative_const_index");
    let hlsl_a0 = hlsl.contains("A0");
    let hlsl_len = hlsl.len();
    let hlsl_path = if !hlsl.is_empty() && probe_no < 4 {
        let path = format!(
            r"./spidey_skin_shader_{:08X}_{:02}.hlsl",
            shader, probe_no
        );
        match std::fs::write(&path, hlsl.as_bytes()) {
            Ok(()) => path,
            Err(_) => "-".to_string(),
        }
    } else {
        "-".to_string()
    };
    let constants_path = if probe_no < 4 {
        let constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
        let mut text = String::new();
        for i in 0..96 {
            let c = constants[i];
            text.push_str(&format!(
                "c{:<2} [{:.8}, {:.8}, {:.8}, {:.8}]\n",
                i, c[0], c[1], c[2], c[3]
            ));
        }
        let path = format!(
            r"./spidey_skin_constants_{:08X}_{:02}.txt",
            shader, probe_no
        );
        match std::fs::write(&path, text.as_bytes()) {
            Ok(()) => path,
            Err(_) => "-".to_string(),
        }
    } else {
        "-".to_string()
    };
    let raw_v5 = shader_decl
        .v5
        .and_then(|elem| {
            let idx = first_indices.first().copied()?;
            let guest_addr = vb_data
                .wrapping_add(idx.saturating_mul(stride))
                .wrapping_add(elem.offset);
            let addr = normalize_guest_ram_ptr(guest_addr)?;
            if addr.checked_add(4).map_or(true, |end| end > 0x2000_0000) {
                return None;
            }
            let bytes = unsafe { std::slice::from_raw_parts(guest_mem.add(addr as usize), 4) };
            Some(format!(
                "idx{}@0x{:08X}/v5+{}=[{:02X} {:02X} {:02X} {:02X}]",
                idx, guest_addr, elem.offset, bytes[0], bytes[1], bytes[2], bytes[3]
            ))
        })
        .unwrap_or_else(|| "missing".to_string());

    let (min_x, min_y, min_z, max_x, max_y, max_z) = bbox;
    debug_log(&format!(
        "[SPIDEY-SKIN-DRAW-PROBE] #{} draw#{} shader=0x{:08X} prim={} indices={} idx=0x{:08X} vb=0x{:08X} stride={} translated={} cpu_vs={} skinned={} screenspace={} decl_stride={} decl_v2={} decl_v4={} decl_v5={} decl_v6={} d3d11_blendindices=R32G32B32A32_FLOAT/hle-expanded raw_v5={} idx_range={}..{} first_idx={:?} verts={} finite={} bbox=[{:.3},{:.3},{:.3}..{:.3},{:.3},{:.3}] first_v5=[{:.5},{:.5},{:.5},{:.5}] sample_v5=[{:.5},{:.5},{:.5},{:.5}] sample_v6=[{:.5},{:.5},{:.5},{:.5}] sample_a0=\"{}\" hlsl_len={} hlsl_a0={} hlsl_rel={} hlsl_path={} constants_c0_c95={}",
        probe_no,
        draw_no,
        shader,
        prim_type,
        index_count,
        index_data_ptr,
        vb_data,
        stride,
        shader_translated,
        cpu_vs_xform,
        shader_info.is_some_and(|info| info.skinned),
        shader_info.is_some_and(|info| info.screenspace),
        shader_decl.stride,
        vertex_decl_elem_summary(shader_decl.v2),
        vertex_decl_elem_summary(shader_decl.v4),
        vertex_decl_elem_summary(shader_decl.v5),
        vertex_decl_elem_summary(shader_decl.v6),
        raw_v5,
        min_index,
        max_index,
        first_indices,
        skin_payload.len(),
        finite_count,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        first.blend_indices[0],
        first.blend_indices[1],
        first.blend_indices[2],
        first.blend_indices[3],
        sample.blend_indices[0],
        sample.blend_indices[1],
        sample.blend_indices[2],
        sample.blend_indices[3],
        sample.blend_weights[0],
        sample.blend_weights[1],
        sample.blend_weights[2],
        sample.blend_weights[3],
        skin_a0_probe(sample.blend_indices),
        hlsl_len,
        hlsl_a0,
        hlsl_rel,
        hlsl_path,
        constants_path,
    ));
}

fn shader_diffuse_color(raw: u32) -> u32 {
    // Xbox D3DCOLOR is ARGB. Some XDK paths use XRGB-like vertex colors where
    // alpha is left at zero; make those visible without inventing RGB.
    let rgb = raw & 0x00FF_FFFF;
    if rgb == 0 {
        // This path is still a geometry proof. A black diffuse value with no
        // populated texture yet erases otherwise-valid Spider-Man meshes.
        0xFFFF_FFFF
    } else if (raw & 0xFF00_0000) == 0 {
        raw | 0xFF00_0000
    } else {
        raw
    }
}

fn current_vs_constant(reg: usize) -> [f32; 4] {
    if reg >= VS_CONSTANT_COUNT {
        return [0.0; 4];
    }
    vs_constants().lock().unwrap_or_else(|e| e.into_inner())[reg]
}

fn shader_apply_c0_diffuse(raw: u32, c0: [f32; 4]) -> u32 {
    let color = shader_diffuse_color(raw);
    let scale = |component: u32, factor: f32| -> u32 {
        let factor = if factor.is_finite() { factor } else { 1.0 };
        ((component as f32 * factor.clamp(0.0, 1.0)).round() as u32).min(255)
    };
    let a = scale((color >> 24) & 0xFF, c0[3]);
    let r = scale((color >> 16) & 0xFF, c0[0]);
    let g = scale((color >> 8) & 0xFF, c0[1]);
    let b = scale(color & 0xFF, c0[2]);
    (a << 24) | (r << 16) | (g << 8) | b
}

fn cache_shader_viewport(x: u32, y: u32, w: u32, h: u32, min_z: f32, max_z: f32) {
    if w == 0 || h == 0 {
        return;
    }
    let min_z = if min_z.is_finite() { min_z } else { 0.0 };
    let max_z = if max_z.is_finite() { max_z } else { 1.0 };
    let vp = ShaderViewport {
        x: x as f32,
        y: y as f32,
        w: w as f32,
        h: h as f32,
        min_z,
        max_z,
    };
    *shader_viewport().lock().unwrap_or_else(|e| e.into_inner()) = vp;
    upload_reserved_screenspace_constants(vp);
}

fn upload_reserved_screenspace_constants(vp: ShaderViewport) {
    const RESERVED_SCALE: usize = 58; // XDK c[-38] after +96 correction.
    const RESERVED_OFFSET: usize = 59; // XDK c[-37] after +96 correction.

    let mode = shader_constant_mode();
    if (mode & X_D3DSCM_NORESERVEDCONSTANTS) != 0 {
        static SCREENSPACE_SKIP_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SCREENSPACE_SKIP_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 12 || n.is_power_of_two() {
            debug_log(&format!(
                "[VS-CONST-SCREENSPACE-SKIP] #{} mode=0x{:08X} noreserved=1 viewport={}x{}+{}+{}",
                n, mode, vp.w as u32, vp.h as u32, vp.x as u32, vp.y as u32
            ));
        }
        return;
    }

    let z_scale = (vp.max_z - vp.min_z).abs().max(1.0);
    let scale = [vp.w * 0.5, vp.h * -0.5, z_scale, 1.0];
    let offset = [vp.x + vp.w * 0.5, vp.y + vp.h * 0.5, vp.min_z, 0.0];

    if spidey_allow_reserved_bone_rows_enabled() {
        static SCREENSPACE_EXTONLY_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SCREENSPACE_EXTONLY_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[VS-CONST-SCREENSPACE-EXTONLY] #{} mode=0x{:08X} preserve_guest_c58_c59=1 scale=[{:.4},{:.4},{:.4},{:.4}] offset=[{:.4},{:.4},{:.4},{:.4}] viewport={}x{}+{}+{} z=[{:.4},{:.4}]",
                n,
                mode,
                scale[0],
                scale[1],
                scale[2],
                scale[3],
                offset[0],
                offset[1],
                offset[2],
                offset[3],
                vp.w as u32,
                vp.h as u32,
                vp.x as u32,
                vp.y as u32,
                vp.min_z,
                vp.max_z
            ));
        }
        return;
    }

    {
        let mut constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
        constants[RESERVED_SCALE] = scale;
        constants[RESERVED_OFFSET] = offset;
    }

    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        let mut data = Vec::with_capacity(8);
        data.extend_from_slice(&scale);
        data.extend_from_slice(&offset);
        gpu.set_vs_constant(RESERVED_SCALE as u32, &data);
    }

    static SCREENSPACE_CONST_LOG: AtomicU32 = AtomicU32::new(0);
    let n = SCREENSPACE_CONST_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 12 || n.is_power_of_two() {
        debug_log(&format!(
            "[VS-CONST-SCREENSPACE] #{} mode=0x{:08X} c58=[{:.4},{:.4},{:.4},{:.4}] c59=[{:.4},{:.4},{:.4},{:.4}] viewport={}x{}+{}+{} z=[{:.4},{:.4}]",
            n,
            mode,
            scale[0],
            scale[1],
            scale[2],
            scale[3],
            offset[0],
            offset[1],
            offset[2],
            offset[3],
            vp.w as u32,
            vp.h as u32,
            vp.x as u32,
            vp.y as u32,
            vp.min_z,
            vp.max_z
        ));
    }
}

fn record_wvp_candidate(src: u32, rows: [[f32; 4]; 4]) {
    if rows.iter().all(|r| r.iter().all(|v| *v == 0.0)) {
        return;
    }

    {
        let mut wvp = vs_wvp_candidate().lock().unwrap_or_else(|e| e.into_inner());
        *wvp = rows;
    }

    let candidate = MatrixCandidate {
        seq: VS_MATRIX_SEQ.fetch_add(1, Ordering::Relaxed),
        src,
        rows,
    };
    let mut candidates = vs_wvp_candidates()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    candidates.push(candidate);
    if candidates.len() > 16 {
        let excess = candidates.len() - 16;
        candidates.drain(0..excess);
    }
}

fn normalize_vs_constant_register(raw: u32) -> Option<usize> {
    let signed = raw as i32;
    // Xbox D3D8 exposes the 192 hardware constants through a signed API window:
    // -96 maps to hardware c0, -95 to c1, ... 0 to c96, ... 95 to c191.
    // Keep this as a core invariant so positive c0..c95 uploads cannot trample
    // the negative register window used by XDK/XGRPH shader globals.
    let reg = signed + 96;
    if (0..VS_CONSTANT_COUNT as i32).contains(&reg) {
        Some(reg as usize)
    } else {
        None
    }
}

fn vs_upload_intersects_skin_window(reg: usize, count: usize, raw_reg: u32) -> bool {
    let signed = raw_reg as i32;
    signed < 0 || (reg..reg.saturating_add(count)).any(|r| (48..=72).contains(&r))
}

fn spiderman_init_shader_constant_rows() -> [[f32; 4]; 4] {
    [
        [0.0, 0.5, 1.0, 2.0],
        [f32::from_bits(0x443F_4005), -79.0, 0.75, 0.0],
        [320.0, -240.0, f32::from_bits(0x4B7F_FFFF), 1.0],
        [320.0, 240.0, 0.0, 0.0],
    ]
}

fn spiderman_fun29c5a0_svsc_canary_enabled() -> bool {
    use std::sync::OnceLock;
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_FUN29C5A0_SVSC_CANARY"))
}

fn spidey_allow_reserved_bone_rows_enabled() -> bool {
    use std::sync::OnceLock;
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_ALLOW_RESERVED_BONE_ROWS"))
}

fn spidey_reserved_bone_row_upload(raw_reg: u32, reg: usize, count: usize) -> bool {
    let signed = raw_reg as i32;
    let end = reg.saturating_add(count);
    count == 3 && (-79..=26).contains(&signed) && reg < 60 && end > 58
}

fn apply_spiderman_fun29c5a0_svsc_canary(reason: &str) {
    let init_rows = spiderman_init_shader_constant_rows();
    {
        let mut constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
        constants[0..4].copy_from_slice(&init_rows);
    }
    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        let mut uploaded = Vec::with_capacity(16);
        for row in init_rows {
            uploaded.extend_from_slice(&row);
        }
        gpu.set_vs_constant(0, &uploaded);
    }

    static CANARY_LOG: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-FUN29C5A0-SVSC-CANARY] #{} reason={} c0=[{:.6},{:.6},{:.6},{:.6}] c1=[{:.6},{:.6},{:.6},{:.6}] c2=[{:.6},{:.6},{:.6},{:.6}] c3=[{:.6},{:.6},{:.6},{:.6}]",
            n,
            reason,
            init_rows[0][0],
            init_rows[0][1],
            init_rows[0][2],
            init_rows[0][3],
            init_rows[1][0],
            init_rows[1][1],
            init_rows[1][2],
            init_rows[1][3],
            init_rows[2][0],
            init_rows[2][1],
            init_rows[2][2],
            init_rows[2][3],
            init_rows[3][0],
            init_rows[3][1],
            init_rows[3][2],
            init_rows[3][3],
        ));
    }
}

fn spiderman_seed_shader_constant_registers(guest_mem: *mut u8) {
    if guest_mem.is_null() {
        return;
    }

    unsafe {
        let base = guest_mem as u64;
        let transform_base = (base + 0x004D_2AA0) as *mut u32;
        let c80 = (base + 0x006F_2C78) as *mut u32;
        let transform_base_value = std::ptr::read_unaligned(transform_base);
        let c80_value = std::ptr::read_unaligned(c80);
        let c95_value = std::ptr::read_unaligned((base + 0x006F_2C98) as *const u32);
        let bone_base_value = std::ptr::read_unaligned((base + 0x004D_2ABC) as *const u32);
        static SEED_PRE_LOG: AtomicU32 = AtomicU32::new(0);
        let pre_n = SEED_PRE_LOG.fetch_add(1, Ordering::Relaxed);
        if pre_n < 4 {
            debug_log(&format!(
                "[SPIDEY-VS-CONST-REG-SEED-PRE] #{} c_proj=0x{:08X} c80=0x{:08X} c_minus95_global=0x{:08X} bone_base=0x{:08X}",
                pre_n, transform_base_value, c80_value, c95_value, bone_base_value
            ));
        }
        if transform_base_value != 0 || c80_value != 0 {
            return;
        }

        const FIXED: &[(u32, u32)] = &[
            (0x005E_2BB4, 0xFFFF_FFA4),
            (0x004D_2AA0, 0xFFFF_FFA4),
            (0x005F_2C68, 0xFFFF_FFA0),
            (0x006F_2C98, 0xFFFF_FFA1),
            (0x004C_E0F0, 0xFFFF_FFA2),
            (0x004D_2AA8, 0xFFFF_FFA3),
            (0x004C_E15C, 0xFFFF_FFA8),
            (0x004C_E404, 0xFFFF_FFA9),
            (0x004D_1418, 0xFFFF_FFAA),
            (0x005F_2C60, 0xFFFF_FFAB),
            (0x004D_1A00, 0xFFFF_FFAF),
            (0x006F_2C78, 0xFFFF_FFB0),
            (0x004D_2ABC, 0xFFFF_FFB1),
        ];

        for &(addr, value) in FIXED {
            std::ptr::write_unaligned((base + addr as u64) as *mut u32, value);
        }

        let computed = [
            (0x006F_2C9C, -92i32),
            (0x006F_2C8C, -88),
            (0x004C_E158, -87),
            (0x005F_2BF8, -86),
            (0x005F_2BFC, -84),
            (0x005F_2C00, -83),
            (0x005F_2C04, -82),
            (0x005F_2C08, -81),
            (0x005F_2C0C, -79),
            (0x005F_2C10, -78),
            (0x005F_2C14, -77),
            (0x005F_2C18, -76),
            (0x005F_2C1C, -74),
            (0x005F_2C20, -73),
            (0x005F_2C24, -72),
            (0x005F_2C28, -71),
            (0x005F_2C2C, -69),
            (0x005F_2C30, -68),
            (0x005F_2C34, -67),
        ];
        for &(addr, value) in &computed {
            std::ptr::write_unaligned((base + addr as u64) as *mut u32, value as u32);
        }
    }

    if spiderman_fun29c5a0_svsc_canary_enabled() {
        apply_spiderman_fun29c5a0_svsc_canary("seed_globals");
    }

    static SEED_LOG: AtomicU32 = AtomicU32::new(0);
    let n = SEED_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 4 {
        debug_log(
            "[SPIDEY-VS-CONST-REG-SEED] restored signed D3D shader constant register globals",
        );
    }
}

fn mul_matrix_point(rows: &[[f32; 4]; 4], mode: TransformMode, p: [f32; 4]) -> [f32; 4] {
    match mode {
        TransformMode::RowClip | TransformMode::RowScreen => [
            p[0] * rows[0][0] + p[1] * rows[0][1] + p[2] * rows[0][2] + p[3] * rows[0][3],
            p[0] * rows[1][0] + p[1] * rows[1][1] + p[2] * rows[1][2] + p[3] * rows[1][3],
            p[0] * rows[2][0] + p[1] * rows[2][1] + p[2] * rows[2][2] + p[3] * rows[2][3],
            p[0] * rows[3][0] + p[1] * rows[3][1] + p[2] * rows[3][2] + p[3] * rows[3][3],
        ],
        TransformMode::ColClip | TransformMode::ColScreen => [
            p[0] * rows[0][0] + p[1] * rows[1][0] + p[2] * rows[2][0] + p[3] * rows[3][0],
            p[0] * rows[0][1] + p[1] * rows[1][1] + p[2] * rows[2][1] + p[3] * rows[3][1],
            p[0] * rows[0][2] + p[1] * rows[1][2] + p[2] * rows[2][2] + p[3] * rows[3][2],
            p[0] * rows[0][3] + p[1] * rows[1][3] + p[2] * rows[2][3] + p[3] * rows[3][3],
        ],
    }
}

fn clip_to_screen(v: [f32; 4], vp: ShaderViewport) -> Option<(f32, f32, f32, f32)> {
    let w = v[3];
    if !w.is_finite() || w.abs() < 1.0e-6 {
        return None;
    }

    let ndc_x = v[0] / w;
    let ndc_y = v[1] / w;
    let ndc_z = v[2] / w;
    if !ndc_x.is_finite() || !ndc_y.is_finite() || !ndc_z.is_finite() {
        return None;
    }
    if ndc_x.abs() > 32.0 || ndc_y.abs() > 32.0 {
        return None;
    }

    // XDK xsasm appends c-38/c-37 scale/divide/offset for non-screenspace
    // shaders. The 0.53125 bias is the documented pixel-center/rounding offset.
    let sx = vp.x + ((ndc_x + 1.0) * 0.5 * vp.w) + 0.53125;
    let sy = vp.y + ((1.0 - ndc_y) * 0.5 * vp.h) + 0.53125;
    let sz = vp.min_z + ndc_z * (vp.max_z - vp.min_z);
    if sx.is_finite() && sy.is_finite() && sz.is_finite() {
        Some((sx, sy, sz, w))
    } else {
        None
    }
}

fn direct_screen(v: [f32; 4]) -> Option<(f32, f32, f32, f32)> {
    if !v[0].is_finite() || !v[1].is_finite() || !v[2].is_finite() {
        return None;
    }
    let w = if v[3].is_finite() && v[3].abs() >= 1.0e-6 {
        v[3]
    } else {
        1.0
    };
    Some((v[0], v[1], v[2], w))
}

fn dot4(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

fn project_simple_dp4_tail(x: f32, y: f32, z: f32) -> Option<(f32, f32, f32, f32)> {
    let constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
    let c4 = constants[4];
    let c5 = constants[5];
    let c6 = constants[6];
    let c7 = constants[7];
    let c58 = constants[58];
    let c59 = constants[59];
    drop(constants);

    let rows = [c4, c5, c6, c7];
    if rows.iter().all(|r| r.iter().all(|v| *v == 0.0)) {
        return None;
    }
    if !c58[0].is_finite()
        || !c58[1].is_finite()
        || !c58[2].is_finite()
        || !c59[0].is_finite()
        || !c59[1].is_finite()
        || !c59[2].is_finite()
    {
        return None;
    }

    let p = [x, y, z, 1.0f32];
    let raw = [dot4(p, c4), dot4(p, c5), dot4(p, c6), dot4(p, c7)];
    if raw.iter().any(|v| !v.is_finite()) {
        return None;
    }

    let w = raw[3];
    if !w.is_finite() || w.abs() < 1.0e-20 {
        return None;
    }

    let inv_w = w.signum() / w.abs().max(1.0e-20);
    let sx = raw[0] * c58[0] * inv_w + c59[0];
    let sy = raw[1] * c58[1] * inv_w + c59[1];
    let sz = raw[2] * c58[2] * inv_w + c59[2];
    if sx.is_finite() && sy.is_finite() && sz.is_finite() {
        Some((sx, sy, sz, 1.0))
    } else {
        None
    }
}

fn apply_transform(
    candidate: MatrixCandidate,
    mode: TransformMode,
    vp: ShaderViewport,
    x: f32,
    y: f32,
    z: f32,
) -> Option<(f32, f32, f32, f32)> {
    let raw = mul_matrix_point(&candidate.rows, mode, [x, y, z, 1.0]);
    match mode {
        TransformMode::RowClip | TransformMode::ColClip => clip_to_screen(raw, vp),
        TransformMode::RowScreen | TransformMode::ColScreen => direct_screen(raw),
    }
}

fn apply_transform_clip_output(
    candidate: MatrixCandidate,
    mode: TransformMode,
    vp: ShaderViewport,
    x: f32,
    y: f32,
    z: f32,
) -> Option<(f32, f32, f32, f32)> {
    let raw = mul_matrix_point(&candidate.rows, mode, [x, y, z, 1.0]);
    if raw.iter().any(|v| !v.is_finite()) {
        return None;
    }
    match mode {
        TransformMode::RowClip | TransformMode::ColClip => {
            let w = if raw[3].abs() < 1.0e-20 { 1.0 } else { raw[3] };
            Some((raw[0], raw[1], raw[2], w))
        }
        TransformMode::RowScreen | TransformMode::ColScreen => {
            let w = if raw[3].abs() < 1.0e-20 { 1.0 } else { raw[3] };
            let sx = (vp.w * 0.5).max(1.0);
            let sy = (vp.h * 0.5).max(1.0);
            let ox = vp.x + sx;
            let oy = vp.y + sy;
            let clip_x = (raw[0] - ox) / sx * w;
            let clip_y = (raw[1] - oy) / -sy * w;
            let clip_z = raw[2] * w;
            Some((clip_x, clip_y, clip_z, w))
        }
    }
}

fn score_transform(
    candidate: MatrixCandidate,
    mode: TransformMode,
    vp: ShaderViewport,
    samples: &[(f32, f32, f32)],
) -> Option<TransformStats> {
    let mut finite = 0usize;
    let mut onscreen = 0usize;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for &(x, y, z) in samples {
        if let Some((sx, sy, sz, _sw)) = apply_transform(candidate, mode, vp, x, y, z) {
            finite += 1;
            min_x = min_x.min(sx);
            min_y = min_y.min(sy);
            min_z = min_z.min(sz);
            max_x = max_x.max(sx);
            max_y = max_y.max(sy);
            max_z = max_z.max(sz);
            if sx >= -64.0 && sx <= vp.x + vp.w + 64.0 && sy >= -64.0 && sy <= vp.y + vp.h + 64.0 {
                onscreen += 1;
            }
        }
    }

    if finite == 0 || onscreen == 0 {
        return None;
    }

    let raw_width = (max_x - min_x).max(0.0);
    let raw_height = (max_y - min_y).max(0.0);
    let width = raw_width.min(vp.w * 4.0);
    let height = raw_height.min(vp.h * 4.0);
    let area = width * height;

    // The first Spider-Man post-start indexed batches expose a bad tie-break:
    // row+clip and col+clip can both put all sampled vertices on-screen, but
    // the wrong row interpretation collapses the model into a 9px sliver and
    // produces a nonsensical clip-space z around 582. Make the selector prefer
    // transforms that produce visible area and sane post-projection depth.
    let z_abs = min_z.abs().max(max_z.abs());
    let z_penalty = if z_abs > 16.0 { 8192.0 } else { 0.0 };
    let skinny_penalty = if height > 96.0 && width < 48.0 {
        (48.0 - width).max(0.0) * 768.0
    } else {
        0.0
    };
    let aspect_penalty = if width > 0.0 && height > width * 4.0 {
        ((height / width) - 4.0).min(16.0) * 2048.0
    } else {
        0.0
    };
    let vp_right = vp.x + vp.w;
    let vp_bottom = vp.y + vp.h;
    let offscreen_px = (vp.x - min_x).max(0.0)
        + (max_x - vp_right).max(0.0)
        + (vp.y - min_y).max(0.0)
        + (max_y - vp_bottom).max(0.0);
    let offscreen_penalty = (offscreen_px / (vp.w + vp.h).max(1.0)).min(32.0) * 16_384.0;
    let extent_ratio = (raw_width / vp.w.max(1.0)).max(raw_height / vp.h.max(1.0));
    if extent_ratio > 12.0 {
        return None;
    }
    let huge_extent_penalty = (extent_ratio - 2.5).max(0.0).min(32.0) * 32_768.0;
    let direct_screen_bonus = if matches!(mode, TransformMode::RowScreen | TransformMode::ColScreen)
    {
        4096.0
    } else {
        0.0
    };
    let coverage_bonus = onscreen as f32 * 1024.0;
    let finite_bonus = finite as f32 * 32.0;
    let area_bonus = (area * 0.25).min(16_384.0);
    Some(TransformStats {
        finite,
        onscreen,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        score: coverage_bonus + finite_bonus + area_bonus + direct_screen_bonus
            - z_penalty
            - skinny_penalty
            - aspect_penalty
            - offscreen_penalty
            - huge_extent_penalty,
    })
}

fn transform_is_tiny_screen(stats: TransformStats, vp: ShaderViewport) -> bool {
    let vp_area = (vp.w * vp.h).max(1.0);
    stats.width() < vp.w * 0.02 || stats.height() < vp.h * 0.02 || stats.area() < vp_area * 0.001
}

fn transform_is_reasonable_clip(stats: TransformStats, vp: ShaderViewport) -> bool {
    let width = stats.width();
    let height = stats.height();
    if width < vp.w * 0.02 || height < vp.h * 0.02 {
        return false;
    }
    if width > vp.w * 3.0 || height > vp.h * 3.0 {
        return false;
    }

    let vp_right = vp.x + vp.w;
    let vp_bottom = vp.y + vp.h;
    let offscreen_px = (vp.x - stats.min_x).max(0.0)
        + (stats.max_x - vp_right).max(0.0)
        + (vp.y - stats.min_y).max(0.0)
        + (stats.max_y - vp_bottom).max(0.0);
    offscreen_px <= (vp.w + vp.h) * 1.5
}

fn select_indexed_transform(
    guest_mem: *mut u8,
    index_data_ptr: u32,
    index_count: u32,
    vb_data: u32,
    stride: u32,
    allow_direct_screen: bool,
) -> Option<SelectedTransform> {
    let candidates = vs_wvp_candidates()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    if candidates.is_empty() || index_count == 0 {
        return None;
    }

    let sample_count = index_count.min(96);
    let step = (index_count / sample_count.max(1)).max(1);
    let mut samples = Vec::with_capacity(sample_count as usize);
    for i in (0..index_count)
        .step_by(step as usize)
        .take(sample_count as usize)
    {
        let idx_addr = index_data_ptr.wrapping_add(i.saturating_mul(2));
        if idx_addr >= 0x2000_0000u32.saturating_sub(2) {
            break;
        }
        let idx =
            unsafe { std::ptr::read_unaligned((guest_mem as u64 + idx_addr as u64) as *const u16) }
                as u32;
        let addr = vb_data.wrapping_add(idx.saturating_mul(stride));
        if addr >= 0x2000_0000u32.saturating_sub(12) {
            continue;
        }
        unsafe {
            let base = guest_mem as u64 + addr as u64;
            let x = std::ptr::read_unaligned(base as *const f32);
            let y = std::ptr::read_unaligned((base + 4) as *const f32);
            let z = std::ptr::read_unaligned((base + 8) as *const f32);
            if x.is_finite() && y.is_finite() && z.is_finite() {
                samples.push((x, y, z));
            }
        }
    }
    if samples.is_empty() {
        return None;
    }

    let vp = *shader_viewport().lock().unwrap_or_else(|e| e.into_inner());
    let choose_best = |modes: &[TransformMode]| {
        let mut best: Option<SelectedTransform> = None;
        for candidate in candidates.iter().rev().take(8).copied() {
            for mode in modes.iter().copied() {
                if let Some(stats) = score_transform(candidate, mode, vp, &samples) {
                    if best
                        .as_ref()
                        .map_or(true, |current| stats.score > current.stats.score)
                    {
                        best = Some(SelectedTransform {
                            matrix: candidate,
                            mode,
                            viewport: vp,
                            stats,
                        });
                    }
                }
            }
        }
        best
    };

    if allow_direct_screen {
        let direct = choose_best(&[TransformMode::RowScreen, TransformMode::ColScreen]);
        let clip = choose_best(&[TransformMode::RowClip, TransformMode::ColClip]);

        if let Some(direct_sel) = direct {
            if transform_is_tiny_screen(direct_sel.stats, vp) {
                if let Some(clip_sel) = clip {
                    let clip_reasonable = transform_is_reasonable_clip(clip_sel.stats, vp);
                    let clip_non_tiny = !transform_is_tiny_screen(clip_sel.stats, vp);
                    if clip_reasonable || clip_non_tiny {
                        static FALLBACK_LOG: AtomicU32 = AtomicU32::new(0);
                        let n = FALLBACK_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 16 || n.is_power_of_two() {
                            debug_log(&format!(
                                "[VS-XFORM-TINY-SCREEN-FALLBACK] direct={} bbox=[{:.1},{:.1}..{:.1},{:.1}] clip={} bbox=[{:.1},{:.1}..{:.1},{:.1}] reasonable={} non_tiny={}",
                                direct_sel.mode.label(),
                                direct_sel.stats.min_x,
                                direct_sel.stats.min_y,
                                direct_sel.stats.max_x,
                                direct_sel.stats.max_y,
                                clip_sel.mode.label(),
                                clip_sel.stats.min_x,
                                clip_sel.stats.min_y,
                                clip_sel.stats.max_x,
                                clip_sel.stats.max_y,
                                clip_reasonable,
                                clip_non_tiny,
                            ));
                        }
                        return Some(clip_sel);
                    }
                }
            }
            Some(direct_sel)
        } else {
            clip
        }
    } else {
        choose_best(&[TransformMode::RowClip, TransformMode::ColClip])
    }
}

pub(super) fn hle_set_vertex_shader_constant(
    args: &[u32; 8],
    guest_mem: *mut u8,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) -> u32 {
    drain_draws_before_state_change("SetVertexShaderConstant");
    spiderman_seed_shader_constant_registers(guest_mem);
    let raw_reg = args[0];
    let Some(reg) = normalize_vs_constant_register(raw_reg) else {
        static BAD_REG_LOG: AtomicU32 = AtomicU32::new(0);
        let n = BAD_REG_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[VS-CONST-BAD-REG] raw=0x{:08X} src=0x{:08X} count={}",
                raw_reg, args[1], args[2]
            ));
        }
        return 0;
    };
    let src = args[1];
    let count = args[2] as usize;
    if src == 0 || src >= 0x2000_0000 || count == 0 {
        return 0;
    }

    let max_count = count.min(VS_CONSTANT_COUNT - reg).min(64);
    let mut first = [[0.0f32; 4]; 4];
    let mut uploaded = Vec::with_capacity(max_count * 4);
    let mut spidey_probe_rows: Vec<(usize, u32, [f32; 4])> = Vec::new();
    {
        let mut constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
        for i in 0..max_count {
            let addr = src.wrapping_add((i * 16) as u32);
            if addr >= 0x2000_0000u32.saturating_sub(16) {
                break;
            }
            unsafe {
                let p = guest_mem.add(addr as usize) as *const f32;
                let row = [
                    std::ptr::read_unaligned(p.add(0)),
                    std::ptr::read_unaligned(p.add(1)),
                    std::ptr::read_unaligned(p.add(2)),
                    std::ptr::read_unaligned(p.add(3)),
                ];
                constants[reg + i] = row;
                if i < first.len() {
                    first[i] = row;
                }
                let c = reg + i;
                if (8..=16).contains(&c) {
                    spidey_probe_rows.push((c, addr, row));
                }
                uploaded.extend_from_slice(&row);
            }
        }
    }
    if max_count >= 4 {
        record_wvp_candidate(src, first);
    }

    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_vs_constant(reg as u32, &uploaded);
    }

    if spiderman_fun29c5a0_svsc_canary_enabled() && reg < 4 && reg.saturating_add(max_count) > 0 {
        apply_spiderman_fun29c5a0_svsc_canary(&format!("after_upload_c{}_count{}", reg, max_count));
    }

    if raw_reg == 0xFFFF_FFA1 || reg == 1 {
        static C_MINUS_95_LOG: AtomicU32 = AtomicU32::new(0);
        let k = C_MINUS_95_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 32 || k.is_power_of_two() {
            debug_log(&format!(
                "[VS-CONST-C-MINUS95] #{} raw_signed={} raw=0x{:08X} mapped=c{} count={} src=0x{:08X} ret=0x{:08X} c1=[{:.6},{:.6},{:.6},{:.6}]",
                k,
                raw_reg as i32,
                raw_reg,
                reg,
                count,
                src,
                ret_addr,
                first[0][0],
                first[0][1],
                first[0][2],
                first[0][3],
            ));
        }
    }

    match ret_addr {
        0x0029_C739 | 0x0029_C7C8 | 0x0029_C83E | 0x0029_C871 => {
            static SPIDEY_INIT_CALL_LOG: AtomicU32 = AtomicU32::new(0);
            let k = SPIDEY_INIT_CALL_LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[SPIDEY-FUN29C5A0-SVSC-CALL] #{} ret=0x{:08X} raw_signed={} raw=0x{:08X} mapped=c{} count={} src=0x{:08X} first=[{:.6},{:.6},{:.6},{:.6}]",
                k,
                ret_addr,
                raw_reg as i32,
                raw_reg,
                reg,
                count,
                src,
                first[0][0],
                first[0][1],
                first[0][2],
                first[0][3],
            ));
        }
        _ => {}
    }

    if cxbxr_reserved_vs_constants_enabled() && shader_constant_mode_uses_reserved_constants() {
        let end = reg.saturating_add(max_count);
        if reg < 60 && end > 58 {
            let skip_for_spidey_bones = spidey_allow_reserved_bone_rows_enabled()
                && spidey_reserved_bone_row_upload(raw_reg, reg, max_count);
            if !skip_for_spidey_bones {
                let vp = *shader_viewport().lock().unwrap_or_else(|e| e.into_inner());
                upload_reserved_screenspace_constants(vp);
            }
            static CXBXR_RESERVED_LOG: AtomicU32 = AtomicU32::new(0);
            let k = CXBXR_RESERVED_LOG.fetch_add(1, Ordering::Relaxed);
            if k < 16 || k.is_power_of_two() {
                debug_log(&format!(
                    "[VS-CONST-CXBXR-RESERVED-RESTORE] #{} raw=0x{:08X} c{} count={} action={}",
                    k,
                    raw_reg,
                    reg,
                    max_count,
                    if skip_for_spidey_bones {
                        "skip_spidey_bone_rows"
                    } else {
                        "restored_c58_c59"
                    }
                ));
            }
        }
    }

    if spidey_vs_const_source_probe_enabled()
        && !spidey_probe_rows.is_empty()
        && crate::xbox::emulator::spidey_synth_handoff_count() != 0
    {
        static SPIDEY_VS_CONST_SRC_LOG: AtomicU32 = AtomicU32::new(0);
        let k = SPIDEY_VS_CONST_SRC_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 256 || k.is_power_of_two() {
            use std::fmt::Write as _;

            let mut rows = String::new();
            for (idx, (c, addr, row)) in spidey_probe_rows.iter().enumerate() {
                let _ = write!(
                    rows,
                    "{}c{}@0x{:08X}=[{:.4},{:.4},{:.4},{:.4}]",
                    if idx == 0 { "" } else { " " },
                    c,
                    addr,
                    row[0],
                    row[1],
                    row[2],
                    row[3]
                );
            }

            let mut source_ctx = String::new();
            let scratch_like_upload = (0x004D_1520..0x004D_1600).contains(&src)
                || (0x0029_EB00..=0x0029_ECFF).contains(&ret_addr);
            if scratch_like_upload {
                let esi = regs[4];
                if let Some(source_obj) = read_u32_guest(guest_mem, esi.wrapping_sub(4)) {
                    if normalize_guest_ram_ptr(source_obj).is_some() {
                        let _ = write!(
                            source_ctx,
                            "esi_minus4=0x{:08X} source_obj=0x{:08X}",
                            esi.wrapping_sub(4),
                            source_obj
                        );
                        for off in [
                            -8i32, -4, 0, 4, 8, 0xC, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28,
                            0x2C, 0x30, 0x34, 0x38, 0x3C, 0x40,
                        ] {
                            let addr = if off < 0 {
                                source_obj.wrapping_sub((-off) as u32)
                            } else {
                                source_obj.wrapping_add(off as u32)
                            };
                            if let Some(word) = read_u32_guest(guest_mem, addr) {
                                let _ = write!(
                                    source_ctx,
                                    " {}{:02X}=0x{:08X}",
                                    if off < 0 { "-" } else { "+" },
                                    off.unsigned_abs(),
                                    word
                                );
                            }
                        }

                        for off in [0u32, 0x10, 0x20, 0x30, 0x40] {
                            if let (Some(x), Some(y), Some(z), Some(w)) = (
                                read_f32_guest(guest_mem, source_obj.wrapping_add(off)),
                                read_f32_guest(guest_mem, source_obj.wrapping_add(off + 4)),
                                read_f32_guest(guest_mem, source_obj.wrapping_add(off + 8)),
                                read_f32_guest(guest_mem, source_obj.wrapping_add(off + 12)),
                            ) {
                                let finite = x.is_finite()
                                    && y.is_finite()
                                    && z.is_finite()
                                    && w.is_finite();
                                let moderate = x.abs() <= 10000.0
                                    && y.abs() <= 10000.0
                                    && z.abs() <= 10000.0
                                    && w.abs() <= 10000.0;
                                if finite && moderate {
                                    let _ = write!(
                                        source_ctx,
                                        " f+{:02X}=[{:.4},{:.4},{:.4},{:.4}]",
                                        off, x, y, z, w
                                    );
                                }
                            }
                        }
                    }
                }
            }

            let (post_draws, post_skin_draws, scene_ticks, handoff_state, handoff_frame, _, _, _) =
                crate::xbox::emulator::spidey_synth_branch_snapshot();
            let current_vs = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
            let stream0_stride = crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.load(Ordering::Relaxed);

            debug_log(&format!(
                "[SPIDEY-VS-CONST-SRC] #{} raw=0x{:08X} c{} count={} max_count={} src=0x{:08X} ret=0x{:08X} esp=0x{:08X} regs eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} current_vs=0x{:08X} stream0_stride={} handoff_state=0x{:08X} handoff_frame={} post_draws={} post_skin_draws={} scene_ticks={} rows=[{}] source_ctx=[{}]",
                k,
                raw_reg,
                reg,
                count,
                max_count,
                src,
                ret_addr,
                r14,
                regs[0],
                regs[1],
                regs[2],
                regs[3],
                regs[4],
                regs[5],
                current_vs,
                stream0_stride,
                handoff_state,
                handoff_frame,
                post_draws,
                post_skin_draws,
                scene_ticks,
                rows,
                source_ctx
            ));
        }
    }

    if vs_upload_intersects_skin_window(reg, max_count, raw_reg) {
        static VS_SKIN_CONST_LOG: AtomicU32 = AtomicU32::new(0);
        let k = VS_SKIN_CONST_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 48 || k.is_power_of_two() {
            debug_log(&format!(
                "[VS-CONST-SKIN] #{} raw_signed={} raw=0x{:08X} mapped=c{} count={} src=0x{:08X} c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}]",
                k,
                raw_reg as i32,
                raw_reg,
                reg,
                count,
                src,
                reg,
                first[0][0],
                first[0][1],
                first[0][2],
                first[0][3],
                reg + 1,
                first[1][0],
                first[1][1],
                first[1][2],
                first[1][3],
                reg + 2,
                first[2][0],
                first[2][1],
                first[2][2],
                first[2][3],
                reg + 3,
                first[3][0],
                first[3][1],
                first[3][2],
                first[3][3],
            ));
        }
    }

    static VS_CONST_LOG: AtomicU32 = AtomicU32::new(0);
    let n = VS_CONST_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32
        || (raw_reg != reg as u32 && n < 256)
        || (reg == 0 && (n < 128 || n.is_power_of_two()))
    {
        debug_log(&format!(
            "[VS-CONST] #{} raw=0x{:08X} c{} count={} src=0x{:08X} c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}] c{}=[{:.4},{:.4},{:.4},{:.4}]",
            n,
            raw_reg,
            reg,
            count,
            src,
            reg,
            first[0][0],
            first[0][1],
            first[0][2],
            first[0][3],
            reg + 1,
            first[1][0],
            first[1][1],
            first[1][2],
            first[1][3],
            reg + 2,
            first[2][0],
            first[2][1],
            first[2][2],
            first[2][3],
            reg + 3,
            first[3][0],
            first[3][1],
            first[3][2],
            first[3][3],
        ));
    }

    0
}

fn ps_float_to_unorm8(value: f32) -> u32 {
    if !value.is_finite() {
        return 0;
    }
    ((value.clamp(0.0, 1.0) * 255.0) + 0.5) as u32
}

fn ps_float4_to_d3dcolor(row: [f32; 4]) -> u32 {
    let r = ps_float_to_unorm8(row[0]);
    let g = ps_float_to_unorm8(row[1]);
    let b = ps_float_to_unorm8(row[2]);
    let a = ps_float_to_unorm8(row[3]);
    (a << 24) | (r << 16) | (g << 8) | b
}

fn ps_route_global_constant_to_stage_slots(global_reg: usize, row: [f32; 4]) -> Vec<(u32, u32)> {
    if global_reg > 15 || !PS_MAPPING_VALID.load(Ordering::Relaxed) {
        return Vec::new();
    }

    let color = ps_float4_to_d3dcolor(row);
    let mut routes = Vec::new();
    {
        let mut states = ps_render_states().lock().unwrap_or_else(|e| e.into_inner());
        let c0_mapping = states[57];
        let c1_mapping = states[58];
        let global_reg = global_reg as u32;
        for stage in 0..8usize {
            let shift = (stage * 4) as u32;
            if ((c0_mapping >> shift) & 0xF) == global_reg {
                let slot = 10 + stage;
                states[slot] = color;
                routes.push((slot as u32, color));
            }
            if ((c1_mapping >> shift) & 0xF) == global_reg {
                let slot = 18 + stage;
                states[slot] = color;
                routes.push((slot as u32, color));
            }
        }
    }

    if !routes.is_empty() {
        PS_RENDER_STATE_SEQ.fetch_add(1, Ordering::Relaxed);
        static PS_CONST_MAP_LOG: AtomicU32 = AtomicU32::new(0);
        let n = PS_CONST_MAP_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 64 || n.is_power_of_two() {
            let route_text = routes
                .iter()
                .map(|(slot, value)| {
                    format!("{}=0x{:08X}", pixel_shader_render_state_name(*slot), value)
                })
                .collect::<Vec<_>>()
                .join(",");
            debug_log(&format!(
                "[PS-CONST-MAP] #{} reg={} color=0x{:08X} routes=[{}]",
                n, global_reg, color, route_text
            ));
        }
    }

    routes
}

fn hle_set_pixel_shader_constant(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    drain_draws_before_state_change("SetPixelShaderConstant");
    let reg = args[0] as usize;
    let src = args[1];
    let count = args[2] as usize;
    if src == 0 || count == 0 {
        return 0;
    }

    let Some(src_norm) = normalize_guest_ram_ptr(src) else {
        static BAD_SRC_LOG: AtomicU32 = AtomicU32::new(0);
        let n = BAD_SRC_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[PS-CONST-BAD-SRC] reg={} src=0x{:08X} count={}",
                reg, src, count
            ));
        }
        return 0;
    };
    if reg >= 32 || src_norm >= 0x2000_0000u32.saturating_sub(16) {
        return 0;
    }

    let max_count = count.min(32 - reg).min(16);
    let mut first = [[0.0f32; 4]; 2];
    let mut first_u32 = [[0u32; 4]; 2];
    let mut uploaded = Vec::with_capacity(max_count * 4);
    let mut mapped_routes = Vec::new();
    {
        let mut constants = ps_constants().lock().unwrap_or_else(|e| e.into_inner());
        for i in 0..max_count {
            let addr = src_norm.wrapping_add((i * 16) as u32);
            if addr >= 0x2000_0000u32.saturating_sub(16) {
                break;
            }
            unsafe {
                let p = guest_mem.add(addr as usize);
                let row = [
                    std::ptr::read_unaligned(p as *const f32),
                    std::ptr::read_unaligned(p.add(4) as *const f32),
                    std::ptr::read_unaligned(p.add(8) as *const f32),
                    std::ptr::read_unaligned(p.add(12) as *const f32),
                ];
                let row_u32 = [
                    std::ptr::read_unaligned(p as *const u32),
                    std::ptr::read_unaligned(p.add(4) as *const u32),
                    std::ptr::read_unaligned(p.add(8) as *const u32),
                    std::ptr::read_unaligned(p.add(12) as *const u32),
                ];
                constants[reg + i] = row;
                if i < first.len() {
                    first[i] = row;
                    first_u32[i] = row_u32;
                }
                uploaded.extend_from_slice(&row);
                mapped_routes.extend(ps_route_global_constant_to_stage_slots(reg + i, row));
            }
        }
    }

    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_ps_constant(reg as u32, &uploaded);
        for (state, value) in mapped_routes.iter().copied() {
            gpu.set_render_state(state, value);
        }
    }

    static PS_CONST_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PS_CONST_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        debug_log(&format!(
            "[PS-CONST] #{} reg={} count={} src=0x{:08X}->0x{:08X} c{}=[{:.4},{:.4},{:.4},{:.4}] u=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] c{}=[{:.4},{:.4},{:.4},{:.4}] u=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}]",
            n,
            reg,
            count,
            src,
            src_norm,
            reg,
            first[0][0],
            first[0][1],
            first[0][2],
            first[0][3],
            first_u32[0][0],
            first_u32[0][1],
            first_u32[0][2],
            first_u32[0][3],
            reg + 1,
            first[1][0],
            first[1][1],
            first[1][2],
            first[1][3],
            first_u32[1][0],
            first_u32[1][1],
            first_u32[1][2],
            first_u32[1][3]
        ));
    }

    0
}

fn project_with_c0_c3(x: f32, y: f32, z: f32) -> Option<(f32, f32, f32, f32)> {
    let c = *vs_wvp_candidate().lock().unwrap_or_else(|e| e.into_inner());
    let vp = *shader_viewport().lock().unwrap_or_else(|e| e.into_inner());

    if c.iter().all(|r| r.iter().all(|v| *v == 0.0)) {
        return None;
    }

    let p = [x, y, z, 1.0f32];
    let row = mul_matrix_point(&c, TransformMode::RowClip, p);
    let col = mul_matrix_point(&c, TransformMode::ColClip, p);
    clip_to_screen(row, vp).or_else(|| clip_to_screen(col, vp))
}

fn log_simple_dp4_probe(draw_n: u32, shader: u32, verts: &[NV2AVertex]) {
    if verts.is_empty() {
        return;
    }

    let rows0 = [
        current_vs_constant(0),
        current_vs_constant(1),
        current_vs_constant(2),
        current_vs_constant(3),
    ];
    let rows4 = [
        current_vs_constant(4),
        current_vs_constant(5),
        current_vs_constant(6),
        current_vs_constant(7),
    ];
    let mut c0_min = [f32::INFINITY; 4];
    let mut c0_max = [f32::NEG_INFINITY; 4];
    let mut c4_min = [f32::INFINITY; 4];
    let mut c4_max = [f32::NEG_INFINITY; 4];

    for v in verts {
        let p = [v.x, v.y, v.z, 1.0f32];
        for i in 0..4 {
            let out0 = dot4(p, rows0[i]);
            let out4 = dot4(p, rows4[i]);
            c0_min[i] = c0_min[i].min(out0);
            c0_max[i] = c0_max[i].max(out0);
            c4_min[i] = c4_min[i].min(out4);
            c4_max[i] = c4_max[i].max(out4);
        }
    }

    let first = verts[0];
    let p = [first.x, first.y, first.z, 1.0f32];
    let first0 = [
        dot4(p, rows0[0]),
        dot4(p, rows0[1]),
        dot4(p, rows0[2]),
        dot4(p, rows0[3]),
    ];
    let first4 = [
        dot4(p, rows4[0]),
        dot4(p, rows4[1]),
        dot4(p, rows4[2]),
        dot4(p, rows4[3]),
    ];

    debug_log(&format!(
        "[HLE-DRAWIDX-DP4-PROBE] draw#{} shader=0x{:08X} verts={} first_raw=({:.3},{:.3},{:.3},w={:.3}) c0_first=[{:.3},{:.3},{:.3},{:.3}] c0_bbox=[{:.1},{:.1},{:.3},{:.3}..{:.1},{:.1},{:.3},{:.3}] c4_first=[{:.3},{:.3},{:.3},{:.3}] c4_bbox=[{:.1},{:.1},{:.3},{:.3}..{:.1},{:.1},{:.3},{:.3}]",
        draw_n,
        shader,
        verts.len(),
        first.x,
        first.y,
        first.z,
        first.w,
        first0[0],
        first0[1],
        first0[2],
        first0[3],
        c0_min[0],
        c0_min[1],
        c0_min[2],
        c0_min[3],
        c0_max[0],
        c0_max[1],
        c0_max[2],
        c0_max[3],
        first4[0],
        first4[1],
        first4[2],
        first4[3],
        c4_min[0],
        c4_min[1],
        c4_min[2],
        c4_min[3],
        c4_max[0],
        c4_max[1],
        c4_max[2],
        c4_max[3],
    ));
}

fn spiderman_scrub_bad_render_callback_ptrs(guest_mem: *mut u8) {
    fn valid_guest_ptr(addr: u32) -> bool {
        addr != 0 && addr < 0x2000_0000
    }

    fn plausible_code_ptr(addr: u32) -> bool {
        (0x0001_0000..0x0080_0000).contains(&addr) || (0x8001_0000..0x8080_0000).contains(&addr)
    }

    unsafe {
        let base = guest_mem as u64;
        let app = *((base + 0x003F_5EB0) as *const u32);
        let frame = *((base + 0x004B_C630) as *const u32);
        if !valid_guest_ptr(app) || !valid_guest_ptr(frame) {
            return;
        }

        static SCRUB_LOG: AtomicU32 = AtomicU32::new(0);
        for &slot in &[0x006F_2CC0u32, 0x006F_2CD0u32] {
            let p = (base + slot as u64) as *mut u32;
            let value = *p;
            if value != 0 && !plausible_code_ptr(value) {
                *p = 0;
                let n = SCRUB_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 8 || n.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-CALLBACK-SCRUB] #{} slot=0x{:08X} bad=0x{:08X}->0",
                        n, slot, value
                    ));
                }
            }
        }
    }
}

fn xapi_is_gamepad_device_type(device_type: u32) -> bool {
    // Spider-Man XDK 4134 passes pointers to static XDEVICE_TYPE descriptors.
    // 0x00379C00 is the gamepad descriptor; 0x00379C70 is the memory-unit
    // descriptor seen in save-scan logs. Reporting gamepads for every device
    // type makes the title attempt XMountMUA and consume an uninitialized drive
    // string such as 0x14:\.
    if device_type == 0x0037_9C70 {
        return false;
    }
    device_type == 0x0037_9C00 || (0x1000..0x2000_0000).contains(&device_type)
}

fn xapi_device_connected_mask(device_type: u32) -> u32 {
    if xapi_is_gamepad_device_type(device_type) {
        0x0000_0001
    } else {
        0
    }
}

fn xapi_device_init_state() -> &'static Mutex<Option<std::time::Instant>> {
    static INIT_AT: OnceLock<Mutex<Option<std::time::Instant>>> = OnceLock::new();
    INIT_AT.get_or_init(|| Mutex::new(None))
}

fn xapi_mark_devices_initializing() {
    let mut guard = xapi_device_init_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *guard = Some(std::time::Instant::now());
}

fn xapi_devices_initializing() -> bool {
    let guard = xapi_device_init_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    guard
        .as_ref()
        .map(|t| t.elapsed() < std::time::Duration::from_millis(500))
        .unwrap_or(false)
}

fn xapi_device_type_state(guest_mem: *mut u8, device_type: u32) -> (u32, u32, u32) {
    if device_type == 0 || device_type >= 0x2000_0000 {
        return (0, 0, 0);
    }
    unsafe {
        let base = guest_mem as u64 + device_type as u64;
        (
            *((base + 0x00) as *const u32),
            *((base + 0x04) as *const u32),
            *((base + 0x08) as *const u32),
        )
    }
}

fn xapi_write_device_type_state(
    guest_mem: *mut u8,
    device_type: u32,
    current: u32,
    changed: u32,
    previous: u32,
) {
    if device_type == 0 || device_type >= 0x2000_0000 {
        return;
    }
    unsafe {
        let base = guest_mem as u64 + device_type as u64;
        *((base + 0x00) as *mut u32) = current;
        *((base + 0x04) as *mut u32) = changed;
        *((base + 0x08) as *mut u32) = previous;
    }
}

fn xapi_seed_device_type_if_empty(guest_mem: *mut u8, device_type: u32) {
    let (current, changed, previous) = xapi_device_type_state(guest_mem, device_type);
    if current == 0 && changed == 0 && previous == 0 {
        let connected = xapi_device_connected_mask(device_type);
        xapi_write_device_type_state(guest_mem, device_type, connected, connected, 0);
    }
}

pub(super) fn xapi_hle_init_devices(guest_mem: *mut u8) -> u32 {
    xapi_mark_devices_initializing();
    xapi_write_device_type_state(guest_mem, 0x0037_9C00, 1, 1, 0);
    xapi_write_device_type_state(guest_mem, 0x0037_9C70, 0, 0, 0);

    static XINIT_LOG: AtomicU32 = AtomicU32::new(0);
    let n = XINIT_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        debug_log("[OOVPA-HLE] XInitDevices: seeded XPP gamepad device state");
    }
    0
}

pub(super) fn xapi_hle_get_devices(guest_mem: *mut u8, device_type: u32, log_tag: &str) -> u32 {
    xapi_seed_device_type_if_empty(guest_mem, device_type);
    let (current, changed, previous) = xapi_device_type_state(guest_mem, device_type);
    let initializing = xapi_devices_initializing();
    let connected = if initializing { 0 } else { current };

    static XGET_DEVICES_LOG: AtomicU32 = AtomicU32::new(0);
    let n = XGET_DEVICES_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 24 || device_type != 0x0037_9C00 || n.is_power_of_two() {
        debug_log(&format!(
            "[OOVPA-HLE] {} type=0x{:08X} cur=0x{:X} chg=0x{:X} prev=0x{:X} init={} -> 0x{:X}",
            log_tag, device_type, current, changed, previous, initializing, connected
        ));
    }
    if menu_wait_trace_log(n) {
        debug_log(&format!(
            "[MENU-WAIT] {} #{} type=0x{:08X} cur=0x{:X} chg_before=0x{:X} prev_before=0x{:X} init={} ret=0x{:X} chg_after=0x{:X} prev_after=0x{:X} consumes_change=0",
            log_tag,
            n,
            device_type,
            current,
            changed,
            previous,
            initializing,
            connected,
            changed,
            previous
        ));
    }
    connected
}

pub(super) fn xapi_hle_get_device_changes(
    guest_mem: *mut u8,
    device_type: u32,
    insertions_ptr: u32,
    removals_ptr: u32,
) -> u32 {
    if xapi_devices_initializing() {
        write_guest_u32_if_valid(guest_mem, insertions_ptr, 0);
        write_guest_u32_if_valid(guest_mem, removals_ptr, 0);
        static INIT_CHANGE_LOG: AtomicU32 = AtomicU32::new(0);
        let n = INIT_CHANGE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE] XGetDeviceChanges #{} type=0x{:08X} initializing -> none",
                n, device_type
            ));
        }
        if menu_wait_trace_log(n) {
            debug_log(&format!(
                "[MENU-WAIT] XGetDeviceChanges #{} type=0x{:08X} init=1 insert=0x0 remove=0x0 ret=0",
                n, device_type
            ));
        }
        return 0;
    }

    xapi_seed_device_type_if_empty(guest_mem, device_type);
    let (current, changed, previous) = xapi_device_type_state(guest_mem, device_type);
    let (mut insertions, mut removals) = if changed != 0 {
        (current & !previous, previous & !current)
    } else {
        (0, 0)
    };
    if changed != 0 {
        let remove_insert = changed & current & previous;
        insertions |= remove_insert;
        removals |= remove_insert;
    }

    xapi_write_device_type_state(guest_mem, device_type, current, 0, current);
    write_guest_u32_if_valid(guest_mem, insertions_ptr, insertions);
    write_guest_u32_if_valid(guest_mem, removals_ptr, removals);

    static LOG_CTR: AtomicU32 = AtomicU32::new(0);
    let n = LOG_CTR.fetch_add(1, Ordering::Relaxed);
    let changed_now = insertions != 0 || removals != 0;
    if n < 16 || changed_now || device_type != 0x0037_9C00 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE] XGetDeviceChanges #{} type=0x{:08X} cur=0x{:X} chg=0x{:X} prev=0x{:X} insert=0x{:X} remove=0x{:X}",
            n, device_type, current, changed, previous, insertions, removals
        ));
    }
    if menu_wait_trace_log(n) || changed_now {
        debug_log(&format!(
            "[MENU-WAIT] XGetDeviceChanges #{} type=0x{:08X} cur=0x{:X} chg_before=0x{:X} prev_before=0x{:X} insert=0x{:X} remove=0x{:X} ret={}",
            n,
            device_type,
            current,
            changed,
            previous,
            insertions,
            removals,
            if changed_now { 1 } else { 0 }
        ));
    }

    if changed_now {
        1
    } else {
        0
    }
}

pub(super) fn xapi_hle_input_open(args: &[u32; 8]) -> u32 {
    // XInputOpen(DeviceType, dwPort, dwSlot, pPollingParams) -> HANDLE.
    // Model a single connected gamepad on port 0.
    let port = args[1];
    static OPEN_LOG: AtomicU32 = AtomicU32::new(0);
    let n = OPEN_LOG.fetch_add(1, Ordering::Relaxed);
    if port == 0 {
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[OOVPA-HLE] XInputOpen/interp: type=0x{:08X} port={} slot={} -> fake handle",
                args[0], port, args[2]
            ));
        }
        0x8000_0000
    } else {
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[OOVPA-HLE] XInputOpen/interp: type=0x{:08X} port={} slot={} -> no device",
                args[0], port, args[2]
            ));
        }
        0
    }
}

fn apply_doom_autonewgame_input(log_tag: &str, buttons: &mut u16, analog_a: &mut u8) {
    if std::env::var_os("RUSTEMU_DOOM_AUTONEWGAME").is_none() {
        return;
    }

    static DOOM_AUTONEWGAME_POLL: AtomicU32 = AtomicU32::new(0);
    static DOOM_AUTONEWGAME_LOG: AtomicU32 = AtomicU32::new(0);

    let poll = DOOM_AUTONEWGAME_POLL.fetch_add(1, Ordering::Relaxed);
    let start_pulse = (4..=9).contains(&poll);
    let a_pulse = (32..=38).contains(&poll)
        || (64..=70).contains(&poll)
        || (96..=102).contains(&poll)
        || (144..=150).contains(&poll)
        || (192..=198).contains(&poll);

    if start_pulse {
        *buttons |= 0x0010;
    }
    if a_pulse {
        *analog_a = 0xFF;
    }

    let log_n = DOOM_AUTONEWGAME_LOG.fetch_add(1, Ordering::Relaxed);
    if start_pulse || a_pulse || log_n < 12 || log_n.is_power_of_two() {
        debug_log(&format!(
            "[DOOM-AUTONEWGAME] {} #{} poll={} START={} A={} buttons=0x{:04X}",
            log_tag,
            log_n,
            poll,
            if start_pulse { 1 } else { 0 },
            if a_pulse { 255 } else { 0 },
            *buttons
        ));
    }
}

pub(super) fn xapi_hle_input_get_state(args: &[u32; 8], guest_mem: *mut u8, log_tag: &str) -> u32 {
    let state_ptr = args[1];
    if state_ptr == 0 || state_ptr >= 0x2000_0000 {
        return 0;
    }

    const JP_SOUTH: u32 = 0;
    const JP_WEST: u32 = 1;
    const JP_SELECT: u32 = 2;
    const JP_START: u32 = 3;
    const JP_UP: u32 = 4;
    const JP_DOWN: u32 = 5;
    const JP_LEFT: u32 = 6;
    const JP_RIGHT: u32 = 7;
    const JP_EAST: u32 = 8;
    const JP_NORTH: u32 = 9;
    const JP_L: u32 = 10;
    const JP_R: u32 = 11;
    const JP_L2: u32 = 12;
    const JP_R2: u32 = 13;
    const JP_L3: u32 = 14;
    const JP_R3: u32 = 15;

    let (raw_joypad_bits, lx, ly, rx, ry) = crate::xbox::emulator::input_snapshot();
    let latched_joypad_bits = crate::xbox::emulator::input_latched_edge_bits();
    let joypad_bits = raw_joypad_bits | latched_joypad_bits;
    let rd = |id: u32| -> bool { (joypad_bits & (1u32 << id)) != 0 };

    let mut buttons: u16 = 0;
    if rd(JP_UP) {
        buttons |= 0x0001;
    }
    if rd(JP_DOWN) {
        buttons |= 0x0002;
    }
    if rd(JP_LEFT) {
        buttons |= 0x0004;
    }
    if rd(JP_RIGHT) {
        buttons |= 0x0008;
    }
    if rd(JP_START) {
        buttons |= 0x0010;
    }
    if rd(JP_SELECT) {
        buttons |= 0x0020;
    }
    if rd(JP_L3) {
        buttons |= 0x0040;
    }
    if rd(JP_R3) {
        buttons |= 0x0080;
    }

    let pressure = |id: u32| -> u8 {
        if rd(id) {
            0xFF
        } else {
            0
        }
    };
    // Libretro names buttons in SNES layout terms: B=south, A=east,
    // Y=west, X=north. Translate by physical position to Xbox ABXY.
    let mut a_a = pressure(JP_SOUTH);
    let a_b = pressure(JP_EAST);
    let a_x = pressure(JP_WEST);
    let a_y = pressure(JP_NORTH);
    let a_black = pressure(JP_L);
    let a_white = pressure(JP_R);
    let a_lt = pressure(JP_L2);
    let a_rt = pressure(JP_R2);

    apply_doom_autonewgame_input(log_tag, &mut buttons, &mut a_a);

    let sig1 = (buttons as u64)
        | ((a_a as u64) << 16)
        | ((a_b as u64) << 24)
        | ((a_x as u64) << 32)
        | ((a_y as u64) << 40)
        | ((a_black as u64) << 48)
        | ((a_white as u64) << 56);
    let sig2 = (a_lt as u64)
        | ((a_rt as u64) << 8)
        | (((lx as u16) as u64) << 16)
        | (((ly as u16) as u64) << 32)
        | (((rx as u16) as u64) << 48);
    let sig3 = ry as u16 as u32;
    static LAST_SIG1: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static LAST_SIG2: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static LAST_SIG3: AtomicU32 = AtomicU32::new(0);
    static PACKET: AtomicU32 = AtomicU32::new(0);
    let packet = if LAST_SIG1.load(Ordering::Relaxed) != sig1
        || LAST_SIG2.load(Ordering::Relaxed) != sig2
        || LAST_SIG3.load(Ordering::Relaxed) != sig3
    {
        LAST_SIG1.store(sig1, Ordering::Relaxed);
        LAST_SIG2.store(sig2, Ordering::Relaxed);
        LAST_SIG3.store(sig3, Ordering::Relaxed);
        PACKET.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    } else {
        PACKET.load(Ordering::Relaxed)
    };

    unsafe {
        let base = (guest_mem as u64 + state_ptr as u64) as *mut u8;
        *(base as *mut u32) = packet;
        *((base as u64 + 4) as *mut u16) = buttons;
        *base.add(6) = a_a;
        *base.add(7) = a_b;
        *base.add(8) = a_x;
        *base.add(9) = a_y;
        *base.add(10) = a_black;
        *base.add(11) = a_white;
        *base.add(12) = a_lt;
        *base.add(13) = a_rt;
        *((base as u64 + 14) as *mut i16) = lx;
        *((base as u64 + 16) as *mut i16) = ly;
        *((base as u64 + 18) as *mut i16) = rx;
        *((base as u64 + 20) as *mut i16) = ry;
    }

    static STATE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = STATE_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 12 || buttons != 0 || a_a != 0 || a_b != 0 || n.is_power_of_two() {
        debug_log(&format!(
            "[OOVPA-HLE] {} #{} hDev=0x{:08X} state=0x{:08X} packet={} raw=0x{:04X} buttons=0x{:04X} A={} B={} X={} Y={} LX={} LY={}",
            log_tag,
            n,
            args[0],
            state_ptr,
            packet,
            joypad_bits,
            buttons,
            a_a,
            a_b,
            a_x,
            a_y,
            lx,
            ly
        ));
    }
    if a_a != 0 || (buttons & 0x0020) != 0 {
        crate::xbox::emulator::mark_test_confirm_sampled();
    }
    0
}

pub(super) fn xapi_hle_input_get_capabilities(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let caps_ptr = args[1];
    if caps_ptr > 0 && caps_ptr < 0x2000_0000 {
        unsafe {
            let caps = (guest_mem as u64 + caps_ptr as u64) as *mut u8;
            std::ptr::write_bytes(caps, 0, 0x1C);
            *caps.add(0) = 0x01;
            std::ptr::write_bytes(caps.add(3), 0xFF, 0x16);
        }
    }
    0
}

pub(crate) fn hle_xget_launch_info(args: &[u32], guest_mem: *mut u8) -> u32 {
    let type_out = args.first().copied().unwrap_or(0);
    let data_out = args.get(1).copied().unwrap_or(0);

    let read_u32 = |addr: u32| -> Option<u32> {
        let off = guest_ram_offset(addr)?;
        Some(unsafe { *((guest_mem as u64 + off as u64) as *const u32) })
    };
    let write_u32 = |addr: u32, value: u32| {
        if let Some(off) = guest_ram_offset(addr) {
            unsafe {
                *((guest_mem as u64 + off as u64) as *mut u32) = value;
            }
        }
    };

    write_u32(type_out, 0);
    write_u32(data_out, 0xFFFF_FFFF);

    let pp_launch = read_u32(0x000B_5228).unwrap_or(0);
    let launch_page = read_u32(pp_launch).unwrap_or(0);
    let Some(_launch_page_off) = guest_ram_offset(launch_page) else {
        debug_log(&format!(
            "[OOVPA-HLE] XGetLaunchInfo no launch page pp=0x{:08X} page=0x{:08X} -> 0x490",
            pp_launch, launch_page
        ));
        return 0x0000_0490;
    };

    let launch_type = read_u32(launch_page).unwrap_or(0);
    if launch_type != 2 && launch_type != 3 {
        let title_info = read_u32(0x0001_0118).unwrap_or(0);
        let expected_title = read_u32(title_info.wrapping_add(8)).unwrap_or(0);
        let page_title = read_u32(launch_page.wrapping_add(4)).unwrap_or(0);
        if expected_title == 0 || page_title != expected_title {
            debug_log(&format!(
                "[OOVPA-HLE] XGetLaunchInfo rejected type={} page_title=0x{:08X} expected=0x{:08X} pp=0x{:08X}",
                launch_type, page_title, expected_title, pp_launch
            ));
            return 0x0000_0490;
        }
    }

    write_u32(type_out, launch_type);
    if let (Some(src), Some(dst)) = (
        guest_ram_offset(launch_page.wrapping_add(0x400)),
        guest_ram_offset(data_out),
    ) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                (guest_mem as u64 + src as u64) as *const u8,
                (guest_mem as u64 + dst as u64) as *mut u8,
                0x0C00,
            );
        }
    }
    write_u32(pp_launch, 0);

    static LAUNCH_LOG: AtomicU32 = AtomicU32::new(0);
    let n = LAUNCH_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        let first = read_u32(data_out).unwrap_or(0);
        debug_log(&format!(
            "[OOVPA-HLE] XGetLaunchInfo #{} type={} pp=0x{:08X} page=0x{:08X} data_out=0x{:08X} first=0x{:08X}",
            n, launch_type, pp_launch, launch_page, data_out, first
        ));
    }
    0
}

fn write_guest_nul_string(guest_mem: *mut u8, addr: u32) {
    if addr != 0 && addr < 0x2000_0000 {
        unsafe {
            *((guest_mem as u64 + addr as u64) as *mut u8) = 0;
        }
    }
}

fn write_guest_u32_if_valid(guest_mem: *mut u8, addr: u32, value: u32) {
    if addr != 0 && addr < 0x2000_0000 {
        unsafe {
            *((guest_mem as u64 + addr as u64) as *mut u32) = value;
        }
    }
}

// ============================================================================
// Specialized SetRenderState_* suffix → slot lookup (2026-04-21)
// ============================================================================
//
// Maps the name-suffix of D3DDevice_SetRenderState_<Suffix> hooks to the
// X_D3DRS_* slot number. Covers every variant the xbsymdb pattern table
// detects for Spider-Man XDK 4134 and Doom XDK 5849.
//
// Where a suffix has no corresponding slot in our render_state::XboxRenderState
// enum (e.g. Xbox-only states not yet ported), returns None; caller falls
// through to the main match and hits the default handler.
pub(super) fn specialized_rs_slot_from_suffix(suffix: &str) -> Option<u32> {
    use crate::xbox::gpu::render_state::XboxRenderState as X;
    Some(match suffix {
        // Depth / stencil
        "ZEnable" => X::ZEnable as u32,
        "ZFunc" => X::ZFunc as u32,
        "ZWriteEnable" => X::ZWriteEnable as u32,
        "ZBias" => X::ZBias as u32,
        "StencilEnable" => X::StencilEnable as u32,
        "StencilFail" => X::StencilFail as u32,
        "StencilZFail" => X::StencilZFail as u32,
        "StencilPass" => X::StencilPass as u32,
        "StencilFunc" => X::StencilFunc as u32,
        "StencilRef" => X::StencilRef as u32,
        "StencilMask" => X::StencilMask as u32,
        "StencilWriteMask" => X::StencilWriteMask as u32,

        // Blend / alpha
        "AlphaBlendEnable" => X::AlphaBlendEnable as u32,
        "AlphaTestEnable" => X::AlphaTestEnable as u32,
        "AlphaRef" => X::AlphaRef as u32,
        "AlphaFunc" => X::AlphaFunc as u32,
        "SrcBlend" => X::SrcBlend as u32,
        "DestBlend" => X::DestBlend as u32,
        "BlendOp" => X::BlendOp as u32,
        "BlendColor" => X::BlendColor as u32,
        "ColorWriteEnable" => X::ColorWriteEnable as u32,

        // Rasterizer
        "CullMode" => X::CullMode as u32,
        "FillMode" => X::FillMode as u32,
        "ShadeMode" => X::ShadeMode as u32,
        "DitherEnable" => X::DitherEnable as u32,
        "LineWidth" => X::LineWidth as u32,
        "EdgeAntiAlias" => X::EdgeAntiAlias as u32,
        "MultiSampleAntiAlias" => X::MultiSampleAntiAlias as u32,
        "MultiSampleMask" => X::MultiSampleMask as u32,
        "TextureFactor" => X::TextureFactor as u32,

        // Fog
        "FogEnable" => X::FogEnable as u32,
        "FogColor" => X::FogColor as u32,
        "FogStart" => X::FogStart as u32,
        "FogEnd" => X::FogEnd as u32,
        "FogDensity" => X::FogDensity as u32,
        "FogTableMode" => X::FogTableMode as u32,
        "RangeFogEnable" => X::RangeFogEnable as u32,

        // Lighting
        "Lighting" => X::Lighting as u32,
        "SpecularEnable" => X::SpecularEnable as u32,
        "LocalViewer" => X::LocalViewer as u32,
        "ColorVertex" => X::ColorVertex as u32,
        "SpecularMaterialSource" => X::SpecularMaterialSource as u32,
        "DiffuseMaterialSource" => X::DiffuseMaterialSource as u32,
        "AmbientMaterialSource" => X::AmbientMaterialSource as u32,
        "EmissiveMaterialSource" => X::EmissiveMaterialSource as u32,
        "Ambient" => X::Ambient as u32,
        "NormalizeNormals" => X::NormalizeNormals as u32,

        // Point sprite
        "PointSize" => X::PointSize as u32,
        "PointSizeMin" => X::PointSizeMin as u32,
        "PointSpriteEnable" => X::PointSpriteEnable as u32,
        "PointScaleEnable" => X::PointScaleEnable as u32,
        "PointScaleA" => X::PointScaleA as u32,
        "PointScaleB" => X::PointScaleB as u32,
        "PointScaleC" => X::PointScaleC as u32,
        "PointSizeMax" => X::PointSizeMax as u32,

        // Texture wrap
        "Wrap0" => X::Wrap0 as u32,
        "Wrap1" => X::Wrap1 as u32,
        "Wrap2" => X::Wrap2 as u32,
        "Wrap3" => X::Wrap3 as u32,

        // Xbox pixel-shader combiner outlier. This lives outside the
        // contiguous 0..56 D3DPIXELSHADERDEF render-state block.
        "PSTextureModes" => 136,

        // Vertex pipeline
        "VertexBlend" => X::VertexBlend as u32,
        "PatchSegments" => X::PatchSegments as u32,

        // Xbox extensions not in our enum (skip — caller returns None and
        // falls through to default):
        //   Simple, FrontFace, BackFillMode, LogicOp, TwoSidedLighting,
        //   ShadowFunc, MultiSampleMode, MultiSampleRenderTargetMode,
        //   SampleAlpha, YuvEnable, OcclusionCullEnable, StencilCullEnable,
        //   RopZCmpAlwaysRead, RopZRead, DoNotCullUncompressed,
        //   StippleEnable, DepthClipControl, PolygonOffsetZSlopeScale,
        //   PolygonOffsetZOffset, PointOffsetEnable, WireframeOffsetEnable,
        //   SolidOffsetEnable, SwathWidth, PresentationInterval,
        //   SwapFilter. These are Xbox-only and don't
        //   have direct D3D11 equivalents; they'll need their own HLE
        //   handlers in future work.
        _ => return None,
    })
}

// ============================================================================
// D3D VBlank/Swap callback system
// ============================================================================

/// Guest function pointer registered via D3DDevice_SetVerticalBlankCallback
pub static VBLANK_CALLBACK: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Guest function pointer registered via D3DDevice_SetSwapCallback
pub static SWAP_CALLBACK: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Continuously incrementing VBlank counter (60Hz)
pub static VBLANK_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Swap counter (incremented each Swap call)
pub static SWAP_COUNT_GLOBAL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

const DEV_SWAP_CALLBACK_4134: u32 = 0x2428;
const DEV_VBLANK_CALLBACK_4134: u32 = 0x242C;
const DEV_SWAP_CALLBACK_4928: u32 = 0x254C;
const DEV_VBLANK_CALLBACK_4928: u32 = 0x2550;

pub(crate) fn d3d_device_callback_offsets() -> (u32, u32) {
    let xdk_build = XDK_BUILD.load(Ordering::Relaxed);
    if xdk_build >= 4928 {
        (DEV_SWAP_CALLBACK_4928, DEV_VBLANK_CALLBACK_4928)
    } else {
        (DEV_SWAP_CALLBACK_4134, DEV_VBLANK_CALLBACK_4134)
    }
}

// ============================================================================
// Guest address validation
// ============================================================================

/// Check if a guest address is a valid non-NULL pointer.
/// Accepts both low RAM (0x00000001-0x1FFFFFFF) and physical mirror (0x80000000-0x9FFFFFFF)
/// and pool/contiguous regions (0x80000000-0x83FFFFFF, 0x04000000+).
/// Rejects NULL, kernel thunks (0xFFFF0000+), and NV2A MMIO (0xFD000000+).
#[inline]
fn valid_guest_ptr(addr: u32) -> bool {
    (addr != 0) && (addr < 0x2000_0000 || (addr >= 0x8000_0000 && addr < 0xA000_0000))
}

/// Normalize a guest RAM pointer to the primary 512MB mapping.
/// Xbox titles frequently pass uncached/physical mirror pointers
/// (0x80000000..0x9fffffff) to D3D resources; host-side readers must strip
/// that mirror bit before indexing into the mapped guest memory.
#[inline]
fn normalize_guest_ram_ptr(addr: u32) -> Option<u32> {
    if addr == 0 {
        None
    } else if addr < 0x2000_0000 {
        Some(addr)
    } else if (0x8000_0000..0xA000_0000).contains(&addr) {
        Some(addr & 0x1FFF_FFFF)
    } else {
        None
    }
}

#[inline]
fn guest_ram_offset(addr: u32) -> Option<usize> {
    normalize_guest_ram_ptr(addr)
        .filter(|addr| *addr < 0x2000_0000)
        .map(|addr| addr as usize)
}

fn spidey_env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn xgrph_lle_canary_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY"))
}

fn xgrph_lle_source_repair_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_SOURCE_REPAIR"))
}

fn xgrph_lle_native_handoff_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_NATIVE_HANDOFF"))
}

fn xgrph_lle_native_skin_supplement_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_NATIVE_SKIN_SUPPLEMENT"))
}

fn xgrph_json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn repair_xgrph_stripped_operands_for_canary(source: &str) -> (String, u32) {
    let mut changed = 0u32;
    let mut out = String::with_capacity(source.len() + 128);
    let a0_mode = std::env::var("RUSTEMU_SPIDEY_XGRPH_A0_REPAIR")
        .unwrap_or_else(|_| "mov_a0".to_string())
        .to_ascii_lowercase();

    for line in source.lines() {
        let trimmed = line.trim_end();
        let lower = trimmed.to_ascii_lowercase();
        let op = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let repaired = if lower.starts_with("mov a0.x,") {
            if a0_mode == "keep" {
                None
            } else if let Some((_, rhs)) = trimmed.split_once(',') {
                let rhs = rhs.trim();
                let rhs_vec = rhs.strip_suffix(".x").unwrap_or(rhs);
                let line = match a0_mode.as_str() {
                    "mov_a0_srcvec" => format!("mov a0, {}", rhs_vec),
                    "mov_a0x_srcvec" => format!("mov a0.x, {}", rhs_vec),
                    "arl_a0x" => format!("arl a0.x, {}", rhs),
                    "arl_a0x_srcvec" => format!("arl a0.x, {}", rhs_vec),
                    "arl_a0" => format!("arl a0, {}", rhs),
                    "arl_a0_srcvec" => format!("arl a0, {}", rhs_vec),
                    _ => format!("mov a0, {}", rhs),
                };
                changed += 1;
                Some(line)
            } else {
                None
            }
        } else {
            let mut line = trimmed.to_string();
            let mut line_changed = false;
            if line.contains(", ,") && matches!(op.as_str(), "sub" | "add" | "mad" | "max" | "min")
            {
                line = line.replace(", ,", ", c[0],");
                line_changed = true;
            }
            if (line.ends_with(", ") || line.ends_with(','))
                && matches!(op.as_str(), "add" | "mul" | "mad" | "mov" | "max" | "min")
            {
                line.push_str(" c[0]");
                line_changed = true;
            }
            if line_changed {
                changed += 1;
                Some(line)
            } else {
                None
            }
        };

        if let Some(repaired) = repaired {
            out.push_str(&repaired);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }

    (out, changed)
}

fn read_xgrph_shader_source(guest_mem: *mut u8, addr: u32, len: u32, max_len: usize) -> String {
    if addr == 0 || addr >= 0x2000_0000 {
        return String::new();
    }
    let max = (len as usize).min(max_len);
    let mut bytes = Vec::with_capacity(max);
    for i in 0..max {
        let b = unsafe { *guest_mem.add(addr as usize + i) };
        let mapped = match b {
            b'\r' => b'\n',
            b'\n' | b'\t' => b,
            0 => break,
            0x20..=0x7E => b,
            _ => b'?',
        };
        bytes.push(mapped);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

static XGRPH_CANARY_LAST_PP_CODE: AtomicU32 = AtomicU32::new(0);
static XGRPH_CANARY_LAST_ENTRY_INDEX: AtomicU32 = AtomicU32::new(0);

pub(super) fn xgrph_lle_canary_tap_entry(
    args: &[u32; 8],
    guest_mem: *mut u8,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) {
    if !xgrph_lle_canary_enabled() {
        return;
    }

    static CANARY_ENTRY_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_ENTRY_COUNT.fetch_add(1, Ordering::Relaxed);
    let source_name = args[0];
    let source = args[1];
    let source_len = args[2];
    let flags = args[3];
    let pp_code = args[5];
    XGRPH_CANARY_LAST_PP_CODE.store(pp_code, Ordering::Relaxed);
    XGRPH_CANARY_LAST_ENTRY_INDEX.store(n, Ordering::Relaxed);

    let mut zeroed_pp_code = false;
    if let Some(pp_code_norm) = guest_ram_offset(pp_code) {
        unsafe {
            std::ptr::write_unaligned(guest_mem.add(pp_code_norm) as *mut u32, 0);
        }
        zeroed_pp_code = true;
    }

    let source_full = read_xgrph_shader_source(guest_mem, source, source_len, 8192);
    let mut redirected_source = 0u32;
    let mut redirected_len = 0u32;
    let mut repaired_operands = 0u32;
    let source_for_hle = if xgrph_lle_source_repair_enabled() {
        let (mut repaired, changed) = repair_xgrph_stripped_operands_for_canary(&source_full);
        repaired_operands = changed;
        if changed != 0 {
            repaired.push('\0');
            let buf = alloc_hle_heap_block(guest_mem, repaired.len() as u32);
            if buf != 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        repaired.as_ptr(),
                        guest_mem.add(buf as usize),
                        repaired.len(),
                    );
                    // Stack layout: [esp]=ret, [esp+4]=source_name, [esp+8]=source, [esp+12]=source_len.
                    std::ptr::write_unaligned(guest_mem.add(r14 as usize + 8) as *mut u32, buf);
                    std::ptr::write_unaligned(
                        guest_mem.add(r14 as usize + 12) as *mut u32,
                        repaired.len().saturating_sub(1) as u32,
                    );
                }
                redirected_source = buf;
                redirected_len = repaired.len().saturating_sub(1) as u32;
                repaired.trim_end_matches('\0').to_string()
            } else {
                source_full.clone()
            }
        } else {
            source_full.clone()
        }
    } else {
        source_full.clone()
    };
    let source_snip = source_for_hle.chars().take(512).collect::<String>();
    record_pending_shader_source(source_snip.clone());
    let captured = crate::xbox::gpu::nv2a_vsh::capture_from_xgrph_source(&source_for_hle);
    crate::xbox::gpu::nv2a_vsh::register_pending_xgrph_shader(captured.clone());

    let mut hle_bytes = Vec::with_capacity(captured.tokens.len() * 4);
    for &word in &captured.tokens {
        hle_bytes.extend_from_slice(&word.to_le_bytes());
    }
    let hle_hash = fnv1a64(&hle_bytes);
    let hle_first = crate::xbox::gpu::nv2a_vsh::first_token_words(&captured.tokens, 24);
    let stem = format!("./xgrph_lle_canary_{:03}", n);
    let _ = std::fs::write(format!("{}_input.xvs", stem), &source_full);
    if redirected_source != 0 {
        let _ = std::fs::write(format!("{}_repaired_input.xvs", stem), &source_for_hle);
    }
    let _ = std::fs::write(format!("{}_hle_tokens.bin", stem), &hle_bytes);
    let json = format!(
        "{{\n  \"event\": \"xgrph_lle_canary_entry\",\n  \"index\": {},\n  \"source_name\": \"0x{:08X}\",\n  \"source\": \"0x{:08X}\",\n  \"source_len\": {},\n  \"redirected_source\": \"0x{:08X}\",\n  \"redirected_len\": {},\n  \"repaired_operands\": {},\n  \"flags\": \"0x{:08X}\",\n  \"pp_code\": \"0x{:08X}\",\n  \"pp_code_zeroed\": {},\n  \"ret_addr\": \"0x{:08X}\",\n  \"esp\": \"0x{:08X}\",\n  \"entry_regs\": {{ \"eax\": \"0x{:08X}\", \"ebx\": \"0x{:08X}\", \"ecx\": \"0x{:08X}\", \"edx\": \"0x{:08X}\", \"esi\": \"0x{:08X}\", \"edi\": \"0x{:08X}\" }},\n  \"hle_token_words\": {},\n  \"hle_hash\": \"0x{:016X}\",\n  \"hle_first_words\": \"{}\",\n  \"hle_decoded_head\": \"{}\",\n  \"source_snippet\": \"{}\"\n}}\n",
        n,
        source_name,
        source,
        source_len,
        redirected_source,
        redirected_len,
        repaired_operands,
        flags,
        pp_code,
        if zeroed_pp_code { "true" } else { "false" },
        ret_addr,
        r14,
        regs[0],
        regs[1],
        regs[2],
        regs[3],
        regs[4],
        regs[5],
        captured.tokens.len(),
        hle_hash,
        xgrph_json_escape(&hle_first),
        xgrph_json_escape(
            &captured
                .decoded_lines
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>()
                .join(";")
        ),
        xgrph_json_escape(
            &source_snip
                .split_whitespace()
                .take(64)
                .collect::<Vec<_>>()
                .join(" ")
        )
    );
    let _ = std::fs::write(format!("{}_entry.json", stem), json);
    if n < 24 || n.is_power_of_two() {
        debug_log(&format!(
            "[XGRPH-LLE-CANARY-ENTRY] #{} name=0x{:08X} src=0x{:08X} len={} flags=0x{:X} ppCode=0x{:08X} zeroed={} ret=0x{:08X} esp=0x{:08X} eax=0x{:08X} edx=0x{:08X} hle_words={} hle_hash=0x{:016X} first=[{}] snip='{}'",
            n,
            source_name,
            source,
            source_len,
            flags,
            pp_code,
            zeroed_pp_code,
            ret_addr,
            r14,
            regs[0],
            regs[3],
            captured.tokens.len(),
            hle_hash,
            hle_first,
            source_snip.split_whitespace().take(48).collect::<Vec<_>>().join(" ")
        ));
        if redirected_source != 0 {
            debug_log(&format!(
                "[XGRPH-LLE-SOURCE-REPAIR] #{} original=0x{:08X}/{} repaired=0x{:08X}/{} operands={}",
                n, source, source_len, redirected_source, redirected_len, repaired_operands
            ));
        }
    }
}

pub(super) fn xgrph_lle_canary_return_tap(
    guest_mem: *mut u8,
    guest_addr: u32,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) {
    if !xgrph_lle_canary_enabled() {
        return;
    }

    static CANARY_RET_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_RET_COUNT.fetch_add(1, Ordering::Relaxed);
    let pp_code = XGRPH_CANARY_LAST_PP_CODE.load(Ordering::Relaxed);
    let entry_index = XGRPH_CANARY_LAST_ENTRY_INDEX.load(Ordering::Relaxed);
    let pp_code_value = read_u32_guest(guest_mem, pp_code).unwrap_or(0);
    let mut pointed_words = Vec::new();
    let mut native_data = 0u32;
    let mut native_size = 0u32;
    let mut native_hash = 0u64;
    let mut native_first_words = Vec::new();
    if pp_code_value != 0 {
        for i in 0..16u32 {
            if let Some(word) = read_u32_guest(guest_mem, pp_code_value.wrapping_add(i * 4)) {
                pointed_words.push(format!("0x{:08X}", word));
            } else {
                break;
            }
        }
        native_data = read_u32_guest(guest_mem, pp_code_value.wrapping_add(4)).unwrap_or(0);
        native_size = read_u32_guest(guest_mem, pp_code_value.wrapping_add(8)).unwrap_or(0);
        if let Some(data_off) = guest_ram_offset(native_data) {
            let dump_len = (native_size as usize).min(64 * 1024);
            let mut bytes = Vec::with_capacity(dump_len);
            for i in 0..dump_len {
                bytes.push(unsafe { *guest_mem.add(data_off + i) });
            }
            native_hash = fnv1a64(&bytes);
            for chunk in bytes.chunks_exact(4).take(32) {
                native_first_words.push(format!(
                    "0x{:08X}",
                    u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                ));
            }
            let _ = std::fs::write(
                format!(
                    "./xgrph_lle_canary_ret_{:03}_native.bin",
                    n
                ),
                &bytes,
            );
        }
    }
    let pointed_words = pointed_words.join(",");
    let native_first_words = native_first_words.join(",");
    let json = format!(
        "{{\n  \"event\": \"xgrph_lle_canary_return\",\n  \"index\": {},\n  \"entry_index\": {},\n  \"guest_addr\": \"0x{:08X}\",\n  \"stack_ret_addr\": \"0x{:08X}\",\n  \"esp\": \"0x{:08X}\",\n  \"eax\": \"0x{:08X}\",\n  \"ebx\": \"0x{:08X}\",\n  \"ecx\": \"0x{:08X}\",\n  \"edx\": \"0x{:08X}\",\n  \"esi\": \"0x{:08X}\",\n  \"edi\": \"0x{:08X}\",\n  \"pp_code\": \"0x{:08X}\",\n  \"pp_code_value\": \"0x{:08X}\",\n  \"pp_code_value_zero\": {},\n  \"native_data\": \"0x{:08X}\",\n  \"native_size\": {},\n  \"native_hash\": \"0x{:016X}\",\n  \"native_first_words\": \"{}\",\n  \"pointed_words\": \"{}\"\n}}\n",
        n,
        entry_index,
        guest_addr,
        ret_addr,
        r14,
        regs[0],
        regs[1],
        regs[2],
        regs[3],
        regs[4],
        regs[5],
        pp_code,
        pp_code_value,
        if pp_code_value == 0 { "true" } else { "false" },
        native_data,
        native_size,
        native_hash,
        xgrph_json_escape(&native_first_words),
        xgrph_json_escape(&pointed_words)
    );
    let _ = std::fs::write(
        format!("./xgrph_lle_canary_ret_{:03}.json", n),
        json,
    );
    if n < 24 || n.is_power_of_two() {
        debug_log(&format!(
            "[XGRPH-LLE-CANARY-RET] #{} entry={} guest=0x{:08X} stack_ret=0x{:08X} esp=0x{:08X} eax=0x{:08X} edx=0x{:08X} ppCode=0x{:08X} *ppCode=0x{:08X} native=0x{:08X}/{} hash=0x{:016X} first=[{}] pointed=[{}]",
            n,
            entry_index,
            guest_addr,
            ret_addr,
            r14,
            regs[0],
            regs[3],
            pp_code,
            pp_code_value,
            native_data,
            native_size,
            native_hash,
            native_first_words,
            pointed_words
        ));
    }
}

pub(super) fn xgrph_lle_canary_lexer_tap(
    guest_mem: *mut u8,
    guest_addr: u32,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) {
    if !xgrph_lle_canary_enabled() {
        return;
    }

    static CANARY_LEX_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_LEX_COUNT.fetch_add(1, Ordering::Relaxed);
    let ctx = regs[2];
    let version_flag = read_u32_guest(guest_mem, ctx.wrapping_add(0x98)).unwrap_or(0);
    let field_9c = read_u32_guest(guest_mem, ctx.wrapping_add(0x9c)).unwrap_or(0);
    let field_a0 = read_u32_guest(guest_mem, ctx.wrapping_add(0xa0)).unwrap_or(0);
    let cur = read_u32_guest(guest_mem, ctx.wrapping_add(0xa4)).unwrap_or(0);
    let end = read_u32_guest(guest_mem, ctx.wrapping_add(0xa8)).unwrap_or(0);
    let cur_byte = read_u32_guest(guest_mem, cur)
        .map(|w| w & 0xff)
        .unwrap_or(0xffff_ffff);
    let mut bytes = Vec::new();
    if cur != 0 {
        for i in 0..96u32 {
            let Some(b) = read_u32_guest(guest_mem, cur.wrapping_add(i)).map(|w| (w & 0xff) as u8)
            else {
                break;
            };
            if b == 0 {
                break;
            }
            bytes.push(match b {
                b'\r' => b'\n',
                b'\n' | b'\t' => b,
                0x20..=0x7e => b,
                _ => b'?',
            });
        }
    }
    let snippet = String::from_utf8_lossy(&bytes).into_owned();
    let json = format!(
        "{{\n  \"event\": \"xgrph_lle_canary_lexer\",\n  \"index\": {},\n  \"guest_addr\": \"0x{:08X}\",\n  \"ret_addr\": \"0x{:08X}\",\n  \"esp\": \"0x{:08X}\",\n  \"eax\": \"0x{:08X}\",\n  \"ebx\": \"0x{:08X}\",\n  \"ecx_ctx\": \"0x{:08X}\",\n  \"edx\": \"0x{:08X}\",\n  \"esi\": \"0x{:08X}\",\n  \"edi\": \"0x{:08X}\",\n  \"ctx_version_flag_98\": \"0x{:08X}\",\n  \"ctx_9c\": \"0x{:08X}\",\n  \"ctx_a0\": \"0x{:08X}\",\n  \"ctx_cur_a4\": \"0x{:08X}\",\n  \"ctx_end_a8\": \"0x{:08X}\",\n  \"cur_byte\": \"0x{:02X}\",\n  \"snippet\": \"{}\"\n}}\n",
        n,
        guest_addr,
        ret_addr,
        r14,
        regs[0],
        regs[1],
        regs[2],
        regs[3],
        regs[4],
        regs[5],
        version_flag,
        field_9c,
        field_a0,
        cur,
        end,
        cur_byte,
        xgrph_json_escape(&snippet.split_whitespace().take(64).collect::<Vec<_>>().join(" "))
    );
    let _ = std::fs::write(
        format!("./xgrph_lle_canary_lex_{:03}.json", n),
        json,
    );
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[XGRPH-LLE-CANARY-LEX] #{} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} ecx_ctx=0x{:08X} v98=0x{:08X} 9c=0x{:08X} a0=0x{:08X} cur=0x{:08X} end=0x{:08X} cur_byte=0x{:02X} snip='{}'",
            n,
            guest_addr,
            ret_addr,
            r14,
            regs[2],
            version_flag,
            field_9c,
            field_a0,
            cur,
            end,
            cur_byte,
            snippet.split_whitespace().take(32).collect::<Vec<_>>().join(" ")
        ));
    }
}

pub(super) fn xgrph_lle_canary_error_tap(
    guest_mem: *mut u8,
    guest_addr: u32,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) {
    if !xgrph_lle_canary_enabled() {
        return;
    }

    static CANARY_ERROR_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
    let stack0 = read_u32_guest(guest_mem, r14).unwrap_or(0);
    let stack1 = read_u32_guest(guest_mem, r14.wrapping_add(4)).unwrap_or(0);
    let stack2 = read_u32_guest(guest_mem, r14.wrapping_add(8)).unwrap_or(0);
    let stack3 = read_u32_guest(guest_mem, r14.wrapping_add(12)).unwrap_or(0);
    let stack4 = read_u32_guest(guest_mem, r14.wrapping_add(16)).unwrap_or(0);
    let stack5 = read_u32_guest(guest_mem, r14.wrapping_add(20)).unwrap_or(0);

    let string_candidates = [
        ("eax", regs[0]),
        ("ecx", regs[2]),
        ("edx", regs[3]),
        ("esi", regs[4]),
        ("edi", regs[5]),
        ("stack1", stack1),
        ("stack2", stack2),
        ("stack3", stack3),
        ("stack4", stack4),
        ("stack5", stack5),
    ];
    let mut strings = Vec::new();
    for (name, ptr) in string_candidates {
        let s = read_xgrph_shader_source(guest_mem, ptr, 160, 160);
        if !s.is_empty() {
            strings.push(format!(
                "{}@0x{:08X}='{}'",
                name,
                ptr,
                xgrph_json_escape(&s)
            ));
        }
    }

    let ctx_candidates = [("ecx", regs[2]), ("stack1", stack1), ("eax", regs[0])];
    let mut ctx_dump = Vec::new();
    let mut cur_snippets = Vec::new();
    for (name, ctx) in ctx_candidates {
        let version_flag = read_u32_guest(guest_mem, ctx.wrapping_add(0x98)).unwrap_or(0);
        let field_9c = read_u32_guest(guest_mem, ctx.wrapping_add(0x9c)).unwrap_or(0);
        let field_a0 = read_u32_guest(guest_mem, ctx.wrapping_add(0xa0)).unwrap_or(0);
        let cur = read_u32_guest(guest_mem, ctx.wrapping_add(0xa4)).unwrap_or(0);
        let end = read_u32_guest(guest_mem, ctx.wrapping_add(0xa8)).unwrap_or(0);
        let line_a = read_u32_guest(guest_mem, ctx.wrapping_add(0x1b4)).unwrap_or(0);
        let line_b = read_u32_guest(guest_mem, ctx.wrapping_add(0x1b8)).unwrap_or(0);
        let plausible = field_9c != 0
            && cur != 0
            && end != 0
            && field_9c <= cur
            && cur <= end
            && end.wrapping_sub(field_9c) < 0x20_000;
        if plausible || version_flag != 0 {
            let mut line = 1u32;
            let mut col = 1u32;
            if plausible {
                let span = cur.saturating_sub(field_9c).min(8192);
                for i in 0..span {
                    let b = read_u32_guest(guest_mem, field_9c.wrapping_add(i))
                        .map(|w| (w & 0xff) as u8)
                        .unwrap_or(0);
                    if b == b'\n' {
                        line = line.saturating_add(1);
                        col = 1;
                    } else {
                        col = col.saturating_add(1);
                    }
                }
            }
            ctx_dump.push(format!(
                "{}@0x{:08X}:v98=0x{:08X},9c=0x{:08X},a0=0x{:08X},cur=0x{:08X},end=0x{:08X},line={} col={} f1b4=0x{:08X} f1b8=0x{:08X}",
                name, ctx, version_flag, field_9c, field_a0, cur, end, line, col, line_a, line_b
            ));
            if cur != 0 {
                let snip = read_xgrph_shader_source(guest_mem, cur, 160, 160);
                if !snip.is_empty() {
                    cur_snippets.push(format!(
                        "{}@0x{:08X}='{}'",
                        name,
                        cur,
                        xgrph_json_escape(
                            &snip
                                .split_whitespace()
                                .take(32)
                                .collect::<Vec<_>>()
                                .join(" ")
                        )
                    ));
                }
            }
        }
    }

    let json = format!(
        "{{\n  \"event\": \"xgrph_lle_canary_error\",\n  \"index\": {},\n  \"guest_addr\": \"0x{:08X}\",\n  \"ret_addr\": \"0x{:08X}\",\n  \"esp\": \"0x{:08X}\",\n  \"eax\": \"0x{:08X}\",\n  \"ebx\": \"0x{:08X}\",\n  \"ecx\": \"0x{:08X}\",\n  \"edx\": \"0x{:08X}\",\n  \"esi\": \"0x{:08X}\",\n  \"edi\": \"0x{:08X}\",\n  \"stack\": [\"0x{:08X}\",\"0x{:08X}\",\"0x{:08X}\",\"0x{:08X}\",\"0x{:08X}\",\"0x{:08X}\"],\n  \"strings\": \"{}\",\n  \"contexts\": \"{}\",\n  \"cur_snippets\": \"{}\"\n}}\n",
        n,
        guest_addr,
        ret_addr,
        r14,
        regs[0],
        regs[1],
        regs[2],
        regs[3],
        regs[4],
        regs[5],
        stack0,
        stack1,
        stack2,
        stack3,
        stack4,
        stack5,
        xgrph_json_escape(&strings.join(" | ")),
        xgrph_json_escape(&ctx_dump.join(" | ")),
        xgrph_json_escape(&cur_snippets.join(" | "))
    );
    let _ = std::fs::write(
        format!("./xgrph_lle_canary_err_{:03}.json", n),
        json,
    );
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[XGRPH-LLE-CANARY-ERR] #{} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] strings=[{}] ctx=[{}] cur=[{}]",
            n,
            guest_addr,
            ret_addr,
            r14,
            regs[0],
            regs[2],
            regs[3],
            stack0,
            stack1,
            stack2,
            stack3,
            stack4,
            stack5,
            strings.join(" | "),
            ctx_dump.join(" | "),
            cur_snippets.join(" | ")
        ));
    }
}

fn read_native_xgrph_shader_tokens(guest_mem: *mut u8, p_func: u32) -> Option<Vec<u32>> {
    let base = guest_ram_offset(p_func)?;
    let header = unsafe { std::ptr::read_unaligned(guest_mem.add(base) as *const u32) };
    let count = if (header & 0xFFFF) == 0x2078 {
        ((header >> 16) as usize).clamp(1, 136)
    } else {
        136
    };
    let word_count = 1usize.saturating_add(count.saturating_mul(4));
    let mut tokens = Vec::with_capacity(word_count);
    for i in 0..word_count {
        let addr = base.checked_add(i * 4)?;
        if addr > 0x2000_0000usize.saturating_sub(4) {
            break;
        }
        let word = unsafe { std::ptr::read_unaligned(guest_mem.add(addr) as *const u32) };
        tokens.push(word);
    }
    if tokens.len() >= 4 {
        Some(tokens)
    } else {
        None
    }
}

fn xgrph_lle_canary_dump_create_vertex_shader(
    create_index: u32,
    fake_handle: u32,
    p_func: u32,
    guest_mem: *mut u8,
) {
    if !xgrph_lle_canary_enabled() {
        return;
    }

    static CANARY_CVS_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = CANARY_CVS_COUNT.fetch_add(1, Ordering::Relaxed);
    let mut header = [0u32; 8];
    let mut p_func_norm = 0u32;
    let mut data_raw = 0u32;
    let mut data_norm = 0u32;
    let mut declared_size = 0u32;
    let mut bytes: Vec<u8> = Vec::new();

    if let Some(p_func_off) = guest_ram_offset(p_func) {
        p_func_norm = p_func_off as u32;
        unsafe {
            let ptr = guest_mem.add(p_func_off) as *const u32;
            for (i, dst) in header.iter_mut().enumerate() {
                *dst = std::ptr::read_unaligned(ptr.add(i));
            }
        }
        data_raw = header[1];
        declared_size = header[2];
        if let Some(data_off) = guest_ram_offset(data_raw) {
            data_norm = data_off as u32;
            let len = declared_size.min(0x10000) as usize;
            if len != 0 {
                bytes.resize(len, 0);
                unsafe {
                    std::ptr::copy_nonoverlapping(guest_mem.add(data_off), bytes.as_mut_ptr(), len);
                }
            }
        }
    }

    let hash = fnv1a64(&bytes);
    let all_zero = !bytes.is_empty() && bytes.iter().all(|b| *b == 0);
    let first_words = bytes
        .chunks_exact(4)
        .take(32)
        .map(|chunk| {
            format!(
                "0x{:08X}",
                u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let stem = format!("./xgrph_lle_canary_cvs_{:03}", n);
    if !bytes.is_empty() {
        let _ = std::fs::write(format!("{}_native_tokens.bin", stem), &bytes);
    }
    let header_text = header
        .iter()
        .map(|w| format!("0x{:08X}", w))
        .collect::<Vec<_>>()
        .join(",");
    let json = format!(
        "{{\n  \"event\": \"xgrph_lle_canary_create_vertex_shader\",\n  \"index\": {},\n  \"create_index\": {},\n  \"fake_handle\": \"0x{:08X}\",\n  \"p_func\": \"0x{:08X}\",\n  \"p_func_norm\": \"0x{:08X}\",\n  \"header_words\": [{}],\n  \"data_raw\": \"0x{:08X}\",\n  \"data_norm\": \"0x{:08X}\",\n  \"declared_size\": {},\n  \"captured_size\": {},\n  \"native_hash\": \"0x{:016X}\",\n  \"native_all_zero\": {},\n  \"native_first_words\": \"{}\"\n}}\n",
        n,
        create_index,
        fake_handle,
        p_func,
        p_func_norm,
        header_text,
        data_raw,
        data_norm,
        declared_size,
        bytes.len(),
        hash,
        if all_zero { "true" } else { "false" },
        xgrph_json_escape(&first_words)
    );
    let _ = std::fs::write(format!("{}_native.json", stem), json);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[XGRPH-LLE-CANARY-CVS] #{} create={} handle=0x{:08X} pFunc=0x{:08X}->0x{:08X} header=[{}] data=0x{:08X}->0x{:08X} size={} captured={} hash=0x{:016X} all_zero={} first=[{}]",
            n,
            create_index,
            fake_handle,
            p_func,
            p_func_norm,
            header_text,
            data_raw,
            data_norm,
            declared_size,
            bytes.len(),
            hash,
            all_zero,
            first_words
        ));
    }
}

fn menu_wait_trace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        spidey_env_flag("RUSTEMU_MENU_WAIT_TRACE")
            || spidey_env_flag("RUSTEMU_XAPI_WAIT_TRACE")
            || spidey_env_flag("RUSTEMU_TITLE_PROBES")
    })
}

fn menu_wait_trace_log(n: u32) -> bool {
    menu_wait_trace_enabled() && (n < 64 || n.is_power_of_two() || n % 1000 == 0)
}

fn spidey_env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            if v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
            {
                true
            } else if v == "0"
                || v.eq_ignore_ascii_case("false")
                || v.eq_ignore_ascii_case("no")
                || v.eq_ignore_ascii_case("off")
            {
                false
            } else {
                default
            }
        })
        .unwrap_or(default)
}

fn spidey_title_patches_enabled() -> bool {
    spidey_env_flag("RUSTEMU_TITLE_PATCHES")
        || spidey_env_flag("RUSTEMU_SPIDEY_TITLE_PATCHES")
        || spidey_env_flag("RUSTEMU_SPIDEY_PATCHES")
}

fn spidey_pbyte4_normalize_enabled() -> bool {
    !spidey_env_flag("RUSTEMU_SPIDEY_PBYTE4_RAW")
        && (spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_C_HLE")
            || spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_HLE_CXBXR")
            || spidey_env_bool("RUSTEMU_SPIDEY_PBYTE4_NORMALIZE", true))
}

fn spidey_hex_preview(guest_mem: *mut u8, addr: u32, len: usize) -> String {
    let Some(addr) = normalize_guest_ram_ptr(addr) else {
        return "-".to_string();
    };
    if addr
        .checked_add(len as u32)
        .map(|end| end > 0x2000_0000)
        .unwrap_or(true)
    {
        return "-".to_string();
    }

    let bytes = unsafe { std::slice::from_raw_parts(guest_mem.add(addr as usize), len) };
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn spidey_ascii_preview(guest_mem: *mut u8, addr: u32, len: usize) -> String {
    let Some(addr) = normalize_guest_ram_ptr(addr) else {
        return "-".to_string();
    };
    if addr
        .checked_add(len as u32)
        .map(|end| end > 0x2000_0000)
        .unwrap_or(true)
    {
        return "-".to_string();
    }

    let bytes = unsafe { std::slice::from_raw_parts(guest_mem.add(addr as usize), len) };
    let mut out = String::new();
    for &b in bytes {
        if b == 0 {
            out.push('.');
        } else if b.is_ascii_graphic() || b == b' ' {
            out.push(b as char);
        } else {
            out.push('.');
        }
    }
    out
}

fn spidey_find_needle_in_range(
    guest_mem: *mut u8,
    start: u32,
    end: u32,
    needle: &[u8],
    max_hits: usize,
) -> Vec<u32> {
    if needle.is_empty() || max_hits == 0 || start >= end {
        return Vec::new();
    }
    let start = start.min(0x2000_0000);
    let end = end.min(0x2000_0000);
    if start >= end || (end - start) as usize <= needle.len() {
        return Vec::new();
    }

    let bytes = unsafe {
        std::slice::from_raw_parts(guest_mem.add(start as usize), (end - start) as usize)
    };
    let mut hits = Vec::new();
    for (off, window) in bytes.windows(needle.len()).enumerate() {
        if window == needle {
            hits.push(start + off as u32);
            if hits.len() >= max_hits {
                break;
            }
        }
    }
    hits
}

fn spidey_peterstu_resource_trace(
    guest_mem: *mut u8,
    this_ptr: u32,
    p_base: u32,
    pixel_addr: u32,
    common: u32,
    format_dw: u32,
    data_off: u32,
    size: usize,
) {
    if !spidey_env_flag("RUSTEMU_SPIDEY_PETERSTU_RESOURCE_TRACE") {
        return;
    }
    let ready_seq = crate::xbox::emulator::spidey_peterstu_read_ready_seq();
    if ready_seq == 0 {
        return;
    }

    static TRACE_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if n < 256 || n.is_power_of_two() {
        let p_base_norm = normalize_guest_ram_ptr(p_base).unwrap_or(0);
        let this_norm = normalize_guest_ram_ptr(this_ptr).unwrap_or(0);
        let scan_start = p_base_norm.saturating_sub(0x1000);
        let scan_end = p_base_norm.saturating_add(0x8000).min(0x2000_0000);
        let mut hits = Vec::new();
        for (label, needle) in [
            ("XBXM", b"XBXM".as_slice()),
            ("peterstudent000", b"peterstudent000".as_slice()),
            ("peterstudent", b"peterstudent".as_slice()),
            ("BIP01", b"BIP01".as_slice()),
        ] {
            for hit in spidey_find_needle_in_range(guest_mem, scan_start, scan_end, needle, 8) {
                hits.push(format!("{}@0x{:08X}", label, hit));
            }
        }
        debug_log(&format!(
            "[SPIDEY-PETERSTU-RESOURCE] #{} seq={} this=0x{:08X}->0x{:08X} pBase=0x{:08X}->0x{:08X} pixel=0x{:08X} common=0x{:08X} data_off=0x{:08X} fmt=0x{:08X} size=0x{:X} pBase_hex=[{}] pBase_ascii='{}' hits=[{}]",
            n,
            ready_seq,
            this_ptr,
            this_norm,
            p_base,
            p_base_norm,
            pixel_addr,
            common,
            data_off,
            format_dw,
            size,
            spidey_hex_preview(guest_mem, p_base_norm, 32),
            spidey_ascii_preview(guest_mem, p_base_norm, 48),
            if hits.is_empty() { "-".to_string() } else { hits.join(" ") }
        ));
    }

    if spidey_env_flag("RUSTEMU_SPIDEY_PETERSTU_GUEST_SCAN") {
        static SCAN_DONE: AtomicU32 = AtomicU32::new(0);
        if SCAN_DONE
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            let ranges = [
                (0x0160_0000, 0x0288_0000, "contig-xpr"),
                (0x0400_0000, 0x0800_0000, "vm-bump"),
                (0x1000_0000, 0x1400_0000, "hle-heap"),
            ];
            for (start, end, label) in ranges {
                for (needle_label, needle) in [
                    ("XBXM", b"XBXM".as_slice()),
                    ("peterstudent000", b"peterstudent000".as_slice()),
                    ("BIP01", b"BIP01".as_slice()),
                    ("spiderman.ani", b"spiderman.ani".as_slice()),
                ] {
                    let hits = spidey_find_needle_in_range(guest_mem, start, end, needle, 16);
                    debug_log(&format!(
                        "[SPIDEY-PETERSTU-GUEST-SCAN] seq={} range={} 0x{:08X}..0x{:08X} needle={} hits=[{}]",
                        ready_seq,
                        label,
                        start,
                        end,
                        needle_label,
                        if hits.is_empty() {
                            "-".to_string()
                        } else {
                            hits.iter()
                                .map(|h| format!("0x{:08X}", h))
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                    ));
                }
            }
        }
    }
}

fn alloc_hle_heap_block(guest_mem: *mut u8, size: u32) -> u32 {
    let aligned = ((size.max(1) + 15) & !15).max(16);
    let _guard = HLE_HEAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let ptr = HLE_HEAP_BUMP.fetch_add(aligned, Ordering::Relaxed);
    if ptr
        .checked_add(aligned)
        .map(|end| end >= HLE_HEAP_POOL_END)
        .unwrap_or(true)
    {
        debug_log(&format!(
            "[HLE] alloc_hle_heap_block: OOM size={} aligned={} ptr=0x{:08X}",
            size, aligned, ptr
        ));
        return 0;
    }
    unsafe {
        std::ptr::write_bytes(guest_mem.add(ptr as usize), 0, aligned as usize);
    }
    let alloc_sizes = HLE_HEAP_ALLOC_SIZES.get_or_init(|| Mutex::new(Vec::new()));
    let mut sizes = alloc_sizes.lock().unwrap_or_else(|e| e.into_inner());
    sizes.push((ptr, aligned));
    ptr
}

#[derive(Clone, Copy, Debug, Default)]
struct HleTextureInfo {
    key: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
    swizzled: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct HleTextureDirtyRange {
    key: u32,
    data: u32,
    len: u32,
    serial: u32,
}

#[derive(Clone, Copy, Debug, Default)]
struct DoomBlitLockInfo {
    seq: u32,
    swap: u64,
    tex_raw: u32,
    tex_key: u32,
    level: u32,
    p_locked_rect: u32,
    p_rect: u32,
    flags: u32,
    rect_l: u32,
    rect_t: u32,
    rect_r: u32,
    rect_b: u32,
    width: u32,
    height: u32,
    format: u32,
    format_code: u32,
    data: u32,
    pitch: u32,
    source_state: u32,
    source_screen: u32,
    source_w: u32,
    source_h: u32,
    source_hash: u64,
    source_nonzero: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct DoomBlitUploadInfo {
    seq: u32,
    lock_seq: u32,
    stage: u32,
    tex_key: u32,
    data: u32,
    width: u32,
    height: u32,
    format_code: u32,
    pitch: u32,
    bpp: u32,
    swizzled: bool,
    visible_w: u32,
    visible_h: u32,
    row_stride: u32,
    full_hash: u64,
    full_nonzero: usize,
    hash: u64,
    nonzero: usize,
    disputed_rows: u32,
    disputed_hash: u64,
    disputed_nonzero: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct DoomBlitSwizzleInfo {
    seq: u32,
    src_arg: u32,
    src_base: u32,
    dst_arg: u32,
    dst_base: u32,
    pitch_arg: u32,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u32,
    src_x: u32,
    src_y: u32,
    rect_w: u32,
    rect_h: u32,
    dst_x: u32,
    dst_y: u32,
    doom_source_w: u32,
    doom_source_h: u32,
    doom_source_pitch_guess: u32,
    src_resolved_hash: u64,
    src_resolved_nonzero: usize,
    src_doom_pitch_hash: u64,
    src_doom_pitch_nonzero: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct DoomBlitDrawInfo {
    seq: u32,
    prim_type: u32,
    vertex_count: u32,
    stride: u32,
    vertex_data_ptr: u32,
    refreshed: u32,
    active_tex0: u32,
    ps_handle: u32,
    ps: PixelShaderStateSnapshot,
    stage0: [u32; 32],
    vertex_sample_count: u32,
    vertices: [NV2AVertex; 4],
}

fn texture_registry() -> &'static Mutex<Vec<HleTextureInfo>> {
    static REGISTRY: OnceLock<Mutex<Vec<HleTextureInfo>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

fn texture_dirty_ranges() -> &'static Mutex<Vec<HleTextureDirtyRange>> {
    static DIRTY: OnceLock<Mutex<Vec<HleTextureDirtyRange>>> = OnceLock::new();
    DIRTY.get_or_init(|| Mutex::new(Vec::new()))
}

fn doom_blit_lock_state() -> &'static Mutex<Option<DoomBlitLockInfo>> {
    static STATE: OnceLock<Mutex<Option<DoomBlitLockInfo>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn doom_blit_upload_state() -> &'static Mutex<Option<DoomBlitUploadInfo>> {
    static STATE: OnceLock<Mutex<Option<DoomBlitUploadInfo>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn doom_blit_swizzle_state() -> &'static Mutex<Option<DoomBlitSwizzleInfo>> {
    static STATE: OnceLock<Mutex<Option<DoomBlitSwizzleInfo>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn doom_blit_draw_state() -> &'static Mutex<Option<DoomBlitDrawInfo>> {
    static STATE: OnceLock<Mutex<Option<DoomBlitDrawInfo>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

fn doom_blit_diag_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        spidey_env_flag("RUSTEMU_DOOM_BLIT_DIAG")
            || spidey_env_flag("RUSTEMU_DOOM_5849_TOML")
            || std::env::var("RUSTEMU_OOVPA_SYMBOLS_TOML")
                .map(|v| v.to_ascii_lowercase().contains("doom"))
                .unwrap_or(false)
    })
}

fn doom_blit_dump_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        spidey_env_flag("RUSTEMU_DOOM_BLIT_DUMP") || spidey_env_flag("RUSTEMU_DOOM_STAGING_DUMP")
    })
}

fn doom_span_probe_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_DOOM_SPAN_PROBE"))
}

pub fn doom_wall_probe_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_DOOM_WALL_PROBE"))
}

fn doom_linear_upload_probe_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        spidey_env_flag("RUSTEMU_DOOM_LINEAR_UPLOAD")
            || spidey_env_flag("RUSTEMU_DOOM_LINEAR_PRESENT_UPLOAD")
    })
}

fn doom_swizzle_bisect_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| spidey_env_flag("RUSTEMU_DOOM_SWIZZLE_BISECT"))
}

fn doom_blit_dump_seq_selected(seq: u32) -> bool {
    matches!(seq, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 | 256 | 512)
}

fn doom_copy_guest_rows(
    guest_mem: *mut u8,
    base: u32,
    pitch: u32,
    row_bytes: u32,
    rows: u32,
) -> Option<Vec<u8>> {
    if base == 0 || pitch == 0 || row_bytes == 0 || rows == 0 || row_bytes > pitch {
        return None;
    }
    let end = base as u64
        + (rows.saturating_sub(1) as u64).saturating_mul(pitch as u64)
        + row_bytes as u64;
    if end > 0x2000_0000 {
        return None;
    }
    let total = (row_bytes as usize).saturating_mul(rows as usize);
    if total == 0 || total > 4 * 1024 * 1024 {
        return None;
    }

    let mut packed = Vec::with_capacity(total);
    unsafe {
        for y in 0..rows {
            let row = std::slice::from_raw_parts(
                guest_mem.add((base + y.saturating_mul(pitch)) as usize),
                row_bytes as usize,
            );
            packed.extend_from_slice(row);
        }
    }
    Some(packed)
}

fn doom_argb_hash(pixels: &[u32]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &px in pixels {
        for byte in px.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    hash
}

fn doom_crop_argb(
    pixels: &[u32],
    src_w: u32,
    src_h: u32,
    crop_w: u32,
    crop_h: u32,
) -> Option<Vec<u32>> {
    if crop_w == 0
        || crop_h == 0
        || crop_w > src_w
        || crop_h > src_h
        || pixels.len() < (src_w as usize).saturating_mul(src_h as usize)
    {
        return None;
    }
    let mut out = Vec::with_capacity((crop_w as usize).saturating_mul(crop_h as usize));
    for y in 0..crop_h as usize {
        let start = y.saturating_mul(src_w as usize);
        out.extend_from_slice(&pixels[start..start + crop_w as usize]);
    }
    Some(out)
}

fn doom_decode_guest_argb_rows(
    guest_mem: *mut u8,
    base: u32,
    pitch: u32,
    width: u32,
    height: u32,
    format_code: u32,
) -> Option<Vec<u32>> {
    let bpp = texture_bytes_per_pixel(format_code)?;
    let row_bytes = width.checked_mul(bpp)?;
    let bytes = doom_copy_guest_rows(guest_mem, base, pitch, row_bytes, height)?;
    crate::xbox::gpu::texture_format::decode_linear_to_argb(
        format_code,
        width,
        height,
        &bytes,
        None,
    )
}

fn doom_argb_nonzero(pixels: &[u32]) -> usize {
    pixels.iter().filter(|px| **px != 0).count()
}

fn doom_dump_staging_view(
    guest_mem: *mut u8,
    seq: u32,
    src_base: u32,
    label: &str,
    width: u32,
    height: u32,
    pitch: u32,
    format_code: u32,
) -> String {
    match doom_decode_guest_argb_rows(guest_mem, src_base, pitch, width, height, format_code) {
        Some(pixels) => {
            let hash = doom_argb_hash(&pixels);
            let nonzero = doom_argb_nonzero(&pixels);
            let path = format!(
                r"./doom_stage_src_{:03}_{}_src{:08X}_{}x{}_p{}.bmp",
                seq, label, src_base, width, height, pitch
            );
            write_readback_bmp(&path, width, height, &pixels);
            format!(
                "{}={}x{} pitch={} hash=0x{:016X} nonzero={} path={}",
                label, width, height, pitch, hash, nonzero, path
            )
        }
        None => format!("{}={}x{} pitch={} unavailable", label, width, height, pitch),
    }
}

fn doom_dump_l8_view(
    guest_mem: *mut u8,
    seq: u32,
    base: u32,
    label: &str,
    width: u32,
    height: u32,
    pitch: u32,
) -> String {
    match doom_copy_guest_rows(guest_mem, base, pitch, width, height) {
        Some(bytes) => {
            let mut pixels = Vec::with_capacity((width as usize).saturating_mul(height as usize));
            for &v in &bytes {
                let v = v as u32;
                pixels.push(0xFF00_0000 | (v << 16) | (v << 8) | v);
            }
            let hash = fnv1a64(&bytes);
            let nonzero = bytes.iter().filter(|b| **b != 0).count();
            let path = format!(
                r"./doom_screen_src_{:03}_{}_src{:08X}_{}x{}_p{}.bmp",
                seq, label, base, width, height, pitch
            );
            write_readback_bmp(&path, width, height, &pixels);
            format!(
                "{}={}x{} pitch={} hash=0x{:016X} nonzero={} path={}",
                label, width, height, pitch, hash, nonzero, path
            )
        }
        None => format!("{}={}x{} pitch={} unavailable", label, width, height, pitch),
    }
}

fn doom_u32_list_preview(values: &[u32], n: usize) -> String {
    values
        .iter()
        .take(n)
        .map(|v| format!("0x{:08X}", v))
        .collect::<Vec<_>>()
        .join(",")
}

fn doom_count_unique_u32(values: &[u32]) -> usize {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    sorted.len()
}

fn doom_first_mismatch<F>(values: &[u32], expected: F) -> Option<(usize, u32, u32)>
where
    F: Fn(usize) -> u32,
{
    values.iter().enumerate().find_map(|(idx, &actual)| {
        let exp = expected(idx);
        (actual != exp).then_some((idx, actual, exp))
    })
}

fn doom_rowlookup_diag_for_state(
    guest_mem: *mut u8,
    seq: u32,
    label: &str,
    state: u32,
    screen_ptr: u32,
    width: u32,
    height: u32,
) -> String {
    if state == 0 || width == 0 || height == 0 {
        return "rowlookup=unavailable".to_string();
    }

    let y_count = height.min(240) as usize;
    let col_count = width.min(320) as usize;
    if y_count == 0 || col_count == 0 {
        return "rowlookup=empty".to_string();
    }

    let y_base = state.saturating_add(0x94CC);
    let col_base = state.saturating_add(0xA1CC);
    let mut y_values = Vec::with_capacity(y_count);
    let mut col_values = Vec::with_capacity(col_count);
    for idx in 0..y_count {
        y_values.push(doom_read_u32(
            guest_mem,
            y_base.saturating_add((idx as u32).saturating_mul(4)),
        ));
    }
    for idx in 0..col_count {
        col_values.push(doom_read_u32(
            guest_mem,
            col_base.saturating_add((idx as u32).saturating_mul(4)),
        ));
    }

    let expected_step = width;
    let mut y_step_ok = 0usize;
    let mut y_step_zero = 0usize;
    let mut y_step_other = 0usize;
    for pair in y_values.windows(2) {
        let delta = pair[1].wrapping_sub(pair[0]);
        if delta == expected_step {
            y_step_ok += 1;
        } else if delta == 0 {
            y_step_zero += 1;
        } else {
            y_step_other += 1;
        }
    }

    let mut col_step_ok = 0usize;
    let mut col_step_zero = 0usize;
    let mut col_step_other = 0usize;
    for pair in col_values.windows(2) {
        let delta = pair[1].wrapping_sub(pair[0]);
        if delta == 1 {
            col_step_ok += 1;
        } else if delta == 0 {
            col_step_zero += 1;
        } else {
            col_step_other += 1;
        }
    }

    let y_abs_match = y_values
        .iter()
        .enumerate()
        .filter(|(idx, actual)| {
            **actual == screen_ptr.saturating_add((*idx as u32).saturating_mul(width))
        })
        .count();
    let y_off_match = y_values
        .iter()
        .enumerate()
        .filter(|(idx, actual)| **actual == (*idx as u32).saturating_mul(width))
        .count();
    let col_match = col_values
        .iter()
        .enumerate()
        .filter(|(idx, actual)| **actual == *idx as u32)
        .count();

    let y_abs_bad = doom_first_mismatch(&y_values, |idx| {
        screen_ptr.saturating_add((idx as u32).saturating_mul(width))
    })
    .map(|(idx, actual, expected)| {
        format!(
            "idx{} actual=0x{:08X} expected_abs=0x{:08X}",
            idx, actual, expected
        )
    })
    .unwrap_or_else(|| "none".to_string());
    let y_off_bad = doom_first_mismatch(&y_values, |idx| (idx as u32).saturating_mul(width))
        .map(|(idx, actual, expected)| {
            format!(
                "idx{} actual=0x{:08X} expected_off=0x{:08X}",
                idx, actual, expected
            )
        })
        .unwrap_or_else(|| "none".to_string());
    let col_bad = doom_first_mismatch(&col_values, |idx| idx as u32)
        .map(|(idx, actual, expected)| {
            format!(
                "idx{} actual=0x{:08X} expected=0x{:08X}",
                idx, actual, expected
            )
        })
        .unwrap_or_else(|| "none".to_string());

    let dc_x = doom_read_u32(guest_mem, state.saturating_add(0xB650));
    let dc_yl = doom_read_u32(guest_mem, state.saturating_add(0xB654));
    let dc_yh = doom_read_u32(guest_mem, state.saturating_add(0xB658));
    let dc_iscale = doom_read_u32(guest_mem, state.saturating_add(0xB65C));
    let dc_texturemid = doom_read_u32(guest_mem, state.saturating_add(0xB660));
    let dc_source = doom_read_u32(guest_mem, state.saturating_add(0xB664));
    let dc_colormap = doom_read_u32(guest_mem, state.saturating_add(0xB64C));

    let path = format!(
        r"./doom_rowlookup_{:03}_{}_state{:08X}.txt",
        seq, label, state
    );
    let mut body = String::new();
    body.push_str(&format!(
        "seq={} label={} state=0x{:08X} screen=0x{:08X} width={} height={}\n",
        seq, label, state, screen_ptr, width, height
    ));
    body.push_str(&format!(
        "dc_x={} dc_yl={} dc_yh={} dc_colormap=0x{:08X} dc_source=0x{:08X} dc_texturemid=0x{:08X} dc_iscale=0x{:08X}\n",
        dc_x, dc_yl, dc_yh, dc_colormap, dc_source, dc_texturemid, dc_iscale
    ));
    body.push_str(&format!(
        "ylookup_base=0x{:08X} y_count={} abs_match={}/{} off_match={}/{} step_ok={} step_zero={} step_other={} unique={} first_abs_bad={} first_off_bad={}\n",
        y_base,
        y_count,
        y_abs_match,
        y_count,
        y_off_match,
        y_count,
        y_step_ok,
        y_step_zero,
        y_step_other,
        doom_count_unique_u32(&y_values),
        y_abs_bad,
        y_off_bad
    ));
    for (idx, value) in y_values.iter().enumerate() {
        body.push_str(&format!(
            "y[{:03}]=0x{:08X} expected_abs=0x{:08X} expected_off=0x{:08X}\n",
            idx,
            value,
            screen_ptr.saturating_add((idx as u32).saturating_mul(width)),
            (idx as u32).saturating_mul(width)
        ));
    }
    body.push_str(&format!(
        "columnofs_base=0x{:08X} col_count={} match={}/{} step_ok={} step_zero={} step_other={} unique={} first_bad={}\n",
        col_base,
        col_count,
        col_match,
        col_count,
        col_step_ok,
        col_step_zero,
        col_step_other,
        doom_count_unique_u32(&col_values),
        col_bad
    ));
    for (idx, value) in col_values.iter().enumerate() {
        body.push_str(&format!(
            "col[{:03}]=0x{:08X} expected=0x{:08X}\n",
            idx, value, idx
        ));
    }
    let _ = std::fs::write(&path, body);

    format!(
        "rowlookup state=0x{:08X} screen=0x{:08X} y_abs={}/{} y_off={}/{} y_step(ok/zero/other)={}/{}/{} y_unique={} y_first_abs_bad={} y_first_off_bad={} y_first8=[{}] col={}/{} col_step(ok/zero/other)={}/{}/{} col_unique={} col_first_bad={} col_first8=[{}] dc_x={} dc_yl={} dc_yh={} dc_colormap=0x{:08X} dc_source=0x{:08X} path={}",
        state,
        screen_ptr,
        y_abs_match,
        y_count,
        y_off_match,
        y_count,
        y_step_ok,
        y_step_zero,
        y_step_other,
        doom_count_unique_u32(&y_values),
        y_abs_bad,
        y_off_bad,
        doom_u32_list_preview(&y_values, 8),
        col_match,
        col_count,
        col_step_ok,
        col_step_zero,
        col_step_other,
        doom_count_unique_u32(&col_values),
        col_bad,
        doom_u32_list_preview(&col_values, 8),
        dc_x,
        dc_yl,
        dc_yh,
        dc_colormap,
        dc_source,
        path
    )
}

fn doom_dump_l8_candidate_views(guest_mem: *mut u8, seq: u32, stage_src: u32) -> String {
    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, Some(stage_src));
    if candidates.is_empty() {
        return "screen_candidates=none".to_string();
    }

    let mut parts = Vec::new();
    for (i, candidate) in candidates.iter().take(16).enumerate() {
        let stage = candidate
            .stage_index
            .map(|idx| idx.to_string())
            .unwrap_or_else(|| "-".to_string());
        let label = format!(
            "candidate{:02}_{}_state{:08X}_stage{}",
            i, candidate.source, candidate.state, stage
        );
        let dump = doom_dump_l8_view(
            guest_mem,
            seq,
            candidate.screen_ptr,
            &label,
            candidate.width,
            candidate.height.min(240),
            candidate.width,
        );
        let rowlookup = doom_rowlookup_diag_for_state(
            guest_mem,
            seq,
            &label,
            candidate.state,
            candidate.screen_ptr,
            candidate.width,
            candidate.height,
        );
        debug_log(&format!(
            "[DOOM-ROWLOOKUP] seq={} candidate={} {}",
            seq, label, rowlookup
        ));
        parts.push(format!(
            "{} state=0x{:08X} screen=0x{:08X} {}x{} stage={} {} {}",
            candidate.source,
            candidate.state,
            candidate.screen_ptr,
            candidate.width,
            candidate.height,
            stage,
            dump,
            rowlookup
        ));
    }

    format!("screen_candidates={}", parts.join(" | "))
}

fn doom_dump_l8_bytes(seq: u32, label: &str, bytes: &[u8], width: u32, height: u32) -> String {
    let expected = (width as usize).saturating_mul(height as usize);
    if bytes.len() < expected || expected == 0 {
        return format!(
            "{}={}x{} unavailable len={}",
            label,
            width,
            height,
            bytes.len()
        );
    }

    let mut pixels = Vec::with_capacity(expected);
    for &v in &bytes[..expected] {
        let v = v as u32;
        pixels.push(0xFF00_0000 | (v << 16) | (v << 8) | v);
    }

    let hash = fnv1a64(&bytes[..expected]);
    let nonzero = bytes[..expected].iter().filter(|b| **b != 0).count();
    let path = format!(
        r"./doom_{}_{:03}_{}x{}.bmp",
        label, seq, width, height
    );
    write_readback_bmp(&path, width, height, &pixels);
    format!(
        "{}={}x{} hash=0x{:016X} nonzero={} path={}",
        label, width, height, hash, nonzero, path
    )
}

fn doom_count_from_cursor(base: u32, cursor: u32, elem_size: u32, max_reasonable: u32) -> u32 {
    if elem_size == 0 || cursor < base {
        return 0;
    }
    let delta = cursor - base;
    let count = delta / elem_size;
    if count <= max_reasonable {
        count
    } else {
        0
    }
}

fn doom_render_player_state_tap(name: &str, seq: u32, guest_mem: *mut u8) {
    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    if state == 0 {
        debug_log(&format!("[DOOM-RENDER-STATE] #{} {} state=NULL", seq, name));
        return;
    }

    let player = doom_read_u32(guest_mem, state.saturating_add(0xB7B0));
    let mo = doom_read_u32(guest_mem, player);
    let viewx_base = doom_read_u32(guest_mem, state.saturating_add(0xB798));
    let viewy_base = doom_read_u32(guest_mem, state.saturating_add(0xB79C));
    let viewx_off = doom_read_u32(guest_mem, state.saturating_add(0x164));
    let viewy_off = doom_read_u32(guest_mem, state.saturating_add(0x168));
    let viewx = viewx_base.wrapping_add(viewx_off);
    let viewy = viewy_base.wrapping_add(viewy_off);
    let viewz = doom_read_u32(guest_mem, state.saturating_add(0xB7A0));
    let viewangle = doom_read_u32(guest_mem, state.saturating_add(0xB7A4));
    let viewsin = doom_read_u32(guest_mem, state.saturating_add(0xB7A8));
    let viewcos = doom_read_u32(guest_mem, state.saturating_add(0xB7AC));

    let scaledviewwidth = doom_read_u32(guest_mem, state.saturating_add(0x94B8));
    let viewwidth = doom_read_u32(guest_mem, state.saturating_add(0x94BC));
    let viewheight = doom_read_u32(guest_mem, state.saturating_add(0x94C0));
    let detailshift = doom_read_u32(guest_mem, state.saturating_add(0xB7B4));
    let screenblocks = doom_read_u32(guest_mem, state.saturating_add(0x12988));
    let setdetail = doom_read_u32(guest_mem, state.saturating_add(0x1298C));
    let centerxfrac = doom_read_u32(guest_mem, state.saturating_add(0xB77C));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0xB784));
    let clipangle = doom_read_u32(guest_mem, state.saturating_add(0xB7B8));
    let draw_column_fn = doom_read_u32(guest_mem, 0x0010_1C28);
    let draw_span_fn = doom_read_u32(guest_mem, 0x0010_1C30);
    let drawer_mode = match (draw_column_fn, draw_span_fn) {
        (0x0002_66F0, 0x0002_6B60) => "hi",
        (0x0002_67C0, 0x0002_6C30) => "low",
        _ => "unknown",
    };

    let wad_loaded = doom_read_u32(guest_mem, state.saturating_add(0x22A0));
    let level_live = doom_read_u32(guest_mem, state.saturating_add(0x2284));
    let map_marker = doom_read_u32(guest_mem, state.saturating_add(0x2714));
    let render_disabled = doom_read_u32(guest_mem, state.saturating_add(0x2750));
    let player_count = doom_read_u32(guest_mem, state.saturating_add(0x1270)) & 0xFFFF;
    let bsp_nodes = doom_read_u32(guest_mem, state.saturating_add(0x5B28));
    let bsp_node_ptr = doom_read_u32(guest_mem, state.saturating_add(0x5B2C));
    let subsector_ptr = doom_read_u32(guest_mem, state.saturating_add(0x5B14));

    let drawseg_base = state.saturating_add(0x62A4);
    let drawseg_cursor = doom_read_u32(guest_mem, state.saturating_add(0x92A4));
    let drawseg_count = doom_count_from_cursor(drawseg_base, drawseg_cursor, 0x30, 0x1000);
    let first_drawseg = drawseg_base;
    let last_drawseg = if drawseg_count > 0 {
        drawseg_base.saturating_add((drawseg_count - 1).saturating_mul(0x30))
    } else {
        0
    };
    let first_ds_x1 = doom_read_u32(guest_mem, first_drawseg.saturating_add(0x04));
    let first_ds_x2 = doom_read_u32(guest_mem, first_drawseg.saturating_add(0x08));
    let last_ds_x1 = doom_read_u32(guest_mem, last_drawseg.saturating_add(0x04));
    let last_ds_x2 = doom_read_u32(guest_mem, last_drawseg.saturating_add(0x08));

    let visplane_base = state.saturating_add(0x28BF8);
    let visplane_cursor = doom_read_u32(guest_mem, state.saturating_add(0x3D7F8));
    let visplane_count = doom_count_from_cursor(visplane_base, visplane_cursor, 0x298, 0x400);
    let first_visplane = visplane_base;
    let last_visplane = if visplane_count > 0 {
        visplane_base.saturating_add((visplane_count - 1).saturating_mul(0x298))
    } else {
        0
    };
    let first_vp_minx = doom_read_u32(guest_mem, first_visplane.saturating_add(0x0C));
    let first_vp_maxx = doom_read_u32(guest_mem, first_visplane.saturating_add(0x10));
    let last_vp_minx = doom_read_u32(guest_mem, last_visplane.saturating_add(0x0C));
    let last_vp_maxx = doom_read_u32(guest_mem, last_visplane.saturating_add(0x10));
    let openings_base = state.saturating_add(0x12998);
    let openings_cursor = doom_read_u32(guest_mem, state.saturating_add(0x1C998));
    let openings_count = doom_count_from_cursor(openings_base, openings_cursor, 2, 0x8000);

    debug_log(&format!(
        "[DOOM-RENDER-STATE] #{} {} state=0x{:08X} player=0x{:08X} mo=0x{:08X} view=({}/{}, {}/{}, z={}/0x{:08X}, angle=0x{:08X}) trig(sin=0x{:08X} cos=0x{:08X}) dims scaled={} view={}x{} detail={} setdetail={} blocks={} centerxfrac=0x{:08X} projection=0x{:08X} clipangle=0x{:08X} drawers={} col=0x{:08X} span=0x{:08X} wad_loaded=0x{:08X} level_live=0x{:08X} map_marker=0x{:08X} render_disabled=0x{:08X} players={} bsp_nodes={} bsp_node_ptr=0x{:08X} subsector_ptr=0x{:08X} drawsegs count={} cursor=0x{:08X} first=0x{:08X}[x1={},x2={}] last=0x{:08X}[x1={},x2={}] visplanes count={} cursor=0x{:08X} first=0x{:08X}[minx={},maxx={}] last=0x{:08X}[minx={},maxx={}] openings count={} cursor=0x{:08X}",
        seq,
        name,
        state,
        player,
        mo,
        viewx as i32,
        viewx,
        viewy as i32,
        viewy,
        viewz as i32,
        viewz,
        viewangle,
        viewsin,
        viewcos,
        scaledviewwidth,
        viewwidth,
        viewheight,
        detailshift,
        setdetail,
        screenblocks,
        centerxfrac,
        projection,
        clipangle,
        drawer_mode,
        draw_column_fn,
        draw_span_fn,
        wad_loaded,
        level_live,
        map_marker,
        render_disabled,
        player_count,
        bsp_nodes,
        bsp_node_ptr,
        subsector_ptr,
        drawseg_count,
        drawseg_cursor,
        first_drawseg,
        first_ds_x1,
        first_ds_x2,
        last_drawseg,
        last_ds_x1,
        last_ds_x2,
        visplane_count,
        visplane_cursor,
        first_visplane,
        first_vp_minx,
        first_vp_maxx,
        last_visplane,
        last_vp_minx,
        last_vp_maxx,
        openings_count,
        openings_cursor
    ));
}

pub fn doom_render_phase_tap(name: &str, guest_mem: *mut u8) {
    if !doom_blit_diag_enabled() {
        return;
    }

    static RENDER_TAP_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = RENDER_TAP_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 32 && !seq.is_power_of_two() {
        return;
    }

    if name == "DOOM_RENDER_BEFORE_VIEW_TAP" || name == "DOOM_RENDER_AFTER_VIEW_TAP" {
        doom_render_player_state_tap(name, seq, guest_mem);
    }

    let curr_slot = doom_read_u32(guest_mem, 0x0010_1B9C);
    let live = doom_read_u32(guest_mem, 0x0010_1BB8);
    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, None);
    let mut parts = Vec::new();
    for (i, candidate) in candidates.iter().take(8).enumerate() {
        let (hash, nonzero) = doom_hash_guest_rows(
            guest_mem,
            candidate.screen_ptr,
            candidate.width,
            candidate.width,
            candidate.height.min(240),
        )
        .unwrap_or((0, 0));

        let dump_line = if doom_blit_dump_enabled() && doom_blit_dump_seq_selected(seq) {
            let label = format!(
                "render_{}_cand{:02}_{}_state{:08X}",
                name, i, candidate.source, candidate.state
            );
            doom_dump_l8_view(
                guest_mem,
                seq,
                candidate.screen_ptr,
                &label,
                candidate.width,
                candidate.height.min(240),
                candidate.width,
            )
        } else {
            "dump=skipped".to_string()
        };

        parts.push(format!(
            "{} state=0x{:08X} screen=0x{:08X} {}x{} hash=0x{:016X} nonzero={} {}",
            candidate.source,
            candidate.state,
            candidate.screen_ptr,
            candidate.width,
            candidate.height,
            hash,
            nonzero,
            dump_line
        ));
    }

    debug_log(&format!(
        "[DOOM-RENDER-PHASE] #{} {} curr_slot={} live=0x{:08X} candidates={}",
        seq,
        name,
        curr_slot,
        live,
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join(" | ")
        }
    ));
}

pub fn doom_column_write_tap(
    name: &str,
    guest_mem: *mut u8,
    dest: u32,
    pixel: u8,
    ecx: u32,
    ebx: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_blit_diag_enabled() {
        return;
    }

    static COLUMN_WRITE_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = COLUMN_WRITE_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 64 && !seq.is_power_of_two() {
        return;
    }

    let curr_slot = doom_read_u32(guest_mem, 0x0010_1B9C);
    let live = doom_read_u32(guest_mem, 0x0010_1BB8);
    let before = if dest < 0x2000_0000 {
        unsafe { *guest_mem.add(dest as usize) }
    } else {
        0
    };

    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, None);
    let mut matches = Vec::new();
    let mut selected_state = live;
    for candidate in candidates.iter().take(16) {
        let visible_h = candidate.height.min(240);
        let Some(len) = candidate.width.checked_mul(visible_h) else {
            continue;
        };
        let start = candidate.screen_ptr;
        let end = start.saturating_add(len);
        if dest >= start && dest < end {
            let off = dest - start;
            let x = if candidate.width != 0 {
                off % candidate.width
            } else {
                0
            };
            let y = if candidate.width != 0 {
                off / candidate.width
            } else {
                0
            };
            selected_state = candidate.state;
            matches.push(format!(
                "{} state=0x{:08X} screen=0x{:08X} off={} x={} y={} {}x{}",
                candidate.source,
                candidate.state,
                candidate.screen_ptr,
                off,
                x,
                y,
                candidate.width,
                candidate.height
            ));
        }
    }

    let dc_x = doom_read_u32(guest_mem, selected_state.saturating_add(0xB650));
    let dc_yl = doom_read_u32(guest_mem, selected_state.saturating_add(0xB654));
    let dc_yh = doom_read_u32(guest_mem, selected_state.saturating_add(0xB658));
    let dc_iscale = doom_read_u32(guest_mem, selected_state.saturating_add(0xB65C));
    let dc_texturemid = doom_read_u32(guest_mem, selected_state.saturating_add(0xB660));
    let dc_source = doom_read_u32(guest_mem, selected_state.saturating_add(0xB664));
    let dc_colormap = doom_read_u32(guest_mem, selected_state.saturating_add(0xB64C));
    let source16 = doom_guest_byte_sample(guest_mem, dc_source, 16);
    let colormap16 = doom_guest_byte_sample(guest_mem, dc_colormap, 16);
    let dest8 = doom_guest_byte_sample(guest_mem, dest, 8);
    let (frac0, src_idx_raw, src_idx, source_at_frac16) =
        doom_column_source_at_frac_sample(guest_mem, dc_source, dc_texturemid, dc_yl, dc_iscale);

    debug_log(&format!(
        "[DOOM-COLUMN-WRITE] #{} {} dest=0x{:08X} before=0x{:02X} pixel=0x{:02X} curr_slot={} live=0x{:08X} match={} state=0x{:08X} dc_x={} dc_yl={} dc_yh={} dc_colormap=0x{:08X} dc_source=0x{:08X} dc_texturemid=0x{:08X} dc_iscale=0x{:08X} frac0=0x{:08X} src_idx_raw={} src_idx={} source16={} source_at_frac16={} colormap16={} dest8_before={} regs ecx=0x{:08X} ebx=0x{:08X} ebp=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        dest,
        before,
        pixel,
        curr_slot,
        live,
        if matches.is_empty() {
            "outside_candidates".to_string()
        } else {
            matches.join(" | ")
        },
        selected_state,
        dc_x,
        dc_yl,
        dc_yh,
        dc_colormap,
        dc_source,
        dc_texturemid,
        dc_iscale,
        frac0,
        src_idx_raw,
        src_idx,
        source16,
        source_at_frac16,
        colormap16,
        dest8,
        ecx,
        ebx,
        ebp,
        esi,
        edi
    ));
}

pub fn doom_column_call_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_blit_diag_enabled() && !doom_wall_probe_enabled() {
        return;
    }

    static COLUMN_CALL_BEFORE_SEQ: AtomicU32 = AtomicU32::new(0);
    static COLUMN_CALL_AFTER_SEQ: AtomicU32 = AtomicU32::new(0);
    let is_after = name == "DOOM_COLUMN_CALL_AFTER_TAP"
        || name == "DOOM_COLUMN_CALL_AFTER_RET"
        || name == "DOOM_COLUMN_CALL_AFTER_RETMISS";
    let seq = if is_after {
        COLUMN_CALL_AFTER_SEQ.fetch_add(1, Ordering::Relaxed)
    } else {
        COLUMN_CALL_BEFORE_SEQ.fetch_add(1, Ordering::Relaxed)
    };
    if seq >= 64 && !seq.is_power_of_two() {
        return;
    }

    let curr_slot = doom_read_u32(guest_mem, 0x0010_1B9C);
    let live = doom_read_u32(guest_mem, 0x0010_1BB8);
    let draw_column_fn = doom_read_u32(guest_mem, 0x0010_1C28);
    let state = live;
    let dc_x = doom_read_u32(guest_mem, state.saturating_add(0xB650));
    let dc_yl = doom_read_u32(guest_mem, state.saturating_add(0xB654));
    let dc_yh = doom_read_u32(guest_mem, state.saturating_add(0xB658));
    let dc_iscale = doom_read_u32(guest_mem, state.saturating_add(0xB65C));
    let dc_texturemid = doom_read_u32(guest_mem, state.saturating_add(0xB660));
    let dc_source = doom_read_u32(guest_mem, state.saturating_add(0xB664));
    let dc_colormap = doom_read_u32(guest_mem, state.saturating_add(0xB64C));
    let rw_x = doom_read_u32(guest_mem, state.saturating_add(0x1E9B0));
    let rw_stopx = doom_read_u32(guest_mem, state.saturating_add(0x1E9B4));
    let rw_distance = doom_read_u32(guest_mem, state.saturating_add(0x1E9C0));
    let rw_scale = doom_read_u32(guest_mem, state.saturating_add(0x1E9C4));
    let rw_scalestep = doom_read_u32(guest_mem, state.saturating_add(0x1E9C8));
    let rw_normalangle = doom_read_u32(guest_mem, state.saturating_add(0x1E9A8));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let detailshift = doom_read_u32(guest_mem, state.saturating_add(0x0B7B4));

    let ylookup_addr = if dc_yl < 240 {
        state
            .saturating_add(0x94CC)
            .saturating_add(dc_yl.saturating_mul(4))
    } else {
        0
    };
    let columnofs_addr = if dc_x < 320 {
        state
            .saturating_add(0xA1CC)
            .saturating_add(dc_x.saturating_mul(4))
    } else {
        0
    };
    let row_ptr = if ylookup_addr != 0 {
        doom_read_u32(guest_mem, ylookup_addr)
    } else {
        0
    };
    let col_off = if columnofs_addr != 0 {
        doom_read_u32(guest_mem, columnofs_addr)
    } else {
        0
    };
    let dest = row_ptr.wrapping_add(col_off);
    let before = if dest < 0x2000_0000 {
        unsafe { *guest_mem.add(dest as usize) }
    } else {
        0
    };
    let source16 = doom_guest_byte_sample(guest_mem, dc_source, 16);
    let colormap16 = doom_guest_byte_sample(guest_mem, dc_colormap, 16);
    let dest8 = doom_guest_byte_sample(guest_mem, dest, 8);
    let (frac0, src_idx_raw, src_idx, source_at_frac16) =
        doom_column_source_at_frac_sample(guest_mem, dc_source, dc_texturemid, dc_yl, dc_iscale);
    let dest8_label = if is_after {
        "dest8_after"
    } else {
        "dest8_before"
    };

    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, None);
    let mut matches = Vec::new();
    for candidate in candidates.iter().take(16) {
        let visible_h = candidate.height.min(240);
        let Some(len) = candidate.width.checked_mul(visible_h) else {
            continue;
        };
        let start = candidate.screen_ptr;
        let end = start.saturating_add(len);
        if dest >= start && dest < end {
            let off = dest - start;
            let x = if candidate.width != 0 {
                off % candidate.width
            } else {
                0
            };
            let y = if candidate.width != 0 {
                off / candidate.width
            } else {
                0
            };
            matches.push(format!(
                "{} state=0x{:08X} screen=0x{:08X} off={} x={} y={} {}x{}",
                candidate.source,
                candidate.state,
                candidate.screen_ptr,
                off,
                x,
                y,
                candidate.width,
                candidate.height
            ));
        }
    }

    debug_log(&format!(
        "[DOOM-COLUMN-CALL] #{} {} fn=0x{:08X} computed_dest=0x{:08X} before=0x{:02X} row_ptr=0x{:08X} col_off=0x{:08X} curr_slot={} live=0x{:08X} match={} rw_x={} rw_stopx={} rw_distance=0x{:08X}/{} rw_scale=0x{:08X}/{} rw_scalestep=0x{:08X}/{} rw_normalangle=0x{:08X} projection=0x{:08X} detailshift={} dc_x={} dc_yl={} dc_yh={} dc_colormap=0x{:08X} dc_source=0x{:08X} dc_texturemid=0x{:08X} dc_iscale=0x{:08X} frac0=0x{:08X} src_idx_raw={} src_idx={} source16={} source_at_frac16={} colormap16={} {}={} regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        draw_column_fn,
        dest,
        before,
        row_ptr,
        col_off,
        curr_slot,
        live,
        if matches.is_empty() {
            "outside_candidates".to_string()
        } else {
            matches.join(" | ")
        },
        rw_x,
        rw_stopx,
        rw_distance,
        rw_distance as i32,
        rw_scale,
        rw_scale as i32,
        rw_scalestep,
        rw_scalestep as i32,
        rw_normalangle,
        projection,
        detailshift,
        dc_x,
        dc_yl,
        dc_yh,
        dc_colormap,
        dc_source,
        dc_texturemid,
        dc_iscale,
        frac0,
        src_idx_raw,
        src_idx,
        source16,
        source_at_frac16,
        colormap16,
        dest8_label,
        dest8,
        eax,
        ecx,
        edx,
        esi,
        edi
    ));
}

pub fn doom_wall_scale_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static WALL_SCALE_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = WALL_SCALE_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 128 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let drawseg = doom_read_u32(guest_mem, state.saturating_add(0x92A4));
    let start = doom_read_u32(guest_mem, state.saturating_add(0x1E9B0));
    let stop = doom_read_u32(guest_mem, state.saturating_add(0x1E9B4));
    let rw_normalangle = doom_read_u32(guest_mem, state.saturating_add(0x1E9A8));
    let rw_distance = doom_read_u32(guest_mem, state.saturating_add(0x1E9C0));
    let rw_scale = doom_read_u32(guest_mem, state.saturating_add(0x1E9C4));
    let rw_scalestep = doom_read_u32(guest_mem, state.saturating_add(0x1E9C8));
    let rw_midtexturemid = doom_read_u32(guest_mem, state.saturating_add(0x1E9CC));
    let rw_toptexturemid = doom_read_u32(guest_mem, state.saturating_add(0x1E9D0));
    let rw_bottomtexturemid = doom_read_u32(guest_mem, state.saturating_add(0x1E9D4));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let detailshift = doom_read_u32(guest_mem, state.saturating_add(0x0B7B4));
    let viewx = doom_read_u32(guest_mem, state.saturating_add(0x0B7A0));

    let ds_curline = doom_read_u32(guest_mem, drawseg);
    let ds_x1 = doom_read_u32(guest_mem, drawseg.saturating_add(0x04));
    let ds_x2 = doom_read_u32(guest_mem, drawseg.saturating_add(0x08));
    let ds_scale1 = doom_read_u32(guest_mem, drawseg.saturating_add(0x0C));
    let ds_scale2 = doom_read_u32(guest_mem, drawseg.saturating_add(0x10));
    let ds_scalestep = doom_read_u32(guest_mem, drawseg.saturating_add(0x14));

    debug_log(&format!(
        "[DOOM-WALL-SCALE] #{} {} state=0x{:08X} drawseg=0x{:08X} start={} stop={} rw_normalangle=0x{:08X} rw_distance=0x{:08X}/{} rw_scale=0x{:08X}/{} rw_scalestep=0x{:08X}/{} texmid(mid/top/bot)=0x{:08X}/0x{:08X}/0x{:08X} projection=0x{:08X} detailshift={} viewx=0x{:08X} ds_curline=0x{:08X} ds_x1={} ds_x2={} ds_scale1=0x{:08X}/{} ds_scale2=0x{:08X}/{} ds_scalestep=0x{:08X}/{} regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        state,
        drawseg,
        start,
        stop,
        rw_normalangle,
        rw_distance,
        rw_distance as i32,
        rw_scale,
        rw_scale as i32,
        rw_scalestep,
        rw_scalestep as i32,
        rw_midtexturemid,
        rw_toptexturemid,
        rw_bottomtexturemid,
        projection,
        detailshift,
        viewx,
        ds_curline,
        ds_x1,
        ds_x2,
        ds_scale1,
        ds_scale1 as i32,
        ds_scale2,
        ds_scale2 as i32,
        ds_scalestep,
        ds_scalestep as i32,
        eax,
        ecx,
        edx,
        esi,
        edi
    ));
}

fn doom_wall_viewmap_selected_idx(idx: u32) -> bool {
    const HOT_LOOKUP_INDICES: &[u32] = &[
        604, 686, 689, 707, 708, 768, 772, 1072, 1412, 1418, 1425, 1426, 1575, 1576, 1635, 1658,
        1659, 1746, 2851, 2853, 3150, 3316, 3492, 3501,
    ];
    idx < 4
        || idx % 512 == 0
        || (440..=520).contains(&idx)
        || (3550..=3620).contains(&idx)
        || (4070..=4095).contains(&idx)
        || HOT_LOOKUP_INDICES.contains(&idx)
}

fn doom_wall_viewmap_expected_tag(tangent: u32) -> &'static str {
    let signed = tangent as i32;
    if signed > 0x0002_0000 {
        "hi->-1"
    } else if signed < -0x0002_0000 {
        "lo->limit+1"
    } else {
        "computed"
    }
}

fn doom_fixed_mul_i32(a: i32, b: i32) -> i32 {
    (((a as i64) * (b as i64)) >> 16) as i32
}

fn doom_expected_viewangletox(
    tangent: u32,
    centerxfrac: u32,
    projection: u32,
    scaledviewwidth: u32,
) -> (i32, i32) {
    let limit = scaledviewwidth.min(640) as i32;
    let raw = if (tangent as i32) > 0x0002_0000 {
        -1
    } else if (tangent as i32) < -0x0002_0000 {
        limit.saturating_add(1)
    } else {
        let projected = doom_fixed_mul_i32(tangent as i32, projection as i32);
        ((centerxfrac as i32)
            .wrapping_sub(projected)
            .wrapping_add(0xFFFF))
            >> 16
    }
    .clamp(-1, limit.saturating_add(1));

    let final_value = if raw == -1 {
        0
    } else if raw == limit.saturating_add(1) {
        limit
    } else {
        raw
    };

    (raw, final_value)
}

pub fn doom_wall_viewmap_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esi: u32,
    edi: u32,
    esp: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static VIEWMAP_ENTRY_SEQ: AtomicU32 = AtomicU32::new(0);
    static VIEWMAP_BUILD_SEQ: AtomicU32 = AtomicU32::new(0);
    static VIEWMAP_CLAMP_SEQ: AtomicU32 = AtomicU32::new(0);

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let limit_x = doom_read_u32(guest_mem, state.saturating_add(0x94B8));
    let view_w = doom_read_u32(guest_mem, state.saturating_add(0x94BC));
    let view_h = doom_read_u32(guest_mem, state.saturating_add(0x94C0));

    if name == "DOOM_WALL_VIEWMAP_BUILD_STORE_TAP" {
        let seq = VIEWMAP_BUILD_SEQ.fetch_add(1, Ordering::Relaxed);
        let src_off = esi.wrapping_sub(4);
        let idx = src_off.wrapping_sub(0x000B_6C88) / 4;
        if src_off < 0x000B_6C88 || idx >= 4096 || !doom_wall_viewmap_selected_idx(idx) {
            return;
        }

        let tangent = doom_read_u32(guest_mem, src_off);
        let target_addr = edx.wrapping_add(eax);
        let old_table = doom_read_u32(guest_mem, target_addr);
        let expected = doom_wall_viewmap_expected_tag(tangent);
        debug_log(&format!(
            "[DOOM-WALL-VIEWMAP-BUILD] #{} idx={} src_off=0x{:08X} tangent=0x{:08X}/{} expected={} store_value=0x{:08X}/{} old_table=0x{:08X}/{} target=0x{:08X} state=0x{:08X} limit_x={} limit_plus1={} view_w={} view_h={} regs eax=0x{:08X} ecx=0x{:08X}/{} edx=0x{:08X} ebx=0x{:08X}/{} esi=0x{:08X} edi=0x{:08X}",
            seq,
            idx,
            src_off,
            tangent,
            tangent as i32,
            expected,
            ecx,
            ecx as i32,
            old_table,
            old_table as i32,
            target_addr,
            state,
            limit_x,
            limit_x.wrapping_add(1),
            view_w,
            view_h,
            eax,
            ecx,
            ecx as i32,
            edx,
            ebx,
            ebx as i32,
            esi,
            edi
        ));
        return;
    }

    if name == "DOOM_WALL_VIEWMAP_CLAMP_INPUT_TAP" || name == "DOOM_WALL_VIEWMAP_CLAMP_FINAL_TAP" {
        let seq = VIEWMAP_CLAMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let idx = esi.wrapping_sub(0x0000_B7BC) / 4;
        if esi < 0x0000_B7BC || idx >= 4096 || !doom_wall_viewmap_selected_idx(idx) {
            return;
        }

        let tangent = doom_read_u32(guest_mem, 0x000B_6C88u32.wrapping_add(idx.wrapping_mul(4)));
        let value = doom_read_u32(guest_mem, ecx.wrapping_add(esi));
        let expected = doom_wall_viewmap_expected_tag(tangent);
        debug_log(&format!(
            "[DOOM-WALL-VIEWMAP-CLAMP] #{} {} idx={} tangent=0x{:08X}/{} expected={} table_value=0x{:08X}/{} target=0x{:08X} state_reg=0x{:08X} global_state=0x{:08X} limit_x={} limit_plus1={} view_w={} view_h={} regs eax=0x{:08X}/{} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X}/{} esi=0x{:08X} edi=0x{:08X}",
            seq,
            name,
            idx,
            tangent,
            tangent as i32,
            expected,
            value,
            value as i32,
            ecx.wrapping_add(esi),
            ecx,
            state,
            limit_x,
            limit_x.wrapping_add(1),
            view_w,
            view_h,
            eax,
            eax as i32,
            ecx,
            edx,
            ebx,
            ebx as i32,
            esi,
            edi
        ));
        return;
    }

    let seq = VIEWMAP_ENTRY_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 64 && !seq.is_power_of_two() {
        return;
    }
    let centerxfrac = doom_read_u32(guest_mem, state.saturating_add(0x0B77C));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let clipangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7B8));
    let setsizeneeded = doom_read_u32(guest_mem, state.saturating_add(0x12984));
    let setblocks = doom_read_u32(guest_mem, state.saturating_add(0x12988));
    let setdetail = doom_read_u32(guest_mem, state.saturating_add(0x1298C));
    let table0 = doom_read_u32(guest_mem, state.saturating_add(0x0B7BC));
    let table449 = doom_read_u32(guest_mem, state.saturating_add(0x0B7BC + 449 * 4));
    let table500 = doom_read_u32(guest_mem, state.saturating_add(0x0B7BC + 500 * 4));
    let table3596 = doom_read_u32(guest_mem, state.saturating_add(0x0B7BC + 3596 * 4));
    let table4076 = doom_read_u32(guest_mem, state.saturating_add(0x0B7BC + 4076 * 4));
    debug_log(&format!(
        "[DOOM-WALL-VIEWMAP-ENTRY] #{} {} state=0x{:08X} limit_x={} limit_plus1={} view_w={} view_h={} centerxfrac=0x{:08X}/{} projection=0x{:08X} clipangle=0x{:08X}/{} setsizeneeded={} setblocks={} setdetail={} samples[0,449,500,3596,4076]=0x{:08X}/{},0x{:08X}/{},0x{:08X}/{},0x{:08X}/{},0x{:08X}/{} args_ecx=0x{:08X}/{} args_edx=0x{:08X}/{} esp=0x{:08X} regs eax=0x{:08X} ebx=0x{:08X}/{} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        state,
        limit_x,
        limit_x.wrapping_add(1),
        view_w,
        view_h,
        centerxfrac,
        centerxfrac as i32,
        projection,
        clipangle,
        clipangle as i32,
        setsizeneeded,
        setblocks,
        setdetail,
        table0,
        table0 as i32,
        table449,
        table449 as i32,
        table500,
        table500 as i32,
        table3596,
        table3596 as i32,
        table4076,
        table4076 as i32,
        ecx,
        ecx as i32,
        edx,
        edx as i32,
        esp,
        eax,
        ebx,
        ebx as i32,
        esi,
        edi
    ));
}

pub fn doom_wall_range_entry_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esi: u32,
    edi: u32,
    esp: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static WALL_RANGE_ENTRY_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = WALL_RANGE_ENTRY_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 128 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let drawseg_head_addr = state.saturating_add(0x92A4);
    let drawseg_head = doom_read_u32(guest_mem, drawseg_head_addr);
    let curline = doom_read_u32(guest_mem, state.saturating_add(0x6290));
    let sidedef = doom_read_u32(guest_mem, state.saturating_add(0x6294));
    let linedef = doom_read_u32(guest_mem, state.saturating_add(0x6298));
    let side = doom_read_u32(guest_mem, state.saturating_add(0x629C));
    let old_rw_x = doom_read_u32(guest_mem, state.saturating_add(0x1E9B0));
    let old_rw_stopx = doom_read_u32(guest_mem, state.saturating_add(0x1E9B4));
    let old_rw_distance = doom_read_u32(guest_mem, state.saturating_add(0x1E9C0));
    let old_rw_scale = doom_read_u32(guest_mem, state.saturating_add(0x1E9C4));
    let old_rw_scalestep = doom_read_u32(guest_mem, state.saturating_add(0x1E9C8));
    let viewangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7A4));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let ret_addr = doom_read_u32(guest_mem, esp);
    let range_tag = if ecx == 0 && edx >= 319 {
        "full"
    } else if edx < ecx || edx > 320 {
        "bad"
    } else {
        "partial"
    };

    debug_log(&format!(
        "[DOOM-WALL-RANGE-ENTRY] #{} {} tag={} start(ecx)={} stop(edx)={} span={} state=0x{:08X} drawseg_head@0x{:08X}=0x{:08X} curline=0x{:08X} sidedef=0x{:08X} linedef=0x{:08X} side=0x{:08X} old_rw_x={} old_rw_stopx={} old_rw_distance=0x{:08X}/{} old_rw_scale=0x{:08X}/{} old_rw_scalestep=0x{:08X}/{} viewangle=0x{:08X} projection=0x{:08X} esp=0x{:08X} ret=0x{:08X} regs eax=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        range_tag,
        ecx,
        edx,
        edx.wrapping_sub(ecx),
        state,
        drawseg_head_addr,
        drawseg_head,
        curline,
        sidedef,
        linedef,
        side,
        old_rw_x,
        old_rw_stopx,
        old_rw_distance,
        old_rw_distance as i32,
        old_rw_scale,
        old_rw_scale as i32,
        old_rw_scalestep,
        old_rw_scalestep as i32,
        viewangle,
        projection,
        esp,
        ret_addr,
        eax,
        ebx,
        esi,
        edi
    ));
}

pub fn doom_wall_projected_x_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    esp: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static WALL_PROJECTED_X_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = WALL_PROJECTED_X_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 160 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let side = doom_read_u32(guest_mem, state.saturating_add(0x629C));
    let curline = doom_read_u32(guest_mem, state.saturating_add(0x6290));
    let sidedef = doom_read_u32(guest_mem, state.saturating_add(0x6294));
    let linedef = doom_read_u32(guest_mem, state.saturating_add(0x6298));
    let viewangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7A4));
    let clipangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7B8));
    let xtoview_base = state.saturating_add(0x0B7BC);
    let slot10 = doom_read_u32(guest_mem, esp.saturating_add(0x10));
    let slot14 = doom_read_u32(guest_mem, esp.saturating_add(0x14));
    let ret_addr = doom_read_u32(guest_mem, esp.saturating_add(0x18));
    let side_front = doom_read_u32(guest_mem, side);
    let side_back = doom_read_u32(guest_mem, side.saturating_add(4));

    let projected_first = if name == "DOOM_WALL_PROJECTED_X_TAP" {
        ebx
    } else {
        slot10
    };
    let projected_after = if name == "DOOM_WALL_PROJECTED_X_TAP" {
        esi
    } else {
        slot14
    };
    let final_stop = projected_after.wrapping_sub(1);
    let tag = if projected_first == 0 && final_stop >= 319 {
        "full"
    } else if final_stop < projected_first || final_stop > 320 {
        "bad"
    } else {
        "partial"
    };

    debug_log(&format!(
        "[DOOM-WALL-PROJECT-X] #{} {} tag={} projected_first={} projected_after={} final_stop={} span={} state=0x{:08X} curline=0x{:08X} sidedef=0x{:08X} linedef=0x{:08X} side=0x{:08X} side_front=0x{:08X} side_back=0x{:08X} viewangle=0x{:08X} clipangle=0x{:08X} xtoview@0x{:08X} esp=0x{:08X} stack10=0x{:08X}/{} stack14=0x{:08X}/{} ret_guess=0x{:08X} regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X}/{} ebp=0x{:08X} esi=0x{:08X}/{} edi=0x{:08X}",
        seq,
        name,
        tag,
        projected_first,
        projected_after,
        final_stop,
        final_stop.wrapping_sub(projected_first),
        state,
        curline,
        sidedef,
        linedef,
        side,
        side_front,
        side_back,
        viewangle,
        clipangle,
        xtoview_base,
        esp,
        slot10,
        slot10 as i32,
        slot14,
        slot14 as i32,
        ret_addr,
        eax,
        ecx,
        edx,
        ebx,
        ebx as i32,
        ebp,
        esi,
        esi as i32,
        edi
    ));
}

fn doom_angle_delta_to_rad(angle: u32, origin: u32) -> f64 {
    let delta = angle.wrapping_sub(origin) as i32;
    (delta as f64 / 4_294_967_296.0) * std::f64::consts::TAU
}

fn doom_oracle_point_angle(x: i32, y: i32, viewx: i32, viewy: i32) -> u32 {
    let dx = x.wrapping_sub(viewx);
    let dy = y.wrapping_sub(viewy);
    if dx == 0 && dy == 0 {
        return 0;
    }

    let mut angle = (dy as f64).atan2(dx as f64);
    if angle < 0.0 {
        angle += std::f64::consts::TAU;
    }
    ((angle / std::f64::consts::TAU) * 4_294_967_296.0) as u32
}

fn doom_oracle_project_x(
    angle: u32,
    viewangle: u32,
    center: f64,
    focal: f64,
    width: u32,
    sign: f64,
) -> Option<f64> {
    let rel = doom_angle_delta_to_rad(angle, viewangle);
    let c = rel.cos();
    if c.abs() < 1.0e-6 {
        return None;
    }
    Some((center + sign * rel.tan() * focal).clamp(0.0, width as f64))
}

pub fn doom_wall_synthetic_projection(guest_mem: *mut u8, seg: u32) -> Option<(u32, u32)> {
    static SYNTH_PROJECT_SEQ: AtomicU32 = AtomicU32::new(0);

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    if state == 0 || seg == 0 {
        return None;
    }

    let viewx = doom_read_u32(guest_mem, state.saturating_add(0x0B798))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x164))) as i32;
    let viewy = doom_read_u32(guest_mem, state.saturating_add(0x0B79C))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x168))) as i32;
    let viewangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7A4));
    let centerxfrac = doom_read_u32(guest_mem, state.saturating_add(0x0B77C));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let width = doom_read_u32(guest_mem, state.saturating_add(0x094B8))
        .max(1)
        .min(640);
    let center = if centerxfrac != 0 {
        centerxfrac as f64 / 65_536.0
    } else {
        width as f64 / 2.0
    };
    let focal = if projection != 0 {
        projection as f64 / 65_536.0
    } else {
        center
    };

    let v1 = doom_read_u32(guest_mem, seg);
    let v2 = doom_read_u32(guest_mem, seg.saturating_add(4));
    if v1 == 0 || v2 == 0 {
        return None;
    }

    let v1x = doom_read_u32(guest_mem, v1) as i32;
    let v1y = doom_read_u32(guest_mem, v1.saturating_add(4)) as i32;
    let v2x = doom_read_u32(guest_mem, v2) as i32;
    let v2y = doom_read_u32(guest_mem, v2.saturating_add(4)) as i32;
    let a1 = doom_oracle_point_angle(v1x, v1y, viewx, viewy);
    let a2 = doom_oracle_point_angle(v2x, v2y, viewx, viewy);

    let mut ranges = [(0u32, 0u32); 2];
    let mut range_count = 0usize;
    for sign in [-1.0, 1.0] {
        let (Some(x1), Some(x2)) = (
            doom_oracle_project_x(a1, viewangle, center, focal, width, sign),
            doom_oracle_project_x(a2, viewangle, center, focal, width, sign),
        ) else {
            continue;
        };
        let lo = x1.min(x2).ceil().clamp(0.0, width as f64) as u32;
        let after = x1.max(x2).ceil().clamp(0.0, width as f64) as u32;
        if after > lo && range_count < ranges.len() {
            ranges[range_count] = (lo, after);
            range_count += 1;
        }
    }

    if range_count == 0 {
        return None;
    }

    let mut synth_start = ranges[0].0;
    let mut synth_after = ranges[0].1;
    for &(lo, after) in ranges.iter().take(range_count).skip(1) {
        synth_start = synth_start.min(lo);
        synth_after = synth_after.max(after);
    }

    let n = SYNTH_PROJECT_SEQ.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[DOOM-WALL-SYNTH-PROJECT] #{} seg=0x{:08X} synth_start={} synth_after={} viewangle=0x{:08X} view=({}, {}) v1=({}, {}) v2=({}, {}) a1=0x{:08X} a2=0x{:08X} ranges={}{}{}",
            n,
            seg,
            synth_start,
            synth_after,
            viewangle,
            viewx,
            viewy,
            v1x,
            v1y,
            v2x,
            v2y,
            a1,
            a2,
            range_count,
            if range_count > 0 {
                format!(" [{}, {})", ranges[0].0, ranges[0].1)
            } else {
                String::new()
            },
            if range_count > 1 {
                format!(" [{}, {})", ranges[1].0, ranges[1].1)
            } else {
                String::new()
            }
        ));
    }

    Some((synth_start, synth_after))
}

pub fn doom_wall_angle_clip_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static WALL_ANGLE_CLIP_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = WALL_ANGLE_CLIP_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 192 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let clipangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7B8));
    let viewangle = doom_read_u32(guest_mem, state.saturating_add(0x0B7A4));
    let projection = doom_read_u32(guest_mem, state.saturating_add(0x0B784));
    let centerxfrac = doom_read_u32(guest_mem, state.saturating_add(0x0B77C));
    let scaledviewwidth = doom_read_u32(guest_mem, state.saturating_add(0x094B8));
    let viewwidth = doom_read_u32(guest_mem, state.saturating_add(0x094BC));
    let viewx = doom_read_u32(guest_mem, state.saturating_add(0x0B798))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x164)));
    let viewy = doom_read_u32(guest_mem, state.saturating_add(0x0B79C))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x168)));
    let seg = ebp;
    let v1 = doom_read_u32(guest_mem, seg);
    let v2 = doom_read_u32(guest_mem, seg.saturating_add(4));
    let v1x = doom_read_u32(guest_mem, v1);
    let v1y = doom_read_u32(guest_mem, v1.saturating_add(4));
    let v2x = doom_read_u32(guest_mem, v2);
    let v2y = doom_read_u32(guest_mem, v2.saturating_add(4));
    let angle1 = edi;
    let angle2 = esi;
    let (plus1, plus2) = if name == "DOOM_WALL_PRE_LOOKUP_TAP" {
        (angle1, angle2)
    } else {
        (
            angle1.wrapping_add(0x4000_0000),
            angle2.wrapping_add(0x4000_0000),
        )
    };
    let idx1 = plus1 >> 19;
    let idx2 = plus2 >> 19;
    let table_base = state.saturating_add(0x0B7BC);
    let table1 = doom_read_u32(guest_mem, table_base.wrapping_add(idx1.wrapping_mul(4)));
    let table2 = doom_read_u32(guest_mem, table_base.wrapping_add(idx2.wrapping_mul(4)));
    let tangent1 = doom_read_u32(guest_mem, 0x000B_6C88u32.wrapping_add(idx1.wrapping_mul(4)));
    let tangent2 = doom_read_u32(guest_mem, 0x000B_6C88u32.wrapping_add(idx2.wrapping_mul(4)));
    let (expected_raw1, expected_final1) =
        doom_expected_viewangletox(tangent1, centerxfrac, projection, scaledviewwidth);
    let (expected_raw2, expected_final2) =
        doom_expected_viewangletox(tangent2, centerxfrac, projection, scaledviewwidth);
    let idx_tag = if idx1 >= 4096 || idx2 >= 4096 {
        "badidx"
    } else {
        "okidx"
    };

    debug_log(&format!(
        "[DOOM-WALL-ANGLE-CLIP] #{} {} {} state=0x{:08X} seg=0x{:08X} v1=0x{:08X} v2=0x{:08X} v1x={} v1y={} v2x={} v2y={} viewx={} viewy={} projection=0x{:08X}/{} centerxfrac=0x{:08X}/{} scaledviewwidth={} viewwidth={} clipangle=0x{:08X}/{} viewangle=0x{:08X} angle1=0x{:08X}/{} angle2=0x{:08X}/{} plus1=0x{:08X} plus2=0x{:08X} idx1={} idx2={} table1=0x{:08X}/{} table2=0x{:08X}/{} tangent1=0x{:08X}/{} tangent2=0x{:08X}/{} expect1_raw={} expect1_final={} expect2_raw={} expect2_final={} regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X}/{} ebp=0x{:08X} esi=0x{:08X}/{} edi=0x{:08X}/{}",
        seq,
        name,
        idx_tag,
        state,
        seg,
        v1,
        v2,
        v1x as i32,
        v1y as i32,
        v2x as i32,
        v2y as i32,
        viewx as i32,
        viewy as i32,
        projection,
        projection as i32,
        centerxfrac,
        centerxfrac as i32,
        scaledviewwidth,
        viewwidth,
        clipangle,
        clipangle as i32,
        viewangle,
        angle1,
        angle1 as i32,
        angle2,
        angle2 as i32,
        plus1,
        plus2,
        idx1,
        idx2,
        table1,
        table1 as i32,
        table2,
        table2 as i32,
        tangent1,
        tangent1 as i32,
        tangent2,
        tangent2 as i32,
        expected_raw1,
        expected_final1,
        expected_raw2,
        expected_final2,
        eax,
        ecx,
        edx,
        ebx,
        ebx as i32,
        ebp,
        esi,
        esi as i32,
        edi,
        edi as i32
    ));
}

pub fn doom_wall_pointangle_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ebx: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static WALL_POINTANGLE_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = WALL_POINTANGLE_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 160 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let v1 = doom_read_u32(guest_mem, ebp);
    let v2 = doom_read_u32(guest_mem, ebp.saturating_add(4));
    let v = if name == "DOOM_WALL_POINTANGLE1_RET_TAP" {
        v1
    } else {
        v2
    };
    let vx = doom_read_u32(guest_mem, v) as i32;
    let vy = doom_read_u32(guest_mem, v.saturating_add(4)) as i32;
    let viewx = doom_read_u32(guest_mem, state.saturating_add(0x0B798))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x164))) as i32;
    let viewy = doom_read_u32(guest_mem, state.saturating_add(0x0B79C))
        .wrapping_add(doom_read_u32(guest_mem, state.saturating_add(0x168))) as i32;
    let dx = vx.wrapping_sub(viewx);
    let dy = vy.wrapping_sub(viewy);
    let host_angle = if dx == 0 && dy == 0 {
        0
    } else {
        let mut a = (dy as f64).atan2(dx as f64);
        if a < 0.0 {
            a += std::f64::consts::TAU;
        }
        ((a / std::f64::consts::TAU) * 4_294_967_296.0) as u32
    };
    let delta = eax.wrapping_sub(host_angle);
    let abs_delta = (delta as i32).wrapping_abs() as u32;

    debug_log(&format!(
        "[DOOM-WALL-POINTANGLE] #{} {} state=0x{:08X} seg=0x{:08X} v1=0x{:08X} v2=0x{:08X} used_v=0x{:08X} vx={} vy={} viewx={} viewy={} dx={} dy={} guest_angle=0x{:08X}/{} host_angle=0x{:08X}/{} delta=0x{:08X}/{} abs_delta={} regs ebx=0x{:08X}/{} esi=0x{:08X}/{} edi=0x{:08X}/{}",
        seq,
        name,
        state,
        ebp,
        v1,
        v2,
        v,
        vx,
        vy,
        viewx,
        viewy,
        dx,
        dy,
        eax,
        eax as i32,
        host_angle,
        host_angle as i32,
        delta,
        delta as i32,
        abs_delta,
        ebx,
        ebx as i32,
        esi,
        esi as i32,
        edi,
        edi as i32
    ));
}

pub fn doom_masked_scale_source_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_wall_probe_enabled() {
        return;
    }

    static MASKED_SCALE_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = MASKED_SCALE_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq >= 128 && !seq.is_power_of_two() {
        return;
    }

    let state = doom_read_u32(guest_mem, 0x0010_1BB8);
    let detailshift = doom_read_u32(guest_mem, state.saturating_add(0x0B7B4));
    let raw_scale = doom_read_u32(guest_mem, esi.saturating_add(0x28));
    let stored_scale = (((raw_scale as i32).wrapping_abs() as u32) >> (detailshift & 31)) as u32;
    let fields = [
        doom_read_u32(guest_mem, esi.saturating_add(0x20)),
        doom_read_u32(guest_mem, esi.saturating_add(0x24)),
        doom_read_u32(guest_mem, esi.saturating_add(0x28)),
        doom_read_u32(guest_mem, esi.saturating_add(0x2C)),
        doom_read_u32(guest_mem, esi.saturating_add(0x30)),
        doom_read_u32(guest_mem, esi.saturating_add(0x34)),
        doom_read_u32(guest_mem, esi.saturating_add(0x38)),
    ];

    debug_log(&format!(
        "[DOOM-MASKED-SCALE-SRC] #{} {} state=0x{:08X} esi=0x{:08X} detailshift={} raw_scale=0x{:08X}/{} computed_dc_iscale=0x{:08X}/{} fields20_38=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        state,
        esi,
        detailshift,
        raw_scale,
        raw_scale as i32,
        stored_scale,
        stored_scale as i32,
        fields[0],
        fields[1],
        fields[2],
        fields[3],
        fields[4],
        fields[5],
        fields[6],
        eax,
        ecx,
        edx,
        edi
    ));
}

pub fn doom_span_call_tap(
    name: &str,
    guest_mem: *mut u8,
    eax: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
) {
    if !doom_span_probe_enabled() {
        return;
    }

    static SPAN_CALL_BEFORE_SEQ: AtomicU32 = AtomicU32::new(0);
    static SPAN_CALL_AFTER_SEQ: AtomicU32 = AtomicU32::new(0);
    let is_after = name == "DOOM_SPAN_CALL_AFTER_TAP"
        || name == "DOOM_SPAN_CALL_AFTER_RET"
        || name == "DOOM_SPAN_CALL_AFTER_RETMISS";
    let seq = if is_after {
        SPAN_CALL_AFTER_SEQ.fetch_add(1, Ordering::Relaxed)
    } else {
        SPAN_CALL_BEFORE_SEQ.fetch_add(1, Ordering::Relaxed)
    };
    if seq >= 128 && !seq.is_power_of_two() {
        return;
    }

    let curr_slot = doom_read_u32(guest_mem, 0x0010_1B9C);
    let live = doom_read_u32(guest_mem, 0x0010_1BB8);
    let draw_span_fn = doom_read_u32(guest_mem, 0x0010_1C30);
    let state = live;
    let ds_y = doom_read_u32(guest_mem, state.saturating_add(0xB740));
    let ds_x1 = doom_read_u32(guest_mem, state.saturating_add(0xB744));
    let ds_x2 = doom_read_u32(guest_mem, state.saturating_add(0xB748));
    let ds_colormap = doom_read_u32(guest_mem, state.saturating_add(0xB74C));
    let ds_xfrac = doom_read_u32(guest_mem, state.saturating_add(0xB750));
    let ds_yfrac = doom_read_u32(guest_mem, state.saturating_add(0xB754));
    let ds_xstep = doom_read_u32(guest_mem, state.saturating_add(0xB758));
    let ds_ystep = doom_read_u32(guest_mem, state.saturating_add(0xB75C));
    let ds_source = doom_read_u32(guest_mem, state.saturating_add(0xB760));
    let scaledviewwidth = doom_read_u32(guest_mem, state.saturating_add(0x94B8));
    let viewwidth = doom_read_u32(guest_mem, state.saturating_add(0x94BC));
    let viewheight = doom_read_u32(guest_mem, state.saturating_add(0x94C0));
    let detailshift = doom_read_u32(guest_mem, state.saturating_add(0xB7B4));
    let screenblocks = doom_read_u32(guest_mem, state.saturating_add(0x12988));
    let setdetail = doom_read_u32(guest_mem, state.saturating_add(0x1298C));
    let draw_column_fn = doom_read_u32(guest_mem, 0x0010_1C28);
    let drawer_mode = match draw_span_fn {
        0x0002_6B60 => "hi",
        0x0002_6C30 => "low",
        _ => "unknown",
    };

    let span_len = if ds_x2 >= ds_x1 {
        ds_x2.wrapping_sub(ds_x1).wrapping_add(1)
    } else {
        0
    };
    let ylookup_addr = if ds_y < 240 {
        state
            .saturating_add(0x94CC)
            .saturating_add(ds_y.saturating_mul(4))
    } else {
        0
    };
    let columnofs_addr = if ds_x1 < 320 {
        state
            .saturating_add(0xA1CC)
            .saturating_add(ds_x1.saturating_mul(4))
    } else {
        0
    };
    let row_ptr = if ylookup_addr != 0 {
        doom_read_u32(guest_mem, ylookup_addr)
    } else {
        0
    };
    let col_off = if columnofs_addr != 0 {
        doom_read_u32(guest_mem, columnofs_addr)
    } else {
        0
    };
    let dest = row_ptr.wrapping_add(col_off);
    let dest_label = if is_after {
        "dest_after"
    } else {
        "dest_before"
    };
    let dest_sample = doom_guest_byte_sample(guest_mem, dest, span_len.min(32) as usize);

    let target_x = if ds_x1 <= 160 && 160 <= ds_x2 {
        160
    } else if span_len != 0 {
        ds_x1.wrapping_add(span_len / 2)
    } else {
        ds_x1
    };
    let target_dest = row_ptr.wrapping_add(if target_x < 320 {
        doom_read_u32(
            guest_mem,
            state
                .saturating_add(0xA1CC)
                .saturating_add(target_x.saturating_mul(4)),
        )
    } else {
        0
    });
    let target_offset = target_x.wrapping_sub(ds_x1);
    let target_xfrac = ds_xfrac.wrapping_add(ds_xstep.wrapping_mul(target_offset));
    let target_yfrac = ds_yfrac.wrapping_add(ds_ystep.wrapping_mul(target_offset));
    let target_spot = doom_span_spot(target_xfrac, target_yfrac);
    let target_src = doom_read_u8(guest_mem, ds_source.wrapping_add(target_spot));
    let target_mapped = doom_read_u8(guest_mem, ds_colormap.wrapping_add(target_src as u32));
    let target_row = doom_guest_byte_sample(guest_mem, target_dest, 16);
    let first_samples = doom_span_sample_points(
        guest_mem,
        ds_source,
        ds_colormap,
        ds_xfrac,
        ds_yfrac,
        ds_xstep,
        ds_ystep,
        span_len.min(8),
    );
    let source16 = doom_guest_byte_sample(guest_mem, ds_source, 16);
    let colormap16 = doom_guest_byte_sample(guest_mem, ds_colormap, 16);

    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, None);
    let mut matches = Vec::new();
    for candidate in candidates.iter().take(16) {
        let visible_h = candidate.height.min(240);
        let Some(len) = candidate.width.checked_mul(visible_h) else {
            continue;
        };
        let start = candidate.screen_ptr;
        let end = start.saturating_add(len);
        if dest >= start && dest < end {
            let off = dest - start;
            let x = if candidate.width != 0 {
                off % candidate.width
            } else {
                0
            };
            let y = if candidate.width != 0 {
                off / candidate.width
            } else {
                0
            };
            matches.push(format!(
                "{} state=0x{:08X} screen=0x{:08X} off={} x={} y={} {}x{}",
                candidate.source,
                candidate.state,
                candidate.screen_ptr,
                off,
                x,
                y,
                candidate.width,
                candidate.height
            ));
        }
    }

    debug_log(&format!(
        "[DOOM-SPAN-CALL] #{} {} mode={} span_fn=0x{:08X} col_fn=0x{:08X} scaledviewwidth={} viewwidth={} viewheight={} detailshift={} setdetail={} screenblocks={} computed_dest=0x{:08X} row_ptr=0x{:08X} col_off=0x{:08X} curr_slot={} live=0x{:08X} match={} ds_y={} ds_x1={} ds_x2={} len={} ds_colormap=0x{:08X} ds_source=0x{:08X} ds_xfrac=0x{:08X} ds_yfrac=0x{:08X} ds_xstep=0x{:08X}/{} ds_ystep=0x{:08X}/{} source16={} colormap16={} first={} target_x={} target_off={} target_spot={} target_src=0x{:02X} target_mapped=0x{:02X} target_dest=0x{:08X} target_row={} {}={} regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
        seq,
        name,
        drawer_mode,
        draw_span_fn,
        draw_column_fn,
        scaledviewwidth,
        viewwidth,
        viewheight,
        detailshift,
        setdetail,
        screenblocks,
        dest,
        row_ptr,
        col_off,
        curr_slot,
        live,
        if matches.is_empty() {
            "outside_candidates".to_string()
        } else {
            matches.join(" | ")
        },
        ds_y,
        ds_x1,
        ds_x2,
        span_len,
        ds_colormap,
        ds_source,
        ds_xfrac,
        ds_yfrac,
        ds_xstep,
        ds_xstep as i32,
        ds_ystep,
        ds_ystep as i32,
        source16,
        colormap16,
        first_samples,
        target_x,
        target_offset,
        target_spot,
        target_src,
        target_mapped,
        target_dest,
        target_row,
        dest_label,
        dest_sample,
        eax,
        ecx,
        edx,
        esi,
        edi
    ));
}

fn doom_span_spot(xfrac: u32, yfrac: u32) -> u32 {
    ((yfrac >> 10) & 0x0FC0).wrapping_add((xfrac >> 16) & 0x003F)
}

fn doom_span_sample_points(
    guest_mem: *mut u8,
    ds_source: u32,
    ds_colormap: u32,
    mut xfrac: u32,
    mut yfrac: u32,
    xstep: u32,
    ystep: u32,
    count: u32,
) -> String {
    if count == 0 {
        return "empty".to_string();
    }
    let mut parts = Vec::new();
    for i in 0..count {
        let spot = doom_span_spot(xfrac, yfrac);
        let src = doom_read_u8(guest_mem, ds_source.wrapping_add(spot));
        let mapped = doom_read_u8(guest_mem, ds_colormap.wrapping_add(src as u32));
        parts.push(format!("{}:{}:{:02X}>{:02X}", i, spot, src, mapped));
        xfrac = xfrac.wrapping_add(xstep);
        yfrac = yfrac.wrapping_add(ystep);
    }
    parts.join(",")
}

fn doom_column_source_at_frac_sample(
    guest_mem: *mut u8,
    dc_source: u32,
    dc_texturemid: u32,
    dc_yl: u32,
    dc_iscale: u32,
) -> (u32, u32, u32, String) {
    const CENTER_Y: i32 = 84;
    let dy = (dc_yl as i32).wrapping_sub(CENTER_Y) as u32;
    let frac0 = dc_texturemid.wrapping_add(dy.wrapping_mul(dc_iscale));
    let src_idx_raw = frac0 >> 16;
    let src_idx = src_idx_raw & 0x7F;
    let sample = doom_guest_byte_sample(guest_mem, dc_source.wrapping_add(src_idx), 16);
    (frac0, src_idx_raw, src_idx, sample)
}

fn doom_guest_byte_sample(guest_mem: *mut u8, base: u32, len: usize) -> String {
    match doom_read_guest_bytes(guest_mem, base, len) {
        Some(bytes) => bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<_>>()
            .join(""),
        None => "unreadable".to_string(),
    }
}

fn doom_read_guest_bytes(guest_mem: *mut u8, base: u32, len: usize) -> Option<Vec<u8>> {
    if base == 0 || len == 0 || len > 16 * 1024 * 1024 {
        return None;
    }
    if (base as u64).saturating_add(len as u64) > 0x2000_0000 {
        return None;
    }
    unsafe { Some(std::slice::from_raw_parts(guest_mem.add(base as usize), len).to_vec()) }
}

fn doom_wad_lump_name(name: &str) -> [u8; 8] {
    let mut out = [0u8; 8];
    for (i, b) in name.bytes().take(8).enumerate() {
        out[i] = b.to_ascii_uppercase();
    }
    out
}

fn doom_load_host_wad_lump(name: &str) -> Option<Vec<u8>> {
    let wanted = doom_wad_lump_name(name);
    let paths = [
        r"./games/doom3/base\classic\w\DOOM.WAD",
        r"./games/doom3/base\classic\w\DOOM2.WAD",
        r"./games/doom3/base\classic\w\DOOMU.WAD",
        r"./games/doom3/base\classic\w\DOOM1.WAD",
        r"./games/doom3/base\classic\w\TNT.WAD",
        r"./games/doom3/base\classic\w\PLUTONIA.WAD",
        r"./games/doom3/base\classic\w\VV.WAD",
    ];

    for path in paths {
        let Ok(data) = std::fs::read(path) else {
            continue;
        };
        if data.len() < 12 {
            continue;
        }
        let lump_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let dir_off = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        let Some(dir_len) = lump_count.checked_mul(16) else {
            continue;
        };
        if dir_off
            .checked_add(dir_len)
            .map(|end| end > data.len())
            .unwrap_or(true)
        {
            continue;
        }
        for i in 0..lump_count {
            let off = dir_off + i * 16;
            if data[off + 8..off + 16] != wanted {
                continue;
            }
            let lump_off =
                u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
                    as usize;
            let lump_size =
                u32::from_le_bytes([data[off + 4], data[off + 5], data[off + 6], data[off + 7]])
                    as usize;
            if lump_off
                .checked_add(lump_size)
                .map(|end| end <= data.len())
                .unwrap_or(false)
            {
                return Some(data[lump_off..lump_off + lump_size].to_vec());
            }
        }
    }

    None
}

fn doom_colormap_stats(bytes: &[u8]) -> (usize, usize, String) {
    if bytes.len() < 256 {
        return (0, 0, "-".to_string());
    }
    let row0 = &bytes[..256];
    let identity = row0
        .iter()
        .enumerate()
        .filter(|(i, b)| **b == *i as u8)
        .count();
    let mut seen = [false; 256];
    for &b in row0 {
        seen[b as usize] = true;
    }
    let unique = seen.iter().filter(|v| **v).count();
    let first16 = row0
        .iter()
        .take(16)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("");
    (identity, unique, first16)
}

fn doom_lumpinfo_probe(guest_mem: *mut u8, name: &str) -> String {
    let wanted = doom_wad_lump_name(name);
    let numlumps = doom_read_u32(guest_mem, 0x0010_375C);
    let lumpinfo = doom_read_u32(guest_mem, 0x0010_3768);
    if numlumps == 0 || numlumps > 100_000 || lumpinfo == 0 || lumpinfo >= 0x1000_0000 {
        return format!(
            "lumpinfo[{}]=unavailable numlumps={} table=0x{:08X}",
            name, numlumps, lumpinfo
        );
    }

    for i in (0..numlumps).rev() {
        let entry = lumpinfo.wrapping_add(i.saturating_mul(20));
        if entry >= 0x2000_0000u32.saturating_sub(20) {
            continue;
        }
        let mut lump_name = [0u8; 8];
        unsafe {
            lump_name.copy_from_slice(std::slice::from_raw_parts(guest_mem.add(entry as usize), 8));
        }
        if lump_name != wanted {
            continue;
        }
        let f8 = doom_read_u32(guest_mem, entry + 8);
        let f12 = doom_read_u32(guest_mem, entry + 12);
        let f16 = doom_read_u32(guest_mem, entry + 16);
        return format!(
            "lumpinfo[{}] index={} entry=0x{:08X} f8=0x{:08X}/{} f12=0x{:08X}/{} f16=0x{:08X}/{}",
            name, i, entry, f8, f8, f12, f12, f16, f16
        );
    }

    format!(
        "lumpinfo[{}]=missing numlumps={} table=0x{:08X}",
        name, numlumps, lumpinfo
    )
}

fn doom_colormap_candidate_line(
    guest_mem: *mut u8,
    seq: u32,
    label: &str,
    base: u32,
    host_colormap: Option<&[u8]>,
) -> (String, bool, Option<usize>) {
    const COLORMAP_W: u32 = 256;
    const COLORMAP_H: u32 = 34;
    const COLORMAP_LEN: usize = (COLORMAP_W as usize) * (COLORMAP_H as usize);

    let Some(bytes) = doom_read_guest_bytes(guest_mem, base, COLORMAP_LEN) else {
        return (format!("{}=0x{:08X} unavailable", label, base), false, None);
    };
    let (identity, unique, first16) = doom_colormap_stats(&bytes);
    let dump_label = format!("colormap_live_{}", label);
    let dump = doom_dump_l8_bytes(seq, &dump_label, &bytes, COLORMAP_W, COLORMAP_H);
    let (exact, diff_count) = host_colormap
        .filter(|host| host.len() >= COLORMAP_LEN)
        .map(|host| {
            let host = &host[..COLORMAP_LEN];
            let diff = bytes
                .iter()
                .zip(host.iter())
                .filter(|(a, b)| a != b)
                .count();
            (diff == 0, Some(diff))
        })
        .unwrap_or((false, None));

    (
        format!(
            "{}=0x{:08X} hash=0x{:016X} row0_identity={}/256 row0_unique={} row0_first16={} exact={} diff={} {}",
            label,
            base,
            fnv1a64(&bytes),
            identity,
            unique,
            first16,
            exact as u8,
            diff_count.unwrap_or(0),
            dump
        ),
        exact,
        diff_count,
    )
}

fn doom_scan_guest_for_exact_bytes(guest_mem: *mut u8, needle: &[u8]) -> Option<u32> {
    if needle.len() < 32 {
        return None;
    }

    const RANGES: &[(u32, u32)] = &[
        (0x0400_0000, 0x0500_0000),
        (0x0100_0000, 0x0400_0000),
        (0x0500_0000, 0x0800_0000),
        (0x0001_0000, 0x0100_0000),
        (0x0800_0000, 0x1000_0000),
    ];

    let prefix = &needle[..32];
    let first = prefix[0];
    for &(start, end) in RANGES {
        let len = end.saturating_sub(start) as usize;
        if len < needle.len() {
            continue;
        }
        let hay = unsafe { std::slice::from_raw_parts(guest_mem.add(start as usize), len) };
        let mut pos = 0usize;
        let max = len - needle.len();
        while pos <= max {
            if hay[pos] == first
                && &hay[pos..pos + prefix.len()] == prefix
                && &hay[pos..pos + needle.len()] == needle
            {
                return Some(start.wrapping_add(pos as u32));
            }
            pos += 1;
        }
    }

    None
}

fn doom_colormap_scan_line(
    guest_mem: *mut u8,
    seq: u32,
    host_colormap: &[u8],
) -> (String, bool, Option<usize>) {
    const COLORMAP_W: u32 = 256;
    const COLORMAP_H: u32 = 34;
    const COLORMAP_LEN: usize = (COLORMAP_W as usize) * (COLORMAP_H as usize);

    static SCAN_RESULT: OnceLock<Option<u32>> = OnceLock::new();
    let Some(host) = host_colormap.get(..COLORMAP_LEN) else {
        return (
            format!("guest_colormap_scan=bad_host_len{}", host_colormap.len()),
            false,
            None,
        );
    };
    let found = *SCAN_RESULT.get_or_init(|| doom_scan_guest_for_exact_bytes(guest_mem, host));
    let Some(base) = found else {
        return (
            "guest_colormap_scan=not_found ranges=00010000..10000000".to_string(),
            false,
            None,
        );
    };

    doom_colormap_candidate_line(guest_mem, seq, "scan_exact", base, Some(host))
}

fn doom_dump_colormap_diag(guest_mem: *mut u8, seq: u32) -> String {
    const PLAYPAL_RGB_LEN: usize = 256 * 3;
    const PLAYPAL_ARGB_LEN: usize = 256 * 4;
    const COLORMAP_W: u32 = 256;
    const COLORMAP_H: u32 = 34;
    const COLORMAP_LEN: usize = (COLORMAP_W as usize) * (COLORMAP_H as usize);

    let state = doom_screen_source_probe(guest_mem)
        .map(|(state, _, _, _, _, _, _)| state)
        .unwrap_or_else(|| crate::xbox::worker::DOOM_GAME_STATE_CACHE.load(Ordering::Relaxed));
    if state == 0 || state >= 0x0800_0000 {
        return "colormap=state-unavailable".to_string();
    }

    let playpal_guess = state.wrapping_add(0x396C);
    let colormap_after_rgb_guess = playpal_guess.wrapping_add(PLAYPAL_RGB_LEN as u32);
    let colormap_after_argb_guess = playpal_guess.wrapping_add(PLAYPAL_ARGB_LEN as u32);
    let live_playpal_rgb = doom_read_guest_bytes(guest_mem, playpal_guess, PLAYPAL_RGB_LEN);
    let live_playpal_argb = doom_read_guest_bytes(guest_mem, playpal_guess, PLAYPAL_ARGB_LEN);
    let host_colormap = doom_load_host_wad_lump("COLORMAP");

    let playpal_line = live_playpal_rgb
        .as_ref()
        .map(|bytes| {
            let argb_first16 = live_playpal_argb
                .as_ref()
                .map(|argb| {
                    argb.iter()
                        .take(16)
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .unwrap_or_else(|| "-".to_string());
            format!(
                "playpal_guess=0x{:08X} rgb_hash=0x{:016X} rgb_nonzero={} argb_hash=0x{:016X} argb_first16={}",
                playpal_guess,
                fnv1a64(bytes),
                bytes.iter().filter(|b| **b != 0).count(),
                live_playpal_argb
                    .as_ref()
                    .map(|argb| fnv1a64(argb))
                    .unwrap_or(0),
                argb_first16
            )
        })
        .unwrap_or_else(|| format!("playpal_guess=0x{:08X} unavailable", playpal_guess));

    let host_line = host_colormap
        .as_ref()
        .filter(|bytes| bytes.len() >= COLORMAP_LEN)
        .map(|bytes| {
            let bytes = &bytes[..COLORMAP_LEN];
            let (identity, unique, first16) = doom_colormap_stats(bytes);
            let dump = doom_dump_l8_bytes(seq, "colormap_host_wad", bytes, COLORMAP_W, COLORMAP_H);
            format!(
                "host_colormap hash=0x{:016X} row0_identity={}/256 row0_unique={} row0_first16={} {}",
                fnv1a64(bytes),
                identity,
                unique,
                first16,
                dump
            )
        })
        .unwrap_or_else(|| {
            host_colormap
                .as_ref()
                .map(|bytes| format!("host_colormap bad_len={}", bytes.len()))
                .unwrap_or_else(|| "host_colormap=unavailable".to_string())
        });

    let host_ref = host_colormap
        .as_ref()
        .filter(|bytes| bytes.len() >= COLORMAP_LEN)
        .map(|bytes| &bytes[..COLORMAP_LEN]);
    let (after_rgb_line, after_rgb_exact, after_rgb_diff) = doom_colormap_candidate_line(
        guest_mem,
        seq,
        "after_rgb_300",
        colormap_after_rgb_guess,
        host_ref,
    );
    let (after_argb_line, after_argb_exact, after_argb_diff) = doom_colormap_candidate_line(
        guest_mem,
        seq,
        "after_argb_400",
        colormap_after_argb_guess,
        host_ref,
    );
    let (scan_line, scan_exact, scan_diff) = host_ref
        .map(|host| doom_colormap_scan_line(guest_mem, seq, host))
        .unwrap_or_else(|| {
            (
                "guest_colormap_scan=host_unavailable".to_string(),
                false,
                None,
            )
        });
    let exact_match = after_rgb_exact || after_argb_exact || scan_exact;
    let diff_count = after_rgb_diff
        .into_iter()
        .chain(after_argb_diff)
        .chain(scan_diff)
        .min()
        .unwrap_or(0);

    format!(
        "state=0x{:08X} {} | {} | {} | {} | {} | exact_match={} best_diff={} | {} | {}",
        state,
        playpal_line,
        after_rgb_line,
        after_argb_line,
        host_line,
        scan_line,
        exact_match as u8,
        diff_count,
        doom_lumpinfo_probe(guest_mem, "PLAYPAL"),
        doom_lumpinfo_probe(guest_mem, "COLORMAP")
    )
}

fn doom_blit_dump_selected_for_swizzle(guest_mem: *mut u8, seq: u32, stage_src: u32) -> bool {
    if doom_blit_dump_seq_selected(seq) {
        return true;
    }

    static LAST_SCREEN_HASH: AtomicU64 = AtomicU64::new(0);
    static HASH_CHANGE_DUMPS: AtomicU32 = AtomicU32::new(0);

    let candidates =
        crate::xbox::worker::doom_framebuffer_candidates_from_guest_mem(guest_mem, Some(stage_src));
    let Some(candidate) = candidates.first() else {
        return false;
    };
    let Some((hash, nonzero)) = doom_hash_guest_rows(
        guest_mem,
        candidate.screen_ptr,
        candidate.width,
        candidate.width,
        candidate.height.min(240),
    ) else {
        return false;
    };

    let prev = LAST_SCREEN_HASH.swap(hash, Ordering::Relaxed);
    if prev == 0 || prev == hash {
        return false;
    }

    let n = HASH_CHANGE_DUMPS.fetch_add(1, Ordering::Relaxed);
    let selected = n < 64 || n.is_power_of_two();
    debug_log(&format!(
        "[DOOM-HASH-CHANGE-DUMP] #{} seq={} selected={} prev=0x{:016X} now=0x{:016X} nonzero={} state=0x{:08X} screen=0x{:08X} {}x{} source={} stage_src=0x{:08X}",
        n,
        seq,
        if selected { 1 } else { 0 },
        prev,
        hash,
        nonzero,
        candidate.state,
        candidate.screen_ptr,
        candidate.width,
        candidate.height,
        candidate.source,
        stage_src
    ));
    selected
}

fn doom_dump_swizzle_staging_views(guest_mem: *mut u8, swizzle: DoomBlitSwizzleInfo) {
    if !doom_blit_dump_enabled()
        || swizzle.bpp != 4
        || !doom_blit_dump_selected_for_swizzle(guest_mem, swizzle.seq, swizzle.src_base)
    {
        return;
    }

    let format_code = crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8;
    let doom_pitch = 320u32.saturating_mul(swizzle.bpp);
    let screen = doom_screen_source_probe(guest_mem)
        .map(|(_, screen, w, h, source, _, _)| {
            doom_dump_l8_view(guest_mem, swizzle.seq, screen, source, w, h.min(240), w)
        })
        .unwrap_or_else(|| "screen=unavailable".to_string());
    let screen_candidates = doom_dump_l8_candidate_views(guest_mem, swizzle.seq, swizzle.src_base);
    let resolved = doom_dump_staging_view(
        guest_mem,
        swizzle.seq,
        swizzle.src_base,
        "xg_full",
        swizzle.width,
        swizzle.height,
        swizzle.pitch,
        format_code,
    );
    let left_320_200_pitch_xg = doom_dump_staging_view(
        guest_mem,
        swizzle.seq,
        swizzle.src_base,
        "left320x200_pitch_xg",
        320,
        200,
        swizzle.pitch,
        format_code,
    );
    let left_320_168_pitch_xg = doom_dump_staging_view(
        guest_mem,
        swizzle.seq,
        swizzle.src_base,
        "left320x168_pitch_xg",
        320,
        168,
        swizzle.pitch,
        format_code,
    );
    let linear_320_200 = doom_dump_staging_view(
        guest_mem,
        swizzle.seq,
        swizzle.src_base,
        "linear320x200_pitch1280",
        320,
        200,
        doom_pitch,
        format_code,
    );
    let linear_320_168 = doom_dump_staging_view(
        guest_mem,
        swizzle.seq,
        swizzle.src_base,
        "linear320x168_pitch1280",
        320,
        168,
        doom_pitch,
        format_code,
    );
    let colormap = doom_dump_colormap_diag(guest_mem, swizzle.seq);

    debug_log(&format!(
        "[DOOM-BLIT-STAGING-DUMP] seq={} src=0x{:08X} bpp={} xg={}x{} pitch={} doom_pitch={} | {} | {} | {} | {} | {} | {} | {} | {}",
        swizzle.seq,
        swizzle.src_base,
        swizzle.bpp,
        swizzle.width,
        swizzle.height,
        swizzle.pitch,
        doom_pitch,
        screen,
        screen_candidates,
        resolved,
        left_320_200_pitch_xg,
        left_320_168_pitch_xg,
        linear_320_200,
        linear_320_168,
        colormap
    ));
}

fn doom_dump_upload_argb(
    stage: u32,
    tex_ptr: u32,
    data_addr: u32,
    width: u32,
    height: u32,
    format_code: u32,
    pixels: &[u32],
) {
    if !doom_blit_dump_enabled()
        || width != 512
        || height != 256
        || format_code != crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
    {
        return;
    }

    static UPLOAD_DUMP_SEQ: AtomicU32 = AtomicU32::new(0);
    let seq = UPLOAD_DUMP_SEQ.fetch_add(1, Ordering::Relaxed);
    if !doom_blit_dump_seq_selected(seq) {
        return;
    }

    let full_hash = doom_argb_hash(pixels);
    let full_nonzero = doom_argb_nonzero(pixels);
    let full_path = format!(
        r"./doom_upload_argb_{:03}_tex{:08X}_{}x{}.bmp",
        seq, tex_ptr, width, height
    );
    write_readback_bmp(&full_path, width, height, pixels);

    let crop_320_200 = doom_crop_argb(pixels, width, height, 320, 200);
    let crop_320_168 = doom_crop_argb(pixels, width, height, 320, 168);
    let crop_320_200_line = if let Some(crop) = crop_320_200 {
        let hash = doom_argb_hash(&crop);
        let nonzero = doom_argb_nonzero(&crop);
        let path = format!(
            r"./doom_upload_argb_{:03}_tex{:08X}_crop320x200.bmp",
            seq, tex_ptr
        );
        write_readback_bmp(&path, 320, 200, &crop);
        format!(
            "crop320x200 hash=0x{:016X} nonzero={} path={}",
            hash, nonzero, path
        )
    } else {
        "crop320x200 unavailable".to_string()
    };
    let crop_320_168_line = if let Some(crop) = crop_320_168 {
        let hash = doom_argb_hash(&crop);
        let nonzero = doom_argb_nonzero(&crop);
        let path = format!(
            r"./doom_upload_argb_{:03}_tex{:08X}_crop320x168.bmp",
            seq, tex_ptr
        );
        write_readback_bmp(&path, 320, 168, &crop);
        format!(
            "crop320x168 hash=0x{:016X} nonzero={} path={}",
            hash, nonzero, path
        )
    } else {
        "crop320x168 unavailable".to_string()
    };

    debug_log(&format!(
        "[DOOM-BLIT-UPLOAD-DUMP] seq={} stage={} tex=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X}/{} full_hash=0x{:016X} full_nonzero={} full_path={} | {} | {}",
        seq,
        stage,
        tex_ptr,
        data_addr,
        width,
        height,
        format_code,
        crate::xbox::gpu::texture_format::format_name(format_code),
        full_hash,
        full_nonzero,
        full_path,
        crop_320_200_line,
        crop_320_168_line
    ));
}

fn doom_dump_swizzle_bisect(
    stage: u32,
    tex_ptr_raw: u32,
    tex_ptr: u32,
    data_addr: u32,
    width: u32,
    height: u32,
    format_code: u32,
    bpp: u32,
    swizzled_bytes: &[u8],
) {
    if !doom_swizzle_bisect_enabled()
        || stage != 0
        || width != 512
        || height != 256
        || format_code != crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
        || bpp != 4
    {
        return;
    }

    static DID_DUMP: AtomicBool = AtomicBool::new(false);
    if DID_DUMP.swap(true, Ordering::AcqRel) {
        return;
    }

    let base_path = format!(
        r"./doom_swizzle_bisect_tex{:08X}_src{:08X}_{}x{}",
        tex_ptr, data_addr, width, height
    );
    let raw_path = format!("{}_swizzled.bin", base_path);
    let raw_result = std::fs::write(&raw_path, swizzled_bytes);

    let Some(linear_bytes) =
        crate::xbox::gpu::swizzle::unswizzle(swizzled_bytes, width, height, bpp)
    else {
        debug_log(&format!(
            "[DOOM-SWIZZLE-BISECT] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X} bpp={} raw_path={} raw_write={:?} unswizzle=failed",
            stage,
            tex_ptr_raw,
            tex_ptr,
            data_addr,
            width,
            height,
            format_code,
            bpp,
            raw_path,
            raw_result.as_ref().map(|_| ())
        ));
        return;
    };

    let Some(pixels) = crate::xbox::gpu::texture_format::decode_linear_to_argb(
        format_code,
        width,
        height,
        &linear_bytes,
        None,
    ) else {
        debug_log(&format!(
            "[DOOM-SWIZZLE-BISECT] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X} bpp={} raw_path={} raw_write={:?} decode=failed",
            stage,
            tex_ptr_raw,
            tex_ptr,
            data_addr,
            width,
            height,
            format_code,
            bpp,
            raw_path,
            raw_result.as_ref().map(|_| ())
        ));
        return;
    };

    let full_path = format!("{}_verified_unswizzle.bmp", base_path);
    write_readback_bmp(&full_path, width, height, &pixels);

    let crop_320_200_path = format!("{}_verified_unswizzle_crop320x200.bmp", base_path);
    let crop_320_200_line = if let Some(crop) = doom_crop_argb(&pixels, width, height, 320, 200) {
        write_readback_bmp(&crop_320_200_path, 320, 200, &crop);
        format!(
            "crop320x200 hash=0x{:016X} nonzero={} path={}",
            doom_argb_hash(&crop),
            doom_argb_nonzero(&crop),
            crop_320_200_path
        )
    } else {
        "crop320x200 unavailable".to_string()
    };

    let crop_320_168_path = format!("{}_verified_unswizzle_crop320x168.bmp", base_path);
    let crop_320_168_line = if let Some(crop) = doom_crop_argb(&pixels, width, height, 320, 168) {
        write_readback_bmp(&crop_320_168_path, 320, 168, &crop);
        format!(
            "crop320x168 hash=0x{:016X} nonzero={} path={}",
            doom_argb_hash(&crop),
            doom_argb_nonzero(&crop),
            crop_320_168_path
        )
    } else {
        "crop320x168 unavailable".to_string()
    };

    debug_log(&format!(
        "[DOOM-SWIZZLE-BISECT] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X}/{} bpp={} raw_bytes={} raw_hash=0x{:016X} linear_hash=0x{:016X} argb_hash=0x{:016X} argb_nonzero={} raw_path={} raw_write={:?} full_path={} | {} | {}",
        stage,
        tex_ptr_raw,
        tex_ptr,
        data_addr,
        width,
        height,
        format_code,
        crate::xbox::gpu::texture_format::format_name(format_code),
        bpp,
        swizzled_bytes.len(),
        fnv1a64(swizzled_bytes),
        fnv1a64(&linear_bytes),
        doom_argb_hash(&pixels),
        doom_argb_nonzero(&pixels),
        raw_path,
        raw_result.as_ref().map(|_| ()),
        full_path,
        crop_320_200_line,
        crop_320_168_line
    ));
}

fn doom_decode_texture_argb_at_guest_memory(
    guest_mem: *mut u8,
    info: HleTextureInfo,
) -> Option<(Vec<u32>, u32, u32, u32, u32)> {
    if guest_mem.is_null() || info.width == 0 || info.height == 0 {
        return None;
    }

    let data_addr = normalize_guest_ram_ptr(info.data & !0x3)?;
    let fmt_code = xbox_texture_format_code(info.format);

    if crate::xbox::gpu::texture_format::is_block_compressed(fmt_code) {
        let bytes_per_block = crate::xbox::gpu::texture_format::block_bytes(fmt_code)?;
        let block_width = (info.width + 3) / 4;
        let block_rows = (info.height + 3) / 4;
        let row_pitch = block_width.saturating_mul(bytes_per_block);
        let byte_len = row_pitch.saturating_mul(block_rows);
        let byte_end = data_addr as usize + byte_len as usize;
        if byte_len == 0 || byte_len > 16 * 1024 * 1024 || byte_end > 0x2000_0000 {
            return None;
        }
        let bytes = unsafe {
            std::slice::from_raw_parts(guest_mem.add(data_addr as usize), byte_len as usize)
        };
        let pixels = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
            fmt_code,
            info.width,
            info.height,
            bytes,
        )?;
        return Some((pixels, fmt_code, data_addr, row_pitch, byte_len));
    }

    let bpp = texture_bytes_per_pixel(fmt_code)?;
    let row_bytes = info.width.checked_mul(bpp)?;
    let swizzled = crate::xbox::gpu::texture_format::is_swizzled(fmt_code)
        && info.width.is_power_of_two()
        && info.height.is_power_of_two();
    let byte_len = if swizzled {
        (info.width as usize)
            .checked_mul(info.height as usize)?
            .checked_mul(bpp as usize)?
    } else {
        let pitch = info.pitch.max(row_bytes);
        (info.height.saturating_sub(1) as usize)
            .checked_mul(pitch as usize)?
            .checked_add(row_bytes as usize)?
    };
    if byte_len == 0 || byte_len > 16 * 1024 * 1024 {
        return None;
    }
    let byte_end = data_addr as usize + byte_len;
    if byte_end > 0x2000_0000 {
        return None;
    }

    let mut linear_bytes = vec![
        0u8;
        (info.width as usize)
            .checked_mul(info.height as usize)?
            .checked_mul(bpp as usize)?
    ];
    unsafe {
        if swizzled {
            let swizzled_bytes =
                std::slice::from_raw_parts(guest_mem.add(data_addr as usize), byte_len);
            if let Some(unswizzled) =
                crate::xbox::gpu::swizzle::unswizzle(swizzled_bytes, info.width, info.height, bpp)
            {
                linear_bytes = unswizzled;
            } else {
                linear_bytes.copy_from_slice(swizzled_bytes);
            }
        } else {
            let pitch = info.pitch.max(row_bytes);
            for y in 0..info.height as usize {
                let src = std::slice::from_raw_parts(
                    guest_mem.add(data_addr as usize + y * pitch as usize),
                    row_bytes as usize,
                );
                let dst = y * row_bytes as usize;
                linear_bytes[dst..dst + row_bytes as usize].copy_from_slice(src);
            }
        }
    }

    let pixels = crate::xbox::gpu::texture_format::decode_linear_to_argb(
        fmt_code,
        info.width,
        info.height,
        &linear_bytes,
        None,
    )?;
    Some((pixels, fmt_code, data_addr, row_bytes, byte_len as u32))
}

fn doom_dump_bound_texture_at_swap(
    guest_mem: *mut u8,
    swap_seq: u32,
    swap_flags: u32,
    active_tex0: u32,
    lock: Option<DoomBlitLockInfo>,
    upload: Option<DoomBlitUploadInfo>,
    draw: Option<DoomBlitDrawInfo>,
) {
    if !doom_blit_dump_enabled() || !doom_blit_dump_seq_selected(swap_seq) || active_tex0 == 0 {
        return;
    }

    let Some(info) = lookup_texture_info(active_tex0) else {
        debug_log(&format!(
            "[DOOM-BLIT-SWAP-TEX] swap={} flags=0x{:08X} tex=0x{:08X} missing-registry upload_seq={}",
            swap_seq,
            swap_flags,
            active_tex0,
            upload.map(|u| u.seq).unwrap_or(0)
        ));
        return;
    };

    let Some((pixels, fmt_code, data_addr, row_pitch, byte_len)) =
        doom_decode_texture_argb_at_guest_memory(guest_mem, info)
    else {
        debug_log(&format!(
            "[DOOM-BLIT-SWAP-TEX] swap={} flags=0x{:08X} tex=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X}/0x{:02X}/{} decode-failed upload_seq={}",
            swap_seq,
            swap_flags,
            active_tex0,
            info.data,
            info.width,
            info.height,
            info.format,
            xbox_texture_format_code(info.format),
            crate::xbox::gpu::texture_format::format_name(xbox_texture_format_code(info.format)),
            upload.map(|u| u.seq).unwrap_or(0)
        ));
        return;
    };

    let upload_seq = upload.map(|u| u.seq).unwrap_or(0);
    let full_hash = doom_argb_hash(&pixels);
    let full_nonzero = doom_argb_nonzero(&pixels);
    let base_path = format!(
        r"./doom_draw_texture_swap{:04}_upload{:04}_tex{:08X}_{}x{}",
        swap_seq, upload_seq, active_tex0, info.width, info.height
    );
    let full_path = format!("{}.bmp", base_path);
    write_readback_bmp(&full_path, info.width, info.height, &pixels);

    let sampled_w = lock
        .map(|l| doom_visible_dims(l, info.width, info.height).0)
        .or_else(|| upload.map(|u| u.visible_w))
        .unwrap_or(info.width.min(320))
        .min(info.width);
    let sampled_h = lock
        .map(|l| doom_visible_dims(l, info.width, info.height).1)
        .or_else(|| upload.map(|u| u.visible_h))
        .unwrap_or(info.height.min(200))
        .min(info.height);
    let crop_line = if let Some(crop) =
        doom_crop_argb(&pixels, info.width, info.height, sampled_w, sampled_h)
    {
        let hash = doom_argb_hash(&crop);
        let nonzero = doom_argb_nonzero(&crop);
        let path = format!("{}_crop{}x{}.bmp", base_path, sampled_w, sampled_h);
        write_readback_bmp(&path, sampled_w, sampled_h, &crop);
        format!(
            "crop={}x{} hash=0x{:016X} nonzero={} path={}",
            sampled_w, sampled_h, hash, nonzero, path
        )
    } else {
        format!("crop={}x{} unavailable", sampled_w, sampled_h)
    };

    let source_line = if let Some(lock) = lock {
        if lock.source_screen != 0 && lock.source_w != 0 && lock.source_h != 0 {
            doom_dump_l8_view(
                guest_mem,
                swap_seq,
                lock.source_screen,
                "screens0_at_swap",
                lock.source_w.min(320),
                lock.source_h.min(240),
                lock.source_w,
            )
        } else {
            "screens0_at_swap=unavailable".to_string()
        }
    } else {
        "screens0_at_swap=no-lock".to_string()
    };

    let draw = draw.unwrap_or_default();
    let bpp = texture_bytes_per_pixel(fmt_code).unwrap_or(0);
    let host_addr = (guest_mem as usize).saturating_add(data_addr as usize);
    let mut vertices = String::new();
    for i in 0..draw.vertex_sample_count.min(draw.vertices.len() as u32) as usize {
        let v = draw.vertices[i];
        vertices.push_str(&format!(
            "v{}=pos({:.6},{:.6},{:.6},{:.6}) uv({:.6},{:.6}) color=0x{:08X}\n",
            i, v.x, v.y, v.z, v.w, v.u, v.v, v.color
        ));
    }
    let meta_path = format!("{}.meta.txt", base_path);
    let meta = format!(
        concat!(
            "swap_seq={}\n",
            "swap_flags=0x{:08X}\n",
            "upload_seq={}\n",
            "lock_seq={}\n",
            "draw_seq={}\n",
            "guest_texture=0x{:08X}\n",
            "guest_data=0x{:08X}\n",
            "host_data=0x{:016X}\n",
            "format=0x{:08X}\n",
            "format_code=0x{:02X}\n",
            "format_name={}\n",
            "width={}\n",
            "height={}\n",
            "bpp={}\n",
            "pitch={}\n",
            "byte_len={}\n",
            "full_hash=0x{:016X}\n",
            "full_nonzero={}\n",
            "sampled_crop={}x{}\n",
            "source_dump={}\n",
            "primitive={}\n",
            "vertex_count={}\n",
            "vertex_stride={}\n",
            "vertex_data=0x{:08X}\n",
            "refreshed={}\n",
            "draw_active_tex0=0x{:08X}\n",
            "pixel_shader_handle=0x{:08X}\n",
            "pixel_shader_seq={}\n",
            "pixel_shader_combiner=0x{:08X}\n",
            "pixel_shader_rgb0=0x{:08X}\n",
            "pixel_shader_alpha0=0x{:08X}\n",
            "pixel_shader_texmodes=0x{:08X}\n",
            "sampler_stage0_addressu={}\n",
            "sampler_stage0_addressv={}\n",
            "sampler_stage0_addressw={}\n",
            "sampler_stage0_magfilter={}\n",
            "sampler_stage0_minfilter={}\n",
            "sampler_stage0_mipfilter={}\n",
            "stage0_colorop={}\n",
            "stage0_alphaop={}\n",
            "{}"
        ),
        swap_seq,
        swap_flags,
        upload_seq,
        lock.map(|l| l.seq).unwrap_or(0),
        draw.seq,
        active_tex0,
        data_addr,
        host_addr,
        info.format,
        fmt_code,
        crate::xbox::gpu::texture_format::format_name(fmt_code),
        info.width,
        info.height,
        bpp,
        row_pitch,
        byte_len,
        full_hash,
        full_nonzero,
        sampled_w,
        sampled_h,
        source_line,
        draw.prim_type,
        draw.vertex_count,
        draw.stride,
        draw.vertex_data_ptr,
        draw.refreshed,
        draw.active_tex0,
        draw.ps_handle,
        draw.ps.seq,
        draw.ps.combiner_count,
        draw.ps.rgb_inputs0,
        draw.ps.alpha_inputs0,
        draw.ps.texture_modes,
        draw.stage0[crate::xbox::gpu::X_D3DTSS_ADDRESSU],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_ADDRESSV],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_ADDRESSW],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_MAGFILTER],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_MINFILTER],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_MIPFILTER],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_COLOROP],
        draw.stage0[crate::xbox::gpu::X_D3DTSS_ALPHAOP],
        vertices
    );
    if let Err(e) = std::fs::write(&meta_path, meta) {
        debug_log(&format!(
            "[DOOM-BLIT-SWAP-TEX] swap={} meta-write-failed path={} err={}",
            swap_seq, meta_path, e
        ));
    }

    debug_log(&format!(
        "[DOOM-BLIT-SWAP-TEX] swap={} flags=0x{:08X} tex=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X}/0x{:02X}/{} row_pitch={} bytes={} upload_seq={} lock_seq={} draw_seq={} ps=0x{:08X} full_hash=0x{:016X} full_nonzero={} full_path={} meta={} | {} | {}",
        swap_seq,
        swap_flags,
        active_tex0,
        data_addr,
        info.width,
        info.height,
        info.format,
        fmt_code,
        crate::xbox::gpu::texture_format::format_name(fmt_code),
        row_pitch,
        byte_len,
        upload_seq,
        lock.map(|l| l.seq).unwrap_or(0),
        draw.seq,
        draw.ps_handle,
        full_hash,
        full_nonzero,
        full_path,
        meta_path,
        crop_line,
        source_line
    ));
}

fn doom_read_u32(guest_mem: *mut u8, addr: u32) -> u32 {
    if addr > 0x2000_0000u32.saturating_sub(4) {
        return 0;
    }
    unsafe { std::ptr::read_unaligned(guest_mem.add(addr as usize) as *const u32) }
}

fn doom_read_u8(guest_mem: *mut u8, addr: u32) -> u8 {
    if addr >= 0x2000_0000 {
        return 0;
    }
    unsafe { *guest_mem.add(addr as usize) }
}

fn doom_hash_guest_rows(
    guest_mem: *mut u8,
    base: u32,
    pitch: u32,
    row_bytes: u32,
    rows: u32,
) -> Option<(u64, usize)> {
    doom_hash_guest_rows_from(guest_mem, base, pitch, 0, row_bytes, rows)
}

fn doom_hash_guest_rows_from(
    guest_mem: *mut u8,
    base: u32,
    pitch: u32,
    start_row: u32,
    row_bytes: u32,
    rows: u32,
) -> Option<(u64, usize)> {
    if base == 0 || pitch == 0 || row_bytes == 0 || rows == 0 || row_bytes > pitch {
        return None;
    }
    let end = base as u64
        + (start_row as u64).saturating_mul(pitch as u64)
        + (rows.saturating_sub(1) as u64).saturating_mul(pitch as u64)
        + row_bytes as u64;
    if end > 0x2000_0000 {
        return None;
    }
    let total = (row_bytes as usize).saturating_mul(rows as usize);
    if total == 0 || total > 4 * 1024 * 1024 {
        return None;
    }

    let mut bytes = Vec::with_capacity(total);
    let mut nonzero = 0usize;
    unsafe {
        for y in 0..rows {
            let row = std::slice::from_raw_parts(
                guest_mem.add(
                    (base + start_row.saturating_mul(pitch) + y.saturating_mul(pitch)) as usize,
                ),
                row_bytes as usize,
            );
            nonzero += row.iter().filter(|b| **b != 0).count();
            bytes.extend_from_slice(row);
        }
    }
    Some((fnv1a64(&bytes), nonzero))
}

fn doom_hash_linear_rows(
    bytes: &[u8],
    row_stride: u32,
    row_bytes: u32,
    rows: u32,
) -> Option<(u64, usize)> {
    doom_hash_linear_rows_from(bytes, row_stride, 0, row_bytes, rows)
}

fn doom_hash_linear_rows_from(
    bytes: &[u8],
    row_stride: u32,
    start_row: u32,
    row_bytes: u32,
    rows: u32,
) -> Option<(u64, usize)> {
    if row_stride == 0 || row_bytes == 0 || rows == 0 || row_bytes > row_stride {
        return None;
    }
    let end = (start_row as usize)
        .saturating_mul(row_stride as usize)
        .saturating_add((rows.saturating_sub(1) as usize).saturating_mul(row_stride as usize))
        .saturating_add(row_bytes as usize);
    if end > bytes.len() {
        return None;
    }
    let total = (row_bytes as usize).saturating_mul(rows as usize);
    if total == 0 || total > 4 * 1024 * 1024 {
        return None;
    }

    let mut packed = Vec::with_capacity(total);
    let mut nonzero = 0usize;
    for y in 0..rows as usize {
        let start = (start_row as usize + y).saturating_mul(row_stride as usize);
        let row = &bytes[start..start + row_bytes as usize];
        nonzero += row.iter().filter(|b| **b != 0).count();
        packed.extend_from_slice(row);
    }
    Some((fnv1a64(&packed), nonzero))
}

fn doom_row_hex(guest_mem: *mut u8, base: u32, offset: u32) -> String {
    if base == 0 {
        return "-".to_string();
    }
    let addr = base as u64 + offset as u64;
    if addr + 16 > 0x2000_0000 {
        return "-".to_string();
    }
    unsafe {
        let bytes = std::slice::from_raw_parts(guest_mem.add(addr as usize), 16);
        format!("{:02X?}", bytes)
    }
}

fn doom_screen_source_probe(
    guest_mem: *mut u8,
) -> Option<(u32, u32, u32, u32, &'static str, u64, usize)> {
    let live = doom_read_u32(guest_mem, 0x0010_1BB8);
    let cached = crate::xbox::worker::DOOM_GAME_STATE_CACHE.load(Ordering::Relaxed);
    for (state, source) in [(live, "live"), (cached, "cached")] {
        if state == 0 || state >= 0x0800_0000 {
            continue;
        }
        let w = doom_read_u32(guest_mem, state.wrapping_add(0x14));
        let h = doom_read_u32(guest_mem, state.wrapping_add(0x18));
        let screen = doom_read_u32(guest_mem, state.wrapping_add(0x21AF0));
        if w == 320 && (160..=240).contains(&h) && screen != 0 && screen < 0x1000_0000 {
            let (hash, nonzero) = doom_hash_guest_rows(guest_mem, screen, w, w, h.min(240))?;
            return Some((state, screen, w, h, source, hash, nonzero));
        }
    }
    None
}

fn doom_rect_from_guest(
    guest_mem: *mut u8,
    p_rect: u32,
    width: u32,
    height: u32,
) -> (u32, u32, u32, u32) {
    let Some(rect) = normalize_guest_ram_ptr(p_rect) else {
        return (0, 0, width, height);
    };
    if rect > 0x2000_0000u32.saturating_sub(16) {
        return (0, 0, width, height);
    }
    unsafe {
        let base = guest_mem.add(rect as usize);
        (
            std::ptr::read_unaligned(base as *const u32),
            std::ptr::read_unaligned(base.add(4) as *const u32),
            std::ptr::read_unaligned(base.add(8) as *const u32),
            std::ptr::read_unaligned(base.add(12) as *const u32),
        )
    }
}

fn doom_visible_dims(lock: DoomBlitLockInfo, width: u32, height: u32) -> (u32, u32) {
    let w = if lock.source_w != 0 {
        lock.source_w
    } else if lock.rect_r > lock.rect_l {
        lock.rect_r - lock.rect_l
    } else {
        320
    };
    let h = if lock.source_h != 0 {
        lock.source_h
    } else if lock.rect_b > lock.rect_t {
        lock.rect_b - lock.rect_t
    } else {
        168
    };
    (w.min(width).max(1), h.min(height).max(1))
}

fn doom_expected_visible_dims(width: u32, height: u32) -> (u32, u32) {
    if width == 512 && height == 256 {
        (320, 168)
    } else {
        (width.min(320).max(1), height.min(168).max(1))
    }
}

fn doom_record_lockrect(
    guest_mem: *mut u8,
    tex_raw: u32,
    level: u32,
    p_locked_rect: u32,
    p_rect: u32,
    flags: u32,
    info: HleTextureInfo,
) {
    if !doom_blit_diag_enabled() {
        return;
    }

    static LOCK_SEQ: AtomicU32 = AtomicU32::new(1);
    let seq = LOCK_SEQ.fetch_add(1, Ordering::Relaxed);
    let format_code = xbox_texture_format_code(info.format);
    let (rect_l, rect_t, rect_r, rect_b) =
        doom_rect_from_guest(guest_mem, p_rect, info.width, info.height);
    let (source_state, source_screen, source_w, source_h, source_kind, source_hash, source_nonzero) =
        doom_screen_source_probe(guest_mem).unwrap_or((0, 0, 0, 0, "none", 0, 0));
    let lock = DoomBlitLockInfo {
        seq,
        swap: OOVPA_SWAP_COUNT.load(Ordering::Relaxed),
        tex_raw,
        tex_key: info.key,
        level,
        p_locked_rect,
        p_rect,
        flags,
        rect_l,
        rect_t,
        rect_r,
        rect_b,
        width: info.width,
        height: info.height,
        format: info.format,
        format_code,
        data: info.data,
        pitch: info.pitch,
        source_state,
        source_screen,
        source_w,
        source_h,
        source_hash,
        source_nonzero,
    };
    *doom_blit_lock_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(lock);

    if seq <= 16 || seq.is_power_of_two() {
        let bpp = texture_bytes_per_pixel(format_code).unwrap_or(4);
        let expected_pitch = info.width.saturating_mul(bpp);
        let expected_visible = doom_expected_visible_dims(info.width, info.height);
        debug_log(&format!(
            "[DOOM-BLIT-LOCK] #{} swap={} tex=0x{:08X}->0x{:08X} level={} rect=({},{}..{},{}) pRect=0x{:08X} flags=0x{:08X} returned pBits=0x{:08X} pitch={} expected_pitch={} pitch_ok={} tex={}x{} expected_visible={}x{} fmt=0x{:08X}/0x{:02X}/{} source={} state=0x{:08X} screen=0x{:08X} source={}x{} expected_source=320x168 source_ok={} src_hash=0x{:016X} src_nonzero={}",
            seq,
            lock.swap,
            tex_raw,
            info.key,
            level,
            rect_l,
            rect_t,
            rect_r,
            rect_b,
            p_rect,
            flags,
            info.data,
            info.pitch,
            expected_pitch,
            (info.pitch == expected_pitch) as u8,
            info.width,
            info.height,
            expected_visible.0,
            expected_visible.1,
            info.format,
            format_code,
            crate::xbox::gpu::texture_format::format_name(format_code),
            source_kind,
            source_state,
            source_screen,
            source_w,
            source_h,
            (source_w == 320 && source_h == 168) as u8,
            source_hash,
            source_nonzero
        ));
    }
}

fn doom_record_texture_upload(
    stage: u32,
    tex_key: u32,
    data: u32,
    width: u32,
    height: u32,
    format_code: u32,
    pitch: u32,
    bpp: u32,
    swizzled: bool,
    linear_bytes: &[u8],
    row_stride: u32,
) {
    if !doom_blit_diag_enabled() || stage != 0 || width == 0 || height == 0 || bpp == 0 {
        return;
    }

    let lock = *doom_blit_lock_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let lock = lock.unwrap_or_default();
    let (visible_w, visible_h) = doom_visible_dims(lock, width, height);
    let row_bytes = visible_w.saturating_mul(bpp);
    let full_row_bytes = width.saturating_mul(bpp);
    let (full_hash, full_nonzero) =
        doom_hash_linear_rows(linear_bytes, row_stride, full_row_bytes, height).unwrap_or((0, 0));
    let (hash, nonzero) =
        doom_hash_linear_rows(linear_bytes, row_stride, row_bytes, visible_h).unwrap_or((0, 0));
    let disputed_rows = height.saturating_sub(168).min(32);
    let disputed_row_bytes = width.min(320).saturating_mul(bpp);
    let (disputed_hash, disputed_nonzero) = if disputed_rows > 0 {
        doom_hash_linear_rows_from(
            linear_bytes,
            row_stride,
            168,
            disputed_row_bytes,
            disputed_rows,
        )
        .unwrap_or((0, 0))
    } else {
        (0, 0)
    };

    static UPLOAD_SEQ: AtomicU32 = AtomicU32::new(1);
    let seq = UPLOAD_SEQ.fetch_add(1, Ordering::Relaxed);
    let upload = DoomBlitUploadInfo {
        seq,
        lock_seq: lock.seq,
        stage,
        tex_key,
        data,
        width,
        height,
        format_code,
        pitch,
        bpp,
        swizzled,
        visible_w,
        visible_h,
        row_stride,
        full_hash,
        full_nonzero,
        hash,
        nonzero,
        disputed_rows,
        disputed_hash,
        disputed_nonzero,
    };
    *doom_blit_upload_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(upload);

    if seq <= 16 || seq.is_power_of_two() {
        let expected_visible = doom_expected_visible_dims(width, height);
        let expected_pitch = width.saturating_mul(bpp);
        debug_log(&format!(
            "[DOOM-BLIT-UPLOAD] #{} lock={} stage={} tex=0x{:08X} data=0x{:08X} tex={}x{} visible={}x{} expected_visible={}x{} fmt=0x{:02X}/{} pitch={} expected_pitch={} pitch_ok={} bpp={} swizzled={} unswizzle_args={}x{} bpp={} row_stride={} full_logical_hash=0x{:016X} full_nonzero={} world_logical_rows[0..{}]_hash=0x{:016X} world_nonzero={} disputed_logical_rows[168..{}]_hash=0x{:016X} disputed_nonzero={}",
            seq,
            lock.seq,
            stage,
            tex_key,
            data,
            width,
            height,
            visible_w,
            visible_h,
            expected_visible.0,
            expected_visible.1,
            format_code,
            crate::xbox::gpu::texture_format::format_name(format_code),
            pitch,
            expected_pitch,
            (pitch == expected_pitch) as u8,
            bpp,
            swizzled as u8,
            width,
            height,
            bpp,
            row_stride,
            full_hash,
            full_nonzero,
            visible_h,
            hash,
            nonzero,
            168 + disputed_rows,
            disputed_hash,
            disputed_nonzero
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn doom_record_xg_swizzle(
    guest_mem: *mut u8,
    seq: u32,
    src_arg: u32,
    src_base: u32,
    dst_arg: u32,
    dst_base: u32,
    pitch_arg: u32,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u32,
    src_x: u32,
    src_y: u32,
    rect_w: u32,
    rect_h: u32,
    dst_x: u32,
    dst_y: u32,
) -> DoomBlitSwizzleInfo {
    let (_, _, doom_source_w, doom_source_h, _, _, _) =
        doom_screen_source_probe(guest_mem).unwrap_or((0, 0, 0, 0, "none", 0, 0));
    let doom_source_pitch_guess = doom_source_w.saturating_mul(bpp);
    let row_bytes = rect_w.saturating_mul(bpp);
    let (src_resolved_hash, src_resolved_nonzero) =
        doom_hash_guest_rows_from(guest_mem, src_base, pitch, src_y, row_bytes, rect_h)
            .unwrap_or((0, 0));
    let (src_doom_pitch_hash, src_doom_pitch_nonzero) =
        if doom_source_pitch_guess != 0 && doom_source_h != 0 {
            doom_hash_guest_rows_from(
                guest_mem,
                src_base,
                doom_source_pitch_guess,
                0,
                doom_source_pitch_guess,
                doom_source_h,
            )
            .unwrap_or((0, 0))
        } else {
            (0, 0)
        };
    let swizzle = DoomBlitSwizzleInfo {
        seq,
        src_arg,
        src_base,
        dst_arg,
        dst_base,
        pitch_arg,
        pitch,
        width,
        height,
        bpp,
        src_x,
        src_y,
        rect_w,
        rect_h,
        dst_x,
        dst_y,
        doom_source_w,
        doom_source_h,
        doom_source_pitch_guess,
        src_resolved_hash,
        src_resolved_nonzero,
        src_doom_pitch_hash,
        src_doom_pitch_nonzero,
    };
    *doom_blit_swizzle_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(swizzle);
    doom_dump_swizzle_staging_views(guest_mem, swizzle);
    swizzle
}

fn doom_primitive_name(prim_type: u32) -> &'static str {
    match prim_type {
        1 => "POINTLIST",
        2 => "LINELIST",
        3 => "LINESTRIP",
        4 => "LINELOOP",
        5 => "TRIANGLELIST",
        6 => "TRIANGLESTRIP",
        7 => "TRIANGLEFAN",
        8 => "QUADLIST",
        _ => "UNKNOWN",
    }
}

fn doom_format_first_vertices(verts: &[NV2AVertex]) -> String {
    let mut out = String::new();
    for (i, v) in verts.iter().take(4).enumerate() {
        if i > 0 {
            out.push_str(" ");
        }
        out.push_str(&format!(
            "#{} pos=({:.2},{:.2},{:.3},{:.3}) uv=({:.5},{:.5}) color=0x{:08X}",
            i, v.x, v.y, v.z, v.w, v.u, v.v, v.color
        ));
    }
    if out.is_empty() {
        "-".to_string()
    } else {
        out
    }
}

fn doom_log_draw_vertices_up_pipeline(
    prim_type: u32,
    vertex_count: u32,
    stride: u32,
    vertex_data_ptr: u32,
    refreshed: u32,
    verts: &[NV2AVertex],
    guest_mem: *mut u8,
) {
    if !doom_blit_diag_enabled() {
        return;
    }
    static DRAW_SEQ: AtomicU32 = AtomicU32::new(1);
    let seq = DRAW_SEQ.fetch_add(1, Ordering::Relaxed);
    if !(seq <= 64 || seq.is_power_of_two()) {
        return;
    }

    let lock = *doom_blit_lock_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let upload = *doom_blit_upload_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let swizzle = *doom_blit_swizzle_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let active_tex0 = active_texture_bindings()[0].load(Ordering::Relaxed);
    let ps_snapshot = pixel_shader_state_snapshot();
    let stage0_state = texture_stage_states()
        .lock()
        .unwrap_or_else(|e| e.into_inner())[0];
    let mut sampled_vertices = [NV2AVertex::default(); 4];
    for (dst, src) in sampled_vertices.iter_mut().zip(verts.iter().take(4)) {
        *dst = *src;
    }
    *doom_blit_draw_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(DoomBlitDrawInfo {
        seq,
        prim_type,
        vertex_count,
        stride,
        vertex_data_ptr,
        refreshed,
        active_tex0,
        ps_handle: CURRENT_PIXEL_SHADER_HANDLE.load(Ordering::Relaxed),
        ps: ps_snapshot,
        stage0: stage0_state,
        vertex_sample_count: verts.len().min(4) as u32,
        vertices: sampled_vertices,
    });

    let (lock_line, verify_line) = if let Some(lock) = lock {
        let bpp = texture_bytes_per_pixel(lock.format_code).unwrap_or(4);
        let (visible_w, visible_h) = doom_visible_dims(lock, lock.width, lock.height);
        let (expected_w, expected_h) = doom_expected_visible_dims(lock.width, lock.height);
        let expected_pitch = lock.width.saturating_mul(bpp);
        let row_bytes = visible_w.saturating_mul(bpp);
        let dest = doom_hash_guest_rows(guest_mem, lock.data, lock.pitch, row_bytes, visible_h)
            .unwrap_or((0, 0));
        let disputed_rows = lock.height.saturating_sub(168).min(32);
        let disputed_row_bytes = lock.width.min(320).saturating_mul(bpp);
        let raw_disputed = if disputed_rows > 0 {
            doom_hash_guest_rows_from(
                guest_mem,
                lock.data,
                lock.pitch,
                168,
                disputed_row_bytes,
                disputed_rows,
            )
            .unwrap_or((0, 0))
        } else {
            (0, 0)
        };
        let row0 = doom_row_hex(guest_mem, lock.data, 0);
        let row1 = doom_row_hex(guest_mem, lock.data, lock.pitch);
        let upload_match = upload
            .map(|u| u.hash == dest.0 && u.visible_w == visible_w && u.visible_h == visible_h)
            .unwrap_or(false);
        (
            format!(
                "Lock: seq={} swap={} texture=0x{:08X}->0x{:08X} level={} rect=({},{}..{},{}) pLockedRect=0x{:08X} pRect=0x{:08X} flags=0x{:08X} pBits=0x{:08X} pitch={} expected_pitch={} pitch_ok={} tex={}x{} expected_visible={}x{} actual_visible={}x{} fmt=0x{:08X}/0x{:02X}/{} source_state=0x{:08X} source_screen=0x{:08X} source={}x{} expected_source=320x168 source_ok={} source_hash=0x{:016X} source_nonzero={}",
                lock.seq,
                lock.swap,
                lock.tex_raw,
                lock.tex_key,
                lock.level,
                lock.rect_l,
                lock.rect_t,
                lock.rect_r,
                lock.rect_b,
                lock.p_locked_rect,
                lock.p_rect,
                lock.flags,
                lock.data,
                lock.pitch,
                expected_pitch,
                (lock.pitch == expected_pitch) as u8,
                lock.width,
                lock.height,
                expected_w,
                expected_h,
                visible_w,
                visible_h,
                lock.format,
                lock.format_code,
                crate::xbox::gpu::texture_format::format_name(lock.format_code),
                lock.source_state,
                lock.source_screen,
                lock.source_w,
                lock.source_h,
                (lock.source_w == 320 && lock.source_h == 168) as u8,
                lock.source_hash,
                lock.source_nonzero
            ),
            format!(
                "Verify: visible={}x{} bpp={} pBits_row0={} pBits_row1={} raw_pBits_stride_offsets[0..{}*pitch]_hash=0x{:016X} raw_nonzero={} disputed_raw_bytes[168*pitch..{}*pitch]_hash=0x{:016X} disputed_raw_nonzero={} upload_match={}",
                visible_w,
                visible_h,
                bpp,
                row0,
                row1,
                visible_h,
                dest.0,
                dest.1,
                168 + disputed_rows,
                raw_disputed.0,
                raw_disputed.1,
                upload_match as u8
            ),
        )
    } else {
        ("Lock: none".to_string(), "Verify: none".to_string())
    };

    let upload_line = if let Some(upload) = upload {
        let (expected_w, expected_h) = doom_expected_visible_dims(upload.width, upload.height);
        let expected_pitch = upload.width.saturating_mul(upload.bpp);
        format!(
            "Upload: seq={} lock={} stage={} tex=0x{:08X} data=0x{:08X} tex={}x{} visible={}x{} expected_visible={}x{} fmt=0x{:02X}/{} pitch={} expected_pitch={} pitch_ok={} bpp={} swizzled={} unswizzle_args={}x{} bpp={} src=0x{:08X} src_len={} row_stride={} full_logical_hash=0x{:016X} full_nonzero={} world_logical_rows[0..{}]_hash=0x{:016X} world_nonzero={} disputed_logical_rows[168..{}]_hash=0x{:016X} disputed_nonzero={}",
            upload.seq,
            upload.lock_seq,
            upload.stage,
            upload.tex_key,
            upload.data,
            upload.width,
            upload.height,
            upload.visible_w,
            upload.visible_h,
            expected_w,
            expected_h,
            upload.format_code,
            crate::xbox::gpu::texture_format::format_name(upload.format_code),
            upload.pitch,
            expected_pitch,
            (upload.pitch == expected_pitch) as u8,
            upload.bpp,
            upload.swizzled as u8,
            upload.width,
            upload.height,
            upload.bpp,
            upload.data,
            (upload.width as usize)
                .saturating_mul(upload.height as usize)
                .saturating_mul(upload.bpp as usize),
            upload.row_stride,
            upload.full_hash,
            upload.full_nonzero,
            upload.visible_h,
            upload.hash,
            upload.nonzero,
            168 + upload.disputed_rows,
            upload.disputed_hash,
            upload.disputed_nonzero
        )
    } else {
        "Upload: none".to_string()
    };

    let roundtrip_line = match (swizzle, upload) {
        (Some(s), Some(u)) => {
            let texture_format = lock.map(|l| l.format).unwrap_or(u.format_code);
            let texture_format_code = lock.map(|l| l.format_code).unwrap_or(u.format_code);
            let expected_unswizzle_pitch = u.width.saturating_mul(u.bpp);
            let swizzle_unswizzle_args = s.width == u.width
                && s.height == u.height
                && s.bpp == u.bpp
                && s.pitch == expected_unswizzle_pitch;
            let full_roundtrip_hash = s.src_resolved_hash == u.full_hash;
            let swizzle_doom_pitch = s.doom_source_pitch_guess != 0
                && s.width == s.doom_source_w
                && s.height == s.doom_source_h
                && s.pitch == s.doom_source_pitch_guess;
            format!(
                "RoundTrip:\n  XGSwizzleRect: seq={} raw_src=0x{:08X}->0x{:08X} raw_dst=0x{:08X}->0x{:08X} raw_w={} raw_h={} raw_bpp={} pitch_arg={} pitch_resolved={} rect=({},{} {}x{}) point=({}, {})\n  unswizzle(): seq={} tex=0x{:08X} src=0x{:08X} w={} h={} bpp={} src_len={} row_stride={} pitch_implicit={} full_hash=0x{:016X} full_nonzero={}\n  Doom source: state={}x{} pitch_guess={} src_hash_at_pitch=0x{:016X} src_nonzero_at_pitch={} src_hash_at_doom_pitch=0x{:016X} src_nonzero_at_doom_pitch={} texture_create_or_header_fmt=0x{:08X}/0x{:02X}/{}\n  MATCH: swizzle_unswizzle_args={} full_roundtrip_hash={} swizzle_doom_pitch={}",
                s.seq,
                s.src_arg,
                s.src_base,
                s.dst_arg,
                s.dst_base,
                s.width,
                s.height,
                s.bpp,
                s.pitch_arg,
                s.pitch,
                s.src_x,
                s.src_y,
                s.rect_w,
                s.rect_h,
                s.dst_x,
                s.dst_y,
                u.seq,
                u.tex_key,
                u.data,
                u.width,
                u.height,
                u.bpp,
                (u.width as usize)
                    .saturating_mul(u.height as usize)
                    .saturating_mul(u.bpp as usize),
                u.row_stride,
                expected_unswizzle_pitch,
                u.full_hash,
                u.full_nonzero,
                s.doom_source_w,
                s.doom_source_h,
                s.doom_source_pitch_guess,
                s.src_resolved_hash,
                s.src_resolved_nonzero,
                s.src_doom_pitch_hash,
                s.src_doom_pitch_nonzero,
                texture_format,
                texture_format_code,
                crate::xbox::gpu::texture_format::format_name(texture_format_code),
                swizzle_unswizzle_args as u8,
                full_roundtrip_hash as u8,
                swizzle_doom_pitch as u8
            )
        }
        _ => "RoundTrip: none".to_string(),
    };

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_u = f32::INFINITY;
    let mut min_v = f32::INFINITY;
    let mut max_u = f32::NEG_INFINITY;
    let mut max_v = f32::NEG_INFINITY;
    for v in verts {
        if v.x.is_finite() {
            min_x = min_x.min(v.x);
            max_x = max_x.max(v.x);
        }
        if v.y.is_finite() {
            min_y = min_y.min(v.y);
            max_y = max_y.max(v.y);
        }
        if v.u.is_finite() {
            min_u = min_u.min(v.u);
            max_u = max_u.max(v.u);
        }
        if v.v.is_finite() {
            min_v = min_v.min(v.v);
            max_v = max_v.max(v.v);
        }
    }
    if verts.is_empty() {
        min_x = 0.0;
        min_y = 0.0;
        max_x = 0.0;
        max_y = 0.0;
        min_u = 0.0;
        min_v = 0.0;
        max_u = 0.0;
        max_v = 0.0;
    }

    let actual_uv_max = lock
        .map(|l| {
            let (vw, vh) = doom_visible_dims(l, l.width, l.height);
            (
                vw as f32 / l.width.max(1) as f32,
                vh as f32 / l.height.max(1) as f32,
            )
        })
        .unwrap_or((0.0, 0.0));
    let expected_uv_max = lock
        .map(|l| {
            let (vw, vh) = doom_expected_visible_dims(l.width, l.height);
            (
                vw as f32 / l.width.max(1) as f32,
                vh as f32 / l.height.max(1) as f32,
            )
        })
        .unwrap_or((0.0, 0.0));

    debug_log(&format!(
        "[DOOM-BLIT-PIPELINE #{}]\n  {}\n  {}\n  {}\n  Draw: prim={}({}) vertex_count={} stride={} data=0x{:08X} refreshed={} tex_stage0=0x{:08X} expected_uv_max=({:.5},{:.5}) actual_visible_uv_max=({:.5},{:.5}) uv_bbox=[{:.5},{:.5}..{:.5},{:.5}] pos_bbox=[{:.2},{:.2}..{:.2},{:.2}]\n  Vertices: {}\n  {}",
        seq,
        lock_line,
        upload_line,
        roundtrip_line,
        prim_type,
        doom_primitive_name(prim_type),
        vertex_count,
        stride,
        vertex_data_ptr,
        refreshed,
        active_tex0,
        expected_uv_max.0,
        expected_uv_max.1,
        actual_uv_max.0,
        actual_uv_max.1,
        min_u,
        min_v,
        max_u,
        max_v,
        min_x,
        min_y,
        max_x,
        max_y,
        doom_format_first_vertices(verts),
        verify_line
    ));
}

fn doom_log_swap_pipeline(swap_flags: u32, guest_mem: *mut u8) {
    if !doom_blit_diag_enabled() {
        return;
    }
    static SWAP_SEQ: AtomicU32 = AtomicU32::new(1);
    let seq = SWAP_SEQ.fetch_add(1, Ordering::Relaxed);
    if !(seq <= 32 || seq.is_power_of_two()) {
        return;
    }
    let lock = *doom_blit_lock_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let upload = *doom_blit_upload_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let draw = *doom_blit_draw_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let active_tex0 = active_texture_bindings()[0].load(Ordering::Relaxed);
    doom_dump_bound_texture_at_swap(guest_mem, seq, swap_flags, active_tex0, lock, upload, draw);
    debug_log(&format!(
        "[DOOM-BLIT-SWAP] #{} flags=0x{:08X} active_tex0=0x{:08X} lock_seq={} upload_seq={} upload_swizzled={}",
        seq,
        swap_flags,
        active_tex0,
        lock.map(|l| l.seq).unwrap_or(0),
        upload.map(|u| u.seq).unwrap_or(0),
        upload.map(|u| u.swizzled as u8).unwrap_or(0)
    ));
}

fn pack_texture_size(width: u32, height: u32) -> u32 {
    let w = width.clamp(1, 4096) - 1;
    let h = height.clamp(1, 4096) - 1;
    (w & 0x0FFF) | ((h & 0x0FFF) << 12)
}

fn sane_texture_dims(width: u32, height: u32) -> bool {
    width > 0 && width <= 4096 && height > 0 && height <= 4096
}

fn upsert_texture_info(info: HleTextureInfo) {
    if info.key == 0 {
        return;
    }
    let mut reg = texture_registry().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = reg.iter_mut().find(|entry| entry.key == info.key) {
        if sane_texture_dims(info.width, info.height) {
            existing.width = info.width;
            existing.height = info.height;
        }
        if info.format != 0 {
            existing.format = info.format;
        }
        if info.data != 0 {
            existing.data = info.data;
        }
        if info.pitch != 0 {
            existing.pitch = info.pitch;
        }
        return;
    }
    if reg.len() < 256 {
        reg.push(info);
    }
}

fn lookup_texture_info(key: u32) -> Option<HleTextureInfo> {
    let reg = texture_registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.iter().find(|entry| entry.key == key).copied()
}

fn ranges_overlap(a_base: u32, a_len: u32, b_base: u32, b_len: u32) -> bool {
    let a_end = (a_base as u64).saturating_add(a_len.max(1) as u64);
    let b_end = (b_base as u64).saturating_add(b_len.max(1) as u64);
    (a_base as u64) < b_end && (b_base as u64) < a_end
}

fn texture_dirty_matches(info: HleTextureInfo, dirty: HleTextureDirtyRange) -> bool {
    if info.key == dirty.key {
        return true;
    }
    let Some(data) = normalize_guest_ram_ptr(info.data & !0x3) else {
        return false;
    };
    let len = texture_mip_chain_byte_len(info.width, info.height, info.format);
    ranges_overlap(data, len, dirty.data, dirty.len)
}

fn mark_texture_dirty_from_lock(info: HleTextureInfo, source: &str) {
    let Some(data) = normalize_guest_ram_ptr(info.data & !0x3) else {
        return;
    };
    let len = texture_mip_chain_byte_len(info.width, info.height, info.format)
        .max(info.pitch)
        .max(1);
    static DIRTY_SERIAL: AtomicU32 = AtomicU32::new(1);
    let serial = DIRTY_SERIAL.fetch_add(1, Ordering::Relaxed);
    texture_dirty_epoch().fetch_add(1, Ordering::Relaxed);
    let dirty = HleTextureDirtyRange {
        key: info.key,
        data,
        len,
        serial,
    };
    let mut ranges = texture_dirty_ranges()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = ranges.iter_mut().find(|entry| entry.key == info.key) {
        *existing = dirty;
    } else {
        if ranges.len() >= 128 {
            ranges.remove(0);
        }
        ranges.push(dirty);
    }
    static DIRTY_LOG: AtomicU32 = AtomicU32::new(0);
    let n = DIRTY_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-TEX-DIRTY] #{} source={} serial={} tex=0x{:08X} data=0x{:08X} len={} {}x{} fmt=0x{:08X}/0x{:02X}/{} pitch={}",
            n,
            source,
            serial,
            info.key,
            data,
            len,
            info.width,
            info.height,
            info.format,
            xbox_texture_format_code(info.format),
            crate::xbox::gpu::texture_format::format_name(xbox_texture_format_code(info.format)),
            info.pitch
        ));
    }
}

fn probe_spidey_fullscreen_lockrect(info: HleTextureInfo, guest_mem: *mut u8) {
    let fmt_code = xbox_texture_format_code(info.format);
    let is_argb_like = matches!(
        fmt_code,
        crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_X8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_X8R8G8B8
    );
    if !is_argb_like || info.width != 640 || info.height != 480 || info.pitch != 640 * 4 {
        return;
    }

    let Some(data) = normalize_guest_ram_ptr(info.data & !0x3) else {
        return;
    };
    if data as u64 + (info.pitch as u64 * info.height as u64) > 0x2000_0000 {
        return;
    }

    static FULL_LOCK_LOG: AtomicU32 = AtomicU32::new(0);
    let n = FULL_LOCK_LOG.fetch_add(1, Ordering::Relaxed);
    if !(n < 8 || n.is_power_of_two()) {
        return;
    }

    let mut pixels = Vec::with_capacity((640 * 480) as usize);
    let mut rgb_nonblack = 0u32;
    let mut alpha_nonzero = 0u32;
    let mut first = 0u32;
    let mut mid = 0u32;
    let mut last = 0u32;

    unsafe {
        for y in 0..480u32 {
            let row = guest_mem.add((data + y * info.pitch) as usize);
            for x in 0..640u32 {
                let px = std::ptr::read_unaligned(row.add((x * 4) as usize) as *const u32);
                if pixels.is_empty() {
                    first = px;
                }
                if x == 320 && y == 240 {
                    mid = px;
                }
                last = px;
                if (px & 0x00FF_FFFF) != 0 {
                    rgb_nonblack = rgb_nonblack.saturating_add(1);
                }
                if (px >> 24) != 0 {
                    alpha_nonzero = alpha_nonzero.saturating_add(1);
                }
                pixels.push(px);
            }
        }
    }

    write_readback_bmp(
        r"./spidey_lockrect_640_latest.bmp",
        640,
        480,
        &pixels,
    );
    if n < 8 {
        let path = format!(r"./spidey_lockrect_640_{:02}.bmp", n);
        write_readback_bmp(&path, 640, 480, &pixels);
    }
    debug_log(&format!(
        "[SPIDEY-LOCKRECT-640] #{} tex=0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} rgb_nonblack={}/{} alpha_nonzero={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X}",
        n,
        info.key,
        data,
        fmt_code,
        crate::xbox::gpu::texture_format::format_name(fmt_code),
        rgb_nonblack,
        640 * 480,
        alpha_nonzero,
        640 * 480,
        first,
        mid,
        last
    ));
}

fn refresh_dirty_active_textures_before_draw(tag: &str, guest_mem: *mut u8) -> u32 {
    let active: Vec<(u32, u32, HleTextureInfo)> = active_texture_bindings()
        .iter()
        .enumerate()
        .filter_map(|(stage, slot)| {
            let tex_key = slot.load(Ordering::Relaxed);
            if tex_key == 0 {
                None
            } else {
                lookup_texture_info(tex_key).map(|info| (stage as u32, tex_key, info))
            }
        })
        .collect();

    if active.is_empty() {
        return 0;
    }

    let mut refreshes = Vec::new();
    {
        let mut ranges = texture_dirty_ranges()
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        ranges.retain(|dirty| {
            let mut matched = false;
            for (stage, tex_key, info) in &active {
                if texture_dirty_matches(*info, *dirty) {
                    refreshes.push((*stage, *tex_key, *dirty));
                    matched = true;
                }
            }
            !matched
        });
    }

    refreshes.sort_by_key(|(stage, tex_key, _)| (*stage, *tex_key));
    refreshes.dedup_by_key(|(stage, tex_key, _)| (*stage, *tex_key));
    for (stage, tex_key, dirty) in &refreshes {
        let mut args = [0u32; 8];
        args[0] = *stage;
        args[1] = *tex_key;
        hle_set_texture_impl(&args, guest_mem, true);

        static REFRESH_LOG: AtomicU32 = AtomicU32::new(0);
        let n = REFRESH_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-TEX-DIRTY-REFRESH] #{} tag={} stage={} tex=0x{:08X} dirty_serial={} data=0x{:08X} len={}",
                n, tag, stage, tex_key, dirty.serial, dirty.data, dirty.len
            ));
        }
    }

    refreshes.len() as u32
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct HleRenderTargetInfo {
    surface: u32,
    key: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
}

fn pending_hle_render_target() -> &'static Mutex<Option<HleRenderTargetInfo>> {
    static PENDING: OnceLock<Mutex<Option<HleRenderTargetInfo>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(None))
}

fn store_pending_hle_render_target(info: HleRenderTargetInfo) {
    let mut pending = pending_hle_render_target()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *pending = Some(info);
}

fn clear_pending_hle_render_target_if_matches(info: HleRenderTargetInfo) {
    let mut pending = pending_hle_render_target()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if pending.as_ref().copied() == Some(info) {
        *pending = None;
    }
}

fn take_pending_hle_render_target() -> Option<HleRenderTargetInfo> {
    pending_hle_render_target()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .take()
}

fn sane_render_target_dims(width: u32, height: u32) -> bool {
    (16..=4096).contains(&width) && (16..=4096).contains(&height)
}

fn sane_pitch_for_format(pitch: u32, width: u32, format_code: u32) -> bool {
    if pitch == 0 || width == 0 {
        return false;
    }

    let expected = texture_pitch_bytes(width, format_code);
    if pitch < expected {
        return false;
    }

    let align = crate::xbox::gpu::texture_format::bytes_per_pixel(format_code)
        .filter(|bpp| *bpp != 0)
        .unwrap_or(4);
    pitch % align == 0 && pitch <= expected.max(64).saturating_mul(8)
}

fn xbox_pixel_container_measures(format: u32, size_field: u32) -> Option<(u32, u32, u32)> {
    let fmt_code = xbox_texture_format_code(format);
    if size_field != 0 {
        // X_D3DPixelContainer.Size layout for linear/non-power-of-two surfaces:
        // bits  0..11 = width - 1
        // bits 12..23 = height - 1
        // bits 24..31 = pitch / 64 - 1
        let width = (size_field & 0x0FFF).saturating_add(1);
        let height = ((size_field >> 12) & 0x0FFF).saturating_add(1);
        let pitch = (((size_field >> 24) & 0xFF).saturating_add(1)).saturating_mul(64);
        if sane_render_target_dims(width, height) && sane_pitch_for_format(pitch, width, fmt_code) {
            return Some((width, height, pitch));
        }
        return None;
    }

    // Swizzled/compressed power-of-two containers leave Size at zero and store
    // log2 dimensions in Format. This is mostly for texture surfaces, but using
    // the real rule here keeps render-target diagnostics honest.
    let log2w = (format >> 20) & 0x0F;
    let log2h = (format >> 24) & 0x0F;
    if log2w == 0 && log2h == 0 {
        return None;
    }
    let width = 1u32.checked_shl(log2w)?;
    let height = 1u32.checked_shl(log2h)?;
    let pitch = texture_pitch_bytes(width, fmt_code);
    if sane_render_target_dims(width, height) && sane_pitch_for_format(pitch, width, fmt_code) {
        Some((width, height, pitch))
    } else {
        None
    }
}

fn packed_surface_size_dims(size_field: u32) -> Option<(u32, u32)> {
    if size_field == 0 {
        return None;
    }
    let width = (size_field & 0x0FFF).saturating_add(1);
    let height = ((size_field >> 12) & 0x0FFF).saturating_add(1);
    if sane_render_target_dims(width, height) {
        Some((width, height))
    } else {
        None
    }
}

fn hle_render_target_info(guest_mem: *mut u8, surface_addr: u32) -> Option<HleRenderTargetInfo> {
    let surface = normalize_guest_ram_ptr(surface_addr)?;
    if surface.saturating_add(0x28) > 0x2000_0000 {
        return None;
    }

    let registry = lookup_texture_info(surface);
    let base = unsafe { guest_mem.add(surface as usize) };
    let data_raw = unsafe { std::ptr::read_unaligned(base.add(0x04) as *const u32) };
    let header_format = unsafe { std::ptr::read_unaligned(base.add(0x0C) as *const u32) };
    let size_field = unsafe { std::ptr::read_unaligned(base.add(0x10) as *const u32) };
    let header_width = unsafe { std::ptr::read_unaligned(base.add(0x1C) as *const u32) };
    let header_height = unsafe { std::ptr::read_unaligned(base.add(0x20) as *const u32) };
    let header_pitch = unsafe { std::ptr::read_unaligned(base.add(0x24) as *const u32) };

    let format = if header_format != 0 {
        header_format
    } else {
        registry
            .map(|info| info.format)
            .filter(|fmt| *fmt != 0)
            .unwrap_or(0x12)
    };
    let fmt_code = xbox_texture_format_code(format);
    let container_measures = xbox_pixel_container_measures(format, size_field);

    let (width, height) = registry
        .filter(|info| sane_render_target_dims(info.width, info.height))
        .map(|info| (info.width, info.height))
        .or_else(|| {
            if sane_render_target_dims(header_width, header_height) {
                Some((header_width, header_height))
            } else if let Some((w, h, _pitch)) = container_measures {
                Some((w, h))
            } else {
                packed_surface_size_dims(size_field)
            }
        })
        .unwrap_or((640, 480));

    let expected_pitch = texture_pitch_bytes(width, fmt_code).max(width.saturating_mul(4));
    let pitch = registry
        .map(|info| info.pitch)
        .filter(|p| sane_pitch_for_format(*p, width, fmt_code))
        .or_else(|| {
            container_measures
                .map(|(_w, _h, p)| p)
                .filter(|p| sane_pitch_for_format(*p, width, fmt_code))
        })
        .or_else(|| {
            if sane_pitch_for_format(header_pitch, width, fmt_code) {
                Some(header_pitch)
            } else {
                None
            }
        })
        .unwrap_or(expected_pitch);
    let data = normalize_guest_ram_ptr(data_raw & !0x3)
        .or_else(|| registry.map(|info| info.data).filter(|data| *data != 0))
        .unwrap_or(0);
    let key = if data != 0 { data } else { surface };

    let info = HleRenderTargetInfo {
        surface,
        key,
        width,
        height,
        format: fmt_code,
        data,
        pitch,
    };
    upsert_texture_info(HleTextureInfo {
        key: surface,
        width,
        height,
        format: fmt_code,
        data,
        pitch,
        swizzled: false,
    });
    if key != surface {
        upsert_texture_info(HleTextureInfo {
            key,
            width,
            height,
            format: fmt_code,
            data,
            pitch,
            swizzled: false,
        });
    }

    Some(info)
}

fn hle_apply_render_target_to_backend(
    gpu: &mut dyn crate::xbox::gpu::GpuBackend,
    info: HleRenderTargetInfo,
) {
    gpu.set_render_state(crate::xbox::gpu::d3d11::D3D11_HLE_RT_KEY, info.key);
    gpu.set_render_state(
        crate::xbox::gpu::d3d11::D3D11_HLE_RT_SIZE,
        (info.width & 0xFFFF) | ((info.height & 0xFFFF) << 16),
    );
    gpu.set_render_state(crate::xbox::gpu::d3d11::D3D11_HLE_RT_FORMAT, info.format);
    gpu.set_render_state(crate::xbox::gpu::d3d11::D3D11_HLE_RT_DATA, info.data);
    gpu.set_render_state(crate::xbox::gpu::d3d11::D3D11_HLE_RT_PITCH, info.pitch);
    gpu.set_render_state(crate::xbox::gpu::d3d11::D3D11_HLE_RT_COMMIT, 1);
}

fn hle_bind_render_target_to_d3d11(info: HleRenderTargetInfo) {
    store_pending_hle_render_target(info);
    let mut guard = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut gpu) = *guard {
        hle_apply_render_target_to_backend(gpu.as_mut(), info);
        clear_pending_hle_render_target_if_matches(info);
    } else {
        static DEFER_LOG: AtomicU32 = AtomicU32::new(0);
        let n = DEFER_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-RT-DEFER] #{} key=0x{:08X} surface=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
                n, info.key, info.surface, info.data, info.width, info.height, info.format, info.pitch
            ));
        }
    }
}

fn hle_flush_deferred_render_target_to_backend(gpu: &mut dyn crate::xbox::gpu::GpuBackend) {
    let Some(info) = take_pending_hle_render_target() else {
        return;
    };
    hle_apply_render_target_to_backend(gpu, info);
    static FLUSH_LOG: AtomicU32 = AtomicU32::new(0);
    let n = FLUSH_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-RT-DEFER-FLUSH] #{} key=0x{:08X} surface=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
            n, info.key, info.surface, info.data, info.width, info.height, info.format, info.pitch
        ));
    }
}

fn hle_flush_deferred_render_target_to_active_backend() {
    let Some(info) = take_pending_hle_render_target() else {
        return;
    };
    let mut guard = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut gpu) = *guard {
        hle_apply_render_target_to_backend(gpu.as_mut(), info);
        static FLUSH_LOG: AtomicU32 = AtomicU32::new(0);
        let n = FLUSH_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-RT-DEFER-FLUSH-ORDER] #{} key=0x{:08X} surface=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
                n, info.key, info.surface, info.data, info.width, info.height, info.format, info.pitch
            ));
        }
    } else {
        store_pending_hle_render_target(info);
    }
}

fn hle_bind_default_render_target_to_d3d11() {
    let mut guard = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut gpu) = *guard {
        gpu.bind_default_render_target();
    }
}

pub(crate) fn hle_set_render_target(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    static RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static CURRENT_RT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static BACKBUFFER_RT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let requested_rt = args[0];
    let _ = drain_draws_before_state_change(&format!(
        "SetRenderTarget(rt=0x{:08X}, ds=0x{:08X})",
        requested_rt, args[1]
    ));
    let effective_rt = if requested_rt != 0 {
        if BACKBUFFER_RT.load(Ordering::Relaxed) == 0 {
            BACKBUFFER_RT.store(requested_rt, Ordering::Relaxed);
        }
        CURRENT_RT.store(requested_rt, Ordering::Relaxed);
        requested_rt
    } else {
        let current = CURRENT_RT.load(Ordering::Relaxed);
        if current != 0 {
            current
        } else {
            BACKBUFFER_RT.load(Ordering::Relaxed)
        }
    };

    let info = if effective_rt != 0 {
        hle_render_target_info(guest_mem, effective_rt)
    } else {
        None
    };
    if let Some(info) = info {
        hle_bind_render_target_to_d3d11(info);
        let n = RT_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-RT] SetRenderTarget #{} req=0x{:08X} eff=0x{:08X} ds=0x{:08X} surface=0x{:08X} key=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
                n,
                requested_rt,
                effective_rt,
                args[1],
                info.surface,
                info.key,
                info.data,
                info.width,
                info.height,
                info.format,
                info.pitch
            ));
        }
    } else {
        hle_bind_default_render_target_to_d3d11();
        let n = RT_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-RT] SetRenderTarget #{} req=0x{:08X} eff=0x{:08X} ds=0x{:08X} default=1",
                n, requested_rt, effective_rt, args[1]
            ));
        }
    }
    0
}

fn active_texture_bindings() -> &'static [AtomicU32; 4] {
    static ACTIVE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    ACTIVE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn set_active_texture_binding(stage: u32, texture_key: u32) {
    if let Some(slot) = active_texture_bindings().get(stage as usize) {
        slot.store(texture_key, Ordering::Relaxed);
    }
}

fn texture_dirty_epoch() -> &'static AtomicU32 {
    static EPOCH: AtomicU32 = AtomicU32::new(1);
    &EPOCH
}

const HLE_SETTEX_KIND_DDS: u32 = 1;
const HLE_SETTEX_KIND_RAW_BC: u32 = 2;
const HLE_SETTEX_KIND_ARGB: u32 = 3;
const HLE_SETTEX_KIND_FALLBACK: u32 = 4;

fn hle_settex_cache_kind() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_tex() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_data() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_width() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_height() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_format() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_len() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_epoch() -> &'static [AtomicU32; 4] {
    static CACHE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    CACHE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn hle_settex_cache_clear(stage: u32) {
    if stage >= 4 {
        return;
    }
    let idx = stage as usize;
    hle_settex_cache_kind()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_tex()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_data()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_width()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_height()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_format()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_len()[idx].store(0, Ordering::Relaxed);
    hle_settex_cache_epoch()[idx].store(0, Ordering::Relaxed);
}

fn hle_settex_cache_note(
    stage: u32,
    kind: u32,
    tex_key: u32,
    data: u32,
    width: u32,
    height: u32,
    format_code: u32,
    byte_len: u32,
) {
    if stage >= 4 || tex_key == 0 {
        return;
    }
    let idx = stage as usize;
    hle_settex_cache_kind()[idx].store(kind, Ordering::Relaxed);
    hle_settex_cache_tex()[idx].store(tex_key, Ordering::Relaxed);
    hle_settex_cache_data()[idx].store(data, Ordering::Relaxed);
    hle_settex_cache_width()[idx].store(width, Ordering::Relaxed);
    hle_settex_cache_height()[idx].store(height, Ordering::Relaxed);
    hle_settex_cache_format()[idx].store(format_code, Ordering::Relaxed);
    hle_settex_cache_len()[idx].store(byte_len, Ordering::Relaxed);
    hle_settex_cache_epoch()[idx].store(
        texture_dirty_epoch().load(Ordering::Relaxed),
        Ordering::Relaxed,
    );
}

fn hle_settex_cache_is_redundant(
    stage: u32,
    kind: u32,
    tex_key: u32,
    data: u32,
    width: u32,
    height: u32,
    format_code: u32,
    byte_len: u32,
    force_upload: bool,
    source: &'static str,
) -> bool {
    if force_upload || stage >= 4 || tex_key == 0 {
        return false;
    }
    let idx = stage as usize;
    let epoch = texture_dirty_epoch().load(Ordering::Relaxed);
    let redundant = hle_settex_cache_kind()[idx].load(Ordering::Relaxed) == kind
        && hle_settex_cache_tex()[idx].load(Ordering::Relaxed) == tex_key
        && hle_settex_cache_data()[idx].load(Ordering::Relaxed) == data
        && hle_settex_cache_width()[idx].load(Ordering::Relaxed) == width
        && hle_settex_cache_height()[idx].load(Ordering::Relaxed) == height
        && hle_settex_cache_format()[idx].load(Ordering::Relaxed) == format_code
        && hle_settex_cache_len()[idx].load(Ordering::Relaxed) == byte_len
        && hle_settex_cache_epoch()[idx].load(Ordering::Relaxed) == epoch;
    if redundant {
        static SKIP_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SKIP_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-SETTEX-CACHE-SKIP] #{} source={} stage={} tex=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X} bytes={} epoch={}",
                n, source, stage, tex_key, data, width, height, format_code, byte_len, epoch
            ));
        }
    }
    redundant
}

#[derive(Clone, Copy, Debug, Default)]
struct HlePaletteInfo {
    key: u32,
    data: u32,
    entries: u32,
}

fn palette_registry() -> &'static Mutex<Vec<HlePaletteInfo>> {
    static REGISTRY: OnceLock<Mutex<Vec<HlePaletteInfo>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

fn active_palette_bindings() -> &'static [AtomicU32; 4] {
    static ACTIVE: OnceLock<[AtomicU32; 4]> = OnceLock::new();
    ACTIVE.get_or_init(|| {
        [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ]
    })
}

fn palette_entry_count(size: u32) -> u32 {
    match size & 0x3 {
        0 => 256,
        1 => 128,
        2 => 64,
        _ => 32,
    }
}

fn palette_size_from_common(common: u32) -> u32 {
    (common >> 30) & 0x3
}

fn upsert_palette_info(info: HlePaletteInfo) {
    if info.key == 0 || info.data == 0 {
        return;
    }
    let mut reg = palette_registry().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = reg.iter_mut().find(|entry| entry.key == info.key) {
        *existing = info;
        return;
    }
    if reg.len() < 64 {
        reg.push(info);
    }
}

fn lookup_palette_info(key: u32) -> Option<HlePaletteInfo> {
    let reg = palette_registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.iter().find(|entry| entry.key == key).copied()
}

fn palette_info_from_header(guest_mem: *mut u8, palette_ptr: u32) -> Option<HlePaletteInfo> {
    let key = normalize_guest_ram_ptr(palette_ptr)?;
    if key.saturating_add(12) > 0x2000_0000 {
        return None;
    }
    let common = unsafe { std::ptr::read_unaligned(guest_mem.add(key as usize) as *const u32) };
    let data_raw =
        unsafe { std::ptr::read_unaligned(guest_mem.add(key as usize + 0x04) as *const u32) };
    let data = normalize_guest_ram_ptr(data_raw & !0x3)?;
    let entries = palette_entry_count(palette_size_from_common(common));
    if data.saturating_add(entries.saturating_mul(4)) > 0x2000_0000 {
        return None;
    }
    Some(HlePaletteInfo { key, data, entries })
}

fn set_active_palette_binding(stage: u32, palette_key: u32) {
    if let Some(slot) = active_palette_bindings().get(stage as usize) {
        slot.store(palette_key, Ordering::Relaxed);
    }
}

fn active_palette_entries(stage: u32, guest_mem: *mut u8) -> Option<[u32; 256]> {
    let key = active_palette_bindings()
        .get(stage as usize)?
        .load(Ordering::Relaxed);
    let info = lookup_palette_info(key).or_else(|| palette_info_from_header(guest_mem, key))?;
    let mut out = [0u32; 256];
    let entries = info.entries.min(256) as usize;
    unsafe {
        let src = guest_mem.add(info.data as usize) as *const u32;
        for (i, dst) in out.iter_mut().take(entries).enumerate() {
            *dst = std::ptr::read_unaligned(src.add(i));
        }
    }
    Some(out)
}

fn hle_create_palette(args: &[u32; 8], guest_mem: *mut u8, returns_object: bool) -> u32 {
    let size_enum = args[0] & 0x3;
    let pp_palette = if returns_object { 0 } else { args[1] };
    let entries = palette_entry_count(size_enum);
    let pal = alloc_fake_d3d_obj(guest_mem, 0x20);
    let data = alloc_fake_d3d_obj(guest_mem, entries.saturating_mul(4));
    if pal == 0 || data == 0 {
        return 0x8876_000E; // D3DERR_OUTOFVIDEOMEMORY-style failure.
    }
    let pal_key = normalize_guest_ram_ptr(pal).unwrap_or(pal);
    let data_key = normalize_guest_ram_ptr(data).unwrap_or(data);
    unsafe {
        let base = guest_mem.add(pal as usize);
        std::ptr::write_unaligned(base.add(0x00) as *mut u32, 0x0003_0001 | (size_enum << 30));
        std::ptr::write_unaligned(base.add(0x04) as *mut u32, data | 0x8000_0000);
        std::ptr::write_unaligned(base.add(0x08) as *mut u32, 0);
        std::ptr::write_bytes(guest_mem.add(data as usize), 0xFF, entries as usize * 4);
    }
    upsert_palette_info(HlePaletteInfo {
        key: pal_key,
        data: data_key,
        entries,
    });
    static PAL_CREATE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PAL_CREATE_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 {
        debug_log(&format!(
            "[HLE-PAL] CreatePalette #{} size_enum={} entries={} -> pal=0x{:08X} data=0x{:08X}",
            n, size_enum, entries, pal, data_key
        ));
    }
    if returns_object {
        pal
    } else {
        if let Some(pp_palette_norm) = guest_ram_offset(pp_palette) {
            unsafe {
                std::ptr::write_unaligned(guest_mem.add(pp_palette_norm) as *mut u32, pal);
            }
        }
        0
    }
}

fn hle_palette_lock(name: &str, args: &[u32; 8], context_rcx: u32, guest_mem: *mut u8) -> u32 {
    let palette_ptr = normalize_guest_ram_ptr(context_rcx)
        .filter(|key| lookup_palette_info(*key).is_some())
        .unwrap_or_else(|| normalize_guest_ram_ptr(args[0]).unwrap_or(args[0]));
    let info = lookup_palette_info(palette_ptr)
        .or_else(|| palette_info_from_header(guest_mem, palette_ptr))
        .unwrap_or_default();
    if info.key == 0 || info.data == 0 {
        return 0;
    }
    upsert_palette_info(info);
    static PAL_LOCK_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PAL_LOCK_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 {
        debug_log(&format!(
            "[HLE-PAL] {} #{} pal=0x{:08X} data=0x{:08X} entries={}",
            name, n, info.key, info.data, info.entries
        ));
    }
    if name == "D3DPalette_Lock" {
        let pp_colors = args[1];
        if let Some(pp_colors_norm) = guest_ram_offset(pp_colors) {
            unsafe {
                std::ptr::write_unaligned(guest_mem.add(pp_colors_norm) as *mut u32, info.data);
            }
        }
        0
    } else {
        info.data
    }
}

fn hle_set_palette(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let stage = args[0];
    let palette_raw = args[1];
    let palette_key = normalize_guest_ram_ptr(palette_raw).unwrap_or(0);
    set_active_palette_binding(stage, palette_key);
    if let Some(info) = palette_info_from_header(guest_mem, palette_key) {
        upsert_palette_info(info);
    }
    static PAL_SET_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PAL_SET_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 24 {
        debug_log(&format!(
            "[HLE-PAL] SetPalette #{} stage={} pal=0x{:08X}->0x{:08X}",
            n, stage, palette_raw, palette_key
        ));
    }

    if let Some(tex_key) = active_texture_bindings().get(stage as usize) {
        let tex = tex_key.load(Ordering::Relaxed);
        if tex != 0 {
            let mut settex = [0u32; 8];
            settex[0] = stage;
            settex[1] = tex;
            hle_set_texture_impl(&settex, guest_mem, true);
        }
    }
    0
}

fn refresh_active_texture_upload_for_data(data_addr: u32, guest_mem: *mut u8) -> u32 {
    let mut refreshed = 0u32;
    for (stage, slot) in active_texture_bindings().iter().enumerate() {
        let tex_key = slot.load(Ordering::Relaxed);
        if tex_key == 0 {
            continue;
        }
        let Some(info) = lookup_texture_info(tex_key) else {
            continue;
        };
        let Some(texture_data) = normalize_guest_ram_ptr(info.data & !0x3) else {
            continue;
        };
        let byte_len = texture_byte_len(info.width, info.height, info.format) as u32;
        let texture_end = texture_data.saturating_add(byte_len.max(1));
        if data_addr < texture_data || data_addr >= texture_end {
            continue;
        }

        let mut args = [0u32; 8];
        args[0] = stage as u32;
        args[1] = tex_key;
        hle_set_texture_impl(&args, guest_mem, true);
        refreshed += 1;

        static REFRESH_LOG: AtomicU32 = AtomicU32::new(0);
        let n = REFRESH_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-XGSWIZZLE-UPLOAD] #{} stage={} tex=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X}",
                n, stage, tex_key, data_addr, info.width, info.height, info.format
            ));
        }
    }
    refreshed
}

fn hle_texture_info_from_surface(guest_mem: *mut u8, surface_addr: u32) -> Option<HleTextureInfo> {
    let key = normalize_guest_ram_ptr(surface_addr)?;
    if key.saturating_add(0x28) > 0x2000_0000 {
        return None;
    }

    let registry = lookup_texture_info(key);
    let base = unsafe { guest_mem.add(key as usize) };
    let header_data = unsafe { std::ptr::read_unaligned(base.add(0x04) as *const u32) };
    let header_format = unsafe { std::ptr::read_unaligned(base.add(0x0C) as *const u32) };
    let size_field = unsafe { std::ptr::read_unaligned(base.add(0x10) as *const u32) };
    let header_width = unsafe { std::ptr::read_unaligned(base.add(0x1C) as *const u32) };
    let header_height = unsafe { std::ptr::read_unaligned(base.add(0x20) as *const u32) };
    let header_pitch = unsafe { std::ptr::read_unaligned(base.add(0x24) as *const u32) };

    let format = if header_format != 0 {
        header_format
    } else {
        registry
            .map(|info| info.format)
            .filter(|fmt| *fmt != 0)
            .unwrap_or(0x12)
    };
    let fmt_code = xbox_texture_format_code(format);
    let container_measures = xbox_pixel_container_measures(format, size_field);
    let width = registry
        .map(|info| info.width)
        .filter(|w| *w != 0)
        .or_else(|| (header_width != 0).then_some(header_width))
        .or_else(|| container_measures.map(|(w, _, _)| w))?;
    let height = registry
        .map(|info| info.height)
        .filter(|h| *h != 0)
        .or_else(|| (header_height != 0).then_some(header_height))
        .or_else(|| container_measures.map(|(_, h, _)| h))?;
    if !sane_texture_dims(width, height) {
        return None;
    }
    let pitch = registry
        .map(|info| info.pitch)
        .filter(|p| sane_pitch_for_format(*p, width, fmt_code))
        .or_else(|| {
            if sane_pitch_for_format(header_pitch, width, fmt_code) {
                Some(header_pitch)
            } else {
                None
            }
        })
        .or_else(|| container_measures.map(|(_, _, p)| p))
        .unwrap_or_else(|| texture_pitch_bytes(width, format));
    let data = normalize_guest_ram_ptr(header_data & !0x3)
        .or_else(|| registry.map(|info| info.data & !0x3))
        .unwrap_or(0);

    Some(HleTextureInfo {
        key,
        width,
        height,
        format,
        data,
        pitch,
        swizzled: registry.map(|info| info.swizzled).unwrap_or(false),
    })
}

fn read_rect_u32(
    guest_mem: *mut u8,
    rects: u32,
    index: u32,
    fallback_w: u32,
    fallback_h: u32,
) -> (u32, u32, u32, u32) {
    if rects == 0 || rects >= 0x2000_0000u32.saturating_sub(16) {
        return (0, 0, fallback_w, fallback_h);
    }
    let base = rects.saturating_add(index.saturating_mul(16));
    if base >= 0x2000_0000u32.saturating_sub(16) {
        return (0, 0, fallback_w, fallback_h);
    }
    unsafe {
        let p = guest_mem.add(base as usize);
        let left = std::ptr::read_unaligned(p as *const u32);
        let top = std::ptr::read_unaligned(p.add(4) as *const u32);
        let right = std::ptr::read_unaligned(p.add(8) as *const u32);
        let bottom = std::ptr::read_unaligned(p.add(12) as *const u32);
        (left, top, right, bottom)
    }
}

fn read_point_u32(guest_mem: *mut u8, points: u32, index: u32) -> (u32, u32) {
    if points == 0 || points >= 0x2000_0000u32.saturating_sub(8) {
        return (0, 0);
    }
    let base = points.saturating_add(index.saturating_mul(8));
    if base >= 0x2000_0000u32.saturating_sub(8) {
        return (0, 0);
    }
    unsafe {
        let p = guest_mem.add(base as usize);
        (
            std::ptr::read_unaligned(p as *const u32),
            std::ptr::read_unaligned(p.add(4) as *const u32),
        )
    }
}

pub(crate) fn hle_copy_rects(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let src_surface = args[0];
    let src_rects = args[1];
    let rect_count = args[2].max(1).min(256);
    let dst_surface = args[3];
    let dst_points = args[4];

    let Some(src) = hle_texture_info_from_surface(guest_mem, src_surface) else {
        debug_log(&format!(
            "[HLE-COPYRECTS-MISS] src=0x{:08X} rects=0x{:08X} count={} dst=0x{:08X} points=0x{:08X}",
            src_surface, src_rects, args[2], dst_surface, dst_points
        ));
        return 0;
    };
    let Some(dst) = hle_texture_info_from_surface(guest_mem, dst_surface) else {
        debug_log(&format!(
            "[HLE-COPYRECTS-MISS] src=0x{:08X} rects=0x{:08X} count={} dst=0x{:08X} points=0x{:08X} src_key=0x{:08X}",
            src_surface, src_rects, args[2], dst_surface, dst_points, src.key
        ));
        return 0;
    };

    let src_fmt = xbox_texture_format_code(src.format);
    let dst_fmt = xbox_texture_format_code(dst.format);
    if src_fmt != dst_fmt || src.data == 0 || dst.data == 0 {
        debug_log(&format!(
            "[HLE-COPYRECTS-SKIP] src=0x{:08X} {}x{} fmt=0x{:02X} data=0x{:08X} dst=0x{:08X} {}x{} fmt=0x{:02X} data=0x{:08X}",
            src.key, src.width, src.height, src_fmt, src.data, dst.key, dst.width, dst.height, dst_fmt, dst.data
        ));
        return 0;
    }

    let mut copied = 0u32;
    let block_compressed = crate::xbox::gpu::texture_format::is_block_compressed(src_fmt);
    unsafe {
        for i in 0..rect_count {
            let (left, top, right, bottom) = read_rect_u32(
                guest_mem,
                src_rects,
                i,
                src.width.min(dst.width),
                src.height.min(dst.height),
            );
            let (dst_x, dst_y) = read_point_u32(guest_mem, dst_points, i);
            if right <= left || bottom <= top {
                continue;
            }
            let rect_w = right
                .saturating_sub(left)
                .min(src.width.saturating_sub(left));
            let rect_h = bottom
                .saturating_sub(top)
                .min(src.height.saturating_sub(top));
            if rect_w == 0 || rect_h == 0 || dst_x >= dst.width || dst_y >= dst.height {
                continue;
            }
            let copy_w = rect_w.min(dst.width.saturating_sub(dst_x));
            let copy_h = rect_h.min(dst.height.saturating_sub(dst_y));
            if copy_w == 0 || copy_h == 0 {
                continue;
            }

            if block_compressed {
                let bytes_per_block =
                    crate::xbox::gpu::texture_format::block_bytes(src_fmt).unwrap_or(16);
                let src_row_pitch = texture_pitch_bytes(src.width, src_fmt);
                let dst_row_pitch = texture_pitch_bytes(dst.width, dst_fmt);
                let src_bx = left / 4;
                let src_by = top / 4;
                let dst_bx = dst_x / 4;
                let dst_by = dst_y / 4;
                let block_w = (copy_w + 3) / 4;
                let block_h = (copy_h + 3) / 4;
                let row_bytes = block_w.saturating_mul(bytes_per_block);
                for by in 0..block_h {
                    let src_off = src_by
                        .saturating_add(by)
                        .saturating_mul(src_row_pitch)
                        .saturating_add(src_bx.saturating_mul(bytes_per_block));
                    let dst_off = dst_by
                        .saturating_add(by)
                        .saturating_mul(dst_row_pitch)
                        .saturating_add(dst_bx.saturating_mul(bytes_per_block));
                    let src_addr = src.data.saturating_add(src_off);
                    let dst_addr = dst.data.saturating_add(dst_off);
                    if src_addr as u64 + row_bytes as u64 <= 0x2000_0000
                        && dst_addr as u64 + row_bytes as u64 <= 0x2000_0000
                    {
                        std::ptr::copy_nonoverlapping(
                            guest_mem.add(src_addr as usize),
                            guest_mem.add(dst_addr as usize),
                            row_bytes as usize,
                        );
                    }
                }
            } else if let Some(bpp) = texture_bytes_per_pixel(src_fmt) {
                let src_pitch = src.pitch.max(src.width.saturating_mul(bpp));
                let dst_pitch = dst.pitch.max(dst.width.saturating_mul(bpp));
                let row_bytes = copy_w.saturating_mul(bpp);
                for y in 0..copy_h {
                    let src_addr = src
                        .data
                        .saturating_add(top.saturating_add(y).saturating_mul(src_pitch))
                        .saturating_add(left.saturating_mul(bpp));
                    let dst_addr = dst
                        .data
                        .saturating_add(dst_y.saturating_add(y).saturating_mul(dst_pitch))
                        .saturating_add(dst_x.saturating_mul(bpp));
                    if src_addr as u64 + row_bytes as u64 <= 0x2000_0000
                        && dst_addr as u64 + row_bytes as u64 <= 0x2000_0000
                    {
                        std::ptr::copy_nonoverlapping(
                            guest_mem.add(src_addr as usize),
                            guest_mem.add(dst_addr as usize),
                            row_bytes as usize,
                        );
                    }
                }
            }
            copied = copied.saturating_add(1);
        }
    }

    if copied != 0 {
        upsert_texture_info(dst);
        mark_texture_dirty_from_lock(dst, "CopyRects");
        let _ = refresh_active_texture_upload_for_data(dst.data, guest_mem);
    }

    static COPYRECTS_LOG: AtomicU32 = AtomicU32::new(0);
    let n = COPYRECTS_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() || copied != 0 {
        debug_log(&format!(
            "[HLE-COPYRECTS] #{} copied={} req_count={} src=0x{:08X} {}x{} fmt=0x{:02X} data=0x{:08X} dst=0x{:08X} {}x{} fmt=0x{:02X} data=0x{:08X} rects=0x{:08X} points=0x{:08X}",
            n,
            copied,
            args[2],
            src.key,
            src.width,
            src.height,
            src_fmt,
            src.data,
            dst.key,
            dst.width,
            dst.height,
            dst_fmt,
            dst.data,
            src_rects,
            dst_points
        ));
    }

    0
}

fn write_fake_texture_header(
    guest_mem: *mut u8,
    texture_addr: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
) {
    let Some(key) = normalize_guest_ram_ptr(texture_addr) else {
        return;
    };
    let width = width.clamp(1, 4096);
    let height = height.clamp(1, 4096);
    let pitch = texture_pitch_bytes(width, format);
    unsafe {
        let base = guest_mem.add(key as usize);
        *(base as *mut u32) = 0x0003_0001; // X_D3DCOMMON_TYPE_TEXTURE | refcount 1
        *(base.add(0x04) as *mut u32) = data;
        *(base.add(0x08) as *mut u32) = 0;
        *(base.add(0x0C) as *mut u32) = format;
        *(base.add(0x10) as *mut u32) = pack_texture_size(width, height);
        *(base.add(0x1C) as *mut u32) = width;
        *(base.add(0x20) as *mut u32) = height;
        *(base.add(0x24) as *mut u32) = pitch;
    }
    upsert_texture_info(HleTextureInfo {
        key,
        width,
        height,
        format,
        data,
        pitch,
        swizzled: false,
    });
}

fn write_surface_desc(
    guest_mem: *mut u8,
    surface_addr: u32,
    out_ptr: u32,
) -> Option<(u32, u32, u32, u32)> {
    let out_norm = guest_ram_offset(out_ptr)?;

    let key = normalize_guest_ram_ptr(surface_addr)?;
    let header = unsafe { guest_mem.add(key as usize) };
    let registry = lookup_texture_info(key);
    let header_format = unsafe { std::ptr::read_unaligned(header.add(0x0C) as *const u32) };
    let size_field = unsafe { std::ptr::read_unaligned(header.add(0x10) as *const u32) };
    let header_width = unsafe { std::ptr::read_unaligned(header.add(0x1C) as *const u32) };
    let header_height = unsafe { std::ptr::read_unaligned(header.add(0x20) as *const u32) };

    let format = registry
        .map(|info| info.format)
        .filter(|fmt| *fmt != 0)
        .unwrap_or(header_format);
    let width = registry
        .map(|info| info.width)
        .filter(|w| (1..=4096).contains(w))
        .or_else(|| {
            if (1..=4096).contains(&header_width) {
                Some(header_width)
            } else if size_field != 0 {
                Some((size_field & 0x0FFF).saturating_add(1))
            } else {
                None
            }
        })?;
    let height = registry
        .map(|info| info.height)
        .filter(|h| (1..=4096).contains(h))
        .or_else(|| {
            if (1..=4096).contains(&header_height) {
                Some(header_height)
            } else if size_field != 0 {
                Some(((size_field >> 12) & 0x0FFF).saturating_add(1))
            } else {
                None
            }
        })?;

    let fmt_code = xbox_texture_format_code(format);
    let size = texture_byte_len(width, height, fmt_code);
    unsafe {
        let base = guest_mem.add(out_norm);
        std::ptr::write_unaligned(base.add(0x00) as *mut u32, fmt_code);
        std::ptr::write_unaligned(base.add(0x04) as *mut u32, 1); // X_D3DRTYPE_SURFACE
        std::ptr::write_unaligned(base.add(0x08) as *mut u32, 0); // Usage
        std::ptr::write_unaligned(base.add(0x0C) as *mut u32, size);
        std::ptr::write_unaligned(base.add(0x10) as *mut u32, 0); // MultiSampleType
        std::ptr::write_unaligned(base.add(0x14) as *mut u32, width);
        std::ptr::write_unaligned(base.add(0x18) as *mut u32, height);
    }
    Some((fmt_code, width, height, size))
}

fn xbox_texture_format_code(format: u32) -> u32 {
    crate::xbox::gpu::texture_format::format_code(format)
}

fn texture_pitch_bytes(width: u32, format: u32) -> u32 {
    crate::xbox::gpu::texture_format::texture_pitch_bytes(width, xbox_texture_format_code(format))
}

fn texture_byte_len(width: u32, height: u32, format: u32) -> u32 {
    crate::xbox::gpu::texture_format::texture_byte_len(
        width,
        height,
        xbox_texture_format_code(format),
    )
}

fn texture_mip_level_count(width: u32, height: u32) -> u32 {
    let mut w = width.max(1);
    let mut h = height.max(1);
    let mut levels = 1u32;
    while (w > 1 || h > 1) && levels < 32 {
        w = (w >> 1).max(1);
        h = (h >> 1).max(1);
        levels += 1;
    }
    levels
}

fn texture_mip_dims(width: u32, height: u32, level: u32) -> (u32, u32, u32) {
    let levels = texture_mip_level_count(width, height);
    let effective_level = level.min(levels.saturating_sub(1));
    (
        (width.max(1) >> effective_level).max(1),
        (height.max(1) >> effective_level).max(1),
        effective_level,
    )
}

fn texture_mip_offset(width: u32, height: u32, format: u32, level: u32) -> u32 {
    let (_mip_w, _mip_h, effective_level) = texture_mip_dims(width, height, level);
    let mut offset = 0u32;
    for mip in 0..effective_level {
        let (w, h, _) = texture_mip_dims(width, height, mip);
        offset = offset.saturating_add(texture_byte_len(w, h, format));
    }
    offset
}

fn texture_mip_chain_byte_len(width: u32, height: u32, format: u32) -> u32 {
    let levels = texture_mip_level_count(width, height);
    let mut len = 0u32;
    for mip in 0..levels {
        let (w, h, _) = texture_mip_dims(width, height, mip);
        len = len.saturating_add(texture_byte_len(w, h, format));
    }
    len.max(texture_byte_len(width, height, format))
}

fn texture_bytes_per_pixel(format_code: u32) -> Option<u32> {
    crate::xbox::gpu::texture_format::bytes_per_pixel(format_code)
}

fn is_linear_texture_format(format_code: u32) -> bool {
    crate::xbox::gpu::texture_format::is_linear(format_code)
}

fn xg_swizzled_pixel_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    fn build_masks(width: u32, height: u32) -> (u32, u32) {
        let mut x_mask = 0u32;
        let mut y_mask = 0u32;
        let mut x_remaining = width;
        let mut y_remaining = height;
        let mut bit = 1u32;
        while x_remaining > 1 || y_remaining > 1 {
            if x_remaining > 1 {
                x_mask |= bit;
                x_remaining >>= 1;
                bit <<= 1;
            }
            if y_remaining > 1 {
                y_mask |= bit;
                y_remaining >>= 1;
                bit <<= 1;
            }
        }
        (x_mask, y_mask)
    }

    fn spread(mut value: u32, mut mask: u32) -> u32 {
        let mut result = 0u32;
        while mask != 0 {
            let low = mask & mask.wrapping_neg();
            if (value & 1) != 0 {
                result |= low;
            }
            value >>= 1;
            mask &= mask - 1;
        }
        result
    }

    let (x_mask, y_mask) = build_masks(width, height);
    spread(x, x_mask) | spread(y, y_mask)
}

pub(super) fn hle_xg_swizzle_rect(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    // XDK: XGSwizzleRect(pSource, Pitch, pRect, pDest, Width, Height, pPoint, BytesPerPixel).
    // The common Spider-Man path passes pRect=NULL, pPoint=NULL, Pitch=0 for
    // whole-texture swizzles into a D3DTexture_LockRect pBits buffer.
    let Some(src_base) = normalize_guest_ram_ptr(args[0]) else {
        return 0;
    };
    let pitch_arg = args[1];
    let rect_ptr = args[2];
    let Some(dst_base) = normalize_guest_ram_ptr(args[3]) else {
        return 0;
    };
    let width = args[4];
    let height = args[5];
    let point_ptr = args[6];
    let bpp = args[7];

    if !sane_texture_dims(width, height) || !matches!(bpp, 1 | 2 | 4) {
        return 0;
    }
    if !width.is_power_of_two() || !height.is_power_of_two() {
        return 0;
    }

    let (src_x, src_y, rect_w, rect_h) = if rect_ptr != 0 {
        let Some(rect_addr) = normalize_guest_ram_ptr(rect_ptr) else {
            return 0;
        };
        if rect_addr > 0x1FFF_FFF0 {
            return 0;
        }
        let (left, top, right, bottom) = unsafe {
            let p = guest_mem.add(rect_addr as usize);
            (
                std::ptr::read_unaligned(p as *const i32).max(0) as u32,
                std::ptr::read_unaligned(p.add(4) as *const i32).max(0) as u32,
                std::ptr::read_unaligned(p.add(8) as *const i32).max(0) as u32,
                std::ptr::read_unaligned(p.add(12) as *const i32).max(0) as u32,
            )
        };
        if right <= left || bottom <= top {
            return 0;
        }
        (left, top, right - left, bottom - top)
    } else {
        (0, 0, width, height)
    };

    let (dst_x, dst_y) = if point_ptr != 0 {
        let Some(point_addr) = normalize_guest_ram_ptr(point_ptr) else {
            return 0;
        };
        if point_addr > 0x1FFF_FFF8 {
            return 0;
        }
        unsafe {
            let p = guest_mem.add(point_addr as usize);
            (
                std::ptr::read_unaligned(p as *const i32).max(0) as u32,
                std::ptr::read_unaligned(p.add(4) as *const i32).max(0) as u32,
            )
        }
    } else {
        (0, 0)
    };

    if dst_x.saturating_add(rect_w) > width || dst_y.saturating_add(rect_h) > height {
        return 0;
    }

    let pitch = if pitch_arg != 0 {
        pitch_arg
    } else {
        width.saturating_mul(bpp)
    };
    let src_end = src_base as usize
        + (src_y as usize + rect_h.saturating_sub(1) as usize).saturating_mul(pitch as usize)
        + (src_x as usize + rect_w as usize).saturating_mul(bpp as usize);
    let dst_len = (width as usize)
        .saturating_mul(height as usize)
        .saturating_mul(bpp as usize);
    let dst_end = dst_base as usize + dst_len;
    if rect_w == 0 || rect_h == 0 || src_end > 0x2000_0000 || dst_end > 0x2000_0000 || dst_len == 0
    {
        return 0;
    }

    let doom_staging_refreshed = crate::xbox::worker::doom_refresh_d3d_staging_source_for_xg(
        guest_mem, src_base, width, height, bpp,
    );

    let mut nonzero_src = false;
    let mut nonzero_dst = false;
    unsafe {
        let src_mem = guest_mem.add(src_base as usize);
        let dst_mem = guest_mem.add(dst_base as usize);
        for y in 0..rect_h {
            let src_row =
                src_mem.add((src_y + y) as usize * pitch as usize + src_x as usize * bpp as usize);
            for x in 0..rect_w {
                let src = src_row.add(x as usize * bpp as usize);
                let dst_pixel = xg_swizzled_pixel_index(dst_x + x, dst_y + y, width, height);
                let dst = dst_mem.add(dst_pixel as usize * bpp as usize);
                for byte_i in 0..bpp as usize {
                    let v = *src.add(byte_i);
                    nonzero_src |= v != 0;
                    *dst.add(byte_i) = v;
                    nonzero_dst |= v != 0;
                }
            }
        }
    }

    let refreshed_stages = if nonzero_dst {
        refresh_active_texture_upload_for_data(dst_base, guest_mem)
    } else {
        0
    };

    static XGSWIZZLE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = XGSWIZZLE_LOG.fetch_add(1, Ordering::Relaxed);
    let doom_swizzle = if doom_blit_diag_enabled() {
        Some(doom_record_xg_swizzle(
            guest_mem, n, args[0], src_base, args[3], dst_base, pitch_arg, pitch, width, height,
            bpp, src_x, src_y, rect_w, rect_h, dst_x, dst_y,
        ))
    } else {
        None
    };
    if n < 32 || n.is_power_of_two() {
        let doom_source_w = doom_swizzle.map(|s| s.doom_source_w).unwrap_or(0);
        let doom_source_h = doom_swizzle.map(|s| s.doom_source_h).unwrap_or(0);
        let doom_pitch_guess = doom_swizzle.map(|s| s.doom_source_pitch_guess).unwrap_or(0);
        let src_resolved_hash = doom_swizzle.map(|s| s.src_resolved_hash).unwrap_or(0);
        let src_doom_pitch_hash = doom_swizzle.map(|s| s.src_doom_pitch_hash).unwrap_or(0);
        debug_log(&format!(
            "[HLE-XGSWIZZLE] #{} src=0x{:08X}->0x{:08X} dst=0x{:08X}->0x{:08X} \
             rect=({},{} {}x{}) point=({}, {}) tex={}x{} raw_pitch_arg={} pitch_resolved={} bpp={} doom_source={}x{} doom_pitch_guess={} src_hash_resolved=0x{:016X} src_hash_doom_pitch=0x{:016X} src_nonzero={} dst_nonzero={} refreshed={} doom_staging_refreshed={}",
            n,
            args[0],
            src_base,
            args[3],
            dst_base,
            src_x,
            src_y,
            rect_w,
            rect_h,
            dst_x,
            dst_y,
            width,
            height,
            pitch_arg,
            pitch,
            bpp,
            doom_source_w,
            doom_source_h,
            doom_pitch_guess,
            src_resolved_hash,
            src_doom_pitch_hash,
            nonzero_src,
            nonzero_dst,
            refreshed_stages,
            doom_staging_refreshed as u8
        ));
    }
    0
}

fn read_guest_dds_info(
    guest_mem: *mut u8,
    data_addr: u32,
) -> Option<(u32, u32, u32, usize, usize)> {
    if data_addr > 0x1FFF_FF80 {
        return None;
    }
    unsafe {
        let base = guest_mem.add(data_addr as usize);
        if std::ptr::read_unaligned(base as *const u32) != 0x2053_4444 {
            return None;
        }
        let header_size = std::ptr::read_unaligned(base.add(4) as *const u32);
        if header_size != 124 {
            return None;
        }
        let height = std::ptr::read_unaligned(base.add(12) as *const u32);
        let width = std::ptr::read_unaligned(base.add(16) as *const u32);
        if !sane_texture_dims(width, height) {
            return None;
        }
        let fourcc = std::ptr::read_unaligned(base.add(84) as *const u32);
        let format_code = match fourcc {
            0x3154_5844 => 0x0C, // DXT1
            0x3354_5844 => 0x0E, // DXT3
            0x3554_5844 => 0x0F, // DXT5
            _ => return None,
        };
        let bytes_per_block = if format_code == 0x0C { 8usize } else { 16usize };
        let block_width = ((width + 3) / 4) as usize;
        let block_height = ((height + 3) / 4) as usize;
        let byte_len = block_width
            .saturating_mul(block_height)
            .saturating_mul(bytes_per_block);
        let payload_offset = 0x80usize;
        let end = data_addr as usize + payload_offset + byte_len;
        if byte_len == 0 || end > 0x2000_0000 {
            return None;
        }
        Some((width, height, format_code, payload_offset, byte_len))
    }
}

fn upload_xgrph_embedded_a1r5g5b5(
    guest_mem: *mut u8,
    src_ptr: u32,
    dst_texture: u32,
) -> Option<(u32, u32, u32)> {
    let src = normalize_guest_ram_ptr(src_ptr)?;
    if src > 0x1FFF_FFE0 {
        return None;
    }

    let (common, dims) = unsafe {
        let base = guest_mem.add(src as usize);
        (
            std::ptr::read_unaligned(base as *const u32),
            std::ptr::read_unaligned(base.add(0x0C) as *const u32),
        )
    };

    let width = dims & 0xFFFF;
    let height = (dims >> 16) & 0xFFFF;
    if common != 0x0002_0000 || !sane_texture_dims(width, height) {
        return None;
    }

    let src_pixels = src.checked_add(0x10)?;
    let pixel_count = width.checked_mul(height)?;
    let src_len = pixel_count.checked_mul(2)?;
    if src_pixels.checked_add(src_len)? > 0x2000_0000 {
        return None;
    }

    let dst_bytes = pixel_count.checked_mul(4)?;
    let dst_data = alloc_fake_d3d_obj(guest_mem, dst_bytes);
    if dst_data == 0 {
        return None;
    }

    let mut alpha_pixels = 0u32;
    unsafe {
        let src_base = guest_mem.add(src_pixels as usize) as *const u16;
        let dst_base = guest_mem.add(dst_data as usize) as *mut u32;
        for y in 0..height as usize {
            let src_y = height as usize - 1 - y;
            for x in 0..width as usize {
                let i = y * width as usize + x;
                let src_i = src_y * width as usize + x;
                let px = std::ptr::read_unaligned(src_base.add(src_i));
                let alpha = if (px & 0x8000) != 0 { 0xFFu32 } else { 0u32 };
                if alpha != 0 {
                    alpha_pixels += 1;
                }
                let r = (((px >> 10) & 0x1F) as u32 * 255) / 31;
                let g = (((px >> 5) & 0x1F) as u32 * 255) / 31;
                let b = ((px & 0x1F) as u32 * 255) / 31;
                let out = if alpha != 0 {
                    (alpha << 24) | (r << 16) | (g << 8) | b
                } else {
                    0
                };
                std::ptr::write_unaligned(dst_base.add(i), out);
            }
        }
    }

    if alpha_pixels == 0 {
        return None;
    }

    write_fake_texture_header(guest_mem, dst_texture, width, height, 0x12, dst_data);
    if let Some(key) = normalize_guest_ram_ptr(dst_texture) {
        upsert_texture_info(HleTextureInfo {
            key,
            width,
            height,
            format: 0x12,
            data: dst_data,
            pitch: width.saturating_mul(4),
            swizzled: false,
        });
    }
    Some((width, height, dst_data))
}

/// Scratch pushbuffer cursor used by D3DDevice_BeginPush/EndPush HLE.
///
/// The guest writes directly to the pointer returned by BeginPush. Returning
/// PB_DUMMY_BASE every time makes each tiny packet overwrite the previous one,
/// so EndPush repeatedly parses one 116-byte glyph packet instead of the full
/// command stream.
static BEGIN_PUSH_CURSOR: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(PB_DUMMY_BASE);
static BEGIN_PUSH_LAST_START: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn reset_begin_push_cursor() {
    BEGIN_PUSH_CURSOR.store(PB_DUMMY_BASE, Ordering::Relaxed);
    BEGIN_PUSH_LAST_START.store(0, Ordering::Relaxed);
}

fn reserve_begin_push_space(count_dwords: u32) -> u32 {
    let reserve = count_dwords.saturating_mul(4).max(4);
    loop {
        let cur = BEGIN_PUSH_CURSOR.load(Ordering::Relaxed);
        let in_range = cur >= PB_DUMMY_BASE && cur < PB_DUMMY_BASE + PB_DUMMY_SIZE;
        let next = if in_range && cur.saturating_add(reserve) <= PB_DUMMY_BASE + PB_DUMMY_SIZE {
            cur + reserve
        } else {
            PB_DUMMY_BASE + reserve
        };
        let dest = if in_range && cur.saturating_add(reserve) <= PB_DUMMY_BASE + PB_DUMMY_SIZE {
            cur
        } else {
            PB_DUMMY_BASE
        };
        if BEGIN_PUSH_CURSOR
            .compare_exchange(cur, next, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return dest;
        }
    }
}

fn advance_begin_push_cursor_to(cursor: u32) {
    if cursor < PB_DUMMY_BASE || cursor > PB_DUMMY_BASE + PB_DUMMY_SIZE {
        return;
    }
    let mut cur = BEGIN_PUSH_CURSOR.load(Ordering::Relaxed);
    while cursor > cur {
        match BEGIN_PUSH_CURSOR.compare_exchange(cur, cursor, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => break,
            Err(actual) => cur = actual,
        }
    }
}

fn remember_begin_push_start(dest: u32) {
    BEGIN_PUSH_LAST_START.store(dest, Ordering::Relaxed);
}

fn begin_push_end_range(p_push: u32) -> Option<(u32, u32, &'static str)> {
    let start = BEGIN_PUSH_LAST_START.load(Ordering::Relaxed);
    if start != 0 {
        if p_push > start && p_push < 0x2000_0000 {
            return Some((start, p_push, "begin-return"));
        }

        // Some SDK paths pass a byte offset instead of an absolute cursor.
        // Keep that compatibility, but anchor it at the actual BeginPush
        // pointer we handed to the guest, not at a fixed assumed base.
        if p_push != 0 && p_push < PB_DUMMY_SIZE {
            let end = start.saturating_add(p_push);
            if end > start && end < 0x2000_0000 {
                return Some((start, end, "begin-return+offset"));
            }
        }
    }

    None
}

#[derive(Clone, Copy)]
struct InlinePushBounds {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    min_u: f32,
    min_v: f32,
    max_u: f32,
    max_v: f32,
}

fn read_begin_push_u32(guest_mem: *mut u8, addr: u32) -> u32 {
    unsafe { std::ptr::read_unaligned((guest_mem as u64 + addr as u64) as *const u32) }
}

fn inline_push_bounds(guest_mem: *mut u8, start: u32, end: u32) -> Option<InlinePushBounds> {
    if guest_mem.is_null() || end <= start || end > 0x2000_0000 {
        return None;
    }
    let byte_count = end.wrapping_sub(start);
    if byte_count < (3 + 24) * 4 {
        return None;
    }

    let begin_cmd = read_begin_push_u32(guest_mem, start);
    let inline_cmd = read_begin_push_u32(guest_mem, start + 8);
    let begin_count = (begin_cmd >> 18) & 0x7FF;
    let inline_count = (inline_cmd >> 18) & 0x7FF;
    let inline_non_inc = (inline_cmd >> 30) & 1 != 0;
    if (begin_cmd & 0x1FFC) != 0x17FC
        || begin_count == 0
        || (inline_cmd & 0x1FFC) != 0x1818
        || inline_count < 24
        || !inline_non_inc
    {
        return None;
    }

    let payload = start + 12;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_u = f32::INFINITY;
    let mut min_v = f32::INFINITY;
    let mut max_u = f32::NEG_INFINITY;
    let mut max_v = f32::NEG_INFINITY;
    for v in 0..4u32 {
        let base = payload + v * 24;
        let x = f32::from_bits(read_begin_push_u32(guest_mem, base));
        let y = f32::from_bits(read_begin_push_u32(guest_mem, base + 4));
        let u = f32::from_bits(read_begin_push_u32(guest_mem, base + 16));
        let tex_v = f32::from_bits(read_begin_push_u32(guest_mem, base + 20));
        if !x.is_finite() || !y.is_finite() || !u.is_finite() || !tex_v.is_finite() {
            return None;
        }
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        min_u = min_u.min(u);
        min_v = min_v.min(tex_v);
        max_u = max_u.max(u);
        max_v = max_v.max(tex_v);
    }

    if min_x > 672.0 || max_x < -32.0 || min_y > 544.0 || max_y < -32.0 {
        return None;
    }
    Some(InlinePushBounds {
        min_x,
        min_y,
        max_x,
        max_y,
        min_u,
        min_v,
        max_u,
        max_v,
    })
}

fn count_alpha_in_uv_rect(
    pixels: &[u32],
    tex_w: u32,
    tex_h: u32,
    min_u: f32,
    min_v: f32,
    max_u: f32,
    max_v: f32,
) -> (usize, usize, usize, u32, u64, u8, usize) {
    if tex_w == 0 || tex_h == 0 || pixels.is_empty() {
        return (0, 0, 0, 0, 0, 0, 0);
    }
    let x0 = ((min_u.min(max_u).clamp(0.0, 1.0) * tex_w as f32).floor() as u32).min(tex_w - 1);
    let x1 = ((min_u.max(max_u).clamp(0.0, 1.0) * tex_w as f32).ceil() as u32).min(tex_w);
    let y0 = ((min_v.min(max_v).clamp(0.0, 1.0) * tex_h as f32).floor() as u32).min(tex_h - 1);
    let y1 = ((min_v.max(max_v).clamp(0.0, 1.0) * tex_h as f32).ceil() as u32).min(tex_h);
    let mut total = 0usize;
    let mut alpha = 0usize;
    let mut rgb = 0usize;
    let mut first = 0u32;
    let mut alpha_sum = 0u64;
    let mut alpha_max = 0u8;
    let mut opaque = 0usize;
    for y in y0..y1.max(y0 + 1) {
        for x in x0..x1.max(x0 + 1) {
            let px = pixels
                .get(
                    (y as usize)
                        .saturating_mul(tex_w as usize)
                        .saturating_add(x as usize),
                )
                .copied()
                .unwrap_or(0);
            if total == 0 {
                first = px;
            }
            total += 1;
            let a = (px >> 24) as u8;
            alpha += usize::from(a != 0);
            alpha_sum += a as u64;
            alpha_max = alpha_max.max(a);
            opaque += usize::from(a == 0xFF);
            rgb += usize::from((px & 0x00FF_FFFF) != 0);
        }
    }
    (total, alpha, rgb, first, alpha_sum, alpha_max, opaque)
}

fn log_overlay_texture_sample(guest_mem: *mut u8, bounds: InlinePushBounds) {
    if guest_mem.is_null() {
        return;
    }
    static OVERLAY_TEX_SAMPLE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = OVERLAY_TEX_SAMPLE_LOG.fetch_add(1, Ordering::Relaxed);
    if n >= 64 && !n.is_power_of_two() {
        return;
    }

    let tex_key = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    let Some(info) = lookup_texture_info(tex_key) else {
        debug_log(&format!(
            "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} missing registry uv=[{:.3}..{:.3},{:.3}..{:.3}]",
            n, tex_key, bounds.min_u, bounds.max_u, bounds.min_v, bounds.max_v
        ));
        return;
    };
    let fmt_code = xbox_texture_format_code(info.format);
    let Some(data_addr) = normalize_guest_ram_ptr(info.data & !0x3) else {
        debug_log(&format!(
            "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} data=0x{:08X} invalid",
            n, tex_key, info.data
        ));
        return;
    };
    let byte_len =
        crate::xbox::gpu::texture_format::texture_byte_len(info.width, info.height, fmt_code);
    let byte_end = data_addr as usize + byte_len as usize;
    if byte_len == 0 || byte_len > 16 * 1024 * 1024 || byte_end > 0x2000_0000 {
        debug_log(&format!(
            "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} data=0x{:08X} bytes={} invalid",
            n, tex_key, data_addr, byte_len
        ));
        return;
    }
    let bytes =
        unsafe { std::slice::from_raw_parts(guest_mem.add(data_addr as usize), byte_len as usize) };
    let pixels = if crate::xbox::gpu::texture_format::is_block_compressed(fmt_code) {
        let Some(mut pixels) = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
            fmt_code,
            info.width,
            info.height,
            bytes,
        ) else {
            debug_log(&format!(
                "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} fmt=0x{:02X} decode-failed",
                n, tex_key, fmt_code
            ));
            return;
        };
        let _ = crate::xbox::gpu::texture_format::promote_alpha_mask_rgb(&mut pixels);
        pixels
    } else {
        let Some(bytes_per_pixel) = crate::xbox::gpu::texture_format::bytes_per_pixel(fmt_code)
        else {
            debug_log(&format!(
                "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} fmt=0x{:02X} unsupported-linear",
                n, tex_key, fmt_code
            ));
            return;
        };
        let linear_storage;
        let linear_bytes = if crate::xbox::gpu::texture_format::is_swizzled(fmt_code) {
            if let Some(unswizzled) = crate::xbox::gpu::swizzle::unswizzle(
                bytes,
                info.width,
                info.height,
                bytes_per_pixel,
            ) {
                linear_storage = unswizzled;
                linear_storage.as_slice()
            } else {
                bytes
            }
        } else {
            bytes
        };
        let Some(pixels) = crate::xbox::gpu::texture_format::decode_linear_to_argb(
            fmt_code,
            info.width,
            info.height,
            linear_bytes,
            None,
        ) else {
            debug_log(&format!(
                "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} fmt=0x{:02X} decode-failed",
                n, tex_key, fmt_code
            ));
            return;
        };
        pixels
    };
    if n == 0 {
        write_readback_bmp(
            r"./spiderman_overlay_active_dxt5.bmp",
            info.width,
            info.height,
            &pixels,
        );
    }
    let normal = count_alpha_in_uv_rect(
        &pixels,
        info.width,
        info.height,
        bounds.min_u,
        bounds.min_v,
        bounds.max_u,
        bounds.max_v,
    );
    let flipped = count_alpha_in_uv_rect(
        &pixels,
        info.width,
        info.height,
        bounds.min_u,
        1.0 - bounds.max_v,
        bounds.max_u,
        1.0 - bounds.min_v,
    );
    debug_log(&format!(
        "[OVERLAY-TEX-SAMPLE #{}] tex=0x{:08X} data=0x{:08X} fmt=0x{:02X}/{} {}x{} uv=[{:.3}..{:.3},{:.3}..{:.3}] normal alpha={}/{} rgb={}/{} sum={} max={} opaque={} first=0x{:08X} flipped alpha={}/{} rgb={}/{} sum={} max={} opaque={} first=0x{:08X}",
        n,
        tex_key,
        data_addr,
        fmt_code,
        crate::xbox::gpu::texture_format::format_name(fmt_code),
        info.width,
        info.height,
        bounds.min_u,
        bounds.max_u,
        bounds.min_v,
        bounds.max_v,
        normal.1,
        normal.0,
        normal.2,
        normal.0,
        normal.4,
        normal.5,
        normal.6,
        normal.3,
        flipped.1,
        flipped.0,
        flipped.2,
        flipped.0,
        flipped.4,
        flipped.5,
        flipped.6,
        flipped.3
    ));
}

fn maybe_arm_begin_push_overlay_blend(guest_mem: *mut u8, start: u32, end: u32) {
    let Some(bounds) = inline_push_bounds(guest_mem, start, end) else {
        return;
    };
    let width = bounds.max_x - bounds.min_x;
    let height = bounds.max_y - bounds.min_y;
    if width <= 0.0 || height <= 0.0 || width > 320.0 || height >= 240.0 {
        return;
    }

    log_overlay_texture_sample(guest_mem, bounds);

    static OVERLAY_BLEND_ARMED: AtomicU32 = AtomicU32::new(0);
    if OVERLAY_BLEND_ARMED.swap(1, Ordering::Relaxed) != 0 {
        return;
    }

    {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            backend.set_render_state(62, 5); // SrcBlend = SrcAlpha
            backend.set_render_state(63, 6); // DestBlend = InvSrcAlpha
            backend.set_render_state(74, 1); // BlendOp = Add
            backend.set_render_state(59, 1); // AlphaBlendEnable = TRUE
        }
    }

    static OVERLAY_BLEND_LOG: AtomicU32 = AtomicU32::new(0);
    let n = OVERLAY_BLEND_LOG.fetch_add(1, Ordering::Relaxed);
    crate::xbox::emulator::debug_log(&format!(
        "[PB-BLEND-STATE #{}] source=EndPushInline bbox=[{:.1},{:.1}..{:.1},{:.1}] -> slot=59 value=1 src=SrcAlpha dst=InvSrcAlpha",
        n, bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y
    ));
}

pub(super) fn hle_begin_state_big(count_dwords: u32, guest_mem: *mut u8) -> u32 {
    let dest = reserve_begin_push_space(count_dwords);
    let dev_ptr = read_dev_ptr(guest_mem);
    if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
        unsafe {
            let dev_base = guest_mem as u64 + dev_ptr as u64;
            *((dev_base + 0x00) as *mut u32) = dest;
            *((dev_base + 0x04) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE - 0x100;
            *((dev_base + DEV_PB_PUT as u64) as *mut u32) = dest;
        }
    }
    static BS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = BS_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-BEGINSTATEBIG #{}] count={} -> 0x{:08X}",
            n, count_dwords, dest
        ));
    }
    dest
}

pub(super) fn hle_reset_pushbuffer(guest_mem: *mut u8) -> u32 {
    reset_begin_push_cursor();
    let dev_ptr = read_dev_ptr(guest_mem);
    if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
        let dev_base = guest_mem as u64 + dev_ptr as u64;
        unsafe {
            *((dev_base + 0x00) as *mut u32) = PB_DUMMY_BASE;
            *((dev_base + 0x04) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE - 0x100;
            *((dev_base + DEV_PB_PUT as u64) as *mut u32) = PB_DUMMY_BASE;
            *((dev_base + DEV_PB_GET as u64) as *mut u32) = PB_DUMMY_BASE;
        }
    }
    PB_DUMMY_BASE
}

fn write_readback_bmp(path: &str, width: u32, height: u32, pixels: &[u32]) {
    let row_stride = ((width * 3 + 3) / 4) * 4;
    let image_size = row_stride * height;
    let file_size = 54 + image_size;
    let mut bmp = Vec::with_capacity(file_size as usize);

    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(file_size as u32).to_le_bytes());
    bmp.extend_from_slice(&[0u8; 4]);
    bmp.extend_from_slice(&54u32.to_le_bytes());
    bmp.extend_from_slice(&40u32.to_le_bytes());
    bmp.extend_from_slice(&(width as i32).to_le_bytes());
    bmp.extend_from_slice(&(height as i32).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());
    bmp.extend_from_slice(&24u16.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&(image_size as u32).to_le_bytes());
    bmp.extend_from_slice(&[0u8; 16]);

    for y in (0..height as usize).rev() {
        let row_start = y * width as usize;
        for x in 0..width as usize {
            let p = pixels.get(row_start + x).copied().unwrap_or(0);
            bmp.push((p & 0xFF) as u8);
            bmp.push(((p >> 8) & 0xFF) as u8);
            bmp.push(((p >> 16) & 0xFF) as u8);
        }
        while (bmp.len() - 54) % row_stride as usize != 0 {
            bmp.push(0);
        }
    }

    if let Err(e) = std::fs::write(path, bmp) {
        static BMP_ERR_LOG: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if !BMP_ERR_LOG.swap(true, Ordering::Relaxed) {
            crate::xbox::emulator::debug_log(&format!(
                "[ORGANIC-BMP] failed to write {}: {}",
                path, e
            ));
        }
    }
}

fn spidey_title_ui_trace_enabled() -> bool {
    std::env::var_os("RUSTEMU_SPIDEY_TITLE_UI_TRACE").is_some()
}

fn spidey_title_ui_candidate_texture(width: u32, height: u32) -> bool {
    matches!(
        (width, height),
        // MENU.xbs title/prompt overlay candidates:
        // 512x64 includes the top Spider-Man title logo; 128/256-wide sheets
        // include arrows, button glyphs, and compact frontend UI sprites.
        (512, 64)
            | (256, 64)
            | (128, 64)
            | (64, 64)
            | (64, 128)
            | (128, 128)
            | (256, 128)
            | (128, 256)
            | (256, 256)
    )
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

// ============================================================================
// HLE stubs
// ============================================================================

pub(super) fn hle_complete_d3d_wait(name: &str, guest_mem: *mut u8) -> u32 {
    let dev_ptr = read_dev_ptr(guest_mem);
    if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
        let dev_base = guest_mem as u64 + dev_ptr as u64;
        unsafe {
            let native_put = *((dev_base + 0x30) as *const u32);
            let hle_put = *((dev_base + DEV_PB_PUT as u64) as *const u32);
            let shadow_put = crate::xbox::aot::nv2a::shadow_read(0x80_0040);
            let completion = if hle_put != 0 {
                hle_put
            } else if native_put != 0 {
                native_put
            } else if shadow_put != 0 {
                shadow_put
            } else {
                PB_DUMMY_BASE
            };

            // Native XDK wait helpers treat [dev+0x34] as a pointer to the
            // GPU GET/completion cell, not as the completion value itself.
            // Use our internal device cell when the native pointer is missing
            // or points outside guest RAM.
            let get_cell = dev_ptr.wrapping_add(DEV_PB_GET);
            let native_get_ptr = *((dev_base + 0x34) as *const u32);
            let effective_get_ptr = if guest_ram_offset(native_get_ptr).is_some() {
                native_get_ptr
            } else {
                *((dev_base + 0x34) as *mut u32) = get_cell;
                get_cell
            };

            *((dev_base + DEV_PB_PUT as u64) as *mut u32) = completion;
            *((dev_base + DEV_PB_GET as u64) as *mut u32) = completion;
            *((dev_base + 0x30) as *mut u32) = completion;
            if let Some(off) = guest_ram_offset(effective_get_ptr) {
                *((guest_mem as u64 + off as u64) as *mut u32) = completion;
            }

            crate::xbox::aot::nv2a::shadow_write(0x80_0040, completion);
            crate::xbox::aot::nv2a::shadow_write(0x80_0044, completion);
            crate::xbox::aot::nv2a::shadow_write(0x3244, completion);

            static WAIT_LOG: AtomicU32 = AtomicU32::new(0);
            let n = WAIT_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 12 || n.is_power_of_two() {
                debug_log(&format!(
                    "[D3D-WAIT-HLE] #{} {} dev=0x{:08X} put=0x{:08X} get_ptr=0x{:08X}->0x{:08X}",
                    n, name, dev_ptr, completion, native_get_ptr, effective_get_ptr
                ));
            }
        }
    }
    0
}

pub(super) fn execute_hle(
    name: &str,
    args: &[u32; 8],
    _argc: u8,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_mem: *mut u8,
    mmio_count: &mut u64,
    pb_commands: &mut u64,
    draw_calls: &mut u64,
    is_manual: bool,
    guest_addr: u32,
) -> u32 {
    // Common D3D lifecycle hooks use the full HLE path even from manual hooks.
    // Only divert to execute_manual_hle for Spider-Man/game-specific manual stubs.
    // Manual hooks that have game-specific handlers in execute_manual_hle
    // route there. All other functions (including SymbolCache entries) go
    // through the main execute_hle match which has the SDK-level handlers.
    //
    // 2026-04-20: Route by NAME, not by is_manual flag. xbsymdb pattern matches
    // arrive with is_manual=false even for functions whose handlers live in
    // execute_manual_hle (e.g. D3DDevice_EndPush matches via xbsymdb first).
    // Previously those calls silently hit the `_ => 0` catch-all in execute_hle.
    let manual_names: &[&str] = &[
        "D3D_RenderStateSort",
        "D3D_RenderStateSort2",
        "D3D_RenderStateInit",
        "D3D_MakeSpace",
        "CDevice_SetStateVB",
        "D3DDevice_SetTexture",
        "D3DDevice_SetPalette",
        "D3DDevice_CreatePalette",
        "D3DDevice_CreatePalette2",
        "D3DPalette_Lock",
        "D3DPalette_Lock2",
        "D3DDevice_SetStreamSource",
        "D3DDevice_SetPixelShaderConstant",
        "D3DDevice_SetVertexShaderConstant",
        "D3DResource_Register",
        "D3DDevice_SetRenderState_FrontFace",
        "D3DDevice_EndPush",
        "D3DDevice_SetViewport",
        "D3DDevice_BlockUntilVerticalBlank",
        "D3DDevice_SetVerticalBlankCallback",
        "D3DDevice_SetSwapCallback",
        "D3DDevice_SetGammaRamp",
        "SceneRender",
        "SceneUpdate",
        "Doom_DirectSoundCreate",
        "Doom_DSoundRelease",
        "RtlAllocateHeap",
        "RtlFreeHeap",
        "XGRPH_CreateTexture",
        "XGRPH_AssembleShader",
        "D3DDevice_CreateTexture", // 2026-04-25: route to execute_manual_hle so the
        "D3DDevice_CreateTexture2",
        // inline-extracted hle_create_texture() actually
        // fires. Without this, execute_hle falls through
        // to its main match (no arm) → `_ => 0`, returning
        // D3D_OK without minting a fake handle. Per agent
        // E1/E6/E8/E10 + the post-edit run showing zero
        // [HLE] CreateTexture: log lines, this routing
        // gap was the actual cause of all 49 NULL writes.
        "BinkOpen",
        "BinkPause",
        "BinkWait",
        "BinkDoFrame",
        "BinkCopyToBuffer",
        "BinkNextFrame",
        "BinkClose",
        "Doom_malloc",
        "Doom_malloc2",
        "Doom_D3D_Clear",
        "Doom_D3D_DrawVerticesUP",
        "Doom_D3D_SetTexture",
        "Doom_D3D_Begin",
        "Doom_D3D_End",
        "Doom_D3D_Swap",
        "Doom_D3D_GetBackBuffer2",
        "Doom_D3D_IsBusy",
        "Doom_D3D_PersistDisplay",
        "Doom_D3D_BlockVBlank",
        "Doom_D3D_SetRenderTarget",
        "Doom_D3D_CopyRects",
        "Doom_D3D_KickOff",
        "Doom_D3D_KickOffAndWait",
        "Doom_D3D_MakeSpace",
        "Doom_D3D_SetViewport",
        "Doom_D3D_LoadVertexShader",
        "Shenmue_CRT_memmove",
        "Shenmue_MatrixLoadCurrent",
        "Shenmue_MatrixStoreCurrent",
        "Shenmue_MatrixMultiplyCurrent",
        "GameAssert",
    ];
    if manual_names.contains(&name) {
        static MANUAL_ROUTE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = MANUAL_ROUTE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 20 {
            debug_log(&format!(
                "[HLE-ROUTE #{}] {} → execute_manual_hle (is_manual={})",
                n, name, is_manual
            ));
        }
        let _ = is_manual; // silence unused-warning gate
        return execute_manual_hle(name, args, context, guest_mem);
    }

    // Specialized SetRenderState_* handlers (2026-04-21). Each of these is
    // stdcall(Value) where the slot is implicit in the function name. The
    // xbsymdb pattern table detects dozens of variants; rather than 50
    // individual match arms, intercept by name prefix and route through
    // render_state::apply_global. Returns 0 (D3D_OK) like every specialized
    // setter does.
    //
    // FrontFace is in manual_names above and handled via execute_manual_hle,
    // so this guard won't reach it.
    if let Some(suffix) = name.strip_prefix("D3DDevice_SetRenderState_") {
        if let Some(slot) = specialized_rs_slot_from_suffix(suffix) {
            let value = args[0]; // stdcall single arg
            drain_draws_before_state_change(name);
            if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
                gpu.set_render_state(slot, value);
            }
            let group = crate::xbox::gpu::render_state::apply_global(slot, value);
            hle_note_pixel_shader_render_state(suffix, slot, value);
            static SPEC_RS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = SPEC_RS_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 40 {
                debug_log(&format!(
                    "[HLE] SetRenderState_{} val=0x{:08X} slot={} group={:?} (#{n})",
                    suffix, value, slot, group
                ));
            }
            return 0;
        }
    }

    if name.starts_with("D3DDevice_DrawIndexedVertices")
        && !name.starts_with("D3DDevice_DrawIndexedVerticesUP")
    {
        static DRAWIDX_ROUTE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = DRAWIDX_ROUTE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 20 || (n % 1000 == 0) {
            debug_log(&format!(
                "[HLE-DRAWIDX-ROUTE] #{} {} -> indexed handler",
                n, name
            ));
        }
        hle_draw_indexed_vertices(args, guest_mem, mmio_count);
        *draw_calls += 1;
        return 0;
    }

    match name {
        "D3DDevice_BeginVisibilityTest"
        | "D3DDevice_EndVisibilityTest"
        | "D3DDevice_GetVisibilityTestResult" => {
            return hle_visibility_test(name, args, guest_mem);
        }
        "D3DDevice_CopyRects" => {
            return hle_copy_rects(args, guest_mem);
        }
        "XGSwizzleRect" => {
            return hle_xg_swizzle_rect(args, guest_mem);
        }
        // Frame-body helper tripwires. Log on entry, skip the body (return 0).
        // Whichever is silent in the log after master init runs identifies the
        // hang candidate. Guest PCs from the Frame-body disasm: post-LOD-alloc
        // sequence at 0x000D68EE / 0x000D68F3 / 0x000D68F8 / 0x000D68FD (call
        // sites), targets 0xD60E0 / 0xD63B0 / 0xD6440 / 0xD64D0 respectively.
        "FRAME_HELPER_A" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            if n < 4 || n.is_power_of_two() {
                debug_log(&format!(
                    "[FRAME-HELPER A #{}] sub_0x000D60E0 reached esp=0x{:08X}",
                    n, context.R14 as u32
                ));
            }
            return 0;
        }
        "FRAME_HELPER_B" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            if n < 4 || n.is_power_of_two() {
                debug_log(&format!(
                    "[FRAME-HELPER B #{}] sub_0x000D63B0 reached esp=0x{:08X}",
                    n, context.R14 as u32
                ));
            }
            return 0;
        }
        "FRAME_HELPER_C" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            if n < 4 || n.is_power_of_two() {
                debug_log(&format!(
                    "[FRAME-HELPER C #{}] sub_0x000D6440 reached esp=0x{:08X}",
                    n, context.R14 as u32
                ));
            }
            return 0;
        }
        "FRAME_HELPER_D" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            if n < 4 || n.is_power_of_two() {
                debug_log(&format!(
                    "[FRAME-HELPER D #{}] sub_0x000D64D0 reached esp=0x{:08X}",
                    n, context.R14 as u32
                ));
            }
            return 0;
        }
        // Engine.Init2 chain tripwires — if Frame body reaches these, they fire.
        // If silent, the hang is BEFORE that probe.
        "ENGINE_INIT2_PROBE" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[ENGINE-INIT2 #{}] sub_0x000F23C0 reached this=0x{:08X} esp=0x{:08X}",
                n, context.Rcx as u32, context.R14 as u32
            ));
            return 0;
        }
        "F_EFDC0_PROBE" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[F-EFDC0 #{}] sub_0x000EFDC0 reached esp=0x{:08X}",
                n, context.R14 as u32
            ));
            return 0;
        }
        "F_1C8C0_PROBE" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[F-1C8C0 #{}] sub_0x0001C8C0 reached this=0x{:08X} esp=0x{:08X}",
                n, context.Rcx as u32, context.R14 as u32
            ));
            return 0;
        }
        "LOD_4SETTER_PROBE" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[LOD-4SETTER #{}] sub_0x000DD470 reached esp=0x{:08X}",
                n, context.R14 as u32
            ));
            return 0;
        }
        "FRAME_TAIL_28F980" => {
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[FRAME-TAIL #{}] sub_0x0028F980 reached esp=0x{:08X}",
                n, context.R14 as u32
            ));
            return 0;
        }
        "LOD_SETTER_2A3C20" => {
            // Called 4× from sub_0xDD470 with (idx, float). args[0]=idx, args[1]=float.
            // If this fires N times, we know Frame body progressed through N of the 4
            // setter calls before hanging on the (N+1)th.
            static LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG.fetch_add(1, Ordering::Relaxed);
            debug_log(&format!(
                "[LOD-SETTER #{}] sub_0x002A3C20 idx={} float=0x{:08X} esp=0x{:08X}",
                n, args[0], args[1], context.R14 as u32
            ));
            return 0;
        }
        "Frame_CTOR_PROBE" => {
            // Diagnostic: does master init ever reach `call 0x2a4a50`? If we
            // see this log, Frame ctor IS called — body throws. If we never
            // see it, master init bails out before reaching 0x2A53FA.
            static FRAME_CTOR_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = FRAME_CTOR_LOG.fetch_add(1, Ordering::Relaxed);
            let r14 = context.R14 as u32;
            debug_log(&format!(
                "[FRAME-PROBE #{}] sub_002A4A50 reached — master init DID call Frame ctor (guest_esp=0x{:08X})",
                n, r14
            ));
            // Return 0 (fake success). Skips the actual Frame ctor body.
            // Frame ptr at 0x4BC630 will still be NULL afterward — that's
            // fine, we're just probing reachability.
            return 0;
        }
        "OutputDebugStringA" => {
            // Read the string pointer from args[0] and log it
            let str_ptr = args[0];
            if str_ptr > 0 && str_ptr < 0x1000_0000 {
                let base = guest_mem as u64 + str_ptr as u64;
                let mut s = String::new();
                for i in 0..256u32 {
                    let b = unsafe { *((base + i as u64) as *const u8) };
                    if b == 0 {
                        break;
                    }
                    s.push(b as char);
                }
                debug_log(&format!("[GUEST-DBG] {}", s.trim()));
            }
            return 0;
        }
        "OutputDebugStringW" => {
            // Wide string - read first 128 chars
            let str_ptr = args[0];
            if str_ptr > 0 && str_ptr < 0x1000_0000 {
                let base = guest_mem as u64 + str_ptr as u64;
                let mut s = String::new();
                for i in 0..128u32 {
                    let w = unsafe { *((base + i as u64 * 2) as *const u16) };
                    if w == 0 {
                        break;
                    }
                    s.push(char::from_u32(w as u32).unwrap_or('?'));
                }
                debug_log(&format!("[GUEST-DBG-W] {}", s.trim()));
            }
            return 0;
        }
        "Direct3D_CreateDevice" => {
            // --- CD-PROBE entry (2026-04-21, test_blue_ltcg diagnostic) ---
            //
            // test_blue_ltcg.cod shows CreateDevice is followed IMMEDIATELY
            // by the main render loop (Clear/Swap/jmp). If our CreateDevice
            // HLE fires 11 times but Clear fires 0 times, something about
            // the return makes the guest loop back to CreateDevice instead
            // of falling into the render loop.
            //
            // Capture: entry R14 (guest ESP), [R14] (pushed return addr),
            // shadow-stack top, and each arg. First 12 fires verbatim.
            let r14_in = context.R14 as u32;
            let ret_addr = if (r14_in as u64) < 0x2000_0000 {
                unsafe {
                    std::ptr::read_unaligned((guest_mem as u64 + r14_in as u64) as *const u32)
                }
            } else {
                0
            };
            let r12_top = context.R12 as u64;
            let shadow_top_val = if r12_top != 0 {
                unsafe { std::ptr::read_unaligned(r12_top as *const u64) }
            } else {
                0
            };
            static CD_PROBE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let cd_n = CD_PROBE.fetch_add(1, Ordering::Relaxed);
            if cd_n < 12 {
                debug_log(&format!(
                    "[CD-PROBE #{}] ENTRY r14=0x{:08X} ret_addr=[r14]=0x{:08X} \
                     args=[{:08X},{:08X},{:08X},{:08X},{:08X},{:08X}] \
                     r12=0x{:016X} [r12]=0x{:016X}",
                    cd_n,
                    r14_in,
                    ret_addr,
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    args[4],
                    args[5],
                    r12_top,
                    shadow_top_val
                ));
            }

            // Diagnostic: check ticdup at CreateDevice time
            let gs_ptr = unsafe { *((guest_mem as u64 + 0x101BB8) as *const u32) };
            if gs_ptr > 0 && gs_ptr < 0x1000_0000 {
                let base = guest_mem as u64 + gs_ptr as u64;
                let word_1272 = unsafe { *((base + 0x1272) as *const u16) };
                let ticdup = unsafe { *((base + 0x1F50) as *const u32) };
                debug_log(&format!(
                    "[TICDUP-DIAG] CreateDevice: gs=0x{:08X} word@1272={} ticdup={}",
                    gs_ptr, word_1272, ticdup
                ));
            }
            let r = hle_create_device(guest_mem, mmio_count, guest_addr);
            // Write device pointer to ppReturnedDeviceInterface (args[5])
            // Xbox Direct3D_CreateDevice: (Adapter, DeviceType, hFocusWindow, BehaviorFlags, pPresentParams, ppDevice)
            let pp_device = args[5];
            if let Some(pp_device_norm) = guest_ram_offset(pp_device) {
                let dev_addr = 0x0030_0E00u32;
                unsafe {
                    *((guest_mem as u64 + pp_device_norm as u64) as *mut u32) = dev_addr;
                }
                debug_log(&format!(
                    "[HLE] CreateDevice: wrote dev=0x{:08X} to ppDevice=0x{:08X}->0x{:08X}",
                    dev_addr, pp_device, pp_device_norm as u32
                ));
            }
            *pb_commands += 1;
            crate::xbox::aot::veh_dpc::signal_crt_boot_complete();

            // --- CD-PROBE exit ---
            //
            // HLE dispatch returns via the emitter's standard stdcall
            // cleanup (argc*4 bytes popped). For stdcall(6), that's +24
            // bytes of R14 cleanup after this handler returns. Log R14
            // RIGHT NOW (still pre-cleanup from HLE's view).
            if cd_n < 12 {
                let r14_exit = context.R14 as u32;
                debug_log(&format!(
                    "[CD-PROBE #{}] EXIT  r14=0x{:08X} (delta from entry: {}) ret_val=0x{:08X}",
                    cd_n,
                    r14_exit,
                    (r14_exit as i64) - (r14_in as i64),
                    r
                ));
            }

            r
        }
        "D3DDevice_Swap" => {
            // Ensure CRT boot complete is signaled (idempotent safety net).
            crate::xbox::aot::veh_dpc::signal_crt_boot_complete();
            // FLAG WATCH: check dev+8 before and after hle_swap
            let gpd = G_PDEVICE_DYNAMIC.load(Ordering::Relaxed);
            let dp = if gpd != 0 {
                unsafe { *((guest_mem as u64 + gpd as u64) as *const u32) }
            } else {
                0
            };
            let flags_before = if dp != 0 {
                unsafe { *((guest_mem as u64 + dp as u64 + 8) as *const u32) }
            } else {
                0
            };

            let swap_flags = args[0];
            let r = hle_swap_with_flags(guest_mem, mmio_count, swap_flags);

            let flags_after = if dp != 0 {
                unsafe { *((guest_mem as u64 + dp as u64 + 8) as *const u32) }
            } else {
                0
            };
            if flags_before != flags_after {
                debug_log(&format!(
                    "[FLAG-WATCH] Swap: dev+8 changed INSIDE hle_swap: 0x{:08X} → 0x{:08X}",
                    flags_before, flags_after
                ));
            }
            static SWAP_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let sc = SWAP_COUNT.fetch_add(1, Ordering::Relaxed);
            if sc < 3 {
                debug_log(&format!(
                    "[FLAG-WATCH] Swap#{}: dev=0x{:08X} flags_before=0x{:08X} flags_after=0x{:08X}",
                    sc, dp, flags_before, flags_after
                ));
            }
            *pb_commands += 1;
            *draw_calls += 1; // Swap = frame presented → Stage 10
            OOVPA_PB_COMMANDS.store(*pb_commands, Ordering::Relaxed);
            OOVPA_SWAP_COUNT.fetch_add(1, Ordering::Relaxed);
            SWAP_COUNT_GLOBAL.fetch_add(1, Ordering::Relaxed);
            r
        }
        "D3DDevice_Clear" => {
            // FLAG WATCH: check dev+8 before and after hle_clear
            let gpd = G_PDEVICE_DYNAMIC.load(Ordering::Relaxed);
            let dp = if gpd != 0 {
                unsafe { *((guest_mem as u64 + gpd as u64) as *const u32) }
            } else {
                0
            };
            let flags_before = if dp != 0 {
                unsafe { *((guest_mem as u64 + dp as u64 + 8) as *const u32) }
            } else {
                0xDEAD
            };

            let r = hle_clear(args, guest_mem, mmio_count);

            let flags_after = if dp != 0 {
                unsafe { *((guest_mem as u64 + dp as u64 + 8) as *const u32) }
            } else {
                0xDEAD
            };
            if flags_before != flags_after {
                debug_log(&format!(
                    "[FLAG-WATCH] Clear: dev+8 INSIDE hle_clear: 0x{:08X} → 0x{:08X} dev=0x{:08X}",
                    flags_before, flags_after, dp
                ));
            } else {
                static CLEAR_OK: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let c = CLEAR_OK.fetch_add(1, Ordering::Relaxed);
                if c < 3 {
                    debug_log(&format!("[FLAG-WATCH] Clear#{}: dev+8 unchanged at 0x{:08X} (dev=0x{:08X}, gpd=0x{:08X})", c, flags_before, dp, gpd));
                }
            }
            r
        }
        "D3DDevice_SetVertexShader" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "execute_hle",
                guest_addr,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            );
            hle_set_vertex_shader(args, guest_mem)
        }
        "D3DDevice_SelectVertexShader" | "D3DDevice_SelectVertexShaderDirect" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "execute_hle",
                guest_addr,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            )
        }
        "D3DDevice_SetVertexShaderInput" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "execute_hle",
                guest_addr,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            );
            hle_set_vertex_shader_input(args, guest_mem)
        }
        "D3DDevice_CreateVertexShader" => hle_create_vertex_shader(args, guest_mem),
        "D3DDevice_CreatePixelShader" => hle_create_pixel_shader(args, guest_mem),
        "D3DDevice_SetPixelShader" | "D3DDevice_SetPixelShaderProgram" => {
            hle_set_pixel_shader(args, guest_mem)
        }
        "D3DDevice_SetTransform" => hle_set_transform(args, guest_mem),
        "D3DDevice_SetShaderConstantMode" => {
            let stack_mode = stack_arg0(context);
            let eax_mode = context.Rax as u32;
            let mode = if _argc > 0 && plausible_shader_constant_mode(args[0]) {
                args[0]
            } else if let Some(mode) = stack_mode.filter(|m| plausible_shader_constant_mode(*m)) {
                mode
            } else {
                eax_mode
            };
            static SCM_LOG: AtomicU32 = AtomicU32::new(0);
            let n = SCM_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE] SetShaderConstantMode: argc={} arg0=0x{:08X} stack_arg0={} eax=0x{:08X} chosen=0x{:08X}",
                    _argc,
                    args[0],
                    stack_mode
                        .map(|m| format!("0x{:08X}", m))
                        .unwrap_or_else(|| "-".to_string()),
                    eax_mode,
                    mode,
                ));
            }
            hle_note_shader_constant_mode("hle", mode);
            0
        }
        "D3DDevice_GetShaderConstantMode" => {
            let p_mode = if _argc > 0 {
                args[0]
            } else {
                stack_arg0(context).unwrap_or(args[0])
            };
            let mode = shader_constant_mode();
            if p_mode != 0 {
                if let Some(ptr) = normalize_guest_ram_ptr(p_mode) {
                    if ptr <= 0x2000_0000u32.saturating_sub(4) {
                        unsafe {
                            *((guest_mem.add(ptr as usize)) as *mut u32) = mode;
                        }
                    }
                }
            }
            static GSCM_LOG: AtomicU32 = AtomicU32::new(0);
            let n = GSCM_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE] GetShaderConstantMode: pMode=0x{:08X} mode=0x{:08X}",
                    p_mode, mode
                ));
            }
            0
        }
        "D3DDevice_DrawVerticesUP" => {
            hle_draw_vertices_up(args, guest_mem);
            *draw_calls += 1;
            0
        }
        "D3DDevice_DrawIndexedVerticesUP" => {
            hle_draw_indexed_vertices_up(args, guest_mem);
            *draw_calls += 1;
            0
        }
        "D3DDevice_BeginStateBig" => hle_begin_state_big(args[0], guest_mem),
        "D3DDevice_BeginPush" => hle_begin_push(args, guest_mem, context, _argc >= 2),
        "D3DDevice_BeginPush_8" => hle_begin_push(args, guest_mem, context, true),
        "D3DDevice_SetScreenSpaceOffset" => {
            let x = f32::from_bits(args[0]);
            let y = f32::from_bits(args[1]);
            // Cxbx-R stores the Xbox screen-space offset with a pixel-center
            // adjustment. Keep the same generic SDK behavior when the TOML
            // fixture enables this hook.
            let adjusted_x = x + 0.53125;
            let adjusted_y = y + 0.53125;
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                let dev_base = guest_mem as u64 + dev_ptr as u64;
                unsafe {
                    *((dev_base + 0x09E8) as *mut f32) = adjusted_x;
                    *((dev_base + 0x09EC) as *mut f32) = adjusted_y;
                }
            }
            static SSO_LOG: AtomicU32 = AtomicU32::new(0);
            let n = SSO_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE] SetScreenSpaceOffset #{} x={} y={} adjusted=({}, {}) dev=0x{:08X}",
                    n, x, y, adjusted_x, adjusted_y, dev_ptr
                ));
            }
            0
        }
        "D3DDevice_SetRenderState_Simple" => {
            // fastcall: ecx=NV2A register, edx=value.
            // On real Xbox D3D8, this writes to the push buffer AND to D3D_g_RenderState shadow array.
            // The game's BST init reads back from the shadow array — if we don't populate it, crash.
            let ecx = context.Rcx as u32;
            let edx = context.Rdx as u32;
            let (slot, value) =
                crate::xbox::gpu::render_state::normalize_render_state_input(ecx, edx);
            drain_draws_before_state_change(name);
            if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
                gpu.set_render_state(slot, value);
            }
            // Write to D3D_g_RenderState shadow array at 0x003009B0 (from SymbolCache).
            // NV2A register 0x000403xx → index = (register - 0x00040300) / 4.
            // Also write at the raw NV2A register offset for code that reads by register address.
            const G_RENDER_STATE: u32 = 0x0030_09B0;
            if ecx >= 0x0004_0300 && ecx < 0x0004_0400 {
                let idx = (ecx - 0x0004_0300) / 4;
                let ptr = G_RENDER_STATE + idx * 4;
                if ptr < 0x1000_0000 {
                    unsafe {
                        *((guest_mem as u64 + ptr as u64) as *mut u32) = edx;
                    }
                }
            }
            // Also write to D3D_g_DeferredRenderState at 0x00300AF8 for deferred states
            const G_DEFERRED_RS: u32 = 0x0030_0AF8;
            if ecx >= 0x0004_0300 && ecx < 0x0004_0500 {
                let idx = (ecx - 0x0004_0300) / 4;
                let ptr = G_DEFERRED_RS + idx * 4;
                if ptr < 0x1000_0000 {
                    unsafe {
                        *((guest_mem as u64 + ptr as u64) as *mut u32) = edx;
                    }
                }
            }
            let group = crate::xbox::gpu::render_state::apply_global(slot, value);
            static RS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = RS_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 10 {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE] SetRenderState_Simple: reg=0x{:08X} val=0x{:08X} slot={} norm_val=0x{:08X} group={:?} (#{n})",
                    ecx,
                    edx,
                    slot,
                    value,
                    group
                ));
            }
            hle_note_pixel_shader_render_state("SetRenderState_Simple", slot, value);
            0
        }
        "D3DDevice_SetRenderStateNotInline" => {
            // stdcall(State, Value) ret 8.
            // Existing path: forward to GPU backend set_render_state (kept
            // for backwards compat).
            let (state, value) =
                crate::xbox::gpu::render_state::normalize_render_state_input(args[0], args[1]);
            drain_draws_before_state_change(name);
            if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
                gpu.set_render_state(state, value);
            }
            // New path (2026-04-21): also track in the typed RenderStateTable
            // so the eventual backend.draw_primitive can read current state.
            // Purely additive — doesn't change the existing gpu_lock behavior.
            let group = crate::xbox::gpu::render_state::apply_global(state, value);
            hle_note_pixel_shader_render_state("SetRenderStateNotInline", state, value);
            static NI_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = NI_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 20 {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE] SetRenderStateNotInline: state={} val=0x{:08X} norm_state={} norm_val=0x{:08X} group={:?} (#{n})",
                    args[0], args[1], state, value, group
                ));
            }
            0
        }
        "D3DDevice_SetTextureStageState"
        | "D3DDevice_SetTextureStageStateNotInline"
        | "D3DDevice_SetTextureStageStateNotInline2"
        | "D3D_CDevice_SetTextureStageStateNotInline" => {
            hle_note_texture_stage_state(name, args[0], args[1], args[2])
        }
        "D3DDevice_SetTextureState_TexCoordIndex" => {
            hle_note_texture_stage_state(name, args[0], 28, args[1])
        }
        "D3DDevice_SetTextureState_BorderColor" => {
            hle_note_texture_stage_state(name, args[0], 29, args[1])
        }
        "D3DDevice_SetTextureState_ColorKeyColor" => {
            hle_note_texture_stage_state(name, args[0], 30, args[1])
        }
        "D3DDevice_SetTextureState_BumpEnv" => {
            static BUMPENV_LOG: AtomicU32 = AtomicU32::new(0);
            let n = BUMPENV_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 24 || n.is_power_of_two() {
                debug_log(&format!(
                    "[TSS] src={} specialized BumpEnv hit args=[0x{:08X},0x{:08X},0x{:08X}] #{}",
                    name, args[0], args[1], args[2], n
                ));
            }
            0
        }
        "D3DDevice_SetLight" => hle_note_set_light(args[0], args[1], guest_mem),
        "D3DDevice_LightEnable" => hle_note_light_enable(args[0], args[1]),
        "D3DDevice_SetMaterial" => hle_note_set_material(args[0], guest_mem),
        "D3DDevice_Reset" => 0, // D3D_OK
        "D3DDevice_SetRenderTarget" | "Doom_D3D_SetRenderTarget" => {
            hle_set_render_target(args, guest_mem)
        }
        // Doom D3D hooks → real HLE handlers
        "Doom_D3D_Clear" => hle_clear(args, guest_mem, mmio_count),
        "Doom_D3D_DrawVerticesUP" => {
            hle_draw_vertices_up(args, guest_mem);
            *draw_calls += 1;
            0
        }
        "Doom_D3D_SetTexture" => {
            hle_set_texture(args, guest_mem);
            0
        }
        "Doom_D3D_Begin" => {
            // Begin(Flags) — stores primitive type for subsequent inline vertex submission.
            // On Xbox D3D8, Begin brackets inline vertex data before End flushes.
            // The actual drawing happens via DrawVerticesUP, so this is informational.
            static BEGIN_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = BEGIN_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 5 {
                debug_log(&format!("[HLE] D3DDevice_Begin: flags=0x{:X}", args[0]));
            }
            0
        }
        "Doom_D3D_End" => {
            // End() — flush inline vertices. DrawVerticesUP handles actual submission.
            0
        }
        "Doom_D3D_GetBackBuffer2" => {
            // GetBackBuffer2(BackBuffer) → returns surface pointer.
            // Return address of our render target area so game can lock/write it.
            0x00F0_0000u32 // render target base from CreateDevice
        }
        "Doom_D3D_IsBusy" => 0,         // GPU not busy — immediate completion
        "Doom_D3D_PersistDisplay" => 0, // no-op in HLE
        "Doom_D3D_BlockVBlank" => {
            // BlockUntilVerticalBlank — yield to simulate VBlank wait
            std::thread::yield_now();
            0
        }
        "Doom_D3D_CopyRects" => {
            // CopyRects(pSourceSurface, pSourceRectsArray, cRects, pDestSurface, pDestPointsArray)
            // In Doom's render path, this copies the software-rendered screen to the back buffer.
            // The source is screens[0] (8-bit paletted), dest is the D3D back buffer.
            // Read Doom's framebuffer and blit it.
            crate::xbox::worker::doom_finish_update_from_guest_mem(guest_mem);
            0
        }
        // Doom_D3D_Render removed — runs natively to issue DrawVerticesUP calls
        "Doom_D3D_Swap" => {
            crate::xbox::aot::veh_dpc::signal_crt_boot_complete();
            if crate::xbox::worker::doom_finish_update_and_present_from_guest_mem(guest_mem) {
                *pb_commands += 1;
                *draw_calls += 1;
                OOVPA_PB_COMMANDS.store(*pb_commands, Ordering::Relaxed);
                OOVPA_SWAP_COUNT.fetch_add(1, Ordering::Relaxed);
                return 0;
            }
            // Diagnostic: check ticdup state on every Swap
            static SWAP_DIAG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let sd = SWAP_DIAG.fetch_add(1, Ordering::Relaxed);
            if sd < 5 {
                let gs_ptr = unsafe { *((guest_mem as u64 + 0x101BB8) as *const u32) };
                if gs_ptr > 0 && gs_ptr < 0x1000_0000 {
                    let base = guest_mem as u64 + gs_ptr as u64;
                    let word_1272 = unsafe { *((base + 0x1272) as *const u16) };
                    let ticdup = unsafe { *((base + 0x1F50) as *const u32) };
                    let gametic = unsafe { *((base + 0x271C) as *const u32) };
                    let gamestate = unsafe { *((base + 0x16C) as *const i32) };
                    debug_log(&format!(
                        "[DOOM-SWAP-DIAG] #{} gs=0x{:08X} word@1272={} ticdup@1F50={} gametic={} gamestate={}",
                        sd, gs_ptr, word_1272, ticdup, gametic, gamestate
                    ));
                }
            }
            let r = hle_swap(guest_mem, mmio_count);
            *pb_commands += 1;
            *draw_calls += 1;
            OOVPA_PB_COMMANDS.store(*pb_commands, Ordering::Relaxed);
            OOVPA_SWAP_COUNT.fetch_add(1, Ordering::Relaxed);
            r
        }
        "Doom_D3D_KickOff" | "Doom_D3D_KickOffAndWait" => 0, // no pushbuffer GPU in HLE
        "Doom_D3D_SetViewport" | "D3DDevice_SetViewport" => {
            // D3DVIEWPORT8: { X, Y, Width, Height, MinZ, MaxZ } = 24 bytes
            let vp_ptr = args[0];
            if vp_ptr != 0 && vp_ptr < 0x2000_0000 {
                let dev_ptr = read_dev_ptr(guest_mem);
                if dev_ptr != 0 && dev_ptr < 0x2000_0000 {
                    unsafe {
                        let src = guest_mem as u64 + vp_ptr as u64;
                        let dst = guest_mem as u64 + dev_ptr as u64 + 0x09D0; // viewport at dev+0x09D0 (from disasm)
                        std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 24);
                    }
                }
                // Read viewport fields and forward to GPU backend
                unsafe {
                    let base = guest_mem.add(vp_ptr as usize);
                    let x = *(base as *const u32);
                    let y = *(base.add(4) as *const u32);
                    let w = *(base.add(8) as *const u32);
                    let h = *(base.add(12) as *const u32);
                    let min_z = *(base.add(16) as *const f32);
                    let max_z = *(base.add(20) as *const f32);
                    cache_shader_viewport(x, y, w, h, min_z, max_z);
                    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
                        gpu.set_viewport(x, y, w, h, min_z, max_z);
                    }
                    static VP_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = VP_LOG.fetch_add(1, Ordering::Relaxed);
                    if n < 5 {
                        debug_log(&format!(
                            "[HLE] SetViewport: {}x{}+{}+{} z=[{:.2},{:.2}]",
                            w, h, x, y, min_z, max_z
                        ));
                    }
                }
            }
            0
        }
        "Doom_D3D_LoadVertexShader" => {
            // LoadVertexShader(Handle, Address) — store shader handle
            hle_set_vertex_shader(args, guest_mem);
            0
        }
        "Doom_D3D_MakeSpace" => {
            // Reset pushbuffer pointers so D3D runtime doesn't stall
            hle_reset_pushbuffer(guest_mem)
        }
        "D3DDevice_KickOff" => {
            // KickOff submits pushbuffer batch to GPU. Consume the commands,
            // then set GET = PUT everywhere the game reads it.
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                let dev_base = guest_mem as u64 + dev_ptr as u64;
                unsafe {
                    let put = *((dev_base + DEV_PB_PUT as u64) as *const u32);
                    let get = *((dev_base + DEV_PB_GET as u64) as *const u32);

                    // Consume pushbuffer commands
                    if put > get && put >= PB_DUMMY_BASE && get >= PB_DUMMY_BASE {
                        let result =
                            crate::xbox::aot::nv2a::consume_pushbuffer(guest_mem, get, put);
                        let draws = result.draw_arrays + result.draw_elements + result.draw_inline;
                        if draws > 0 {
                            OOVPA_DRAW_CALLS.fetch_add(draws as u64, Ordering::Relaxed);
                        }
                        OOVPA_PB_COMMANDS
                            .fetch_add(result.total_commands as u64, Ordering::Relaxed);
                        static KO_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = KO_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 5 || (draws > 0) {
                            crate::xbox::emulator::debug_log(&format!(
                                "[PB-KICKOFF] cmds={} draws={} clears={} begin_end={} verts={}",
                                result.total_commands,
                                draws,
                                result.clears,
                                result.begin_end,
                                result.vertex_data
                            ));
                        }
                    }

                    // 1. Device struct GET = PUT
                    *((dev_base + DEV_PB_GET as u64) as *mut u32) = put;
                    // 2. Guest memory NV2A USER channel
                    let nv2a_user = guest_mem as u64 + 0xFD800000u64;
                    *((nv2a_user + 0x40) as *mut u32) = put; // USER PUT
                    *((nv2a_user + 0x44) as *mut u32) = put; // USER GET
                }
            }
            // 3+4. NV2A shadow registers
            let put = crate::xbox::aot::nv2a::shadow_read(0x80_0040);
            let put_val = if put != 0 { put } else { PB_DUMMY_BASE };
            crate::xbox::aot::nv2a::shadow_write(0x80_0040, put_val);
            crate::xbox::aot::nv2a::shadow_write(0x80_0044, put_val);
            crate::xbox::aot::nv2a::shadow_write(0x3244, put_val);

            // Sync device structs for native D3D polling loop.
            // The loop at 0x002FA4F5 compares device[0] (PUT cursor) vs
            // pChannel[0x44] (NV2A USER GET). Both must match for the loop to exit.
            //
            // KickOff is __thiscall: ECX = device pointer (the 'this' pointer).
            // The polling loop uses THIS device from the stack, which may differ
            // from the global g_pDevice at [0x003038E0]. We must sync BOTH.
            let ecx_dev = (context.Rcx & 0xFFFF_FFFF) as u32;
            let global_dev = unsafe { *((guest_mem as u64 + 0x003038E0u64) as *const u32) };
            // Dump stack around R14 to find the thiscall device pointer
            let r14 = context.R14 as u32;
            let r15 = context.R15;
            let mut stack_vals = [0u32; 16];
            for i in 0..16 {
                stack_vals[i] = unsafe { *((r15 + (r14 + i as u32 * 4) as u64) as *const u32) };
            }
            crate::xbox::emulator::debug_log(&format!(
                "[KO-ECX] ECX=0x{:08X} global=0x{:08X} regs: EAX=0x{:08X} EBX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X}",
                ecx_dev, global_dev,
                context.Rax as u32, context.Rbx as u32, context.Rsi as u32, context.Rdi as u32,
            ));
            crate::xbox::emulator::debug_log(&format!(
                "[KO-STACK] R14=0x{:08X} +0: {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
                r14, stack_vals[0], stack_vals[1], stack_vals[2], stack_vals[3],
                stack_vals[4], stack_vals[5], stack_vals[6], stack_vals[7],
            ));
            crate::xbox::emulator::debug_log(&format!(
                "[KO-STACK] +32: {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
                stack_vals[8],
                stack_vals[9],
                stack_vals[10],
                stack_vals[11],
                stack_vals[12],
                stack_vals[13],
                stack_vals[14],
                stack_vals[15],
            ));

            // Polling loop fix: after KickOff returns (thiscall, cleanup=0),
            // the caller at 0x002FA4F5 reads [ESP+8] as the device pointer.
            // ESP after return = R14+4, so [ESP+8] = [R14+0xC].
            // The device pointer on the stack is NULL — write the real one.
            let caller_dev_slot = r15 + (r14 + 0x0C) as u64;
            let slot_val = unsafe { *(caller_dev_slot as *const u32) };
            if slot_val == 0 && global_dev != 0 {
                unsafe {
                    *(caller_dev_slot as *mut u32) = global_dev;
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[KO-STACK-FIX] [R14+0xC] was 0x{:08X}, wrote device=0x{:08X}",
                    slot_val, global_dev
                ));
            }

            // Also ensure zero-page pChannel is valid for fallback reads
            unsafe {
                // Zero-page [0x2264] = pChannel (in case device ptr is still 0)
                *((r15 + 0x2264) as *mut u32) = 0xFD80_0000;
            }

            // Sync helper: set pChannel and GET=device[0] on a device struct
            let sync_dev = |dev_addr: u32, label: &str| {
                if dev_addr == 0 || dev_addr >= 0x1000_0000 {
                    return;
                }
                let dev_base = guest_mem as u64 + dev_addr as u64;
                unsafe {
                    let dev0 = *((dev_base + 0x00) as *const u32);
                    let ch_ptr = *((dev_base + 0x2264) as *const u32);

                    // If pChannel is NULL, initialize it to NV2A USER channel base.
                    if ch_ptr == 0 {
                        *((dev_base + 0x2264) as *mut u32) = 0xFD80_0000;
                        *((dev_base + 0x2268) as *mut u32) = 0xFD80_0000;
                    }

                    // Sync shadow registers GET/PUT = device[0]
                    crate::xbox::aot::nv2a::shadow_write(0x80_0044, dev0);
                    crate::xbox::aot::nv2a::shadow_write(0x80_0040, dev0);
                    crate::xbox::aot::nv2a::shadow_write(0x3244, dev0);

                    // Zero-page offset 0x44 — fallback for pChannel=0 reads
                    *((guest_mem as u64 + 0x44) as *mut u32) = dev0;

                    // NV2A USER channel in guest memory
                    let nv2a_user = guest_mem as u64 + 0xFD800000u64;
                    *((nv2a_user + 0x40) as *mut u32) = dev0;
                    *((nv2a_user + 0x44) as *mut u32) = dev0;

                    let ch_verify = *((dev_base + 0x2264) as *const u32);
                    crate::xbox::emulator::debug_log(&format!(
                        "[KO-SYNC] {} dev=0x{:08X} dev[0]=0x{:08X} pCh=0x{:08X}→0x{:08X}",
                        label, dev_addr, dev0, ch_ptr, ch_verify
                    ));
                }
            };

            // Always sync the ECX (thiscall 'this') device — this is what the caller polls
            sync_dev(ecx_dev, "ECX");
            // Also sync global device if it's different
            if global_dev != ecx_dev {
                sync_dev(global_dev, "GPD");
            }
            0
        }
        "D3D_SetFence" | "D3DDevice_InsertFence" => {
            // This hooks D3D::SetFence (the inner function). Its prologue is:
            //   56 8B 35 [g_pDevice:4] — push esi; mov esi, [g_pDevice]
            // Extract g_pDevice from bytes 3-6 to ensure we read the right global.
            unsafe {
                let func = guest_mem.add(guest_addr as usize);
                if *func.add(1) == 0x8B && *func.add(2) == 0x35 {
                    let gpd = u32::from_le_bytes([
                        *func.add(3),
                        *func.add(4),
                        *func.add(5),
                        *func.add(6),
                    ]);
                    if gpd > 0x10000 && gpd < 0x800000 {
                        G_PDEVICE_DYNAMIC.store(gpd, Ordering::Relaxed);
                    }
                }
            }
            // Increment fence counter, mark as immediately complete
            static FENCE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
            let token = FENCE.fetch_add(1, Ordering::Relaxed);
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                let dev_base = guest_mem as u64 + dev_ptr as u64;
                unsafe {
                    *((dev_base + 0x30) as *mut u32) = token; // fence PUT
                    let get_cell = dev_ptr.wrapping_add(DEV_PB_GET);
                    *((dev_base + 0x34) as *mut u32) = get_cell; // pointer to fence GET
                    *((dev_base + DEV_PB_GET as u64) as *mut u32) = token; // fence GET = done
                }
            }
            hle_complete_d3d_wait(name, guest_mem);
            token
        }
        "D3DDevice_IsFencePending" => {
            // Fence is always immediately complete in HLE — not pending
            0 // FALSE = fence completed
        }
        "D3DDevice_BlockOnFence"
        | "D3D_BlockOnFence"
        | "D3D_BlockOnTime"
        | "D3D_BlockOnResource" => {
            // Fence is always immediately complete in HLE — just return
            hle_complete_d3d_wait(name, guest_mem)
        }
        n if crate::xbox::apu::is_dsound_hle_name(n) => {
            crate::xbox::apu::handle_dsound_hle(n, args, guest_mem)
        }
        "D3DDevice_DrawVertices" => {
            hle_draw(args, guest_mem, mmio_count);
            *draw_calls += 1; // Stage 10
            0
        }
        "D3DDevice_DrawIndexedVertices" => {
            hle_draw_indexed_vertices(args, guest_mem, mmio_count);
            *draw_calls += 1; // Stage 10
            0
        }
        "D3DDevice_GetBackBuffer" => {
            // GetBackBuffer(BackBuffer, Type, ppBackBuffer)
            // Read the surface from the device's surface table at [device + index*4 + 0x207C].
            // This matches what the guest GetBackBuffer code does natively.
            let back_buffer = args[0];
            let pp_surface = args[2];
            if let Some(pp_surface_norm) = guest_ram_offset(pp_surface) {
                let dev_ptr = crate::xbox::aot::oovpa::read_dev_ptr(guest_mem);
                let index = if back_buffer == 0xFFFF_FFFF {
                    1u32
                } else {
                    0u32
                };
                let surf = if dev_ptr != 0 {
                    unsafe {
                        *((guest_mem as u64 + dev_ptr as u64 + 0x207C + index as u64 * 4)
                            as *const u32)
                    }
                } else {
                    0
                };
                if surf != 0 {
                    unsafe {
                        *((guest_mem as u64 + pp_surface_norm as u64) as *mut u32) = surf;
                    }
                    debug_log(&format!(
                        "[HLE] GetBackBuffer: dev=0x{:08X} index={} → surface 0x{:08X}",
                        dev_ptr, index, surf
                    ));
                } else {
                    debug_log(&format!(
                        "[HLE] GetBackBuffer: dev=0x{:08X} index={} → NULL surface!",
                        dev_ptr, index
                    ));
                }
            }
            0 // D3D_OK
        }
        "D3DVertexBuffer_Lock"
        | "D3DVertexBuffer_Lock2"
        | "D3DIndexBuffer_Lock"
        | "D3DIndexBuffer_Lock2" => {
            // stdcall(5): D3DVertexBuffer_Lock(pVB, OffsetToLock, SizeToLock, ppbData, Flags)
            // stdcall(2): D3DVertexBuffer_Lock2(pVB, Flags) → pbData (return value)
            //
            // Context (2026-04-21): pattern matches at guest 0x2F4B30 in
            // Spider-Man XDK 4134. Before this handler existed the match
            // fell through to the default return-0 and *ppbData was never
            // populated. Game read ppbData expecting a buffer cursor, got
            // stale memory, wrote vertices to nowhere, never called Draw.
            // With 262144 VertexBuffer_Lock calls logged across a single
            // run and 0 draws, this was the exact wall.
            //
            // The Xbox D3DResource header (for Vertex/Index buffers) has
            // the guest data pointer at offset 0x04 (X_D3DResource.Data).
            // Top bits may be flag-encoded (0x03 or 0x8X), strip to get
            // the raw physical address. Then add OffsetToLock and write
            // that back to *ppbData.
            let p_vb = args[0];
            let offset = args[1];
            let _size = args[2];
            let pp_data = args[3];
            let _flags = args[4];

            static LOCK_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOCK_LOG.fetch_add(1, Ordering::Relaxed);

            if let (Some(p_vb_norm), Some(pp_data_norm)) =
                (guest_ram_offset(p_vb), guest_ram_offset(pp_data))
            {
                // Read VB resource Data field at offset 0x04
                let vb_resource = guest_mem as u64 + p_vb_norm as u64;
                let data_raw =
                    unsafe { std::ptr::read_unaligned((vb_resource + 0x04) as *const u32) };
                // [VB-LOCK-DUMP probe, 2026-04-21] Dump the full VB header
                // to correlate with the CreateVertexBuffer VB-VERIFY probe.
                // If Create logs `data=0x04000000` and Lock sees `common=0
                // data=0 size=0`, the header got zeroed between them.
                if n < 8 {
                    let (common, data, size) = unsafe {
                        (
                            std::ptr::read_unaligned((vb_resource + 0x00) as *const u32),
                            std::ptr::read_unaligned((vb_resource + 0x04) as *const u32),
                            std::ptr::read_unaligned((vb_resource + 0x08) as *const u32),
                        )
                    };
                    debug_log(&format!(
                        "[VB-LOCK-DUMP #{}] pVB=0x{:08X}: common=0x{:08X} data=0x{:08X} size=0x{:X}",
                        n + 1, p_vb, common, data, size
                    ));
                }
                // Xbox D3DResource.Data top bits encode a DMA class flag.
                // Real Xbox has 64MB RAM so phys address fits in 26 bits,
                // but our pool allocator uses the full 512MB reservation
                // (0x00000000..0x1FFFFFFF). The old 0x03FF_FFFF mask
                // truncated 0x04000000 to 0 — breaking Lock for any VB
                // whose Data field points past 64MB. Use a 512MB mask
                // to match our actual guest RAM extent. 2026-04-24.
                let data_addr = if data_raw >= 0x8000_0000 {
                    data_raw // 0x80000000-range: physical RAM mirror
                } else {
                    data_raw & 0x1FFF_FFFF
                };

                if data_addr != 0 {
                    let buffer_ptr = data_addr.wrapping_add(offset);
                    unsafe {
                        std::ptr::write_unaligned(
                            (guest_mem as u64 + pp_data_norm as u64) as *mut u32,
                            buffer_ptr,
                        );
                    }
                    if n < 8 {
                        debug_log(&format!(
                            "[HLE] D3DVertexBuffer_Lock #{}: pVB=0x{:08X} data=0x{:08X} offset=0x{:X} → *ppbData=0x{:08X}",
                            n + 1, p_vb, data_addr, offset, buffer_ptr
                        ));
                    }
                } else {
                    if n < 8 {
                        debug_log(&format!(
                            "[HLE] D3DVertexBuffer_Lock #{}: pVB=0x{:08X} has NULL Data field (resource not bound?)",
                            n + 1, p_vb
                        ));
                    }
                    // Fallback: hand out a slot from our heap pool so the
                    // game has SOMETHING to write to. 64KB per lock.
                    // Temporary — better to find why Data is zero.
                    unsafe {
                        std::ptr::write_unaligned(
                            (guest_mem as u64 + pp_data_norm as u64) as *mut u32,
                            0x1C00_0000u32, // a safe guest RAM address above our heap pool
                        );
                    }
                }
            }
            0 // D3D_OK
        }
        "D3DResource_IsBusy" | "D3DDevice_IsBusy" | "D3D_IsBusy" => {
            // Always report GPU as idle — HLE render path completes
            // synchronously, so no resource is ever "busy" from the
            // guest's perspective. Game's IsBusy-polling loops exit
            // on the first call.
            static ISBUSY_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = ISBUSY_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 3 {
                debug_log(&format!(
                    "[HLE] {} #{}: returning 0 (not busy)",
                    name,
                    n + 1
                ));
            }
            0 // FALSE = not busy
        }
        "Lock2DSurface" => {
            // Lock2DSurface(pPixelContainer, FaceType, Level, pLockedRect, pRect, Flags)
            // ret 0x18 = 6 args. Writes D3DLOCKED_RECT {Pitch, pBits} to args[3].
            let pixel_container = args[0];
            let p_locked_rect = args[3];

            static LOCK_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOCK_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 5 {
                debug_log(&format!(
                    "[HLE] Lock2DSurface: call #{}, container=0x{:08X} pLockedRect=0x{:08X}",
                    n + 1,
                    pixel_container,
                    p_locked_rect
                ));
            }

            let pixel_container_norm = guest_ram_offset(pixel_container);
            let locked_rect_norm = guest_ram_offset(p_locked_rect);
            if pixel_container_norm.is_none() {
                // NULL container — return D3DERR_INVALIDCALL so caller's error path kicks in
                0x8876_086Cu32
            } else if let (Some(pixel_container_norm), Some(locked_rect_norm)) =
                (pixel_container_norm, locked_rect_norm)
            {
                let sp = unsafe { guest_mem.add(pixel_container_norm) };
                let data = unsafe { *(sp.add(0x04) as *const u32) }; // X_D3DResource.Data
                let w = unsafe { *(sp.add(0x1C) as *const u32) }; // Width
                let pitch = if w > 0 { w * 4 } else { 640 * 4 };
                let data_addr = normalize_guest_ram_ptr(data & !0x3).unwrap_or(0x00F0_0000);
                unsafe {
                    let base = guest_mem.add(locked_rect_norm);
                    *(base as *mut u32) = pitch;
                    *(base.add(4) as *mut u32) = data_addr;
                }
                static LOCK2D_NORM_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let k = LOCK2D_NORM_LOG.fetch_add(1, Ordering::Relaxed);
                if k < 16 || k.is_power_of_two() {
                    debug_log(&format!(
                        "[HLE] Lock2DSurface normalized: container=0x{:08X}->0x{:08X} pLockedRect=0x{:08X}->0x{:08X} data=0x{:08X}->0x{:08X} pitch={}",
                        pixel_container,
                        pixel_container_norm as u32,
                        p_locked_rect,
                        locked_rect_norm as u32,
                        data,
                        data_addr,
                        pitch
                    ));
                }
                0 // D3D_OK
            } else {
                0 // D3D_OK
            }
        }
        "D3DSurface_GetDesc" => {
            // stdcall(2): (pSurface, pDesc). The DDS loader reads Format
            // and Size from this struct before deciding how many bytes to
            // copy into the locked texture. Returning success with zeros
            // makes every organic LEGAL.xbs texture fail immediately.
            let p_surface = args[0];
            let out_ptr = args[1];
            if let Some((fmt, w, h, size)) = write_surface_desc(guest_mem, p_surface, out_ptr) {
                static SURF_DESC_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SURF_DESC_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 20 {
                    debug_log(&format!(
                        "[HLE] Surface_GetDesc: surf=0x{:08X} fmt=0x{:02X} {}x{} size={} out=0x{:08X}",
                        p_surface, fmt, w, h, size, out_ptr
                    ));
                }
            }
            0
        }
        "Get2DSurfaceDesc" | "D3D_GetSurfaceInfo" => {
            // stdcall(3): (pPixelContainer, Level, pDesc). ret 12.
            let p_surface = args[0];
            let _level = args[1];
            let out_ptr = args[2];
            // Try the HLE texture registry first; fake Xbox resources may live
            // in the 0x83xxxxxx mirror while SetTexture reads the low alias.
            let (w, h, format, data, pitch, size) =
                if let Some(key) = normalize_guest_ram_ptr(p_surface) {
                    let registry = lookup_texture_info(key);
                    let sp = unsafe { guest_mem.add(key as usize) };
                    let header_data = unsafe { *(sp.add(0x04) as *const u32) };
                    let header_format = unsafe { *(sp.add(0x0C) as *const u32) };
                    let size_field = unsafe { *(sp.add(0x10) as *const u32) };
                    let header_width = unsafe { *(sp.add(0x1C) as *const u32) };
                    let header_height = unsafe { *(sp.add(0x20) as *const u32) };
                    let format = if header_format != 0 {
                        header_format
                    } else {
                        registry.map(|info| info.format).unwrap_or(0x06)
                    };
                    let fmt_code = xbox_texture_format_code(format);
                    let container_measures = xbox_pixel_container_measures(format, size_field);
                    let w = registry
                        .map(|info| info.width)
                        .filter(|w| sane_texture_dims(*w, 1))
                        .or_else(|| {
                            if sane_texture_dims(header_width, 1) {
                                Some(header_width)
                            } else if let Some((w, _h, _pitch)) = container_measures {
                                Some(w)
                            } else if size_field != 0 {
                                Some((size_field & 0x0FFF).saturating_add(1))
                            } else {
                                None
                            }
                        })
                        .unwrap_or(640);
                    let h = registry
                        .map(|info| info.height)
                        .filter(|h| sane_texture_dims(1, *h))
                        .or_else(|| {
                            if sane_texture_dims(1, header_height) {
                                Some(header_height)
                            } else if let Some((_w, h, _pitch)) = container_measures {
                                Some(h)
                            } else if size_field != 0 {
                                Some(((size_field >> 12) & 0x0FFF).saturating_add(1))
                            } else {
                                None
                            }
                        })
                        .unwrap_or(480);
                    let pitch = container_measures
                        .map(|(_w, _h, p)| p)
                        .filter(|p| sane_pitch_for_format(*p, w, fmt_code))
                        .unwrap_or_else(|| texture_pitch_bytes(w, format));
                    let size = texture_byte_len(w, h, format);
                    let data = normalize_guest_ram_ptr(header_data & !0x3)
                        .or_else(|| registry.map(|info| info.data))
                        .unwrap_or(0);
                    (w, h, format, data, pitch, size)
                } else {
                    (640, 480, 0x06, 0, 640 * 4, 640 * 480 * 4)
                };
            if let Some(key) = normalize_guest_ram_ptr(p_surface) {
                upsert_texture_info(HleTextureInfo {
                    key,
                    width: w,
                    height: h,
                    format,
                    data,
                    pitch,
                    swizzled: false,
                });
            }
            if let Some(out_norm) = guest_ram_offset(out_ptr) {
                let base = guest_mem as u64 + out_norm as u64;
                unsafe {
                    *((base + 0x00) as *mut u32) = xbox_texture_format_code(format);
                    *((base + 0x04) as *mut u32) = match xbox_texture_format_code(format) {
                        0x0C | 0x0E | 0x0F => 0,
                        0x00 | 0x0B | 0x19 => 1,
                        0x02 | 0x03 | 0x04 | 0x05 | 0x10 | 0x11 | 0x1C | 0x1D | 0x1F | 0x20 => 2,
                        _ => 4,
                    };
                    *((base + 0x08) as *mut u32) = 0; // type: surface
                    *((base + 0x0C) as *mut u32) = pitch;
                    *((base + 0x10) as *mut u32) = size;
                    *((base + 0x14) as *mut u32) = w; // width
                    *((base + 0x18) as *mut u32) = h; // height
                }
            }
            debug_log(&format!(
                "[HLE] Get2DSurfaceDesc: surf=0x{:08X} fmt=0x{:02X} → {}x{} pitch={} size={} out=0x{:08X}",
                p_surface, xbox_texture_format_code(format), w, h, pitch, size, out_ptr
            ));
            0
        }
        // ---- D3D resource creation / management (SDK functions, not game code) ----
        "D3DTexture_LockRect" | "D3DSurface_LockRect" | "D3DCubeTexture_LockRect" => {
            let p_texture = args[0];
            let (level, p_locked_rect, p_rect, flags) = match name {
                // stdcall(4): (pSurface, pLockedRect, pRect, Flags). ret 16.
                "D3DSurface_LockRect" => (0, args[1], args[2], args[3]),
                // Texture LockRect is stdcall(5): (pTexture, Level, pLockedRect, pRect, Flags).
                // Keep the existing cube-texture argument lane for now; the active Spider-Man
                // corruption is on D3DTexture_LockRect and cube variants differ by SDK/LTCG.
                _ => (args[1], args[2], args[3], args[4]),
            };

            if let Some(locked_rect_norm) = guest_ram_offset(p_locked_rect) {
                // Read texture header/registry to get Data pointer and dimensions.
                let (data_addr, pitch, mip_level, mip_w, mip_h, mip_offset, base_data) =
                    if let Some(tex_key) = normalize_guest_ram_ptr(p_texture) {
                        let tp = unsafe { guest_mem.add(tex_key as usize) };
                        let data = unsafe { *(tp.add(0x04) as *const u32) }; // X_D3DResource.Data
                        let format = unsafe { *(tp.add(0x0C) as *const u32) };
                        let size_field = unsafe { *(tp.add(0x10) as *const u32) }; // Size encoding
                        let registry = lookup_texture_info(tex_key);
                        let (w, h) = registry
                            .filter(|info| sane_texture_dims(info.width, info.height))
                            .map(|info| (info.width, info.height))
                            .unwrap_or_else(|| {
                                if size_field != 0 {
                                    (
                                        ((size_field & 0x00000FFF) + 1) as u32,
                                        (((size_field >> 12) & 0x00000FFF) + 1) as u32,
                                    )
                                } else {
                                    (640, 480)
                                }
                            });
                        let w = w.clamp(1, 4096);
                        let h = h.clamp(1, 4096);
                        let expected_pitch = texture_pitch_bytes(w, format);
                        let alloc_len = texture_mip_chain_byte_len(w, h, format);
                        let pitch = registry
                            .map(|info| info.pitch)
                            .filter(|p| *p >= expected_pitch)
                            .unwrap_or(expected_pitch);
                        let da = normalize_guest_ram_ptr(data & !0x3).unwrap_or_else(|| {
                            // Allocate pixel data for this texture and write it back
                            // to the fake header so later SetTexture can bind it.
                            let alloc_raw = alloc_fake_d3d_obj(guest_mem, alloc_len.max(pitch));
                            let alloc = normalize_guest_ram_ptr(alloc_raw).unwrap_or(alloc_raw);
                            if alloc != 0 {
                                unsafe {
                                    *(tp.add(0x04) as *mut u32) = alloc;
                                }
                            }
                            alloc
                        });
                        let info = HleTextureInfo {
                            key: tex_key,
                            width: w,
                            height: h,
                            format,
                            data: da,
                            pitch,
                            swizzled: false,
                        };
                        upsert_texture_info(info);
                        mark_texture_dirty_from_lock(info, "LockRect");
                        probe_spidey_fullscreen_lockrect(info, guest_mem);
                        doom_record_lockrect(
                            guest_mem,
                            p_texture,
                            level,
                            p_locked_rect,
                            p_rect,
                            flags,
                            info,
                        );
                        let (mip_w, mip_h, mip_level) = texture_mip_dims(w, h, level);
                        let mip_offset = texture_mip_offset(w, h, format, mip_level);
                        let mip_pitch = texture_pitch_bytes(mip_w, format);
                        let mip_data = da.saturating_add(mip_offset);
                        if level != 0 || mip_level != 0 {
                            static MIP_LOCK_LOG: AtomicU32 = AtomicU32::new(0);
                            let n = MIP_LOCK_LOG.fetch_add(1, Ordering::Relaxed);
                            if n < 256 || n.is_power_of_two() {
                                debug_log(&format!(
                                "[HLE-MIP-LOCK] #{} {} tex=0x{:08X} req_level={} eff_level={} base=0x{:08X} mip=0x{:08X} offset={} dims={}x{} pitch={} fmt=0x{:08X}/0x{:02X}/{}",
                                n,
                                name,
                                p_texture,
                                level,
                                mip_level,
                                da,
                                mip_data,
                                mip_offset,
                                mip_w,
                                mip_h,
                                mip_pitch,
                                format,
                                xbox_texture_format_code(format),
                                crate::xbox::gpu::texture_format::format_name(xbox_texture_format_code(format))
                            ));
                            }
                        }
                        (mip_data, mip_pitch, mip_level, mip_w, mip_h, mip_offset, da)
                    } else {
                        // NULL texture — allocate scratch buffer so game doesn't crash
                        let alloc = alloc_fake_d3d_obj(guest_mem, 640 * 480 * 4);
                        (alloc, 640 * 4, 0, 640, 480, 0, alloc)
                    };

                if data_addr != 0 {
                    unsafe {
                        let base = guest_mem.add(locked_rect_norm);
                        *(base as *mut u32) = pitch;
                        *(base.add(4) as *mut u32) = data_addr;
                    }
                }
                debug_log(&format!(
                    "[HLE] LockRect: tex=0x{:08X} level={} mip={} {}x{} base=0x{:08X} offset={} pLockedRect=0x{:08X}->0x{:08X} pRect=0x{:08X} flags=0x{:08X} → pBits=0x{:08X} pitch={}",
                    p_texture,
                    level,
                    mip_level,
                    mip_w,
                    mip_h,
                    base_data,
                    mip_offset,
                    p_locked_rect,
                    locked_rect_norm as u32,
                    p_rect,
                    flags,
                    data_addr,
                    pitch
                ));
            }
            0 // D3D_OK
        }
        "D3DTexture_GetSurfaceLevel" => {
            // stdcall(3): (pTexture, Level, ppSurfaceLevel). ret 12.
            let p_texture = args[0];
            let _level = args[1];
            let pp_surface = args[2];
            if let Some(pp_surface_norm) = guest_ram_offset(pp_surface) {
                let surf = if valid_guest_ptr(p_texture) {
                    p_texture // Xbox D3D8 textures and surfaces share the same base struct
                } else {
                    // Allocate a fake surface with pixel data backing
                    let s = alloc_fake_d3d_obj(guest_mem, 0x48);
                    if s != 0 {
                        let data = alloc_fake_d3d_obj(guest_mem, 640 * 480 * 4);
                        unsafe {
                            let sp = guest_mem.add(s as usize);
                            *(sp as *mut u32) = 0x0001_0001;
                            *(sp.add(0x04) as *mut u32) = data;
                            *(sp.add(0x0C) as *mut u32) = 0x12; // LIN_A8R8G8B8
                            *(sp.add(0x1C) as *mut u32) = 640;
                            *(sp.add(0x20) as *mut u32) = 480;
                            *(sp.add(0x24) as *mut u32) = 640 * 4;
                        }
                    }
                    s
                };
                unsafe {
                    *((guest_mem as u64 + pp_surface_norm as u64) as *mut u32) = surf;
                }
                debug_log(&format!(
                    "[HLE] GetSurfaceLevel: tex=0x{:08X} level={} → surf=0x{:08X} ppSurf=0x{:08X}->0x{:08X}",
                    p_texture, _level, surf, pp_surface, pp_surface_norm as u32
                ));
            }
            0
        }
        "D3DDevice_CreateVertexBuffer" | "D3DDevice_CreateVertexBuffer2" => {
            // CreateVertexBuffer(Length, Usage, FVF, Pool, ppVertexBuffer)
            // Allocate a fake VB object with a Data region the game can write vertices to.
            let length = args[0];
            let pp_vb = args[4];
            if let Some(pp_vb_norm) = guest_ram_offset(pp_vb) {
                // Allocate VB header (0x14 bytes) + data region
                let vb_addr = alloc_fake_d3d_obj(guest_mem, 0x14);
                let data_addr = alloc_fake_d3d_obj(guest_mem, length.max(4096));
                if vb_addr != 0 {
                    unsafe {
                        let vb = guest_mem.add(vb_addr as usize);
                        *(vb as *mut u32) = 0x0001_0001; // Common: type + refcount
                        *(vb.add(0x04) as *mut u32) = data_addr; // Data pointer
                        *(vb.add(0x08) as *mut u32) = length; // Size
                    }
                    unsafe {
                        *((guest_mem as u64 + pp_vb_norm as u64) as *mut u32) = vb_addr;
                    }
                    // [VB-VERIFY probe, 2026-04-21] Immediately re-read the VB
                    // header we just wrote, via BOTH the address form used by
                    // the write (`guest_mem.add(vb_addr)`) and the form used
                    // by VertexBuffer_Lock (`guest_mem as u64 + vb_addr as u64`).
                    // They should be identical host pointers; if they diverge
                    // we've got a mirror-mapping problem. Also dump the
                    // ppVertexBuffer slot and the full [0x5D2BA0..0x5D2BAC]
                    // table to see which slots are populated.
                    let (readback_common, readback_data, readback_size) = unsafe {
                        let vb_read = guest_mem.add(vb_addr as usize);
                        (
                            *(vb_read as *const u32),
                            *(vb_read.add(0x04) as *const u32),
                            *(vb_read.add(0x08) as *const u32),
                        )
                    };
                    let readback_via_u64 = unsafe {
                        std::ptr::read_unaligned(
                            ((guest_mem as u64) + (vb_addr as u64) + 0x04) as *const u32,
                        )
                    };
                    let pp_vb_contents = unsafe {
                        std::ptr::read_unaligned(
                            ((guest_mem as u64) + (pp_vb_norm as u64)) as *const u32,
                        )
                    };
                    // VB pointer table dump (assumes ppVertexBuffer lies in a
                    // contiguous table of slots)
                    let table_base = (pp_vb_norm as u32) & !0xFu32;
                    let mut table_dump = String::new();
                    for i in 0..4u32 {
                        let slot_addr = table_base + i * 4;
                        let slot_val = unsafe {
                            std::ptr::read_unaligned(
                                ((guest_mem as u64) + (slot_addr as u64)) as *const u32,
                            )
                        };
                        if !table_dump.is_empty() {
                            table_dump.push(' ');
                        }
                        table_dump.push_str(&format!("[0x{:08X}]=0x{:08X}", slot_addr, slot_val));
                    }
                    debug_log(&format!(
                        "[HLE] CreateVertexBuffer: len={} → vb=0x{:08X} data=0x{:08X} \
                         readback:(common=0x{:08X} data=0x{:08X} size=0x{:X}) \
                         via_u64:data=0x{:08X} *ppVB=0x{:08X} table_base=0x{:08X} {{{}}}",
                        length,
                        vb_addr,
                        data_addr,
                        readback_common,
                        readback_data,
                        readback_size,
                        readback_via_u64,
                        pp_vb_contents,
                        table_base,
                        table_dump
                    ));
                }
            }
            0
        }
        "D3DDevice_CreateImageSurface" | "D3D_CreateStandAloneSurface" => {
            // stdcall(4): (Width, Height, Format, ppSurface). ret 16.
            let width = args[0];
            let height = args[1];
            let _format = args[2];
            let pp_surface = args[3];
            let w = if width > 0 && width <= 4096 {
                width
            } else {
                640
            };
            let h = if height > 0 && height <= 4096 {
                height
            } else {
                480
            };
            if let Some(pp_surface_norm) = guest_ram_offset(pp_surface) {
                let surf_addr = alloc_fake_d3d_obj(guest_mem, 0x48);
                let data_addr = alloc_fake_d3d_obj(guest_mem, w * h * 4);
                if surf_addr != 0 {
                    unsafe {
                        let s = guest_mem.add(surf_addr as usize);
                        *(s as *mut u32) = 0x0001_0001; // Common: type + refcount
                        *(s.add(0x04) as *mut u32) = data_addr; // Data
                        *(s.add(0x0C) as *mut u32) = 0x12; // Format: LIN_A8R8G8B8
                        *(s.add(0x1C) as *mut u32) = w; // Width
                        *(s.add(0x20) as *mut u32) = h; // Height
                        *(s.add(0x24) as *mut u32) = w * 4; // Pitch
                    }
                    unsafe {
                        *((guest_mem as u64 + pp_surface_norm as u64) as *mut u32) = surf_addr;
                    }
                    debug_log(&format!(
                        "[HLE] {}: {}x{} ppSurf=0x{:08X}->0x{:08X} → surf=0x{:08X} data=0x{:08X}",
                        name, w, h, pp_surface, pp_surface_norm as u32, surf_addr, data_addr
                    ));
                }
            } else {
                debug_log(&format!(
                    "[HLE] {}: {}x{} ppSurf=0x{:08X} — INVALID output pointer!",
                    name, w, h, pp_surface
                ));
            }
            0
        }
        "D3DResource_Release" => {
            // Decrement refcount. We don't actually free fake objects (bump allocator).
            0
        }
        "D3D_SetPushBufferSize" => {
            // D3D_SetPushBufferSize(Size, KickOffSize) — called before CreateDevice.
            // Store for later use but nothing to do (our CreateDevice handles PB init).
            0
        }
        // XAPI stubs: must return TRUE (1) to prevent game aborting to dashboard
        "XMountUtilityDrive" | "XMountAlternateTitleA" | "XUnmountAlternateTitleA" => {
            // Mount operations succeed (we handle file I/O via kernel ordinals)
            1 // TRUE
        }
        "XMountMUA" | "XMountMURootA" => {
            // Memory units are not emulated. If XGetDevices lies and reports an
            // MU, Spider-Man consumes the uninitialized drive buffer and opens
            // garbage paths like "\x14:\". Fail the mount cleanly and clear the
            // output string so callers cannot reuse stack junk as a drive.
            write_guest_nul_string(guest_mem, args[2]);
            static MU_MOUNT_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = MU_MOUNT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 8 || n.is_power_of_two() {
                debug_log(&format!(
                    "[OOVPA-HLE] {} port={} slot={} out=0x{:08X} -> ERROR_DEVICE_NOT_CONNECTED",
                    name, args[0], args[1], args[2]
                ));
            }
            if menu_wait_trace_log(n) {
                debug_log(&format!(
                    "[MENU-WAIT] {} #{} port={} slot={} out=0x{:08X} ret=0x0000048F",
                    name, n, args[0], args[1], args[2]
                ));
            }
            0x0000_048F // ERROR_DEVICE_NOT_CONNECTED
        }
        "XUnmountMU" => 0,
        "XapiMapLetterToDirectory" => {
            // Drive letter mapping — succeed silently
            1 // TRUE
        }
        "XGetLaunchInfo" => hle_xget_launch_info(args, guest_mem),
        "XapiBootToDash" => {
            // Game wants to reboot to dashboard — block this, return 0
            debug_log("[HLE] XapiBootToDash BLOCKED — game tried to reboot to dashboard");
            0
        }
        // ---- XPP USB driver stubs ----
        "XPP_Init" => 1,
        "XInitDevices" => xapi_hle_init_devices(guest_mem),
        "XGetDevices" => xapi_hle_get_devices(guest_mem, args[0], "XGetDevices"),
        "XGetDeviceChanges" => xapi_hle_get_device_changes(guest_mem, args[0], args[1], args[2]),
        "XInputOpen" => {
            // XInputOpen(DeviceType, dwPort, dwSlot, pPollingParams) → HANDLE
            // Model a single connected gamepad on port 0. Returning handles
            // for every port makes games scan unpolled ghost controller
            // objects, so scripted/real input written to port 0 is ignored.
            let port = args[1];
            if port == 0 {
                debug_log(&format!(
                    "[OOVPA-HLE] XInputOpen: port={}, returning fake handle",
                    port
                ));
                0x8000_0000
            } else {
                debug_log(&format!(
                    "[OOVPA-HLE] XInputOpen: port={}, no device connected",
                    port
                ));
                0
            }
        }
        "XInputGetState" => {
            // XInputGetState(hDevice, pState) → DWORD (ERROR_SUCCESS = 0)
            // XINPUT_STATE layout (Xbox-specific):
            //   +0x00  DWORD dwPacketNumber
            //   +0x04  WORD  wButtons
            //   +0x06  BYTE  bAnalogButtons[8]    (A, B, X, Y, Black, White, LT, RT)
            //   +0x0E  SHORT sThumbLX
            //   +0x10  SHORT sThumbLY
            //   +0x12  SHORT sThumbRX
            //   +0x14  SHORT sThumbRY
            //
            // E-session wire-up (2026-04-22): read real libretro input for
            // port 0 and translate to Xbox XINPUT_GAMEPAD bits. Replaces the
            // earlier synthetic "START held briefly on polls 60..90" probe,
            // which fired during boot logo before Spider-Man's menu state
            // machine was listening (game polls ~34x/sec, accumulated 8,192+
            // polls in 60 seconds while sitting on title screen — our window
            // closed long before the menu state machine was checking buttons).
            let state_ptr = args[1];
            if state_ptr == 0 || state_ptr >= 0x2000_0000 {
                return 0;
            }

            const JP_SOUTH: u32 = 0;
            const JP_WEST: u32 = 1;
            const JP_SELECT: u32 = 2; // Libretro "Select" label; Xbox BACK button.
            const JP_START: u32 = 3; // Xbox START
            const JP_UP: u32 = 4;
            const JP_DOWN: u32 = 5;
            const JP_LEFT: u32 = 6;
            const JP_RIGHT: u32 = 7;
            const JP_EAST: u32 = 8;
            const JP_NORTH: u32 = 9;
            const JP_L: u32 = 10; // Xbox BLACK
            const JP_R: u32 = 11; // Xbox WHITE
            const JP_L2: u32 = 12; // Xbox LEFT_TRIGGER
            const JP_R2: u32 = 13; // Xbox RIGHT_TRIGGER
            const JP_L3: u32 = 14; // Xbox LEFT_THUMB (click)
            const JP_R3: u32 = 15; // Xbox RIGHT_THUMB (click)

            // The libretro callbacks are polled on the frontend thread in
            // retro_run(); the guest worker reads this atomic snapshot.
            let (raw_joypad_bits, mut lx, mut ly, rx, ry) = crate::xbox::emulator::input_snapshot();
            let latched_joypad_bits = crate::xbox::emulator::input_latched_edge_bits();
            let joypad_bits = raw_joypad_bits | latched_joypad_bits;
            if latched_joypad_bits != 0 {
                static INPUT_LATCH_CONSUME_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let latch_n =
                    INPUT_LATCH_CONSUME_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if latch_n < 16 || latch_n.is_power_of_two() {
                    debug_log(&format!(
                        "[HLE-INPUT-LATCH] #{} raw=0x{:04X} latched=0x{:04X} effective=0x{:04X}",
                        latch_n, raw_joypad_bits, latched_joypad_bits, joypad_bits
                    ));
                }
            }
            let rd = |id: u32| -> bool { (joypad_bits & (1u32 << id)) != 0 };

            static POLL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = POLL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            // Digital buttons → Xbox XINPUT_GAMEPAD.wButtons bits
            let mut buttons: u16 = 0;
            let select_down = rd(JP_SELECT);
            let start_down = rd(JP_START);
            if rd(JP_UP) {
                buttons |= 0x0001;
            } // DPAD_UP
            if rd(JP_DOWN) {
                buttons |= 0x0002;
            } // DPAD_DOWN
            if rd(JP_LEFT) {
                buttons |= 0x0004;
            } // DPAD_LEFT
            if rd(JP_RIGHT) {
                buttons |= 0x0008;
            } // DPAD_RIGHT
            if start_down {
                buttons |= 0x0010;
            } // START
            if select_down {
                buttons |= 0x0020;
            } // BACK
            if rd(JP_L3) {
                buttons |= 0x0040;
            } // LEFT_THUMB
            if rd(JP_R3) {
                buttons |= 0x0080;
            } // RIGHT_THUMB

            // Analog buttons (pressure 0..255); libretro digital → on=255
            let pressure = |id: u32| -> u8 {
                if rd(id) {
                    0xFFu8
                } else {
                    0
                }
            };
            // Libretro names buttons in SNES layout terms: B=south, A=east,
            // Y=west, X=north. Translate by physical position to Xbox ABXY.
            let mut a_a = pressure(JP_SOUTH);
            let mut a_b = pressure(JP_EAST);
            let mut a_x = pressure(JP_WEST);
            let mut a_y = pressure(JP_NORTH);
            let a_black = pressure(JP_L);
            let a_white = pressure(JP_R);
            let a_lt = pressure(JP_L2);
            let a_rt = pressure(JP_R2);
            let normal_buttons = buttons;
            let normal_a_a = a_a;
            let normal_a_b = a_b;
            let normal_a_x = a_x;
            let normal_a_y = a_y;
            let normal_lx = lx;
            let normal_ly = ly;
            let spidey_synth_training = crate::xbox::emulator::spidey_synth_training_enabled();
            let spidey_synth_passthrough =
                crate::xbox::emulator::spidey_synth_training_input_passthrough();
            let spidey_fast_originz =
                spidey_synth_training || std::env::var_os("RUSTEMU_SPIDEY_FAST_ORIGINZ").is_some();
            let spidey_auto_new_game = spidey_fast_originz
                || std::env::var_os("RUSTEMU_SPIDEY_AUTONEWGAME").is_some()
                || std::env::var_os("RUSTEMU_TEST_AUTOPRESS_NEW_GAME").is_some();
            let spidey_auto_level_confirm = spidey_auto_new_game
                || spidey_synth_training
                || std::env::var_os("RUSTEMU_TEST_AUTOPRESS_LEVEL_CONFIRM").is_some()
                || std::env::var_os("RUSTEMU_SPIDEY_AUTONEWGAME_LEVEL_CONFIRM").is_some();

            apply_doom_autonewgame_input("XInputGetState", &mut buttons, &mut a_a);

            // Boot-level scripted input: the scene-aware verifier below only
            // starts once Spider-Man has published bonus\menu/M0menu\menu.
            // The first title/legal "Press Start" screen can appear before
            // those scene strings are stable, so emit clean START edges from
            // the first XInput polls when the autopress verifier is enabled.
            let boot_test_autopress_start =
                std::env::var_os("RUSTEMU_TEST_AUTOPRESS_START").is_some() || spidey_auto_new_game;
            let boot_title_start_only = spidey_title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_TITLE_START_ONLY").is_some();
            let boot_test_autopress_confirm =
                std::env::var_os("RUSTEMU_TEST_AUTOPRESS_CONFIRM").is_some();
            if boot_test_autopress_start || boot_title_start_only {
                static BOOT_START_PULSE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static BOOT_TITLE_START_DONE: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                static BOOT_TITLE_START_READY_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let read_u32 = |addr: u32| -> u32 {
                    if (0x1000..0x2000_0000).contains(&addr) {
                        unsafe {
                            std::ptr::read_unaligned((guest_mem as u64 + addr as u64) as *const u32)
                        }
                    } else {
                        0
                    }
                };
                let valid_low_ptr = |addr: u32| -> bool { (0x1000..0x2000_0000).contains(&addr) };
                let frame_ctx = read_u32(0x004B_C630);
                let frame_scene = if valid_low_ptr(frame_ctx) {
                    read_u32(frame_ctx.wrapping_add(0x18))
                } else {
                    0
                };
                let frame_root = if valid_low_ptr(frame_scene) {
                    read_u32(frame_scene.wrapping_add(0x28))
                } else {
                    0
                };
                let title_scene_ready = valid_low_ptr(frame_ctx)
                    && valid_low_ptr(frame_scene)
                    && valid_low_ptr(frame_root);
                if title_scene_ready {
                    let was_done =
                        BOOT_TITLE_START_DONE.swap(true, std::sync::atomic::Ordering::Relaxed);
                    if !was_done {
                        let log_n = BOOT_TITLE_START_READY_LOG
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        debug_log(&format!(
                            "[SPIDEY-BOOT-AUTOPRESS-READY] #{} poll={} frame=0x{:08X} scene=0x{:08X} root=0x{:08X}; releasing START",
                            log_n, n, frame_ctx, frame_scene, frame_root
                        ));
                    }
                }
                let title_start_delay = if boot_title_start_only {
                    std::env::var("RUSTEMU_SPIDEY_TITLE_START_DELAY")
                        .ok()
                        .and_then(|v| v.trim().parse::<u32>().ok())
                        .unwrap_or(0)
                } else {
                    0
                };
                let title_start_width = if boot_title_start_only {
                    std::env::var("RUSTEMU_SPIDEY_TITLE_START_WIDTH")
                        .ok()
                        .and_then(|v| v.trim().parse::<u32>().ok())
                        .filter(|v| (3..=4096).contains(v))
                        .unwrap_or(720)
                } else {
                    0
                };
                let title_start_interval = if boot_title_start_only {
                    std::env::var("RUSTEMU_SPIDEY_TITLE_START_INTERVAL")
                        .ok()
                        .and_then(|v| v.trim().parse::<u32>().ok())
                        .filter(|v| (4..=240).contains(v))
                        .unwrap_or(24)
                } else {
                    1
                };
                let title_start_pulse_width = if boot_title_start_only {
                    std::env::var("RUSTEMU_SPIDEY_TITLE_START_PULSE_WIDTH")
                        .ok()
                        .and_then(|v| v.trim().parse::<u32>().ok())
                        .filter(|v| (1..=60).contains(v))
                        .unwrap_or(3)
                        .min(title_start_interval)
                } else {
                    0
                };
                let boot_window = if boot_title_start_only {
                    title_start_delay.saturating_add(title_start_width)
                } else {
                    720
                };
                let boot_sequence_done = BOOT_TITLE_START_DONE
                    .load(std::sync::atomic::Ordering::Relaxed)
                    || SPIDEY_MENU_SELECTOR_SEEN.load(std::sync::atomic::Ordering::Relaxed)
                    || SPIDEY_VISIBLE_MENU_CONFIRM_STARTED
                        .load(std::sync::atomic::Ordering::Relaxed);
                let boot_pulse = if boot_title_start_only {
                    let pulse_phase = n.saturating_sub(title_start_delay) % title_start_interval;
                    !boot_sequence_done
                        && n >= title_start_delay
                        && n < boot_window
                        && pulse_phase < title_start_pulse_width
                } else {
                    !boot_sequence_done && n < boot_window && (n % 24) < 3
                };
                let boot_back_edge =
                    boot_test_autopress_confirm && n < boot_window && (n % 48) == 12;
                if boot_pulse {
                    buttons |= 0x0010; // START
                }
                if boot_back_edge {
                    buttons |= 0x0020; // BACK (Spider-Man action 0x15)
                    note_spidey_manual_confirm_edge();
                }
                if boot_pulse || boot_back_edge {
                    let log_n =
                        BOOT_START_PULSE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_n < 16 || log_n.is_power_of_two() {
                        debug_log(&format!(
                            "[SPIDEY-BOOT-AUTOPRESS] #{} poll={} START={} BACK={} title_only={} frame=0x{:08X} scene=0x{:08X} root=0x{:08X} delay={} width={} interval={} pulse_width={} done={}",
                            log_n,
                            n,
                            if boot_pulse { 1 } else { 0 },
                            if boot_back_edge { 1 } else { 0 },
                            if boot_title_start_only { 1 } else { 0 },
                            frame_ctx,
                            frame_scene,
                            frame_root,
                            title_start_delay,
                            title_start_width,
                            title_start_interval,
                            title_start_pulse_width,
                            if BOOT_TITLE_START_DONE
                                .load(std::sync::atomic::Ordering::Relaxed)
                            {
                                1
                            } else {
                                0
                            }
                        ));
                    }
                }
            }

            // Optional compatibility shim for unusual RetroArch profiles
            // where the user's confirm button is reported as libretro
            // "Select" (Xbox BACK). Keep it opt-in so normal Xbox 360
            // Start/Back/A mapping remains transparent during manual play.
            let back_as_confirm = std::env::var_os("RUSTEMU_INPUT_BACK_AS_CONFIRM").is_some();
            let select_as_confirm = back_as_confirm && select_down && !rd(JP_START) && a_a == 0;
            if select_as_confirm {
                buttons |= 0x0010; // START
                a_a = 0xFF; // A

                static SELECT_CONFIRM_ALIAS_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let alias_n =
                    SELECT_CONFIRM_ALIAS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if alias_n < 8 || alias_n.is_power_of_two() {
                    debug_log(&format!(
                        "[HLE-INPUT-CONFIRM-ALIAS] #{} RetroPad BACK -> BACK|START plus A pressure",
                        alias_n
                    ));
                }
            }
            static SELECT_RELEASE_CARRY: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let carry_select_release = if back_as_confirm {
                if select_down {
                    SELECT_RELEASE_CARRY.store(1, std::sync::atomic::Ordering::Relaxed);
                    false
                } else {
                    SELECT_RELEASE_CARRY.swap(0, std::sync::atomic::Ordering::Relaxed) != 0
                }
            } else {
                SELECT_RELEASE_CARRY.store(0, std::sync::atomic::Ordering::Relaxed);
                false
            };
            if carry_select_release {
                buttons |= 0x0020; // BACK
                if !start_down && a_a == 0 {
                    buttons |= 0x0010; // START
                    a_a = 0xFF; // A
                }

                static SELECT_RELEASE_CARRY_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let carry_n =
                    SELECT_RELEASE_CARRY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if carry_n < 8 || carry_n.is_power_of_two() {
                    debug_log(&format!(
                        "[HLE-INPUT-CONFIRM-CARRY] #{} RetroPad BACK release -> one extra BACK|START plus A pressure",
                        carry_n
                    ));
                }
            }

            // Origin_Z's loading/tutorial script asks SELECT_PRESSED for
            // action 0x15, which the native input object maps to Xbox BACK.
            // Keep that alias out of the frontend by default: MENU.xbs has
            // its own controller table, and forcing the 0x15/0x1A path there
            // leaves the menu stuck with no listener for slot 0x1A.
            let (spidey_confirm_alias_scene, spidey_scene_name, spidey_stash_name) = {
                fn read_scene_string(mem: *mut u8, addr: u32, max_len: usize) -> String {
                    let mut bytes = Vec::with_capacity(max_len.min(64));
                    for i in 0..max_len {
                        let b = unsafe { *((mem as u64 + addr as u64 + i as u64) as *const u8) };
                        if b == 0 {
                            break;
                        }
                        if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                            bytes.push(b);
                        } else {
                            break;
                        }
                    }
                    String::from_utf8_lossy(&bytes).into_owned()
                }

                let scene_name = read_scene_string(guest_mem, 0x004B_C848, 64);
                let stash_name = read_scene_string(guest_mem, 0x004B_C948, 64);
                let bonus_menu_alias_enabled = spidey_title_patches_enabled()
                    && std::env::var_os("RUSTEMU_SPIDEY_MENU_CONFIRM_ALIAS").is_some();
                let is_spidey_script_scene =
                    scene_name.to_ascii_lowercase().starts_with("levels\\")
                        || (bonus_menu_alias_enabled
                            && scene_name.eq_ignore_ascii_case("bonus\\menu")
                            && stash_name.eq_ignore_ascii_case("M0menu\\menu"));
                (is_spidey_script_scene, scene_name, stash_name)
            };
            let spidey_bonus_menu = spidey_scene_name.eq_ignore_ascii_case("bonus\\menu")
                && spidey_stash_name.eq_ignore_ascii_case("M0menu\\menu");
            let spidey_origin_z = spidey_scene_name.eq_ignore_ascii_case("levels\\origin_z");
            if std::env::var_os("RUSTEMU_SPIDEY_XBS_SCRIPT_PROBE").is_some() {
                static SPIDEY_FOCUS_CHAIN_PROBE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static SPIDEY_FOCUS_CHAIN_LAST_SCENE: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static SPIDEY_FOCUS_CHAIN_LAST_XROOT_ITEM: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static SPIDEY_FOCUS_CHAIN_LAST_ENGINE_ITEM: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let read_u32 = |addr: u32| -> u32 {
                    if (0x1000..0x2000_0000).contains(&addr) {
                        unsafe {
                            std::ptr::read_unaligned((guest_mem as u64 + addr as u64) as *const u32)
                        }
                    } else {
                        0
                    }
                };
                let read_focus_chain = |owner: u32| -> (u32, u32, u32, u32) {
                    let mgr = read_u32(owner.wrapping_add(0x1A8));
                    let head = read_u32(mgr.wrapping_add(0x10));
                    let first = read_u32(head);
                    let item = read_u32(first.wrapping_add(0x08));
                    (mgr, head, first, item)
                };
                let xroot = read_u32(0x004C_06B8);
                let engine_scene = read_u32(0x004B_C614);
                let frame = read_u32(0x004B_C630);
                let frame_scene = read_u32(frame.wrapping_add(0x18));
                let global_scene = read_u32(0x003F_5BEC);
                let (xroot_mgr, xroot_head, xroot_first, xroot_item) = read_focus_chain(xroot);
                let (engine_mgr, engine_head, engine_first, engine_item) =
                    read_focus_chain(engine_scene);
                let (frame_mgr, frame_head, frame_first, frame_item) =
                    read_focus_chain(frame_scene);
                let (global_mgr, global_head, global_first, global_item) =
                    read_focus_chain(global_scene);
                let scene_changed = SPIDEY_FOCUS_CHAIN_LAST_SCENE
                    .swap(engine_scene, Ordering::Relaxed)
                    != engine_scene;
                let xroot_item_changed = SPIDEY_FOCUS_CHAIN_LAST_XROOT_ITEM
                    .swap(xroot_item, Ordering::Relaxed)
                    != xroot_item;
                let engine_item_changed = SPIDEY_FOCUS_CHAIN_LAST_ENGINE_ITEM
                    .swap(engine_item, Ordering::Relaxed)
                    != engine_item;
                let focus_log_n = SPIDEY_FOCUS_CHAIN_PROBE_LOG.fetch_add(1, Ordering::Relaxed);
                if scene_changed
                    || xroot_item_changed
                    || engine_item_changed
                    || focus_log_n < 24
                    || focus_log_n.is_power_of_two()
                    || (spidey_origin_z && focus_log_n % 64 == 0)
                {
                    debug_log(&format!(
                        "[SPIDEY-FOCUS-CHAIN-PROBE] #{} poll={} scene='{}' stash='{}' xroot=0x{:08X} engine=0x{:08X} frame=0x{:08X} frame_scene=0x{:08X} global=0x{:08X} xroot_chain=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} engine_chain=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} frame_chain=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} global_chain=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}",
                        focus_log_n,
                        n,
                        spidey_scene_name,
                        spidey_stash_name,
                        xroot,
                        engine_scene,
                        frame,
                        frame_scene,
                        global_scene,
                        xroot_mgr,
                        xroot_head,
                        xroot_first,
                        xroot_item,
                        engine_mgr,
                        engine_head,
                        engine_first,
                        engine_item,
                        frame_mgr,
                        frame_head,
                        frame_first,
                        frame_item,
                        global_mgr,
                        global_head,
                        global_first,
                        global_item,
                    ));
                }
            }

            // Narrow secondary-menu probe: after the title START pulse has
            // parked us on the visible bonus\menu selector, emit exactly one
            // short confirm edge. The older A-only mode is preserved for
            // comparison, but the selector's live compare hook watches the
            // digital action-0x15/BACK edge.
            let selector_a_once_enabled = spidey_title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_SELECTOR_A_ONCE").is_some();
            let selector_confirm_once_enabled = spidey_title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_SELECTOR_CONFIRM_ONCE").is_some();
            if (selector_a_once_enabled || selector_confirm_once_enabled) && spidey_bonus_menu {
                static SELECTOR_A_POLLS: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static SELECTOR_A_DONE: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                static SELECTOR_A_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);

                let poll = SELECTOR_A_POLLS.fetch_add(1, Ordering::Relaxed);
                let delay = std::env::var("RUSTEMU_SPIDEY_SELECTOR_A_DELAY")
                    .ok()
                    .and_then(|raw| raw.trim().parse::<u32>().ok())
                    .filter(|v| *v <= 4096)
                    .unwrap_or(180);
                let width = std::env::var("RUSTEMU_SPIDEY_SELECTOR_A_WIDTH")
                    .ok()
                    .and_then(|raw| raw.trim().parse::<u32>().ok())
                    .filter(|v| (1..=30).contains(v))
                    .unwrap_or(3);
                let done = SELECTOR_A_DONE.load(Ordering::Relaxed);
                let pulse = !done && poll >= delay && poll < delay.saturating_add(width);
                if pulse {
                    buttons &= !0x0030; // keep the probe edge clean
                    a_a = 0xFF;
                    if selector_confirm_once_enabled {
                        buttons |= 0x0020; // Spider-Man menu SELECT/action 0x15
                    }
                    note_spidey_manual_confirm_edge();
                }
                if !done && poll >= delay.saturating_add(width) {
                    SELECTOR_A_DONE.store(true, Ordering::Relaxed);
                }

                let log_n = SELECTOR_A_LOG.fetch_add(1, Ordering::Relaxed);
                if pulse || log_n < 16 || log_n.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-SELECTOR-CONFIRM-ONCE] #{} poll={} delay={} width={} pulse={} done={} mode={} scene='{}' stash='{}' START={} BACK={} A={}",
                        log_n,
                        poll,
                        delay,
                        width,
                        if pulse { 1 } else { 0 },
                        if SELECTOR_A_DONE.load(Ordering::Relaxed) { 1 } else { 0 },
                        if selector_confirm_once_enabled { "back+a" } else { "a" },
                        spidey_scene_name,
                        spidey_stash_name,
                        if (buttons & 0x0010) != 0 { 1 } else { 0 },
                        if (buttons & 0x0020) != 0 { 1 } else { 0 },
                        a_a
                    ));
                }
            }

            // Focused menu repro: emit one clean confirm pulse after MENU.xbs
            // is active, then go hands-off. The older raw 360-map injector
            // cycles START/BACK/A forever, which can skip the secondary menu
            // we need to inspect.
            if spidey_title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_SINGLE_MENU_CONFIRM").is_some()
                && spidey_bonus_menu
            {
                static SINGLE_MENU_CONFIRM_POLLS: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static SINGLE_MENU_CONFIRM_DONE: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                static SINGLE_MENU_CONFIRM_STARTED: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                static SINGLE_MENU_WAIT_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let read_u32 = |addr: u32| -> u32 {
                    if (0x1000..0x2000_0000).contains(&addr) {
                        unsafe {
                            std::ptr::read_unaligned((guest_mem as u64 + addr as u64) as *const u32)
                        }
                    } else {
                        0
                    }
                };
                let read_u8 = |addr: u32| -> u8 {
                    if (0x1000..0x2000_0000).contains(&addr) {
                        unsafe { *((guest_mem as u64 + addr as u64) as *const u8) }
                    } else {
                        0
                    }
                };
                let engine_scene = read_u32(0x004B_C614);
                let frame = read_u32(0x004B_C630);
                let frame_scene = read_u32(frame.wrapping_add(0x18));
                let engine10 = read_u8(engine_scene.wrapping_add(0x10));
                let engine11 = read_u8(engine_scene.wrapping_add(0x11));
                let engine18c = read_u8(engine_scene.wrapping_add(0x18C));
                let origin1_active = ORIGIN1_BINK_ACTIVE.load(Ordering::Relaxed) != 0;
                let origin1_frame = ORIGIN1_BINK_FRAME.load(Ordering::Relaxed);
                let origin1_frames = ORIGIN1_BINK_FRAMES.load(Ordering::Relaxed);
                let scene_ready_now = (0x1000..0x2000_0000).contains(&engine_scene)
                    && engine10 != 0
                    && engine11 != 0
                    && !origin1_active;
                let poll = SINGLE_MENU_CONFIRM_POLLS.load(std::sync::atomic::Ordering::Relaxed);
                let already_done =
                    SINGLE_MENU_CONFIRM_DONE.load(std::sync::atomic::Ordering::Relaxed);
                let pulse_started =
                    SINGLE_MENU_CONFIRM_STARTED.load(std::sync::atomic::Ordering::Relaxed);
                let scene_ready = scene_ready_now || pulse_started;
                let pulse_end = std::env::var("RUSTEMU_SPIDEY_SINGLE_MENU_CONFIRM_POLLS")
                    .ok()
                    .and_then(|raw| raw.trim().parse::<u32>().ok())
                    .filter(|v| *v >= 4 && *v <= 4096)
                    .unwrap_or(48);
                let pulse_start = 24u32;
                let pulse_limit = pulse_start.saturating_add(pulse_end);
                let pulse_active =
                    scene_ready && !already_done && (pulse_start..pulse_limit).contains(&poll);
                let menu_back_alias =
                    std::env::var_os("RUSTEMU_SPIDEY_MENU_CONFIRM_ALIAS").is_some();
                let menu_confirm_with_a =
                    std::env::var_os("RUSTEMU_SPIDEY_SINGLE_MENU_CONFIRM_WITH_A").is_some();
                if pulse_active {
                    SINGLE_MENU_CONFIRM_STARTED.store(true, std::sync::atomic::Ordering::Relaxed);
                    let next_poll = SINGLE_MENU_CONFIRM_POLLS
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                        .saturating_add(1);
                    buttons |= 0x0010; // START
                    if menu_confirm_with_a {
                        a_a = 0xFF;
                    }
                    if menu_back_alias && (poll & 1) == 0 {
                        buttons |= 0x0020; // BACK drives Spider-Man action 0x15
                        note_spidey_manual_confirm_edge();
                    }
                    if next_poll >= pulse_limit {
                        SINGLE_MENU_CONFIRM_DONE.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                } else if scene_ready && !already_done && !pulse_started && poll < 24 {
                    SINGLE_MENU_CONFIRM_POLLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else if !already_done && pulse_started && poll < pulse_limit {
                    let next_poll = SINGLE_MENU_CONFIRM_POLLS
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                        .saturating_add(1);
                    if next_poll >= pulse_limit {
                        SINGLE_MENU_CONFIRM_DONE.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                } else if !already_done && poll >= pulse_limit {
                    SINGLE_MENU_CONFIRM_DONE.store(true, std::sync::atomic::Ordering::Relaxed);
                }

                static SINGLE_MENU_CONFIRM_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let log_n =
                    SINGLE_MENU_CONFIRM_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let wait_n =
                    SINGLE_MENU_WAIT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if pulse_active || log_n < 16 || log_n.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-SINGLE-MENU-CONFIRM] #{} poll={} scene='{}' stash='{}' ready={} ready_now={} engine=0x{:08X} e10/11={}/{} e18c={} frame_scene=0x{:08X} pulse={} done={} pulse_end={} START={} BACK={} A={} with_a={}",
                        log_n,
                        poll,
                        spidey_scene_name,
                        spidey_stash_name,
                        if scene_ready { 1 } else { 0 },
                        if scene_ready_now { 1 } else { 0 },
                        engine_scene,
                        engine10,
                        engine11,
                        engine18c,
                        frame_scene,
                        if pulse_active { 1 } else { 0 },
                        if SINGLE_MENU_CONFIRM_DONE.load(std::sync::atomic::Ordering::Relaxed) { 1 } else { 0 },
                        pulse_end,
                        if pulse_active { 1 } else { 0 },
                        if pulse_active && menu_back_alias && (poll & 1) == 0 { 1 } else { 0 },
                        if pulse_active && menu_confirm_with_a { 255 } else { 0 },
                        if menu_confirm_with_a { 1 } else { 0 }
                    ));
                } else if origin1_active && (wait_n < 8 || wait_n.is_power_of_two()) {
                    debug_log(&format!(
                        "[SPIDEY-SINGLE-MENU-WAIT-BINK] #{} poll={} origin1={}/{} scene='{}' stash='{}'",
                        wait_n, poll, origin1_frame, origin1_frames, spidey_scene_name, spidey_stash_name
                    ));
                } else if !scene_ready && (wait_n < 8 || wait_n.is_power_of_two()) {
                    debug_log(&format!(
                        "[SPIDEY-SINGLE-MENU-WAIT] #{} scene='{}' stash='{}' engine=0x{:08X} e10/11={}/{} e18c={} frame_scene=0x{:08X} ready=0",
                        wait_n, spidey_scene_name, spidey_stash_name, engine_scene, engine10, engine11, engine18c, frame_scene
                    ));
                }
            }

            let spidey_manual_script_enabled = spidey_title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_DISABLE_MENU_ASSIST").is_none();
            // Manual play should not run the verifier-style menu driver by
            // default. That path can consume one real START press during the
            // early bonus\menu transition and skip past the visible selector.
            // Keep it opt-in for diagnostics; the lighter START/A -> BACK
            // alias below still fixes Spider-Man's confirm button shape.
            let spidey_manual_sequence_enabled = spidey_manual_script_enabled
                && std::env::var_os("RUSTEMU_SPIDEY_MANUAL_SEQUENCE").is_some();
            let spidey_manual_script_trigger_scene = spidey_bonus_menu || spidey_origin_z;
            let manual_confirm_down = spidey_manual_script_enabled
                && (spidey_confirm_alias_scene
                    || (spidey_manual_sequence_enabled && spidey_manual_script_trigger_scene))
                && (start_down || select_down || a_a != 0);
            SPIDEY_MANUAL_CONFIRM_HELD.store(
                if manual_confirm_down { 1 } else { 0 },
                std::sync::atomic::Ordering::Relaxed,
            );
            let prev_manual_confirm = SPIDEY_MANUAL_SCRIPT_PREV_CONFIRM.swap(
                if manual_confirm_down { 1 } else { 0 },
                std::sync::atomic::Ordering::Relaxed,
            );
            let manual_script_was_active =
                SPIDEY_MANUAL_SCRIPT_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) != 0;
            if spidey_manual_sequence_enabled
                && manual_confirm_down
                && prev_manual_confirm == 0
                && !manual_script_was_active
            {
                SPIDEY_MANUAL_SCRIPT_ACTIVE.store(1, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_START_HOLD_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_POST_WAIT_A_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_VISIBLE_MENU_CONFIRM_STARTED
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_MENU_READY_FOR_CONFIRM.store(false, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_LEVEL_CONFIRM_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_LEVEL_CONFIRM_STARTED.store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_LEVEL_CONFIRM_LAST_STATE
                    .store(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_LEVEL_CONFIRM_ORIGIN_READY_POLLS
                    .store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_ORIGINZ_LAST_VALID_STATE
                    .store(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_ORIGINZ_SUSTAIN_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);

                let arm_n =
                    SPIDEY_MANUAL_SCRIPT_ARM_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if arm_n < 8 || arm_n.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-MANUAL-SCRIPT] armed #{} by START/BACK/A scene='{}' stash='{}'",
                        arm_n, spidey_scene_name, spidey_stash_name
                    ));
                }
            }
            let mut spidey_manual_script_active = spidey_manual_sequence_enabled
                && SPIDEY_MANUAL_SCRIPT_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) != 0;
            if spidey_manual_script_active
                && spidey_scene_name
                    .to_ascii_lowercase()
                    .starts_with("levels\\")
                && !spidey_origin_z
            {
                SPIDEY_MANUAL_SCRIPT_ACTIVE.store(0, std::sync::atomic::Ordering::Relaxed);
                SPIDEY_MANUAL_CONFIRM_HELD.store(0, std::sync::atomic::Ordering::Relaxed);
                spidey_manual_script_active = false;
                debug_log(&format!(
                    "[SPIDEY-MANUAL-SCRIPT] stopped after scene advanced to '{}'",
                    spidey_scene_name
                ));
            }
            // Organic START/A holds must still feed Spider-Man's script-side
            // SELECT_PRESSED action (0x15), which is wired to Xbox BACK. The
            // verifier path emits clean BACK edges for as long as confirm is
            // held; keep the same behavior for real controller input in the
            // menu/loading script scenes unless the assist is explicitly
            // disabled.
            let spidey_menu_assist = spidey_manual_script_enabled;
            let spidey_start_a_confirm =
                spidey_menu_assist && spidey_confirm_alias_scene && (start_down || a_a != 0);
            static START_A_CONFIRM_CARRY_POLLS: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            static START_A_CONFIRM_PULSE_PHASE: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let spidey_start_a_carry = if spidey_start_a_confirm {
                START_A_CONFIRM_CARRY_POLLS.store(4, std::sync::atomic::Ordering::Relaxed);
                false
            } else {
                let carry = START_A_CONFIRM_CARRY_POLLS.load(std::sync::atomic::Ordering::Relaxed);
                if carry > 0 {
                    START_A_CONFIRM_CARRY_POLLS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                    true
                } else {
                    false
                }
            };
            if spidey_confirm_alias_scene && !spidey_start_a_confirm && !spidey_start_a_carry {
                START_A_CONFIRM_PULSE_PHASE.store(0, std::sync::atomic::Ordering::Relaxed);
            }
            if spidey_start_a_confirm || spidey_start_a_carry {
                let phase =
                    START_A_CONFIRM_PULSE_PHASE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let emit_back_edge = (phase & 1) == 0;
                // In this Spider-Man script path BACK is the game's action
                // 0x15. If the physical confirm is mapped as RetroPad BACK,
                // the raw mapping above already set BACK, which prevents the
                // off half of this pulse train from ever becoming a release.
                // Own the bit here so manual input produces the same clean
                // 0->1 edges as the scripted probe.
                buttons &= !0x0020;
                if emit_back_edge {
                    buttons |= 0x0020; // BACK (Spider-Man action 0x15)
                    note_spidey_manual_confirm_edge();
                }

                static START_A_CONFIRM_ALIAS_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let alias_n =
                    START_A_CONFIRM_ALIAS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if alias_n < 12 || alias_n.is_power_of_two() {
                    debug_log(&format!(
                        "[HLE-INPUT-CONFIRM-ALIAS] #{} START/A -> pulsed BACK for Spider-Man action 0x15 phase={} back={} carry={}",
                        alias_n,
                        phase,
                        if emit_back_edge { 1 } else { 0 },
                        if spidey_start_a_carry { 1 } else { 0 }
                    ));
                }
            }

            if spidey_manual_script_active {
                // The scripted path needs release windows. If the user keeps
                // holding START/A, mask only those confirm controls and let the
                // verifier-style sequence below drive clean edges.
                buttons &= !(0x0010 | 0x0020);
                a_a = 0;
            }

            // Scripted Spider-Man verification needs to be keyed to guest
            // input polls, not frontend frames. On slow debug runs RetroArch
            // may not reach the frame-window pulse before Spider-Man is
            // already polling XInput at the title screen.
            let spidey_test_autopress_start =
                std::env::var_os("RUSTEMU_TEST_AUTOPRESS_START").is_some();
            let spidey_script_start_active =
                spidey_test_autopress_start || spidey_manual_script_active || spidey_auto_new_game;
            // Spider-Man's menu action 0x15 is wired to Xbox BACK even though
            // the user-facing prompt is START. The scripted START verifier
            // therefore also needs to produce BACK edges.
            let spidey_script_confirm_active = std::env::var_os("RUSTEMU_TEST_AUTOPRESS_CONFIRM")
                .is_some()
                || spidey_test_autopress_start
                || spidey_manual_script_active
                || spidey_auto_new_game;
            if spidey_script_start_active {
                fn read_guest_c_string(mem: *mut u8, addr: u32, max_len: usize) -> String {
                    let mut bytes = Vec::with_capacity(max_len.min(64));
                    for i in 0..max_len {
                        let b = unsafe { *((mem as u64 + addr as u64 + i as u64) as *const u8) };
                        if b == 0 {
                            break;
                        }
                        if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                            bytes.push(b);
                        } else {
                            break;
                        }
                    }
                    String::from_utf8_lossy(&bytes).into_owned()
                }

                let scene_name = read_guest_c_string(guest_mem, 0x004B_C848, 64);
                let stash_name = read_guest_c_string(guest_mem, 0x004B_C948, 64);
                if scene_name.eq_ignore_ascii_case("bonus\\menu")
                    && stash_name.eq_ignore_ascii_case("M0menu\\menu")
                {
                    let read_u32 = |addr: u32| -> u32 {
                        if (0x1000..0x2000_0000).contains(&addr) {
                            unsafe {
                                std::ptr::read_unaligned(
                                    (guest_mem as u64 + addr as u64) as *const u32,
                                )
                            }
                        } else {
                            0
                        }
                    };
                    let global_scene = read_u32(0x003F_5BEC);
                    let engine_scene = read_u32(0x004B_C614);
                    let frame = read_u32(0x004B_C630);
                    let frame_scene = read_u32(frame.wrapping_add(0x18));
                    let scene_has_state = |scene: u32| -> bool {
                        if !(0x1000..0x2000_0000).contains(&scene) {
                            return false;
                        }
                        let state_top = read_u32(scene.wrapping_add(0xA0));
                        if !(0x1000..0x2000_0000).contains(&state_top) || state_top < 0x14 {
                            return false;
                        }
                        let state_arr = read_u32(state_top.wrapping_sub(0x14));
                        (0x1000..0x2000_0000).contains(&state_arr)
                    };
                    let scene = if scene_has_state(frame_scene) {
                        frame_scene
                    } else if scene_has_state(engine_scene) {
                        engine_scene
                    } else {
                        global_scene
                    };
                    let state_top = read_u32(scene.wrapping_add(0xA0));
                    let state_arr = read_u32(state_top.wrapping_sub(0x14));
                    let state_idx = read_u32(state_top.wrapping_sub(0x10));
                    let state_val = if state_idx < 0x300 {
                        read_u32(state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let scene_18c_ready = (read_u32(scene.wrapping_add(0x18C)) & 0xFF) != 0;
                    let scene_186_ready = (read_u32(scene.wrapping_add(0x186)) & 0xFF) != 0;
                    let bonus_menu_ready_for_confirm = scene_18c_ready || state_val == 4;
                    if bonus_menu_ready_for_confirm {
                        SPIDEY_MENU_READY_FOR_CONFIRM
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    let menu_ready_for_confirm =
                        SPIDEY_MENU_READY_FOR_CONFIRM.load(std::sync::atomic::Ordering::Relaxed);
                    let selector_seen_for_confirm =
                        SPIDEY_MENU_SELECTOR_SEEN.load(Ordering::Relaxed);
                    let state3_menu_ready = state_val == 3
                        && (menu_ready_for_confirm
                            || (scene_186_ready && selector_seen_for_confirm));
                    let hold_n =
                        SPIDEY_START_HOLD_POLLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let allow_state3_autopress =
                        std::env::var_os("RUSTEMU_SPIDEY_STATE3_AUTOPRESS").is_some();
                    let auto_start_interval =
                        std::env::var("RUSTEMU_SPIDEY_AUTONEWGAME_START_INTERVAL")
                            .ok()
                            .and_then(|raw| raw.trim().parse::<u32>().ok())
                            .filter(|v| (4..=240).contains(v))
                            .unwrap_or(24);
                    let auto_start_width = std::env::var("RUSTEMU_SPIDEY_AUTONEWGAME_START_WIDTH")
                        .ok()
                        .and_then(|raw| raw.trim().parse::<u32>().ok())
                        .filter(|v| (1..=60).contains(v))
                        .unwrap_or(4)
                        .min(auto_start_interval);
                    let scene_lc = scene_name.to_ascii_lowercase();
                    let stash_lc = stash_name.to_ascii_lowercase();
                    let auto_new_game_title_or_menu = !scene_lc.starts_with("levels\\")
                        && (scene_lc.is_empty()
                            || scene_lc == "bonus\\menu"
                            || stash_lc.is_empty()
                            || stash_lc == "m0menu\\menu");
                    let auto_start_all_pressed = spidey_auto_new_game
                        && auto_new_game_title_or_menu
                        && spidey_script_confirm_active
                        && (hold_n % auto_start_interval) < auto_start_width;
                    let state3_new_game_script = allow_state3_autopress;
                    let title_state3_down_to_new_game = state3_new_game_script
                        && spidey_script_confirm_active
                        && state3_menu_ready
                        && (12..20).contains(&hold_n);
                    let title_state3_a_pressed = state3_new_game_script
                        && spidey_script_confirm_active
                        && state3_menu_ready
                        && (28..44).contains(&hold_n);
                    let title_state3_confirm_edge = state3_new_game_script
                        && !spidey_auto_new_game
                        && spidey_script_confirm_active
                        && state3_menu_ready
                        && (28..44).contains(&hold_n)
                        && (hold_n & 1) == 1;
                    let allow_early_autopress =
                        std::env::var_os("RUSTEMU_SPIDEY_EARLY_AUTOPRESS").is_some();
                    let title_start_pressed = (allow_early_autopress
                        && state_val != 4
                        && state_val != 3
                        && hold_n < 720
                        && (hold_n % 24) < 3)
                        || title_state3_confirm_edge;
                    let title_select_edge = spidey_script_confirm_active
                        && ((allow_early_autopress
                            && state_val != 4
                            && state_val != 3
                            && hold_n == 25)
                            || title_state3_confirm_edge);
                    if state_val == 3 && hold_n < 72 {
                        debug_log(&format!(
                            "[SPIDEY-TEST-INPUT-POLL] #{} poll={} state_val={} 186={} 18c={} selector={} ready={} DOWN={} START={} BACK={} A={} buttons_pre=0x{:04X}",
                            hold_n,
                            n,
                            state_val,
                            if scene_186_ready { 1 } else { 0 },
                            if scene_18c_ready { 1 } else { 0 },
                            if selector_seen_for_confirm { 1 } else { 0 },
                            if menu_ready_for_confirm { 1 } else { 0 },
                            if title_state3_down_to_new_game { 1 } else { 0 },
                            if title_start_pressed || auto_start_all_pressed { 1 } else { 0 },
                            if title_select_edge { 1 } else { 0 },
                            if title_state3_a_pressed { 255 } else { 0 },
                            buttons
                        ));
                    }
                    if title_state3_down_to_new_game
                        || title_state3_a_pressed
                        || title_start_pressed
                        || auto_start_all_pressed
                        || title_select_edge
                    {
                        if title_start_pressed || auto_start_all_pressed {
                            buttons |= 0x0010; // START
                        }
                        if title_state3_down_to_new_game {
                            buttons |= 0x0002; // DPAD_DOWN to New Game
                            ly = -32768;
                        }
                        if title_select_edge {
                            buttons |= 0x0020; // BACK (Spider-Man action 0x15)
                            note_spidey_manual_confirm_edge();
                        }
                        if title_state3_a_pressed {
                            a_a = 0xFF; // A
                        }
                        if hold_n < 32 || hold_n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-TEST-START-PULSE] #{} poll={} scene='{}' stash='{}' state_val={} DOWN={} START={} BACK={} A={}",
                                hold_n,
                                n,
                                scene_name,
                                stash_name,
                                state_val,
                                if title_state3_down_to_new_game { 1 } else { 0 },
                                if title_start_pressed || auto_start_all_pressed { 1 } else { 0 },
                                if title_select_edge { 1 } else { 0 },
                                a_a
                            ));
                        }
                    } else if spidey_script_confirm_active && !spidey_auto_new_game {
                        let input_sequence_started =
                            SPIDEY_POST_WAIT_A_POLLS.load(std::sync::atomic::Ordering::Relaxed) > 0;
                        let origin1_active = ORIGIN1_BINK_ACTIVE.load(Ordering::Relaxed) != 0;
                        let origin1_frame = ORIGIN1_BINK_FRAME.load(Ordering::Relaxed);
                        let origin1_frames = ORIGIN1_BINK_FRAMES.load(Ordering::Relaxed);
                        let origin1_done = !origin1_active
                            && (origin1_frames == 0
                                || origin1_frame >= origin1_frames.saturating_sub(1));
                        let visible_menu_seen =
                            state_val == 1 && SPIDEY_MENU_SELECTOR_SEEN.load(Ordering::Relaxed);
                        let wait_for_visible_bink =
                            std::env::var_os("RUSTEMU_SPIDEY_WAIT_VISIBLE_MENU_BINK").is_some();
                        let visible_menu_ready = visible_menu_seen
                            && menu_ready_for_confirm
                            && (!wait_for_visible_bink || origin1_done);
                        if visible_menu_seen && wait_for_visible_bink && !origin1_done {
                            static VISIBLE_MENU_WAIT_BINK_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let wait_log =
                                VISIBLE_MENU_WAIT_BINK_LOG.fetch_add(1, Ordering::Relaxed);
                            if wait_log < 12 || wait_log.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-MENU-CONFIRM-WAIT-BINK] #{} poll={} state_val={} origin1_active={} frame={}/{}",
                                    wait_log,
                                    n,
                                    state_val,
                                    if origin1_active { 1 } else { 0 },
                                    origin1_frame,
                                    origin1_frames
                                ));
                            }
                        }
                        let visible_menu_sequence = if visible_menu_ready {
                            SPIDEY_VISIBLE_MENU_CONFIRM_STARTED
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                            true
                        } else {
                            SPIDEY_VISIBLE_MENU_CONFIRM_STARTED
                                .load(std::sync::atomic::Ordering::Relaxed)
                        };
                        let suppress_visible_loading_bleed = wait_for_visible_bink
                            && visible_menu_sequence
                            && !visible_menu_ready
                            && state_val != 4;
                        let ready_for_confirm = state3_menu_ready
                            || state_val == 4
                            || visible_menu_ready
                            || (input_sequence_started && !suppress_visible_loading_bleed);
                        let confirm_n = if ready_for_confirm {
                            SPIDEY_POST_WAIT_A_POLLS
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                        } else {
                            0xFFFF_FFFF
                        };
                        let new_game_submenu = state_val == 4;
                        let auto_new_game_down_window = (2..10).contains(&confirm_n)
                            || (48..56).contains(&confirm_n)
                            || (96..104).contains(&confirm_n);
                        let auto_new_game_confirm_window = (20..32).contains(&confirm_n)
                            || (72..84).contains(&confirm_n)
                            || (128..140).contains(&confirm_n);
                        let down_to_new_game = spidey_auto_new_game
                            && ready_for_confirm
                            && !new_game_submenu
                            && auto_new_game_down_window;
                        let confirm_pressed = if new_game_submenu {
                            if spidey_auto_new_game {
                                auto_new_game_confirm_window
                            } else {
                                (8..16).contains(&confirm_n)
                            }
                        } else if visible_menu_sequence {
                            if spidey_auto_new_game {
                                auto_new_game_confirm_window
                            } else {
                                let visible_confirm_polls =
                                    std::env::var("RUSTEMU_SPIDEY_VISIBLE_MENU_CONFIRM_POLLS")
                                        .ok()
                                        .and_then(|raw| raw.trim().parse::<u32>().ok())
                                        .unwrap_or(1024);
                                confirm_n < visible_confirm_polls
                            }
                        } else {
                            if spidey_auto_new_game {
                                auto_new_game_confirm_window
                            } else {
                                confirm_n < 8
                            }
                        };
                        let late_start_pressed = if visible_menu_sequence && !new_game_submenu {
                            false
                        } else if spidey_auto_new_game {
                            false
                        } else {
                            let late_start_start = if new_game_submenu { 16 } else { 8 };
                            (late_start_start..(late_start_start + 72)).contains(&confirm_n)
                                || (128..208).contains(&confirm_n)
                        };
                        let confirm_start_pressed =
                            late_start_pressed || (confirm_pressed && !spidey_auto_new_game);
                        if confirm_start_pressed {
                            buttons |= 0x0010; // START
                        }
                        if down_to_new_game {
                            buttons |= 0x0002; // DPAD_DOWN to New Game
                            ly = -32768;
                        }
                        let visible_menu_back_enabled =
                            std::env::var_os("RUSTEMU_SPIDEY_VISIBLE_MENU_NO_BACK").is_none();
                        let confirm_select_edge = (state_val == 3
                            || new_game_submenu
                            || (visible_menu_sequence && visible_menu_back_enabled))
                            && !spidey_auto_new_game
                            && (confirm_pressed || late_start_pressed)
                            && (n & 1) == 0;
                        if confirm_select_edge {
                            buttons |= 0x0020; // BACK (Spider-Man action 0x15)
                            note_spidey_manual_confirm_edge();
                        }
                        if confirm_pressed {
                            a_a = 0xFF; // A
                        }
                        if confirm_n < 12 || confirm_n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-TEST-A-HOLD] #{} poll={} scene='{}' stash='{}' scene_obj=0x{:08X} frame_scene=0x{:08X} engine=0x{:08X} global=0x{:08X} state_idx={} state_val={} 186={} 18c={} ready={} DOWN={} START={} BACK={} A={}",
                                confirm_n,
                                n,
                                scene_name,
                                stash_name,
                                scene,
                                frame_scene,
                                engine_scene,
                                global_scene,
                                state_idx,
                                state_val,
                                if scene_186_ready { 1 } else { 0 },
                                if scene_18c_ready { 1 } else { 0 },
                                if menu_ready_for_confirm { 1 } else { 0 },
                                if down_to_new_game { 1 } else { 0 },
                                if confirm_start_pressed { 1 } else { 0 },
                                if confirm_select_edge { 1 } else { 0 },
                                if confirm_pressed { 255 } else { 0 }
                            ));
                        }
                        if late_start_pressed && (confirm_n < 56 || confirm_n % 16 == 0) {
                            debug_log(&format!(
                                "[SPIDEY-TEST-LATE-START] #{} poll={} scene='{}' stash='{}' scene_obj=0x{:08X} frame_scene=0x{:08X} engine=0x{:08X} global=0x{:08X} state_idx={} state_val={} START=1",
                                confirm_n,
                                n,
                                scene_name,
                                stash_name,
                                scene,
                                frame_scene,
                                engine_scene,
                                global_scene,
                                state_idx,
                                state_val
                            ));
                        }
                    }
                } else if spidey_script_confirm_active
                    && spidey_auto_level_confirm
                    && scene_name.to_ascii_lowercase().starts_with("levels\\")
                {
                    let read_u32 = |addr: u32| -> u32 {
                        if (0x1000..0x2000_0000).contains(&addr) {
                            unsafe {
                                std::ptr::read_unaligned(
                                    (guest_mem as u64 + addr as u64) as *const u32,
                                )
                            }
                        } else {
                            0
                        }
                    };
                    let read_u8 = |addr: u32| -> u8 {
                        if (0x1000..0x2000_0000).contains(&addr) {
                            unsafe {
                                std::ptr::read_unaligned(
                                    (guest_mem as u64 + addr as u64) as *const u8,
                                )
                            }
                        } else {
                            0
                        }
                    };
                    let write_u8 = |addr: u32, value: u8| {
                        if (0x1000..0x2000_0000).contains(&addr) {
                            unsafe {
                                std::ptr::write_unaligned(
                                    (guest_mem as u64 + addr as u64) as *mut u8,
                                    value,
                                );
                            }
                        }
                    };
                    let write_u32 = |addr: u32, value: u32| {
                        if (0x1000..0x2000_0000).contains(&addr) {
                            unsafe {
                                std::ptr::write_unaligned(
                                    (guest_mem as u64 + addr as u64) as *mut u32,
                                    value,
                                );
                            }
                        }
                    };
                    let engine_scene = read_u32(0x004B_C614);
                    let state_top = read_u32(engine_scene.wrapping_add(0xA0));
                    let state_idx = if state_top >= 0x14 {
                        read_u32(state_top.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_arr = if state_top >= 0x14 {
                        read_u32(state_top.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_slot = if state_idx < 0x300 {
                        state_arr.wrapping_add(state_idx.wrapping_mul(4))
                    } else {
                        0
                    };
                    let state_read_valid = state_idx < 0x300
                        && (0x1000..0x2000_0000).contains(&state_arr)
                        && state_slot != 0;
                    let mut state_val = if state_read_valid {
                        read_u32(state_slot)
                    } else {
                        0
                    };
                    let mut scene_b17f = read_u8(engine_scene.wrapping_add(0x17F));
                    let mut scene_b183 = read_u8(engine_scene.wrapping_add(0x183));
                    let mut scene_b186 = read_u8(engine_scene.wrapping_add(0x186));
                    let scene_b18b = read_u8(engine_scene.wrapping_add(0x18B));
                    let mut scene_b18c = read_u8(engine_scene.wrapping_add(0x18C));
                    let origin_z_scene = scene_name.eq_ignore_ascii_case("levels\\origin_z");
                    let peterstu_ready_seq =
                        crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                    if origin_z_scene && state_read_valid && matches!(state_val, 1 | 3 | 4 | 6) {
                        SPIDEY_ORIGINZ_LAST_VALID_STATE
                            .store(state_val, std::sync::atomic::Ordering::Relaxed);
                    } else if origin_z_scene
                        && peterstu_ready_seq != 0
                        && !state_read_valid
                        && scene_b183 != 0
                        && scene_b18c != 0
                    {
                        let fallback_state = SPIDEY_ORIGINZ_LAST_VALID_STATE
                            .load(std::sync::atomic::Ordering::Relaxed);
                        if matches!(fallback_state, 3 | 4 | 6) {
                            let old_state_val = state_val;
                            state_val = fallback_state;
                            let fallback_n = SPIDEY_ORIGINZ_STATE_FALLBACK_LOG
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if fallback_n < 16 || fallback_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGINZ-STATE-FALLBACK] #{} scene='{}' stash='{}' engine=0x{:08X} state_top=0x{:08X} idx={} arr=0x{:08X} slot=0x{:08X} old_state={} fallback_state={} flags17f/183/186/18b/18c={}/{}/{}/{}/{} peterstu_seq={}",
                                    fallback_n,
                                    scene_name,
                                    stash_name,
                                    engine_scene,
                                    state_top,
                                    state_idx,
                                    state_arr,
                                    state_slot,
                                    old_state_val,
                                    fallback_state,
                                    scene_b17f,
                                    scene_b183,
                                    scene_b186,
                                    scene_b18b,
                                    scene_b18c,
                                    peterstu_ready_seq
                                ));
                            }
                        }
                    }
                    let mut origin_z_state_poked = false;
                    let peterstu_ready_mode =
                        std::env::var("RUSTEMU_SPIDEY_PETERSTU_READY_POKE").unwrap_or_default();
                    let peterstu_force_state = matches!(
                        peterstu_ready_mode.trim().to_ascii_lowercase().as_str(),
                        "force" | "state" | "direct"
                    );
                    let unsafe_origin_z_signal_poke =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_SIGNAL_POKE")
                            .map(|v| {
                                matches!(
                                    v.trim().to_ascii_lowercase().as_str(),
                                    "unsafe" | "force-unsafe"
                                )
                            })
                            .unwrap_or(false);
                    if origin_z_scene
                        && state_val == 3
                        && state_slot != 0
                        && peterstu_ready_seq != 0
                        && !peterstu_ready_mode.is_empty()
                        && spidey_title_patches_enabled()
                        && unsafe_origin_z_signal_poke
                    {
                        let old_17f = scene_b17f;
                        let old_183 = scene_b183;
                        let old_186 = scene_b186;
                        let old_18c = scene_b18c;
                        write_u8(engine_scene.wrapping_add(0x17F), 0);
                        write_u8(engine_scene.wrapping_add(0x183), 1);
                        write_u8(engine_scene.wrapping_add(0x186), 1);
                        write_u8(engine_scene.wrapping_add(0x18C), 1);
                        scene_b17f = 0;
                        scene_b183 = 1;
                        scene_b186 = 1;
                        scene_b18c = 1;
                        if peterstu_force_state {
                            write_u32(state_slot, 4);
                            state_val = 4;
                        }
                        origin_z_state_poked = true;
                        let poke_n = SPIDEY_ORIGINZ_STATE_POKE_LOG.fetch_add(1, Ordering::Relaxed);
                        if poke_n < 8 || poke_n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-STATE-POKE] #{} mode='{}' peterstu_seq={} engine=0x{:08X} state_top=0x{:08X} idx={} arr=0x{:08X} slot=0x{:08X} state_write={} b17f:{}->0 b183:{}->1 b186:{}->1 b18c:{}->1",
                                poke_n,
                                peterstu_ready_mode.trim(),
                                peterstu_ready_seq,
                                engine_scene,
                                state_top,
                                state_idx,
                                state_arr,
                                state_slot,
                                if peterstu_force_state { "3->4" } else { "signal-only" },
                                old_17f,
                                old_183,
                                old_186,
                                old_18c
                            ));
                        }
                    }
                    let last_state = SPIDEY_LEVEL_CONFIRM_LAST_STATE
                        .swap(state_val, std::sync::atomic::Ordering::Relaxed);
                    if last_state != state_val {
                        SPIDEY_LEVEL_CONFIRM_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);
                        SPIDEY_LEVEL_CONFIRM_ORIGIN_READY_POLLS
                            .store(0, std::sync::atomic::Ordering::Relaxed);
                    }
                    SPIDEY_LEVEL_CONFIRM_STARTED.store(1, std::sync::atomic::Ordering::Relaxed);
                    let confirm_n = SPIDEY_LEVEL_CONFIRM_POLLS
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let phase = confirm_n % 192;
                    let origin_z_state3 = origin_z_scene && state_val == 3;
                    let origin_z_ready =
                        origin_z_state3 && engine_scene >= 0x1000 && scene_b17f == 0;
                    let origin_z_ready_poll = if origin_z_ready {
                        SPIDEY_LEVEL_CONFIRM_ORIGIN_READY_POLLS
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                    } else {
                        SPIDEY_LEVEL_CONFIRM_ORIGIN_READY_POLLS
                            .store(0, std::sync::atomic::Ordering::Relaxed);
                        u32::MAX
                    };
                    let origin_z_interactive_state = matches!(state_val, 3 | 4 | 6);
                    let origin_z_waiting_for_assets = origin_z_scene && peterstu_ready_seq == 0;
                    let origin_z_loader_quiet = origin_z_waiting_for_assets
                        || (origin_z_scene
                            && peterstu_ready_seq != 0
                            && !origin_z_interactive_state);
                    let origin_z_state4_input_enabled =
                        std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_STATE4_INPUT").is_some();
                    let origin_z_transition_wait =
                        origin_z_scene && !origin_z_state3 && !origin_z_state4_input_enabled;
                    // origin_z state 3 checks action 0x84 on the one frame
                    // where 17F has already cleared. The input poll that sees
                    // 17F=0 happens after that gate, so pre-arm START while the
                    // loader is waiting instead of reacting one poll late. Do
                    // not press through the asset stream itself: PETERSTU is the
                    // level-load boundary, and poking confirm before it is ready
                    // can bounce the frontend loader back into a visible cycle.
                    let origin_z_start_confirm_enabled = spidey_auto_new_game
                        || std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_START_CONFIRM").is_some();
                    let origin_z_start_prearm = origin_z_state3
                        && origin_z_start_confirm_enabled
                        && !origin_z_loader_quiet
                        && scene_b17f != 0
                        && confirm_n >= 8
                        && (phase % 4) == 0;
                    let origin_z_ready_start_limit =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_READY_STARTS")
                            .ok()
                            .and_then(|v| v.trim().parse::<u32>().ok())
                            .filter(|v| *v <= 16)
                            .unwrap_or(1);
                    let origin_z_ready_start_period =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_READY_START_PERIOD")
                            .ok()
                            .and_then(|v| v.trim().parse::<u32>().ok())
                            .filter(|v| (2..=16).contains(v))
                            .unwrap_or(8);
                    let origin_z_ready_start_hold =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_READY_START_HOLD")
                            .ok()
                            .and_then(|v| v.trim().parse::<u32>().ok())
                            .filter(|v| *v > 0)
                            .unwrap_or(2)
                            .min(origin_z_ready_start_period);
                    let origin_z_ready_start_edge = origin_z_state3
                        && origin_z_start_confirm_enabled
                        && !origin_z_loader_quiet
                        && origin_z_ready
                        && origin_z_ready_poll < 48
                        && (origin_z_ready_poll / origin_z_ready_start_period)
                            < origin_z_ready_start_limit
                        && (origin_z_ready_poll % origin_z_ready_start_period)
                            < origin_z_ready_start_hold;
                    let origin_z_start_pressed = origin_z_start_confirm_enabled
                        && !origin_z_loader_quiet
                        && (origin_z_start_prearm || origin_z_ready_start_edge);
                    // Any Origin_Z state after the loader handoff belongs to
                    // gameplay/menu code. Default automation stops there; the
                    // opt-in diagnostic path below can still drive it.
                    let release_settle = !origin_z_state3 && state_val == 4 && confirm_n < 16;
                    let opening_confirm = if state_val == 4 {
                        (16..40).contains(&confirm_n)
                    } else {
                        confirm_n < 12
                    };
                    let late_confirm = (96..128).contains(&phase);
                    let level_select_edge = !origin_z_state3
                        && !origin_z_loader_quiet
                        && !origin_z_transition_wait
                        && !release_settle
                        && (opening_confirm || late_confirm || (48..80).contains(&phase))
                        && (n & 1) == 0;
                    let mut a_pressed = if origin_z_state3 {
                        false
                    } else {
                        !origin_z_loader_quiet
                            && !origin_z_transition_wait
                            && !release_settle
                            && (opening_confirm || late_confirm || (48..80).contains(&phase))
                    };
                    let mut start_pressed = if origin_z_state3 {
                        origin_z_start_pressed
                    } else if origin_z_scene {
                        // Once Origin_Z accepts the loader handoff, START is the
                        // pause/options button. Keep any non-loader START input
                        // opt-in so the canary does not pause the tutorial as
                        // soon as gameplay becomes alive.
                        origin_z_state4_input_enabled
                            && origin_z_start_confirm_enabled
                            && !origin_z_transition_wait
                            && !origin_z_loader_quiet
                            && !release_settle
                            && (opening_confirm || late_confirm)
                    } else {
                        !origin_z_loader_quiet
                            && !release_settle
                            && (opening_confirm || late_confirm)
                    };
                    let mut back_pressed = level_select_edge && !origin_z_scene;
                    let origin_z_sustain_confirm = origin_z_scene
                        && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_SUSTAIN_CONFIRM").is_some()
                        && (state_val == 3
                            || (origin_z_state4_input_enabled && matches!(state_val, 4 | 6)))
                        && !origin_z_loader_quiet;
                    let origin_z_sustain_n = if origin_z_sustain_confirm {
                        SPIDEY_ORIGINZ_SUSTAIN_POLLS
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                    } else {
                        SPIDEY_ORIGINZ_SUSTAIN_POLLS.store(0, std::sync::atomic::Ordering::Relaxed);
                        0
                    };
                    let origin_z_sustain_release =
                        origin_z_sustain_confirm && origin_z_sustain_n < 8;
                    let origin_z_sustain_limit =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_SUSTAIN_CONFIRM")
                            .ok()
                            .and_then(|v| v.trim().parse::<u32>().ok())
                            .filter(|v| *v >= 16)
                            .unwrap_or(4096);
                    let origin_z_sustain_down = origin_z_sustain_confirm
                        && !origin_z_sustain_release
                        && origin_z_sustain_n < origin_z_sustain_limit;
                    if origin_z_sustain_release {
                        start_pressed = false;
                        back_pressed = false;
                        a_pressed = false;
                    } else if origin_z_sustain_down {
                        // START becomes Spider-Man action/event 0x1A in Origin_Z,
                        // but that scene's dispatch table has no slot 0x1A listener.
                        // Keep START as an opt-in diagnostic and let A be the normal
                        // confirm button for the automated path.
                        start_pressed = origin_z_start_confirm_enabled;
                        a_pressed = true;
                        // BACK maps into Spider-Man's event 0x1A path here, and the
                        // Origin_Z table has no listener for that slot. Keep it as an
                        // explicit diagnostic only; the default sustain should be the
                        // native confirm buttons without poisoning pending=0x100.
                        back_pressed = std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_SUSTAIN_BACK")
                            .is_some()
                            && (origin_z_sustain_n & 1) == 0;
                    }
                    if a_pressed {
                        a_a = 0xFF;
                    }
                    if back_pressed {
                        buttons |= 0x0020; // BACK
                        note_spidey_manual_confirm_edge();
                    }
                    if start_pressed {
                        buttons |= 0x0010; // START
                    }
                    let axis_phase = confirm_n % 256;
                    let axis_label = if origin_z_state3 {
                        "-"
                    } else if origin_z_loader_quiet {
                        "-"
                    } else if origin_z_transition_wait {
                        "-"
                    } else if (32..64).contains(&axis_phase) {
                        buttons |= 0x0001; // DPAD_UP
                        ly = i16::MAX;
                        "UP"
                    } else if (96..128).contains(&axis_phase) {
                        buttons |= 0x0002; // DPAD_DOWN
                        ly = i16::MIN;
                        "DOWN"
                    } else if (160..192).contains(&axis_phase) {
                        buttons |= 0x0008; // DPAD_RIGHT
                        lx = i16::MAX;
                        "RIGHT"
                    } else if (224..256).contains(&axis_phase) {
                        buttons |= 0x0004; // DPAD_LEFT
                        lx = i16::MIN;
                        "LEFT"
                    } else {
                        "-"
                    };
                    let xroot = read_u32(0x004C_06B8);
                    let frame = read_u32(0x004B_C630);
                    let frame_scene = read_u32(frame.wrapping_add(0x18));
                    let global_scene = read_u32(0x003F_5BEC);
                    let frame_delta_bits = read_u32(xroot.wrapping_add(0x18C));
                    let time_scale_bits = read_u32(xroot.wrapping_add(0x440));
                    let read_owner_chain = |owner: u32, manager_off: u32| -> (u32, u32, u32, u32) {
                        let mgr = read_u32(owner.wrapping_add(manager_off));
                        let head = read_u32(mgr.wrapping_add(0x10));
                        let first = read_u32(head);
                        let item = read_u32(first.wrapping_add(0x08));
                        (mgr, head, first, item)
                    };
                    let read_focus_chain =
                        |owner: u32| -> (u32, u32, u32, u32) { read_owner_chain(owner, 0x1A8) };
                    let read_script_chain =
                        |owner: u32| -> (u32, u32, u32, u32) { read_owner_chain(owner, 0x1A0) };
                    let script_global_mgr = read_u32(0x004B_8F04);
                    let script_global_ready = read_u32(0x004B_8F08);
                    let script_global_active = read_u32(0x004B_8F0C);
                    let script_global_table = read_u32(script_global_mgr.wrapping_add(0x20));
                    let (
                        xroot_script_mgr,
                        xroot_script_head,
                        xroot_script_first,
                        xroot_script_item,
                    ) = read_script_chain(xroot);
                    let (
                        engine_script_mgr,
                        engine_script_head,
                        engine_script_first,
                        engine_script_item,
                    ) = read_script_chain(engine_scene);
                    let (
                        frame_script_mgr,
                        frame_script_head,
                        frame_script_first,
                        frame_script_item,
                    ) = read_script_chain(frame_scene);
                    let (
                        global_script_mgr,
                        global_script_head,
                        global_script_first,
                        global_script_item,
                    ) = read_script_chain(global_scene);
                    let (focus_mgr, focus_head, focus_first, focus_item) = read_focus_chain(xroot);
                    let (
                        engine_focus_mgr,
                        engine_focus_head,
                        engine_focus_first,
                        engine_focus_item,
                    ) = read_focus_chain(engine_scene);
                    let (frame_focus_mgr, frame_focus_head, frame_focus_first, frame_focus_item) =
                        read_focus_chain(frame_scene);
                    let (
                        global_focus_mgr,
                        global_focus_head,
                        global_focus_first,
                        global_focus_item,
                    ) = read_focus_chain(global_scene);
                    let focus_flags8 = read_u8(focus_item.wrapping_add(0x08));
                    let focus_pc = read_u32(focus_item.wrapping_add(0x1C));
                    let focus_word = if focus_pc >= 0x1000 {
                        (read_u8(focus_pc) as u16)
                            | ((read_u8(focus_pc.wrapping_add(1)) as u16) << 8)
                    } else {
                        0
                    };
                    let focus_major = focus_word >> 8;
                    let focus_op = focus_word & 0x007F;
                    let focus_stack = read_u32(focus_item.wrapping_add(0x14));
                    let mut focus_wait = read_u32(focus_item.wrapping_add(0x2C));
                    let focus_timer = read_u32(focus_item.wrapping_add(0x40));
                    let focus_counter = read_u32(focus_item.wrapping_add(0x44));
                    let origin_z_state4_040a_wait = origin_z_scene
                        && state_val == 4
                        && scene_b17f == 0
                        && scene_b186 != 0
                        && scene_b18b != 0
                        && scene_b18c == 0
                        && peterstu_ready_seq != 0
                        && focus_word == 0x040A
                        && focus_wait != 0;
                    let origin_z_state4_040a_after =
                        std::env::var("RUSTEMU_SPIDEY_ORIGINZ_STATE4_COMPLETE_AFTER")
                            .ok()
                            .and_then(|v| v.trim().parse::<u32>().ok())
                            .filter(|v| *v >= 8)
                            .unwrap_or(32);
                    if origin_z_state4_040a_wait
                        && confirm_n >= origin_z_state4_040a_after
                        && (spidey_auto_new_game
                            || std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_STATE4_COMPLETE_AFTER")
                                .is_some())
                        && std::env::var_os("RUSTEMU_SPIDEY_DISABLE_ORIGINZ_STATE4_COMPLETE")
                            .is_none()
                    {
                        write_u8(engine_scene.wrapping_add(0x18C), 1);
                        scene_b18c = 1;
                        if std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_CLEAR_FOCUS_WAIT").is_some()
                            && std::env::var_os("RUSTEMU_SPIDEY_DISABLE_ORIGINZ_FOCUS_WAIT_CLEAR")
                                .is_none()
                        {
                            write_u32(focus_item.wrapping_add(0x2C), 0);
                            let old_focus_wait = focus_wait;
                            focus_wait = 0;
                            let clear_n = SPIDEY_ORIGINZ_STATE4_040A_WAIT_CLEAR_LOG
                                .fetch_add(1, Ordering::Relaxed);
                            if clear_n < 8 || clear_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGINZ-STATE4-040A-WAIT-CLEAR] #{} confirm_n={} focus=0x{:08X} pc=0x{:08X} word=0x{:04X} old_wait={} wrote [+0x2C]=0",
                                    clear_n,
                                    confirm_n,
                                    focus_item,
                                    focus_pc,
                                    focus_word,
                                    old_focus_wait
                                ));
                            }
                        }
                        let fire_n =
                            SPIDEY_ORIGINZ_STATE4_040A_HLE_LOG.fetch_add(1, Ordering::Relaxed);
                        if fire_n < 8 || fire_n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-STATE4-040A-HLE-FIRE] #{} confirm_n={} this=0x{:08X} focus=0x{:08X} pc=0x{:08X} word=0x{:04X} wrote [+0x18C]=1",
                                fire_n,
                                confirm_n,
                                engine_scene,
                                focus_item,
                                focus_pc,
                                focus_word
                            ));
                        }
                    }
                    if confirm_n < 16
                        || confirm_n == 16
                        || confirm_n == 48
                        || confirm_n == 96
                        || confirm_n == 160
                        || confirm_n % 192 == 0
                        || origin_z_state3
                    {
                        debug_log(&format!(
                            "[SPIDEY-TEST-LEVEL-CONFIRM] #{} phase={} poll={} scene='{}' stash='{}' engine=0x{:08X} frame=0x{:08X} frame_scene=0x{:08X} global=0x{:08X} state_top=0x{:08X} state_idx={} state_arr=0x{:08X} state_slot=0x{:08X} state_val={} flags17f/183/186/18b/18c={}/{}/{}/{}/{} START={} BACK={} A={} axis={} LX={} LY={} oz_pulse={} oz_sustain_n={} peterstu_seq={} state_poked={} quiet={} xroot=0x{:08X} delta=0x{:08X}/{:.6} scale=0x{:08X}/{:.6} focus=0x{:08X} f8=0x{:02X} pc=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} stack=0x{:08X} wait={} timer=0x{:08X}/{:.6} ctr={} script_global[mgr/ready/active/table=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}] script_chain[xroot mgr/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} engine=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} frame=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} global=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}] focus_chain[xroot mgr/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} engine=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} frame=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} global=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}]",
                            confirm_n,
                            phase,
                            n,
                            scene_name,
                            stash_name,
                            engine_scene,
                            frame,
                            frame_scene,
                            global_scene,
                            state_top,
                            state_idx,
                            state_arr,
                            state_slot,
                            state_val,
                            scene_b17f,
                            scene_b183,
                            scene_b186,
                            scene_b18b,
                            scene_b18c,
                            if start_pressed { 1 } else { 0 },
                            if back_pressed { 1 } else { 0 },
                            if a_pressed { 255 } else { 0 },
                            axis_label,
                            lx,
                            ly,
                            if origin_z_start_prearm && !origin_z_ready {
                                "prearm-start"
                            } else if origin_z_ready_start_edge {
                                "ready-start"
                            } else if origin_z_sustain_release {
                                "sustain-release"
                            } else if origin_z_sustain_down {
                                "sustain-hold"
                            } else if origin_z_start_pressed {
                                "start-edge"
                            } else if origin_z_state3 {
                                "wait-17f"
                            } else {
                                "-"
                            },
                            if origin_z_sustain_confirm {
                                origin_z_sustain_n
                            } else {
                                u32::MAX
                            },
                            peterstu_ready_seq,
                            if origin_z_state_poked { 1 } else { 0 },
                            if origin_z_loader_quiet { 1 } else { 0 },
                            xroot,
                            frame_delta_bits,
                            f32::from_bits(frame_delta_bits),
                            time_scale_bits,
                            f32::from_bits(time_scale_bits),
                            focus_item,
                            focus_flags8,
                            focus_pc,
                            focus_word,
                            focus_major,
                            focus_op,
                            focus_stack,
                            focus_wait,
                            focus_timer,
                            f32::from_bits(focus_timer),
                            focus_counter,
                            script_global_mgr,
                            script_global_ready,
                            script_global_active,
                            script_global_table,
                            xroot_script_mgr,
                            xroot_script_head,
                            xroot_script_first,
                            xroot_script_item,
                            engine_script_mgr,
                            engine_script_head,
                            engine_script_first,
                            engine_script_item,
                            frame_script_mgr,
                            frame_script_head,
                            frame_script_first,
                            frame_script_item,
                            global_script_mgr,
                            global_script_head,
                            global_script_first,
                            global_script_item,
                            focus_mgr,
                            focus_head,
                            focus_first,
                            focus_item,
                            engine_focus_mgr,
                            engine_focus_head,
                            engine_focus_first,
                            engine_focus_item,
                            frame_focus_mgr,
                            frame_focus_head,
                            frame_focus_first,
                            frame_focus_item,
                            global_focus_mgr,
                            global_focus_head,
                            global_focus_first,
                            global_focus_item
                        ));
                    }
                }
            }

            if std::env::var_os("RUSTEMU_INPUT_MAP_PROBE").is_some() {
                fn names_for_joypad_bits(bits: u32) -> String {
                    const NAMES: [&str; 16] = [
                        "SOUTH->XboxA",
                        "WEST->XboxX",
                        "BACK",
                        "START",
                        "UP",
                        "DOWN",
                        "LEFT",
                        "RIGHT",
                        "EAST->XboxB",
                        "NORTH->XboxY",
                        "BLACK/LB",
                        "WHITE/RB",
                        "LT",
                        "RT",
                        "L3",
                        "R3",
                    ];
                    let mut out = String::new();
                    for (idx, name) in NAMES.iter().enumerate() {
                        if (bits & (1u32 << idx)) != 0 {
                            if !out.is_empty() {
                                out.push('|');
                            }
                            out.push_str(name);
                        }
                    }
                    if out.is_empty() {
                        out.push('-');
                    }
                    out
                }

                fn read_probe_scene_string(mem: *mut u8, addr: u32, max_len: usize) -> String {
                    let mut bytes = Vec::with_capacity(max_len.min(64));
                    for i in 0..max_len {
                        let b = unsafe { *((mem as u64 + addr as u64 + i as u64) as *const u8) };
                        if b == 0 {
                            break;
                        }
                        if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                            bytes.push(b);
                        } else {
                            break;
                        }
                    }
                    String::from_utf8_lossy(&bytes).into_owned()
                }

                if spidey_synth_passthrough {
                    static SPIDEY_SYNTH_INPUT_RELEASE_LOG: std::sync::atomic::AtomicBool =
                        std::sync::atomic::AtomicBool::new(false);
                    buttons = normal_buttons;
                    a_a = normal_a_a;
                    a_b = normal_a_b;
                    a_x = normal_a_x;
                    a_y = normal_a_y;
                    lx = normal_lx;
                    ly = normal_ly;
                    if !SPIDEY_SYNTH_INPUT_RELEASE_LOG
                        .swap(true, std::sync::atomic::Ordering::Relaxed)
                    {
                        debug_log(&format!(
                            "[SPIDEY-SYNTH-INPUT-PASSTHROUGH] poll={} synthetic_input_released=1",
                            n
                        ));
                    }
                }

                let final_changed = normal_buttons != buttons
                    || normal_a_a != a_a
                    || normal_a_b != a_b
                    || normal_a_x != a_x
                    || normal_a_y != a_y
                    || normal_lx != lx
                    || normal_ly != ly;
                let raw_active = joypad_bits != 0;
                static INPUT_MAP_PROBE_LAST_RAW: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(u32::MAX);
                static INPUT_MAP_PROBE_LAST_FINAL: std::sync::atomic::AtomicU64 =
                    std::sync::atomic::AtomicU64::new(u64::MAX);
                let final_sig = (buttons as u64)
                    | ((a_a as u64) << 16)
                    | ((a_b as u64) << 24)
                    | ((a_x as u64) << 32)
                    | ((a_y as u64) << 40)
                    | (((lx as u16) as u64) << 48);
                let last_raw = INPUT_MAP_PROBE_LAST_RAW
                    .swap(joypad_bits, std::sync::atomic::Ordering::Relaxed);
                let last_final = INPUT_MAP_PROBE_LAST_FINAL
                    .swap(final_sig, std::sync::atomic::Ordering::Relaxed);
                if n < 4
                    || raw_active
                    || final_changed
                    || last_raw != joypad_bits
                    || last_final != final_sig
                {
                    let scene_name = read_probe_scene_string(guest_mem, 0x004B_C848, 64);
                    let stash_name = read_probe_scene_string(guest_mem, 0x004B_C948, 64);
                    debug_log(&format!(
                        "[INPUT-MAP-PROBE] poll={} raw=0x{:04X} raw_names={} normal(w=0x{:04X} A={} B={} X={} Y={} LX={} LY={}) final(w=0x{:04X} A={} B={} X={} Y={} LX={} LY={}) synthetic={} scene='{}' stash='{}'",
                        n,
                        joypad_bits,
                        names_for_joypad_bits(joypad_bits),
                        normal_buttons,
                        normal_a_a,
                        normal_a_b,
                        normal_a_x,
                        normal_a_y,
                        normal_lx,
                        normal_ly,
                        buttons,
                        a_a,
                        a_b,
                        a_x,
                        a_y,
                        lx,
                        ly,
                        if final_changed { 1 } else { 0 },
                        scene_name,
                        stash_name
                    ));
                }
            }

            let sig1 = (buttons as u64)
                | ((a_a as u64) << 16)
                | ((a_b as u64) << 24)
                | ((a_x as u64) << 32)
                | ((a_y as u64) << 40)
                | ((a_black as u64) << 48)
                | ((a_white as u64) << 56);
            let sig2 = (a_lt as u64)
                | ((a_rt as u64) << 8)
                | (((lx as u16) as u64) << 16)
                | (((ly as u16) as u64) << 32)
                | (((rx as u16) as u64) << 48);
            let sig3 = ry as u16 as u32;
            static LAST_XINPUT_SIG1: std::sync::atomic::AtomicU64 =
                std::sync::atomic::AtomicU64::new(0);
            static LAST_XINPUT_SIG2: std::sync::atomic::AtomicU64 =
                std::sync::atomic::AtomicU64::new(0);
            static LAST_XINPUT_SIG3: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            static XINPUT_PACKET: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let last1 = LAST_XINPUT_SIG1.load(std::sync::atomic::Ordering::Relaxed);
            let last2 = LAST_XINPUT_SIG2.load(std::sync::atomic::Ordering::Relaxed);
            let last3 = LAST_XINPUT_SIG3.load(std::sync::atomic::Ordering::Relaxed);
            let packet = if last1 != sig1 || last2 != sig2 || last3 != sig3 {
                LAST_XINPUT_SIG1.store(sig1, std::sync::atomic::Ordering::Relaxed);
                LAST_XINPUT_SIG2.store(sig2, std::sync::atomic::Ordering::Relaxed);
                LAST_XINPUT_SIG3.store(sig3, std::sync::atomic::Ordering::Relaxed);
                XINPUT_PACKET
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                    .wrapping_add(1)
            } else {
                XINPUT_PACKET.load(std::sync::atomic::Ordering::Relaxed)
            };

            unsafe {
                let base = (guest_mem as u64 + state_ptr as u64) as *mut u8;
                // dwPacketNumber
                *(base as *mut u32) = packet;
                // wButtons
                *((base as u64 + 4) as *mut u16) = buttons;
                // bAnalogButtons[8]
                *base.add(6) = a_a;
                *base.add(7) = a_b;
                *base.add(8) = a_x;
                *base.add(9) = a_y;
                *base.add(10) = a_black;
                *base.add(11) = a_white;
                *base.add(12) = a_lt;
                *base.add(13) = a_rt;
                // Thumbsticks
                *((base as u64 + 14) as *mut i16) = lx;
                *((base as u64 + 16) as *mut i16) = ly;
                *((base as u64 + 18) as *mut i16) = rx;
                *((base as u64 + 20) as *mut i16) = ry;
            }

            // Log a few early polls + any non-zero button reading so we can
            // see in debug.log whether the user actually pressed anything.
            if n < 3 || (buttons != 0 && n % 64 == 0) {
                debug_log(&format!(
                    "[HLE] XInputGetState #{} packet={} hDev=0x{:08X} wButtons=0x{:04X} A={} B={} X={} Y={} LX={} LY={}",
                    n, packet, args[0], buttons, a_a, a_b, a_x, a_y, lx, ly
                ));
            }
            if menu_wait_trace_log(n) || buttons != 0 || a_a != 0 || a_b != 0 {
                debug_log(&format!(
                    "[MENU-WAIT] XInputGetState #{} hDev=0x{:08X} state=0x{:08X} ret=0 packet={} raw=0x{:04X} buttons=0x{:04X} A={} B={} START={} BACK={} LX={} LY={}",
                    n,
                    args[0],
                    state_ptr,
                    packet,
                    joypad_bits,
                    buttons,
                    a_a,
                    a_b,
                    if (buttons & 0x0010) != 0 { 1 } else { 0 },
                    if (buttons & 0x0020) != 0 { 1 } else { 0 },
                    lx,
                    ly
                ));
            }
            if buttons != 0 || a_a != 0 || a_b != 0 || a_x != 0 || a_y != 0 {
                static NONZERO_INPUT_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let nonzero_n =
                    NONZERO_INPUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if nonzero_n < 24 {
                    debug_log(&format!(
                        "[HLE] XInputGetState NONZERO #{} poll={} packet={} hDev=0x{:08X} wButtons=0x{:04X} A={} B={} X={} Y={} LX={} LY={}",
                        nonzero_n, n, packet, args[0], buttons, a_a, a_b, a_x, a_y, lx, ly
                    ));
                }
            }
            if a_a != 0 || (buttons & 0x0020) != 0 {
                crate::xbox::emulator::mark_test_confirm_sampled();
            }
            0 // ERROR_SUCCESS
        }
        "XInputGetCapabilities" => {
            // XInputGetCapabilities(hDevice, pCaps) → DWORD
            // Xbox XINPUT_CAPABILITIES is packed as:
            //   +0  BYTE SubType
            //   +1  WORD Reserved
            //   +3  XINPUT_GAMEPAD input capabilities
            //   +21 XINPUT_RUMBLE feedback capabilities
            //
            // Cxbx-R sets SubType and fills the input+feedback capability area
            // with 0xFF. Returning an all-zero body makes titles believe the
            // connected controller has no analog axes or pressure buttons.
            let caps_ptr = args[1];
            if caps_ptr > 0 && caps_ptr < 0x2000_0000 {
                unsafe {
                    let caps = (guest_mem as u64 + caps_ptr as u64) as *mut u8;
                    std::ptr::write_bytes(caps, 0, 0x1C);
                    *caps.add(0) = 0x01; // XINPUT_DEVSUBTYPE_GC_GAMEPAD
                    std::ptr::write_bytes(caps.add(3), 0xFF, 0x16);
                }
            }
            0 // ERROR_SUCCESS
        }
        _ => 0, // generic: return D3D_OK
    }
}

const BINK_OFF_WIDTH: usize = 0x00;
const BINK_OFF_HEIGHT: usize = 0x04;
const BINK_OFF_FRAMES: usize = 0x08;
const BINK_OFF_FRAME_NUM: usize = 0x0C;
const BINK_OFF_KIND: usize = 0x10;
const BINK_OFF_LAST_READY_MS: usize = 0x14;
const BINK_KIND_FAST_SKIP: u32 = 0;
const BINK_KIND_ORIGIN1_TIMED: u32 = 1;
static ORIGIN1_BINK_ACTIVE: AtomicU32 = AtomicU32::new(0);
static ORIGIN1_BINK_FRAME: AtomicU32 = AtomicU32::new(0);
static ORIGIN1_BINK_FRAMES: AtomicU32 = AtomicU32::new(0);

fn hle_elapsed_ms() -> u32 {
    static T0: OnceLock<Instant> = OnceLock::new();
    T0.get_or_init(Instant::now)
        .elapsed()
        .as_millis()
        .min(u32::MAX as u128) as u32
}

fn spidey_origin1_bink_frames() -> u32 {
    static FRAMES: OnceLock<u32> = OnceLock::new();
    *FRAMES.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_ORIGIN1_BINK_FRAMES")
            .ok()
            .and_then(|v| parse_u32_env_token(&v))
            .unwrap_or(180)
            .clamp(1, 3600)
    })
}

fn spidey_origin1_bink_period_ms() -> u32 {
    static PERIOD: OnceLock<u32> = OnceLock::new();
    *PERIOD.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_ORIGIN1_BINK_PERIOD_MS")
            .ok()
            .and_then(|v| parse_u32_env_token(&v))
            .unwrap_or(33)
            .clamp(1, 1000)
    })
}

const SHENMUE_MATRIX_CURRENT: u32 = 0x01A4_4BD0;
const SHENMUE_MATRIX_STACK_TOP_PTR: u32 = 0x01A4_4BA8;

fn shenmue_matrix_stack_top(guest_mem: *mut u8) -> u32 {
    read_u32_guest(guest_mem, SHENMUE_MATRIX_STACK_TOP_PTR).unwrap_or(0)
}

fn shenmue_copy_matrix64(
    guest_mem: *mut u8,
    dst: u32,
    src: u32,
    tag: &str,
    n: u32,
    esp: u32,
) -> bool {
    let dst_norm = normalize_guest_ram_ptr(dst);
    let src_norm = normalize_guest_ram_ptr(src);
    let valid = |addr: Option<u32>| -> Option<usize> {
        let addr = addr? as usize;
        addr.checked_add(64)
            .filter(|end| *end <= 0x2000_0000)
            .map(|_| addr)
    };

    match (valid(dst_norm), valid(src_norm)) {
        (Some(dst_off), Some(src_off)) => {
            unsafe {
                std::ptr::copy(guest_mem.add(src_off), guest_mem.add(dst_off), 64);
            }
            if n < 24 || n.is_power_of_two() {
                let first = read_u32_guest(guest_mem, dst).unwrap_or(0);
                debug_log(&format!(
                    "[{}] #{} dst=0x{:08X}->0x{:08X} src=0x{:08X}->0x{:08X} first=0x{:08X} esp=0x{:08X}",
                    tag, n, dst, dst_off as u32, src, src_off as u32, first, esp
                ));
            }
            true
        }
        _ => {
            if n < 24 || n.is_power_of_two() {
                debug_log(&format!(
                    "[{}] #{} skipped invalid dst=0x{:08X} src=0x{:08X} esp=0x{:08X}",
                    tag, n, dst, src, esp
                ));
            }
            false
        }
    }
}

fn shenmue_multiply_current_matrix(guest_mem: *mut u8, src: u32, n: u32, esp: u32) -> bool {
    let mut lhs = [0.0f32; 16];
    let mut rhs = [0.0f32; 16];
    for i in 0..16u32 {
        let Some(l) = read_f32_guest(guest_mem, src.wrapping_add(i * 4)) else {
            if n < 24 || n.is_power_of_two() {
                debug_log(&format!(
                    "[SHENMUE-MTX-MUL] #{} skipped invalid src=0x{:08X} esp=0x{:08X}",
                    n, src, esp
                ));
            }
            return false;
        };
        let Some(r) = read_f32_guest(guest_mem, SHENMUE_MATRIX_CURRENT.wrapping_add(i * 4)) else {
            if n < 24 || n.is_power_of_two() {
                debug_log(&format!(
                    "[SHENMUE-MTX-MUL] #{} skipped invalid current=0x{:08X} esp=0x{:08X}",
                    n, SHENMUE_MATRIX_CURRENT, esp
                ));
            }
            return false;
        };
        lhs[i as usize] = l;
        rhs[i as usize] = r;
    }

    let mut out = [0.0f32; 16];
    for row in 0..4usize {
        for col in 0..4usize {
            out[row * 4 + col] = lhs[row * 4] * rhs[col]
                + lhs[row * 4 + 1] * rhs[4 + col]
                + lhs[row * 4 + 2] * rhs[8 + col]
                + lhs[row * 4 + 3] * rhs[12 + col];
        }
    }

    for i in 0..16u32 {
        if write_f32_guest(
            guest_mem,
            SHENMUE_MATRIX_CURRENT.wrapping_add(i * 4),
            out[i as usize],
        )
        .is_none()
        {
            return false;
        }
    }

    if n < 24 || n.is_power_of_two() {
        debug_log(&format!(
            "[SHENMUE-MTX-MUL] #{} current=0x{:08X} rhs=0x{:08X} first={:.6} esp=0x{:08X}",
            n, SHENMUE_MATRIX_CURRENT, src, out[0], esp
        ));
    }
    true
}

pub(super) fn execute_manual_hle(
    name: &str,
    args: &[u32; 8],
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_mem: *mut u8,
) -> u32 {
    // Manual hooks: skip the function entirely, return safe values
    match name {
        "Shenmue_CRT_memmove" => {
            let esp = context.R14 as u32;
            let dst = read_u32_guest(guest_mem, esp.wrapping_add(4)).unwrap_or(0);
            let src = read_u32_guest(guest_mem, esp.wrapping_add(8)).unwrap_or(0);
            let count = read_u32_guest(guest_mem, esp.wrapping_add(12)).unwrap_or(0);
            let dst_norm = normalize_guest_ram_ptr(dst);
            let src_norm = normalize_guest_ram_ptr(src);
            let count_usize = count as usize;
            let valid_range = |base: Option<u32>| -> Option<usize> {
                let base = base? as usize;
                base.checked_add(count_usize)
                    .filter(|end| *end <= 0x2000_0000)
                    .map(|_| base)
            };

            static SHENMUE_MEMMOVE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = SHENMUE_MEMMOVE_LOG.fetch_add(1, Ordering::Relaxed);

            match (valid_range(dst_norm), valid_range(src_norm)) {
                (Some(dst_off), Some(src_off)) => {
                    if count_usize != 0 {
                        unsafe {
                            std::ptr::copy(
                                guest_mem.add(src_off),
                                guest_mem.add(dst_off),
                                count_usize,
                            );
                        }
                    }
                    if n < 24 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[SHENMUE-CRT-MEMMOVE] #{} dst=0x{:08X}->0x{:08X} src=0x{:08X}->0x{:08X} count={} esp=0x{:08X}",
                            n,
                            dst,
                            dst_off as u32,
                            src,
                            src_off as u32,
                            count,
                            esp
                        ));
                    }
                }
                _ => {
                    if n < 24 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[SHENMUE-CRT-MEMMOVE] #{} skipped invalid dst=0x{:08X} src=0x{:08X} count={} esp=0x{:08X}",
                            n, dst, src, count, esp
                        ));
                    }
                }
            }

            dst
        }
        "Shenmue_MatrixLoadCurrent" => {
            let esp = context.R14 as u32;
            let arg = read_u32_guest(guest_mem, esp.wrapping_add(4)).unwrap_or(0);
            let src = if arg == 0 {
                shenmue_matrix_stack_top(guest_mem)
            } else {
                arg
            };
            static SHENMUE_MTX_LOAD_LOG: AtomicU32 = AtomicU32::new(0);
            let n = SHENMUE_MTX_LOAD_LOG.fetch_add(1, Ordering::Relaxed);
            let dst = SHENMUE_MATRIX_CURRENT;
            let _ = shenmue_copy_matrix64(guest_mem, dst, src, "SHENMUE-MTX-LOAD", n, esp);
            dst
        }
        "Shenmue_MatrixStoreCurrent" => {
            let esp = context.R14 as u32;
            let arg = read_u32_guest(guest_mem, esp.wrapping_add(4)).unwrap_or(0);
            let dst = if arg == 0 {
                shenmue_matrix_stack_top(guest_mem)
            } else {
                arg
            };
            static SHENMUE_MTX_STORE_LOG: AtomicU32 = AtomicU32::new(0);
            let n = SHENMUE_MTX_STORE_LOG.fetch_add(1, Ordering::Relaxed);
            let src = SHENMUE_MATRIX_CURRENT;
            let _ = shenmue_copy_matrix64(guest_mem, dst, src, "SHENMUE-MTX-STORE", n, esp);
            dst
        }
        "Shenmue_MatrixMultiplyCurrent" => {
            let esp = context.R14 as u32;
            let src = read_u32_guest(guest_mem, esp.wrapping_add(4)).unwrap_or(0);
            static SHENMUE_MTX_MUL_LOG: AtomicU32 = AtomicU32::new(0);
            let n = SHENMUE_MTX_MUL_LOG.fetch_add(1, Ordering::Relaxed);
            let _ = shenmue_multiply_current_matrix(guest_mem, src, n, esp);
            src
        }
        "BinkOpen" => {
            // RAD Bink is third-party middleware statically linked into the XBE.
            // Native decode currently renders correctly but advances at only a
            // few FPS under AOT, which traps the boot chain in ACTIVISN.bik.
            // Return a minimal BINK-like struct with one frame so the game's
            // own movie wrapper opens, initializes surfaces, then naturally
            // decides the clip is complete and advances to the next boot step.
            let handle = alloc_fake_bink_obj(guest_mem, 0x300);
            if handle == 0 {
                return 0;
            }

            fn read_c_string(guest_mem: *mut u8, addr: u32, max_len: usize) -> String {
                if addr == 0 || addr >= 0x2000_0000 {
                    return String::new();
                }
                let mut bytes = Vec::with_capacity(max_len.min(64));
                for i in 0..max_len {
                    let b = unsafe { *guest_mem.add(addr as usize + i) };
                    if b == 0 {
                        break;
                    }
                    if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                        bytes.push(b);
                    } else {
                        break;
                    }
                }
                String::from_utf8_lossy(&bytes).into_owned()
            }

            let file = read_c_string(guest_mem, args[0], 96);
            let lower = file.to_ascii_lowercase();
            let kind = if lower.contains("origin1.bik") {
                BINK_KIND_ORIGIN1_TIMED
            } else {
                BINK_KIND_FAST_SKIP
            };
            let frames = if kind == BINK_KIND_ORIGIN1_TIMED {
                spidey_origin1_bink_frames()
            } else {
                1
            };

            unsafe {
                let base = guest_mem.add(handle as usize);
                std::ptr::write_unaligned(base.add(BINK_OFF_WIDTH) as *mut u32, 640);
                std::ptr::write_unaligned(base.add(BINK_OFF_HEIGHT) as *mut u32, 480);
                std::ptr::write_unaligned(base.add(BINK_OFF_FRAMES) as *mut u32, frames);
                std::ptr::write_unaligned(base.add(BINK_OFF_FRAME_NUM) as *mut u32, 0);
                std::ptr::write_unaligned(base.add(BINK_OFF_KIND) as *mut u32, kind);
                std::ptr::write_unaligned(
                    base.add(BINK_OFF_LAST_READY_MS) as *mut u32,
                    hle_elapsed_ms(),
                );
            }
            if kind == BINK_KIND_ORIGIN1_TIMED {
                ORIGIN1_BINK_ACTIVE.store(1, Ordering::Relaxed);
                ORIGIN1_BINK_FRAME.store(0, Ordering::Relaxed);
                ORIGIN1_BINK_FRAMES.store(frames, Ordering::Relaxed);
            }

            static BINK_OPEN_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BINK_OPEN_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 12 {
                debug_log(&format!(
                    "[BINK-HLE] Open #{} file='{}' flags=0x{:08X} kind={} frames={} period_ms={} -> handle=0x{:08X}",
                    n, file, args[1], kind, frames, spidey_origin1_bink_period_ms(), handle
                ));
            }
            handle
        }
        "BinkPause" => {
            static BINK_PAUSE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BINK_PAUSE_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 8 {
                debug_log(&format!(
                    "[BINK-HLE] Pause #{} handle=0x{:08X} pause={}",
                    n, args[0], args[1]
                ));
            }
            0
        }
        "BinkWait" => {
            if args[0] != 0 {
                unsafe {
                    let base = guest_mem.add(args[0] as usize);
                    let kind = std::ptr::read_unaligned(base.add(BINK_OFF_KIND) as *const u32);
                    if kind == BINK_KIND_ORIGIN1_TIMED {
                        let frame =
                            std::ptr::read_unaligned(base.add(BINK_OFF_FRAME_NUM) as *const u32);
                        let frames =
                            std::ptr::read_unaligned(base.add(BINK_OFF_FRAMES) as *const u32)
                                .max(1);
                        if frame < frames {
                            let now = hle_elapsed_ms();
                            let last = std::ptr::read_unaligned(
                                base.add(BINK_OFF_LAST_READY_MS) as *const u32
                            );
                            if now.saturating_sub(last) < spidey_origin1_bink_period_ms() {
                                return 1;
                            }
                            std::ptr::write_unaligned(
                                base.add(BINK_OFF_LAST_READY_MS) as *mut u32,
                                now,
                            );
                        }
                    }
                }
            }
            0
        }
        "BinkDoFrame" => 0,
        "BinkCopyToBuffer" => 0,
        "BinkNextFrame" => {
            if args[0] != 0 {
                unsafe {
                    let base = guest_mem.add(args[0] as usize);
                    let frames =
                        std::ptr::read_unaligned(base.add(BINK_OFF_FRAMES) as *const u32).max(1);
                    let kind = std::ptr::read_unaligned(base.add(BINK_OFF_KIND) as *const u32);
                    if kind == BINK_KIND_ORIGIN1_TIMED {
                        let frame =
                            std::ptr::read_unaligned(base.add(BINK_OFF_FRAME_NUM) as *const u32);
                        let next = frame.saturating_add(1).min(frames);
                        std::ptr::write_unaligned(base.add(BINK_OFF_FRAME_NUM) as *mut u32, next);
                        ORIGIN1_BINK_FRAME.store(next, Ordering::Relaxed);
                        static ORIGIN1_NEXT_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = ORIGIN1_NEXT_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 12 || next == frames || next % 60 == 0 {
                            debug_log(&format!(
                                "[BINK-HLE] ORIGIN1 NextFrame #{} handle=0x{:08X} frame={}/{}",
                                n, args[0], next, frames
                            ));
                        }
                    } else {
                        std::ptr::write_unaligned(base.add(BINK_OFF_FRAME_NUM) as *mut u32, frames);
                    }
                }
            }
            0
        }
        "BinkClose" => {
            static BINK_CLOSE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BINK_CLOSE_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 12 {
                let (kind, frame, frames) = if args[0] != 0 {
                    unsafe {
                        let base = guest_mem.add(args[0] as usize);
                        (
                            std::ptr::read_unaligned(base.add(BINK_OFF_KIND) as *const u32),
                            std::ptr::read_unaligned(base.add(BINK_OFF_FRAME_NUM) as *const u32),
                            std::ptr::read_unaligned(base.add(BINK_OFF_FRAMES) as *const u32),
                        )
                    }
                } else {
                    (0, 0, 0)
                };
                if kind == BINK_KIND_ORIGIN1_TIMED {
                    ORIGIN1_BINK_ACTIVE.store(0, Ordering::Relaxed);
                    ORIGIN1_BINK_FRAME.store(frame, Ordering::Relaxed);
                    ORIGIN1_BINK_FRAMES.store(frames, Ordering::Relaxed);
                }
                debug_log(&format!(
                    "[BINK-HLE] Close #{} handle=0x{:08X} kind={} frame={}/{}",
                    n, args[0], kind, frame, frames
                ));
            }
            0
        }
        "D3D_RenderStateSort" | "D3D_RenderStateSort2" => 0, // skip sort
        "D3D_RenderStateInit" => 0,                          // skip init
        "D3DResource_Register" => {
            hle_register_resource(context.Rcx as u32, args, guest_mem);
            0
        }
        "D3DDevice_CreatePalette" => hle_create_palette(args, guest_mem, false),
        "D3DDevice_CreatePalette2" => hle_create_palette(args, guest_mem, true),
        "D3DPalette_Lock" => hle_palette_lock(name, args, context.Rcx as u32, guest_mem),
        "D3DPalette_Lock2" => hle_palette_lock(name, args, context.Rcx as u32, guest_mem),
        "D3DDevice_SetPalette" => hle_set_palette(args, guest_mem),
        "RtlAllocateHeap" => {
            // CRT statically-linked RtlAllocateHeap core at guest sub_002AB360.
            // stdcall(heap, flags, size) → ptr.
            //
            // We IGNORE the heap handle arg (args[0]) — all allocations go to a
            // shared emulated heap in guest RAM. Unlike the old pure bump
            // allocator, this keeps a host-side size table so RtlFreeHeap can
            // recycle blocks without changing the guest pointer layout that
            // earlier Spider-Man initialization already proved stable.
            //
            // Thread-safety: atomic fetch_add provides reservation; separate
            // HLE_HEAP_LOCK serializes zero-fill + log writes across threads.
            let _flags = args[1];
            let size = args[2];
            let aligned = ((size.max(1) + 15) & !15).max(16);

            // Pool range: 0x1000_0000..0x1C00_0000.
            //
            // Moved here 2026-04-21 after the Stage 11/12 breakthrough revealed
            // a three-way collision at low RAM:
            //   - MmAllocateContiguousMemory: phys 0x00D0_0000 → 0x0400_0000 (54MB)
            //   - (old) RtlAllocateHeap pool: phys 0x0200_0000 → 0x0C00_0000
            //                                 ← OVERLAPPED CONTIG AT 0x0200-0x0400_0000
            //   - NtAllocateVirtualMemory bump + fake HEAP struct: phys 0x0400_0000+
            //
            // 0x1000_0000 is past contiguous + fake-D3D data pools and stays
            // below the high guest stacks near 0x1EFFFFxx.
            let _guard = HLE_HEAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let free_list = HLE_HEAP_FREE_LIST.get_or_init(|| Mutex::new(Vec::new()));
            let alloc_sizes = HLE_HEAP_ALLOC_SIZES.get_or_init(|| Mutex::new(Vec::new()));

            let mut ptr = 0u32;
            {
                let mut free = free_list.lock().unwrap_or_else(|e| e.into_inner());
                if let Some((idx, _)) = free
                    .iter()
                    .enumerate()
                    .find(|(_, &(_, block_size))| block_size >= aligned)
                {
                    let (reused, _) = free.swap_remove(idx);
                    ptr = reused;
                }
            }

            if ptr == 0 {
                ptr = HLE_HEAP_BUMP.fetch_add(aligned, Ordering::Relaxed);
                if ptr
                    .checked_add(aligned)
                    .map(|end| end >= HLE_HEAP_POOL_END)
                    .unwrap_or(true)
                {
                    debug_log(&format!(
                        "[HLE] RtlAllocateHeap: OOM size={} aligned={} bump=0x{:08X} (pool exhausted)",
                        size, aligned, ptr
                    ));
                    return 0;
                }
                {
                    let mut sizes = alloc_sizes.lock().unwrap_or_else(|e| e.into_inner());
                    sizes.push((ptr, aligned));
                }
            }

            // Always zero-fill — the CRT frequently passes flags=0 yet relies on
            // freshly-minted std::string / std::vector memory being clean. Cheap
            // and matches what the real RtlAllocateHeap does under HEAP_ZERO_MEMORY.
            if size > 0 && size < 0x0100_0000 {
                unsafe {
                    // ptr is a virtual guest address inside guest RAM (0x02000000+),
                    // so use it directly as a host offset into guest_mem.
                    std::ptr::write_bytes(guest_mem.add(ptr as usize), 0, aligned as usize);
                }
            }

            static LOG_CTR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG_CTR.fetch_add(1, Ordering::Relaxed);
            if n < 20 {
                debug_log(&format!(
                    "[HLE] RtlAllocateHeap: size={} flags=0x{:X} → 0x{:08X}",
                    size, _flags, ptr
                ));
            } else if n == 20 {
                debug_log("[HLE] RtlAllocateHeap: (subsequent allocations suppressed)");
            }

            // [SUBSYS-ALLOC] Constructor-fingerprinting probe.
            //
            // Goal: when RtlAllocateHeap returns a pointer that ends up stored
            // at a watched subsystem slot (0x4BC614 = 0x102489B0, etc.), capture
            // VERBOSE context — size, flags, AND the guest stack top so we can
            // identify the constructor that called us.
            //
            // Strategy: hard-code the interesting target pointers we already
            // know about from earlier SUBSYS-WATCH runs. When the bump
            // allocator hands out one of those, dump:
            //   - The size requested (tells us object class size)
            //   - The guest ESP and top 8 dwords of guest stack
            //     The first dword is the return address — guest_pc of caller.
            //     Subsequent dwords are saved registers / locals / call args.
            //
            // Once we know the caller, the disasm DB tells us the function and
            // we can examine its body for the missing +0x30 sub-allocation.
            const WATCHED_PTRS: &[u32] = &[
                0x10202800, // "App" object (frame ~300)
                0x10203750, // "Scene" object (frame ~300)
                0x102489B0, // The "Engine" / LOD-primary object (frame 493)
            ];
            if WATCHED_PTRS.contains(&ptr) {
                // Walk the guest stack 32 dwords deep so we get past the
                // CRT malloc wrappers and reach the actual GAME caller.
                //
                // Expected chain at our entry (sub_002AB360 = RtlAllocateHeap):
                //   stack[0]    = ret to sub_002B4FA0  (0x002B4FC6)
                //   stack[1..3] = stdcall args (heap, flags, size)
                //   stack[4..7] = sub_002B4FC7 frame (ret 0x002B4FD7, size, ...)
                //   stack[6..7] = sub_002B4FF3 frame (ret 0x002B5002, ...)
                //   stack[8..]  = SUB_002B4FF3's caller — usually the GAME
                //                 constructor that wanted memory.
                //
                // Look for guest PCs in the .text/D3D ranges (0x00011000 to
                // 0x002EBD90 / 0x002EBDA0..0x003038F8) — those are real call
                // sites in the game. Anything > 0x00400000 is likely data,
                // not code.
                let r14 = context.R14 as u32;
                let mut hex_lines = String::new();
                if r14 != 0 && r14 < 0x2000_0000 {
                    for i in 0..32u32 {
                        let a = guest_mem as u64 + (r14 as u64) + (i as u64) * 4;
                        let w = unsafe { std::ptr::read_unaligned(a as *const u32) };
                        // Mark probable code addresses with [C].
                        let is_code = (0x00011000..=0x003038F8).contains(&w);
                        if i % 8 == 0 && i > 0 {
                            hex_lines.push('|');
                        } else if i > 0 {
                            hex_lines.push(' ');
                        }
                        hex_lines.push_str(&format!("{:08X}{}", w, if is_code { "C" } else { "" }));
                    }
                }
                debug_log(&format!(
                    "[SUBSYS-ALLOC] target=0x{:08X} size={} flags=0x{:X} esp=0x{:08X} stack[0..32]={}",
                    ptr, size, _flags, r14, hex_lines
                ));
            }

            ptr
        }
        "RtlFreeHeap" => {
            // CRT statically-linked RtlFreeHeap core at guest sub_002AC3A2.
            // stdcall(heap, flags, ptr) → BOOL.
            // Recycle blocks allocated by the HLE heap above. Unknown pointers
            // still return TRUE because the guest only needs the API contract,
            // not Win32 heap metadata fidelity.
            // The game's __free_base wrapper negates AL and computes
            // `(al == 0 ? ptr : 0)` from the return — returning 1 means "freed OK,
            // ptr is now dead, caller will NULL out the reference".
            let ptr = args[2];
            if ptr >= HLE_HEAP_POOL_BASE && ptr < HLE_HEAP_POOL_END {
                let _guard = HLE_HEAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
                let alloc_sizes = HLE_HEAP_ALLOC_SIZES.get_or_init(|| Mutex::new(Vec::new()));
                let size = {
                    let sizes = alloc_sizes.lock().unwrap_or_else(|e| e.into_inner());
                    sizes
                        .iter()
                        .find(|&&(p, _)| p == ptr)
                        .map(|&(_, size)| size)
                        .unwrap_or(0)
                };
                if size > 0 {
                    let free_list = HLE_HEAP_FREE_LIST.get_or_init(|| Mutex::new(Vec::new()));
                    let mut free = free_list.lock().unwrap_or_else(|e| e.into_inner());
                    if !free.iter().any(|&(p, _)| p == ptr) {
                        free.push((ptr, size));
                    }
                }
            }
            static FREE_CTR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = FREE_CTR.fetch_add(1, Ordering::Relaxed);
            if n < 20 {
                debug_log(&format!(
                    "[HLE] RtlFreeHeap: ptr=0x{:08X} flags=0x{:X} (no-op)",
                    args[2], args[1]
                ));
            } else if n == 20 {
                debug_log("[HLE] RtlFreeHeap: (subsequent frees suppressed)");
            }
            1 // TRUE
        }
        "XGRPH_AssembleShader" => {
            // XGRPH shader assembler/preprocessor wrapper at 0x0033022F.
            // This is Xbox graphics middleware. Spider-Man feeds short static
            // shader strings here, then immediately passes the returned XGBuffer
            // into D3DDevice_CreateVertexShader. Our D3D shader HLE ignores the
            // compiled microcode bytes, and CreateVertexShader consumes that
            // XGBuffer shortly afterwards. Capture the raw NV2A token stream
            // now so the D3D11 backend can compile a translated HLSL shader.
            let source_name = args[0];
            let source = args[1];
            let source_len = args[2];
            let flags = args[3];
            let pp_code = args[5];
            fn read_shader_source(
                guest_mem: *mut u8,
                addr: u32,
                len: u32,
                max_len: usize,
            ) -> String {
                if addr == 0 || addr >= 0x2000_0000 {
                    return String::new();
                }
                let max = (len as usize).min(max_len);
                let mut bytes = Vec::with_capacity(max);
                for i in 0..max {
                    let b = unsafe { *guest_mem.add(addr as usize + i) };
                    let mapped = match b {
                        b'\r' => b'\n',
                        b'\n' | b'\t' => b,
                        0 => break,
                        0x20..=0x7E => b,
                        _ => b'?',
                    };
                    bytes.push(mapped);
                }
                String::from_utf8_lossy(&bytes).into_owned()
            }
            let source_full = read_shader_source(guest_mem, source, source_len, 8192);
            static XGRPH_SOURCE_DUMP: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let dump_index = XGRPH_SOURCE_DUMP.fetch_add(1, Ordering::Relaxed);
            if dump_index < 16 || source_full.contains("vsf_sphere") {
                let _ = std::fs::write(
                    format!(
                        "./spidey_xgrph_shader_{:02}_{:04X}.xvs",
                        dump_index, source_len
                    ),
                    &source_full,
                );
            }
            let source_snip = source_full.chars().take(512).collect::<String>();
            record_pending_shader_source(source_snip.clone());
            let captured = crate::xbox::gpu::nv2a_vsh::capture_from_xgrph_source(&source_full);
            let token_bytes = (captured.tokens.len() * 4).max(0x10) as u32;
            let token_log = crate::xbox::gpu::nv2a_vsh::first_token_words(&captured.tokens, 16);
            crate::xbox::gpu::nv2a_vsh::register_pending_xgrph_shader(captured.clone());
            if let Some(pp_code_norm) = guest_ram_offset(pp_code) {
                let buf = alloc_hle_heap_block(guest_mem, 0x0C);
                let data = alloc_hle_heap_block(guest_mem, token_bytes);
                if buf != 0 && data != 0 {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            captured.tokens.as_ptr() as *const u8,
                            guest_mem.add(data as usize),
                            captured.tokens.len() * 4,
                        );

                        let buf_ptr = guest_mem.add(buf as usize) as *mut u32;
                        std::ptr::write_unaligned(buf_ptr.add(0), 1); // refcount
                        std::ptr::write_unaligned(buf_ptr.add(1), data);
                        std::ptr::write_unaligned(buf_ptr.add(2), token_bytes);
                        std::ptr::write_unaligned(guest_mem.add(pp_code_norm) as *mut u32, buf);
                    }
                }
            }

            static XGRPH_ASM_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = XGRPH_ASM_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 20 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE] XGRPH_AssembleShader #{} name=0x{:08X} src=0x{:08X} len={} flags=0x{:X} ppCode=0x{:08X} tokens={} first=[{}] decoded=[{}] snip='{}' -> S_OK",
                    n,
                    source_name,
                    source,
                    source_len,
                    flags,
                    pp_code,
                    token_bytes / 4,
                    token_log,
                    captured.decoded_lines.iter().take(8).cloned().collect::<Vec<_>>().join(";"),
                    source_snip.split_whitespace().take(48).collect::<Vec<_>>().join(" ")
                ));
            }
            0
        }
        "XGRPH_CreateTexture" => {
            // XGRPH wrapper: (device, pDescOrData, format, ppTexture).
            // Spider-Man calls this with pDescOrData = pointer into .data
            // (e.g. 0x003E10B8) or into the decompressed stash heap
            // (0x1020xxxx). Read a few words from there to find width,
            // height, and the actual pixel-data pointer — then seed the
            // fake texture header with those so subsequent SetTexture
            // uploads read from the real Treyarch-managed buffer, not
            // our decoy 0x045E0000.
            let pp_texture = args[3];
            let p_desc = args[1];
            let format = args[2];

            let is_guest_ptr = |p: u32| -> bool {
                (p >= 0x1_0000 && p < 0x2000_0000) || (p >= 0x8000_0000 && p < 0xA000_0000)
            };

            if let Some(pp_texture_norm) = guest_ram_offset(pp_texture) {
                let fake = alloc_fake_d3d_obj(guest_mem, 0x48);
                if fake != 0 {
                    // Read up to 8 words from the descriptor to hunt for
                    // width/height/data_ptr patterns.
                    let mut desc = [0u32; 8];
                    if is_guest_ptr(p_desc) {
                        unsafe {
                            let base = guest_mem.add((p_desc & 0x1FFF_FFFF) as usize);
                            for i in 0..8 {
                                desc[i] = *((base.add(i * 4)) as *const u32);
                            }
                        }
                    }
                    if let Some((tw, th, data_addr)) =
                        upload_xgrph_embedded_a1r5g5b5(guest_mem, p_desc, fake)
                    {
                        unsafe {
                            let pp = guest_mem.add(pp_texture_norm);
                            *(pp as *mut u32) = fake;
                        }
                        static XGRPH_EMBED_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = XGRPH_EMBED_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 8 {
                            debug_log(&format!(
                                "[HLE] XGRPH embedded texture: src=0x{:08X} {}x{} A1R5G5B5 -> data=0x{:08X} fake=0x{:08X} @[0x{:08X}]",
                                p_desc, tw, th, data_addr, fake, pp_texture
                            ));
                        }
                        return 0;
                    }
                    // Heuristic: find a word that looks like a valid guest
                    // RAM pointer INTO the heap/decompressed-stash region.
                    // That's our real data pointer. Spider-Man's LEGAL.xbs
                    // decompressed output lives at 0x1020xxxx+.
                    let mut data_ptr: u32 = 0;
                    for &w in &desc {
                        let nrm = w & 0x1FFF_FFFF;
                        if nrm >= 0x0200_0000 && nrm < 0x2000_0000 {
                            data_ptr = nrm;
                            break;
                        }
                    }
                    // 2026-04-24 EOS: removed the 0x0072AF58 bridge. Agents
                    // D01/I06 dumped that address and found it's a VERTEX
                    // buffer (two 1.0f floats + 0x00FF00FF white-color
                    // stream), NOT a texture descriptor. The earlier
                    // "sparse dots" rendering was raw vertex/depth data
                    // misinterpreted as BGRA. Leave Data=0 and rely on the
                    // self-healing read in hle_set_texture (which lazily
                    // picks up whatever Register / XGSetTextureHeader
                    // writes to [tex+0x04] later).
                    let _ = data_ptr; // keep as detected (0 if template)
                                      // If desc[0], desc[1] look like sane dimensions, use
                                      // them; else compute dims that FIT the stash buffer.
                                      // LEGAL.xbs primary decompressed output is 15505 bytes.
                                      // 64x64 A8R8G8B8 = 16384 bytes ≈ fits. Using 64x64
                                      // means glyph UV sampling (~0.09, 0.12) lands at byte
                                      // ~2300 which IS inside the valid decompressed range.
                                      // This lets us visually confirm real stash data flows,
                                      // even if the final image needs proper format decode.
                    let w_candidate = desc[0];
                    let h_candidate = desc[1];
                    let (tw, th) = if sane_texture_dims(w_candidate, h_candidate) {
                        (w_candidate, h_candidate)
                    } else {
                        (64, 64)
                    };

                    write_fake_texture_header(guest_mem, fake, tw, th, format, data_ptr);

                    // 2026-04-25: organic-only path. The earlier
                    // host-side `legal_decompressed.bin` injection at
                    // 0x00900000 was an anti-pattern — it bypassed the
                    // game's own decompressed buffer and reimplemented
                    // resource init in the emulator. Per agent J5, the
                    // game already passes its decompressed pointer
                    // through pDesc[] (the same word picked up by the
                    // heuristic loop above into `data_ptr`). Trust it,
                    // but verify: pointer must land in guest RAM, and
                    // format low byte must be a recognised D3DFMT code.
                    //
                    // One-time diagnostic of the organic descriptor so
                    // we can compare what the game ships against the
                    // bytes the host injection used to force.
                    use std::sync::atomic::{AtomicBool, Ordering as AO_DIAG};
                    static ORGANIC_DIAG_LOGGED: AtomicBool = AtomicBool::new(false);
                    let format_code = xbox_texture_format_code(format);
                    let format_sane = matches!(
                        format_code,
                        // Subset of swizzled/linear D3DFMT_* codes Spider-Man
                        // emits at this stage. Extend if a new code shows up.
                        0x06 | 0x07 | 0x0C | 0x0D | 0x0E | 0x0F  // A8R8G8B8, X8R8G8B8, DXT1/3/5
                            | 0x11 | 0x12                        // L8, A8L8
                            | 0x1A | 0x1D | 0x1E // LIN_A8R8G8B8 / LIN_DXT*
                    );
                    let data_sane = data_ptr != 0
                        && (data_ptr & 0x1FFF_FFFF) >= 0x1_0000
                        && (data_ptr & 0x1FFF_FFFF) < 0x2000_0000;
                    if data_sane && !ORGANIC_DIAG_LOGGED.swap(true, AO_DIAG::AcqRel) {
                        let mut peek = [0u8; 32];
                        unsafe {
                            let src = guest_mem.add((data_ptr & 0x1FFF_FFFF) as usize);
                            std::ptr::copy_nonoverlapping(src, peek.as_mut_ptr(), 32);
                        }
                        debug_log(&format!(
                            "[HLE] XGRPH-ORGANIC: data=0x{:08X} fmt=0x{:08X} (code=0x{:02X} sane={}) {}x{} \
                             first32=[{:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X} \
                             {:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X} \
                             {:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X} \
                             {:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X}]",
                            data_ptr, format, format_code, format_sane, tw, th,
                            peek[0], peek[1], peek[2], peek[3],
                            peek[4], peek[5], peek[6], peek[7],
                            peek[8], peek[9], peek[10], peek[11],
                            peek[12], peek[13], peek[14], peek[15],
                            peek[16], peek[17], peek[18], peek[19],
                            peek[20], peek[21], peek[22], peek[23],
                            peek[24], peek[25], peek[26], peek[27],
                            peek[28], peek[29], peek[30], peek[31],
                        ));
                    }
                    // Was previously gated by `injected_ok`. Now always
                    // false — fall through to the inline-Register
                    // resolve path which uses pBase + descriptor offset,
                    // matching real D3DResource8::Register semantics.
                    let injected_ok = false;
                    let _ = (data_sane, format_sane); // diagnostic-only

                    // Inline IDirect3DResource8::Register semantics:
                    //   pThis->Data = (pBase + pThis->Data_offset) | 0x80000000
                    // The "template_data_offset" is the original Data field
                    // of the descriptor (desc[1] is our best guess — the
                    // pDescOrData arg was previously assumed to be a desc,
                    // but Register-style ctors pass pBase directly as args[1]).
                    // 2026-04-24 EOS pBase override: args[1] points at the
                    // .data descriptor (vertex buffer content), not the
                    // real stash pixel data. When args[1] is in .data VA
                    // range, scan the LEGAL.xbs decompressed buffer at
                    // 0x00726F58 for DDS / XPR0 magic and point pBase at
                    // the discovered pixel data. Falls back to the raw
                    // 0x00726F58 base if no magic is found.
                    const STASH_BASE: u32 = 0x0072_6F58;
                    const STASH_LEN: usize = 32 * 1024; // 32KB scan window
                    const DDS_MAGIC: u32 = 0x2053_4444; // 'DDS '
                    const XPR0_MAGIC: u32 = 0x3052_5058; // 'XPR0'
                    const DDS_HEADER_SIZE: u32 = 0x80; // sizeof(DDS_HEADER)+magic
                    let mut p_base = if args[1] >= 0x003D_0000 && args[1] < 0x0040_0000 {
                        let mut found: u32 = STASH_BASE;
                        unsafe {
                            let scan_base = guest_mem.add(STASH_BASE as usize);
                            // Step by 4 bytes — both magics are 4-aligned in
                            // every Xbox archive seen so far.
                            let mut off = 0usize;
                            while off + 4 <= STASH_LEN {
                                let w = *((scan_base.add(off)) as *const u32);
                                if w == DDS_MAGIC {
                                    // Skip past 0x80-byte DDS header to the
                                    // first mip's pixel data.
                                    found = STASH_BASE
                                        .wrapping_add(off as u32)
                                        .wrapping_add(DDS_HEADER_SIZE);
                                    debug_log(&format!(
                                        "[HLE] XGRPH magic-scan: DDS @ 0x{:08X} -> pixels 0x{:08X}",
                                        STASH_BASE.wrapping_add(off as u32),
                                        found
                                    ));
                                    break;
                                }
                                if w == XPR0_MAGIC {
                                    // XPR0 header word at +0x08 holds the
                                    // pixel-data offset; if it looks sane,
                                    // honour it, else assume 0x2000 (default
                                    // XPR resource alignment).
                                    let hdr = STASH_BASE.wrapping_add(off as u32);
                                    let pix_off = *((scan_base.add(off + 8)) as *const u32);
                                    let pix = if pix_off > 0 && pix_off < 0x4000 {
                                        hdr.wrapping_add(pix_off)
                                    } else {
                                        hdr.wrapping_add(0x2000)
                                    };
                                    found = pix;
                                    debug_log(&format!(
                                        "[HLE] XGRPH magic-scan: XPR0 @ 0x{:08X} pix_off=0x{:X} -> pixels 0x{:08X}",
                                        hdr, pix_off, pix
                                    ));
                                    break;
                                }
                                off += 4;
                            }
                        }
                        found
                    } else {
                        args[1]
                    };
                    let _ = &mut p_base; // p_base used immutably below
                    let p_base = p_base;
                    let template_data_offset = desc[1];
                    // If we injected a real DDS, our overrides above have
                    // already written the correct header + registry info.
                    // Skip the inline-Register resolve (which would point
                    // at the stash scratch buffer instead of the DDS).
                    if injected_ok {
                        if let Some(tex_key) = normalize_guest_ram_ptr(fake) {
                            // DXT5 pitch = (block_width) * 16 bytes per block
                            let block_width = (tw + 3) / 4;
                            let pitch = block_width.saturating_mul(16);
                            upsert_texture_info(HleTextureInfo {
                                key: tex_key,
                                width: tw,
                                height: th,
                                format,
                                data: data_ptr,
                                pitch,
                                swizzled: false,
                            });
                        }
                    } else if p_base != 0
                        && (p_base & 0x1FFF_FFFF) < 0x2000_0000
                        && (p_base & 0x1FFF_FFFF) >= 0x1_0000
                    {
                        let resolved = ((p_base.wrapping_add(template_data_offset)) & 0x1FFF_FFFC)
                            | 0x8000_0000;
                        unsafe {
                            let data_field =
                                guest_mem.add(((fake & 0x1FFF_FFFC) + 4) as usize) as *mut u32;
                            *data_field = resolved;
                        }
                        debug_log(&format!(
                            "[HLE] XGRPH inline-Register: pBase=0x{:08X} \
                             off=0x{:08X} -> [fake+0x04]=0x{:08X}",
                            p_base, template_data_offset, resolved
                        ));
                        // Refresh data_ptr so the registry below stores the
                        // resolved physical address (masked, no |0x80000000).
                        let registry_data = resolved & 0x1FFF_FFFF;
                        if let Some(tex_key) = normalize_guest_ram_ptr(fake) {
                            let pitch = tw.saturating_mul(4);
                            upsert_texture_info(HleTextureInfo {
                                key: tex_key,
                                width: tw,
                                height: th,
                                format,
                                data: registry_data,
                                pitch,
                                swizzled: false,
                            });
                        }
                    } else if let Some(tex_key) = normalize_guest_ram_ptr(fake) {
                        // Fallback: no usable pBase — keep heuristic data_ptr.
                        let pitch = tw.saturating_mul(4);
                        upsert_texture_info(HleTextureInfo {
                            key: tex_key,
                            width: tw,
                            height: th,
                            format,
                            data: data_ptr,
                            pitch,
                            swizzled: false,
                        });
                    }
                    unsafe {
                        let pp = guest_mem.add(pp_texture_norm);
                        *(pp as *mut u32) = fake;
                    }
                    debug_log(&format!(
                        "[HLE] XGRPH_CreateTexture: pDesc=0x{:08X} fmt=0x{:X} \
                         desc[0..4]=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] \
                         data_ptr=0x{:08X} {}x{} -> fake=0x{:08X} @[0x{:08X}]",
                        p_desc,
                        format,
                        desc[0],
                        desc[1],
                        desc[2],
                        desc[3],
                        data_ptr,
                        tw,
                        th,
                        fake,
                        pp_texture
                    ));
                }
            }
            0 // D3D_OK
        }
        "D3D_MakeSpace" => {
            // Consume pushbuffer commands written since last MakeSpace/KickOff,
            // then reset PUT to base so the D3D driver has space to write more.
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                let dev_base = guest_mem as u64 + dev_ptr as u64;
                unsafe {
                    let put = *((dev_base + 0x00) as *const u32);
                    let get = *((dev_base + DEV_PB_GET as u64) as *const u32);

                    // Consume commands from GET to PUT
                    if put > get && put >= PB_DUMMY_BASE && get >= PB_DUMMY_BASE {
                        let result =
                            crate::xbox::aot::nv2a::consume_pushbuffer(guest_mem, get, put);
                        // Update global draw counters from pushbuffer parsing
                        static PB_DRAWS: std::sync::atomic::AtomicU64 =
                            std::sync::atomic::AtomicU64::new(0);
                        static PB_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let draws = result.draw_arrays + result.draw_elements + result.draw_inline;
                        if draws > 0 {
                            PB_DRAWS.fetch_add(draws as u64, Ordering::Relaxed);
                            OOVPA_DRAW_CALLS.fetch_add(draws as u64, Ordering::Relaxed);
                        }
                        OOVPA_PB_COMMANDS
                            .fetch_add(result.total_commands as u64, Ordering::Relaxed);
                        let n = PB_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 5 || (n % 1000 == 0) {
                            crate::xbox::emulator::debug_log(&format!(
                                "[PB-CONSUME] cmds={} draws={} (arrays={} elem={} inline={}) clears={} verts={} other={} total_draws={}",
                                result.total_commands, draws, result.draw_arrays, result.draw_elements,
                                result.draw_inline, result.clears, result.vertex_data, result.other,
                                PB_DRAWS.load(Ordering::Relaxed)
                            ));
                        }
                    }

                    // Reset pushbuffer: PUT back to base, GET = PUT
                    // After TAP, dev[0]=NV2A base (0xFD000000) — do NOT overwrite
                    if !super::oovpa_dispatch::tap_completed() {
                        *((dev_base + 0x00) as *mut u32) = PB_DUMMY_BASE;
                        *((dev_base + 0x04) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE - 0x100;
                    }
                    *((dev_base + DEV_PB_PUT as u64) as *mut u32) = PB_DUMMY_BASE;
                    *((dev_base + DEV_PB_GET as u64) as *mut u32) = PB_DUMMY_BASE;
                    return PB_DUMMY_BASE;
                }
            }
            PB_DUMMY_BASE
        }
        "CDevice_SetStateVB" => 0, // skip pre-draw state flush
        "D3DDevice_SetTexture" => {
            hle_set_texture(args, guest_mem);
            0
        }
        "D3DDevice_SetStreamSource" => {
            drain_draws_before_state_change(name);
            hle_set_stream_source(args, guest_mem);
            0
        }
        "D3DDevice_SetViewport" => {
            // SetViewport(pViewport) — copy D3DVIEWPORT8 struct into device
            let vp_ptr = args[0];
            if vp_ptr != 0 && vp_ptr < 0x2000_0000 {
                drain_draws_before_state_change(name);
                let dev_ptr = read_dev_ptr(guest_mem);
                if dev_ptr != 0 && dev_ptr < 0x2000_0000 {
                    unsafe {
                        let src = guest_mem as u64 + vp_ptr as u64;
                        let dst = guest_mem as u64 + dev_ptr as u64 + 0x09D0; // viewport at dev+0x09D0 (from disasm) // viewport offset in device
                                                                              // D3DVIEWPORT8: X, Y, Width, Height, MinZ, MaxZ = 24 bytes
                        std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 24);
                    }
                }
                static VP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let n = VP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let base = guest_mem as u64 + vp_ptr as u64;
                let (x, y, w, h, min_z, max_z) = unsafe {
                    (
                        *(base as *const u32),
                        *((base + 4) as *const u32),
                        *((base + 8) as *const u32),
                        *((base + 12) as *const u32),
                        *((base + 16) as *const f32),
                        *((base + 20) as *const f32),
                    )
                };
                cache_shader_viewport(x, y, w, h, min_z, max_z);
                if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
                    gpu.set_viewport(x, y, w, h, min_z, max_z);
                }
                if n < 5 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[HLE] SetViewport: {}x{}+{}+{} z=[{:.2},{:.2}]",
                        w, h, x, y, min_z, max_z
                    ));
                }
            }
            0
        }
        "D3DDevice_CreateTexture" => hle_create_texture(args, guest_mem),
        "D3DDevice_CreateTexture2" => hle_create_texture2(args, guest_mem),
        "D3DDevice_BlockUntilVerticalBlank" => 1, // return 1 (VBlank occurred)
        "D3DDevice_SetVerticalBlankCallback" => {
            // SetVerticalBlankCallback(pCallback) — register VBlank notification callback
            let cb = args[0];
            VBLANK_CALLBACK.store(cb, Ordering::Relaxed);
            let (_, vblank_cb_off) = d3d_device_callback_offsets();
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                unsafe {
                    *((guest_mem as u64 + dev_ptr as u64 + vblank_cb_off as u64) as *mut u32) = cb;
                }
            }
            debug_log(&format!(
                "[HLE] SetVerticalBlankCallback: cb=0x{:08X} dev=0x{:08X} off=0x{:X} updated={}",
                cb,
                dev_ptr,
                vblank_cb_off,
                dev_ptr != 0 && dev_ptr < 0x1000_0000
            ));
            0
        }
        "D3DDevice_SetSwapCallback" => {
            // SetSwapCallback(pCallback) — register swap notification callback
            let cb = args[0];
            SWAP_CALLBACK.store(cb, Ordering::Relaxed);
            let (swap_cb_off, _) = d3d_device_callback_offsets();
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                unsafe {
                    *((guest_mem as u64 + dev_ptr as u64 + swap_cb_off as u64) as *mut u32) = cb;
                }
            }
            debug_log(&format!(
                "[HLE] SetSwapCallback: cb=0x{:08X} dev=0x{:08X} off=0x{:X} updated={}",
                cb,
                dev_ptr,
                swap_cb_off,
                dev_ptr != 0 && dev_ptr < 0x1000_0000
            ));
            0
        }
        "D3DDevice_SetGammaRamp" => {
            // SetGammaRamp(Flags, pRamp). Host gamma is not exposed through the
            // current backend; consume the SDK call so games do not run the
            // statically-linked D3D body against our synthetic device object.
            static GAMMA_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = GAMMA_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 8 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE] SetGammaRamp: flags=0x{:08X} ramp=0x{:08X}",
                    args[0], args[1]
                ));
            }
            0
        }
        "D3DDevice_SetPixelShaderConstant" => hle_set_pixel_shader_constant(args, guest_mem),
        "D3DDevice_SetVertexShaderConstant" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "manual_hle",
                0,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            );
            hle_set_vertex_shader_constant(
                args,
                guest_mem,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            )
        }
        "D3DDevice_SelectVertexShader" | "D3DDevice_SelectVertexShaderDirect" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "manual_hle",
                0,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            )
        }
        "D3DDevice_SetVertexShaderInput" => {
            let ret = read_u32_guest(guest_mem, context.R14 as u32).unwrap_or(0);
            hle_trace_vertex_shader_call(
                name,
                args,
                guest_mem,
                "manual_hle",
                0,
                ret,
                context.R14 as u32,
                [
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                ],
            );
            hle_set_vertex_shader_input(args, guest_mem)
        }
        "D3DDevice_EndPush" => {
            // EndPush(pPush) — pPush is the current write cursor.
            // Update device PUT pointer to pPush so the game knows commands were submitted.
            let p_push = args[0];
            let dev_ptr = read_dev_ptr(guest_mem);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 && p_push != 0 {
                let dev_base = guest_mem as u64 + dev_ptr as u64;
                unsafe {
                    if super::oovpa_dispatch::tap_completed() {
                        *((dev_base + DEV_PB_PUT as u64) as *mut u32) = p_push;
                        *((dev_base + DEV_PB_GET as u64) as *mut u32) = p_push;
                    } else {
                        *((dev_base + 0x00) as *mut u32) = p_push;
                        *((dev_base + 0x04) as *mut u32) = p_push;
                    }
                }
            }
            // Splice 2026-04-20: drive the nv2a_pb parser on the range the
            // game just wrote. Use the pointer returned by BeginPush as the
            // source of truth; the scratch pushbuffer base is not fixed across
            // all paths.
            let (start, new_cursor, range_source) =
                if let Some(range) = begin_push_end_range(p_push) {
                    range
                } else if p_push > 0x0020_0000 {
                    (PB_DUMMY_BASE, p_push, "fallback-absolute")
                } else {
                    (PB_DUMMY_BASE, PB_DUMMY_BASE + p_push, "fallback-offset")
                };
            let tracked_start = BEGIN_PUSH_LAST_START.load(Ordering::Relaxed);
            let first_returned = tracked_start != 0 && start == tracked_start;
            let start = if first_returned && start < new_cursor {
                start
            } else {
                PB_DUMMY_BASE
            };
            static EP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let nn = EP_LOG.fetch_add(1, Ordering::Relaxed);
            if nn < 20 || nn.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-ENDPUSH #{}] arg=0x{:X} source={} begin=0x{:08X} new_cursor=0x{:08X} start=0x{:08X} delta={}",
                    nn,
                    p_push,
                    range_source,
                    tracked_start,
                    new_cursor,
                    start,
                    new_cursor.wrapping_sub(start)
                ));
            }
            // 2026-04-20 dump the pushbuffer contents for the first 3 EndPush
            // calls so we can decode the NV2A method classes.
            // Read via BOTH paths: base+phys and base+mirror (0x80000000 OR).
            // Write-combined writes on real Xbox go via the mirror; on our
            // host they land in the same RAM but we check both to be sure.
            if nn < 3 && new_cursor > start && new_cursor < 0x2000_0000 {
                let byte_count = new_cursor.wrapping_sub(start) as usize;
                let dword_count = (byte_count / 4).min(32);
                let base_host = guest_mem as u64;
                let phys_base = base_host + start as u64;
                let mirror_base = base_host + (0x8000_0000u64 + start as u64);
                let mut phys_hex = String::new();
                let mut mirror_hex = String::new();
                let mut any_nonzero = false;
                for i in 0..dword_count {
                    let wp = unsafe { *((phys_base + (i * 4) as u64) as *const u32) };
                    let wm = unsafe { *((mirror_base + (i * 4) as u64) as *const u32) };
                    if wp != 0 || wm != 0 {
                        any_nonzero = true;
                    }
                    if i > 0 {
                        phys_hex.push(' ');
                        mirror_hex.push(' ');
                    }
                    phys_hex.push_str(&format!("{:08X}", wp));
                    mirror_hex.push_str(&format!("{:08X}", wm));
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-ENDPUSH #{}] phys[0x{:08X}..+{}]: {}",
                    nn, start, dword_count, phys_hex
                ));
                if any_nonzero {
                    crate::xbox::emulator::debug_log(&format!(
                        "[HLE-ENDPUSH #{}] mirr[0x{:08X}..+{}]: {}",
                        nn,
                        0x8000_0000u32 | start,
                        dword_count,
                        mirror_hex
                    ));
                }
                // Also scan the whole 0x00A00000..0x00B00000 region for the
                // first non-zero dword — locates where guest actually writes.
                let scan_base = start;
                let scan_len = PB_DUMMY_SIZE.min(0x0010_0000);
                let mut first_nonzero_offset: Option<u32> = None;
                for off in (0..scan_len).step_by(4) {
                    let a = base_host + (scan_base as u64 + off as u64);
                    let w = unsafe { *(a as *const u32) };
                    if w != 0 {
                        first_nonzero_offset = Some(off);
                        break;
                    }
                }
                if let Some(off) = first_nonzero_offset {
                    let va = scan_base + off;
                    let w = unsafe { *((base_host + va as u64) as *const u32) };
                    crate::xbox::emulator::debug_log(&format!(
                        "[HLE-ENDPUSH #{}] first non-zero in tracked range 0x{:08X}..0x{:08X}: va=0x{:08X} val=0x{:08X}",
                        nn,
                        scan_base,
                        scan_base.saturating_add(scan_len),
                        va,
                        w
                    ));
                } else {
                    crate::xbox::emulator::debug_log(&format!(
                        "[HLE-ENDPUSH #{}] tracked range 0x{:08X}..0x{:08X} is ZERO",
                        nn,
                        scan_base,
                        scan_base.saturating_add(scan_len)
                    ));
                }
            }
            if new_cursor > start && new_cursor < 0x2000_0000 {
                maybe_arm_begin_push_overlay_blend(guest_mem, start, new_cursor);
                crate::xbox::aot::nv2a_pb::drive_range(guest_mem, start, new_cursor);
                advance_begin_push_cursor_to(new_cursor);
            }
            0
        }
        "SceneUpdate" => {
            // Scene update: stub to prevent null pointer cascade from empty scene graph.
            // The real function walks scene objects, updates transforms, queues draw calls.
            // With stub objects this crashes. Return 0 (no error).
            0
        }
        "SceneRender" => {
            // Scene render hook: instead of iterating null scene objects (crash),
            // draw a colored fullscreen quad to prove the render pipeline works.
            // On a real Xbox, this function calls DrawIndexedVertices with scene geometry.
            // We call our GPU backend directly.
            static SCENE_RENDER_COUNT: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = SCENE_RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            if n < 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[SCENE-RENDER] #{} ECX=0x{:08X} guest_mem=0x{:016X}",
                    n, args[0], guest_mem as u64
                ));
            }
            // Increment draw_calls counter (read by main thread HUD)
            {
                static DRAW_COUNT: std::sync::atomic::AtomicU64 =
                    std::sync::atomic::AtomicU64::new(0);
                DRAW_COUNT.fetch_add(1, Ordering::Relaxed);
            }
            // Write a colored rectangle to the guest framebuffer at 0x00F00000
            // (where hle_swap reads for guest→host blit). This puts pixels on screen.
            let fb_addr = 0x00F0_0000u64; // matches hle_swap pcrtc_start
            let fb_ptr = (guest_mem as u64 + fb_addr) as *mut u32;
            let width = 640usize;
            let height = 480usize;
            // Draw a gradient: red top, green bottom, blue sides
            for y in 0..height {
                for x in 0..width {
                    let r = ((y * 255) / height) as u32;
                    let g = (((height - y) * 255) / height) as u32;
                    let b = ((x * 255) / width) as u32;
                    unsafe {
                        *fb_ptr.add(y * width + x) = 0xFF000000 | (r << 16) | (g << 8) | b;
                    }
                }
            }
            // Verify write
            if n < 3 {
                let p0 = unsafe { *fb_ptr };
                let pc = unsafe { *fb_ptr.add(240 * 640 + 320) };
                crate::xbox::emulator::debug_log(&format!(
                    "[SCENE-RENDER] verify: pixel[0]=0x{:08X} pixel[center]=0x{:08X}",
                    p0, pc
                ));
            }
            0
        }
        "Get2DSurfaceDesc" | "D3D_GetSurfaceInfo" => {
            // stdcall(3): (pPixelContainer, Level, pDesc). ret 12.
            // Duplicated from execute_hle for manual hook routing.
            let p_surface = args[0];
            let out_ptr = args[2];
            let (w, h) = if let Some(p_surface_norm) = guest_ram_offset(p_surface) {
                let sp = unsafe { guest_mem.add(p_surface_norm) };
                let sw = unsafe { *(sp.add(0x1C) as *const u32) };
                let sh = unsafe { *(sp.add(0x20) as *const u32) };
                if sw > 0 && sw <= 4096 && sh > 0 && sh <= 4096 {
                    (sw, sh)
                } else {
                    (640, 480)
                }
            } else {
                (640, 480)
            };
            if let Some(out_norm) = guest_ram_offset(out_ptr) {
                let base = guest_mem as u64 + out_norm as u64;
                unsafe {
                    *((base + 0x00) as *mut u32) = 0x06; // format: X8R8G8B8
                    *((base + 0x04) as *mut u32) = 4; // bytes per pixel
                    *((base + 0x08) as *mut u32) = 0; // type: surface
                    *((base + 0x0C) as *mut u32) = w * 4; // pitch
                    *((base + 0x10) as *mut u32) = w * h * 4; // total size
                    *((base + 0x14) as *mut u32) = w; // width
                    *((base + 0x18) as *mut u32) = h; // height
                }
            }
            0 // D3D_OK
        }
        // ---- Classic Doom hooks ----
        "Doom_I_FinishUpdate" => {
            // Read Doom's 8-bit framebuffer, apply PLAYPAL palette, write to guest FB
            crate::xbox::worker::doom_finish_update_from_guest_mem(guest_mem);
            0
        }
        "Doom_I_WaitVBL" => {
            // VBlank wait — safe to skip, just timing
            0
        }
        "Doom_D3D_Swap" => {
            OOVPA_SWAP_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // Read Doom framebuffer on every Swap — this is I_FinishUpdate's D3D Present
            crate::xbox::worker::doom_finish_update_and_present_from_guest_mem(guest_mem);
            std::thread::yield_now();
            0
        }
        "Doom_D3D_DrawVerticesUP" => {
            hle_draw_vertices_up(args, guest_mem);
            OOVPA_DRAW_CALLS.fetch_add(1, Ordering::Relaxed);
            0
        }
        "Doom_D3D_SetTexture" => {
            hle_set_texture(args, guest_mem);
            0
        }
        "Doom_DSoundRelease" => 1, // Skip linked list traversal (fake buffer objects)
        n if n.starts_with("Doom_D3D_") && n != "Doom_D3D_Swap" && n != "Doom_D3D_MakeSpace" => {
            // All D3D internals: skip guest code (would crash on uninitialized NV2A state).
            // The software renderer writes directly to screens[0]; we read it in Swap HLE.
            0
        }
        "Doom_D3D_MakeSpace" => hle_reset_pushbuffer(guest_mem),
        "Doom_I_GetTime" => {
            // Return game tic count: 35 tics/sec based on host time
            static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
            static DIAG_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let start = START.get_or_init(std::time::Instant::now);
            let elapsed = start.elapsed().as_millis() as u32;
            let tics = elapsed * 35 / 1000;
            // Dump game state periodically
            let dc = DIAG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if dc < 5 || dc % 200 == 0 {
                let gm = guest_mem as u64;
                let gs_idx = unsafe { *((gm + 0x0010_1B9Cu64) as *const i32) };
                let gs = unsafe { *((gm + 0x0010_1BB8u64) as *const u32) };
                if gs != 0 && gs < 0x0800_0000 {
                    // Xbox Doom struct offsets differ from PC Doom:
                    // gamestate at 0x16C (not 0x2284), other offsets TBD
                    let gamestate = unsafe { *((gm + gs as u64 + 0x16Cu64) as *const i32) };
                    let gametic = unsafe { *((gm + gs as u64 + 0x271Cu64) as *const u32) };
                    let maketic = unsafe { *((gm + gs as u64 + 0x1F44u64) as *const u32) };
                    let render_flag = unsafe { *((gm + 0x000E_0568u64) as *const u8) };
                    let screen0 = unsafe { *((gm + gs as u64 + 0x21AF0u64) as *const u32) };
                    // Also read the old offset for comparison
                    let gs_old = unsafe { *((gm + gs as u64 + 0x2284u64) as *const i32) };
                    crate::xbox::emulator::debug_log(&format!(
                        "[DOOM-DIAG] #{} tics={} gs_idx={} gamestate@16C={} old@2284={} gametic={} maketic={} render={} screen0=0x{:08X}",
                        dc, tics, gs_idx, gamestate, gs_old, gametic, maketic, render_flag, screen0
                    ));
                } else {
                    crate::xbox::emulator::debug_log(&format!(
                        "[DOOM-DIAG] #{} tics={} gs_idx={} gs_ptr=0x{:08X} (invalid)",
                        dc, tics, gs_idx, gs
                    ));
                }
            }
            tics
        }
        "Doom_malloc" | "Doom_malloc2" => {
            // cdecl malloc: read size from [ESP+4] (first arg, not popped by us)
            let size = unsafe { *((guest_mem as u64 + (context.R14 as u64 + 4)) as *const u32) };
            let aligned = (size + 15) & !15;
            // Use bump allocator
            static DOOM_BUMP: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0x0110_0000);
            let ptr = DOOM_BUMP.fetch_add(aligned, std::sync::atomic::Ordering::Relaxed);
            if ptr + aligned >= 0x1000_0000 {
                0 // out of memory
            } else {
                // Zero the allocated memory
                if size > 0 && size < 0x100_0000 {
                    unsafe {
                        std::ptr::write_bytes(
                            (guest_mem as u64 + ptr as u64) as *mut u8,
                            0,
                            size as usize,
                        );
                    }
                }
                ptr
            }
        }
        "Doom_TryRunTics" => {
            // DO NOT HLE game logic — let guest code run TryRunTics natively.
            // This is game code, not OS. G_Ticker must run to process demos,
            // advance gamestate, and draw TITLEPIC via D_PageDrawer/V_DrawPatch.
            // If this was HLE'd, the game loop would spin without processing ticks.
            //
            // This handler should not be reached if the hook is removed from
            // the manual hooks list. If it fires, something is wrong.
            crate::xbox::emulator::debug_log(
                "[DOOM-BUG] TryRunTics HLE fired — should be TAP or removed!",
            );
            0
        }
        "Doom_R_Init_screen" => {
            // Let R_Init_screen run natively via TAP — D3D hooks handle surface setup.
            // Previously skipped because it crashed, but now our D3D HLE handles the
            // CreateDevice/Clear/Swap calls it makes internally.
            // Return 0 = success (D3D_OK). The actual screen setup (render target,
            // back buffer) is handled by CreateDevice HLE.
            0
        }
        "Doom_DirectSoundCreate" => {
            // Route to proper HLE that sets up fake IDirectSound8 + vtable stubs
            hle_dsound_create(args, guest_mem)
        }
        // ---- XPP USB driver stubs ----
        "XPP_Init" => 1,
        "XGetDevices" => {
            // XGetDevices (was XPP_Init at wrong address) — returns bitmask of connected devices.
            // Cxbx-R SymbolCache: XGetDevices at 0x37AD85 (was 0x37AD80 = 5 bytes before).
            xapi_hle_get_devices(guest_mem, args[0], "XGetDevices/manual")
        }
        "GameAssert" => {
            // Game's assertion/fatal error handler at 0x13E20.
            // cdecl varargs: (format_string, ...). 87 callers across the codebase.
            // Original code: vsprintf into stack buffer, then INT3 (debug break).
            // argc=0 (cdecl, caller cleans), so args[] is empty.
            // Read format string directly from guest stack: [R14+4] = first arg.
            let r14 = context.R14 as u32;
            let ret_addr = unsafe { *((guest_mem as u64 + r14 as u64) as *const u32) };
            let fmt_ptr = unsafe { *((guest_mem as u64 + (r14 + 4) as u64) as *const u32) };
            let arg1 = unsafe { *((guest_mem as u64 + (r14 + 8) as u64) as *const u32) };
            let arg2 = unsafe { *((guest_mem as u64 + (r14 + 12) as u64) as *const u32) };
            // Dump stack frames to find 0x47080's caller:
            // [R14+0x00]=ret_assert [R14+0x04]=fmt [R14+0x08]=len [R14+0x0C]=str
            // [R14+0x10]=saved_esi [R14+0x14]=saved_ebx [R14+0x18]=ret_of_0x47080_caller
            let caller_ret = unsafe { *((guest_mem as u64 + (r14 + 0x18) as u64) as *const u32) };
            // Also dump a wider stack window for complex frames
            let mut stack_dump = String::new();
            for off in (0..0x30).step_by(4) {
                let val = unsafe { *((guest_mem as u64 + (r14 + off) as u64) as *const u32) };
                if !stack_dump.is_empty() {
                    stack_dump.push(' ');
                }
                stack_dump.push_str(&format!("{:08X}", val));
            }
            // Accept pointers in low RAM, physical mirror, AND stack (0x1E000000+)
            if fmt_ptr > 0
                && (fmt_ptr < 0x2000_0000 || (fmt_ptr >= 0x8000_0000 && fmt_ptr < 0xA000_0000))
            {
                let read_guest_cstr_lossy = |ptr: u32, max_len: u32| -> (String, String) {
                    if ptr == 0
                        || !((ptr < 0x2000_0000) || (ptr >= 0x8000_0000 && ptr < 0xA000_0000))
                    {
                        return (String::new(), String::new());
                    }
                    let mut text = String::new();
                    let mut hex = String::new();
                    let base = guest_mem as u64 + ptr as u64;
                    for i in 0..max_len {
                        let b = unsafe { *((base + i as u64) as *const u8) };
                        if i != 0 {
                            hex.push(' ');
                        }
                        hex.push_str(&format!("{:02X}", b));
                        if b == 0 {
                            break;
                        }
                        if (0x20..0x7F).contains(&b) {
                            text.push(b as char);
                        } else {
                            text.push('.');
                        }
                    }
                    (text, hex)
                };
                let base = guest_mem as u64 + fmt_ptr as u64;
                let mut s = String::new();
                for i in 0..512u32 {
                    let b = unsafe { *((base + i as u64) as *const u8) };
                    if b == 0 {
                        break;
                    }
                    s.push(b as char);
                }
                // Try to resolve %s arguments
                let mut arg1_str = String::new();
                if arg1 > 0x10000 && arg1 < 0x1000_0000 && s.contains("%s") {
                    arg1_str = read_guest_cstr_lossy(arg1, 256).0;
                }
                // Try to also resolve arg2 as a string if %s is present twice or for additional context
                let mut arg2_str = String::new();
                if arg2 > 0x10000 && arg2 < 0x1000_0000 && s.contains("%s") {
                    arg2_str = read_guest_cstr_lossy(arg2, 256).0;
                }
                // Log with return address + caller chain + stack dump
                let arg_detail = if !arg1_str.is_empty() && !arg2_str.is_empty() {
                    format!("arg1=\"{}\" arg2=\"{}\"", arg1_str, arg2_str)
                } else if !arg2_str.is_empty() {
                    format!("arg1=0x{:08X}({}) arg2=\"{}\"", arg1, arg1 as i32, arg2_str)
                } else {
                    format!("arg1=0x{:08X}({}) arg2=0x{:08X}", arg1, arg1 as i32, arg2)
                };
                debug_log(&format!(
                    "[GAME-ASSERT] ret=0x{:08X} caller=0x{:08X} \"{}\" {} stack=[{}]",
                    ret_addr,
                    caller_ret,
                    s.trim(),
                    arg_detail,
                    stack_dump
                ));
                if s.contains("Character cannot be packed")
                    || s.contains("Couldn't open")
                    || s.contains("String [%s] too long")
                    || s.contains("Out of string buffers")
                    || s.contains("Char length:")
                    || s.contains("Unable to load font")
                    || s.contains("Bad token in font file")
                {
                    let (arg1_text, arg1_hex) = read_guest_cstr_lossy(arg1, 96);
                    let (arg2_text, arg2_hex) = read_guest_cstr_lossy(arg2, 96);
                    let string_init = unsafe { *((guest_mem as u64 + 0x004B_1DC4) as *const u8) };
                    let small_free = unsafe { *((guest_mem as u64 + 0x003F_8C30) as *const u32) };
                    let medium_free = unsafe { *((guest_mem as u64 + 0x0042_EFB8) as *const u32) };
                    let large_free = unsafe { *((guest_mem as u64 + 0x004B_1DC0) as *const u32) };
                    let hash_bucket0 = unsafe { *((guest_mem as u64 + 0x003F_8830) as *const u32) };
                    let hash_bucket_ff =
                        unsafe { *((guest_mem as u64 + 0x003F_8C2C) as *const u32) };
                    let describe_string_obj = |ptr: u32| -> String {
                        if ptr == 0
                            || !((ptr < 0x2000_0000) || (ptr >= 0x8000_0000 && ptr < 0xA000_0000))
                        {
                            return format!("ptr=0x{:08X}/invalid", ptr);
                        }
                        let base = guest_mem as u64 + ptr as u64;
                        let w0 = unsafe { *(base as *const u32) };
                        let w1 = unsafe { *((base + 4) as *const u32) };
                        let w2 = unsafe { *((base + 8) as *const u32) };
                        let w3 = unsafe { *((base + 12) as *const u32) };
                        let w4 = unsafe { *((base + 16) as *const u32) };
                        format!(
                            "ptr=0x{:08X} words={:08X}/{:08X}/{:08X}/{:08X}/{:08X}",
                            ptr, w0, w1, w2, w3, w4
                        )
                    };
                    debug_log(&format!(
                        "[GAME-ASSERT-DUMP] fmt=\"{}\" arg1=0x{:08X} text=\"{}\" hex=[{}] arg1_obj={} arg2=0x{:08X} text=\"{}\" hex=[{}] arg2_obj={} string_pool init={} free_small={} free_medium={} free_large={} hash0={} hashff={}",
                        s.trim(),
                        arg1,
                        arg1_text,
                        arg1_hex,
                        describe_string_obj(arg1),
                        arg2,
                        arg2_text,
                        arg2_hex,
                        describe_string_obj(arg2),
                        string_init,
                        small_free,
                        medium_free,
                        large_free,
                        hash_bucket0,
                        hash_bucket_ff
                    ));
                }
            } else {
                debug_log(&format!(
                    "[GAME-ASSERT] ret=0x{:08X} fmt_ptr=0x{:08X} (invalid) R14=0x{:08X}",
                    ret_addr, fmt_ptr, r14
                ));
            }
            0
        }
        _ => 0,
    }
}

/// CreateDevice: zero device struct, populate PB pointers, write g_pDevice
pub(crate) fn hle_create_device(guest_mem: *mut u8, mmio_count: &mut u64, guest_addr: u32) -> u32 {
    reset_begin_push_cursor();

    let dev_addr = 0x0030_0E00u32; // device struct location
    let dev_size = 0x2C00u32;

    // Dynamically discover g_pDevice address by scanning CreateDevice's guest code.
    // Look for "mov [imm32], reg" patterns that write a device struct pointer to a global.
    // Falls back to the static G_PDEVICE if nothing found.
    let g_pdevice_addr = {
        let mem = unsafe { std::slice::from_raw_parts(guest_mem, 0x0800_0000) };
        let func = guest_addr as usize;
        let mut found = 0u32;
        // Scan first 256 bytes of the function
        for off in 0..256usize {
            if func + off + 10 > mem.len() {
                break;
            }
            let b = mem[func + off];
            // mov [imm32], imm32 — C7 05 xx xx xx xx yy yy yy yy
            if b == 0xC7 && mem[func + off + 1] == 0x05 {
                let addr = u32::from_le_bytes([
                    mem[func + off + 2],
                    mem[func + off + 3],
                    mem[func + off + 4],
                    mem[func + off + 5],
                ]);
                let val = u32::from_le_bytes([
                    mem[func + off + 6],
                    mem[func + off + 7],
                    mem[func + off + 8],
                    mem[func + off + 9],
                ]);
                // g_pDevice should be in valid range and store a non-zero value
                if addr > 0x10000 && addr < 0x800000 && val != 0 {
                    found = addr;
                    break;
                }
            }
            // mov [imm32], reg — 89 1D/35/3D xx xx xx xx (mov [imm32], ebx/esi/edi)
            if b == 0x89
                && (mem[func + off + 1] == 0x1D
                    || mem[func + off + 1] == 0x35
                    || mem[func + off + 1] == 0x3D)
            {
                let addr = u32::from_le_bytes([
                    mem[func + off + 2],
                    mem[func + off + 3],
                    mem[func + off + 4],
                    mem[func + off + 5],
                ]);
                if addr > 0x10000 && addr < 0x800000 && found == 0 {
                    found = addr;
                    // Don't break — prefer C7 05 pattern
                }
            }
        }
        // Known-game overrides (dynamic scan can hit wrong C7 05 pattern)
        let result = match guest_addr {
            0x002F3BD0 => G_PDEVICE,   // Spider-Man XDK 4134
            0x00054C10 => 0x0005_FBC8, // Classic Doom XDK 5849
            _ if found != 0 => found,  // dynamic scan result
            _ => G_PDEVICE,            // fallback
        };
        debug_log(&format!(
            "[OOVPA-HLE] CreateDevice: g_pDevice=0x{:08X} (scan=0x{:08X}, guest=0x{:08X})",
            result, found, guest_addr
        ));
        result
    };

    // Store discovered g_pDevice for other hooks (Clear, Swap, etc.)
    G_PDEVICE_DYNAMIC.store(g_pdevice_addr, Ordering::Relaxed);

    // Zero device struct
    for i in (0..dev_size).step_by(4) {
        unsafe {
            *((guest_mem as u64 + (dev_addr + i) as u64) as *mut u32) = 0;
        }
    }

    // Write g_pDevice pointer to BOTH discovered and fallback locations
    unsafe {
        *((guest_mem as u64 + g_pdevice_addr as u64) as *mut u32) = dev_addr;
        if g_pdevice_addr != G_PDEVICE {
            *((guest_mem as u64 + G_PDEVICE as u64) as *mut u32) = dev_addr;
        }
    }

    // Set up dummy pushbuffer pointers
    let dev_base = guest_mem as u64 + dev_addr as u64;
    unsafe {
        *((dev_base + DEV_PB_PUT as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_GET as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_BASE as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_LIMIT as u64) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE;
    }

    // Set GLOBAL pushbuffer pointers at dev+0 (pPut) and dev+4 (pThreshold).
    // The D3D push-buffer write loop at 0x002EBE30 reads pPut from [dev+0]
    // and pThreshold from [dev+4] to decide when to call MakeSpace.
    // Unicorn found dev+0=0x83591178 and dev+4=0x8359537C (Cxbx-R garbage).
    // These MUST point into our pushbuffer region or MakeSpace loops forever.
    unsafe {
        // dev+0 = pPut cursor (where next command writes)
        *((dev_base + 0x00) as *mut u32) = PB_DUMMY_BASE;
        // dev+4 = pThreshold (when PUT >= threshold, call MakeSpace)
        *((dev_base + 0x04) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE - 0x100;
        // dev+8 = flags (0x13 from Unicorn: bit0=created, bit1=active, bit4=skip)
        *((dev_base + 0x08) as *mut u32) = 0x13;
    }
    // Re-write g_pDevice to discovered + fallback locations (after dev struct is populated)
    unsafe {
        *((guest_mem as u64 + g_pdevice_addr as u64) as *mut u32) = dev_addr;
        if g_pdevice_addr != G_PDEVICE {
            *((guest_mem as u64 + G_PDEVICE as u64) as *mut u32) = dev_addr;
        }
    }

    // Initialize D3D device internal state — Xbox D3D8 device struct layout
    // from Spider-Man XDK 4134 disassembly (xbox_scout_v2.db, 80 offsets extracted).
    //
    // CRITICAL: 0x2070-0x2084 is the SURFACE POINTER TABLE, not push buffer!
    // GetBackBuffer reads [device + index*4 + 0x207C] for back buffer surfaces.
    // Other D3D code reads [device+0x2070] as render target surface.
    unsafe {
        // ---- NV2A channel pointers (dev+0x0010 - dev+0x0028) ----
        // Used by 18 D3D functions. On real hardware these point to NV2A PGRAPH
        // DMA channel registers. In HLE mode, point to our NV2A PRAMIN region
        // (0xFD700000, PAGE_READWRITE) so reads/writes don't fault.
        let nv2a_pramin = 0xFD70_0000u32;
        *((dev_base + 0x0010) as *mut u32) = nv2a_pramin; // PGRAPH channel base (18 funcs read this!)
        *((dev_base + 0x0014) as *mut u32) = nv2a_pramin; // PGRAPH channel related (17 funcs)
        *((dev_base + 0x0018) as *mut u32) = nv2a_pramin; // NV2A channel (9 funcs)
        *((dev_base + 0x001C) as *mut u32) = nv2a_pramin; // NV2A / transform (7 funcs)
        *((dev_base + 0x0020) as *mut u32) = nv2a_pramin; // NV2A / multisampling (5 funcs)
        *((dev_base + 0x0024) as *mut u32) = nv2a_pramin; // NV2A channel (3 funcs)
        *((dev_base + 0x0028) as *mut u32) = nv2a_pramin; // NV2A / transform (3 funcs)

        // ---- Fence / busy state (dev+0x002C - dev+0x0058) ----
        // Cxbx-R ground truth: PUT=5, GET=NV2A addr, 0x0044=3
        *((dev_base + 0x002C) as *mut u32) = 0; // fence/busy counter
        *((dev_base + 0x0030) as *mut u32) = 5; // fence PUT (Cxbx-R: 5 after CreateDevice)
        *((dev_base + 0x0034) as *mut u32) = 0; // fence GET (completion)
        *((dev_base + 0x0038) as *mut u32) = 0; // SetFence / transform
        *((dev_base + 0x003C) as *mut u32) = 0; // SetFence / ShaderConstantMode
        *((dev_base + 0x0040) as *mut u32) = 0; // SetFence / EndPush
        *((dev_base + 0x0044) as *mut u32) = 3; // IsBusy / ShaderConstantMode (Cxbx-R: 3)
        *((dev_base + 0x0058) as *mut u32) = 0; // SetFence / CreateDevice

        // ---- Pushbuffer internal tracking (dev+0x0350-0x035C) ----
        *((dev_base + 0x0350) as *mut u32) = PB_DUMMY_BASE; // MakeSpace / GetPushBufferOffset
        *((dev_base + 0x0354) as *mut u32) = PB_DUMMY_BASE; // MakeSpace internal
        *((dev_base + 0x035C) as *mut u32) = PB_DUMMY_BASE; // EndPush PB tracking

        // ---- Pixel/vertex shader state (dev+0x0370-0x0394) ----
        *((dev_base + 0x0370) as *mut u32) = 0; // pixel shader program/state
        *((dev_base + 0x0374) as *mut u32) = 0; // SetPixelShader
                                                // 0x0380 = m_VertexShader (already set to 0 by memzero)
        *((dev_base + 0x0384) as *mut u32) = 0; // VS related
        *((dev_base + 0x0388) as *mut u32) = 0; // SelectVertexShader

        // ---- Ref count (dev+0x043C) ----
        *((dev_base + 0x043C) as *mut u32) = 1; // RefCount (AddRef starts at 1)

        // ---- Transform cache: 10 identity 4x4 matrices (dev+0x0750 - dev+0x09C8) ----
        // Cxbx-R ground truth: identity matrix at each slot (world, view, projection, etc.)
        // Each 4x4 matrix = 16 floats = 64 bytes. 10 matrices = 640 bytes.
        for i in 0..10u64 {
            let base_off = 0x0750u64 + i * 64;
            // Identity matrix: 1 on diagonal, 0 elsewhere
            for row in 0..4u64 {
                for col in 0..4u64 {
                    let val: f32 = if row == col { 1.0 } else { 0.0 };
                    *((dev_base + base_off + (row * 4 + col) * 4) as *mut f32) = val;
                }
            }
        }

        // ---- Projection matrix at dev+0x0790 ----
        // Already covered by identity matrix loop above (slot index 1)

        // ---- Viewport at dev+0x09D0 (NOT 0x2500!) ----
        // D3DVIEWPORT8: { X, Y, Width, Height, MinZ, MaxZ } = 24 bytes
        *((dev_base + 0x09D0) as *mut u32) = 0; // X
        *((dev_base + 0x09D4) as *mut u32) = 0; // Y
        *((dev_base + 0x09D8) as *mut u32) = 640; // Width
        *((dev_base + 0x09DC) as *mut u32) = 480; // Height
        *((dev_base + 0x09E0) as *mut f32) = 0.0; // MinZ
        *((dev_base + 0x09E4) as *mut f32) = 1.0; // MaxZ
        *((dev_base + 0x09E8) as *mut f32) = 0.0; // ScreenSpaceOffset.X
        *((dev_base + 0x09EC) as *mut f32) = 0.0; // ScreenSpaceOffset.Y

        // ---- Transform state (dev+0x0440-0x04A8) ----
        // Cxbx-R ground truth: identity matrices + projection constants
        // 0x0440-0x044C: 1.0f, 1.0f, 1.0f, large float (0x4B7FFFFF)
        *((dev_base + 0x0440) as *mut f32) = 1.0;
        *((dev_base + 0x0444) as *mut f32) = 1.0;
        *((dev_base + 0x0448) as *mut f32) = 1.0;
        *((dev_base + 0x044C) as *mut u32) = 0x4B7F_FFFF; // large float
                                                          // 0x0450: 0, 1.0, 1.0, 1.0
        *((dev_base + 0x0450) as *mut u32) = 0;
        *((dev_base + 0x0454) as *mut f32) = 1.0;
        *((dev_base + 0x0458) as *mut f32) = 1.0;
        *((dev_base + 0x045C) as *mut f32) = 1.0;
        // 0x0460: 0, 1.0, 1.0, 0
        *((dev_base + 0x0460) as *mut u32) = 0;
        *((dev_base + 0x0464) as *mut f32) = 1.0;
        *((dev_base + 0x0468) as *mut f32) = 1.0;
        *((dev_base + 0x046C) as *mut u32) = 0;
        // 0x0470: 320.0f (0x43A00000), zeros, ...
        *((dev_base + 0x0470) as *mut f32) = 320.0; // half-width
                                                    // 0x0480: -240.0f (0xC3700000)
        *((dev_base + 0x0480) as *mut f32) = -240.0; // neg half-height
                                                     // 0x0490: large float again
        *((dev_base + 0x0490) as *mut u32) = 0x4B7F_FFFF;
        // 0x04A0: 320.0, 240.0 (screen center)
        *((dev_base + 0x04A0) as *mut f32) = 320.0;
        *((dev_base + 0x04A4) as *mut f32) = 240.0;
        // 0x04A8: 0, 1.0
        *((dev_base + 0x04A8) as *mut u32) = 0;
        *((dev_base + 0x04AC) as *mut f32) = 1.0;

        // ---- RT dimensions (dev+0x0454-0x0458) ----
        // NOTE: These overlap transform state above. Cxbx-R shows these as 1.0f, 1.0f
        // which means RT width/height are actually at different offsets in this XDK version.
        // The disassembly-derived offsets 0x0454/0x0458 store transform floats, not pixel dims.
        // RT pixel dimensions are derived from viewport + surface format at runtime.

        // ---- Misc state from Cxbx-R ground truth ----
        *((dev_base + 0x2020) as *mut u32) = 3; // Cxbx-R: 3 (multisampling?)
        *((dev_base + 0x2078) as *mut u32) = 3; // Cxbx-R: 3 (surface count)

        // ---- Scissors (dev+0x21DC-0x2260) ----
        // Cxbx-R ground truth: scissors rect = viewport dimensions
        *((dev_base + 0x21DC) as *mut u32) = 0; // Scissors X
        *((dev_base + 0x21E0) as *mut u32) = 0; // Scissors Y
        *((dev_base + 0x21E4) as *mut u32) = 640; // Scissors Width
        *((dev_base + 0x21E8) as *mut u32) = 480; // Scissors Height
        *((dev_base + 0x225C) as *mut u32) = 1; // Scissors enabled (Cxbx-R: 1)

        // ---- IsBusy / EndPush / display (dev+0x2264-0x2268) ----
        *((dev_base + 0x2264) as *mut u32) = 0; // IsBusy / EndPush check
        *((dev_base + 0x2268) as *mut u32) = 0xFD00_0000; // NV2A base (FlickerFilter/SetTile/EndPush)

        // ---- Slot remap table (dev+0x2470 - dev+0x2A70) ----
        // Cxbx-R ground truth: 6 x 256-byte identity remap tables (0x00,0x01,...,0xFF)
        // Used for vertex attribute / texture stage slot mapping
        for table in 0..6u64 {
            let table_base = 0x2470u64 + table * 256;
            for j in 0..256u64 {
                *((dev_base + table_base + j) as *mut u8) = j as u8;
            }
        }
        // dev+0x2A7C = 1 (Cxbx-R ground truth)
        *((dev_base + 0x2A7C) as *mut u32) = 1;

        // ---- Callbacks / events (dev+0x2434+) ----
        // NOTE: disassembly shows BlockUntilVerticalBlank uses 0x2434 (not SymbolCache's 0x2428)
        *((dev_base + 0x2434) as *mut u32) = 0; // VBlank event handle
        *((dev_base + 0x2444) as *mut u32) = 0; // BlockOnTime target

        // ---- Display mode (dev+0x2450+) ----
        *((dev_base + 0x2450) as *mut u32) = 0x00050001; // NTSC 640x480

        // Display mode / surface format
        *((dev_base + 0x205C) as *mut u32) = 0x11; // D3DFMT_LIN_A8R8G8B8 (32bpp)
        *((dev_base + 0x2060) as *mut u32) = 640; // back buffer width
        *((dev_base + 0x2064) as *mut u32) = 480; // back buffer height
        *((dev_base + 0x2068) as *mut u32) = 640 * 4; // 2560 bytes/row (32bpp)

        // Render target / framebuffer base addresses
        let rt_addr = 0x00F0_0000u32; // 15MB mark — safe area in guest RAM
        *((dev_base + 0x2048) as *mut u32) = rt_addr;
        *((dev_base + 0x204C) as *mut u32) = rt_addr; // depth stencil base
        *((dev_base + 0x2050) as *mut u32) = 640 * 4;
        *((dev_base + 0x2054) as *mut u32) = 640 * 4;

        // ---- Allocate fake surface objects for the surface table ----
        // X_D3DPixelContainer layout: Common(4) Data(4) Lock(4) Format(4) Size(4) Parent(4) = 24 bytes
        let surf_size = 0x20u32;

        // Back buffer surface 0
        let bb0 = alloc_fake_d3d_obj(guest_mem, 0x80);
        if bb0 != 0 {
            let sp = guest_mem.add(bb0 as usize);
            *(sp as *mut u32) = 0x0001_0001; // Common: type=surface, refcount=1
            *(sp.add(0x04) as *mut u32) = rt_addr; // Data: framebuffer pixel data
            *(sp.add(0x08) as *mut u32) = 0; // Lock: unlocked
                                             // Format: 640x480 X8R8G8B8 linear (Xbox D3DFMT_LIN_A8R8G8B8 = 0x12)
                                             // Format field encoding: see Cxbx-R D3D8Types.h X_D3DFORMAT
            *(sp.add(0x0C) as *mut u32) = 0x00000012; // Format: LIN_A8R8G8B8
            *(sp.add(0x10) as *mut u32) = 640 * 480 * 4; // Size
                                                         // Extra fields used by Lock2DSurface:
            *(sp.add(0x1C) as *mut u32) = 640; // Width
            *(sp.add(0x20) as *mut u32) = 480; // Height
            *(sp.add(0x24) as *mut u32) = 640 * 4; // Pitch
            debug_log(&format!(
                "[HLE] CreateDevice: back buffer 0 surface at 0x{:08X}, data=0x{:08X}",
                bb0, rt_addr
            ));
        }

        // Back buffer surface 1 (front buffer)
        let bb1 = alloc_fake_d3d_obj(guest_mem, 0x80);
        if bb1 != 0 {
            let sp = guest_mem.add(bb1 as usize);
            *(sp as *mut u32) = 0x0001_0001;
            *(sp.add(0x04) as *mut u32) = rt_addr;
            *(sp.add(0x08) as *mut u32) = 0;
            *(sp.add(0x0C) as *mut u32) = 0x00000012;
            *(sp.add(0x10) as *mut u32) = 640 * 480 * 4;
            *(sp.add(0x1C) as *mut u32) = 640;
            *(sp.add(0x20) as *mut u32) = 480;
            *(sp.add(0x24) as *mut u32) = 640 * 4;
        }

        // Populate device surface table
        *((dev_base + 0x2070) as *mut u32) = bb0; // render target surface
        *((dev_base + 0x2074) as *mut u32) = bb0; // depth stencil (reuse RT for now)
        *((dev_base + 0x207C) as *mut u32) = bb0; // back buffer 0
        *((dev_base + 0x2080) as *mut u32) = bb1; // back buffer 1 (front buffer)
        *((dev_base + 0x2084) as *mut u32) = bb0; // back buffer 2 (alias)
        debug_log(&format!(
            "[HLE] CreateDevice: surface table populated: RT=0x{:08X} BB0=0x{:08X} BB1=0x{:08X}",
            bb0, bb0, bb1
        ));

        // NOTE: dev+0x08 is FLAGS (0x13 = created+active), set earlier. Do NOT overwrite here.
        // Surface count / back buffer count
        *((dev_base + 0x2044) as *mut u32) = 1;
        // (display mode at 0x2450 and NV2A at 0x2268 set above)
    }

    // ---- Initialize D3D render state arrays from Cxbx-R ground truth ----
    // D3D_g_RenderState at 0x003009B0: 1536 bytes, initial values after CreateDevice
    // D3D_g_DeferredTextureState at 0x003007B0: 1024 bytes, initial per-stage texture state
    unsafe {
        let base = guest_mem as u64;

        // D3D_g_RenderState: non-zero entries from Cxbx-R dump (offset in DWORDS from 0x003009B0)
        // Offsets 0x00E0-0x0440 contain render state defaults
        let rs_base = base + 0x003009B0u64;
        let rs_vals: &[(u32, u32)] = &[
            // offset, value (offset in bytes from array start)
            (0x00E4, 0x0203),
            (0x00E8, 0x0207),
            (0x00F8, 1),
            (0x0100, 1),
            (0x0108, 0x1D01),
            (0x010C, 0x01010101),
            (0x0110, 0x1E00),
            (0x0114, 0x1E00),
            (0x0118, 0x0207),
            (0x0120, 0xFFFFFFFF),
            (0x0124, 0xFFFFFFFF),
            (0x0128, 0x8006),
            (0x0130, 4),
            (0x0134, 0x80000000),
            (0x0138, 0x80000000),
            (0x0154, 0x3F800000),
            (0x0158, 0x3F800000), // 1.0f, 1.0f
            (0x0170, 1),
            (0x0178, 1),
            (0x017C, 1),
            (0x0180, 2),
            (0x0184, 1),
            (0x0190, 2),
            (0x0194, 1),
            (0x01A8, 0x3F800000), // 1.0f
            (0x01B4, 0x3F800000), // 1.0f
            (0x01C4, 0x42800000), // 64.0f
            (0x01CC, 0x3F800000), // 1.0f
            (0x01D0, 2),
            (0x01E0, 0x1B02),
            (0x01E4, 0x1B02),
            (0x01F0, 1),
            (0x01F8, 0x1E00),
            (0x01FC, 0x0900),
            (0x0200, 0x0901),
            (0x0204, 0xFFFFFFFF),
            (0x0214, 1),
            (0x0218, 0xFFFFFFFF),
            (0x0224, 0x0200),
            (0x0228, 0x3F800000),
            (0x022C, 1),
            (0x0234, 1),
            (0x0238, 1),
            (0x0248, 5),
            (0x024C, 1),
            (0x0250, 1),
            // Texture stage defaults: 0x034C onwards, every 16 bytes = 0x32 then 4x0x02
            (0x034C, 0x32),
            (0x035C, 2),
            (0x036C, 2),
            (0x037C, 2),
            (0x038C, 2),
            (0x039C, 2),
            (0x03AC, 2),
            (0x03BC, 2),
            (0x03CC, 2),
            (0x03DC, 2),
            (0x03EC, 2),
            (0x03FC, 2),
            (0x040C, 2),
            (0x041C, 2),
            (0x042C, 2),
            (0x043C, 2),
        ];
        for &(off, val) in rs_vals {
            *((rs_base + off as u64) as *mut u32) = val;
        }

        // D3D_g_DeferredTextureState: 4 texture stages, each 32 DWORDs (128 bytes)
        // Pattern from Cxbx-R: repeating per-stage block
        let dts_base = base + 0x003007B0u64;
        let stage_template: [u32; 32] = [
            1, 1, 1, 1, 1, 0, 0, 0, // states 0-7
            1, 0, 0, 0, 4, 1, 2, 1, // states 8-15 (stage 0 uses 4, others use 1)
            2, 1, 2, 1, 1, 0, 0, 0, // states 16-23
            0, 0, 0, 0, 0, 0, 0, 0, // states 24-31
        ];
        for stage in 0..4u64 {
            for i in 0..32u64 {
                let mut val = stage_template[i as usize];
                // Stage 0: state 12 = 4; stages 1-3: state 12 = 1
                if i == 12 && stage > 0 {
                    val = 1;
                }
                // Stage 3 (last): state 24 = 2, state 28 = 3
                if stage == 3 && i == 24 {
                    val = 2;
                }
                if stage == 3 && i == 28 {
                    val = 3;
                }
                *((dts_base + stage * 128 + i * 4) as *mut u32) = val;
            }
        }

        debug_log("[HLE] CreateDevice: render state + texture state arrays initialized from Cxbx-R ground truth");
    }

    // Hydrate CRT heap globals — _crtheap and __active_heap are 0 because the CRT's
    // _heap_init code path differs from what our kernel stubs expect. The process heap
    // at [0x7E18B0] IS valid (set by NtAllocateVirtualMemory during pool_init). Copy it
    // to _crtheap so the CRT malloc retry logic (_callnewh) works correctly.
    // __active_heap=3 selects the MSVC 7.0 heap manager (used by XDK 4134).
    unsafe {
        let base = guest_mem as u64;
        let process_heap = *((base + 0x007E_18B0u64) as *const u32);
        if process_heap != 0 {
            *((base + 0x007E_1584u64) as *mut u32) = process_heap; // _crtheap
            *((base + 0x007E_1C4Cu64) as *mut u32) = 3; // __active_heap
                                                        // Diagnostic: hex dump of NT HEAP header (Windows 2000 layout)
                                                        // +0x00: HEAP_ENTRY (8b), +0x08: Signature, +0x0C: Flags,
                                                        // +0x14: VirtualMemoryThreshold, +0x18: SegmentReserve,
                                                        // +0x1C: SegmentCommit, +0x28: TotalFreeSize, +0x2C: MaxAllocSize,
                                                        // +0x58: Segments[0] ptr → HEAP_SEGMENT
            let heap_base = base + process_heap as u64;
            let mut hex_line1 = String::new();
            let mut hex_line2 = String::new();
            for i in 0..16u64 {
                hex_line1.push_str(&format!("{:08X} ", *((heap_base + i * 4) as *const u32)));
            }
            for i in 16..32u64 {
                hex_line2.push_str(&format!("{:08X} ", *((heap_base + i * 4) as *const u32)));
            }
            crate::xbox::emulator::debug_log(&format!(
                "[HEAP-DUMP] base=0x{:08X} +0x00: {}",
                process_heap,
                hex_line1.trim()
            ));
            crate::xbox::emulator::debug_log(&format!(
                "[HEAP-DUMP] base=0x{:08X} +0x40: {}",
                process_heap,
                hex_line2.trim()
            ));
            // Xbox HEAP has 16-byte HEAP_ENTRY → signature at +0x10
            let sig = *((heap_base + 0x10) as *const u32);
            let flags = *((heap_base + 0x14) as *const u32);
            let virt_thresh = *((heap_base + 0x1C) as *const u32);
            let total_free = *((heap_base + 0x30) as *const u32);
            // Segments[0] at +0x60 (offset by 8 due to 16-byte HEAP_ENTRY)
            let seg0_ptr = *((heap_base + 0x60) as *const u32);
            crate::xbox::emulator::debug_log(&format!(
                "[OOVPA-HLE] Hydrated _crtheap=0x{:08X} sig=0x{:08X} flags=0x{:X} virt_thresh=0x{:X} total_free=0x{:X} seg0_ptr=0x{:08X}",
                process_heap, sig, flags, virt_thresh, total_free, seg0_ptr
            ));
            // Dump HEAP_SEGMENT structure (16-byte HEAP_ENTRY version)
            // +0x00: HEAP_ENTRY(16b), +0x10: Signature(0xFFEEFFEE), +0x14: Flags,
            // +0x18: Heap ptr, +0x1C: LargestUncommittedRange,
            // +0x20: BaseAddress, +0x24: NumberOfPages, +0x28: FirstEntry,
            // +0x2C: LastValidEntry, +0x30: NumberOfUncommittedPages
            if seg0_ptr >= process_heap && seg0_ptr < process_heap + 0x100000 {
                let seg_base = base + seg0_ptr as u64;
                let mut seg_hex = String::new();
                for i in 0..16u64 {
                    seg_hex.push_str(&format!("{:08X} ", *((seg_base + i * 4) as *const u32)));
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[HEAP-DUMP] seg0=0x{:08X}: {}",
                    seg0_ptr,
                    seg_hex.trim()
                ));
                // Key segment fields
                let seg_sig = *((seg_base + 0x10) as *const u32);
                let seg_heap = *((seg_base + 0x18) as *const u32);
                let seg_pages = *((seg_base + 0x24) as *const u32);
                let first_entry = *((seg_base + 0x28) as *const u32);
                let last_valid = *((seg_base + 0x2C) as *const u32);
                let uncommitted = *((seg_base + 0x30) as *const u32);
                crate::xbox::emulator::debug_log(&format!(
                    "[HEAP-DUMP] seg0 sig=0x{:08X} heap=0x{:08X} pages={} first=0x{:08X} last=0x{:08X} uncommitted={}",
                    seg_sig, seg_heap, seg_pages, first_entry, last_valid, uncommitted
                ));
                // Scan ALL 128 dedicated FreeLists + FreeLists[0] for corruption
                // FreeLists[0] at heap+0x180, FreeLists[1] at +0x188, ...
                // Each is a LIST_ENTRY (8 bytes): Flink, Blink
                let mut corrupt_count = 0u32;
                let mut nonempty_count = 0u32;
                for i in 0..128u32 {
                    // NT heap has exactly 128 FreeLists entries (0..127)
                    let off = 0x180u64 + i as u64 * 8;
                    let flink = *((heap_base + off) as *const u32);
                    let blink = *((heap_base + off + 4) as *const u32);
                    let self_addr = process_heap + off as u32;
                    let is_empty = flink == self_addr && blink == self_addr;
                    if is_empty {
                        continue;
                    }
                    nonempty_count += 1;
                    // Check for corruption: NULL pointers, mismatched, or out-of-range
                    let bad_flink = flink == 0 || (flink < process_heap && flink != self_addr);
                    let bad_blink = blink == 0 || (blink < process_heap && blink != self_addr);
                    if bad_flink || bad_blink {
                        corrupt_count += 1;
                        crate::xbox::emulator::debug_log(&format!(
                            "[HEAP-CORRUPT] FreeLists[{}] +0x{:03X}: flink=0x{:08X} blink=0x{:08X} BAD:{}{}",
                            i, off, flink, blink,
                            if bad_flink { " flink" } else { "" },
                            if bad_blink { " blink" } else { "" }
                        ));
                    } else {
                        crate::xbox::emulator::debug_log(&format!(
                            "[HEAP-FL] FreeLists[{}] +0x{:03X}: flink=0x{:08X} blink=0x{:08X} OK",
                            i, off, flink, blink
                        ));
                    }
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[HEAP-SCAN] {} non-empty freelists, {} corrupted",
                    nonempty_count, corrupt_count
                ));
                // Walk FreeLists[0] at heap+0x180 — dump first 5 free blocks
                // Each free block: HEAP_ENTRY (16 bytes) then LIST_ENTRY at +0x10
                // So flink points to the LIST_ENTRY inside the block, HEAP_ENTRY is at flink-16
                let fl0_flink = *((heap_base + 0x180) as *const u32);
                let fl0_self = process_heap + 0x180;
                if fl0_flink != 0 && fl0_flink != fl0_self {
                    let mut cursor = fl0_flink;
                    for i in 0..8u32 {
                        if cursor == fl0_self || cursor == 0 {
                            break;
                        }
                        let cursor_base = base + cursor as u64;
                        let next_flink = *(cursor_base as *const u32);
                        // Read Size/PrevSize/Flags at -8 (standard NT fields in 16-byte entry)
                        let size_u16 = *((cursor_base - 8) as *const u16);
                        let prev_u16 = *((cursor_base - 6) as *const u16);
                        let flags_byte = *((cursor_base - 3) as *const u8);
                        let size_bytes = size_u16 as u32 * 16;
                        crate::xbox::emulator::debug_log(&format!(
                            "[HEAP-FREE] #{} entry=0x{:08X} Size={} ({}B) Prev={} Flags=0x{:02X} flink→0x{:08X}",
                            i, cursor.wrapping_sub(16), size_u16, size_bytes, prev_u16, flags_byte, next_flink
                        ));
                        cursor = next_flink;
                    }
                } else {
                    crate::xbox::emulator::debug_log("[HEAP-FREE] FreeLists[0] is empty");
                }
                // Dump first BUSY entry right after segment header to verify entry layout
                // Segment first_entry is at seg0+0x28
                let seg0_ptr = *((heap_base + 0x60) as *const u32);
                let first_entry = *((base + seg0_ptr as u64 + 0x28) as *const u32);
                if first_entry != 0 {
                    let fe_base = base + first_entry as u64;
                    let raw: [u32; 8] = [
                        *((fe_base) as *const u32),
                        *((fe_base + 4) as *const u32),
                        *((fe_base + 8) as *const u32),
                        *((fe_base + 12) as *const u32),
                        *((fe_base + 16) as *const u32),
                        *((fe_base + 20) as *const u32),
                        *((fe_base + 24) as *const u32),
                        *((fe_base + 28) as *const u32),
                    ];
                    crate::xbox::emulator::debug_log(&format!(
                        "[HEAP-BUSY] first_entry=0x{:08X} raw[0..32]: {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
                        first_entry, raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7]
                    ));
                    // Read Size at offset+8 (matching free block pattern)
                    let busy_size = *((fe_base + 8) as *const u16);
                    let busy_prev = *((fe_base + 10) as *const u16);
                    let busy_flags = *((fe_base + 13) as *const u8);
                    crate::xbox::emulator::debug_log(&format!(
                        "[HEAP-BUSY] Size={} ({}B) Prev={} Flags=0x{:02X} (expect 0x01=BUSY)",
                        busy_size,
                        busy_size as u32 * 16,
                        busy_prev,
                        busy_flags
                    ));
                }
            }
        }
    }

    // Null out render-state sort callbacks — the game's custom sort callback at
    // [0x006F2CBC] calls into BST code (sub_0001CAA0) that reads uninitialized tree
    // roots, causing AV at 0x0001CA98. With these zeroed, sub_0029D5B0 takes its
    // D3D-default fallback path instead. See spider-man.md blocker #2.
    unsafe {
        let base = guest_mem as u64;
        *((base + 0x006F_2CBCu64) as *mut u32) = 0; // custom sort callback
        *((base + 0x006F_2CCCu64) as *mut u32) = 0; // secondary sort callback
        *((base + 0x006F_2CD0u64) as *mut u32) = 0; // tertiary sort callback
    }

    *mmio_count += 1; // CreateDevice counts as MMIO activity (Stage 8 trigger)

    // Inject a render callback into the allocated render context.
    // The render pipeline at 0x00297A94 iterates [render_ctx+0x114] linked list.
    // Without callbacks, the game clears+swaps empty frames. With a callback,
    // the pipeline calls DrawIndexedVertices through a known call site.
    let render_ctx = unsafe { *((guest_mem as u64 + 0x003F_10E8u64) as *const u32) };
    if render_ctx != 0 && render_ctx < 0x1000_0000 {
        // Zero the lists first to prevent garbage pointer walks
        unsafe {
            *((guest_mem as u64 + render_ctx as u64 + 0x108) as *mut u32) = 0;
            *((guest_mem as u64 + render_ctx as u64 + 0x10C) as *mut u32) = 0;
            *((guest_mem as u64 + render_ctx as u64 + 0x11C) as *mut u32) = 0;
            *((guest_mem as u64 + render_ctx as u64 + 0x120) as *mut u32) = 0;
        }
        // Build a callback entry at a safe address (0x01F01000)
        // Callback struct: +0x00=next, +0x14=func_ptr, +0x18=arg
        let cb = 0x01F0_1000u32;
        unsafe {
            let base = guest_mem as u64;
            *((base + cb as u64 + 0x00) as *mut u32) = 0; // next = NULL
            *((base + cb as u64 + 0x14) as *mut u32) = 0x0029_817D; // DrawIndexedVertices caller
            *((base + cb as u64 + 0x18) as *mut u32) = 0; // arg
                                                          // Link into render_ctx callback lists
            *((base + render_ctx as u64 + 0x114) as *mut u32) = cb;
            *((base + render_ctx as u64 + 0x118) as *mut u32) = 0; // second list empty
        }
        debug_log(&format!(
            "[OOVPA-HLE] CreateDevice: injected draw callback at [0x{:08X}+0x114] -> 0x{:08X} -> fn 0x0029817D",
            render_ctx, cb
        ));
    }

    // Request GPU init on main thread (D3D11 device creation conflicts with VEH on worker)
    crate::xbox::gpu::request_gpu_init();
    debug_log(&format!(
        "[OOVPA-HLE] CreateDevice: dev=0x{:08X} g_pDevice=0x{:08X} (dynamic) PB=0x{:08X} (GPU init requested)",
        dev_addr, g_pdevice_addr, PB_DUMMY_BASE
    ));
    0 // D3D_OK
}

/// Swap: present frame. Reads back from D3D9 backend and signals main thread.
pub(crate) fn hle_swap(guest_mem: *mut u8, mmio_count: &mut u64) -> u32 {
    hle_swap_with_flags(guest_mem, mmio_count, X_D3DSWAP_DEFAULT)
}

/// Xbox D3DDevice::Swap(DWORD Flags). COPY/BYPASSCOPY and FINISH are
/// distinct phases on Xbox, so this cannot be reduced to PC-style Present().
pub(crate) fn hle_swap_with_flags(
    guest_mem: *mut u8,
    mmio_count: &mut u64,
    swap_flags: u32,
) -> u32 {
    // 2026-04-24 profiling: snapshot event counters at entry so we can
    // report the per-swap delta on exit. Every 100th swap logs a one-liner
    // showing how many VEH/MMIO/PB events fired during this swap's work.
    // This tells us which VEH path dominates the 60,000× slowdown so a
    // targeted emitter fix can kill the hot path.
    use std::sync::atomic::Ordering;
    let snap_swap_n = crate::xbox::aot::oovpa::OOVPA_SWAP_COUNT.load(Ordering::Relaxed);
    let snap_mmio = crate::xbox::aot::oovpa::OOVPA_MMIO_COUNT.load(Ordering::Relaxed);
    let snap_pb = crate::xbox::aot::oovpa::OOVPA_PB_COMMANDS.load(Ordering::Relaxed);
    let snap_hooks = crate::xbox::aot::oovpa::OOVPA_HOOKS_FIRED.load(Ordering::Relaxed);
    let snap_nv2a_draws = crate::xbox::aot::nv2a_pb::NV2A_DRAW_METHODS.load(Ordering::Relaxed);
    let snap_ts = std::time::Instant::now();

    // Keep this tag out of the high-frequency HLE-SWAP filter so manual/menu
    // runs tell us whether the hooked Swap layer is receiving Xbox flags.
    {
        static SWAP_FLAGS_EARLY_LOG: AtomicU32 = AtomicU32::new(0);
        let n = SWAP_FLAGS_EARLY_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 64 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[SWAP-FLAGS-EARLY #{}] swap={} flags=0x{:08X} default={} copy={} bypass={} finish={}",
                n,
                snap_swap_n,
                swap_flags,
                swap_flags == X_D3DSWAP_DEFAULT,
                (swap_flags & X_D3DSWAP_COPY) != 0,
                (swap_flags & X_D3DSWAP_BYPASSCOPY) != 0,
                (swap_flags & X_D3DSWAP_FINISH) != 0
            ));
        }
    }

    // 2026-04-20: Ensure GPU backend is initialized. CreateDevice runs in TAP mode,
    // so hle_create_device's request_gpu_init() never fires — we'd forever read
    // the empty guest framebuffer instead of the host-rendered one.
    // Phase 41 (86bf789) wired the end-to-end Clear/Swap/readback/G_VIDEO
    // pipeline; this re-arms it from the Swap entry point.
    crate::xbox::gpu::request_gpu_init();
    doom_log_swap_pipeline(swap_flags, guest_mem);

    if crate::xbox::worker::doom_finish_update_and_present_from_guest_mem(guest_mem) {
        static DOOM_SWAP_PRESENT_LOG: AtomicU32 = AtomicU32::new(0);
        let n = DOOM_SWAP_PRESENT_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[DOOM-SWAP-PRESENT] #{} flags=0x{:08X} published software framebuffer",
                n, swap_flags
            ));
        }
        *mmio_count += 1;
        return 0;
    }

    // Increment device frame counter
    let dev_ptr = read_dev_ptr(guest_mem);
    if dev_ptr != 0 {
        let frame_addr = guest_mem as u64 + dev_ptr as u64 + DEV_FRAME_CTR as u64;
        let frame = unsafe { *(frame_addr as *const u32) };
        unsafe {
            *(frame_addr as *mut u32) = frame + 1;
        }
        // Splice from JIT 2026-04-20: parse the pushbuffer THIS frame wrote
        // BEFORE resetting PB_PUT to base. Spider-Man bypasses MMIO USER_PUT
        // writes by going through the SDK D3DDevice_KickOff → Swap path —
        // our hle_swap HLE intercepts it, so veh_mmio's drive-on-PUT-write
        // never triggers. Capture the [PB_BASE..PB_PUT) range here, feed
        // the parser, THEN reset.
        // The DEV_PB_PUT/DEV_PB_BASE offsets in the device struct are INTERNAL
        // tracking fields (not the true Xbox CDevice layout) and have been
        // observed holding junk (pb_base=0x8B8A8988). Use the NV2A shadow
        // USER_PUT register instead, which is the authoritative last-seen
        // guest PUT value. pb_base derives from the allocation page
        // (page-aligned down by 64KB since pushbuffers are typically 64KB+).
        let pb_put = super::super::nv2a::shadow_read(0x80_0040);
        let pb_base = pb_put & 0xFFFF_0000; // 64KB page alignment floor
        static LAST_SHADOW_PUT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let prev_put = LAST_SHADOW_PUT.swap(pb_put, std::sync::atomic::Ordering::Relaxed);
        // Diagnostic: see what PB state each Swap observes.
        {
            static SWAP_PB_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let sn = SWAP_PB_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if sn < 10 || sn.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[SWAP-PB #{}] dev=0x{:08X} shadow_put=0x{:08X} prev_shadow=0x{:08X} derived_base=0x{:08X}",
                    sn, dev_ptr, pb_put, prev_put, pb_base
                ));
            }
        }
        // One-shot probe: dump 64 dwords at pb_base so we can see if the game
        // is actually writing NV2A commands to the hardware PB region, or if
        // the writes are going somewhere else entirely (BeginPush out-param
        // ignored, or a different base address from the SDK trampoline).
        {
            static PB_PROBE_ONCE: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !PB_PROBE_ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                let mut hex_phys = String::new();
                let mut any_nz = false;
                for i in 0..64 {
                    let a = guest_mem as u64 + pb_base as u64 + (i * 4) as u64;
                    let w = unsafe { *(a as *const u32) };
                    if w != 0 {
                        any_nz = true;
                    }
                    if i > 0 {
                        hex_phys.push(' ');
                    }
                    hex_phys.push_str(&format!("{:08X}", w));
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[PB-PROBE] pb_base=0x{:08X} pb_put=0x{:08X} first 64 dwords (nonzero={}): {}",
                    pb_base, pb_put, any_nz, hex_phys
                ));
                // Also scan 0x00A00000..0x00E00000 in 4KB steps for the first
                // page containing NV2A-command-like dwords (method≠0 in bits 12:2).
                for page in (0x00A0_0000u32..0x00E0_0000u32).step_by(0x1000) {
                    let mut hits = 0u32;
                    let mut first_nz_dw: Option<u32> = None;
                    for off in (0..0x1000u32).step_by(4) {
                        let w = unsafe {
                            *((guest_mem as u64 + page as u64 + off as u64) as *const u32)
                        };
                        if w != 0 {
                            if first_nz_dw.is_none() {
                                first_nz_dw = Some(w);
                            }
                            // Looks like an NV2A command if bits [1:0]==0 and
                            // method (bits 12:2) is a known NV097_ offset.
                            let mth = w & 0x1FFC;
                            if (w & 3) == 0
                                && (mth == 0x0100
                                    || mth == 0x1B80
                                    || mth == 0x17FC
                                    || mth == 0x1810
                                    || mth == 0x1818
                                    || mth == 0x0800
                                    || mth == 0x1900
                                    || mth == 0x1908)
                            {
                                hits += 1;
                            }
                        }
                    }
                    if hits >= 3 {
                        crate::xbox::emulator::debug_log(&format!(
                            "[PB-SCAN-HIT] page=0x{:08X} nv2a-like-hits={} first_nz=0x{:08X}",
                            page,
                            hits,
                            first_nz_dw.unwrap_or(0)
                        ));
                    }
                }
            }
        }
        // Parse the range the game populated between prev_put and current put.
        // For first call (prev_put == 0), use pb_base as the start.
        let start = if prev_put != 0 && prev_put < pb_put {
            prev_put
        } else {
            pb_base
        };
        if pb_put > start && pb_put < 0x2000_0000 {
            crate::xbox::aot::nv2a_pb::drive_range(guest_mem, start, pb_put);
        }
        // ALSO parse the page AT and AFTER pb_put. 2026-04-20 PB-PROBE showed
        // [pb_base..pb_put) is ZERO but [pb_put..pb_put+4KB) has 18 NV2A-like
        // command dwords. The game's SDK BeginPush/EndPush path advances its
        // internal cursor dev->m_pPBPut but may not re-kick USER_PUT via MMIO,
        // so shadow_put stays pinned at an early value while real commands
        // accumulate past it. Bound the parse at the first zero dword or 16KB,
        // whichever comes first (prevents walking into unrelated pool memory).
        if pb_put != 0 && pb_put < 0x2000_0000 {
            let scan_base = pb_put;
            let mut scan_end = scan_base;
            let mut zero_run = 0u32;
            for i in 0..(16u32 * 1024 / 4) {
                let addr = scan_base + i * 4;
                if addr >= 0x2000_0000 {
                    break;
                }
                let w = unsafe { *((guest_mem as u64 + addr as u64) as *const u32) };
                if w == 0 {
                    zero_run += 1;
                    if zero_run >= 16 {
                        break;
                    } // 16 consecutive zeros = likely end
                } else {
                    zero_run = 0;
                    scan_end = addr + 4;
                }
            }
            if scan_end > scan_base {
                static POSTPUT_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = POSTPUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 5 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[PB-POSTPUT #{}] parsing [0x{:08X}..0x{:08X}) ({} bytes)",
                        n,
                        scan_base,
                        scan_end,
                        scan_end - scan_base
                    ));
                }
                crate::xbox::aot::nv2a_pb::drive_range(guest_mem, scan_base, scan_end);
            }
        }
        // Reset PB PUT to base
        unsafe {
            *((guest_mem as u64 + dev_ptr as u64 + DEV_PB_PUT as u64) as *mut u32) = pb_base;
        }
    }
    // Drain the nv2a_pb vertex queue + readback under a single gpu_lock.
    // Order: drain (replays queued draw batches into backend) BEFORE
    // readback_framebuffer so the presented frame contains THIS pass's
    // geometry. Splice from JIT 2026-04-20.
    let mut did_present_frame = false;
    {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            hle_flush_deferred_render_target_to_backend(backend.as_mut());
            let n_drained = crate::xbox::gpu::drain_draw_queue(backend.as_mut());
            if n_drained > 0 {
                static DRAIN_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let k = DRAIN_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if k < 8 || k.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[hle_swap] drained {} queued draws",
                        n_drained
                    ));
                }
            }
            // 2026-04-24: force-test PROVED backend works (green triangle rendered).
            // Removed. The stall is now proven to be Spider-Man's vertex data
            // producing degenerate/zero-area geometry that rasterizes to no
            // fragments. Next investigation: nv2a_pb.rs vertex decode.
            let active_rt = backend.debug_active_render_target();
            let pcrtc_start = crate::xbox::aot::nv2a::shadow_read(0x60_0800);
            let copy_phase = (swap_flags & (X_D3DSWAP_COPY | X_D3DSWAP_BYPASSCOPY)) != 0
                && (swap_flags & X_D3DSWAP_FINISH) == 0;
            let finish_phase = (swap_flags & X_D3DSWAP_FINISH) != 0;
            let pending_before = XBOX_SWAP_COPY_PENDING.load(Ordering::Relaxed);
            let mut resolved = false;
            let mut front_bound = false;
            let mut restored_backbuffer = false;
            let black_clear_without_draw = backend.black_clear_without_draw_pending();
            let pixels = if copy_phase {
                if (swap_flags & X_D3DSWAP_COPY) != 0 {
                    resolved = backend.copy_backbuffer_to_display(pcrtc_start);
                }
                front_bound = backend.bind_display_as_render_target(pcrtc_start);
                XBOX_SWAP_COPY_PENDING.store(true, Ordering::Relaxed);
                None
            } else {
                if finish_phase {
                    if (swap_flags & X_D3DSWAP_COPY) != 0 {
                        resolved = backend.copy_backbuffer_to_display(pcrtc_start);
                    }
                    let frame = backend.present_display(pcrtc_start).to_vec();
                    backend.restore_backbuffer_render_target();
                    XBOX_SWAP_COPY_PENDING.store(false, Ordering::Relaxed);
                    restored_backbuffer = true;
                    Some(frame)
                } else {
                    // Ordinary Swap(0) presents through the scanout-aware path.
                    // The HLE SetRenderTarget bridge keeps PCRTC_START linked to
                    // the logical backbuffer, so this no longer blackholes the
                    // menu by reading the stale 0x03C00000 bootstrap surface.
                    Some(backend.present_framebuffer().to_vec())
                }
            };
            let capture_every_swap = std::env::var_os("RUSTEMU_TEST_AUTOPRESS_START").is_some()
                || std::env::var_os("RUSTEMU_CAPTURE_EVERY_SWAP").is_some();
            let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) = pixels
                .as_ref()
                .map(|p| sampled_argb_stats(p))
                .unwrap_or((0, 0, 0, 0, 0));
            let legal_or_title_overlay = pixels
                .as_ref()
                .map(|p| has_legal_like_bright_text_block(p))
                .unwrap_or(false);
            let (scene_name_for_present, stash_name_for_present) = {
                fn read_scene_string(mem: *mut u8, addr: u32, max_len: usize) -> String {
                    let mut bytes = Vec::with_capacity(max_len.min(64));
                    for i in 0..max_len {
                        let b = unsafe { *((mem as u64 + addr as u64 + i as u64) as *const u8) };
                        if b == 0 {
                            break;
                        }
                        if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                            bytes.push(b);
                        } else {
                            break;
                        }
                    }
                    String::from_utf8_lossy(&bytes).into_owned()
                }
                (
                    read_scene_string(guest_mem, 0x004B_C848, 64),
                    read_scene_string(guest_mem, 0x004B_C948, 64),
                )
            };
            let hold_bonus_menu_black_frame = pixels.is_some()
                && swap_flags == X_D3DSWAP_DEFAULT
                && scene_name_for_present.eq_ignore_ascii_case("bonus\\menu")
                && stash_name_for_present.eq_ignore_ascii_case("M0menu\\menu")
                && (sample_rgb == 0 || black_clear_without_draw)
                && sample_alpha != 0;
            let bonus_menu_active = pixels.is_some()
                && swap_flags == X_D3DSWAP_DEFAULT
                && scene_name_for_present.eq_ignore_ascii_case("bonus\\menu")
                && stash_name_for_present.eq_ignore_ascii_case("M0menu\\menu");
            let full_white_transition =
                sample_rgb == 1024 && first_px == 0xFFFF_FFFF && mid_px == 0xFFFF_FFFF;
            let red_web_frame = pixels
                .as_ref()
                .map(|p| is_spidey_red_web_like_frame(p, sample_rgb, first_px, mid_px))
                .unwrap_or(false);
            let selector_seen = SPIDEY_MENU_SELECTOR_SEEN.load(Ordering::Relaxed);
            let enable_bonus_menu_composite = std::env::var_os("RUSTEMU_SPIDEY_MENU_COMPOSITE_ON")
                .is_some()
                && std::env::var_os("RUSTEMU_SPIDEY_MENU_COMPOSITE_OFF").is_none();
            let enable_bonus_menu_panel_sanitize = enable_bonus_menu_composite
                || (std::env::var_os("RUSTEMU_SPIDEY_MENU_PANEL_FIX_ON").is_some()
                    && std::env::var_os("RUSTEMU_SPIDEY_MENU_PANEL_FIX_OFF").is_none());
            let cache_bonus_menu_underlay = enable_bonus_menu_composite
                && bonus_menu_active
                && red_web_frame
                && !full_white_transition
                && !legal_or_title_overlay
                && !selector_seen;
            let sparse_bonus_menu_widget = enable_bonus_menu_composite
                && bonus_menu_active
                && selector_seen
                && sample_rgb > 0
                && sample_rgb <= 64
                && sample_alpha != 0;
            let cache_bonus_menu_selector_frame = enable_bonus_menu_composite
                && bonus_menu_active
                && selector_seen
                && sample_rgb > 96
                && sample_alpha != 0
                && !red_web_frame
                && !full_white_transition
                && !legal_or_title_overlay;
            drop(gpu);

            static SWAP_FLAGS_LOG: AtomicU32 = AtomicU32::new(0);
            let flag_log = SWAP_FLAGS_LOG.fetch_add(1, Ordering::Relaxed);
            if flag_log < 64 || flag_log.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-SWAP-FLAGS] swap={} flags=0x{:08X} default={} copy={} bypass={} finish={} copy_phase={} pending_before={} resolved={} front_bound={} restored={} presented={}",
                    snap_swap_n,
                    swap_flags,
                    swap_flags == X_D3DSWAP_DEFAULT,
                    (swap_flags & X_D3DSWAP_COPY) != 0,
                    (swap_flags & X_D3DSWAP_BYPASSCOPY) != 0,
                    finish_phase,
                    copy_phase,
                    pending_before,
                    resolved,
                    front_bound,
                    restored_backbuffer,
                    pixels.is_some()
                ));
            }

            if let Some((key, width, height, data, pitch)) = active_rt {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-SWAP-RT] swap={} pcrtc_start=0x{:08X} active_rt=0x{:08X} {}x{} data=0x{:08X} pitch={} resolved={} sample_nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} legal_like={}",
                    snap_swap_n,
                    pcrtc_start,
                    key,
                    width,
                    height,
                    data,
                    pitch,
                    resolved,
                    sample_nonzero,
                    sample_alpha,
                    sample_rgb,
                    first_px,
                    mid_px,
                    legal_or_title_overlay as u8
                ));
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-SWAP-RT] swap={} pcrtc_start=0x{:08X} active_rt=unknown resolved={} sample_nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} legal_like={}",
                    snap_swap_n,
                    pcrtc_start,
                    resolved,
                    sample_nonzero,
                    sample_alpha,
                    sample_rgb,
                    first_px,
                    mid_px,
                    legal_or_title_overlay as u8
                ));
            }
            if let Some(pixels) = pixels {
                did_present_frame = true;
                static BONUS_MENU_UNDERLAY_CACHE: std::sync::Mutex<Option<Vec<u32>>> =
                    std::sync::Mutex::new(None);
                static BONUS_MENU_OVERLAY_CACHE: std::sync::Mutex<Option<Vec<u32>>> =
                    std::sync::Mutex::new(None);
                static BONUS_MENU_PRESENT_CACHE: std::sync::Mutex<Option<Vec<u32>>> =
                    std::sync::Mutex::new(None);
                static BONUS_MENU_SELECTOR_CACHE_RESET: AtomicBool = AtomicBool::new(false);
                static BONUS_MENU_RUN_CACHE_RESET: AtomicBool = AtomicBool::new(false);
                if snap_swap_n == 0 && !BONUS_MENU_RUN_CACHE_RESET.swap(true, Ordering::Relaxed) {
                    SPIDEY_MENU_SELECTOR_SEEN.store(false, Ordering::Relaxed);
                    *SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    *BONUS_MENU_UNDERLAY_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    *BONUS_MENU_OVERLAY_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    *BONUS_MENU_PRESENT_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    BONUS_MENU_SELECTOR_CACHE_RESET.store(false, Ordering::Relaxed);
                    crate::xbox::emulator::debug_log(
                        "[SPIDEY-MENU-RUN-CACHE-RESET] swap=0 cleared menu presentation caches",
                    );
                }
                if bonus_menu_active
                    && selector_seen
                    && !BONUS_MENU_SELECTOR_CACHE_RESET.swap(true, Ordering::Relaxed)
                {
                    *BONUS_MENU_UNDERLAY_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    *BONUS_MENU_OVERLAY_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    *BONUS_MENU_PRESENT_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner()) = None;
                    crate::xbox::emulator::debug_log(&format!(
                        "[SPIDEY-MENU-SELECTOR-CACHE-RESET] swap={} scene='{}' stash='{}'",
                        snap_swap_n, scene_name_for_present, stash_name_for_present
                    ));
                }
                let has_bonus_menu_present_cache = BONUS_MENU_PRESENT_CACHE
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .is_some();
                let has_selector_present_cache = SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .is_some();
                let hold_post_selector_menu_frames =
                    std::env::var_os("RUSTEMU_SPIDEY_MENU_PRESENT_HOLD_ON").is_some()
                        && std::env::var_os("RUSTEMU_SPIDEY_MENU_PRESENT_HOLD_OFF").is_none();
                let hold_cached_bonus_menu_frame = (hold_bonus_menu_black_frame && !selector_seen)
                    || (hold_post_selector_menu_frames
                        && ((bonus_menu_active
                            && selector_seen
                            && red_web_frame
                            && has_selector_present_cache)
                            || (bonus_menu_active
                                && selector_seen
                                && swap_flags == X_D3DSWAP_DEFAULT
                                && sample_rgb == 0
                                && sample_alpha != 0
                                && has_bonus_menu_present_cache)));
                if hold_cached_bonus_menu_frame {
                    static BONUS_MENU_BLACK_HOLD_LOG: AtomicU32 = AtomicU32::new(0);
                    let hold_n = BONUS_MENU_BLACK_HOLD_LOG.fetch_add(1, Ordering::Relaxed);
                    let selector_cached = if selector_seen {
                        SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .clone()
                    } else {
                        None
                    };
                    let present_cached = BONUS_MENU_PRESENT_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .clone();
                    let underlay_cached = BONUS_MENU_UNDERLAY_CACHE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .clone();
                    let cached_kind = if selector_cached.is_some() {
                        "selector"
                    } else if present_cached.is_some() {
                        "present"
                    } else if underlay_cached.is_some() {
                        "underlay"
                    } else {
                        "none"
                    };
                    let cached_present = selector_cached.or(present_cached).or(underlay_cached);
                    if hold_n < 16 || hold_n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[SPIDEY-MENU-PRESENT-HOLD] #{} swap={} scene='{}' stash='{}' sample_rgb={} black_clear_without_draw={} selector_seen={} cached_present={} cache_kind={}",
                            hold_n,
                            snap_swap_n,
                            scene_name_for_present,
                            stash_name_for_present,
                            sample_rgb,
                            black_clear_without_draw,
                            selector_seen as u8,
                            cached_present.is_some() as u8,
                            cached_kind
                        ));
                    }
                    if let Some(ref present) = cached_present {
                        crate::xbox::gpu::store_readback(present);
                        if capture_every_swap || snap_swap_n < 4 || snap_swap_n % 50 == 0 {
                            write_readback_bmp(
                                r"./organic_latest_swap_readback.bmp",
                                640,
                                480,
                                present,
                            );
                        }
                    }
                } else {
                    let mut presented_pixels: Option<Vec<u32>> = None;
                    if cache_bonus_menu_selector_frame {
                        {
                            let mut cache = BONUS_MENU_UNDERLAY_CACHE
                                .lock()
                                .unwrap_or_else(|e| e.into_inner());
                            *cache = Some(pixels.clone());
                        }
                        {
                            let mut selector_cache = SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                                .lock()
                                .unwrap_or_else(|e| e.into_inner());
                            *selector_cache = Some(pixels.clone());
                        }
                        let overlay = BONUS_MENU_OVERLAY_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner());
                        if let Some(ref cached_overlay) = *overlay {
                            if let Some(composited) =
                                composite_menu_widget_overlay(&pixels, cached_overlay)
                            {
                                presented_pixels = Some(composited);
                            }
                        }
                        static SELECTOR_CACHE_LOG: AtomicU32 = AtomicU32::new(0);
                        let cache_n = SELECTOR_CACHE_LOG.fetch_add(1, Ordering::Relaxed);
                        if cache_n < 16 || cache_n.is_power_of_two() {
                            crate::xbox::emulator::debug_log(&format!(
                                "[SPIDEY-MENU-SELECTOR-CACHE] #{} swap={} sample_rgb={} first=0x{:08X} mid=0x{:08X}",
                                cache_n, snap_swap_n, sample_rgb, first_px, mid_px
                            ));
                        }
                    } else if cache_bonus_menu_underlay {
                        let mut cache = BONUS_MENU_UNDERLAY_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner());
                        *cache = Some(pixels.clone());
                        static CACHE_LOG: AtomicU32 = AtomicU32::new(0);
                        let cache_n = CACHE_LOG.fetch_add(1, Ordering::Relaxed);
                        if cache_n < 16 || cache_n.is_power_of_two() {
                            crate::xbox::emulator::debug_log(&format!(
                                "[SPIDEY-MENU-UNDERLAY-CACHE] #{} swap={} sample_rgb={} first=0x{:08X} mid=0x{:08X}",
                                cache_n, snap_swap_n, sample_rgb, first_px, mid_px
                            ));
                        }
                    } else if sparse_bonus_menu_widget {
                        let cache = BONUS_MENU_UNDERLAY_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner());
                        if let Some(ref underlay) = *cache {
                            {
                                let mut overlay = BONUS_MENU_OVERLAY_CACHE
                                    .lock()
                                    .unwrap_or_else(|e| e.into_inner());
                                *overlay = Some(pixels.clone());
                            }
                            if let Some(composited) =
                                composite_menu_widget_overlay(underlay, &pixels)
                            {
                                static COMPOSITE_LOG: AtomicU32 = AtomicU32::new(0);
                                let comp_n = COMPOSITE_LOG.fetch_add(1, Ordering::Relaxed);
                                if comp_n < 16 || comp_n.is_power_of_two() {
                                    crate::xbox::emulator::debug_log(&format!(
                                        "[SPIDEY-MENU-WIDGET-COMPOSITE] #{} swap={} overlay_rgb={} cached_underlay=true",
                                        comp_n, snap_swap_n, sample_rgb
                                    ));
                                }
                                presented_pixels = Some(composited);
                            }
                        } else {
                            {
                                let mut overlay = BONUS_MENU_OVERLAY_CACHE
                                    .lock()
                                    .unwrap_or_else(|e| e.into_inner());
                                *overlay = Some(pixels.clone());
                            }
                            static MISS_LOG: AtomicU32 = AtomicU32::new(0);
                            let miss_n = MISS_LOG.fetch_add(1, Ordering::Relaxed);
                            if miss_n < 16 || miss_n.is_power_of_two() {
                                crate::xbox::emulator::debug_log(&format!(
                                    "[SPIDEY-MENU-WIDGET-COMPOSITE-MISS] #{} swap={} overlay_rgb={} cached_overlay=true no cached underlay",
                                    miss_n, snap_swap_n, sample_rgb
                                ));
                            }
                        }
                    }

                    if enable_bonus_menu_panel_sanitize
                        && bonus_menu_active
                        && !full_white_transition
                    {
                        let selector_cached = if selector_seen {
                            SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .clone()
                        } else {
                            None
                        };
                        let cached_underlay = BONUS_MENU_UNDERLAY_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .clone();
                        let cached_present = BONUS_MENU_PRESENT_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .clone();
                        let fallback = selector_cached
                            .as_deref()
                            .or(cached_present.as_deref())
                            .or(cached_underlay.as_deref());
                        let candidate = presented_pixels.as_deref().unwrap_or(&pixels);
                        if let Some(fixed) = sanitize_bonus_menu_white_panel(candidate, fallback) {
                            presented_pixels = Some(fixed);
                        }
                    }

                    let present_ref: &[u32] = presented_pixels.as_deref().unwrap_or(&pixels);
                    if bonus_menu_active
                        && !full_white_transition
                        && (sparse_bonus_menu_widget || cache_bonus_menu_selector_frame)
                    {
                        let mut present_cache = BONUS_MENU_PRESENT_CACHE
                            .lock()
                            .unwrap_or_else(|e| e.into_inner());
                        *present_cache = Some(present_ref.to_vec());
                        if cache_bonus_menu_selector_frame {
                            let mut selector_cache = SPIDEY_MENU_SELECTOR_PRESENT_CACHE
                                .lock()
                                .unwrap_or_else(|e| e.into_inner());
                            *selector_cache = Some(present_ref.to_vec());
                        }
                    }
                    crate::xbox::gpu::store_readback(present_ref);
                    // Capture every 50th swap (plus first 4) so we get a snapshot
                    // of the actual rendered content throughout the run, not just at boot.
                    if capture_every_swap || snap_swap_n < 4 || snap_swap_n % 50 == 0 {
                        write_readback_bmp(
                            r"./organic_latest_swap_readback.bmp",
                            640,
                            480,
                            present_ref,
                        );
                    }
                }
            }
        }
    }
    // Signal main thread that a new frame is ready
    if did_present_frame {
        crate::xbox::gpu::set_frame_dirty();
    }
    // 2026-04-24 profiling: compute per-swap event deltas. Every 100th swap
    // (plus the first 10) logs a single line showing where overhead went.
    // Diagnostic only — zero effect on behavior.
    {
        let d_mmio = crate::xbox::aot::oovpa::OOVPA_MMIO_COUNT
            .load(Ordering::Relaxed)
            .wrapping_sub(snap_mmio);
        let d_pb = crate::xbox::aot::oovpa::OOVPA_PB_COMMANDS
            .load(Ordering::Relaxed)
            .wrapping_sub(snap_pb);
        let d_hooks = crate::xbox::aot::oovpa::OOVPA_HOOKS_FIRED
            .load(Ordering::Relaxed)
            .wrapping_sub(snap_hooks);
        let d_nv2a_draws = crate::xbox::aot::nv2a_pb::NV2A_DRAW_METHODS
            .load(Ordering::Relaxed)
            .wrapping_sub(snap_nv2a_draws);
        let elapsed_us = snap_ts.elapsed().as_micros();
        if snap_swap_n < 10 || (snap_swap_n + 1) % 100 == 0 {
            crate::xbox::emulator::debug_log(&format!(
                "[SWAP-PROF #{}] elapsed={:>6}us mmio_delta={:>4} pb_writes_delta={:>3} hooks_delta={:>4} nv2a_methods_delta={:>4}",
                snap_swap_n, elapsed_us, d_mmio, d_pb, d_hooks, d_nv2a_draws
            ));
        }
    }

    *mmio_count += 1;
    // Spider-Man's real VBlank DPC path can enter a very hot SDK Swap loop.
    // Keep pacing opt-in so the default smoke gate measures the emulator's
    // normal timing behavior, not a scheduler nudge.
    static SWAP_PACE_MS: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    let pace_ms = *SWAP_PACE_MS.get_or_init(|| {
        std::env::var("RUSTEMU_HLE_SWAP_PACE_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
    });
    if pace_ms > 0 {
        static LAST_SWAP: std::sync::LazyLock<std::sync::Mutex<std::time::Instant>> =
            std::sync::LazyLock::new(|| std::sync::Mutex::new(std::time::Instant::now()));
        if let Ok(mut last) = LAST_SWAP.lock() {
            let target = std::time::Duration::from_millis(pace_ms);
            let elapsed = last.elapsed();
            if elapsed < target {
                std::thread::sleep(target - elapsed);
            }
            *last = std::time::Instant::now();
        }
    }
    0
}

/// Clear: translate Xbox D3DDevice_Clear → host GPU clear.
/// Xbox D3D8 Clear(Count, pRects, Flags, Color, Z, Stencil)
/// args[0]=Count, args[1]=pRects, args[2]=Flags, args[3]=Color
pub(crate) fn hle_clear(args: &[u32; 8], _guest_mem: *mut u8, mmio_count: &mut u64) -> u32 {
    const X_D3DCLEAR_ZBUFFER: u32 = 0x0000_0001;
    const X_D3DCLEAR_STENCIL: u32 = 0x0000_0002;
    const X_D3DCLEAR_TARGET: u32 = 0x0000_00F0;

    let count = args[0];
    let rects = args[1];
    let flags = args[2];
    let color = args[3]; // ARGB clear color
    let clear_color = (flags & X_D3DCLEAR_TARGET) != 0;
    let clear_depth = (flags & X_D3DCLEAR_ZBUFFER) != 0;
    let clear_stencil = (flags & X_D3DCLEAR_STENCIL) != 0;

    let mut gpu = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut backend) = *gpu {
        backend.clear_surface(color, clear_color, clear_depth, clear_stencil);
    }
    static CLEAR_LOG: AtomicU32 = AtomicU32::new(0);
    let n = CLEAR_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        let active_rt = gpu
            .as_ref()
            .and_then(|backend| backend.debug_active_render_target());
        if let Some((key, width, height, data, pitch)) = active_rt {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-CLEAR] #{} count={} rects=0x{:08X} flags=0x{:08X} target={} z={} stencil={} color=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                n,
                count,
                rects,
                flags,
                clear_color,
                clear_depth,
                clear_stencil,
                color,
                key,
                width,
                height,
                data,
                pitch
            ));
        } else {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-CLEAR] #{} count={} rects=0x{:08X} flags=0x{:08X} target={} z={} stencil={} color=0x{:08X} rt=unknown",
                n,
                count,
                rects,
                flags,
                clear_color,
                clear_depth,
                clear_stencil,
                color
            ));
        }
    }
    *mmio_count += 1;
    0
}

/// SetVertexShader: store handle in device struct + mirror into nv2a_pb
/// parser state so the pushbuffer parser can derive stride from FVF.
pub(super) fn hle_set_vertex_shader(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let handle = args[0];
    drain_draws_before_state_change("SetVertexShader");
    let dev_ptr = read_dev_ptr(guest_mem);
    if dev_ptr != 0 {
        unsafe {
            *((guest_mem as u64 + dev_ptr as u64 + DEV_VSHADER as u64) as *mut u32) = handle;
        }
    }
    fn reuse_zero_vsh_enabled() -> bool {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| {
            std::env::var("RUSTEMU_SPIDEY_REUSE_ZERO_VSH")
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false)
        })
    }
    static LAST_TRANSLATED_VSH: AtomicU32 = AtomicU32::new(0);
    let requested_hlsl = crate::xbox::gpu::nv2a_vsh::hlsl_for_handle(handle);
    if handle != 0 && requested_hlsl.is_some() {
        LAST_TRANSLATED_VSH.store(handle, Ordering::Relaxed);
    }
    let mut effective_handle = handle;
    let mut reused_zero = false;
    if handle == 0 && reuse_zero_vsh_enabled() {
        let last = LAST_TRANSLATED_VSH.load(Ordering::Relaxed);
        if last != 0 && crate::xbox::gpu::nv2a_vsh::shader_has_hlsl(last) {
            effective_handle = last;
            reused_zero = true;
        }
    }
    // Splice from JIT 2026-04-20: publish the FVF/shader handle so nv2a_pb
    // parser can decode inline vertex streams with the right stride/fields.
    crate::xbox::aot::nv2a_pb::VERTEX_SHADER
        .store(effective_handle, std::sync::atomic::Ordering::Relaxed);
    let fvf = crate::xbox::aot::nv2a_pb::fvf_from_vertex_shader_value(effective_handle);
    let fvf_stride = if fvf != 0 {
        crate::xbox::aot::nv2a_pb::seed_fvf_vertex_slots(fvf)
    } else {
        0
    };
    let hlsl = if reused_zero {
        crate::xbox::gpu::nv2a_vsh::hlsl_for_handle(effective_handle)
    } else {
        requested_hlsl
    };
    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_vertex_shader_hlsl(effective_handle, hlsl.as_deref());
    }
    static VSH_BIND_LOG: AtomicU32 = AtomicU32::new(0);
    let n = VSH_BIND_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-VSH-BIND] #{} handle=0x{:08X} effective=0x{:08X} fvf=0x{:X} fvf_stride={} translated={} reused_zero={}",
            n,
            handle,
            effective_handle,
            fvf,
            fvf_stride,
            hlsl.is_some(),
            reused_zero
        ));
    }
    0
}

pub(super) fn hle_trace_vertex_shader_call(
    name: &str,
    args: &[u32; 8],
    guest_mem: *mut u8,
    source: &str,
    guest_addr: u32,
    ret_addr: u32,
    r14: u32,
    regs: [u32; 6],
) -> u32 {
    if !spidey_vsh_call_trace_enabled() {
        return 0;
    }

    static VSH_CALL_TRACE: AtomicU32 = AtomicU32::new(0);
    let n = VSH_CALL_TRACE.fetch_add(1, Ordering::Relaxed);
    if n >= 512 && !n.is_power_of_two() {
        return 0;
    }

    let current_vs = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
    let stream0_stride = crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.load(Ordering::Relaxed);
    let slot_summary = [0usize, 1, 2, 3, 9]
        .iter()
        .map(|&slot| {
            let ty = crate::xbox::aot::nv2a_pb::VS_SLOT_TYPE[slot].load(Ordering::Relaxed);
            let sz = crate::xbox::aot::nv2a_pb::VS_SLOT_SIZE[slot].load(Ordering::Relaxed);
            let stride = crate::xbox::aot::nv2a_pb::VS_SLOT_STRIDE[slot].load(Ordering::Relaxed);
            let off = crate::xbox::aot::nv2a_pb::VS_SLOT_OFFSET[slot].load(Ordering::Relaxed);
            format!("s{}:t{} sz{} st{} off0x{:08X}", slot, ty, sz, stride, off)
        })
        .collect::<Vec<_>>()
        .join(" ");
    let args_summary = args
        .iter()
        .map(|v| format!("0x{:08X}", v))
        .collect::<Vec<_>>()
        .join(",");
    let arg_mem_summary = args
        .iter()
        .take(4)
        .enumerate()
        .map(|(idx, &ptr)| {
            let words = (0..4u32)
                .map(|i| {
                    read_u32_guest(guest_mem, ptr.wrapping_add(i * 4))
                        .map(|v| format!("{:08X}", v))
                        .unwrap_or_else(|| "--------".to_string())
                })
                .collect::<Vec<_>>()
                .join("/");
            format!("arg{}@0x{:08X}=[{}]", idx, ptr, words)
        })
        .collect::<Vec<_>>()
        .join(" ");

    debug_log(&format!(
        "[HLE-VSH-CALL] #{} {} source={} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} \
         args=[{}] regs eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} \
         current_vs=0x{:08X} stream0_stride={} {} {}",
        n,
        name,
        source,
        guest_addr,
        ret_addr,
        r14,
        args_summary,
        regs[0],
        regs[1],
        regs[2],
        regs[3],
        regs[4],
        regs[5],
        current_vs,
        stream0_stride,
        slot_summary,
        arg_mem_summary
    ));
    0
}

/// Capture the first SetVertexShaderInput stream override.
///
/// Xbox D3D8 stores X_STREAMINPUT as { VertexBuffer*, Stride, Offset }. This
/// HLE first pass only mirrors stream 0 into the same state used by our manual
/// DrawVertices/DrawIndexedVertices decoders.
pub(super) fn hle_set_vertex_shader_input(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let handle = args[0];
    let stream_count = args[1].min(16);
    let inputs = args[2];

    static VSI_LOG: AtomicU32 = AtomicU32::new(0);
    let n = VSI_LOG.fetch_add(1, Ordering::Relaxed);

    if handle == 0 {
        if n < 16 || n.is_power_of_two() {
            debug_log("[HLE-VSI] clear override handle=0");
        }
        return 0;
    }

    if stream_count == 0 {
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-VSI] skip handle=0x{:08X} stream_count=0 inputs=0x{:08X}",
                handle, inputs
            ));
        }
        return 0;
    }

    let Some(input0) = normalize_guest_ram_ptr(inputs) else {
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[HLE-VSI] skip handle=0x{:08X} invalid inputs=0x{:08X}",
                handle, inputs
            ));
        }
        return 0;
    };
    if input0 > 0x2000_0000u32.saturating_sub(12) {
        return 0;
    }

    let vb_ptr = read_u32_guest(guest_mem, input0).unwrap_or(0);
    let stride = read_u32_guest(guest_mem, input0.wrapping_add(4)).unwrap_or(0);
    let offset = read_u32_guest(guest_mem, input0.wrapping_add(8)).unwrap_or(0);
    let vb_norm = normalize_guest_ram_ptr(vb_ptr).unwrap_or(0);
    let data_raw = if vb_norm != 0 {
        read_u32_guest(guest_mem, vb_norm.wrapping_add(4)).unwrap_or(0)
    } else {
        0
    };
    let data = if data_raw != 0 {
        data_raw
            .checked_add(offset)
            .and_then(normalize_guest_ram_ptr)
            .unwrap_or(0)
    } else {
        0
    };

    if data != 0 && (1..=256).contains(&stride) {
        STREAM0_VB_PTR.store(data, Ordering::Relaxed);
        STREAM0_STRIDE.store(stride, Ordering::Relaxed);
        crate::xbox::aot::nv2a_pb::STREAM0_VB_ADDR.store(data, Ordering::Relaxed);
        crate::xbox::aot::nv2a_pb::STREAM0_STRIDE.store(stride, Ordering::Relaxed);
    }

    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-VSI] #{} handle=0x{:08X} streams={} input0=0x{:08X}->0x{:08X} vb=0x{:08X}->0x{:08X} data=0x{:08X}+0x{:X}->0x{:08X} stride={} captured={}",
            n,
            handle,
            stream_count,
            inputs,
            input0,
            vb_ptr,
            vb_norm,
            data_raw,
            offset,
            data,
            stride,
            data != 0 && (1..=256).contains(&stride)
        ));
    }
    0
}

/// CreateVertexShader(pDeclaration, pFunction, pHandle, Usage) — ret 0x10.
/// Captures the D3DVSD_* token stream passed by the game so we can recover
/// the vertex format / stride without parsing the NV2A pushbuffer.
///
/// Returns D3D_OK and writes a fake handle to *pHandle so the game continues.
/// The real value comes from logging pDeclaration's token array (terminates
/// at 0xFFFFFFFF = D3DVSD_END).
pub(super) fn hle_create_vertex_shader(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static VS_CREATE_COUNT: AtomicU32 = AtomicU32::new(0);
    static VS_HANDLE_BUMP: AtomicU32 = AtomicU32::new(0x0000_1001);

    let p_decl = args[0];
    let p_func = args[1];
    let p_handle = args[2];
    let usage = args[3];
    let n = VS_CREATE_COUNT.fetch_add(1, Ordering::Relaxed);

    // Walk the token stream starting at pDeclaration until D3DVSD_END (0xFFFFFFFF).
    // Cap at 64 tokens to prevent runaway on bad input.
    let mut tokens: Vec<u32> = Vec::with_capacity(64);
    if p_decl != 0 && p_decl < 0x2000_0000 {
        for i in 0u32..64 {
            let addr = p_decl.wrapping_add(i * 4);
            if addr as u64 + 4 > 0x2000_0000 {
                break;
            }
            let tok = unsafe { *((guest_mem as u64 + addr as u64) as *const u32) };
            tokens.push(tok);
            if tok == 0xFFFF_FFFF {
                break;
            }
        }
    }

    // Format tokens for the log line.
    let tok_str = tokens
        .iter()
        .map(|t| format!("0x{:08X}", t))
        .collect::<Vec<_>>()
        .join(",");

    // Compute approximate stride by decoding D3DVSD_REG tokens.
    // D3DVSD_REG layout: 0x20000000 | (DataType<<16) | Register.
    // DataType sizes (in bytes):
    //   D3DVSDT_FLOAT1=0x12→4, FLOAT2=0x22→8, FLOAT3=0x32→12, FLOAT4=0x42→16,
    //   D3DCOLOR=0x40→4, UBYTE4=0x05→4, SHORT2=0x25→4, SHORT4=0x45→8,
    //   NORMPACKED3=0x16→4, SHORT1=0x15→2, SHORT3=0x35→6,
    //   NORMSHORT1=0x11→2, NORMSHORT2=0x21→4, NORMSHORT3=0x31→6, NORMSHORT4=0x41→8,
    //   PBYTE1=0x14→1, PBYTE2=0x24→2,
    //   PBYTE3=0x34→3, PBYTE4=0x44→4.
    // Unknown types count as 0 (logged as ?).
    fn type_size(data_type: u8) -> u32 {
        match data_type {
            0x12 => 4,
            0x22 => 8,
            0x32 => 12,
            0x42 => 16,
            0x40 => 4, // D3DCOLOR
            0x05 => 4, // UBYTE4
            0x15 => 2, // SHORT1
            0x25 => 4, // SHORT2 / FLOAT16_2
            0x35 => 6, // SHORT3 / NORMSHORT3
            0x45 => 8, // SHORT4 / FLOAT16_4
            0x16 => 4, // NORMPACKED3
            0x11 => 2, // NORMSHORT1
            0x21 => 4, // NORMSHORT2
            0x31 => 6, // NORMSHORT3
            0x41 => 8, // NORMSHORT4
            0x24 => 2, // PBYTE2
            0x44 => 4, // PBYTE4
            0x14 => 1, // PBYTE1
            0x34 => 3, // PBYTE3
            0x61 => 4, // UBYTE4
            _ => 0,
        }
    }
    let mut stride: u32 = 0;
    let mut offset: u32 = 0;
    let mut decl_info = VertexDeclInfo::default();
    let mut per_reg_decoded: Vec<String> = Vec::new();
    for &tok in &tokens {
        if tok == 0xFFFF_FFFF {
            break;
        }
        // D3DVSD_REG token: top nibble == 2 (TOKENTYPE_STREAMDATA).
        let token_type = (tok >> 29) & 0x7;
        if token_type == 2 {
            // STREAMDATA: 0x20000000 | (data_type<<16) | reg
            let data_type = ((tok >> 16) & 0xFF) as u8;
            let reg = (tok & 0x1F) as u8;
            let sz = type_size(data_type);
            let elem = VertexDeclElement {
                data_type,
                offset,
                size: sz,
            };
            match reg {
                2 => decl_info.v2 = Some(elem),
                4 => decl_info.v4 = Some(elem),
                5 => decl_info.v5 = Some(elem),
                6 => decl_info.v6 = Some(elem),
                _ => {}
            }
            stride += sz;
            decl_info.stride = stride;
            per_reg_decoded.push(format!(
                "reg{}:dt=0x{:02X}:off={}:sz={}",
                reg, data_type, offset, sz
            ));
            offset = offset.saturating_add(sz);
        }
    }

    // Allocate a fake handle so the game continues.
    let fake_handle = VS_HANDLE_BUMP.fetch_add(2, Ordering::Relaxed);
    xgrph_lle_canary_dump_create_vertex_shader(n, fake_handle, p_func, guest_mem);
    if p_handle != 0 && p_handle < 0x2000_0000 {
        unsafe {
            *((guest_mem as u64 + p_handle as u64) as *mut u32) = fake_handle;
        }
    }
    let shader_info =
        register_vertex_shader_info(fake_handle, take_pending_shader_source(), decl_info);
    let source_captured_shader = crate::xbox::gpu::nv2a_vsh::take_pending_xgrph_shader();
    let captured_shader = if xgrph_lle_native_handoff_enabled() {
        read_native_xgrph_shader_tokens(guest_mem, p_func)
            .map(|tokens| {
                let screenspace = source_captured_shader
                    .as_ref()
                    .map(|shader| shader.screenspace)
                    .unwrap_or(false);
                let source_snippet = source_captured_shader
                    .as_ref()
                    .map(|shader| shader.source_snippet.clone())
                    .unwrap_or_else(|| format!("native pFunc=0x{p_func:08X}"));
                let supplement_skin = xgrph_lle_native_skin_supplement_enabled()
                    && shader_info.as_ref().is_some_and(|info| info.skinned);
                crate::xbox::gpu::nv2a_vsh::capture_from_native_xgrph_tokens(
                    tokens,
                    screenspace,
                    source_snippet,
                    supplement_skin,
                    source_captured_shader
                        .as_ref()
                        .map(|shader| shader.tokens.as_slice()),
                )
            })
            .or(source_captured_shader)
    } else {
        source_captured_shader
    };
    if let Some(shader) = captured_shader.clone() {
        let first = crate::xbox::gpu::nv2a_vsh::first_token_words(&shader.tokens, 16);
        crate::xbox::gpu::nv2a_vsh::register_shader_handle(fake_handle, shader.clone());
        static VSH_REGISTER_LOG: AtomicU32 = AtomicU32::new(0);
        let k = VSH_REGISTER_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 24 || k.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[NV2A-VSH-REGISTER] #{} handle=0x{:08X} pFunc=0x{:08X} native_handoff={} native_skin_supplement={} tokens={} first=[{}] screenspace={} hlsl_len={} src='{}'",
                k,
                fake_handle,
                p_func,
                xgrph_lle_native_handoff_enabled(),
                xgrph_lle_native_skin_supplement_enabled()
                    && shader_info.as_ref().is_some_and(|info| info.skinned),
                shader.tokens.len() / 4,
                first,
                shader.screenspace,
                shader.hlsl.len(),
                shader.source_snippet
            ));
        }
        if shader_info.as_ref().is_some_and(|info| info.skinned) || fake_handle == 0x0000_1003 {
            static VSH_HLSL_LOG: AtomicU32 = AtomicU32::new(0);
            let s = VSH_HLSL_LOG.fetch_add(1, Ordering::Relaxed);
            if s < 8 || s.is_power_of_two() {
                let mut hlsl = shader.hlsl.chars().take(6000).collect::<String>();
                hlsl = hlsl.replace('\n', "\\n");
                crate::xbox::emulator::debug_log(&format!(
                    "[NV2A-VSH-HLSL] #{} handle=0x{:08X} tokens={} simple_dp4={} skinned={} hlsl_len={} hlsl=\"{}\"",
                    s,
                    fake_handle,
                    shader.tokens.len() / 4,
                    shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
                    shader_info.as_ref().is_some_and(|info| info.skinned),
                    shader.hlsl.len(),
                    hlsl
                ));
            }
        }
    }

    crate::xbox::emulator::debug_log(&format!(
        "[VS-DECL] #{} pDecl=0x{:08X} pFunc=0x{:08X} pHandle=0x{:08X} usage=0x{:X} handle=0x{:08X} stride={} screenspace={} simple_dp4={} skinned={} regs=[{}] tokens=[{}] vsh_tokens={} src='{}'",
        n,
        p_decl,
        p_func,
        p_handle,
        usage,
        fake_handle,
        stride,
        shader_info.as_ref().is_some_and(|info| info.screenspace),
        shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
        shader_info.as_ref().is_some_and(|info| info.skinned),
        per_reg_decoded.join(","),
        tok_str,
        captured_shader.as_ref().map(|s| s.tokens.len() / 4).unwrap_or(0),
        shader_info
            .as_ref()
            .map(|info| info.snippet.as_str())
            .unwrap_or("")
    ));

    0 // D3D_OK
}

/// CreatePixelShader(pFunction, pHandle) — ret 0x8.
/// Mirrors the XDK helper closely enough for titles that keep the returned
/// shader object around: allocate a tiny object, point its token pointer at
/// inline copied function data, and publish the object through *pHandle.
pub(super) fn hle_create_pixel_shader(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static PS_CREATE_COUNT: AtomicU32 = AtomicU32::new(0);

    let p_func = args[0];
    let p_handle = args[1];
    let fake = alloc_fake_d3d_obj(guest_mem, 0x100);
    let n = PS_CREATE_COUNT.fetch_add(1, Ordering::Relaxed);

    if fake == 0 {
        if n < 8 {
            debug_log(&format!(
                "[HLE] CreatePixelShader: OOM pFunc=0x{:08X} pHandle=0x{:08X}",
                p_func, p_handle
            ));
        }
        return 0x8007_000E; // E_OUTOFMEMORY
    }

    unsafe {
        let obj = guest_mem.add(fake as usize);
        std::ptr::write_unaligned(obj as *mut u32, 1);
        std::ptr::write_unaligned(obj.add(0x04) as *mut u32, 1);
        std::ptr::write_unaligned(obj.add(0x08) as *mut u32, fake + 0x0C);

        if let Some(src) = normalize_guest_ram_ptr(p_func) {
            if src <= 0x2000_0000u32.saturating_sub(0xF0) {
                std::ptr::copy_nonoverlapping(guest_mem.add(src as usize), obj.add(0x0C), 0xF0);
            }
        }

        if valid_guest_ptr(p_handle) {
            let Some(dst) = normalize_guest_ram_ptr(p_handle) else {
                if n < 8 {
                    debug_log(&format!(
                        "[HLE] CreatePixelShader: invalid handle mirror pHandle=0x{:08X}",
                        p_handle
                    ));
                }
                return 0;
            };
            std::ptr::write_unaligned(guest_mem.add(dst as usize) as *mut u32, fake);
        }
    }

    if n < 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE] CreatePixelShader #{}: pFunc=0x{:08X} pHandle=0x{:08X} -> fake=0x{:08X}",
            n, p_func, p_handle, fake
        ));
    }

    0 // D3D_OK
}

/// CreateTexture(W, H, Levels, Usage, Format, Pool, ppTexture) — mint a fake texture
/// handle (0x83F0xxxx) and write it to the guest output pointer.
///
/// Called from BOTH the VEH path (execute_manual_hle's match arm for
/// "D3DDevice_CreateTexture") AND the interpreter path
/// (oovpa_dispatch::execute_hle_for_interpreter).
///
/// 2026-04-25: extracted from inline match arm in execute_manual_hle. The body
/// was correct but unreachable from the interpreter path — oovpa_dispatch.rs:335
/// had a hardcoded `=> 0` stub bypassing it. Result: 49 CreateTexture calls
/// per Spider-Man legal-screen run all returned `*ppTex=NULL`, the engine
/// then short-circuited every downstream font/atlas bind path, and only
/// 0x83F00100 (LEGAL DDS, minted via XGRPH path) was ever bound — producing
/// the "barcode" rendering of glyphs sampled against the wrong texture.
/// Agents E1/E6/E8/E10 all converged on the dispatch-stub root cause.
pub(super) fn hle_create_texture(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let pp_texture = args[6];
    if let Some(pp_texture_norm) = guest_ram_offset(pp_texture) {
        let fake = alloc_fake_d3d_obj(guest_mem, 0x48);
        if fake != 0 {
            write_fake_texture_header(guest_mem, fake, args[0], args[1], args[4], 0);
            unsafe {
                let pp = guest_mem.add(pp_texture_norm);
                *(pp as *mut u32) = fake;
            }
            static TEX_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = TEX_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 5 {
                debug_log(&format!(
                    "[HLE] CreateTexture: {}x{} fmt=0x{:X} -> fake=0x{:08X} @[0x{:08X}->0x{:08X}]",
                    args[0], args[1], args[4], fake, pp_texture, pp_texture_norm as u32
                ));
            }
        }
    }
    0 // D3D_OK
}

/// CreateTexture2(W, H, Depth, Levels, Usage, Format, Pool) — internal XDK helper
/// that returns the texture object directly in EAX instead of writing ppTexture.
pub(super) fn hle_create_texture2(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let fake = alloc_fake_d3d_obj(guest_mem, 0x48);
    if fake != 0 {
        write_fake_texture_header(guest_mem, fake, args[0], args[1], args[5], 0);
    }

    static TEX2_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = TEX2_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE] CreateTexture2: {}x{} depth={} levels={} usage=0x{:X} fmt=0x{:X} pool={} -> fake=0x{:08X}",
            args[0], args[1], args[2], args[3], args[4], args[5], args[6], fake
        ));
    }

    fake
}

/// SetTransform: copy 4x4 matrix to device struct
///
/// Pushbuffer BeginPush/EndPush UI draws are queued with only their vertices.
/// They must be replayed before later D3D state mutations, otherwise glyphs and
/// widgets render with the next texture/blend/depth state instead of the state
/// active when the game submitted them.
fn drain_draws_before_state_change(source: &str) -> usize {
    hle_flush_deferred_render_target_to_active_backend();
    let n = crate::xbox::gpu::drain_queued_draws_into_active_backend(source);
    if n == 0 {
        return 0;
    }
    static DRAIN_TEX_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let k = DRAIN_TEX_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if k < 16 || k.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-STATE-DRAIN] #{} drained={} before {}",
            k, n, source
        ));
    }
    n
}

/// SetTexture(Stage, pTexture) — read guest texture data and upload to GPU backend.
fn drain_draws_before_texture_state_change(stage: u32, p_texture: u32) {
    let source = format!("SetTexture(stage={}, pTex=0x{:08X})", stage, p_texture);
    let _ = drain_draws_before_state_change(&source);
}

fn spidey_watch_texture(stage: u32, tex_raw: u32, tex_norm: u32) -> bool {
    if stage != 0 {
        return false;
    }
    let active = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    spidey_focus_texture_match(stage, tex_raw, tex_norm)
        || tex_norm == active
        || tex_norm == 0x03C3_A630
        || (0x03C3_0000..0x03C4_0000).contains(&tex_norm)
        || (0x83C3_0000..0x83C4_0000).contains(&tex_raw)
}

fn spidey_focus_overlay_texture(stage: u32, tex_raw: u32, tex_norm: u32) -> bool {
    spidey_focus_texture_match(stage, tex_raw, tex_norm)
        || (stage == 0 && (tex_raw == 0x83C3_A630 || tex_norm == 0x03C3_A630))
}

fn sampled_nonzero_bytes(bytes: &[u8]) -> usize {
    let sample_step = (bytes.len() / 1024).max(1);
    bytes
        .iter()
        .step_by(sample_step)
        .take(1024)
        .filter(|b| **b != 0)
        .count()
}

fn spidey_find_loaded_rspiderc_source(
    guest_mem: *mut u8,
    target_data: u32,
) -> Option<(u32, u32, u32, u32, Vec<u8>)> {
    let candidates: Vec<HleTextureInfo> = {
        let reg = texture_registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.iter()
            .copied()
            .filter(|info| {
                info.width == 512
                    && info.height == 512
                    && xbox_texture_format_code(info.format)
                        == crate::xbox::gpu::texture_format::X_D3DFMT_DXT5
                    && normalize_guest_ram_ptr(info.data & !0x3)
                        .map(|data| data != target_data)
                        .unwrap_or(false)
            })
            .collect()
    };

    for info in candidates {
        let Some(data) = normalize_guest_ram_ptr(info.data & !0x3) else {
            continue;
        };
        let byte_len = texture_byte_len(info.width, info.height, info.format);
        if byte_len == 0 || byte_len > 16 * 1024 * 1024 {
            continue;
        }
        if data as u64 + byte_len as u64 > 0x2000_0000 {
            continue;
        }
        let bytes =
            unsafe { std::slice::from_raw_parts(guest_mem.add(data as usize), byte_len as usize) };
        let Some(pixels) = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT5,
            info.width,
            info.height,
            bytes,
        ) else {
            continue;
        };
        let (_nonzero, alpha, rgb, first, mid) = sampled_argb_stats(&pixels);
        let all_white = alpha == 1024 && rgb == 1024 && first == 0xFFFF_FFFF && mid == 0xFFFF_FFFF;
        if rgb == 0 || all_white {
            continue;
        }
        return Some((info.key, info.width, info.height, data, bytes.to_vec()));
    }

    None
}

fn argb_components(px: u32) -> (u8, u8, u8, u8) {
    (
        ((px >> 24) & 0xFF) as u8,
        ((px >> 16) & 0xFF) as u8,
        ((px >> 8) & 0xFF) as u8,
        (px & 0xFF) as u8,
    )
}

fn make_argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

fn rgb888_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3)
}

fn downsample_argb_box(
    src: &[u32],
    src_w: u32,
    src_h: u32,
    dst_w: u32,
    dst_h: u32,
) -> Option<Vec<u32>> {
    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return None;
    }
    if src.len() < (src_w as usize).saturating_mul(src_h as usize) {
        return None;
    }

    let mut dst = vec![0u32; (dst_w as usize).saturating_mul(dst_h as usize)];
    for dy in 0..dst_h {
        let y0 = (dy as u64 * src_h as u64 / dst_h as u64) as u32;
        let mut y1 = (((dy + 1) as u64 * src_h as u64 + dst_h as u64 - 1) / dst_h as u64) as u32;
        y1 = y1.max(y0 + 1).min(src_h);
        for dx in 0..dst_w {
            let x0 = (dx as u64 * src_w as u64 / dst_w as u64) as u32;
            let mut x1 =
                (((dx + 1) as u64 * src_w as u64 + dst_w as u64 - 1) / dst_w as u64) as u32;
            x1 = x1.max(x0 + 1).min(src_w);

            let mut sum_a = 0u64;
            let mut sum_r = 0u64;
            let mut sum_g = 0u64;
            let mut sum_b = 0u64;
            let mut count = 0u64;
            for sy in y0..y1 {
                for sx in x0..x1 {
                    let (a, r, g, b) = argb_components(src[(sy * src_w + sx) as usize]);
                    let aw = a as u64;
                    sum_a += aw;
                    sum_r += r as u64 * aw;
                    sum_g += g as u64 * aw;
                    sum_b += b as u64 * aw;
                    count += 1;
                }
            }

            let a = if count != 0 {
                (sum_a / count).min(255) as u8
            } else {
                0
            };
            let (r, g, b) = if sum_a != 0 {
                (
                    (sum_r / sum_a).min(255) as u8,
                    (sum_g / sum_a).min(255) as u8,
                    (sum_b / sum_a).min(255) as u8,
                )
            } else {
                (0, 0, 0)
            };
            dst[(dy * dst_w + dx) as usize] = make_argb(a, r, g, b);
        }
    }

    Some(dst)
}

fn nearest_byte_index(value: u8, table: &[u8]) -> usize {
    let mut best_i = 0usize;
    let mut best_d = u32::MAX;
    for (i, &candidate) in table.iter().enumerate() {
        let d = (value as i32 - candidate as i32).unsigned_abs();
        if d < best_d {
            best_d = d;
            best_i = i;
        }
    }
    best_i
}

fn nearest_rgb_index(r: u8, g: u8, b: u8, table: &[(u8, u8, u8); 4]) -> usize {
    let mut best_i = 0usize;
    let mut best_d = u32::MAX;
    for (i, &(tr, tg, tb)) in table.iter().enumerate() {
        let dr = r as i32 - tr as i32;
        let dg = g as i32 - tg as i32;
        let db = b as i32 - tb as i32;
        let d = (dr * dr + dg * dg + db * db) as u32;
        if d < best_d {
            best_d = d;
            best_i = i;
        }
    }
    best_i
}

fn encode_dxt5_from_argb(pixels: &[u32], width: u32, height: u32) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || width % 4 != 0 || height % 4 != 0 {
        return None;
    }
    if pixels.len() < (width as usize).saturating_mul(height as usize) {
        return None;
    }

    let blocks_x = width / 4;
    let blocks_y = height / 4;
    let mut out = Vec::with_capacity((blocks_x * blocks_y * 16) as usize);

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let mut block = [0u32; 16];
            let mut min_a = u8::MAX;
            let mut max_a = 0u8;
            let mut min_luma = u32::MAX;
            let mut max_luma = 0u32;
            let mut min_rgb = (0u8, 0u8, 0u8);
            let mut max_rgb = (0u8, 0u8, 0u8);

            for py in 0..4 {
                for px in 0..4 {
                    let idx = ((by * 4 + py) * width + (bx * 4 + px)) as usize;
                    let block_idx = (py * 4 + px) as usize;
                    let p = pixels[idx];
                    block[block_idx] = p;
                    let (a, r, g, b) = argb_components(p);
                    min_a = min_a.min(a);
                    max_a = max_a.max(a);
                    if a > 8 {
                        let luma = 77u32 * r as u32 + 150u32 * g as u32 + 29u32 * b as u32;
                        if luma < min_luma {
                            min_luma = luma;
                            min_rgb = (r, g, b);
                        }
                        if luma > max_luma {
                            max_luma = luma;
                            max_rgb = (r, g, b);
                        }
                    }
                }
            }

            if min_luma == u32::MAX {
                min_luma = u32::MAX;
                max_luma = 0;
                for &p in &block {
                    let (_a, r, g, b) = argb_components(p);
                    let luma = 77u32 * r as u32 + 150u32 * g as u32 + 29u32 * b as u32;
                    if luma < min_luma {
                        min_luma = luma;
                        min_rgb = (r, g, b);
                    }
                    if luma > max_luma {
                        max_luma = luma;
                        max_rgb = (r, g, b);
                    }
                }
            }

            let a0 = max_a;
            let a1 = min_a;
            let alpha_table = [
                a0,
                a1,
                ((6 * a0 as u16 + a1 as u16) / 7) as u8,
                ((5 * a0 as u16 + 2 * a1 as u16) / 7) as u8,
                ((4 * a0 as u16 + 3 * a1 as u16) / 7) as u8,
                ((3 * a0 as u16 + 4 * a1 as u16) / 7) as u8,
                ((2 * a0 as u16 + 5 * a1 as u16) / 7) as u8,
                ((a0 as u16 + 6 * a1 as u16) / 7) as u8,
            ];
            out.push(a0);
            out.push(a1);

            let mut alpha_bits = 0u64;
            for (i, &p) in block.iter().enumerate() {
                let (a, _r, _g, _b) = argb_components(p);
                let ai = nearest_byte_index(a, &alpha_table) as u64;
                alpha_bits |= ai << (3 * i);
            }
            for i in 0..6 {
                out.push(((alpha_bits >> (8 * i)) & 0xFF) as u8);
            }

            let mut c0 = rgb888_to_rgb565(max_rgb.0, max_rgb.1, max_rgb.2);
            let mut c1 = rgb888_to_rgb565(min_rgb.0, min_rgb.1, min_rgb.2);
            if c0 < c1 {
                std::mem::swap(&mut c0, &mut c1);
            }
            let (r0, g0, b0) = rgb565_to_rgb888(c0);
            let (r1, g1, b1) = rgb565_to_rgb888(c1);
            let color_table = [
                (r0, g0, b0),
                (r1, g1, b1),
                (
                    ((2 * r0 as u16 + r1 as u16) / 3) as u8,
                    ((2 * g0 as u16 + g1 as u16) / 3) as u8,
                    ((2 * b0 as u16 + b1 as u16) / 3) as u8,
                ),
                (
                    ((r0 as u16 + 2 * r1 as u16) / 3) as u8,
                    ((g0 as u16 + 2 * g1 as u16) / 3) as u8,
                    ((b0 as u16 + 2 * b1 as u16) / 3) as u8,
                ),
            ];
            out.extend_from_slice(&c0.to_le_bytes());
            out.extend_from_slice(&c1.to_le_bytes());

            let mut color_bits = 0u32;
            for (i, &p) in block.iter().enumerate() {
                let (_a, r, g, b) = argb_components(p);
                let ci = nearest_rgb_index(r, g, b, &color_table) as u32;
                color_bits |= ci << (2 * i);
            }
            out.extend_from_slice(&color_bits.to_le_bytes());
        }
    }

    Some(out)
}

fn spidey_rebuild_widget_mip_target(
    guest_mem: *mut u8,
    target: HleTextureInfo,
    source_data: u32,
    source_w: u32,
    source_h: u32,
    source_bytes: &[u8],
) -> Option<Vec<u8>> {
    if target.format != crate::xbox::gpu::texture_format::X_D3DFMT_DXT5
        || target.width == 0
        || target.height == 0
    {
        return None;
    }
    let decoded = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
        crate::xbox::gpu::texture_format::X_D3DFMT_DXT5,
        source_w,
        source_h,
        source_bytes,
    )?;
    let downsampled =
        downsample_argb_box(&decoded, source_w, source_h, target.width, target.height)?;
    let encoded = encode_dxt5_from_argb(&downsampled, target.width, target.height)?;
    let expected = texture_byte_len(target.width, target.height, target.format) as usize;
    if expected == 0 || encoded.len() != expected {
        return None;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            encoded.as_ptr(),
            guest_mem.add(target.data as usize),
            encoded.len(),
        );
    }
    upsert_texture_info(target);
    debug_log(&format!(
        "[SPIDEY-WIDGET-TEX-MIP-FIX] target=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt={} source_data=0x{:08X} wrote={} bytes",
        target.key | 0x8000_0000,
        target.key,
        target.data,
        target.width,
        target.height,
        crate::xbox::gpu::texture_format::format_name(target.format),
        source_data,
        encoded.len()
    ));
    Some(encoded)
}

fn log_dxt5_upload_source64(
    label: &str,
    stage: u32,
    tex_raw: u32,
    tex_norm: u32,
    data_raw: u32,
    data_addr: u32,
    payload_addr: u32,
    width: u32,
    height: u32,
    fmt_code: u32,
    bytes: &[u8],
) {
    if stage != 0 || fmt_code != crate::xbox::gpu::texture_format::X_D3DFMT_DXT5 {
        return;
    }

    static DXT5_UPLOAD_SEQ: AtomicU32 = AtomicU32::new(0);
    let dxt5_seq = DXT5_UPLOAD_SEQ.fetch_add(1, Ordering::Relaxed) + 1;

    // The title background starts with a 128x256 DXT5 tile, followed by the
    // 256x256 working tile and then the black top-right tile. Capture that
    // short burst each time it appears so the source bytes can be compared.
    static TITLE_TILE_ARM: AtomicU32 = AtomicU32::new(0);
    let mut should_log = false;
    if width == 128 && height == 256 {
        TITLE_TILE_ARM.store(6, Ordering::Relaxed);
        should_log = true;
    }
    let armed = TITLE_TILE_ARM.load(Ordering::Relaxed);
    if armed > 0 {
        TITLE_TILE_ARM.store(armed.saturating_sub(1), Ordering::Relaxed);
        should_log = true;
    }
    if !should_log {
        return;
    }

    static SRC64_LOG_N: AtomicU32 = AtomicU32::new(0);
    let n = SRC64_LOG_N.fetch_add(1, Ordering::Relaxed);
    if n >= 384 {
        return;
    }

    use std::fmt::Write as _;

    let mut head = String::with_capacity(64 * 3);
    for (i, b) in bytes.iter().take(64).enumerate() {
        if i > 0 {
            head.push(' ');
        }
        let _ = write!(&mut head, "{:02X}", b);
    }
    let nonzero64 = bytes.iter().take(64).filter(|b| **b != 0).count();
    let sample_nonzero = sampled_nonzero_bytes(bytes);
    let swap = SWAP_COUNT_GLOBAL.load(Ordering::Relaxed);

    debug_log(&format!(
        "[HLE-TEX-SRC64] #{} dxt5_seq={} swap={} label={} stage={} tex=0x{:08X}->0x{:08X} data_raw=0x{:08X} data=0x{:08X} payload=0x{:08X} {}x{} fmt=0x{:02X}/{} bytes={} nz64={} sample_nz={}/1024 head64={}",
        n,
        dxt5_seq,
        swap,
        label,
        stage,
        tex_raw,
        tex_norm,
        data_raw,
        data_addr,
        payload_addr,
        width,
        height,
        fmt_code,
        crate::xbox::gpu::texture_format::format_name(fmt_code),
        bytes.len(),
        nonzero64,
        sample_nonzero,
        head
    ));

    log_dxt5_first_block_probe(
        label,
        stage,
        tex_raw,
        tex_norm,
        data_addr,
        payload_addr,
        width,
        height,
        bytes,
    );
    dump_dxt5_upload_bmp_probe(payload_addr, width, height, fmt_code, bytes);
}

fn rgb565_to_rgb888(c: u16) -> (u8, u8, u8) {
    let r5 = ((c >> 11) & 0x1F) as u32;
    let g6 = ((c >> 5) & 0x3F) as u32;
    let b5 = (c & 0x1F) as u32;
    (
        ((r5 * 255 + 15) / 31) as u8,
        ((g6 * 255 + 31) / 63) as u8,
        ((b5 * 255 + 15) / 31) as u8,
    )
}

fn log_dxt5_first_block_probe(
    label: &str,
    stage: u32,
    tex_raw: u32,
    tex_norm: u32,
    data_addr: u32,
    payload_addr: u32,
    width: u32,
    height: u32,
    bytes: &[u8],
) {
    if stage != 0 || bytes.len() < 16 {
        return;
    }

    let (tile_name, bit) = match payload_addr {
        0x04D6_8000 => ("working-top", 1),
        0x04D7_8000 => ("black-top-right", 2),
        0x03E1_06B0 => ("dynamic-red-left", 4),
        0x0561_8000 => ("dynamic-red-mid", 8),
        0x0562_8000 => ("dynamic-red-right", 16),
        _ => return,
    };

    static BLOCK_PROBE_MASK: AtomicU32 = AtomicU32::new(0);
    let old = BLOCK_PROBE_MASK.fetch_or(bit, Ordering::Relaxed);
    if (old & bit) != 0 {
        return;
    }

    use std::fmt::Write as _;

    let mut first16 = String::with_capacity(16 * 3);
    for (i, b) in bytes.iter().take(16).enumerate() {
        if i > 0 {
            first16.push(' ');
        }
        let _ = write!(&mut first16, "{:02X}", b);
    }

    let alpha0 = bytes[0];
    let alpha1 = bytes[1];
    let mut alpha_indices = 0u64;
    for i in 0..6 {
        alpha_indices |= (bytes[2 + i] as u64) << (8 * i);
    }
    let c0 = u16::from_le_bytes([bytes[8], bytes[9]]);
    let c1 = u16::from_le_bytes([bytes[10], bytes[11]]);
    let color_indices = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    let (r0, g0, b0) = rgb565_to_rgb888(c0);
    let (r1, g1, b1) = rgb565_to_rgb888(c1);

    debug_log(&format!(
        "[HLE-DXT5-BLOCK0] tile={} label={} tex=0x{:08X}->0x{:08X} data=0x{:08X} payload=0x{:08X} {}x{} first16={} alpha0={} alpha1={} alpha_idx=0x{:012X} c0=0x{:04X} rgb0=({},{},{}) c1=0x{:04X} rgb1=({},{},{}) color_idx=0x{:08X}",
        tile_name,
        label,
        tex_raw,
        tex_norm,
        data_addr,
        payload_addr,
        width,
        height,
        first16,
        alpha0,
        alpha1,
        alpha_indices,
        c0,
        r0,
        g0,
        b0,
        c1,
        r1,
        g1,
        b1,
        color_indices
    ));
}

fn dump_dxt5_upload_bmp_probe(
    payload_addr: u32,
    width: u32,
    height: u32,
    fmt_code: u32,
    bytes: &[u8],
) {
    let (bit, path, tile_name) = match payload_addr {
        0x04D6_8000 => (
            1,
            r"./tile_working_04D68000.bmp",
            "working-top",
        ),
        0x04D7_8000 => (
            2,
            r"./tile_black_04D78000.bmp",
            "black-top-right",
        ),
        0x03E1_06B0 => (
            4,
            r"./tile_dynamic_red_03E106B0.bmp",
            "dynamic-red-left",
        ),
        0x0561_8000 => (
            8,
            r"./tile_dynamic_red_05618000.bmp",
            "dynamic-red-mid",
        ),
        0x0562_8000 => (
            16,
            r"./tile_dynamic_red_05628000.bmp",
            "dynamic-red-right",
        ),
        _ => return,
    };

    static BMP_DUMP_MASK: AtomicU32 = AtomicU32::new(0);
    let old = BMP_DUMP_MASK.fetch_or(bit, Ordering::Relaxed);
    if (old & bit) != 0 {
        return;
    }

    let Some(mut pixels) = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
        fmt_code, width, height, bytes,
    ) else {
        debug_log(&format!(
            "[HLE-DXT5-BMP-DUMP-FAIL] tile={} payload=0x{:08X} {}x{} fmt=0x{:02X} bytes={}",
            tile_name,
            payload_addr,
            width,
            height,
            fmt_code,
            bytes.len()
        ));
        return;
    };
    let sample_step = (pixels.len() / 1024).max(1);
    let alpha_sample = pixels
        .iter()
        .step_by(sample_step)
        .take(1024)
        .filter(|px| (**px & 0xFF00_0000) != 0)
        .count();
    let rgb_before = pixels
        .iter()
        .step_by(sample_step)
        .take(1024)
        .filter(|px| (**px & 0x00FF_FFFF) != 0)
        .count();
    let promoted = crate::xbox::gpu::texture_format::promote_alpha_mask_rgb(&mut pixels);
    let rgb_after = pixels
        .iter()
        .step_by(sample_step)
        .take(1024)
        .filter(|px| (**px & 0x00FF_FFFF) != 0)
        .count();
    write_readback_bmp(path, width, height, &pixels);
    debug_log(&format!(
        "[HLE-DXT5-BMP-DUMP] tile={} payload=0x{:08X} {}x{} fmt=0x{:02X} bytes={} alpha_sample={} rgb_sample={}=>{} promoted={} path={}",
        tile_name,
        payload_addr,
        width,
        height,
        fmt_code,
        bytes.len(),
        alpha_sample,
        rgb_before,
        rgb_after,
        promoted,
        path
    ));
}

fn sampled_argb_stats(pixels: &[u32]) -> (usize, usize, usize, u32, u32) {
    let sample_step = (pixels.len() / 1024).max(1);
    let mut nonzero = 0usize;
    let mut alpha = 0usize;
    let mut rgb = 0usize;
    for px in pixels.iter().step_by(sample_step).take(1024) {
        if *px != 0 {
            nonzero += 1;
        }
        if (*px & 0xFF00_0000) != 0 {
            alpha += 1;
        }
        if (*px & 0x00FF_FFFF) != 0 {
            rgb += 1;
        }
    }
    let first = pixels.first().copied().unwrap_or(0);
    let mid = pixels
        .get(pixels.len().saturating_div(2))
        .copied()
        .unwrap_or(0);
    (nonzero, alpha, rgb, first, mid)
}

fn sampled_red_dominant_count(pixels: &[u32]) -> usize {
    let sample_step = (pixels.len() / 1024).max(1);
    pixels
        .iter()
        .step_by(sample_step)
        .take(1024)
        .filter(|&&px| {
            let r = (px >> 16) & 0xFF;
            let g = (px >> 8) & 0xFF;
            let b = px & 0xFF;
            r > 32 && r > g.saturating_mul(2) && r > b.saturating_mul(2)
        })
        .count()
}

fn is_spidey_red_web_like_frame(
    pixels: &[u32],
    sample_rgb: usize,
    first_px: u32,
    mid_px: u32,
) -> bool {
    if sample_rgb < 768 {
        return false;
    }
    let red_edge_or_center =
        ((first_px & 0x00FF_0000) >= 0x0010_0000) || ((mid_px & 0x00FF_0000) >= 0x0010_0000);
    red_edge_or_center && sampled_red_dominant_count(pixels) >= 256
}

fn read_spidey_scene_pair(guest_mem: *mut u8) -> (String, String) {
    fn read_scene_string(mem: *mut u8, addr: u32, max_len: usize) -> String {
        let mut bytes = Vec::with_capacity(max_len.min(64));
        for i in 0..max_len {
            let b = unsafe { *((mem as u64 + addr as u64 + i as u64) as *const u8) };
            if b == 0 {
                break;
            }
            if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                bytes.push(b);
            } else {
                break;
            }
        }
        String::from_utf8_lossy(&bytes).into_owned()
    }

    (
        read_scene_string(guest_mem, 0x004B_C848, 64),
        read_scene_string(guest_mem, 0x004B_C948, 64),
    )
}

fn maybe_cache_spidey_bonus_menu_present(guest_mem: *mut u8, pixels: &[u32], source: &str) {
    let (scene_name, stash_name) = read_spidey_scene_pair(guest_mem);
    if !scene_name.eq_ignore_ascii_case("bonus\\menu")
        || !stash_name.eq_ignore_ascii_case("M0menu\\menu")
    {
        return;
    }

    let (_sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) = sampled_argb_stats(pixels);
    if sample_alpha == 0
        || sample_rgb < 96
        || sample_rgb >= 768
        || has_legal_like_bright_text_block(pixels)
        || is_spidey_red_web_like_frame(pixels, sample_rgb, first_px, mid_px)
    {
        return;
    }

    SPIDEY_MENU_SELECTOR_SEEN.store(true, Ordering::Relaxed);
    *SPIDEY_MENU_SELECTOR_PRESENT_CACHE
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(pixels.to_vec());

    static GOOD_MENU_CACHE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = GOOD_MENU_CACHE_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-MENU-GOOD-PRESENT-CACHE] #{} source={} scene='{}' stash='{}' sample_rgb={} red_dominant={} first=0x{:08X} mid=0x{:08X}",
            n,
            source,
            scene_name,
            stash_name,
            sample_rgb,
            sampled_red_dominant_count(pixels),
            first_px,
            mid_px
        ));
    }
}

fn composite_nonblack_overlay(underlay: &[u32], overlay: &[u32]) -> Option<Vec<u32>> {
    if underlay.len() != overlay.len() {
        return None;
    }
    let mut out = underlay.to_vec();
    for (dst, src) in out.iter_mut().zip(overlay.iter().copied()) {
        if (src & 0xFF00_0000) != 0 && (src & 0x00FF_FFFF) != 0 {
            *dst = src;
        }
    }
    Some(out)
}

fn is_bright_menu_text(px: u32) -> bool {
    if (px & 0xFF00_0000) == 0 {
        return false;
    }
    let r = (px >> 16) & 0xFF;
    let g = (px >> 8) & 0xFF;
    let b = px & 0xFF;
    let luma = 77 * r + 150 * g + 29 * b;
    luma >= 46_000 && r >= 150 && g >= 140 && b >= 110
}

fn has_legal_like_bright_text_block(pixels: &[u32]) -> bool {
    const W: usize = 640;
    const H: usize = 480;
    if pixels.len() != W * H {
        return false;
    }

    let mut count = 0usize;
    let mut min_x = W;
    let mut min_y = H;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for (i, &px) in pixels.iter().enumerate() {
        if !is_bright_menu_text(px) {
            continue;
        }
        let x = i % W;
        let y = i / W;
        count += 1;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    count > 18_000 && min_y < 300 && max_y > 410 && max_x.saturating_sub(min_x) > 320
}

fn composite_menu_widget_overlay(underlay: &[u32], overlay: &[u32]) -> Option<Vec<u32>> {
    if underlay.len() != overlay.len() {
        return None;
    }
    let mut out = underlay.to_vec();
    for (dst, src) in out.iter_mut().zip(overlay.iter().copied()) {
        if (src & 0xFF00_0000) == 0 || (src & 0x00FF_FFFF) == 0 {
            continue;
        }
        if is_bright_menu_text(*dst) {
            continue;
        }
        *dst = src;
    }
    Some(out)
}

#[cfg(test)]
mod overlay_composite_tests {
    use super::{composite_menu_widget_overlay, composite_nonblack_overlay};

    #[test]
    fn composite_nonblack_overlay_rejects_length_mismatch() {
        let underlay = [0xFF10_2030, 0xFF40_5060];
        let overlay = [0xFFAA_BBCC];

        assert!(composite_nonblack_overlay(&underlay, &overlay).is_none());
    }

    #[test]
    fn composite_nonblack_overlay_uses_only_visible_nonblack_pixels() {
        let underlay = [0xFF10_2030, 0xFF40_5060, 0xFF70_8090, 0xFFA0_B0C0];
        let overlay = [0x0001_0203, 0xFF00_0000, 0xFFAA_BBCC, 0x7F11_2233];

        let composited = composite_nonblack_overlay(&underlay, &overlay).unwrap();

        assert_eq!(
            composited,
            [0xFF10_2030, 0xFF40_5060, 0xFFAA_BBCC, 0x7F11_2233]
        );
    }

    #[test]
    fn composite_menu_widget_overlay_preserves_bright_underlay_text() {
        let underlay = [0xFF10_2030, 0xFFF0_F0E0, 0xFF30_4050];
        let overlay = [0xFFAA_0000, 0xFF00_AA00, 0xFF00_00AA];

        let composited = composite_menu_widget_overlay(&underlay, &overlay).unwrap();

        assert_eq!(composited, [0xFFAA_0000, 0xFFF0_F0E0, 0xFF00_00AA]);
    }

    #[test]
    fn composite_menu_widget_overlay_ignores_transparent_and_black_pixels() {
        let underlay = [0xFF10_2030, 0xFF40_5060, 0xFF70_8090, 0xFFA0_B0C0];
        let overlay = [0x0001_0203, 0xFF00_0000, 0xFFAA_BBCC, 0x7F11_2233];

        let composited = composite_menu_widget_overlay(&underlay, &overlay).unwrap();

        assert_eq!(
            composited,
            [0xFF10_2030, 0xFF40_5060, 0xFFAA_BBCC, 0x7F11_2233]
        );
    }
}

fn is_near_white_argb(px: u32) -> bool {
    let r = (px >> 16) & 0xFF;
    let g = (px >> 8) & 0xFF;
    let b = px & 0xFF;
    r >= 236 && g >= 236 && b >= 236
}

fn sanitize_bonus_menu_white_panel(pixels: &[u32], fallback: Option<&[u32]>) -> Option<Vec<u32>> {
    const W: usize = 640;
    const H: usize = 480;
    if pixels.len() != W * H {
        return None;
    }

    let mut count = 0usize;
    let mut min_x = W;
    let mut min_y = H;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for (i, &px) in pixels.iter().enumerate() {
        if is_near_white_argb(px) {
            let x = i % W;
            let y = i / W;
            count += 1;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    if count < 20_000 || min_x < 200 || min_y < 180 || max_x < 500 || max_y < 420 {
        return None;
    }

    let fallback = fallback.filter(|f| f.len() == pixels.len());
    let mut out = pixels.to_vec();
    let mut replaced = 0usize;
    for (i, dst) in out.iter_mut().enumerate() {
        if !is_near_white_argb(*dst) {
            continue;
        }
        let replacement = fallback
            .and_then(|f| f.get(i).copied())
            .filter(|px| !is_near_white_argb(*px))
            .unwrap_or(0xFF00_0000);
        *dst = replacement | 0xFF00_0000;
        replaced += 1;
    }

    static WHITE_PANEL_FIX_LOG: AtomicU32 = AtomicU32::new(0);
    let n = WHITE_PANEL_FIX_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-MENU-WHITE-PANEL-FIX] #{} replaced={} bbox=[{},{}..{},{}] fallback={}",
            n,
            replaced,
            min_x,
            min_y,
            max_x,
            max_y,
            fallback.is_some() as u8
        ));
    }

    Some(out)
}

pub(crate) fn hle_visibility_test(name: &str, args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    static BEGIN_COUNT: AtomicU32 = AtomicU32::new(0);
    static END_COUNT: AtomicU32 = AtomicU32::new(0);
    static RESULT_COUNT: AtomicU32 = AtomicU32::new(0);
    static LAST_INDEX: AtomicU32 = AtomicU32::new(0);
    static VIS_TIMESTAMP: AtomicU32 = AtomicU32::new(1);

    let (active_rt, black_clear_without_draw) = {
        let gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref backend) = *gpu {
            (
                backend
                    .debug_active_render_target()
                    .map(|(key, width, height, data, _pitch)| (key, width, height, data)),
                backend.black_clear_without_draw_pending(),
            )
        } else {
            (None, false)
        }
    };
    let (rt_key, rt_w, rt_h, rt_data) = active_rt.unwrap_or((0, 0, 0, 0));

    match name {
        "D3DDevice_BeginVisibilityTest" => {
            let n = BEGIN_COUNT.fetch_add(1, Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE-VIS-BEGIN] #{} rt=0x{:08X} {}x{} data=0x{:08X} black_clear_without_draw={}",
                    n, rt_key, rt_w, rt_h, rt_data, black_clear_without_draw
                ));
            }
            0
        }
        "D3DDevice_EndVisibilityTest" => {
            let index = args[0] & 0x0FFF;
            LAST_INDEX.store(index, Ordering::Relaxed);
            let n = END_COUNT.fetch_add(1, Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE-VIS-END] #{} index={} raw_index=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} black_clear_without_draw={}",
                    n, index, args[0], rt_key, rt_w, rt_h, rt_data, black_clear_without_draw
                ));
            }
            0
        }
        "D3DDevice_GetVisibilityTestResult" => {
            let index = args[0] & 0x0FFF;
            let result_ptr = args[1];
            let timestamp_ptr = args[2];
            let timestamp = VIS_TIMESTAMP.fetch_add(1, Ordering::Relaxed);
            // We do not have the NV2A pixel counter yet. Return a completed,
            // visible result so games do not block forever on an unimplemented
            // query, while logging every call loudly enough to prove whether
            // this path is actually part of the Spider-Man menu/spider issue.
            let visible_pixels = 1u32;
            unsafe {
                if result_ptr >= 4 && result_ptr < 0x2000_0000 {
                    std::ptr::write_unaligned(
                        guest_mem.add(result_ptr as usize) as *mut u32,
                        visible_pixels,
                    );
                }
                if timestamp_ptr >= 4 && timestamp_ptr < 0x2000_0000 {
                    std::ptr::write_unaligned(
                        guest_mem.add(timestamp_ptr as usize) as *mut u32,
                        timestamp,
                    );
                }
            }
            LAST_INDEX.store(index, Ordering::Relaxed);
            let n = RESULT_COUNT.fetch_add(1, Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() {
                debug_log(&format!(
                    "[HLE-VIS-RESULT] #{} index={} raw_index=0x{:08X} pResult=0x{:08X}->{} pTimestamp=0x{:08X}->{} last_end_index={} rt=0x{:08X} {}x{} data=0x{:08X} black_clear_without_draw={}",
                    n,
                    index,
                    args[0],
                    result_ptr,
                    visible_pixels,
                    timestamp_ptr,
                    timestamp,
                    LAST_INDEX.load(Ordering::Relaxed),
                    rt_key,
                    rt_w,
                    rt_h,
                    rt_data,
                    black_clear_without_draw
                ));
            }
            0
        }
        _ => 0,
    }
}

fn read_f32_guest(guest_mem: *mut u8, addr: u32) -> Option<f32> {
    let addr = normalize_guest_ram_ptr(addr)?;
    if addr > 0x2000_0000u32.saturating_sub(4) {
        return None;
    }
    Some(unsafe { std::ptr::read_unaligned(guest_mem.add(addr as usize) as *const f32) })
}

fn write_f32_guest(guest_mem: *mut u8, addr: u32, value: f32) -> Option<()> {
    let addr = normalize_guest_ram_ptr(addr)?;
    if addr > 0x2000_0000u32.saturating_sub(4) {
        return None;
    }
    unsafe {
        std::ptr::write_unaligned(guest_mem.add(addr as usize) as *mut f32, value);
    }
    Some(())
}

fn read_u32_guest(guest_mem: *mut u8, addr: u32) -> Option<u32> {
    let addr = normalize_guest_ram_ptr(addr)?;
    if addr > 0x2000_0000u32.saturating_sub(4) {
        return None;
    }
    Some(unsafe { std::ptr::read_unaligned(guest_mem.add(addr as usize) as *const u32) })
}

fn plausible_ffp36_uv(v: f32) -> bool {
    v.is_finite() && (-4096.0..=4096.0).contains(&v)
}

fn decode_no_shader_stride36_vertex(
    guest_mem: *mut u8,
    addr: u32,
    stride: u32,
    shader: u32,
) -> Option<NV2AVertex> {
    if shader != 0 || stride != 36 {
        return None;
    }
    if std::env::var_os("RUSTEMU_FFP36_DECODE").is_none() {
        return None;
    }

    let raw_w_or_diffuse = read_u32_guest(guest_mem, addr.wrapping_add(12))?;
    let w_or_diffuse = f32::from_bits(raw_w_or_diffuse);
    let looks_like_color_at_12 =
        (raw_w_or_diffuse >> 24) != 0 || !w_or_diffuse.is_finite() || w_or_diffuse.abs() > 4096.0;
    if !looks_like_color_at_12 {
        return None;
    }

    let x = read_f32_guest(guest_mem, addr)?;
    let y = read_f32_guest(guest_mem, addr.wrapping_add(4))?;
    let z = read_f32_guest(guest_mem, addr.wrapping_add(8))?;
    let raw_16 = read_u32_guest(guest_mem, addr.wrapping_add(16)).unwrap_or(0);
    let raw_20 = read_u32_guest(guest_mem, addr.wrapping_add(20)).unwrap_or(0);
    let raw_24 = read_u32_guest(guest_mem, addr.wrapping_add(24)).unwrap_or(0);
    let raw_28 = read_u32_guest(guest_mem, addr.wrapping_add(28)).unwrap_or(0);
    let raw_32 = read_u32_guest(guest_mem, addr.wrapping_add(32)).unwrap_or(0);
    let f16 = f32::from_bits(raw_16);
    let f20 = f32::from_bits(raw_20);
    let f24 = f32::from_bits(raw_24);
    let f28 = f32::from_bits(raw_28);
    let f32 = f32::from_bits(raw_32);
    let slot16_looks_like_specular_or_padding =
        raw_16 == 0 || (raw_16 >> 24) != 0 || !plausible_ffp36_uv(f16) || f16.abs() < 1.0e-6;
    let (default_u, default_v, default_source) = if slot16_looks_like_specular_or_padding
        && plausible_ffp36_uv(f20)
        && plausible_ffp36_uv(f24)
    {
        (f20, f24, "20/24")
    } else if plausible_ffp36_uv(f16) && plausible_ffp36_uv(f20) {
        (f16, f20, "16/20")
    } else {
        (0.0, 0.0, "zero")
    };
    let forced_uv = std::env::var("RUSTEMU_FFP36_UV_OFFSET")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .and_then(|offset| match offset {
            16 if plausible_ffp36_uv(f16) && plausible_ffp36_uv(f20) => {
                Some((f16, f20, "env16/20"))
            }
            20 if plausible_ffp36_uv(f20) && plausible_ffp36_uv(f24) => {
                Some((f20, f24, "env20/24"))
            }
            24 if plausible_ffp36_uv(f24) && plausible_ffp36_uv(f28) => {
                Some((f24, f28, "env24/28"))
            }
            28 if plausible_ffp36_uv(f28) && plausible_ffp36_uv(f32) => {
                Some((f28, f32, "env28/32"))
            }
            _ => None,
        });
    let (u, v, uv_source) = forced_uv.unwrap_or((default_u, default_v, default_source));

    let fmt_pair = |a: f32, b: f32| -> String {
        format!(
            "({:.4},{:.4}){}",
            a,
            b,
            if plausible_ffp36_uv(a) && plausible_ffp36_uv(b) {
                ""
            } else {
                "!"
            }
        )
    };

    static FFP36_LOG: AtomicU32 = AtomicU32::new(0);
    let n = FFP36_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-FFP36-DECODE] #{} addr=0x{:08X} xyz=({:.3},{:.3},{:.3}) color=0x{:08X} raw16=0x{:08X} raw20=0x{:08X} raw24=0x{:08X} raw28=0x{:08X} raw32=0x{:08X} uv16_20={} uv20_24={} uv24_28={} uv28_32={} selected={} uv=({:.4},{:.4})",
            n,
            addr,
            x,
            y,
            z,
            raw_w_or_diffuse,
            raw_16,
            raw_20,
            raw_24,
            raw_28,
            raw_32,
            fmt_pair(f16, f20),
            fmt_pair(f20, f24),
            fmt_pair(f24, f28),
            fmt_pair(f28, f32),
            uv_source,
            u,
            v
        ));
    }

    Some(NV2AVertex {
        x,
        y,
        z,
        w: 1.0,
        color: raw_w_or_diffuse,
        u,
        v,
    })
}

fn light_type_name(t: u32) -> &'static str {
    match t {
        1 => "POINT",
        2 => "SPOT",
        3 => "DIRECTIONAL",
        _ => "UNKNOWN",
    }
}

pub(super) fn hle_note_light_enable(index: u32, enable: u32) -> u32 {
    if index < 32 {
        let bit = 1u32 << index;
        if enable != 0 {
            LIGHT_ENABLE_MASK.fetch_or(bit, Ordering::Relaxed);
        } else {
            LIGHT_ENABLE_MASK.fetch_and(!bit, Ordering::Relaxed);
        }
    }
    let seq = LIGHT_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    static LIGHT_ENABLE_LOG: AtomicU32 = AtomicU32::new(0);
    let n = LIGHT_ENABLE_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-LIGHT-ENABLE] seq={} index={} enable={} mask=0x{:08X}",
            seq,
            index,
            enable,
            LIGHT_ENABLE_MASK.load(Ordering::Relaxed)
        ));
    }
    0
}

pub(super) fn hle_note_set_light(index: u32, light_ptr: u32, guest_mem: *mut u8) -> u32 {
    let typ = read_u32_guest(guest_mem, light_ptr).unwrap_or(0);
    let diffuse = [
        read_f32_guest(guest_mem, light_ptr.wrapping_add(4)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(8)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(12)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(16)).unwrap_or(0.0),
    ];
    let ambient = [
        read_f32_guest(guest_mem, light_ptr.wrapping_add(36)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(40)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(44)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(48)).unwrap_or(0.0),
    ];
    let pos = [
        read_f32_guest(guest_mem, light_ptr.wrapping_add(52)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(56)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(60)).unwrap_or(0.0),
    ];
    let dir = [
        read_f32_guest(guest_mem, light_ptr.wrapping_add(64)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(68)).unwrap_or(0.0),
        read_f32_guest(guest_mem, light_ptr.wrapping_add(72)).unwrap_or(0.0),
    ];
    let range = read_f32_guest(guest_mem, light_ptr.wrapping_add(76)).unwrap_or(0.0);
    let seq = LIGHT_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    static SET_LIGHT_LOG: AtomicU32 = AtomicU32::new(0);
    let n = SET_LIGHT_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-SET-LIGHT] seq={} index={} ptr=0x{:08X} type={}({}) diffuse=[{:.3},{:.3},{:.3},{:.3}] ambient=[{:.3},{:.3},{:.3},{:.3}] pos=[{:.3},{:.3},{:.3}] dir=[{:.3},{:.3},{:.3}] range={:.3} enabled_mask=0x{:08X}",
            seq,
            index,
            light_ptr,
            typ,
            light_type_name(typ),
            diffuse[0],
            diffuse[1],
            diffuse[2],
            diffuse[3],
            ambient[0],
            ambient[1],
            ambient[2],
            ambient[3],
            pos[0],
            pos[1],
            pos[2],
            dir[0],
            dir[1],
            dir[2],
            range,
            LIGHT_ENABLE_MASK.load(Ordering::Relaxed)
        ));
    }
    0
}

pub(super) fn hle_note_set_material(material_ptr: u32, guest_mem: *mut u8) -> u32 {
    let diffuse = [
        read_f32_guest(guest_mem, material_ptr).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(4)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(8)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(12)).unwrap_or(0.0),
    ];
    let ambient = [
        read_f32_guest(guest_mem, material_ptr.wrapping_add(16)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(20)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(24)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(28)).unwrap_or(0.0),
    ];
    let emissive = [
        read_f32_guest(guest_mem, material_ptr.wrapping_add(48)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(52)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(56)).unwrap_or(0.0),
        read_f32_guest(guest_mem, material_ptr.wrapping_add(60)).unwrap_or(0.0),
    ];
    let power = read_f32_guest(guest_mem, material_ptr.wrapping_add(64)).unwrap_or(0.0);
    let seq = MATERIAL_STATE_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    static MATERIAL_LOG: AtomicU32 = AtomicU32::new(0);
    let n = MATERIAL_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-SET-MATERIAL] seq={} ptr=0x{:08X} diffuse=[{:.3},{:.3},{:.3},{:.3}] ambient=[{:.3},{:.3},{:.3},{:.3}] emissive=[{:.3},{:.3},{:.3},{:.3}] power={:.3}",
            seq,
            material_ptr,
            diffuse[0],
            diffuse[1],
            diffuse[2],
            diffuse[3],
            ambient[0],
            ambient[1],
            ambient[2],
            ambient[3],
            emissive[0],
            emissive[1],
            emissive[2],
            emissive[3],
            power
        ));
    }
    0
}

pub(super) fn hle_set_texture(args: &[u32; 8], guest_mem: *mut u8) {
    hle_set_texture_impl(args, guest_mem, false);
}

fn hle_set_texture_impl(args: &[u32; 8], guest_mem: *mut u8, force_upload: bool) {
    static SETTEX_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static SETTEX_NULL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static SETTEX_NONNULL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let stage = args[0];
    let p_texture = args[1];
    let tex_ptr_norm = normalize_guest_ram_ptr(p_texture);
    let prev_stage0_raw = CURRENT_STAGE0_TEXTURE_RAW.load(Ordering::Relaxed);
    let prev_stage0_norm = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    if stage == 0
        && p_texture == 0
        && is_spidey_select_circle_texture(prev_stage0_raw, prev_stage0_norm)
    {
        debug_log(&format!(
            "[SPIDEY-CIRCLE-UNBIND] tex=0x{:08X}->0x{:08X}",
            prev_stage0_raw, prev_stage0_norm
        ));
    }

    drain_draws_before_texture_state_change(stage, p_texture);

    set_active_texture_binding(stage, tex_ptr_norm.unwrap_or(0));
    if stage == 0 {
        CURRENT_STAGE0_TEXTURE.store(tex_ptr_norm.unwrap_or(0), Ordering::Relaxed);
        CURRENT_STAGE0_TEXTURE_RAW.store(p_texture, Ordering::Relaxed);
        if p_texture == 0 {
            CURRENT_STAGE0_TEXTURE_DATA.store(0, Ordering::Relaxed);
            CURRENT_STAGE0_TEXTURE_FORMAT.store(0, Ordering::Relaxed);
        }
    }

    if p_texture == 0 {
        SETTEX_NULL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            hle_settex_cache_clear(stage);
            backend.clear_texture(stage);
        }
    } else {
        let n_nn = SETTEX_NONNULL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // Log first 20 NON-NULL SetTexture calls with their header
        if n_nn < 20 {
            // Read 20-byte header safely
            if let Some(tex_ptr) = tex_ptr_norm {
                let common = unsafe {
                    std::ptr::read_unaligned((guest_mem as u64 + tex_ptr as u64) as *const u32)
                };
                let data = unsafe {
                    std::ptr::read_unaligned((guest_mem as u64 + tex_ptr as u64 + 4) as *const u32)
                };
                let format = unsafe {
                    std::ptr::read_unaligned((guest_mem as u64 + tex_ptr as u64 + 12) as *const u32)
                };
                crate::xbox::aot::veh::veh_log(&format!(
                    "[HLE-SETTEX] #{} stage={} pTex=0x{:08X}->0x{:08X} Common=0x{:08X} Data=0x{:08X} Format=0x{:08X}",
                    n_nn, stage, p_texture, tex_ptr, common, data, format
                ));
            }
        }
    }

    // Every 100K calls, dump tally
    let n = SETTEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n % 100_000 == 0 {
        let nulls = SETTEX_NULL.load(std::sync::atomic::Ordering::Relaxed);
        let nns = SETTEX_NONNULL.load(std::sync::atomic::Ordering::Relaxed);
        crate::xbox::aot::veh::veh_log(&format!(
            "[HLE-SETTEX-TALLY] total={} null={} nonnull={}",
            n + 1,
            nulls,
            nns
        ));
    }

    use crate::xbox::aot::oovpa_d3d::*;

    let stage = args[0];
    let tex_ptr_raw = args[1]; // guest address of Xbox D3DTexture object

    let Some(tex_ptr) = normalize_guest_ram_ptr(tex_ptr_raw) else {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            hle_settex_cache_clear(stage);
            backend.clear_texture(stage);
        }
        return; // NULL or invalid texture
    };

    // SELF-HEALING: re-read the texture's live [tex+0x04] Data field every
    // call. Xbox IDirect3DResource8::Register and XGSetTextureHeader write
    // the real physical-mirror pointer into this slot AFTER we registered
    // the texture via CreateTexture/LockRect. Strip the mirror bit and the
    // two low control flags to get the clean guest offset.
    let live_data =
        unsafe { std::ptr::read_unaligned((guest_mem as u64 + tex_ptr as u64 + 4) as *const u32) };
    let live_data_clean = live_data & 0x1FFF_FFFC;
    let live_format =
        unsafe { std::ptr::read_unaligned((guest_mem as u64 + tex_ptr as u64 + 12) as *const u32) };
    let fmt_code = xbox_texture_format_code(live_format);
    if stage == 0 {
        CURRENT_STAGE0_TEXTURE_DATA.store(live_data_clean, Ordering::Relaxed);
        CURRENT_STAGE0_TEXTURE_FORMAT.store(fmt_code, Ordering::Relaxed);
    }
    {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            backend.set_texture_debug_info(stage, tex_ptr_raw, tex_ptr, live_data_clean, fmt_code);
        }
    }

    // Render-to-texture surfaces do not necessarily have guest-visible pixel
    // storage in X_D3DResource.Data. If SetRenderTarget created a host RT for
    // this key, bind that D3D11 SRV directly instead of falling through to the
    // normal guest-memory texture upload path.
    {
        let registry_info = lookup_texture_info(tex_ptr);
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            let mut bound = backend.bind_render_target_texture(stage, tex_ptr);
            let mut bind_source = "key";
            if !bound {
                if let Some(info) = registry_info {
                    let data_key = info.data & !0x3;
                    if data_key != 0 {
                        bound = backend.bind_render_target_texture(stage, data_key);
                        if bound {
                            bind_source = "registry-data-key";
                        }
                    }
                }
            }
            if !bound && live_data_clean != 0 {
                bound = backend.bind_render_target_texture(stage, live_data_clean);
                if bound {
                    bind_source = "live-data-key";
                }
            }
            if !bound && live_data_clean != 0 {
                bound = backend.bind_render_target_texture_by_data(stage, live_data_clean);
                if bound {
                    bind_source = "live-data-range";
                }
            }
            if !bound {
                if let Some(info) = registry_info {
                    let data_key = info.data & !0x3;
                    if data_key != 0 {
                        bound = backend.bind_render_target_texture_by_data(stage, data_key);
                        if bound {
                            bind_source = "registry-data-range";
                        }
                    }
                }
            }
            if bound {
                hle_settex_cache_clear(stage);
                backend.set_texture_debug_info(
                    stage,
                    tex_ptr_raw,
                    tex_ptr,
                    live_data_clean,
                    fmt_code,
                );
                static RT_BIND_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = RT_BIND_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() || bind_source.ends_with("range") {
                    debug_log(&format!(
                        "[HLE-SETTEX-RT] #{} source={} stage={} tex=0x{:08X}->0x{:08X} live_data=0x{:08X}->0x{:08X} data_key=0x{:08X} fmt=0x{:02X}/{}",
                        n,
                        bind_source,
                        stage,
                        tex_ptr_raw,
                        tex_ptr,
                        live_data,
                        live_data_clean,
                        registry_info.map(|info| info.data & !0x3).unwrap_or(0),
                        fmt_code,
                        crate::xbox::gpu::texture_format::format_name(fmt_code)
                    ));
                }
                return;
            }
        }
    }

    log_spidey_select_circle_bind(stage, tex_ptr_raw, tex_ptr, live_data_clean, fmt_code);
    let watch_texture = spidey_watch_texture(stage, tex_ptr_raw, tex_ptr);
    let focus_overlay_texture = spidey_focus_overlay_texture(stage, tex_ptr_raw, tex_ptr);
    if watch_texture {
        static SPIDEY_TEX_WATCH_HEADER_N: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let k = SPIDEY_TEX_WATCH_HEADER_N.fetch_add(1, Ordering::Relaxed);
        if focus_overlay_texture || k < 32 || k.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-TEX-WATCH] #{} stage={} tex=0x{:08X}->0x{:08X} live_data=0x{:08X}->0x{:08X} live_fmt=0x{:08X}/0x{:02X} active=0x{:08X}",
                k,
                stage,
                tex_ptr_raw,
                tex_ptr,
                live_data,
                live_data_clean,
                live_format,
                fmt_code,
                CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed)
            ));
        }
    }

    // Some XGRPH-created Spider-Man resources retain a stale/placeholder
    // D3D header format (0x2C looks like D16) while their Data field points
    // at the real decompressed DDS stream. Trust the organic DDS magic before
    // applying color/depth classification to the header.
    if let Some(data_addr) = normalize_guest_ram_ptr(live_data & !0x3) {
        let live_dds_info = read_guest_dds_info(guest_mem, data_addr);
        if live_dds_info.is_none() {
            static DDS_MISS_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = DDS_MISS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let zlib_header = if data_addr <= 0x1FFF_FFFE {
                unsafe {
                    let b0 = *guest_mem.add(data_addr as usize);
                    let b1 = *guest_mem.add(data_addr as usize + 1);
                    b0 == 0x78 && matches!(b1, 0x01 | 0x5E | 0x9C | 0xDA)
                }
            } else {
                false
            };
            if n < 8 && data_addr <= 0x1FFF_FFE0 {
                let mut head = [0u8; 16];
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        guest_mem.add(data_addr as usize),
                        head.as_mut_ptr(),
                        16,
                    );
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-TEX-DDS-MISS] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} \
                     header_fmt=0x{:08X}/0x{:02X} zlib={} head={:02X?}",
                    n,
                    stage,
                    tex_ptr_raw,
                    tex_ptr,
                    data_addr,
                    live_format,
                    fmt_code,
                    zlib_header,
                    head
                ));
            }
        }
        if let Some((dds_width, dds_height, dds_fmt, payload_offset, byte_len)) = live_dds_info {
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    guest_mem.add(data_addr as usize + payload_offset),
                    byte_len,
                )
            };
            static DDS_BIND_LIVE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = DDS_BIND_LIVE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-TEX-DDS-LIVE] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} \
                     header_fmt=0x{:08X}/0x{:02X} dds={}x{} fmt=0x{:02X} payload={} bytes",
                    n,
                    stage,
                    tex_ptr_raw,
                    tex_ptr,
                    data_addr,
                    live_format,
                    fmt_code,
                    dds_width,
                    dds_height,
                    dds_fmt,
                    byte_len
                ));
            }
            if watch_texture {
                static SPIDEY_TEX_WATCH_DDS_N: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let k = SPIDEY_TEX_WATCH_DDS_N.fetch_add(1, Ordering::Relaxed);
                if focus_overlay_texture || k < 32 || k.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-TEX-WATCH-DDS] #{} tex=0x{:08X}->0x{:08X} data=0x{:08X} dds={}x{} fmt=0x{:02X} payload={} bytes nonzero_sample={}/1024",
                        k,
                        tex_ptr_raw,
                        tex_ptr,
                        data_addr,
                        dds_width,
                        dds_height,
                        dds_fmt,
                        byte_len,
                        sampled_nonzero_bytes(bytes)
                    ));
                }
            }
            log_dxt5_upload_source64(
                "live-dds",
                stage,
                tex_ptr_raw,
                tex_ptr,
                live_data,
                data_addr,
                data_addr.saturating_add(payload_offset as u32),
                dds_width,
                dds_height,
                dds_fmt,
                bytes,
            );
            let upload_data = data_addr.saturating_add(payload_offset as u32);
            let upload_len = byte_len.min(u32::MAX as usize) as u32;
            if hle_settex_cache_is_redundant(
                stage,
                HLE_SETTEX_KIND_DDS,
                tex_ptr,
                upload_data,
                dds_width,
                dds_height,
                dds_fmt,
                upload_len,
                force_upload,
                "live-dds",
            ) {
                return;
            }
            let mut gpu = crate::xbox::gpu::gpu_lock();
            if let Some(ref mut backend) = *gpu {
                backend.set_texture_swizzle_hint(stage, false);
                backend.set_texture_raw(stage, dds_width, dds_height, dds_fmt, bytes);
                hle_settex_cache_note(
                    stage,
                    HLE_SETTEX_KIND_DDS,
                    tex_ptr,
                    upload_data,
                    dds_width,
                    dds_height,
                    dds_fmt,
                    upload_len,
                );
            }
            return;
        }
    }

    // X_D3DFMT_D16 = 0x2C (16-bit depth). Skip — depth buffers are not
    // sampleable color textures. Binding one as BGRA renders garbage.
    if crate::xbox::gpu::texture_format::is_depth_format(fmt_code) {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            hle_settex_cache_clear(stage);
            backend.clear_texture(stage);
        }
        return;
    }

    // ── Decode actual width/height from the live Format DWORD.
    //
    // From agent F02/D04 reverse-eng of the Xbox D3DTexture header:
    //   bits 20..23 = USize log2  →  width  = 1 << usize_log2
    //   bits 24..27 = VSize log2  →  height = 1 << vsize_log2
    //
    // The cached info.width/info.height in the texture registry was seeded
    // at XGRPH / XGSetTextureHeader time with placeholders, and never
    // refreshed when Register/CreateTexture overwrote [tex+0x0C]. Live-read
    // is the source of truth at SetTexture time.
    //
    let usize_log2 = ((live_format >> 20) & 0xF) as u32;
    let vsize_log2 = ((live_format >> 24) & 0xF) as u32;
    let bpp = texture_bytes_per_pixel(fmt_code).unwrap_or(4);
    let live_width = if (1..=12).contains(&usize_log2) {
        1u32 << usize_log2
    } else {
        0
    };
    let live_height = if (1..=12).contains(&vsize_log2) {
        1u32 << vsize_log2
    } else {
        0
    };

    if let Some(mut info) = lookup_texture_info(tex_ptr) {
        if live_width > 0 && live_height > 0 {
            // Override the (incorrectly-seeded) registry dims with the live
            // truth from [tex+0x0C]. Pitch for DXT* is (width/4) * 8 (DXT1)
            // or 16 (DXT3/5); for linear formats it's width * bpp.
            let live_pitch = texture_pitch_bytes(live_width, fmt_code);
            if info.width != live_width || info.height != live_height {
                static DIM_FIX_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let dn = DIM_FIX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if dn < 20 {
                    crate::xbox::aot::veh::veh_log(&format!(
                        "[HLE-SETTEX-DIMS] #{} tex=0x{:08X} fmt=0x{:08X} fmt_code=0x{:02X} bpp={} \
                         registry={}x{} pitch={} → live={}x{} pitch={} (USizeL2={} VSizeL2={})",
                        dn,
                        tex_ptr_raw,
                        live_format,
                        fmt_code,
                        bpp,
                        info.width,
                        info.height,
                        info.pitch,
                        live_width,
                        live_height,
                        live_pitch,
                        usize_log2,
                        vsize_log2
                    ));
                }
                info.width = live_width;
                info.height = live_height;
                info.pitch = live_pitch;
            }
            if live_format != 0 {
                info.format = live_format;
            }
        }

        // Patch the registry's Data field with the live-read value if it
        // changed (Register/XGSetTextureHeader pattern).
        if live_data_clean >= 0x1_0000
            && live_data_clean < 0x2000_0000
            && live_data_clean != (info.data & !0x3)
        {
            // Data changed! Log both values and what we think pBase must have been.
            debug_log(&format!(
                "[HLE-SETTEX-DATA-CHANGED] tex=0x{:08X} old=0x{:08X} new=0x{:08X} delta=0x{:08X}",
                tex_ptr_raw,
                info.data,
                live_data_clean,
                live_data_clean.wrapping_sub(info.data)
            ));
            info.data = live_data_clean;
        }
        if sane_texture_dims(info.width, info.height) {
            if let Some(data_addr) = normalize_guest_ram_ptr(info.data & !0x3) {
                let width = info.width;
                let height = info.height;
                let upload_format = if live_format != 0 {
                    live_format
                } else {
                    info.format
                };
                let upload_fmt_code = xbox_texture_format_code(upload_format);
                let is_block_compressed =
                    crate::xbox::gpu::texture_format::is_block_compressed(upload_fmt_code);
                if let Some((dds_width, dds_height, dds_fmt, payload_offset, byte_len)) =
                    read_guest_dds_info(guest_mem, data_addr)
                {
                    let bytes = unsafe {
                        std::slice::from_raw_parts(
                            guest_mem.add(data_addr as usize + payload_offset),
                            byte_len,
                        )
                    };
                    static DDS_BIND_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = DDS_BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 20 {
                        crate::xbox::emulator::debug_log(&format!(
                            "[HLE-TEX-DDS] #{} stage={} tex=0x{:08X}->0x{:08X} dds=0x{:08X} {}x{} fmt=0x{:02X} payload={} bytes",
                            n,
                            stage,
                            tex_ptr_raw,
                            tex_ptr,
                            data_addr,
                            dds_width,
                            dds_height,
                            dds_fmt,
                            byte_len
                        ));
                    }
                    log_dxt5_upload_source64(
                        "registry-dds",
                        stage,
                        tex_ptr_raw,
                        tex_ptr,
                        info.data,
                        data_addr,
                        data_addr.saturating_add(payload_offset as u32),
                        dds_width,
                        dds_height,
                        dds_fmt,
                        bytes,
                    );
                    let upload_data = data_addr.saturating_add(payload_offset as u32);
                    let upload_len = byte_len.min(u32::MAX as usize) as u32;
                    if hle_settex_cache_is_redundant(
                        stage,
                        HLE_SETTEX_KIND_DDS,
                        tex_ptr,
                        upload_data,
                        dds_width,
                        dds_height,
                        dds_fmt,
                        upload_len,
                        force_upload,
                        "registry-dds",
                    ) {
                        return;
                    }
                    let mut gpu = crate::xbox::gpu::gpu_lock();
                    if let Some(ref mut backend) = *gpu {
                        backend.set_texture_swizzle_hint(stage, false);
                        backend.set_texture_raw(stage, dds_width, dds_height, dds_fmt, bytes);
                        hle_settex_cache_note(
                            stage,
                            HLE_SETTEX_KIND_DDS,
                            tex_ptr,
                            upload_data,
                            dds_width,
                            dds_height,
                            dds_fmt,
                            upload_len,
                        );
                    }
                    return;
                }
                if is_block_compressed {
                    // DXTn resources are a linear array of 4x4 blocks on Xbox.
                    // Do not run the Morton unswizzler here; that is only for
                    // uncompressed swizzled pixel containers below.
                    let bytes_per_block =
                        crate::xbox::gpu::texture_format::block_bytes(upload_fmt_code)
                            .unwrap_or(16);
                    let block_width = (width + 3) / 4;
                    let block_rows = (height + 3) / 4;
                    let row_pitch = block_width.saturating_mul(bytes_per_block);
                    let byte_len = row_pitch.saturating_mul(block_rows);
                    let byte_end = data_addr as usize + byte_len as usize;
                    if byte_len > 0 && byte_len <= 16 * 1024 * 1024 && byte_end <= 0x2000_0000 {
                        let bytes = unsafe {
                            std::slice::from_raw_parts(
                                guest_mem.add(data_addr as usize),
                                byte_len as usize,
                            )
                        };
                        let sample_step = (bytes.len() / 1024).max(1);
                        let has_pixels = bytes
                            .iter()
                            .step_by(sample_step)
                            .take(1024)
                            .any(|b| *b != 0);
                        static REG_TEX_RAW_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = REG_TEX_RAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 20 {
                            crate::xbox::emulator::debug_log(&format!(
                                "[HLE-TEX-REG-RAW] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} row_pitch={} bytes={} fmt=0x{:08X}/0x{:02X} nonzero={}",
                                n, stage, tex_ptr_raw, tex_ptr, data_addr, width, height,
                                row_pitch, byte_len, upload_format, upload_fmt_code, has_pixels
                            ));
                        }
                        if spidey_title_ui_trace_enabled()
                            && has_pixels
                            && spidey_title_ui_candidate_texture(width, height)
                        {
                            let trace_n =
                                SPIDEY_TITLE_UI_TEX_TRACE_N.fetch_add(1, Ordering::Relaxed);
                            if trace_n < 192 || trace_n.is_power_of_two() {
                                let compressed_hash = fnv1a64(bytes);
                                let mut argb_hash = 0u64;
                                let mut sample_nonzero = 0usize;
                                let mut sample_alpha = 0usize;
                                let mut sample_rgb = 0usize;
                                let mut first_px = 0u32;
                                let mut mid_px = 0u32;
                                let dump_path = if let Some(pixels) =
                                    crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                                        upload_fmt_code,
                                        width,
                                        height,
                                        bytes,
                                    )
                                {
                                    argb_hash = {
                                        let mut hash = 0xcbf2_9ce4_8422_2325u64;
                                        for &px in &pixels {
                                            for byte in px.to_le_bytes() {
                                                hash ^= byte as u64;
                                                hash = hash
                                                    .wrapping_mul(0x0000_0100_0000_01B3);
                                            }
                                        }
                                        hash
                                    };
                                    let stats = sampled_argb_stats(&pixels);
                                    sample_nonzero = stats.0;
                                    sample_alpha = stats.1;
                                    sample_rgb = stats.2;
                                    first_px = stats.3;
                                    mid_px = stats.4;
                                    if trace_n < 64 {
                                        let path = format!(
                                            r"./spidey_title_ui_tex_{:03}_tex_{:08X}_{}x{}_fmt{:02X}_h{:016X}.bmp",
                                            trace_n,
                                            tex_ptr,
                                            width,
                                            height,
                                            upload_fmt_code,
                                            compressed_hash
                                        );
                                        write_readback_bmp(&path, width, height, &pixels);
                                        path
                                    } else {
                                        String::from("-")
                                    }
                                } else {
                                    String::from("-")
                                };
                                crate::xbox::emulator::debug_log(&format!(
                                    "[SPIDEY-TITLE-UI-TEX] #{} raw stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} row_pitch={} fmt=0x{:08X}/0x{:02X}/{} bytes={} compressed_hash=0x{:016X} argb_hash=0x{:016X} nonzero_bytes={}/1024 nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} source={} dump={}",
                                    trace_n,
                                    stage,
                                    tex_ptr_raw,
                                    tex_ptr,
                                    data_addr,
                                    width,
                                    height,
                                    row_pitch,
                                    upload_format,
                                    upload_fmt_code,
                                    crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                                    byte_len,
                                    compressed_hash,
                                    argb_hash,
                                    sampled_nonzero_bytes(bytes),
                                    sample_nonzero,
                                    sample_alpha,
                                    sample_rgb,
                                    first_px,
                                    mid_px,
                                    if (0x83E0_0000..0x84F0_0000).contains(&tex_ptr_raw) {
                                        "xbs-ish"
                                    } else {
                                        "runtime"
                                    },
                                    dump_path
                                ));
                            }
                        }
                        if watch_texture {
                            static SPIDEY_TEX_WATCH_RAW_N: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let k = SPIDEY_TEX_WATCH_RAW_N.fetch_add(1, Ordering::Relaxed);
                            if focus_overlay_texture || k < 32 || k.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-TEX-WATCH-RAW] #{} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} row_pitch={} bytes={} fmt=0x{:08X}/0x{:02X} nonzero_sample={}/1024",
                                    k,
                                    tex_ptr_raw,
                                    tex_ptr,
                                    data_addr,
                                    width,
                                    height,
                                    row_pitch,
                                    byte_len,
                                    upload_format,
                                    upload_fmt_code,
                                    sampled_nonzero_bytes(bytes)
                                ));
                            }
                            if focus_overlay_texture {
                                if let Some(mut pixels) =
                                    crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                                        upload_fmt_code,
                                        width,
                                        height,
                                        bytes,
                                    )
                                {
                                    let promoted = data_addr == 0x04DE_8000
                                        && crate::xbox::gpu::texture_format::promote_alpha_mask_rgb(
                                            &mut pixels,
                                        );
                                    let path = format!(
                                        r"./spidey_focus_tex_{:08X}_{}x{}.bmp",
                                        tex_ptr, width, height
                                    );
                                    write_readback_bmp(&path, width, height, &pixels);
                                    let (
                                        sample_nonzero,
                                        sample_alpha,
                                        sample_rgb,
                                        first_px,
                                        mid_px,
                                    ) = sampled_argb_stats(&pixels);
                                    debug_log(&format!(
                                        "[SPIDEY-TEX-FOCUS-DECODE] tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X}/0x{:02X}/{} nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} promoted={} dump={}",
                                        tex_ptr_raw,
                                        tex_ptr,
                                        data_addr,
                                        width,
                                        height,
                                        upload_format,
                                        upload_fmt_code,
                                        crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                                        sample_nonzero,
                                        sample_alpha,
                                        sample_rgb,
                                        first_px,
                                        mid_px,
                                        promoted,
                                        path
                                    ));
                                }
                            }
                        }
                        if focus_overlay_texture
                            && upload_fmt_code == crate::xbox::gpu::texture_format::X_D3DFMT_DXT5
                            && width == 64
                            && height == 64
                        {
                            let target_is_white =
                                crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                                    upload_fmt_code,
                                    width,
                                    height,
                                    bytes,
                                )
                                .map(|pixels| {
                                    let (_nonzero, alpha, rgb, first, mid) =
                                        sampled_argb_stats(&pixels);
                                    alpha == 1024
                                        && rgb == 1024
                                        && first == 0xFFFF_FFFF
                                        && mid == 0xFFFF_FFFF
                                })
                                .unwrap_or(false);

                            if target_is_white {
                                if let Some((src_key, src_w, src_h, src_data, src_bytes)) =
                                    spidey_find_loaded_rspiderc_source(guest_mem, data_addr)
                                {
                                    let target_info = HleTextureInfo {
                                        key: tex_ptr,
                                        width,
                                        height,
                                        format: upload_fmt_code,
                                        data: data_addr,
                                        pitch: row_pitch,
                                        swizzled: false,
                                    };
                                    if let Some(fixed_bytes) = spidey_rebuild_widget_mip_target(
                                        guest_mem,
                                        target_info,
                                        src_data,
                                        src_w,
                                        src_h,
                                        &src_bytes,
                                    ) {
                                        let mut gpu = crate::xbox::gpu::gpu_lock();
                                        if let Some(ref mut backend) = *gpu {
                                            backend.set_texture_swizzle_hint(stage, false);
                                            backend.set_texture_raw(
                                                stage,
                                                width,
                                                height,
                                                upload_fmt_code,
                                                &fixed_bytes,
                                            );
                                        }
                                        return;
                                    }

                                    static WIDGET_RESCUE_LOG: AtomicU32 = AtomicU32::new(0);
                                    let r = WIDGET_RESCUE_LOG.fetch_add(1, Ordering::Relaxed);
                                    if r < 16 || r.is_power_of_two() {
                                        debug_log(&format!(
                                            "[SPIDEY-WIDGET-TEX-RESCUE] #{} target=0x{:08X}->0x{:08X} data=0x{:08X} was_white=true source=0x{:08X} {}x{} data=0x{:08X} bytes={} reason=mip_fix_failed",
                                            r,
                                            tex_ptr_raw,
                                            tex_ptr,
                                            data_addr,
                                            src_key,
                                            src_w,
                                            src_h,
                                            src_data,
                                            src_bytes.len()
                                        ));
                                    }
                                } else {
                                    static WIDGET_RESCUE_MISS_LOG: AtomicU32 = AtomicU32::new(0);
                                    let r = WIDGET_RESCUE_MISS_LOG.fetch_add(1, Ordering::Relaxed);
                                    if r < 16 || r.is_power_of_two() {
                                        debug_log(&format!(
                                            "[SPIDEY-WIDGET-TEX-RESCUE-MISS] #{} target=0x{:08X}->0x{:08X} data=0x{:08X} was_white=true",
                                            r, tex_ptr_raw, tex_ptr, data_addr
                                        ));
                                    }
                                }
                            }
                        }
                        if hle_settex_cache_is_redundant(
                            stage,
                            HLE_SETTEX_KIND_RAW_BC,
                            tex_ptr,
                            data_addr,
                            width,
                            height,
                            upload_fmt_code,
                            byte_len,
                            force_upload,
                            "registry-raw-bc",
                        ) {
                            return;
                        }
                        log_dxt5_upload_source64(
                            "registry-raw",
                            stage,
                            tex_ptr_raw,
                            tex_ptr,
                            info.data,
                            data_addr,
                            data_addr,
                            width,
                            height,
                            upload_fmt_code,
                            bytes,
                        );
                        let mut gpu = crate::xbox::gpu::gpu_lock();
                        if let Some(ref mut backend) = *gpu {
                            if has_pixels {
                                backend.set_texture_swizzle_hint(
                                    stage,
                                    info.swizzled
                                        && crate::xbox::gpu::texture_format::is_swizzled_block_compressed(
                                            upload_fmt_code,
                                        ),
                                );
                                backend.set_texture_raw(
                                    stage,
                                    width,
                                    height,
                                    upload_fmt_code,
                                    bytes,
                                );
                                hle_settex_cache_note(
                                    stage,
                                    HLE_SETTEX_KIND_RAW_BC,
                                    tex_ptr,
                                    data_addr,
                                    width,
                                    height,
                                    upload_fmt_code,
                                    byte_len,
                                );
                            } else {
                                hle_settex_cache_clear(stage);
                                backend.clear_texture(stage);
                            }
                        }
                        return;
                    }
                }

                let bytes_per_pixel = texture_bytes_per_pixel(upload_fmt_code).unwrap_or(4);
                let format_swizzled_upload = bytes_per_pixel > 0
                    && !is_linear_texture_format(upload_fmt_code)
                    && width.is_power_of_two()
                    && height.is_power_of_two();
                let row_bytes = width.saturating_mul(bytes_per_pixel);
                let doom_linear_upload_probe = format_swizzled_upload
                    && doom_linear_upload_probe_enabled()
                    && stage == 0
                    && width == 512
                    && height == 256
                    && upload_fmt_code == crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
                    && bytes_per_pixel == 4
                    && info.pitch >= row_bytes;
                let swizzled_upload = format_swizzled_upload && !doom_linear_upload_probe;
                let pitch = if swizzled_upload {
                    row_bytes
                } else {
                    info.pitch.max(row_bytes)
                };
                let pixel_count = (width as usize).saturating_mul(height as usize);
                let pixel_byte_len = pixel_count.saturating_mul(bytes_per_pixel as usize);
                let last_row = (height - 1) as usize;
                let byte_end = if swizzled_upload {
                    data_addr as usize + pixel_byte_len
                } else {
                    data_addr as usize
                        + last_row.saturating_mul(pitch as usize)
                        + row_bytes as usize
                };
                if bytes_per_pixel > 0
                    && pixel_count > 0
                    && pixel_count <= 16 * 1024 * 1024
                    && pixel_byte_len <= 16 * 1024 * 1024
                    && byte_end <= 0x2000_0000
                {
                    let mut linear_bytes = vec![0u8; pixel_byte_len];
                    unsafe {
                        if swizzled_upload {
                            let swizzled = std::slice::from_raw_parts(
                                guest_mem.add(data_addr as usize),
                                pixel_byte_len,
                            );
                            doom_dump_swizzle_bisect(
                                stage,
                                tex_ptr_raw,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                upload_fmt_code,
                                bytes_per_pixel,
                                swizzled,
                            );
                            if let Some(unswizzled) = crate::xbox::gpu::swizzle::unswizzle(
                                swizzled,
                                width,
                                height,
                                bytes_per_pixel,
                            ) {
                                linear_bytes = unswizzled;
                            } else {
                                linear_bytes.copy_from_slice(swizzled);
                            }
                        } else {
                            if doom_linear_upload_probe {
                                static DOOM_LINEAR_UPLOAD_LOG: std::sync::atomic::AtomicU32 =
                                    std::sync::atomic::AtomicU32::new(0);
                                let n = DOOM_LINEAR_UPLOAD_LOG
                                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if n < 20 || n.is_power_of_two() {
                                    crate::xbox::emulator::debug_log(&format!(
                                        "[DOOM-LINEAR-UPLOAD-PROBE] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} pitch={} row_bytes={} fmt=0x{:02X}",
                                        n,
                                        stage,
                                        tex_ptr_raw,
                                        tex_ptr,
                                        data_addr,
                                        width,
                                        height,
                                        pitch,
                                        row_bytes,
                                        upload_fmt_code
                                    ));
                                }
                            }
                            for y in 0..height as usize {
                                let src = std::slice::from_raw_parts(
                                    guest_mem.add(data_addr as usize + y * pitch as usize),
                                    row_bytes as usize,
                                );
                                let dst_start = y * row_bytes as usize;
                                linear_bytes[dst_start..dst_start + row_bytes as usize]
                                    .copy_from_slice(src);
                            }
                        }
                    }
                    doom_record_texture_upload(
                        stage,
                        tex_ptr,
                        data_addr,
                        width,
                        height,
                        upload_fmt_code,
                        pitch,
                        bytes_per_pixel,
                        swizzled_upload,
                        &linear_bytes,
                        row_bytes,
                    );
                    let palette = active_palette_entries(stage, guest_mem);
                    let Some(mut pixels) = crate::xbox::gpu::texture_format::decode_linear_to_argb(
                        upload_fmt_code,
                        width,
                        height,
                        &linear_bytes,
                        palette.as_ref(),
                    ) else {
                        static TEX_DECODE_MISS_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n =
                            TEX_DECODE_MISS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 20 {
                            crate::xbox::emulator::debug_log(&format!(
                                "[HLE-TEX-DECODE-MISS] #{} stage={} tex=0x{:08X}->0x{:08X} fmt=0x{:08X}/0x{:02X}/{} {}x{} bytes={}",
                                n,
                                stage,
                                tex_ptr_raw,
                                tex_ptr,
                                upload_format,
                                upload_fmt_code,
                                crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                                width,
                                height,
                                linear_bytes.len()
                            ));
                        }
                        let mut gpu = crate::xbox::gpu::gpu_lock();
                        if let Some(ref mut backend) = *gpu {
                            hle_settex_cache_clear(stage);
                            backend.clear_texture(stage);
                        }
                        return;
                    };
                    let sample_step = (pixels.len() / 1024).max(1);
                    let sampled_alpha_all_zero = pixels
                        .iter()
                        .step_by(sample_step)
                        .take(1024)
                        .all(|px| (*px & 0xFF00_0000) == 0);
                    let force_opaque_alpha = matches!(
                        upload_fmt_code,
                        crate::xbox::gpu::texture_format::X_D3DFMT_X8R8G8B8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_X8R8G8B8
                    ) || (upload_fmt_code == 0x12
                        && width == 640
                        && height == 480
                        && sampled_alpha_all_zero);
                    if force_opaque_alpha {
                        for px in &mut pixels {
                            *px |= 0xFF00_0000;
                        }
                    }
                    let has_pixels = pixels
                        .iter()
                        .step_by(sample_step)
                        .take(1024)
                        .any(|px| *px != 0);
                    let sampled_nonzero_alpha = pixels
                        .iter()
                        .step_by(sample_step)
                        .take(1024)
                        .filter(|px| (**px & 0xFF00_0000) != 0)
                        .count();
                    let mut sampled_nonzero_rgb = pixels
                        .iter()
                        .step_by(sample_step)
                        .take(1024)
                        .filter(|px| (**px & 0x00FF_FFFF) != 0)
                        .count();
                    let alpha_only_format = matches!(
                        upload_fmt_code,
                        crate::xbox::gpu::texture_format::X_D3DFMT_A8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_A8L8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8L8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_AL8
                            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_AL8
                    );
                    if alpha_only_format && sampled_nonzero_alpha > 0 && sampled_nonzero_rgb == 0 {
                        static A8_PROMOTE_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let k = A8_PROMOTE_LOG.fetch_add(1, Ordering::Relaxed);
                        for px in &mut pixels {
                            if (*px & 0xFF00_0000) != 0 {
                                *px |= 0x00FF_FFFF;
                            }
                        }
                        sampled_nonzero_rgb = pixels
                            .iter()
                            .step_by(sample_step)
                            .take(1024)
                            .filter(|px| (**px & 0x00FF_FFFF) != 0)
                            .count();
                        if k < 16 || k.is_power_of_two() {
                            crate::xbox::emulator::debug_log(&format!(
                                "[HLE-TEX-A8-PROMOTE] #{} stage={} tex=0x{:08X}->0x{:08X} {}x{} fmt=0x{:02X} alpha_sample={}/1024",
                                k,
                                stage,
                                tex_ptr_raw,
                                tex_ptr,
                                width,
                                height,
                                upload_fmt_code,
                                sampled_nonzero_alpha
                            ));
                        }
                    }
                    doom_dump_upload_argb(
                        stage,
                        tex_ptr,
                        data_addr,
                        width,
                        height,
                        upload_fmt_code,
                        &pixels,
                    );
                    static REG_TEX_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = REG_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 20 {
                        crate::xbox::emulator::debug_log(&format!(
                            "[HLE-TEX-REG] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} pitch={} bpp={} swz={} fmt=0x{:08X}/0x{:02X}/{} palette={} nonzero={} alpha_sample={}/1024 rgb_sample={}/1024",
                            n, stage, tex_ptr_raw, tex_ptr, data_addr, width, height, pitch,
                            bytes_per_pixel, swizzled_upload, upload_format, upload_fmt_code,
                            crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                            palette.is_some(), has_pixels, sampled_nonzero_alpha,
                            sampled_nonzero_rgb
                        ));
                    }
                    if spidey_title_ui_trace_enabled()
                        && has_pixels
                        && spidey_title_ui_candidate_texture(width, height)
                    {
                        let trace_n = SPIDEY_TITLE_UI_TEX_TRACE_N.fetch_add(1, Ordering::Relaxed);
                        if trace_n < 192 || trace_n.is_power_of_two() {
                            let compressed_hash = fnv1a64(&linear_bytes);
                            let argb_hash = {
                                let mut hash = 0xcbf2_9ce4_8422_2325u64;
                                for &px in &pixels {
                                    for byte in px.to_le_bytes() {
                                        hash ^= byte as u64;
                                        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
                                    }
                                }
                                hash
                            };
                            let dump_path = if trace_n < 64 {
                                let path = format!(
                                    r"./spidey_title_ui_tex_{:03}_tex_{:08X}_{}x{}_fmt{:02X}_h{:016X}.bmp",
                                    trace_n,
                                    tex_ptr,
                                    width,
                                    height,
                                    upload_fmt_code,
                                    compressed_hash
                                );
                                write_readback_bmp(&path, width, height, &pixels);
                                path
                            } else {
                                String::from("-")
                            };
                            crate::xbox::emulator::debug_log(&format!(
                                "[SPIDEY-TITLE-UI-TEX] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} pitch={} bpp={} swz={} fmt=0x{:08X}/0x{:02X}/{} bytes={} compressed_hash=0x{:016X} argb_hash=0x{:016X} alpha_sample={}/1024 rgb_sample={}/1024 source={} dump={}",
                                trace_n,
                                stage,
                                tex_ptr_raw,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                pitch,
                                bytes_per_pixel,
                                swizzled_upload,
                                upload_format,
                                upload_fmt_code,
                                crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                                linear_bytes.len(),
                                compressed_hash,
                                argb_hash,
                                sampled_nonzero_alpha,
                                sampled_nonzero_rgb,
                                if (0x83E0_0000..0x84F0_0000).contains(&tex_ptr_raw) {
                                    "xbs-ish"
                                } else {
                                    "runtime"
                                },
                                dump_path
                            ));
                        }
                    }
                    if width == 128 && height == 128 {
                        static MENU_ATLAS_DUMPED: std::sync::atomic::AtomicBool =
                            std::sync::atomic::AtomicBool::new(false);
                        if !MENU_ATLAS_DUMPED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                            write_readback_bmp(
                                r"./spiderman_menu_atlas_argb.bmp",
                                width,
                                height,
                                &pixels,
                            );
                        }
                    }
                    if watch_texture {
                        static SPIDEY_TEX_WATCH_DECODE_N: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let dump_n = SPIDEY_TEX_WATCH_DECODE_N
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
                            sampled_argb_stats(&pixels);
                        if focus_overlay_texture || dump_n < 32 || dump_n.is_power_of_two() {
                            let path = if focus_overlay_texture {
                                Some(format!(
                                    r"./spidey_focus_tex_{:08X}_{}x{}.bmp",
                                    tex_ptr, width, height
                                ))
                            } else if dump_n < 8 {
                                Some(format!(
                                    r"./spidey_watch_tex_{:02}_{:08X}_{}x{}.bmp",
                                    dump_n, tex_ptr, width, height
                                ))
                            } else {
                                None
                            };
                            if let Some(path) = path.as_ref() {
                                write_readback_bmp(path, width, height, &pixels);
                            }
                            debug_log(&format!(
                                "[SPIDEY-TEX-WATCH-DECODE] #{} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} pitch={} fmt=0x{:08X}/0x{:02X}/{} nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} dump={}",
                                dump_n,
                                tex_ptr_raw,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                pitch,
                                upload_format,
                                upload_fmt_code,
                                crate::xbox::gpu::texture_format::format_name(upload_fmt_code),
                                sample_nonzero,
                                sample_alpha,
                                sample_rgb,
                                first_px,
                                mid_px,
                                path.as_deref().unwrap_or("-")
                            ));
                        }
                    }
                    if (0x83E0_0000..0x83E1_0000).contains(&tex_ptr_raw) {
                        static SPIDEY_XBS_TEX_DUMP_N: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let dump_n = SPIDEY_XBS_TEX_DUMP_N
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if dump_n < 12 {
                            let path = format!(
                                r"./spiderman_xbs_tex_{:02}_tex_{:08X}_{}x{}.bmp",
                                dump_n, tex_ptr_raw, width, height
                            );
                            write_readback_bmp(&path, width, height, &pixels);
                            crate::xbox::emulator::debug_log(&format!(
                                "[SPIDEY-XBS-TEX-DUMP] #{} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} fmt=0x{:02X} path={}",
                                dump_n,
                                tex_ptr_raw,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                upload_fmt_code,
                                path
                            ));
                        }
                    }
                    if has_pixels
                        && hle_settex_cache_is_redundant(
                            stage,
                            HLE_SETTEX_KIND_ARGB,
                            tex_ptr,
                            data_addr,
                            width,
                            height,
                            upload_fmt_code,
                            pixel_byte_len.min(u32::MAX as usize) as u32,
                            force_upload,
                            "registry-argb",
                        )
                    {
                        return;
                    }
                    // DEBUG TEST PATTERN (2026-04-24): when the guest-provided
                    // texture buffer is empty (decompression pipeline not yet
                    // writing into 0x045E0000), upload a magenta/cyan
                    // checkerboard so the glyph rendering is visibly ORGANIC
                    // end-to-end — vertex geometry, quad positioning, SetTexture
                    // binding, and PS sampling are all proven. The next and
                    // final gap is getting real pixel bytes into the texture
                    // data buffer (either XFMC stash parse + DDS decode, or
                    // finding where the guest memcpy should land but doesn't).
                    //
                    // 2026-04-24 EOS: disabled again. Agent F01 found that
                    // format 0x2C is X_D3DFMT_D16 (depth buffer), NOT a font
                    // atlas — so our "sparse dots" pattern was raw Z-buffer
                    // bytes rendered as BGRA. Keeping the test pattern on
                    // would now just paint depth buffers with synthetic
                    // colors, which is noise. Leave it off; rely on the
                    // self-healing live [tex+0x04] read + format 0x2C skip
                    // added to hle_set_texture above. When real color
                    // textures start binding, has_pixels=true short-circuits
                    // here anyway and we upload genuine bytes.
                    const TEST_PATTERN_ON_EMPTY: bool = false;
                    let mut gpu = crate::xbox::gpu::gpu_lock();
                    if let Some(ref mut backend) = *gpu {
                        if has_pixels {
                            backend.set_texture(stage, width, height, &pixels);
                            hle_settex_cache_note(
                                stage,
                                HLE_SETTEX_KIND_ARGB,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                upload_fmt_code,
                                pixel_byte_len.min(u32::MAX as usize) as u32,
                            );
                        } else if TEST_PATTERN_ON_EMPTY {
                            // 8x8 pixel checkerboard in SATURATED RGB so that
                            // ANY UV sample — especially the top-left where
                            // glyph quads live — produces a bright visible
                            // pixel. Colors cycle through red/green/blue/yellow
                            // so adjacent glyphs sample different bright cells.
                            // Previous gradient had r,g ≈ 0 at top-left so
                            // glyphs sampled near-black and looked unchanged.
                            const CELL: usize = 8;
                            const PALETTE: [u32; 4] = [
                                0xFFFF_0000, // bright red
                                0xFF00_FF00, // bright green
                                0xFF00_00FF, // bright blue
                                0xFFFF_FF00, // bright yellow
                            ];
                            for y in 0..height as usize {
                                for x in 0..width as usize {
                                    let cell_idx = ((x / CELL) + (y / CELL) * 2) & 3;
                                    pixels[y * width as usize + x] = PALETTE[cell_idx];
                                }
                            }
                            static TEST_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let tn = TEST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if tn < 5 {
                                crate::xbox::emulator::debug_log(&format!(
                                    "[TEX-TEST-PATTERN] #{} stage={} {}x{} uploaded checkerboard (real buffer was empty)",
                                    tn, stage, width, height
                                ));
                            }
                            backend.set_texture(stage, width, height, &pixels);
                            hle_settex_cache_note(
                                stage,
                                HLE_SETTEX_KIND_FALLBACK,
                                tex_ptr,
                                data_addr,
                                width,
                                height,
                                upload_fmt_code,
                                pixel_byte_len.min(u32::MAX as usize) as u32,
                            );
                        } else {
                            // Empty locked textures are still being populated by the
                            // guest. Do not bind an all-black SRV that masks valid
                            // vertex-colored legalbox spans.
                            hle_settex_cache_clear(stage);
                            backend.clear_texture(stage);
                        }
                    }
                    return;
                }
            }
        }
    }

    // Read Xbox D3D resource header from guest memory
    unsafe {
        let base = guest_mem.add(tex_ptr as usize);
        let _common = *(base as *const u32);
        let data_addr_raw = *(base.add(X_D3DRES_DATA as usize) as *const u32);
        let format = *(base.add(X_D3DRES_FORMAT as usize) as *const u32);
        let size_field = *(base.add(X_D3DRES_SIZE as usize) as *const u32);
        if size_field == 0 {
            static ZERO_SIZE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = ZERO_SIZE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[HLE-SETTEX-ZEROSIZE] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} fmt=0x{:08X} — unbinding",
                    n, stage, tex_ptr_raw, tex_ptr, data_addr_raw, format
                ));
            }
            let mut gpu = crate::xbox::gpu::gpu_lock();
            if let Some(ref mut backend) = *gpu {
                hle_settex_cache_clear(stage);
                backend.clear_texture(stage);
            }
            return;
        }

        // Extract dimensions from Format + Size fields
        // Xbox D3D format field packing (varies by type, using Size field for dimensions):
        //   Size field: bits[0..11] = width-1, bits[12..23] = height-1, bits[24..27] = pitch log2
        // For linear textures, Data points directly to pixel data.
        let width = ((size_field & 0xFFF) + 1) as u32;
        let height = (((size_field >> 12) & 0xFFF) + 1) as u32;

        // Sanity check on dimensions
        if width == 0 || width > 2048 || height == 0 || height > 2048 {
            return;
        }

        // Debug-magenta fallback: when pTex.Data is unset (Spider-Man's
        // XPR-packed textures leave Data=0 because Register() is
        // LTCG-inlined — our hook at 0x002F33F0 never fires), upload a
        // magenta (0xFFFF00FF) BGRA fill so textured quads render VISIBLY
        // as debug-pink instead of invisibly black. Confirms pipeline
        // end-to-end health and surfaces WHERE the game tries to bind
        // textures so we can trace back to the missing upload path.
        let data_addr = normalize_guest_ram_ptr(data_addr_raw & !0x3);
        if data_addr.is_none() {
            static DBG_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = DBG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[HLE-SETTEX-DBGFILL] #{} stage={} tex=0x{:08X}->0x{:08X} {}x{} data=0x{:08X} → magenta fill",
                    n, stage, tex_ptr_raw, tex_ptr, width, height, data_addr_raw
                ));
            }
            let size = (width * height) as usize;
            let pixels: Vec<u32> = vec![0xFFFF_00FFu32; size]; // BGRA magenta
            let mut gpu = crate::xbox::gpu::gpu_lock();
            if let Some(ref mut backend) = *gpu {
                backend.set_texture(stage, width, height, &pixels);
            }
            return;
        }
        let data_addr = data_addr.unwrap();
        let fmt_code = xbox_texture_format_code(format);

        if crate::xbox::gpu::texture_format::is_block_compressed(fmt_code) {
            let byte_len = texture_byte_len(width, height, fmt_code) as usize;
            let byte_end = (data_addr as usize).checked_add(byte_len);
            if byte_len == 0
                || byte_len > 16 * 1024 * 1024
                || byte_end.map_or(true, |end| end > 0x2000_0000)
            {
                static BAD_BC_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = BAD_BC_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 20 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[HLE-TEX-FALLBACK-BC-BAD] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X}->0x{:08X} fmt=0x{:08X}/0x{:02X} size=0x{:08X} {}x{} bytes={}",
                        n,
                        stage,
                        tex_ptr_raw,
                        tex_ptr,
                        data_addr_raw,
                        data_addr,
                        format,
                        fmt_code,
                        size_field,
                        width,
                        height,
                        byte_len
                    ));
                }
                let mut gpu = crate::xbox::gpu::gpu_lock();
                if let Some(ref mut backend) = *gpu {
                    hle_settex_cache_clear(stage);
                    backend.clear_texture(stage);
                }
                return;
            }

            let bytes = std::slice::from_raw_parts(guest_mem.add(data_addr as usize), byte_len);
            if hle_settex_cache_is_redundant(
                stage,
                HLE_SETTEX_KIND_RAW_BC,
                tex_ptr,
                data_addr,
                width,
                height,
                fmt_code,
                byte_len.min(u32::MAX as usize) as u32,
                force_upload,
                "fallback-bc",
            ) {
                return;
            }
            static BC_FALLBACK_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BC_FALLBACK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 24 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[HLE-TEX-FALLBACK-BC] #{} stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X}->0x{:08X} fmt=0x{:08X}/0x{:02X}/{} size=0x{:08X} {}x{} bytes={} sample={:02X?}",
                    n,
                    stage,
                    tex_ptr_raw,
                    tex_ptr,
                    data_addr_raw,
                    data_addr,
                    format,
                    fmt_code,
                    crate::xbox::gpu::texture_format::format_name(fmt_code),
                    size_field,
                    width,
                    height,
                    byte_len,
                    &bytes[..bytes.len().min(8)]
                ));
            }
            let mut gpu = crate::xbox::gpu::gpu_lock();
            if let Some(ref mut backend) = *gpu {
                backend.set_texture_swizzle_hint(stage, false);
                backend.set_texture_raw(stage, width, height, fmt_code, bytes);
                hle_settex_cache_note(
                    stage,
                    HLE_SETTEX_KIND_RAW_BC,
                    tex_ptr,
                    data_addr,
                    width,
                    height,
                    fmt_code,
                    byte_len.min(u32::MAX as usize) as u32,
                );
            }
            return;
        }

        // Read pixel data from guest memory (assume ARGB8888)
        let pixel_count = (width * height) as usize;
        let byte_len =
            (pixel_count.saturating_mul(std::mem::size_of::<u32>())).min(u32::MAX as usize) as u32;
        if hle_settex_cache_is_redundant(
            stage,
            HLE_SETTEX_KIND_ARGB,
            tex_ptr,
            data_addr,
            width,
            height,
            fmt_code,
            byte_len,
            force_upload,
            "fallback-argb",
        ) {
            return;
        }
        let data_base = guest_mem.add(data_addr as usize);
        let pixels = std::slice::from_raw_parts(data_base as *const u32, pixel_count);

        static TEX_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 5 {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-TEX] SetTexture stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X}->0x{:08X} fmt=0x{:08X} size=0x{:08X} → {}x{} px[0]=0x{:08X}",
                stage, tex_ptr_raw, tex_ptr, data_addr_raw, data_addr, format, size_field, width, height,
                if pixel_count > 0 { pixels[0] } else { 0 }
            ));
        }

        // Upload to GPU backend
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            backend.set_texture(stage, width, height, pixels);
            hle_settex_cache_note(
                stage,
                HLE_SETTEX_KIND_ARGB,
                tex_ptr,
                data_addr,
                width,
                height,
                fmt_code,
                byte_len,
            );
        }
    }
}

pub(super) fn hle_set_stream_source(args: &[u32; 8], guest_mem: *mut u8) {
    spiderman_scrub_bad_render_callback_ptrs(guest_mem);

    // Xbox D3D8: SetStreamSource(StreamNumber, pVertexBuffer, Stride)
    // args[0] = StreamNumber (0 = primary), args[1] = pVertexBuffer, args[2] = Stride
    let stream_num = args[0];
    let vb_ptr = args[1];
    let stride = args[2];
    if stream_num == 0 && vb_ptr != 0 && vb_ptr < 0x1000_0000 {
        // Xbox VB resource: [+0x00]=Common, [+0x04]=Data ptr, [+0x08]=Lock.
        let data_ptr = unsafe { *((guest_mem as u64 + vb_ptr as u64 + 0x04) as *const u32) };
        const RAM_MASK: u32 = 0x1FFF_FFFF;
        STREAM0_VB_PTR.store(data_ptr & RAM_MASK, Ordering::Relaxed);
        if stride > 0 && stride <= 128 {
            STREAM0_STRIDE.store(stride, Ordering::Relaxed);
        }

        // Mirror the binding into the nv2a_pb pushbuffer parser state. Parser
        // uses these atomics when a raw DRAW_ARRAYS method fires.
        crate::xbox::aot::nv2a_pb::STREAM0_VB_ADDR
            .store(data_ptr & RAM_MASK, std::sync::atomic::Ordering::Relaxed);
        if stride > 0 && stride <= 128 {
            crate::xbox::aot::nv2a_pb::STREAM0_STRIDE
                .store(stride, std::sync::atomic::Ordering::Relaxed);
        }

        static SSS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = SSS_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 10 {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-SSS] SetStreamSource({}, vb=0x{:08X}, data=0x{:08X}, stride={})",
                stream_num, vb_ptr, data_ptr, stride
            ));
        }
    }
}

fn hle_set_transform(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let state_type = args[0];
    let matrix_ptr = args[1];
    if matrix_ptr == 0 || matrix_ptr >= 0x1000_0000 {
        return 0;
    }
    drain_draws_before_state_change("SetTransform");
    // Read 4x4 float matrix (16 floats = 64 bytes) from guest memory
    let mut matrix = [0.0f32; 16];
    unsafe {
        let src = guest_mem.add(matrix_ptr as usize) as *const f32;
        for i in 0..16 {
            matrix[i] = *src.add(i);
        }
    }
    // Forward to GPU backend
    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_transform(state_type, &matrix);
    }
    static TF_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = TF_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 10 {
        debug_log(&format!(
            "[HLE] SetTransform: state={} mat[0..4]=[{:.2}, {:.2}, {:.2}, {:.2}]",
            state_type, matrix[0], matrix[1], matrix[2], matrix[3]
        ));
    }
    0
}

/// Allocate a fake D3D object in guest memory (pool bump region).
/// Returns guest address of zero-filled block, or 0 on failure.
/// Small allocations (<64KB) use the header pool; large ones use the data pool.
fn alloc_fake_d3d_obj(guest_mem: *mut u8, size: u32) -> u32 {
    // Header pool: small objects (surfaces, textures, VB headers). Spider-Man
    // creates thousands of short-lived 72-byte resources during the title flow,
    // so the old ~960KB arena exhausted before the game could reach the menu.
    //
    // The Rust port maps a 512MB RAM mirror at 0x8000_0000..0x9FFF_FFFF.
    // Keep allocating mirrored D3D handles there, but do not stop at the old
    // 64MB boundary: Spider-Man's level load creates enough tiny surface/cube
    // headers to exhaust 0x83C0_0000..0x83FF_F000 before the hero assets finish.
    static FAKE_BUMP: AtomicU32 = AtomicU32::new(0x83C0_0000);
    const FAKE_CEIL: u32 = 0x9FFF_F000;
    // Data pool: large allocations (VB data, texture pixels). Keep this below
    // 0x1000_0000 because several legacy D3D HLE paths treat low guest pointers
    // as directly readable, but keep it above the kernel VA bump's early
    // 0x0400_0000 process-heap lane. Spider-Man's CRT heap lives at 0x04000000;
    // placing VB backing data there tramples the heap header before menu scripts
    // can finish.
    static DATA_BUMP: AtomicU32 = AtomicU32::new(0x0800_0000);
    const DATA_CEIL: u32 = 0x1000_0000; // 128MB data pool below legacy pointer cutoff

    let aligned_size = (size + 15) & !15; // 16-byte aligned
    let (addr, ceil) = if size >= 0x10000 {
        // Large allocation → data pool
        let a = DATA_BUMP.fetch_add(aligned_size, Ordering::Relaxed);
        (a, DATA_CEIL)
    } else {
        // Small allocation → header pool
        let a = FAKE_BUMP.fetch_add(aligned_size, Ordering::Relaxed);
        (a, FAKE_CEIL)
    };
    if addr.saturating_add(aligned_size) > ceil {
        debug_log(&format!(
            "[HLE] alloc_fake_d3d_obj: POOL EXHAUSTED size={} addr=0x{:08X}",
            size, addr
        ));
        return 0;
    }
    // Zero-fill the block
    unsafe {
        let ptr = guest_mem.add(addr as usize);
        std::ptr::write_bytes(ptr, 0, aligned_size as usize);
    }
    addr
}

/// Allocate fake Bink middleware handles away from the D3D resource pool.
///
/// Spider-Man can load Bink handles before its real menu textures. Keeping
/// these handles out of the 0x83C0_0000 D3D object arena avoids stale
/// address-based texture diagnostics mistaking movie state for font resources.
fn alloc_fake_bink_obj(guest_mem: *mut u8, size: u32) -> u32 {
    static BINK_BUMP: AtomicU32 = AtomicU32::new(0x83B0_0000);
    const BINK_CEIL: u32 = 0x83C0_0000;

    let aligned_size = (size + 15) & !15;
    let addr = BINK_BUMP.fetch_add(aligned_size, Ordering::Relaxed);
    if addr == 0 || addr.saturating_add(aligned_size) > BINK_CEIL {
        debug_log(&format!(
            "[BINK-HLE] fake handle pool exhausted size=0x{:X} addr=0x{:08X}",
            size, addr
        ));
        return 0;
    }

    unsafe {
        let ptr = guest_mem.add(addr as usize);
        std::ptr::write_bytes(ptr, 0, aligned_size as usize);
    }
    addr
}

/// DrawVerticesUP: read inline vertex data from guest memory, send to GPU backend.
/// Xbox D3D8: DrawVerticesUP(PrimitiveType, VertexCount, pVertexStreamZeroData, VertexStreamZeroStride)
pub(crate) fn hle_draw_vertices_up(args: &[u32; 8], guest_mem: *mut u8) {
    use crate::xbox::gpu::{
        NV2AVertex, NV097_LINES, NV097_LINE_STRIP, NV097_POINTS, NV097_QUAD_LIST, NV097_TRIANGLES,
        NV097_TRIANGLE_FAN, NV097_TRIANGLE_STRIP,
    };

    let prim_type = args[0];
    let vertex_count = args[1];
    let vertex_data_ptr = args[2]; // guest address of vertex data
    let stride = args[3];

    if vertex_count == 0 || vertex_count > 65536 || stride == 0 || stride > 256 {
        return;
    }

    // Map Xbox D3DPRIMITIVETYPE to NV097.
    // Xbox D3DPT values are +1 from host D3D9 (confirmed via .cod disassembly):
    //   Xbox: POINTLIST=1, LINELIST=2, LINESTRIP=3, LINELOOP=4,
    //         TRIANGLELIST=5, TRIANGLESTRIP=6, TRIANGLEFAN=7
    // Some games also use Xbox's extended D3DPT_QUADLIST value 7
    // through non-UP draw paths; route that as an explicit host primitive
    // where the call contract is known.
    let nv_prim = match prim_type {
        1 => NV097_POINTS,
        2 => NV097_LINES,
        3 | 4 => NV097_LINE_STRIP,
        5 => NV097_TRIANGLES,
        6 => NV097_TRIANGLE_STRIP,
        7 => NV097_TRIANGLE_FAN,
        8 => NV097_QUAD_LIST,
        _ => NV097_TRIANGLES,
    };

    // Read vertices from guest memory.
    // FVF D3DFVF_XYZRHW | D3DFVF_DIFFUSE = 20 bytes (x,y,z,rhw,color)
    // FVF D3DFVF_XYZRHW | D3DFVF_DIFFUSE | D3DFVF_TEX1 = 28 bytes (+u,v)
    let has_uv = stride >= 28;
    let shader = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
    let mut verts = Vec::with_capacity(vertex_count as usize);
    for i in 0..vertex_count {
        let addr = vertex_data_ptr.wrapping_add(i.saturating_mul(stride));
        if addr >= 0x2000_0000 || addr.saturating_add(20) > 0x2000_0000 {
            break;
        } // bounds check
        if let Some(v) = decode_no_shader_stride36_vertex(guest_mem, addr, stride, shader) {
            verts.push(v);
            continue;
        }
        let base = guest_mem as u64 + addr as u64;
        unsafe {
            let x = *(base as *const f32);
            let y = *((base + 4) as *const f32);
            let z = *((base + 8) as *const f32);
            let rhw = *((base + 12) as *const f32);
            let mut color = *((base + 16) as *const u32);
            let (mut u, mut v) = if has_uv {
                (*((base + 20) as *const f32), *((base + 24) as *const f32))
            } else {
                (0.0, 0.0)
            };

            if has_uv && stride >= 36 && ((color >> 24) == 0 || !u.is_finite() || !v.is_finite()) {
                // Some XDK helper paths pass wider screen-space vertices. Classic
                // Doom's 52-byte UP quad stores diffuse at +0x18 and texcoord0 at
                // +0x1C/+0x20; the compact TL layout above reads alpha=0/NaN UVs.
                let alt_color = std::ptr::read_unaligned((base + 24) as *const u32);
                let alt_u = std::ptr::read_unaligned((base + 28) as *const f32);
                let alt_v = std::ptr::read_unaligned((base + 32) as *const f32);
                if (alt_color >> 24) != 0
                    && alt_u.is_finite()
                    && alt_v.is_finite()
                    && alt_u >= -4096.0
                    && alt_v >= -4096.0
                    && alt_u <= 4096.0
                    && alt_v <= 4096.0
                {
                    color = alt_color;
                    u = alt_u;
                    v = alt_v;
                }
            }
            verts.push(NV2AVertex {
                x,
                y,
                z,
                w: rhw,
                color,
                u,
                v,
            });
        }
    }

    static DRAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 5 {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAWUP] prim={} count={} stride={} data=0x{:08X} v0=({:.0},{:.0},{:.0} c=0x{:08X})",
            prim_type, vertex_count, stride, vertex_data_ptr,
            verts.first().map(|v| v.x).unwrap_or(0.0),
            verts.first().map(|v| v.y).unwrap_or(0.0),
            verts.first().map(|v| v.z).unwrap_or(0.0),
            verts.first().map(|v| v.color).unwrap_or(0),
        ));
    }
    log_pixel_shader_draw_snapshot("DrawVerticesUP", n);
    log_texture_stage_draw_snapshot("DrawVerticesUP", n);
    log_spidey_select_circle_draw_probe("DrawVerticesUP", n, prim_type, &verts, 0, "pre-backend");
    let refreshed = refresh_dirty_active_textures_before_draw("DrawVerticesUP", guest_mem);
    doom_log_draw_vertices_up_pipeline(
        prim_type,
        vertex_count,
        stride,
        vertex_data_ptr,
        refreshed,
        &verts,
        guest_mem,
    );

    // Worker/HLE-submitted UP draws must follow the same path as pushbuffer
    // draws: queue them here, then let Swap drain into the active backend
    // immediately before readback. Calling the D3D11 backend directly from this
    // side can miss the single-threaded context/readback path and leaves the
    // presented frame black even though the hook fired.
    crate::xbox::gpu::queue_draw(verts, nv_prim);
}

/// DrawIndexedVerticesUP: expand index buffer into vertex list, send to GPU.
/// Xbox D3D8 signature: DrawIndexedVerticesUP(PrimitiveType, VertexCount, pIndexData, pVertexData, Stride)
/// args: [0]=PrimType, [1]=VertexCount, [2]=pIndexData, [3]=pVertexData, [4]=Stride
fn expand_indexed_primitive_indices(
    prim_type: u32,
    index_count: u32,
    index_data_ptr: u32,
    guest_mem: *mut u8,
) -> Vec<u32> {
    let mut raw = Vec::with_capacity(index_count as usize);
    for i in 0..index_count {
        let idx_addr = guest_mem as u64 + index_data_ptr as u64 + (i as u64 * 2);
        if idx_addr + 2 > guest_mem as u64 + 0x2000_0000 {
            break;
        }
        raw.push(unsafe { std::ptr::read_unaligned(idx_addr as *const u16) } as u32);
    }

    if prim_type != 6 {
        raw.retain(|idx| *idx != 0xFFFF);
        return raw;
    }

    let mut out = Vec::with_capacity(raw.len().saturating_mul(3));
    let mut strip: Vec<u32> = Vec::new();
    let mut tri_in_segment = 0usize;
    let mut restart_count = 0u32;
    let mut degenerate_count = 0u32;

    for idx in raw {
        if idx == 0xFFFF {
            strip.clear();
            tri_in_segment = 0;
            restart_count = restart_count.saturating_add(1);
            continue;
        }

        strip.push(idx);
        if strip.len() < 3 {
            continue;
        }

        let a = strip[strip.len() - 3];
        let b = strip[strip.len() - 2];
        let c = strip[strip.len() - 1];
        if a == b || b == c || a == c {
            degenerate_count = degenerate_count.saturating_add(1);
            tri_in_segment = tri_in_segment.saturating_add(1);
            continue;
        }

        if (tri_in_segment & 1) == 0 {
            out.extend_from_slice(&[a, b, c]);
        } else {
            out.extend_from_slice(&[b, a, c]);
        }
        tri_in_segment = tri_in_segment.saturating_add(1);
    }

    static STRIP_EXPAND_LOG: AtomicU32 = AtomicU32::new(0);
    let n = STRIP_EXPAND_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-STRIP-EXPAND] #{} idx=0x{:08X} raw={} out={} tris={} restarts={} degenerates={}",
            n,
            index_data_ptr,
            index_count,
            out.len(),
            out.len() / 3,
            restart_count,
            degenerate_count
        ));
    }

    out
}

pub(crate) fn hle_draw_indexed_vertices_up(args: &[u32; 8], guest_mem: *mut u8) {
    use crate::xbox::gpu::{
        NV2AVertex, NV097_LINES, NV097_LINE_STRIP, NV097_POINTS, NV097_QUAD_LIST, NV097_TRIANGLES,
        NV097_TRIANGLE_FAN, NV097_TRIANGLE_STRIP,
    };

    let prim_type = args[0];
    let index_count = args[1];
    let index_data_ptr = args[2];
    let vertex_data_ptr = args[3];
    let stride = args[4];

    if index_count == 0 || index_count > 65536 || stride == 0 || stride > 256 {
        return;
    }

    let nv_prim = match prim_type {
        1 => NV097_POINTS,
        2 => NV097_LINES,
        3 | 4 => NV097_LINE_STRIP,
        5 => NV097_TRIANGLES,
        6 => NV097_TRIANGLE_STRIP,
        7 => NV097_TRIANGLE_FAN,
        8 => NV097_QUAD_LIST,
        _ => NV097_TRIANGLES,
    };

    let expanded_indices =
        expand_indexed_primitive_indices(prim_type, index_count, index_data_ptr, guest_mem);

    // Read 16-bit indices from guest memory and expand into vertex list
    let mut verts = Vec::with_capacity(expanded_indices.len());
    let shader = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
    for idx in expanded_indices.iter().copied() {
        let addr = vertex_data_ptr.wrapping_add(idx.saturating_mul(stride));
        if addr >= 0x2000_0000 || addr.saturating_add(20) > 0x2000_0000 {
            break;
        }
        if let Some(v) = decode_no_shader_stride36_vertex(guest_mem, addr, stride, shader) {
            verts.push(v);
            continue;
        }
        let base = guest_mem as u64 + addr as u64;
        unsafe {
            verts.push(NV2AVertex {
                x: *(base as *const f32),
                y: *((base + 4) as *const f32),
                z: *((base + 8) as *const f32),
                w: *((base + 12) as *const f32),
                color: *((base + 16) as *const u32),
                u: 0.0,
                v: 0.0,
            });
        }
    }

    static DRAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 5 {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAWIDXUP] prim={} indices={} stride={} idx=0x{:08X} vtx=0x{:08X} expanded={}",
            prim_type,
            index_count,
            stride,
            index_data_ptr,
            vertex_data_ptr,
            verts.len()
        ));
    }
    log_pixel_shader_draw_snapshot("DrawIndexedVerticesUP", n);
    log_texture_stage_draw_snapshot("DrawIndexedVerticesUP", n);
    log_spidey_select_circle_draw_probe(
        "DrawIndexedVerticesUP",
        n,
        prim_type,
        &verts,
        0,
        "pre-backend",
    );
    refresh_dirty_active_textures_before_draw("DrawIndexedVerticesUP", guest_mem);

    let mut gpu = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut backend) = *gpu {
        let backend_prim = if prim_type == 6 {
            NV097_TRIANGLES
        } else {
            nv_prim
        };
        backend.draw_primitive(&verts, backend_prim);
        capture_spidey_select_circle_after_draw(
            backend.as_mut(),
            "DrawIndexedVerticesUP",
            n,
            prim_type,
            &verts,
        );
    }
}

/// BeginPush: return dummy PB pointer to prevent stall.
///
/// Xbox SDKs expose two closely-related forms here. The common `BeginPush_4`
/// form takes only `Count`, returns the write cursor in eax, and is paired with
/// EndPush(cursor). Some fixture paths name a two-arg `BeginPush_8` out-param
/// variant; keep supporting it without making the one-arg form scribble over a
/// caller stack local.
fn hle_begin_push(
    args: &[u32; 8],
    guest_mem: *mut u8,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    out_param: bool,
) -> u32 {
    // args[0] = Count (dwords caller promises to write)
    let dest = reserve_begin_push_space(args[0]);
    remember_begin_push_start(dest);
    let pp_push = if out_param {
        // args[1] = PDWORD** ppPush for the ret-8/out-param variant.
        let pp_push = args[1];
        // Use full 512MB guest range to match valid guest stack pointers
        // (0x1E000000-0x1EFFFFFF).
        if let Some(pp_push_norm) = guest_ram_offset(pp_push) {
            unsafe {
                *((guest_mem as u64 + pp_push_norm as u64) as *mut u32) = dest;
            }
        }
        pp_push
    } else {
        0
    };
    static BP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = BP_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[HLE-BEGINPUSH #{}] count={} mode={} ppPush=0x{:08X} cursor=0x{:08X}",
            n,
            args[0],
            if out_param { "out-param" } else { "return" },
            pp_push,
            dest
        ));
    }
    let ret = if out_param { 0 } else { dest };
    context.Rax = ret as u64;
    ret
}

/// Draw: issue draw call to GPU backend
pub(crate) fn hle_draw(args: &[u32; 8], guest_mem: *mut u8, mmio_count: &mut u64) {
    use crate::xbox::gpu::{
        NV2AVertex, NV097_QUAD_LIST, NV097_TRIANGLES, NV097_TRIANGLE_FAN, NV097_TRIANGLE_STRIP,
    };

    // DrawVertices(PrimitiveType, StartVertex, VertexCount)
    let prim_type = args[0];
    let start_vertex = args[1];
    let vertex_count = args[2];

    if (0x0000_1000u32..0x2000_0000u32).contains(&vertex_count) && start_vertex <= 65_536 {
        static DRAW_MISROUTE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = DRAW_MISROUTE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 20 || (n % 1000 == 0) {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-DRAW-MISROUTE] #{} DrawVertices args look indexed: prim={} index_count={} index_ptr=0x{:08X}",
                n, prim_type, start_vertex, vertex_count
            ));
        }
        hle_draw_indexed_vertices(args, guest_mem, mmio_count);
        return;
    }

    *mmio_count += 1;

    let vb_data = STREAM0_VB_PTR.load(Ordering::Relaxed);
    let stride = STREAM0_STRIDE.load(Ordering::Relaxed);
    let shader = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);

    // Default stride: Xbox D3D8 most common vertex format is XYZRHW+DIFFUSE+TEX1 = 32 bytes
    let stride = if stride == 0 { 32 } else { stride };

    static DRAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = DRAW_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 20 || (n % 1000 == 0) {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAW] #{} prim={} start={} count={} vb=0x{:08X} stride={}",
            n, prim_type, start_vertex, vertex_count, vb_data, stride
        ));
    }

    if vb_data == 0 || vertex_count == 0 || vertex_count > 65536 {
        return; // No vertex data or invalid count
    }

    // Read vertex data from guest memory and convert to NV2AVertex
    let mut verts = Vec::with_capacity(vertex_count as usize);
    for i in 0..vertex_count {
        let offset = (start_vertex + i) * stride;
        let addr = vb_data.wrapping_add(offset);
        if addr >= 0x1000_0000 {
            break;
        } // out of range

        if let Some(v) = decode_no_shader_stride36_vertex(guest_mem, addr, stride, shader) {
            verts.push(v);
            continue;
        }

        let ptr = (guest_mem as u64 + addr as u64) as *const f32;
        unsafe {
            let x = *ptr;
            let y = *ptr.add(1);
            let z = *ptr.add(2);
            // Try to read W, color, UV depending on stride
            let (w, color, u, v) = if stride >= 28 {
                let w = *ptr.add(3);
                let c = *(ptr.add(4) as *const u32);
                let u = if stride >= 24 { *ptr.add(5) } else { 0.0 };
                let v = if stride >= 28 { *ptr.add(6) } else { 0.0 };
                (w, c, u, v)
            } else if stride >= 16 {
                let c = *(ptr.add(3) as *const u32);
                (1.0, c, 0.0, 0.0)
            } else {
                (1.0, 0xFFFFFFFF, 0.0, 0.0)
            };

            verts.push(NV2AVertex {
                x,
                y,
                z,
                w,
                color,
                u,
                v,
            });
        }
    }

    if verts.is_empty() {
        return;
    }
    log_pixel_shader_draw_snapshot("DrawVertices", n);
    log_texture_stage_draw_snapshot("DrawVertices", n);
    log_spidey_select_circle_draw_probe("DrawVertices", n, prim_type, &verts, 0, "pre-backend");

    // Map Xbox D3D8 primitive type to NV097 type
    let nv_prim = match prim_type {
        1 => crate::xbox::gpu::NV097_POINTS,         // D3DPT_POINTLIST
        2 => crate::xbox::gpu::NV097_LINES,          // D3DPT_LINELIST
        3 | 4 => crate::xbox::gpu::NV097_LINE_STRIP, // D3DPT_LINELOOP/LINESTRIP
        5 => NV097_TRIANGLES,                        // D3DPT_TRIANGLELIST
        6 => NV097_TRIANGLE_STRIP,                   // D3DPT_TRIANGLESTRIP
        7 => NV097_TRIANGLE_FAN,                     // D3DPT_TRIANGLEFAN
        8 => NV097_QUAD_LIST,                        // Xbox D3DPT_QUADLIST
        _ => NV097_TRIANGLES,
    };
    refresh_dirty_active_textures_before_draw("DrawVertices", guest_mem);

    // Submit to GPU backend
    let mut guard = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut gpu) = *guard {
        gpu.draw_primitive(&verts, nv_prim);
        capture_spidey_select_circle_after_draw(gpu.as_mut(), "DrawVertices", n, prim_type, &verts);
    }
}

/// DrawIndexedVertices: read 16-bit indices from pIndexData and fetch vertices
/// from the current stream source. Xbox D3D8 signature:
/// DrawIndexedVertices(PrimitiveType, VertexCount, pIndexData).
fn log_skinned_draw_constants(draw_index: u32, shader: u32) {
    use std::fmt::Write as _;

    static SKIN_DRAW_CONST_LOG: AtomicU32 = AtomicU32::new(0);
    let k = SKIN_DRAW_CONST_LOG.fetch_add(1, Ordering::Relaxed);
    if !(k < 32 || k.is_power_of_two()) {
        return;
    }
    let constants = vs_constants().lock().unwrap_or_else(|e| e.into_inner());
    let row = |idx: usize| constants.get(idx).copied().unwrap_or([0.0; 4]);
    let mut rows = String::new();
    for idx in 4..=19 {
        let c = row(idx);
        let _ = write!(
            rows,
            " c{}=[{:.4},{:.4},{:.4},{:.4}]",
            idx, c[0], c[1], c[2], c[3]
        );
    }
    debug_log(&format!(
        "[VS-DRAW-CONSTS] #{} draw#{} shader=0x{:08X}{}",
        k, draw_index, shader, rows
    ));
}

pub(crate) fn hle_draw_indexed_vertices(args: &[u32; 8], guest_mem: *mut u8, mmio_count: &mut u64) {
    *mmio_count += 1;

    let prim_type = args[0];
    let index_count = args[1];
    let index_data_ptr = args[2];
    let vb_data = STREAM0_VB_PTR.load(Ordering::Relaxed);
    let stride = STREAM0_STRIDE.load(Ordering::Relaxed);
    let stride = if stride == 0 { 32 } else { stride };
    let shader = crate::xbox::aot::nv2a_pb::VERTEX_SHADER.load(Ordering::Relaxed);
    let shader_hle_path = (0x0000_1000..0x0100_0000).contains(&shader);
    let shader_translated = shader_hle_path && crate::xbox::gpu::nv2a_vsh::shader_has_hlsl(shader);
    let cpu_vs_xform = shader_hle_path && cpu_vs_xform_enabled();
    let cpu_vs_clip_output = cpu_vs_xform && cpu_vs_clip_output_enabled();
    let shader_info = if shader_hle_path {
        lookup_vertex_shader_info(shader)
    } else {
        None
    };
    let shader_decl = shader_info
        .as_ref()
        .map(|info| info.decl)
        .unwrap_or_default();
    let shader_simple_output = cpu_vs_xform
        && stride == 36
        && shader_info
            .as_ref()
            .is_some_and(|info| info.simple_dp4_o_pos && !info.skinned);
    if cpu_vs_xform && !shader_simple_output && !shader_translated {
        log_spidey_select_circle_draw_probe(
            "DrawIndexedVertices",
            0,
            prim_type,
            &[],
            shader,
            "skip-unsupported-shader",
        );
        static UNSUPPORTED_SHADER_SKIP: AtomicU32 = AtomicU32::new(0);
        let k = UNSUPPORTED_SHADER_SKIP.fetch_add(1, Ordering::Relaxed);
        if k < 24 || k.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-DRAWIDX-SKIP-UNSUPPORTED] #{} shader=0x{:08X} stride={} skinned={} simple_dp4={} tex0=0x{:08X} indices={} idx=0x{:08X}",
                k,
                shader,
                stride,
                shader_info.as_ref().is_some_and(|info| info.skinned),
                shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
                CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed),
                index_count,
                index_data_ptr
            ));
        }
        return;
    }
    let shader_passes_diffuse =
        shader_simple_output && shader_info.as_ref().is_some_and(|info| info.passes_diffuse);
    let shader_diffuse_mul_c0 =
        shader_passes_diffuse && shader_info.as_ref().is_some_and(|info| info.diffuse_mul_c0);
    let shader_passes_tex0 =
        shader_simple_output && shader_info.as_ref().is_some_and(|info| info.passes_tex0);
    let shader_tex0_add_c0 =
        shader_passes_tex0 && shader_info.as_ref().is_some_and(|info| info.tex0_add_c0);
    if cpu_vs_xform && shader_simple_output && !shader_passes_tex0 && !shader_translated {
        log_spidey_select_circle_draw_probe(
            "DrawIndexedVertices",
            0,
            prim_type,
            &[],
            shader,
            "skip-nontextured-shader",
        );
        static NONTEXTURED_SHADER_SKIP: AtomicU32 = AtomicU32::new(0);
        let k = NONTEXTURED_SHADER_SKIP.fetch_add(1, Ordering::Relaxed);
        if k < 16 || k.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[HLE-DRAWIDX-SKIP-NONTEXTURED] #{} shader=0x{:08X} stride={} diffuse={} mul_c0={} indices={} idx=0x{:08X}",
                k,
                shader,
                stride,
                shader_info.as_ref().is_some_and(|info| info.passes_diffuse),
                shader_info.as_ref().is_some_and(|info| info.diffuse_mul_c0),
                index_count,
                index_data_ptr
            ));
        }
        return;
    }
    let shader_c0 = if shader_diffuse_mul_c0 || shader_tex0_add_c0 {
        current_vs_constant(0)
    } else {
        [0.0; 4]
    };
    let stage0_tex = CURRENT_STAGE0_TEXTURE.load(Ordering::Relaxed);
    let shader_screenspace = shader_info.as_ref().is_some_and(|info| info.screenspace);
    if shader_hle_path && shader_screenspace {
        let vp = *shader_viewport().lock().unwrap_or_else(|e| e.into_inner());
        upload_reserved_screenspace_constants(vp);
    }

    static DRAWIDX_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = DRAWIDX_LOG.fetch_add(1, Ordering::Relaxed);
    if shader == 0x0000_1001 {
        let c4 = current_vs_constant(4);
        let c5 = current_vs_constant(5);
        let c6 = current_vs_constant(6);
        let c7 = current_vs_constant(7);
        let c58 = current_vs_constant(58);
        let c59 = current_vs_constant(59);
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAWIDX-SHADER1001-PREDRAW] draw#{} prim={} indices={} screenspace={} translated={} cpu_vs={} tex0=0x{:08X} c4=[{:.3},{:.3},{:.3},{:.3}] c5=[{:.3},{:.3},{:.3},{:.3}] c6=[{:.3},{:.3},{:.3},{:.3}] c7=[{:.3},{:.3},{:.3},{:.3}] c58=[{:.3},{:.3},{:.3},{:.3}] c59=[{:.3},{:.3},{:.3},{:.3}]",
            n,
            prim_type,
            index_count,
            shader_screenspace,
            shader_translated,
            cpu_vs_xform,
            stage0_tex,
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
        ));
    }
    if n < 20 || (n % 1000 == 0) {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAWIDX] #{} prim={} indices={} idx=0x{:08X} vb=0x{:08X} stride={} shader=0x{:08X} tex0=0x{:08X} translated={} cpu_vs={} screenspace={} simple_dp4={} od0={} od0_mul_c0={} ot0={} ot0_add_c0={} skinned={}",
            n,
            prim_type,
            index_count,
            index_data_ptr,
            vb_data,
            stride,
            shader,
            stage0_tex,
            shader_translated,
            cpu_vs_xform,
            shader_screenspace,
            shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
            shader_info.as_ref().is_some_and(|info| info.passes_diffuse),
            shader_info.as_ref().is_some_and(|info| info.diffuse_mul_c0),
            shader_info.as_ref().is_some_and(|info| info.passes_tex0),
            shader_info.as_ref().is_some_and(|info| info.tex0_add_c0),
            shader_info.as_ref().is_some_and(|info| info.skinned)
        ));
        log_pixel_shader_draw_snapshot("DrawIndexedVertices", n);
        log_texture_stage_draw_snapshot("DrawIndexedVertices", n);
    }
    if shader == 0x0000_1003 || shader_info.as_ref().is_some_and(|info| info.skinned) {
        log_skinned_draw_constants(n, shader);
    }

    if vb_data == 0
        || index_data_ptr == 0
        || index_count == 0
        || index_count > 65_536
        || stride == 0
        || stride > 256
    {
        return;
    }

    let selected_transform = if cpu_vs_xform {
        select_indexed_transform(
            guest_mem,
            index_data_ptr,
            index_count,
            vb_data,
            stride,
            shader_screenspace,
        )
    } else {
        None
    };
    if let Some(sel) = selected_transform {
        static XFORM_SELECT_LOG: AtomicU32 = AtomicU32::new(0);
        let k = XFORM_SELECT_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 24 || (n % 1000 == 0) {
            debug_log(&format!(
                "[VS-XFORM-SELECT] draw#{} shader=0x{:08X} seq={} src=0x{:08X} mode={} score={:.1} finite={} onscreen={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] vp=[{:.0},{:.0},{:.0},{:.0}]",
                n,
                shader,
                sel.matrix.seq,
                sel.matrix.src,
                sel.mode.label(),
                sel.stats.score,
                sel.stats.finite,
                sel.stats.onscreen,
                sel.stats.min_x,
                sel.stats.min_y,
                sel.stats.min_z,
                sel.stats.max_x,
                sel.stats.max_y,
                sel.stats.max_z,
                sel.viewport.x,
                sel.viewport.y,
                sel.viewport.w,
                sel.viewport.h
            ));
        }
    }

    let expanded_indices =
        expand_indexed_primitive_indices(prim_type, index_count, index_data_ptr, guest_mem);
    let mut verts = Vec::with_capacity(expanded_indices.len());
    let mut skin_payload = Vec::with_capacity(expanded_indices.len());
    let mut first_indices = Vec::with_capacity(12);
    let mut min_index = u32::MAX;
    let mut max_index = 0u32;
    for idx in expanded_indices.iter().copied() {
        if first_indices.len() < 12 {
            first_indices.push(idx);
        }
        min_index = min_index.min(idx);
        max_index = max_index.max(idx);
        let addr = vb_data.wrapping_add(idx.saturating_mul(stride));
        if addr >= 0x2000_0000 || addr.saturating_add(20) > 0x2000_0000 {
            break;
        }

        let base = guest_mem as u64 + addr as u64;
        let blend_indices = shader_decl
            .v5
            .map(|elem| read_decl_vec4_pbyte4_raw(guest_mem, base, stride, elem))
            .unwrap_or([0.0; 4]);
        let blend_weights = shader_decl
            .v6
            .map(|elem| read_decl_vec4(guest_mem, base, stride, elem))
            .unwrap_or([0.0; 4]);
        let normal = shader_decl
            .v2
            .map(|elem| read_decl_vec4(guest_mem, base, stride, elem))
            .unwrap_or([0.0, 0.0, 1.0, 1.0]);
        let uv1 = shader_decl
            .v4
            .map(|elem| read_decl_vec4(guest_mem, base, stride, elem))
            .unwrap_or([0.0, 0.0, 0.0, 1.0]);
        unsafe {
            if let Some(v) = decode_no_shader_stride36_vertex(guest_mem, addr, stride, shader) {
                verts.push(v);
                skin_payload.push(VertexSkinPayload {
                    normal,
                    uv1,
                    blend_indices,
                    blend_weights,
                });
                continue;
            }
            let x = std::ptr::read_unaligned(base as *const f32);
            let y = std::ptr::read_unaligned((base + 4) as *const f32);
            let z = std::ptr::read_unaligned((base + 8) as *const f32);
            let (mut w, mut color, mut u, mut v) = if shader_hle_path && stride >= 28 {
                (
                    1.0,
                    std::ptr::read_unaligned((base + 12) as *const u32),
                    std::ptr::read_unaligned((base + 20) as *const f32),
                    std::ptr::read_unaligned((base + 24) as *const f32),
                )
            } else if stride >= 28 {
                (
                    std::ptr::read_unaligned((base + 12) as *const f32),
                    std::ptr::read_unaligned((base + 16) as *const u32),
                    std::ptr::read_unaligned((base + 20) as *const f32),
                    std::ptr::read_unaligned((base + 24) as *const f32),
                )
            } else if stride >= 20 {
                (
                    std::ptr::read_unaligned((base + 12) as *const f32),
                    std::ptr::read_unaligned((base + 16) as *const u32),
                    0.0,
                    0.0,
                )
            } else {
                (1.0, 0xFFFF_FFFF, 0.0, 0.0)
            };
            let (mut out_x, mut out_y, mut out_z) = (x, y, z);
            let mut transformed = false;
            let mut xform_label = selected_transform
                .map(|sel| sel.mode.label())
                .unwrap_or("fallback");
            if cpu_vs_xform {
                let exact_dp4_projected = if cpu_vs_exact_dp4_enabled()
                    && shader_simple_output
                    && shader_info
                        .as_ref()
                        .is_some_and(|info| info.simple_dp4_o_pos && !info.skinned)
                {
                    project_simple_dp4_tail(x, y, z)
                } else {
                    None
                };
                let projected = if let Some(projected) = exact_dp4_projected {
                    xform_label = "exact-dp4";
                    Some(projected)
                } else if cpu_vs_clip_output {
                    xform_label = selected_transform
                        .map(|sel| match sel.mode {
                            TransformMode::RowClip | TransformMode::ColClip => "clip-direct",
                            TransformMode::RowScreen | TransformMode::ColScreen => {
                                "screen-reverse-clip"
                            }
                        })
                        .unwrap_or("clip-missing");
                    selected_transform.and_then(|sel| {
                        apply_transform_clip_output(sel.matrix, sel.mode, sel.viewport, x, y, z)
                    })
                } else {
                    selected_transform
                        .and_then(|sel| {
                            apply_transform(sel.matrix, sel.mode, sel.viewport, x, y, z)
                        })
                        .or_else(|| project_with_c0_c3(x, y, z))
                };
                if let Some((sx, sy, sz, sw)) = projected {
                    out_x = sx;
                    out_y = sy;
                    if cpu_vs_flatten_z_enabled() {
                        out_z = 0.5;
                        w = 1.0;
                    } else {
                        out_z = sz;
                        w = sw;
                    }
                    transformed = true;
                }
                if shader_simple_output {
                    // The HLSL translator initializes oD0 from v1 before it
                    // executes shader tokens. If the guest shader does not
                    // explicitly write oD0, diffuse must still pass through.
                    color = if shader_diffuse_mul_c0 {
                        shader_apply_c0_diffuse(color, shader_c0)
                    } else {
                        shader_diffuse_color(color)
                    };
                    if shader_passes_tex0 {
                        if shader_tex0_add_c0 {
                            u += shader_c0[0];
                            v += shader_c0[1];
                        }
                    } else {
                        u = 0.0;
                        v = 0.0;
                    }
                } else {
                    // Fallback for skinned/complex shaders until the real NV2A
                    // microcode path exists. Keep the proved topology visible.
                    color = 0xFFFF_FFFF;
                    u = 0.0;
                    v = 0.0;
                }
            }
            verts.push(NV2AVertex {
                x: out_x,
                y: out_y,
                z: out_z,
                w,
                color,
                u,
                v,
            });
            skin_payload.push(VertexSkinPayload {
                normal,
                uv1,
                blend_indices,
                blend_weights,
            });
            if transformed {
                static XFORM_LOG: AtomicU32 = AtomicU32::new(0);
                let k = XFORM_LOG.fetch_add(1, Ordering::Relaxed);
                if k < 16 {
                    debug_log(&format!(
                        "[HLE-DRAWIDX-XFORM] #{} shader=0x{:08X} mode={} screenspace={} simple_dp4={} raw=({:.4},{:.4},{:.4}) -> screen=({:.1},{:.1},{:.3}) w={:.4} color=0x{:08X}",
                        k,
                        shader,
                        xform_label,
                        shader_info.as_ref().is_some_and(|info| info.screenspace),
                        shader_info.as_ref().is_some_and(|info| info.simple_dp4_o_pos),
                        x,
                        y,
                        z,
                        out_x,
                        out_y,
                        out_z,
                        w,
                        color
                    ));
                }
            }
        }
    }

    if verts.is_empty() {
        return;
    }
    if min_index == u32::MAX {
        min_index = 0;
    }

    let first = verts[0];
    let (min_x, max_x, min_y, max_y, min_z, max_z, finite_count) = verts.iter().fold(
        (
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            0usize,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z, finite_count), v| {
            let finite = v.x.is_finite() && v.y.is_finite() && v.z.is_finite();
            (
                min_x.min(v.x),
                max_x.max(v.x),
                min_y.min(v.y),
                max_y.max(v.y),
                min_z.min(v.z),
                max_z.max(v.z),
                finite_count + usize::from(finite),
            )
        },
    );
    let bbox_w = max_x - min_x;
    let bbox_h = max_y - min_y;
    let wild_cpu_xform_bbox = finite_count != verts.len()
        || min_x < -128.0
        || min_y < -128.0
        || max_x > 768.0
        || max_y > 608.0
        || bbox_w > 960.0
        || bbox_h > 720.0;
    if spidey_drop_wild_cpu_xform_enabled()
        && cpu_vs_xform
        && shader_screenspace
        && shader_simple_output
        && shader == 0x0000_1002
        && wild_cpu_xform_bbox
    {
        static WILD_CPU_XFORM_DROP_LOG: AtomicU32 = AtomicU32::new(0);
        let k = WILD_CPU_XFORM_DROP_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 64 || k.is_power_of_two() {
            debug_log(&format!(
                "[HLE-DRAWIDX-DROP-WILD-CPU-XFORM] #{} draw#{} shader=0x{:08X} prim={} indices={} expanded={} finite={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] size={:.1}x{:.1} tex0=0x{:08X} v0=({:.3},{:.3},{:.3},w={:.3}) c=0x{:08X} uv=({:.3},{:.3})",
                k,
                n,
                shader,
                prim_type,
                index_count,
                verts.len(),
                finite_count,
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
                bbox_w,
                bbox_h,
                stage0_tex,
                first.x,
                first.y,
                first.z,
                first.w,
                first.color,
                first.u,
                first.v
            ));
        }
        return;
    }

    log_spidey_skin_draw_probe(
        guest_mem,
        n,
        shader,
        prim_type,
        index_count,
        index_data_ptr,
        vb_data,
        stride,
        shader_translated,
        cpu_vs_xform,
        shader_info.as_ref(),
        shader_decl,
        &first_indices,
        min_index,
        max_index,
        &skin_payload,
        finite_count,
        (min_x, min_y, min_z, max_x, max_y, max_z),
    );

    let shader_uses_skin_inputs =
        stride == 0x2C || shader_info.as_ref().is_some_and(|info| info.skinned);
    if shader_uses_skin_inputs {
        static SKIN_INPUT_LOG: AtomicU32 = AtomicU32::new(0);
        let k = SKIN_INPUT_LOG.fetch_add(1, Ordering::Relaxed);
        if k < 16 || k.is_power_of_two() {
            let first = skin_payload.first().copied().unwrap_or_default();
            let sample = skin_payload
                .iter()
                .copied()
                .find(|p| {
                    p.blend_indices.iter().any(|v| v.abs() > 0.0001)
                        || p.blend_weights.iter().any(|v| v.abs() > 0.0001)
                })
                .unwrap_or(first);
            let max_v5 = skin_payload
                .iter()
                .flat_map(|p| p.blend_indices)
                .fold(0.0f32, |m, v| m.max(v.abs()));
            let max_v6 = skin_payload
                .iter()
                .flat_map(|p| p.blend_weights)
                .fold(0.0f32, |m, v| m.max(v.abs()));
            let v5 = shader_decl
                .v5
                .map(|e| format!("dt=0x{:02X}:off={}:sz={}", e.data_type, e.offset, e.size))
                .unwrap_or_else(|| "missing".to_string());
            let v6 = shader_decl
                .v6
                .map(|e| format!("dt=0x{:02X}:off={}:sz={}", e.data_type, e.offset, e.size))
                .unwrap_or_else(|| "missing".to_string());
            debug_log(&format!(
                "[VS-SKIN-INPUT] #{} draw#{} shader=0x{:08X} stride={} verts={} v5={} first_v5=[{:.3},{:.3},{:.3},{:.3}] sample_v5=[{:.3},{:.3},{:.3},{:.3}] max_v5={:.3} v6={} first_v6=[{:.3},{:.3},{:.3},{:.3}] sample_v6=[{:.3},{:.3},{:.3},{:.3}] max_v6={:.3}",
                k,
                n,
                shader,
                stride,
                verts.len(),
                v5,
                first.blend_indices[0],
                first.blend_indices[1],
                first.blend_indices[2],
                first.blend_indices[3],
                sample.blend_indices[0],
                sample.blend_indices[1],
                sample.blend_indices[2],
                sample.blend_indices[3],
                max_v5,
                v6,
                first.blend_weights[0],
                first.blend_weights[1],
                first.blend_weights[2],
                first.blend_weights[3],
                sample.blend_weights[0],
                sample.blend_weights[1],
                sample.blend_weights[2],
                sample.blend_weights[3],
                max_v6,
            ));
        }
    }

    let capture_hle_drawidx = spidey_capture_hle_drawidx_requested(n);
    if capture_hle_drawidx || n < 20 || (n % 1000 == 0) {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-DRAWIDX] #{} expanded={} finite={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] v0=({:.3},{:.3},{:.3},w={:.3}) c=0x{:08X} uv=({:.3},{:.3})",
            n,
            verts.len(),
            finite_count,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            first.x,
            first.y,
            first.z,
            first.w,
            first.color,
            first.u,
            first.v
        ));
    }
    let suspect_red_texture = stage0_tex == 0x03E2_1930 || stage0_tex == 0x83E2_1930;
    if (n < 8 || (suspect_red_texture && n % 1000 == 0))
        && shader == 0x0000_1002
        && shader_screenspace
        && shader_info
            .as_ref()
            .is_some_and(|info| info.simple_dp4_o_pos)
    {
        log_simple_dp4_probe(n, shader, &verts);
    }
    log_spidey_select_circle_draw_probe(
        "DrawIndexedVertices",
        n,
        prim_type,
        &verts,
        shader,
        "pre-backend",
    );

    let nv_prim = match prim_type {
        1 => crate::xbox::gpu::NV097_POINTS,
        2 => crate::xbox::gpu::NV097_LINES,
        3 | 4 => crate::xbox::gpu::NV097_LINE_STRIP,
        5 => crate::xbox::gpu::NV097_TRIANGLES,
        6 => crate::xbox::gpu::NV097_TRIANGLES,
        7 => crate::xbox::gpu::NV097_TRIANGLE_FAN,
        8 => crate::xbox::gpu::NV097_QUAD_LIST,
        _ => crate::xbox::gpu::NV097_TRIANGLES,
    };
    refresh_dirty_active_textures_before_draw("DrawIndexedVertices", guest_mem);

    let mut guard = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut gpu) = *guard {
        let capture_shader1001 = shader_hle_path && shader == 0x0000_1001 && {
            static SHADER1001_CAPTURED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            !SHADER1001_CAPTURED.swap(true, Ordering::Relaxed)
        };
        let capture_n = if shader_hle_path {
            static DRAWIDX_BMP_N: AtomicU32 = AtomicU32::new(0);
            Some(DRAWIDX_BMP_N.fetch_add(1, Ordering::Relaxed))
        } else {
            None
        };
        if shader_hle_path && cpu_vs_xform {
            static SHADEROUT_LOG: AtomicU32 = AtomicU32::new(0);
            let k = SHADEROUT_LOG.fetch_add(1, Ordering::Relaxed);
            if cpu_vs_clip_output {
                gpu.set_vertex_shader_hlsl(0xFFFF_0002, Some(CPU_VS_CLIP_PASSTHROUGH_HLSL));
            } else {
                gpu.set_vertex_shader_hlsl(0, None);
            }
            if shader_simple_output {
                if k < 12 {
                    debug_log(&format!(
                        "[HLE-DRAWIDX-SHADEROUT] #{} shader=0x{:08X} mode={} diffuse={} mul_c0={} tex0={} add_c0={} c0=[{:.3},{:.3},{:.3},{:.3}] texture={}",
                        k,
                        shader,
                        if cpu_vs_clip_output {
                            "cpu-xform/clip-passthrough"
                        } else {
                            "cpu-xform/screen-passthrough"
                        },
                        shader_passes_diffuse,
                        shader_diffuse_mul_c0,
                        shader_passes_tex0,
                        shader_tex0_add_c0,
                        shader_c0[0],
                        shader_c0[1],
                        shader_c0[2],
                        shader_c0[3],
                        if shader_passes_tex0 { "preserve" } else { "clear" }
                    ));
                }
                if !shader_passes_tex0 {
                    log_spidey_select_circle_draw_probe(
                        "DrawIndexedVertices",
                        n,
                        prim_type,
                        &verts,
                        shader,
                        "backend-clear-no-tex0",
                    );
                    for stage in 0..4 {
                        hle_settex_cache_clear(stage);
                        gpu.clear_texture(stage);
                    }
                }
            } else {
                if k < 12 {
                    debug_log("[HLE-DRAWIDX-DIAG] shader path forced untextured white");
                }
                log_spidey_select_circle_draw_probe(
                    "DrawIndexedVertices",
                    n,
                    prim_type,
                    &verts,
                    shader,
                    "backend-clear-complex-shader",
                );
                for stage in 0..4 {
                    hle_settex_cache_clear(stage);
                    gpu.clear_texture(stage);
                }
            }
        }
        if capture_n.is_some_and(|k| k < 3) && spidey_isolate_drawidx_enabled() {
            gpu.clear(0xFF00_0000);
        }
        let (rt_key, rt_w, rt_h, _, _) =
            gpu.debug_active_render_target().unwrap_or((0, 0, 0, 0, 0));
        let skinned_decl = stride == 0x2C || shader_info.as_ref().is_some_and(|info| info.skinned);
        crate::xbox::emulator::note_spidey_synth_draw(
            n as u64,
            shader,
            stride,
            skinned_decl,
            rt_key,
            rt_w,
            rt_h,
        );
        crate::xbox::gpu::set_next_draw_skin_payload(skin_payload);
        gpu.draw_primitive(&verts, nv_prim);
        if spidey_env_flag("RUSTEMU_SPIDEY_CAPTURE_WORLD_DRAWS")
            && stride != 0x2C
            && verts.len() >= 900
            && finite_count == verts.len()
            && !shader_info.as_ref().is_some_and(|info| info.skinned)
        {
            static WORLD_DRAW_CAPTURE_N: AtomicU32 = AtomicU32::new(0);
            let seq = WORLD_DRAW_CAPTURE_N.fetch_add(1, Ordering::Relaxed);
            if seq < 32 {
                let pixels = gpu.readback_framebuffer().to_vec();
                let path = format!(
                    r"./originz_world_after_draw_{:02}_drawidx_{:05}.bmp",
                    seq, n
                );
                write_readback_bmp(&path, 640, 480, &pixels);
                let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
                    sampled_argb_stats(&pixels);
                let active_rt = gpu.debug_active_render_target();
                let (rt_key, rt_w, rt_h, rt_data, rt_pitch) = active_rt.unwrap_or((0, 0, 0, 0, 0));
                let rs = crate::xbox::gpu::render_state::GLOBAL_RENDER_STATE
                    .lock()
                    .map(|state| *state)
                    .unwrap_or_default();
                let c0 = current_vs_constant(0);
                let c1 = current_vs_constant(1);
                let c2 = current_vs_constant(2);
                let c3 = current_vs_constant(3);
                let c4 = current_vs_constant(4);
                let c5 = current_vs_constant(5);
                let c6 = current_vs_constant(6);
                let c7 = current_vs_constant(7);
                let c58 = current_vs_constant(58);
                let c59 = current_vs_constant(59);
                debug_log(&format!(
                    "[SPIDEY-WORLD-DRAW-CAPTURE] seq={} draw#{} shader=0x{:08X} tex0=0x{:08X} prim={} xbox_prim={} stride={} vb=0x{:08X} indices={} expanded={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] screen_size={:.1}x{:.1} sample_nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} rt=0x{:08X} {}x{} data=0x{:08X} pitch={} lighting={} ambient=0x{:08X} tfactor=0x{:08X} light_mask=0x{:08X} light_seq={} material_seq={} fog={} fog_color=0x{:08X} z={} zwrite={} alpha_blend={} color_write=0x{:X} c0=[{:.3},{:.3},{:.3},{:.3}] c1=[{:.3},{:.3},{:.3},{:.3}] c2=[{:.3},{:.3},{:.3},{:.3}] c3=[{:.3},{:.3},{:.3},{:.3}] c4=[{:.3},{:.3},{:.3},{:.3}] c5=[{:.3},{:.3},{:.3},{:.3}] c6=[{:.3},{:.3},{:.3},{:.3}] c7=[{:.3},{:.3},{:.3},{:.3}] c58=[{:.3},{:.3},{:.3},{:.3}] c59=[{:.3},{:.3},{:.3},{:.3}] path={}",
                    seq,
                    n,
                    shader,
                    stage0_tex,
                    nv_prim,
                    prim_type,
                    stride,
                    vb_data,
                    index_count,
                    verts.len(),
                    min_x,
                    min_y,
                    min_z,
                    max_x,
                    max_y,
                    max_z,
                    bbox_w,
                    bbox_h,
                    sample_nonzero,
                    sample_alpha,
                    sample_rgb,
                    first_px,
                    mid_px,
                    rt_key,
                    rt_w,
                    rt_h,
                    rt_data,
                    rt_pitch,
                    rs.lighting,
                    rs.ambient,
                    rs.texture_factor,
                    LIGHT_ENABLE_MASK.load(Ordering::Relaxed),
                    LIGHT_STATE_SEQ.load(Ordering::Relaxed),
                    MATERIAL_STATE_SEQ.load(Ordering::Relaxed),
                    rs.fog_enable,
                    rs.fog_color,
                    rs.z_enable,
                    rs.z_write_enable,
                    rs.alpha_blend_enable,
                    rs.color_write_enable,
                    c0[0],
                    c0[1],
                    c0[2],
                    c0[3],
                    c1[0],
                    c1[1],
                    c1[2],
                    c1[3],
                    c2[0],
                    c2[1],
                    c2[2],
                    c2[3],
                    c3[0],
                    c3[1],
                    c3[2],
                    c3[3],
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
                    path
                ));
            }
        }
        capture_spidey_select_circle_after_draw(
            gpu.as_mut(),
            "DrawIndexedVertices",
            n,
            prim_type,
            &verts,
        );
        capture_spidey_focus_texture_after_draw(
            gpu.as_mut(),
            "DrawIndexedVertices",
            n,
            prim_type,
            shader,
            &verts,
        );
        if capture_hle_drawidx {
            let pixels = gpu.readback_framebuffer().to_vec();
            let path = format!(r"./gate3_hle_after_drawidx_{:05}.bmp", n);
            write_readback_bmp(&path, 640, 480, &pixels);
            let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
                sampled_argb_stats(&pixels);
            let rs = crate::xbox::gpu::render_state::GLOBAL_RENDER_STATE
                .lock()
                .map(|state| *state)
                .unwrap_or_default();
            let tss = texture_stage_snapshot();
            let s0 = &tss.states[0];
            let ps = pixel_shader_state_snapshot();
            let px = |x: usize, y: usize| -> u32 {
                pixels
                    .get(y.saturating_mul(640).saturating_add(x))
                    .copied()
                    .unwrap_or(0)
            };
            debug_log(&format!(
                "[HLE-DRAWIDX-CAPTURE] draw#{} shader=0x{:08X} tex0=0x{:08X} prim={} verts={} bbox=[{:.1},{:.1},{:.3}..{:.1},{:.1},{:.3}] nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} px_left=0x{:08X} px_center=0x{:08X} px_right=0x{:08X} blend={} src={:?} dst={:?} op={} alpha_test={} alpha_ref={} alpha_func={:?} z={} zwrite={} zfunc={:?} color_write=0x{:X} tss_color={}({},{},{}) tss_alpha={}({},{},{}) ps_combiner={} rgb0=0x{:08X} alpha0=0x{:08X} path={}",
                n,
                shader,
                stage0_tex,
                nv_prim,
                verts.len(),
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
                sample_nonzero,
                sample_alpha,
                sample_rgb,
                first_px,
                mid_px,
                px(160, 240),
                px(320, 240),
                px(480, 240),
                rs.alpha_blend_enable,
                rs.src_blend,
                rs.dest_blend,
                rs.blend_op,
                rs.alpha_test_enable,
                rs.alpha_ref,
                rs.alpha_func,
                rs.z_enable,
                rs.z_write_enable,
                rs.z_func,
                rs.color_write_enable,
                texture_op_name(s0[12]),
                texture_arg_name(s0[13]),
                texture_arg_name(s0[14]),
                texture_arg_name(s0[15]),
                texture_op_name(s0[16]),
                texture_arg_name(s0[17]),
                texture_arg_name(s0[18]),
                texture_arg_name(s0[19]),
                ps.combiner_count,
                ps.rgb_inputs0,
                ps.alpha_inputs0,
                path
            ));
        }
        if capture_shader1001 {
            let pixels = gpu.readback_framebuffer().to_vec();
            maybe_cache_spidey_bonus_menu_present(guest_mem, &pixels, "shader1001-draw");
            write_readback_bmp(
                r"./organic_shader1001_after_draw.bmp",
                640,
                480,
                &pixels,
            );
            let (sample_nonzero, sample_alpha, sample_rgb, first_px, mid_px) =
                sampled_argb_stats(&pixels);
            debug_log(&format!(
                "[HLE-DRAWIDX-SHADER1001-CAPTURE] shader=0x{:08X} tex0=0x{:08X} prim={} verts={} nonzero={}/1024 alpha={}/1024 rgb={}/1024 first=0x{:08X} mid=0x{:08X} path=./organic_shader1001_after_draw.bmp",
                shader,
                stage0_tex,
                nv_prim,
                verts.len(),
                sample_nonzero,
                sample_alpha,
                sample_rgb,
                first_px,
                mid_px
            ));
        }
        if let Some(bmp_n) = capture_n {
            let should_present = bmp_n < 3 || bmp_n % 64 == 0;
            if should_present {
                let pixels = gpu.readback_framebuffer().to_vec();
                maybe_cache_spidey_bonus_menu_present(guest_mem, &pixels, "drawidx-present");
                write_readback_bmp(
                    r"./organic_latest_drawidx_present.bmp",
                    640,
                    480,
                    &pixels,
                );
                if bmp_n < 3 {
                    let path = format!(r"./organic_latest_drawidx_{:02}.bmp", bmp_n);
                    write_readback_bmp(&path, 640, 480, &pixels);
                    debug_log(&format!(
                        "[HLE-DRAWIDX-DIAG] wrote immediate readback {}",
                        path
                    ));
                }
            }
        }
    }
    drop(guard);
}

/// DirectSoundCreate: delegate to canonical APU HLE implementation
pub(super) fn hle_dsound_create(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    crate::xbox::apu::dsound::hle_dsound_create(args, guest_mem)
}

/// Public wrapper for hle_swap — callable from external frame loop hooks.
pub fn hle_swap_external(guest_mem: *mut u8, mmio_count: &mut u64) {
    hle_swap_with_flags(guest_mem, mmio_count, X_D3DSWAP_DEFAULT);
}

/// Public wrapper for Xbox D3DDevice::Swap(DWORD Flags).
pub fn hle_swap_external_with_flags(guest_mem: *mut u8, mmio_count: &mut u64, flags: u32) {
    hle_swap_with_flags(guest_mem, mmio_count, flags);
}

/// HLE stub for IDirect3DResource8::Register(this, pBase).
/// Thiscall: args[0] = pBase (ECX = this is passed separately via RuntimeContext).
/// Reads the 20-byte X_D3DResource header at `this`, records metadata for
/// later use at SetTexture binding. Upload to GPU happens via the existing
/// hle_set_texture path on next bind.
pub(super) fn hle_register_resource(ecx: u32, args: &[u32; 8], guest_mem: *mut u8) {
    use crate::xbox::gpu::xpr::XboxTexHeader;

    #[derive(Clone, Copy)]
    struct RegisterCandidate {
        this_ptr: u32,
        norm_this: u32,
        p_base: u32,
        hdr: XboxTexHeader,
        hdr_bytes: [u8; 20],
        res_type: u32,
        style: &'static str,
    }

    fn decode_register_candidate(
        guest_mem: *mut u8,
        this_ptr: u32,
        p_base: u32,
        style: &'static str,
    ) -> Option<RegisterCandidate> {
        let norm_this = normalize_guest_ram_ptr(this_ptr)?;
        if norm_this < 0x0001_0000 || norm_this.saturating_add(20) > 0x2000_0000 {
            return None;
        }

        let mut hdr_bytes = [0u8; 20];
        unsafe {
            std::ptr::copy_nonoverlapping(
                guest_mem.add(norm_this as usize),
                hdr_bytes.as_mut_ptr(),
                hdr_bytes.len(),
            );
        }

        let hdr = XboxTexHeader::from_resource_header(&hdr_bytes)?;
        let common = u32::from_le_bytes([hdr_bytes[0], hdr_bytes[1], hdr_bytes[2], hdr_bytes[3]]);
        let res_type = (common >> 16) & 0x7;
        Some(RegisterCandidate {
            this_ptr,
            norm_this,
            p_base,
            hdr,
            hdr_bytes,
            res_type,
            style,
        })
    }

    // XDK 4134's Register body returns `ret 8`, so the canonical form is
    // stdcall(resource, pBase). ECX can still contain a guest-looking value
    // from surrounding code, so validate both candidates and prefer the stack
    // resource when it decodes as a texture.
    let candidates = [
        decode_register_candidate(guest_mem, args[0], args[1], "stdcall"),
        decode_register_candidate(guest_mem, ecx, args[0], "thiscall"),
    ];
    let Some(candidate) = candidates
        .iter()
        .flatten()
        .copied()
        .find(|c| c.res_type == 3)
        .or_else(|| candidates.iter().flatten().copied().next())
    else {
        if spidey_env_flag("RUSTEMU_SPIDEY_PETERSTU_RESOURCE_TRACE")
            && crate::xbox::emulator::spidey_peterstu_read_ready_seq() != 0
        {
            static SPIDEY_REG_MISS_LOG: AtomicU32 = AtomicU32::new(0);
            let sn = SPIDEY_REG_MISS_LOG.fetch_add(1, Ordering::Relaxed);
            if sn < 64 || sn.is_power_of_two() {
                debug_log(&format!(
                    "[SPIDEY-PETERSTU-REG-MISS] #{} ecx=0x{:08X} args=[0x{:08X},0x{:08X}] ecx_hex=[{}] arg0_hex=[{}] arg1_hex=[{}]",
                    sn,
                    ecx,
                    args[0],
                    args[1],
                    spidey_hex_preview(guest_mem, ecx, 32),
                    spidey_hex_preview(guest_mem, args[0], 32),
                    spidey_hex_preview(guest_mem, args[1], 32)
                ));
            }
        }
        static REG_MISS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = REG_MISS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 20 {
            debug_log(&format!(
                "[HLE-REG-MISS] #{} ecx=0x{:08X} args=[0x{:08X},0x{:08X}]",
                n, ecx, args[0], args[1]
            ));
        }
        return;
    };

    let this_ptr = candidate.this_ptr;
    let p_base = candidate.p_base;
    let hdr = candidate.hdr;
    let res_type = candidate.res_type;
    let common = u32::from_le_bytes([
        candidate.hdr_bytes[0],
        candidate.hdr_bytes[1],
        candidate.hdr_bytes[2],
        candidate.hdr_bytes[3],
    ]);
    let format_dw = u32::from_le_bytes([
        candidate.hdr_bytes[12],
        candidate.hdr_bytes[13],
        candidate.hdr_bytes[14],
        candidate.hdr_bytes[15],
    ]);
    debug_log(&format!(
        "[HLE-REG] fire! {} this=0x{:08X} pBase=0x{:08X} type={} fmt=0x{:02X} {}x{}",
        candidate.style, this_ptr, p_base, res_type, hdr.format_code, hdr.width, hdr.height
    ));

    // Compute pixel address. The source Data field is an offset from pBase;
    // Register's side effect is to write the resolved guest address back into
    // X_D3DResource.Data so later SetTexture/Lock calls see the real storage.
    let data_off = hdr.data_offset & !0x3;
    let resolved_raw = p_base.wrapping_add(data_off);
    let pixel_addr = resolved_raw & 0x1FFF_FFFC;
    let registered_data = pixel_addr | 0x8000_0000;
    let size = hdr.mip0_size();

    spidey_peterstu_resource_trace(
        guest_mem, this_ptr, p_base, pixel_addr, common, format_dw, data_off, size,
    );

    if size == 0 || size > 16 * 1024 * 1024 {
        return;
    }
    if (pixel_addr as usize + size) > 0x2000_0000 {
        return;
    }

    unsafe {
        std::ptr::write_unaligned(
            guest_mem.add(candidate.norm_this as usize + 0x04) as *mut u32,
            registered_data,
        );
    }

    // Copy pixels
    let mut pixels = vec![0u8; size];
    unsafe {
        std::ptr::copy_nonoverlapping(
            guest_mem.add(pixel_addr as usize),
            pixels.as_mut_ptr(),
            size,
        );
    }

    // Log first 20 registrations
    static REG_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = REG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 20 {
        crate::xbox::aot::veh::veh_log(&format!(
            "[D3D-REGISTER] #{} {} this=0x{:08X}->0x{:08X} pBase=0x{:08X} type={} fmt=0x{:02X} {}x{} data_addr=0x{:08X} stored=0x{:08X} size={} comp={} swz={}",
            n, candidate.style, this_ptr, candidate.norm_this, p_base, res_type, hdr.format_code,
            hdr.width, hdr.height, pixel_addr, registered_data, size,
            hdr.is_compressed, hdr.is_swizzled
        ));
    }

    if res_type == 3 {
        upsert_texture_info(HleTextureInfo {
            key: candidate.norm_this,
            width: hdr.width,
            height: hdr.height,
            format: format_dw,
            data: pixel_addr,
            pitch: texture_pitch_bytes(hdr.width, format_dw),
            swizzled: hdr.is_swizzled,
        });

        // Upload via GPU backend at stage 0 (will be re-bound by SetTexture later).
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            hle_settex_cache_clear(0);
            backend.set_texture_swizzle_hint(0, hdr.is_swizzled && hdr.is_compressed);
            backend.set_texture_raw(0, hdr.width, hdr.height, hdr.format_code, &pixels);
        }
    }
}
