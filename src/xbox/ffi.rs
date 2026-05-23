/// Raw libretro FFI entry points.
/// These are the `#[no_mangle] extern "C"` symbols exported to RetroArch.
/// All globals (G_ENVIRON, G_VIDEO, etc.) live in emulator.rs and are
/// accessed via `super::emulator::*`.
use super::emulator::{
    debug_log, init_logger, RetroAudioSampleBatchT, RetroAudioSampleT, RetroEnvironmentT,
    RetroGameGeometry, RetroGameInfo, RetroInputPollT, RetroInputStateT, RetroSystemAvInfo,
    RetroSystemInfo, RetroSystemTiming, RetroVariable, RetroVideoRefreshT, XboxEmulator,
    G_AUDIO_SAMPLE, G_AUDIO_SAMPLE_BATCH, G_EMULATOR, G_ENVIRON, G_INPUT_POLL, G_INPUT_STATE,
    G_VIDEO, LIB_NAME, LIB_VERSION, RETRO_API_VERSION, RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
    RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, RETRO_ENVIRONMENT_SET_VARIABLES,
    RETRO_PIXEL_FORMAT_XRGB8888, VALID_EXTENSIONS,
};
use crate::xbox::display::stage::{FB_HEIGHT, FB_WIDTH};

// ============================================================================
// Raw libretro FFI entry points
// ============================================================================

unsafe fn configure_pixel_format(cb: RetroEnvironmentT, context: &str) {
    let mut pixel_format: u32 = RETRO_PIXEL_FORMAT_XRGB8888;
    let accepted = cb(
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
        &mut pixel_format as *mut u32 as *mut std::os::raw::c_void,
    );
    crate::xbox::emulator::G_VIDEO_XRGB8888.store(accepted, std::sync::atomic::Ordering::Relaxed);
    if accepted {
        debug_log(&format!("{context}: pixel format XRGB8888 accepted"));
    } else {
        debug_log(&format!(
            "{context}: pixel format XRGB8888 rejected; using 0RGB1555 fallback"
        ));
    }
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(cb: RetroEnvironmentT) {
    G_ENVIRON = Some(cb);
    let mut no_game: bool = true;
    cb(
        RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
        &mut no_game as *mut bool as *mut std::os::raw::c_void,
    );

    let variables = [
        RetroVariable {
            key: b"rustemu_cpu_backend\0".as_ptr() as *const std::os::raw::c_char,
            value: b"CPU backend; aot|jit-diagnostic\0".as_ptr() as *const std::os::raw::c_char,
        },
        RetroVariable {
            key: b"rustemu_dump_diagnostics\0".as_ptr() as *const std::os::raw::c_char,
            value: b"Dump diagnostic state; disabled|enabled\0".as_ptr()
                as *const std::os::raw::c_char,
        },
        RetroVariable {
            key: std::ptr::null(),
            value: std::ptr::null(),
        },
    ];
    cb(
        RETRO_ENVIRONMENT_SET_VARIABLES,
        variables.as_ptr() as *mut std::os::raw::c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(cb: RetroVideoRefreshT) {
    G_VIDEO = Some(cb);
}
#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(cb: RetroAudioSampleT) {
    G_AUDIO_SAMPLE = Some(cb);
    crate::xbox::apu::dsound::set_audio_output_active(true);
}
#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(cb: RetroAudioSampleBatchT) {
    G_AUDIO_SAMPLE_BATCH = Some(cb);
    crate::xbox::apu::dsound::set_audio_output_active(true);
}
#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(cb: RetroInputPollT) {
    G_INPUT_POLL = Some(cb);
}
#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(cb: RetroInputStateT) {
    G_INPUT_STATE = Some(cb);
    crate::xbox::emulator::debug_log("retro_set_input_state: callback installed");
}

#[no_mangle]
pub unsafe extern "C" fn retro_init() {
    init_logger();
    // RetroArch's log callback is printf-style variadic. Calling it safely
    // from Rust requires a C shim so we can pass "%s", message. Passing the
    // whole Rustemu log line as the format string can crash inside msvcrt if
    // the line contains a stray percent sequence, so keep desktop runs on the
    // buffered file logger by default.
    if std::env::var_os("RUSTEMU_ENABLE_UNSAFE_RETRO_LOG_CALLBACK").is_some() {
        if let Some(env_cb) = crate::xbox::emulator::G_ENVIRON {
            #[repr(C)]
            struct RetroLogCallback {
                log: *const std::os::raw::c_void,
            }
            let mut log_cb = RetroLogCallback {
                log: std::ptr::null(),
            };
            if env_cb(
                crate::xbox::emulator::RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
                &mut log_cb as *mut RetroLogCallback as *mut std::os::raw::c_void,
            ) {
                if !log_cb.log.is_null() {
                    let printf_fn: crate::xbox::logging::RetroLogPrintfT =
                        std::mem::transmute(log_cb.log);
                    crate::xbox::logging::set_retro_callback(printf_fn);
                    log::info!(target: "emu",
                    "retro_init: UNSAFE RetroArch log callback installed");
                } else {
                    log::warn!(target: "emu",
                    "retro_init: GET_LOG_INTERFACE returned null printf — using file fallback");
                }
            } else {
                log::warn!(target: "emu",
                "retro_init: frontend declined GET_LOG_INTERFACE — using file fallback");
            }
        }
    } else {
        log::info!(target: "emu",
            "retro_init: RetroArch log callback disabled; using buffered file logger");
    }
    G_EMULATOR = Some(XboxEmulator::new());
    log::info!(target: "emu", "retro_init: emulator created");
}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {
    debug_log("retro_deinit");
    crate::xbox::aot::veh_fixup::dump_r15_fixup_profile("retro_deinit");
    crate::xbox::profiler::dump_cpu_frame_profile("retro_deinit");
    G_EMULATOR = None;
    debug_log("retro_deinit complete");
}

#[no_mangle]
pub extern "C" fn retro_api_version() -> u32 {
    RETRO_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut RetroSystemInfo) {
    let info = &mut *info;
    info.library_name = LIB_NAME.as_ptr() as *const _;
    info.library_version = LIB_VERSION.as_ptr() as *const _;
    info.valid_extensions = VALID_EXTENSIONS.as_ptr() as *const _;
    info.need_fullpath = true;
    info.block_extract = false;
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut RetroSystemAvInfo) {
    let info = &mut *info;
    info.geometry.base_width = FB_WIDTH as u32;
    info.geometry.base_height = FB_HEIGHT as u32;
    info.geometry.max_width = FB_WIDTH as u32;
    info.geometry.max_height = FB_HEIGHT as u32;
    info.geometry.aspect_ratio = 4.0 / 3.0;
    info.timing.fps = 60.0;
    info.timing.sample_rate = 48000.0;
}

#[no_mangle]
pub unsafe extern "C" fn retro_load_game(game: *const RetroGameInfo) -> bool {
    debug_log("retro_load_game called");
    crate::xbox::profiler::reset_cpu_frame_profile();
    debug_log(&format!(
        "retro_load_game: CPU frame profile reset active={}",
        crate::xbox::profiler::cpu_frame_prof_active()
    ));
    // Per H04 finding: veh_dpc::reset_all() was only called at retro_unload_game
    // entry, leaving DPC/ISR capture statics stale if RetroArch calls
    // retro_load_game twice without an unload. That's a nondeterminism source.
    // Calling reset_all() here too is low-risk defensive — per-load reset.
    crate::xbox::aot::veh_dpc::reset_all();
    debug_log("retro_load_game: veh_dpc state reset");
    crate::xbox::aot::veh_fixup::reset_r15_fixup_profile();
    debug_log(&format!(
        "retro_load_game: R15 fixup profile reset active={}",
        crate::xbox::aot::veh_fixup::r15_fixup_profile_active()
    ));
    crate::xbox::emulator::reset_input_snapshot();
    debug_log("retro_load_game: input snapshot reset");
    if let Some(env_cb) = G_ENVIRON {
        configure_pixel_format(env_cb, "retro_load_game");
    }
    if let Some(emu) = G_EMULATOR.as_mut() {
        emu.load_game(game)
    } else {
        debug_log("retro_load_game: emulator not initialized!");
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn retro_run() {
    if let Some(poll) = G_INPUT_POLL {
        poll();
    }
    crate::xbox::emulator::poll_input_snapshot();
    if let Some(emu) = G_EMULATOR.as_mut() {
        emu.run();
    }
}

#[no_mangle]
pub unsafe extern "C" fn retro_unload_game() {
    if let Some(emu) = G_EMULATOR.as_mut() {
        emu.unload();
    }
    crate::xbox::aot::veh_fixup::dump_r15_fixup_profile("retro_unload_game");
    crate::xbox::profiler::dump_cpu_frame_profile("retro_unload_game");
}

#[no_mangle]
pub unsafe extern "C" fn retro_reset() {
    debug_log("retro_reset: reset requested");
    // Per H07: a real retro_reset now exists. Resets veh_dpc state so
    // RetroArch's "Restart" button clears stale DPC/ISR capture atomics
    // that otherwise persist across sessions and cause boot nondeterminism.
    // Does NOT re-spawn the worker — frontend normally calls unload+load
    // if it wants a full reload. This is a soft reset: guest state is
    // left as-is; only per-session statics are zeroed.
    crate::xbox::aot::veh_dpc::reset_all();
    debug_log("retro_reset: veh_dpc state cleared");
}
#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(_port: u32, _device: u32) {}
#[no_mangle]
pub extern "C" fn retro_get_region() -> u32 {
    0
}
#[no_mangle]
pub extern "C" fn retro_load_game_special(
    _typ: u32,
    _info: *const RetroGameInfo,
    _num: usize,
) -> bool {
    false
}
#[no_mangle]
pub extern "C" fn retro_serialize_size() -> usize {
    0
}
#[no_mangle]
pub extern "C" fn retro_serialize(_data: *mut std::os::raw::c_void, _size: usize) -> bool {
    false
}
#[no_mangle]
pub extern "C" fn retro_unserialize(_data: *const std::os::raw::c_void, _size: usize) -> bool {
    false
}
#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_data(id: u32) -> *mut std::os::raw::c_void {
    if let Some(emu) = G_EMULATOR.as_ref() {
        emu.retro_memory_data(id)
    } else {
        std::ptr::null_mut()
    }
}
#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_size(id: u32) -> usize {
    if let Some(emu) = G_EMULATOR.as_ref() {
        emu.retro_memory_size(id)
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn retro_cheat_reset() {}
#[no_mangle]
pub extern "C" fn retro_cheat_set(_index: u32, _enabled: bool, _code: *const std::os::raw::c_char) {
}
