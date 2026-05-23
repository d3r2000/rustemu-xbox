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

// Source: Xapi/3911.inl
static XAPI_CLOSEHANDLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000A, 0x85),
    ov!(0x000B, 0xC0),
    ov!(0x000E, 0x33),
    ov!(0x0012, 0x08),
    ov!(0x0019, 0x33),
    ov!(0x001A, 0xC0),
];
static XAPI_CLOSEHANDLE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_CONVERTTHREADTOFIBER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x64),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x0D),
    ov!(0x0024, 0x83),
    ov!(0x0025, 0x61),
    ov!(0x0026, 0x08),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x89),
    ov!(0x0029, 0x41),
    ov!(0x002A, 0x04),
    ov!(0x0033, 0xC2),
    ov!(0x0034, 0x04),
];
static XAPI_CONVERTTHREADTOFIBER_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "_tls_index",
    },
    OovpaXref {
        offset: 0x0008,
        target: "_tls_array",
    },
    OovpaXref {
        offset: 0x0015,
        target: "XapiThreadFiberData_OFFSET",
    },
    OovpaXref {
        offset: 0x002D,
        target: "XapiCurrentFiber_OFFSET",
    },
];

// Source: Xapi/3911.inl
static XAPI_CREATEEVENTA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0031, 0xFF),
    ov!(0x0032, 0x15),
    ov!(0x0042, 0x68),
    ov!(0x0043, 0xB7),
    ov!(0x0044, 0x00),
    ov!(0x0045, 0x00),
    ov!(0x0056, 0xE8),
    ov!(0x005E, 0xC2),
    ov!(0x005F, 0x10),
];
static XAPI_CREATEEVENTA_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0033,
        target: "KT_FUNC_NtCreateEvent",
    },
    OovpaXref {
        offset: 0x0057,
        target: "XapiSetLastNTError",
    },
];

// Source: Xapi/3911.inl
static XAPI_CREATEFIBER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x56),
    ov!(0x0005, 0x57),
    ov!(0x0006, 0x33),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x3B),
    ov!(0x0009, 0xC7),
    ov!(0x000A, 0x75),
    ov!(0x000B, 0x05),
    ov!(0x000C, 0xA1),
    ov!(0x000D, 0x30),
    ov!(0x000E, 0x01),
    ov!(0x000F, 0x01),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0xB9),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x30),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x3B),
    ov!(0x0017, 0xC1),
    ov!(0x0018, 0x73),
    ov!(0x0019, 0x02),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0xC1),
    ov!(0x001C, 0x8D),
    ov!(0x001D, 0xB0),
    ov!(0x001E, 0xFF),
    ov!(0x001F, 0x0F),
];
static XAPI_CREATEFIBER_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_CREATEMUTEXA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0039, 0x68),
    ov!(0x003A, 0xB7),
    ov!(0x003B, 0x00),
    ov!(0x0047, 0x8B),
    ov!(0x0048, 0x45),
    ov!(0x0049, 0x10),
];
static XAPI_CREATEMUTEXA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0018,
    target: "XapiFormatObjectAttributes",
}];

// Source: Xapi/3911.inl
static XAPI_CREATETHREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x000A, 0xA1),
    ov!(0x000B, 0x30),
    ov!(0x000C, 0x01),
    ov!(0x000D, 0x01),
    ov!(0x000E, 0x00),
    ov!(0x0012, 0x68),
    ov!(0x001C, 0x81),
    ov!(0x001D, 0xE1),
    ov!(0x001E, 0x01),
    ov!(0x001F, 0xFF),
    ov!(0x0020, 0xFF),
];
static XAPI_CREATETHREAD_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0013,
    target: "XapiThreadStartup",
}];

// Source: Xapi/3911.inl
static XAPI_DELETEFIBER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x70),
    ov!(0x0006, 0x08),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x70),
    ov!(0x0009, 0x04),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x15),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x00),
];
static XAPI_DELETEFIBER_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_EXITTHREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x00),
    ov!(0x0002, 0xE8),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x74),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x04),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0x15),
    ov!(0x0011, 0xCC),
];
static XAPI_EXITTHREAD_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0003,
        target: "XapiCallThreadNotifyRoutines",
    },
    OovpaXref {
        offset: 0x000D,
        target: "KT_FUNC_PsTerminateSystemThread",
    },
];

// Source: Xapi/3911.inl
static XAPI_GETEXITCODETHREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x8D),
    ov!(0x0004, 0x45),
    ov!(0x0005, 0x08),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0x4D),
    ov!(0x001C, 0x08),
    ov!(0x002B, 0xB8),
    ov!(0x002C, 0x03),
    ov!(0x002D, 0x01),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x08),
];
static XAPI_GETEXITCODETHREAD_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_GETLASTERROR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x64),
    ov!(0x0001, 0x0F),
    ov!(0x0002, 0xB6),
    ov!(0x0008, 0x3C),
    ov!(0x0009, 0x02),
    ov!(0x0021, 0x8B),
    ov!(0x0022, 0x80),
    ov!(0x0027, 0xC3),
];
static XAPI_GETLASTERROR_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0013,
        target: "_tls_index",
    },
    OovpaXref {
        offset: 0x001A,
        target: "_tls_array",
    },
    OovpaXref {
        offset: 0x0023,
        target: "XapiLastErrorCode_OFFSET",
    },
];

// Source: Xapi/3911.inl
static XAPI_GETOVERLAPPEDRESULT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0x75),
    ov!(0x0018, 0xC0),
    ov!(0x0027, 0xEB),
    ov!(0x0032, 0x00),
    ov!(0x003F, 0xEB),
    ov!(0x004C, 0x89),
    ov!(0x0059, 0x56),
];
static XAPI_GETOVERLAPPEDRESULT_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_GETTHREADPRIORITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0xFF),
    ov!(0x000E, 0x75),
    ov!(0x000F, 0x08),
    ov!(0x0018, 0x7C),
    ov!(0x0019, 0x2B),
    ov!(0x002F, 0x83),
    ov!(0x0030, 0xFE),
    ov!(0x0031, 0xF0),
    ov!(0x0037, 0x8B),
    ov!(0x0038, 0x4D),
];
static XAPI_GETTHREADPRIORITY_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_GETTIMEZONEINFORMATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x28),
    ov!(0x002E, 0x28),
    ov!(0x004F, 0x59),
    ov!(0x0056, 0xAB),
    ov!(0x008C, 0xC0),
    ov!(0x00B9, 0x80),
    ov!(0x00F7, 0x99),
];
static XAPI_GETTIMEZONEINFORMATION_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_MU_INIT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x10),
    ov!(0x0095, 0x50),
    ov!(0x0096, 0x6A),
    ov!(0x0097, 0x00),
    ov!(0x0098, 0x6A),
    ov!(0x0099, 0x3A),
    ov!(0x009A, 0x8D),
    ov!(0x009B, 0x45),
    ov!(0x009C, 0xF0),
];
static XAPI_MU_INIT_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_MOVEFILEA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0017, 0x6A),
    ov!(0x0018, 0xFD),
    ov!(0x005F, 0x6A),
    ov!(0x0060, 0x0A),
    ov!(0x0061, 0x6A),
    ov!(0x0062, 0x10),
    ov!(0x0093, 0xC2),
    ov!(0x0094, 0x08),
];
static XAPI_MOVEFILEA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x008A,
    target: "XapiSetLastNTError",
}];

// Source: Xapi/3911.inl
static XAPI_OPENEVENTA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x000C, 0x68),
    ov!(0x000D, 0x0D),
    ov!(0x000E, 0x00),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0xC0),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x35),
    ov!(0x0041, 0xFF),
    ov!(0x0042, 0x15),
    ov!(0x004C, 0xE8),
    ov!(0x0059, 0xC2),
    ov!(0x005A, 0x0C),
];
static XAPI_OPENEVENTA_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0032,
        target: "KT_VAR__ExEventObjectType",
    },
    OovpaXref {
        offset: 0x0043,
        target: "KT_FUNC_ObOpenObjectByName",
    },
    OovpaXref {
        offset: 0x004D,
        target: "XapiSetLastNTError",
    },
];

// Source: Xapi/3911.inl
static XAPI_OUTPUTDEBUGSTRINGA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000E, 0x8A),
    ov!(0x000F, 0x10),
    ov!(0x0011, 0x84),
    ov!(0x0012, 0xD2),
    ov!(0x002B, 0xCD),
    ov!(0x002C, 0x2D),
    ov!(0x002D, 0xCC),
    ov!(0x002F, 0xC2),
    ov!(0x0030, 0x04),
];
static XAPI_OUTPUTDEBUGSTRINGA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_OUTPUTDEBUGSTRINGW_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0014, 0x6A),
    ov!(0x0015, 0x01),
    ov!(0x0031, 0xFF),
    ov!(0x0032, 0x75),
    ov!(0x0033, 0xFC),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x04),
];
static XAPI_OUTPUTDEBUGSTRINGW_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "OutputDebugStringA",
}];

// Source: Xapi/3911.inl
static XAPI_PULSEEVENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x00),
    ov!(0x0002, 0xFF),
    ov!(0x0003, 0x74),
    ov!(0x0004, 0x24),
    ov!(0x0005, 0x08),
    ov!(0x0006, 0xFF),
    ov!(0x0007, 0x15),
    ov!(0x0016, 0xE8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x04),
];
static XAPI_PULSEEVENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0008,
        target: "KT_FUNC_NtPulseEvent",
    },
    OovpaXref {
        offset: 0x0017,
        target: "XapiSetLastNTError",
    },
];

// Source: Xapi/3911.inl
static XAPI_RAISEEXCEPTION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x83),
    ov!(0x0014, 0x8B),
    ov!(0x001F, 0xC7),
    ov!(0x002A, 0x10),
    ov!(0x0035, 0x89),
    ov!(0x0040, 0x5F),
    ov!(0x004B, 0xFF),
];
static XAPI_RAISEEXCEPTION_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_READFILEEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x48),
    ov!(0x0014, 0x8D),
    ov!(0x001F, 0xFF),
    ov!(0x002A, 0xFF),
    ov!(0x002F, 0x00),
    ov!(0x0035, 0x00),
    ov!(0x0040, 0x50),
    ov!(0x004B, 0xC0),
];
static XAPI_READFILEEX_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_RESETEVENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x74),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x15),
    ov!(0x0014, 0xE8),
    ov!(0x001B, 0xC2),
    ov!(0x001C, 0x04),
];
static XAPI_RESETEVENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0006,
        target: "KT_FUNC_NtClearEvent",
    },
    OovpaXref {
        offset: 0x0015,
        target: "XapiSetLastNTError",
    },
];

// Source: Xapi/3911.inl
static XAPI_SETEVENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x00),
    ov!(0x0002, 0xFF),
    ov!(0x0003, 0x74),
    ov!(0x0004, 0x24),
    ov!(0x0005, 0x08),
    ov!(0x0006, 0xFF),
    ov!(0x0007, 0x15),
    ov!(0x0016, 0xE8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x04),
];
static XAPI_SETEVENT_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0008,
        target: "KT_FUNC_NtSetEvent",
    },
    OovpaXref {
        offset: 0x0017,
        target: "XapiSetLastNTError",
    },
];

// Source: Xapi/3911.inl
static XAPI_SETLASTERROR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x64),
    ov!(0x0001, 0x0F),
    ov!(0x0002, 0xB6),
    ov!(0x0008, 0x3C),
    ov!(0x0009, 0x02),
    ov!(0x0025, 0x89),
    ov!(0x0026, 0x88),
    ov!(0x002B, 0xC2),
    ov!(0x002C, 0x04),
];
static XAPI_SETLASTERROR_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0015,
        target: "_tls_array",
    },
    OovpaXref {
        offset: 0x001A,
        target: "_tls_index",
    },
    OovpaXref {
        offset: 0x0027,
        target: "XapiLastErrorCode_OFFSET",
    },
];

// Source: Xapi/3911.inl
static XAPI_SETTHREADPRIORITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0022, 0x6A),
    ov!(0x0023, 0x10),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xF8),
    ov!(0x0028, 0xF1),
    ov!(0x002B, 0x6A),
    ov!(0x002C, 0xF0),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static XAPI_SETTHREADPRIORITY_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_SWITCHTOFIBER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x64),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x0D),
    ov!(0x0009, 0x04),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x04),
    ov!(0x0011, 0x55),
    ov!(0x0012, 0x56),
    ov!(0x0013, 0x57),
    ov!(0x0014, 0x53),
    ov!(0x0015, 0x64),
    ov!(0x0016, 0xFF),
    ov!(0x0017, 0x35),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x8B),
    ov!(0x001D, 0x14),
    ov!(0x001E, 0x91),
    ov!(0x001F, 0x8B),
];
static XAPI_SWITCHTOFIBER_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_SWITCHTOTHREAD_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x15),
    ov!(0x0006, 0x33),
    ov!(0x0007, 0xC9),
    ov!(0x0008, 0x3D),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x00),
    ov!(0x000C, 0x40),
    ov!(0x000D, 0x0F),
    ov!(0x000E, 0x95),
    ov!(0x000F, 0xC1),
    ov!(0x0012, 0xC3),
];
static XAPI_SWITCHTOTHREAD_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_UNHANDLEDEXCEPTIONFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0009, 0xFF),
    ov!(0x000A, 0x74),
    ov!(0x000B, 0x24),
    ov!(0x000C, 0x04),
    ov!(0x000D, 0xFF),
    ov!(0x000E, 0xD0),
    ov!(0x000F, 0x83),
    ov!(0x0010, 0xF8),
    ov!(0x0011, 0xFF),
    ov!(0x0014, 0x0B),
    ov!(0x0015, 0xC0),
    ov!(0x0018, 0x33),
    ov!(0x0019, 0xC0),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x04),
];
static XAPI_UNHANDLEDEXCEPTIONFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "g_XapiCurrentTopLevelFilter",
}];

// Source: Xapi/3911.inl
static XAPI_WRITEFILEEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x48),
    ov!(0x0014, 0x8D),
    ov!(0x001F, 0xFF),
    ov!(0x002A, 0xFF),
    ov!(0x002F, 0xFC),
    ov!(0x0035, 0x00),
    ov!(0x0040, 0x50),
    ov!(0x004B, 0xC0),
];
static XAPI_WRITEFILEEX_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XAUTOPOWERDOWNRESETTIMER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0xCD),
    ov!(0x0002, 0x59),
    ov!(0x0008, 0x51),
    ov!(0x0009, 0xB8),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x90),
    ov!(0x000C, 0x65),
    ov!(0x000D, 0xB5),
    ov!(0x000E, 0x50),
    ov!(0x0014, 0xFF),
    ov!(0x001A, 0xC3),
];
static XAPI_XAUTOPOWERDOWNRESETTIMER_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XCALCULATESIGNATUREBEGIN_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x6A),
    ov!(0x0002, 0x7C),
    ov!(0x0003, 0x6A),
    ov!(0x0004, 0x00),
    ov!(0x0010, 0x6A),
    ov!(0x0011, 0x08),
    ov!(0x002E, 0x6A),
    ov!(0x002F, 0x10),
    ov!(0x003B, 0xC2),
    ov!(0x003C, 0x04),
];
static XAPI_XCALCULATESIGNATUREBEGIN_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XFREESECTIONBYHANDLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x74),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000A, 0x85),
    ov!(0x000B, 0xC0),
    ov!(0x0018, 0x33),
    ov!(0x0019, 0xC0),
    ov!(0x001A, 0x40),
    ov!(0x001B, 0xC2),
    ov!(0x001C, 0x04),
];
static XAPI_XFREESECTIONBYHANDLE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XGETDEVICECHANGES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0007, 0x33),
    ov!(0x0008, 0xC0),
    ov!(0x0033, 0xF7),
    ov!(0x0034, 0xD2),
    ov!(0x0042, 0x0B),
    ov!(0x0043, 0xD7),
    ov!(0x0051, 0x8A),
    ov!(0x0052, 0xC8),
];
static XAPI_XGETDEVICECHANGES_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XGETDEVICES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000D, 0x83),
    ov!(0x000E, 0x62),
    ov!(0x000F, 0x04),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x8A),
    ov!(0x0012, 0xC8),
    ov!(0x0016, 0xFF),
    ov!(0x0017, 0x15),
    ov!(0x001F, 0xC2),
    ov!(0x0020, 0x04),
];
static XAPI_XGETDEVICES_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XGETLAUNCHINFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0x8B),
    ov!(0x0018, 0x15),
    ov!(0x0025, 0x8B),
    ov!(0x0032, 0x30),
    ov!(0x003F, 0x00),
    ov!(0x004C, 0x83),
    ov!(0x0059, 0x5E),
];
static XAPI_XGETLAUNCHINFO_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XGETSECTIONHANDLEA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x10),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x1D),
    ov!(0x000D, 0x56),
    ov!(0x000E, 0x57),
    ov!(0x000F, 0xFF),
];
static XAPI_XGETSECTIONHANDLEA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XID_FCLOSEDEVICE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x15),
    ov!(0x003E, 0x8D),
    ov!(0x003F, 0x45),
    ov!(0x0040, 0xF4),
    ov!(0x0041, 0x89),
    ov!(0x0042, 0x45),
];
static XAPI_XID_FCLOSEDEVICE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "KT_FUNC_KeRaiseIrqlToDpcLevel",
}];

// Source: Xapi/3911.inl
static XAPI_XINITDEVICES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x68),
    ov!(0x0004, 0xB4),
    ov!(0x0010, 0x74),
    ov!(0x0011, 0x13),
    ov!(0x005B, 0x0F),
    ov!(0x005C, 0xB6),
    ov!(0x005D, 0x86),
    ov!(0x005E, 0xA1),
    ov!(0x008B, 0xC2),
    ov!(0x008C, 0x08),
];
static XAPI_XINITDEVICES_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XINPUTCLOSE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xE8),
    ov!(0x0009, 0xC2),
    ov!(0x000A, 0x04),
];
static XAPI_XINPUTCLOSE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0005,
    target: "XID_fCloseDevice",
}];

// Source: Xapi/3911.inl
static XAPI_XINPUTGETCAPABILITIES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x15),
    ov!(0x001E, 0x0F),
    ov!(0x001F, 0x84),
    ov!(0x0036, 0x8B),
    ov!(0x0037, 0xFA),
    ov!(0x0038, 0xF3),
    ov!(0x0039, 0xAB),
    ov!(0x003A, 0xAA),
    ov!(0x003B, 0x8A),
    ov!(0x003C, 0x46),
    ov!(0x003D, 0x0B),
    ov!(0x003E, 0x88),
];
static XAPI_XINPUTGETCAPABILITIES_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XINPUTGETSTATE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0x80),
    ov!(0x000F, 0xBA),
    ov!(0x0010, 0xA3),
    ov!(0x0014, 0x01),
    ov!(0x0017, 0x6A),
    ov!(0x0018, 0x57),
    ov!(0x001A, 0xEB),
    ov!(0x001B, 0x46),
    ov!(0x0028, 0xBB),
    ov!(0x0029, 0x8F),
    ov!(0x002A, 0x04),
    ov!(0x006E, 0xC2),
    ov!(0x006F, 0x08),
];
static XAPI_XINPUTGETSTATE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XINPUTOPEN_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0xEB),
    ov!(0x0021, 0x0B),
    ov!(0x0029, 0x75),
    ov!(0x002A, 0x3D),
    ov!(0x004A, 0x83),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x10),
    ov!(0x0066, 0xEB),
    ov!(0x0067, 0x09),
    ov!(0x0068, 0x6A),
    ov!(0x0069, 0x57),
];
static XAPI_XINPUTOPEN_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000B,
        target: "g_DeviceType_Gamepad",
    },
    OovpaXref {
        offset: 0x0018,
        target: "g_DeviceType_Keyboard",
    },
    OovpaXref {
        offset: 0x0025,
        target: "g_DeviceType_IRDongle",
    },
];

// Source: Xapi/3911.inl
static XAPI_XINPUTPOLL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x56),
    ov!(0x0002, 0x33),
    ov!(0x0003, 0xF6),
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x15),
    ov!(0x0018, 0x04),
    ov!(0x0019, 0x02),
    ov!(0x001A, 0x75),
    ov!(0x001B, 0x29),
    ov!(0x001C, 0xF6),
    ov!(0x001D, 0x80),
    ov!(0x001E, 0xA2),
    ov!(0x001F, 0x00),
];
static XAPI_XINPUTPOLL_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XINPUTSETSTATE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8D),
    ov!(0x0005, 0x81),
    ov!(0x0006, 0xA3),
    ov!(0x000F, 0x6A),
    ov!(0x0010, 0x57),
    ov!(0x0012, 0xEB),
    ov!(0x0013, 0x21),
    ov!(0x002D, 0x88),
    ov!(0x002E, 0x42),
    ov!(0x002F, 0x41),
    ov!(0x0035, 0xC2),
    ov!(0x0036, 0x08),
];
static XAPI_XINPUTSETSTATE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XLAUNCHNEWIMAGEA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x80),
    ov!(0x003E, 0xC0),
    ov!(0x005E, 0xFF),
    ov!(0x007E, 0xFC),
    ov!(0x009E, 0x08),
    ov!(0x00BE, 0x50),
    ov!(0x00DE, 0x05),
    ov!(0x00FE, 0x85),
];
static XAPI_XLAUNCHNEWIMAGEA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XLOADSECTIONBYHANDLE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x74),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x000C, 0x85),
    ov!(0x000D, 0xC0),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0x46),
    ov!(0x001C, 0x04),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x04),
];
static XAPI_XLOADSECTIONBYHANDLE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XMUNAMEFROMDRIVELETTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x74),
    ov!(0x0059, 0x68),
    ov!(0x005A, 0x00),
    ov!(0x005B, 0x00),
    ov!(0x005C, 0x10),
    ov!(0x005D, 0x80),
    ov!(0x006A, 0xFF),
    ov!(0x006B, 0x15),
    ov!(0x009E, 0xFF),
    ov!(0x009F, 0x15),
];
static XAPI_XMUNAMEFROMDRIVELETTER_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0014,
        target: "g_XapiMountedMUs",
    },
    OovpaXref {
        offset: 0x006C,
        target: "KT_FUNC_NtOpenFile",
    },
    OovpaXref {
        offset: 0x00A0,
        target: "KT_FUNC_NtFsControlFile",
    },
];

// Source: Xapi/3911.inl
static XAPI_XMUPORTFROMDRIVELETTERA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8A),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x3C),
    ov!(0x0005, 0x46),
    ov!(0x0012, 0x99),
    ov!(0x0015, 0xD1),
    ov!(0x0016, 0xF8),
    ov!(0x001C, 0xC2),
    ov!(0x001D, 0x04),
];
static XAPI_XMUPORTFROMDRIVELETTERA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XMUSLOTFROMDRIVELETTERA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8A),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x3C),
    ov!(0x0005, 0x46),
    ov!(0x0012, 0x99),
    ov!(0x0016, 0xF7),
    ov!(0x0017, 0xF9),
    ov!(0x001F, 0xC2),
    ov!(0x0020, 0x04),
];
static XAPI_XMUSLOTFROMDRIVELETTERA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XMUWRITENAMETODRIVELETTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x74),
    ov!(0x0059, 0x68),
    ov!(0x005A, 0x00),
    ov!(0x005B, 0x00),
    ov!(0x005C, 0x10),
    ov!(0x005D, 0x40),
    ov!(0x006A, 0xFF),
    ov!(0x006B, 0x15),
    ov!(0x00AC, 0xFF),
    ov!(0x00AD, 0x15),
];
static XAPI_XMUWRITENAMETODRIVELETTER_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0014,
        target: "g_XapiMountedMUs",
    },
    OovpaXref {
        offset: 0x006C,
        target: "KT_FUNC_NtOpenFile",
    },
    OovpaXref {
        offset: 0x00AE,
        target: "KT_FUNC_NtFsControlFile",
    },
];

// Source: Xapi/3911.inl
static XAPI_XMOUNTALTERNATETITLEA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xEC),
    ov!(0x000F, 0x18),
    ov!(0x0010, 0x01),
    ov!(0x0011, 0x01),
    ov!(0x0012, 0x00),
    ov!(0x0013, 0x53),
    ov!(0x0014, 0x8A),
    ov!(0x0030, 0x39),
    ov!(0x0031, 0x55),
    ov!(0x0032, 0x0C),
    ov!(0x0033, 0x74),
    ov!(0x0034, 0x09),
    ov!(0x003D, 0xEC),
];
static XAPI_XMOUNTALTERNATETITLEA_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XMOUNTMUA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x0C),
    ov!(0x003E, 0x66),
    ov!(0x0061, 0x85),
    ov!(0x007E, 0x8D),
    ov!(0x00A2, 0x0F),
    ov!(0x00BE, 0x50),
    ov!(0x00DE, 0x74),
];
static XAPI_XMOUNTMUA_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x002E,
        target: "g_XapiMountedMUs",
    },
    OovpaXref {
        offset: 0x00CE,
        target: "XapiMapLetterToDirectory",
    },
];

// Source: Xapi/3911.inl
static XAPI_XMOUNTMUROOTA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0x0C),
    ov!(0x003E, 0x00),
    ov!(0x0061, 0x8B),
    ov!(0x007E, 0x00),
    ov!(0x009E, 0x00),
    ov!(0x00BE, 0xFF),
    ov!(0x00DE, 0xFF),
];
static XAPI_XMOUNTMUROOTA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "g_XapiMountedMUs",
}];

// Source: Xapi/3911.inl
static XAPI_XMOUNTUTILITYDRIVE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x81),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0006, 0x01),
    ov!(0x0009, 0x53),
    ov!(0x000A, 0x56),
    ov!(0x000B, 0x57),
    ov!(0x0013, 0x50),
    ov!(0x0014, 0xFF),
    ov!(0x0015, 0x75),
    ov!(0x0016, 0x08),
    ov!(0x0017, 0xE8),
];
static XAPI_XMOUNTUTILITYDRIVE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0018,
    target: "XapiSelectCachePartition",
}];

// Source: Xapi/3911.inl
static XAPI_XREGISTERTHREADNOTIFYROUTINE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x83),
    ov!(0x000E, 0x7C),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x0C),
    ov!(0x0012, 0x74),
    ov!(0x0013, 0x1C),
    ov!(0x0024, 0x89),
    ov!(0x0025, 0x48),
    ov!(0x0026, 0x04),
    ov!(0x0046, 0xC2),
    ov!(0x0047, 0x08),
];
static XAPI_XREGISTERTHREADNOTIFYROUTINE_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XUNMOUNTALTERNATETITLEA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x65),
    ov!(0x0016, 0xFF),
    ov!(0x0023, 0x83),
    ov!(0x002E, 0x45),
    ov!(0x003B, 0xFF),
    ov!(0x003C, 0x15),
    ov!(0x0046, 0x0B),
    ov!(0x0052, 0x50),
    ov!(0x0059, 0xC9),
    ov!(0x005A, 0xC2),
    ov!(0x005B, 0x04),
];
static XAPI_XUNMOUNTALTERNATETITLEA_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x003D,
        target: "KT_FUNC_IoDeleteSymbolicLink",
    },
    OovpaXref {
        offset: 0x004D,
        target: "g_XapiAltLett_MU",
    },
];

// Source: Xapi/3911.inl
static XAPI_XUNMOUNTMU_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x005D, 0x6A),
    ov!(0x005E, 0x20),
    ov!(0x005F, 0x6A),
    ov!(0x0060, 0x01),
    ov!(0x0061, 0x33),
    ov!(0x0062, 0xFF),
];
static XAPI_XUNMOUNTMU_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x001F,
        target: "g_XapiMountedMUs",
    },
    OovpaXref {
        offset: 0x0038,
        target: "XUnmountAlternateTitleA",
    },
];

// Source: Xapi/3911.inl
static XAPI_XAPIBOOTTODASH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x81),
    ov!(0x0004, 0xEC),
    ov!(0x0006, 0x0C),
    ov!(0x0009, 0xA1),
    ov!(0x000A, 0x18),
    ov!(0x000B, 0x01),
    ov!(0x000C, 0x01),
    ov!(0x0025, 0xF3),
    ov!(0x0026, 0xAB),
    ov!(0x0059, 0xC2),
    ov!(0x005A, 0x0C),
];
static XAPI_XAPIBOOTTODASH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0053,
    target: "XLaunchNewImageA",
}];

// Source: Xapi/3911.inl
static XAPI_XAPICALLTHREADNOTIFYROUTINES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x56),
    ov!(0x0002, 0x57),
    ov!(0x0003, 0xBB),
    ov!(0x0024, 0xFF),
    ov!(0x0025, 0x50),
    ov!(0x0026, 0x08),
    ov!(0x0032, 0x5F),
    ov!(0x0033, 0x5E),
    ov!(0x0034, 0x5B),
    ov!(0x0035, 0xC2),
    ov!(0x0036, 0x04),
];
static XAPI_XAPICALLTHREADNOTIFYROUTINES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0016,
    target: "XapiThreadNotifyRoutineList",
}];

// Source: Xapi/3911.inl
static XAPI_XAPIFIBERSTARTUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x08),
    ov!(0x0025, 0xFF),
    ov!(0x0026, 0x30),
    ov!(0x0027, 0xFF),
    ov!(0x0028, 0x55),
    ov!(0x0029, 0x08),
    ov!(0x002A, 0xEB),
    ov!(0x002B, 0x0C),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x75),
    ov!(0x002E, 0xEC),
    ov!(0x002F, 0xE8),
    ov!(0x0044, 0xCC),
];
static XAPI_XAPIFIBERSTARTUP_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XAPIFORMATOBJECTATTRIBUTES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x15),
    ov!(0x0017, 0xC7),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0xFC),
    ov!(0x001A, 0xFF),
    ov!(0x001B, 0xFF),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x0C),
];
static XAPI_XAPIFORMATOBJECTATTRIBUTES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "KT_FUNC_RtlInitAnsiString",
}];

// Source: Xapi/3911.inl
static XAPI_XAPIINITPROCESS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x30),
    ov!(0x000F, 0x6A),
    ov!(0x0010, 0x0C),
    ov!(0x0017, 0xF3),
    ov!(0x0018, 0xAB),
    ov!(0x0042, 0x75),
    ov!(0x0043, 0x0A),
];
static XAPI_XAPIINITPROCESS_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_XAPIMAPLETTERTODIRECTORY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x81),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x84),
    ov!(0x0006, 0x02),
    ov!(0x0007, 0x00),
    ov!(0x0242, 0xE8),
];
static XAPI_XAPIMAPLETTERTODIRECTORY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0243,
    target: "XGetSectionSize",
}];

// Source: Xapi/3911.inl
static XAPI_XAPISELECTCACHEPARTITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0012, 0x6A),
    ov!(0x0013, 0x10),
    ov!(0x0017, 0x6A),
    ov!(0x0018, 0x03),
    ov!(0x0019, 0x8D),
    ov!(0x001A, 0x45),
    ov!(0x001B, 0xE0),
    ov!(0x001D, 0x8D),
    ov!(0x001E, 0x45),
    ov!(0x001F, 0xD4),
    ov!(0x003D, 0xFF),
    ov!(0x003E, 0x15),
    ov!(0x0072, 0xFF),
    ov!(0x0073, 0x15),
];
static XAPI_XAPISELECTCACHEPARTITION_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x003F,
        target: "KT_FUNC_NtOpenFile",
    },
    OovpaXref {
        offset: 0x0074,
        target: "KT_FUNC_NtReadFile",
    },
];

// Source: Xapi/3911.inl
static XAPI_XAPISETLASTNTERROR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x74),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0xD0),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0xC2),
    ov!(0x0014, 0xC2),
    ov!(0x0015, 0x04),
];
static XAPI_XAPISETLASTNTERROR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000E,
    target: "SetLastError",
}];

// Source: Xapi/3911.inl
static XAPI_XAPITHREADSTARTUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x6A),
    ov!(0x0001, 0x18),
    ov!(0x0002, 0x68),
    ov!(0x003C, 0xC1),
    ov!(0x003D, 0xE9),
    ov!(0x003E, 0x02),
    ov!(0x0043, 0x83),
    ov!(0x0044, 0xE1),
    ov!(0x0045, 0x03),
    ov!(0x0086, 0xC3),
    ov!(0x0097, 0xCC),
];
static XAPI_XAPITHREADSTARTUP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0068,
        target: "XapiCallThreadNotifyRoutines",
    },
    OovpaXref {
        offset: 0x0082,
        target: "UnhandledExceptionFilter",
    },
];

// Source: Xapi/3911.inl
static XAPI__CINIT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x000D, 0xB8),
    ov!(0x0012, 0xBF),
    ov!(0x0023, 0x83),
    ov!(0x0024, 0xF8),
    ov!(0x0025, 0xFF),
    ov!(0x0047, 0x83),
    ov!(0x0048, 0xF8),
    ov!(0x0049, 0xFF),
    ov!(0x004C, 0xFF),
    ov!(0x004D, 0xD0),
    ov!(0x0057, 0xC3),
];
static XAPI__CINIT_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI__RTINIT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0002, 0xB8),
    ov!(0x0007, 0xBF),
    ov!(0x0018, 0x83),
    ov!(0x0019, 0xF8),
    ov!(0x001A, 0xFF),
    ov!(0x001D, 0xFF),
    ov!(0x001E, 0xD0),
    ov!(0x0028, 0xC3),
];
static XAPI__RTINIT_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_LSTRCMPIW_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x56),
    ov!(0x000F, 0x01),
    ov!(0x0010, 0xE8),
    ov!(0x0019, 0x39),
    ov!(0x0022, 0x0E),
    ov!(0x002D, 0x59),
    ov!(0x0034, 0xEB),
    ov!(0x003D, 0x03),
];
static XAPI_LSTRCMPIW_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_MAINCRTSTARTUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0005, 0x2B),
    ov!(0x0006, 0x05),
    ov!(0x0017, 0x83),
    ov!(0x0018, 0xE0),
    ov!(0x0019, 0xF0),
    ov!(0x001A, 0x6A),
    ov!(0x001B, 0xFC),
    ov!(0x0033, 0x68),
    ov!(0x003C, 0xE8),
    ov!(0x004C, 0xE8),
];
static XAPI_MAINCRTSTARTUP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0034,
        target: "mainXapiStartup",
    },
    OovpaXref {
        offset: 0x003D,
        target: "CreateThread",
    },
    OovpaXref {
        offset: 0x004D,
        target: "XapiBootToDash",
    },
];

// Source: Xapi/3911.inl
static XAPI_MAINXAPISTARTUP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0005, 0x64),
    ov!(0x0006, 0xA1),
    ov!(0x0007, 0x20),
    ov!(0x0008, 0x00),
    ov!(0x0047, 0xE8),
    ov!(0x004C, 0xE8),
];
static XAPI_MAINXAPISTARTUP_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0001,
        target: "XapiInitProcess",
    },
    OovpaXref {
        offset: 0x0048,
        target: "_rtinit",
    },
    OovpaXref {
        offset: 0x004D,
        target: "_cinit",
    },
];

// Source: Xapi/3911.inl
static XAPI_TIMEKILLEVENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0xBF),
    ov!(0x0013, 0x0D),
    ov!(0x0018, 0x0F),
    ov!(0x0019, 0xB7),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x48),
    ov!(0x001C, 0x85),
    ov!(0x001D, 0xC9),
    ov!(0x001E, 0x74),
    ov!(0x001F, 0x3E),
    ov!(0x004A, 0x6A),
    ov!(0x0055, 0x15),
];
static XAPI_TIMEKILLEVENT_3911_XREFS: &[OovpaXref] = &[];

// Source: Xapi/3911.inl
static XAPI_TIMESETEVENT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x14),
    ov!(0x0006, 0x53),
    ov!(0x0007, 0x68),
    ov!(0x001B, 0xCB),
    ov!(0x001C, 0x75),
    ov!(0x002A, 0x45),
    ov!(0x0055, 0x53),
];
static XAPI_TIMESETEVENT_3911_XREFS: &[OovpaXref] = &[];

pub const PATTERNS: &[OovpaPattern] = &[
    OovpaPattern {
        name: "CloseHandle",
        detect_size: 0x001B,
        entries: XAPI_CLOSEHANDLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "ConvertThreadToFiber",
        detect_size: 0x0035,
        entries: XAPI_CONVERTTHREADTOFIBER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CreateEventA",
        detect_size: 0x0060,
        entries: XAPI_CREATEEVENTA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CreateFiber",
        detect_size: 0x0020,
        entries: XAPI_CREATEFIBER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CreateMutexA",
        detect_size: 0x004A,
        entries: XAPI_CREATEMUTEXA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CreateThread",
        detect_size: 0x0021,
        entries: XAPI_CREATETHREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DeleteFiber",
        detect_size: 0x0013,
        entries: XAPI_DELETEFIBER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "ExitThread",
        detect_size: 0x0012,
        entries: XAPI_EXITTHREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "GetExitCodeThread",
        detect_size: 0x004B,
        entries: XAPI_GETEXITCODETHREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "GetLastError",
        detect_size: 0x0028,
        entries: XAPI_GETLASTERROR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "GetOverlappedResult",
        detect_size: 0x005A,
        entries: XAPI_GETOVERLAPPEDRESULT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "GetThreadPriority",
        detect_size: 0x0039,
        entries: XAPI_GETTHREADPRIORITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "GetTimeZoneInformation",
        detect_size: 0x00F8,
        entries: XAPI_GETTIMEZONEINFORMATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "MU_Init",
        detect_size: 0x009D,
        entries: XAPI_MU_INIT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "MoveFileA",
        detect_size: 0x0095,
        entries: XAPI_MOVEFILEA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "OpenEventA",
        detect_size: 0x005B,
        entries: XAPI_OPENEVENTA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "OutputDebugStringA",
        detect_size: 0x0031,
        entries: XAPI_OUTPUTDEBUGSTRINGA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "OutputDebugStringW",
        detect_size: 0x004B,
        entries: XAPI_OUTPUTDEBUGSTRINGW_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "PulseEvent",
        detect_size: 0x001F,
        entries: XAPI_PULSEEVENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "RaiseException",
        detect_size: 0x004C,
        entries: XAPI_RAISEEXCEPTION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "ReadFileEx",
        detect_size: 0x004C,
        entries: XAPI_READFILEEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "ResetEvent",
        detect_size: 0x001D,
        entries: XAPI_RESETEVENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "SetEvent",
        detect_size: 0x001F,
        entries: XAPI_SETEVENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "SetLastError",
        detect_size: 0x002D,
        entries: XAPI_SETLASTERROR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "SetThreadPriority",
        detect_size: 0x0051,
        entries: XAPI_SETTHREADPRIORITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "SwitchToFiber",
        detect_size: 0x0020,
        entries: XAPI_SWITCHTOFIBER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "SwitchToThread",
        detect_size: 0x0013,
        entries: XAPI_SWITCHTOTHREAD_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "UnhandledExceptionFilter",
        detect_size: 0x001C,
        entries: XAPI_UNHANDLEDEXCEPTIONFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "WriteFileEx",
        detect_size: 0x004C,
        entries: XAPI_WRITEFILEEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XAutoPowerDownResetTimer",
        detect_size: 0x001B,
        entries: XAPI_XAUTOPOWERDOWNRESETTIMER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XCalculateSignatureBegin",
        detect_size: 0x003D,
        entries: XAPI_XCALCULATESIGNATUREBEGIN_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XFreeSectionByHandle",
        detect_size: 0x001D,
        entries: XAPI_XFREESECTIONBYHANDLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGetDeviceChanges",
        detect_size: 0x0053,
        entries: XAPI_XGETDEVICECHANGES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGetDevices",
        detect_size: 0x0021,
        entries: XAPI_XGETDEVICES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGetLaunchInfo",
        detect_size: 0x005A,
        entries: XAPI_XGETLAUNCHINFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XGetSectionHandleA",
        detect_size: 0x0010,
        entries: XAPI_XGETSECTIONHANDLEA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XID_fCloseDevice",
        detect_size: 0x0043,
        entries: XAPI_XID_FCLOSEDEVICE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInitDevices",
        detect_size: 0x008D,
        entries: XAPI_XINITDEVICES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputClose",
        detect_size: 0x000B,
        entries: XAPI_XINPUTCLOSE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputGetCapabilities",
        detect_size: 0x003F,
        entries: XAPI_XINPUTGETCAPABILITIES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputGetState",
        detect_size: 0x0070,
        entries: XAPI_XINPUTGETSTATE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputOpen",
        detect_size: 0x006A,
        entries: XAPI_XINPUTOPEN_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputPoll",
        detect_size: 0x0020,
        entries: XAPI_XINPUTPOLL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XInputSetState",
        detect_size: 0x0037,
        entries: XAPI_XINPUTSETSTATE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XLaunchNewImageA",
        detect_size: 0x00FF,
        entries: XAPI_XLAUNCHNEWIMAGEA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XLoadSectionByHandle",
        detect_size: 0x0020,
        entries: XAPI_XLOADSECTIONBYHANDLE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMUNameFromDriveLetter",
        detect_size: 0x00A4,
        entries: XAPI_XMUNAMEFROMDRIVELETTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMUPortFromDriveLetterA",
        detect_size: 0x001E,
        entries: XAPI_XMUPORTFROMDRIVELETTERA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMUSlotFromDriveLetterA",
        detect_size: 0x0021,
        entries: XAPI_XMUSLOTFROMDRIVELETTERA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMUWriteNameToDriveLetter",
        detect_size: 0x00B2,
        entries: XAPI_XMUWRITENAMETODRIVELETTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMountAlternateTitleA",
        detect_size: 0x003E,
        entries: XAPI_XMOUNTALTERNATETITLEA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMountMUA",
        detect_size: 0x00DF,
        entries: XAPI_XMOUNTMUA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMountMURootA",
        detect_size: 0x00DF,
        entries: XAPI_XMOUNTMUROOTA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XMountUtilityDrive",
        detect_size: 0x001C,
        entries: XAPI_XMOUNTUTILITYDRIVE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XRegisterThreadNotifyRoutine",
        detect_size: 0x0048,
        entries: XAPI_XREGISTERTHREADNOTIFYROUTINE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XUnmountAlternateTitleA",
        detect_size: 0x005C,
        entries: XAPI_XUNMOUNTALTERNATETITLEA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XUnmountMU",
        detect_size: 0x0063,
        entries: XAPI_XUNMOUNTMU_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiBootToDash",
        detect_size: 0x005B,
        entries: XAPI_XAPIBOOTTODASH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiCallThreadNotifyRoutines",
        detect_size: 0x0037,
        entries: XAPI_XAPICALLTHREADNOTIFYROUTINES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiFiberStartup",
        detect_size: 0x0045,
        entries: XAPI_XAPIFIBERSTARTUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiFormatObjectAttributes",
        detect_size: 0x0027,
        entries: XAPI_XAPIFORMATOBJECTATTRIBUTES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiInitProcess",
        detect_size: 0x0044,
        entries: XAPI_XAPIINITPROCESS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiMapLetterToDirectory",
        detect_size: 0x0247,
        entries: XAPI_XAPIMAPLETTERTODIRECTORY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiSelectCachePartition",
        detect_size: 0x0078,
        entries: XAPI_XAPISELECTCACHEPARTITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiSetLastNTError",
        detect_size: 0x0016,
        entries: XAPI_XAPISETLASTNTERROR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XapiThreadStartup",
        detect_size: 0x0098,
        entries: XAPI_XAPITHREADSTARTUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "_cinit",
        detect_size: 0x0058,
        entries: XAPI__CINIT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "_rtinit",
        detect_size: 0x0029,
        entries: XAPI__RTINIT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "lstrcmpiW",
        detect_size: 0x003E,
        entries: XAPI_LSTRCMPIW_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "mainCRTStartup",
        detect_size: 0x0051,
        entries: XAPI_MAINCRTSTARTUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "mainXapiStartup",
        detect_size: 0x0051,
        entries: XAPI_MAINXAPISTARTUP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "timeKillEvent",
        detect_size: 0x0056,
        entries: XAPI_TIMEKILLEVENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "timeSetEvent",
        detect_size: 0x0056,
        entries: XAPI_TIMESETEVENT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
];

pub const METADATA: &[OovpaPatternMeta] = &[
    OovpaPatternMeta {
        name: "CloseHandle",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CLOSEHANDLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "ConvertThreadToFiber",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CONVERTTHREADTOFIBER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CreateEventA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CREATEEVENTA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CreateFiber",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CREATEFIBER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CreateMutexA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CREATEMUTEXA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CreateThread",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_CREATETHREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DeleteFiber",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_DELETEFIBER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "ExitThread",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_EXITTHREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "GetExitCodeThread",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_GETEXITCODETHREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "GetLastError",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_GETLASTERROR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "GetOverlappedResult",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_GETOVERLAPPEDRESULT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "GetThreadPriority",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_GETTHREADPRIORITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "GetTimeZoneInformation",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_GETTIMEZONEINFORMATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "MU_Init",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_MU_INIT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "MoveFileA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_MOVEFILEA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "OpenEventA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_OPENEVENTA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "OutputDebugStringA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_OUTPUTDEBUGSTRINGA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "OutputDebugStringW",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_OUTPUTDEBUGSTRINGW_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "PulseEvent",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_PULSEEVENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "RaiseException",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_RAISEEXCEPTION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "ReadFileEx",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_READFILEEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "ResetEvent",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_RESETEVENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "SetEvent",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_SETEVENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "SetLastError",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_SETLASTERROR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "SetThreadPriority",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_SETTHREADPRIORITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "SwitchToFiber",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_SWITCHTOFIBER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "SwitchToThread",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_SWITCHTOTHREAD_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "UnhandledExceptionFilter",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_UNHANDLEDEXCEPTIONFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "WriteFileEx",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_WRITEFILEEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XAutoPowerDownResetTimer",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAUTOPOWERDOWNRESETTIMER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XCalculateSignatureBegin",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XCALCULATESIGNATUREBEGIN_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XFreeSectionByHandle",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XFREESECTIONBYHANDLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGetDeviceChanges",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XGETDEVICECHANGES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGetDevices",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XGETDEVICES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGetLaunchInfo",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XGETLAUNCHINFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XGetSectionHandleA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XGETSECTIONHANDLEA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XID_fCloseDevice",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XID_FCLOSEDEVICE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInitDevices",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINITDEVICES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputClose",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTCLOSE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputGetCapabilities",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTGETCAPABILITIES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputGetState",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTGETSTATE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputOpen",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTOPEN_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputPoll",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTPOLL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XInputSetState",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XINPUTSETSTATE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XLaunchNewImageA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XLAUNCHNEWIMAGEA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XLoadSectionByHandle",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XLOADSECTIONBYHANDLE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMUNameFromDriveLetter",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMUNAMEFROMDRIVELETTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMUPortFromDriveLetterA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMUPORTFROMDRIVELETTERA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMUSlotFromDriveLetterA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMUSLOTFROMDRIVELETTERA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMUWriteNameToDriveLetter",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMUWRITENAMETODRIVELETTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMountAlternateTitleA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMOUNTALTERNATETITLEA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMountMUA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMOUNTMUA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMountMURootA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMOUNTMUROOTA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XMountUtilityDrive",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XMOUNTUTILITYDRIVE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XRegisterThreadNotifyRoutine",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XREGISTERTHREADNOTIFYROUTINE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XUnmountAlternateTitleA",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XUNMOUNTALTERNATETITLEA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XUnmountMU",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XUNMOUNTMU_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiBootToDash",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPIBOOTTODASH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiCallThreadNotifyRoutines",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPICALLTHREADNOTIFYROUTINES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiFiberStartup",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPIFIBERSTARTUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiFormatObjectAttributes",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPIFORMATOBJECTATTRIBUTES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiInitProcess",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPIINITPROCESS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiMapLetterToDirectory",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPIMAPLETTERTODIRECTORY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiSelectCachePartition",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPISELECTCACHEPARTITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiSetLastNTError",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPISETLASTNTERROR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XapiThreadStartup",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_XAPITHREADSTARTUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "_cinit",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI__CINIT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "_rtinit",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI__RTINIT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "lstrcmpiW",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_LSTRCMPIW_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "mainCRTStartup",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_MAINCRTSTARTUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "mainXapiStartup",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_MAINXAPISTARTUP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "timeKillEvent",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_TIMEKILLEVENT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "timeSetEvent",
        min_version: 3911,
        source_file: "Xapi/3911.inl",
        xrefs: XAPI_TIMESETEVENT_3911_XREFS,
    },
];
