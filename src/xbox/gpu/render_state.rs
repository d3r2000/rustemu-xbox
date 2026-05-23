//! D3D8 render-state table and host translation.
//!
//! Ported from Cxbx-Reloaded's `XbD3D8Types.h` X_D3DRENDERSTATETYPE enum
//! (lines 698-872) and the translation logic in `XbState.cpp`.
//!
//! **Scope:** Only the "simple" + "deferred" + "complex" render states used
//! by the D3D8 fixed-function pipeline. Pixel-shader combiner states
//! (X_D3DRS_PSALPHAINPUTS0..X_D3DRS_PSINPUTTEXTURE, states 0-56) are left
//! out — those need NV2A register-combiner HLSL translation, which is a
//! separate port (XbPixelShader.cpp).
//!
//! Zero dependency on emulator or GPU backend. Pure enum + table + state
//! struct with unit tests.

/// Xbox D3D render-state slot numbers. Matches `X_D3DRENDERSTATETYPE` in
/// Cxbx-Reloaded XbD3D8Types.h:698-872 for the simple/deferred/complex
/// ranges we care about for fixed-function rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum XboxRenderState {
    // ---- "Simple" range (57..91) — small state values, set directly ----
    ZFunc = 57,            // D3DCMPFUNC
    AlphaFunc = 58,        // D3DCMPFUNC
    AlphaBlendEnable = 59, // TRUE/FALSE
    AlphaTestEnable = 60,  // TRUE/FALSE
    AlphaRef = 61,         // BYTE
    SrcBlend = 62,         // D3DBLEND
    DestBlend = 63,        // D3DBLEND
    ZWriteEnable = 64,     // TRUE/FALSE
    DitherEnable = 65,     // TRUE/FALSE
    ShadeMode = 66,        // D3DSHADEMODE
    ColorWriteEnable = 67, // per-channel write mask
    StencilZFail = 68,     // D3DSTENCILOP
    StencilPass = 69,      // D3DSTENCILOP
    StencilFunc = 70,      // D3DCMPFUNC
    StencilRef = 71,       // BYTE
    StencilMask = 72,      // BYTE
    StencilWriteMask = 73, // BYTE
    BlendOp = 74,          // D3DBLENDOP
    BlendColor = 75,       // D3DCOLOR (ARGB)

    // ---- "Deferred" range (92..127) — light/material/fog/point ----
    FogEnable = 92,
    FogTableMode = 93,
    FogStart = 94,   // float
    FogEnd = 95,     // float
    FogDensity = 96, // float
    RangeFogEnable = 97,
    Wrap0 = 98,
    Wrap1 = 99,
    Wrap2 = 100,
    Wrap3 = 101,
    Lighting = 102, // TRUE/FALSE
    SpecularEnable = 103,
    LocalViewer = 104,
    ColorVertex = 105,
    SpecularMaterialSource = 110,
    DiffuseMaterialSource = 111,
    AmbientMaterialSource = 112,
    EmissiveMaterialSource = 113,
    Ambient = 115,   // D3DCOLOR
    PointSize = 116, // float
    PointSizeMin = 117,
    PointSpriteEnable = 118,
    PointScaleEnable = 119,
    PointScaleA = 120,
    PointScaleB = 121,
    PointScaleC = 122,
    PointSizeMax = 123,
    PatchSegments = 125,

    // ---- "Complex" range (136..166) ----
    VertexBlend = 137,
    FogColor = 138, // D3DCOLOR
    FillMode = 139, // D3DFILLMODE
    NormalizeNormals = 142,
    ZEnable = 143, // D3DZBUFFERTYPE: 0 off, 1 Z, 2 W
    StencilEnable = 144,
    StencilFail = 145,
    CullMode = 147,      // D3DCULL: 1 NONE, 2 CW, 3 CCW
    TextureFactor = 148, // D3DCOLOR
    ZBias = 149,
    EdgeAntiAlias = 151,
    MultiSampleAntiAlias = 152,
    MultiSampleMask = 153,
    LineWidth = 158, // Xbox ext., float
}

impl XboxRenderState {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            57 => Self::ZFunc,
            58 => Self::AlphaFunc,
            59 => Self::AlphaBlendEnable,
            60 => Self::AlphaTestEnable,
            61 => Self::AlphaRef,
            62 => Self::SrcBlend,
            63 => Self::DestBlend,
            64 => Self::ZWriteEnable,
            65 => Self::DitherEnable,
            66 => Self::ShadeMode,
            67 => Self::ColorWriteEnable,
            68 => Self::StencilZFail,
            69 => Self::StencilPass,
            70 => Self::StencilFunc,
            71 => Self::StencilRef,
            72 => Self::StencilMask,
            73 => Self::StencilWriteMask,
            74 => Self::BlendOp,
            75 => Self::BlendColor,
            92 => Self::FogEnable,
            93 => Self::FogTableMode,
            94 => Self::FogStart,
            95 => Self::FogEnd,
            96 => Self::FogDensity,
            97 => Self::RangeFogEnable,
            98 => Self::Wrap0,
            99 => Self::Wrap1,
            100 => Self::Wrap2,
            101 => Self::Wrap3,
            102 => Self::Lighting,
            103 => Self::SpecularEnable,
            104 => Self::LocalViewer,
            105 => Self::ColorVertex,
            110 => Self::SpecularMaterialSource,
            111 => Self::DiffuseMaterialSource,
            112 => Self::AmbientMaterialSource,
            113 => Self::EmissiveMaterialSource,
            115 => Self::Ambient,
            116 => Self::PointSize,
            117 => Self::PointSizeMin,
            118 => Self::PointSpriteEnable,
            119 => Self::PointScaleEnable,
            120 => Self::PointScaleA,
            121 => Self::PointScaleB,
            122 => Self::PointScaleC,
            123 => Self::PointSizeMax,
            125 => Self::PatchSegments,
            137 => Self::VertexBlend,
            138 => Self::FogColor,
            139 => Self::FillMode,
            142 => Self::NormalizeNormals,
            143 => Self::ZEnable,
            144 => Self::StencilEnable,
            145 => Self::StencilFail,
            147 => Self::CullMode,
            148 => Self::TextureFactor,
            149 => Self::ZBias,
            151 => Self::EdgeAntiAlias,
            152 => Self::MultiSampleAntiAlias,
            153 => Self::MultiSampleMask,
            158 => Self::LineWidth,
            _ => return None,
        })
    }

    /// Which host-side state group does this render-state belong to? Backends
    /// use this to decide what pipeline object to rebuild (e.g. D3D11 rebuilds
    /// a blend-state block when any Blend* RS changes).
    ///
    /// Full paths used throughout because several variant names collide between
    /// `XboxRenderState` and `HostStateGroup` (e.g. `Lighting`).
    pub fn host_group(self) -> HostStateGroup {
        use XboxRenderState as X;
        match self {
            X::AlphaBlendEnable
            | X::AlphaTestEnable
            | X::AlphaRef
            | X::SrcBlend
            | X::DestBlend
            | X::BlendOp
            | X::BlendColor
            | X::AlphaFunc
            | X::ColorWriteEnable => HostStateGroup::Blend,

            X::ZFunc | X::ZEnable | X::ZWriteEnable | X::ZBias => HostStateGroup::Depth,

            X::StencilZFail
            | X::StencilPass
            | X::StencilFunc
            | X::StencilRef
            | X::StencilMask
            | X::StencilWriteMask
            | X::StencilEnable
            | X::StencilFail => HostStateGroup::Stencil,

            X::CullMode
            | X::FillMode
            | X::EdgeAntiAlias
            | X::MultiSampleAntiAlias
            | X::MultiSampleMask
            | X::LineWidth
            | X::ShadeMode
            | X::DitherEnable
            | X::TextureFactor => HostStateGroup::Rasterizer,

            X::FogEnable
            | X::FogTableMode
            | X::FogStart
            | X::FogEnd
            | X::FogDensity
            | X::FogColor
            | X::RangeFogEnable => HostStateGroup::FogPipeline,

            X::Lighting
            | X::SpecularEnable
            | X::LocalViewer
            | X::ColorVertex
            | X::SpecularMaterialSource
            | X::DiffuseMaterialSource
            | X::AmbientMaterialSource
            | X::EmissiveMaterialSource
            | X::Ambient
            | X::NormalizeNormals => HostStateGroup::Lighting,

            X::Wrap0 | X::Wrap1 | X::Wrap2 | X::Wrap3 => HostStateGroup::TextureWrap,

            X::PointSize
            | X::PointSizeMin
            | X::PointSpriteEnable
            | X::PointScaleEnable
            | X::PointScaleA
            | X::PointScaleB
            | X::PointScaleC
            | X::PointSizeMax => HostStateGroup::PointSprite,

            X::VertexBlend | X::PatchSegments => HostStateGroup::VertexPipeline,
        }
    }
}

fn gl_cmp_to_d3d(value: u32) -> u32 {
    match value {
        0x0200 => CmpFunc::Never as u32,
        0x0201 => CmpFunc::Less as u32,
        0x0202 => CmpFunc::Equal as u32,
        0x0203 => CmpFunc::LessEqual as u32,
        0x0204 => CmpFunc::Greater as u32,
        0x0205 => CmpFunc::NotEqual as u32,
        0x0206 => CmpFunc::GreaterEqual as u32,
        0x0207 => CmpFunc::Always as u32,
        _ => value,
    }
}

fn gl_blend_to_d3d(value: u32) -> u32 {
    match value {
        0x0000 => Blend::Zero as u32,
        0x0001 => Blend::One as u32,
        0x0300 => Blend::SrcColor as u32,
        0x0301 => Blend::InvSrcColor as u32,
        0x0302 => Blend::SrcAlpha as u32,
        0x0303 => Blend::InvSrcAlpha as u32,
        0x0304 => Blend::DestAlpha as u32,
        0x0305 => Blend::InvDestAlpha as u32,
        0x0306 => Blend::DestColor as u32,
        0x0307 => Blend::InvDestColor as u32,
        0x0308 => Blend::SrcAlphaSat as u32,
        _ => value,
    }
}

fn gl_blend_op_to_d3d(value: u32) -> u32 {
    match value {
        0x8006 => 1, // FUNC_ADD
        0x800A => 2, // FUNC_SUBTRACT
        0x800B => 3, // FUNC_REVERSE_SUBTRACT
        _ => value,
    }
}

fn gl_shade_to_d3d(value: u32) -> u32 {
    match value {
        0x1D00 => 1, // GL_FLAT -> D3DSHADE_FLAT
        0x1D01 => 2, // GL_SMOOTH -> D3DSHADE_GOURAUD
        _ => value,
    }
}

fn color_mask_to_d3d(value: u32) -> u32 {
    if value <= 0x0F {
        return value;
    }

    let mut mask = 0u32;
    if (value & 0x0000_00FF) != 0 {
        mask |= 0x1; // blue
    }
    if (value & 0x0000_FF00) != 0 {
        mask |= 0x2; // green
    }
    if (value & 0x00FF_0000) != 0 {
        mask |= 0x4; // red
    }
    if (value & 0xFF00_0000) != 0 {
        mask |= 0x8; // alpha
    }
    mask
}

/// Normalize the fast SetRenderState path used by Xbox D3D8.
///
/// `D3DDevice_SetRenderState_Simple` writes NV097 method IDs such as
/// `0x00040344` (SET_BLEND_FUNC_SFACTOR), while the typed state table uses
/// X_D3DRS slots such as `SrcBlend = 62`. Treating the hardware method as a
/// linear D3DRS slot corrupts the pixel-shader combiner tracker, because
/// ordinary blend/depth writes land in states 0..56.
pub fn normalize_render_state_input(state: u32, value: u32) -> (u32, u32) {
    let method = if (0x0004_0000..0x0004_2000).contains(&state) {
        state - 0x0004_0000
    } else {
        state
    };

    use XboxRenderState as X;
    match method {
        0x0290 => (X::ShadeMode as u32, gl_shade_to_d3d(value)),
        0x0300 => (X::AlphaTestEnable as u32, value),
        0x0304 => (X::AlphaBlendEnable as u32, value),
        0x0308 => {
            if value == 0 {
                (X::CullMode as u32, CullMode::None as u32)
            } else {
                (X::CullMode as u32, CullMode::CounterClock as u32)
            }
        }
        0x030C => (X::ZEnable as u32, value),
        0x0310 => (X::DitherEnable as u32, value),
        0x0314 => (X::Lighting as u32, value),
        0x033C => (X::AlphaFunc as u32, gl_cmp_to_d3d(value)),
        0x0340 => (X::AlphaRef as u32, value),
        0x0344 => (X::SrcBlend as u32, gl_blend_to_d3d(value)),
        0x0348 => (X::DestBlend as u32, gl_blend_to_d3d(value)),
        0x034C => (X::BlendColor as u32, value),
        0x0350 => (X::BlendOp as u32, gl_blend_op_to_d3d(value)),
        0x0354 => (X::ZFunc as u32, gl_cmp_to_d3d(value)),
        0x0358 => (X::ColorWriteEnable as u32, color_mask_to_d3d(value)),
        0x035C => (X::ZWriteEnable as u32, value),
        0x03B8 => (X::SpecularEnable as u32, value),
        _ => (state, value),
    }
}

/// Host-side pipeline state object group. When a set-render-state call
/// comes in, the backend only needs to rebuild the object tied to the
/// group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostStateGroup {
    Blend,
    Depth,
    Stencil,
    Rasterizer,
    FogPipeline,
    Lighting,
    TextureWrap,
    PointSprite,
    VertexPipeline,
}

/// Xbox D3DCMPFUNC values (D3DCMP_LESS etc.) — identical to PC D3D8/D3D9.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CmpFunc {
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

impl CmpFunc {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            1 => Self::Never,
            2 => Self::Less,
            3 => Self::Equal,
            4 => Self::LessEqual,
            5 => Self::Greater,
            6 => Self::NotEqual,
            7 => Self::GreaterEqual,
            8 => Self::Always,
            _ => return None,
        })
    }
}

/// Xbox D3DCULL values — identical to PC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CullMode {
    None = 1,
    Clockwise = 2,    // D3DCULL_CW
    CounterClock = 3, // D3DCULL_CCW
}

impl CullMode {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            1 => Self::None,
            2 => Self::Clockwise,
            3 => Self::CounterClock,
            _ => return None,
        })
    }
}

/// Xbox D3DFILLMODE values — identical to PC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FillMode {
    Point = 1,
    Wireframe = 2,
    Solid = 3,
}

impl FillMode {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            1 => Self::Point,
            2 => Self::Wireframe,
            3 => Self::Solid,
            _ => return None,
        })
    }
}

/// Xbox D3DBLEND values. Most match PC values 1-17; some are Xbox ext.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Blend {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    InvSrcColor = 4,
    SrcAlpha = 5,
    InvSrcAlpha = 6,
    DestAlpha = 7,
    InvDestAlpha = 8,
    DestColor = 9,
    InvDestColor = 10,
    SrcAlphaSat = 11,
    BothSrcAlpha = 12,
    BothInvSrcAlpha = 13,
    BlendFactor = 14, // Xbox ext. (PC D3D9 but not D3D8)
    InvBlendFactor = 15,
}

impl Blend {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            1 => Self::Zero,
            2 => Self::One,
            3 => Self::SrcColor,
            4 => Self::InvSrcColor,
            5 => Self::SrcAlpha,
            6 => Self::InvSrcAlpha,
            7 => Self::DestAlpha,
            8 => Self::InvDestAlpha,
            9 => Self::DestColor,
            10 => Self::InvDestColor,
            11 => Self::SrcAlphaSat,
            12 => Self::BothSrcAlpha,
            13 => Self::BothInvSrcAlpha,
            14 => Self::BlendFactor,
            15 => Self::InvBlendFactor,
            _ => return None,
        })
    }
}

/// Live render-state tracker. Holds current values for every RS the game has
/// set via D3DDevice_SetRenderState. Initialized to D3D8 documented defaults.
///
/// Backends consult this struct at draw time (or when host_group() indicates
/// a change) to build their pipeline state objects.
#[derive(Debug, Clone, Copy)]
pub struct RenderStateTable {
    // Blend
    pub alpha_blend_enable: bool,
    pub alpha_test_enable: bool,
    pub alpha_ref: u8,
    pub alpha_func: CmpFunc,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: u32,           // D3DBLENDOP enum (1=ADD, 2=SUB, ...)
    pub blend_color: u32,        // D3DCOLOR ARGB
    pub color_write_enable: u32, // channel mask
    // Depth
    pub z_enable: u32, // 0=off, 1=Z, 2=W
    pub z_write_enable: bool,
    pub z_func: CmpFunc,
    pub z_bias: u32,
    // Stencil
    pub stencil_enable: bool,
    pub stencil_ref: u8,
    pub stencil_mask: u8,
    pub stencil_write_mask: u8,
    pub stencil_func: CmpFunc,
    pub stencil_fail: u32,
    pub stencil_zfail: u32,
    pub stencil_pass: u32,
    // Rasterizer
    pub cull_mode: CullMode,
    pub fill_mode: FillMode,
    pub shade_mode: u32,
    pub dither_enable: bool,
    pub line_width: f32,
    // Fog
    pub fog_enable: bool,
    pub fog_color: u32,
    pub fog_start: f32,
    pub fog_end: f32,
    pub fog_density: f32,
    // Lighting
    pub lighting: bool,
    pub ambient: u32,
    pub normalize_normals: bool,
    // Factor
    pub texture_factor: u32,
}

impl Default for RenderStateTable {
    fn default() -> Self {
        // Defaults match D3D8 documented defaults for each state.
        RenderStateTable {
            alpha_blend_enable: false,
            alpha_test_enable: false,
            alpha_ref: 0,
            alpha_func: CmpFunc::Always,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: 1, // D3DBLENDOP_ADD
            blend_color: 0,
            color_write_enable: 0x0F, // all channels
            z_enable: 1,              // D3DZB_TRUE
            z_write_enable: true,
            z_func: CmpFunc::LessEqual,
            z_bias: 0,
            stencil_enable: false,
            stencil_ref: 0,
            stencil_mask: 0xFF,
            stencil_write_mask: 0xFF,
            stencil_func: CmpFunc::Always,
            stencil_fail: 1, // D3DSTENCILOP_KEEP
            stencil_zfail: 1,
            stencil_pass: 1,
            cull_mode: CullMode::CounterClock,
            fill_mode: FillMode::Solid,
            shade_mode: 2, // D3DSHADE_GOURAUD
            dither_enable: false,
            line_width: 1.0,
            fog_enable: false,
            fog_color: 0,
            fog_start: 0.0,
            fog_end: 1.0,
            fog_density: 1.0,
            lighting: true, // D3D8 default is lighting ON
            ambient: 0,
            normalize_normals: false,
            texture_factor: 0xFFFFFFFF, // opaque white
        }
    }
}

impl RenderStateTable {
    /// Apply a SetRenderState call. Returns the host state group that needs
    /// to be rebuilt, or `None` if the state is unknown / ignored.
    ///
    /// The caller is the HLE handler for D3DDevice_SetRenderState which
    /// knows the raw (slot, value) tuple from guest args. It then calls
    /// this to update local state and learn what backend work is needed.
    ///
    /// Full-path matching on `XboxRenderState` because several variant
    /// names collide with the standalone `CullMode`/`FillMode` types.
    pub fn apply(&mut self, slot: u32, value: u32) -> Option<HostStateGroup> {
        let rs = XboxRenderState::from_u32(slot)?;
        use XboxRenderState as X;
        match rs {
            X::AlphaBlendEnable => self.alpha_blend_enable = value != 0,
            X::AlphaTestEnable => self.alpha_test_enable = value != 0,
            X::AlphaRef => self.alpha_ref = value as u8,
            X::AlphaFunc => self.alpha_func = CmpFunc::from_u32(value).unwrap_or(self.alpha_func),
            X::SrcBlend => self.src_blend = Blend::from_u32(value).unwrap_or(self.src_blend),
            X::DestBlend => self.dest_blend = Blend::from_u32(value).unwrap_or(self.dest_blend),
            X::BlendOp => self.blend_op = value,
            X::BlendColor => self.blend_color = value,
            X::ColorWriteEnable => self.color_write_enable = value,

            X::ZEnable => self.z_enable = value,
            X::ZWriteEnable => self.z_write_enable = value != 0,
            X::ZFunc => self.z_func = CmpFunc::from_u32(value).unwrap_or(self.z_func),
            X::ZBias => self.z_bias = value,

            X::StencilEnable => self.stencil_enable = value != 0,
            X::StencilRef => self.stencil_ref = value as u8,
            X::StencilMask => self.stencil_mask = value as u8,
            X::StencilWriteMask => self.stencil_write_mask = value as u8,
            X::StencilFunc => {
                self.stencil_func = CmpFunc::from_u32(value).unwrap_or(self.stencil_func)
            }
            X::StencilFail => self.stencil_fail = value,
            X::StencilZFail => self.stencil_zfail = value,
            X::StencilPass => self.stencil_pass = value,

            X::CullMode => self.cull_mode = CullMode::from_u32(value).unwrap_or(self.cull_mode),
            X::FillMode => self.fill_mode = FillMode::from_u32(value).unwrap_or(self.fill_mode),
            X::ShadeMode => self.shade_mode = value,
            X::DitherEnable => self.dither_enable = value != 0,
            X::LineWidth => self.line_width = f32::from_bits(value),

            X::FogEnable => self.fog_enable = value != 0,
            X::FogColor => self.fog_color = value,
            X::FogStart => self.fog_start = f32::from_bits(value),
            X::FogEnd => self.fog_end = f32::from_bits(value),
            X::FogDensity => self.fog_density = f32::from_bits(value),

            X::Lighting => self.lighting = value != 0,
            X::Ambient => self.ambient = value,
            X::NormalizeNormals => self.normalize_normals = value != 0,

            X::TextureFactor => self.texture_factor = value,

            // Any state we recognize but don't track in this struct — still
            // return its group so the backend could record/forward.
            _ => {}
        }
        Some(rs.host_group())
    }
}

// ============================================================================
// Global singleton for HLE handlers to update on SetRenderState
// ============================================================================

use std::sync::Mutex;

/// Global render-state table. A single instance tracks the guest's current
/// render-state slots as set via SetRenderState HLE calls. Backends consult
/// this at draw time.
///
/// Wrapped in a Mutex because the worker thread calls SetRenderState while
/// the main thread consults the table during readback / pipeline rebuild.
/// Contention is low (SetRenderState is guest-driven, not hot).
pub static GLOBAL_RENDER_STATE: Mutex<RenderStateTable> =
    Mutex::new(RenderStateTable::const_default());

impl RenderStateTable {
    /// `const fn` version of Default for static initialization. Kept in sync
    /// with `impl Default`.
    pub const fn const_default() -> Self {
        RenderStateTable {
            alpha_blend_enable: false,
            alpha_test_enable: false,
            alpha_ref: 0,
            alpha_func: CmpFunc::Always,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: 1,
            blend_color: 0,
            color_write_enable: 0x0F,
            z_enable: 1,
            z_write_enable: true,
            z_func: CmpFunc::LessEqual,
            z_bias: 0,
            stencil_enable: false,
            stencil_ref: 0,
            stencil_mask: 0xFF,
            stencil_write_mask: 0xFF,
            stencil_func: CmpFunc::Always,
            stencil_fail: 1,
            stencil_zfail: 1,
            stencil_pass: 1,
            cull_mode: CullMode::CounterClock,
            fill_mode: FillMode::Solid,
            shade_mode: 2,
            dither_enable: false,
            line_width: 1.0,
            fog_enable: false,
            fog_color: 0,
            fog_start: 0.0,
            fog_end: 1.0,
            fog_density: 1.0,
            lighting: true,
            ambient: 0,
            normalize_normals: false,
            texture_factor: 0xFFFFFFFF,
        }
    }
}

/// Convenience entry point for HLE handlers.
///
/// Returns the `HostStateGroup` that was touched (so the caller can log /
/// debounce pipeline rebuilds), or `None` if the slot isn't recognized.
pub fn apply_global(slot: u32, value: u32) -> Option<HostStateGroup> {
    if let Ok(mut t) = GLOBAL_RENDER_STATE.lock() {
        t.apply(slot, value)
    } else {
        None
    }
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_table_matches_d3d8_docs() {
        let t = RenderStateTable::default();
        assert_eq!(t.alpha_blend_enable, false);
        assert_eq!(t.z_enable, 1);
        assert_eq!(t.z_write_enable, true);
        assert_eq!(t.z_func, CmpFunc::LessEqual);
        assert_eq!(t.cull_mode, CullMode::CounterClock);
        assert_eq!(t.lighting, true);
    }

    #[test]
    fn apply_alpha_blend_enable() {
        let mut t = RenderStateTable::default();
        let group = t.apply(XboxRenderState::AlphaBlendEnable as u32, 1);
        assert_eq!(group, Some(HostStateGroup::Blend));
        assert!(t.alpha_blend_enable);
    }

    #[test]
    fn apply_cull_mode() {
        let mut t = RenderStateTable::default();
        let group = t.apply(XboxRenderState::CullMode as u32, CullMode::Clockwise as u32);
        assert_eq!(group, Some(HostStateGroup::Rasterizer));
        assert_eq!(t.cull_mode, CullMode::Clockwise);
    }

    #[test]
    fn apply_z_enable() {
        let mut t = RenderStateTable::default();
        assert_eq!(
            t.apply(XboxRenderState::ZEnable as u32, 0),
            Some(HostStateGroup::Depth)
        );
        assert_eq!(t.z_enable, 0);
    }

    #[test]
    fn apply_unknown_slot_returns_none() {
        let mut t = RenderStateTable::default();
        assert_eq!(t.apply(999, 0), None);
        // State unchanged
        assert_eq!(t.alpha_blend_enable, false);
    }

    #[test]
    fn normalizes_fast_path_method_ids_to_render_state_slots() {
        assert_eq!(
            normalize_render_state_input(0x0004_0344, 0x0302),
            (XboxRenderState::SrcBlend as u32, Blend::SrcAlpha as u32)
        );
        assert_eq!(
            normalize_render_state_input(0x0004_0348, 0x0303),
            (XboxRenderState::DestBlend as u32, Blend::InvSrcAlpha as u32)
        );
        assert_eq!(
            normalize_render_state_input(0x0004_0358, 0xFFFF_FFFF),
            (XboxRenderState::ColorWriteEnable as u32, 0x0F)
        );
    }

    #[test]
    fn unmapped_fast_path_method_is_not_treated_as_linear_slot() {
        let input = (0x0004_1880, 0xDEAD_BEEF);
        assert_eq!(normalize_render_state_input(input.0, input.1), input);

        let mut t = RenderStateTable::default();
        assert_eq!(t.apply(input.0, input.1), None);
        assert_eq!(t.src_blend, Blend::One);
        assert_eq!(t.dest_blend, Blend::Zero);
        assert_eq!(t.color_write_enable, 0x0F);
    }

    #[test]
    fn blend_enum_roundtrip() {
        for v in 1u32..=15 {
            let b = Blend::from_u32(v).unwrap();
            assert_eq!(b as u32, v);
        }
        assert!(Blend::from_u32(16).is_none());
        assert!(Blend::from_u32(0).is_none());
    }

    #[test]
    fn cmpfunc_enum_roundtrip() {
        for v in 1u32..=8 {
            let c = CmpFunc::from_u32(v).unwrap();
            assert_eq!(c as u32, v);
        }
    }

    #[test]
    fn apply_float_state() {
        let mut t = RenderStateTable::default();
        let f: f32 = 42.5;
        t.apply(XboxRenderState::FogStart as u32, f.to_bits());
        assert_eq!(t.fog_start, 42.5);
    }

    #[test]
    fn host_group_coverage() {
        // All defined RS slots must return a group
        let slots = [57u32, 59, 62, 64, 66, 92, 102, 116, 137, 139, 143, 147];
        for s in slots {
            let rs = XboxRenderState::from_u32(s).unwrap();
            let _ = rs.host_group(); // must not panic
        }
    }

    #[test]
    fn invalid_enum_value_doesnt_corrupt_state() {
        let mut t = RenderStateTable::default();
        // CullMode slot with invalid value → state preserved
        t.apply(XboxRenderState::CullMode as u32, 999);
        assert_eq!(t.cull_mode, CullMode::CounterClock);
    }
}
