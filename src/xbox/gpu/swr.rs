/// Software rasterizer fallback — CPU-only, no external deps.
/// Renders NV2AVertex triangles/strips to an XRGB8888 framebuffer.
/// Used when D3D11 init fails (e.g., Xbox UWP dev mode).
use std::sync::{
    atomic::{AtomicU32, Ordering},
    OnceLock,
};

use super::{
    GpuBackend, NV2AVertex, NV097_QUAD_LIST, NV097_TRIANGLES, NV097_TRIANGLE_FAN,
    NV097_TRIANGLE_STRIP,
};

struct SwrTexture {
    width: u32,
    height: u32,
    pixels: Vec<u32>, // ARGB8888, row-major
}

pub struct SwrBackend {
    width: i32,
    height: i32,
    color_buf: Vec<u32>, // XRGB8888
    depth_buf: Vec<f32>, // per-pixel depth
    texture0: Option<SwrTexture>,
}

struct PixelProbeConfig {
    enabled: bool,
    x: i32,
    y: i32,
    max_logs: u32,
}

impl SwrBackend {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            color_buf: Vec::new(),
            depth_buf: Vec::new(),
            texture0: None,
        }
    }
}

impl GpuBackend for SwrBackend {
    fn init(&mut self, width: i32, height: i32) -> bool {
        self.width = width;
        self.height = height;
        let n = (width * height) as usize;
        self.color_buf = vec![0u32; n];
        self.depth_buf = vec![1.0f32; n];
        true
    }

    fn shutdown(&mut self) {
        self.color_buf.clear();
        self.depth_buf.clear();
        self.texture0 = None;
    }

    fn draw_primitive(&mut self, verts: &[NV2AVertex], prim_type: i32) {
        // Expand to triangle list
        let tris = expand_triangles(verts, prim_type);
        for tri in tris.chunks(3) {
            if tri.len() == 3 {
                self.rasterize_triangle(&tri[0], &tri[1], &tri[2]);
            }
        }
    }

    fn clear(&mut self, color: u32) {
        // color is ARGB32 → XRGB8888 (same layout, just ignore alpha)
        let xrgb = color & 0x00FF_FFFF;
        for p in self.color_buf.iter_mut() {
            *p = xrgb;
        }
        for d in self.depth_buf.iter_mut() {
            *d = 1.0;
        }
    }

    fn set_texture(&mut self, stage: u32, width: u32, height: u32, pixels: &[u32]) {
        if stage != 0 || width == 0 || height == 0 {
            return;
        }
        let expected = (width as usize).saturating_mul(height as usize);
        if pixels.len() < expected {
            return;
        }

        self.texture0 = Some(SwrTexture {
            width,
            height,
            pixels: pixels[..expected].to_vec(),
        });

        static LOG_N: AtomicU32 = AtomicU32::new(0);
        let n = LOG_N.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[SWR-TEX] #{} stage={} {}x{} pixels={} first=0x{:08X} mid=0x{:08X}",
                n,
                stage,
                width,
                height,
                expected,
                pixels.first().copied().unwrap_or(0),
                pixels.get(expected / 2).copied().unwrap_or(0)
            ));
        }
    }

    fn clear_texture(&mut self, stage: u32) {
        if stage == 0 {
            self.texture0 = None;
        }
    }

    fn readback_framebuffer(&mut self) -> &[u32] {
        &self.color_buf
    }

    fn name(&self) -> &'static str {
        "SWR"
    }
}

impl SwrBackend {
    fn rasterize_triangle(&mut self, v0: &NV2AVertex, v1: &NV2AVertex, v2: &NV2AVertex) {
        let w = self.width as f32;
        let h = self.height as f32;

        // Screen-space coords (Xbox uses 0..640, 0..480 already)
        let x0 = v0.x;
        let y0 = v0.y;
        let x1 = v1.x;
        let y1 = v1.y;
        let x2 = v2.x;
        let y2 = v2.y;

        // Bounding box
        let min_x = x0.min(x1).min(x2).max(0.0) as i32;
        let max_x = x0.max(x1).max(x2).min(w - 1.0) as i32;
        let min_y = y0.min(y1).min(y2).max(0.0) as i32;
        let max_y = y0.max(y1).max(y2).min(h - 1.0) as i32;

        let area = edge(x0, y0, x1, y1, x2, y2);
        if area.abs() < 0.001 {
            return; // degenerate
        }
        let inv_area = 1.0 / area;

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;

                let w0 = edge(x1, y1, x2, y2, fx, fy) * inv_area;
                let w1 = edge(x2, y2, x0, y0, fx, fy) * inv_area;
                let w2 = 1.0 - w0 - w1;

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let z = v0.z * w0 + v1.z * w1 + v2.z * w2;
                    let idx = (py * self.width + px) as usize;

                    if z <= self.depth_buf[idx] + 0.000001 {
                        self.depth_buf[idx] = z;
                        let c = if let Some(tex) = &self.texture0 {
                            let u = v0.u * w0 + v1.u * w1 + v2.u * w2;
                            let v = v0.v * w0 + v1.v * w1 + v2.v * w2;
                            let (sampled, tx, ty) = sample_texture_debug(tex, u, v);
                            maybe_log_pixel_probe(
                                px,
                                py,
                                min_x,
                                min_y,
                                max_x,
                                max_y,
                                z,
                                w0,
                                w1,
                                w2,
                                u,
                                v,
                                Some(tex),
                                tx,
                                ty,
                                sampled,
                            );
                            sampled
                        } else {
                            let color = lerp_color(v0.color, v1.color, v2.color, w0, w1, w2);
                            maybe_log_pixel_probe(
                                px, py, min_x, min_y, max_x, max_y, z, w0, w1, w2, 0.0, 0.0, None,
                                0, 0, color,
                            );
                            color
                        };
                        self.color_buf[idx] = c & 0x00FF_FFFF; // XRGB
                    }
                }
            }
        }
    }
}

fn sample_texture_debug(tex: &SwrTexture, u: f32, v: f32) -> (u32, i32, i32) {
    if tex.width == 0 || tex.height == 0 || tex.pixels.is_empty() {
        return (0, 0, 0);
    }

    let tx = ((u * tex.width as f32).floor() as i32).clamp(0, tex.width as i32 - 1);
    let ty = ((v * tex.height as f32).floor() as i32).clamp(0, tex.height as i32 - 1);
    (
        tex.pixels[(ty as u32 * tex.width + tx as u32) as usize],
        tx,
        ty,
    )
}

fn swr_pixel_probe_config() -> &'static PixelProbeConfig {
    static CONFIG: OnceLock<PixelProbeConfig> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let enabled = std::env::var_os("RUSTEMU_SWR_PIXEL_PROBE").is_some();
        let x = std::env::var("RUSTEMU_SWR_PIXEL_PROBE_X")
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(320);
        let y = std::env::var("RUSTEMU_SWR_PIXEL_PROBE_Y")
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(240);
        let max_logs = std::env::var("RUSTEMU_SWR_PIXEL_PROBE_MAX")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(64);
        PixelProbeConfig {
            enabled,
            x,
            y,
            max_logs,
        }
    })
}

#[allow(clippy::too_many_arguments)]
fn maybe_log_pixel_probe(
    px: i32,
    py: i32,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    z: f32,
    w0: f32,
    w1: f32,
    w2: f32,
    u: f32,
    v: f32,
    tex: Option<&SwrTexture>,
    tx: i32,
    ty: i32,
    sampled: u32,
) {
    let cfg = swr_pixel_probe_config();
    if !cfg.enabled || px != cfg.x || py != cfg.y {
        return;
    }

    static LOG_N: AtomicU32 = AtomicU32::new(0);
    let n = LOG_N.fetch_add(1, Ordering::Relaxed);
    if n >= cfg.max_logs {
        return;
    }

    let (textured, tex_width, tex_height) = tex
        .map(|tex| (true, tex.width, tex.height))
        .unwrap_or((false, 0, 0));
    crate::xbox::emulator::debug_log(&format!(
        "[SWR-PIXEL] #{} p=({}, {}) bbox=({}, {})-({}, {}) z={:.6} bary=({:.6},{:.6},{:.6}) textured={} uv=({:.6},{:.6}) tex={}x{} texel=({}, {}) sample=0x{:08X}",
        n,
        px,
        py,
        min_x,
        min_y,
        max_x,
        max_y,
        z,
        w0,
        w1,
        w2,
        textured,
        u,
        v,
        tex_width,
        tex_height,
        tx,
        ty,
        sampled
    ));
}

/// 2D edge function (cross product)
fn edge(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}

/// Barycentric color interpolation (ARGB32)
fn lerp_color(c0: u32, c1: u32, c2: u32, w0: f32, w1: f32, w2: f32) -> u32 {
    let r = ((c0 >> 16) & 0xFF) as f32 * w0
        + ((c1 >> 16) & 0xFF) as f32 * w1
        + ((c2 >> 16) & 0xFF) as f32 * w2;
    let g = ((c0 >> 8) & 0xFF) as f32 * w0
        + ((c1 >> 8) & 0xFF) as f32 * w1
        + ((c2 >> 8) & 0xFF) as f32 * w2;
    let b = (c0 & 0xFF) as f32 * w0 + (c1 & 0xFF) as f32 * w1 + (c2 & 0xFF) as f32 * w2;
    let a = ((c0 >> 24) & 0xFF) as f32 * w0
        + ((c1 >> 24) & 0xFF) as f32 * w1
        + ((c2 >> 24) & 0xFF) as f32 * w2;
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Expand primitive to triangle list
fn expand_triangles(verts: &[NV2AVertex], prim_type: i32) -> Vec<NV2AVertex> {
    match prim_type {
        NV097_TRIANGLES => verts.to_vec(),
        NV097_TRIANGLE_STRIP => {
            let mut out = Vec::new();
            for i in 2..verts.len() {
                if i % 2 == 0 {
                    out.push(verts[i - 2]);
                    out.push(verts[i - 1]);
                    out.push(verts[i]);
                } else {
                    out.push(verts[i - 1]);
                    out.push(verts[i - 2]);
                    out.push(verts[i]);
                }
            }
            out
        }
        NV097_TRIANGLE_FAN => {
            let mut out = Vec::new();
            for i in 2..verts.len() {
                out.push(verts[0]);
                out.push(verts[i - 1]);
                out.push(verts[i]);
            }
            out
        }
        NV097_QUAD_LIST => {
            let mut out = Vec::new();
            for quad in verts.chunks_exact(4) {
                out.push(quad[0]);
                out.push(quad[1]);
                out.push(quad[2]);
                out.push(quad[2]);
                out.push(quad[3]);
                out.push(quad[0]);
            }
            out
        }
        _ => verts.to_vec(), // fallback: treat as triangle list
    }
}
