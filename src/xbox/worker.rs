use super::emulator::{
    debug_log, SharedWorkerCounters, VBlankSignal, WorkerStats, WORKER_THREAD_ID,
};
use crate::xbox::aot::runtime::{
    RuntimeContext, AOT_EXIT_ERROR, AOT_EXIT_HALT, AOT_EXIT_KERNEL_CALL, AOT_EXIT_RET_TO_ZERO,
    AOT_EXIT_RUNNING, AOT_EXIT_SPAWN_THREAD, AOT_EXIT_SYSTEM_TRAP, AOT_EXIT_THREAD_EXIT,
    AOT_EXIT_UNRESOLVED,
};
use crate::xbox::memory::guest_memory::{
    GuestMemory, RAM_MIRROR_BASE, RAM_SIZE, RAM_UNCACHED_BASE,
};
use std::sync::atomic::Ordering;
/// Worker thread dispatch loop.
/// Extracted from emulator.rs — no logic changes.
use std::sync::Arc;

// ============================================================================
// Worker thread dispatch loop
// ============================================================================

/// Cached Doom game state pointer (heap-allocated, ~0x050007C0).
/// [0x101BB8] gets overwritten by TryRunTics, so we cache during CRT init.
pub static DOOM_GAME_STATE_CACHE: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

pub fn doom_synthetic_framebuffer_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_DOOM_SYNTH_FB")
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

pub fn doom_clear_synthetic_framebuffer_flag_from_guest_mem(guest_mem: *mut u8) {
    unsafe {
        *((guest_mem as u64 + DOOM_FB_FLAG as u64) as *mut u32) = 0;
    }
}

const MAX_GUEST_THREADS: u32 = 32; // Spider-Man creates many short-lived loader/audio threads.
static CHILD_THREAD_NUM: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
// Late in Spider-Man boot, RET_TO_ZERO stack scanning repeatedly picks stale
// call-return continuations from XGRPH/Bink parser frames. Treat that as a
// diagnostic fallback, not a normal scheduler path.
const ENABLE_RET0_STACK_SCAN_RECOVERY: bool = false;

/// Normalize guest address (strip 0x80000000 mirror bit).
pub fn normalize_guest_addr(addr: u32) -> u32 {
    if addr >= 0x8000_0000 && addr < 0xA000_0000 {
        addr & 0x1FFF_FFFF
    } else {
        addr
    }
}

fn guest_data_addr(addr: u32) -> Option<u32> {
    let normalized = normalize_guest_addr(addr);
    if (0x1000..0x2000_0000).contains(&normalized) {
        Some(normalized)
    } else {
        None
    }
}

fn take_pending_dpc_object(memory: &GuestMemory, reason: &str) -> Option<u32> {
    let raw = crate::xbox::aot::veh_dpc::take_queued_dpc_object()?;
    let log_enabled = {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| std::env::var_os("RUSTEMU_DPC_TAKE_LOG").is_some())
    };
    let Some(dpc) = guest_data_addr(raw) else {
        if log_enabled {
            static DPC_TAKE_INVALID_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = DPC_TAKE_INVALID_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 8 || n.is_power_of_two() {
                debug_log(&format!(
                    "[DPC-TAKE] #{} reason={} invalid obj=0x{:08X}",
                    n, reason, raw
                ));
            }
        }
        return Some(raw);
    };

    let inserted_before = memory.read_u8(dpc + 0x02);
    memory.write_u8(dpc + 0x02, 0);
    let inserted_after = memory.read_u8(dpc + 0x02);

    if log_enabled {
        static DPC_TAKE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DPC_TAKE_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 24 || n.is_power_of_two() {
            debug_log(&format!(
                "[DPC-TAKE] #{} reason={} obj=0x{:08X}->0x{:08X} inserted={}→{} routine=0x{:08X} ctx=0x{:08X} sa1=0x{:08X} sa2=0x{:08X}",
                n,
                reason,
                raw,
                dpc,
                inserted_before,
                inserted_after,
                memory.read_u32(dpc + 0x0C),
                memory.read_u32(dpc + 0x10),
                memory.read_u32(dpc + 0x14),
                memory.read_u32(dpc + 0x18)
            ));
        }
    }

    Some(dpc)
}

fn read_inline_ascii(memory: &GuestMemory, addr: u32, max_len: usize) -> String {
    let Some(addr) = guest_data_addr(addr) else {
        return String::new();
    };

    let mut out = String::new();
    for i in 0..max_len {
        let b = memory.read_u8(addr.wrapping_add(i as u32));
        if b == 0 {
            break;
        }
        if b.is_ascii_graphic() || b == b' ' {
            out.push(b as char);
        } else {
            out.push('.');
        }
    }
    out
}

fn log_spidey_file_kcall(
    memory: &GuestMemory,
    source: &str,
    kernel_calls: u64,
    ordinal: u32,
    guest_eip: u32,
    ret_addr: u32,
    esp_before: u32,
    ebp: u32,
    args: &[u32; 12],
) {
    if ordinal != crate::xbox::kernel::ordinals::NtCreateFile
        && ordinal != crate::xbox::kernel::ordinals::NtOpenFile
        && ordinal != crate::xbox::kernel::ordinals::NtWriteFile
    {
        return;
    }

    static SPIDEY_FILE_KCALL_LOG: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0);
    let n = SPIDEY_FILE_KCALL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n >= 160 && !n.is_power_of_two() {
        return;
    }

    let path = if ordinal == crate::xbox::kernel::ordinals::NtCreateFile
        || ordinal == crate::xbox::kernel::ordinals::NtOpenFile
    {
        crate::xbox::kernel::file::extract_object_path(memory, args[2]).unwrap_or_default()
    } else {
        String::new()
    };

    let mut chain = String::new();
    for off in (0..0x80u32).step_by(4) {
        let v = memory.read_u32(esp_before.wrapping_add(off));
        if (0x0001_0000..0x0040_0000).contains(&v) {
            chain.push_str(&format!(" +{:02X}=0x{:08X}", off, v));
        }
    }

    let lower_path = path.to_ascii_lowercase();
    let deep_file_probe = lower_path.contains("menu.xbs")
        || lower_path.contains("origin")
        || lower_path.contains("peter")
        || lower_path.contains("m0menu")
        || lower_path.contains("vshader.key");
    let mut stack32 = String::new();
    let mut ebp_chain = String::new();
    if deep_file_probe {
        for off in (0..0x80u32).step_by(4) {
            let v = memory.read_u32(esp_before.wrapping_add(off));
            stack32.push_str(&format!(" +{:02X}=0x{:08X}", off, v));
        }

        let mut frame_ebp = normalize_guest_addr(ebp);
        for depth in 0..8u32 {
            if !(0x1000..0x2000_0000).contains(&frame_ebp) {
                break;
            }
            let next = normalize_guest_addr(memory.read_u32(frame_ebp));
            let frame_ret = memory.read_u32(frame_ebp.wrapping_add(4));
            ebp_chain.push_str(&format!(
                " #{} ebp=0x{:08X} ret=0x{:08X} next=0x{:08X}",
                depth, frame_ebp, frame_ret, next
            ));
            if next == 0 || next == frame_ebp {
                break;
            }
            frame_ebp = next;
        }
    }

    debug_log(&format!(
        "[SPIDEY-FILE-KCALL #{}] source={} kcall={} ord={} ({}) path='{}' guest_eip=0x{:08X} ret=0x{:08X} esp=0x{:08X} ebp=0x{:08X} args=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] chain:{}{}{}{}{}",
        n,
        source,
        kernel_calls,
        ordinal,
        crate::xbox::kernel::ordinals::name(ordinal),
        path,
        guest_eip,
        ret_addr,
        esp_before,
        ebp,
        args[0],
        args[1],
        args[2],
        args[3],
        args[4],
        args[5],
        args[6],
        args[7],
        args[8],
        chain,
        if stack32.is_empty() { "" } else { " stack32:" },
        stack32,
        if ebp_chain.is_empty() { "" } else { " ebp_chain:" },
        ebp_chain
    ));
}

fn log_spiderman_scene38_change(
    memory: &GuestMemory,
    ctx: &RuntimeContext,
    ordinal: u32,
    kernel_calls: u64,
    esp_before: u32,
    phase: &str,
) {
    const SCENE_GLOBAL: u32 = 0x003F_5BEC;

    let scene = memory.read_u32(SCENE_GLOBAL);
    let Some(scene_phys) = guest_data_addr(scene) else {
        return;
    };

    let field_addr = scene_phys.wrapping_add(0x38);
    let value = memory.read_u32(field_addr);
    static LAST_SCENE38: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0xFFFF_FFFF);
    static SCENE38_LOGS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let previous = LAST_SCENE38.swap(value, Ordering::Relaxed);
    if previous == value || previous == 0xFFFF_FFFF {
        return;
    }

    let inline = read_inline_ascii(memory, field_addr, 64);
    let ptr_ascii = read_inline_ascii(memory, value, 64);
    let interesting = value == 0x645C_3A64
        || inline.to_ascii_lowercase().contains("d:\\data")
        || inline.to_ascii_lowercase().contains("d:/data");
    let n = SCENE38_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 96 || interesting || n.is_power_of_two() {
        let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
        debug_log(&format!(
            "[SCENE38-WATCH] #{} {} kcall #{} ord={} ({}) 0x{:08X}->0x{:08X} \
             scene=0x{:08X} field=0x{:08X} inline='{}' ptr_ascii='{}' \
             esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X}",
            n,
            phase,
            kernel_calls,
            ordinal,
            ord_name,
            previous,
            value,
            scene,
            field_addr,
            inline,
            ptr_ascii,
            esp_before,
            ctx.guest.eax,
            ctx.guest.ecx,
            ctx.guest.edx,
            ctx.guest.ebx,
            ctx.guest.esi,
            ctx.guest.edi,
            ctx.guest.ebp
        ));
    }
}

fn log_spiderman_scene28_change(
    memory: &GuestMemory,
    ctx: &RuntimeContext,
    ordinal: u32,
    kernel_calls: u64,
    esp_before: u32,
    phase: &str,
) {
    const SCENE_GLOBAL: u32 = 0x003F_5BEC;
    const XGRAPH_GLOBAL: u32 = 0x004C_06B8;

    let scene = memory.read_u32(SCENE_GLOBAL);
    let Some(scene_phys) = guest_data_addr(scene) else {
        return;
    };

    let field_addr = scene_phys.wrapping_add(0x28);
    let value = memory.read_u32(field_addr);
    static LAST_SCENE28: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0xFFFF_FFFF);
    static SCENE28_LOGS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let previous = LAST_SCENE28.swap(value, Ordering::Relaxed);
    if previous == value || previous == 0xFFFF_FFFF {
        return;
    }

    let xgraph = memory.read_u32(XGRAPH_GLOBAL);
    let xgraph_phys = guest_data_addr(xgraph);
    let x_144 = xgraph_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x144)));
    let x_154 = xgraph_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x154)));
    let x_15c = xgraph_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x15C)));
    let x_1e4 = xgraph_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x1E4)));
    let x_1e8 = xgraph_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x1E8)));
    let root_phys = guest_data_addr(value);
    let r_144 = root_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x144)));
    let r_154 = root_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x154)));
    let r_15c = root_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x15C)));
    let r_1e4 = root_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x1E4)));
    let r_1e8 = root_phys.map_or(0, |p| memory.read_u32(p.wrapping_add(0x1E8)));

    let n = SCENE28_LOGS.fetch_add(1, Ordering::Relaxed);
    if n < 96 || n.is_power_of_two() || value <= 0x1000 || previous <= 0x1000 {
        let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
        debug_log(&format!(
            "[SCENE28-WATCH] #{} {} kcall #{} ord={} ({}) 0x{:08X}->0x{:08X} \
             scene=0x{:08X} field=0x{:08X} xgraph=0x{:08X} x144/154/15c/1e4/1e8=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} \
             root144/154/15c/1e4/1e8=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} \
             esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X}",
            n,
            phase,
            kernel_calls,
            ordinal,
            ord_name,
            previous,
            value,
            scene,
            field_addr,
            xgraph,
            x_144,
            x_154,
            x_15c,
            x_1e4,
            x_1e8,
            r_144,
            r_154,
            r_15c,
            r_1e4,
            r_1e8,
            esp_before,
            ctx.guest.eax,
            ctx.guest.ecx,
            ctx.guest.edx,
            ctx.guest.ebx,
            ctx.guest.esi,
            ctx.guest.edi,
            ctx.guest.ebp
        ));
    }
}

fn is_guest_code_addr(ctx: &RuntimeContext, addr: u32) -> bool {
    let normalized = normalize_guest_addr(addr);
    let in_exec = ctx
        .exec_ranges
        .iter()
        .any(|&(start, end)| normalized >= start && normalized < end);
    let in_data = ctx
        .data_ranges
        .iter()
        .any(|&(start, end)| normalized >= start && normalized < end);
    in_exec && !in_data
}

fn read_ke_delay_interval_100ns(memory: &GuestMemory, interval_ptr: u32) -> i64 {
    if guest_i64_ptr_valid(interval_ptr) {
        let lo = memory.read_u32(interval_ptr) as i64;
        let hi = memory.read_u32(interval_ptr + 4) as i64;
        (hi << 32) | (lo & 0xFFFF_FFFF)
    } else {
        0
    }
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

fn find_ke_delay_caller(ctx: &RuntimeContext, memory: &GuestMemory, esp_before: u32) -> u32 {
    let jmp_ret = normalize_guest_addr(ctx.kernel_jmp_ret_addr);
    if jmp_ret != 0 && is_guest_code_addr(ctx, jmp_ret) {
        return jmp_ret;
    }

    let pre_cleanup = ctx.kernel_r14_pre_cleanup;
    if pre_cleanup != 0 && pre_cleanup < 0x2000_0000 {
        let candidate = normalize_guest_addr(memory.read_u32(pre_cleanup));
        if candidate != 0 && is_guest_code_addr(ctx, candidate) {
            return candidate;
        }
    }

    for off in (0..0x100u32).step_by(4) {
        let addr = esp_before.wrapping_add(off);
        if addr >= 0x2000_0000 {
            break;
        }
        let candidate = normalize_guest_addr(memory.read_u32(addr));
        if candidate != 0 && is_guest_code_addr(ctx, candidate) {
            return candidate;
        }
    }

    normalize_guest_addr(ctx.guest.exit_guest_addr)
}

fn spawn_child_thread_from_worker(
    parent_ctx: &RuntimeContext,
    memory: &GuestMemory,
    vblank: &Arc<VBlankSignal>,
    entry: u32,
    start_routine: u32,
    start_context: u32,
    thread_handle: u32,
    source: &str,
) {
    let tnum = CHILD_THREAD_NUM.fetch_add(1, Ordering::Relaxed);
    if tnum >= MAX_GUEST_THREADS {
        debug_log(&format!(
            "Worker: thread #{} SKIPPED by {} (limit={}) entry=0x{:08X} start=0x{:08X}",
            tnum, source, MAX_GUEST_THREADS, entry, start_routine
        ));
        return;
    }

    let mut child_ctx = Box::new(RuntimeContext::new_worker(parent_ctx));
    child_ctx.kernel_state = parent_ctx.kernel_state;
    let child_mem = memory.clone_shared();
    let child_mem_for_signal = child_mem.clone_shared();
    let child_kernel_state = child_ctx.kernel_state as usize;
    let child_counters = Arc::new(SharedWorkerCounters {
        dispatch_count: std::sync::atomic::AtomicU64::new(0),
        mmio_count: std::sync::atomic::AtomicU64::new(0),
        pb_commands: std::sync::atomic::AtomicU64::new(0),
        draw_calls: std::sync::atomic::AtomicU64::new(0),
        kernel_calls: std::sync::atomic::AtomicU64::new(0),
        alive: std::sync::atomic::AtomicBool::new(true),
    });
    let child_vblank = Arc::clone(vblank);
    let child_counters2 = Arc::clone(&child_counters);
    debug_log(&format!(
        "Worker: spawning child thread #{} via {} entry=0x{:08X} start=0x{:08X} ctx=0x{:08X} handle=0x{:08X}",
        tnum, source, entry, start_routine, start_context, thread_handle
    ));
    let _ = std::thread::Builder::new()
        .name(format!("xbox-child-{}", tnum))
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            debug_log(&format!(
                "[CHILD-{}] Thread started, entering dispatch loop",
                tnum
            ));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                worker_dispatch_loop(
                    child_ctx,
                    child_mem,
                    entry,
                    start_routine,
                    start_context,
                    child_counters2,
                    child_vblank,
                )
            })) {
                Ok(_stats) => {
                    debug_log(&format!("[CHILD-{}] Thread exited normally", tnum));
                }
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        format!("{:?}", e.type_id())
                    };
                    debug_log(&format!("[CHILD-{}] PANIC: {}", tnum, msg));
                }
            }
            signal_guest_thread_object(
                child_kernel_state,
                &child_mem_for_signal,
                thread_handle,
                tnum,
            );
        });
}

fn signal_guest_thread_object(
    kernel_state: usize,
    memory: &GuestMemory,
    thread_handle: u32,
    tnum: u32,
) {
    if kernel_state == 0 || thread_handle == 0 {
        return;
    }
    let state = unsafe { &mut *(kernel_state as *mut crate::xbox::kernel::KernelState) };
    if let Some(native) = state.get_native_handle(thread_handle) {
        unsafe {
            let _ = windows::Win32::System::Threading::SetEvent(
                windows::Win32::Foundation::HANDLE(native as _),
            );
        }
    }
    if let Some(obj_addr) = state.get_object_addr(thread_handle) {
        memory.write_u32(obj_addr + 0x04, 1);
        debug_log(&format!(
            "[THREAD-OBJ] child #{} signaled handle=0x{:08X} obj=0x{:08X}",
            tnum, thread_handle, obj_addr
        ));
    } else {
        debug_log(&format!(
            "[THREAD-OBJ] child #{} had no tracked object for handle=0x{:08X}",
            tnum, thread_handle
        ));
    }
}

fn observe_spidey_scene_loader_latch(
    memory: &GuestMemory,
    state: &crate::xbox::kernel::KernelState,
    ordinal: u32,
    kernel_calls: u64,
    esp_before: u32,
) {
    const SCENE_GLOBAL: u32 = 0x003F_5BEC;
    const SCENE_LOADER_THREAD: u32 = 0x0001_C550;

    let scene = memory.read_u32(SCENE_GLOBAL);
    let Some(scene_phys) = guest_data_addr(scene) else {
        return;
    };

    let handle = memory.read_u32(scene_phys.wrapping_add(0x118));
    if handle == 0 {
        return;
    }

    let Some(obj) = state.object_table.get(&handle) else {
        return;
    };
    if obj.obj_type != crate::xbox::kernel::XboxObjectType::Thread {
        return;
    }

    let signal_state = memory.read_u32(obj.guest_obj_addr.wrapping_add(0x04));
    if signal_state == 0 {
        return;
    }

    static CLEAR_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = CLEAR_COUNT.fetch_add(1, Ordering::Relaxed);
    if n < 12 || n.is_power_of_two() {
        debug_log(&format!(
            "[SPIDEY-SCENE118-OBS] #{} scene=0x{:08X} handle=0x{:08X} obj=0x{:08X} signal={} \
             ord={} ({}) kcall={} esp=0x{:08X} loader=0x{:08X}",
            n,
            scene_phys,
            handle,
            obj.guest_obj_addr,
            signal_state,
            ordinal,
            crate::xbox::kernel::ordinals::name(ordinal),
            kernel_calls,
            esp_before,
            SCENE_LOADER_THREAD
        ));
    }
}

/// Check if guest memory at `addr` is preceded by a plausible CALL instruction.
///
/// Real return addresses always sit immediately after an x86 CALL. Data pointers
/// that happen to be in an executable section (e.g., KeDpc struct pointers in
/// the D3D section) do NOT have a CALL before them, so this catches the
/// data-as-code stack-scan-recovery pitfall seen with test_blue_padded.
///
/// Recognized CALL encodings:
///   E8 rel32                 (5 bytes)  — direct relative CALL
///   FF D0..D7                (2 bytes)  — CALL reg (EAX..EDI)
///   FF 15 disp32             (6 bytes)  — CALL [disp32]
///   FF 14 sib                (3 bytes)  — CALL [reg*scale+reg]
///   FF 50..57 disp8          (3 bytes)  — CALL [reg+disp8]
///   FF 90..97 disp32         (6 bytes)  — CALL [reg+disp32]
///   FF 54 sib disp8          (4 bytes)  — CALL [reg*scale+reg+disp8]
///   FF 94 sib disp32         (7 bytes)  — CALL [reg*scale+reg+disp32]
///
/// Returns true if any of the recognized encodings matches.
fn looks_like_ret_addr(memory: &GuestMemory, addr: u32) -> bool {
    // Need at least 2 bytes before addr for even the shortest CALL form.
    if addr < 8 {
        return false;
    }

    // Bytes at increasing distance before addr (addr-1, addr-2, ...).
    // All reads are within 512MB; addr is already validated >= 0x10000.
    let b = |offset: u32| -> u8 {
        let a = addr.wrapping_sub(offset);
        if a >= 0x2000_0000 {
            0
        } else {
            memory.read_u8(a)
        }
    };

    // E8 rel32 (5 bytes): addr-5 == 0xE8
    if b(5) == 0xE8 {
        return true;
    }

    // FF D0..D7 (2 bytes): addr-2 == 0xFF, addr-1 in 0xD0..0xD7
    if b(2) == 0xFF && (b(1) & 0xF8) == 0xD0 {
        return true;
    }

    // FF 15 disp32 (6 bytes): addr-6 == 0xFF, addr-5 == 0x15
    if b(6) == 0xFF && b(5) == 0x15 {
        return true;
    }

    // FF 14 sib (3 bytes): addr-3 == 0xFF, addr-2 == 0x14
    if b(3) == 0xFF && b(2) == 0x14 {
        return true;
    }

    // FF 50..57 disp8 (3 bytes): addr-3 == 0xFF, addr-2 in 0x50..0x57
    if b(3) == 0xFF && (b(2) & 0xF8) == 0x50 {
        return true;
    }

    // FF 90..97 disp32 (6 bytes): addr-6 == 0xFF, addr-5 in 0x90..0x97
    if b(6) == 0xFF && (b(5) & 0xF8) == 0x90 {
        return true;
    }

    // FF 54 sib disp8 (4 bytes): addr-4 == 0xFF, addr-3 == 0x54
    if b(4) == 0xFF && b(3) == 0x54 {
        return true;
    }

    // FF 94 sib disp32 (7 bytes): addr-7 == 0xFF, addr-6 == 0x94
    if b(7) == 0xFF && b(6) == 0x94 {
        return true;
    }

    false
}

fn guest_code_prefix(memory: &GuestMemory, addr: u32) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    if addr < 0x2000_0000 {
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = memory.read_u8(addr.wrapping_add(i as u32));
        }
    }
    bytes
}

fn guest_code_all_zero(memory: &GuestMemory, addr: u32) -> Option<[u8; 16]> {
    if !(0x0001_0000..0x2000_0000).contains(&addr) {
        return None;
    }
    let bytes = guest_code_prefix(memory, addr);
    if bytes.iter().all(|&b| b == 0) {
        Some(bytes)
    } else {
        None
    }
}

fn log_zero_guest_code_skip(label: &str, addr: u32, bytes: &[u8; 16]) {
    static ZERO_CODE_SKIP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = ZERO_CODE_SKIP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        debug_log(&format!(
            "[GUEST-CODE-ZERO-SKIP #{}] {} target=0x{:08X} bytes={:02X?}",
            n, label, addr, bytes
        ));
    }
}

fn should_skip_zero_guest_code(
    ctx: &RuntimeContext,
    memory: &GuestMemory,
    addr: u32,
    label: &str,
) -> bool {
    let Some(bytes) = guest_code_all_zero(memory, addr) else {
        return false;
    };

    if ctx.addr_hash.lookup(addr).is_some() {
        static ZERO_CODE_COMPILED_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = ZERO_CODE_COMPILED_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            debug_log(&format!(
                "[GUEST-CODE-ZERO-COMPILED #{}] {} target=0x{:08X} bytes={:02X?}; compiled AOT block exists, firing anyway",
                n, label, addr, bytes
            ));
        }
        return false;
    }

    log_zero_guest_code_skip(label, addr, &bytes);
    true
}

/// Fire a guest routine (ISR or DPC) synchronously from the dispatch loop.
/// Builds a stdcall frame with sentinel return address, enters the trampoline,
/// and runs a mini dispatch loop until the routine RETs to zero.
/// Returns true if the routine ran successfully, false if not compiled.
///
/// This matches real Xbox kernel behavior: DPCs fire during thread scheduling
/// (e.g., when a thread wakes from KeDelayExecutionThread). By firing from the
/// dispatch loop instead of VEH INT3 injection, we guarantee delivery regardless
/// of the guest's INT3 density.
fn fire_guest_routine(
    ctx: &mut Box<RuntimeContext>,
    memory: &GuestMemory,
    vblank: &Arc<VBlankSignal>,
    routine_guest_addr: u32,
    args: &[u32],
    label: &str,
) -> bool {
    if should_skip_zero_guest_code(ctx, memory, routine_guest_addr, label) {
        return false;
    }

    let host_off = match ctx.addr_hash.lookup(routine_guest_addr) {
        Some(off) => off,
        None => {
            debug_log(&format!(
                "[WORKER-DPC] {} at 0x{:08X} NOT in addr_hash — skipping",
                label, routine_guest_addr
            ));
            return false;
        }
    };

    // Save guest GPRs (Pitfall #28: ISR/DPC clobbers everything)
    let saved_eax = ctx.guest.eax;
    let saved_ecx = ctx.guest.ecx;
    let saved_edx = ctx.guest.edx;
    let saved_ebx = ctx.guest.ebx;
    let saved_esi = ctx.guest.esi;
    let saved_edi = ctx.guest.edi;
    let saved_ebp = ctx.guest.ebp;
    let saved_esp = ctx.guest.esp;
    let saved_shadow_stack_top = ctx.guest.shadow_stack_top;

    // Save FPU/SSE state — Xbox DPCs do NOT save/restore FP state per XDK docs:
    // "the floating point processor state is not saved and restored across a DPC call"
    // So we must protect the caller's FPU/SSE regs from DPC clobbering.
    #[repr(align(16))]
    struct FxSaveArea([u8; 512]);
    let mut fxsave_buf = FxSaveArea([0u8; 512]);
    unsafe {
        std::arch::asm!("fxsave [{}]", in(reg) fxsave_buf.0.as_mut_ptr(), options(nostack));
    }

    // Build stdcall frame on a dedicated scheduler/DPC stack:
    // [ret_addr=0, arg0, arg1, ...].
    //
    // Do not borrow the interrupted thread's ESP here. Spider-Man's DPC path
    // passes stack-local helper objects through C++ code and writes through
    // those pointers; when the DPC frame was placed directly under the
    // interrupted NtYieldExecution frame, that code overwrote the suspended
    // SwitchToThread return slot at 0x1EFFFEF4. Real kernel DPC/callback work
    // runs on kernel-owned stack space, so keep this emulation stack isolated
    // from the current guest thread stack.
    const SCHED_DPC_STACK_TOP: u32 = 0x1DFF_F000;
    let frame_size = 4 + args.len() as u32 * 4;
    let new_esp = SCHED_DPC_STACK_TOP.wrapping_sub(frame_size);
    memory.write_u32(new_esp, 0); // ret addr = 0 (RET_TO_ZERO sentinel)
    for (i, &arg) in args.iter().enumerate() {
        memory.write_u32(new_esp + 4 + i as u32 * 4, arg);
    }
    ctx.guest.esp = new_esp;

    {
        static DPC_FIRE_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DPC_FIRE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 5 || n % 100 == 0 {
            debug_log(&format!(
                "[WORKER-DPC] #{} Firing {} at 0x{:08X} (host+0x{:X}), {} args, ESP 0x{:08X}→0x{:08X}",
                n, label, routine_guest_addr, host_off, args.len(), saved_esp, new_esp
            ));
        }
    }

    // Enter trampoline at the routine
    let mut entry = unsafe { ctx.code_base.add(host_off as usize) };
    let mut iterations = 0u32;
    const MAX_ITERATIONS: u32 = 10_000; // safety limit

    // 2026-04-26 diagnostic: when fire_guest_routine hits MAX_ITERATIONS, we
    // have to know WHY. Track per-fire histogram of exit_reasons + which guest
    // addresses we kept exiting on. Cheap: a pair of small HashMaps that only
    // matter on abort. Keep raw counts here (not atomics — single-thread per
    // fire) and serialize the top entries when we abort.
    use std::collections::HashMap;
    let mut exit_reason_hist: HashMap<u32, u32> = HashMap::new();
    let mut exit_addr_hist: HashMap<u32, u32> = HashMap::new();
    let mut last_exit_reason: u32 = 0;
    let mut last_exit_addr: u32 = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            // Sort histograms and emit top entries so we can see WHAT exit
            // reason / WHICH guest address we were stuck on.
            let mut reasons: Vec<(u32, u32)> =
                exit_reason_hist.iter().map(|(k, v)| (*k, *v)).collect();
            reasons.sort_by(|a, b| b.1.cmp(&a.1));
            let mut addrs: Vec<(u32, u32)> = exit_addr_hist.iter().map(|(k, v)| (*k, *v)).collect();
            addrs.sort_by(|a, b| b.1.cmp(&a.1));
            let top_reasons = reasons
                .iter()
                .take(8)
                .map(|(r, c)| format!("reason={}({})", r, c))
                .collect::<Vec<_>>()
                .join(" ");
            let top_addrs = addrs
                .iter()
                .take(10)
                .map(|(a, c)| format!("0x{:08X}({})", a, c))
                .collect::<Vec<_>>()
                .join(" ");
            debug_log(&format!(
                "[WORKER-DPC] {} exceeded {} iterations — aborting last_reason={} last_addr=0x{:08X} reasons[{}] addrs[{}]",
                label, MAX_ITERATIONS, last_exit_reason, last_exit_addr, top_reasons, top_addrs
            ));
            break;
        }

        let guest_prof = crate::xbox::profiler::start(crate::xbox::profiler::CpuPhase::GuestExec);
        let exit_reason = unsafe { crate::xbox::aot::trampoline::enter_guest(ctx, entry) };
        crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::GuestExec, guest_prof);
        last_exit_reason = exit_reason;
        last_exit_addr = ctx.guest.exit_guest_addr;
        *exit_reason_hist.entry(exit_reason).or_insert(0) += 1;
        *exit_addr_hist.entry(last_exit_addr).or_insert(0) += 1;

        if exit_reason == AOT_EXIT_RET_TO_ZERO {
            // Routine returned normally
            crate::rate_log!(
                20,
                "[WORKER-DPC] {} returned after {} iterations",
                label,
                iterations
            );
            break;
        }

        if exit_reason == AOT_EXIT_KERNEL_CALL {
            // Handle kernel call made by the ISR/DPC
            let ordinal = ctx.kernel_ordinal;
            let args_k = ctx.kernel_args;
            let kstate =
                unsafe { &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState) };

            let kr = crate::xbox::kernel::dispatch_inline(
                ordinal,
                &args_k,
                &mut ctx.guest.eax,
                &mut ctx.guest.edx,
                &mut ctx.guest.ecx,
                kstate,
                ctx.guest_mem_base,
            );
            if let crate::xbox::kernel::KernelResult::SpawnThread {
                entry,
                start_routine,
                context,
                thread_handle,
            } = kr
            {
                spawn_child_thread_from_worker(
                    ctx.as_ref(),
                    memory,
                    vblank,
                    entry,
                    start_routine,
                    context,
                    thread_handle,
                    "guest-routine",
                );
            }

            // Set next entry point
            if ctx.kernel_is_call {
                entry = ctx.kernel_host_resume as *mut u8;
            } else {
                let jmp_ret = ctx.kernel_jmp_ret_addr;
                if jmp_ret == 0 {
                    break;
                }
                let normalized = normalize_guest_addr(jmp_ret);
                if let Some(off) = ctx.addr_hash.lookup(normalized) {
                    entry = unsafe { ctx.code_base.add(off as usize) };
                } else {
                    debug_log(&format!(
                        "[WORKER-DPC] {} JMP ret 0x{:08X} not in addr_hash — aborting",
                        label, jmp_ret
                    ));
                    break;
                }
            }
            continue;
        }

        if exit_reason == AOT_EXIT_UNRESOLVED {
            // Rescue-compile and retry
            let unresolved = ctx.guest.exit_guest_addr;
            let normalized = normalize_guest_addr(unresolved);
            let in_exec_range = ctx
                .exec_ranges
                .iter()
                .any(|&(start, end)| normalized >= start && normalized < end);
            if !in_exec_range {
                debug_log(&format!(
                    "[WORKER-DPC] {} hit non-exec unresolved 0x{:08X} (norm=0x{:08X}) — aborting without rescue",
                    label, unresolved, normalized
                ));
                break;
            }
            if let Some(_host_off) = ctx.rescue_emit(unresolved) {
                // Re-enter at the newly compiled code
                entry = unsafe { ctx.code_base.add(_host_off as usize) };
                continue;
            }
            debug_log(&format!(
                "[WORKER-DPC] {} hit unresolved 0x{:08X} — aborting",
                label, unresolved
            ));
            break;
        }

        // Any other exit: abort
        debug_log(&format!(
            "[WORKER-DPC] {} unexpected exit_reason={} — aborting",
            label, exit_reason
        ));
        break;
    }

    // Restore FPU/SSE state (before GPRs, order doesn't matter but logically pairs with save)
    unsafe {
        std::arch::asm!("fxrstor [{}]", in(reg) fxsave_buf.0.as_ptr(), options(nostack));
    }

    // Restore guest GPRs
    ctx.guest.eax = saved_eax;
    ctx.guest.ecx = saved_ecx;
    ctx.guest.edx = saved_edx;
    ctx.guest.ebx = saved_ebx;
    ctx.guest.esi = saved_esi;
    ctx.guest.edi = saved_edi;
    ctx.guest.ebp = saved_ebp;
    ctx.guest.esp = saved_esp;
    ctx.guest.shadow_stack_top = saved_shadow_stack_top;

    true
}

#[derive(Clone, Copy)]
struct GuestThreadSnapshot {
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    shadow_stack_top: u64,
}

impl GuestThreadSnapshot {
    fn capture(ctx: &RuntimeContext) -> Self {
        Self {
            eax: ctx.guest.eax,
            ecx: ctx.guest.ecx,
            edx: ctx.guest.edx,
            ebx: ctx.guest.ebx,
            esp: ctx.guest.esp,
            ebp: ctx.guest.ebp,
            esi: ctx.guest.esi,
            edi: ctx.guest.edi,
            shadow_stack_top: ctx.guest.shadow_stack_top,
        }
    }

    fn restore(self, ctx: &mut RuntimeContext) {
        ctx.guest.eax = self.eax;
        ctx.guest.ecx = self.ecx;
        ctx.guest.edx = self.edx;
        ctx.guest.ebx = self.ebx;
        ctx.guest.esp = self.esp;
        ctx.guest.ebp = self.ebp;
        ctx.guest.esi = self.esi;
        ctx.guest.edi = self.edi;
        ctx.guest.shadow_stack_top = self.shadow_stack_top;
    }

    fn differs_from(self, ctx: &RuntimeContext) -> bool {
        ctx.guest.eax != self.eax
            || ctx.guest.ecx != self.ecx
            || ctx.guest.edx != self.edx
            || ctx.guest.ebx != self.ebx
            || ctx.guest.esp != self.esp
            || ctx.guest.ebp != self.ebp
            || ctx.guest.esi != self.esi
            || ctx.guest.edi != self.edi
            || ctx.guest.shadow_stack_top != self.shadow_stack_top
    }
}

fn try_fire_isr_dpc_preserving_thread_context(
    ctx: &mut Box<RuntimeContext>,
    memory: &GuestMemory,
    vblank: &Arc<VBlankSignal>,
    reason: &str,
) {
    let saved = GuestThreadSnapshot::capture(ctx.as_ref());
    try_fire_isr_dpc_from_dispatch(ctx, memory, vblank);
    if saved.differs_from(ctx.as_ref()) {
        static RESTORE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = RESTORE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 10 || n.is_power_of_two() {
            debug_log(&format!(
                "[SCHED-CTX-RESTORE] #{} {} restored guest context: ESP 0x{:08X}->0x{:08X} R12 0x{:016X}->0x{:016X}",
                n,
                reason,
                ctx.guest.esp,
                saved.esp,
                ctx.guest.shadow_stack_top,
                saved.shadow_stack_top
            ));
        }
        saved.restore(ctx.as_mut());
    }
}

/// Fire ISR→DPC sequence from the dispatch loop after a wait-related kernel call.
/// Matches real Xbox kernel: VBlank ISR fires → queues DPC → DPC fires during scheduling.
fn try_fire_isr_dpc_from_dispatch(
    ctx: &mut Box<RuntimeContext>,
    memory: &GuestMemory,
    vblank: &Arc<VBlankSignal>,
) {
    use crate::xbox::aot::veh_dpc;

    if !veh_dpc::is_crt_boot_complete() {
        return;
    }

    // Native Direct3D_CreateDevice TAP builds the D3D device, miniport, and
    // VBlank callback state in guest code. Letting the worker fire guest DPC
    // callbacks while that TAP window is still active runs scheduler work on
    // half-built display structures and can leave KPCR/KTHREAD pointers on the
    // guest return path. Defer VBlank delivery until CreateDevice unwinds.
    if crate::xbox::aot::oovpa::tap_active() {
        static TAP_DPC_DEFER_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = TAP_DPC_DEFER_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            debug_log(&format!(
                "[DPC-DEFER] worker VBlank/DPC skipped during CreateDevice TAP #{} esp=0x{:08X}",
                n, ctx.guest.esp
            ));
        }
        return;
    }

    // Only fire VBlank emulation from the main worker thread (worker #0).
    // Child threads (background loaders, audio) must not trigger VBlank — the
    // fire_guest_routine for callbacks enters the trampoline and can corrupt
    // child thread state.
    {
        use crate::xbox::emulator::WORKER_THREAD_ID;
        let main_tid = WORKER_THREAD_ID.load(std::sync::atomic::Ordering::Relaxed);
        let current_tid = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };
        if main_tid != 0 && current_tid != main_tid {
            return;
        }
    }

    // Pace VBlank at ~60Hz. Real Xbox fires VBlank once per frame (16.67ms).
    // NtYieldExecution spins call this on every iteration — without pacing we'd
    // fire millions of VBlanks per second.
    {
        use std::sync::Mutex;
        use std::time::{Duration, Instant};
        static LAST_VBLANK: std::sync::LazyLock<Mutex<Instant>> =
            std::sync::LazyLock::new(|| Mutex::new(Instant::now()));
        if let Ok(mut last) = LAST_VBLANK.lock() {
            let now = Instant::now();
            if now.duration_since(*last) < Duration::from_millis(16) {
                return; // too soon — skip this tick
            }
            *last = now;
        }
    }

    // Yield host thread so child threads (loading, audio) get CPU time.
    // On real Xbox, NtYieldExecution surrenders the timeslice; without this,
    // the main thread monopolizes the core and child threads starve.
    std::thread::yield_now();

    let isr_addr = veh_dpc::get_isr_address();
    let dpc_ctx_addr = veh_dpc::get_dpc_context(); // device/miniport struct
    let has_dpc_context = dpc_ctx_addr != 0 && dpc_ctx_addr < 0x1000_0000;

    if isr_addr != 0 && has_dpc_context {
        veh_dpc::increment_vblank();

        // Emulate VBlank ISR effects in Rust (can't run guest ISR — MMIO read loop hangs).
        use crate::xbox::aot::nv2a;

        // Increment VBlank counter at device+0x1F0
        let vblank_ctr = memory.read_u32(dpc_ctx_addr + 0x1F0);
        memory.write_u32(dpc_ctx_addr + 0x1F0, vblank_ctr.wrapping_add(1));

        // Copy current VBlank to device+0x1F8
        let f4_val = memory.read_u32(dpc_ctx_addr + 0x1F4);
        memory.write_u32(dpc_ctx_addr + 0x1F8, f4_val);

        // Sync NV2A USER GET = device[0] in both shadow regs AND guest memory.
        let dev_ptr = memory.read_u32(0x003038E0); // g_pDevice
        if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
            let dev0 = memory.read_u32(dev_ptr);
            nv2a::shadow_write(0x80_0044, dev0);
            nv2a::shadow_write(0x80_0040, dev0);
            unsafe {
                let base = ctx.guest_mem_base as u64;
                *((base + 0xFD800044) as *mut u32) = dev0;
                *((base + 0xFD800040) as *mut u32) = dev0;
                *((base + 0x44) as *mut u32) = dev0;
            }
        }

        // Set PFIFO idle flag so ISR's exit condition is met.
        memory.write_u32(
            dpc_ctx_addr + 0x3214,
            memory.read_u32(dpc_ctx_addr + 0x3214) | 0x10,
        );

        // Set NV2A shadow registers to simulate VBlank interrupt.
        nv2a::shadow_write(0x0100, 0x0100_0000); // PMC_INTR: PCRTC bit (24)
        nv2a::shadow_write(0x0140, 0x0100_0001); // PMC_INTR_EN: enable PCRTC
        nv2a::shadow_write(0x60_0100, 0x01); // PCRTC_INTR: VBlank pending

        // Signal VBlank event directly (device+0x1C8)
        let event_addr = dpc_ctx_addr + 0x1C8;
        if event_addr < 0x1000_0000 {
            memory.write_u32(event_addr + 0x04, 1);
        }

        // Re-signal timer if captured
        let timer = veh_dpc::get_dpc_timer();
        if timer != 0 && timer < 0x1000_0000 {
            memory.write_u32(timer + 4, 1);
        }

        static DPC_EMU_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DPC_EMU_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 5 || n % 100 == 0 {
            debug_log(&format!(
                "[VBLANK-EMU] #{} dev=0x{:08X} ctr=0x1F0: {}→{} f4=0x{:08X}",
                n,
                dpc_ctx_addr,
                vblank_ctr,
                vblank_ctr.wrapping_add(1),
                f4_val
            ));
        }
    }

    // Fire the queued or periodic guest DPC routine as guest code. DPCs may
    // legitimately use a NULL DeferredContext (Classic Doom does), so queued
    // one-shot delivery is intentionally not gated on the D3D miniport/device
    // context used by the VBlank side effects. Periodic delivery is enabled
    // only by timer HLE when a title arms a repeating frame timer.
    // DPC signature: void __stdcall DpcRoutine(PKDPC, DeferredContext, SysArg1, SysArg2)
    let dpc_pending = veh_dpc::is_dpc_pending();
    let dpc_timer_periodic = veh_dpc::is_timer_periodic();
    if dpc_pending || dpc_timer_periodic {
        let queued_dpc_object = if dpc_pending {
            take_pending_dpc_object(memory, "VBlankDPC")
        } else {
            None
        };
        let captured_dpc_object = veh_dpc::get_dpc_object();
        let dpc_object = queued_dpc_object.unwrap_or(captured_dpc_object);
        let mut dpc_routine = veh_dpc::get_dpc_routine();
        let mut dpc_call_context = veh_dpc::get_dpc_context();
        let mut stored_sys_arg1 = 0;
        let mut stored_sys_arg2 = 0;
        if let Some(obj) = queued_dpc_object.and_then(guest_data_addr) {
            let queued_routine = memory.read_u32(obj + 0x0C);
            if queued_routine != 0 {
                dpc_routine = queued_routine;
            }
            dpc_call_context = memory.read_u32(obj + 0x10);
            stored_sys_arg1 = memory.read_u32(obj + 0x14);
            stored_sys_arg2 = memory.read_u32(obj + 0x18);
        }
        if dpc_routine != 0 {
            if veh_dpc::suppress_guest_isr_dpc_injection() {
                static SUPPRESS_DPC_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SUPPRESS_DPC_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 10 || n.is_power_of_two() {
                    debug_log(&format!(
                        "[DPC-SUPPRESS] VBlankDPC guest call skipped #{} routine=0x{:08X} ctx=0x{:08X} obj=0x{:08X}",
                        n, dpc_routine, dpc_call_context, dpc_object
                    ));
                }
            } else {
                // 2026-04-26: removed should_defer_spiderman_vblank_dpc()
                // defender. It was suppressing every DPC fire when
                // FrameContext (0x004BC630), Upd1 (0x003F87A8), or Upd2
                // (0x003F7D90) were NULL — but those slots are populated BY
                // the DPC chain itself via FSM advance. Circular dependency
                // that became load-bearing once SYNTH-FRAMECTX was removed
                // (Codex final pass). KS-TELEM at a837d5f confirmed:
                // dpc_pend_set=100, dpc_a=0, dpc_s=0, WORKER-DPC=0 — defer
                // was skipping every fire. Letting the OS act like an OS:
                // queue pop = execute. If the DPC body faults on NULL
                // state, we get a clean signal about what to populate
                // organically.
                static DPC_FIRE_DIAG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = DPC_FIRE_DIAG.fetch_add(1, Ordering::Relaxed);
                if n < 5 || n.is_power_of_two() {
                    let dpc_timer = veh_dpc::get_dpc_timer();
                    let frame_ctx = memory.read_u32(0x004B_C630);
                    let upd1 = memory.read_u32(0x003F_87A8);
                    let upd2 = memory.read_u32(0x003F_7D90);
                    let scene = memory.read_u32(0x003F_5BEC);
                    let app = memory.read_u32(0x003F_5EB0);
                    let app_104 = if app != 0 && app < 0x2000_0000 {
                        memory.read_u32(app + 0x104)
                    } else {
                        0
                    };
                    let app_108 = if app != 0 && app < 0x2000_0000 {
                        memory.read_u32(app + 0x108)
                    } else {
                        0
                    };
                    let cminiport_base = memory.read_u32(0x0030_3068);
                    let cminiport_100 = memory.read_u32(0x0030_3068 + 0x100);
                    let cminiport_b0 = memory.read_u32(0x0030_3068 + 0xB0);
                    let cminiport_vblank_cb = if has_dpc_context {
                        memory.read_u32(dpc_ctx_addr + 0x1C4)
                    } else {
                        0
                    };
                    let dev_ptr = memory.read_u32(0x0030_38E0);
                    let (dev_swap_cb_off, dev_vblank_cb_off) =
                        crate::xbox::aot::oovpa::oovpa_hle::d3d_device_callback_offsets();
                    let dev_swap_cb = if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                        memory.read_u32(dev_ptr + dev_swap_cb_off)
                    } else {
                        0
                    };
                    let dev_vblank_cb = if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                        memory.read_u32(dev_ptr + dev_vblank_cb_off)
                    } else {
                        0
                    };
                    let dev_vblank_event = if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                        memory.read_u32(dev_ptr + 0x2430)
                    } else {
                        0
                    };
                    let cminiport_state = if cminiport_base == 0xFD00_0000 {
                        "nv2a"
                    } else {
                        "INVALID"
                    };
                    debug_log(&format!(
                        "[DPC-FIRE-DIAG] #{} routine=0x{:08X} ctx=0x{:08X} has_ctx={} pending={} periodic={} timer=0x{:08X} \
                         frame_ctx=0x{:08X} upd1=0x{:08X} upd2=0x{:08X} \
                         scene=0x{:08X} App=0x{:08X} App+0x104=0x{:08X} App+0x108=0x{:08X} \
                         CMiniport[0]=0x{:08X}({}) CMiniport+0x100=0x{:08X} CMiniport+0xB0=0x{:08X} CMiniport+0x1C4=0x{:08X} \
                         g_pDevice=0x{:08X} dev.swap_cb[+0x{:X}]=0x{:08X} dev.vblank_cb[+0x{:X}]=0x{:08X} dev.vblank_event=0x{:08X}",
                        n, dpc_routine, dpc_call_context, has_dpc_context, dpc_pending, dpc_timer_periodic, dpc_timer,
                        frame_ctx, upd1, upd2, scene, app, app_104, app_108,
                        cminiport_base, cminiport_state, cminiport_100, cminiport_b0, cminiport_vblank_cb,
                        dev_ptr, dev_swap_cb_off, dev_swap_cb, dev_vblank_cb_off, dev_vblank_cb, dev_vblank_event
                    ));
                }
                veh_dpc::set_worker_dpc_active(true);
                let sys_arg1 = if stored_sys_arg1 != 0 {
                    stored_sys_arg1
                } else {
                    veh_dpc::dpc_system_argument1(dpc_routine, dpc_call_context)
                };
                fire_guest_routine(
                    ctx,
                    memory,
                    vblank,
                    dpc_routine,
                    &[dpc_object, dpc_call_context, sys_arg1, stored_sys_arg2],
                    "VBlankDPC",
                );
                veh_dpc::set_worker_dpc_active(false);
            }
        }
    }

    // Fire D3D VBlank/Swap callbacks (registered via SetVerticalBlankCallback / SetSwapCallback)
    {
        use crate::xbox::aot::oovpa::oovpa_hle::{
            SWAP_CALLBACK, SWAP_COUNT_GLOBAL, VBLANK_CALLBACK, VBLANK_COUNT,
        };

        let vblank_count = VBLANK_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        let swap_count = SWAP_COUNT_GLOBAL.load(Ordering::Relaxed);

        let (dev_swap_cb_off, dev_vblank_cb_off) =
            crate::xbox::aot::oovpa::oovpa_hle::d3d_device_callback_offsets();
        let dev_ptr = memory.read_u32(0x0030_38E0);
        let dev_vblank_cb = if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
            memory.read_u32(dev_ptr + dev_vblank_cb_off)
        } else {
            0
        };
        let dev_swap_cb = if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
            memory.read_u32(dev_ptr + dev_swap_cb_off)
        } else {
            0
        };

        let registered_vblank_cb = VBLANK_CALLBACK.load(Ordering::Relaxed);
        let vblank_cb = if registered_vblank_cb != 0 {
            registered_vblank_cb
        } else {
            dev_vblank_cb
        };
        if vblank_cb != 0 {
            let vblank_data_addr = 0x83FF_FF00u32;
            memory.write_u32(vblank_data_addr + 0, vblank_count);
            memory.write_u32(vblank_data_addr + 4, swap_count);
            memory.write_u32(vblank_data_addr + 8, 1);
            if veh_dpc::suppress_guest_isr_dpc_injection() {
                static SUPPRESS_VBLANK_CB_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SUPPRESS_VBLANK_CB_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 5 || n.is_power_of_two() {
                    debug_log(&format!(
                        "[DPC-SUPPRESS] VBlankCB guest call skipped #{} cb=0x{:08X} data=0x{:08X}",
                        n, vblank_cb, vblank_data_addr
                    ));
                }
            } else {
                fire_guest_routine(
                    ctx,
                    memory,
                    vblank,
                    vblank_cb,
                    &[vblank_data_addr],
                    "VBlankCB",
                );
            }
        }

        let registered_swap_cb = SWAP_CALLBACK.load(Ordering::Relaxed);
        let swap_cb = if registered_swap_cb != 0 {
            registered_swap_cb
        } else {
            dev_swap_cb
        };
        if swap_cb != 0 {
            let swap_data_addr = 0x83FF_FF20u32;
            memory.write_u32(swap_data_addr + 0x00, swap_count);
            memory.write_u32(swap_data_addr + 0x04, vblank_count);
            memory.write_u32(swap_data_addr + 0x08, 0);
            memory.write_u32(swap_data_addr + 0x0C, 0);
            memory.write_u32(swap_data_addr + 0x10, 12_200_000);
            if veh_dpc::suppress_guest_isr_dpc_injection() {
                static SUPPRESS_SWAP_CB_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SUPPRESS_SWAP_CB_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 5 || n.is_power_of_two() {
                    debug_log(&format!(
                        "[DPC-SUPPRESS] SwapCB guest call skipped #{} cb=0x{:08X} data=0x{:08X}",
                        n, swap_cb, swap_data_addr
                    ));
                }
            } else {
                fire_guest_routine(ctx, memory, vblank, swap_cb, &[swap_data_addr], "SwapCB");
            }
        }
    }
}

/// Runs the AOT dispatch loop for a worker thread.
/// The worker owns its RuntimeContext and a non-owning clone of GuestMemory.
/// Runs independently until ThreadExit or guest_addr reaches 0.
pub(crate) fn worker_dispatch_loop(
    mut ctx: Box<RuntimeContext>,
    memory: GuestMemory,
    entry: u32,
    start_routine: u32,
    start_context: u32,
    live_counters: Arc<SharedWorkerCounters>,
    vblank: Arc<VBlankSignal>,
) -> WorkerStats {
    // Worker guest stack: each thread gets its own 1MB region.
    // Thread 0 (main worker): 0x1EF00000-0x1F000000
    // Thread 1: 0x1EE00000-0x1EF00000, etc.
    // Classic Doom's CRT init does 196K nested malloc calls that need ~11MB of guest stack,
    // so the main worker gets the full range 0x1E000000-0x1F000000.
    static THREAD_STACK_INDEX: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let stack_idx = THREAD_STACK_INDEX.fetch_add(1, Ordering::Relaxed);

    // Store native thread ID for VBlank thread gating (only main worker #0).
    // Child threads must NOT overwrite this — it gates ISR/DPC to the main thread.
    {
        let tid = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };
        if stack_idx == 0 {
            WORKER_THREAD_ID.store(tid, std::sync::atomic::Ordering::Relaxed);
        }
        debug_log(&format!(
            "AOT worker: native thread ID = {} (stack_idx={})",
            tid, stack_idx
        ));
        if !ctx.thunk_table.is_empty() {
            debug_log(&format!(
                "[THUNK] Worker start: {} entries, range 0x{:08X}-0x{:08X}",
                ctx.thunk_table.len(),
                ctx.thunk_table.start_addr(),
                ctx.thunk_table.end_addr()
            ));
        }
    }
    let worker_stack_top: u32 = if stack_idx == 0 {
        0x1F00_0000 // main worker: full 16MB range
    } else {
        // Additional threads: 256KB each, starting at 0x1D000000 going down
        // 0x1D000000, 0x1CFC0000, 0x1CF80000, ...
        0x1D00_0000u32.wrapping_sub((stack_idx - 1) * 0x0004_0000)
    };
    debug_log(&format!(
        "Worker stack: idx={} top=0x{:08X} entry=0x{:08X} start=0x{:08X}",
        stack_idx, worker_stack_top, entry, start_routine
    ));

    let via_system_routine = entry != start_routine && entry != 0;
    if via_system_routine {
        // XapiThreadStartup: stack = [ret=0, StartRoutine, StartContext]
        ctx.guest.esp = worker_stack_top - 12;
        memory.write_u32(ctx.guest.esp, 0); // return addr → 0 (RET_TO_ZERO)
        memory.write_u32(ctx.guest.esp + 4, start_routine); // arg0: StartRoutine
        memory.write_u32(ctx.guest.esp + 8, start_context); // arg1: StartContext
    } else {
        // Direct entry: stack = [ret=0, start_context]
        ctx.guest.esp = worker_stack_top - 8;
        memory.write_u32(ctx.guest.esp, 0); // return addr → 0 (RET_TO_ZERO)
        memory.write_u32(ctx.guest.esp + 4, start_context); // arg0: StartContext
    }
    ctx.guest.eax = 0;
    ctx.guest.ecx = 0;
    ctx.guest.edx = 0;
    ctx.guest.ebx = 0;
    ctx.guest.ebp = 0;
    ctx.guest.esi = 0;
    ctx.guest.edi = 0;
    ctx.reset_shadow_stack();

    // Worker shares main thread's KernelState (matching C++ global pattern).
    // kernel_state pointer was set by caller before spawning.
    debug_log(&format!(
        "AOT worker: using shared kernel_state at {:p}",
        ctx.kernel_state
    ));

    // Set per-thread VEH context so exception handler finds this ctx
    crate::xbox::aot::veh::set_thread_context(&mut *ctx as *mut RuntimeContext);

    let mut guest_addr = entry;

    debug_log(&format!(
        "AOT worker: shadow_stack top=0x{:016X} base={:p} size={} host_stack_ptr={:p} host_stack_len={}",
        ctx.guest.shadow_stack_top,
        ctx.shadow_stack.usable_base, ctx.shadow_stack.usable_size,
        ctx.host_stack.as_ptr(), ctx.host_stack.len()
    ));
    debug_log(&format!(
        "AOT worker: starting at 0x{:08X} (start=0x{:08X} ctx=0x{:08X}) esp=0x{:08X}",
        entry, start_routine, start_context, ctx.guest.esp
    ));

    // Verify NV2A MMIO page protection is intact
    {
        let nv2a_ptr = (ctx.guest_mem_base as u64 + 0xFD000000u64) as *const u8;
        let mut mbi = unsafe {
            std::mem::zeroed::<windows::Win32::System::Memory::MEMORY_BASIC_INFORMATION>()
        };
        let ret = unsafe {
            windows::Win32::System::Memory::VirtualQuery(
                Some(nv2a_ptr as *const _),
                &mut mbi,
                std::mem::size_of::<windows::Win32::System::Memory::MEMORY_BASIC_INFORMATION>(),
            )
        };
        debug_log(&format!(
            "[MMIO-CHECK] NV2A page at {:p}: Protect=0x{:X} State=0x{:X} RegionSize=0x{:X} ret={}",
            nv2a_ptr, mbi.Protect.0, mbi.State.0, mbi.RegionSize, ret
        ));
    }

    // Try the first dispatch via trampoline. If it overflows (exit_guest_addr=0xDEAD0001),
    // fall back to micro-interpreter for the entire startup chain.
    static INTERPRETER_FALLBACK: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    let max_dispatches = 500_000u64;
    let mut kernel_calls = 0u64;
    let mut last_error_addr: u32 = 0;
    let mut error_repeat_count: u32 = 0;
    let mut ret_to_zero_count: u32 = 0;
    let mut phase = 0u32; // 0=initial code, 1=DPC re-entry, 2+=ISR injection loop
    let mut deferred_child_spawns: Vec<(u32, u32, u32, u32, &'static str)> = Vec::new();

    // STARTUP CONTINUITY: let the worker's natural StartRoutine chain run.
    // pool_init → static_ctors_1 → static_ctors_2 → game_main
    // Set to false to re-enable forced phase injection as fallback.
    const ORGANIC_MODE: bool = true;

    // Rolling buffer of last 20 kernel calls for diagnostics
    let mut call_history: Vec<(u64, u32, [u32; 4], u32)> = Vec::with_capacity(20); // (idx, ordinal, args[0..4], eax)

    // RPCS3-style hybrid: interpret the startup chain for games that need it.
    // Spider-Man uses the proven AOT-only path — CreateDevice fires during static_ctors_1
    // via AOT forced phases (requires CxbxDB OOVPA hooks for D3D call chain intercept).
    // Classic Doom and other games use interpreter for startup SEH + TLS setup.
    let use_interpreter = start_routine != 0x002A_9BC9; // Spider-Man = AOT-only
    if use_interpreter && ctx.dispatch_count == 0 && guest_addr != 0 {
        use crate::xbox::aot::micro_interp::{self, InterpResult, X86Regs};
        debug_log(&format!(
            "AOT worker: Interpreter startup at 0x{:08X} ESP=0x{:08X}",
            guest_addr, ctx.guest.esp
        ));
        let mut iregs = X86Regs {
            eax: ctx.guest.eax,
            ecx: ctx.guest.ecx,
            edx: ctx.guest.edx,
            ebx: ctx.guest.ebx,
            esp: ctx.guest.esp,
            ebp: ctx.guest.ebp,
            esi: ctx.guest.esi,
            edi: ctx.guest.edi,
            eip: guest_addr,
            eflags: 0x202,
            fpu_stack: [0.0; 8],
            fpu_top: 0,
            xmm: [[0u8; 16]; 8],
        };
        // OOVPA hooks enabled — CreateDevice MUST fire as HLE hook.
        // The interpreter can't execute CreateDevice natively (NV2A register writes,
        // fild, hardware polling, indirect calls through device pointers).
        let create_device_addr = crate::xbox::aot::oovpa::get_create_device_addr();
        let mut oovpa_addrs = crate::xbox::aot::oovpa::get_hooked_guest_addrs();
        let seh_addrs = crate::xbox::aot::oovpa::get_seh_addrs();
        oovpa_addrs.retain(|&addr| {
            if addr >= 0x00040000 && addr < 0x00048000 {
                return false;
            } // CRT range
            if seh_addrs.contains(&addr) {
                return false;
            } // _SEH_prolog/_SEH_epilog: interpreter runs natively
            true
        });
        // Classic Doom (XDK 5849): add manual HLE hooks for SDK/OS functions.
        // Game functions (I_GetTime, I_FinishUpdate, etc.) run natively — "HLE the OS, not the game".
        if start_routine == 0x0003_E537 {
            // S_Init (0x11AF0) removed — game code, must run natively to set ticdup/sound state.
            // DirectSoundCreate (0x84325): DSOUND section spins in KeStallExecutionProcessor
            // polling APU hardware. Hook to return DS_OK (0) without touching APU.
            oovpa_addrs.push(0x0008_4325);
            // CRT _nh_malloc (0x45325): CRT heap handle at [0x106E34] is uninitialized (0),
            // causing heap corruption. Bump allocator HLE.
            oovpa_addrs.push(0x0004_5325);
            // Also hook malloc directly in case it's called from non-CRT code
            oovpa_addrs.push(0x0004_5652);
            // W_CheckNumForName (0x236F0): WAD lump name→index lookup (O(n) → O(1) HLE).
            oovpa_addrs.push(0x0002_36F0);
            debug_log("[INTERP] Added Doom manual HLE: DirectSoundCreate, _nh_malloc, malloc, W_CheckNumForName");
        }
        // Add start_routine as an OOVPA hook target so interpreter yields at
        // the function boundary. Worker handles this by switching to AOT.
        // Doom (0x0003_E537): interpreter runs StartRoutine natively — don't hook it.
        if start_routine != 0
            && start_routine != 0x0003_E537
            && !oovpa_addrs.contains(&start_routine)
        {
            oovpa_addrs.push(start_routine);
        }
        debug_log(&format!(
            "[INTERP] {} OOVPA hooks (excluded CRT+{} SEH, CreateDevice={:?} INCLUDED, start_routine=0x{:08X})",
            oovpa_addrs.len(), seh_addrs.len(), create_device_addr, start_routine
        ));
        let mut interp_kernel_calls = 0u64;
        let mut hle_boot_done = false;
        // Diagnostic: track last OOVPA hook that fired (addr, ret_addr) for ReturnedTo(0) logging
        let mut last_oovpa_hook: Option<(u32, u32)> = None;
        // DPC sentinel: used as expected_ret for nested interpret() call during DPC delivery.
        const DPC_SENTINEL: u32 = 0xDDDD_0001;
        // Track last-known-good SEH state including scope table data.
        // rep movsd corrupts KPCR AND scope table, so we must save everything.
        struct SehSnapshot {
            head: u32,
            prev: u32,
            handler: u32,
            scope_addr: u32,
            frame_ebp: u32,
            trylevel: u32,
            // Saved scope table entries: (enclosing, filter, except_handler)
            scope_entries: Vec<(u32, u32, u32)>,
        }
        let mut last_good_seh: Option<SehSnapshot> = None;

        // Interpreter loop: run → kernel call → dispatch → resume → repeat.
        // Stop early if EIP enters AOT-compiled code (addr_hash hit) — hand off to trampoline.
        // This lets the interpreter handle XapiThreadStartup's SEH + indirect call,
        // then AOT takes over for the bulk of CRT init (fast compiled code).
        loop {
            let entry_eip = iregs.eip;
            // AOT handoff: when the interpreter reaches StartRoutine, hand off to AOT.
            // The interpreter handles XapiThreadStartup's SEH setup + indirect call [ebp+8].
            // Once we enter StartRoutine, AOT-compiled code handles the entire CRT boot
            // (pool_init, static_ctors, game_main) — no more manual opcode implementations.
            // AOT handoff removed — now handled via OOVPA hook on start_routine.
            // When interpreter CALLs start_routine, it returns OovpaHook which
            // the outer handler converts to an AOT entry.

            // DIAGNOSTIC: Log key CRT addresses to trace where execution diverges
            // before reaching _initterm (0x40E9C / 0x40E44)
            {
                let crt_milestones: &[(u32, &str)] = &[
                    (0x00040E9C, "_initterm (CRT constructors)"),
                    (0x00040E44, "_initterm (CRT initializers)"),
                    (0x00040CD5, "XapiInitProcess"),
                    (0x00040508, "partition enumerator"),
                    (0x000431D4, "SEH-protected D3D pre-init"),
                    (0x00040F29, "HW detect (HLE'd)"),
                    (0x0003E537, "StartRoutine entry"),
                ];
                for &(addr, name) in crt_milestones {
                    if entry_eip == addr {
                        debug_log(&format!(
                            "[CRT-TRACE] Reached {} at 0x{:08X} ESP=0x{:08X} EAX=0x{:08X} kcalls={}",
                            name, addr, iregs.esp, iregs.eax, interp_kernel_calls
                        ));
                    }
                }
                // Also log whenever EIP is in the CRT range (0x40000-0x50000)
                // but only for function entries (first time at this address)
                if entry_eip >= 0x00040000 && entry_eip < 0x00050000 {
                    static CRT_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let cl = CRT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if cl < 100 {
                        debug_log(&format!(
                            "[CRT-EXEC] 0x{:08X} ESP=0x{:08X} EBP=0x{:08X} EAX=0x{:08X} kcalls={}",
                            entry_eip, iregs.esp, iregs.ebp, iregs.eax, interp_kernel_calls
                        ));
                    }
                }
            }

            // Track when interpreter is in StartRoutine area (Spider-Man)
            if entry_eip >= 0x002A9BC0 && entry_eip < 0x002A9CE0 {
                static SR_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let sl = SR_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if sl < 50 {
                    debug_log(&format!(
                        "[START-ROUTINE] EIP=0x{:08X} ESP=0x{:08X} EAX=0x{:08X} kcalls={}",
                        entry_eip, iregs.esp, iregs.eax, interp_kernel_calls
                    ));
                }
            }
            // Track when interpreter enters game_main area
            if entry_eip == 0x002A52A0 || entry_eip == 0x002AE296 || entry_eip == 0x002AE23E {
                debug_log(&format!(
                    "[INTERP-MILESTONE] Reached 0x{:08X} ESP=0x{:08X} EAX=0x{:08X} kcalls={}",
                    entry_eip, iregs.esp, iregs.eax, interp_kernel_calls
                ));
            }
            // Track when interpreter enters Doom engine code
            if entry_eip >= 0x11000 && entry_eip < 0x40000 {
                static DOOM_ENTRY: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let de = DOOM_ENTRY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if de < 50 {
                    debug_log(&format!(
                        "[INTERP-DOOM] Entered Doom engine at 0x{:08X} ESP=0x{:08X} kcalls={}",
                        entry_eip, iregs.esp, interp_kernel_calls
                    ));
                }
            }
            // Key D_DoomMain milestones (cap at 50 logs)
            if entry_eip == 0x126B3
                || entry_eip == 0x126B8
                || entry_eip == 0x12706
                || entry_eip == 0x12778
                || entry_eip == 0x12781
                || entry_eip == 0x126AD
            {
                static MILE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let ml = MILE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if ml < 50 {
                    let np = memory.read_u32(0x0010_1B58);
                    let d9 = memory.read_u32(0x000D_9C68);
                    debug_log(&format!(
                        "[DOOM-MILESTONE] 0x{:08X} ESP=0x{:08X} EAX=0x{:08X} kcalls={} players=[0x101B58]={} [0xD9C68]={}",
                        entry_eip, iregs.esp, iregs.eax, interp_kernel_calls, np, d9
                    ));
                }
            }
            // Snapshot SEH chain + scope table data before interpreting.
            // rep movsd can corrupt ALL guest memory including KPCR and scope tables.
            {
                const FS_Z: u32 = 0x0C00_0000;
                let head = memory.read_u32(FS_Z);
                if head != 0xFFFF_FFFF && head != 0 && head < 0x2000_0000 {
                    let prev = memory.read_u32(head);
                    let handler = memory.read_u32(head + 4);
                    let scope_addr = memory.read_u32(head + 8);
                    let ebp = head.wrapping_add(0x10);
                    let trylevel = memory.read_u32(ebp.wrapping_sub(4));
                    // Save scope table entries (up to trylevel+8 entries to cover enclosing chain)
                    let mut scope_entries = Vec::new();
                    if scope_addr >= 0x10000 && scope_addr < 0x0080_0000 && trylevel <= 100 {
                        let max_entries = (trylevel + 8).min(32);
                        for i in 0..max_entries {
                            let entry_addr = scope_addr + i * 12;
                            if entry_addr + 12 > 0x0080_0000 {
                                break;
                            }
                            let enclosing = memory.read_u32(entry_addr);
                            let filter = memory.read_u32(entry_addr + 4);
                            let except_handler = memory.read_u32(entry_addr + 8);
                            scope_entries.push((enclosing, filter, except_handler));
                        }
                    }
                    last_good_seh = Some(SehSnapshot {
                        head,
                        prev,
                        handler,
                        scope_addr,
                        frame_ebp: ebp,
                        trylevel,
                        scope_entries,
                    });
                }
            }
            let result = micro_interp::interpret(
                &memory,
                &mut iregs,
                entry_eip,
                0,
                5_000_000_000,
                &oovpa_addrs,
                &ctx.exec_ranges,
            );
            // Repair any thunk entries corrupted during interpretation
            {
                let repaired = ctx.thunk_table.repair_all(&memory);
                if repaired > 0 {
                    crate::rate_log!(
                        10,
                        "[THUNK] Repaired {} entries after interpret(0x{:08X})",
                        repaired,
                        entry_eip
                    );
                }
            }
            // Log EVERY result for debugging
            static RESULT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let rn = RESULT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if rn < 200 {
                let desc = match &result {
                    InterpResult::KernelCall { ordinal, ret_addr } => {
                        format!("KernelCall(ord={},ret=0x{:08X})", ordinal, ret_addr)
                    }
                    InterpResult::ReturnedTo(a) => format!("ReturnedTo(0x{:08X})", a),
                    InterpResult::Timeout => "Timeout".to_string(),
                    InterpResult::Unhandled { eip, mnemonic } => {
                        format!("Unhandled(0x{:08X},{})", eip, mnemonic)
                    }
                    InterpResult::OovpaHook { guest_addr, .. } => {
                        format!("OovpaHook(0x{:08X})", guest_addr)
                    }
                    InterpResult::AccessViolation { eip, addr } => {
                        format!("AV(eip=0x{:08X},addr=0x{:08X})", eip, addr)
                    }
                };
                debug_log(&format!(
                    "[INTERP-RESULT] #{} EIP=0x{:08X} {}",
                    rn, iregs.eip, desc
                ));
                // Write to separate fast log
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("./interp.log")
                {
                    use std::io::Write;
                    let _ = writeln!(
                        f,
                        "[INTERP-RESULT] #{} EIP=0x{:08X} {}",
                        rn, iregs.eip, desc
                    );
                }
            }
            match result {
                InterpResult::KernelCall { ordinal, ret_addr } => {
                    interp_kernel_calls += 1;
                    // Publish interpreter progress to shared counters (HUD reads these)
                    live_counters
                        .kernel_calls
                        .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                    live_counters.dispatch_count.store(
                        interp_kernel_calls * 100,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    live_counters
                        .alive
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                    let mut kargs = [0u32; 12];
                    let kargc = crate::xbox::kernel::ordinals::arg_count(ordinal);
                    let esp_before = iregs.esp;
                    for i in 0..kargc.min(12) {
                        kargs[i as usize] = memory.read_u32(esp_before + 4 + i * 4);
                    }
                    log_spidey_file_kcall(
                        &memory,
                        "interp",
                        interp_kernel_calls,
                        ordinal,
                        iregs.eip,
                        ret_addr,
                        esp_before,
                        iregs.ebp,
                        &kargs,
                    );
                    // Dispatch kernel call using the same API as the main dispatch loop
                    let kresult = crate::xbox::kernel::dispatch_kernel_call(
                        ordinal,
                        &memory,
                        &mut iregs.esp,
                        &mut iregs.eax,
                        &mut iregs.edx,
                        &mut iregs.ecx,
                        unsafe {
                            &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState)
                        },
                    );
                    // dispatch_kernel_call does stdcall cleanup: ESP += argc*4 (pops args only).
                    // We also need to pop the return address: ESP += 4.
                    iregs.esp = iregs.esp.wrapping_add(4); // pop return address
                    iregs.eip = ret_addr;

                    // Periodic Doom game state dump (every 1000 kernel calls)
                    if start_routine == 0x0003_E537
                        && (interp_kernel_calls == 50
                            || interp_kernel_calls == 100
                            || interp_kernel_calls == 200
                            || interp_kernel_calls % 1000 == 0)
                    {
                        // Xbox Doom loop state (from disassembly of D_DoomLoop at 0x12706):
                        //   [0x101B58] = game class (this ptr for TryRunTics)
                        //   [0x101B70] = base I_GetTime value
                        //   [0x101B74] = computed tic count
                        //   [0x101B78] = tic accumulator
                        //   [0x101BB8] = game_state pointer (used inside TryRunTics)
                        //   [0xE0568]  = render flag (set by TryRunTics)
                        let base_time = memory.read_u32(0x0010_1B70);
                        let tic_count = memory.read_u32(0x0010_1B74);
                        let tic_accum = memory.read_u32(0x0010_1B78);
                        let gs = memory.read_u32(0x0010_1BB8);
                        let render_flag = memory.read_u8(0x000E_0568);
                        let game_class = memory.read_u32(0x0010_1B58);
                        let num_players = if game_class != 0 && game_class < 0x1000_0000 {
                            memory.read_u32(game_class)
                        } else {
                            0
                        };
                        // Cache game state pointer when it's valid (heap range 0x01100000+)
                        // [0x101BB8] gets overwritten by TryRunTics later, so save it now
                        if gs >= 0x0100_0000 && gs < 0x1000_0000 {
                            DOOM_GAME_STATE_CACHE.store(gs, std::sync::atomic::Ordering::Relaxed);
                        }
                        debug_log(&format!(
                            "[DOOM-STATE] kcall={} base_time={} tic_count={} tic_accum={} gs=0x{:08X} render={} players={} ord={} EIP=0x{:08X} ESP=0x{:08X}",
                            interp_kernel_calls, base_time, tic_count, tic_accum, gs, render_flag, num_players, ordinal, iregs.eip, iregs.esp
                        ));
                    }

                    // OS-layer timer DPC delivery: after KeDelayExecutionThread (ord 99)
                    // or KeWaitForSingleObject (ord 159), fire the captured DPC routine.
                    // On real Xbox, the kernel fires timer DPCs after sleep/wait completes.
                    // Without this, games in sleep-poll-tic loops never get tic increments.
                    //
                    // Run the DPC as a nested interpret() call: push DPC args + sentinel
                    // return address, call interpret() with expected_ret = sentinel, then
                    // restore ESP. The DPC runs to completion inline — no ESP drift.
                    if ordinal == 99 || ordinal == 159 {
                        let (_, dpc_routine) = crate::xbox::aot::veh_dpc::get_captured_routines();
                        if dpc_routine != 0 {
                            let dpc_ctx = crate::xbox::aot::veh_dpc::get_dpc_context();
                            let dpc_obj = crate::xbox::aot::veh_dpc::get_dpc_object();
                            let saved_esp = iregs.esp;
                            // Set timer SignalState = 1 so DPC routine's check passes.
                            // On real Xbox, the kernel sets this when the timer expires.
                            // The DPC routine reads a flag near the timer/DPC objects.
                            // Timer at 0x106EE0, DPC object at 0x106EC0.
                            // The flag at timer+0x28 (0x106F08) is the "timer expired" signal.
                            let timer_ptr = crate::xbox::aot::veh_dpc::get_dpc_timer();
                            if timer_ptr != 0 {
                                memory.write_u32(timer_ptr + 4, 1); // SignalState (KTIMER+4)
                                memory.write_u32(timer_ptr + 0x28, 1); // Game's custom flag past KTIMER
                            }
                            // Push DPC call frame: sentinel return address + 4 args
                            iregs.esp = iregs.esp.wrapping_sub(20);
                            memory.write_u32(iregs.esp, DPC_SENTINEL); // sentinel return addr
                            memory.write_u32(iregs.esp + 4, dpc_obj); // arg0: KDPC*
                            memory.write_u32(iregs.esp + 8, dpc_ctx); // arg1: DeferredContext
                            let sys_arg1 = crate::xbox::aot::veh_dpc::dpc_system_argument1(
                                dpc_routine,
                                dpc_ctx,
                            );
                            memory.write_u32(iregs.esp + 12, sys_arg1); // arg2: SystemArgument1
                            memory.write_u32(iregs.esp + 16, 0); // arg3: SystemArgument2
                            static DPC_FIRE_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let n = DPC_FIRE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n < 10 {
                                // Dump DPC routine bytes for disassembly verification
                                let mut bytes = [0u8; 16];
                                for i in 0..16 {
                                    bytes[i] = memory.read_u8(dpc_routine + i as u32);
                                }
                                debug_log(&format!(
                                    "[INTERP-DPC] Firing DPC 0x{:08X}(obj=0x{:08X}, ctx=0x{:08X}) after ord {} #{} ESP=0x{:08X} bytes={:02X?}",
                                    dpc_routine, dpc_obj, dpc_ctx, ordinal, interp_kernel_calls, iregs.esp, bytes
                                ));
                                // Dump thunk values referenced by DPC
                                let thunk1 = memory.read_u32(0x000B52D0);
                                let thunk2 = memory.read_u32(0x000B5320);
                                let flag = memory.read_u32(0x00106F08);
                                debug_log(&format!("[INTERP-DPC] thunk@0xB52D0=0x{:08X} thunk@0xB5320=0x{:08X} [0x106F08]={}", thunk1, thunk2, flag));
                            }
                            // Run DPC as nested interpreter call — handles kernel thunks internally
                            let dpc_result = micro_interp::interpret(
                                &memory,
                                &mut iregs,
                                dpc_routine,
                                DPC_SENTINEL,
                                100_000,
                                &oovpa_addrs,
                                &ctx.exec_ranges,
                            );
                            if n < 10 {
                                debug_log(&format!(
                                    "[INTERP-DPC] phase 1 result: {:?}",
                                    dpc_result
                                ));
                            }
                            // Handle kernel calls made by the DPC routine — may need
                            // multiple rounds if the DPC calls multiple kernel thunks
                            let mut dpc_res = dpc_result;
                            for _round in 0..10 {
                                match dpc_res {
                                    micro_interp::InterpResult::KernelCall {
                                        ordinal: dpc_ord,
                                        ret_addr: dpc_ret,
                                    } => {
                                        let dpc_ord_name =
                                            crate::xbox::kernel::ordinals::name(dpc_ord);
                                        if n < 10 {
                                            debug_log(&format!(
                                                "[INTERP-DPC] kernel call: #{} {} ret=0x{:08X}",
                                                dpc_ord, dpc_ord_name, dpc_ret
                                            ));
                                        }
                                        let _dpc_kr = crate::xbox::kernel::dispatch_kernel_call(
                                            dpc_ord,
                                            &memory,
                                            &mut iregs.esp,
                                            &mut iregs.eax,
                                            &mut iregs.edx,
                                            &mut iregs.ecx,
                                            unsafe {
                                                &mut *(ctx.kernel_state
                                                    as *mut crate::xbox::kernel::KernelState)
                                            },
                                        );
                                        iregs.esp = iregs.esp.wrapping_add(4); // pop ret addr
                                                                               // Continue DPC after kernel call
                                        dpc_res = micro_interp::interpret(
                                            &memory,
                                            &mut iregs,
                                            dpc_ret,
                                            DPC_SENTINEL,
                                            100_000,
                                            &oovpa_addrs,
                                            &ctx.exec_ranges,
                                        );
                                        if n < 10 {
                                            debug_log(&format!(
                                                "[INTERP-DPC] phase {} result: {:?}",
                                                _round + 2,
                                                dpc_res
                                            ));
                                        }
                                    }
                                    micro_interp::InterpResult::ReturnedTo(addr)
                                        if addr == DPC_SENTINEL =>
                                    {
                                        break; // DPC completed successfully
                                    }
                                    ref other => {
                                        if n < 10 {
                                            debug_log(&format!(
                                                "[INTERP-DPC] unexpected: {:?}",
                                                other
                                            ));
                                        }
                                        break;
                                    }
                                }
                            }
                            // Restore ESP — DPC is a side-effect, shouldn't affect caller's stack
                            iregs.esp = saved_esp;
                            iregs.eip = ret_addr; // resume where KeDelay/KeWait would return
                            if n < 10 {
                                debug_log(&format!(
                                    "[INTERP-DPC] DPC done, ESP restored to 0x{:08X}, resume at 0x{:08X}",
                                    saved_esp, ret_addr
                                ));
                            }
                        }
                        // Periodic game state dump for Doom tic loop diagnosis
                        if start_routine == 0x0003_E537 {
                            static DOOM_DIAG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let dd = DOOM_DIAG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if dd < 5 || (dd % 500 == 0) {
                                let gs = memory.read_u32(0x0010_1BB8);
                                if gs != 0 {
                                    let singletics = memory.read_u32(gs.wrapping_add(0x22A0));
                                    let gameaction = memory.read_u32(gs.wrapping_add(0x2284));
                                    let demoplay = memory.read_u32(gs.wrapping_add(0x2288));
                                    let advancedemo = memory.read_u32(gs.wrapping_add(0x2750));
                                    let consoleplayer = memory.read_u32(gs.wrapping_add(0x2714));
                                    let maketic = memory.read_u32(gs.wrapping_add(0x1F44));
                                    let gametic = memory.read_u32(gs.wrapping_add(0x271C));
                                    let i_gettime_cached = memory.read_u32(gs.wrapping_add(0x3968));
                                    let tick_ptr = memory.read_u32(0x000B_5248);
                                    let ke_tick = if tick_ptr != 0 && tick_ptr < 0x2000_0000 {
                                        memory.read_u32(tick_ptr)
                                    } else {
                                        0
                                    };
                                    debug_log(&format!(
                                        "[DOOM-STATE] #{} gs=0x{:08X} singletics={} gameaction={} demoplay={} advancedemo={} consoleplayer={} maketic={} gametic={} i_gettime_cache={} KeTickPtr=0x{:08X} KeTickVal={} EIP=0x{:08X} ESP=0x{:08X}",
                                        dd, gs, singletics, gameaction, demoplay, advancedemo, consoleplayer,
                                        maketic, gametic, i_gettime_cached, tick_ptr, ke_tick, iregs.eip, iregs.esp
                                    ));
                                }
                            }
                        }
                    }
                    match kresult {
                        crate::xbox::kernel::KernelResult::SpawnThread {
                            entry,
                            start_routine,
                            context,
                            thread_handle,
                        } => {
                            spawn_child_thread_from_worker(
                                &ctx,
                                &memory,
                                &vblank,
                                entry,
                                start_routine,
                                context,
                                thread_handle,
                                "interp-dispatch",
                            );
                        }
                        crate::xbox::kernel::KernelResult::Halt => {
                            debug_log("AOT worker: Interpreter hit Halt — exiting");
                            break;
                        }
                        crate::xbox::kernel::KernelResult::QuickReboot => {
                            // Cxbx-R style: re-enter XBE from entry point with persistent
                            // memory intact. The game wrote launch data before calling
                            // HalReturnToFirmware(2). On re-entry, the CRT sees the
                            // populated LaunchDataPage and takes the normal boot path
                            // (reaches _initterm, initializes file I/O, loads game).
                            debug_log(&format!(
                                "[QUICK-REBOOT] Re-entering from XapiThreadStartup (0x{:08X}) → StartRoutine (0x{:08X}) after {} kernel calls",
                                entry, start_routine, interp_kernel_calls
                            ));
                            // Reset interpreter state — fresh stack, re-enter via XapiThreadStartup
                            iregs.esp = worker_stack_top - 12;
                            memory.write_u32(iregs.esp, 0); // return addr = 0 (RET_TO_ZERO sentinel)
                            memory.write_u32(iregs.esp + 4, start_routine); // arg0: StartRoutine
                            memory.write_u32(iregs.esp + 8, start_context); // arg1: StartContext
                            iregs.eip = entry; // XapiThreadStartup
                            iregs.eax = 0;
                            iregs.ecx = 0;
                            iregs.edx = 0;
                            iregs.ebx = 0;
                            iregs.ebp = 0;
                            iregs.esi = 0;
                            iregs.edi = 0;
                            iregs.eflags = 0x202;
                            // Don't reset interp_kernel_calls — keep counting
                            continue;
                        }
                        _ => {}
                    }
                    if interp_kernel_calls <= 50
                        || interp_kernel_calls >= 2040
                        || interp_kernel_calls % 100000 == 0
                    {
                        let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                        // Log args for MmAllocContiguous to debug EAX=0 issue
                        if ordinal == 166 {
                            debug_log(&format!(
                                "[INTERP-KERN] #{} {} (ord={}) ret=0x{:08X} eax=0x{:08X} ESP=0x{:08X} args=[{:08X},{:08X},{:08X},{:08X},{:08X}]",
                                interp_kernel_calls, ord_name, ordinal, ret_addr, iregs.eax, iregs.esp,
                                memory.read_u32(iregs.esp), memory.read_u32(iregs.esp+4),
                                memory.read_u32(iregs.esp+8), memory.read_u32(iregs.esp+12),
                                memory.read_u32(iregs.esp+16)
                            ));
                        } else {
                            debug_log(&format!(
                                "[INTERP-KERN] #{} {} (ord={}) ret=0x{:08X} eax=0x{:08X} ESP=0x{:08X}",
                                interp_kernel_calls, ord_name, ordinal, ret_addr, iregs.eax, iregs.esp
                            ));
                        }
                    }
                }
                InterpResult::ReturnedTo(0) => {
                    interp_kernel_calls += 1;
                    {
                        static RET0_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let rl = RET0_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if rl < 30 {
                            let last_hook_str = match last_oovpa_hook {
                                Some((addr, ret)) => {
                                    format!("last_hook=0x{:08X}(ret=0x{:08X})", addr, ret)
                                }
                                None => "last_hook=none".to_string(),
                            };
                            debug_log(&format!(
                                "[RET-TO-NULL] Interpreter returned to 0 after {} kcalls ESP=0x{:08X} last_EIP=0x{:08X} {} — probable broken hook return",
                                interp_kernel_calls, iregs.esp, entry_eip, last_hook_str
                            ));
                        }
                    }
                    live_counters
                        .kernel_calls
                        .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                    live_counters.dispatch_count.store(
                        interp_kernel_calls * 100,
                        std::sync::atomic::Ordering::Relaxed,
                    );
                    // Game-specific re-entry targets for the CRT boot interpreter:
                    static DOOM_MAIN_ENTERED: std::sync::atomic::AtomicBool =
                        std::sync::atomic::AtomicBool::new(false);
                    if start_routine == 0x0003_E537
                        && !DOOM_MAIN_ENTERED.load(std::sync::atomic::Ordering::Relaxed)
                    {
                        DOOM_MAIN_ENTERED.store(true, std::sync::atomic::Ordering::Relaxed);
                        // Classic Doom: FIRST return-to-0 is after _initterm.
                        // Enter D_DoomMain which runs full init + enters D_DoomLoop.
                        debug_log(&format!(
                            "[DOOM-RET0] After _initterm: entering D_DoomMain (0x12600) with fresh stack, {} kernel calls",
                            interp_kernel_calls
                        ));
                        iregs.esp = worker_stack_top - 0x2000;
                        memory.write_u32(iregs.esp, 0); // sentinel return address
                        iregs.eip = 0x0001_2600; // D_DoomMain full init
                                                 // Populate XLaunchData at a fixed guest address. On real Xbox,
                                                 // the dashboard fills this before launching the game. The config
                                                 // parser at 0x3E484 reads [0xB5228] → launch data pointer.
                                                 // Without valid launch data, the parser returns error 0x490 and
                                                 // numplayers/skill/episode stay at 0 — game can't start.
                        const LAUNCH_DATA: u32 = 0x01F0_0000; // below bump alloc
                                                              // Launch type = 2 (game launch from dashboard)
                        memory.write_u32(LAUNCH_DATA, 2);
                        // [launch_data+4] must match [0x1018C] for config parser check
                        let expected = memory.read_u32(0x0001_018C);
                        memory.write_u32(LAUNCH_DATA + 4, expected);
                        // Game config starts at offset 0x400 (copied by rep movsd at 0x3E4C7)
                        let cfg = LAUNCH_DATA + 0x400;
                        // Launch config: [0]=players, [1]=skill, [2]=episode, [3]=map
                        // byte[3]=-1 skips G_InitNew → game never starts a level!
                        // Set: 1 player, skill 2 (Hurt Me Plenty), episode 1, map 1
                        memory.write_u32(cfg, 0x01_01_02_01); // byte[0]=1 player, [1]=2 skill, [2]=1 ep, [3]=1 map
                        memory.write_u32(cfg + 4, 0); // byte[4]=0 nomonsters, [5]=0 respawn
                                                      // Set pointer: [0xB5228] = &launch_data_ptr, [[0xB5228]] = launch_data
                                                      // Parser: eax = [0xB5228]; ecx = [eax]; check ecx (launch type)
                        const LAUNCH_PTR: u32 = LAUNCH_DATA + 0x1000;
                        memory.write_u32(LAUNCH_PTR, LAUNCH_DATA); // indirect pointer
                        memory.write_u32(0x000B_5228, LAUNCH_PTR); // [0xB5228] → LAUNCH_PTR → LAUNCH_DATA
                        debug_log(&format!("[DOOM-PREINIT] XLaunchData at 0x{:08X}, ptr at 0x{:08X}, [0xB5228]=0x{:08X}",
                            LAUNCH_DATA, LAUNCH_PTR, LAUNCH_PTR));
                        continue;
                    } else if start_routine == 0x0003_E537
                        && DOOM_MAIN_ENTERED.load(std::sync::atomic::Ordering::Relaxed)
                    {
                        // Doom: D_DoomMain returned (init complete or crash).
                        // BREAK to outer AOT dispatch loop — D_DoomLoop's .text code
                        // is AOT-compiled and runs at native speed. The hybrid dispatch
                        // at the outer loop handles interpreter fallback for any gaps.
                        debug_log(&format!(
                            "[DOOM-AOT] D_DoomMain returned after {} kernel calls. Switching to AOT dispatch for D_DoomLoop.",
                            interp_kernel_calls
                        ));
                        // Set iregs.eip so post-interpreter handoff picks it up (line 1171).
                        // Enter the persistent loop label after D_DoomMain's one-time setup.
                        // The frame shape matches the prologue at 0x12600:
                        //   sub esp,0xC08; mov [esp+0xC04], __security_cookie; push esi
                        // The return slot remains at esp+0xC0C.
                        iregs.eip = 0x0001_2706; // D_DoomLoop top
                        let doom_entry_esp = worker_stack_top - 0x2000;
                        iregs.esp = doom_entry_esp - 0xC08 - 4;
                        memory.write_u32(iregs.esp, 0); // saved ESI
                        memory.write_u32(iregs.esp + 0xC08, memory.read_u32(0x000D_F5B0));
                        memory.write_u32(iregs.esp + 0xC0C, 0); // return sentinel
                        kernel_calls = interp_kernel_calls;
                        break; // Exit interpreter, enter outer AOT dispatch loop
                    } else if start_routine == 0x002A_9BC9 {
                        // Spider-Man: re-enter game_main on each ReturnedTo(0).
                        // The organic interpreter runs XapiThreadStartup→StartRoutine→
                        // pool_init→static_ctors→game_main, advancing 32 kcalls per cycle.
                        // Each re-entry preserves memory state from prior cycles.
                        // Do NOT break to forced phases — keep the interpreter running.
                        static SM_RET0: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let rn = SM_RET0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if rn < 10 || rn % 100 == 0 {
                            debug_log(&format!(
                                "[SM-RET0] #{} ReturnedTo(0) at {} kcalls, ESP=0x{:08X} EAX=0x{:08X}",
                                rn, interp_kernel_calls, iregs.esp, iregs.eax
                            ));
                        }
                        // Don't let the interpreter loop exit — keep re-entering.
                        // Re-enter at game_main with a fresh stack each time.
                        iregs.esp = worker_stack_top - 0x2000;
                        memory.write_u32(iregs.esp, 0); // sentinel
                        iregs.eip = 0x002A_52A0; // game_main
                        continue;
                    }
                    // Generic: worker init done. Break to outer loop for VBlank-paced DPC.
                    debug_log(&format!(
                        "AOT worker: Init complete ({} kernel calls). Breaking to outer dispatch loop.",
                        interp_kernel_calls
                    ));
                    ctx.guest.esp = iregs.esp;
                    guest_addr = 0;
                    break;
                }
                InterpResult::Timeout => {
                    // Don't break — continue interpreting. The CRT boot may need
                    // 200M+ instructions. Only exit when ESP returns to start_func level.
                    static TIMEOUT_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let tn = TIMEOUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if tn < 10 {
                        debug_log(&format!(
                            "[INTERP] Timeout #{} at 0x{:08X} ESP=0x{:08X} after {} kernel calls — continuing",
                            tn, iregs.eip, iregs.esp, interp_kernel_calls
                        ));
                    }
                    // Continue interpreting from where we left off
                }
                InterpResult::Unhandled { eip, mnemonic } => {
                    debug_log(&format!(
                        "AOT worker: Interpreter unhandled at 0x{:08X}: {} ({} kernel calls so far)",
                        eip, mnemonic, interp_kernel_calls
                    ));
                    break;
                }
                InterpResult::ReturnedTo(addr) => {
                    // Non-zero return — set EIP and let the loop-top AOT handoff check fire
                    if addr >= 0xF000_0000 && addr != 0xFFFF_FFFF {
                        let last_hook_str = match last_oovpa_hook {
                            Some((a, r)) => format!("last_hook=0x{:08X}(ret=0x{:08X})", a, r),
                            None => "last_hook=none".to_string(),
                        };
                        debug_log(&format!(
                            "[RET-SUSPICIOUS] Interpreter returned to 0x{:08X} {} ESP=0x{:08X}",
                            addr, last_hook_str, iregs.esp
                        ));
                    }
                    iregs.eip = addr;
                }
                InterpResult::OovpaHook {
                    guest_addr: ga,
                    ret_addr: or,
                } => {
                    // Track last hook for ReturnedTo(0) diagnostics
                    last_oovpa_hook = Some((ga, or));

                    // Warn on null or suspicious return address
                    if or == 0 || or >= 0xF000_0000 {
                        debug_log(&format!(
                            "[OOVPA-WARN] Hook 0x{:08X} has BAD ret_addr=0x{:08X} ESP=0x{:08X} — init may be broken",
                            ga, or, iregs.esp
                        ));
                    }

                    // AOT handoff: interpreter reached StartRoutine boundary.
                    // Transfer registers and let AOT handle the entire CRT boot.
                    if ga == start_routine && start_routine != 0x0003_E537 {
                        debug_log(&format!(
                            "[INTERP] AOT handoff at StartRoutine 0x{:08X} after {} kernel calls. ESP=0x{:08X}",
                            ga, interp_kernel_calls, iregs.esp
                        ));
                        ctx.guest.eax = iregs.eax;
                        ctx.guest.ebx = iregs.ebx;
                        ctx.guest.ecx = iregs.ecx;
                        ctx.guest.edx = iregs.edx;
                        ctx.guest.esi = iregs.esi;
                        ctx.guest.edi = iregs.edi;
                        ctx.guest.ebp = iregs.ebp;
                        ctx.guest.esp = iregs.esp;
                        // Generic XDK stub: mainXapiStartup always starts with
                        // CALL XapiApplyKernelPatches (E8 rel32). In HLE there's
                        // no kernel to patch — skip the CALL entirely.
                        // XapiApplyKernelPatches reads kernel code at 0xFFFF0000+
                        // which doesn't exist in our emulator (sign-extension AV).
                        let first_byte = memory.read_u8(ga);
                        if first_byte == 0xE8 {
                            let rel = memory.read_u32(ga + 1) as i32;
                            let kpatch = ga.wrapping_add(5).wrapping_add(rel as u32);
                            debug_log(&format!(
                                "[AOT-STUB] Skipping XapiApplyKernelPatches at 0x{:08X} (CALL from StartRoutine 0x{:08X})",
                                kpatch, ga
                            ));
                            guest_addr = ga + 5; // Skip CALL, enter at second instruction
                            iregs.eip = ga + 5; // Sync so post-loop code doesn't override
                        } else {
                            guest_addr = ga;
                        }
                        kernel_calls = interp_kernel_calls;
                        break;
                    }
                    // Classic Doom manual HLE hooks (not in OOVPA state)
                    if ga == 0x0001_1050 && doom_synthetic_framebuffer_enabled() {
                        // I_FinishUpdate: read Doom's 8-bit framebuffer, apply PLAYPAL
                        // palette, write XRGB8888 to guest memory for main thread display.
                        // POC: inject TITLEPIC from WAD if screens[0] is empty/uninitialized
                        doom_inject_titlepic(&memory);
                        doom_finish_update(&memory);
                        iregs.eax = 0;
                        iregs.esp = iregs.esp.wrapping_add(4);
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        live_counters
                            .alive
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    if ga == 0x0008_4325 {
                        // DirectSoundCreate: stdcall, 4 args (cleanup=16).
                        // Call proper HLE that sets up fake IDirectSound8 + vtable stubs.
                        let mut ds_args = [0u32; 8];
                        for i in 0..4u32 {
                            ds_args[i as usize] =
                                memory.read_u32(iregs.esp.wrapping_add(4 + i * 4));
                        }
                        let guest_mem = memory.base();
                        iregs.eax =
                            crate::xbox::apu::dsound::hle_dsound_create(&ds_args, guest_mem);
                        iregs.esp = iregs.esp.wrapping_add(4 + 16); // ret + 4 stdcall args
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    if ga == 0x0001_2C90 {
                        // TryRunTics: thiscall (ecx=this=0x101B58), 1 stack arg (tic count).
                        // Full game logic (G_Ticker → D_PageDrawer → W_CacheLumpName)
                        // takes millions of interpreted instructions. HLE to advance state.
                        let tic_count_arg = memory.read_u32(iregs.esp.wrapping_add(4));
                        let gs = memory.read_u32(0x0010_1BB8);
                        if gs >= 0x0100_0000 && gs < 0x1000_0000 {
                            // Advance gametic (gs+0x271C not confirmed — check render_flag path)
                            // Set render flag at [0xE0568] = 1 so D_DoomLoop renders
                            memory.write_u8(0x000E_0568, 1);
                            // Set [gs+0x2264] non-zero (draw count) for the render flag path
                            memory.write_u32(gs.wrapping_add(0x2264), 1);
                            // Set [gs+0x11D4] non-zero (tic available)
                            memory.write_u32(gs.wrapping_add(0x11D4), 1);
                        }
                        static TRT_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = TRT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 10 || n % 100 == 0 {
                            debug_log(&format!(
                                "[DOOM-TRT] #{} TryRunTics(tics={}) gs=0x{:08X} render_flag=1",
                                n, tic_count_arg, gs
                            ));
                        }
                        iregs.eax = 1; // al=1 → D_DoomLoop proceeds to render
                        iregs.esp = iregs.esp.wrapping_add(4 + 4); // ret + 1 stdcall arg
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        live_counters
                            .alive
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    if ga == 0x0001_1070 {
                        // I_WaitVBL (0x11070): skip BlockUntilVerticalBlank — just timing
                        iregs.eax = 0;
                        iregs.esp = iregs.esp.wrapping_add(4); // pop return address only
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        live_counters
                            .alive
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    // I_InitSound (0x11A80), S_Init (0x11AF0), R_Init (0x11080) removed —
                    // game code must run natively. DirectSoundCreate at 0x84325 is HLE'd
                    // to return proper fake IDirectSound8 objects.
                    if ga == 0x0004_5652 || ga == 0x0004_5325 {
                        // CRT malloc(size) at 0x45652 OR _nh_malloc(size, flag) at 0x45325
                        // Both are cdecl. Redirect to VA bump allocator.
                        let size = memory.read_u32(iregs.esp.wrapping_add(4));
                        let aligned = (size + 15) & !15; // 16-byte align
                        let kstate = unsafe {
                            &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState)
                        };
                        let ptr = if aligned > 0 && kstate.bump_va + aligned < 0x1000_0000 {
                            let p = kstate.bump_va;
                            kstate.bump_va += aligned;
                            p
                        } else {
                            0
                        };
                        // Game state pre-population REMOVED (2026-03-29).
                        // Let D_DoomMain populate its own game_state organically.
                        {
                            static ML: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let n = ML.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n < 30 || (n % 100000 == 0) {
                                // Trace: [ESP]=ret to _calloc_crt, [ESP+4]=size, [ESP+8]=flag
                                // [ESP+12]=caller of _calloc_crt
                                let caller = memory.read_u32(iregs.esp.wrapping_add(12));
                                debug_log(&format!(
                                    "[DOOM-MALLOC] #{} malloc(0x{:X}) → 0x{:08X} ret=0x{:08X} caller=0x{:08X} ESP=0x{:08X}",
                                    n, size, ptr, or, caller, iregs.esp
                                ));
                            }
                        }
                        iregs.eax = ptr;
                        iregs.esp = iregs.esp.wrapping_add(4); // pop ret addr (cdecl: caller cleans args)
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    if ga == 0x0002_36F0 {
                        // W_CheckNumForName(char* name) — name passed in ECX (MSVC __thiscall)
                        // Linear backward search through lumpinfo array.
                        // HLE: do the same search in Rust (much faster than interpreting).
                        let name_ptr = iregs.ecx;
                        let numlumps = memory.read_u32(0x0010_375C);
                        let lumpinfo = memory.read_u32(0x0010_3768);
                        let mut search_name = [0u8; 8];
                        for i in 0..8 {
                            let b = memory.read_u8(name_ptr.wrapping_add(i as u32));
                            search_name[i] = b.to_ascii_uppercase();
                        }
                        let mut result: i32 = -1;
                        // Search backward (last match wins, same as original)
                        if numlumps > 0
                            && numlumps < 100_000
                            && lumpinfo != 0
                            && lumpinfo < 0x1000_0000
                        {
                            for i in (0..numlumps as i32).rev() {
                                let entry = lumpinfo.wrapping_add((i as u32) * 20);
                                let mut lump_name = [0u8; 8];
                                for j in 0..8 {
                                    lump_name[j] = memory.read_u8(entry.wrapping_add(j as u32));
                                }
                                if lump_name == search_name {
                                    result = i;
                                    break;
                                }
                            }
                        }
                        {
                            static WCN_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let n = WCN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n < 20 || (n % 1000 == 0) {
                                let name_str = std::str::from_utf8(&search_name)
                                    .unwrap_or("?")
                                    .trim_end_matches('\0');
                                debug_log(&format!(
                                    "[DOOM-WCN] #{} W_CheckNumForName({:?}) → {} (numlumps={} lumpinfo=0x{:08X})",
                                    n, name_str, result, numlumps, lumpinfo
                                ));
                            }
                        }
                        iregs.eax = result as u32;
                        iregs.esp = iregs.esp.wrapping_add(4); // pop ret (cdecl)
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    if ga == 0x0003_E52F {
                        // Ensure player_flag stays set — TryRunTics clears it each iteration
                        // and calls 0x12820 to re-check. Without real network state, 0x12820
                        // returns 0 (inactive). Force it active so game tics actually run.
                        if memory.read_u8(0x0010_1B5C) == 0 {
                            memory.write_u8(0x0010_1B5C, 1);
                        }
                        // Ensure game_state+0x11D4 (tic available flag) is non-zero.
                        // TryRunTics skips G_Ticker entirely when this is 0.
                        // Set by network code in multiplayer; in single player we seed it.
                        let gs_ptr = memory.read_u32(0x0010_1BB8);
                        if gs_ptr >= 0x0100_0000 && gs_ptr < 0x1000_0000 {
                            let tic_flag = memory.read_u32(gs_ptr.wrapping_add(0x11D4));
                            if tic_flag == 0 {
                                memory.write_u32(gs_ptr.wrapping_add(0x11D4), 1);
                            }
                        }
                        // I_GetTime: Xbox Doom returns millisecond-scale value.
                        // Game loop computes: delta_ms * 0.001 * (-35.0) then neg → tics.
                        // So 1000ms delta → 35 tics (classic Doom rate).
                        // We advance 30ms per call (~1 tic per 2 calls, game calls twice per loop).
                        static DOOM_TICK: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let call_num = DOOM_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        // Return monotonic ms: 30ms per call → ~35 tics/sec when called ~2x/frame
                        let ms = call_num * 30;
                        iregs.eax = ms;
                        // On first game-loop I_GetTime, verify init + set advancedemo.
                        // game_state at [0x101BB8] is allocated by D_DoomMain's malloc.
                        // advancedemo = [gs+0x2750] must be non-zero so G_Ticker enters
                        // the title screen / demo path instead of crashing on empty
                        // level data. D_StartTitle normally sets this.
                        if call_num == 1 {
                            // Use cached game state from DOOM-STATE (captured during CRT init)
                            // [0x101BB8] is unreliable here — TryRunTics overwrites it
                            let gs =
                                DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
                            let np = memory.read_u32(0x0010_1B58);
                            let player_flag = memory.read_u8(0x0010_1B5C);
                            debug_log(&format!(
                                "[DOOM-GETTIME] Init check: cached_gs=0x{:08X} players={} player_flag={} cur_bb8=0x{:08X} base_time=0x{:08X} tic_count={} tic_accum={}",
                                gs, np, player_flag, memory.read_u32(0x0010_1BB8),
                                memory.read_u32(0x0010_1B70), memory.read_u32(0x0010_1B74), memory.read_u32(0x0010_1B78)
                            ));
                            // TryRunTics reads [0x101B5C] (player 0 active flag).
                            // If 0, TryRunTics returns 0 and D_DoomLoop never renders.
                            // G_InitNew should set this; if it didn't run, seed it.
                            if player_flag == 0 {
                                memory.write_u8(0x0010_1B5C, 1);
                                debug_log("[DOOM-FIX] Set player_flag[0]=1 at [0x101B5C]");
                            }
                            if gs >= 0x0100_0000 && gs < 0x1000_0000 {
                                let advdemo = memory.read_u32(gs.wrapping_add(0x2750));
                                debug_log(&format!(
                                    "[DOOM-GETTIME] advancedemo={} [gs+0x1F50]={} [gs+0x2284]={}",
                                    advdemo,
                                    memory.read_u32(gs.wrapping_add(0x1F50)),
                                    memory.read_u32(gs.wrapping_add(0x2284))
                                ));
                                if advdemo == 0 {
                                    memory.write_u32(gs.wrapping_add(0x2750), 1);
                                    debug_log("[DOOM-FIX] Set advancedemo=1 for title screen");
                                }
                                // Prevent div/0 in G_Ticker: [gs+0x1F50] is screen height divisor
                                let f50 = memory.read_u32(gs.wrapping_add(0x1F50));
                                if f50 == 0 {
                                    memory.write_u32(gs.wrapping_add(0x1F50), 1);
                                    debug_log("[DOOM-FIX] Set [gs+0x1F50]=1 (div/0 guard)");
                                }
                            }
                        }
                        if call_num < 10 || call_num % 100 == 0 {
                            let gs =
                                DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
                            let (
                                gametic,
                                maketic,
                                singletics,
                                gameaction,
                                gamestate_val,
                                render_flag,
                            ) = if gs >= 0x0100_0000 && gs < 0x1000_0000 {
                                (
                                    memory.read_u32(gs.wrapping_add(0x271C)),
                                    memory.read_u32(gs.wrapping_add(0x1F44)),
                                    memory.read_u32(gs.wrapping_add(0x22A0)),
                                    memory.read_u32(gs.wrapping_add(0x2284)),
                                    memory.read_u32(gs.wrapping_add(0x2288)),
                                    memory.read_u8(0x000E_0568),
                                )
                            } else {
                                (0, 0, 0, 0, 0, 0)
                            };
                            debug_log(&format!(
                                "[DOOM-GETTIME] #{} ms={} ret=0x{:08X} gametic={} maketic={} singletics={} gameaction={} gs_val={} render={}",
                                call_num, ms, or, gametic, maketic, singletics, gameaction, gamestate_val, render_flag
                            ));
                        }
                        iregs.esp = iregs.esp.wrapping_add(4); // pop return address
                        iregs.eip = or;
                        interp_kernel_calls += 1;
                        live_counters
                            .kernel_calls
                            .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    // Execute the OOVPA HLE function inline
                    let guest_mem_ptr = memory.base() as *mut u8;
                    let hle_result = crate::xbox::aot::oovpa::execute_hle_for_interpreter(
                        ga,
                        or,
                        guest_mem_ptr,
                        &memory,
                        &mut iregs.eax,
                        &mut iregs.ecx,
                        &mut iregs.edx,
                        iregs.esp,
                        unsafe {
                            &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState)
                        },
                    );
                    // Pop return address + stdcall cleanup
                    iregs.esp = iregs.esp.wrapping_add(4 + hle_result.cleanup as u32);
                    iregs.eip = or;
                    interp_kernel_calls += 1;
                    live_counters
                        .kernel_calls
                        .store(interp_kernel_calls, std::sync::atomic::Ordering::Relaxed);
                    live_counters
                        .alive
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                    debug_log(&format!(
                        "[INTERP-OOVPA] #{} {} at 0x{:08X} ret=0x{:08X} eax=0x{:08X} ESP=0x{:08X} cleanup={}",
                        interp_kernel_calls, hle_result.name, ga, or, iregs.eax, iregs.esp, hle_result.cleanup
                    ));
                }
                InterpResult::AccessViolation { eip, addr } => {
                    // Try guest SEH dispatch — the game's __try/__except should catch this.
                    const FS_ZERO: u32 = 0x0C00_0000;
                    let mut seh_head = memory.read_u32(FS_ZERO);
                    debug_log(&format!(
                        "[INTERP-AV] AV at 0x{:08X} addr=0x{:08X} fs:[0]=0x{:08X} ESP=0x{:08X} saved={:?}",
                        eip, addr, seh_head, iregs.esp, last_good_seh.as_ref().map(|s| s.head)
                    ));

                    // If KPCR was corrupted by wild write (rep movsd), use saved SEH state
                    let use_saved =
                        (seh_head == 0 || seh_head == 0xFFFF_FFFF || seh_head >= 0x2000_0000)
                            && last_good_seh.is_some();

                    let mut dispatched = false;
                    if use_saved {
                        // KPCR corrupted — dispatch from saved snapshot (including scope table data)
                        let snap = last_good_seh.as_ref().unwrap();
                        debug_log(&format!(
                            "[INTERP-SEH-SAVED] head=0x{:08X} prev=0x{:08X} handler=0x{:08X} scope=0x{:08X} ebp=0x{:08X} trylevel={} saved_entries={}",
                            snap.head, snap.prev, snap.handler, snap.scope_addr, snap.frame_ebp, snap.trylevel, snap.scope_entries.len()
                        ));
                        if snap.handler >= 0x10000
                            && snap.handler < 0x0080_0000
                            && snap.trylevel <= 100
                            && !snap.scope_entries.is_empty()
                        {
                            // Walk saved scope entries (not memory — memory is corrupted)
                            let mut level = snap.trylevel;
                            for _walk in 0..32 {
                                if level == 0xFFFF_FFFF {
                                    break;
                                }
                                let idx = level as usize;
                                if idx >= snap.scope_entries.len() {
                                    break;
                                }
                                let (enclosing, filter, except_handler) = snap.scope_entries[idx];
                                debug_log(&format!(
                                    "[INTERP-SEH-SAVED] scope[{}]: enclosing={} filter=0x{:08X} handler=0x{:08X}",
                                    idx, enclosing as i32, filter, except_handler
                                ));
                                // Accept catch-all (0xFFFFFFFF) OR any filter function for hard AVs.
                                // Real MSVC SEH would call the filter function, but we can't do that
                                // in the interpreter. For AVs at clearly invalid addresses, assume
                                // the filter returns EXCEPTION_EXECUTE_HANDLER.
                                let is_catch_all = filter == 0xFFFF_FFFF;
                                let is_hard_av = addr >= 0x8000_0000 || addr == 0;
                                let filter_is_func = filter >= 0x10000 && filter < 0x0080_0000;
                                if (is_catch_all || (is_hard_av && filter_is_func))
                                    && except_handler >= 0x10000
                                    && except_handler < 0x0080_0000
                                {
                                    debug_log(&format!(
                                        "[INTERP-SEH-SAVED] AV at 0x{:08X} → handler=0x{:08X} EBP=0x{:08X} (from saved SEH)",
                                        eip, except_handler, snap.frame_ebp
                                    ));
                                    // Restore KPCR: unlink this frame
                                    memory.write_u32(FS_ZERO, snap.prev);
                                    // Set trylevel to enclosing
                                    memory.write_u32(snap.frame_ebp.wrapping_sub(4), enclosing);
                                    iregs.esp = snap.frame_ebp;
                                    iregs.ebp = snap.frame_ebp;
                                    iregs.eax = 0;
                                    iregs.eip = except_handler;
                                    dispatched = true;
                                    break;
                                }
                                level = enclosing;
                            }
                        }
                    }

                    if !dispatched
                        && seh_head != 0xFFFF_FFFF
                        && seh_head != 0
                        && seh_head < 0x2000_0000
                    {
                        let mut frame_ptr = seh_head;
                        for _walk in 0..16 {
                            if frame_ptr == 0xFFFF_FFFF
                                || frame_ptr == 0
                                || frame_ptr >= 0x2000_0000
                            {
                                break;
                            }
                            let prev = memory.read_u32(frame_ptr);
                            let handler = memory.read_u32(frame_ptr + 4);
                            if handler < 0x10000 || handler >= 0x0080_0000 {
                                frame_ptr = prev;
                                continue;
                            }
                            let scope_table = memory.read_u32(frame_ptr + 8);
                            let frame_ebp = frame_ptr.wrapping_add(0x10);
                            let trylevel = memory.read_u32(frame_ebp.wrapping_sub(4));
                            if scope_table < 0x10000 || scope_table >= 0x0080_0000 || trylevel > 100
                            {
                                frame_ptr = prev;
                                continue;
                            }
                            // Walk scope table for SEH handler
                            let mut level = trylevel;
                            for _scope in 0..32 {
                                if level == 0xFFFF_FFFF {
                                    break;
                                }
                                let entry_addr = scope_table + level * 12;
                                if entry_addr >= 0x0080_0000 {
                                    break;
                                }
                                let enclosing = memory.read_u32(entry_addr) as i32;
                                let filter = memory.read_u32(entry_addr + 4);
                                let except_handler = memory.read_u32(entry_addr + 8);
                                let is_catch_all = filter == 0xFFFF_FFFF;
                                let is_hard_av = addr >= 0x8000_0000 || addr == 0;
                                let filter_is_func = filter >= 0x10000 && filter < 0x0080_0000;
                                if (is_catch_all || (is_hard_av && filter_is_func))
                                    && except_handler >= 0x10000
                                    && except_handler < 0x0080_0000
                                {
                                    // Dispatch to __except handler
                                    debug_log(&format!(
                                        "[INTERP-SEH-DISPATCH] AV at 0x{:08X} → handler=0x{:08X} EBP=0x{:08X}",
                                        eip, except_handler, frame_ebp
                                    ));
                                    // Unlink SEH frame
                                    memory.write_u32(FS_ZERO, prev);
                                    // Set trylevel to enclosing
                                    memory.write_u32(frame_ebp.wrapping_sub(4), enclosing as u32);
                                    // Restore state: ESP = EBP (standard frame), EBP = frame pointer
                                    iregs.esp = frame_ebp;
                                    iregs.ebp = frame_ebp;
                                    iregs.eax = 0;
                                    iregs.eip = except_handler;
                                    dispatched = true;
                                    break;
                                }
                                level = enclosing as u32;
                            }
                            if dispatched {
                                break;
                            }
                            frame_ptr = prev;
                        }
                    }
                    if !dispatched {
                        debug_log(&format!(
                            "[INTERP-AV] No SEH handler found. Exiting interpreter. EIP=0x{:08X}",
                            eip
                        ));
                        break;
                    }
                    // Continue interpreting from the __except handler
                }
                _ => {
                    debug_log(&format!(
                        "AOT worker: Interpreter exit after {} kernel calls, EIP=0x{:08X}",
                        interp_kernel_calls, iregs.eip
                    ));
                    break;
                }
            }
        }
        // Sync regs back
        ctx.guest.eax = iregs.eax;
        ctx.guest.ecx = iregs.ecx;
        ctx.guest.edx = iregs.edx;
        ctx.guest.ebx = iregs.ebx;
        ctx.guest.esp = iregs.esp;
        ctx.guest.ebp = iregs.ebp;
        ctx.guest.esi = iregs.esi;
        ctx.guest.edi = iregs.edi;
        kernel_calls = interp_kernel_calls;
        debug_log(&format!(
            "AOT worker: Interpreter startup complete — {} kernel calls, EIP=0x{:08X}",
            interp_kernel_calls, iregs.eip
        ));
        // Hand off to AOT trampoline at the interpreter's current EIP.
        // If EIP is 0, the interpreter returned to sentinel → use forced phases.
        // If EIP is non-zero, resume AOT execution from where the interpreter left off.
        if iregs.eip != 0 && iregs.eip < 0x8000_0000 {
            guest_addr = iregs.eip;
            debug_log(&format!(
                "AOT worker: Interpreter → AOT handoff at 0x{:08X}",
                guest_addr
            ));
        } else {
            // Interpreter returned to sentinel (EIP=0). CRT boot is done.
            //
            // 2026-04-24: game_main forced re-entry REMOVED.
            // Agent investigation WS-A10 + WS-A15 determined:
            // - Organic phase already ran through 9814 swaps/clears via OOVPA HLE
            // - Re-entering game_main with ESP=fresh, sentinel RET=0 immediately exits
            //   (exit=7) because game_main's SEH prolog pops past the sentinel
            // - Net effect was dispatches+=1-2 then halt — no useful progress
            //
            // If a future game actually needs the re-entry, gate it on dispatches==0
            // AND specific start_routine. Currently this is a no-op log.
            let is_child_thread_early = stack_idx > 0;
            if !is_child_thread_early && start_routine == 0x002A_9BC9 {
                debug_log(&format!(
                    "[WORKER-GAMEMAIN-SKIPPED] organic phase done, game_main re-entry disabled (dispatches={} kernel_calls={})",
                    ctx.dispatch_count, kernel_calls
                ));
            } else {
                debug_log(&format!(
                    "[WORKER-GAMEMAIN-SKIPPED] CRT boot done, forced re-entry disabled (start_routine=0x{:08X} dispatches={} kernel_calls={})",
                    start_routine, ctx.dispatch_count, kernel_calls
                ));
            }
            // Leave guest_addr = 0 so the outer loop flows into the VBlank-wait /
            // thread-exit path instead of forcing a fresh-stack game_main re-entry.
            guest_addr = 0;
        }
    }

    let is_child_thread = stack_idx > 0;

    'outer: loop {
        // Child threads: exit immediately when guest code returns (no DPC/ISR injection).
        if guest_addr == 0 && is_child_thread {
            let scene = memory.read_u32(0x003F_5BEC);
            let scene_118 = if scene != 0 && scene < 0x2000_0000 {
                memory.read_u32(scene + 0x118)
            } else {
                0
            };
            debug_log(&format!(
                "Child thread #{} exiting: dispatches={} kernel_calls={} scene=0x{:08X} scene+0x118=0x{:08X}",
                stack_idx, ctx.dispatch_count, kernel_calls, scene, scene_118
            ));
            break 'outer;
        }

        // After guest code exits (guest_addr=0), wait for VBlank signal from main thread.
        // C++ uses SuspendThread/SetThreadContext for async ISR injection.
        // We use condvar-based synchronization: worker blocks, main signals at 60Hz.
        if guest_addr == 0 {
            // === DIAGNOSTIC: Log organic state before synthetic injection ===
            if phase == 0 {
                // H06 fix (2026-04-24): one-shot gate. Without this, when the
                // worker keeps re-entering phase==0 (each iteration after a
                // RET_TO_ZERO / recovery that returns guest_addr=0), we fire
                // signal_worker_done() + ORGANIC PHASE COMPLETE hundreds of
                // times. Observed 380× in one 20-second run, spamming the log
                // and re-notifying the main thread redundantly. Gate both
                // one-shot via an AcqRel atomic so the phase-0 completion is
                // a single event.
                static PHASE0_COMPLETE: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                let first_phase0 = !PHASE0_COMPLETE.swap(true, std::sync::atomic::Ordering::AcqRel);

                if first_phase0 {
                    // Signal that the worker's organic phase is done.
                    // The main thread's NtWaitForSingleObject is blocking on this.
                    crate::xbox::aot::veh::signal_worker_done();
                    // NOTE: Do NOT signal CRT_BOOT_COMPLETE here — this is too early.
                    // ISR injection should only start after D3D CreateDevice registers
                    // the real VBlank ISR. The capture_isr filter in hal.rs prevents
                    // USB OHCI from poisoning the heartbeat, but we also don't want
                    // to inject a non-existent VBlank ISR during CRT init.
                    debug_log(&format!(
                        "[BACKTRACE] ORGANIC PHASE COMPLETE (worker_done signaled): dispatches={} kernel_calls={} mmio={} pb={} draw={} esp=0x{:08X}",
                        ctx.dispatch_count, kernel_calls, ctx.mmio_count, ctx.pb_commands, ctx.draw_calls, ctx.guest.esp
                    ));
                }
                // If not first_phase0, still run the block below (stats / synthetic
                // FrameContext init are idempotent via their own FC_INIT latch),
                // but skip the noisy log + redundant signal.
                let _ = first_phase0;

                // 2026-04-23: Forced game_main re-entry removed (WS-A10).
                //
                // The organic phase already ran game_main's logic: the
                // [BACKTRACE] line above this block shows kernel_calls=577
                // and ~9814 Swap/Clear iterations at signal_worker_done.
                // Forced re-entry with a fresh ESP + ret=0 sentinel caused
                // game_main's SEH prolog to immediately exit (exit_reason=7,
                // per WS-A15) — useless work that also destroyed the real
                // worker stack state. Trust the organic phase.
                //
                // Synthetic FrameContext allocation removed from the default path.
                // If Spider-Man reaches this block, the worker has already returned
                // from the organic startup path; publishing a zeroed object here
                // makes later frame code look healthier than the guest actually is.
                // Dump last 5 kernel calls for chain analysis
                for (i, (idx, ord, a, eax)) in call_history.iter().rev().take(5).enumerate() {
                    let ord_name = crate::xbox::kernel::ordinals::name(*ord);
                    debug_log(&format!(
                        "[BACKTRACE]   organic_tail[{}]: #{} ord={} ({}) args=[0x{:X},0x{:X}] eax=0x{:08X}",
                        i, idx, ord, ord_name, a[0], a[1], eax
                    ));
                }
            }

            // Dump key globals after organic phase to see what pool_init/organic path set.
            //
            // 2026-04-21 corrections to labels:
            //   - "Input" was misnamed — 0x00726690 is the game's FSM/state-machine
            //     value (0=IDLE, 1=intermediate, 2=ACTIVE), NOT an Input subsystem
            //     pointer. Renamed to "State".
            //   - "Upd1"/"Upd2" kept as-is — those are likely update-callback
            //     subsystem pointers per their offsets in the game globals area.
            {
                let g_app = memory.read_u32(0x003F5EB0);
                let g_scene = memory.read_u32(0x003F5BEC);
                let g_state = memory.read_u32(0x00726690);
                let g_upd1 = memory.read_u32(0x003F87A8);
                let g_upd2 = memory.read_u32(0x003F7D90);
                let g_engine = memory.read_u32(0x004BC614);
                let g_frame = memory.read_u32(0x004BC630);
                let mut sub_count = 0u32;
                for j in 0..102u32 {
                    if memory.read_u32(0x003DC550 + j * 4) != 0 {
                        sub_count += 1;
                    }
                }
                debug_log(&format!(
                    "[GLOBALS] phase={} App=0x{:08X} Scene=0x{:08X} State=0x{:08X} Upd1=0x{:08X} Upd2=0x{:08X} Engine=0x{:08X} Frame=0x{:08X} SubFlags={}/102",
                    phase, g_app, g_scene, g_state, g_upd1, g_upd2, g_engine, g_frame, sub_count
                ));
            }

            // H06 fix (2026-04-24): also one-shot-gate this log. Same pattern
            // as the BACKTRACE log above — without the gate this fires every
            // retry iteration and floods the log.
            if ORGANIC_MODE && phase == 0 {
                static PHASE0_LOG: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                if !PHASE0_LOG.swap(true, std::sync::atomic::Ordering::AcqRel) {
                    debug_log(&format!(
                        "[ORGANIC] Phase 0 complete. kernel_calls={} esp=0x{:08X}",
                        kernel_calls, ctx.guest.esp
                    ));
                }
            }

            // LEGACY_FORCED_PHASES: forced init function injection + synthetic D3D +
            // ISR injection from worker thread. All disabled — organic chain handles this.
            // Gate: set to true to re-enable legacy scaffolding for debugging.
            const LEGACY_FORCED_PHASES: bool = false;

            if LEGACY_FORCED_PHASES && phase <= 3 {
                if phase == 0 {
                    if crate::xbox::aot::oovpa::has_d3d_table_init_fired()
                        && !crate::xbox::aot::oovpa::has_create_device_fired()
                    {
                        debug_log("[SYNTH-D3D] D3D table init done, injecting synthetic CreateDevice+Swap+Draw");
                        crate::xbox::aot::oovpa::force_synthetic_d3d(&mut *ctx);
                    }
                }

                // Force-enter init functions — GAME SPECIFIC.
                // Only Spider-Man has known addresses. Other games use pure organic + hybrid.
                // Detect Spider-Man by its StartRoutine address
                let is_spiderman = start_routine == 0x002A9BC9;
                let init_funcs: &[(u32, &str)] = if is_spiderman {
                    &[
                        (0x002AE296, "static_ctors_1"),
                        (0x002AE23E, "static_ctors_2"),
                        (0x002A4BF0, "app_ctor"),
                        (0x002A52A0, "game_main"),
                    ]
                } else {
                    // For non-Spider-Man games: no forced phases.
                    // The organic worker + hybrid interpreter handles everything.
                    &[]
                };
                // Use phase to track which function to enter next
                let func_idx = phase as usize; // phase 0 = first function
                if func_idx < init_funcs.len() {
                    let (func_addr, name) = init_funcs[func_idx];
                    if let Some(_) = ctx.addr_hash.lookup(func_addr) {
                        let new_esp = worker_stack_top - 0x2000; // clean stack area
                        ctx.guest.esp = new_esp;
                        ctx.guest.ebp = 0;
                        memory.write_u32(new_esp, 0); // ret addr → RET_TO_ZERO
                                                      // For game_main: re-init game state objects AFTER app_ctor
                                                      // (app_ctor may have cleared our pre-init values).
                        if func_addr == 0x002A52A0 {
                            memory.write_u32(new_esp + 4, 0);
                            memory.write_u32(new_esp + 8, 0);
                            memory.write_u32(new_esp + 12, 0);
                            // Re-init engine/App/scene objects that app_ctor may have wiped
                            let app_addr = 0x01F0_0000u32;
                            memory.write_u32(0x003F_5EB0, app_addr);
                            memory.write_u32(0x004B_C614, app_addr);
                            // Quit flag at +0x184 must be 0
                            memory.write_u8(app_addr + 0x184, 0);
                            let scene_mgr = 0x01F0_0400u32;
                            let frame_ctx = 0x01F0_0800u32;
                            memory.write_u32(frame_ctx + 0x18, scene_mgr);
                            memory.write_u32(frame_ctx + 0x20, 0); // frame counter: 0 = enter render path immediately
                            memory.write_u32(0x004B_C630, frame_ctx);
                            memory.write_u32(0x003F_5BEC, scene_mgr); // global scene manager pointer
                                                                      // Scene render flags: enable now that XGRPH parser is stubbed.
                                                                      // Without these, the scene render at 0x000F710C is skipped every frame.
                            memory.write_u8(scene_mgr + 0x186, 1); // render enable (0x000F21FC)
                            memory.write_u8(scene_mgr + 0x183, 1); // scene render flag (0x000F7100)

                            // === GAME STATE TRANSACTION ===
                            // All fields written atomically before worker starts reading.
                            // Worker is blocked in dispatch loop — no partial reads possible.
                            //
                            // State machine at 0x726690:
                            //   +0x00 = state (0=idle, 1=transition, 2=active)
                            //   +0x14 = scene count
                            //   +0x18 = scene[0].time (float)
                            //   +0x1C = scene[0].ptr (scene object)
                            //
                            // App object:
                            //   +0x108 = scene count (read by 0x000F7536)
                            //
                            // Scene manager:
                            //   +0x186 = render enable (already set above)
                            //   +0x18  = active scene list head
                            //
                            let sm = 0x0072_6690u32;
                            memory.write_u32(sm + 0x00, 0); // state = IDLE initially
                                                            // State transitions to ACTIVE after file I/O completes.
                                                            // Set by main thread retro_run() after N frames via deferred activation.
                            memory.write_u32(sm + 0x14, 1); // scene_count = 1
                            memory.write_u32(sm + 0x18, 0x3F80_0000); // scene[0].time = 1.0f
                            memory.write_u32(sm + 0x1C, scene_mgr); // scene[0].ptr → our scene mgr
                            memory.write_u32(sm + 0x108, 0); // sub_state = 0
                            memory.write_u32(sm + 0x1FC, 0); // flags = 0
                            memory.write_u32(app_addr + 0x108, 1); // App.scene_count = 1
                                                                   // Scene manager needs a scene list for 0x000F21F0 to iterate
                            memory.write_u32(scene_mgr + 0x18, 0); // scene list head = NULL (safe: checked before deref)

                            // === RENDER CALLBACK INJECTION ===
                            // The render pipeline at 0x00297A94 iterates a linked list at
                            // [render_ctx+0x114]. Each entry has:
                            //   +0x00: next pointer (NULL = end of list)
                            //   +0x14: function pointer (the draw function to call)
                            //   +0x18: argument (passed as push arg before call)
                            // We inject a single callback that calls DrawIndexedVertices
                            // via one of its 4 call sites (0x0029817D).
                            //
                            // The render context is at static address 0x4CE160.
                            // Callback entry placed at 0x01F01000 (safe pre-init area).
                            let callback_entry = 0x01F0_1000u32;
                            memory.write_u32(callback_entry + 0x00, 0); // next = NULL
                            memory.write_u32(callback_entry + 0x04, 0); // padding
                            memory.write_u32(callback_entry + 0x08, 0); // padding
                            memory.write_u32(callback_entry + 0x0C, 0); // padding
                            memory.write_u32(callback_entry + 0x10, 0); // padding
                            memory.write_u32(callback_entry + 0x14, 0x0029_817D); // DrawIndexedVertices caller
                            memory.write_u32(callback_entry + 0x18, 0); // arg = 0

                            // Write callback entry to BOTH callback lists in the static
                            // render context at 0x4CE160
                            let render_ctx = 0x004C_E160u32;
                            memory.write_u32(render_ctx + 0x114, callback_entry);
                            memory.write_u32(render_ctx + 0x118, callback_entry);
                            debug_log(&format!(
                                "[RE-INIT] Render callback injected: [0x{:08X}+0x114] -> 0x{:08X} -> fn 0x0029817D",
                                render_ctx, callback_entry
                            ));
                            debug_log("[RE-INIT] Game state TRANSACTION committed: state=ACTIVE scene_count=1");
                        }
                        debug_log(&format!(
                            "[BACKTRACE] PRE-PHASE {}: dispatches={} kernel_calls={} mmio={} pb={} draw={}",
                            phase, ctx.dispatch_count, kernel_calls, ctx.mmio_count, ctx.pb_commands, ctx.draw_calls
                        ));
                        debug_log(&format!(
                            "[THREAD] Force-entering {} at 0x{:08X} (phase {})",
                            name, func_addr, phase
                        ));

                        // For game_main: use the micro-interpreter ONLY if no AOT code.
                        // Normally game_main is AOT-compiled, so we just set guest_addr
                        // and let the outer loop enter the trampoline.
                        if func_addr == 0x002A_52A0 && ctx.addr_hash.lookup(func_addr).is_none() {
                            use crate::xbox::aot::micro_interp::{self, InterpResult, X86Regs};
                            debug_log(
                                "[INTERP] Starting micro-interpreter for game_main (0x002A52A0)",
                            );
                            let mut iregs = X86Regs::new();
                            iregs.esp = ctx.guest.esp;
                            iregs.ebp = ctx.guest.ebp;
                            iregs.eax = ctx.guest.eax;
                            iregs.ecx = ctx.guest.ecx;
                            iregs.edx = ctx.guest.edx;
                            iregs.ebx = ctx.guest.ebx;
                            iregs.esi = ctx.guest.esi;
                            iregs.edi = ctx.guest.edi;

                            // OOVPA addresses that should route to HLE
                            let oovpa_addrs: Vec<u32> = vec![
                                0x002F_3BD0, // Direct3D_CreateDevice
                                0x002F_4CA0, // D3DDevice_Clear
                                0x002F_47D0, // D3DDevice_Swap
                                0x002F_1BB0, // D3DDevice_SetVertexShader
                                0x002F_1680, // D3DDevice_SetPixelShaderConstant
                                0x002F_2D70, // D3D_MakeSpace
                                0x002F_2920, // D3DDevice_BeginPush
                                0x002F_2960, // D3DDevice_KickOff
                                0x002E_D860, // D3DDevice_DrawVertices
                                0x002E_D900, // D3DDevice_DrawIndexedVertices
                            ];

                            let ret_addr = 0u32; // game_main should never return (frame loop)
                            let max_insns = 10_000_000u64; // 10M instructions max

                            let mut total_interpreted = 0u64;
                            let mut kernel_calls_interp = 0u32;
                            let mut oovpa_calls_interp = 0u32;

                            loop {
                                let entry_eip = iregs.eip;
                                let result = micro_interp::interpret(
                                    &memory,
                                    &mut iregs,
                                    entry_eip,
                                    ret_addr,
                                    max_insns,
                                    &oovpa_addrs,
                                    &ctx.exec_ranges,
                                );
                                match result {
                                    InterpResult::ReturnedTo(addr) => {
                                        debug_log(&format!(
                                            "[INTERP] game_main returned to 0x{:08X} (should never happen). insns={} kcalls={} oovpa={}",
                                            addr, total_interpreted, kernel_calls_interp, oovpa_calls_interp
                                        ));
                                        break;
                                    }
                                    InterpResult::KernelCall {
                                        ordinal,
                                        ret_addr: kr,
                                    } => {
                                        kernel_calls_interp += 1;
                                        // Route to kernel dispatcher
                                        let args = [
                                            memory.read_u32(iregs.esp + 4),
                                            memory.read_u32(iregs.esp + 8),
                                            memory.read_u32(iregs.esp + 12),
                                            memory.read_u32(iregs.esp + 16),
                                        ];
                                        if kernel_calls_interp <= 20 {
                                            debug_log(&format!(
                                                "[INTERP] Kernel #{} ord={} ret=0x{:08X} args=[0x{:X},0x{:X}]",
                                                kernel_calls_interp, ordinal, kr, args[0], args[1]
                                            ));
                                        }
                                        // Pop ret addr and args (stdcall)
                                        let argc =
                                            crate::xbox::kernel::ordinals::arg_count(ordinal);
                                        iregs.esp += 4 + (argc as u32) * 4;
                                        iregs.eip = kr;
                                        // TODO: actually dispatch the kernel call and set EAX
                                        iregs.eax = 0; // default return
                                    }
                                    InterpResult::OovpaHook {
                                        guest_addr: ga,
                                        ret_addr: or,
                                    } => {
                                        oovpa_calls_interp += 1;
                                        if oovpa_calls_interp <= 30 || or == 0 || or >= 0xF000_0000
                                        {
                                            debug_log(&format!(
                                                "[INTERP] OOVPA #{} at 0x{:08X} ret=0x{:08X} ESP=0x{:08X}{}",
                                                oovpa_calls_interp, ga, or, iregs.esp,
                                                if or == 0 { " *** BAD RET ADDR — hook will jump to NULL" }
                                                else if or >= 0xF000_0000 { " *** SUSPICIOUS ret addr" }
                                                else { "" }
                                            ));
                                        }
                                        // Pop ret addr (caller cleanup depends on function)
                                        iregs.esp += 4; // pop ret addr only, caller does cleanup
                                        iregs.eip = or;
                                        iregs.eax = 0; // HLE return value
                                    }
                                    InterpResult::Timeout => {
                                        debug_log(&format!(
                                            "[INTERP] Timeout after {}M instructions. EIP=0x{:08X} ESP=0x{:08X}",
                                            max_insns / 1_000_000, iregs.eip, iregs.esp
                                        ));
                                        break;
                                    }
                                    InterpResult::Unhandled { eip, mnemonic } => {
                                        debug_log(&format!(
                                            "[INTERP] Unhandled opcode at 0x{:08X}: {} (insns={} kcalls={} oovpa={})",
                                            eip, mnemonic, total_interpreted, kernel_calls_interp, oovpa_calls_interp
                                        ));
                                        break;
                                    }
                                    InterpResult::AccessViolation { eip, addr } => {
                                        debug_log(&format!(
                                            "[INTERP] AV at 0x{:08X} accessing 0x{:08X}",
                                            eip, addr
                                        ));
                                        break;
                                    }
                                }
                            }

                            // Copy interpreter regs back to guest state
                            ctx.guest.esp = iregs.esp;
                            ctx.guest.ebp = iregs.ebp;
                            ctx.guest.eax = iregs.eax;
                            ctx.guest.ecx = iregs.ecx;
                            ctx.guest.edx = iregs.edx;
                            ctx.guest.ebx = iregs.ebx;
                            ctx.guest.esi = iregs.esi;
                            ctx.guest.edi = iregs.edi;

                            debug_log(&format!(
                                "[INTERP] game_main interpreter done. App=[0x3F5EB0]=0x{:08X} Engine=[0x4BC614]=0x{:08X}",
                                memory.read_u32(0x003F5EB0), memory.read_u32(0x004BC614)
                            ));

                            // Continue to next phase (or break if game_main entered frame loop)
                            phase += 1;
                            continue 'outer;
                        }

                        guest_addr = func_addr;
                        phase += 1;
                        continue 'outer;
                    }
                }
            }

            if !LEGACY_FORCED_PHASES {
                // Organic mode: game's worker returned after init (KeSetTimer + DPC).
                // Check if a DPC was captured — if so, fire it on each VBlank to
                // drive the game's timer-based frame loop.
                // This matches real Xbox: KeSetTimer → timer expires → kernel fires DPC.
                let (isr, dpc) = crate::xbox::aot::veh_dpc::get_captured_routines();
                static DPC_CHECK_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let chk = DPC_CHECK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if chk < 5 {
                    debug_log(&format!(
                        "[DPC-CHECK] #{} isr=0x{:08X} dpc=0x{:08X} phase={} guest_addr=0x{:08X}",
                        chk, isr, dpc, phase, guest_addr
                    ));
                }
                if isr == 0 && dpc == 0 {
                    // No ISR/DPC captured yet — wait and retry
                    if crate::xbox::aot::veh::is_stop_requested() {
                        debug_log("AOT worker: stop requested, exiting");
                        break 'outer;
                    }
                    if !vblank.wait_timeout(std::time::Duration::from_millis(500)) {
                        if crate::xbox::aot::veh::is_stop_requested() {
                            break 'outer;
                        }
                    }
                    live_counters
                        .dispatch_count
                        .store(ctx.dispatch_count, Ordering::Relaxed);
                    live_counters
                        .mmio_count
                        .store(ctx.mmio_count, Ordering::Relaxed);
                    live_counters
                        .pb_commands
                        .store(ctx.pb_commands, Ordering::Relaxed);
                    live_counters
                        .draw_calls
                        .store(ctx.draw_calls, Ordering::Relaxed);
                    live_counters
                        .kernel_calls
                        .store(kernel_calls, Ordering::Relaxed);
                    continue 'outer;
                }
                // DPC or ISR captured — fire it to drive the game's frame loop.
                // Set timer SignalState before firing DPC (real kernel does this).
                let timer_ptr = crate::xbox::aot::veh_dpc::get_dpc_timer();
                if timer_ptr != 0 {
                    memory.write_u32(timer_ptr + 4, 1); // SignalState = 1
                }
                if crate::xbox::aot::veh_dpc::suppress_guest_isr_dpc_injection() {
                    if !vblank.wait_timeout(std::time::Duration::from_millis(100)) {
                        if crate::xbox::aot::veh::is_stop_requested() {
                            break 'outer;
                        }
                    }
                    let _ = take_pending_dpc_object(&memory, "post-ret-suppress");
                    static SUPPRESS_POST_RET_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n =
                        SUPPRESS_POST_RET_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 10 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[DPC-SUPPRESS] post-RET guest ISR/DPC/frame-loop fire skipped #{} isr=0x{:08X} dpc=0x{:08X} timer=0x{:08X} esp=0x{:08X}",
                            n, isr, dpc, timer_ptr, ctx.guest.esp
                        ));
                    }
                    live_counters
                        .dispatch_count
                        .store(ctx.dispatch_count, Ordering::Relaxed);
                    live_counters
                        .mmio_count
                        .store(ctx.mmio_count, Ordering::Relaxed);
                    live_counters
                        .pb_commands
                        .store(ctx.pb_commands, Ordering::Relaxed);
                    live_counters
                        .draw_calls
                        .store(ctx.draw_calls, Ordering::Relaxed);
                    live_counters
                        .kernel_calls
                        .store(kernel_calls, Ordering::Relaxed);
                    continue 'outer;
                }
                // Wait for VBlank (60Hz pacing). 100ms is a safety ceiling —
                // normally returns immediately because main thread signals
                // every frame and the worker takes ~200ms per DPC callback
                // (dominated by D3D spin-loops inside the guest DPC routine),
                // so signals are already queued by the time we wait. Measured
                // on Spider-Man: wait=0ms, build=1-3ms, guest DPC execution
                // ~200ms → effective 5 DPC/sec rate bounded by guest code
                // spin loops (same `[device+0x400700]` pattern as the one
                // already RET-patched for Blue Padded at 0x004BD48B — not
                // a transport-layer issue).
                if !vblank.wait_timeout(std::time::Duration::from_millis(100)) {
                    if crate::xbox::aot::veh::is_stop_requested() {
                        break 'outer;
                    }
                    continue 'outer;
                }
                // Guard: prevent VEH from injecting ISR/DPC during our DPC fire
                crate::xbox::aot::veh_dpc::set_worker_dpc_active(true);
                // Use stable ESP for each DPC fire — prevents drift from crash
                // recovery or stdcall mismatch accumulating across frames.
                static DPC_STACK_BASE: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let base = DPC_STACK_BASE.load(std::sync::atomic::Ordering::Relaxed);
                if base == 0 {
                    DPC_STACK_BASE.store(ctx.guest.esp, std::sync::atomic::Ordering::Relaxed);
                }
                let stable_esp = if base != 0 { base } else { ctx.guest.esp };
                ctx.guest.esp = stable_esp;

                // H1 (2026-04-24): alternate DPC fires with Spider-Man frame-loop
                // entry. fe01e68 disabled forced game_main re-entry, leaving the
                // worker spinning on DPC fires only → game's per-frame tick never
                // runs → no 0x1760/INLINE_ARRAY → no visible geometry. Without
                // reintroducing the infinite-restart symptom, we drive the frame
                // loop at 0x002A_4BB0 on every other VBlank (after a 5-tick DPC
                // warmup so captured ISR/DPC state stabilises first).
                static H1_TICK: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let tick = H1_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let is_spiderman = start_routine == 0x002A_9BC9;
                let valid_guest_ptr = |p: u32| (0x1000..0x2000_0000).contains(&p);
                let frame_ctx_raw = memory.read_u32(0x004B_C630);
                let frame_ctx_ptr = frame_ctx_raw & !1;
                let global_scene_mgr = memory.read_u32(0x003F_5BEC) & !1;
                let frame_scene_mgr = if valid_guest_ptr(frame_ctx_ptr) {
                    memory.read_u32(frame_ctx_ptr + 0x18) & !1
                } else {
                    0
                };
                let engine_scene_mgr = memory.read_u32(0x004B_C614) & !1;
                let global_scene_root = if valid_guest_ptr(global_scene_mgr) {
                    memory.read_u32(global_scene_mgr + 0x28) & !1
                } else {
                    0
                };
                let frame_scene_root = if valid_guest_ptr(frame_scene_mgr) {
                    memory.read_u32(frame_scene_mgr + 0x28) & !1
                } else {
                    0
                };
                let engine_scene_root = if valid_guest_ptr(engine_scene_mgr) {
                    memory.read_u32(engine_scene_mgr + 0x28) & !1
                } else {
                    0
                };
                let (scene_mgr, scene_root, scene_source) = if valid_guest_ptr(frame_scene_root) {
                    (frame_scene_mgr, frame_scene_root, "frame")
                } else if valid_guest_ptr(engine_scene_root) {
                    (engine_scene_mgr, engine_scene_root, "engine")
                } else {
                    (global_scene_mgr, global_scene_root, "global")
                };
                let xgraph_root = memory.read_u32(0x004C_06B8);
                let scene_root_ready = valid_guest_ptr(scene_root);
                let do_frame_loop = is_spiderman
                    && tick > 5
                    && (tick % 2 == 0)
                    && valid_guest_ptr(frame_ctx_ptr)
                    && valid_guest_ptr(scene_mgr)
                    && scene_root_ready;
                let queued_dpc_pending = crate::xbox::aot::veh_dpc::is_dpc_pending();

                if is_spiderman
                    && tick > 5
                    && (tick % 2 == 0)
                    && valid_guest_ptr(frame_ctx_ptr)
                    && valid_guest_ptr(scene_mgr)
                    && !scene_root_ready
                {
                    static H1_ROOT_WAIT_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = H1_ROOT_WAIT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 20 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[H1-FRAME-WAIT] #{} tick={} frame=0x{:08X} scene=0x{:08X} scene_source={} scene+28=0x{:08X} frame_scene=0x{:08X}/root=0x{:08X} engine_scene=0x{:08X}/root=0x{:08X} global_scene=0x{:08X}/root=0x{:08X} xgraph=0x{:08X}",
                            n,
                            tick,
                            frame_ctx_ptr,
                            scene_mgr,
                            scene_source,
                            scene_root,
                            frame_scene_mgr,
                            frame_scene_root,
                            engine_scene_mgr,
                            engine_scene_root,
                            global_scene_mgr,
                            global_scene_root,
                            xgraph_root
                        ));
                    }
                }

                // Build DPC call frame and enter guest code
                if do_frame_loop {
                    // H1 iter-2 (2026-04-24): iter-1 entered 0x002A4BB0 20 times
                    // but produced 0 VS-SLOT-FORMAT writes — the frame-loop's
                    // prolog (quit-flag check + VBlank poll) short-circuits before
                    // reaching vertex submission. Per spider-man.md:186 call chain:
                    //   0x002A4BB0 → poll quit, poll VBlank → 0x000D6C00 (frame update)
                    //     → 0x000F7390 (scene tick) / 0x000F710C (scene render)
                    // Skip the prolog, enter 0x000D6C00 directly where per-frame
                    // draw dispatch lives.
                    let entry_addr = 0x000D_6C00u32;
                    static H1_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let ln = H1_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if ln < 20 || ln % 100 == 0 {
                        debug_log(&format!(
                            "[H1-FRAME] Entering Spider-Man frame_update @ 0x{:08X} (tick={}, #{})",
                            entry_addr, tick, ln
                        ));
                    }
                    if ln < 20 || ln.is_power_of_two() || ln % 100 == 0 {
                        let canon = |p: u32| -> u32 {
                            if (0x8000_0000..0xA000_0000).contains(&p) {
                                p & 0x1FFF_FFFF
                            } else {
                                p
                            }
                        };
                        let frame_18 = if valid_guest_ptr(frame_ctx_ptr) {
                            memory.read_u32(frame_ctx_ptr + 0x18)
                        } else {
                            0
                        };
                        let frame_20 = if valid_guest_ptr(frame_ctx_ptr) {
                            memory.read_u32(frame_ctx_ptr + 0x20)
                        } else {
                            0
                        };
                        let scene_phys = canon(scene_mgr);
                        let scene_valid = valid_guest_ptr(scene_phys);
                        let scene_a0 = if scene_valid {
                            memory.read_u32(scene_phys + 0xA0)
                        } else {
                            0
                        };
                        let scene_114 = if scene_valid {
                            memory.read_u32(scene_phys + 0x114)
                        } else {
                            0
                        };
                        let scene_118 = if scene_valid {
                            memory.read_u32(scene_phys + 0x118)
                        } else {
                            0
                        };
                        let scene_b25 = if scene_valid {
                            memory.read_u8(scene_phys + 0x25)
                        } else {
                            0
                        };
                        let scene_b183 = if scene_valid {
                            memory.read_u8(scene_phys + 0x183)
                        } else {
                            0
                        };
                        let scene_b186 = if scene_valid {
                            memory.read_u8(scene_phys + 0x186)
                        } else {
                            0
                        };
                        let scene_b18b = if scene_valid {
                            memory.read_u8(scene_phys + 0x18B)
                        } else {
                            0
                        };
                        let scene_b18c = if scene_valid {
                            memory.read_u8(scene_phys + 0x18C)
                        } else {
                            0
                        };
                        let ssp_phys = canon(scene_a0);
                        let (state_idx, state_arr, state_val) =
                            if ssp_phys >= 0x18 && valid_guest_ptr(ssp_phys) {
                                let idx = memory.read_u32(ssp_phys - 0x10);
                                let arr = memory.read_u32(ssp_phys - 0x14);
                                let arr_phys = canon(arr);
                                let val = if valid_guest_ptr(arr_phys) && idx < 0x1000 {
                                    memory.read_u32(arr_phys + idx * 4)
                                } else {
                                    0xDEAD_BEEF
                                };
                                (idx, arr, val)
                            } else {
                                (0xFFFF_FFFF, 0, 0xDEAD_BEEF)
                            };
                        let fsm = memory.read_u32(0x0072_6690);
                        let active_render_ctx = memory.read_u32(0x003F_10E8);
                        let render_list_a = memory.read_u32(0x004C_E274);
                        let render_list_b = memory.read_u32(0x004C_E278);
                        debug_log(&format!(
                            "[H1-FRAME-DIAG] #{} frame=0x{:08X} [18]=0x{:08X} [20]=0x{:08X} \
                             scene=0x{:08X} source={} +A0=0x{:08X} state(idx={},arr=0x{:08X},val=0x{:08X}) \
                             +114=0x{:08X} +118=0x{:08X} b25={:02X} b183={:02X} b186={:02X} b18b={:02X} b18c={:02X} \
                             fsm=0x{:08X} active_ctx=0x{:08X} lists=0x{:08X}/0x{:08X}",
                            ln,
                            frame_ctx_ptr,
                            frame_18,
                            frame_20,
                            scene_mgr,
                            scene_source,
                            scene_a0,
                            state_idx,
                            state_arr,
                            state_val,
                            scene_114,
                            scene_118,
                            scene_b25,
                            scene_b183,
                            scene_b186,
                            scene_b18b,
                            scene_b18c,
                            fsm,
                            active_render_ctx,
                            render_list_a,
                            render_list_b
                        ));
                    }
                    // VBlank tick flag so the frame loop's poll at 0x003F1D48 sees it
                    memory.write_u8(0x003F_1D48, 1);
                    // Frame counter gate: disasm of 0x000D6C40 shows:
                    //   mov eax, [esi+0x20]
                    //   test eax, eax; je 0xd6c5f   ; 0 → RENDER PATH
                    //   dec eax; jge 0xd6ce1         ; non-zero → skip render
                    // Write 0 to hit the render branch. Also wire scene_mgr
                    // at [frame_ctx+0x18] (used by call 0xf7390 at 0x000D6C38).
                    // Guest RAM is 512MB at 0x0-0x1FFFFFFF (plus mirror at 0x80000000+).
                    // Spider-Man objects live in VA bump zone 0x1XXXXXXX (>0x10000000).
                    if valid_guest_ptr(frame_ctx_ptr) {
                        memory.write_u32(frame_ctx_ptr + 0x20, 0); // 0 = take render branch
                        if valid_guest_ptr(scene_mgr) {
                            memory.write_u32(frame_ctx_ptr + 0x18, scene_mgr);
                        }
                    }
                    // GPU status poke — game spins at 0x00345780 waiting for
                    // (value & ~3) >= 0x20 at [0xFE820010]. Without this the
                    // frame loop stalls immediately.
                    memory.write_u32(0xFE82_0010, 0x1000);
                    memory.write_u32(0x0082_0010, 0x1000);
                    // Engine-NULL defensive: quit flag at [Engine+0x184].
                    let engine = memory.read_u32(0x004B_C614);
                    if engine == 0 {
                        memory.write_u8(0x184, 0);
                    } else if valid_guest_ptr(engine) {
                        memory.write_u8(engine + 0x184, 0);
                    }
                    // Fresh stack with sentinel return so frame-loop's RET hits
                    // RET_TO_ZERO cleanly instead of unwinding into our worker.
                    let new_esp = worker_stack_top - 0x2000;
                    ctx.guest.esp = new_esp;
                    ctx.guest.ebp = 0;
                    memory.write_u32(new_esp, 0); // RET_TO_ZERO sentinel
                                                  // 0x000D6C00 prologue: `mov esi, ecx` — thiscall, ECX = `this`.
                                                  // Pass frame_ctx as `this` so [esi+0x18]/[esi+0x20] resolve.
                    ctx.guest.ecx = frame_ctx_ptr;
                    guest_addr = entry_addr;
                } else if !is_spiderman && isr != 0 && !queued_dpc_pending {
                    if should_skip_zero_guest_code(&ctx, &memory, isr, "TimerISR") {
                        crate::xbox::aot::veh_dpc::set_worker_dpc_active(false);
                        continue 'outer;
                    }
                    crate::xbox::aot::veh_dpc::setup_vblank_shadow_regs();
                    let isr_context = crate::xbox::aot::veh_dpc::get_isr_context();
                    static ISR_FIRE_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = ISR_FIRE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 20 || (n % 100 == 0) {
                        debug_log(&format!(
                            "[TIMER-ISR] Firing ISR #{}: routine=0x{:08X} ctx=0x{:08X}",
                            n, isr, isr_context
                        ));
                    }
                    guest_addr = isr;
                    let new_esp = stable_esp.wrapping_sub(12);
                    ctx.guest.esp = new_esp;
                    memory.write_u32(new_esp, 0); // ret addr
                    memory.write_u32(new_esp + 4, 0); // PKINTERRUPT
                    memory.write_u32(new_esp + 8, isr_context); // ServiceContext
                } else if dpc != 0 {
                    let queued_dpc_object = if !is_spiderman && queued_dpc_pending {
                        take_pending_dpc_object(&memory, "TimerDPC")
                    } else {
                        None
                    };
                    let captured_dpc_object = crate::xbox::aot::veh_dpc::get_dpc_object();
                    let dpc_object = queued_dpc_object.unwrap_or(captured_dpc_object);
                    let mut dpc_routine = dpc;
                    let mut dpc_context = crate::xbox::aot::veh_dpc::get_dpc_context();
                    let mut stored_sys_arg1 = 0;
                    let mut stored_sys_arg2 = 0;
                    if let Some(obj) = queued_dpc_object.and_then(guest_data_addr) {
                        let queued_routine = memory.read_u32(obj + 0x0C);
                        if queued_routine != 0 {
                            dpc_routine = queued_routine;
                        }
                        dpc_context = memory.read_u32(obj + 0x10);
                        stored_sys_arg1 = memory.read_u32(obj + 0x14);
                        stored_sys_arg2 = memory.read_u32(obj + 0x18);
                    }
                    if should_skip_zero_guest_code(&ctx, &memory, dpc_routine, "TimerDPC") {
                        crate::xbox::aot::veh_dpc::set_worker_dpc_active(false);
                        continue 'outer;
                    }
                    static DPC_FIRE_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = DPC_FIRE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 20 || (n % 100 == 0) {
                        debug_log(&format!(
                            "[TIMER-DPC] Firing DPC #{}: routine=0x{:08X} ctx=0x{:08X} obj=0x{:08X} timer=0x{:08X}",
                            n, dpc_routine, dpc_context, dpc_object, timer_ptr
                        ));
                    }
                    let sys_arg1 = if stored_sys_arg1 != 0 {
                        stored_sys_arg1
                    } else {
                        crate::xbox::aot::veh_dpc::dpc_system_argument1(dpc_routine, dpc_context)
                    };

                    if is_spiderman {
                        crate::xbox::aot::veh_dpc::set_worker_dpc_active(true);
                        let _ = fire_guest_routine(
                            &mut ctx,
                            &memory,
                            &vblank,
                            dpc_routine,
                            &[dpc_object, dpc_context, sys_arg1, stored_sys_arg2],
                            "TimerDPC",
                        );
                        crate::xbox::aot::veh_dpc::set_worker_dpc_active(false);
                        live_counters
                            .dispatch_count
                            .store(ctx.dispatch_count, Ordering::Relaxed);
                        live_counters
                            .mmio_count
                            .store(ctx.mmio_count, Ordering::Relaxed);
                        live_counters
                            .pb_commands
                            .store(ctx.pb_commands, Ordering::Relaxed);
                        live_counters
                            .draw_calls
                            .store(ctx.draw_calls, Ordering::Relaxed);
                        live_counters
                            .kernel_calls
                            .store(kernel_calls, Ordering::Relaxed);
                        if phase > 1 {
                            phase = 1;
                        }
                        continue 'outer;
                    }

                    guest_addr = dpc_routine;
                    let new_esp = stable_esp.wrapping_sub(20);
                    ctx.guest.esp = new_esp;
                    memory.write_u32(new_esp, 0); // ret addr → RET_TO_ZERO
                    memory.write_u32(new_esp + 4, dpc_object); // PKDPC
                    memory.write_u32(new_esp + 8, dpc_context); // DeferredContext
                    memory.write_u32(new_esp + 12, sys_arg1); // SystemArgument1
                    memory.write_u32(new_esp + 16, stored_sys_arg2); // SystemArgument2
                } else if isr != 0 {
                    if should_skip_zero_guest_code(&ctx, &memory, isr, "TimerISR") {
                        crate::xbox::aot::veh_dpc::set_worker_dpc_active(false);
                        continue 'outer;
                    }
                    crate::xbox::aot::veh_dpc::setup_vblank_shadow_regs();
                    let isr_context = crate::xbox::aot::veh_dpc::get_isr_context();
                    guest_addr = isr;
                    let new_esp = ctx.guest.esp.wrapping_sub(12);
                    ctx.guest.esp = new_esp;
                    memory.write_u32(new_esp, 0); // ret addr
                    memory.write_u32(new_esp + 4, 0); // PKINTERRUPT
                    memory.write_u32(new_esp + 8, isr_context); // ServiceContext
                } else {
                    continue 'outer;
                }
                // Update counters. Don't let `phase += 1` at bottom push us into
                // phase-2 force-entry — stay looping the DPC-fire path so the
                // 60Hz VBlank heartbeat keeps ticking the game's frame timer.
                // guest_addr is already set to dpc/isr; the dispatch code below
                // (outside this `if guest_addr == 0` block) will enter guest
                // code at that address.
                live_counters
                    .dispatch_count
                    .store(ctx.dispatch_count, Ordering::Relaxed);
                live_counters
                    .mmio_count
                    .store(ctx.mmio_count, Ordering::Relaxed);
                live_counters
                    .pb_commands
                    .store(ctx.pb_commands, Ordering::Relaxed);
                live_counters
                    .draw_calls
                    .store(ctx.draw_calls, Ordering::Relaxed);
                live_counters
                    .kernel_calls
                    .store(kernel_calls, Ordering::Relaxed);
                // Keep phase at 0 (or 1 if first fire) so we stay in the DPC-
                // firing branch on next iteration.
                if phase > 1 {
                    phase = 1;
                }
                // Fall through to the guest-dispatch code below (line ~2318).
            } else {
                phase += 1;
            }

            // If the organic path above selected a DPC/ISR/frame entry, skip
            // the older pre-phase injector below. Falling through used to
            // overwrite `guest_addr = dpc` with `guest_addr = isr`, so every
            // tick only queued a DPC and never executed the captured DPC body.
            if guest_addr == 0 {
                // Check for stop request
                if crate::xbox::aot::veh::is_stop_requested() {
                    debug_log("AOT worker: stop requested, exiting");
                    break 'outer;
                }

                // Phase 2+: Re-enter game's own frame loop instead of synthetic frames.
                // The game's frame loop at 0x002A4BB0 does:
                //   1. Check quit flag [0x4BC614]+0x184
                //   2. Poll VBlank [0x3F1D48]
                //   3. Call frame update 0x000D6C00
                //   4. Loop
                // We re-enter it each time it hits RET_TO_ZERO so the game drives its own frames.
                if phase >= 2 {
                    // Wait for VBlank from main thread (60Hz pacing)
                    if !vblank.wait_timeout(std::time::Duration::from_millis(500)) {
                        if crate::xbox::aot::veh::is_stop_requested() {
                            break 'outer;
                        }
                        continue 'outer;
                    }
                    // GPU progress: write non-zero to NV2A GPU status addresses.
                    // The game polls [0xFE820010] in a tight loop at 0x00345780
                    // waiting for value & ~3 >= 0x20. This is a GPU notification area
                    // that the real GPU writes to after processing commands.
                    // Write 0x1000 (4096) so the >= 0x20 check passes immediately.
                    memory.write_u32(0xFE82_0010, 0x1000);
                    // Also try the mirrored low address and nearby offsets
                    memory.write_u32(0x0082_0010, 0x1000);
                    // Log verification (first 3 times only)
                    static GPU_POKE_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    if GPU_POKE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 3 {
                        let v1 = memory.read_u32(0xFE82_0010);
                        let v2 = memory.read_u32(0x0082_0010);
                        debug_log(&format!(
                            "[GPU-POKE] [0xFE820010]=0x{:08X} [0x00820010]=0x{:08X}",
                            v1, v2
                        ));
                    }

                    let valid_guest_ptr = |p: u32| (0x1000..0x2000_0000).contains(&p);
                    if start_routine == 0x002A9BC9 {
                        let frame_ctx_ptr = memory.read_u32(0x004B_C630) & !1;
                        let scene_mgr = memory.read_u32(0x003F_5BEC);
                        let scene_root = if valid_guest_ptr(scene_mgr) {
                            memory.read_u32(scene_mgr + 0x28)
                        } else {
                            0
                        };
                        if !valid_guest_ptr(frame_ctx_ptr)
                            || !valid_guest_ptr(scene_mgr)
                            || !valid_guest_ptr(scene_root)
                        {
                            static FRAME_DEFER_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let n =
                                FRAME_DEFER_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n < 20 || n.is_power_of_two() {
                                debug_log(&format!(
                                "[FRAME-DEFER] Spider-Man frame loop skipped #{} frame=0x{:08X} scene=0x{:08X} scene+28=0x{:08X}",
                                n, frame_ctx_ptr, scene_mgr, scene_root
                            ));
                            }
                            continue 'outer;
                        }
                    }

                    // STARVE vtable dispatch: the frame update at 0x0D6C18 calls 0x44E40
                    // with ECX = [0x3F87A8] (Upd1). When Upd1 is NULL, ESI=0 and the
                    // function reads [0x10]..[0x14] from the zero page as array bounds.
                    // The game's CRT sets FS:[0x14] (ArbitraryUserPointer) to 0xFFFFFFFF,
                    // making the loop iterate 4GB of "objects". Fix: ensure [0x10]==[0x14]
                    // whenever Upd1/Upd2 are NULL so the vtable loop is a no-op.
                    if start_routine == 0x002A9BC9 {
                        // When Upd1/Upd2 are NULL (this=0), vtable iteration reads
                        // zero page TIB fields as array bounds:
                        //   0x44E40: [0x10] (FiberData) .. [0x14] (ArbitraryUserPointer)
                        //   0x3AC20: [0x08] (StackLimit) .. [0x0C] (SubSystemTib)
                        // Equalize each pair so begin==end → loop is always a no-op.
                        let upd1 = memory.read_u32(0x003F_87A8);
                        let upd2 = memory.read_u32(0x003F_7D90);
                        let engine = memory.read_u32(0x004B_C614);
                        // When Engine/Upd1/Upd2 are NULL, frame update reads zero page
                        // as object fields. Fix critical offsets:
                        //   [0x10]/[0x14]: vtable array bounds (Upd1 via 0x44E40)
                        //   [0x08]/[0x0C]: vtable array bounds (Upd2 via 0x3AC20)
                        //   [0x184]: quit flag (Engine via frame loop) — must be 0
                        if engine == 0 {
                            memory.write_u8(0x184, 0); // quit flag = 0 (don't quit)
                        }
                        if upd1 == 0 || upd2 == 0 {
                            // Fix pair [0x10]/[0x14]
                            let zp10 = memory.read_u32(0x10);
                            let zp14 = memory.read_u32(0x14);
                            if zp10 != zp14 {
                                memory.write_u32(0x14, zp10);
                            }
                            // Fix pair [0x08]/[0x0C]
                            let zp0c = memory.read_u32(0x0C);
                            let zp08 = memory.read_u32(0x08);
                            if zp08 != zp0c {
                                memory.write_u32(0x0C, zp08);
                            }
                            static ZP_FIX_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            if ZP_FIX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 3 {
                                debug_log(&format!(
                                "[STARVE] NULL ptrs: Upd1=0x{:08X} Upd2=0x{:08X} Eng=0x{:08X} — zp fixed",
                                upd1, upd2, engine
                            ));
                            }
                        }
                        if upd1 != 0 && upd1 < 0x1000_0000 {
                            let arr_start = memory.read_u32(upd1 + 0x10);
                            let arr_end = memory.read_u32(upd1 + 0x14);
                            if arr_start != arr_end
                                && !crate::xbox::aot::oovpa::has_create_device_fired()
                            {
                                memory.write_u32(upd1 + 0x10, 0);
                                memory.write_u32(upd1 + 0x14, 0);
                                static STARVE_LOG: std::sync::atomic::AtomicU32 =
                                    std::sync::atomic::AtomicU32::new(0);
                                if STARVE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 5
                                {
                                    debug_log(&format!(
                                    "[STARVE] Upd1=0x{:08X} array [{:#X}..{:#X}] zeroed (pre-CreateDevice)",
                                    upd1, arr_start, arr_end
                                ));
                                }
                            }
                        }
                        // Upd2 non-null starve (pre-CreateDevice only)
                        if upd2 != 0
                            && upd2 < 0x1000_0000
                            && !crate::xbox::aot::oovpa::has_create_device_fired()
                        {
                            let arr_start = memory.read_u32(upd2 + 0x10);
                            let arr_end = memory.read_u32(upd2 + 0x14);
                            if arr_start != arr_end {
                                memory.write_u32(upd2 + 0x10, 0);
                                memory.write_u32(upd2 + 0x14, 0);
                            }
                        }
                    }
                    // Re-enter the game's frame loop (game-specific).
                    // Set guest_addr even if not AOT-compiled — the hybrid interpreter
                    // at line 1682 handles addresses not in addr_hash.
                    let frame_loop = if start_routine == 0x0003_E537 {
                        0x0001_2706u32 // Doom: persistent loop label after setup
                    } else {
                        0x002A_4BB0u32 // Spider-Man: frame loop
                    };
                    guest_addr = frame_loop;
                    // Doom: reset stack for D_DoomLoop (sub esp,0xC08 + push esi).
                    if start_routine == 0x0003_E537 {
                        let doom_entry_esp = worker_stack_top - 0x2000;
                        ctx.guest.esp = doom_entry_esp - 0xC08 - 4;
                        memory.write_u32(ctx.guest.esp, 0); // saved ESI
                        memory.write_u32(ctx.guest.esp + 0xC08, memory.read_u32(0x000D_F5B0));
                        memory.write_u32(ctx.guest.esp + 0xC0C, 0); // return sentinel
                    }
                    // Spider-Man: clean stack for frame loop entry
                    if start_routine == 0x002A9BC9 {
                        let new_esp = worker_stack_top - 0x2000;
                        ctx.guest.esp = new_esp;
                        ctx.guest.ebp = 0;
                        memory.write_u32(new_esp, 0); // ret addr → RET_TO_ZERO sentinel
                    }
                    // Update live counters
                    live_counters
                        .dispatch_count
                        .store(ctx.dispatch_count, Ordering::Relaxed);
                    live_counters
                        .mmio_count
                        .store(ctx.mmio_count, Ordering::Relaxed);
                    live_counters
                        .pb_commands
                        .store(ctx.pb_commands, Ordering::Relaxed);
                    live_counters
                        .draw_calls
                        .store(ctx.draw_calls, Ordering::Relaxed);
                    live_counters
                        .kernel_calls
                        .store(kernel_calls, Ordering::Relaxed);
                    continue 'outer;
                }

                // Pre-phase 2: try ISR/DPC injection (legacy path)
                // Always wait for VBlank before injecting — prevents DPC spin loops
                // where a DPC routine runs, returns, and immediately re-fires.
                if !vblank.wait_timeout(std::time::Duration::from_millis(100)) {
                    if phase <= 5 {
                        debug_log(&format!("AOT worker: waiting for VBlank (phase={})", phase));
                    }
                    continue 'outer;
                }
                let (isr, dpc) = crate::xbox::aot::veh_dpc::get_captured_routines();
                if isr == 0 && dpc == 0 {
                    continue 'outer;
                }
                if crate::xbox::aot::veh_dpc::suppress_guest_isr_dpc_injection() {
                    let _ = take_pending_dpc_object(&memory, "legacy-suppress");
                    static SUPPRESS_LEGACY_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = SUPPRESS_LEGACY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 10 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[DPC-SUPPRESS] legacy pre-phase ISR/DPC injection skipped #{} isr=0x{:08X} dpc=0x{:08X} phase={}",
                            n, isr, dpc, phase
                        ));
                    }
                    continue 'outer;
                }

                if isr != 0 {
                    // ISR captured — inject VBlank ISR
                    crate::xbox::aot::nv2a::shadow_write(
                        0x0100,
                        crate::xbox::aot::nv2a::shadow_read(0x0100) | 0x0100_1000,
                    );
                    crate::xbox::aot::nv2a::shadow_write(
                        0x0140,
                        crate::xbox::aot::nv2a::shadow_read(0x0140) | 0x0100_1001,
                    );
                    crate::xbox::aot::nv2a::shadow_write(
                        0x60_0100,
                        crate::xbox::aot::nv2a::shadow_read(0x60_0100) | 0x01,
                    );
                    crate::xbox::aot::nv2a::shadow_write(
                        0x40_0100,
                        crate::xbox::aot::nv2a::shadow_read(0x40_0100) | 0x01,
                    );
                    crate::xbox::aot::nv2a::shadow_write(
                        0x0C,
                        crate::xbox::aot::nv2a::shadow_read(0x0C) | 0x03,
                    );
                    crate::xbox::aot::nv2a::shadow_write(
                        0x10,
                        crate::xbox::aot::nv2a::shadow_read(0x10) | 0x8000_0003,
                    );

                    let isr_context = crate::xbox::aot::veh_dpc::get_isr_context();
                    if phase <= 5 || (phase % 1000 == 0) {
                        debug_log(&format!(
                            "AOT worker: ISR injection #{} at 0x{:08X} ctx=0x{:08X} (dispatches={})",
                            phase, isr, isr_context, ctx.dispatch_count
                        ));
                    }
                    if should_skip_zero_guest_code(&ctx, &memory, isr, "LegacyISR") {
                        continue 'outer;
                    }
                    guest_addr = isr;
                    ctx.guest.esp -= 12;
                    memory.write_u32(ctx.guest.esp, 0);
                    memory.write_u32(ctx.guest.esp + 4, 0);
                    memory.write_u32(ctx.guest.esp + 8, isr_context);
                } else if dpc != 0 {
                    if phase <= 5 || (phase % 1000 == 0) {
                        debug_log(&format!(
                            "AOT worker: DPC injection #{} at 0x{:08X} (dispatches={}, no ISR yet)",
                            phase, dpc, ctx.dispatch_count
                        ));
                    }
                    let dpc_context = crate::xbox::aot::veh_dpc::get_dpc_context();
                    let dpc_object = crate::xbox::aot::veh_dpc::get_dpc_object();
                    if should_skip_zero_guest_code(&ctx, &memory, dpc, "LegacyDPC") {
                        continue 'outer;
                    }
                    guest_addr = dpc;
                    ctx.guest.esp -= 20;
                    memory.write_u32(ctx.guest.esp, 0);
                    memory.write_u32(ctx.guest.esp + 4, dpc_object);
                    memory.write_u32(ctx.guest.esp + 8, dpc_context);
                    let sys_arg1 =
                        crate::xbox::aot::veh_dpc::dpc_system_argument1(dpc, dpc_context);
                    memory.write_u32(ctx.guest.esp + 12, sys_arg1);
                    memory.write_u32(ctx.guest.esp + 16, 0);
                } else {
                    continue 'outer; // retry next VBlank
                }
            }
        }

        if ctx.dispatch_count >= max_dispatches {
            debug_log(&format!(
                "AOT worker: hit max dispatches {}",
                max_dispatches
            ));
            break 'outer;
        }

        let host_offset = match ctx.addr_hash.lookup(guest_addr) {
            Some(off) => off,
            None => {
                // HYBRID DISPATCH (RPCS3-style): AOT can't handle this address.
                // Fall back to micro-interpreter. Run until we hit an address
                // that IS in the AOT hash table, then switch back to AOT.
                let normalized = normalize_guest_addr(guest_addr);
                let in_code = is_guest_code_addr(&ctx, guest_addr);
                if !in_code && guest_addr < 0xFFFF_0000 {
                    debug_log(&format!(
                        "[HYBRID] refusing non-code AOT miss 0x{:08X} (norm=0x{:08X}) esp=0x{:08X}",
                        guest_addr, normalized, ctx.guest.esp
                    ));
                    guest_addr = 0;
                    continue 'outer;
                }

                use crate::xbox::aot::micro_interp::{self, InterpResult, X86Regs};
                static INTERP_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = INTERP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if n < 50 {
                    debug_log(&format!(
                        "[HYBRID] AOT miss at 0x{:08X} — entering interpreter (#{}) esp=0x{:08X}",
                        guest_addr, n, ctx.guest.esp
                    ));
                }

                // Build interpreter register state from guest state
                let mut iregs = X86Regs::new();
                iregs.esp = ctx.guest.esp;
                iregs.ebp = ctx.guest.ebp;
                iregs.eax = ctx.guest.eax;
                iregs.ecx = ctx.guest.ecx;
                iregs.edx = ctx.guest.edx;
                iregs.ebx = ctx.guest.ebx;
                iregs.esi = ctx.guest.esi;
                iregs.edi = ctx.guest.edi;
                iregs.eip = guest_addr;

                // OOVPA hooks that should be routed to HLE
                let oovpa_addrs: Vec<u32> = ctx
                    .trap_table
                    .iter()
                    .filter(|t| {
                        t.trap_type == crate::xbox::aot::runtime::TrapType::System
                            || t.guest_addr >= 0xFFFF_0000
                    })
                    .map(|t| t.guest_addr)
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();

                // Run interpreter until it reaches an AOT-compiled address
                let max_interp = 100_000u64; // 100K instructions max per fallback
                let mut interp_count = 0u64;
                loop {
                    let entry = iregs.eip;

                    // Check if current EIP is in AOT hash — if so, switch back
                    if ctx.addr_hash.lookup(entry).is_some() && interp_count > 0 {
                        if n < 50 {
                            debug_log(&format!(
                                "[HYBRID] Interpreter reached AOT code at 0x{:08X} after {} insns — switching back",
                                entry, interp_count
                            ));
                        }
                        // Copy regs back and let AOT take over
                        ctx.guest.esp = iregs.esp;
                        ctx.guest.ebp = iregs.ebp;
                        ctx.guest.eax = iregs.eax;
                        ctx.guest.ecx = iregs.ecx;
                        ctx.guest.edx = iregs.edx;
                        ctx.guest.ebx = iregs.ebx;
                        ctx.guest.esi = iregs.esi;
                        ctx.guest.edi = iregs.edi;
                        guest_addr = entry;
                        break; // back to AOT dispatch
                    }

                    // Check for RET to zero
                    if entry == 0 {
                        if n < 50 {
                            debug_log(&format!(
                                "[HYBRID] Interpreter hit RET_TO_ZERO after {} insns",
                                interp_count
                            ));
                        }
                        ctx.guest.esp = iregs.esp;
                        guest_addr = 0;
                        break;
                    }

                    // Check for kernel thunk
                    if entry >= 0xFFFF_0000 {
                        let ordinal = entry & 0xFFFF;
                        let ret = memory.read_u32(iregs.esp);
                        let argc = crate::xbox::kernel::ordinals::arg_count(ordinal);
                        let mut args = [0u32; 12];
                        for i in 0..argc.min(12) {
                            args[i as usize] = memory.read_u32(iregs.esp + 4 + i * 4);
                        }
                        kernel_calls += 1;
                        if n < 20 && kernel_calls <= 300 {
                            let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                            debug_log(&format!(
                                "[HYBRID-KERN] #{} ord={} ({}) ret=0x{:08X}",
                                kernel_calls, ordinal, ord_name, ret
                            ));
                        }
                        // Dispatch kernel call via dispatch_inline (pre-read args, no ESP touch).
                        let kstate = unsafe {
                            &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState)
                        };
                        let mut args12 = [0u32; 12];
                        for i in 0..argc.min(12) {
                            args12[i as usize] = args[i as usize];
                        }
                        log_spidey_file_kcall(
                            &memory,
                            "hybrid-interp",
                            kernel_calls,
                            ordinal,
                            iregs.eip,
                            ret,
                            iregs.esp,
                            iregs.ebp,
                            &args12,
                        );
                        let kr = crate::xbox::kernel::dispatch_inline(
                            ordinal,
                            &args12,
                            &mut iregs.eax,
                            &mut iregs.edx,
                            &mut iregs.ecx,
                            kstate,
                            memory.base(),
                        );
                        match kr {
                            crate::xbox::kernel::KernelResult::SpawnThread {
                                entry,
                                start_routine,
                                context,
                                thread_handle,
                            } => {
                                spawn_child_thread_from_worker(
                                    &ctx,
                                    &memory,
                                    &vblank,
                                    entry,
                                    start_routine,
                                    context,
                                    thread_handle,
                                    "hybrid-interp",
                                );
                            }
                            crate::xbox::kernel::KernelResult::ThreadExit
                            | crate::xbox::kernel::KernelResult::Halt => {
                                debug_log(&format!(
                                    "[HYBRID-KERN] ord={} ({}) is no-return; terminating guest thread",
                                    ordinal,
                                    crate::xbox::kernel::ordinals::name(ordinal)
                                ));
                                ctx.guest.esp = iregs.esp;
                                guest_addr = 0;
                                break 'outer;
                            }
                            crate::xbox::kernel::KernelResult::QuickReboot => {
                                debug_log(&format!(
                                    "[HYBRID-KERN] ord={} ({}) requested quick reboot; terminating interpreter slice",
                                    ordinal,
                                    crate::xbox::kernel::ordinals::name(ordinal)
                                ));
                                ctx.guest.esp = iregs.esp;
                                guest_addr = 0;
                                break 'outer;
                            }
                            crate::xbox::kernel::KernelResult::Handled
                            | crate::xbox::kernel::KernelResult::NotHandled => {}
                        }
                        // Stdcall cleanup
                        iregs.esp += 4 + argc * 4;
                        iregs.eip = ret;
                        interp_count += 1;
                        continue;
                    }

                    // Run one instruction
                    let result = micro_interp::interpret(
                        &memory,
                        &mut iregs,
                        entry,
                        0,
                        1,
                        &[],
                        &ctx.exec_ranges,
                    );
                    interp_count += 1;

                    match result {
                        InterpResult::Unhandled { eip, mnemonic } => {
                            if n < 20 {
                                debug_log(&format!(
                                    "[HYBRID] Unhandled opcode at 0x{:08X}: {} after {} insns",
                                    eip, mnemonic, interp_count
                                ));
                            }
                            // Skip and try to continue
                            iregs.eip = eip + 1; // crude skip
                        }
                        InterpResult::Timeout => {
                            // Ran 1 instruction successfully, loop
                        }
                        InterpResult::ReturnedTo(addr) => {
                            iregs.eip = addr;
                        }
                        InterpResult::KernelCall { ordinal, ret_addr } => {
                            // Handled above via eip >= 0xFFFF0000 check
                            iregs.eip = ret_addr;
                        }
                        InterpResult::OovpaHook {
                            guest_addr: ga,
                            ret_addr: or,
                        } => {
                            if n < 20 || or == 0 || or >= 0xF000_0000 {
                                debug_log(&format!(
                                    "[HYBRID-OOVPA] Hook at 0x{:08X} ret=0x{:08X} ESP=0x{:08X}{}",
                                    ga,
                                    or,
                                    iregs.esp,
                                    if or == 0 {
                                        " *** BAD RET ADDR"
                                    } else if or >= 0xF000_0000 {
                                        " *** SUSPICIOUS ret addr"
                                    } else {
                                        ""
                                    }
                                ));
                            }
                            iregs.esp += 4;
                            iregs.eip = or;
                            iregs.eax = 0;
                        }
                        InterpResult::AccessViolation { eip, addr } => {
                            if n < 20 {
                                debug_log(&format!(
                                    "[HYBRID-AV] AV at 0x{:08X} addr=0x{:08X}",
                                    eip, addr
                                ));
                            }
                            iregs.eax = 0;
                            iregs.eip = eip + 4; // crude skip
                        }
                    }

                    if interp_count >= max_interp {
                        if n < 20 {
                            debug_log(&format!(
                                "[HYBRID] Interpreter timeout at 0x{:08X} after {}K insns",
                                iregs.eip,
                                interp_count / 1000
                            ));
                        }
                        ctx.guest.esp = iregs.esp;
                        guest_addr = 0;
                        break;
                    }
                }
                continue 'outer;
            }
        };

        let mut entry_host = unsafe { ctx.code_base.add(host_offset as usize) };
        ctx.guest.exit_reason = AOT_EXIT_RUNNING;

        // NOTE: Do NOT reset shadow stack between dispatches.
        // The GPS approach relies on shadow stack entries persisting across
        // kernel call exits (inner loop) and outer loop re-entries.
        // Only reset at worker start (line 1389).

        if ctx.dispatch_count < 5 {
            debug_log(&format!(
                "AOT worker PRE-enter: guest_addr=0x{:08X} host_offset=0x{:X} code_base={:p} ESP=0x{:08X} EBP=0x{:08X}",
                guest_addr, host_offset, ctx.code_base, ctx.guest.esp, ctx.guest.ebp
            ));
            // Dump first 48 bytes of compiled host code at entry point
            let dump_len =
                48usize.min((ctx.code_size as usize).saturating_sub(host_offset as usize));
            let host_bytes: Vec<u8> = (0..dump_len)
                .map(|i| unsafe { *ctx.code_base.add(host_offset as usize + i) })
                .collect();
            debug_log(&format!(
                "AOT worker HOST-CODE[0x{:X}..+{}]: {}",
                host_offset,
                dump_len,
                host_bytes
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            // Also show what guest bytes are at this address
            let guest_bytes: Vec<u8> = (0..16)
                .map(|i| memory.read_u8(guest_addr + i as u32))
                .collect();
            debug_log(&format!(
                "AOT worker GUEST-CODE[0x{:08X}..+16]: {}",
                guest_addr,
                guest_bytes
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            // MEMORY DUMP: if guest code is zeroed, dump .text to disk for diffing
            if guest_bytes.iter().all(|&b| b == 0) {
                static DUMP_ONCE: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                if !DUMP_ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                    let text_start: u32 = 0x00011000;
                    let text_size: u32 = 0x002DAD90; // .text section size
                    let mut dump = Vec::with_capacity(text_size as usize);
                    for off in 0..text_size {
                        dump.push(memory.read_u8(text_start + off));
                    }
                    let dump_path = r"./guest_text_dump.bin";
                    if let Err(e) = std::fs::write(dump_path, &dump) {
                        debug_log(&format!("[DUMP] Failed to write {}: {}", dump_path, e));
                    } else {
                        debug_log(&format!(
                            "[DUMP] Wrote {} bytes of guest .text to {}",
                            dump.len(),
                            dump_path
                        ));
                    }
                }
            }
        }

        // Inner loop: enter guest → handle kernel calls in safe Rust → re-enter
        let mut blocks_without_kcall: u64 = 0;
        loop {
            // Repair thunk table entries before each trampoline entry.
            // Guest compiled code writes directly through R15+offset, zeroing thunk entries.
            ctx.thunk_table.repair_all(&memory);
            // (AOT-TRACE removed — too verbose)
            // (GPS sentinel probe removed — was writing to shadow stack guard page)
            blocks_without_kcall += 1;

            // Child thread stuck detection: if >500K blocks without a kernel call,
            // the thread is likely in an infinite AOT loop (decompression or broken flags).
            // Log the guest EIP so we can diagnose what instruction it's stuck on.
            if is_child_thread && blocks_without_kcall == 500_000 {
                let ga = ctx.guest.exit_guest_addr;
                debug_log(&format!(
                    "[CHILD-{}-STUCK] {} blocks without kernel call! guest_eip=0x{:08X} esp=0x{:08X} \
                     eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X}",
                    stack_idx, blocks_without_kcall, ga, ctx.guest.esp,
                    ctx.guest.eax, ctx.guest.ecx, ctx.guest.edx, ctx.guest.ebx
                ));
            }
            if is_child_thread && blocks_without_kcall == 2_000_000 {
                let ga = ctx.guest.exit_guest_addr;
                debug_log(&format!(
                    "[CHILD-{}-STUCK] 2M blocks! guest_eip=0x{:08X} esp=0x{:08X} \
                     eax=0x{:08X} ecx=0x{:08X} — yielding to unblock other threads",
                    stack_idx, ga, ctx.guest.esp, ctx.guest.eax, ctx.guest.ecx
                ));
                std::thread::yield_now();
            }
            if is_child_thread && blocks_without_kcall % 5_000_000 == 0 && blocks_without_kcall > 0
            {
                let ga = ctx.guest.exit_guest_addr;
                debug_log(&format!(
                    "[CHILD-{}-STUCK] {}M blocks! guest_eip=0x{:08X} esp=0x{:08X}",
                    stack_idx,
                    blocks_without_kcall / 1_000_000,
                    ga,
                    ctx.guest.esp
                ));
                std::thread::yield_now();
            }

            let guest_prof =
                crate::xbox::profiler::start(crate::xbox::profiler::CpuPhase::GuestExec);
            let exit_reason =
                unsafe { crate::xbox::aot::trampoline::enter_guest(&mut ctx, entry_host) };
            crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::GuestExec, guest_prof);

            // Per-thread diagnostic: only log non-kernel exits (RET_TO_ZERO, UNRESOLVED)
            if exit_reason != AOT_EXIT_KERNEL_CALL {
                static MAIN_TRACE: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = MAIN_TRACE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 100 {
                    let ga = ctx.guest.exit_guest_addr;
                    let reason = match exit_reason {
                        AOT_EXIT_RET_TO_ZERO => "RET_TO_ZERO",
                        AOT_EXIT_UNRESOLVED => "UNRESOLVED",
                        _ => "OTHER",
                    };
                    debug_log(&format!(
                        "[MAIN-TRACE] #{} {} kernel={} guest={} esp=0x{:08X} eax=0x{:08X}",
                        n,
                        reason,
                        kernel_calls,
                        crate::xbox::logging::fmt_guest_addr(ga),
                        ctx.guest.esp,
                        ctx.guest.eax
                    ));
                }
            }
            // [L4-SHADOW] Check shadow stack AFTER trampoline exit
            // ctx.guest.shadow_stack_top was saved by trampoline exit (MOV [R13+0x40], R12)
            {
                static POST_ENTER: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = POST_ENTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 10 {
                    let r12_saved = ctx.guest.shadow_stack_top;
                    let shadow_base = ctx.shadow_stack.usable_base as u64;
                    let shadow_end = shadow_base + ctx.shadow_stack.usable_size as u64;
                    let at_r12 = if r12_saved >= shadow_base && r12_saved < shadow_end {
                        unsafe { *(r12_saved as *const u64) }
                    } else {
                        0xBAD0_0000
                    };
                    let code_base = ctx.code_base as u64;
                    let valid = at_r12 >= code_base && at_r12 < code_base + ctx.code_size as u64;
                    debug_log(&format!(
                        "[L4-SHADOW] POST-ENTER #{} R12=0x{:X} [R12]=0x{:X} valid={} exit={}",
                        n, r12_saved, at_r12, valid, exit_reason
                    ));
                }
            }

            if exit_reason != AOT_EXIT_KERNEL_CALL {
                // Clear worker DPC active flag on any non-kernel exit
                // (DPC returned via RET_TO_ZERO or other exit path)
                crate::xbox::aot::veh_dpc::set_worker_dpc_active(false);
                ctx.dispatch_count += 1;
                // Publish live stats for main thread display
                live_counters
                    .dispatch_count
                    .store(ctx.dispatch_count, Ordering::Relaxed);
                live_counters
                    .mmio_count
                    .store(ctx.mmio_count, Ordering::Relaxed);
                live_counters
                    .pb_commands
                    .store(ctx.pb_commands, Ordering::Relaxed);
                live_counters
                    .draw_calls
                    .store(ctx.draw_calls, Ordering::Relaxed);
                live_counters
                    .kernel_calls
                    .store(kernel_calls, Ordering::Relaxed);
                // Log thunk corruption if any entries were damaged this cycle
                {
                    let repaired = ctx.thunk_table.repair_all(&memory);
                    if repaired > 0 {
                        crate::rate_log!(
                            20,
                            "[THUNK] dispatch#{}: repaired {} entries after exit={} guest=0x{:08X}",
                            ctx.dispatch_count,
                            repaired,
                            exit_reason,
                            ctx.guest.exit_guest_addr
                        );
                    }
                }
                if ctx.dispatch_count <= 20 {
                    debug_log(&format!(
                        "AOT worker dispatch#{}: exit={} guest_addr={} esp=0x{:08X} ebp=0x{:08X} kernel_calls={}",
                        ctx.dispatch_count,
                        exit_reason,
                        crate::xbox::logging::fmt_guest_addr(ctx.guest.exit_guest_addr),
                        ctx.guest.esp,
                        ctx.guest.ebp,
                        kernel_calls
                    ));
                }
                // Handle non-kernel exit
                match exit_reason {
                    AOT_EXIT_RET_TO_ZERO => {
                        ret_to_zero_count += 1;
                        // Historical context: this park was added when "RET_TO_ZERO"
                        // meant "guest main() returned, Xbox would hang". Now that
                        // we fire VBlank DPC callbacks at 60Hz via the worker
                        // dispatch loop (after commit feacbdc's E2 guard landed
                        // and the phase-0-loop fix cleared the heartbeat), each
                        // DPC completion legitimately generates a RET_TO_ZERO —
                        // the DPC routine returned to its sentinel 0 retaddr.
                        // Parking after 4 of those kills the heartbeat.
                        //
                        // Keep the park as a catastrophic-stall guard (if we've
                        // had >1000 RET_TO_ZERO events AND zero forward kernel-
                        // call progress for a while, something is broken and
                        // spinning won't help). But bump the threshold high
                        // enough that a legit 60Hz DPC heartbeat never hits it
                        // during a reasonable run (3,600 events per minute at
                        // 60Hz, so 100,000 covers ~28 minutes of play).
                        if ret_to_zero_count > 100_000 && kernel_calls > 100 {
                            if ret_to_zero_count == 100_001 {
                                debug_log(&format!(
                                    "[PARK] Thread parked: RET_TO_ZERO #{} after {} kernel calls — catastrophic-stall threshold, parking.",
                                    ret_to_zero_count, kernel_calls
                                ));
                            }
                            loop {
                                std::thread::sleep(std::time::Duration::from_millis(1000));
                            }
                        }
                        // Check .text integrity at RET_TO_ZERO
                        let text_p1 = memory.read_u32(0x11000);
                        let text_p2 = memory.read_u32(0x20000);
                        let text_p3 = memory.read_u32(0x100000);
                        debug_log(&format!(
                            "AOT worker: RET_TO_ZERO at dispatch#{} kernel_calls={} esp=0x{:08X} eax=0x{:08X} .text=[0x{:08X},0x{:08X},0x{:08X}]",
                            ctx.dispatch_count, kernel_calls, ctx.guest.esp, ctx.guest.eax, text_p1, text_p2, text_p3
                        ));
                        crate::xbox::aot::jit::note_ret_to_zero(
                            ctx.dispatch_count,
                            kernel_calls,
                            ctx.guest.esp,
                            ctx.guest.eax,
                        );
                        // Pool state at RET_TO_ZERO
                        {
                            let small = memory.read_u32(0x3F8C30);
                            let medium = memory.read_u32(0x42EFB8);
                            let large = memory.read_u32(0x4B1DC0);
                            let zp_0 = memory.read_u32(0);
                            let zp_10 = memory.read_u32(0x10);
                            debug_log(&format!(
                                "[POOL-POST] small={} medium={} large={} zp[0]=0x{:08X} zp[0x10]=0x{:08X}",
                                small, medium, large, zp_0, zp_10
                            ));
                        }
                        // Dump stack around ESP
                        {
                            let esp = ctx.guest.esp;
                            debug_log(&format!(
                                "=== STACK at ESP=0x{:08X} (worker initial ESP was 0x{:08X}) ===",
                                esp,
                                worker_stack_top - 12
                            ));
                            // Show 16 dwords below and 16 above ESP
                            let start = esp.saturating_sub(16);
                            for j in 0..32u32 {
                                let addr = start + j * 4;
                                let val = memory.read_u32(addr);
                                let marker = if addr == esp { " <<< ESP" } else { "" };
                                debug_log(&format!("  [0x{:08X}] = 0x{:08X}{}", addr, val, marker));
                            }
                            debug_log("=== END STACK ===");
                        }
                        // Dump last 20 kernel calls for diagnostics
                        if !call_history.is_empty() {
                            debug_log(&format!(
                                "=== WORKER RET_TO_ZERO: last {} kernel calls ===",
                                call_history.len()
                            ));
                            for (i, (idx, ord, a, eax)) in call_history.iter().enumerate() {
                                let ord_name = crate::xbox::kernel::ordinals::name(*ord);
                                debug_log(&format!(
                                    "  [{}] #{} ord={} ({}) args=[0x{:X},0x{:X},0x{:X},0x{:X}] eax=0x{:08X}",
                                    i, idx, ord, ord_name, a[0], a[1], a[2], a[3], eax
                                ));
                            }
                            debug_log("=== END WORKER RET_TO_ZERO ===");
                        }
                        // LEGACY REINJECT: disabled. Spider-Man reinjection used
                        // hardcoded App address 0x01F00000 from pre-init scaffolding.
                        // With scaffolding removed, organic chain must reach game_main naturally.
                        {
                            // RET_TO_ZERO recovery: scan stack near ESP for a plausible code
                            // address. Prefer already-compiled addresses (in addr_hash) over
                            // rescue-emitting unknown addresses (which may be data, not code).
                            let esp = ctx.guest.esp;
                            let mut recovered = false;
                            // Scan stack for already-compiled code addresses (in addr_hash).
                            // Don't rescue-emit from stack — values may be data, not code.
                            // REQUIRE the candidate to be preceded by a CALL instruction —
                            // this rejects data pointers that happen to live in exec sections.
                            let allow_ret0_stack_scan =
                                ENABLE_RET0_STACK_SCAN_RECOVERY || start_routine == 0x0003_E537;
                            if allow_ret0_stack_scan {
                                for slot in 0..24u32 {
                                    let addr = esp + slot * 4;
                                    if addr >= 0x2000_0000 {
                                        break;
                                    } // 512MB RAM limit
                                    let candidate = memory.read_u32(addr);
                                    let norm = normalize_guest_addr(candidate);
                                    if norm >= 0x10000 {
                                        if let Some(_) = ctx.addr_hash.lookup(norm) {
                                            if !looks_like_ret_addr(&memory, norm) {
                                                crate::rate_log!(200,
                                                    "[RET0-RECOVER] Skip non-call-preceded 0x{:08X} at ESP+{}",
                                                    norm, slot * 4);
                                                continue;
                                            }
                                            ctx.guest.esp = addr + 4;
                                            guest_addr = norm;
                                            crate::rate_log!(50, "[RET0-RECOVER] Stack scan hit compiled 0x{:08X} at ESP+{} start=0x{:08X}", norm, slot * 4, start_routine);
                                            recovered = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if !recovered {
                                static GENERIC_REINJECT: std::sync::atomic::AtomicU32 =
                                    std::sync::atomic::AtomicU32::new(0);
                                let n = GENERIC_REINJECT
                                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if n < 5 {
                                    debug_log(&format!(
                                        "[GENERIC] Worker RET_TO_ZERO #{} after {} kernel calls. Blocking on VBlank.",
                                        n, kernel_calls
                                    ));
                                }
                                std::thread::sleep(std::time::Duration::from_millis(16));
                                guest_addr = 0;
                            }
                        }
                    }
                    AOT_EXIT_THREAD_EXIT => {
                        debug_log(&format!(
                            "AOT worker: ThreadExit at dispatch#{}",
                            ctx.dispatch_count
                        ));
                        guest_addr = 0;
                    }
                    AOT_EXIT_HALT => {
                        debug_log("AOT worker: Halt");
                        guest_addr = 0;
                    }
                    AOT_EXIT_SPAWN_THREAD => {
                        debug_log("AOT worker: sub-spawn (unexpected exit)");
                        guest_addr = normalize_guest_addr(ctx.guest.exit_guest_addr);
                    }
                    AOT_EXIT_SYSTEM_TRAP => {
                        debug_log(&format!(
                            "AOT worker: non-kernel trap at 0x{:08X} — stopping",
                            ctx.guest.exit_guest_addr
                        ));
                        // (minidump removed — was blocking dispatch loop)
                        guest_addr = 0;
                    }
                    AOT_EXIT_UNRESOLVED => {
                        guest_addr = normalize_guest_addr(ctx.guest.exit_guest_addr);

                        // Try rescue compilation — only for addresses in executable sections.
                        // Device objects, heap data, etc. must NOT be rescue-compiled as code.
                        let in_code = is_guest_code_addr(&ctx, guest_addr);
                        if in_code
                            && guest_addr > 0
                            && guest_addr < 0x2000_0000
                            && ctx.addr_hash.lookup(guest_addr).is_none()
                        {
                            if let Some(_host_off) = ctx.rescue_emit(guest_addr) {
                                continue; // re-enter dispatch loop with new code
                            }
                        }
                        if ctx.dispatch_count <= 20 {
                            debug_log(&format!(
                                "AOT worker: UNRESOLVED target={} esp=0x{:08X} eax=0x{:08X}",
                                crate::xbox::logging::fmt_guest_addr(guest_addr),
                                ctx.guest.esp,
                                ctx.guest.eax
                            ));
                            // UNRESOLVED CALLER TRACE (2026-04-22): dump guest
                            // stack top + R12 shadow top so we can back-trace
                            // which indirect CALL fed this bogus target.
                            // [ESP] is the return address pushed by the CALL
                            // that triggered this — subtract 5 (direct call) or
                            // scan backwards for call-site encoding.
                            let esp = ctx.guest.esp;
                            let ebp = ctx.guest.ebp;
                            debug_log(&format!(
                                "  UNRESOLVED-CTX: ebp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                                ebp, ctx.guest.eax, ctx.guest.ecx, ctx.guest.edx,
                                ctx.guest.ebx, ctx.guest.esi, ctx.guest.edi
                            ));
                            // R12 shadow stack top — what we predicted we'd
                            // return to from this call
                            let shadow_top_ptr = ctx.guest.shadow_stack_top as u64;
                            let shadow_base = ctx.shadow_stack.usable_base as u64;
                            let shadow_end = shadow_base + ctx.shadow_stack.usable_size as u64;
                            let shadow_top = if shadow_top_ptr >= shadow_base
                                && shadow_top_ptr + 8 <= shadow_end
                            {
                                unsafe { std::ptr::read_unaligned(shadow_top_ptr as *const u64) }
                            } else {
                                0
                            };
                            debug_log(&format!(
                                "  UNRESOLVED-CTX: R12=0x{:016X} R12[0]=0x{:016X} (host ret predicted)",
                                shadow_top_ptr, shadow_top
                            ));
                            // Guest stack top 16 dwords with hash-hit classification
                            debug_log(&format!(
                                "  UNRESOLVED-CTX: guest stack top @ ESP=0x{:08X}:",
                                esp
                            ));
                            for j in 0..16u32 {
                                let addr = esp.wrapping_add(j * 4);
                                if addr >= 0x2000_0000 {
                                    break;
                                }
                                let raw = memory.read_u32(addr);
                                let norm = normalize_guest_addr(raw);
                                let in_code = is_guest_code_addr(&ctx, raw);
                                let hash_hit = ctx.addr_hash.lookup(norm).is_some();
                                let kind = match (in_code, hash_hit) {
                                    (_, true) => "CODE-HIT",
                                    (true, false) => "code-miss",
                                    (false, _) => "data/ram",
                                };
                                debug_log(&format!(
                                    "    [ESP+{:02X}] raw=0x{:08X} norm=0x{:08X} {}",
                                    j * 4,
                                    raw,
                                    norm,
                                    kind
                                ));
                            }
                        }
                        // If target is bogus (outside all executable sections), simulate RET
                        // to skip the bad call and continue execution from the caller.
                        // Check exec_ranges, not a hardcoded threshold — addresses like
                        // 0x00D134B0 (ISR ServiceContext masked) are in RAM but not code.
                        let in_code = is_guest_code_addr(&ctx, guest_addr);
                        if !in_code && ctx.addr_hash.lookup(guest_addr).is_none() {
                            // Dump last 20 kernel calls on first bogus target
                            {
                                debug_log(&format!(
                                    "=== BOGUS TARGET DUMP: target={}, dispatch#{}, last {} kernel calls ===",
                                    crate::xbox::logging::fmt_guest_addr(guest_addr),
                                    ctx.dispatch_count,
                                    call_history.len()
                                ));
                                for (i, (idx, ord, a, eax)) in call_history.iter().enumerate() {
                                    let ord_name = crate::xbox::kernel::ordinals::name(*ord);
                                    debug_log(&format!(
                                        "  [{}] #{} ord={} ({}) args=[0x{:X},0x{:X},0x{:X},0x{:X}] eax=0x{:08X}",
                                        i, idx, ord, ord_name, a[0], a[1], a[2], a[3], eax
                                    ));
                                }
                                // Decisive diagnostic: for each stack dword, show normalized value,
                                // whether it's in exec_ranges, and whether addr_hash has it.
                                let esp = ctx.guest.esp;
                                debug_log(&format!(
                                    "  Stack at ESP=0x{:08X} (8 dwords with hash probe):",
                                    esp
                                ));
                                for j in 0..8u32 {
                                    let raw = memory.read_u32(esp + j * 4);
                                    let norm = normalize_guest_addr(raw);
                                    let in_code_j = is_guest_code_addr(&ctx, raw);
                                    let hash_hit = ctx.addr_hash.lookup(norm).is_some();
                                    debug_log(&format!(
                                        "    [ESP+{:02X}] raw=0x{:08X} norm=0x{:08X} in_code={} hash={}",
                                        j * 4, raw, norm, in_code_j, if hash_hit { "HIT" } else { "MISS" }
                                    ));
                                }
                                debug_log("=== END BOGUS TARGET DUMP ===");
                            }
                            // Scan stack for a valid code address to resume at.
                            // Simple [ESP] pop often hits 0 (sentinel). Scan deeper.
                            // REQUIRE a CALL precedes the candidate — otherwise we risk
                            // resuming at a data pointer that happens to live in an exec
                            // section (e.g., KeDpc struct pointers in D3D).
                            let esp = ctx.guest.esp;
                            let mut recovered = false;
                            for slot in 0..24u32 {
                                let addr = esp + slot * 4;
                                if addr >= 0x2000_0000 {
                                    break;
                                } // 512MB RAM limit
                                let candidate = memory.read_u32(addr);
                                let norm = normalize_guest_addr(candidate);
                                if norm >= 0x10000 {
                                    let candidate_in_code = is_guest_code_addr(&ctx, candidate);
                                    if candidate_in_code {
                                        if let Some(_) = ctx.addr_hash.lookup(norm) {
                                            if !looks_like_ret_addr(&memory, norm) {
                                                if ctx.dispatch_count <= 20 {
                                                    debug_log(&format!(
                                                        "AOT worker: skip non-call-preceded 0x{:08X} at ESP+{}",
                                                        norm, slot * 4
                                                    ));
                                                }
                                                continue;
                                            }
                                            ctx.guest.esp = addr + 4;
                                            guest_addr = norm;
                                            if ctx.dispatch_count <= 20 {
                                                debug_log(&format!(
                                                    "AOT worker: bogus target {}, stack scan recovered {} at ESP+{}",
                                                    crate::xbox::logging::fmt_guest_addr(ctx.guest.exit_guest_addr),
                                                    crate::xbox::logging::fmt_guest_addr(norm),
                                                    slot * 4
                                                ));
                                            }
                                            recovered = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if !recovered {
                                let ret_addr = memory.read_u32(esp);
                                let normalized = normalize_guest_addr(ret_addr);
                                // Sanity check: if [ESP] isn't a plausible code
                                // address (in_exec + hash + call-preceded), we're
                                // in a corrupt-stack infinite loop — bail out
                                // cleanly instead of spinning forever.
                                let in_exec = is_guest_code_addr(&ctx, ret_addr);
                                let in_hash = ctx.addr_hash.lookup(normalized).is_some();
                                let call_preceded =
                                    in_exec && in_hash && looks_like_ret_addr(&memory, normalized);
                                if !call_preceded {
                                    static BOGUS_GIVE_UP: std::sync::atomic::AtomicU32 =
                                        std::sync::atomic::AtomicU32::new(0);
                                    let n = BOGUS_GIVE_UP
                                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                    if n < 3 {
                                        debug_log(&format!(
                                            "AOT worker: no valid ret address on stack (ESP=0x{:08X} [ESP]=0x{:08X}) — exiting worker \
                                             (seen {} times; preventing infinite bogus-target loop)",
                                            esp, ret_addr, n + 1
                                        ));
                                    }
                                    guest_addr = 0;
                                    // Fall out of match; outer loop will exit because guest_addr==0
                                } else {
                                    ctx.guest.esp = esp + 4;
                                    guest_addr = normalized;
                                    if ctx.dispatch_count <= 20 {
                                        debug_log(&format!(
                                            "AOT worker: bogus target, simulating RET -> {}",
                                            crate::xbox::logging::fmt_guest_addr(ret_addr)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    AOT_EXIT_ERROR => {
                        let crash_addr = ctx.guest.exit_guest_addr;
                        // Stack overflow recovery (0xDEAD0001): don't count as crash,
                        // just let the reinject loop try again with fresh host stack.
                        if crash_addr == 0xDEAD_0001
                            && !INTERPRETER_FALLBACK.load(std::sync::atomic::Ordering::Relaxed)
                        {
                            INTERPRETER_FALLBACK.store(true, std::sync::atomic::Ordering::Relaxed);
                            debug_log(
                                "AOT worker: Stack overflow — switching to micro-interpreter",
                            );
                            // The trampoline overflows on this game's StartRoutine.
                            // Use the micro-interpreter instead — it runs on the dispatch
                            // loop's stack with no VEH, no trampoline, no host_stack switch.
                            use crate::xbox::aot::micro_interp::{self, InterpResult, X86Regs};
                            let mut iregs = X86Regs {
                                eax: ctx.guest.eax,
                                ecx: ctx.guest.ecx,
                                edx: ctx.guest.edx,
                                ebx: ctx.guest.ebx,
                                esp: worker_stack_top - 12,
                                ebp: 0,
                                esi: ctx.guest.esi,
                                edi: ctx.guest.edi,
                                eip: start_routine,
                                eflags: 0x202,
                                fpu_stack: [0.0; 8],
                                fpu_top: 0,
                                xmm: [[0u8; 16]; 8],
                            };
                            // Set up worker stack: [ESP]=0 (ret sentinel)
                            memory.write_u32(worker_stack_top - 12, 0);
                            let oovpa_addrs: Vec<u32> = ctx
                                .trap_table
                                .iter()
                                .filter(|t| {
                                    t.trap_type == crate::xbox::aot::runtime::TrapType::System
                                })
                                .map(|t| t.guest_addr)
                                .collect();
                            debug_log(&format!(
                                "AOT worker: Interpreting StartRoutine at 0x{:08X} (ESP=0x{:08X})",
                                start_routine, iregs.esp
                            ));
                            let result = micro_interp::interpret(
                                &memory,
                                &mut iregs,
                                start_routine,
                                0,
                                500_000,
                                &oovpa_addrs,
                                &ctx.exec_ranges,
                            );
                            // Sync regs back
                            ctx.guest.eax = iregs.eax;
                            ctx.guest.ecx = iregs.ecx;
                            ctx.guest.edx = iregs.edx;
                            ctx.guest.ebx = iregs.ebx;
                            ctx.guest.esp = iregs.esp;
                            ctx.guest.ebp = iregs.ebp;
                            ctx.guest.esi = iregs.esi;
                            ctx.guest.edi = iregs.edi;
                            debug_log(&format!(
                                "AOT worker: Interpreter result: EIP=0x{:08X} ESP=0x{:08X}",
                                iregs.eip, iregs.esp
                            ));
                            match result {
                                InterpResult::KernelCall { ordinal, ret_addr } => {
                                    debug_log(&format!(
                                        "AOT worker: Interpreter hit kernel ord={} ret=0x{:08X}",
                                        ordinal, ret_addr
                                    ));
                                    guest_addr = ret_addr;
                                    continue 'outer;
                                }
                                InterpResult::OovpaHook {
                                    guest_addr: ga,
                                    ret_addr,
                                } => {
                                    debug_log(&format!(
                                        "AOT worker: Interpreter hit OOVPA at 0x{:08X} ret=0x{:08X}",
                                        ga, ret_addr
                                    ));
                                    guest_addr = ga;
                                    continue 'outer;
                                }
                                _ => {
                                    guest_addr = 0;
                                    continue 'outer;
                                }
                            }
                        }
                        // Track repeated crashes at the same address
                        if crash_addr == last_error_addr {
                            error_repeat_count += 1;
                        } else {
                            last_error_addr = crash_addr;
                            error_repeat_count = 1;
                        }
                        if error_repeat_count <= 3 {
                            debug_log(&format!(
                                "AOT worker: ERROR at dispatch#{} addr=0x{:08X} eax=0x{:08X} ecx=0x{:08X} esp=0x{:08X}",
                                ctx.dispatch_count, crash_addr,
                                ctx.guest.eax, ctx.guest.ecx, ctx.guest.esp
                            ));
                        }
                        if error_repeat_count >= 3 {
                            // Same crash 3+ times — stop re-entering, set guest_addr=0
                            // to trigger phase advancement
                            if error_repeat_count == 3 {
                                debug_log(&format!(
                                    "AOT worker: REPEATED CRASH at 0x{:08X} ({} times) — stopping re-entry",
                                    crash_addr, error_repeat_count
                                ));
                            }
                            guest_addr = 0;
                        } else {
                            let ret_addr = memory.read_u32(ctx.guest.esp);
                            ctx.guest.esp += 4;
                            guest_addr = ret_addr;
                        }
                    }
                    _ => {
                        guest_addr = normalize_guest_addr(ctx.guest.exit_guest_addr);
                    }
                }
                break; // exit inner loop
            }

            // ---- AOT_EXIT_KERNEL_CALL: dispatch in safe Rust, re-enter ----
            blocks_without_kcall = 0;
            kernel_calls += 1;
            let ordinal = ctx.kernel_ordinal;
            let args = ctx.kernel_args;
            let esp_before = ctx.guest.esp;
            log_spiderman_scene38_change(&memory, &ctx, ordinal, kernel_calls, esp_before, "pre");
            log_spiderman_scene28_change(&memory, &ctx, ordinal, kernel_calls, esp_before, "pre");

            // Start guest-created threads only after the creator has executed at
            // least one more slice of guest code. This avoids a host scheduling
            // race where a tiny loader thread completes before the caller stores
            // the returned thread handle into its scene object.
            if !deferred_child_spawns.is_empty() {
                let spawns = std::mem::take(&mut deferred_child_spawns);
                for (entry, start_routine, context, thread_handle, source) in spawns {
                    spawn_child_thread_from_worker(
                        &ctx,
                        &memory,
                        &vblank,
                        entry,
                        start_routine,
                        context,
                        thread_handle,
                        source,
                    );
                }
            }

            // Watchpoint: [0x7D3D60] resource counter
            {
                static LAST_7D3D60: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let val = memory.read_u32(0x7D3D60);
                let prev = LAST_7D3D60.swap(val, Ordering::Relaxed);
                if val != prev {
                    let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                    debug_log(&format!(
                        "[WATCH-7D3D60] {} → {} at kcall #{} ord={} ({}) thread={}",
                        prev,
                        val,
                        kernel_calls,
                        ordinal,
                        ord_name,
                        if is_child_thread { "child" } else { "main" }
                    ));
                }
            }
            // 2026-04-20 RENDER PIPELINE WATCHPOINTS — see if game logic populates
            // the draw queue or if the render thread sees a different context.
            //   [0x4CE160 + 0x114] = default render context's first draw list head
            //   [0x4CE160 + 0x118] = default render context's second draw list head
            //   [0x3F10E8]         = ACTIVE render context pointer (changes on Push/Pop)
            {
                static LAST_DRAW_114: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static LAST_DRAW_118: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                static LAST_ACTIVE_CTX: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0xFFFF_FFFF);
                let v114 = memory.read_u32(0x004C_E274); // 0x4CE160 + 0x114
                let v118 = memory.read_u32(0x004C_E278); // 0x4CE160 + 0x118
                let active = memory.read_u32(0x003F_10E8);
                let p114 = LAST_DRAW_114.swap(v114, Ordering::Relaxed);
                let p118 = LAST_DRAW_118.swap(v118, Ordering::Relaxed);
                let pa = LAST_ACTIVE_CTX.swap(active, Ordering::Relaxed);
                if v114 != p114 {
                    debug_log(&format!(
                        "[WATCH-DRAW-114] 0x{:08X} → 0x{:08X} at kcall #{} (default render ctx, list1 head)",
                        p114, v114, kernel_calls
                    ));
                }
                if v118 != p118 {
                    debug_log(&format!(
                        "[WATCH-DRAW-118] 0x{:08X} → 0x{:08X} at kcall #{} (default render ctx, list2 head)",
                        p118, v118, kernel_calls
                    ));
                }
                if active != pa {
                    debug_log(&format!(
                        "[WATCH-ACTIVE-CTX] 0x{:08X} → 0x{:08X} at kcall #{} (active render ctx ptr)",
                        pa, active, kernel_calls
                    ));
                }
            }
            // Thread ordinal histogram: log top ordinals every 50K/500K calls
            if !is_child_thread {
                static MAIN_ORD_COUNTS: std::sync::LazyLock<std::sync::Mutex<[u32; 400]>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new([0u32; 400]));
                if let Ok(mut counts) = MAIN_ORD_COUNTS.lock() {
                    if (ordinal as usize) < 400 {
                        counts[ordinal as usize] += 1;
                    }
                    if kernel_calls % 500_000 == 0 {
                        let mut top: Vec<(u32, u32)> = counts
                            .iter()
                            .enumerate()
                            .filter(|(_, &c)| c > 0)
                            .map(|(i, &c)| (i as u32, c))
                            .collect();
                        top.sort_by(|a, b| b.1.cmp(&a.1));
                        let top5: Vec<String> = top
                            .iter()
                            .take(5)
                            .map(|(o, c)| {
                                format!("{}({})={}", crate::xbox::kernel::ordinals::name(*o), o, c)
                            })
                            .collect();
                        debug_log(&format!(
                            "[MAIN] kcall #{}: top ordinals: {}  esp=0x{:08X}",
                            kernel_calls,
                            top5.join(", "),
                            esp_before
                        ));
                        *counts = [0u32; 400];
                    }
                }
            }
            // One-shot: log NtYieldExecution caller EIP + registers to identify the spin loop
            if !is_child_thread && ordinal == crate::xbox::kernel::ordinals::NtYieldExecution {
                static NYE_COUNT: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = NYE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n == 0 || n == 10 || n == 100 || n == 1000 || n == 10000 || n == 100000 {
                    let ret_eip = memory.read_u32(esp_before);
                    // Check the spin-wait target: [0x83E03908] = [Scene(0x83E037F0) + 0x118]
                    let scene = memory.read_u32(0x003F5BEC);
                    let field_118 = if scene > 0 && scene < 0x2000_0000 {
                        memory.read_u32(scene + 0x118)
                    } else if scene >= 0x8000_0000 && scene < 0xA000_0000 {
                        memory.read_u32((scene & 0x1FFF_FFFF) + 0x118)
                    } else {
                        0xDEAD
                    };
                    let app = memory.read_u32(0x003F5EB0);
                    let app_flag = if app > 0 && app < 0x2000_0000 {
                        memory.read_u32(app + 4) as u8 // byte at +6 is within dword at +4
                    } else if app >= 0x8000_0000 && app < 0xA000_0000 {
                        memory.read_u32(((app & 0x1FFF_FFFF) + 4)) as u8
                    } else {
                        0xDE
                    };
                    let app6 = if app >= 0x8000_0000 && app < 0xA000_0000 {
                        let phys = app & 0x1FFF_FFFF;
                        unsafe { *((memory.base() as u64 + phys as u64 + 6) as *const u8) }
                    } else if app > 0 && app < 0x2000_0000 {
                        unsafe { *((memory.base() as u64 + app as u64 + 6) as *const u8) }
                    } else {
                        0xDE
                    };
                    // Read [scene+0xFC] and strings
                    let scene_phys = if scene >= 0x8000_0000 && scene < 0xA000_0000 {
                        scene & 0x1FFF_FFFF
                    } else {
                        scene
                    };
                    let scene_fc = if scene_phys < 0x2000_0000 {
                        memory.read_u32(scene_phys + 0xFC)
                    } else {
                        0
                    };
                    let read_str = |addr: u32| -> String {
                        let a = if addr >= 0x8000_0000 && addr < 0xA000_0000 {
                            addr & 0x1FFF_FFFF
                        } else {
                            addr
                        };
                        if a >= 0x2000_0000 {
                            return format!("(bad 0x{:08X})", addr);
                        }
                        let base = memory.base() as u64;
                        let mut s = Vec::new();
                        for i in 0..64u32 {
                            let b = unsafe { *((base + a as u64 + i as u64) as *const u8) };
                            if b == 0 {
                                break;
                            }
                            s.push(b as char);
                        }
                        s.into_iter().collect()
                    };
                    let str1 = read_str(0x003820A8);
                    let str2 = read_str(scene_fc);
                    debug_log(&format!(
                        "[NYE-DIAG] #{}: ret=0x{:08X} esp=0x{:08X} scene=0x{:08X} [+0x118]=0x{:08X} \
                         app=0x{:08X} [+6]=0x{:02X} edx=0x{:08X}\n  \
                         [+0xFC]=0x{:08X} cmp_str=\"{}\" scene_fc_str=\"{}\"",
                        n, ret_eip, esp_before, scene, field_118,
                        app, app6, ctx.guest.edx,
                        scene_fc, str1, str2
                    ));

                    // 2026-04-20 replicate sub_000F7CF0 to log the scene state stack value.
                    // The state-machine dispatcher at sub_000DD500 routes based on this:
                    //   dec eax; cmp eax, 5; ja default
                    //   case 0 (state=1) → sub_F7D30 → sub_F7450 → sets byte_186 → drawable scene
                    //   case 1-4 (state=2..5) → other state handlers
                    //   case 5 (state=6) → sub_F7D10 (advance index)
                    // If state isn't 1, the scene-enable chain never fires → no geometry.
                    // Also check this->byte_186 — if it's 1, render IS drawing (empty queue).
                    if scene_phys < 0x2000_0000 {
                        // 2026-04-20 dump key scene-object fields to see how much of
                        // the object is actually initialized. byte_186 is the draw
                        // gate; +0xA0 is the state-stack ptr; +0xFC is the filename
                        // we already print; +0x114/+0x118 are the draw-list heads.
                        let field_a0 = memory.read_u32(scene_phys + 0xA0);
                        let field_114 = memory.read_u32(scene_phys + 0x114);
                        let field_118 = memory.read_u32(scene_phys + 0x118);
                        let field_183 = unsafe {
                            *((memory.base() as u64 + scene_phys as u64 + 0x183) as *const u8)
                        };
                        let field_186 = unsafe {
                            *((memory.base() as u64 + scene_phys as u64 + 0x186) as *const u8)
                        };
                        let field_25 = unsafe {
                            *((memory.base() as u64 + scene_phys as u64 + 0x25) as *const u8)
                        };
                        debug_log(&format!(
                            "  [SCENE-FIELDS] +0xA0={:08X} +0x114={:08X} +0x118={:08X} \
                             byte[+0x25]={:02X} byte[+0x183]={:02X} byte[+0x186]={:02X}",
                            field_a0, field_114, field_118, field_25, field_183, field_186
                        ));
                        let state_stack_ptr = field_a0;
                        let ssp_phys =
                            if state_stack_ptr >= 0x8000_0000 && state_stack_ptr < 0xA000_0000 {
                                state_stack_ptr & 0x1FFF_FFFF
                            } else {
                                state_stack_ptr
                            };
                        if ssp_phys >= 0x18 && ssp_phys < 0x2000_0000 {
                            let idx = memory.read_u32(ssp_phys.wrapping_sub(0x10));
                            let arr = memory.read_u32(ssp_phys.wrapping_sub(0x14));
                            let arr_phys = if arr >= 0x8000_0000 && arr < 0xA000_0000 {
                                arr & 0x1FFF_FFFF
                            } else {
                                arr
                            };
                            let state_val = if arr_phys < 0x2000_0000 && idx < 0x1000 {
                                memory.read_u32(arr_phys + idx * 4)
                            } else {
                                0xDEAD_BEEF
                            };
                            // And the scene-drawable gate: scene->byte_186
                            let byte_186 = unsafe {
                                *((memory.base() as u64 + scene_phys as u64 + 0x186) as *const u8)
                            };
                            debug_log(&format!(
                                "  [SCENE-STATE] ssp=0x{:08X} idx={} arr=0x{:08X} state=0x{:08X} byte_186=0x{:02X}  \
                                 (state=1→draw-enable path, byte_186=1→drawing)",
                                state_stack_ptr, idx, arr, state_val, byte_186
                            ));
                        } else {
                            debug_log(&format!(
                                "  [SCENE-STATE] state_stack ptr invalid: 0x{:08X} (ssp_phys=0x{:08X})",
                                state_stack_ptr, ssp_phys
                            ));
                        }
                    }
                }
                // 2026-04-24: NYE-FORCE block REMOVED.
                //
                // Previously this force-zeroed [scene+0x118] every 50K
                // NYE calls to break the NtYieldExecution deadlock seen
                // when CHILD-2 got stuck in a decompression loop. In
                // practice the field holds pool-allocator poison
                // (0x0000DEAD = low word of 0xDEADBEEE heap-free tag)
                // because the scene worker thread at 0x0001C8C0 was
                // never spawned (gated on the VFS/stash init chain
                // that the SYNTH-FRAMECTX scaffolding was short-
                // circuiting).
                //
                // Also the bound check `scene_phys < 0x1000_0000` was
                // wrong — organic scene addresses land in 0x102xxxxx
                // (outside that range), so the force-zero never
                // actually ran in current builds.
                //
                // Removed alongside SYNTH-FRAMECTX in the same session
                // to let the organic init path run unimpeded.
            }
            if is_child_thread {
                // Log first 20 + every 10th child thread kernel call to trace the loop
                if kernel_calls <= 20 || (kernel_calls <= 200 && kernel_calls % 10 == 0) {
                    let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                    let ret_addr = memory.read_u32(esp_before);
                    debug_log(&format!(
                        "[CHILD-{}-TRACE] kcall #{} ord={} ({}) ret=0x{:08X} esp=0x{:08X} args=[0x{:X},0x{:X},0x{:X}]",
                        stack_idx, kernel_calls, ordinal, ord_name, ret_addr, esp_before,
                        args[0], args[1], args[2]
                    ));
                }
                static CHILD_ORD_COUNTS: std::sync::LazyLock<std::sync::Mutex<[u32; 400]>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new([0u32; 400]));
                if let Ok(mut counts) = CHILD_ORD_COUNTS.lock() {
                    if (ordinal as usize) < 400 {
                        counts[ordinal as usize] += 1;
                    }
                    if kernel_calls == 100 || kernel_calls % 50_000 == 0 {
                        let mut top: Vec<(u32, u32)> = counts
                            .iter()
                            .enumerate()
                            .filter(|(_, &c)| c > 0)
                            .map(|(i, &c)| (i as u32, c))
                            .collect();
                        top.sort_by(|a, b| b.1.cmp(&a.1));
                        let top5: Vec<String> = top
                            .iter()
                            .take(5)
                            .map(|(o, c)| {
                                format!("{}({})={}", crate::xbox::kernel::ordinals::name(*o), o, c)
                            })
                            .collect();
                        debug_log(&format!(
                            "[CHILD-{}] kcall #{}: top ordinals: {}  esp=0x{:08X}",
                            stack_idx,
                            kernel_calls,
                            top5.join(", "),
                            esp_before
                        ));
                        *counts = [0u32; 400];
                    }
                }
            }

            // Watchpoint: detect when [0x00C1FD74] changes to 0
            // Check AFTER kernel call (guest code ran between last exit and this entry)
            {
                let watch_addr = 0x00C1FD74u32;
                let watch_val = memory.read_u32(watch_addr);
                static LAST_WATCH: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0xDEAD);
                let prev = LAST_WATCH.swap(watch_val, std::sync::atomic::Ordering::Relaxed);
                if prev != 0xDEAD && prev != watch_val {
                    let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                    if (watch_val == 0 && prev != 0) || kernel_calls <= 5 {
                        debug_log(&format!(
                            "[WATCHPOINT] [0x{:08X}] 0x{:08X} -> 0x{:08X} at kernel call #{} ord={} ({}) esp=0x{:08X}",
                            watch_addr, prev, watch_val, kernel_calls, ordinal, ord_name, esp_before
                        ));
                        // Also dump nearby stack
                        if watch_val == 0 && prev != 0 {
                            for j in 0..8u32 {
                                let a = watch_addr - 16 + j * 4;
                                debug_log(&format!(
                                    "  [0x{:08X}] = 0x{:08X}{}",
                                    a,
                                    memory.read_u32(a),
                                    if a == watch_addr { " <<< WATCH" } else { "" }
                                ));
                            }
                        }
                    }
                }
            }

            // Record in rolling history (now includes esp_before)
            if call_history.len() >= 20 {
                call_history.remove(0);
            }
            call_history.push((
                kernel_calls,
                ordinal,
                [args[0], args[1], args[2], args[3]],
                ctx.guest.eax,
            ));

            // === OPTION C: Log non-yield kernel calls (game_main phase) ===
            if kernel_calls > 400
                && ordinal != crate::xbox::kernel::ordinals::NtYieldExecution
                && kernel_calls < 10_000
            {
                let ord_name = crate::xbox::kernel::ordinals::name(ordinal);
                let argc = crate::xbox::kernel::ordinals::arg_count(ordinal);
                debug_log(&format!(
                    "[GAME-MAIN] #{} ord={} ({}) argc={} args=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] eax=0x{:08X} esp=0x{:08X}",
                    kernel_calls, ordinal, ord_name, argc, args[0], args[1], args[2], args[3],
                    ctx.guest.eax, esp_before
                ));
                log_spidey_file_kcall(
                    &memory,
                    "aot",
                    kernel_calls,
                    ordinal,
                    ctx.guest.exit_guest_addr,
                    memory.read_u32(esp_before),
                    esp_before,
                    ctx.guest.ebp,
                    &args,
                );
                // MmQueryStatistics (ord 181): dump output buffer
                let ord_name_str = ord_name;
                if ord_name_str == "MmQueryStatistics" {
                    let ptr = args[0];
                    debug_log(&format!("  MmQS ptr=0x{:08X}", ptr));
                    if ptr > 0 && ptr < 0x1000_0000 {
                        let fields = [
                            "Length",
                            "TotalPhysPages",
                            "AvailPages",
                            "VirtCommitted",
                            "VirtReserved",
                            "CachePages",
                            "PoolPages",
                            "StackPages",
                            "ImagePages",
                        ];
                        for i in 0..9u32 {
                            debug_log(&format!(
                                "  MmQS [+0x{:02X}] {} = {}",
                                i * 4,
                                fields[i as usize],
                                memory.read_u32(ptr + i * 4)
                            ));
                        }
                    }
                }
                // KeDelayExecutionThread (ord 99): log interval
                if ord_name_str == "KeDelayExecutionThread" {
                    let interval_ptr = args[2];
                    debug_log(&format!("  KeDelay interval_ptr=0x{:08X}", interval_ptr));
                    if guest_i64_ptr_valid(interval_ptr) {
                        let lo = memory.read_u32(interval_ptr);
                        let hi = memory.read_u32(interval_ptr + 4);
                        debug_log(&format!(
                            "  KeDelay interval=0x{:08X}:{:08X} ({}us)",
                            hi,
                            lo,
                            (lo as i32) / -10
                        ));
                    }
                    // 2026-04-20: stdcall dispatch pops args, so [esp+0] is caller
                    // local. Scan 0..0x80 for slots whose value is a plausible guest
                    // code address (0x00010000..0x00380000 = XBE image range). These
                    // are saved ret_addrs up the call chain.
                    let mut chain = String::new();
                    static CALLER_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let cn = CALLER_LOG.fetch_add(1, Ordering::Relaxed);
                    if cn < 8 || cn % 200 == 0 {
                        for off in (0..0x80u32).step_by(4) {
                            let addr = esp_before.wrapping_add(off);
                            let v = memory.read_u32(addr);
                            if v >= 0x00010000 && v < 0x00400000 {
                                chain.push_str(&format!(" +{:02X}=0x{:08X}", off, v));
                            }
                        }
                        debug_log(&format!("  KeDelay-STACK-WALK #{}:{}", cn, chain));
                    }
                }
                // Also dump key game state addresses
                debug_log(&format!("  [0x7E15DC]=0x{:08X} [0x7E1630]=0x{:08X} [0x7E1C4C]=0x{:08X} [0x3F1D48]=0x{:08X}",
                    memory.read_u32(0x7E15DC), memory.read_u32(0x7E1630),
                    memory.read_u32(0x7E1C4C), memory.read_u32(0x3F1D48)
                ));

                // Post-title Spider-Man crash tracer. The current failing run
                // advances the menu FSM, then exits on bogus target 0x3DCCCCCD
                // immediately after kernel call #696. Dump the kernel boundary
                // stack while it is still intact so the RET/epilog source is
                // visible before the unresolved handler sees the already-popped
                // float sentinel.
                if kernel_calls >= 688 && kernel_calls <= 700 {
                    let pre = ctx.kernel_r14_pre_cleanup;
                    let post = esp_before;
                    let ret_or_arg0 = if pre < 0x2000_0000 {
                        memory.read_u32(pre)
                    } else {
                        0
                    };
                    debug_log(&format!(
                        "[SPIDEY-POSTSTART-KCALL] #{} ord={} ({}) is_call={} argc={} pre_esp=0x{:08X} post_esp=0x{:08X} \
                         stack0=0x{:08X} jmp_ret=0x{:08X} host_resume=0x{:016X} regs eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X}",
                        kernel_calls,
                        ordinal,
                        ord_name,
                        ctx.kernel_is_call,
                        ctx.kernel_arg_count,
                        pre,
                        post,
                        ret_or_arg0,
                        ctx.kernel_jmp_ret_addr,
                        ctx.kernel_host_resume,
                        ctx.guest.eax,
                        ctx.guest.ebx,
                        ctx.guest.ecx,
                        ctx.guest.edx,
                        ctx.guest.esi,
                        ctx.guest.edi,
                        ctx.guest.ebp
                    ));

                    let start = pre.saturating_sub(0x20);
                    for off in (0..0x80u32).step_by(4) {
                        let addr = start.wrapping_add(off);
                        if addr >= 0x2000_0000 {
                            break;
                        }
                        let v = memory.read_u32(addr);
                        let norm = normalize_guest_addr(v);
                        let kind = if ctx.addr_hash.lookup(norm).is_some() {
                            "hash"
                        } else if is_guest_code_addr(&ctx, norm) {
                            "code"
                        } else {
                            "data"
                        };
                        let mut marks = String::new();
                        if addr == pre {
                            marks.push_str(" <pre>");
                        }
                        if addr == post {
                            marks.push_str(" <post>");
                        }
                        if v == 0x3DCC_CCCD {
                            marks.push_str(" <3DCCCCCD>");
                        }
                        debug_log(&format!(
                            "  [SPIDEY-POSTSTART-STACK] [0x{:08X}] = 0x{:08X} norm=0x{:08X} {}{}",
                            addr, v, norm, kind, marks
                        ));
                    }
                }
            }

            // KeSetEvent spin diagnostic: deep stack dump to find caller chain
            if ordinal == crate::xbox::kernel::ordinals::KeSetEvent {
                static KSE_SPIN_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = KSE_SPIN_LOG.fetch_add(1, Ordering::Relaxed);
                if n == 0 || n == 10 || n == 100 || n == 1000 {
                    debug_log(&format!(
                        "[KSE-SPIN] #{} esp=0x{:08X} ebp=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebx=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
                        n, esp_before, ctx.guest.ebp, ctx.guest.esi, ctx.guest.edi, ctx.guest.ebx,
                        ctx.guest.eax, ctx.guest.ecx, ctx.guest.edx
                    ));
                    // Deep stack dump: 20 dwords
                    let mut stk = String::new();
                    for i in 0..20u32 {
                        let v = memory.read_u32(esp_before + i * 4);
                        stk.push_str(&format!(" {:08X}", v));
                        if i % 8 == 7 {
                            stk.push('\n');
                        }
                    }
                    debug_log(&format!("  stack @0x{:08X}:{}", esp_before, stk));
                    // Read device[0] and guest mem at NV2A MMIO addresses
                    // esi = DPC context = 0x303068. ebx = [esi] = [0x303068]
                    let esi_val = ctx.guest.esi;
                    let ebx_val = if esi_val < 0x1000_0000 {
                        memory.read_u32(esi_val)
                    } else {
                        0xDEAD
                    };
                    let dev_ptr = memory.read_u32(0x003038E0);
                    debug_log(&format!(
                        "  [esi=0x{:08X}]=0x{:08X} (ebx=0x{:08X}) dev_ptr=0x{:08X} mmio={}",
                        esi_val, ebx_val, ctx.guest.ebx, dev_ptr, ctx.mmio_count
                    ));
                }
            }
            let is_call = ctx.kernel_is_call;
            let jmp_ret_addr = ctx.kernel_jmp_ret_addr;
            let host_resume = ctx.kernel_host_resume;

            // Get kernel state
            let kstate_ptr = ctx.kernel_state as *mut crate::xbox::kernel::KernelState;
            let kstate = unsafe { &mut *kstate_ptr };

            let ke_delay_probe = if ordinal == crate::xbox::kernel::ordinals::KeDelayExecutionThread
                && crate::xbox::profiler::cpu_frame_prof_active()
            {
                Some((
                    find_ke_delay_caller(&ctx, &memory, esp_before),
                    args[2],
                    read_ke_delay_interval_100ns(&memory, args[2]),
                    args[1],
                    args[0] != 0,
                    std::time::Instant::now(),
                ))
            } else {
                None
            };

            // Dispatch kernel call in safe Rust (outside VEH!)
            let kr = crate::xbox::kernel::dispatch_inline(
                ordinal,
                &args,
                &mut ctx.guest.eax,
                &mut ctx.guest.edx,
                &mut ctx.guest.ecx,
                kstate,
                ctx.guest_mem_base,
            );
            if let Some((caller, interval_ptr, interval_100ns, wait_mode, alertable, start)) =
                ke_delay_probe
            {
                crate::xbox::profiler::record_ke_delay(
                    caller,
                    interval_ptr,
                    interval_100ns,
                    wait_mode,
                    alertable,
                    start,
                );
            }
            log_spiderman_scene38_change(&memory, &ctx, ordinal, kernel_calls, esp_before, "post");
            log_spiderman_scene28_change(&memory, &ctx, ordinal, kernel_calls, esp_before, "post");
            observe_spidey_scene_loader_latch(&memory, kstate, ordinal, kernel_calls, esp_before);

            // WATCHPOINT: check 7 critical globals after each kernel call
            // Log the FIRST time each global changes from 0 to non-zero
            {
                static WATCH_LOGGED: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let logged = WATCH_LOGGED.load(std::sync::atomic::Ordering::Relaxed);
                if logged < 7 {
                    // stop checking once all 7 found
                    let globals: [(u32, &str); 7] = [
                        (0x003F5EB0, "App"),
                        (0x003F5BEC, "Scene"),
                        (0x00726690, "Input"),
                        (0x003F87A8, "Upd1"),
                        (0x003F7D90, "Upd2"),
                        (0x004BC614, "Engine"),
                        (0x004BC630, "Frame"),
                    ];
                    for (addr, name) in &globals {
                        let val = memory.read_u32(*addr);
                        if val != 0 {
                            // Check if we already logged this one
                            static SEEN: std::sync::atomic::AtomicU64 =
                                std::sync::atomic::AtomicU64::new(0);
                            let bit = match *addr {
                                0x003F5EB0 => 0,
                                0x003F5BEC => 1,
                                0x00726690 => 2,
                                0x003F87A8 => 3,
                                0x003F7D90 => 4,
                                0x004BC614 => 5,
                                0x004BC630 => 6,
                                _ => 7,
                            };
                            let mask = 1u64 << bit;
                            let prev = SEEN.fetch_or(mask, std::sync::atomic::Ordering::Relaxed);
                            if prev & mask == 0 {
                                debug_log(&format!(
                                    "[WATCHPOINT] {} at [0x{:08X}] set to 0x{:08X} at kernel_call #{} (ord={} {})",
                                    name, addr, val, kernel_calls, ordinal,
                                    crate::xbox::kernel::ordinals::name(ordinal)
                                ));
                                WATCH_LOGGED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                    }
                }
            }

            // Publish live counters every 100 kernel calls (so HUD updates during long dispatches)
            if kernel_calls % 100 == 0 {
                live_counters
                    .dispatch_count
                    .store(ctx.dispatch_count, Ordering::Relaxed);
                live_counters
                    .mmio_count
                    .store(ctx.mmio_count, Ordering::Relaxed);
                live_counters
                    .pb_commands
                    .store(ctx.pb_commands, Ordering::Relaxed);
                live_counters
                    .draw_calls
                    .store(ctx.draw_calls, Ordering::Relaxed);
                live_counters
                    .kernel_calls
                    .store(kernel_calls, Ordering::Relaxed);
            }

            // Spider-Man's post-title menu/update loop can spend long stretches
            // in ordinary SDK/game HLE calls instead of Wait/Yield calls. If
            // VBlank delivery is tied only to wait-related kernel exits, the
            // DPC chain falls to a few ticks per minute and the "please wait"
            // scene never finishes. The helper below is already guarded by:
            //   - CRT boot completion,
            //   - main-worker-thread identity,
            //   - a 16 ms wall-clock throttle,
            //   - full guest-context save/restore.
            //
            // Calling it once at every safe kernel boundary gives the guest a
            // real scheduler heartbeat without reintroducing arbitrary VEH-time
            // ISR injection.
            try_fire_isr_dpc_preserving_thread_context(
                &mut ctx,
                &memory,
                &vblank,
                "kernel-boundary",
            );

            // After wait-related kernel calls, fire ISR→DPC from dispatch loop.
            // Real Xbox: VBlank ISR fires when thread wakes from sleep, DPC fires during scheduling.
            // This guarantees DPC delivery even when guest code has no INT3 boundaries between waits.
            {
                use crate::xbox::kernel::ordinals;
                if ordinal == ordinals::KeDelayExecutionThread
                    || ordinal == ordinals::KeWaitForSingleObject
                    || ordinal == ordinals::KeWaitForMultipleObjects
                    || ordinal == ordinals::NtWaitForSingleObject
                    || ordinal == ordinals::NtWaitForSingleObjectEx
                    || ordinal == ordinals::NtWaitForMultipleObjectsEx
                {
                    try_fire_isr_dpc_preserving_thread_context(
                        &mut ctx,
                        &memory,
                        &vblank,
                        crate::xbox::kernel::ordinals::name(ordinal),
                    );
                }
                // NtYieldExecution: fire DPCs paced at ~60Hz.
                // Real Xbox fires pending DPCs during thread scheduling.
                // NOTE: Do NOT fire VBlank from KeSetEvent — the VBlank ISR itself
                // calls KeSetEvent. Re-arming interrupts from KeSetEvent dispatch
                // creates an infinite loop (ISR W1C clears → KeSetEvent → VBlank
                // emulation re-sets PMC_INTR → ISR loops).
                if ordinal == ordinals::NtYieldExecution {
                    static NYE_DPC_CTR: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let nye_n = NYE_DPC_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if nye_n < 5 || nye_n == 100 || nye_n == 1000 || nye_n == 10000 {
                        debug_log(&format!(
                            "[NYE-VBLANK] #{} about to fire ISR/DPC from NtYieldExecution",
                            nye_n
                        ));
                    }
                    // HW watchpoint arm: by default keep the older utility
                    // watch. For origin_z loader diagnostics, repoint it at
                    // a scene flag without changing guest behavior.
                    if nye_n == 0 {
                        let r15 = ctx.guest_mem_base as u64;
                        let global_scene_watch =
                            std::env::var_os("RUSTEMU_SPIDEY_GLOBAL_SCENE_WATCH").is_some()
                                || std::env::var_os("RUSTEMU_SPIDEY_WATCH_GLOBAL_SCENE").is_some();
                        let watch_guest = if global_scene_watch {
                            0x003F_5BEC
                        } else if std::env::var_os("RUSTEMU_SPIDEY_WATCH_SCENE17F").is_some() {
                            0x1024_99BF
                        } else if std::env::var_os("RUSTEMU_SPIDEY_WATCH_SCENE18C").is_some() {
                            0x1024_9ACC
                        } else {
                            0x1EFF_FEF4
                        };
                        let watch_addr = r15.wrapping_add(watch_guest as u64);
                        debug_log(&format!(
                            "[HW-WATCH] arming (nye_n=0) guest=0x{:08X} watch_addr=0x{:016X} r15=0x{:016X}",
                            watch_guest, watch_addr, r15
                        ));
                        if global_scene_watch {
                            crate::xbox::aot::veh::replace_hw_watchpoint(
                                watch_addr,
                                "RUSTEMU_SPIDEY_GLOBAL_SCENE_WATCH",
                            );
                        } else {
                            crate::xbox::aot::veh::arm_hw_watchpoint(watch_addr);
                        }
                    }
                    try_fire_isr_dpc_preserving_thread_context(
                        &mut ctx,
                        &memory,
                        &vblank,
                        "NtYieldExecution",
                    );
                }
            }

            match kr {
                crate::xbox::kernel::KernelResult::Handled
                | crate::xbox::kernel::KernelResult::NotHandled => {
                    if is_call {
                        // CALL: re-enter at host_resume (skip INT3)
                        entry_host = host_resume as *mut u8;
                    } else {
                        // JMP (tail call): resolve jmp_ret_addr
                        let normalized = normalize_guest_addr(jmp_ret_addr);
                        if jmp_ret_addr == 0 {
                            guest_addr = 0;
                            break; // exit inner loop
                        } else if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                            entry_host = unsafe { ctx.code_base.add(host_off as usize) };
                        } else {
                            guest_addr = normalized;
                            break; // exit inner loop, outer loop will resolve
                        }
                    }
                    // Continue inner loop — re-enter trampoline
                }
                crate::xbox::kernel::KernelResult::SpawnThread {
                    entry: spawn_entry,
                    start_routine: spawn_sr,
                    context: spawn_ctx,
                    thread_handle,
                } => {
                    // Defer the actual host spawn until after the caller has
                    // resumed guest code. Several XDK wrappers return the new
                    // thread handle in EAX and the caller stores it immediately
                    // after PsCreateSystemThreadEx; if our host child runs first,
                    // it can signal completion before the handle is published.
                    deferred_child_spawns.push((
                        spawn_entry,
                        spawn_sr,
                        spawn_ctx,
                        thread_handle,
                        "aot-dispatch",
                    ));

                    if is_call {
                        entry_host = host_resume as *mut u8;
                    } else {
                        let normalized = normalize_guest_addr(jmp_ret_addr);
                        if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                            entry_host = unsafe { ctx.code_base.add(host_off as usize) };
                        } else {
                            guest_addr = normalized;
                            break;
                        }
                    }
                }
                crate::xbox::kernel::KernelResult::ThreadExit => {
                    guest_addr = 0;
                    break;
                }
                crate::xbox::kernel::KernelResult::Halt => {
                    guest_addr = 0;
                    break;
                }
                crate::xbox::kernel::KernelResult::QuickReboot => {
                    // Cxbx-R style: re-enter XBE from entry point with persistent
                    // memory intact. The game wrote launch data before calling
                    // HalReturnToFirmware(2). On re-entry, the CRT sees the
                    // populated LaunchDataPage and takes the normal boot path.
                    debug_log(&format!(
                        "[QUICK-REBOOT] Re-entering from entry=0x{:08X} → start_routine=0x{:08X} after {} kernel calls",
                        entry, start_routine, kernel_calls
                    ));
                    // Reset guest state — fresh stack, re-enter via XapiThreadStartup
                    ctx.guest.esp = worker_stack_top - 12;
                    memory.write_u32(ctx.guest.esp, 0); // return addr = 0 (RET_TO_ZERO sentinel)
                    memory.write_u32(ctx.guest.esp + 4, start_routine); // arg0: StartRoutine
                    memory.write_u32(ctx.guest.esp + 8, start_context); // arg1: StartContext
                    ctx.guest.eax = 0;
                    ctx.guest.ecx = 0;
                    ctx.guest.edx = 0;
                    ctx.guest.ebx = 0;
                    ctx.guest.ebp = 0;
                    ctx.guest.esi = 0;
                    ctx.guest.edi = 0;
                    ctx.reset_shadow_stack();
                    guest_addr = entry;
                    break; // exit inner loop, outer loop re-enters at entry
                }
            }
        }

        // Fall through to top of outer loop — re-entry check will handle
        // guest_addr=0 by injecting ISR/DPC (cooperative async injection)
    }

    // Signal main thread that worker is done
    live_counters.alive.store(false, Ordering::Relaxed);

    // Don't null out kernel_state — it's shared with main thread (main owns it)
    crate::xbox::aot::veh::clear_thread_context();

    debug_log(&format!(
        "AOT worker: finished — {} dispatches, {} MMIO, {} PB cmds, {} sign-ext fixups",
        ctx.dispatch_count, ctx.mmio_count, ctx.pb_commands, ctx.sign_ext_fixups
    ));
    WorkerStats {
        dispatch_count: ctx.dispatch_count,
        mmio_count: ctx.mmio_count,
        pb_commands: ctx.pb_commands,
        draw_calls: ctx.draw_calls,
        sign_ext_fixups: ctx.sign_ext_fixups,
    }
}

// ============================================================================
// Doom framebuffer rendering (HLE I_FinishUpdate)
// ============================================================================

/// Guest memory address where we write the 640×480 XRGB8888 Doom framebuffer.
/// Main thread reads from here. Located above bump allocator range.
pub const DOOM_FB_ADDR: u32 = 0x1A00_0000;
/// Flag: set to 1 when Doom FB has been written at least once.
pub const DOOM_FB_FLAG: u32 = 0x19FF_FFFC;

/// WAD file candidates (shared across lump loaders).
const WAD_CANDIDATES: &[&str] = &[
    "./RetroArch-Win64/system/xbox/EmuDisk/partition6/temp/doom.wad",
    "./games/doom3/base/classic/w/DOOM.WAD",
];

/// Load a named lump from DOOM.WAD on host filesystem.
/// Returns the raw lump bytes, or None if not found.
fn load_wad_lump(lump_name: &str) -> Option<Vec<u8>> {
    for path in WAD_CANDIDATES {
        if let Ok(data) = std::fs::read(path) {
            if data.len() < 12 {
                continue;
            }
            let magic = &data[0..4];
            if magic != b"IWAD" && magic != b"PWAD" {
                continue;
            }
            let num_lumps = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
            let dir_off = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
            for i in 0..num_lumps {
                let entry = dir_off + i * 16;
                if entry + 16 > data.len() {
                    break;
                }
                let name_bytes = &data[entry + 8..entry + 16];
                let name = std::str::from_utf8(name_bytes)
                    .unwrap_or("")
                    .trim_end_matches('\0');
                if name == lump_name {
                    let lump_off = u32::from_le_bytes([
                        data[entry],
                        data[entry + 1],
                        data[entry + 2],
                        data[entry + 3],
                    ]) as usize;
                    let lump_size = u32::from_le_bytes([
                        data[entry + 4],
                        data[entry + 5],
                        data[entry + 6],
                        data[entry + 7],
                    ]) as usize;
                    if lump_off + lump_size <= data.len() {
                        debug_log(&format!(
                            "[DOOM-WAD] Loaded {} ({} bytes) from {} (lump #{})",
                            lump_name, lump_size, path, i
                        ));
                        return Some(data[lump_off..lump_off + lump_size].to_vec());
                    }
                }
            }
        }
    }
    debug_log(&format!(
        "[DOOM-WAD] WARNING: Could not load lump '{}' from any WAD file",
        lump_name
    ));
    None
}

/// Load PLAYPAL palette (768 bytes = 256 RGB entries) from DOOM.WAD on host.
fn load_playpal() -> Option<[u8; 768]> {
    let data = load_wad_lump("PLAYPAL")?;
    if data.len() >= 768 {
        let mut pal = [0u8; 768];
        pal.copy_from_slice(&data[..768]);
        Some(pal)
    } else {
        None
    }
}

/// Decode a Doom patch_t (column-major picture format) into a flat 320×200 buffer.
/// Returns None if the lump is malformed.
fn decode_doom_patch(lump: &[u8], width: usize, height: usize) -> Option<Vec<u8>> {
    if lump.len() < 8 + width * 4 {
        return None;
    }
    let mut pixels = vec![0u8; width * height];

    for col in 0..width {
        let col_off = u32::from_le_bytes([
            lump[8 + col * 4],
            lump[9 + col * 4],
            lump[10 + col * 4],
            lump[11 + col * 4],
        ]) as usize;
        let mut pos = col_off;
        loop {
            if pos >= lump.len() {
                break;
            }
            let topdelta = lump[pos] as usize;
            if topdelta == 0xFF {
                break;
            } // End of column
            pos += 1;
            if pos >= lump.len() {
                break;
            }
            let length = lump[pos] as usize;
            pos += 2; // skip length byte + padding byte
            for row in 0..length {
                let y = topdelta + row;
                if y < height && pos < lump.len() {
                    pixels[y * width + col] = lump[pos];
                }
                pos += 1;
            }
            pos += 1; // skip trailing padding byte
        }
    }
    Some(pixels)
}

/// Inject TITLEPIC from WAD directly into screens[0] guest memory, then render
/// to the XRGB8888 framebuffer. This is a POC that bypasses game logic entirely.
fn doom_inject_titlepic(memory: &GuestMemory) {
    if !doom_synthetic_framebuffer_enabled() {
        return;
    }

    use std::sync::OnceLock;
    static TITLEPIC: OnceLock<Option<Vec<u8>>> = OnceLock::new();
    static PALETTE: OnceLock<Option<[u8; 768]>> = OnceLock::new();
    static INJECTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    if INJECTED.load(std::sync::atomic::Ordering::Relaxed) {
        return; // Already injected, doom_finish_update handles rendering
    }

    let titlepic = TITLEPIC.get_or_init(|| {
        let lump = load_wad_lump("TITLEPIC")?;
        if lump.len() < 8 {
            return None;
        }
        let w = u16::from_le_bytes([lump[0], lump[1]]) as usize;
        let h = u16::from_le_bytes([lump[2], lump[3]]) as usize;
        debug_log(&format!(
            "[DOOM-TITLEPIC] Decoding patch_t: {}x{} ({} bytes)",
            w,
            h,
            lump.len()
        ));
        decode_doom_patch(&lump, w, h)
    });
    let pal = PALETTE.get_or_init(|| load_playpal());

    let titlepic = match titlepic {
        Some(t) if t.len() >= 64000 => t,
        _ => {
            debug_log("[DOOM-TITLEPIC] No TITLEPIC lump found or decode failed");
            return;
        }
    };
    let pal = match pal {
        Some(p) => p,
        None => return,
    };

    // Find screens[0] pixel buffer address
    let screen_mgr = memory.read_u32(0x0010_1BB8);
    let screen_mgr = if screen_mgr != 0 && screen_mgr < 0x0800_0000 {
        screen_mgr
    } else {
        let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
        if cached == 0 || cached >= 0x0800_0000 {
            debug_log("[DOOM-TITLEPIC] No screen_mgr — rendering TITLEPIC directly to FB");
            doom_render_titlepic_direct(memory.base() as *mut u8, titlepic, pal, memory);
            INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);
            return;
        }
        cached
    };

    let screen_ptr = memory.read_u32(screen_mgr + 0x21AF0);
    if screen_ptr == 0 || screen_ptr >= 0x1000_0000 {
        debug_log(&format!(
            "[DOOM-TITLEPIC] screens[0]=0x{:08X} invalid — rendering directly to FB",
            screen_ptr
        ));
        doom_render_titlepic_direct(memory.base() as *mut u8, titlepic, pal, memory);
        INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);
        return;
    }

    // Write decoded 320×200 8-bit pixels into screens[0] guest memory
    debug_log(&format!(
        "[DOOM-TITLEPIC] Injecting decoded TITLEPIC into screens[0] at 0x{:08X}",
        screen_ptr
    ));
    for i in 0..64000usize {
        memory.write_u8(screen_ptr + i as u32, titlepic[i]);
    }
    INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);

    // Now render it via the normal pipeline
    doom_render_8bit_to_xrgb(memory.base() as *mut u8, screen_ptr, pal, memory);
}

/// Render TITLEPIC directly to DOOM_FB_ADDR (bypass screens[0] entirely).
/// Used when screen_mgr/screens[0] aren't allocated yet.
fn doom_render_titlepic_direct(
    base: *mut u8,
    titlepic: &[u8],
    pal: &[u8; 768],
    memory: &GuestMemory,
) {
    const DOOM_W: u32 = 320;
    const DOOM_H: u32 = 200;
    const OUT_W: u32 = 640;
    const OUT_H: u32 = 480;
    const Y_OFF: u32 = 40;

    unsafe {
        let dst = base.add(DOOM_FB_ADDR as usize);

        // Clear top and bottom bars (black)
        std::ptr::write_bytes(dst, 0, (Y_OFF * OUT_W * 4) as usize);
        std::ptr::write_bytes(
            dst.add(((Y_OFF + DOOM_H * 2) * OUT_W * 4) as usize),
            0,
            ((OUT_H - Y_OFF - DOOM_H * 2) * OUT_W * 4) as usize,
        );

        for y in 0..DOOM_H {
            for x in 0..DOOM_W {
                let pixel_idx = titlepic[(y * DOOM_W + x) as usize] as usize;
                let r = pal[pixel_idx * 3] as u32;
                let g = pal[pixel_idx * 3 + 1] as u32;
                let b = pal[pixel_idx * 3 + 2] as u32;
                let xrgb = (r << 16) | (g << 8) | b;

                let dx = (x * 2) as usize;
                let dy = ((Y_OFF + y * 2) * OUT_W) as usize;
                let dy1 = ((Y_OFF + y * 2 + 1) * OUT_W) as usize;

                let out = dst as *mut u32;
                *out.add(dy + dx) = xrgb;
                *out.add(dy + dx + 1) = xrgb;
                *out.add(dy1 + dx) = xrgb;
                *out.add(dy1 + dx + 1) = xrgb;
            }
        }
    }

    // Dump BMP
    unsafe {
        let fb = std::slice::from_raw_parts(
            base.add(DOOM_FB_ADDR as usize) as *const u32,
            (OUT_W * OUT_H) as usize,
        );
        write_bmp("./framebuffer.bmp", OUT_W, OUT_H, fb);
    }

    memory.write_u32(DOOM_FB_FLAG, 1);
    memory.write_u32(0xFD60_0800, DOOM_FB_ADDR);
    debug_log("[DOOM-TITLEPIC] TITLEPIC rendered to framebuffer — Stage 10 POC");
}

/// VEH-callable TITLEPIC injector using raw guest_mem pointer.
/// Called from OOVPA TryRunTics handler to inject TITLEPIC into framebuffer.
pub fn doom_inject_titlepic_from_guest_mem(guest_mem: *mut u8) {
    if !doom_synthetic_framebuffer_enabled() {
        return;
    }

    use std::sync::OnceLock;
    static TITLEPIC: OnceLock<Option<Vec<u8>>> = OnceLock::new();
    static PALETTE: OnceLock<Option<[u8; 768]>> = OnceLock::new();
    static INJECTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    if INJECTED.load(std::sync::atomic::Ordering::Relaxed) {
        // Already injected — just refresh the framebuffer from screens[0]
        doom_finish_update_from_guest_mem(guest_mem);
        return;
    }

    let titlepic = TITLEPIC.get_or_init(|| {
        let lump = load_wad_lump("TITLEPIC")?;
        if lump.len() < 8 {
            return None;
        }
        let w = u16::from_le_bytes([lump[0], lump[1]]) as usize;
        let h = u16::from_le_bytes([lump[2], lump[3]]) as usize;
        debug_log(&format!(
            "[DOOM-TITLEPIC] VEH: Decoding patch_t: {}x{} ({} bytes)",
            w,
            h,
            lump.len()
        ));
        decode_doom_patch(&lump, w, h)
    });
    let pal = PALETTE.get_or_init(|| load_playpal());

    let titlepic = match titlepic {
        Some(t) if t.len() >= 64000 => t,
        _ => return,
    };
    let pal = match pal {
        Some(p) => p,
        None => return,
    };

    // Write decoded TITLEPIC into screens[0] if available, otherwise render directly
    let screen_mgr = unsafe { *((guest_mem as u64 + 0x0010_1BB8u64) as *const u32) };
    let screen_mgr = if screen_mgr != 0 && screen_mgr < 0x0800_0000 {
        screen_mgr
    } else {
        let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
        if cached != 0 && cached < 0x0800_0000 {
            cached
        } else {
            0
        }
    };

    if screen_mgr != 0 {
        let screen_ptr =
            unsafe { *((guest_mem as u64 + screen_mgr as u64 + 0x21AF0u64) as *const u32) };
        if screen_ptr != 0 && screen_ptr < 0x1000_0000 {
            // Write TITLEPIC into screens[0]
            debug_log(&format!(
                "[DOOM-TITLEPIC] VEH: Injecting into screens[0] at 0x{:08X}",
                screen_ptr
            ));
            unsafe {
                let dst = guest_mem.add(screen_ptr as usize);
                std::ptr::copy_nonoverlapping(titlepic.as_ptr(), dst, 64000);
            }
            INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);
            doom_finish_update_from_guest_mem(guest_mem);
            return;
        }
    }

    // No screens[0] — render directly to framebuffer
    debug_log("[DOOM-TITLEPIC] VEH: No screens[0] — rendering directly to FB");
    unsafe {
        let dst = guest_mem.add(DOOM_FB_ADDR as usize);
        const DOOM_W: u32 = 320;
        const DOOM_H: u32 = 200;
        const OUT_W: u32 = 640;
        const OUT_H: u32 = 480;
        const Y_OFF: u32 = 40;

        std::ptr::write_bytes(dst, 0, (Y_OFF * OUT_W * 4) as usize);
        std::ptr::write_bytes(
            dst.add(((Y_OFF + DOOM_H * 2) * OUT_W * 4) as usize),
            0,
            ((OUT_H - Y_OFF - DOOM_H * 2) * OUT_W * 4) as usize,
        );

        for y in 0..DOOM_H {
            for x in 0..DOOM_W {
                let pixel_idx = titlepic[(y * DOOM_W + x) as usize] as usize;
                let r = pal[pixel_idx * 3] as u32;
                let g = pal[pixel_idx * 3 + 1] as u32;
                let b = pal[pixel_idx * 3 + 2] as u32;
                let xrgb = (r << 16) | (g << 8) | b;

                let dx = (x * 2) as usize;
                let dy = ((Y_OFF + y * 2) * OUT_W) as usize;
                let dy1 = ((Y_OFF + y * 2 + 1) * OUT_W) as usize;

                let out = dst as *mut u32;
                *out.add(dy + dx) = xrgb;
                *out.add(dy + dx + 1) = xrgb;
                *out.add(dy1 + dx) = xrgb;
                *out.add(dy1 + dx + 1) = xrgb;
            }
        }

        *((guest_mem as u64 + DOOM_FB_FLAG as u64) as *mut u32) = 1;
        *((guest_mem as u64 + 0xFD60_0800u64) as *mut u32) = DOOM_FB_ADDR;
    }
    INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);
    debug_log("[DOOM-TITLEPIC] VEH: TITLEPIC rendered to framebuffer");
}

/// Read Doom's 8-bit screens[0] framebuffer, apply PLAYPAL palette, scale 2×2,
/// and write 640×480 XRGB8888 to guest memory at DOOM_FB_ADDR.
fn doom_finish_update(memory: &GuestMemory) {
    if !doom_synthetic_framebuffer_enabled() {
        return;
    }

    use std::sync::OnceLock;
    static PALETTE: OnceLock<Option<[u8; 768]>> = OnceLock::new();
    static FRAME_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let pal = PALETTE.get_or_init(|| load_playpal());
    let pal = match pal {
        Some(p) => p,
        None => return, // No palette, can't render
    };

    // Screen manager at [0x101BB8] holds the game state struct.
    // The actual 8-bit pixel buffer for screens[0] is at [screen_mgr + 0x21AF0].
    // [0xE056C] holds screen OBJECTS (C++ wrappers with vtable), NOT pixel data.
    let screen_mgr = memory.read_u32(0x0010_1BB8);
    if screen_mgr == 0 || screen_mgr >= 0x0800_0000 {
        // Try cached value
        let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
        if cached == 0 || cached >= 0x0800_0000 {
            return;
        }
        // Use cached
        let screen_ptr = memory.read_u32(cached + 0x21AF0);
        if screen_ptr == 0 || screen_ptr >= 0x1000_0000 {
            return;
        }
        let frame = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if frame < 3 || frame % 60 == 0 {
            debug_log(&format!(
                "[DOOM-FB] Frame #{}: screen_mgr=0x{:08X}(cached) screens[0]=0x{:08X}",
                frame, cached, screen_ptr
            ));
        }
        doom_render_8bit_to_xrgb(memory.base() as *mut u8, screen_ptr, pal, memory);
        return;
    }
    let screen_ptr = memory.read_u32(screen_mgr + 0x21AF0);
    if screen_ptr == 0 || screen_ptr >= 0x1000_0000 {
        return;
    }

    let frame = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if frame < 3 || frame % 60 == 0 {
        debug_log(&format!(
            "[DOOM-FB] Frame #{}: screen_mgr=0x{:08X} screens[0]=0x{:08X}",
            frame, screen_mgr, screen_ptr
        ));
    }

    doom_render_8bit_to_xrgb(memory.base() as *mut u8, screen_ptr, pal, memory);
}

/// Shared render: convert 8-bit indexed pixels at `screen_ptr` (guest addr) to XRGB8888
/// at DOOM_FB_ADDR, scaled 2×2 to 640×480. Sets DOOM_FB_FLAG for main thread.
fn doom_render_8bit_to_xrgb(base: *mut u8, screen_ptr: u32, pal: &[u8; 768], memory: &GuestMemory) {
    const DOOM_W: u32 = 320;
    const DOOM_H: u32 = 200;
    const OUT_W: u32 = 640;
    const OUT_H: u32 = 480;
    const Y_OFF: u32 = 40;

    unsafe {
        let src = base.add(screen_ptr as usize);
        let dst = base.add(DOOM_FB_ADDR as usize);

        // Clear top and bottom bars (black)
        std::ptr::write_bytes(dst, 0, (Y_OFF * OUT_W * 4) as usize);
        std::ptr::write_bytes(
            dst.add(((Y_OFF + DOOM_H * 2) * OUT_W * 4) as usize),
            0,
            ((OUT_H - Y_OFF - DOOM_H * 2) * OUT_W * 4) as usize,
        );

        for y in 0..DOOM_H {
            for x in 0..DOOM_W {
                let pixel_idx = *src.add((y * DOOM_W + x) as usize) as usize;
                let r = pal[pixel_idx * 3] as u32;
                let g = pal[pixel_idx * 3 + 1] as u32;
                let b = pal[pixel_idx * 3 + 2] as u32;
                let xrgb = (r << 16) | (g << 8) | b;

                let dx = (x * 2) as usize;
                let dy = ((Y_OFF + y * 2) * OUT_W) as usize;
                let dy1 = ((Y_OFF + y * 2 + 1) * OUT_W) as usize;

                let out = dst as *mut u32;
                *out.add(dy + dx) = xrgb;
                *out.add(dy + dx + 1) = xrgb;
                *out.add(dy1 + dx) = xrgb;
                *out.add(dy1 + dx + 1) = xrgb;
            }
        }
    }

    // Dump framebuffer as BMP every 30 frames
    {
        static DUMP_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let dc = DUMP_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if dc % 30 == 0 {
            let fb = unsafe {
                std::slice::from_raw_parts(
                    base.add(DOOM_FB_ADDR as usize) as *const u32,
                    (OUT_W * OUT_H) as usize,
                )
            };
            write_bmp("./framebuffer.bmp", OUT_W, OUT_H, fb);
        }
    }

    memory.write_u32(DOOM_FB_FLAG, 1);
    memory.write_u32(0xFD60_0800, DOOM_FB_ADDR);
}

/// Write XRGB8888 pixel buffer as a 24-bit BMP file.
fn write_bmp(path: &str, w: u32, h: u32, pixels: &[u32]) {
    use std::io::Write;
    let row_bytes = w * 3;
    let row_pad = (4 - (row_bytes % 4)) % 4;
    let data_size = (row_bytes + row_pad) * h;
    let file_size = 54 + data_size;
    let mut f = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    // BMP header
    let _ = f.write_all(&[0x42, 0x4D]); // "BM"
    let _ = f.write_all(&(file_size as u32).to_le_bytes());
    let _ = f.write_all(&[0u8; 4]); // reserved
    let _ = f.write_all(&54u32.to_le_bytes()); // data offset
                                               // DIB header
    let _ = f.write_all(&40u32.to_le_bytes()); // header size
    let _ = f.write_all(&(w as i32).to_le_bytes());
    let _ = f.write_all(&(-(h as i32)).to_le_bytes()); // top-down
    let _ = f.write_all(&1u16.to_le_bytes()); // planes
    let _ = f.write_all(&24u16.to_le_bytes()); // bpp
    let _ = f.write_all(&[0u8; 24]); // rest of DIB header
                                     // Pixel data (BGR)
    let pad = vec![0u8; row_pad as usize];
    for y in 0..h {
        for x in 0..w {
            let p = pixels[(y * w + x) as usize];
            let _ = f.write_all(&[
                (p & 0xFF) as u8,
                ((p >> 8) & 0xFF) as u8,
                ((p >> 16) & 0xFF) as u8,
            ]);
        }
        if row_pad > 0 {
            let _ = f.write_all(&pad);
        }
    }
}

fn doom_read_u32_from_guest_mem(guest_mem: *mut u8, addr: u32) -> u32 {
    unsafe { *((guest_mem as u64 + addr as u64) as *const u32) }
}

fn doom_valid_framebuffer_state_from_guest_mem(
    guest_mem: *mut u8,
    state: u32,
) -> Option<(u32, u32, u32)> {
    if state == 0 || state >= 0x0800_0000 {
        return None;
    }

    let w = doom_read_u32_from_guest_mem(guest_mem, state.wrapping_add(0x14));
    let h = doom_read_u32_from_guest_mem(guest_mem, state.wrapping_add(0x18));
    let screen_ptr = doom_read_u32_from_guest_mem(guest_mem, state.wrapping_add(0x21AF0));
    if w == 320 && (160..=240).contains(&h) && screen_ptr != 0 && screen_ptr < 0x1000_0000 {
        Some((w, h, screen_ptr))
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub struct DoomFramebufferCandidate {
    pub state: u32,
    pub width: u32,
    pub height: u32,
    pub screen_ptr: u32,
    pub source: String,
    pub stage_index: Option<u32>,
}

fn doom_stage_index_for_source_from_guest_mem(
    guest_mem: *mut u8,
    state: u32,
    src_norm: u32,
) -> Option<u32> {
    if src_norm == 0 || state == 0 || state >= 0x0800_0000 {
        return None;
    }

    let stage0 = doom_read_u32_from_guest_mem(guest_mem, state.wrapping_add(0x3D70));
    let stage1 = doom_read_u32_from_guest_mem(guest_mem, state.wrapping_add(0x3D74));
    if src_norm == normalize_guest_addr(stage0) {
        Some(0)
    } else if src_norm == normalize_guest_addr(stage1) {
        Some(1)
    } else {
        None
    }
}

fn doom_push_framebuffer_candidate(
    guest_mem: *mut u8,
    out: &mut Vec<DoomFramebufferCandidate>,
    state: u32,
    source: String,
    stage_src: Option<u32>,
) {
    if out.iter().any(|candidate| candidate.state == state) {
        return;
    }

    let Some((width, height, screen_ptr)) =
        doom_valid_framebuffer_state_from_guest_mem(guest_mem, state)
    else {
        return;
    };

    let stage_index = stage_src.map(normalize_guest_addr).and_then(|src_norm| {
        doom_stage_index_for_source_from_guest_mem(guest_mem, state, src_norm)
    });

    out.push(DoomFramebufferCandidate {
        state,
        width,
        height,
        screen_ptr,
        source,
        stage_index,
    });
}

pub fn doom_framebuffer_candidates_from_guest_mem(
    guest_mem: *mut u8,
    stage_src: Option<u32>,
) -> Vec<DoomFramebufferCandidate> {
    let mut candidates = Vec::new();

    let live = doom_read_u32_from_guest_mem(guest_mem, 0x0010_1BB8);
    doom_push_framebuffer_candidate(
        guest_mem,
        &mut candidates,
        live,
        "live".to_string(),
        stage_src,
    );

    let current_index = doom_read_u32_from_guest_mem(guest_mem, 0x0010_1B9C);
    if current_index < 16 {
        let table_state = doom_read_u32_from_guest_mem(
            guest_mem,
            0x0010_1BA4u32.wrapping_add(current_index.saturating_mul(4)),
        );
        doom_push_framebuffer_candidate(
            guest_mem,
            &mut candidates,
            table_state,
            format!("table_current{}", current_index),
            stage_src,
        );
    }

    for i in 0..16u32 {
        let table_state =
            doom_read_u32_from_guest_mem(guest_mem, 0x0010_1BA4u32.wrapping_add(i * 4));
        doom_push_framebuffer_candidate(
            guest_mem,
            &mut candidates,
            table_state,
            format!("table{}", i),
            stage_src,
        );
    }

    let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
    doom_push_framebuffer_candidate(
        guest_mem,
        &mut candidates,
        cached,
        "cached".to_string(),
        stage_src,
    );

    candidates
}

fn doom_resolve_framebuffer_state_from_guest_mem(
    guest_mem: *mut u8,
) -> Option<(u32, u32, u32, u32, &'static str)> {
    let live = doom_read_u32_from_guest_mem(guest_mem, 0x0010_1BB8);
    if let Some((w, h, screen_ptr)) = doom_valid_framebuffer_state_from_guest_mem(guest_mem, live) {
        return Some((live, w, h, screen_ptr, "live"));
    }

    let current_index = doom_read_u32_from_guest_mem(guest_mem, 0x0010_1B9C);
    if current_index < 16 {
        let table_state = doom_read_u32_from_guest_mem(
            guest_mem,
            0x0010_1BA4u32.wrapping_add(current_index.saturating_mul(4)),
        );
        if let Some((w, h, screen_ptr)) =
            doom_valid_framebuffer_state_from_guest_mem(guest_mem, table_state)
        {
            return Some((table_state, w, h, screen_ptr, "table_current"));
        }
    }

    let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
    if let Some((w, h, screen_ptr)) = doom_valid_framebuffer_state_from_guest_mem(guest_mem, cached)
    {
        return Some((cached, w, h, screen_ptr, "cached"));
    }

    None
}

/// Refresh Doom's 512x256 ARGB staging surface from the verified 8-bit
/// software framebuffer before XGSwizzleRect consumes it.
pub fn doom_refresh_d3d_staging_source_for_xg(
    guest_mem: *mut u8,
    src_addr: u32,
    tex_w: u32,
    tex_h: u32,
    bpp: u32,
) -> bool {
    if bpp != 4 || tex_w < 320 || tex_h < 200 || tex_w > 2048 || tex_h > 2048 {
        return false;
    }

    use std::sync::OnceLock;
    static PALETTE: OnceLock<Option<[u8; 768]>> = OnceLock::new();
    static REFRESH_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let pal = match PALETTE.get_or_init(|| load_playpal()) {
        Some(p) => p,
        None => return false,
    };

    let src_norm = normalize_guest_addr(src_addr);
    let Some(candidate) = doom_framebuffer_candidates_from_guest_mem(guest_mem, Some(src_norm))
        .into_iter()
        .find(|candidate| candidate.stage_index.is_some())
    else {
        return false;
    };

    let screen_mgr = candidate.state;
    let screen_ptr = candidate.screen_ptr;
    let stage_index = candidate.stage_index.unwrap_or(0);
    if screen_ptr == 0 || screen_ptr >= 0x2000_0000 || src_norm >= 0x2000_0000 {
        return false;
    }

    let pitch = tex_w as usize * 4usize;
    let total = pitch.saturating_mul(tex_h as usize);
    if total == 0 || src_norm as usize + total > 0x2000_0000 {
        return false;
    }

    unsafe {
        let dst = guest_mem.add(src_norm as usize);
        let src = guest_mem.add(screen_ptr as usize);

        std::ptr::write_bytes(dst, 0, total);

        for y in 0..200usize {
            let dst_row = dst.add(y * pitch);
            let src_row = src.add(y * 320usize);
            for x in 0..320usize {
                let idx = *src_row.add(x) as usize;
                let r = pal[idx * 3] as u32;
                let g = pal[idx * 3 + 1] as u32;
                let b = pal[idx * 3 + 2] as u32;
                let argb = 0xFF00_0000u32 | (r << 16) | (g << 8) | b;
                std::ptr::write_unaligned(dst_row.add(x * 4) as *mut u32, argb);
            }
        }
    }

    let n = REFRESH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        debug_log(&format!(
            "[DOOM-STAGING-XG] #{} state={} screen_mgr=0x{:08X} screen=0x{:08X} stage{}=0x{:08X} tex={}x{} pitch={} refreshed=320x200",
            n,
            candidate.source,
            screen_mgr,
            screen_ptr,
            stage_index,
            src_norm,
            tex_w,
            tex_h,
            pitch
        ));
    }

    true
}

/// VEH-callable version using raw guest_mem pointer (no GuestMemory wrapper needed).
pub fn doom_finish_update_from_guest_mem(guest_mem: *mut u8) {
    if !doom_synthetic_framebuffer_enabled() {
        doom_clear_synthetic_framebuffer_flag_from_guest_mem(guest_mem);
        return;
    }

    use std::sync::OnceLock;
    static PALETTE: OnceLock<Option<[u8; 768]>> = OnceLock::new();
    static FRAME_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let pal = PALETTE.get_or_init(|| load_playpal());
    let pal = match pal {
        Some(p) => p,
        None => return,
    };

    // [0x101BB8] is reused by TryRunTics during the frame loop. Use it only
    // when the pointed-to struct validates; otherwise fall back to the last
    // verified state captured during Doom init.
    let Some((screen_mgr, _, _, screen_ptr, source)) =
        doom_resolve_framebuffer_state_from_guest_mem(guest_mem)
    else {
        return;
    };

    let frame = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if frame < 3 || frame % 60 == 0 {
        let src_base = unsafe { guest_mem.add(screen_ptr as usize) };
        let nonzero = unsafe {
            let s = std::slice::from_raw_parts(src_base, 320 * 200);
            s.iter().filter(|&&b| b != 0).count()
        };
        debug_log(&format!(
            "[DOOM-FB] VEH Frame #{}: source={} screen_mgr=0x{:08X} screens[0]=0x{:08X} nonzero={}/64000",
            frame, source, screen_mgr, screen_ptr, nonzero
        ));
    }

    // Create a temporary GuestMemory wrapper for the shared render function
    // (safe because guest_mem is the base of our 4GB reservation)
    unsafe {
        let dst = guest_mem.add(DOOM_FB_ADDR as usize);
        let src = guest_mem.add(screen_ptr as usize);
        const DOOM_W: u32 = 320;
        const DOOM_H: u32 = 200;
        const OUT_W: u32 = 640;
        const OUT_H: u32 = 480;
        const Y_OFF: u32 = 40;

        std::ptr::write_bytes(dst, 0, (Y_OFF * OUT_W * 4) as usize);
        std::ptr::write_bytes(
            dst.add(((Y_OFF + DOOM_H * 2) * OUT_W * 4) as usize),
            0,
            ((OUT_H - Y_OFF - DOOM_H * 2) * OUT_W * 4) as usize,
        );

        for y in 0..DOOM_H {
            for x in 0..DOOM_W {
                let pixel_idx = *src.add((y * DOOM_W + x) as usize) as usize;
                let r = pal[pixel_idx * 3] as u32;
                let g = pal[pixel_idx * 3 + 1] as u32;
                let b = pal[pixel_idx * 3 + 2] as u32;
                let xrgb = (r << 16) | (g << 8) | b;

                let dx = (x * 2) as usize;
                let dy = ((Y_OFF + y * 2) * OUT_W) as usize;
                let dy1 = ((Y_OFF + y * 2 + 1) * OUT_W) as usize;

                let out = dst as *mut u32;
                *out.add(dy + dx) = xrgb;
                *out.add(dy + dx + 1) = xrgb;
                *out.add(dy1 + dx) = xrgb;
                *out.add(dy1 + dx + 1) = xrgb;
            }
        }

        // Dump BMP every 30 frames
        {
            static VEH_DUMP: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let dc = VEH_DUMP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if dc % 30 == 0 {
                let fb = std::slice::from_raw_parts(dst as *const u32, (OUT_W * OUT_H) as usize);
                write_bmp("./framebuffer.bmp", OUT_W, OUT_H, fb);
            }
        }

        // Set DOOM_FB_FLAG
        *((guest_mem as u64 + DOOM_FB_FLAG as u64) as *mut u32) = 1;
        // Set PCRTC_START
        *((guest_mem as u64 + 0xFD60_0800u64) as *mut u32) = DOOM_FB_ADDR;
    }
}

pub fn doom_framebuffer_state_ready_from_guest_mem(guest_mem: *mut u8) -> bool {
    if !doom_synthetic_framebuffer_enabled() {
        doom_clear_synthetic_framebuffer_flag_from_guest_mem(guest_mem);
        return false;
    }

    let live = doom_read_u32_from_guest_mem(guest_mem, 0x0010_1BB8);
    let live_w = if live != 0 && live < 0x0800_0000 {
        doom_read_u32_from_guest_mem(guest_mem, live.wrapping_add(0x14))
    } else {
        0
    };
    let live_h = if live != 0 && live < 0x0800_0000 {
        doom_read_u32_from_guest_mem(guest_mem, live.wrapping_add(0x18))
    } else {
        0
    };
    let live_screen = if live != 0 && live < 0x0800_0000 {
        doom_read_u32_from_guest_mem(guest_mem, live.wrapping_add(0x21AF0))
    } else {
        0
    };
    let cached = DOOM_GAME_STATE_CACHE.load(std::sync::atomic::Ordering::Relaxed);
    let resolved = doom_resolve_framebuffer_state_from_guest_mem(guest_mem);
    let ready = resolved.is_some();
    let (state, w, h, screen_ptr, source) =
        resolved.unwrap_or((live, live_w, live_h, live_screen, "none"));

    static GATE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = GATE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 16 || (ready && n < 64) {
        debug_log(&format!(
            "[DOOM-FB-GATE] #{} ready={} source={} live=0x{:08X} cached=0x{:08X} state=0x{:08X} w={} h={} screen=0x{:08X}",
            n, ready as u8, source, live, cached, state, w, h, screen_ptr
        ));
    }

    ready
}

/// Convert Doom's software framebuffer and publish it to the libretro readback path.
///
/// Classic Doom routes its final frame through D3D Swap, but the actual picture is
/// already in `screens[0]` as 8-bit paletted pixels. This helper bridges that
/// software buffer into Rustemu's normal XRGB8888 readback buffer.
pub fn doom_finish_update_and_present_from_guest_mem(guest_mem: *mut u8) -> bool {
    if !doom_framebuffer_state_ready_from_guest_mem(guest_mem) {
        return false;
    }

    doom_finish_update_from_guest_mem(guest_mem);

    let flag = unsafe { *((guest_mem as u64 + DOOM_FB_FLAG as u64) as *const u32) };
    if flag == 0 {
        return false;
    }

    let fb = unsafe {
        std::slice::from_raw_parts(
            guest_mem.add(DOOM_FB_ADDR as usize),
            (640usize * 480usize * 4usize),
        )
    };
    crate::xbox::gpu::update_from_guest_framebuffer(fb);

    static PRESENT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = PRESENT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 8 || n.is_power_of_two() {
        let pixels = unsafe {
            std::slice::from_raw_parts(
                guest_mem.add(DOOM_FB_ADDR as usize) as *const u32,
                640usize * 480usize,
            )
        };
        let sample = pixels
            .iter()
            .step_by((pixels.len() / 4096).max(1))
            .take(4096)
            .filter(|&&p| (p & 0x00FF_FFFF) != 0)
            .count();
        debug_log(&format!(
            "[DOOM-PRESENT] #{} fb=0x{:08X} sample_nonblack={}/4096",
            n, DOOM_FB_ADDR, sample
        ));
    }

    true
}

/// Write the current framebuffer to a BMP file (called on exit).
pub fn dump_framebuffer_bmp(memory: &GuestMemory) {
    let flag = memory.read_u32(DOOM_FB_FLAG);
    if flag == 0 {
        debug_log("[DOOM-BMP] No Doom framebuffer to dump");
        return;
    }

    const W: u32 = 640;
    const H: u32 = 480;
    let pixel_count = (W * H) as usize;
    let row_size = (W * 3 + 3) & !3; // BMP rows are 4-byte aligned
    let pixel_data_size = row_size * H;
    let file_size = 54 + pixel_data_size;

    let mut bmp = Vec::with_capacity(file_size as usize);

    // BMP header (14 bytes)
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(file_size as u32).to_le_bytes());
    bmp.extend_from_slice(&[0u8; 4]); // reserved
    bmp.extend_from_slice(&54u32.to_le_bytes()); // pixel data offset

    // DIB header (40 bytes)
    bmp.extend_from_slice(&40u32.to_le_bytes()); // header size
    bmp.extend_from_slice(&(W as i32).to_le_bytes());
    bmp.extend_from_slice(&(H as i32).to_le_bytes()); // positive = bottom-up
    bmp.extend_from_slice(&1u16.to_le_bytes()); // planes
    bmp.extend_from_slice(&24u16.to_le_bytes()); // bpp
    bmp.extend_from_slice(&[0u8; 24]); // compression, size, resolution, colors

    // Pixel data (bottom-up, BGR)
    let base = memory.base() as *const u8;
    for y in (0..H).rev() {
        for x in 0..W {
            let off = DOOM_FB_ADDR + (y * W + x) * 4;
            let xrgb = unsafe { *(base.add(off as usize) as *const u32) };
            let r = ((xrgb >> 16) & 0xFF) as u8;
            let g = ((xrgb >> 8) & 0xFF) as u8;
            let b = (xrgb & 0xFF) as u8;
            bmp.push(b); // BGR order
            bmp.push(g);
            bmp.push(r);
        }
        // Pad row to 4-byte boundary
        let padding = (row_size - W * 3) as usize;
        for _ in 0..padding {
            bmp.push(0);
        }
    }

    let path = "./framebuffer.bmp";
    match std::fs::write(path, &bmp) {
        Ok(_) => debug_log(&format!("[DOOM-BMP] Wrote {} ({} bytes)", path, bmp.len())),
        Err(e) => debug_log(&format!("[DOOM-BMP] Failed to write {}: {}", path, e)),
    }
}
