pub mod d3d11;
pub mod d3d12;
pub mod d3d9;
pub mod nv2a_psh;
pub mod nv2a_vsh;
mod shaders;
/// Xbox texture Morton-order swizzle/unswizzle kernel. Pure module —
/// no existing call sites invoke it yet. Spliced from rustemu-jit
/// 2026-04-20 as an additive step; wiring into the readback path
/// is intentionally a separate follow-up change.
pub mod swizzle;
/// GPU rendering backend abstraction.
/// Trait + global accessor for HLE stubs to call draw/clear/swap.
/// D3D11 is the most complete hardware backend today; D3D9 is an explicit
/// Cxbx-R-style comparison target but still missing several parity lanes; SWR
/// remains the CPU diagnostic fallback. Runtime selection.
pub mod swr;
pub mod texture_format;

/// D3D8 Flexible Vertex Format (FVF) decoder — pure bitfield math + tables
/// ported from Cxbx-Reloaded, no backend dependency. First foundation piece
/// for organic Spider-Man geometry rendering. Unit-tested standalone.
pub mod fvf_decode;

/// D3D8 render-state table and host pipeline-group classifier. Tracks the
/// guest's SetRenderState calls in a strongly-typed struct. Ported from
/// Cxbx-Reloaded XbD3D8Types.h + XbState.cpp. Unit-tested standalone.
pub mod render_state;

pub mod xpr;

use std::sync::{Mutex, MutexGuard, OnceLock};

// ============================================================================
// NV2AVertex — matches C++ NV2AInterceptor.h (28 bytes)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct NV2AVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub color: u32, // ARGB32
    pub u: f32,
    pub v: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct VertexSkinPayload {
    pub normal: [f32; 4],
    pub uv1: [f32; 4],
    pub blend_indices: [f32; 4],
    pub blend_weights: [f32; 4],
}

impl Default for VertexSkinPayload {
    fn default() -> Self {
        Self {
            normal: [0.0, 0.0, 1.0, 1.0],
            uv1: [0.0, 0.0, 0.0, 1.0],
            blend_indices: [0.0; 4],
            blend_weights: [0.0; 4],
        }
    }
}

static NEXT_DRAW_SKIN_PAYLOAD: OnceLock<Mutex<Vec<VertexSkinPayload>>> = OnceLock::new();

pub fn set_next_draw_skin_payload(payload: Vec<VertexSkinPayload>) {
    let mut guard = NEXT_DRAW_SKIN_PAYLOAD
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    *guard = payload;
}

pub fn take_next_draw_skin_payload(expected_len: usize) -> Vec<VertexSkinPayload> {
    let mut guard = NEXT_DRAW_SKIN_PAYLOAD
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let mut payload = std::mem::take(&mut *guard);
    if payload.len() < expected_len {
        payload.resize(expected_len, VertexSkinPayload::default());
    }
    payload.truncate(expected_len);
    payload
}

// NV097 primitive types (NV2A hardware values)
pub const NV097_POINTS: i32 = 1;
pub const NV097_LINES: i32 = 2;
pub const NV097_LINE_LOOP: i32 = 3;
pub const NV097_LINE_STRIP: i32 = 4;
pub const NV097_TRIANGLES: i32 = 5;
pub const NV097_TRIANGLE_STRIP: i32 = 6;
pub const NV097_TRIANGLE_FAN: i32 = 7;
pub const NV097_QUAD_LIST: i32 = 8; // Xbox D3DPT_QUADLIST; expanded by host backends.

// ============================================================================
// GpuBackend trait — equivalent to C++ IGPUBackend
// ============================================================================

/// Xbox D3D8 render state IDs (subset used by Spider-Man)
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum XboxRenderState {
    ZEnable = 0x03AC,
    FillMode = 0x03B8,
    ShadeMode = 0x03BC,
    ZWriteEnable = 0x03B4,
    AlphaTestEnable = 0x0340,
    SrcBlend = 0x0344,
    DestBlend = 0x0348,
    CullMode = 0x038C,
    ZFunc = 0x0354,
    AlphaRef = 0x034C,
    AlphaFunc = 0x0350,
    AlphaBlendEnable = 0x0338,
    FogEnable = 0x0398,
    SpecularEnable = 0x03C0,
    FogColor = 0x03A0,
    Lighting = 0x03C4,
    // Catch-all for unknown states
    Unknown = 0xFFFF,
}

/// Transform state types (matching D3DTRANSFORMSTATETYPE)
pub const D3DTS_WORLD: u32 = 256;
pub const D3DTS_VIEW: u32 = 2;
pub const D3DTS_PROJECTION: u32 = 3;

pub const X_D3DTSS_ADDRESSU: usize = 0;
pub const X_D3DTSS_ADDRESSV: usize = 1;
pub const X_D3DTSS_ADDRESSW: usize = 2;
pub const X_D3DTSS_MAGFILTER: usize = 3;
pub const X_D3DTSS_MINFILTER: usize = 4;
pub const X_D3DTSS_MIPFILTER: usize = 5;
pub const X_D3DTSS_COLOROP: usize = 12;
pub const X_D3DTSS_COLORARG0: usize = 13;
pub const X_D3DTSS_COLORARG1: usize = 14;
pub const X_D3DTSS_COLORARG2: usize = 15;
pub const X_D3DTSS_ALPHAOP: usize = 16;
pub const X_D3DTSS_ALPHAARG0: usize = 17;
pub const X_D3DTSS_ALPHAARG1: usize = 18;
pub const X_D3DTSS_ALPHAARG2: usize = 19;
pub const X_D3DTSS_RESULTARG: usize = 20;
pub const X_D3DTSS_TEXTURETRANSFORMFLAGS: usize = 21;
pub const X_D3DTSS_TEXCOORDINDEX: usize = 28;

pub const X_D3DTOP_DISABLE: u32 = 1;
pub const X_D3DTOP_SELECTARG1: u32 = 2;
pub const X_D3DTOP_SELECTARG2: u32 = 3;
pub const X_D3DTOP_MODULATE: u32 = 4;
pub const X_D3DTOP_MODULATE2X: u32 = 5;
pub const X_D3DTOP_MODULATE4X: u32 = 6;
pub const X_D3DTOP_ADD: u32 = 7;
pub const X_D3DTOP_ADDSIGNED: u32 = 8;

pub const X_D3DTA_DIFFUSE: u32 = 0;
pub const X_D3DTA_CURRENT: u32 = 1;
pub const X_D3DTA_TEXTURE: u32 = 2;
pub const X_D3DTA_TFACTOR: u32 = 3;

pub const X_D3DTADDRESS_WRAP: u32 = 1;

pub const X_D3DTEXF_NONE: u32 = 0;
pub const X_D3DTEXF_POINT: u32 = 1;

pub fn default_texture_stage_states() -> [[u32; 32]; 4] {
    let mut states = [[0u32; 32]; 4];

    for (stage, s) in states.iter_mut().enumerate() {
        s[X_D3DTSS_ADDRESSU] = X_D3DTADDRESS_WRAP;
        s[X_D3DTSS_ADDRESSV] = X_D3DTADDRESS_WRAP;
        s[X_D3DTSS_ADDRESSW] = X_D3DTADDRESS_WRAP;
        s[X_D3DTSS_MAGFILTER] = X_D3DTEXF_POINT;
        s[X_D3DTSS_MINFILTER] = X_D3DTEXF_POINT;
        s[X_D3DTSS_MIPFILTER] = X_D3DTEXF_NONE;
        s[X_D3DTSS_COLOROP] = if stage == 0 {
            X_D3DTOP_MODULATE
        } else {
            X_D3DTOP_DISABLE
        };
        s[X_D3DTSS_COLORARG0] = X_D3DTA_CURRENT;
        s[X_D3DTSS_COLORARG1] = X_D3DTA_TEXTURE;
        s[X_D3DTSS_COLORARG2] = X_D3DTA_DIFFUSE;
        s[X_D3DTSS_ALPHAOP] = if stage == 0 {
            X_D3DTOP_SELECTARG1
        } else {
            X_D3DTOP_DISABLE
        };
        s[X_D3DTSS_ALPHAARG0] = X_D3DTA_CURRENT;
        s[X_D3DTSS_ALPHAARG1] = X_D3DTA_TEXTURE;
        s[X_D3DTSS_ALPHAARG2] = X_D3DTA_DIFFUSE;
        s[X_D3DTSS_RESULTARG] = X_D3DTA_CURRENT;
        s[X_D3DTSS_TEXTURETRANSFORMFLAGS] = 0;
        s[X_D3DTSS_TEXCOORDINDEX] = stage as u32;
    }

    states
}

pub trait GpuBackend: Send {
    fn init(&mut self, width: i32, height: i32) -> bool;
    fn shutdown(&mut self);
    fn draw_primitive(&mut self, verts: &[NV2AVertex], prim_type: i32);
    fn clear(&mut self, color: u32);
    fn clear_surface(
        &mut self,
        color: u32,
        clear_color: bool,
        _clear_depth: bool,
        _clear_stencil: bool,
    ) {
        if clear_color {
            self.clear(color);
        }
    }
    /// Upload texture data from guest memory. pixels is ARGB8888, row-major.
    fn set_texture(&mut self, _stage: u32, _width: u32, _height: u32, _pixels: &[u32]) {}
    /// Unbind a texture stage and return to vertex-color-only shading.
    fn clear_texture(&mut self, _stage: u32) {}
    /// Upload raw texture bytes with Xbox format code.
    /// Supports DXT1 (format=0x0C) as DXGI_FORMAT_BC1_UNORM passthrough.
    /// Default impl falls through to set_texture for A8R8G8B8.
    fn set_texture_raw(
        &mut self,
        _stage: u32,
        _width: u32,
        _height: u32,
        _format_code: u32,
        _bytes: &[u8],
    ) {
    }
    /// Bind a previously-created render target as a texture. Used for Xbox
    /// render-to-texture surfaces that never have guest-visible pixel data.
    fn bind_render_target_texture(&mut self, _stage: u32, _key: u32) -> bool {
        false
    }
    /// Bind a previously-created render target whose backing memory range
    /// contains the supplied guest data address.
    fn bind_render_target_texture_by_data(&mut self, _stage: u32, _data_addr: u32) -> bool {
        false
    }
    /// Attach guest-side texture identity to the next backend texture bind.
    /// This is diagnostic-only; rendering still uses the uploaded pixels/SRV.
    fn set_texture_debug_info(
        &mut self,
        _stage: u32,
        _tex_raw: u32,
        _tex_norm: u32,
        _data_addr: u32,
        _format_code: u32,
    ) {
    }
    /// Hint whether the next block-compressed upload is stored in Xbox block
    /// swizzled order. Runtime LockRect updates are linear even when the format
    /// code is DXT1, so the backend must not infer this from format alone.
    fn set_texture_swizzle_hint(&mut self, _stage: u32, _swizzled_blocks: bool) {}
    /// Set viewport (x, y, width, height, min_z, max_z)
    fn set_viewport(&mut self, _x: u32, _y: u32, _w: u32, _h: u32, _min_z: f32, _max_z: f32) {}
    /// Set a 4x4 transform matrix (world, view, or projection)
    fn set_transform(&mut self, _state: u32, _matrix: &[f32; 16]) {}
    /// Set a render state (NV2A register, value)
    fn set_render_state(&mut self, _state: u32, _value: u32) {}
    /// Set a texture-stage state (Xbox D3D8 D3DTSS_* slot, value).
    fn set_texture_stage_state(&mut self, _stage: u32, _state: u32, _value: u32) {}
    /// Set vertex shader constant (register index, 4 floats)
    fn set_vs_constant(&mut self, _reg: u32, _data: &[f32]) {}
    /// Bind a translated NV2A vertex shader. None restores the fixed passthrough shader.
    fn set_vertex_shader_hlsl(&mut self, _handle: u32, _hlsl: Option<&str>) {}
    /// Set pixel shader constant (register index, 4 floats)
    fn set_ps_constant(&mut self, _reg: u32, _data: &[f32]) {}
    /// Resolve/copy the current back buffer into the hardware scanout surface.
    fn resolve_scanout(&mut self, _pcrtc_start: u32) -> bool {
        false
    }
    /// Xbox Swap(COPY) copies the current back buffer into the emulated
    /// display/front surface without presenting it yet.
    fn copy_backbuffer_to_display(&mut self, pcrtc_start: u32) -> bool {
        self.resolve_scanout(pcrtc_start)
    }
    /// Route subsequent front-buffer/UI draws to the emulated display surface
    /// until Swap(FINISH) restores the normal back buffer.
    fn bind_display_as_render_target(&mut self, _pcrtc_start: u32) -> bool {
        false
    }
    /// Restore ordinary back-buffer rendering after a front-buffer pass.
    fn restore_backbuffer_render_target(&mut self) {}
    /// Bind the implicit D3D8 backbuffer that exists after CreateDevice.
    ///
    /// Many games never call SetRenderTarget before their first draw; real D3D8
    /// starts with the swap-chain backbuffer already active.
    fn bind_default_render_target(&mut self) {
        self.restore_backbuffer_render_target()
    }
    /// Present the emulated display/front surface.
    fn present_display(&mut self, _pcrtc_start: u32) -> &[u32] {
        self.present_framebuffer()
    }
    /// Returns readback buffer as XRGB8888 pixels (width * height).
    fn readback_framebuffer(&mut self) -> &[u32];
    /// Returns the frame that should be presented to RetroArch.
    /// Diagnostic captures should use readback_framebuffer(), while Swap uses
    /// this presentation path so it can follow PCRTC scanout after a resolve.
    fn present_framebuffer(&mut self) -> &[u32] {
        self.readback_framebuffer()
    }
    /// Debug identity for the currently bound render target:
    /// (key, width, height, guest data address, pitch).
    fn debug_active_render_target(&self) -> Option<(u32, u32, u32, u32, u32)> {
        None
    }
    /// True when the current color target was cleared to opaque black and no
    /// draw has touched it since. Some Xbox video/overlay setup paths use that
    /// clear/swap as synchronization, not as a frame worth presenting.
    fn black_clear_without_draw_pending(&self) -> bool {
        false
    }
    fn name(&self) -> &'static str;
}

// ============================================================================
// Backend selection
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    D3D9,
    D3D11,
    D3D12,
    Swr,
}

// ============================================================================
// Global GPU backend (accessed from OOVPA HLE stubs)
// ============================================================================

static GPU: Mutex<Option<Box<dyn GpuBackend>>> = Mutex::new(None);

/// Shared readback buffer — written by hle_swap, read by main thread each frame.
static READBACK: Mutex<Option<Vec<u32>>> = Mutex::new(None);

// ============================================================================
// Draw queue (spliced from JIT branch, 2026-04-20)
// ============================================================================
//
// Worker thread's NV2A pushbuffer parser pushes finished vertex batches here.
// hle_swap drains the queue into the backend just before readback so the
// frame reflects this pass's geometry. Lock is held only during push/drain,
// never during the hot inner extraction loop.

#[derive(Debug, Clone)]
pub struct QueuedDraw {
    pub verts: Vec<NV2AVertex>,
    pub prim_type: i32,
}

static DRAW_QUEUE: Mutex<Vec<QueuedDraw>> = Mutex::new(Vec::new());

/// Debug flag (per C03): temporarily suppress the first queued draw so
/// Spider-Man's guest-authored full-screen opaque-black splash quad
/// (color=0xFF000000, z=1510, covers entire viewport) doesn't mask the
/// hle_clear liveness pulse diagnostic during HUD debugging. Flip to
/// `false` once the HUD renders reliably on top of the splash quad.
/// Rationale documented at agent C03: without this flag, a successful
/// boot looks identical to a dead-pipeline run — both are pure black.
const SUPPRESS_DRAW_0: bool = false;

/// Worker calls this from nv2a_pb parser to queue a completed batch.
pub fn queue_draw(verts: Vec<NV2AVertex>, prim_type: i32) {
    static PUSH_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = PUSH_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        let (xmin, xmax, ymin, ymax) = verts.iter().fold(
            (
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ),
            |(xmin, xmax, ymin, ymax), v| {
                (xmin.min(v.x), xmax.max(v.x), ymin.min(v.y), ymax.max(v.y))
            },
        );
        crate::xbox::emulator::debug_log(&format!(
            "[queue_draw] #{} prim={} nverts={} x=[{:.1}..{:.1}] y=[{:.1}..{:.1}]",
            n,
            prim_type,
            verts.len(),
            xmin,
            xmax,
            ymin,
            ymax
        ));
    }
    // Suppress Draw #0 (full-screen splash quad) while debugging HUD.
    if SUPPRESS_DRAW_0 && n == 0 {
        crate::xbox::emulator::debug_log(
            "[queue_draw] SUPPRESSED #0 (debug: keep hle_clear pulse visible)",
        );
        return;
    }
    let mut q = DRAW_QUEUE.lock().unwrap_or_else(|e| e.into_inner());
    q.push(QueuedDraw { verts, prim_type });
}

/// Drain the accumulated draw queue into the given backend.
/// Called from the main thread inside hle_swap BEFORE readback_framebuffer
/// so the presented frame reflects the worker's submitted geometry.
pub fn drain_draw_queue(backend: &mut dyn GpuBackend) -> usize {
    // Swap under lock, replay without holding the lock so worker pushes
    // can still land during drain.
    let drained: Vec<QueuedDraw> = {
        let mut q = DRAW_QUEUE.lock().unwrap_or_else(|e| e.into_inner());
        std::mem::take(&mut *q)
    };
    let n = drained.len();
    // Heartbeat log (added 2026-04-23 Sprint 6): distinguishes
    // "queue empty every swap" from "queue fills but pixels wrong".
    // Keep it sparse; Spider-Man can drain tens of thousands of non-empty
    // queues and this log otherwise starves higher-value frame markers.
    static DRAIN_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let drain_i = DRAIN_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if drain_i < 8 || n > 0 && (drain_i.is_power_of_two() || drain_i % 1024 == 0) {
        crate::xbox::emulator::debug_log(&format!(
            "[drain_draw_queue] #{} drained={} backend={} (backend.draw_primitive calls this swap)",
            drain_i,
            n,
            backend.name()
        ));
    }
    for (i, d) in drained.into_iter().enumerate() {
        if drain_i < 4 && i < 8 {
            let (xmin, xmax, ymin, ymax) = d.verts.iter().fold(
                (
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                ),
                |(xmin, xmax, ymin, ymax), v| {
                    (xmin.min(v.x), xmax.max(v.x), ymin.min(v.y), ymax.max(v.y))
                },
            );
            crate::xbox::emulator::debug_log(&format!(
                "[drain_draw_queue-call] drain={} item={} backend={} prim={} nverts={} x=[{:.1}..{:.1}] y=[{:.1}..{:.1}]",
                drain_i,
                i,
                backend.name(),
                d.prim_type,
                d.verts.len(),
                xmin,
                xmax,
                ymin,
                ymax
            ));
        }
        backend.draw_primitive(&d.verts, d.prim_type);
    }
    n
}

/// Drain queued PB/HLE geometry into the active backend before a state mutation.
/// Xbox pushbuffers are strictly ordered; if we queue vertices but apply state
/// changes immediately, old quads render with future textures/materials.
pub fn drain_queued_draws_into_active_backend(reason: &str) -> usize {
    let mut gpu = gpu_lock();
    let Some(ref mut backend) = *gpu else {
        return 0;
    };
    let n = drain_draw_queue(backend.as_mut());
    if n != 0 {
        static ORDER_DRAIN_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let k = ORDER_DRAIN_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if k < 32 || k.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[GPU-ORDER-DRAIN] #{} drained={} before {}",
                k, n, reason
            ));
        }
    }
    n
}

pub fn gpu_lock() -> MutexGuard<'static, Option<Box<dyn GpuBackend>>> {
    GPU.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn install_gpu(backend: Box<dyn GpuBackend>) {
    *GPU.lock().unwrap_or_else(|e| e.into_inner()) = Some(backend);
}

pub fn remove_gpu() {
    if let Some(mut gpu) = GPU.lock().unwrap_or_else(|e| e.into_inner()).take() {
        gpu.shutdown();
    }
}

pub fn is_gpu_installed() -> bool {
    GPU.lock().unwrap_or_else(|e| e.into_inner()).is_some()
}

pub fn backend_name() -> &'static str {
    let guard = GPU.lock().unwrap_or_else(|e| e.into_inner());
    match &*guard {
        Some(gpu) => gpu.name(),
        None => "none",
    }
}

/// Flag: worker thread requests GPU init, main thread performs it.
/// Avoids D3D11 device creation on worker thread (conflicts with VEH).
static GPU_INIT_REQUESTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Worker calls this to request GPU init (instead of calling init_backend directly).
pub fn request_gpu_init() {
    GPU_INIT_REQUESTED.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Main thread calls this each frame to check if GPU init is needed.
/// Returns the backend kind if init was performed this frame.
pub fn poll_gpu_init(width: i32, height: i32) -> Option<BackendKind> {
    if GPU_INIT_REQUESTED
        .compare_exchange(
            true,
            false,
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_ok()
        && !is_gpu_installed()
    {
        Some(init_backend(width, height))
    } else {
        None
    }
}

/// Flag: worker thread signals that a new frame is ready for readback.
/// Main thread checks this and performs D3D11 readback (context is single-threaded).
static FRAME_DIRTY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Worker calls this after Swap to signal a frame is ready.
pub fn set_frame_dirty() {
    FRAME_DIRTY.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// Main thread calls this to acknowledge a new frame is ready.
/// The worker thread already performed readback and stored pixels in hle_swap.
/// This just clears the dirty flag — no GPU access needed on main thread.
pub fn poll_readback() -> bool {
    FRAME_DIRTY
        .compare_exchange(
            true,
            false,
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_ok()
}

/// Store readback pixels (called from main thread after readback).
pub fn store_readback(pixels: &[u32]) {
    let mut guard = READBACK.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut buf) = *guard {
        buf.copy_from_slice(pixels);
    } else {
        *guard = Some(pixels.to_vec());
    }
    // Note: store_readback path is not where the tile-pattern framebuffer
    // comes from. The dump probe lives at the retro_run present site in
    // emulator.rs, which captures the actual u32 slice passed to G_VIDEO.
}

/// Get readback pixels (called from main thread each frame).
/// Returns None if no frame has been rendered yet.
/// Clones instead of taking — keeps last frame available so missed
/// swaps don't flash to black (worker and main run at different rates).
pub fn take_readback() -> Option<Vec<u32>> {
    READBACK.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Copy guest framebuffer (XRGB8888) directly to the readback buffer.
/// Called from hle_swap when the guest writes frames via Lock/Unlock
/// (BIK video, legal splash, etc.) bypassing our GPU backend.
pub fn update_from_guest_framebuffer(guest_fb: &[u8]) {
    update_from_guest_framebuffer_ex(guest_fb, 640, 480, false);
}

/// Explicit-format variant. If `swizzled` is true AND width/height are both
/// power-of-2, runs `swizzle::unswizzle` before the BGRA→RGBA conversion.
/// For non-power-of-2 dimensions, `swizzled=true` silently falls through to
/// the linear path (matching swizzle::unswizzle's own guard). Spliced
/// 2026-04-20 from rustemu-jit; existing `update_from_guest_framebuffer`
/// call sites still take the linear path unchanged.
pub fn update_from_guest_framebuffer_ex(guest_fb: &[u8], width: u32, height: u32, swizzled: bool) {
    let pixel_count = (width * height) as usize;
    let mut unswizzled_buf: Vec<u8>;
    let source: &[u8] = if swizzled {
        match swizzle::unswizzle(guest_fb, width, height, 4) {
            Some(v) => {
                unswizzled_buf = v;
                &unswizzled_buf
            }
            None => guest_fb, // non-power-of-2 or invalid — skip silently
        }
    } else {
        guest_fb
    };
    let mut pixels = vec![0u32; pixel_count];
    // Guest framebuffer is XRGB8888 (4 bytes per pixel)
    let num = source.len().min(pixel_count * 4) / 4;
    for i in 0..num {
        let offset = i * 4;
        if offset + 3 < source.len() {
            // Xbox XRGB: B G R X → host XRGB: R G B X
            let b = source[offset] as u32;
            let g = source[offset + 1] as u32;
            let r = source[offset + 2] as u32;
            pixels[i] = (r << 16) | (g << 8) | b;
        }
    }
    store_readback(&pixels);
    set_frame_dirty();
}

/// Initialize the GPU backend. Try D3D11 first, then D3D9 (Cxbx-R style), then SWR.
/// Set RUSTEMU_GPU_BACKEND=swr for the diagnostic software rasterizer, d3d9 for
/// the Cxbx-R-style D3D9 canary, or d3d12 to force an exploratory D3D12 init first.
pub fn init_backend(width: i32, height: i32) -> BackendKind {
    let synth_training = crate::xbox::emulator::spidey_synth_training_enabled();
    let requested_raw = std::env::var("RUSTEMU_GPU_BACKEND")
        .ok()
        .map(|s| s.to_ascii_lowercase());
    let requested = if synth_training {
        if requested_raw.as_deref().is_some_and(|v| v != "d3d11") {
            crate::xbox::emulator::debug_log(&format!(
                "[GPU] RUSTEMU_SPIDEY_SYNTH_TRAINING overriding RUSTEMU_GPU_BACKEND={}",
                requested_raw.as_deref().unwrap_or_default()
            ));
        }
        Some("d3d11".to_string())
    } else {
        requested_raw
    };
    if requested.as_deref() == Some("swr") {
        let mut swr = swr::SwrBackend::new();
        swr.init(width, height);
        crate::xbox::emulator::debug_log(&format!(
            "[GPU] SWR backend forced by RUSTEMU_GPU_BACKEND=swr ({}x{})",
            width, height
        ));
        install_gpu(Box::new(swr));
        return BackendKind::Swr;
    }

    if requested.as_deref() == Some("d3d12")
        || (!synth_training && std::env::var_os("RUSTEMU_D3D12").is_some())
    {
        let mut d3d12 = d3d12::D3D12Backend::new();
        if d3d12.init(width, height) {
            crate::xbox::emulator::debug_log(&format!(
                "[GPU] D3D12 backend initialized ({}x{})",
                width, height
            ));
            install_gpu(Box::new(d3d12));
            return BackendKind::D3D12;
        }
        crate::xbox::emulator::debug_log("[GPU] D3D12 init failed, falling back to D3D11");
    }

    if requested.as_deref() == Some("d3d9") {
        let mut d3d9 = d3d9::D3D9Backend::new();
        if d3d9.init(width, height) {
            d3d9.bind_default_render_target();
            crate::xbox::emulator::debug_log(&format!(
                "[GPU] D3D9 backend forced by RUSTEMU_GPU_BACKEND=d3d9 ({}x{})",
                width, height
            ));
            install_gpu(Box::new(d3d9));
            return BackendKind::D3D9;
        }
        crate::xbox::emulator::debug_log("[GPU] forced D3D9 init failed, falling back to D3D11");
    }

    // Try D3D11 first — no NULL-HWND internal thread crash like D3D9
    let mut d3d11 = d3d11::D3D11Backend::new();
    if d3d11.init(width, height) {
        d3d11.bind_default_render_target();
        crate::xbox::emulator::debug_log(&format!(
            "[GPU] D3D11 backend initialized ({}x{}){}",
            width,
            height,
            if synth_training {
                " by RUSTEMU_SPIDEY_SYNTH_TRAINING"
            } else {
                ""
            }
        ));
        install_gpu(Box::new(d3d11));
        return BackendKind::D3D11;
    }
    crate::xbox::emulator::debug_log("[GPU] D3D11 init failed, trying D3D9");

    // Try D3D9 (matches Cxbx-R's Xbox D3D8 → host D3D9 approach)
    let mut d3d9 = d3d9::D3D9Backend::new();
    if d3d9.init(width, height) {
        d3d9.bind_default_render_target();
        crate::xbox::emulator::debug_log(&format!(
            "[GPU] D3D9 backend initialized ({}x{})",
            width, height
        ));
        install_gpu(Box::new(d3d9));
        return BackendKind::D3D9;
    }
    crate::xbox::emulator::debug_log("[GPU] D3D9 init failed, falling back to SWR");

    // Fall back to SWR
    let mut swr = swr::SwrBackend::new();
    swr.init(width, height);
    crate::xbox::emulator::debug_log(&format!(
        "[GPU] SWR backend initialized ({}x{})",
        width, height
    ));
    install_gpu(Box::new(swr));
    BackendKind::Swr
}
