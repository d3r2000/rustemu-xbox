#[cfg(windows)]
use std::collections::HashMap;
/// D3D9Ex hardware rendering backend — intended Cxbx-R-style host target.
/// Cxbx-R translates Xbox D3D8 → host D3D9Ex. This backend uses the same API
/// family, but it is not yet feature-parity with the D3D11 backend or Cxbx-R:
/// shader binding, render-target texture routing, scanout/display routing, and
/// some texture/render-state translation still need to be ported/shared before
/// forced-D3D9 runs are fair visual parity tests.
///
/// Architecture: offscreen render target → GetRenderTargetData → CPU readback.
/// No swap chain — RetroArch owns the window.

#[cfg(windows)]
use windows::{Win32::Foundation::HWND, Win32::Graphics::Direct3D9::*};

use super::{
    default_texture_stage_states,
    render_state::{
        normalize_render_state_input, CullMode, FillMode, RenderStateTable, XboxRenderState as Rs,
    },
    GpuBackend, NV2AVertex, X_D3DTA_TEXTURE, X_D3DTOP_DISABLE, X_D3DTSS_ADDRESSU,
    X_D3DTSS_ADDRESSV, X_D3DTSS_ADDRESSW, X_D3DTSS_ALPHAARG0, X_D3DTSS_ALPHAARG1,
    X_D3DTSS_ALPHAARG2, X_D3DTSS_ALPHAOP, X_D3DTSS_COLORARG0, X_D3DTSS_COLORARG1,
    X_D3DTSS_COLORARG2, X_D3DTSS_COLOROP, X_D3DTSS_MAGFILTER, X_D3DTSS_MINFILTER,
    X_D3DTSS_MIPFILTER, X_D3DTSS_RESULTARG, X_D3DTSS_TEXCOORDINDEX, X_D3DTSS_TEXTURETRANSFORMFLAGS,
};

/// Map NV097 primitive type → D3D9 primitive type
fn map_prim_type(nv097: i32) -> D3DPRIMITIVETYPE {
    match nv097 {
        1 => D3DPT_POINTLIST,
        2 => D3DPT_LINELIST,
        3 | 4 => D3DPT_LINESTRIP,
        5 => D3DPT_TRIANGLELIST,
        6 => D3DPT_TRIANGLESTRIP,
        7 => D3DPT_TRIANGLEFAN,
        _ => D3DPT_TRIANGLELIST,
    }
}

/// Vertex format for D3D9 FVF: position (XYZRHW) + diffuse color + tex coords
#[repr(C)]
#[derive(Clone, Copy)]
struct D3D9Vertex {
    x: f32,
    y: f32,
    z: f32,
    rhw: f32,
    color: u32,
    u: f32,
    v: f32,
}

const D3D9_FVF: u32 = 0x00000004 | 0x00000040 | 0x00000100;
// D3DFVF_XYZRHW (0x04) | D3DFVF_DIFFUSE (0x40) | D3DFVF_TEX1 (0x100)

#[cfg(windows)]
const DEFAULT_BACKBUFFER_RT_KEY: u32 = 0xFFFF_FF00;

#[cfg(windows)]
#[derive(Clone, Copy, Default)]
struct D3D9PendingRenderTarget {
    key: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
}

#[cfg(windows)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct D3D9NormalizedRenderTarget {
    key: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
}

#[cfg(windows)]
struct D3D9RenderTargetSurface {
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
    tex: IDirect3DTexture9,
    surface: IDirect3DSurface9,
    depth: Option<IDirect3DSurface9>,
    offscreen: IDirect3DSurface9,
}

pub struct D3D9Backend {
    width: i32,
    height: i32,
    #[cfg(windows)]
    d3d9: Option<IDirect3D9>,
    #[cfg(windows)]
    device: Option<IDirect3DDevice9>,
    #[cfg(windows)]
    rt: Option<IDirect3DSurface9>,
    #[cfg(windows)]
    default_depth: Option<IDirect3DSurface9>,
    #[cfg(windows)]
    offscreen: Option<IDirect3DSurface9>,
    #[cfg(windows)]
    textures: Vec<Option<IDirect3DTexture9>>,
    #[cfg(windows)]
    rt_cache: HashMap<u32, D3D9RenderTargetSurface>,
    #[cfg(windows)]
    pending_rt: D3D9PendingRenderTarget,
    #[cfg(windows)]
    active_rt_key: u32,
    #[cfg(windows)]
    backbuffer_rt_key: u32,
    #[cfg(windows)]
    backbuffer_rt_info: D3D9PendingRenderTarget,
    render_state: RenderStateTable,
    texture_stage_states: [[u32; 32]; 4],
    display_buffer: Vec<u32>,
}

// SAFETY: D3D9 backend is only used from the main thread (GPU init + readback).
// Worker thread signals via atomic flag, main thread does the actual D3D9 calls.
unsafe impl Send for D3D9Backend {}

impl D3D9Backend {
    pub fn new() -> Self {
        let mut render_state = RenderStateTable::default();
        // The current D3D9 path submits pre-lit XYZRHW vertices. Until the
        // backend has Cxbx-style material/normal routing, keep host lighting
        // disabled like the historical thin D3D9 path did.
        render_state.lighting = false;
        Self {
            width: 0,
            height: 0,
            #[cfg(windows)]
            d3d9: None,
            #[cfg(windows)]
            device: None,
            #[cfg(windows)]
            rt: None,
            #[cfg(windows)]
            default_depth: None,
            #[cfg(windows)]
            offscreen: None,
            #[cfg(windows)]
            textures: std::iter::repeat_with(|| None).take(4).collect(),
            #[cfg(windows)]
            rt_cache: HashMap::new(),
            #[cfg(windows)]
            pending_rt: D3D9PendingRenderTarget::default(),
            #[cfg(windows)]
            active_rt_key: 0,
            #[cfg(windows)]
            backbuffer_rt_key: 0,
            #[cfg(windows)]
            backbuffer_rt_info: D3D9PendingRenderTarget::default(),
            render_state,
            texture_stage_states: default_texture_stage_states(),
            display_buffer: Vec::new(),
        }
    }

    #[cfg(windows)]
    fn bool_u32(value: bool) -> u32 {
        if value {
            1
        } else {
            0
        }
    }

    #[cfg(windows)]
    fn map_address_mode(value: u32) -> u32 {
        match value {
            0 | 1 | 2 | 3 | 4 => value,
            5 => 4, // Xbox CLAMPTOEDGE: D3D9 BORDER is Cxbx-R's closest fallback.
            _ => 1,
        }
    }

    #[cfg(windows)]
    fn map_filter(value: u32, mip: bool) -> u32 {
        match value {
            0 if mip => 0,
            0 => 1,
            1 | 2 | 3 => value,
            4 => 3, // QUINCUNX -> ANISOTROPIC
            5 => 7, // GAUSSIANCUBIC -> GAUSSIANQUAD
            _ if mip => 0,
            _ => 1,
        }
    }

    #[cfg(windows)]
    fn map_texcoord_index(value: u32) -> u32 {
        let coord = value & 0x0000_FFFF;
        let coord = if coord > 3 { value & 3 } else { coord };
        match value & 0xFFFF_0000 {
            0x0000_0000 | 0x0001_0000 | 0x0002_0000 | 0x0003_0000 => value,
            0x0004_0000 => coord,
            0x0005_0000 => 0x0004_0000 | coord, // D3DTSS_TCI_SPHEREMAP
            _ => coord,
        }
    }

    #[cfg(windows)]
    unsafe fn apply_texture_stage_state_to_device(
        &self,
        dev: &IDirect3DDevice9,
        stage: u32,
        state: usize,
        value: u32,
    ) {
        match state {
            X_D3DTSS_ADDRESSU => {
                let _ = dev.SetSamplerState(stage, D3DSAMP_ADDRESSU, Self::map_address_mode(value));
            }
            X_D3DTSS_ADDRESSV => {
                let _ = dev.SetSamplerState(stage, D3DSAMP_ADDRESSV, Self::map_address_mode(value));
            }
            X_D3DTSS_ADDRESSW => {
                let _ = dev.SetSamplerState(stage, D3DSAMP_ADDRESSW, Self::map_address_mode(value));
            }
            X_D3DTSS_MAGFILTER => {
                let _ =
                    dev.SetSamplerState(stage, D3DSAMP_MAGFILTER, Self::map_filter(value, false));
            }
            X_D3DTSS_MINFILTER => {
                let _ =
                    dev.SetSamplerState(stage, D3DSAMP_MINFILTER, Self::map_filter(value, false));
            }
            X_D3DTSS_MIPFILTER => {
                let _ =
                    dev.SetSamplerState(stage, D3DSAMP_MIPFILTER, Self::map_filter(value, true));
            }
            X_D3DTSS_COLOROP => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_COLOROP, value);
            }
            X_D3DTSS_COLORARG0 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_COLORARG0, value);
            }
            X_D3DTSS_COLORARG1 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_COLORARG1, value);
            }
            X_D3DTSS_COLORARG2 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_COLORARG2, value);
            }
            X_D3DTSS_ALPHAOP => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_ALPHAOP, value);
            }
            X_D3DTSS_ALPHAARG0 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_ALPHAARG0, value);
            }
            X_D3DTSS_ALPHAARG1 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_ALPHAARG1, value);
            }
            X_D3DTSS_ALPHAARG2 => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_ALPHAARG2, value);
            }
            X_D3DTSS_RESULTARG => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_RESULTARG, value);
            }
            X_D3DTSS_TEXTURETRANSFORMFLAGS => {
                let _ = dev.SetTextureStageState(stage, D3DTSS_TEXTURETRANSFORMFLAGS, value);
            }
            X_D3DTSS_TEXCOORDINDEX => {
                let _ = dev.SetTextureStageState(
                    stage,
                    D3DTSS_TEXCOORDINDEX,
                    Self::map_texcoord_index(value),
                );
            }
            _ => {}
        }
    }

    #[cfg(windows)]
    unsafe fn apply_all_texture_stage_states(&self, dev: &IDirect3DDevice9) {
        for stage in 0..4u32 {
            for state in 0..32usize {
                self.apply_texture_stage_state_to_device(
                    dev,
                    stage,
                    state,
                    self.texture_stage_states[stage as usize][state],
                );
            }
        }
    }

    #[cfg(windows)]
    unsafe fn apply_render_state_to_device(
        &self,
        dev: &IDirect3DDevice9,
        state: Rs,
        raw_value: u32,
    ) {
        let rs = self.render_state;
        match state {
            Rs::ZFunc => {
                let _ = dev.SetRenderState(D3DRS_ZFUNC, rs.z_func as u32);
            }
            Rs::AlphaFunc => {
                let _ = dev.SetRenderState(D3DRS_ALPHAFUNC, rs.alpha_func as u32);
            }
            Rs::AlphaBlendEnable => {
                let _ = dev.SetRenderState(
                    D3DRS_ALPHABLENDENABLE,
                    Self::bool_u32(rs.alpha_blend_enable),
                );
            }
            Rs::AlphaTestEnable => {
                let _ =
                    dev.SetRenderState(D3DRS_ALPHATESTENABLE, Self::bool_u32(rs.alpha_test_enable));
            }
            Rs::AlphaRef => {
                let _ = dev.SetRenderState(D3DRS_ALPHAREF, rs.alpha_ref as u32);
            }
            Rs::SrcBlend => {
                let _ = dev.SetRenderState(D3DRS_SRCBLEND, rs.src_blend as u32);
            }
            Rs::DestBlend => {
                let _ = dev.SetRenderState(D3DRS_DESTBLEND, rs.dest_blend as u32);
            }
            Rs::ZWriteEnable => {
                let _ = dev.SetRenderState(D3DRS_ZWRITEENABLE, Self::bool_u32(rs.z_write_enable));
            }
            Rs::DitherEnable => {
                let _ = dev.SetRenderState(D3DRS_DITHERENABLE, Self::bool_u32(rs.dither_enable));
            }
            Rs::ShadeMode => {
                let _ = dev.SetRenderState(D3DRS_SHADEMODE, rs.shade_mode);
            }
            Rs::ColorWriteEnable => {
                let _ = dev.SetRenderState(D3DRS_COLORWRITEENABLE, rs.color_write_enable);
            }
            Rs::StencilZFail => {
                let _ = dev.SetRenderState(D3DRS_STENCILZFAIL, rs.stencil_zfail);
            }
            Rs::StencilPass => {
                let _ = dev.SetRenderState(D3DRS_STENCILPASS, rs.stencil_pass);
            }
            Rs::StencilFunc => {
                let _ = dev.SetRenderState(D3DRS_STENCILFUNC, rs.stencil_func as u32);
            }
            Rs::StencilRef => {
                let _ = dev.SetRenderState(D3DRS_STENCILREF, rs.stencil_ref as u32);
            }
            Rs::StencilMask => {
                let _ = dev.SetRenderState(D3DRS_STENCILMASK, rs.stencil_mask as u32);
            }
            Rs::StencilWriteMask => {
                let _ = dev.SetRenderState(D3DRS_STENCILWRITEMASK, rs.stencil_write_mask as u32);
            }
            Rs::BlendOp => {
                let _ = dev.SetRenderState(D3DRS_BLENDOP, rs.blend_op);
            }
            Rs::BlendColor => {
                let _ = dev.SetRenderState(D3DRS_BLENDFACTOR, rs.blend_color);
            }
            Rs::FogEnable => {
                let _ = dev.SetRenderState(D3DRS_FOGENABLE, Self::bool_u32(rs.fog_enable));
            }
            Rs::FogTableMode => {
                let _ = dev.SetRenderState(D3DRS_FOGTABLEMODE, raw_value);
            }
            Rs::FogStart => {
                let _ = dev.SetRenderState(D3DRS_FOGSTART, rs.fog_start.to_bits());
            }
            Rs::FogEnd => {
                let _ = dev.SetRenderState(D3DRS_FOGEND, rs.fog_end.to_bits());
            }
            Rs::FogDensity => {
                let _ = dev.SetRenderState(D3DRS_FOGDENSITY, rs.fog_density.to_bits());
            }
            Rs::RangeFogEnable => {
                let _ = dev.SetRenderState(D3DRS_RANGEFOGENABLE, raw_value);
            }
            Rs::Lighting => {
                let _ = dev.SetRenderState(D3DRS_LIGHTING, Self::bool_u32(rs.lighting));
            }
            Rs::SpecularEnable => {
                let _ = dev.SetRenderState(D3DRS_SPECULARENABLE, raw_value);
            }
            Rs::LocalViewer => {
                let _ = dev.SetRenderState(D3DRS_LOCALVIEWER, raw_value);
            }
            Rs::ColorVertex => {
                let _ = dev.SetRenderState(D3DRS_COLORVERTEX, raw_value);
            }
            Rs::Ambient => {
                let _ = dev.SetRenderState(D3DRS_AMBIENT, rs.ambient);
            }
            Rs::FogColor => {
                let _ = dev.SetRenderState(D3DRS_FOGCOLOR, rs.fog_color);
            }
            Rs::FillMode => {
                let _ = dev.SetRenderState(D3DRS_FILLMODE, rs.fill_mode as u32);
            }
            Rs::NormalizeNormals => {
                let _ = dev
                    .SetRenderState(D3DRS_NORMALIZENORMALS, Self::bool_u32(rs.normalize_normals));
            }
            Rs::ZEnable => {
                let _ = dev.SetRenderState(D3DRS_ZENABLE, rs.z_enable);
            }
            Rs::StencilEnable => {
                let _ = dev.SetRenderState(D3DRS_STENCILENABLE, Self::bool_u32(rs.stencil_enable));
            }
            Rs::StencilFail => {
                let _ = dev.SetRenderState(D3DRS_STENCILFAIL, rs.stencil_fail);
            }
            Rs::CullMode => {
                let value = match rs.cull_mode {
                    CullMode::None => D3DCULL_NONE.0 as u32,
                    CullMode::Clockwise => D3DCULL_CW.0 as u32,
                    CullMode::CounterClock => D3DCULL_CCW.0 as u32,
                };
                let _ = dev.SetRenderState(D3DRS_CULLMODE, value);
            }
            Rs::TextureFactor => {
                let _ = dev.SetRenderState(D3DRS_TEXTUREFACTOR, rs.texture_factor);
            }
            Rs::LineWidth => {
                let _ = dev.SetRenderState(D3DRS_POINTSIZE, rs.line_width.to_bits());
            }
            Rs::PointSize
            | Rs::PointSizeMin
            | Rs::PointSizeMax
            | Rs::PointSpriteEnable
            | Rs::PointScaleEnable
            | Rs::PointScaleA
            | Rs::PointScaleB
            | Rs::PointScaleC => {
                let state = match state {
                    Rs::PointSize => D3DRS_POINTSIZE,
                    Rs::PointSizeMin => D3DRS_POINTSIZE_MIN,
                    Rs::PointSizeMax => D3DRS_POINTSIZE_MAX,
                    Rs::PointSpriteEnable => D3DRS_POINTSPRITEENABLE,
                    Rs::PointScaleEnable => D3DRS_POINTSCALEENABLE,
                    Rs::PointScaleA => D3DRS_POINTSCALE_A,
                    Rs::PointScaleB => D3DRS_POINTSCALE_B,
                    Rs::PointScaleC => D3DRS_POINTSCALE_C,
                    _ => unreachable!(),
                };
                let _ = dev.SetRenderState(state, raw_value);
            }
            Rs::ZBias
            | Rs::EdgeAntiAlias
            | Rs::MultiSampleAntiAlias
            | Rs::MultiSampleMask
            | Rs::VertexBlend
            | Rs::Wrap0
            | Rs::Wrap1
            | Rs::Wrap2
            | Rs::Wrap3
            | Rs::PatchSegments
            | Rs::SpecularMaterialSource
            | Rs::DiffuseMaterialSource
            | Rs::AmbientMaterialSource
            | Rs::EmissiveMaterialSource => {}
        }
    }

    #[cfg(windows)]
    fn texture_arg_uses_texture(value: u32) -> bool {
        (value & 0xF) == X_D3DTA_TEXTURE
    }

    #[cfg(windows)]
    fn stage_needs_texture(&self, stage: usize) -> bool {
        let s = self.texture_stage_states[stage];
        let color_op = s[X_D3DTSS_COLOROP];
        let alpha_op = s[X_D3DTSS_ALPHAOP];
        let color_uses_texture = color_op != X_D3DTOP_DISABLE
            && (Self::texture_arg_uses_texture(s[X_D3DTSS_COLORARG0])
                || Self::texture_arg_uses_texture(s[X_D3DTSS_COLORARG1])
                || Self::texture_arg_uses_texture(s[X_D3DTSS_COLORARG2]));
        let alpha_uses_texture = alpha_op != X_D3DTOP_DISABLE
            && (Self::texture_arg_uses_texture(s[X_D3DTSS_ALPHAARG0])
                || Self::texture_arg_uses_texture(s[X_D3DTSS_ALPHAARG1])
                || Self::texture_arg_uses_texture(s[X_D3DTSS_ALPHAARG2]));
        color_uses_texture || alpha_uses_texture
    }

    #[cfg(windows)]
    fn stage_has_texture(&self, stage: usize) -> bool {
        self.textures
            .get(stage)
            .and_then(|texture| texture.as_ref())
            .is_some()
    }

    #[cfg(windows)]
    fn sample_stats(pixels: &[u32]) -> (usize, u32, u32, u32) {
        if pixels.is_empty() {
            return (0, 0, 0, 0);
        }
        let nonblack = pixels.iter().filter(|&&p| (p & 0x00FF_FFFF) != 0).count();
        let first = pixels[0];
        let mid = pixels[pixels.len() / 2];
        let last = *pixels.last().unwrap_or(&0);
        (nonblack, first, mid, last)
    }

    #[cfg(windows)]
    fn normalize_pending_render_target(
        pending: D3D9PendingRenderTarget,
        host_width: i32,
        host_height: i32,
    ) -> Option<D3D9NormalizedRenderTarget> {
        if pending.key == 0 {
            return None;
        }
        let width = pending.width.clamp(1, host_width.max(1) as u32).max(1);
        let height = pending.height.clamp(1, host_height.max(1) as u32).max(1);
        let data = if pending.data != 0 {
            pending.data & !0x3
        } else {
            pending.key & !0x3
        };
        let pitch = if pending.pitch != 0 {
            pending.pitch
        } else {
            width.saturating_mul(4)
        };
        Some(D3D9NormalizedRenderTarget {
            key: pending.key & !0x3,
            width,
            height,
            format: pending.format,
            data,
            pitch,
        })
    }

    #[cfg(windows)]
    fn d3d9_render_target_format(format_code: u32) -> Option<D3DFORMAT> {
        match crate::xbox::gpu::texture_format::format_code(format_code) {
            crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8 => Some(D3DFMT_A8R8G8B8),
            crate::xbox::gpu::texture_format::X_D3DFMT_X8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_X8R8G8B8 => Some(D3DFMT_X8R8G8B8),
            _ => None,
        }
    }

    #[cfg(windows)]
    fn render_target_alias_base(key: u32, surface: &D3D9RenderTargetSurface) -> u32 {
        if surface.data != 0 {
            surface.data & !0x3
        } else {
            key & !0x3
        }
    }

    #[cfg(windows)]
    fn render_target_alias_len(surface: &D3D9RenderTargetSurface) -> u64 {
        let pitch = surface.pitch.max(surface.width.saturating_mul(4)).max(4);
        (pitch as u64).saturating_mul(surface.height.max(1) as u64)
    }

    #[cfg(windows)]
    fn find_render_target_alias_by_data(&self, data_addr: u32) -> Option<(u32, u32, u64)> {
        let addr = data_addr & !0x3;
        if addr == 0 {
            return None;
        }

        let mut best: Option<(u32, u32, u64)> = None;
        for (key, surface) in &self.rt_cache {
            if *key == 0 || *key == self.active_rt_key {
                continue;
            }
            let base = Self::render_target_alias_base(*key, surface);
            let len = Self::render_target_alias_len(surface);
            if addr == base && len != 0 {
                match best {
                    Some((_best_key, _best_base, best_len)) if best_len <= len => {}
                    _ => best = Some((*key, base, len)),
                }
            }
        }
        best
    }

    #[cfg(windows)]
    unsafe fn create_render_target_surface(
        &self,
        key: u32,
        width: u32,
        height: u32,
        format: u32,
        data: u32,
        pitch: u32,
    ) -> Option<D3D9RenderTargetSurface> {
        let dev = self.device.as_ref()?;
        let d3dfmt = match Self::d3d9_render_target_format(format) {
            Some(fmt) => fmt,
            None => {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-RT-UNSUPPORTED] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X}",
                    key, width, height, format, data
                ));
                return None;
            }
        };

        let width = width.clamp(1, 4096);
        let height = height.clamp(1, 4096);

        let mut tex: Option<IDirect3DTexture9> = None;
        let hr = dev.CreateTexture(
            width,
            height,
            1,
            D3DUSAGE_RENDERTARGET as u32,
            d3dfmt,
            D3DPOOL_DEFAULT,
            &mut tex,
            std::ptr::null_mut(),
        );
        if hr.is_err() || tex.is_none() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateTexture {:?}",
                key, width, height, format, data, hr
            ));
            return None;
        }
        let tex = tex.unwrap();

        let surface = match tex.GetSurfaceLevel(0) {
            Ok(surface) => surface,
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=GetSurfaceLevel {:?}",
                    key, width, height, format, data, e
                ));
                return None;
            }
        };
        let mut depth: Option<IDirect3DSurface9> = None;
        let depth_hr = dev.CreateDepthStencilSurface(
            width,
            height,
            D3DFMT_D24S8,
            D3DMULTISAMPLE_NONE,
            0,
            windows::Win32::Foundation::BOOL::from(false),
            &mut depth as *mut Option<IDirect3DSurface9>,
            std::ptr::null_mut(),
        );
        if depth_hr.is_err() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateDepthStencilSurface {:?}",
                key, width, height, format, data, depth_hr
            ));
            depth = None;
        }

        let mut offscreen: Option<IDirect3DSurface9> = None;
        let hr = dev.CreateOffscreenPlainSurface(
            width,
            height,
            d3dfmt,
            D3DPOOL_SYSTEMMEM,
            &mut offscreen as *mut Option<IDirect3DSurface9>,
            std::ptr::null_mut(),
        );
        if hr.is_err() || offscreen.is_none() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateOffscreenPlainSurface {:?}",
                key, width, height, format, data, hr
            ));
            return None;
        }

        Some(D3D9RenderTargetSurface {
            width,
            height,
            format,
            data,
            pitch,
            tex,
            surface,
            depth,
            offscreen: offscreen.unwrap(),
        })
    }

    #[cfg(windows)]
    unsafe fn bind_active_render_target(&self) {
        let Some(dev) = self.device.as_ref() else {
            return;
        };

        let cached = if self.active_rt_key != 0 {
            self.rt_cache.get(&self.active_rt_key)
        } else if self.backbuffer_rt_key != 0 {
            self.rt_cache.get(&self.backbuffer_rt_key)
        } else {
            None
        };

        if let Some(surface) = cached {
            let _ = dev.SetRenderTarget(0, Some(&surface.surface));
            let _ = dev.SetDepthStencilSurface(surface.depth.as_ref());
        } else {
            let _ = dev.SetRenderTarget(0, self.rt.as_ref());
            let _ = dev.SetDepthStencilSurface(self.default_depth.as_ref());
        }
    }

    #[cfg(windows)]
    unsafe fn commit_hle_render_target(&mut self) {
        let key = self.pending_rt.key & !0x3;
        if key == 0 {
            static D3D9_NULL_RT_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = D3D9_NULL_RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-RT] retain color target on NULL key active=0x{:08X}",
                    self.active_rt_key
                ));
            }
            self.bind_active_render_target();
            return;
        }

        self.pending_rt.key = key;
        let Some(normalized) =
            Self::normalize_pending_render_target(self.pending_rt, self.width, self.height)
        else {
            return;
        };

        if self.backbuffer_rt_key == 0 {
            self.backbuffer_rt_key = normalized.key;
        }
        if normalized.key == self.backbuffer_rt_key {
            self.backbuffer_rt_info = D3D9PendingRenderTarget {
                key: normalized.key,
                width: normalized.width,
                height: normalized.height,
                format: normalized.format,
                data: normalized.data,
                pitch: normalized.pitch,
            };

            let expected_width = self.width.max(1) as u32;
            let expected_height = self.height.max(1) as u32;
            let scanout = (if normalized.data != 0 {
                normalized.data
            } else {
                normalized.key
            }) & !0x3;
            if normalized.width == expected_width
                && normalized.height == expected_height
                && scanout != 0
            {
                let old_scanout = crate::xbox::aot::nv2a::shadow_read(0x60_0800);
                if old_scanout != scanout {
                    crate::xbox::aot::nv2a::shadow_write(0x60_0800, scanout);
                    static D3D9_SCANOUT_LINK_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n =
                        D3D9_SCANOUT_LINK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 16 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D9-SCANOUT-LINK] #{} pcrtc_start 0x{:08X}->0x{:08X} backbuffer_key=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
                            n,
                            old_scanout,
                            scanout,
                            normalized.key,
                            normalized.data,
                            normalized.width,
                            normalized.height,
                            normalized.format,
                            normalized.pitch
                        ));
                    }
                }
            }
        }

        let needs_create = self
            .rt_cache
            .get(&normalized.key)
            .map(|surface| {
                surface.width != normalized.width
                    || surface.height != normalized.height
                    || surface.format != normalized.format
            })
            .unwrap_or(true);
        if needs_create {
            if self.rt_cache.len() >= 64 {
                self.rt_cache.clear();
            }
            let Some(surface) = self.create_render_target_surface(
                normalized.key,
                normalized.width,
                normalized.height,
                normalized.format,
                normalized.data,
                normalized.pitch,
            ) else {
                return;
            };
            self.rt_cache.insert(normalized.key, surface);
        }

        self.active_rt_key = normalized.key;
        self.bind_active_render_target();

        static D3D9_RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = D3D9_RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-RT] bind key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} pitch={} cache={} created={} backbuffer=0x{:08X} active=0x{:08X}",
                normalized.key,
                normalized.width,
                normalized.height,
                normalized.format,
                normalized.data,
                normalized.pitch,
                self.rt_cache.len(),
                needs_create,
                self.backbuffer_rt_key,
                self.active_rt_key
            ));
        }
    }

    #[cfg(windows)]
    unsafe fn bind_default_render_target_impl(&mut self, reason: &'static str) {
        if self.backbuffer_rt_key == 0 {
            self.backbuffer_rt_key = DEFAULT_BACKBUFFER_RT_KEY;
        }
        if self.backbuffer_rt_info.key == 0 {
            let width = self.width.max(1) as u32;
            let height = self.height.max(1) as u32;
            self.backbuffer_rt_info = D3D9PendingRenderTarget {
                key: self.backbuffer_rt_key,
                width,
                height,
                format: crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8,
                data: 0,
                pitch: width.saturating_mul(4),
            };
        }
        self.active_rt_key = self.backbuffer_rt_key;
        self.bind_active_render_target();

        static DEFAULT_RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DEFAULT_RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-DEFAULT-RT] #{} reason={} active=0x{:08X} backbuffer=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                n,
                reason,
                self.active_rt_key,
                self.backbuffer_rt_info.key,
                self.backbuffer_rt_info.width,
                self.backbuffer_rt_info.height,
                self.backbuffer_rt_info.data,
                self.backbuffer_rt_info.pitch
            ));
        }
    }

    #[cfg(windows)]
    unsafe fn handle_hle_render_target_control(&mut self, state: u32, value: u32) -> bool {
        match state {
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_KEY => self.pending_rt.key = value,
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_SIZE => {
                self.pending_rt.width = value & 0xFFFF;
                self.pending_rt.height = (value >> 16) & 0xFFFF;
            }
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_FORMAT => self.pending_rt.format = value,
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_DATA => self.pending_rt.data = value,
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_PITCH => self.pending_rt.pitch = value,
            crate::xbox::gpu::d3d11::D3D11_HLE_RT_COMMIT => self.commit_hle_render_target(),
            _ => return false,
        }
        true
    }

    #[cfg(windows)]
    unsafe fn readback_surfaces(
        &mut self,
        rt: &IDirect3DSurface9,
        offscreen: &IDirect3DSurface9,
        rt_width: u32,
        rt_height: u32,
        active_rt_key: u32,
        source: &'static str,
    ) -> &[u32] {
        let dev = match self.device.as_ref() {
            Some(d) => d,
            None => return &self.display_buffer,
        };

        let hr = dev.GetRenderTargetData(rt, offscreen);
        if hr.is_err() {
            return &self.display_buffer;
        }

        let mut locked = D3DLOCKED_RECT::default();
        let hr = offscreen.LockRect(&mut locked, std::ptr::null(), D3DLOCK_READONLY as u32);
        if hr.is_err() || locked.pBits.is_null() || locked.Pitch <= 0 {
            return &self.display_buffer;
        }

        let out_w = self.width.max(0) as usize;
        let out_h = self.height.max(0) as usize;
        if out_w == 0 || out_h == 0 {
            let _ = offscreen.UnlockRect();
            return &self.display_buffer;
        }
        let needed = out_w.saturating_mul(out_h);
        if self.display_buffer.len() != needed {
            self.display_buffer.resize(needed, 0xFF00_0000);
        }
        self.display_buffer.fill(0xFF00_0000);

        let pitch = locked.Pitch as usize;
        let src = locked.pBits as *const u8;
        let copy_w = (rt_width as usize).min(out_w);
        let copy_h = (rt_height as usize).min(out_h);
        for y in 0..copy_h {
            let row = std::slice::from_raw_parts(src.add(y * pitch) as *const u32, copy_w);
            self.display_buffer[y * out_w..y * out_w + copy_w].copy_from_slice(row);
        }

        let _ = offscreen.UnlockRect();

        static D3D9_READBACK_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = D3D9_READBACK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            let (nonblack, first, mid, last) = Self::sample_stats(&self.display_buffer);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-READBACK] #{} source={} key=0x{:08X} rt={}x{} out={}x{} nonblack={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X}",
                n,
                source,
                active_rt_key,
                rt_width,
                rt_height,
                out_w,
                out_h,
                nonblack,
                self.display_buffer.len(),
                first,
                mid,
                last
            ));
        }

        &self.display_buffer
    }

    #[cfg(windows)]
    fn cached_readback_resources(
        &self,
        key: u32,
    ) -> Option<(IDirect3DSurface9, IDirect3DSurface9, u32, u32)> {
        let surface = self.rt_cache.get(&key)?;
        Some((
            surface.surface.clone(),
            surface.offscreen.clone(),
            surface.width,
            surface.height,
        ))
    }

    #[cfg(windows)]
    unsafe fn ensure_scanout_render_target(&mut self, key: u32, width: u32, height: u32) -> bool {
        let key = key & !0x3;
        if key == 0 {
            return false;
        }
        if self.rt_cache.contains_key(&key) {
            return true;
        }
        let width = width.clamp(1, self.width.max(1) as u32).max(1);
        let height = height.clamp(1, self.height.max(1) as u32).max(1);
        let Some(surface) = self.create_render_target_surface(
            key,
            width,
            height,
            crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8,
            key,
            width.saturating_mul(4),
        ) else {
            return false;
        };
        self.rt_cache.insert(key, surface);
        true
    }

    #[cfg(windows)]
    fn d3d9_compressed_format(format_code: u32) -> Option<D3DFORMAT> {
        match format_code {
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT1 => Some(D3DFMT_DXT1),
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT3 => Some(D3DFMT_DXT3),
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT5 => Some(D3DFMT_DXT5),
            _ => None,
        }
    }

    #[cfg(windows)]
    fn d3d9_uncompressed_native_format(format_code: u32) -> Option<D3DFORMAT> {
        match format_code {
            crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8 => Some(D3DFMT_A8R8G8B8),
            crate::xbox::gpu::texture_format::X_D3DFMT_X8R8G8B8
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_X8R8G8B8 => Some(D3DFMT_X8R8G8B8),
            crate::xbox::gpu::texture_format::X_D3DFMT_R5G6B5
            | crate::xbox::gpu::texture_format::X_D3DFMT_LIN_R5G6B5 => Some(D3DFMT_R5G6B5),
            _ => None,
        }
    }

    #[cfg(windows)]
    unsafe fn upload_native_texture_bytes(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        format_code: u32,
        d3dfmt: D3DFORMAT,
        bytes: &[u8],
        row_bytes: usize,
        row_count: usize,
        path: &str,
    ) -> bool {
        let Some(dev) = self.device.as_ref().cloned() else {
            return false;
        };
        if stage >= 4 || width == 0 || height == 0 || row_bytes == 0 || row_count == 0 {
            return false;
        }
        let expected = match row_bytes.checked_mul(row_count) {
            Some(v) => v,
            None => return false,
        };
        if bytes.len() < expected {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-TEX-RAW] short upload stage={} fmt=0x{:02X}/{} {}x{} bytes={} expected={} path={}",
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                bytes.len(),
                expected,
                path
            ));
            return false;
        }

        let mut tex: Option<IDirect3DTexture9> = None;
        let hr = dev.CreateTexture(
            width,
            height,
            1,
            0,
            d3dfmt,
            D3DPOOL_MANAGED,
            &mut tex,
            std::ptr::null_mut(),
        );
        if hr.is_err() || tex.is_none() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-TEX-RAW] CreateTexture failed stage={} fmt=0x{:02X}/{} {}x{} hr={:?} path={}",
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                hr,
                path
            ));
            return false;
        }
        let tex = tex.unwrap();

        let mut locked = D3DLOCKED_RECT::default();
        let hr = tex.LockRect(0, &mut locked, std::ptr::null(), 0);
        if hr.is_err() || locked.pBits.is_null() || locked.Pitch <= 0 {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-TEX-RAW] LockRect failed stage={} fmt=0x{:02X}/{} {}x{} hr={:?} pitch={} path={}",
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                hr,
                locked.Pitch,
                path
            ));
            return false;
        }

        let dst_pitch = locked.Pitch as usize;
        if dst_pitch < row_bytes {
            let _ = tex.UnlockRect(0);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-TEX-RAW] pitch too small stage={} fmt=0x{:02X}/{} pitch={} row_bytes={} path={}",
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                dst_pitch,
                row_bytes,
                path
            ));
            return false;
        }

        let dst_base = locked.pBits as *mut u8;
        for row in 0..row_count {
            let src = bytes.as_ptr().add(row * row_bytes);
            let dst = dst_base.add(row * dst_pitch);
            std::ptr::copy_nonoverlapping(src, dst, row_bytes);
        }
        let _ = tex.UnlockRect(0);

        let bind_hr = dev.SetTexture(stage, &tex);
        let index = stage as usize;
        if self.textures.len() <= index {
            self.textures.resize_with(index + 1, || None);
        }
        self.textures[index] = Some(tex);
        for state in 0..32usize {
            self.apply_texture_stage_state_to_device(
                &dev,
                stage,
                state,
                self.texture_stage_states[index][state],
            );
        }

        static D3D9_RAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = D3D9_RAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() || bind_hr.is_err() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D9-TEX-RAW] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} expected={} row_bytes={} rows={} pitch={} path={} bind={:?}",
                n,
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                bytes.len(),
                expected,
                row_bytes,
                row_count,
                dst_pitch,
                path,
                bind_hr
            ));
        }
        true
    }
}

#[cfg(windows)]
impl GpuBackend for D3D9Backend {
    fn init(&mut self, width: i32, height: i32) -> bool {
        self.width = width;
        self.height = height;
        self.display_buffer = vec![0u32; (width * height) as usize];

        unsafe {
            // Create D3D9 object
            let d3d9 = match Direct3DCreate9(D3D_SDK_VERSION) {
                Some(d) => d,
                None => {
                    crate::xbox::emulator::debug_log("[D3D9] Direct3DCreate9 failed");
                    return false;
                }
            };

            // Present parameters — offscreen device with auto depth stencil
            let mut pp = D3DPRESENT_PARAMETERS {
                BackBufferWidth: width as u32,
                BackBufferHeight: height as u32,
                BackBufferFormat: D3DFMT_X8R8G8B8,
                BackBufferCount: 1,
                SwapEffect: D3DSWAPEFFECT_DISCARD,
                Windowed: true.into(),
                hDeviceWindow: HWND::default(),
                EnableAutoDepthStencil: true.into(),
                AutoDepthStencilFormat: D3DFMT_D24S8,
                ..Default::default()
            };

            // Create device — software vertex processing for compatibility
            let mut device: Option<IDirect3DDevice9> = None;
            let hr = d3d9.CreateDevice(
                0, // D3DADAPTER_DEFAULT
                D3DDEVTYPE_HAL,
                HWND::default(),
                D3DCREATE_SOFTWARE_VERTEXPROCESSING as u32,
                &mut pp,
                &mut device,
            );

            if hr.is_err() || device.is_none() {
                crate::xbox::emulator::debug_log(&format!("[D3D9] CreateDevice failed: {:?}", hr));
                return false;
            }

            let dev = device.unwrap();

            // Create offscreen render target
            let mut rt: Option<IDirect3DSurface9> = None;
            let hr = dev.CreateRenderTarget(
                width as u32,
                height as u32,
                D3DFMT_X8R8G8B8,
                D3DMULTISAMPLE_NONE,
                0,
                windows::Win32::Foundation::BOOL::from(false),
                &mut rt as *mut Option<IDirect3DSurface9>,
                std::ptr::null_mut(),
            );
            if hr.is_err() || rt.is_none() {
                crate::xbox::emulator::debug_log("[D3D9] CreateRenderTarget failed");
                return false;
            }

            // Create offscreen plain surface for readback (lockable)
            let mut offscreen: Option<IDirect3DSurface9> = None;
            let hr = dev.CreateOffscreenPlainSurface(
                width as u32,
                height as u32,
                D3DFMT_X8R8G8B8,
                D3DPOOL_SYSTEMMEM,
                &mut offscreen as *mut Option<IDirect3DSurface9>,
                std::ptr::null_mut(),
            );
            if hr.is_err() || offscreen.is_none() {
                crate::xbox::emulator::debug_log("[D3D9] CreateOffscreenPlainSurface failed");
                return false;
            }

            // Set render target
            let _ = dev.SetRenderTarget(0, rt.as_ref());
            let default_depth = dev.GetDepthStencilSurface().ok();

            // Set basic render state
            let _ = dev.SetRenderState(D3DRS_LIGHTING, Self::bool_u32(self.render_state.lighting));
            let _ = dev.SetRenderState(D3DRS_ZENABLE, self.render_state.z_enable);
            let _ = dev.SetRenderState(
                D3DRS_ZWRITEENABLE,
                Self::bool_u32(self.render_state.z_write_enable),
            );
            let _ = dev.SetRenderState(D3DRS_ZFUNC, self.render_state.z_func as u32);
            let _ = dev.SetRenderState(
                D3DRS_ALPHABLENDENABLE,
                Self::bool_u32(self.render_state.alpha_blend_enable),
            );
            let _ = dev.SetRenderState(
                D3DRS_ALPHATESTENABLE,
                Self::bool_u32(self.render_state.alpha_test_enable),
            );
            let _ = dev.SetRenderState(D3DRS_SRCBLEND, self.render_state.src_blend as u32);
            let _ = dev.SetRenderState(D3DRS_DESTBLEND, self.render_state.dest_blend as u32);
            let _ = dev.SetRenderState(D3DRS_CULLMODE, D3DCULL_CCW.0 as u32);
            let _ = dev.SetRenderState(D3DRS_FILLMODE, FillMode::Solid as u32);
            let _ = dev.SetRenderState(D3DRS_SHADEMODE, self.render_state.shade_mode);
            let _ =
                dev.SetRenderState(D3DRS_COLORWRITEENABLE, self.render_state.color_write_enable);
            let _ = dev.SetFVF(D3D9_FVF);
            self.apply_all_texture_stage_states(&dev);

            self.d3d9 = Some(d3d9);
            self.rt = rt;
            self.default_depth = default_depth;
            self.offscreen = offscreen;
            self.device = Some(dev);

            crate::xbox::emulator::debug_log(&format!(
                "[D3D9] Backend initialized ({}x{})",
                width, height
            ));
            true
        }
    }

    fn shutdown(&mut self) {
        self.textures.clear();
        self.rt_cache.clear();
        self.pending_rt = D3D9PendingRenderTarget::default();
        self.active_rt_key = 0;
        self.backbuffer_rt_key = 0;
        self.backbuffer_rt_info = D3D9PendingRenderTarget::default();
        self.rt = None;
        self.default_depth = None;
        self.offscreen = None;
        self.device = None;
        self.d3d9 = None;
    }

    fn draw_primitive(&mut self, verts: &[NV2AVertex], prim_type: i32) {
        #[cfg(windows)]
        unsafe {
            let dev = match self.device.as_ref() {
                Some(d) => d,
                None => return,
            };

            // Convert NV2AVertex → D3D9Vertex (pre-transformed screen space)
            let d3d9_verts: Vec<D3D9Vertex> = verts
                .iter()
                .map(|v| D3D9Vertex {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    rhw: 1.0, // screen space, no perspective
                    color: v.color,
                    u: v.u,
                    v: v.v,
                })
                .collect();

            let prim = map_prim_type(prim_type);
            let prim_count = match prim_type {
                5 => verts.len() / 3,               // triangles
                6 => verts.len().saturating_sub(2), // strip
                7 => verts.len().saturating_sub(2), // fan
                2 => verts.len() / 2,               // lines
                _ => verts.len(),                   // points
            };
            if prim_count == 0 {
                return;
            }
            if self.stage_needs_texture(0) && !self.stage_has_texture(0) {
                static D3D9_SKIP_NOTEX_DRAWS: std::sync::OnceLock<bool> =
                    std::sync::OnceLock::new();
                static D3D9_NOTEX_DRAWS: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let skip_notex_draws = *D3D9_SKIP_NOTEX_DRAWS.get_or_init(|| {
                    std::env::var("RUSTEMU_D3D9_SKIP_NOTEX")
                        .map(|v| {
                            matches!(
                                v.as_str(),
                                "1" | "true" | "TRUE" | "on" | "ON" | "yes" | "YES"
                            )
                        })
                        .unwrap_or(false)
                });
                let n = D3D9_NOTEX_DRAWS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 || n.is_power_of_two() {
                    let s = self.texture_stage_states[0];
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-DRAW-NOTEX] #{} action={} verts={} prim={} colorop={}({},{},{}) alphaop={}({},{},{})",
                        n,
                        if skip_notex_draws { "skip" } else { "draw" },
                        verts.len(),
                        prim_type,
                        s[X_D3DTSS_COLOROP],
                        s[X_D3DTSS_COLORARG0],
                        s[X_D3DTSS_COLORARG1],
                        s[X_D3DTSS_COLORARG2],
                        s[X_D3DTSS_ALPHAOP],
                        s[X_D3DTSS_ALPHAARG0],
                        s[X_D3DTSS_ALPHAARG1],
                        s[X_D3DTSS_ALPHAARG2],
                    ));
                }
                if skip_notex_draws {
                    return;
                }
            }

            // BeginScene wraps draw only — Clear is called separately before this
            let _ = dev.BeginScene();
            let _ = dev.DrawPrimitiveUP(
                prim,
                prim_count as u32,
                d3d9_verts.as_ptr() as *const _,
                std::mem::size_of::<D3D9Vertex>() as u32,
            );
            let _ = dev.EndScene();
        }
    }

    fn clear(&mut self, color: u32) {
        #[cfg(windows)]
        unsafe {
            if let Some(ref dev) = self.device {
                let _ = dev.Clear(
                    0,
                    std::ptr::null(),
                    (D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER) as u32,
                    color,
                    1.0,
                    0,
                );
            }
        }
    }

    fn set_texture(&mut self, stage: u32, width: u32, height: u32, pixels: &[u32]) {
        #[cfg(windows)]
        unsafe {
            if stage >= 4 {
                return;
            }
            let dev = match self.device.as_ref() {
                Some(d) => d,
                None => return,
            };

            // Create a new D3D9 texture
            let mut tex: Option<IDirect3DTexture9> = None;
            let hr = dev.CreateTexture(
                width,
                height,
                1,
                0,
                D3DFMT_A8R8G8B8,
                D3DPOOL_MANAGED,
                &mut tex,
                std::ptr::null_mut(),
            );
            if hr.is_err() || tex.is_none() {
                return;
            }
            let tex = tex.unwrap();

            // Lock and copy pixel data
            let mut locked = D3DLOCKED_RECT::default();
            if tex.LockRect(0, &mut locked, std::ptr::null(), 0).is_ok() {
                let pitch = locked.Pitch as u32;
                let dst = locked.pBits as *mut u8;
                for row in 0..height {
                    let src_offset = (row * width) as usize;
                    let dst_offset = (row * pitch) as usize;
                    let row_bytes = (width * 4) as usize;
                    if src_offset + width as usize <= pixels.len() {
                        std::ptr::copy_nonoverlapping(
                            pixels[src_offset..].as_ptr() as *const u8,
                            dst.add(dst_offset),
                            row_bytes,
                        );
                    }
                }
                let _ = tex.UnlockRect(0);
            }

            // Bind texture on the device
            let _ = dev.SetTexture(stage, &tex);
            let index = stage as usize;
            if self.textures.len() <= index {
                self.textures.resize_with(index + 1, || None);
            }
            self.textures[index] = Some(tex);
            for state in 0..32usize {
                self.apply_texture_stage_state_to_device(
                    dev,
                    stage,
                    state,
                    self.texture_stage_states[stage as usize][state],
                );
            }

            crate::xbox::emulator::debug_log(&format!(
                "[D3D9] Texture uploaded: {}x{} stage={} px0=0x{:08X} mid=0x{:08X}",
                width,
                height,
                stage,
                pixels.first().copied().unwrap_or(0),
                pixels
                    .get(pixels.len().saturating_div(2))
                    .copied()
                    .unwrap_or(0)
            ));
        }
    }

    fn set_texture_raw(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        format_code: u32,
        bytes: &[u8],
    ) {
        #[cfg(windows)]
        unsafe {
            if stage >= 4 || width == 0 || height == 0 || bytes.is_empty() {
                return;
            }

            let format_code = crate::xbox::gpu::texture_format::format_code(format_code);
            if crate::xbox::gpu::texture_format::is_depth_format(format_code) {
                static D3D9_DEPTH_TEX_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = D3D9_DEPTH_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-TEX-RAW-DEPTH] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={}",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len()
                    ));
                }
                return;
            }

            if crate::xbox::gpu::texture_format::is_block_compressed(format_code) {
                let Some(d3dfmt) = Self::d3d9_compressed_format(format_code) else {
                    return;
                };
                let Some(block_bytes) = crate::xbox::gpu::texture_format::block_bytes(format_code)
                else {
                    return;
                };
                let row_bytes = ((width as usize + 3) / 4) * block_bytes as usize;
                let row_count = (height as usize + 3) / 4;
                if self.upload_native_texture_bytes(
                    stage,
                    width,
                    height,
                    format_code,
                    d3dfmt,
                    bytes,
                    row_bytes,
                    row_count,
                    "bc-native",
                ) {
                    return;
                }

                if let Some(pixels) =
                    crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                        format_code,
                        width,
                        height,
                        bytes,
                    )
                {
                    self.set_texture(stage, width, height, &pixels);
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-TEX-RAW] bc fallback decoded stage={} fmt=0x{:02X}/{} {}x{} bytes={}",
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len()
                    ));
                }
                return;
            }

            let bpp = match crate::xbox::gpu::texture_format::bytes_per_pixel(format_code) {
                Some(v) if v != 0 => v,
                _ => {
                    static D3D9_UNSUPPORTED_TEX_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n =
                        D3D9_UNSUPPORTED_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D9-TEX-RAW-UNSUPPORTED] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={}",
                            n,
                            stage,
                            format_code,
                            crate::xbox::gpu::texture_format::format_name(format_code),
                            width,
                            height,
                            bytes.len()
                        ));
                    }
                    return;
                }
            };

            let unswizzled: Vec<u8>;
            let swizzled = crate::xbox::gpu::texture_format::is_swizzled(format_code);
            let (upload_bytes, path): (&[u8], &str) = if swizzled {
                let expected = match (width as usize)
                    .checked_mul(height as usize)
                    .and_then(|px| px.checked_mul(bpp as usize))
                {
                    Some(v) => v,
                    None => return,
                };
                if bytes.len() < expected {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-TEX-RAW] short swizzled upload stage={} fmt=0x{:02X}/{} {}x{} bpp={} bytes={} expected={}",
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bpp,
                        bytes.len(),
                        expected
                    ));
                    return;
                }
                match crate::xbox::gpu::swizzle::unswizzle(&bytes[..expected], width, height, bpp) {
                    Some(v) => {
                        unswizzled = v;
                        (&unswizzled, "unswizzled")
                    }
                    None => {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D9-TEX-RAW] unswizzle failed; trying linear stage={} fmt=0x{:02X}/{} {}x{} bpp={} bytes={}",
                            stage,
                            format_code,
                            crate::xbox::gpu::texture_format::format_name(format_code),
                            width,
                            height,
                            bpp,
                            bytes.len()
                        ));
                        (bytes, "linear-fallback")
                    }
                }
            } else {
                (bytes, "linear")
            };

            if let Some(d3dfmt) = Self::d3d9_uncompressed_native_format(format_code) {
                let row_bytes =
                    crate::xbox::gpu::texture_format::texture_pitch_bytes(width, format_code)
                        as usize;
                if self.upload_native_texture_bytes(
                    stage,
                    width,
                    height,
                    format_code,
                    d3dfmt,
                    upload_bytes,
                    row_bytes,
                    height as usize,
                    path,
                ) {
                    return;
                }
            }

            if let Some(pixels) = crate::xbox::gpu::texture_format::decode_linear_to_argb(
                format_code,
                width,
                height,
                upload_bytes,
                None,
            ) {
                let sample = pixels.first().copied().unwrap_or(0);
                let mid = pixels
                    .get(pixels.len().saturating_div(2))
                    .copied()
                    .unwrap_or(0);
                self.set_texture(stage, width, height, &pixels);

                static D3D9_RAW_DECODE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = D3D9_RAW_DECODE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-TEX-RAW] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} path={} px0=0x{:08X} mid=0x{:08X}",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len(),
                        path,
                        sample,
                        mid
                    ));
                }
            } else {
                static D3D9_UNSUPPORTED_TEX_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = D3D9_UNSUPPORTED_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D9-TEX-RAW-UNSUPPORTED] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} path={}",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len(),
                        path
                    ));
                }
            }
        }
        #[cfg(not(windows))]
        {
            let _ = (stage, width, height, format_code, bytes);
        }
    }

    fn clear_texture(&mut self, stage: u32) {
        #[cfg(windows)]
        unsafe {
            if stage >= 4 {
                return;
            }
            if let Some(ref dev) = self.device {
                let _ = dev.SetTexture(stage, None);
            }
            if let Some(slot) = self.textures.get_mut(stage as usize) {
                *slot = None;
            }
        }
    }

    fn set_render_state(&mut self, state: u32, value: u32) {
        #[cfg(windows)]
        unsafe {
            if self.handle_hle_render_target_control(state, value) {
                return;
            }
            let (state, value) = normalize_render_state_input(state, value);
            let Some(rs) = Rs::from_u32(state) else {
                return;
            };
            self.render_state.apply(state, value);
            if let Some(ref dev) = self.device {
                self.apply_render_state_to_device(dev, rs, value);
            }

            static D3D9_RS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = D3D9_RS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-RS] #{} state={} value=0x{:08X}",
                    n, state, value
                ));
            }
        }
        #[cfg(not(windows))]
        {
            let _ = (state, value);
        }
    }

    fn set_texture_stage_state(&mut self, stage: u32, state: u32, value: u32) {
        #[cfg(windows)]
        unsafe {
            if stage >= 4 || state >= 32 {
                return;
            }
            self.texture_stage_states[stage as usize][state as usize] = value;
            if let Some(ref dev) = self.device {
                self.apply_texture_stage_state_to_device(dev, stage, state as usize, value);
            }

            static D3D9_TSS_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = D3D9_TSS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-TSS] #{} stage={} state={} value=0x{:08X}",
                    n, stage, state, value
                ));
            }
        }
        #[cfg(not(windows))]
        {
            let _ = (stage, state, value);
        }
    }

    fn set_viewport(&mut self, x: u32, y: u32, w: u32, h: u32, min_z: f32, max_z: f32) {
        #[cfg(windows)]
        unsafe {
            let Some(dev) = self.device.as_ref() else {
                return;
            };

            let out_w = self.width.max(1) as u32;
            let out_h = self.height.max(1) as u32;
            let clamped_x = x.min(out_w.saturating_sub(1));
            let clamped_y = y.min(out_h.saturating_sub(1));
            let clamped_w = w.min(out_w.saturating_sub(clamped_x)).max(1);
            let clamped_h = h.min(out_h.saturating_sub(clamped_y)).max(1);
            let vp = D3DVIEWPORT9 {
                X: clamped_x,
                Y: clamped_y,
                Width: clamped_w,
                Height: clamped_h,
                MinZ: min_z.clamp(0.0, 1.0),
                MaxZ: max_z.clamp(0.0, 1.0),
            };
            let hr = dev.SetViewport(&vp);

            static D3D9_VIEWPORT_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = D3D9_VIEWPORT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() || hr.is_err() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-VIEWPORT] #{} req={}x{}+{}+{} z=[{:.3},{:.3}] set={}x{}+{}+{} z=[{:.3},{:.3}] hr={:?}",
                    n,
                    w,
                    h,
                    x,
                    y,
                    min_z,
                    max_z,
                    vp.Width,
                    vp.Height,
                    vp.X,
                    vp.Y,
                    vp.MinZ,
                    vp.MaxZ,
                    hr
                ));
            }
        }
        #[cfg(not(windows))]
        {
            let _ = (x, y, w, h, min_z, max_z);
        }
    }

    fn readback_framebuffer(&mut self) -> &[u32] {
        #[cfg(windows)]
        unsafe {
            let active_key = self.active_rt_key;
            if active_key != 0 {
                if let Some((rt, offscreen, width, height)) =
                    self.cached_readback_resources(active_key)
                {
                    return self
                        .readback_surfaces(&rt, &offscreen, width, height, active_key, "active");
                }
            }
            let backbuffer_key = self.backbuffer_rt_key;
            if backbuffer_key != 0 {
                if let Some((rt, offscreen, width, height)) =
                    self.cached_readback_resources(backbuffer_key)
                {
                    return self.readback_surfaces(
                        &rt,
                        &offscreen,
                        width,
                        height,
                        backbuffer_key,
                        "backbuffer",
                    );
                }
            }

            let rt = match self.rt.as_ref() {
                Some(r) => r.clone(),
                None => return &self.display_buffer,
            };

            let offscreen = match self.offscreen.as_ref() {
                Some(o) => o.clone(),
                None => return &self.display_buffer,
            };

            return self.readback_surfaces(
                &rt,
                &offscreen,
                self.width.max(0) as u32,
                self.height.max(0) as u32,
                0,
                "default",
            );
        }
        #[cfg(not(windows))]
        {
            &self.display_buffer
        }
    }

    fn bind_render_target_texture(&mut self, stage: u32, key: u32) -> bool {
        #[cfg(windows)]
        unsafe {
            if stage >= 4 {
                return false;
            }
            let key = key & !0x3;
            if key == 0 || key == self.active_rt_key {
                return false;
            }
            let Some(dev) = self.device.as_ref().cloned() else {
                return false;
            };
            let Some(surface) = self.rt_cache.get(&key) else {
                return false;
            };
            let tex = surface.tex.clone();
            let width = surface.width;
            let height = surface.height;
            let format = surface.format;
            let data = surface.data;
            let pitch = surface.pitch;
            let bind_hr = dev.SetTexture(stage, &tex);
            if bind_hr.is_err() {
                return false;
            }
            let index = stage as usize;
            if self.textures.len() <= index {
                self.textures.resize_with(index + 1, || None);
            }
            self.textures[index] = Some(tex);
            for state in 0..32usize {
                self.apply_texture_stage_state_to_device(
                    &dev,
                    stage,
                    state,
                    self.texture_stage_states[index][state],
                );
            }

            static D3D9_RT_TEX_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = D3D9_RT_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D9-RT-TEX] #{} stage={} key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} pitch={} active_rt=0x{:08X}",
                    n,
                    stage,
                    key,
                    width,
                    height,
                    format,
                    data,
                    pitch,
                    self.active_rt_key
                ));
            }
            true
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
            let Some((key, _base, _len)) = self.find_render_target_alias_by_data(data_addr) else {
                return false;
            };
            self.bind_render_target_texture(stage, key)
        }
        #[cfg(not(windows))]
        {
            let _ = (stage, data_addr);
            false
        }
    }

    fn resolve_scanout(&mut self, pcrtc_start: u32) -> bool {
        #[cfg(windows)]
        unsafe {
            let scanout_key = pcrtc_start & !0x3;
            if scanout_key == 0 {
                return false;
            }
            if self.rt_cache.contains_key(&scanout_key) {
                return true;
            }
            let (width, height) = if self.backbuffer_rt_info.key != 0 {
                (
                    self.backbuffer_rt_info.width,
                    self.backbuffer_rt_info.height,
                )
            } else {
                (self.width.max(1) as u32, self.height.max(1) as u32)
            };
            self.ensure_scanout_render_target(scanout_key, width, height)
        }
        #[cfg(not(windows))]
        {
            let _ = pcrtc_start;
            false
        }
    }

    fn copy_backbuffer_to_display(&mut self, pcrtc_start: u32) -> bool {
        self.resolve_scanout(pcrtc_start)
    }

    fn bind_display_as_render_target(&mut self, pcrtc_start: u32) -> bool {
        #[cfg(windows)]
        unsafe {
            let scanout_key = pcrtc_start & !0x3;
            if scanout_key == 0 || !self.rt_cache.contains_key(&scanout_key) {
                return false;
            }
            self.active_rt_key = scanout_key;
            self.bind_active_render_target();
            true
        }
        #[cfg(not(windows))]
        {
            let _ = pcrtc_start;
            false
        }
    }

    fn restore_backbuffer_render_target(&mut self) {
        #[cfg(windows)]
        unsafe {
            self.bind_default_render_target_impl("restore");
        }
    }

    fn bind_default_render_target(&mut self) {
        #[cfg(windows)]
        unsafe {
            self.bind_default_render_target_impl("default");
        }
    }

    fn present_display(&mut self, pcrtc_start: u32) -> &[u32] {
        #[cfg(windows)]
        unsafe {
            let scanout_key = pcrtc_start & !0x3;
            if scanout_key != 0 {
                if let Some((rt, offscreen, width, height)) =
                    self.cached_readback_resources(scanout_key)
                {
                    return self.readback_surfaces(
                        &rt,
                        &offscreen,
                        width,
                        height,
                        scanout_key,
                        "scanout",
                    );
                }
            }
        }
        self.present_framebuffer()
    }

    fn present_framebuffer(&mut self) -> &[u32] {
        #[cfg(windows)]
        unsafe {
            let pcrtc_start = crate::xbox::aot::nv2a::shadow_read(0x60_0800) & !0x3;
            if pcrtc_start != 0 {
                if let Some((rt, offscreen, width, height)) =
                    self.cached_readback_resources(pcrtc_start)
                {
                    return self.readback_surfaces(
                        &rt,
                        &offscreen,
                        width,
                        height,
                        pcrtc_start,
                        "scanout",
                    );
                }
            }
        }
        self.readback_framebuffer()
    }

    fn debug_active_render_target(&self) -> Option<(u32, u32, u32, u32, u32)> {
        #[cfg(windows)]
        {
            if self.active_rt_key != 0 {
                if let Some(surface) = self.rt_cache.get(&self.active_rt_key) {
                    return Some((
                        self.active_rt_key,
                        surface.width,
                        surface.height,
                        surface.data,
                        surface.pitch,
                    ));
                }
            }
            if self.backbuffer_rt_info.key != 0 {
                Some((
                    self.backbuffer_rt_info.key,
                    self.backbuffer_rt_info.width,
                    self.backbuffer_rt_info.height,
                    self.backbuffer_rt_info.data,
                    self.backbuffer_rt_info.pitch,
                ))
            } else {
                Some((
                    0,
                    self.width.max(0) as u32,
                    self.height.max(0) as u32,
                    0,
                    self.width.max(0) as u32 * 4,
                ))
            }
        }
        #[cfg(not(windows))]
        {
            None
        }
    }

    fn name(&self) -> &'static str {
        "D3D9"
    }
}

#[cfg(not(windows))]
impl GpuBackend for D3D9Backend {
    fn init(&mut self, width: i32, height: i32) -> bool {
        false
    }
    fn shutdown(&mut self) {}
    fn draw_primitive(&mut self, _verts: &[NV2AVertex], _prim_type: i32) {}
    fn clear(&mut self, _color: u32) {}
    fn readback_framebuffer(&mut self) -> &[u32] {
        &self.display_buffer
    }
    fn name(&self) -> &'static str {
        "D3D9-stub"
    }
}
