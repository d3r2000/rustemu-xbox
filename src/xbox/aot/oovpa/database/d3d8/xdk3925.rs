// Auto-generated from Cxbx-R XbSymbolDatabase.
// Regenerate with: python3 tools/import_xbsymdb_database.py --xdk 3925
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

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x56),
    ov!(0x000B, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_OcclusionCullEnable",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x56),
    ov!(0x000B, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_StencilCullEnable",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_1024_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x000B, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_1024_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DRS_YuvEnable",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3D_UPDATEPROJECTIONVIEWPORTTRANSFORM_3900_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xE4),
    ov!(0x0005, 0xF0),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xEC),
    ov!(0x0008, 0x58),
    ov!(0x000A, 0x8B),
    ov!(0x0010, 0x8B),
    ov!(0x0016, 0xDB),
    ov!(0x001F, 0x7D),
    ov!(0x0020, 0x06),
];
static D3D8_D3D_UPDATEPROJECTIONVIEWPORTTRANSFORM_3900_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_FREEFRAMEBUFFERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x55),
    ov!(0x0002, 0x56),
    ov!(0x0003, 0x57),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0xF1),
    ov!(0x0006, 0xFF),
    ov!(0x0007, 0x15),
    ov!(0x001E, 0xFF),
    ov!(0x001F, 0x15),
    ov!(0x006E, 0x52),
    ov!(0x006F, 0xE8),
    ov!(0x0074, 0x89),
    ov!(0x0075, 0xAE),
    ov!(0x007A, 0x39),
    ov!(0x007B, 0xAE),
];
static D3D8_CDEVICE_FREEFRAMEBUFFERS_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0008,
        target: "KT_FUNC_AvGetSavedDataAddress",
    },
    OovpaXref {
        offset: 0x0020,
        target: "KT_FUNC_AvSendTVEncoderOption",
    },
];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_INITIALIZEFRAMEBUFFERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x001B, 0x72),
    ov!(0x001C, 0x04),
    ov!(0x001D, 0x89),
    ov!(0x001E, 0x44),
    ov!(0x001F, 0x24),
    ov!(0x0024, 0x50),
    ov!(0x0025, 0xE8),
    ov!(0x002A, 0x8B),
    ov!(0x002C, 0x24),
];
static D3D8_CDEVICE_INITIALIZEFRAMEBUFFERS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_KICKOFF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0xA1),
    ov!(0x000E, 0xF6),
    ov!(0x000F, 0x41),
    ov!(0x0010, 0x0C),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x74),
    ov!(0x0013, 0x08),
    ov!(0x001E, 0xA1),
    ov!(0x0069, 0xC3),
];
static D3D8_CDEVICE_KICKOFF_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_MAKESPACE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0006, 0xF6),
    ov!(0x0007, 0x46),
    ov!(0x0008, 0x0C),
    ov!(0x0009, 0x04),
    ov!(0x001E, 0x2B),
    ov!(0x001F, 0xCF),
    ov!(0x0020, 0x03),
    ov!(0x0021, 0xD1),
    ov!(0x002F, 0x83),
    ov!(0x0030, 0xC4),
    ov!(0x0032, 0xC3),
];
static D3D8_CDEVICE_MAKESPACE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_SETSTATEUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x000A, 0xE8),
    ov!(0x001B, 0x25),
    ov!(0x001C, 0xFF),
    ov!(0x001D, 0xFE),
    ov!(0x001E, 0xFF),
    ov!(0x001F, 0xFF),
    ov!(0x0021, 0x0D),
    ov!(0x0022, 0x80),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x00),
];
static D3D8_CDEVICE_SETSTATEUP_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CDEVICE_SETSTATEVB_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0008, 0xE8),
    ov!(0x0010, 0xA9),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x40),
    ov!(0x002C, 0x25),
    ov!(0x002D, 0x7F),
    ov!(0x002E, 0xFF),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0xBF),
];
static D3D8_CDEVICE_SETSTATEVB_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CMINIPORT_CREATECTXDMAOBJECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x51),
    ov!(0x0004, 0x51),
    ov!(0x0005, 0x53),
    ov!(0x0006, 0x56),
    ov!(0x0007, 0x57),
    ov!(0x0008, 0x33),
    ov!(0x0009, 0xF6),
    ov!(0x000A, 0x56),
    ov!(0x000B, 0x8D),
    ov!(0x000C, 0x45),
    ov!(0x000D, 0xFC),
    ov!(0x000E, 0x50),
    ov!(0x000F, 0x8D),
    ov!(0x0010, 0x45),
    ov!(0x0011, 0xF8),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x75),
    ov!(0x0015, 0x10),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0xD1),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x3A),
    ov!(0x001A, 0x89),
    ov!(0x001B, 0x75),
    ov!(0x001C, 0xF8),
    ov!(0x001D, 0x89),
    ov!(0x001E, 0x75),
    ov!(0x001F, 0xFC),
];
static D3D8_CMINIPORT_CREATECTXDMAOBJECT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CMINIPORT_GETDISPLAYCAPABILITIES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x33),
    ov!(0x0001, 0xC0),
    ov!(0x0002, 0x39),
    ov!(0x0003, 0x05),
    ov!(0x0008, 0x75),
    ov!(0x0009, 0x0F),
    ov!(0x000A, 0x68),
    ov!(0x000F, 0x50),
    ov!(0x0010, 0x6A),
    ov!(0x0011, 0x06),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x15),
    ov!(0x0019, 0xA1),
    ov!(0x001E, 0xC3),
];
static D3D8_CMINIPORT_GETDISPLAYCAPABILITIES_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CMINIPORT_INITHARDWARE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x0C),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x56),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xF1),
    ov!(0x000A, 0x56),
    ov!(0x000B, 0x68),
    ov!(0x0010, 0x8D),
    ov!(0x0011, 0x86),
    ov!(0x0012, 0x88),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x50),
    ov!(0x0017, 0xFF),
    ov!(0x0018, 0x15),
    ov!(0x001D, 0x80),
    ov!(0x001E, 0xA6),
    ov!(0x001F, 0xF8),
];
static D3D8_CMINIPORT_INITHARDWARE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_CMINIPORT_ISFLIPPENDING_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x81),
    ov!(0x0002, 0xDC),
    ov!(0x0003, 0x01),
    ov!(0x0004, 0x00),
    ov!(0x0005, 0x00),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x84),
    ov!(0x0008, 0xC1),
    ov!(0x0009, 0xB4),
    ov!(0x000A, 0x01),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0xC3),
];
static D3D8_CMINIPORT_ISFLIPPENDING_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DBASETEXTURE_GETLEVELCOUNT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x0F),
    ov!(0x0005, 0xB6),
    ov!(0x0006, 0x40),
    ov!(0x0007, 0x0E),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xE0),
    ov!(0x000A, 0x0F),
    ov!(0x000B, 0xC2),
    ov!(0x000C, 0x04),
];
static D3D8_D3DBASETEXTURE_GETLEVELCOUNT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DCUBETEXTURE_GETCUBEMAPSURFACE_3911_ENTRIES: &[OovpaEntry] = &[
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
    ov!(0x0046, 0xE8),
    ov!(0x004C, 0x83),
    ov!(0x004D, 0xC4),
    ov!(0x004E, 0x08),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x10),
];
static D3D8_D3DCUBETEXTURE_GETCUBEMAPSURFACE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DCUBETEXTURE_LOCKRECT_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static D3D8_D3DCUBETEXTURE_LOCKRECT_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "D3D8_Lock2DSurface",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x04),
    ov!(0x000B, 0x41),
    ov!(0x000C, 0x89),
    ov!(0x000D, 0x88),
    ov!(0x000E, 0x08),
    ov!(0x000F, 0x04),
];
static D3D8_D3DDEVICE_ADDREF_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_APPLYSTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x001D, 0x83),
    ov!(0x0040, 0x83),
    ov!(0x005B, 0x16),
    ov!(0x007A, 0x01),
    ov!(0x0099, 0x46),
    ov!(0x00B8, 0x06),
    ov!(0x00D7, 0x39),
    ov!(0x00F6, 0x51),
];
static D3D8_D3DDEVICE_APPLYSTATEBLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BEGIN_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xC7),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0xFC),
    ov!(0x0023, 0x17),
    ov!(0x0024, 0x04),
    ov!(0x0025, 0x00),
    ov!(0x002E, 0x0D),
    ov!(0x002F, 0x00),
    ov!(0x0030, 0x08),
    ov!(0x0038, 0xC2),
    ov!(0x0039, 0x04),
];
static D3D8_D3DDEVICE_BEGIN_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BEGINPUSHBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0xCE),
    ov!(0x001B, 0x57),
    ov!(0x0025, 0x00),
    ov!(0x0032, 0x06),
    ov!(0x003F, 0x03),
    ov!(0x004C, 0x04),
    ov!(0x0059, 0x04),
];
static D3D8_D3DDEVICE_BEGINPUSHBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BEGINSTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0x48),
    ov!(0x0007, 0x0C),
    ov!(0x0008, 0x20),
    ov!(0x0009, 0xE9),
];
static D3D8_D3DDEVICE_BEGINSTATEBLOCK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "D3D_ClearStateBlockFlags",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BEGINVISIBILITYTEST_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x000D, 0xB9),
    ov!(0x000E, 0x01),
    ov!(0x000F, 0x00),
    ov!(0x0012, 0xC7),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0xC8),
    ov!(0x0015, 0x17),
    ov!(0x0016, 0x08),
    ov!(0x0017, 0x00),
    ov!(0x001E, 0x83),
    ov!(0x001F, 0xC0),
    ov!(0x0020, 0x0C),
    ov!(0x0024, 0xC3),
];
static D3D8_D3DDEVICE_BEGINVISIBILITYTEST_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BLOCKONFENCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0002, 0x24),
    ov!(0x0004, 0x6A),
    ov!(0x0006, 0x50),
    ov!(0x000C, 0xC2),
    ov!(0x000D, 0x04),
    ov!(0x000E, 0x00),
];
static D3D8_D3DDEVICE_BLOCKONFENCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "D3D_BlockOnTime",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_BLOCKUNTILVERTICALBLANK_3911_ENTRIES: &[OovpaEntry] = &[
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
    ov!(0x0023, 0xC3),
];
static D3D8_D3DDEVICE_BLOCKUNTILVERTICALBLANK_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3DDevice__m_VerticalBlankEvent_OFFSET",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CAPTURESTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x001E, 0x76),
    ov!(0x003E, 0xE8),
    ov!(0x005E, 0x06),
    ov!(0x007E, 0x26),
    ov!(0x009E, 0xFF),
    ov!(0x00BE, 0x04),
    ov!(0x00DE, 0xF8),
];
static D3D8_D3DDEVICE_CAPTURESTATEBLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CLEAR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x38),
    ov!(0x0025, 0xF6),
    ov!(0x0026, 0xC1),
    ov!(0x0027, 0x01),
    ov!(0x005D, 0x89),
    ov!(0x005E, 0x06),
    ov!(0x005F, 0x33),
    ov!(0x0060, 0xED),
];
static D3D8_D3DDEVICE_CLEAR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_COPYRECTS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x81),
    ov!(0x0001, 0xEC),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xAC),
    ov!(0x000A, 0x24),
    ov!(0x0010, 0x0F),
    ov!(0x0011, 0xB6),
    ov!(0x0012, 0x75),
    ov!(0x0013, 0x0D),
    ov!(0x0014, 0x8A),
    ov!(0x0015, 0x9E),
];
static D3D8_D3DDEVICE_COPYRECTS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATECUBETEXTURE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x18),
    ov!(0x000C, 0x50),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x6A),
    ov!(0x0014, 0x01),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x44),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x1C),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x18),
];
static D3D8_D3DDEVICE_CREATECUBETEXTURE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEIMAGESURFACE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x24),
    ov!(0x000E, 0x44),
    ov!(0x0012, 0x00),
    ov!(0x0016, 0xE8),
    ov!(0x001B, 0xC2),
];
static D3D8_D3DDEVICE_CREATEIMAGESURFACE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEINDEXBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0013, 0xB8),
    ov!(0x0014, 0x0E),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x07),
    ov!(0x0017, 0x80),
    ov!(0x0018, 0xC2),
    ov!(0x0019, 0x14),
    ov!(0x002E, 0xC7),
    ov!(0x002F, 0x00),
    ov!(0x0030, 0x01),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x01),
    ov!(0x0033, 0x01),
];
static D3D8_D3DDEVICE_CREATEINDEXBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEPALETTE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x6A),
    ov!(0x0002, 0x0C),
    ov!(0x0003, 0x6A),
    ov!(0x0004, 0x40),
    ov!(0x0010, 0xB8),
    ov!(0x0011, 0x0E),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x07),
    ov!(0x0014, 0x80),
    ov!(0x004E, 0xC1),
    ov!(0x004F, 0xE6),
    ov!(0x0050, 0x1E),
];
static D3D8_D3DDEVICE_CREATEPALETTE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEPIXELSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x68),
    ov!(0x0001, 0xFC),
    ov!(0x0010, 0xB8),
    ov!(0x0011, 0x0E),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x07),
    ov!(0x0014, 0x80),
    ov!(0x0031, 0xB9),
    ov!(0x0032, 0x3C),
    ov!(0x0042, 0xC2),
    ov!(0x0043, 0x08),
];
static D3D8_D3DDEVICE_CREATEPIXELSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATESTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x001E, 0x8B),
    ov!(0x003E, 0x89),
    ov!(0x005E, 0x24),
    ov!(0x007E, 0xF8),
    ov!(0x009F, 0x01),
    ov!(0x00BE, 0xB6),
    ov!(0x00DE, 0xF8),
    ov!(0x00FE, 0x76),
];
static D3D8_D3DDEVICE_CREATESTATEBLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATETEXTURE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x1C),
    ov!(0x000C, 0x50),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x6A),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x51),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x52),
    ov!(0x0029, 0xC2),
    ov!(0x002A, 0x1C),
];
static D3D8_D3DDEVICE_CREATETEXTURE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEVERTEXBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0003, 0x6A),
    ov!(0x0004, 0x40),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x44),
    ov!(0x0012, 0x24),
    ov!(0x0013, 0x08),
    ov!(0x0048, 0xC7),
    ov!(0x0049, 0x06),
    ov!(0x004A, 0x01),
    ov!(0x004B, 0x00),
    ov!(0x004D, 0x01),
    ov!(0x0053, 0xC2),
    ov!(0x0054, 0x14),
];
static D3D8_D3DDEVICE_CREATEVERTEXBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x53),
    ov!(0x0002, 0x55),
    ov!(0x0003, 0x8B),
    ov!(0x0004, 0x6C),
    ov!(0x0005, 0x24),
    ov!(0x0006, 0x14),
    ov!(0x0007, 0x85),
    ov!(0x0008, 0xED),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0x74),
    ov!(0x000B, 0x10),
    ov!(0x000C, 0x0F),
    ov!(0x000D, 0xB7),
    ov!(0x000E, 0x45),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x0F),
    ov!(0x0011, 0xB7),
    ov!(0x0012, 0x4D),
    ov!(0x0013, 0x02),
    ov!(0x0014, 0x89),
    ov!(0x0015, 0x44),
    ov!(0x0016, 0x24),
    ov!(0x0017, 0x0C),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x4C),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x18),
    ov!(0x001C, 0x33),
    ov!(0x001D, 0xF6),
    ov!(0x001E, 0x85),
    ov!(0x001F, 0xED),
    ov!(0x003E, 0xE8),
    ov!(0x0043, 0xC1),
    ov!(0x0044, 0xE0),
    ov!(0x0045, 0x02),
    ov!(0x0046, 0x8D),
    ov!(0x0047, 0x1C),
    ov!(0x0048, 0x30),
    ov!(0x005E, 0x75),
    ov!(0x0065, 0x07),
    ov!(0x0066, 0x80),
    ov!(0x0069, 0xC2),
    ov!(0x006A, 0x10),
    ov!(0x007E, 0x04),
    ov!(0x009E, 0x24),
    ov!(0x00BE, 0x24),
    ov!(0x00DE, 0x83),
    ov!(0x00FE, 0xC7),
];
static D3D8_D3DDEVICE_CREATEVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_CREATEVOLUMETEXTURE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x20),
    ov!(0x000C, 0x50),
    ov!(0x0011, 0x6A),
    ov!(0x0012, 0x01),
    ov!(0x0013, 0x6A),
    ov!(0x0014, 0x00),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x4C),
    ov!(0x0018, 0x24),
    ov!(0x0019, 0x1C),
    ov!(0x002C, 0xC2),
    ov!(0x002D, 0x20),
];
static D3D8_D3DDEVICE_CREATEVOLUMETEXTURE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DELETEPATCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0005, 0x0D),
    ov!(0x000C, 0xA1),
    ov!(0x0011, 0x56),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x34),
    ov!(0x0014, 0x88),
    ov!(0x0015, 0x85),
    ov!(0x0016, 0xF6),
    ov!(0x0017, 0x74),
    ov!(0x0018, 0x17),
    ov!(0x0019, 0x8B),
    ov!(0x001A, 0x4E),
    ov!(0x001B, 0x04),
    ov!(0x001C, 0xC7),
    ov!(0x001D, 0x04),
    ov!(0x001E, 0x88),
    ov!(0x001F, 0x00),
    ov!(0x0025, 0xE8),
];
static D3D8_D3DDEVICE_DELETEPATCH_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DELETEPIXELSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x75),
    ov!(0x000A, 0x04),
    ov!(0x000E, 0x09),
    ov!(0x0012, 0x04),
    ov!(0x0018, 0xC2),
    ov!(0x001A, 0x00),
];
static D3D8_D3DDEVICE_DELETEPIXELSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DELETESTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0011, 0x76),
    ov!(0x0024, 0x3B),
    ov!(0x0037, 0xE8),
    ov!(0x004A, 0x50),
    ov!(0x005D, 0x74),
    ov!(0x0070, 0x06),
    ov!(0x0083, 0xEB),
];
static D3D8_D3DDEVICE_DELETESTATEBLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DELETEVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0xFF),
    ov!(0x0009, 0x89),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x75),
    ov!(0x000C, 0x09),
    ov!(0x0016, 0xC2),
    ov!(0x0017, 0x04),
];
static D3D8_D3DDEVICE_DELETEVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DRAWINDEXEDVERTICES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x08),
    ov!(0x0008, 0x8B),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x75),
    ov!(0x001A, 0xF8),
    ov!(0x001B, 0xE8),
];
static D3D8_D3DDEVICE_DRAWINDEXEDVERTICES_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000A,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x001C,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DRAWINDEXEDVERTICESUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x3D),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xCF),
    ov!(0x0011, 0x89),
    ov!(0x0013, 0xF8),
    ov!(0x0014, 0xE8),
];
static D3D8_D3DDEVICE_DRAWINDEXEDVERTICESUP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0015,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DRAWVERTICES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x1D),
    ov!(0x000D, 0xE8),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x7C),
    ov!(0x0014, 0x24),
    ov!(0x0015, 0x18),
    ov!(0x0016, 0x8D),
    ov!(0x0017, 0x77),
    ov!(0x0018, 0xFF),
    ov!(0x0019, 0xC1),
    ov!(0x001A, 0xEE),
    ov!(0x001B, 0x08),
];
static D3D8_D3DDEVICE_DRAWVERTICES_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000E,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_DRAWVERTICESUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x10),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x3D),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xCF),
    ov!(0x0011, 0x89),
    ov!(0x0013, 0xFC),
    ov!(0x0014, 0xE8),
];
static D3D8_D3DDEVICE_DRAWVERTICESUP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0015,
        target: "D3D_CDevice_SetStateUP",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ENABLEOVERLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x80),
    ov!(0x000B, 0x04),
    ov!(0x000C, 0x04),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x33),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x3B),
    ov!(0x0012, 0xD1),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x0A),
    ov!(0x0015, 0x39),
    ov!(0x0016, 0x88),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x87),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x75),
    ov!(0x001C, 0xF8),
    ov!(0x001D, 0xEB),
    ov!(0x001E, 0x0A),
    ov!(0x001F, 0xC7),
];
static D3D8_D3DDEVICE_ENABLEOVERLAY_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_END_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0017, 0xC7),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0xFC),
    ov!(0x001A, 0x17),
    ov!(0x001B, 0x04),
    ov!(0x001C, 0x00),
    ov!(0x002C, 0xF6),
    ov!(0x002D, 0xC4),
    ov!(0x002E, 0x10),
    ov!(0x002F, 0x74),
    ov!(0x0030, 0x07),
    ov!(0x0040, 0xC3),
];
static D3D8_D3DDEVICE_END_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ENDPUSHBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x8B),
    ov!(0x000C, 0xE8),
    ov!(0x0044, 0x81),
    ov!(0x0045, 0x66),
    ov!(0x0047, 0x7B),
    ov!(0x0048, 0xFF),
    ov!(0x0049, 0xFF),
    ov!(0x004A, 0xFF),
    ov!(0x005D, 0xF7),
    ov!(0x005E, 0xC1),
    ov!(0x005F, 0xFF),
    ov!(0x0060, 0xFF),
    ov!(0x0061, 0x78),
    ov!(0x0062, 0x00),
];
static D3D8_D3DDEVICE_ENDPUSHBUFFER_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0004,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ENDSTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0x60),
    ov!(0x0007, 0x0C),
    ov!(0x0008, 0xDF),
];
static D3D8_D3DDEVICE_ENDSTATEBLOCK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "D3D_RecordStateBlock",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ENDVISIBILITYTEST_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0006, 0xE8),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xF0),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xF6),
    ov!(0x000F, 0x75),
    ov!(0x0010, 0x09),
    ov!(0x0011, 0xB8),
    ov!(0x0012, 0x0E),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x07),
    ov!(0x0015, 0x80),
    ov!(0x0016, 0x5E),
    ov!(0x0017, 0xC2),
    ov!(0x0018, 0x04),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x57),
    ov!(0x001B, 0x8B),
    ov!(0x001C, 0x3D),
    ov!(0x0037, 0x00),
];
static D3D8_D3DDEVICE_ENDVISIBILITYTEST_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_FLUSHVERTEXCACHE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x56),
    ov!(0x0008, 0xE8),
    ov!(0x000D, 0xC7),
    ov!(0x000F, 0x10),
    ov!(0x0012, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x001C, 0x08),
];
static D3D8_D3DDEVICE_FLUSHVERTEXCACHE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETBACKBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x83),
    ov!(0x0005, 0xF8),
    ov!(0x0006, 0xFF),
    ov!(0x000D, 0x75),
    ov!(0x000E, 0x07),
    ov!(0x001E, 0x8D),
    ov!(0x001F, 0x84),
    ov!(0x0020, 0xC1),
    ov!(0x0021, 0x50),
    ov!(0x0022, 0x21),
    ov!(0x0031, 0xC2),
    ov!(0x0032, 0x0C),
];
static D3D8_D3DDEVICE_GETBACKBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETBACKMATERIAL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x56),
    ov!(0x0006, 0x57),
    ov!(0x000A, 0x0C),
    ov!(0x000D, 0x5C),
    ov!(0x000E, 0x0B),
    ov!(0x0012, 0x11),
    ov!(0x0016, 0xF3),
    ov!(0x001A, 0xC2),
];
static D3D8_D3DDEVICE_GETBACKMATERIAL_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETCREATIONPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0006, 0x8B),
    ov!(0x000E, 0x8B),
    ov!(0x0016, 0x4E),
    ov!(0x001E, 0x83),
    ov!(0x0026, 0x00),
    ov!(0x002E, 0x5E),
    ov!(0x0036, 0xC9),
];
static D3D8_D3DDEVICE_GETCREATIONPARAMETERS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETDEVICECAPS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0006, 0xB9),
    ov!(0x0007, 0x35),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x0010, 0xF3),
    ov!(0x0011, 0xA5),
    ov!(0x0014, 0xC2),
    ov!(0x0015, 0x04),
];
static D3D8_D3DDEVICE_GETDEVICECAPS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETDISPLAYFIELDSTATUS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x4C),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x04),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0x51),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0xF7),
    ov!(0x001C, 0x74),
];
static D3D8_D3DDEVICE_GETDISPLAYFIELDSTATUS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETDISPLAYMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0xB4),
    ov!(0x0014, 0x8B),
    ov!(0x001F, 0x10),
    ov!(0x002A, 0x1B),
    ov!(0x0035, 0x8B),
    ov!(0x0041, 0x89),
    ov!(0x004B, 0x89),
];
static D3D8_D3DDEVICE_GETDISPLAYMODE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETGAMMARAMP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x56),
    ov!(0x000C, 0x8D),
    ov!(0x000D, 0x0C),
    ov!(0x000E, 0x49),
    ov!(0x000F, 0xC1),
    ov!(0x0010, 0xE1),
    ov!(0x0011, 0x08),
    ov!(0x0016, 0x0C),
    ov!(0x001C, 0x00),
    ov!(0x0022, 0x00),
    ov!(0x0027, 0xC2),
];
static D3D8_D3DDEVICE_GETGAMMARAMP_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETLIGHT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xB1),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x7C),
    ov!(0x0014, 0x24),
    ov!(0x0015, 0x10),
    ov!(0x001E, 0xB9),
    ov!(0x001F, 0x1A),
];
static D3D8_D3DDEVICE_GETLIGHT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETLIGHTENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xA1),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x90),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x80),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x8D),
    ov!(0x0016, 0x0C),
    ov!(0x0017, 0xC9),
    ov!(0x0018, 0xC1),
    ov!(0x0019, 0xE1),
    ov!(0x001A, 0x04),
    ov!(0x001B, 0x03),
    ov!(0x001C, 0xCA),
    ov!(0x001D, 0x85),
    ov!(0x001E, 0xC0),
    ov!(0x001F, 0x74),
];
static D3D8_D3DDEVICE_GETLIGHTENABLE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0005,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETMATERIAL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x56),
    ov!(0x0006, 0x57),
    ov!(0x000A, 0x0C),
    ov!(0x000C, 0xB0),
    ov!(0x000D, 0x18),
    ov!(0x000E, 0x0B),
    ov!(0x0011, 0xB9),
    ov!(0x0016, 0xF3),
    ov!(0x001A, 0xC2),
];
static D3D8_D3DDEVICE_GETMATERIAL_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETMODELVIEW_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x57),
    ov!(0x000A, 0x85),
    ov!(0x0010, 0xB0),
    ov!(0x0011, 0xE0),
    ov!(0x0012, 0x05),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0xB9),
    ov!(0x0016, 0x10),
    ov!(0x001C, 0x5E),
];
static D3D8_D3DDEVICE_GETMODELVIEW_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETPIXELSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0x04),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x54),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x04),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0x0A),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x04),
    ov!(0x0013, 0x00),
];
static D3D8_D3DDEVICE_GETPIXELSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETPROJECTIONVIEWPORTMATRIX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0002, 0x35),
    ov!(0x0007, 0x57),
    ov!(0x000A, 0x24),
    ov!(0x000E, 0xA0),
    ov!(0x0012, 0xB9),
    ov!(0x0016, 0x00),
    ov!(0x001A, 0x5E),
];
static D3D8_D3DDEVICE_GETPROJECTIONVIEWPORTMATRIX_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETPUSHBUFFEROFFSET_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0014, 0xB8),
    ov!(0x002A, 0x8B),
    ov!(0x0040, 0x47),
    ov!(0x0056, 0x89),
    ov!(0x006C, 0xAB),
    ov!(0x0082, 0x04),
    ov!(0x0098, 0x89),
];
static D3D8_D3DDEVICE_GETPUSHBUFFEROFFSET_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETRENDERTARGET_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x80),
    ov!(0x000B, 0x85),
    ov!(0x000C, 0xC0),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x4C),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x04),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x01),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x06),
    ov!(0x0015, 0x50),
    ov!(0x0016, 0xE8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x04),
];
static D3D8_D3DDEVICE_GETRENDERTARGET_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0007,
        target: "OFFSET_D3DDevice__m_RenderTarget",
    },
    OovpaXref {
        offset: 0x0017,
        target: "D3DResource_AddRef",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETSCISSORS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x85),
    ov!(0x0005, 0xC9),
    ov!(0x0006, 0xA1),
    ov!(0x0011, 0x00),
    ov!(0x0018, 0x08),
    ov!(0x0019, 0x85),
    ov!(0x001A, 0xC9),
    ov!(0x001B, 0x74),
    ov!(0x001C, 0x08),
    ov!(0x001D, 0x8B),
    ov!(0x001E, 0x90),
];
static D3D8_D3DDEVICE_GETSCISSORS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETSHADERCONSTANTMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0x40),
    ov!(0x0008, 0x21),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x54),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x04),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0x0A),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x04),
    ov!(0x0013, 0x00),
];
static D3D8_D3DDEVICE_GETSHADERCONSTANTMODE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETTILE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000A, 0x56),
    ov!(0x000B, 0x57),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0x7C),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x10),
    ov!(0x0010, 0x8D),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x40),
    ov!(0x0013, 0x8D),
    ov!(0x0014, 0xB4),
    ov!(0x0015, 0xC1),
    ov!(0x001F, 0xF3),
    ov!(0x0023, 0xC2),
    ov!(0x0024, 0x08),
];
static D3D8_D3DDEVICE_GETTILE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETTRANSFORM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x74),
    ov!(0x0008, 0x24),
    ov!(0x0012, 0xC1),
    ov!(0x0013, 0xE6),
    ov!(0x0014, 0x06),
    ov!(0x0015, 0x03),
    ov!(0x0016, 0xF0),
    ov!(0x0017, 0xB9),
    ov!(0x0018, 0x10),
    ov!(0x0020, 0xC2),
    ov!(0x0021, 0x08),
];
static D3D8_D3DDEVICE_GETTRANSFORM_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0x74),
    ov!(0x0008, 0x04),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x54),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x04),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0x0A),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x04),
    ov!(0x0013, 0x00),
];
static D3D8_D3DDEVICE_GETVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERCONSTANT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x74),
    ov!(0x0007, 0x24),
    ov!(0x0008, 0x08),
    ov!(0x0009, 0x57),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x3D),
    ov!(0x0018, 0x00),
    ov!(0x001F, 0xF7),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERCONSTANT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERDECLARATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x8A),
    ov!(0x0013, 0xC7),
    ov!(0x0014, 0x01),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x33),
    ov!(0x001A, 0xC0),
    ov!(0x001B, 0x83),
    ov!(0x001C, 0xC4),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x0C),
    ov!(0x0040, 0x33),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERDECLARATION_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERFUNCTION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0016, 0xC0),
    ov!(0x002E, 0x01),
    ov!(0x0046, 0x10),
    ov!(0x005E, 0x07),
    ov!(0x0076, 0xC7),
    ov!(0x008E, 0x00),
    ov!(0x00A6, 0xF8),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERFUNCTION_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERINPUT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0005, 0xC9),
    ov!(0x000B, 0x74),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x89),
    ov!(0x0014, 0x11),
    ov!(0x0015, 0x8B),
    ov!(0x0016, 0x4C),
    ov!(0x0017, 0x24),
    ov!(0x0018, 0x04),
    ov!(0x0019, 0x85),
    ov!(0x001A, 0xC9),
    ov!(0x001B, 0x74),
    ov!(0x001C, 0x08),
    ov!(0x001D, 0x8B),
    ov!(0x0034, 0x8D),
    ov!(0x0035, 0x0C),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERINPUT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERSIZE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0004, 0x85),
    ov!(0x0007, 0x09),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x4C),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x04),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0x51),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
    ov!(0x0013, 0x00),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERSIZE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVERTEXSHADERTYPE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x8B),
    ov!(0x000A, 0x07),
    ov!(0x0010, 0xEB),
    ov!(0x0016, 0x00),
    ov!(0x001C, 0x40),
    ov!(0x0022, 0xC9),
    ov!(0x0028, 0x08),
];
static D3D8_D3DDEVICE_GETVERTEXSHADERTYPE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVIEWPORT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000C, 0xB0),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0xB9),
    ov!(0x0012, 0x06),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0xF3),
    ov!(0x0017, 0xA5),
    ov!(0x0018, 0x5F),
    ov!(0x0019, 0x5E),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x04),
    ov!(0x001C, 0x00),
];
static D3D8_D3DDEVICE_GETVIEWPORT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_GETVISIBILITYTESTRESULT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000C, 0xC1),
    ov!(0x000D, 0xE9),
    ov!(0x000E, 0x08),
    ov!(0x000F, 0x25),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0x00),
    ov!(0x002B, 0xB8),
    ov!(0x002C, 0x28),
    ov!(0x002D, 0x08),
    ov!(0x002E, 0x76),
    ov!(0x002F, 0x88),
];
static D3D8_D3DDEVICE_GETVISIBILITYTESTRESULT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_INSERTCALLBACK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000D, 0x8B),
    ov!(0x0018, 0x8B),
    ov!(0x0025, 0x50),
    ov!(0x0032, 0x10),
    ov!(0x003F, 0x00),
    ov!(0x004C, 0x00),
    ov!(0x0059, 0x03),
];
static D3D8_D3DDEVICE_INSERTCALLBACK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_INSERTFENCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x00),
    ov!(0x0002, 0xE8),
    ov!(0x0007, 0xC3),
];
static D3D8_D3DDEVICE_INSERTFENCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_SetFence",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ISBUSY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0012, 0x49),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x01),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0xC3),
    ov!(0x0026, 0x8B),
    ov!(0x0027, 0x92),
    ov!(0x0033, 0xC3),
];
static D3D8_D3DDEVICE_ISBUSY_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_ISFENCEPENDING_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x88),
    ov!(0x0007, 0xF0),
    ov!(0x0008, 0x03),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x40),
    ov!(0x000D, 0x1C),
    ov!(0x001A, 0x1B),
    ov!(0x001F, 0x04),
];
static D3D8_D3DDEVICE_ISFENCEPENDING_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_KICKPUSHBUFFER_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0x8B), ov!(0x0001, 0x0D), ov!(0x0006, 0xE9)];
static D3D8_D3DDEVICE_KICKPUSHBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0007,
    target: "D3D_CDevice_KickOff",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_LIGHTENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x68),
    ov!(0x0003, 0x53),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x5C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x70),
    ov!(0x0008, 0x56),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x35),
    ov!(0x000F, 0x3B),
    ov!(0x0010, 0x9E),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x73),
    ov!(0x0016, 0x13),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x8E),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x8D),
    ov!(0x001E, 0x04),
    ov!(0x001F, 0xDB),
    ov!(0x0036, 0x00),
    ov!(0x0052, 0x00),
    ov!(0x006E, 0x75),
    ov!(0x008A, 0x88),
    ov!(0x00A6, 0x75),
    ov!(0x00C2, 0x00),
];
static D3D8_D3DDEVICE_LIGHTENABLE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0067,
        target: "D3DDevice_SetLight",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_LOADVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0007, 0x8A),
    ov!(0x0008, 0x43),
    ov!(0x0009, 0x0C),
    ov!(0x003D, 0xC7),
    ov!(0x003E, 0x00),
    ov!(0x003F, 0x9C),
    ov!(0x0040, 0x1E),
    ov!(0x0041, 0x04),
    ov!(0x004E, 0x89),
    ov!(0x004F, 0x13),
];
static D3D8_D3DDEVICE_LOADVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_LOADVERTEXSHADERPROGRAM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x53),
    ov!(0x0005, 0x0F),
    ov!(0x0006, 0xB7),
    ov!(0x0007, 0x58),
    ov!(0x0008, 0x02),
    ov!(0x0009, 0x55),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x2D),
    ov!(0x0014, 0xE3),
    ov!(0x0015, 0x02),
    ov!(0x0016, 0xF6),
    ov!(0x0017, 0xC1),
];
static D3D8_D3DDEVICE_LOADVERTEXSHADERPROGRAM_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_MULTIPLYTRANSFORM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xE4),
    ov!(0x0005, 0xF0),
    ov!(0x0012, 0x56),
    ov!(0x0013, 0x8B),
    ov!(0x0014, 0x75),
    ov!(0x0015, 0x0C),
    ov!(0x0016, 0x57),
    ov!(0x0017, 0xB9),
    ov!(0x0018, 0x10),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
    ov!(0x004F, 0x00),
];
static D3D8_D3DDEVICE_MULTIPLYTRANSFORM_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_PERSISTDISPLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0008, 0xFF),
    ov!(0x0009, 0x15),
    ov!(0x0017, 0xFF),
    ov!(0x0018, 0x15),
    ov!(0x001F, 0xFF),
    ov!(0x0020, 0x15),
    ov!(0x0036, 0xC3),
];
static D3D8_D3DDEVICE_PERSISTDISPLAY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0004,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_PRESENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x000A, 0x6A),
    ov!(0x000B, 0x02),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x86),
    ov!(0x0013, 0x18),
    ov!(0x0014, 0x25),
    ov!(0x00BE, 0xD1),
    ov!(0x00BF, 0xEB),
    ov!(0x00C0, 0x33),
    ov!(0x00C1, 0xFF),
];
static D3D8_D3DDEVICE_PRESENT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_PRIMEVERTEXCACHE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000D, 0x00),
    ov!(0x001C, 0x55),
    ov!(0x002B, 0x00),
    ov!(0x003A, 0x24),
    ov!(0x0049, 0xCB),
    ov!(0x0058, 0x74),
    ov!(0x0067, 0x00),
    ov!(0x0076, 0x08),
];
static D3D8_D3DDEVICE_PRIMEVERTEXCACHE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x57),
    ov!(0x000D, 0x83),
    ov!(0x000E, 0xF8),
    ov!(0x000F, 0x01),
    ov!(0x0010, 0x75),
    ov!(0x0012, 0x8B),
    ov!(0x0014, 0xE8),
    ov!(0x002C, 0x5F),
    ov!(0x002D, 0xC3),
    ov!(0x002E, 0x48),
    ov!(0x0035, 0x5F),
    ov!(0x0036, 0xC3),
];
static D3D8_D3DDEVICE_RELEASE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_RESET_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x001E, 0x75),
    ov!(0x001F, 0xF5),
    ov!(0x0022, 0xE8),
    ov!(0x0027, 0x8B),
    ov!(0x0028, 0x74),
    ov!(0x0029, 0x24),
    ov!(0x002A, 0x14),
];
static D3D8_D3DDEVICE_RESET_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "D3D_CDevice_FreeFrameBuffers",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_RUNPUSHBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0005, 0x1D),
    ov!(0x000A, 0x55),
    ov!(0x000B, 0x56),
    ov!(0x000C, 0x57),
    ov!(0x000D, 0x6A),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0xCB),
    ov!(0x0011, 0xE8),
    ov!(0x001A, 0x83),
    ov!(0x0023, 0x8B),
];
static D3D8_D3DDEVICE_RUNPUSHBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_RUNVERTEXSTATESHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000D, 0x8B),
    ov!(0x0010, 0x0C),
    ov!(0x0011, 0x85),
    ov!(0x0012, 0xC9),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x21),
    ov!(0x0015, 0xD9),
    ov!(0x0016, 0x41),
    ov!(0x001F, 0x41),
];
static D3D8_D3DDEVICE_RUNVERTEXSTATESHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SELECTVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0016, 0x81),
    ov!(0x0017, 0xC9),
    ov!(0x0018, 0xA0),
    ov!(0x0019, 0x03),
    ov!(0x003F, 0xC7),
    ov!(0x0040, 0x00),
    ov!(0x0041, 0x94),
    ov!(0x0042, 0x1E),
    ov!(0x0043, 0x08),
    ov!(0x0051, 0x89),
    ov!(0x0052, 0x06),
];
static D3D8_D3DDEVICE_SELECTVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETBACKMATERIAL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000A, 0x57),
    ov!(0x000B, 0x8D),
    ov!(0x000C, 0xB8),
    ov!(0x000D, 0x5C),
    ov!(0x000E, 0x0B),
    ov!(0x000F, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0018, 0x8B),
    ov!(0x001E, 0x5F),
];
static D3D8_D3DDEVICE_SETBACKMATERIAL_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETFLICKERFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x91),
    ov!(0x000C, 0x08),
    ov!(0x000D, 0x23),
    ov!(0x0010, 0x6A),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x6A),
    ov!(0x0014, 0x0B),
    ov!(0x0015, 0x52),
    ov!(0x001C, 0xC2),
    ov!(0x001D, 0x04),
];
static D3D8_D3DDEVICE_SETFLICKERFILTER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETGAMMARAMP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xE0),
    ov!(0x0013, 0x01),
    ov!(0x0014, 0x53),
    ov!(0x002F, 0xF3),
    ov!(0x0030, 0xA5),
    ov!(0x003E, 0x53),
    ov!(0x003F, 0x8B),
    ov!(0x0040, 0xCA),
];
static D3D8_D3DDEVICE_SETGAMMARAMP_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETINDICES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x10),
    ov!(0x003E, 0xF7),
    ov!(0x003F, 0xC1),
    ov!(0x0040, 0xFF),
    ov!(0x0041, 0xFF),
    ov!(0x0042, 0x78),
    ov!(0x0067, 0x89),
    ov!(0x0068, 0xBE),
    ov!(0x0069, 0x7C),
    ov!(0x006A, 0x04),
];
static D3D8_D3DDEVICE_SETINDICES_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETLIGHT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x3B),
    ov!(0x0018, 0xDF),
    ov!(0x0019, 0x89),
    ov!(0x001A, 0x7C),
    ov!(0x001B, 0x24),
    ov!(0x001C, 0x14),
    ov!(0x001D, 0x0F),
    ov!(0x001E, 0x82),
    ov!(0x001F, 0xBF),
    ov!(0x003B, 0x8B),
    ov!(0x003C, 0xD8),
    ov!(0x0062, 0x8B),
    ov!(0x0063, 0xF3),
];
static D3D8_D3DDEVICE_SETLIGHT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETMATERIAL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000B, 0x8D),
    ov!(0x000C, 0xB8),
    ov!(0x000D, 0x18),
    ov!(0x000E, 0x0B),
    ov!(0x0011, 0xB9),
    ov!(0x0012, 0x11),
    ov!(0x001B, 0x83),
    ov!(0x001C, 0xC9),
    ov!(0x001D, 0x20),
    ov!(0x0023, 0xC2),
    ov!(0x0024, 0x04),
];
static D3D8_D3DDEVICE_SETMATERIAL_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETMODELVIEW_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0012, 0x08),
    ov!(0x0026, 0x53),
    ov!(0x003A, 0x8B),
    ov!(0x004E, 0x80),
    ov!(0x0062, 0x00),
    ov!(0x0076, 0x00),
    ov!(0x008A, 0x0C),
];
static D3D8_D3DDEVICE_SETMODELVIEW_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETPIXELSHADERCONSTANT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x001E, 0x4C),
    ov!(0x003E, 0x11),
    ov!(0x0061, 0xDF),
    ov!(0x007E, 0xF6),
    ov!(0x009E, 0x05),
    ov!(0x00C0, 0x51),
    ov!(0x00DE, 0xC4),
];
static D3D8_D3DDEVICE_SETPIXELSHADERCONSTANT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETPIXELSHADERPROGRAM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x85),
    ov!(0x0005, 0xD2),
    ov!(0x0006, 0xA1),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0xC7),
    ov!(0x0014, 0x01),
    ov!(0x0015, 0x01),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0xC7),
    ov!(0x001A, 0x80),
    ov!(0x0029, 0x89),
    ov!(0x003A, 0xE9),
];
static D3D8_D3DDEVICE_SETPIXELSHADERPROGRAM_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x50),
    ov!(0x0001, 0x51),
    ov!(0x0002, 0xE8),
    ov!(0x0007, 0xC3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3DDevice_SetRenderStateNotInline",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE2_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x50),
    ov!(0x0001, 0x51),
    ov!(0x0002, 0xE8),
    ov!(0x0007, 0xC3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE2_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3DDevice_SetRenderStateNotInline",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATENOTINLINE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0008, 0x7D),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x0C),
    ov!(0x000C, 0xB5),
    ov!(0x0018, 0xE8),
];
static D3D8_D3DDEVICE_SETRENDERSTATENOTINLINE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0019,
        target: "D3DDevice_SetRenderState_Simple",
    },
    OovpaXref {
        offset: 0x0020,
        target: "D3D_g_RenderState",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0013, 0xA3),
    ov!(0x003B, 0x83),
    ov!(0x003C, 0xC0),
    ov!(0x003D, 0x0C),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3DRS_FillMode",
    },
    OovpaXref {
        offset: 0x0014,
        target: "D3DRS_BackFillMode",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_CULLMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0013, 0xC7),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x08),
    ov!(0x0016, 0x03),
    ov!(0x0017, 0x04),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x75),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_CULLMODE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0025,
        target: "D3DRS_CullMode",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_DEFERRED_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x04),
    ov!(0x0002, 0x8D),
    ov!(0x0007, 0x09),
    ov!(0x0008, 0x05),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x14),
    ov!(0x000F, 0x8D),
    ov!(0x0014, 0xC3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_DEFERRED_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_DONOTCULLUNCOMPRESSED_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0009, 0xE8),
    ov!(0x000E, 0xC2),
    ov!(0x000F, 0x04),
    ov!(0x0010, 0x00),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_DONOTCULLUNCOMPRESSED_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0005,
        target: "D3DRS_DoNotCullUncompressed",
    },
    OovpaXref {
        offset: 0x000A,
        target: "D3D_CommonSetDebugRegisters",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_DXT1NOISEENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x8B),
    ov!(0x0005, 0x56),
    ov!(0x001D, 0x80),
    ov!(0x001E, 0xE2),
    ov!(0x001F, 0x3C),
    ov!(0x0022, 0x80),
    ov!(0x0023, 0xFA),
    ov!(0x0024, 0x20),
    ov!(0x002E, 0x83),
    ov!(0x002F, 0xE1),
    ov!(0x0030, 0x01),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_DXT1NOISEENABLE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0017, 0x89),
    ov!(0x0018, 0x48),
    ov!(0x0019, 0x04),
    ov!(0x001A, 0x89),
    ov!(0x001B, 0x48),
    ov!(0x001C, 0x08),
    ov!(0x001D, 0x83),
    ov!(0x001E, 0xC0),
    ov!(0x001F, 0x0C),
    ov!(0x0029, 0xC2),
    ov!(0x002A, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_FILLMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000D, 0x8B),
    ov!(0x0013, 0x8B),
    ov!(0x0023, 0xC7),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x8C),
    ov!(0x0026, 0x03),
    ov!(0x0027, 0x08),
    ov!(0x0028, 0x00),
    ov!(0x002F, 0x83),
    ov!(0x0030, 0xC0),
    ov!(0x0031, 0x0C),
    ov!(0x0034, 0x89),
    ov!(0x003B, 0xC2),
    ov!(0x003C, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_FILLMODE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000F,
        target: "D3DRS_TwoSidedLighting",
    },
    OovpaXref {
        offset: 0x0015,
        target: "D3DRS_BackFillMode",
    },
    OovpaXref {
        offset: 0x0036,
        target: "D3DRS_FillMode",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x002E, 0xC7),
    ov!(0x002F, 0x00),
    ov!(0x0030, 0xA8),
    ov!(0x0031, 0x02),
    ov!(0x0032, 0x04),
    ov!(0x0033, 0x00),
    ov!(0x0037, 0x83),
    ov!(0x0038, 0xC0),
    ov!(0x0039, 0x08),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_FRONTFACE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x001A, 0x83),
    ov!(0x001B, 0xC0),
    ov!(0x001C, 0x08),
    ov!(0x0024, 0x89),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_FRONTFACE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0026,
        target: "D3DRS_FrontFace",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0015, 0xD8),
    ov!(0x0016, 0x8E),
    ov!(0x0017, 0x08),
    ov!(0x0018, 0x05),
    ov!(0x0032, 0x81),
    ov!(0x0033, 0xFF),
    ov!(0x003A, 0xBF),
    ov!(0x003B, 0xFF),
    ov!(0x005C, 0xC2),
    ov!(0x005D, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_LOGICOP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0013, 0x75),
    ov!(0x0015, 0xC7),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0xBC),
    ov!(0x0018, 0x17),
    ov!(0x0019, 0x04),
    ov!(0x001A, 0x00),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_LOGICOP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0025,
        target: "D3DRS_LogicOp",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x000D, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000E,
        target: "D3DRS_MultiSampleAntiAlias",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x35),
    ov!(0x000D, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000E,
        target: "D3DRS_MultiSampleMask",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x001E, 0xE8),
    ov!(0x002B, 0xC2),
    ov!(0x002C, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000E,
        target: "D3DRS_MultiSampleType",
    },
    OovpaXref {
        offset: 0x001F,
        target: "D3DDevice_SetViewport",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001D, 0x89),
    ov!(0x001E, 0x06),
    ov!(0x001F, 0x83),
    ov!(0x0020, 0x4E),
    ov!(0x0021, 0x08),
    ov!(0x0022, 0x02),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x56),
    ov!(0x000C, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3DRS_OcclusionCullEnable",
    },
    OovpaXref {
        offset: 0x0012,
        target: "XMETAL_StartPush",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_PSTEXTUREMODES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000C, 0x81),
    ov!(0x000D, 0xCA),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x04),
    ov!(0x001B, 0x89),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_PSTEXTUREMODES_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x001D,
        target: "D3DRS_PSTextureModes",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_ROPZCMPALWAYSREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0009, 0xE8),
    ov!(0x000E, 0xC2),
    ov!(0x000F, 0x04),
    ov!(0x0010, 0x00),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_ROPZCMPALWAYSREAD_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0005,
        target: "D3DRS_RopZCmpAlwaysRead",
    },
    OovpaXref {
        offset: 0x000A,
        target: "D3D_CommonSetDebugRegisters",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_ROPZREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0009, 0xE8),
    ov!(0x000E, 0xC2),
    ov!(0x000F, 0x04),
    ov!(0x0010, 0x00),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_ROPZREAD_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0005,
        target: "D3DRS_RopZRead",
    },
    OovpaXref {
        offset: 0x000A,
        target: "D3D_CommonSetDebugRegisters",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0xC7),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x6C),
    ov!(0x0014, 0x1E),
    ov!(0x0015, 0x04),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x8D),
    ov!(0x0018, 0x91),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0xFE),
    ov!(0x0020, 0x83),
    ov!(0x0021, 0xC0),
    ov!(0x0022, 0x08),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_SIMPLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0xC0),
    ov!(0x0007, 0x08),
    ov!(0x0015, 0x89),
    ov!(0x0016, 0x48),
    ov!(0x0017, 0xF8),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x50),
    ov!(0x001A, 0xFC),
    ov!(0x001C, 0x52),
    ov!(0x001D, 0x51),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_SIMPLE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x56),
    ov!(0x000C, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3DRS_StencilCullEnable",
    },
    OovpaXref {
        offset: 0x0012,
        target: "XMETAL_StartPush",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0068, 0xC7),
    ov!(0x006A, 0x2C),
    ov!(0x006B, 0x03),
    ov!(0x006C, 0x04),
    ov!(0x006D, 0x00),
    ov!(0x0076, 0x89),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0078,
        target: "D3DRS_StencilEnable",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0051, 0xC7),
    ov!(0x0053, 0x70),
    ov!(0x0054, 0x03),
    ov!(0x0055, 0x04),
    ov!(0x0056, 0x00),
    ov!(0x005F, 0x89),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0061,
        target: "D3DRS_StencilFail",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x86),
    ov!(0x0009, 0x14),
    ov!(0x000A, 0x04),
    ov!(0x002E, 0xF3),
    ov!(0x002F, 0xAB),
    ov!(0x003C, 0xC2),
    ov!(0x003D, 0x04),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000A, 0x8B),
    ov!(0x000E, 0x8B),
    ov!(0x001C, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x001D,
        target: "D3DRS_TwoSidedLighting",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC9),
    ov!(0x000C, 0x02),
    ov!(0x001A, 0xC7),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x28),
    ov!(0x001D, 0x03),
    ov!(0x001E, 0x04),
    ov!(0x0023, 0x83),
    ov!(0x0024, 0xC0),
    ov!(0x0025, 0x08),
    ov!(0x002F, 0xC2),
    ov!(0x0030, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0005, 0x8B),
    ov!(0x000C, 0xA3),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0007,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000D,
        target: "D3DRS_YuvEnable",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_ZBIAS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0001, 0x56),
    ov!(0x000B, 0x0F),
    ov!(0x000C, 0x95),
    ov!(0x000D, 0xC0),
    ov!(0x0014, 0xDB),
    ov!(0x0015, 0x44),
    ov!(0x0016, 0x24),
    ov!(0x0017, 0x10),
    ov!(0x001A, 0x7D),
    ov!(0x001B, 0x06),
    ov!(0x001C, 0xD8),
    ov!(0x001D, 0x05),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_ZBIAS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_ZENABLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x11),
    ov!(0x0015, 0x8B),
    ov!(0x0016, 0x96),
    ov!(0x0017, 0x10),
    ov!(0x0018, 0x04),
    ov!(0x0028, 0xC7),
    ov!(0x0029, 0x00),
    ov!(0x002A, 0x0C),
    ov!(0x002B, 0x03),
    ov!(0x002C, 0x04),
    ov!(0x0069, 0xC2),
    ov!(0x006A, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_ZENABLE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETSHADERCONSTANTMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xA8),
    ov!(0x0005, 0x10),
    ov!(0x0007, 0x8B),
    ov!(0x0012, 0x81),
    ov!(0x0013, 0xC9),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x02),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
];
static D3D8_D3DDEVICE_SETSHADERCONSTANTMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETSOFTDISPLAYFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x91),
    ov!(0x000C, 0x08),
    ov!(0x000D, 0x23),
    ov!(0x0010, 0x6A),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x6A),
    ov!(0x0014, 0x0E),
    ov!(0x0015, 0x52),
    ov!(0x001C, 0xC2),
    ov!(0x001D, 0x04),
];
static D3D8_D3DDEVICE_SETSOFTDISPLAYFILTER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETSTREAMSOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0039, 0x81),
    ov!(0x003A, 0xC2),
    ov!(0x003B, 0x00),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0xF8),
    ov!(0x003E, 0xFF),
    ov!(0x0049, 0x75),
    ov!(0x004A, 0x06),
    ov!(0x006A, 0x81),
    ov!(0x006B, 0xC9),
    ov!(0x006C, 0x80),
    ov!(0x006D, 0x02),
];
static D3D8_D3DDEVICE_SETSTREAMSOURCE_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0023,
        target: "D3D_g_Stream_i_pVertexBuffer",
    },
    OovpaXref {
        offset: 0x0053,
        target: "D3D_g_Stream",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0004, 0x83),
    ov!(0x0005, 0xF8),
    ov!(0x0013, 0xC1),
    ov!(0x0015, 0x05),
    ov!(0x0024, 0x89),
    ov!(0x002F, 0xC2),
    ov!(0x0030, 0x0C),
];
static D3D8_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0027,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0013, 0xC1),
    ov!(0x0014, 0xE2),
    ov!(0x0015, 0x06),
    ov!(0x0016, 0x81),
    ov!(0x0017, 0xC2),
    ov!(0x0018, 0x24),
    ov!(0x0019, 0x1B),
    ov!(0x001A, 0x04),
    ov!(0x002A, 0xC1),
    ov!(0x002B, 0xE1),
    ov!(0x002C, 0x07),
    ov!(0x0034, 0xC2),
    ov!(0x0035, 0x08),
];
static D3D8_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTATE_BUMPENV_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000E, 0x8B),
    ov!(0x0016, 0x8B),
    ov!(0x0022, 0x24),
    ov!(0x002E, 0x24),
    ov!(0x003A, 0x04),
    ov!(0x0046, 0x8B),
    ov!(0x0052, 0xB5),
];
static D3D8_D3DDEVICE_SETTEXTURESTATE_BUMPENV_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000D, 0x8B),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x8D),
    ov!(0x0014, 0xE0),
    ov!(0x0015, 0x0A),
    ov!(0x0016, 0x04),
    ov!(0x0017, 0x00),
    ov!(0x001A, 0x8B),
    ov!(0x001C, 0x24),
    ov!(0x001D, 0x0C),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x08),
];
static D3D8_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTATE_DEFERRED_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x04),
    ov!(0x0002, 0x95),
    ov!(0x000E, 0xC1),
    ov!(0x000F, 0xE1),
    ov!(0x0010, 0x05),
    ov!(0x0013, 0x03),
    ov!(0x0014, 0xCA),
    ov!(0x0015, 0x8B),
    ov!(0x0016, 0x54),
    ov!(0x0017, 0x24),
    ov!(0x0018, 0x08),
    ov!(0x001F, 0x89),
    ov!(0x0020, 0x14),
    ov!(0x0021, 0x8D),
];
static D3D8_D3DDEVICE_SETTEXTURESTATE_DEFERRED_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0022,
    target: "D3D_g_DeferredTextureState",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000B, 0xC1),
    ov!(0x000C, 0xE0),
    ov!(0x000D, 0x07),
    ov!(0x0024, 0x81),
    ov!(0x0025, 0xF9),
    ov!(0x0028, 0x02),
    ov!(0x003B, 0xBF),
    ov!(0x003C, 0x11),
    ov!(0x003D, 0x85),
    ov!(0x0097, 0xD3),
    ov!(0x0098, 0xE0),
];
static D3D8_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "D3DTSS_TEXCOORDINDEX",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTILE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xEC),
    ov!(0x0008, 0x18),
    ov!(0x0068, 0x81),
    ov!(0x006A, 0xFF),
    ov!(0x006B, 0xFF),
    ov!(0x006C, 0xFF),
    ov!(0x006D, 0x03),
];
static D3D8_D3DDEVICE_SETTILE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETTRANSFORM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x54),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x53),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x1D),
    ov!(0x0014, 0xC1),
    ov!(0x0015, 0xE7),
    ov!(0x0016, 0x06),
];
static D3D8_D3DDEVICE_SETTRANSFORM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000B,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATA2F_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x35),
    ov!(0x0011, 0x8D),
    ov!(0x0012, 0x14),
    ov!(0x0013, 0xCD),
    ov!(0x0014, 0x80),
    ov!(0x0015, 0x18),
    ov!(0x0016, 0x08),
    ov!(0x0017, 0x00),
    ov!(0x0028, 0x83),
    ov!(0x0029, 0xC0),
    ov!(0x002A, 0x0C),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x0C),
];
static D3D8_D3DDEVICE_SETVERTEXDATA2F_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0003,
    target: "D3D_g_pDevice",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATA2S_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x8D),
    ov!(0x0012, 0x14),
    ov!(0x0013, 0x8D),
    ov!(0x0024, 0xC1),
    ov!(0x0025, 0xE1),
    ov!(0x0026, 0x10),
    ov!(0x0029, 0x89),
    ov!(0x002A, 0x48),
    ov!(0x002B, 0x04),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x0C),
];
static D3D8_D3DDEVICE_SETVERTEXDATA2S_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATA4F_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xF9),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x75),
    ov!(0x0015, 0x07),
    ov!(0x001D, 0x81),
    ov!(0x001E, 0xC1),
    ov!(0x001F, 0xA0),
    ov!(0x0020, 0x01),
    ov!(0x0050, 0xC2),
    ov!(0x0051, 0x14),
];
static D3D8_D3DDEVICE_SETVERTEXDATA4F_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATA4S_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x56),
    ov!(0x0010, 0x08),
    ov!(0x0019, 0xBF),
    ov!(0x0022, 0x24),
    ov!(0x002B, 0x54),
    ov!(0x0034, 0x24),
    ov!(0x003D, 0x08),
    ov!(0x0046, 0x00),
];
static D3D8_D3DDEVICE_SETVERTEXDATA4S_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATA4UB_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x56),
    ov!(0x0010, 0x08),
    ov!(0x0019, 0xC9),
    ov!(0x0022, 0x54),
    ov!(0x002B, 0x14),
    ov!(0x0034, 0x24),
    ov!(0x003D, 0xFC),
];
static D3D8_D3DDEVICE_SETVERTEXDATA4UB_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXDATACOLOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0x0F),
    ov!(0x0020, 0xB6),
    ov!(0x0021, 0x54),
    ov!(0x0022, 0x24),
    ov!(0x0023, 0x12),
    ov!(0x0026, 0x81),
    ov!(0x0027, 0xE7),
    ov!(0x0028, 0xFF),
    ov!(0x002C, 0xC1),
    ov!(0x002D, 0xE7),
    ov!(0x002E, 0x10),
    ov!(0x0031, 0x81),
    ov!(0x0032, 0xE1),
    ov!(0x0033, 0x00),
    ov!(0x0034, 0xFF),
    ov!(0x0035, 0x00),
    ov!(0x0036, 0xFF),
];
static D3D8_D3DDEVICE_SETVERTEXDATACOLOR_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXSHADER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0005, 0xF6),
    ov!(0x0006, 0xC3),
    ov!(0x0007, 0x01),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x86),
    ov!(0x006D, 0xC2),
    ov!(0x006E, 0x04),
    ov!(0x008C, 0xC7),
    ov!(0x008D, 0x40),
    ov!(0x008E, 0x08),
    ov!(0x008F, 0x94),
    ov!(0x0090, 0x1E),
];
static D3D8_D3DDEVICE_SETVERTEXSHADER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0012,
    target: "D3DDevice__m_VertexShader_OFFSET",
}];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXSHADERCONSTANT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0011, 0x8A),
    ov!(0x0012, 0x4B),
    ov!(0x0013, 0x0C),
    ov!(0x001A, 0xF6),
    ov!(0x001B, 0xC1),
    ov!(0x001C, 0x10),
    ov!(0x0063, 0x7E),
    ov!(0x0064, 0x19),
    ov!(0x009B, 0xC2),
    ov!(0x009C, 0x0C),
];
static D3D8_D3DDEVICE_SETVERTEXSHADERCONSTANT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTEXSHADERINPUT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x001E, 0x03),
    ov!(0x003E, 0x8B),
    ov!(0x005E, 0xC7),
    ov!(0x007E, 0xF8),
    ov!(0x009E, 0xC6),
    ov!(0x00BE, 0x7F),
    ov!(0x00DE, 0xCA),
    ov!(0x00FE, 0x17),
];
static D3D8_D3DDEVICE_SETVERTEXSHADERINPUT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVERTICALBLANKCALLBACK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000A, 0x89),
    ov!(0x000B, 0x81),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x00),
];
static D3D8_D3DDEVICE_SETVERTICALBLANKCALLBACK_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0006,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x000C,
        target: "D3DDevice__m_VBlankCallback_OFFSET",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SETVIEWPORT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x002F, 0x8B),
    ov!(0x0030, 0x5C),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x1C),
    ov!(0x0055, 0x75),
    ov!(0x0056, 0x12),
    ov!(0x009D, 0x42),
];
static D3D8_D3DDEVICE_SETVIEWPORT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SUSPEND_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x0D),
    ov!(0x0006, 0x6A),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0xE8),
    ov!(0x000D, 0xC3),
];
static D3D8_D3DDEVICE_SUSPEND_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0002,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0009,
        target: "D3D_CDevice_SetStateVB",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_SWITCHTEXTURE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0xC0),
    ov!(0x0007, 0x0C),
    ov!(0x0008, 0x3B),
    ov!(0x0009, 0x05),
    ov!(0x000E, 0x73),
    ov!(0x000F, 0x15),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static D3D8_D3DDEVICE_SWITCHTEXTURE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE_UPDATEOVERLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x08),
    ov!(0x001F, 0x7C),
    ov!(0x006F, 0x8B),
    ov!(0x0080, 0x8B),
    ov!(0x0081, 0x54),
    ov!(0x0082, 0x24),
    ov!(0x0083, 0x20),
    ov!(0x0084, 0x8B),
    ov!(0x0085, 0x3F),
    ov!(0x0086, 0x83),
    ov!(0x0087, 0xE1),
];
static D3D8_D3DDEVICE_UPDATEOVERLAY_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DDEVICE__M_VERTICALBLANKEVENT__GENERICFRAGMENT_3911_ENTRIES: &[OovpaEntry] = &[
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
    ov!(0x001C, 0x50),
    ov!(0x001D, 0xFF),
];
static D3D8_D3DDEVICE__M_VERTICALBLANKEVENT__GENERICFRAGMENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3DDevice__m_VerticalBlankEvent_OFFSET",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DPALETTE_LOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xF6),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x0C),
    ov!(0x0004, 0xA0),
    ov!(0x000A, 0x75),
    ov!(0x000B, 0x06),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x4C),
    ov!(0x001A, 0x24),
    ov!(0x001B, 0x0C),
    ov!(0x001C, 0x89),
    ov!(0x001D, 0x01),
];
static D3D8_D3DPALETTE_LOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DRS_STENCILS_AND_OCCLUSION__GENERICFRAGMENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0011, 0x8B),
    ov!(0x001B, 0x8B),
    ov!(0x0025, 0x81),
    ov!(0x0031, 0x83),
    ov!(0x0032, 0xC9),
    ov!(0x0033, 0x01),
];
static D3D8_D3DRS_STENCILS_AND_OCCLUSION__GENERICFRAGMENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0002,
        target: "D3DRS_StencilCullEnable",
    },
    OovpaXref {
        offset: 0x0013,
        target: "D3DRS_OcclusionCullEnable",
    },
    OovpaXref {
        offset: 0x001D,
        target: "D3DRS_StencilEnable",
    },
    OovpaXref {
        offset: 0x0027,
        target: "D3DRS_StencilFail",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0xA9),
    ov!(0x0008, 0xFF),
    ov!(0x0009, 0xFF),
    ov!(0x000E, 0x25),
    ov!(0x0011, 0x07),
    ov!(0x0013, 0x3D),
    ov!(0x0016, 0x05),
    ov!(0x0033, 0x5E),
    ov!(0x0034, 0xC2),
    ov!(0x0035, 0x04),
];
static D3D8_D3DRESOURCE_ADDREF_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_BLOCKUNTILNOTBUSY_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static D3D8_D3DRESOURCE_BLOCKUNTILNOTBUSY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "D3D_BlockOnResource",
}];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_GETTYPE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0013, 0x2A),
    ov!(0x0028, 0x00),
    ov!(0x003D, 0x00),
    ov!(0x0052, 0x74),
    ov!(0x0067, 0x00),
    ov!(0x007C, 0x04),
    ov!(0x0091, 0x00),
];
static D3D8_D3DRESOURCE_GETTYPE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_ISBUSY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0024, 0xA9),
    ov!(0x0025, 0x00),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x78),
    ov!(0x0035, 0x75),
    ov!(0x0036, 0x44),
    ov!(0x004E, 0x8B),
    ov!(0x004F, 0x41),
    ov!(0x0050, 0x14),
    ov!(0x0079, 0x73),
    ov!(0x007A, 0x09),
];
static D3D8_D3DRESOURCE_ISBUSY_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_REGISTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x54),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x41),
    ov!(0x000A, 0x04),
    ov!(0x000F, 0x81),
    ov!(0x0010, 0xE2),
    ov!(0x0013, 0x07),
    ov!(0x0015, 0x81),
    ov!(0x0016, 0xFA),
    ov!(0x0019, 0x02),
    ov!(0x001B, 0x74),
    ov!(0x001C, 0x05),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x08),
];
static D3D8_D3DRESOURCE_REGISTER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DRESOURCE_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0x81),
    ov!(0x000A, 0xE1),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0xFF),
    ov!(0x0014, 0x25),
    ov!(0x0017, 0x07),
    ov!(0x0019, 0x3D),
    ov!(0x001C, 0x05),
    ov!(0x003E, 0x5E),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x04),
];
static D3D8_D3DRESOURCE_RELEASE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DSURFACE_GETDESC_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x04),
    ov!(0x0008, 0x50),
    ov!(0x0009, 0x6A),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x51),
    ov!(0x000C, 0xE8),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
    ov!(0x0013, 0x00),
];
static D3D8_D3DSURFACE_GETDESC_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DSURFACE_LOCKRECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x10),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0x50),
    ov!(0x0011, 0x51),
    ov!(0x0012, 0x52),
    ov!(0x0013, 0x6A),
    ov!(0x0015, 0x6A),
    ov!(0x0018, 0xE8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x10),
];
static D3D8_D3DSURFACE_LOCKRECT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DTEXTURE_GETSURFACELEVEL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x8D),
    ov!(0x000B, 0x14),
    ov!(0x0017, 0x8B),
    ov!(0x001A, 0x24),
    ov!(0x0023, 0x28),
    ov!(0x0024, 0x51),
    ov!(0x0025, 0x52),
    ov!(0x0026, 0x6A),
    ov!(0x0043, 0xE8),
    ov!(0x004C, 0xC2),
    ov!(0x004D, 0x0C),
];
static D3D8_D3DTEXTURE_GETSURFACELEVEL_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DTEXTURE_LOCKRECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x14),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x10),
    ov!(0x000C, 0x50),
    ov!(0x0011, 0x51),
    ov!(0x0016, 0x52),
    ov!(0x0017, 0x50),
    ov!(0x0018, 0x6A),
    ov!(0x0019, 0x00),
    ov!(0x001B, 0xE8),
    ov!(0x0020, 0xC2),
    ov!(0x0021, 0x14),
];
static D3D8_D3DTEXTURE_LOCKRECT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DVERTEXBUFFER_GETDESC_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x74),
    ov!(0x000A, 0xC7),
    ov!(0x000E, 0x00),
    ov!(0x0015, 0x89),
    ov!(0x0016, 0x46),
    ov!(0x001A, 0x08),
];
static D3D8_D3DVERTEXBUFFER_GETDESC_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DVERTEXBUFFER_LOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8A),
    ov!(0x0002, 0x5C),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x18),
    ov!(0x0009, 0x75),
    ov!(0x000A, 0x1E),
    ov!(0x0029, 0xF6),
    ov!(0x002A, 0xC3),
    ov!(0x002B, 0xA0),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x14),
];
static D3D8_D3DVERTEXBUFFER_LOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3DVOLUMETEXTURE_LOCKBOX_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static D3D8_D3DVOLUMETEXTURE_LOCKBOX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "D3D8_Lock3DSurface",
}];

// Source: D3D8/3911.inl
static D3D8_D3D_ALLOCCONTIGUOUSMEMORY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x04),
    ov!(0x0008, 0x68),
    ov!(0x0009, 0x04),
    ov!(0x000A, 0x04),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x50),
];
static D3D8_D3D_ALLOCCONTIGUOUSMEMORY_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_BLOCKONRESOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x74),
    ov!(0x000C, 0x24),
    ov!(0x000D, 0x08),
    ov!(0x0010, 0x25),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x07),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x3D),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x00),
    ov!(0x0018, 0x05),
    ov!(0x0019, 0x00),
];
static D3D8_D3D_BLOCKONRESOURCE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_BLOCKONTIME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000A, 0x57),
    ov!(0x004E, 0x3D),
    ov!(0x004F, 0x00),
    ov!(0x0050, 0x80),
    ov!(0x0051, 0x00),
    ov!(0x006E, 0xBD),
    ov!(0x006F, 0x00),
    ov!(0x0070, 0x01),
    ov!(0x0071, 0x04),
];
static D3D8_D3D_BLOCKONTIME_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_CHECKDEVICEFORMAT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000E, 0x18),
    ov!(0x001E, 0x00),
    ov!(0x002E, 0x42),
    ov!(0x0041, 0x40),
    ov!(0x004E, 0x74),
    ov!(0x005E, 0x08),
    ov!(0x006E, 0x3C),
];
static D3D8_D3D_CHECKDEVICEFORMAT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_CLEARSTATEBLOCKFLAGS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x15),
    ov!(0x0007, 0x57),
    ov!(0x000F, 0xBF),
    ov!(0x0014, 0xF3),
    ov!(0x0015, 0xAB),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x8A),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x33),
    ov!(0x001D, 0xF6),
    ov!(0x001E, 0x8D),
    ov!(0x001F, 0x82),
];
static D3D8_D3D_CLEARSTATEBLOCKFLAGS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_COMMONSETDEBUGREGISTERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000D, 0x83),
    ov!(0x000E, 0xE2),
    ov!(0x000F, 0xF7),
    ov!(0x0016, 0x8B),
    ov!(0x0031, 0x81),
    ov!(0x0032, 0xE2),
    ov!(0x0033, 0xFF),
    ov!(0x0034, 0xFF),
    ov!(0x0035, 0xEF),
    ov!(0x0036, 0xE7),
    ov!(0x003D, 0x8B),
    ov!(0x0054, 0xA1),
];
static D3D8_D3D_COMMONSETDEBUGREGISTERS_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "D3D_g_pDevice",
    },
    OovpaXref {
        offset: 0x0018,
        target: "D3DRS_DoNotCullUncompressed",
    },
    OovpaXref {
        offset: 0x003F,
        target: "D3DRS_RopZCmpAlwaysRead",
    },
    OovpaXref {
        offset: 0x0055,
        target: "D3DRS_RopZRead",
    },
];

// Source: D3D8/3911.inl
static D3D8_D3D_CREATETEXTURE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x20),
    ov!(0x0004, 0x53),
    ov!(0x004F, 0x83),
    ov!(0x0050, 0x64),
    ov!(0x0051, 0x24),
    ov!(0x0052, 0x30),
    ov!(0x0053, 0xF7),
    ov!(0x0054, 0x6A),
    ov!(0x0055, 0x14),
    ov!(0x0056, 0x6A),
    ov!(0x0057, 0x40),
];
static D3D8_D3D_CREATETEXTURE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_DESTROYRESOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x07),
    ov!(0x000A, 0x81),
    ov!(0x000B, 0xE6),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x07),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x81),
    ov!(0x0011, 0xFE),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x05),
    ov!(0x0015, 0x00),
    ov!(0x001C, 0x57),
    ov!(0x0028, 0x81),
    ov!(0x0029, 0xFE),
    ov!(0x002A, 0x00),
    ov!(0x002B, 0x00),
    ov!(0x002C, 0x05),
    ov!(0x002D, 0x00),
];
static D3D8_D3D_DESTROYRESOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001E,
    target: "D3D_BlockOnResource",
}];

// Source: D3D8/3911.inl
static D3D8_D3D_ENUMADAPTERMODES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0012, 0x57),
    ov!(0x0013, 0x89),
    ov!(0x0014, 0x44),
    ov!(0x0015, 0x24),
    ov!(0x0016, 0x14),
    ov!(0x0017, 0x89),
    ov!(0x0018, 0x4C),
    ov!(0x0019, 0x24),
    ov!(0x001A, 0x20),
    ov!(0x001B, 0xE8),
    ov!(0x0050, 0x6C),
    ov!(0x0051, 0x24),
];
static D3D8_D3D_ENUMADAPTERMODES_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_GETADAPTERDISPLAYMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x85),
    ov!(0x0005, 0xC0),
    ov!(0x0006, 0x74),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0xB8),
    ov!(0x0009, 0x6C),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x76),
    ov!(0x000C, 0x88),
    ov!(0x000D, 0xC2),
    ov!(0x000E, 0x08),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x8B),
    ov!(0x001E, 0x33),
    ov!(0x001F, 0xC0),
    ov!(0x0030, 0x00),
    ov!(0x0031, 0x8B),
];
static D3D8_D3D_GETADAPTERDISPLAYMODE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_GETADAPTERIDENTIFIER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x85),
    ov!(0x000A, 0x08),
    ov!(0x0010, 0x56),
    ov!(0x0016, 0xB9),
    ov!(0x0020, 0xF3),
    ov!(0x0022, 0x5F),
    ov!(0x0028, 0x00),
];
static D3D8_D3D_GETADAPTERIDENTIFIER_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_GETADAPTERMODECOUNT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0xEC),
    ov!(0x0002, 0x08),
    ov!(0x0003, 0x53),
    ov!(0x0004, 0x55),
    ov!(0x0005, 0x56),
    ov!(0x0006, 0x57),
    ov!(0x0007, 0xC7),
    ov!(0x0008, 0x44),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0xE8),
    ov!(0x001D, 0x8B),
    ov!(0x001E, 0xD1),
    ov!(0x0040, 0x14),
    ov!(0x0041, 0xEB),
];
static D3D8_D3D_GETADAPTERMODECOUNT_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_GETDEVICECAPS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x85),
    ov!(0x000A, 0x08),
    ov!(0x0010, 0x83),
    ov!(0x0016, 0x08),
    ov!(0x001C, 0xC2),
    ov!(0x0022, 0x0C),
    ov!(0x0029, 0x33),
];
static D3D8_D3D_GETDEVICECAPS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_KICKOFFANDWAITFORIDLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x48),
    ov!(0x0007, 0x1C),
    ov!(0x000A, 0x51),
    ov!(0x0010, 0xC3),
];
static D3D8_D3D_KICKOFFANDWAITFORIDLE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_LAZYSETPOINTPARAMS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x000A, 0x57),
    ov!(0x000B, 0x08),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xE2),
    ov!(0x000E, 0xFE),
    ov!(0x000F, 0x57),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0x57),
    ov!(0x0012, 0x08),
    ov!(0x0013, 0xE8),
    ov!(0x0037, 0x89),
    ov!(0x006E, 0x15),
];
static D3D8_D3D_LAZYSETPOINTPARAMS_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_RECORDSTATEBLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x18),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x56),
    ov!(0x000F, 0x8B),
    ov!(0x001F, 0x89),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x45),
    ov!(0x0039, 0xEC),
    ov!(0x003A, 0x8A),
    ov!(0x003B, 0x02),
    ov!(0x003C, 0x42),
    ov!(0x003D, 0x84),
    ov!(0x003E, 0xC0),
    ov!(0x005C, 0xE8),
];
static D3D8_D3D_RECORDSTATEBLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_SETFENCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x7E),
    ov!(0x0011, 0x1C),
    ov!(0x003D, 0x83),
    ov!(0x003E, 0xE1),
    ov!(0x003F, 0x3F),
    ov!(0x0077, 0x89),
    ov!(0x0078, 0x3C),
    ov!(0x0079, 0xAE),
    ov!(0x0096, 0xC2),
    ov!(0x0097, 0x04),
];
static D3D8_D3D_SETFENCE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_D3D_SETPUSHBUFFERSIZE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x4C),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0xA3),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x0D),
    ov!(0x0013, 0xC2),
    ov!(0x0014, 0x08),
];
static D3D8_D3D_SETPUSHBUFFERSIZE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_DIRECT3D_CHECKDEVICEMULTISAMPLETYPE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0x00),
    ov!(0x001E, 0x54),
    ov!(0x002E, 0xC9),
    ov!(0x003E, 0x8B),
    ov!(0x004E, 0x08),
    ov!(0x005E, 0x72),
    ov!(0x006E, 0x03),
];
static D3D8_DIRECT3D_CHECKDEVICEMULTISAMPLETYPE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_GET2DSURFACEDESC_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x002B, 0x0F),
    ov!(0x002C, 0xB6),
    ov!(0x002D, 0x57),
    ov!(0x002E, 0x0D),
    ov!(0x0056, 0x3B),
    ov!(0x0057, 0x82),
    ov!(0x0058, 0x54),
    ov!(0x0059, 0x21),
    ov!(0x00AE, 0xC2),
    ov!(0x00AF, 0x0C),
];
static D3D8_GET2DSURFACEDESC_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_LOCK2DSURFACE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x51),
    ov!(0x0006, 0xF6),
    ov!(0x0007, 0xC3),
    ov!(0x0008, 0x20),
    ov!(0x003F, 0xF6),
    ov!(0x0040, 0xC3),
    ov!(0x0041, 0x40),
    ov!(0x0071, 0xC1),
    ov!(0x0072, 0xEA),
    ov!(0x0073, 0x03),
    ov!(0x0098, 0xC2),
    ov!(0x0099, 0x18),
];
static D3D8_LOCK2DSURFACE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_LOCK3DSURFACE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0008, 0xF6),
    ov!(0x0009, 0xC3),
    ov!(0x000A, 0x20),
    ov!(0x0040, 0xF6),
    ov!(0x0041, 0xC3),
    ov!(0x0042, 0x40),
    ov!(0x006D, 0x83),
    ov!(0x006E, 0xE2),
    ov!(0x006F, 0x3C),
    ov!(0x0099, 0xC2),
    ov!(0x009A, 0x14),
];
static D3D8_LOCK3DSURFACE_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3911.inl
static D3D8_XMETAL_STARTPUSH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x01),
    ov!(0x0006, 0x3B),
    ov!(0x0007, 0x41),
    ov!(0x0008, 0x04),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x04),
];
static D3D8_XMETAL_STARTPUSH_3911_XREFS: &[OovpaXref] = &[];

// Source: D3D8/3925.inl
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3925_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x4C),
    ov!(0x0007, 0x24),
    ov!(0x0008, 0x04),
    ov!(0x001D, 0xE8),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3925_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "D3DRS_MultiSampleType",
    },
    OovpaXref {
        offset: 0x001E,
        target: "D3DDevice_SetRenderTarget",
    },
];

pub const PATTERNS: &[OovpaPattern] = &[
    OovpaPattern {
        name: "D3DDevice_SetRenderState_OcclusionCullEnable",
        detect_size: 0x0010,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilCullEnable",
        detect_size: 0x0010,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_YuvEnable",
        detect_size: 0x0010,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_1024_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 1024,
    },
    OovpaPattern {
        name: "D3D_UpdateProjectionViewportTransform",
        detect_size: 0x0021,
        entries: D3D8_D3D_UPDATEPROJECTIONVIEWPORTTRANSFORM_3900_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3900,
    },
    OovpaPattern {
        name: "CDevice_FreeFrameBuffers",
        detect_size: 0x007C,
        entries: D3D8_CDEVICE_FREEFRAMEBUFFERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_InitializeFrameBuffers",
        detect_size: 0x002D,
        entries: D3D8_CDEVICE_INITIALIZEFRAMEBUFFERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_KickOff",
        detect_size: 0x006A,
        entries: D3D8_CDEVICE_KICKOFF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_MakeSpace",
        detect_size: 0x0033,
        entries: D3D8_CDEVICE_MAKESPACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_SetStateUP",
        detect_size: 0x0025,
        entries: D3D8_CDEVICE_SETSTATEUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDevice_SetStateVB",
        detect_size: 0x0031,
        entries: D3D8_CDEVICE_SETSTATEVB_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMiniport_CreateCtxDmaObject",
        detect_size: 0x0020,
        entries: D3D8_CMINIPORT_CREATECTXDMAOBJECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMiniport_GetDisplayCapabilities",
        detect_size: 0x001F,
        entries: D3D8_CMINIPORT_GETDISPLAYCAPABILITIES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMiniport_InitHardware",
        detect_size: 0x0020,
        entries: D3D8_CMINIPORT_INITHARDWARE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMiniport_IsFlipPending",
        detect_size: 0x000E,
        entries: D3D8_CMINIPORT_ISFLIPPENDING_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DBaseTexture_GetLevelCount",
        detect_size: 0x000D,
        entries: D3D8_D3DBASETEXTURE_GETLEVELCOUNT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DCubeTexture_GetCubeMapSurface",
        detect_size: 0x0051,
        entries: D3D8_D3DCUBETEXTURE_GETCUBEMAPSURFACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DCubeTexture_LockRect",
        detect_size: 0x0005,
        entries: D3D8_D3DCUBETEXTURE_LOCKRECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_AddRef",
        detect_size: 0x0010,
        entries: D3D8_D3DDEVICE_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_ApplyStateBlock",
        detect_size: 0x00F7,
        entries: D3D8_D3DDEVICE_APPLYSTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Begin",
        detect_size: 0x003A,
        entries: D3D8_D3DDEVICE_BEGIN_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_BeginPushBuffer",
        detect_size: 0x005A,
        entries: D3D8_D3DDEVICE_BEGINPUSHBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_BeginStateBlock",
        detect_size: 0x000E,
        entries: D3D8_D3DDEVICE_BEGINSTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_BeginVisibilityTest",
        detect_size: 0x0025,
        entries: D3D8_D3DDEVICE_BEGINVISIBILITYTEST_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_BlockOnFence",
        detect_size: 0x000F,
        entries: D3D8_D3DDEVICE_BLOCKONFENCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_BlockUntilVerticalBlank",
        detect_size: 0x0024,
        entries: D3D8_D3DDEVICE_BLOCKUNTILVERTICALBLANK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CaptureStateBlock",
        detect_size: 0x00DF,
        entries: D3D8_D3DDEVICE_CAPTURESTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Clear",
        detect_size: 0x0061,
        entries: D3D8_D3DDEVICE_CLEAR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CopyRects",
        detect_size: 0x0016,
        entries: D3D8_D3DDEVICE_COPYRECTS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateCubeTexture",
        detect_size: 0x0027,
        entries: D3D8_D3DDEVICE_CREATECUBETEXTURE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateImageSurface",
        detect_size: 0x001C,
        entries: D3D8_D3DDEVICE_CREATEIMAGESURFACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateIndexBuffer",
        detect_size: 0x0034,
        entries: D3D8_D3DDEVICE_CREATEINDEXBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreatePalette",
        detect_size: 0x0051,
        entries: D3D8_D3DDEVICE_CREATEPALETTE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreatePixelShader",
        detect_size: 0x0044,
        entries: D3D8_D3DDEVICE_CREATEPIXELSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateStateBlock",
        detect_size: 0x00FF,
        entries: D3D8_D3DDEVICE_CREATESTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateTexture",
        detect_size: 0x002B,
        entries: D3D8_D3DDEVICE_CREATETEXTURE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateVertexBuffer",
        detect_size: 0x0055,
        entries: D3D8_D3DDEVICE_CREATEVERTEXBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateVertexShader",
        detect_size: 0x00FF,
        entries: D3D8_D3DDEVICE_CREATEVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_CreateVolumeTexture",
        detect_size: 0x002E,
        entries: D3D8_D3DDEVICE_CREATEVOLUMETEXTURE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DeletePatch",
        detect_size: 0x0026,
        entries: D3D8_D3DDEVICE_DELETEPATCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DeletePixelShader",
        detect_size: 0x001B,
        entries: D3D8_D3DDEVICE_DELETEPIXELSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DeleteStateBlock",
        detect_size: 0x0084,
        entries: D3D8_D3DDEVICE_DELETESTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DeleteVertexShader",
        detect_size: 0x0018,
        entries: D3D8_D3DDEVICE_DELETEVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVertices",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_DRAWINDEXEDVERTICES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DrawIndexedVerticesUP",
        detect_size: 0x0019,
        entries: D3D8_D3DDEVICE_DRAWINDEXEDVERTICESUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVertices",
        detect_size: 0x001C,
        entries: D3D8_D3DDEVICE_DRAWVERTICES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_DrawVerticesUP",
        detect_size: 0x0019,
        entries: D3D8_D3DDEVICE_DRAWVERTICESUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_EnableOverlay",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_ENABLEOVERLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_End",
        detect_size: 0x0041,
        entries: D3D8_D3DDEVICE_END_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_EndPushBuffer",
        detect_size: 0x0063,
        entries: D3D8_D3DDEVICE_ENDPUSHBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_EndStateBlock",
        detect_size: 0x000E,
        entries: D3D8_D3DDEVICE_ENDSTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_EndVisibilityTest",
        detect_size: 0x0038,
        entries: D3D8_D3DDEVICE_ENDVISIBILITYTEST_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_FlushVertexCache",
        detect_size: 0x001D,
        entries: D3D8_D3DDEVICE_FLUSHVERTEXCACHE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetBackBuffer",
        detect_size: 0x0033,
        entries: D3D8_D3DDEVICE_GETBACKBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetBackMaterial",
        detect_size: 0x001B,
        entries: D3D8_D3DDEVICE_GETBACKMATERIAL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetCreationParameters",
        detect_size: 0x0037,
        entries: D3D8_D3DDEVICE_GETCREATIONPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetDeviceCaps",
        detect_size: 0x0016,
        entries: D3D8_D3DDEVICE_GETDEVICECAPS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetDisplayFieldStatus",
        detect_size: 0x001D,
        entries: D3D8_D3DDEVICE_GETDISPLAYFIELDSTATUS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetDisplayMode",
        detect_size: 0x004C,
        entries: D3D8_D3DDEVICE_GETDISPLAYMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetGammaRamp",
        detect_size: 0x0028,
        entries: D3D8_D3DDEVICE_GETGAMMARAMP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetLight",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_GETLIGHT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetLightEnable",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_GETLIGHTENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetMaterial",
        detect_size: 0x001B,
        entries: D3D8_D3DDEVICE_GETMATERIAL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetModelView",
        detect_size: 0x001D,
        entries: D3D8_D3DDEVICE_GETMODELVIEW_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetPixelShader",
        detect_size: 0x0014,
        entries: D3D8_D3DDEVICE_GETPIXELSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetProjectionViewportMatrix",
        detect_size: 0x001B,
        entries: D3D8_D3DDEVICE_GETPROJECTIONVIEWPORTMATRIX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetPushBufferOffset",
        detect_size: 0x0099,
        entries: D3D8_D3DDEVICE_GETPUSHBUFFEROFFSET_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetRenderTarget",
        detect_size: 0x001F,
        entries: D3D8_D3DDEVICE_GETRENDERTARGET_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetScissors",
        detect_size: 0x001F,
        entries: D3D8_D3DDEVICE_GETSCISSORS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetShaderConstantMode",
        detect_size: 0x0014,
        entries: D3D8_D3DDEVICE_GETSHADERCONSTANTMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetTile",
        detect_size: 0x0025,
        entries: D3D8_D3DDEVICE_GETTILE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetTransform",
        detect_size: 0x0022,
        entries: D3D8_D3DDEVICE_GETTRANSFORM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShader",
        detect_size: 0x0014,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderConstant",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERCONSTANT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderDeclaration",
        detect_size: 0x0041,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERDECLARATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderFunction",
        detect_size: 0x00A7,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERFUNCTION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderInput",
        detect_size: 0x0036,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERINPUT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderSize",
        detect_size: 0x0014,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERSIZE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVertexShaderType",
        detect_size: 0x0029,
        entries: D3D8_D3DDEVICE_GETVERTEXSHADERTYPE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetViewport",
        detect_size: 0x001D,
        entries: D3D8_D3DDEVICE_GETVIEWPORT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_GetVisibilityTestResult",
        detect_size: 0x0030,
        entries: D3D8_D3DDEVICE_GETVISIBILITYTESTRESULT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_InsertCallback",
        detect_size: 0x005A,
        entries: D3D8_D3DDEVICE_INSERTCALLBACK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_InsertFence",
        detect_size: 0x0008,
        entries: D3D8_D3DDEVICE_INSERTFENCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_IsBusy",
        detect_size: 0x0034,
        entries: D3D8_D3DDEVICE_ISBUSY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_IsFencePending",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_ISFENCEPENDING_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_KickPushBuffer",
        detect_size: 0x000B,
        entries: D3D8_D3DDEVICE_KICKPUSHBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_LightEnable",
        detect_size: 0x00C3,
        entries: D3D8_D3DDEVICE_LIGHTENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShader",
        detect_size: 0x0050,
        entries: D3D8_D3DDEVICE_LOADVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_LoadVertexShaderProgram",
        detect_size: 0x0018,
        entries: D3D8_D3DDEVICE_LOADVERTEXSHADERPROGRAM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_MultiplyTransform",
        detect_size: 0x0050,
        entries: D3D8_D3DDEVICE_MULTIPLYTRANSFORM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_PersistDisplay",
        detect_size: 0x0037,
        entries: D3D8_D3DDEVICE_PERSISTDISPLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Present",
        detect_size: 0x00C2,
        entries: D3D8_D3DDEVICE_PRESENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_PrimeVertexCache",
        detect_size: 0x0077,
        entries: D3D8_D3DDEVICE_PRIMEVERTEXCACHE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Release",
        detect_size: 0x0037,
        entries: D3D8_D3DDEVICE_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Reset",
        detect_size: 0x002B,
        entries: D3D8_D3DDEVICE_RESET_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_RunPushBuffer",
        detect_size: 0x0024,
        entries: D3D8_D3DDEVICE_RUNPUSHBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_RunVertexStateShader",
        detect_size: 0x0020,
        entries: D3D8_D3DDEVICE_RUNVERTEXSTATESHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SelectVertexShader",
        detect_size: 0x0053,
        entries: D3D8_D3DDEVICE_SELECTVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetBackMaterial",
        detect_size: 0x001F,
        entries: D3D8_D3DDEVICE_SETBACKMATERIAL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetFlickerFilter",
        detect_size: 0x001E,
        entries: D3D8_D3DDEVICE_SETFLICKERFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetGammaRamp",
        detect_size: 0x0041,
        entries: D3D8_D3DDEVICE_SETGAMMARAMP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetIndices",
        detect_size: 0x006B,
        entries: D3D8_D3DDEVICE_SETINDICES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetLight",
        detect_size: 0x0064,
        entries: D3D8_D3DDEVICE_SETLIGHT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetMaterial",
        detect_size: 0x0025,
        entries: D3D8_D3DDEVICE_SETMATERIAL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetModelView",
        detect_size: 0x008B,
        entries: D3D8_D3DDEVICE_SETMODELVIEW_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShaderConstant",
        detect_size: 0x00DF,
        entries: D3D8_D3DDEVICE_SETPIXELSHADERCONSTANT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetPixelShaderProgram",
        detect_size: 0x003B,
        entries: D3D8_D3DDEVICE_SETPIXELSHADERPROGRAM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState",
        detect_size: 0x0008,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState2",
        detect_size: 0x0008,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE2_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderStateNotInline",
        detect_size: 0x0024,
        entries: D3D8_D3DDEVICE_SETRENDERSTATENOTINLINE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_BackFillMode",
        detect_size: 0x004A,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_CullMode",
        detect_size: 0x0029,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_CULLMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_Deferred",
        detect_size: 0x0015,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_DEFERRED_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_DoNotCullUncompressed",
        detect_size: 0x0011,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_DONOTCULLUNCOMPRESSED_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_Dxt1NoiseEnable",
        detect_size: 0x0031,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_DXT1NOISEENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        detect_size: 0x002B,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FillMode",
        detect_size: 0x003D,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_FILLMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FogColor",
        detect_size: 0x0046,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_FrontFace",
        detect_size: 0x002A,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_FRONTFACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LineWidth",
        detect_size: 0x005E,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_LogicOp",
        detect_size: 0x0029,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_LOGICOP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        detect_size: 0x0012,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        detect_size: 0x0012,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleType",
        detect_size: 0x002D,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        detect_size: 0x002C,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_OcclusionCullEnable",
        detect_size: 0x0016,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_PSTextureModes",
        detect_size: 0x0023,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_PSTEXTUREMODES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_RopZCmpAlwaysRead",
        detect_size: 0x0011,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_ROPZCMPALWAYSREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_RopZRead",
        detect_size: 0x0011,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_ROPZREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        detect_size: 0x0023,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_Simple",
        detect_size: 0x001E,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_SIMPLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilCullEnable",
        detect_size: 0x0016,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilEnable",
        detect_size: 0x007C,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_StencilFail",
        detect_size: 0x0065,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_TextureFactor",
        detect_size: 0x004B,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        detect_size: 0x0021,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_VertexBlend",
        detect_size: 0x0031,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_YuvEnable",
        detect_size: 0x0011,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZBias",
        detect_size: 0x001E,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_ZBIAS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_ZEnable",
        detect_size: 0x006B,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_ZENABLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetShaderConstantMode",
        detect_size: 0x0018,
        entries: D3D8_D3DDEVICE_SETSHADERCONSTANTMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetSoftDisplayFilter",
        detect_size: 0x001E,
        entries: D3D8_D3DDEVICE_SETSOFTDISPLAYFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetStreamSource",
        detect_size: 0x006E,
        entries: D3D8_D3DDEVICE_SETSTREAMSOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureStageStateNotInline",
        detect_size: 0x0031,
        entries: D3D8_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BorderColor",
        detect_size: 0x0036,
        entries: D3D8_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_BumpEnv",
        detect_size: 0x0053,
        entries: D3D8_D3DDEVICE_SETTEXTURESTATE_BUMPENV_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        detect_size: 0x0032,
        entries: D3D8_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_Deferred",
        detect_size: 0x0026,
        entries: D3D8_D3DDEVICE_SETTEXTURESTATE_DEFERRED_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        detect_size: 0x0099,
        entries: D3D8_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTile",
        detect_size: 0x006E,
        entries: D3D8_D3DDEVICE_SETTILE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetTransform",
        detect_size: 0x0017,
        entries: D3D8_D3DDEVICE_SETTRANSFORM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2f",
        detect_size: 0x0030,
        entries: D3D8_D3DDEVICE_SETVERTEXDATA2F_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData2s",
        detect_size: 0x0034,
        entries: D3D8_D3DDEVICE_SETVERTEXDATA2S_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4f",
        detect_size: 0x0052,
        entries: D3D8_D3DDEVICE_SETVERTEXDATA4F_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4s",
        detect_size: 0x0047,
        entries: D3D8_D3DDEVICE_SETVERTEXDATA4S_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexData4ub",
        detect_size: 0x003E,
        entries: D3D8_D3DDEVICE_SETVERTEXDATA4UB_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexDataColor",
        detect_size: 0x0037,
        entries: D3D8_D3DDEVICE_SETVERTEXDATACOLOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShader",
        detect_size: 0x0091,
        entries: D3D8_D3DDEVICE_SETVERTEXSHADER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShaderConstant",
        detect_size: 0x009D,
        entries: D3D8_D3DDEVICE_SETVERTEXSHADERCONSTANT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVertexShaderInput",
        detect_size: 0x00FF,
        entries: D3D8_D3DDEVICE_SETVERTEXSHADERINPUT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetVerticalBlankCallback",
        detect_size: 0x0013,
        entries: D3D8_D3DDEVICE_SETVERTICALBLANKCALLBACK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetViewport",
        detect_size: 0x009E,
        entries: D3D8_D3DDEVICE_SETVIEWPORT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_Suspend",
        detect_size: 0x000E,
        entries: D3D8_D3DDEVICE_SUSPEND_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SwitchTexture",
        detect_size: 0x0024,
        entries: D3D8_D3DDEVICE_SWITCHTEXTURE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_UpdateOverlay",
        detect_size: 0x0088,
        entries: D3D8_D3DDEVICE_UPDATEOVERLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice__m_VerticalBlankEvent__GenericFragment",
        detect_size: 0x001E,
        entries: D3D8_D3DDEVICE__M_VERTICALBLANKEVENT__GENERICFRAGMENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DPalette_Lock",
        detect_size: 0x001E,
        entries: D3D8_D3DPALETTE_LOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DRS_Stencils_and_Occlusion__GenericFragment",
        detect_size: 0x0034,
        entries: D3D8_D3DRS_STENCILS_AND_OCCLUSION__GENERICFRAGMENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_AddRef",
        detect_size: 0x0036,
        entries: D3D8_D3DRESOURCE_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_BlockUntilNotBusy",
        detect_size: 0x0005,
        entries: D3D8_D3DRESOURCE_BLOCKUNTILNOTBUSY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_GetType",
        detect_size: 0x0092,
        entries: D3D8_D3DRESOURCE_GETTYPE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_IsBusy",
        detect_size: 0x007B,
        entries: D3D8_D3DRESOURCE_ISBUSY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_Register",
        detect_size: 0x0027,
        entries: D3D8_D3DRESOURCE_REGISTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DResource_Release",
        detect_size: 0x004D,
        entries: D3D8_D3DRESOURCE_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DSurface_GetDesc",
        detect_size: 0x0014,
        entries: D3D8_D3DSURFACE_GETDESC_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DSurface_LockRect",
        detect_size: 0x001F,
        entries: D3D8_D3DSURFACE_LOCKRECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DTexture_GetSurfaceLevel",
        detect_size: 0x004E,
        entries: D3D8_D3DTEXTURE_GETSURFACELEVEL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DTexture_LockRect",
        detect_size: 0x0022,
        entries: D3D8_D3DTEXTURE_LOCKRECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DVertexBuffer_GetDesc",
        detect_size: 0x001B,
        entries: D3D8_D3DVERTEXBUFFER_GETDESC_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DVertexBuffer_Lock",
        detect_size: 0x004C,
        entries: D3D8_D3DVERTEXBUFFER_LOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DVolumeTexture_LockBox",
        detect_size: 0x0005,
        entries: D3D8_D3DVOLUMETEXTURE_LOCKBOX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_AllocContiguousMemory",
        detect_size: 0x000E,
        entries: D3D8_D3D_ALLOCCONTIGUOUSMEMORY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_BlockOnResource",
        detect_size: 0x001A,
        entries: D3D8_D3D_BLOCKONRESOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_BlockOnTime",
        detect_size: 0x0072,
        entries: D3D8_D3D_BLOCKONTIME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_CheckDeviceFormat",
        detect_size: 0x006F,
        entries: D3D8_D3D_CHECKDEVICEFORMAT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_ClearStateBlockFlags",
        detect_size: 0x0020,
        entries: D3D8_D3D_CLEARSTATEBLOCKFLAGS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_CommonSetDebugRegisters",
        detect_size: 0x0059,
        entries: D3D8_D3D_COMMONSETDEBUGREGISTERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_CreateTexture",
        detect_size: 0x0058,
        entries: D3D8_D3D_CREATETEXTURE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_DestroyResource",
        detect_size: 0x002E,
        entries: D3D8_D3D_DESTROYRESOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_EnumAdapterModes",
        detect_size: 0x0052,
        entries: D3D8_D3D_ENUMADAPTERMODES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_GetAdapterDisplayMode",
        detect_size: 0x0032,
        entries: D3D8_D3D_GETADAPTERDISPLAYMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_GetAdapterIdentifier",
        detect_size: 0x0029,
        entries: D3D8_D3D_GETADAPTERIDENTIFIER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_GetAdapterModeCount",
        detect_size: 0x0042,
        entries: D3D8_D3D_GETADAPTERMODECOUNT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_GetDeviceCaps",
        detect_size: 0x002A,
        entries: D3D8_D3D_GETDEVICECAPS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_KickOffAndWaitForIdle",
        detect_size: 0x0011,
        entries: D3D8_D3D_KICKOFFANDWAITFORIDLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_LazySetPointParams",
        detect_size: 0x006F,
        entries: D3D8_D3D_LAZYSETPOINTPARAMS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_RecordStateBlock",
        detect_size: 0x005D,
        entries: D3D8_D3D_RECORDSTATEBLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_SetFence",
        detect_size: 0x0098,
        entries: D3D8_D3D_SETFENCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3D_SetPushBufferSize",
        detect_size: 0x0015,
        entries: D3D8_D3D_SETPUSHBUFFERSIZE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "Direct3D_CheckDeviceMultiSampleType",
        detect_size: 0x006F,
        entries: D3D8_DIRECT3D_CHECKDEVICEMULTISAMPLETYPE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "Get2DSurfaceDesc",
        detect_size: 0x00B0,
        entries: D3D8_GET2DSURFACEDESC_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "Lock2DSurface",
        detect_size: 0x009A,
        entries: D3D8_LOCK2DSURFACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "Lock3DSurface",
        detect_size: 0x009B,
        entries: D3D8_LOCK3DSURFACE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMETAL_StartPush",
        detect_size: 0x0012,
        entries: D3D8_XMETAL_STARTPUSH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "D3DDevice_SetRenderState_MultiSampleType",
        detect_size: 0x0024,
        entries: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3925_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3925,
    },
];

pub const METADATA: &[OovpaPatternMeta] = &[
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_OcclusionCullEnable",
        min_version: 1024,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilCullEnable",
        min_version: 1024,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_YuvEnable",
        min_version: 1024,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_1024_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_UpdateProjectionViewportTransform",
        min_version: 3900,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_UPDATEPROJECTIONVIEWPORTTRANSFORM_3900_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_FreeFrameBuffers",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_FREEFRAMEBUFFERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_InitializeFrameBuffers",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_INITIALIZEFRAMEBUFFERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_KickOff",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_KICKOFF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_MakeSpace",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_MAKESPACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateUP",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_SETSTATEUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDevice_SetStateVB",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CDEVICE_SETSTATEVB_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMiniport_CreateCtxDmaObject",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CMINIPORT_CREATECTXDMAOBJECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMiniport_GetDisplayCapabilities",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CMINIPORT_GETDISPLAYCAPABILITIES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMiniport_InitHardware",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CMINIPORT_INITHARDWARE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMiniport_IsFlipPending",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_CMINIPORT_ISFLIPPENDING_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DBaseTexture_GetLevelCount",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DBASETEXTURE_GETLEVELCOUNT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DCubeTexture_GetCubeMapSurface",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DCUBETEXTURE_GETCUBEMAPSURFACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DCubeTexture_LockRect",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DCUBETEXTURE_LOCKRECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_AddRef",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_ApplyStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_APPLYSTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Begin",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BEGIN_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginPushBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BEGINPUSHBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BEGINSTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BeginVisibilityTest",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BEGINVISIBILITYTEST_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BlockOnFence",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BLOCKONFENCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_BlockUntilVerticalBlank",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_BLOCKUNTILVERTICALBLANK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CaptureStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CAPTURESTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Clear",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CLEAR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CopyRects",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_COPYRECTS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateCubeTexture",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATECUBETEXTURE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateImageSurface",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEIMAGESURFACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateIndexBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEINDEXBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreatePalette",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEPALETTE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreatePixelShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEPIXELSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATESTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateTexture",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATETEXTURE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateVertexBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEVERTEXBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_CreateVolumeTexture",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_CREATEVOLUMETEXTURE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeletePatch",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DELETEPATCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeletePixelShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DELETEPIXELSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeleteStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DELETESTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DeleteVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DELETEVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVertices",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DRAWINDEXEDVERTICES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawIndexedVerticesUP",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DRAWINDEXEDVERTICESUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVertices",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DRAWVERTICES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_DrawVerticesUP",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_DRAWVERTICESUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EnableOverlay",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ENABLEOVERLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_End",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_END_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndPushBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ENDPUSHBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ENDSTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_EndVisibilityTest",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ENDVISIBILITYTEST_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_FlushVertexCache",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_FLUSHVERTEXCACHE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetBackBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETBACKBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetBackMaterial",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETBACKMATERIAL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetCreationParameters",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETCREATIONPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDeviceCaps",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETDEVICECAPS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDisplayFieldStatus",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETDISPLAYFIELDSTATUS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetDisplayMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETDISPLAYMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetGammaRamp",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETGAMMARAMP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetLight",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETLIGHT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetLightEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETLIGHTENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetMaterial",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETMATERIAL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetModelView",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETMODELVIEW_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetPixelShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETPIXELSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetProjectionViewportMatrix",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETPROJECTIONVIEWPORTMATRIX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetPushBufferOffset",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETPUSHBUFFEROFFSET_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetRenderTarget",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETRENDERTARGET_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetScissors",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETSCISSORS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetShaderConstantMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETSHADERCONSTANTMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetTile",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETTILE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetTransform",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETTRANSFORM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderConstant",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERCONSTANT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderDeclaration",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERDECLARATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderFunction",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERFUNCTION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderInput",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERINPUT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderSize",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERSIZE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVertexShaderType",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVERTEXSHADERTYPE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetViewport",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVIEWPORT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_GetVisibilityTestResult",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_GETVISIBILITYTESTRESULT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_InsertCallback",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_INSERTCALLBACK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_InsertFence",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_INSERTFENCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_IsBusy",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ISBUSY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_IsFencePending",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_ISFENCEPENDING_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_KickPushBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_KICKPUSHBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LightEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_LIGHTENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_LOADVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_LoadVertexShaderProgram",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_LOADVERTEXSHADERPROGRAM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_MultiplyTransform",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_MULTIPLYTRANSFORM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_PersistDisplay",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_PERSISTDISPLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Present",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_PRESENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_PrimeVertexCache",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_PRIMEVERTEXCACHE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Release",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Reset",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_RESET_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_RunPushBuffer",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_RUNPUSHBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_RunVertexStateShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_RUNVERTEXSTATESHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SelectVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SELECTVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetBackMaterial",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETBACKMATERIAL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetFlickerFilter",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETFLICKERFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetGammaRamp",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETGAMMARAMP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetIndices",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETINDICES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetLight",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETLIGHT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetMaterial",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETMATERIAL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetModelView",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETMODELVIEW_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShaderConstant",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETPIXELSHADERCONSTANT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetPixelShaderProgram",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETPIXELSHADERPROGRAM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState2",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE2_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderStateNotInline",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATENOTINLINE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_BackFillMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_BACKFILLMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_CullMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_CULLMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_Deferred",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_DEFERRED_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_DoNotCullUncompressed",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_DONOTCULLUNCOMPRESSED_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_Dxt1NoiseEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_DXT1NOISEENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_EdgeAntiAlias",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_EDGEANTIALIAS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FillMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_FILLMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FogColor",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_FOGCOLOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_FrontFace",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_FRONTFACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LineWidth",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_LINEWIDTH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_LogicOp",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_LOGICOP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleAntiAlias",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEANTIALIAS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleMask",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLEMASK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleType",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_NormalizeNormals",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_NORMALIZENORMALS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_OcclusionCullEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_OCCLUSIONCULLENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_PSTextureModes",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_PSTEXTUREMODES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_RopZCmpAlwaysRead",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_ROPZCMPALWAYSREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_RopZRead",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_ROPZREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ShadowFunc",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_SHADOWFUNC_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_Simple",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_SIMPLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilCullEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILCULLENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_StencilFail",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_STENCILFAIL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_TextureFactor",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_TEXTUREFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_TwoSidedLighting",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_TWOSIDEDLIGHTING_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_VertexBlend",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_VERTEXBLEND_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_YuvEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_YUVENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZBias",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_ZBIAS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_ZEnable",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_ZENABLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetShaderConstantMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETSHADERCONSTANTMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetSoftDisplayFilter",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETSOFTDISPLAYFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetStreamSource",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETSTREAMSOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureStageStateNotInline",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTAGESTATENOTINLINE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BorderColor",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTATE_BORDERCOLOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_BumpEnv",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTATE_BUMPENV_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_ColorKeyColor",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTATE_COLORKEYCOLOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_Deferred",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTATE_DEFERRED_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTextureState_TexCoordIndex",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTEXTURESTATE_TEXCOORDINDEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTile",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTILE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetTransform",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETTRANSFORM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2f",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATA2F_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData2s",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATA2S_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4f",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATA4F_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4s",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATA4S_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexData4ub",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATA4UB_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexDataColor",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXDATACOLOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShader",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXSHADER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShaderConstant",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXSHADERCONSTANT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVertexShaderInput",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTEXSHADERINPUT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetVerticalBlankCallback",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVERTICALBLANKCALLBACK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetViewport",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SETVIEWPORT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_Suspend",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SUSPEND_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SwitchTexture",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_SWITCHTEXTURE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_UpdateOverlay",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE_UPDATEOVERLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice__m_VerticalBlankEvent__GenericFragment",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DDEVICE__M_VERTICALBLANKEVENT__GENERICFRAGMENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DPalette_Lock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DPALETTE_LOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DRS_Stencils_and_Occlusion__GenericFragment",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRS_STENCILS_AND_OCCLUSION__GENERICFRAGMENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_AddRef",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_BlockUntilNotBusy",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_BLOCKUNTILNOTBUSY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_GetType",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_GETTYPE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_IsBusy",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_ISBUSY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_Register",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_REGISTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DResource_Release",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DRESOURCE_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DSurface_GetDesc",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DSURFACE_GETDESC_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DSurface_LockRect",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DSURFACE_LOCKRECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DTexture_GetSurfaceLevel",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DTEXTURE_GETSURFACELEVEL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DTexture_LockRect",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DTEXTURE_LOCKRECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DVertexBuffer_GetDesc",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DVERTEXBUFFER_GETDESC_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DVertexBuffer_Lock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DVERTEXBUFFER_LOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DVolumeTexture_LockBox",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3DVOLUMETEXTURE_LOCKBOX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_AllocContiguousMemory",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_ALLOCCONTIGUOUSMEMORY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_BlockOnResource",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_BLOCKONRESOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_BlockOnTime",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_BLOCKONTIME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CheckDeviceFormat",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_CHECKDEVICEFORMAT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_ClearStateBlockFlags",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_CLEARSTATEBLOCKFLAGS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CommonSetDebugRegisters",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_COMMONSETDEBUGREGISTERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_CreateTexture",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_CREATETEXTURE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_DestroyResource",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_DESTROYRESOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_EnumAdapterModes",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_ENUMADAPTERMODES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_GetAdapterDisplayMode",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_GETADAPTERDISPLAYMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_GetAdapterIdentifier",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_GETADAPTERIDENTIFIER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_GetAdapterModeCount",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_GETADAPTERMODECOUNT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_GetDeviceCaps",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_GETDEVICECAPS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_KickOffAndWaitForIdle",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_KICKOFFANDWAITFORIDLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_LazySetPointParams",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_LAZYSETPOINTPARAMS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_RecordStateBlock",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_RECORDSTATEBLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetFence",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_SETFENCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3D_SetPushBufferSize",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_D3D_SETPUSHBUFFERSIZE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "Direct3D_CheckDeviceMultiSampleType",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_DIRECT3D_CHECKDEVICEMULTISAMPLETYPE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "Get2DSurfaceDesc",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_GET2DSURFACEDESC_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "Lock2DSurface",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_LOCK2DSURFACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "Lock3DSurface",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_LOCK3DSURFACE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMETAL_StartPush",
        min_version: 3911,
        source_file: "D3D8/3911.inl",
        xrefs: D3D8_XMETAL_STARTPUSH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "D3DDevice_SetRenderState_MultiSampleType",
        min_version: 3925,
        source_file: "D3D8/3925.inl",
        xrefs: D3D8_D3DDEVICE_SETRENDERSTATE_MULTISAMPLETYPE_3925_XREFS,
    },
];
