// Auto-generated from Cxbx-R XbSymbolDatabase.
// Regenerate with: python3 tools/import_xbsymdb_database.py --xdk 3936
// SPDX source: XbSymbolDatabase OOVPADatabase .inl files.

#![allow(dead_code)]

use crate::xbox::aot::oovpa::{HleMode, OovpaEntry, OovpaPattern, OovpaPatternMeta, OovpaXref};

macro_rules! ov {
    ($off:expr, $val:expr) => {
        OovpaEntry {
            offset: $off,
            value: $val,
        }
    };
}

// Source: XGraphic/3911.inl
static XGRAPHIC_XGCOMPRESSRECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8D),
    ov!(0x0014, 0x83),
    ov!(0x0015, 0x7D),
    ov!(0x0016, 0x6C),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x56),
    ov!(0x0019, 0x57),
    ov!(0x001A, 0xC7),
    ov!(0x001B, 0x45),
    ov!(0x00C0, 0x7E),
    ov!(0x00C1, 0x01),
];
static XGRAPHIC_XGCOMPRESSRECT_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGISSWIZZLEDFORMAT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x7F),
    ov!(0x0010, 0x7C),
    ov!(0x0019, 0x0B),
    ov!(0x0022, 0xF8),
    ov!(0x002B, 0x83),
    ov!(0x0034, 0x0A),
    ov!(0x003D, 0x7F),
];
static XGRAPHIC_XGISSWIZZLEDFORMAT_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSETINDEXBUFFERHEADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x18),
    ov!(0x0008, 0xC7),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x01),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x01),
    ov!(0x000D, 0x00),
    ov!(0x0010, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x18),
];
static XGRAPHIC_XGSETINDEXBUFFERHEADER_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSETSURFACEHEADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x14),
    ov!(0x0007, 0x8D),
    ov!(0x0009, 0x10),
    ov!(0x000F, 0x6A),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x00),
];
static XGRAPHIC_XGSETSURFACEHEADER_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSETTEXTUREHEADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x75),
    ov!(0x000A, 0x00),
    ov!(0x0010, 0xFF),
    ov!(0x0016, 0xFF),
    ov!(0x001C, 0x75),
    ov!(0x0026, 0x5D),
    ov!(0x0028, 0x24),
];
static XGRAPHIC_XGSETTEXTUREHEADER_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSETVERTEXBUFFERHEADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0x8B),
    ov!(0x0007, 0x18),
    ov!(0x0008, 0xC7),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x01),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x89),
    ov!(0x0010, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x18),
];
static XGRAPHIC_XGSETVERTEXBUFFERHEADER_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSWIZZLEBOX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x75),
    ov!(0x003E, 0x4D),
    ov!(0x005E, 0x48),
    ov!(0x007E, 0x04),
    ov!(0x009E, 0xD8),
    ov!(0x00C0, 0x83),
    ov!(0x00DE, 0xAF),
    ov!(0x00FE, 0x45),
];
static XGRAPHIC_XGSWIZZLEBOX_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGSWIZZLERECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x03),
    ov!(0x003E, 0x89),
    ov!(0x005E, 0x83),
    ov!(0x007E, 0x6C),
    ov!(0x009E, 0xFF),
    ov!(0x00BE, 0xFF),
    ov!(0x00DE, 0x89),
    ov!(0x00FE, 0x89),
];
static XGRAPHIC_XGSWIZZLERECT_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGUNSWIZZLEBOX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x26),
    ov!(0x003E, 0x55),
    ov!(0x005E, 0x58),
    ov!(0x007E, 0x89),
    ov!(0x00A0, 0xFF),
    ov!(0x00BE, 0x2C),
    ov!(0x00DE, 0x24),
    ov!(0x00FE, 0x20),
];
static XGRAPHIC_XGUNSWIZZLEBOX_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGUNSWIZZLERECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x03),
    ov!(0x003E, 0x00),
    ov!(0x005E, 0xD2),
    ov!(0x007E, 0x75),
    ov!(0x009E, 0x70),
    ov!(0x00C1, 0xE9),
    ov!(0x00DE, 0x89),
    ov!(0x00FE, 0x60),
];
static XGRAPHIC_XGUNSWIZZLERECT_3911_XREFS: &[OovpaXref] = &[];

// Source: XGraphic/3911.inl
static XGRAPHIC_XGWRITESURFACEORTEXTURETOXPR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x08),
    ov!(0x001E, 0x3D),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x74),
    ov!(0x0024, 0x0A),
    ov!(0x0025, 0xB8),
    ov!(0x0026, 0x05),
    ov!(0x0027, 0x40),
    ov!(0x0028, 0x00),
    ov!(0x002E, 0x00),
    ov!(0x003E, 0xE0),
    ov!(0x0047, 0x03),
    ov!(0x005E, 0x75),
    ov!(0x007E, 0x33),
    ov!(0x009E, 0xC2),
    ov!(0x00AE, 0x4D),
    ov!(0x00BE, 0xF0),
];
static XGRAPHIC_XGWRITESURFACEORTEXTURETOXPR_3911_XREFS: &[OovpaXref] = &[];

pub const PATTERNS: &[OovpaPattern] = &[
    OovpaPattern {
        name: "XGCompressRect",
        detect_size: 0x00C2,
        entries: XGRAPHIC_XGCOMPRESSRECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGIsSwizzledFormat",
        detect_size: 0x003E,
        entries: XGRAPHIC_XGISSWIZZLEDFORMAT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSetIndexBufferHeader",
        detect_size: 0x0013,
        entries: XGRAPHIC_XGSETINDEXBUFFERHEADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSetSurfaceHeader",
        detect_size: 0x0013,
        entries: XGRAPHIC_XGSETSURFACEHEADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSetTextureHeader",
        detect_size: 0x0029,
        entries: XGRAPHIC_XGSETTEXTUREHEADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSetVertexBufferHeader",
        detect_size: 0x0013,
        entries: XGRAPHIC_XGSETVERTEXBUFFERHEADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSwizzleBox",
        detect_size: 0x00FF,
        entries: XGRAPHIC_XGSWIZZLEBOX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGSwizzleRect",
        detect_size: 0x00FF,
        entries: XGRAPHIC_XGSWIZZLERECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGUnswizzleBox",
        detect_size: 0x00FF,
        entries: XGRAPHIC_XGUNSWIZZLEBOX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGUnswizzleRect",
        detect_size: 0x00FF,
        entries: XGRAPHIC_XGUNSWIZZLERECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGWriteSurfaceOrTextureToXPR",
        detect_size: 0x00BF,
        entries: XGRAPHIC_XGWRITESURFACEORTEXTURETOXPR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
];

pub const METADATA: &[OovpaPatternMeta] = &[
    OovpaPatternMeta {
        name: "XGCompressRect",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGCOMPRESSRECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGIsSwizzledFormat",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGISSWIZZLEDFORMAT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSetIndexBufferHeader",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSETINDEXBUFFERHEADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSetSurfaceHeader",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSETSURFACEHEADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSetTextureHeader",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSETTEXTUREHEADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSetVertexBufferHeader",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSETVERTEXBUFFERHEADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSwizzleBox",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSWIZZLEBOX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGSwizzleRect",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGSWIZZLERECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGUnswizzleBox",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGUNSWIZZLEBOX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGUnswizzleRect",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGUNSWIZZLERECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGWriteSurfaceOrTextureToXPR",
        min_version: 3911,
        source_file: "XGraphic/3911.inl",
        xrefs: XGRAPHIC_XGWRITESURFACEORTEXTURETOXPR_3911_XREFS,
    },
];
