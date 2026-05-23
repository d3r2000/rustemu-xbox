// Auto-generated from Cxbx-R XbSymbolDatabase.
// Regenerate with: python3 tools/import_xbsymdb_database.py --xdk 5558
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

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000C, 0xBB),
    ov!(0x000D, 0x00),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x80),
    ov!(0x0020, 0xEB),
    ov!(0x0021, 0x06),
    ov!(0x0043, 0x80),
    ov!(0x0044, 0x66),
    ov!(0x0046, 0x7F),
    ov!(0x0078, 0xC2),
    ov!(0x0079, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0040, 0x74),
    ov!(0x0041, 0x0B),
    ov!(0x0051, 0xC2),
    ov!(0x0052, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0038,
    target: "CMcpxBuffer_GetCurrentPosition",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x48),
    ov!(0x002D, 0x20),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0x74),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x10),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0034,
    target: "CMcpxBuffer_GetStatus",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0034, 0xF6),
    ov!(0x0035, 0x45),
    ov!(0x0036, 0x24),
    ov!(0x0037, 0x01),
    ov!(0x0059, 0x8B),
    ov!(0x005A, 0x80),
    ov!(0x005B, 0x40),
    ov!(0x005C, 0x01),
    ov!(0x009F, 0x73),
    ov!(0x00A0, 0x11),
    ov!(0x00A1, 0x8B),
    ov!(0x00A2, 0x76),
    ov!(0x00A3, 0x1C),
];
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0045,
    target: "CDirectSoundBuffer_GetCurrentPosition",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0x74),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x18),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x10),
    ov!(0x004F, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0015, 0x68),
    ov!(0x0020, 0xB8),
    ov!(0x002D, 0x20),
    ov!(0x0039, 0x24),
    ov!(0x0045, 0x0B),
    ov!(0x0051, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003C,
    target: "CMcpxBuffer_Play_Ex",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x002A, 0x24),
    ov!(0x002E, 0x24),
    ov!(0x0032, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x18),
    ov!(0x0052, 0xC2),
    ov!(0x0053, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002A, 0xD9),
    ov!(0x002B, 0x45),
    ov!(0x002C, 0x14),
    ov!(0x0042, 0xD9),
    ov!(0x0043, 0x1C),
    ov!(0x0044, 0x24),
    ov!(0x0063, 0xC2),
    ov!(0x0064, 0x14),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0049,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x74),
    ov!(0x0032, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x003C, 0x74),
    ov!(0x003D, 0x0B),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0034,
    target: "CMcpxBuffer_SetCurrentPosition",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x85),
    ov!(0x001B, 0x15),
    ov!(0x0025, 0xEB),
    ov!(0x002F, 0x10),
    ov!(0x0039, 0x74),
    ov!(0x0046, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetFormat",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x85),
    ov!(0x001B, 0x15),
    ov!(0x0025, 0xEB),
    ov!(0x002F, 0x10),
    ov!(0x0039, 0x74),
    ov!(0x0046, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x74),
    ov!(0x0032, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x80),
    ov!(0x003C, 0x8D),
    ov!(0x003D, 0x1C),
    ov!(0x003E, 0x0E),
    ov!(0x003F, 0x3B),
    ov!(0x0040, 0x98),
    ov!(0x0061, 0x00),
    ov!(0x0062, 0x00),
    ov!(0x0063, 0x8B),
    ov!(0x0064, 0x4A),
    ov!(0x0065, 0x20),
    ov!(0x0066, 0xE8),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0031, 0xD9),
    ov!(0x0032, 0x1C),
    ov!(0x0033, 0x24),
    ov!(0x0053, 0x0C),
    ov!(0x0054, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0031, 0xD9),
    ov!(0x0032, 0x1C),
    ov!(0x0033, 0x24),
    ov!(0x0053, 0x0C),
    ov!(0x0054, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0030, 0xFF),
    ov!(0x0031, 0x74),
    ov!(0x0032, 0x24),
    ov!(0x0033, 0x14),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0009, 0xFF),
    ov!(0x000A, 0x74),
    ov!(0x000B, 0x24),
    ov!(0x000C, 0x10),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x4E),
    ov!(0x000F, 0x1C),
    ov!(0x0010, 0xE8),
    ov!(0x001C, 0xE8),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetOutputBuffer",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x24),
    ov!(0x002B, 0x10),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x0048, 0x5F),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0018, 0x68),
    ov!(0x0023, 0xB8),
    ov!(0x0037, 0x14),
    ov!(0x0038, 0x8B),
    ov!(0x0039, 0x75),
    ov!(0x003A, 0x0C),
    ov!(0x003B, 0x03),
    ov!(0x003C, 0xF0),
    ov!(0x003D, 0x3B),
    ov!(0x003E, 0xB1),
    ov!(0x007C, 0xC2),
    ov!(0x007D, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002A, 0xD9),
    ov!(0x002B, 0x45),
    ov!(0x002C, 0x14),
    ov!(0x0042, 0xD9),
    ov!(0x0043, 0x1C),
    ov!(0x0044, 0x24),
    ov!(0x0063, 0xC2),
    ov!(0x0064, 0x14),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0049,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002A, 0xD9),
    ov!(0x002B, 0x45),
    ov!(0x002C, 0x14),
    ov!(0x0042, 0xD9),
    ov!(0x0043, 0x1C),
    ov!(0x0044, 0x24),
    ov!(0x0063, 0xC2),
    ov!(0x0064, 0x14),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0049,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x22),
    ov!(0x002C, 0xFF),
    ov!(0x002D, 0x74),
    ov!(0x002E, 0x24),
    ov!(0x002F, 0x10),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOP_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x48),
    ov!(0x002D, 0x20),
    ov!(0x0038, 0x74),
    ov!(0x0039, 0x0B),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x04),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOP_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0030,
    target: "CMcpxBuffer_Stop",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0023, 0xB8),
    ov!(0x0049, 0xEB),
    ov!(0x004A, 0x11),
    ov!(0x004B, 0xFF),
    ov!(0x004C, 0x75),
    ov!(0x004D, 0x10),
    ov!(0x004E, 0x8B),
    ov!(0x004F, 0x45),
    ov!(0x0050, 0x08),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0024, 0x8B),
    ov!(0x0025, 0x4C),
    ov!(0x0028, 0xFF),
    ov!(0x0029, 0x41),
    ov!(0x002A, 0x08),
    ov!(0x002E, 0x8B),
    ov!(0x002F, 0x71),
    ov!(0x0030, 0x08),
    ov!(0x0041, 0xC2),
    ov!(0x0042, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0027, 0x8B),
    ov!(0x002A, 0x08),
    ov!(0x002B, 0x8B),
    ov!(0x002D, 0x24),
    ov!(0x002F, 0xE8),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0030,
    target: "CMcpxStream_Discontinuity",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0027, 0x8B),
    ov!(0x002A, 0x08),
    ov!(0x002B, 0x8B),
    ov!(0x002D, 0x24),
    ov!(0x002F, 0xE8),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0030,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x002D, 0x8B),
    ov!(0x002E, 0x7C),
    ov!(0x002F, 0x24),
    ov!(0x0030, 0x10),
    ov!(0x0037, 0x8B),
    ov!(0x0038, 0x4F),
    ov!(0x0039, 0x24),
    ov!(0x0042, 0x8B),
    ov!(0x0043, 0x47),
    ov!(0x0044, 0x20),
    ov!(0x0048, 0x83),
    ov!(0x0049, 0x66),
    ov!(0x004A, 0x08),
    ov!(0x004B, 0x00),
    ov!(0x0063, 0xC2),
    ov!(0x0064, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0027, 0x8B),
    ov!(0x002A, 0x08),
    ov!(0x002B, 0x8B),
    ov!(0x002D, 0x24),
    ov!(0x0033, 0x8B),
    ov!(0x0036, 0x0C),
    ov!(0x0037, 0xF7),
    ov!(0x0039, 0x1B),
    ov!(0x003B, 0xF7),
    ov!(0x003F, 0x89),
    ov!(0x0040, 0x01),
    ov!(0x0051, 0xC2),
    ov!(0x0052, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x48),
    ov!(0x002D, 0x24),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0034,
    target: "CMcpxStream_Pause",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_PROCESS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x57),
    ov!(0x0028, 0x8B),
    ov!(0x002B, 0x0C),
    ov!(0x002C, 0x8B),
    ov!(0x002E, 0x24),
    ov!(0x0034, 0x85),
    ov!(0x0035, 0xC0),
    ov!(0x0039, 0x32),
    ov!(0x003A, 0x00),
    ov!(0x003B, 0x78),
    ov!(0x003C, 0x88),
    ov!(0x003F, 0xFF),
    ov!(0x0042, 0x10),
    ov!(0x0043, 0x8B),
    ov!(0x0045, 0x24),
    ov!(0x0060, 0xC2),
    ov!(0x0061, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_PROCESS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x002C, 0x83),
    ov!(0x002D, 0xC0),
    ov!(0x002E, 0x04),
    ov!(0x0046, 0x8B),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x04),
    ov!(0x004C, 0x00),
];
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "DSound_CRefCount_Release",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0038, 0xE8),
    ov!(0x0043, 0x68),
    ov!(0x004E, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x002F, 0x18),
    ov!(0x0032, 0x04),
    ov!(0x0036, 0x18),
    ov!(0x003C, 0xE8),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0016, 0x74),
    ov!(0x0023, 0xB8),
    ov!(0x0029, 0x3B),
    ov!(0x0038, 0xEC),
    ov!(0x0043, 0x24),
    ov!(0x004C, 0xE8),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004D,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0020, 0xB8),
    ov!(0x0023, 0x00),
    ov!(0x0038, 0xE8),
    ov!(0x003D, 0x85),
    ov!(0x004E, 0x8B),
    ov!(0x0052, 0xC2),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x0B),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x26),
    ov!(0x0029, 0x24),
    ov!(0x003D, 0x74),
    ov!(0x003E, 0x0B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetFormat",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0014, 0x0B),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x26),
    ov!(0x0029, 0x24),
    ov!(0x003D, 0x74),
    ov!(0x003E, 0x0B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0038, 0xE8),
    ov!(0x0043, 0x68),
    ov!(0x004E, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0x00),
    ov!(0x000E, 0xF0),
    ov!(0x0011, 0x85),
    ov!(0x0014, 0x0B),
    ov!(0x0039, 0x85),
    ov!(0x003C, 0xF8),
    ov!(0x003F, 0x68),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x003C, 0xE8),
    ov!(0x0047, 0x68),
    ov!(0x0052, 0x8B),
    ov!(0x0056, 0xC2),
    ov!(0x0057, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x003C, 0xE8),
    ov!(0x0047, 0x68),
    ov!(0x0052, 0x8B),
    ov!(0x0056, 0xC2),
    ov!(0x0057, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0xF0),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0038, 0xE8),
    ov!(0x0043, 0x68),
    ov!(0x004E, 0x8B),
    ov!(0x0052, 0xC2),
    ov!(0x0053, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetOutputBuffer",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
    ov!(0x0050, 0x00),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0016, 0x74),
    ov!(0x0023, 0xB8),
    ov!(0x0029, 0x3B),
    ov!(0x0038, 0xEC),
    ov!(0x0057, 0x68),
    ov!(0x0062, 0x8B),
    ov!(0x0067, 0xC2),
    ov!(0x0068, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004D,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0016, 0x74),
    ov!(0x0023, 0xB8),
    ov!(0x0029, 0x3B),
    ov!(0x0034, 0x83),
    ov!(0x003F, 0x45),
    ov!(0x0062, 0x8B),
    ov!(0x0067, 0xC2),
    ov!(0x0068, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004D,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0013, 0x74),
    ov!(0x0020, 0xB8),
    ov!(0x0029, 0x24),
    ov!(0x0034, 0xE8),
    ov!(0x003F, 0x68),
    ov!(0x004A, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x76),
    ov!(0x000A, 0x16),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x44),
    ov!(0x001A, 0xB9),
    ov!(0x001B, 0x50),
    ov!(0x001E, 0x72),
    ov!(0x001F, 0xEC),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0x89),
    ov!(0x000D, 0x84),
    ov!(0x000E, 0x00),
    ov!(0x0014, 0x09),
    ov!(0x0015, 0x88),
    ov!(0x0016, 0x38),
    ov!(0x0017, 0x01),
    ov!(0x002F, 0xC2),
    ov!(0x0030, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001E,
    target: "CMcpxVoiceClient_Commit3dSettings",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000D, 0x6A),
    ov!(0x000E, 0x10),
    ov!(0x0016, 0xF3),
    ov!(0x0017, 0xA5),
    ov!(0x0018, 0x8B),
    ov!(0x0019, 0x42),
    ov!(0x001A, 0x10),
    ov!(0x0021, 0xFF),
    ov!(0x0026, 0x01),
    ov!(0x0033, 0xC2),
    ov!(0x0034, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0006, 0x10),
    ov!(0x0007, 0x8B),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x89),
    ov!(0x000C, 0x91),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x91),
    ov!(0x0027, 0x10),
    ov!(0x002C, 0x01),
    ov!(0x0037, 0xC2),
    ov!(0x0038, 0x10),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x08),
    ov!(0x001C, 0xFC),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0x18),
    ov!(0x0034, 0xF6),
    ov!(0x0035, 0x45),
    ov!(0x0036, 0x18),
    ov!(0x0037, 0x01),
    ov!(0x0038, 0x5E),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x89),
    ov!(0x000C, 0x90),
    ov!(0x000D, 0x04),
    ov!(0x000E, 0x01),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x8B),
    ov!(0x0014, 0x83),
    ov!(0x001A, 0x10),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETEG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0xFF),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x0C),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETEG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "CMcpxVoiceClient_SetEG",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0xFF),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x0C),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFILTER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "CMcpxVoiceClient_SetFilter",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0001, 0x56),
    ov!(0x000A, 0x6A),
    ov!(0x000B, 0x01),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x002D,
        target: "CMcpxVoiceClient_SetMixBins",
    },
    OovpaXref {
        offset: 0x003B,
        target: "CMcpxVoiceClient_SetPitch",
    },
];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x08),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0015,
        target: "XAudioCalculatePitch",
    },
    OovpaXref {
        offset: 0x001C,
        target: "CDirectSoundVoice_SetPitch",
    },
];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0014, 0x89),
    ov!(0x0015, 0x70),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x4A),
    ov!(0x0019, 0x0C),
    ov!(0x001A, 0xE8),
    ov!(0x0020, 0xC2),
    ov!(0x0021, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001B,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x54),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x001B, 0x80),
    ov!(0x001C, 0x88),
    ov!(0x001D, 0x38),
    ov!(0x001E, 0x01),
    ov!(0x001F, 0x00),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x80),
    ov!(0x0022, 0xF6),
    ov!(0x0023, 0x44),
    ov!(0x0024, 0x24),
    ov!(0x0025, 0x14),
    ov!(0x0026, 0x01),
    ov!(0x0057, 0xC2),
    ov!(0x0058, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0xFF),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x0C),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETLFO_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "CMcpxVoiceClient_SetLFO",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x89),
    ov!(0x000C, 0x90),
    ov!(0x000D, 0x0C),
    ov!(0x0014, 0x83),
    ov!(0x0015, 0x88),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x04),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x89),
    ov!(0x000C, 0x90),
    ov!(0x000D, 0x08),
    ov!(0x0014, 0x83),
    ov!(0x0015, 0x88),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x04),
    ov!(0x002A, 0xC2),
    ov!(0x002B, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_8_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0005, 0xFF),
    ov!(0x0006, 0x74),
    ov!(0x0007, 0x24),
    ov!(0x0008, 0x0C),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x4E),
    ov!(0x000B, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x4E),
    ov!(0x0013, 0x0C),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_8_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000D,
        target: "CDirectSoundVoiceSettings_SetMixBinVolumes",
    },
    OovpaXref {
        offset: 0x0015,
        target: "CMcpxVoiceClient_SetVolume",
    },
];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x74),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0xFF),
    ov!(0x0006, 0x74),
    ov!(0x0007, 0x24),
    ov!(0x0008, 0x0C),
    ov!(0x0009, 0x8B),
    ov!(0x000A, 0x4E),
    ov!(0x000B, 0x10),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x4E),
    ov!(0x0013, 0x0C),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x000D,
        target: "CDirectSoundVoiceSettings_SetMixBins",
    },
    OovpaXref {
        offset: 0x0015,
        target: "CMcpxVoiceClient_SetMixBins",
    },
];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xF6),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x0C),
    ov!(0x0004, 0x01),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x44),
    ov!(0x0007, 0x24),
    ov!(0x0016, 0x75),
    ov!(0x0019, 0xE8),
    ov!(0x0020, 0xC2),
    ov!(0x0021, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0010, 0x24),
    ov!(0x0011, 0x10),
    ov!(0x0012, 0x23),
    ov!(0x0013, 0xF0),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x47),
    ov!(0x0016, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x80),
    ov!(0x0045, 0xE8),
    ov!(0x0051, 0xC2),
    ov!(0x0052, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x10),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x89),
    ov!(0x000C, 0x51),
    ov!(0x000E, 0x8B),
    ov!(0x0016, 0xC2),
    ov!(0x0017, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPITCH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0012,
    target: "CMcpxVoiceClient_SetPitch",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x08),
    ov!(0x001C, 0xDC),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0xFF),
    ov!(0x0034, 0xF6),
    ov!(0x0035, 0x45),
    ov!(0x0036, 0x18),
    ov!(0x0037, 0x01),
    ov!(0x0038, 0x5E),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0005, 0x08),
    ov!(0x001C, 0xE8),
    ov!(0x0031, 0x00),
    ov!(0x0032, 0x00),
    ov!(0x0033, 0x40),
    ov!(0x0034, 0xF6),
    ov!(0x0035, 0x45),
    ov!(0x0036, 0x18),
    ov!(0x0037, 0x01),
    ov!(0x0038, 0x5E),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x14),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x41),
    ov!(0x0006, 0x10),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x54),
    ov!(0x0009, 0x24),
    ov!(0x000A, 0x08),
    ov!(0x000B, 0x2B),
    ov!(0x000C, 0x50),
    ov!(0x000D, 0x28),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0024, 0xB8),
    ov!(0x0025, 0x05),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x80),
    ov!(0x0052, 0x83),
    ov!(0x0053, 0xA0),
    ov!(0x0054, 0x84),
    ov!(0x0055, 0x00),
    ov!(0x0056, 0x00),
    ov!(0x006F, 0xC9),
];
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0045,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x24),
    ov!(0x0027, 0x8B),
    ov!(0x0028, 0x44),
    ov!(0x0031, 0x6A),
    ov!(0x0033, 0x6A),
    ov!(0x0035, 0xE8),
    ov!(0x004C, 0xC2),
    ov!(0x004D, 0x04),
];
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0xB8),
    ov!(0x0026, 0x05),
    ov!(0x0027, 0x40),
    ov!(0x0029, 0x80),
    ov!(0x002E, 0x6A),
    ov!(0x002F, 0x24),
    ov!(0x004C, 0x81),
    ov!(0x004D, 0xE6),
    ov!(0x004E, 0xF2),
    ov!(0x004F, 0xFF),
    ov!(0x0050, 0xF8),
    ov!(0x0051, 0x7F),
    ov!(0x009C, 0xC2),
    ov!(0x009D, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0025, 0xB8),
    ov!(0x002A, 0xEB),
    ov!(0x002B, 0x62),
    ov!(0x002C, 0x6A),
    ov!(0x002D, 0x01),
    ov!(0x002E, 0x6A),
    ov!(0x002F, 0x28),
    ov!(0x004C, 0x81),
    ov!(0x004D, 0xE6),
    ov!(0x004E, 0xF2),
    ov!(0x004F, 0xFF),
    ov!(0x0050, 0xF8),
    ov!(0x0051, 0x7F),
    ov!(0x0091, 0xC2),
    ov!(0x0092, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x75),
    ov!(0x002D, 0x08),
    ov!(0x002E, 0xFF),
    ov!(0x002F, 0x75),
    ov!(0x0030, 0x14),
    ov!(0x0031, 0x8B),
    ov!(0x0032, 0x4E),
    ov!(0x0033, 0x08),
    ov!(0x0039, 0xFF),
    ov!(0x003B, 0x18),
    ov!(0x003F, 0xFF),
    ov!(0x0041, 0x10),
    ov!(0x0042, 0xFF),
    ov!(0x0044, 0x0C),
    ov!(0x0060, 0xC2),
    ov!(0x0061, 0x14),
];
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0042, 0x0F),
    ov!(0x0043, 0x95),
    ov!(0x0044, 0xC3),
    ov!(0x004F, 0x0D),
    ov!(0x0050, 0x00),
    ov!(0x0051, 0x00),
    ov!(0x0052, 0x00),
    ov!(0x0053, 0x80),
    ov!(0x0056, 0x25),
    ov!(0x0057, 0xFF),
    ov!(0x0058, 0xFF),
    ov!(0x0059, 0xFF),
    ov!(0x005A, 0x7F),
    ov!(0x007F, 0xC2),
    ov!(0x0080, 0x08),
];
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_GETCAPS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0015, 0x68),
    ov!(0x0036, 0x44),
    ov!(0x0037, 0x24),
    ov!(0x0038, 0x18),
    ov!(0x0039, 0x8B),
    ov!(0x003A, 0x48),
    ov!(0x003B, 0x0C),
    ov!(0x003C, 0x56),
    ov!(0x003D, 0xE8),
    ov!(0x006C, 0xC2),
    ov!(0x006D, 0x08),
];
static DSOUND_CDIRECTSOUND_GETCAPS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x80),
    ov!(0x0028, 0xEB),
    ov!(0x0029, 0x2F),
    ov!(0x002A, 0x8B),
    ov!(0x002B, 0x45),
    ov!(0x0040, 0xE8),
    ov!(0x005B, 0xC2),
    ov!(0x005C, 0x14),
];
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x003F, 0x80),
    ov!(0x0040, 0x88),
    ov!(0x0041, 0x84),
    ov!(0x0042, 0x00),
    ov!(0x0043, 0x00),
    ov!(0x0044, 0x00),
    ov!(0x0045, 0xFF),
    ov!(0x0046, 0xF6),
    ov!(0x0047, 0x44),
    ov!(0x0048, 0x24),
    ov!(0x0049, 0x18),
    ov!(0x004A, 0x01),
    ov!(0x0067, 0xC2),
    ov!(0x0068, 0x0C),
];
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0032, 0x89),
    ov!(0x0033, 0x50),
    ov!(0x0034, 0x48),
    ov!(0x0035, 0x8B),
    ov!(0x0036, 0x41),
    ov!(0x0037, 0x08),
    ov!(0x0038, 0x83),
    ov!(0x0039, 0x88),
    ov!(0x003A, 0x84),
    ov!(0x003E, 0x60),
    ov!(0x004E, 0x74),
    ov!(0x004F, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0032, 0x89),
    ov!(0x0033, 0x50),
    ov!(0x0034, 0x50),
    ov!(0x0038, 0x83),
    ov!(0x0039, 0x88),
    ov!(0x003A, 0x84),
    ov!(0x003E, 0x40),
    ov!(0x004E, 0x74),
    ov!(0x004F, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETEFFECTDATA_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x80),
    ov!(0x0028, 0xEB),
    ov!(0x0029, 0x32),
    ov!(0x002A, 0x8B),
    ov!(0x002B, 0x45),
    ov!(0x0043, 0xE8),
    ov!(0x005E, 0xC2),
    ov!(0x005F, 0x18),
];
static DSOUND_CDIRECTSOUND_SETEFFECTDATA_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0032, 0x8B),
    ov!(0x0034, 0x08),
    ov!(0x0035, 0x83),
    ov!(0x0038, 0xFF),
    ov!(0x0039, 0x75),
    ov!(0x003A, 0x09),
    ov!(0x003B, 0xC7),
    ov!(0x003E, 0x32),
    ov!(0x003F, 0x00),
    ov!(0x0040, 0x78),
    ov!(0x0041, 0x88),
];
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0033, 0x8B),
    ov!(0x0034, 0x7C),
    ov!(0x0035, 0x24),
    ov!(0x0036, 0x14),
    ov!(0x005C, 0xC2),
    ov!(0x005D, 0x0C),
];
static DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0043,
    target: "CMcpxAPU_SetMixBinHeadroom",
}];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETORIENTATION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0030, 0x8B),
    ov!(0x0032, 0x0C),
    ov!(0x0039, 0x8B),
    ov!(0x003B, 0x10),
    ov!(0x0042, 0x8B),
    ov!(0x0044, 0x14),
    ov!(0x004B, 0x8B),
    ov!(0x004D, 0x18),
    ov!(0x0054, 0x8B),
    ov!(0x0056, 0x1C),
    ov!(0x005D, 0x8B),
    ov!(0x005F, 0x20),
];
static DSOUND_CDIRECTSOUND_SETORIENTATION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002A, 0x8B),
    ov!(0x002B, 0x4D),
    ov!(0x002C, 0x08),
    ov!(0x003E, 0x89),
    ov!(0x003F, 0x7A),
    ov!(0x0040, 0x1C),
    ov!(0x004B, 0x80),
    ov!(0x004C, 0x88),
    ov!(0x004D, 0x84),
    ov!(0x004E, 0x00),
    ov!(0x0051, 0xFF),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x14),
];
static DSOUND_CDIRECTSOUND_SETPOSITION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0020, 0xB8),
    ov!(0x0021, 0x05),
    ov!(0x0022, 0x40),
    ov!(0x0023, 0x00),
    ov!(0x0024, 0x80),
    ov!(0x0032, 0x89),
    ov!(0x0033, 0x50),
    ov!(0x0034, 0x4C),
    ov!(0x0035, 0x8B),
    ov!(0x0036, 0x41),
    ov!(0x0037, 0x08),
    ov!(0x0038, 0x83),
    ov!(0x0039, 0x88),
    ov!(0x003A, 0x84),
    ov!(0x003E, 0x04),
    ov!(0x004E, 0x74),
    ov!(0x004F, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CDIRECTSOUND_SETVELOCITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0026, 0x00),
    ov!(0x0027, 0x80),
    ov!(0x0036, 0x89),
    ov!(0x0037, 0x7A),
    ov!(0x0038, 0x24),
    ov!(0x004B, 0x83),
    ov!(0x004C, 0x88),
    ov!(0x004D, 0x84),
    ov!(0x004F, 0x00),
    ov!(0x0051, 0x40),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x14),
];
static DSOUND_CDIRECTSOUND_SETVELOCITY_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0022, 0xD9),
    ov!(0x0023, 0xE1),
    ov!(0x00CB, 0xF0),
    ov!(0x00CC, 0xD8),
    ov!(0x00CD, 0xFF),
    ov!(0x00CE, 0xFF),
    ov!(0x00D1, 0xC2),
    ov!(0x00D2, 0x04),
];
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0038,
    target: "CFullHRTFSource_GetCenterVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0038,
    target: "CLightHRTFSource_GetCenterVolume",
}];

// Source: DSound/4039.inl
static DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0022, 0xD9),
    ov!(0x0023, 0xE1),
    ov!(0x00B3, 0xF0),
    ov!(0x00B4, 0xD8),
    ov!(0x00B5, 0xFF),
    ov!(0x00B6, 0xFF),
    ov!(0x00B9, 0xC2),
    ov!(0x00BA, 0x04),
];
static DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXAPU_SETMIXBINHEADROOM_4_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0014, 0xA1),
    ov!(0x0015, 0x10),
    ov!(0x0016, 0x00),
    ov!(0x0017, 0x82),
    ov!(0x0018, 0xFE),
    ov!(0x0019, 0x83),
    ov!(0x001A, 0xE0),
    ov!(0x001B, 0xFC),
    ov!(0x001C, 0x83),
    ov!(0x001D, 0xF8),
    ov!(0x001E, 0x04),
];
static DSOUND_CMCPXAPU_SETMIXBINHEADROOM_4_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x86),
    ov!(0x0018, 0x90),
    ov!(0x0019, 0x00),
    ov!(0x0021, 0x0F),
    ov!(0x0022, 0x85),
    ov!(0x0023, 0x88),
    ov!(0x0024, 0x00),
    ov!(0x00A5, 0xF7),
    ov!(0x00A6, 0xB1),
    ov!(0x00A7, 0x40),
    ov!(0x00A8, 0x01),
    ov!(0x00A9, 0x00),
    ov!(0x00AA, 0x00),
    ov!(0x00DB, 0xC2),
    ov!(0x00DC, 0x08),
    ov!(0x00DD, 0x00),
];
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXBUFFER_GETSTATUS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x4D),
    ov!(0x0016, 0x08),
    ov!(0x001A, 0x33),
    ov!(0x001B, 0xD2),
    ov!(0x0022, 0x42),
    ov!(0x0026, 0x74),
    ov!(0x0027, 0x15),
    ov!(0x0037, 0xC7),
    ov!(0x0038, 0x01),
    ov!(0x0039, 0x05),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static DSOUND_CMCPXBUFFER_GETSTATUS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXBUFFER_PLAY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0014, 0x3B),
    ov!(0x0015, 0xC3),
    ov!(0x0025, 0xF6),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x09),
    ov!(0x0028, 0x20),
    ov!(0x0029, 0x74),
    ov!(0x002A, 0x0C),
    ov!(0x003F, 0xF6),
    ov!(0x0040, 0x45),
    ov!(0x0041, 0x08),
    ov!(0x0042, 0x02),
];
static DSOUND_CMCPXBUFFER_PLAY_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXBUFFER_PLAY_EX_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0011, 0xFF),
    ov!(0x0012, 0x75),
    ov!(0x0013, 0x10),
    ov!(0x0023, 0x75),
    ov!(0x0024, 0x0C),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x0C),
];
static DSOUND_CMCPXBUFFER_PLAY_EX_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002B,
    target: "CMcpxBuffer_Play",
}];

// Source: DSound/4039.inl
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0011, 0x8D),
    ov!(0x0012, 0x86),
    ov!(0x0013, 0x90),
    ov!(0x0043, 0x72),
    ov!(0x0044, 0x10),
    ov!(0x006A, 0x8B),
    ov!(0x006B, 0x86),
    ov!(0x006C, 0xE4),
    ov!(0x006D, 0x00),
];
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0015, 0x8B),
    ov!(0x0017, 0xE8),
    ov!(0x001E, 0x5E),
    ov!(0x001F, 0xC3),
];
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0018,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/4039.inl
static DSOUND_CMCPXSTREAM_FLUSH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0009, 0x33),
    ov!(0x0032, 0x83),
    ov!(0x0033, 0xFF),
    ov!(0x0034, 0x03),
    ov!(0x004A, 0xE8),
    ov!(0x0058, 0xE8),
    ov!(0x0098, 0xC9),
    ov!(0x0099, 0xC3),
];
static DSOUND_CMCPXSTREAM_FLUSH_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXSTREAM_PAUSE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0020, 0x83),
    ov!(0x0021, 0xC8),
    ov!(0x0022, 0x04),
    ov!(0x0023, 0xEB),
    ov!(0x0024, 0x0F),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x86),
    ov!(0x002D, 0x90),
    ov!(0x002E, 0x00),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static DSOUND_CMCPXSTREAM_PAUSE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0042, 0x83),
    ov!(0x0043, 0xFF),
    ov!(0x0044, 0x02),
    ov!(0x0053, 0x8B),
    ov!(0x0055, 0xF4),
    ov!(0x0056, 0x00),
    ov!(0x0058, 0x00),
    ov!(0x0073, 0x8B),
    ov!(0x0075, 0xF4),
    ov!(0x0076, 0x00),
    ov!(0x0078, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETEG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0xEB),
    ov!(0x003C, 0x84),
    ov!(0x003D, 0x86),
    ov!(0x003E, 0x00),
    ov!(0x003F, 0x00),
    ov!(0x0040, 0x00),
    ov!(0x0041, 0x8B),
    ov!(0x0042, 0x86),
    ov!(0x0043, 0xE4),
    ov!(0x00D3, 0xC2),
    ov!(0x00D4, 0x04),
];
static DSOUND_CMCPXVOICECLIENT_SETEG_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0xF8),
    ov!(0x0012, 0xE8),
    ov!(0x0020, 0x00),
    ov!(0x0021, 0x74),
    ov!(0x0022, 0x13),
    ov!(0x0023, 0x6A),
    ov!(0x0024, 0x06),
    ov!(0x0025, 0x59),
    ov!(0x0026, 0x8D),
    ov!(0x0027, 0x7D),
    ov!(0x0047, 0x83),
    ov!(0x004D, 0x8B),
];
static DSOUND_CMCPXVOICECLIENT_SETFILTER_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0xEB),
    ov!(0x0057, 0x72),
    ov!(0x0058, 0xF3),
    ov!(0x0059, 0x33),
    ov!(0x005A, 0xC0),
    ov!(0x005B, 0x85),
    ov!(0x005C, 0xC9),
    ov!(0x005D, 0x76),
    ov!(0x005E, 0x62),
    ov!(0x005F, 0x8B),
    ov!(0x00CD, 0xC2),
    ov!(0x00CE, 0x04),
];
static DSOUND_CMCPXVOICECLIENT_SETLFO_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0017, 0xF6),
    ov!(0x0018, 0x86),
    ov!(0x0019, 0x90),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x00),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x01),
    ov!(0x001E, 0x0F),
    ov!(0x001F, 0x84),
    ov!(0x0020, 0x98),
    ov!(0x0021, 0x00),
    ov!(0x0022, 0x00),
    ov!(0x0023, 0x00),
    ov!(0x0066, 0xA3),
    ov!(0x0067, 0xF8),
    ov!(0x0068, 0x02),
    ov!(0x0069, 0x82),
    ov!(0x006A, 0xFE),
    ov!(0x00C8, 0xC9),
    ov!(0x00C9, 0xC3),
];
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0x8D),
    ov!(0x0026, 0xBE),
    ov!(0x0027, 0xC4),
    ov!(0x0028, 0x00),
    ov!(0x005C, 0x8D),
    ov!(0x005D, 0x0C),
    ov!(0x005E, 0x00),
    ov!(0x0072, 0x8D),
    ov!(0x0073, 0x86),
    ov!(0x0074, 0x84),
    ov!(0x0075, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_SETPITCH_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0015, 0xF6),
    ov!(0x0016, 0x86),
    ov!(0x0017, 0x90),
    ov!(0x0018, 0x00),
    ov!(0x0019, 0x00),
    ov!(0x001A, 0x00),
    ov!(0x001B, 0x01),
    ov!(0x001C, 0x74),
    ov!(0x001D, 0x75),
    ov!(0x0073, 0x8B),
    ov!(0x0074, 0x54),
    ov!(0x0075, 0x85),
    ov!(0x0076, 0xEC),
];
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_DSOUND_CREFCOUNT_ADDREF_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x40),
    ov!(0x0006, 0x04),
    ov!(0x0007, 0x8B),
    ov!(0x0008, 0x40),
    ov!(0x0009, 0x04),
    ov!(0x000A, 0xC2),
    ov!(0x000B, 0x04),
];
static DSOUND_DSOUND_CREFCOUNT_ADDREF_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_DSOUND_CREFCOUNT_RELEASE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000B, 0x48),
    ov!(0x000E, 0x89),
    ov!(0x000F, 0x41),
    ov!(0x0010, 0x04),
    ov!(0x001D, 0x8B),
    ov!(0x001E, 0x41),
    ov!(0x001F, 0x04),
    ov!(0x0020, 0xC2),
    ov!(0x0021, 0x04),
];
static DSOUND_DSOUND_CREFCOUNT_RELEASE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDCREATE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0x8B),
    ov!(0x000C, 0xF0),
    ov!(0x0011, 0xE8),
    ov!(0x001A, 0x7C),
    ov!(0x001B, 0x13),
    ov!(0x0026, 0x1B),
    ov!(0x0027, 0xC9),
    ov!(0x0043, 0xC2),
    ov!(0x0044, 0x0C),
];
static DSOUND_DIRECTSOUNDCREATE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDCREATEBUFFER_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0008, 0x56),
    ov!(0x0009, 0x57),
    ov!(0x0032, 0x8D),
    ov!(0x0033, 0x45),
    ov!(0x0034, 0xFC),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_DIRECTSOUNDCREATEBUFFER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002C,
    target: "CDirectSound_CreateSoundBuffer",
}];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDCREATESTREAM_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0008, 0x56),
    ov!(0x0009, 0x57),
    ov!(0x0032, 0x8D),
    ov!(0x0033, 0x45),
    ov!(0x0034, 0xFC),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_DIRECTSOUNDCREATESTREAM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002C,
    target: "CDirectSound_CreateSoundStream",
}];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0009, 0x81),
    ov!(0x000A, 0xE1),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0xFF),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x00),
    ov!(0x0024, 0xC2),
    ov!(0x0025, 0x04),
];
static DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "DirectSoundEnterCriticalSection",
}];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0xF0),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xF6),
    ov!(0x0011, 0x0B),
    ov!(0x0017, 0xFF),
    ov!(0x001D, 0xC3),
];
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "CHRTFSource_SetFullHRTF5Channel",
}];

// Source: DSound/4039.inl
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x8B),
    ov!(0x0007, 0xF0),
    ov!(0x000D, 0x85),
    ov!(0x000E, 0xF6),
    ov!(0x0011, 0x0B),
    ov!(0x0017, 0xFF),
    ov!(0x001D, 0xC3),
];
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0009,
    target: "CHRTFSource_SetLightHRTF5Channel",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetAllParameters",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xE4),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetConeAngles",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0xFF),
    ov!(0x000B, 0x74),
    ov!(0x000C, 0x24),
    ov!(0x000D, 0x0C),
    ov!(0x0015, 0x23),
    ov!(0x0016, 0xC8),
    ov!(0x001E, 0x0C),
    ov!(0x001F, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetConeOutsideVolume",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETEG_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETEG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetEG",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[
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
static DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetFilter",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETFORMAT_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetFormat",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x000F, 0x1B),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x23),
    ov!(0x0012, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetFrequency",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[
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
static DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetHeadroom",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x000E, 0x83),
    ov!(0x000F, 0xC0),
    ov!(0x0010, 0xE4),
    ov!(0x001E, 0x0C),
    ov!(0x001F, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetI3DL2Source",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETLFO_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetLFO",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xFF),
    ov!(0x0005, 0x74),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x08),
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x000F, 0x1B),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x23),
    ov!(0x0012, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetMixBinVolumes",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x83),
    ov!(0x000B, 0xC0),
    ov!(0x000C, 0xE4),
    ov!(0x000F, 0x1B),
    ov!(0x0010, 0xC9),
    ov!(0x0011, 0x23),
    ov!(0x0012, 0xC8),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetMixBins",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETMODE_4039_ENTRIES: &[OovpaEntry] = &[
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
static DSOUND_IDIRECTSOUNDBUFFER_SETMODE_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetMode",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[
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
static DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetPitch",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETPLAYREGION_4039_ENTRIES: &[OovpaEntry] = &[
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
static DSOUND_IDIRECTSOUNDBUFFER_SETPLAYREGION_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSoundBuffer_SetPlayRegion",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETEG_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETEG_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetEG",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetFilter",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETFORMAT_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETFORMAT_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetFormat",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetFrequency",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetHeadroom",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETLFO_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETLFO_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetLFO",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetMixBinVolumes_8",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetMixBins",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetPitch",
}];

// Source: DSound/4039.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_4039_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_4039_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetVolume",
}];

// Source: DSound/4039.inl
static DSOUND_ISVALIDFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0014, 0x74),
    ov!(0x0015, 0x04),
    ov!(0x0016, 0x33),
    ov!(0x0017, 0xC0),
    ov!(0x0018, 0xEB),
    ov!(0x0019, 0x16),
    ov!(0x001A, 0x51),
    ov!(0x001B, 0xE8),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x04),
];
static DSOUND_ISVALIDFORMAT_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_XAUDIOCALCULATEPITCH_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0010, 0x3D),
    ov!(0x0011, 0x80),
    ov!(0x0012, 0xBB),
    ov!(0x0013, 0x00),
    ov!(0x0019, 0xEB),
    ov!(0x001A, 0x21),
    ov!(0x003C, 0x8D),
    ov!(0x003D, 0x4D),
    ov!(0x003E, 0xFC),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static DSOUND_XAUDIOCALCULATEPITCH_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_XAUDIOCREATEADPCMFORMAT_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0007, 0x08),
    ov!(0x0010, 0xE9),
    ov!(0x0019, 0x8D),
    ov!(0x0027, 0x66),
    ov!(0x0028, 0xC7),
    ov!(0x0029, 0x40),
    ov!(0x002A, 0x0E),
    ov!(0x002B, 0x04),
    ov!(0x002C, 0x00),
    ov!(0x002D, 0x66),
    ov!(0x0034, 0x66),
    ov!(0x003D, 0x12),
];
static DSOUND_XAUDIOCREATEADPCMFORMAT_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4039.inl
static DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4039_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000A, 0x33),
    ov!(0x000B, 0xFF),
    ov!(0x000C, 0x89),
    ov!(0x000D, 0x75),
    ov!(0x000E, 0xF8),
    ov!(0x000F, 0xE8),
    ov!(0x003C, 0x83),
    ov!(0x004E, 0xE8),
    ov!(0x0065, 0x8B),
];
static DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4039_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0041, 0x74),
    ov!(0x0042, 0x0B),
    ov!(0x0052, 0xC2),
    ov!(0x0053, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0039,
    target: "CMcpxBuffer_GetCurrentPosition",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x003D, 0x74),
    ov!(0x003E, 0x0B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxBuffer_GetStatus",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0037, 0xF6),
    ov!(0x0038, 0x45),
    ov!(0x0039, 0x24),
    ov!(0x003A, 0x01),
    ov!(0x0053, 0xF6),
    ov!(0x0054, 0x45),
    ov!(0x0055, 0x24),
    ov!(0x0056, 0x02),
    ov!(0x0081, 0x2B),
    ov!(0x0082, 0x4D),
    ov!(0x0083, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_LOCK_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0048,
    target: "CDirectSoundBuffer_GetCurrentPosition",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x003D, 0x74),
    ov!(0x003E, 0x0B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxBuffer_Play",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0016, 0x68),
    ov!(0x0022, 0x05),
    ov!(0x002E, 0x20),
    ov!(0x003A, 0x24),
    ov!(0x0046, 0x0B),
    ov!(0x0052, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CMcpxBuffer_Play_Ex",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x74),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0031, 0xFF),
    ov!(0x0032, 0x74),
    ov!(0x0033, 0x24),
    ov!(0x0034, 0x18),
    ov!(0x0054, 0x10),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0xD9),
    ov!(0x002C, 0x45),
    ov!(0x002D, 0x14),
    ov!(0x0043, 0xD9),
    ov!(0x0044, 0x1C),
    ov!(0x0045, 0x24),
    ov!(0x0065, 0x14),
    ov!(0x0066, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004A,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0031, 0xFF),
    ov!(0x0032, 0x74),
    ov!(0x0033, 0x24),
    ov!(0x0034, 0x14),
    ov!(0x0050, 0x0C),
    ov!(0x0051, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x003D, 0x74),
    ov!(0x003E, 0x0B),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxBuffer_SetCurrentPosition",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0032, 0xD9),
    ov!(0x0033, 0x1C),
    ov!(0x0034, 0x24),
    ov!(0x0054, 0x0C),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetDistanceFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x002D, 0xFF),
    ov!(0x002E, 0x74),
    ov!(0x002F, 0x24),
    ov!(0x0030, 0x14),
    ov!(0x0032, 0xD9),
    ov!(0x0033, 0x1C),
    ov!(0x0034, 0x24),
    ov!(0x0054, 0x0C),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetDopplerFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETEG_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetFormat",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0031, 0xFF),
    ov!(0x0032, 0x74),
    ov!(0x0033, 0x24),
    ov!(0x0034, 0x14),
    ov!(0x0050, 0x0C),
    ov!(0x0051, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0024, 0xB8),
    ov!(0x0025, 0x05),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x80),
    ov!(0x0029, 0xEB),
    ov!(0x002A, 0x55),
    ov!(0x0056, 0x2B),
    ov!(0x0057, 0xCE),
    ov!(0x0082, 0xC2),
    ov!(0x0083, 0x0C),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0032, 0xD9),
    ov!(0x0033, 0x1C),
    ov!(0x0034, 0x24),
    ov!(0x0054, 0x0C),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0032, 0xD9),
    ov!(0x0033, 0x1C),
    ov!(0x0034, 0x24),
    ov!(0x0054, 0x0C),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x74),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0012, 0x85),
    ov!(0x001C, 0x15),
    ov!(0x0026, 0xEB),
    ov!(0x0030, 0x10),
    ov!(0x003A, 0x74),
    ov!(0x0047, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0010, 0x0F),
    ov!(0x0011, 0xB6),
    ov!(0x0012, 0xF8),
    ov!(0x0017, 0x74),
    ov!(0x0018, 0x0B),
    ov!(0x0046, 0xBE),
    ov!(0x0047, 0x32),
    ov!(0x0048, 0x00),
    ov!(0x0049, 0x78),
    ov!(0x004A, 0x88),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0xD9),
    ov!(0x002C, 0x45),
    ov!(0x002D, 0x14),
    ov!(0x0043, 0xD9),
    ov!(0x0044, 0x1C),
    ov!(0x0045, 0x24),
    ov!(0x0065, 0x14),
    ov!(0x0066, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004A,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0032, 0xD9),
    ov!(0x0033, 0x1C),
    ov!(0x0034, 0x24),
    ov!(0x0054, 0x0C),
    ov!(0x0055, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetRolloffFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0xD9),
    ov!(0x002C, 0x45),
    ov!(0x002D, 0x14),
    ov!(0x0043, 0xD9),
    ov!(0x0044, 0x1C),
    ov!(0x0045, 0x24),
    ov!(0x0065, 0x14),
    ov!(0x0066, 0x00),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004A,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x0F),
    ov!(0x000E, 0xB6),
    ov!(0x000F, 0xF0),
    ov!(0x0026, 0xEB),
    ov!(0x0027, 0x22),
    ov!(0x002D, 0xFF),
    ov!(0x002E, 0x74),
    ov!(0x002F, 0x24),
    ov!(0x0030, 0x10),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOP_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x002C, 0x8B),
    ov!(0x002D, 0x48),
    ov!(0x002E, 0x20),
    ov!(0x003B, 0x74),
    ov!(0x003C, 0x0B),
    ov!(0x004C, 0xC2),
    ov!(0x004D, 0x04),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOP_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0033,
    target: "CMcpxBuffer_Stop",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0016, 0x68),
    ov!(0x0022, 0x05),
    ov!(0x002E, 0x20),
    ov!(0x003A, 0x24),
    ov!(0x0046, 0x0B),
    ov!(0x0052, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CMcpxBuffer_Stop_Ex",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0027, 0x8B),
    ov!(0x0028, 0x44),
    ov!(0x002B, 0xFF),
    ov!(0x002C, 0x40),
    ov!(0x002D, 0x08),
    ov!(0x0031, 0x8B),
    ov!(0x0032, 0x70),
    ov!(0x0033, 0x08),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0x8B),
    ov!(0x002B, 0x08),
    ov!(0x002C, 0x8B),
    ov!(0x002E, 0x24),
    ov!(0x0030, 0xE8),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CMcpxStream_Discontinuity",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0x8B),
    ov!(0x002B, 0x08),
    ov!(0x002C, 0x8B),
    ov!(0x002E, 0x24),
    ov!(0x0030, 0xE8),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_FLUSHEX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0x83),
    ov!(0x002C, 0x7D),
    ov!(0x002D, 0x14),
    ov!(0x002E, 0x00),
    ov!(0x003E, 0xFF),
    ov!(0x003F, 0x75),
    ov!(0x0041, 0xE8),
];
static DSOUND_CDIRECTSOUNDSTREAM_FLUSHEX_4134_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0042,
        target: "CMcpxStream_Stop_Ex",
    },
    OovpaXref {
        offset: 0x004C,
        target: "CMcpxStream_Flush",
    },
    OovpaXref {
        offset: 0x0049,
        target: "CMcpxStream_Flush",
    },
];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0032, 0xC7),
    ov!(0x0033, 0x06),
    ov!(0x0034, 0x05),
    ov!(0x0035, 0x00),
    ov!(0x0036, 0x00),
    ov!(0x0037, 0x00),
    ov!(0x0049, 0x83),
    ov!(0x004A, 0x66),
    ov!(0x004B, 0x08),
    ov!(0x004C, 0x00),
    ov!(0x0064, 0xC2),
    ov!(0x0065, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x002C, 0x8B),
    ov!(0x002D, 0x48),
    ov!(0x002E, 0x24),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxStream_GetStatus",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x002C, 0x8B),
    ov!(0x002D, 0x48),
    ov!(0x002E, 0x24),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxStream_Pause",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002D, 0x83),
    ov!(0x002E, 0xC0),
    ov!(0x002F, 0x04),
    ov!(0x0047, 0x8B),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x04),
    ov!(0x004D, 0x00),
];
static DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "DSound_CRefCount_Release",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0039, 0xE8),
    ov!(0x0044, 0x68),
    ov!(0x004F, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetAllParameters",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x0030, 0x18),
    ov!(0x0033, 0x04),
    ov!(0x0037, 0x18),
    ov!(0x003D, 0xE8),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetConeAngles",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0017, 0x74),
    ov!(0x0024, 0xB8),
    ov!(0x002A, 0x3B),
    ov!(0x0039, 0xEC),
    ov!(0x0044, 0x24),
    ov!(0x004D, 0xE8),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004E,
    target: "CDirectSoundVoice_SetConeOrientation",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0021, 0xB8),
    ov!(0x0024, 0x00),
    ov!(0x0039, 0xE8),
    ov!(0x003E, 0x85),
    ov!(0x004F, 0x8B),
    ov!(0x0053, 0xC2),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetConeOutsideVolume",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x003D, 0xE8),
    ov!(0x0048, 0x68),
    ov!(0x0053, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetDistanceFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x003D, 0xE8),
    ov!(0x0048, 0x68),
    ov!(0x0053, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetDopplerFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETEG_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetEG",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetFilter",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetFormat",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0015, 0x0B),
    ov!(0x0026, 0xEB),
    ov!(0x0027, 0x26),
    ov!(0x002A, 0x24),
    ov!(0x003E, 0x74),
    ov!(0x003F, 0x0B),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetFrequency",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetHeadroom",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0039, 0xE8),
    ov!(0x0044, 0x68),
    ov!(0x004F, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetI3DL2Source",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x000F, 0xF0),
    ov!(0x0012, 0x85),
    ov!(0x0015, 0x0B),
    ov!(0x003A, 0x85),
    ov!(0x003D, 0xF8),
    ov!(0x0040, 0x68),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetLFO",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x003D, 0xE8),
    ov!(0x0048, 0x68),
    ov!(0x0053, 0x8B),
    ov!(0x0057, 0xC2),
    ov!(0x0058, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetMaxDistance",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0039, 0xD9),
    ov!(0x003A, 0x1C),
    ov!(0x003B, 0x24),
    ov!(0x0057, 0xC2),
    ov!(0x0058, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetMinDistance",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x000E, 0xB6),
    ov!(0x000F, 0xF0),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetMixBinVolumes",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetMixBins",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0039, 0xE8),
    ov!(0x0044, 0x68),
    ov!(0x004F, 0x8B),
    ov!(0x0053, 0xC2),
    ov!(0x0054, 0x0C),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetMode",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetOutputBuffer",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
    ov!(0x0051, 0x00),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetPitch",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0017, 0x74),
    ov!(0x0024, 0xB8),
    ov!(0x002A, 0x3B),
    ov!(0x0039, 0xEC),
    ov!(0x0058, 0x68),
    ov!(0x0063, 0x8B),
    ov!(0x0068, 0xC2),
    ov!(0x0069, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004E,
    target: "CDirectSoundVoice_SetPosition",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x003D, 0xE8),
    ov!(0x0048, 0x68),
    ov!(0x0053, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetRolloffFactor",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0017, 0x74),
    ov!(0x0024, 0xB8),
    ov!(0x002A, 0x3B),
    ov!(0x0035, 0x83),
    ov!(0x0040, 0x45),
    ov!(0x0063, 0x8B),
    ov!(0x0068, 0xC2),
    ov!(0x0069, 0x14),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004E,
    target: "CDirectSoundVoice_SetVelocity",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0035, 0xE8),
    ov!(0x0040, 0x68),
    ov!(0x004B, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_SetVolume",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0009, 0x76),
    ov!(0x000A, 0x16),
    ov!(0x0018, 0x89),
    ov!(0x0019, 0x44),
    ov!(0x001A, 0xB9),
    ov!(0x001B, 0x30),
    ov!(0x001E, 0x72),
    ov!(0x001F, 0xEC),
    ov!(0x0022, 0xC2),
    ov!(0x0023, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000E, 0x8B),
    ov!(0x000F, 0x89),
    ov!(0x0010, 0xA4),
    ov!(0x0011, 0x00),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x80),
    ov!(0x0016, 0xB4),
    ov!(0x0017, 0x00),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0021,
    target: "CMcpxVoiceClient_Commit3dSettings",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x8B),
    ov!(0x0063, 0x8B),
    ov!(0x0066, 0x8B),
    ov!(0x0083, 0x24),
    ov!(0x0084, 0x8B),
    ov!(0x0085, 0x50),
    ov!(0x0086, 0x10),
    ov!(0x0087, 0x8B),
    ov!(0x0088, 0x92),
    ov!(0x0089, 0xB4),
    ov!(0x008A, 0x00),
    ov!(0x008B, 0x00),
    ov!(0x008C, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x51),
    ov!(0x0013, 0x1C),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x48),
    ov!(0x0016, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x89),
    ov!(0x0023, 0x20),
    ov!(0x0030, 0x10),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x24),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x28),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x2C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x30),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x40),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x48),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x31),
    ov!(0x0014, 0x89),
    ov!(0x0015, 0x72),
    ov!(0x0016, 0x4C),
    ov!(0x0044, 0xD9),
    ov!(0x0045, 0x41),
    ov!(0x0046, 0x10),
    ov!(0x00AD, 0x0C),
    ov!(0x00AE, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x00A6,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x38),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x34),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xF6),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x0C),
    ov!(0x0004, 0x01),
    ov!(0x0005, 0x8B),
    ov!(0x0006, 0x44),
    ov!(0x0007, 0x24),
    ov!(0x0019, 0x75),
    ov!(0x001C, 0xE8),
    ov!(0x0023, 0xC2),
    ov!(0x0024, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x04),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x08),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x44),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x70),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x10),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x14),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x18),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x4C),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x000B, 0x2B),
    ov!(0x000C, 0x50),
    ov!(0x000D, 0x20),
    ov!(0x0011, 0x8B),
    ov!(0x0012, 0x49),
    ov!(0x0013, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CMcpxVoiceClient_SetVolume",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0026, 0xEB),
    ov!(0x0027, 0x24),
    ov!(0x0028, 0x8B),
    ov!(0x0029, 0x44),
    ov!(0x0032, 0x6A),
    ov!(0x0034, 0x6A),
    ov!(0x0036, 0xE8),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x04),
];
static DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0027, 0x80),
    ov!(0x002A, 0x6A),
    ov!(0x002B, 0x24),
    ov!(0x004A, 0x81),
    ov!(0x004B, 0xE6),
    ov!(0x004C, 0xF2),
    ov!(0x004D, 0xFF),
    ov!(0x004E, 0xF8),
    ov!(0x004F, 0x7F),
    ov!(0x0099, 0xC2),
    ov!(0x009A, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0023, 0xB8),
    ov!(0x0024, 0x05),
    ov!(0x0025, 0x40),
    ov!(0x0027, 0x80),
    ov!(0x002A, 0x6A),
    ov!(0x002B, 0x28),
    ov!(0x004A, 0x81),
    ov!(0x004B, 0xE6),
    ov!(0x004C, 0xF2),
    ov!(0x004D, 0xFF),
    ov!(0x004E, 0xF8),
    ov!(0x004F, 0x7F),
    ov!(0x008E, 0xC2),
    ov!(0x008F, 0x10),
];
static DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002C, 0x8B),
    ov!(0x002D, 0x75),
    ov!(0x002E, 0x08),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0x75),
    ov!(0x0031, 0x14),
    ov!(0x0032, 0x8B),
    ov!(0x0033, 0x4E),
    ov!(0x0034, 0x08),
    ov!(0x003A, 0xFF),
    ov!(0x003C, 0x18),
    ov!(0x0040, 0xFF),
    ov!(0x0042, 0x10),
    ov!(0x0043, 0xFF),
    ov!(0x0045, 0x0C),
    ov!(0x0061, 0xC2),
    ov!(0x0062, 0x14),
];
static DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0029, 0xB8),
    ov!(0x004C, 0x83),
    ov!(0x004D, 0x7D),
    ov!(0x004E, 0x0C),
    ov!(0x004F, 0x00),
    ov!(0x0050, 0x74),
    ov!(0x0051, 0x07),
    ov!(0x0052, 0x0D),
    ov!(0x0053, 0x00),
    ov!(0x0063, 0x0C),
    ov!(0x0064, 0xE8),
];
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_GETCAPS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x57),
    ov!(0x0016, 0x68),
    ov!(0x0037, 0x44),
    ov!(0x0038, 0x24),
    ov!(0x0039, 0x18),
    ov!(0x003A, 0x8B),
    ov!(0x003B, 0x48),
    ov!(0x003C, 0x0C),
    ov!(0x003D, 0x56),
    ov!(0x003E, 0xE8),
    ov!(0x006D, 0xC2),
    ov!(0x006E, 0x08),
];
static DSOUND_CDIRECTSOUND_GETCAPS_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x00),
    ov!(0x0024, 0xB8),
    ov!(0x0025, 0x05),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x80),
    ov!(0x0029, 0xEB),
    ov!(0x002A, 0x2F),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x45),
    ov!(0x0041, 0xE8),
    ov!(0x005C, 0xC2),
    ov!(0x005D, 0x14),
];
static DSOUND_CDIRECTSOUND_GETEFFECTDATA_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0xE8),
    ov!(0x000C, 0x00),
    ov!(0x0021, 0xB8),
    ov!(0x002B, 0x8B),
    ov!(0x002D, 0x24),
    ov!(0x0032, 0x8B),
    ov!(0x0034, 0x24),
];
static DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0002,
    target: "DirectSoundEnterCriticalSection",
}];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0035, 0x68),
    ov!(0x0036, 0x8B),
    ov!(0x0037, 0x41),
    ov!(0x0038, 0x08),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x88),
    ov!(0x003B, 0xA4),
    ov!(0x003F, 0x60),
    ov!(0x004F, 0x74),
    ov!(0x0050, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0035, 0x70),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x88),
    ov!(0x003B, 0xA4),
    ov!(0x003F, 0x40),
    ov!(0x004F, 0x74),
    ov!(0x0050, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0031, 0x8B),
    ov!(0x0033, 0x08),
    ov!(0x0034, 0x83),
    ov!(0x0037, 0xFF),
    ov!(0x0038, 0x75),
    ov!(0x0039, 0x0A),
    ov!(0x003A, 0xBF),
    ov!(0x003B, 0x32),
    ov!(0x003C, 0x00),
    ov!(0x003D, 0x78),
    ov!(0x003E, 0x88),
];
static DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETORIENTATION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0031, 0x8B),
    ov!(0x0033, 0x0C),
    ov!(0x003A, 0x8B),
    ov!(0x003C, 0x10),
    ov!(0x0043, 0x8B),
    ov!(0x0045, 0x14),
    ov!(0x004C, 0x8B),
    ov!(0x004E, 0x18),
    ov!(0x0055, 0x8B),
    ov!(0x0057, 0x1C),
    ov!(0x005E, 0x8B),
    ov!(0x0060, 0x20),
];
static DSOUND_CDIRECTSOUND_SETORIENTATION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x4D),
    ov!(0x002D, 0x08),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x7A),
    ov!(0x0039, 0x38),
    ov!(0x003F, 0x89),
    ov!(0x0040, 0x7A),
    ov!(0x0041, 0x3C),
    ov!(0x0047, 0x89),
    ov!(0x0048, 0x7A),
    ov!(0x0049, 0x40),
];
static DSOUND_CDIRECTSOUND_SETPOSITION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0036, 0x8B),
    ov!(0x0037, 0x41),
    ov!(0x0038, 0x08),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x88),
    ov!(0x003F, 0x04),
];
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CDIRECTSOUND_SETVELOCITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0024, 0xB8),
    ov!(0x0025, 0x05),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x80),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x7A),
    ov!(0x0039, 0x44),
    ov!(0x004C, 0x83),
    ov!(0x004D, 0x88),
    ov!(0x004E, 0xA4),
    ov!(0x004F, 0x00),
    ov!(0x0052, 0x40),
    ov!(0x0073, 0xC2),
    ov!(0x0074, 0x14),
];
static DSOUND_CDIRECTSOUND_SETVELOCITY_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0022, 0xD9),
    ov!(0x0062, 0xDF),
    ov!(0x00B6, 0xF0),
    ov!(0x00B7, 0xD8),
    ov!(0x00B8, 0xFF),
    ov!(0x00B9, 0xFF),
    ov!(0x00BC, 0xC2),
    ov!(0x00BD, 0x04),
];
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0017, 0x8D),
    ov!(0x0018, 0x4D),
    ov!(0x0019, 0xF0),
    ov!(0x001A, 0x89),
    ov!(0x001B, 0x45),
    ov!(0x001C, 0xF8),
];
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000E, 0x8D),
    ov!(0x000F, 0x4D),
    ov!(0x0010, 0xF8),
    ov!(0x0025, 0xE8),
    ov!(0x0031, 0x74),
    ov!(0x0032, 0x02),
    ov!(0x0044, 0xE8),
    ov!(0x005C, 0x66),
    ov!(0x005D, 0xF7),
    ov!(0x005E, 0x46),
    ov!(0x005F, 0x12),
    ov!(0x0060, 0x00),
    ov!(0x0061, 0x02),
];
static DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_GETSTATUS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000A, 0x8B),
    ov!(0x000B, 0xF1),
    ov!(0x0020, 0x74),
    ov!(0x0021, 0x18),
    ov!(0x0022, 0x66),
    ov!(0x0023, 0xF7),
    ov!(0x0024, 0x46),
    ov!(0x0025, 0x12),
    ov!(0x002C, 0x66),
    ov!(0x002D, 0xF7),
    ov!(0x002E, 0x46),
];
static DSOUND_CMCPXBUFFER_GETSTATUS_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_PLAY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000E, 0x3B),
    ov!(0x000F, 0xDF),
    ov!(0x0018, 0xF6),
    ov!(0x0019, 0x40),
    ov!(0x001A, 0x09),
    ov!(0x001B, 0x20),
    ov!(0x001C, 0x74),
    ov!(0x001D, 0x09),
    ov!(0x0030, 0xF6),
    ov!(0x0031, 0x46),
    ov!(0x0032, 0x12),
    ov!(0x0033, 0x02),
];
static DSOUND_CMCPXBUFFER_PLAY_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_SETBUFFERDATA_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x91),
    ov!(0x0006, 0x33),
    ov!(0x0007, 0xC0),
    ov!(0x000E, 0xF6),
    ov!(0x000F, 0x42),
    ov!(0x0011, 0x04),
    ov!(0x0014, 0xE9),
    ov!(0x0019, 0xC3),
];
static DSOUND_CMCPXBUFFER_SETBUFFERDATA_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0011, 0x8A),
    ov!(0x0012, 0x46),
    ov!(0x0013, 0x12),
    ov!(0x005F, 0x72),
    ov!(0x0060, 0x10),
    ov!(0x0086, 0x0F),
    ov!(0x0087, 0xB6),
    ov!(0x0088, 0x46),
    ov!(0x0089, 0x64),
];
static DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_STOP_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0x3C),
    ov!(0x000C, 0x03),
    ov!(0x001A, 0x75),
    ov!(0x001B, 0x59),
    ov!(0x001C, 0xF6),
    ov!(0x001D, 0x44),
    ov!(0x001E, 0x24),
    ov!(0x001F, 0x0C),
    ov!(0x0020, 0x02),
];
static DSOUND_CMCPXBUFFER_STOP_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXBUFFER_STOP_EX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0011, 0xFF),
    ov!(0x0012, 0x75),
    ov!(0x0013, 0x10),
    ov!(0x0023, 0x75),
    ov!(0x0024, 0x0C),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x0C),
];
static DSOUND_CMCPXBUFFER_STOP_EX_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002B,
    target: "CMcpxBuffer_Stop",
}];

// Source: DSound/4134.inl
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x001A, 0x8B),
    ov!(0x001C, 0xE8),
    ov!(0x0023, 0x5E),
    ov!(0x0024, 0xC3),
];
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/4134.inl
static DSOUND_CMCPXSTREAM_GETSTATUS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x002F, 0x03),
    ov!(0x0032, 0x03),
    ov!(0x003D, 0x80),
    ov!(0x003E, 0x48),
    ov!(0x003F, 0x02),
    ov!(0x0040, 0x02),
    ov!(0x004B, 0x81),
    ov!(0x004C, 0xC9),
    ov!(0x004D, 0x00),
    ov!(0x004E, 0x00),
    ov!(0x004F, 0x04),
    ov!(0x0050, 0x00),
];
static DSOUND_CMCPXSTREAM_GETSTATUS_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXSTREAM_PAUSE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x001E, 0x83),
    ov!(0x001F, 0xC8),
    ov!(0x0020, 0x04),
    ov!(0x0021, 0xEB),
    ov!(0x0022, 0x0D),
    ov!(0x002D, 0x83),
    ov!(0x002E, 0xE0),
    ov!(0x002F, 0xFB),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x04),
];
static DSOUND_CMCPXSTREAM_PAUSE_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXSTREAM_STOP_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0030, 0xF6),
    ov!(0x0031, 0x45),
    ov!(0x0032, 0x08),
    ov!(0x0033, 0x02),
];
static DSOUND_CMCPXSTREAM_STOP_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXSTREAM_STOP_EX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0011, 0xFF),
    ov!(0x0012, 0x75),
    ov!(0x0013, 0x10),
    ov!(0x0023, 0x75),
    ov!(0x0024, 0x0C),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x0C),
];
static DSOUND_CMCPXSTREAM_STOP_EX_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002B,
    target: "CMcpxStream_Stop",
}];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x003D, 0x83),
    ov!(0x003E, 0xFF),
    ov!(0x003F, 0x02),
    ov!(0x0059, 0x8B),
    ov!(0x005B, 0xB4),
    ov!(0x005C, 0x00),
    ov!(0x005E, 0x00),
    ov!(0x0066, 0x8B),
    ov!(0x0068, 0xB4),
    ov!(0x0069, 0x00),
    ov!(0x006B, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETEG_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0025, 0xEB),
    ov!(0x0038, 0x74),
    ov!(0x0039, 0x7B),
    ov!(0x003E, 0x8D),
    ov!(0x003F, 0x0C),
    ov!(0x0040, 0xC0),
    ov!(0x00AE, 0x41),
    ov!(0x00AF, 0x40),
    ov!(0x00B0, 0x40),
    ov!(0x00C1, 0xC2),
    ov!(0x00C2, 0x04),
];
static DSOUND_CMCPXVOICECLIENT_SETEG_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETFILTER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0023, 0x6A),
    ov!(0x0024, 0x06),
    ov!(0x0047, 0x83),
    ov!(0x0048, 0xE0),
    ov!(0x0049, 0x03),
    ov!(0x0050, 0xC1),
    ov!(0x0051, 0xE8),
    ov!(0x0052, 0x12),
    ov!(0x0053, 0x83),
    ov!(0x0054, 0xE0),
    ov!(0x0055, 0x07),
];
static DSOUND_CMCPXVOICECLIENT_SETFILTER_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETLFO_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0038, 0x74),
    ov!(0x0039, 0x71),
    ov!(0x003A, 0x0F),
    ov!(0x003B, 0xB6),
    ov!(0x003C, 0x4E),
    ov!(0x003D, 0x64),
    ov!(0x0059, 0x0F),
    ov!(0x005A, 0xB7),
    ov!(0x005B, 0x08),
];
static DSOUND_CMCPXVOICECLIENT_SETLFO_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0024, 0x8D),
    ov!(0x0025, 0x45),
    ov!(0x0026, 0xFC),
    ov!(0x002F, 0x8D),
    ov!(0x0030, 0x45),
    ov!(0x0031, 0xD0),
    ov!(0x005E, 0xA3),
    ov!(0x005F, 0xF8),
    ov!(0x0060, 0x02),
    ov!(0x0061, 0x82),
    ov!(0x0062, 0xFE),
    ov!(0x00BF, 0xC9),
    ov!(0x00C0, 0xC3),
];
static DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETPITCH_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0022, 0x8D),
    ov!(0x0023, 0x7E),
    ov!(0x0024, 0x44),
    ov!(0x004E, 0x8D),
    ov!(0x004F, 0x0C),
    ov!(0x0050, 0x00),
    ov!(0x0064, 0x8D),
    ov!(0x0065, 0x46),
    ov!(0x0066, 0x0C),
];
static DSOUND_CMCPXVOICECLIENT_SETPITCH_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x002A, 0x8D),
    ov!(0x002B, 0x04),
    ov!(0x002C, 0x49),
    ov!(0x0045, 0x0F),
    ov!(0x0046, 0xB7),
    ov!(0x0047, 0x11),
    ov!(0x006C, 0x8B),
    ov!(0x006D, 0x54),
    ov!(0x006E, 0x85),
    ov!(0x006F, 0xEC),
    ov!(0x0084, 0x40),
    ov!(0x0085, 0x41),
    ov!(0x0086, 0x41),
];
static DSOUND_CMCPXVOICECLIENT_SETVOLUME_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0022, 0x83),
    ov!(0x0023, 0x7C),
    ov!(0x0024, 0x24),
    ov!(0x0025, 0x10),
    ov!(0x0026, 0x00),
    ov!(0x0035, 0xF3),
    ov!(0x0036, 0xAB),
    ov!(0x0042, 0xC2),
    ov!(0x0043, 0x0C),
    ov!(0x0044, 0x00),
];
static DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDCREATE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000B, 0x0F),
    ov!(0x000C, 0xB6),
    ov!(0x000D, 0xF0),
    ov!(0x0012, 0xE8),
    ov!(0x001B, 0x7C),
    ov!(0x001C, 0x13),
    ov!(0x0027, 0x1B),
    ov!(0x0028, 0xC9),
    ov!(0x0043, 0xC9),
];
static DSOUND_DIRECTSOUNDCREATE_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDCREATEBUFFER_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0008, 0x53),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0x57),
    ov!(0x003C, 0xFF),
    ov!(0x003D, 0x50),
    ov!(0x003E, 0x08),
    ov!(0x0054, 0xC2),
    ov!(0x0055, 0x08),
];
static DSOUND_DIRECTSOUNDCREATEBUFFER_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002F,
    target: "CDirectSound_CreateSoundBuffer",
}];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDCREATESTREAM_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x83),
    ov!(0x0005, 0x65),
    ov!(0x0006, 0xFC),
    ov!(0x0008, 0x53),
    ov!(0x0009, 0x56),
    ov!(0x000A, 0x57),
    ov!(0x003C, 0xFF),
    ov!(0x003D, 0x50),
    ov!(0x003E, 0x08),
    ov!(0x0054, 0xC2),
    ov!(0x0055, 0x08),
];
static DSOUND_DIRECTSOUNDCREATESTREAM_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002F,
    target: "CDirectSound_CreateSoundStream",
}];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDDOWORK_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0006, 0x0F),
    ov!(0x0007, 0xB6),
    ov!(0x0008, 0xF0),
    ov!(0x0009, 0xA1),
    ov!(0x000E, 0x85),
    ov!(0x0010, 0x74),
    ov!(0x0018, 0x85),
    ov!(0x001C, 0x0B),
    ov!(0x0022, 0xFF),
    ov!(0x0028, 0xC3),
];
static DSOUND_DIRECTSOUNDDOWORK_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0014,
    target: "CDirectSound_DoWork",
}];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0019, 0x25),
    ov!(0x001A, 0xFF),
    ov!(0x001B, 0xFF),
    ov!(0x001C, 0x00),
    ov!(0x001D, 0x00),
    ov!(0x0032, 0xC2),
    ov!(0x0033, 0x04),
];
static DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "DirectSoundEnterCriticalSection",
}];

// Source: DSound/4134.inl
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x0F),
    ov!(0x0007, 0xB6),
    ov!(0x000E, 0x85),
    ov!(0x000F, 0xF6),
    ov!(0x0012, 0x0B),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "CHRTFSource_SetFullHRTF5Channel",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xE4),
    ov!(0x0022, 0x0C),
    ov!(0x0023, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetDistanceFactor",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0008, 0x8B),
    ov!(0x0009, 0x44),
    ov!(0x000A, 0x24),
    ov!(0x000B, 0x08),
    ov!(0x0019, 0x23),
    ov!(0x001A, 0xC8),
    ov!(0x0022, 0x0C),
    ov!(0x0023, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetDopplerFactor",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0004, 0xD9),
    ov!(0x0005, 0x44),
    ov!(0x0006, 0x24),
    ov!(0x0007, 0x0C),
    ov!(0x0012, 0x83),
    ov!(0x0013, 0xC0),
    ov!(0x0014, 0xE4),
    ov!(0x0022, 0x0C),
    ov!(0x0023, 0x00),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetRolloffFactor",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDSTREAM_FLUSHEX_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x74),
    ov!(0x0004, 0xFF),
    ov!(0x0007, 0x10),
    ov!(0x000A, 0x24),
    ov!(0x000D, 0x74),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x10),
];
static DSOUND_IDIRECTSOUNDSTREAM_FLUSHEX_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_FlushEx",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0009, 0xD9),
    ov!(0x000C, 0xFF),
    ov!(0x000E, 0x24),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_SetDistanceFactor",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0009, 0xD9),
    ov!(0x000C, 0xFF),
    ov!(0x000E, 0x24),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_SetDopplerFactor",
}];

// Source: DSound/4134.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0009, 0xD9),
    ov!(0x000C, 0xFF),
    ov!(0x000E, 0x24),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x0C),
];
static DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_SetRolloffFactor",
}];

// Source: DSound/4134.inl
static DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4134_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000B, 0x33),
    ov!(0x000C, 0xFF),
    ov!(0x000D, 0x89),
    ov!(0x000E, 0x5D),
    ov!(0x000F, 0xFC),
    ov!(0x0010, 0x33),
    ov!(0x0011, 0xF6),
    ov!(0x0012, 0xE8),
    ov!(0x0041, 0x83),
    ov!(0x0053, 0xE8),
    ov!(0x006A, 0x8B),
];
static DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4134_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x000C, 0x00),
    ov!(0x001B, 0xFF),
    ov!(0x0025, 0x80),
    ov!(0x002C, 0x0C),
    ov!(0x002D, 0x8B),
    ov!(0x002E, 0x4E),
    ov!(0x002F, 0x1C),
    ov!(0x0030, 0x57),
    ov!(0x0047, 0xE8),
    ov!(0x005D, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0x3D),
    ov!(0x0027, 0x8B),
    ov!(0x0028, 0x4C),
    ov!(0x0029, 0x24),
    ov!(0x002A, 0x04),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x49),
    ov!(0x002D, 0x08),
    ov!(0x002E, 0x8B),
    ov!(0x0035, 0x81),
    ov!(0x0037, 0xFF),
    ov!(0x0038, 0xFF),
    ov!(0x0039, 0xFF),
    ov!(0x003A, 0x7F),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
    ov!(0x0050, 0x00),
];
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_4242_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "DirectSoundEnterCriticalSection",
}];

// Source: DSound/4242.inl
static DSOUND_CFULLHRTFSOURCE_GETHRTFFILTERPAIR_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x74),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0xD9),
    ov!(0x0006, 0x46),
    ov!(0x0007, 0x14),
    ov!(0x0008, 0x51),
    ov!(0x0009, 0xD8),
    ov!(0x000A, 0x1D),
    ov!(0x001A, 0x05),
    ov!(0x001F, 0xEB),
    ov!(0x0058, 0xD8),
    ov!(0x0059, 0x05),
];
static DSOUND_CFULLHRTFSOURCE_GETHRTFFILTERPAIR_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CHRTFSOURCE_SETALGORITHM_FULLHRTF_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x83),
    ov!(0x0001, 0x25),
    ov!(0x0006, 0x00),
    ov!(0x0007, 0xC7),
    ov!(0x0008, 0x05),
    ov!(0x0011, 0xC7),
    ov!(0x0012, 0x05),
    ov!(0x001B, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETALGORITHM_FULLHRTF_4242_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0017,
    target: "CFullHrtfSource_GetHrtfFilterPair",
}];

// Source: DSound/4242.inl
static DSOUND_CHRTFSOURCE_SETALGORITHM_LIGHTHRTF_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x0001, 0x05),
    ov!(0x0006, 0x01),
    ov!(0x0007, 0x00),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0xC7),
    ov!(0x000B, 0x05),
    ov!(0x0014, 0xC7),
    ov!(0x0015, 0x05),
    ov!(0x001E, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETALGORITHM_LIGHTHRTF_4242_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001A,
    target: "CLightHrtfSource_GetHrtfFilterPair",
}];

// Source: DSound/4242.inl
static DSOUND_CLIGHTHRTFSOURCE_GETHRTFFILTERPAIR_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0x74),
    ov!(0x0003, 0x24),
    ov!(0x0004, 0x08),
    ov!(0x0005, 0xD9),
    ov!(0x0006, 0x46),
    ov!(0x0007, 0x10),
    ov!(0x0008, 0x51),
    ov!(0x0009, 0xD9),
    ov!(0x000A, 0xE1),
    ov!(0x000B, 0xD8),
    ov!(0x000C, 0x05),
    ov!(0x0019, 0x99),
    ov!(0x001A, 0x6A),
    ov!(0x008B, 0x89),
];
static DSOUND_CLIGHTHRTFSOURCE_GETHRTFFILTERPAIR_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CMCPXBUFFER_STOP_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0x3C),
    ov!(0x000F, 0x03),
    ov!(0x0017, 0x74),
    ov!(0x0018, 0x2A),
    ov!(0x0019, 0xF6),
    ov!(0x001A, 0x44),
    ov!(0x001B, 0x24),
    ov!(0x001C, 0x10),
    ov!(0x001D, 0x02),
];
static DSOUND_CMCPXBUFFER_STOP_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0043, 0x83),
    ov!(0x0044, 0xFF),
    ov!(0x0045, 0x02),
    ov!(0x0069, 0x8B),
    ov!(0x006B, 0xB4),
    ov!(0x006C, 0x00),
    ov!(0x006E, 0x00),
    ov!(0x0076, 0x8B),
    ov!(0x0078, 0xB4),
    ov!(0x0079, 0x00),
    ov!(0x007B, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CMCPXVOICECLIENT_SETEG_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0016, 0x8B),
    ov!(0x0017, 0x45),
    ov!(0x0018, 0x08),
    ov!(0x0019, 0x8B),
    ov!(0x001A, 0x08),
    ov!(0x001B, 0x85),
    ov!(0x001C, 0xC9),
    ov!(0x001D, 0x75),
    ov!(0x001E, 0x75),
    ov!(0x001F, 0x8B),
    ov!(0x0050, 0x56),
    ov!(0x0051, 0x24),
];
static DSOUND_CMCPXVOICECLIENT_SETEG_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_CMCPXVOICECLIENT_SETLFO_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0014, 0x8B),
    ov!(0x0018, 0x08),
    ov!(0x0019, 0x85),
    ov!(0x001A, 0xC9),
    ov!(0x001B, 0x75),
    ov!(0x001C, 0x4A),
    ov!(0x001D, 0x39),
    ov!(0x001E, 0x48),
    ov!(0x001F, 0x04),
    ov!(0x0050, 0x31),
    ov!(0x0051, 0x4E),
];
static DSOUND_CMCPXVOICECLIENT_SETLFO_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0xE8),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0x3D),
    ov!(0x000C, 0x02),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x0B),
    ov!(0x001C, 0x68),
];
static DSOUND_DIRECTSOUNDUSEFULLHRTF_4242_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0013,
    target: "CHrtfSource_SetAlgorithm_FullHrtf",
}];

// Source: DSound/4242.inl
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0xE8),
    ov!(0x0006, 0x83),
    ov!(0x0007, 0x3D),
    ov!(0x000C, 0x02),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x0B),
    ov!(0x001C, 0x68),
];
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_4242_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0013,
    target: "CHrtfSource_SetAlgorithm_LightHrtf",
}];

// Source: DSound/4242.inl
static DSOUND_XFILECREATEMEDIAOBJECT_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x003D, 0x18),
    ov!(0x0040, 0x14),
    ov!(0x0043, 0x10),
    ov!(0x0046, 0x0C),
    ov!(0x0049, 0x08),
    ov!(0x0080, 0xC2),
    ov!(0x0081, 0x18),
];
static DSOUND_XFILECREATEMEDIAOBJECT_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_XFILECREATEMEDIAOBJECTEX_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0028, 0x1B),
    ov!(0x0036, 0x78),
    ov!(0x0037, 0x1C),
    ov!(0x0038, 0xFF),
    ov!(0x0039, 0x74),
    ov!(0x003A, 0x24),
    ov!(0x003B, 0x10),
    ov!(0x003C, 0x57),
    ov!(0x003D, 0xE8),
    ov!(0x0072, 0xC2),
    ov!(0x0073, 0x08),
];
static DSOUND_XFILECREATEMEDIAOBJECTEX_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0025, 0xF7),
    ov!(0x0045, 0xF6),
    ov!(0x0046, 0x7C),
    ov!(0x0047, 0x23),
    ov!(0x0048, 0x83),
    ov!(0x0049, 0x7C),
    ov!(0x004A, 0x24),
    ov!(0x004B, 0x14),
    ov!(0x004C, 0x00),
    ov!(0x0089, 0xC2),
    ov!(0x008A, 0x0C),
];
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4242.inl
static DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4242_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0016, 0x0B),
    ov!(0x0025, 0xF7),
    ov!(0x0049, 0xEB),
    ov!(0x004A, 0x0A),
    ov!(0x004B, 0xFF),
    ov!(0x004C, 0x74),
    ov!(0x004D, 0x24),
    ov!(0x004E, 0x14),
    ov!(0x004F, 0x57),
    ov!(0x0050, 0xE8),
    ov!(0x0085, 0xC2),
    ov!(0x0086, 0x0C),
];
static DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4242_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x74),
    ov!(0x0035, 0xFF),
    ov!(0x0040, 0x8B),
    ov!(0x004F, 0x8B),
];
static DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003A,
    target: "CDirectSoundVoice_SetRolloffCurve",
}];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDSTREAM_PAUSEEX_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0041, 0x85),
    ov!(0x004C, 0xFF),
    ov!(0x0053, 0xC7),
    ov!(0x0056, 0xC2),
    ov!(0x0057, 0x10),
];
static DSOUND_CDIRECTSOUNDSTREAM_PAUSEEX_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CMcpxStream_Pause_Ex",
}];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x00),
    ov!(0x0016, 0x68),
    ov!(0x0022, 0x05),
    ov!(0x002E, 0x74),
    ov!(0x003A, 0x24),
    ov!(0x0046, 0x74),
    ov!(0x0053, 0x8B),
];
static DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003E,
    target: "CDirectSoundVoice_SetRolloffCurve",
}];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x30),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x40),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x48),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x38),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x34),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x51),
    ov!(0x0013, 0x70),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x48),
    ov!(0x0016, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x89),
    ov!(0x0023, 0x74),
    ov!(0x0030, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x44),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x83),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUND_GETCAPS_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x000D, 0xB6),
    ov!(0x002D, 0x8B),
    ov!(0x002E, 0x44),
    ov!(0x002F, 0x24),
    ov!(0x0030, 0x08),
    ov!(0x0031, 0x89),
    ov!(0x0032, 0x10),
    ov!(0x0033, 0x8B),
    ov!(0x0034, 0x15),
    ov!(0x0052, 0x03),
    ov!(0x0069, 0xC2),
    ov!(0x006A, 0x08),
];
static DSOUND_CDIRECTSOUND_GETCAPS_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CDIRECTSOUND_GETOUTPUTLEVELS_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x10),
    ov!(0x0018, 0x5F),
    ov!(0x0019, 0x5E),
    ov!(0x001A, 0x74),
    ov!(0x001B, 0x25),
    ov!(0x001C, 0x89),
    ov!(0x001D, 0x02),
    ov!(0x001E, 0xA3),
    ov!(0x001F, 0xB0),
    ov!(0x0043, 0xC2),
    ov!(0x0044, 0x0C),
];
static DSOUND_CDIRECTSOUND_GETOUTPUTLEVELS_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CMCPXBUFFER_PLAY_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000E, 0x3B),
    ov!(0x000F, 0xDF),
    ov!(0x001C, 0x74),
    ov!(0x001D, 0x09),
    ov!(0x0030, 0xF6),
    ov!(0x0031, 0x46),
    ov!(0x0032, 0x12),
    ov!(0x0033, 0x02),
    ov!(0x0071, 0xFF),
    ov!(0x0072, 0x50),
    ov!(0x0073, 0x18),
];
static DSOUND_CMCPXBUFFER_PLAY_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CMCPXSTREAM_FLUSH_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0012, 0x33),
    ov!(0x003D, 0x83),
    ov!(0x003E, 0xFE),
    ov!(0x003F, 0x06),
    ov!(0x0056, 0xE8),
    ov!(0x0067, 0xE8),
    ov!(0x00D1, 0xC9),
    ov!(0x00D2, 0xC3),
];
static DSOUND_CMCPXSTREAM_FLUSH_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_CMCPXSTREAM_PAUSE_EX_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000D, 0x8B),
    ov!(0x001C, 0x6A),
    ov!(0x001D, 0x05),
    ov!(0x0023, 0x75),
    ov!(0x002A, 0xE8),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x0C),
];
static DSOUND_CMCPXSTREAM_PAUSE_EX_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002B,
    target: "CMcpxStream_Pause",
}];

// Source: DSound/4361.inl
static DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0024, 0x83),
    ov!(0x0025, 0x7C),
    ov!(0x0026, 0x24),
    ov!(0x0027, 0x10),
    ov!(0x0028, 0x00),
    ov!(0x0037, 0xF3),
    ov!(0x0038, 0xAB),
    ov!(0x0044, 0xC2),
    ov!(0x0045, 0x0C),
    ov!(0x0046, 0x00),
];
static DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_DIRECTSOUNDGETSAMPLETIME_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x33),
    ov!(0x0001, 0xC0),
    ov!(0x000A, 0xA1),
    ov!(0x000B, 0x0C),
    ov!(0x000C, 0x20),
    ov!(0x000D, 0x80),
    ov!(0x000E, 0xFE),
    ov!(0x000F, 0xC3),
];
static DSOUND_DIRECTSOUNDGETSAMPLETIME_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0003, 0x10),
    ov!(0x0008, 0xFF),
    ov!(0x000D, 0xC8),
    ov!(0x0012, 0x83),
    ov!(0x0017, 0x1B),
    ov!(0x001C, 0xE8),
    ov!(0x0021, 0xC2),
];
static DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_SetRolloffCurve",
}];

// Source: DSound/4361.inl
static DSOUND_IDIRECTSOUNDSTREAM_PAUSEEX_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x24),
    ov!(0x000E, 0x24),
    ov!(0x0010, 0xE8),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x10),
];
static DSOUND_IDIRECTSOUNDSTREAM_PAUSEEX_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSoundStream_PauseEx",
}];

// Source: DSound/4361.inl
static DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_SetRolloffCurve",
}];

// Source: DSound/4361.inl
static DSOUND_IDIRECTSOUND_GETOUTPUTLEVELS_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0xFF),
    ov!(0x000E, 0x83),
    ov!(0x0012, 0xD9),
    ov!(0x0016, 0xC8),
    ov!(0x001D, 0xC2),
    ov!(0x001E, 0x0C),
];
static DSOUND_IDIRECTSOUND_GETOUTPUTLEVELS_4361_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0019,
    target: "CDirectSound_GetOutputLevels",
}];

// Source: DSound/4361.inl
static DSOUND_XFILECREATEMEDIAOBJECT_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x56),
    ov!(0x0034, 0x18),
    ov!(0x0037, 0x14),
    ov!(0x003A, 0x10),
    ov!(0x003D, 0x0C),
    ov!(0x0040, 0x08),
    ov!(0x0067, 0xC2),
    ov!(0x0068, 0x18),
];
static DSOUND_XFILECREATEMEDIAOBJECT_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_XFILECREATEMEDIAOBJECTEX_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0x1B),
    ov!(0x002D, 0x78),
    ov!(0x002E, 0x1C),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0x74),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x0C),
    ov!(0x0033, 0x57),
    ov!(0x0034, 0xE8),
    ov!(0x0059, 0xC2),
    ov!(0x005A, 0x08),
];
static DSOUND_XFILECREATEMEDIAOBJECTEX_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001C, 0xF7),
    ov!(0x003C, 0xF6),
    ov!(0x003D, 0x7C),
    ov!(0x003E, 0x23),
    ov!(0x003F, 0x83),
    ov!(0x0040, 0x7C),
    ov!(0x0041, 0x24),
    ov!(0x0042, 0x10),
    ov!(0x0043, 0x00),
    ov!(0x0070, 0xC2),
    ov!(0x0071, 0x0C),
];
static DSOUND_XWAVEFILECREATEMEDIAOBJECT_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4361.inl
static DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4361_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x0B),
    ov!(0x001C, 0xF7),
    ov!(0x0040, 0xEB),
    ov!(0x0041, 0x0A),
    ov!(0x0042, 0xFF),
    ov!(0x0043, 0x74),
    ov!(0x0044, 0x24),
    ov!(0x0045, 0x10),
    ov!(0x0046, 0x57),
    ov!(0x0047, 0xE8),
    ov!(0x006C, 0xC2),
    ov!(0x006D, 0x0C),
];
static DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4361_XREFS: &[OovpaXref] = &[];

// Source: DSound/4432.inl
static DSOUND_XFILECREATEMEDIAOBJECTASYNC_4432_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x001F, 0x1B),
    ov!(0x002B, 0x07),
    ov!(0x002C, 0x80),
    ov!(0x002D, 0x78),
    ov!(0x002E, 0x20),
    ov!(0x002F, 0xFF),
    ov!(0x0030, 0x74),
    ov!(0x0031, 0x24),
    ov!(0x0032, 0x10),
    ov!(0x005D, 0xC2),
    ov!(0x005E, 0x0C),
];
static DSOUND_XFILECREATEMEDIAOBJECTASYNC_4432_XREFS: &[OovpaXref] = &[];

// Source: DSound/4531.inl
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4531_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0012, 0x66),
    ov!(0x0013, 0xBA),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x08),
    ov!(0x0023, 0xE8),
    ov!(0x002D, 0x5E),
    ov!(0x002E, 0xC3),
];
static DSOUND_CMCPXSTREAM_DISCONTINUITY_4531_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0024,
    target: "CMcpxStream_Stop_Ex",
}];

// Source: DSound/4627.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x40),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x48),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x78),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0035, 0x68),
    ov!(0x0036, 0x8B),
    ov!(0x0037, 0x41),
    ov!(0x0038, 0x08),
    ov!(0x0039, 0x80),
    ov!(0x003A, 0x88),
    ov!(0x003B, 0xA4),
    ov!(0x003F, 0xE0),
    ov!(0x004F, 0x74),
    ov!(0x0050, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0035, 0x70),
    ov!(0x0039, 0x80),
    ov!(0x003A, 0x88),
    ov!(0x003B, 0xA4),
    ov!(0x003F, 0x80),
    ov!(0x004F, 0x74),
    ov!(0x0050, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CDIRECTSOUND_SETVELOCITY_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0024, 0xB8),
    ov!(0x0025, 0x05),
    ov!(0x0026, 0x40),
    ov!(0x0027, 0x00),
    ov!(0x0028, 0x80),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x7A),
    ov!(0x0039, 0x44),
    ov!(0x004C, 0x80),
    ov!(0x004D, 0x88),
    ov!(0x004E, 0xA4),
    ov!(0x004F, 0x00),
    ov!(0x0052, 0x80),
    ov!(0x0073, 0xC2),
    ov!(0x0074, 0x14),
];
static DSOUND_CDIRECTSOUND_SETVELOCITY_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0042, 0x83),
    ov!(0x0043, 0xFF),
    ov!(0x0044, 0x02),
    ov!(0x0068, 0x8B),
    ov!(0x006A, 0xB4),
    ov!(0x006B, 0x00),
    ov!(0x006D, 0x00),
    ov!(0x0077, 0x8B),
    ov!(0x0079, 0xB4),
    ov!(0x007A, 0x00),
    ov!(0x007C, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4627.inl
static DSOUND_CMEMORYMANAGER_MEMALLOC_4627_ENTRIES: &[OovpaEntry] = &[
    ov!(0x001B, 0x83),
    ov!(0x001C, 0x7C),
    ov!(0x001D, 0x24),
    ov!(0x001E, 0x10),
    ov!(0x001F, 0x00),
    ov!(0x0033, 0x83),
    ov!(0x0034, 0xE1),
    ov!(0x0035, 0x03),
    ov!(0x0065, 0xC2),
    ov!(0x0066, 0x0C),
];
static DSOUND_CMEMORYMANAGER_MEMALLOC_4627_XREFS: &[OovpaXref] = &[];

// Source: DSound/4721.inl
static DSOUND_CDIRECTSOUNDBUFFER_PAUSE_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0039, 0x85),
    ov!(0x0044, 0xFF),
    ov!(0x004B, 0xC7),
    ov!(0x004E, 0xC2),
    ov!(0x004F, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_PAUSE_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CMcpxBuffer_Pause",
}];

// Source: DSound/4721.inl
static DSOUND_CDIRECTSOUNDBUFFER_PAUSEEX_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x0014, 0x74),
    ov!(0x0021, 0xB8),
    ov!(0x002A, 0x24),
    ov!(0x0041, 0x85),
    ov!(0x004C, 0xFF),
    ov!(0x0053, 0xC7),
    ov!(0x0056, 0xC2),
    ov!(0x0057, 0x10),
];
static DSOUND_CDIRECTSOUNDBUFFER_PAUSEEX_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x003D,
    target: "CMcpxBuffer_Pause_Ex",
}];

// Source: DSound/4721.inl
static DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0008, 0x6A),
    ov!(0x0009, 0x01),
    ov!(0x0041, 0xC2),
    ov!(0x0042, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4721_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0030,
        target: "CMcpxVoiceClient_SetMixBins",
    },
    OovpaXref {
        offset: 0x003C,
        target: "CMcpxVoiceClient_SetPitch",
    },
];

// Source: DSound/4721.inl
static DSOUND_CMCPXBUFFER_PAUSE_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0014, 0x8A),
    ov!(0x0020, 0x01),
    ov!(0x0021, 0x75),
    ov!(0x0022, 0x04),
    ov!(0x0023, 0x6A),
    ov!(0x0024, 0x04),
    ov!(0x0025, 0xEB),
    ov!(0x0026, 0x08),
    ov!(0x0027, 0x83),
    ov!(0x0042, 0xC2),
    ov!(0x0043, 0x04),
];
static DSOUND_CMCPXBUFFER_PAUSE_4721_XREFS: &[OovpaXref] = &[];

// Source: DSound/4721.inl
static DSOUND_CMCPXBUFFER_PAUSE_EX_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000D, 0x8B),
    ov!(0x001C, 0x6A),
    ov!(0x001D, 0x05),
    ov!(0x0023, 0x75),
    ov!(0x002A, 0xE8),
    ov!(0x0036, 0xC2),
    ov!(0x0037, 0x0C),
];
static DSOUND_CMCPXBUFFER_PAUSE_EX_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002B,
    target: "CMcpxBuffer_Pause",
}];

// Source: DSound/4721.inl
static DSOUND_CMCPXBUFFER_PLAY_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x002E, 0xF6),
    ov!(0x002F, 0x40),
    ov!(0x0030, 0x09),
    ov!(0x0031, 0x20),
    ov!(0x0044, 0xF6),
    ov!(0x0045, 0x46),
    ov!(0x0046, 0x12),
    ov!(0x0047, 0x02),
    ov!(0x0052, 0x33),
    ov!(0x0053, 0xC0),
];
static DSOUND_CMCPXBUFFER_PLAY_4721_XREFS: &[OovpaXref] = &[];

// Source: DSound/4721.inl
static DSOUND_CMCPXSTREAM_GETSTATUS_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x0F),
    ov!(0x0009, 0x00),
    ov!(0x000A, 0x33),
    ov!(0x000B, 0xC9),
    ov!(0x000C, 0x39),
    ov!(0x000D, 0x00),
    ov!(0x000E, 0x8B),
    ov!(0x000F, 0x44),
    ov!(0x0010, 0x24),
    ov!(0x0011, 0x04),
    ov!(0x0012, 0x0F),
    ov!(0x0013, 0x95),
    ov!(0x001C, 0x80),
    ov!(0x0041, 0x00),
];
static DSOUND_CMCPXSTREAM_GETSTATUS_4721_XREFS: &[OovpaXref] = &[];

// Source: DSound/4721.inl
static DSOUND_IDIRECTSOUNDBUFFER_PAUSE_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_PAUSE_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_Pause",
}];

// Source: DSound/4721.inl
static DSOUND_IDIRECTSOUNDBUFFER_PAUSEEX_4721_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0012, 0x83),
    ov!(0x0016, 0xD9),
    ov!(0x001A, 0xC8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUNDBUFFER_PAUSEEX_4721_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSoundBuffer_PauseEx",
}];

// Source: DSound/4831.inl
static DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0001, 0x44),
    ov!(0x0002, 0x24),
    ov!(0x0003, 0x04),
    ov!(0x0004, 0x8B),
    ov!(0x0005, 0x48),
    ov!(0x0006, 0x0C),
    ov!(0x000C, 0xC2),
    ov!(0x000D, 0x04),
];
static DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_4831_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "CMcpxAPU_SynchPlayback",
}];

// Source: DSound/4831.inl
static DSOUND_CMCPXAPU_SYNCHPLAYBACK_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x001A, 0x1A),
    ov!(0x0025, 0x53),
    ov!(0x0037, 0xF2),
    ov!(0x007F, 0x47),
    ov!(0x0080, 0x04),
    ov!(0x0084, 0x43),
    ov!(0x009E, 0x64),
    ov!(0x00FF, 0x00),
];
static DSOUND_CMCPXAPU_SYNCHPLAYBACK_4831_XREFS: &[OovpaXref] = &[];

// Source: DSound/4831.inl
static DSOUND_CMCPXBUFFER_PAUSE_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0027, 0x83),
    ov!(0x0028, 0xE0),
    ov!(0x0029, 0xBB),
    ov!(0x002D, 0xE8),
    ov!(0x0040, 0x83),
    ov!(0x0041, 0xE0),
    ov!(0x0042, 0xBF),
    ov!(0x0070, 0xC2),
    ov!(0x0071, 0x04),
];
static DSOUND_CMCPXBUFFER_PAUSE_4831_XREFS: &[OovpaXref] = &[];

// Source: DSound/4831.inl
static DSOUND_CMCPXBUFFER_PLAY_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x53),
    ov!(0x0032, 0xF6),
    ov!(0x0033, 0x46),
    ov!(0x0034, 0x12),
    ov!(0x0035, 0x42),
    ov!(0x004D, 0xF6),
    ov!(0x004E, 0x40),
    ov!(0x004F, 0x09),
    ov!(0x0050, 0x20),
    ov!(0x0071, 0x33),
    ov!(0x0072, 0xC0),
];
static DSOUND_CMCPXBUFFER_PLAY_4831_XREFS: &[OovpaXref] = &[];

// Source: DSound/4831.inl
static DSOUND_CMCPXSTREAM_PAUSE_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0017, 0x66),
    ov!(0x0018, 0x25),
    ov!(0x0019, 0xDF),
    ov!(0x0032, 0xEB),
    ov!(0x0033, 0x56),
    ov!(0x0039, 0x0F),
    ov!(0x003A, 0xB7),
    ov!(0x003B, 0x46),
    ov!(0x003C, 0x12),
    ov!(0x008B, 0xC2),
    ov!(0x008C, 0x04),
];
static DSOUND_CMCPXSTREAM_PAUSE_4831_XREFS: &[OovpaXref] = &[];

// Source: DSound/4831.inl
static DSOUND_IDIRECTSOUND_SYNCHPLAYBACK_4831_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x83),
    ov!(0x0007, 0xC0),
    ov!(0x0008, 0xF8),
    ov!(0x000B, 0x1B),
    ov!(0x000C, 0xC9),
    ov!(0x0015, 0xC2),
    ov!(0x0016, 0x04),
];
static DSOUND_IDIRECTSOUND_SYNCHPLAYBACK_4831_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0011,
    target: "CDirectSound_SynchPlayback",
}];

// Source: DSound/5028.inl
static DSOUND_CDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x002C, 0x10),
    ov!(0x0030, 0x10),
    ov!(0x0031, 0xE8),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_GetVoiceProperties",
}];

// Source: DSound/5028.inl
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0028, 0x8B),
    ov!(0x002B, 0x08),
    ov!(0x002C, 0x8B),
    ov!(0x002E, 0x24),
    ov!(0x0031, 0xE8),
    ov!(0x0048, 0xC2),
    ov!(0x0049, 0x04),
];
static DSOUND_CDIRECTSOUNDSTREAM_FLUSH_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CMcpxStream_Flush",
}];

// Source: DSound/5028.inl
static DSOUND_CDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x002B, 0x08),
    ov!(0x0030, 0x10),
    ov!(0x0035, 0xE8),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_GetVoiceProperties",
}];

// Source: DSound/5028.inl
static DSOUND_CDIRECTSOUNDVOICE_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0008, 0x8B),
    ov!(0x000B, 0xE8),
    ov!(0x0010, 0xC2),
    ov!(0x0011, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000C,
    target: "CMcpxVoiceClient_GetVoiceProperties",
}];

// Source: DSound/5028.inl
static DSOUND_CMCPXSTREAM_STOP_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x001F, 0xF6),
    ov!(0x0020, 0x45),
    ov!(0x0021, 0x08),
    ov!(0x0022, 0x06),
];
static DSOUND_CMCPXSTREAM_STOP_5028_XREFS: &[OovpaXref] = &[];

// Source: DSound/5028.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x003F, 0x83),
    ov!(0x0040, 0xFF),
    ov!(0x0041, 0x02),
    ov!(0x0061, 0x8B),
    ov!(0x0063, 0xB4),
    ov!(0x0064, 0x00),
    ov!(0x0066, 0x00),
    ov!(0x0070, 0x8B),
    ov!(0x0072, 0xB4),
    ov!(0x0073, 0x00),
    ov!(0x0075, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5028_XREFS: &[OovpaXref] = &[];

// Source: DSound/5028.inl
static DSOUND_CMCPXVOICECLIENT_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x83),
    ov!(0x0006, 0x83),
    ov!(0x000A, 0xF6),
    ov!(0x00B5, 0xC1),
    ov!(0x00B6, 0xEE),
    ov!(0x00B7, 0x06),
    ov!(0x0102, 0xC7),
    ov!(0x0103, 0x07),
    ov!(0x0104, 0xF0),
    ov!(0x0105, 0xD8),
    ov!(0x0106, 0xFF),
    ov!(0x0107, 0xFF),
];
static DSOUND_CMCPXVOICECLIENT_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[];

// Source: DSound/5028.inl
static DSOUND_IDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x83),
    ov!(0x000E, 0xD9),
    ov!(0x0012, 0xC8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_GetVoiceProperties",
}];

// Source: DSound/5028.inl
static DSOUND_IDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_GetVoiceProperties",
}];

// Source: DSound/5028.inl
static DSOUND_XAUDIOSETEFFECTDATA_5028_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0x55), ov!(0x0001, 0x8B), ov!(0x0003, 0x81)];
static DSOUND_XAUDIOSETEFFECTDATA_5028_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x007A,
        target: "CDirectSound_GetEffectData",
    },
    OovpaXref {
        offset: 0x00D1,
        target: "CDirectSound_SetEffectData",
    },
];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8D),
    ov!(0x0005, 0x81),
    ov!(0x003A, 0xF6),
    ov!(0x003B, 0xC1),
    ov!(0x003C, 0x40),
    ov!(0x00A3, 0xF7),
    ov!(0x00A4, 0x45),
    ov!(0x00A5, 0x6C),
    ov!(0x00A6, 0x01),
    ov!(0x00A7, 0x00),
    ov!(0x00A8, 0x41),
    ov!(0x00A9, 0x00),
    ov!(0x00D4, 0x6A),
    ov!(0x00D5, 0x03),
];
static DSOUND_CDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x8B),
    ov!(0x000F, 0xB8),
    ov!(0x0010, 0xF0),
    ov!(0x0011, 0xD8),
    ov!(0x0012, 0xFF),
    ov!(0x0013, 0xFF),
    ov!(0x0092, 0x04),
    ov!(0x009B, 0x02),
    ov!(0x00A0, 0x05),
    ov!(0x00A5, 0x07),
    ov!(0x00AA, 0x09),
    ov!(0x00AF, 0x0A),
];
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x83),
    ov!(0x0052, 0x83),
    ov!(0x0053, 0x0E),
    ov!(0x0054, 0x04),
    ov!(0x0065, 0x50),
    ov!(0x0066, 0x51),
    ov!(0x0067, 0x51),
    ov!(0x0087, 0x83),
    ov!(0x0088, 0x0E),
    ov!(0x0089, 0x08),
    ov!(0x008D, 0x89),
    ov!(0x008E, 0x4E),
    ov!(0x008F, 0x18),
];
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0003, 0x83),
    ov!(0x002D, 0xA9),
    ov!(0x002E, 0x10),
    ov!(0x002F, 0x00),
    ov!(0x0030, 0x20),
    ov!(0x0031, 0x15),
    ov!(0x008C, 0x66),
    ov!(0x008D, 0xF7),
    ov!(0x008E, 0x45),
    ov!(0x008F, 0x0A),
    ov!(0x0090, 0x14),
    ov!(0x0091, 0x40),
];
static DSOUND_CDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0001, 0x44),
    ov!(0x0004, 0x8B),
    ov!(0x0007, 0xE8),
    ov!(0x000C, 0x33),
    ov!(0x000D, 0xC0),
    ov!(0x0010, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0008,
    target: "CMcpxVoiceClient_Commit3dSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x51),
    ov!(0x0013, 0x20),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x48),
    ov!(0x0016, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x89),
    ov!(0x0023, 0x24),
    ov!(0x0030, 0x04),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x28),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x2C),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x30),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x34),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x02),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x44),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x02),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x4C),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x03),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0012, 0x8B),
    ov!(0x0013, 0x31),
    ov!(0x0014, 0x89),
    ov!(0x0015, 0xB2),
    ov!(0x0016, 0x80),
    ov!(0x00B3, 0x80),
    ov!(0x00B4, 0x49),
    ov!(0x00B6, 0x7F),
    ov!(0x00C7, 0xC2),
    ov!(0x00C8, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x00C1,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x3C),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x02),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x38),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x02),
    ov!(0x0031, 0x0C),
    ov!(0x0032, 0x00),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0009, 0xB4),
    ov!(0x0010, 0x08),
    ov!(0x0013, 0x40),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x41),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x02),
    ov!(0x0020, 0x40),
    ov!(0x002E, 0x33),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETMODE_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x08),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x0C),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x10),
];
static DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x51),
    ov!(0x0013, 0x50),
    ov!(0x0014, 0x8B),
    ov!(0x0015, 0x48),
    ov!(0x0016, 0x10),
    ov!(0x0017, 0x8B),
    ov!(0x0018, 0x89),
    ov!(0x0023, 0x54),
    ov!(0x0030, 0x01),
];
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000D, 0x8B),
    ov!(0x000E, 0x54),
    ov!(0x000F, 0x24),
    ov!(0x0010, 0x08),
    ov!(0x0011, 0x89),
    ov!(0x0012, 0x50),
    ov!(0x0013, 0x48),
    ov!(0x0014, 0x8B),
    ov!(0x001D, 0x80),
    ov!(0x001F, 0x03),
    ov!(0x0030, 0xC2),
    ov!(0x0031, 0x0C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x002A,
    target: "CDirectSoundVoice_CommitDeferredSettings",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000F, 0x8B),
    ov!(0x0010, 0x55),
    ov!(0x0011, 0x0C),
    ov!(0x0012, 0x89),
    ov!(0x0013, 0x51),
    ov!(0x0014, 0x14),
    ov!(0x0021, 0x89),
    ov!(0x0022, 0x51),
    ov!(0x0023, 0x18),
    ov!(0x0030, 0x89),
    ov!(0x0031, 0x51),
    ov!(0x0032, 0x1C),
];
static DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0028, 0xB8),
    ov!(0x0029, 0x05),
    ov!(0x002A, 0x40),
    ov!(0x002C, 0x80),
    ov!(0x0097, 0xC9),
];
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5344_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0047,
        target: "CDirectSound3DCalculator_Calculate3D",
    },
    OovpaXref {
        offset: 0x005F,
        target: "CDirectSoundVoice_CommitDeferredSettings",
    },
];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x51),
    ov!(0x0005, 0x83),
    ov!(0x0006, 0x65),
    ov!(0x0007, 0xFC),
    ov!(0x0008, 0x00),
    ov!(0x0009, 0xE8),
    ov!(0x002D, 0x05),
    ov!(0x003D, 0x08),
    ov!(0x004D, 0xC3),
];
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_MAPBUFFERDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x002B, 0x08),
    ov!(0x002E, 0x0C),
    ov!(0x0034, 0x83),
    ov!(0x0035, 0xC1),
    ov!(0x0036, 0x60),
    ov!(0x0047, 0x18),
    ov!(0x0066, 0xC2),
    ov!(0x0067, 0x10),
];
static DSOUND_CDIRECTSOUND_MAPBUFFERDATA_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x48),
    ov!(0x003C, 0x08),
    ov!(0x003F, 0x24),
    ov!(0x004C, 0x74),
    ov!(0x004D, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x48),
    ov!(0x003C, 0x20),
    ov!(0x003F, 0x24),
    ov!(0x004C, 0x74),
    ov!(0x004D, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SETPOSITION_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x4D),
    ov!(0x002D, 0x08),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x7A),
    ov!(0x0039, 0x3C),
    ov!(0x003F, 0x89),
    ov!(0x0040, 0x7A),
    ov!(0x0041, 0x40),
    ov!(0x0047, 0x89),
    ov!(0x0048, 0x7A),
    ov!(0x0049, 0x44),
];
static DSOUND_CDIRECTSOUND_SETPOSITION_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0021, 0xB8),
    ov!(0x0022, 0x05),
    ov!(0x0023, 0x40),
    ov!(0x0024, 0x00),
    ov!(0x0025, 0x80),
    ov!(0x0033, 0x89),
    ov!(0x0034, 0x50),
    ov!(0x0039, 0x83),
    ov!(0x003A, 0x48),
    ov!(0x003C, 0x10),
    ov!(0x003F, 0x24),
    ov!(0x004C, 0x74),
    ov!(0x004D, 0x0B),
];
static DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SETVELOCITY_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x002B, 0x8B),
    ov!(0x002C, 0x4D),
    ov!(0x002D, 0x08),
    ov!(0x002E, 0x8D),
    ov!(0x002F, 0x41),
    ov!(0x0030, 0x08),
    ov!(0x0037, 0x89),
    ov!(0x0038, 0x7A),
    ov!(0x004C, 0x83),
    ov!(0x004D, 0x48),
    ov!(0x004F, 0x02),
    ov!(0x0070, 0xC2),
    ov!(0x0071, 0x14),
];
static DSOUND_CDIRECTSOUND_SETVELOCITY_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0007, 0x3D),
    ov!(0x000C, 0x00),
    ov!(0x0016, 0x68),
    ov!(0x002D, 0x48),
    ov!(0x002E, 0x0C),
    ov!(0x002F, 0x57),
    ov!(0x0036, 0xF6),
    ov!(0x003B, 0x68),
];
static DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CMcpxAPU_SynchPlayback",
}];

// Source: DSound/5344.inl
static DSOUND_CDIRECTSOUND_UNMAPBUFFERDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x000C, 0x00),
    ov!(0x002B, 0x08),
    ov!(0x002E, 0x0C),
    ov!(0x0032, 0x0C),
    ov!(0x0033, 0x83),
    ov!(0x0034, 0xC1),
    ov!(0x0035, 0x60),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUND_UNMAPBUFFERDATA_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0013, 0xD9),
    ov!(0x0029, 0xDF),
    ov!(0x003D, 0x2D),
    ov!(0x0054, 0xC1),
    ov!(0x0067, 0x4D),
    ov!(0x007E, 0xD9),
    ov!(0x0091, 0x10),
];
static DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CHRTFSOURCE_SETFULLHRTF4CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x000A, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC7),
    ov!(0x005A, 0xC7),
    ov!(0x0064, 0xC7),
    ov!(0x006A, 0x01),
    ov!(0x006B, 0x00),
    ov!(0x006C, 0x00),
    ov!(0x006D, 0x00),
    ov!(0x006E, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETFULLHRTF4CHANNEL_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x000A, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC7),
    ov!(0x005A, 0xC7),
    ov!(0x0064, 0xC7),
    ov!(0x006A, 0x03),
    ov!(0x006B, 0x00),
    ov!(0x006C, 0x00),
    ov!(0x006D, 0x00),
    ov!(0x006E, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004C,
    target: "CFullHRTFSource_GetCenterVolume",
}];

// Source: DSound/5344.inl
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF4CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x000A, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC7),
    ov!(0x005A, 0xC7),
    ov!(0x0064, 0xC7),
    ov!(0x006A, 0x02),
    ov!(0x006B, 0x00),
    ov!(0x006C, 0x00),
    ov!(0x006D, 0x00),
    ov!(0x006E, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF4CHANNEL_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xC7),
    ov!(0x000A, 0xC7),
    ov!(0x0014, 0xC7),
    ov!(0x001E, 0xC7),
    ov!(0x0028, 0xC7),
    ov!(0x0032, 0xC7),
    ov!(0x003C, 0xC7),
    ov!(0x0046, 0xC7),
    ov!(0x0050, 0xC7),
    ov!(0x005A, 0xC7),
    ov!(0x0064, 0xC7),
    ov!(0x006A, 0x04),
    ov!(0x006B, 0x00),
    ov!(0x006C, 0x00),
    ov!(0x006D, 0x00),
    ov!(0x006E, 0xC3),
];
static DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x004C,
    target: "CLightHRTFSource_GetCenterVolume",
}];

// Source: DSound/5344.inl
static DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0013, 0xD9),
    ov!(0x0014, 0xE0),
    ov!(0x003B, 0xD9),
    ov!(0x003C, 0xE8),
    ov!(0x0061, 0xF0),
    ov!(0x0062, 0xD8),
    ov!(0x0063, 0xFF),
    ov!(0x0064, 0xFF),
    ov!(0x0066, 0xC2),
    ov!(0x0067, 0x10),
];
static DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CMCPXSTREAM_FLUSH_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0003, 0x83),
    ov!(0x0004, 0xEC),
    ov!(0x0005, 0x10),
    ov!(0x000A, 0x64),
    ov!(0x000B, 0x0F),
    ov!(0x000C, 0xB6),
    ov!(0x000D, 0x05),
    ov!(0x000E, 0x24),
    ov!(0x000F, 0x00),
    ov!(0x0010, 0x00),
    ov!(0x0011, 0x00),
];
static DSOUND_CMCPXSTREAM_FLUSH_5344_XREFS: &[OovpaXref] = &[];

// Source: DSound/5344.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x005E, 0x8B),
    ov!(0x0060, 0xB4),
    ov!(0x0061, 0x00),
    ov!(0x0063, 0x00),
    ov!(0x006A, 0x8B),
    ov!(0x006C, 0xB4),
    ov!(0x006D, 0x00),
    ov!(0x006F, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5344_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x002C,
        target: "CDirectSound3DCalculator_Calculate3D",
    },
    OovpaXref {
        offset: 0x0057,
        target: "CDirectSound3DCalculator_GetVoiceData",
    },
];

// Source: DSound/5344.inl
static DSOUND_DIRECTSOUNDUSEFULLHRTF4CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x0F),
    ov!(0x0007, 0xB6),
    ov!(0x000E, 0x85),
    ov!(0x000F, 0xF6),
    ov!(0x0012, 0x0B),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSEFULLHRTF4CHANNEL_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "CHRTFSource_SetFullHRTF4Channel",
}];

// Source: DSound/5344.inl
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x0F),
    ov!(0x0007, 0xB6),
    ov!(0x000E, 0x85),
    ov!(0x000F, 0xF6),
    ov!(0x0012, 0x0B),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSELIGHTHRTF_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "CHRTFSource_SetLightHRTF5Channel",
}];

// Source: DSound/5344.inl
static DSOUND_DIRECTSOUNDUSELIGHTHRTF4CHANNEL_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x0F),
    ov!(0x0007, 0xB6),
    ov!(0x000E, 0x85),
    ov!(0x000F, 0xF6),
    ov!(0x0012, 0x0B),
    ov!(0x0018, 0xFF),
    ov!(0x001E, 0xC3),
];
static DSOUND_DIRECTSOUNDUSELIGHTHRTF4CHANNEL_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x000A,
    target: "CHRTFSource_SetLightHRTF4Channel",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSound3DCalculator_Calculate3D",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSound3DCalculator_GetMixBinVolumes",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000A, 0x24),
    ov!(0x0019, 0xE8),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x10),
];
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001A,
    target: "CDirectSound3DCalculator_GetPanData",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0x55), ov!(0x0003, 0x5D), ov!(0x0004, 0xE9)];
static DSOUND_IDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0005,
    target: "CDirectSound3DCalculator_GetVoiceData",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND_MAPBUFFERDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xFF),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x0014, 0xF8),
    ov!(0x0021, 0xC2),
    ov!(0x0022, 0x10),
];
static DSOUND_IDIRECTSOUND_MAPBUFFERDATA_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x001D,
    target: "CDirectSound_MapBufferData",
}];

// Source: DSound/5344.inl
static DSOUND_IDIRECTSOUND_UNMAPBUFFERDATA_5344_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0002, 0x24),
    ov!(0x0006, 0x24),
    ov!(0x000C, 0xF8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUND_UNMAPBUFFERDATA_5344_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSound_UnmapBufferData",
}];

// Source: DSound/5344.inl
static DSOUND_XAUDIOSETEFFECTDATA_5344_ENTRIES: &[OovpaEntry] =
    &[ov!(0x0000, 0x55), ov!(0x0001, 0x8B), ov!(0x0003, 0x81)];
static DSOUND_XAUDIOSETEFFECTDATA_5344_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x007A,
        target: "CDirectSound_GetEffectData",
    },
    OovpaXref {
        offset: 0x00C2,
        target: "CDirectSound_SetEffectData",
    },
];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x16),
    ov!(0x0016, 0x68),
    ov!(0x0036, 0x85),
    ov!(0x003A, 0x74),
    ov!(0x003C, 0x68),
    ov!(0x004B, 0xC2),
    ov!(0x004C, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0032,
    target: "CDirectSoundVoice_Set3DVoiceData",
}];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0011, 0x16),
    ov!(0x0016, 0x68),
    ov!(0x0035, 0xE8),
    ov!(0x003E, 0x74),
    ov!(0x0040, 0x68),
    ov!(0x004F, 0xC2),
    ov!(0x0050, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0036,
    target: "CDirectSoundVoice_Set3DVoiceData",
}];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUNDVOICE_SET3DVOICEDATA_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x0004, 0x8B),
    ov!(0x0028, 0x01),
    ov!(0x003C, 0x02),
    ov!(0x0050, 0x04),
    ov!(0x0073, 0x08),
    ov!(0x0096, 0x10),
    ov!(0x00B9, 0x20),
    ov!(0x00CD, 0x40),
];
static DSOUND_CDIRECTSOUNDVOICE_SET3DVOICEDATA_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0027, 0xB8),
    ov!(0x0028, 0x05),
    ov!(0x0029, 0x40),
    ov!(0x002B, 0x80),
    ov!(0x0096, 0xC9),
];
static DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5455_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x0046,
        target: "CDirectSound3DCalculator_Calculate3D",
    },
    OovpaXref {
        offset: 0x005E,
        target: "CDirectSoundVoice_CommitDeferredSettings",
    },
];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0016, 0x0F),
    ov!(0x0017, 0xB6),
    ov!(0x0018, 0xC0),
    ov!(0x003E, 0xC1),
    ov!(0x0040, 0x1F),
    ov!(0x0053, 0xBF),
    ov!(0x0054, 0x00),
    ov!(0x0055, 0x00),
    ov!(0x0056, 0x00),
    ov!(0x0057, 0x80),
    ov!(0x00C4, 0x83),
    ov!(0x00C5, 0x7D),
    ov!(0x00C6, 0x0C),
    ov!(0x00C7, 0x00),
    ov!(0x0118, 0xC2),
    ov!(0x0119, 0x08),
];
static DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5455.inl
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0xE8),
    ov!(0x000B, 0x00),
    ov!(0x0014, 0x0B),
    ov!(0x0020, 0xB8),
    ov!(0x0030, 0x08),
    ov!(0x0031, 0x81),
    ov!(0x0033, 0xFF),
    ov!(0x0034, 0xFF),
    ov!(0x0035, 0xFF),
    ov!(0x0036, 0x7F),
    ov!(0x003C, 0x0B),
    ov!(0x0048, 0x33),
    ov!(0x004A, 0xC2),
    ov!(0x004B, 0x08),
];
static DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5455.inl
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x0001, 0x8B),
    ov!(0x0002, 0xEC),
    ov!(0x0011, 0x8D),
    ov!(0x0012, 0x4D),
    ov!(0x0013, 0xF0),
    ov!(0x0017, 0x89),
    ov!(0x0018, 0x45),
    ov!(0x0019, 0xF8),
];
static DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5455.inl
static DSOUND_CMCPXSTREAM_DISCONTINUITY_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x56),
    ov!(0x0001, 0x57),
    ov!(0x0012, 0x66),
    ov!(0x0013, 0xBA),
    ov!(0x0014, 0x00),
    ov!(0x0015, 0x08),
    ov!(0x0021, 0xE8),
    ov!(0x002B, 0x5E),
    ov!(0x002C, 0xC3),
];
static DSOUND_CMCPXSTREAM_DISCONTINUITY_5455_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0022,
    target: "CMcpxStream_Stop",
}];

// Source: DSound/5455.inl
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x005D, 0x8B),
    ov!(0x005F, 0xB4),
    ov!(0x0060, 0x00),
    ov!(0x0062, 0x00),
    ov!(0x0069, 0x8B),
    ov!(0x006B, 0xB4),
    ov!(0x006C, 0x00),
    ov!(0x006E, 0x00),
];
static DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5455_XREFS: &[OovpaXref] = &[
    OovpaXref {
        offset: 0x002C,
        target: "CDirectSound3DCalculator_Calculate3D",
    },
    OovpaXref {
        offset: 0x0056,
        target: "CDirectSound3DCalculator_GetVoiceData",
    },
];

// Source: DSound/5455.inl
static DSOUND_DSOUND_CREFCOUNT_RELEASE_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x8B),
    ov!(0x000B, 0x48),
    ov!(0x000C, 0x89),
    ov!(0x000D, 0x41),
    ov!(0x000E, 0x04),
    ov!(0x001B, 0x8B),
    ov!(0x001C, 0x41),
    ov!(0x001D, 0x04),
    ov!(0x001E, 0xC2),
    ov!(0x001F, 0x04),
];
static DSOUND_DSOUND_CREFCOUNT_RELEASE_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5455.inl
static DSOUND_IDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x24),
    ov!(0x0008, 0x8B),
    ov!(0x000C, 0xE4),
    ov!(0x000E, 0xD9),
    ov!(0x0011, 0x23),
    ov!(0x0014, 0xE8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_Set3DVoiceData",
}];

// Source: DSound/5455.inl
static DSOUND_IDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_Set3DVoiceData",
}];

// Source: DSound/5455.inl
static DSOUND_XAUDIOCALCULATEPITCH_5455_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0000, 0x55),
    ov!(0x000F, 0x3D),
    ov!(0x0010, 0x80),
    ov!(0x0011, 0xBB),
    ov!(0x0012, 0x00),
    ov!(0x0018, 0xEB),
    ov!(0x0019, 0x21),
    ov!(0x003B, 0x8D),
    ov!(0x003C, 0x4D),
    ov!(0x003D, 0x08),
    ov!(0x0047, 0xC2),
    ov!(0x0048, 0x04),
];
static DSOUND_XAUDIOCALCULATEPITCH_5455_XREFS: &[OovpaXref] = &[];

// Source: DSound/5558.inl
static DSOUND_CDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x0F),
    ov!(0x0010, 0x16),
    ov!(0x0015, 0x68),
    ov!(0x0036, 0xC9),
    ov!(0x0039, 0x74),
    ov!(0x003B, 0x68),
    ov!(0x0049, 0xC2),
    ov!(0x004A, 0x08),
];
static DSOUND_CDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0031,
    target: "CDirectSoundVoice_Use3DVoiceData",
}];

// Source: DSound/5558.inl
static DSOUND_CDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_ENTRIES: &[OovpaEntry] = &[
    ov!(0x000C, 0x0F),
    ov!(0x0010, 0x16),
    ov!(0x0015, 0x68),
    ov!(0x003A, 0xC9),
    ov!(0x003D, 0x74),
    ov!(0x003F, 0x68),
    ov!(0x004D, 0xC2),
    ov!(0x004E, 0x08),
];
static DSOUND_CDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0035,
    target: "CDirectSoundVoice_Use3DVoiceData",
}];

// Source: DSound/5558.inl
static DSOUND_CDIRECTSOUNDVOICE_USE3DVOICEDATA_5558_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0004, 0x00),
    ov!(0x0007, 0x24),
    ov!(0x000B, 0x10),
    ov!(0x000F, 0x48),
    ov!(0x0012, 0xEB),
    ov!(0x0016, 0x0B),
    ov!(0x0018, 0x33),
    ov!(0x001A, 0xC2),
    ov!(0x001B, 0x08),
];
static DSOUND_CDIRECTSOUNDVOICE_USE3DVOICEDATA_5558_XREFS: &[OovpaXref] = &[];

// Source: DSound/5558.inl
static DSOUND_IDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_ENTRIES: &[OovpaEntry] = &[
    ov!(0x0006, 0x24),
    ov!(0x0008, 0x8B),
    ov!(0x000C, 0xE4),
    ov!(0x000E, 0xD9),
    ov!(0x0011, 0x23),
    ov!(0x0014, 0xE8),
    ov!(0x0019, 0xC2),
    ov!(0x001A, 0x08),
];
static DSOUND_IDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0015,
    target: "CDirectSoundBuffer_Use3DVoiceData",
}];

// Source: DSound/5558.inl
static DSOUND_IDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_ENTRIES: &[OovpaEntry] = &[ov!(0x0000, 0xE9)];
static DSOUND_IDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_XREFS: &[OovpaXref] = &[OovpaXref {
    offset: 0x0001,
    target: "CDirectSoundStream_Use3DVoiceData",
}];

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
    OovpaPattern {
        name: "CDirectSoundBufferSettings_SetBufferData",
        detect_size: 0x007A,
        entries: DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetStatus",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Lock",
        detect_size: 0x00A4,
        entries: DSOUND_CDIRECTSOUNDBUFFER_LOCK_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Play",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_PlayEx",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetAllParameters",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeAngles",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOrientation",
        detect_size: 0x0065,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetEG",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFilter",
        detect_size: 0x0047,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFormat",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFrequency",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetHeadroom",
        detect_size: 0x0047,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLFO",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLoopRegion",
        detect_size: 0x0067,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMaxDistance",
        detect_size: 0x0055,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMinDistance",
        detect_size: 0x0055,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMixBinVolumes",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMixBins",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMode",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        detect_size: 0x0024,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetOutputBuffer",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPitch",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPlayRegion",
        detect_size: 0x007E,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPosition",
        detect_size: 0x0065,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVelocity",
        detect_size: 0x0065,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVolume",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Stop",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOP_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_StopEx",
        detect_size: 0x0074,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_AddRef",
        detect_size: 0x0043,
        entries: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Discontinuity",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Flush",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetInfo",
        detect_size: 0x0065,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetStatus",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Pause",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Process",
        detect_size: 0x0062,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PROCESS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Release",
        detect_size: 0x004D,
        entries: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetAllParameters",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeAngles",
        detect_size: 0x0041,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOrientation",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetEG",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFilter",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFormat",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFrequency",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetHeadroom",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetI3DL2Source",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetLFO",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMaxDistance",
        detect_size: 0x0058,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMinDistance",
        detect_size: 0x0058,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBinVolumes",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBins",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMode",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetOutputBuffer",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPitch",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPosition",
        detect_size: 0x0069,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVelocity",
        detect_size: 0x0069,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVolume",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoiceSettings_SetMixBinVolumes",
        detect_size: 0x0024,
        entries: DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        detect_size: 0x0031,
        entries: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetAllParameters",
        detect_size: 0x0035,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeAngles",
        detect_size: 0x0039,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOrientation",
        detect_size: 0x0046,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetEG",
        detect_size: 0x0012,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFilter",
        detect_size: 0x0012,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFormat",
        detect_size: 0x0046,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFrequency",
        detect_size: 0x0023,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetHeadroom",
        detect_size: 0x0022,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetI3DL2Source",
        detect_size: 0x0059,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetLFO",
        detect_size: 0x0012,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMaxDistance",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMinDistance",
        detect_size: 0x002C,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMixBinVolumes",
        detect_size: 0x001C,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_8_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMixBins",
        detect_size: 0x001C,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMode",
        detect_size: 0x0022,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMODE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetOutputBuffer",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPitch",
        detect_size: 0x0018,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPosition",
        detect_size: 0x0046,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVelocity",
        detect_size: 0x0046,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVolume",
        detect_size: 0x001B,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_CommitDeferredSettings",
        detect_size: 0x0070,
        entries: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_CommitEffectData",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundBuffer",
        detect_size: 0x009E,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundStream",
        detect_size: 0x0093,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_DownloadEffectsImage",
        detect_size: 0x0062,
        entries: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_EnableHeadphones",
        detect_size: 0x0081,
        entries: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_GetCaps",
        detect_size: 0x006E,
        entries: DSOUND_CDIRECTSOUND_GETCAPS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_GetEffectData",
        detect_size: 0x005D,
        entries: DSOUND_CDIRECTSOUND_GETEFFECTDATA_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetAllParameters",
        detect_size: 0x0069,
        entries: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetDistanceFactor",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetDopplerFactor",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetEffectData",
        detect_size: 0x0060,
        entries: DSOUND_CDIRECTSOUND_SETEFFECTDATA_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetI3DL2Listener",
        detect_size: 0x0042,
        entries: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetMixBinHeadroom",
        detect_size: 0x005E,
        entries: DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetOrientation",
        detect_size: 0x0060,
        entries: DSOUND_CDIRECTSOUND_SETORIENTATION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetPosition",
        detect_size: 0x0074,
        entries: DSOUND_CDIRECTSOUND_SETPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetRolloffFactor",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSound_SetVelocity",
        detect_size: 0x0074,
        entries: DSOUND_CDIRECTSOUND_SETVELOCITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CFullHRTFSource_GetCenterVolume",
        detect_size: 0x00D3,
        entries: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CHRTFSource_SetFullHRTF5Channel",
        detect_size: 0x0051,
        entries: DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CHRTFSource_SetLightHRTF5Channel",
        detect_size: 0x0051,
        entries: DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CLightHRTFSource_GetCenterVolume",
        detect_size: 0x00BB,
        entries: DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxAPU_SetMixBinHeadroom",
        detect_size: 0x001F,
        entries: DSOUND_CMCPXAPU_SETMIXBINHEADROOM_4_4039_ENTRIES,
        argc: 1,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetCurrentPosition",
        detect_size: 0x00DE,
        entries: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetStatus",
        detect_size: 0x004A,
        entries: DSOUND_CMCPXBUFFER_GETSTATUS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0043,
        entries: DSOUND_CMCPXBUFFER_PLAY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play_Ex",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXBUFFER_PLAY_EX_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxBuffer_SetCurrentPosition",
        detect_size: 0x006E,
        entries: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxStream_Discontinuity",
        detect_size: 0x0020,
        entries: DSOUND_CMCPXSTREAM_DISCONTINUITY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxStream_Flush",
        detect_size: 0x009A,
        entries: DSOUND_CMCPXSTREAM_FLUSH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxStream_Pause",
        detect_size: 0x004A,
        entries: DSOUND_CMCPXSTREAM_PAUSE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x0079,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetEG",
        detect_size: 0x00D5,
        entries: DSOUND_CMCPXVOICECLIENT_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetFilter",
        detect_size: 0x004E,
        entries: DSOUND_CMCPXVOICECLIENT_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetLFO",
        detect_size: 0x00CF,
        entries: DSOUND_CMCPXVOICECLIENT_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetMixBins",
        detect_size: 0x00CA,
        entries: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetPitch",
        detect_size: 0x0076,
        entries: DSOUND_CMCPXVOICECLIENT_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetVolume",
        detect_size: 0x0077,
        entries: DSOUND_CMCPXVOICECLIENT_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DSound_CRefCount_AddRef",
        detect_size: 0x000C,
        entries: DSOUND_DSOUND_CREFCOUNT_ADDREF_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DSound_CRefCount_Release",
        detect_size: 0x0022,
        entries: DSOUND_DSOUND_CREFCOUNT_RELEASE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundCreate",
        detect_size: 0x0045,
        entries: DSOUND_DIRECTSOUNDCREATE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundCreateBuffer",
        detect_size: 0x0051,
        entries: DSOUND_DIRECTSOUNDCREATEBUFFER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundCreateStream",
        detect_size: 0x0051,
        entries: DSOUND_DIRECTSOUNDCREATESTREAM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundOverrideSpeakerConfig",
        detect_size: 0x0026,
        entries: DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundUseFullHRTF",
        detect_size: 0x001E,
        entries: DSOUND_DIRECTSOUNDUSEFULLHRTF_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "DirectSoundUseLightHRTF",
        detect_size: 0x001E,
        entries: DSOUND_DIRECTSOUNDUSELIGHTHRTF_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetAllParameters",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetConeAngles",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetConeOutsideVolume",
        detect_size: 0x0020,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetEG",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetFilter",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetFormat",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetFrequency",
        detect_size: 0x0019,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetHeadroom",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetI3DL2Source",
        detect_size: 0x0020,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetLFO",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMixBinVolumes",
        detect_size: 0x0019,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMixBins",
        detect_size: 0x0019,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetMode",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETMODE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetPitch",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetPlayRegion",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETPLAYREGION_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetVolume",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetEG",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETEG_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetFilter",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetFormat",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetFrequency",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetHeadroom",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetLFO",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETLFO_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMixBinVolumes",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_ENTRIES,
        argc: 2,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetMixBins",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetPitch",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetVolume",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "IsValidFormat",
        detect_size: 0x0032,
        entries: DSOUND_ISVALIDFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "XAudioCalculatePitch",
        detect_size: 0x004A,
        entries: DSOUND_XAUDIOCALCULATEPITCH_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "XAudioCreateAdpcmFormat",
        detect_size: 0x003E,
        entries: DSOUND_XAUDIOCREATEADPCMFORMAT_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "XAudioDownloadEffectsImage",
        detect_size: 0x0066,
        entries: DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4039_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4039,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetStatus",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Lock",
        detect_size: 0x0084,
        entries: DSOUND_CDIRECTSOUNDBUFFER_LOCK_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Play",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_PlayEx",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetAllParameters",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeAngles",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOrientation",
        detect_size: 0x0067,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetDistanceFactor",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetDopplerFactor",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetEG",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETEG_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFilter",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetFormat",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetHeadroom",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLFO",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetLoopRegion",
        detect_size: 0x0084,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMaxDistance",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMinDistance",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetMode",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPitch",
        detect_size: 0x0048,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPlayRegion",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetPosition",
        detect_size: 0x0067,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetRolloffFactor",
        detect_size: 0x0056,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVelocity",
        detect_size: 0x0067,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetVolume",
        detect_size: 0x004D,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Stop",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOP_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_StopEx",
        detect_size: 0x0053,
        entries: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_AddRef",
        detect_size: 0x0046,
        entries: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Discontinuity",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Flush",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_FlushEx",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_FLUSHEX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetInfo",
        detect_size: 0x0066,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetStatus",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Pause",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Release",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetAllParameters",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeAngles",
        detect_size: 0x0042,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOrientation",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetDistanceFactor",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetDopplerFactor",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetEG",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETEG_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFilter",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFormat",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetFrequency",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetHeadroom",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetI3DL2Source",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetLFO",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMaxDistance",
        detect_size: 0x0059,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMinDistance",
        detect_size: 0x0059,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBinVolumes",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMixBins",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetMode",
        detect_size: 0x0055,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetOutputBuffer",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPitch",
        detect_size: 0x0052,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetPosition",
        detect_size: 0x006A,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetRolloffFactor",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVelocity",
        detect_size: 0x006A,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetVolume",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoiceSettings_SetMixBinVolumes",
        detect_size: 0x0024,
        entries: DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        detect_size: 0x0038,
        entries: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetAllParameters",
        detect_size: 0x008D,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeAngles",
        detect_size: 0x0031,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOrientation",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDistanceFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDopplerFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetI3DL2Source",
        detect_size: 0x00AF,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMaxDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMinDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMode",
        detect_size: 0x0025,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMODE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPosition",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetRolloffFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVelocity",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVolume",
        detect_size: 0x0019,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_CommitEffectData",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundBuffer",
        detect_size: 0x009B,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_CreateSoundStream",
        detect_size: 0x0090,
        entries: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_DownloadEffectsImage",
        detect_size: 0x0063,
        entries: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_EnableHeadphones",
        detect_size: 0x0065,
        entries: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_GetCaps",
        detect_size: 0x006F,
        entries: DSOUND_CDIRECTSOUND_GETCAPS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_GetEffectData",
        detect_size: 0x005E,
        entries: DSOUND_CDIRECTSOUND_GETEFFECTDATA_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetAllParameters",
        detect_size: 0x0035,
        entries: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetDistanceFactor",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetDopplerFactor",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetI3DL2Listener",
        detect_size: 0x003F,
        entries: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetOrientation",
        detect_size: 0x0061,
        entries: DSOUND_CDIRECTSOUND_SETORIENTATION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetPosition",
        detect_size: 0x004A,
        entries: DSOUND_CDIRECTSOUND_SETPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetRolloffFactor",
        detect_size: 0x0040,
        entries: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSound_SetVelocity",
        detect_size: 0x0075,
        entries: DSOUND_CDIRECTSOUND_SETVELOCITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CFullHRTFSource_GetCenterVolume",
        detect_size: 0x00BE,
        entries: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        detect_size: 0x001D,
        entries: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetCurrentPosition",
        detect_size: 0x0062,
        entries: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_GetStatus",
        detect_size: 0x002F,
        entries: DSOUND_CMCPXBUFFER_GETSTATUS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0034,
        entries: DSOUND_CMCPXBUFFER_PLAY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_SetBufferData",
        detect_size: 0x001A,
        entries: DSOUND_CMCPXBUFFER_SETBUFFERDATA_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_SetCurrentPosition",
        detect_size: 0x008A,
        entries: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Stop",
        detect_size: 0x0021,
        entries: DSOUND_CMCPXBUFFER_STOP_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Stop_Ex",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXBUFFER_STOP_EX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxStream_Discontinuity",
        detect_size: 0x0025,
        entries: DSOUND_CMCPXSTREAM_DISCONTINUITY_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxStream_GetStatus",
        detect_size: 0x0051,
        entries: DSOUND_CMCPXSTREAM_GETSTATUS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxStream_Pause",
        detect_size: 0x0046,
        entries: DSOUND_CMCPXSTREAM_PAUSE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxStream_Stop",
        detect_size: 0x0034,
        entries: DSOUND_CMCPXSTREAM_STOP_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxStream_Stop_Ex",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXSTREAM_STOP_EX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x006C,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetEG",
        detect_size: 0x00C3,
        entries: DSOUND_CMCPXVOICECLIENT_SETEG_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetFilter",
        detect_size: 0x0056,
        entries: DSOUND_CMCPXVOICECLIENT_SETFILTER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetLFO",
        detect_size: 0x005C,
        entries: DSOUND_CMCPXVOICECLIENT_SETLFO_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetMixBins",
        detect_size: 0x00C1,
        entries: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetPitch",
        detect_size: 0x0067,
        entries: DSOUND_CMCPXVOICECLIENT_SETPITCH_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetVolume",
        detect_size: 0x0087,
        entries: DSOUND_CMCPXVOICECLIENT_SETVOLUME_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DSound_CMemoryManager_PoolAlloc",
        detect_size: 0x0045,
        entries: DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundCreate",
        detect_size: 0x0044,
        entries: DSOUND_DIRECTSOUNDCREATE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundCreateBuffer",
        detect_size: 0x0056,
        entries: DSOUND_DIRECTSOUNDCREATEBUFFER_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundCreateStream",
        detect_size: 0x0056,
        entries: DSOUND_DIRECTSOUNDCREATESTREAM_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundDoWork",
        detect_size: 0x0029,
        entries: DSOUND_DIRECTSOUNDDOWORK_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundOverrideSpeakerConfig",
        detect_size: 0x0034,
        entries: DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "DirectSoundUseFullHRTF",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSEFULLHRTF_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetDistanceFactor",
        detect_size: 0x0024,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetDopplerFactor",
        detect_size: 0x0024,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetRolloffFactor",
        detect_size: 0x0024,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundStream_FlushEx",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_FLUSHEX_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetDistanceFactor",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetDopplerFactor",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetRolloffFactor",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "XAudioDownloadEffectsImage",
        detect_size: 0x006B,
        entries: DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4134_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4134,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        detect_size: 0x005E,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CDirectSound_GetSpeakerConfig",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CFullHrtfSource_GetHrtfFilterPair",
        detect_size: 0x005A,
        entries: DSOUND_CFULLHRTFSOURCE_GETHRTFFILTERPAIR_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CHrtfSource_SetAlgorithm_FullHrtf",
        detect_size: 0x001C,
        entries: DSOUND_CHRTFSOURCE_SETALGORITHM_FULLHRTF_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CHrtfSource_SetAlgorithm_LightHrtf",
        detect_size: 0x001F,
        entries: DSOUND_CHRTFSOURCE_SETALGORITHM_LIGHTHRTF_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CLightHrtfSource_GetHrtfFilterPair",
        detect_size: 0x008C,
        entries: DSOUND_CLIGHTHRTFSOURCE_GETHRTFFILTERPAIR_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Stop",
        detect_size: 0x001E,
        entries: DSOUND_CMCPXBUFFER_STOP_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x007C,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetEG",
        detect_size: 0x0052,
        entries: DSOUND_CMCPXVOICECLIENT_SETEG_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_SetLFO",
        detect_size: 0x0052,
        entries: DSOUND_CMCPXVOICECLIENT_SETLFO_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "DirectSoundUseFullHRTF",
        detect_size: 0x001D,
        entries: DSOUND_DIRECTSOUNDUSEFULLHRTF_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "DirectSoundUseLightHRTF",
        detect_size: 0x001D,
        entries: DSOUND_DIRECTSOUNDUSELIGHTHRTF_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "XFileCreateMediaObject",
        detect_size: 0x0082,
        entries: DSOUND_XFILECREATEMEDIAOBJECT_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "XFileCreateMediaObjectEx",
        detect_size: 0x0074,
        entries: DSOUND_XFILECREATEMEDIAOBJECTEX_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "XWaveFileCreateMediaObject",
        detect_size: 0x008B,
        entries: DSOUND_XWAVEFILECREATEMEDIAOBJECT_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "XWaveFileCreateMediaObjectEx",
        detect_size: 0x0087,
        entries: DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4242_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4242,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_SetRolloffCurve",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundStream_PauseEx",
        detect_size: 0x0058,
        entries: DSOUND_CDIRECTSOUNDSTREAM_PAUSEEX_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundStream_SetRolloffCurve",
        detect_size: 0x0054,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDistanceFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDopplerFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMaxDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMinDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetRolloffCurve",
        detect_size: 0x0031,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetRolloffFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSound_GetCaps",
        detect_size: 0x006B,
        entries: DSOUND_CDIRECTSOUND_GETCAPS_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CDirectSound_GetOutputLevels",
        detect_size: 0x0045,
        entries: DSOUND_CDIRECTSOUND_GETOUTPUTLEVELS_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0074,
        entries: DSOUND_CMCPXBUFFER_PLAY_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CMcpxStream_Flush",
        detect_size: 0x00D3,
        entries: DSOUND_CMCPXSTREAM_FLUSH_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "CMcpxStream_Pause_Ex",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXSTREAM_PAUSE_EX_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "DSound_CMemoryManager_PoolAlloc",
        detect_size: 0x0047,
        entries: DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "DirectSoundGetSampleTime",
        detect_size: 0x0010,
        entries: DSOUND_DIRECTSOUNDGETSAMPLETIME_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_SetRolloffCurve",
        detect_size: 0x0022,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "IDirectSoundStream_PauseEx",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUNDSTREAM_PAUSEEX_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "IDirectSoundStream_SetRolloffCurve",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "IDirectSound_GetOutputLevels",
        detect_size: 0x001F,
        entries: DSOUND_IDIRECTSOUND_GETOUTPUTLEVELS_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "XFileCreateMediaObject",
        detect_size: 0x0069,
        entries: DSOUND_XFILECREATEMEDIAOBJECT_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "XFileCreateMediaObjectEx",
        detect_size: 0x005B,
        entries: DSOUND_XFILECREATEMEDIAOBJECTEX_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "XWaveFileCreateMediaObject",
        detect_size: 0x0072,
        entries: DSOUND_XWAVEFILECREATEMEDIAOBJECT_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "XWaveFileCreateMediaObjectEx",
        detect_size: 0x006E,
        entries: DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4361_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4361,
    },
    OovpaPattern {
        name: "XFileCreateMediaObjectAsync",
        detect_size: 0x005F,
        entries: DSOUND_XFILECREATEMEDIAOBJECTASYNC_4432_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4432,
    },
    OovpaPattern {
        name: "CMcpxStream_Discontinuity",
        detect_size: 0x002F,
        entries: DSOUND_CMCPXSTREAM_DISCONTINUITY_4531_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4531,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDistanceFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDopplerFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CDirectSound_SetDistanceFactor",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CDirectSound_SetDopplerFactor",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CDirectSound_SetVelocity",
        detect_size: 0x0075,
        entries: DSOUND_CDIRECTSOUND_SETVELOCITY_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x007D,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CMemoryManager_MemAlloc",
        detect_size: 0x0067,
        entries: DSOUND_CMEMORYMANAGER_MEMALLOC_4627_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4627,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Pause",
        detect_size: 0x0050,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PAUSE_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_PauseEx",
        detect_size: 0x0058,
        entries: DSOUND_CDIRECTSOUNDBUFFER_PAUSEEX_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetFormat",
        detect_size: 0x0043,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Pause",
        detect_size: 0x0044,
        entries: DSOUND_CMCPXBUFFER_PAUSE_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Pause_Ex",
        detect_size: 0x0038,
        entries: DSOUND_CMCPXBUFFER_PAUSE_EX_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0054,
        entries: DSOUND_CMCPXBUFFER_PLAY_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CMcpxStream_GetStatus",
        detect_size: 0x0042,
        entries: DSOUND_CMCPXSTREAM_GETSTATUS_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Pause",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_PAUSE_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_PauseEx",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUNDBUFFER_PAUSEEX_4721_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4721,
    },
    OovpaPattern {
        name: "CDirectSound_SynchPlayback",
        detect_size: 0x000E,
        entries: DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "CMcpxAPU_SynchPlayback",
        detect_size: 0x0100,
        entries: DSOUND_CMCPXAPU_SYNCHPLAYBACK_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Pause",
        detect_size: 0x0072,
        entries: DSOUND_CMCPXBUFFER_PAUSE_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "CMcpxBuffer_Play",
        detect_size: 0x0073,
        entries: DSOUND_CMCPXBUFFER_PLAY_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "CMcpxStream_Pause",
        detect_size: 0x008D,
        entries: DSOUND_CMCPXSTREAM_PAUSE_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "IDirectSound_SynchPlayback",
        detect_size: 0x0017,
        entries: DSOUND_IDIRECTSOUND_SYNCHPLAYBACK_4831_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 4831,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_GetVoiceProperties",
        detect_size: 0x004D,
        entries: DSOUND_CDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Flush",
        detect_size: 0x004A,
        entries: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CDirectSoundStream_GetVoiceProperties",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_GetVoiceProperties",
        detect_size: 0x0012,
        entries: DSOUND_CDIRECTSOUNDVOICE_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CMcpxStream_Stop",
        detect_size: 0x0023,
        entries: DSOUND_CMCPXSTREAM_STOP_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x0076,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_GetVoiceProperties",
        detect_size: 0x0108,
        entries: DSOUND_CMCPXVOICECLIENT_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_GetVoiceProperties",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "IDirectSoundStream_GetVoiceProperties",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "XAudioSetEffectData",
        detect_size: 0x00D5,
        entries: DSOUND_XAUDIOSETEFFECTDATA_5028_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5028,
    },
    OovpaPattern {
        name: "CDirectSound3DCalculator_Calculate3D",
        detect_size: 0x00D6,
        entries: DSOUND_CDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound3DCalculator_GetMixBinVolumes",
        detect_size: 0x00B0,
        entries: DSOUND_CDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound3DCalculator_GetPanData",
        detect_size: 0x0090,
        entries: DSOUND_CDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound3DCalculator_GetVoiceData",
        detect_size: 0x0092,
        entries: DSOUND_CDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        detect_size: 0x0011,
        entries: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeAngles",
        detect_size: 0x0031,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOrientation",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDistanceFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetDopplerFactor",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetI3DL2Source",
        detect_size: 0x00C9,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMaxDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMinDistance",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetMode",
        detect_size: 0x0032,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETMODE_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetPosition",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetRolloffCurve",
        detect_size: 0x0031,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetRolloffFactor",
        detect_size: 0x0032,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_SetVelocity",
        detect_size: 0x0033,
        entries: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_CommitDeferredSettings",
        detect_size: 0x0098,
        entries: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_EnableHeadphones",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_MapBufferData",
        detect_size: 0x0068,
        entries: DSOUND_CDIRECTSOUND_MAPBUFFERDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SetDistanceFactor",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SetDopplerFactor",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SetPosition",
        detect_size: 0x004A,
        entries: DSOUND_CDIRECTSOUND_SETPOSITION_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SetRolloffFactor",
        detect_size: 0x004E,
        entries: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SetVelocity",
        detect_size: 0x0072,
        entries: DSOUND_CDIRECTSOUND_SETVELOCITY_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_SynchPlayback",
        detect_size: 0x003C,
        entries: DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSound_UnmapBufferData",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUND_UNMAPBUFFERDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CFullHRTFSource_GetCenterVolume",
        detect_size: 0x0092,
        entries: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CHRTFSource_SetFullHRTF4Channel",
        detect_size: 0x006F,
        entries: DSOUND_CHRTFSOURCE_SETFULLHRTF4CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CHRTFSource_SetFullHRTF5Channel",
        detect_size: 0x006F,
        entries: DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CHRTFSource_SetLightHRTF4Channel",
        detect_size: 0x006F,
        entries: DSOUND_CHRTFSOURCE_SETLIGHTHRTF4CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CHRTFSource_SetLightHRTF5Channel",
        detect_size: 0x006F,
        entries: DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CLightHRTFSource_GetCenterVolume",
        detect_size: 0x0068,
        entries: DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CMcpxStream_Flush",
        detect_size: 0x0012,
        entries: DSOUND_CMCPXSTREAM_FLUSH_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x0070,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "DirectSoundUseFullHRTF4Channel",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSEFULLHRTF4CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "DirectSoundUseLightHRTF",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSELIGHTHRTF_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "DirectSoundUseLightHRTF4Channel",
        detect_size: 0x001F,
        entries: DSOUND_DIRECTSOUNDUSELIGHTHRTF4CHANNEL_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound3DCalculator_Calculate3D",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound3DCalculator_GetMixBinVolumes",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound3DCalculator_GetPanData",
        detect_size: 0x0020,
        entries: DSOUND_IDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound3DCalculator_GetVoiceData",
        detect_size: 0x0009,
        entries: DSOUND_IDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound_MapBufferData",
        detect_size: 0x0023,
        entries: DSOUND_IDIRECTSOUND_MAPBUFFERDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "IDirectSound_UnmapBufferData",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUND_UNMAPBUFFERDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "XAudioSetEffectData",
        detect_size: 0x00C6,
        entries: DSOUND_XAUDIOSETEFFECTDATA_5344_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5344,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Set3DVoiceData",
        detect_size: 0x004D,
        entries: DSOUND_CDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Set3DVoiceData",
        detect_size: 0x0051,
        entries: DSOUND_CDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_Set3DVoiceData",
        detect_size: 0x00CE,
        entries: DSOUND_CDIRECTSOUNDVOICE_SET3DVOICEDATA_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSound_CommitDeferredSettings",
        detect_size: 0x0097,
        entries: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSound_EnableHeadphones",
        detect_size: 0x011A,
        entries: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSound_GetSpeakerConfig",
        detect_size: 0x004C,
        entries: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        detect_size: 0x001A,
        entries: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CMcpxStream_Discontinuity",
        detect_size: 0x002D,
        entries: DSOUND_CMCPXSTREAM_DISCONTINUITY_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CMcpxVoiceClient_Commit3dSettings",
        detect_size: 0x006F,
        entries: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "DSound_CRefCount_Release",
        detect_size: 0x0020,
        entries: DSOUND_DSOUND_CREFCOUNT_RELEASE_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Set3DVoiceData",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "IDirectSoundStream_Set3DVoiceData",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "XAudioCalculatePitch",
        detect_size: 0x0049,
        entries: DSOUND_XAUDIOCALCULATEPITCH_5455_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5455,
    },
    OovpaPattern {
        name: "CDirectSoundBuffer_Use3DVoiceData",
        detect_size: 0x004B,
        entries: DSOUND_CDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5558,
    },
    OovpaPattern {
        name: "CDirectSoundStream_Use3DVoiceData",
        detect_size: 0x004F,
        entries: DSOUND_CDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5558,
    },
    OovpaPattern {
        name: "CDirectSoundVoice_Use3DVoiceData",
        detect_size: 0x001C,
        entries: DSOUND_CDIRECTSOUNDVOICE_USE3DVOICEDATA_5558_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5558,
    },
    OovpaPattern {
        name: "IDirectSoundBuffer_Use3DVoiceData",
        detect_size: 0x001B,
        entries: DSOUND_IDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5558,
    },
    OovpaPattern {
        name: "IDirectSoundStream_Use3DVoiceData",
        detect_size: 0x0005,
        entries: DSOUND_IDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_ENTRIES,
        argc: 0,
        hle_mode: HleMode::Hle,
        min_version: 5558,
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
    OovpaPatternMeta {
        name: "CDirectSoundBufferSettings_SetBufferData",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFERSETTINGS_SETBUFFERDATA_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetStatus",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Lock",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_LOCK_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Play",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_PlayEx",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetAllParameters",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeAngles",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOrientation",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFrequency",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFREQUENCY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLoopRegion",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMaxDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMinDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMode",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetOutputBuffer",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETOUTPUTBUFFER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPlayRegion",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVelocity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Stop",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOP_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_StopEx",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_AddRef",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Discontinuity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Flush",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetInfo",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetStatus",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Pause",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Process",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PROCESS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Release",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetAllParameters",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeAngles",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOrientation",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFrequency",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetI3DL2Source",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMaxDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMinDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMode",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetOutputBuffer",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVelocity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoiceSettings_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetAllParameters",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeAngles",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOrientation",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFrequency",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFREQUENCY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetI3DL2Source",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMaxDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMinDistance",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINVOLUMES_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMode",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMODE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetOutputBuffer",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETOUTPUTBUFFER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVelocity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitDeferredSettings",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitEffectData",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundBuffer",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundStream",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_DownloadEffectsImage",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_EnableHeadphones",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetCaps",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETCAPS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetEffectData",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETEFFECTDATA_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetAllParameters",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDistanceFactor",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDopplerFactor",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetEffectData",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETEFFECTDATA_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetI3DL2Listener",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetMixBinHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETMIXBINHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetOrientation",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETORIENTATION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetRolloffFactor",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetVelocity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETVELOCITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CFullHRTFSource_GetCenterVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetFullHRTF5Channel",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetLightHRTF5Channel",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CLightHRTFSource_GetCenterVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_SetMixBinHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXAPU_SETMIXBINHEADROOM_4_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetCurrentPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetStatus",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETSTATUS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play_Ex",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_EX_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_SetCurrentPosition",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Discontinuity",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXSTREAM_DISCONTINUITY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Flush",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXSTREAM_FLUSH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Pause",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXSTREAM_PAUSE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CRefCount_AddRef",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DSOUND_CREFCOUNT_ADDREF_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CRefCount_Release",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DSOUND_CREFCOUNT_RELEASE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreate",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateBuffer",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATEBUFFER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateStream",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATESTREAM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundOverrideSpeakerConfig",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseFullHRTF",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDUSEFULLHRTF_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseLightHRTF",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_DIRECTSOUNDUSELIGHTHRTF_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetAllParameters",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETALLPARAMETERS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetConeAngles",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCONEANGLES_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetConeOutsideVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetFrequency",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETFREQUENCY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetI3DL2Source",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINVOLUMES_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetMode",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETMODE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetPlayRegion",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETPLAYREGION_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetEG",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETEG_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetFilter",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETFILTER_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetFrequency",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETFREQUENCY_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetHeadroom",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETHEADROOM_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetLFO",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETLFO_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMixBinVolumes",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetMixBins",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETMIXBINS_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetPitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetVolume",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETVOLUME_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "IsValidFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_ISVALIDFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCalculatePitch",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_XAUDIOCALCULATEPITCH_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCreateAdpcmFormat",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_XAUDIOCREATEADPCMFORMAT_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioDownloadEffectsImage",
        min_version: 4039,
        source_file: "DSound/4039.inl",
        xrefs: DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4039_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetCurrentPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETCURRENTPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetStatus",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETSTATUS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Lock",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_LOCK_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Play",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_PlayEx",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PLAYEX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetAllParameters",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETALLPARAMETERS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeAngles",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEANGLES_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOrientation",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEORIENTATION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetConeOutsideVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCONEOUTSIDEVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETCURRENTPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetEG",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETEG_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFilter",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFILTER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetFormat",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETFORMAT_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetHeadroom",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETHEADROOM_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetI3DL2Source",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETI3DL2SOURCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLFO",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLFO_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetLoopRegion",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETLOOPREGION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMaxDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMAXDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMinDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMINDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetMode",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETMODE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPitch",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPITCH_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPlayRegion",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPLAYREGION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVelocity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVELOCITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Stop",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOP_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_StopEx",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_STOPEX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_AddRef",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_ADDREF_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Discontinuity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_DISCONTINUITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Flush",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_FlushEx",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_FLUSHEX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetInfo",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETINFO_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetStatus",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETSTATUS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Pause",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PAUSE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Release",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_RELEASE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetAllParameters",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETALLPARAMETERS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeAngles",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEANGLES_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOrientation",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEORIENTATION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetConeOutsideVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETCONEOUTSIDEVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetEG",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETEG_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFilter",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFILTER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFormat",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFORMAT_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetFrequency",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETFREQUENCY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetHeadroom",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETHEADROOM_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetI3DL2Source",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETI3DL2SOURCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetLFO",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETLFO_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMaxDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMAXDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMinDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMINDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBinVolumes",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINVOLUMES_8_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMixBins",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMIXBINS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetMode",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETMODE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetOutputBuffer",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETOUTPUTBUFFER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPitch",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPITCH_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVelocity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVELOCITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoiceSettings_SetMixBinVolumes",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICESETTINGS_SETMIXBINVOLUMES_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetAllParameters",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETALLPARAMETERS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeAngles",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOrientation",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetI3DL2Source",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMaxDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMinDistance",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMode",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMODE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVelocity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitEffectData",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITEFFECTDATA_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundBuffer",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDBUFFER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CreateSoundStream",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_CREATESOUNDSTREAM_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_DownloadEffectsImage",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_DOWNLOADEFFECTSIMAGE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_EnableHeadphones",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetCaps",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETCAPS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetEffectData",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETEFFECTDATA_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetAllParameters",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETALLPARAMETERS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetI3DL2Listener",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETI3DL2LISTENER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetOrientation",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETORIENTATION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetVelocity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETVELOCITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CFullHRTFSource_GetCenterVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetCurrentPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETCURRENTPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_GetStatus",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_GETSTATUS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_SetBufferData",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_SETBUFFERDATA_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_SetCurrentPosition",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_SETCURRENTPOSITION_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Stop",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_STOP_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Stop_Ex",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXBUFFER_STOP_EX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Discontinuity",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXSTREAM_DISCONTINUITY_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_GetStatus",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXSTREAM_GETSTATUS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Pause",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXSTREAM_PAUSE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Stop",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXSTREAM_STOP_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Stop_Ex",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXSTREAM_STOP_EX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetEG",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETEG_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetFilter",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETFILTER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetLFO",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETLFO_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetMixBins",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETMIXBINS_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetPitch",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETPITCH_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetVolume",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETVOLUME_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CMemoryManager_PoolAlloc",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreate",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateBuffer",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATEBUFFER_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundCreateStream",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDCREATESTREAM_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundDoWork",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDDOWORK_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundOverrideSpeakerConfig",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDOVERRIDESPEAKERCONFIG_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseFullHRTF",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_DIRECTSOUNDUSEFULLHRTF_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_FlushEx",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_FLUSHEX_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetDistanceFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETDISTANCEFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetDopplerFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETDOPPLERFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetRolloffFactor",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFFACTOR_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioDownloadEffectsImage",
        min_version: 4134,
        source_file: "DSound/4134.inl",
        xrefs: DSOUND_XAUDIODOWNLOADEFFECTSIMAGE_4134_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetNotificationPositions",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETNOTIFICATIONPOSITIONS_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetSpeakerConfig",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CFullHrtfSource_GetHrtfFilterPair",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CFULLHRTFSOURCE_GETHRTFFILTERPAIR_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CHrtfSource_SetAlgorithm_FullHrtf",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETALGORITHM_FULLHRTF_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CHrtfSource_SetAlgorithm_LightHrtf",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETALGORITHM_LIGHTHRTF_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CLightHrtfSource_GetHrtfFilterPair",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CLIGHTHRTFSOURCE_GETHRTFFILTERPAIR_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Stop",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CMCPXBUFFER_STOP_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetEG",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETEG_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_SetLFO",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_SETLFO_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseFullHRTF",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_DIRECTSOUNDUSEFULLHRTF_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseLightHRTF",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_DIRECTSOUNDUSELIGHTHRTF_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObject",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECT_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObjectEx",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECTEX_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "XWaveFileCreateMediaObject",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_XWAVEFILECREATEMEDIAOBJECT_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "XWaveFileCreateMediaObjectEx",
        min_version: 4242,
        source_file: "DSound/4242.inl",
        xrefs: DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4242_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_SetRolloffCurve",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_PauseEx",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_PAUSEEX_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_SetRolloffCurve",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDistanceFactor",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDopplerFactor",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMaxDistance",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMinDistance",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetRolloffCurve",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetRolloffFactor",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetCaps",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETCAPS_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetOutputLevels",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETOUTPUTLEVELS_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Flush",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CMCPXSTREAM_FLUSH_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Pause_Ex",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_CMCPXSTREAM_PAUSE_EX_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CMemoryManager_PoolAlloc",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_DSOUND_CMEMORYMANAGER_POOLALLOC_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundGetSampleTime",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_DIRECTSOUNDGETSAMPLETIME_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_SetRolloffCurve",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SETROLLOFFCURVE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_PauseEx",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_PAUSEEX_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_SetRolloffCurve",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SETROLLOFFCURVE_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_GetOutputLevels",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_IDIRECTSOUND_GETOUTPUTLEVELS_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObject",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECT_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObjectEx",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECTEX_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "XWaveFileCreateMediaObject",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_XWAVEFILECREATEMEDIAOBJECT_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "XWaveFileCreateMediaObjectEx",
        min_version: 4361,
        source_file: "DSound/4361.inl",
        xrefs: DSOUND_XWAVEFILECREATEMEDIAOBJECTEX_4361_XREFS,
    },
    OovpaPatternMeta {
        name: "XFileCreateMediaObjectAsync",
        min_version: 4432,
        source_file: "DSound/4432.inl",
        xrefs: DSOUND_XFILECREATEMEDIAOBJECTASYNC_4432_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Discontinuity",
        min_version: 4531,
        source_file: "DSound/4531.inl",
        xrefs: DSOUND_CMCPXSTREAM_DISCONTINUITY_4531_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDistanceFactor",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDopplerFactor",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDistanceFactor",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDopplerFactor",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetVelocity",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETVELOCITY_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CMemoryManager_MemAlloc",
        min_version: 4627,
        source_file: "DSound/4627.inl",
        xrefs: DSOUND_CMEMORYMANAGER_MEMALLOC_4627_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Pause",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PAUSE_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_PauseEx",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_PAUSEEX_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetFormat",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETFORMAT_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Pause",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CMCPXBUFFER_PAUSE_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Pause_Ex",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CMCPXBUFFER_PAUSE_EX_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_GetStatus",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_CMCPXSTREAM_GETSTATUS_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Pause",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_PAUSE_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_PauseEx",
        min_version: 4721,
        source_file: "DSound/4721.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_PAUSEEX_4721_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SynchPlayback",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_SynchPlayback",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_CMCPXAPU_SYNCHPLAYBACK_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Pause",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_CMCPXBUFFER_PAUSE_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxBuffer_Play",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_CMCPXBUFFER_PLAY_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Pause",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_CMCPXSTREAM_PAUSE_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_SynchPlayback",
        min_version: 4831,
        source_file: "DSound/4831.inl",
        xrefs: DSOUND_IDIRECTSOUND_SYNCHPLAYBACK_4831_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Flush",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_FLUSH_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Stop",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CMCPXSTREAM_STOP_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_GetVoiceProperties",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_GETVOICEPROPERTIES_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioSetEffectData",
        min_version: 5028,
        source_file: "DSound/5028.inl",
        xrefs: DSOUND_XAUDIOSETEFFECTDATA_5028_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound3DCalculator_Calculate3D",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound3DCalculator_GetMixBinVolumes",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound3DCalculator_GetPanData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound3DCalculator_GetVoiceData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_CommitDeferredSettings",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_COMMITDEFERREDSETTINGS_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeAngles",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEANGLES_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOrientation",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEORIENTATION_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetConeOutsideVolume",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETCONEOUTSIDEVOLUME_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDistanceFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDISTANCEFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetDopplerFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETDOPPLERFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetI3DL2Source",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETI3DL2SOURCE_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMaxDistance",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMAXDISTANCE_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMinDistance",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMINDISTANCE_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetMode",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETMODE_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetPosition",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETPOSITION_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetRolloffCurve",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFCURVE_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetRolloffFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETROLLOFFFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_SetVelocity",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SETVELOCITY_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitDeferredSettings",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_EnableHeadphones",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_MapBufferData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_MAPBUFFERDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDistanceFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDISTANCEFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetDopplerFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETDOPPLERFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetPosition",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETPOSITION_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetRolloffFactor",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETROLLOFFFACTOR_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SetVelocity",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SETVELOCITY_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_SynchPlayback",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_SYNCHPLAYBACK_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_UnmapBufferData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CDIRECTSOUND_UNMAPBUFFERDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CFullHRTFSource_GetCenterVolume",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CFULLHRTFSOURCE_GETCENTERVOLUME_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetFullHRTF4Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETFULLHRTF4CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetFullHRTF5Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETFULLHRTF5CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetLightHRTF4Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETLIGHTHRTF4CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CHRTFSource_SetLightHRTF5Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CHRTFSOURCE_SETLIGHTHRTF5CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CLightHRTFSource_GetCenterVolume",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CLIGHTHRTFSOURCE_GETCENTERVOLUME_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Flush",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CMCPXSTREAM_FLUSH_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseFullHRTF4Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_DIRECTSOUNDUSEFULLHRTF4CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseLightHRTF",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_DIRECTSOUNDUSELIGHTHRTF_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "DirectSoundUseLightHRTF4Channel",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_DIRECTSOUNDUSELIGHTHRTF4CHANNEL_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound3DCalculator_Calculate3D",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND3DCALCULATOR_CALCULATE3D_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound3DCalculator_GetMixBinVolumes",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND3DCALCULATOR_GETMIXBINVOLUMES_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound3DCalculator_GetPanData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND3DCALCULATOR_GETPANDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound3DCalculator_GetVoiceData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND3DCALCULATOR_GETVOICEDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_MapBufferData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND_MAPBUFFERDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSound_UnmapBufferData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_IDIRECTSOUND_UNMAPBUFFERDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioSetEffectData",
        min_version: 5344,
        source_file: "DSound/5344.inl",
        xrefs: DSOUND_XAUDIOSETEFFECTDATA_5344_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Set3DVoiceData",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Set3DVoiceData",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_Set3DVoiceData",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_SET3DVOICEDATA_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_CommitDeferredSettings",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUND_COMMITDEFERREDSETTINGS_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_EnableHeadphones",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUND_ENABLEHEADPHONES_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSound_GetSpeakerConfig",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CDIRECTSOUND_GETSPEAKERCONFIG_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxAPU_ServiceDeferredCommandsLow",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CMCPXAPU_SERVICEDEFERREDCOMMANDSLOW_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxStream_Discontinuity",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CMCPXSTREAM_DISCONTINUITY_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CMcpxVoiceClient_Commit3dSettings",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_CMCPXVOICECLIENT_COMMIT3DSETTINGS_0_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "DSound_CRefCount_Release",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_DSOUND_CREFCOUNT_RELEASE_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Set3DVoiceData",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_SET3DVOICEDATA_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_Set3DVoiceData",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_SET3DVOICEDATA_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "XAudioCalculatePitch",
        min_version: 5455,
        source_file: "DSound/5455.inl",
        xrefs: DSOUND_XAUDIOCALCULATEPITCH_5455_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundBuffer_Use3DVoiceData",
        min_version: 5558,
        source_file: "DSound/5558.inl",
        xrefs: DSOUND_CDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundStream_Use3DVoiceData",
        min_version: 5558,
        source_file: "DSound/5558.inl",
        xrefs: DSOUND_CDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_XREFS,
    },
    OovpaPatternMeta {
        name: "CDirectSoundVoice_Use3DVoiceData",
        min_version: 5558,
        source_file: "DSound/5558.inl",
        xrefs: DSOUND_CDIRECTSOUNDVOICE_USE3DVOICEDATA_5558_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundBuffer_Use3DVoiceData",
        min_version: 5558,
        source_file: "DSound/5558.inl",
        xrefs: DSOUND_IDIRECTSOUNDBUFFER_USE3DVOICEDATA_5558_XREFS,
    },
    OovpaPatternMeta {
        name: "IDirectSoundStream_Use3DVoiceData",
        min_version: 5558,
        source_file: "DSound/5558.inl",
        xrefs: DSOUND_IDIRECTSOUNDSTREAM_USE3DVOICEDATA_5558_XREFS,
    },
];
