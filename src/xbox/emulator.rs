/// XboxEmulator — libretro Core implementation.
/// Loads XBE files into guest memory and displays boot progress.
///
/// We write raw FFI entry points instead of using retro_core! because
/// rust-libretro-sys generates retro_game_info as an opaque 1-byte struct
/// (bindgen limitation), causing retro_core!'s `Some(*game)` to copy only
/// 1 byte of the 32-byte C struct. Our FFI layer handles the pointer correctly.
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use crate::xbox::aot::decoder;
use crate::xbox::aot::emitter;
use crate::xbox::aot::runtime::{
    AddrTier, RuntimeContext, AOT_EXIT_ERROR, AOT_EXIT_HALT, AOT_EXIT_KERNEL_CALL,
    AOT_EXIT_RET_TO_ZERO, AOT_EXIT_RUNNING, AOT_EXIT_SPAWN_THREAD, AOT_EXIT_SYSTEM_TRAP,
    AOT_EXIT_THREAD_EXIT, AOT_EXIT_UNRESOLVED,
};
use crate::xbox::display::stage::{BootEvent, BootStage, StageDisplay, FB_HEIGHT, FB_WIDTH};
use crate::xbox::kernel;
use crate::xbox::loader::xbe::{self, XbeInfo};
use crate::xbox::memory::guest_memory::GuestMemory;

/// Initialize structured logging backend (spliced 2026-04-20 from rustemu-jit).
/// Replaces the old mpsc-channel logger with `log` crate + RetroArch callback +
/// buffered file. Kernel-call ring buffer + symbol table initialized too so
/// crash dumps are useful. Safe to call multiple times (idempotent).
pub(crate) fn init_logger() {
    crate::xbox::logging::init(Some(r"./debug.log"));
    crate::xbox::logging::init_ring_buffer(256);
    crate::xbox::logging::clear_symbols();
}

struct DebugLogStamp {
    seq: u64,
    elapsed_ms: u128,
    thread_id: std::thread::ThreadId,
    wall_clock: Option<DebugWallClock>,
}

#[derive(Clone, Copy)]
struct DebugWallClock {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    millis: u16,
}

#[cfg(windows)]
#[repr(C)]
struct WindowsSystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn GetLocalTime(system_time: *mut WindowsSystemTime);
}

static DEBUG_LOG_SEQ: AtomicU64 = AtomicU64::new(0);
static DEBUG_LOG_START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

#[cfg(windows)]
fn debug_wall_clock_now() -> Option<DebugWallClock> {
    let mut st = WindowsSystemTime {
        year: 0,
        month: 0,
        day_of_week: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        milliseconds: 0,
    };
    unsafe {
        GetLocalTime(&mut st);
    }
    Some(DebugWallClock {
        year: st.year,
        month: st.month,
        day: st.day,
        hour: st.hour,
        minute: st.minute,
        second: st.second,
        millis: st.milliseconds,
    })
}

#[cfg(not(windows))]
fn debug_wall_clock_now() -> Option<DebugWallClock> {
    None
}

fn debug_log_stamp() -> DebugLogStamp {
    let start = DEBUG_LOG_START.get_or_init(std::time::Instant::now);
    DebugLogStamp {
        seq: DEBUG_LOG_SEQ.fetch_add(1, Ordering::Relaxed),
        elapsed_ms: start.elapsed().as_millis(),
        thread_id: std::thread::current().id(),
        wall_clock: debug_wall_clock_now(),
    }
}

fn format_stamped_debug_line(msg: &str, stamp: &DebugLogStamp) -> String {
    let seconds = stamp.elapsed_ms / 1000;
    let millis = stamp.elapsed_ms % 1000;
    if let Some(wall) = stamp.wall_clock {
        format!(
            "[rustemu] {} [wall={:02}:{:02}:{:02}.{:03} {:02}-{:02}-{:04} t={:06}.{:03}s seq={:08} tid={:?}]",
            msg,
            wall.hour,
            wall.minute,
            wall.second,
            wall.millis,
            wall.day,
            wall.month,
            wall.year,
            seconds,
            millis,
            stamp.seq,
            stamp.thread_id
        )
    } else {
        format!(
            "[rustemu] {} [t={:06}.{:03}s seq={:08} tid={:?}]",
            msg, seconds, millis, stamp.seq, stamp.thread_id
        )
    }
}

fn write_stamped_debug_line<W: Write>(f: &mut W, msg: &str, stamp: &DebugLogStamp) {
    let line = format_stamped_debug_line(msg, stamp);
    let _ = writeln!(f, "{}", line);
}

fn async_debug_log_enabled() -> bool {
    use std::sync::OnceLock;
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_LOG_ASYNC")
            .map(|v| {
                let v = v.trim();
                !(v == "0"
                    || v.eq_ignore_ascii_case("false")
                    || v.eq_ignore_ascii_case("no")
                    || v.eq_ignore_ascii_case("off"))
            })
            .unwrap_or(true)
    })
}

fn enqueue_debug_log_line(line: String) -> bool {
    use std::io::Write;
    use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
    use std::sync::OnceLock;
    use std::time::Duration;

    static SENDER: OnceLock<Option<SyncSender<String>>> = OnceLock::new();
    let sender = SENDER.get_or_init(|| {
        let (tx, rx) = sync_channel::<String>(65_536);
        let spawn_result = std::thread::Builder::new()
            .name("rustemu-debug-log".to_string())
            .spawn(move || {
                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(r"./debug.log")
                    .ok();
                let mut pending = 0usize;
                loop {
                    match rx.recv_timeout(Duration::from_millis(250)) {
                        Ok(line) => {
                            if let Some(ref mut f) = file {
                                let _ = f.write_all(line.as_bytes());
                                pending += 1;
                                if pending >= 512 {
                                    let _ = f.flush();
                                    pending = 0;
                                }
                            }
                        }
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                            if pending != 0 {
                                if let Some(ref mut f) = file {
                                    let _ = f.flush();
                                }
                                pending = 0;
                            }
                        }
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                            if let Some(ref mut f) = file {
                                let _ = f.flush();
                            }
                            break;
                        }
                    }
                }
            });
        spawn_result.ok().map(|_| tx)
    });

    let Some(sender) = sender.as_ref() else {
        return false;
    };
    match sender.try_send(line) {
        Ok(()) => true,
        Err(TrySendError::Full(_)) => true,
        Err(TrySendError::Disconnected(_)) => false,
    }
}

/// Legacy shim — hundreds of call sites use `debug_log(&format!(...))`. We
/// write directly to the hardcoded debug.log path AND route through the log
/// crate. Direct write guarantees visibility even if set_logger raced (another
/// dylib may have installed a logger first), while log::debug! enables the
/// RetroArch callback + UWP-safe path + ring-buffer features.
pub fn debug_log(msg: &str) {
    fn env_truthy(name: &str) -> bool {
        std::env::var(name)
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes"))
            .unwrap_or(false)
    }

    fn env_falsey(name: &str) -> bool {
        std::env::var(name)
            .map(|v| v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("no"))
            .unwrap_or(false)
    }

    fn verbose_main_log() -> bool {
        use std::sync::OnceLock;
        static ENABLED: OnceLock<bool> = OnceLock::new();
        *ENABLED.get_or_init(|| env_truthy("RUSTEMU_VERBOSE_LOG"))
    }

    fn should_write_main_log(msg: &str) -> bool {
        if verbose_main_log() {
            return true;
        }

        if (msg.starts_with("[ALLOC-WATCH]") || msg.starts_with("[ALLOC-WATCH-EXTRA]"))
            && !env_truthy("RUSTEMU_LOG_ALLOC_WATCH")
        {
            return false;
        }

        if (msg.starts_with("[WATCH-ACTIVE-CTX]")
            || msg.starts_with("[WATCH-DRAW-118]")
            || msg.starts_with("[WATCH-"))
            && !env_truthy("RUSTEMU_LOG_WATCH")
        {
            return false;
        }

        if msg.starts_with("[D3D11-CHIS22-DRAW]") && !env_truthy("RUSTEMU_LOG_CHIS22_DRAW") {
            return false;
        }

        if msg.starts_with("[APU-NOTIFY]") && !env_truthy("RUSTEMU_LOG_APU_NOTIFY") {
            return false;
        }

        if msg.starts_with("NtReadFile(") && !env_truthy("RUSTEMU_LOG_NTREADFILE") {
            return false;
        }

        if msg.starts_with("[FIXUP]") && !env_truthy("RUSTEMU_LOG_FIXUP") {
            return false;
        }

        if msg.starts_with("[GAME-MAIN]") && !env_truthy("RUSTEMU_LOG_GAME_MAIN") {
            return false;
        }

        // The script TAP stream can be hundreds of MB per useful run. Keep the
        // explicit gate/milestone lines, but hide per-frame TAP snapshots unless
        // RUSTEMU_VERBOSE_LOG=1 is requested.
        if msg.starts_with("[SPIDEY-TAP #")
            || msg.starts_with("[SPIDEY-F2740-INPUT-MGR]")
            || msg.starts_with("[SPIDEY-F28-BRANCH]")
            || msg.starts_with("[SPIDEY-F27-STATE4]")
            || msg.starts_with("[SPIDEY-SELECT-PRESSED")
            || (msg.starts_with("[SPIDEY-F8580-GATES]")
                && !env_truthy("RUSTEMU_SPIDEY_F8580_TRACE"))
            || msg.starts_with("[SPIDEY-FSM-GATE]")
        {
            return false;
        }

        // Constant-update probes are useful in the sidecar GPU log, but too
        // repetitive for the primary timeline.
        if msg.starts_with("[D3D11-VS-CONST-SKIN]") || msg.starts_with("[VS-CONST-SCREENSPACE]") {
            return false;
        }

        // High-frequency render/dispatch diagnostics belong in the focused
        // sidecar logs. Writing them to debug.log can turn a single run into
        // hundreds of MB and slow the guest enough to miss the draw window.
        if msg.starts_with("[D3D] #")
            || msg.starts_with("[ESP-TRACK]")
            || msg.starts_with("[PS-RS-DRAW]")
            || msg.starts_with("[TSS-DRAW]")
            || msg.starts_with("[HLE-DRAWIDX]")
            || msg.starts_with("[HLE-DRAWIDX-")
            || msg.starts_with("[D3D11-RT]")
            || msg.starts_with("[D3D11-RT-TEX]")
            || msg.starts_with("[D3D11-DRAW #")
            || msg.starts_with("[D3D11-RAWPOS-GUARD]")
            || msg.starts_with("[D3D11-PSH-BIND]")
            || msg.starts_with("[D3D11-REGION-DIAG")
            || msg.starts_with("[D3D11-TEX-DIAG")
            || msg.starts_with("[D3D11-TILE-DIAG")
            || msg.starts_with("[D3D11-VSH-COMPILE]")
            || msg.starts_with("[HLE-SWAP")
            || msg.starts_with("[HLE-VSH-BIND]")
            || msg.starts_with("[VS-SKIN-DRAW-CONSTS]")
            || msg.starts_with("[NV2A-VSH-REGISTER]")
            || (msg.starts_with("[SPIDEY-EVENT-LISTENER-REGISTER")
                && !env_truthy("RUSTEMU_SPIDEY_EVENT_TRACE"))
            || msg.starts_with("[SPIDEY-ROOT-WRITE")
            || msg.starts_with("[SPIDEY-SIGNAL-")
            || msg.starts_with("[SPIDEY-SCRIPT-51C00]")
            || msg.starts_with("[SPIDEY-SCRIPT-PC-WRITE]")
            || msg.starts_with("[SPIDEY-SCRIPT-NATIVE]")
            || msg.starts_with("[SPIDEY-SCRIPT-VM-")
            || msg.starts_with("[SPIDEY-NATIVE-")
        {
            return false;
        }

        true
    }

    fn should_write_gate3_gpu(msg: &str) -> bool {
        if env_falsey("RUSTEMU_GATE3_GPU_LOG") {
            return false;
        }
        msg.starts_with("[HLE-DRAWIDX]")
            || msg.starts_with("[HLE-DRAWIDX-")
            || msg.starts_with("[D3D11-RAWPOS-GUARD]")
            || msg.starts_with("[D3D11-READBACK]")
            || msg.starts_with("[D3D11-DRAW #")
            || msg.starts_with("[D3D11-PREFLIGHT-FAIL]")
            || msg.starts_with("[D3D11-PSH-BIND]")
            || msg.starts_with("[D3D11-REGION-DIAG")
            || msg.starts_with("[D3D11-TEX-DIAG")
            || msg.starts_with("[D3D11-TILE-DIAG")
            || msg.starts_with("[D3D11-VSH-COMPILE]")
            || msg.starts_with("[D3D11-VS-CONST-SKIN]")
            || msg.starts_with("[D3D11-CLEAR]")
            || msg.starts_with("[D3D11-RT")
            || msg.starts_with("[B-DIAG-C212]")
            || msg.starts_with("[HLE-SWAP")
            || msg.starts_with("[HLE-VSH-BIND]")
            || msg.starts_with("[NV2A-VSH-REGISTER]")
            || msg.starts_with("[VS-CONST-SCREENSPACE]")
            || msg.starts_with("[VS-SKIN-DRAW-CONSTS]")
    }

    fn write_gate3_gpu_log(msg: &str, stamp: &DebugLogStamp) {
        use std::sync::{Mutex, OnceLock};

        static GPU_LOG_FILE: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();
        let handle_lock = GPU_LOG_FILE.get_or_init(|| {
            Mutex::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(r"./gate3_gpu.log")
                    .ok(),
            )
        });
        if let Ok(mut guard) = handle_lock.lock() {
            if let Some(ref mut f) = *guard {
                write_stamped_debug_line(f, msg, stamp);
            }
        }
    }

    let write_gate3 = should_write_gate3_gpu(msg);
    let write_main = should_write_main_log(msg);
    if !write_gate3 && !write_main {
        return;
    }

    let log_prof = crate::xbox::profiler::start(crate::xbox::profiler::CpuPhase::DebugLog);
    let stamp = debug_log_stamp();

    if write_gate3 {
        write_gate3_gpu_log(msg, &stamp);
    }

    if !write_main {
        crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::DebugLog, log_prof);
        return;
    }

    let line = format_stamped_debug_line(msg, &stamp);
    if async_debug_log_enabled() && enqueue_debug_log_line(format!("{}\n", line)) {
        crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::DebugLog, log_prof);
        return;
    }

    // Cached file handle — avoid CreateFileW + CloseHandle per call
    // (~5-20μs on NTFS per open). Measured: 19,272 calls/run × 50μs =
    // ~1s of blocking I/O previously. Now: single open, append forever,
    // flush per write. Agent S10-A24 / S10-A25 audit.
    use std::sync::Mutex;
    use std::sync::OnceLock;
    static LOG_FILE: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();
    let handle_lock = LOG_FILE.get_or_init(|| {
        Mutex::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(r"./debug.log")
                .ok(),
        )
    });
    if let Ok(mut guard) = handle_lock.lock() {
        if let Some(ref mut f) = *guard {
            let _ = writeln!(f, "{}", line);
        }
    }
    // Also route through the log crate only when explicitly requested. The log
    // backend writes to the same desktop debug.log file, so forwarding every
    // legacy debug_log call creates duplicate lines during PC diagnosis. UWP
    // runs can opt into the RetroArch callback path once the printf-style shim
    // is in place.
    if env_truthy("RUSTEMU_FORWARD_DEBUG_LOG")
        || std::env::var_os("RUSTEMU_ENABLE_UNSAFE_RETRO_LOG_CALLBACK").is_some()
    {
        log::debug!(target: "emu", "{}", msg);
    }
    crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::DebugLog, log_prof);
}

// ============================================================================
// Raw libretro types (matching the real C ABI, not the opaque bindgen output)
// ============================================================================

#[repr(C)]
pub struct RetroSystemInfo {
    pub library_name: *const std::os::raw::c_char,
    pub library_version: *const std::os::raw::c_char,
    pub valid_extensions: *const std::os::raw::c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
}

#[repr(C)]
pub struct RetroGameGeometry {
    pub base_width: u32,
    pub base_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub aspect_ratio: f32,
}

#[repr(C)]
pub struct RetroSystemTiming {
    pub fps: f64,
    pub sample_rate: f64,
}

#[repr(C)]
pub struct RetroSystemAvInfo {
    pub geometry: RetroGameGeometry,
    pub timing: RetroSystemTiming,
}

#[repr(C)]
pub struct RetroGameInfo {
    pub path: *const std::os::raw::c_char,
    pub data: *const std::os::raw::c_void,
    pub size: usize,
    pub meta: *const std::os::raw::c_char,
}

#[repr(C)]
struct RetroLogCallback {
    log: *const std::os::raw::c_void,
}

pub(crate) const RETRO_API_VERSION: u32 = 1;
pub(crate) const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: u32 = 10;
pub(crate) const RETRO_ENVIRONMENT_GET_VARIABLE: u32 = 15;
pub(crate) const RETRO_ENVIRONMENT_SET_VARIABLES: u32 = 16;
pub(crate) const RETRO_ENVIRONMENT_GET_LOG_INTERFACE: u32 = 27;
pub(crate) const RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: u32 = 18;
pub(crate) const RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY: u32 = 9;
pub(crate) const RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY: u32 = 31;
pub(crate) const RETRO_ENVIRONMENT_EXPERIMENTAL: u32 = 0x10000;
pub(crate) const RETRO_ENVIRONMENT_SET_MEMORY_MAPS: u32 = 36 | RETRO_ENVIRONMENT_EXPERIMENTAL;
pub(crate) const RETRO_PIXEL_FORMAT_XRGB8888: u32 = 1;
pub(crate) const RETRO_MEMORY_SYSTEM_RAM: u32 = 2;
pub(crate) const RETRO_MEMORY_VIDEO_RAM: u32 = 3;
const RETRO_MEMDESC_SYSTEM_RAM: u64 = 1 << 2;
const RETRO_MEMDESC_VIDEO_RAM: u64 = 1 << 4;
const XBOX_EXPOSED_SYSTEM_RAM_SIZE: usize = 64 * 1024 * 1024;
const XBOX_VRAM_START: usize = 0x03C0_0000;
const XBOX_EXPOSED_VRAM_SIZE: usize = 4 * 1024 * 1024;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct RetroMemoryDescriptor {
    pub flags: u64,
    pub ptr: *mut std::os::raw::c_void,
    pub offset: usize,
    pub start: usize,
    pub select: usize,
    pub disconnect: usize,
    pub len: usize,
    pub addrspace: *const std::os::raw::c_char,
}

#[repr(C)]
pub(crate) struct RetroMemoryMap {
    pub descriptors: *const RetroMemoryDescriptor,
    pub num_descriptors: u32,
}

fn retro_memory_map_descriptors() -> *mut RetroMemoryDescriptor {
    static DESCRIPTORS: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *DESCRIPTORS.get_or_init(|| {
        let descs = Box::new([
            RetroMemoryDescriptor {
                flags: 0,
                ptr: std::ptr::null_mut(),
                offset: 0,
                start: 0,
                select: 0,
                disconnect: 0,
                len: 0,
                addrspace: std::ptr::null(),
            },
            RetroMemoryDescriptor {
                flags: 0,
                ptr: std::ptr::null_mut(),
                offset: 0,
                start: 0,
                select: 0,
                disconnect: 0,
                len: 0,
                addrspace: std::ptr::null(),
            },
        ]);
        Box::into_raw(descs) as usize
    }) as *mut RetroMemoryDescriptor
}

fn retro_memory_api_enabled() -> bool {
    std::env::var_os("RUSTEMU_ENABLE_RETRO_MEMORY").is_some()
}

pub(crate) type RetroEnvironmentT =
    unsafe extern "C" fn(cmd: u32, data: *mut std::os::raw::c_void) -> bool;
pub(crate) type RetroVideoRefreshT =
    unsafe extern "C" fn(data: *const std::os::raw::c_void, width: u32, height: u32, pitch: usize);
pub(crate) type RetroInputPollT = unsafe extern "C" fn();
pub(crate) type RetroInputStateT =
    unsafe extern "C" fn(port: u32, device: u32, index: u32, id: u32) -> i16;
pub(crate) type RetroAudioSampleT = unsafe extern "C" fn(left: i16, right: i16);
pub(crate) type RetroAudioSampleBatchT =
    unsafe extern "C" fn(data: *const i16, frames: usize) -> usize;

#[repr(C)]
pub(crate) struct RetroVariable {
    pub(crate) key: *const std::os::raw::c_char,
    pub(crate) value: *const std::os::raw::c_char,
}

// ============================================================================
// Global state
// ============================================================================

pub(crate) static mut G_ENVIRON: Option<RetroEnvironmentT> = None;
pub(crate) static mut G_VIDEO: Option<RetroVideoRefreshT> = None;
pub(crate) static mut G_AUDIO_SAMPLE: Option<RetroAudioSampleT> = None;
pub(crate) static mut G_AUDIO_SAMPLE_BATCH: Option<RetroAudioSampleBatchT> = None;
pub(crate) static mut G_INPUT_POLL: Option<RetroInputPollT> = None;
pub(crate) static mut G_INPUT_STATE: Option<RetroInputStateT> = None;
pub(crate) static mut G_EMULATOR: Option<XboxEmulator> = None;
pub(crate) static G_VIDEO_XRGB8888: AtomicBool = AtomicBool::new(false);

static INPUT_JOYPAD_BITS: AtomicU32 = AtomicU32::new(0);
static INPUT_LX: AtomicI32 = AtomicI32::new(0);
static INPUT_LY: AtomicI32 = AtomicI32::new(0);
static INPUT_RX: AtomicI32 = AtomicI32::new(0);
static INPUT_RY: AtomicI32 = AtomicI32::new(0);
static INPUT_FRAME: AtomicU32 = AtomicU32::new(0);
static INPUT_LAST_BITS: AtomicU32 = AtomicU32::new(u32::MAX);
static INPUT_EDGE_LATCH_BITS: AtomicU32 = AtomicU32::new(0);
static INPUT_EDGE_LATCH_POLLS: AtomicU32 = AtomicU32::new(0);
static INPUT_TEST_LOGGED: AtomicBool = AtomicBool::new(false);
static INPUT_TEST_CONFIRM_CONSUMED: AtomicBool = AtomicBool::new(false);
static INPUT_NO_CB_LOGGED: AtomicBool = AtomicBool::new(false);
static INPUT_MAP_INJECT_LOGGED: AtomicBool = AtomicBool::new(false);
static SPIDEY_PETERSTU_READ_READY_SEQ: AtomicU32 = AtomicU32::new(0);
static SPIDEY_POST_PETERSTU_VSHADER_KEY_SEQ: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_CURRENT_FRAME: AtomicU64 = AtomicU64::new(0);
static SPIDEY_SYNTH_FIRST_HANDOFF_FRAME: AtomicU64 = AtomicU64::new(0);
static SPIDEY_SYNTH_HANDOFF_COUNT: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_HANDOFF_STATE_INITIAL: AtomicU32 = AtomicU32::new(u32::MAX);
static SPIDEY_SYNTH_TIMEOUT_LOGGED: AtomicBool = AtomicBool::new(false);
static SPIDEY_SYNTH_SUCCESS: AtomicBool = AtomicBool::new(false);
static SPIDEY_SYNTH_SKIN_STREAK: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_LAST_SKIN_FRAME: AtomicU64 = AtomicU64::new(0);
static SPIDEY_SYNTH_POST_HANDOFF_DRAWS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_POST_HANDOFF_SKIN_DRAWS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_POST_HANDOFF_D3D11_DRAWS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_SCENE_UPDATE_TICKS: AtomicU32 = AtomicU32::new(0);
static SPIDEY_SYNTH_UPDATE_HOOK_ENTRIES: AtomicU32 = AtomicU32::new(0);
static SPIDEY_ACTION_OBJECT_DUMP_LOG: AtomicU32 = AtomicU32::new(0);
static SPIDEY_RENDER_SCRATCH_DUMP_LOG: AtomicU32 = AtomicU32::new(0);

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn spidey_valid_guest_ptr(addr: u32) -> bool {
    (0x1000..0x2000_0000).contains(&addr)
}

fn spidey_log_action_object_dump(
    mem: &GuestMemory,
    frame: u64,
    scene_name: &str,
    scene_mgr: u32,
    scene_state: u32,
    action_mgr: u32,
    action_active: u32,
    action_slot0: u32,
    action_slot1: u32,
    object: u32,
) {
    if !spidey_valid_guest_ptr(object) {
        return;
    }

    let n = SPIDEY_ACTION_OBJECT_DUMP_LOG.fetch_add(1, Ordering::Relaxed);
    if n >= 8 && !n.is_power_of_two() {
        return;
    }

    use std::fmt::Write as _;

    let vtable = mem.read_u32(object);
    let mut vt_words = String::new();
    if spidey_valid_guest_ptr(vtable) {
        for i in 0..8u32 {
            let _ = write!(
                vt_words,
                "{}0x{:08X}",
                if i == 0 { "" } else { "," },
                mem.read_u32(vtable.wrapping_add(i * 4))
            );
        }
    }

    let mut rows = String::new();
    for row in 0..4u32 {
        let row_off = row * 0x40;
        let _ = write!(
            rows,
            "{}+{:02X}:",
            if row == 0 { "" } else { " | " },
            row_off
        );
        for col in 0..16u32 {
            let off = row_off + col * 4;
            let _ = write!(
                rows,
                "{}{:08X}",
                if col == 0 { "" } else { " " },
                mem.read_u32(object.wrapping_add(off))
            );
        }
    }

    let mut pointer_fields = String::new();
    let mut pointer_count = 0u32;
    for off in (0..0x100u32).step_by(4) {
        let val = mem.read_u32(object.wrapping_add(off));
        if !spidey_valid_guest_ptr(val) {
            continue;
        }
        let pointee_vt = mem.read_u32(val);
        let _ = write!(
            pointer_fields,
            "{}+{:02X}=0x{:08X}(vt=0x{:08X})",
            if pointer_count == 0 { "" } else { " " },
            off,
            val,
            pointee_vt
        );
        pointer_count += 1;
        if pointer_count >= 24 {
            break;
        }
    }

    let mut float3_fields = String::new();
    let mut float3_count = 0u32;
    for off in (0..=0xF4u32).step_by(4) {
        let x = f32::from_bits(mem.read_u32(object.wrapping_add(off)));
        let y = f32::from_bits(mem.read_u32(object.wrapping_add(off + 4)));
        let z = f32::from_bits(mem.read_u32(object.wrapping_add(off + 8)));
        let finite = x.is_finite() && y.is_finite() && z.is_finite();
        let moderate = x.abs() <= 10_000.0 && y.abs() <= 10_000.0 && z.abs() <= 10_000.0;
        let nonzero = x.abs() + y.abs() + z.abs() > 0.001;
        if finite && moderate && nonzero {
            let _ = write!(
                float3_fields,
                "{}+{:02X}=({:.3},{:.3},{:.3})",
                if float3_count == 0 { "" } else { " " },
                off,
                x,
                y,
                z
            );
            float3_count += 1;
            if float3_count >= 24 {
                break;
            }
        }
    }

    debug_log(&format!(
        "[SPIDEY-ACTION-OBJECT-DUMP] #{} frame={} scene='{}' scene_mgr=0x{:08X} state=0x{:08X} action_mgr=0x{:08X} active={} slot0=0x{:08X} slot1=0x{:08X} object=0x{:08X} vtable=0x{:08X} vt_words=[{}] ptr_fields=[{}] float3_fields=[{}] dwords=[{}]",
        n,
        frame,
        scene_name,
        scene_mgr,
        scene_state,
        action_mgr,
        action_active,
        action_slot0,
        action_slot1,
        object,
        vtable,
        vt_words,
        pointer_fields,
        float3_fields,
        rows
    ));
}

fn spidey_read_f32(mem: &GuestMemory, addr: u32) -> f32 {
    f32::from_bits(mem.read_u32(addr))
}

fn spidey_log_render_scratch_dump(
    mem: &GuestMemory,
    frame: u64,
    scene_name: &str,
    scene_mgr: u32,
    scene_state: u32,
    render_enable: u8,
    scratch_count_hint: u32,
) {
    if scratch_count_hint == 0 {
        return;
    }

    let n = SPIDEY_RENDER_SCRATCH_DUMP_LOG.fetch_add(1, Ordering::Relaxed);
    if n >= 8 && !n.is_power_of_two() {
        return;
    }

    use std::fmt::Write as _;

    const SCRATCH_BASE: u32 = 0x004D_1520;
    const SCRATCH_STRIDE: u32 = 0x68;
    let entry_count = scratch_count_hint.clamp(1, 4);

    let mut table = String::new();
    for i in 0..0x20u32 {
        let addr = 0x005F_2C00u32.wrapping_add(i * 4);
        let _ = write!(
            table,
            "{}+{:02X}=0x{:08X}",
            if i == 0 { "" } else { " " },
            i * 4,
            mem.read_u32(addr)
        );
    }

    let mut entries = String::new();
    for entry_idx in 0..entry_count {
        let base = SCRATCH_BASE.wrapping_add(entry_idx * SCRATCH_STRIDE);
        let mut ptr_fields = String::new();
        let mut ptr_count = 0u32;
        for off in (0..SCRATCH_STRIDE).step_by(4) {
            let val = mem.read_u32(base.wrapping_add(off));
            if !spidey_valid_guest_ptr(val) {
                continue;
            }
            let first = mem.read_u32(val);
            let _ = write!(
                ptr_fields,
                "{}+{:02X}=0x{:08X}(first=0x{:08X})",
                if ptr_count == 0 { "" } else { " " },
                off,
                val,
                first
            );
            ptr_count += 1;
            if ptr_count >= 8 {
                break;
            }
        }

        let mut dwords = String::new();
        for off in (0..SCRATCH_STRIDE).step_by(4) {
            let _ = write!(
                dwords,
                "{}{:08X}",
                if off == 0 { "" } else { " " },
                mem.read_u32(base.wrapping_add(off))
            );
        }

        let mut vecs = String::new();
        for off in [0x10u32, 0x20, 0x30, 0x40, 0x50] {
            let x = spidey_read_f32(mem, base.wrapping_add(off));
            let y = spidey_read_f32(mem, base.wrapping_add(off + 4));
            let z = spidey_read_f32(mem, base.wrapping_add(off + 8));
            let w = spidey_read_f32(mem, base.wrapping_add(off + 12));
            let _ = write!(
                vecs,
                "{}+{:02X}=[{:.4},{:.4},{:.4},{:.4}]",
                if vecs.is_empty() { "" } else { " " },
                off,
                x,
                y,
                z,
                w
            );
        }

        let _ = write!(
            entries,
            "{}entry{}@0x{:08X} ptrs=[{}] vecs=[{}] dwords=[{}]",
            if entry_idx == 0 { "" } else { " | " },
            entry_idx,
            base,
            ptr_fields,
            vecs,
            dwords
        );
    }

    debug_log(&format!(
        "[SPIDEY-RENDER-SCRATCH-DUMP] #{} frame={} scene='{}' scene_mgr=0x{:08X} state=0x{:08X} render_enable=0x{:02X} scratch_count_hint={} table=[{}] entries=[{}]",
        n,
        frame,
        scene_name,
        scene_mgr,
        scene_state,
        render_enable,
        scratch_count_hint,
        table,
        entries
    ));
}

pub(crate) fn spidey_synth_training_enabled() -> bool {
    use std::sync::OnceLock;
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| env_flag("RUSTEMU_SPIDEY_SYNTH_TRAINING"))
}

pub(crate) fn reset_input_snapshot() {
    INPUT_JOYPAD_BITS.store(0, Ordering::Relaxed);
    INPUT_LX.store(0, Ordering::Relaxed);
    INPUT_LY.store(0, Ordering::Relaxed);
    INPUT_RX.store(0, Ordering::Relaxed);
    INPUT_RY.store(0, Ordering::Relaxed);
    INPUT_FRAME.store(0, Ordering::Relaxed);
    INPUT_LAST_BITS.store(u32::MAX, Ordering::Relaxed);
    INPUT_EDGE_LATCH_BITS.store(0, Ordering::Relaxed);
    INPUT_EDGE_LATCH_POLLS.store(0, Ordering::Relaxed);
    INPUT_TEST_LOGGED.store(false, Ordering::Relaxed);
    INPUT_TEST_CONFIRM_CONSUMED.store(false, Ordering::Relaxed);
    INPUT_NO_CB_LOGGED.store(false, Ordering::Relaxed);
    INPUT_MAP_INJECT_LOGGED.store(false, Ordering::Relaxed);
    SPIDEY_PETERSTU_READ_READY_SEQ.store(0, Ordering::Relaxed);
    SPIDEY_POST_PETERSTU_VSHADER_KEY_SEQ.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_CURRENT_FRAME.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_HANDOFF_COUNT.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_HANDOFF_STATE_INITIAL.store(u32::MAX, Ordering::Relaxed);
    SPIDEY_SYNTH_TIMEOUT_LOGGED.store(false, Ordering::Relaxed);
    SPIDEY_SYNTH_SUCCESS.store(false, Ordering::Relaxed);
    SPIDEY_SYNTH_SKIN_STREAK.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_LAST_SKIN_FRAME.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_POST_HANDOFF_DRAWS.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_POST_HANDOFF_SKIN_DRAWS.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_POST_HANDOFF_D3D11_DRAWS.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_SCENE_UPDATE_TICKS.store(0, Ordering::Relaxed);
    SPIDEY_SYNTH_UPDATE_HOOK_ENTRIES.store(0, Ordering::Relaxed);
    SPIDEY_ACTION_OBJECT_DUMP_LOG.store(0, Ordering::Relaxed);
    SPIDEY_RENDER_SCRATCH_DUMP_LOG.store(0, Ordering::Relaxed);
}

pub(crate) fn mark_spidey_peterstu_read_ready() -> u32 {
    SPIDEY_PETERSTU_READ_READY_SEQ.fetch_add(1, Ordering::Relaxed) + 1
}

pub(crate) fn spidey_peterstu_read_ready_seq() -> u32 {
    SPIDEY_PETERSTU_READ_READY_SEQ.load(Ordering::Relaxed)
}

pub(crate) fn mark_spidey_post_peterstu_vshader_key_read(
    host_path: &str,
    bytes_read: usize,
) -> u32 {
    if !spidey_synth_training_enabled() || spidey_peterstu_read_ready_seq() == 0 {
        return SPIDEY_POST_PETERSTU_VSHADER_KEY_SEQ.load(Ordering::Relaxed);
    }
    let seq = SPIDEY_POST_PETERSTU_VSHADER_KEY_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    if seq <= 4 || seq.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-SYNTH-TRAINING-STAGE] post_peterstu_vshader_key_ready seq={} peterstu_ready_seq={} bytes={} path='{}'",
            seq,
            spidey_peterstu_read_ready_seq(),
            bytes_read,
            host_path
        ));
    }
    seq
}

pub(crate) fn spidey_post_peterstu_vshader_key_seq() -> u32 {
    SPIDEY_POST_PETERSTU_VSHADER_KEY_SEQ.load(Ordering::Relaxed)
}

pub(crate) fn note_spidey_synth_frame(frame: u64) {
    if !spidey_synth_training_enabled() {
        return;
    }
    SPIDEY_SYNTH_CURRENT_FRAME.store(frame, Ordering::Relaxed);
    if SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed) != 0
        && !SPIDEY_SYNTH_SUCCESS.load(Ordering::Relaxed)
    {
        SPIDEY_SYNTH_SCENE_UPDATE_TICKS.fetch_add(1, Ordering::Relaxed);
    }
}

pub(crate) fn spidey_synth_handoff_count() -> u32 {
    SPIDEY_SYNTH_HANDOFF_COUNT.load(Ordering::Relaxed)
}

pub(crate) fn spidey_synth_handoff_timed_out() -> bool {
    SPIDEY_SYNTH_TIMEOUT_LOGGED.load(Ordering::Relaxed)
}

pub(crate) fn record_spidey_synth_handoff(frame: u64, state: u32) -> u32 {
    let count = SPIDEY_SYNTH_HANDOFF_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let _ = SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.compare_exchange(
        0,
        frame,
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
    let _ = SPIDEY_SYNTH_HANDOFF_STATE_INITIAL.compare_exchange(
        u32::MAX,
        state,
        Ordering::Relaxed,
        Ordering::Relaxed,
    );
    count
}

pub(crate) fn note_spidey_synth_draw(
    draw_index: u64,
    shader_handle: u32,
    stride: u32,
    skinned_decl: bool,
    rt_key: u32,
    rt_width: u32,
    rt_height: u32,
) {
    if !spidey_synth_training_enabled() {
        return;
    }

    let first_handoff_frame = SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed);
    if first_handoff_frame != 0 {
        let draws = SPIDEY_SYNTH_POST_HANDOFF_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
        if draws <= 8 || draws.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-SYNTH-DRAW-AFTER-HANDOFF] #{} draw={} shader=0x{:08X} stride={} skinned_decl={} rt=0x{:08X} {}x{}",
                draws, draw_index, shader_handle, stride, skinned_decl, rt_key, rt_width, rt_height
            ));
        }
    } else {
        return;
    }

    let main_backbuffer = rt_key != 0xFFFF_FF00 && rt_width >= 600 && rt_height >= 400;
    let peter_draw = main_backbuffer && (skinned_decl || stride == 44);
    if !peter_draw {
        return;
    }

    if first_handoff_frame != 0 {
        let skin_draws = SPIDEY_SYNTH_POST_HANDOFF_SKIN_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
        if skin_draws <= 8 || skin_draws.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-SYNTH-SKIN-DRAW] #{} draw={} shader=0x{:08X} stride={} rt=0x{:08X} {}x{}",
                skin_draws, draw_index, shader_handle, stride, rt_key, rt_width, rt_height
            ));
        }
    }

    // "Frame" here means the libretro host frame, which is what paces the
    // guest-visible VBlank path. Do not count multiple pushbuffer flushes inside
    // one host frame as synthetic success.
    let frame = SPIDEY_SYNTH_CURRENT_FRAME.load(Ordering::Relaxed);
    if frame == 0 {
        return;
    }
    let last_frame = SPIDEY_SYNTH_LAST_SKIN_FRAME.swap(frame, Ordering::Relaxed);
    let streak = if last_frame == frame {
        SPIDEY_SYNTH_SKIN_STREAK.load(Ordering::Relaxed)
    } else if last_frame != 0 && frame == last_frame.saturating_add(1) {
        SPIDEY_SYNTH_SKIN_STREAK.fetch_add(1, Ordering::Relaxed) + 1
    } else {
        SPIDEY_SYNTH_SKIN_STREAK.store(1, Ordering::Relaxed);
        1
    };

    if streak >= 5 && !SPIDEY_SYNTH_SUCCESS.swap(true, Ordering::Relaxed) {
        debug_log(&format!(
            "[SPIDEY-SYNTH-INPUT-PASSTHROUGH] success=1 frame={} streak={} draw={} shader=0x{:08X} stride={} rt=0x{:08X} {}x{}",
            frame, streak, draw_index, shader_handle, stride, rt_key, rt_width, rt_height
        ));
    }
}

pub(crate) fn note_spidey_synth_d3d11_draw(
    draw_index: u64,
    active_vs: u32,
    rt_key: u32,
    rt_width: u32,
    rt_height: u32,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
) {
    if !spidey_synth_training_enabled()
        || SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed) == 0
        || SPIDEY_SYNTH_SUCCESS.load(Ordering::Relaxed)
    {
        return;
    }
    let draws = SPIDEY_SYNTH_POST_HANDOFF_D3D11_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
    if draws <= 8 || draws.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-SYNTH-D3D11-DRAW-AFTER-HANDOFF] #{} draw={} active_vs=0x{:08X} rt=0x{:08X} {}x{} bbox=[{:.1},{:.1}..{:.1},{:.1}]",
            draws,
            draw_index,
            active_vs,
            rt_key,
            rt_width,
            rt_height,
            min_x,
            min_y,
            max_x,
            max_y
        ));
    }
}

pub(crate) fn note_spidey_synth_update_hook(source: &str, scene_this: u32) {
    if !spidey_synth_training_enabled()
        || SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed) == 0
        || SPIDEY_SYNTH_SUCCESS.load(Ordering::Relaxed)
    {
        return;
    }
    let n = SPIDEY_SYNTH_UPDATE_HOOK_ENTRIES.fetch_add(1, Ordering::Relaxed) + 1;
    if n <= 8 || n.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-SYNTH-UPDATE-HOOK] #{} source={} scene_this=0x{:08X} frame={}",
            n,
            source,
            scene_this,
            SPIDEY_SYNTH_CURRENT_FRAME.load(Ordering::Relaxed)
        ));
    }
}

pub(crate) fn spidey_synth_training_input_passthrough() -> bool {
    SPIDEY_SYNTH_SUCCESS.load(Ordering::Relaxed)
}

pub(crate) fn spidey_synth_branch_snapshot() -> (u32, u32, u32, u32, u64, u32, u32, u32) {
    (
        SPIDEY_SYNTH_POST_HANDOFF_DRAWS.load(Ordering::Relaxed),
        SPIDEY_SYNTH_POST_HANDOFF_SKIN_DRAWS.load(Ordering::Relaxed),
        SPIDEY_SYNTH_SCENE_UPDATE_TICKS.load(Ordering::Relaxed),
        SPIDEY_SYNTH_HANDOFF_STATE_INITIAL.load(Ordering::Relaxed),
        SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed),
        SPIDEY_SYNTH_SKIN_STREAK.load(Ordering::Relaxed),
        SPIDEY_SYNTH_UPDATE_HOOK_ENTRIES.load(Ordering::Relaxed),
        SPIDEY_SYNTH_POST_HANDOFF_D3D11_DRAWS.load(Ordering::Relaxed),
    )
}

pub(crate) fn spidey_synth_timeout_due(current_frame: u64) -> bool {
    if !spidey_synth_training_enabled()
        || SPIDEY_SYNTH_SUCCESS.load(Ordering::Relaxed)
        || SPIDEY_SYNTH_TIMEOUT_LOGGED.load(Ordering::Relaxed)
    {
        return false;
    }
    let first = SPIDEY_SYNTH_FIRST_HANDOFF_FRAME.load(Ordering::Relaxed);
    first != 0 && current_frame.saturating_sub(first) >= 1800
}

pub(crate) fn mark_spidey_synth_timeout_logged() -> bool {
    !SPIDEY_SYNTH_TIMEOUT_LOGGED.swap(true, Ordering::Relaxed)
}

fn should_force_spidey_menu_complete(
    scene_name: &str,
    state: u32,
    render_enable: u8,
    scene_18c: u8,
    peterstu_ready_seq: u32,
) -> bool {
    scene_name.eq_ignore_ascii_case("bonus\\menu")
        && state == 3
        && render_enable != 0
        && scene_18c == 0
        && peterstu_ready_seq != 0
}

#[cfg(test)]
mod tests {
    use super::should_force_spidey_menu_complete;

    #[test]
    fn spidey_menu_complete_gate_fires_only_for_ready_state3_menu() {
        assert!(should_force_spidey_menu_complete("bonus\\menu", 3, 1, 0, 1));
        assert!(should_force_spidey_menu_complete(
            "BONUS\\MENU",
            3,
            1,
            0,
            42
        ));
    }

    #[test]
    fn spidey_menu_complete_gate_rejects_non_target_states() {
        let cases = [
            ("levels\\origin_z", 3, 1, 0, 1),
            ("bonus\\menu", 2, 1, 0, 1),
            ("bonus\\menu", 4, 1, 0, 1),
            ("bonus\\menu", 3, 0, 0, 1),
            ("bonus\\menu", 3, 1, 1, 1),
            ("bonus\\menu", 3, 1, 0, 0),
        ];

        for (scene_name, state, render_enable, scene_18c, peterstu_ready_seq) in cases {
            assert!(
                !should_force_spidey_menu_complete(
                    scene_name,
                    state,
                    render_enable,
                    scene_18c,
                    peterstu_ready_seq
                ),
                "unexpected force for scene={scene_name} state={state} render={render_enable} 18c={scene_18c} ready={peterstu_ready_seq}"
            );
        }
    }
}

pub(crate) fn mark_test_confirm_sampled() {
    if std::env::var_os("RUSTEMU_TEST_AUTOPRESS_CONFIRM").is_none() {
        return;
    }
    if !INPUT_TEST_CONFIRM_CONSUMED.swap(true, Ordering::Relaxed) {
        debug_log("[TEST-INPUT] confirm sampled by guest");
    }
}

pub(crate) fn poll_input_snapshot() {
    const RETRO_DEVICE_JOYPAD: u32 = 1;
    const RETRO_DEVICE_ANALOG: u32 = 5;
    const ANALOG_LEFT: u32 = 0;
    const ANALOG_RIGHT: u32 = 1;
    const ANALOG_X: u32 = 0;
    const ANALOG_Y: u32 = 1;

    let mut bits = 0u32;
    let mut lx = 0i32;
    let mut ly = 0i32;
    let mut rx = 0i32;
    let mut ry = 0i32;

    if let Some(cb) = unsafe { G_INPUT_STATE } {
        for id in 0..16u32 {
            if unsafe { cb(0, RETRO_DEVICE_JOYPAD, 0, id) } != 0 {
                bits |= 1u32 << id;
            }
        }

        lx = unsafe { cb(0, RETRO_DEVICE_ANALOG, ANALOG_LEFT, ANALOG_X) } as i32;
        ly = unsafe { cb(0, RETRO_DEVICE_ANALOG, ANALOG_LEFT, ANALOG_Y) } as i32;
        rx = unsafe { cb(0, RETRO_DEVICE_ANALOG, ANALOG_RIGHT, ANALOG_X) } as i32;
        ry = unsafe { cb(0, RETRO_DEVICE_ANALOG, ANALOG_RIGHT, ANALOG_Y) } as i32;
    } else if !INPUT_NO_CB_LOGGED.swap(true, Ordering::Relaxed) {
        debug_log("[INPUT-SNAPSHOT] no libretro input_state callback installed yet");
    }

    let frame = INPUT_FRAME.fetch_add(1, Ordering::Relaxed);
    let test_start = std::env::var_os("RUSTEMU_TEST_AUTOPRESS_START").is_some();
    let test_confirm = std::env::var_os("RUSTEMU_TEST_AUTOPRESS_CONFIRM").is_some();
    let test_new_game = std::env::var_os("RUSTEMU_SPIDEY_AUTONEWGAME").is_some()
        || std::env::var_os("RUSTEMU_TEST_AUTOPRESS_NEW_GAME").is_some();
    let test_mask_input = std::env::var_os("RUSTEMU_TEST_MASK_INPUT").is_some();
    let allow_host_input = std::env::var_os("RUSTEMU_TEST_ALLOW_HOST_INPUT").is_some();
    if (test_mask_input || test_start || test_confirm || test_new_game) && !allow_host_input {
        // Automated probes must be deterministic. RetroArch can report host
        // keyboard/gamepad state while the harness window has focus; mask that
        // out so only the scripted pulse reaches the guest.
        bits = 0;
        lx = 0;
        ly = 0;
        rx = 0;
        ry = 0;

        if test_mask_input
            && !test_start
            && !test_confirm
            && !test_new_game
            && !INPUT_TEST_LOGGED.swap(true, Ordering::Relaxed)
        {
            debug_log("[TEST-INPUT] RUSTEMU_TEST_MASK_INPUT active; masking host input");
        }

        // Spider-Man scripted input is now generated in XInputGetState, keyed
        // to guest input polls and scene names. Frontend-frame pulses are too
        // easy to miss or to leak into the next menu state.
        if (test_start || test_confirm || test_new_game)
            && !INPUT_TEST_LOGGED.swap(true, Ordering::Relaxed)
        {
            debug_log("[TEST-INPUT] scripted XInput mode active; masking host input snapshot");
        }
    } else if allow_host_input
        && (test_mask_input || test_start || test_confirm || test_new_game)
        && !INPUT_TEST_LOGGED.swap(true, Ordering::Relaxed)
    {
        debug_log("[TEST-INPUT] scripted XInput mode active; host input passthrough enabled");
    }

    if std::env::var_os("RUSTEMU_INPUT_INJECT_360_MAP").is_some() {
        if !INPUT_MAP_INJECT_LOGGED.swap(true, Ordering::Relaxed) {
            debug_log(
                "[TEST-INPUT] RUSTEMU_INPUT_INJECT_360_MAP active; cycling raw START/BACK/XboxA",
            );
        }
        bits = 0;
        lx = 0;
        ly = 0;
        rx = 0;
        ry = 0;
        match frame % 240 {
            30..=59 => bits |= 1u32 << 3,   // START
            90..=119 => bits |= 1u32 << 2,  // BACK
            150..=179 => bits |= 1u32 << 0, // Xbox A via libretro south button
            _ => {}
        }
    }

    // Libretro reports analog Y in screen coordinates (positive = down).
    // Xbox XInput thumbstick Y is Cartesian (positive = up).
    let invert_thumb_y = |v: i32| -> i32 { (-v).clamp(i16::MIN as i32, i16::MAX as i32) };
    ly = invert_thumb_y(ly);
    ry = invert_thumb_y(ry);

    let prev = INPUT_LAST_BITS.swap(bits, Ordering::Relaxed);
    if prev != u32::MAX {
        // RetroArch input is sampled on the frontend thread, but Xbox titles
        // consume XInput from guest worker threads. A very normal tap can land
        // entirely between two guest polls, so hold real confirm-button edges
        // briefly in guest-poll time. D-pad movement is deliberately excluded.
        const EDGE_LATCH_MASK: u32 = (1u32 << 0)  // B
            | (1u32 << 2)                         // BACK / Select
            | (1u32 << 3)                         // START
            | (1u32 << 8); // A
        let rising = (bits & !prev) & EDGE_LATCH_MASK;
        if rising != 0 {
            INPUT_EDGE_LATCH_BITS.fetch_or(rising, Ordering::Relaxed);
            INPUT_EDGE_LATCH_POLLS.store(8, Ordering::Relaxed);
            debug_log(&format!(
                "[INPUT-LATCH] rising=0x{:04X} raw=0x{:04X} hold_polls=8",
                rising, bits
            ));
        }
    }
    if prev != bits && (bits != 0 || prev != u32::MAX) {
        debug_log(&format!(
            "[INPUT-SNAPSHOT] frame={} joypad_bits=0x{:04X}",
            frame, bits
        ));
    }

    INPUT_JOYPAD_BITS.store(bits, Ordering::Relaxed);
    INPUT_LX.store(lx, Ordering::Relaxed);
    INPUT_LY.store(ly, Ordering::Relaxed);
    INPUT_RX.store(rx, Ordering::Relaxed);
    INPUT_RY.store(ry, Ordering::Relaxed);
}

pub(crate) fn input_snapshot() -> (u32, i16, i16, i16, i16) {
    (
        INPUT_JOYPAD_BITS.load(Ordering::Relaxed),
        INPUT_LX.load(Ordering::Relaxed) as i16,
        INPUT_LY.load(Ordering::Relaxed) as i16,
        INPUT_RX.load(Ordering::Relaxed) as i16,
        INPUT_RY.load(Ordering::Relaxed) as i16,
    )
}

pub(crate) fn input_latched_edge_bits() -> u32 {
    let bits = INPUT_EDGE_LATCH_BITS.load(Ordering::Relaxed);
    if bits == 0 {
        return 0;
    }

    let polls = INPUT_EDGE_LATCH_POLLS.load(Ordering::Relaxed);
    if polls == 0 {
        INPUT_EDGE_LATCH_BITS.store(0, Ordering::Relaxed);
        return 0;
    }

    if INPUT_EDGE_LATCH_POLLS.fetch_sub(1, Ordering::Relaxed) <= 1 {
        INPUT_EDGE_LATCH_BITS.store(0, Ordering::Relaxed);
    }
    bits
}

// Static strings for system info (must outlive the retro_get_system_info call)
/// Worker thread native ID for RIP sampling from watchdog.
pub(crate) static WORKER_THREAD_ID: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Worker RIP sample histogram — keyed by 64-byte guest PC bucket.
/// Populated by sample_worker_rip(); logged at shutdown or every 1000 samples.
static WORKER_RIP_HIST: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<u32, u64>>> =
    std::sync::OnceLock::new();
static WORKER_RIP_SAMPLES: AtomicU64 = AtomicU64::new(0);
static WORKER_RIP_SPIDEY_SCRIPT_EXACT_LOGS: AtomicU32 = AtomicU32::new(0);
static WORKER_RIP_SPIDEY_XBS_SCHED_LOGS: AtomicU32 = AtomicU32::new(0);

fn spidey_xbs_scheduler_label(guest_pc: u32) -> Option<&'static str> {
    match guest_pc {
        0x0004_BDF0..=0x0004_BE6F => Some("queue_run"),
        0x0004_BE70..=0x0004_BF1F => Some("queue_run_one"),
        0x0004_D640..=0x0004_D6CF => Some("queue_enqueue"),
        0x0006_0990..=0x0006_09CF => Some("global_queue_run"),
        0x0006_0A00..=0x0006_0A4F => Some("global_queue_enqueue"),
        0x0010_9BF0..=0x0010_9C8F => Some("preload"),
        0x0010_9CB0..=0x0010_9D3F => Some("preload_sibling"),
        0x0012_4D90..=0x0012_4E7F => Some("auto_preload"),
        0x0012_70A0..=0x0012_71AF => Some("script_enqueue_run"),
        0x0013_95F0..=0x0013_96CF => Some("script_enqueue"),
        0x0016_30C0..=0x0016_31AF => Some("0x001630c0_SCRIPT_ENQUEUE"),
        _ => None,
    }
}

/// Sample the worker thread's current RIP, map to guest PC via addr_hash,
/// bump a histogram bucket. Called every ~10 frames from run(). Uses
/// SuspendThread/GetThreadContext/ResumeThread for a host-side snapshot.
/// Safe: brief (<100us) suspend while reading registers; worker resumed
/// immediately. Histogram buckets are 64-byte aligned (mask low 6 bits) to
/// collect samples from the same basic block together.
#[cfg(windows)]
fn sample_worker_rip(
    code_base: u64,
    code_size: u32,
    addr_hash: &crate::xbox::aot::runtime::AddrHash,
) {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::Debug::{
        GetThreadContext, CONTEXT, CONTEXT_CONTROL_AMD64, CONTEXT_INTEGER_AMD64,
    };
    use windows::Win32::System::Threading::{
        OpenThread, ResumeThread, SuspendThread, THREAD_GET_CONTEXT, THREAD_SUSPEND_RESUME,
    };

    let tid = WORKER_THREAD_ID.load(Ordering::Relaxed);
    if tid == 0 {
        return;
    }

    unsafe {
        let h = match OpenThread(THREAD_GET_CONTEXT | THREAD_SUSPEND_RESUME, false, tid) {
            Ok(h) => h,
            Err(_) => return,
        };

        // Brief suspend to freeze the RIP we're about to read.
        let prev = SuspendThread(h);
        if prev == u32::MAX {
            let _ = CloseHandle(h);
            return;
        }

        let mut ctx: CONTEXT = std::mem::zeroed();
        ctx.ContextFlags = CONTEXT_CONTROL_AMD64 | CONTEXT_INTEGER_AMD64;
        let ok = GetThreadContext(h, &mut ctx).is_ok();
        let rip = if ok { ctx.Rip } else { 0 };

        // Always resume.
        let _ = ResumeThread(h);
        let _ = CloseHandle(h);

        if rip == 0 {
            return;
        }

        // Categorize: in code buffer, in addr_hash (JIT-compiled), or elsewhere?
        let bucket: u32 = if rip >= code_base && rip < code_base + code_size as u64 {
            // Host offset into our code buffer; reverse-lookup to guest PC.
            let host_off = (rip - code_base) as u32;
            let guest_pc = addr_hash.reverse_lookup(host_off);
            if std::env::var_os("RUSTEMU_SPIDEY_XBS_SCRIPT_PROBE").is_some()
                && (0x0005_1C00..0x0005_2C00).contains(&guest_pc)
            {
                let n = WORKER_RIP_SPIDEY_SCRIPT_EXACT_LOGS.fetch_add(1, Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() {
                    let script = ctx.Rbp as u32;
                    let (flags8, stack14, base18, pc1c, word, major, op) =
                        if ctx.R15 != 0 && script < 0xFD00_0000 {
                            unsafe {
                                let read_u8 =
                                    |addr: u32| -> u8 { *((ctx.R15 + addr as u64) as *const u8) };
                                let read_u16 = |addr: u32| -> u16 {
                                    u16::from_le(std::ptr::read_unaligned(
                                        (ctx.R15 + addr as u64) as *const u16,
                                    ))
                                };
                                let read_u32 = |addr: u32| -> u32 {
                                    u32::from_le(std::ptr::read_unaligned(
                                        (ctx.R15 + addr as u64) as *const u32,
                                    ))
                                };
                                let pc = read_u32(script.wrapping_add(0x1C));
                                let word = if pc < 0xFD00_0000 { read_u16(pc) } else { 0 };
                                (
                                    read_u8(script.wrapping_add(0x08)),
                                    read_u32(script.wrapping_add(0x14)),
                                    read_u32(script.wrapping_add(0x18)),
                                    pc,
                                    word,
                                    word >> 8,
                                    word & 0x007F,
                                )
                            }
                        } else {
                            (0, 0, 0, 0, 0, 0, 0)
                        };
                    debug_log(&format!(
                        "[SPIDEY-SCRIPT-RIP-EXACT] #{} guest_pc=0x{:08X} host_off=0x{:08X} rip=0x{:016X} script=0x{:08X} flags8=0x{:02X} stack14=0x{:08X} base18=0x{:08X} pc1c=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} rsp=0x{:016X} r14=0x{:08X} r15=0x{:016X}",
                        n + 1,
                        guest_pc,
                        host_off,
                        rip,
                        script,
                        flags8,
                        stack14,
                        base18,
                        pc1c,
                        word,
                        major,
                        op,
                        ctx.Rsp,
                        ctx.R14 as u32,
                        ctx.R15
                    ));
                }
            }
            if std::env::var_os("RUSTEMU_SPIDEY_XBS_SCHED_PROBE").is_some() {
                if let Some(label) = spidey_xbs_scheduler_label(guest_pc) {
                    let n = WORKER_RIP_SPIDEY_XBS_SCHED_LOGS.fetch_add(1, Ordering::Relaxed);
                    if n < 96 || n.is_power_of_two() {
                        let (
                            global_script_mgr,
                            global_active_script,
                            xroot,
                            xroot_script_mgr,
                            xroot_script_table,
                            global_scene,
                            frame_ctx,
                            live_scene,
                            ecx_10,
                            ecx_14,
                            ecx_20,
                            ecx_head_next,
                            ecx_head_prev,
                            ebp_script_pc,
                            ebp_script_word,
                            ebp_script_major,
                            stack0,
                            stack4,
                            stack8,
                            stackc,
                            stack10,
                        ) = if ctx.R15 != 0 {
                            unsafe {
                                let read_u32 = |addr: u32| -> u32 {
                                    u32::from_le(std::ptr::read_unaligned(
                                        (ctx.R15 + addr as u64) as *const u32,
                                    ))
                                };
                                let read_u16 = |addr: u32| -> u16 {
                                    u16::from_le(std::ptr::read_unaligned(
                                        (ctx.R15 + addr as u64) as *const u16,
                                    ))
                                };
                                let valid_ptr = |addr: u32| -> bool {
                                    (0x0001_0000..0x2000_0000).contains(&addr)
                                };
                                let read_ptr_u32 = |addr: u32| -> u32 {
                                    if valid_ptr(addr) {
                                        read_u32(addr)
                                    } else {
                                        0
                                    }
                                };

                                let xroot = read_u32(0x004C_06B8);
                                let xroot_script_mgr = read_ptr_u32(xroot.wrapping_add(0x1A0));
                                let frame_ctx = read_u32(0x004B_C630);
                                let live_scene = read_ptr_u32(frame_ctx.wrapping_add(0x18));
                                let ecx = ctx.Rcx as u32;
                                let ebp = ctx.Rbp as u32;
                                let esp = ctx.R14 as u32;
                                let ecx_10 = read_ptr_u32(ecx.wrapping_add(0x10));
                                let ecx_head_next = read_ptr_u32(ecx_10);
                                let ecx_head_prev = read_ptr_u32(ecx_10.wrapping_add(4));
                                let ebp_script_pc = read_ptr_u32(ebp.wrapping_add(0x1C));
                                let ebp_script_word = if valid_ptr(ebp_script_pc) {
                                    read_u16(ebp_script_pc) as u32
                                } else {
                                    0
                                };

                                (
                                    read_u32(0x004B_8F04),
                                    read_u32(0x004B_8F0C),
                                    xroot,
                                    xroot_script_mgr,
                                    read_ptr_u32(xroot_script_mgr.wrapping_add(0x20)),
                                    read_u32(0x003F_5BEC),
                                    frame_ctx,
                                    live_scene,
                                    ecx_10,
                                    read_ptr_u32(ecx.wrapping_add(0x14)),
                                    read_ptr_u32(ecx.wrapping_add(0x20)),
                                    ecx_head_next,
                                    ecx_head_prev,
                                    ebp_script_pc,
                                    ebp_script_word,
                                    ebp_script_word >> 8,
                                    read_ptr_u32(esp),
                                    read_ptr_u32(esp.wrapping_add(4)),
                                    read_ptr_u32(esp.wrapping_add(8)),
                                    read_ptr_u32(esp.wrapping_add(0x0C)),
                                    read_ptr_u32(esp.wrapping_add(0x10)),
                                )
                            }
                        } else {
                            (
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            )
                        };
                        debug_log(&format!(
                            "[SPIDEY-XBS-SCHED-RIP] #{} {} guest_pc=0x{:08X} host_off=0x{:08X} rip=0x{:016X} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X} esp=0x{:08X} g_script_mgr=0x{:08X} g_active_script=0x{:08X} xroot=0x{:08X} xroot_script_mgr=0x{:08X} script_table=0x{:08X} global_scene=0x{:08X} frame_ctx=0x{:08X} live_scene=0x{:08X} ecx+10=0x{:08X} ecx+14=0x{:08X} ecx+20=0x{:08X} head_next=0x{:08X} head_prev=0x{:08X} ebp_pc=0x{:08X} ebp_word=0x{:04X} ebp_major=0x{:02X} stack=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}",
                            n + 1,
                            label,
                            guest_pc,
                            host_off,
                            rip,
                            ctx.Rax as u32,
                            ctx.Rbx as u32,
                            ctx.Rcx as u32,
                            ctx.Rdx as u32,
                            ctx.Rsi as u32,
                            ctx.Rdi as u32,
                            ctx.Rbp as u32,
                            ctx.R14 as u32,
                            global_script_mgr,
                            global_active_script,
                            xroot,
                            xroot_script_mgr,
                            xroot_script_table,
                            global_scene,
                            frame_ctx,
                            live_scene,
                            ecx_10,
                            ecx_14,
                            ecx_20,
                            ecx_head_next,
                            ecx_head_prev,
                            ebp_script_pc,
                            ebp_script_word,
                            ebp_script_major,
                            stack0,
                            stack4,
                            stack8,
                            stackc,
                            stack10
                        ));
                    }
                }
            }
            guest_pc & !0x3F // 64-byte basic-block bucket
        } else {
            // Not in our JIT buffer — worker is in stdlib, ntdll, VEH, etc.
            // Tag with sentinel so it shows up distinctly in the histogram.
            0xFFFF_0000
        };

        let hist = WORKER_RIP_HIST.get_or_init(|| std::sync::Mutex::new(Default::default()));
        if let Ok(mut h) = hist.lock() {
            *h.entry(bucket).or_insert(0) += 1;
        }
        WORKER_RIP_SAMPLES.fetch_add(1, Ordering::Relaxed);
    }
}

/// Dump the top-N RIP histogram buckets to debug.log. Called from run()
/// every 1000 samples. Shows where the worker spends its time.
fn dump_worker_rip_histogram() {
    let hist = match WORKER_RIP_HIST.get() {
        Some(h) => h,
        None => return,
    };
    let snapshot: Vec<(u32, u64)> = {
        let h = hist.lock().unwrap_or_else(|e| e.into_inner());
        h.iter().map(|(&k, &v)| (k, v)).collect()
    };
    let total: u64 = snapshot.iter().map(|(_, v)| v).sum();
    if total == 0 {
        return;
    }

    let mut sorted = snapshot;
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    debug_log(&format!(
        "[RIP-HIST] === {} samples, {} unique buckets, top 15 ===",
        total,
        sorted.len()
    ));
    for (bucket, count) in sorted.iter().take(15) {
        let pct = (*count as f64 / total as f64) * 100.0;
        let label = if *bucket == 0xFFFF_0000 {
            "(non-JIT: stdlib/ntdll/VEH)".to_string()
        } else {
            format!(
                "guest_pc~={}",
                crate::xbox::logging::fmt_guest_addr(*bucket)
            )
        };
        debug_log(&format!(
            "[RIP-HIST]   {:>6} ({:5.1}%)  {}",
            count, pct, label
        ));
    }
}

/// VBlank event — auto-reset Win32 event. Main thread (retro_run) signals it each frame.
/// KeDelayExecutionThread waits on it instead of sleeping, matching Xbox ISR behavior.
pub static VBLANK_EVENT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Worker completion event — signaled when the worker thread's organic init phase finishes.
/// The main thread's NtWaitForSingleObject blocks on this instead of returning immediately.
/// This ensures the main thread continues into game_main AFTER the worker sets up CRT globals.
pub static WORKER_DONE_EVENT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(crate) static LIB_NAME: &[u8] = b"Rustemu Xbox\0";
pub(crate) static LIB_VERSION: &[u8] = b"0.1.0\0";
pub(crate) static VALID_EXTENSIONS: &[u8] = b"xbe|iso\0";

// ============================================================================
// XboxEmulator
// ============================================================================

pub struct XboxEmulator {
    memory: Option<GuestMemory>,
    xbe_info: Option<XbeInfo>,
    display: StageDisplay,
    frame_count: u64,
    timing_start: std::time::Instant,
    aot_ctx: Option<Box<RuntimeContext>>,
    code_buffer: Option<CodeBuffer>,
    kernel_state: Option<kernel::KernelState>,
    worker_handles: Vec<std::thread::JoinHandle<WorkerStats>>,
    worker_total_dispatches: u64,
    worker_total_mmio: u64,
    worker_total_pb: u64,
    worker_total_draw_calls: u64,
    worker_live_counters: Option<Arc<SharedWorkerCounters>>,
    /// VBlank signal: main thread signals worker each frame (60Hz ISR injection).
    vblank_signal: Option<Arc<VBlankSignal>>,
    /// Stuck detection for DPC injection (C++ v1 STUCK_THRESHOLD=12)
    stuck_last_dispatch_count: u64,
    stuck_last_kernel_calls: u64,
    stuck_vblank_count: u32,
    /// Legacy conversion buffer retained for old diagnostic paths.
    video_buf_1555: Vec<u16>,
    /// One-shot Spider-Man fallback: publish a minimal frame context if the
    /// organic init reaches App/Scene/Engine but never fills 0x004BC630.
    spiderman_framectx_injected: bool,
    /// Last observed state of the diagnostic dump core option. The option is
    /// edge-triggered so leaving it enabled does not dump 64MB every frame.
    diag_dump_option_enabled: bool,
    /// Prevents a persisted RetroArch core option from dumping during the
    /// first boot frame. A dump should require a deliberate OFF -> ON edge.
    diag_dump_option_initialized: bool,
    diag_dump_count: u32,
}

/// VBlank signal: main thread notifies worker to run ISR each frame.
pub(crate) struct VBlankSignal {
    mutex: std::sync::Mutex<bool>,
    condvar: std::sync::Condvar,
}

impl VBlankSignal {
    pub(crate) fn new() -> Self {
        Self {
            mutex: std::sync::Mutex::new(false),
            condvar: std::sync::Condvar::new(),
        }
    }

    /// Main thread: signal one VBlank tick.
    pub(crate) fn signal(&self) {
        let mut ready = self.mutex.lock().unwrap();
        *ready = true;
        self.condvar.notify_one();
    }

    /// Worker thread: wait for VBlank tick (blocks until signaled).
    pub(crate) fn wait(&self) {
        let mut ready = self.mutex.lock().unwrap();
        while !*ready {
            ready = self.condvar.wait(ready).unwrap();
        }
        *ready = false;
    }

    /// Worker thread: wait with timeout (returns true if signaled).
    pub(crate) fn wait_timeout(&self, dur: std::time::Duration) -> bool {
        let mut ready = self.mutex.lock().unwrap();
        if *ready {
            *ready = false;
            return true;
        }
        let (mut ready, result) = self.condvar.wait_timeout(ready, dur).unwrap();
        if *ready {
            *ready = false;
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub(crate) struct WorkerStats {
    pub(crate) dispatch_count: u64,
    pub(crate) mmio_count: u64,
    pub(crate) pb_commands: u64,
    pub(crate) draw_calls: u64,
    pub(crate) sign_ext_fixups: u64,
}

/// Shared atomic counters for live worker stat visibility.
/// Worker increments these as it runs; main thread reads them each frame.
pub(crate) struct SharedWorkerCounters {
    pub(crate) dispatch_count: AtomicU64,
    pub(crate) mmio_count: AtomicU64,
    pub(crate) pb_commands: AtomicU64,
    pub(crate) draw_calls: AtomicU64,
    pub(crate) kernel_calls: AtomicU64,
    pub(crate) alive: AtomicBool,
}

// ============================================================================
// Fake KPCR/KTHREAD initialization (matches C++ FakeKPCR.h)
// ============================================================================

// Backing structure addresses (within committed 512MB RAM at 0x0C000000)
const FAKE_KPCR_BASE: u32 = 0x0C00_0000;
const FAKE_KPCR_OFFSET: u32 = 0x0000;
const FAKE_KPRCB_OFFSET: u32 = 0x0300;
const FAKE_KTHREAD_OFFSET: u32 = 0x0600;
const FAKE_NT_TIB_OFFSET: u32 = 0x0A00;
const FAKE_GDT_OFFSET: u32 = 0x0A40;
const FAKE_IDT_OFFSET: u32 = 0x0A70;
const FAKE_TLS_OFFSET: u32 = 0x0B00;

fn init_fake_kpcr(memory: &GuestMemory) {
    let kpcr_addr = FAKE_KPCR_BASE + FAKE_KPCR_OFFSET;
    let kprcb_addr = FAKE_KPCR_BASE + FAKE_KPRCB_OFFSET;
    let kthread_addr = FAKE_KPCR_BASE + FAKE_KTHREAD_OFFSET;
    let nt_tib_addr = FAKE_KPCR_BASE + FAKE_NT_TIB_OFFSET;
    let gdt_addr = FAKE_KPCR_BASE + FAKE_GDT_OFFSET;
    let idt_addr = FAKE_KPCR_BASE + FAKE_IDT_OFFSET;
    let tls_addr = FAKE_KPCR_BASE + FAKE_TLS_OFFSET;

    // Default stack bounds (main thread)
    let stack_base: u32 = 0x00B0_0000;
    let stack_limit: u32 = 0x00AF_0000;

    // Zero the 4KB backing block at 0x0C000000
    memory.zero_fill(FAKE_KPCR_BASE, 0x1000);

    // --- KTHREAD at 0x0C000600 ---
    memory.write_u8(kthread_addr + 0x00, 6); // Type = ThreadObject
    memory.write_u8(kthread_addr + 0x02, 0x80); // Size (in u32s, ~0x80)
    memory.write_u32(kthread_addr + 0x1C, stack_base); // StackBase
    memory.write_u32(kthread_addr + 0x20, stack_limit); // StackLimit
    memory.write_u32(kthread_addr + 0x24, stack_base); // KernelStack = top
    memory.write_u32(kthread_addr + 0x28, tls_addr); // TlsData
    memory.write_u8(kthread_addr + 0x2C, 2); // State = Running
    memory.write_u8(kthread_addr + 0x32, 8); // Priority = 8 (Normal)
    memory.write_u8(kthread_addr + 0x5C, 8); // BasePriority = 8
    memory.write_u8(kthread_addr + 0x5F, 6); // Quantum = 6
                                             // APC list heads: empty circular lists (Flink=Blink=self)
    let apc_kernel = kthread_addr + 0x34;
    memory.write_u32(apc_kernel, apc_kernel); // Flink
    memory.write_u32(apc_kernel + 4, apc_kernel); // Blink
    let apc_user = apc_kernel + 8;
    memory.write_u32(apc_user, apc_user); // Flink
    memory.write_u32(apc_user + 4, apc_user); // Blink
                                              // Thread list entry: circular (just this thread)
    let thread_list = kthread_addr + 0x12C;
    memory.write_u32(thread_list, thread_list); // Flink
    memory.write_u32(thread_list + 4, thread_list); // Blink

    // --- KPRCB at 0x0C000300 ---
    memory.write_u32(kprcb_addr + 0x00, kthread_addr); // CurrentThread
    memory.write_u32(kprcb_addr + 0x04, 0); // NextThread = 0
    memory.write_u32(kprcb_addr + 0x08, kthread_addr); // IdleThread = same
    memory.write_u8(kprcb_addr + 0x0C, 0); // Processor 0
                                           // DPC list head: empty circular
    let dpc_list = kprcb_addr + 0x20;
    memory.write_u32(dpc_list, dpc_list);
    memory.write_u32(dpc_list + 4, dpc_list);

    // --- Standalone NT_TIB at 0x0C000A00 ---
    memory.write_u32(nt_tib_addr + 0x00, 0xFFFF_FFFF); // ExceptionList = end
    memory.write_u32(nt_tib_addr + 0x04, stack_base); // StackBase
    memory.write_u32(nt_tib_addr + 0x08, stack_limit); // StackLimit
    memory.write_u32(nt_tib_addr + 0x10, 0x0000_1E00); // FiberData (not a fiber)
    memory.write_u32(nt_tib_addr + 0x18, nt_tib_addr); // Self

    // --- KPCR at 0x0C000000 (backing copy) ---
    // Embedded NT_TIB at +0x00
    memory.write_u32(kpcr_addr + 0x00, 0xFFFF_FFFF); // ExceptionList
    memory.write_u32(kpcr_addr + 0x04, stack_base); // StackBase
    memory.write_u32(kpcr_addr + 0x08, stack_limit); // StackLimit
    memory.write_u32(kpcr_addr + 0x10, 0x0000_1E00); // FiberData
    memory.write_u32(kpcr_addr + 0x18, kpcr_addr); // TIB.Self = KPCR base
    memory.write_u32(kpcr_addr + 0x1C, kpcr_addr); // SelfPcr
    memory.write_u32(kpcr_addr + 0x20, kprcb_addr); // Prcb
    memory.write_u8(kpcr_addr + 0x24, 0); // Irql = PASSIVE_LEVEL
    memory.write_u32(kpcr_addr + 0x28, kthread_addr); // PrcbData.CurrentThread
    memory.write_u32(kpcr_addr + 0x2C, 0); // PrcbData.NextThread
    memory.write_u32(kpcr_addr + 0x30, kthread_addr); // PrcbData.IdleThread
    memory.write_u32(kpcr_addr + 0x38, idt_addr); // IDT pointer
    memory.write_u32(kpcr_addr + 0x3C, gdt_addr); // GDT pointer
    memory.write_u16(kpcr_addr + 0x44, 1); // MajorVersion
    memory.write_u32(kpcr_addr + 0x48, 733); // StallScaleFactor

    // Zero page mirror is set up separately by init_fake_kpcr_zero_page() after
    // the AOT pre-flight zeroing pass, which wipes address 0x0000-0x0FFF.
    // FS: reads are redirected to 0x0C000000 by the emitter, but raw NULL pointer
    // dereferences (e.g. empty hash table bucket chains) hit address 0 directly.

    debug_log(&format!(
        "KPCR at 0x{:08X}: CurrentThread=0x{:08X}, TlsData=0x{:08X}",
        kpcr_addr, kthread_addr, tls_addr
    ));
}

/// Mirror critical KPCR fields to the zero page (address 0x0000).
/// On real Xbox, address 0 IS the KPCR — games that dereference null pointers
/// or follow empty linked lists read real KPCR data. Without this, all-zero
/// bytes at address 0 cause infinite loops in strncmp-based hash tables.
fn init_fake_kpcr_zero_page(memory: &GuestMemory) {
    let kpcr_addr = FAKE_KPCR_BASE + FAKE_KPCR_OFFSET; // 0x0C000000
    let kthread_addr = FAKE_KPCR_BASE + FAKE_KTHREAD_OFFSET;
    let kprcb_addr = FAKE_KPCR_BASE + FAKE_KPRCB_OFFSET;
    let idt_addr = FAKE_KPCR_BASE + FAKE_IDT_OFFSET;
    let gdt_addr = FAKE_KPCR_BASE + FAKE_GDT_OFFSET;
    let stack_base: u32 = 0x00B0_0000;
    let stack_limit: u32 = 0x00AF_0000;

    // Fill first 24 bytes matching real Xbox KPCR/TIB layout at address 0.
    // On real Xbox, address 0 IS the KPCR. Games that dereference NULL pointers
    // or follow empty linked lists read real KPCR data.
    // ExceptionList=0xFFFFFFFF acts as "infinity key" for skip list comparisons.
    memory.write_u32(0x00, 0xFFFF_FFFF); // NT_TIB.ExceptionList (end of SEH chain)
    memory.write_u32(0x04, stack_base); // NT_TIB.StackBase
    memory.write_u32(0x08, stack_limit); // NT_TIB.StackLimit
    memory.write_u32(0x0C, 0); // NT_TIB.SubSystemTib
    memory.write_u32(0x10, 0); // NT_TIB.FiberData (0 = not a fiber)
    memory.write_u32(0x14, 0); // NT_TIB.ArbitraryUserPointer
    memory.write_u32(0x18, kpcr_addr); // TIB.Self = KPCR
    memory.write_u32(0x1C, kpcr_addr); // SelfPcr
    memory.write_u32(0x20, kprcb_addr); // Prcb
    memory.write_u32(0x24, 0); // Irql = PASSIVE_LEVEL
    memory.write_u32(0x28, kthread_addr); // PrcbData.CurrentThread
    memory.write_u32(0x2C, 0); // PrcbData.NextThread
    memory.write_u32(0x30, kthread_addr); // PrcbData.IdleThread
    memory.write_u32(0x38, idt_addr); // IDT
    memory.write_u32(0x3C, gdt_addr); // GDT
}

/// Executable code buffer backed by VirtualAlloc.
struct CodeBuffer {
    ptr: *mut u8,
    len: usize,
}

#[cfg(windows)]
impl CodeBuffer {
    fn new(code: &[u8]) -> Option<Self> {
        use windows::Win32::Foundation::*;
        use windows::Win32::System::Memory::*;

        // Over-allocate: reserve extra 1MB for rescue pool (on-demand JIT)
        let rescue_reserve = 1024 * 1024usize;
        let total_alloc = code.len() + rescue_reserve;

        unsafe {
            // Allocate RW (full size including rescue reserve)
            let ptr = VirtualAlloc(None, total_alloc, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
            if ptr.is_null() {
                return None;
            }
            // Copy main code
            std::ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u8, code.len());
            // Flip main code to RX (rescue pool stays RW for now)
            let mut old_protect = PAGE_PROTECTION_FLAGS(0);
            let _ = VirtualProtect(ptr, code.len(), PAGE_EXECUTE_READ, &mut old_protect);
            Some(CodeBuffer {
                ptr: ptr as *mut u8,
                len: total_alloc,
            })
        }
    }

    fn base(&self) -> *mut u8 {
        self.ptr
    }
}

#[cfg(windows)]
impl Drop for CodeBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            use windows::Win32::System::Memory::*;
            unsafe {
                let _ = VirtualFree(self.ptr as _, 0, MEM_RELEASE);
            }
        }
    }
}

#[cfg(not(windows))]
impl CodeBuffer {
    fn new(_code: &[u8]) -> Option<Self> {
        None
    }
    fn base(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
}

impl XboxEmulator {
    pub fn new() -> Self {
        Self {
            memory: None,
            xbe_info: None,
            display: StageDisplay::new(),
            frame_count: 0,
            timing_start: std::time::Instant::now(),
            aot_ctx: None,
            code_buffer: None,
            kernel_state: None,
            worker_handles: Vec::new(),
            worker_total_dispatches: 0,
            worker_total_mmio: 0,
            worker_total_pb: 0,
            worker_total_draw_calls: 0,
            worker_live_counters: None,
            vblank_signal: None,
            stuck_last_dispatch_count: 0,
            stuck_last_kernel_calls: 0,
            stuck_vblank_count: 0,
            video_buf_1555: vec![0u16; FB_WIDTH * FB_HEIGHT],
            spiderman_framectx_injected: false,
            diag_dump_option_enabled: false,
            diag_dump_option_initialized: false,
            diag_dump_count: 0,
        }
    }

    pub(crate) fn retro_memory_data(&self, id: u32) -> *mut std::os::raw::c_void {
        if !retro_memory_api_enabled() {
            return std::ptr::null_mut();
        }

        let Some(memory) = self.memory.as_ref() else {
            return std::ptr::null_mut();
        };

        match id {
            RETRO_MEMORY_SYSTEM_RAM => memory.base() as *mut std::os::raw::c_void,
            RETRO_MEMORY_VIDEO_RAM => unsafe {
                memory.base().add(XBOX_VRAM_START) as *mut std::os::raw::c_void
            },
            _ => std::ptr::null_mut(),
        }
    }

    pub(crate) fn retro_memory_size(&self, id: u32) -> usize {
        if !retro_memory_api_enabled() {
            return 0;
        }

        match id {
            RETRO_MEMORY_SYSTEM_RAM if self.memory.is_some() => XBOX_EXPOSED_SYSTEM_RAM_SIZE,
            RETRO_MEMORY_VIDEO_RAM if self.memory.is_some() => XBOX_EXPOSED_VRAM_SIZE,
            _ => 0,
        }
    }

    fn publish_memory_maps(&self) {
        static XBOX_RAM_ADDRSPACE: &[u8] = b"XBOX_RAM\0";
        static XBOX_VRAM_ADDRSPACE: &[u8] = b"XBOXVRAM\0";

        let Some(memory) = self.memory.as_ref() else {
            return;
        };
        let Some(env_cb) = (unsafe { G_ENVIRON }) else {
            return;
        };

        if !retro_memory_api_enabled() {
            debug_log(
                "[LIBRETRO-MEMORY] disabled by default; set RUSTEMU_ENABLE_RETRO_MEMORY=1 \
                 to expose RAM to RetroArch",
            );
            return;
        }

        if std::env::var_os("RUSTEMU_ENABLE_MEMORY_MAPS").is_none() {
            debug_log(
                "[LIBRETRO-MEMORY-MAP] skipped experimental SET_MEMORY_MAPS; \
                 retro_get_memory_data/size remains active",
            );
            return;
        }

        let base = memory.base() as *mut std::os::raw::c_void;
        let descriptors = retro_memory_map_descriptors();
        unsafe {
            // Keep the descriptor backing storage alive for the frontend. The
            // Xbox uses unified memory, but listing the framebuffer window first
            // lets RetroArch tools classify that physical range as VRAM while
            // retro_get_memory_data(RETRO_MEMORY_SYSTEM_RAM) still exposes all
            // 64MB as one contiguous forensic region.
            *descriptors.add(0) = RetroMemoryDescriptor {
                flags: RETRO_MEMDESC_VIDEO_RAM,
                ptr: base,
                offset: XBOX_VRAM_START,
                start: XBOX_VRAM_START,
                select: 0,
                disconnect: 0,
                len: XBOX_EXPOSED_VRAM_SIZE,
                addrspace: XBOX_VRAM_ADDRSPACE.as_ptr() as *const std::os::raw::c_char,
            };
            *descriptors.add(1) = RetroMemoryDescriptor {
                flags: RETRO_MEMDESC_SYSTEM_RAM,
                ptr: base,
                offset: 0,
                start: 0,
                select: 0,
                disconnect: 0,
                len: XBOX_EXPOSED_SYSTEM_RAM_SIZE,
                addrspace: XBOX_RAM_ADDRSPACE.as_ptr() as *const std::os::raw::c_char,
            };
        }

        let mut map = RetroMemoryMap {
            descriptors,
            num_descriptors: 2,
        };
        let ok = unsafe {
            env_cb(
                RETRO_ENVIRONMENT_SET_MEMORY_MAPS,
                &mut map as *mut RetroMemoryMap as *mut std::os::raw::c_void,
            )
        };
        debug_log(&format!(
            "[LIBRETRO-MEMORY-MAP] publish ok={} system_ram=0x{:X} vram_start=0x{:08X} vram_size=0x{:X}",
            ok, XBOX_EXPOSED_SYSTEM_RAM_SIZE, XBOX_VRAM_START, XBOX_EXPOSED_VRAM_SIZE
        ));
    }

    fn clear_memory_maps(&self) {
        let Some(env_cb) = (unsafe { G_ENVIRON }) else {
            return;
        };
        let mut map = RetroMemoryMap {
            descriptors: std::ptr::null(),
            num_descriptors: 0,
        };
        let _ = unsafe {
            env_cb(
                RETRO_ENVIRONMENT_SET_MEMORY_MAPS,
                &mut map as *mut RetroMemoryMap as *mut std::os::raw::c_void,
            )
        };
    }

    fn diagnostic_dump_option_enabled() -> bool {
        Self::core_option_value(b"rustemu_dump_diagnostics\0")
            .map(|value| {
                let value = value.to_ascii_lowercase();
                value == "enabled" || value == "1" || value == "true" || value == "yes"
            })
            .unwrap_or(false)
    }

    fn core_option_value(key: &'static [u8]) -> Option<String> {
        let Some(env_cb) = (unsafe { G_ENVIRON }) else {
            return None;
        };

        let mut var = RetroVariable {
            key: key.as_ptr() as *const std::os::raw::c_char,
            value: std::ptr::null(),
        };
        let ok = unsafe {
            env_cb(
                RETRO_ENVIRONMENT_GET_VARIABLE,
                &mut var as *mut RetroVariable as *mut std::os::raw::c_void,
            )
        };
        if !ok || var.value.is_null() {
            return None;
        }

        Some(
            unsafe { std::ffi::CStr::from_ptr(var.value) }
                .to_string_lossy()
                .to_string(),
        )
    }

    fn configure_cpu_backend() {
        let core_value = Self::core_option_value(b"rustemu_cpu_backend\0");
        crate::xbox::aot::jit::configure(core_value.as_deref());
    }

    fn poll_diagnostic_dump_option(&mut self) {
        let enabled = Self::diagnostic_dump_option_enabled();
        if !self.diag_dump_option_initialized {
            self.diag_dump_option_enabled = enabled;
            self.diag_dump_option_initialized = true;
            return;
        }
        if enabled && !self.diag_dump_option_enabled {
            self.write_diagnostic_dump("core-option");
        }
        self.diag_dump_option_enabled = enabled;
    }

    fn write_diagnostic_dump(&mut self, trigger: &str) {
        let Some(memory) = self.memory.as_ref() else {
            debug_log("[DIAG-DUMP] skipped: no guest memory loaded");
            return;
        };

        self.diag_dump_count = self.diag_dump_count.wrapping_add(1);
        let dir = std::path::Path::new(r".");
        let ram_path = dir.join("spider_ram.bin");
        let heap_path = dir.join("spider_heap_10000000_16mb.bin");
        let scene_path = dir.join("spider_scene.txt");
        let live_windows_path = dir.join("spider_live_windows.txt");
        let d3d_path = dir.join("spider_d3d.txt");

        let epoch_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let ram_len = XBOX_EXPOSED_SYSTEM_RAM_SIZE.min(memory.ram_size() as usize);
        let ram = unsafe { std::slice::from_raw_parts(memory.base() as *const u8, ram_len) };
        let ram_result = std::fs::write(&ram_path, ram);
        let heap_start = 0x1000_0000usize;
        let heap_len = if (memory.ram_size() as usize) > heap_start {
            0x0100_0000usize.min(memory.ram_size() as usize - heap_start)
        } else {
            0
        };
        let heap_result = if heap_len != 0 {
            let heap = unsafe {
                std::slice::from_raw_parts((memory.base() as *const u8).add(heap_start), heap_len)
            };
            std::fs::write(&heap_path, heap)
        } else {
            Ok(())
        };

        fn norm_addr(memory: &GuestMemory, addr: u32) -> Option<u32> {
            let ram_size = memory.ram_size();
            if addr < ram_size {
                Some(addr)
            } else if addr >= 0x8000_0000 && addr.wrapping_sub(0x8000_0000) < ram_size {
                Some(addr.wrapping_sub(0x8000_0000))
            } else {
                None
            }
        }
        fn read_u8_safe(memory: &GuestMemory, addr: u32) -> u8 {
            norm_addr(memory, addr)
                .map(|a| memory.read_u8(a))
                .unwrap_or(0)
        }
        fn read_u32_safe(memory: &GuestMemory, addr: u32) -> u32 {
            norm_addr(memory, addr)
                .map(|a| memory.read_u32(a))
                .unwrap_or(0)
        }
        fn dump_dwords(memory: &GuestMemory, base: u32, count: u32) -> String {
            let mut out = String::new();
            for i in 0..count {
                let addr = base.wrapping_add(i * 4);
                out.push_str(&format!(
                    "  +0x{:03X} [0x{:08X}] = 0x{:08X}\n",
                    i * 4,
                    addr,
                    read_u32_safe(memory, addr)
                ));
            }
            out
        }
        fn looks_printable(byte: u8) -> bool {
            byte == b'\t' || byte == b'\n' || byte == b'\r' || (0x20..=0x7e).contains(&byte)
        }
        fn read_c_string_safe(memory: &GuestMemory, addr: u32, max_len: usize) -> Option<String> {
            norm_addr(memory, addr)?;
            let mut bytes = Vec::new();
            for i in 0..max_len {
                let b = read_u8_safe(memory, addr.wrapping_add(i as u32));
                if b == 0 {
                    break;
                }
                if !looks_printable(b) {
                    return None;
                }
                bytes.push(b);
            }
            if bytes.len() >= 4 {
                Some(String::from_utf8_lossy(&bytes).to_string())
            } else {
                None
            }
        }
        fn dump_live_window(memory: &GuestMemory, name: &str, base: u32, len: usize) -> String {
            let mut out = String::new();
            let Some(norm) = norm_addr(memory, base) else {
                out.push_str(&format!("\n[{}] base=0x{:08X} invalid\n", name, base));
                return out;
            };
            let max_len = len.min(memory.ram_size().saturating_sub(norm) as usize);
            out.push_str(&format!(
                "\n[{}] base=0x{:08X} norm=0x{:08X} len=0x{:X}\n",
                name, base, norm, max_len
            ));
            let dword_count = (max_len / 4).min(96);
            out.push_str("  dwords:\n");
            for i in 0..dword_count {
                let addr = base.wrapping_add((i * 4) as u32);
                let val = read_u32_safe(memory, addr);
                let ptr_note = if val != 0 {
                    if let Some(s) = read_c_string_safe(memory, val, 80) {
                        format!(" -> '{}'", s)
                    } else if norm_addr(memory, val).is_some() {
                        " -> ptr".to_string()
                    } else if val >= 0x8000_0000 && norm_addr(memory, val).is_some() {
                        " -> mirror".to_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                out.push_str(&format!(
                    "    +0x{:03X} [0x{:08X}] = 0x{:08X}{}\n",
                    i * 4,
                    addr,
                    val,
                    ptr_note
                ));
            }
            out.push_str("  hex/ascii:\n");
            for row in 0..(max_len.min(512) + 15) / 16 {
                let addr = base.wrapping_add((row * 16) as u32);
                let mut hex = String::new();
                let mut ascii = String::new();
                for col in 0..16 {
                    let off = row * 16 + col;
                    if off < max_len.min(512) {
                        let b = read_u8_safe(memory, addr.wrapping_add(col as u32));
                        hex.push_str(&format!("{:02X} ", b));
                        ascii.push(if (0x20..=0x7e).contains(&b) {
                            b as char
                        } else {
                            '.'
                        });
                    } else {
                        hex.push_str("   ");
                        ascii.push(' ');
                    }
                }
                out.push_str(&format!("    0x{:08X}: {:48} {}\n", addr, hex, ascii));
            }
            out
        }
        fn add_window(
            windows: &mut Vec<(String, u32, usize)>,
            seen: &mut std::collections::BTreeSet<u32>,
            memory: &GuestMemory,
            name: impl Into<String>,
            addr: u32,
            len: usize,
        ) {
            let Some(norm) = norm_addr(memory, addr) else {
                return;
            };
            if addr == 0 || !seen.insert(norm) {
                return;
            }
            windows.push((name.into(), addr, len));
        }

        let frame_ctx = read_u32_safe(memory, 0x004B_C630);
        let frame_scene = read_u32_safe(memory, frame_ctx.wrapping_add(0x18));
        let frame_root = read_u32_safe(memory, frame_scene.wrapping_add(0x28));
        let frame_focus = read_u32_safe(memory, frame_root.wrapping_add(0x1A8));
        let global_scene = read_u32_safe(memory, 0x003F_5BEC);
        let global_root = read_u32_safe(memory, global_scene.wrapping_add(0x28));
        let global_focus = read_u32_safe(memory, global_root.wrapping_add(0x1A8));
        let xroot = read_u32_safe(memory, 0x004C_06B8);
        let xroot_focus = read_u32_safe(memory, xroot.wrapping_add(0x1A8));
        let fsm_state = read_u32_safe(memory, 0x0072_6690);
        let fsm_list = read_u32_safe(memory, 0x0072_6690 + 0x14);
        let pending_word = read_u32_safe(memory, 0x003D_CFA0);
        let pend_begin = read_u32_safe(memory, 0x003D_CEC0);
        let pend_table = read_u32_safe(memory, 0x003D_CEC4);
        let pend_end = read_u32_safe(memory, 0x003D_CEC8);
        let event_1a = read_u32_safe(memory, 0x003F_87A8);

        let mut key_table = String::new();
        for slot in 0..28u32 {
            let base = 0x003D_CEC0u32 + slot * 8;
            key_table.push_str(&format!(
                "  slot {:02X}: key=0x{:08X} pending=0x{:08X}\n",
                slot,
                read_u32_safe(memory, base),
                read_u32_safe(memory, base + 4)
            ));
        }

        let scene18c = read_u8_safe(memory, frame_scene.wrapping_add(0x18C));
        let scene17f = read_u8_safe(memory, frame_scene.wrapping_add(0x17F));
        let scene183 = read_u8_safe(memory, frame_scene.wrapping_add(0x183));
        let scene186 = read_u8_safe(memory, frame_scene.wrapping_add(0x186));
        let scene18b = read_u8_safe(memory, frame_scene.wrapping_add(0x18B));

        let mut scene_txt = String::new();
        scene_txt.push_str(&format!(
            "Rustemu Spider-Man diagnostic dump\n\
             trigger: {}\n\
             dump_count: {}\n\
             epoch_ms: {}\n\
             frame_count: {}\n",
            trigger, self.diag_dump_count, epoch_ms, self.frame_count
        ));
        if let Some(xbe) = &self.xbe_info {
            scene_txt.push_str(&format!(
                "xbe_title: {}\nentry: 0x{:08X}\nimage_size: 0x{:08X}\nd3d8_build: {}\n",
                xbe.title, xbe.entry_point, xbe.image_size, xbe.d3d8_build_version
            ));
        }
        scene_txt.push_str(&format!(
            "\n[scene]\n\
             frame_ctx=0x{:08X}\n\
             frame_scene=0x{:08X}\n\
             frame_root=0x{:08X}\n\
             frame_focus=0x{:08X}\n\
             global_scene=0x{:08X}\n\
             global_root=0x{:08X}\n\
             global_focus=0x{:08X}\n\
             xroot=0x{:08X}\n\
             xroot_focus=0x{:08X}\n\
             scene_flags 17f/183/186/18b/18c = {}/{}/{}/{}/{}\n\
             fsm_state=0x{:08X}\n\
             fsm_list=0x{:08X}\n\
             pending_word=0x{:08X}\n\
             pend_begin/table/end=0x{:08X}/0x{:08X}/0x{:08X}\n\
             global_event1a=0x{:08X}\n",
            frame_ctx,
            frame_scene,
            frame_root,
            frame_focus,
            global_scene,
            global_root,
            global_focus,
            xroot,
            xroot_focus,
            scene17f,
            scene183,
            scene186,
            scene18b,
            scene18c,
            fsm_state,
            fsm_list,
            pending_word,
            pend_begin,
            pend_table,
            pend_end,
            event_1a
        ));
        scene_txt.push_str("\n[key_table_003DCEC0]\n");
        scene_txt.push_str(&key_table);
        scene_txt.push_str("\n[frame_ctx_dwords]\n");
        scene_txt.push_str(&dump_dwords(memory, frame_ctx, 24));
        scene_txt.push_str("\n[frame_scene_dwords]\n");
        scene_txt.push_str(&dump_dwords(memory, frame_scene, 40));

        let mut windows = Vec::new();
        let mut seen_windows = std::collections::BTreeSet::new();
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "global_frame_ctx_ptr_004BC630",
            0x004B_C630,
            0x100,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "key_table_003DCEC0",
            0x003D_CEC0,
            0x120,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "frame_ctx",
            frame_ctx,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "frame_scene",
            frame_scene,
            0x600,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "frame_root",
            frame_root,
            0x800,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "frame_focus",
            frame_focus,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "global_scene",
            global_scene,
            0x600,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "global_root",
            global_root,
            0x800,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "global_focus",
            global_focus,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "xroot",
            xroot,
            0x800,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "xroot_focus",
            xroot_focus,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "fsm_state",
            fsm_state,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "fsm_list",
            fsm_list,
            0x400,
        );
        add_window(
            &mut windows,
            &mut seen_windows,
            memory,
            "global_event1a",
            event_1a,
            0x400,
        );
        for i in 0..128u32 {
            let off = i * 4;
            let scene_val = read_u32_safe(memory, frame_scene.wrapping_add(off));
            add_window(
                &mut windows,
                &mut seen_windows,
                memory,
                format!("frame_scene_ptr_plus_{:03X}", off),
                scene_val,
                0x300,
            );
            let root_val = read_u32_safe(memory, frame_root.wrapping_add(off));
            add_window(
                &mut windows,
                &mut seen_windows,
                memory,
                format!("frame_root_ptr_plus_{:03X}", off),
                root_val,
                0x300,
            );
        }
        let mut live_windows_txt = String::new();
        live_windows_txt.push_str(&format!(
            "Rustemu Spider-Man live memory windows\n\
             trigger: {}\n\
             dump_count: {}\n\
             epoch_ms: {}\n\
             live_ram_size: 0x{:08X}\n\
             spider_ram_bin_range: 0x00000000..0x{:08X}\n\
             heap_bin_range: 0x{:08X}..0x{:08X}\n\
             note: spider_ram.bin is intentionally the RetroArch-exposed low window; \
             these windows include live 0x10xxxxxx menu heap objects.\n",
            trigger,
            self.diag_dump_count,
            epoch_ms,
            memory.ram_size(),
            ram_len,
            heap_start,
            heap_start + heap_len
        ));
        for (name, addr, len) in &windows {
            live_windows_txt.push_str(&dump_live_window(memory, name, *addr, *len));
        }

        let live = self.worker_live_counters.as_ref();
        let live_dispatches = live
            .map(|c| c.dispatch_count.load(Ordering::Relaxed))
            .unwrap_or(self.worker_total_dispatches);
        let live_mmio = live
            .map(|c| c.mmio_count.load(Ordering::Relaxed))
            .unwrap_or(self.worker_total_mmio);
        let live_pb = live
            .map(|c| c.pb_commands.load(Ordering::Relaxed))
            .unwrap_or(self.worker_total_pb);
        let live_draws = live
            .map(|c| c.draw_calls.load(Ordering::Relaxed))
            .unwrap_or(self.worker_total_draw_calls);
        let live_kernel = live
            .map(|c| c.kernel_calls.load(Ordering::Relaxed))
            .unwrap_or(0);

        let d3d_txt = format!(
            "Rustemu D3D/GPU diagnostic dump\n\
             trigger: {}\n\
             dump_count: {}\n\
             epoch_ms: {}\n\
             frame_count: {}\n\
             backend: {}\n\
             oovpa_hooks: {}\n\
             oovpa_swaps: {}\n\
             oovpa_draws: {}\n\
             oovpa_pb_commands: {}\n\
             oovpa_mmio: {}\n\
             worker_dispatches: {}\n\
             worker_mmio: {}\n\
             worker_pb_commands: {}\n\
             worker_draws: {}\n\
             worker_kernel_calls: {}\n\
             live_ram_size: 0x{:08X}\n\
             ram_dump: {} bytes -> {}\n",
            trigger,
            self.diag_dump_count,
            epoch_ms,
            self.frame_count,
            crate::xbox::gpu::backend_name(),
            crate::xbox::aot::oovpa::OOVPA_HOOKS_FIRED.load(Ordering::Relaxed),
            crate::xbox::aot::oovpa::OOVPA_SWAP_COUNT.load(Ordering::Relaxed),
            crate::xbox::aot::oovpa::OOVPA_DRAW_CALLS.load(Ordering::Relaxed),
            crate::xbox::aot::oovpa::OOVPA_PB_COMMANDS.load(Ordering::Relaxed),
            crate::xbox::aot::oovpa::OOVPA_MMIO_COUNT.load(Ordering::Relaxed),
            live_dispatches,
            live_mmio,
            live_pb,
            live_draws,
            live_kernel,
            memory.ram_size(),
            ram_len,
            ram_path.display()
        );

        let scene_result = std::fs::write(&scene_path, scene_txt);
        let live_windows_result = std::fs::write(&live_windows_path, live_windows_txt);
        let d3d_result = std::fs::write(&d3d_path, d3d_txt);
        debug_log(&format!(
            "[DIAG-DUMP] trigger={} count={} ram={} heap={} windows={} scene={} d3d={} ram_result={:?} heap_result={:?} windows_result={:?} scene_result={:?} d3d_result={:?}",
            trigger,
            self.diag_dump_count,
            ram_path.display(),
            heap_path.display(),
            live_windows_path.display(),
            scene_path.display(),
            d3d_path.display(),
            ram_result.as_ref().map(|_| ()),
            heap_result.as_ref().map(|_| ()),
            live_windows_result.as_ref().map(|_| ()),
            scene_result.as_ref().map(|_| ()),
            d3d_result.as_ref().map(|_| ())
        ));
    }

    pub(crate) fn load_game(&mut self, info: *const RetroGameInfo) -> bool {
        self.spiderman_framectx_injected = false;
        self.diag_dump_option_enabled = false;
        self.diag_dump_option_initialized = false;
        self.diag_dump_count = 0;
        self.timing_start = std::time::Instant::now();
        Self::configure_cpu_backend();

        // Extract path
        let path = if !info.is_null() {
            let raw = unsafe { &*info };
            if !raw.path.is_null() {
                let c_str = unsafe { std::ffi::CStr::from_ptr(raw.path) };
                match c_str.to_str() {
                    Ok(s) => {
                        debug_log(&format!("path = {}", s));
                        Some(s.to_string())
                    }
                    Err(e) => {
                        debug_log(&format!("path UTF-8 error: {}", e));
                        None
                    }
                }
            } else {
                debug_log("path is NULL");
                None
            }
        } else {
            debug_log("game info is NULL");
            None
        };

        if let Some(path) = path {
            self.display.set_stage(BootStage::XbeLoading);
            debug_log("Allocating guest memory...");

            let memory = match GuestMemory::new() {
                Ok(m) => {
                    debug_log(&format!("Guest memory allocated: base={:p}", m.base()));
                    m
                }
                Err(e) => {
                    debug_log(&format!("GuestMemory::new() FAILED: {}", e));
                    return false;
                }
            };

            // Zero the guest zero page (0x0000-0x0FFF) then initialize fake KPCR.
            unsafe {
                std::ptr::write_bytes(memory.base(), 0u8, 0x1000);
            }

            // Initialize fake KPCR/KTHREAD structures in guest memory.
            // On x64 Windows FS.base=0, so `MOV EAX, FS:[0x28]` → `MOV EAX, [R15+0x28]`
            // reads guest address 0x28. Games (especially commercial XDK titles) expect
            // a fully initialized KPCR at FS:0 with valid CurrentThread, TLS, stack info.
            // Without this, null pointer chains corrupt ESP after ~6 dispatches.
            //
            // Layout matches C++ FakeKPCR.h:
            //   Guest 0x00000000: KPCR (zero page, where FS: reads land)
            //   Guest 0x0C000000: Backing structures (KPCR copy, KPRCB, KTHREAD, TLS)
            init_fake_kpcr(&memory);
            debug_log("Initialized fake KPCR/KTHREAD at zero page + 0x0C000000");

            debug_log(&format!("Parsing XBE: {}", path));
            let xbe_info = match xbe::load_xbe(&path, &memory) {
                Ok(info) => {
                    debug_log(&format!(
                        "XBE loaded: '{}' entry=0x{:08X} sections={} retail={}",
                        info.title, info.entry_point, info.num_sections, info.is_retail
                    ));
                    info
                }
                Err(e) => {
                    debug_log(&format!("load_xbe FAILED: {}", e));
                    return false;
                }
            };

            debug_log("PROBE 1: before set_game_title");
            self.display.set_game_title(&xbe_info.title);
            crate::xbox::logging::init_symbols_for_game(&path, &xbe_info.title);
            debug_log("PROBE 2: before set_stage");
            self.display.set_stage(BootStage::AotCompiling);
            debug_log("PROBE 3: after set_stage, before AOT");
            // Verify rdtsc bytes at 0x002A9397 after XBE load
            {
                let b0 = memory.read_u8(0x002A_9397);
                let b1 = memory.read_u8(0x002A_9398);
                debug_log(&format!(
                    "VERIFY: 0x002A9397=[{:02X},{:02X}] (expect 0F,31=rdtsc)",
                    b0, b1
                ));
            }

            // Pre-AOT guest code patches.
            super::patches::apply_pre_aot_patches(&memory, &xbe_info);

            // AOT compile all executable sections
            let mem_base = memory.base();
            let r15_base = mem_base as u64;

            // Phase 1: Decode + emit all sections, collecting blocks
            let mut emitted_blocks: Vec<(emitter::EmittedBlock, u32)> = Vec::new(); // (block, base_offset)
            let mut total_code = Vec::new();
            let mut all_traps = Vec::new();
            let mut total_addr_entries = 0u32;
            // E2 fix (2026-04-22): accumulate decoded instructions across all
            // executable sections so we can do a global recursive-descent
            // reachability pass before populating addr_hash. Without this,
            // linear-sweep decoded data bytes in EXEC-flagged sections (e.g.
            // KeDpc structs in the D3D section, INIT table padding) enter
            // the trusted addr_hash and get picked up by stack-scan recovery,
            // indirect-CALL resolution, and RET dispatch — cascading into
            // AV storms when the "code" they point at decodes to privileged
            // instructions.
            let mut all_instrs: Vec<decoder::DecodedInstr> = Vec::new();

            // Build thunk table for emitter constant folding and runtime repair.
            // Snapshot resolved thunk entries so the emitter can fold
            // `mov reg, [thunk_addr]` into `mov reg, imm32`, preventing corruption
            // from guest code that zeroes .rdata at runtime.
            let thunk_table = crate::xbox::loader::xbe::ThunkTable::snapshot(
                memory.base(),
                xbe_info.kernel_thunk_addr,
                xbe_info.base_address,
                xbe_info.image_size,
            );
            debug_log(&format!(
                "AOT: thunk table: {} entries at 0x{:08X}",
                thunk_table.len(),
                xbe_info.kernel_thunk_addr
            ));

            let aot_start = std::time::Instant::now();
            debug_log(&format!(
                "AOT: {} sections to scan",
                xbe_info.sections.len()
            ));
            // hash_shift for inline RET probe. Estimated at 10 (= 32 - 22 bits,
            // matching ~1.2M entries). Real value set on RuntimeContext after hash
            // table is sized. RetMiss VEH handler resolves any mismatches.
            let hash_shift_est: u8 = 10;
            for section in &xbe_info.sections {
                debug_log(&format!(
                    "AOT: section '{}' flags=0x{:08X} exec={} vsize=0x{:X}",
                    section.name, section.flags, section.executable, section.virtual_size
                ));
                if !section.executable || section.virtual_size == 0 {
                    continue;
                }
                let sname = section.name.to_lowercase();
                if sname == ".data" || sname == ".rdata" || sname == ".bss" {
                    debug_log(&format!("AOT: skipping '{}' (data section)", section.name));
                    continue;
                }

                debug_log(&format!(
                    "AOT: decoding section '{}' at 0x{:08X} size=0x{:X}",
                    section.name, section.virtual_address, section.virtual_size
                ));

                let guest_code: Vec<u8> = (0..section.virtual_size)
                    .map(|i| memory.read_u8(section.virtual_address + i))
                    .collect();

                let (instrs, stats) = decoder::decode_section(&guest_code, section.virtual_address);
                debug_log(&format!(
                    "AOT: decoded {} instrs (alu={} mov={} branch={} stack={} system={} other={})",
                    stats.total,
                    stats.cat_alu,
                    stats.cat_mov,
                    stats.cat_branch,
                    stats.cat_stack,
                    stats.cat_system,
                    stats.cat_other
                ));

                let block = emitter::emit_section(
                    &instrs,
                    &guest_code,
                    section.virtual_address,
                    &memory,
                    hash_shift_est,
                    thunk_table.as_slice(),
                );
                debug_log(&format!(
                    "AOT: emitted {} bytes (identity={} mem={} esp={} push={} call={} ret={} trap_sys={} trap_ind={} encode_fail={})",
                    block.code.len(), block.stats.identity, block.stats.rewrite_mem,
                    block.stats.rewrite_esp_reg,
                    block.stats.rewrite_push, block.stats.rewrite_call, block.stats.rewrite_ret,
                    block.stats.trap_system, block.stats.trap_indirect,
                    block.encode_fail_count
                ));
                if block.encode_fail_count > 0 {
                    for (addr, mnem) in &block.encode_fail_samples {
                        debug_log(&format!("  encode fail: 0x{:08X} {}", addr, mnem));
                    }
                }

                let base_offset = total_code.len() as u32;
                total_addr_entries += block.addr_map.len() as u32;

                for trap in &block.traps {
                    let mut adjusted = *trap;
                    adjusted.host_offset += base_offset;
                    all_traps.push(adjusted);
                }

                total_code.extend_from_slice(&block.code);
                emitted_blocks.push((block, base_offset));
                // E2: consume the per-section instrs vector into the global
                // reachability-analysis buffer. (emit_section took &instrs so
                // the vec is still owned here.)
                all_instrs.extend(instrs);
            }

            // E2 fix: compute the reachable-address set via recursive descent.
            // Seeds: the XBE entry point PLUS every direct branch target
            // found in the decoded instructions. The branch-target seeding
            // is critical — a walk from entry alone terminates early any
            // time it hits a `call` whose target happens to sit at a byte
            // offset the linear-sweep decoder chose as mid-instruction
            // (common when the called function is across a data-in-EXEC
            // desync boundary). Seeding every direct branch target catches
            // those function entries independently of walk chaining, and
            // the recursive walk then fans out from each one.
            let mut seeds: Vec<u32> = Vec::with_capacity(1024);
            seeds.push(xbe_info.entry_point);
            for di in &all_instrs {
                if let Some(target) = di.branch_target {
                    seeds.push(target);
                }
            }
            seeds.sort();
            seeds.dedup();
            let reachable = decoder::compute_reachable(&all_instrs, &seeds);
            debug_log(&format!(
                "AOT: reachable = {} of {} decoded addresses ({:.1}%) [{} seeds, entry=0x{:08X}]",
                reachable.len(),
                all_instrs.len(),
                100.0 * reachable.len() as f64 / all_instrs.len().max(1) as f64,
                seeds.len(),
                xbe_info.entry_point,
            ));

            let aot_elapsed = aot_start.elapsed();

            // Phase 2: Create RuntimeContext with properly sized hash table
            // Open addressing needs ~2x capacity for good performance
            let hash_bits = if total_addr_entries == 0 {
                16
            } else {
                let needed = (total_addr_entries as u64 * 2).next_power_of_two();
                (needed.trailing_zeros()).max(16)
            };
            debug_log(&format!(
                "AOT: {} address entries → hash table 2^{} = {} slots",
                total_addr_entries,
                hash_bits,
                1u64 << hash_bits
            ));

            let ctx_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                Box::new(RuntimeContext::new(mem_base, r15_base, hash_bits))
            }));
            let mut ctx = match ctx_result {
                Ok(c) => c,
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "unknown panic".to_string()
                    };
                    debug_log(&format!("AOT: RuntimeContext PANIC: {}", msg));
                    self.xbe_info = Some(xbe_info);
                    self.memory = Some(memory);
                    self.publish_memory_maps();
                    return true;
                }
            };

            // Insert all address mappings into the sized hash.
            //
            // E2 STATUS (2026-04-22): reachable-set filter is computed above
            // (`reachable` HashSet) but NOT applied here. Two attempts:
            //   1. Seed = [entry_point]. Walk reached only 390/1.57M addrs
            //      (0.0%). Walk terminated early at every direct CALL whose
            //      target landed on a byte offset the linear-sweep decoder
            //      did NOT choose as an instruction boundary (common when
            //      linear scan desyncs across data-in-EXEC regions).
            //      Blue Padded broke: no Clear/Swap fired.
            //   2. Seed = entry + ALL direct branch_targets (~72K seeds).
            //      Walk reached 64.1% — much better. But still breaks Blue
            //      Padded because ~36% of decoded addresses are filtered,
            //      including some legitimate callees that rescue_emit can't
            //      fully recover in the execution chain.
            //
            // The fundamental issue: we need recursive-descent DECODING,
            // not recursive-descent ANALYSIS of a linear sweep. Each walk
            // step should re-decode starting at the branch target to find
            // the real instruction boundary. That's a bigger change than
            // this session's budget — compute_reachable stays as dead
            // infrastructure; the reachability log above is diagnostic
            // only.
            //
            // The E2 symptom (KeDpc struct pointer 0x004C1BFC entering
            // addr_hash, then picked up by stack-scan recovery) is blocked
            // at the consumer side by the looks_like_ret_addr call-
            // precedence check in worker.rs (commit 1c3eb54). That handles
            // the observed repro. Indirect-CALL / RET dispatch paths could
            // eventually get the same consumer-side guard without needing
            // an emission-time filter.
            for (block, base_offset) in &emitted_blocks {
                for entry in &block.addr_map {
                    ctx.addr_hash.insert(
                        entry.guest_addr,
                        entry.host_offset + base_offset,
                        AddrTier::Main,
                    );
                }
            }
            let _reachable_len = reachable.len(); // keep compute alive for diag log

            // Phase 2b: Resolve cross-section branch fixups.
            // emit_call_rel creates JMP rel32 fixups that resolve within a single
            // section. Cross-section calls (e.g. .text calling D3D) are left as
            // JMP +0 (unresolved). Now that ALL sections are in addr_hash, we can
            // fix them by scanning each block's unresolved fixups.
            {
                let code = &mut total_code;
                let mut cross_fixed = 0u32;
                let mut still_unresolved = Vec::new();
                for (block, base_offset) in &emitted_blocks {
                    for fixup in &block.unresolved {
                        let global_host_offset = fixup.host_offset + base_offset;
                        if let Some(target_host) = ctx.addr_hash.lookup(fixup.guest_target) {
                            let rel32 = target_host as i64
                                - (global_host_offset as i64 + fixup.instr_end_delta as i64);
                            let rel32 = rel32 as i32;
                            let pos = global_host_offset as usize;
                            if pos + 4 <= code.len() {
                                code[pos..pos + 4].copy_from_slice(&rel32.to_le_bytes());
                                cross_fixed += 1;
                            }
                        } else {
                            still_unresolved.push((*fixup, *base_offset));
                        }
                    }
                }
                if cross_fixed > 0 {
                    debug_log(&format!(
                        "AOT: resolved {} cross-section branch fixups",
                        cross_fixed
                    ));
                }
                if !still_unresolved.is_empty() {
                    debug_log(&format!(
                        "AOT: {} fixups still unresolved (will retry after rescue-compile)",
                        still_unresolved.len()
                    ));
                    ctx.unresolved_fixups = still_unresolved;
                }
            }

            // Phase 3: Scan for jump tables (jmp [reg*4 + disp32]) and compile
            // their target addresses as additional blocks. The AOT linear scan
            // misses these because data bytes between jump table entries cause
            // the disassembler to desync, skipping the case handler code.
            let mut jt_targets_compiled = 0u32;
            let mut jt_fixups_resolved = 0u32;
            let mut jt_fixups_deferred = 0u32;
            for section in &xbe_info.sections {
                if !section.executable || section.virtual_size == 0 {
                    continue;
                }
                let sname = section.name.to_lowercase();
                if sname == ".data" || sname == ".rdata" || sname == ".bss" {
                    continue;
                }
                let va = section.virtual_address;
                let size = section.virtual_size;
                let guest_bytes: Vec<u8> = (0..size).map(|i| memory.read_u8(va + i)).collect();
                let (instrs, _) = decoder::decode_section(&guest_bytes, va);
                let mut jt_targets: Vec<u32> = Vec::new();
                for di in &instrs {
                    use iced_x86::{Mnemonic, OpKind, Register};
                    let inst = &di.instruction;
                    if inst.mnemonic() != Mnemonic::Jmp {
                        continue;
                    }
                    if inst.op0_kind() != OpKind::Memory {
                        continue;
                    }
                    if inst.memory_index_scale() != 4 {
                        continue;
                    }
                    if inst.memory_base() != Register::None {
                        continue;
                    }
                    let table_base = inst.memory_displacement32();
                    if table_base < va || table_base >= va + size {
                        continue;
                    }
                    for j in 0..32u32 {
                        let entry_addr = table_base + j * 4;
                        if entry_addr + 4 > va + size {
                            break;
                        }
                        let target = memory.read_u32(entry_addr);
                        if target < va || target >= va + size {
                            break;
                        }
                        if ctx.addr_hash.lookup(target).is_none() {
                            jt_targets.push(target);
                        }
                    }
                }
                jt_targets.sort();
                jt_targets.dedup();
                // Compile each missing jump table target as a mini-block
                for target in &jt_targets {
                    let offset_in_section = *target - va;
                    if offset_in_section >= size {
                        continue;
                    }
                    let remaining = (size - offset_in_section) as usize;
                    let block_size = remaining.min(512);
                    let block_bytes = &guest_bytes
                        [offset_in_section as usize..offset_in_section as usize + block_size];
                    let (block_instrs, _) = decoder::decode_section(block_bytes, *target);
                    let mut block = emitter::emit_section(
                        &block_instrs,
                        block_bytes,
                        *target,
                        &memory,
                        hash_shift_est,
                        thunk_table.as_slice(),
                    );
                    if !block.code.is_empty() {
                        let base_offset = total_code.len() as u32;
                        for fixup in &block.unresolved {
                            let global_host_offset = fixup.host_offset + base_offset;
                            if let Some(target_host) = ctx.addr_hash.lookup(fixup.guest_target) {
                                let rel32 = target_host as i64
                                    - (global_host_offset as i64 + fixup.instr_end_delta as i64);
                                let pos = fixup.host_offset as usize;
                                if pos + 4 <= block.code.len() {
                                    block.code[pos..pos + 4]
                                        .copy_from_slice(&(rel32 as i32).to_le_bytes());
                                    jt_fixups_resolved += 1;
                                }
                            } else {
                                ctx.unresolved_fixups.push((*fixup, base_offset));
                                jt_fixups_deferred += 1;
                            }
                        }
                        for entry in &block.addr_map {
                            ctx.addr_hash.insert(
                                entry.guest_addr,
                                entry.host_offset + base_offset,
                                AddrTier::Main,
                            );
                        }
                        for trap in &block.traps {
                            let mut adjusted = *trap;
                            adjusted.host_offset += base_offset;
                            all_traps.push(adjusted);
                        }
                        total_code.extend_from_slice(&block.code);
                        jt_targets_compiled += 1;
                    }
                }
            }
            if jt_targets_compiled > 0 {
                // Re-sort traps
                all_traps.sort_by_key(|t| t.host_offset);
                debug_log(&format!(
                    "AOT: compiled {} jump table targets ({} bytes added, mini-fixups resolved={} deferred={})",
                    jt_targets_compiled,
                    total_code.len(),
                    jt_fixups_resolved,
                    jt_fixups_deferred
                ));
            }

            // Phase 3b: Rescue-compile unresolved cross-section fixup targets.
            // The D3D section often contains inline data that desyncs the linear
            // decoder, leaving function entry points out of addr_hash. Compile
            // each unresolved target as a mini-block so fixups can resolve.
            {
                let mut rescue_compiled = 0u32;
                let mut rescue_fixups_resolved = 0u32;
                let mut targets: Vec<u32> = ctx
                    .unresolved_fixups
                    .iter()
                    .map(|(f, _)| f.guest_target)
                    .filter(|t| ctx.addr_hash.lookup(*t).is_none())
                    .collect();
                targets.sort();
                targets.dedup();

                for &target in &targets {
                    // Find which section contains this target
                    let section = xbe_info.sections.iter().find(|s| {
                        s.executable
                            && s.virtual_size > 0
                            && target >= s.virtual_address
                            && target < s.virtual_address + s.virtual_size
                    });
                    let section = match section {
                        Some(s) => s,
                        None => continue,
                    };
                    let va = section.virtual_address;
                    let size = section.virtual_size;
                    let offset_in_section = target - va;
                    let remaining = (size - offset_in_section) as usize;
                    let block_size = remaining.min(1024);
                    let block_bytes: Vec<u8> = (0..block_size as u32)
                        .map(|i| memory.read_u8(target + i))
                        .collect();
                    let (block_instrs, _) = decoder::decode_section(&block_bytes, target);
                    let mut block = emitter::emit_section(
                        &block_instrs,
                        &block_bytes,
                        target,
                        &memory,
                        hash_shift_est,
                        thunk_table.as_slice(),
                    );
                    if !block.code.is_empty() {
                        let base_offset = total_code.len() as u32;
                        for fixup in &block.unresolved {
                            let global_host_offset = fixup.host_offset + base_offset;
                            if let Some(target_host) = ctx.addr_hash.lookup(fixup.guest_target) {
                                let rel32 = target_host as i64
                                    - (global_host_offset as i64 + fixup.instr_end_delta as i64);
                                let pos = fixup.host_offset as usize;
                                if pos + 4 <= block.code.len() {
                                    block.code[pos..pos + 4]
                                        .copy_from_slice(&(rel32 as i32).to_le_bytes());
                                    rescue_fixups_resolved += 1;
                                }
                            } else {
                                ctx.unresolved_fixups.push((*fixup, base_offset));
                            }
                        }
                        for entry in &block.addr_map {
                            ctx.addr_hash.insert(
                                entry.guest_addr,
                                entry.host_offset + base_offset,
                                AddrTier::Main,
                            );
                        }
                        for trap in &block.traps {
                            let mut adjusted = *trap;
                            adjusted.host_offset += base_offset;
                            all_traps.push(adjusted);
                        }
                        total_code.extend_from_slice(&block.code);
                        rescue_compiled += 1;
                    }
                }

                // Re-resolve previously unresolved fixups
                if rescue_compiled > 0 {
                    let code = &mut total_code;
                    for (fixup, base_offset) in &ctx.unresolved_fixups {
                        let global_host_offset = fixup.host_offset + base_offset;
                        if let Some(target_host) = ctx.addr_hash.lookup(fixup.guest_target) {
                            let rel32 = target_host as i64
                                - (global_host_offset as i64 + fixup.instr_end_delta as i64);
                            let rel32 = rel32 as i32;
                            let pos = global_host_offset as usize;
                            if pos + 4 <= code.len() {
                                code[pos..pos + 4].copy_from_slice(&rel32.to_le_bytes());
                                rescue_fixups_resolved += 1;
                            }
                        }
                    }
                    all_traps.sort_by_key(|t| t.host_offset);
                    debug_log(&format!(
                        "AOT: rescue-compiled {} fixup targets, resolved {}/{} fixups",
                        rescue_compiled,
                        rescue_fixups_resolved,
                        ctx.unresolved_fixups.len()
                    ));
                }
            }

            debug_log(&format!(
                "AOT: total code buffer = {} bytes, {} address entries, {} traps (compiled in {:.1}s)",
                total_code.len(), ctx.addr_hash.count, all_traps.len(),
                aot_elapsed.as_secs_f64()
            ));

            // Sort traps for binary search
            all_traps.sort_by_key(|t| t.host_offset);
            ctx.trap_table = all_traps;

            // Build executable section ranges for interpreter guard.
            // Trust XBE section flags — if a section is marked executable, the
            // interpreter must be allowed to run code there. Xbox XBEs commonly
            // have executable .data sections (function pointer tables, launcher
            // stubs). Excluding them by name breaks indirect calls into those
            // sections (e.g. Classic Doom's launcher init at 0xE1840 in .data).
            ctx.exec_ranges = xbe_info
                .sections
                .iter()
                .filter(|s| s.executable && s.virtual_size > 0)
                .map(|s| (s.virtual_address, s.virtual_address + s.virtual_size))
                .collect();
            debug_log(&format!(
                "AOT: {} executable ranges for interpreter guard",
                ctx.exec_ranges.len()
            ));

            // E2 consumer-side guard: populate data_ranges with sections whose
            // NAME marks them as pure data (.data / .rdata / .bss), regardless
            // of the XBE header's EXEC flag. Spider-Man's .data is flagged
            // 0x7 (RWX) but contains string tables and statics — if a corrupt
            // function pointer resolves into that range, rescue_emit would
            // otherwise compile ASCII bytes as code and the resulting x64
            // block would fault on `gs: outsb` / `gs: popad` at runtime.
            ctx.data_ranges = xbe_info
                .sections
                .iter()
                .filter(|s| s.virtual_size > 0)
                .filter(|s| {
                    let n = s.name.to_lowercase();
                    n == ".data" || n == ".rdata" || n == ".bss"
                })
                .map(|s| (s.virtual_address, s.virtual_address + s.virtual_size))
                .collect();
            debug_log(&format!(
                "AOT: {} data-only section ranges (rescue_emit will reject): {:?}",
                ctx.data_ranges.len(),
                ctx.data_ranges
                    .iter()
                    .map(|(s, e)| format!("0x{:X}..0x{:X}", s, e))
                    .collect::<Vec<_>>()
            ));

            // Allocate executable code buffer
            if let Some(code_buf) = CodeBuffer::new(&total_code) {
                ctx.code_base = code_buf.base();
                ctx.code_size = total_code.len() as u32; // main code size (not including rescue reserve)

                // Set up rescue pool for on-demand JIT (lazy compilation)
                {
                    let rescue_offset = ((total_code.len() + 0xFFF) & !0xFFF) as u32; // page-align
                    let rescue_size = 1024 * 1024u32; // 1MB reserved in CodeBuffer::new
                    ctx.rescue_pool = unsafe { code_buf.base().add(rescue_offset as usize) };
                    ctx.rescue_pool_offset = rescue_offset;
                    ctx.rescue_pool_size = rescue_size;
                    ctx.rescue_pool_used = 0;
                    debug_log(&format!(
                        "Rescue pool: code_base+0x{:X} ({:p}) size={}KB",
                        rescue_offset,
                        ctx.rescue_pool,
                        rescue_size / 1024
                    ));
                }

                ctx.guest.esp = 0x00B0_0000; // Initial stack pointer (in guest RAM)
                memory.write_u32(0x00B0_0000, 0); // Sentinel: RET pops 0 → RET_TO_ZERO
                ctx.guest.r15_base = r15_base;

                // Set entry point
                if let Some(entry_host_offset) = ctx.addr_hash.lookup(xbe_info.entry_point) {
                    debug_log(&format!(
                        "AOT: entry 0x{:08X} -> host offset 0x{:X}",
                        xbe_info.entry_point, entry_host_offset
                    ));
                } else {
                    debug_log(&format!(
                        "AOT: WARNING — entry point 0x{:08X} not in address hash!",
                        xbe_info.entry_point
                    ));
                }

                // Zero the first page of guest RAM (zero page).
                // XapiThreadStartup with ctx=0 reads [ctx+8] for callback ptr.
                // If zero page contains stale data, CALL [0x10] enters random code
                // and the worker spins forever. Zeroing ensures callbacks = NULL.
                for i in (0..4096u32).step_by(8) {
                    memory.write_u32(i, 0);
                    memory.write_u32(i + 4, 0);
                }
                // Restore KPCR at zero page — on real Xbox, address 0 IS the KPCR.
                // Game code that dereferences null pointers (e.g. empty hash table
                // bucket chains) reads from here. Without valid KPCR data, strncmp
                // on address 0 returns negative (all zeros) causing infinite loops.
                // With 0xFFFFFFFF at offset 0, strncmp returns positive and exits.
                init_fake_kpcr_zero_page(&memory);
                debug_log("AOT: zeroed guest zero page (4KB) + restored KPCR");

                // Verify memmove jump table at 0x2B7680 (debugging partition0 issue)
                {
                    let jt_base = 0x002B7680u32;
                    let mut vals = [0u32; 4];
                    for i in 0..4 {
                        vals[i] = memory.read_u32(jt_base + i as u32 * 4);
                    }
                    debug_log(&format!(
                        "VERIFY: memmove jump table at 0x{:08X}: [{:#010X}, {:#010X}, {:#010X}, {:#010X}]",
                        jt_base, vals[0], vals[1], vals[2], vals[3]
                    ));
                }

                // NV2A bootstrap — initialize GPU shadow registers + PRAMIN
                // Must happen BEFORE VEH install (VEH reads shadow regs on MMIO access)
                let nv2a_state = crate::xbox::aot::nv2a::bootstrap(&memory);
                crate::xbox::aot::nv2a::install_state(nv2a_state);

                // Install VEH
                crate::xbox::aot::veh::install_veh(&mut ctx);

                // OOVPA: scan D3D/executable sections for SDK function patterns
                {
                    use crate::xbox::aot::oovpa;
                    use crate::xbox::aot::oovpa::{OovpaScanRange, OovpaScanRangeKind};
                    let guest_base = memory.base();

                    // Section-targeted OOVPA scanning. The scanner now consumes
                    // library-aware ranges so D3D patterns don't walk the whole XBE.
                    let mut scan_ranges: Vec<OovpaScanRange> = Vec::new();
                    for section in &xbe_info.sections {
                        if section.executable && section.virtual_size > 0 {
                            let upper_name = section.name.to_ascii_uppercase();
                            let kind = if upper_name.contains("D3DX") {
                                OovpaScanRangeKind::D3dx
                            } else if upper_name.contains("D3D") {
                                OovpaScanRangeKind::D3d
                            } else if upper_name.contains("XGRPH") || upper_name.contains("XGRAPH")
                            {
                                OovpaScanRangeKind::Xgrph
                            } else if upper_name.contains("DSOUND") {
                                OovpaScanRangeKind::Dsound
                            } else if upper_name.contains("XPP")
                                || upper_name.contains("XAPI")
                                || upper_name.contains("XINPUT")
                            {
                                OovpaScanRangeKind::Xapi
                            } else {
                                OovpaScanRangeKind::OtherExec
                            };
                            scan_ranges.push(OovpaScanRange {
                                start: section.virtual_address,
                                end: section.virtual_address + section.virtual_size,
                                kind,
                            });
                        }
                    }
                    let mut scan_start = u32::MAX;
                    let mut scan_end = 0u32;
                    let mut scan_bytes = 0u32;
                    for r in &scan_ranges {
                        if r.start < scan_start {
                            scan_start = r.start;
                        }
                        if r.end > scan_end {
                            scan_end = r.end;
                        }
                        scan_bytes += r.end - r.start;
                    }
                    debug_log(&format!(
                        "[OOVPA] Scan range: {} sections, {} bytes (0x{:08X}-0x{:08X})",
                        scan_ranges.len(),
                        scan_bytes,
                        scan_start,
                        scan_end
                    ));

                    // Compute XBE hash for cache keying
                    let xbe_hash = {
                        let base = memory.base();
                        let mut h: u64 = 0xcbf29ce484222325; // FNV-1a offset basis
                                                             // Hash first 4KB of XBE header + entry point + build version
                        for i in 0..4096u32.min(xbe_info.image_size) {
                            let b =
                                unsafe { *base.add(xbe_info.base_address as usize + i as usize) };
                            h ^= b as u64;
                            h = h.wrapping_mul(0x100000001b3); // FNV-1a prime
                        }
                        h
                    };

                    let symbol_fixture_enabled = [
                        "RUSTEMU_OOVPA_MAP_SYMBOLS",
                        "RUSTEMU_OOVPA_SYMBOLS_MAP",
                        "RUSTEMU_OOVPA_SYMBOLS_TOML",
                        "RUSTEMU_OOVPA_SYMBOLS_INI",
                        "RUSTEMU_OOVPA_SYMBOLS_CACHE",
                        "RUSTEMU_SPIDEY_4134_TOML",
                        "RUSTEMU_DOOM_5849_TOML",
                    ]
                    .iter()
                    .any(|name| {
                        std::env::var(name)
                            .map(|v| {
                                let v = v.trim().to_ascii_lowercase();
                                !(v.is_empty()
                                    || v == "0"
                                    || v == "false"
                                    || v == "off"
                                    || v == "no")
                            })
                            .unwrap_or(false)
                    }) || oovpa::symbol_cache_input_enabled(
                        xbe_info.entry_point,
                        Some(xbe_info.title_id),
                        xbe_info.d3d8_build_version,
                        xbe_info.d3d8_is_ltcg,
                    );

                    // Try loading from cache. External SymbolCache/TOML inputs
                    // are authoritative scan inputs, so they must bypass the
                    // generated cache; otherwise stale scanner-only output can
                    // mask newly loaded Cxbx-R symbol coverage.
                    let cache_path = oovpa::cache_path(&path, xbe_hash);
                    let mut matches = if symbol_fixture_enabled {
                        debug_log(&format!(
                            "[OOVPA] Symbol fixture/cache enabled; bypassing generated cache: {}",
                            cache_path
                        ));
                        oovpa::scan_ranges(
                            guest_base,
                            &scan_ranges,
                            xbe_info.d3d8_build_version,
                            xbe_info.entry_point,
                            xbe_info.d3d8_is_ltcg,
                            Some(xbe_info.title_id),
                        )
                    } else {
                        if let Some(cached) = oovpa::load_cache(
                            &cache_path,
                            xbe_info.d3d8_build_version,
                            xbe_info.entry_point,
                        ) {
                            debug_log(&format!(
                                "[OOVPA] Loaded {} matches from cache: {}",
                                cached.len(),
                                cache_path
                            ));
                            cached
                        } else {
                            let m = oovpa::scan_ranges(
                                guest_base,
                                &scan_ranges,
                                xbe_info.d3d8_build_version,
                                xbe_info.entry_point,
                                xbe_info.d3d8_is_ltcg,
                                Some(xbe_info.title_id),
                            );
                            oovpa::save_cache(&cache_path, &m, xbe_info.d3d8_build_version);
                            m
                        }
                    };
                    oovpa::log_diff_against_previous_cache(
                        &cache_path,
                        xbe_info.d3d8_build_version,
                        xbe_info.entry_point,
                        &matches,
                    );
                    oovpa::infer_g_pdevice_global(guest_base, scan_end, &matches);

                    if scan_start < scan_end {
                        let (mut planted, misaligned) =
                            oovpa::plant_hooks(&mut matches, ctx.code_base, &ctx.addr_hash);

                        // Rescue-compile misaligned OOVPA entries so they get exact addr_hash entries
                        if !misaligned.is_empty() {
                            debug_log(&format!(
                                "[OOVPA] Rescue-compiling {} misaligned hooks...",
                                misaligned.len()
                            ));
                            for &(idx, guest_addr) in &misaligned {
                                crate::xbox::aot::jit::note_oovpa_rescue_candidate(
                                    matches[idx].pattern_name,
                                    guest_addr,
                                );
                                if let Some(host_off) = ctx.rescue_emit(guest_addr) {
                                    if oovpa::plant_int3_pub(
                                        ctx.code_base,
                                        host_off,
                                        &mut matches[idx],
                                    ) {
                                        planted += 1;
                                        debug_log(&format!(
                                            "[OOVPA] Rescue-planted {} at guest 0x{:08X} host=+0x{:X}",
                                            matches[idx].pattern_name, guest_addr, host_off
                                        ));
                                    }
                                } else {
                                    debug_log(&format!(
                                        "[OOVPA] Rescue-compile failed for {} at guest 0x{:08X}",
                                        matches[idx].pattern_name, guest_addr
                                    ));
                                }
                            }
                            // Now re-resolve main code fixups that targeted rescue-compiled addresses
                            let re_resolved = ctx.resolve_pending_fixups();
                            if re_resolved > 0 {
                                debug_log(&format!(
                                    "[OOVPA] Re-resolved {} main code fixups after rescue-compile",
                                    re_resolved
                                ));
                            }
                        }

                        if planted < matches.len() as u32 {
                            let mut dropped = Vec::new();
                            for m in matches.iter().filter(|m| !m.active).take(16) {
                                dropped.push(format!("{}@0x{:08X}", m.pattern_name, m.guest_addr));
                            }
                            debug_log(&format!(
                                "[OOVPA] Hooks not planted: {} of {} [{}{}]",
                                matches.len().saturating_sub(planted as usize),
                                matches.len(),
                                dropped.join(", "),
                                if matches.len().saturating_sub(planted as usize) > dropped.len() {
                                    ", ..."
                                } else {
                                    ""
                                }
                            ));
                        }

                        debug_log(&format!(
                            "OOVPA: {} matches, {} hooks planted (XDK build={})",
                            matches.len(),
                            planted,
                            xbe_info.d3d8_build_version
                        ));

                        // Validate all hooks against Cxbx-R SymbolCache + check prologues/argc
                        oovpa::validate_hooks(
                            &matches,
                            guest_base,
                            scan_end,
                            xbe_info.entry_point,
                            Some(xbe_info.title_id),
                            xbe_info.d3d8_build_version,
                            xbe_info.d3d8_is_ltcg,
                        );

                        let state = oovpa::OovpaState {
                            matches,
                            hooks_planted: planted > 0,
                        };
                        oovpa::install_state(state);
                    } else {
                        debug_log("OOVPA: no executable sections to scan");
                    }
                }

                // Guest code patches: NOP both HalReturnToFirmware call sites.
                // Spider-Man XDK 4134 has exactly 2 calls to HalReturnToFirmware:
                //   0x002A9547: push 2; call [0x381838]  (6 bytes: 6A 02 FF 15 38 18 38 00)
                //   0x002AD4B3: push 4; call [0x381838]  (8 bytes: 6A 04 FF 15 38 18 38 00)
                // NOP both: replace push+call (8 bytes) with NOPs (0x90).
                // This prevents the game from rebooting on ANY error condition.
                {
                    let patches: &[(u32, usize, &str)] = &[
                        (0x002A_9547, 8, "HalReturnToFirmware(2) — partition0 error"),
                        (0x002A_D4B3, 8, "HalReturnToFirmware(4) — fatal error"),
                    ];
                    for (addr, len, desc) in patches {
                        let b0 = memory.read_u8(*addr);
                        if b0 == 0x6A {
                            // push imm8
                            for i in 0..*len as u32 {
                                memory.write_u8(*addr + i, 0x90); // NOP
                            }
                            debug_log(&format!(
                                "PATCH: 0x{:08X} NOP {} bytes ({})",
                                addr, len, desc
                            ));
                        } else {
                            debug_log(&format!(
                                "PATCH: 0x{:08X} expected 0x6A (push), got 0x{:02X} — skip",
                                addr, b0
                            ));
                        }
                    }
                    // Also patch the JL→JMP at 0x002A952E for good measure
                    if memory.read_u8(0x002A_952E) == 0x7C {
                        memory.write_u8(0x002A_952E, 0xEB);
                        debug_log("PATCH: 0x002A952E JL→JMP (skip error path)");
                    }

                    // 0x002AD598 pool_init: NO LONGER PATCHED — inline RET fixes cdecl.
                    // Was: HLE stub returning 1. Now: let full pool init run.
                }

                // Pre-initialize D3D device struct so D3D helpers don't crash on NULL g_pDevice.
                // On real Xbox, D3D_CreateDevice sets this up. We pre-populate it so early
                // D3D helper calls (before CreateDevice) see a valid device pointer.
                {
                    use crate::xbox::aot::oovpa::{
                        DEV_PB_BASE, DEV_PB_LIMIT, DEV_PB_PUT, G_PDEVICE, PB_DUMMY_BASE,
                        PB_DUMMY_SIZE,
                    };
                    let dev_addr = 0x0030_0E00u32;
                    let dev_size = 0x2C00u32;
                    // Zero device struct
                    for i in (0..dev_size).step_by(4) {
                        memory.write_u32(dev_addr + i, 0);
                    }
                    // Write g_pDevice
                    memory.write_u32(G_PDEVICE, dev_addr);

                    // Device+0 and +4: pPut and pThreshold (read by D3D_MakeSpace at 0x2F2D80)
                    // Must point into the pushbuffer region, NOT zero!
                    memory.write_u32(dev_addr + 0, PB_DUMMY_BASE); // pPut cursor
                    memory.write_u32(dev_addr + 4, PB_DUMMY_BASE + PB_DUMMY_SIZE - 0x400); // pThreshold
                                                                                           // Device+8: flags — matches Unicorn dump (0x13 = bit0 + bit1 + bit4)
                                                                                           // bit0: device created, bit1: device active, bit4: skip bulk copy
                    memory.write_u32(dev_addr + 8, 0x13);

                    // Pushbuffer pointers at higher offsets (used by KickOff, Swap, etc.)
                    memory.write_u32(dev_addr + DEV_PB_PUT, PB_DUMMY_BASE);
                    memory.write_u32(dev_addr + DEV_PB_BASE, PB_DUMMY_BASE);
                    memory.write_u32(dev_addr + DEV_PB_LIMIT, PB_DUMMY_BASE + PB_DUMMY_SIZE);

                    // g_PushBufferSize at 0x3038EC (read by D3D_MakeSpace slow path)
                    memory.write_u32(0x003038EC, PB_DUMMY_SIZE);

                    debug_log(&format!(
                        "D3D pre-init: dev=0x{:08X} g_pDevice=0x{:08X} PB=0x{:08X} size=0x{:X}",
                        dev_addr, G_PDEVICE, PB_DUMMY_BASE, PB_DUMMY_SIZE
                    ));
                }

                // Pre-initialize Xbox HDD partition count global.
                // [0x381804] is a pointer to a u32 containing the number of HDD partitions.
                // The game reads this to size a memcpy buffer during partition0 parsing.
                // If uninitialized, the memcpy count is garbage (ECX=0x38307F03) → AV crash.
                // Standard Xbox has 5 partitions. We store the value at 0x0C001000
                // (safe backing area) and point 0x381804 there.
                {
                    let partition_count_addr = 0x0C00_1000u32; // backing store
                    memory.write_u32(partition_count_addr, 5); // 5 standard partitions
                    memory.write_u32(0x0038_1804, partition_count_addr); // pointer to count
                    debug_log(&format!(
                        "HDD partition count: [0x381804]=0x{:08X} → *={}",
                        partition_count_addr, 5
                    ));
                }

                // Save .text snapshot for VEH instruction decoding.
                // VirtualProtect(PAGE_READONLY) does NOT work on MapViewOfFile3 pages,
                // so we save a copy of the original bytes instead.
                {
                    let text_start = 0x0001_1000u32;
                    let text_size = 0x002E_BDB0u32 - text_start; // .text section size
                    crate::xbox::aot::veh_dispatch::save_text_snapshot(
                        memory.base(),
                        text_start,
                        text_size,
                    );
                }

                // Verify bytes survived AOT + watchpoint setup
                {
                    let b0 = memory.read_u8(0x002A_9397);
                    let b1 = memory.read_u8(0x002A_9398);
                    debug_log(&format!(
                        "VERIFY POST-AOT: 0x002A9397=[{:02X},{:02X}]",
                        b0, b1
                    ));
                }

                self.code_buffer = Some(code_buf);
                // Read XBE file data for on-demand section loading (XeLoadSection)
                let xbe_data = std::fs::read(&path).unwrap_or_default();
                debug_log(&format!(
                    "Kernel: loaded {} bytes of XBE file data for XeLoadSection",
                    xbe_data.len()
                ));

                // Query RetroArch system directory for EmuDisk partition layout
                let system_dir = unsafe {
                    let mut sys_ptr: *const std::os::raw::c_char = std::ptr::null();
                    if let Some(cb) = G_ENVIRON {
                        cb(
                            RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY,
                            &mut sys_ptr as *mut _ as *mut std::os::raw::c_void,
                        );
                    }
                    if sys_ptr.is_null() {
                        String::new()
                    } else {
                        std::ffi::CStr::from_ptr(sys_ptr)
                            .to_string_lossy()
                            .to_string()
                    }
                };
                debug_log(&format!("RetroArch system dir: '{}'", system_dir));
                let mut ks = kernel::KernelState::with_xbe_data(
                    xbe_data,
                    &path,
                    &system_dir,
                    xbe_info.title_id,
                    xbe_info.allowed_media,
                    xbe_info.game_region,
                );
                ks.thunk_info = Some((
                    xbe_info.kernel_thunk_addr,
                    xbe_info.base_address,
                    xbe_info.image_size,
                ));
                self.kernel_state = Some(ks);
                self.display.set_stage(BootStage::CacheReady);
            } else {
                debug_log("AOT: FAILED to allocate executable code buffer");
            }

            // Snapshot resolved thunk table so VEH can repair entries overwritten by guest code
            ctx.snapshot_thunks(
                xbe_info.kernel_thunk_addr,
                xbe_info.base_address,
                xbe_info.image_size,
            );

            self.xbe_info = Some(xbe_info);
            self.aot_ctx = Some(ctx);
            self.memory = Some(memory);
            self.publish_memory_maps();
        } else {
            debug_log("No content — showing boot display only");
        }

        true
    }

    pub(crate) fn run(&mut self) {
        self.frame_count += 1;
        note_spidey_synth_frame(self.frame_count);
        self.poll_diagnostic_dump_option();
        self.submit_audio_frame();

        // Worker RIP sampler: suspend-read-resume the worker thread every
        // 3 frames (~20Hz at 60fps) to build a histogram of where it spends
        // time. Dump top-15 buckets every 200 samples. Finds what the
        // game is waiting on when no VEH events fire per swap.
        #[cfg(windows)]
        if self.frame_count % 3 == 0 {
            if let (Some(cb), Some(ctx)) = (self.code_buffer.as_ref(), self.aot_ctx.as_ref()) {
                let before = WORKER_RIP_SAMPLES.load(Ordering::Relaxed);
                sample_worker_rip(cb.ptr as u64, cb.len as u32, &ctx.addr_hash);
                let samples = WORKER_RIP_SAMPLES.load(Ordering::Relaxed);
                // First-sample diag so we know the sampler is firing.
                if before == 0 && samples > 0 {
                    debug_log("[RIP-HIST] sampler armed — first sample taken");
                }
                if samples > 0 && samples % 200 == 0 {
                    dump_worker_rip_histogram();
                }
            }
        }

        // Observe the game FSM without forcing it. Earlier probes wrote
        // ACTIVE here, which contaminated the exact boot transition we now
        // need to trace with TAP hooks.
        if self.frame_count == 60 {
            if let Some(ref mem) = self.memory {
                let sm = 0x0072_6690u32;
                let current_state = mem.read_u32(sm);
                debug_log(&format!(
                    "[FSM] frame 60 observed state=0x{:08X}",
                    current_state
                ));
            }
        }

        // Spider-Man organic fallback: once the real App/Scene/Engine globals
        // exist, but the FrameContext global still hasn't populated, seed a
        // minimal frame_ctx so the frame-update path can call scene tick via
        // [frame_ctx+0x18] and use [frame_ctx+0x20] as its render gate.
        // 2026-04-24: SYNTH-FRAMECTX scaffolding REMOVED.
        //
        // Previously this block synthesized a minimal FrameContext at
        // guest 0x004BC630 once the organic App/Scene/Engine globals
        // were populated but FrameContext itself was still NULL.
        //
        // Consequence: the game's organic FrameContext constructor is
        // the trigger for the VFS/stash manifest init chain (which
        // populates scene+0xFC with "heroes\aux_xbox.txt"). Pre-seeding
        // the FrameContext pointer short-circuits that constructor, so
        // the stash chain never runs, scene+0xFC stays NULL, the
        // compare at 0x0001AE6D spins a WaitForThreadCompletion loop
        // forever on the never-spawned scene worker thread.
        //
        // Removing this will likely surface whatever earlier bug the
        // scaffolding was originally papering over (older docs mention
        // "null callback at 0x003F5A48" in the static-ctor phase).
        // That crash — if it reappears — is the real next problem to
        // fix with targeted kernel HLE, not a short-circuit.
        let _ = self.spiderman_framectx_injected; // keep field for log consumers

        // Shenmue II diagnostic probe. The post-loading black screen currently
        // parks in a native AOT wait loop at 0x000F7857:
        //   target = [0x01AC2124] + [0x019A5C04] - 1
        //   wait while target > [0x01AC2068]
        // That counter is normally advanced by the D3D VBlank callback chain.
        // Because Rustemu's callback delivery is still dispatch-bound, this
        // env-gated poke tests whether the remaining stall is only missing
        // async VBlank delivery while guest code spins without traps.
        if std::env::var_os("RUSTEMU_SHENMUE_FRAME_COUNTER_POKE").is_some()
            && self.xbe_info.as_ref().map(|x| x.title_id) == Some(0x4D53_0032)
        {
            if let Some(ref mem) = self.memory {
                const SHENMUE_FRAME_COUNTER: u32 = 0x01AC_2068;
                const SHENMUE_FRAME_BASE: u32 = 0x01AC_2124;
                const SHENMUE_FRAME_DELTA: u32 = 0x019A_5C04;

                let base = mem.read_u32(SHENMUE_FRAME_BASE);
                let delta = mem.read_u32(SHENMUE_FRAME_DELTA);
                let current = mem.read_u32(SHENMUE_FRAME_COUNTER);
                let target = base.wrapping_add(delta).wrapping_sub(1);

                static SHENMUE_FRAME_POKE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                if self.frame_count.is_multiple_of(300) {
                    debug_log(&format!(
                        "[SHENMUE-FRAME-POKE] sample frame={} base={} delta={} target={} current={}",
                        self.frame_count, base, delta, target, current
                    ));
                }

                if base != 0
                    && delta != 0
                    && base < 1_000_000
                    && delta < 1_000_000
                    && target < 1_000_000
                    && target > current
                {
                    mem.write_u32(SHENMUE_FRAME_COUNTER, target);
                    let n = SHENMUE_FRAME_POKE_LOG.fetch_add(1, Ordering::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[SHENMUE-FRAME-POKE] #{} frame={} counter {} -> {} (base={} delta={})",
                            n, self.frame_count, current, target, base, delta
                        ));
                    }
                }
            }
        }

        // [FRAME-PTR-TICK] Per-frame direct sample of the Frame subsystem pointer
        // at 0x004BC630. SUBSYS-WATCH only logs transitions and misses 0→0
        // writes. This logger prints the CURRENT value every frame for the
        // first 200 frames, and then only when the value is non-zero (catches
        // transient populate-then-clobber windows if they happen between
        // frames). If every sample is 0x00000000 for hundreds of frames after
        // master init runs, Frame body (sub_0xD67E0) is returning NULL and
        // the post-body store at 0x2A4A87 writes 0→0 — confirming Branch B.
        if let Some(ref mem) = self.memory {
            let frame_val = mem.read_u32(0x004B_C630);
            static FIRST_NONZERO_LOGGED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if self.frame_count < 200 {
                if self.frame_count % 10 == 0 || frame_val != 0 {
                    debug_log(&format!(
                        "[FRAME-PTR-TICK] frame={} [0x004BC630]=0x{:08X}",
                        self.frame_count, frame_val
                    ));
                }
            }
            if frame_val != 0
                && !FIRST_NONZERO_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                debug_log(&format!(
                    "[FRAME-PTR-TICK] FIRST NON-ZERO at frame={} value=0x{:08X}",
                    self.frame_count, frame_val
                ));
            }
        }

        // [CALLBACK-WATCH] 0x003F5A48 is the null-callback pointer whose empty
        // state drives the spin loop at 0x00016F50 we documented earlier. If
        // one of the Frame helpers (A/B/C/D) is the one that was supposed to
        // install it, we'll see this pointer populate at a specific frame —
        // and that helper is our hang culprit. Log first non-zero transition
        // with the frame number and the value, then stop.
        if let Some(ref mem) = self.memory {
            let cb = mem.read_u32(0x003F_5A48);
            static CB_LOGGED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if cb != 0 && !CB_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                debug_log(&format!(
                    "[CALLBACK-WATCH] [0x003F5A48] populated at frame={} value=0x{:08X}",
                    self.frame_count, cb
                ));
            }
            // Also log periodically while it stays NULL (every 300 frames)
            if cb == 0 && self.frame_count > 0 && self.frame_count.is_multiple_of(300) {
                debug_log(&format!(
                    "[CALLBACK-WATCH] frame={} [0x003F5A48]=0x00000000 (still NULL)",
                    self.frame_count
                ));
            }
        }

        // [SUBSYS-WATCH] Per-frame polling watchpoint on 0x004BC600..0x004BC700.
        //
        // Engine subsystem ptrs Engine=0x004BC614 and Frame=0x004BC630 sit in
        // the same .data block (28 bytes apart — likely fields of a single
        // global SubsystemRegistry struct). Their constructors use indirect
        // addressing (mov [reg+disp], ptr) so static disasm can't find the
        // writer. This polling watchpoint catches transitions at 60Hz: every
        // 0→non-zero (constructor fired) and every non-zero→0 (destructor
        // fired) is logged with the host frame# so we can correlate writes
        // to other events.
        //
        // Why per-frame poll vs PAGE_GUARD trap: polling can miss writes that
        // are overwritten between frames, but the transitions we care about
        // (constructor sets ptr, destructor clears it) are stable for many
        // frames. PAGE_GUARD would catch every write but cost a VEH trap each
        // time. Polling is the right tradeoff for engine bootstrap analysis.
        if let Some(ref mem) = self.memory {
            const SUBSYS_BASE: u32 = 0x004B_C600;
            const SUBSYS_SLOTS: usize = 64; // 256 bytes
                                            // Last-seen shadow. None until the first scan.
            static SHADOW: std::sync::Mutex<Option<[u32; SUBSYS_SLOTS]>> =
                std::sync::Mutex::new(None);
            let mut guard = SHADOW.lock().unwrap_or_else(|e| e.into_inner());
            let snap = guard.get_or_insert([0u32; SUBSYS_SLOTS]);
            for i in 0..SUBSYS_SLOTS {
                let addr = SUBSYS_BASE + (i as u32) * 4;
                let cur = mem.read_u32(addr);
                let prev = snap[i];
                if prev != cur {
                    let kind = if prev == 0 && cur != 0 {
                        "ALLOC"
                    } else if prev != 0 && cur == 0 {
                        "FREE"
                    } else {
                        "MUTATE"
                    };
                    debug_log(&format!(
                        "[SUBSYS-WATCH] frame={} addr=0x{:08X} {} 0x{:08X} -> 0x{:08X}",
                        self.frame_count, addr, kind, prev, cur
                    ));
                    snap[i] = cur;
                }
            }

            // [ALLOC-WATCH] 2026-04-22: monitor the size-class allocator
            // metadata region 0x003F5A40..0x003F5AE0 (40 slots / 160B).
            // 0x003F5A48 is the documented "alloc-failed callback" at
            // the head of the retry loop at 0x16F50. If the bisection
            // JMP skipped the init path that seeds this table, we'll
            // see all-zero state forever. If something writes it,
            // we'll catch 0→nonzero transitions.
            const ALLOC_BASE: u32 = 0x003F_5A40;
            const ALLOC_SLOTS: usize = 40; // 160 bytes covers the hot region
            static ALLOC_SHADOW: std::sync::Mutex<Option<[u32; ALLOC_SLOTS]>> =
                std::sync::Mutex::new(None);
            let mut aguard = ALLOC_SHADOW.lock().unwrap_or_else(|e| e.into_inner());
            let asnap = aguard.get_or_insert([0u32; ALLOC_SLOTS]);
            for i in 0..ALLOC_SLOTS {
                let addr = ALLOC_BASE + (i as u32) * 4;
                let cur = mem.read_u32(addr);
                let prev = asnap[i];
                if prev != cur {
                    let kind = if prev == 0 && cur != 0 {
                        "ALLOC"
                    } else if prev != 0 && cur == 0 {
                        "FREE"
                    } else {
                        "MUTATE"
                    };
                    debug_log(&format!(
                        "[ALLOC-WATCH] frame={} addr=0x{:08X} {} 0x{:08X} -> 0x{:08X}",
                        self.frame_count, addr, kind, prev, cur
                    ));
                    asnap[i] = cur;
                }
            }

            // Optional one-off probes for specific guest addresses. Use a
            // comma/semicolon/space separated list, e.g.
            // RUSTEMU_ALLOC_WATCH_EXTRA=0x4B8F04,0x4B8F08,0x10249AE0.
            static EXTRA_ALLOC_SHADOW: std::sync::Mutex<Option<Vec<(u32, Option<u32>)>>> =
                std::sync::Mutex::new(None);
            let mut extra_guard = EXTRA_ALLOC_SHADOW.lock().unwrap_or_else(|e| e.into_inner());
            if extra_guard.is_none() {
                let entries = std::env::var("RUSTEMU_ALLOC_WATCH_EXTRA")
                    .ok()
                    .map(|raw| {
                        raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
                            .filter_map(|part| {
                                let s = part.trim();
                                if s.is_empty() {
                                    return None;
                                }
                                let s = s
                                    .strip_prefix("0x")
                                    .or_else(|| s.strip_prefix("0X"))
                                    .unwrap_or(s);
                                u32::from_str_radix(s, 16).ok().map(|addr| (addr, None))
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                *extra_guard = Some(entries);
            }
            if let Some(entries) = extra_guard.as_mut() {
                for (addr, prev) in entries.iter_mut() {
                    let cur = mem.read_u32(*addr);
                    let changed = prev.map_or(true, |p| p != cur);
                    if changed {
                        let kind = match *prev {
                            None => "INIT",
                            Some(0) if cur != 0 => "ALLOC",
                            Some(p) if p != 0 && cur == 0 => "FREE",
                            Some(_) => "MUTATE",
                        };
                        debug_log(&format!(
                            "[ALLOC-WATCH-EXTRA] frame={} addr=0x{:08X} {} {} -> 0x{:08X}",
                            self.frame_count,
                            *addr,
                            kind,
                            prev.map(|p| format!("0x{:08X}", p))
                                .unwrap_or_else(|| "unset".to_string()),
                            cur
                        ));
                        *prev = Some(cur);
                    }
                }
            }
        }

        // [GLOBALS-TICK] Periodic dump of engine subsystem pointers (every 300 frames
        // = ~5 sec at 60Hz). Tells us when (or if) the App/Scene/Engine constructors
        // populate their globals. If they stay 0 forever, engine bootstrap is wedged.
        //
        // 2026-04-21 corrections:
        //   - Renamed "Input" → "State" — 0x00726690 is the game's FSM/state-machine
        //     value (0=IDLE, 1=?, 2=ACTIVE), NOT an Input subsystem pointer. The
        //     disasm shows two writers: 0x000F751C writes 1, 0x000F75CD writes 2.
        //     No code path writes 0 — yet GLOBALS-TICK reads 0 from frame 300+, so
        //     either neither writer fires OR something we don't see (DB-missed
        //     dynamic dispatch?) zeros it.
        //   - Observation-only now: no state writes from the host. TAP hooks
        //     on the FSM/boot chain provide the reachability answer without
        //     changing the game state.
        // Spider-Man's post-title/menu FSM path already reaches the real
        // frame tick functions. Feed only the missing frame/render gates and
        // let the guest's own FSM/config fields run; App+0x104/App+0x108 are
        // demo-count inputs for sub_000F7450 and must not be host-forced.
        {
            use std::sync::atomic::{AtomicU32, Ordering};
            static CLOCK_BRIDGE_LOG: AtomicU32 = AtomicU32::new(0);
            const SPIDERMAN_ENTRY: u32 = 0x002A_9C38;

            let is_spiderman = self
                .xbe_info
                .as_ref()
                .map(|x| x.entry_point == SPIDERMAN_ENTRY)
                .unwrap_or(false);
            if is_spiderman && self.frame_count >= 600 {
                if let Some(ref mem) = self.memory {
                    fn valid_guest_ptr(addr: u32) -> bool {
                        (0x1000..0x2000_0000).contains(&addr)
                    }

                    fn read_scene_root(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                        scene: u32,
                    ) -> u32 {
                        if !valid_guest_ptr(scene) {
                            return 0;
                        }
                        let root = mem.read_u32(scene.wrapping_add(0x28));
                        if valid_guest_ptr(root) {
                            root
                        } else {
                            0
                        }
                    }

                    fn decode_scene_state_cursor(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                        a0: u32,
                    ) -> Option<(u32, u32, u32)> {
                        if (a0 & 3) != 0 || !valid_guest_ptr(a0) || a0 < 0x18 {
                            return None;
                        }
                        let arr = mem.read_u32(a0.wrapping_sub(0x14));
                        let idx = mem.read_u32(a0.wrapping_sub(0x10));
                        if !valid_guest_ptr(arr) || idx >= 64 {
                            return None;
                        }
                        let state_addr = arr.wrapping_add(idx.wrapping_mul(4));
                        if !valid_guest_ptr(state_addr) {
                            return None;
                        }
                        let state = mem.read_u32(state_addr);
                        if !(1..=6).contains(&state) {
                            return None;
                        }
                        Some((arr, idx, state))
                    }

                    fn read_scene_state_cursor(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                        scene: u32,
                    ) -> Option<(u32, u32, u32, u32)> {
                        if !valid_guest_ptr(scene) {
                            return None;
                        }
                        let a0 = mem.read_u32(scene.wrapping_add(0xA0));
                        let (arr, idx, state) = decode_scene_state_cursor(mem, a0)?;
                        Some((a0, arr, idx, state))
                    }

                    fn spidey_menu_resource_slots(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                    ) -> (u32, String) {
                        let mut active = 0u32;
                        let mut summary = String::new();
                        for slot in 1..=5u32 {
                            let base = 0x0073_7C18u32.wrapping_add(slot.wrapping_mul(0x2C));
                            let obj = mem.read_u32(base);
                            let enabled = mem.read_u8(base.wrapping_add(0x04));
                            let wait_flag = mem.read_u8(base.wrapping_add(0x05));
                            let busy_a = mem.read_u8(base.wrapping_add(0x08));
                            let busy_b = mem.read_u32(base.wrapping_add(0x0A));
                            let aux = mem.read_u32(base.wrapping_add(0x10));
                            if valid_guest_ptr(obj) && enabled != 0 {
                                active = active.saturating_add(1);
                            }
                            if slot <= 5 {
                                use std::fmt::Write as _;
                                let _ = write!(
                                    summary,
                                    " s{}:base=0x{:08X} obj=0x{:08X} en={} wait={} b8={} bA=0x{:08X} aux=0x{:08X}",
                                    slot, base, obj, enabled, wait_flag, busy_a, busy_b, aux
                                );
                            }
                        }
                        (active, summary)
                    }

                    fn raw_scene_state_cursor(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                        scene: u32,
                    ) -> (u32, u32, u32, u32) {
                        if !valid_guest_ptr(scene) {
                            return (0, 0, 0, 0);
                        }
                        let a0 = mem.read_u32(scene.wrapping_add(0xA0));
                        if (a0 & 3) != 0 || !valid_guest_ptr(a0) || a0 < 0x18 {
                            return (a0, 0, 0, 0);
                        }
                        let arr = mem.read_u32(a0.wrapping_sub(0x14));
                        let idx = mem.read_u32(a0.wrapping_sub(0x10));
                        let state = if valid_guest_ptr(arr) && idx < 64 {
                            mem.read_u32(arr.wrapping_add(idx.wrapping_mul(4)))
                        } else {
                            0
                        };
                        (a0, arr, idx, state)
                    }

                    fn read_spidey_scene_name(
                        mem: &crate::xbox::memory::guest_memory::GuestMemory,
                    ) -> String {
                        let mut scene_name_bytes = Vec::new();
                        for i in 0..64u32 {
                            let b = mem.read_u8(0x004B_C848u32.wrapping_add(i));
                            if b == 0 {
                                break;
                            }
                            if !(0x20..=0x7E).contains(&b) {
                                scene_name_bytes.clear();
                                break;
                            }
                            scene_name_bytes.push(b);
                        }
                        String::from_utf8_lossy(&scene_name_bytes).to_ascii_lowercase()
                    }

                    fn valid_spidey_scene_mgr(addr: u32) -> bool {
                        (0x0001_0000..0x2000_0000).contains(&addr)
                    }

                    fn spidey_writable_scene_18c(scene: u32) -> bool {
                        scene
                            .checked_add(0x18C)
                            .map(|addr| (0x0001_0000..0x2000_0000).contains(&addr))
                            .unwrap_or(false)
                    }

                    let app = mem.read_u32(0x003F_5EB0);
                    let frame = mem.read_u32(0x004B_C630);
                    let engine = mem.read_u32(0x004B_C614);
                    let mut global_scene = mem.read_u32(0x003F_5BEC);
                    let scene_mgr = if valid_guest_ptr(frame) {
                        mem.read_u32(frame.wrapping_add(0x18))
                    } else {
                        0
                    };

                    if valid_guest_ptr(app) {
                        let fsm_state = mem.read_u32(0x0072_6690);
                        let app_104 = mem.read_u32(app.wrapping_add(0x104));
                        let app_108 = mem.read_u32(app.wrapping_add(0x108));
                        let fsm_list = mem.read_u32(0x0072_6690 + 0x14);
                        let frame_20_before = if valid_guest_ptr(frame) {
                            mem.read_u32(frame.wrapping_add(0x20))
                        } else {
                            0
                        };
                        let scene_root = read_scene_root(mem, scene_mgr);
                        let mut global_scene_root = read_scene_root(mem, global_scene);
                        let scene_ready = scene_root != 0;
                        let force_global_scene_sync =
                            std::env::var_os("RUSTEMU_SPIDEY_FORCE_GLOBAL_SCENE_SYNC").is_some();
                        let force_root_focus_sync =
                            std::env::var_os("RUSTEMU_SPIDEY_FORCE_ROOT_FOCUS_SYNC").is_some();
                        let selector_seen_for_scene_sync =
                            crate::xbox::aot::oovpa::oovpa_hle::spidey_menu_selector_seen();
                        if force_global_scene_sync
                            && selector_seen_for_scene_sync
                            && scene_ready
                            && valid_guest_ptr(scene_mgr)
                            && global_scene != scene_mgr
                            && global_scene_root == 0
                        {
                            static GLOBAL_SCENE_SYNC_LOG: AtomicU32 = AtomicU32::new(0);
                            let sync_n = GLOBAL_SCENE_SYNC_LOG.fetch_add(1, Ordering::Relaxed);
                            let old_global_scene = global_scene;
                            let old_global_root_raw = if valid_guest_ptr(old_global_scene) {
                                mem.read_u32(old_global_scene.wrapping_add(0x28))
                            } else {
                                0
                            };
                            let scene_root_focus = if valid_guest_ptr(scene_root) {
                                mem.read_u32(scene_root.wrapping_add(0x1A8))
                            } else {
                                0
                            };
                            mem.write_u32(0x003F_5BEC, scene_mgr);
                            global_scene = scene_mgr;
                            global_scene_root = scene_root;
                            if sync_n < 16 || sync_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-GLOBAL-SCENE-SYNC] #{} frame={} old_scene=0x{:08X} old_root_raw=0x{:08X} new_scene=0x{:08X} new_root=0x{:08X} new_root+1A8=0x{:08X} scene+17F/183/186/18B/18C={}/{}/{}/{}/{} frame_ctx=0x{:08X} frame+20=0x{:08X}",
                                    sync_n,
                                    self.frame_count,
                                    old_global_scene,
                                    old_global_root_raw,
                                    scene_mgr,
                                    scene_root,
                                    scene_root_focus,
                                    mem.read_u8(scene_mgr.wrapping_add(0x17F)),
                                    mem.read_u8(scene_mgr.wrapping_add(0x183)),
                                    mem.read_u8(scene_mgr.wrapping_add(0x186)),
                                    mem.read_u8(scene_mgr.wrapping_add(0x18B)),
                                    mem.read_u8(scene_mgr.wrapping_add(0x18C)),
                                    frame,
                                    frame_20_before
                                ));
                            }
                        }
                        if force_global_scene_sync
                            && force_root_focus_sync
                            && scene_ready
                            && valid_guest_ptr(scene_root)
                        {
                            let root_focus = mem.read_u32(scene_root.wrapping_add(0x1A8));
                            let action_mgr = mem.read_u32(0x003F_7D90);
                            let action_active = if valid_guest_ptr(action_mgr) {
                                mem.read_u32(action_mgr.wrapping_add(0x58))
                            } else {
                                0
                            };
                            let action_slot1 = if valid_guest_ptr(action_mgr) {
                                mem.read_u32(action_mgr.wrapping_add(0x30))
                            } else {
                                0
                            };
                            let action_slot1_vt = if valid_guest_ptr(action_slot1) {
                                mem.read_u32(action_slot1)
                            } else {
                                0
                            };
                            if root_focus == 0
                                && valid_guest_ptr(action_mgr)
                                && action_active != 0
                                && valid_guest_ptr(action_slot1)
                                && action_slot1_vt == 0x0038_1E50
                            {
                                static ROOT_FOCUS_SYNC_LOG: AtomicU32 = AtomicU32::new(0);
                                let focus_n = ROOT_FOCUS_SYNC_LOG.fetch_add(1, Ordering::Relaxed);
                                mem.write_u32(scene_root.wrapping_add(0x1A8), action_mgr);
                                if focus_n < 16 || focus_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-ROOT-FOCUS-SYNC] #{} frame={} root=0x{:08X} old_focus=0x{:08X} new_focus=0x{:08X} action+58=0x{:08X} slot1=0x{:08X} slot1_vt=0x{:08X} scene=0x{:08X} scene+17F/183/186/18B/18C={}/{}/{}/{}/{}",
                                        focus_n,
                                        self.frame_count,
                                        scene_root,
                                        root_focus,
                                        action_mgr,
                                        action_active,
                                        action_slot1,
                                        action_slot1_vt,
                                        scene_mgr,
                                        mem.read_u8(scene_mgr.wrapping_add(0x17F)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x183)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x186)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x18B)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x18C))
                                    ));
                                }
                            }
                        }
                        if scene_ready && valid_guest_ptr(scene_mgr) {
                            static LAST_A0_SCENE: AtomicU32 = AtomicU32::new(0);
                            static LAST_GOOD_A0: AtomicU32 = AtomicU32::new(0);
                            static SCENE_A0_LOG: AtomicU32 = AtomicU32::new(0);
                            static SCENE_A0_REPAIR_LOG: AtomicU32 = AtomicU32::new(0);
                            if let Some((a0, arr, idx, state)) =
                                read_scene_state_cursor(mem, scene_mgr)
                            {
                                let prev_scene = LAST_A0_SCENE.load(Ordering::Relaxed);
                                if prev_scene != scene_mgr {
                                    LAST_A0_SCENE.store(scene_mgr, Ordering::Relaxed);
                                    let log_n = SCENE_A0_LOG.fetch_add(1, Ordering::Relaxed);
                                    if log_n < 16 || log_n.is_power_of_two() {
                                        debug_log(&format!(
                                            "[SPIDEY-SCENE-A0-TRACK] #{} frame={} scene=0x{:08X} a0=0x{:08X} arr=0x{:08X} idx={} state={} previous_scene=0x{:08X}",
                                            log_n,
                                            self.frame_count,
                                            scene_mgr,
                                            a0,
                                            arr,
                                            idx,
                                            state,
                                            prev_scene
                                        ));
                                    }
                                }
                                LAST_GOOD_A0.store(a0, Ordering::Relaxed);
                            } else {
                                let last_scene = LAST_A0_SCENE.load(Ordering::Relaxed);
                                let last_a0 = LAST_GOOD_A0.load(Ordering::Relaxed);
                                if last_scene == scene_mgr
                                    && last_a0 != 0
                                    && decode_scene_state_cursor(mem, last_a0).is_some()
                                {
                                    let (bad_a0, bad_arr, bad_idx, bad_state) =
                                        raw_scene_state_cursor(mem, scene_mgr);
                                    if bad_a0 != last_a0 {
                                        let (good_arr, good_idx, good_state) =
                                            decode_scene_state_cursor(mem, last_a0)
                                                .unwrap_or((0, 0, 0));
                                        mem.write_u32(scene_mgr.wrapping_add(0xA0), last_a0);
                                        let repair_n =
                                            SCENE_A0_REPAIR_LOG.fetch_add(1, Ordering::Relaxed);
                                        if repair_n < 32 || repair_n.is_power_of_two() {
                                            debug_log(&format!(
                                                "[SPIDEY-SCENE-A0-REPAIR] #{} frame={} scene=0x{:08X} bad_a0=0x{:08X} bad_arr=0x{:08X} bad_idx=0x{:08X} bad_state=0x{:08X} restored_a0=0x{:08X} good_arr=0x{:08X} good_idx={} good_state={} scene+17F/183/186/18B/18C={}/{}/{}/{}/{}",
                                                repair_n,
                                                self.frame_count,
                                                scene_mgr,
                                                bad_a0,
                                                bad_arr,
                                                bad_idx,
                                                bad_state,
                                                last_a0,
                                                good_arr,
                                                good_idx,
                                                good_state,
                                                mem.read_u8(scene_mgr.wrapping_add(0x17F)),
                                                mem.read_u8(scene_mgr.wrapping_add(0x183)),
                                                mem.read_u8(scene_mgr.wrapping_add(0x186)),
                                                mem.read_u8(scene_mgr.wrapping_add(0x18B)),
                                                mem.read_u8(scene_mgr.wrapping_add(0x18C))
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        if scene_ready
                            && valid_guest_ptr(scene_mgr)
                            && global_scene == scene_mgr
                            && global_scene_root == scene_root
                            && frame_20_before == 0x0A
                        {
                            let scene_name = read_spidey_scene_name(mem);
                            let state_cursor = read_scene_state_cursor(mem, scene_mgr);
                            let (state_a0, state_arr, state_idx, state) =
                                state_cursor.unwrap_or((0, 0, 0, 0));
                            let init_done = mem.read_u8(scene_mgr.wrapping_add(0x25));
                            let render_enable = mem.read_u8(scene_mgr.wrapping_add(0x186));
                            let scene_18c = mem.read_u8(scene_mgr.wrapping_add(0x18C));
                            let peterstu_ready_seq = spidey_peterstu_read_ready_seq();
                            let spidey_fast_originz = spidey_synth_training_enabled()
                                || std::env::var_os("RUSTEMU_SPIDEY_FAST_ORIGINZ").is_some();
                            let force_menu_complete = spidey_fast_originz
                                || std::env::var_os("RUSTEMU_SPIDEY_FORCE_MENU_COMPLETE").is_some();
                            let menu_complete_candidate = should_force_spidey_menu_complete(
                                &scene_name,
                                state,
                                render_enable,
                                scene_18c,
                                peterstu_ready_seq,
                            );
                            if menu_complete_candidate && force_menu_complete {
                                static MENU_COMPLETE_FORCE_LOG: AtomicU32 = AtomicU32::new(0);
                                let force_n =
                                    MENU_COMPLETE_FORCE_LOG.fetch_add(1, Ordering::Relaxed);
                                mem.write_u8(scene_mgr.wrapping_add(0x18C), 1);
                                if force_n < 16 || force_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-FORCE-MENU-COMPLETE-FIRE] #{} frame={} scene=0x{:08X} root=0x{:08X} scene_name='{}' state={} render_enable={} 18c {}->1 peterstu_ready_seq={} state_a0=0x{:08X} state_arr=0x{:08X} state_idx={} global_scene=0x{:08X} global_root=0x{:08X}",
                                        force_n,
                                        self.frame_count,
                                        scene_mgr,
                                        scene_root,
                                        scene_name,
                                        state,
                                        render_enable,
                                        scene_18c,
                                        peterstu_ready_seq,
                                        state_a0,
                                        state_arr,
                                        state_idx,
                                        global_scene,
                                        global_scene_root
                                    ));
                                }
                            } else if menu_complete_candidate {
                                static MENU_COMPLETE_CANDIDATE_LOG: AtomicU32 = AtomicU32::new(0);
                                let obs_n =
                                    MENU_COMPLETE_CANDIDATE_LOG.fetch_add(1, Ordering::Relaxed);
                                if obs_n < 16 || obs_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-FORCE-MENU-COMPLETE-CANDIDATE] #{} frame={} scene=0x{:08X} root=0x{:08X} scene_name='{}' state={} render_enable={} 18c={} peterstu_ready_seq={} force_env=0 state_a0=0x{:08X} state_arr=0x{:08X} state_idx={}",
                                        obs_n,
                                        self.frame_count,
                                        scene_mgr,
                                        scene_root,
                                        scene_name,
                                        state,
                                        render_enable,
                                        scene_18c,
                                        peterstu_ready_seq,
                                        state_a0,
                                        state_arr,
                                        state_idx
                                    ));
                                }
                            }
                            let force_menu_init_done = spidey_fast_originz
                                || std::env::var_os("RUSTEMU_SPIDEY_FORCE_MENU_INIT_DONE")
                                    .is_some();
                            if scene_name == "bonus\\menu"
                                && state == 1
                                && valid_guest_ptr(state_arr)
                                && state_idx < 64
                                && render_enable == 0
                                && force_menu_init_done
                            {
                                static MENU_INIT_DONE_SYNC_LOG: AtomicU32 = AtomicU32::new(0);
                                let sync_n =
                                    MENU_INIT_DONE_SYNC_LOG.fetch_add(1, Ordering::Relaxed);
                                let old_183 = mem.read_u8(scene_mgr.wrapping_add(0x183));
                                let old_185 = mem.read_u8(scene_mgr.wrapping_add(0x185));
                                let old_186 = mem.read_u8(scene_mgr.wrapping_add(0x186));
                                mem.write_u8(scene_mgr.wrapping_add(0x183), 1);
                                mem.write_u8(scene_mgr.wrapping_add(0x185), 0);
                                mem.write_u8(scene_mgr.wrapping_add(0x186), 1);
                                mem.write_u8(scene_mgr.wrapping_add(0x25), 0);
                                mem.write_u32(state_arr.wrapping_add(state_idx.wrapping_mul(4)), 2);
                                if sync_n < 16 || sync_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-MENU-INIT-DONE-SYNC] #{} frame={} scene=0x{:08X} root=0x{:08X} state_a0=0x{:08X} state_arr=0x{:08X} state_idx={} state {}->2 old25=0x{:02X}->0 scene+183/185/186 {}/{}/{} -> 1/0/1 scene+17F/18B/18C={}/{}/{} global_scene=0x{:08X} global_root=0x{:08X} frame_ctx=0x{:08X} frame+20=0x{:08X}",
                                        sync_n,
                                        self.frame_count,
                                        scene_mgr,
                                        scene_root,
                                        state_a0,
                                        state_arr,
                                        state_idx,
                                        state,
                                        init_done,
                                        old_183,
                                        old_185,
                                        old_186,
                                        mem.read_u8(scene_mgr.wrapping_add(0x17F)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x18B)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x18C)),
                                        global_scene,
                                        global_scene_root,
                                        frame,
                                        frame_20_before
                                    ));
                                }
                            } else if scene_name == "bonus\\menu"
                                && state == 1
                                && render_enable == 0
                            {
                                static MENU_INIT_OBS_LOG: AtomicU32 = AtomicU32::new(0);
                                let obs_n = MENU_INIT_OBS_LOG.fetch_add(1, Ordering::Relaxed);
                                if obs_n < 8 || obs_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-MENU-INIT-DONE-OBS] #{} frame={} scene=0x{:08X} root=0x{:08X} state_a0=0x{:08X} state_arr=0x{:08X} state_idx={} state={} old25=0x{:02X} scene+183/185/186={}/{}/{} force_env=0",
                                        obs_n,
                                        self.frame_count,
                                        scene_mgr,
                                        scene_root,
                                        state_a0,
                                        state_arr,
                                        state_idx,
                                        state,
                                        init_done,
                                        mem.read_u8(scene_mgr.wrapping_add(0x183)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x185)),
                                        mem.read_u8(scene_mgr.wrapping_add(0x186))
                                    ));
                                }
                            }
                            if scene_name == "bonus\\menu" && state >= 2 && render_enable != 0 {
                                static MENU_RESOURCE_WAKE_LOG: AtomicU32 = AtomicU32::new(0);
                                let queued = mem.read_u32(0x007D_3D60);
                                let registered = mem.read_u32(0x007D_3D68);
                                let (active_slots, slot_summary) = spidey_menu_resource_slots(mem);
                                let wake_n = MENU_RESOURCE_WAKE_LOG.fetch_add(1, Ordering::Relaxed);
                                if wake_n < 16 || wake_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-MENU-RESOURCE-OBS] #{} frame={} scene=0x{:08X} state={} queued={} active_slots={} registered={}{}",
                                        wake_n,
                                        self.frame_count,
                                        scene_mgr,
                                        state,
                                        queued,
                                        active_slots,
                                        registered,
                                        slot_summary
                                    ));
                                }
                            }
                        }
                        let n = CLOCK_BRIDGE_LOG.fetch_add(1, Ordering::Relaxed);
                        let peterstu_ready_seq = spidey_peterstu_read_ready_seq();
                        if peterstu_ready_seq != 0 {
                            static MENU_COMPLETE_MISS_LOG: AtomicU32 = AtomicU32::new(0);
                            let miss_n = MENU_COMPLETE_MISS_LOG.fetch_add(1, Ordering::Relaxed);
                            if miss_n < 16 || miss_n.is_power_of_two() {
                                let valid_scene = valid_guest_ptr(scene_mgr);
                                let scene_state = if valid_scene {
                                    read_scene_state_cursor(mem, scene_mgr)
                                        .map(|(_, _, _, state)| state)
                                        .unwrap_or(0xDEAD_BEEF)
                                } else {
                                    0xDEAD_BEEF
                                };
                                let scene_181 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x181))
                                } else {
                                    0
                                };
                                let scene_182 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x182))
                                } else {
                                    0
                                };
                                let scene_184 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x184))
                                } else {
                                    0
                                };
                                let scene_185 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x185))
                                } else {
                                    0
                                };
                                let scene_186 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x186))
                                } else {
                                    0
                                };
                                let scene_188 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x188))
                                } else {
                                    0
                                };
                                let scene_189 = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x189))
                                } else {
                                    0
                                };
                                let scene_18b = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x18B))
                                } else {
                                    0
                                };
                                let scene_18c = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x18C))
                                } else {
                                    0
                                };
                                let scene_18e = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x18E))
                                } else {
                                    0
                                };
                                let scene_18f = if valid_scene {
                                    mem.read_u8(scene_mgr.wrapping_add(0x18F))
                                } else {
                                    0
                                };
                                // DAT_004bc614/DAT_003f5bec is the stable game scene object.
                                // It is not expected to equal the frame scene manager loaded
                                // for a level, so do not classify that difference as a blocker.
                                let outer_ready = scene_ready && valid_scene;
                                let miss_reason = if !scene_ready {
                                    "scene_not_ready"
                                } else if !valid_scene {
                                    "invalid_scene_mgr"
                                } else if scene_state != 3 {
                                    "state_not_3"
                                } else if scene_186 == 0 {
                                    "render_not_enabled"
                                } else if scene_18c != 0 {
                                    "18c_already_set"
                                } else {
                                    "candidate"
                                };
                                debug_log(&format!(
                                    "[SPIDEY-SCENE-OBSERVATION] #{} frame={} reason={} outer_ready={} force_env={} peterstu_ready_seq={} scene_mgr=0x{:08X} root=0x{:08X} state=0x{:08X} flags181/182/184/185/186/188/189/18b/18c/18e/18f={:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X} stable_scene=0x{:08X} stable_root=0x{:08X} stable_differs={} frame+20=0x{:08X}",
                                    miss_n,
                                    self.frame_count,
                                    miss_reason,
                                    outer_ready,
                                    std::env::var_os("RUSTEMU_SPIDEY_FORCE_MENU_COMPLETE").is_some(),
                                    peterstu_ready_seq,
                                    scene_mgr,
                                    scene_root,
                                    scene_state,
                                    scene_181,
                                    scene_182,
                                    scene_184,
                                    scene_185,
                                    scene_186,
                                    scene_188,
                                    scene_189,
                                    scene_18b,
                                    scene_18c,
                                    scene_18e,
                                    scene_18f,
                                    global_scene,
                                    global_scene_root,
                                    global_scene != scene_mgr,
                                    frame_20_before
                                ));
                            }
                        }
                        if spidey_synth_training_enabled() {
                            static SYNTH_STAGE_LOG: AtomicU32 = AtomicU32::new(0);
                            static SYNTH_SKIP_LOG: AtomicU32 = AtomicU32::new(0);
                            static SYNTH_STATE_TRANSITION_LOG: AtomicBool = AtomicBool::new(false);

                            let vshader_key_seq = spidey_post_peterstu_vshader_key_seq();
                            let scene_name = read_spidey_scene_name(mem);
                            let valid_scene_mgr =
                                scene_mgr != 0 && valid_spidey_scene_mgr(scene_mgr);
                            let writable_18c =
                                valid_scene_mgr && spidey_writable_scene_18c(scene_mgr);
                            let state_cursor = if valid_scene_mgr {
                                read_scene_state_cursor(mem, scene_mgr)
                            } else {
                                None
                            };
                            let (state_a0, state_arr, state_idx, scene_state) =
                                state_cursor.unwrap_or((0, 0, 0, 0xDEAD_BEEF));
                            let render_enable = if valid_scene_mgr {
                                mem.read_u8(scene_mgr.wrapping_add(0x186))
                            } else {
                                0
                            };
                            let scene_18c = if valid_scene_mgr {
                                mem.read_u8(scene_mgr.wrapping_add(0x18C))
                            } else {
                                0
                            };
                            let action_mgr = mem.read_u32(0x003F_7D90);
                            let action_slot0 = if valid_guest_ptr(action_mgr) {
                                mem.read_u32(action_mgr.wrapping_add(0x2C))
                            } else {
                                0
                            };
                            let action_slot1 = if valid_guest_ptr(action_mgr) {
                                mem.read_u32(action_mgr.wrapping_add(0x30))
                            } else {
                                0
                            };
                            let action_active = if valid_guest_ptr(action_mgr) {
                                mem.read_u32(action_mgr.wrapping_add(0x58))
                            } else {
                                0
                            };
                            let player_ptr_guess = if valid_guest_ptr(action_slot1) {
                                action_slot1
                            } else if valid_guest_ptr(action_slot0) {
                                action_slot0
                            } else {
                                0
                            };
                            let stage_n = SYNTH_STAGE_LOG.fetch_add(1, Ordering::Relaxed);
                            if stage_n < 8 || stage_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-TRAINING-STAGE] #{} frame={} scene='{}' scene_mgr=0x{:08X} valid_scene_mgr={} writable_18c={} state=0x{:08X} scene+186=0x{:02X} scene+18c=0x{:02X} peterstu_ready_seq={} post_peterstu_vshader_key_seq={} handoffs={} timeout={} passthrough={} player_ptr_guess=0x{:08X}",
                                    stage_n,
                                    self.frame_count,
                                    scene_name,
                                    scene_mgr,
                                    valid_scene_mgr,
                                    writable_18c,
                                    scene_state,
                                    render_enable,
                                    scene_18c,
                                    peterstu_ready_seq,
                                    vshader_key_seq,
                                    spidey_synth_handoff_count(),
                                    spidey_synth_handoff_timed_out(),
                                    spidey_synth_training_input_passthrough(),
                                    player_ptr_guess
                                ));
                            }

                            let ready_assets = peterstu_ready_seq != 0 && vshader_key_seq != 0;
                            let scene_is_origin_z = scene_name == "levels\\origin_z";
                            let action_object_ready = scene_is_origin_z
                                && valid_scene_mgr
                                && scene_state == 3
                                && render_enable == 1
                                && valid_guest_ptr(player_ptr_guess);
                            if action_object_ready {
                                spidey_log_action_object_dump(
                                    mem,
                                    self.frame_count,
                                    &scene_name,
                                    scene_mgr,
                                    scene_state,
                                    action_mgr,
                                    action_active,
                                    action_slot0,
                                    action_slot1,
                                    player_ptr_guess,
                                );
                            }
                            if scene_is_origin_z
                                && valid_scene_mgr
                                && scene_state == 3
                                && render_enable == 1
                            {
                                spidey_log_render_scratch_dump(
                                    mem,
                                    self.frame_count,
                                    &scene_name,
                                    scene_mgr,
                                    scene_state,
                                    render_enable,
                                    mem.read_u32(0x005F_2C58),
                                );
                            }
                            let can_handoff = ready_assets
                                && scene_is_origin_z
                                && valid_scene_mgr
                                && writable_18c
                                && scene_state == 3
                                && render_enable == 1
                                && scene_18c == 0
                                && spidey_synth_handoff_count() == 0
                                && !spidey_synth_handoff_timed_out()
                                && !spidey_synth_training_input_passthrough();

                            if can_handoff {
                                let force_18c_handoff =
                                    std::env::var_os("RUSTEMU_SPIDEY_SYNTH_FORCE_18C").is_some();
                                let handoff_n =
                                    record_spidey_synth_handoff(self.frame_count, scene_state);
                                if force_18c_handoff {
                                    mem.write_u8(scene_mgr.wrapping_add(0x18C), 1);
                                }
                                let (
                                    post_draws,
                                    post_skin_draws,
                                    update_ticks,
                                    initial_state,
                                    first_frame,
                                    streak,
                                    update_hook_entries,
                                    post_d3d11_draws,
                                ) = spidey_synth_branch_snapshot();
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-SCENE-HANDOFF] #{} frame={} first_handoff_frame={} scene_mgr=0x{:08X} scene='{}' state=0x{:08X} scene+186=0x{:02X} scene+18c=0x{:02X} force18c={} state_a0=0x{:08X} state_arr=0x{:08X} state_idx={} peterstu_ready_seq={} post_peterstu_vshader_key_seq={} player_ptr_guess=0x{:08X} action_mgr=0x{:08X} action_slot0=0x{:08X} action_slot1=0x{:08X} post_draws={} post_skin_draws={} post_d3d11_draws={} update_ticks={} update_hook_entries={} initial_state=0x{:08X} streak={}",
                                    handoff_n,
                                    self.frame_count,
                                    first_frame,
                                    scene_mgr,
                                    scene_name,
                                    scene_state,
                                    render_enable,
                                    scene_18c,
                                    force_18c_handoff,
                                    state_a0,
                                    state_arr,
                                    state_idx,
                                    peterstu_ready_seq,
                                    vshader_key_seq,
                                    player_ptr_guess,
                                    action_mgr,
                                    action_slot0,
                                    action_slot1,
                                    post_draws,
                                    post_skin_draws,
                                    post_d3d11_draws,
                                    update_ticks,
                                    update_hook_entries,
                                    initial_state,
                                    streak
                                ));
                            } else if ready_assets
                                && !spidey_synth_handoff_timed_out()
                                && !spidey_synth_training_input_passthrough()
                            {
                                let skip_n = SYNTH_SKIP_LOG.fetch_add(1, Ordering::Relaxed);
                                if skip_n < 16 || skip_n.is_power_of_two() {
                                    let reason = if !scene_is_origin_z {
                                        "scene_not_origin_z"
                                    } else if !valid_scene_mgr {
                                        "invalid_scene_mgr"
                                    } else if !writable_18c {
                                        "scene_18c_not_writable"
                                    } else if scene_state != 3 {
                                        "state_not_3"
                                    } else if render_enable != 1 {
                                        "scene_186_not_1"
                                    } else if scene_18c != 0 {
                                        "scene_18c_not_0"
                                    } else if spidey_synth_handoff_count() != 0 {
                                        "handoff_already_fired"
                                    } else {
                                        "unknown"
                                    };
                                    debug_log(&format!(
                                        "[SPIDEY-SYNTH-HANDOFF-SKIP] #{} frame={} reason={} scene='{}' scene_mgr=0x{:08X} valid_scene_mgr={} writable_18c={} state=0x{:08X} scene+186=0x{:02X} scene+18c=0x{:02X} handoffs={} peterstu_ready_seq={} post_peterstu_vshader_key_seq={} player_ptr_guess=0x{:08X}",
                                        skip_n,
                                        self.frame_count,
                                        reason,
                                        scene_name,
                                        scene_mgr,
                                        valid_scene_mgr,
                                        writable_18c,
                                        scene_state,
                                        render_enable,
                                        scene_18c,
                                        spidey_synth_handoff_count(),
                                        peterstu_ready_seq,
                                        vshader_key_seq,
                                        player_ptr_guess
                                    ));
                                }
                            }

                            let first_handoff_frame = spidey_synth_branch_snapshot().4;
                            if first_handoff_frame != 0
                                && scene_state != 3
                                && !SYNTH_STATE_TRANSITION_LOG.swap(true, Ordering::Relaxed)
                            {
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-SCENE-STATE-TRANSITION] frame={} first_handoff_frame={} scene_mgr=0x{:08X} scene='{}' state_now=0x{:08X} scene+186=0x{:02X} scene+18c=0x{:02X}",
                                    self.frame_count,
                                    first_handoff_frame,
                                    scene_mgr,
                                    scene_name,
                                    scene_state,
                                    render_enable,
                                    scene_18c
                                ));
                            }

                            if spidey_synth_timeout_due(self.frame_count)
                                && mark_spidey_synth_timeout_logged()
                            {
                                let (
                                    post_draws,
                                    post_skin_draws,
                                    update_ticks,
                                    initial_state,
                                    first_handoff_frame,
                                    streak,
                                    update_hook_entries,
                                    post_d3d11_draws,
                                ) = spidey_synth_branch_snapshot();
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-HANDOFF-TIMEOUT] frame={} first_handoff_frame={} elapsed_frames={} scene='{}' scene_mgr=0x{:08X} state_initial=0x{:08X} state_now=0x{:08X} scene+186=0x{:02X} scene+18c=0x{:02X} player_ptr_guess=0x{:08X} player_ptr_valid={} action_mgr=0x{:08X} action_slot0=0x{:08X} action_slot1=0x{:08X} update_ticks={} update_hook_entries={} post_handoff_draws={} post_handoff_skinned_draws={} post_handoff_d3d11_draws={} skin_streak={} branch_hint='{}'",
                                    self.frame_count,
                                    first_handoff_frame,
                                    self.frame_count.saturating_sub(first_handoff_frame),
                                    scene_name,
                                    scene_mgr,
                                    initial_state,
                                    scene_state,
                                    render_enable,
                                    scene_18c,
                                    player_ptr_guess,
                                    valid_guest_ptr(player_ptr_guess),
                                    action_mgr,
                                    action_slot0,
                                    action_slot1,
                                    update_ticks,
                                    update_hook_entries,
                                    post_draws,
                                    post_skin_draws,
                                    post_d3d11_draws,
                                    streak,
                                    if !valid_guest_ptr(player_ptr_guess) {
                                        "player_ptr_invalid_or_missing"
                                    } else if update_hook_entries == 0 && post_d3d11_draws != 0 {
                                        "update_hook_not_entered_gpu_primitives_continue"
                                    } else if update_hook_entries == 0 {
                                        "update_hook_not_entered"
                                    } else if post_draws == 0 && post_d3d11_draws == 0 {
                                        "no_post_handoff_gpu_draws"
                                    } else if post_draws == 0 {
                                        "gpu_primitives_continue_no_hle_drawidx"
                                    } else if post_skin_draws == 0 {
                                        "draws_continue_no_skinned_decls"
                                    } else {
                                        "skinned_draws_seen_no_success"
                                    }
                                ));
                            }
                        }
                        if n < 8 || n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-CLOCK-BRIDGE] #{} frame={} app=0x{:08X} fsm 0x{:08X}->0x{:08X} +104 0x{:08X}->0x{:08X} +108 0x{:08X}->0x{:08X} frame_ctx=0x{:08X} frame+20 0x{:08X}->0x{:08X} engine=0x{:08X} scene_mgr=0x{:08X} root=0x{:08X} ready={} scene+a0=0x{:08X} scene+25=0x{:02X} scene+18c=0x{:02X} peterstu_ready_seq={} fsm_list=0x{:08X} scene+186=0x{:02X} global_scene=0x{:08X} global_root=0x{:08X} global+a0=0x{:08X} global+25=0x{:02X} global+186=0x{:02X}",
                                n,
                                self.frame_count,
                                app,
                                fsm_state,
                                mem.read_u32(0x0072_6690),
                                app_104,
                                mem.read_u32(app.wrapping_add(0x104)),
                                app_108,
                                mem.read_u32(app.wrapping_add(0x108)),
                                frame,
                                frame_20_before,
                                if valid_guest_ptr(frame) {
                                    mem.read_u32(frame.wrapping_add(0x20))
                                } else {
                                    0
                                },
                                engine,
                                scene_mgr,
                                scene_root,
                                scene_ready,
                                if valid_guest_ptr(scene_mgr) {
                                    mem.read_u32(scene_mgr.wrapping_add(0xA0))
                                } else {
                                    0
                                },
                                if valid_guest_ptr(scene_mgr) {
                                    mem.read_u32(scene_mgr.wrapping_add(0x25)) & 0xFF
                                } else {
                                    0
                                },
                                if valid_guest_ptr(scene_mgr) {
                                    mem.read_u32(scene_mgr.wrapping_add(0x18C)) & 0xFF
                                } else {
                                    0
                                },
                                peterstu_ready_seq,
                                mem.read_u32(0x0072_6690 + 0x14),
                                if valid_guest_ptr(scene_mgr) {
                                    mem.read_u32(scene_mgr.wrapping_add(0x186)) & 0xFF
                                } else {
                                    0
                                },
                                global_scene,
                                global_scene_root,
                                if valid_guest_ptr(global_scene) {
                                    mem.read_u32(global_scene.wrapping_add(0xA0))
                                } else {
                                    0
                                },
                                if valid_guest_ptr(global_scene) {
                                    mem.read_u32(global_scene.wrapping_add(0x25)) & 0xFF
                                } else {
                                    0
                                },
                                if valid_guest_ptr(global_scene) {
                                    mem.read_u32(global_scene.wrapping_add(0x186)) & 0xFF
                                } else {
                                    0
                                }
                            ));
                        }
                    }
                }
            }
        }

        if self.frame_count > 0 && self.frame_count.is_multiple_of(300) {
            if let Some(ref mem) = self.memory {
                let g_app = mem.read_u32(0x003F5EB0);
                let g_scene = mem.read_u32(0x003F5BEC);
                let g_state_before = mem.read_u32(0x00726690);
                let g_engine = mem.read_u32(0x004BC614);
                let g_frame = mem.read_u32(0x004BC630);
                let mut sub_count = 0u32;
                for j in 0..102u32 {
                    if mem.read_u32(0x003DC550 + j * 4) != 0 {
                        sub_count += 1;
                    }
                }
                // 2026-04-24: Sample FSM at 0x00726690 every GLOBALS-TICK. Per
                // agent A04, sub_000F7450 writes FSM=1/2 at 0x000F751C/0x000F75CD.
                // If FSM_WRITE_TAPs fire but this still reads 0, our reader is wrong
                // OR something writes 0 back. If we see FSM=1 or 2 here, FSM is
                // advancing organically (just nothing renders the legal quad yet).
                let fsm_now = mem.read_u32(0x0072_6690);
                // Also read mirror — Xbox phys/virt aliasing means
                // [0x00726690] and [0x80726690] should be the same byte,
                // but verify because R15 sign-extension fixup might
                // route writes through the mirror differently.
                let fsm_mirror = mem.read_u32(0x8072_6690);
                let fsm_692c = mem.read_u32(0x0072_692C);
                let fsm_6694 = mem.read_u32(0x0072_6694);
                let scene_mgr_186 = if g_scene != 0 && g_scene < 0x2000_0000 {
                    mem.read_u32(g_scene + 0x186)
                } else {
                    0
                };
                debug_log(&format!(
                    "[GLOBALS-TICK] frame={} App=0x{:08X} Scene=0x{:08X} State=0x{:08X} Engine=0x{:08X} Frame=0x{:08X} SubFlags={}/102 FSM=0x{:08X} mirror=0x{:08X} +0x4=0x{:08X} +0xA0=0x{:08X} scene+0x186=0x{:08X}",
                    self.frame_count,
                    g_app, g_scene, g_state_before, g_engine, g_frame, sub_count,
                    fsm_now, fsm_mirror, fsm_6694, fsm_692c, scene_mgr_186
                ));
            }

            // [METHOD-HIST] Top-15 most-frequent NV2A method IDs the parser
            // has seen. Tells us what state Spider-Man is configuring while
            // it idles in the prepare-but-no-draw loop. Look for:
            //   - SetTransform / SetViewport methods → 2D ortho splash
            //   - Texture binds with addr=0 → asset wait
            //   - SetVertexShader / SetFVF → vertex pipeline ready, just needs DRAW
            //   - SetCombiner methods → pixel shader path active
            // If DRAW_ARRAYS (0x1810) or SET_BEGIN_END (0x17FC) appear with
            // non-zero count, the game IS drawing and the parser is just
            // miscounting NV2A_DRAW_METHODS — different bug.
            let top = crate::xbox::aot::nv2a_pb::dump_method_histogram(15);
            if !top.is_empty() {
                let mut s = String::with_capacity(256);
                for (mth, cnt) in &top {
                    if !s.is_empty() {
                        s.push(' ');
                    }
                    let name = crate::xbox::aot::nv2a_pb::method_name(*mth);
                    s.push_str(&format!("0x{:04X}({})x{}", mth, name, cnt));
                }
                debug_log(&format!(
                    "[METHOD-HIST] frame={} top15: {}",
                    self.frame_count, s
                ));
            }
        }

        // Reset ISR cooldown at frame boundary (VBlank tick = 60Hz from RetroArch)
        crate::xbox::aot::veh_dpc::reset_isr_cooldown();

        // Stuck detection for DPC injection (matching C++ v1 STUCK_THRESHOLD=12).
        // Only set DPC pending when worker dispatch count hasn't changed for 12
        // consecutive VBlanks (~200ms). This prevents DPC from firing mid-execution
        // during game_main init, which corrupts the stack via R14 modification.
        // C++ v1: AOT_Harness.cpp lines 1750-1805.
        if let Some(ref counters) = self.worker_live_counters {
            let current_dispatches = counters.dispatch_count.load(Ordering::Relaxed);
            let current_kernel = counters.kernel_calls.load(Ordering::Relaxed);
            let past_init = current_kernel >= 800;

            if past_init && crate::xbox::aot::veh_dpc::has_dpc() {
                if current_dispatches == self.stuck_last_dispatch_count
                    && current_kernel == self.stuck_last_kernel_calls
                {
                    self.stuck_vblank_count += 1;
                } else {
                    self.stuck_vblank_count = 0;
                    self.stuck_last_dispatch_count = current_dispatches;
                    self.stuck_last_kernel_calls = current_kernel;
                }

                // Only inject DPC when genuinely stuck (12 VBlanks = ~200ms)
                if self.stuck_vblank_count >= 12 {
                    crate::xbox::aot::veh_dpc::set_dpc_pending();
                    self.stuck_vblank_count = 0;
                    self.stuck_last_dispatch_count = current_dispatches;
                    self.stuck_last_kernel_calls = current_kernel;
                }
            }
        }

        // Create VBlank event on first frame (auto-reset, initially unsignaled)
        if VBLANK_EVENT.load(std::sync::atomic::Ordering::Relaxed) == 0 {
            use windows::Win32::System::Threading::CreateEventW;
            let h = unsafe { CreateEventW(None, false, false, None) };
            if let Ok(h) = h {
                VBLANK_EVENT.store(h.0 as u64, std::sync::atomic::Ordering::Relaxed);
                debug_log(&format!(
                    "[VBLANK] Event created: handle=0x{:X}",
                    h.0 as u64
                ));
            }
        }
        // Create worker-done event (manual-reset, initially unsignaled)
        if WORKER_DONE_EVENT.load(std::sync::atomic::Ordering::Relaxed) == 0 {
            use windows::Win32::System::Threading::CreateEventW;
            let h = unsafe { CreateEventW(None, true, false, None) }; // manual-reset
            if let Ok(h) = h {
                WORKER_DONE_EVENT.store(h.0 as u64, std::sync::atomic::Ordering::Relaxed);
                debug_log(&format!(
                    "[WORKER] Done event created: handle=0x{:X}",
                    h.0 as u64
                ));
            }
        }
        // Signal VBlank event — wakes any worker thread in KeDelayExecutionThread
        let vblank_h = VBLANK_EVENT.load(std::sync::atomic::Ordering::Relaxed);
        if vblank_h != 0 {
            use windows::Win32::Foundation::HANDLE;
            use windows::Win32::System::Threading::SetEvent;
            unsafe {
                SetEvent(HANDLE(vblank_h as *mut _)).ok();
            }
        }

        // Update kernel time exports from wall-clock elapsed time.
        // Xbox KeTickCount ticks every ~1ms. Tying it to retro_run() count
        // makes game time crawl whenever the core presents below 60 Hz; Doom's
        // title/menu timers exposed this by advancing only a few tics in a
        // minute. Keep these exports monotonic and real-time based instead.
        if let Some(ref memory) = self.memory {
            let elapsed_ms = self
                .timing_start
                .elapsed()
                .as_millis()
                .min(u32::MAX as u128) as u32;
            let tick_addr = crate::xbox::loader::xbe::KE_TICK_COUNT_ADDR
                .load(std::sync::atomic::Ordering::Relaxed);
            if tick_addr != 0 {
                let old = memory.read_u32(tick_addr);
                let tick = elapsed_ms.max(1).max(old);
                memory.write_u32(tick_addr, tick);
            }

            // Advance KeInterruptTime and KeSystemTime (8-byte KSYSTEM_TIME, 100ns units).
            // Doom's I_GetTime reads KeInterruptTime to compute game tics.
            let kernel_time =
                132_000_000_000_000_000u64.wrapping_add((elapsed_ms as u64).saturating_mul(10_000));
            for time_addr_ref in &[
                &crate::xbox::loader::xbe::KE_INTERRUPT_TIME_ADDR,
                &crate::xbox::loader::xbe::KE_SYSTEM_TIME_ADDR,
            ] {
                let time_addr = time_addr_ref.load(std::sync::atomic::Ordering::Relaxed);
                if time_addr != 0 {
                    memory.write_u32(time_addr, kernel_time as u32);
                    memory.write_u32(time_addr + 4, (kernel_time >> 32) as u32);
                }
            }
        }

        // Doom timing must flow through the real kernel time exports above.
        // Do not synthesize writes to game fields such as [gs+0x3968]:
        // 0x1B500 is a game-side getter for that field, not I_GetTime.
        // Main-thread writes here race the AOT game loop and can turn a
        // status/widget delta into a huge negative count.

        if let Some(ref memory) = self.memory {
            // Observe the frame counter at [frame_ctx+0x20]. The game decrements
            // this each frame; when it hits 0, the frame update enters render.
            let frame_ctx = memory.read_u32(0x004B_C630);
            if frame_ctx != 0 && frame_ctx < 0x1000_0000 {
                let old_val = memory.read_u32(frame_ctx + 0x20);
                static FRAME_DIAG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = FRAME_DIAG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 10 || (n % 300 == 0) {
                    debug_log(&format!(
                        "[FRAME-DIAG] frame={} [frame_ctx+0x20]={} (0=consumed, 1=untouched)",
                        n, old_val
                    ));
                }
            }
            // GPU progress: the game polls [0xFE820010] in a tight loop at
            // 0x00345780 waiting for (value & ~3) >= 0x20. This is a GPU
            // notification area. Write 0x1000 so the check passes.
            memory.write_u32(0xFE82_0010, 0x1000);

            // GPU progress: poke ALL known completion/status locations every frame.
            // The worker spins on various addresses waiting for GPU to signal "done".
            let dev_ptr = memory.read_u32(0x003038E0);
            if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                let put = memory.read_u32(dev_ptr + 0x30);
                memory.write_u32(dev_ptr + 0x34, put); // fence GET = PUT
                let pb_put = memory.read_u32(dev_ptr + 0x40);
                if pb_put != 0 {
                    memory.write_u32(dev_ptr + 0x44, pb_put);
                }
                // Keep the real SDK-facing completion fields moving, but do
                // not poke surface-table entries. dev+0x2074/0x207C are live
                // surface pointers in the XDK 4134 device layout; earlier
                // "notifier" scaffolding overwrote the depth/back-buffer
                // surface format and could hide valid legal/Bink output.
                // Poke NV2A shadow registers (read via VEH MMIO handler)
                let pfifo_put = crate::xbox::aot::nv2a::shadow_read(0x3240);
                crate::xbox::aot::nv2a::shadow_write(0x3244, pfifo_put); // GET = PUT

                // D3D busy/idle flags: set non-surface status fields to "idle".
                // device+0x08 is the device flags word (0x13 after CreateDevice),
                // not a completion flag; leave it under the guest/HLE D3D code.
                // device+0x2058: internal GPU busy state
                memory.write_u32(dev_ptr + 0x2058, 0);
            }
        }

        // ====================================================================
        // PROBE B · Uninit std::string scan (2026-04-20, docs/xbe_ctor_reference.md)
        // Fires ONCE at frame 1200 (20s in), AFTER the font-loader stall has
        // occurred. Scans Spider-Man's .data BSS region for the uninit string
        // signature: length=0, capacity=0xFFFFFFFF, buf_ptr=0 (or stale).
        // Also checks specific candidate addresses (callback pointers etc.)
        // documented in CLAUDE.md memory.
        // ====================================================================
        {
            static PROBE_B_FIRED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            // Fires at frame 120 (~2s) so we see state shortly after CRT init has
            // had a chance to run. If we miss at 120, retry at 300 and 600 in case
            // the game's CRT startup happens later.
            if (self.frame_count == 120 || self.frame_count == 300 || self.frame_count == 600)
                && !PROBE_B_FIRED.swap(true, Ordering::Relaxed)
            {
                if let Some(ref mem) = self.memory {
                    debug_log("[STRING-SCAN] === Probe B: uninit std::string scan ===");

                    // Candidate pointers from CLAUDE.md / prior investigation
                    let candidates = [
                        (
                            0x003F_5A48u32,
                            "null-callback spin target (OOM handler ptr)",
                        ),
                        (0x003F_10E8u32, "engine/render ctx global"),
                        (0x003F_1D40u32, "last_ret global"),
                        (0x007D_3D60u32, "main-thread spin counter"),
                        (
                            0x007E_1C4Cu32,
                            "__active_heap selector (expect 0=sys/1=sbh_user/3=sbh_crt)",
                        ),
                        (
                            0x007E_1584u32,
                            "_crtheap (CRT heap handle, NULL in sys mode is OK)",
                        ),
                        (
                            0x007E_18B0u32,
                            "process_heap (KEY! if NULL, _heap_init failed)",
                        ),
                        (0x007E_18B4u32, "process_heap+4 (heap struct first field)"),
                        (0x003F_7EBCu32, "sub_42580 alloc counter"),
                        (0x003F_7EB0u32, "sub_42580 metadata slot"),
                        (0x004B_C614u32, "engine_ptr global"),
                        (0x004B_C630u32, "frame_ctx global"),
                    ];
                    // Read the actual heap STRUCTURE at 0x04000000 (process_heap value).
                    // Real RtlCreateHeap fills first 64+ bytes with:
                    //   Size, Flags, Signature (0xEEFFEEFF), Magic, Segment list, etc.
                    // If all zero, RtlCreateHeap never wrote the struct —
                    // _heap_init ran partway and stopped after VirtualAlloc.
                    debug_log("[STRING-SCAN] Heap structure at [0x04000000] (first 48 bytes):");
                    for i in 0..12 {
                        let off = i * 4;
                        let v = mem.read_u32(0x0400_0000 + off);
                        debug_log(&format!(
                            "[STRING-SCAN]   [0x0400{:04X}] = 0x{:08X}",
                            off, v
                        ));
                    }
                    debug_log("[STRING-SCAN] Candidate global pointers:");
                    for (addr, name) in candidates.iter() {
                        let v = mem.read_u32(*addr);
                        debug_log(&format!(
                            "[STRING-SCAN]   [0x{:08X}] = 0x{:08X} {}  ({})",
                            addr,
                            v,
                            if v == 0 { "<-- NULL (uninit?)" } else { "" },
                            name
                        ));
                    }

                    // Scan .data BSS: VA=0x003DB000, vsize=0x406C64. We only
                    // need to scan the BSS part (past raw_size=0x1A6F0), so
                    // range is 0x003F56F0..0x007E1C64. That's ~4MB / 12-byte
                    // std::string = ~350K candidate slots, doable.
                    //
                    // Pattern: length=0, capacity=0xFFFFFFFF, buf_ptr=*.
                    // Stride: 4 bytes (32-bit aligned). Looking for the
                    // specific triple at N, N+4, N+8.
                    let scan_start = 0x003F_56F0u32;
                    let scan_end = 0x007E_1C64u32;
                    let mut hits: Vec<u32> = Vec::new();
                    let mut addr = scan_start;
                    while addr + 12 <= scan_end {
                        let len = mem.read_u32(addr);
                        let cap = mem.read_u32(addr + 4);
                        let buf = mem.read_u32(addr + 8);
                        if len == 0 && cap == 0xFFFF_FFFF && buf < 0x1000_0000 {
                            hits.push(addr);
                            if hits.len() >= 100 {
                                break;
                            }
                        }
                        addr += 4;
                    }
                    debug_log(&format!(
                        "[STRING-SCAN] BSS scan [0x{:08X}..0x{:08X}]: {} uninit std::string signatures found",
                        scan_start, scan_end, hits.len()
                    ));
                    for (i, h) in hits.iter().enumerate().take(20) {
                        let buf = mem.read_u32(*h + 8);
                        debug_log(&format!(
                            "[STRING-SCAN]   #{}: [0x{:08X}] len=0 cap=0xFFFFFFFF buf=0x{:08X}",
                            i, h, buf
                        ));
                    }

                    debug_log("[STRING-SCAN] === end Probe B ===");
                }
            }
        }

        // TRANSITION-STATS one-shots at frames 60, 120, 300, 600 — shows the
        // real totals regardless of whether WATCHDOG fires.
        {
            static STATS_FIRED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            if self.frame_count == 60
                || self.frame_count == 120
                || self.frame_count == 300
                || self.frame_count == 600
            {
                let _ = STATS_FIRED.fetch_add(1, Ordering::Relaxed);
                let (pushes, clean, mismatched) = crate::xbox::aot::transition::counters();
                debug_log(&format!(
                    "[TRANSITION-STATS] frame={} pushes={} verified_clean={} mismatched={} (Kernel+Hle+Manual boundaries)",
                    self.frame_count, pushes, clean, mismatched
                ));
            }
        }

        // WATCHDOG: log worker state every ~5 seconds (300 frames @ 60fps).
        if let Some(ref counters) = self.worker_live_counters {
            if self.frame_count % 300 == 0 {
                let kc = counters.kernel_calls.load(Ordering::Relaxed);
                let mmio = counters.mmio_count.load(Ordering::Relaxed);
                let pb = counters.pb_commands.load(Ordering::Relaxed);
                let dr = counters.draw_calls.load(Ordering::Relaxed);
                let alive = counters.alive.load(Ordering::Relaxed);
                // Read frame loop state from guest memory
                let (engine_ptr, quit_flag, frame_ctx) = if let Some(ref mem) = self.memory {
                    let ep = mem.read_u32(0x004B_C614);
                    let qf = if ep != 0
                        && (ep < 0x2000_0000 || (ep >= 0x8000_0000 && ep < 0xA000_0000))
                    {
                        mem.read_u8(ep + 0x184)
                    } else {
                        0xFF
                    };
                    let fc = mem.read_u32(0x004B_C630);
                    (ep, qf, fc)
                } else {
                    (0, 0xFF, 0)
                };
                let last_ret = if let Some(ref mem) = self.memory {
                    mem.read_u32(0x003F_1D40)
                } else {
                    0
                };
                // Callback lists at [0x3F10E8]+0x114 and +0x118
                let (cb_ptr, list1, list2) = if let Some(ref mem) = self.memory {
                    let p = mem.read_u32(0x003F_10E8);
                    if p != 0 && p < 0x1000_0000 {
                        (p, mem.read_u32(p + 0x114), mem.read_u32(p + 0x118))
                    } else {
                        (p, 0, 0)
                    }
                } else {
                    (0, 0, 0)
                };
                // Sample worker RIP via SuspendThread + GetThreadContext
                let (worker_rip, worker_regs) = {
                    let tid = WORKER_THREAD_ID.load(std::sync::atomic::Ordering::Relaxed);
                    if tid != 0 {
                        use windows::Win32::System::Diagnostics::Debug::*;
                        use windows::Win32::System::Threading::*;
                        unsafe {
                            let h =
                                OpenThread(THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT, false, tid);
                            if let Ok(h) = h {
                                SuspendThread(h);
                                let mut ctx: CONTEXT = std::mem::zeroed();
                                ctx.ContextFlags = CONTEXT_CONTROL_AMD64 | CONTEXT_INTEGER_AMD64;
                                let ok = GetThreadContext(h, &mut ctx);
                                ResumeThread(h);
                                windows::Win32::Foundation::CloseHandle(h).ok();
                                if ok.is_ok() {
                                    (
                                        ctx.Rip,
                                        Some((
                                            ctx.Rax, ctx.Rbx, ctx.Rcx, ctx.Rdx, ctx.Rsi, ctx.Rdi,
                                            ctx.R14, ctx.R15,
                                        )),
                                    )
                                } else {
                                    (0, None)
                                }
                            } else {
                                (0, None)
                            }
                        }
                    } else {
                        (0, None)
                    }
                };
                // Map host RIP to guest address via code_base
                let guest_pc = if worker_rip != 0 {
                    if let (Some(ctx_ref), Some(mem)) =
                        (self.aot_ctx.as_ref(), self.memory.as_ref())
                    {
                        let cb = ctx_ref.code_base as u64;
                        let cs = ctx_ref.code_size as u64;
                        if worker_rip >= cb && worker_rip < cb + cs {
                            let host_off = (worker_rip - cb) as u32;
                            ctx_ref.addr_hash.reverse_lookup(host_off)
                        } else {
                            0xFFFF_FFFF
                        } // outside code buffer
                    } else {
                        0
                    }
                } else {
                    0
                };
                // Read render context callback list pointers
                // Engine object + quit flag
                // Read inline RET probe + stack contents around worker ESP
                let (last_guest_ret, stack_top4) = if let Some(ref mem) = self.memory {
                    let probe = mem.read_u32(0x003F1D40);
                    // Read 4 dwords from current worker R14 area (0x00C1FEA8 based on last trace)
                    let s0 = mem.read_u32(0x00C1FEA0);
                    let s1 = mem.read_u32(0x00C1FEA4);
                    let s2 = mem.read_u32(0x00C1FEA8);
                    let s3 = mem.read_u32(0x00C1FEAC);
                    (
                        probe,
                        format!(
                            "stk[A0]={:08X} [A4]={:08X} [A8]={:08X} [AC]={:08X}",
                            s0, s1, s2, s3
                        ),
                    )
                } else {
                    (0, String::new())
                };
                let (engine_ptr, quit_flag_val, vblank_tick) = if let Some(ref mem) = self.memory {
                    let ep = mem.read_u32(0x004BC614);
                    let qf = if ep > 0
                        && (ep < 0x2000_0000 || (ep >= 0x8000_0000 && ep < 0xA000_0000))
                    {
                        mem.read_u8(ep + 0x184)
                    } else {
                        0xFF
                    };
                    let vb = mem.read_u8(0x003F1D48);
                    (ep, qf, vb)
                } else {
                    (0, 0, 0)
                };
                let (render_ctx_ptr, cb_list_114, cb_list_118) = if let Some(ref mem) = self.memory
                {
                    let rcp = mem.read_u32(0x003F10E8);
                    let c114 = if rcp > 0 && rcp < 0x1000_0000 {
                        mem.read_u32(rcp + 0x114)
                    } else {
                        0xDEAD
                    };
                    let c118 = if rcp > 0 && rcp < 0x1000_0000 {
                        mem.read_u32(rcp + 0x118)
                    } else {
                        0xDEAD
                    };
                    (rcp, c114, c118)
                } else {
                    (0, 0, 0)
                };
                // Read DIAG_ADDR for synthetic D3D test XBEs:
                // v1: [0]=SPD2, [1]=surface width, [2]=frame, [3]=fence, [4]=pending.
                // v02 adds [5..11]=PH01..PH07 phase markers.
                let (
                    diag_magic,
                    diag_surface_width,
                    diag_frame,
                    diag_fence,
                    diag_pending,
                    diag_phase1,
                    diag_phase2,
                    diag_phase3,
                    diag_phase4,
                    diag_phase5,
                    diag_phase6,
                    diag_phase7,
                ) = if let Some(ref mem) = self.memory {
                    (
                        mem.read_u32(0x101000),
                        mem.read_u32(0x101004),
                        mem.read_u32(0x101008),
                        mem.read_u32(0x10100C),
                        mem.read_u32(0x101010),
                        mem.read_u32(0x101014),
                        mem.read_u32(0x101018),
                        mem.read_u32(0x10101C),
                        mem.read_u32(0x101020),
                        mem.read_u32(0x101024),
                        mem.read_u32(0x101028),
                        mem.read_u32(0x10102C),
                    )
                } else {
                    (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
                };
                debug_log(&format!(
                    "[WATCHDOG] frame={} kernel={} kernel_calls={} probe=0x{:08X} engine=0x{:08X} quit={} {} worker_rip=0x{:X} guest_pc={} DIAG=[magic=0x{:08X} width={} frame={} fence={} pend={}] PH=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}]",
                    self.frame_count, kc, kc, last_guest_ret, engine_ptr, quit_flag_val, stack_top4,
                    worker_rip,
                    crate::xbox::logging::fmt_guest_addr(guest_pc),
                    diag_magic,
                    diag_surface_width,
                    diag_frame,
                    diag_fence,
                    diag_pending,
                    diag_phase1,
                    diag_phase2,
                    diag_phase3,
                    diag_phase4,
                    diag_phase5,
                    diag_phase6,
                    diag_phase7
                ));
                // Phase 3/4/5 matched-transition global counter dump.
                // Each worker thread has its own transition stack, but these
                // counters are process-wide. Shows REAL totals, not just
                // the logged-sample subset.
                {
                    let (pushes, clean, mismatched) = crate::xbox::aot::transition::counters();
                    debug_log(&format!(
                        "[TRANSITION-STATS] pushes={} verified_clean={} mismatched={} (across Kernel+Hle+Manual boundaries)",
                        pushes, clean, mismatched
                    ));
                }
                // Dump guest registers when stuck in known spin loops
                if let Some((rax, rbx, _rcx, _rdx, rsi, _rdi, r14, r15)) = worker_regs {
                    if guest_pc >= 0x002AAE30 && guest_pc <= 0x002AAE50 {
                        let ebx = rbx as u32;
                        let eax = rax as u32;
                        let ebp_guest = rsi as u32; // EBP might be in RSI from AOT
                        let r14_val = r14 as u32;
                        // Dump guest stack (return addresses) to trace call chain
                        if let Some(ref mem) = self.memory {
                            let mut stack_trace = String::new();
                            for i in 0..16u32 {
                                let addr = r14_val.wrapping_add(i * 4);
                                if (addr as u64) < 0x2000_0000 {
                                    let val = mem.read_u32(addr);
                                    if val >= 0x00011000 && val < 0x00400000 {
                                        // Looks like a code address
                                        stack_trace.push_str(&format!(" 0x{:08X}", val));
                                    }
                                }
                            }
                            debug_log(&format!(
                                "[SPIN-DIAG] EAX=0x{:08X} EBX=0x{:08X} ESI=0x{:08X} R14=0x{:08X} stack_addrs:{}",
                                eax, ebx, rsi as u32, r14_val, stack_trace
                            ));
                            // Also dump raw stack words
                            let mut raw = String::new();
                            for i in 0..12u32 {
                                let addr = r14_val.wrapping_add(i * 4);
                                if (addr as u64) < 0x2000_0000 {
                                    raw.push_str(&format!(" {:08X}", mem.read_u32(addr)));
                                }
                            }
                            debug_log(&format!("[SPIN-STACK] R14+0:{}", raw));
                        }
                    }
                }
            }
        }

        // Signal worker thread: VBlank tick (60Hz ISR injection).
        // Worker blocks on this after its initial run, then runs ISR each frame.
        if let Some(ref vblank) = self.vblank_signal {
            vblank.signal();
        }

        // 2026-04-22: frame_ctx watcher — Spider-Man title-screen gate.
        // [0x004BC630] is g_frameCtx (pointer set by sub_2A4A50 inside
        // game_main). If it ever becomes non-zero, we've passed the gate
        // even momentarily. If it never does, game_main's init chain
        // never reached the successful alloc+store at 0x002A4A87.
        if let Some(ref memory) = self.memory {
            static FRAME_CTX_SEEN: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let v = memory.read_u32(0x004B_C630);
            if v != 0 {
                let prev = FRAME_CTX_SEEN.swap(v, std::sync::atomic::Ordering::Relaxed);
                if prev != v {
                    debug_log(&format!(
                        "[FRAME-CTX-WATCH] [0x004BC630] became 0x{:08X} (was 0x{:08X})",
                        v, prev
                    ));
                }
            } else {
                let prev = FRAME_CTX_SEEN.swap(0, std::sync::atomic::Ordering::Relaxed);
                if prev != 0 {
                    debug_log(&format!(
                        "[FRAME-CTX-WATCH] [0x004BC630] CLEARED to 0 (was 0x{:08X})",
                        prev
                    ));
                }
            }
        }

        // Run AOT dispatch loop (up to N dispatches per frame)
        if let (Some(ctx), Some(xbe_info), Some(memory), Some(kstate)) = (
            self.aot_ctx.as_mut(),
            self.xbe_info.as_ref(),
            self.memory.as_ref(),
            self.kernel_state.as_mut(),
        ) {
            if ctx.code_base != std::ptr::null_mut() {
                // Set kernel state pointer for VEH inline dispatch
                ctx.kernel_state = kstate as *mut kernel::KernelState as *mut u8;

                if ctx.dispatch_count == 0 {
                    debug_log(&format!(
                        "AOT: first dispatch — entry=0x{:08X} esp=0x{:08X} code_base={:p} code_size={}",
                        xbe_info.entry_point, ctx.guest.esp, ctx.code_base, ctx.code_size
                    ));
                }

                let max_dispatches = 1000u32;
                // Determine next guest address:
                // - First frame: start from XBE entry point
                // - After RET_TO_ZERO: main thread is done (guest_addr=0)
                // - After error (0xDEADxxxx from VEH crash): treat as done
                // - Otherwise: resume from where we left off
                let saved_addr = ctx.guest.exit_guest_addr;
                let mut guest_addr = if ctx.dispatch_count == 0 {
                    xbe_info.entry_point // First frame: start from XBE entry
                } else if saved_addr == 0 || saved_addr >= 0xDEAD0000 {
                    0 // Main thread finished or errored — don't restart
                } else {
                    saved_addr
                };

                // Collect worker handles spawned this frame
                let mut new_workers: Vec<std::thread::JoinHandle<WorkerStats>> = Vec::new();
                let mut loop_exit_reason = "exhausted";

                for _ in 0..max_dispatches {
                    if guest_addr == 0 {
                        loop_exit_reason = "guest_addr=0";
                        break;
                    }

                    // Look up host offset for guest address
                    let host_offset = match ctx.addr_hash.lookup(guest_addr) {
                        Some(off) => off,
                        None => {
                            // Unresolvable guest address — stop dispatch
                            if ctx.dispatch_count < 50 {
                                debug_log(&format!(
                                    "AOT: unresolved guest addr 0x{:08X} — stopping dispatch",
                                    guest_addr
                                ));
                            }
                            loop_exit_reason = "unresolved_addr";
                            guest_addr = 0; // Don't restart from entry
                            break;
                        }
                    };

                    // Enter guest code
                    let mut entry_host = unsafe { ctx.code_base.add(host_offset as usize) };
                    ctx.guest.exit_reason = AOT_EXIT_RUNNING;

                    // NOTE: Do NOT reset shadow stack between dispatches (GPS approach).

                    if ctx.dispatch_count < 10 {
                        debug_log(&format!(
                            "AOT: entering guest#{} at 0x{:08X} host={:p} offset=0x{:X} esp=0x{:08X}",
                            ctx.dispatch_count + 1, guest_addr, entry_host, host_offset, ctx.guest.esp
                        ));
                    }

                    // Inner loop: enter guest → kernel calls in safe Rust → re-enter
                    loop {
                        let guest_prof = crate::xbox::profiler::start(
                            crate::xbox::profiler::CpuPhase::GuestExec,
                        );
                        let exit_reason =
                            unsafe { crate::xbox::aot::trampoline::enter_guest(ctx, entry_host) };
                        crate::xbox::profiler::finish(
                            crate::xbox::profiler::CpuPhase::GuestExec,
                            guest_prof,
                        );

                        if exit_reason != AOT_EXIT_KERNEL_CALL {
                            ctx.dispatch_count += 1;
                            if ctx.dispatch_count <= 50 {
                                debug_log(&format!(
                                    "AOT: dispatch#{} exit={} guest_addr=0x{:08X} esp=0x{:08X}",
                                    ctx.dispatch_count,
                                    exit_reason,
                                    ctx.guest.exit_guest_addr,
                                    ctx.guest.esp
                                ));
                            }
                            match exit_reason {
                                AOT_EXIT_RET_TO_ZERO => {
                                    debug_log(&format!(
                                        "MAIN: RET_TO_ZERO at dispatch#{} esp=0x{:08X} eax=0x{:08X}",
                                        ctx.dispatch_count, ctx.guest.esp, ctx.guest.eax
                                    ));
                                    // Main-thread continuity was tried (commit 03b2bce)
                                    // but game_main's host code faulted on the main
                                    // thread because main lacks the TLS/heap state the
                                    // worker established during its start_routine.
                                    // HEAP-CANARY fired at RIP in Windows DLL, VEH
                                    // passthrough skipped bytes, RET popped the
                                    // sentinel. Superseded by worker-side trigger —
                                    // see worker.rs for the game_main force path that
                                    // runs AFTER the worker's start_routine completes
                                    // (worker has proven state).
                                    guest_addr = 0;
                                }
                                AOT_EXIT_THREAD_EXIT => {
                                    debug_log("AOT: ThreadExit");
                                    guest_addr = 0;
                                }
                                AOT_EXIT_HALT => {
                                    loop_exit_reason = "kernel_halt";
                                    guest_addr = 0;
                                }
                                AOT_EXIT_SYSTEM_TRAP => {
                                    loop_exit_reason = "non_kernel_trap";
                                    guest_addr = 0;
                                }
                                AOT_EXIT_UNRESOLVED => {
                                    guest_addr = normalize_guest_addr(ctx.guest.exit_guest_addr);
                                    // Try rescue compilation first
                                    if guest_addr > 0
                                        && guest_addr < 0x2000_0000
                                        && ctx.addr_hash.lookup(guest_addr).is_none()
                                    {
                                        if let Some(_host_off) = ctx.rescue_emit(guest_addr) {
                                            continue; // re-enter dispatch loop with new code
                                        }
                                    }
                                    // If target is bogus (outside code sections), simulate RET
                                    if guest_addr > 0x0080_0000
                                        && ctx.addr_hash.lookup(guest_addr).is_none()
                                    {
                                        let ret_addr = memory.read_u32(ctx.guest.esp);
                                        ctx.guest.esp += 4;
                                        guest_addr = normalize_guest_addr(ret_addr);
                                        if ctx.dispatch_count <= 20 {
                                            debug_log(&format!(
                                                "AOT main: bogus target 0x{:08X}, RET -> 0x{:08X}",
                                                ctx.guest.exit_guest_addr, guest_addr
                                            ));
                                        }
                                    }
                                }
                                AOT_EXIT_ERROR => {
                                    debug_log(&format!(
                                        "AOT: error at 0x{:08X} dispatch={}",
                                        ctx.guest.exit_guest_addr, ctx.dispatch_count
                                    ));
                                    guest_addr = 0;
                                }
                                _ => {
                                    guest_addr = normalize_guest_addr(ctx.guest.exit_guest_addr);
                                }
                            }
                            break; // exit inner loop
                        }

                        // ---- AOT_EXIT_KERNEL_CALL: dispatch in safe Rust ----
                        let ordinal = ctx.kernel_ordinal;
                        let args = ctx.kernel_args;
                        let is_call = ctx.kernel_is_call;
                        let jmp_ret_addr = ctx.kernel_jmp_ret_addr;
                        let host_resume = ctx.kernel_host_resume;

                        let kstate_ptr = ctx.kernel_state as *mut crate::xbox::kernel::KernelState;
                        let kstate = unsafe { &mut *kstate_ptr };

                        let kr = crate::xbox::kernel::dispatch_inline(
                            ordinal,
                            &args,
                            &mut ctx.guest.eax,
                            &mut ctx.guest.edx,
                            &mut ctx.guest.ecx,
                            kstate,
                            ctx.guest_mem_base,
                        );

                        match kr {
                            crate::xbox::kernel::KernelResult::Handled
                            | crate::xbox::kernel::KernelResult::NotHandled => {
                                if is_call {
                                    entry_host = host_resume as *mut u8;
                                } else {
                                    let normalized = normalize_guest_addr(jmp_ret_addr);
                                    if jmp_ret_addr == 0 {
                                        guest_addr = 0;
                                        break;
                                    } else if let Some(ho) = ctx.addr_hash.lookup(normalized) {
                                        entry_host = unsafe { ctx.code_base.add(ho as usize) };
                                    } else {
                                        guest_addr = normalized;
                                        break;
                                    }
                                }
                            }
                            crate::xbox::kernel::KernelResult::SpawnThread {
                                entry: se,
                                start_routine: sr,
                                context: sc,
                                thread_handle: sh,
                            } => {
                                ctx.spawn_entry = se;
                                ctx.spawn_start_routine = sr;
                                ctx.spawn_context = sc;
                                ctx.spawn_thread_handle = sh;
                                debug_log(&format!(
                                    "MAIN: SpawnThread returned — is_call={} host_resume=0x{:X} jmp_ret=0x{:08X} esp=0x{:08X}",
                                    is_call, host_resume as u64, jmp_ret_addr, ctx.guest.esp
                                ));
                                if is_call {
                                    entry_host = host_resume as *mut u8;
                                } else {
                                    let normalized = normalize_guest_addr(jmp_ret_addr);
                                    debug_log(&format!(
                                        "MAIN: SpawnThread JMP path — normalized=0x{:08X}",
                                        normalized
                                    ));
                                    if let Some(ho) = ctx.addr_hash.lookup(normalized) {
                                        entry_host = unsafe { ctx.code_base.add(ho as usize) };
                                    } else {
                                        debug_log(&format!(
                                            "MAIN: SpawnThread JMP target 0x{:08X} UNRESOLVED — main thread stops",
                                            normalized
                                        ));
                                        guest_addr = normalized;
                                        break;
                                    }
                                }
                            }
                            crate::xbox::kernel::KernelResult::ThreadExit => {
                                guest_addr = 0;
                                break;
                            }
                            crate::xbox::kernel::KernelResult::Halt
                            | crate::xbox::kernel::KernelResult::QuickReboot => {
                                guest_addr = 0;
                                break;
                            }
                        }
                    }

                    // Check for pending worker spawn (VEH continues inline, defers spawn)
                    if ctx.spawn_entry != 0 {
                        let entry = ctx.spawn_entry;
                        let start_routine = ctx.spawn_start_routine;
                        let start_ctx = ctx.spawn_context;
                        let _thread_handle = ctx.spawn_thread_handle;
                        ctx.spawn_entry = 0;
                        ctx.spawn_start_routine = 0;
                        ctx.spawn_context = 0;
                        ctx.spawn_thread_handle = 0;
                        let is_first = self.worker_handles.is_empty() && new_workers.is_empty();
                        let thread_num = self.worker_handles.len() + new_workers.len();
                        let mut worker_ctx = Box::new(RuntimeContext::new_worker(ctx));
                        // Share main thread's KernelState (C++ uses global shared state)
                        let shared_kstate = ctx.kernel_state;
                        worker_ctx.kernel_state = shared_kstate;
                        let worker_mem = memory.clone_shared();
                        let live_counters = Arc::new(SharedWorkerCounters {
                            dispatch_count: AtomicU64::new(0),
                            mmio_count: AtomicU64::new(0),
                            pb_commands: AtomicU64::new(0),
                            draw_calls: AtomicU64::new(0),
                            kernel_calls: AtomicU64::new(0),
                            alive: AtomicBool::new(true),
                        });
                        let worker_counters = Arc::clone(&live_counters);
                        // Only set live_counters/vblank for first worker (main game thread)
                        if is_first {
                            self.worker_live_counters = Some(Arc::clone(&live_counters));
                        }
                        let vblank = if is_first {
                            let v = Arc::new(VBlankSignal::new());
                            self.vblank_signal = Some(Arc::clone(&v));
                            v
                        } else {
                            // Additional threads share the existing vblank or get a dummy
                            self.vblank_signal
                                .clone()
                                .unwrap_or_else(|| Arc::new(VBlankSignal::new()))
                        };
                        let worker_vblank = vblank;
                        if is_first {
                            if let Some(ref xbe) = self.xbe_info {
                                let thunk75 = memory.read_u32(xbe.kernel_thunk_addr + 75 * 4);
                                debug_log(&format!(
                                    "[THUNK-CHECK] Pre-worker: [thunk+0x{:X}] = 0x{:08X} (expect 0xFFFF00B8 for NtAllocateVirtualMemory)",
                                    75 * 4, thunk75
                                ));
                            }
                        }
                        debug_log(&format!(
                            "AOT: spawning worker #{}: entry=0x{:08X} start=0x{:08X} ctx=0x{:08X} (shared kstate={:p})",
                            thread_num, entry, start_routine, start_ctx, shared_kstate
                        ));
                        let thread_name = format!("xbox-worker-{}", thread_num);
                        let handle = std::thread::Builder::new()
                            .name(thread_name)
                            .stack_size(256 * 1024 * 1024) // 256MB
                            .spawn(move || {
                                let result =
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                        worker_dispatch_loop(
                                            worker_ctx,
                                            worker_mem,
                                            entry,
                                            start_routine,
                                            start_ctx,
                                            worker_counters,
                                            worker_vblank,
                                        )
                                    }));
                                match result {
                                    Ok(stats) => stats,
                                    Err(e) => {
                                        let msg = if let Some(s) = e.downcast_ref::<&str>() {
                                            s.to_string()
                                        } else if let Some(s) = e.downcast_ref::<String>() {
                                            s.clone()
                                        } else {
                                            "unknown panic".to_string()
                                        };
                                        debug_log(&format!(
                                            "AOT worker #{}: PANIC — {}",
                                            thread_num, msg
                                        ));
                                        WorkerStats::default()
                                    }
                                }
                            })
                            .expect("failed to spawn xbox-worker thread");
                        new_workers.push(handle);
                    }
                }

                if self.frame_count <= 5 {
                    debug_log(&format!(
                        "AOT: frame done — dispatches={} exit='{}' guest_addr=0x{:08X} esp=0x{:08X}",
                        ctx.dispatch_count, loop_exit_reason, guest_addr, ctx.guest.esp
                    ));
                }

                // Save state for next frame — do NOT restart from entry
                ctx.guest.exit_guest_addr = guest_addr;

                // Track worker threads
                self.worker_handles.extend(new_workers);
                // Collect stats from finished workers
                let mut still_running = Vec::new();
                for handle in self.worker_handles.drain(..) {
                    if handle.is_finished() {
                        if let Ok(stats) = handle.join() {
                            self.worker_total_dispatches += stats.dispatch_count;
                            self.worker_total_mmio += stats.mmio_count;
                            self.worker_total_pb += stats.pb_commands;
                            self.worker_total_draw_calls += stats.draw_calls;
                        }
                    } else {
                        still_running.push(handle);
                    }
                }
                self.worker_handles = still_running;

                // Update display counters including live worker stats
                let veh_dispatches = crate::xbox::aot::veh::take_veh_dispatch_count();
                let (
                    live_worker_dispatches,
                    live_worker_mmio,
                    live_worker_pb,
                    live_worker_draws,
                    live_worker_kernel,
                ) = if let Some(ref counters) = self.worker_live_counters {
                    (
                        counters.dispatch_count.load(Ordering::Relaxed),
                        counters.mmio_count.load(Ordering::Relaxed),
                        counters.pb_commands.load(Ordering::Relaxed),
                        counters.draw_calls.load(Ordering::Relaxed),
                        counters.kernel_calls.load(Ordering::Relaxed),
                    )
                } else {
                    (0, 0, 0, 0, 0)
                };
                self.display.dispatches = veh_dispatches as u32
                    + live_worker_dispatches as u32
                    + self.worker_total_dispatches as u32;
                // Include OOVPA globals (updated by VEH hooks, not by dispatch loop)
                let oovpa_mmio = crate::xbox::aot::oovpa::OOVPA_MMIO_COUNT
                    .load(std::sync::atomic::Ordering::Relaxed);
                let oovpa_pb = crate::xbox::aot::oovpa::OOVPA_PB_COMMANDS
                    .load(std::sync::atomic::Ordering::Relaxed);
                let oovpa_draws = crate::xbox::aot::oovpa::OOVPA_DRAW_CALLS
                    .load(std::sync::atomic::Ordering::Relaxed);
                self.display.mmio_count =
                    (ctx.mmio_count + self.worker_total_mmio + live_worker_mmio).max(oovpa_mmio)
                        as u32;
                self.display.pb_commands =
                    (ctx.pb_commands + self.worker_total_pb + live_worker_pb).max(oovpa_pb) as u32;
                self.display.draw_calls =
                    (ctx.draw_calls + self.worker_total_draw_calls + live_worker_draws)
                        .max(oovpa_draws) as u32;
                // Pipeline status fields
                self.display.veh_events = veh_dispatches as u32;
                self.display.kernel_calls = live_worker_kernel as u32;
                self.display.worker_alive = self.worker_handles.iter().any(|_| true);

                // Pushbuffer spin breaker: sync GET=PUT every frame.
                // D3D render state setters inline the pushbuffer space check.
                // The worker thread can spin forever if GET falls behind PUT.
                // This runs from the main thread every ~16ms and writes to shared
                // guest memory, breaking the inline spin in the worker.
                crate::xbox::aot::oovpa::sync_pushbuffer_get_eq_put(ctx.guest_mem_base);
                || {
                    self.worker_live_counters
                        .as_ref()
                        .map(|c| c.alive.load(std::sync::atomic::Ordering::Relaxed))
                        .unwrap_or(false)
                };
                self.display.gpu_backend = crate::xbox::gpu::backend_name();
                // OOVPA HLE telemetry
                let (hooks_fired, cd, swap, draw) = crate::xbox::aot::oovpa::get_hud_stats();
                self.display.oovpa_hooks_fired = hooks_fired;
                self.display.hle_create_device = cd;
                self.display.hle_swap_count = swap;
                self.display.hle_draw_count = draw;
                // Real draw count: NV2A draw-method submissions parsed from
                // the pushbuffer (NV097_DRAW_ARRAYS, INLINE_ARRAY closes,
                // INLINE_ELEMENTS). Spider-Man uses BeginPush macros so
                // hle_draw_count (SDK helpers) stays at 0; nv2a_draws is the
                // real "geometry submitted" measure for the HUD.
                self.display.nv2a_draws = crate::xbox::aot::nv2a_pb::NV2A_DRAW_METHODS
                    .load(std::sync::atomic::Ordering::Relaxed);
                // Fire milestone events (game-agnostic, no counter thresholds)
                if self.display.kernel_calls > 0 {
                    self.display.signal_event(BootEvent::FirstKernelCall);
                }
                if self.display.worker_alive {
                    self.display.signal_event(BootEvent::WorkerAlive);
                }
                if hooks_fired > 0 {
                    self.display.signal_event(BootEvent::OovpaHooksFired);
                }
                if cd {
                    self.display.signal_event(BootEvent::CreateDeviceFired);
                }
                self.display.update_from_counters();

                // Log live worker stats every 5 seconds (~300 frames)
                if self.frame_count % 300 == 0
                    && (live_worker_dispatches > 0 || self.worker_total_dispatches > 0)
                {
                    debug_log(&format!(
                        "LIVE: frame={} veh={} worker_disp={} worker_mmio={} total_disp={} stage={:?}",
                        self.frame_count, veh_dispatches, live_worker_dispatches + self.worker_total_dispatches,
                        live_worker_mmio + self.worker_total_mmio, self.display.dispatches, self.display.stage()
                    ));
                    // Hotspot dump: shows which guest PC buckets the worker
                    // is spending time in. Useful for finding tight loops
                    // when worker_disp count is low but VEH count is high.
                    crate::xbox::aot::veh::pc_hist_dump(10);
                }
            }
        }

        // RetroArch's CLI verification path may only give us one early
        // retro_run() before the worker reaches D3D. CreateDevice also runs on
        // the worker, so waiting for its request can miss the only main-thread
        // init window. Proactively prepare the host backend once a guest worker
        // exists; this initializes rendering plumbing only, not game pixels.
        if !crate::xbox::gpu::is_gpu_installed() && !self.worker_handles.is_empty() {
            static PROACTIVE_GPU_INIT_LOG: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !PROACTIVE_GPU_INIT_LOG.swap(true, std::sync::atomic::Ordering::Relaxed) {
                debug_log("[GPU] Proactive backend init requested for live guest worker");
            }
            crate::xbox::gpu::request_gpu_init();
        }

        // Poll for GPU init request (worker requests, main thread creates device)
        if let Some(kind) = crate::xbox::gpu::poll_gpu_init(FB_WIDTH as i32, FB_HEIGHT as i32) {
            debug_log(&format!(
                "[GPU] Main thread initialized backend: {:?}",
                kind
            ));
        }

        // D3D11 readback on main thread (context is single-threaded, can't do on worker)
        crate::xbox::gpu::poll_readback();

        // Stage 10+: Worker thread drives Clear → Draw → Swap each frame.
        // Main thread just displays whatever readback pixels the worker produced.
        // No main-thread rendering needed — worker calls GPU backend directly.

        // Use GPU readback if available, otherwise stage display. The libretro
        // environment is explicitly configured for XRGB8888, so this buffer is
        // handed to RetroArch without a lossy 1555 conversion.
        // Read NV2A framebuffer address directly from guest memory.
        // The interpreter writes NV2A registers at 0xFD000000+ (PRAMIN PAGE_READWRITE).
        let (guest_fb_addr, fb_pitch, fb_width, fb_height) = if let Some(ref mem) = self.memory {
            // PCRTC_START = physical address the display scans from
            let pcrtc = mem.read_u32(0xFD60_0800);
            // PGRAPH surface color offset = render target address
            let pgraph_surface = mem.read_u32(0xFD40_0208);
            // PGRAPH surface pitch (low 16 bits = color pitch)
            let pitch_reg = mem.read_u32(0xFD40_020C);
            let pitch = (pitch_reg & 0xFFFF) as u32;

            // Read more NV2A registers for surface format detection
            let surface_fmt = mem.read_u32(0xFD40_0300); // PGRAPH_SURFACE_FORMAT
            let surface_clip_h = mem.read_u32(0xFD40_0304); // SURFACE_CLIP_HORIZONTAL
            let surface_clip_v = mem.read_u32(0xFD40_0308); // SURFACE_CLIP_VERTICAL

            // Log once for debugging
            static FB_LOG_ONCE: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !FB_LOG_ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                debug_log(&format!(
                    "[FB-DETECT] PCRTC=0x{:08X} PGRAPH=0x{:08X} pitch_reg=0x{:08X} pitch={} fmt=0x{:08X} clip_h=0x{:08X} clip_v=0x{:08X}",
                    pcrtc, pgraph_surface, pitch_reg, pitch, surface_fmt, surface_clip_h, surface_clip_v
                ));
                // Scan for framebuffer: look for regions with pixel-like data
                // Real pixels have A=0xFF or A=0x00 in high byte (X8R8G8B8)
                for &candidate in &[
                    0x3C000u32, 0xF00000, 0x100000, 0x200000, 0x400000, 0x800000, 0x1000000,
                    0x80000, 0x40000, 0x50000, 0x60000, 0x70000,
                ] {
                    let p0 = mem.read_u32(candidate);
                    let p1 = mem.read_u32(candidate + 4);
                    let p_mid = mem.read_u32(candidate + 320 * 4); // halfway across first row
                                                                   // Check if values look like pixels (high byte = 0x00 or 0xFF)
                    let hi0 = (p0 >> 24) & 0xFF;
                    let hi1 = (p1 >> 24) & 0xFF;
                    let looks_like_pixels =
                        (hi0 == 0x00 || hi0 == 0xFF) && (hi1 == 0x00 || hi1 == 0xFF);
                    if p0 != 0 {
                        debug_log(&format!(
                            "[FB-SCAN] 0x{:06X}: p0=0x{:08X} p1=0x{:08X} mid=0x{:08X} pixel={}",
                            candidate,
                            p0,
                            p1,
                            p_mid,
                            if looks_like_pixels { "YES" } else { "no" }
                        ));
                    }
                }
            }

            // Check Doom FB flag first — Doom renders to 0x1A000000 (416MB)
            // which is outside the normal 64MB PCRTC range
            let doom_fb_flag = mem.read_u32(crate::xbox::worker::DOOM_FB_FLAG);
            let addr = if doom_fb_flag == 1 {
                crate::xbox::worker::DOOM_FB_ADDR
            } else if pcrtc != 0 && pcrtc < 0x0400_0000 {
                pcrtc
            } else {
                0x03C0_0000u32 // 60MB default
            };

            // Doom renders 640×480 XRGB8888 (2x scaled 320×200 + letterbox).
            // Spider-Man: 640×256 at PCRTC_START.
            let actual_pitch = if pitch > 0 { pitch } else { 640 * 4 };
            let w = 640u32;
            let h = if doom_fb_flag == 1 { 480u32 } else { 256u32 };

            (addr, actual_pitch, w, h)
        } else {
            (0x00F0_0000u32, 640 * 4, 640u32, 480u32)
        };

        // Check Doom screen state object for render buffer location.
        //
        // 2026-04-24: Gate on Doom XBE title. This probe unconditionally
        // reads guest 0x0010_1BB8 expecting Doom's `gamestate` global. For
        // non-Doom titles the address contains unrelated data and the probe
        // falls through to a hardcoded `gameaction=0xDEAD` literal, which
        // then appears in the logs and sends investigations on wild goose
        // chases (see 25-agent audit: X01, G01, G02, G04 all independently
        // confirmed this log was a phantom for Spider-Man).
        let is_doom = self
            .xbe_info
            .as_ref()
            .map(|x| x.title.to_lowercase().contains("doom"))
            .unwrap_or(false);
        if is_doom {
            if let Some(ref mem) = self.memory {
                static DOOM_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let doom_n = DOOM_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if doom_n % 120 == 60 {
                    // Log once per ~2 seconds (after 1 second warmup)
                    let state = mem.read_u32(0x0010_1BB8);
                    let w = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x14)
                    } else {
                        0
                    };
                    let h = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x18)
                    } else {
                        0
                    };
                    let screens0 = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x21AF0)
                    } else {
                        0
                    };
                    let pal_base = if state != 0 && state < 0x0800_0000 {
                        state + 0x396C
                    } else {
                        0
                    };
                    let gameaction = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x2260)
                    } else {
                        0xDEAD
                    };
                    let gametic = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x271C)
                    } else {
                        0
                    };
                    let advdemo = if state != 0 && state < 0x0800_0000 {
                        mem.read_u32(state + 0x2750)
                    } else {
                        0
                    };
                    debug_log(&format!(
                    "[DOOM-STATE] state=0x{:08X} w={} h={} screens[0]=0x{:08X} pal=0x{:08X} gameaction={} gametic={} advdemo={}",
                    state, w, h, screens0, pal_base, gameaction, gametic, advdemo
                ));
                    // Check pixel content at screens[0]
                    if screens0 != 0 && screens0 < 0x1000_0000 {
                        let nonzero = (0..320u32 * 200)
                            .filter(|&i| mem.read_u8(screens0 + i) != 0)
                            .take(100)
                            .count();
                        debug_log(&format!(
                            "  screens[0] nonzero pixels: {}+ / 64000",
                            nonzero
                        ));
                    }
                }
            }
        } // end if is_doom

        // Check for Doom HLE framebuffer (worker writes XRGB8888 at DOOM_FB_ADDR)
        let doom_fb_ready = if let Some(ref mem) = self.memory {
            mem.read_u32(crate::xbox::worker::DOOM_FB_FLAG) != 0
        } else {
            false
        };

        if doom_fb_ready {
            if let Some(ref mem) = self.memory {
                let fb_ptr =
                    (mem.base() as u64 + crate::xbox::worker::DOOM_FB_ADDR as u64) as *const u32;
                let src_u32 = unsafe { std::slice::from_raw_parts(fb_ptr, FB_WIDTH * FB_HEIGHT) };
                unsafe {
                    if let Some(video) = G_VIDEO {
                        if G_VIDEO_XRGB8888.load(Ordering::Relaxed) {
                            video(
                                src_u32.as_ptr() as *const std::os::raw::c_void,
                                FB_WIDTH as u32,
                                FB_HEIGHT as u32,
                                FB_WIDTH * 4,
                            );
                        } else {
                            for (i, &pixel) in src_u32.iter().enumerate() {
                                let r = ((pixel >> 16) & 0xFF) as u16;
                                let g = ((pixel >> 8) & 0xFF) as u16;
                                let b = (pixel & 0xFF) as u16;
                                self.video_buf_1555[i] =
                                    ((r >> 3) << 10) | ((g >> 3) << 5) | (b >> 3);
                            }
                            video(
                                self.video_buf_1555.as_ptr() as *const std::os::raw::c_void,
                                FB_WIDTH as u32,
                                FB_HEIGHT as u32,
                                FB_WIDTH * 2,
                            );
                        }
                    }
                }
                return;
            }
        }

        let has_guest_content = if let Some(ref mem) = self.memory {
            let p0 = mem.read_u32(guest_fb_addr);
            let p_mid = mem.read_u32(guest_fb_addr + fb_pitch * (fb_height / 2));
            // Scan entire guest RAM for pixel-like data (0xFF high byte = X8R8G8B8)
            static DUMPED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !DUMPED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                // Scan every 64KB page for pixel content
                debug_log("[FB-SCAN] Scanning all guest RAM for pixel data...");
                for page in (0x00010000u32..0x08000000).step_by(0x10000) {
                    let raw = unsafe {
                        std::slice::from_raw_parts(
                            (mem.base() as u64 + page as u64) as *const u8,
                            0x10000,
                        )
                    };
                    // Check for pixel-like data: many bytes with 0xFF in position 3 (alpha)
                    let mut pixel_count = 0u32;
                    for i in (0..4096).step_by(4) {
                        if raw[i + 3] == 0xFF || (raw[i] != 0 && raw[i + 1] != 0 && raw[i + 2] != 0)
                        {
                            pixel_count += 1;
                        }
                    }
                    if pixel_count > 200 {
                        // >20% of samples look like pixels
                        let p0 = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
                        let p1 = u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]);
                        debug_log(&format!(
                            "[FB-FOUND] 0x{:08X}: {} pixel-like samples p0=0x{:08X} p1=0x{:08X}",
                            page, pixel_count, p0, p1
                        ));
                        // Dump this page
                        let dump_size = 4096 * 480;
                        let dump = unsafe {
                            std::slice::from_raw_parts(
                                (mem.base() as u64 + page as u64) as *const u8,
                                dump_size.min((0x08000000 - page) as usize),
                            )
                        };
                        let path = format!("./fb_{:07X}.raw", page);
                        let _ = std::fs::write(&path, dump);
                    }
                }
            }
            // Lie-detector fix 2026-04-20: the bare `p0 != 0 || p_mid != 0`
            // heuristic trips on ANY non-zero byte near 0x03C00000, which is
            // uninitialized RAM in the 4GB VirtualAlloc2 reservation. Random
            // heap pointers / alloc headers / kernel state fragments at that
            // address would falsely mark "has guest content" and send us into
            // a swizzle/deswizzle investigation chasing heap garbage as pixels.
            // Require that the NV2A PGRAPH surface color register is ALSO
            // non-zero, proving the GPU engine was actually programmed to use
            // this page as a render target. If PGRAPH is still zero, the
            // guest hasn't configured a surface — fall back to stage_display.
            let pgraph_color = mem.read_u32(0xFD40_0208);
            (p0 != 0 || p_mid != 0) && pgraph_color != 0
        } else {
            false
        };

        let mut gpu_pixels = crate::xbox::gpu::take_readback();
        // GPU readback takes priority over guest framebuffer scan.
        // When HLE D3D is active, the D3D9 backend renders offscreen —
        // guest memory won't have valid pixel data.
        let fb_source: &'static str;
        let src_u32: &[u32] = if gpu_pixels.is_some() {
            // Use GPU readback (from HLE Clear/Draw/Swap → backend)
            fb_source = "gpu_readback";
            let pixels = gpu_pixels.as_mut().unwrap();
            // 2026-04-24: HUD overlay suppressed by default — it was painting
            // diagnostic text over every GPU-readback frame once stage hit
            // HudOverlay (which triggers as soon as any draw fires). Set
            // RUSTEMU_HUD_OVERLAY=1 to re-enable for debugging.
            let overlay_enabled = std::env::var("RUSTEMU_HUD_OVERLAY")
                .map(|v| v == "1")
                .unwrap_or(false);
            if overlay_enabled && self.display.stage() >= BootStage::HudOverlay {
                self.display.render_hud_overlay(pixels);
            }
            pixels
        } else if has_guest_content {
            // Guest framebuffer has content — read from guest memory.
            // Xbox NV2A uses X8R8G8B8 (BGRA byte order on little-endian).
            // RetroArch XRGB8888 is also BGRA byte order. No swizzle needed.
            // Read at the detected pitch — try multiple common pitches.
            fb_source = "guest_memory";
            if let Some(ref mem) = self.memory {
                // Read raw guest pixels into our 640x480 output buffer.
                // Guest might be 512x256 or 640x480. Scale to fit.
                let fb_base = mem.base() as u64 + guest_fb_addr as u64;
                // Try pitch = 640*4 first, then 512*4, then 1024*4
                let pitch = 640 * 4u32; // 2560 bytes per line
                let fb_ptr = fb_base as *const u32;
                unsafe { std::slice::from_raw_parts(fb_ptr, FB_WIDTH * FB_HEIGHT) }
            } else {
                let fb = self.display.render();
                unsafe {
                    std::slice::from_raw_parts(fb.as_ptr() as *const u32, FB_WIDTH * FB_HEIGHT)
                }
            }
        } else {
            fb_source = "stage_display";
            let fb = self.display.render();
            unsafe { std::slice::from_raw_parts(fb.as_ptr() as *const u32, FB_WIDTH * FB_HEIGHT) }
        };
        // 2026-04-20 raw framebuffer dump for offline Python analysis.
        // Captures the EXACT u32 buffer that gets sent to RetroArch — includes
        // guest_fb path (raw guest memory), gpu_pixels path, and fallback.
        // Writes first 20 non-black frames to fb_dump_NN.bin. Each file is
        // FB_WIDTH*FB_HEIGHT*4 bytes of u32 (little-endian) XRGB8888.
        {
            // Capture across the full run, not just the first 20 frames.
            // Frames 0-9: every frame (catch early stage_display).
            // Frames 10-99: every 10th (catch CRT/D3D-init transition).
            // Frames 100+: every 100th, up to 50 total dumps.
            static FRAME_COUNTER: std::sync::atomic::AtomicU64 =
                std::sync::atomic::AtomicU64::new(0);
            static DUMP_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let fr = FRAME_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let should_dump = if fr < 10 {
                true
            } else if fr < 100 {
                fr % 10 == 0
            } else {
                fr % 100 == 0
            };
            let n = DUMP_N.load(std::sync::atomic::Ordering::Relaxed);
            if should_dump && n < 50 {
                let nonzero = src_u32
                    .iter()
                    .filter(|&&p| p != 0 && p != 0xFF000000)
                    .count();
                if nonzero > 256 {
                    let path = format!(r"./fb_dump_{:02}.bin", n);
                    if let Ok(mut f) = std::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(&path)
                    {
                        use std::io::Write;
                        let bytes: &[u8] = unsafe {
                            std::slice::from_raw_parts(
                                src_u32.as_ptr() as *const u8,
                                src_u32.len() * 4,
                            )
                        };
                        let _ = f.write_all(bytes);

                        // Sidecar metadata so offline analysis knows EXACTLY
                        // what this buffer is and where it came from.
                        // Without this, each PNG variant is a guess in a vacuum.
                        let (hooks_fired, cd_count, swap_count, draw_count) =
                            crate::xbox::aot::oovpa::get_hud_stats();
                        let (live_disp, live_mmio, live_pb, live_draw, live_kern) =
                            if let Some(ref counters) = self.worker_live_counters {
                                use std::sync::atomic::Ordering;
                                (
                                    counters.dispatch_count.load(Ordering::Relaxed),
                                    counters.mmio_count.load(Ordering::Relaxed),
                                    counters.pb_commands.load(Ordering::Relaxed),
                                    counters.draw_calls.load(Ordering::Relaxed),
                                    counters.kernel_calls.load(Ordering::Relaxed),
                                )
                            } else {
                                (0, 0, 0, 0, 0)
                            };
                        // NV2A surface-format / PCRTC registers (same addresses
                        // FB-DETECT reads on first frame). Re-read here so we
                        // capture whatever the state is AT DUMP TIME.
                        let (pcrtc, pgraph_surf, pitch_reg, surf_fmt, clip_h, clip_v) =
                            if let Some(ref mem) = self.memory {
                                (
                                    mem.read_u32(0xFD60_0800),
                                    mem.read_u32(0xFD40_0208),
                                    mem.read_u32(0xFD40_020C),
                                    mem.read_u32(0xFD40_0300),
                                    mem.read_u32(0xFD40_0304),
                                    mem.read_u32(0xFD40_0308),
                                )
                            } else {
                                (0, 0, 0, 0, 0, 0)
                            };
                        // Count unique non-zero pixel values — if ≤ ~8, this
                        // buffer is a clear/fill pattern, NOT a real image.
                        let mut uniq = std::collections::BTreeSet::new();
                        for &p in src_u32.iter().take(16384) {
                            if p != 0 && p != 0xFF000000 {
                                uniq.insert(p);
                                if uniq.len() > 64 {
                                    break;
                                }
                            }
                        }
                        let first16: Vec<String> = src_u32
                            .iter()
                            .take(16)
                            .map(|p| format!("{:08X}", p))
                            .collect();
                        let meta = format!(
                            "frame: {}\n\
                             emulator_frame: {}\n\
                             source: {}\n\
                             fb_width_out: {}\n\
                             fb_height_out: {}\n\
                             non_zero_pixels: {}\n\
                             unique_nonzero_sample: {} (of first 16384 px, capped at 64)\n\
                             first_16_dwords: [{}]\n\
                             \n\
                             [guest_framebuffer_detection]\n\
                             guest_fb_addr: 0x{:08X}\n\
                             fb_pitch: {} bytes ({} px at 4bpp)\n\
                             fb_width_detected: {}\n\
                             fb_height_detected: {}\n\
                             \n\
                             [nv2a_registers_at_dump_time]\n\
                             PCRTC_START (0xFD600800): 0x{:08X}\n\
                             PGRAPH_SURFACE_COLOR (0xFD400208): 0x{:08X}\n\
                             PGRAPH_SURFACE_PITCH (0xFD40020C): 0x{:08X}\n\
                             PGRAPH_SURFACE_FORMAT (0xFD400300): 0x{:08X}\n\
                             SURFACE_CLIP_HORIZONTAL (0xFD400304): 0x{:08X}\n\
                             SURFACE_CLIP_VERTICAL (0xFD400308): 0x{:08X}\n\
                             \n\
                             [counters]\n\
                             hle_hooks_fired: {}\n\
                             hle_create_device: {}\n\
                             hle_swap_count: {}\n\
                             hle_draw_count: {}\n\
                             worker_dispatches: {}\n\
                             worker_mmio: {}\n\
                             worker_pb_commands: {}\n\
                             worker_draws: {}\n\
                             worker_kernel_calls: {}\n",
                            n,
                            fr,
                            fb_source,
                            FB_WIDTH,
                            FB_HEIGHT,
                            nonzero,
                            uniq.len(),
                            first16.join(", "),
                            guest_fb_addr,
                            fb_pitch,
                            fb_pitch / 4,
                            fb_width,
                            fb_height,
                            pcrtc,
                            pgraph_surf,
                            pitch_reg,
                            surf_fmt,
                            clip_h,
                            clip_v,
                            hooks_fired,
                            cd_count,
                            swap_count,
                            draw_count,
                            live_disp,
                            live_mmio,
                            live_pb,
                            live_draw,
                            live_kern,
                        );
                        let meta_path = format!(r"./fb_dump_{:02}.meta.txt", n);
                        let _ = std::fs::write(&meta_path, &meta);

                        debug_log(&format!(
                            "[FB-DUMP #{}] {} bytes={} px={} nonzero={} uniq<=64? {} source={} \
                             pcrtc=0x{:08X} pgraph=0x{:08X} fmt=0x{:08X} draws_hle={} swaps_hle={}",
                            n,
                            path,
                            bytes.len(),
                            src_u32.len(),
                            nonzero,
                            uniq.len(),
                            fb_source,
                            pcrtc,
                            pgraph_surf,
                            surf_fmt,
                            draw_count,
                            swap_count
                        ));
                        DUMP_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
        }
        unsafe {
            if let Some(video) = G_VIDEO {
                if G_VIDEO_XRGB8888.load(Ordering::Relaxed) {
                    video(
                        src_u32.as_ptr() as *const std::os::raw::c_void,
                        FB_WIDTH as u32,
                        FB_HEIGHT as u32,
                        FB_WIDTH * 4,
                    );
                } else {
                    for (i, &pixel) in src_u32.iter().enumerate() {
                        let r = ((pixel >> 16) & 0xFF) as u16;
                        let g = ((pixel >> 8) & 0xFF) as u16;
                        let b = (pixel & 0xFF) as u16;
                        self.video_buf_1555[i] = ((r >> 3) << 10) | ((g >> 3) << 5) | (b >> 3);
                    }
                    video(
                        self.video_buf_1555.as_ptr() as *const std::os::raw::c_void,
                        FB_WIDTH as u32,
                        FB_HEIGHT as u32,
                        FB_WIDTH * 2,
                    );
                }
            }
        }
    }

    fn submit_audio_frame(&mut self) {
        const AUDIO_FRAMES_PER_TICK: usize = 48_000 / 60;

        let Some(memory) = self.memory.as_ref() else {
            return;
        };

        let samples =
            crate::xbox::apu::dsound::mix_audio_frame(memory.base(), AUDIO_FRAMES_PER_TICK);
        if samples.is_empty() {
            return;
        }

        unsafe {
            if let Some(batch) = G_AUDIO_SAMPLE_BATCH {
                let frames = samples.len() / 2;
                let submitted = batch(samples.as_ptr(), frames);
                static AUDIO_SUBMIT_LOG: AtomicU32 = AtomicU32::new(0);
                let n = AUDIO_SUBMIT_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 8 || (n.is_power_of_two() && submitted != frames) {
                    debug_log(&format!(
                        "[AUDIO-LIBRETRO] #{} submitted {}/{} frames",
                        n, submitted, frames
                    ));
                }
            } else if let Some(sample) = G_AUDIO_SAMPLE {
                for pair in samples.chunks_exact(2) {
                    sample(pair[0], pair[1]);
                }
            }
        }
    }

    pub(crate) fn unload(&mut self) {
        debug_log(&format!(
            "Unloading game (frame_count={})",
            self.frame_count
        ));

        // Log ordinal hit summary before cleanup
        if let Some(kstate) = &self.kernel_state {
            for line in kstate.hit_summary_lines() {
                debug_log(&line);
            }
        }

        // Signal all guest threads to stop, then wait with timeout
        crate::xbox::aot::veh::request_stop();

        let worker_count = self.worker_handles.len();
        if worker_count > 0 {
            debug_log(&format!(
                "Waiting for {} worker thread(s) (stop requested)...",
                worker_count
            ));
            for handle in self.worker_handles.drain(..) {
                // Wait up to 2 seconds per worker, then abandon
                let start = std::time::Instant::now();
                loop {
                    if handle.is_finished() {
                        if let Ok(stats) = handle.join() {
                            self.worker_total_dispatches += stats.dispatch_count;
                            self.worker_total_mmio += stats.mmio_count;
                            self.worker_total_pb += stats.pb_commands;
                            self.worker_total_draw_calls += stats.draw_calls;
                        }
                        break;
                    }
                    if start.elapsed() > std::time::Duration::from_secs(2) {
                        debug_log("Worker thread did not stop in 2s — abandoning");
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
            debug_log("All worker threads joined or abandoned");
        }

        // Dump Doom framebuffer to BMP before teardown
        if let Some(ref mem) = self.memory {
            crate::xbox::worker::dump_framebuffer_bmp(mem);
        }

        // Remove OOVPA state and dump summary before VEH removal
        if let Some(state) = crate::xbox::aot::oovpa::remove_state() {
            crate::xbox::aot::oovpa::dump_summary(&state.matches);
        }

        // Remove VEH and reset DPC state before dropping context
        crate::xbox::aot::veh::remove_veh();
        crate::xbox::aot::veh_dpc::reset_all();

        // Drop AOT context and code buffer
        if let Some(ctx) = self.aot_ctx.take() {
            debug_log(&format!(
                "AOT stats: dispatches={} shadow_hits={} hash_lookups={} indirect={} system={} mmio={} pb={}",
                ctx.dispatch_count, ctx.ret_shadow_hits, ctx.ret_hash_lookups,
                ctx.indirect_dispatches, ctx.system_traps, ctx.mmio_count, ctx.pb_commands
            ));
        }
        self.code_buffer = None;
        self.kernel_state = None;

        self.xbe_info = None;
        self.clear_memory_maps();
        self.memory = None;
        self.display = StageDisplay::new();
        self.frame_count = 0;
        self.timing_start = std::time::Instant::now();
        self.spiderman_framectx_injected = false;
        self.diag_dump_option_enabled = false;
        self.diag_dump_option_initialized = false;
        self.diag_dump_count = 0;
    }
}

// normalize_guest_addr and worker_dispatch_loop moved to worker.rs
pub(crate) use super::worker::normalize_guest_addr;
use super::worker::worker_dispatch_loop;

// FFI entry points (retro_*) moved to ffi.rs
