pub mod dsound;
/// APU (Audio Processing Unit) — MCPX southbridge audio emulation.
///
/// The Xbox APU is an nForce-based audio chip mapped at 0xFE800000-0xFE8FFFFF
/// with AC97 codec registers at 0xFEC00000-0xFEC0FFFF.
///
/// Current approach: DirectSound HLE via OOVPA pattern matching. Buffers and
/// streams are tracked in a host-side registry with simulated play cursors.
/// APU MMIO reads return 0xFFFFFFFF to satisfy hardware polling loops.
/// No audio output.
///
/// Reference: xemu hw/xbox/mcpx/apu/ (full APU emulation with DSP).
/// Reference: Cxbx-R CxbxKrnl/EmuDSound.cpp (HLE stub approach).
/// Reference: C++ v1 AOT_DSound.cpp (this project's original C++ implementation).
pub mod mmio;

use crate::xbox::emulator::debug_log;
use std::sync::atomic::{AtomicU32, Ordering};

/// APU MMIO base addresses.
pub const APU_BASE: u32 = 0xFE80_0000;
pub const APU_END: u32 = 0xFE8F_FFFF;
pub const AC97_BASE: u32 = 0xFEC0_0000;
pub const AC97_END: u32 = 0xFEC0_FFFF;
pub const NIC_BASE: u32 = 0xFEF0_0000;
pub const NIC_END: u32 = 0xFEF0_FFFF;

/// Spider-Man DSOUND section range (XDK 4134).
pub const DSOUND_SECTION_START: u32 = 0x0033_AE80;
pub const DSOUND_SECTION_END: u32 = 0x0035_C1B4;

/// DS_OK — DirectSound success HRESULT.
pub const DS_OK: u32 = 0;

/// DSERR_GENERIC — 0x80004005 (E_FAIL).
#[allow(dead_code)]
pub const DSERR_GENERIC: u32 = 0x8000_4005;

/// True when an OOVPA symbol belongs to the Xbox DirectSound/APU SDK layer.
/// These are safe to HLE: they replace statically-linked SDK code, not game logic.
pub fn is_dsound_hle_name(name: &str) -> bool {
    name.starts_with("DirectSound")
        || name.starts_with("CDirectSound")
        || name.starts_with("IDirectSound")
        || name.starts_with("CMcpx")
        || name.starts_with("XAudio")
}

/// Global monotonic handle counter for fake DirectSound objects.
static NEXT_HANDLE: AtomicU32 = AtomicU32::new(0x00EB_0000);

/// Allocate a new fake handle for a DirectSound object.
fn alloc_fake_handle() -> u32 {
    NEXT_HANDLE.fetch_add(0x100, Ordering::Relaxed)
}

/// APU state — tracks basic audio subsystem state for diagnostics.
pub struct ApuState {
    pub initialized: bool,
    pub sample_rate: u32,
    pub channels: u32,
    pub dsound_handle: u32,
    pub buffers_created: u32,
    pub streams_created: u32,
    pub total_hle_calls: u64,
}

impl ApuState {
    pub fn new() -> Self {
        Self {
            initialized: false,
            sample_rate: 48000, // Xbox default
            channels: 2,        // stereo
            dsound_handle: 0,
            buffers_created: 0,
            streams_created: 0,
            total_hle_calls: 0,
        }
    }
}

impl Default for ApuState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global APU state (lazy-initialized on first HLE call).
static APU_STATE: std::sync::Mutex<Option<ApuState>> = std::sync::Mutex::new(None);

fn with_apu_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut ApuState) -> R,
{
    let mut guard = APU_STATE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(ApuState::new());
    }
    f(guard.as_mut().unwrap())
}

// ============================================================================
// Central HLE dispatcher — called from OOVPA VEH handler
// ============================================================================

/// Dispatch a DirectSound HLE call by OOVPA pattern name.
///
/// `name`       — OOVPA pattern name (e.g. "DirectSoundCreate", "CDirectSoundBuffer_Play")
/// `args`       — stdcall arguments read from guest stack (up to 8)
/// `guest_mem`  — raw pointer to guest memory base (R15)
///
/// Returns: HRESULT (0 = DS_OK for most stubs).
pub fn handle_dsound_hle(name: &str, args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    with_apu_state(|apu| {
        apu.total_hle_calls += 1;
        let call_num = apu.total_hle_calls;

        // Log first 20 calls and then every 100th
        if call_num <= 20 || call_num % 100 == 0 {
            debug_log(&format!(
                "[APU-HLE] #{} {} args=[0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}]",
                call_num, name, args[0], args[1], args[2], args[3]
            ));
        }
    });

    match name {
        // ---- Core lifecycle ----
        "DirectSoundCreate" => dsound::hle_dsound_create(args, guest_mem),
        "DirectSoundDoWork" | "CDirectSound_DoWork" => dsound::hle_do_work(guest_mem),
        "DirectSoundUseFullHRTF" | "DirectSoundUseLightHRTF" => {
            debug_log("[APU-HLE] HRTF mode set (stub)");
            DS_OK
        }
        "DirectSoundOverrideSpeakerConfig" => DS_OK,
        "XAudioDownloadEffectsImage" | "CDirectSound_DownloadEffectsImage" => {
            debug_log("[APU-HLE] DownloadEffectsImage (stub — no DSP)");
            DS_OK
        }

        // ---- IDirectSound methods ----
        "CDirectSound_CreateSoundBuffer" | "IDirectSound_CreateSoundBuffer" => {
            dsound::hle_create_sound_buffer(args, guest_mem, false)
        }
        "DirectSoundCreateBuffer" => dsound::hle_create_sound_buffer(args, guest_mem, true),
        "CDirectSound_CreateSoundStream" | "IDirectSound_CreateSoundStream" => {
            dsound::hle_create_sound_stream(args, guest_mem, false)
        }
        "DirectSoundCreateStream" => dsound::hle_create_sound_stream(args, guest_mem, true),
        "IDirectSound_SetI3DL2Listener" => DS_OK,
        "IDirectSound_SetPosition" => DS_OK,
        "IDirectSound_SetVelocity" => DS_OK,
        "IDirectSound_SetOrientation" => DS_OK,
        "IDirectSound_SetDistanceFactor" => DS_OK,
        "IDirectSound_SetRolloffFactor" => DS_OK,
        "IDirectSound_SetDopplerFactor" => DS_OK,
        "IDirectSound_CommitDeferredSettings" => DS_OK,
        "IDirectSound_CommitEffectData" => DS_OK,
        "IDirectSound_SetEffectData" => DS_OK,
        "IDirectSound_GetEffectData" => DS_OK,
        "IDirectSound_EnableHeadphones" => DS_OK,
        "IDirectSound_GetCaps" => DS_OK,
        "IDirectSound_GetSpeakerConfig" => DS_OK,
        "IDirectSound_SetMixBinHeadroom" => DS_OK,
        "IDirectSound_SetAllParameters" => DS_OK,
        "CDirectSound_SetI3DL2Listener" => DS_OK,
        "CDirectSound_SetPosition" => DS_OK,
        "CDirectSound_SetVelocity" => DS_OK,
        "CDirectSound_SetOrientation" => DS_OK,
        "CDirectSound_SetDistanceFactor" => DS_OK,
        "CDirectSound_SetRolloffFactor" => DS_OK,
        "CDirectSound_SetDopplerFactor" => DS_OK,
        "CDirectSound_CommitDeferredSettings" => DS_OK,
        "CDirectSound_CommitEffectData" => DS_OK,
        "CDirectSound_SetEffectData" => DS_OK,
        "CDirectSound_GetEffectData" => DS_OK,
        "CDirectSound_EnableHeadphones" => DS_OK,
        "CDirectSound_GetCaps" => DS_OK,
        "CDirectSound_SetMixBinHeadroom" => DS_OK,
        "CDirectSound_SetAllParameters" => DS_OK,

        // ---- IDirectSoundBuffer methods (stateful) ----
        "CDirectSoundBuffer_Play" | "IDirectSoundBuffer_Play" | "CMcpxBuffer_Play" => {
            dsound::hle_play(args, guest_mem)
        }
        "CDirectSoundBuffer_PlayEx" | "IDirectSoundBuffer_PlayEx" => {
            dsound::hle_play(args, guest_mem)
        }
        "CDirectSoundBuffer_Stop" | "IDirectSoundBuffer_Stop" | "CMcpxBuffer_Stop" => {
            dsound::hle_stop(args, guest_mem)
        }
        "CDirectSoundBuffer_StopEx" | "IDirectSoundBuffer_StopEx" | "CMcpxBuffer_Stop_Ex" => {
            dsound::hle_stop(args, guest_mem)
        }
        "CDirectSoundBuffer_SetBufferData"
        | "IDirectSoundBuffer_SetBufferData"
        | "CDirectSoundBufferSettings_SetBufferData"
        | "CMcpxBuffer_SetBufferData" => dsound::hle_set_buffer_data(args, guest_mem),
        "CDirectSoundBuffer_SetCurrentPosition"
        | "IDirectSoundBuffer_SetCurrentPosition"
        | "CMcpxBuffer_SetCurrentPosition" => dsound::hle_set_current_position(args, guest_mem),
        "CDirectSoundBuffer_GetCurrentPosition"
        | "IDirectSoundBuffer_GetCurrentPosition"
        | "CMcpxBuffer_GetCurrentPosition" => dsound::hle_get_current_position(args, guest_mem),
        "CDirectSoundBuffer_GetStatus"
        | "IDirectSoundBuffer_GetStatus"
        | "CMcpxBuffer_GetStatus" => dsound::hle_get_status(args, guest_mem),
        "CDirectSoundBuffer_Lock" | "IDirectSoundBuffer_Lock" => {
            dsound::hle_buffer_lock(args, guest_mem)
        }

        // ---- IDirectSoundBuffer methods (parameter stubs) ----
        "CDirectSoundBuffer_SetVolume" | "IDirectSoundBuffer_SetVolume" => {
            dsound::hle_set_volume(args, guest_mem)
        }
        "CDirectSoundBuffer_SetFrequency" | "IDirectSoundBuffer_SetFrequency" => {
            dsound::hle_set_frequency(args, guest_mem)
        }
        "CDirectSoundBuffer_SetNotificationPositions"
        | "IDirectSoundBuffer_SetNotificationPositions" => {
            dsound::hle_set_notification_positions(args, guest_mem)
        }
        "CDirectSoundBuffer_SetPitch" => DS_OK,
        "CDirectSoundBuffer_SetHeadroom" => DS_OK,
        "CDirectSoundBuffer_SetMixBins" => DS_OK,
        "CDirectSoundBuffer_SetMixBinVolumes_8" => DS_OK,
        "CDirectSoundBuffer_SetLoopRegion" => DS_OK,
        "CDirectSoundBuffer_SetPlayRegion" => DS_OK,
        "CDirectSoundBuffer_SetMaxDistance" => DS_OK,
        "CDirectSoundBuffer_SetMinDistance" => DS_OK,
        "CDirectSoundBuffer_SetRolloffFactor" => DS_OK,
        "CDirectSoundBuffer_SetDistanceFactor" => DS_OK,
        "CDirectSoundBuffer_SetDopplerFactor" => DS_OK,
        "CDirectSoundBuffer_SetConeAngles" => DS_OK,
        "CDirectSoundBuffer_SetConeOrientation" => DS_OK,
        "CDirectSoundBuffer_SetConeOutsideVolume" => DS_OK,
        "CDirectSoundBuffer_SetPosition" => DS_OK,
        "CDirectSoundBuffer_SetVelocity" => DS_OK,
        "CDirectSoundBuffer_SetI3DL2Source" => DS_OK,
        "CDirectSoundBuffer_SetAllParameters" => DS_OK,
        "CDirectSoundBuffer_SetMode" => DS_OK,
        "CDirectSoundBuffer_SetFilter" => DS_OK,
        "CDirectSoundBuffer_SetFormat" => DS_OK,
        "CDirectSoundBuffer_SetEG" => DS_OK,
        "CDirectSoundBuffer_SetLFO" => DS_OK,
        "CDirectSoundBuffer_SetOutputBuffer" => DS_OK,
        name if name.starts_with("IDirectSoundBuffer_Set") => DS_OK,

        // ---- IDirectSoundBuffer (IDirectSoundBuffer_ COM wrappers) ----
        "IDirectSoundBuffer_SetRolloffFactor" => DS_OK,
        "IDirectSoundBuffer_SetDistanceFactor" => DS_OK,
        "IDirectSoundBuffer_SetDopplerFactor" => DS_OK,

        // ---- IDirectSoundBuffer lifecycle ----
        "CDirectSoundBuffer_Release" | "IDirectSoundBuffer_Release" => {
            dsound::hle_release(args, guest_mem)
        }
        "CDirectSoundBuffer_AddRef" | "IDirectSoundBuffer_AddRef" => {
            dsound::hle_addref(args, guest_mem)
        }

        // ---- IDirectSoundStream methods (stateful) ----
        "CDirectSoundStream_Process" => dsound::hle_stream_process(args, guest_mem),
        "CDirectSoundStream_Pause" | "IDirectSoundStream_Pause" | "CMcpxStream_Pause" => {
            dsound::hle_stream_pause(args, guest_mem)
        }
        "CDirectSoundStream_GetStatus"
        | "IDirectSoundStream_GetStatus"
        | "CMcpxStream_GetStatus" => dsound::hle_stream_get_status(args, guest_mem),
        "CDirectSoundStream_Discontinuity"
        | "IDirectSoundStream_Discontinuity"
        | "CMcpxStream_Discontinuity" => DS_OK,
        "CDirectSoundStream_Flush" | "IDirectSoundStream_Flush" | "CMcpxStream_Flush" => DS_OK,
        "CDirectSoundStream_FlushEx" | "IDirectSoundStream_FlushEx" => DS_OK,
        "CDirectSoundStream_GetInfo" | "IDirectSoundStream_GetInfo" => DS_OK,

        // ---- IDirectSoundStream (parameter stubs) ----
        "CDirectSoundStream_SetVolume" => DS_OK,
        "CDirectSoundStream_SetFrequency" => DS_OK,
        "CDirectSoundStream_SetPitch" => DS_OK,
        "CDirectSoundStream_SetHeadroom" => DS_OK,
        "CDirectSoundStream_SetMixBins" => DS_OK,
        "CDirectSoundStream_SetMixBinVolumes_8" => DS_OK,
        "CDirectSoundStream_SetMaxDistance" => DS_OK,
        "CDirectSoundStream_SetMinDistance" => DS_OK,
        "CDirectSoundStream_SetRolloffFactor" => DS_OK,
        "CDirectSoundStream_SetDistanceFactor" => DS_OK,
        "CDirectSoundStream_SetDopplerFactor" => DS_OK,
        "CDirectSoundStream_SetConeOrientation" => DS_OK,
        "CDirectSoundStream_SetConeOutsideVolume" => DS_OK,
        "CDirectSoundStream_SetPosition" => DS_OK,
        "CDirectSoundStream_SetVelocity" => DS_OK,
        "CDirectSoundStream_SetI3DL2Source" => DS_OK,
        "CDirectSoundStream_SetAllParameters" => DS_OK,
        "CDirectSoundStream_SetMode" => DS_OK,
        "CDirectSoundStream_SetFilter" => DS_OK,
        "CDirectSoundStream_SetFormat" => DS_OK,
        "CDirectSoundStream_SetEG" => DS_OK,
        "CDirectSoundStream_SetLFO" => DS_OK,
        "CDirectSoundStream_SetOutputBuffer" => DS_OK,
        "CDirectSoundStream_SetConeAngles" => DS_OK,
        name if name.starts_with("IDirectSoundStream_Set") => DS_OK,

        // ---- IDirectSoundStream lifecycle ----
        "CDirectSoundStream_Release" | "IDirectSoundStream_Release" => {
            dsound::hle_release(args, guest_mem)
        }
        "CDirectSoundStream_AddRef" | "IDirectSoundStream_AddRef" => {
            dsound::hle_addref(args, guest_mem)
        }

        // ---- IDirectSoundStream (IDirectSoundStream_ COM wrappers) ----
        "IDirectSoundStream_SetDistanceFactor" => DS_OK,
        "IDirectSoundStream_SetRolloffFactor" => DS_OK,
        "IDirectSoundStream_SetDopplerFactor" => DS_OK,
        "IDirectSoundStream_FlushEx" => DS_OK,

        // ---- Voice internals ----
        "CDirectSoundVoice_SetVolume" => DS_OK,
        "CDirectSoundVoice_SetMaxDistance" => DS_OK,
        "CDirectSoundVoice_SetMinDistance" => DS_OK,
        "CDirectSoundVoice_SetRolloffFactor" => DS_OK,
        "CDirectSoundVoice_SetDistanceFactor" => DS_OK,
        "CDirectSoundVoice_SetDopplerFactor" => DS_OK,
        "CDirectSoundVoice_SetConeAngles" => DS_OK,
        "CDirectSoundVoice_SetConeOrientation" => DS_OK,
        "CDirectSoundVoice_SetConeOutsideVolume" => DS_OK,
        "CDirectSoundVoice_SetPosition" => DS_OK,
        "CDirectSoundVoice_SetVelocity" => DS_OK,
        "CDirectSoundVoice_SetI3DL2Source" => DS_OK,
        "CDirectSoundVoice_SetAllParameters" => DS_OK,
        "CDirectSoundVoice_SetMode" => DS_OK,
        "CDirectSoundVoice_CommitDeferredSettings" => DS_OK,
        "CDirectSoundVoiceSettings_SetMixBinVolumes" => DS_OK,

        // ---- MCPx hardware layer stubs ----
        "CMcpxVoiceClient_SetVolume" => DS_OK,
        "CMcpxVoiceClient_SetFilter" => DS_OK,
        "CMcpxVoiceClient_SetLFO" => DS_OK,
        "CMcpxVoiceClient_SetEG" => DS_OK,
        "CMcpxVoiceClient_SetMixBins" => DS_OK,
        "CMcpxVoiceClient_SetPitch" => DS_OK,
        "CMcpxVoiceClient_Commit3dSettings_0" => DS_OK,
        "CMcpxAPU_ServiceDeferredCommandsLow" => DS_OK,
        "CMcpxStream_Stop" | "CMcpxStream_Stop_Ex" => DS_OK,

        // ---- Memory manager ----
        "DSound_CMemoryManager_PoolAlloc" => {
            // Return a fake allocation — games check for non-null
            alloc_fake_handle()
        }

        // ---- HRTF internals ----
        "CFullHRTFSource_GetCenterVolume" => DS_OK,

        // ---- Catch-all ----
        _ => {
            debug_log(&format!("[APU-HLE] UNHANDLED: {} — returning DS_OK", name));
            DS_OK
        }
    }
}

// ============================================================================
// NV_PAPU register offsets (from xemu apu_regs.h).
// ============================================================================

#[allow(dead_code)]
pub mod regs {
    // ---- Interrupt status / enable ----
    pub const NV_PAPU_ISTS: u32 = 0x0000_1000;
    pub const NV_PAPU_ISTS_GINTSTS: u32 = 1 << 0;
    pub const NV_PAPU_ISTS_FETINTSTS: u32 = 1 << 4;
    pub const NV_PAPU_IEN: u32 = 0x0000_1004;

    // ---- Front-end (FE) control ----
    pub const NV_PAPU_FECTL: u32 = 0x0000_1100;
    pub const NV_PAPU_FECTL_FEMETHMODE: u32 = 0x0000_00E0;
    pub const NV_PAPU_FECTL_FEMETHMODE_FREE_RUNNING: u32 = 0x0000_0000;
    pub const NV_PAPU_FECTL_FEMETHMODE_HALTED: u32 = 0x0000_0080;
    pub const NV_PAPU_FECTL_FEMETHMODE_TRAPPED: u32 = 0x0000_00E0;
    pub const NV_PAPU_FECTL_FETRAPREASON: u32 = 0x0000_0F00;
    pub const NV_PAPU_FECTL_FETRAPREASON_REQUESTED: u32 = 0x0000_0F00;
    pub const NV_PAPU_FECV: u32 = 0x0000_1110;
    pub const NV_PAPU_FEAV: u32 = 0x0000_1118;
    pub const NV_PAPU_FENADDR: u32 = 0x0000_115C;
    pub const NV_PAPU_FEMEMADDR: u32 = 0x0000_1324;
    pub const NV_PAPU_FEMEMDATA: u32 = 0x0000_1334;

    // ---- Fetch engine (FET) ----
    pub const NV_PAPU_FETFORCE0: u32 = 0x0000_1500;
    pub const NV_PAPU_FETFORCE1: u32 = 0x0000_1504;
    pub const NV_PAPU_FETFORCE1_SE2FE_IDLE_VOICE: u32 = 1 << 15;

    // ---- Setup engine (SE) / global control ----
    pub const NV_PAPU_SECTL: u32 = 0x0000_2000;
    pub const NV_PAPU_SECTL_XCNTMODE: u32 = 0x0000_0018;
    pub const NV_PAPU_SECTL_XCNTMODE_OFF: u32 = 0;
    pub const NV_PAPU_XGSCNT: u32 = 0x0000_200C;

    // ---- Voice processor (VP) ----
    pub const NV_PAPU_VPVADDR: u32 = 0x0000_202C;
    pub const NV_PAPU_VPSGEADDR: u32 = 0x0000_2030;
    pub const NV_PAPU_VPSSLADDR: u32 = 0x0000_2034;

    // ---- GP / EP DSP address registers ----
    pub const NV_PAPU_GPSADDR: u32 = 0x0000_2040;
    pub const NV_PAPU_GPFADDR: u32 = 0x0000_2044;
    pub const NV_PAPU_EPSADDR: u32 = 0x0000_2048;
    pub const NV_PAPU_EPFADDR: u32 = 0x0000_204C;

    // ---- FIFO registers (GP output) ----
    pub const NV_PAPU_GPOFBASE0: u32 = 0x0000_3024;
    pub const NV_PAPU_GPOFEND0: u32 = 0x0000_3028;
    pub const NV_PAPU_GPOFCUR0: u32 = 0x0000_302C;

    // ---- GP/EP DSP scratch + reset ----
    pub const NV_PAPU_GPRST: u32 = 0x0000_FFFC;
    pub const NV_PAPU_EPRST: u32 = 0x0000_FFFC; // EP-local (in EP address space)

    // ---- Spider-Man register hotspot offsets (from C++ AOT_DSound.h) ----
    // These are offsets from APU_BASE (0xFE800000).
    pub const APU_GP_STATUS: u32 = 0x00_1000; // GP status/control (= NV_PAPU_ISTS)
    pub const APU_GP_CONTROL: u32 = 0x00_1004; // GP enable/disable (= NV_PAPU_IEN)
    pub const APU_GP_VOICES_ACTIVE: u32 = 0x00_1100; // Active voice count (= NV_PAPU_FECTL)
    pub const APU_VP_STATUS: u32 = 0x00_200C; // VP status (= NV_PAPU_XGSCNT)
    pub const APU_GP_DSP_SCRATCH: u32 = 0x02_0010; // GP DSP scratch RAM (hottest: 33 refs)
    pub const APU_GP_DSP_CONTROL: u32 = 0x02_0104; // GP DSP control
    pub const APU_EP_STATUS: u32 = 0x03_FFFC; // EP status (4 refs)
    pub const APU_EP_DSP_STATUS: u32 = 0x05_FFFC; // EP DSP status (4 refs)
}

/// Check if an OOVPA pattern name is a DirectSound function that should be
/// dispatched through `handle_dsound_hle` rather than the D3D path.
pub fn is_dsound_function(name: &str) -> bool {
    name.starts_with("DirectSound")
        || name.starts_with("CDirectSound")
        || name.starts_with("IDirectSound")
        || name.starts_with("CMcpx")
        || name.starts_with("DSound_")
        || name.starts_with("CFullHRTF")
        || name.starts_with("XAudioDownload")
}
