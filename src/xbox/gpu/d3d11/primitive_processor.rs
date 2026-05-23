//! Primitive processor layer.
//!
//! Owns topology mapping and primitive expansion for the D3D11 backend. Keep
//! host-D3D11 topology values here; move backend-neutral Xbox primitive
//! semantics above the backend if D3D12 needs the same rule.

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::*;

use super::super::{NV2AVertex, VertexSkinPayload};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct D3D11DrawVertex {
    pub(super) pos: [f32; 4],
    pub(super) color: u32,
    pub(super) uv: [f32; 2],
    pub(super) blend_indices: [f32; 4],
    pub(super) blend_weights: [f32; 4],
    pub(super) normal: [f32; 4],
    pub(super) uv1: [f32; 4],
    pub(super) vregs: [[f32; 4]; 16],
}

#[derive(Clone, Copy, Default)]
pub(super) struct RecentDrawSummary {
    pub(super) draw_index: u32,
    pub(super) verts: u32,
    pub(super) prim_type: i32,
    pub(super) min_x: f32,
    pub(super) min_y: f32,
    pub(super) max_x: f32,
    pub(super) max_y: f32,
    pub(super) min_u: f32,
    pub(super) min_v: f32,
    pub(super) max_u: f32,
    pub(super) max_v: f32,
    pub(super) first_color: u32,
    pub(super) min_alpha: u8,
    pub(super) max_alpha: u8,
    pub(super) tex_handle: u32,
    pub(super) tex_guest_raw: u32,
    pub(super) tex_guest_norm: u32,
    pub(super) tex_data: u32,
    pub(super) tex_width: u32,
    pub(super) tex_height: u32,
    pub(super) tex_format: u32,
    pub(super) tex_guest_format: u32,
    pub(super) tex_source: &'static str,
    pub(super) tex_srv_bound: bool,
    pub(super) active_vs: u32,
    pub(super) blend: bool,
    pub(super) z_enable: bool,
    pub(super) z_write: bool,
    pub(super) color_write: u32,
    pub(super) alpha_test: bool,
    pub(super) alpha_ref: u32,
    pub(super) rt_key: u32,
}

impl D3D11DrawVertex {
    pub(super) fn from_nv2a(v: NV2AVertex, skin: VertexSkinPayload) -> Self {
        let pos = [v.x, v.y, v.z, v.w];
        let color = v.color;
        let uv = [v.u, v.v];
        let mut vregs = [[0.0, 0.0, 0.0, 1.0]; 16];
        vregs[0] = pos;
        vregs[1] = [
            ((color >> 16) & 255) as f32 / 255.0,
            ((color >> 8) & 255) as f32 / 255.0,
            (color & 255) as f32 / 255.0,
            ((color >> 24) & 255) as f32 / 255.0,
        ];
        vregs[2] = skin.normal;
        vregs[3] = [uv[0], uv[1], 0.0, 1.0];
        vregs[4] = skin.uv1;
        vregs[5] = skin.blend_indices;
        vregs[6] = skin.blend_weights;
        Self {
            pos,
            color,
            uv,
            blend_indices: skin.blend_indices,
            blend_weights: skin.blend_weights,
            normal: skin.normal,
            uv1: skin.uv1,
            vregs,
        }
    }
}

#[cfg(windows)]
pub(super) fn map_prim_type(nv097: i32) -> D3D_PRIMITIVE_TOPOLOGY {
    match nv097 {
        1 => D3D11_PRIMITIVE_TOPOLOGY_POINTLIST,
        2 => D3D11_PRIMITIVE_TOPOLOGY_LINELIST,
        3 | 4 => D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP,
        5 => D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        6 => D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
        7 => D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        super::super::NV097_QUAD_LIST => D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        _ => D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
    }
}

pub(super) fn expand_triangle_fan(verts: &[NV2AVertex]) -> Vec<NV2AVertex> {
    if verts.len() < 3 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity((verts.len() - 2) * 3);
    for i in 2..verts.len() {
        out.push(verts[0]);
        out.push(verts[i - 1]);
        out.push(verts[i]);
    }
    out
}

pub(super) fn expand_xbox_quad_list(verts: &[NV2AVertex]) -> Vec<NV2AVertex> {
    let quad_count = verts.len() / 4;
    if quad_count == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(quad_count * 6);
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

pub(super) fn expand_triangle_fan_skin(payload: &[VertexSkinPayload]) -> Vec<VertexSkinPayload> {
    if payload.len() < 3 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity((payload.len() - 2) * 3);
    for i in 2..payload.len() {
        out.push(payload[0]);
        out.push(payload[i - 1]);
        out.push(payload[i]);
    }
    out
}

pub(super) fn expand_xbox_quad_list_skin(payload: &[VertexSkinPayload]) -> Vec<VertexSkinPayload> {
    let quad_count = payload.len() / 4;
    if quad_count == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(quad_count * 6);
    for quad in payload.chunks_exact(4) {
        out.push(quad[0]);
        out.push(quad[1]);
        out.push(quad[2]);
        out.push(quad[2]);
        out.push(quad[3]);
        out.push(quad[0]);
    }
    out
}
