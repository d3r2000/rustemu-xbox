//! D3D8 Flexible Vertex Format (FVF) decoder.
//!
//! Ported from Cxbx-Reloaded's `XbD3D8Types.h` (FVF bit constants) and
//! `XbConvert.cpp` (primitive-type mappings). This module contains **no
//! dependency on the emulator or any GPU backend** — pure bitfield math +
//! lookup tables, fully unit-testable.
//!
//! Foundation for `nv2a_pb.rs` draw-method decoding and the eventual
//! `backend.draw_primitive()` wiring. Next session will take the output of
//! these functions and build host D3D11/D3D12 input layouts from them.

// ============================================================================
// FVF bitfield constants (Xbox D3D8 layout — identical to PC D3D8 where
// overlap exists; some slots differ.)
// Source: Cxbx-Reloaded/src/core/hle/D3D8/XbD3D8Types.h:1282-1312
// ============================================================================

pub mod fvf {
    // Position type (bits 0-3, mask 0x00E — Xbox adds XYZB4 which PC doesn't
    // have at 0x00C; we match Xbox layout verbatim).
    pub const POSITION_MASK: u32 = 0x00E;
    pub const XYZ: u32 = 0x002; // 3 floats
    pub const XYZRHW: u32 = 0x004; // 4 floats (pre-transformed)
    pub const XYZB1: u32 = 0x006; // XYZ + 1 blend weight
    pub const XYZB2: u32 = 0x008; // XYZ + 2 blend weights
    pub const XYZB3: u32 = 0x00A; // XYZ + 3 blend weights
    pub const XYZB4: u32 = 0x00C; // XYZ + 4 blend weights (Xbox-only)

    pub const NORMAL: u32 = 0x010; // 3 floats
    pub const DIFFUSE: u32 = 0x040; // 1 DWORD ARGB
    pub const SPECULAR: u32 = 0x080; // 1 DWORD ARGB

    // Texture count in bits 8-11, mask 0xF00. The bits encode N where
    // TEX_N means N texture coordinate sets.
    pub const TEXCOUNT_MASK: u32 = 0xF00;
    pub const TEXCOUNT_SHIFT: u32 = 8;
    pub const TEX0: u32 = 0x000;
    pub const TEX1: u32 = 0x100;
    pub const TEX2: u32 = 0x200;
    pub const TEX3: u32 = 0x300;
    pub const TEX4: u32 = 0x400;
    pub const TEX5: u32 = 0x500;
    pub const TEX6: u32 = 0x600;
    pub const TEX7: u32 = 0x700;
    pub const TEX8: u32 = 0x800;

    /// Per-texcoord-set size encoding lives in bits >= 16, two bits per set:
    ///   0 → 2 floats (default)
    ///   1 → 3 floats
    ///   2 → 4 floats
    ///   3 → 1 float
    /// The shift is `index * 2 + 16` so texture set N occupies bits
    /// 16+2N and 17+2N.
    pub const TEXCOORDSIZE_SHIFT_BASE: u32 = 16;
}

/// Semantic tag for each attribute we emit into the input-layout list.
///
/// Matches the enum values `X_D3DVSDE_*` in XbD3D8Types.h:1183+, which
/// correspond to NV2A vertex stream attribute slot numbers 0..15.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VertexSemantic {
    Position = 0,
    BlendWeight = 1,
    Normal = 2,
    Diffuse = 3,
    Specular = 4,
    // 5..8 are reserved (BACKDIFFUSE, BACKSPECULAR, FOG, POINTSIZE).
    TexCoord0 = 9,
    TexCoord1 = 10,
    TexCoord2 = 11,
    TexCoord3 = 12,
}

/// One element of a decoded vertex layout.
///
/// `offset` is the byte offset of this attribute relative to the start
/// of the vertex. `format` is a descriptive enum of the wire format so
/// the backend can translate to its native vertex-element descriptor
/// (DXGI_FORMAT for D3D11, VkFormat for Vulkan, etc.).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexElement {
    pub semantic: VertexSemantic,
    pub offset: u32,
    pub format: VertexFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    /// 3 × f32 = 12 bytes (XYZ, NORMAL).
    Float3,
    /// 4 × f32 = 16 bytes (XYZRHW, position+blendweights where N=1 → XYZ+W).
    Float4,
    /// N × f32 = N*4 bytes. Used for blend weights and texture coords with
    /// explicit count.
    FloatN(u8),
    /// 1 × u32 = 4 bytes. BGRA color (D3D8 D3DCOLOR is ARGB stored in little-
    /// endian so on-wire order is B,G,R,A).
    Color,
}

impl VertexFormat {
    pub fn size_bytes(self) -> u32 {
        match self {
            VertexFormat::Float3 => 12,
            VertexFormat::Float4 => 16,
            VertexFormat::FloatN(n) => (n as u32) * 4,
            VertexFormat::Color => 4,
        }
    }
}

/// Decode an FVF bitfield into an ordered list of vertex elements with
/// computed byte offsets and a total stride.
///
/// The order and relative offsets are fixed by the D3D8 FVF layout rules:
///
///   1. Position (XYZ / XYZRHW / XYZBn)
///   2. Blend weights (if XYZBn, n weights as FloatN)
///   3. Normal (if set)
///   4. Diffuse color (if set)
///   5. Specular color (if set)
///   6. Texture coordinate sets 0..N-1, each sized by the per-set bits
///
/// Returns (elements, stride_bytes). Returns `None` only if the FVF is
/// structurally invalid (e.g. NORMAL set with a pre-transformed XYZRHW
/// position, which would be rejected by the real D3D runtime).
pub fn decode_fvf(fvf: u32) -> Option<(Vec<VertexElement>, u32)> {
    let mut elements = Vec::new();
    let mut offset: u32 = 0;

    let position_bits = fvf & fvf::POSITION_MASK;
    let has_normal = fvf & fvf::NORMAL != 0;
    let has_diffuse = fvf & fvf::DIFFUSE != 0;
    let has_specular = fvf & fvf::SPECULAR != 0;
    let texcount = ((fvf & fvf::TEXCOUNT_MASK) >> fvf::TEXCOUNT_SHIFT) as u8;

    // Position and blend weights.
    let (pos_format, blend_count): (VertexFormat, u8) = match position_bits {
        fvf::XYZ => (VertexFormat::Float3, 0),
        fvf::XYZRHW => {
            // XYZRHW is pre-transformed; normals and lighting are meaningless.
            // The real runtime rejects the combination.
            if has_normal {
                return None;
            }
            (VertexFormat::Float4, 0)
        }
        fvf::XYZB1 => (VertexFormat::Float3, 1),
        fvf::XYZB2 => (VertexFormat::Float3, 2),
        fvf::XYZB3 => (VertexFormat::Float3, 3),
        fvf::XYZB4 => (VertexFormat::Float3, 4),
        0 => return None, // no position → invalid
        _ => return None, // reserved / unknown
    };

    elements.push(VertexElement {
        semantic: VertexSemantic::Position,
        offset,
        format: pos_format,
    });
    offset += pos_format.size_bytes();

    if blend_count > 0 {
        elements.push(VertexElement {
            semantic: VertexSemantic::BlendWeight,
            offset,
            format: VertexFormat::FloatN(blend_count),
        });
        offset += (blend_count as u32) * 4;
    }

    if has_normal {
        elements.push(VertexElement {
            semantic: VertexSemantic::Normal,
            offset,
            format: VertexFormat::Float3,
        });
        offset += 12;
    }

    if has_diffuse {
        elements.push(VertexElement {
            semantic: VertexSemantic::Diffuse,
            offset,
            format: VertexFormat::Color,
        });
        offset += 4;
    }

    if has_specular {
        elements.push(VertexElement {
            semantic: VertexSemantic::Specular,
            offset,
            format: VertexFormat::Color,
        });
        offset += 4;
    }

    // Texture coordinate sets. Per-set size comes from bits at shift
    // (index * 2 + 16), 2 bits each:
    //   0 → 2 floats, 1 → 3 floats, 2 → 4 floats, 3 → 1 float.
    if texcount > 4 {
        return None;
    } // we only tag TexCoord0..3
    for i in 0..texcount {
        let shift = i as u32 * 2 + fvf::TEXCOORDSIZE_SHIFT_BASE;
        let code = (fvf >> shift) & 0x3;
        let n_floats: u8 = match code {
            0 => 2,
            1 => 3,
            2 => 4,
            3 => 1,
            _ => unreachable!(),
        };
        let sem = match i {
            0 => VertexSemantic::TexCoord0,
            1 => VertexSemantic::TexCoord1,
            2 => VertexSemantic::TexCoord2,
            3 => VertexSemantic::TexCoord3,
            _ => return None,
        };
        elements.push(VertexElement {
            semantic: sem,
            offset,
            format: VertexFormat::FloatN(n_floats),
        });
        offset += (n_floats as u32) * 4;
    }

    Some((elements, offset))
}

// ============================================================================
// Primitive type mapping.
// Source: Cxbx-Reloaded/src/core/hle/D3D8/XbConvert.cpp:1197-1224.
// ============================================================================

/// Xbox D3D primitive types. Values 0..10 match Xbox's enum verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum XboxPrimitive {
    None = 0,
    PointList = 1,
    LineList = 2,
    LineLoop = 3, // Xbox-only
    LineStrip = 4,
    TriangleList = 5,
    TriangleStrip = 6,
    TriangleFan = 7,
    QuadList = 8,  // Xbox-only
    QuadStrip = 9, // Xbox-only
    Polygon = 10,  // Xbox-only
}

impl XboxPrimitive {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            0 => XboxPrimitive::None,
            1 => XboxPrimitive::PointList,
            2 => XboxPrimitive::LineList,
            3 => XboxPrimitive::LineLoop,
            4 => XboxPrimitive::LineStrip,
            5 => XboxPrimitive::TriangleList,
            6 => XboxPrimitive::TriangleStrip,
            7 => XboxPrimitive::TriangleFan,
            8 => XboxPrimitive::QuadList,
            9 => XboxPrimitive::QuadStrip,
            10 => XboxPrimitive::Polygon,
            _ => return None,
        })
    }
}

/// Host topology, agnostic of backend API. Mapped to D3D11_PRIMITIVE_TOPOLOGY
/// / D3DPRIMITIVETYPE / VkPrimitiveTopology at the backend boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
}

/// Translation plan for Xbox primitives that have no direct host equivalent.
///
/// Xbox extensions (LineLoop, QuadList, QuadStrip, Polygon) need index-buffer
/// patching to render on host D3D. The `Direct` variant fast-paths native
/// types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTranslation {
    /// Host topology matches Xbox topology directly. Pass vertex count and
    /// vertex/index stream through unchanged.
    Direct { host: HostTopology },
    /// Need to convert N-vertex Xbox quads to 6-index host triangles.
    /// Triangulates each quad as two triangles sharing an edge.
    QuadListToTriangleList,
    /// Xbox quad strip → triangle strip with index re-order.
    QuadStripToTriangleStrip,
    /// Xbox line loop → line strip + closing segment; caller needs to
    /// re-emit first vertex index at end.
    LineLoopToLineStrip,
    /// Xbox polygon (convex) → triangle fan.
    PolygonToTriangleFan,
}

/// Map an Xbox primitive type to a host translation plan.
///
/// Matches Cxbx-Reloaded's `g_XboxPrimitiveTypeToHost` table
/// (XbConvert.cpp:1211-1225). Xbox-only primitives are marked with the
/// translation variant they need.
pub fn translate_primitive(p: XboxPrimitive) -> Option<PrimitiveTranslation> {
    Some(match p {
        XboxPrimitive::None => return None,
        XboxPrimitive::PointList => PrimitiveTranslation::Direct {
            host: HostTopology::PointList,
        },
        XboxPrimitive::LineList => PrimitiveTranslation::Direct {
            host: HostTopology::LineList,
        },
        XboxPrimitive::LineLoop => PrimitiveTranslation::LineLoopToLineStrip,
        XboxPrimitive::LineStrip => PrimitiveTranslation::Direct {
            host: HostTopology::LineStrip,
        },
        XboxPrimitive::TriangleList => PrimitiveTranslation::Direct {
            host: HostTopology::TriangleList,
        },
        XboxPrimitive::TriangleStrip => PrimitiveTranslation::Direct {
            host: HostTopology::TriangleStrip,
        },
        XboxPrimitive::TriangleFan => PrimitiveTranslation::Direct {
            host: HostTopology::TriangleFan,
        },
        XboxPrimitive::QuadList => PrimitiveTranslation::QuadListToTriangleList,
        XboxPrimitive::QuadStrip => PrimitiveTranslation::QuadStripToTriangleStrip,
        XboxPrimitive::Polygon => PrimitiveTranslation::PolygonToTriangleFan,
    })
}

/// Given an Xbox primitive and Xbox vertex count, compute the equivalent host
/// vertex count after translation. Needed for the caller to size host
/// index/vertex buffers when Xbox-only primitives get expanded.
///
/// Source logic: Cxbx-Reloaded's `g_XboxPrimitiveTypeInfo` table
/// (XbConvert.cpp:1197-1207) encodes `(vertex_overhead, vertices_per_prim)`
/// for each type. Primitive count = (xbox_vertex_count - overhead) /
/// vertices_per_prim. We use it here inverted to compute host vertex count.
pub fn host_vertex_count(p: XboxPrimitive, xbox_vertex_count: u32) -> u32 {
    if xbox_vertex_count == 0 {
        return 0;
    }
    match p {
        // Xbox-native → host-native: vertex count is unchanged.
        XboxPrimitive::None => 0,
        XboxPrimitive::PointList => xbox_vertex_count,
        XboxPrimitive::LineList => xbox_vertex_count,
        XboxPrimitive::LineStrip => xbox_vertex_count,
        XboxPrimitive::TriangleList => xbox_vertex_count,
        XboxPrimitive::TriangleStrip => xbox_vertex_count,
        XboxPrimitive::TriangleFan => xbox_vertex_count,

        // LineLoop → LineStrip: need to re-emit first vertex as last.
        XboxPrimitive::LineLoop => xbox_vertex_count + 1,

        // QuadList (N quads = 4N vertices) → TriangleList (6N indices).
        // Host vertex count stays the same (indexed draw); callers handle
        // indices separately.
        XboxPrimitive::QuadList => xbox_vertex_count,

        // QuadStrip: N quads = 2N+2 vertices → TriangleStrip 2N+2 vertices
        // with vertex index reorder. Same count.
        XboxPrimitive::QuadStrip => xbox_vertex_count,

        // Polygon → TriangleFan: same vertex count.
        XboxPrimitive::Polygon => xbox_vertex_count,
    }
}

// ============================================================================
// Unit tests — runnable via `cargo test --lib fvf_decode`.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xyz_only() {
        let (elems, stride) = decode_fvf(fvf::XYZ).unwrap();
        assert_eq!(stride, 12);
        assert_eq!(elems.len(), 1);
        assert_eq!(elems[0].semantic, VertexSemantic::Position);
        assert_eq!(elems[0].format, VertexFormat::Float3);
        assert_eq!(elems[0].offset, 0);
    }

    #[test]
    fn xyz_normal_diffuse_tex1() {
        // A common static mesh vertex: pos + normal + color + 1 texcoord.
        // 12 (pos) + 12 (normal) + 4 (diffuse) + 8 (tex0, 2 floats) = 36.
        let fvf_bits = fvf::XYZ | fvf::NORMAL | fvf::DIFFUSE | fvf::TEX1;
        let (elems, stride) = decode_fvf(fvf_bits).unwrap();
        assert_eq!(stride, 36);
        assert_eq!(elems.len(), 4);
        assert_eq!(elems[0].semantic, VertexSemantic::Position);
        assert_eq!(elems[0].offset, 0);
        assert_eq!(elems[1].semantic, VertexSemantic::Normal);
        assert_eq!(elems[1].offset, 12);
        assert_eq!(elems[2].semantic, VertexSemantic::Diffuse);
        assert_eq!(elems[2].offset, 24);
        assert_eq!(elems[3].semantic, VertexSemantic::TexCoord0);
        assert_eq!(elems[3].offset, 28);
        assert_eq!(elems[3].format, VertexFormat::FloatN(2));
    }

    // Handy FVF constant: 1 texture set.
    const TEX1: u32 = fvf::TEX1;

    #[test]
    fn xyzrhw_tex1() {
        // Pre-transformed screen-space quad with 1 texcoord:
        // 16 (XYZRHW) + 8 (tex0 2-float) = 24.
        let bits = fvf::XYZRHW | TEX1;
        let (elems, stride) = decode_fvf(bits).unwrap();
        assert_eq!(stride, 24);
        assert_eq!(elems[0].format, VertexFormat::Float4);
    }

    #[test]
    fn xyzrhw_rejects_normal() {
        // Normal + XYZRHW is semantically invalid — runtime rejects it.
        assert!(decode_fvf(fvf::XYZRHW | fvf::NORMAL).is_none());
    }

    #[test]
    fn skinned_mesh_xyzb3() {
        // XYZB3 = XYZ + 3 blend weights = 12 + 12 = 24 bytes.
        // Plus normal + diffuse + tex1 (2-float) → 24+12+4+8 = 48.
        let bits = fvf::XYZB3 | fvf::NORMAL | fvf::DIFFUSE | fvf::TEX1;
        let (elems, stride) = decode_fvf(bits).unwrap();
        assert_eq!(stride, 48);
        assert_eq!(elems[0].semantic, VertexSemantic::Position);
        assert_eq!(elems[0].format, VertexFormat::Float3);
        assert_eq!(elems[1].semantic, VertexSemantic::BlendWeight);
        assert_eq!(elems[1].format, VertexFormat::FloatN(3));
        assert_eq!(elems[1].offset, 12);
    }

    #[test]
    fn texcoord_size_override_3f() {
        // TEX1 with the per-set size field forced to 1 (= 3 floats).
        let override_bits = 1u32 << (fvf::TEXCOORDSIZE_SHIFT_BASE); // i=0, code=1
        let bits = fvf::XYZ | TEX1 | override_bits;
        let (elems, stride) = decode_fvf(bits).unwrap();
        // 12 (XYZ) + 12 (tex0 as 3 floats) = 24.
        assert_eq!(stride, 24);
        assert_eq!(elems[1].format, VertexFormat::FloatN(3));
    }

    #[test]
    fn two_texcoords_different_sizes() {
        // TEX2, set 0 default (2 floats), set 1 forced to 4 floats (code=2).
        let override_bits = 2u32 << (fvf::TEXCOORDSIZE_SHIFT_BASE + 2); // i=1, code=2
        let bits = fvf::XYZ | fvf::TEX2 | override_bits;
        let (elems, stride) = decode_fvf(bits).unwrap();
        // 12 (XYZ) + 8 (tex0 2f) + 16 (tex1 4f) = 36.
        assert_eq!(stride, 36);
        assert_eq!(elems.len(), 3);
        assert_eq!(elems[1].format, VertexFormat::FloatN(2));
        assert_eq!(elems[2].format, VertexFormat::FloatN(4));
    }

    #[test]
    fn semantic_values_match_xbox_vertex_declaration_slots() {
        assert_eq!(VertexSemantic::Position as u8, 0);
        assert_eq!(VertexSemantic::BlendWeight as u8, 1);
        assert_eq!(VertexSemantic::Normal as u8, 2);
        assert_eq!(VertexSemantic::Diffuse as u8, 3);
        assert_eq!(VertexSemantic::Specular as u8, 4);
        assert_eq!(VertexSemantic::TexCoord0 as u8, 9);
        assert_eq!(VertexSemantic::TexCoord1 as u8, 10);
        assert_eq!(VertexSemantic::TexCoord2 as u8, 11);
        assert_eq!(VertexSemantic::TexCoord3 as u8, 12);
    }

    #[test]
    fn full_fixed_function_layout_offsets_and_stride() {
        let tex1_as_1f = 3u32 << (fvf::TEXCOORDSIZE_SHIFT_BASE + 2);
        let bits = fvf::XYZ | fvf::NORMAL | fvf::DIFFUSE | fvf::SPECULAR | fvf::TEX2 | tex1_as_1f;
        let (elems, stride) = decode_fvf(bits).unwrap();

        assert_eq!(stride, 44);
        assert_eq!(
            elems,
            vec![
                VertexElement {
                    semantic: VertexSemantic::Position,
                    offset: 0,
                    format: VertexFormat::Float3,
                },
                VertexElement {
                    semantic: VertexSemantic::Normal,
                    offset: 12,
                    format: VertexFormat::Float3,
                },
                VertexElement {
                    semantic: VertexSemantic::Diffuse,
                    offset: 24,
                    format: VertexFormat::Color,
                },
                VertexElement {
                    semantic: VertexSemantic::Specular,
                    offset: 28,
                    format: VertexFormat::Color,
                },
                VertexElement {
                    semantic: VertexSemantic::TexCoord0,
                    offset: 32,
                    format: VertexFormat::FloatN(2),
                },
                VertexElement {
                    semantic: VertexSemantic::TexCoord1,
                    offset: 40,
                    format: VertexFormat::FloatN(1),
                },
            ]
        );
    }

    #[test]
    fn xyzb4_layout_uses_four_blend_weights_not_stale_stride() {
        let bits = fvf::XYZB4 | fvf::NORMAL | fvf::DIFFUSE | fvf::TEX1;
        let (elems, stride) = decode_fvf(bits).unwrap();

        assert_eq!(stride, 52);
        assert_eq!(elems[1].semantic, VertexSemantic::BlendWeight);
        assert_eq!(elems[1].offset, 12);
        assert_eq!(elems[1].format, VertexFormat::FloatN(4));
        assert_eq!(elems[2].semantic, VertexSemantic::Normal);
        assert_eq!(elems[2].offset, 28);
    }

    #[test]
    fn unused_texcoord_size_bits_do_not_create_slots_or_change_stride() {
        let stale_texcoord_bits = 0xFFFF_0000;
        let (elems, stride) = decode_fvf(fvf::XYZ | stale_texcoord_bits).unwrap();

        assert_eq!(stride, 12);
        assert_eq!(elems.len(), 1);
        assert_eq!(elems[0].semantic, VertexSemantic::Position);
    }

    #[test]
    fn rejects_unsupported_texcoord_slots_instead_of_zero_declaration() {
        assert!(decode_fvf(fvf::XYZ | fvf::TEX5).is_none());
        assert!(decode_fvf(fvf::XYZ | fvf::TEX8).is_none());
    }

    #[test]
    fn invalid_no_position() {
        assert!(decode_fvf(fvf::NORMAL | fvf::DIFFUSE).is_none());
    }

    #[test]
    fn primitive_type_roundtrip() {
        for v in 0u32..=10 {
            let p = XboxPrimitive::from_u32(v).unwrap();
            let _ = translate_primitive(p); // must not panic
        }
        assert!(XboxPrimitive::from_u32(11).is_none());
    }

    #[test]
    fn primitive_direct_mappings() {
        assert_eq!(
            translate_primitive(XboxPrimitive::TriangleList),
            Some(PrimitiveTranslation::Direct {
                host: HostTopology::TriangleList
            })
        );
        assert_eq!(
            translate_primitive(XboxPrimitive::TriangleStrip),
            Some(PrimitiveTranslation::Direct {
                host: HostTopology::TriangleStrip
            })
        );
    }

    #[test]
    fn primitive_xbox_extensions() {
        assert_eq!(
            translate_primitive(XboxPrimitive::QuadList),
            Some(PrimitiveTranslation::QuadListToTriangleList)
        );
        assert_eq!(
            translate_primitive(XboxPrimitive::LineLoop),
            Some(PrimitiveTranslation::LineLoopToLineStrip)
        );
        assert_eq!(
            translate_primitive(XboxPrimitive::Polygon),
            Some(PrimitiveTranslation::PolygonToTriangleFan)
        );
    }

    #[test]
    fn host_vertex_count_line_loop() {
        // LineLoop(N) → LineStrip with N+1 vertices (first re-emitted at end).
        assert_eq!(host_vertex_count(XboxPrimitive::LineLoop, 4), 5);
    }

    #[test]
    fn host_vertex_count_triangle_list_unchanged() {
        assert_eq!(host_vertex_count(XboxPrimitive::TriangleList, 300), 300);
    }
}

// Helper FVF constant re-exports for callers that don't want the verbose
// `super::fvf::TEX1` path.
#[allow(dead_code)]
pub const X_FVF_XYZ: u32 = fvf::XYZ;
#[allow(dead_code)]
pub const X_FVF_NORMAL: u32 = fvf::NORMAL;
#[allow(dead_code)]
pub const X_FVF_DIFFUSE: u32 = fvf::DIFFUSE;
#[allow(dead_code)]
pub const X_FVF_TEX1: u32 = fvf::TEX1;
