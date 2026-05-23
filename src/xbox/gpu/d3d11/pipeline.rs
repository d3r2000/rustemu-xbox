//! D3D11 pipeline state layer.
//!
//! Owns fixed-function render-state translation, blend/depth state rebuilds,
//! and pixel-shader constant uploads for the D3D11 backend.
//!
//! This module is intentionally host-D3D11-specific. Backend-neutral Xbox state
//! decoding should live above this module if D3D12 needs the same semantics.

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::Fxc::{
    D3DCompile, D3DCOMPILE_ENABLE_STRICTNESS, D3DCOMPILE_OPTIMIZATION_LEVEL1,
};
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::*;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::*;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::*;

use super::super::render_state::{normalize_render_state_input, Blend, CmpFunc, HostStateGroup};
use super::{
    D3D11Backend, D3D11_VS_CBUFFER_COUNT, D3D11_VS_CLIP_RANGE_REG, D3D11_VS_CONSTANT_COUNT,
    D3D11_VS_SURFACE_SIZE_REG,
};

#[cfg(windows)]
const TEXTURED_PS_HLSL: &str = r#"
Texture2D tex0 : register(t0);
Texture2D tex1 : register(t1);
Texture2D tex2 : register(t2);
Texture2D tex3 : register(t3);
SamplerState samp0 : register(s0);
SamplerState samp1 : register(s1);
SamplerState samp2 : register(s2);
SamplerState samp3 : register(s3);

cbuffer RustemuPsConstants : register(b0) {
    float4 ps_c[32];
    float4 ps_state;
    float4 ps_tss[4];
    float4 ps_tss_args[4];
    float4 ps_tss_misc[4];
    float4 ps_tfactor;
};

struct VS_OUT {
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
};

float4 ff_sample(uint stage, float2 uv) {
    if (stage == 0) return tex0.Sample(samp0, uv);
    if (stage == 1) return tex1.Sample(samp1, uv);
    if (stage == 2) return tex2.Sample(samp2, uv);
    return tex3.Sample(samp3, uv);
}

float4 ff_arg4(
    uint encoded,
    float4 tex_color,
    float4 diffuse,
    float4 current,
    float4 specular,
    float4 temp,
    float4 tfactor
) {
    uint base = encoded & 0x0F;
    float4 v = current;
    if (base == 0) {
        v = diffuse;
    } else if (base == 1) {
        v = current;
    } else if (base == 2) {
        v = tex_color;
    } else if (base == 3) {
        v = tfactor;
    } else if (base == 4) {
        v = specular;
    } else if (base == 5) {
        v = temp;
    } else if (base == 6) {
        v = tfactor;
    } else {
        v = float4(0.0, 0.0, 0.0, 0.0);
    }

    if ((encoded & 0x20) != 0) {
        return saturate(v.aaaa);
    } else if ((encoded & 0x10) != 0) {
        v = 1.0 - v;
    }
    return saturate(v);
}

float4 ff_texture_op(
    uint op,
    float4 a,
    float4 b,
    float4 arg0,
    float4 current,
    float4 diffuse,
    float4 tex_color,
    float4 tfactor
) {
    if (op == 1) return current;          // DISABLE
    if (op == 2) return a;                // SELECTARG1
    if (op == 3) return b;                // SELECTARG2
    if (op == 4) return a * b;            // MODULATE
    if (op == 5) return 2.0 * a * b;      // MODULATE2X
    if (op == 6) return 4.0 * a * b;      // MODULATE4X
    if (op == 7) return a + b;            // ADD
    if (op == 8) return a + b - 0.5;      // ADDSIGNED
    if (op == 9) return 2.0 * (a + b - 0.5); // ADDSIGNED2X
    if (op == 10) return a - b;           // SUBTRACT
    if (op == 11) return a + b * (1.0 - a); // ADDSMOOTH
    if (op == 12) return a * diffuse.a + b * (1.0 - diffuse.a); // BLENDDIFFUSEALPHA
    if (op == 13) return a * current.a + b * (1.0 - current.a); // BLENDCURRENTALPHA
    if (op == 14) return a * tex_color.a + b * (1.0 - tex_color.a); // BLENDTEXTUREALPHA
    if (op == 15) return a * tfactor.a + b * (1.0 - tfactor.a); // BLENDFACTORALPHA
    if (op == 16) return a + b * (1.0 - tex_color.a); // BLENDTEXTUREALPHAPM
    if (op == 17) return a;                // PREMODULATE
    if (op == 18) return float4(a.rgb + a.a * b.rgb, 1.0); // MODULATEALPHA_ADDCOLOR
    if (op == 19) return float4(a.rgb * b.rgb + a.a, 1.0); // MODULATECOLOR_ADDALPHA
    if (op == 20) return float4((1.0 - a.a) * b.rgb + a.rgb, 1.0);
    if (op == 21) return float4((1.0 - a.rgb) * b.rgb + a.a, 1.0);
    if (op == 22) {
        float d = dot((a.rgb - 0.5) * 2.0, (b.rgb - 0.5) * 2.0);
        return saturate(float4(d, d, d, d));
    }
    if (op == 23) return arg0 + a * b;     // MULTIPLYADD
    if (op == 24) return arg0 * a + (1.0 - arg0) * b; // LERP
    return a * b;
}

float ff_alpha_test(float alpha) {
    if (ps_state.z < 0.5) return 1.0;

    uint func = (uint)round(ps_state.y);
    float ref_alpha = ps_state.w;
    if (func == 1) return -1.0; // NEVER
    if (func == 2) return (alpha < ref_alpha) ? 1.0 : -1.0;
    if (func == 3) return (abs(alpha - ref_alpha) <= (0.5 / 255.0)) ? 1.0 : -1.0;
    if (func == 4) return (alpha <= ref_alpha) ? 1.0 : -1.0;
    if (func == 5) return (alpha > ref_alpha) ? 1.0 : -1.0;
    if (func == 6) return (abs(alpha - ref_alpha) > (0.5 / 255.0)) ? 1.0 : -1.0;
    if (func == 7) return (alpha >= ref_alpha) ? 1.0 : -1.0;
    return 1.0; // ALWAYS and unknown values
}

void ff_apply_stage(uint stage, inout uint previous_op, inout float4 current, inout float4 temp, float4 diffuse, float4 specular, float4 tfactor, float2 uv) {
    uint color_op = (uint)round(ps_tss[stage].x);
    if (color_op == 1) {
        previous_op = color_op;
        return;
    }

    uint alpha_op = (uint)round(ps_tss[stage].y);
    uint color_arg1 = (uint)round(ps_tss[stage].z);
    uint color_arg2 = (uint)round(ps_tss[stage].w);
    uint alpha_arg1 = (uint)round(ps_tss_args[stage].x);
    uint alpha_arg2 = (uint)round(ps_tss_args[stage].y);
    uint color_arg0 = (uint)round(ps_tss_args[stage].z);
    uint alpha_arg0 = (uint)round(ps_tss_args[stage].w);
    uint result_arg = (uint)round(ps_tss_misc[stage].x);

    float4 tex_color = ff_sample(stage, uv);
    if (previous_op == 17) {
        current *= tex_color;
    }

    float4 c0 = ff_arg4(color_arg0, tex_color, diffuse, current, specular, temp, tfactor);
    float4 c1 = ff_arg4(color_arg1, tex_color, diffuse, current, specular, temp, tfactor);
    float4 c2 = ff_arg4(color_arg2, tex_color, diffuse, current, specular, temp, tfactor);
    float4 a0 = ff_arg4(alpha_arg0, tex_color, diffuse, current, specular, temp, tfactor);
    float4 a1 = ff_arg4(alpha_arg1, tex_color, diffuse, current, specular, temp, tfactor);
    float4 a2 = ff_arg4(alpha_arg2, tex_color, diffuse, current, specular, temp, tfactor);

    float4 value = current;
    value.rgb = ff_texture_op(color_op, c1, c2, c0, current, diffuse, tex_color, tfactor).rgb;
    if (alpha_op != 1) {
        value.a = ff_texture_op(alpha_op, a1, a2, a0, current, diffuse, tex_color, tfactor).a;
    }

    if (result_arg == 5) {
        temp = value;
    } else {
        current = value;
    }
    previous_op = color_op;
}

float4 main(VS_OUT input) : SV_Target {
    float alpha_mod = ps_state.x;

    float4 diffuse = saturate(input.color);
    float4 current = diffuse;
    float4 specular = diffuse;
    float4 temp = float4(0.0, 0.0, 0.0, 0.0);
    uint previous_op = 0xFFFFFFFF;

    ff_apply_stage(0, previous_op, current, temp, diffuse, specular, ps_tfactor, input.uv);
    ff_apply_stage(1, previous_op, current, temp, diffuse, specular, ps_tfactor, input.uv);
    ff_apply_stage(2, previous_op, current, temp, diffuse, specular, ps_tfactor, input.uv);
    ff_apply_stage(3, previous_op, current, temp, diffuse, specular, ps_tfactor, input.uv);

    current.a *= alpha_mod;
    clip(ff_alpha_test(current.a));
    return saturate(current);
}
"#;

#[cfg(windows)]
const SPIDEY_SOLID_PS_HLSL: &str = r#"
struct VS_OUT {
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
};

float4 main(VS_OUT input) : SV_Target {
    return float4(1.0, 0.0, 1.0, 1.0);
}
"#;

impl D3D11Backend {
    #[cfg(windows)]
    pub(super) fn upload_vs_constants(&self) {
        let (Some(ctx), Some(cb)) = (&self.ctx, &self.vs_cbuffer) else {
            return;
        };

        let mut data = [[0.0f32; 4]; D3D11_VS_CBUFFER_COUNT];
        data[..D3D11_VS_CONSTANT_COUNT].copy_from_slice(&self.vs_constants);
        data[D3D11_VS_SURFACE_SIZE_REG] = [self.width as f32, self.height as f32, 0.0, 0.0];
        data[D3D11_VS_CLIP_RANGE_REG] = [0.0, 1.0, 0.0, 0.0];

        unsafe {
            ctx.UpdateSubresource(cb, 0, None, data.as_ptr() as *const core::ffi::c_void, 0, 0);
            ctx.VSSetConstantBuffers(0, Some(&[Some(cb.clone())]));
        }
    }

    #[cfg(windows)]
    pub(super) fn compile_textured_pixel_shader(
        device: &ID3D11Device,
    ) -> Option<ID3D11PixelShader> {
        let mut code: Option<ID3DBlob> = None;
        let mut errors: Option<ID3DBlob> = None;
        let result = unsafe {
            D3DCompile(
                TEXTURED_PS_HLSL.as_ptr() as *const core::ffi::c_void,
                TEXTURED_PS_HLSL.len(),
                windows::core::s!("rustemu_textured_ps"),
                None,
                None,
                windows::core::s!("main"),
                windows::core::s!("ps_4_0"),
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
                "[D3D11-PS-TEXTURED-COMPILE-FAIL] err={} {}",
                e, err_text
            ));
            return None;
        }

        let code = code?;
        let bytecode = unsafe {
            std::slice::from_raw_parts(code.GetBufferPointer() as *const u8, code.GetBufferSize())
        };
        let mut ps: Option<ID3D11PixelShader> = None;
        if let Err(e) = unsafe { device.CreatePixelShader(bytecode, None, Some(&mut ps)) } {
            crate::xbox::emulator::debug_log(&format!("[D3D11-PS-TEXTURED-CREATE-FAIL] err={}", e));
            return None;
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-PS-TEXTURED-COMPILE] bytes={} hlsl_len={}",
            bytecode.len(),
            TEXTURED_PS_HLSL.len()
        ));
        ps
    }

    #[cfg(windows)]
    pub(super) fn compile_spidey_solid_pixel_shader(
        device: &ID3D11Device,
    ) -> Option<ID3D11PixelShader> {
        let mut code: Option<ID3DBlob> = None;
        let mut errors: Option<ID3DBlob> = None;
        let result = unsafe {
            D3DCompile(
                SPIDEY_SOLID_PS_HLSL.as_ptr() as *const core::ffi::c_void,
                SPIDEY_SOLID_PS_HLSL.len(),
                windows::core::s!("rustemu_spidey_solid_ps"),
                None,
                None,
                windows::core::s!("main"),
                windows::core::s!("ps_4_0"),
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
                "[D3D11-PS-SPIDEY-SOLID-COMPILE-FAIL] err={} {}",
                e, err_text
            ));
            return None;
        }

        let code = code?;
        let bytecode = unsafe {
            std::slice::from_raw_parts(code.GetBufferPointer() as *const u8, code.GetBufferSize())
        };
        let mut ps: Option<ID3D11PixelShader> = None;
        if let Err(e) = unsafe { device.CreatePixelShader(bytecode, None, Some(&mut ps)) } {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-PS-SPIDEY-SOLID-CREATE-FAIL] err={}",
                e
            ));
            return None;
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-PS-SPIDEY-SOLID-COMPILE] bytes={} hlsl_len={}",
            bytecode.len(),
            SPIDEY_SOLID_PS_HLSL.len()
        ));
        ps
    }

    #[cfg(windows)]
    pub(super) fn compile_nv2a_vertex_shader(
        &self,
        handle: u32,
        hlsl: &str,
    ) -> Option<(ID3D11VertexShader, ID3D11InputLayout)> {
        let Some(device) = &self.device else {
            return None;
        };
        let mut code: Option<ID3DBlob> = None;
        let mut errors: Option<ID3DBlob> = None;
        let patched_hlsl;
        let shader_source = if hlsl.contains("bool zeroTransform = ")
            && hlsl.contains("rawPos")
            && !hlsl.contains("c[191].x > 0.5")
        {
            patched_hlsl = hlsl.replace(
                "bool zeroTransform = ",
                "bool zeroTransform = (c[191].x > 0.5) && ",
            );
            patched_hlsl.as_str()
        } else {
            hlsl
        };

        let result = unsafe {
            D3DCompile(
                shader_source.as_ptr() as *const core::ffi::c_void,
                shader_source.len(),
                windows::core::s!("rustemu_nv2a_vsh"),
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
                "[D3D11-VSH-COMPILE-FAIL] handle=0x{handle:08X} err={} {}",
                e, err_text
            ));
            return None;
        }

        let code = code?;
        let bytecode = unsafe {
            std::slice::from_raw_parts(code.GetBufferPointer() as *const u8, code.GetBufferSize())
        };
        let mut vs: Option<ID3D11VertexShader> = None;
        if let Err(e) = unsafe { device.CreateVertexShader(bytecode, None, Some(&mut vs)) } {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-VSH-CREATE-FAIL] handle=0x{handle:08X} err={}",
                e
            ));
            return None;
        }
        let register_base = 92u32;
        let layout_desc: Vec<D3D11_INPUT_ELEMENT_DESC> = (0..16u32)
            .map(|i| D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::s!("TEXCOORD"),
                SemanticIndex: i,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: register_base + i * 16,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            })
            .collect();
        let mut layout: Option<ID3D11InputLayout> = None;
        if let Err(e) =
            unsafe { device.CreateInputLayout(&layout_desc, bytecode, Some(&mut layout)) }
        {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-VSH-LAYOUT-FAIL] handle=0x{handle:08X} err={}",
                e
            ));
            return None;
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-VSH-COMPILE] handle=0x{handle:08X} bytes={} hlsl_len={} rawpos_guard={} cxbxr_screenspace={}",
            bytecode.len(),
            shader_source.len(),
            shader_source.contains("c[191].x > 0.5"),
            shader_source.contains("screenScale = c[212]")
        ));
        Some((vs?, layout?))
    }

    #[cfg(windows)]
    pub(super) fn compile_nv2a_pixel_shader(
        &self,
        key: u64,
        hlsl: &str,
    ) -> Option<ID3D11PixelShader> {
        let Some(device) = &self.device else {
            return None;
        };
        if let Ok(raw_dir) = std::env::var("RUSTEMU_DUMP_PSH_HLSL") {
            let raw_dir = raw_dir.trim();
            if raw_dir != "0" && !raw_dir.eq_ignore_ascii_case("false") {
                let dir =
                    if raw_dir.is_empty() || raw_dir == "1" || raw_dir.eq_ignore_ascii_case("true")
                    {
                        std::path::PathBuf::from("psh_dumps")
                    } else {
                        std::path::PathBuf::from(raw_dir)
                    };
                if std::fs::create_dir_all(&dir).is_ok() {
                    let path = dir.join(format!("psh_{key:016X}.hlsl"));
                    if std::fs::write(&path, hlsl).is_ok() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-PSH-DUMP] key=0x{key:016X} path={}",
                            path.display()
                        ));
                    }
                }
            }
        }
        let mut code: Option<ID3DBlob> = None;
        let mut errors: Option<ID3DBlob> = None;
        let result = unsafe {
            D3DCompile(
                hlsl.as_ptr() as *const core::ffi::c_void,
                hlsl.len(),
                windows::core::s!("rustemu_nv2a_psh"),
                None,
                None,
                windows::core::s!("main"),
                windows::core::s!("ps_4_0"),
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
                "[D3D11-PSH-COMPILE-FAIL] key=0x{key:016X} err={} {}",
                e, err_text
            ));
            return None;
        }

        let code = code?;
        let bytecode = unsafe {
            std::slice::from_raw_parts(code.GetBufferPointer() as *const u8, code.GetBufferSize())
        };
        let mut ps: Option<ID3D11PixelShader> = None;
        if let Err(e) = unsafe { device.CreatePixelShader(bytecode, None, Some(&mut ps)) } {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-PSH-CREATE-FAIL] key=0x{key:016X} err={}",
                e
            ));
            return None;
        }
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-PSH-COMPILE] key=0x{key:016X} bytes={} hlsl_len={}",
            bytecode.len(),
            hlsl.len()
        ));
        ps
    }

    #[cfg(windows)]
    pub(super) fn d3d_blend_factor(value: Blend) -> D3D11_BLEND {
        match value {
            Blend::Zero => D3D11_BLEND_ZERO,
            Blend::One => D3D11_BLEND_ONE,
            Blend::SrcColor => D3D11_BLEND_SRC_COLOR,
            Blend::InvSrcColor => D3D11_BLEND_INV_SRC_COLOR,
            Blend::SrcAlpha | Blend::BothSrcAlpha => D3D11_BLEND_SRC_ALPHA,
            Blend::InvSrcAlpha | Blend::BothInvSrcAlpha => D3D11_BLEND_INV_SRC_ALPHA,
            Blend::DestAlpha => D3D11_BLEND_DEST_ALPHA,
            Blend::InvDestAlpha => D3D11_BLEND_INV_DEST_ALPHA,
            Blend::DestColor => D3D11_BLEND_DEST_COLOR,
            Blend::InvDestColor => D3D11_BLEND_INV_DEST_COLOR,
            Blend::SrcAlphaSat => D3D11_BLEND_SRC_ALPHA_SAT,
            Blend::BlendFactor => D3D11_BLEND_BLEND_FACTOR,
            Blend::InvBlendFactor => D3D11_BLEND_INV_BLEND_FACTOR,
        }
    }

    #[cfg(windows)]
    pub(super) fn d3d_blend_op(value: u32) -> D3D11_BLEND_OP {
        match value {
            2 => D3D11_BLEND_OP_SUBTRACT,
            3 => D3D11_BLEND_OP_REV_SUBTRACT,
            4 => D3D11_BLEND_OP_MIN,
            5 => D3D11_BLEND_OP_MAX,
            _ => D3D11_BLEND_OP_ADD,
        }
    }

    #[cfg(windows)]
    fn d3d_compare_func(value: CmpFunc) -> D3D11_COMPARISON_FUNC {
        match value {
            CmpFunc::Never => D3D11_COMPARISON_NEVER,
            CmpFunc::Less => D3D11_COMPARISON_LESS,
            CmpFunc::Equal => D3D11_COMPARISON_EQUAL,
            CmpFunc::LessEqual => D3D11_COMPARISON_LESS_EQUAL,
            CmpFunc::Greater => D3D11_COMPARISON_GREATER,
            CmpFunc::NotEqual => D3D11_COMPARISON_NOT_EQUAL,
            CmpFunc::GreaterEqual => D3D11_COMPARISON_GREATER_EQUAL,
            CmpFunc::Always => D3D11_COMPARISON_ALWAYS,
        }
    }

    #[cfg(windows)]
    pub(super) fn d3d_color_write_mask(value: u32) -> u8 {
        let mut mask = 0u8;
        if value & 0x1 != 0 {
            mask |= D3D11_COLOR_WRITE_ENABLE_RED.0 as u8;
        }
        if value & 0x2 != 0 {
            mask |= D3D11_COLOR_WRITE_ENABLE_GREEN.0 as u8;
        }
        if value & 0x4 != 0 {
            mask |= D3D11_COLOR_WRITE_ENABLE_BLUE.0 as u8;
        }
        if value & 0x8 != 0 {
            mask |= D3D11_COLOR_WRITE_ENABLE_ALPHA.0 as u8;
        }
        mask
    }

    #[cfg(windows)]
    pub(super) fn rebuild_blend_state(&mut self) {
        let (Some(device), Some(ctx)) = (&self.device, &self.ctx) else {
            return;
        };
        let rs = self.render_state;
        let write_mask = Self::d3d_color_write_mask(rs.color_write_enable);
        let mut blend_desc = D3D11_BLEND_DESC::default();
        blend_desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: rs.alpha_blend_enable.into(),
            SrcBlend: Self::d3d_blend_factor(rs.src_blend),
            DestBlend: Self::d3d_blend_factor(rs.dest_blend),
            BlendOp: Self::d3d_blend_op(rs.blend_op),
            SrcBlendAlpha: Self::d3d_blend_factor(rs.src_blend),
            DestBlendAlpha: Self::d3d_blend_factor(rs.dest_blend),
            BlendOpAlpha: Self::d3d_blend_op(rs.blend_op),
            RenderTargetWriteMask: write_mask,
        };
        let mut bs: Option<ID3D11BlendState> = None;
        unsafe {
            if device.CreateBlendState(&blend_desc, Some(&mut bs)).is_ok() {
                if let Some(ref bs) = bs {
                    ctx.OMSetBlendState(bs, None, 0xFFFF_FFFF);
                }
                self.bs = bs;
            }
        }
        static BLEND_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = BLEND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RS-BLEND] #{} enable={} src={:?} dst={:?} op={} mask=0x{:X} d3d_mask=0x{:X}",
                n,
                rs.alpha_blend_enable,
                rs.src_blend,
                rs.dest_blend,
                rs.blend_op,
                rs.color_write_enable,
                write_mask
            ));
        }
    }

    #[cfg(windows)]
    pub(super) fn rebuild_depth_state(&mut self) {
        let (Some(device), Some(ctx)) = (&self.device, &self.ctx) else {
            return;
        };
        let rs = self.render_state;
        let ds_state_desc = D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: (rs.z_enable != 0).into(),
            DepthWriteMask: if rs.z_write_enable {
                D3D11_DEPTH_WRITE_MASK_ALL
            } else {
                D3D11_DEPTH_WRITE_MASK_ZERO
            },
            DepthFunc: Self::d3d_compare_func(rs.z_func),
            StencilEnable: false.into(),
            ..Default::default()
        };
        let mut dss: Option<ID3D11DepthStencilState> = None;
        unsafe {
            if device
                .CreateDepthStencilState(&ds_state_desc, Some(&mut dss))
                .is_ok()
            {
                if let Some(ref dss) = dss {
                    ctx.OMSetDepthStencilState(dss, 0);
                }
                self.dss = dss;
            }
        }
        static DEPTH_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DEPTH_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RS-DEPTH] #{} enable={} write={} func={:?} bias=0x{:08X}",
                n, rs.z_enable, rs.z_write_enable, rs.z_func, rs.z_bias
            ));
        }
    }

    #[cfg(windows)]
    fn effective_ps_alpha_mod(&self) -> f32 {
        // 2026-05-08: The NV2A pixel-shader combiner constant c[2][w] is a
        // per-draw user constant, not a global alpha scale. Correct combiner-
        // level alpha scaling needs a proper NV2A combiner parser.
        1.0
    }

    #[cfg(windows)]
    pub(super) fn upload_ps_constants(&self) {
        let (Some(ctx), Some(cb)) = (&self.ctx, &self.ps_cbuffer) else {
            return;
        };

        let mut data = [[0.0f32; 4]; 46];
        data[..32].copy_from_slice(&self.ps_constants);
        data[32] = [
            self.effective_ps_alpha_mod(),
            self.render_state.alpha_func as u32 as f32,
            if self.render_state.alpha_test_enable {
                1.0
            } else {
                0.0
            },
            self.render_state.alpha_ref as f32 / 255.0,
        ];
        for stage in 0..4 {
            let s = self.texture_stage_states[stage];
            data[33 + stage] = [
                s[super::super::X_D3DTSS_COLOROP] as f32,
                s[super::super::X_D3DTSS_ALPHAOP] as f32,
                s[super::super::X_D3DTSS_COLORARG1] as f32,
                s[super::super::X_D3DTSS_COLORARG2] as f32,
            ];
            data[37 + stage] = [
                s[super::super::X_D3DTSS_ALPHAARG1] as f32,
                s[super::super::X_D3DTSS_ALPHAARG2] as f32,
                s[super::super::X_D3DTSS_COLORARG0] as f32,
                s[super::super::X_D3DTSS_ALPHAARG0] as f32,
            ];
            data[41 + stage] = [s[super::super::X_D3DTSS_RESULTARG] as f32, 0.0, 0.0, 0.0];
        }

        let tfactor = self.render_state.texture_factor;
        let tfactor_rgba = [
            ((tfactor >> 16) & 0xFF) as f32 / 255.0,
            ((tfactor >> 8) & 0xFF) as f32 / 255.0,
            (tfactor & 0xFF) as f32 / 255.0,
            ((tfactor >> 24) & 0xFF) as f32 / 255.0,
        ];
        data[45] = tfactor_rgba;

        unsafe {
            ctx.UpdateSubresource(cb, 0, None, data.as_ptr() as *const core::ffi::c_void, 0, 0);
            ctx.PSSetConstantBuffers(0, Some(&[Some(cb.clone())]));
        }
    }

    #[cfg(windows)]
    pub(super) fn apply_render_state(&mut self, state: u32, value: u32) {
        if self.handle_hle_render_target_control(state, value) {
            return;
        }
        let (state, value) = normalize_render_state_input(state, value);
        if Self::is_pixel_shader_render_state(state) {
            self.ps_render_states[state as usize] = value;
        }
        if let Some(group) = self.render_state.apply(state, value) {
            if matches!(group, HostStateGroup::Blend) {
                self.rebuild_blend_state();
                self.mark_ps_constants_dirty();
            } else if matches!(group, HostStateGroup::Depth | HostStateGroup::Stencil) {
                self.rebuild_depth_state();
            }
        }
    }
}
