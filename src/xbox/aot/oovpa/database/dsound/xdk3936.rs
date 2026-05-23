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

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000C, 0xBB),
    ov!(0x000D, 0x00),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x80),
    ov!(0x001D, 0xEB),
    ov!(0x001E, 0x06),
    ov!(0x003D, 0x80),
    ov!(0x003E, 0x66),
    ov!(0x0040, 0x7F),
    ov!(0x006F, 0xC2),
    ov!(0x0070, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0021, 0x74),
    ov!(0x0022, 0x0B),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CMcpxBuffer_GetCurrentPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x74),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x0013, 0x20),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxBuffer_GetStatus",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0037, 0x8B),
    ov!(0x0038, 0x40),
    ov!(0x0039, 0x4C),
    ov!(0x0068, 0x89),
    ov!(0x0069, 0x1F),
    ov!(0x006A, 0x74),
    ov!(0x006B, 0x1F),
    ov!(0x0085, 0x83),
    ov!(0x0086, 0x22),
    ov!(0x0087, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CDirectSoundBuffer_GetCurrentPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0xFF),
    ov!(0x000C, 0xF0),
    ov!(0x0013, 0x24),
    ov!(0x001A, 0x85),
    ov!(0x0025, 0xFF),
    ov!(0x002B, 0x8B),
    ov!(0x002F, 0xC2),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0016,
    target: "CDirectSoundBuffer_PlayEx",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0xF0),
    ov!(0x0013, 0x24),
    ov!(0x0017, 0x24),
    ov!(0x001B, 0x24),
    ov!(0x0025, 0x85),
    ov!(0x0027, 0x8B),
    ov!(0x003A, 0xC2),
    ov!(0x003B, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CMcpxBuffer_Play",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0022, 0x3B),
    ov!(0x0023, 0x48),
    ov!(0x0024, 0x4C),
    ov!(0x0038, 0x8B),
    ov!(0x0039, 0x4E),
    ov!(0x003A, 0x1C),
    ov!(0x0071, 0xC9),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x003D,
        target: "CDirectSoundBufferSettings_SetBufferData",
    },
    OovpaXref {
        offset: 0x0055,
        target: "CMcpxBuffer_SetBufferData",
    },
];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0xD9),
    ov!(0x000D, 0x5C),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x08),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x14),
    ov!(0x0028, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x0013, 0x20),
    ov!(0x001D, 0x74),
    ov!(0x001E, 0x0B),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxBuffer_SetCurrentPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0x3B),
    ov!(0x0021, 0x51),
    ov!(0x0022, 0x54),
    ov!(0x002C, 0x8B),
    ov!(0x002D, 0x4E),
    ov!(0x002E, 0x20),
    ov!(0x0038, 0x74),
    ov!(0x0039, 0x0B),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x74),
    ov!(0x0004, 0xD9),
    ov!(0x0007, 0x0C),
    ov!(0x000A, 0x1C),
    ov!(0x000D, 0x74),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x74),
    ov!(0x0004, 0xD9),
    ov!(0x0007, 0x0C),
    ov!(0x000A, 0x1C),
    ov!(0x000D, 0x74),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x24),
    ov!(0x0014, 0x24),
    ov!(0x001F, 0x74),
    ov!(0x002A, 0x50),
    ov!(0x0035, 0xF6),
    ov!(0x0040, 0x07),
    ov!(0x004D, 0x5F),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x74),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x000D, 0xFF),
    ov!(0x000E, 0x74),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0029, 0x5F),
    ov!(0x002B, 0xC2),
    ov!(0x002C, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0012,
    target: "CDirectSoundVoice_SetOutputBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0027, 0x14),
    ov!(0x0028, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0027, 0x14),
    ov!(0x0028, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x6A),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x6A),
    ov!(0x000A, 0x00),
    ov!(0x000B, 0x6A),
    ov!(0x000C, 0x00),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0xD8),
    ov!(0x001C, 0x74),
    ov!(0x001D, 0x0B),
    ov!(0x002D, 0xC2),
    ov!(0x002E, 0x04),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOP_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0014,
    target: "CDirectSoundBuffer_StopEx",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0xA8),
    ov!(0x0010, 0x03),
    ov!(0x0015, 0x83),
    ov!(0x0016, 0xE0),
    ov!(0x0017, 0x01),
    ov!(0x0032, 0xFF),
    ov!(0x0033, 0x75),
    ov!(0x0034, 0x0C),
    ov!(0x0053, 0xC2),
    ov!(0x0054, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CMcpxBuffer_Stop",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0xC0),
    ov!(0x0006, 0x04),
    ov!(0x0007, 0x50),
    ov!(0x0008, 0xE8),
    ov!(0x000D, 0xC2),
    ov!(0x000E, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "DSound_CRefCount_AddRef",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_CONSTRUCTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xF1),
    ov!(0x0003, 0x57),
    ov!(0x0008, 0x8D),
    ov!(0x0009, 0x7E),
    ov!(0x000A, 0x04),
    ov!(0x0012, 0xC7),
    ov!(0x0013, 0x07),
    ov!(0x0019, 0xC7),
    ov!(0x001A, 0x06),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_CONSTRUCTOR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0x8B),
    ov!(0x000C, 0x0C),
    ov!(0x000D, 0x8B),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0xE8),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CMcpxStream_Discontinuity",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0x8B),
    ov!(0x000C, 0x0C),
    ov!(0x000D, 0x8B),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0xE8),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0x7C),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x10),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x4F),
    ov!(0x0018, 0x24),
    ov!(0x0023, 0x8B),
    ov!(0x0024, 0x47),
    ov!(0x0025, 0x20),
    ov!(0x002D, 0x83),
    ov!(0x002E, 0x66),
    ov!(0x002F, 0x08),
    ov!(0x0030, 0x00),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0008, 0x8B),
    ov!(0x000B, 0x08),
    ov!(0x000C, 0x8B),
    ov!(0x000E, 0x24),
    ov!(0x0014, 0x8B),
    ov!(0x0017, 0x0C),
    ov!(0x0018, 0xF7),
    ov!(0x001A, 0x1B),
    ov!(0x001C, 0xF7),
    ov!(0x0020, 0x89),
    ov!(0x0021, 0x01),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0xFF),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x24),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "CMcpxStream_Pause",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_PROCESS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0x8B),
    ov!(0x0007, 0x24),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xC0),
    ov!(0x0012, 0x32),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x78),
    ov!(0x0015, 0x88),
    ov!(0x0018, 0xFF),
    ov!(0x001B, 0x0C),
    ov!(0x001C, 0x8B),
    ov!(0x001E, 0x24),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_PROCESS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0xC0),
    ov!(0x0006, 0x04),
    ov!(0x0007, 0x50),
    ov!(0x0008, 0xE8),
    ov!(0x000D, 0xC2),
    ov!(0x000E, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "DSound_CRefCount_Release",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xC0),
    ov!(0x000E, 0x04),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xC0),
    ov!(0x000E, 0x04),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x10),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x0C),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0025,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xC0),
    ov!(0x000E, 0x04),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x74),
    ov!(0x0004, 0x8B),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x24),
    ov!(0x000D, 0xC0),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000D, 0x83),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x04),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000D, 0x83),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x04),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x74),
    ov!(0x0004, 0x8B),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x24),
    ov!(0x000D, 0xC0),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xC0),
    ov!(0x000E, 0x04),
    ov!(0x000F, 0x50),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x74),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x10),
    ov!(0x001A, 0xFF),
    ov!(0x001B, 0x74),
    ov!(0x001C, 0x24),
    ov!(0x001D, 0x14),
    ov!(0x003D, 0xC2),
    ov!(0x003E, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CDirectSoundVoice_SetOutputBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x0C),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0025,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x0C),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0025,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x44),
    ov!(0x000B, 0x24),
    ov!(0x000C, 0x14),
    ov!(0x000D, 0xF7),
    ov!(0x000E, 0xD0),
    ov!(0x000F, 0x83),
    ov!(0x0010, 0xE0),
    ov!(0x0011, 0x01),
    ov!(0x0038, 0xC2),
    ov!(0x0039, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CMcpxVoiceClient_Set3dParameters",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x000E, 0xD0),
    ov!(0x0016, 0x18),
    ov!(0x001E, 0x18),
    ov!(0x0027, 0x85),
    ov!(0x0032, 0xFF),
    ov!(0x0038, 0x8B),
    ov!(0x003E, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0x8B),
    ov!(0x0022, 0x45),
    ov!(0x0023, 0x18),
    ov!(0x0024, 0xF7),
    ov!(0x0025, 0xD0),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0x01),
    ov!(0x0031, 0x8B),
    ov!(0x0032, 0x48),
    ov!(0x0033, 0x14),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxVoiceClient_Set3dConeOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x44),
    ov!(0x000B, 0x24),
    ov!(0x000C, 0x14),
    ov!(0x000D, 0xF7),
    ov!(0x000E, 0xD0),
    ov!(0x000F, 0x83),
    ov!(0x0010, 0xE0),
    ov!(0x0011, 0x01),
    ov!(0x0038, 0xC2),
    ov!(0x0039, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CMcpxVoiceClient_Set3dConeOutsideVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETEG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x57),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETEG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxVoiceClient_SetEG",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x57),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxVoiceClient_SetFilter",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0015, 0x8B),
    ov!(0x0016, 0x46),
    ov!(0x0017, 0x18),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x40),
    ov!(0x001A, 0x10),
    ov!(0x0040, 0xC2),
    ov!(0x0041, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0020,
    target: "XAudioCalculatePitch",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x0C),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x42),
    ov!(0x0013, 0x18),
    ov!(0x003C, 0xC2),
    ov!(0x003D, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x000E, 0xD0),
    ov!(0x0016, 0x10),
    ov!(0x001E, 0xE8),
    ov!(0x0026, 0xF8),
    ov!(0x002E, 0xFF),
    ov!(0x0036, 0x5F),
];
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CMcpxVoiceClient_SetI3DL2Source",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x57),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x002E, 0xC2),
    ov!(0x002F, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETLFO_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxVoiceClient_SetLFO",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x14),
    ov!(0x0011, 0xF7),
    ov!(0x0012, 0xD0),
    ov!(0x0013, 0x83),
    ov!(0x0014, 0xE0),
    ov!(0x0015, 0x01),
    ov!(0x001C, 0x8B),
    ov!(0x001D, 0x48),
    ov!(0x001E, 0x14),
    ov!(0x003C, 0xC2),
    ov!(0x003D, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxVoiceClient_Set3dMaxDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x44),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x14),
    ov!(0x0011, 0xF7),
    ov!(0x0012, 0xD0),
    ov!(0x0013, 0x83),
    ov!(0x0014, 0xE0),
    ov!(0x0015, 0x01),
    ov!(0x001C, 0x8B),
    ov!(0x001D, 0x48),
    ov!(0x001E, 0x14),
    ov!(0x003C, 0xC2),
    ov!(0x003D, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxVoiceClient_Set3dMinDistance",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_12_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x65),
    ov!(0x001C, 0x53),
    ov!(0x002B, 0xC0),
    ov!(0x003A, 0xC7),
    ov!(0x0049, 0xFC),
    ov!(0x0058, 0x83),
    ov!(0x0067, 0x15),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_12_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0054,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x4E),
    ov!(0x0013, 0x18),
    ov!(0x0025, 0x8B),
    ov!(0x0026, 0x4E),
    ov!(0x0027, 0x14),
    ov!(0x0055, 0xC2),
    ov!(0x0056, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0029,
    target: "CMcpxVoiceClient_SetMixBins",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0017, 0xFF),
    ov!(0x0018, 0x74),
    ov!(0x0019, 0x24),
    ov!(0x001A, 0x14),
    ov!(0x001B, 0x8B),
    ov!(0x001C, 0x48),
    ov!(0x001D, 0x14),
    ov!(0x0038, 0xC2),
    ov!(0x0039, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CMcpxVoiceClient_Set3dMode",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0xF7),
    ov!(0x0019, 0x8B),
    ov!(0x0025, 0x3B),
    ov!(0x0030, 0xE8),
    ov!(0x0035, 0x8B),
    ov!(0x0046, 0x74),
    ov!(0x0051, 0xD8),
];
static DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x48),
    ov!(0x0013, 0x18),
    ov!(0x0023, 0x74),
    ov!(0x0024, 0x0B),
    ov!(0x0034, 0xC2),
    ov!(0x0035, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPITCH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001B,
    target: "CMcpxVoiceClient_SetPitch",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0x8B),
    ov!(0x0022, 0x45),
    ov!(0x0023, 0x18),
    ov!(0x0024, 0xF7),
    ov!(0x0025, 0xD0),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0x01),
    ov!(0x0031, 0x8B),
    ov!(0x0032, 0x48),
    ov!(0x0033, 0x14),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxVoiceClient_Set3dPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0x8B),
    ov!(0x0022, 0x45),
    ov!(0x0023, 0x18),
    ov!(0x0024, 0xF7),
    ov!(0x0025, 0xD0),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0x01),
    ov!(0x0031, 0x8B),
    ov!(0x0032, 0x48),
    ov!(0x0033, 0x14),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxVoiceClient_Set3dVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x41),
    ov!(0x0013, 0x18),
    ov!(0x001A, 0x8B),
    ov!(0x001B, 0x49),
    ov!(0x001C, 0x14),
    ov!(0x001D, 0xE8),
    ov!(0x0037, 0xC2),
    ov!(0x0038, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001E,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x8B),
    ov!(0x000D, 0x48),
    ov!(0x000E, 0x0C),
    ov!(0x000F, 0x6A),
    ov!(0x0010, 0x00),
    ov!(0x0019, 0x74),
    ov!(0x001A, 0x0B),
    ov!(0x0028, 0xC2),
    ov!(0x0029, 0x04),
];
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0012,
    target: "CMcpxAPU_Commit3dSettings",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0008, 0x8B),
    ov!(0x000B, 0x08),
    ov!(0x000C, 0x8B),
    ov!(0x000E, 0x0C),
    ov!(0x0015, 0x6A),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x6A),
    ov!(0x0018, 0x00),
    ov!(0x0020, 0x5E),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x04),
];
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002D, 0x81),
    ov!(0x002E, 0xE6),
    ov!(0x002F, 0xF2),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0xF8),
    ov!(0x0032, 0x7F),
    ov!(0x0033, 0x81),
    ov!(0x0034, 0xC6),
    ov!(0x0035, 0x0E),
    ov!(0x0036, 0x00),
    ov!(0x0037, 0x07),
    ov!(0x0038, 0x80),
    ov!(0x003C, 0x78),
    ov!(0x003D, 0x21),
    ov!(0x007D, 0xC2),
    ov!(0x007E, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x002D, 0x81),
    ov!(0x002E, 0xE6),
    ov!(0x002F, 0xF2),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0xF8),
    ov!(0x0032, 0x7F),
    ov!(0x0033, 0x81),
    ov!(0x0034, 0xC6),
    ov!(0x0035, 0x0E),
    ov!(0x0036, 0x00),
    ov!(0x0037, 0x07),
    ov!(0x0038, 0x80),
    ov!(0x003C, 0x78),
    ov!(0x003D, 0x16),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_DOWORK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x44),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x08),
    ov!(0x0017, 0x74),
    ov!(0x0018, 0x0B),
    ov!(0x0024, 0xC2),
    ov!(0x0025, 0x04),
];
static DSOUND_CDIRECTSOUND_DOWORK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0010,
    target: "CMcpxAPU_ServiceDeferredCommandsLow",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x75),
    ov!(0x000C, 0x08),
    ov!(0x000D, 0xFF),
    ov!(0x000E, 0x75),
    ov!(0x000F, 0x14),
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x4E),
    ov!(0x0012, 0x08),
    ov!(0x001A, 0xFF),
    ov!(0x001C, 0x18),
    ov!(0x0020, 0xFF),
    ov!(0x0022, 0x10),
    ov!(0x0023, 0xFF),
    ov!(0x0025, 0x0C),
    ov!(0x0041, 0xC2),
    ov!(0x0042, 0x14),
];
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0x0F),
    ov!(0x0026, 0x95),
    ov!(0x0027, 0xC2),
    ov!(0x0038, 0x81),
    ov!(0x0039, 0xCB),
    ov!(0x003A, 0x00),
    ov!(0x003B, 0x00),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0x80),
    ov!(0x0040, 0x81),
    ov!(0x0041, 0xE3),
    ov!(0x0042, 0xFF),
    ov!(0x0043, 0xFF),
    ov!(0x0044, 0xFF),
    ov!(0x0045, 0x7F),
    ov!(0x00CB, 0xC2),
    ov!(0x00CC, 0x08),
];
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_GETCAPS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x8B),
    ov!(0x0012, 0x8D),
    ov!(0x001C, 0x0C),
    ov!(0x0026, 0xFF),
    ov!(0x0034, 0x03),
    ov!(0x003A, 0xDB),
    ov!(0x0048, 0x8B),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUND_GETCAPS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x75),
    ov!(0x000C, 0x18),
    ov!(0x000F, 0xFF),
    ov!(0x0010, 0x75),
    ov!(0x0011, 0x14),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x45),
    ov!(0x0014, 0x08),
    ov!(0x0015, 0xFF),
    ov!(0x0017, 0x10),
    ov!(0x0018, 0x8B),
    ov!(0x001A, 0x0C),
    ov!(0x001B, 0xFF),
    ov!(0x001D, 0x0C),
    ov!(0x003F, 0xC2),
    ov!(0x0040, 0x14),
];
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x08),
    ov!(0x0007, 0x8B),
    ov!(0x0009, 0x08),
    ov!(0x000A, 0x8B),
    ov!(0x000D, 0x08),
    ov!(0x000E, 0x25),
    ov!(0x000F, 0xFF),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0xFF),
    ov!(0x0012, 0x7F),
    ov!(0x0017, 0xC2),
    ov!(0x0018, 0x08),
];
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_GETTIME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x15),
    ov!(0x000A, 0x33),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xC2),
    ov!(0x000D, 0x08),
];
static DSOUND_CDIRECTSOUND_GETTIME_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x8B),
    ov!(0x000E, 0xD0),
    ov!(0x0016, 0x10),
    ov!(0x001E, 0xE8),
    ov!(0x0026, 0xF8),
    ov!(0x002E, 0xFF),
    ov!(0x0036, 0x5F),
];
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CMcpxAPU_Set3dParameters",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0007, 0xD9),
    ov!(0x0008, 0x44),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x0011, 0xF7),
    ov!(0x0012, 0xD0),
    ov!(0x0013, 0x83),
    ov!(0x0014, 0xE0),
    ov!(0x0015, 0x01),
];
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxAPU_Set3dDistanceFactor",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0xD9),
    ov!(0x000E, 0x44),
    ov!(0x0016, 0x50),
    ov!(0x001E, 0x0C),
    ov!(0x0027, 0x85),
    ov!(0x0032, 0xFF),
    ov!(0x0038, 0x8B),
    ov!(0x003E, 0x00),
];
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxAPU_Set3dDopplerFactor",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0x4D),
    ov!(0x000C, 0x1C),
    ov!(0x0013, 0xFF),
    ov!(0x0014, 0x75),
    ov!(0x0015, 0x18),
    ov!(0x0018, 0xFF),
    ov!(0x0019, 0x75),
    ov!(0x001A, 0x14),
    ov!(0x001B, 0x8B),
    ov!(0x001C, 0x45),
    ov!(0x001D, 0x08),
    ov!(0x001E, 0xFF),
    ov!(0x0020, 0x10),
    ov!(0x0021, 0x8B),
    ov!(0x0023, 0x0C),
    ov!(0x0024, 0xFF),
    ov!(0x0026, 0x0C),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x18),
];
static DSOUND_CDIRECTSOUND_SETEFFECTDATA_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0xF8),
    ov!(0x0012, 0x0C),
    ov!(0x001C, 0x18),
    ov!(0x0026, 0x51),
    ov!(0x0033, 0x8B),
    ov!(0x003E, 0xFF),
    ov!(0x0044, 0x5F),
];
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002F,
    target: "CMcpxAPU_SetI3DL2Listener",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0007, 0xFF),
    ov!(0x0008, 0x74),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x14),
    ov!(0x0021, 0x74),
    ov!(0x0022, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CMcpxAPU_SetMixBinHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x8B),
    ov!(0x0011, 0x0C),
    ov!(0x0015, 0x8B),
    ov!(0x0017, 0x10),
    ov!(0x001B, 0x8B),
    ov!(0x001D, 0x14),
    ov!(0x0021, 0x8B),
    ov!(0x0023, 0x18),
    ov!(0x0027, 0x8B),
    ov!(0x0029, 0x1C),
    ov!(0x002D, 0x8B),
    ov!(0x002F, 0x20),
];
static DSOUND_CDIRECTSOUND_SETORIENTATION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x56),
    ov!(0x0007, 0x57),
    ov!(0x0024, 0xF7),
    ov!(0x0025, 0xD0),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0x01),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x14),
];
static DSOUND_CDIRECTSOUND_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxAPU_Set3dPosition",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0007, 0xD9),
    ov!(0x0008, 0x44),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x10),
    ov!(0x0011, 0xF7),
    ov!(0x0012, 0xD0),
    ov!(0x0013, 0x83),
    ov!(0x0014, 0xE0),
    ov!(0x0015, 0x01),
];
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0023,
    target: "CMcpxAPU_Set3dRolloffFactor",
}];

// Source: DSound/3911.inl
static DSOUND_CDIRECTSOUND_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x56),
    ov!(0x0007, 0x57),
    ov!(0x0024, 0xF7),
    ov!(0x0025, 0xD0),
    ov!(0x0026, 0x83),
    ov!(0x0027, 0xE0),
    ov!(0x0028, 0x01),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x14),
];
static DSOUND_CDIRECTSOUND_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxAPU_Set3dVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_COMMIT3DSETTINGS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0056, 0xD9),
    ov!(0x0057, 0x80),
    ov!(0x0058, 0x74),
    ov!(0x0059, 0x01),
    ov!(0x005A, 0x00),
    ov!(0x005B, 0x00),
    ov!(0x00A8, 0xDE),
    ov!(0x00A9, 0xE9),
    ov!(0x00D4, 0xBE),
    ov!(0x00D5, 0x18),
    ov!(0x00D6, 0x01),
];
static DSOUND_CMCPXAPU_COMMIT3DSETTINGS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x55),
    ov!(0x000C, 0xBD),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x02),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0020, 0x8B),
    ov!(0x0021, 0x01),
    ov!(0x0033, 0x83),
    ov!(0x0034, 0xC7),
    ov!(0x0035, 0x04),
    ov!(0x0036, 0x4D),
];
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DDISTANCEFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0xB4),
    ov!(0x0007, 0x01),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x60),
    ov!(0x000B, 0x83),
    ov!(0x000C, 0x7C),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x08),
    ov!(0x0012, 0x78),
    ov!(0x0013, 0x01),
];
static DSOUND_CMCPXAPU_SET3DDISTANCEFACTOR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DDOPPLERFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0xB4),
    ov!(0x0007, 0x01),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x40),
    ov!(0x000B, 0x83),
    ov!(0x000C, 0x7C),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x08),
    ov!(0x0012, 0x80),
    ov!(0x0013, 0x01),
];
static DSOUND_CMCPXAPU_SET3DDOPPLERFACTOR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x57),
    ov!(0x000C, 0xB8),
    ov!(0x0013, 0x80),
    ov!(0x001A, 0x83),
    ov!(0x0021, 0x74),
    ov!(0x002C, 0x33),
    ov!(0x002F, 0x08),
];
static DSOUND_CMCPXAPU_SET3DPARAMETERS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0xA5),
    ov!(0x000D, 0xA5),
    ov!(0x000E, 0xA5),
    ov!(0x000F, 0x80),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0xB4),
    ov!(0x0012, 0x01),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0xFF),
    ov!(0x0028, 0xC2),
    ov!(0x0029, 0x08),
];
static DSOUND_CMCPXAPU_SET3DPOSITION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DROLLOFFFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0xB4),
    ov!(0x0007, 0x01),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x04),
    ov!(0x000B, 0x83),
    ov!(0x000C, 0x7C),
    ov!(0x000D, 0x24),
    ov!(0x000E, 0x08),
    ov!(0x0012, 0x7C),
    ov!(0x0013, 0x01),
];
static DSOUND_CMCPXAPU_SET3DROLLOFFFACTOR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SET3DVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0xA5),
    ov!(0x000D, 0xA5),
    ov!(0x000E, 0xA5),
    ov!(0x000F, 0x83),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0xB4),
    ov!(0x0012, 0x01),
    ov!(0x0013, 0x00),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x40),
    ov!(0x0028, 0xC2),
    ov!(0x0029, 0x08),
];
static DSOUND_CMCPXAPU_SET3DVELOCITY_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SETI3DL2LISTENER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x6A),
    ov!(0x0009, 0x0C),
    ov!(0x000A, 0x59),
    ov!(0x0013, 0x66),
    ov!(0x0014, 0x81),
    ov!(0x0015, 0x88),
    ov!(0x0016, 0xB4),
    ov!(0x0017, 0x01),
    ov!(0x0018, 0x00),
    ov!(0x001A, 0x80),
    ov!(0x001B, 0x01),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x08),
];
static DSOUND_CMCPXAPU_SETI3DL2LISTENER_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXAPU_SETMIXBINHEADROOM_8_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0012, 0x83),
    ov!(0x0013, 0x3D),
    ov!(0x0014, 0x10),
    ov!(0x0015, 0x00),
    ov!(0x0016, 0x82),
    ov!(0x0017, 0xFE),
    ov!(0x0018, 0x04),
    ov!(0x001F, 0x83),
    ov!(0x0020, 0xE2),
    ov!(0x0021, 0x07),
    ov!(0x002D, 0x7C),
    ov!(0x002E, 0xD8),
];
static DSOUND_CMCPXAPU_SETMIXBINHEADROOM_8_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x46),
    ov!(0x0015, 0x83),
    ov!(0x0016, 0xE0),
    ov!(0x0017, 0x03),
    ov!(0x0018, 0x3C),
    ov!(0x0019, 0x03),
    ov!(0x001A, 0x75),
    ov!(0x001B, 0x79),
    ov!(0x008E, 0xF7),
    ov!(0x008F, 0x71),
    ov!(0x0090, 0x4C),
    ov!(0x00C8, 0xC2),
    ov!(0x00C9, 0x08),
    ov!(0x00CA, 0x00),
];
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_GETSTATUS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0010, 0x8B),
    ov!(0x0011, 0x45),
    ov!(0x0012, 0x08),
    ov!(0x0016, 0x33),
    ov!(0x0017, 0xC9),
    ov!(0x0018, 0x41),
    ov!(0x001C, 0x74),
    ov!(0x001D, 0x17),
    ov!(0x002F, 0xC7),
    ov!(0x0030, 0x00),
    ov!(0x0031, 0x05),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static DSOUND_CMCPXBUFFER_GETSTATUS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_PLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x56),
    ov!(0x000E, 0xF1),
    ov!(0x0018, 0x75),
    ov!(0x0019, 0x08),
    ov!(0x001A, 0x6A),
    ov!(0x001B, 0x02),
    ov!(0x0026, 0x75),
    ov!(0x002F, 0x8B),
    ov!(0x0036, 0xC2),
];
static DSOUND_CMCPXBUFFER_PLAY_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_SETBUFFERDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x91),
    ov!(0x0006, 0x33),
    ov!(0x0007, 0xC0),
    ov!(0x0008, 0xF6),
    ov!(0x0009, 0x42),
    ov!(0x000B, 0x04),
    ov!(0x000E, 0xE9),
    ov!(0x0013, 0xC3),
];
static DSOUND_CMCPXBUFFER_SETBUFFERDATA_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0025, 0x8B),
    ov!(0x0026, 0x86),
    ov!(0x0027, 0x48),
    ov!(0x0028, 0x01),
    ov!(0x004A, 0x57),
    ov!(0x004B, 0x6A),
    ov!(0x004C, 0x04),
    ov!(0x0071, 0x0F),
    ov!(0x0072, 0xB7),
    ov!(0x0073, 0x40),
    ov!(0x0074, 0x02),
];
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXBUFFER_STOP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x0B),
    ov!(0x0009, 0x44),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x10),
    ov!(0x0019, 0x6A),
    ov!(0x001A, 0x03),
    ov!(0x0024, 0x8B),
    ov!(0x0025, 0xCE),
    ov!(0x0031, 0xC2),
    ov!(0x0032, 0x08),
];
static DSOUND_CMCPXBUFFER_STOP_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXSTREAM_DISCONTINUITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x03),
    ov!(0x000F, 0x03),
    ov!(0x001B, 0x33),
    ov!(0x001D, 0x8D),
    ov!(0x0025, 0x0F),
    ov!(0x0026, 0x94),
    ov!(0x0027, 0xC1),
    ov!(0x0064, 0x8B),
    ov!(0x0066, 0xE8),
    ov!(0x006E, 0xC9),
    ov!(0x006F, 0xC3),
];
static DSOUND_CMCPXSTREAM_DISCONTINUITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0067,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/3911.inl
static DSOUND_CMCPXSTREAM_FLUSH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x03),
    ov!(0x000F, 0x03),
    ov!(0x0020, 0x33),
    ov!(0x0022, 0x33),
    ov!(0x002D, 0x83),
    ov!(0x002E, 0xFF),
    ov!(0x002F, 0x03),
    ov!(0x004B, 0xE8),
    ov!(0x0058, 0x04),
    ov!(0x0059, 0x40),
    ov!(0x005A, 0x00),
    ov!(0x005B, 0x80),
    ov!(0x0062, 0xE8),
    ov!(0x009D, 0xC9),
    ov!(0x009E, 0xC3),
];
static DSOUND_CMCPXSTREAM_FLUSH_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXSTREAM_PAUSE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x46),
    ov!(0x0019, 0x83),
    ov!(0x001A, 0xC8),
    ov!(0x001B, 0x04),
    ov!(0x0027, 0x83),
    ov!(0x0028, 0xE0),
    ov!(0x0029, 0xFB),
    ov!(0x0044, 0xC9),
    ov!(0x0045, 0xC2),
    ov!(0x0046, 0x04),
];
static DSOUND_CMCPXSTREAM_PAUSE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_4_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0017, 0x09),
    ov!(0x0018, 0x88),
    ov!(0x0019, 0x80),
    ov!(0x001A, 0x00),
    ov!(0x002B, 0x05),
    ov!(0x002C, 0x80),
    ov!(0x002D, 0x00),
    ov!(0x003D, 0x0F),
    ov!(0x003E, 0xB1),
    ov!(0x003F, 0x11),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_4_3911_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0066,
        target: "CMcpxVoiceClient_SetVolume",
    },
    OovpaXref {
        offset: 0x0084,
        target: "CMcpxVoiceClient_SetFilter",
    },
];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x83),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x80),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x18),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x07),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0x80),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x10),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0x41),
    ov!(0x0012, 0x4C),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0x80),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x04),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0x41),
    ov!(0x0012, 0x54),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0x80),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x04),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0x41),
    ov!(0x0012, 0x50),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DMINDISTANCE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x80),
    ov!(0x0005, 0x89),
    ov!(0x0006, 0x80),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0xFF),
    ov!(0x0010, 0x89),
    ov!(0x0011, 0x41),
    ov!(0x0012, 0x58),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DMODE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x74),
    ov!(0x0008, 0x6A),
    ov!(0x0009, 0x10),
    ov!(0x000A, 0x59),
    ov!(0x000B, 0x8D),
    ov!(0x000C, 0x78),
    ov!(0x000D, 0x1C),
    ov!(0x000E, 0xF3),
    ov!(0x0014, 0x00),
    ov!(0x0018, 0x7C),
    ov!(0x001F, 0x09),
];
static DSOUND_CMCPXVOICECLIENT_SET3DPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0025,
    target: "CMcpxVoiceClient_Commit3dSettings",
}];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x80),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x80),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0xFF),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x07),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DPOSITION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SET3DVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0xA5),
    ov!(0x000A, 0xA5),
    ov!(0x000B, 0xA5),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x80),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x00),
    ov!(0x0012, 0x40),
    ov!(0x0025, 0xC2),
    ov!(0x0026, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SET3DVELOCITY_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETEG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0010, 0x8B),
    ov!(0x0021, 0xEB),
    ov!(0x006B, 0x82),
    ov!(0x006C, 0xFE),
    ov!(0x0095, 0x8B),
    ov!(0x0096, 0x96),
    ov!(0x0097, 0x8C),
    ov!(0x0098, 0x00),
    ov!(0x0099, 0x00),
    ov!(0x009A, 0x00),
    ov!(0x009B, 0x89),
    ov!(0x009C, 0x15),
];
static DSOUND_CMCPXVOICECLIENT_SETEG_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0xE8),
    ov!(0x001F, 0x6A),
    ov!(0x0073, 0x00),
    ov!(0x0074, 0xF6),
    ov!(0x0075, 0x47),
    ov!(0x0076, 0x0C),
    ov!(0x0077, 0x10),
    ov!(0x0078, 0x8B),
    ov!(0x0079, 0x7D),
    ov!(0x007A, 0xE0),
    ov!(0x00A6, 0x8B),
    ov!(0x00A7, 0xE0),
];
static DSOUND_CMCPXVOICECLIENT_SETFILTER_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x08),
    ov!(0x000A, 0x59),
    ov!(0x0010, 0x80),
    ov!(0x0016, 0x80),
    ov!(0x001C, 0x5F),
    ov!(0x0022, 0x8B),
    ov!(0x0029, 0x33),
];
static DSOUND_CMCPXVOICECLIENT_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0010, 0x8B),
    ov!(0x0021, 0xEB),
    ov!(0x007D, 0x75),
    ov!(0x007E, 0x28),
    ov!(0x007F, 0x39),
    ov!(0x0080, 0x05),
    ov!(0x0081, 0x10),
    ov!(0x0082, 0x00),
    ov!(0x0083, 0x82),
    ov!(0x0084, 0xFE),
    ov!(0x00A1, 0x89),
    ov!(0x00B7, 0x6C),
];
static DSOUND_CMCPXVOICECLIENT_SETLFO_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001D, 0x86),
    ov!(0x001E, 0x84),
    ov!(0x0021, 0x00),
    ov!(0x002E, 0x8B),
    ov!(0x0030, 0xE0),
    ov!(0x0031, 0x00),
    ov!(0x0041, 0x74),
    ov!(0x0042, 0x7F),
    ov!(0x005A, 0xA3),
    ov!(0x005B, 0xF8),
    ov!(0x005C, 0x02),
    ov!(0x005D, 0x82),
    ov!(0x005E, 0xFE),
    ov!(0x00BF, 0x72),
    ov!(0x00C0, 0x8C),
    ov!(0x00D6, 0xC3),
];
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0x8D),
    ov!(0x0022, 0xBE),
    ov!(0x0023, 0xB4),
    ov!(0x0024, 0x00),
    ov!(0x0058, 0x0F),
    ov!(0x0059, 0xB7),
    ov!(0x005A, 0x40),
    ov!(0x005B, 0x02),
    ov!(0x005E, 0x48),
    ov!(0x005F, 0xD1),
    ov!(0x0060, 0xF8),
];
static DSOUND_CMCPXVOICECLIENT_SETPITCH_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0026, 0x8B),
    ov!(0x0027, 0x8E),
    ov!(0x0028, 0xE0),
    ov!(0x0029, 0x00),
    ov!(0x002F, 0x0F),
    ov!(0x0030, 0xB7),
    ov!(0x0031, 0x49),
    ov!(0x0032, 0x02),
    ov!(0x0035, 0x49),
    ov!(0x0036, 0xD1),
    ov!(0x0037, 0xF9),
];
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001E, 0xEB),
    ov!(0x002E, 0x6A),
    ov!(0x003E, 0x40),
    ov!(0x005E, 0x7E),
    ov!(0x007E, 0x1E),
    ov!(0x00A2, 0xD9),
    ov!(0x00BE, 0x09),
];
static DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_CSENSAURA3D_GETLITEHRTFFILTERPAIR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0015, 0x51),
    ov!(0x0019, 0xE8),
    ov!(0x001E, 0x99),
    ov!(0x001F, 0x6A),
    ov!(0x0020, 0x03),
    ov!(0x0021, 0x59),
    ov!(0x0022, 0xF7),
    ov!(0x0023, 0xF9),
];
static DSOUND_CSENSAURA3D_GETLITEHRTFFILTERPAIR_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DSOUND_CREFCOUNT_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x41),
    ov!(0x000C, 0x04),
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x71),
    ov!(0x0011, 0x04),
    ov!(0x0019, 0xFF),
    ov!(0x001A, 0x15),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static DSOUND_DSOUND_CREFCOUNT_ADDREF_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DSOUND_CREFCOUNT_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0x49),
    ov!(0x000D, 0x04),
    ov!(0x000E, 0x8B),
    ov!(0x000F, 0x71),
    ov!(0x0010, 0x04),
    ov!(0x0026, 0xFF),
    ov!(0x0027, 0x15),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x04),
];
static DSOUND_DSOUND_CREFCOUNT_RELEASE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDCREATE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0023, 0x83),
    ov!(0x0024, 0xC0),
    ov!(0x0025, 0x08),
    ov!(0x0034, 0x6A),
    ov!(0x0035, 0x1C),
    ov!(0x0075, 0x1B),
    ov!(0x0076, 0xC0),
    ov!(0x009B, 0xC2),
    ov!(0x009C, 0x0C),
];
static DSOUND_DIRECTSOUNDCREATE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDCREATEBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0020, 0x0C),
    ov!(0x0023, 0x08),
    ov!(0x0026, 0xFC),
    ov!(0x002E, 0x8D),
    ov!(0x002F, 0x45),
    ov!(0x0030, 0xFC),
    ov!(0x003B, 0xC2),
    ov!(0x003C, 0x08),
];
static DSOUND_DIRECTSOUNDCREATEBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0028,
    target: "IDirectSound_CreateSoundBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDCREATESTREAM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0020, 0x0C),
    ov!(0x0023, 0x08),
    ov!(0x0026, 0xFC),
    ov!(0x002E, 0x8D),
    ov!(0x002F, 0x45),
    ov!(0x0030, 0xFC),
    ov!(0x003B, 0xC2),
    ov!(0x003C, 0x08),
];
static DSOUND_DIRECTSOUNDCREATESTREAM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0028,
    target: "IDirectSound_CreateSoundStream",
}];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDDOWORK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x74),
    ov!(0x0010, 0x06),
    ov!(0x0019, 0x5E),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x0B),
    ov!(0x0027, 0xC3),
];
static DSOUND_DIRECTSOUNDDOWORK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0013,
    target: "CDirectSound_DoWork",
}];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDENTERCRITICALSECTION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0xB6),
    ov!(0x0006, 0x00),
    ov!(0x000A, 0x74),
    ov!(0x000E, 0xC3),
    ov!(0x0014, 0xFF),
    ov!(0x001A, 0x33),
    ov!(0x001B, 0xC0),
];
static DSOUND_DIRECTSOUNDENTERCRITICALSECTION_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDGETSAMPLETIME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xA1),
    ov!(0x0001, 0x0C),
    ov!(0x0002, 0x20),
    ov!(0x0003, 0x80),
    ov!(0x0004, 0xFE),
    ov!(0x0005, 0xC3),
];
static DSOUND_DIRECTSOUNDGETSAMPLETIME_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDUSEFULLHRTF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x85),
    ov!(0x0006, 0xC0),
    ov!(0x0011, 0x74),
    ov!(0x0012, 0x0B),
    ov!(0x0013, 0x68),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSEFULLHRTF_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CSensaura3d_GetFullHRTFFilterPair",
}];

// Source: DSound/3911.inl
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x85),
    ov!(0x0006, 0xC0),
    ov!(0x0011, 0x74),
    ov!(0x0012, 0x0B),
    ov!(0x0013, 0x68),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CSensaura3d_GetLiteHRTFFilterPair",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8D),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0xE4),
    ov!(0x0007, 0xF7),
    ov!(0x0008, 0xD8),
    ov!(0x0009, 0x1B),
    ov!(0x000A, 0xC0),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0x51),
    ov!(0x0012, 0x04),
    ov!(0x0013, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_ADDREF_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xE4),
    ov!(0x0013, 0x1B),
    ov!(0x0014, 0xC9),
    ov!(0x0015, 0x23),
    ov!(0x0016, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_GetCurrentPosition",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_GETSTATUS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_GETSTATUS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_GetStatus",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_LOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0xFF),
    ov!(0x0004, 0x75),
    ov!(0x0005, 0x24),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xC0),
    ov!(0x0013, 0xE4),
    ov!(0x002D, 0xC2),
    ov!(0x002E, 0x20),
];
static DSOUND_IDIRECTSOUNDBUFFER_LOCK_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0028,
    target: "CDirectSoundBuffer_Lock",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_PLAY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0001, 0x74),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x10),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xE4),
    ov!(0x0017, 0x1B),
    ov!(0x0018, 0xC9),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUNDBUFFER_PLAY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_Play",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_PLAYEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x10),
    ov!(0x0008, 0xFF),
    ov!(0x000D, 0xC8),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_PLAYEX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_PlayEx",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8D),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0xE4),
    ov!(0x0007, 0xF7),
    ov!(0x0008, 0xD8),
    ov!(0x0009, 0x1B),
    ov!(0x000A, 0xC0),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0x51),
    ov!(0x0012, 0x08),
    ov!(0x0013, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_RELEASE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xE4),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetBufferData",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x10),
    ov!(0x0008, 0xFF),
    ov!(0x000D, 0xC8),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x0028, 0xD9),
    ov!(0x0029, 0x1C),
    ov!(0x002A, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x0034, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "CDirectSoundBuffer_SetConeOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x74),
    ov!(0x000C, 0x24),
    ov!(0x000D, 0x0C),
    ov!(0x0015, 0x23),
    ov!(0x0016, 0xC8),
    ov!(0x001E, 0x0C),
    ov!(0x001F, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetCurrentPosition",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETEG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETEG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x000F, 0x1B),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x23),
    ov!(0x0012, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETLFO_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETLOOPREGION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xE4),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETLOOPREGION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetLoopRegion",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x0C),
    ov!(0x0008, 0x8B),
    ov!(0x000D, 0x8B),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetMaxDistance",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x0C),
    ov!(0x0008, 0x8B),
    ov!(0x000D, 0x8B),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetMinDistance",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x000F, 0x1B),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x23),
    ov!(0x0012, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMODE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xE4),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetNotificationPositions",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetOutputBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x45),
    ov!(0x000B, 0x08),
    ov!(0x0028, 0xD9),
    ov!(0x0029, 0x1C),
    ov!(0x002A, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x0034, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "CDirectSoundBuffer_SetPosition",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x45),
    ov!(0x000B, 0x08),
    ov!(0x0028, 0xD9),
    ov!(0x0029, 0x1C),
    ov!(0x002A, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x0034, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "CDirectSoundBuffer_SetVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_STOP_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0xC8),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xC0),
    ov!(0x0008, 0xE4),
    ov!(0x0016, 0x04),
    ov!(0x0017, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_STOP_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundBuffer_Stop",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_STOPEX_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x10),
    ov!(0x0008, 0xFF),
    ov!(0x000D, 0xC8),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUNDBUFFER_STOPEX_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_StopEx",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDBUFFER_UNLOCK_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x33),
    ov!(0x0001, 0xC0),
    ov!(0x0002, 0xC2),
    ov!(0x0003, 0x14),
    ov!(0x0004, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_UNLOCK_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_PAUSE_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_PAUSE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_Pause",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetAllParameters",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEANGLES_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEANGLES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetConeAngles",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x14),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundStream_SetConeOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetConeOutsideVolume",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETEG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETEG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetI3DL2Source",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETLFO_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETLFO_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0xFF),
    ov!(0x000D, 0x74),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x0C),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_SetMaxDistance",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0xFF),
    ov!(0x000D, 0x74),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x0C),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_SetMinDistance",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetMixBinVolumes_12",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMODE_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETMODE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetMode",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetOutputBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x14),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundStream_SetPosition",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x001A, 0xD9),
    ov!(0x001B, 0x1C),
    ov!(0x001C, 0x24),
    ov!(0x0026, 0xC2),
    ov!(0x0027, 0x14),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CDirectSoundStream_SetVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0008, 0x83),
    ov!(0x0009, 0xC0),
    ov!(0x000A, 0x04),
    ov!(0x0011, 0xC2),
    ov!(0x0012, 0x08),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000D,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_ADDREF_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8D),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0xF8),
    ov!(0x0007, 0xF7),
    ov!(0x0008, 0xD8),
    ov!(0x0009, 0x1B),
    ov!(0x000A, 0xC0),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x08),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0x51),
    ov!(0x0012, 0x04),
];
static DSOUND_IDIRECTSOUND_ADDREF_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xC0),
    ov!(0x0008, 0xF8),
    ov!(0x0009, 0xF7),
    ov!(0x000A, 0xD9),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x04),
];
static DSOUND_IDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSound_CommitDeferredSettings",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_COMMITEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0xC8),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xC0),
    ov!(0x0008, 0xF8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x04),
];
static DSOUND_IDIRECTSOUND_COMMITEFFECTDATA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSound_CommitEffectData",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_CREATESOUNDBUFFER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xF8),
    ov!(0x0017, 0x1B),
    ov!(0x0018, 0xC9),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUND_CREATESOUNDBUFFER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_CreateSoundBuffer",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_CREATESOUNDSTREAM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xF8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUND_CREATESOUNDSTREAM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_CreateSoundStream",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0xFF),
    ov!(0x0004, 0x75),
    ov!(0x0005, 0x18),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x08),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xC0),
    ov!(0x0013, 0xF8),
    ov!(0x0024, 0xC2),
    ov!(0x0025, 0x14),
];
static DSOUND_IDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CDirectSound_DownloadEffectsImage",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_ENABLEHEADPHONES_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xF8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUND_ENABLEHEADPHONES_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSound_EnableHeadphones",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_GETCAPS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUND_GETCAPS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSound_GetCaps",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_GETEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0xFF),
    ov!(0x0004, 0x75),
    ov!(0x0005, 0x18),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x08),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xC0),
    ov!(0x0013, 0xF8),
    ov!(0x0024, 0xC2),
    ov!(0x0025, 0x14),
];
static DSOUND_IDIRECTSOUND_GETEFFECTDATA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001F,
    target: "CDirectSound_GetEffectData",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_GETSPEAKERCONFIG_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUND_GETSPEAKERCONFIG_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSound_GetSpeakerConfig",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_GETTIME_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xF8),
    ov!(0x000D, 0xF7),
    ov!(0x000E, 0xD9),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUND_GETTIME_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSound_GetTime",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_RELEASE_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8D),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0xF8),
    ov!(0x0007, 0xF7),
    ov!(0x0008, 0xD8),
    ov!(0x0010, 0xFF),
    ov!(0x0011, 0x51),
    ov!(0x0012, 0x08),
    ov!(0x0013, 0xC2),
    ov!(0x0014, 0x04),
];
static DSOUND_IDIRECTSOUND_RELEASE_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETALLPARAMETERS_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xC8),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xF8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETALLPARAMETERS_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSound_SetAllParameters",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETCOOPERATIVELEVEL_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x33),
    ov!(0x0001, 0xC0),
    ov!(0x0002, 0xC2),
    ov!(0x0003, 0x0C),
    ov!(0x0004, 0x00),
];
static DSOUND_IDIRECTSOUND_SETCOOPERATIVELEVEL_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETDISTANCEFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0x51),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xF8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETDISTANCEFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_SetDistanceFactor",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETDOPPLERFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0x51),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xF8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETDOPPLERFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_SetDopplerFactor",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETEFFECTDATA_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0xFF),
    ov!(0x0004, 0x75),
    ov!(0x0005, 0x1C),
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x08),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0xC0),
    ov!(0x0013, 0xF8),
    ov!(0x0027, 0xC2),
    ov!(0x0028, 0x18),
];
static DSOUND_IDIRECTSOUND_SETEFFECTDATA_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0022,
    target: "CDirectSound_SetEffectData",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETI3DL2LISTENER_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETI3DL2LISTENER_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSound_SetI3DL2Listener",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETMIXBINHEADROOM_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0xC8),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xF8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETMIXBINHEADROOM_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSound_SetMixBinHeadroom",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETORIENTATION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x20),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x18),
    ov!(0x0018, 0x83),
    ov!(0x0019, 0xC0),
    ov!(0x001A, 0xF8),
    ov!(0x001F, 0xF7),
    ov!(0x0020, 0xD9),
    ov!(0x0047, 0xC2),
    ov!(0x0048, 0x20),
];
static DSOUND_IDIRECTSOUND_SETORIENTATION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0042,
    target: "CDirectSound_SetOrientation",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETPOSITION_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x0C),
    ov!(0x001B, 0xF7),
    ov!(0x001C, 0xD9),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x14),
];
static DSOUND_IDIRECTSOUND_SETPOSITION_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "CDirectSound_SetPosition",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETROLLOFFFACTOR_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000C, 0x51),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xF8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x0C),
];
static DSOUND_IDIRECTSOUND_SETROLLOFFFACTOR_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_SetRolloffFactor",
}];

// Source: DSound/3911.inl
static DSOUND_IDIRECTSOUND_SETVELOCITY_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0xD9),
    ov!(0x0007, 0x45),
    ov!(0x0008, 0x14),
    ov!(0x000C, 0x83),
    ov!(0x000D, 0xEC),
    ov!(0x000E, 0x0C),
    ov!(0x001B, 0xF7),
    ov!(0x001C, 0xD9),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x14),
];
static DSOUND_IDIRECTSOUND_SETVELOCITY_3911_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002D,
    target: "CDirectSound_SetVelocity",
}];

// Source: DSound/3911.inl
static DSOUND_ISVALIDFORMAT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x0F),
    ov!(0x0007, 0x48),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xE8),
    ov!(0x000C, 0x68),
    ov!(0x0014, 0xE8),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x04),
];
static DSOUND_ISVALIDFORMAT_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_XAUDIOCALCULATEPITCH_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0008, 0x81),
    ov!(0x0009, 0xFE),
    ov!(0x000A, 0x80),
    ov!(0x000B, 0xBB),
    ov!(0x001D, 0xEB),
    ov!(0x001E, 0x2B),
    ov!(0x001F, 0x8D),
    ov!(0x0020, 0x4D),
    ov!(0x0021, 0x08),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x04),
];
static DSOUND_XAUDIOCALCULATEPITCH_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_XAUDIOCREATEADPCMFORMAT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x8B),
    ov!(0x0009, 0x8B),
    ov!(0x0010, 0x66),
    ov!(0x0011, 0xC7),
    ov!(0x0012, 0x02),
    ov!(0x0013, 0x69),
    ov!(0x0014, 0x00),
    ov!(0x0020, 0x04),
    ov!(0x0022, 0xE8),
    ov!(0x0038, 0x02),
    ov!(0x003E, 0x40),
    ov!(0x0041, 0xC2),
    ov!(0x0042, 0x0C),
];
static DSOUND_XAUDIOCREATEADPCMFORMAT_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_XAUDIOCREATEPCMFORMAT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0011, 0x02),
    ov!(0x0018, 0x0E),
    ov!(0x002E, 0x10),
    ov!(0x0034, 0x66),
    ov!(0x0035, 0xC7),
    ov!(0x0036, 0x01),
    ov!(0x0037, 0x01),
    ov!(0x0038, 0x00),
    ov!(0x0047, 0xC2),
    ov!(0x0048, 0x10),
];
static DSOUND_XAUDIOCREATEPCMFORMAT_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_XFILECREATEMEDIAOBJECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0008, 0xE8),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xC0),
    ov!(0x000F, 0x74),
    ov!(0x0010, 0x16),
    ov!(0x0011, 0x83),
    ov!(0x0012, 0x60),
    ov!(0x0013, 0x04),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0xC7),
    ov!(0x0016, 0x00),
    ov!(0x0078, 0xC2),
    ov!(0x0079, 0x18),
];
static DSOUND_XFILECREATEMEDIAOBJECT_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3911.inl
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_3911_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0007, 0x5C),
    ov!(0x0053, 0x8B),
    ov!(0x0054, 0x4D),
    ov!(0x0055, 0x10),
    ov!(0x0056, 0x89),
    ov!(0x0057, 0x01),
    ov!(0x0058, 0x8D),
    ov!(0x0059, 0x45),
    ov!(0x005A, 0xFC),
    ov!(0x005B, 0x50),
    ov!(0x005C, 0xE8),
    ov!(0x0065, 0xC2),
    ov!(0x0066, 0x0C),
];
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_3911_XREFS: &[OovpaXref] = &[];

// Source: DSound/3936.inl
static DSOUND_CMCPXSTREAM_FLUSH_3936_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0xF6),
    ov!(0x0012, 0xF6),
    ov!(0x0022, 0x33),
    ov!(0x0024, 0x33),
    ov!(0x002F, 0x83),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x03),
    ov!(0x004D, 0xE8),
    ov!(0x0055, 0x04),
    ov!(0x0056, 0x40),
    ov!(0x0057, 0x00),
    ov!(0x0058, 0x80),
    ov!(0x005F, 0xE8),
    ov!(0x00A1, 0xC9),
    ov!(0x00A2, 0xC3),
];
static DSOUND_CMCPXSTREAM_FLUSH_3936_XREFS: &[OovpaXref] = &[];

// Source: DSound/3936.inl
static DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3936_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x0C),
    ov!(0x000E, 0xD9),
    ov!(0x0020, 0xEB),
    ov!(0x0028, 0xD9),
    ov!(0x0030, 0x6A),
    ov!(0x00AA, 0xEB),
    ov!(0x00AB, 0x17),
    ov!(0x00AC, 0xD8),
    ov!(0x00AD, 0x05),
];
static DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3936_XREFS: &[OovpaXref] = &[];

pub const PATTERNS: &[OovpaPattern] = &[
    OovpaPattern {
        name: "CDirectSoundBufferSettings_SetBufferData",
        detect_size: 0x0071,
        entries: DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        detect_size: 0x0034,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetStatus",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Lock",
        detect_size: 0x0088,
        entries: DSOUND_CDIRECTSOUNDBUFFER_LOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Play",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_PlayEx",
        detect_size: 0x003C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetAllParameters",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetBufferData",
        detect_size: 0x0074,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeAngles",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOrientation",
        detect_size: 0x0029,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetEG",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFilter",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFrequency",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetHeadroom",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLFO",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLoopRegion",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMaxDistance",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMinDistance",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMixBinVolumes",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMixBins",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMode",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetOutputBuffer",
        detect_size: 0x002D,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPitch",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPosition",
        detect_size: 0x0029,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVelocity",
        detect_size: 0x0029,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVolume",
        detect_size: 0x0005,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Stop",
        detect_size: 0x002F,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_StopEx",
        detect_size: 0x0055,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_AddRef",
        detect_size: 0x000F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Constructor",
        detect_size: 0x0024,
        entries: DSOUND_CDIRECTSOUNDSTREAM_CONSTRUCTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Discontinuity",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Flush",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetInfo",
        detect_size: 0x004A,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetStatus",
        detect_size: 0x0034,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Pause",
        detect_size: 0x0012,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Process",
        detect_size: 0x0027,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PROCESS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Release",
        detect_size: 0x000F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetAllParameters",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeAngles",
        detect_size: 0x001B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOrientation",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetEG",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFilter",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFrequency",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetHeadroom",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetI3DL2Source",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetLFO",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMaxDistance",
        detect_size: 0x001B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMinDistance",
        detect_size: 0x001B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBinVolumes",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBins",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMode",
        detect_size: 0x0017,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetOutputBuffer",
        detect_size: 0x003F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPitch",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPosition",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVelocity",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVolume",
        detect_size: 0x0013,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetAllParameters",
        detect_size: 0x003A,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeAngles",
        detect_size: 0x003F,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOrientation",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        detect_size: 0x003A,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetEG",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFilter",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFrequency",
        detect_size: 0x0042,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetHeadroom",
        detect_size: 0x003E,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetI3DL2Source",
        detect_size: 0x0037,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetLFO",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMaxDistance",
        detect_size: 0x003E,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMinDistance",
        detect_size: 0x003E,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMixBinVolumes",
        detect_size: 0x0068,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_12_3911_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMixBins",
        detect_size: 0x0057,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMode",
        detect_size: 0x003A,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetOutputBuffer",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPitch",
        detect_size: 0x0036,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPosition",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVelocity",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVolume",
        detect_size: 0x0039,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_CommitDeferredSettings",
        detect_size: 0x002A,
        entries: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_CommitEffectData",
        detect_size: 0x0032,
        entries: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundBuffer",
        detect_size: 0x007F,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundStream",
        detect_size: 0x0074,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_DoWork",
        detect_size: 0x0026,
        entries: DSOUND_CDIRECTSOUND_DOWORK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_DownloadEffectsImage",
        detect_size: 0x0043,
        entries: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_EnableHeadphones",
        detect_size: 0x00CD,
        entries: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_GetCaps",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUND_GETCAPS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_GetEffectData",
        detect_size: 0x0041,
        entries: DSOUND_CDIRECTSOUND_GETEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_GetSpeakerConfig",
        detect_size: 0x0019,
        entries: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_GetTime",
        detect_size: 0x000E,
        entries: DSOUND_CDIRECTSOUND_GETTIME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetAllParameters",
        detect_size: 0x0037,
        entries: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetDistanceFactor",
        detect_size: 0x0027,
        entries: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetDopplerFactor",
        detect_size: 0x003F,
        entries: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetEffectData",
        detect_size: 0x004A,
        entries: DSOUND_CDIRECTSOUND_SETEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetI3DL2Listener",
        detect_size: 0x0045,
        entries: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetMixBinHeadroom",
        detect_size: 0x0023,
        entries: DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetOrientation",
        detect_size: 0x0030,
        entries: DSOUND_CDIRECTSOUND_SETORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetPosition",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetRolloffFactor",
        detect_size: 0x0027,
        entries: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CDirectSound_SetVelocity",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Commit3dSettings",
        detect_size: 0x00D7,
        entries: DSOUND_CMCPXAPU_COMMIT3DSETTINGS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        detect_size: 0x0037,
        entries: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dDistanceFactor",
        detect_size: 0x0014,
        entries: DSOUND_CMCPXAPU_SET3DDISTANCEFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dDopplerFactor",
        detect_size: 0x0014,
        entries: DSOUND_CMCPXAPU_SET3DDOPPLERFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dParameters",
        detect_size: 0x0030,
        entries: DSOUND_CMCPXAPU_SET3DPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dPosition",
        detect_size: 0x002A,
        entries: DSOUND_CMCPXAPU_SET3DPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dRolloffFactor",
        detect_size: 0x0014,
        entries: DSOUND_CMCPXAPU_SET3DROLLOFFFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_Set3dVelocity",
        detect_size: 0x002A,
        entries: DSOUND_CMCPXAPU_SET3DVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_SetI3DL2Listener",
        detect_size: 0x0032,
        entries: DSOUND_CMCPXAPU_SETI3DL2LISTENER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxAPU_SetMixBinHeadroom",
        detect_size: 0x002F,
        entries: DSOUND_CMCPXAPU_SETMIXBINHEADROOM_8_3911_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetCurrentPosition",
        detect_size: 0x00CB,
        entries: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetStatus",
        detect_size: 0x004A,
        entries: DSOUND_CMCPXBUFFER_GETSTATUS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0037,
        entries: DSOUND_CMCPXBUFFER_PLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_SetBufferData",
        detect_size: 0x0014,
        entries: DSOUND_CMCPXBUFFER_SETBUFFERDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_SetCurrentPosition",
        detect_size: 0x0075,
        entries: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Stop",
        detect_size: 0x0033,
        entries: DSOUND_CMCPXBUFFER_STOP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxStream_Discontinuity",
        detect_size: 0x0070,
        entries: DSOUND_CMCPXSTREAM_DISCONTINUITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxStream_Flush",
        detect_size: 0x009F,
        entries: DSOUND_CMCPXSTREAM_FLUSH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxStream_Pause",
        detect_size: 0x0047,
        entries: DSOUND_CMCPXSTREAM_PAUSE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x0088,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_4_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dConeOrientation",
        detect_size: 0x0027,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dConeOutsideVolume",
        detect_size: 0x0020,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dMaxDistance",
        detect_size: 0x0020,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dMinDistance",
        detect_size: 0x0020,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dMode",
        detect_size: 0x0020,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dParameters",
        detect_size: 0x0029,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dPosition",
        detect_size: 0x0027,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Set3dVelocity",
        detect_size: 0x0027,
        entries: DSOUND_CMCPXVOICECLIENT_SET3DVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetEG",
        detect_size: 0x009D,
        entries: DSOUND_CMCPXVOICECLIENT_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetFilter",
        detect_size: 0x00A8,
        entries: DSOUND_CMCPXVOICECLIENT_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetI3DL2Source",
        detect_size: 0x002A,
        entries: DSOUND_CMCPXVOICECLIENT_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetLFO",
        detect_size: 0x00B8,
        entries: DSOUND_CMCPXVOICECLIENT_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetMixBins",
        detect_size: 0x00D7,
        entries: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetPitch",
        detect_size: 0x0061,
        entries: DSOUND_CMCPXVOICECLIENT_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetVolume",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXVOICECLIENT_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CSensaura3d_GetFullHRTFFilterPair",
        detect_size: 0x00BF,
        entries: DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CSensaura3d_GetLiteHRTFFilterPair",
        detect_size: 0x0024,
        entries: DSOUND_CSENSAURA3D_GETLITEHRTFFILTERPAIR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DSound_CRefCount_AddRef",
        detect_size: 0x0024,
        entries: DSOUND_DSOUND_CREFCOUNT_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DSound_CRefCount_Release",
        detect_size: 0x0032,
        entries: DSOUND_DSOUND_CREFCOUNT_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundCreate",
        detect_size: 0x009D,
        entries: DSOUND_DIRECTSOUNDCREATE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundCreateBuffer",
        detect_size: 0x003D,
        entries: DSOUND_DIRECTSOUNDCREATEBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundCreateStream",
        detect_size: 0x003D,
        entries: DSOUND_DIRECTSOUNDCREATESTREAM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundDoWork",
        detect_size: 0x0028,
        entries: DSOUND_DIRECTSOUNDDOWORK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundEnterCriticalSection",
        detect_size: 0x001C,
        entries: DSOUND_DIRECTSOUNDENTERCRITICALSECTION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundGetSampleTime",
        detect_size: 0x0006,
        entries: DSOUND_DIRECTSOUNDGETSAMPLETIME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundUseFullHRTF",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSEFULLHRTF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "DirectSoundUseLightHRTF",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSELIGHTHRTF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_AddRef",
        detect_size: 0x0014,
        entries: DSOUND_IDIRECTSOUNDBUFFER_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_GetCurrentPosition",
        detect_size: 0x001D,
        entries: DSOUND_IDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_GetStatus",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_GETSTATUS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Lock",
        detect_size: 0x002F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_LOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Play",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUNDBUFFER_PLAY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_PlayEx",
        detect_size: 0x0022,
        entries: DSOUND_IDIRECTSOUNDBUFFER_PLAYEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Release",
        detect_size: 0x0014,
        entries: DSOUND_IDIRECTSOUNDBUFFER_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetAllParameters",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetBufferData",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetConeAngles",
        detect_size: 0x0022,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetConeOrientation",
        detect_size: 0x0035,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetConeOutsideVolume",
        detect_size: 0x0020,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetCurrentPosition",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetEG",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetFilter",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetFrequency",
        detect_size: 0x0019,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetHeadroom",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetI3DL2Source",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetLFO",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetLoopRegion",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETLOOPREGION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMaxDistance",
        detect_size: 0x0022,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMinDistance",
        detect_size: 0x0022,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMixBinVolumes",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMixBins",
        detect_size: 0x0019,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMode",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetNotificationPositions",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetOutputBuffer",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetPitch",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetPosition",
        detect_size: 0x0035,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetVelocity",
        detect_size: 0x0035,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetVolume",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Stop",
        detect_size: 0x0018,
        entries: DSOUND_IDIRECTSOUNDBUFFER_STOP_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_StopEx",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUNDBUFFER_STOPEX_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Unlock",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDBUFFER_UNLOCK_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_Pause",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_PAUSE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetAllParameters",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetConeAngles",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETCONEANGLES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetConeOrientation",
        detect_size: 0x0028,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetConeOutsideVolume",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetEG",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETEG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetFilter",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetFrequency",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetHeadroom",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetI3DL2Source",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetLFO",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETLFO_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMaxDistance",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMinDistance",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMixBinVolumes",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMixBins",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMode",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMODE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetOutputBuffer",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetPitch",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetPosition",
        detect_size: 0x0028,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetVelocity",
        detect_size: 0x0028,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetVolume",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_AddRef",
        detect_size: 0x0013,
        entries: DSOUND_IDIRECTSOUND_ADDREF_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_CommitDeferredSettings",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_CommitEffectData",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUND_COMMITEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_CreateSoundBuffer",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_CREATESOUNDBUFFER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_CreateSoundStream",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_CREATESOUNDSTREAM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_DownloadEffectsImage",
        detect_size: 0x0026,
        entries: DSOUND_IDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_EnableHeadphones",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUND_ENABLEHEADPHONES_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_GetCaps",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUND_GETCAPS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_GetEffectData",
        detect_size: 0x0026,
        entries: DSOUND_IDIRECTSOUND_GETEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_GetSpeakerConfig",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUND_GETSPEAKERCONFIG_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_GetTime",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUND_GETTIME_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_Release",
        detect_size: 0x0015,
        entries: DSOUND_IDIRECTSOUND_RELEASE_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetAllParameters",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUND_SETALLPARAMETERS_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetCooperativeLevel",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUND_SETCOOPERATIVELEVEL_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetDistanceFactor",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_SETDISTANCEFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetDopplerFactor",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_SETDOPPLERFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetEffectData",
        detect_size: 0x0029,
        entries: DSOUND_IDIRECTSOUND_SETEFFECTDATA_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetI3DL2Listener",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUND_SETI3DL2LISTENER_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetMixBinHeadroom",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUND_SETMIXBINHEADROOM_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetOrientation",
        detect_size: 0x0049,
        entries: DSOUND_IDIRECTSOUND_SETORIENTATION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetPosition",
        detect_size: 0x0034,
        entries: DSOUND_IDIRECTSOUND_SETPOSITION_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetRolloffFactor",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_SETROLLOFFFACTOR_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IDirectSound_SetVelocity",
        detect_size: 0x0034,
        entries: DSOUND_IDIRECTSOUND_SETVELOCITY_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "IsValidFormat",
        detect_size: 0x0023,
        entries: DSOUND_ISVALIDFORMAT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XAudioCalculatePitch",
        detect_size: 0x0050,
        entries: DSOUND_XAUDIOCALCULATEPITCH_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XAudioCreateAdpcmFormat",
        detect_size: 0x0043,
        entries: DSOUND_XAUDIOCREATEADPCMFORMAT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XAudioCreatePcmFormat",
        detect_size: 0x0049,
        entries: DSOUND_XAUDIOCREATEPCMFORMAT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XFileCreateMediaObject",
        detect_size: 0x007A,
        entries: DSOUND_XFILECREATEMEDIAOBJECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "XWaveFileCreateMediaObject",
        detect_size: 0x0067,
        entries: DSOUND_XWAVEFILECREATEMEDIAOBJECT_3911_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3911,
    },
    OovpaPattern {
        name: "CMcpxStream_Flush",
        detect_size: 0x00A3,
        entries: DSOUND_CMCPXSTREAM_FLUSH_3936_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3936,
    },
    OovpaPattern {
        name: "CSensaura3d_GetFullHRTFFilterPair",
        detect_size: 0x00AE,
        entries: DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3936_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 3936,
    },
];

pub const METADATA: &[OovpaPatternMeta] = &[
    OovpaPatternMeta {
        name: "CDirectSoundBufferSettings_SetBufferData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetStatus",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Lock",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_LOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Play",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_PlayEx",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetBufferData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeAngles",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFrequency",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLoopRegion",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMixBinVolumes",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetOutputBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Stop",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_StopEx",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_AddRef",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Constructor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_CONSTRUCTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Discontinuity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Flush",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetInfo",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetStatus",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Pause",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Process",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PROCESS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Release",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeAngles",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFrequency",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBinVolumes",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetOutputBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeAngles",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFrequency",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMixBinVolumes",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_12_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetOutputBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitDeferredSettings",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundStream",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_DoWork",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_DOWORK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_DownloadEffectsImage",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_EnableHeadphones",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetCaps",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETCAPS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetSpeakerConfig",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetTime",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETTIME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDistanceFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDopplerFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetI3DL2Listener",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetMixBinHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetRolloffFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Commit3dSettings",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_COMMIT3DSETTINGS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dDistanceFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DDISTANCEFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dDopplerFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DDOPPLERFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dRolloffFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DROLLOFFFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_Set3dVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SET3DVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_SetI3DL2Listener",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SETI3DL2LISTENER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_SetMixBinHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXAPU_SETMIXBINHEADROOM_8_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetStatus",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETSTATUS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_SetBufferData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_SETBUFFERDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_SetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Stop",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXBUFFER_STOP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Discontinuity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXSTREAM_DISCONTINUITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Flush",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXSTREAM_FLUSH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Pause",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXSTREAM_PAUSE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_4_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Set3dVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SET3DVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CSensaura3d_GetFullHRTFFilterPair",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CSensaura3d_GetLiteHRTFFilterPair",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_CSENSAURA3D_GETLITEHRTFFILTERPAIR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CRefCount_AddRef",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DSOUND_CREFCOUNT_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CRefCount_Release",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DSOUND_CREFCOUNT_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreate",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATEBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateStream",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATESTREAM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundDoWork",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDDOWORK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundEnterCriticalSection",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDENTERCRITICALSECTION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundGetSampleTime",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDGETSAMPLETIME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseFullHRTF",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDUSEFULLHRTF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseLightHRTF",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_DIRECTSOUNDUSELIGHTHRTF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_AddRef",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_GetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_GETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_GetStatus",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_GETSTATUS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Lock",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_LOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Play",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_PLAY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_PlayEx",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_PLAYEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Release",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetBufferData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETBUFFERDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetConeAngles",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetCurrentPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCURRENTPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetFrequency",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetLoopRegion",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETLOOPREGION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMixBinVolumes",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_12_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetNotificationPositions",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetOutputBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Stop",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_STOP_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_StopEx",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_STOPEX_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Unlock",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_UNLOCK_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_Pause",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_PAUSE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetConeAngles",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETCONEANGLES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetConeOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETCONEORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetConeOutsideVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetEG",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETEG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetFilter",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetFrequency",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetI3DL2Source",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETI3DL2SOURCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetLFO",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETLFO_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMaxDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMAXDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMinDistance",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMINDISTANCE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMixBinVolumes",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_12_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMixBins",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMode",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMODE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetOutputBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetPitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetVolume",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_AddRef",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_ADDREF_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_CommitDeferredSettings",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_COMMITDEFERREDSETTINGS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_CommitEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_COMMITEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_CreateSoundBuffer",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_CREATESOUNDBUFFER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_CreateSoundStream",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_CREATESOUNDSTREAM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_DownloadEffectsImage",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_DOWNLOADEFFECTSIMAGE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_EnableHeadphones",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_ENABLEHEADPHONES_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_GetCaps",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_GETCAPS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_GetEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_GETEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_GetSpeakerConfig",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_GETSPEAKERCONFIG_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_GetTime",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_GETTIME_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_Release",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_RELEASE_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetAllParameters",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETALLPARAMETERS_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetCooperativeLevel",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETCOOPERATIVELEVEL_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetDistanceFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETDISTANCEFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetDopplerFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETDOPPLERFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetEffectData",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETEFFECTDATA_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetI3DL2Listener",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETI3DL2LISTENER_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetMixBinHeadroom",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETMIXBINHEADROOM_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetOrientation",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETORIENTATION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetPosition",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETPOSITION_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetRolloffFactor",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETROLLOFFFACTOR_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SetVelocity",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_IDIRECTSOUND_SETVELOCITY_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "IsValidFormat",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_ISVALIDFORMAT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCalculatePitch",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_XAUDIOCALCULATEPITCH_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCreateAdpcmFormat",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_XAUDIOCREATEADPCMFORMAT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCreatePcmFormat",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_XAUDIOCREATEPCMFORMAT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObject",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "XWaveFileCreateMediaObject",
        min_version: 3911,
        source_file: "DSound/3911.inl",
        xrefs: DSOUND_XWAVEFILECREATEMEDIAOBJECT_3911_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Flush",
        min_version: 3936,
        source_file: "DSound/3936.inl",
        xrefs: DSOUND_CMCPXSTREAM_FLUSH_3936_XREFS,
    },
    OovpaPatternMeta {
        name: "CSensaura3d_GetFullHRTFFilterPair",
        min_version: 3936,
        source_file: "DSound/3936.inl",
        xrefs: DSOUND_CSENSAURA3D_GETFULLHRTFFILTERPAIR_3936_XREFS,
    },
];
