use super::{KernelResult, KernelState, XboxObjectType};
/// Xbox kernel HAL, RTL, and misc stubs.
/// Ported from AOT_KrnlHal.cpp.
///
/// Covers: debug, HAL, timing, strings, RTL, crypto stubs, Nt* stubs,
/// Ke* event/sync stubs, Ob*, Ps*, Io*, Xe*, etc.
use crate::xbox::memory::guest_memory::{
    GuestMemory, RAM_MIRROR_BASE, RAM_SIZE, RAM_UNCACHED_BASE,
};
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

const PERF_FREQ: u32 = 3_375_000; // 3.375 MHz fake clock
const PERF_INCR: u64 = 33_333; // ~30Hz tick increment
const FAKE_TIME_INCR: u64 = 100_000;

/// Shared IRQL state — real kernel stores at 0x80035C00.
/// 0 = PASSIVE_LEVEL, 2 = DISPATCH_LEVEL, 0x1C = SYNCH_LEVEL
static CURRENT_IRQL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

const XC_GAME_REGION_NA: u32 = 0x0000_0001;
const XC_GAME_REGION_JAPAN: u32 = 0x0000_0002;
const XC_GAME_REGION_RESTOFWORLD: u32 = 0x0000_0004;

const AV_PACK_HDTV: u32 = 0x0000_0004;
const AV_STANDARD_NTSC_M: u32 = 0x0000_0100;
const AV_STANDARD_NTSC_J: u32 = 0x0000_0200;
const AV_STANDARD_PAL_I: u32 = 0x0000_0300;
const AV_FLAGS_HDTV_480P: u32 = 0x0008_0000;
const AV_FLAGS_60HZ: u32 = 0x0040_0000;
const AV_FLAGS_50HZ: u32 = 0x0080_0000;
const STATUS_WAIT_0: u32 = 0;
const STATUS_TIMEOUT: u32 = 0x0000_0102;

const DISPATCHER_TYPE_NOTIFICATION_EVENT: u8 = 0;
const DISPATCHER_TYPE_SYNCHRONIZATION_EVENT: u8 = 1;
const DISPATCHER_TYPE_MUTANT: u8 = 2;
const DISPATCHER_TYPE_SEMAPHORE: u8 = 5;
const DISPATCHER_TYPE_THREAD: u8 = 6;
const DISPATCHER_TYPE_TIMER_BASE: u8 = 8;
const DISPATCHER_TYPE_MASK: u8 = 0x7F;

struct DispatcherWaiter {
    generation: Mutex<u64>,
    condvar: Condvar,
}

fn dispatcher_waiters() -> &'static Mutex<HashMap<u32, Arc<DispatcherWaiter>>> {
    static WAITERS: OnceLock<Mutex<HashMap<u32, Arc<DispatcherWaiter>>>> = OnceLock::new();
    WAITERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn dispatcher_waiter(obj_addr: u32) -> Arc<DispatcherWaiter> {
    let mut waiters = dispatcher_waiters()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    waiters
        .entry(obj_addr)
        .or_insert_with(|| {
            Arc::new(DispatcherWaiter {
                generation: Mutex::new(0),
                condvar: Condvar::new(),
            })
        })
        .clone()
}

fn notify_dispatcher_waiters(obj_addr: u32) {
    let waiters = dispatcher_waiters()
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    for key in [obj_addr, 0] {
        if let Some(waiter) = waiters.get(&key) {
            let mut generation = waiter.generation.lock().unwrap_or_else(|e| e.into_inner());
            *generation = generation.wrapping_add(1);
            waiter.condvar.notify_all();
        }
    }
}

fn menu_wait_trace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_MENU_WAIT_TRACE")
            .or_else(|_| std::env::var("RUSTEMU_TITLE_PROBES"))
            .map(|v| {
                let v = v.trim();
                v == "1"
                    || v.eq_ignore_ascii_case("true")
                    || v.eq_ignore_ascii_case("yes")
                    || v.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

fn menu_wait_trace_log(n: u32) -> bool {
    menu_wait_trace_enabled() && (n < 64 || n.is_power_of_two() || n % 1000 == 0)
}

fn dispatcher_obj_valid(obj_addr: u32) -> bool {
    obj_addr != 0 && obj_addr < 0x1000_0000
}

fn dispatcher_type(memory: &GuestMemory, obj_addr: u32) -> u8 {
    memory.read_u8(obj_addr) & DISPATCHER_TYPE_MASK
}

fn dispatcher_is_signaled(memory: &GuestMemory, obj_addr: u32) -> bool {
    dispatcher_obj_valid(obj_addr) && memory.read_u32(obj_addr + 0x04) != 0
}

fn satisfy_dispatcher_object(memory: &GuestMemory, obj_addr: u32) {
    if !dispatcher_obj_valid(obj_addr) {
        return;
    }
    match dispatcher_type(memory, obj_addr) {
        DISPATCHER_TYPE_SYNCHRONIZATION_EVENT => {
            memory.write_u32(obj_addr + 0x04, 0);
        }
        DISPATCHER_TYPE_SEMAPHORE => {
            let signal = memory.read_u32(obj_addr + 0x04);
            if signal > 0 {
                memory.write_u32(obj_addr + 0x04, signal - 1);
            }
        }
        DISPATCHER_TYPE_MUTANT => {
            let signal = memory.read_u32(obj_addr + 0x04);
            if signal > 0 {
                memory.write_u32(obj_addr + 0x04, signal - 1);
            }
        }
        _ => {}
    }
}

fn init_dispatcher_header(
    memory: &GuestMemory,
    obj_addr: u32,
    obj_type: u8,
    size_longs: u8,
    signal_state: u32,
) {
    if !dispatcher_obj_valid(obj_addr) {
        return;
    }
    memory.write_u8(obj_addr, obj_type);
    memory.write_u8(obj_addr + 2, size_longs);
    memory.write_u32(obj_addr + 0x04, signal_state);
    memory.write_u32(obj_addr + 0x08, obj_addr + 0x08);
    memory.write_u32(obj_addr + 0x0C, obj_addr + 0x08);
}

#[derive(Clone, Copy)]
enum WaitDeadline {
    Infinite,
    Poll,
    Until(Instant),
}

fn guest_i64_ptr_valid(ptr: u32) -> bool {
    if ptr == 0 {
        return false;
    }
    let ptr = ptr as u64;
    let end = ptr.saturating_add(8);
    let ram_size = RAM_SIZE as u64;
    let mirror_base = RAM_MIRROR_BASE as u64;
    let uncached_base = RAM_UNCACHED_BASE as u64;

    end <= ram_size
        || (ptr >= mirror_base && end <= mirror_base + ram_size)
        || (ptr >= uncached_base && end <= uncached_base + ram_size)
}

fn read_guest_i64(memory: &GuestMemory, ptr: u32) -> Option<i64> {
    if !guest_i64_ptr_valid(ptr) {
        return None;
    }
    let lo = memory.read_u32(ptr) as u64;
    let hi = memory.read_u32(ptr + 4) as u64;
    Some(((hi << 32) | lo) as i64)
}

fn wait_deadline(memory: &GuestMemory, timeout_ptr: u32) -> WaitDeadline {
    if timeout_ptr == 0 {
        return WaitDeadline::Infinite;
    }

    let Some(raw) = read_guest_i64(memory, timeout_ptr) else {
        return WaitDeadline::Infinite;
    };
    if raw == 0 {
        WaitDeadline::Poll
    } else if raw < 0 {
        let ticks_100ns = raw.saturating_neg() as u64;
        let dur = Duration::from_nanos(ticks_100ns.saturating_mul(100).min(10_000_000_000));
        WaitDeadline::Until(Instant::now() + dur)
    } else {
        // Absolute kernel times are rare in the current titles; cap them so a
        // bad guest timestamp cannot suspend the host thread for minutes.
        WaitDeadline::Until(Instant::now() + Duration::from_secs(10))
    }
}

fn wait_chunk(deadline: WaitDeadline) -> Option<Duration> {
    const MAX_CHUNK: Duration = Duration::from_millis(16);
    match deadline {
        WaitDeadline::Infinite => Some(MAX_CHUNK),
        WaitDeadline::Poll => None,
        WaitDeadline::Until(deadline) => {
            let now = Instant::now();
            if now >= deadline {
                None
            } else {
                Some((deadline - now).min(MAX_CHUNK))
            }
        }
    }
}

fn ke_wait_for_dispatcher_object(
    memory: &GuestMemory,
    obj_addr: u32,
    timeout_ptr: u32,
    log_tag: &str,
) -> u32 {
    if dispatcher_is_signaled(memory, obj_addr) {
        satisfy_dispatcher_object(memory, obj_addr);
        return STATUS_WAIT_0;
    }

    let deadline = wait_deadline(memory, timeout_ptr);
    if matches!(deadline, WaitDeadline::Poll) {
        return STATUS_TIMEOUT;
    }

    let waiter = dispatcher_waiter(obj_addr);
    let mut generation = waiter.generation.lock().unwrap_or_else(|e| e.into_inner());
    let start = Instant::now();
    let mut logged_wait = false;
    loop {
        if dispatcher_is_signaled(memory, obj_addr) {
            satisfy_dispatcher_object(memory, obj_addr);
            return STATUS_WAIT_0;
        }

        let Some(chunk) = wait_chunk(deadline) else {
            return STATUS_TIMEOUT;
        };

        if !logged_wait && start.elapsed() >= Duration::from_secs(1) {
            logged_wait = true;
            crate::xbox::emulator::debug_log(&format!(
                "[KERNEL-WAIT] {} object=0x{:08X} still unsignaled after {}ms type={} signal={}",
                log_tag,
                obj_addr,
                start.elapsed().as_millis(),
                if dispatcher_obj_valid(obj_addr) {
                    dispatcher_type(memory, obj_addr)
                } else {
                    0xFF
                },
                if dispatcher_obj_valid(obj_addr) {
                    memory.read_u32(obj_addr + 0x04)
                } else {
                    0
                }
            ));
        }

        let wait_result = waiter
            .condvar
            .wait_timeout(generation, chunk)
            .unwrap_or_else(|e| e.into_inner());
        generation = wait_result.0;
    }
}

fn ke_wait_for_multiple_dispatcher_objects(
    memory: &GuestMemory,
    objects_ptr: u32,
    count: u32,
    wait_type: u32,
    timeout_ptr: u32,
) -> u32 {
    let deadline = wait_deadline(memory, timeout_ptr);
    let waiter = dispatcher_waiter(0);
    let mut generation = waiter.generation.lock().unwrap_or_else(|e| e.into_inner());

    loop {
        let mut all_signaled = true;
        let mut any_signaled = false;
        let mut first_signaled = 0u32;

        for i in 0..count {
            let obj_ptr = memory.read_u32(objects_ptr + i * 4);
            let signaled = dispatcher_is_signaled(memory, obj_ptr);
            if signaled && !any_signaled {
                first_signaled = i;
            }
            any_signaled |= signaled;
            all_signaled &= signaled;
        }

        let satisfied = if wait_type == 0 {
            all_signaled
        } else {
            any_signaled
        };
        if satisfied {
            let end = if wait_type == 0 {
                count
            } else {
                first_signaled + 1
            };
            for i in 0..end {
                let obj_ptr = memory.read_u32(objects_ptr + i * 4);
                satisfy_dispatcher_object(memory, obj_ptr);
            }
            return STATUS_WAIT_0 + first_signaled;
        }

        if matches!(deadline, WaitDeadline::Poll) {
            return STATUS_TIMEOUT;
        }

        let Some(chunk) = wait_chunk(deadline) else {
            return STATUS_TIMEOUT;
        };

        let wait_result = waiter
            .condvar
            .wait_timeout(generation, chunk)
            .unwrap_or_else(|e| e.into_inner());
        generation = wait_result.0;
    }
}

fn console_game_region_for_title(game_region: u32) -> u32 {
    if let Ok(value) = std::env::var("RUSTEMU_XBOX_REGION") {
        match value.trim().to_ascii_lowercase().as_str() {
            "na" | "ntsc-u" | "usa" => return XC_GAME_REGION_NA,
            "jp" | "japan" | "ntsc-j" => return XC_GAME_REGION_JAPAN,
            "pal" | "eu" | "europe" | "restofworld" => return XC_GAME_REGION_RESTOFWORLD,
            "ignore" | "all" | "any" => {
                return XC_GAME_REGION_NA | XC_GAME_REGION_JAPAN | XC_GAME_REGION_RESTOFWORLD;
            }
            _ => {}
        }
    }

    let region = if game_region == 0 {
        XC_GAME_REGION_NA
    } else {
        game_region
    };
    if region & XC_GAME_REGION_RESTOFWORLD != 0
        && region & (XC_GAME_REGION_NA | XC_GAME_REGION_JAPAN) == 0
    {
        XC_GAME_REGION_RESTOFWORLD
    } else if region & XC_GAME_REGION_JAPAN != 0 && region & XC_GAME_REGION_NA == 0 {
        XC_GAME_REGION_JAPAN
    } else {
        XC_GAME_REGION_NA
    }
}

fn factory_av_region_for_console(game_region: u32) -> u32 {
    match console_game_region_for_title(game_region) {
        XC_GAME_REGION_RESTOFWORLD => AV_STANDARD_PAL_I | AV_FLAGS_50HZ,
        XC_GAME_REGION_JAPAN => AV_STANDARD_NTSC_J | AV_FLAGS_60HZ,
        _ => AV_STANDARD_NTSC_M | AV_FLAGS_60HZ,
    }
}

fn av_capabilities_for_console(game_region: u32) -> u32 {
    AV_PACK_HDTV | AV_FLAGS_HDTV_480P | factory_av_region_for_console(game_region)
}

/// Dispatch HAL/RTL/misc kernel ordinals.
/// Returns Some((result, eax)) if handled, None if not ours.
pub fn dispatch(
    ordinal: u32,
    args: &[u32; 12],
    state: &mut KernelState,
    memory: &GuestMemory,
    guest_eax: &mut u32,
    guest_edx: &mut u32,
    guest_ecx: &mut u32,
) -> Option<(KernelResult, u32)> {
    use super::ordinals;
    match ordinal {
        // ---- Debug ----
        ordinals::DbgPrint => {
            let fmt_ptr = args[0];
            let msg = read_guest_string(memory, fmt_ptr, 256);
            crate::xbox::emulator::debug_log(&format!("[DbgPrint] \"{}\"", msg));
            Some((KernelResult::Handled, 0))
        }
        ordinals::DbgBreakPoint => Some((KernelResult::Handled, 0)),
        ordinals::DbgBreakPointWithStatus => Some((KernelResult::Handled, 0)),
        ordinals::DbgLoadImageSymbols | ordinals::DbgUnLoadImageSymbols => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::DbgPrompt => Some((KernelResult::Handled, 0)),
        ordinals::RtlAssert => Some((KernelResult::Handled, 0)),

        // ---- AV ----
        ordinals::AvGetSavedDataAddress => Some((KernelResult::Handled, 0)),
        ordinals::AvSetSavedDataAddress => Some((KernelResult::Handled, 0)),
        ordinals::AvSendTVEncoderOption => {
            // AvSendTVEncoderOption(RegisterBase, Option, Param, Result)
            let option = args[1];
            let readback_ptr = args[3];
            if readback_ptr != 0 && readback_ptr < 0x1000_0000 {
                let val = match option {
                    6 => av_capabilities_for_console(state.xbe_game_region),
                    13 => 0, // AV_OPTION_QUERY_MODE
                    15 => 0, // AV_OPTION_GUESS_FIELD: even field
                    16 => 0, // AV_QUERY_ENCODER_TYPE
                    17 => 0, // AV_QUERY_MODE_TABLE_VERSION
                    _ => 0,
                };
                memory.write_u32(readback_ptr, val);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::AvSetDisplayMode => {
            // args: width, height, bpp, flags, pitch, fb_addr
            let fb_addr = args[5];
            if fb_addr != 0 {
                crate::xbox::aot::nv2a::shadow_write(0x60_0800, fb_addr);
            }
            Some((KernelResult::Handled, 0))
        }

        // ---- HAL ----
        ordinals::HalReturnToFirmware => {
            let reason = args[0];
            if reason == 2 {
                // Reason 2 = XDK CRT reboot cycle.
                // XWriteTitleInfoAndRebootA wrote launch data to LaunchDataPage,
                // then calls HalReturnToFirmware(2). On real Xbox this reboots.
                //
                // DON'T reboot. Just return normally. XWriteTitleInfoAndRebootA
                // will RET back to XapiInitProcess, which continues past the
                // reboot call. The launch data was already written to LaunchDataPage
                // before this call, so downstream code sees valid data.
                //
                // QuickReboot (re-enter from entry point) fails because the XDK
                // says "all memory is cleared except 4KB" on reboot — keeping all
                // memory intact causes the CRT to spin on stale globals.
                crate::xbox::emulator::debug_log(
                    "HalReturnToFirmware(2) — NOP return (skip reboot, let CRT continue)",
                );
                Some((KernelResult::Handled, 0))
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "HalReturnToFirmware({}) called — halting worker (never returns on real hardware)",
                    reason
                ));
                Some((KernelResult::Halt, 0))
            }
        }
        ordinals::HalReadSMCTrayState => Some((KernelResult::Handled, 0x10)), // tray closed
        ordinals::HalReadSMBusValue => Some((KernelResult::Handled, 0)),
        ordinals::HalWriteSMBusValue => Some((KernelResult::Handled, 0)),
        ordinals::HalReadWritePCISpace => Some((KernelResult::Handled, 0)),
        ordinals::HalRegisterShutdownNotification => Some((KernelResult::Handled, 0)),
        ordinals::HalClearSoftwareInterrupt => Some((KernelResult::Handled, 0)),
        ordinals::HalDisableSystemInterrupt => Some((KernelResult::Handled, 0)),
        ordinals::HalEnableSystemInterrupt => Some((KernelResult::Handled, 0)),
        ordinals::HalGetInterruptVector => {
            // HalGetInterruptVector(BusInterruptLevel, pIrql) → vector
            // Real kernel (v4627) at 0x80015044:
            //   vector = BusInterruptLevel + 0x30
            //   if 0x30 <= vector <= 0x4A: *pIrql = 0x4A - vector; return vector
            //   else: return 0
            let bus_level = args[0];
            let p_irql = args[1];
            let vector = bus_level.wrapping_add(0x30);
            if vector >= 0x30 && vector <= 0x4A {
                if p_irql != 0
                    && (p_irql < 0x2000_0000 || (p_irql >= 0x8000_0000 && p_irql < 0xA000_0000))
                {
                    memory.write_u8(p_irql, (0x4A - vector) as u8);
                }
                Some((KernelResult::Handled, vector))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::HalRequestSoftwareInterrupt => Some((KernelResult::Handled, 0)),
        ordinals::HalIsResetOrShutdownPending => Some((KernelResult::Handled, 0)),
        ordinals::HalInitiateShutdown => Some((KernelResult::Handled, 1)),
        ordinals::HalEnableSecureTrayEject => Some((KernelResult::Handled, 0)),
        ordinals::HalWriteSMCScratchRegister => Some((KernelResult::Handled, 0)),

        // ---- Timing ----
        ordinals::KeQueryPerformanceCounter => {
            state.perf_counter += PERF_INCR;
            *guest_eax = state.perf_counter as u32;
            *guest_edx = (state.perf_counter >> 32) as u32;
            Some((KernelResult::Handled, state.perf_counter as u32))
        }
        ordinals::KeQueryPerformanceFrequency => Some((KernelResult::Handled, PERF_FREQ)),
        ordinals::KeQueryInterruptTime => {
            state.perf_counter += PERF_INCR;
            *guest_eax = state.perf_counter as u32;
            *guest_edx = (state.perf_counter >> 32) as u32;
            static KQIT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = KQIT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if menu_wait_trace_log(n) {
                crate::xbox::emulator::debug_log(&format!(
                    "[MENU-WAIT] KeQueryInterruptTime #{} ret=0x{:016X}",
                    n, state.perf_counter
                ));
            }
            Some((KernelResult::Handled, state.perf_counter as u32))
        }
        ordinals::KeQuerySystemTime => {
            let ptr = args[0];
            if ptr != 0 {
                state.system_time += FAKE_TIME_INCR;
                memory.write_u32(ptr, state.system_time as u32);
                memory.write_u32(ptr + 4, (state.system_time >> 32) as u32);
            }
            static KQST_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = KQST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if menu_wait_trace_log(n) {
                crate::xbox::emulator::debug_log(&format!(
                    "[MENU-WAIT] KeQuerySystemTime #{} ptr=0x{:08X} value=0x{:016X}",
                    n, ptr, state.system_time
                ));
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeStallExecutionProcessor => Some((KernelResult::Handled, 0)),

        // ---- Strings ----
        ordinals::RtlInitAnsiString => {
            let dest = args[0];
            let src = args[1];
            if dest != 0 {
                if src != 0 {
                    let len = guest_strlen(memory, src, 512) as u16;
                    memory.write_u16(dest, len); // Length
                    memory.write_u16(dest + 2, len + 1); // MaximumLength
                    memory.write_u32(dest + 4, src); // Buffer
                } else {
                    // Null source — zero the ANSI_STRING (matches C++)
                    memory.write_u16(dest, 0);
                    memory.write_u16(dest + 2, 0);
                    memory.write_u32(dest + 4, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlInitUnicodeString => {
            let dest = args[0];
            let src = args[1];
            if dest != 0 {
                if src != 0 {
                    let len = guest_wstrlen(memory, src, 512) as u16;
                    let byte_len = len * 2;
                    memory.write_u16(dest, byte_len);
                    memory.write_u16(dest + 2, byte_len + 2);
                    memory.write_u32(dest + 4, src);
                } else {
                    memory.write_u16(dest, 0);
                    memory.write_u16(dest + 2, 0);
                    memory.write_u32(dest + 4, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlAnsiStringToUnicodeString => {
            // RtlAnsiStringToUnicodeString(PUNICODE_STRING Dest, PANSI_STRING Src, BOOLEAN AllocDest)
            let dest = args[0];
            let src = args[1];
            let alloc = args[2] != 0;
            if dest == 0 || src == 0 {
                return Some((KernelResult::Handled, 0xC000_000D)); // STATUS_INVALID_PARAMETER
            }
            let src_len = memory.read_u16(src) as u32; // Length in bytes
            let src_buf = memory.read_u32(src + 4);
            let unicode_len = src_len * 2; // Each ANSI char → 2 bytes Unicode
            if alloc {
                // Allocate buffer from bump pool
                let buf_size = align_up(unicode_len + 2, 16);
                let buf = bump_alloc_hal(&mut state.bump_pool, buf_size, memory);
                if buf == 0 {
                    return Some((KernelResult::Handled, 0xC000_0017)); // STATUS_NO_MEMORY
                }
                memory.write_u16(dest, unicode_len as u16);
                memory.write_u16(dest + 2, (unicode_len + 2) as u16);
                memory.write_u32(dest + 4, buf);
                // Convert: zero-extend each byte to u16
                for i in 0..src_len {
                    let b = memory.read_u8(src_buf + i) as u16;
                    memory.write_u16(buf + i * 2, b);
                }
                memory.write_u16(buf + unicode_len, 0); // null terminator
            } else {
                let dest_max = memory.read_u16(dest + 2) as u32;
                let dest_buf = memory.read_u32(dest + 4);
                let copy_len = unicode_len.min(dest_max.saturating_sub(2));
                for i in 0..(copy_len / 2) {
                    let b = memory.read_u8(src_buf + i) as u16;
                    memory.write_u16(dest_buf + i * 2, b);
                }
                memory.write_u16(dest, copy_len as u16);
                if copy_len + 2 <= dest_max {
                    memory.write_u16(dest_buf + copy_len, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUnicodeStringToAnsiString => {
            // RtlUnicodeStringToAnsiString(PANSI_STRING Dest, PUNICODE_STRING Src, BOOLEAN AllocDest)
            let dest = args[0];
            let src = args[1];
            let alloc = args[2] != 0;
            if dest == 0 || src == 0 {
                return Some((KernelResult::Handled, 0xC000_000D));
            }
            let src_len = memory.read_u16(src) as u32; // Length in bytes (Unicode)
            let src_buf = memory.read_u32(src + 4);
            let ansi_len = src_len / 2; // Each u16 → 1 byte
            if alloc {
                let buf_size = align_up(ansi_len + 1, 16);
                let buf = bump_alloc_hal(&mut state.bump_pool, buf_size, memory);
                if buf == 0 {
                    return Some((KernelResult::Handled, 0xC000_0017));
                }
                memory.write_u16(dest, ansi_len as u16);
                memory.write_u16(dest + 2, (ansi_len + 1) as u16);
                memory.write_u32(dest + 4, buf);
                for i in 0..ansi_len {
                    let w = memory.read_u16(src_buf + i * 2);
                    memory.write_u8(buf + i, w as u8); // truncate to low byte
                }
                memory.write_u8(buf + ansi_len, 0);
            } else {
                let dest_max = memory.read_u16(dest + 2) as u32;
                let dest_buf = memory.read_u32(dest + 4);
                let copy_len = ansi_len.min(dest_max.saturating_sub(1));
                for i in 0..copy_len {
                    let w = memory.read_u16(src_buf + i * 2);
                    memory.write_u8(dest_buf + i, w as u8);
                }
                memory.write_u16(dest, copy_len as u16);
                if copy_len + 1 <= dest_max {
                    memory.write_u8(dest_buf + copy_len, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlFreeAnsiString | ordinals::RtlFreeUnicodeString => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlCopyString => {
            // RtlCopyString(PSTRING Dest, PSTRING Src) -> void
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let src_len = memory.read_u16(src) as u32;
                let dest_max = memory.read_u16(dest + 2) as u32;
                let src_buf = memory.read_u32(src + 4);
                let dest_buf = memory.read_u32(dest + 4);
                let copy_len = src_len.min(dest_max);
                if src_buf != 0 && dest_buf != 0 {
                    for i in 0..copy_len {
                        let b = memory.read_u8(src_buf + i);
                        memory.write_u8(dest_buf + i, b);
                    }
                }
                memory.write_u16(dest, copy_len as u16);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlCopyUnicodeString => {
            // RtlCopyUnicodeString(PUNICODE_STRING Dest, PUNICODE_STRING Src) -> void
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let src_len = memory.read_u16(src) as u32; // Length in bytes
                let dest_max = memory.read_u16(dest + 2) as u32; // MaximumLength
                let src_buf = memory.read_u32(src + 4);
                let dest_buf = memory.read_u32(dest + 4);
                if src_buf != 0 && dest_buf != 0 {
                    let copy_len = src_len.min(dest_max);
                    for i in 0..copy_len {
                        let b = memory.read_u8(src_buf + i);
                        memory.write_u8(dest_buf + i, b);
                    }
                    memory.write_u16(dest, copy_len as u16);
                    // Null-terminate if space
                    if copy_len + 2 <= dest_max {
                        memory.write_u16(dest_buf + copy_len, 0);
                    }
                }
            } else if dest != 0 && src == 0 {
                // Null source — set Length to 0
                memory.write_u16(dest, 0);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlAppendStringToString => {
            // RtlAppendStringToString(PSTRING Dest, PSTRING Src) -> NTSTATUS
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let dest_len = memory.read_u16(dest) as u32;
                let dest_max = memory.read_u16(dest + 2) as u32;
                let dest_buf = memory.read_u32(dest + 4);
                let src_len = memory.read_u16(src) as u32;
                let src_buf = memory.read_u32(src + 4);
                if dest_len + src_len > dest_max {
                    return Some((KernelResult::Handled, 0xC000_0106)); // STATUS_BUFFER_TOO_SMALL
                }
                if src_buf != 0 && dest_buf != 0 {
                    for i in 0..src_len {
                        let b = memory.read_u8(src_buf + i);
                        memory.write_u8(dest_buf + dest_len + i, b);
                    }
                }
                memory.write_u16(dest, (dest_len + src_len) as u16);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlAppendUnicodeStringToString => {
            // RtlAppendUnicodeStringToString(PUNICODE_STRING Dest, PUNICODE_STRING Src) -> NTSTATUS
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let dest_len = memory.read_u16(dest) as u32; // bytes
                let dest_max = memory.read_u16(dest + 2) as u32;
                let dest_buf = memory.read_u32(dest + 4);
                let src_len = memory.read_u16(src) as u32;
                let src_buf = memory.read_u32(src + 4);
                let needed = dest_len + src_len;
                if needed > dest_max {
                    return Some((KernelResult::Handled, 0xC000_0106)); // STATUS_BUFFER_TOO_SMALL
                }
                if src_buf != 0 && dest_buf != 0 {
                    for i in 0..src_len {
                        let b = memory.read_u8(src_buf + i);
                        memory.write_u8(dest_buf + dest_len + i, b);
                    }
                }
                memory.write_u16(dest, needed as u16);
                // Null-terminate if space
                if needed + 2 <= dest_max {
                    memory.write_u16(dest_buf + needed, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlAppendUnicodeToString => {
            // RtlAppendUnicodeToString(PUNICODE_STRING Dest, PCWSTR Src) -> NTSTATUS
            let dest = args[0];
            let src = args[1]; // null-terminated wide string
            if dest != 0 && src != 0 {
                let dest_len = memory.read_u16(dest) as u32;
                let dest_max = memory.read_u16(dest + 2) as u32;
                let dest_buf = memory.read_u32(dest + 4);
                // Measure source string
                let src_len = guest_wstrlen(memory, src, 512) as u32 * 2; // bytes
                let needed = dest_len + src_len;
                if needed > dest_max {
                    return Some((KernelResult::Handled, 0xC000_0106));
                }
                if dest_buf != 0 {
                    for i in 0..src_len {
                        let b = memory.read_u8(src + i);
                        memory.write_u8(dest_buf + dest_len + i, b);
                    }
                }
                memory.write_u16(dest, needed as u16);
                if needed + 2 <= dest_max {
                    memory.write_u16(dest_buf + needed, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlCompareString => {
            // RtlCompareString(PANSI_STRING s1, PANSI_STRING s2, BOOLEAN caseInsensitive) -> LONG
            let s1_ptr = args[0];
            let s2_ptr = args[1];
            let case_insensitive = args[2] != 0;
            if s1_ptr == 0 || s2_ptr == 0 {
                Some((KernelResult::Handled, 0))
            } else {
                let len1 = memory.read_u16(s1_ptr) as u32;
                let len2 = memory.read_u16(s2_ptr) as u32;
                let buf1 = memory.read_u32(s1_ptr + 4);
                let buf2 = memory.read_u32(s2_ptr + 4);
                let cmp_len = len1.min(len2);
                let result = guest_strncmp(memory, buf1, buf2, cmp_len, case_insensitive);
                let ret = if result != 0 {
                    result
                } else if len1 < len2 {
                    -1i32
                } else if len1 > len2 {
                    1i32
                } else {
                    0
                };
                Some((KernelResult::Handled, ret as u32))
            }
        }
        ordinals::RtlCompareUnicodeString => {
            // RtlCompareUnicodeString(s1, s2, caseInsensitive) -> LONG
            let s1_ptr = args[0];
            let s2_ptr = args[1];
            let case_insensitive = args[2] != 0;
            if s1_ptr == 0 || s2_ptr == 0 {
                Some((KernelResult::Handled, 0))
            } else {
                let len1 = memory.read_u16(s1_ptr) as u32; // Length in bytes
                let len2 = memory.read_u16(s2_ptr) as u32;
                let buf1 = memory.read_u32(s1_ptr + 4);
                let buf2 = memory.read_u32(s2_ptr + 4);
                let cmp_chars = (len1.min(len2)) / 2;
                let mut result: i32 = 0;
                for i in 0..cmp_chars {
                    let mut c1 = memory.read_u16(buf1 + i * 2);
                    let mut c2 = memory.read_u16(buf2 + i * 2);
                    if case_insensitive {
                        if (0x0041..=0x005A).contains(&c1) {
                            c1 += 0x20;
                        }
                        if (0x0041..=0x005A).contains(&c2) {
                            c2 += 0x20;
                        }
                    }
                    if c1 != c2 {
                        result = (c1 as i32) - (c2 as i32);
                        break;
                    }
                }
                if result == 0 {
                    result = (len1 as i32) - (len2 as i32);
                }
                Some((KernelResult::Handled, result as u32))
            }
        }
        ordinals::RtlEqualString => {
            // RtlEqualString(PANSI_STRING s1, PANSI_STRING s2, BOOLEAN caseInsensitive) -> BOOLEAN
            let s1_ptr = args[0];
            let s2_ptr = args[1];
            let case_insensitive = args[2] != 0;
            if s1_ptr == 0 || s2_ptr == 0 {
                Some((KernelResult::Handled, 0))
            } else {
                let len1 = memory.read_u16(s1_ptr) as u32;
                let len2 = memory.read_u16(s2_ptr) as u32;
                // Log first 50 comparisons to find what the game searches for
                {
                    static EQ_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = EQ_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 50 {
                        let buf1 = memory.read_u32(s1_ptr + 4);
                        let buf2 = memory.read_u32(s2_ptr + 4);
                        let s1 = if buf1 != 0 {
                            guest_read_string(memory, buf1, len1.min(64))
                        } else {
                            "<null>".to_string()
                        };
                        let s2 = if buf2 != 0 {
                            guest_read_string(memory, buf2, len2.min(64))
                        } else {
                            "<null>".to_string()
                        };
                        crate::xbox::emulator::debug_log(&format!(
                            "[STRING-CMP] #{} '{}' vs '{}' len={}/{} ci={}",
                            n, s1, s2, len1, len2, case_insensitive
                        ));
                    }
                }
                if len1 != len2 {
                    Some((KernelResult::Handled, 0))
                } else {
                    let buf1 = memory.read_u32(s1_ptr + 4);
                    let buf2 = memory.read_u32(s2_ptr + 4);
                    if buf1 == 0 || buf2 == 0 {
                        Some((KernelResult::Handled, 0))
                    } else {
                        let result = guest_strncmp(memory, buf1, buf2, len1, case_insensitive);
                        Some((KernelResult::Handled, if result == 0 { 1 } else { 0 }))
                    }
                }
            }
        }
        ordinals::RtlEqualUnicodeString => {
            // RtlEqualUnicodeString(s1, s2, caseInsensitive) -> BOOLEAN
            let s1_ptr = args[0];
            let s2_ptr = args[1];
            let case_insensitive = args[2] != 0;
            if s1_ptr == 0 || s2_ptr == 0 {
                Some((KernelResult::Handled, 0))
            } else {
                let len1 = memory.read_u16(s1_ptr) as u32;
                let len2 = memory.read_u16(s2_ptr) as u32;
                if len1 != len2 {
                    Some((KernelResult::Handled, 0)) // FALSE — different lengths
                } else {
                    let buf1 = memory.read_u32(s1_ptr + 4);
                    let buf2 = memory.read_u32(s2_ptr + 4);
                    if buf1 == 0 || buf2 == 0 {
                        return Some((KernelResult::Handled, if buf1 == buf2 { 1 } else { 0 }));
                    }
                    let chars = len1 / 2;
                    let mut equal = true;
                    for i in 0..chars {
                        let mut c1 = memory.read_u16(buf1 + i * 2);
                        let mut c2 = memory.read_u16(buf2 + i * 2);
                        if case_insensitive {
                            if (0x0041..=0x005A).contains(&c1) {
                                c1 += 0x20;
                            }
                            if (0x0041..=0x005A).contains(&c2) {
                                c2 += 0x20;
                            }
                        }
                        if c1 != c2 {
                            equal = false;
                            break;
                        }
                    }
                    Some((KernelResult::Handled, if equal { 1 } else { 0 }))
                }
            }
        }
        ordinals::RtlCreateUnicodeString => Some((KernelResult::Handled, 1)),
        ordinals::RtlCharToInteger
        | ordinals::RtlIntegerToChar
        | ordinals::RtlIntegerToUnicodeString
        | ordinals::RtlUnicodeStringToInteger => Some((KernelResult::Handled, 0)),

        // ---- RTL misc ----
        ordinals::RtlNtStatusToDosError => {
            // NTSTATUS → Win32 error code lookup (matches C++ AOT_KrnlHal.cpp)
            let status = args[0];
            let dos_err = match status {
                0x00000000 => 0,    // STATUS_SUCCESS → ERROR_SUCCESS
                0x00000103 => 997,  // STATUS_PENDING → ERROR_IO_PENDING
                0x80000006 => 18,   // STATUS_NO_MORE_FILES → ERROR_NO_MORE_FILES
                0xC0000001 => 31,   // STATUS_UNSUCCESSFUL → ERROR_GEN_FAILURE
                0xC0000002 => 1,    // STATUS_NOT_IMPLEMENTED → ERROR_INVALID_FUNCTION
                0xC0000005 => 998,  // STATUS_ACCESS_VIOLATION → ERROR_NOACCESS
                0xC000000D => 87,   // STATUS_INVALID_PARAMETER → ERROR_INVALID_PARAMETER
                0xC000000E => 2,    // STATUS_NO_SUCH_DEVICE → ERROR_FILE_NOT_FOUND
                0xC000000F => 2,    // STATUS_NO_SUCH_FILE → ERROR_FILE_NOT_FOUND
                0xC0000017 => 8,    // STATUS_NO_MEMORY → ERROR_NOT_ENOUGH_MEMORY
                0xC0000022 => 5,    // STATUS_ACCESS_DENIED → ERROR_ACCESS_DENIED
                0xC0000034 => 2,    // STATUS_OBJECT_NAME_NOT_FOUND → ERROR_FILE_NOT_FOUND
                0xC0000035 => 183,  // STATUS_OBJECT_NAME_COLLISION → ERROR_ALREADY_EXISTS
                0xC000003A => 3,    // STATUS_OBJECT_PATH_NOT_FOUND → ERROR_PATH_NOT_FOUND
                0xC0000043 => 32,   // STATUS_SHARING_VIOLATION → ERROR_SHARING_VIOLATION
                0xC00000BA => 21,   // STATUS_FILE_IS_A_DIRECTORY → ERROR_NOT_READY
                0xC000010A => 2,    // STATUS_PROCESS_IS_TERMINATING → ERROR_FILE_NOT_FOUND
                0xC0000120 => 1168, // STATUS_CANCELLED → ERROR_CANCELLED
                _ => {
                    if status & 0x80000000 != 0 {
                        31
                    } else {
                        0
                    }
                } // error → GEN_FAILURE, success → SUCCESS
            };
            Some((KernelResult::Handled, dos_err))
        }
        ordinals::RtlSnprintf => {
            // RtlSnprintf(Buffer, BufferSize, Format, ...)
            // Read format string from guest, write truncated to buffer.
            let buf_addr = args[0];
            let buf_size = args[1].min(4096);
            let fmt_addr = args[2];
            if buf_addr != 0
                && buf_addr < 0x1000_0000
                && fmt_addr != 0
                && fmt_addr < 0x1000_0000
                && buf_size > 0
            {
                // Read format string (up to 256 chars)
                let mut fmt = Vec::new();
                for i in 0..256u32 {
                    let b = memory.read_u8(fmt_addr + i);
                    if b == 0 {
                        break;
                    }
                    fmt.push(b);
                }
                // Write format string as-is to buffer (no substitution — good enough for debug strings)
                let copy_len = fmt.len().min((buf_size - 1) as usize);
                for i in 0..copy_len {
                    memory.write_u8(buf_addr + i as u32, fmt[i]);
                }
                memory.write_u8(buf_addr + copy_len as u32, 0); // null terminate
                Some((KernelResult::Handled, copy_len as u32))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::RtlSprintf => {
            // RtlSprintf(Buffer, Format, ...) — no size limit
            let buf_addr = args[0];
            let fmt_addr = args[1];
            if buf_addr != 0 && buf_addr < 0x1000_0000 && fmt_addr != 0 && fmt_addr < 0x1000_0000 {
                let mut fmt = Vec::new();
                for i in 0..256u32 {
                    let b = memory.read_u8(fmt_addr + i);
                    if b == 0 {
                        break;
                    }
                    fmt.push(b);
                }
                for (i, &b) in fmt.iter().enumerate() {
                    memory.write_u8(buf_addr + i as u32, b);
                }
                memory.write_u8(buf_addr + fmt.len() as u32, 0);
                Some((KernelResult::Handled, fmt.len() as u32))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::RtlVsnprintf | ordinals::RtlVsprintf => {
            // va_list variants — same approach: copy format string without substitution
            let buf_addr = args[0];
            let fmt_addr = if ordinal == ordinals::RtlVsnprintf {
                args[2]
            } else {
                args[1]
            };
            let buf_size = if ordinal == ordinals::RtlVsnprintf {
                args[1].min(4096)
            } else {
                4096
            };
            if buf_addr != 0
                && buf_addr < 0x1000_0000
                && fmt_addr != 0
                && fmt_addr < 0x1000_0000
                && buf_size > 0
            {
                let mut fmt = Vec::new();
                for i in 0..256u32 {
                    let b = memory.read_u8(fmt_addr + i);
                    if b == 0 {
                        break;
                    }
                    fmt.push(b);
                }
                let copy_len = fmt.len().min((buf_size - 1) as usize);
                for i in 0..copy_len {
                    memory.write_u8(buf_addr + i as u32, fmt[i]);
                }
                memory.write_u8(buf_addr + copy_len as u32, 0);
                Some((KernelResult::Handled, copy_len as u32))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::RtlFillMemory => {
            let dst = args[0];
            let len = args[1];
            let fill = (args[2] & 0xFF) as u8;
            for i in 0..len.min(0x100000) {
                memory.write_u8(dst + i, fill);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlFillMemoryUlong => {
            let dst = args[0];
            let len = args[1];
            let fill = args[2];
            for i in (0..len.min(0x100000)).step_by(4) {
                memory.write_u32(dst + i, fill);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlZeroMemory => {
            let dst = args[0];
            let len = args[1];
            for i in (0..len.min(0x100000)).step_by(4) {
                memory.write_u32(dst + i, 0);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlMoveMemory => {
            let dst = args[0];
            let src = args[1];
            let len = args[2];
            // Byte-by-byte copy (handles overlap)
            if dst < src {
                for i in 0..len.min(0x100000) {
                    let b = memory.read_u8(src + i);
                    memory.write_u8(dst + i, b);
                }
            } else if dst > src {
                for i in (0..len.min(0x100000)).rev() {
                    let b = memory.read_u8(src + i);
                    memory.write_u8(dst + i, b);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlCompareMemory => {
            let s1 = args[0];
            let s2 = args[1];
            let len = args[2];
            let mut matched = 0u32;
            for i in 0..len.min(0x100000) {
                if memory.read_u8(s1 + i) != memory.read_u8(s2 + i) {
                    break;
                }
                matched += 1;
            }
            Some((KernelResult::Handled, matched))
        }
        ordinals::RtlCompareMemoryUlong => Some((KernelResult::Handled, 0)),
        ordinals::RtlCaptureContext | ordinals::RtlCaptureStackBackTrace => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlRaiseException | ordinals::RtlRaiseStatus | ordinals::RtlRip => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUnwind => Some((KernelResult::Handled, 0)),
        ordinals::RtlMapGenericMask => Some((KernelResult::Handled, 0)),
        ordinals::RtlTimeFieldsToTime => {
            // RtlTimeFieldsToTime(TIME_FIELDS* tf, LARGE_INTEGER* time) -> BOOLEAN
            // TIME_FIELDS: Year(u16), Month(u16), Day(u16), Hour(u16), Minute(u16), Second(u16), Ms(u16), Weekday(u16)
            let tf = args[0];
            let time_ptr = args[1];
            if tf != 0 && time_ptr != 0 {
                let year = memory.read_u16(tf) as i64;
                let month = memory.read_u16(tf + 2) as i64;
                let day = memory.read_u16(tf + 4) as i64;
                let hour = memory.read_u16(tf + 6) as i64;
                let minute = memory.read_u16(tf + 8) as i64;
                let second = memory.read_u16(tf + 10) as i64;
                let ms = memory.read_u16(tf + 12) as i64;
                // Approximate: days since 1601-01-01
                let y = year - 1601;
                let days = y * 365 + y / 4 - y / 100 + y / 400 + (month - 1) * 30 + day - 1;
                let ticks =
                    ((days * 24 + hour) * 3600 + minute * 60 + second) * 10_000_000 + ms * 10_000;
                memory.write_u32(time_ptr, ticks as u32);
                memory.write_u32(time_ptr + 4, (ticks >> 32) as u32);
            }
            Some((KernelResult::Handled, 1)) // TRUE = success
        }
        ordinals::RtlTimeToTimeFields => {
            // RtlTimeToTimeFields(LARGE_INTEGER* time, TIME_FIELDS* tf)
            let time_ptr = args[0];
            let tf = args[1];
            if time_ptr != 0 && tf != 0 {
                let lo = memory.read_u32(time_ptr) as u64;
                let hi = memory.read_u32(time_ptr + 4) as u64;
                let ticks = lo | (hi << 32);
                let total_secs = ticks / 10_000_000;
                let ms = ((ticks % 10_000_000) / 10_000) as u16;
                let total_days = total_secs / 86400;
                let day_secs = total_secs % 86400;
                let hour = (day_secs / 3600) as u16;
                let minute = ((day_secs % 3600) / 60) as u16;
                let second = (day_secs % 60) as u16;
                // Approximate year/month/day from days since 1601
                let y = 1601u16 + (total_days / 365) as u16;
                let mut d = (total_days % 365) as u16;
                let month = (d / 30).min(11) as u16 + 1;
                d = d % 30 + 1;
                memory.write_u16(tf, y);
                memory.write_u16(tf + 2, month);
                memory.write_u16(tf + 4, d);
                memory.write_u16(tf + 6, hour);
                memory.write_u16(tf + 8, minute);
                memory.write_u16(tf + 10, second);
                memory.write_u16(tf + 12, ms);
                memory.write_u16(tf + 14, 0); // weekday
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUlongByteSwap => {
            let v = args[0];
            Some((KernelResult::Handled, v.swap_bytes()))
        }
        ordinals::RtlUshortByteSwap => {
            let v = (args[0] & 0xFFFF) as u16;
            Some((KernelResult::Handled, v.swap_bytes() as u32))
        }
        ordinals::RtlUpperChar => {
            let c = (args[0] & 0xFF) as u8;
            let upper = if (b'a'..=b'z').contains(&c) {
                c - 32
            } else {
                c
            };
            Some((KernelResult::Handled, upper as u32))
        }
        ordinals::RtlLowerChar => {
            let c = (args[0] & 0xFF) as u8;
            let lower = if (b'A'..=b'Z').contains(&c) {
                c + 32
            } else {
                c
            };
            Some((KernelResult::Handled, lower as u32))
        }
        ordinals::RtlUpcaseUnicodeChar => {
            let c = args[0] as u16;
            let upper = if (0x0061..=0x007A).contains(&c) {
                c - 0x20
            } else {
                c
            };
            Some((KernelResult::Handled, upper as u32))
        }
        ordinals::RtlDowncaseUnicodeChar => {
            let c = args[0] as u16;
            let lower = if (0x0041..=0x005A).contains(&c) {
                c + 0x20
            } else {
                c
            };
            Some((KernelResult::Handled, lower as u32))
        }
        ordinals::RtlUpperString => {
            // RtlUpperString(PSTRING Dest, PSTRING Src) -> void
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let src_len = memory.read_u16(src) as u32;
                let src_buf = memory.read_u32(src + 4);
                let dest_buf = memory.read_u32(dest + 4);
                let dest_max = memory.read_u16(dest + 2) as u32;
                let copy_len = src_len.min(dest_max);
                if src_buf != 0 && dest_buf != 0 {
                    for i in 0..copy_len {
                        let b = memory.read_u8(src_buf + i);
                        let upper = if (b'a'..=b'z').contains(&b) {
                            b - 32
                        } else {
                            b
                        };
                        memory.write_u8(dest_buf + i, upper);
                    }
                }
                memory.write_u16(dest, copy_len as u16);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUpcaseUnicodeString => {
            // RtlUpcaseUnicodeString(PUNICODE_STRING Dest, PUNICODE_STRING Src, BOOLEAN Alloc)
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let src_len = memory.read_u16(src) as u32;
                let src_buf = memory.read_u32(src + 4);
                let dest_buf = if args[2] != 0 {
                    // Allocate
                    let buf_size = align_up(src_len + 2, 16);
                    let buf = bump_alloc_hal(&mut state.bump_pool, buf_size, memory);
                    memory.write_u16(dest, src_len as u16);
                    memory.write_u16(dest + 2, (src_len + 2) as u16);
                    memory.write_u32(dest + 4, buf);
                    buf
                } else {
                    memory.read_u32(dest + 4)
                };
                if src_buf != 0 && dest_buf != 0 {
                    let chars = src_len / 2;
                    for i in 0..chars {
                        let c = memory.read_u16(src_buf + i * 2);
                        let upper = if (0x0061..=0x007A).contains(&c) {
                            c - 0x20
                        } else {
                            c
                        };
                        memory.write_u16(dest_buf + i * 2, upper);
                    }
                    memory.write_u16(dest, src_len as u16);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlDowncaseUnicodeString => {
            // RtlDowncaseUnicodeString(PUNICODE_STRING Dest, PUNICODE_STRING Src, BOOLEAN Alloc)
            let dest = args[0];
            let src = args[1];
            if dest != 0 && src != 0 {
                let src_len = memory.read_u16(src) as u32;
                let src_buf = memory.read_u32(src + 4);
                let dest_buf = if args[2] != 0 {
                    let buf_size = align_up(src_len + 2, 16);
                    let buf = bump_alloc_hal(&mut state.bump_pool, buf_size, memory);
                    memory.write_u16(dest, src_len as u16);
                    memory.write_u16(dest + 2, (src_len + 2) as u16);
                    memory.write_u32(dest + 4, buf);
                    buf
                } else {
                    memory.read_u32(dest + 4)
                };
                if src_buf != 0 && dest_buf != 0 {
                    let chars = src_len / 2;
                    for i in 0..chars {
                        let c = memory.read_u16(src_buf + i * 2);
                        let lower = if (0x0041..=0x005A).contains(&c) {
                            c + 0x20
                        } else {
                            c
                        };
                        memory.write_u16(dest_buf + i * 2, lower);
                    }
                    memory.write_u16(dest, src_len as u16);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlMultiByteToUnicodeN => {
            // RtlMultiByteToUnicodeN(UnicodeString, MaxBytesInUnicode, *BytesInUnicode, MultiByteString, BytesInMultiByte)
            let uni_buf = args[0];
            let max_bytes = args[1];
            let bytes_out = args[2]; // optional
            let mb_buf = args[3];
            let mb_len = args[4];
            let chars = (mb_len).min(max_bytes / 2);
            for i in 0..chars {
                let b = memory.read_u8(mb_buf + i) as u16;
                memory.write_u16(uni_buf + i * 2, b);
            }
            if bytes_out != 0 {
                memory.write_u32(bytes_out, chars * 2);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlMultiByteToUnicodeSize => {
            // RtlMultiByteToUnicodeSize(*BytesInUnicode, MultiByteString, BytesInMultiByte)
            let bytes_out = args[0];
            let mb_len = args[2];
            if bytes_out != 0 {
                memory.write_u32(bytes_out, mb_len * 2);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUnicodeToMultiByteN => {
            // RtlUnicodeToMultiByteN(MultiByteString, MaxBytesInMultiByte, *BytesInMultiByte, UnicodeString, BytesInUnicode)
            let mb_buf = args[0];
            let max_bytes = args[1];
            let bytes_out = args[2]; // optional
            let uni_buf = args[3];
            let uni_len = args[4]; // bytes
            let chars = (uni_len / 2).min(max_bytes);
            for i in 0..chars {
                let w = memory.read_u16(uni_buf + i * 2);
                memory.write_u8(mb_buf + i, w as u8); // truncate to low byte
            }
            if bytes_out != 0 {
                memory.write_u32(bytes_out, chars);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUnicodeToMultiByteSize => {
            // RtlUnicodeToMultiByteSize(*BytesInMultiByte, UnicodeString, BytesInUnicode)
            let bytes_out = args[0];
            let uni_len = args[2];
            if bytes_out != 0 {
                memory.write_u32(bytes_out, uni_len / 2);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlUpcaseUnicodeToMultiByteN => Some((KernelResult::Handled, 0)),
        ordinals::RtlWalkFrameChain => Some((KernelResult::Handled, 0)),
        ordinals::RtlGetCallersAddress => Some((KernelResult::Handled, 0)),
        ordinals::RtlExtendedIntegerMultiply => {
            // RtlExtendedIntegerMultiply(LARGE_INTEGER multiplicand, LONG multiplier)
            // Args: [0]=lo, [1]=hi, [2]=multiplier. Returns LARGE_INTEGER in EDX:EAX.
            let lo = args[0] as u64;
            let hi = args[1] as u64;
            let multiplicand = (hi << 32) | lo;
            let multiplier = args[2] as i32 as i64;
            let result = (multiplicand as i64).wrapping_mul(multiplier) as u64;
            *guest_eax = result as u32;
            *guest_edx = (result >> 32) as u32;
            Some((KernelResult::Handled, result as u32))
        }
        ordinals::RtlExtendedLargeIntegerDivide => {
            // RtlExtendedLargeIntegerDivide(LARGE_INTEGER dividend, ULONG divisor, PULONG remainder)
            // Args: [0]=lo, [1]=hi, [2]=divisor, [3]=remainder_ptr. Returns LARGE_INTEGER in EDX:EAX.
            let lo = args[0] as u64;
            let hi = args[1] as u64;
            let dividend = (hi << 32) | lo;
            let divisor = args[2] as u64;
            if divisor == 0 {
                // Division by zero — return 0
                *guest_eax = 0;
                *guest_edx = 0;
                Some((KernelResult::Handled, 0))
            } else {
                let quotient = dividend / divisor;
                let remainder = (dividend % divisor) as u32;
                if args[3] != 0 {
                    memory.write_u32(args[3], remainder);
                }
                *guest_eax = quotient as u32;
                *guest_edx = (quotient >> 32) as u32;
                Some((KernelResult::Handled, quotient as u32))
            }
        }
        ordinals::RtlExtendedMagicDivide => {
            // RtlExtendedMagicDivide(LARGE_INTEGER dividend, LARGE_INTEGER magic, CCHAR shift)
            // Args: [0]=div_lo, [1]=div_hi, [2]=mag_lo, [3]=mag_hi, [4]=shift
            let div_lo = args[0] as u64;
            let div_hi = args[1] as u64;
            let dividend = ((div_hi << 32) | div_lo) as i64;
            let mag_lo = args[2] as u64;
            let mag_hi = args[3] as u64;
            let magic = ((mag_hi << 32) | mag_lo) as i64;
            let shift = args[4];
            // Result = (dividend * magic) >> (64 + shift)
            let product = (dividend as i128).wrapping_mul(magic as i128);
            let result = (product >> (64 + shift)) as i64 as u64;
            *guest_eax = result as u32;
            *guest_edx = (result >> 32) as u32;
            Some((KernelResult::Handled, result as u32))
        }

        // ---- Critical sections (RTL) ----
        // Real Xbox kernel RTL_CRITICAL_SECTION layout (0x1C = 28 bytes):
        //   +0x00: DISPATCHER_HEADER — Type(u8), AbsoluteInsert(u8), Size(u16)
        //   +0x04: SignalState (LONG)
        //   +0x08: WaitListHead (LIST_ENTRY, 8 bytes — Flink/Blink)
        //   +0x10: LockCount (LONG, -1 = unlocked)
        //   +0x14: RecursionCount (LONG)
        //   +0x18: OwningThread (HANDLE)
        // Total: 0x1C = 28 bytes. Do NOT write past +0x18 — adjacent memory!
        ordinals::RtlInitializeCriticalSection => {
            // Real kernel (v4627) at 0x80023487:
            //   [+0x00] Type = 1, [+0x02] Size = 4, [+0x04] SignalState = 0
            //   [+0x08/+0x0C] WaitListHead self-referencing
            //   [+0x10] LockCount = -1, [+0x14] RecursionCount = 0, [+0x18] OwningThread = 0
            let cs = args[0];
            if cs != 0 {
                memory.write_u8(cs, 1); // Type = 1
                memory.write_u8(cs + 1, 0); // (padding)
                memory.write_u8(cs + 2, 4); // Size = 4
                memory.write_u8(cs + 3, 0); // (padding)
                memory.write_u32(cs + 0x04, 0); // SignalState = 0
                memory.write_u32(cs + 0x08, cs + 0x08); // Flink = &WaitListHead
                memory.write_u32(cs + 0x0C, cs + 0x08); // Blink = &WaitListHead
                memory.write_u32(cs + 0x10, 0xFFFF_FFFF); // LockCount = -1 (unlocked)
                memory.write_u32(cs + 0x14, 0); // RecursionCount = 0
                memory.write_u32(cs + 0x18, 0); // OwningThread = NULL
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlEnterCriticalSection | ordinals::RtlEnterCriticalSectionAndRegion => {
            // On Xbox, address 0 is mapped (TEB/zero page), so cs=0 is valid.
            // Game passes CS=0x0 during path processing and expects side effects.
            let cs = args[0];
            let current_thread: u32 = 0x80D0_0100;
            // InterlockedIncrement(&LockCount)
            let lock = memory.read_u32(cs + 0x10) as i32;
            let new_lock = lock.wrapping_add(1);
            memory.write_u32(cs + 0x10, new_lock as u32);
            if new_lock == 0 {
                // Was -1 (unlocked) → now 0 (locked by us)
                memory.write_u32(cs + 0x18, current_thread); // OwningThread
                memory.write_u32(cs + 0x14, 1); // RecursionCount = 1
            } else {
                // Already locked — check if recursive (same thread)
                let owner = memory.read_u32(cs + 0x18);
                if owner == current_thread {
                    // Same thread — increment recursion
                    let rec = memory.read_u32(cs + 0x14);
                    memory.write_u32(cs + 0x14, rec.wrapping_add(1));
                } else {
                    // Different thread would block — since we're single-threaded
                    // in the interpreter, just take ownership
                    memory.write_u32(cs + 0x18, current_thread);
                    memory.write_u32(cs + 0x14, 1);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlLeaveCriticalSection | ordinals::RtlLeaveCriticalSectionAndRegion => {
            // Real kernel (v4627) at 0x800231D0:
            //   dec RecursionCount; if != 0 → still recursed, just dec LockCount
            //   if RecursionCount == 0 → clear OwningThread, dec LockCount
            // On Xbox, cs=0 is valid (zero page is mapped TEB memory)
            let cs = args[0];
            let rec = memory.read_u32(cs + 0x14) as i32;
            let new_rec = rec.wrapping_sub(1);
            memory.write_u32(cs + 0x14, new_rec as u32);
            if new_rec == 0 {
                // Fully released — clear owner then dec LockCount
                memory.write_u32(cs + 0x18, 0); // OwningThread = NULL
            }
            let lock = memory.read_u32(cs + 0x10) as i32;
            memory.write_u32(cs + 0x10, lock.wrapping_sub(1) as u32);
            Some((KernelResult::Handled, 0))
        }
        ordinals::RtlTryEnterCriticalSection => {
            // On Xbox, cs=0 is valid (zero page is mapped TEB memory)
            let cs = args[0];
            let current_thread: u32 = 0x80D0_0100;
            let lock = memory.read_u32(cs + 0x10) as i32;
            if lock == -1 {
                // Unlocked — take it
                memory.write_u32(cs + 0x10, 0); // LockCount = 0
                memory.write_u32(cs + 0x18, current_thread); // OwningThread
                memory.write_u32(cs + 0x14, 1); // RecursionCount
                Some((KernelResult::Handled, 1)) // TRUE = success
            } else {
                let owner = memory.read_u32(cs + 0x18);
                if owner == current_thread {
                    // Recursive — increment
                    let new_lock = lock.wrapping_add(1);
                    memory.write_u32(cs + 0x10, new_lock as u32);
                    let rec = memory.read_u32(cs + 0x14);
                    memory.write_u32(cs + 0x14, rec.wrapping_add(1));
                    Some((KernelResult::Handled, 1))
                } else {
                    Some((KernelResult::Handled, 0)) // FALSE = already locked
                }
            }
        }

        // ---- Ke events/sync ----
        ordinals::KeInitializeEvent => {
            // Real kernel (v4627) at 0x8001A43B:
            //   [+0x00] Type = arg1 (0=Notification, 1=Synchronization)
            //   [+0x02] Size = 4
            //   [+0x04] SignalState = arg2 (initial state)
            //   [+0x08/+0x0C] WaitListHead self-referencing
            let ev = args[0];
            if ev != 0 {
                init_dispatcher_header(memory, ev, args[1] as u8, 4, args[2]);
                // Create real Win32 event backed by this guest address
                let manual_reset = args[1] == 0; // NotificationEvent=manual, SynchronizationEvent=auto
                let initial_state = args[2] != 0;
                let h = unsafe {
                    windows::Win32::System::Threading::CreateEventW(
                        None,
                        manual_reset,
                        initial_state,
                        None,
                    )
                };
                if let Ok(handle) = h {
                    state.create_object_at_addr(XboxObjectType::Event, handle.0 as usize, ev);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeSetEvent => {
            // KeSetEvent(Event*, Increment, Wait) — sets event to signaled
            let ev = args[0];
            let prev = if ev != 0 {
                memory.read_u32(ev + 0x04)
            } else {
                0
            };
            if ev != 0 {
                memory.write_u32(ev + 0x04, 1); // SignalState = 1
                notify_dispatcher_waiters(ev);
                if let Some(native) = state.get_native_handle_by_addr(ev) {
                    unsafe {
                        let _ = windows::Win32::System::Threading::SetEvent(
                            windows::Win32::Foundation::HANDLE(native as _),
                        );
                    }
                }
            }
            // VBlank event KeSetEvent: set PFIFO idle flag so ISR exit condition is met.
            // ISR at 0x002FC10E: test byte ptr [esi+3214h], 10h; je loop
            // esi = DPC context (device struct). Without bit4, ISR loops forever.
            if ev == 0x00303230 {
                let dpc_ctx = crate::xbox::aot::veh_dpc::get_dpc_context();
                if dpc_ctx != 0 && dpc_ctx < 0x1000_0000 {
                    let cur = memory.read_u32(dpc_ctx + 0x3214);
                    memory.write_u32(dpc_ctx + 0x3214, cur | 0x10);
                }
            }
            Some((KernelResult::Handled, prev))
        }
        ordinals::KeResetEvent => {
            // KeResetEvent(Event*) — returns previous SignalState, resets to 0
            let ev = args[0];
            let prev = if ev != 0 {
                memory.read_u32(ev + 0x04)
            } else {
                0
            };
            if ev != 0 {
                memory.write_u32(ev + 0x04, 0);
                if let Some(native) = state.get_native_handle_by_addr(ev) {
                    unsafe {
                        let _ = windows::Win32::System::Threading::ResetEvent(
                            windows::Win32::Foundation::HANDLE(native as _),
                        );
                    }
                }
            }
            Some((KernelResult::Handled, prev))
        }
        ordinals::KePulseEvent => {
            // KePulseEvent — set then immediately reset
            let ev = args[0];
            let prev = if ev != 0 {
                memory.read_u32(ev + 0x04)
            } else {
                0
            };
            if ev != 0 {
                if let Some(native) = state.get_native_handle_by_addr(ev) {
                    let h = windows::Win32::Foundation::HANDLE(native as _);
                    unsafe {
                        memory.write_u32(ev + 0x04, 1);
                        notify_dispatcher_waiters(ev);
                        let _ = windows::Win32::System::Threading::SetEvent(h);
                        memory.write_u32(ev + 0x04, 0);
                        let _ = windows::Win32::System::Threading::ResetEvent(h);
                    }
                }
            }
            Some((KernelResult::Handled, prev))
        }
        ordinals::KeInitializeMutant => {
            // Real kernel (v4627) at 0x8001A58F:
            //   [+0x00] Type = 2, [+0x02] Size = 8
            //   [+0x04] SignalState = 1 (if !InitialOwner), 0 (if InitialOwner)
            //   [+0x08/+0x0C] WaitListHead self-referencing
            //   [+0x18] OwnerThread, [+0x1C] Abandoned = 0
            let m = args[0];
            let initial_owner = args[1] != 0;
            if m != 0 {
                init_dispatcher_header(
                    memory,
                    m,
                    DISPATCHER_TYPE_MUTANT,
                    8,
                    if initial_owner { 0 } else { 1 },
                );
                memory.write_u32(m + 0x18, 0); // OwnerThread
                memory.write_u8(m + 0x1C, 0); // Abandoned
                                              // Create real Win32 mutex
                let h = unsafe {
                    windows::Win32::System::Threading::CreateMutexW(None, initial_owner, None)
                };
                if let Ok(handle) = h {
                    state.create_object_at_addr(XboxObjectType::Mutant, handle.0 as usize, m);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeReleaseMutant => {
            // KeReleaseMutant(Mutant*, Increment, Abandoned, Wait)
            let m = args[0];
            let prev = if m != 0 {
                memory.read_u32(m + 0x04) as i32
            } else {
                0
            };
            if m != 0 {
                // Increment SignalState (mutant: 0=owned, 1=released)
                memory.write_u32(m + 0x04, (prev + 1) as u32);
                notify_dispatcher_waiters(m);
                if let Some(native) = state.get_native_handle_by_addr(m) {
                    unsafe {
                        let _ = windows::Win32::System::Threading::ReleaseMutex(
                            windows::Win32::Foundation::HANDLE(native as _),
                        );
                    }
                }
            }
            Some((KernelResult::Handled, prev as u32))
        }
        ordinals::KeInitializeSemaphore => {
            // KeInitializeSemaphore(Semaphore, Count, Limit)
            let s = args[0];
            if s != 0 {
                init_dispatcher_header(memory, s, DISPATCHER_TYPE_SEMAPHORE, 5, args[1]);
                memory.write_u32(s + 0x10, args[2]); // Limit
                let h = unsafe {
                    windows::Win32::System::Threading::CreateSemaphoreW(
                        None,
                        args[1] as i32,
                        args[2] as i32,
                        None,
                    )
                };
                if let Ok(handle) = h {
                    state.create_object_at_addr(XboxObjectType::Semaphore, handle.0 as usize, s);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeReleaseSemaphore => {
            // KeReleaseSemaphore(Semaphore*, Increment, Adjustment, Wait)
            let s = args[0];
            let prev = if s != 0 { memory.read_u32(s + 0x04) } else { 0 };
            if s != 0 {
                let adjustment = args[2];
                memory.write_u32(s + 0x04, prev + adjustment);
                notify_dispatcher_waiters(s);
                if let Some(native) = state.get_native_handle_by_addr(s) {
                    unsafe {
                        let _ = windows::Win32::System::Threading::ReleaseSemaphore(
                            windows::Win32::Foundation::HANDLE(native as _),
                            adjustment as i32,
                            None,
                        );
                    }
                }
            }
            Some((KernelResult::Handled, prev))
        }
        ordinals::KeInitializeQueue => Some((KernelResult::Handled, 0)),
        ordinals::KeInsertQueue
        | ordinals::KeInsertHeadQueue
        | ordinals::KeRemoveQueue
        | ordinals::KeRundownQueue => Some((KernelResult::Handled, 0)),
        ordinals::KeInitializeTimerEx => {
            // Real kernel (v4627) at 0x800194C0:
            //   [+0x00] Type = arg1 + 8, [+0x02] Size = 0x0A, [+0x03] Inserted = 0
            //   [+0x04] SignalState = 0
            //   [+0x08/+0x0C] WaitListHead self-referencing
            //   [+0x10] DueTime.Low = 0, [+0x14] DueTime.High = 0
            //   [+0x24] Period = 0
            let t = args[0];
            if t != 0 {
                // Zero 40 bytes (0x28)
                for i in (0u32..40).step_by(4) {
                    memory.write_u32(t + i, 0);
                }
                init_dispatcher_header(
                    memory,
                    t,
                    (args[1] as u8).wrapping_add(DISPATCHER_TYPE_TIMER_BASE),
                    0x0A,
                    0,
                );
                memory.write_u8(t + 3, 0); // Inserted = FALSE
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeSetTimer | ordinals::KeSetTimerEx => {
            // KeSetTimer(Timer, DueDateLow, DueDateHigh, Dpc)
            let timer_ptr = args[0];
            if timer_ptr != 0 {
                let periodic = ordinal == ordinals::KeSetTimerEx && args[4] != 0;
                // Inserted = 1 (offset +3)
                memory.write_u8(timer_ptr + 3, 1);
                // SignalState = 1 (offset +4) — pre-signal as expired so game's
                // wait-after-set pattern (KeSetTimer → KeDelay → check) sees it fired
                memory.write_u32(timer_ptr + 4, 1);
                // Capture timer address for DPC SignalState management
                crate::xbox::aot::veh_dpc::capture_timer(timer_ptr);
                crate::xbox::aot::veh_dpc::set_timer_periodic(periodic);
                notify_dispatcher_waiters(timer_ptr);
                // Timer expired immediately → queue the associated DPC.
                // Real Xbox kernel fires DPC when timer expires; since we pre-signal,
                // we must also set DPC_PENDING so the dispatch loop delivers it.
                crate::xbox::aot::veh_dpc::set_dpc_pending();
                crate::xbox::emulator::debug_log(&format!(
                    "KeSetTimer: timer=0x{:08X} signal_state=1 periodic={} (pre-signaled), DPC queued",
                    timer_ptr, periodic
                ));
            }
            Some((KernelResult::Handled, 0)) // return FALSE (not previously set)
        }
        ordinals::KeCancelTimer => {
            crate::xbox::aot::veh_dpc::set_timer_periodic(false);
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeInitializeDpc => {
            // Real kernel (v4627) at 0x8001A384:
            //   [+0x00] Type = 0x13 (KDPC), [+0x02] Importance = 0
            //   [+0x0C] DeferredRoutine, [+0x10] DeferredContext
            //   ret 0xC = 3 args
            let dpc_ptr = args[0];
            if dpc_ptr != 0 {
                // Zero 28-byte KDPC structure
                for i in (0u32..28).step_by(4) {
                    memory.write_u32(dpc_ptr + i, 0);
                }
                memory.write_u16(dpc_ptr, 0x13); // Type = KDPC
                memory.write_u32(dpc_ptr + 0x0C, args[1]); // DeferredRoutine
                memory.write_u32(dpc_ptr + 0x10, args[2]); // DeferredContext
            }
            // Capture for L4 DPC heartbeat injector
            crate::xbox::aot::veh_dpc::capture_dpc(args[0], args[1], args[2]);
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeInsertQueueDpc => {
            // Real KeInsertQueueDpc returns FALSE when the KDPC is already
            // queued, and it updates the KDPC object itself:
            //   +0x02 Inserted, +0x14 SystemArgument1, +0x18 SystemArgument2.
            // Cxbx-R models this literally; titles can observe these fields.
            let dpc = args[0];
            let queued = if dpc != 0 && dpc < 0x2000_0000 {
                if memory.read_u8(dpc + 0x02) != 0 {
                    false
                } else {
                    memory.write_u8(dpc + 0x02, 1);
                    memory.write_u32(dpc + 0x14, args[1]);
                    memory.write_u32(dpc + 0x18, args[2]);
                    let queued = crate::xbox::aot::veh_dpc::queue_dpc_object(dpc);
                    if !queued {
                        memory.write_u8(dpc + 0x02, 0);
                    }
                    queued
                }
            } else {
                crate::xbox::aot::veh_dpc::queue_dpc_object(dpc)
            };
            Some((KernelResult::Handled, u32::from(queued)))
        }
        ordinals::KeRemoveQueueDpc => {
            let dpc = args[0];
            let was_inserted = dpc != 0 && dpc < 0x2000_0000 && memory.read_u8(dpc + 0x02) != 0;
            if was_inserted {
                memory.write_u8(dpc + 0x02, 0);
            }
            let removed = crate::xbox::aot::veh_dpc::remove_dpc_object(dpc) || was_inserted;
            Some((KernelResult::Handled, u32::from(removed)))
        }
        ordinals::KeInitializeApc | ordinals::KeInsertQueueApc => Some((KernelResult::Handled, 0)),
        ordinals::KeInitializeDeviceQueue
        | ordinals::KeInsertDeviceQueue
        | ordinals::KeInsertByKeyDeviceQueue
        | ordinals::KeRemoveDeviceQueue
        | ordinals::KeRemoveByKeyDeviceQueue
        | ordinals::KeRemoveEntryDeviceQueue => Some((KernelResult::Handled, 0)),

        // ---- Ke thread/process ----
        ordinals::KeGetCurrentThread => {
            // Real kernel: mov eax, [0x80035C04] — return current thread pointer
            // Write to guest memory too so compiled code sees it
            memory.write_u32(0x0003_5C04, 0x80D0_0100);
            Some((KernelResult::Handled, 0x80D0_0100))
        }
        ordinals::KeGetCurrentIrql => {
            // Real kernel: movzx eax, byte ptr [0x80035C00]
            let irql = CURRENT_IRQL.load(std::sync::atomic::Ordering::Relaxed) as u32;
            Some((KernelResult::Handled, irql))
        }
        ordinals::KeRaiseIrqlToDpcLevel | ordinals::KeRaiseIrqlToSynchLevel => {
            // Real kernel: old = [0x80035C00]; [0x80035C00] = 2; return old
            let new_irql: u8 = if ordinal == ordinals::KeRaiseIrqlToDpcLevel {
                2
            } else {
                0x1C
            };
            let old = CURRENT_IRQL.swap(new_irql, std::sync::atomic::Ordering::Relaxed) as u32;
            // Also write to guest memory so compiled code reading [0x80035C00] sees it
            memory.write_u8(0x0003_5C00, new_irql);
            Some((KernelResult::Handled, old))
        }
        ordinals::KfRaiseIrql => {
            // Real kernel: old = [0x80035C00]; [0x80035C00] = cl; return old
            let new_irql = *guest_ecx as u8;
            let old = CURRENT_IRQL.swap(new_irql, std::sync::atomic::Ordering::Relaxed) as u32;
            memory.write_u8(0x0003_5C00, new_irql);
            Some((KernelResult::Handled, old))
        }
        ordinals::KfLowerIrql => {
            // Real kernel: [0x80035C00] = cl; check pending DPCs
            let new_irql = *guest_ecx as u8;
            CURRENT_IRQL.store(new_irql, std::sync::atomic::Ordering::Relaxed);
            memory.write_u8(0x0003_5C00, new_irql);
            Some((KernelResult::Handled, 0))
        }
        ordinals::KiUnlockDispatcherDatabase => Some((KernelResult::Handled, 0)),
        ordinals::KeEnterCriticalRegion | ordinals::KeLeaveCriticalRegion => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeIsExecutingDpc => Some((KernelResult::Handled, 0)),
        ordinals::KeAlertResumeThread | ordinals::KeAlertThread => Some((KernelResult::Handled, 0)),
        ordinals::KeBoostPriorityThread
        | ordinals::KeSetBasePriorityThread
        | ordinals::KeSetPriorityThread
        | ordinals::KeSetDisableBoostThread
        | ordinals::KeSetPriorityProcess
        | ordinals::KeSetEventBoostPriority => Some((KernelResult::Handled, 0)),
        ordinals::KeQueryBasePriorityThread => Some((KernelResult::Handled, 8)),
        ordinals::KeResumeThread | ordinals::KeSuspendThread => Some((KernelResult::Handled, 0)),
        ordinals::KeTestAlertThread => Some((KernelResult::Handled, 0)),
        ordinals::KeDelayExecutionThread => {
            // KeDelayExecutionThread(Alertable, WaitMode, Interval)
            // When Alertable=TRUE, the thread expects pending APCs (I/O completion
            // routines from ReadFileEx, etc.) to be delivered during the wait.
            // Xbox async file I/O queues completion APCs; SleepEx(Alertable=TRUE)
            // drains them. Without APC delivery the caller spins forever.
            let alertable = args[0] != 0;
            let interval_ptr = args[2];

            // Read interval for diagnostic (negative = relative, 100ns units)
            let interval_100ns = read_guest_i64(memory, interval_ptr).unwrap_or(0);

            {
                static KDEL_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = KDEL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 10 || n % 1000 == 0 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[KeDelay] #{} alertable={} interval={}us esp=0x{:08X}",
                        n,
                        alertable,
                        -interval_100ns / 10,
                        args[2]
                    ));
                }
                if menu_wait_trace_log(n) {
                    crate::xbox::emulator::debug_log(&format!(
                        "[MENU-WAIT] KeDelayExecutionThread #{} alertable={} wait_mode={} interval_ptr=0x{:08X} interval_100ns={} interval_us={} ret=0",
                        n,
                        alertable,
                        args[1],
                        interval_ptr,
                        interval_100ns,
                        -interval_100ns / 10
                    ));
                }
            }

            // Zero interval is still a poll/yield by default. The env policy is
            // only for A/B testing the hot Spider-Man yield storm.
            let vblank_h =
                crate::xbox::emulator::VBLANK_EVENT.load(std::sync::atomic::Ordering::Relaxed);
            if interval_100ns == 0 {
                #[derive(Clone, Copy, Debug, PartialEq)]
                enum ZeroPolicy {
                    Spin,
                    Noop,
                    Yield,
                }

                static ZERO_POLICY: std::sync::OnceLock<ZeroPolicy> = std::sync::OnceLock::new();
                let policy = *ZERO_POLICY.get_or_init(|| {
                    if let Ok(val) = std::env::var("RUSTEMU_KDELAY_ZERO_POLICY") {
                        match val.trim().to_lowercase().as_str() {
                            "spin" => ZeroPolicy::Spin,
                            "noop" => ZeroPolicy::Noop,
                            _ => ZeroPolicy::Yield,
                        }
                    } else {
                        ZeroPolicy::Yield
                    }
                });

                static ZERO_SLEEP_US: std::sync::OnceLock<Option<u64>> = std::sync::OnceLock::new();
                let sleep_us = *ZERO_SLEEP_US.get_or_init(|| {
                    std::env::var("RUSTEMU_KDELAY_ZERO_SLEEP_US")
                        .ok()
                        .and_then(|v| v.trim().parse::<u64>().ok())
                        .filter(|&v| v > 0)
                });

                if let Some(us) = sleep_us {
                    std::thread::sleep(std::time::Duration::from_micros(us));
                } else {
                    match policy {
                        ZeroPolicy::Spin => std::hint::spin_loop(),
                        ZeroPolicy::Noop => {}
                        ZeroPolicy::Yield => std::thread::yield_now(),
                    }
                }
            } else if vblank_h != 0 {
                use windows::Win32::Foundation::HANDLE;
                use windows::Win32::System::Threading::WaitForSingleObject;
                unsafe {
                    WaitForSingleObject(HANDLE(vblank_h as *mut _), 16); // max 16ms
                }
            } else {
                std::thread::yield_now();
            }

            // Return STATUS_SUCCESS (0) always. Returning WAIT_IO_COMPLETION (0xC0)
            // when alertable caused a regression: the game's early-init KeDelay call
            // interpreted 0xC0 as "APC delivered" but no APC actually ran, sending
            // init down an error path to XapiBootToDash. Spider-Man's CHILD-2 used
            // synchronous NtReadFile (no APCs), so there's nothing to deliver.
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeWaitForSingleObject => {
            // KeWaitForSingleObject(Object, WaitReason, WaitMode, Alertable, Timeout)
            // Object is a guest DISPATCHER_OBJECT pointer. Only return success
            // when that specific object is signaled; Cxbx-R does this by
            // inserting a KWAIT_BLOCK into the object's WaitListHead.
            let obj_addr = args[0];
            let timeout_ptr = args[4];
            Some((
                KernelResult::Handled,
                ke_wait_for_dispatcher_object(
                    memory,
                    obj_addr,
                    timeout_ptr,
                    "KeWaitForSingleObject",
                ),
            ))
        }
        ordinals::KeWaitForMultipleObjects => {
            // KeWaitForMultipleObjects(Count, Objects[], WaitType, WaitReason, WaitMode, Alertable, Timeout, WaitBlocks)
            let count = args[0].min(64);
            let objects_ptr = args[1];
            let wait_type = args[2]; // 0=WaitAll, 1=WaitAny
            let timeout_ptr = args[6];
            Some((
                KernelResult::Handled,
                ke_wait_for_multiple_dispatcher_objects(
                    memory,
                    objects_ptr,
                    count,
                    wait_type,
                    timeout_ptr,
                ),
            ))
        }
        ordinals::KeSynchronizeExecution => Some((KernelResult::Handled, 1)),
        ordinals::KeConnectInterrupt | ordinals::KeDisconnectInterrupt => {
            Some((KernelResult::Handled, 1)) // TRUE = success
        }
        ordinals::KeInitializeInterrupt => {
            // KeInitializeInterrupt(IntObj, ServiceRoutine, ServiceCtx, Vector, Irql, Mode, ShareVector)
            // Real kernel (v4627) at 0x800195F0: writes interrupt object fields then
            // copies 88-byte dispatch template from 0x8001AEB6 into IntObj+0x18.
            // We skip the template (HLE doesn't fire real interrupts) but must set fields.
            let int_obj = args[0];
            let service_routine = args[1];
            let service_ctx = args[2];
            let vector = args[3];
            let irql = args[4];
            let mode = args[5];

            if int_obj != 0
                && (int_obj < 0x2000_0000 || (int_obj >= 0x8000_0000 && int_obj < 0xA000_0000))
            {
                memory.write_u32(int_obj + 0x00, service_routine); // ServiceRoutine
                memory.write_u32(int_obj + 0x04, service_ctx); // ServiceContext
                memory.write_u32(int_obj + 0x08, vector.wrapping_sub(0x30)); // BusInterruptLevel
                memory.write_u32(int_obj + 0x0C, irql & 0xFF); // Irql
                memory.write_u8(int_obj + 0x10, 0); // Connected = FALSE
                memory.write_u8(int_obj + 0x12, mode as u8); // Mode
                                                             // +0x18..+0x70: dispatch code template (88 bytes) — not needed for HLE
                                                             // Zero it so driver code doesn't see garbage
                for i in 0..88u32 {
                    memory.write_u8(int_obj + 0x18 + i, 0);
                }
            }
            // Capture ISR for VBlank heartbeat injector — but ONLY for NV2A GPU.
            // On Xbox, NV2A GPU interrupt vector = 0x33 (IRQ 3, bus_level 3).
            // USB OHCI is vec=0x31 (bus_level 1, irql=25). Their ServiceContext is
            // a device extension pointer (e.g. 0x80D134B0) that gets misinterpreted
            // as a code address if injected as a heartbeat ISR.
            let bus_level = vector.wrapping_sub(0x30);
            if bus_level == 3 {
                // NV2A GPU interrupt — this is the VBlank heartbeat
                crate::xbox::aot::veh_dpc::capture_isr(service_routine, service_ctx);
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "KeInitializeInterrupt: SKIPPING ISR capture for non-GPU vec={} (bus_level={})",
                    vector, bus_level
                ));
            }
            crate::xbox::emulator::debug_log(&format!(
                "KeInitializeInterrupt: obj=0x{:08X} isr=0x{:08X} ctx=0x{:08X} vec={} irql={} mode={}",
                int_obj, service_routine, service_ctx, vector, irql, mode
            ));
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeSaveFloatingPointState | ordinals::KeRestoreFloatingPointState => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::KeBugCheck | ordinals::KeBugCheckEx => {
            crate::xbox::emulator::debug_log(&format!(
                "[KERNEL] KeBugCheck(0x{:08X}) — FATAL, halting guest (guest_eax=0x{:08X} guest_ecx=0x{:08X} guest_edx=0x{:08X})",
                args[0], *guest_eax, *guest_ecx, *guest_edx
            ));
            for (i, entry) in state.call_log.iter().enumerate() {
                crate::xbox::emulator::debug_log(&format!(
                    "[KERNEL]   bugcheck_tail[{}]: {}",
                    i, entry
                ));
            }
            Some((KernelResult::Halt, 0))
        }

        // ---- Interlocked (fastcall: ECX=addr, EDX=value) ----
        ordinals::InterlockedIncrement => {
            let addr = *guest_ecx;
            if addr != 0 && addr < 0x1000_0000 {
                let old = memory.read_u32(addr);
                let new_val = old.wrapping_add(1);
                memory.write_u32(addr, new_val);
                Some((KernelResult::Handled, new_val))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedDecrement => {
            let addr = *guest_ecx;
            if addr != 0 && addr < 0x1000_0000 {
                let old = memory.read_u32(addr);
                let new_val = old.wrapping_sub(1);
                memory.write_u32(addr, new_val);
                Some((KernelResult::Handled, new_val))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedExchange => {
            let addr = *guest_ecx;
            let value = *guest_edx;
            if addr != 0 && addr < 0x1000_0000 {
                let old = memory.read_u32(addr);
                memory.write_u32(addr, value);
                Some((KernelResult::Handled, old))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedCompareExchange => {
            let addr = *guest_ecx;
            let exchange = *guest_edx;
            let comperand = args[0];
            if addr != 0 && addr < 0x1000_0000 {
                let old = memory.read_u32(addr);
                if old == comperand {
                    memory.write_u32(addr, exchange);
                }
                Some((KernelResult::Handled, old))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedExchangeAdd => {
            let addr = *guest_ecx;
            let value = *guest_edx;
            if addr != 0 && addr < 0x1000_0000 {
                let old = memory.read_u32(addr);
                memory.write_u32(addr, old.wrapping_add(value));
                Some((KernelResult::Handled, old))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedFlushSList => {
            // __fastcall: ECX = PSLIST_HEADER (guest ptr)
            // Flush: atomically remove all entries, return pointer to first (old head).
            // SLIST_HEADER: { Next: u32 (offset 0), Depth: u16 (offset 4), Sequence: u16 (offset 6) }
            let header = *guest_ecx;
            if header != 0 && header < 0x1000_0000 {
                let old_head = memory.read_u32(header); // Next pointer
                memory.write_u32(header, 0); // Clear head
                memory.write_u16(header + 4, 0); // Clear depth
                Some((KernelResult::Handled, old_head))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedPopEntrySList => {
            // __fastcall: ECX = PSLIST_HEADER
            // Pop: remove head entry, return it. Each entry starts with Next pointer at offset 0.
            let header = *guest_ecx;
            if header != 0 && header < 0x1000_0000 {
                let head = memory.read_u32(header);
                if head != 0 && head < 0x1000_0000 {
                    let next = memory.read_u32(head); // head->Next
                    memory.write_u32(header, next); // header->Next = head->Next
                    let depth = memory.read_u16(header + 4);
                    if depth > 0 {
                        memory.write_u16(header + 4, depth - 1);
                    }
                    Some((KernelResult::Handled, head))
                } else {
                    Some((KernelResult::Handled, 0)) // Empty list
                }
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::InterlockedPushEntrySList => {
            // __fastcall: ECX = PSLIST_HEADER, EDX = PSLIST_ENTRY (new item)
            // Push: insert entry at head.
            let header = *guest_ecx;
            let entry = *guest_edx;
            if header != 0 && header < 0x1000_0000 && entry != 0 && entry < 0x1000_0000 {
                let old_head = memory.read_u32(header);
                memory.write_u32(entry, old_head); // entry->Next = old head
                memory.write_u32(header, entry); // header->Next = entry
                let depth = memory.read_u16(header + 4);
                memory.write_u16(header + 4, depth + 1);
                Some((KernelResult::Handled, old_head))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }

        // ---- Executive misc ----
        ordinals::ExAcquireReadWriteLockExclusive
        | ordinals::ExAcquireReadWriteLockShared
        | ordinals::ExInitializeReadWriteLock
        | ordinals::ExReleaseReadWriteLock => Some((KernelResult::Handled, 0)),
        ordinals::ExInterlockedAddLargeInteger
        | ordinals::ExInterlockedAddLargeStatistic
        | ordinals::ExInterlockedCompareExchange64 => Some((KernelResult::Handled, 0)),
        ordinals::ExfInterlockedInsertHeadList => {
            // __fastcall: ECX = PLIST_ENTRY ListHead, EDX = PLIST_ENTRY Entry
            // LIST_ENTRY: { Flink: u32, Blink: u32 }
            // Insert Entry at head: Entry goes after ListHead, before ListHead->Flink.
            let list_head = *guest_ecx;
            let entry = *guest_edx;
            if list_head != 0 && list_head < 0x1000_0000 && entry != 0 && entry < 0x1000_0000 {
                let old_flink = memory.read_u32(list_head); // ListHead->Flink
                memory.write_u32(entry, old_flink); // Entry->Flink = old_flink
                memory.write_u32(entry + 4, list_head); // Entry->Blink = ListHead
                if old_flink != 0 && old_flink < 0x1000_0000 {
                    memory.write_u32(old_flink + 4, entry); // old_flink->Blink = Entry
                }
                memory.write_u32(list_head, entry); // ListHead->Flink = Entry
                                                    // Return old Flink (NULL if list was empty = head pointed to itself)
                let result = if old_flink == list_head { 0 } else { old_flink };
                Some((KernelResult::Handled, result))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::ExfInterlockedInsertTailList => {
            // __fastcall: ECX = PLIST_ENTRY ListHead, EDX = PLIST_ENTRY Entry
            // Insert Entry at tail: Entry goes before ListHead, after ListHead->Blink.
            let list_head = *guest_ecx;
            let entry = *guest_edx;
            if list_head != 0 && list_head < 0x1000_0000 && entry != 0 && entry < 0x1000_0000 {
                let old_blink = memory.read_u32(list_head + 4); // ListHead->Blink
                memory.write_u32(entry, list_head); // Entry->Flink = ListHead
                memory.write_u32(entry + 4, old_blink); // Entry->Blink = old_blink
                if old_blink != 0 && old_blink < 0x1000_0000 {
                    memory.write_u32(old_blink, entry); // old_blink->Flink = Entry
                }
                memory.write_u32(list_head + 4, entry); // ListHead->Blink = Entry
                let result = if old_blink == list_head { 0 } else { old_blink };
                Some((KernelResult::Handled, result))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::ExfInterlockedRemoveHeadList => {
            // __fastcall: ECX = PLIST_ENTRY ListHead
            // Remove and return head entry (ListHead->Flink). Returns NULL if empty.
            let list_head = *guest_ecx;
            if list_head != 0 && list_head < 0x1000_0000 {
                let entry = memory.read_u32(list_head); // ListHead->Flink
                if entry == 0 || entry == list_head || entry >= 0x1000_0000 {
                    Some((KernelResult::Handled, 0)) // Empty list
                } else {
                    let next = memory.read_u32(entry); // Entry->Flink
                    memory.write_u32(list_head, next); // ListHead->Flink = Entry->Flink
                    if next != 0 && next < 0x1000_0000 {
                        memory.write_u32(next + 4, list_head); // next->Blink = ListHead
                    }
                    Some((KernelResult::Handled, entry))
                }
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::ExRaiseException | ordinals::ExRaiseStatus => Some((KernelResult::Handled, 0)),
        ordinals::ExQueryNonVolatileSetting => {
            // ExQueryNonVolatileSetting(ValueIndex, pType, pValue, ValueLength, pResultLength)
            let idx = args[0];
            let p_type = args[1];
            let p_value = args[2];
            let val_len = args[3];
            let p_res_len = args[4];

            let result_type: u32 = 4; // REG_DWORD
            let mut result_len: u32 = 4;
            let result: u32 = match idx {
                0x00 => 0,           // XC_TIMEZONE_BIAS
                0x04 => 1,           // XC_TZ_DLT_DATE
                0x05 => 0,           // XC_TZ_STD_BIAS
                0x06 => 0x0100_0000, // XC_TZ_DLT_BIAS
                0x07 => 1,           // XC_LANGUAGE (1 = English)
                0x08 => 0,           // XC_VIDEO: user video flags (0 = use factory defaults)
                0x09 => 0xFFFF_FFFF, // XC_AUDIO
                0x0A => 0,           // XC_P_CONTROL_GAMES
                0x0B => 0,           // XC_P_CONTROL_PASSWORD
                0x0C => 0x0004_0F00, // XC_P_CONTROL_MOVIES
                0x0D => 0,           // XC_ONLINE_IP_ADDRESS
                0x0E => 0,           // XC_ONLINE_DNS_ADDRESS
                0x0F => 0,           // XC_ONLINE_DEFAULT_GATEWAY
                0x10 => 0,           // XC_ONLINE_SUBNET_ADDRESS
                0x11 => 0x0004_0F00, // XC_MISC (MiscFlags)
                0x100 => 0,          // XC_FACTORY_SERIAL_NUMBER
                0x103 => factory_av_region_for_console(state.xbe_game_region),
                0x104 => console_game_region_for_title(state.xbe_game_region),
                0xFFFF => {
                    // XC_MAX_ALL — full EEPROM dump
                    if p_value != 0 && val_len >= 256 {
                        for i in 0u32..256 {
                            let b = (0xA5u8).wrapping_add((i as u8).wrapping_mul(0x37));
                            memory.write_u8(p_value + i, b);
                        }
                        // First 4 bytes overwrite
                        memory.write_u8(p_value, 0x04);
                        memory.write_u8(p_value + 1, 0x0F);
                        memory.write_u8(p_value + 2, 0x00);
                        memory.write_u8(p_value + 3, 0x00);
                    }
                    result_len = 256;
                    0
                }
                _ => 0,
            };

            if p_type != 0 {
                memory.write_u32(p_type, result_type);
            }
            if p_value != 0 && val_len >= 4 && idx != 0xFFFF {
                memory.write_u32(p_value, result);
            }
            if p_res_len != 0 {
                memory.write_u32(p_res_len, result_len);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::ExReadWriteRefurbInfo | ordinals::ExSaveNonVolatileSetting => {
            Some((KernelResult::Handled, 0))
        }

        // ---- Nt syscalls (non-file) ----
        // File I/O is handled by file.rs (NtCreateFile, NtOpenFile, NtReadFile, etc.)
        // NtClose for non-file handles falls through from file.rs to here.
        ordinals::NtClose => Some((KernelResult::Handled, 0)),
        ordinals::NtCreateEvent => {
            // NtCreateEvent(OUT PHANDLE EventHandle, IN POBJECT_ATTRIBUTES, IN EVENT_TYPE, IN BOOLEAN InitialState)
            // EventType: 0=NotificationEvent (manual-reset), 1=SynchronizationEvent (auto-reset)
            let event_type = args[2];
            let initial_state = args[3] != 0;
            let manual_reset = event_type == 0; // NotificationEvent = manual-reset
            let h = unsafe {
                windows::Win32::System::Threading::CreateEventW(
                    None,
                    manual_reset,
                    initial_state,
                    None,
                )
            };
            let native = match h {
                Ok(handle) => handle.0 as usize,
                Err(_) => 0,
            };
            let guest_handle = state.create_object(XboxObjectType::Event, native);
            if let Some(obj_addr) = state.get_object_addr(guest_handle) {
                init_dispatcher_header(
                    memory,
                    obj_addr,
                    event_type as u8,
                    4,
                    if initial_state { 1 } else { 0 },
                );
            }
            if args[0] != 0 {
                memory.write_u32(args[0], guest_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtCreateMutant => {
            // NtCreateMutant(OUT PHANDLE MutantHandle, IN POBJECT_ATTRIBUTES, IN BOOLEAN InitialOwner)
            let initial_owner = args[2] != 0;
            let h = unsafe {
                windows::Win32::System::Threading::CreateMutexW(None, initial_owner, None)
            };
            let native = match h {
                Ok(handle) => handle.0 as usize,
                Err(_) => 0,
            };
            let guest_handle = state.create_object(XboxObjectType::Mutant, native);
            if let Some(obj_addr) = state.get_object_addr(guest_handle) {
                init_dispatcher_header(
                    memory,
                    obj_addr,
                    DISPATCHER_TYPE_MUTANT,
                    8,
                    if initial_owner { 0 } else { 1 },
                );
            }
            if args[0] != 0 {
                memory.write_u32(args[0], guest_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtCreateSemaphore => {
            // NtCreateSemaphore(OUT PHANDLE, IN POBJECT_ATTRIBUTES, IN LONG InitialCount, IN LONG MaximumCount)
            let initial = args[2] as i32;
            let max = args[3] as i32;
            let h = unsafe {
                windows::Win32::System::Threading::CreateSemaphoreW(None, initial, max, None)
            };
            let native = match h {
                Ok(handle) => handle.0 as usize,
                Err(_) => 0,
            };
            let guest_handle = state.create_object(XboxObjectType::Semaphore, native);
            if let Some(obj_addr) = state.get_object_addr(guest_handle) {
                init_dispatcher_header(
                    memory,
                    obj_addr,
                    DISPATCHER_TYPE_SEMAPHORE,
                    5,
                    initial as u32,
                );
                memory.write_u32(obj_addr + 0x10, max as u32);
            }
            if args[0] != 0 {
                memory.write_u32(args[0], guest_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtCreateTimer => {
            // NtCreateTimer(OUT PHANDLE, IN POBJECT_ATTRIBUTES, IN TIMER_TYPE)
            // Use a manual-reset event as timer proxy
            let h =
                unsafe { windows::Win32::System::Threading::CreateEventW(None, true, false, None) };
            let native = match h {
                Ok(handle) => handle.0 as usize,
                Err(_) => 0,
            };
            let guest_handle = state.create_object(XboxObjectType::Timer, native);
            if let Some(obj_addr) = state.get_object_addr(guest_handle) {
                init_dispatcher_header(memory, obj_addr, DISPATCHER_TYPE_TIMER_BASE, 0x0A, 0);
            }
            if args[0] != 0 {
                memory.write_u32(args[0], guest_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtCreateDirectoryObject | ordinals::NtOpenDirectoryObject => {
            if args[0] != 0 {
                state.next_handle += 1;
                memory.write_u32(args[0], state.next_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtOpenSymbolicLinkObject => {
            // NtOpenSymbolicLinkObject(PHANDLE LinkHandle, POBJECT_ATTRIBUTES ObjectAttributes)
            state.next_handle += 1;
            let handle = state.next_handle;
            if args[0] != 0 {
                memory.write_u32(args[0], handle);
            }
            // Read OBJECT_ATTRIBUTES → ObjectName to track which symlink this handle refers to
            let name =
                crate::xbox::kernel::file::extract_object_path(memory, args[1]).unwrap_or_default();
            if !name.is_empty() {
                crate::xbox::emulator::debug_log(&format!(
                    "NtOpenSymbolicLinkObject: handle=0x{:X} name='{}'",
                    handle, name
                ));
                if let Some(ref mut fs) = state.file_state {
                    fs.symlink_handles.insert(handle, name);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtCreateIoCompletion => {
            if args[0] != 0 {
                state.next_handle += 1;
                memory.write_u32(args[0], state.next_handle);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtQueryDirectoryObject => Some((KernelResult::Handled, 0)),
        ordinals::NtSetEvent => {
            // NtSetEvent(Handle, OUT PreviousState OPTIONAL)
            let handle = args[0];
            if let Some(obj_addr) = state.get_object_addr(handle) {
                let prev = memory.read_u32(obj_addr + 0x04);
                memory.write_u32(obj_addr + 0x04, 1);
                if args[1] != 0 && args[1] < 0x1000_0000 {
                    memory.write_u32(args[1], prev);
                }
                notify_dispatcher_waiters(obj_addr);
            }
            if let Some(native) = state.get_native_handle(handle) {
                unsafe {
                    let _ = windows::Win32::System::Threading::SetEvent(
                        windows::Win32::Foundation::HANDLE(native as _),
                    );
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtClearEvent => {
            let handle = args[0];
            if let Some(obj_addr) = state.get_object_addr(handle) {
                memory.write_u32(obj_addr + 0x04, 0);
            }
            if let Some(native) = state.get_native_handle(handle) {
                unsafe {
                    let _ = windows::Win32::System::Threading::ResetEvent(
                        windows::Win32::Foundation::HANDLE(native as _),
                    );
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtPulseEvent => {
            let handle = args[0];
            if let Some(obj_addr) = state.get_object_addr(handle) {
                memory.write_u32(obj_addr + 0x04, 1);
                notify_dispatcher_waiters(obj_addr);
                memory.write_u32(obj_addr + 0x04, 0);
            }
            if let Some(native) = state.get_native_handle(handle) {
                let h = windows::Win32::Foundation::HANDLE(native as _);
                unsafe {
                    let _ = windows::Win32::System::Threading::SetEvent(h);
                    let _ = windows::Win32::System::Threading::ResetEvent(h);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtQueryEvent => Some((KernelResult::Handled, 0)),
        ordinals::NtReleaseMutant => {
            // NtReleaseMutant(Handle, OUT PreviousCount OPTIONAL)
            let handle = args[0];
            if let Some(native) = state.get_native_handle(handle) {
                unsafe {
                    let _ = windows::Win32::System::Threading::ReleaseMutex(
                        windows::Win32::Foundation::HANDLE(native as _),
                    );
                }
            }
            if args[1] != 0 {
                memory.write_u32(args[1], 0); // PreviousCount = 0
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtQueryMutant => Some((KernelResult::Handled, 0)),
        ordinals::NtReleaseSemaphore => {
            // NtReleaseSemaphore(Handle, ReleaseCount, OUT PreviousCount OPTIONAL)
            let handle = args[0];
            let count = args[1] as i32;
            if let Some(obj_addr) = state.get_object_addr(handle) {
                let prev = memory.read_u32(obj_addr + 0x04);
                memory.write_u32(obj_addr + 0x04, prev.saturating_add(args[1]));
                if args[2] != 0 && args[2] < 0x1000_0000 {
                    memory.write_u32(args[2], prev);
                }
                notify_dispatcher_waiters(obj_addr);
            }
            if let Some(native) = state.get_native_handle(handle) {
                let mut prev: i32 = 0;
                unsafe {
                    let _ = windows::Win32::System::Threading::ReleaseSemaphore(
                        windows::Win32::Foundation::HANDLE(native as _),
                        count,
                        Some(&mut prev),
                    );
                }
                if args[2] != 0 {
                    memory.write_u32(args[2], prev as u32);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtQuerySemaphore => Some((KernelResult::Handled, 0)),
        ordinals::NtQueryTimer => Some((KernelResult::Handled, 0)),
        ordinals::NtCancelTimer => {
            crate::xbox::aot::veh_dpc::set_timer_periodic(false);
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtSetTimerEx => {
            // NtSetTimerEx(Handle, DueTime, ApcRoutine, ApcContext, WakeTimer, Period, PreviousState, ...)
            // Real kernel: arms timer, fires associated DPC when timer expires.
            // Our implementation: signal the timer event immediately (pre-expire) and queue DPC.
            let handle = args[0];
            if let Some(obj_addr) = state.get_object_addr(handle) {
                memory.write_u32(obj_addr + 0x04, 1);
                crate::xbox::aot::veh_dpc::capture_timer(obj_addr);
                notify_dispatcher_waiters(obj_addr);
            }
            let periodic = true;
            if let Some(native) = state.get_native_handle(handle) {
                unsafe {
                    let _ = windows::Win32::System::Threading::SetEvent(
                        windows::Win32::Foundation::HANDLE(native as *mut _),
                    );
                }
                crate::xbox::emulator::debug_log(&format!(
                    "NtSetTimerEx: handle=0x{:X} period_arg=0x{:08X} → signaled timer event, periodic={}, DPC queued",
                    handle, args[5], periodic
                ));
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "NtSetTimerEx: handle=0x{:X} → no native handle found",
                    handle
                ));
            }
            // Queue the captured DPC (from KeInitializeDpc) to fire on next wait return
            crate::xbox::aot::veh_dpc::set_timer_periodic(periodic);
            crate::xbox::aot::veh_dpc::set_dpc_pending();
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtSetIoCompletion | ordinals::NtQueryIoCompletion => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtRemoveIoCompletion => {
            std::thread::yield_now();
            Some((KernelResult::Handled, 0x0000_0102)) // STATUS_TIMEOUT
        }
        ordinals::NtQuerySymbolicLinkObject => {
            // NtQuerySymbolicLinkObject(HANDLE LinkHandle, PANSI_STRING LinkTarget, PULONG ReturnedLength)
            let handle = args[0];
            let p_target = args[1]; // ANSI_STRING output
            let p_ret_len = args[2];
            let mut status = 0u32; // STATUS_SUCCESS
                                   // Look up the symlink name from the handle, then resolve target from symlink table
            let target = if let Some(ref fs) = state.file_state {
                if let Some(name) = fs.symlink_handles.get(&handle) {
                    let key = name.to_lowercase();
                    fs.symlinks.get(&key).cloned()
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(ref target_str) = target {
                let bytes = target_str.as_bytes();
                let len = bytes.len() as u16;
                if p_target != 0 {
                    // ANSI_STRING: +0=Length(2), +2=MaxLength(2), +4=Buffer(4)
                    let max_len = memory.read_u16(p_target + 2);
                    let buf_ptr = memory.read_u32(p_target + 4);
                    if buf_ptr != 0 && max_len >= len {
                        memory.write_u16(p_target, len);
                        for (i, &b) in bytes.iter().enumerate() {
                            memory.write_u8(buf_ptr + i as u32, b);
                        }
                    } else {
                        status = 0xC000_0023; // STATUS_BUFFER_TOO_SMALL
                    }
                }
                if p_ret_len != 0 {
                    memory.write_u32(p_ret_len, len as u32);
                }
                crate::xbox::emulator::debug_log(&format!(
                    "NtQuerySymbolicLinkObject: handle=0x{:X} → '{}'",
                    handle, target_str
                ));
            } else {
                // No target found — return STATUS_OBJECT_NAME_NOT_FOUND
                status = 0xC000_0034;
                crate::xbox::emulator::debug_log(&format!(
                    "NtQuerySymbolicLinkObject: handle=0x{:X} — no target found",
                    handle
                ));
            }
            Some((KernelResult::Handled, status))
        }
        ordinals::NtSetSystemTime => Some((KernelResult::Handled, 0)),
        ordinals::NtWaitForSingleObject | ordinals::NtWaitForSingleObjectEx => {
            // NtWaitForSingleObject(Handle, Alertable, Timeout)
            // NtWaitForSingleObjectEx(Handle, WaitMode, Alertable, Timeout)
            let handle = args[0];
            let timeout_arg = if ordinal == ordinals::NtWaitForSingleObjectEx {
                args[3]
            } else {
                args[2]
            };
            // Timeout: 0 = infinite, else guest pointer to LARGE_INTEGER (100ns units, negative = relative)
            let timeout_ms = if timeout_arg == 0 {
                10_000u32 // infinite → cap at 10s to avoid permanent hang
            } else {
                let li = memory.read_u32(timeout_arg) as i64
                    | ((memory.read_u32(timeout_arg + 4) as i64) << 32);
                if li == 0 {
                    0 // poll
                } else if li < 0 {
                    // Relative: negative 100ns units → ms
                    ((-li) / 10_000).min(10_000) as u32
                } else {
                    // Absolute: just wait up to 10s
                    10_000u32
                }
            };
            if let Some(native) = state.get_native_handle(handle) {
                let result = unsafe {
                    windows::Win32::System::Threading::WaitForSingleObject(
                        windows::Win32::Foundation::HANDLE(native as _),
                        timeout_ms,
                    )
                };
                let status = match result {
                    windows::Win32::Foundation::WAIT_OBJECT_0 => 0u32, // STATUS_WAIT_0
                    windows::Win32::Foundation::WAIT_TIMEOUT => 0x00000102, // STATUS_TIMEOUT
                    windows::Win32::Foundation::WAIT_ABANDONED => 0x00000080, // STATUS_ABANDONED_WAIT_0
                    _ => 0,
                };
                Some((KernelResult::Handled, status))
            } else {
                // Unknown handle — return success to avoid hang
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::NtWaitForMultipleObjectsEx => {
            // NtWaitForMultipleObjectsEx(Count, Handles[], WaitType, WaitMode, Alertable, Timeout)
            let count = args[0].min(64) as usize;
            let handles_ptr = args[1];
            let wait_all = args[2] != 0; // WaitAll=1, WaitAny=0
            let timeout_arg = args[5];

            let timeout_ms = if timeout_arg == 0 {
                10_000u32
            } else {
                let li = memory.read_u32(timeout_arg) as i64
                    | ((memory.read_u32(timeout_arg + 4) as i64) << 32);
                if li == 0 {
                    0
                } else if li < 0 {
                    ((-li) / 10_000).min(10_000) as u32
                } else {
                    10_000u32
                }
            };

            // Collect native handles
            let mut native_handles: Vec<windows::Win32::Foundation::HANDLE> =
                Vec::with_capacity(count);
            for i in 0..count {
                let guest_h = memory.read_u32(handles_ptr + (i as u32) * 4);
                if let Some(native) = state.get_native_handle(guest_h) {
                    native_handles.push(windows::Win32::Foundation::HANDLE(native as _));
                }
            }

            if !native_handles.is_empty() {
                let result = unsafe {
                    windows::Win32::System::Threading::WaitForMultipleObjects(
                        &native_handles,
                        wait_all,
                        timeout_ms,
                    )
                };
                let status = match result {
                    windows::Win32::Foundation::WAIT_TIMEOUT => 0x00000102,
                    e if (e.0 as usize) < count => e.0 as u32, // WAIT_OBJECT_0 + index
                    _ => 0,
                };
                Some((KernelResult::Handled, status))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::NtSignalAndWaitForSingleObjectEx => Some((KernelResult::Handled, 0)),
        ordinals::NtResumeThread | ordinals::NtSuspendThread => Some((KernelResult::Handled, 0)),
        ordinals::NtYieldExecution => {
            // VBlank simulation: game's polling loop calls NtYieldExecution
            // repeatedly waiting for [0x3F1D48] to become non-zero.
            // Set it every 1000 yields to simulate VBlank tick.
            static YIELD_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = YIELD_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n % 1000 == 0 {
                memory.write_u8(0x003F_1D48, 1);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::NtUserIoApcDispatcher | ordinals::NtQueueApcThread => {
            Some((KernelResult::Handled, 0))
        }

        // ---- Object manager ----
        ordinals::ObCreateObject => {
            // Write fake object ptr to args[3]
            if args[3] != 0 {
                state.next_handle += 1;
                memory.write_u32(args[3], 0x80E0_0000 + state.next_handle * 0x100);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::ObInsertObject => Some((KernelResult::Handled, 0)),
        ordinals::ObMakeTemporaryObject => Some((KernelResult::Handled, 0)),
        ordinals::ObOpenObjectByName | ordinals::ObOpenObjectByPointer => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::ObReferenceObjectByHandle => {
            // ObReferenceObjectByHandle(Handle, ObjectType, OUT *ReturnedObject)
            // Cxbx-R: args are (Handle, ObjectType, AccessMode, *Object) — 4 args
            let handle = args[0];
            let p_object = args[3]; // OUT pointer
            let obj_ptr = if handle == 0xFFFF_FFFE {
                // NtCurrentThread() — return fake thread object
                0x00F0_0000
            } else if handle == 0xFFFF_FFFF {
                // NtCurrentProcess()
                0x00F1_0000
            } else if let Some(addr) = state.get_object_addr(handle) {
                // Real tracked object — increment ref count
                if let Some(obj) = state.object_table.get_mut(&handle) {
                    obj.ref_count += 1;
                }
                addr
            } else {
                // Unknown handle — return a stable fake address
                0x00F0_0000 + (handle & 0xFFFF) * 0x20
            };
            if p_object != 0 && p_object < 0x1000_0000 {
                memory.write_u32(p_object, obj_ptr);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::ObReferenceObjectByName | ordinals::ObReferenceObjectByPointer => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::ObfDereferenceObject => {
            // __fastcall ObfDereferenceObject(Object*) — ECX = object pointer
            let obj_addr = *guest_ecx;
            if let Some(&guest_handle) = state.obj_addr_to_handle.get(&obj_addr) {
                if let Some(obj) = state.object_table.get_mut(&guest_handle) {
                    obj.ref_count -= 1;
                    // Don't close on ref_count==0 — games may re-reference
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::ObfReferenceObject => {
            // __fastcall ObfReferenceObject(Object*) — ECX = object pointer
            let obj_addr = *guest_ecx;
            if let Some(&guest_handle) = state.obj_addr_to_handle.get(&obj_addr) {
                if let Some(obj) = state.object_table.get_mut(&guest_handle) {
                    obj.ref_count += 1;
                }
            }
            Some((KernelResult::Handled, 0))
        }

        // ---- Process/Thread ----
        ordinals::PsCreateSystemThread | ordinals::PsCreateSystemThreadEx => {
            // Create a real waitable thread object. Several game loaders keep
            // the returned handle and wait on it to observe async stash loads.
            let thread_handle = unsafe {
                windows::Win32::System::Threading::CreateEventW(None, true, false, None)
                    .ok()
                    .map(|h| {
                        let guest_handle =
                            state.create_object(XboxObjectType::Thread, h.0 as usize);
                        if let Some(obj_addr) = state.get_object_addr(guest_handle) {
                            // Minimal DISPATCHER_HEADER:
                            // Type(+0), Size(+2), SignalState(+4), WaitListHead(+8/+C).
                            init_dispatcher_header(
                                memory,
                                obj_addr,
                                DISPATCHER_TYPE_THREAD,
                                0x10,
                                0,
                            );
                        }
                        guest_handle
                    })
                    .unwrap_or_else(|| {
                        state.next_handle += 1;
                        state.next_handle
                    })
            };

            // Write thread handle to args[0]
            if args[0] != 0 {
                memory.write_u32(args[0], thread_handle);
            }
            // Xbox kernel creates thread running SystemRoutine(StartRoutine, StartContext).
            // args[5] = StartRoutine (actual game function, e.g. 0x002A9BC9)
            // args[6] = StartContext (passed to StartRoutine by SystemRoutine)
            // args[9] = SystemRoutine (thread wrapper, e.g. XapiThreadStartup 0x002A9A1D)
            //
            // XapiThreadStartup(StartRoutine, StartContext) does:
            //   1. TLS init, copies static data to thread-local storage
            //   2. Processes thread notify callbacks
            //   3. Calls StartRoutine(StartContext) — the actual game function
            //
            // Worker stack layout: [ret=0, StartRoutine, StartContext]
            let start_routine = args[5];
            let start_context = args[6];
            let system_routine = args[9];
            // Log all args for debugging
            {
                use std::io::Write;
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(r"./debug.log")
                {
                    let _ = writeln!(f, "[rustemu] PsCreateSystemThreadEx: all args: [{:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}]",
                        args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
                    let _ = writeln!(f, "[rustemu] PsCreateSystemThreadEx: system=0x{:08X} start=0x{:08X} ctx=0x{:08X}",
                        system_routine, start_routine, start_context);
                }
            }
            // Enter via SystemRoutine (XapiThreadStartup) if available.
            // XapiThreadStartup does TLS init + thread notify callbacks, then calls StartRoutine.
            // Previously bypassed (matching C++), but missing TLS init may prevent
            // the game from progressing past D3D table init to CreateDevice.
            let entry = if system_routine != 0 {
                system_routine
            } else {
                start_routine
            };
            if entry != 0 && !state.worker_launched {
                // First thread: spawn real OS worker thread
                state.worker_launched = true;
                log::info!("kernel: PsCreateSystemThreadEx SPAWN entry=0x{:08X} start=0x{:08X} ctx=0x{:08X}",
                    entry, start_routine, start_context);
                Some((
                    KernelResult::SpawnThread {
                        entry,
                        start_routine,
                        context: start_context,
                        thread_handle,
                    },
                    0,
                ))
            } else if entry != 0 {
                // Additional threads: spawn real OS threads (needed for game init — Spider-Man creates 11)
                crate::xbox::emulator::debug_log(&format!(
                    "PsCreateSystemThreadEx: SPAWN additional thread entry=0x{:08X} start=0x{:08X} ctx=0x{:08X}",
                    entry, start_routine, start_context
                ));
                Some((
                    KernelResult::SpawnThread {
                        entry,
                        start_routine,
                        context: start_context,
                        thread_handle,
                    },
                    0,
                ))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }
        ordinals::PsTerminateSystemThread => {
            log::info!("kernel: PsTerminateSystemThread(0x{:X})", args[0]);
            Some((KernelResult::ThreadExit, 0))
        }
        ordinals::PsQueryStatistics => Some((KernelResult::Handled, 0)),
        ordinals::PsSetCreateThreadNotifyRoutine => Some((KernelResult::Handled, 0)),

        // ---- I/O Manager ----
        ordinals::IoAllocateIrp
        | ordinals::IoFreeIrp
        | ordinals::IoInitializeIrp
        | ordinals::IoDeleteDevice
        | ordinals::IoCreateFile => Some((KernelResult::Handled, 0)),
        ordinals::IoCreateDevice => {
            // IoCreateDevice(DriverObject, ExtSize, DeviceName, DeviceType, Exclusive, *pDeviceObject)
            // Real kernel (v4627) at 0x80015B24: DEVICE_OBJECT is 0x48 bytes base + extension.
            // ret 0x18 = 6 args stdcall.
            let driver_obj = args[0];
            let ext_size = args[1];
            let _device_name = args[2]; // PSTRING (ignored for HLE)
            let device_type = args[3];
            let exclusive = args[4] != 0;
            let out_ptr = args[5]; // PDEVICE_OBJECT*

            // Align extension to 8 bytes (real kernel does this)
            let aligned_ext = (ext_size + 7) & !7;
            let total = aligned_ext + 0x48;

            static DEVICE_BUMP: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0x00D0_0000);
            let dev = DEVICE_BUMP.fetch_add(total, std::sync::atomic::Ordering::Relaxed);

            // Zero-fill entire allocation (real kernel does rep stosd)
            for i in 0..total {
                memory.write_u8(dev + i, 0);
            }

            // +0x00: Type = 3 (IO_TYPE_DEVICE)
            memory.write_u16(dev + 0x00, 3);
            // +0x02: Size = ext_size + 0x48 (unaligned, matches real kernel)
            memory.write_u16(dev + 0x02, (ext_size + 0x48) as u16);
            // +0x08: DriverObject
            memory.write_u32(dev + 0x08, driver_obj);
            // +0x1C: DeviceType (byte)
            memory.write_u8(dev + 0x1C, device_type as u8);

            // Disk-like types (7=DISK, 0x3A, 2=CD_ROM, 0x3B) get special init
            let is_disk_type =
                device_type == 7 || device_type == 0x3A || device_type == 2 || device_type == 0x3B;
            if is_disk_type {
                memory.write_u8(dev + 0x34, 1);
                memory.write_u8(dev + 0x36, 4);
                memory.write_u32(dev + 0x38, 1);
                // +0x3C/+0x40: LIST_ENTRY self-referencing
                memory.write_u32(dev + 0x40, dev + 0x3C);
                memory.write_u32(dev + 0x3C, dev + 0x3C); // real kernel: Flink = [+0x40]
                                                          // +0x0C: MountedOrSelfDevice = NULL for disk types
            } else {
                // +0x0C: MountedOrSelfDevice = self
                memory.write_u32(dev + 0x0C, dev);
            }

            // +0x14: Flags = 0x10 | (exclusive ? 2 : 0) | (named ? 8 : 0)
            let mut flags: u32 = 0x10;
            if exclusive {
                flags |= 2;
            }
            if _device_name != 0 {
                flags |= 8;
            }
            memory.write_u32(dev + 0x14, flags);

            // +0x18: DeviceExtension pointer (NULL if no extension)
            if ext_size > 0 {
                memory.write_u32(dev + 0x18, dev + 0x48);
            }

            // +0x1E: StackSize = 1
            memory.write_u8(dev + 0x1E, 1);

            // +0x28: KeInitializeDeviceQueue (12 bytes)
            // Real kernel at 0x8001A24F:
            //   Type=0x14, Size=0x0C, Busy=0, Flink=Blink=&DeviceListHead
            let queue = dev + 0x28;
            memory.write_u16(queue + 0x00, 0x14); // Type
            memory.write_u8(queue + 0x02, 0x0C); // Size
            memory.write_u8(queue + 0x03, 0); // Busy
            memory.write_u32(queue + 0x04, queue + 0x04); // Flink = self
            memory.write_u32(queue + 0x08, queue + 0x04); // Blink = self

            // Write device pointer to output
            if out_ptr != 0
                && (out_ptr < 0x2000_0000 || (out_ptr >= 0x8000_0000 && out_ptr < 0xA000_0000))
            {
                memory.write_u32(out_ptr, dev);
            }

            crate::xbox::emulator::debug_log(&format!(
                "IoCreateDevice: drv=0x{:08X} ext={} type={} flags=0x{:X} → dev=0x{:08X}",
                driver_obj, ext_size, device_type, flags, dev
            ));
            Some((KernelResult::Handled, 0)) // STATUS_SUCCESS
        }
        ordinals::IoCreateSymbolicLink => {
            // IoCreateSymbolicLink(PSTRING SymbolicLinkName, PSTRING DeviceName) — 2 args
            let link_name = crate::xbox::kernel::file::guest_ansi_string(memory, args[0]);
            let device_name = crate::xbox::kernel::file::guest_ansi_string(memory, args[1]);
            crate::xbox::emulator::debug_log(&format!(
                "IoCreateSymbolicLink: '{}' → '{}'",
                link_name, device_name
            ));
            // Store in symlink table for dynamic path resolution
            if let Some(ref mut fs) = state.file_state {
                fs.add_symlink(&link_name, &device_name);
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::IoDeleteSymbolicLink => Some((KernelResult::Handled, 0)),
        ordinals::IoBuildAsynchronousFsdRequest
        | ordinals::IoBuildDeviceIoControlRequest
        | ordinals::IoBuildSynchronousFsdRequest => Some((KernelResult::Handled, 0)),
        ordinals::IoCheckShareAccess
        | ordinals::IoRemoveShareAccess
        | ordinals::IoSetShareAccess => Some((KernelResult::Handled, 0)),
        ordinals::IoStartNextPacket
        | ordinals::IoStartNextPacketByKey
        | ordinals::IoStartPacket => Some((KernelResult::Handled, 0)),
        ordinals::IoQueryFileInformation | ordinals::IoQueryVolumeInformation => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::IoQueueThreadIrp | ordinals::IoSetIoCompletion => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::IoSynchronousDeviceIoControlRequest | ordinals::IoSynchronousFsdRequest => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::IofCallDriver | ordinals::IofCompleteRequest => Some((KernelResult::Handled, 0)),
        ordinals::IoInvalidDeviceRequest => Some((KernelResult::Handled, 0xC000_0010)), // STATUS_INVALID_DEVICE_REQUEST
        ordinals::IoDismountVolume | ordinals::IoDismountVolumeByName => {
            Some((KernelResult::Handled, 0))
        }
        ordinals::IoMarkIrpMustComplete => Some((KernelResult::Handled, 0)),

        // ---- Filesystem cache ----
        ordinals::FscGetCacheSize => Some((KernelResult::Handled, 0x40000)), // 256KB
        ordinals::FscSetCacheSize | ordinals::FscInvalidateIdleBlocks => {
            Some((KernelResult::Handled, 0))
        }

        // ---- PHY ----
        ordinals::PhyInitialize => {
            crate::xbox::emulator::debug_log("[PHY] PhyInitialize -> STATUS_SUCCESS");
            Some((KernelResult::Handled, 0))
        }
        ordinals::PhyGetLinkState => {
            // Match the common Xbox link-state shape: active, 100 Mbps, full duplex.
            const XNET_ETHERNET_LINK_ACTIVE: u32 = 0x01;
            const XNET_ETHERNET_LINK_100MBPS: u32 = 0x02;
            const XNET_ETHERNET_LINK_FULL_DUPLEX: u32 = 0x08;
            let link_state = XNET_ETHERNET_LINK_ACTIVE
                | XNET_ETHERNET_LINK_100MBPS
                | XNET_ETHERNET_LINK_FULL_DUPLEX;
            crate::xbox::emulator::debug_log(&format!(
                "[PHY] PhyGetLinkState -> 0x{link_state:08X}"
            ));
            Some((KernelResult::Handled, link_state))
        }

        // ---- Port I/O ----
        ordinals::READ_PORT_BUFFER_UCHAR
        | ordinals::READ_PORT_BUFFER_USHORT
        | ordinals::READ_PORT_BUFFER_ULONG
        | ordinals::WRITE_PORT_BUFFER_UCHAR
        | ordinals::WRITE_PORT_BUFFER_USHORT
        | ordinals::WRITE_PORT_BUFFER_ULONG => Some((KernelResult::Handled, 0)),

        // ---- Crypto ----
        ordinals::XcSHAInit => {
            // XcSHAInit(SHA_CTX* ctx) — initialize 92-byte SHA1 context
            let ctx_ptr = args[0];
            if ctx_ptr != 0 {
                // SHA1 initial hash values at offset 0 (5 x u32)
                memory.write_u32(ctx_ptr, 0x6745_2301);
                memory.write_u32(ctx_ptr + 4, 0xEFCD_AB89);
                memory.write_u32(ctx_ptr + 8, 0x98BA_DCFE);
                memory.write_u32(ctx_ptr + 12, 0x1032_5476);
                memory.write_u32(ctx_ptr + 16, 0xC3D2_E1F0);
                // Count at offset 20 (2 x u32)
                memory.write_u32(ctx_ptr + 20, 0);
                memory.write_u32(ctx_ptr + 24, 0);
                // Buffer at offset 28 (64 bytes) — zeroed
                for i in (0u32..64).step_by(4) {
                    memory.write_u32(ctx_ptr + 28 + i, 0);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::XcSHAUpdate => {
            // XcSHAUpdate(SHA_CTX* ctx, BYTE* data, DWORD len)
            // Stub: just update the byte count so XcSHAFinal produces something
            let ctx_ptr = args[0];
            let data_len = args[2];
            if ctx_ptr != 0 {
                let count = memory.read_u32(ctx_ptr + 20);
                memory.write_u32(ctx_ptr + 20, count.wrapping_add(data_len));
            }
            Some((KernelResult::Handled, 0))
        }
        ordinals::XcSHAFinal => {
            // XcSHAFinal(SHA_CTX* ctx, BYTE digest[20])
            // Write a deterministic fake digest based on the hash state
            let ctx_ptr = args[0];
            let digest = args[1];
            if digest != 0 {
                // Write the 5 state words as the "digest"
                for i in 0u32..5 {
                    let h = if ctx_ptr != 0 {
                        memory.read_u32(ctx_ptr + i * 4)
                    } else {
                        0
                    };
                    memory.write_u32(digest + i * 4, h);
                }
            }
            Some((KernelResult::Handled, 0))
        }
        // VOID functions — return 0
        ordinals::XcRC4Key
        | ordinals::XcRC4Crypt
        | ordinals::XcDESKeyParity
        | ordinals::XcKeyTable
        | ordinals::XcBlockCrypt
        | ordinals::XcBlockCryptCBC
        | ordinals::XcCryptService
        | ordinals::XcUpdateCrypto => Some((KernelResult::Handled, 0)),
        // BOOLEAN-returning functions — return 1 (TRUE = success)
        ordinals::XcHMAC | ordinals::XcVerifyPKCS1Signature => Some((KernelResult::Handled, 1)),
        // Size-returning functions
        ordinals::XcPKGetKeyLen => {
            Some((KernelResult::Handled, 128)) // 1024-bit key = 128 bytes
        }
        // NTSTATUS-returning — return 0 (STATUS_SUCCESS)
        ordinals::XcPKEncPublic | ordinals::XcPKDecPrivate | ordinals::XcModExp => {
            Some((KernelResult::Handled, 0))
        }

        // XeLoadSection/XeUnloadSection and file I/O handled by file.rs
        _ => None, // Not handled by HAL module
    }
}

fn align_up(val: u32, align: u32) -> u32 {
    (val + align - 1) & !(align - 1)
}

/// Bump allocate from the pool for HAL string operations (small buffers).
fn bump_alloc_hal(bump: &mut u32, size: u32, memory: &GuestMemory) -> u32 {
    let addr = *bump;
    let ceil = 0x8400_0000u32;
    let end = addr.wrapping_add(size);
    if end > ceil || end < addr {
        return 0;
    }
    for i in (0..size).step_by(4) {
        memory.write_u32(addr + i, 0);
    }
    *bump = end;
    addr
}

/// Read null-terminated ASCII string from guest memory.
/// Compare two guest memory strings, optionally case-insensitive.
/// Returns 0 if equal, <0 if s1 < s2, >0 if s1 > s2.
fn guest_strncmp(
    memory: &GuestMemory,
    buf1: u32,
    buf2: u32,
    len: u32,
    case_insensitive: bool,
) -> i32 {
    for i in 0..len {
        let mut c1 = memory.read_u8(buf1 + i) as i32;
        let mut c2 = memory.read_u8(buf2 + i) as i32;
        if case_insensitive {
            if (b'A'..=b'Z').contains(&(c1 as u8)) {
                c1 += 32;
            }
            if (b'A'..=b'Z').contains(&(c2 as u8)) {
                c2 += 32;
            }
        }
        if c1 != c2 {
            return c1 - c2;
        }
    }
    0
}

fn read_guest_string(memory: &GuestMemory, addr: u32, max_len: usize) -> String {
    let mut bytes = Vec::with_capacity(max_len);
    for i in 0..max_len {
        let b = memory.read_u8(addr + i as u32);
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).to_string()
}

/// Get length of null-terminated ASCII string in guest memory.
fn guest_read_string(memory: &GuestMemory, addr: u32, max_len: u32) -> String {
    let mut s = Vec::new();
    for i in 0..max_len {
        let b = memory.read_u8(addr + i);
        if b == 0 {
            break;
        }
        s.push(b);
    }
    String::from_utf8_lossy(&s).to_string()
}

fn guest_strlen(memory: &GuestMemory, addr: u32, max: usize) -> u32 {
    for i in 0..max {
        if memory.read_u8(addr + i as u32) == 0 {
            return i as u32;
        }
    }
    max as u32
}

/// Get length of null-terminated wide string (UTF-16LE) in guest memory (in chars).
fn guest_wstrlen(memory: &GuestMemory, addr: u32, max: usize) -> u32 {
    for i in 0..max {
        if memory.read_u16(addr + (i as u32) * 2) == 0 {
            return i as u32;
        }
    }
    max as u32
}
