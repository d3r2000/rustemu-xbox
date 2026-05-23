//! Minimal NV2A vertex shader token capture and HLSL emission.
//!
//! This is the first Rust-side foundation for the Cxbx-R/xemu style path:
//! keep the raw 16-byte NV2A microcode tokens, decode them, and let D3D11 run
//! the transform. The parser intentionally starts small (MOV/MUL/ADD/MAD/DP3/
//! DP4 plus the first skinning/lighting ops) but the token field map matches xemu/Cxbx-R so the file can grow into
//! the full translator instead of extending CPU-side transform selectors.

use std::sync::{Mutex, OnceLock};

const TOKEN_DWORDS: usize = 4;
const MAX_CAPTURED_SHADERS: usize = 64;

#[derive(Clone, Debug)]
pub struct CapturedShader {
    pub tokens: Vec<u32>,
    pub hlsl: String,
    pub source_snippet: String,
    pub decoded_lines: Vec<String>,
    pub screenspace: bool,
}

#[derive(Clone)]
struct RegisteredShader {
    handle: u32,
    shader: CapturedShader,
}

static PENDING_SHADERS: OnceLock<Mutex<Vec<CapturedShader>>> = OnceLock::new();
static REGISTERED_SHADERS: OnceLock<Mutex<Vec<RegisteredShader>>> = OnceLock::new();

fn pending_shaders() -> &'static Mutex<Vec<CapturedShader>> {
    PENDING_SHADERS.get_or_init(|| Mutex::new(Vec::new()))
}

fn registered_shaders() -> &'static Mutex<Vec<RegisteredShader>> {
    REGISTERED_SHADERS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn register_pending_xgrph_shader(shader: CapturedShader) {
    let mut pending = pending_shaders().lock().unwrap_or_else(|e| e.into_inner());
    pending.push(shader);
    if pending.len() > MAX_CAPTURED_SHADERS {
        pending.remove(0);
    }
}

pub fn take_pending_xgrph_shader() -> Option<CapturedShader> {
    let mut pending = pending_shaders().lock().unwrap_or_else(|e| e.into_inner());
    if pending.is_empty() {
        None
    } else {
        Some(pending.remove(0))
    }
}

pub fn register_shader_handle(handle: u32, shader: CapturedShader) {
    let mut shaders = registered_shaders()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    shaders.retain(|old| old.handle != handle);
    shaders.push(RegisteredShader { handle, shader });
    if shaders.len() > MAX_CAPTURED_SHADERS {
        shaders.remove(0);
    }
}

pub fn hlsl_for_handle(handle: u32) -> Option<String> {
    registered_shaders()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .rev()
        .find(|shader| shader.handle == handle)
        .map(|shader| shader.shader.hlsl.clone())
}

pub fn is_registered_shader_handle(handle: u32) -> bool {
    registered_shaders()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .rev()
        .any(|shader| shader.handle == handle)
}

pub fn shader_has_hlsl(handle: u32) -> bool {
    hlsl_for_handle(handle).is_some()
}

pub fn first_token_words(tokens: &[u32], max_words: usize) -> String {
    tokens
        .iter()
        .take(max_words)
        .map(|w| format!("0x{w:08X}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn capture_from_xgrph_source(source: &str) -> CapturedShader {
    let screenspace = source.to_ascii_lowercase().contains("#pragma screenspace");
    let spidey_d_canary = spidey_xgrph_d_canary_source_enabled(source);
    let mut tokens = assemble_source_to_tokens(source, spidey_d_canary);
    if tokens.is_empty() {
        tokens = fallback_transform_tokens();
    }
    let color_const_patches = patch_color_constants_by_lowered_expr(&mut tokens);
    if color_const_patches != 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[NV2A-VSH-SOURCE-COLOR-CONST-PATCH] patches={} source='{}'",
            color_const_patches,
            source
                .split_whitespace()
                .take(32)
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    if let Some(last) = tokens.chunks_exact_mut(TOKEN_DWORDS).last() {
        set_field(last, Field::Final, 1);
    }
    let decoded_lines = decode_lines(&tokens);
    let hlsl = translate_tokens_to_hlsl(&tokens, screenspace);
    CapturedShader {
        tokens,
        hlsl,
        source_snippet: source
            .split_whitespace()
            .take(32)
            .collect::<Vec<_>>()
            .join(" "),
        decoded_lines,
        screenspace,
    }
}

pub fn capture_from_native_xgrph_tokens(
    words: Vec<u32>,
    screenspace: bool,
    source_snippet: String,
    supplement_skin_index: bool,
    source_normalized_tokens: Option<&[u32]>,
) -> CapturedShader {
    let mut tokens = native_xgrph_program_words(words);
    tokens.truncate((tokens.len() / TOKEN_DWORDS) * TOKEN_DWORDS);
    if let Some(final_insn) = tokens
        .chunks_exact(TOKEN_DWORDS)
        .position(|token| get_field(token, Field::Final) != 0)
    {
        tokens.truncate((final_insn + 1) * TOKEN_DWORDS);
    }
    if supplement_skin_index {
        supplement_native_skin_index_missing_operands(&mut tokens);
    }
    let mut color_const_patches =
        patch_native_color_constants_from_source(&mut tokens, source_normalized_tokens);
    if color_const_patches == 0 {
        color_const_patches += patch_native_color_constants_by_window(&mut tokens);
    }
    if color_const_patches == 0 {
        color_const_patches += patch_color_constants_by_lowered_expr(&mut tokens);
    }
    if color_const_patches != 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[NV2A-VSH-NATIVE-COLOR-CONST-PATCH] patches={} source='{}'",
            color_const_patches, source_snippet
        ));
    }
    let decoded_lines = decode_lines(&tokens);
    let hlsl = translate_tokens_to_hlsl(&tokens, screenspace);
    CapturedShader {
        tokens,
        hlsl,
        source_snippet,
        decoded_lines,
        screenspace,
    }
}

fn native_xgrph_program_words(words: Vec<u32>) -> Vec<u32> {
    if let Some(&header) = words.first() {
        let count = (header >> 16) as usize;
        let has_xvs_header = (header & 0xFFFF) == 0x2078;
        let needed = 1usize.saturating_add(count.saturating_mul(TOKEN_DWORDS));
        if has_xvs_header && count != 0 && words.len() >= needed {
            return words[1..needed].to_vec();
        }
    }
    words
}

fn patch_native_color_constants_from_source(
    native_tokens: &mut [u32],
    source_normalized_tokens: Option<&[u32]>,
) -> usize {
    let Some(source_tokens) = source_normalized_tokens else {
        return 0;
    };
    let source_color_raw = encode_signed_c_register(-80).unwrap_or(16);
    let native_blue_bias_raw = encode_signed_c_register(-96).unwrap_or(0);
    let mut patched = 0usize;

    for (native, source) in native_tokens
        .chunks_exact_mut(TOKEN_DWORDS)
        .zip(source_tokens.chunks_exact(TOKEN_DWORDS))
    {
        if get_field(source, Field::Const) != source_color_raw {
            continue;
        }
        if get_field(native, Field::Const) != native_blue_bias_raw {
            continue;
        }
        if get_field(source, Field::OutMux) != 0 || get_field(native, Field::OutMux) != 0 {
            continue;
        }
        if !token_uses_const(source) || !token_uses_const(native) {
            continue;
        }
        if !same_instruction_shape_for_const_patch(native, source) {
            continue;
        }

        set_field(native, Field::Const, source_color_raw);
        patched += 1;
    }

    patched
}

fn token_uses_const(token: &[u32]) -> bool {
    get_field(token, Field::AMux) == ParamType::Const as u32
        || get_field(token, Field::BMux) == ParamType::Const as u32
        || get_field(token, Field::CMux) == ParamType::Const as u32
}

fn patch_native_color_constants_by_window(native_tokens: &mut [u32]) -> usize {
    let source_color_raw = encode_signed_c_register(-80).unwrap_or(16);
    let native_blue_bias_raw = encode_signed_c_register(-96).unwrap_or(0);
    let mut patched = 0usize;
    let mut color_window = 0usize;

    for token in native_tokens.chunks_exact_mut(TOKEN_DWORDS) {
        let is_raw_c0 = get_field(token, Field::Const) == native_blue_bias_raw
            && get_field(token, Field::OutMux) == 0
            && token_uses_const(token);
        if is_raw_c0 && temp_dest_is(token, 11) {
            set_field(token, Field::Const, source_color_raw);
            patched += 1;
            color_window = 5;
            continue;
        }

        if color_window != 0 {
            if is_raw_c0 && (temp_dest_is(token, 0) || temp_dest_is(token, 11)) {
                set_field(token, Field::Const, source_color_raw);
                patched += 1;
            }
            color_window -= 1;
        }
    }

    patched
}

fn patch_color_constants_by_lowered_expr(native_tokens: &mut [u32]) -> usize {
    let source_color_raw = encode_signed_c_register(-80).unwrap_or(16);
    let mut patched = 0usize;
    let mut color_window = 0usize;

    for token in native_tokens.chunks_exact_mut(TOKEN_DWORDS) {
        let dst = dest_expr(token).map(|(dst, _)| dst).unwrap_or_default();
        let expr = hlsl_expr_for_token(token).unwrap_or_default();
        let is_color_seed = dst == "R11"
            && expr.contains("v1")
            && expr.contains("c[0]")
            && !expr.contains("nv2a_const_index");

        if is_color_seed {
            set_field(token, Field::Const, source_color_raw);
            patched += 1;
            color_window = 5;
            continue;
        }

        if color_window != 0 {
            let is_color_followup = (dst == "R0" || dst == "R11")
                && expr.contains("c[0]")
                && !expr.contains("nv2a_const_index");
            if is_color_followup {
                set_field(token, Field::Const, source_color_raw);
                patched += 1;
            }
            color_window -= 1;
        }
    }

    patched
}

fn temp_dest_is(token: &[u32], reg: u32) -> bool {
    get_field(token, Field::OutMacMask) != 0
        && get_field(token, Field::OutR) == reg
        && get_field(token, Field::OutOMask) == 0
        && get_field(token, Field::OutIluMask) == 0
}

fn same_instruction_shape_for_const_patch(a: &[u32], b: &[u32]) -> bool {
    get_field(a, Field::Mac) == get_field(b, Field::Mac)
        && get_field(a, Field::Ilu) == get_field(b, Field::Ilu)
        && get_field(a, Field::OutMacMask) == get_field(b, Field::OutMacMask)
        && get_field(a, Field::OutIluMask) == get_field(b, Field::OutIluMask)
        && get_field(a, Field::OutOMask) == get_field(b, Field::OutOMask)
        && get_field(a, Field::OutR) == get_field(b, Field::OutR)
        && get_field(a, Field::OutAddress) == get_field(b, Field::OutAddress)
        && get_field(a, Field::AMux) == get_field(b, Field::AMux)
        && get_field(a, Field::BMux) == get_field(b, Field::BMux)
        && get_field(a, Field::CMux) == get_field(b, Field::CMux)
        && get_field(a, Field::V) == get_field(b, Field::V)
}

fn supplement_native_skin_index_missing_operands(tokens: &mut [u32]) {
    for token in tokens.chunks_exact_mut(TOKEN_DWORDS) {
        if MacOp::from_u32(get_field(token, Field::Mac)) == MacOp::Mad
            && get_field(token, Field::OutR) == 4
            && get_field(token, Field::OutMacMask) == 0xF
            && get_field(token, Field::AMux) == ParamType::Input as u32
            && get_field(token, Field::V) == 5
            && get_field(token, Field::BMux) == ParamType::Const as u32
            && get_field(token, Field::CMux) == ParamType::Const as u32
        {
            set_field(token, Field::BMux, ParamType::Unknown as u32);
            set_field(token, Field::CMux, ParamType::Unknown as u32);
            set_field(token, Field::BNeg, 0);
            set_field(token, Field::CNeg, 0);
        }
    }
}

fn spidey_xgrph_d_canary_source_enabled(source: &str) -> bool {
    if !spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_D_CANARY") {
        return false;
    }
    let lower = source.to_ascii_lowercase();
    lower.contains("#pragma screenspace")
        && lower.contains(";---- vsf_sphere_enviromap")
        && lower.contains("add r_reflection.z, r_reflection.z,")
        && lower.contains("sub r2.y, , r2.y")
}

#[derive(Clone, Copy)]
enum Field {
    Ilu,
    Mac,
    Const,
    V,
    ANeg,
    ASwzX,
    ASwzY,
    ASwzZ,
    ASwzW,
    AR,
    AMux,
    BNeg,
    BSwzX,
    BSwzY,
    BSwzZ,
    BSwzW,
    BR,
    BMux,
    CNeg,
    CSwzX,
    CSwzY,
    CSwzZ,
    CSwzW,
    CRHigh,
    CRLow,
    CMux,
    OutMacMask,
    OutR,
    OutIluMask,
    OutOMask,
    OutOrb,
    OutAddress,
    OutMux,
    A0X,
    Final,
}

#[derive(Clone, Copy)]
struct FieldMapping {
    subtoken: usize,
    start_bit: u8,
    bit_len: u8,
}

fn field_mapping(field: Field) -> FieldMapping {
    match field {
        Field::Ilu => FieldMapping {
            subtoken: 1,
            start_bit: 25,
            bit_len: 3,
        },
        Field::Mac => FieldMapping {
            subtoken: 1,
            start_bit: 21,
            bit_len: 4,
        },
        Field::Const => FieldMapping {
            subtoken: 1,
            start_bit: 13,
            bit_len: 8,
        },
        Field::V => FieldMapping {
            subtoken: 1,
            start_bit: 9,
            bit_len: 4,
        },
        Field::ANeg => FieldMapping {
            subtoken: 1,
            start_bit: 8,
            bit_len: 1,
        },
        Field::ASwzX => FieldMapping {
            subtoken: 1,
            start_bit: 6,
            bit_len: 2,
        },
        Field::ASwzY => FieldMapping {
            subtoken: 1,
            start_bit: 4,
            bit_len: 2,
        },
        Field::ASwzZ => FieldMapping {
            subtoken: 1,
            start_bit: 2,
            bit_len: 2,
        },
        Field::ASwzW => FieldMapping {
            subtoken: 1,
            start_bit: 0,
            bit_len: 2,
        },
        Field::AR => FieldMapping {
            subtoken: 2,
            start_bit: 28,
            bit_len: 4,
        },
        Field::AMux => FieldMapping {
            subtoken: 2,
            start_bit: 26,
            bit_len: 2,
        },
        Field::BNeg => FieldMapping {
            subtoken: 2,
            start_bit: 25,
            bit_len: 1,
        },
        Field::BSwzX => FieldMapping {
            subtoken: 2,
            start_bit: 23,
            bit_len: 2,
        },
        Field::BSwzY => FieldMapping {
            subtoken: 2,
            start_bit: 21,
            bit_len: 2,
        },
        Field::BSwzZ => FieldMapping {
            subtoken: 2,
            start_bit: 19,
            bit_len: 2,
        },
        Field::BSwzW => FieldMapping {
            subtoken: 2,
            start_bit: 17,
            bit_len: 2,
        },
        Field::BR => FieldMapping {
            subtoken: 2,
            start_bit: 13,
            bit_len: 4,
        },
        Field::BMux => FieldMapping {
            subtoken: 2,
            start_bit: 11,
            bit_len: 2,
        },
        Field::CNeg => FieldMapping {
            subtoken: 2,
            start_bit: 10,
            bit_len: 1,
        },
        Field::CSwzX => FieldMapping {
            subtoken: 2,
            start_bit: 8,
            bit_len: 2,
        },
        Field::CSwzY => FieldMapping {
            subtoken: 2,
            start_bit: 6,
            bit_len: 2,
        },
        Field::CSwzZ => FieldMapping {
            subtoken: 2,
            start_bit: 4,
            bit_len: 2,
        },
        Field::CSwzW => FieldMapping {
            subtoken: 2,
            start_bit: 2,
            bit_len: 2,
        },
        Field::CRHigh => FieldMapping {
            subtoken: 2,
            start_bit: 0,
            bit_len: 2,
        },
        Field::CRLow => FieldMapping {
            subtoken: 3,
            start_bit: 30,
            bit_len: 2,
        },
        Field::CMux => FieldMapping {
            subtoken: 3,
            start_bit: 28,
            bit_len: 2,
        },
        Field::OutMacMask => FieldMapping {
            subtoken: 3,
            start_bit: 24,
            bit_len: 4,
        },
        Field::OutR => FieldMapping {
            subtoken: 3,
            start_bit: 20,
            bit_len: 4,
        },
        Field::OutIluMask => FieldMapping {
            subtoken: 3,
            start_bit: 16,
            bit_len: 4,
        },
        Field::OutOMask => FieldMapping {
            subtoken: 3,
            start_bit: 12,
            bit_len: 4,
        },
        Field::OutOrb => FieldMapping {
            subtoken: 3,
            start_bit: 11,
            bit_len: 1,
        },
        Field::OutAddress => FieldMapping {
            subtoken: 3,
            start_bit: 3,
            bit_len: 8,
        },
        Field::OutMux => FieldMapping {
            subtoken: 3,
            start_bit: 2,
            bit_len: 1,
        },
        Field::A0X => FieldMapping {
            subtoken: 3,
            start_bit: 1,
            bit_len: 1,
        },
        Field::Final => FieldMapping {
            subtoken: 3,
            start_bit: 0,
            bit_len: 1,
        },
    }
}

fn get_field(token: &[u32], field: Field) -> u32 {
    let f = field_mapping(field);
    let mask = if f.bit_len == 32 {
        u32::MAX
    } else {
        (1u32 << f.bit_len) - 1
    };
    (token[f.subtoken] >> f.start_bit) & mask
}

fn set_field(token: &mut [u32], field: Field, value: u32) {
    let f = field_mapping(field);
    let mask = ((1u32 << f.bit_len) - 1) << f.start_bit;
    token[f.subtoken] = (token[f.subtoken] & !mask) | ((value << f.start_bit) & mask);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParamType {
    Unknown = 0,
    Temp = 1,
    Input = 2,
    Const = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MacOp {
    Nop = 0,
    Mov = 1,
    Mul = 2,
    Add = 3,
    Mad = 4,
    Dp3 = 5,
    Dph = 6,
    Dp4 = 7,
    Min = 8,
    Max = 9,
    Sge = 10,
    Slt = 11,
    Rcp = 12,
    Rsq = 13,
    Arl = 14,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum IluOp {
    Nop = 0,
    Rcc = 1,
    Lit = 2,
    Exp = 3,
    Log = 4,
    Frc = 5,
}

impl IluOp {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "rcc" => Some(Self::Rcc),
            "lit" => Some(Self::Lit),
            "exp" => Some(Self::Exp),
            "log" => Some(Self::Log),
            "frc" => Some(Self::Frc),
            _ => None,
        }
    }

    fn from_u32(v: u32) -> Self {
        match v {
            1 => Self::Rcc,
            2 => Self::Lit,
            3 => Self::Exp,
            4 => Self::Log,
            5 => Self::Frc,
            _ => Self::Nop,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Nop => "nop",
            Self::Rcc => "rcc",
            Self::Lit => "lit",
            Self::Exp => "exp",
            Self::Log => "log",
            Self::Frc => "frc",
        }
    }
}

impl MacOp {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "mov" => Some(Self::Mov),
            "mul" => Some(Self::Mul),
            "add" => Some(Self::Add),
            "mad" => Some(Self::Mad),
            "dp3" => Some(Self::Dp3),
            "dph" => Some(Self::Dph),
            "dp4" => Some(Self::Dp4),
            "min" => Some(Self::Min),
            "max" => Some(Self::Max),
            "sge" => Some(Self::Sge),
            "slt" => Some(Self::Slt),
            "rcp" => Some(Self::Rcp),
            "rsq" => Some(Self::Rsq),
            "arl" => Some(Self::Arl),
            _ => None,
        }
    }

    fn from_u32(v: u32) -> Self {
        match v {
            1 => Self::Mov,
            2 => Self::Mul,
            3 => Self::Add,
            4 => Self::Mad,
            5 => Self::Dp3,
            6 => Self::Dph,
            7 => Self::Dp4,
            8 => Self::Min,
            9 => Self::Max,
            10 => Self::Sge,
            11 => Self::Slt,
            12 => Self::Rcp,
            13 => Self::Rsq,
            14 => Self::Arl,
            _ => Self::Nop,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Nop => "nop",
            Self::Mov => "mov",
            Self::Mul => "mul",
            Self::Add => "add",
            Self::Mad => "mad",
            Self::Dp3 => "dp3",
            Self::Dph => "dph",
            Self::Dp4 => "dp4",
            Self::Min => "min",
            Self::Max => "max",
            Self::Sge => "sge",
            Self::Slt => "slt",
            Self::Rcp => "rcp",
            Self::Rsq => "rsq",
            Self::Arl => "arl",
        }
    }
}

#[derive(Clone, Copy)]
enum DestKind {
    Temp(u32),
    Output(u32),
    A0X,
}

#[derive(Clone, Copy)]
struct Dest {
    kind: DestKind,
    mask: u32,
}

#[derive(Clone, Copy)]
struct Src {
    ty: ParamType,
    reg: i32,
    rel_a0: bool,
    neg: bool,
    swizzle: [u32; 4],
}

fn assemble_source_to_tokens(source: &str, spidey_d_canary: bool) -> Vec<u32> {
    let mut tokens = Vec::new();
    let mut fragment = XgrphSourceFragment::None;
    for raw_line in source.lines() {
        let marker = raw_line.trim().to_ascii_lowercase();
        if let Some(next_fragment) = XgrphSourceFragment::from_marker(&marker) {
            fragment = next_fragment;
            continue;
        }
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let lower = line
            .trim_start_matches('+')
            .trim_start()
            .to_ascii_lowercase();
        if lower.starts_with("xvs.") || lower.starts_with("vs.") {
            continue;
        }
        let lower = normalize_xgrph_fragment_constants(&lower, fragment);
        let lower = normalize_xdk_screenspace_tail_line(&lower);
        if let Some(token) = assemble_line(&lower, spidey_d_canary) {
            tokens.extend_from_slice(&token);
        } else if !is_known_opcode_line(&lower) {
            log_unknown_opcode(&lower);
        }
    }
    tokens
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum XgrphSourceFragment {
    None,
    Transform,
    SkinTransform,
    SphereEnvMap,
    MapScrollUv,
    Color,
    End,
}

impl XgrphSourceFragment {
    fn from_marker(line: &str) -> Option<Self> {
        let name = line.strip_prefix(";----")?.trim();
        Some(match name {
            "vsf_transform" => Self::Transform,
            "vsf_skin_transform" => Self::SkinTransform,
            "vsf_sphere_enviromap" => Self::SphereEnvMap,
            "vsf_map_scrolluv" => Self::MapScrollUv,
            "vsf_color_notint" | "vsf_color_tint" => Self::Color,
            "vsf_end" => Self::End,
            _ => Self::None,
        })
    }
}

fn normalize_xgrph_fragment_constants(line: &str, fragment: XgrphSourceFragment) -> String {
    match fragment {
        XgrphSourceFragment::Transform => remap_contiguous_constants(line, -92),
        XgrphSourceFragment::SkinTransform if line.starts_with("dp4 opos.") => {
            remap_contiguous_constants(line, -92)
        }
        XgrphSourceFragment::SphereEnvMap
            if line.starts_with("dp4 r0.") || line.starts_with("dp3 r_worldspace_normal.") =>
        {
            remap_contiguous_constants(line, -85)
        }
        XgrphSourceFragment::SphereEnvMap if line == "sub r_temp, r0, c[0]" => {
            line.replace("c[0]", "c[-86]")
        }
        XgrphSourceFragment::MapScrollUv => line.replace("c[0]", "c[-81]"),
        XgrphSourceFragment::Color => line.replace("c[0]", "c[-80]"),
        _ => line.to_string(),
    }
}

fn remap_contiguous_constants(line: &str, signed_base: i32) -> String {
    let mut out = line.to_string();
    for i in (0..4).rev() {
        out = out.replace(&format!("c[{i}]"), &format!("c[{}]", signed_base + i));
    }
    out
}

fn normalize_xdk_screenspace_tail_line(line: &str) -> String {
    // XDK xsasm emits the final screen-space fixup as:
    //   mul oPos.xyz, r12, c[-38]
    //   rcc r1.x, r12.w
    //   mad oPos.xyz, r12, r1.x, c[-37]
    // Spider-Man's XGRPH source reaches us with c[0] placeholders in those
    // two tail instructions. Route them to the reserved scale/offset constants
    // that the runtime keeps at corrected host slots c58/c59.
    let compact = line.replace(',', " ");
    let mut parts = compact.split_whitespace();
    match (
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
    ) {
        (Some("mul"), Some("opos.xyz"), Some("r12"), Some("c[0]"), _) => {
            line.replacen("c[0]", "c[-38]", 1)
        }
        (Some("mad"), Some("opos.xyz"), Some("r12"), Some("r1.x"), Some("c[0]")) => {
            line.replacen("c[0]", "c[-37]", 1)
        }
        _ => line.to_string(),
    }
}

fn is_known_opcode_line(line: &str) -> bool {
    let op = line.split_whitespace().next().unwrap_or("");
    MacOp::from_str(op).is_some() || IluOp::from_str(op).is_some() || op == "sub"
}

fn log_unknown_opcode(line: &str) {
    let op = line.split_whitespace().next().unwrap_or("<empty>");
    if op.is_empty()
        || op.starts_with("//")
        || op.starts_with("def")
        || op.starts_with("dcl")
        || op.starts_with("#")
    {
        return;
    }
    crate::xbox::emulator::debug_log(&format!(
        "[NV2A-VSH-UNKNOWN-OPCODE] op={} line='{}'",
        op, line
    ));
}

fn assemble_line(line: &str, spidey_d_canary: bool) -> Option<[u32; TOKEN_DWORDS]> {
    let cleaned = line.replace(',', " ");
    let mut parts = cleaned.split_whitespace();
    let op_name = parts.next()?;
    if let Some(ilu) = IluOp::from_str(op_name) {
        return assemble_ilu_line(ilu, parts);
    }
    if op_name == "sub" {
        return assemble_sub_line(parts, line, spidey_d_canary);
    }
    let op = MacOp::from_str(op_name)?;
    let dest = parse_dest(parts.next()?)?;
    let needed_sources = match op {
        MacOp::Mov | MacOp::Rcp | MacOp::Rsq | MacOp::Arl => 1,
        MacOp::Mul
        | MacOp::Add
        | MacOp::Dp3
        | MacOp::Dph
        | MacOp::Dp4
        | MacOp::Min
        | MacOp::Max
        | MacOp::Sge
        | MacOp::Slt => 2,
        MacOp::Mad => 3,
        MacOp::Nop => 0,
    };
    let mut srcs = Vec::with_capacity(needed_sources);
    for part in parts {
        if part == "+" || srcs.len() == needed_sources {
            break;
        }
        if let Some(src) = parse_src(part) {
            srcs.push(src);
        } else {
            return None;
        }
    }
    let recover_spidey_skin_index = matches!(op, MacOp::Mad)
        && matches!(dest.kind, DestKind::Temp(4))
        && srcs.len() == 1
        && srcs[0].ty == ParamType::Input
        && srcs[0].reg == 5
        && line.starts_with("mad r4, v5");
    let recover_spidey_skin_w = matches!(op, MacOp::Mov)
        && matches!(dest.kind, DestKind::Temp(0))
        && dest.mask == 0x1
        && srcs.is_empty()
        && line.starts_with("mov r0.w");
    let recover_spidey_sphere_env_literal = spidey_d_canary
        && srcs.len() + 1 == needed_sources
        && matches!(op, MacOp::Add | MacOp::Mul | MacOp::Mad)
        && (line == "add r_reflection.z, r_reflection.z,"
            || line == "mul r1.x, r2.x,"
            || line == "mad r2.x, r_reflection.x, r1.x,"
            || line == "mad r2.y, r_reflection.y, r1.x,");
    if srcs.len() < needed_sources {
        if !recover_spidey_skin_index
            && !recover_spidey_skin_w
            && !recover_spidey_sphere_env_literal
        {
            return None;
        }
    }
    let mut token = [0u32; TOKEN_DWORDS];
    set_default_swizzles(&mut token);
    set_field(&mut token, Field::Mac, op as u32);
    encode_dest(&mut token, dest);
    if recover_spidey_skin_index {
        encode_src(&mut token, 'a', srcs[0]);
        return Some(token);
    }
    if recover_spidey_skin_w {
        return Some(token);
    }
    if recover_spidey_sphere_env_literal {
        match op {
            MacOp::Add => {
                encode_src(&mut token, 'a', *srcs.get(0)?);
            }
            MacOp::Mul => {
                encode_src(&mut token, 'a', *srcs.get(0)?);
            }
            MacOp::Mad => {
                encode_src(&mut token, 'a', *srcs.get(0)?);
                encode_src(&mut token, 'b', *srcs.get(1)?);
            }
            _ => {}
        }
        return Some(token);
    }
    match op {
        MacOp::Mov | MacOp::Rcp | MacOp::Rsq | MacOp::Arl => {
            encode_src(&mut token, 'a', *srcs.get(0)?);
        }
        MacOp::Mul
        | MacOp::Dp3
        | MacOp::Dph
        | MacOp::Dp4
        | MacOp::Min
        | MacOp::Max
        | MacOp::Sge
        | MacOp::Slt => {
            encode_src(&mut token, 'a', *srcs.get(0)?);
            encode_src(&mut token, 'b', *srcs.get(1)?);
        }
        MacOp::Add => {
            encode_src(&mut token, 'a', *srcs.get(0)?);
            encode_src(&mut token, 'c', *srcs.get(1)?);
        }
        MacOp::Mad => {
            encode_src(&mut token, 'a', *srcs.get(0)?);
            encode_src(&mut token, 'b', *srcs.get(1)?);
            encode_src(&mut token, 'c', *srcs.get(2)?);
        }
        MacOp::Nop => {}
    }
    Some(token)
}

fn assemble_sub_line<'a>(
    mut parts: impl Iterator<Item = &'a str>,
    line: &str,
    spidey_d_canary: bool,
) -> Option<[u32; TOKEN_DWORDS]> {
    let dest = parse_dest(parts.next()?)?;
    if spidey_d_canary && line == "sub r2.y, , r2.y" {
        let b = parse_src("r2.y")?;
        let mut token = [0u32; TOKEN_DWORDS];
        set_default_swizzles(&mut token);
        set_field(&mut token, Field::Mac, MacOp::Add as u32);
        encode_dest(&mut token, dest);
        let mut b = b;
        b.neg = !b.neg;
        encode_src(&mut token, 'c', b);
        return Some(token);
    }
    let a = parse_src(parts.next()?)?;
    let mut b = parse_src(parts.next()?)?;
    b.neg = !b.neg;
    let mut token = [0u32; TOKEN_DWORDS];
    set_default_swizzles(&mut token);
    set_field(&mut token, Field::Mac, MacOp::Add as u32);
    encode_dest(&mut token, dest);
    encode_src(&mut token, 'a', a);
    encode_src(&mut token, 'c', b);
    Some(token)
}

fn assemble_ilu_line<'a>(
    op: IluOp,
    mut parts: impl Iterator<Item = &'a str>,
) -> Option<[u32; TOKEN_DWORDS]> {
    let dest = parse_dest(parts.next()?)?;
    let src = parts.find_map(parse_src)?;
    let mut token = [0u32; TOKEN_DWORDS];
    set_default_swizzles(&mut token);
    set_field(&mut token, Field::Ilu, op as u32);
    encode_ilu_dest(&mut token, dest);
    encode_src(&mut token, 'a', src);
    Some(token)
}

fn set_default_swizzles(token: &mut [u32; TOKEN_DWORDS]) {
    for (x, y, z, w) in [
        (Field::ASwzX, Field::ASwzY, Field::ASwzZ, Field::ASwzW),
        (Field::BSwzX, Field::BSwzY, Field::BSwzZ, Field::BSwzW),
        (Field::CSwzX, Field::CSwzY, Field::CSwzZ, Field::CSwzW),
    ] {
        set_field(token, x, 0);
        set_field(token, y, 1);
        set_field(token, z, 2);
        set_field(token, w, 3);
    }
}

fn parse_dest(s: &str) -> Option<Dest> {
    let (base, suffix) = split_suffix(s);
    let mask = suffix.map(mask_from_suffix).unwrap_or(0xF);
    let kind = if base == "opos" {
        DestKind::Output(0)
    } else if base == "od0" {
        DestKind::Output(3)
    } else if base == "od1" {
        DestKind::Output(4)
    } else if base == "ofog" {
        DestKind::Output(5)
    } else if base == "opts" {
        DestKind::Output(6)
    } else if let Some(n) = base.strip_prefix("ot").and_then(|n| n.parse::<u32>().ok()) {
        DestKind::Output(9 + n)
    } else if let Some(n) = base.strip_prefix('r').and_then(|n| n.parse::<u32>().ok()) {
        DestKind::Temp(n)
    } else if let Some(n) = symbolic_temp_register(base) {
        DestKind::Temp(n)
    } else if base == "a0" {
        DestKind::A0X
    } else {
        return None;
    };
    Some(Dest { kind, mask })
}

fn parse_src(s: &str) -> Option<Src> {
    let (neg, body) = if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else {
        (false, s)
    };
    let (base, suffix) = split_suffix(body);
    let swizzle = suffix.map(swizzle_from_suffix).unwrap_or([0, 1, 2, 3]);
    let (ty, reg, rel_a0) =
        if let Some(n) = base.strip_prefix('r').and_then(|n| n.parse::<i32>().ok()) {
            (ParamType::Temp, n, false)
        } else if let Some(n) = symbolic_temp_register(base) {
            (ParamType::Temp, n as i32, false)
        } else if let Some(n) = base.strip_prefix('v').and_then(|n| n.parse::<i32>().ok()) {
            (ParamType::Input, n, false)
        } else if base.starts_with("c[") && base.ends_with(']') {
            let inner = base[2..base.len() - 1].replace(' ', "");
            if let Some(offset) = parse_a0_const_offset(&inner) {
                (ParamType::Const, offset, true)
            } else {
                let n = inner.parse::<i32>().ok()?;
                let raw = if n < 0 {
                    encode_signed_c_register(n)? as i32
                } else {
                    n
                };
                (ParamType::Const, raw, false)
            }
        } else {
            return None;
        };
    Some(Src {
        ty,
        reg,
        rel_a0,
        neg,
        swizzle,
    })
}

fn symbolic_temp_register(name: &str) -> Option<u32> {
    match name {
        "r_worldspace_normal" => Some(5),
        "r_temp" => Some(6),
        "r_eye_vector" => Some(7),
        "r_dot2" => Some(8),
        "r_reflection" => Some(9),
        _ => None,
    }
}

fn parse_a0_const_offset(inner: &str) -> Option<i32> {
    let rest = inner.strip_prefix("a0.x")?;
    if rest.is_empty() {
        return Some(0);
    }
    rest.parse::<i32>().ok()
}

fn encode_signed_c_register(signed_reg: i32) -> Option<u32> {
    if !(-96..=159).contains(&signed_reg) {
        return None;
    }
    let group = signed_reg.div_euclid(32);
    let low = signed_reg.rem_euclid(32);
    let high = group + 3;
    if !(0..=7).contains(&high) {
        return None;
    }
    Some(((high as u32) << 5) | low as u32)
}

fn split_suffix(s: &str) -> (&str, Option<&str>) {
    if let Some(close) = s.rfind(']') {
        if let Some(dot) = s[close + 1..].find('.') {
            let dot = close + 1 + dot;
            return (&s[..dot], Some(&s[dot + 1..]));
        }
        return (s, None);
    }
    if let Some((base, suffix)) = s.rsplit_once('.') {
        (base, Some(suffix))
    } else {
        (s, None)
    }
}

fn mask_from_suffix(s: &str) -> u32 {
    let mut mask = 0;
    if s.contains('x') {
        mask |= 0x8;
    }
    if s.contains('y') {
        mask |= 0x4;
    }
    if s.contains('z') {
        mask |= 0x2;
    }
    if s.contains('w') {
        mask |= 0x1;
    }
    if mask == 0 {
        0xF
    } else {
        mask
    }
}

fn swizzle_from_suffix(s: &str) -> [u32; 4] {
    let mut out = [0, 1, 2, 3];
    for (i, ch) in s.chars().take(4).enumerate() {
        out[i] = match ch {
            'x' | 'r' => 0,
            'y' | 'g' => 1,
            'z' | 'b' => 2,
            'w' | 'a' => 3,
            _ => out[i],
        };
    }
    if s.len() == 1 {
        out = [out[0]; 4];
    }
    out
}

fn encode_dest(token: &mut [u32; TOKEN_DWORDS], dest: Dest) {
    match dest.kind {
        DestKind::Temp(reg) => {
            set_field(token, Field::OutR, reg & 0xF);
            set_field(token, Field::OutMacMask, dest.mask);
        }
        DestKind::Output(reg) => {
            set_field(token, Field::OutAddress, reg & 0xFF);
            set_field(token, Field::OutOMask, dest.mask);
            set_field(token, Field::OutOrb, 1);
        }
        DestKind::A0X => {
            set_field(token, Field::A0X, 1);
            set_field(token, Field::OutMacMask, dest.mask);
        }
    }
}

fn encode_ilu_dest(token: &mut [u32; TOKEN_DWORDS], dest: Dest) {
    match dest.kind {
        DestKind::Temp(reg) => {
            set_field(token, Field::OutR, reg & 0xF);
            set_field(token, Field::OutIluMask, dest.mask);
        }
        DestKind::Output(reg) => {
            set_field(token, Field::OutAddress, reg & 0xFF);
            set_field(token, Field::OutOMask, dest.mask);
            set_field(token, Field::OutOrb, 1);
        }
        DestKind::A0X => {
            set_field(token, Field::A0X, 1);
            set_field(token, Field::OutIluMask, dest.mask);
        }
    }
}

fn encode_src(token: &mut [u32; TOKEN_DWORDS], slot: char, src: Src) {
    match slot {
        'a' => {
            set_field(token, Field::ANeg, src.neg as u32);
            set_field(token, Field::ASwzX, src.swizzle[0]);
            set_field(token, Field::ASwzY, src.swizzle[1]);
            set_field(token, Field::ASwzZ, src.swizzle[2]);
            set_field(token, Field::ASwzW, src.swizzle[3]);
            set_field(token, Field::AMux, src.ty as u32);
            if src.ty == ParamType::Temp {
                set_field(token, Field::AR, (src.reg as u32) & 0xF);
            }
        }
        'b' => {
            set_field(token, Field::BNeg, src.neg as u32);
            set_field(token, Field::BSwzX, src.swizzle[0]);
            set_field(token, Field::BSwzY, src.swizzle[1]);
            set_field(token, Field::BSwzZ, src.swizzle[2]);
            set_field(token, Field::BSwzW, src.swizzle[3]);
            set_field(token, Field::BMux, src.ty as u32);
            if src.ty == ParamType::Temp {
                set_field(token, Field::BR, (src.reg as u32) & 0xF);
            }
        }
        'c' => {
            set_field(token, Field::CNeg, src.neg as u32);
            set_field(token, Field::CSwzX, src.swizzle[0]);
            set_field(token, Field::CSwzY, src.swizzle[1]);
            set_field(token, Field::CSwzZ, src.swizzle[2]);
            set_field(token, Field::CSwzW, src.swizzle[3]);
            set_field(token, Field::CMux, src.ty as u32);
            if src.ty == ParamType::Temp {
                set_field(token, Field::CRLow, (src.reg as u32) & 0x3);
                set_field(token, Field::CRHigh, ((src.reg as u32) >> 2) & 0x3);
            }
        }
        _ => {}
    }
    if src.ty == ParamType::Input {
        set_field(token, Field::V, (src.reg as u32) & 0xF);
    } else if src.ty == ParamType::Const {
        set_field(token, Field::Const, (src.reg as u32) & 0xFF);
        if src.rel_a0 {
            // Reuse the otherwise idle output mux bit in our source-assembled
            // token stream to mark c[a0.x+N] addressing without colliding with
            // A0 writes.
            set_field(token, Field::OutMux, 1);
        }
    }
}

fn fallback_transform_tokens() -> Vec<u32> {
    let mut out = Vec::with_capacity(4 * TOKEN_DWORDS);
    for component in ["x", "y", "z", "w"] {
        let line = format!("dp4 opos.{component}, v0, c[0]");
        let mut token = assemble_line(&line, false).unwrap_or([0; TOKEN_DWORDS]);
        set_field(
            &mut token,
            Field::Const,
            match component {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                _ => 3,
            },
        );
        out.extend_from_slice(&token);
    }
    out
}

fn convert_c_register(raw: u32) -> i32 {
    let r = ((((raw >> 5) & 7) as i32 - 3) * 32) + (raw & 31) as i32;
    r + 96
}

fn sign_extend_8(v: u32) -> i32 {
    (v as u8 as i8) as i32
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

fn spidey_xgrph_c_hle_enabled() -> bool {
    spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_C_HLE")
        || spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_HLE_CXBXR")
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

fn spidey_skin_a0_remap_enabled() -> bool {
    spidey_env_flag("RUSTEMU_SPIDEY_SKIN_A0_REMAP")
        || spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_C_HLE_A0_REMAP")
}

fn spidey_skin_raw4_canary_enabled() -> bool {
    spidey_env_flag("RUSTEMU_SPIDEY_XGRPH_SKIN_RAW4_CANARY")
        || spidey_env_flag("RUSTEMU_SPIDEY_SKIN_RAW4_CANARY")
}

fn spidey_pbyte4_normalize_enabled() -> bool {
    !spidey_skin_raw4_canary_enabled()
        && !spidey_env_flag("RUSTEMU_SPIDEY_PBYTE4_RAW")
        && (spidey_xgrph_c_hle_enabled()
            || spidey_env_bool("RUSTEMU_SPIDEY_PBYTE4_NORMALIZE", true))
}

fn src_expr(token: &[u32], slot: char) -> String {
    let (mux, neg, reg, swz) = match slot {
        'a' => (
            get_field(token, Field::AMux),
            get_field(token, Field::ANeg),
            get_field(token, Field::AR),
            [
                get_field(token, Field::ASwzX),
                get_field(token, Field::ASwzY),
                get_field(token, Field::ASwzZ),
                get_field(token, Field::ASwzW),
            ],
        ),
        'b' => (
            get_field(token, Field::BMux),
            get_field(token, Field::BNeg),
            get_field(token, Field::BR),
            [
                get_field(token, Field::BSwzX),
                get_field(token, Field::BSwzY),
                get_field(token, Field::BSwzZ),
                get_field(token, Field::BSwzW),
            ],
        ),
        'c' => {
            let low = get_field(token, Field::CRLow);
            let high = get_field(token, Field::CRHigh);
            (
                get_field(token, Field::CMux),
                get_field(token, Field::CNeg),
                low | (high << 2),
                [
                    get_field(token, Field::CSwzX),
                    get_field(token, Field::CSwzY),
                    get_field(token, Field::CSwzZ),
                    get_field(token, Field::CSwzW),
                ],
            )
        }
        _ => (0, 0, 0, [0, 1, 2, 3]),
    };
    let base = match mux {
        1 => temp_register_name(reg).to_string(),
        2 => format!("v{}", get_field(token, Field::V)),
        3 => {
            let raw_const = get_field(token, Field::Const);
            if get_field(token, Field::OutMux) != 0 {
                let helper = if spidey_skin_a0_remap_enabled() {
                    "nv2a_relative_const_index"
                } else {
                    "nv2a_const_index"
                };
                format!("c[{}(A0.x, {})]", helper, sign_extend_8(raw_const))
            } else {
                let reg_idx = convert_c_register(raw_const);
                let final_reg = if spidey_env_flag("RUSTEMU_SPIDEY_ALLOW_RESERVED_BONE_ROWS") {
                    if reg_idx == 58 {
                        212
                    } else if reg_idx == 59 {
                        213
                    } else {
                        reg_idx
                    }
                } else {
                    reg_idx
                };
                format!("c[{}]", final_reg.clamp(0, 218))
            }
        }
        _ => "float4(0.0, 0.0, 0.0, 0.0)".to_string(),
    };
    let mut suffix = String::with_capacity(5);
    suffix.push('.');
    for component in swz {
        suffix.push(match component {
            0 => 'x',
            1 => 'y',
            2 => 'z',
            _ => 'w',
        });
    }
    let expr = if suffix == ".xyzw" {
        base
    } else {
        format!("{base}{suffix}")
    };
    if neg != 0 {
        format!("(-{expr})")
    } else {
        expr
    }
}

fn temp_register_name(reg: u32) -> &'static str {
    // NV2A aliases temporary register r12 to the current oPos value. Several
    // Xbox shaders read it back after writing oPos to do screen-space fixups.
    if reg == 12 {
        "oPos"
    } else {
        match reg {
            0 => "R0",
            1 => "R1",
            2 => "R2",
            3 => "R3",
            4 => "R4",
            5 => "R5",
            6 => "R6",
            7 => "R7",
            8 => "R8",
            9 => "R9",
            10 => "R10",
            11 => "R11",
            _ => "R12",
        }
    }
}

fn dest_expr(token: &[u32]) -> Option<(String, u32)> {
    let omask = get_field(token, Field::OutOMask);
    if omask != 0 {
        return Some((
            output_name(get_field(token, Field::OutAddress)).to_string(),
            omask,
        ));
    }
    let rmask = get_field(token, Field::OutMacMask);
    if rmask != 0 {
        if get_field(token, Field::A0X) != 0 {
            return Some(("A0".to_string(), rmask));
        }
        return Some((
            temp_register_name(get_field(token, Field::OutR)).to_string(),
            rmask,
        ));
    }
    let imask = get_field(token, Field::OutIluMask);
    if imask != 0 {
        if get_field(token, Field::A0X) != 0 {
            return Some(("A0".to_string(), imask));
        }
        return Some((
            temp_register_name(get_field(token, Field::OutR)).to_string(),
            imask,
        ));
    }
    None
}

fn output_name(reg: u32) -> &'static str {
    match reg {
        0 => "oPos",
        3 => "oD0",
        4 => "oD1",
        5 => "oFog",
        6 => "oPts",
        7 => "oB0",
        8 => "oB1",
        9 => "oT0",
        10 => "oT1",
        11 => "oT2",
        12 => "oT3",
        _ => "oUnknown",
    }
}

fn mask_suffix(mask: u32) -> &'static str {
    match mask & 0xF {
        0x8 => "x",
        0x4 => "y",
        0x2 => "z",
        0x1 => "w",
        0xC => "xy",
        0xA => "xz",
        0x9 => "xw",
        0x6 => "yz",
        0x5 => "yw",
        0x3 => "zw",
        0xE => "xyz",
        0xD => "xyw",
        0xB => "xzw",
        0x7 => "yzw",
        _ => "xyzw",
    }
}

fn write_masked(line: &mut String, dst: &str, mask: u32, expr: &str) {
    let suffix = mask_suffix(mask);
    if dst == "A0" {
        if suffix == "xyzw" {
            line.push_str(&format!("    {dst} = floor({expr} + 0.001);\n"));
        } else {
            line.push_str(&format!(
                "    {dst}.{suffix} = floor(({expr}).{suffix} + 0.001);\n"
            ));
        }
        return;
    }
    if suffix == "xyzw" {
        line.push_str(&format!("    {dst} = {expr};\n"));
    } else {
        line.push_str(&format!("    {dst}.{suffix} = ({expr}).{suffix};\n"));
    }
}

fn hlsl_expr_for_token(token: &[u32]) -> Option<String> {
    let op = MacOp::from_u32(get_field(token, Field::Mac));
    if matches!(op, MacOp::Nop) {
        let ilu = IluOp::from_u32(get_field(token, Field::Ilu));
        if !matches!(ilu, IluOp::Nop) {
            let a = src_expr(token, 'a');
            return match ilu {
                IluOp::Rcc => Some(format!("nv2a_rcc({a})")),
                IluOp::Lit => Some(format!("nv2a_lit({a})")),
                IluOp::Exp => Some(format!("nv2a_exp({a})")),
                IluOp::Log => Some(format!("nv2a_log({a})")),
                IluOp::Frc => Some(format!("frac({a})")),
                IluOp::Nop => None,
            };
        }
    }
    let out_r = get_field(token, Field::OutR);
    let out_mask = get_field(token, Field::OutMacMask);
    if matches!(op, MacOp::Add)
        && out_r == 0
        && out_mask == 0x4
        && get_field(token, Field::AMux) == ParamType::Temp as u32
        && get_field(token, Field::AR) == 0
        && get_field(token, Field::CMux) == ParamType::Unknown as u32
    {
        return Some("(R0 + float4(0.0, 0.0, 1.0, 0.0))".to_string());
    }
    if matches!(op, MacOp::Mul)
        && out_r == 1
        && out_mask == 0x1
        && get_field(token, Field::AMux) == ParamType::Temp as u32
        && get_field(token, Field::AR) == 2
        && get_field(token, Field::BMux) == ParamType::Unknown as u32
    {
        return Some("(R2.xxxx * float4(0.5, 0.5, 0.5, 0.5))".to_string());
    }
    if matches!(op, MacOp::Mad)
        && out_r == 2
        && (out_mask == 0x1 || out_mask == 0x2)
        && get_field(token, Field::AMux) == ParamType::Temp as u32
        && get_field(token, Field::AR) == 0
        && get_field(token, Field::BMux) == ParamType::Temp as u32
        && get_field(token, Field::BR) == 1
        && get_field(token, Field::CMux) == ParamType::Unknown as u32
    {
        let component = if out_mask == 0x2 { "yyyy" } else { "xxxx" };
        return Some(format!(
            "((R0.{component} * R1.xxxx) + float4(0.5, 0.5, 0.5, 0.5))"
        ));
    }
    if matches!(op, MacOp::Add)
        && out_r == 2
        && out_mask == 0x2
        && get_field(token, Field::AMux) == ParamType::Unknown as u32
        && get_field(token, Field::CMux) == ParamType::Temp as u32
        && get_field(token, Field::CRLow) == 2
        && get_field(token, Field::CNeg) != 0
    {
        return Some("(float4(1.0, 1.0, 1.0, 1.0) - R2.yyyy)".to_string());
    }
    if matches!(op, MacOp::Mad)
        && out_r == 4
        && out_mask == 0xF
        && get_field(token, Field::AMux) == ParamType::Input as u32
        && get_field(token, Field::V) == 5
        && get_field(token, Field::BMux) == ParamType::Unknown as u32
        && get_field(token, Field::CMux) == ParamType::Unknown as u32
    {
        let expr = if spidey_pbyte4_normalize_enabled() {
            // Cxbx-R's XGRPH translation fills Spider-Man's stripped
            // operands as c(-95).x and c(-95).y. Rustemu stores signed Xbox
            // constants at host index +96, so c(-95) is c[1].
            "((v5 * c[1].xxxx) + c[1].yyyy)"
        } else {
            "((v5 * 4.0) - 92.0)"
        };
        return Some(expr.to_string());
    }
    if matches!(op, MacOp::Mov)
        && out_r == 0
        && out_mask == 0x1
        && get_field(token, Field::AMux) == ParamType::Unknown as u32
    {
        return Some("float4(1.0, 1.0, 1.0, 1.0)".to_string());
    }
    let a = src_expr(token, 'a');
    let b = src_expr(token, 'b');
    let c = src_expr(token, 'c');
    match op {
        MacOp::Mov => Some(a),
        MacOp::Mul => Some(format!("({a} * {b})")),
        MacOp::Add => Some(format!("({a} + {c})")),
        MacOp::Mad => Some(format!("(({a} * {b}) + {c})")),
        MacOp::Dp3 => Some(format!("dot({a}.xyz, {b}.xyz).xxxx")),
        MacOp::Dph => Some(format!("dot(float4({a}.xyz, 1.0), {b}).xxxx")),
        MacOp::Dp4 => Some(format!("dot({a}, {b}).xxxx")),
        MacOp::Min => Some(format!("min({a}, {b})")),
        MacOp::Max => Some(format!("max({a}, {b})")),
        MacOp::Sge => Some(format!("nv2a_sge({a}, {b})")),
        MacOp::Slt => Some(format!("nv2a_slt({a}, {b})")),
        MacOp::Rcp => Some(format!("nv2a_rcp({a})")),
        MacOp::Rsq => Some(format!("nv2a_rsq({a})")),
        MacOp::Arl => Some(a),
        MacOp::Nop => None,
    }
}

fn decode_lines(tokens: &[u32]) -> Vec<String> {
    tokens
        .chunks_exact(TOKEN_DWORDS)
        .filter_map(|token| {
            let (dst, mask) = dest_expr(token)?;
            let op = MacOp::from_u32(get_field(token, Field::Mac));
            if matches!(op, MacOp::Nop) {
                let ilu = IluOp::from_u32(get_field(token, Field::Ilu));
                Some(format!("{} {}.{}", ilu.label(), dst, mask_suffix(mask)))
            } else {
                Some(format!("{} {}.{}", op.label(), dst, mask_suffix(mask)))
            }
        })
        .collect()
}

pub fn translate_tokens_to_hlsl(tokens: &[u32], screenspace: bool) -> String {
    let mut hlsl = String::new();
    hlsl.push_str(
        "cbuffer Nv2aVshConstants : register(b0) {\n\
         \tfloat4 c[219];\n\
         \tfloat4 surfaceSize;\n\
         \tfloat4 clipRange;\n\
         };\n\
         struct VS_IN {\n\
         \tfloat4 v0 : TEXCOORD0;\n\
         \tfloat4 v1 : TEXCOORD1;\n\
         \tfloat4 v2 : TEXCOORD2;\n\
         \tfloat4 v3 : TEXCOORD3;\n\
         \tfloat4 v4 : TEXCOORD4;\n\
         \tfloat4 v5 : TEXCOORD5;\n\
         \tfloat4 v6 : TEXCOORD6;\n\
         \tfloat4 v7 : TEXCOORD7;\n\
         \tfloat4 v8 : TEXCOORD8;\n\
         \tfloat4 v9 : TEXCOORD9;\n\
         \tfloat4 v10 : TEXCOORD10;\n\
         \tfloat4 v11 : TEXCOORD11;\n\
         \tfloat4 v12 : TEXCOORD12;\n\
         \tfloat4 v13 : TEXCOORD13;\n\
         \tfloat4 v14 : TEXCOORD14;\n\
         \tfloat4 v15 : TEXCOORD15;\n\
         };\n\
         struct VS_OUT {\n\
         \tfloat4 pos : SV_Position;\n\
         \tfloat4 color : COLOR;\n\
         \tfloat2 uv : TEXCOORD;\n\
         \tfloat4 normal_passthrough : TEXCOORD1;\n\
         \tfloat4 uv1_passthrough : TEXCOORD2;\n\
         };\n\
         int nv2a_const_index(float a0, int offset) {\n\
         \t// Cxbx-R maps Xbox signed constant registers [-96, 95] to host [0, 191].\n\
         \tint i = (int)floor(a0 + 0.001) + offset + 96;\n\
         \treturn (i >= 0 && i < 192) ? i : 218;\n\
         }\n\
         int nv2a_relative_const_index(float a0, int offset) {\n\
         \tint raw = (int)floor(a0 + 0.001);\n\
         \tint skinNumerator = raw + 92;\n\
         \tif (skinNumerator >= 0 && (skinNumerator % 4) == 0) {\n\
         \t\tint bone = skinNumerator / 4;\n\
         \t\tif (bone >= 0 && bone < 48 && offset >= 0 && offset <= 2) {\n\
         \t\t\treturn clamp(17 + bone * 3 + offset, 0, 191);\n\
         \t\t}\n\
         \t}\n\
         \treturn nv2a_const_index(a0, offset);\n\
         }\n\
         float4 nv2a_sge(float4 a, float4 b) { return float4(a.x >= b.x ? 1.0 : 0.0, a.y >= b.y ? 1.0 : 0.0, a.z >= b.z ? 1.0 : 0.0, a.w >= b.w ? 1.0 : 0.0); }\n\
         float4 nv2a_slt(float4 a, float4 b) { return float4(a.x < b.x ? 1.0 : 0.0, a.y < b.y ? 1.0 : 0.0, a.z < b.z ? 1.0 : 0.0, a.w < b.w ? 1.0 : 0.0); }\n\
         float4 nv2a_rcp(float4 a) { float s = 1.0 / max(abs(a.x), 1.0e-20); return float4(s, s, s, s); }\n\
         float4 nv2a_rsq(float4 a) { float s = rsqrt(max(abs(a.x), 1.0e-20)); return float4(s, s, s, s); }\n\
         float4 nv2a_rcc(float4 a) { float s = sign(a.x) / max(abs(a.x), 1.0e-20); return float4(s, s, s, s); }\n\
         float4 nv2a_exp(float4 a) { float s = exp2(a.x); return float4(s, s, s, s); }\n\
         float4 nv2a_log(float4 a) { float s = log2(max(abs(a.x), 1.0e-20)); return float4(s, s, s, s); }\n\
         float4 nv2a_lit(float4 a) { float4 r = float4(1.0, max(a.x, 0.0), 0.0, 1.0); if (a.x > 0.0 && a.y > 0.0) { r.z = exp2(clamp(a.w, -128.0, 128.0) * log2(max(a.y, 1.0e-20))); } return r; }\n\
         VS_OUT main(VS_IN input) {\n\
         \tVS_OUT outp;\n\
         \tfloat4 v0 = input.v0;\n\
         \tfloat4 v1 = input.v1;\n\
         \tfloat4 v2 = input.v2;\n\
         \tfloat4 v3 = input.v3;\n",
    );
    hlsl.push_str(
        "\tfloat4 v4 = input.v4;\n\
         \tfloat4 v5 = input.v5;\n\
         \tfloat4 v6 = input.v6;\n",
    );
    for i in 7..16 {
        hlsl.push_str(&format!("\tfloat4 v{i} = input.v{i};\n"));
    }
    for i in 0..13 {
        hlsl.push_str(&format!("\tfloat4 R{i} = float4(0.0, 0.0, 0.0, 0.0);\n"));
    }
    hlsl.push_str("\tfloat4 A0 = float4(0.0, 0.0, 0.0, 0.0);\n");
    hlsl.push_str(
        "\tfloat4 oPos = v0;\n\
         \tfloat4 oD0 = v1;\n\
         \tfloat4 oD1 = float4(1.0, 1.0, 1.0, 1.0);\n\
         \tfloat4 oFog = float4(0.0, 0.0, 0.0, 1.0);\n\
         \tfloat4 oPts = float4(1.0, 1.0, 1.0, 1.0);\n\
         \tfloat4 oB0 = float4(0.0, 0.0, 0.0, 1.0);\n\
         \tfloat4 oB1 = float4(0.0, 0.0, 0.0, 1.0);\n\
         \tfloat4 oT0 = v3;\n\
         \tfloat4 oT1 = v4;\n\
         \tfloat4 oT2 = float4(0.0, 0.0, 0.0, 1.0);\n\
         \tfloat4 oT3 = float4(0.0, 0.0, 0.0, 1.0);\n",
    );
    if screenspace {
        hlsl.push_str("\tfloat4 rawPos = oPos;\n");
    }
    for token in tokens.chunks_exact(TOKEN_DWORDS) {
        if let (Some((dst, mask)), Some(expr)) = (dest_expr(token), hlsl_expr_for_token(token)) {
            if dst != "oUnknown" {
                write_masked(&mut hlsl, &dst, mask, &expr);
            }
        }
        if get_field(token, Field::Final) != 0 {
            break;
        }
    }
    if screenspace {
        if crate::xbox::emulator::spidey_synth_training_enabled()
            || spidey_xgrph_c_hle_enabled()
            || spidey_env_flag("RUSTEMU_CXBXR_SCREENSPACE_SHIM")
        {
            let z_remap = crate::xbox::emulator::spidey_synth_training_enabled()
                || spidey_xgrph_c_hle_enabled()
                || spidey_env_bool("RUSTEMU_CXBXR_SCREENSPACE_Z_REMAP", true);
            let y_flip = spidey_env_flag("RUSTEMU_CXBXR_SCREENSPACE_Y_FLIP");
            let z_line = if z_remap {
                "\tfinalPos.z = (finalPos.z + finalPos.w) * 0.5;\n"
            } else {
                ""
            };
            let y_line = if y_flip {
                "\tfinalPos.y = -finalPos.y;\n"
            } else {
                ""
            };
            static CXBXR_SCREENSPACE_HLSL_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = CXBXR_SCREENSPACE_HLSL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[NV2A-VSH-CXBXR-SCREENSPACE] #{} z_remap={} y_flip={} scale_reg=212 offset_reg=213 tokens={}",
                    n,
                    z_remap as u32,
                    y_flip as u32,
                    tokens.len() / TOKEN_DWORDS
                ));
            }
            hlsl.push_str(&format!(
                "\tfloat4 finalPos = oPos;\n\
                 \t// Derived from Cxbx-Reloaded's reverseScreenspaceTransform helper.\n\
                 \t// Rustemu stores these host-owned transform constants at c212/c213,\n\
                 \t// leaving Xbox guest constants c0..c191 untouched.\n\
                 \tfloat4 screenScale = c[212];\n\
                 \tscreenScale.x = abs(screenScale.x) < 1.0e-6 ? 1.0 : screenScale.x;\n\
                 \tscreenScale.y = abs(screenScale.y) < 1.0e-6 ? -1.0 : screenScale.y;\n\
                 \tscreenScale.z = abs(screenScale.z) < 1.0e-6 ? 1.0 : screenScale.z;\n\
                 \tscreenScale.w = abs(screenScale.w) < 1.0e-6 ? 1.0 : screenScale.w;\n\
                 \tfinalPos -= c[213];\n\
                 \tfinalPos /= screenScale;\n\
                 \tif (abs(finalPos.w) < 1.0e-6) finalPos.w = 1.0;\n\
                 \tfinalPos.xyz *= finalPos.w;\n\
                 {y_line}\
                 {z_line}\
                 \toutp.pos = finalPos;\n"
            ));
        } else {
            hlsl.push_str(
                "\tbool forceRawPixel = c[191].x > 0.5;\n\
                 \tbool zeroTransform = all(abs(c[4]) < 1.0e-7) && all(abs(c[5]) < 1.0e-7) && all(abs(c[6]) < 1.0e-7) && all(abs(c[7]) < 1.0e-7);\n\
                 \tfloat4 finalPos = oPos;\n\
                 \tif (forceRawPixel) {\n\
                 \t\tfinalPos = rawPos;\n\
                 \t} else if (zeroTransform) {\n\
                 \t\tfinalPos = float4((rawPos.xyz * c[58].xyz) + c[59].xyz, rawPos.w);\n\
                 \t}\n\
                 \tfloat2 viewport_scale = max(abs(c[58].xy), float2(1.0, 1.0));\n\
                 \tfloat2 viewport_center = c[59].xy;\n\
                 \tfloat2 ndc = float2((finalPos.x - viewport_center.x) / viewport_scale.x, -(finalPos.y - viewport_center.y) / viewport_scale.y);\n\
                 \tfloat z = saturate((finalPos.z - clipRange.x) / max(clipRange.y - clipRange.x, 1.0e-6));\n\
                 \toutp.pos = float4(ndc, z, 1.0);\n",
            );
        }
    } else {
        hlsl.push_str("\toutp.pos = oPos;\n");
    }
    hlsl.push_str(
        "\toutp.color = oD0;\n\
         \toutp.uv = oT0.xy;\n\
         \toutp.normal_passthrough = v2;\n\
         \toutp.uv1_passthrough = v4;\n\
         \treturn outp;\n\
         }\n",
    );
    hlsl
}
