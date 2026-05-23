// Auto-generated from Cxbx-R XbSymbolDatabase.
// Regenerate with: python3 tools/import_xbsymdb_database.py --xdk 4928
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

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0019, 0xC7),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0xC8),
    ov!(0x001C, 0x17),
    ov!(0x001D, 0x08),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0xB9),
    ov!(0x0020, 0x01),
    ov!(0x0021, 0x00),
    ov!(0x002A, 0x83),
    ov!(0x002B, 0xC0),
    ov!(0x002C, 0x0C),
    ov!(0x0030, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x35),
    ov!(0x000D, 0x89),
    ov!(0x000F, 0xEC),
    ov!(0x0010, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0009,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0011,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_ISFENCEPENDING_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0010, 0xD1),
    ov!(0x0011, 0x2B),
    ov!(0x0012, 0x44),
    ov!(0x0013, 0x24),
    ov!(0x0014, 0x04),
    ov!(0x0015, 0x3B),
];
static D3D8_LTCG_D3DDEVICE_ISFENCEPENDING_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETINDICES_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x000F, 0x74),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x81),
    ov!(0x0012, 0x03),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x08),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETINDICES_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMODE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0015, 0x3B),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMODE_1024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000B,
    target: "D3DRS_MultiSampleMode",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x001D, 0x8D),
    ov!(0x0020, 0x80),
    ov!(0x0021, 0x18),
    ov!(0x0022, 0x08),
    ov!(0x0023, 0x00),
    ov!(0x0034, 0x83),
    ov!(0x0035, 0xC0),
    ov!(0x0036, 0x0C),
    ov!(0x003A, 0xC2),
    ov!(0x003B, 0x0C),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x001D, 0x8D),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x19),
    ov!(0x0022, 0x04),
    ov!(0x0023, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0x80),
    ov!(0x0021, 0x19),
    ov!(0x0022, 0x08),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x0F),
    ov!(0x0025, 0xBF),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001D, 0x8D),
    ov!(0x0020, 0x40),
    ov!(0x0021, 0x19),
    ov!(0x0022, 0x04),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x33),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x5C),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0xF6),
    ov!(0x0006, 0xC3),
    ov!(0x0007, 0x01),
    ov!(0x0008, 0x55),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x35),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0012,
    target: "D3DDevice__m_VertexShader_OFFSET",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERINPUT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x83),
    ov!(0x0005, 0xEC),
    ov!(0x0006, 0x40),
    ov!(0x0007, 0x85),
    ov!(0x0008, 0xC0),
    ov!(0x0009, 0x53),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x1D),
    ov!(0x001A, 0x25),
    ov!(0x001B, 0xFF),
    ov!(0x001C, 0xFF),
    ov!(0x001D, 0xFF),
    ov!(0x001E, 0xBF),
    ov!(0x001F, 0x83),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERINPUT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DSURFACE_GETDESC_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0006, 0x57),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x7C),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x000B, 0x33),
    ov!(0x000C, 0xDB),
    ov!(0x000D, 0xE8),
];
static D3D8_LTCG_D3DSURFACE_GETDESC_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3D_SETFENCE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0024, 0xC9),
    ov!(0x0025, 0x89),
    ov!(0x0026, 0x48),
    ov!(0x0027, 0x0C),
    ov!(0x0028, 0x89),
    ov!(0x0029, 0x48),
    ov!(0x002A, 0x14),
    ov!(0x002B, 0xC7),
];
static D3D8_LTCG_D3D_SETFENCE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0019, 0x24),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x8D),
    ov!(0x001D, 0x24),
    ov!(0x001E, 0x14),
    ov!(0x001F, 0x50),
    ov!(0x0020, 0x8B),
    ov!(0x0022, 0x24),
    ov!(0x0023, 0x24),
    ov!(0x0044, 0xE8),
    ov!(0x004A, 0x83),
    ov!(0x004B, 0xC4),
    ov!(0x004C, 0x08),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x10),
];
static D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_BEGIN_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0033, 0x81),
    ov!(0x0034, 0x4E),
    ov!(0x0035, 0x08),
    ov!(0x0036, 0x00),
    ov!(0x0037, 0x08),
    ov!(0x0038, 0x00),
    ov!(0x0039, 0x00),
    ov!(0x003A, 0x5E),
    ov!(0x003B, 0xC2),
    ov!(0x003C, 0x04),
];
static D3D8_LTCG_D3DDEVICE_BEGIN_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_CLEAR_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0040, 0xFF),
    ov!(0x0041, 0xFD),
    ov!(0x0042, 0xFF),
    ov!(0x0043, 0xFF),
    ov!(0x0044, 0x89),
    ov!(0x0045, 0x44),
];
static D3D8_LTCG_D3DDEVICE_CLEAR_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0010, 0x20),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x50),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x44),
    ov!(0x0016, 0x24),
    ov!(0x0017, 0x14),
    ov!(0x0018, 0x51),
    ov!(0x0029, 0x5F),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x1C),
];
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x1D),
    ov!(0x000C, 0xE8),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x7C),
    ov!(0x0013, 0x24),
    ov!(0x0014, 0x18),
];
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_ENDPUSHBUFFER_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x8B),
    ov!(0x000C, 0xE8),
    ov!(0x004C, 0x81),
    ov!(0x004D, 0xE2),
    ov!(0x004E, 0x7B),
    ov!(0x004F, 0xFF),
    ov!(0x0050, 0xFF),
    ov!(0x0051, 0xFF),
    ov!(0x0062, 0xA9),
    ov!(0x0063, 0xFF),
    ov!(0x0064, 0xFF),
    ov!(0x0065, 0x78),
    ov!(0x0066, 0x00),
];
static D3D8_LTCG_D3DDEVICE_ENDPUSHBUFFER_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0004,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0040, 0xB8),
    ov!(0x0041, 0x66),
    ov!(0x0042, 0x08),
    ov!(0x0043, 0x76),
    ov!(0x0044, 0x88),
    ov!(0x0046, 0xC2),
    ov!(0x0047, 0x04),
    ov!(0x0048, 0x00),
];
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x002F, 0xB8),
    ov!(0x0030, 0x05),
    ov!(0x0031, 0x40),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0x80),
    ov!(0x0034, 0x5B),
    ov!(0x0035, 0x59),
    ov!(0x0036, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0090, 0x81),
    ov!(0x0091, 0xE1),
    ov!(0x0092, 0xFF),
    ov!(0x0093, 0xFF),
    ov!(0x0094, 0xFF),
    ov!(0x0095, 0x0F),
    ov!(0x0096, 0x41),
    ov!(0x0097, 0x89),
];
static D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0014, 0x44),
    ov!(0x0015, 0x24),
    ov!(0x0016, 0x20),
    ov!(0x0017, 0x85),
    ov!(0x0018, 0xC0),
    ov!(0x0019, 0x89),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x24),
    ov!(0x001C, 0x0C),
    ov!(0x001D, 0x89),
    ov!(0x001E, 0x4C),
    ov!(0x001F, 0x24),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x81),
    ov!(0x0007, 0xCA),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x02),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x56),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x35),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xA8),
    ov!(0x0005, 0x10),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x1D),
    ov!(0x001F, 0xFF),
];
static D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0025, 0xB9),
    ov!(0x0026, 0x18),
    ov!(0x0027, 0x15),
    ov!(0x0028, 0x00),
    ov!(0x0029, 0x00),
    ov!(0x002A, 0xEB),
    ov!(0x002B, 0x09),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0014, 0x83),
    ov!(0x0015, 0xC1),
    ov!(0x0016, 0x60),
    ov!(0x0017, 0xC1),
    ov!(0x0018, 0xE2),
    ov!(0x0019, 0x02),
    ov!(0x001A, 0xA8),
    ov!(0x001B, 0x10),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DRESOURCE_GETTYPE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0010, 0x77),
    ov!(0x0011, 0x28),
    ov!(0x0012, 0x74),
    ov!(0x0013, 0x1E),
    ov!(0x0014, 0x85),
    ov!(0x0015, 0xC0),
];
static D3D8_LTCG_D3DRESOURCE_GETTYPE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0006, 0xC0),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x08),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0xA1),
];
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_CMINIPORT_INITHARDWARE_4_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x10),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x57),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xF8),
    ov!(0x000A, 0x57),
    ov!(0x000B, 0x68),
];
static D3D8_LTCG_CMINIPORT_INITHARDWARE_4_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0019, 0x24),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x8D),
    ov!(0x001D, 0x24),
    ov!(0x001E, 0x14),
    ov!(0x001F, 0x50),
    ov!(0x0020, 0x8B),
    ov!(0x0022, 0x24),
    ov!(0x0023, 0x24),
    ov!(0x003F, 0xE8),
    ov!(0x0045, 0x83),
    ov!(0x0046, 0xC4),
    ov!(0x0047, 0x08),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x0C),
];
static D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_BEGINPUSH_4_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x000A, 0xE8),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x06),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x76),
    ov!(0x0013, 0x04),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x4C),
    ov!(0x0016, 0x24),
    ov!(0x0017, 0x08),
    ov!(0x0018, 0x81),
    ov!(0x0019, 0xC6),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x02),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x00),
    ov!(0x001E, 0x8D),
];
static D3D8_LTCG_D3DDEVICE_BEGINPUSH_4_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000B,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x003D, 0xF7),
    ov!(0x003E, 0x44),
    ov!(0x003F, 0x24),
    ov!(0x0040, 0x1C),
    ov!(0x0041, 0x00),
    ov!(0x0042, 0x00),
    ov!(0x0043, 0x01),
    ov!(0x0044, 0x00),
    ov!(0x0045, 0x74),
];
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_DELETESTATEBLOCK_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x55),
    ov!(0x0002, 0x56),
    ov!(0x0003, 0x57),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x7C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0x33),
    ov!(0x0009, 0xED),
    ov!(0x000A, 0x33),
    ov!(0x000B, 0xDB),
];
static D3D8_LTCG_D3DDEVICE_DELETESTATEBLOCK_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0010, 0xB8),
    ov!(0x0011, 0x0E),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x07),
    ov!(0x0014, 0x80),
    ov!(0x0015, 0x5E),
    ov!(0x0016, 0xC2),
    ov!(0x0017, 0x04),
];
static D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETDISPLAYMODE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x90),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000D, 0x10),
    ov!(0x000E, 0x85),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x4A),
];
static D3D8_LTCG_D3DDEVICE_GETDISPLAYMODE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0xB0),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x24),
    ov!(0x002D, 0xE8),
    ov!(0x0037, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0008,
        target: "OFFSET_D3DDevice__m_RenderTarget",
    },
    OovpaXref {
        offset: 0x002E,
        target: "D3DResource_AddRef",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETTRANSFORM_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0xC1),
    ov!(0x000B, 0xE1),
    ov!(0x000C, 0x06),
    ov!(0x000D, 0x57),
    ov!(0x000E, 0x8B),
    ov!(0x000F, 0x7C),
    ov!(0x0010, 0x24),
    ov!(0x0011, 0x10),
    ov!(0x0012, 0x8D),
    ov!(0x0013, 0xB4),
];
static D3D8_LTCG_D3DDEVICE_GETTRANSFORM_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_ISBUSY_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0xBC),
    ov!(0x0008, 0x23),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x90),
];
static D3D8_LTCG_D3DDEVICE_ISBUSY_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_LIGHTENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0023, 0xF6),
    ov!(0x0024, 0x44),
    ov!(0x0025, 0x01),
    ov!(0x0026, 0x68),
    ov!(0x0027, 0x01),
    ov!(0x0028, 0x75),
];
static D3D8_LTCG_D3DDEVICE_LIGHTENABLE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x006E, 0xC7),
    ov!(0x006F, 0x84),
    ov!(0x0070, 0x98),
    ov!(0x0071, 0xDC),
    ov!(0x0072, 0x07),
    ov!(0x0073, 0x00),
    ov!(0x0074, 0x00),
    ov!(0x0075, 0x01),
    ov!(0x0076, 0x00),
    ov!(0x0077, 0x00),
    ov!(0x0078, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SAMPLEALPHA_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x000B, 0xA3),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SAMPLEALPHA_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_SampleAlpha",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0028, 0x81),
    ov!(0x0029, 0xC1),
    ov!(0x002A, 0x00),
    ov!(0x002B, 0x00),
    ov!(0x002C, 0xF8),
    ov!(0x002D, 0xFF),
    ov!(0x002E, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0x08),
    ov!(0x0020, 0x8B),
    ov!(0x0021, 0xD1),
    ov!(0x0022, 0xC1),
    ov!(0x0023, 0xE2),
    ov!(0x0024, 0x06),
    ov!(0x0025, 0x81),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x24),
    ov!(0x0028, 0x1B),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x56),
    ov!(0x001A, 0x8D),
    ov!(0x001B, 0x5E),
    ov!(0x001C, 0x01),
    ov!(0x001D, 0xF6),
    ov!(0x001E, 0xC3),
    ov!(0x001F, 0x03),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SWAP_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0028, 0x75),
    ov!(0x0029, 0x05),
    ov!(0x002A, 0xBB),
    ov!(0x002B, 0x05),
    ov!(0x002C, 0x00),
    ov!(0x002D, 0x00),
    ov!(0x002E, 0x00),
    ov!(0x002F, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SWAP_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DPALETTE_LOCK2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xF6),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0004, 0xA0),
    ov!(0x0005, 0x56),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x74),
];
static D3D8_LTCG_D3DPALETTE_LOCK2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DTEXTURE_LOCKRECT_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x14),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x54),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x04),
    ov!(0x000C, 0x56),
    ov!(0x000D, 0x8B),
];
static D3D8_LTCG_D3DTEXTURE_LOCKRECT_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DVERTEXBUFFER_LOCK2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8A),
    ov!(0x0004, 0x0C),
    ov!(0x0005, 0xF6),
    ov!(0x0006, 0xC3),
    ov!(0x0007, 0x10),
    ov!(0x0008, 0x56),
    ov!(0x0009, 0x75),
    ov!(0x0011, 0x8B),
];
static D3D8_LTCG_D3DVERTEXBUFFER_LOCK2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3D_KICKOFFANDWAITFORIDLE2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x4C),
    ov!(0x0012, 0x24),
    ov!(0x0013, 0x08),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x44),
    ov!(0x0016, 0x24),
    ov!(0x0017, 0x04),
    ov!(0x0018, 0xE8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x08),
];
static D3D8_LTCG_D3D_KICKOFFANDWAITFORIDLE2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x85),
    ov!(0x000C, 0xC9),
    ov!(0x000D, 0x75),
    ov!(0x000E, 0x03),
    ov!(0x000F, 0x33),
    ov!(0x0010, 0xC0),
    ov!(0x0011, 0xC3),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x01),
    ov!(0x0014, 0xA9),
];
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DTEXTURE_GETSURFACELEVEL2_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0005, 0x7C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0x8D),
    ov!(0x0009, 0x44),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x18),
    ov!(0x000C, 0x50),
    ov!(0x000D, 0x8D),
    ov!(0x0045, 0xC2),
    ov!(0x0046, 0x08),
];
static D3D8_LTCG_D3DTEXTURE_GETSURFACELEVEL2_1024_XREFS: &[OovpaXref] = &[];

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

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x08),
    ov!(0x0008, 0x8B),
    ov!(0x0013, 0x89),
    ov!(0x0014, 0x75),
    ov!(0x0015, 0xF8),
    ov!(0x0016, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000A,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0017,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x35),
    ov!(0x000D, 0x89),
    ov!(0x000F, 0xF8),
    ov!(0x0010, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0009,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0011,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x001A, 0x4C),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0x20),
    ov!(0x0020, 0x03),
    ov!(0x0021, 0x08),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x89),
    ov!(0x0024, 0x48),
    ov!(0x0025, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0019, 0x8B),
    ov!(0x001F, 0x8B),
    ov!(0x002F, 0xC7),
    ov!(0x0030, 0x00),
    ov!(0x0031, 0x8C),
    ov!(0x0032, 0x03),
    ov!(0x0033, 0x08),
    ov!(0x0034, 0x00),
    ov!(0x003B, 0x83),
    ov!(0x003C, 0xC0),
    ov!(0x003D, 0x0C),
    ov!(0x0040, 0x89),
    ov!(0x0047, 0xC2),
    ov!(0x0048, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x001B,
        target: "D3DRS_TwoSidedLighting",
    },
    OovpaXref {
        offset: 0x0021,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x0042,
        target: "D3DRS_FillMode",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0x54),
    ov!(0x0021, 0x24),
    ov!(0x0022, 0x0E),
    ov!(0x0023, 0x8B),
    ov!(0x0024, 0xF9),
    ov!(0x0025, 0x81),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xC0),
    ov!(0x0028, 0x08),
    ov!(0x002B, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x002D,
        target: "D3DRS_FrontFace",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0x75),
    ov!(0x0021, 0xC7),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0xBC),
    ov!(0x0024, 0x17),
    ov!(0x0025, 0x04),
    ov!(0x0026, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0031,
        target: "D3DRS_LogicOp",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0xA4),
    ov!(0x0020, 0x03),
    ov!(0x0021, 0x04),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x89),
    ov!(0x0024, 0x48),
    ov!(0x0025, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x001F, 0x6C),
    ov!(0x0020, 0x1E),
    ov!(0x0021, 0x04),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x8D),
    ov!(0x0024, 0x91),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0071, 0xC7),
    ov!(0x0073, 0x08),
    ov!(0x0074, 0x2C),
    ov!(0x0075, 0x03),
    ov!(0x0076, 0x04),
    ov!(0x0077, 0x00),
    ov!(0x0080, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0082,
        target: "D3DRS_StencilEnable",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x005A, 0xC7),
    ov!(0x005C, 0x08),
    ov!(0x005D, 0x70),
    ov!(0x005E, 0x03),
    ov!(0x005F, 0x04),
    ov!(0x0060, 0x00),
    ov!(0x0069, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x006B,
        target: "D3DRS_StencilFail",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1036_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0x8B), ov!(0x0006, 0x8B), ov!(0x0016, 0x8B)];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0018,
        target: "D3DRS_FillMode",
    },
    OovpaXref {
        offset: 0x001D,
        target: "D3DRS_TwoSidedLighting",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0012, 0x81),
    ov!(0x0013, 0xCA),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x02),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x3B),
    ov!(0x0019, 0xC1),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0037, 0xC7),
    ov!(0x0038, 0x00),
    ov!(0x0039, 0x0C),
    ov!(0x003A, 0x03),
    ov!(0x003B, 0x04),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0x89),
    ov!(0x003E, 0x50),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x3C),
    ov!(0x0047, 0x0F),
    ov!(0x0048, 0x85),
    ov!(0x004A, 0x01),
    ov!(0x004B, 0x00),
    ov!(0x004C, 0x00),
    ov!(0x004D, 0x8B),
    ov!(0x004E, 0x0D),
    ov!(0x0053, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004F,
    target: "D3DRS_MultiSampleMode",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0023, 0xEB),
    ov!(0x0024, 0x06),
    ov!(0x0025, 0x89),
    ov!(0x0026, 0x44),
    ov!(0x0027, 0x24),
    ov!(0x0028, 0x08),
    ov!(0x0029, 0x8B),
    ov!(0x002A, 0xF8),
];
static D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SWAP_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x000E, 0x75),
    ov!(0x000F, 0x05),
    ov!(0x0010, 0xBB),
    ov!(0x0011, 0x05),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SWAP_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4134.inl
static D3D8_LTCG_D3DDEVICE_MAKESPACE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0009, 0x51),
    ov!(0x000A, 0xE8),
    ov!(0x000F, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_MAKESPACE_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000B,
    target: "D3D_MakeRequestedSpace",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0048, 0x00),
    ov!(0x0049, 0x81),
    ov!(0x004A, 0x07),
    ov!(0x004B, 0x00),
    ov!(0x004C, 0x00),
    ov!(0x004D, 0x08),
    ov!(0x004E, 0x00),
    ov!(0x004F, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_END_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x003D, 0x81),
    ov!(0x003E, 0x66),
    ov!(0x003F, 0x08),
    ov!(0x0040, 0xFF),
    ov!(0x0041, 0xE7),
    ov!(0x0042, 0xFF),
    ov!(0x0043, 0xFF),
    ov!(0x0044, 0x5E),
    ov!(0x0045, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_END_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x002E, 0x8B),
    ov!(0x002F, 0xFB),
    ov!(0x0030, 0xF3),
    ov!(0x0031, 0xA5),
    ov!(0x0032, 0xF6),
    ov!(0x0033, 0x44),
    ov!(0x0034, 0x24),
    ov!(0x0035, 0x10),
    ov!(0x0036, 0x02),
    ov!(0x0037, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0020, 0x8D),
    ov!(0x0023, 0x80),
    ov!(0x0024, 0x18),
    ov!(0x0025, 0x08),
    ov!(0x0026, 0x00),
    ov!(0x0037, 0x83),
    ov!(0x0038, 0xC0),
    ov!(0x0039, 0x0C),
    ov!(0x003D, 0xC2),
    ov!(0x003E, 0x0C),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x8B),
    ov!(0x0003, 0x5C),
    ov!(0x0004, 0x24),
    ov!(0x0005, 0x0C),
    ov!(0x0006, 0xF6),
    ov!(0x0007, 0xC3),
    ov!(0x0008, 0x01),
    ov!(0x0009, 0x55),
    ov!(0x000A, 0x56),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x35),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0013,
    target: "D3DDevice__m_VertexShader_OFFSET",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3D_SETFENCE_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000D, 0x8B),
    ov!(0x0010, 0x46),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x57),
    ov!(0x0013, 0x72),
    ov!(0x0014, 0x0E),
    ov!(0x0015, 0xA1),
];
static D3D8_LTCG_D3D_SETFENCE_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x001C, 0xC7),
    ov!(0x001D, 0x00),
    ov!(0x001E, 0xC8),
    ov!(0x001F, 0x17),
    ov!(0x0020, 0x08),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0xB9),
    ov!(0x0023, 0x01),
    ov!(0x0024, 0x00),
    ov!(0x002D, 0x83),
    ov!(0x002E, 0xC0),
    ov!(0x002F, 0x0C),
    ov!(0x0033, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1036_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x53),
    ov!(0x0005, 0x0F),
    ov!(0x0006, 0xB7),
    ov!(0x0007, 0x5A),
    ov!(0x0008, 0x02),
    ov!(0x0009, 0x55),
];
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0040, 0x04),
    ov!(0x0041, 0xC7),
    ov!(0x0042, 0x00),
    ov!(0x0043, 0x94),
    ov!(0x0044, 0x1E),
    ov!(0x0045, 0x08),
    ov!(0x0046, 0x00),
    ov!(0x0047, 0x83),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETINDICES_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0014, 0x74),
    ov!(0x0015, 0x10),
    ov!(0x0016, 0x81),
    ov!(0x0017, 0x03),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x08),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETINDICES_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0076, 0xC7),
    ov!(0x0077, 0x40),
    ov!(0x0078, 0x04),
    ov!(0x0079, 0x00),
    ov!(0x007A, 0x00),
    ov!(0x007B, 0x21),
    ov!(0x007C, 0x00),
    ov!(0x007D, 0x83),
    ov!(0x007E, 0xC0),
    ov!(0x007F, 0x08),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0088, 0xC7),
    ov!(0x0089, 0x00),
    ov!(0x008A, 0xB4),
    ov!(0x008B, 0x02),
    ov!(0x008C, 0x04),
    ov!(0x008D, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0020, 0x8D),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x19),
    ov!(0x0025, 0x04),
    ov!(0x0026, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0023, 0x80),
    ov!(0x0024, 0x19),
    ov!(0x0025, 0x08),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x0F),
    ov!(0x0028, 0xBF),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0x8D),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x19),
    ov!(0x0025, 0x04),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x33),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x08),
    ov!(0x00CA, 0x77),
    ov!(0x00CB, 0x07),
    ov!(0x00CC, 0xB8),
    ov!(0x00CD, 0x00),
    ov!(0x00CE, 0x00),
    ov!(0x00CF, 0x10),
    ov!(0x00D0, 0x00),
    ov!(0x00D1, 0xEB),
];
static D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3D_BLOCKONTIME_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0110, 0x6A),
    ov!(0x0111, 0x00),
    ov!(0x0112, 0x6A),
    ov!(0x0113, 0x00),
    ov!(0x0114, 0x6A),
    ov!(0x0115, 0x01),
    ov!(0x0116, 0x6A),
    ov!(0x0117, 0x06),
    ov!(0x0118, 0x56),
];
static D3D8_LTCG_D3D_BLOCKONTIME_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3D_COMMONSETRENDERTARGET_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0040, 0x83),
    ov!(0x0041, 0xFD),
    ov!(0x0042, 0x0C),
    ov!(0x0043, 0x74),
    ov!(0x0044, 0x26),
    ov!(0x0045, 0x83),
    ov!(0x0046, 0xFD),
    ov!(0x0047, 0x0D),
    ov!(0x0048, 0x7E),
];
static D3D8_LTCG_D3D_COMMONSETRENDERTARGET_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x000D, 0x74),
    ov!(0x000E, 0x24),
    ov!(0x002C, 0xE8),
    ov!(0x0035, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1036_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0007,
        target: "OFFSET_D3DDevice__m_RenderTarget",
    },
    OovpaXref {
        offset: 0x002D,
        target: "D3DResource_AddRef",
    },
];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0007, 0x15),
    ov!(0x0019, 0x45),
    ov!(0x001A, 0x0C),
    ov!(0x001B, 0x8B),
    ov!(0x001C, 0x45),
    ov!(0x001D, 0x10),
    ov!(0x001E, 0x85),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1037_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x08),
    ov!(0x0008, 0x8B),
    ov!(0x0014, 0x89),
    ov!(0x0015, 0x75),
    ov!(0x0016, 0xF8),
    ov!(0x0017, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1037_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000A,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_1044_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x06),
    ov!(0x0010, 0x81),
    ov!(0x0011, 0x03),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x08),
    ov!(0x0015, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_1044_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0022,
        target: "D3D_g_Stream_i_pVertexBuffer",
    },
    OovpaXref {
        offset: 0x0052,
        target: "D3D_g_Stream",
    },
];

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

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_BEGIN_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0030, 0x81),
    ov!(0x0031, 0x4E),
    ov!(0x0032, 0x08),
    ov!(0x0033, 0x00),
    ov!(0x0034, 0x08),
    ov!(0x0035, 0x00),
    ov!(0x0036, 0x00),
    ov!(0x0037, 0x5E),
    ov!(0x0038, 0xC2),
    ov!(0x0039, 0x04),
];
static D3D8_LTCG_D3DDEVICE_BEGIN_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0062, 0x18),
    ov!(0x0063, 0xC7),
    ov!(0x0064, 0x00),
    ov!(0x0065, 0x9C),
    ov!(0x0066, 0x1E),
    ov!(0x0067, 0x04),
    ov!(0x0068, 0x00),
];
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETLIGHT_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0xF0),
    ov!(0x0029, 0x89),
    ov!(0x002A, 0x44),
    ov!(0x002B, 0x24),
    ov!(0x002C, 0x10),
    ov!(0x002D, 0x8D),
    ov!(0x002E, 0x04),
    ov!(0x002F, 0xC0),
];
static D3D8_LTCG_D3DDEVICE_SETLIGHT_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x001D, 0x4C),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0x20),
    ov!(0x0023, 0x03),
    ov!(0x0024, 0x08),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x89),
    ov!(0x0027, 0x48),
    ov!(0x0028, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001C, 0x8B),
    ov!(0x0022, 0x8B),
    ov!(0x0032, 0xC7),
    ov!(0x0033, 0x00),
    ov!(0x0034, 0x8C),
    ov!(0x0035, 0x03),
    ov!(0x0036, 0x08),
    ov!(0x0037, 0x00),
    ov!(0x003E, 0x83),
    ov!(0x003F, 0xC0),
    ov!(0x0040, 0x0C),
    ov!(0x0043, 0x89),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x001E,
        target: "D3DRS_TwoSidedLighting",
    },
    OovpaXref {
        offset: 0x0024,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x0045,
        target: "D3DRS_FillMode",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0023, 0x54),
    ov!(0x0024, 0x24),
    ov!(0x0025, 0x0E),
    ov!(0x0026, 0x8B),
    ov!(0x0027, 0xF9),
    ov!(0x0028, 0x81),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0029, 0x83),
    ov!(0x002A, 0xC0),
    ov!(0x002B, 0x08),
    ov!(0x0033, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0035,
        target: "D3DRS_FrontFace",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x75),
    ov!(0x0024, 0xC7),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0xBC),
    ov!(0x0027, 0x17),
    ov!(0x0028, 0x04),
    ov!(0x0029, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0034,
        target: "D3DRS_LogicOp",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0xA4),
    ov!(0x0023, 0x03),
    ov!(0x0024, 0x04),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x89),
    ov!(0x0027, 0x48),
    ov!(0x0028, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x0022, 0x6C),
    ov!(0x0023, 0x1E),
    ov!(0x0024, 0x04),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x8D),
    ov!(0x0027, 0x91),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0074, 0xC7),
    ov!(0x0076, 0x08),
    ov!(0x0077, 0x2C),
    ov!(0x0078, 0x03),
    ov!(0x0079, 0x04),
    ov!(0x007A, 0x00),
    ov!(0x0083, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0085,
        target: "D3DRS_StencilEnable",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x005D, 0xC7),
    ov!(0x005F, 0x08),
    ov!(0x0060, 0x70),
    ov!(0x0061, 0x03),
    ov!(0x0062, 0x04),
    ov!(0x0063, 0x00),
    ov!(0x006C, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x006E,
        target: "D3DRS_StencilFail",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0036, 0xC9),
    ov!(0x0037, 0xC7),
    ov!(0x0038, 0x00),
    ov!(0x0039, 0x0C),
    ov!(0x003A, 0x03),
    ov!(0x003B, 0x04),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0x89),
    ov!(0x003E, 0x48),
    ov!(0x003F, 0x04),
    ov!(0x0040, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x000E, 0x55),
    ov!(0x000F, 0x33),
    ov!(0x0010, 0xED),
    ov!(0x0011, 0x3B),
    ov!(0x0012, 0xD5),
    ov!(0x0013, 0x56),
    ov!(0x0014, 0x57),
    ov!(0x0015, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x85),
    ov!(0x000D, 0xF6),
    ov!(0x000E, 0x75),
    ov!(0x000F, 0x04),
    ov!(0x0010, 0x33),
    ov!(0x0011, 0xC0),
    ov!(0x0012, 0x5E),
    ov!(0x0013, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x74),
    ov!(0x0010, 0x0F),
    ov!(0x0011, 0x50),
    ov!(0x0012, 0xFF),
    ov!(0x0013, 0x15),
];
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0042, 0xF7),
    ov!(0x0043, 0x44),
    ov!(0x0044, 0x24),
    ov!(0x0045, 0x1C),
    ov!(0x0046, 0x00),
    ov!(0x0047, 0x00),
    ov!(0x0048, 0x01),
    ov!(0x0049, 0x00),
    ov!(0x004A, 0x74),
];
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0xA1),
    ov!(0x00FD, 0x77),
    ov!(0x00FE, 0x07),
    ov!(0x00FF, 0xB8),
    ov!(0x0100, 0x00),
    ov!(0x0101, 0x00),
    ov!(0x0102, 0x10),
    ov!(0x0103, 0x00),
    ov!(0x0104, 0xEB),
];
static D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1049_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0xC7),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x08),
    ov!(0x0022, 0x03),
    ov!(0x0023, 0x04),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1049_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0031,
        target: "D3DRS_CullMode",
    },
];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1052_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0xC7),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x08),
    ov!(0x0025, 0x03),
    ov!(0x0026, 0x04),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x75),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1052_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0034,
        target: "D3DRS_CullMode",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0072, 0xC7),
    ov!(0x0073, 0x00),
    ov!(0x0074, 0xB4),
    ov!(0x0075, 0x02),
    ov!(0x0076, 0x04),
    ov!(0x0077, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x000C, 0xA3),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1060_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3DRS_MultiSampleAntiAlias",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x3D),
    ov!(0x000B, 0xA3),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1060_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_MultiSampleMask",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x55),
    ov!(0x0003, 0x56),
    ov!(0x000D, 0x0F),
    ov!(0x000E, 0x95),
    ov!(0x000F, 0xC0),
    ov!(0x0016, 0xDB),
    ov!(0x0017, 0x44),
    ov!(0x0018, 0x24),
    ov!(0x0019, 0x18),
    ov!(0x001C, 0x7D),
    ov!(0x001D, 0x06),
    ov!(0x001E, 0xD8),
    ov!(0x001F, 0x05),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x55),
    ov!(0x000E, 0x85),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0x74),
    ov!(0x0011, 0x0F),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x15),
];
static D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1072_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0074, 0xC7),
    ov!(0x0075, 0x00),
    ov!(0x0076, 0xB4),
    ov!(0x0077, 0x02),
    ov!(0x0078, 0x04),
    ov!(0x0079, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSCISSORS_1072_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1944_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0002, 0x5C),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xC3),
    ov!(0x000D, 0xC1),
    ov!(0x000E, 0xE0),
    ov!(0x000F, 0x07),
    ov!(0x0010, 0x57),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x3D),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1944_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "D3DTSS_TEXCOORDINDEX",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1958_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0008, 0x35),
    ov!(0x0014, 0xC1),
    ov!(0x0015, 0xE0),
    ov!(0x0016, 0x07),
    ov!(0x0017, 0x89),
    ov!(0x0018, 0x98),
    ov!(0x001D, 0x8B),
    ov!(0x001E, 0x06),
    ov!(0x001F, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1958_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "D3DTSS_TEXCOORDINDEX",
}];

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

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DELETEPIXELSHADER_0__LTCG_EAX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x08),
    ov!(0x0002, 0x75),
    ov!(0x0003, 0x0D),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0x04),
    ov!(0x0007, 0x85),
    ov!(0x0008, 0xC9),
    ov!(0x0009, 0x74),
    ov!(0x000A, 0x06),
    ov!(0x000B, 0x50),
    ov!(0x000C, 0xE8),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_DELETEPIXELSHADER_0__LTCG_EAX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_LIGHTENABLE_4__LTCG_EAX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0021, 0xF6),
    ov!(0x0022, 0x44),
    ov!(0x0023, 0x01),
    ov!(0x0024, 0x68),
    ov!(0x0025, 0x01),
    ov!(0x0026, 0x75),
];
static D3D8_LTCG_D3DDEVICE_LIGHTENABLE_4__LTCG_EAX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_MULTIPLYTRANSFORM_0__LTCG_EBX1_EAX2_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xE4),
    ov!(0x0005, 0xF0),
    ov!(0x0006, 0x81),
    ov!(0x0007, 0xEC),
    ov!(0x0008, 0x88),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x56),
    ov!(0x000D, 0x57),
    ov!(0x000E, 0x8B),
    ov!(0x000F, 0xF0),
    ov!(0x0010, 0xB9),
    ov!(0x0011, 0x10),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x8D),
    ov!(0x0016, 0x7C),
];
static D3D8_LTCG_D3DDEVICE_MULTIPLYTRANSFORM_0__LTCG_EBX1_EAX2_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x001E, 0xB9),
    ov!(0x001F, 0x18),
    ov!(0x0020, 0x15),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0xEB),
    ov!(0x0024, 0x09),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DRESOURCE_GETTYPE_0__LTCG_ECX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x01),
    ov!(0x0007, 0x3D),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x03),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x77),
    ov!(0x000D, 0x22),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x1A),
];
static D3D8_LTCG_D3DRESOURCE_GETTYPE_0__LTCG_ECX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x0F),
    ov!(0x0001, 0xB6),
    ov!(0x002A, 0x79),
    ov!(0x002B, 0x09),
    ov!(0x002C, 0xC7),
    ov!(0x002D, 0x46),
    ov!(0x002E, 0x08),
    ov!(0x002F, 0x01),
    ov!(0x0030, 0x00),
    ov!(0x0031, 0x00),
];
static D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_GETVIEWPORTOFFSETANDSCALE_0__LTCG_EDX1_ECX2_2024_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0xDB),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x85),
    ov!(0x0016, 0xF6),
    ov!(0x0017, 0x57),
    ov!(0x0018, 0x7D),
    ov!(0x0019, 0x06),
    ov!(0x001A, 0xD8),
    ov!(0x001B, 0x05),
];
static D3D8_LTCG_D3DDEVICE_GETVIEWPORTOFFSETANDSCALE_0__LTCG_EDX1_ECX2_2024_XREFS: &[OovpaXref] =
    &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_0__LTCG_EAX1_EBX2_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x0003, 0x8B),
    ov!(0x003D, 0xC7),
    ov!(0x003E, 0x00),
    ov!(0x003F, 0x94),
    ov!(0x0040, 0x1E),
    ov!(0x0041, 0x08),
    ov!(0x0042, 0x00),
    ov!(0x0046, 0xC7),
    ov!(0x0047, 0x40),
    ov!(0x0048, 0x04),
    ov!(0x0049, 0x06),
    ov!(0x004A, 0x00),
    ov!(0x0050, 0x83),
    ov!(0x0051, 0xC0),
    ov!(0x0052, 0x0C),
    ov!(0x0084, 0x5E),
    ov!(0x0085, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_0__LTCG_EAX1_EBX2_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETFLICKERFILTER_0__LTCG_ESI1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x001D, 0x6A),
    ov!(0x001E, 0x00),
    ov!(0x001F, 0x56),
    ov!(0x0020, 0x6A),
    ov!(0x0021, 0x0B),
    ov!(0x0022, 0x50),
    ov!(0x0023, 0xFF),
    ov!(0x0024, 0x15),
];
static D3D8_LTCG_D3DDEVICE_SETFLICKERFILTER_0__LTCG_ESI1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x006D, 0xC7),
    ov!(0x006E, 0x40),
    ov!(0x006F, 0x04),
    ov!(0x0070, 0x00),
    ov!(0x0071, 0x00),
    ov!(0x0072, 0x21),
    ov!(0x0073, 0x00),
    ov!(0x0074, 0x83),
    ov!(0x0075, 0xC0),
    ov!(0x0076, 0x08),
    ov!(0x007B, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x000C, 0x75),
    ov!(0x000D, 0x05),
    ov!(0x000E, 0xBB),
    ov!(0x000F, 0x05),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETINDICES_4__LTCG_EBX1_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xDB),
    ov!(0x0006, 0x08),
    ov!(0x000F, 0x74),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x81),
    ov!(0x0012, 0x03),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x08),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETINDICES_4__LTCG_EBX1_2024_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2024_ENTRIES: &[OovpaEntry] = &[
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
    ov!(0x0081, 0x83),
    ov!(0x0082, 0xC4),
    ov!(0x0083, 0x18),
    ov!(0x0084, 0xC3),
];
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2024_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0002,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4721.inl
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_4__LTCG_ECX2_EAX3_2024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x55),
    ov!(0x0002, 0x8B),
    ov!(0x0003, 0x2D),
    ov!(0x000A, 0x6A),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x55),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0xF8),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xD9),
    ov!(0x0011, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_4__LTCG_ECX2_EAX3_2024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0004,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0012,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADERDIRECT_0__LTCG_EAX1_EBX2_2024_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0010, 0x01),
    ov!(0x0011, 0xB9),
    ov!(0x0012, 0x40),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0xBF),
    ov!(0x0025, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADERDIRECT_0__LTCG_EAX1_EBX2_2024_XREFS: &[OovpaXref] =
    &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_4__LTCG_EAX1_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0050, 0xC7),
    ov!(0x0051, 0x00),
    ov!(0x0052, 0x9C),
    ov!(0x0053, 0x1E),
    ov!(0x0054, 0x04),
    ov!(0x0055, 0x00),
    ov!(0x0056, 0x89),
    ov!(0x0057, 0x48),
    ov!(0x0058, 0x04),
];
static D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_4__LTCG_EAX1_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x0F),
    ov!(0x0001, 0xB6),
    ov!(0x0029, 0x79),
    ov!(0x002A, 0x09),
    ov!(0x002B, 0xC7),
    ov!(0x002C, 0x46),
    ov!(0x002D, 0x08),
    ov!(0x002E, 0x01),
    ov!(0x002F, 0x00),
    ov!(0x0030, 0x00),
];
static D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x85),
    ov!(0x0072, 0xC7),
    ov!(0x0073, 0x40),
    ov!(0x0074, 0x04),
    ov!(0x0075, 0x00),
    ov!(0x0076, 0x00),
    ov!(0x0077, 0x21),
    ov!(0x0078, 0x00),
    ov!(0x0079, 0x83),
    ov!(0x007A, 0xC0),
    ov!(0x007B, 0x08),
    ov!(0x0080, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0021, 0xB9),
    ov!(0x0022, 0x18),
    ov!(0x0023, 0x15),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0xEB),
    ov!(0x0027, 0x09),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_ECX6_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000A, 0x85),
    ov!(0x000B, 0xC9),
    ov!(0x000C, 0x75),
    ov!(0x000D, 0x0A),
    ov!(0x000E, 0xC7),
    ov!(0x000F, 0x05),
    ov!(0x003A, 0x18),
];
static D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_ECX6_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4721.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX1_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x002B, 0x81),
    ov!(0x002C, 0xC1),
    ov!(0x002D, 0x00),
    ov!(0x002E, 0x00),
    ov!(0x002F, 0xF8),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX1_2036_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4721.inl
static D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2036_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x56),
    ov!(0x0026, 0x75),
    ov!(0x0027, 0x05),
    ov!(0x0028, 0xBB),
    ov!(0x0029, 0x05),
    ov!(0x002A, 0x00),
    ov!(0x002B, 0x00),
    ov!(0x002C, 0x00),
    ov!(0x002D, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2036_XREFS: &[OovpaXref] = &[];

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

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EAX1_2040_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x06),
    ov!(0x0015, 0x81),
    ov!(0x0016, 0x03),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x08),
    ov!(0x001A, 0x00),
];
static D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EAX1_2040_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0023,
        target: "D3D_g_Stream_i_pVertexBuffer",
    },
    OovpaXref {
        offset: 0x0053,
        target: "D3D_g_Stream",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2040_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x8B),
    ov!(0x0003, 0x5C),
    ov!(0x0004, 0x24),
    ov!(0x0005, 0x0C),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0xC6),
    ov!(0x0012, 0x89),
    ov!(0x0018, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2040_XREFS: &[OovpaXref] =
    &[OovpaXref {
        offset: 0x0014,
        target: "D3DTSS_TEXCOORDINDEX",
    }];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2045_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x8B),
    ov!(0x0003, 0x1D),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0xC7),
    ov!(0x000F, 0xC1),
    ov!(0x0010, 0xE0),
    ov!(0x0011, 0x07),
    ov!(0x0012, 0x89),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2045_XREFS: &[OovpaXref] =
    &[OovpaXref {
        offset: 0x0014,
        target: "D3DTSS_TEXCOORDINDEX",
    }];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER_8__LTCG_EAX1_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xF8),
    ov!(0x0009, 0x75),
    ov!(0x000A, 0x07),
    ov!(0x000B, 0xB8),
    ov!(0x000C, 0x01),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0xEB),
    ov!(0x0011, 0x07),
    ov!(0x0012, 0xF7),
    ov!(0x004C, 0xC2),
    ov!(0x004D, 0x08),
];
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER_8__LTCG_EAX1_2048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0005,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0045,
        target: "D3DResource_AddRef",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0023, 0x8B),
    ov!(0x0024, 0xCE),
    ov!(0x0025, 0xC1),
    ov!(0x0026, 0xE1),
    ov!(0x0027, 0x06),
    ov!(0x0028, 0x81),
    ov!(0x0029, 0xC1),
    ov!(0x002A, 0x24),
    ov!(0x0044, 0xC2),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_LOCK2DSURFACE_16__LTCG_ESI4_EAX5_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x53),
    ov!(0x0004, 0x24),
    ov!(0x0005, 0x18),
    ov!(0x0006, 0xF6),
    ov!(0x0007, 0xC3),
    ov!(0x0008, 0x20),
    ov!(0x0009, 0x55),
    ov!(0x000A, 0x8B),
];
static D3D8_LTCG_LOCK2DSURFACE_16__LTCG_ESI4_EAX5_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_LOCK3DSURFACE_16__LTCG_EAX4_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0003, 0x53),
    ov!(0x0004, 0x8A),
    ov!(0x0005, 0x5C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x1C),
    ov!(0x0008, 0xF6),
    ov!(0x0009, 0xC3),
    ov!(0x000A, 0x20),
    ov!(0x000B, 0x55),
];
static D3D8_LTCG_LOCK3DSURFACE_16__LTCG_EAX4_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0043, 0x00),
    ov!(0x0044, 0x81),
    ov!(0x0045, 0x07),
    ov!(0x0046, 0x00),
    ov!(0x0047, 0x00),
    ov!(0x0048, 0x08),
    ov!(0x0049, 0x00),
    ov!(0x004A, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_4__LTCG_EAX2_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x57),
    ov!(0x000C, 0x6A),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x56),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xF8),
    ov!(0x0011, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_4__LTCG_EAX2_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_0__LTCG_EAX1_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0xE8),
    ov!(0x000C, 0xB8),
    ov!(0x000D, 0x0E),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x07),
    ov!(0x0010, 0x80),
    ov!(0x0011, 0x5E),
    ov!(0x0012, 0xC3),
    ov!(0x0013, 0x57),
];
static D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_0__LTCG_EAX1_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_RUNVERTEXSTATESHADER_4__LTCG_ESI2_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x57),
    ov!(0x002E, 0xC7),
    ov!(0x002F, 0x40),
    ov!(0x0030, 0xEC),
    ov!(0x0031, 0x80),
    ov!(0x0032, 0x1E),
    ov!(0x0033, 0x10),
    ov!(0x0034, 0x00),
    ov!(0x0035, 0xD9),
];
static D3D8_LTCG_D3DDEVICE_RUNVERTEXSTATESHADER_4__LTCG_ESI2_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATENOTINLINE_0__LTCG_ESI1_EDI2_2048_ENTRIES: &[OovpaEntry] =
    &[
        ov!(0x0000, 0x83),
        ov!(0x0001, 0xFE),
        ov!(0x001A, 0xC3),
        ov!(0x001B, 0x81),
        ov!(0x001C, 0xFE),
        ov!(0x001D, 0x88),
        ov!(0x001E, 0x00),
        ov!(0x001F, 0x00),
        ov!(0x0020, 0x00),
        ov!(0x0021, 0x7D),
        ov!(0x0022, 0x1D),
        ov!(0x0023, 0x8B),
        ov!(0x0024, 0x0D),
    ];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATENOTINLINE_0__LTCG_ESI1_EDI2_2048_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000F,
        target: "D3DDevice_SetRenderState_Simple",
    },
    OovpaXref {
        offset: 0x0016,
        target: "D3D_g_RenderState",
    },
];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SETSOFTDISPLAYFILTER_0__LTCG_UNK1_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x0016, 0x6A),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x56),
    ov!(0x0019, 0x6A),
    ov!(0x001A, 0x0E),
    ov!(0x001B, 0x52),
    ov!(0x001C, 0xFF),
    ov!(0x001D, 0x15),
];
static D3D8_LTCG_D3DDEVICE_SETSOFTDISPLAYFILTER_0__LTCG_UNK1_2048_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xEC),
    ov!(0x0008, 0x18),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xF1),
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
];
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2048_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0002,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_GET2DSURFACEDESC_4__LTCG_EDI1_ESI3_2048_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x0F),
    ov!(0x0001, 0xB6),
    ov!(0x002F, 0x79),
    ov!(0x0030, 0x09),
    ov!(0x0031, 0xC7),
    ov!(0x0032, 0x46),
    ov!(0x0033, 0x08),
    ov!(0x0034, 0x01),
    ov!(0x0035, 0x00),
    ov!(0x0036, 0x00),
];
static D3D8_LTCG_GET2DSURFACEDESC_4__LTCG_EDI1_ESI3_2048_XREFS: &[OovpaXref] = &[];

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

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0026, 0x8B),
    ov!(0x0027, 0xCE),
    ov!(0x0028, 0xC1),
    ov!(0x0029, 0xE1),
    ov!(0x002A, 0x06),
    ov!(0x002B, 0x81),
    ov!(0x002C, 0xC1),
    ov!(0x002D, 0x24),
    ov!(0x0047, 0xC2),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4928.inl
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2060_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0046, 0x00),
    ov!(0x0047, 0x81),
    ov!(0x0048, 0x07),
    ov!(0x0049, 0x00),
    ov!(0x004A, 0x00),
    ov!(0x004B, 0x08),
    ov!(0x004C, 0x00),
    ov!(0x004D, 0x8B),
];
static D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2060_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2072_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x006B, 0xC7),
    ov!(0x006C, 0x40),
    ov!(0x006D, 0x04),
    ov!(0x006E, 0x00),
    ov!(0x006F, 0x00),
    ov!(0x0070, 0x21),
    ov!(0x0071, 0x00),
    ov!(0x0072, 0x83),
    ov!(0x0073, 0xC0),
    ov!(0x0074, 0x08),
    ov!(0x0079, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2072_XREFS: &[OovpaXref] = &[];

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

// Source: D3D8LTCG/4034.inl
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_4034_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x15),
    ov!(0x0016, 0xFF),
    ov!(0x0017, 0x15),
    ov!(0x0022, 0x55),
    ov!(0x0063, 0x8B),
    ov!(0x0064, 0xB3),
    ov!(0x0069, 0xE8),
    ov!(0x006E, 0x89),
    ov!(0x0074, 0x39),
];
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_4034_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0002,
        target: "KT_FUNC_AvGetSavedDataAddress",
    },
    OovpaXref {
        offset: 0x0018,
        target: "KT_FUNC_AvSendTVEncoderOption",
    },
];

// Source: D3D8LTCG/4034.inl
static D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_4034_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x1D),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xC8),
    ov!(0x000A, 0xC1),
    ov!(0x000B, 0xE1),
    ov!(0x000C, 0x06),
    ov!(0x00F4, 0xE8),
    ov!(0x0103, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_4034_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x00F5,
        target: "D3D_UpdateProjectionViewportTransform",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATACOLOR_4038_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x74),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x0C),
    ov!(0x0005, 0x57),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x3D),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0x07),
    ov!(0x000E, 0x3B),
    ov!(0x000F, 0x47),
    ov!(0x0010, 0x04),
    ov!(0x0011, 0x72),
    ov!(0x0013, 0xA1),
];
static D3D8_LTCG_D3DDEVICE_SETVERTEXDATACOLOR_4038_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_ESI1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0018, 0x72),
    ov!(0x0019, 0x04),
    ov!(0x001A, 0x89),
    ov!(0x001B, 0x44),
    ov!(0x001C, 0x24),
    ov!(0x001E, 0x8B),
    ov!(0x0020, 0x08),
    ov!(0x0021, 0xE8),
    ov!(0x002A, 0x8B),
    ov!(0x002C, 0x24),
    ov!(0x004E, 0x89),
    ov!(0x004F, 0x86),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_ESI1_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x42),
    ov!(0x0002, 0x08),
    ov!(0x0082, 0x5F),
    ov!(0x0083, 0x0D),
    ov!(0x0084, 0x00),
    ov!(0x0085, 0x20),
    ov!(0x0086, 0x00),
    ov!(0x0087, 0x00),
    ov!(0x0088, 0x5D),
    ov!(0x008D, 0xC3),
];
static D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0xEC),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0xA9),
    ov!(0x0009, 0x8F),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0x3F),
    ov!(0x000F, 0xE8),
    ov!(0x001B, 0x0F),
    ov!(0x001C, 0x84),
    ov!(0x0021, 0x8B),
    ov!(0x0022, 0x4E),
    ov!(0x0023, 0x04),
    ov!(0x0024, 0x83),
    ov!(0x0025, 0xE0),
    ov!(0x0026, 0xDF),
    ov!(0x0027, 0x83),
    ov!(0x0028, 0xC8),
    ov!(0x0029, 0x50),
];
static D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_CDEVICE_SETSTATEVB_8_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0004, 0x8B),
    ov!(0x000A, 0x8B),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xE0),
    ov!(0x000E, 0xAF),
    ov!(0x000F, 0xF7),
    ov!(0x0010, 0xC3),
    ov!(0x0011, 0x8F),
    ov!(0x0012, 0xFF),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x3F),
];
static D3D8_LTCG_CDEVICE_SETSTATEVB_8_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4__LTCG_EAX1_EDX3_ECX4_EDI6_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x01),
    ov!(0x0002, 0x51),
    ov!(0x0007, 0x52),
    ov!(0x0009, 0x6A),
    ov!(0x000A, 0x01),
    ov!(0x000B, 0x50),
    ov!(0x000C, 0x50),
    ov!(0x000D, 0x32),
    ov!(0x000E, 0xD2),
    ov!(0x000F, 0xE8),
    ov!(0x0014, 0xC2),
    ov!(0x0015, 0x08),
];
static D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4__LTCG_EAX1_EDX3_ECX4_EDI6_4039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0010,
    target: "D3D_CreateTexture",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE_8__LTCG_EDX3_ECX4_EAX5_EDI7_4039_ENTRIES: &[OovpaEntry] =
    &[
        ov!(0x0000, 0x6A),
        ov!(0x0001, 0x00),
        ov!(0x0002, 0x50),
        ov!(0x0007, 0x51),
        ov!(0x000C, 0x52),
        ov!(0x000D, 0x6A),
        ov!(0x000E, 0x01),
        ov!(0x0011, 0x32),
        ov!(0x0012, 0xD2),
        ov!(0x0013, 0xE8),
        ov!(0x0018, 0xC2),
        ov!(0x0019, 0x0C),
    ];
static D3D8_LTCG_D3DDEVICE_CREATETEXTURE_8__LTCG_EDX3_ECX4_EAX5_EDI7_4039_XREFS: &[OovpaXref] =
    &[OovpaXref {
        offset: 0x0014,
        target: "D3D_CreateTexture",
    }];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_CREATEVOLUMETEXTURE_12__LTCG_EDX4_ECX5_EAX6_EDI8_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x00),
    ov!(0x0002, 0x50),
    ov!(0x0007, 0x51),
    ov!(0x000C, 0x52),
    ov!(0x0014, 0xB2),
    ov!(0x0015, 0x01),
    ov!(0x0016, 0xE8),
    ov!(0x001B, 0xC2),
    ov!(0x001C, 0x10),
];
static D3D8_LTCG_D3DDEVICE_CREATEVOLUMETEXTURE_12__LTCG_EDX4_ECX5_EAX6_EDI8_4039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0017,
    target: "D3D_CreateTexture",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_8__LTCG_EAX3_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x5C),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x0C),
    ov!(0x0005, 0x55),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x2D),
    ov!(0x000E, 0x6A),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0xF8),
    ov!(0x0013, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_DRAWVERTICES_8__LTCG_EAX3_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0008,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0014,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0032, 0x75),
    ov!(0x0034, 0xE8),
    ov!(0x0039, 0x57),
];
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x0003, 0x8B),
    ov!(0x004C, 0xC7),
    ov!(0x004D, 0x00),
    ov!(0x004E, 0x94),
    ov!(0x004F, 0x1E),
    ov!(0x0050, 0x08),
    ov!(0x0051, 0x00),
    ov!(0x0052, 0xC7),
    ov!(0x0053, 0x40),
    ov!(0x0054, 0x04),
    ov!(0x0055, 0x06),
    ov!(0x0056, 0x00),
    ov!(0x005C, 0x8D),
    ov!(0x005D, 0x48),
    ov!(0x005E, 0x0C),
    ov!(0x0092, 0xC2),
    ov!(0x0093, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFE),
    ov!(0x0003, 0x7D),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x0C),
    ov!(0x0007, 0xB5),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0xD7),
    ov!(0x000E, 0xE8),
    ov!(0x0013, 0x89),
    ov!(0x0014, 0x3C),
    ov!(0x0015, 0xB5),
];
static D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000F,
        target: "D3DDevice_SetRenderState_Simple",
    },
    OovpaXref {
        offset: 0x0016,
        target: "D3D_g_RenderState",
    },
];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFA),
    ov!(0x0018, 0xC1),
    ov!(0x0019, 0xE0),
    ov!(0x001A, 0x05),
    ov!(0x0020, 0x89),
    ov!(0x002E, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE2_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFA),
    ov!(0x0018, 0xC1),
    ov!(0x0019, 0xE0),
    ov!(0x001A, 0x05),
    ov!(0x0020, 0x89),
    ov!(0x002E, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE2_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0xF0),
    ov!(0x0012, 0x8B),
    ov!(0x0014, 0x3B),
    ov!(0x0015, 0xC1),
    ov!(0x0016, 0x72),
    ov!(0x0017, 0x0B),
    ov!(0x0023, 0x8D),
    ov!(0x0026, 0xE0),
    ov!(0x0027, 0x0A),
    ov!(0x0028, 0x04),
    ov!(0x0029, 0x00),
    ov!(0x0040, 0xC2),
    ov!(0x0041, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4039_XREFS: &[OovpaXref] =
    &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EBX1_EAX2_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x18),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x2D),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xF0),
    ov!(0x002C, 0x89),
    ov!(0x002D, 0x54),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x20),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x54),
    ov!(0x0032, 0x24),
    ov!(0x0033, 0x1C),
];
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EBX1_EAX2_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0006,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_ECX1_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x11),
    ov!(0x0022, 0xF7),
    ov!(0x0023, 0xC2),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x78),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x75),
];
static D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_ECX1_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3D_CDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFA),
    ov!(0x0018, 0xC1),
    ov!(0x0019, 0xE0),
    ov!(0x001A, 0x05),
    ov!(0x001F, 0x89),
    ov!(0x0030, 0xC3),
];
static D3D8_LTCG_D3D_CDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0022,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3D_CREATETEXTURE_28__LTCG_DL8_EDI9_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x56),
    ov!(0x0002, 0x8D),
    ov!(0x0041, 0x83),
    ov!(0x0042, 0x64),
    ov!(0x0043, 0x24),
    ov!(0x0044, 0x24),
    ov!(0x0045, 0xF7),
    ov!(0x0046, 0x6A),
    ov!(0x0047, 0x14),
    ov!(0x0048, 0x6A),
    ov!(0x0049, 0x40),
];
static D3D8_LTCG_D3D_CREATETEXTURE_28__LTCG_DL8_EDI9_4039_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4040_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0xEC),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0xA9),
    ov!(0x0009, 0x8F),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0x3F),
    ov!(0x001B, 0x0F),
    ov!(0x001C, 0x84),
    ov!(0x0021, 0x83),
    ov!(0x0022, 0xE0),
    ov!(0x0023, 0xDF),
    ov!(0x0024, 0x83),
    ov!(0x0025, 0xC8),
    ov!(0x0026, 0x50),
    ov!(0x0032, 0x8B),
    ov!(0x0033, 0x4E),
    ov!(0x0034, 0x04),
];
static D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4040_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4040_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0032, 0x75),
    ov!(0x0035, 0xE8),
    ov!(0x003A, 0x57),
];
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4040_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/4039.inl
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4041_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0033, 0x75),
    ov!(0x0036, 0xE8),
    ov!(0x003B, 0x57),
];
static D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4041_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0037,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0006, 0xFF),
    ov!(0x0007, 0x15),
    ov!(0x002E, 0x74),
    ov!(0x002F, 0x20),
    ov!(0x0030, 0x8B),
    ov!(0x0031, 0x48),
    ov!(0x0032, 0x14),
    ov!(0x006F, 0xE8),
    ov!(0x0074, 0x89),
    ov!(0x007A, 0x39),
];
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4432_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "KT_FUNC_AvGetSavedDataAddress",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0017, 0x8B),
    ov!(0x0019, 0x08),
    ov!(0x0027, 0x8B),
    ov!(0x0029, 0x24),
    ov!(0x0161, 0x83),
    ov!(0x0162, 0xC0),
    ov!(0x0163, 0x18),
    ov!(0x0164, 0x49),
    ov!(0x0176, 0x8D),
    ov!(0x0177, 0x83),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4432_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_CDEVICE_KICKOFF_4_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0007, 0xF6),
    ov!(0x0008, 0xC5),
    ov!(0x0009, 0x20),
    ov!(0x00B6, 0x81),
    ov!(0x00B7, 0xC9),
    ov!(0x00B8, 0x00),
    ov!(0x00B9, 0x20),
    ov!(0x00BA, 0x00),
    ov!(0x00BB, 0x00),
    ov!(0x00C1, 0xC2),
    ov!(0x00C2, 0x04),
];
static D3D8_LTCG_CDEVICE_KICKOFF_4_4432_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x10),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x01),
    ov!(0x0013, 0x50),
    ov!(0x0018, 0x51),
    ov!(0x001A, 0x6A),
    ov!(0x001B, 0x01),
    ov!(0x001C, 0x50),
    ov!(0x001D, 0x50),
    ov!(0x001E, 0x32),
    ov!(0x001F, 0xD2),
    ov!(0x0020, 0xE8),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x18),
];
static D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4432_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "D3D_CreateTexture",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_RESET_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0032, 0x75),
    ov!(0x0035, 0xE8),
    ov!(0x003A, 0x8B),
    ov!(0x003C, 0x24),
    ov!(0x003D, 0x14),
];
static D3D8_LTCG_D3DDEVICE_RESET_4432_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EAX2_EDX3_4432_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xF8),
    ov!(0x0016, 0xC1),
    ov!(0x0017, 0xE1),
    ov!(0x0018, 0x05),
    ov!(0x0022, 0x89),
    ov!(0x002A, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EAX2_EDX3_4432_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0025,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x0009, 0x3B),
    ov!(0x000A, 0x46),
    ov!(0x000B, 0x04),
    ov!(0x000C, 0x72),
    ov!(0x000D, 0x0E),
    ov!(0x001C, 0x8B),
    ov!(0x001E, 0x24),
    ov!(0x001F, 0x08),
    ov!(0x0020, 0x8D),
    ov!(0x0023, 0xE0),
    ov!(0x0024, 0x0A),
    ov!(0x0025, 0x04),
    ov!(0x0026, 0x00),
    ov!(0x0029, 0x8B),
    ov!(0x002B, 0x24),
    ov!(0x002C, 0x0C),
    ov!(0x003F, 0xC2),
    ov!(0x0040, 0x08),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4432_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_ECX1_EAX2_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x18),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x2D),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0xF0),
    ov!(0x0013, 0x8B),
    ov!(0x0014, 0xD9),
    ov!(0x002F, 0x89),
    ov!(0x0030, 0x54),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x24),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x54),
    ov!(0x0035, 0x24),
    ov!(0x0036, 0x20),
];
static D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_ECX1_EAX2_4432_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0007,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4433_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x15),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x20),
    ov!(0x002F, 0x8B),
    ov!(0x0030, 0x48),
    ov!(0x0031, 0x14),
    ov!(0x006E, 0xE8),
    ov!(0x0073, 0x89),
    ov!(0x0079, 0x39),
];
static D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4433_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "KT_FUNC_AvGetSavedDataAddress",
}];

// Source: D3D8LTCG/4432.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4433_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0017, 0x8B),
    ov!(0x0019, 0x08),
    ov!(0x0027, 0x8B),
    ov!(0x0029, 0x24),
    ov!(0x0160, 0x83),
    ov!(0x0161, 0xC0),
    ov!(0x0162, 0x18),
    ov!(0x0163, 0x49),
    ov!(0x0175, 0x8D),
    ov!(0x0176, 0x83),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4433_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4531_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0018, 0x72),
    ov!(0x0019, 0x04),
    ov!(0x001A, 0x89),
    ov!(0x001B, 0x44),
    ov!(0x001C, 0x24),
    ov!(0x001E, 0x8B),
    ov!(0x0020, 0x08),
    ov!(0x0021, 0xE8),
    ov!(0x002A, 0x8B),
    ov!(0x002C, 0x24),
    ov!(0x0142, 0x8B),
    ov!(0x0143, 0xF8),
    ov!(0x0158, 0x83),
    ov!(0x0159, 0xC0),
    ov!(0x015A, 0x18),
    ov!(0x015B, 0x49),
    ov!(0x0171, 0x8D),
    ov!(0x0172, 0x83),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4531_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_CDEVICE_KICKOFF_4_4531_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xF6),
    ov!(0x0005, 0x40),
    ov!(0x0006, 0x08),
    ov!(0x0007, 0x04),
    ov!(0x0009, 0x74),
    ov!(0x000A, 0x08),
    ov!(0x00B8, 0x81),
    ov!(0x00B9, 0xC9),
    ov!(0x00BA, 0x00),
    ov!(0x00BB, 0x20),
    ov!(0x00BC, 0x00),
    ov!(0x00BD, 0x00),
    ov!(0x00C3, 0xC2),
    ov!(0x00C4, 0x04),
];
static D3D8_LTCG_CDEVICE_KICKOFF_4_4531_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4531_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x0003, 0x8B),
    ov!(0x004C, 0xC7),
    ov!(0x004D, 0x00),
    ov!(0x004E, 0x94),
    ov!(0x004F, 0x1E),
    ov!(0x0050, 0x08),
    ov!(0x0051, 0x00),
    ov!(0x0055, 0xC7),
    ov!(0x0056, 0x40),
    ov!(0x0057, 0x04),
    ov!(0x0058, 0x06),
    ov!(0x0059, 0x00),
    ov!(0x005F, 0x83),
    ov!(0x0060, 0xC0),
    ov!(0x0061, 0x0C),
    ov!(0x0095, 0xC2),
    ov!(0x0096, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4531_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ESI3_4531_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xFA),
    ov!(0x0015, 0xC1),
    ov!(0x0016, 0xE0),
    ov!(0x0017, 0x05),
    ov!(0x0022, 0x89),
    ov!(0x002A, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ESI3_4531_XREFS:
    &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8LTCG/4531.inl
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4531_ENTRIES:
    &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0xF0),
    ov!(0x0012, 0x8B),
    ov!(0x0014, 0x3B),
    ov!(0x0015, 0xC1),
    ov!(0x0016, 0x72),
    ov!(0x0017, 0x0E),
    ov!(0x0026, 0x8D),
    ov!(0x0029, 0xE0),
    ov!(0x002A, 0x0A),
    ov!(0x002B, 0x04),
    ov!(0x002C, 0x00),
    ov!(0x0043, 0xC2),
    ov!(0x0044, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4531_XREFS: &[OovpaXref] =
    &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_4626_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000D, 0x75),
    ov!(0x000E, 0x07),
    ov!(0x0014, 0xEB),
    ov!(0x0015, 0x07),
    ov!(0x0016, 0xF7),
    ov!(0x0017, 0xD8),
    ov!(0x0018, 0x1B),
    ov!(0x0019, 0xC0),
    ov!(0x001A, 0x83),
    ov!(0x001B, 0xE0),
    ov!(0x001C, 0x02),
];
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_4626_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x001D, 0x72),
    ov!(0x001E, 0x04),
    ov!(0x001F, 0x89),
    ov!(0x0020, 0x44),
    ov!(0x0021, 0x24),
    ov!(0x0023, 0x8B),
    ov!(0x0025, 0x08),
    ov!(0x0026, 0xE8),
    ov!(0x002F, 0x8B),
    ov!(0x0031, 0x24),
];
static D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_4627_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_COPYRECTS_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x81),
    ov!(0x0001, 0xEC),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0xB4),
    ov!(0x000B, 0x24),
    ov!(0x0010, 0x0F),
    ov!(0x0011, 0xB6),
    ov!(0x0012, 0x56),
    ov!(0x0013, 0x0D),
    ov!(0x0014, 0x8A),
    ov!(0x0015, 0x9A),
];
static D3D8_LTCG_D3DDEVICE_COPYRECTS_4627_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_0__LTCG_EAX1_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xF8),
    ov!(0x0002, 0xFF),
    ov!(0x0009, 0x75),
    ov!(0x000A, 0x07),
    ov!(0x000B, 0xB8),
    ov!(0x000C, 0x01),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0xEB),
    ov!(0x0011, 0x07),
    ov!(0x0012, 0xF7),
    ov!(0x0048, 0xC3),
];
static D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_0__LTCG_EAX1_4627_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0005,
    target: "D3D_g_pDevice",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_RESET_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x5C),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0033, 0x75),
    ov!(0x0036, 0xE8),
];
static D3D8_LTCG_D3DDEVICE_RESET_4627_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0037,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x85),
    ov!(0x0001, 0xC0),
    ov!(0x0003, 0x8B),
    ov!(0x0042, 0xC7),
    ov!(0x0043, 0x00),
    ov!(0x0044, 0x94),
    ov!(0x0045, 0x1E),
    ov!(0x0046, 0x08),
    ov!(0x0047, 0x00),
    ov!(0x004B, 0xC7),
    ov!(0x004C, 0x40),
    ov!(0x004D, 0x04),
    ov!(0x004E, 0x06),
    ov!(0x004F, 0x00),
    ov!(0x0055, 0x83),
    ov!(0x0056, 0xC0),
    ov!(0x0057, 0x0C),
    ov!(0x008B, 0xC2),
    ov!(0x008C, 0x04),
];
static D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4627_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4627.inl
static D3D8_LTCG_D3DDEVICE_COPYRECTS_4628_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4A),
    ov!(0x0006, 0x10),
    ov!(0x0008, 0xEC),
];
static D3D8_LTCG_D3DDEVICE_COPYRECTS_4628_XREFS: &[OovpaXref] = &[];

// Source: D3D8LTCG/4721.inl
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_ECX1_EAX2_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x18),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0xF0),
    ov!(0x0009, 0xA1),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xD9),
    ov!(0x002D, 0xC7),
    ov!(0x002E, 0x44),
    ov!(0x002F, 0x24),
    ov!(0x0030, 0x20),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0x00),
    ov!(0x0034, 0x00),
    ov!(0x0035, 0xC7),
    ov!(0x0036, 0x44),
    ov!(0x0037, 0x24),
    ov!(0x0038, 0x1C),
];
static D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_ECX1_EAX2_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "D3D_g_pDevice",
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
        name: "D3DDevice_BeginVisibilityTest",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVerticesUP",
        detect_size: 0x0015,
        entries: D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_1024_ENTRIES,
        argc: 3,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_IsFencePending",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_ISFENCEPENDING_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetIndices",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_SETINDICES_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleMode",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMODE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2f",
        detect_size: 0x003C,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2s",
        detect_size: 0x0024,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4s",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4ub",
        detect_size: 0x0025,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShader",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShaderInput",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERINPUT_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DSurface_GetDesc",
        detect_size: 0x000E,
        entries: D3D8_LTCG_D3DSURFACE_GETDESC_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3D_SetFence",
        detect_size: 0x002C,
        entries: D3D8_LTCG_D3D_SETFENCE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DCubeTexture_GetCubeMapSurface",
        detect_size: 0x004F,
        entries: D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_Begin",
        detect_size: 0x003D,
        entries: D3D8_LTCG_D3DDEVICE_BEGIN_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_Clear",
        detect_size: 0x0046,
        entries: D3D8_LTCG_D3DDEVICE_CLEAR_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_CreateTexture",
        detect_size: 0x002C,
        entries: D3D8_LTCG_D3DDEVICE_CREATETEXTURE_1024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVertices",
        detect_size: 0x0015,
        entries: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_1024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_EndPushBuffer",
        detect_size: 0x0067,
        entries: D3D8_LTCG_D3DDEVICE_ENDPUSHBUFFER_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_GetDepthStencilSurface",
        detect_size: 0x0049,
        entries: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_PersistDisplay",
        detect_size: 0x0037,
        entries: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_RunPushBuffer",
        detect_size: 0x0098,
        entries: D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShaderConstant",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_VertexBlend",
        detect_size: 0x000F,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetShaderConstantMode",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4f",
        detect_size: 0x002C,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_1024_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShaderConstant",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_1024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DResource_GetType",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DRESOURCE_GETTYPE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "Direct3D_CreateDevice",
        detect_size: 0x0014,
        entries: D3D8_LTCG_DIRECT3D_CREATEDEVICE_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "CMiniport_InitHardware",
        detect_size: 0x000C,
        entries: D3D8_LTCG_CMINIPORT_INITHARDWARE_4_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DCubeTexture_GetCubeMapSurface2",
        detect_size: 0x004A,
        entries: D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_BeginPush",
        detect_size: 0x001F,
        entries: D3D8_LTCG_D3DDEVICE_BEGINPUSH_4_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_CreateTexture2",
        detect_size: 0x0046,
        entries: D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_DeleteStateBlock",
        detect_size: 0x000C,
        entries: D3D8_LTCG_D3DDEVICE_DELETESTATEBLOCK_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_EndVisibilityTest",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_GetDisplayMode",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_GETDISPLAYMODE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_GetRenderTarget2",
        detect_size: 0x0038,
        entries: D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_GetTransform",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_GETTRANSFORM_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_IsBusy",
        detect_size: 0x000D,
        entries: D3D8_LTCG_D3DDEVICE_ISBUSY_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_LightEnable",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_LIGHTENABLE_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetGammaRamp",
        detect_size: 0x0079,
        entries: D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_SampleAlpha",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SAMPLEALPHA_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTexture",
        detect_size: 0x002F,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURE_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BorderColor",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_1024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BumpEnv",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_1024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_Swap",
        detect_size: 0x0030,
        entries: D3D8_LTCG_D3DDEVICE_SWAP_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DPalette_Lock2",
        detect_size: 0x0008,
        entries: D3D8_LTCG_D3DPALETTE_LOCK2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DTexture_LockRect",
        detect_size: 0x000E,
        entries: D3D8_LTCG_D3DTEXTURE_LOCKRECT_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DVertexBuffer_Lock2",
        detect_size: 0x0012,
        entries: D3D8_LTCG_D3DVERTEXBUFFER_LOCK2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3D_KickOffAndWaitForIdle2",
        detect_size: 0x001F,
        entries: D3D8_LTCG_D3D_KICKOFFANDWAITFORIDLE2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_GetDepthStencilSurface2",
        detect_size: 0x0015,
        entries: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DTexture_GetSurfaceLevel2",
        detect_size: 0x0047,
        entries: D3D8_LTCG_D3DTEXTURE_GETSURFACELEVEL2_1024_ENTRIES,
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
        name: "D3DDevice_DrawIndexedVertices",
        detect_size: 0x001B,
        entries: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVerticesUP",
        detect_size: 0x0015,
        entries: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FillMode",
        detect_size: 0x0049,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FogColor",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FrontFace",
        detect_size: 0x0031,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LogicOp",
        detect_size: 0x0035,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        detect_size: 0x0025,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilEnable",
        detect_size: 0x0086,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilFail",
        detect_size: 0x006F,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        detect_size: 0x0021,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_VertexBlend",
        detect_size: 0x001A,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZEnable",
        detect_size: 0x003F,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderTarget",
        detect_size: 0x0054,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetViewport",
        detect_size: 0x002B,
        entries: D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_Swap",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_SWAP_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_MakeSpace",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_MAKESPACE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_BeginPushBuffer",
        detect_size: 0x0050,
        entries: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_End",
        detect_size: 0x0046,
        entries: D3D8_LTCG_D3DDEVICE_END_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetGammaRamp",
        detect_size: 0x0038,
        entries: D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2f",
        detect_size: 0x003F,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShader",
        detect_size: 0x0017,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3D_SetFence",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3D_SETFENCE_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_BeginVisibilityTest",
        detect_size: 0x0034,
        entries: D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShaderProgram",
        detect_size: 0x000A,
        entries: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x0048,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_1036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetIndices",
        detect_size: 0x001D,
        entries: D3D8_LTCG_D3DDEVICE_SETINDICES_1036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShader",
        detect_size: 0x0080,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetScissors",
        detect_size: 0x008E,
        entries: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2s",
        detect_size: 0x0027,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4s",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4ub",
        detect_size: 0x0028,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_UpdateOverlay",
        detect_size: 0x00D2,
        entries: D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1036_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3D_BlockOnTime",
        detect_size: 0x0119,
        entries: D3D8_LTCG_D3D_BLOCKONTIME_1036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3D_CommonSetRenderTarget",
        detect_size: 0x0049,
        entries: D3D8_LTCG_D3D_COMMONSETRENDERTARGET_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_GetRenderTarget2",
        detect_size: 0x0036,
        entries: D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShaderConstant",
        detect_size: 0x001F,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1036,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVertices",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1037_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1037,
    },
    OovpaPattern {
        name: "D3DDevice_SetStreamSource",
        detect_size: 0x0056,
        entries: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_1044_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 1044,
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
        name: "D3DDevice_Begin",
        detect_size: 0x003A,
        entries: D3D8_LTCG_D3DDEVICE_BEGIN_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShaderProgram",
        detect_size: 0x0069,
        entries: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetLight",
        detect_size: 0x0030,
        entries: D3D8_LTCG_D3DDEVICE_SETLIGHT_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FillMode",
        detect_size: 0x004C,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FogColor",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FrontFace",
        detect_size: 0x0039,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LogicOp",
        detect_size: 0x0038,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        detect_size: 0x0028,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilEnable",
        detect_size: 0x0089,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilFail",
        detect_size: 0x0072,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZEnable",
        detect_size: 0x0041,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderTarget",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_GetDepthStencilSurface2",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_PersistDisplay",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_CreateTexture2",
        detect_size: 0x004B,
        entries: D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_UpdateOverlay",
        detect_size: 0x0105,
        entries: D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1048_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 1048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_CullMode",
        detect_size: 0x0035,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1049_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1049,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_CullMode",
        detect_size: 0x0038,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1052_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1052,
    },
    OovpaPattern {
        name: "D3DDevice_SetScissors",
        detect_size: 0x0078,
        entries: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1060,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        detect_size: 0x0011,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1060,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1060,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZBias",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1060,
    },
    OovpaPattern {
        name: "D3DDevice_PersistDisplay",
        detect_size: 0x0015,
        entries: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1060,
    },
    OovpaPattern {
        name: "D3DDevice_SetScissors",
        detect_size: 0x007A,
        entries: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1072_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1072,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x001D,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1944_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1944,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x0020,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1958_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 1958,
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
        name: "D3DDevice_DeletePixelShader",
        detect_size: 0x0012,
        entries: D3D8_LTCG_D3DDEVICE_DELETEPIXELSHADER_0__LTCG_EAX1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_LightEnable",
        detect_size: 0x0027,
        entries: D3D8_LTCG_D3DDEVICE_LIGHTENABLE_4__LTCG_EAX1_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_MultiplyTransform",
        detect_size: 0x0017,
        entries: D3D8_LTCG_D3DDEVICE_MULTIPLYTRANSFORM_0__LTCG_EBX1_EAX2_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4f",
        detect_size: 0x0025,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2024_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DResource_GetType",
        detect_size: 0x0010,
        entries: D3D8_LTCG_D3DRESOURCE_GETTYPE_0__LTCG_ECX1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "Get2DSurfaceDesc",
        detect_size: 0x0032,
        entries: D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_GetViewportOffsetAndScale",
        detect_size: 0x001C,
        entries: D3D8_LTCG_D3DDEVICE_GETVIEWPORTOFFSETANDSCALE_0__LTCG_EDX1_ECX2_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x0086,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_0__LTCG_EAX1_EBX2_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetFlickerFilter",
        detect_size: 0x0025,
        entries: D3D8_LTCG_D3DDEVICE_SETFLICKERFILTER_0__LTCG_ESI1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShader",
        detect_size: 0x007C,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_Swap",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SetIndices",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_SETINDICES_4__LTCG_EBX1_2024_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3D_SetTileNoWait",
        detect_size: 0x0085,
        entries: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVertices",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_4__LTCG_ECX2_EAX3_2024_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShaderDirect",
        detect_size: 0x0026,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADERDIRECT_0__LTCG_EAX1_EBX2_2024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2024,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShader",
        detect_size: 0x0059,
        entries: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_4__LTCG_EAX1_2036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "Get2DSurfaceDesc",
        detect_size: 0x0031,
        entries: D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShader",
        detect_size: 0x0081,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4f",
        detect_size: 0x0028,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2036_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "Direct3D_CreateDevice",
        detect_size: 0x003B,
        entries: D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_ECX6_2036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "D3DDevice_SetTexture",
        detect_size: 0x0032,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX1_2036_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2036,
    },
    OovpaPattern {
        name: "D3DDevice_Swap",
        detect_size: 0x002E,
        entries: D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2036_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2036,
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
        name: "D3DDevice_SetStreamSource",
        detect_size: 0x0057,
        entries: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EAX1_2040_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2040,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x0019,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2040_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2040,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2045_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2045,
    },
    OovpaPattern {
        name: "D3DDevice_GetBackBuffer",
        detect_size: 0x004E,
        entries: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER_8__LTCG_EAX1_2048_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BorderColor",
        detect_size: 0x0045,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2048_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "Lock2DSurface",
        detect_size: 0x000B,
        entries: D3D8_LTCG_LOCK2DSURFACE_16__LTCG_ESI4_EAX5_2048_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "Lock3DSurface",
        detect_size: 0x000C,
        entries: D3D8_LTCG_LOCK3DSURFACE_16__LTCG_EAX4_2048_ENTRIES,
        argc: 4,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_BeginPushBuffer",
        detect_size: 0x004B,
        entries: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_RunPushBuffer",
        detect_size: 0x0012,
        entries: D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_4__LTCG_EAX2_2048_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_EndVisibilityTest",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_0__LTCG_EAX1_2048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_RunVertexStateShader",
        detect_size: 0x0036,
        entries: D3D8_LTCG_D3DDEVICE_RUNVERTEXSTATESHADER_4__LTCG_ESI2_2048_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderStateNotInline",
        detect_size: 0x0025,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATENOTINLINE_0__LTCG_ESI1_EDI2_2048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3DDevice_SetSoftDisplayFilter",
        detect_size: 0x001E,
        entries: D3D8_LTCG_D3DDEVICE_SETSOFTDISPLAYFILTER_0__LTCG_UNK1_2048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "D3D_SetTileNoWait",
        detect_size: 0x0038,
        entries: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2048_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2048,
    },
    OovpaPattern {
        name: "Get2DSurfaceDesc",
        detect_size: 0x0037,
        entries: D3D8_LTCG_GET2DSURFACEDESC_4__LTCG_EDI1_ESI3_2048_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2048,
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
        name: "D3DDevice_SetTextureState_BorderColor",
        detect_size: 0x0048,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2060_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 2060,
    },
    OovpaPattern {
        name: "D3DDevice_BeginPushBuffer",
        detect_size: 0x004E,
        entries: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2060_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2060,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShader",
        detect_size: 0x007A,
        entries: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2072_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 2072,
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
    OovpaPattern {
        name: "CDevice_FreeFrameBuffers",
        detect_size: 0x0075,
        entries: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_4034_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4034,
    },
    OovpaPattern {
        name: "D3DDevice_SetTransform",
        detect_size: 0x0104,
        entries: D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_4034_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4034,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexDataColor",
        detect_size: 0x0014,
        entries: D3D8_LTCG_D3DDEVICE_SETVERTEXDATACOLOR_4038_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4038,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x0050,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_ESI1_4039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDevice_KickOff",
        detect_size: 0x008E,
        entries: D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDevice_SetStateUP",
        detect_size: 0x002A,
        entries: D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDevice_SetStateVB",
        detect_size: 0x0015,
        entries: D3D8_LTCG_CDEVICE_SETSTATEVB_8_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_CreateCubeTexture",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4__LTCG_EAX1_EDX3_ECX4_EDI6_4039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_CreateTexture",
        detect_size: 0x001A,
        entries: D3D8_LTCG_D3DDEVICE_CREATETEXTURE_8__LTCG_EDX3_ECX4_EAX5_EDI7_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_CreateVolumeTexture",
        detect_size: 0x001D,
        entries: D3D8_LTCG_D3DDEVICE_CREATEVOLUMETEXTURE_12__LTCG_EDX4_ECX5_EAX6_EDI8_4039_ENTRIES,
        argc: 3,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVertices",
        detect_size: 0x0018,
        entries: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_8__LTCG_EAX3_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x003A,
        entries: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x0094,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderStateInline__GenericFragment",
        detect_size: 0x001A,
        entries: D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline",
        detect_size: 0x002F,
        entries:
            D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline2",
        detect_size: 0x002F,
        entries:
            D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE2_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        detect_size: 0x0042,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3DDevice_SetTile",
        detect_size: 0x0034,
        entries: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EBX1_EAX2_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3D_BlockOnResource",
        detect_size: 0x0029,
        entries: D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_ECX1_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3D_CDevice_SetTextureStageStateNotInline",
        detect_size: 0x0031,
        entries:
            D3D8_LTCG_D3D_CDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "D3D_CreateTexture",
        detect_size: 0x004A,
        entries: D3D8_LTCG_D3D_CREATETEXTURE_28__LTCG_DL8_EDI9_4039_ENTRIES,
        argc: 7,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDevice_SetStateUP",
        detect_size: 0x0035,
        entries: D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4040_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4040,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x003B,
        entries: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4040_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4040,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x003C,
        entries: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4041_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4041,
    },
    OovpaPattern {
        name: "CDevice_FreeFrameBuffers",
        detect_size: 0x007B,
        entries: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4432_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x0178,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4432_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "CDevice_KickOff",
        detect_size: 0x00C3,
        entries: D3D8_LTCG_CDEVICE_KICKOFF_4_4432_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "D3DDevice_CreateCubeTexture",
        detect_size: 0x0028,
        entries: D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4432_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x003E,
        entries: D3D8_LTCG_D3DDEVICE_RESET_4432_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline",
        detect_size: 0x002B,
        entries:
            D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EAX2_EDX3_4432_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        detect_size: 0x0041,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4432_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "D3DDevice_SetTile",
        detect_size: 0x0037,
        entries: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_ECX1_EAX2_4432_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "CDevice_FreeFrameBuffers",
        detect_size: 0x007A,
        entries: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4433_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4433,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x0177,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4433_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4433,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x0173,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4531_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "CDevice_KickOff",
        detect_size: 0x00C5,
        entries: D3D8_LTCG_CDEVICE_KICKOFF_4_4531_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x0097,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4531_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline",
        detect_size: 0x002B,
        entries:
            D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ESI3_4531_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        detect_size: 0x0045,
        entries: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4531_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "D3DDevice_GetBackBuffer2",
        detect_size: 0x001D,
        entries: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_4626_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4626,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x0032,
        entries: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_4627_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "D3DDevice_CopyRects",
        detect_size: 0x0016,
        entries: D3D8_LTCG_D3DDEVICE_COPYRECTS_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "D3DDevice_GetBackBuffer2",
        detect_size: 0x0049,
        entries: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_0__LTCG_EAX1_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x003B,
        entries: D3D8_LTCG_D3DDEVICE_RESET_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x008D,
        entries: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4627_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "D3DDevice_CopyRects",
        detect_size: 0x0009,
        entries: D3D8_LTCG_D3DDEVICE_COPYRECTS_4628_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4628,
    },
    OovpaPattern {
        name: "D3D_SetTileNoWait",
        detect_size: 0x0039,
        entries: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_ECX1_EAX2_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
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
        name: "D3DDevice_BeginVisibilityTest",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVerticesUP",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWVERTICESUP_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_IsFencePending",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_ISFENCEPENDING_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetIndices",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETINDICES_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleMode",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMODE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2f",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2s",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4s",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4ub",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShader",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShaderInput",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERINPUT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DSurface_GetDesc",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DSURFACE_GETDESC_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetFence",
        min_version: 1024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3D_SETFENCE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DCubeTexture_GetCubeMapSurface",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Begin",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGIN_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Clear",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CLEAR_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateTexture",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATETEXTURE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVertices",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndPushBuffer",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_ENDPUSHBUFFER_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDepthStencilSurface",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_PersistDisplay",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_RunPushBuffer",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShaderConstant",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_VertexBlend",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetShaderConstantMode",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSHADERCONSTANTMODE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4f",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShaderConstant",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADERCONSTANT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_GetType",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DRESOURCE_GETTYPE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "Direct3D_CreateDevice",
        min_version: 1024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_DIRECT3D_CREATEDEVICE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "CMiniport_InitHardware",
        min_version: 1024,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_CMINIPORT_INITHARDWARE_4_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DCubeTexture_GetCubeMapSurface2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DCUBETEXTURE_GETCUBEMAPSURFACE2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginPush",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINPUSH_4_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateTexture2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeleteStateBlock",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DELETESTATEBLOCK_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndVisibilityTest",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDisplayMode",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETDISPLAYMODE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetRenderTarget2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetTransform",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETTRANSFORM_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_IsBusy",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_ISBUSY_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LightEnable",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LIGHTENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetGammaRamp",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_SampleAlpha",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SAMPLEALPHA_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTexture",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BorderColor",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BumpEnv",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BUMPENV_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Swap",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SWAP_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DPalette_Lock2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DPALETTE_LOCK2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DTexture_LockRect",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DTEXTURE_LOCKRECT_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DVertexBuffer_Lock2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DVERTEXBUFFER_LOCK2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_KickOffAndWaitForIdle2",
        min_version: 1024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3D_KICKOFFANDWAITFORIDLE2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDepthStencilSurface2",
        min_version: 1024,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DTexture_GetSurfaceLevel2",
        min_version: 1024,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DTEXTURE_GETSURFACELEVEL2_1024_XREFS,
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
        name: "D3DDevice_DrawIndexedVertices",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVerticesUP",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICESUP_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FillMode",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FogColor",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FrontFace",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LogicOp",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilEnable",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilFail",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_VertexBlend",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZEnable",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderTarget",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetViewport",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVIEWPORT_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Swap",
        min_version: 1036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SWAP_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_MakeSpace",
        min_version: 1036,
        source_file: "D3D8LTCG/4134.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_MAKESPACE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginPushBuffer",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_End",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_END_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetGammaRamp",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETGAMMARAMP_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2f",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2F_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShader",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXSHADER_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetFence",
        min_version: 1036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3D_SETFENCE_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginVisibilityTest",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINVISIBILITYTEST_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShaderProgram",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetIndices",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETINDICES_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetScissors",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2s",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA2S_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4s",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4S_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4ub",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4UB_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_UpdateOverlay",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_BlockOnTime",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3D_BLOCKONTIME_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CommonSetRenderTarget",
        min_version: 1036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3D_COMMONSETRENDERTARGET_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetRenderTarget2",
        min_version: 1036,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETRENDERTARGET2_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShaderConstant",
        min_version: 1036,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADERCONSTANT_1036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVertices",
        min_version: 1037,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWINDEXEDVERTICES_1037_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetStreamSource",
        min_version: 1044,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_1044_XREFS,
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
        name: "D3DDevice_Begin",
        min_version: 1048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGIN_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShaderProgram",
        min_version: 1048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADERPROGRAM_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetLight",
        min_version: 1048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETLIGHT_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FillMode",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FILLMODE_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FogColor",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FrontFace",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_FRONTFACE_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LogicOp",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_LOGICOP_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilEnable",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilFail",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZEnable",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZENABLE_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderTarget",
        min_version: 1048,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERTARGET_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDepthStencilSurface2",
        min_version: 1048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETDEPTHSTENCILSURFACE2_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_PersistDisplay",
        min_version: 1048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateTexture2",
        min_version: 1048,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATETEXTURE2_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_UpdateOverlay",
        min_version: 1048,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_UPDATEOVERLAY_1048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_CullMode",
        min_version: 1049,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1049_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_CullMode",
        min_version: 1052,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_CULLMODE_1052_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetScissors",
        min_version: 1060,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        min_version: 1060,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_1060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        min_version: 1060,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_1060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZBias",
        min_version: 1060,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATE_ZBIAS_1060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_PersistDisplay",
        min_version: 1060,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_PERSISTDISPLAY_1060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetScissors",
        min_version: 1072,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSCISSORS_1072_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 1944,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1944_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 1958,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_1958_XREFS,
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
        name: "D3DDevice_DeletePixelShader",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DELETEPIXELSHADER_0__LTCG_EAX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LightEnable",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LIGHTENABLE_4__LTCG_EAX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_MultiplyTransform",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_MULTIPLYTRANSFORM_0__LTCG_EBX1_EAX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4f",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_GetType",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DRESOURCE_GETTYPE_0__LTCG_ECX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "Get2DSurfaceDesc",
        min_version: 2024,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetViewportOffsetAndScale",
        min_version: 2024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETVIEWPORTOFFSETANDSCALE_0__LTCG_EDX1_ECX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 2024,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_0__LTCG_EAX1_EBX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetFlickerFilter",
        min_version: 2024,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETFLICKERFILTER_0__LTCG_ESI1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 2024,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Swap",
        min_version: 2024,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetIndices",
        min_version: 2024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETINDICES_4__LTCG_EBX1_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetTileNoWait",
        min_version: 2024,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVertices",
        min_version: 2024,
        source_file: "D3D8LTCG/4721.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_4__LTCG_ECX2_EAX3_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShaderDirect",
        min_version: 2024,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADERDIRECT_0__LTCG_EAX1_EBX2_2024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShader",
        min_version: 2036,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_LOADVERTEXSHADER_4__LTCG_EAX1_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "Get2DSurfaceDesc",
        min_version: 2036,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_GET2DSURFACEDESC_0__LTCG_EDI1_EBX2_ESI3_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 2036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4f",
        min_version: 2036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATA4F_16__LTCG_EDI1_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "Direct3D_CreateDevice",
        min_version: 2036,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_DIRECT3D_CREATEDEVICE_16__LTCG_EAX4_ECX6_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTexture",
        min_version: 2036,
        source_file: "D3D8LTCG/4721.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURE_4__LTCG_EAX1_2036_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Swap",
        min_version: 2036,
        source_file: "D3D8LTCG/4721.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SWAP_0__LTCG_EAX1_2036_XREFS,
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
        name: "D3DDevice_SetStreamSource",
        min_version: 2040,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSTREAMSOURCE_8__LTCG_EAX1_2040_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 2040,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2040_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 2045,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_4__LTCG_ESI1_2045_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetBackBuffer",
        min_version: 2048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER_8__LTCG_EAX1_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BorderColor",
        min_version: 2048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "Lock2DSurface",
        min_version: 2048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_LOCK2DSURFACE_16__LTCG_ESI4_EAX5_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "Lock3DSurface",
        min_version: 2048,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_LOCK3DSURFACE_16__LTCG_EAX4_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginPushBuffer",
        min_version: 2048,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_RunPushBuffer",
        min_version: 2048,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RUNPUSHBUFFER_4__LTCG_EAX2_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndVisibilityTest",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_ENDVISIBILITYTEST_0__LTCG_EAX1_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_RunVertexStateShader",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RUNVERTEXSTATESHADER_4__LTCG_ESI2_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderStateNotInline",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATENOTINLINE_0__LTCG_ESI1_EDI2_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetSoftDisplayFilter",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETSOFTDISPLAYFILTER_0__LTCG_UNK1_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetTileNoWait",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_EAX1_ECX2_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "Get2DSurfaceDesc",
        min_version: 2048,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_GET2DSURFACEDESC_4__LTCG_EDI1_ESI3_2048_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 2060,
        source_file: "D3D8LTCG/3911.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BorderColor",
        min_version: 2060,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_4__LTCG_EAX1_2060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginPushBuffer",
        min_version: 2060,
        source_file: "D3D8LTCG/4928.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_BEGINPUSHBUFFER_0__LTCG_EDI1_2060_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShader",
        min_version: 2072,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETPIXELSHADER_0__LTCG_EAX1_2072_XREFS,
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
    OovpaPatternMeta {
        name: "CDevice_FreeFrameBuffers",
        min_version: 4034,
        source_file: "D3D8LTCG/4034.inl",
        xrefs: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_0__LTCG_EBX1_4034_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTransform",
        min_version: 4034,
        source_file: "D3D8LTCG/4034.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTRANSFORM_0__LTCG_EAX1_EDX2_4034_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexDataColor",
        min_version: 4038,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETVERTEXDATACOLOR_4038_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_ESI1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_KickOff",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_CDEVICE_KICKOFF_0__LTCG_EDX1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateUP",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateVB",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_CDEVICE_SETSTATEVB_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateCubeTexture",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4__LTCG_EAX1_EDX3_ECX4_EDI6_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateTexture",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATETEXTURE_8__LTCG_EDX3_ECX4_EAX5_EDI7_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateVolumeTexture",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATEVOLUMETEXTURE_12__LTCG_EDX4_ECX5_EAX6_EDI8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVertices",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_DRAWVERTICES_8__LTCG_EAX3_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderStateInline__GenericFragment",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETRENDERSTATEINLINE__GENERICFRAGMENT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline2",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE2_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTile",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_EBX1_EAX2_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_BlockOnResource",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3D_BLOCKONRESOURCE_0__LTCG_ECX1_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CDevice_SetTextureStageStateNotInline",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs:
            D3D8_LTCG_D3D_CDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ECX3_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CreateTexture",
        min_version: 4039,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3D_CREATETEXTURE_28__LTCG_DL8_EDI9_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateUP",
        min_version: 4040,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_CDEVICE_SETSTATEUP_0__LTCG_ESI1_4040_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 4040,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4040_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 4041,
        source_file: "D3D8LTCG/4039.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_0__LTCG_EDI1_4041_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_FreeFrameBuffers",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_KickOff",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_CDEVICE_KICKOFF_4_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateCubeTexture",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_CREATECUBETEXTURE_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_ECX1_EAX2_EDX3_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTile",
        min_version: 4432,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTILE_0__LTCG_ECX1_EAX2_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_FreeFrameBuffers",
        min_version: 4433,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_CDEVICE_FREEFRAMEBUFFERS_4_4433_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 4433,
        source_file: "D3D8LTCG/4432.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4433_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 4531,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_4__LTCG_EBX1_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_KickOff",
        min_version: 4531,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_CDEVICE_KICKOFF_4_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 4531,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline",
        min_version: 4531,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_0__LTCG_EAX1_EDX2_ESI3_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        min_version: 4531,
        source_file: "D3D8LTCG/4531.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_4__LTCG_EAX1_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetBackBuffer2",
        min_version: 4626,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_4626_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 4627,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_CDEVICE_INITIALIZEFRAMEBUFFERS_8_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CopyRects",
        min_version: 4627,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_COPYRECTS_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetBackBuffer2",
        min_version: 4627,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_GETBACKBUFFER2_0__LTCG_EAX1_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 4627,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_RESET_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 4627,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_SELECTVERTEXSHADER_4__LTCG_EAX1_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CopyRects",
        min_version: 4628,
        source_file: "D3D8LTCG/4627.inl",
        xrefs: D3D8_LTCG_D3DDEVICE_COPYRECTS_4628_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetTileNoWait",
        min_version: 4721,
        source_file: "D3D8LTCG/4721.inl",
        xrefs: D3D8_LTCG_D3D_SETTILENOWAIT_0__LTCG_ECX1_EAX2_4721_XREFS,
    },
];
