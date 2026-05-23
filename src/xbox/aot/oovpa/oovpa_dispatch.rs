use super::{
    oovpa_hle,
    oovpa_hooks::rearm_pending,
    oovpa_scan::{auto_detect_argc, known_argc},
    read_dev_ptr, HleMode, G_PDEVICE, G_PDEVICE_DYNAMIC, OOVPA_DRAW_CALLS, OOVPA_HOOKS_FIRED,
    OOVPA_MMIO_COUNT, OOVPA_PB_COMMANDS, OOVPA_STATE, OOVPA_SWAP_COUNT,
};
/// OOVPA dispatch — VEH INT3 handler and interpreter HLE dispatch.
/// Split from oovpa.rs for modularity.
use crate::xbox::emulator::debug_log;
use std::sync::atomic::Ordering;

/// When true, CreateDevice TAP is active — suppress D3D HLE hooks so internal
/// functions run natively and properly initialize device data structures.
static CREATEDEVICE_TAP_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// Saved R14 value when CreateDevice TAP started — used to detect return.
static CREATEDEVICE_TAP_R14: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Set once CreateDevice TAP completes. After TAP, device[0] = NV2A register base
/// (0xFD000000) and must NOT be overwritten with pushbuffer pointers.
static CREATEDEVICE_TAP_COMPLETED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// When true, Reset TAP is active — suppress D3D HLE hooks so Reset can
/// tear down and recreate framebuffers/depth-stencil natively.
static RESET_TAP_ACTIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Saved R14 value when Reset TAP started — used to detect return.
static RESET_TAP_R14: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Count of OOVPA hooks fired since Reset TAP started — auto-deactivate after limit.
static RESET_TAP_HOOK_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

static SPIDEY_EVENT1_DEFER_PENDING: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT1_DEFER_OBJ: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT1_DEFER_SCENE: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT1_DEFER_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT1_REPLAY_LOG: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_TABLE_ADDR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_TABLE_COUNT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_TABLE_SLOTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_00_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_01_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_06_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_07_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_0A_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_0C_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_0E_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_10_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_12_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_14_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_16_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT_1A_LAST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static SPIDEY_EVENT_SLOT1_WATCH_GUEST: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Returns true if TAP mode completed and device[0] holds the NV2A register base.
pub fn tap_completed() -> bool {
    CREATEDEVICE_TAP_COMPLETED.load(Ordering::Relaxed)
}

/// Returns true if CreateDevice TAP is currently active (native D3D init running).
pub fn tap_active() -> bool {
    CREATEDEVICE_TAP_ACTIVE.load(Ordering::Relaxed)
}

/// Result of executing an HLE function from the interpreter.
pub struct HleExecResult {
    pub name: &'static str,
    pub cleanup: u16, // stdcall cleanup bytes (argc * 4)
}

#[allow(dead_code)]
fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn env_u32(name: &str) -> Option<u32> {
    let raw = std::env::var(name).ok()?;
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        raw.parse::<u32>().ok()
    }
}

fn oracle_interp_pc() -> Option<u32> {
    env_u32("RUSTEMU_ORACLE_INTERP_PC").or_else(|| env_u32("RUSTEMU_ORACLE_PASS_THROUGH_PC"))
}

fn oracle_interp_should_run(guest_addr: u32) -> Option<u32> {
    if oracle_interp_pc()? != guest_addr {
        return None;
    }
    static ORACLE_INTERP_SEEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let seen = ORACLE_INTERP_SEEN.fetch_add(1, Ordering::Relaxed);
    let skip = env_u32("RUSTEMU_ORACLE_INTERP_SKIP").unwrap_or(0);
    let limit = env_u32("RUSTEMU_ORACLE_INTERP_LIMIT").unwrap_or(1);
    if limit == 0 || seen < skip || seen >= skip.saturating_add(limit) {
        return None;
    }
    Some(seen)
}

#[cfg(windows)]
fn regs_from_context(
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
    entry: u32,
) -> crate::xbox::aot::micro_interp::X86Regs {
    let mut regs = crate::xbox::aot::micro_interp::X86Regs::new();
    regs.eax = context.Rax as u32;
    regs.ecx = context.Rcx as u32;
    regs.edx = context.Rdx as u32;
    regs.ebx = context.Rbx as u32;
    regs.esp = context.R14 as u32;
    regs.ebp = context.Rbp as u32;
    regs.esi = context.Rsi as u32;
    regs.edi = context.Rdi as u32;
    regs.eip = entry;
    regs.eflags = context.EFlags;
    regs
}

#[cfg(windows)]
fn apply_regs_to_context(
    regs: &crate::xbox::aot::micro_interp::X86Regs,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
) {
    context.Rax = regs.eax as u64;
    context.Rcx = regs.ecx as u64;
    context.Rdx = regs.edx as u64;
    context.Rbx = regs.ebx as u64;
    context.R14 = regs.esp as u64;
    context.Rbp = regs.ebp as u64;
    context.Rsi = regs.esi as u64;
    context.Rdi = regs.edi as u64;
    context.EFlags = regs.eflags;
}

fn title_patches_enabled() -> bool {
    // Cxbx-R-style HLE patches SDK/API boundaries, not Spider-Man's private
    // event/listener state. Keep these dispatch-side title taps observe-only.
    false
}

fn spidey_event_patches_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_EVENT_PATCHES")
        || env_flag("RUSTEMU_SPIDEY_INPUT_EVENT_PATCHES")
        || env_flag("RUSTEMU_SPIDEY_ORIGINZ_EVENT_PATCHES")
}

#[allow(dead_code)]
fn spidey_event1_replay_enabled() -> bool {
    // The event-1 replay/alias experiment wrote into Spider-Man's listener
    // tables and can consume the native title UI signal. Disabled by policy.
    false
}

fn valid_guest_string_ptr(addr: u32) -> bool {
    (addr >= 0x1000 && addr < 0x2000_0000) || (addr >= 0x8000_0000 && addr < 0xA000_0000)
}

fn raw_read_u8(base: *mut u8, addr: u32) -> u8 {
    if valid_guest_string_ptr(addr) {
        unsafe { *((base as u64 + addr as u64) as *const u8) }
    } else {
        0
    }
}

fn raw_read_u16(base: *mut u8, addr: u32) -> u16 {
    if valid_guest_string_ptr(addr) {
        unsafe { *((base as u64 + addr as u64) as *const u16) }
    } else {
        0
    }
}

fn raw_read_u32(base: *mut u8, addr: u32) -> u32 {
    if valid_guest_string_ptr(addr) {
        unsafe { *((base as u64 + addr as u64) as *const u32) }
    } else {
        0
    }
}

fn read_guest_ansi_raw(base: *mut u8, ansi_string_addr: u32) -> String {
    if !valid_guest_string_ptr(ansi_string_addr) {
        return String::new();
    }
    let len = raw_read_u16(base, ansi_string_addr) as u32;
    let buf = raw_read_u32(base, ansi_string_addr.wrapping_add(4));
    if len == 0 || !valid_guest_string_ptr(buf) {
        return String::new();
    }
    let mut bytes = Vec::with_capacity(len.min(512) as usize);
    for i in 0..len.min(512) {
        bytes.push(raw_read_u8(base, buf.wrapping_add(i)));
    }
    String::from_utf8_lossy(&bytes)
        .trim_end_matches('\0')
        .to_string()
}

fn read_guest_cstr_raw(base: *mut u8, addr: u32, max_len: u32) -> String {
    if !valid_guest_string_ptr(addr) {
        return String::new();
    }
    let mut bytes = Vec::new();
    for i in 0..max_len.min(128) {
        let b = raw_read_u8(base, addr.wrapping_add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn normalize_drive_symlink(drive: &str) -> Option<String> {
    let drive = drive.trim().trim_end_matches(['\\', '/']);
    if drive.is_empty() {
        return None;
    }
    if drive.starts_with("\\??\\") || drive.starts_with("\\??/") {
        return Some(drive.replace('/', "\\"));
    }
    let colon = drive.find(':')?;
    let letter = drive[..colon].chars().last()?.to_ascii_uppercase();
    Some(format!("\\??\\{}:", letter))
}

fn xapi_map_letter_to_directory(
    state: &mut crate::xbox::kernel::KernelState,
    guest_mem: *mut u8,
    args: &[u32; 8],
    source: &str,
) -> u32 {
    let drive = read_guest_ansi_raw(guest_mem, args[0]);
    let device = read_guest_ansi_raw(guest_mem, args[1]);
    let title_id = read_guest_cstr_raw(guest_mem, args[2], 64);
    let create_dir = args[3] != 0;
    let drive_link = match normalize_drive_symlink(&drive) {
        Some(d) => d,
        None => {
            debug_log(&format!(
                "[XAPI-MAP] {} invalid drive p=0x{:08X} text='{}' device='{}' title='{}'",
                source, args[0], drive, device, title_id
            ));
            return 0;
        }
    };

    let mut target = device.trim_end_matches(['\\', '/']).to_string();
    let lower_target = target.to_ascii_lowercase();
    let title_clean = title_id.trim_matches('\0').trim();
    if !title_clean.is_empty()
        && (lower_target.ends_with("\\udata")
            || lower_target.ends_with("/udata")
            || lower_target.ends_with("\\tdata")
            || lower_target.ends_with("/tdata"))
        && !lower_target.ends_with(&format!("\\{}", title_clean.to_ascii_lowercase()))
        && !lower_target.ends_with(&format!("/{}", title_clean.to_ascii_lowercase()))
    {
        target.push('\\');
        target.push_str(title_clean);
    }

    let mut host_created = None;
    if let Some(fs) = state.file_state.as_mut() {
        fs.add_symlink(&drive_link, &target);
        if create_dir
            || target.to_ascii_lowercase().contains("\\udata\\")
            || target.to_ascii_lowercase().contains("\\tdata\\")
        {
            host_created = fs.ensure_xbox_directory(&target);
        }
    }
    debug_log(&format!(
        "[XAPI-MAP] {} {} -> '{}' title='{}' create_dir={} host={}",
        source,
        drive_link,
        target,
        title_clean,
        create_dir,
        host_created.as_deref().unwrap_or("<not-created>")
    ));
    0 // STATUS_SUCCESS
}

/// Execute an OOVPA HLE function inline from the interpreter.
/// Looks up the hook by guest address, reads args from guest stack, dispatches.
pub fn execute_hle_for_interpreter(
    guest_addr: u32,
    ret_addr: u32,
    guest_mem: *mut u8,
    memory: &crate::xbox::memory::guest_memory::GuestMemory,
    eax: &mut u32,
    ecx: &mut u32,
    edx: &mut u32,
    esp: u32,
    kstate: &mut crate::xbox::kernel::KernelState,
) -> HleExecResult {
    let mut guard = OOVPA_STATE.lock().unwrap();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => {
            return HleExecResult {
                name: "unknown",
                cleanup: 0,
            }
        }
    };
    let hook = match state
        .matches
        .iter_mut()
        .find(|m| m.guest_addr == guest_addr)
    {
        Some(h) => h,
        None => {
            return HleExecResult {
                name: "unknown",
                cleanup: 0,
            }
        }
    };
    hook.call_count += 1;
    let mut argc = hook.argc;
    let name = hook.pattern_name;
    let _cpu_phase = crate::xbox::profiler::guard(crate::xbox::profiler::CpuPhase::OovpaHle);
    let _hle_subphase = crate::xbox::profiler::hle_name_guard(name);

    // Normalize CxbxDB LTCG pattern names to base function name.
    // "Direct3D_CreateDevice_16__LTCG_eax4_ebx6" → "Direct3D_CreateDevice"
    // "D3DDevice_DrawVerticesUP_12__LTCG_ebx3" → "D3DDevice_DrawVerticesUP"
    let is_ltcg = name.contains("__LTCG");
    let base_name = if let Some(idx) = name.find("__LTCG") {
        &name[..idx]
    } else {
        name
    };
    // Strip trailing "_N" size suffix (e.g. "_16" in "Direct3D_CreateDevice_16")
    let base_name = if base_name.len() > 3 {
        let last_underscore = base_name.rfind('_').unwrap_or(0);
        let suffix = &base_name[last_underscore + 1..];
        if suffix.len() <= 2 && suffix.chars().all(|c| c.is_ascii_digit()) {
            &base_name[..last_underscore]
        } else {
            base_name
        }
    } else {
        base_name
    };

    // argc fix: use known_argc table first (API-defined, immune to LTCG reshuffling),
    // then fall back to auto-detect from ret N for LTCG entries.
    if let Some(known) = known_argc(name) {
        if known != argc {
            debug_log(&format!(
                "[ARGC-OVERRIDE] {} argc {} → {} (known API argc)",
                name, argc, known
            ));
        }
        argc = known;
    } else if is_ltcg {
        let detected = auto_detect_argc(guest_mem as *const u8, guest_addr, guest_addr + 1024);
        if detected > 0 && detected != argc {
            debug_log(&format!(
                "[LTCG-FIX] {} argc {} → {} (from ret {})",
                name,
                argc,
                detected,
                detected * 4
            ));
            argc = detected;
        }
    }

    // Read args from guest stack after API argc normalization. Several XbSymDB
    // patterns report argc=0 even for public stdcall entries; using the final
    // argc keeps HLE output-pointer writes and stack cleanup in sync.
    let mut args = [0u32; 8];
    for i in 0..argc.min(8) as u32 {
        args[i as usize] = memory.read_u32(esp + 4 + i * 4);
    }

    // LTCG SetTexture RAX-dispatch arms were removed after empirical proof
    // that Spider-Man XDK 4134 does NOT compile SetTexture in LTCG-specialized
    // form (both __LTCG_eax1 and __LTCG_eax2 patterns scored <70% match rate
    // across the entire D3D/D3DX/XGRPH section bounds). The arms remain dead
    // code for this title; re-add them if/when a game that actually uses
    // LTCG SetTexture specialization is targeted.

    // Execute the HLE function based on name
    *eax = match base_name {
        "Direct3D_CreateDevice" => {
            // Find g_pDevice address dynamically from the matched function.
            // The CreateDevice code contains "mov [g_pDevice], reg" — scan for it.
            // Pattern: C7 05 xx xx xx xx (mov [imm32], imm32) near the function start.
            let mem = unsafe { std::slice::from_raw_parts(guest_mem, 0x08000000) };
            let func_addr = guest_addr as usize;
            let mut g_pdevice_global = 0u32;
            // Scan first 256 bytes of CreateDevice for mov [imm32], reg patterns
            // that write a device struct pointer to a global
            for off in (0..256).step_by(1) {
                if func_addr + off + 10 > mem.len() {
                    break;
                }
                let b = mem[func_addr + off];
                // Look for: mov [imm32], imm32 (C7 05 xx xx xx xx yy yy yy yy)
                // where imm32 is the g_pDevice address and value is a struct ptr.
                // Skip zero-value stores (flag resets like "mov [flag], 0").
                if b == 0xC7 && mem[func_addr + off + 1] == 0x05 {
                    let addr = u32::from_le_bytes([
                        mem[func_addr + off + 2],
                        mem[func_addr + off + 3],
                        mem[func_addr + off + 4],
                        mem[func_addr + off + 5],
                    ]);
                    let val = u32::from_le_bytes([
                        mem[func_addr + off + 6],
                        mem[func_addr + off + 7],
                        mem[func_addr + off + 8],
                        mem[func_addr + off + 9],
                    ]);
                    // g_pDevice should be in .data range and store a non-zero value
                    if addr > 0x10000 && addr < 0x800000 && val != 0 {
                        g_pdevice_global = addr;
                        break;
                    }
                }
                // Also check: mov esi/edi/ebx, [imm32] (8B 35/3D/1D xx xx xx xx)
                if b == 0x8B
                    && (mem[func_addr + off + 1] == 0x35
                        || mem[func_addr + off + 1] == 0x3D
                        || mem[func_addr + off + 1] == 0x1D)
                {
                    let addr = u32::from_le_bytes([
                        mem[func_addr + off + 2],
                        mem[func_addr + off + 3],
                        mem[func_addr + off + 4],
                        mem[func_addr + off + 5],
                    ]);
                    if addr > 0x10000 && addr < 0x800000 && g_pdevice_global == 0 {
                        g_pdevice_global = addr;
                        // Don't break — prefer C7 05 pattern
                    }
                }
            }
            // Use known game-specific g_pDevice address based on CreateDevice location
            if guest_addr == 0x00054C10 {
                g_pdevice_global = 0x0005_FBC8; // Classic Doom XDK 5849 — g_pDevice at 0x5FBC8
            } else if guest_addr == 0x002F3BD0 {
                g_pdevice_global = G_PDEVICE; // Spider-Man XDK 4134 (0x3038E0)
            } else if g_pdevice_global == 0 {
                g_pdevice_global = G_PDEVICE; // fallback
            }

            // Device struct address: game-specific
            // Classic Doom: static device at 0x5FBD0 (inside D3D section data)
            // Spider-Man: heap-allocated at 0x300E00
            let dev_addr = if guest_addr == 0x00054C10 {
                0x0005_FBD0u32
            } else {
                0x00300E00u32
            };
            let pb_base = 0x00EA0000u32;
            let pb_size = 0x00040000u32; // 256KB pushbuffer
            unsafe {
                let gm = guest_mem as u64;
                let db = gm + dev_addr as u64;

                // Zero device struct first (32KB)
                for i in (0..0x8000u32).step_by(4) {
                    *((db + i as u64) as *mut u32) = 0;
                }

                // g_pDevice pointer — also store in dynamic global for other hooks
                *((gm + g_pdevice_global as u64) as *mut u32) = dev_addr;
                G_PDEVICE_DYNAMIC.store(g_pdevice_global, std::sync::atomic::Ordering::Relaxed);
                if args[5] != 0 && args[5] < 0x1000_0000 {
                    *((gm + args[5] as u64) as *mut u32) = dev_addr;
                }

                // Pushbuffer pointers (most critical — 6 reads by D3D code)
                *((db + 0x00) as *mut u32) = pb_base; // pPut cursor
                *((db + 0x04) as *mut u32) = pb_base + pb_size - 0x100; // pThreshold
                *((db + 0x08) as *mut u32) = 0x13; // flags (created+active+skip_copy)
                *((db + 0x1A04u64) as *mut u32) = pb_base; // PUT pointer
                *((db + 0x1A08u64) as *mut u32) = pb_base; // current position
                *((db + 0x1A14u64) as *mut u32) = pb_base + pb_size; // limit/threshold
                *((db + 0x2078u64) as *mut u32) = pb_base; // GET

                // Surface format fields
                *((db + 0x0784u64) as *mut u32) = 0; // surface state
                *((db + 0x0794u64) as *mut u32) = 0; // surface config
                *((db + 0x0798u64) as *mut u32) = 0; // back buffer count
                *((db + 0x08D4u64) as *mut u32) = 0; // present interval
                *((db + 0x08D8u64) as *mut u32) = 0; // swap state

                // Viewport floats (fild output targets)
                *((db + 0x096Cu64) as *mut u32) = 0x3F800000; // 1.0f
                *((db + 0x0970u64) as *mut u32) = 0x3F800000; // 1.0f
                *((db + 0x196Cu64) as *mut u32) = 0x11; // D3DFMT_LIN_A8R8G8B8

                // Fence/flip state
                *((db + 0x1C28u64) as *mut u32) = 0; // fence value
                *((db + 0x2428u64) as *mut u32) = 0; // flip count

                // Display mode / surface format (same as AOT path)
                *((db + 0x205Cu64) as *mut u32) = 0x11; // D3DFMT_LIN_A8R8G8B8
                *((db + 0x2060u64) as *mut u32) = 640; // back buffer width
                *((db + 0x2064u64) as *mut u32) = 480; // back buffer height
                *((db + 0x2068u64) as *mut u32) = 640 * 4; // stride
                *((db + 0x2044u64) as *mut u32) = 1; // back buffer count = 1

                // Render target surface
                let rt_addr = 0x00F0_0000u32;
                *((db + 0x2048u64) as *mut u32) = rt_addr; // render target base
                *((db + 0x204Cu64) as *mut u32) = rt_addr; // depth stencil base
                *((db + 0x2050u64) as *mut u32) = 640 * 4; // RT pitch
                *((db + 0x2054u64) as *mut u32) = 640 * 4; // DS pitch

                // Zero remaining read fields to prevent garbage dereferences
                for &off in &[
                    0x2Cu32, 0x30, 0x764, 0x934, 0x95C, 0x960, 0xEE0, 0xEE8, 0xF00, 0xF40, 0x192C,
                    0x1A18,
                ] {
                    *((db + off as u64) as *mut u32) = 0;
                }

                // Also set D3D initialized flag if Doom-specific address known
                if g_pdevice_global == 0x0005_FBC8 {
                    *((gm + 0x60508u64) as *mut u32) = 1; // D3D initialized flag
                }
            }
            OOVPA_MMIO_COUNT.fetch_add(10, std::sync::atomic::Ordering::Relaxed);
            OOVPA_PB_COMMANDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            crate::xbox::gpu::request_gpu_init();
            // Diagnostic: check ticdup at CreateDevice time
            unsafe {
                let gm2 = guest_mem as u64;
                let gs_ptr = *((gm2 + 0x101BB8) as *const u32);
                let (word_1272, ticdup) = if gs_ptr > 0 && gs_ptr < 0x1000_0000 {
                    let base = gm2 + gs_ptr as u64;
                    (
                        *((base + 0x1272) as *const u16),
                        *((base + 0x1F50) as *const u32),
                    )
                } else {
                    (0xDEAD, 0xDEAD_BEEF)
                };
                crate::xbox::emulator::debug_log(&format!(
                    "[TICDUP-DIAG] CreateDevice: gs@101BB8=0x{:08X} word@1272={} ticdup={}",
                    gs_ptr, word_1272, ticdup
                ));
            }
            crate::xbox::emulator::debug_log(&format!(
                "[INTERP-HLE] CreateDevice: g_pDevice=[0x{:08X}]→0x{:08X} PB=0x{:08X} (22 fields populated)",
                g_pdevice_global, dev_addr, pb_base
            ));
            0 // S_OK
        }
        "D3DDevice_Clear" => {
            let mut mmio = 0u64;
            oovpa_hle::hle_clear(&args, guest_mem, &mut mmio)
        }
        "D3DDevice_BeginVisibilityTest"
        | "D3DDevice_EndVisibilityTest"
        | "D3DDevice_GetVisibilityTestResult" => {
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_visibility_test(base_name, &a, guest_mem)
        }
        "D3DDevice_Swap" => {
            OOVPA_SWAP_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            OOVPA_PB_COMMANDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if guest_addr == 0x0005_0920
                && crate::xbox::worker::doom_finish_update_and_present_from_guest_mem(guest_mem)
            {
                return HleExecResult {
                    name,
                    cleanup: argc as u16 * 4,
                };
            }
            // Route through the same full hle_swap path as the AOT/VEH handler.
            // hle_swap drains the draw queue FIRST, then resolves scanout, then
            // readbacks. The partial present_framebuffer()-only approach (aac20db)
            // skipped drain_draw_queue → readback captured clear-only frame → black.
            oovpa_hle::hle_swap_external_with_flags(guest_mem, &mut 0u64, args[0]);
            crate::xbox::gpu::set_frame_dirty();
            0
        }
        "D3DDevice_SetVertexShader" => {
            oovpa_hle::hle_trace_vertex_shader_call(
                base_name,
                &args,
                guest_mem,
                "interp_hle",
                guest_addr,
                ret_addr,
                esp,
                [*eax, 0, *ecx, *edx, 0, 0],
            );
            oovpa_hle::hle_set_vertex_shader(&args, guest_mem)
        }
        "D3DDevice_SelectVertexShader" | "D3DDevice_SelectVertexShaderDirect" => {
            oovpa_hle::hle_trace_vertex_shader_call(
                base_name,
                &args,
                guest_mem,
                "interp_hle",
                guest_addr,
                ret_addr,
                esp,
                [*eax, 0, *ecx, *edx, 0, 0],
            )
        }
        "D3DDevice_SetVertexShaderInput" => {
            oovpa_hle::hle_trace_vertex_shader_call(
                base_name,
                &args,
                guest_mem,
                "interp_hle",
                guest_addr,
                ret_addr,
                esp,
                [*eax, 0, *ecx, *edx, 0, 0],
            );
            oovpa_hle::hle_set_vertex_shader_input(&args, guest_mem)
        }
        "D3DDevice_CreateVertexShader" => {
            // Interpreter path — convert slice to fixed array
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_create_vertex_shader(&a, guest_mem)
        }
        "D3DDevice_CreatePixelShader" => oovpa_hle::hle_create_pixel_shader(&args, guest_mem),
        "D3DDevice_SetPixelShader" | "D3DDevice_SetPixelShaderProgram" => {
            oovpa_hle::hle_set_pixel_shader(&args, guest_mem)
        }
        "D3DDevice_SetPixelShaderConstant" => 0,
        "D3DDevice_SetVertexShaderConstant" => {
            oovpa_hle::hle_trace_vertex_shader_call(
                base_name,
                &args,
                guest_mem,
                "interp_hle",
                guest_addr,
                ret_addr,
                esp,
                [*eax, 0, *ecx, *edx, 0, 0],
            );
            oovpa_hle::hle_set_vertex_shader_constant(
                &args,
                guest_mem,
                ret_addr,
                esp,
                [*eax, 0, *ecx, *edx, 0, 0],
            )
        }
        "XGSwizzleRect" => {
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_xg_swizzle_rect(&a, guest_mem)
        }
        "D3DDevice_CreateTexture" => {
            // Interpreter path — convert slice to fixed array, route to shared HLE.
            // 2026-04-25: was `=> 0` (broken stub). All 49 Spider-Man calls returned
            // *ppTex=NULL because the actual minting code in oovpa_hle.rs was never
            // invoked from this path. Per agents E1/E6/E8/E10 the fix is to call
            // the same hle_create_texture() helper that execute_manual_hle uses.
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_create_texture(&a, guest_mem)
        }
        "D3DDevice_CreateTexture2" => {
            // Internal XDK variant: returns the texture pointer in EAX. Doom stores
            // EAX directly after the call, so treating arg6 as ppTexture leaves its
            // render textures NULL even though the hook fires.
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_create_texture2(&a, guest_mem)
        }
        "D3DDevice_SetTexture" => {
            // Interpreter path — convert slice to fixed array
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_set_texture(&a, guest_mem);
            0
        }
        // LTCG-suffix arms intentionally omitted — base_name strips __LTCG_*
        // suffix, so these would never fire via this match anyway (the arms
        // are in the pre-match block above when re-added for specific games).
        "D3DResource_Register" => {
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_register_resource(*ecx, &a, guest_mem);
            0
        }
        "D3DDevice_CopyRects" => oovpa_hle::hle_copy_rects(&args, guest_mem),
        "D3DDevice_SetRenderTarget" => oovpa_hle::hle_set_render_target(&args, guest_mem),
        "D3DDevice_SetRenderState_Simple" => {
            let state = oovpa_hle::render_state_slot_from_fastcall_reg(*ecx);
            oovpa_hle::hle_apply_interpreter_render_state(
                "SetRenderState_Simple/interp",
                state,
                *edx,
            )
        }
        "D3DDevice_SetRenderStateNotInline" => oovpa_hle::hle_apply_interpreter_render_state(
            "SetRenderStateNotInline/interp",
            args[0],
            args[1],
        ),
        "D3DDevice_SetTextureStageState"
        | "D3DDevice_SetTextureStageStateNotInline"
        | "D3DDevice_SetTextureStageStateNotInline2"
        | "D3D_CDevice_SetTextureStageStateNotInline" => oovpa_hle::hle_note_texture_stage_state(
            "SetTextureStageState/interp",
            args[0],
            args[1],
            args[2],
        ),
        "D3DDevice_SetTextureState_TexCoordIndex" => oovpa_hle::hle_note_texture_stage_state(
            "SetTextureState_TexCoordIndex/interp",
            args[0],
            28,
            args[1],
        ),
        "D3DDevice_SetTextureState_BorderColor" => oovpa_hle::hle_note_texture_stage_state(
            "SetTextureState_BorderColor/interp",
            args[0],
            29,
            args[1],
        ),
        "D3DDevice_SetTextureState_ColorKeyColor" => oovpa_hle::hle_note_texture_stage_state(
            "SetTextureState_ColorKeyColor/interp",
            args[0],
            30,
            args[1],
        ),
        "D3DDevice_SetLight" => oovpa_hle::hle_note_set_light(args[0], args[1], guest_mem),
        "D3DDevice_LightEnable" => oovpa_hle::hle_note_light_enable(args[0], args[1]),
        "D3DDevice_SetMaterial" => oovpa_hle::hle_note_set_material(args[0], guest_mem),
        n if n.starts_with("D3DDevice_SetRenderState_") => {
            let suffix = &n["D3DDevice_SetRenderState_".len()..];
            if let Some(slot) = oovpa_hle::specialized_rs_slot_from_suffix(suffix) {
                oovpa_hle::hle_apply_interpreter_render_state(suffix, slot, args[0])
            } else {
                0
            }
        }
        "D3DDevice_SetStreamSource" => {
            let mut a = [0u32; 8];
            for (i, &v) in args.iter().take(8).enumerate() {
                a[i] = v;
            }
            oovpa_hle::hle_set_stream_source(&a, guest_mem);
            0
        }
        "D3DDevice_Reset" => 0,
        "D3DDevice_DrawVertices" => {
            let mut mmio = 0u64;
            oovpa_hle::hle_draw(&args, guest_mem, &mut mmio);
            OOVPA_DRAW_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            0
        }
        "D3DDevice_DrawIndexedVertices" => {
            let mut mmio = 0u64;
            oovpa_hle::hle_draw_indexed_vertices(&args, guest_mem, &mut mmio);
            OOVPA_DRAW_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            0
        }
        "D3DDevice_DrawVerticesUP" => {
            oovpa_hle::hle_draw_vertices_up(&args, guest_mem);
            OOVPA_DRAW_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            0
        }
        "D3DDevice_DrawIndexedVerticesUP" => {
            oovpa_hle::hle_draw_indexed_vertices_up(&args, guest_mem);
            OOVPA_DRAW_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            0
        }
        "D3DDevice_BeginStateBig" => oovpa_hle::hle_begin_state_big(args[0], guest_mem),
        n if crate::xbox::apu::is_dsound_hle_name(n) => {
            crate::xbox::apu::handle_dsound_hle(n, &args, guest_mem)
        }
        "D3DDevice_KickOff" => 0,
        "D3D_SetFence" | "D3DDevice_InsertFence" => {
            oovpa_hle::hle_complete_d3d_wait("interp:SetFence", guest_mem)
        }
        "D3DDevice_IsFencePending" => 0,
        "D3DDevice_BlockOnFence"
        | "D3D_BlockOnFence"
        | "D3D_BlockOnTime"
        | "D3D_BlockOnResource" => oovpa_hle::hle_complete_d3d_wait(name, guest_mem),
        "D3DDevice_BlockUntilVerticalBlank" => {
            std::thread::sleep(std::time::Duration::from_millis(1));
            1 // VBlank occurred
        }
        "D3D_MakeSpace" => oovpa_hle::hle_reset_pushbuffer(guest_mem),
        "CDevice_SetStateVB" => 0,
        "XapiMapLetterToDirectory" => {
            xapi_map_letter_to_directory(kstate, guest_mem, &args, "interp")
        }
        "XGetLaunchInfo" => oovpa_hle::hle_xget_launch_info(&args, guest_mem),
        "XInitDevices" => oovpa_hle::xapi_hle_init_devices(guest_mem),
        "XGetDevices" => oovpa_hle::xapi_hle_get_devices(guest_mem, args[0], "XGetDevices/interp"),
        "XGetDeviceChanges" => {
            oovpa_hle::xapi_hle_get_device_changes(guest_mem, args[0], args[1], args[2])
        }
        "XInputOpen" => oovpa_hle::xapi_hle_input_open(&args),
        "XInputGetState" => {
            oovpa_hle::xapi_hle_input_get_state(&args, guest_mem, "XInputGetState/interp")
        }
        "XInputGetCapabilities" => oovpa_hle::xapi_hle_input_get_capabilities(&args, guest_mem),
        "XInputClose" | "XInputPoll" | "XInputSetState" => 0,
        "CDirectSound_CreateSoundBuffer" => {
            // args: [0]=pThis, [1]=pdsbd, [2]=ppBuffer (out), [3]=pUnkOuter
            // Write a fake buffer object to ppBuffer to stop the infinite retry loop.
            // Bump allocator for fake COM objects (each 64 bytes).
            static DSOUND_BUF_BUMP: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0x01E0_0000);
            let pp_buffer = args[2];
            if pp_buffer != 0 && pp_buffer < 0x1000_0000 {
                let fake_buf = DSOUND_BUF_BUMP.fetch_add(64, std::sync::atomic::Ordering::Relaxed);
                memory.write_u32(pp_buffer, fake_buf);
                // Write a minimal vtable-like structure (all zeros = no-op methods)
                for i in 0..16u32 {
                    memory.write_u32(fake_buf + i * 4, 0);
                }
            }
            0 // S_OK
        }
        "Doom_DSoundRelease" => {
            // DirectSound buffer list cleanup — skip entirely.
            // Our fake DirectSoundCreateBuffer objects have NULL linked list
            // pointers, causing infinite traversal. Return 1 (success).
            1
        }
        "DirectSoundCreateBuffer" => {
            // Same as CDirectSound_CreateSoundBuffer: write a fake buffer object
            let pp_buffer = args[1]; // DirectSoundCreateBuffer: args[0]=pdsbd, args[1]=ppBuffer
            if pp_buffer != 0 && pp_buffer < 0x1000_0000 {
                static DSOUND_BUF_BUMP2: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0x01E8_0000);
                let fake_buf = DSOUND_BUF_BUMP2.fetch_add(64, std::sync::atomic::Ordering::Relaxed);
                memory.write_u32(pp_buffer, fake_buf);
                for i in 0..16u32 {
                    memory.write_u32(fake_buf + i * 4, 0);
                }
            }
            0 // S_OK
        }
        "CDirectSoundBuffer_SetFormat"
        | "CDirectSoundBuffer_SetHeadroom"
        | "CDirectSoundBuffer_SetEG"
        | "CDirectSoundBuffer_SetFilter"
        | "CDirectSoundBuffer_Play"
        | "CDirectSoundBuffer_PlayEx"
        | "CDirectSoundBuffer_Stop"
        | "CDirectSoundBuffer_GetStatus"
        | "CDirectSoundBuffer_SetCurrentPosition"
        | "CDirectSoundStream_AddRef"
        | "CDirectSoundStream_Discontinuity"
        | "CDirectSoundStream_Pause"
        | "CDirectSoundStream_Release"
        | "DirectSoundDoWork" => 0,
        // Doom D3D hooks — skip guest code, read framebuffer on Swap
        "Doom_D3D_Swap" => {
            crate::xbox::worker::doom_finish_update_and_present_from_guest_mem(guest_mem);
            std::thread::yield_now();
            0
        }
        "Doom_D3D_MakeSpace" => oovpa_hle::hle_reset_pushbuffer(guest_mem),
        n if n.starts_with("Doom_D3D_") => 0,
        _ => {
            crate::xbox::emulator::debug_log(&format!(
                "[INTERP-HLE] Unhandled hook: {} at 0x{:08X}",
                name, guest_addr
            ));
            0
        }
    };
    drop(guard);
    HleExecResult {
        name,
        cleanup: argc as u16 * 4,
    }
}

/// Check if an INT3 at host_offset is an OOVPA hook and dispatch it.
/// Called from VEH INT3 handler. Returns true if handled.
#[cfg(windows)]
pub fn veh_try_dispatch(
    host_offset: u32,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    ctx: &mut crate::xbox::aot::runtime::RuntimeContext,
    code_base: u64,
) -> bool {
    let mut guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let state = match guard.as_mut() {
        Some(s) if s.hooks_planted => s,
        _ => return false,
    };

    // Rearm any pending hooks
    rearm_pending(&mut state.matches, ctx.code_base);

    // Find matching hook
    let idx = state
        .matches
        .iter()
        .position(|m| m.host_offset == host_offset && m.active);
    let idx = match idx {
        Some(i) => i,
        None => return false,
    };

    let m = &mut state.matches[idx];
    m.call_count += 1;
    let cc = m.call_count;
    let mut argc = m.argc;
    let name = m.pattern_name;

    let hle_mode = m.hle_mode;
    let is_manual = m.is_manual;
    let original_byte = m.original_byte;
    let guest_addr = m.guest_addr;

    // argc fix: known_argc table first, then LTCG auto-detect fallback
    if let Some(known) = known_argc(name) {
        if known != argc {
            if cc == 1 {
                debug_log(&format!(
                    "[ARGC-OVERRIDE] {} argc {} → {} (known API argc)",
                    name, argc, known
                ));
            }
            argc = known;
        }
    } else if name.contains("__LTCG") {
        let detected = auto_detect_argc(context.R15 as *const u8, guest_addr, guest_addr + 1024);
        if detected > 0 && detected != argc {
            if cc == 1 {
                debug_log(&format!(
                    "[LTCG-FIX] {} argc {} → {} (from ret {})",
                    name,
                    argc,
                    detected,
                    detected * 4
                ));
            }
            argc = detected;
        }
    }

    // Warn if init-class functions fire more than once — signals broken init loop
    if cc == 2 {
        let is_init_class = name.contains("Init")
            || name.contains("Create")
            || name.contains("Startup")
            || name.contains("_prolog")
            || name == "Direct3D_CreateDevice"
            || name == "Doom_DirectSoundCreate";
        if is_init_class {
            debug_log(&format!(
                "[INIT-REPEAT-WARN] {} at 0x{:08X} fired {} times — init-class function should fire once",
                name, guest_addr, cc
            ));
        }
    }

    // === Wine-style SEH runtime handlers ===
    // Uses explicit frame registry (seh_runtime.rs) for guaranteed prolog/epilog symmetry.
    if name == "_SEH_prolog" {
        // First call: read the handler address from the prolog's `push <handler>` instruction
        crate::xbox::aot::seh_runtime::set_prolog_handler_lazy(context.R15, m.guest_addr);
        crate::xbox::aot::seh_runtime::push_frame(context, context.R15);
        // Jump to return address (caller continues after `call _SEH_prolog`)
        let ret_addr = unsafe { *((context.R15 + context.R14 as u64 - 4) as *const u32) };
        // Actually ret_addr was already consumed by push_frame — read from frame
        let (p, _) = crate::xbox::aot::seh_runtime::counts();
        let normalized = crate::xbox::emulator::normalize_guest_addr(
            crate::xbox::aot::seh_runtime::FRAME_STACK
                .with(|fs| fs.borrow().last().map(|f| f.caller_ret_addr).unwrap_or(0)),
        );
        if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
            context.Rip = code_base + host_off as u64;
            return true;
        }
        let saved_shadow = ctx.guest.shadow_stack_top;
        unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
        ctx.guest.shadow_stack_top = saved_shadow;
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
        ctx.guest.exit_guest_addr = normalized;
        context.Rip = ctx.guest.exit_addr;
        return true;
    }

    if name == "_SEH_epilog" {
        let ret_target = crate::xbox::aot::seh_runtime::pop_frame(context, context.R15);
        let normalized = crate::xbox::emulator::normalize_guest_addr(ret_target);
        if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
            context.Rip = code_base + host_off as u64;
            return true;
        }
        let saved_shadow = ctx.guest.shadow_stack_top;
        unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
        ctx.guest.shadow_stack_top = saved_shadow;
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
        ctx.guest.exit_guest_addr = normalized;
        context.Rip = ctx.guest.exit_addr;
        return true;
    }

    // POOL WATCHPOINT: check desc0 on every OOVPA hook
    {
        use std::sync::atomic::{AtomicU32, Ordering as AO};
        static OOVPA_POOL_PREV: AtomicU32 = AtomicU32::new(0xDEAD_BEEF);
        let desc0 = unsafe { *((context.R15 + 0x3F8C3C) as *const u32) };
        let prev = OOVPA_POOL_PREV.load(AO::Relaxed);
        if prev == 0xDEAD_BEEF {
            OOVPA_POOL_PREV.store(desc0, AO::Relaxed);
        } else if desc0 != prev {
            let counter = unsafe { *((context.R15 + 0x3F8C30) as *const u32) };
            debug_log(&format!(
                "[POOL-WATCH-OOVPA] desc0 changed: 0x{:08X} → 0x{:08X} during {} (#{}) counter=0x{:08X}",
                prev, desc0, name, cc, counter
            ));
            OOVPA_POOL_PREV.store(desc0, AO::Relaxed);
        }
    }

    // Any D3D OOVPA hook firing means game reached GPU-related code (Stage 8)
    ctx.mmio_count += 1;
    OOVPA_MMIO_COUNT.store(ctx.mmio_count, Ordering::Relaxed);
    OOVPA_HOOKS_FIRED.fetch_add(1, Ordering::Relaxed);

    // Read guest stack: [R14]=ret_addr, [R14+4..]=args
    let r14 = context.R14 as u32;
    let r15 = context.R15;
    let ret_addr = unsafe { *((r15 + r14 as u64) as *const u32) };
    let mut args = [0u32; 8];
    for i in 0..argc.min(8) {
        args[i as usize] = unsafe { *((r15 + (r14 + 4 + i as u32 * 4) as u64) as *const u32) };
    }

    // LTCG register calling convention: pattern names like
    // "Get2DSurfaceDesc_0__LTCG_edi1_ebx2_esi3" indicate args passed in registers.
    // Parse "_REGn" suffixes and override args[] from context registers.
    if name.contains("__LTCG") {
        if let Some(ltcg_idx) = name.find("__LTCG") {
            let suffix = &name[ltcg_idx + 6..]; // after "__LTCG"
            for part in suffix.split('_') {
                if part.is_empty() {
                    continue;
                }
                // Parse: "edi1", "ebx2", "esi3", "eax4", "ecx5", "edx6"
                let (reg_name, arg_idx) = if part.len() >= 4 {
                    let idx_str = &part[3..];
                    let reg = &part[..3];
                    (reg, idx_str.parse::<usize>().ok())
                } else {
                    ("", None)
                };
                if let Some(idx) = arg_idx {
                    if idx >= 1 && idx <= 8 {
                        let val = match reg_name {
                            "eax" => (context.Rax & 0xFFFF_FFFF) as u32,
                            "ebx" => (context.Rbx & 0xFFFF_FFFF) as u32,
                            "ecx" => (context.Rcx & 0xFFFF_FFFF) as u32,
                            "edx" => (context.Rdx & 0xFFFF_FFFF) as u32,
                            "esi" => (context.Rsi & 0xFFFF_FFFF) as u32,
                            "edi" => (context.Rdi & 0xFFFF_FFFF) as u32,
                            _ => continue,
                        };
                        args[idx - 1] = val; // 1-based → 0-based
                        if cc == 1 {
                            debug_log(&format!(
                                "[LTCG-REG] {} arg[{}] = {} = 0x{:08X}",
                                name,
                                idx - 1,
                                reg_name,
                                val
                            ));
                        }
                    }
                }
            }
        }
    }

    if let Some(oracle_call_index) = oracle_interp_should_run(guest_addr) {
        let entry_name = name;
        let entry_guest = guest_addr;
        let entry_ret = ret_addr;
        let entry_r14 = r14;
        let mut regs = regs_from_context(context, entry_guest);
        let memory =
            unsafe { crate::xbox::memory::guest_memory::GuestMemory::from_raw(ctx.guest_mem_base) };
        let mut oovpa_addrs = crate::xbox::aot::oovpa::get_hooked_guest_addrs();
        oovpa_addrs.retain(|&addr| addr != entry_guest);
        let exec_ranges = ctx.exec_ranges.clone();
        drop(guard);

        debug_log(&format!(
            "[ORACLE-INTERP] call#{} {} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} argc={} — running native body under micro_interp",
            oracle_call_index, entry_name, entry_guest, entry_ret, entry_r14, argc
        ));

        let result = crate::xbox::aot::micro_interp::interpret(
            &memory,
            &mut regs,
            entry_guest,
            entry_ret,
            env_u32("RUSTEMU_ORACLE_INTERP_MAX_INSNS").unwrap_or(100_000) as u64,
            &oovpa_addrs,
            &exec_ranges,
        );
        apply_regs_to_context(&regs, context);

        match result {
            crate::xbox::aot::micro_interp::InterpResult::ReturnedTo(returned_to) => {
                let normalized = crate::xbox::emulator::normalize_guest_addr(returned_to);
                debug_log(&format!(
                    "[ORACLE-INTERP] {} returned_to=0x{:08X} esp=0x{:08X} eax=0x{:08X}",
                    entry_name, normalized, regs.esp, regs.eax
                ));
                if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                    context.Rip = code_base + host_off as u64;
                    return true;
                }

                let saved_shadow = ctx.guest.shadow_stack_top;
                unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
                ctx.guest.shadow_stack_top = saved_shadow;
                ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
                ctx.guest.exit_guest_addr = normalized;
                context.Rip = ctx.guest.exit_addr;
                return true;
            }
            crate::xbox::aot::micro_interp::InterpResult::KernelCall { ordinal, ret_addr } => {
                debug_log(&format!(
                    "[ORACLE-INTERP] {} trapped on kernel ordinal={} ret=0x{:08X} eip=0x{:08X}",
                    entry_name, ordinal, ret_addr, regs.eip
                ));
            }
            crate::xbox::aot::micro_interp::InterpResult::OovpaHook {
                guest_addr,
                ret_addr,
            } => {
                debug_log(&format!(
                    "[ORACLE-INTERP] {} trapped on nested OOVPA guest=0x{:08X} ret=0x{:08X}",
                    entry_name, guest_addr, ret_addr
                ));
            }
            crate::xbox::aot::micro_interp::InterpResult::Timeout => {
                debug_log(&format!(
                    "[ORACLE-INTERP] {} timed out at eip=0x{:08X}",
                    entry_name, regs.eip
                ));
            }
            crate::xbox::aot::micro_interp::InterpResult::Unhandled { eip, mnemonic } => {
                debug_log(&format!(
                    "[ORACLE-INTERP] {} unhandled at 0x{:08X}: {}",
                    entry_name, eip, mnemonic
                ));
            }
            crate::xbox::aot::micro_interp::InterpResult::AccessViolation { eip, addr } => {
                debug_log(&format!(
                    "[ORACLE-INTERP] {} AV at eip=0x{:08X} addr=0x{:08X}",
                    entry_name, eip, addr
                ));
            }
        }

        let saved_shadow = ctx.guest.shadow_stack_top;
        unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
        ctx.guest.shadow_stack_top = saved_shadow;
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_ERROR;
        ctx.guest.exit_guest_addr = regs.eip;
        context.Rip = ctx.guest.exit_addr;
        return true;
    }

    if name == "SPIDEY_FSM_DISPATCH_TAP"
        || name == "SPIDEY_FRAME_DISPATCH_GATE_TAP"
        || name == "SPIDEY_BOOT_GATE_TAP"
        || name.starts_with("SPIDEY_F8580_")
        || name.starts_with("SPIDEY_FUN29C5A0_")
        || name == "SPIDEY_SCENE_ADVANCE_FLAG_SET_TAP"
        || name == "SPIDEY_ORIGIN_MENU_ATTACH_TAP"
        || name.starts_with("SPIDEY_EVENT_DISPATCH_")
        || name.starts_with("SPIDEY_EVENT_LISTENER_")
        || name.starts_with("SPIDEY_SIGNAL_")
        || name.starts_with("SPIDEY_SCENE118_")
        || name == "SPIDEY_BOOT_SEQUENCE_TAP"
        || name == "SPIDEY_SCENE_WARNING_TAP"
        || name == "SPIDEY_SCENE_DYNAMIC_TAP"
        || name == "SPIDEY_SCENE_LEGAL_TAP"
        || name == "SPIDEY_SCENE_START_TAP"
        || name == "SPIDEY_SCENE_LOADER_ENTRY_TAP"
        || name == "SPIDEY_LOADER_GATE1_CALL_TAP"
        || name == "SPIDEY_LOADER_GATE1_RET_TAP"
        || name == "SPIDEY_LOADER_GATE2_CALL_TAP"
        || name == "SPIDEY_LOADER_GATE2_RET_TAP"
        || name == "SPIDEY_LOADER_SYNC_FALLBACK_TAP"
        || name == "SPIDEY_LEGAL_GUARD_ENTRY_TAP"
        || name == "SPIDEY_SCENE118_LOOP_TAP"
        || name.starts_with("SPIDEY_F0690_")
        || name.starts_with("SPIDEY_E9D00_")
        || name == "SPIDEY_AFTER_LEGAL_LOAD_TAP"
        || name == "SPIDEY_AFTER_LEGAL_CHECK_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_LEGALBOX_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_19B10_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_D3D_BATCH_TAP"
        || name == "SPIDEY_BOOT_EDI_LOOP_BODY_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_LEGAL_CLEANUP_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_MOVIE_CHAIN_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_ACTIVISION_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_LOGO_CALLS_TAP"
        || name == "SPIDEY_BOOT_EDI_PRE_START_BRANCH_TAP"
        || name == "SPIDEY_BOOT_EDI_PRE_START_LOAD_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_START_GUARD_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_SETTER_BATCH_TAP"
        || name == "SPIDEY_BOOT_EDI_AFTER_E9140_TAP"
        || name == "SPIDEY_MOVIE_HELPER_ENTRY_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_EA580_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_STRCMP1_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_STRCMP2_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_2B4B20_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_AUDIO_INIT_TAP"
        || name == "SPIDEY_MOVIE_HELPER_BEFORE_COPY_SAVE_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_STRING_COPY_TAP"
        || name == "SPIDEY_MOVIE_HELPER_BEFORE_UPDATE_TAP"
        || name == "SPIDEY_MOVIE_HELPER_AFTER_UPDATE_TAP"
        || name == "SPIDEY_MOVIE_HELPER_BEFORE_RET_TAP"
        || name == "SPIDEY_EA580_ENTRY_TAP"
        || name == "SPIDEY_EA580_RUNTIME_PATH_TAP"
        || name == "SPIDEY_EA580_BEFORE_STRCMP_TAP"
        || name == "SPIDEY_EA580_AFTER_STRCMP_TAP"
        || name == "SPIDEY_EA580_BEFORE_RET_TAP"
        || name == "SPIDEY_EA580_INNER_ENTRY_TAP"
        || name == "SPIDEY_EA580_INNER_BEFORE_POP_EDI_TAP"
        || name == "SPIDEY_EA580_INNER_AFTER_POP_EDI_TAP"
        || name == "SPIDEY_STRCMP_ENTRY_TAP"
        || name == "SPIDEY_STRCMP_FALLBACK_JMP_TAP"
        || name == "SPIDEY_STRCMP_FAST_PATH_TAP"
        || name == "SPIDEY_STRCMP_AFTER_PUSH_EDI_TAP"
        || name == "SPIDEY_STRCMP_BEFORE_POP_EDI_TAP"
        || name == "SPIDEY_STRCMP_AFTER_POP_EDI_TAP"
        || name == "SPIDEY_STRCMP_BEFORE_RET_TAP"
        || name == "SPIDEY_STRCMP_FB_ENTRY_TAP"
        || name == "SPIDEY_STRCMP_FB_AFTER_PUSH_EDI_TAP"
        || name == "SPIDEY_STRCMP_FB_BEFORE_POP_EBX_TAP"
        || name == "SPIDEY_STRCMP_FB_BEFORE_POP_ESI_TAP"
        || name == "SPIDEY_STRCMP_FB_BEFORE_POP_EDI_TAP"
        || name == "SPIDEY_STRCMP_FB_AFTER_POP_EDI_TAP"
        || name == "SPIDEY_STRCMP_FB_BEFORE_RET_TAP"
        || name == "SPIDEY_LEGALBOX_LOADER_TAP"
        || name == "SPIDEY_LEGALBOX_CACHE_RET_TAP"
        || name == "SPIDEY_LEGALBOX_LOAD_RET_TAP"
        || name == "SPIDEY_REGISTER_CALL1_TAP"
        || name == "SPIDEY_REGISTER_CALL2_TAP"
        || name == "SPIDEY_FSM_ADVANCE_ENTRY_TAP"
        || name == "SPIDEY_FSM_READY_STAGE_TAP"
        || name == "SPIDEY_SCENE_RENDER_ENABLE_WRITE_TAP"
        || name == "SPIDEY_SCENE_F7D30_ENTRY_TAP"
        || name == "SPIDEY_SCENE_F7EC0_ENTRY_TAP"
        || name.starts_with("SPIDEY_SCENE_F27")
        || name.starts_with("SPIDEY_SCENE_F28")
        || name.starts_with("SPIDEY_SCENE_F29")
        || name.starts_with("SPIDEY_SCENE_ED")
        || name.starts_with("SPIDEY_SCENE_F10")
        || name.starts_with("SPIDEY_SCENE_F11")
        || name.starts_with("SPIDEY_SCENE_F12")
        || name.starts_with("SPIDEY_SCENE_F7D")
        || name.starts_with("SPIDEY_SCENE_F7F")
        || name.starts_with("SPIDEY_SCRIPT_51C00")
        || name.starts_with("SPIDEY_SCRIPT_NATIVE_")
        || name.starts_with("SPIDEY_SCRIPT_VM_")
        || name.starts_with("SPIDEY_STRING_")
        || name.starts_with("SPIDEY_NATIVE_METHOD_")
        || name.starts_with("SPIDEY_XBS_LIFECYCLE_")
        || name == "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP"
        || name == "SPIDEY_SCENE_DD130_CALL_TAP"
        || name == "SPIDEY_SCENE_AFTER_DD130_TAP"
        || name.starts_with("SPIDEY_DD130_")
        || name == "SPIDEY_SCENE_DIRTY_WRITE_TAP"
        || name == "SPIDEY_XGRAPH_DIRTY_PROMOTE_TAP"
        || name == "SPIDEY_FSM_WRITE_1_TAP"
        || name == "SPIDEY_FSM_WRITE_2_TAP"
        || name == "SPIDEY_GAMEMAIN_FRAME_CTOR_CALL_TAP"
        || name == "SPIDEY_FRAME_CTOR_ENTRY_TAP"
        || name == "SPIDEY_FRAME_BODY_ENTRY_TAP"
        || name == "SPIDEY_FRAME_BODY_RET_TAP"
        || name == "SPIDEY_FRAME_STORE_TAP"
        || name == "SPIDEY_FRAME_STORE_ZERO_TAP"
        || name == "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP"
        || name == "SPIDEY_FRAME_RENDER_PREP_TAP"
        || name == "SPIDEY_FRAME_RENDER_CALL_TAP"
        || name == "SPIDEY_FRAME_RENDER_ENTRY_TAP"
        || name == "SPIDEY_FRAME_RENDER_RET_TAP"
        || name == "SPIDEY_RENDER_BODY_READY_RET_TAP"
        || name == "SPIDEY_RENDER_BODY_SCENE_CALL_TAP"
        || name == "SPIDEY_RENDER_BODY_AFTER_SCENE_TAP"
        || name == "SPIDEY_RENDER_BODY_POST_SCENE_TAP"
        || name == "SPIDEY_RENDER_BODY_TAIL_CALL_TAP"
        || name == "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP"
        || name == "SPIDEY_INPUT_CB_ENTRY_TAP"
        || name == "SPIDEY_INPUT_CB_XINPUT_CALL_TAP"
        || name.starts_with("SPIDEY_SELECT_PRESSED_")
        || name.starts_with("SPIDEY_XBS_PARSE_")
        || name.starts_with("SPIDEY_FLAG17F_")
        || name.starts_with("SPIDEY_ROOT_WRITE_")
        || name.starts_with("SPIDEY_TRIGGER_")
        || name == "SPIDEY_ACTION_GATE_ENTRY_TAP"
        || name == "SPIDEY_ACTION_GATE_SELECTED_TAP"
        || name == "SPIDEY_ACTION_GATE_FALSE_TAP"
        || name == "SPIDEY_ACTION_GATE_TRUE_TAP"
        || name == "DOOM_GAMESTATE_OUTER_ENTRY_TAP"
        || name == "DOOM_GAMESTATE_INNER_ENTRY_TAP"
        || name == "DOOM_ALLOC_RET_TAP"
    {
        use std::sync::atomic::{AtomicU32, Ordering as AO};

        fn read_c_string(r15: u64, addr: u32, max_len: usize) -> String {
            let mut bytes = Vec::with_capacity(max_len.min(64));
            for i in 0..max_len {
                let b = unsafe { *((r15 + addr as u64 + i as u64) as *const u8) };
                if b == 0 {
                    break;
                }
                if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                    bytes.push(b);
                } else {
                    break;
                }
            }
            String::from_utf8_lossy(&bytes).into_owned()
        }

        fn valid_guest_ptr(addr: u32) -> bool {
            addr != 0 && addr < 0x2000_0000
        }

        fn read_guest_u32(r15: u64, addr: u32) -> u32 {
            if valid_guest_ptr(addr) {
                unsafe { *((r15 + addr as u64) as *const u32) }
            } else {
                0
            }
        }

        fn spidey_section(addr: u32) -> &'static str {
            match addr {
                0x0001_1000..=0x002E_BD9F => ".text",
                0x002E_BDA0..=0x0030_38FF => "D3D",
                0x0030_3900..=0x0032_5EFF => "D3DX",
                0x0032_5F00..=0x0033_AE7F => "XGRPH",
                0x0033_AE80..=0x0035_C1BF => "DSOUND",
                0x0035_C1C0..=0x0037_9BDF => "BINK",
                0x0037_9BE0..=0x0038_16DF => "XPP",
                0x0038_16E0..=0x003D_AFFF => ".rdata",
                0x003D_B000..=0x007E_1C7F => ".data",
                _ => "unknown",
            }
        }

        fn signal_active_list_info(
            r15: u64,
            signal: u32,
        ) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
            let list_ptr = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal.wrapping_add(0x10))
            } else {
                0
            };
            let sentinel = if valid_guest_ptr(list_ptr) {
                read_guest_u32(r15, list_ptr)
            } else {
                0
            };
            let first = if valid_guest_ptr(sentinel) {
                read_guest_u32(r15, sentinel)
            } else {
                0
            };
            let first_prev = if valid_guest_ptr(first) {
                read_guest_u32(r15, first.wrapping_add(0x04))
            } else {
                0
            };
            let second = if valid_guest_ptr(first) {
                read_guest_u32(r15, first)
            } else {
                0
            };
            let first_cb = if valid_guest_ptr(first) && first != sentinel {
                read_guest_u32(r15, first.wrapping_add(0x08))
            } else {
                0
            };
            let first_cb_vt = if valid_guest_ptr(first_cb) {
                read_guest_u32(r15, first_cb)
            } else {
                0
            };
            let first_cb_fn = if valid_guest_ptr(first_cb_vt) {
                read_guest_u32(r15, first_cb_vt.wrapping_add(0x08))
            } else {
                0
            };
            let mut count = 0u32;
            let mut cur = first;
            while valid_guest_ptr(cur) && cur != sentinel && count < 32 {
                count += 1;
                cur = read_guest_u32(r15, cur);
            }
            (
                list_ptr,
                sentinel,
                first,
                first_prev,
                second,
                first_cb,
                first_cb_vt,
                first_cb_fn,
                count,
            )
        }

        fn read_guest_u16(r15: u64, addr: u32) -> u16 {
            if valid_guest_ptr(addr) {
                unsafe { *((r15 + addr as u64) as *const u16) }
            } else {
                0
            }
        }

        fn write_guest_u32(r15: u64, addr: u32, value: u32) {
            if valid_guest_ptr(addr) {
                unsafe {
                    std::ptr::write_unaligned((r15 + addr as u64) as *mut u32, value);
                }
            }
        }

        fn write_guest_u16(r15: u64, addr: u32, value: u16) {
            if valid_guest_ptr(addr) {
                unsafe {
                    std::ptr::write_unaligned((r15 + addr as u64) as *mut u16, value);
                }
            }
        }

        fn write_guest_u8(r15: u64, addr: u32, value: u8) {
            if valid_guest_ptr(addr) {
                unsafe {
                    std::ptr::write_unaligned((r15 + addr as u64) as *mut u8, value);
                }
            }
        }

        fn read_guest_u8(r15: u64, addr: u32) -> u8 {
            if valid_guest_ptr(addr) {
                unsafe { *((r15 + addr as u64) as *const u8) }
            } else {
                0
            }
        }

        fn bits_to_f32(bits: u32) -> f32 {
            f32::from_bits(bits)
        }

        fn c_string_probe(r15: u64, addr: u32, max_len: usize) -> (usize, bool, String) {
            if !valid_guest_ptr(addr) {
                return (0, false, String::new());
            }
            let mut len = 0usize;
            let mut preview = Vec::with_capacity(max_len.min(80));
            let mut terminated = false;
            while len < max_len {
                let b = read_guest_u8(r15, addr.wrapping_add(len as u32));
                if b == 0 {
                    terminated = true;
                    break;
                }
                if preview.len() < 80 {
                    preview.push(if b.is_ascii_graphic() || b == b' ' || b == b'\\' {
                        b
                    } else {
                        b'.'
                    });
                }
                len += 1;
            }
            (
                len,
                terminated,
                String::from_utf8_lossy(&preview).into_owned(),
            )
        }

        fn string_desc_summary(r15: u64, desc: u32) -> String {
            if !valid_guest_ptr(desc) {
                return format!("desc=0x{desc:08X}/invalid");
            }
            let data = read_guest_u32(r15, desc);
            let refs = read_guest_u32(r15, desc.wrapping_add(4));
            let len = read_guest_u32(r15, desc.wrapping_add(8));
            let used = read_guest_u32(r15, desc.wrapping_add(0x0C));
            let cap = read_guest_u32(r15, desc.wrapping_add(0x10));
            let (c_len, c_term, text) = c_string_probe(r15, data, 96);
            format!(
                "desc=0x{desc:08X} data=0x{data:08X} refs=0x{refs:08X} len=0x{len:X} used=0x{used:X} cap=0x{cap:X} c_len={} term={} text=\"{}\"",
                c_len, c_term, text
            )
        }

        fn string_pair_summary(r15: u64, pair: u32) -> String {
            if !valid_guest_ptr(pair) {
                return format!("pair=0x{pair:08X}/invalid");
            }
            let desc = read_guest_u32(r15, pair);
            let data_alias = read_guest_u32(r15, pair.wrapping_add(4));
            format!(
                "pair=0x{pair:08X} desc=0x{desc:08X} data_alias=0x{data_alias:08X} {}",
                string_desc_summary(r15, desc)
            )
        }

        if name.starts_with("SPIDEY_STRING_") {
            static SPIDEY_STRING_LOG: AtomicU32 = AtomicU32::new(0);
            let sn = SPIDEY_STRING_LOG.fetch_add(1, AO::Relaxed);
            let eax = (context.Rax & 0xFFFF_FFFF) as u32;
            let ecx = (context.Rcx & 0xFFFF_FFFF) as u32;
            let edx = (context.Rdx & 0xFFFF_FFFF) as u32;
            let ebx = (context.Rbx & 0xFFFF_FFFF) as u32;
            let esi = (context.Rsi & 0xFFFF_FFFF) as u32;
            let edi = (context.Rdi & 0xFFFF_FFFF) as u32;
            let stack0 = read_guest_u32(r15, r14);
            let stack4 = read_guest_u32(r15, r14.wrapping_add(4));
            let stack8 = read_guest_u32(r15, r14.wrapping_add(8));
            let stack_c = read_guest_u32(r15, r14.wrapping_add(0x0C));
            let stack10 = read_guest_u32(r15, r14.wrapping_add(0x10));
            let suspicious = matches!(
                name,
                "SPIDEY_STRING_ALLOC_ENTRY_TAP" if args[0] >= 0x3ff
            ) || matches!(
                name,
                "SPIDEY_STRING_GROW_ENTRY_TAP" if args[0] != 0xFFFF_FFFF && args[0] >= 0x3ff
            ) || stack4 == 0x00AF_0000
                || stack8 == 0x00AF_0000
                || ecx == 0x00AF_0000;
            if sn < 256 || suspicious || sn.is_power_of_two() {
                let detail = match name {
                    "SPIDEY_STRING_GETLINE_ENTRY_TAP" => format!(
                        "file={} buf=0x{:08X} max=0x{:X} delim=0x{:02X} done=0x{:08X}",
                        string_pair_summary(r15, ecx),
                        args[0],
                        args[1],
                        args[2] & 0xFF,
                        args[3]
                    ),
                    "SPIDEY_STRING_ALLOC_ENTRY_TAP" => {
                        let (src_len, src_term, src_text) = c_string_probe(r15, args[1], 128);
                        format!(
                            "alloc_len=0x{:X} src=0x{:08X} src_len={} term={} src=\"{}\"",
                            args[0], args[1], src_len, src_term, src_text
                        )
                    }
                    "SPIDEY_STRING_GROW_ENTRY_TAP" => format!(
                        "this={} requested=0x{:X}",
                        string_pair_summary(r15, ecx),
                        args[0]
                    ),
                    "SPIDEY_STRING_APPEND_LITERAL_ENTRY_TAP" => {
                        let (lit_len, lit_term, lit_text) = c_string_probe(r15, args[0], 128);
                        format!(
                            "dst={} lit=0x{:08X} lit_len={} term={} requested=0x{:X} lit=\"{}\"",
                            string_pair_summary(r15, ecx),
                            args[0],
                            lit_len,
                            lit_term,
                            args[1],
                            lit_text
                        )
                    }
                    "SPIDEY_STRING_APPEND_OBJECT_ENTRY_TAP" => format!(
                        "dst={} src={}",
                        string_pair_summary(r15, ecx),
                        string_pair_summary(r15, args[0])
                    ),
                    "SPIDEY_STRING_FROM_CSTR_ENTRY_TAP" => {
                        let (src_len, src_term, src_text) = c_string_probe(r15, args[0], 128);
                        format!(
                            "dst_pair=0x{:08X} src=0x{:08X} src_len={} term={} src=\"{}\"",
                            ecx, args[0], src_len, src_term, src_text
                        )
                    }
                    "SPIDEY_STRING_PATH_JOIN_ENTRY_TAP" => {
                        let (suffix_len, suffix_term, suffix_text) =
                            c_string_probe(r15, args[1], 96);
                        format!(
                            "dst_pair=0x{:08X} base={} suffix=0x{:08X} suffix_len={} term={} suffix=\"{}\"",
                            ecx,
                            string_pair_summary(r15, args[0]),
                            args[1],
                            suffix_len,
                            suffix_term,
                            suffix_text
                        )
                    }
                    _ => String::new(),
                };
                debug_log(&format!(
                    "[SPIDEY-STRING-PROBE #{}] {} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} {}",
                    sn,
                    name,
                    guest_addr,
                    ret_addr,
                    r14,
                    stack0,
                    stack4,
                    stack8,
                    stack_c,
                    stack10,
                    eax,
                    ecx,
                    edx,
                    ebx,
                    esi,
                    edi,
                    detail
                ));
            }
        }

        if name.starts_with("SPIDEY_FUN29C5A0_") {
            static FUN29C5A0_LOG: AtomicU32 = AtomicU32::new(0);
            let fun_n = FUN29C5A0_LOG.fetch_add(1, AO::Relaxed);
            if fun_n < 64 || fun_n.is_power_of_two() {
                let esp0 = read_guest_u32(r15, r14);
                let esp4 = read_guest_u32(r15, r14.wrapping_add(4));
                let esp8 = read_guest_u32(r15, r14.wrapping_add(8));
                let esp_c = read_guest_u32(r15, r14.wrapping_add(0x0C));
                let b0 = read_guest_u8(r15, guest_addr);
                let b1 = read_guest_u8(r15, guest_addr.wrapping_add(1));
                let b2 = read_guest_u8(r15, guest_addr.wrapping_add(2));
                let b3 = read_guest_u8(r15, guest_addr.wrapping_add(3));
                let b4 = read_guest_u8(r15, guest_addr.wrapping_add(4));
                let call_target = if b0 == 0xE8 {
                    let rel = (b1 as u32)
                        | ((b2 as u32) << 8)
                        | ((b3 as u32) << 16)
                        | ((b4 as u32) << 24);
                    guest_addr.wrapping_add(5).wrapping_add(rel)
                } else {
                    0
                };
                let c_m96 = read_guest_u32(r15, 0x005F_2C68);
                let c_m95 = read_guest_u32(r15, 0x006F_2C98);
                let c_m94 = read_guest_u32(r15, 0x004C_E0F0);
                let c_m93 = read_guest_u32(r15, 0x004D_2AA8);
                let bone_base = read_guest_u32(r15, 0x004D_2ABC);
                let scratch = [
                    read_guest_u32(r15, 0x004D_1520),
                    read_guest_u32(r15, 0x004D_1524),
                    read_guest_u32(r15, 0x004D_1528),
                    read_guest_u32(r15, 0x004D_152C),
                ];
                debug_log(&format!(
                    "[SPIDEY-FUN29C5A0-TAP] #{} {} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} bytes=[{:02X} {:02X} {:02X} {:02X} {:02X}] call_target=0x{:08X} globals m96=0x{:08X} m95=0x{:08X} m94=0x{:08X} m93=0x{:08X} bone_base=0x{:08X} scratch=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] scratch_f=[{:.6},{:.6},{:.6},{:.6}]",
                    fun_n,
                    name,
                    guest_addr,
                    ret_addr,
                    r14,
                    esp0,
                    esp4,
                    esp8,
                    esp_c,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    b0,
                    b1,
                    b2,
                    b3,
                    b4,
                    call_target,
                    c_m96,
                    c_m95,
                    c_m94,
                    c_m93,
                    bone_base,
                    scratch[0],
                    scratch[1],
                    scratch[2],
                    scratch[3],
                    bits_to_f32(scratch[0]),
                    bits_to_f32(scratch[1]),
                    bits_to_f32(scratch[2]),
                    bits_to_f32(scratch[3]),
                ));
            }
        }

        fn spidey_event_slot_summary(r15: u64, slots: u32, count: u32) -> String {
            let keys = [
                0u32, 1, 6, 7, 0x0A, 0x0C, 0x0E, 0x10, 0x12, 0x14, 0x16, 0x1A,
            ];
            let mut s = String::new();
            for key in keys {
                if !s.is_empty() {
                    s.push(' ');
                }
                let slot = if valid_guest_ptr(slots) && key < count.min(0x400) {
                    read_guest_u32(r15, slots.wrapping_add(key.wrapping_mul(4)))
                } else {
                    0
                };
                s.push_str(&format!("{:02X}:0x{:08X}", key, slot));
            }
            s
        }

        fn spidey_event_active_slot_summary(r15: u64, slots: u32, count: u32) -> String {
            if !valid_guest_ptr(slots) || count == 0 {
                return "<none>".to_string();
            }

            let mut s = String::new();
            for key in 0u32..count.min(64) {
                let slot = read_guest_u32(r15, slots.wrapping_add(key.wrapping_mul(4)));
                if slot == 0 {
                    continue;
                }
                let vt = if valid_guest_ptr(slot) {
                    read_guest_u32(r15, slot)
                } else {
                    0
                };
                let listener_0c = if valid_guest_ptr(slot) {
                    read_guest_u32(r15, slot.wrapping_add(0x0C))
                } else {
                    0
                };
                let listener_20 = if valid_guest_ptr(slot) {
                    read_guest_u32(r15, slot.wrapping_add(0x20))
                } else {
                    0
                };
                let listener_0c_string = if valid_guest_ptr(listener_0c) {
                    read_c_string(r15, listener_0c, 40)
                } else {
                    String::new()
                };
                let listener_20_string = if valid_guest_ptr(listener_20) {
                    read_c_string(r15, listener_20, 40)
                } else {
                    String::new()
                };
                if !s.is_empty() {
                    s.push(' ');
                }
                s.push_str(&format!(
                    "{:02X}:0x{:08X}/vt=0x{:08X}/0c='{}'/20='{}'",
                    key, slot, vt, listener_0c_string, listener_20_string
                ));
            }
            if s.is_empty() {
                "<none>".to_string()
            } else {
                s
            }
        }

        fn spidey_event_slot_last(key: u32) -> Option<&'static std::sync::atomic::AtomicU32> {
            match key {
                0 => Some(&SPIDEY_EVENT_SLOT_00_LAST),
                1 => Some(&SPIDEY_EVENT_SLOT_01_LAST),
                6 => Some(&SPIDEY_EVENT_SLOT_06_LAST),
                7 => Some(&SPIDEY_EVENT_SLOT_07_LAST),
                0x0A => Some(&SPIDEY_EVENT_SLOT_0A_LAST),
                0x0C => Some(&SPIDEY_EVENT_SLOT_0C_LAST),
                0x0E => Some(&SPIDEY_EVENT_SLOT_0E_LAST),
                0x10 => Some(&SPIDEY_EVENT_SLOT_10_LAST),
                0x12 => Some(&SPIDEY_EVENT_SLOT_12_LAST),
                0x14 => Some(&SPIDEY_EVENT_SLOT_14_LAST),
                0x16 => Some(&SPIDEY_EVENT_SLOT_16_LAST),
                0x1A => Some(&SPIDEY_EVENT_SLOT_1A_LAST),
                _ => None,
            }
        }

        fn spidey_xapi_device_summary(r15: u64) -> String {
            let pad = 0x0037_9C00u32;
            let mu = 0x0037_9C70u32;
            format!(
                "pad(cur/chg/prev)=0x{:X}/0x{:X}/0x{:X} mu(cur/chg/prev)=0x{:X}/0x{:X}/0x{:X}",
                read_guest_u32(r15, pad),
                read_guest_u32(r15, pad.wrapping_add(4)),
                read_guest_u32(r15, pad.wrapping_add(8)),
                read_guest_u32(r15, mu),
                read_guest_u32(r15, mu.wrapping_add(4)),
                read_guest_u32(r15, mu.wrapping_add(8))
            )
        }

        if name.starts_with("SPIDEY_ROOT_WRITE_") {
            static ROOT_WRITE_LOG: AtomicU32 = AtomicU32::new(0);
            let root_n = ROOT_WRITE_LOG.fetch_add(1, AO::Relaxed);

            let base = context.Rsi as u32;
            let pre_1a4 = read_guest_u32(r15, base.wrapping_add(0x1A4));
            let pre_1a8 = read_guest_u32(r15, base.wrapping_add(0x1A8));
            let eax = context.Rax as u32;
            let ebx = context.Rbx as u32;
            let ecx = context.Rcx as u32;
            let edi = context.Rdi as u32;

            let (new_1a4, new_1a8) = if name == "SPIDEY_ROOT_WRITE_367D4_TAP" {
                (ecx, ebx)
            } else if name == "SPIDEY_ROOT_WRITE_B81BF_TAP" {
                (ebx, eax)
            } else if name == "SPIDEY_ROOT_WRITE_1018CB_TAP" {
                (edi, edi)
            } else if name == "SPIDEY_ROOT_WRITE_12CBDD_TAP" {
                (ebx, ebx)
            } else if name == "SPIDEY_ROOT_WRITE_130C7D_TAP" {
                (eax, pre_1a8)
            } else if name == "SPIDEY_ROOT_WRITE_130CCE_TAP" {
                (pre_1a4, eax)
            } else if name == "SPIDEY_ROOT_WRITE_138CD0_TAP" {
                (pre_1a4, ebx)
            } else if name == "SPIDEY_ROOT_WRITE_138F8A_TAP" {
                (ebx, pre_1a8)
            } else {
                (pre_1a4, pre_1a8)
            };

            let engine = read_guest_u32(r15, 0x004B_C614);
            let frame = read_guest_u32(r15, 0x004B_C630);
            let frame_scene = if valid_guest_ptr(frame) {
                read_guest_u32(r15, frame.wrapping_add(0x18))
            } else {
                0
            };
            let engine_root = if valid_guest_ptr(engine) {
                read_guest_u32(r15, engine.wrapping_add(0x28))
            } else {
                0
            };
            let frame_root = if valid_guest_ptr(frame_scene) {
                read_guest_u32(r15, frame_scene.wrapping_add(0x28))
            } else {
                0
            };
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let scene_lc = scene_name.to_ascii_lowercase();
            let interesting_scene = scene_lc == "bonus\\menu" || scene_lc.starts_with("levels\\");

            if root_n < 96
                || base == engine_root
                || base == frame_root
                || base == engine
                || base == frame_scene
                || interesting_scene
            {
                debug_log(&format!(
                    "[SPIDEY-ROOT-WRITE #{}] {} base=0x{:08X} pre+1A4/1A8=0x{:08X}/0x{:08X} new+1A4/1A8=0x{:08X}/0x{:08X} engine=0x{:08X} engine_root=0x{:08X} frame=0x{:08X} frame_scene=0x{:08X} frame_root=0x{:08X} matches(engine/root/frame_scene/root)={}/{}/{}/{} regs eax/ebx/ecx/edx/esi/edi=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} scene='{}' stash='{}'",
                    root_n,
                    name,
                    base,
                    pre_1a4,
                    pre_1a8,
                    new_1a4,
                    new_1a8,
                    engine,
                    engine_root,
                    frame,
                    frame_scene,
                    frame_root,
                    base == engine,
                    base == engine_root,
                    base == frame_scene,
                    base == frame_root,
                    eax,
                    ebx,
                    ecx,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    edi,
                    scene_name,
                    stash_name
                ));
            }
        }

        fn decode_x87_ext80(low: u64, high: i64) -> f64 {
            let sign_exp = (high as u64 & 0xFFFF) as u16;
            let sign = if (sign_exp & 0x8000) != 0 { -1.0 } else { 1.0 };
            let exp = (sign_exp & 0x7FFF) as i32;
            if exp == 0 && low == 0 {
                return sign * 0.0;
            }
            if exp == 0x7FFF {
                return if low == 0x8000_0000_0000_0000 {
                    sign * f64::INFINITY
                } else {
                    f64::NAN
                };
            }

            let mantissa = (low as f64) / 9_223_372_036_854_775_808.0;
            let unbiased = if exp == 0 { 1 - 16383 } else { exp - 16383 };
            sign * mantissa * 2.0f64.powi(unbiased)
        }

        fn x87_st0_from_context(
            context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
        ) -> (f64, f64, u16, u8) {
            let fp = unsafe { &context.Anonymous.FltSave };
            let top = ((fp.StatusWord >> 11) & 7) as u8;
            let st0 = decode_x87_ext80(fp.FloatRegisters[0].Low, fp.FloatRegisters[0].High);
            let top_reg = fp.FloatRegisters[top as usize];
            let st_top = decode_x87_ext80(top_reg.Low, top_reg.High);
            (st0, st_top, fp.StatusWord, top)
        }

        fn dump_spidey_action_bindings(r15: u64, mgr: u32, action: u32) -> String {
            if !valid_guest_ptr(mgr) {
                return format!("a{:02X}:mgr-null", action);
            }

            let tree = mgr.wrapping_add(0x14);
            let sentinel = read_guest_u32(r15, tree);
            if !valid_guest_ptr(sentinel) {
                return format!("a{:02X}:tree-null", action);
            }

            let mut candidate = sentinel;
            let mut node = read_guest_u32(r15, sentinel.wrapping_add(0x04));
            for _ in 0..64 {
                if !valid_guest_ptr(node) || node == sentinel {
                    break;
                }
                let key = read_guest_u32(r15, node.wrapping_add(0x10));
                candidate = node;
                node = if key < action {
                    read_guest_u32(r15, node.wrapping_add(0x0C))
                } else {
                    read_guest_u32(r15, node.wrapping_add(0x08))
                };
            }

            if !valid_guest_ptr(candidate) || candidate == sentinel {
                return format!("a{:02X}:missing", action);
            }
            let key = read_guest_u32(r15, candidate.wrapping_add(0x10));
            if key != action {
                return format!(
                    "a{:02X}:missing cand=0x{:08X}/0x{:X}",
                    action, candidate, key
                );
            }

            let list = read_guest_u32(r15, candidate.wrapping_add(0x1C));
            if !valid_guest_ptr(list) {
                return format!("a{:02X}:node=0x{:08X} list-null", action, candidate);
            }

            let mut entries = String::new();
            let mut cur = read_guest_u32(r15, list);
            let mut count = 0u32;
            while valid_guest_ptr(cur) && cur != list && count < 8 {
                if !entries.is_empty() {
                    entries.push(',');
                }
                entries.push_str(&format!(
                    "0x{:08X}{{p{} id=0x{:X} arg=0x{:X}}}",
                    cur,
                    read_guest_u32(r15, cur.wrapping_add(0x08)),
                    read_guest_u32(r15, cur.wrapping_add(0x0C)),
                    read_guest_u32(r15, cur.wrapping_add(0x10))
                ));
                cur = read_guest_u32(r15, cur);
                count += 1;
            }

            format!(
                "a{:02X}:node=0x{:08X} key=0x{:X} list=0x{:08X} n={} [{}]",
                action, candidate, key, list, count, entries
            )
        }

        static TAP_PROBE_LOG: AtomicU32 = AtomicU32::new(0);
        let n = TAP_PROBE_LOG.fetch_add(1, AO::Relaxed);
        let xbs_parse_tap = name.starts_with("SPIDEY_XBS_PARSE_");
        let xbs_parse_record_tap = matches!(
            name,
            "SPIDEY_XBS_PARSE_RECORD_ALLOC_RET_TAP" | "SPIDEY_XBS_PARSE_RECORD_REGISTER_RET_TAP"
        );
        let xbs_parse_verbose = std::env::var_os("RUSTEMU_SPIDEY_XBS_PARSE_VERBOSE").is_some();
        let xbs_parse_should_log = if xbs_parse_tap {
            static XBS_PARSE_LOG: AtomicU32 = AtomicU32::new(0);
            static XBS_PARSE_RECORD_LOG: AtomicU32 = AtomicU32::new(0);
            let k = if xbs_parse_record_tap {
                XBS_PARSE_RECORD_LOG.fetch_add(1, AO::Relaxed)
            } else {
                XBS_PARSE_LOG.fetch_add(1, AO::Relaxed)
            };
            xbs_parse_verbose
                || if xbs_parse_record_tap {
                    k < 8 || k.is_power_of_two()
                } else {
                    k < 64 || k.is_power_of_two()
                }
        } else {
            false
        };
        let force_detail = name.starts_with("SPIDEY_MOVIE_HELPER_")
            || name.starts_with("SPIDEY_EA580_")
            || name.starts_with("SPIDEY_STRCMP_")
            || name.starts_with("SPIDEY_FRAME_RENDER_")
            || name.starts_with("SPIDEY_RENDER_BODY_")
            || name.starts_with("SPIDEY_FSM_")
            || name == "SPIDEY_BOOT_GATE_TAP"
            || name.starts_with("SPIDEY_F8580_")
            || name == "SPIDEY_SCENE_ADVANCE_FLAG_SET_TAP"
            || name == "SPIDEY_ORIGIN_MENU_ATTACH_TAP"
            || name == "SPIDEY_SCENE_F7D30_ENTRY_TAP"
            || name == "SPIDEY_SCENE_F7EC0_ENTRY_TAP"
            || name.starts_with("SPIDEY_SCENE_F27")
            || name.starts_with("SPIDEY_SCENE_F28")
            || name.starts_with("SPIDEY_SCENE_F29")
            || name.starts_with("SPIDEY_SCENE_ED")
            || name.starts_with("SPIDEY_SCENE_F10")
            || name.starts_with("SPIDEY_SCENE_F11")
            || name.starts_with("SPIDEY_SCENE_F12")
            || name.starts_with("SPIDEY_SCENE_F7D")
            || name.starts_with("SPIDEY_SCENE_F7F")
            || name.starts_with("SPIDEY_SCRIPT_51C00")
            || name.starts_with("SPIDEY_SCRIPT_NATIVE_")
            || name.starts_with("SPIDEY_SCRIPT_VM_")
            || name.starts_with("SPIDEY_NATIVE_METHOD_")
            || name.starts_with("SPIDEY_XBS_LIFECYCLE_")
            || name.starts_with("SPIDEY_SELECT_PRESSED_")
            || name.starts_with("SPIDEY_KEYPRESS_TABLE_")
            || name.starts_with("SPIDEY_EVENT_DISPATCH_")
            || name.starts_with("SPIDEY_EVENT_LISTENER_")
            || name.starts_with("SPIDEY_SIGNAL_")
            || (xbs_parse_tap && xbs_parse_should_log)
            || name.starts_with("SPIDEY_FLAG17F_")
            || name.starts_with("SPIDEY_TRIGGER_")
            || name == "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP"
            || name.starts_with("SPIDEY_F0690_")
            || name.starts_with("SPIDEY_E9D00_")
            || name == "SPIDEY_SCENE_DD130_CALL_TAP"
            || name == "SPIDEY_SCENE_AFTER_DD130_TAP"
            || name.starts_with("SPIDEY_DD130_")
            || name == "SPIDEY_SCENE_DIRTY_WRITE_TAP"
            || name == "SPIDEY_XGRAPH_DIRTY_PROMOTE_TAP"
            || name == "SPIDEY_SCENE_RENDER_ENABLE_WRITE_TAP"
            || name == "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP"
            || name == "SPIDEY_BOOT_EDI_AFTER_ACTIVISION_TAP";
        if force_detail || n < 80 || n.is_power_of_two() {
            let fsm = unsafe { *((r15 + 0x0072_6690) as *const u32) };
            let app = unsafe { *((r15 + 0x003F_5EB0) as *const u32) };
            let scene = unsafe { *((r15 + 0x003F_5BEC) as *const u32) };
            let engine = unsafe { *((r15 + 0x004B_C614) as *const u32) };
            let frame = unsafe { *((r15 + 0x004B_C630) as *const u32) };
            let scene_mgr_from_frame = if valid_guest_ptr(frame) {
                read_guest_u32(r15, frame.wrapping_add(0x18))
            } else {
                0
            };
            let scene_gate_186 = read_guest_u8(r15, scene_mgr_from_frame.wrapping_add(0x186));
            let fsm_state_ptr = fsm;
            let fsm_state_word0 = if fsm_state_ptr >= 0x1000 && valid_guest_ptr(fsm_state_ptr) {
                read_guest_u32(r15, fsm_state_ptr)
            } else {
                0
            };
            let callback_3f5a48 = unsafe { *((r15 + 0x003F_5A48) as *const u32) };
            // Localization manager pointer — read for legalbox loader probe
            // (may also be useful diagnostic for other TAP sites).
            let loc_mgr = unsafe { *((r15 + 0x004C_20E4) as *const u32) };
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let stack0 = unsafe { *((r15 + r14 as u64) as *const u32) };
            let stack1 = unsafe { *((r15 + r14 as u64 + 4) as *const u32) };
            let stack2 = unsafe { *((r15 + r14 as u64 + 8) as *const u32) };
            let stack0_s = if stack0 < 0x0100_0000 {
                read_c_string(r15, stack0, 64)
            } else {
                String::new()
            };
            let stack1_s = if stack1 < 0x0100_0000 {
                read_c_string(r15, stack1, 64)
            } else {
                String::new()
            };
            let stack2_s = if stack2 < 0x0100_0000 {
                read_c_string(r15, stack2, 64)
            } else {
                String::new()
            };
            if name.starts_with("SPIDEY_XBS_LIFECYCLE_") {
                static SPIDEY_XBS_LIFECYCLE_TAP_LOG: AtomicU32 = AtomicU32::new(0);
                let ln = SPIDEY_XBS_LIFECYCLE_TAP_LOG.fetch_add(1, AO::Relaxed);
                if ln < 192
                    || ln.is_power_of_two()
                    || scene_name.contains("origin_z")
                    || stash_name.contains("origin_z")
                    || name == "SPIDEY_XBS_LIFECYCLE_PRELOAD_APPEND_ENTRY_TAP"
                {
                    let read_owner_script =
                        |owner: u32| -> (u32, u32, u32, u32, u32, u32, u32, u32, String) {
                            let script_mgr = read_guest_u32(r15, owner.wrapping_add(0x1A0));
                            let script_ready = read_guest_u32(r15, owner.wrapping_add(0x1AC));
                            let pause_obj = read_guest_u32(r15, owner.wrapping_add(0x1A4));
                            let focus_obj = read_guest_u32(r15, owner.wrapping_add(0x1A8));
                            let table = read_guest_u32(r15, script_mgr.wrapping_add(0x20));
                            let head = read_guest_u32(r15, script_mgr.wrapping_add(0x10));
                            let first = read_guest_u32(r15, head);
                            let first_item = read_guest_u32(r15, first.wrapping_add(0x08));
                            let script_file = read_c_string(r15, owner.wrapping_add(0x1B4), 96);
                            (
                                script_mgr,
                                script_ready,
                                pause_obj,
                                focus_obj,
                                table,
                                head,
                                first,
                                first_item,
                                script_file,
                            )
                        };
                    let global_script_mgr = read_guest_u32(r15, 0x004B_8F04);
                    let global_script_ready = read_guest_u32(r15, 0x004B_8F08);
                    let global_active_script = read_guest_u32(r15, 0x004B_8F0C);
                    let global_table = read_guest_u32(r15, global_script_mgr.wrapping_add(0x20));
                    let global_head = read_guest_u32(r15, global_script_mgr.wrapping_add(0x10));
                    let global_first = read_guest_u32(r15, global_head);
                    let global_first_item = read_guest_u32(r15, global_first.wrapping_add(0x08));
                    let xroot = read_guest_u32(r15, 0x004C_06B8);
                    let hero0 = read_guest_u32(r15, xroot.wrapping_add(0x134));
                    let hero1 = read_guest_u32(r15, xroot.wrapping_add(0x138));
                    let hero2 = read_guest_u32(r15, xroot.wrapping_add(0x13C));
                    let hero3 = read_guest_u32(r15, xroot.wrapping_add(0x140));
                    let hero_count = read_guest_u32(r15, xroot.wrapping_add(0x154));
                    let active_hero = read_guest_u32(r15, xroot.wrapping_add(0x1E4));
                    let live_scene = if valid_guest_ptr(frame) {
                        read_guest_u32(r15, frame.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let (
                        ecx_mgr,
                        ecx_ready,
                        ecx_pause,
                        ecx_focus,
                        ecx_table,
                        ecx_head,
                        ecx_first,
                        ecx_item,
                        ecx_script_file,
                    ) = read_owner_script(context.Rcx as u32);
                    let (
                        esi_mgr,
                        esi_ready,
                        esi_pause,
                        esi_focus,
                        esi_table,
                        esi_head,
                        esi_first,
                        esi_item,
                        esi_script_file,
                    ) = read_owner_script(context.Rsi as u32);
                    let (
                        xroot_mgr,
                        xroot_ready,
                        xroot_pause,
                        xroot_focus,
                        xroot_table,
                        xroot_head,
                        xroot_first,
                        xroot_item,
                        xroot_script_file,
                    ) = read_owner_script(xroot);
                    let read_auto_triplet = |entry: u32| -> (u32, u32, u32, String) {
                        let entity = read_guest_u32(r15, entry);
                        let name_word = read_guest_u32(r15, entry.wrapping_add(0x04));
                        let extra = read_guest_u32(r15, entry.wrapping_add(0x08));
                        let text = read_c_string(r15, entry.wrapping_add(0x04), 64);
                        (entity, name_word, extra, text)
                    };
                    let read_object_meta = |obj: u32| -> (u32, u32, u32, u32) {
                        if !valid_guest_ptr(obj) {
                            return (0, 0, 0, 0);
                        }
                        (
                            read_guest_u32(r15, obj),
                            read_guest_u32(r15, obj.wrapping_add(0x7C)),
                            read_guest_u32(r15, obj.wrapping_add(0x70)),
                            read_guest_u32(r15, obj.wrapping_add(0xC0)),
                        )
                    };
                    let ecx_auto_begin =
                        read_guest_u32(r15, (context.Rcx as u32).wrapping_add(0x408));
                    let ecx_auto_end =
                        read_guest_u32(r15, (context.Rcx as u32).wrapping_add(0x40C));
                    let esi_auto_begin =
                        read_guest_u32(r15, (context.Rsi as u32).wrapping_add(0x408));
                    let esi_auto_end =
                        read_guest_u32(r15, (context.Rsi as u32).wrapping_add(0x40C));
                    let xroot_auto_begin = read_guest_u32(r15, xroot.wrapping_add(0x408));
                    let xroot_auto_end = read_guest_u32(r15, xroot.wrapping_add(0x40C));
                    let (iter_ent, iter_name, iter_extra, iter_text) =
                        read_auto_triplet(context.Rsi as u32);
                    let (begin_ent, begin_name, begin_extra, begin_text) =
                        read_auto_triplet(ecx_auto_begin);
                    let (hero0_vt, hero0_type, hero0_flags70, hero0_c0) = read_object_meta(hero0);
                    let (active_vt, active_type, active_flags70, active_c0) =
                        read_object_meta(active_hero);
                    let (iter_vt, iter_type, iter_flags70, iter_c0) = read_object_meta(iter_ent);
                    let (begin_vt, begin_type, begin_flags70, begin_c0) =
                        read_object_meta(begin_ent);
                    if name == "SPIDEY_XBS_LIFECYCLE_PRELOAD_APPEND_ENTRY_TAP" {
                        static SPIDEY_XBS_APPEND_TAP_LOG: AtomicU32 = AtomicU32::new(0);
                        let an = SPIDEY_XBS_APPEND_TAP_LOG.fetch_add(1, AO::Relaxed);
                        if an < 512
                            || scene_name.contains("origin_z")
                            || stash_name.contains("origin_z")
                            || stack1 == hero0
                            || stack2 == hero0
                            || context.Rcx as u32 == xroot
                        {
                            let (ecx_vt, ecx_type, ecx_flags70, ecx_c0) =
                                read_object_meta(context.Rcx as u32);
                            let (edx_vt, edx_type, edx_flags70, edx_c0) =
                                read_object_meta(context.Rdx as u32);
                            let (stack1_vt, stack1_type, stack1_flags70, stack1_c0) =
                                read_object_meta(stack1);
                            let (stack2_vt, stack2_type, stack2_flags70, stack2_c0) =
                                read_object_meta(stack2);
                            debug_log(&format!(
                                "[SPIDEY-XBS-PRELOAD-APPEND] #{} ecx=0x{:08X} edx=0x{:08X} stack1=0x{:08X} stack2=0x{:08X} stack1_s='{}' stack2_s='{}' scene='{}' stash='{}' xroot=0x{:08X} hero0=0x{:08X} hero_count={} ecx408/40c=0x{:08X}/0x{:08X} xroot408/40c=0x{:08X}/0x{:08X} meta ecx=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} edx=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} stack1=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} stack2=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X}",
                                an + 1,
                                context.Rcx as u32,
                                context.Rdx as u32,
                                stack1,
                                stack2,
                                stack1_s,
                                stack2_s,
                                scene_name,
                                stash_name,
                                xroot,
                                hero0,
                                hero_count,
                                ecx_auto_begin,
                                ecx_auto_end,
                                xroot_auto_begin,
                                xroot_auto_end,
                                ecx_vt,
                                ecx_type,
                                ecx_flags70,
                                ecx_c0,
                                edx_vt,
                                edx_type,
                                edx_flags70,
                                edx_c0,
                                stack1_vt,
                                stack1_type,
                                stack1_flags70,
                                stack1_c0,
                                stack2_vt,
                                stack2_type,
                                stack2_flags70,
                                stack2_c0,
                            ));
                        }
                    }
                    let stack1_file = if valid_guest_ptr(stack1) {
                        read_c_string(r15, stack1.wrapping_add(0x1B4), 96)
                    } else {
                        String::new()
                    };
                    if matches!(
                        name,
                        "SPIDEY_XBS_LIFECYCLE_SCRIPT_RUN_ENTRY_TAP"
                            | "SPIDEY_XBS_LIFECYCLE_SCRIPT_VM_CALL_TAP"
                            | "SPIDEY_XBS_LIFECYCLE_SCRIPT_VM_RET_TAP"
                    ) {
                        static SPIDEY_XBS_VM_OBJECT_LOG: AtomicU32 = AtomicU32::new(0);
                        let vn = SPIDEY_XBS_VM_OBJECT_LOG.fetch_add(1, AO::Relaxed);
                        let script_obj = if name == "SPIDEY_XBS_LIFECYCLE_SCRIPT_RUN_ENTRY_TAP" {
                            stack1
                        } else if name == "SPIDEY_XBS_LIFECYCLE_SCRIPT_VM_CALL_TAP" {
                            context.Rcx as u32
                        } else {
                            context.Rsi as u32
                        };
                        if valid_guest_ptr(script_obj)
                            && (vn < 256
                                || vn.is_power_of_two()
                                || scene_name.contains("origin_z")
                                || stash_name.contains("origin_z"))
                        {
                            let script_v0 = read_guest_u32(r15, script_obj);
                            let script_v4 = read_guest_u32(r15, script_obj.wrapping_add(0x04));
                            let script_flags8 = read_guest_u8(r15, script_obj.wrapping_add(0x08));
                            let script_stack = read_guest_u32(r15, script_obj.wrapping_add(0x14));
                            let script_base = read_guest_u32(r15, script_obj.wrapping_add(0x18));
                            let script_pc = read_guest_u32(r15, script_obj.wrapping_add(0x1C));
                            let script_wait = read_guest_u32(r15, script_obj.wrapping_add(0x2C));
                            let script_timer = read_guest_u32(r15, script_obj.wrapping_add(0x40));
                            let script_counter = read_guest_u32(r15, script_obj.wrapping_add(0x44));
                            let op_word = if valid_guest_ptr(script_pc) {
                                read_guest_u16(r15, script_pc)
                            } else {
                                0
                            };
                            let op_major = op_word >> 8;
                            let op = op_word & 0x007F;
                            let native_command = if op_major == 0x04 && valid_guest_ptr(script_pc) {
                                read_guest_u32(r15, script_pc)
                            } else {
                                0
                            };
                            let native_vt = if valid_guest_ptr(native_command) {
                                read_guest_u32(r15, native_command)
                            } else {
                                0
                            };
                            let native_cb04 = if valid_guest_ptr(native_vt) {
                                read_guest_u32(r15, native_vt.wrapping_add(0x04))
                            } else {
                                0
                            };
                            let native_cb08 = if valid_guest_ptr(native_vt) {
                                read_guest_u32(r15, native_vt.wrapping_add(0x08))
                            } else {
                                0
                            };
                            let native_cb0c = if valid_guest_ptr(native_vt) {
                                read_guest_u32(r15, native_vt.wrapping_add(0x0C))
                            } else {
                                0
                            };
                            let native_name = match native_cb04 {
                                0x0006_1400 => "originz_61400",
                                0x0006_7370 => "originz_67370",
                                0x0006_7460 => "timer_67460",
                                0x0007_8F00 => "originz_78f00",
                                0x0008_5390 => "script_85390",
                                0x0009_3590 => "set_timescale_factor",
                                0x0009_3720 => "started_intro_animation_playing",
                                0x0009_3E60 => "is_scene_anim_playing",
                                0x0009_EB90 => "is_shown",
                                0x0009_ECC0 => "is_faded",
                                0x000A_37C0 => "hide_all_widgets",
                                _ => "-",
                            };
                            let pc_rel = script_pc.wrapping_sub(script_base);
                            let mut pc_bytes = String::new();
                            if valid_guest_ptr(script_pc) {
                                for i in 0..16u32 {
                                    if i != 0 {
                                        pc_bytes.push(' ');
                                    }
                                    pc_bytes.push_str(&format!(
                                        "{:02X}",
                                        read_guest_u8(r15, script_pc.wrapping_add(i))
                                    ));
                                }
                            }
                            if std::env::var_os("RUSTEMU_SPIDEY_XBS_SCRIPT_PROBE").is_some()
                                && name == "SPIDEY_XBS_LIFECYCLE_SCRIPT_RUN_ENTRY_TAP"
                                && (scene_name.to_ascii_lowercase().contains("origin_z")
                                    || stash_name.to_ascii_lowercase().contains("origin_z"))
                                && valid_guest_ptr(script_pc)
                            {
                                static SPIDEY_XBS_BYTECODE_SCAN_LOG: AtomicU32 = AtomicU32::new(0);
                                let scan_log =
                                    SPIDEY_XBS_BYTECODE_SCAN_LOG.fetch_add(1, AO::Relaxed);
                                let should_log_scan = scan_log < 96 || scan_log.is_power_of_two();
                                if should_log_scan {
                                    let script_entry_pc = if valid_guest_ptr(script_v4) {
                                        read_guest_u32(r15, script_v4.wrapping_add(0x28))
                                    } else {
                                        0
                                    };
                                    let scan_start = if valid_guest_ptr(script_entry_pc) {
                                        script_entry_pc
                                    } else {
                                        script_pc
                                    };
                                    let mut scan_pc = scan_start;
                                    let mut steps = 0u32;
                                    let mut invalid_at = 0u32;
                                    let mut first_yield_pc = 0u32;
                                    let mut major_counts = [0u32; 0x2E];
                                    let mut target_counts = [0u32; 6];
                                    let mut hits = String::new();
                                    let mut yields = String::new();
                                    while steps < 1024 && valid_guest_ptr(scan_pc) {
                                        let word = read_guest_u16(r15, scan_pc);
                                        let major = (word >> 8) as u32;
                                        let op = (word & 0x007F) as u32;
                                        if (major as usize) < major_counts.len() {
                                            major_counts[major as usize] =
                                                major_counts[major as usize].saturating_add(1);
                                        }
                                        if (0x28..=0x2D).contains(&major) {
                                            let idx = (major - 0x28) as usize;
                                            target_counts[idx] =
                                                target_counts[idx].saturating_add(1);
                                            if hits.len() < 192 {
                                                if !hits.is_empty() {
                                                    hits.push(' ');
                                                }
                                                hits.push_str(&format!(
                                                    "pc=0x{:08X}:maj=0x{:02X}:op=0x{:02X}",
                                                    scan_pc, major, op
                                                ));
                                            }
                                        }
                                        if major == 0x04 {
                                            if first_yield_pc == 0 {
                                                first_yield_pc = scan_pc;
                                            }
                                            if yields.len() < 192 {
                                                if !yields.is_empty() {
                                                    yields.push(' ');
                                                }
                                                yields.push_str(&format!(
                                                    "pc=0x{:08X}:op=0x{:02X}",
                                                    scan_pc, op
                                                ));
                                            }
                                        }

                                        let mut len = 2u32;
                                        if (word & 0x0080) != 0 {
                                            len = len.saturating_add(2);
                                        }
                                        len = len.saturating_add(match op {
                                            1 | 2 | 3 | 8 | 9 | 10 | 11 => 4,
                                            4 | 5 | 6 | 7 | 15 | 16 => 2,
                                            _ => 0,
                                        });
                                        let next_pc = scan_pc.wrapping_add(len);
                                        if len == 0 || !valid_guest_ptr(next_pc) {
                                            invalid_at = scan_pc;
                                            break;
                                        }
                                        scan_pc = next_pc;
                                        steps = steps.saturating_add(1);
                                    }
                                    debug_log(&format!(
                                        "[SPIDEY-XBS-BYTECODE-SCAN] #{} scene='{}' stash='{}' script=0x{:08X} def=0x{:08X} cur_pc=0x{:08X} entry_pc=0x{:08X} scan_start=0x{:08X} steps={} end=0x{:08X} invalid=0x{:08X} first_yield=0x{:08X} majors[00/04/1D/1E/28/29/2A/2B/2C/2D]={}/{}/{}/{}/{}/{}/{}/{}/{}/{} targets[28..2D]={}/{}/{}/{}/{}/{} hits=[{}] yields=[{}]",
                                        scan_log + 1,
                                        scene_name,
                                        stash_name,
                                        script_obj,
                                        script_v4,
                                        script_pc,
                                        script_entry_pc,
                                        scan_start,
                                        steps,
                                        scan_pc,
                                        invalid_at,
                                        first_yield_pc,
                                        major_counts[0x00],
                                        major_counts[0x04],
                                        major_counts[0x1D],
                                        major_counts[0x1E],
                                        major_counts[0x28],
                                        major_counts[0x29],
                                        major_counts[0x2A],
                                        major_counts[0x2B],
                                        major_counts[0x2C],
                                        major_counts[0x2D],
                                        target_counts[0],
                                        target_counts[1],
                                        target_counts[2],
                                        target_counts[3],
                                        target_counts[4],
                                        target_counts[5],
                                        hits,
                                        yields
                                    ));
                                }
                            }
                            let mut stack_words = String::new();
                            if valid_guest_ptr(script_stack.wrapping_sub(0x10)) {
                                for i in 0..8u32 {
                                    if i != 0 {
                                        stack_words.push(' ');
                                    }
                                    let addr = script_stack.wrapping_sub(0x10).wrapping_add(i * 4);
                                    stack_words
                                        .push_str(&format!("0x{:08X}", read_guest_u32(r15, addr)));
                                }
                            }
                            debug_log(&format!(
                                "[SPIDEY-XBS-VM-SCRIPT] #{} {} scene='{}' stash='{}' script=0x{:08X} v0/v4=0x{:08X}/0x{:08X} flags8=0x{:02X} stack=0x{:08X} base=0x{:08X} pc=0x{:08X} pc_rel=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} bytes=[{}] native=0x{:08X} nvt=0x{:08X} ncb04/08/0c=0x{:08X}/0x{:08X}/0x{:08X} nname={} wait=0x{:08X} timer=0x{:08X} ctr={} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} stack[-10..+0c]=[{}]",
                                vn + 1,
                                name,
                                scene_name,
                                stash_name,
                                script_obj,
                                script_v0,
                                script_v4,
                                script_flags8,
                                script_stack,
                                script_base,
                                script_pc,
                                pc_rel,
                                op_word,
                                op_major,
                                op,
                                pc_bytes,
                                native_command,
                                native_vt,
                                native_cb04,
                                native_cb08,
                                native_cb0c,
                                native_name,
                                script_wait,
                                script_timer,
                                script_counter,
                                context.Rax as u32,
                                context.Rax as u8,
                                context.Rcx as u32,
                                context.Rdx as u32,
                                context.Rsi as u32,
                                stack_words
                            ));
                        }
                    }
                    if name == "SPIDEY_XBS_LIFECYCLE_NATIVE_50D30_ENTRY_TAP"
                        || name == "SPIDEY_XBS_LIFECYCLE_NATIVE_50D30_RET_TAP"
                    {
                        static SPIDEY_XBS_NATIVE_50D30_LOG: AtomicU32 = AtomicU32::new(0);
                        let nn = SPIDEY_XBS_NATIVE_50D30_LOG.fetch_add(1, AO::Relaxed);
                        let is_entry = name == "SPIDEY_XBS_LIFECYCLE_NATIVE_50D30_ENTRY_TAP";
                        let script_frame = if is_entry {
                            context.Rcx as u32
                        } else {
                            context.Rsi as u32
                        };
                        let value_ptr = if is_entry {
                            read_guest_u32(r15, r14.wrapping_add(0x04))
                        } else {
                            read_guest_u32(r15, r14.wrapping_add(0x0C))
                        };
                        let old_pc = if is_entry {
                            read_guest_u32(r15, r14.wrapping_add(0x08))
                        } else {
                            read_guest_u32(r15, r14.wrapping_add(0x10))
                        };
                        let command_obj = read_guest_u32(r15, value_ptr);
                        let command_vt = read_guest_u32(r15, command_obj);
                        let command_cb04 = read_guest_u32(r15, command_vt.wrapping_add(0x04));
                        let command_cb08 = read_guest_u32(r15, command_vt.wrapping_add(0x08));
                        let command_cb0c = read_guest_u32(r15, command_vt.wrapping_add(0x0C));
                        let command_name = match command_cb04 {
                            0x0006_1400 => "wait_frame",
                            0x0006_7370 => "delay",
                            0x0006_7460 => "time_inc",
                            0x0007_8F00 => "timed_anim_78f00",
                            0x0008_5390 => "script_85390",
                            0x0009_3590 => "set_timescale_factor",
                            0x0009_3720 => "started_intro_animation_playing",
                            0x0009_3E60 => "is_scene_anim_playing",
                            0x0009_EB90 => "is_shown",
                            0x0009_ECC0 => "is_faded",
                            0x000A_37C0 => "hide_all_widgets",
                            _ => "-",
                        };
                        let script_stack = read_guest_u32(r15, script_frame.wrapping_add(0x14));
                        let saved_stack = read_guest_u32(r15, script_frame.wrapping_add(0x20));
                        let saved_pc = read_guest_u32(r15, script_frame.wrapping_add(0x28));
                        let wait_flag = read_guest_u32(r15, script_frame.wrapping_add(0x2C));
                        let script_timer = read_guest_u32(r15, script_frame.wrapping_add(0x40));
                        let script_counter = read_guest_u32(r15, script_frame.wrapping_add(0x44));
                        let xroot = read_guest_u32(r15, 0x004C_06B8);
                        let xroot_delta = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x18C))
                        } else {
                            0
                        };
                        let xroot_tick158 = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x158))
                        } else {
                            0
                        };
                        let xroot_tick15c = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x15C))
                        } else {
                            0
                        };
                        let xroot_timescale = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x440))
                        } else {
                            0
                        };
                        let active_obj = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x1E4))
                        } else {
                            0
                        };
                        let active_time = if valid_guest_ptr(active_obj) {
                            read_guest_u32(r15, active_obj.wrapping_add(0x1EC))
                        } else {
                            0
                        };
                        let frame_global = read_guest_u32(r15, 0x004B_C630);
                        let frame_scene = if valid_guest_ptr(frame_global) {
                            read_guest_u32(r15, frame_global.wrapping_add(0x18))
                        } else {
                            0
                        };
                        let frame_scene_root = if valid_guest_ptr(frame_scene) {
                            read_guest_u32(r15, frame_scene.wrapping_add(0x28))
                        } else {
                            0
                        };
                        let frame_scene_delta = if valid_guest_ptr(frame_scene) {
                            read_guest_u32(r15, frame_scene.wrapping_add(0x18C))
                        } else {
                            0
                        };
                        let frame_scene_tick158 = if valid_guest_ptr(frame_scene) {
                            read_guest_u32(r15, frame_scene.wrapping_add(0x158))
                        } else {
                            0
                        };
                        let frame_scene_tick15c = if valid_guest_ptr(frame_scene) {
                            read_guest_u32(r15, frame_scene.wrapping_add(0x15C))
                        } else {
                            0
                        };
                        let stack_m18 = read_guest_u32(r15, script_stack.wrapping_sub(0x18));
                        let stack_m14 = read_guest_u32(r15, script_stack.wrapping_sub(0x14));
                        let stack_m10 = read_guest_u32(r15, script_stack.wrapping_sub(0x10));
                        let stack_m0c = read_guest_u32(r15, script_stack.wrapping_sub(0x0C));
                        let stack_m08 = read_guest_u32(r15, script_stack.wrapping_sub(0x08));
                        let stack_m04 = read_guest_u32(r15, script_stack.wrapping_sub(0x04));
                        let stack_p00 = read_guest_u32(r15, script_stack);
                        let stack_p04 = read_guest_u32(r15, script_stack.wrapping_add(0x04));
                        let stack_p08 = read_guest_u32(r15, script_stack.wrapping_add(0x08));
                        let stack_p0c = read_guest_u32(r15, script_stack.wrapping_add(0x0C));
                        let (
                            anim_base,
                            anim_obj,
                            anim_axis_x,
                            anim_axis_y,
                            anim_axis_z,
                            anim_angle,
                            anim_remaining,
                            anim_d8,
                            anim_rate,
                            anim_mode,
                            anim_ctx,
                            anim_ctx_time,
                            anim_active_match,
                        ) = if command_cb04 == 0x0007_8F00 {
                            let base = if is_entry {
                                script_stack.wrapping_sub(0x18)
                            } else {
                                script_stack
                            };
                            let obj = if valid_guest_ptr(base) {
                                read_guest_u32(r15, base)
                            } else {
                                0
                            };
                            let d8 = if valid_guest_ptr(obj) {
                                read_guest_u32(r15, obj.wrapping_add(0xD8))
                            } else {
                                0
                            };
                            let rate = if valid_guest_ptr(d8) {
                                read_guest_u32(r15, d8.wrapping_add(0x08))
                            } else {
                                0
                            };
                            let mode = if valid_guest_ptr(d8) {
                                read_guest_u32(r15, d8.wrapping_add(0x0C))
                            } else {
                                0
                            };
                            let ctx = read_guest_u32(r15, script_frame.wrapping_add(0x18));
                            let ctx_time = if valid_guest_ptr(ctx) {
                                read_guest_u32(r15, ctx.wrapping_add(0x38))
                            } else {
                                0
                            };
                            (
                                base,
                                obj,
                                read_guest_u32(r15, base.wrapping_add(0x04)),
                                read_guest_u32(r15, base.wrapping_add(0x08)),
                                read_guest_u32(r15, base.wrapping_add(0x0C)),
                                read_guest_u32(r15, base.wrapping_add(0x10)),
                                read_guest_u32(r15, base.wrapping_add(0x14)),
                                d8,
                                rate,
                                mode,
                                ctx,
                                ctx_time,
                                u32::from(obj != 0 && obj == active_obj),
                            )
                        } else {
                            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
                        };
                        let old_word = read_guest_u16(r15, old_pc);
                        let old_major = old_word >> 8;
                        let old_op = old_word & 0x007F;
                        let mut old_bytes = String::new();
                        if valid_guest_ptr(old_pc) {
                            for i in 0..16u32 {
                                if i != 0 {
                                    old_bytes.push(' ');
                                }
                                old_bytes.push_str(&format!(
                                    "{:02X}",
                                    read_guest_u8(r15, old_pc.wrapping_add(i))
                                ));
                            }
                        }
                        let is_origin_z = scene_name.eq_ignore_ascii_case("levels\\origin_z")
                            || stash_name.eq_ignore_ascii_case("M1origin\\origin_z");
                        if nn < 256
                            || nn.is_power_of_two()
                            || is_origin_z
                            || matches!(
                                command_cb04,
                                0x0006_1400 | 0x0006_7370 | 0x0006_7460 | 0x0007_8F00
                            )
                        {
                            debug_log(&format!(
                                "[SPIDEY-XBS-NATIVE-50D30] #{} {} scene='{}' stash='{}' frame=0x{:08X} value_ptr=0x{:08X} old_pc=0x{:08X} old_word=0x{:04X} old_major=0x{:02X} old_op=0x{:02X} old_bytes=[{}] command=0x{:08X} vt=0x{:08X} cb04/08/0c=0x{:08X}/0x{:08X}/0x{:08X} cname={} stack=0x{:08X} saved_stack=0x{:08X} saved_pc=0x{:08X} wait={} timer=0x{:08X} ctr={} xroot=0x{:08X} xdt=0x{:08X}/{:.6} x158/15c=0x{:08X}/0x{:08X} xscale=0x{:08X}/{:.6} active=0x{:08X} active_time=0x{:08X}/{:.6} frame_global=0x{:08X} frame_scene=0x{:08X} froot=0x{:08X} fdt=0x{:08X}/{:.6} f158/15c=0x{:08X}/0x{:08X} stackwin=[0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6}] anim base=0x{:08X} obj=0x{:08X} active_match={} axis=[0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6}] angle=0x{:08X}/{:.6} remain=0x{:08X}/{:.6} d8=0x{:08X} rate=0x{:08X}/{:.6} mode=0x{:08X} ctx=0x{:08X} ctx_time=0x{:08X}/{:.6} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X}",
                                nn + 1,
                                name,
                                scene_name,
                                stash_name,
                                script_frame,
                                value_ptr,
                                old_pc,
                                old_word,
                                old_major,
                                old_op,
                                old_bytes,
                                command_obj,
                                command_vt,
                                command_cb04,
                                command_cb08,
                                command_cb0c,
                                command_name,
                                script_stack,
                                saved_stack,
                                saved_pc,
                                wait_flag,
                                script_timer,
                                script_counter,
                                xroot,
                                xroot_delta,
                                bits_to_f32(xroot_delta),
                                xroot_tick158,
                                xroot_tick15c,
                                xroot_timescale,
                                bits_to_f32(xroot_timescale),
                                active_obj,
                                active_time,
                                bits_to_f32(active_time),
                                frame_global,
                                frame_scene,
                                frame_scene_root,
                                frame_scene_delta,
                                bits_to_f32(frame_scene_delta),
                                frame_scene_tick158,
                                frame_scene_tick15c,
                                stack_m18,
                                bits_to_f32(stack_m18),
                                stack_m14,
                                bits_to_f32(stack_m14),
                                stack_m10,
                                bits_to_f32(stack_m10),
                                stack_m0c,
                                bits_to_f32(stack_m0c),
                                stack_m08,
                                bits_to_f32(stack_m08),
                                stack_m04,
                                bits_to_f32(stack_m04),
                                stack_p00,
                                bits_to_f32(stack_p00),
                                stack_p04,
                                bits_to_f32(stack_p04),
                                stack_p08,
                                bits_to_f32(stack_p08),
                                stack_p0c,
                                bits_to_f32(stack_p0c),
                                anim_base,
                                anim_obj,
                                anim_active_match,
                                anim_axis_x,
                                bits_to_f32(anim_axis_x),
                                anim_axis_y,
                                bits_to_f32(anim_axis_y),
                                anim_axis_z,
                                bits_to_f32(anim_axis_z),
                                anim_angle,
                                bits_to_f32(anim_angle),
                                anim_remaining,
                                bits_to_f32(anim_remaining),
                                anim_d8,
                                anim_rate,
                                bits_to_f32(anim_rate),
                                anim_mode,
                                anim_ctx,
                                anim_ctx_time,
                                bits_to_f32(anim_ctx_time),
                                context.Rax as u32,
                                context.Rax as u8,
                                context.Rcx as u32,
                                context.Rdx as u32,
                                context.Rsi as u32,
                            ));
                        }
                    }
                    debug_log(&format!(
                        "[SPIDEY-XBS-LIFECYCLE] #{} {} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X} ret=0x{:08X} stack1=0x{:08X} stack1_s='{}' stack1_file='{}' scene='{}' stash='{}' engine=0x{:08X} frame=0x{:08X} live_scene=0x{:08X} xroot=0x{:08X} hero count/slots/active={}/0x{:08X},0x{:08X},0x{:08X},0x{:08X}/0x{:08X} objmeta hero0 vt/type/flags70/c0=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} active vt/type/flags70/c0=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} iter vt/type/flags70/c0=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} begin vt/type/flags70/c0=0x{:08X}/0x{:X}/0x{:08X}/0x{:08X} globals mgr/ready/active/table/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} ecx script=0x{:08X}/0x{:08X} pause/focus=0x{:08X}/0x{:08X} table/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} file='{}' esi script=0x{:08X}/0x{:08X} pause/focus=0x{:08X}/0x{:08X} table/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} file='{}' xroot script=0x{:08X}/0x{:08X} pause/focus=0x{:08X}/0x{:08X} table/head/first/item=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} file='{}' auto ecx408/40c=0x{:08X}/0x{:08X} esi408/40c=0x{:08X}/0x{:08X} xroot408/40c=0x{:08X}/0x{:08X} iter[esi]=0x{:08X}/0x{:08X}/0x{:08X} text='{}' begin[ecx+408]=0x{:08X}/0x{:08X}/0x{:08X} text='{}'",
                        ln + 1,
                        name,
                        context.Rax as u32,
                        context.Rbx as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        r14,
                        stack0,
                        stack1,
                        stack1_s,
                        stack1_file,
                        scene_name,
                        stash_name,
                        engine,
                        frame,
                        live_scene,
                        xroot,
                        hero_count,
                        hero0,
                        hero1,
                        hero2,
                        hero3,
                        active_hero,
                        hero0_vt,
                        hero0_type,
                        hero0_flags70,
                        hero0_c0,
                        active_vt,
                        active_type,
                        active_flags70,
                        active_c0,
                        iter_vt,
                        iter_type,
                        iter_flags70,
                        iter_c0,
                        begin_vt,
                        begin_type,
                        begin_flags70,
                        begin_c0,
                        global_script_mgr,
                        global_script_ready,
                        global_active_script,
                        global_table,
                        global_head,
                        global_first,
                        global_first_item,
                        ecx_mgr,
                        ecx_ready,
                        ecx_pause,
                        ecx_focus,
                        ecx_table,
                        ecx_head,
                        ecx_first,
                        ecx_item,
                        ecx_script_file,
                        esi_mgr,
                        esi_ready,
                        esi_pause,
                        esi_focus,
                        esi_table,
                        esi_head,
                        esi_first,
                        esi_item,
                        esi_script_file,
                        xroot_mgr,
                        xroot_ready,
                        xroot_pause,
                        xroot_focus,
                        xroot_table,
                        xroot_head,
                        xroot_first,
                        xroot_item,
                        xroot_script_file,
                        ecx_auto_begin,
                        ecx_auto_end,
                        esi_auto_begin,
                        esi_auto_end,
                        xroot_auto_begin,
                        xroot_auto_end,
                        iter_ent,
                        iter_name,
                        iter_extra,
                        iter_text,
                        begin_ent,
                        begin_name,
                        begin_extra,
                        begin_text,
                    ));
                }
            }
            if name.starts_with("SPIDEY_TRIGGER_") {
                static SPIDEY_TRIGGER_TAP_LOG: AtomicU32 = AtomicU32::new(0);
                let tn = SPIDEY_TRIGGER_TAP_LOG.fetch_add(1, AO::Relaxed);
                if tn < 256 || tn.is_power_of_two() {
                    let this = context.Rcx as u32;
                    let this_vt = read_guest_u32(r15, this);
                    let this_flags = read_guest_u32(r15, this.wrapping_add(0x1C));
                    let this_18c = read_guest_u32(r15, this.wrapping_add(0x18C));
                    let this_190 = read_guest_u32(r15, this.wrapping_add(0x190));
                    let this_ed = read_guest_u8(r15, this.wrapping_add(0xED));
                    let this_ee = read_guest_u8(r15, this.wrapping_add(0xEE));
                    let this_fc = read_guest_u8(r15, this.wrapping_add(0xFC));
                    let this_105 = read_guest_u8(r15, this.wrapping_add(0x105));
                    let this_17c = read_guest_u8(r15, this.wrapping_add(0x17C));
                    let this_d4 = read_guest_u32(r15, this.wrapping_add(0xD4));
                    let this_d8 = read_guest_u32(r15, this.wrapping_add(0xD8));
                    let this_dc = read_guest_u32(r15, this.wrapping_add(0xDC));
                    let this_e0 = read_guest_u32(r15, this.wrapping_add(0xE0));
                    let child = read_guest_u32(r15, this.wrapping_add(0x48));
                    let child_vt = read_guest_u32(r15, child);
                    let child_slot38 = read_guest_u32(r15, child_vt.wrapping_add(0x38));
                    let child_ee = read_guest_u8(r15, child.wrapping_add(0xEE));
                    let child_fc = read_guest_u8(r15, child.wrapping_add(0xFC));
                    let child_18c = read_guest_u8(r15, child.wrapping_add(0x18C));
                    let child_d4 = read_guest_u32(r15, child.wrapping_add(0xD4));
                    let child_d8 = read_guest_u32(r15, child.wrapping_add(0xD8));
                    let child_dc = read_guest_u32(r15, child.wrapping_add(0xDC));
                    let child_e0 = read_guest_u32(r15, child.wrapping_add(0xE0));
                    let ret_on_stack = read_guest_u32(r15, r14);
                    let arg0 = read_guest_u32(r15, r14.wrapping_add(0x04));
                    let arg1 = read_guest_u32(r15, r14.wrapping_add(0x08));
                    let arg2 = read_guest_u32(r15, r14.wrapping_add(0x0C));
                    let arg3 = read_guest_u32(r15, r14.wrapping_add(0x10));
                    let arg4 = read_guest_u32(r15, r14.wrapping_add(0x14));
                    let arg_pos_x = read_guest_u32(r15, arg0);
                    let arg_pos_y = read_guest_u32(r15, arg0.wrapping_add(0x04));
                    let arg_pos_z = read_guest_u32(r15, arg0.wrapping_add(0x08));
                    debug_log(&format!(
                        "[SPIDEY-TRIGGER-TAP #{}] {} guest=0x{:08X} ret=0x{:08X} this=0x{:08X} vt=0x{:08X} flags=0x{:08X} this+18c=0x{:08X} link190=0x{:08X} this+ed/ee/fc/105/17c={:02X}/{:02X}/{:02X}/{:02X}/{:02X} this_sphere=[{:.3},{:.3},{:.3}] r={:.3} child48=0x{:08X} child_vt=0x{:08X} child_slot38=0x{:08X} child+ee/fc/18c={:02X}/{:02X}/{:02X} child_sphere=[{:.3},{:.3},{:.3}] r={:.3} stack_ret=0x{:08X} args=[pos=0x{:08X}({:.3},{:.3},{:.3}), radius=0x{:08X}/{:.3}, 0x{:08X}, 0x{:08X}, 0x{:08X}] scene='{}' stash='{}'",
                        tn,
                        name,
                        guest_addr,
                        ret_addr,
                        this,
                        this_vt,
                        this_flags,
                        this_18c,
                        this_190,
                        this_ed,
                        this_ee,
                        this_fc,
                        this_105,
                        this_17c,
                        bits_to_f32(this_d4),
                        bits_to_f32(this_d8),
                        bits_to_f32(this_dc),
                        bits_to_f32(this_e0),
                        child,
                        child_vt,
                        child_slot38,
                        child_ee,
                        child_fc,
                        child_18c,
                        bits_to_f32(child_d4),
                        bits_to_f32(child_d8),
                        bits_to_f32(child_dc),
                        bits_to_f32(child_e0),
                        ret_on_stack,
                        arg0,
                        bits_to_f32(arg_pos_x),
                        bits_to_f32(arg_pos_y),
                        bits_to_f32(arg_pos_z),
                        arg1,
                        bits_to_f32(arg1),
                        arg2,
                        arg3,
                        arg4,
                        scene_name,
                        stash_name
                    ));
                }
            }
            debug_log(&format!(
                "[SPIDEY-TAP #{}] {} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} fsm=0x{:08X} fsm0=0x{:08X} cb3F5A48=0x{:08X} app=0x{:08X} scene=0x{:08X} engine=0x{:08X} frame=0x{:08X} scene_mgr=0x{:08X} scene_gate186=0x{:02X} loc_mgr=0x{:08X} stack=[0x{:08X}:'{}', 0x{:08X}:'{}'] scene_name='{}' stash_name='{}'",
                n,
                name,
                guest_addr,
                ret_addr,
                r14,
                context.Rax as u32,
                context.Rbx as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
                fsm,
                fsm_state_word0,
                callback_3f5a48,
                app,
                scene,
                engine,
                frame,
                scene_mgr_from_frame,
                scene_gate_186,
                loc_mgr,
                stack0,
                stack0_s,
                stack1,
                stack1_s,
                scene_name,
                stash_name
            ));
            if xbs_parse_tap && xbs_parse_should_log {
                fn xbs_obj_fields(r15: u64, label: &str, base: u32) -> String {
                    if !valid_guest_ptr(base) {
                        return format!("{}=0x{:08X}:invalid", label, base);
                    }
                    let root = read_guest_u32(r15, base.wrapping_add(0x28));
                    let root_1a4 = if valid_guest_ptr(root) {
                        read_guest_u32(r15, root.wrapping_add(0x1A4))
                    } else {
                        0
                    };
                    let root_1a8 = if valid_guest_ptr(root) {
                        read_guest_u32(r15, root.wrapping_add(0x1A8))
                    } else {
                        0
                    };
                    format!(
                        "{}=0x{:08X} +40/44/50/54=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} +1A4/1A8=0x{:08X}/0x{:08X} root=0x{:08X} root+1A4/1A8=0x{:08X}/0x{:08X}",
                        label,
                        base,
                        read_guest_u32(r15, base.wrapping_add(0x40)),
                        read_guest_u32(r15, base.wrapping_add(0x44)),
                        read_guest_u32(r15, base.wrapping_add(0x50)),
                        read_guest_u32(r15, base.wrapping_add(0x54)),
                        read_guest_u32(r15, base.wrapping_add(0x1A4)),
                        read_guest_u32(r15, base.wrapping_add(0x1A8)),
                        root,
                        root_1a4,
                        root_1a8
                    )
                }

                let xbs_path = read_c_string(r15, 0x003F_6028, 128);
                let h00 = read_guest_u32(r15, 0x003F_5EC0);
                let h04 = read_guest_u32(r15, 0x003F_5EC4);
                let h08 = read_guest_u32(r15, 0x003F_5EC8);
                let h0c = read_guest_u32(r15, 0x003F_5ECC);
                let h10 = read_guest_u32(r15, 0x003F_5ED0);
                let h14 = read_guest_u32(r15, 0x003F_5ED4);
                let h18 = read_guest_u32(r15, 0x003F_5ED8);
                let h1c = read_guest_u32(r15, 0x003F_5EDC);
                let h20 = read_guest_u32(r15, 0x003F_5EE0);
                let h24 = read_guest_u32(r15, 0x003F_5EE4);
                let h28 = read_guest_u32(r15, 0x003F_5EE8);
                let h2c = read_guest_u32(r15, 0x003F_5EEC);
                let h30 = read_guest_u32(r15, 0x003F_5EF0);
                let h34 = read_guest_u32(r15, 0x003F_5EF4);
                let table_a = read_guest_u32(r15, 0x003F_6240);
                let table_a_len = read_guest_u32(r15, 0x003F_6244);
                let table_b = read_guest_u32(r15, 0x003F_6254);
                let table_b_len = read_guest_u32(r15, 0x003F_6258);
                let records = read_guest_u32(r15, 0x003F_6940);

                debug_log(&format!(
                    "[SPIDEY-XBS-PARSE] {} path='{}' hdr00/04/08/0c/10/14/18/1c/20/24/28/2c/30/34=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} tblA=0x{:08X}+0x{:X} tblB=0x{:08X}+0x{:X} records=0x{:08X} stack14/18/1c/20/24=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} {} | {} | {} | {} | {}",
                    name,
                    xbs_path,
                    h00,
                    h04,
                    h08,
                    h0c,
                    h10,
                    h14,
                    h18,
                    h1c,
                    h20,
                    h24,
                    h28,
                    h2c,
                    h30,
                    h34,
                    table_a,
                    table_a_len,
                    table_b,
                    table_b_len,
                    records,
                    read_guest_u32(r15, r14.wrapping_add(0x14)),
                    read_guest_u32(r15, r14.wrapping_add(0x18)),
                    read_guest_u32(r15, r14.wrapping_add(0x1C)),
                    read_guest_u32(r15, r14.wrapping_add(0x20)),
                    read_guest_u32(r15, r14.wrapping_add(0x24)),
                    xbs_obj_fields(r15, "global_scene", scene),
                    xbs_obj_fields(r15, "engine_scene", engine),
                    xbs_obj_fields(r15, "frame_scene", scene_mgr_from_frame),
                    xbs_obj_fields(r15, "ecx", context.Rcx as u32),
                    xbs_obj_fields(r15, "esi", context.Rsi as u32)
                ));
            }
            if name == "SPIDEY_FSM_DISPATCH_TAP" {
                let this_scene = context.Rcx as u32;
                let this_a0 = read_guest_u32(r15, this_scene.wrapping_add(0xA0));
                let this_idx = read_guest_u32(r15, this_a0.wrapping_sub(0x10));
                let this_arr = read_guest_u32(r15, this_a0.wrapping_sub(0x14));
                let this_val = if valid_guest_ptr(this_arr) && this_idx < 0x1000 {
                    read_guest_u32(r15, this_arr.wrapping_add(this_idx.wrapping_mul(4)))
                } else {
                    0
                };
                let this_08 = read_guest_u8(r15, this_scene.wrapping_add(0x08));
                let this_09 = read_guest_u8(r15, this_scene.wrapping_add(0x09));
                let this_0a = read_guest_u8(r15, this_scene.wrapping_add(0x0A));
                let this_18b = read_guest_u8(r15, this_scene.wrapping_add(0x18B));
                static MENU_DISPATCH_THIS_LOG: AtomicU32 = AtomicU32::new(0);
                let disp_n = MENU_DISPATCH_THIS_LOG.fetch_add(1, AO::Relaxed);
                if disp_n < 128 || disp_n.is_power_of_two() {
                    debug_log(&format!(
                        "[SPIDEY-FSM-DISPATCH-THIS] #{} scene=0x{:08X} a0=0x{:08X} arr=0x{:08X} idx=0x{:08X} val=0x{:08X} flags8/9/A/18B={}/{}/{}/{}",
                        disp_n,
                        this_scene,
                        this_a0,
                        this_arr,
                        this_idx,
                        this_val,
                        this_08,
                        this_09,
                        this_0a,
                        this_18b
                    ));
                }
            }
            if name.starts_with("SPIDEY_KEYPRESS_TABLE_") {
                static KEYPRESS_TABLE_LOG: AtomicU32 = AtomicU32::new(0);
                let kn = KEYPRESS_TABLE_LOG.fetch_add(1, AO::Relaxed);
                let wanted_key = if name == "SPIDEY_KEYPRESS_TABLE_MARK_ENTRY_TAP" {
                    read_guest_u32(r15, r14.wrapping_add(4))
                } else {
                    context.Rcx as u32
                };
                let wanted_text = if valid_guest_ptr(wanted_key) {
                    read_c_string(r15, wanted_key, 96)
                } else {
                    String::new()
                };
                let slot = context.Rax as u32;
                let slot_index = slot;
                let (slot_key, slot_flag, slot_key_text) = if slot_index < 0x1C {
                    let key_addr = 0x003D_CEC0u32.wrapping_add(slot_index.wrapping_mul(8));
                    let key = read_guest_u32(r15, key_addr);
                    let flag = read_guest_u32(r15, key_addr.wrapping_add(4));
                    let text = if valid_guest_ptr(key) {
                        read_c_string(r15, key, 96)
                    } else {
                        String::new()
                    };
                    (key, flag, text)
                } else {
                    (0, 0, String::new())
                };
                let drain_key = if name == "SPIDEY_KEYPRESS_TABLE_DRAIN_ENTRY_TAP" {
                    let mut first = 0u32;
                    for s in 0u32..0x1C {
                        let key_addr = 0x003D_CEC0u32.wrapping_add(s.wrapping_mul(8));
                        let key = read_guest_u32(r15, key_addr);
                        let flag = read_guest_u32(r15, key_addr.wrapping_add(4));
                        if flag != 0 {
                            first = key;
                            break;
                        }
                    }
                    first
                } else {
                    0
                };
                let drain_key_text = if valid_guest_ptr(drain_key) {
                    read_c_string(r15, drain_key, 96)
                } else {
                    String::new()
                };
                let drain_gate_word = read_guest_u32(r15, 0x003D_CFA0);
                let drain_gate_byte = drain_gate_word & 0xFF;
                let start_ptr = 0x0038_7574u32;
                let start_slot = (0u32..0x1C).find(|slot| {
                    read_guest_u32(r15, 0x003D_CEC0u32.wrapping_add(slot.wrapping_mul(8)))
                        == start_ptr
                });
                let mut active = String::new();
                for s in 0u32..0x1C {
                    let key_addr = 0x003D_CEC0u32.wrapping_add(s.wrapping_mul(8));
                    let key = read_guest_u32(r15, key_addr);
                    let flag = read_guest_u32(r15, key_addr.wrapping_add(4));
                    if key != 0 || flag != 0 {
                        if !active.is_empty() {
                            active.push(' ');
                        }
                        active.push_str(&format!("{:02}:0x{:08X}/{}", s, key, flag));
                    }
                }
                if active.is_empty() {
                    active.push_str("<empty>");
                }
                let should_log = kn < 256
                    || kn.is_power_of_two()
                    || scene_name.eq_ignore_ascii_case("levels\\origin_z")
                    || wanted_key == start_ptr
                    || slot_key == start_ptr
                    || drain_key == start_ptr;
                if should_log {
                    debug_log(&format!(
                        "[SPIDEY-KEYPRESS-TABLE #{}] {} wanted=0x{:08X} wanted_text='{}' slot={} slot_key=0x{:08X} slot_flag={} slot_text='{}' drain_key=0x{:08X} drain_text='{}' drain_gate=0x{:08X}/{} start_ptr=0x{:08X} start_slot={:?} eax=0x{:08X} ecx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X} active=[{}] scene='{}' stash='{}'",
                        kn,
                        name,
                        wanted_key,
                        wanted_text,
                        slot_index,
                        slot_key,
                        slot_flag,
                        slot_key_text,
                        drain_key,
                        drain_key_text,
                        drain_gate_word,
                        drain_gate_byte,
                        start_ptr,
                        start_slot,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        r14,
                        active,
                        scene_name,
                        stash_name
                    ));
                }
                if name == "SPIDEY_KEYPRESS_TABLE_MARK_ENTRY_TAP"
                    && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                    && title_patches_enabled()
                    && std::env::var_os("RUSTEMU_SPIDEY_KEYPRESS_FORCE_WANTED").is_some()
                    && slot == 0
                {
                    let target_slot = 0x1Au32;
                    let key_addr = 0x003D_CEC0u32.wrapping_add(target_slot.wrapping_mul(8));
                    let old_key = read_guest_u32(r15, key_addr);
                    let old_flag = read_guest_u32(r15, key_addr.wrapping_add(4));
                    write_guest_u32(r15, key_addr, wanted_key);
                    write_guest_u32(r15, key_addr.wrapping_add(4), 1);
                    debug_log(&format!(
                        "[SPIDEY-KEYPRESS-FORCE-WANTED] wanted=0x{:08X} text='{}' slot={} key:0x{:08X}->0x{:08X} flag:{}->1 scene='{}'",
                        wanted_key,
                        wanted_text,
                        target_slot,
                        old_key,
                        wanted_key,
                        old_flag,
                        scene_name
                    ));
                }
            }
            if name.starts_with("SPIDEY_FSM_")
                || name == "SPIDEY_BOOT_GATE_TAP"
                || name.starts_with("SPIDEY_F8580_")
                || name == "SPIDEY_SCENE_RENDER_ENABLE_WRITE_TAP"
                || name == "SPIDEY_ORIGIN_MENU_ATTACH_TAP"
                || name == "SPIDEY_SCENE_F7D30_ENTRY_TAP"
                || name == "SPIDEY_SCENE_F7EC0_ENTRY_TAP"
                || name.starts_with("SPIDEY_SCENE_F27")
                || name.starts_with("SPIDEY_SCENE_F28")
                || name.starts_with("SPIDEY_SCENE_F29")
                || name.starts_with("SPIDEY_SCENE_ED")
                || name.starts_with("SPIDEY_SCENE_F10")
                || name.starts_with("SPIDEY_SCENE_F11")
                || name.starts_with("SPIDEY_SCENE_F12")
                || name.starts_with("SPIDEY_SCENE_F7D")
                || name.starts_with("SPIDEY_SCENE_F7F")
                || name.starts_with("SPIDEY_SCRIPT_51C00")
                || name.starts_with("SPIDEY_SCRIPT_NATIVE_")
                || name.starts_with("SPIDEY_SCRIPT_VM_")
                || name.starts_with("SPIDEY_NATIVE_METHOD_")
                || name.starts_with("SPIDEY_XBS_LIFECYCLE_")
                || name.starts_with("SPIDEY_KEYPRESS_TABLE_")
                || name.starts_with("SPIDEY_FLAG17F_")
                || name == "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP"
                || name.starts_with("SPIDEY_F0690_")
                || name.starts_with("SPIDEY_E9D00_")
                || name == "SPIDEY_SCENE_DD130_CALL_TAP"
                || name == "SPIDEY_SCENE_AFTER_DD130_TAP"
                || name.starts_with("SPIDEY_DD130_")
                || name == "SPIDEY_SCENE_DIRTY_WRITE_TAP"
                || name == "SPIDEY_XGRAPH_DIRTY_PROMOTE_TAP"
            {
                let app_100 = read_guest_u32(r15, app.wrapping_add(0x100));
                let app_104 = read_guest_u32(r15, app.wrapping_add(0x104));
                let app_108 = read_guest_u32(r15, app.wrapping_add(0x108));
                let scene_24 = read_guest_u8(r15, scene.wrapping_add(0x24));
                let scene_25 = read_guest_u8(r15, scene.wrapping_add(0x25));
                let scene_184 = read_guest_u8(r15, scene.wrapping_add(0x184));
                let scene_185 = read_guest_u8(r15, scene.wrapping_add(0x185));
                let scene_186 = read_guest_u8(r15, scene.wrapping_add(0x186));
                let scene_18e = read_guest_u8(r15, scene.wrapping_add(0x18E));
                let scene_18f = read_guest_u8(r15, scene.wrapping_add(0x18F));
                let scene_root = read_guest_u32(r15, scene.wrapping_add(0x28));
                let scene_cur = read_guest_u32(r15, scene.wrapping_add(0x38));
                let scene_a0 = read_guest_u32(r15, scene.wrapping_add(0xA0));
                let scene_state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                } else {
                    0
                };
                let scene_state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                } else {
                    0
                };
                let scene_state_val =
                    if valid_guest_ptr(scene_state_arr) && scene_state_idx < 0x1000 {
                        read_guest_u32(
                            r15,
                            scene_state_arr.wrapping_add(scene_state_idx.wrapping_mul(4)),
                        )
                    } else {
                        0
                    };
                debug_log(&format!(
                    "[SPIDEY-FSM-GATE] {} app=0x{:08X} app100=0x{:08X} app104=0x{:08X} app108=0x{:08X} scene=0x{:08X} flags24/25/184/185/186/18e/18f={}/{}/{}/{}/{}/{}/{} root=0x{:08X} cur=0x{:08X} sceneA0=0x{:08X} state_idx=0x{:08X} state_arr=0x{:08X} state_val=0x{:08X} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                    name,
                    app,
                    app_100,
                    app_104,
                    app_108,
                    scene,
                    scene_24,
                    scene_25,
                    scene_184,
                    scene_185,
                    scene_186,
                    scene_18e,
                    scene_18f,
                    scene_root,
                    scene_cur,
                    scene_a0,
                    scene_state_idx,
                    scene_state_arr,
                    scene_state_val,
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32
                ));
                if name == "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP" {
                    let scene_this = context.Rcx as u32;
                    let stream_obj = read_guest_u32(r15, scene_this.wrapping_add(0x28));
                    let scene_stream_mgr = read_guest_u32(r15, scene_this.wrapping_add(0x1A0));
                    let scene_stream_ctx = read_guest_u32(r15, scene_this.wrapping_add(0x1AC));
                    let obj_stream_mgr = read_guest_u32(r15, stream_obj.wrapping_add(0x1A0));
                    let obj_stream_ctx = read_guest_u32(r15, stream_obj.wrapping_add(0x1AC));
                    let global_mgr = read_guest_u32(r15, 0x004B_8F04);
                    let global_ctx = read_guest_u32(r15, 0x004B_8F08);
                    let global_handle = read_guest_u32(r15, 0x004B_8F0C);
                    debug_log(&format!(
                        "[SPIDEY-STREAM-GLOBALS] {} scene_this=0x{:08X} stream_obj[scene+28]=0x{:08X} scene+1A0/1AC=0x{:08X}/0x{:08X} obj+1A0/1AC=0x{:08X}/0x{:08X} globals[4B8F04/08/0C]=0x{:08X}/0x{:08X}/0x{:08X} scene='{}' stash='{}'",
                        name,
                        scene_this,
                        stream_obj,
                        scene_stream_mgr,
                        scene_stream_ctx,
                        obj_stream_mgr,
                        obj_stream_ctx,
                        global_mgr,
                        global_ctx,
                        global_handle,
                        scene_name,
                        stash_name
                    ));
                }
                if name == "SPIDEY_SCENE_F7EC0_ENTRY_TAP" {
                    let scene_this = context.Rcx as u32;
                    let scene_08 = read_guest_u8(r15, scene_this.wrapping_add(0x08));
                    let scene_09 = read_guest_u8(r15, scene_this.wrapping_add(0x09));
                    let scene_0a = read_guest_u8(r15, scene_this.wrapping_add(0x0A));
                    let scene_17f = read_guest_u8(r15, scene_this.wrapping_add(0x17F));
                    let scene_186 = read_guest_u8(r15, scene_this.wrapping_add(0x186));
                    let scene_18b = read_guest_u8(r15, scene_this.wrapping_add(0x18B));
                    let scene_18c = read_guest_u8(r15, scene_this.wrapping_add(0x18C));
                    let scene_1a8 = read_guest_u32(r15, scene_this.wrapping_add(0x1A8));
                    let scene_a0 = read_guest_u32(r15, scene_this.wrapping_add(0xA0));
                    let scene_state_idx = read_guest_u32(r15, scene_a0.wrapping_sub(0x10));
                    let scene_state_arr = read_guest_u32(r15, scene_a0.wrapping_sub(0x14));
                    let scene_state_val =
                        if valid_guest_ptr(scene_state_arr) && scene_state_idx < 0x1000 {
                            read_guest_u32(
                                r15,
                                scene_state_arr.wrapping_add(scene_state_idx.wrapping_mul(4)),
                            )
                        } else {
                            0
                        };
                    let global_08 = read_guest_u8(r15, scene.wrapping_add(0x08));
                    let global_09 = read_guest_u8(r15, scene.wrapping_add(0x09));
                    let global_0a = read_guest_u8(r15, scene.wrapping_add(0x0A));
                    let updater50 = read_guest_u32(r15, scene_this.wrapping_add(0x50));
                    let updater54 = read_guest_u32(r15, scene_this.wrapping_add(0x54));
                    let updater50_vt = read_guest_u32(r15, updater50);
                    let updater54_vt = read_guest_u32(r15, updater54);
                    let updater50_step = read_guest_u32(r15, updater50_vt.wrapping_add(0x3C));
                    let updater54_step = read_guest_u32(r15, updater54_vt.wrapping_add(0x3C));
                    let arg0 = read_guest_u32(r15, r14.wrapping_add(4));
                    let arg0_word = read_guest_u32(r15, arg0);
                    debug_log(&format!(
                        "[SPIDEY-F7EC0-GATES] this=0x{:08X} +8={} +9={} +A={} +18B={} +1A8=0x{:08X} sceneA0=0x{:08X} state_idx=0x{:08X} state_arr=0x{:08X} state_val=0x{:08X} up50=0x{:08X}/vt=0x{:08X}/step=0x{:08X} up54=0x{:08X}/vt=0x{:08X}/step=0x{:08X} global=0x{:08X} global+8/+9/+A={}/{}/{} arg0=0x{:08X} *arg0=0x{:08X}",
                        scene_this,
                        scene_08,
                        scene_09,
                        scene_0a,
                        scene_18b,
                        scene_1a8,
                        scene_a0,
                        scene_state_idx,
                        scene_state_arr,
                        scene_state_val,
                        updater50,
                        updater50_vt,
                        updater50_step,
                        updater54,
                        updater54_vt,
                        updater54_step,
                        scene,
                        global_08,
                        global_09,
                        global_0a,
                        arg0,
                        arg0_word
                    ));
                    let peterstu_ready_seq =
                        crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                    let origin_state4_f7ec0_wait = scene_name
                        .eq_ignore_ascii_case("levels\\origin_z")
                        && scene_state_val == 4
                        && scene_08 != 0
                        && scene_09 == 0
                        && scene_0a != 0
                        && scene_17f == 0
                        && scene_186 != 0
                        && scene_18b != 0
                        && scene_18c != 0
                        && peterstu_ready_seq != 0;
                    if origin_state4_f7ec0_wait {
                        static ORIGINZ_F7EC0_09_POLLS: AtomicU32 = AtomicU32::new(0);
                        static ORIGINZ_F7EC0_09_FIRE_LOG: AtomicU32 = AtomicU32::new(0);
                        let wait_n = ORIGINZ_F7EC0_09_POLLS.fetch_add(1, AO::Relaxed);
                        let fire_after =
                            std::env::var("RUSTEMU_SPIDEY_ORIGINZ_F7EC0_COMPLETE_AFTER")
                                .ok()
                                .and_then(|v| v.trim().parse::<u32>().ok())
                                .filter(|v| *v >= 8)
                                .unwrap_or(32);
                        let enabled = wait_n >= fire_after
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_DISABLE_ORIGINZ_F7EC0_COMPLETE")
                                .is_none();
                        if wait_n < 8 || wait_n.is_power_of_two() || wait_n == fire_after {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-F7EC0-09-WAIT] #{} this=0x{:08X} flags8/9/A/17F/186/18B/18C={}/{}/{}/{}/{}/{}/{} state_val={} peterstu_seq={} fire_after={} enabled={}",
                                wait_n,
                                scene_this,
                                scene_08,
                                scene_09,
                                scene_0a,
                                scene_17f,
                                scene_186,
                                scene_18b,
                                scene_18c,
                                scene_state_val,
                                peterstu_ready_seq,
                                fire_after,
                                if enabled { 1 } else { 0 }
                            ));
                        }
                        if enabled {
                            write_guest_u8(r15, scene_this.wrapping_add(0x09), 1);
                            let fire_n = ORIGINZ_F7EC0_09_FIRE_LOG.fetch_add(1, AO::Relaxed);
                            if fire_n < 8 || fire_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGINZ-F7EC0-09-FIRE] #{} wait_n={} this=0x{:08X} wrote [+0x09]=1",
                                    fire_n, wait_n, scene_this
                                ));
                            }
                        }
                    }
                }
                if name.starts_with("SPIDEY_FLAG17F_") {
                    static FLAG17F_LOG_EARLY: AtomicU32 = AtomicU32::new(0);
                    let n = FLAG17F_LOG_EARLY.fetch_add(1, AO::Relaxed);
                    let value_obj = read_guest_u32(r15, r14.wrapping_add(4));
                    let value_stack = if valid_guest_ptr(value_obj) {
                        read_guest_u32(r15, value_obj.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let value_bits = if valid_guest_ptr(value_stack.wrapping_sub(4)) {
                        read_guest_u32(r15, value_stack.wrapping_sub(4))
                    } else {
                        0
                    };
                    let target = read_guest_u32(r15, 0x004B_C614);
                    let scene_a0 = if valid_guest_ptr(target) {
                        read_guest_u32(r15, target.wrapping_add(0xA0))
                    } else {
                        0
                    };
                    let state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                        read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let flag8 = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let flag17f = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x17F))
                    } else {
                        0
                    };
                    let flag18b = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x18B))
                    } else {
                        0
                    };
                    debug_log(&format!(
                        "[SPIDEY-FLAG17F-SETTER] #{} target=0x{:08X} value_obj=0x{:08X} value_stack=0x{:08X} arg_bits=0x{:08X}/{:.6} pre flags8/17F/18B={}/{}/{} state_idx=0x{:08X} state_val=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} scene='{}' stash='{}'",
                        n,
                        target,
                        value_obj,
                        value_stack,
                        value_bits,
                        bits_to_f32(value_bits),
                        flag8,
                        flag17f,
                        flag18b,
                        state_idx,
                        state_val,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        scene_name,
                        stash_name
                    ));
                }
                if name.starts_with("SPIDEY_SCENE_F27")
                    || name.starts_with("SPIDEY_SCENE_F28")
                    || name.starts_with("SPIDEY_SCENE_F29")
                    || name.starts_with("SPIDEY_SCENE_ED")
                    || name.starts_with("SPIDEY_SCENE_F10")
                    || name.starts_with("SPIDEY_SCENE_F11")
                    || name.starts_with("SPIDEY_SCENE_F12")
                    || name.starts_with("SPIDEY_SCENE_F7D")
                    || name.starts_with("SPIDEY_SCENE_F7F")
                {
                    let scene_this = if name == "SPIDEY_SCENE_F2740_ENTRY_TAP" {
                        context.Rcx as u32
                    } else if name == "SPIDEY_SCENE_F1080_FINALIZE_ENTRY_TAP" {
                        context.Rcx as u32
                    } else if name == "SPIDEY_SCENE_F1057_SET17F_TAP" {
                        context.Rbp as u32
                    } else if name == "SPIDEY_SCENE_F7D10_ADVANCE_TAP" {
                        context.Rcx as u32
                    } else if name == "SPIDEY_SCENE_ED320_ENTRY_TAP" {
                        context.Rcx as u32
                    } else if name.starts_with("SPIDEY_SCENE_F10")
                        || name.starts_with("SPIDEY_SCENE_F11")
                        || name.starts_with("SPIDEY_SCENE_F12")
                    {
                        context.Rsi as u32
                    } else if name.starts_with("SPIDEY_SCENE_F28")
                        || name.starts_with("SPIDEY_SCENE_F29")
                        || name.starts_with("SPIDEY_SCENE_ED")
                    {
                        context.Rsi as u32
                    } else if name.starts_with("SPIDEY_SCENE_F7D") {
                        context.Rcx as u32
                    } else if name.starts_with("SPIDEY_SCENE_F7F") {
                        context.Rdi as u32
                    } else {
                        context.Rsi as u32
                    };
                    let scene_08 = read_guest_u8(r15, scene_this.wrapping_add(0x08));
                    let scene_09 = read_guest_u8(r15, scene_this.wrapping_add(0x09));
                    let scene_0a = read_guest_u8(r15, scene_this.wrapping_add(0x0A));
                    let scene_17f = read_guest_u8(r15, scene_this.wrapping_add(0x17F));
                    let scene_18b = read_guest_u8(r15, scene_this.wrapping_add(0x18B));
                    let scene_18c = read_guest_u8(r15, scene_this.wrapping_add(0x18C));
                    let scene_186 = read_guest_u8(r15, scene_this.wrapping_add(0x186));
                    let scene_b4 = read_guest_u32(r15, scene_this.wrapping_add(0xB4));
                    let scene_a0 = read_guest_u32(r15, scene_this.wrapping_add(0xA0));
                    let scene_state_idx = read_guest_u32(r15, scene_a0.wrapping_sub(0x10));
                    let scene_state_arr = read_guest_u32(r15, scene_a0.wrapping_sub(0x14));
                    let scene_state_val =
                        if valid_guest_ptr(scene_state_arr) && scene_state_idx < 0x1000 {
                            read_guest_u32(
                                r15,
                                scene_state_arr.wrapping_add(scene_state_idx.wrapping_mul(4)),
                            )
                        } else {
                            0
                        };
                    let xroot = read_guest_u32(r15, 0x004C_06B8);
                    let focus_mgr = read_guest_u32(r15, xroot.wrapping_add(0x1A8));
                    let focus_head = if focus_mgr >= 0x1000 && valid_guest_ptr(focus_mgr) {
                        read_guest_u32(r15, focus_mgr.wrapping_add(0x10))
                    } else {
                        0
                    };
                    let focus_first = if focus_head >= 0x1000 && valid_guest_ptr(focus_head) {
                        read_guest_u32(r15, focus_head)
                    } else {
                        0
                    };
                    let focus_item = if focus_first >= 0x1000 && valid_guest_ptr(focus_first) {
                        read_guest_u32(r15, focus_first.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let focus_item_flags = if focus_item >= 0x1000 && valid_guest_ptr(focus_item) {
                        read_guest_u8(r15, focus_item.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let focus_item_pc = if focus_item >= 0x1000 && valid_guest_ptr(focus_item) {
                        read_guest_u32(r15, focus_item.wrapping_add(0x1C))
                    } else {
                        0
                    };
                    let focus_item_word =
                        if focus_item_pc >= 0x1000 && valid_guest_ptr(focus_item_pc) {
                            read_guest_u16(r15, focus_item_pc)
                        } else {
                            0
                        };
                    let focus_item_op = focus_item_word & 0x007F;
                    let focus_item_major = focus_item_word >> 8;
                    let focus_item_stack = if focus_item >= 0x1000 && valid_guest_ptr(focus_item) {
                        read_guest_u32(r15, focus_item.wrapping_add(0x14))
                    } else {
                        0
                    };
                    let focus_item_counter = if focus_item >= 0x1000 && valid_guest_ptr(focus_item)
                    {
                        read_guest_u32(r15, focus_item.wrapping_add(0x44))
                    } else {
                        0
                    };
                    let focus_item_timer = if focus_item >= 0x1000 && valid_guest_ptr(focus_item) {
                        read_guest_u32(r15, focus_item.wrapping_add(0x40))
                    } else {
                        0
                    };
                    let stack10 = read_guest_u32(r15, r14.wrapping_add(0x10));
                    let stack24 = read_guest_u32(r15, r14.wrapping_add(0x24));
                    if name == "SPIDEY_SCENE_F2740_ENTRY_TAP" {
                        static F2740_INPUT_LOG: AtomicU32 = AtomicU32::new(0);
                        let input_n = F2740_INPUT_LOG.fetch_add(1, AO::Relaxed);
                        let mgr = read_guest_u32(r15, 0x003F_7D90);
                        let cur = read_guest_u32(r15, mgr.wrapping_add(0x58));
                        let mut slots = String::new();
                        let mut any_live_input = false;
                        for slot in 0..10u32 {
                            let obj = read_guest_u32(
                                r15,
                                mgr.wrapping_add(0x2C).wrapping_add(slot.wrapping_mul(4)),
                            );
                            if !valid_guest_ptr(obj) {
                                continue;
                            }
                            let vt = read_guest_u32(r15, obj);
                            let f08 = read_guest_u32(r15, vt.wrapping_add(0x08));
                            let f10 = read_guest_u32(r15, vt.wrapping_add(0x10));
                            let f14 = read_guest_u32(r15, vt.wrapping_add(0x14));
                            let f18 = read_guest_u32(r15, vt.wrapping_add(0x18));
                            let f1c = read_guest_u32(r15, vt.wrapping_add(0x1C));
                            let f44 = read_guest_u32(r15, vt.wrapping_add(0x44));
                            let handle = read_guest_u32(r15, obj.wrapping_add(0x50));
                            let connected = read_guest_u32(r15, obj.wrapping_add(0x9C));
                            let prev_connected = read_guest_u32(r15, obj.wrapping_add(0xA0));
                            let port = read_guest_u32(r15, obj.wrapping_add(0xA4));
                            let cur_packet = read_guest_u32(r15, obj.wrapping_add(0x08));
                            let cur_buttons = read_guest_u16(r15, obj.wrapping_add(0x0C));
                            let cur_a = read_guest_u8(r15, obj.wrapping_add(0x0E));
                            let cur_lx = read_guest_u16(r15, obj.wrapping_add(0x16)) as i16;
                            let cur_ly = read_guest_u16(r15, obj.wrapping_add(0x18)) as i16;
                            let prev_packet = read_guest_u32(r15, obj.wrapping_add(0x1E));
                            let prev_buttons = read_guest_u16(r15, obj.wrapping_add(0x22));
                            let prev_a = read_guest_u8(r15, obj.wrapping_add(0x24));
                            let prev_lx = read_guest_u16(r15, obj.wrapping_add(0x2C)) as i16;
                            let prev_ly = read_guest_u16(r15, obj.wrapping_add(0x2E)) as i16;
                            any_live_input |= cur_buttons != 0
                                || prev_buttons != 0
                                || cur_a != 0
                                || prev_a != 0
                                || cur_lx != 0
                                || cur_ly != 0
                                || prev_lx != 0
                                || prev_ly != 0;
                            slots.push_str(&format!(
                                " [{}]=obj:0x{:08X} vt:0x{:08X} f08:0x{:08X} f10:0x{:08X} f14:0x{:08X} f18:0x{:08X} f1c:0x{:08X} f44:0x{:08X} h:0x{:08X} conn:{}/{} port:{} cur(pkt:{} btn:0x{:04X} A:{} lx:{} ly:{}) prev(pkt:{} btn:0x{:04X} A:{} lx:{} ly:{})",
                                slot + 1,
                                obj,
                                vt,
                                f08,
                                f10,
                                f14,
                                f18,
                                f1c,
                                f44,
                                handle,
                                connected,
                                prev_connected,
                                port,
                                cur_packet,
                                cur_buttons,
                                cur_a,
                                cur_lx,
                                cur_ly,
                                prev_packet,
                                prev_buttons,
                                prev_a,
                                prev_lx,
                                prev_ly
                            ));
                        }
                        if input_n < 16
                            || input_n.is_power_of_two()
                            || any_live_input
                            || (scene_state_val == 4 && input_n < 128)
                            || (scene_state_val == 4 && input_n % 64 == 0)
                        {
                            debug_log(&format!(
                                "[SPIDEY-F2740-INPUT-MGR] #{} state_val=0x{:08X} mgr=0x{:08X} cur={} ebp_input=0x{:08X} maps=[{} | {} | {} | {} | {}]{}",
                                input_n,
                                scene_state_val,
                                mgr,
                                cur,
                                context.Rbp as u32,
                                dump_spidey_action_bindings(r15, mgr, 0x15),
                                dump_spidey_action_bindings(r15, mgr, 0x16),
                                dump_spidey_action_bindings(r15, mgr, 0x40),
                                dump_spidey_action_bindings(r15, mgr, 0x41),
                                dump_spidey_action_bindings(r15, mgr, 0x84),
                                slots
                            ));
                        }
                    }
                    if name.starts_with("SPIDEY_SCENE_F28") {
                        let local_action84 = read_guest_u32(r15, r14.wrapping_add(0x10));
                        let local_primary = read_guest_u32(r15, r14.wrapping_add(0x24));
                        let dt_ptr = context.Rbx as u32;
                        let dt_bits = read_guest_u32(r15, dt_ptr);
                        static F28_BRANCH_LOG: AtomicU32 = AtomicU32::new(0);
                        static F28_STATE4_LOG: AtomicU32 = AtomicU32::new(0);
                        let bn = F28_BRANCH_LOG.fetch_add(1, AO::Relaxed);
                        let state4_n = if scene_state_val == 4 {
                            F28_STATE4_LOG.fetch_add(1, AO::Relaxed)
                        } else {
                            u32::MAX
                        };
                        if bn < 256
                            || bn.is_power_of_two()
                            || state4_n < 192
                            || state4_n.is_power_of_two()
                        {
                            debug_log(&format!(
                                "[SPIDEY-F28-BRANCH] #{} state4#{} {} this=0x{:08X} flags8/9/A/18B={}/{}/{}/{} b4=0x{:08X}/{:.6} local24=0x{:08X}/{:.6} action84=0x{:08X}/{:.6} dt_ptr=0x{:08X} dt=0x{:08X}/{:.6} state_val=0x{:08X} ebp_input=0x{:08X} eax=0x{:08X} al=0x{:02X} edx=0x{:08X}",
                                bn,
                                state4_n,
                                name,
                                scene_this,
                                scene_08,
                                scene_09,
                                scene_0a,
                                scene_18b,
                                scene_b4,
                                bits_to_f32(scene_b4),
                                local_primary,
                                bits_to_f32(local_primary),
                                local_action84,
                                bits_to_f32(local_action84),
                                dt_ptr,
                                dt_bits,
                                bits_to_f32(dt_bits),
                                scene_state_val,
                                context.Rbp as u32,
                                context.Rax as u32,
                                context.Rax as u8,
                                context.Rdx as u32
                            ));
                        }
                        let bypass_origin_gate = name == "SPIDEY_SCENE_F28C5_SECONDARY_ELSE_TAP"
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_BYPASS_ORIGIN_GATE").is_some()
                            && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                            && scene_state_val == 3
                            && scene_17f == 0
                            && scene_186 != 0;
                        if bypass_origin_gate {
                            let target_guest =
                                match std::env::var("RUSTEMU_SPIDEY_BYPASS_ORIGIN_GATE")
                                    .unwrap_or_default()
                                    .to_ascii_lowercase()
                                    .as_str()
                                {
                                    "f28ae" | "ed320" => 0x000F_28AE,
                                    _ => 0x000F_2864,
                                };
                            static ORIGIN_GATE_BYPASS_LOG: AtomicU32 = AtomicU32::new(0);
                            let bypass_n = ORIGIN_GATE_BYPASS_LOG.fetch_add(1, AO::Relaxed);
                            if let Some(target_host_off) = ctx.addr_hash.lookup(target_guest) {
                                debug_log(&format!(
                                    "[SPIDEY-BYPASS-ORIGIN-GATE] #{} {} -> 0x{:08X} this=0x{:08X} flags8/9/A/17F/186/18B={}/{}/{}/{}/{}/{} b4=0x{:08X}/{:.6} local24=0x{:08X}/{:.6} action84=0x{:08X}/{:.6}",
                                    bypass_n,
                                    name,
                                    target_guest,
                                    scene_this,
                                    scene_08,
                                    scene_09,
                                    scene_0a,
                                    scene_17f,
                                    scene_186,
                                    scene_18b,
                                    scene_b4,
                                    bits_to_f32(scene_b4),
                                    local_primary,
                                    bits_to_f32(local_primary),
                                    local_action84,
                                    bits_to_f32(local_action84)
                                ));
                                m.active = false;
                                m.pending_rearm = true;
                                #[cfg(windows)]
                                {
                                    use windows::Win32::System::Memory::*;
                                    let target = unsafe { ctx.code_base.add(host_offset as usize) };
                                    let mut old_prot = PAGE_PROTECTION_FLAGS(0);
                                    if unsafe {
                                        VirtualProtect(
                                            target as *const _,
                                            1,
                                            PAGE_EXECUTE_READWRITE,
                                            &mut old_prot,
                                        )
                                    }
                                    .is_ok()
                                    {
                                        unsafe {
                                            *target = original_byte;
                                        }
                                        let _ = unsafe {
                                            VirtualProtect(
                                                target as *const _,
                                                1,
                                                old_prot,
                                                &mut old_prot,
                                            )
                                        };
                                    }
                                }
                                context.Rip = code_base + target_host_off as u64;
                                return true;
                            }
                            debug_log(&format!(
                                "[SPIDEY-BYPASS-ORIGIN-GATE-FAIL] #{} target=0x{:08X} missing host mapping",
                                bypass_n, target_guest
                            ));
                        }
                        // Synthetic menu-loading completion. This mirrors the missing
                        // async completion callback that advances bonus\menu out of the
                        // "Please Wait" state once the frontend render/widgets are alive.
                        // Keep the env disable switch because this is still a Spider-Man
                        // scaffold, not a generic Xbox scene rule.
                        {
                            static MENU_COMPLETE_CHECKS: AtomicU32 = AtomicU32::new(0);
                            let synth_menu = name == "SPIDEY_SCENE_F28C5_SECONDARY_ELSE_TAP"
                                && scene_name.eq_ignore_ascii_case("bonus\\menu")
                                && scene_state_val == 3
                                && scene_18c == 0;
                            if synth_menu {
                                let check_n = MENU_COMPLETE_CHECKS.fetch_add(1, AO::Relaxed);
                                let synth_menu_requested =
                                    std::env::var_os("RUSTEMU_SPIDEY_SYNTH_MENU_COMPLETE")
                                        .is_some()
                                        || std::env::var_os("RUSTEMU_SPIDEY_MENU_COMPLETE_AFTER")
                                            .is_some();
                                let title_patches = title_patches_enabled();
                                let synth_menu_default_ready = scene_186 != 0;
                                let synth_menu_enabled = synth_menu_requested
                                    && title_patches
                                    && std::env::var_os(
                                        "RUSTEMU_SPIDEY_DISABLE_SYNTH_MENU_COMPLETE",
                                    )
                                    .is_none();
                                let fire_after =
                                    std::env::var("RUSTEMU_SPIDEY_MENU_COMPLETE_AFTER")
                                        .ok()
                                        .and_then(|v| v.trim().parse::<u32>().ok())
                                        .unwrap_or(90);
                                if check_n < 4 || check_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-MENU-COMPLETE-WAIT] #{} this=0x{:08X} scene='{}' state_val={} 18c={} 186={} synth_requested={} default_ready={} title_patches={} synth_enabled={} fire_after={}",
                                        check_n, scene_this, scene_name, scene_state_val,
                                        scene_18c, scene_186, synth_menu_requested,
                                        synth_menu_default_ready, title_patches, synth_menu_enabled, fire_after
                                    ));
                                }
                                if synth_menu_enabled && check_n >= fire_after {
                                    write_guest_u8(r15, scene_this.wrapping_add(0x18C), 1);
                                    if check_n == fire_after || check_n.is_power_of_two() {
                                        debug_log(&format!(
                                            "[SPIDEY-MENU-COMPLETE-FIRE] #{} this=0x{:08X} scene='{}' state_val={} wrote [+0x18C]=1",
                                            check_n, scene_this, scene_name, scene_state_val
                                        ));
                                    }
                                } else if !synth_menu_enabled && check_n == fire_after {
                                    debug_log(&format!(
                                        "[SPIDEY-MENU-COMPLETE-SKIP] this=0x{:08X} scene='{}' state_val={} left [+0x18C]=0",
                                        scene_this, scene_name, scene_state_val
                                    ));
                                }
                            }
                        }

                        let origin_state4_wait_ready = scene_state_val == 4
                            && scene_17f == 0
                            && scene_186 != 0
                            && scene_18b != 0;
                        let origin_state3_wait_ready = scene_state_val == 3
                            && scene_17f != 0
                            && scene_186 != 0
                            && scene_18c == 0;
                        let synth_origin_candidate = name
                            == "SPIDEY_SCENE_F28C5_SECONDARY_ELSE_TAP"
                            && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                            && (origin_state4_wait_ready || origin_state3_wait_ready);
                        if synth_origin_candidate {
                            static ORIGIN_COMPLETE_SYNTH_LOG: AtomicU32 = AtomicU32::new(0);
                            static ORIGIN_STATE4_040A_POLLS: AtomicU32 = AtomicU32::new(0);
                            static ORIGIN_STATE4_040A_FIRE_LOG: AtomicU32 = AtomicU32::new(0);
                            let synth_n = ORIGIN_COMPLETE_SYNTH_LOG.fetch_add(1, AO::Relaxed);
                            let synth_mode = std::env::var("RUSTEMU_SPIDEY_SYNTH_ORIGIN_COMPLETE")
                                .unwrap_or_default()
                                .to_ascii_lowercase();
                            let synth_origin_requested = !synth_mode.is_empty()
                                || std::env::var_os("RUSTEMU_SPIDEY_ORIGIN_COMPLETE_AFTER")
                                    .is_some();
                            let title_patches = title_patches_enabled();
                            let peterstu_ready_seq =
                                crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                            let origin_state4_040a_wait = origin_state4_wait_ready
                                && scene_18c == 0
                                && peterstu_ready_seq != 0
                                && focus_item_word == 0x040A
                                && focus_item_flags != 0
                                && local_action84 == 0;
                            let origin_state4_040a_n = if origin_state4_040a_wait {
                                ORIGIN_STATE4_040A_POLLS.fetch_add(1, AO::Relaxed)
                            } else {
                                ORIGIN_STATE4_040A_POLLS.load(AO::Relaxed)
                            };
                            let origin_state4_040a_after =
                                std::env::var("RUSTEMU_SPIDEY_ORIGINZ_STATE4_COMPLETE_AFTER")
                                    .ok()
                                    .and_then(|v| v.trim().parse::<u32>().ok())
                                    .filter(|v| *v >= 8)
                                    .unwrap_or(32);
                            let origin_state4_040a_policy_enabled = title_patches
                                || env_flag("RUSTEMU_SPIDEY_AUTONEWGAME")
                                || std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_STATE4_COMPLETE_AFTER")
                                    .is_some();
                            let origin_state4_040a_enabled = origin_state4_040a_wait
                                && origin_state4_040a_n >= origin_state4_040a_after
                                && origin_state4_040a_policy_enabled
                                && std::env::var_os(
                                    "RUSTEMU_SPIDEY_DISABLE_ORIGINZ_STATE4_COMPLETE",
                                )
                                .is_none();
                            if origin_state4_040a_wait
                                && (origin_state4_040a_n < 8
                                    || origin_state4_040a_n.is_power_of_two()
                                    || origin_state4_040a_n == origin_state4_040a_after)
                            {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGINZ-STATE4-040A-WAIT] #{} this=0x{:08X} focus=0x{:08X} pc=0x{:08X} word=0x{:04X} timer=0x{:08X}/{:.6} ctr={} flags8/9/A/17F/186/18B/18C={}/{}/{}/{}/{}/{}/{} peterstu_seq={} fire_after={} policy_enabled={} enabled={}",
                                    origin_state4_040a_n,
                                    scene_this,
                                    focus_item,
                                    focus_item_pc,
                                    focus_item_word,
                                    focus_item_timer,
                                    bits_to_f32(focus_item_timer),
                                    focus_item_counter,
                                    scene_08,
                                    scene_09,
                                    scene_0a,
                                    scene_17f,
                                    scene_186,
                                    scene_18b,
                                    scene_18c,
                                    peterstu_ready_seq,
                                    origin_state4_040a_after,
                                    origin_state4_040a_policy_enabled,
                                    origin_state4_040a_enabled
                                ));
                            }
                            if origin_state4_040a_enabled {
                                write_guest_u8(r15, scene_this.wrapping_add(0x18C), 1);
                                let fire_n = ORIGIN_STATE4_040A_FIRE_LOG.fetch_add(1, AO::Relaxed);
                                if fire_n < 8 || fire_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-ORIGINZ-STATE4-040A-FIRE] #{} wait_n={} this=0x{:08X} focus=0x{:08X} pc=0x{:08X} word=0x{:04X} wrote [+0x18C]=1",
                                        fire_n,
                                        origin_state4_040a_n,
                                        scene_this,
                                        focus_item,
                                        focus_item_pc,
                                        focus_item_word
                                    ));
                                }
                            }
                            // The Origin_Z +0x18C write is only a diagnostic poke. Two
                            // live runs showed that defaulting it on after PETERSTU
                            // readiness advances the script into the boot/movie cycle
                            // instead of the training load path.
                            let synth_origin_default_ready = false;
                            let synth_origin_enabled = (synth_origin_requested
                                || synth_origin_default_ready)
                                && title_patches
                                && std::env::var_os("RUSTEMU_SPIDEY_DISABLE_SYNTH_ORIGIN_COMPLETE")
                                    .is_none();
                            let fire_after = std::env::var("RUSTEMU_SPIDEY_ORIGIN_COMPLETE_AFTER")
                                .ok()
                                .and_then(|v| v.trim().parse::<u32>().ok())
                                .unwrap_or(if synth_origin_requested { 0 } else { 8 });
                            if synth_n < 8 || synth_n.is_power_of_two() || synth_n == fire_after {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGIN-COMPLETE-WAIT] #{} {} this=0x{:08X} flags8/9/A/17F/186/18B/18C={}/{}/{}/{}/{}/{}/{} state_val=0x{:08X} action84=0x{:08X}/{:.6} peterstu_seq={} synth_requested={} default_ready={} title_patches={} synth_enabled={} fire_after={}",
                                    synth_n,
                                    name,
                                    scene_this,
                                    scene_08,
                                    scene_09,
                                    scene_0a,
                                    scene_17f,
                                    scene_186,
                                    scene_18b,
                                    scene_18c,
                                    scene_state_val,
                                    local_action84,
                                    bits_to_f32(local_action84),
                                    peterstu_ready_seq,
                                    synth_origin_requested,
                                    synth_origin_default_ready,
                                    title_patches,
                                    synth_origin_enabled,
                                    fire_after
                                ));
                            }
                            if synth_origin_enabled && synth_n >= fire_after {
                                write_guest_u8(r15, scene_this.wrapping_add(0x18C), 1);
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-ORIGIN-COMPLETE] #{} {} this=0x{:08X} flags8/9/A/17F/186/18B/18C={}/{}/{}/{}/{}/{}/{} state_val=0x{:08X} action84=0x{:08X}/{:.6}",
                                    synth_n,
                                    name,
                                    scene_this,
                                    scene_08,
                                    scene_09,
                                    scene_0a,
                                    scene_17f,
                                    scene_186,
                                    scene_18b,
                                    scene_18c,
                                    scene_state_val,
                                    local_action84,
                                    bits_to_f32(local_action84)
                                ));
                                if (synth_mode == "f8580" || synth_mode == "call")
                                    && synth_n == fire_after
                                {
                                    if let Some(target_host_off) = ctx.addr_hash.lookup(0x000F_8580)
                                    {
                                        let new_r14 = r14.wrapping_sub(8);
                                        write_guest_u32(r15, new_r14, 0x000F_293C);
                                        write_guest_u32(r15, new_r14.wrapping_add(4), 0);
                                        debug_log(&format!(
                                            "[SPIDEY-SYNTH-ORIGIN-COMPLETE-CALL] #{} -> F8580 return=0x000F293C old_esp=0x{:08X} new_esp=0x{:08X}",
                                            synth_n, r14, new_r14
                                        ));
                                        m.active = false;
                                        m.pending_rearm = true;
                                        #[cfg(windows)]
                                        {
                                            use windows::Win32::System::Memory::*;
                                            let target =
                                                unsafe { ctx.code_base.add(host_offset as usize) };
                                            let mut old_prot = PAGE_PROTECTION_FLAGS(0);
                                            if unsafe {
                                                VirtualProtect(
                                                    target as *const _,
                                                    1,
                                                    PAGE_EXECUTE_READWRITE,
                                                    &mut old_prot,
                                                )
                                            }
                                            .is_ok()
                                            {
                                                unsafe {
                                                    *target = original_byte;
                                                }
                                                let _ = unsafe {
                                                    VirtualProtect(
                                                        target as *const _,
                                                        1,
                                                        old_prot,
                                                        &mut old_prot,
                                                    )
                                                };
                                            }
                                        }
                                        context.R14 = new_r14 as u64;
                                        context.Rcx = scene_this as u64;
                                        context.Rip = code_base + target_host_off as u64;
                                        return true;
                                    }
                                    debug_log(
                                        "[SPIDEY-SYNTH-ORIGIN-COMPLETE-CALL-FAIL] F8580 missing host mapping",
                                    );
                                }
                            } else if !synth_origin_enabled && synth_n == fire_after {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGIN-COMPLETE-SKIP] this=0x{:08X} scene='{}' state_val={} left [+0x18C]=0",
                                    scene_this, scene_name, scene_state_val
                                ));
                            }
                        }
                    }
                    debug_log(&format!(
                        "[SPIDEY-F27-STATE4] {} this=0x{:08X} flags8/9/A/17F/186/18B={}/{}/{}/{}/{}/{} b4=0x{:08X} state_idx=0x{:08X} state_val=0x{:08X} xroot=0x{:08X} focus=0x{:08X} head=0x{:08X} first=0x{:08X} item=0x{:08X} item+8=0x{:02X} item_pc=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} item_stack=0x{:08X} item_ctr={} item_timer=0x{:08X} stack10=0x{:08X} stack24=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X}",
                        name,
                        scene_this,
                        scene_08,
                        scene_09,
                        scene_0a,
                        scene_17f,
                        scene_186,
                        scene_18b,
                        scene_b4,
                        scene_state_idx,
                        scene_state_val,
                        xroot,
                        focus_mgr,
                        focus_head,
                        focus_first,
                        focus_item,
                        focus_item_flags,
                        focus_item_pc,
                        focus_item_word,
                        focus_item_major,
                        focus_item_op,
                        focus_item_stack,
                        focus_item_counter,
                        focus_item_timer,
                        stack10,
                        stack24,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        context.Rbp as u32
                    ));
                }
                if name == "SPIDEY_FLAG17F_SCRIPT_SETTER_TAP" {
                    static FLAG17F_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = FLAG17F_LOG.fetch_add(1, AO::Relaxed);
                    let value_obj = read_guest_u32(r15, r14.wrapping_add(4));
                    let value_stack = if valid_guest_ptr(value_obj) {
                        read_guest_u32(r15, value_obj.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let value_bits = if valid_guest_ptr(value_stack.wrapping_sub(4)) {
                        read_guest_u32(r15, value_stack.wrapping_sub(4))
                    } else {
                        0
                    };
                    let target = read_guest_u32(r15, 0x004B_C614);
                    let scene_a0 = if valid_guest_ptr(target) {
                        read_guest_u32(r15, target.wrapping_add(0xA0))
                    } else {
                        0
                    };
                    let state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                        read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let flag8 = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let flag17f = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x17F))
                    } else {
                        0
                    };
                    let flag18b = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x18B))
                    } else {
                        0
                    };
                    if n < 64
                        || scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        || state_val == 3
                    {
                        debug_log(&format!(
                            "[SPIDEY-FLAG17F-SETTER] #{} target=0x{:08X} value_obj=0x{:08X} value_stack=0x{:08X} arg_bits=0x{:08X}/{:.6} pre flags8/17F/18B={}/{}/{} state_idx=0x{:08X} state_val=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} scene='{}' stash='{}'",
                            n,
                            target,
                            value_obj,
                            value_stack,
                            value_bits,
                            bits_to_f32(value_bits),
                            flag8,
                            flag17f,
                            flag18b,
                            state_idx,
                            state_val,
                            context.Rax as u32,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            scene_name,
                            stash_name
                        ));
                    }
                }
                if name == "SPIDEY_FLAG17F_CLEAR_POST_TAP" || name == "SPIDEY_FLAG17F_SET_POST_TAP"
                {
                    static FLAG17F_POST_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = FLAG17F_POST_LOG.fetch_add(1, AO::Relaxed);
                    let global_target = read_guest_u32(r15, 0x004B_C614);
                    let branch_target = if name == "SPIDEY_FLAG17F_CLEAR_POST_TAP" {
                        context.Rax as u32
                    } else {
                        context.Rcx as u32
                    };
                    let target = if valid_guest_ptr(branch_target) {
                        branch_target
                    } else {
                        global_target
                    };
                    let scene_a0 = if valid_guest_ptr(target) {
                        read_guest_u32(r15, target.wrapping_add(0xA0))
                    } else {
                        0
                    };
                    let state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                        read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let flag8 = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let flag17f = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x17F))
                    } else {
                        0
                    };
                    let flag18b = if valid_guest_ptr(target) {
                        read_guest_u8(r15, target.wrapping_add(0x18B))
                    } else {
                        0
                    };
                    debug_log(&format!(
                        "[SPIDEY-FLAG17F-POST] #{} {} target=0x{:08X} global=0x{:08X} branch_reg=0x{:08X} flags8/17F/18B={}/{}/{} state_idx=0x{:08X} state_val=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} scene='{}' stash='{}'",
                        n,
                        name,
                        target,
                        global_target,
                        branch_target,
                        flag8,
                        flag17f,
                        flag18b,
                        state_idx,
                        state_val,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        scene_name,
                        stash_name
                    ));
                }
                if name.starts_with("SPIDEY_SCRIPT_51C00") {
                    let loop_opcode_tap = name == "SPIDEY_SCRIPT_51C00_LOOP_OPCODE_TAP";
                    static SCRIPT_51C00_LOOP_DIAG: AtomicU32 = AtomicU32::new(0);
                    let loop_n = if loop_opcode_tap {
                        SCRIPT_51C00_LOOP_DIAG.fetch_add(1, AO::Relaxed)
                    } else {
                        0
                    };
                    let script = if name == "SPIDEY_SCRIPT_51C00_ENTRY_TAP" {
                        context.Rcx as u32
                    } else {
                        context.Rbp as u32
                    };
                    let script_pc = if loop_opcode_tap {
                        context.Rax as u32
                    } else {
                        read_guest_u32(r15, script.wrapping_add(0x1C))
                    };
                    let op_word = if loop_opcode_tap {
                        context.Rcx as u16
                    } else {
                        read_guest_u16(r15, script_pc)
                    };
                    let op = op_word & 0x007F;
                    let major = op_word >> 8;
                    let op_hint = if name.contains("OP2A") {
                        "0x2A"
                    } else if name.contains("OP2B") {
                        "0x2B"
                    } else if name.contains("OP2C") {
                        "0x2C"
                    } else if name.contains("OP2D") {
                        "0x2D"
                    } else {
                        "-"
                    };
                    let script_flags = read_guest_u8(r15, script.wrapping_add(0x08));
                    let script_stack = read_guest_u32(r15, script.wrapping_add(0x14));
                    let script_base = read_guest_u32(r15, script.wrapping_add(0x18));
                    let script_counter = read_guest_u32(r15, script.wrapping_add(0x44));
                    let script_timer = read_guest_u32(r15, script.wrapping_add(0x40));
                    let interesting_reg_op = (0x2A..=0x2D).contains(&major);
                    let should_log = !loop_opcode_tap
                        || loop_n < 256
                        || loop_n.is_power_of_two()
                        || interesting_reg_op;
                    if should_log {
                        debug_log(&format!(
                            "[SPIDEY-SCRIPT-51C00] {} loop#{} script=0x{:08X} flags8=0x{:02X} stack14=0x{:08X} base18=0x{:08X} pc1c=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} regop_hint={} ctr={} timer=0x{:08X} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} ebp=0x{:08X}",
                            name,
                            loop_n,
                            script,
                            script_flags,
                            script_stack,
                            script_base,
                            script_pc,
                            op_word,
                            major,
                            op,
                            op_hint,
                            script_counter,
                            script_timer,
                            context.Rax as u32,
                            context.Rax as u8,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rbp as u32
                        ));
                    }
                }
                if name.starts_with("SPIDEY_SCRIPT_VM_") {
                    let script = context.Rbp as u32;
                    let pc_after = read_guest_u32(r15, script.wrapping_add(0x1C));
                    let pc_from_stack = read_guest_u32(r15, r14.wrapping_add(0x20));
                    let instr_pc = pc_after.wrapping_sub(6);
                    let instr_pc = if valid_guest_ptr(pc_from_stack)
                        && matches!(read_guest_u16(r15, pc_from_stack), 0x1C08 | 0x1D08)
                    {
                        pc_from_stack
                    } else {
                        instr_pc
                    };
                    let word = read_guest_u16(r15, instr_pc);
                    let major = word >> 8;
                    let op = word & 0x007F;
                    let imm_hi = read_guest_u16(r15, instr_pc.wrapping_add(2)) as u32;
                    let imm_lo = read_guest_u16(r15, instr_pc.wrapping_add(4)) as u32;
                    let raw_id = (imm_hi << 16) | imm_lo;
                    let resolved_addr = context.Rbx as u32;
                    let copy_len = (context.Rdx as u32) & 0xFFFF;
                    let script_stack = read_guest_u32(r15, script.wrapping_add(0x14));
                    let mem_value = read_guest_u32(r15, resolved_addr);
                    let stack_src = script_stack.wrapping_sub(copy_len.max(4));
                    let stack_value = read_guest_u32(r15, stack_src);
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);
                    let is_target_id =
                        raw_id == 0x0002_0008 || matches!(instr_pc, 0x01F8_39F4 | 0x01F9_40F2);
                    if is_target_id {
                        static SCRIPT_VM_20008_LOG: AtomicU32 = AtomicU32::new(0);
                        let vn = SCRIPT_VM_20008_LOG.fetch_add(1, AO::Relaxed);
                        if vn == 0 && valid_guest_ptr(resolved_addr) {
                            let watch_host = r15.wrapping_add(resolved_addr as u64);
                            context.Dr0 = watch_host;
                            context.Dr1 = 0;
                            context.Dr2 = 0;
                            context.Dr3 = 0;
                            context.Dr7 = (1 << 0)
                                | (1 << 1)
                                | (1 << 8)
                                | (1 << 9)
                                | (0b01 << 16)
                                | (0b11 << 18);
                            debug_log(&format!(
                                "[HW-WATCH] DR0 context-replaced by SPIDEY_SCRIPT_VM_20008 guest=0x{:08X} host=0x{:016X} (Dr7=0x{:X})",
                                resolved_addr,
                                watch_host,
                                context.Dr7
                            ));
                        }
                        if vn < 64 || vn.is_power_of_two() || name == "SPIDEY_SCRIPT_VM_STORE8_TAP"
                        {
                            let mut bytes = String::new();
                            for i in 0..12u32 {
                                if i != 0 {
                                    bytes.push(' ');
                                }
                                bytes.push_str(&format!(
                                    "{:02X}",
                                    read_guest_u8(r15, instr_pc.wrapping_add(i))
                                ));
                            }
                            debug_log(&format!(
                                "[SPIDEY-SCRIPT-VM-20008] #{} {} scene='{}' stash='{}' script=0x{:08X} instr_pc=0x{:08X} pc_after=0x{:08X} pc_stack=0x{:08X} bytes=[{}] word=0x{:04X} major=0x{:02X} op=0x{:02X} raw_id=0x{:08X} resolved=0x{:08X} host=0x{:016X} copy_len={} mem=0x{:08X}/{:.6} stack_src=0x{:08X} stack=0x{:08X}/{:.6} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                                vn,
                                name,
                                scene_name,
                                stash_name,
                                script,
                                instr_pc,
                                pc_after,
                                pc_from_stack,
                                bytes,
                                word,
                                major,
                                op,
                                raw_id,
                                resolved_addr,
                                r15.wrapping_add(resolved_addr as u64),
                                copy_len,
                                mem_value,
                                bits_to_f32(mem_value),
                                stack_src,
                                stack_value,
                                bits_to_f32(stack_value),
                                context.Rax as u32,
                                context.Rbx as u32,
                                context.Rcx as u32,
                                context.Rdx as u32,
                                context.Rsi as u32,
                                context.Rdi as u32
                            ));
                        }
                    }
                }
                if name == "SPIDEY_SCRIPT_EVENT_SLOT_GET_TAP" {
                    static SCRIPT_EVENT_SLOT_GET_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = SCRIPT_EVENT_SLOT_GET_LOG.fetch_add(1, AO::Relaxed);
                    let requested_slot = read_guest_u32(r15, r14);
                    let event_obj = context.Rcx as u32;
                    let event_record = read_guest_u32(r15, event_obj.wrapping_add(0x08));
                    let table_slots = if valid_guest_ptr(event_record) {
                        read_guest_u32(r15, event_record.wrapping_add(0x04))
                    } else {
                        0
                    };
                    let table_count = if valid_guest_ptr(event_record) {
                        read_guest_u32(r15, event_record)
                    } else {
                        0
                    };
                    let current_value = if valid_guest_ptr(table_slots) && requested_slot < 0x100 {
                        read_guest_u32(
                            r15,
                            table_slots.wrapping_add(requested_slot.wrapping_mul(4)),
                        )
                    } else {
                        0
                    };
                    let script = context.Rbp as u32;
                    let script_pc = if valid_guest_ptr(script) {
                        read_guest_u32(r15, script.wrapping_add(0x1C))
                    } else {
                        0
                    };
                    let script_stack = if valid_guest_ptr(script) {
                        read_guest_u32(r15, script.wrapping_add(0x14))
                    } else {
                        0
                    };
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);
                    let frame = read_guest_u32(r15, 0x004B_C630);
                    let frame_scene = if valid_guest_ptr(frame) {
                        read_guest_u32(r15, frame.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let scene18c = if valid_guest_ptr(frame_scene) {
                        read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
                    } else {
                        0
                    };
                    let scene_a0 = if valid_guest_ptr(frame_scene) {
                        read_guest_u32(r15, frame_scene.wrapping_add(0xA0))
                    } else {
                        0
                    };
                    let state_idx = if valid_guest_ptr(scene_a0) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_arr = if valid_guest_ptr(scene_a0) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                        read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let should_log = n < 256
                        || requested_slot == 1
                        || requested_slot == 6
                        || requested_slot == 0x10
                        || requested_slot == 0x12
                        || requested_slot == 0x14
                        || requested_slot == 0x16;
                    if should_log {
                        debug_log(&format!(
                            "[SPIDEY-SCRIPT-EVENT-SLOT-GET] #{} slot=0x{:02X} value=0x{:08X} event_obj=0x{:08X} record=0x{:08X} count={} slots=0x{:08X} key_slots=[{}] script=0x{:08X} pc=0x{:08X} stack=0x{:08X} scene='{}' stash='{}' state_val={} scene18c={} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebp=0x{:08X}",
                            n,
                            requested_slot,
                            current_value,
                            event_obj,
                            event_record,
                            table_count,
                            table_slots,
                            spidey_event_slot_summary(r15, table_slots, table_count),
                            script,
                            script_pc,
                            script_stack,
                            scene_name,
                            stash_name,
                            state_val,
                            scene18c,
                            context.Rax as u32,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rbp as u32,
                        ));
                    }
                }
                if name == "SPIDEY_NATIVE_METHOD_61400_ENTRY_TAP" {
                    static NATIVE_61400_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = NATIVE_61400_LOG.fetch_add(1, AO::Relaxed);
                    let ret = read_guest_u32(r15, r14);
                    let arg1 = read_guest_u32(r15, r14.wrapping_add(4));
                    let arg2 = read_guest_u32(r15, r14.wrapping_add(8));
                    let arg1_vt = if valid_guest_ptr(arg1) {
                        read_guest_u32(r15, arg1)
                    } else {
                        0
                    };
                    let arg2_word = if valid_guest_ptr(arg2) {
                        read_guest_u32(r15, arg2)
                    } else {
                        0
                    };
                    let arg2_vt = if valid_guest_ptr(arg2_word) {
                        read_guest_u32(r15, arg2_word)
                    } else {
                        0
                    };
                    let global_mgr = read_guest_u32(r15, 0x004B_8F04);
                    let global_ctx = read_guest_u32(r15, 0x004B_8F08);
                    let global_handle = read_guest_u32(r15, 0x004B_8F0C);
                    let is_origin_z = scene_name.eq_ignore_ascii_case("levels\\origin_z");
                    let origin_globals_active =
                        global_mgr != 0x1046_5A00 && global_ctx != 0x1046_83E0;
                    if is_origin_z && origin_globals_active && arg2 == 0 && valid_guest_ptr(arg1) {
                        static ORIGIN_61400_WATCH_A: AtomicU32 = AtomicU32::new(0);
                        static ORIGIN_61400_WATCH_B: AtomicU32 = AtomicU32::new(0);
                        static ORIGIN_61400_WATCH_C: AtomicU32 = AtomicU32::new(0);
                        let a = ORIGIN_61400_WATCH_A.load(AO::Relaxed);
                        let b = ORIGIN_61400_WATCH_B.load(AO::Relaxed);
                        let c = ORIGIN_61400_WATCH_C.load(AO::Relaxed);
                        if arg1 != a && arg1 != b && arg1 != c {
                            for slot in [
                                &ORIGIN_61400_WATCH_A,
                                &ORIGIN_61400_WATCH_B,
                                &ORIGIN_61400_WATCH_C,
                            ] {
                                if slot
                                    .compare_exchange(0, arg1, AO::AcqRel, AO::Relaxed)
                                    .is_ok()
                                {
                                    break;
                                }
                            }
                        }

                        let w1 = ORIGIN_61400_WATCH_A.load(AO::Relaxed);
                        let w2 = ORIGIN_61400_WATCH_B.load(AO::Relaxed);
                        let w3 = ORIGIN_61400_WATCH_C.load(AO::Relaxed);
                        let menu_slot = 0x1046_5A48u32;
                        context.Dr0 = r15.wrapping_add(menu_slot as u64);
                        context.Dr1 = if valid_guest_ptr(w1) {
                            r15.wrapping_add(w1 as u64)
                        } else {
                            0
                        };
                        context.Dr2 = if valid_guest_ptr(w2) {
                            r15.wrapping_add(w2 as u64)
                        } else {
                            0
                        };
                        context.Dr3 = if valid_guest_ptr(w3) {
                            r15.wrapping_add(w3 as u64)
                        } else {
                            0
                        };
                        let mut dr7 = (1u64 << 8) | (1u64 << 9);
                        for slot in 0..4u64 {
                            let host = match slot {
                                0 => context.Dr0,
                                1 => context.Dr1,
                                2 => context.Dr2,
                                _ => context.Dr3,
                            };
                            if host != 0 {
                                dr7 |= (1u64 << (slot * 2))
                                    | (1u64 << (slot * 2 + 1))
                                    | (0b01u64 << (16 + slot * 4))
                                    | (0b11u64 << (18 + slot * 4));
                            }
                        }
                        context.Dr7 = dr7;

                        static ORIGIN_61400_WATCH_LOG: AtomicU32 = AtomicU32::new(0);
                        let wn = ORIGIN_61400_WATCH_LOG.fetch_add(1, AO::Relaxed);
                        if wn < 8 || wn.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-NATIVE-61400-WATCH] #{} DR0(menu)=0x{:08X} DR1=0x{:08X} DR2=0x{:08X} DR3=0x{:08X} Dr7=0x{:X}",
                                wn, menu_slot, w1, w2, w3, context.Dr7
                            ));
                        }
                    }
                    if n < 128 || n.is_power_of_two() || is_origin_z {
                        debug_log(&format!(
                            "[SPIDEY-NATIVE-61400] #{} scene='{}' stash='{}' esp=0x{:08X} ret=0x{:08X} arg1=0x{:08X} arg1_vt=0x{:08X} arg2=0x{:08X} arg2_word=0x{:08X}/{:.6} arg2_vt=0x{:08X} globals[4B8F04/08/0C]=0x{:08X}/0x{:08X}/0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                            n,
                            scene_name,
                            stash_name,
                            r14,
                            ret,
                            arg1,
                            arg1_vt,
                            arg2,
                            arg2_word,
                            bits_to_f32(arg2_word),
                            arg2_vt,
                            global_mgr,
                            global_ctx,
                            global_handle,
                            context.Rax as u32,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rsi as u32,
                            context.Rdi as u32
                        ));
                    }
                }
                if name.starts_with("SPIDEY_NATIVE_METHOD_78F00_") {
                    static NATIVE_78F00_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = NATIVE_78F00_LOG.fetch_add(1, AO::Relaxed);
                    let ret = read_guest_u32(r15, r14);
                    let arg1 = read_guest_u32(r15, r14.wrapping_add(4));
                    let arg2 = read_guest_u32(r15, r14.wrapping_add(8));
                    let arg1_stack = if valid_guest_ptr(arg1) {
                        read_guest_u32(r15, arg1.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let is_entry = name == "SPIDEY_NATIVE_METHOD_78F00_ENTRY_TAP";
                    let state = if is_entry {
                        arg1_stack.wrapping_sub(0x18)
                    } else {
                        arg1_stack
                    };
                    let state_obj = if valid_guest_ptr(state) {
                        read_guest_u32(r15, state)
                    } else {
                        0
                    };
                    let state_pos = if valid_guest_ptr(state) {
                        read_guest_u32(r15, state.wrapping_add(0x04))
                    } else {
                        0
                    };
                    let state_mid = if valid_guest_ptr(state) {
                        read_guest_u32(r15, state.wrapping_add(0x10))
                    } else {
                        0
                    };
                    let state_remaining = if valid_guest_ptr(state) {
                        read_guest_u32(r15, state.wrapping_add(0x14))
                    } else {
                        0
                    };
                    let arg1_ctx = if valid_guest_ptr(arg1) {
                        read_guest_u32(r15, arg1.wrapping_add(0x0C))
                    } else {
                        0
                    };
                    let xroot = read_guest_u32(r15, 0x004C_06B8);
                    let active_obj = if valid_guest_ptr(xroot) {
                        read_guest_u32(r15, xroot.wrapping_add(0x1E4))
                    } else {
                        0
                    };
                    let active_time = if valid_guest_ptr(active_obj) {
                        read_guest_u32(r15, active_obj.wrapping_add(0x1EC))
                    } else {
                        0
                    };
                    let frame_delta = if valid_guest_ptr(xroot) {
                        read_guest_u32(r15, xroot.wrapping_add(0x18C))
                    } else {
                        0
                    };
                    let time_scale = if valid_guest_ptr(xroot) {
                        read_guest_u32(r15, xroot.wrapping_add(0x440))
                    } else {
                        0
                    };
                    let is_origin_z = scene_name.eq_ignore_ascii_case("levels\\origin_z");
                    if n < 128 || n.is_power_of_two() || is_origin_z {
                        debug_log(&format!(
                            "[SPIDEY-NATIVE-78F00] #{} {} scene='{}' stash='{}' esp=0x{:08X} ret=0x{:08X} arg1=0x{:08X} arg2=0x{:08X} arg1_stack=0x{:08X} arg1_ctx=0x{:08X} state=0x{:08X} obj=0x{:08X} state04=0x{:08X}/{:.6} state10=0x{:08X}/{:.6} remain=0x{:08X}/{:.6} xroot=0x{:08X} active=0x{:08X} active_time=0x{:08X}/{:.6} frame_delta=0x{:08X}/{:.6} time_scale=0x{:08X}/{:.6} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                            n,
                            name,
                            scene_name,
                            stash_name,
                            r14,
                            ret,
                            arg1,
                            arg2,
                            arg1_stack,
                            arg1_ctx,
                            state,
                            state_obj,
                            state_pos,
                            bits_to_f32(state_pos),
                            state_mid,
                            bits_to_f32(state_mid),
                            state_remaining,
                            bits_to_f32(state_remaining),
                            xroot,
                            active_obj,
                            active_time,
                            bits_to_f32(active_time),
                            frame_delta,
                            bits_to_f32(frame_delta),
                            time_scale,
                            bits_to_f32(time_scale),
                            context.Rax as u32,
                            context.Rax as u8,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rsi as u32,
                            context.Rdi as u32
                        ));
                    }
                }
                if name.starts_with("SPIDEY_SCRIPT_NATIVE_") {
                    let (script, value_ptr, old_pc) = if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" {
                        let script = context.Rcx as u32;
                        let value_ptr = read_guest_u32(r15, r14.wrapping_add(4));
                        let old_pc = read_guest_u32(r15, r14.wrapping_add(8));
                        (script, value_ptr, old_pc)
                    } else if name == "SPIDEY_SCRIPT_NATIVE_PC_WRITE_TAP" {
                        let script = context.Rsi as u32;
                        let value_ptr = read_guest_u32(r15, r14.wrapping_add(0x08));
                        let old_pc = read_guest_u32(r15, r14.wrapping_add(0x0C));
                        (script, value_ptr, old_pc)
                    } else {
                        let script = context.Rsi as u32;
                        let value_ptr = read_guest_u32(r15, r14.wrapping_add(0x0C));
                        let old_pc = read_guest_u32(r15, r14.wrapping_add(0x10));
                        (script, value_ptr, old_pc)
                    };
                    let command_obj = read_guest_u32(r15, value_ptr);
                    let command_vt = read_guest_u32(r15, command_obj);
                    let command_call = read_guest_u32(r15, command_vt.wrapping_add(4));
                    let script_pc = read_guest_u32(r15, script.wrapping_add(0x1C));
                    let script_stack = read_guest_u32(r15, script.wrapping_add(0x14));
                    let wait_flag = read_guest_u32(r15, script.wrapping_add(0x2C));
                    let script_counter = read_guest_u32(r15, script.wrapping_add(0x44));
                    let script_timer = read_guest_u32(r15, script.wrapping_add(0x40));
                    let xroot = read_guest_u32(r15, 0x004C_06B8);
                    let focus_mgr = read_guest_u32(r15, xroot.wrapping_add(0x1A8));
                    let focus_head = if focus_mgr >= 0x1000 && valid_guest_ptr(focus_mgr) {
                        read_guest_u32(r15, focus_mgr.wrapping_add(0x10))
                    } else {
                        0
                    };
                    let focus_first = if focus_head >= 0x1000 && valid_guest_ptr(focus_head) {
                        read_guest_u32(r15, focus_head)
                    } else {
                        0
                    };
                    let focus_item = if focus_first >= 0x1000 && valid_guest_ptr(focus_first) {
                        read_guest_u32(r15, focus_first.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let target_focus_script =
                        script == focus_item || (0x0193_4500..0x0193_4700).contains(&script_pc);
                    let menu_native_name = match command_call {
                        0x0009_3E60 => Some("is_scene_anim_playing"),
                        0x0009_3720 => Some("started_intro_animation_playing"),
                        0x0009_EB90 => Some("is_shown"),
                        0x0009_ECC0 => Some("is_faded"),
                        0x0007_1120 => Some("wait_finished"),
                        0x0009_3590 => Some("set_timescale_factor"),
                        0x000A_37C0 => Some("hide_all_widgets"),
                        0x0006_B810 => Some("check_disk_space"),
                        _ => None,
                    };
                    if let Some(menu_native_name) = menu_native_name {
                        let force_anim_done = title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_MENU_BYPASS_ANIM_GATE").is_some()
                            && name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP"
                            && matches!(command_call, 0x0009_3E60 | 0x0009_3720);
                        if force_anim_done {
                            context.Rax &= !0xFF;
                        }
                        let xroot_delta = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x18C))
                        } else {
                            0
                        };
                        let xroot_timescale = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x440))
                        } else {
                            0
                        };
                        let frame_scene = read_guest_u32(r15, 0x004B_C630)
                            .checked_add(0)
                            .and_then(|frame| {
                                if valid_guest_ptr(frame) {
                                    Some(read_guest_u32(r15, frame.wrapping_add(0x18)))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(0);
                        let scene_widget_holder = if valid_guest_ptr(frame_scene) {
                            read_guest_u32(r15, frame_scene.wrapping_add(0x54))
                        } else {
                            0
                        };
                        let active_obj = if valid_guest_ptr(xroot) {
                            read_guest_u32(r15, xroot.wrapping_add(0x1E4))
                        } else {
                            0
                        };
                        let active_time = if valid_guest_ptr(active_obj) {
                            read_guest_u32(r15, active_obj.wrapping_add(0x1EC))
                        } else {
                            0
                        };
                        static MENU_NATIVE_GATE_LOG: AtomicU32 = AtomicU32::new(0);
                        let mn = MENU_NATIVE_GATE_LOG.fetch_add(1, AO::Relaxed);
                        if mn < 1024 || mn.is_power_of_two() || target_focus_script {
                            debug_log(&format!(
                                "[SPIDEY-MENU-NATIVE-GATE] #{} {} fn={} forced={} scene='{}' stash='{}' script=0x{:08X} old_pc=0x{:08X} script_pc=0x{:08X} command=0x{:08X} vt=0x{:08X} call=0x{:08X} xroot=0x{:08X} delta=0x{:08X}/{:.6} timescale=0x{:08X}/{:.6} active=0x{:08X} active_time=0x{:08X}/{:.6} focus_mgr=0x{:08X} focus_item=0x{:08X} scene=0x{:08X} holder=0x{:08X} wait={} ctr={} timer=0x{:08X} eax=0x{:08X} al=0x{:02X}",
                                mn,
                                name,
                                menu_native_name,
                                if force_anim_done { 1 } else { 0 },
                                scene_name,
                                stash_name,
                                script,
                                old_pc,
                                script_pc,
                                command_obj,
                                command_vt,
                                command_call,
                                xroot,
                                xroot_delta,
                                bits_to_f32(xroot_delta),
                                xroot_timescale,
                                bits_to_f32(xroot_timescale),
                                active_obj,
                                active_time,
                                bits_to_f32(active_time),
                                focus_mgr,
                                focus_item,
                                frame_scene,
                                scene_widget_holder,
                                wait_flag,
                                script_counter,
                                script_timer,
                                context.Rax as u32,
                                context.Rax as u8
                            ));
                            let event_table = SPIDEY_EVENT_TABLE_ADDR.load(AO::Relaxed);
                            let event_count = SPIDEY_EVENT_TABLE_COUNT.load(AO::Relaxed);
                            let event_slots = SPIDEY_EVENT_TABLE_SLOTS.load(AO::Relaxed);
                            let root_1a4 = if valid_guest_ptr(xroot) {
                                read_guest_u32(r15, xroot.wrapping_add(0x1A4))
                            } else {
                                0
                            };
                            let root_1a8 = if valid_guest_ptr(xroot) {
                                read_guest_u32(r15, xroot.wrapping_add(0x1A8))
                            } else {
                                0
                            };
                            let action_mgr = read_guest_u32(r15, 0x003F_7D90);
                            let action_active = if valid_guest_ptr(action_mgr) {
                                read_guest_u32(r15, action_mgr.wrapping_add(0x58))
                            } else {
                                0
                            };
                            let action_slot0 = if valid_guest_ptr(action_mgr) {
                                read_guest_u32(r15, action_mgr.wrapping_add(0x2C))
                            } else {
                                0
                            };
                            let action_slot1 = if valid_guest_ptr(action_mgr) {
                                read_guest_u32(r15, action_mgr.wrapping_add(0x30))
                            } else {
                                0
                            };
                            let pending_begin = read_guest_u32(r15, 0x003D_CEC0);
                            let pending_table = read_guest_u32(r15, 0x003D_CEC4);
                            let pending_end = read_guest_u32(r15, 0x003D_CEC8);
                            debug_log(&format!(
                                "[SPIDEY-MENU-DISK-FOCUS] #{} {} fn={} event_table=0x{:08X} count={} slots=0x{:08X} key_slots=[{}] active_slots=[{}] pending=0x{:08X}/0x{:08X}/0x{:08X} xapi=[{}] root+1A4/1A8=0x{:08X}/0x{:08X} action_mgr=0x{:08X} action+58=0x{:08X} action_slots=0x{:08X}/0x{:08X} focus_mgr=0x{:08X} focus_item=0x{:08X} scene=0x{:08X} holder=0x{:08X} eax=0x{:08X} al=0x{:02X}",
                                mn,
                                name,
                                menu_native_name,
                                event_table,
                                event_count,
                                event_slots,
                                spidey_event_slot_summary(r15, event_slots, event_count),
                                spidey_event_active_slot_summary(r15, event_slots, event_count),
                                pending_begin,
                                pending_table,
                                pending_end,
                                spidey_xapi_device_summary(r15),
                                root_1a4,
                                root_1a8,
                                action_mgr,
                                action_active,
                                action_slot0,
                                action_slot1,
                                focus_mgr,
                                focus_item,
                                frame_scene,
                                scene_widget_holder,
                                context.Rax as u32,
                                context.Rax as u8
                            ));
                        }
                    }
                    static ORIGINZ_AFTER_78F00_CAPTURE_REMAINING: AtomicU32 = AtomicU32::new(0);
                    static ORIGINZ_AFTER_78F00_CAPTURE_LOG: AtomicU32 = AtomicU32::new(0);
                    static ORIGINZ_67370_SYNTH_FIRED: AtomicU32 = AtomicU32::new(0);
                    if (name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP"
                        || name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP")
                        && command_call == 0x0007_8F00
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                    {
                        let arg_base = read_guest_u32(r15, script.wrapping_add(0x14));
                        let frame_delta_addr = xroot.wrapping_add(0x18C);
                        let mut frame_delta_bits = read_guest_u32(r15, frame_delta_addr);
                        let peterstu_ready_seq =
                            crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                        let synth_delta = name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP"
                            && peterstu_ready_seq != 0
                            && frame_delta_bits == 0
                            && valid_guest_ptr(frame_delta_addr)
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_SYNTH_FRAME_DELTA")
                                .is_some();
                        if synth_delta {
                            static ORIGINZ_FRAME_DELTA_SYNTH_LOG: AtomicU32 = AtomicU32::new(0);
                            let dn = ORIGINZ_FRAME_DELTA_SYNTH_LOG.fetch_add(1, AO::Relaxed);
                            frame_delta_bits = 0x3C88_8889; // 1.0 / 60.0
                            write_guest_u32(r15, frame_delta_addr, frame_delta_bits);
                            if dn < 16 || dn.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-ORIGINZ-FRAME-DELTA-POKE] #{} script=0x{:08X} pc=0x{:08X} xroot=0x{:08X} addr=0x{:08X} peterstu_seq={} value=0x{:08X}/{:.6}",
                                    dn,
                                    script,
                                    script_pc,
                                    xroot,
                                    frame_delta_addr,
                                    peterstu_ready_seq,
                                    frame_delta_bits,
                                    bits_to_f32(frame_delta_bits)
                                ));
                            }
                        }
                        let time_scale_bits = read_guest_u32(r15, xroot.wrapping_add(0x440));
                        let stack00 = read_guest_u32(r15, arg_base);
                        let stack04 = read_guest_u32(r15, arg_base.wrapping_add(0x04));
                        let stack08 = read_guest_u32(r15, arg_base.wrapping_add(0x08));
                        let stack0c = read_guest_u32(r15, arg_base.wrapping_add(0x0C));
                        let stack10 = read_guest_u32(r15, arg_base.wrapping_add(0x10));
                        let stack14 = read_guest_u32(r15, arg_base.wrapping_add(0x14));
                        static ORIGINZ_78F00_FLOW_LOG: AtomicU32 = AtomicU32::new(0);
                        let flow_n = ORIGINZ_78F00_FLOW_LOG.fetch_add(1, AO::Relaxed);
                        if flow_n < 10 {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-78F00-FLOW] #{} phase={} script=0x{:08X} pc=0x{:08X} arg=0x{:08X} frame_delta=0x{:08X}/{:.6} time_scale=0x{:08X}/{:.6} countdown=0x{:08X}/{:.6} stack=[0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6}] al={}",
                                flow_n,
                                if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" { "entry" } else { "exit" },
                                script,
                                script_pc,
                                arg_base,
                                frame_delta_bits,
                                bits_to_f32(frame_delta_bits),
                                time_scale_bits,
                                bits_to_f32(time_scale_bits),
                                stack14,
                                bits_to_f32(stack14),
                                stack00,
                                bits_to_f32(stack00),
                                stack04,
                                bits_to_f32(stack04),
                                stack08,
                                bits_to_f32(stack08),
                                stack0c,
                                bits_to_f32(stack0c),
                                stack10,
                                bits_to_f32(stack10),
                                stack14,
                                bits_to_f32(stack14),
                                context.Rax as u8
                            ));
                        }
                    }
                    if name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP"
                        && command_call == 0x0007_8F00
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                    {
                        let arg_base = read_guest_u32(r15, script.wrapping_add(0x14));
                        let anim_obj = read_guest_u32(r15, arg_base);
                        let blend_bits = read_guest_u32(r15, arg_base.wrapping_add(0x10));
                        let remain_bits = read_guest_u32(r15, arg_base.wrapping_add(0x14));
                        let bypass = title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_BYPASS_78F00").is_some();
                        static ORIGINZ_78F00_RET_LOG: AtomicU32 = AtomicU32::new(0);
                        let rn = ORIGINZ_78F00_RET_LOG.fetch_add(1, AO::Relaxed);
                        if bypass {
                            context.Rax = (context.Rax & !0xFF) | 1;
                        }
                        let completed = (context.Rax as u8) != 0;
                        if completed
                            && std::env::var_os("RUSTEMU_SPIDEY_CAPTURE_AFTER_78F00").is_some()
                        {
                            ORIGINZ_AFTER_78F00_CAPTURE_REMAINING.store(64, AO::Relaxed);
                            let arm_n = ORIGINZ_AFTER_78F00_CAPTURE_LOG.fetch_add(1, AO::Relaxed);
                            if arm_n < 32 || arm_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-CAPTURE-AFTER-78F00-ARM] #{} script=0x{:08X} pc=0x{:08X} arg=0x{:08X} obj=0x{:08X} remain=0x{:08X}/{:.6} al={} window=64",
                                    arm_n,
                                    script,
                                    script_pc,
                                    arg_base,
                                    anim_obj,
                                    remain_bits,
                                    bits_to_f32(remain_bits),
                                    context.Rax as u8
                                ));
                            }
                        }
                        if rn < 256 || rn.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-78F00-RET] #{} bypass={} script=0x{:08X} pc=0x{:08X} arg=0x{:08X} obj=0x{:08X} blend=0x{:08X}/{:.6} remain=0x{:08X}/{:.6} al={}",
                                rn,
                                if bypass { 1 } else { 0 },
                                script,
                                script_pc,
                                arg_base,
                                anim_obj,
                                blend_bits,
                                bits_to_f32(blend_bits),
                                remain_bits,
                                bits_to_f32(remain_bits),
                                context.Rax as u8
                            ));
                        }
                    }
                    if scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        && command_call != 0x0007_8F00
                        && (name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP"
                            || name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP")
                    {
                        let peterstu_ready_seq =
                            crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                        let broad_67370 = title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_SYNTH_ORIGIN_67370_BROAD")
                                .is_some()
                            && ORIGINZ_67370_SYNTH_FIRED.load(AO::Relaxed) != 0
                            && matches!(old_pc, 0x0194_ABFE | 0x0194_AD1E | 0x0194_AD44);
                        let synth_67370 = name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP"
                            && command_call == 0x0006_7370
                            && (old_pc == 0x0195_AF50 || broad_67370)
                            && peterstu_ready_seq != 0
                            && (context.Rax as u8) == 0
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_SYNTH_ORIGIN_67370").is_some();
                        if synth_67370 {
                            static ORIGINZ_67370_SYNTH_LOG: AtomicU32 = AtomicU32::new(0);
                            let sn = ORIGINZ_67370_SYNTH_LOG.fetch_add(1, AO::Relaxed);
                            ORIGINZ_67370_SYNTH_FIRED.fetch_add(1, AO::Relaxed);
                            let active_arg_base = script_stack.wrapping_sub(4);
                            let arg0 = read_guest_u32(r15, active_arg_base);
                            let arg1 = read_guest_u32(r15, active_arg_base.wrapping_add(0x04));
                            let arg2 = read_guest_u32(r15, active_arg_base.wrapping_add(0x08));
                            let arg3 = read_guest_u32(r15, active_arg_base.wrapping_add(0x0C));
                            context.Rax = (context.Rax & !0xFF) | 1;
                            if sn < 128 || sn.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-ORIGIN-67370] #{} broad={} script=0x{:08X} old_pc=0x{:08X} script_pc=0x{:08X} command=0x{:08X} vt=0x{:08X} peterstu_seq={} arg_base=0x{:08X} args=[0x{:08X},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6}] wait={} timer=0x{:08X} forced_al=1",
                                    sn,
                                    if broad_67370 { 1 } else { 0 },
                                    script,
                                    old_pc,
                                    script_pc,
                                    command_obj,
                                    command_vt,
                                    peterstu_ready_seq,
                                    active_arg_base,
                                    arg0,
                                    arg1,
                                    bits_to_f32(arg1),
                                    arg2,
                                    bits_to_f32(arg2),
                                    arg3,
                                    bits_to_f32(arg3),
                                    wait_flag,
                                    script_timer
                                ));
                            }
                        }
                        let remaining = ORIGINZ_AFTER_78F00_CAPTURE_REMAINING.load(AO::Relaxed);
                        if remaining > 0 {
                            ORIGINZ_AFTER_78F00_CAPTURE_REMAINING.fetch_sub(1, AO::Relaxed);
                            let capture_idx = 64u32.saturating_sub(remaining);
                            let active_arg_base = if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" {
                                script_stack.wrapping_sub(8)
                            } else {
                                script_stack.wrapping_sub(4)
                            };
                            let arg0 = read_guest_u32(r15, active_arg_base);
                            let arg1 = read_guest_u32(r15, active_arg_base.wrapping_add(0x04));
                            let arg2 = read_guest_u32(r15, active_arg_base.wrapping_add(0x08));
                            let arg3 = read_guest_u32(r15, active_arg_base.wrapping_add(0x0C));
                            let cb00 = read_guest_u32(r15, command_vt);
                            let cb04 = read_guest_u32(r15, command_vt.wrapping_add(0x04));
                            let cb08 = read_guest_u32(r15, command_vt.wrapping_add(0x08));
                            let cb0c = read_guest_u32(r15, command_vt.wrapping_add(0x0C));
                            let cb10 = read_guest_u32(r15, command_vt.wrapping_add(0x10));
                            let engine = read_guest_u32(r15, 0x004B_C614);
                            let state_top = if valid_guest_ptr(engine) {
                                read_guest_u32(r15, engine.wrapping_add(0x1C))
                            } else {
                                0
                            };
                            let state_idx = if valid_guest_ptr(state_top) {
                                read_guest_u32(r15, state_top.wrapping_add(0x04))
                            } else {
                                0
                            };
                            let state_arr = if valid_guest_ptr(state_top) {
                                read_guest_u32(r15, state_top.wrapping_add(0x08))
                            } else {
                                0
                            };
                            let state_val = if valid_guest_ptr(state_arr) {
                                read_guest_u32(
                                    r15,
                                    state_arr.wrapping_add(state_idx.wrapping_mul(4)),
                                )
                            } else {
                                0
                            };
                            let mut pc_bytes = String::new();
                            for i in 0..16u32 {
                                if i != 0 {
                                    pc_bytes.push(' ');
                                }
                                pc_bytes.push_str(&format!(
                                    "{:02X}",
                                    read_guest_u8(r15, old_pc.wrapping_add(i))
                                ));
                            }
                            debug_log(&format!(
                                "[SPIDEY-CAPTURE-AFTER-78F00] idx={} {} script=0x{:08X} old_pc=0x{:08X} script_pc=0x{:08X} pc_bytes=[{}] command=0x{:08X} vt=0x{:08X} cb00/04/08/0c/10=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} arg_base=0x{:08X} args=[0x{:08X},0x{:08X}/{:.6},0x{:08X}/{:.6},0x{:08X}/{:.6}] wait={} timer=0x{:08X} stack=0x{:08X} state_top=0x{:08X} state_idx={} state_val=0x{:08X} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X}",
                                capture_idx,
                                name,
                                script,
                                old_pc,
                                script_pc,
                                pc_bytes,
                                command_obj,
                                command_vt,
                                cb00,
                                cb04,
                                cb08,
                                cb0c,
                                cb10,
                                active_arg_base,
                                arg0,
                                arg1,
                                bits_to_f32(arg1),
                                arg2,
                                bits_to_f32(arg2),
                                arg3,
                                bits_to_f32(arg3),
                                wait_flag,
                                script_timer,
                                script_stack,
                                state_top,
                                state_idx,
                                state_val,
                                context.Rax as u32,
                                context.Rax as u8,
                                context.Rcx as u32,
                                context.Rdx as u32,
                                context.Rsi as u32
                            ));
                        }
                    }
                    static SCRIPT_NATIVE_LOG: AtomicU32 = AtomicU32::new(0);
                    let sn = SCRIPT_NATIVE_LOG.fetch_add(1, AO::Relaxed);
                    if target_focus_script || sn < 256 || sn.is_power_of_two() {
                        let old_word = read_guest_u16(r15, old_pc);
                        let cur_word = read_guest_u16(r15, script_pc);
                        let old_major = old_word >> 8;
                        let cur_major = cur_word >> 8;
                        let old_op = old_word & 0x007F;
                        let cur_op = cur_word & 0x007F;
                        let mut old_bytes = String::new();
                        let mut cur_bytes = String::new();
                        for i in 0..12u32 {
                            if i != 0 {
                                old_bytes.push(' ');
                                cur_bytes.push(' ');
                            }
                            old_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, old_pc.wrapping_add(i))
                            ));
                            cur_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, script_pc.wrapping_add(i))
                            ));
                        }
                        debug_log(&format!(
                            "[SPIDEY-SCRIPT-NATIVE] #{} {} scene='{}' stash='{}' script=0x{:08X} focus_item=0x{:08X} value_ptr=0x{:08X} command=0x{:08X} vt=0x{:08X} call=0x{:08X} old_pc=0x{:08X} old_word=0x{:04X} old_major=0x{:02X} old_op=0x{:02X} old_bytes=[{}] script_pc=0x{:08X} cur_word=0x{:04X} cur_major=0x{:02X} cur_op=0x{:02X} cur_bytes=[{}] stack=0x{:08X} wait={} ctr={} timer=0x{:08X} eax=0x{:08X} al=0x{:02X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X}",
                            sn,
                            name,
                            scene_name,
                            stash_name,
                            script,
                            focus_item,
                            value_ptr,
                            command_obj,
                            command_vt,
                            command_call,
                            old_pc,
                            old_word,
                            old_major,
                            old_op,
                            old_bytes,
                            script_pc,
                            cur_word,
                            cur_major,
                            cur_op,
                            cur_bytes,
                            script_stack,
                            wait_flag,
                            script_counter,
                            script_timer,
                            context.Rax as u32,
                            context.Rax as u8,
                            context.Rcx as u32,
                            context.Rdx as u32,
                            context.Rsi as u32
                        ));
                    }
                    if command_call == 0x0006_1400
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        && (name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP"
                            || name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP")
                    {
                        let call_arg_base = script_stack.wrapping_sub(8);
                        let ret_arg_base = script_stack.wrapping_sub(4);
                        let active_arg_base = if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" {
                            call_arg_base
                        } else {
                            ret_arg_base
                        };
                        let vm_arg0 = read_guest_u32(r15, active_arg_base);
                        let vm_arg1 = read_guest_u32(r15, active_arg_base.wrapping_add(4));
                        let cpu_ret = read_guest_u32(r15, r14);
                        let cpu_arg1 = read_guest_u32(r15, r14.wrapping_add(4));
                        let cpu_arg2 = read_guest_u32(r15, r14.wrapping_add(8));
                        let cpu_arg3 = read_guest_u32(r15, r14.wrapping_add(0x0C));
                        let obj_vt = if valid_guest_ptr(vm_arg0) {
                            read_guest_u32(r15, vm_arg0)
                        } else {
                            0
                        };
                        let obj_cb04 = if valid_guest_ptr(obj_vt) {
                            read_guest_u32(r15, obj_vt.wrapping_add(0x04))
                        } else {
                            0
                        };
                        let call0 = read_guest_u32(r15, call_arg_base);
                        let call1 = read_guest_u32(r15, call_arg_base.wrapping_add(4));
                        let ret0 = read_guest_u32(r15, ret_arg_base);
                        let ret1 = read_guest_u32(r15, ret_arg_base.wrapping_add(4));
                        let stack_m10 = read_guest_u32(r15, script_stack.wrapping_sub(0x10));
                        let stack_m0c = read_guest_u32(r15, script_stack.wrapping_sub(0x0C));
                        let stack_m08 = read_guest_u32(r15, script_stack.wrapping_sub(0x08));
                        let stack_m04 = read_guest_u32(r15, script_stack.wrapping_sub(0x04));
                        let stack_00 = read_guest_u32(r15, script_stack);
                        let stack_04 = read_guest_u32(r15, script_stack.wrapping_add(0x04));
                        let peterstu_ready_seq =
                            crate::xbox::emulator::spidey_peterstu_read_ready_seq();
                        let targeted_originz_61400_idx = match old_pc {
                            0x0193_2AFC => Some(0),
                            0x0193_AE12 => Some(1),
                            0x0193_D0B6 => Some(2),
                            0x0193_D284 => Some(3),
                            0x0192_B2A4 => Some(4),
                            _ => None,
                        };
                        let targeted_originz_61400 = targeted_originz_61400_idx.is_some();
                        let broad_synth_61400 = targeted_originz_61400
                            && wait_flag != 0
                            && ORIGINZ_67370_SYNTH_FIRED.load(AO::Relaxed) != 0
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_SYNTH_ORIGIN_61400_BROAD")
                                .is_some();
                        let post_67370_synth_61400 = if name
                            == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP"
                            && peterstu_ready_seq != 0
                            && (context.Rax as u8) == 0
                            && wait_flag == 0
                            && ORIGINZ_67370_SYNTH_FIRED.load(AO::Relaxed) != 0
                            && title_patches_enabled()
                            && std::env::var_os("RUSTEMU_SPIDEY_SYNTH_ORIGIN_61400_POST67370")
                                .is_some()
                        {
                            if let Some(idx) = targeted_originz_61400_idx {
                                static ORIGINZ_61400_POST67370_MASK: AtomicU32 = AtomicU32::new(0);
                                let bit = 1u32 << idx;
                                (ORIGINZ_61400_POST67370_MASK.fetch_or(bit, AO::Relaxed) & bit) == 0
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        let synth_61400 = name == "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP"
                            && peterstu_ready_seq != 0
                            && (context.Rax as u8) == 0
                            && (((valid_guest_ptr(call0) && valid_guest_ptr(call1))
                                && title_patches_enabled()
                                && std::env::var_os("RUSTEMU_SPIDEY_SYNTH_ORIGIN_61400")
                                    .is_some())
                                || broad_synth_61400
                                || post_67370_synth_61400);
                        if synth_61400 {
                            static ORIGINZ_61400_SYNTH_LOG: AtomicU32 = AtomicU32::new(0);
                            let sn = ORIGINZ_61400_SYNTH_LOG.fetch_add(1, AO::Relaxed);
                            context.Rax = (context.Rax & !0xFF) | 1;
                            if sn < 128 || sn.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-SYNTH-ORIGIN-61400] #{} mode={} script=0x{:08X} old_pc=0x{:08X} script_pc=0x{:08X} peterstu_seq={} vm_arg0=0x{:08X} vm_arg1=0x{:08X}/{} wait={} timer=0x{:08X} forced_al=1",
                                    sn,
                                    if post_67370_synth_61400 {
                                        2
                                    } else if broad_synth_61400 {
                                        1
                                    } else {
                                        0
                                    },
                                    script,
                                    old_pc,
                                    script_pc,
                                    peterstu_ready_seq,
                                    vm_arg0,
                                    vm_arg1,
                                    bits_to_f32(vm_arg1),
                                    wait_flag,
                                    script_timer
                                ));
                            }
                        }
                        let mut pc_bytes = String::new();
                        for i in 0..16u32 {
                            if i != 0 {
                                pc_bytes.push(' ');
                            }
                            pc_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, old_pc.wrapping_add(i))
                            ));
                        }
                        static ORIGINZ_61400_ARG_LOG: AtomicU32 = AtomicU32::new(0);
                        let an = ORIGINZ_61400_ARG_LOG.fetch_add(1, AO::Relaxed);
                        if an < 512 || an.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-ORIGINZ-61400-ARGS] #{} phase={} script=0x{:08X} old_pc=0x{:08X} script_pc=0x{:08X} pc_bytes=[{}] value_ptr=0x{:08X} command=0x{:08X} vt=0x{:08X} stack=0x{:08X} call_arg_base=0x{:08X} ret_arg_base=0x{:08X} active_arg_base=0x{:08X} vm_arg0=0x{:08X} vm_arg1=0x{:08X}/{} vm_arg0_vt=0x{:08X} vm_arg0_cb04=0x{:08X} call_args=[0x{:08X},0x{:08X}/{}] ret_args=[0x{:08X},0x{:08X}/{}] stack[-10..+04]=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}/{},0x{:08X},0x{:08X}] cpu_esp=0x{:08X} cpu=[ret=0x{:08X},arg1=0x{:08X},arg2=0x{:08X}/{},arg3=0x{:08X}] wait={} timer=0x{:08X} al=0x{:02X}",
                                an,
                                if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" {
                                    "entry"
                                } else {
                                    "exit"
                                },
                                script,
                                old_pc,
                                script_pc,
                                pc_bytes,
                                value_ptr,
                                command_obj,
                                command_vt,
                                script_stack,
                                call_arg_base,
                                ret_arg_base,
                                active_arg_base,
                                vm_arg0,
                                vm_arg1,
                                bits_to_f32(vm_arg1),
                                obj_vt,
                                obj_cb04,
                                call0,
                                call1,
                                bits_to_f32(call1),
                                ret0,
                                ret1,
                                bits_to_f32(ret1),
                                stack_m10,
                                stack_m0c,
                                stack_m08,
                                stack_m04,
                                bits_to_f32(stack_m04),
                                stack_00,
                                stack_04,
                                r14,
                                cpu_ret,
                                cpu_arg1,
                                cpu_arg2,
                                bits_to_f32(cpu_arg2),
                                cpu_arg3,
                                wait_flag,
                                script_timer,
                                context.Rax as u8
                            ));
                        }
                    }
                    if name == "SPIDEY_SCRIPT_NATIVE_WAIT_TAP"
                        || name == "SPIDEY_SCRIPT_NATIVE_PC_WRITE_TAP"
                    {
                        static SCRIPT_PC_WRITE_LOG: AtomicU32 = AtomicU32::new(0);
                        let pn = SCRIPT_PC_WRITE_LOG.fetch_add(1, AO::Relaxed);
                        let stack08 = read_guest_u32(r15, r14.wrapping_add(0x08));
                        let stack0c = read_guest_u32(r15, r14.wrapping_add(0x0C));
                        let stack10 = read_guest_u32(r15, r14.wrapping_add(0x10));
                        let write_value = if name == "SPIDEY_SCRIPT_NATIVE_PC_WRITE_TAP" {
                            context.Rcx as u32
                        } else {
                            stack10
                        };
                        let is_origin_z = scene_name.eq_ignore_ascii_case("levels\\origin_z");
                        if pn < 256
                            || pn.is_power_of_two()
                            || is_origin_z
                            || target_focus_script
                            || write_value == 0x0C00_0000
                            || stack10 == 0x0C00_0000
                        {
                            debug_log(&format!(
                                "[SPIDEY-SCRIPT-PC-WRITE] #{} {} scene='{}' script=0x{:08X} focus_item=0x{:08X} pc_before=0x{:08X} write_value=0x{:08X} ecx=0x{:08X} esp=0x{:08X} stack08=0x{:08X} stack0c=0x{:08X} stack10=0x{:08X} value_ptr=0x{:08X} command=0x{:08X} vt=0x{:08X} call=0x{:08X} eax=0x{:08X} al=0x{:02X} esi=0x{:08X}",
                                pn,
                                name,
                                scene_name,
                                script,
                                focus_item,
                                script_pc,
                                write_value,
                                context.Rcx as u32,
                                r14,
                                stack08,
                                stack0c,
                                stack10,
                                value_ptr,
                                command_obj,
                                command_vt,
                                command_call,
                                context.Rax as u32,
                                context.Rax as u8,
                                context.Rsi as u32
                            ));
                        }
                    }
                    if command_call == 0x0008_5390 && target_focus_script {
                        let arg_base = if name == "SPIDEY_SCRIPT_NATIVE_CALL_TAP" {
                            script_stack.wrapping_sub(8)
                        } else {
                            script_stack.wrapping_sub(4)
                        };
                        let arg0 = read_guest_u32(r15, arg_base);
                        let arg1 = read_guest_u32(r15, arg_base.wrapping_add(4));
                        let obj_vt = read_guest_u32(r15, arg0);
                        let obj_cb20 = read_guest_u32(r15, obj_vt.wrapping_add(0x20));
                        let name_ptr = read_guest_u32(r15, arg1.wrapping_add(4));
                        let name_text = if valid_guest_ptr(name_ptr) {
                            read_c_string(r15, name_ptr, 96)
                        } else {
                            String::new()
                        };
                        let result_word = read_guest_u32(r15, script_stack.wrapping_sub(4));
                        let mut pc_bytes = String::new();
                        for i in 0..16u32 {
                            if i != 0 {
                                pc_bytes.push(' ');
                            }
                            pc_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, old_pc.wrapping_add(i))
                            ));
                        }
                        static SCRIPT_85390_LOG: AtomicU32 = AtomicU32::new(0);
                        let hn = SCRIPT_85390_LOG.fetch_add(1, AO::Relaxed);
                        if hn < 256 || hn.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-SCRIPT-85390] #{} {} script=0x{:08X} old_pc=0x{:08X} pc_bytes=[{}] arg_base=0x{:08X} arg0_obj=0x{:08X} arg0_vt=0x{:08X} cb20=0x{:08X} arg1=0x{:08X} name_ptr=0x{:08X} name='{}' stack=0x{:08X} result[-4]=0x{:08X}/{} wait={} timer=0x{:08X}",
                                hn,
                                name,
                                script,
                                old_pc,
                                pc_bytes,
                                arg_base,
                                arg0,
                                obj_vt,
                                obj_cb20,
                                arg1,
                                name_ptr,
                                name_text,
                                script_stack,
                                result_word,
                                bits_to_f32(result_word),
                                wait_flag,
                                script_timer
                            ));
                        }
                    }
                    if command_call == 0x0006_7460 {
                        let xroot = read_guest_u32(r15, 0x004C_06B8);
                        let delta_bits = read_guest_u32(r15, xroot.wrapping_add(0x18C));
                        let scale_bits = read_guest_u32(r15, xroot.wrapping_add(0x440));
                        let stack_ctx = script.wrapping_add(0x0C);
                        let stack_top = read_guest_u32(r15, stack_ctx.wrapping_add(0x08));
                        let countdown_minus4_bits = read_guest_u32(r15, stack_top.wrapping_sub(4));
                        let countdown_bits = read_guest_u32(r15, stack_top);
                        let countdown_plus4_bits = read_guest_u32(r15, stack_top.wrapping_add(4));
                        let old_word = read_guest_u16(r15, old_pc);
                        let cur_word = read_guest_u16(r15, script_pc);
                        let old_major = old_word >> 8;
                        let cur_major = cur_word >> 8;
                        let old_op = old_word & 0x007F;
                        let cur_op = cur_word & 0x007F;
                        let mut old_bytes = String::new();
                        let mut cur_bytes = String::new();
                        for i in 0..12u32 {
                            if i != 0 {
                                old_bytes.push(' ');
                                cur_bytes.push(' ');
                            }
                            old_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, old_pc.wrapping_add(i))
                            ));
                            cur_bytes.push_str(&format!(
                                "{:02X}",
                                read_guest_u8(r15, script_pc.wrapping_add(i))
                            ));
                        }
                        static TIMER_67460_LOG: AtomicU32 = AtomicU32::new(0);
                        let tn = TIMER_67460_LOG.fetch_add(1, AO::Relaxed);
                        if tn < 512 || tn.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-TIMER-67460] #{} {} scene='{}' stash='{}' xroot=0x{:08X} delta=0x{:08X}/{:.6} scale=0x{:08X}/{:.6} stack_ctx=0x{:08X} stack_top=0x{:08X} cd[-4]=0x{:08X}/{:.6} cd[0]=0x{:08X}/{:.6} cd[+4]=0x{:08X}/{:.6} wait={} old_pc=0x{:08X} old_word=0x{:04X} old_major=0x{:02X} old_op=0x{:02X} old_bytes=[{}] script_pc=0x{:08X} cur_word=0x{:04X} cur_major=0x{:02X} cur_op=0x{:02X} cur_bytes=[{}]",
                                tn,
                                name,
                                scene_name,
                                stash_name,
                                xroot,
                                delta_bits,
                                bits_to_f32(delta_bits),
                                scale_bits,
                                bits_to_f32(scale_bits),
                                stack_ctx,
                                stack_top,
                                countdown_minus4_bits,
                                bits_to_f32(countdown_minus4_bits),
                                countdown_bits,
                                bits_to_f32(countdown_bits),
                                countdown_plus4_bits,
                                bits_to_f32(countdown_plus4_bits),
                                wait_flag,
                                old_pc,
                                old_word,
                                old_major,
                                old_op,
                                old_bytes,
                                script_pc,
                                cur_word,
                                cur_major,
                                cur_op,
                                cur_bytes
                            ));
                        }
                    }
                }
                if name == "SPIDEY_SCENE_ADVANCE_FLAG_SET_TAP" {
                    static ADVANCE_FLAG_SET_LOG: AtomicU32 = AtomicU32::new(0);
                    let an = ADVANCE_FLAG_SET_LOG.fetch_add(1, AO::Relaxed);
                    let target_scene = context.Rcx as u32;
                    let old_18c = read_guest_u8(r15, target_scene.wrapping_add(0x18C));
                    let frame = read_guest_u32(r15, 0x004B_C630);
                    let frame_scene = if valid_guest_ptr(frame) {
                        read_guest_u32(r15, frame.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let engine = read_guest_u32(r15, 0x004B_C614);
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);
                    debug_log(&format!(
                        "[SPIDEY-SCENE-ADVANCE-FLAG] #{} target=0x{:08X} old18c={} frame_scene=0x{:08X} engine=0x{:08X} matches_frame={} matches_engine={} scene='{}' stash='{}' eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                        an,
                        target_scene,
                        old_18c,
                        frame_scene,
                        engine,
                        target_scene == frame_scene,
                        target_scene == engine,
                        scene_name,
                        stash_name,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32
                    ));
                }
                if name == "SPIDEY_ORIGIN_MENU_ATTACH_TAP" {
                    static ORIGIN_MENU_ATTACH_LOG: AtomicU32 = AtomicU32::new(0);
                    let an = ORIGIN_MENU_ATTACH_LOG.fetch_add(1, AO::Relaxed);
                    let scene_this = context.Rbp as u32;
                    let target_focus = read_guest_u32(r15, scene_this.wrapping_add(0x1A8));
                    let target_vt = if valid_guest_ptr(target_focus) {
                        read_guest_u32(r15, target_focus)
                    } else {
                        0
                    };
                    let target_04 = if valid_guest_ptr(target_focus) {
                        read_guest_u32(r15, target_focus.wrapping_add(0x04))
                    } else {
                        0
                    };
                    let target_08 = if valid_guest_ptr(target_focus) {
                        read_guest_u32(r15, target_focus.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let target_10 = if valid_guest_ptr(target_focus) {
                        read_guest_u32(r15, target_focus.wrapping_add(0x10))
                    } else {
                        0
                    };
                    let target_14 = if valid_guest_ptr(target_focus) {
                        read_guest_u32(r15, target_focus.wrapping_add(0x14))
                    } else {
                        0
                    };
                    let scene_root = read_guest_u32(r15, scene_this.wrapping_add(0x28));
                    let scene_root_focus = if valid_guest_ptr(scene_root) {
                        read_guest_u32(r15, scene_root.wrapping_add(0x1A8))
                    } else {
                        0
                    };
                    let xroot = read_guest_u32(r15, 0x004C_06B8);
                    let xroot_focus = if valid_guest_ptr(xroot) {
                        read_guest_u32(r15, xroot.wrapping_add(0x1A8))
                    } else {
                        0
                    };
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);
                    let attach_name = read_c_string(r15, 0x0039_2B68, 64);
                    debug_log(&format!(
                        "[SPIDEY-ORIGIN-MENU-ATTACH] #{} pc=0x000F0E79 scene_this=0x{:08X} scene+1A8=0x{:08X} target_vt=0x{:08X} target+04/08/10/14=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} scene_root=0x{:08X} scene_root+1A8=0x{:08X} xroot=0x{:08X} xroot+1A8=0x{:08X} attach='{}' scene='{}' stash='{}' eax_pre=0x{:08X} ebx=0x{:08X} ebp=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                        an,
                        scene_this,
                        target_focus,
                        target_vt,
                        target_04,
                        target_08,
                        target_10,
                        target_14,
                        scene_root,
                        scene_root_focus,
                        xroot,
                        xroot_focus,
                        attach_name,
                        scene_name,
                        stash_name,
                        context.Rax as u32,
                        context.Rbx as u32,
                        context.Rbp as u32,
                        context.Rsi as u32,
                        context.Rdi as u32
                    ));
                }
                if name == "SPIDEY_BOOT_GATE_TAP" || name.starts_with("SPIDEY_F8580_") {
                    let (scene_this, scene_source, dt_arg) = if name == "SPIDEY_BOOT_GATE_TAP" {
                        (
                            context.Rcx as u32,
                            "ecx",
                            read_guest_u32(r15, r14.wrapping_add(4)),
                        )
                    } else if name == "SPIDEY_F8580_FINAL_CB_TAP" || name == "SPIDEY_F8580_RET_TAP"
                    {
                        (
                            read_guest_u32(r15, r14),
                            "saved_esi",
                            read_guest_u32(r15, r14.wrapping_add(8)),
                        )
                    } else if name == "SPIDEY_F8580_ADVANCE_CALL_TAP"
                        || name == "SPIDEY_F8580_POST_ADVANCE_TAP"
                    {
                        (
                            context.Rsi as u32,
                            "esi",
                            read_guest_u32(r15, r14.wrapping_add(12)),
                        )
                    } else {
                        (
                            context.Rsi as u32,
                            "esi",
                            read_guest_u32(r15, r14.wrapping_add(8)),
                        )
                    };
                    crate::xbox::emulator::note_spidey_synth_update_hook(name, scene_this);
                    let scene_181 = read_guest_u8(r15, scene_this.wrapping_add(0x181));
                    let scene_182 = read_guest_u8(r15, scene_this.wrapping_add(0x182));
                    let scene_184 = read_guest_u8(r15, scene_this.wrapping_add(0x184));
                    let scene_185 = read_guest_u8(r15, scene_this.wrapping_add(0x185));
                    let scene_186 = read_guest_u8(r15, scene_this.wrapping_add(0x186));
                    let scene_188 = read_guest_u8(r15, scene_this.wrapping_add(0x188));
                    let scene_189 = read_guest_u8(r15, scene_this.wrapping_add(0x189));
                    let scene_18b = read_guest_u8(r15, scene_this.wrapping_add(0x18B));
                    let scene_18c = read_guest_u8(r15, scene_this.wrapping_add(0x18C));
                    let scene_18e = read_guest_u8(r15, scene_this.wrapping_add(0x18E));
                    let scene_18f = read_guest_u8(r15, scene_this.wrapping_add(0x18F));
                    let root = read_guest_u32(r15, scene_this.wrapping_add(0x28));
                    let root_78 = read_guest_u32(r15, root.wrapping_add(0x78));
                    let root_7c = read_guest_u32(r15, root.wrapping_add(0x7C));
                    let root_134 = read_guest_u32(r15, root.wrapping_add(0x134));
                    let root_1e8 = read_guest_u8(r15, root.wrapping_add(0x1E8));
                    let xgraph_root = read_guest_u32(r15, 0x004C_06B8);
                    let xgraph_134 = read_guest_u32(r15, xgraph_root.wrapping_add(0x134));
                    let xgraph_cb_vt = read_guest_u32(r15, xgraph_134);
                    let xgraph_cb_308 = read_guest_u32(r15, xgraph_cb_vt.wrapping_add(0x308));
                    let app_obj = read_guest_u32(r15, 0x003F_5EB0);
                    let app_d4 = read_guest_u32(r15, app_obj.wrapping_add(0xD4));
                    let app_f4 = read_guest_u32(r15, app_obj.wrapping_add(0xF4));
                    let engine_ptr = read_guest_u32(r15, 0x004B_C614);
                    let engine_gate = read_guest_u8(r15, engine_ptr);
                    let update_mgr = read_guest_u32(r15, 0x003F_7D90);
                    let update_gate = read_guest_u8(r15, update_mgr.wrapping_add(0x24));
                    debug_log(&format!(
                        "[SPIDEY-F8580-GATES] hook={} source={} scene_this=0x{:08X} ecx=0x{:08X} esi=0x{:08X} flags181/182/184/185/186/188/189/18b/18c/18e/18f={}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{} root=0x{:08X} root78=0x{:08X} root7c=0x{:08X} root134=0x{:08X} root1e8={} xroot=0x{:08X} x134=0x{:08X} xvt=0x{:08X} xcb308=0x{:08X} app=0x{:08X} appD4=0x{:08X} appF4=0x{:08X} engine=0x{:08X} engineByte={} update=0x{:08X} update24={} dt=0x{:08X}",
                        name,
                        scene_source,
                        scene_this,
                        context.Rcx as u32,
                        context.Rsi as u32,
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
                        root,
                        root_78,
                        root_7c,
                        root_134,
                        root_1e8,
                        xgraph_root,
                        xgraph_134,
                        xgraph_cb_vt,
                        xgraph_cb_308,
                        app_obj,
                        app_d4,
                        app_f4,
                        engine_ptr,
                        engine_gate,
                        update_mgr,
                        update_gate,
                        dt_arg
                    ));
                }
                if name == "SPIDEY_SCENE_DD130_CALL_TAP"
                    || name == "SPIDEY_SCENE_AFTER_DD130_TAP"
                    || name.starts_with("SPIDEY_DD130_")
                {
                    let this = if name == "SPIDEY_SCENE_DD130_CALL_TAP"
                        || name == "SPIDEY_DD130_ENTRY_TAP"
                    {
                        context.Rcx as u32
                    } else if name == "SPIDEY_SCENE_AFTER_DD130_TAP" {
                        context.Rbp as u32
                    } else {
                        context.Rsi as u32
                    };
                    let s40 = read_guest_u32(r15, this.wrapping_add(0x40));
                    let s44 = read_guest_u32(r15, this.wrapping_add(0x44));
                    let s50 = read_guest_u32(r15, this.wrapping_add(0x50));
                    let s54 = read_guest_u32(r15, this.wrapping_add(0x54));
                    let vt40 = read_guest_u32(r15, s40);
                    let vt44 = read_guest_u32(r15, s44);
                    let vt50 = read_guest_u32(r15, s50);
                    let vt54 = read_guest_u32(r15, s54);
                    let slot40_120 = read_guest_u32(r15, vt40.wrapping_add(0x120));
                    let slot50_3c = read_guest_u32(r15, vt50.wrapping_add(0x3C));
                    let slot50_40 = read_guest_u32(r15, vt50.wrapping_add(0x40));
                    let slot54_3c = read_guest_u32(r15, vt54.wrapping_add(0x3C));
                    let slot54_40 = read_guest_u32(r15, vt54.wrapping_add(0x40));
                    debug_log(&format!(
                        "[SPIDEY-DD130-MGR] {} this=0x{:08X} s40=0x{:08X}/vt=0x{:08X}/slot120=0x{:08X} s44=0x{:08X}/vt=0x{:08X} s50=0x{:08X}/vt=0x{:08X}/slot3c=0x{:08X}/slot40=0x{:08X} s54=0x{:08X}/vt=0x{:08X}/slot3c=0x{:08X}/slot40=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} ebp=0x{:08X}",
                        name,
                        this,
                        s40,
                        vt40,
                        slot40_120,
                        s44,
                        vt44,
                        s50,
                        vt50,
                        slot50_3c,
                        slot50_40,
                        s54,
                        vt54,
                        slot54_3c,
                        slot54_40,
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rbp as u32
                    ));
                }
                if name.starts_with("SPIDEY_F0690_") {
                    let this = context.Rbp as u32;
                    let frame_ctx = read_guest_u32(r15, 0x004B_C630);
                    let frame_scene = if valid_guest_ptr(frame_ctx) {
                        read_guest_u32(r15, frame_ctx.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let global_scene = read_guest_u32(r15, 0x003F_5BEC);
                    let scene_a0 = read_guest_u32(r15, this.wrapping_add(0xA0));
                    let state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                    } else {
                        0
                    };
                    let state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                        read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                    } else {
                        0
                    };
                    let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                        read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                    } else {
                        0
                    };
                    let latch = read_guest_u32(r15, global_scene.wrapping_add(0x118));
                    let mut obj_addr = 0u32;
                    let mut obj_signal = 0u32;
                    if latch != 0 && !ctx.kernel_state.is_null() {
                        let state = unsafe {
                            &*(ctx.kernel_state as *const crate::xbox::kernel::KernelState)
                        };
                        if let Some(obj) = state.object_table.get(&latch) {
                            obj_addr = obj.guest_obj_addr;
                            obj_signal = read_guest_u32(r15, obj_addr.wrapping_add(0x04));
                        }
                    }
                    debug_log(&format!(
                        "[SPIDEY-F0690-PROBE] {} this=0x{:08X} frame_scene=0x{:08X} global_scene=0x{:08X} eax=0x{:08X}/al={} latch=0x{:08X} obj=0x{:08X} signal={} state_idx={} state={} scene+25/17F/183/186/18B/18C={}/{}/{}/{}/{}/{} frame+20=0x{:08X} scene='{}' stash='{}'",
                        name,
                        this,
                        frame_scene,
                        global_scene,
                        context.Rax as u32,
                        (context.Rax & 0xFF) as u8,
                        latch,
                        obj_addr,
                        obj_signal,
                        state_idx,
                        state_val,
                        read_guest_u8(r15, this.wrapping_add(0x25)),
                        read_guest_u8(r15, this.wrapping_add(0x17F)),
                        read_guest_u8(r15, this.wrapping_add(0x183)),
                        read_guest_u8(r15, this.wrapping_add(0x186)),
                        read_guest_u8(r15, this.wrapping_add(0x18B)),
                        read_guest_u8(r15, this.wrapping_add(0x18C)),
                        if valid_guest_ptr(frame_ctx) {
                            read_guest_u32(r15, frame_ctx.wrapping_add(0x20))
                        } else {
                            0
                        },
                        scene_name,
                        stash_name
                    ));
                }
                if name.starts_with("SPIDEY_E9D00_") {
                    let this = if name == "SPIDEY_E9D00_ENTRY_TAP" {
                        context.Rcx as u32
                    } else {
                        context.Rdi as u32
                    };
                    let frame_ctx = read_guest_u32(r15, 0x004B_C630);
                    let frame_scene = if valid_guest_ptr(frame_ctx) {
                        read_guest_u32(r15, frame_ctx.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let global_scene = read_guest_u32(r15, 0x003F_5BEC);
                    let scene_obj = read_guest_u32(r15, this.wrapping_add(0x0C));
                    let scene_obj_vt = read_guest_u32(r15, scene_obj);
                    let obj_4bca64 = read_guest_u32(r15, 0x004B_CA64);
                    let obj_4c01e4 = read_guest_u32(r15, 0x004C_01E4);
                    let obj_4c01e0 = read_guest_u32(r15, 0x004C_01E0);
                    let obj_4c01dc = read_guest_u32(r15, 0x004C_01DC);
                    debug_log(&format!(
                        "[SPIDEY-E9D00-PROBE] {} this=0x{:08X} frame_scene=0x{:08X} global_scene=0x{:08X} scene_obj=0x{:08X}/vt=0x{:08X} globals 4BCA64=0x{:08X} 4C01E4=0x{:08X} 4C01E0=0x{:08X} 4C01DC=0x{:08X} this+1A4/1A8=0x{:08X}/0x{:08X} this+40/44/50/54=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} flags25/17F/183/186/18B/18C={}/{}/{}/{}/{}/{} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X} esp=0x{:08X}",
                        name,
                        this,
                        frame_scene,
                        global_scene,
                        scene_obj,
                        scene_obj_vt,
                        obj_4bca64,
                        obj_4c01e4,
                        obj_4c01e0,
                        obj_4c01dc,
                        read_guest_u32(r15, this.wrapping_add(0x1A4)),
                        read_guest_u32(r15, this.wrapping_add(0x1A8)),
                        read_guest_u32(r15, this.wrapping_add(0x40)),
                        read_guest_u32(r15, this.wrapping_add(0x44)),
                        read_guest_u32(r15, this.wrapping_add(0x50)),
                        read_guest_u32(r15, this.wrapping_add(0x54)),
                        read_guest_u8(r15, this.wrapping_add(0x25)),
                        read_guest_u8(r15, this.wrapping_add(0x17F)),
                        read_guest_u8(r15, this.wrapping_add(0x183)),
                        read_guest_u8(r15, this.wrapping_add(0x186)),
                        read_guest_u8(r15, this.wrapping_add(0x18B)),
                        read_guest_u8(r15, this.wrapping_add(0x18C)),
                        context.Rax as u32,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        context.Rbp as u32,
                        context.R14 as u32
                    ));
                }
            }
        }

        if name == "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP"
            || name == "SPIDEY_FRAME_RENDER_PREP_TAP"
            || name == "SPIDEY_FRAME_RENDER_CALL_TAP"
            || name == "SPIDEY_FRAME_RENDER_ENTRY_TAP"
            || name == "SPIDEY_FRAME_RENDER_RET_TAP"
            || name == "SPIDEY_RENDER_BODY_READY_RET_TAP"
            || name == "SPIDEY_RENDER_BODY_SCENE_CALL_TAP"
            || name == "SPIDEY_RENDER_BODY_AFTER_SCENE_TAP"
            || name == "SPIDEY_RENDER_BODY_POST_SCENE_TAP"
            || name == "SPIDEY_RENDER_BODY_TAIL_CALL_TAP"
        {
            if name == "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP"
                && read_c_string(r15, 0x004B_C848, 64).eq_ignore_ascii_case("levels\\origin_z")
            {
                let xgraph_root = read_guest_u32(r15, 0x004C_06B8);
                let action_mgr = read_guest_u32(r15, 0x003F_7D90);
                if valid_guest_ptr(xgraph_root) {
                    let old_delta = read_guest_u32(r15, xgraph_root.wrapping_add(0x18C));
                    let force_delta = title_patches_enabled()
                        && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_SEED_FRAME_DELTA").is_some();
                    let force_render_ready = title_patches_enabled()
                        && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_FORCE_RENDER_READY").is_some();
                    if force_delta && old_delta == 0 {
                        write_guest_u32(r15, xgraph_root.wrapping_add(0x18C), 0x3DCC_CCCD);
                    }
                    let ready_before = if valid_guest_ptr(action_mgr) {
                        read_guest_u8(r15, action_mgr.wrapping_add(0x24))
                    } else {
                        0
                    };
                    if force_render_ready && valid_guest_ptr(action_mgr) && ready_before == 0 {
                        // 0x120D30 is the render-body readiness gate:
                        //   mov eax, [0x3f7d90]; mov al, [eax+0x24]; ret
                        // If the async ready signal never arrives, the frame
                        // renderer skips the scene draw path every frame.
                        write_guest_u8(r15, action_mgr.wrapping_add(0x24), 1);
                    }
                    static ORIGINZ_FRAME_DELTA_SEED_LOG: AtomicU32 = AtomicU32::new(0);
                    let dn = ORIGINZ_FRAME_DELTA_SEED_LOG.fetch_add(1, AO::Relaxed);
                    if dn < 32 || dn.is_power_of_two() || force_delta || force_render_ready {
                        let new_delta = read_guest_u32(r15, xgraph_root.wrapping_add(0x18C));
                        let ready_after = if valid_guest_ptr(action_mgr) {
                            read_guest_u8(r15, action_mgr.wrapping_add(0x24))
                        } else {
                            0
                        };
                        debug_log(&format!(
                            "[SPIDEY-ORIGINZ-FRAME-DELTA] #{} xgraph=0x{:08X} old=0x{:08X}/{:.6} new=0x{:08X}/{:.6} scale=0x{:08X} action_mgr=0x{:08X} ready24={}->{} force_delta={} force_ready={}",
                            dn,
                            xgraph_root,
                            old_delta,
                            f32::from_bits(old_delta),
                            new_delta,
                            f32::from_bits(new_delta),
                            read_guest_u32(r15, xgraph_root.wrapping_add(0x440)),
                            action_mgr,
                            ready_before,
                            ready_after,
                            if force_delta { 1 } else { 0 },
                            if force_render_ready { 1 } else { 0 }
                        ));
                    }
                }
            }
            if name == "SPIDEY_RENDER_BODY_READY_RET_TAP"
                && title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_ORIGINZ_FORCE_RENDER_READY").is_some()
                && crate::xbox::emulator::spidey_peterstu_read_ready_seq() != 0
            {
                let old_al = context.Rax as u8;
                if old_al == 0 {
                    context.Rax = (context.Rax & !0xFF) | 1;
                    static ORIGINZ_READY_RET_FORCE_LOG: AtomicU32 = AtomicU32::new(0);
                    let n = ORIGINZ_READY_RET_FORCE_LOG.fetch_add(1, AO::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        let action_mgr = read_guest_u32(r15, 0x003F_7D90);
                        let ready24 = if valid_guest_ptr(action_mgr) {
                            read_guest_u8(r15, action_mgr.wrapping_add(0x24))
                        } else {
                            0
                        };
                        debug_log(&format!(
                            "[SPIDEY-ORIGINZ-READY-RET-FORCE] #{} scene='{}' peterstu_seq={} old_al={} new_al=1 action_mgr=0x{:08X} ready24={}",
                            n,
                            read_c_string(r15, 0x004B_C848, 64),
                            crate::xbox::emulator::spidey_peterstu_read_ready_seq(),
                            old_al,
                            action_mgr,
                            ready24
                        ));
                    }
                }
            }
            static FRAME_RENDER_DIAG: AtomicU32 = AtomicU32::new(0);
            let rn = FRAME_RENDER_DIAG.fetch_add(1, AO::Relaxed);
            if rn < 160 || rn.is_power_of_two() {
                let app = read_guest_u32(r15, 0x003F_5EB0);
                let scene = read_guest_u32(r15, 0x003F_5BEC);
                let frame = read_guest_u32(r15, 0x004B_C630);
                let xgraph_root = read_guest_u32(r15, 0x004C_06B8);
                let frame_scene = read_guest_u32(r15, frame.wrapping_add(0x18));
                let frame_20 = read_guest_u32(r15, frame.wrapping_add(0x20));
                let scene_root = read_guest_u32(r15, scene.wrapping_add(0x28));
                let scene_cur = read_guest_u32(r15, scene.wrapping_add(0x38));
                let scene_gate = read_guest_u8(r15, scene.wrapping_add(0x186));
                let scene_25 = read_guest_u8(r15, scene.wrapping_add(0x25));
                let root_count = read_guest_u32(r15, scene_root.wrapping_add(0x154));
                let root_first = read_guest_u32(r15, scene_root.wrapping_add(0x144));
                let root_last = read_guest_u32(r15, scene_root.wrapping_add(0x15C));
                let root_active = read_guest_u32(r15, scene_root.wrapping_add(0x1E4));
                let root_active_flag = read_guest_u8(r15, scene_root.wrapping_add(0x1E8));
                let x_count = read_guest_u32(r15, xgraph_root.wrapping_add(0x154));
                let x_first = read_guest_u32(r15, xgraph_root.wrapping_add(0x144));
                let x_last = read_guest_u32(r15, xgraph_root.wrapping_add(0x15C));
                let x_active = read_guest_u32(r15, xgraph_root.wrapping_add(0x1E4));
                let x_active_flag = read_guest_u8(r15, xgraph_root.wrapping_add(0x1E8));
                debug_log(&format!(
                    "[SPIDEY-FRAME-RENDER] #{} {} guest=0x{:08X} ret=0x{:08X} app=0x{:08X} frame=0x{:08X} frame_scene=0x{:08X} frame20=0x{:08X} scene=0x{:08X} scene25={} scene186={} scene_root=0x{:08X} scene_cur=0x{:08X} root(count={} first=0x{:08X} last=0x{:08X} active=0x{:08X} flag={}) xgraph=0x{:08X} x(count={} first=0x{:08X} last=0x{:08X} active=0x{:08X} flag={}) ecx=0x{:08X} esi=0x{:08X}",
                    rn,
                    name,
                    guest_addr,
                    ret_addr,
                    app,
                    frame,
                    frame_scene,
                    frame_20,
                    scene,
                    scene_25,
                    scene_gate,
                    scene_root,
                    scene_cur,
                    root_count,
                    root_first,
                    root_last,
                    root_active,
                    root_active_flag,
                    xgraph_root,
                    x_count,
                    x_first,
                    x_last,
                    x_active,
                    x_active_flag,
                    context.Rcx as u32,
                    context.Rsi as u32
                ));
            }
        }

        if name == "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP" || name == "SPIDEY_UPDATE_AFTER_CB_TAP" {
            static UPDATE_DISPATCH_DIAG: AtomicU32 = AtomicU32::new(0);
            let un = UPDATE_DISPATCH_DIAG.fetch_add(1, AO::Relaxed);
            if un < 96 || un.is_power_of_two() {
                let upd_this = if name == "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP" {
                    context.Rcx as u32
                } else {
                    unsafe { *((r15 + 0x003F_7D90) as *const u32) }
                };
                let begin = read_guest_u32(r15, upd_this.wrapping_add(0x08));
                let end = read_guest_u32(r15, upd_this.wrapping_add(0x0C));
                let len_slots = if valid_guest_ptr(begin) && valid_guest_ptr(end) && end >= begin {
                    (end - begin) / 4
                } else {
                    0
                };
                let cur_slot = if name == "SPIDEY_UPDATE_AFTER_CB_TAP" {
                    context.Rsi as u32
                } else {
                    begin
                };
                let cur_obj = read_guest_u32(r15, cur_slot);
                let cur_vtable = read_guest_u32(r15, cur_obj);
                let cur_cb20 = read_guest_u32(r15, cur_vtable.wrapping_add(0x20));
                let first_obj = read_guest_u32(r15, begin);
                let first_vtable = read_guest_u32(r15, first_obj);
                let first_cb20 = read_guest_u32(r15, first_vtable.wrapping_add(0x20));
                let this58 = read_guest_u32(r15, upd_this.wrapping_add(0x58));
                debug_log(&format!(
                    "[SPIDEY-UPD-DIAG #{}] {} this=0x{:08X} list=0x{:08X}..0x{:08X} slots={} cur_slot=0x{:08X} cur_obj=0x{:08X} vt=0x{:08X} cb20=0x{:08X} first_obj=0x{:08X} first_vt=0x{:08X} first_cb20=0x{:08X} this+58=0x{:08X} eax=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                    un,
                    name,
                    upd_this,
                    begin,
                    end,
                    len_slots,
                    cur_slot,
                    cur_obj,
                    cur_vtable,
                    cur_cb20,
                    first_obj,
                    first_vtable,
                    first_cb20,
                    this58,
                    context.Rax as u32,
                    context.Rsi as u32,
                    context.Rdi as u32
                ));
            }
        }

        if name == "SPIDEY_INPUT_CB_ENTRY_TAP" || name == "SPIDEY_INPUT_CB_XINPUT_CALL_TAP" {
            static INPUT_CB_DIAG: AtomicU32 = AtomicU32::new(0);
            let input_n = INPUT_CB_DIAG.fetch_add(1, AO::Relaxed);
            if input_n < 32 || input_n.is_power_of_two() {
                let obj = context.Rcx as u32;
                let handle = read_guest_u32(r15, obj.wrapping_add(0x50));
                let disabled = read_guest_u32(r15, obj.wrapping_add(0x9C));
                let port = read_guest_u32(r15, obj.wrapping_add(0xA4));
                let device_index = read_guest_u32(r15, obj.wrapping_add(0x04));
                let mgr = unsafe { *((r15 + 0x003F_7D90) as *const u32) };
                let mgr_active = read_guest_u32(r15, mgr.wrapping_add(0x58));
                let engine = unsafe { *((r15 + 0x004B_C614) as *const u32) };
                let engine10 = if valid_guest_ptr(engine) {
                    unsafe { *((r15 + engine as u64 + 0x10) as *const u8) }
                } else {
                    0
                };
                let engine11 = if valid_guest_ptr(engine) {
                    unsafe { *((r15 + engine as u64 + 0x11) as *const u8) }
                } else {
                    0
                };
                debug_log(&format!(
                    "[SPIDEY-INPUT-CB #{}] {} obj=0x{:08X} handle=0x{:08X} disabled=0x{:08X} port=0x{:08X} device_index=0x{:08X} mgr=0x{:08X} mgr+58=0x{:08X} engine=0x{:08X} engine+10/11={}/{} esi=0x{:08X} edi=0x{:08X} eax=0x{:08X} edx=0x{:08X}",
                    input_n,
                    name,
                    obj,
                    handle,
                    disabled,
                    port,
                    device_index,
                    mgr,
                    mgr_active,
                    engine,
                    engine10,
                    engine11,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    context.Rax as u32,
                    context.Rdx as u32
                ));
            }
        }

        if name.starts_with("SPIDEY_SELECT_PRESSED_") {
            static SELECT_PRESSED_DIAG: AtomicU32 = AtomicU32::new(0);
            static SELECT_FORCE_FALLTHROUGH_POLLS: AtomicU32 = AtomicU32::new(0);
            let sel_n = SELECT_PRESSED_DIAG.fetch_add(1, AO::Relaxed);
            let is_registration = name == "SPIDEY_SELECT_PRESSED_REGISTER_TAP";
            let is_emit = name == "SPIDEY_SELECT_PRESSED_EMIT_TAP";

            if name == "SPIDEY_SELECT_PRESSED_CHECK_TAP" {
                let input = context.Rsi as u32;
                let held_level = oovpa_hle::spidey_manual_confirm_held();
                let pending_edge = if valid_guest_ptr(input) {
                    oovpa_hle::take_spidey_manual_confirm_edge()
                } else {
                    false
                };
                let selector_1a_probe = std::env::var_os("RUSTEMU_SPIDEY_SELECTOR_1A_PROBE")
                    .is_some()
                    && oovpa_hle::spidey_menu_selector_seen();
                if valid_guest_ptr(input)
                    && (title_patches_enabled() || selector_1a_probe)
                    && (held_level || pending_edge)
                {
                    let cur_packet = read_guest_u32(r15, input.wrapping_add(0x08));
                    let cur_buttons = read_guest_u16(r15, input.wrapping_add(0x0C));
                    let prev_packet = read_guest_u32(r15, input.wrapping_add(0x1E));
                    let prev_buttons = read_guest_u16(r15, input.wrapping_add(0x22));
                    let forced_packet = if cur_packet == prev_packet {
                        cur_packet.wrapping_add(1)
                    } else {
                        cur_packet
                    };
                    write_guest_u32(r15, input.wrapping_add(0x08), forced_packet);
                    write_guest_u16(r15, input.wrapping_add(0x0C), cur_buttons | 0x0020);
                    write_guest_u16(r15, input.wrapping_add(0x22), prev_buttons & !0x0020);
                    SELECT_FORCE_FALLTHROUGH_POLLS.store(4, AO::Relaxed);

                    static SELECT_PENDING_EDGE_LOG: AtomicU32 = AtomicU32::new(0);
                    let edge_n = SELECT_PENDING_EDGE_LOG.fetch_add(1, AO::Relaxed);
                    if edge_n < 16 || edge_n.is_power_of_two() {
                        debug_log(&format!(
                            "[SPIDEY-SELECT-ACTION15] #{} mode={} source={} input=0x{:08X} cur(pkt:{} btn:0x{:04X})->(pkt:{} btn:0x{:04X}) prev(pkt:{} btn:0x{:04X})->btn:0x{:04X} guest=0x{:08X} ret=0x{:08X}",
                            edge_n,
                            if pending_edge { "edge" } else { "held" },
                            if title_patches_enabled() { "title-patch" } else { "selector-1a-probe" },
                            input,
                            cur_packet,
                            cur_buttons,
                            forced_packet,
                            cur_buttons | 0x0020,
                            prev_packet,
                            prev_buttons,
                            prev_buttons & !0x0020,
                            guest_addr,
                            ret_addr
                        ));
                    }
                }
            }

            let should_log = is_registration || is_emit || sel_n < 128 || sel_n.is_power_of_two();
            if should_log {
                let read_field_u32 = |base: u32, off: u32| -> u32 {
                    if valid_guest_ptr(base) {
                        read_guest_u32(r15, base.wrapping_add(off))
                    } else {
                        0
                    }
                };
                let read_field_u16 = |base: u32, off: u32| -> u16 {
                    if valid_guest_ptr(base) {
                        read_guest_u16(r15, base.wrapping_add(off))
                    } else {
                        0
                    }
                };
                let read_field_u8 = |base: u32, off: u32| -> u8 {
                    if valid_guest_ptr(base) {
                        read_guest_u8(r15, base.wrapping_add(off))
                    } else {
                        0
                    }
                };

                let xroot = read_guest_u32(r15, 0x004C_06B8);
                let xroot_focus = read_field_u32(xroot, 0x1A8);
                let global_scene = read_guest_u32(r15, 0x003F_5BEC);
                let global_root = read_field_u32(global_scene, 0x28);
                let global_focus = read_field_u32(global_root, 0x1A8);
                let frame = read_guest_u32(r15, 0x004B_C630);
                let frame_scene = read_field_u32(frame, 0x18);
                let frame_root = read_field_u32(frame_scene, 0x28);
                let frame_focus = read_field_u32(frame_root, 0x1A8);
                let same_global_focus = xroot_focus != 0 && xroot_focus == global_focus;
                let same_frame_focus = xroot_focus != 0 && xroot_focus == frame_focus;

                let esi_input = context.Rsi as u32;
                let ecx_input = context.Rcx as u32;
                let esi_vt = read_field_u32(esi_input, 0);
                let ecx_vt = read_field_u32(ecx_input, 0);
                let esi_cb10 = read_field_u32(esi_vt, 0x10);
                let esi_cb1c = read_field_u32(esi_vt, 0x1C);
                let ecx_cb10 = read_field_u32(ecx_vt, 0x10);
                let ecx_cb1c = read_field_u32(ecx_vt, 0x1C);
                let cur_packet = read_field_u32(esi_input, 0x08);
                let cur_buttons = read_field_u16(esi_input, 0x0C);
                let cur_a = read_field_u8(esi_input, 0x0E);
                let prev_packet = read_field_u32(esi_input, 0x1E);
                let prev_buttons = read_field_u16(esi_input, 0x22);
                let prev_a = read_field_u8(esi_input, 0x24);
                let connected = read_field_u32(esi_input, 0x9C);
                let prev_connected = read_field_u32(esi_input, 0xA0);
                let port = read_field_u32(esi_input, 0xA4);
                let handle = read_field_u32(esi_input, 0x50);
                let status_ax = context.Rax as u16;
                let status_ah = ((context.Rax as u32) >> 8) as u8;
                let select_event_global = read_guest_u32(r15, 0x003F_87A8);
                let pending_flag = read_guest_u32(r15, 0x003D_CFA0);
                let pending_begin = read_guest_u32(r15, 0x003D_CEC0);
                let pending_table = read_guest_u32(r15, 0x003D_CEC4);

                debug_log(&format!(
                    "[SPIDEY-SELECT-PRESSED #{}] {} fired={} guest=0x{:08X} ret=0x{:08X} ecx=0x{:08X}/vt=0x{:08X}/cb10=0x{:08X}/cb1c=0x{:08X} esi=0x{:08X}/vt=0x{:08X}/cb10=0x{:08X}/cb1c=0x{:08X} eax=0x{:08X} ax=0x{:04X} ah=0x{:02X} event=0x1A button=0x15 handle=0x{:08X} conn={}/{} port={} cur(pkt:{} btn:0x{:04X} A:{}) prev(pkt:{} btn:0x{:04X} A:{}) xroot=0x{:08X} xfocus=0x{:08X} global_scene=0x{:08X} global_root=0x{:08X} global_focus=0x{:08X} frame_scene=0x{:08X} frame_root=0x{:08X} frame_focus=0x{:08X} match_global={} match_frame={} signaller=0x{:08X} pending=0x{:08X} pend_begin=0x{:08X} pend_table=0x{:08X}",
                    sel_n,
                    name,
                    if is_emit { 1 } else { 0 },
                    guest_addr,
                    ret_addr,
                    ecx_input,
                    ecx_vt,
                    ecx_cb10,
                    ecx_cb1c,
                    esi_input,
                    esi_vt,
                    esi_cb10,
                    esi_cb1c,
                    context.Rax as u32,
                    status_ax,
                    status_ah,
                    handle,
                    connected,
                    prev_connected,
                    port,
                    cur_packet,
                    cur_buttons,
                    cur_a,
                    prev_packet,
                    prev_buttons,
                    prev_a,
                    xroot,
                    xroot_focus,
                    global_scene,
                    global_root,
                    global_focus,
                    frame_scene,
                    frame_root,
                    frame_focus,
                    same_global_focus,
                    same_frame_focus,
                    select_event_global,
                    pending_flag,
                    pending_begin,
                    pending_table
                ));

                if is_emit {
                    let dispatch_obj = context.Rbx as u32;
                    let dispatch_vt = read_field_u32(dispatch_obj, 0);
                    let dispatch_cb10 = read_field_u32(dispatch_vt, 0x10);
                    let dispatch_cb1c = read_field_u32(dispatch_vt, 0x1C);
                    let obj_root = read_field_u32(dispatch_obj, 0x28);
                    let obj_58 = read_field_u32(dispatch_obj, 0x58);
                    let obj_78 = read_field_u32(dispatch_obj, 0x78);
                    let obj_7c = read_field_u32(dispatch_obj, 0x7C);
                    let obj_a0 = read_field_u32(dispatch_obj, 0xA0);
                    let frame_delta = dispatch_obj.wrapping_sub(frame_scene);
                    let global_delta = dispatch_obj.wrapping_sub(global_scene);
                    debug_log(&format!(
                        "[SPIDEY-EVENT-73560-PREDICT #{}] event=0x1A next_ecx_from_ebx=0x{:08X} vt=0x{:08X} cb10=0x{:08X} cb1c=0x{:08X} frame_scene=0x{:08X} frame_delta=0x{:08X} global_scene=0x{:08X} global_delta=0x{:08X} obj+28=0x{:08X} obj+58=0x{:08X} obj+78=0x{:08X} obj+7c=0x{:08X} obj+a0=0x{:08X} global_pending=0x{:08X} global_pend_begin=0x{:08X} global_pend_table=0x{:08X}",
                        sel_n,
                        dispatch_obj,
                        dispatch_vt,
                        dispatch_cb10,
                        dispatch_cb1c,
                        frame_scene,
                        frame_delta,
                        global_scene,
                        global_delta,
                        obj_root,
                        obj_58,
                        obj_78,
                        obj_7c,
                        obj_a0,
                        pending_flag,
                        pending_begin,
                        pending_table
                    ));
                }
            }

            if name == "SPIDEY_SELECT_PRESSED_CB10_RET_TAP"
                || name == "SPIDEY_SELECT_PRESSED_CB1C_RET_TAP"
            {
                let slot = if name == "SPIDEY_SELECT_PRESSED_CB10_RET_TAP" {
                    0x10
                } else {
                    0x1C
                };
                let input = context.Rsi as u32;
                let vtable = read_guest_u32(r15, input);
                let target = read_guest_u32(r15, vtable.wrapping_add(slot));
                debug_log(&format!(
                    "[SPIDEY-SELECT-VTRET] {} slot=0x{:02X} target=0x{:08X} eax=0x{:08X} input=0x{:08X} vtable=0x{:08X} guest=0x{:08X} ret=0x{:08X}",
                    name,
                    slot,
                    target,
                    context.Rax as u32,
                    input,
                    vtable,
                    guest_addr,
                    ret_addr
                ));

                if name == "SPIDEY_SELECT_PRESSED_CB1C_RET_TAP" {
                    let threshold_bits = read_guest_u32(r15, 0x0038_7504);
                    let threshold = bits_to_f32(threshold_bits);
                    let (st0, st_top, x87_status, x87_top) = x87_st0_from_context(context);
                    let cur_buttons = read_guest_u16(r15, input.wrapping_add(0x0C));
                    let prev_buttons = read_guest_u16(r15, input.wrapping_add(0x22));
                    let cur_packet = read_guest_u32(r15, input.wrapping_add(0x08));
                    let prev_packet = read_guest_u32(r15, input.wrapping_add(0x1E));
                    let action15_slot = if (cur_buttons & 0x0020) != 0 {
                        1.0
                    } else {
                        0.0
                    } - if (prev_buttons & 0x0020) != 0 {
                        1.0
                    } else {
                        0.0
                    };
                    debug_log(&format!(
                        "[SPIDEY-SELECT-FCOMP] action=0x15 st0={:.6} st_top={:.6} slot_delta={:.6} threshold={:.6} threshold_bits=0x{:08X} x87_sw=0x{:04X} x87_top={} cur(pkt:{} back:{} btn:0x{:04X}) prev(pkt:{} back:{} btn:0x{:04X}) guest=0x{:08X} ret=0x{:08X}",
                        st0,
                        st_top,
                        action15_slot,
                        threshold,
                        threshold_bits,
                        x87_status,
                        x87_top,
                        cur_packet,
                        if (cur_buttons & 0x0020) != 0 { 1 } else { 0 },
                        cur_buttons,
                        prev_packet,
                        if (prev_buttons & 0x0020) != 0 { 1 } else { 0 },
                        prev_buttons,
                        guest_addr,
                        ret_addr
                    ));
                }
            }

            if name == "SPIDEY_SELECT_PRESSED_PRE_EMIT_TAP" {
                let input = context.Rsi as u32;
                let cur_buttons = read_guest_u16(r15, input.wrapping_add(0x0C));
                let cur_a = read_guest_u8(r15, input.wrapping_add(0x0E));
                let scene_name = read_guest_cstr_raw(r15 as *mut u8, 0x004B_C848, 64);
                let stash_name = read_c_string(r15, 0x004B_C948, 64);
                let spidey_menu_assist_enabled =
                    std::env::var_os("RUSTEMU_SPIDEY_DISABLE_MENU_ASSIST").is_none();
                let bonus_menu_force_enabled =
                    std::env::var_os("RUSTEMU_SPIDEY_MENU_CONFIRM_ALIAS").is_some();
                let selector_a_once_enabled =
                    std::env::var_os("RUSTEMU_SPIDEY_SELECTOR_A_ONCE").is_some();
                let selector_confirm_once_enabled =
                    std::env::var_os("RUSTEMU_SPIDEY_SELECTOR_CONFIRM_ONCE").is_some();
                let spidey_script_scene = scene_name.to_ascii_lowercase().starts_with("levels\\")
                    || ((bonus_menu_force_enabled
                        || selector_a_once_enabled
                        || selector_confirm_once_enabled)
                        && scene_name.eq_ignore_ascii_case("bonus\\menu")
                        && stash_name.eq_ignore_ascii_case("M0menu\\menu"));
                let live_confirm = (cur_buttons & (0x0010 | 0x0020)) != 0 || cur_a != 0;
                let force_fallthrough = SELECT_FORCE_FALLTHROUGH_POLLS.load(AO::Relaxed) > 0;
                if force_fallthrough {
                    SELECT_FORCE_FALLTHROUGH_POLLS.fetch_sub(1, AO::Relaxed);
                }
                if spidey_menu_assist_enabled
                    && spidey_script_scene
                    && (live_confirm || force_fallthrough)
                {
                    context.EFlags &= !0x0004;
                    static FORCE_SELECT_EMIT_LOG: AtomicU32 = AtomicU32::new(0);
                    let force_n = FORCE_SELECT_EMIT_LOG.fetch_add(1, AO::Relaxed);
                    if force_n < 16 || force_n.is_power_of_two() {
                        debug_log(&format!(
                            "[SPIDEY-SELECT-FORCE-EMIT] #{} scene='{}' stash='{}' cur_btn=0x{:04X} A={} force_fallthrough={} eax=0x{:08X} guest=0x{:08X} ret=0x{:08X}",
                            force_n,
                            scene_name,
                            stash_name,
                            cur_buttons,
                            cur_a,
                            if force_fallthrough { 1 } else { 0 },
                            context.Rax as u32,
                            guest_addr,
                            ret_addr
                        ));
                    }
                }
                let flags = context.EFlags as u32;
                debug_log(&format!(
                    "[SPIDEY-SELECT-PRE-EMIT] eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} eflags=0x{:08X} cf={} pf={} zf={} sf={} of={} guest=0x{:08X} ret=0x{:08X}",
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rbx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    flags,
                    (flags & 0x0001) != 0,
                    (flags & 0x0004) != 0,
                    (flags & 0x0040) != 0,
                    (flags & 0x0080) != 0,
                    (flags & 0x0800) != 0,
                    guest_addr,
                    ret_addr
                ));
            }
        }

        if name.starts_with("SPIDEY_EVENT_DISPATCH_") {
            static EVENT_DISPATCH_DIAG: AtomicU32 = AtomicU32::new(0);
            let event_n = EVENT_DISPATCH_DIAG.fetch_add(1, AO::Relaxed);
            let obj = if name == "SPIDEY_EVENT_DISPATCH_ENTRY_TAP" {
                context.Rcx as u32
            } else if name == "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP"
                || name == "SPIDEY_EVENT_DISPATCH_POST_LISTENER_TAP"
            {
                context.Rbx as u32
            } else {
                context.Rdi as u32
            };
            let event_id = if name == "SPIDEY_EVENT_DISPATCH_ENTRY_TAP" {
                read_guest_u32(r15, r14.wrapping_add(4))
            } else if name == "SPIDEY_EVENT_DISPATCH_BUILD_RET_TAP" {
                read_guest_u32(r15, r14.wrapping_add(8))
            } else {
                context.Rsi as u32
            };

            let slot_scan_cached_table = read_guest_u32(r15, obj.wrapping_add(0x08));
            let slot_scan_table = if name == "SPIDEY_EVENT_DISPATCH_BUILD_RET_TAP" {
                context.Rax as u32
            } else {
                slot_scan_cached_table
            };
            let slot_scan_count = if valid_guest_ptr(slot_scan_table) {
                read_guest_u32(r15, slot_scan_table)
            } else {
                0
            };
            let slot_scan_slots = if valid_guest_ptr(slot_scan_table) {
                read_guest_u32(r15, slot_scan_table.wrapping_add(0x04))
            } else {
                0
            };
            if valid_guest_ptr(slot_scan_table)
                && valid_guest_ptr(slot_scan_slots)
                && slot_scan_count != 0
            {
                SPIDEY_EVENT_TABLE_ADDR.store(slot_scan_table, AO::Relaxed);
                SPIDEY_EVENT_TABLE_COUNT.store(slot_scan_count, AO::Relaxed);
                SPIDEY_EVENT_TABLE_SLOTS.store(slot_scan_slots, AO::Relaxed);
                let is_main_menu_event_table = obj == 0x1024_9C64 && slot_scan_count == 28;
                let watch_slot = std::env::var("RUSTEMU_SPIDEY_EVENT_SLOT_WATCH")
                    .ok()
                    .and_then(|v| {
                        let trimmed = v.trim();
                        if let Some(hex) = trimmed
                            .strip_prefix("0x")
                            .or_else(|| trimmed.strip_prefix("0X"))
                        {
                            u32::from_str_radix(hex, 16).ok()
                        } else {
                            trimmed.parse::<u32>().ok()
                        }
                    })
                    .unwrap_or(1);
                let watch_enabled = std::env::var_os("RUSTEMU_SPIDEY_SLOT1_WATCH").is_some()
                    || std::env::var_os("RUSTEMU_SPIDEY_EVENT_SLOT_WATCH").is_some();
                let watch_slot_addr = slot_scan_slots.wrapping_add(watch_slot.wrapping_mul(4));
                if watch_enabled
                    && is_main_menu_event_table
                    && watch_slot < slot_scan_count.min(0x400)
                    && valid_guest_ptr(watch_slot_addr)
                    && SPIDEY_EVENT_SLOT1_WATCH_GUEST.swap(watch_slot_addr, AO::AcqRel)
                        != watch_slot_addr
                {
                    context.Dr0 = r15.wrapping_add(watch_slot_addr as u64);
                    context.Dr1 = 0;
                    context.Dr2 = 0;
                    context.Dr3 = 0;
                    context.Dr7 =
                        (1 << 0) | (1 << 1) | (1 << 8) | (1 << 9) | (0b01 << 16) | (0b11 << 18);
                    debug_log(&format!(
                        "[SPIDEY-SLOT-WATCH-ARM] menu_table=1 obj=0x{:08X} table=0x{:08X} count={} slots=0x{:08X} slot=0x{:02X} slot_addr=0x{:08X} host=0x{:016X} dispatch_event=0x{:X} tap={} Dr7=0x{:X}",
                        obj,
                        slot_scan_table,
                        slot_scan_count,
                        slot_scan_slots,
                        watch_slot,
                        watch_slot_addr,
                        context.Dr0,
                        event_id,
                        name,
                        context.Dr7
                    ));
                }

                let slot_scan_frame = read_guest_u32(r15, 0x004B_C630);
                let slot_scan_frame_scene = if valid_guest_ptr(slot_scan_frame) {
                    read_guest_u32(r15, slot_scan_frame.wrapping_add(0x18))
                } else {
                    0
                };
                let slot_scan_scene_18c = if valid_guest_ptr(slot_scan_frame_scene) {
                    read_guest_u8(r15, slot_scan_frame_scene.wrapping_add(0x18C))
                } else {
                    0
                };
                let slot_scan_scene_a0 = if valid_guest_ptr(slot_scan_frame_scene) {
                    read_guest_u32(r15, slot_scan_frame_scene.wrapping_add(0xA0))
                } else {
                    0
                };
                let slot_scan_state_idx = if valid_guest_ptr(slot_scan_scene_a0) {
                    read_guest_u32(r15, slot_scan_scene_a0.wrapping_sub(0x10))
                } else {
                    0
                };
                let slot_scan_state_arr = if valid_guest_ptr(slot_scan_scene_a0) {
                    read_guest_u32(r15, slot_scan_scene_a0.wrapping_sub(0x14))
                } else {
                    0
                };
                let slot_scan_state_val =
                    if valid_guest_ptr(slot_scan_state_arr) && slot_scan_state_idx < 0x1000 {
                        read_guest_u32(
                            r15,
                            slot_scan_state_arr.wrapping_add(slot_scan_state_idx.wrapping_mul(4)),
                        )
                    } else {
                        0
                    };
                let slot_scan_pending = read_guest_u32(r15, 0x003D_CFA0);
                for key in [
                    0u32, 1, 6, 7, 0x0A, 0x0C, 0x0E, 0x10, 0x12, 0x14, 0x16, 0x1A,
                ] {
                    if key >= slot_scan_count.min(0x400) {
                        continue;
                    }
                    let Some(last) = spidey_event_slot_last(key) else {
                        continue;
                    };
                    let slot_addr = slot_scan_slots.wrapping_add(key.wrapping_mul(4));
                    let slot = read_guest_u32(r15, slot_addr);
                    let old = last.swap(slot, std::sync::atomic::Ordering::Relaxed);
                    if old != slot {
                        let listener_vt = if valid_guest_ptr(slot) {
                            read_guest_u32(r15, slot)
                        } else {
                            0
                        };
                        let listener_0c = if valid_guest_ptr(slot) {
                            read_guest_u32(r15, slot.wrapping_add(0x0C))
                        } else {
                            0
                        };
                        let listener_10 = if valid_guest_ptr(slot) {
                            read_guest_u32(r15, slot.wrapping_add(0x10))
                        } else {
                            0
                        };
                        let listener_20 = if valid_guest_ptr(slot) {
                            read_guest_u32(r15, slot.wrapping_add(0x20))
                        } else {
                            0
                        };
                        let listener_0c_string = if valid_guest_ptr(listener_0c) {
                            read_c_string(r15, listener_0c, 64)
                        } else {
                            String::new()
                        };
                        let listener_20_string = if valid_guest_ptr(listener_20) {
                            read_c_string(r15, listener_20, 64)
                        } else {
                            String::new()
                        };
                        let scene_name = read_c_string(r15, 0x004B_C848, 64);
                        let stash_name = read_c_string(r15, 0x004B_C948, 64);
                        debug_log(&format!(
                            "[SPIDEY-EVENT-SLOT-CHANGE] key=0x{:02X} old=0x{:08X} new=0x{:08X} slot_addr=0x{:08X} lvt=0x{:08X} l+0C/10/20=0x{:08X}/0x{:08X}/0x{:08X} l0c_str='{}' l20_str='{}' table=0x{:08X} count={} slots=0x{:08X} dispatch_event=0x{:X} tap={} obj=0x{:08X} frame_scene=0x{:08X} scene18c={} state_val={} pending=0x{:08X} scene='{}' stash='{}'",
                            key,
                            old,
                            slot,
                            slot_addr,
                            listener_vt,
                            listener_0c,
                            listener_10,
                            listener_20,
                            listener_0c_string,
                            listener_20_string,
                            slot_scan_table,
                            slot_scan_count,
                            slot_scan_slots,
                            event_id,
                            name,
                            obj,
                            slot_scan_frame_scene,
                            slot_scan_scene_18c,
                            slot_scan_state_val,
                            slot_scan_pending,
                            scene_name,
                            stash_name
                        ));
                    }
                }
            }

            if event_id == 1 || event_id == 0x1A || event_n < 64 || event_n.is_power_of_two() {
                let obj_vt = read_guest_u32(r15, obj);
                let obj_cb10 = read_guest_u32(r15, obj_vt.wrapping_add(0x10));
                let obj_cb14 = read_guest_u32(r15, obj_vt.wrapping_add(0x14));
                let obj_cb1c = read_guest_u32(r15, obj_vt.wrapping_add(0x1C));
                let obj_flags = read_guest_u32(r15, obj.wrapping_add(0x04));
                let cached_table = read_guest_u32(r15, obj.wrapping_add(0x08));
                let table = if name == "SPIDEY_EVENT_DISPATCH_BUILD_RET_TAP" {
                    context.Rax as u32
                } else {
                    cached_table
                };
                let table_count = if valid_guest_ptr(table) {
                    read_guest_u32(r15, table)
                } else {
                    0
                };
                let table_slots = if valid_guest_ptr(table) {
                    read_guest_u32(r15, table.wrapping_add(0x04))
                } else {
                    0
                };
                if valid_guest_ptr(table) && valid_guest_ptr(table_slots) && table_count != 0 {
                    SPIDEY_EVENT_TABLE_ADDR.store(table, AO::Relaxed);
                    SPIDEY_EVENT_TABLE_COUNT.store(table_count, AO::Relaxed);
                    SPIDEY_EVENT_TABLE_SLOTS.store(table_slots, AO::Relaxed);
                }
                let mut event_slot = if valid_guest_ptr(table_slots) && event_id < 0x400 {
                    read_guest_u32(r15, table_slots.wrapping_add(event_id.wrapping_mul(4)))
                } else {
                    0
                };
                if event_id == 0x1A
                    && event_slot == 0
                    && valid_guest_ptr(table_slots)
                    && spidey_event_patches_enabled()
                {
                    if let Ok(alias_raw) = std::env::var("RUSTEMU_SPIDEY_EVENT_1A_ALIAS_SLOT") {
                        let alias_scene = read_c_string(r15, 0x004B_C848, 64);
                        let alias_stash = read_c_string(r15, 0x004B_C948, 64);
                        let alias_allowed = alias_scene.eq_ignore_ascii_case("levels\\origin_z")
                            || alias_stash.eq_ignore_ascii_case("M1origin\\origin_z")
                            || crate::xbox::emulator::spidey_peterstu_read_ready_seq() != 0
                            || std::env::var_os("RUSTEMU_SPIDEY_EVENT_1A_ALIAS_EARLY").is_some();
                        if alias_allowed {
                            let alias_trimmed = alias_raw.trim();
                            let (alias_slot, alias_desc) =
                                if alias_trimmed.eq_ignore_ascii_case("global") {
                                    (
                                        read_guest_u32(r15, 0x003F_87A8),
                                        "global_event1a".to_string(),
                                    )
                                } else if let Some(hex) = alias_trimmed
                                    .strip_prefix("0x")
                                    .or_else(|| alias_trimmed.strip_prefix("0X"))
                                {
                                    let alias_idx = u32::from_str_radix(hex, 16).unwrap_or(0x15);
                                    if alias_idx < 0x400 {
                                        (
                                            read_guest_u32(
                                                r15,
                                                table_slots.wrapping_add(alias_idx.wrapping_mul(4)),
                                            ),
                                            format!("slot[0x{:X}]", alias_idx),
                                        )
                                    } else {
                                        (0, format!("slot[0x{:X}]-out-of-range", alias_idx))
                                    }
                                } else {
                                    let alias_idx = alias_trimmed.parse::<u32>().unwrap_or(0x15);
                                    if alias_idx < 0x400 {
                                        (
                                            read_guest_u32(
                                                r15,
                                                table_slots.wrapping_add(alias_idx.wrapping_mul(4)),
                                            ),
                                            format!("slot[{}]", alias_idx),
                                        )
                                    } else {
                                        (0, format!("slot[{}]-out-of-range", alias_idx))
                                    }
                                };

                            if alias_slot != 0 {
                                write_guest_u32(
                                    r15,
                                    table_slots.wrapping_add(event_id.wrapping_mul(4)),
                                    alias_slot,
                                );
                                event_slot = alias_slot;
                                static EVENT_1A_ALIAS_LOG: AtomicU32 = AtomicU32::new(0);
                                let alias_n = EVENT_1A_ALIAS_LOG.fetch_add(1, AO::Relaxed);
                                if alias_n < 16 || alias_n.is_power_of_two() {
                                    debug_log(&format!(
                                        "[SPIDEY-EVENT-1A-ALIAS] #{} {} scene='{}' stash='{}' table=0x{:08X} slots=0x{:08X} alias={} alias_slot=0x{:08X} slot1a_addr=0x{:08X}",
                                        alias_n,
                                        name,
                                        alias_scene,
                                        alias_stash,
                                        table,
                                        table_slots,
                                        alias_desc,
                                        alias_slot,
                                        table_slots.wrapping_add(event_id.wrapping_mul(4))
                                    ));
                                }
                            }
                        } else {
                            static EVENT_1A_ALIAS_SKIP_LOG: AtomicU32 = AtomicU32::new(0);
                            let skip_n = EVENT_1A_ALIAS_SKIP_LOG.fetch_add(1, AO::Relaxed);
                            if skip_n < 8 || skip_n.is_power_of_two() {
                                debug_log(&format!(
                                    "[SPIDEY-EVENT-1A-ALIAS-SKIP] #{} {} scene='{}' stash='{}' table=0x{:08X} slots=0x{:08X}",
                                    skip_n, name, alias_scene, alias_stash, table, table_slots
                                ));
                            }
                        }
                    }
                }
                let obj_bits_lo = read_guest_u32(r15, obj.wrapping_add(0x0C));
                let obj_bits_hi = read_guest_u32(r15, obj.wrapping_add(0x10));
                let pending_word = read_guest_u32(r15, 0x003D_CFA0);
                let frame = read_guest_u32(r15, 0x004B_C630);
                let frame_scene = if valid_guest_ptr(frame) {
                    read_guest_u32(r15, frame.wrapping_add(0x18))
                } else {
                    0
                };
                let global_pending = read_guest_u32(r15, 0x003D_CFA0);
                let global_pend_begin = read_guest_u32(r15, 0x003D_CEC0);
                let global_pend_table = read_guest_u32(r15, 0x003D_CEC4);
                let global_pend_end = read_guest_u32(r15, 0x003D_CEC8);
                let scene_18c = if valid_guest_ptr(frame_scene) {
                    read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
                } else {
                    0
                };
                let scene_a0 = if valid_guest_ptr(frame_scene) {
                    read_guest_u32(r15, frame_scene.wrapping_add(0xA0))
                } else {
                    0
                };
                let state_idx = if valid_guest_ptr(scene_a0) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                } else {
                    0
                };
                let state_arr = if valid_guest_ptr(scene_a0) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                } else {
                    0
                };
                let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                    read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                } else {
                    0
                };
                if valid_guest_ptr(table_slots) && table_count != 0 {
                    for key in [
                        0u32, 1, 6, 7, 0x0A, 0x0C, 0x0E, 0x10, 0x12, 0x14, 0x16, 0x1A,
                    ] {
                        if key >= table_count.min(0x400) {
                            continue;
                        }
                        let Some(last) = spidey_event_slot_last(key) else {
                            continue;
                        };
                        let slot_addr = table_slots.wrapping_add(key.wrapping_mul(4));
                        let slot = read_guest_u32(r15, slot_addr);
                        let old = last.swap(slot, std::sync::atomic::Ordering::Relaxed);
                        if old != slot {
                            let listener_vt = if valid_guest_ptr(slot) {
                                read_guest_u32(r15, slot)
                            } else {
                                0
                            };
                            let listener_0c = if valid_guest_ptr(slot) {
                                read_guest_u32(r15, slot.wrapping_add(0x0C))
                            } else {
                                0
                            };
                            let listener_10 = if valid_guest_ptr(slot) {
                                read_guest_u32(r15, slot.wrapping_add(0x10))
                            } else {
                                0
                            };
                            let listener_20 = if valid_guest_ptr(slot) {
                                read_guest_u32(r15, slot.wrapping_add(0x20))
                            } else {
                                0
                            };
                            let listener_0c_string = if valid_guest_ptr(listener_0c) {
                                read_c_string(r15, listener_0c, 64)
                            } else {
                                String::new()
                            };
                            let listener_20_string = if valid_guest_ptr(listener_20) {
                                read_c_string(r15, listener_20, 64)
                            } else {
                                String::new()
                            };
                            let scene_name = read_c_string(r15, 0x004B_C848, 64);
                            let stash_name = read_c_string(r15, 0x004B_C948, 64);
                            debug_log(&format!(
                                "[SPIDEY-EVENT-SLOT-CHANGE] key=0x{:02X} old=0x{:08X} new=0x{:08X} slot_addr=0x{:08X} lvt=0x{:08X} l+0C/10/20=0x{:08X}/0x{:08X}/0x{:08X} l0c_str='{}' l20_str='{}' table=0x{:08X} count={} slots=0x{:08X} dispatch_event=0x{:X} tap={} obj=0x{:08X} frame_scene=0x{:08X} scene18c={} state_val={} pending=0x{:08X} scene='{}' stash='{}'",
                                key,
                                old,
                                slot,
                                slot_addr,
                                listener_vt,
                                listener_0c,
                                listener_10,
                                listener_20,
                                listener_0c_string,
                                listener_20_string,
                                table,
                                table_count,
                                table_slots,
                                event_id,
                                name,
                                obj,
                                frame_scene,
                                scene_18c,
                                state_val,
                                pending_word,
                                scene_name,
                                stash_name
                            ));
                        }
                    }
                }
                if spidey_event1_replay_enabled()
                    && event_id == 1
                    && event_slot == 0
                    && state_val == 3
                    && valid_guest_ptr(table_slots)
                {
                    let scene_name_for_event1 = read_c_string(r15, 0x004B_C848, 64);
                    if scene_name_for_event1.eq_ignore_ascii_case("bonus\\menu") {
                        let alias_idx = std::env::var("RUSTEMU_SPIDEY_EVENT1_ALIAS_SLOT")
                            .ok()
                            .and_then(|v| {
                                let trimmed = v.trim();
                                if let Some(hex) = trimmed
                                    .strip_prefix("0x")
                                    .or_else(|| trimmed.strip_prefix("0X"))
                                {
                                    u32::from_str_radix(hex, 16).ok()
                                } else {
                                    trimmed.parse::<u32>().ok()
                                }
                            })
                            .unwrap_or(0);
                        if alias_idx < table_count.min(64) {
                            let alias_slot = read_guest_u32(
                                r15,
                                table_slots.wrapping_add(alias_idx.wrapping_mul(4)),
                            );
                            if valid_guest_ptr(alias_slot) {
                                write_guest_u32(
                                    r15,
                                    table_slots.wrapping_add(event_id.wrapping_mul(4)),
                                    alias_slot,
                                );
                                event_slot = alias_slot;
                                let alias_n = SPIDEY_EVENT1_REPLAY_LOG.fetch_add(1, AO::Relaxed);
                                debug_log(&format!(
                                    "[SPIDEY-EVENT1-ALIAS] #{} event=1 alias_slot={} listener=0x{:08X} table=0x{:08X} slots=0x{:08X} slot1_addr=0x{:08X} frame_scene=0x{:08X} state_val={} scene18c={} scene='{}'",
                                    alias_n,
                                    alias_idx,
                                    alias_slot,
                                    table,
                                    table_slots,
                                    table_slots.wrapping_add(event_id.wrapping_mul(4)),
                                    frame_scene,
                                    state_val,
                                    scene_18c,
                                    scene_name_for_event1
                                ));
                            }
                        }
                    }
                }
                if spidey_event1_replay_enabled()
                    && name == "SPIDEY_EVENT_DISPATCH_LOOKUP_TAP"
                    && event_id == 1
                    && event_slot == 0
                    && state_val == 3
                    && scene_18c == 0
                {
                    let scene_name_for_event1 = read_c_string(r15, 0x004B_C848, 64);
                    if scene_name_for_event1.eq_ignore_ascii_case("bonus\\menu") {
                        SPIDEY_EVENT1_DEFER_PENDING.store(1, AO::Relaxed);
                        SPIDEY_EVENT1_DEFER_OBJ.store(obj, AO::Relaxed);
                        SPIDEY_EVENT1_DEFER_SCENE.store(frame_scene, AO::Relaxed);
                        let defer_n = SPIDEY_EVENT1_DEFER_LOG.fetch_add(1, AO::Relaxed);
                        if defer_n < 8 || defer_n.is_power_of_two() {
                            debug_log(&format!(
                                "[SPIDEY-EVENT1-DEFER] #{} event lookup had no listener; obj=0x{:08X} table=0x{:08X} slots=0x{:08X} slot1_addr=0x{:08X} pending=0x{:08X} frame_scene=0x{:08X} scene='{}'",
                                defer_n,
                                obj,
                                table,
                                table_slots,
                                if valid_guest_ptr(table_slots) {
                                    table_slots.wrapping_add(4)
                                } else {
                                    0
                                },
                                pending_word,
                                frame_scene,
                                scene_name_for_event1
                            ));
                        }
                    }
                }
                let listener = if name == "SPIDEY_EVENT_DISPATCH_LISTENER_CALL_TAP" {
                    context.Rcx as u32
                } else {
                    event_slot
                };
                let listener_vt = if valid_guest_ptr(listener) {
                    read_guest_u32(r15, listener)
                } else {
                    0
                };
                let listener_04 = if valid_guest_ptr(listener) {
                    read_guest_u32(r15, listener.wrapping_add(0x04))
                } else {
                    0
                };
                let listener_08 = if valid_guest_ptr(listener) {
                    read_guest_u32(r15, listener.wrapping_add(0x08))
                } else {
                    0
                };
                let listener_0c = if valid_guest_ptr(listener) {
                    read_guest_u32(r15, listener.wrapping_add(0x0C))
                } else {
                    0
                };
                let listener_10 = if valid_guest_ptr(listener) {
                    read_guest_u32(r15, listener.wrapping_add(0x10))
                } else {
                    0
                };
                let (
                    sig_list,
                    sig_sentinel,
                    sig_first,
                    sig_first_prev,
                    sig_second,
                    sig_first_cb,
                    sig_first_cb_vt,
                    sig_first_cb_fn,
                    sig_cb_count,
                ) = signal_active_list_info(r15, listener);
                debug_log(&format!(
                    "[SPIDEY-EVENT-DISPATCH #{}] {} event=0x{:X} obj=0x{:08X} vt=0x{:08X} cb10=0x{:08X} cb14=0x{:08X} cb1c=0x{:08X} flags=0x{:08X} cached_table=0x{:08X} table=0x{:08X} table_count={} slots=0x{:08X} event_slot=0x{:08X} listener=0x{:08X} lvt=0x{:08X} l+04/08/0c/10=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} sig_list=0x{:08X} sentinel=0x{:08X} first=0x{:08X} first_prev=0x{:08X} second=0x{:08X} first_cb=0x{:08X} cb_vt=0x{:08X} cb_fn=0x{:08X} cb_sec={} cb_count={} bits0c/10=0x{:08X}/0x{:08X} pending=0x{:08X} frame_scene=0x{:08X} scene18c={} state_idx={} state_val={} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    event_n,
                    name,
                    event_id,
                    obj,
                    obj_vt,
                    obj_cb10,
                    obj_cb14,
                    obj_cb1c,
                    obj_flags,
                    cached_table,
                    table,
                    table_count,
                    table_slots,
                    event_slot,
                    listener,
                    listener_vt,
                    listener_04,
                    listener_08,
                    listener_0c,
                    listener_10,
                    sig_list,
                    sig_sentinel,
                    sig_first,
                    sig_first_prev,
                    sig_second,
                    sig_first_cb,
                    sig_first_cb_vt,
                    sig_first_cb_fn,
                    spidey_section(sig_first_cb_fn),
                    sig_cb_count,
                    obj_bits_lo,
                    obj_bits_hi,
                    pending_word,
                    frame_scene,
                    scene_18c,
                    state_idx,
                    state_val,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));

                if event_id == 1 || event_id == 0x1A {
                    let mut near_slots = String::new();
                    let near_start = event_id.saturating_sub(6);
                    let near_end = (event_id + 6).min(0x3F);
                    for idx in near_start..=near_end {
                        let slot = if valid_guest_ptr(table_slots) {
                            read_guest_u32(r15, table_slots.wrapping_add(idx.wrapping_mul(4)))
                        } else {
                            0
                        };
                        if !near_slots.is_empty() {
                            near_slots.push(' ');
                        }
                        near_slots.push_str(&format!("{:02X}:0x{:08X}", idx, slot));
                    }

                    let mut active_slots = String::new();
                    let scan_count = table_count.min(64);
                    for idx in 0..scan_count {
                        let slot = if valid_guest_ptr(table_slots) {
                            read_guest_u32(r15, table_slots.wrapping_add(idx.wrapping_mul(4)))
                        } else {
                            0
                        };
                        if slot != 0 {
                            if !active_slots.is_empty() {
                                active_slots.push(' ');
                            }
                            active_slots.push_str(&format!("{:02X}:0x{:08X}", idx, slot));
                        }
                    }
                    if active_slots.is_empty() {
                        active_slots.push_str("<none>");
                    }

                    let global_pend_begin = read_guest_u32(r15, 0x003D_CEC0);
                    let global_pend_table = read_guest_u32(r15, 0x003D_CEC4);
                    let global_pend_end = read_guest_u32(r15, 0x003D_CEC8);
                    let global_event_1a = read_guest_u32(r15, 0x003F_87A8);
                    let global_event_vt = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a)
                    } else {
                        0
                    };
                    let global_event_04 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x04))
                    } else {
                        0
                    };
                    let global_event_08 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x08))
                    } else {
                        0
                    };
                    let global_event_0c = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x0C))
                    } else {
                        0
                    };
                    let global_event_10 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x10))
                    } else {
                        0
                    };
                    let global_event_14 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x14))
                    } else {
                        0
                    };
                    let global_event_18 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x18))
                    } else {
                        0
                    };
                    let global_event_1c = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x1C))
                    } else {
                        0
                    };
                    let global_event_20 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x20))
                    } else {
                        0
                    };
                    let global_event_24 = if valid_guest_ptr(global_event_1a) {
                        read_guest_u32(r15, global_event_1a.wrapping_add(0x24))
                    } else {
                        0
                    };
                    let (
                        global_sig_list,
                        global_sig_sentinel,
                        global_sig_first,
                        global_sig_first_prev,
                        global_sig_second,
                        global_sig_first_cb,
                        global_sig_first_cb_vt,
                        global_sig_first_cb_fn,
                        global_sig_cb_count,
                    ) = signal_active_list_info(r15, global_event_1a);
                    let pending_slot = if valid_guest_ptr(global_pend_table) {
                        read_guest_u32(
                            r15,
                            global_pend_table.wrapping_add(event_id.wrapping_mul(4)),
                        )
                    } else {
                        0
                    };
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);

                    debug_log(&format!(
                        "[SPIDEY-EVENT-TABLE #{}] {} event=0x{:X} obj=0x{:08X} table=0x{:08X} count={} slots=0x{:08X} slot_addr=0x{:08X} slot=0x{:08X} near=[{}] active=[{}] cached_table=0x{:08X} obj_bits=0x{:08X}/0x{:08X} pending_word=0x{:08X} pend_begin/table/end=0x{:08X}/0x{:08X}/0x{:08X} pend_slot=0x{:08X} global_event1a=0x{:08X} gvt=0x{:08X} g+04/08/0c/10/14/18/1c/20/24=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} gsig_list=0x{:08X} gsentinel=0x{:08X} gfirst=0x{:08X} gfirst_prev=0x{:08X} gsecond=0x{:08X} gfirst_cb=0x{:08X} gcb_vt=0x{:08X} gcb_fn=0x{:08X} gcb_sec={} gcb_count={} frame_scene=0x{:08X} scene18c={} scene='{}' stash='{}'",
                        event_n,
                        name,
                        event_id,
                        obj,
                        table,
                        table_count,
                        table_slots,
                        if valid_guest_ptr(table_slots) {
                            table_slots.wrapping_add(event_id.wrapping_mul(4))
                        } else {
                            0
                        },
                        event_slot,
                        near_slots,
                        active_slots,
                        cached_table,
                        obj_bits_lo,
                        obj_bits_hi,
                        pending_word,
                        global_pend_begin,
                        global_pend_table,
                        global_pend_end,
                        pending_slot,
                        global_event_1a,
                        global_event_vt,
                        global_event_04,
                        global_event_08,
                        global_event_0c,
                        global_event_10,
                        global_event_14,
                        global_event_18,
                        global_event_1c,
                        global_event_20,
                        global_event_24,
                        global_sig_list,
                        global_sig_sentinel,
                        global_sig_first,
                        global_sig_first_prev,
                        global_sig_second,
                        global_sig_first_cb,
                        global_sig_first_cb_vt,
                        global_sig_first_cb_fn,
                        spidey_section(global_sig_first_cb_fn),
                        global_sig_cb_count,
                        frame_scene,
                        scene_18c,
                        scene_name,
                        stash_name
                    ));
                }

                if name == "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP"
                    || name == "SPIDEY_EVENT_DISPATCH_POST_LISTENER_TAP"
                {
                    let scene_name = read_c_string(r15, 0x004B_C848, 64);
                    let stash_name = read_c_string(r15, 0x004B_C948, 64);
                    let mut listener_words = String::new();
                    if valid_guest_ptr(listener) {
                        for off in (0x00u32..=0x40u32).step_by(4) {
                            if !listener_words.is_empty() {
                                listener_words.push(' ');
                            }
                            listener_words.push_str(&format!(
                                "+{:02X}=0x{:08X}",
                                off,
                                read_guest_u32(r15, listener.wrapping_add(off))
                            ));
                        }
                    } else {
                        listener_words.push_str("<invalid>");
                    }

                    let mut vtable_words = String::new();
                    if valid_guest_ptr(listener_vt) {
                        for off in (0x00u32..=0x28u32).step_by(4) {
                            if !vtable_words.is_empty() {
                                vtable_words.push(' ');
                            }
                            vtable_words.push_str(&format!(
                                "+{:02X}=0x{:08X}({})",
                                off,
                                read_guest_u32(r15, listener_vt.wrapping_add(off)),
                                spidey_section(read_guest_u32(r15, listener_vt.wrapping_add(off)))
                            ));
                        }
                    } else {
                        vtable_words.push_str("<invalid>");
                    }

                    let listener_0c_string = if valid_guest_ptr(listener_0c) {
                        read_c_string(r15, listener_0c, 64)
                    } else {
                        String::new()
                    };
                    let listener_20 = if valid_guest_ptr(listener) {
                        read_guest_u32(r15, listener.wrapping_add(0x20))
                    } else {
                        0
                    };
                    let listener_20_string = if valid_guest_ptr(listener_20) {
                        read_c_string(r15, listener_20, 64)
                    } else {
                        String::new()
                    };
                    let mut forced_accept = false;
                    if name == "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP"
                        && event_id == 0x1A
                        && listener_vt == 0x0038_49D0
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        && spidey_event_patches_enabled()
                        && std::env::var_os("RUSTEMU_SPIDEY_EVENT_1A_FORCE_LISTENER_RET").is_some()
                    {
                        context.Rax = (context.Rax & !0xFF) | 1;
                        forced_accept = true;
                    }
                    let mut forced_queue = false;
                    let mut forced_queue_detail = String::new();
                    let mut forced_pending_ring = false;
                    let mut forced_pending_ring_detail = String::new();
                    if name == "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP"
                        && event_id == 0x1A
                        && listener_vt == 0x0038_49D0
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        && spidey_event_patches_enabled()
                        && std::env::var_os("RUSTEMU_SPIDEY_EVENT_1A_FORCE_LISTENER_QUEUE")
                            .is_some()
                    {
                        context.Rax = (context.Rax & !0xFF) | 1;
                        forced_accept = true;

                        let upd1 = read_guest_u32(r15, 0x003F_87A8);
                        let old_l1e = if valid_guest_ptr(listener) {
                            read_guest_u8(r15, listener.wrapping_add(0x1E))
                        } else {
                            0
                        };
                        let old_l08 = if valid_guest_ptr(listener) {
                            read_guest_u32(r15, listener.wrapping_add(0x08))
                        } else {
                            0
                        };
                        let cursor = if valid_guest_ptr(upd1) {
                            read_guest_u32(r15, upd1.wrapping_add(0x14))
                        } else {
                            0
                        };
                        let limit = if valid_guest_ptr(upd1) {
                            read_guest_u32(r15, upd1.wrapping_add(0x18))
                        } else {
                            0
                        };
                        let mut wrote_slot = 0u32;
                        let mut bumped_to = cursor;

                        if valid_guest_ptr(listener) {
                            // 0x46200 sets these completion bits when the callback receives
                            // listener+20 and listener+24 payloads. The forced event path
                            // only passes raw 0x1A, so emulate the side effect explicitly.
                            write_guest_u8(r15, listener.wrapping_add(0x1E), old_l1e | 0x03);

                            if (old_l08 & 0x02) == 0 {
                                write_guest_u32(r15, listener.wrapping_add(0x08), old_l08 | 0x02);
                            }

                            if valid_guest_ptr(upd1) {
                                if valid_guest_ptr(cursor) {
                                    write_guest_u32(r15, cursor, listener);
                                    wrote_slot = cursor;
                                }
                                bumped_to = cursor.wrapping_add(4);
                                write_guest_u32(r15, upd1.wrapping_add(0x14), bumped_to);
                                if cursor == limit {
                                    // The native callback calls a grow/insert helper when
                                    // the vector cursor reaches capacity. For this gated
                                    // experiment, reserve one synthetic slot in-place so
                                    // the consumer sees the appended listener.
                                    write_guest_u32(r15, upd1.wrapping_add(0x18), bumped_to);
                                }
                                forced_queue = true;
                            }
                        }

                        forced_queue_detail = format!(
                            " upd1=0x{:08X} cursor=0x{:08X} limit=0x{:08X} wrote_slot=0x{:08X} bumped_to=0x{:08X} l1e:{:02X}->{:02X} l08:0x{:08X}->0x{:08X}",
                            upd1,
                            cursor,
                            limit,
                            wrote_slot,
                            bumped_to,
                            old_l1e,
                            if valid_guest_ptr(listener) {
                                read_guest_u8(r15, listener.wrapping_add(0x1E))
                            } else {
                                0
                            },
                            old_l08,
                            if valid_guest_ptr(listener) {
                                read_guest_u32(r15, listener.wrapping_add(0x08))
                            } else {
                                0
                            }
                        );
                    }
                    if name == "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP"
                        && event_id == 0x1A
                        && (listener_vt == 0x0038_49D0 || listener == 0)
                        && scene_name.eq_ignore_ascii_case("levels\\origin_z")
                        && spidey_event_patches_enabled()
                        && std::env::var_os("RUSTEMU_SPIDEY_EVENT_1A_FORCE_PENDING_RING").is_some()
                    {
                        context.Rax = (context.Rax & !0xFF) | 1;
                        forced_accept = true;

                        // 0x73560/0x73170 reveal this is a 28-entry keypress table:
                        // { key, pending_flag } at 0x003DCEC0 + index * 8. It is not a
                        // FIFO, despite earlier log labels calling it pend_begin/table/end.
                        // The table drain is separately gated by byte 0x003DCFA0; setting
                        // only a slot flag leaves pending_word stuck at 0x100 and the
                        // caller returns before the 0x73170 drain loop.
                        let old_master_pending = read_guest_u8(r15, 0x003D_CFA0);
                        write_guest_u8(r15, 0x003D_CFA0, 1);
                        let global_event_1a = read_guest_u32(r15, 0x003F_87A8);
                        let fallback_key = if valid_guest_ptr(event_slot) {
                            event_slot
                        } else if valid_guest_ptr(listener) {
                            listener
                        } else if valid_guest_ptr(global_event_1a) {
                            global_event_1a
                        } else {
                            event_id
                        };
                        let candidates = [event_id, event_slot, listener, global_event_1a];
                        let mut hit_mask = 0u32;
                        let mut write_count = 0u32;
                        let mut natural_detail = String::new();
                        for slot in 0u32..0x1C {
                            let key_addr = 0x003D_CEC0u32.wrapping_add(slot.wrapping_mul(8));
                            let flag_addr = key_addr.wrapping_add(4);
                            let old_key = read_guest_u32(r15, key_addr);
                            let old_flag = read_guest_u32(r15, flag_addr);
                            let natural = slot == event_id;
                            let matched = candidates
                                .iter()
                                .any(|candidate| *candidate != 0 && *candidate == old_key);
                            if natural || matched {
                                if old_key == 0 {
                                    write_guest_u32(r15, key_addr, fallback_key);
                                }
                                write_guest_u32(r15, flag_addr, 1);
                                write_count = write_count.wrapping_add(1);
                                hit_mask |= 1u32.wrapping_shl(slot.min(31));
                                if natural {
                                    natural_detail = format!(
                                        " natural_slot={} key_addr=0x{:08X} flag_addr=0x{:08X} key:0x{:08X}->0x{:08X} flag:0x{:08X}->0x{:08X}",
                                        slot,
                                        key_addr,
                                        flag_addr,
                                        old_key,
                                        read_guest_u32(r15, key_addr),
                                        old_flag,
                                        read_guest_u32(r15, flag_addr)
                                    );
                                }
                            }
                        }
                        forced_pending_ring = true;
                        forced_pending_ring_detail = format!(
                            " keypress_table=1 master_pending:{}->1 fallback_key=0x{:08X} writes={} hit_mask=0x{:08X}{}",
                            old_master_pending,
                            fallback_key,
                            write_count,
                            hit_mask,
                            natural_detail
                        );
                    }
                    debug_log(&format!(
                        "[SPIDEY-EVENT-LISTENER-RESULT #{}] {} event=0x{:X} obj=0x{:08X} listener=0x{:08X} lvt=0x{:08X} ret_eax=0x{:08X} forced_accept={} forced_queue={}{} forced_pending_ring={}{} pending_word=0x{:08X} pend_begin/table/end=0x{:08X}/0x{:08X}/0x{:08X} frame_scene=0x{:08X} scene18c={} state_idx={} state_val={} flags=0x{:08X} bits0c/10=0x{:08X}/0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X} listener_words=[{}] vtable=[{}] l0c_str='{}' l20_str='{}' scene='{}' stash='{}'",
                        event_n,
                        name,
                        event_id,
                        obj,
                        listener,
                        listener_vt,
                        context.Rax as u32,
                        if forced_accept { 1 } else { 0 },
                        if forced_queue { 1 } else { 0 },
                        forced_queue_detail,
                        if forced_pending_ring { 1 } else { 0 },
                        forced_pending_ring_detail,
                        global_pending,
                        global_pend_begin,
                        global_pend_table,
                        global_pend_end,
                        frame_scene,
                        scene_18c,
                        state_idx,
                        state_val,
                        obj_flags,
                        obj_bits_lo,
                        obj_bits_hi,
                        context.Rcx as u32,
                        context.Rdx as u32,
                        context.Rsi as u32,
                        context.Rdi as u32,
                        r14,
                        listener_words,
                        vtable_words,
                        listener_0c_string,
                        listener_20_string,
                        scene_name,
                        stash_name
                    ));
                }
            }
        }

        if name.starts_with("SPIDEY_SIGNAL_FIRE_") {
            static SIGNAL_FIRE_DIAG: AtomicU32 = AtomicU32::new(0);
            let fire_n = SIGNAL_FIRE_DIAG.fetch_add(1, AO::Relaxed);

            let signal = if name == "SPIDEY_SIGNAL_FIRE_ENTRY_TAP" {
                context.Rcx as u32
            } else {
                context.Rdi as u32
            };
            let scene_name = read_guest_cstr_raw(r15 as *mut u8, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let frame = read_guest_u32(r15, 0x004B_C630);
            let frame_scene = if valid_guest_ptr(frame) {
                read_guest_u32(r15, frame.wrapping_add(0x18))
            } else {
                0
            };
            let scene_18c = if valid_guest_ptr(frame_scene) {
                read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
            } else {
                0
            };

            if fire_n < 128
                || signal == 0x105C_A7B0
                || scene_name.to_ascii_lowercase().starts_with("levels\\")
            {
                let sig_vt = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal)
                } else {
                    0
                };
                let sig_04 = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x04))
                } else {
                    0
                };
                let sig_08 = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x08))
                } else {
                    0
                };
                let sig_0c = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x0C))
                } else {
                    0
                };
                let sig_10 = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x10))
                } else {
                    0
                };
                let sig_14 = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x14))
                } else {
                    0
                };
                let sig_18 = if valid_guest_ptr(signal) {
                    read_guest_u32(r15, signal.wrapping_add(0x18))
                } else {
                    0
                };
                let sig_name = if valid_guest_ptr(sig_0c) {
                    read_c_string(r15, sig_0c, 80)
                } else {
                    String::new()
                };

                let head14_next = if valid_guest_ptr(sig_14) {
                    read_guest_u32(r15, sig_14)
                } else {
                    0
                };
                let head14_prev = if valid_guest_ptr(sig_14) {
                    read_guest_u32(r15, sig_14.wrapping_add(4))
                } else {
                    0
                };
                let head14_first_cb = if valid_guest_ptr(head14_next) {
                    read_guest_u32(r15, head14_next.wrapping_add(8))
                } else {
                    0
                };
                let (
                    active_list,
                    active_sentinel,
                    active_first,
                    active_first_prev,
                    active_second,
                    active_first_cb,
                    cb_vt,
                    cb_fn,
                    active_count,
                ) = signal_active_list_info(r15, signal);

                debug_log(&format!(
                    "[SPIDEY-SIGNAL-FIRE #{}] {} signal=0x{:08X} vt=0x{:08X} sig+04/08/0c/10/14/18=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} sig_name='{}' list14(next/prev/cb)=0x{:08X}/0x{:08X}/0x{:08X} active(list/sentinel/first/prev/second/cb/vt/fn/sec/count)=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/{}/{} frame_scene=0x{:08X} scene18c={} scene='{}' stash='{}' eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    fire_n,
                    name,
                    signal,
                    sig_vt,
                    sig_04,
                    sig_08,
                    sig_0c,
                    sig_10,
                    sig_14,
                    sig_18,
                    sig_name,
                    head14_next,
                    head14_prev,
                    head14_first_cb,
                    active_list,
                    active_sentinel,
                    active_first,
                    active_first_prev,
                    active_second,
                    active_first_cb,
                    cb_vt,
                    cb_fn,
                    spidey_section(cb_fn),
                    active_count,
                    frame_scene,
                    scene_18c,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_EVENT_LISTENER_MATCH_ENTRY_TAP" {
            static EVENT_MATCH_ENTRY_DIAG: AtomicU32 = AtomicU32::new(0);
            let match_n = EVENT_MATCH_ENTRY_DIAG.fetch_add(1, AO::Relaxed);
            let listener = context.Rcx as u32;
            let ret = read_guest_u32(r15, r14);
            let arg_addr = r14.wrapping_add(4);
            let arg0 = read_guest_u32(r15, arg_addr);
            let listener_vt = if valid_guest_ptr(listener) {
                read_guest_u32(r15, listener)
            } else {
                0
            };
            let listener_1e = if valid_guest_ptr(listener) {
                read_guest_u16(r15, listener.wrapping_add(0x1E))
            } else {
                0
            };
            let listener_20 = if valid_guest_ptr(listener) {
                read_guest_u32(r15, listener.wrapping_add(0x20))
            } else {
                0
            };
            let listener_24 = if valid_guest_ptr(listener) {
                read_guest_u32(r15, listener.wrapping_add(0x24))
            } else {
                0
            };
            let listener_08 = if valid_guest_ptr(listener) {
                read_guest_u32(r15, listener.wrapping_add(0x08))
            } else {
                0
            };
            let listener_18 = if valid_guest_ptr(listener) {
                read_guest_u32(r15, listener.wrapping_add(0x18))
            } else {
                0
            };
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let pending_word = read_guest_u32(r15, 0x003D_CFA0);
            let global_pend_begin = read_guest_u32(r15, 0x003D_CEC0);
            let global_pend_table = read_guest_u32(r15, 0x003D_CEC4);
            let global_pend_end = read_guest_u32(r15, 0x003D_CEC8);
            let frame = read_guest_u32(r15, 0x004B_C630);
            let frame_scene = if valid_guest_ptr(frame) {
                read_guest_u32(r15, frame.wrapping_add(0x18))
            } else {
                0
            };
            let scene_18c = if valid_guest_ptr(frame_scene) {
                read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
            } else {
                0
            };
            let scene_a0 = if valid_guest_ptr(frame_scene) {
                read_guest_u32(r15, frame_scene.wrapping_add(0xA0))
            } else {
                0
            };
            let state_idx = if valid_guest_ptr(scene_a0.wrapping_sub(0x10)) {
                read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
            } else {
                0
            };
            let state_arr = if valid_guest_ptr(scene_a0.wrapping_sub(0x14)) {
                read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
            } else {
                0
            };
            let state_val = if valid_guest_ptr(state_arr) {
                read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
            } else {
                0
            };

            let mut rewritten_to = 0u32;
            let rewrite_mode = if spidey_event_patches_enabled() {
                std::env::var("RUSTEMU_SPIDEY_EVENT_PAYLOAD_REWRITE").ok()
            } else {
                None
            };
            if scene_name.eq_ignore_ascii_case("levels\\origin_z")
                && arg0 == 0x1A
                && listener_vt == 0x0038_49D0
            {
                rewritten_to = match rewrite_mode.as_deref() {
                    Some("l20") | Some("20") | Some("+20") => listener_20,
                    Some("l24") | Some("24") | Some("+24") => listener_24,
                    _ => 0,
                };
                if valid_guest_ptr(rewritten_to)
                    || rewritten_to == listener_20
                    || rewritten_to == listener_24
                {
                    write_guest_u32(r15, arg_addr, rewritten_to);
                } else {
                    rewritten_to = 0;
                }
            }

            if match_n < 128
                || arg0 == 0x1A
                || rewritten_to != 0
                || scene_name.to_ascii_lowercase().starts_with("levels\\")
                || match_n.is_power_of_two()
            {
                debug_log(&format!(
                    "[SPIDEY-EVENT-LISTENER-MATCH-ENTRY #{}] listener=0x{:08X} vt=0x{:08X} ret=0x{:08X} arg_addr=0x{:08X} arg0=0x{:08X} rewrite_mode={:?} rewritten_to=0x{:08X} l+08/18/1e/20/24=0x{:08X}/0x{:08X}/0x{:04X}/0x{:08X}/0x{:08X} pending_word=0x{:08X} pend_begin/table/end=0x{:08X}/0x{:08X}/0x{:08X} frame_scene=0x{:08X} scene18c={} state_idx={} state_val={} scene='{}' stash='{}' eax=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    match_n,
                    listener,
                    listener_vt,
                    ret,
                    arg_addr,
                    arg0,
                    rewrite_mode,
                    rewritten_to,
                    listener_08,
                    listener_18,
                    listener_1e,
                    listener_20,
                    listener_24,
                    pending_word,
                    global_pend_begin,
                    global_pend_table,
                    global_pend_end,
                    frame_scene,
                    scene_18c,
                    state_idx,
                    state_val,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_SIGNAL_WRAPPER_FIRE_CALL_TAP"
            || name == "SPIDEY_SIGNAL_SECONDARY_FIRE_CALL_TAP"
        {
            static SIGNAL_CALLSITE_DIAG: AtomicU32 = AtomicU32::new(0);
            let sig_n = SIGNAL_CALLSITE_DIAG.fetch_add(1, AO::Relaxed);
            let signal = context.Rcx as u32;
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let sig_vt = read_guest_u32(r15, signal);
            let sig_04 = read_guest_u32(r15, signal.wrapping_add(0x04));
            let sig_08 = read_guest_u32(r15, signal.wrapping_add(0x08));
            let sig_0c = read_guest_u32(r15, signal.wrapping_add(0x0C));
            let sig_10 = read_guest_u32(r15, signal.wrapping_add(0x10));
            let sig_14 = read_guest_u32(r15, signal.wrapping_add(0x14));
            let sig_18 = read_guest_u32(r15, signal.wrapping_add(0x18));
            let sig_name = if valid_guest_ptr(sig_0c) {
                read_c_string(r15, sig_0c, 80)
            } else {
                String::new()
            };
            let (
                active_list,
                active_sentinel,
                active_first,
                active_first_prev,
                active_second,
                active_first_cb,
                cb_vt,
                cb_fn,
                active_count,
            ) = signal_active_list_info(r15, signal);

            if sig_n < 256
                || active_count != 0
                || scene_name.to_ascii_lowercase().starts_with("levels\\")
                || sig_n.is_power_of_two()
            {
                debug_log(&format!(
                    "[SPIDEY-SIGNAL-CALLSITE #{}] {} signal=0x{:08X} vt=0x{:08X} sig+04/08/0c/10/14/18=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} sig_name='{}' active(list/sentinel/first/prev/second/cb/vt/fn/sec/count)=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/{}/{} scene='{}' stash='{}' eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    sig_n,
                    name,
                    signal,
                    sig_vt,
                    sig_04,
                    sig_08,
                    sig_0c,
                    sig_10,
                    sig_14,
                    sig_18,
                    sig_name,
                    active_list,
                    active_sentinel,
                    active_first,
                    active_first_prev,
                    active_second,
                    active_first_cb,
                    cb_vt,
                    cb_fn,
                    spidey_section(cb_fn),
                    active_count,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rbx as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_SIGNAL_CALLBACK_CALL_TAP" {
            static SIGNAL_CALLBACK_DIAG: AtomicU32 = AtomicU32::new(0);
            let cb_n = SIGNAL_CALLBACK_DIAG.fetch_add(1, AO::Relaxed);

            let signal = context.Rdi as u32;
            let list_node = context.Rsi as u32;
            let callback = context.Rcx as u32;
            let callback_vt = context.Rax as u32;
            let callback_fn = if valid_guest_ptr(callback_vt) {
                read_guest_u32(r15, callback_vt.wrapping_add(0x08))
            } else {
                0
            };
            let callback_vt_m4 = if valid_guest_ptr(callback_vt.wrapping_sub(4)) {
                read_guest_u32(r15, callback_vt.wrapping_sub(4))
            } else {
                0
            };
            let stack_arg0 = read_guest_u32(r15, r14);
            let stack_arg1 = read_guest_u32(r15, r14.wrapping_add(4));
            let sig_vt = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal)
            } else {
                0
            };
            let sig_name_ptr = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal.wrapping_add(0x0C))
            } else {
                0
            };
            let sig_name = if valid_guest_ptr(sig_name_ptr) {
                read_c_string(r15, sig_name_ptr, 80)
            } else {
                String::new()
            };
            let cb_04 = if valid_guest_ptr(callback) {
                read_guest_u32(r15, callback.wrapping_add(0x04))
            } else {
                0
            };
            let cb_08 = if valid_guest_ptr(callback) {
                read_guest_u32(r15, callback.wrapping_add(0x08))
            } else {
                0
            };
            let cb_0c = if valid_guest_ptr(callback) {
                read_guest_u32(r15, callback.wrapping_add(0x0C))
            } else {
                0
            };
            let cb_10 = if valid_guest_ptr(callback) {
                read_guest_u32(r15, callback.wrapping_add(0x10))
            } else {
                0
            };
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let frame = read_guest_u32(r15, 0x004B_C630);
            let frame_scene = if valid_guest_ptr(frame) {
                read_guest_u32(r15, frame.wrapping_add(0x18))
            } else {
                0
            };
            let scene_18c = if valid_guest_ptr(frame_scene) {
                read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
            } else {
                0
            };

            if cb_n < 128
                || signal == 0x105C_A1B0
                || scene_name.to_ascii_lowercase().starts_with("levels\\")
            {
                debug_log(&format!(
                    "[SPIDEY-SIGNAL-CALLBACK #{}] signal=0x{:08X} sig_vt=0x{:08X} sig_name='{}' node=0x{:08X} callback=0x{:08X} cb_vt=0x{:08X} vt[-4]=0x{:08X} cb_fn=0x{:08X} fn_sec={} stack_args=0x{:08X}/0x{:08X} cb+04/08/0c/10=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} frame_scene=0x{:08X} scene18c={} scene='{}' stash='{}' eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    cb_n,
                    signal,
                    sig_vt,
                    sig_name,
                    list_node,
                    callback,
                    callback_vt,
                    callback_vt_m4,
                    callback_fn,
                    spidey_section(callback_fn),
                    stack_arg0,
                    stack_arg1,
                    cb_04,
                    cb_08,
                    cb_0c,
                    cb_10,
                    frame_scene,
                    scene_18c,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_SIGNAL_LOOP_HEAD_TAP" || name == "SPIDEY_SIGNAL_CALLBACK_RET_TAP" {
            static SIGNAL_LOOP_DIAG: AtomicU32 = AtomicU32::new(0);
            let loop_n = SIGNAL_LOOP_DIAG.fetch_add(1, AO::Relaxed);

            let signal = context.Rdi as u32;
            let current_node = context.Rsi as u32;
            let sentinel = context.Rbx as u32;
            let head = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal.wrapping_add(0x10))
            } else {
                0
            };
            let first_node = if valid_guest_ptr(head) {
                read_guest_u32(r15, head)
            } else {
                0
            };
            let first_prev = if valid_guest_ptr(head) {
                read_guest_u32(r15, head.wrapping_add(4))
            } else {
                0
            };
            let node_next = if valid_guest_ptr(current_node) {
                read_guest_u32(r15, current_node)
            } else {
                0
            };
            let node_prev = if valid_guest_ptr(current_node) {
                read_guest_u32(r15, current_node.wrapping_add(4))
            } else {
                0
            };
            let callback = if valid_guest_ptr(current_node) {
                read_guest_u32(r15, current_node.wrapping_add(8))
            } else {
                0
            };
            let callback_vt = if valid_guest_ptr(callback) {
                read_guest_u32(r15, callback)
            } else {
                0
            };
            let callback_fn = if valid_guest_ptr(callback_vt) {
                read_guest_u32(r15, callback_vt.wrapping_add(8))
            } else {
                0
            };
            let callback_vt_m4 = if valid_guest_ptr(callback_vt.wrapping_sub(4)) {
                read_guest_u32(r15, callback_vt.wrapping_sub(4))
            } else {
                0
            };
            let sig_vt = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal)
            } else {
                0
            };
            let sig_name_ptr = if valid_guest_ptr(signal) {
                read_guest_u32(r15, signal.wrapping_add(0x0C))
            } else {
                0
            };
            let sig_name = if valid_guest_ptr(sig_name_ptr) {
                read_c_string(r15, sig_name_ptr, 80)
            } else {
                String::new()
            };
            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let interesting_scene = scene_name.to_ascii_lowercase().starts_with("levels\\");
            if loop_n < 256 || interesting_scene || loop_n.is_power_of_two() {
                debug_log(&format!(
                    "[SPIDEY-SIGNAL-LOOP #{}] {} signal=0x{:08X} sig_vt=0x{:08X} sig_name='{}' head=0x{:08X} head(next/prev)=0x{:08X}/0x{:08X} cur=0x{:08X} cur(next/prev/cb)=0x{:08X}/0x{:08X}/0x{:08X} sentinel=0x{:08X} cb_vt=0x{:08X} vt[-4]=0x{:08X} cb_fn=0x{:08X} fn_sec={} ret_eax=0x{:08X} scene='{}' stash='{}' eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    loop_n,
                    name,
                    signal,
                    sig_vt,
                    sig_name,
                    head,
                    first_node,
                    first_prev,
                    current_node,
                    node_next,
                    node_prev,
                    callback,
                    sentinel,
                    callback_vt,
                    callback_vt_m4,
                    callback_fn,
                    spidey_section(callback_fn),
                    context.Rax as u32,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_EVENT_LISTENER_REGISTER_TAP" {
            static EVENT_LISTENER_REGISTER_DIAG: AtomicU32 = AtomicU32::new(0);
            let reg_n = EVENT_LISTENER_REGISTER_DIAG.fetch_add(1, AO::Relaxed);

            let signal = context.Rcx as u32;
            let ret_on_stack = read_guest_u32(r15, r14);
            let callsite = ret_on_stack.wrapping_sub(5);
            let arg0 = read_guest_u32(r15, r14.wrapping_add(4));
            let arg1 = read_guest_u32(r15, r14.wrapping_add(8));
            let arg2 = read_guest_u32(r15, r14.wrapping_add(12));
            let arg3 = read_guest_u32(r15, r14.wrapping_add(16));
            let slot_param5 = arg3 & 0xFF;
            let wrapper = match ret_on_stack {
                0x0005_0F60 => "register_global_wrapper",
                0x0005_16C1 => "register_local_wrapper",
                _ => "unknown",
            };

            let scene_name = read_c_string(r15, 0x004B_C848, 64);
            let stash_name = read_c_string(r15, 0x004B_C948, 64);
            let interesting_scene = scene_name.eq_ignore_ascii_case("bonus\\menu")
                || scene_name.to_ascii_lowercase().starts_with("levels\\");

            if reg_n < 128
                || interesting_scene
                || matches!(slot_param5, 0x18 | 0x19 | 0x1A)
                || ret_on_stack == 0x0005_0F60
                || ret_on_stack == 0x0005_16C1
            {
                let frame = read_guest_u32(r15, 0x004B_C630);
                let frame_scene = if valid_guest_ptr(frame) {
                    read_guest_u32(r15, frame.wrapping_add(0x18))
                } else {
                    0
                };
                let scene_18c = if valid_guest_ptr(frame_scene) {
                    read_guest_u8(r15, frame_scene.wrapping_add(0x18C))
                } else {
                    0
                };
                let scene_a0 = if valid_guest_ptr(frame_scene) {
                    read_guest_u32(r15, frame_scene.wrapping_add(0xA0))
                } else {
                    0
                };
                let state_idx = if valid_guest_ptr(scene_a0) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x10))
                } else {
                    0
                };
                let state_arr = if valid_guest_ptr(scene_a0) {
                    read_guest_u32(r15, scene_a0.wrapping_sub(0x14))
                } else {
                    0
                };
                let state_val = if valid_guest_ptr(state_arr) && state_idx < 0x1000 {
                    read_guest_u32(r15, state_arr.wrapping_add(state_idx.wrapping_mul(4)))
                } else {
                    0
                };

                let sig_vt = read_guest_u32(r15, signal);
                let sig_04 = read_guest_u32(r15, signal.wrapping_add(0x04));
                let sig_08 = read_guest_u32(r15, signal.wrapping_add(0x08));
                let sig_0c = read_guest_u32(r15, signal.wrapping_add(0x0C));
                let sig_10 = read_guest_u32(r15, signal.wrapping_add(0x10));
                let sig_14 = read_guest_u32(r15, signal.wrapping_add(0x14));
                let sig_18 = read_guest_u32(r15, signal.wrapping_add(0x18));
                let sig_1c = read_guest_u32(r15, signal.wrapping_add(0x1C));
                let sig_20 = read_guest_u32(r15, signal.wrapping_add(0x20));
                let sig_name = if valid_guest_ptr(sig_0c) {
                    read_c_string(r15, sig_0c, 80)
                } else {
                    String::new()
                };
                let arg0_vt = if valid_guest_ptr(arg0) {
                    read_guest_u32(r15, arg0)
                } else {
                    0
                };
                let arg1_vt = if valid_guest_ptr(arg1) {
                    read_guest_u32(r15, arg1)
                } else {
                    0
                };
                let arg2_vt = if valid_guest_ptr(arg2) {
                    read_guest_u32(r15, arg2)
                } else {
                    0
                };
                let head14_next = if valid_guest_ptr(sig_14) {
                    read_guest_u32(r15, sig_14)
                } else {
                    0
                };
                let head14_prev = if valid_guest_ptr(sig_14) {
                    read_guest_u32(r15, sig_14.wrapping_add(4))
                } else {
                    0
                };

                if spidey_event1_replay_enabled()
                    && SPIDEY_EVENT1_DEFER_PENDING.load(AO::Relaxed) != 0
                    && scene_name.eq_ignore_ascii_case("bonus\\menu")
                    && state_val == 3
                {
                    let deferred_obj = SPIDEY_EVENT1_DEFER_OBJ.load(AO::Relaxed);
                    let deferred_scene = SPIDEY_EVENT1_DEFER_SCENE.load(AO::Relaxed);
                    let target_matches = deferred_obj == 0
                        || sig_18 == deferred_obj
                        || frame_scene == deferred_scene;
                    if target_matches
                        && SPIDEY_EVENT1_DEFER_PENDING
                            .compare_exchange(1, 0, AO::Relaxed, AO::Relaxed)
                            .is_ok()
                    {
                        let master_addr = 0x003D_CFA0u32;
                        let key_addr = 0x003D_CEC8u32;
                        let flag_addr = 0x003D_CECCu32;
                        let old_master = read_guest_u8(r15, master_addr);
                        let old_key = read_guest_u32(r15, key_addr);
                        let old_flag = read_guest_u32(r15, flag_addr);
                        if old_key == 0 {
                            write_guest_u32(r15, key_addr, 1);
                        }
                        write_guest_u32(r15, flag_addr, 1);
                        write_guest_u8(r15, master_addr, old_master | 1);

                        let obj_table = if valid_guest_ptr(deferred_obj) {
                            read_guest_u32(r15, deferred_obj.wrapping_add(0x08))
                        } else {
                            0
                        };
                        let obj_slots = if valid_guest_ptr(obj_table) {
                            read_guest_u32(r15, obj_table.wrapping_add(0x04))
                        } else {
                            0
                        };
                        let obj_slot1 = if valid_guest_ptr(obj_slots) {
                            read_guest_u32(r15, obj_slots.wrapping_add(4))
                        } else {
                            0
                        };
                        let replay_n = SPIDEY_EVENT1_REPLAY_LOG.fetch_add(1, AO::Relaxed);
                        debug_log(&format!(
                            "[SPIDEY-EVENT1-REPLAY] #{} listener_reg={} signal=0x{:08X} sig18=0x{:08X} deferred_obj=0x{:08X} deferred_scene=0x{:08X} frame_scene=0x{:08X} table=0x{:08X} slot1=0x{:08X} master:{:02X}->{:02X} key:0x{:08X}->0x{:08X} flag:0x{:08X}->0x{:08X} state_val={} scene18c={} scene='{}'",
                            replay_n,
                            reg_n,
                            signal,
                            sig_18,
                            deferred_obj,
                            deferred_scene,
                            frame_scene,
                            obj_table,
                            obj_slot1,
                            old_master,
                            read_guest_u8(r15, master_addr),
                            old_key,
                            read_guest_u32(r15, key_addr),
                            old_flag,
                            read_guest_u32(r15, flag_addr),
                            state_val,
                            scene_18c,
                            scene_name
                        ));
                    }
                }

                debug_log(&format!(
                    "[SPIDEY-EVENT-LISTENER-REGISTER #{}] signal=0x{:08X} vt=0x{:08X} sig+04/08/0c/10/14/18/1c/20=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} sig_name='{}' list14(next/prev)=0x{:08X}/0x{:08X} ret=0x{:08X} callsite=0x{:08X} wrapper={} slot_param5=0x{:02X} args=0x{:08X}/0x{:08X}/0x{:08X}/0x{:08X} arg_vt=0x{:08X}/0x{:08X}/0x{:08X} frame_scene=0x{:08X} scene18c={} state_idx={} state_val={} scene='{}' stash='{}' eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} esp=0x{:08X}",
                    reg_n,
                    signal,
                    sig_vt,
                    sig_04,
                    sig_08,
                    sig_0c,
                    sig_10,
                    sig_14,
                    sig_18,
                    sig_1c,
                    sig_20,
                    sig_name,
                    head14_next,
                    head14_prev,
                    ret_on_stack,
                    callsite,
                    wrapper,
                    slot_param5,
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg0_vt,
                    arg1_vt,
                    arg2_vt,
                    frame_scene,
                    scene_18c,
                    state_idx,
                    state_val,
                    scene_name,
                    stash_name,
                    context.Rax as u32,
                    context.Rcx as u32,
                    context.Rdx as u32,
                    context.Rsi as u32,
                    context.Rdi as u32,
                    r14
                ));
            }
        }

        if name == "SPIDEY_BOOT_GATE_TAP" {
            static BOOT_GATE_DIAG: AtomicU32 = AtomicU32::new(0);
            let boot_n = BOOT_GATE_DIAG.fetch_add(1, AO::Relaxed);
            if boot_n < 64 || boot_n.is_power_of_two() {
                let scene_mgr = context.Rcx as u32;
                let engine = unsafe { *((r15 + 0x004B_C614) as *const u32) };
                let frame_ctx = unsafe { *((r15 + 0x004B_C630) as *const u32) };
                let action_mgr = unsafe { *((r15 + 0x003F_7D90) as *const u32) };
                let loading_mgr = unsafe { *((r15 + 0x003F_5EB0) as *const u32) };
                let xgraph = unsafe { *((r15 + 0x004C_06B8) as *const u32) };
                let xgraph134 = read_guest_u32(r15, xgraph.wrapping_add(0x134));
                let xgraph134_vt = read_guest_u32(r15, xgraph134);
                let xgraph_ready_cb = read_guest_u32(r15, xgraph134_vt.wrapping_add(0x308));
                let mut patched_ready_cb = 0u32;
                if title_patches_enabled()
                    && std::env::var_os("RUSTEMU_SPIDEY_FORCE_XGRAPH_READY").is_some()
                    && valid_guest_ptr(xgraph134_vt.wrapping_add(0x308))
                    && xgraph_ready_cb == 0x0015_C2A0
                {
                    // The current menu xgraph readiness callback is a false
                    // stub (`xor al, al; ret`). Redirect only this vtable
                    // slot to an existing true stub (`mov al, 1; ret`) so the
                    // native F8580 gate can take its real ready path.
                    write_guest_u32(r15, xgraph134_vt.wrapping_add(0x308), 0x0001_4F80);
                    patched_ready_cb = 0x0001_4F80;
                }
                let scene28 = read_guest_u32(r15, scene_mgr.wrapping_add(0x28));
                let scene28_begin = read_guest_u32(r15, scene28.wrapping_add(0x78));
                let scene28_end = read_guest_u32(r15, scene28.wrapping_add(0x7C));
                let scene28_134 = read_guest_u32(r15, scene28.wrapping_add(0x134));
                let event_arg = read_guest_u32(r15, (context.R14 as u32).wrapping_add(4));
                debug_log(&format!(
                    "[SPIDEY-BOOT-GATE #{}] scene_mgr=0x{:08X} flags181/182/184/185/188/189/18c/18e/18f={:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X}/{:02X} \
                     engine=0x{:08X} eng0/10/11={:02X}/{:02X}/{:02X} frame=0x{:08X} frame+20=0x{:08X} action_mgr=0x{:08X} action+58=0x{:08X} \
                     load_mgr=0x{:08X} load+d4/f4=0x{:08X}/0x{:08X} xgraph=0x{:08X} xgraph+134=0x{:08X} cb308=0x{:08X} patched_cb308=0x{:08X} \
                     scene28=0x{:08X} scene28+78/7c=0x{:08X}/0x{:08X} scene28+134=0x{:08X} arg=0x{:08X} eax=0x{:08X} edx=0x{:08X}",
                    boot_n,
                    scene_mgr,
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x181)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x182)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x184)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x185)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x188)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x189)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x18C)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x18E)),
                    read_guest_u8(r15, scene_mgr.wrapping_add(0x18F)),
                    engine,
                    read_guest_u8(r15, engine),
                    read_guest_u8(r15, engine.wrapping_add(0x10)),
                    read_guest_u8(r15, engine.wrapping_add(0x11)),
                    frame_ctx,
                    read_guest_u32(r15, frame_ctx.wrapping_add(0x20)),
                    action_mgr,
                    read_guest_u32(r15, action_mgr.wrapping_add(0x58)),
                    loading_mgr,
                    read_guest_u32(r15, loading_mgr.wrapping_add(0xD4)),
                    read_guest_u32(r15, loading_mgr.wrapping_add(0xF4)),
                    xgraph,
                    xgraph134,
                    xgraph_ready_cb,
                    patched_ready_cb,
                    scene28,
                    scene28_begin,
                    scene28_end,
                    scene28_134,
                    event_arg,
                    context.Rax as u32,
                    context.Rdx as u32
                ));
            }
        }

        if name == "SPIDEY_ACTION_GATE_ENTRY_TAP"
            || name == "SPIDEY_ACTION_GATE_SELECTED_TAP"
            || name == "SPIDEY_ACTION_GATE_FALSE_TAP"
            || name == "SPIDEY_ACTION_GATE_TRUE_TAP"
        {
            static ACTION_GATE_DIAG: AtomicU32 = AtomicU32::new(0);
            let action_n = ACTION_GATE_DIAG.fetch_add(1, AO::Relaxed);
            if action_n < 96 || action_n.is_power_of_two() {
                let mgr = unsafe { *((r15 + 0x003F_7D90) as *const u32) };
                let engine = unsafe { *((r15 + 0x004B_C614) as *const u32) };
                let engine10 = read_guest_u8(r15, engine.wrapping_add(0x10));
                let engine11 = read_guest_u8(r15, engine.wrapping_add(0x11));
                let active = read_guest_u32(r15, mgr.wrapping_add(0x58));
                let arg_off = if name == "SPIDEY_ACTION_GATE_ENTRY_TAP" {
                    4
                } else {
                    0x1C
                };
                let arg0 = read_guest_u32(r15, (context.R14 as u32).wrapping_add(arg_off));
                let selected_obj = context.Rsi as u32;
                let selected_vt = if selected_obj >= 0x1000 && valid_guest_ptr(selected_obj) {
                    read_guest_u32(r15, selected_obj)
                } else {
                    0
                };
                let selected_cb08 = if selected_vt >= 0x1000 && valid_guest_ptr(selected_vt) {
                    read_guest_u32(r15, selected_vt.wrapping_add(0x08))
                } else {
                    0
                };
                let selected_cb10 = if selected_vt >= 0x1000 && valid_guest_ptr(selected_vt) {
                    read_guest_u32(r15, selected_vt.wrapping_add(0x10))
                } else {
                    0
                };
                let selected_cb14 = if selected_vt >= 0x1000 && valid_guest_ptr(selected_vt) {
                    read_guest_u32(r15, selected_vt.wrapping_add(0x14))
                } else {
                    0
                };
                let selected_cb1c = if selected_vt >= 0x1000 && valid_guest_ptr(selected_vt) {
                    read_guest_u32(r15, selected_vt.wrapping_add(0x1C))
                } else {
                    0
                };
                let slot1 = read_guest_u32(r15, mgr.wrapping_add(0x30));
                let slot1_vt = if slot1 >= 0x1000 && valid_guest_ptr(slot1) {
                    read_guest_u32(r15, slot1)
                } else {
                    0
                };
                debug_log(&format!(
                    "[SPIDEY-ACTION #{}] {} action=0x{:08X} mgr=0x{:08X} active={} engine=0x{:08X} engine+10/11={}/{} ebx={} ebp=0x{:08X} selected=0x{:08X} vt=0x{:08X} cb08=0x{:08X} cb10=0x{:08X} cb14=0x{:08X} cb1c=0x{:08X} slot1=0x{:08X} slot1_vt=0x{:08X} eax=0x{:08X} edx=0x{:08X}",
                    action_n,
                    name,
                    arg0,
                    mgr,
                    active,
                    engine,
                    engine10,
                    engine11,
                    context.Rbx as u32,
                    context.Rbp as u32,
                    selected_obj,
                    selected_vt,
                    selected_cb08,
                    selected_cb10,
                    selected_cb14,
                    selected_cb1c,
                    slot1,
                    slot1_vt,
                    context.Rax as u32,
                    context.Rdx as u32
                ));
            }
        }
    }

    // ---- Doom gamestate-init probe (2026-04-26, Doom-gated) ----
    // Three TAPs identify which step in the gamestate-NULL chain breaks:
    //   OUTER_ENTRY: was sub_00012C10 reached at all? (count arg = [esp+4])
    //   INNER_ENTRY: was sub_000129F0 reached? (preexisting ptr in ECX/EDI)
    //   ALLOC_RET:   what did sub_45652 return? (EAX after add esp,4)
    if name == "DOOM_GAMESTATE_OUTER_ENTRY_TAP"
        || name == "DOOM_GAMESTATE_INNER_ENTRY_TAP"
        || name == "DOOM_ALLOC_RET_TAP"
    {
        use std::sync::atomic::{AtomicU32, Ordering as AO};
        static DOOM_PROBE_DIAG: AtomicU32 = AtomicU32::new(0);
        let dn = DOOM_PROBE_DIAG.fetch_add(1, AO::Relaxed);
        if dn < 32 || dn.is_power_of_two() {
            // Doom gamestate globals
            let init_flag = unsafe { *((r15 + 0x0010_1BB4u64) as *const u32) };
            let curr_slot = unsafe { *((r15 + 0x0010_1B9Cu64) as *const u32) };
            let gamestate = unsafe { *((r15 + 0x0010_1BB8u64) as *const u32) };
            let gs_array_0 = unsafe { *((r15 + 0x0010_1BA4u64) as *const u32) };
            let arg0 = unsafe { *((r15 + r14 as u64 + 4) as *const u32) };
            let arg1 = unsafe { *((r15 + r14 as u64 + 8) as *const u32) };
            debug_log(&format!(
                "[DOOM-PROBE #{}] {} guest=0x{:08X} ret=0x{:08X} esp=0x{:08X} \
                 ecx=0x{:08X} edi=0x{:08X} eax=0x{:08X} arg0=0x{:08X} arg1=0x{:08X} \
                 gamestate=0x{:08X} init_flag=0x{:08X} curr_slot=0x{:08X} gs_array[0]=0x{:08X}",
                dn,
                name,
                guest_addr,
                ret_addr,
                r14,
                context.Rcx as u32,
                context.Rdi as u32,
                context.Rax as u32,
                arg0,
                arg1,
                gamestate,
                init_flag,
                curr_slot,
                gs_array_0
            ));
        }
    }

    if name.starts_with("DOOM_RENDER_") {
        super::oovpa_hle::doom_render_phase_tap(name, context.R15 as *mut u8);
    }
    if name == "DOOM_COLUMN_WRITE_TAP" || name == "DOOM_COLUMN_EDX_TAP" {
        super::oovpa_hle::doom_column_write_tap(
            name,
            context.R15 as *mut u8,
            context.Rdx as u32,
            context.Rax as u8,
            context.Rcx as u32,
            context.Rbx as u32,
            context.Rbp as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_COLUMN_CALL_TAP" || name == "DOOM_COLUMN_CALL_AFTER_TAP" {
        super::oovpa_hle::doom_column_call_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_SPAN_CALL_TAP" || name == "DOOM_SPAN_CALL_AFTER_TAP" {
        super::oovpa_hle::doom_span_call_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_WALL_CLIP_SOLID_ENTRY_TAP"
        || name == "DOOM_WALL_CLIP_PASS_ENTRY_TAP"
        || name == "DOOM_WALL_RANGE_ENTRY_TAP"
    {
        super::oovpa_hle::doom_wall_range_entry_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rbx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
            context.R14 as u32,
        );
    } else if name == "DOOM_WALL_VIEWMAP_ENTRY_TAP"
        || name == "DOOM_WALL_SETVIEW_ENTRY_TAP"
        || name == "DOOM_WALL_EXEC_SETVIEW_ENTRY_TAP"
        || name == "DOOM_WALL_RENDER_INIT_TAP"
        || name == "DOOM_WALL_VIEWMAP_BUILD_STORE_TAP"
        || name == "DOOM_WALL_VIEWMAP_CLAMP_INPUT_TAP"
        || name == "DOOM_WALL_VIEWMAP_CLAMP_FINAL_TAP"
    {
        super::oovpa_hle::doom_wall_viewmap_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rbx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
            context.R14 as u32,
        );
    } else if name == "DOOM_WALL_PROJECTED_X_TAP"
        || name == "DOOM_WALL_PASS_DECISION_TAP"
        || name == "DOOM_WALL_SOLID_DECISION_TAP"
    {
        if name == "DOOM_WALL_PROJECTED_X_TAP" {
            if env_flag("RUSTEMU_DOOM_SYNTH_PROJECT") {
                static SYNTH_PROJECT_APPLY_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                if let Some((synth_start, synth_after)) =
                    super::oovpa_hle::doom_wall_synthetic_projection(
                        context.R15 as *mut u8,
                        context.Rbp as u32,
                    )
                {
                    let old_start = context.Rbx as u32;
                    let old_after = context.Rsi as u32;
                    context.Rbx = (context.Rbx & !0xFFFF_FFFFu64) | synth_start as u64;
                    context.Rsi = (context.Rsi & !0xFFFF_FFFFu64) | synth_after as u64;
                    let n = SYNTH_PROJECT_APPLY_LOG.fetch_add(1, Ordering::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        debug_log(&format!(
                            "[DOOM-WALL-SYNTH-APPLY] #{} start {} -> {} after {} -> {} at guest=0x{:08X} seg=0x{:08X}",
                            n,
                            old_start,
                            synth_start,
                            old_after,
                            synth_after,
                            guest_addr,
                            context.Rbp as u32
                        ));
                    }
                }
            } else if let Some(force_after) = env_u32("RUSTEMU_DOOM_FORCE_PROJECTED_AFTER") {
                static FORCE_PROJECTED_AFTER_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let old = context.Rsi as u32;
                context.Rsi = (context.Rsi & !0xFFFF_FFFFu64) | force_after as u64;
                let n = FORCE_PROJECTED_AFTER_LOG.fetch_add(1, Ordering::Relaxed);
                if n < 32 || n.is_power_of_two() {
                    debug_log(&format!(
                        "[DOOM-WALL-FORCE-PROJECT] #{} after_x esi {} -> {} at guest=0x{:08X}",
                        n, old, force_after, guest_addr
                    ));
                }
            }
        }
        super::oovpa_hle::doom_wall_projected_x_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rbx as u32,
            context.Rbp as u32,
            context.Rsi as u32,
            context.Rdi as u32,
            context.R14 as u32,
        );
    } else if name == "DOOM_WALL_CLIP_INPUT_TAP"
        || name == "DOOM_WALL_CLIP_OUTPUT_TAP"
        || name == "DOOM_WALL_PRE_LOOKUP_TAP"
    {
        super::oovpa_hle::doom_wall_angle_clip_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rbx as u32,
            context.Rbp as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_WALL_POINTANGLE1_RET_TAP" || name == "DOOM_WALL_POINTANGLE2_RET_TAP" {
        super::oovpa_hle::doom_wall_pointangle_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rbx as u32,
            context.Rbp as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_WALL_SCALE_SETUP_TAP" {
        super::oovpa_hle::doom_wall_scale_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    } else if name == "DOOM_MASKED_SCALE_SOURCE_TAP" {
        super::oovpa_hle::doom_masked_scale_source_tap(
            name,
            context.R15 as *mut u8,
            context.Rax as u32,
            context.Rcx as u32,
            context.Rdx as u32,
            context.Rsi as u32,
            context.Rdi as u32,
        );
    }

    // LTCG SetTexture [HLE-SETTEX-LTCG] logging block reverted — Spider-Man
    // XDK 4134 proven not to compile SetTexture in LTCG-specialized form.
    // Re-add this block alongside the matching arms in the interpreter/match
    // paths if a title that uses LTCG SetTexture specialization is targeted.

    // Warn unconditionally on null/suspicious return address
    if ret_addr == 0 || ret_addr >= 0xF000_0000 {
        debug_log(&format!(
            "[OOVPA-WARN] #{} {} at guest=0x{:08X} has BAD ret_addr=0x{:08X} R14=0x{:08X} — will jump to null/invalid",
            cc, name, guest_addr, ret_addr, r14
        ));
    }

    // Log ALL HLE calls with structured format: [D3D] FunctionName(arg0, arg1, ...) → result
    // Rate-limit after 50 calls per function to avoid log flooding
    if cc <= 50 || (cc & (cc - 1)) == 0 {
        let mut arg_str = String::with_capacity(64);
        for i in 0..argc.min(8) {
            if i > 0 {
                arg_str.push_str(", ");
            }
            arg_str.push_str(&format!("0x{:08X}", args[i as usize]));
        }
        debug_log(&format!(
            "[D3D] #{} {}({}) ret_to=0x{:08X} guest=0x{:08X}{}",
            cc,
            name,
            arg_str,
            ret_addr,
            guest_addr,
            if is_manual { " [manual]" } else { "" }
        ));
    }

    // Normalize LTCG pattern names for HLE dispatch
    let hle_name = if name.contains("__LTCG") {
        let base = if let Some(idx) = name.find("__LTCG") {
            &name[..idx]
        } else {
            name
        };
        // Strip trailing "_N" size suffix
        if base.len() > 3 {
            let last_us = base.rfind('_').unwrap_or(0);
            let suffix = &base[last_us + 1..];
            if suffix.len() <= 2 && suffix.chars().all(|c| c.is_ascii_digit()) {
                &base[..last_us]
            } else {
                base
            }
        } else {
            base
        }
    } else {
        name
    };

    let force_harvester_hle = env_flag("RUSTEMU_D3D_HARVESTER_HLE_CREATEDEVICE")
        || env_flag("RUSTEMU_OOVPA_MAP_HLE_CREATEDEVICE");
    let force_createdevice_hle =
        super::IS_BLUE_PADDED.load(Ordering::Relaxed) || force_harvester_hle;
    let force_reset_hle = force_harvester_hle || env_flag("RUSTEMU_D3D_HARVESTER_HLE_RESET");

    // TAP override: let CreateDevice run natively so the D3D runtime initializes
    // its internal state (contiguous memory, VBlank DPC, interrupt setup).
    // Our HLE stub skipped all internal kernel calls, leaving the game in error state.
    let hle_mode = if hle_name == "D3DDevice_Reset" && !force_reset_hle {
        debug_log(&format!(
            "[TAP] Reset pass-through at guest=0x{:08X} — letting native XTL recreate framebuffers",
            guest_addr
        ));
        RESET_TAP_ACTIVE.store(true, Ordering::Relaxed);
        RESET_TAP_R14.store(context.R14 as u32, Ordering::Relaxed);
        RESET_TAP_HOOK_COUNT.store(0, Ordering::Relaxed);
        debug_log(&format!(
            "[TAP] Suppressing D3D hooks while Reset runs (R14=0x{:08X})",
            context.R14 as u32
        ));
        HleMode::Tap
    } else if hle_name == "D3DDevice_Reset" && force_reset_hle {
        debug_log(&format!(
            "[OOVPA-HARVESTER] D3DDevice_Reset HLE override at guest=0x{:08X} (native TAP disabled by fixture gate)",
            guest_addr
        ));
        hle_mode
    } else if hle_name == "Direct3D_CreateDevice" && !force_createdevice_hle {
        // TAP path for real games (Spider-Man, etc.). For test_blue_padded the
        // IS_BLUE_PADDED flag is set by patches::apply_pre_aot_patches, which
        // diverts CreateDevice to the HLE-override arm in oovpa_hle.rs — the
        // test's main only needs a valid device pointer written to its ppDevice
        // out-arg, not any guest-built internal D3D state.
        debug_log(&format!(
            "[TAP] CreateDevice pass-through at guest=0x{:08X} — letting native D3D init run",
            guest_addr
        ));
        // Capture g_pDevice address before passing through
        {
            let mem = unsafe { std::slice::from_raw_parts(ctx.guest_mem_base, 0x0800_0000) };
            let func = guest_addr as usize;
            for off in 0..256usize {
                if func + off + 6 > mem.len() {
                    break;
                }
                // mov [imm32], reg — 89 1D/35/3D xx xx xx xx
                if mem[func + off] == 0x89
                    && (mem[func + off + 1] == 0x1D
                        || mem[func + off + 1] == 0x35
                        || mem[func + off + 1] == 0x3D)
                {
                    let addr = u32::from_le_bytes([
                        mem[func + off + 2],
                        mem[func + off + 3],
                        mem[func + off + 4],
                        mem[func + off + 5],
                    ]);
                    if addr > 0x10000 && addr < 0x800000 {
                        // Spider-Man's native CreateDevice contains early
                        // absolute stores into the CDevice object itself
                        // (for example 0x00300E08). Those are not g_pDevice
                        // globals, and treating them as such makes later HLE
                        // APIs read device flags as a pointer. The known XDK
                        // 4134 global remains the authoritative slot.
                        let g_pdevice_addr = if guest_addr == 0x002F3BD0 {
                            super::G_PDEVICE
                        } else {
                            addr
                        };
                        super::G_PDEVICE_DYNAMIC
                            .store(g_pdevice_addr, std::sync::atomic::Ordering::Relaxed);
                        debug_log(&format!(
                            "[TAP] CreateDevice: captured g_pDevice=0x{:08X} (scan=0x{:08X})",
                            g_pdevice_addr, addr
                        ));
                        break;
                    }
                }
            }
        }
        crate::xbox::aot::veh_dpc::signal_crt_boot_complete();
        // UI-SEED moved to worker startup (worker.rs) — fires unconditionally,
        // not gated on CreateDevice TAP entry which may not always run.

        // Request GPU backend init on main thread (2026-04-21).
        //
        // The HLE match arm for "Direct3D_CreateDevice" in oovpa_hle.rs:253
        // already does this. But for XBEs where CreateDevice is TAP'd
        // (canonical non-LTCG prologue, like Spider-Man and the padded
        // test XBE), we bypass the HLE arm entirely. Without this call,
        // Backend stays 'none' and no pixel output is possible even when
        // native CreateDevice completes. Symptom on the HUD:
        //   "Backend: none    Swap: 0    Draw: 0"
        // even though CreateDevice: YES and stages advance.
        crate::xbox::gpu::request_gpu_init();

        // Activate TAP suppression: all D3D HLE hooks will pass-through
        // until CreateDevice returns (R14 rises above saved value).
        CREATEDEVICE_TAP_ACTIVE.store(true, Ordering::Relaxed);
        CREATEDEVICE_TAP_R14.store(context.R14 as u32, Ordering::Relaxed);
        debug_log(&format!(
            "[TAP] Suppressing D3D hooks while CreateDevice runs (R14=0x{:08X})",
            context.R14 as u32
        ));
        HleMode::Tap
    } else if hle_name == "Direct3D_CreateDevice" && force_createdevice_hle {
        debug_log(&format!(
            "[OOVPA-HARVESTER] Direct3D_CreateDevice HLE override at guest=0x{:08X} (native TAP disabled by fixture gate)",
            guest_addr
        ));
        hle_mode
    } else {
        hle_mode
    };

    // CreateDevice TAP suppression: force D3D hooks to pass-through while active.
    // Check if R14 has risen above CreateDevice's entry R14 (= function returned).
    let hle_mode = if CREATEDEVICE_TAP_ACTIVE.load(Ordering::Relaxed) {
        let saved_r14 = CREATEDEVICE_TAP_R14.load(Ordering::Relaxed);
        if context.R14 as u32 > saved_r14 {
            // CreateDevice returned — stack unwound past entry point
            CREATEDEVICE_TAP_ACTIVE.store(false, Ordering::Relaxed);
            CREATEDEVICE_TAP_COMPLETED.store(true, Ordering::Relaxed);
            debug_log(&format!(
                "[TAP] CreateDevice returned (R14=0x{:08X} > saved=0x{:08X}) — re-enabling hooks, dev[0]=NV2A base locked",
                context.R14 as u32, saved_r14
            ));
            hle_mode // use normal mode
        } else if hle_name != "Direct3D_CreateDevice"
            // KickOff MUST stay HLE'd even during CreateDevice — it does the
            // PB spinlock breaker (GET=PUT sync) that prevents polling loops.
            && hle_name != "D3DDevice_KickOff"
            && hle_name != "D3D_KickOffAndWaitForIdle"
            // SDK fence/wait helpers are safe to complete in HLE. Letting these
            // spin natively only waits on NV2A state we already model at the
            // HLE boundary.
            && hle_name != "D3D_SetFence"
            && hle_name != "D3DDevice_InsertFence"
            && hle_name != "D3DDevice_IsFencePending"
            && hle_name != "D3DDevice_BlockOnFence"
            && hle_name != "D3D_BlockOnFence"
            && hle_name != "D3D_BlockOnTime"
            && hle_name != "D3D_BlockOnResource"
            // Classic Doom polls this helper immediately after draw/setup work.
            // Passing it through leaves the guest waiting on native D3D queue
            // pointers that our HLE path has already consumed, so it spins at
            // 0x5A590 forever. Treat it like the fence helpers above.
            && hle_name != "Doom_D3D_IsBusy"
            // BeginPush/EndPush MUST stay HLE'd even during CreateDevice so the
            // pushbuffer parser sees the XDK's initialization geometry. HLE
            // handlers still update dev PUT/GET so native XDK state stays sane.
            // Snoop during CreateDevice is essential: that's where the initial
            // NV2A state + vertex format + shader constants get programmed.
            && hle_name != "D3DDevice_BeginPush"
            && hle_name != "D3DDevice_EndPush"
            // RtlAllocateHeap/RtlFreeHeap are CRT/kernel compatibility shims,
            // not D3D state hooks. Suppressing them during CreateDevice lets
            // Spider-Man fall back into its partially initialized native heap
            // and trip the CRT _amsg_exit/KeBugCheck path.
            && hle_name != "RtlAllocateHeap"
            && hle_name != "RtlFreeHeap"
        {
            // Still inside CreateDevice — force TAP for all other D3D hooks
            static TAP_SUPPRESS_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = TAP_SUPPRESS_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 20 {
                debug_log(&format!(
                    "[TAP-SUPPRESS] {} at 0x{:08X} → pass-through (inside CreateDevice)",
                    name, guest_addr
                ));
            }
            HleMode::Tap
        } else {
            hle_mode
        }
    } else {
        hle_mode
    };

    // Reset TAP suppression: force D3D hooks to pass-through while Reset runs.
    // Auto-deactivate after 30 hooks OR when R14 rises above saved value.
    let hle_mode = if RESET_TAP_ACTIVE.load(Ordering::Relaxed) {
        let saved_r14 = RESET_TAP_R14.load(Ordering::Relaxed);
        let hook_n = RESET_TAP_HOOK_COUNT.fetch_add(1, Ordering::Relaxed);
        if context.R14 as u32 > saved_r14 || hook_n >= 30 {
            // Reset returned (R14 check) or enough hooks passed (timeout)
            RESET_TAP_ACTIVE.store(false, Ordering::Relaxed);
            debug_log(&format!(
                "[TAP] Reset completed (R14=0x{:08X} saved=0x{:08X} hooks={}) — re-enabling hooks",
                context.R14 as u32, saved_r14, hook_n
            ));
            hle_mode
        } else if hle_name != "D3DDevice_Reset"
            && hle_name != "D3DDevice_KickOff"
            && hle_name != "D3D_KickOffAndWaitForIdle"
            && hle_name != "D3DDevice_Clear"
            && hle_name != "D3D_SetFence"
            && hle_name != "D3DDevice_InsertFence"
            && hle_name != "D3DDevice_IsFencePending"
            && hle_name != "D3DDevice_BlockOnFence"
            && hle_name != "D3D_BlockOnFence"
            && hle_name != "D3D_BlockOnTime"
            && hle_name != "D3D_BlockOnResource"
            && hle_name != "RtlAllocateHeap"
            && hle_name != "RtlFreeHeap"
            // Same as CreateDevice gate: let pushbuffer snoops see Reset's
            // state-rebuild geometry too.
            && hle_name != "D3DDevice_BeginPush"
            && hle_name != "D3DDevice_EndPush"
        {
            static RESET_TAP_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RESET_TAP_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 30 {
                debug_log(&format!(
                    "[TAP-SUPPRESS] {} at 0x{:08X} → pass-through (inside Reset, hook#{})",
                    name, guest_addr, hook_n
                ));
            }
            HleMode::Tap
        } else {
            hle_mode
        }
    } else {
        hle_mode
    };

    if hle_mode != HleMode::Hle {
        match hle_name {
            "D3DDevice_SetTextureStageState"
            | "D3DDevice_SetTextureStageStateNotInline"
            | "D3DDevice_SetTextureStageStateNotInline2"
            | "D3D_CDevice_SetTextureStageStateNotInline" => {
                oovpa_hle::hle_note_texture_stage_state(
                    "SetTextureStageState/tap-snoop",
                    args[0],
                    args[1],
                    args[2],
                );
            }
            "D3DDevice_SetTextureState_TexCoordIndex" => {
                oovpa_hle::hle_note_texture_stage_state(
                    "SetTextureState_TexCoordIndex/tap-snoop",
                    args[0],
                    28,
                    args[1],
                );
            }
            "D3DDevice_SetTextureState_BorderColor" => {
                oovpa_hle::hle_note_texture_stage_state(
                    "SetTextureState_BorderColor/tap-snoop",
                    args[0],
                    29,
                    args[1],
                );
            }
            "D3DDevice_SetTextureState_ColorKeyColor" => {
                oovpa_hle::hle_note_texture_stage_state(
                    "SetTextureState_ColorKeyColor/tap-snoop",
                    args[0],
                    30,
                    args[1],
                );
            }
            "D3DDevice_SetShaderConstantMode" => {
                let stack_mode = unsafe { *((r15 + (r14 + 4) as u64) as *const u32) };
                let mode = if argc > 0 { args[0] } else { stack_mode };
                oovpa_hle::hle_note_shader_constant_mode("tap-snoop", mode);
            }
            _ => {}
        }
    }

    // Spider-Man: the update-dispatch entry hook is not an SDK boundary.
    // In HLE mode, do NOT return 0 and do NOT synthetic-write scene globals.
    // Recreate the native prologue stack, then resume at the real callback loop:
    //
    //   0003AC20 push esi
    //   0003AC21 mov  esi,[ecx+8]
    //   0003AC24 push edi
    //   0003AC25 mov  edi,[ecx+0xc]
    //   ...
    //   0003AC30 mov  ecx,[esi]
    //   0003AC32 mov  eax,[ecx]
    //   0003AC34 call [eax+0x20]
    //
    // The older route jumped directly to the first callback, so RET returned
    // to 0xF2623 and skipped the remaining update objects forever.
    if hle_mode == HleMode::Hle
        && name == "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP"
        && title_patches_enabled()
    {
        fn valid_guest_ptr(addr: u32) -> bool {
            addr >= 0x1000 && addr < 0x2000_0000
        }
        fn read_guest_u32(r15: u64, addr: u32) -> u32 {
            if valid_guest_ptr(addr) {
                unsafe { *((r15 + addr as u64) as *const u32) }
            } else {
                0
            }
        }
        fn write_guest_u32(r15: u64, addr: u32, value: u32) {
            if valid_guest_ptr(addr) {
                unsafe {
                    *((r15 + addr as u64) as *mut u32) = value;
                }
            }
        }

        let update_this = context.Rcx as u32;
        let begin = read_guest_u32(r15, update_this.wrapping_add(0x08));
        let end = read_guest_u32(r15, update_this.wrapping_add(0x0C));
        let slots = if valid_guest_ptr(begin) && valid_guest_ptr(end) && end >= begin {
            ((end - begin) / 4).min(64)
        } else {
            0
        };
        let first_obj = read_guest_u32(r15, begin);
        let first_vt = read_guest_u32(r15, first_obj);
        let first_cb = read_guest_u32(r15, first_vt.wrapping_add(0x20));

        static SPIDEY_UPDATE_ROUTE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let route_n = SPIDEY_UPDATE_ROUTE_LOG.fetch_add(1, Ordering::Relaxed);
        if route_n < 128 || route_n.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-UPDATE-CALL] #{} mode=native-loop this=0x{:08X} list=0x{:08X}..0x{:08X} slots={} first_obj=0x{:08X} first_vt=0x{:08X} first_cb20=0x{:08X} ret=0x{:08X} esp=0x{:08X}",
                route_n,
                update_this,
                begin,
                end,
                slots,
                first_obj,
                first_vt,
                first_cb,
                ret_addr,
                r14
            ));
        }

        if slots != 0 {
            let normalized = crate::xbox::emulator::normalize_guest_addr(0x0003_AC30);
            if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
                let new_r14 = (r14 as u32).wrapping_sub(8);
                write_guest_u32(r15, new_r14, context.Rdi as u32);
                write_guest_u32(r15, new_r14.wrapping_add(4), context.Rsi as u32);
                context.R14 = new_r14 as u64;
                context.Rsi = begin as u64;
                context.Rdi = end as u64;
                context.Rip = code_base + host_off as u64;
                return true;
            }

            let saved_shadow = ctx.guest.shadow_stack_top;
            unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
            ctx.guest.shadow_stack_top = saved_shadow;
            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
            ctx.guest.exit_guest_addr = normalized;
            context.Rip = ctx.guest.exit_addr;
            return true;
        }

        // Empty/invalid list: behave like the native empty-path return.
        debug_log("[SPIDEY-UPDATE-CALL] empty/invalid list; returning to caller");
        context.R14 += 4;
        context.Rax = 0;
        let normalized = crate::xbox::emulator::normalize_guest_addr(ret_addr);
        if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
            context.Rip = code_base + host_off as u64;
        } else {
            let saved_shadow = ctx.guest.shadow_stack_top;
            unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
            ctx.guest.shadow_stack_top = saved_shadow;
            ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
            ctx.guest.exit_guest_addr = normalized;
            context.Rip = ctx.guest.exit_addr;
        }
        return true;
    }

    if hle_mode == HleMode::Tap && name == "SPIDEY_SCENE118_LOOP_TAP" {
        let base = ctx.guest_mem_base;
        let scene_phys = crate::xbox::emulator::normalize_guest_addr(context.Rsi as u32);
        let latch_addr = scene_phys.wrapping_add(0x118);
        let latch = raw_read_u32(base, latch_addr);
        let mut obj_addr = 0u32;
        let mut signal_state = 0u32;
        let mut tracked_thread = false;
        if latch != 0 && !ctx.kernel_state.is_null() {
            let state = unsafe { &*(ctx.kernel_state as *const crate::xbox::kernel::KernelState) };
            if let Some(obj) = state.object_table.get(&latch) {
                obj_addr = obj.guest_obj_addr;
                tracked_thread = obj.obj_type == crate::xbox::kernel::XboxObjectType::Thread;
                signal_state = raw_read_u32(base, obj_addr.wrapping_add(0x04));
            }
        }

        static SCENE118_LOOP_OBS_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = SCENE118_LOOP_OBS_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-SCENE118-LOOP-OBS] #{} scene=0x{:08X} latch=0x{:08X} tracked_thread={} obj=0x{:08X} signal={} ret=0x{:08X} esp=0x{:08X}",
                n, scene_phys, latch, tracked_thread, obj_addr, signal_state, ret_addr, r14
            ));
        }

        // This tap sits on sub_1AE60's loop body:
        //   mov edx, [esi+0x118]
        //
        // The entry tap can miss the real completion because the loader thread
        // often signals while sub_1AE60 is already spinning. Clearing the latch
        // here preserves the native wait semantics: the guest still sees the
        // thread handle until the tracked object is signaled, then the original
        // MOV reads zero and the helper exits normally.
        if latch != 0
            && tracked_thread
            && signal_state != 0
            && valid_guest_string_ptr(latch_addr)
            && std::env::var_os("RUSTEMU_SPIDEY_CLEAR_SCENE118_LATCH").is_some()
        {
            unsafe {
                *((base as u64 + latch_addr as u64) as *mut u32) = 0;
            }
            static SCENE118_LOOP_CLEAR_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let clear_n = SCENE118_LOOP_CLEAR_LOG.fetch_add(1, Ordering::Relaxed);
            if clear_n < 32 || clear_n.is_power_of_two() {
                debug_log(&format!(
                    "[SPIDEY-SCENE118-LOOP-CLEAR] #{} scene=0x{:08X} cleared_latch=0x{:08X} obj=0x{:08X} signal={} ret=0x{:08X}",
                    clear_n, scene_phys, latch, obj_addr, signal_state, ret_addr
                ));
            }
        }
    }

    if hle_mode == HleMode::Tap && name == "SPIDEY_LEGAL_GUARD_ENTRY_TAP" {
        fn legal_guard_guest_ptr(addr: u32) -> bool {
            (0x1000..0x2000_0000).contains(&addr)
        }
        fn legal_guard_read_u32(base: u64, addr: u32) -> u32 {
            if legal_guard_guest_ptr(addr) {
                unsafe { *((base + addr as u64) as *const u32) }
            } else {
                0
            }
        }
        let base = ctx.guest_mem_base as u64;
        let scene = legal_guard_read_u32(base, 0x003F_5BEC);
        let scene_phys = crate::xbox::emulator::normalize_guest_addr(scene);
        let latch = if legal_guard_guest_ptr(scene_phys) {
            legal_guard_read_u32(base, scene_phys.wrapping_add(0x118))
        } else {
            0
        };
        let mut obj_addr = 0u32;
        let mut signal_state = 0u32;
        let mut tracked_thread = false;
        if latch != 0 && !ctx.kernel_state.is_null() {
            let state = unsafe { &*(ctx.kernel_state as *const crate::xbox::kernel::KernelState) };
            if let Some(obj) = state.object_table.get(&latch) {
                obj_addr = obj.guest_obj_addr;
                tracked_thread = obj.obj_type == crate::xbox::kernel::XboxObjectType::Thread;
                signal_state = legal_guard_read_u32(base, obj_addr.wrapping_add(0x04));
            }
        }

        static LEGAL_GUARD_OBS_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = LEGAL_GUARD_OBS_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 8 || n.is_power_of_two() {
            debug_log(&format!(
                "[SPIDEY-LEGAL-GUARD-OBS] #{} scene=0x{:08X} latch=0x{:08X} tracked_thread={} obj=0x{:08X} signal={} ret=0x{:08X} esp=0x{:08X}",
                n, scene_phys, latch, tracked_thread, obj_addr, signal_state, ret_addr, r14
            ));
        }
        // F0690 scene-load latch completion:
        //
        // sub_1AE60 returns true only when scene+0x118 becomes zero. The
        // field stores a worker-thread handle while an async scene load is in
        // flight. Our HLE object can already be signaled, but the guest scene
        // field still contains the stale handle, so the wait loop spins forever
        // at the ret=0xF0C35 call site. Clear the latch only after the tracked
        // thread object is signaled; clearing it earlier destroys the handle
        // before the guest can wait on it.
        if latch != 0
            && tracked_thread
            && signal_state != 0
            && ret_addr == 0x000F_0C35
            && legal_guard_guest_ptr(scene_phys.wrapping_add(0x118))
            && std::env::var_os("RUSTEMU_SPIDEY_CLEAR_SCENE118_LATCH").is_some()
        {
            static SCENE_LATCH_CLEAR_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            unsafe {
                *((base + scene_phys as u64 + 0x118) as *mut u32) = 0;
            }
            let clear_n = SCENE_LATCH_CLEAR_LOG.fetch_add(1, Ordering::Relaxed);
            if clear_n < 16 || clear_n.is_power_of_two() {
                debug_log(&format!(
                    "[SPIDEY-SCENE118-CLEAR] #{} scene=0x{:08X} cleared_latch=0x{:08X} obj=0x{:08X} signal={} ret=0x{:08X}",
                    clear_n, scene_phys, latch, obj_addr, signal_state, ret_addr
                ));
            }
        }

        // Gate 2 mechanism (canonical cca32cc / Gate-2-PASS baseline):
        // After ≥10 observations of signal==0, write signal=1 to [obj_addr+0x04].
        // This unblocks KeWaitForSingleObject on the legal thread sync object,
        // advancing state_val 1→3 and letting F28C5 fire naturally.
        //
        // NOTE: Do NOT write [scene+0x118]=0 in this early signal-forcing path.
        // The latch clear above is deliberately later and requires the tracked
        // thread object to already be signaled.
        if latch != 0 && signal_state == 0 && obj_addr != 0 && n >= 10 && title_patches_enabled() {
            if legal_guard_guest_ptr(obj_addr.wrapping_add(0x04)) {
                unsafe {
                    *((base + obj_addr as u64 + 4) as *mut u32) = 1;
                }
                debug_log(&format!(
                    "[SPIDEY-LEGAL-GUARD-FIRE] #{} wrote signal=1 to obj=0x{:08X}+0x04 latch=0x{:08X}",
                    n, obj_addr, latch
                ));
            }
        } else if latch != 0 && signal_state == 0 && obj_addr != 0 && n == 10 {
            debug_log(&format!(
                "[SPIDEY-LEGAL-GUARD-SKIP] #{} title_patches=0 would_signal obj=0x{:08X}+0x04 latch=0x{:08X}",
                n, obj_addr, latch
            ));
        }
    }

    if hle_mode == HleMode::Hle {
        let _cpu_phase = crate::xbox::profiler::guard(crate::xbox::profiler::CpuPhase::OovpaHle);
        let hle_name_static: &'static str =
            unsafe { std::mem::transmute::<&str, &'static str>(hle_name) };
        let _hle_subphase = crate::xbox::profiler::hle_name_guard(hle_name_static);
        // GUARD: Ensure dev+8 flags stay 0x13 (something keeps zeroing them).
        // This is defensive — ideally we'd find the root cause, but until then
        // re-assert flags before every HLE call so the game doesn't spin.
        {
            let gp = super::G_PDEVICE_DYNAMIC.load(std::sync::atomic::Ordering::Relaxed);
            if gp != 0 {
                let dev_ptr = unsafe { *((ctx.guest_mem_base as u64 + gp as u64) as *const u32) };
                if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                    let flags_addr = (ctx.guest_mem_base as u64 + dev_ptr as u64 + 8) as *mut u32;
                    let flags = unsafe { *flags_addr };
                    if flags != 0x13 && dev_ptr == 0x00300E00 {
                        unsafe {
                            *flags_addr = 0x13;
                        }
                    }
                }
            }
        }

        // --------------------------------------------------------------
        // PHASE 5 · Matched-transition push (HLE boundary).
        // Captures the R14 at HLE entry, the hook name (static lifetime
        // from OovpaPattern so &'static str is safe here), and argc.
        // Pop+verify fires immediately after stdcall cleanup below.
        // Log-only — zero behavioral effect.
        // --------------------------------------------------------------
        let transition_pushed = {
            use crate::xbox::aot::transition;
            // Safety: hle_name comes from OovpaPattern or known_argc table,
            // both of which store &'static str literals. is_manual-routed
            // names also come from manual_names literals.
            transition::push(transition::TransitionFrame::new(
                if is_manual {
                    transition::TransitionKind::Manual
                } else {
                    transition::TransitionKind::Hle
                },
                hle_name_static,
                context.R14 as u32,
                guest_addr,
                ret_addr,
                argc as u8,
            ));
            true
        };

        // Execute HLE stub. Replacing an x86 SDK/middleware function with HLE
        // still has to obey the guest ABI: EBX/EBP/ESI/EDI are callee-saved.
        let saved_rbx = context.Rbx;
        let saved_rbp = context.Rbp;
        let saved_rsi = context.Rsi;
        let saved_rdi = context.Rdi;
        let result = if hle_name == "SPIDEY_XGRAPH_READY_CB_HLE" {
            static XGRAPH_READY_HLE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            static XGRAPH_MENU_ZERO_COUNT: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let force_ready = title_patches_enabled()
                && std::env::var_os("RUSTEMU_SPIDEY_FORCE_XGRAPH_READY").is_some();
            let scene_mgr = unsafe { *((r15 + 0x004B_C614) as *const u32) };
            let frame_ctx = unsafe { *((r15 + 0x004B_C630) as *const u32) };
            let xgraph = unsafe { *((r15 + 0x004C_06B8) as *const u32) };
            let xgraph134 = if (0x1000..0x2000_0000).contains(&xgraph) {
                unsafe { *((r15 + xgraph.wrapping_add(0x134) as u64) as *const u32) }
            } else {
                0
            };
            let n = XGRAPH_READY_HLE_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() || force_ready {
                let scene18c = if (0x1000..0x2000_0000).contains(&scene_mgr) {
                    unsafe { *((r15 + scene_mgr.wrapping_add(0x18C) as u64) as *const u8) }
                } else {
                    0
                };
                let frame20 = if (0x1000..0x2000_0000).contains(&frame_ctx) {
                    unsafe { *((r15 + frame_ctx.wrapping_add(0x20) as u64) as *const u32) }
                } else {
                    0
                };
                let scene_name = read_guest_cstr_raw(r15 as *mut u8, 0x004B_C848, 64);
                let menu_zero_ready_base = scene_name.eq_ignore_ascii_case("bonus\\menu")
                    && (0x1000..0x2000_0000).contains(&scene_mgr)
                    && (0x1000..0x2000_0000).contains(&frame_ctx)
                    && (0x1000..0x2000_0000).contains(&xgraph)
                    && xgraph134 == context.Rcx as u32
                    && frame20 == 0;
                let menu_zero_count = if menu_zero_ready_base {
                    XGRAPH_MENU_ZERO_COUNT.fetch_add(1, Ordering::Relaxed)
                } else {
                    XGRAPH_MENU_ZERO_COUNT.store(0, Ordering::Relaxed);
                    0
                };
                // Keep the zero-countdown tracking as a diagnostic only. Forcing
                // this callback ready by default makes F8580 take the early-exit
                // path and prevents the selector circle/focus loop from drawing.
                let would_auto_ready = menu_zero_ready_base && menu_zero_count >= 1;
                debug_log(&format!(
                    "[SPIDEY-XGRAPH-READY-CB #{}] this=0x{:08X} vt=0x{:08X} scene=0x{:08X} scene18c={} frame=0x{:08X} frame20=0x{:08X} xgraph=0x{:08X} xgraph134=0x{:08X} scene_name='{}' menu_zero_count={} would_auto={} force={} ret={}",
                    n,
                    context.Rcx as u32,
                    if (0x1000..0x2000_0000).contains(&(context.Rcx as u32)) {
                        unsafe { *((r15 + context.Rcx as u64) as *const u32) }
                    } else {
                        0
                    },
                    scene_mgr,
                    scene18c,
                    frame_ctx,
                    frame20,
                    xgraph,
                    xgraph134,
                    scene_name,
                    menu_zero_count,
                    if would_auto_ready { 1 } else { 0 },
                    if force_ready { 1 } else { 0 },
                    if force_ready { 1 } else { 0 },
                ));
            }
            let scene_name = read_guest_cstr_raw(r15 as *mut u8, 0x004B_C848, 64);
            let scene18c = if (0x1000..0x2000_0000).contains(&scene_mgr) {
                unsafe { *((r15 + scene_mgr.wrapping_add(0x18C) as u64) as *const u8) }
            } else {
                0
            };
            let frame20 = if (0x1000..0x2000_0000).contains(&frame_ctx) {
                unsafe { *((r15 + frame_ctx.wrapping_add(0x20) as u64) as *const u32) }
            } else {
                u32::MAX
            };
            let menu_zero_ready_base = scene_name.eq_ignore_ascii_case("bonus\\menu")
                && (0x1000..0x2000_0000).contains(&scene_mgr)
                && (0x1000..0x2000_0000).contains(&frame_ctx)
                && (0x1000..0x2000_0000).contains(&xgraph)
                && xgraph134 == context.Rcx as u32
                && frame20 == 0;
            let menu_zero_count = XGRAPH_MENU_ZERO_COUNT.load(Ordering::Relaxed);
            let _would_auto_ready = menu_zero_ready_base && menu_zero_count >= 2;
            if force_ready {
                1
            } else {
                let _ = scene18c;
                0
            }
        } else if hle_name == "XapiMapLetterToDirectory" && !ctx.kernel_state.is_null() {
            let state =
                unsafe { &mut *(ctx.kernel_state as *mut crate::xbox::kernel::KernelState) };
            xapi_map_letter_to_directory(state, ctx.guest_mem_base, &args, "veh")
        } else {
            oovpa_hle::execute_hle(
                hle_name,
                &args,
                argc,
                context,
                ctx.guest_mem_base,
                &mut ctx.mmio_count,
                &mut ctx.pb_commands,
                &mut ctx.draw_calls,
                is_manual,
                guest_addr,
            )
        };
        if context.Rbx != saved_rbx
            || context.Rbp != saved_rbp
            || context.Rsi != saved_rsi
            || context.Rdi != saved_rdi
        {
            static HLE_ABI_RESTORE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = HLE_ABI_RESTORE_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 40 {
                debug_log(&format!(
                    "[HLE-ABI-RESTORE] {} @0x{:08X} EBX {:08X}->{:08X} EBP {:08X}->{:08X} ESI {:08X}->{:08X} EDI {:08X}->{:08X}",
                    name,
                    guest_addr,
                    saved_rbx as u32,
                    context.Rbx as u32,
                    saved_rbp as u32,
                    context.Rbp as u32,
                    saved_rsi as u32,
                    context.Rsi as u32,
                    saved_rdi as u32,
                    context.Rdi as u32
                ));
            }
        }
        context.Rbx = saved_rbx;
        context.Rbp = saved_rbp;
        context.Rsi = saved_rsi;
        context.Rdi = saved_rdi;
        let _ = transition_pushed; // (silences unused warning in cfg variants)

        // Log result for first N calls (completes the [D3D] call→result trace)
        if cc <= 10 {
            debug_log(&format!("[D3D] #{} {} → 0x{:08X}", cc, name, result));
        }

        // FLAG WATCHPOINT: detect who zeroes dev+8 after CreateDevice sets it to 0x13
        {
            use std::sync::atomic::{AtomicU32, Ordering as AO};
            static FLAG_WATCH: AtomicU32 = AtomicU32::new(0);
            let gp = super::G_PDEVICE_DYNAMIC.load(AO::Relaxed);
            if gp != 0 {
                let dev_ptr = unsafe { *((ctx.guest_mem_base as u64 + gp as u64) as *const u32) };
                if dev_ptr != 0 && dev_ptr < 0x1000_0000 {
                    let flags = unsafe {
                        *((ctx.guest_mem_base as u64 + dev_ptr as u64 + 8) as *const u32)
                    };
                    let prev = FLAG_WATCH.load(AO::Relaxed);
                    if flags != prev {
                        debug_log(&format!("[FLAG-WATCH] dev+8 changed: 0x{:08X} → 0x{:08X} after {} (#{} @ 0x{:08X})",
                            prev, flags, name, cc, guest_addr));
                        FLAG_WATCH.store(flags, AO::Relaxed);
                    }
                }
            }
        }

        // Stdcall cleanup: pop ret_addr + args from guest stack (R14)
        let r14_before = context.R14 as u32;
        context.R14 += (4 + argc as u64 * 4) as u64;
        context.Rax = result as u64;

        // --------------------------------------------------------------
        // PHASE 5 · Matched-transition pop+verify (HLE boundary).
        // The TransitionFrame's expected_r14_on_leave = entry + 4 + argc*4,
        // matching our cleanup above exactly. Delta should be 0 every time
        // unless the HLE handler itself modified R14 mid-execution (which
        // would be an HLE-handler bug).
        // --------------------------------------------------------------
        {
            use crate::xbox::aot::transition;
            let _ = transition::pop_and_verify(None, context.R14 as u32);
        }

        // ESP imbalance detector: log every HLE call's R14 adjustment during first frame.
        // If any function has wrong argc, R14 will drift and eventually cause RET_TO_ZERO.
        {
            // PHASE 1 of matched-transition plan (2026-04-20):
            // Cap raised 200→10000 so we capture ESP deltas THROUGH the
            // font-loader chain at dispatch #1400+. A consistent +4 delta
            // per function is correct (stdcall 0-arg cleanup = +4 for ret_addr).
            // A function whose R14 delta is inconsistent between calls is a
            // latent argc-mismatch drift source — that's what we're hunting.
            static ESP_TRACK: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = ESP_TRACK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 10_000 {
                debug_log(&format!(
                    "[ESP-TRACK] #{} {} argc={} R14: 0x{:08X} → 0x{:08X} (+{}) ret=0x{:08X}",
                    n,
                    name,
                    argc,
                    r14_before,
                    context.R14 as u32,
                    4 + argc as u32 * 4,
                    ret_addr
                ));
            }
        }

        // Trace post-dispatch state for CreateDevice internals
        if name == "D3DDevice_SetSoftDisplayFilter" || name == "D3DDevice_SetFlickerFilter" {
            debug_log(&format!(
                "[HLE-RETURN] {} → R14=0x{:08X} ret=0x{:08X} argc={} cleanup={}",
                name,
                context.R14 as u32,
                ret_addr,
                argc,
                4 + argc as u32 * 4
            ));
        }

        // Try to jump directly to the host code for ret_addr (stay in trampoline).
        // This avoids exiting to the dispatch loop for every HLE call.
        // R12 pop DISABLED — emit_call_rel no longer pushes to R12.

        let normalized = crate::xbox::emulator::normalize_guest_addr(ret_addr);
        if let Some(host_off) = ctx.addr_hash.lookup(normalized) {
            context.Rip = code_base + host_off as u64;
            return true;
        }

        // Fallback: exit trampoline for dispatch loop to resolve ret_addr.
        // Sync guest registers BUT preserve shadow_stack_top — the OOVPA
        // HLE changed R12 (stdcall cleanup popped args) and syncing it
        // would overwrite the shadow stack position from the kernel handler,
        // causing the VLAN hop (R12 jumps to wrong position at next entry).
        let saved_shadow = ctx.guest.shadow_stack_top;
        unsafe { crate::xbox::aot::veh::sync_guest_from_context(ctx, context) };
        ctx.guest.shadow_stack_top = saved_shadow; // preserve R12 position
        ctx.guest.eax = result;
        ctx.guest.esp = context.R14 as u32;
        ctx.guest.exit_reason = crate::xbox::aot::runtime::AOT_EXIT_UNRESOLVED;
        ctx.guest.exit_guest_addr = normalized;
        context.Rip = ctx.guest.exit_addr;
        return true;
    }

    // Reframed E Probe A — TAP-mode probe at DPC body entry 0x002FBF10.
    // When RUSTEMU_DPC_BODY_PROBE=1 plants this hook, every entry into the
    // DPC body gets an instrumented log line BEFORE the body runs natively.
    // We dump GPRs and dereference [esi+0x100] (NV_PMC_INTR_0 MMIO) and
    // [esi+0x108] from the NV2A shadow. esi is the dereference of the KDPC
    // DeferredContext (CMiniport pointer) which holds 0xFD000000 at +0x0,
    // so [esi+0x100] is the PMC_INTR aggregate the body branches on.
    if name == "DPC_BODY_PROBE_ENTRY" {
        use std::sync::atomic::{AtomicU32, Ordering};
        static PROBE_COUNT: AtomicU32 = AtomicU32::new(0);
        let n = PROBE_COUNT.fetch_add(1, Ordering::Relaxed);
        if n < 200 {
            let eax = context.Rax as u32;
            let ecx = context.Rcx as u32;
            let edx = context.Rdx as u32;
            let ebx = context.Rbx as u32;
            let esi = context.Rsi as u32;
            let edi = context.Rdi as u32;
            let ebp = context.Rbp as u32;
            // At probe entry registers are pre-prolog and often stale from the
            // scheduler. The DPC body itself starts with:
            //   mov ebx, [esp+0x0c]
            //   mov ebp, [ebx]
            //   mov esi, [ebp+0x100]
            // so dump the stack slots too; they are the authoritative call
            // frame for this routine.
            let pmc_intr = crate::xbox::aot::nv2a::shadow_read(0x100);
            let pmc_intr_en = crate::xbox::aot::nv2a::shadow_read(0x140);
            let pcrtc_intr = crate::xbox::aot::nv2a::shadow_read(0x60_0100);
            let pfifo_intr = crate::xbox::aot::nv2a::shadow_read(0x2100);
            let pgraph_intr = crate::xbox::aot::nv2a::shadow_read(0x40_0100);
            // Try to deref the KDPC's DeferredContext at [ecx+0x10] (XDK
            // KDPC layout). If ecx looks like a guest RAM ptr, follow it.
            let mut def_ctx = 0u32;
            let mut ctx_field_0 = 0u32;
            if ecx != 0 && ecx < 0x2000_0000 {
                def_ctx = unsafe { *((context.R15 as u64 + ecx as u64 + 0x10) as *const u32) };
                if def_ctx != 0 && def_ctx < 0x2000_0000 {
                    ctx_field_0 = unsafe { *((context.R15 as u64 + def_ctx as u64) as *const u32) };
                }
            }
            let esp = context.R14 as u32;
            let read_guest_u32 = |addr: u32| -> u32 {
                if addr < 0x2000_0000 {
                    unsafe { *((context.R15 as u64 + addr as u64) as *const u32) }
                } else {
                    0
                }
            };
            let st0 = read_guest_u32(esp);
            let st4 = read_guest_u32(esp.wrapping_add(4));
            let st8 = read_guest_u32(esp.wrapping_add(8));
            let stc = read_guest_u32(esp.wrapping_add(0x0c));
            let st10 = read_guest_u32(esp.wrapping_add(0x10));
            let st14 = read_guest_u32(esp.wrapping_add(0x14));
            // The native routine begins with `push ebx`, then reads
            // `mov ebx, [esp+0x0c]`. At probe entry that corresponds to the
            // original caller stack slot at [esp+0x08].
            let body_ebx = st8;
            let body_ebp = read_guest_u32(body_ebx);
            let body_vblank_cb = read_guest_u32(body_ebx.wrapping_add(0x1C4));
            let body_frame_event = read_guest_u32(body_ebx.wrapping_add(0x1C8));
            let body_esi = if (0xFD00_0000..0xFE00_0000).contains(&body_ebp) {
                crate::xbox::aot::nv2a::shadow_read(body_ebp.wrapping_sub(0xFD00_0000) + 0x100)
            } else if body_ebp < 0x2000_0000 {
                read_guest_u32(body_ebp.wrapping_add(0x100))
            } else {
                0
            };
            crate::xbox::emulator::debug_log(&format!(
                "[DPC-BODY-PROBE] #{} EAX=0x{:08X} ECX=0x{:08X} EDX=0x{:08X} EBX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X} EBP=0x{:08X} \
                 ESP=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] \
                 body_ebx=[entry_esp+0x08]=0x{:08X} body_ebp=[body_ebx]=0x{:08X} body_esi=[body_ebp+0x100]=0x{:08X} body_vblank_cb=[body_ebx+0x1C4]=0x{:08X} body_event=[body_ebx+0x1C8]=0x{:08X} \
                 KDPC.DeferredContext=0x{:08X} *DefCtx[0]=0x{:08X} \
                 PMC_INTR=0x{:08X} PMC_INTR_EN=0x{:08X} PCRTC_INTR=0x{:08X} PFIFO_INTR=0x{:08X} PGRAPH_INTR=0x{:08X}",
                n, eax, ecx, edx, ebx, esi, edi, ebp,
                esp, st0, st4, st8, stc, st10, st14,
                body_ebx, body_ebp, body_esi, body_vblank_cb, body_frame_event,
                def_ctx, ctx_field_0,
                pmc_intr, pmc_intr_en, pcrtc_intr, pfifo_intr, pgraph_intr
            ));
        }
    }

    if name == "SPIDEY_VECTOR_GROW_TAP" {
        use std::sync::atomic::{AtomicU32, Ordering};
        static VECTOR_GROW_COUNT: AtomicU32 = AtomicU32::new(0);
        let n = VECTOR_GROW_COUNT.fetch_add(1, Ordering::Relaxed);
        if n < 256 || n.is_power_of_two() {
            let r15 = context.R15;
            let esp = context.R14 as u32;
            let owner = context.Rcx as u32;
            let read_guest_u32 = |addr: u32| -> u32 {
                if addr != 0 && addr < 0x2000_0000 {
                    unsafe { *((r15 + addr as u64) as *const u32) }
                } else {
                    0
                }
            };
            let ret = read_guest_u32(esp);
            let grow_by = read_guest_u32(esp.wrapping_add(4));
            let base = read_guest_u32(owner);
            let used = read_guest_u32(owner.wrapping_add(4));
            let end = read_guest_u32(owner.wrapping_add(8));
            let count = read_guest_u32(owner.wrapping_add(0x0C));
            let selected = read_guest_u32(owner.wrapping_add(0x10));
            let cursor = read_guest_u32(owner.wrapping_add(0x18));
            let chunk = read_guest_u32(owner.wrapping_add(0x1C));
            let requested = used.wrapping_add(grow_by);
            let capacity = end.wrapping_sub(base);
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-VECTOR-GROW] #{} ret=0x{:08X} esp=0x{:08X} owner=0x{:08X} \
                 base=0x{:08X} used=0x{:08X} end=0x{:08X} cap=0x{:08X} count=0x{:08X} \
                 selected=0x{:08X} cursor=0x{:08X} grow_by=0x{:08X} chunk=0x{:08X} \
                 requested=0x{:08X} eax=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                n,
                ret,
                esp,
                owner,
                base,
                used,
                end,
                capacity,
                count,
                selected,
                cursor,
                grow_by,
                chunk,
                requested,
                context.Rax as u32,
                context.Rbx as u32,
                context.Rsi as u32,
                context.Rdi as u32
            ));
        }
    }

    // TAP mode: restore original byte, re-execute, mark for rearm.
    //
    // Bink-specific timing instrumentation (Step 4 of Codex 2026-04-26 plan):
    // when RUSTEMU_NATIVE_BINK=1 places Bink* hooks in TAP mode for native
    // execution, log a wall-clock entry timestamp + per-name call count. The
    // delta between consecutive BinkDoFrame entries is effectively the wall
    // time the native decoder takes per frame; a stuck BinkWait shows up as
    // a long gap before the next entry. Together with BinkCopyToBuffer entry
    // ticks they isolate AOT/perf vs DSound vs D3D-copy as the first blocker.
    if name.starts_with("Bink") {
        use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
        use std::time::Instant;
        static BINK_T0: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
        static LAST_TICK_PER_HOOK: [AtomicU64; 8] = [
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
        ];
        static BINK_CALL_COUNTS: [AtomicU32; 8] = [
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ];
        // Trap-category snapshot from the previous logged BINK-TIMING line —
        // not per-hook. Tracks "what happened in the BINK section since the
        // last time we logged anything Bink-related." Order matches
        // bink_profile::snapshot(): [interp, ret, ind, sys, av].
        static LAST_PROFILE_SNAPSHOT: [AtomicU64; 5] = [
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
            AtomicU64::new(0),
        ];
        // Cheap name → slot map; order matches Step 3 plant set.
        let slot = match name {
            "BinkOpen" => 0,
            "BinkPause" => 1,
            "BinkWait" => 2,
            "BinkDoFrame" => 3,
            "BinkCopyToBuffer" => 4,
            "BinkNextFrame" => 5,
            "BinkClose" => 6,
            _ => 7,
        };
        let t0 = BINK_T0.get_or_init(Instant::now);
        let now_us = t0.elapsed().as_micros() as u64;
        let prev_us = LAST_TICK_PER_HOOK[slot].swap(now_us, Ordering::Relaxed);
        let n = BINK_CALL_COUNTS[slot].fetch_add(1, Ordering::Relaxed);
        // Log densely for the first few calls per hook (boot-time char), then
        // every 16th call (steady-state pace). Enough resolution to compute
        // call rate / inter-call delta without flooding the log.
        if n < 8 || (n & 0xF) == 0 {
            let delta_us = if prev_us > 0 {
                now_us.saturating_sub(prev_us)
            } else {
                0
            };
            // Snapshot trap counters and compute delta vs previous logged line.
            // Race: another thread may bump a counter between the 5 swaps;
            // bound is "a few extra increments leak into the next delta",
            // negligible for diagnostic purposes.
            let prof_now = crate::xbox::aot::bink_profile::snapshot();
            let prof_delta = [
                prof_now[0]
                    .saturating_sub(LAST_PROFILE_SNAPSHOT[0].swap(prof_now[0], Ordering::Relaxed)),
                prof_now[1]
                    .saturating_sub(LAST_PROFILE_SNAPSHOT[1].swap(prof_now[1], Ordering::Relaxed)),
                prof_now[2]
                    .saturating_sub(LAST_PROFILE_SNAPSHOT[2].swap(prof_now[2], Ordering::Relaxed)),
                prof_now[3]
                    .saturating_sub(LAST_PROFILE_SNAPSHOT[3].swap(prof_now[3], Ordering::Relaxed)),
                prof_now[4]
                    .saturating_sub(LAST_PROFILE_SNAPSHOT[4].swap(prof_now[4], Ordering::Relaxed)),
            ];
            crate::xbox::emulator::debug_log(&format!(
                "[BINK-TIMING] {} #{} t={}.{:06}s prev_delta={}.{:03}ms \
                 interp=+{} ret=+{} ind=+{} sys=+{} av=+{}",
                name,
                n,
                now_us / 1_000_000,
                now_us % 1_000_000,
                delta_us / 1000,
                delta_us % 1000,
                prof_delta[0],
                prof_delta[1],
                prof_delta[2],
                prof_delta[3],
                prof_delta[4],
            ));
        }
    }
    if hle_mode == HleMode::Tap && name == "XGRPH_AssembleShader" {
        oovpa_hle::xgrph_lle_canary_tap_entry(
            &args,
            ctx.guest_mem_base,
            ret_addr,
            r14,
            [
                context.Rax as u32,
                context.Rbx as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            ],
        );
    }
    if hle_mode == HleMode::Tap && name == "XGRPH_AssembleShader_RET_TAP" {
        oovpa_hle::xgrph_lle_canary_return_tap(
            ctx.guest_mem_base,
            guest_addr,
            ret_addr,
            r14,
            [
                context.Rax as u32,
                context.Rbx as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            ],
        );
    }
    if hle_mode == HleMode::Tap && name == "XGRPH_Lexer_TAP" {
        oovpa_hle::xgrph_lle_canary_lexer_tap(
            ctx.guest_mem_base,
            guest_addr,
            ret_addr,
            r14,
            [
                context.Rax as u32,
                context.Rbx as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            ],
        );
    }
    if hle_mode == HleMode::Tap && name == "XGRPH_Error_TAP" {
        oovpa_hle::xgrph_lle_canary_error_tap(
            ctx.guest_mem_base,
            guest_addr,
            ret_addr,
            r14,
            [
                context.Rax as u32,
                context.Rbx as u32,
                context.Rcx as u32,
                context.Rdx as u32,
                context.Rsi as u32,
                context.Rdi as u32,
            ],
        );
    }
    m.active = false;
    m.pending_rearm = true;
    #[cfg(windows)]
    {
        use windows::Win32::System::Memory::*;
        let target = unsafe { ctx.code_base.add(host_offset as usize) };
        let mut old_prot = PAGE_PROTECTION_FLAGS(0);
        if unsafe { VirtualProtect(target as *const _, 1, PAGE_EXECUTE_READWRITE, &mut old_prot) }
            .is_ok()
        {
            unsafe {
                *target = original_byte;
            }
            let _ = unsafe { VirtualProtect(target as *const _, 1, old_prot, &mut old_prot) };
        }
    }
    context.Rip = code_base + host_offset as u64;
    true
}
