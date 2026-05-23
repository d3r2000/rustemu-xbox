// Auto-generated from Cxbx-R XbSymbolDatabase.
// Regenerate with: python3 tools/import_xbsymdb_database.py --xdk 3911
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

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_BLOCKUNTILVERTICALBLANK_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x6A),
    ov!(0x0006, 0x00),
    ov!(0x0007, 0x6A),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x6A),
    ov!(0x000A, 0x01),
    ov!(0x000B, 0xC7),
    ov!(0x000C, 0x80),
    ov!(0x0015, 0x6A),
    ov!(0x0016, 0x06),
    ov!(0x0017, 0x05),
    ov!(0x001D, 0xFF),
    ov!(0x001E, 0x15),
    ov!(0x0023, 0xA1),
];
static D3D8_LTCG_D3DDEVICE_BLOCKUNTILVERTICALBLANK_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3DDevice__m_VerticalBlankEvent_OFFSET",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_CREATEVERTEXSHADER_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x6C),
    ov!(0x000C, 0x0F),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x18),
    ov!(0x001C, 0x33),
    ov!(0x001D, 0xF6),
    ov!(0x001E, 0x85),
    ov!(0x001F, 0xED),
];
static D3D8_LTCG_D3DDEVICE_CREATEVERTEXSHADER_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x08),
    ov!(0x0006, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x000D, 0x8B),
    ov!(0x0015, 0x89),
    ov!(0x0016, 0x75),
    ov!(0x0017, 0xF8),
    ov!(0x0018, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0009,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0019,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x3D),
    ov!(0x0010, 0x89),
    ov!(0x0012, 0xF8),
    ov!(0x0013, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0014,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_PRESENT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0003, 0x56),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x35),
    ov!(0x000B, 0x6A),
    ov!(0x000C, 0x02),
    ov!(0x000D, 0xE8),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x86),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x8E),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x00),
];
static D3D8_LTCG_D3DDEVICE_PRESENT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x0D),
    ov!(0x000A, 0xA3),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0x4C),
    ov!(0x0011, 0x24),
    ov!(0x0012, 0x04),
    ov!(0x0013, 0xE9),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0006,
        target: "D3DRS_FillMode",
    },
    OovpaXref {
        offset: 0x000B,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x0014,
        target: "D3DDevice_SetRenderState_FillMode",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0012, 0x4C),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x20),
    ov!(0x001C, 0x03),
    ov!(0x001D, 0x08),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0x89),
    ov!(0x0020, 0x48),
    ov!(0x0021, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0015, 0x8B),
    ov!(0x001B, 0x8B),
    ov!(0x002B, 0xC7),
    ov!(0x002C, 0x00),
    ov!(0x002D, 0x8C),
    ov!(0x002E, 0x03),
    ov!(0x002F, 0x08),
    ov!(0x0030, 0x00),
    ov!(0x0037, 0x83),
    ov!(0x0038, 0xC0),
    ov!(0x0039, 0x0C),
    ov!(0x003C, 0x89),
    ov!(0x0043, 0xC2),
    ov!(0x0044, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0017,
        target: "D3DRS_TwoSidedLighting",
    },
    OovpaXref {
        offset: 0x001D,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x003E,
        target: "D3DRS_FillMode",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001C, 0x54),
    ov!(0x001D, 0x24),
    ov!(0x001E, 0x0E),
    ov!(0x001F, 0x8B),
    ov!(0x0020, 0xF9),
    ov!(0x0021, 0x81),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0022, 0x83),
    ov!(0x0023, 0xC0),
    ov!(0x0024, 0x08),
    ov!(0x002C, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x002E,
        target: "D3DRS_FrontFace",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x5C),
    ov!(0x0011, 0x44),
    ov!(0x0012, 0x24),
    ov!(0x0013, 0x0C),
    ov!(0x0014, 0x57),
    ov!(0x0015, 0xD8),
    ov!(0x0016, 0x8E),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_1024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x005B,
    target: "D3DRS_LineWidth",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001B, 0x75),
    ov!(0x001D, 0xC7),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0xBC),
    ov!(0x0020, 0x17),
    ov!(0x0021, 0x04),
    ov!(0x0022, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x002D,
        target: "D3DRS_LogicOp",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x000B, 0xA3),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_MultiSampleAntiAlias",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x000B, 0xA3),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_MultiSampleMask",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001B, 0xA4),
    ov!(0x001C, 0x03),
    ov!(0x001D, 0x04),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0x89),
    ov!(0x0020, 0x48),
    ov!(0x0021, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x001B, 0x6C),
    ov!(0x001C, 0x1E),
    ov!(0x001D, 0x04),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0x8D),
    ov!(0x0020, 0x91),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x006D, 0xC7),
    ov!(0x006F, 0x08),
    ov!(0x0070, 0x2C),
    ov!(0x0071, 0x03),
    ov!(0x0072, 0x04),
    ov!(0x0073, 0x00),
    ov!(0x007C, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x007E,
        target: "D3DRS_StencilEnable",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0056, 0xC7),
    ov!(0x0058, 0x08),
    ov!(0x0059, 0x70),
    ov!(0x005A, 0x03),
    ov!(0x005B, 0x04),
    ov!(0x005C, 0x00),
    ov!(0x0065, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0067,
        target: "D3DRS_StencilFail",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1024_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xA1), ov!(0x0009, 0x8B), ov!(0x0014, 0xA3)];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0015,
        target: "D3DRS_TwoSidedLighting",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x56),
    ov!(0x000C, 0x0F),
    ov!(0x000D, 0x95),
    ov!(0x000E, 0xC0),
    ov!(0x0015, 0xDB),
    ov!(0x0016, 0x44),
    ov!(0x0017, 0x24),
    ov!(0x0018, 0x14),
    ov!(0x001B, 0x7D),
    ov!(0x001C, 0x06),
    ov!(0x001D, 0xD8),
    ov!(0x001E, 0x05),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x002F, 0xC9),
    ov!(0x0030, 0xC7),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x0C),
    ov!(0x0033, 0x03),
    ov!(0x0034, 0x04),
    ov!(0x0035, 0x00),
    ov!(0x0036, 0x89),
    ov!(0x0037, 0x48),
    ov!(0x0038, 0x04),
    ov!(0x0039, 0x83),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0013, 0x75),
    ov!(0x0014, 0x44),
    ov!(0x0015, 0x8B),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x0B),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x8B),
    ov!(0x001D, 0x08),
    ov!(0x001E, 0x0B),
    ov!(0x001F, 0x00),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x8B),
    ov!(0x0023, 0x04),
    ov!(0x0024, 0x0B),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x06),
    ov!(0x0027, 0x89),
    ov!(0x0028, 0x44),
    ov!(0x0029, 0x24),
    ov!(0x002A, 0x08),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0xF8),
];
static D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_CLEAR_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0042, 0xFF),
    ov!(0x0043, 0xFD),
    ov!(0x0044, 0xFF),
    ov!(0x0045, 0xFF),
    ov!(0x0046, 0x89),
    ov!(0x0047, 0x44),
];
static D3D8_LTCG_D3DDEVICE_CLEAR_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000B, 0xA3),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x06),
    ov!(0x0012, 0x3B),
    ov!(0x0013, 0x46),
    ov!(0x0014, 0x04),
    ov!(0x0015, 0x57),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x3D),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3DRS_FillMode",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x86),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x75),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x06),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DRESOURCE_RELEASE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x57),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x7C),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x07),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0xC8),
    ov!(0x0009, 0x81),
    ov!(0x000A, 0xE1),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0xFF),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
];
static D3D8_LTCG_D3DRESOURCE_RELEASE_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1045_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001B, 0xC7),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x08),
    ov!(0x001E, 0x03),
    ov!(0x001F, 0x04),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1045_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x002D,
        target: "D3DRS_CullMode",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0024, 0xC7),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x28),
    ov!(0x0027, 0x03),
    ov!(0x0028, 0x04),
    ov!(0x0029, 0x00),
    ov!(0x002A, 0x89),
    ov!(0x002B, 0x48),
    ov!(0x002C, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_12__LTCG_EBX3_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x3D),
    ov!(0x000F, 0x89),
    ov!(0x0011, 0xFC),
    ov!(0x0012, 0xE8),
    ov!(0x0025, 0x8B),
    ov!(0x0027, 0x08),
    ov!(0x0028, 0x8B),
    ov!(0x002A, 0x10),
    ov!(0x002B, 0x89),
];
static D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_12__LTCG_EBX3_2024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000A,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0013,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_0__LTCG_ECX1_EAX2_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x003F, 0xC7),
    ov!(0x0040, 0x00),
    ov!(0x0041, 0x9C),
    ov!(0x0042, 0x1E),
    ov!(0x0043, 0x04),
    ov!(0x0044, 0x00),
    ov!(0x0045, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_0__LTCG_ECX1_EAX2_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX2_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0xF0),
    ov!(0x0027, 0x81),
    ov!(0x0028, 0xC1),
    ov!(0x0029, 0x00),
    ov!(0x002A, 0x00),
    ov!(0x002B, 0xF8),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX2_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_0__LTCG_EAX1_EBX2_2024_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x001C, 0xC1),
    ov!(0x001D, 0xE1),
    ov!(0x001E, 0x06),
    ov!(0x001F, 0x81),
    ov!(0x0020, 0xC1),
    ov!(0x0021, 0x24),
    ov!(0x0022, 0x1B),
    ov!(0x0023, 0x04),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_0__LTCG_EAX1_EBX2_2024_XREFS:
    &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_8__LTCG_EAX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x85),
    ov!(0x0017, 0xC0),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0xDF),
    ov!(0x001A, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_8__LTCG_EAX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_0__LTCG_EBX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xF6),
    ov!(0x0001, 0xC3),
    ov!(0x0002, 0x01),
    ov!(0x0003, 0x55),
    ov!(0x0004, 0x56),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x0015, 0x74),
    ov!(0x0016, 0x05),
    ov!(0x0017, 0x8D),
    ov!(0x0018, 0x7B),
    ov!(0x0019, 0xFF),
    ov!(0x001A, 0xEB),
    ov!(0x001B, 0x0E),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_0__LTCG_EBX1_2024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "D3DDevice__m_VertexShader_OFFSET",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_8__LTCG_EDX3_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0013, 0x83),
    ov!(0x0014, 0xC1),
    ov!(0x0015, 0x60),
    ov!(0x0016, 0xC1),
    ov!(0x0017, 0xE2),
    ov!(0x0018, 0x02),
    ov!(0x0019, 0xA8),
    ov!(0x001A, 0x10),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_8__LTCG_EDX3_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_EBX6_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0006, 0x85),
    ov!(0x0007, 0xC9),
    ov!(0x0008, 0x75),
    ov!(0x0009, 0x0A),
    ov!(0x000A, 0xC7),
    ov!(0x000B, 0x05),
    ov!(0x0028, 0x56),
    ov!(0x0029, 0x8B),
];
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_EBX6_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EDX1_2039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x1D),
    ov!(0x0038, 0x81),
    ov!(0x0039, 0x45),
    ov!(0x003A, 0x00),
    ov!(0x003B, 0x00),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0x08),
    ov!(0x003E, 0x00),
    ov!(0x003F, 0x8D),
];
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EDX1_2039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0047,
        target: "D3D_g_Stream_i_pVertexBuffer",
    },
    OovpaXref {
        offset: 0x0077,
        target: "D3D_g_Stream",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_0__LTCG_EDI1_EAX2_2039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xD9),
    ov!(0x0003, 0xC1),
    ov!(0x0004, 0xE1),
    ov!(0x0005, 0x07),
    ov!(0x0013, 0x25),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0xFF),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_0__LTCG_EDI1_EAX2_2039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "D3DTSS_TEXCOORDINDEX",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x005C, 0xC7),
    ov!(0x005D, 0x40),
    ov!(0x005E, 0x04),
    ov!(0x005F, 0x00),
    ov!(0x0060, 0x00),
    ov!(0x0061, 0x21),
    ov!(0x0062, 0x00),
    ov!(0x0063, 0x83),
    ov!(0x0064, 0xC0),
    ov!(0x0065, 0x08),
    ov!(0x006A, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x15),
    ov!(0x0016, 0xFF),
    ov!(0x0017, 0x15),
    ov!(0x0064, 0x8B),
    ov!(0x0065, 0xB3),
    ov!(0x006A, 0xE8),
    ov!(0x006F, 0x89),
    ov!(0x0075, 0x39),
];
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0002,
        target: "KT_FUNC_AvGetSavedDataAddress",
    },
    OovpaXref {
        offset: 0x0018,
        target: "KT_FUNC_AvSendTVEncoderOption",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0019, 0x72),
    ov!(0x001A, 0x04),
    ov!(0x001B, 0x89),
    ov!(0x001C, 0x44),
    ov!(0x001D, 0x24),
    ov!(0x001F, 0x8B),
    ov!(0x0020, 0x47),
    ov!(0x0021, 0x08),
    ov!(0x0022, 0xE8),
    ov!(0x0029, 0x8B),
    ov!(0x002B, 0x24),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x85),
    ov!(0x0006, 0xC0),
    ov!(0x0007, 0x75),
    ov!(0x0008, 0x2B),
    ov!(0x0009, 0xF6),
    ov!(0x000A, 0x42),
    ov!(0x000B, 0x0C),
    ov!(0x000C, 0x04),
    ov!(0x000D, 0x56),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x08),
    ov!(0x0033, 0xC3),
];
static D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_CDEVICE_SETSTATEUP_4_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0008, 0xE8),
    ov!(0x0019, 0x25),
    ov!(0x001A, 0xFF),
    ov!(0x001B, 0xFE),
    ov!(0x001C, 0xFF),
    ov!(0x001D, 0xFF),
    ov!(0x001F, 0x0D),
    ov!(0x0020, 0x80),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0x00),
];
static D3D8_LTCG_CDEVICE_SETSTATEUP_4_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_CDEVICE_SETSTATEVB_8_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0006, 0xE8),
    ov!(0x0012, 0xA9),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x40),
    ov!(0x002D, 0x25),
    ov!(0x002E, 0x7F),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0xBF),
];
static D3D8_LTCG_CDEVICE_SETSTATEVB_8_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_COPYRECTS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x81),
    ov!(0x0001, 0xEC),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xAC),
    ov!(0x000A, 0x24),
    ov!(0x0013, 0x0F),
    ov!(0x0014, 0xB6),
    ov!(0x0015, 0x75),
    ov!(0x0016, 0x0D),
    ov!(0x0017, 0x8A),
    ov!(0x0018, 0x9E),
];
static D3D8_LTCG_D3DDEVICE_COPYRECTS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_DELETEVERTEXSHADER_0__LTCG_EAX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x48),
    ov!(0x0002, 0xFF),
    ov!(0x0003, 0x48),
    ov!(0x0004, 0x49),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0x08),
    ov!(0x0007, 0x75),
    ov!(0x0008, 0x06),
    ov!(0x000A, 0xE8),
    ov!(0x000F, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_DELETEVERTEXSHADER_0__LTCG_EAX1_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0029, 0x75),
    ov!(0x002B, 0xE8),
    ov!(0x0030, 0x57),
];
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002C,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x0003, 0x8B),
    ov!(0x0040, 0xC7),
    ov!(0x0041, 0x00),
    ov!(0x0042, 0x94),
    ov!(0x0043, 0x1E),
    ov!(0x0044, 0x08),
    ov!(0x0045, 0x00),
    ov!(0x0046, 0xC7),
    ov!(0x0047, 0x40),
    ov!(0x0048, 0x04),
    ov!(0x0049, 0x06),
    ov!(0x004A, 0x00),
    ov!(0x004D, 0x83),
    ov!(0x004E, 0xC0),
    ov!(0x004F, 0x0C),
    ov!(0x007B, 0xC2),
    ov!(0x007C, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFE),
    ov!(0x0003, 0x8B),
    ov!(0x0004, 0xBB),
    ov!(0x0009, 0x7D),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x0C),
    ov!(0x000D, 0xB5),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0xD7),
    ov!(0x0014, 0xE8),
    ov!(0x0019, 0x89),
    ov!(0x001A, 0x3C),
    ov!(0x001B, 0xB5),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0015,
        target: "D3DDevice_SetRenderState_Simple",
    },
    OovpaXref {
        offset: 0x001C,
        target: "D3D_g_RenderState",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_0__LTCG_EAX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA8),
    ov!(0x0001, 0x10),
    ov!(0x0003, 0x8B),
    ov!(0x0004, 0x1D),
    ov!(0x000E, 0x81),
    ov!(0x000F, 0xC9),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x02),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x002E, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_0__LTCG_EAX1_3911_XREFS: &[OovpaXref] =
    &[OovpaXref {
        offset: 0x0005,
        target: "D3D_g_pDevice",
    }];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EDX2_EAX3_3911_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0002, 0xD8),
    ov!(0x0003, 0xA1),
    ov!(0x0009, 0x83),
    ov!(0x000A, 0xFA),
    ov!(0x0023, 0xC1),
    ov!(0x0024, 0xE1),
    ov!(0x0025, 0x05),
    ov!(0x002F, 0x89),
    ov!(0x0041, 0xC2),
    ov!(0x0042, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EDX2_EAX3_3911_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_0__LTCG_EAX1_EBX2_3911_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xF8),
    ov!(0x000D, 0x8B),
    ov!(0x000F, 0x3B),
    ov!(0x0010, 0xC1),
    ov!(0x0011, 0x72),
    ov!(0x0012, 0x07),
    ov!(0x001A, 0x8D),
    ov!(0x001D, 0xE0),
    ov!(0x001E, 0x0A),
    ov!(0x001F, 0x04),
    ov!(0x0020, 0x00),
    ov!(0x0036, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_0__LTCG_EAX1_EBX2_3911_XREFS:
    &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EAX1_ECX2_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xEC),
    ov!(0x0008, 0x18),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0xF1),
    ov!(0x002C, 0xC7),
    ov!(0x002D, 0x44),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x20),
    ov!(0x0030, 0x00),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0x00),
    ov!(0x0034, 0xC7),
    ov!(0x0035, 0x44),
    ov!(0x0036, 0x24),
    ov!(0x0037, 0x1C),
    ov!(0x0080, 0x83),
    ov!(0x0081, 0xC4),
    ov!(0x0082, 0x18),
    ov!(0x0083, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EAX1_ECX2_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0002,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x1D),
    ov!(0x0009, 0x8D),
    ov!(0x000A, 0x78),
    ov!(0x000B, 0x22),
    ov!(0x000C, 0xC1),
    ov!(0x000D, 0xE7),
    ov!(0x000E, 0x06),
    ov!(0x00EA, 0xE8),
    ov!(0x00F9, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x00EB,
        target: "D3D_UpdateProjectionViewportTransform",
    },
];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_EAX1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0xF0),
    ov!(0x0028, 0xF7),
    ov!(0x0029, 0xC1),
    ov!(0x002A, 0x00),
    ov!(0x002B, 0x00),
    ov!(0x002C, 0x78),
    ov!(0x002D, 0x00),
    ov!(0x002E, 0x75),
    ov!(0x002F, 0x0C),
    ov!(0x0030, 0x85),
];
static D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_EAX1_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/3911.inl
static D3D8_LTCG_D3D_DESTROYRESOURCE_0__LTCG_EDI1_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x07),
    ov!(0x0005, 0x81),
    ov!(0x0006, 0xE6),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x07),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x81),
    ov!(0x000C, 0xFE),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x05),
    ov!(0x0010, 0x00),
    ov!(0x0017, 0x8B),
    ov!(0x0024, 0x81),
    ov!(0x0025, 0xFE),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x05),
    ov!(0x0029, 0x00),
];
static D3D8_LTCG_D3D_DESTROYRESOURCE_0__LTCG_EDI1_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001A,
    target: "D3D_BlockOnResource",
}];

pub const PATTERNS: &[OovpaPattern] = &[
    OovpaPattern {
        name: "D3DDevice_BlockUntilVerticalBlank",
        detect_size: 0x0024,
        entries: D3D8_LTCG_D3DDEVICE_BLOCKUNTILVERTICALBLANK_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_CreateVertexShader",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_CREATEVERTEXSHADER_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVertices",
        detect_size: 0x001D,
        entries: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVerticesUP",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_Present",
        detect_size: 0x001E,
        entries: D3D8_LTCG_D3DDEVICE_PRESENT_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_BackFillMode",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        detect_size: 0x0022,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FillMode",
        detect_size: 0x0045,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FogColor",
        detect_size: 0x0022,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FrontFace",
        detect_size: 0x0032,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LineWidth",
        detect_size: 0x005F,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LogicOp",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        detect_size: 0x0022,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        detect_size: 0x0021,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilEnable",
        detect_size: 0x0082,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilFail",
        detect_size: 0x006B,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        detect_size: 0x0019,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZBias",
        detect_size: 0x001F,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZEnable",
        detect_size: 0x003A,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetScissors",
        detect_size: 0x0027,
        entries: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetViewport",
        detect_size: 0x002D,
        entries: D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_Clear",
        detect_size: 0x0048,
        entries: D3D8_LTCG_D3DDEVICE_CLEAR_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_BackFillMode",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_TextureFactor",
        detect_size: 0x0013,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DResource_Release",
        detect_size: 0x000F,
        entries: D3D8_LTCG_D3DRESOURCE_RELEASE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_CullMode",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1045_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1045,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_VertexBlend",
        detect_size: 0x002D,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVerticesUP",
        detect_size: 0x002C,
        entries: D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_12__LTCG_EBX3_2024_ENTRIES,
        argc: 3,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShader",
        detect_size: 0x0046,
        entries: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_0__LTCG_ECX1_EAX2_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTexture",
        detect_size: 0x002E,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX2_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BorderColor",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_0__LTCG_EAX1_EBX2_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BumpEnv",
        detect_size: 0x001B,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_8__LTCG_EAX1_2024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShader",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_0__LTCG_EBX1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShaderConstant",
        detect_size: 0x001B,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_8__LTCG_EDX3_2024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "Direct3D_CreateDevice",
        detect_size: 0x002A,
        entries: D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_EBX6_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetStreamSource",
        detect_size: 0x007B,
        entries: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EDX1_2039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2039,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x0017,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_0__LTCG_EDI1_EAX2_2039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2039,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShader",
        detect_size: 0x006B,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2060,
    },
    OovpaPattern {
        name: "CDevice_FreeFrameBuffers",
        detect_size: 0x0076,
        entries: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_3911_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x002C,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_3911_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_KickOff",
        detect_size: 0x0034,
        entries: D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_SetStateUP",
        detect_size: 0x0023,
        entries: D3D8_LTCG_CDEVICE_SETSTATEUP_4_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_SetStateVB",
        detect_size: 0x0032,
        entries: D3D8_LTCG_CDEVICE_SETSTATEVB_8_3911_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CopyRects",
        detect_size: 0x0019,
        entries: D3D8_LTCG_D3DDEVICE_COPYRECTS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DeleteVertexShader",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_DELETEVERTEXSHADER_0__LTCG_EAX1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x007D,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_3911_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderStateInline__GenericFragment",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetShaderConstantMode",
        detect_size: 0x002F,
        entries: D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_0__LTCG_EAX1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline",
        detect_size: 0x0043,
        entries:
            D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EDX2_EAX3_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        detect_size: 0x0037,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_0__LTCG_EAX1_EBX2_3911_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTile",
        detect_size: 0x0084,
        entries: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EAX1_ECX2_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTransform",
        detect_size: 0x00FA,
        entries: D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_BlockOnResource",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_EAX1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_DestroyResource",
        detect_size: 0x002A,
        entries: D3D8_LTCG_D3D_DESTROYRESOURCE_0__LTCG_EDI1_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
];

pub const METADATA: &[OovpaPatternMeta] = &[
    OovpaPatternMeta {
        name: "D3DDevice_BlockUntilVerticalBlank",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BLOCKUNTILVERTICALBLANK_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateVertexShader",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATEVERTEXSHADER_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVertices",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVerticesUP",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Present",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_PRESENT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_BackFillMode",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FillMode",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FogColor",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FrontFace",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LineWidth",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LogicOp",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilEnable",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilFail",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZBias",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZEnable",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetScissors",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetViewport",
        min_version: 1024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Clear",
        min_version: 1036,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CLEAR_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_BackFillMode",
        min_version: 1036,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_TextureFactor",
        min_version: 1036,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_Release",
        min_version: 1036,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DRESOURCE_RELEASE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_CullMode",
        min_version: 1045,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1045_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_VertexBlend",
        min_version: 1048,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVerticesUP",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_12__LTCG_EBX3_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShader",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_0__LTCG_ECX1_EAX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTexture",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BorderColor",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_0__LTCG_EAX1_EBX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BumpEnv",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_8__LTCG_EAX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShader",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_0__LTCG_EBX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShaderConstant",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_8__LTCG_EDX3_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "Direct3D_CreateDevice",
        min_version: 2024,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_EBX6_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetStreamSource",
        min_version: 2039,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EDX1_2039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 2039,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_0__LTCG_EDI1_EAX2_2039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 2060,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2060_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_FreeFrameBuffers",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_KickOff",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateUP",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_CDEVICE_SETSTATEUP_4_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateVB",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_CDEVICE_SETSTATEVB_8_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CopyRects",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_COPYRECTS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeleteVertexShader",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DELETEVERTEXSHADER_0__LTCG_EAX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderStateInline__GenericFragment",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetShaderConstantMode",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_0__LTCG_EAX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EDX2_EAX3_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_0__LTCG_EAX1_EBX2_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTile",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EAX1_ECX2_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTransform",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_BlockOnResource",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_EAX1_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_DestroyResource",
        min_version: 3911,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3D_DESTROYRESOURCE_0__LTCG_EDI1_3911_XREFS,
    },
];
