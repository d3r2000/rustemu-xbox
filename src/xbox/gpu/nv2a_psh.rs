//! NV2A register-combiner to HLSL pixel shader path.
//!
//! This is a deliberately small, debuggable translator. It covers the common
//! Xbox D3D8 register-combiner fields and degrades unsupported routing to the
//! current `r0` value instead of refusing multi-stage shaders.

#[derive(Clone, Copy, Debug)]
pub struct PixelShaderState {
    pub final_abcd: u32,
    pub final_efg: u32,
    pub combiner_count: u32,
    pub texture_modes: u32,
    pub rgb_inputs: [u32; 8],
    pub alpha_inputs: [u32; 8],
    pub rgb_outputs: [u32; 8],
    pub alpha_outputs: [u32; 8],
    pub c0_mapping: u32,
    pub c1_mapping: u32,
    pub const0: [u32; 8],
    pub const1: [u32; 8],
    pub final_constant0: u32,
    pub final_constant1: u32,
    pub compare_mode: u32,
    pub dot_mapping: u32,
    pub input_texture: u32,
}

#[derive(Clone, Debug)]
pub struct PixelShaderProgram {
    pub key: u64,
    pub hlsl: String,
}

#[derive(Clone, Copy)]
struct InputInfo {
    reg: u32,
    chan: u32,
    mapping: u32,
}

#[derive(Clone, Copy)]
struct OutputInfo {
    cd: u32,
    ab: u32,
    muxsum: u32,
    flags: u32,
    cd_dot: bool,
    ab_dot: bool,
    mux: bool,
    mapping: u32,
}

impl PixelShaderState {
    pub fn from_render_states(states: &[u32; 137]) -> Self {
        let mut rgb_inputs = [0u32; 8];
        let mut alpha_inputs = [0u32; 8];
        let mut rgb_outputs = [0u32; 8];
        let mut alpha_outputs = [0u32; 8];
        let mut const0 = [0u32; 8];
        let mut const1 = [0u32; 8];

        rgb_inputs.copy_from_slice(&states[34..42]);
        alpha_inputs.copy_from_slice(&states[0..8]);
        rgb_outputs.copy_from_slice(&states[45..53]);
        alpha_outputs.copy_from_slice(&states[26..34]);
        const0.copy_from_slice(&states[10..18]);
        const1.copy_from_slice(&states[18..26]);

        Self {
            final_abcd: states[8],
            final_efg: states[9],
            combiner_count: states[53],
            texture_modes: states[136],
            rgb_inputs,
            alpha_inputs,
            rgb_outputs,
            alpha_outputs,
            c0_mapping: states[57],
            c1_mapping: states[58],
            const0,
            const1,
            final_constant0: states[43],
            final_constant1: states[44],
            compare_mode: states[42],
            dot_mapping: states[55],
            input_texture: states[56],
        }
    }

    fn key(self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for value in self.key_words() {
            hash ^= value as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    fn key_words(self) -> impl Iterator<Item = u32> {
        [
            self.final_abcd,
            self.final_efg,
            self.combiner_count,
            self.texture_modes,
            self.c0_mapping,
            self.c1_mapping,
            self.final_constant0,
            self.final_constant1,
            self.compare_mode,
            self.dot_mapping,
            self.input_texture,
        ]
        .into_iter()
        .chain(self.rgb_inputs)
        .chain(self.alpha_inputs)
        .chain(self.rgb_outputs)
        .chain(self.alpha_outputs)
        .chain(self.const0)
        .chain(self.const1)
    }

    fn stage_count(self) -> usize {
        (self.combiner_count & 0xFF).min(8) as usize
    }

    fn flags(self) -> u32 {
        self.combiner_count >> 8
    }
}

pub fn translate_basic_modulate(state: PixelShaderState) -> Option<PixelShaderProgram> {
    if state.key_words().all(|value| value == 0) {
        return None;
    }

    Some(PixelShaderProgram {
        key: state.key(),
        hlsl: pixel_shader_hlsl(state),
    })
}

const PS_TEXTUREMODES_NONE: u32 = 0x00;
const PS_TEXTUREMODES_PROJECT2D: u32 = 0x01;
const PS_TEXTUREMODES_PROJECT3D: u32 = 0x02;
const PS_TEXTUREMODES_CUBEMAP: u32 = 0x03;
const PS_TEXTUREMODES_PASSTHRU: u32 = 0x04;
const PS_TEXTUREMODES_DOTPRODUCT: u32 = 0x11;

const PS_INPUTMAPPING_UNSIGNED_IDENTITY: u32 = 0x00;
const PS_INPUTMAPPING_UNSIGNED_INVERT: u32 = 0x20;
const PS_INPUTMAPPING_EXPAND_NORMAL: u32 = 0x40;
const PS_INPUTMAPPING_EXPAND_NEGATE: u32 = 0x60;
const PS_INPUTMAPPING_HALFBIAS_NORMAL: u32 = 0x80;
const PS_INPUTMAPPING_HALFBIAS_NEGATE: u32 = 0xA0;
const PS_INPUTMAPPING_SIGNED_IDENTITY: u32 = 0xC0;
const PS_INPUTMAPPING_SIGNED_NEGATE: u32 = 0xE0;

const PS_CHANNEL_ALPHA: u32 = 0x10;

const PS_REGISTER_ZERO: u32 = 0x00;
const PS_REGISTER_C0: u32 = 0x01;
const PS_REGISTER_C1: u32 = 0x02;
const PS_REGISTER_FOG: u32 = 0x03;
const PS_REGISTER_V0: u32 = 0x04;
const PS_REGISTER_V1: u32 = 0x05;
const PS_REGISTER_T0: u32 = 0x08;
const PS_REGISTER_T1: u32 = 0x09;
const PS_REGISTER_T2: u32 = 0x0A;
const PS_REGISTER_T3: u32 = 0x0B;
const PS_REGISTER_R0: u32 = 0x0C;
const PS_REGISTER_R1: u32 = 0x0D;
const PS_REGISTER_V1R0_SUM: u32 = 0x0E;
const PS_REGISTER_EF_PROD: u32 = 0x0F;

const PS_COMBINERCOUNT_UNIQUE_C0: u32 = 0x0010;
const PS_COMBINERCOUNT_UNIQUE_C1: u32 = 0x0100;

const PS_COMBINEROUTPUT_IDENTITY: u32 = 0x00;
const PS_COMBINEROUTPUT_BIAS: u32 = 0x08;
const PS_COMBINEROUTPUT_SHIFTLEFT_1: u32 = 0x10;
const PS_COMBINEROUTPUT_SHIFTLEFT_1_BIAS: u32 = 0x18;
const PS_COMBINEROUTPUT_SHIFTLEFT_2: u32 = 0x20;
const PS_COMBINEROUTPUT_SHIFTRIGHT_1: u32 = 0x30;

fn stage_texture_mode(state: PixelShaderState, stage: usize) -> u32 {
    (state.texture_modes >> (stage * 5)) & 0x1F
}

fn stage_samples_texture(state: PixelShaderState, stage: usize) -> bool {
    if state.texture_modes == 0 && stage == 0 {
        return true;
    }
    !matches!(
        stage_texture_mode(state, stage),
        PS_TEXTUREMODES_NONE | PS_TEXTUREMODES_PASSTHRU
    )
}

fn needs_texture2d(mode: u32) -> bool {
    !matches!(
        mode,
        PS_TEXTUREMODES_NONE | PS_TEXTUREMODES_CUBEMAP | PS_TEXTUREMODES_PASSTHRU
    )
}

fn parse_input(value: u32) -> InputInfo {
    InputInfo {
        reg: value & 0x0F,
        chan: value & 0x10,
        mapping: value & 0xE0,
    }
}

fn parse_inputs(value: u32) -> [InputInfo; 4] {
    [
        parse_input((value >> 24) & 0xFF),
        parse_input((value >> 16) & 0xFF),
        parse_input((value >> 8) & 0xFF),
        parse_input(value & 0xFF),
    ]
}

fn parse_output(value: u32) -> OutputInfo {
    let flags = value >> 12;
    OutputInfo {
        cd: value & 0x0F,
        ab: (value >> 4) & 0x0F,
        muxsum: (value >> 8) & 0x0F,
        flags,
        cd_dot: flags & 0x01 != 0,
        ab_dot: flags & 0x02 != 0,
        mux: flags & 0x04 != 0,
        mapping: flags & 0x38,
    }
}

fn color_const(value: u32) -> String {
    if value == 0 {
        return "float4(0.0, 0.0, 0.0, 0.0)".to_string();
    }
    let a = ((value >> 24) & 0xFF) as f32 / 255.0;
    let r = ((value >> 16) & 0xFF) as f32 / 255.0;
    let g = ((value >> 8) & 0xFF) as f32 / 255.0;
    let b = (value & 0xFF) as f32 / 255.0;
    format!("float4({r:.8}, {g:.8}, {b:.8}, {a:.8})")
}

fn c0_expr(state: PixelShaderState, stage: usize) -> String {
    let index = if state.flags() & PS_COMBINERCOUNT_UNIQUE_C0 != 0 {
        stage.min(7)
    } else {
        0
    };
    if state.const0[index] != 0 {
        color_const(state.const0[index])
    } else {
        format!("ps_c[{}]", index * 2)
    }
}

fn c1_expr(state: PixelShaderState, stage: usize) -> String {
    let index = if state.flags() & PS_COMBINERCOUNT_UNIQUE_C1 != 0 {
        stage.min(7)
    } else {
        0
    };
    if state.const1[index] != 0 {
        color_const(state.const1[index])
    } else {
        format!("ps_c[{}]", index * 2 + 1)
    }
}

fn source4_expr(state: PixelShaderState, input: InputInfo, stage: usize) -> String {
    match input.reg {
        PS_REGISTER_ZERO => "float4(0.0, 0.0, 0.0, 0.0)".to_string(),
        PS_REGISTER_C0 => c0_expr(state, stage),
        PS_REGISTER_C1 => c1_expr(state, stage),
        PS_REGISTER_FOG => "fog".to_string(),
        PS_REGISTER_V0 => "v0".to_string(),
        PS_REGISTER_V1 => "v1".to_string(),
        PS_REGISTER_T0 => "t0".to_string(),
        PS_REGISTER_T1 => "t1".to_string(),
        PS_REGISTER_T2 => "t2".to_string(),
        PS_REGISTER_T3 => "t3".to_string(),
        PS_REGISTER_R0 => "r0".to_string(),
        PS_REGISTER_R1 => "r1".to_string(),
        PS_REGISTER_V1R0_SUM => "(v1 + r0)".to_string(),
        PS_REGISTER_EF_PROD => "ef".to_string(),
        _ => "r0".to_string(),
    }
}

fn apply_mapping(expr: String, mapping: u32) -> String {
    match mapping {
        PS_INPUTMAPPING_UNSIGNED_IDENTITY => format!("max(({expr}), 0.0)"),
        PS_INPUTMAPPING_UNSIGNED_INVERT => format!("(1.0 - max(({expr}), 0.0))"),
        PS_INPUTMAPPING_EXPAND_NORMAL => format!("(2.0 * max(({expr}), 0.0) - 1.0)"),
        PS_INPUTMAPPING_EXPAND_NEGATE => format!("(1.0 - 2.0 * max(({expr}), 0.0))"),
        PS_INPUTMAPPING_HALFBIAS_NORMAL => format!("(max(({expr}), 0.0) - 0.5)"),
        PS_INPUTMAPPING_HALFBIAS_NEGATE => format!("(0.5 - max(({expr}), 0.0))"),
        PS_INPUTMAPPING_SIGNED_IDENTITY => expr,
        PS_INPUTMAPPING_SIGNED_NEGATE => format!("-({expr})"),
        _ => expr,
    }
}

fn input_rgb_expr(state: PixelShaderState, input: InputInfo, stage: usize) -> String {
    if input.reg == PS_REGISTER_ZERO {
        return "float3(0.0, 0.0, 0.0)".to_string();
    }
    let base = source4_expr(state, input, stage);
    let expr = if input.chan == PS_CHANNEL_ALPHA {
        format!("({base}).aaa")
    } else {
        format!("({base}).rgb")
    };
    apply_mapping(expr, input.mapping)
}

fn input_alpha_expr(state: PixelShaderState, input: InputInfo, stage: usize) -> String {
    if input.reg == PS_REGISTER_ZERO {
        return "0.0".to_string();
    }
    let base = source4_expr(state, input, stage);
    let expr = if input.chan == PS_CHANNEL_ALPHA {
        format!("({base}).a")
    } else {
        format!("({base}).b")
    };
    apply_mapping(expr, input.mapping)
}

fn output_mapping_expr(expr: String, mapping: u32) -> String {
    match mapping {
        PS_COMBINEROUTPUT_IDENTITY => expr,
        PS_COMBINEROUTPUT_BIAS => format!("(({expr}) - 0.5)"),
        PS_COMBINEROUTPUT_SHIFTLEFT_1 => format!("(({expr}) * 2.0)"),
        PS_COMBINEROUTPUT_SHIFTLEFT_1_BIAS => format!("((({expr}) - 0.5) * 2.0)"),
        PS_COMBINEROUTPUT_SHIFTLEFT_2 => format!("(({expr}) * 4.0)"),
        PS_COMBINEROUTPUT_SHIFTRIGHT_1 => format!("(({expr}) / 2.0)"),
        _ => expr,
    }
}

fn dest_name(reg: u32) -> Option<&'static str> {
    match reg {
        PS_REGISTER_ZERO => None,
        PS_REGISTER_V0 => Some("v0"),
        PS_REGISTER_V1 => Some("v1"),
        PS_REGISTER_T0 => Some("t0"),
        PS_REGISTER_T1 => Some("t1"),
        PS_REGISTER_T2 => Some("t2"),
        PS_REGISTER_T3 => Some("t3"),
        PS_REGISTER_R0 => Some("r0"),
        PS_REGISTER_R1 => Some("r1"),
        _ => Some("r0"),
    }
}

fn dot_or_mul_rgb(a: String, b: String, dot: bool) -> String {
    if dot {
        format!("dot({a}, {b}).xxx")
    } else {
        format!("(({a}) * ({b}))")
    }
}

fn dot_or_mul_alpha(a: String, b: String, _dot: bool) -> String {
    format!("(({a}) * ({b}))")
}

fn append_assign(hlsl: &mut String, dest: Option<&str>, mask: &str, expr: String) -> String {
    if let Some(dest) = dest {
        hlsl.push_str(&format!("    {dest}.{mask} = saturate({expr});\n"));
        format!("{dest}.{mask}")
    } else {
        format!("({expr})")
    }
}

fn append_stage_rgb(hlsl: &mut String, state: PixelShaderState, stage: usize) -> bool {
    let inputs = parse_inputs(state.rgb_inputs[stage]);
    let output = parse_output(state.rgb_outputs[stage]);
    let a = input_rgb_expr(state, inputs[0], stage);
    let b = input_rgb_expr(state, inputs[1], stage);
    let c = input_rgb_expr(state, inputs[2], stage);
    let d = input_rgb_expr(state, inputs[3], stage);
    let ab = dot_or_mul_rgb(a, b, output.ab_dot);
    let cd = dot_or_mul_rgb(c, d, output.cd_dot);
    let ab_mapped = output_mapping_expr(ab.clone(), output.mapping);
    let cd_mapped = output_mapping_expr(cd.clone(), output.mapping);
    let ab_expr = append_assign(hlsl, dest_name(output.ab), "rgb", ab_mapped);
    let cd_expr = append_assign(hlsl, dest_name(output.cd), "rgb", cd_mapped);

    if output.flags & 0x80 != 0 {
        if let Some(dest) = dest_name(output.ab) {
            hlsl.push_str(&format!("    {dest}.a = {dest}.b;\n"));
        }
    }
    if output.flags & 0x40 != 0 {
        if let Some(dest) = dest_name(output.cd) {
            hlsl.push_str(&format!("    {dest}.a = {dest}.b;\n"));
        }
    }

    let sum = if output.mux {
        format!("((r0.a >= 0.5) ? ({cd_expr}) : ({ab_expr}))")
    } else {
        format!("(({ab}) + ({cd}))")
    };
    let sum_mapped = output_mapping_expr(sum, output.mapping);
    append_assign(hlsl, dest_name(output.muxsum), "rgb", sum_mapped);
    state.rgb_outputs[stage] != 0
}

fn append_stage_alpha(hlsl: &mut String, state: PixelShaderState, stage: usize) -> bool {
    let inputs = parse_inputs(state.alpha_inputs[stage]);
    let output = parse_output(state.alpha_outputs[stage]);
    let a = input_alpha_expr(state, inputs[0], stage);
    let b = input_alpha_expr(state, inputs[1], stage);
    let c = input_alpha_expr(state, inputs[2], stage);
    let d = input_alpha_expr(state, inputs[3], stage);
    let ab = dot_or_mul_alpha(a, b, output.ab_dot);
    let cd = dot_or_mul_alpha(c, d, output.cd_dot);
    let ab_mapped = output_mapping_expr(ab.clone(), output.mapping);
    let cd_mapped = output_mapping_expr(cd.clone(), output.mapping);
    let ab_expr = append_assign(hlsl, dest_name(output.ab), "a", ab_mapped);
    let cd_expr = append_assign(hlsl, dest_name(output.cd), "a", cd_mapped);
    let sum = if output.mux {
        format!("((r0.a >= 0.5) ? ({cd_expr}) : ({ab_expr}))")
    } else {
        format!("(({ab}) + ({cd}))")
    };
    let sum_mapped = output_mapping_expr(sum, output.mapping);
    append_assign(hlsl, dest_name(output.muxsum), "a", sum_mapped);
    state.alpha_outputs[stage] != 0
}

fn texture_declarations(state: PixelShaderState) -> String {
    let mut hlsl = String::new();
    for stage in 0..4 {
        let mode = stage_texture_mode(state, stage);
        if stage_samples_texture(state, stage) && mode == PS_TEXTUREMODES_CUBEMAP {
            hlsl.push_str(&format!(
                "TextureCube texCube{stage} : register(t{stage});\n"
            ));
            hlsl.push_str(&format!("SamplerState samp{stage} : register(s{stage});\n"));
        } else if stage_samples_texture(state, stage) || needs_texture2d(mode) {
            hlsl.push_str(&format!("Texture2D tex{stage} : register(t{stage});\n"));
            hlsl.push_str(&format!("SamplerState samp{stage} : register(s{stage});\n"));
        }
    }
    if hlsl.is_empty() {
        hlsl.push_str("Texture2D tex0 : register(t0);\n");
        hlsl.push_str("SamplerState samp0 : register(s0);\n");
    }
    hlsl
}

fn texture_sample_lines(state: PixelShaderState) -> String {
    let mut hlsl = String::new();
    for stage in 0..4 {
        let mode = stage_texture_mode(state, stage);
        let line = if state.texture_modes == 0 && stage == 0 {
            "float4 t0 = tex0.Sample(samp0, input.uv);".to_string()
        } else {
            match mode {
                PS_TEXTUREMODES_NONE => {
                    format!("float4 t{stage} = float4(0.0, 0.0, 0.0, 0.0);")
                }
                PS_TEXTUREMODES_PROJECT2D => {
                    format!("float4 t{stage} = tex{stage}.Sample(samp{stage}, input.uv);")
                }
                PS_TEXTUREMODES_PROJECT3D => {
                    format!("float4 t{stage} = tex{stage}.Sample(samp{stage}, input.uv);")
                }
                PS_TEXTUREMODES_CUBEMAP => format!(
                    "float4 t{stage} = texCube{stage}.Sample(samp{stage}, normalize(float3(input.uv * 2.0 - 1.0, 1.0)));"
                ),
                PS_TEXTUREMODES_PASSTHRU => {
                    format!("float4 t{stage} = float4(input.uv, 0.0, 1.0);")
                }
                PS_TEXTUREMODES_DOTPRODUCT => {
                    format!("float4 t{stage} = dot(float3(input.uv, 1.0), t0.rgb).xxxx;")
                }
                _ => format!("float4 t{stage} = tex{stage}.Sample(samp{stage}, input.uv);"),
            }
        };
        hlsl.push_str("    ");
        hlsl.push_str(&line);
        hlsl.push('\n');
    }
    hlsl
}

fn texture_alpha_kill_lines(state: PixelShaderState) -> String {
    let _ = state;
    String::new()
}

fn append_final_combiner(hlsl: &mut String, state: PixelShaderState) {
    if state.final_abcd == 0 && state.final_efg == 0 {
        return;
    }

    let abcd = parse_inputs(state.final_abcd);
    let efg = parse_inputs(state.final_efg);
    let e_rgb = input_rgb_expr(state, efg[0], 8);
    let f_rgb = input_rgb_expr(state, efg[1], 8);
    let e_a = input_alpha_expr(state, efg[0], 8);
    let f_a = input_alpha_expr(state, efg[1], 8);
    hlsl.push_str("    // Final combiner\n");
    hlsl.push_str(&format!(
        "    ef = float4(({e_rgb}) * ({f_rgb}), ({e_a}) * ({f_a}));\n"
    ));

    let a = input_rgb_expr(state, abcd[0], 8);
    let b = input_rgb_expr(state, abcd[1], 8);
    let c = input_rgb_expr(state, abcd[2], 8);
    let d = input_rgb_expr(state, abcd[3], 8);
    let g = input_alpha_expr(state, efg[2], 8);
    hlsl.push_str(&format!(
        "    r0.rgb = saturate(({d}) + lerp(({c}), ({b}), ({a})));\n"
    ));
    hlsl.push_str(&format!("    r0.a = saturate({g});\n"));
}

fn stage_body(state: PixelShaderState) -> String {
    let mut hlsl = String::new();
    let mut any_stage_wrote = false;
    for stage in 0..state.stage_count() {
        hlsl.push_str(&format!("    // Combiner stage {stage}\n"));
        let rgb_wrote = append_stage_rgb(&mut hlsl, state, stage);
        let alpha_wrote = append_stage_alpha(&mut hlsl, state, stage);
        any_stage_wrote |= rgb_wrote || alpha_wrote;
        if !rgb_wrote && !alpha_wrote {
            hlsl.push_str("    // Unsupported/empty stage output, keeping r0 as fallback.\n");
        }
    }
    if !any_stage_wrote {
        hlsl.push_str("    r0 = saturate(t0 * v0);\n");
    }
    append_final_combiner(&mut hlsl, state);
    hlsl
}

fn pixel_shader_hlsl(state: PixelShaderState) -> String {
    let declarations = texture_declarations(state);
    let samples = texture_sample_lines(state);
    let alpha_kill = texture_alpha_kill_lines(state);
    let body = stage_body(state);
    format!(
        r#"
{declarations}

cbuffer RustemuPsConstants : register(b0) {{
    float4 ps_c[32];
    float4 ps_state;
}};

float rustemu_alpha_test(float alpha) {{
    if (ps_state.z < 0.5) return 1.0;

    uint func = (uint)round(ps_state.y);
    float ref_alpha = ps_state.w;
    if (func == 1) return -1.0;
    if (func == 2) return (alpha < ref_alpha) ? 1.0 : -1.0;
    if (func == 3) return (abs(alpha - ref_alpha) <= (0.5 / 255.0)) ? 1.0 : -1.0;
    if (func == 4) return (alpha <= ref_alpha) ? 1.0 : -1.0;
    if (func == 5) return (alpha > ref_alpha) ? 1.0 : -1.0;
    if (func == 6) return (abs(alpha - ref_alpha) > (0.5 / 255.0)) ? 1.0 : -1.0;
    if (func == 7) return (alpha >= ref_alpha) ? 1.0 : -1.0;
    return 1.0;
}}

struct PS_IN {{
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
}};

float4 main(PS_IN input) : SV_Target {{
{samples}
{alpha_kill}
    float4 v0 = input.color;
    float4 v1 = input.color;
    float4 fog = float4(0.0, 0.0, 0.0, 1.0);
    float4 r0 = float4(0.0, 0.0, 0.0, 1.0);
    float4 r1 = float4(0.0, 0.0, 0.0, 1.0);
    float4 ef = float4(0.0, 0.0, 0.0, 0.0);
{body}
    r0.a *= ps_state.x;
    clip(rustemu_alpha_test(r0.a));
    return saturate(r0);
}}
"#
    )
}
