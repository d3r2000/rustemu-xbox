/// OOVPA — Ordered Offset-Value Pair Array pattern matching.
/// Scans guest XBE sections for known SDK functions (D3D8, DSound),
/// plants INT3 hooks at matched addresses, dispatches to HLE stubs.
///
/// Port of C++ AOT_OOVPA.cpp. Two-phase merged scan:
///   Phase 1: 14 hand-tuned patterns (Spider-Man XDK 4134, full HLE)
///   Phase 2: Cxbx-R DB patterns (generic HLE — return D3D_OK)
use crate::xbox::emulator::debug_log;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

// Re-export submodules for external callers
pub use oovpa_dispatch::{
    execute_hle_for_interpreter, tap_active, tap_completed, veh_try_dispatch,
};
pub use oovpa_hooks::{plant_hooks, plant_int3_pub};
pub(crate) use oovpa_scan::infer_g_pdevice_global;
pub(crate) use oovpa_scan::symbol_cache_input_enabled;
pub use oovpa_scan::validate_against_symbol_cache as validate_hooks;
pub use oovpa_scan::{scan, scan_ranges, OovpaScanRange, OovpaScanRangeKind};

// ============================================================================
// Global OOVPA counters — published by HLE hooks, read by main thread for HUD
// ============================================================================
pub static OOVPA_MMIO_COUNT: AtomicU64 = AtomicU64::new(0);
/// XDK D3D8 build version — set during scan(), read during plant_hooks() for manual hook gating.
pub static XDK_BUILD: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);
/// XBE entry point — set during scan(), used to gate game-specific manual hooks.
pub static XBE_ENTRY: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// True when the loaded XBE is the test_blue_padded test harness (title
/// "Blue Padded (4361)"). Set in patches::apply_pre_aot_patches. Used by
/// oovpa_dispatch to switch Direct3D_CreateDevice from TAP passthrough
/// (needed for real games so internal state gets built) to full HLE
/// override (safe here because the test's main is just Clear+Swap in a
/// loop and needs no guest-built D3D internal state).
pub static IS_BLUE_PADDED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub static OOVPA_PB_COMMANDS: AtomicU64 = AtomicU64::new(0);
pub static OOVPA_DRAW_CALLS: AtomicU64 = AtomicU64::new(0);
pub static OOVPA_SWAP_COUNT: AtomicU64 = AtomicU64::new(0);
pub static OOVPA_HOOKS_FIRED: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// Global OOVPA state (accessed from VEH handler)
// ============================================================================

pub(crate) static OOVPA_STATE: Mutex<Option<OovpaState>> = Mutex::new(None);

/// Install OOVPA state for VEH access.
pub fn install_state(state: OovpaState) {
    *OOVPA_STATE.lock().unwrap() = Some(state);
}

/// Remove OOVPA state (called on unload).
pub fn remove_state() -> Option<OovpaState> {
    OOVPA_STATE.lock().unwrap().take()
}

// ============================================================================
// Pattern types
// ============================================================================

/// Single offset-value check within a function prologue.
#[derive(Debug, Clone, Copy)]
pub struct OovpaEntry {
    pub offset: u16,
    pub value: u8,
}

/// A named pattern with its byte checks, scan window, and stdcall arg count.
#[derive(Debug, Clone)]
pub struct OovpaPattern {
    pub name: &'static str,
    pub detect_size: u16, // max offset + 1
    pub entries: &'static [OovpaEntry],
    pub argc: u8, // stdcall arg count (0 = auto-detect from ret N)
    pub hle_mode: HleMode,
    /// Minimum XDK build version for this pattern (0 = match any).
    /// Pattern is only scanned if xbe_build >= min_version.
    pub min_version: u16,
}

/// Cxbx-R-style XREF check attached to an OOVPA pattern.
///
/// The flat generated tables keep only byte checks. Cxbx-R also keeps leading
/// `XREF_ENTRY(offset, symbol)` records and validates the 32-bit operand at
/// `offset` as either a direct address or a PC-relative target. Keep this as
/// side metadata so old generated `OovpaPattern` literals remain usable.
#[derive(Debug, Clone, Copy)]
pub struct OovpaXref {
    pub offset: u16,
    pub target: &'static str,
}

/// Source metadata for one Cxbx-R OOVPA revision.
#[derive(Debug, Clone, Copy)]
pub struct OovpaPatternMeta {
    pub name: &'static str,
    pub min_version: u16,
    pub source_file: &'static str,
    pub xrefs: &'static [OovpaXref],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HleMode {
    Tap, // log and pass through (re-execute original)
    Hle, // execute HLE stub, skip guest code
}

// ============================================================================
// Match state
// ============================================================================

#[derive(Debug, Clone)]
pub struct OovpaMatch {
    pub guest_addr: u32,
    pub host_offset: u32,
    pub original_byte: u8,
    pub pattern_name: &'static str,
    pub argc: u8,
    pub hle_mode: HleMode,
    pub is_cxbx: bool,
    pub is_manual: bool,
    pub active: bool,
    pub pending_rearm: bool,
    pub call_count: u64,
}

/// Result of OOVPA scan + hook planting.
pub struct OovpaState {
    pub matches: Vec<OovpaMatch>,
    pub hooks_planted: bool,
}

impl OovpaState {
    /// Get all matched guest addresses (for interpreter OOVPA detection).
    pub fn guest_addrs(&self) -> Vec<u32> {
        self.matches.iter().map(|m| m.guest_addr).collect()
    }

    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
            hooks_planted: false,
        }
    }
}

/// Query the planted hook mode for interpreter fallback.
/// VEH can re-execute TAP hooks by restoring the original host byte; the
/// interpreter reads guest memory directly, so it should simply pass through
/// TAP addresses instead of yielding to HLE.
pub fn hook_mode_for_guest(guest_addr: u32) -> Option<HleMode> {
    let guard = OOVPA_STATE.lock().ok()?;
    let state = guard.as_ref()?;
    state
        .matches
        .iter()
        .find(|m| m.guest_addr == guest_addr)
        .map(|m| m.hle_mode)
}

pub fn is_tap_hook(guest_addr: u32) -> bool {
    hook_mode_for_guest(guest_addr) == Some(HleMode::Tap)
}

// ============================================================================
// XDK 4134 constants (Spider-Man)
// ============================================================================

/// g_pDevice global address (Spider-Man fallback)
pub const G_PDEVICE: u32 = 0x0030_38E0;
/// Dynamic g_pDevice address — set by CreateDevice HLE, used by all other hooks.
/// Replaces hardcoded G_PDEVICE for XDK-agnostic operation.
pub static G_PDEVICE_DYNAMIC: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Dummy pushbuffer region (512KB — large enough that inline space checks
/// rarely trigger within a single frame's worth of D3D commands).
/// Located at 0x00A0_0000: above XBE sections (~0x003B0000), below contiguous
/// alias (0x00D00000), and below PB_DUMMY_BASE old value (0x00EA0000).
pub const PB_DUMMY_BASE: u32 = 0x00A0_0000;
pub const PB_DUMMY_SIZE: u32 = 0x0008_0000;

// Vertex stream state — captured by SetStreamSource, consumed by DrawVertices
pub(crate) static STREAM0_VB_PTR: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static STREAM0_STRIDE: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

// Device struct offsets — surface table (from Spider-Man XDK 4134 disassembly)
// GetBackBuffer reads [device + index*4 + 0x207C] for back buffer surfaces.
// DO NOT use 0x2070-0x2084 for push buffer — those are surface pointers!
pub const DEV_RT_SURFACE: u32 = 0x2070; // render target surface pointer
pub const DEV_DS_SURFACE: u32 = 0x2074; // depth stencil surface (or color buffer)
pub const DEV_BACKBUF_0: u32 = 0x207C; // back buffer surface 0
pub const DEV_BACKBUF_1: u32 = 0x2080; // back buffer surface 1

// Internal push buffer tracking — stored at 0x2900+ to avoid surface table conflict.
// The REAL guest push buffer uses device+0x00 (pPut) and device+0x04 (pThreshold).
// These offsets are for our HLE's push buffer management only.
pub const DEV_PB_PUT: u32 = 0x2900;
pub const DEV_PB_BASE: u32 = 0x2904;
pub const DEV_PB_GET: u32 = 0x2908;
pub const DEV_PB_LIMIT: u32 = 0x290C;

pub const DEV_VSHADER: u32 = 0x0380;
pub const DEV_FRAME_CTR: u32 = 0x2ABC;

/// Read the device struct pointer via the dynamic g_pDevice global.
/// Falls back to the Spider-Man hardcoded address if dynamic wasn't set.
pub(crate) fn read_dev_ptr(guest_mem: *mut u8) -> u32 {
    let gpd = G_PDEVICE_DYNAMIC.load(std::sync::atomic::Ordering::Relaxed);
    let addr = if gpd != 0 { gpd } else { G_PDEVICE };
    let candidate = unsafe { *((guest_mem as u64 + addr as u64) as *const u32) };
    if candidate >= 0x0001_0000 && candidate < 0x1000_0000 && (candidate & 3) == 0 {
        return candidate;
    }
    if addr != G_PDEVICE {
        let fallback = unsafe { *((guest_mem as u64 + G_PDEVICE as u64) as *const u32) };
        if fallback >= 0x0001_0000 && fallback < 0x1000_0000 && (fallback & 3) == 0 {
            return fallback;
        }
    }
    candidate
}

/// Sync pushbuffer GET = PUT in the device struct.
/// Called from VEH shadow stack overflow handler to break inline pushbuffer
/// spin loops in D3D render state setters (they check space without calling
/// the hooked MakeSpace/KickOff functions).
pub fn sync_pushbuffer_get_eq_put(guest_mem: *mut u8) {
    // During TAP (CreateDevice running natively), don't touch device memory.
    // Native code sets device[0]=NV2A base, pChannel, etc. Our PB_DUMMY_BASE
    // would corrupt the spin-loop condition (device[0] XOR [pChannel+0x44]).
    if oovpa_dispatch::tap_active() {
        return;
    }
    let gpd = G_PDEVICE_DYNAMIC.load(std::sync::atomic::Ordering::Relaxed);
    if gpd == 0 {
        return;
    } // CreateDevice hasn't fired yet
    let dev_ptr = unsafe { *((guest_mem as u64 + gpd as u64) as *const u32) };
    if dev_ptr == 0 || dev_ptr >= 0x1000_0000 {
        return;
    }
    let dev_base = guest_mem as u64 + dev_ptr as u64;
    unsafe {
        // After TAP, dev[0]=NV2A register base (0xFD000000) — don't overwrite
        if !oovpa_dispatch::tap_completed() {
            *((dev_base + 0x00) as *mut u32) = PB_DUMMY_BASE;
        }
        // Always sync the hardware PUT/GET pair at their dedicated offsets
        *((dev_base + DEV_PB_PUT as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_GET as u64) as *mut u32) = PB_DUMMY_BASE;
    }
}

// ============================================================================
// Public query functions
// ============================================================================

/// Get all OOVPA-matched guest addresses (for interpreter inline dispatch).
pub fn get_create_device_addr() -> Option<u32> {
    match OOVPA_STATE.lock() {
        Ok(guard) => match guard.as_ref() {
            Some(state) => state
                .matches
                .iter()
                .find(|m| m.pattern_name == "Direct3D_CreateDevice")
                .map(|m| m.guest_addr),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn get_hooked_guest_addrs() -> Vec<u32> {
    match OOVPA_STATE.lock() {
        Ok(guard) => match guard.as_ref() {
            Some(state) => state.guest_addrs(),
            None => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}

/// Get addresses of _SEH_prolog/_SEH_epilog hooks (interpreter should run these natively)
pub fn get_seh_addrs() -> Vec<u32> {
    match OOVPA_STATE.lock() {
        Ok(guard) => match guard.as_ref() {
            Some(state) => state
                .matches
                .iter()
                .filter(|m| m.pattern_name == "_SEH_prolog" || m.pattern_name == "_SEH_epilog")
                .map(|m| m.guest_addr)
                .collect(),
            None => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}

/// Check if any D3D table init hooks have fired (RenderStateInit/Sort2).
pub fn has_d3d_table_init_fired() -> bool {
    let guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return false,
    };
    state.matches.iter().any(|m| {
        (m.pattern_name == "D3D_RenderStateInit" || m.pattern_name == "D3D_RenderStateSort2")
            && m.call_count > 0
    })
}

/// Check if CreateDevice has already been called organically.
pub fn has_create_device_fired() -> bool {
    let guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return false,
    };
    state
        .matches
        .iter()
        .any(|m| m.pattern_name == "Direct3D_CreateDevice" && m.call_count > 0)
}

/// Get HUD telemetry: (total_hooks_fired, create_device, swap_count, draw_count)
pub fn get_hud_stats() -> (u32, bool, u32, u32) {
    let guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return (0, false, 0, 0),
    };
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return (0, false, 0, 0),
    };
    let mut total = 0u64;
    let mut create_device = false;
    let mut swap = 0u64;
    let mut draw = 0u64;
    for m in &state.matches {
        if m.call_count > 0 {
            total += m.call_count;
            if m.pattern_name == "Direct3D_CreateDevice" || m.pattern_name == "D3D_RenderStateInit"
            {
                create_device = true;
            }
            if m.pattern_name == "D3DDevice_Swap" {
                swap += m.call_count;
            }
            if m.pattern_name == "D3DDevice_DrawIndexedVertices"
                || m.pattern_name == "D3DDevice_DrawVertices"
            {
                draw += m.call_count;
            }
        }
    }
    (total as u32, create_device, swap as u32, draw as u32)
}

/// Dump call count summary.
pub fn dump_summary(matches: &[OovpaMatch]) {
    if matches.is_empty() {
        return;
    }
    debug_log("[OOVPA] === Call Summary ===");
    for m in matches {
        if m.call_count > 0 {
            debug_log(&format!(
                "[OOVPA]   {}: {} calls (guest=0x{:08X})",
                m.pattern_name, m.call_count, m.guest_addr
            ));
        }
    }
}

/// Synthetic D3D injection: force CreateDevice + Swap to bootstrap Stage 9/10.
/// Called from worker dispatch loop after D3D table init phase completes.
pub fn force_synthetic_d3d(ctx: &mut crate::xbox::aot::runtime::RuntimeContext) {
    debug_log("[OOVPA-SYNTH] Forcing synthetic CreateDevice + Swap + Draw counters");
    oovpa_hle::hle_create_device(ctx.guest_mem_base, &mut ctx.mmio_count, 0);
    ctx.pb_commands += 1;
    ctx.draw_calls += 1;

    oovpa_hle::hle_swap(ctx.guest_mem_base, &mut ctx.mmio_count);
    ctx.pb_commands += 1;

    debug_log(&format!(
        "[OOVPA-SYNTH] Done: mmio={} pb={} draw={}",
        ctx.mmio_count, ctx.pb_commands, ctx.draw_calls
    ));
}

/// Run one frame of synthetic game loop: Clear → Draw → Swap.
/// Called from worker thread each VBlank after CreateDevice has fired.
/// Worker signals main thread for GPU operations via request/dirty flags.
pub fn run_synthetic_frame(ctx: &mut crate::xbox::aot::runtime::RuntimeContext, frame_num: u32) {
    let guest_mem = ctx.guest_mem_base;

    // Clear with rotating color (visual proof the loop is running)
    let r = ((frame_num * 3) % 256) as u32;
    let g = ((frame_num * 5 + 80) % 256) as u32;
    let b = ((frame_num * 7 + 160) % 256) as u32;
    let clear_color = 0xFF000000 | (r << 16) | (g << 8) | b;

    // Call HLE stubs directly (no guest code, no INT3)
    {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            backend.clear(clear_color);

            // Draw test triangle with per-frame animation
            let t = frame_num as f32 * 0.05;
            let cx = 320.0 + (t.sin() * 100.0);
            let cy = 240.0 + (t.cos() * 80.0);
            let verts = [
                crate::xbox::gpu::NV2AVertex {
                    x: cx,
                    y: cy - 160.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFFFF0000,
                    u: 0.0,
                    v: 0.0,
                },
                crate::xbox::gpu::NV2AVertex {
                    x: cx + 200.0,
                    y: cy + 120.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFF00FF00,
                    u: 1.0,
                    v: 1.0,
                },
                crate::xbox::gpu::NV2AVertex {
                    x: cx - 200.0,
                    y: cy + 120.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFF0000FF,
                    u: 0.0,
                    v: 1.0,
                },
            ];
            backend.draw_primitive(&verts, crate::xbox::gpu::NV097_TRIANGLES);

            // Readback for main thread display
            let pixels = backend.readback_framebuffer().to_vec();
            drop(gpu);
            crate::xbox::gpu::store_readback(&pixels);
        }
    }

    // Update counters
    ctx.mmio_count += 1;
    ctx.pb_commands += 1;
    ctx.draw_calls += 1;

    // Swap: increment device frame counter
    oovpa_hle::hle_swap(guest_mem, &mut ctx.mmio_count);
}

// Submodules
pub(crate) mod database;
mod oovpa_dispatch;
pub(crate) mod oovpa_hle;
mod oovpa_hooks;
mod oovpa_scan;
pub(crate) mod xbsymdb_dsound;
pub(crate) mod xbsymdb_patterns;
pub(crate) mod xbsymdb_xapi;

// ============================================================================
// Symbol cache — scan once, save to disk, skip scanning on subsequent loads.
// Cache stored alongside the XBE as <xbe_dir>/oovpa_cache/<hash>.txt
// ============================================================================

/// Generate cache file path from XBE path and hash.
pub fn cache_path(xbe_path: &str, xbe_hash: u64) -> String {
    let xbe_dir = if let Some(pos) = xbe_path.rfind('/').or_else(|| xbe_path.rfind('\\')) {
        &xbe_path[..=pos]
    } else {
        "./"
    };
    format!("{}oovpa_cache/{:016X}.txt", xbe_dir, xbe_hash)
}

/// Try to load cached OOVPA matches from disk.
/// Returns None if cache is missing, corrupt, or version-mismatched.
pub fn load_cache(path: &str, xdk_build: u16, entry_point: u32) -> Option<Vec<OovpaMatch>> {
    use std::io::BufRead;

    let file = std::fs::File::open(path).ok()?;
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();

    // Header line: "OOVPA_CACHE v3 profile=fast_boot xdk=NNNN entry=0xNNNNNNNN count=NNN"
    let header = lines.next()?.ok()?;
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 5 || parts[0] != "OOVPA_CACHE" || parts[1] != "v3" {
        return None;
    }
    let cached_profile = parts[2].strip_prefix("profile=")?;
    let active_profile = oovpa_scan::active_scan_profile_name();
    if cached_profile != active_profile {
        debug_log(&format!(
            "[OOVPA] Cache profile mismatch: {} vs {}",
            cached_profile, active_profile
        ));
        return None;
    }
    let cached_xdk: u16 = parts[3].strip_prefix("xdk=")?.parse().ok()?;
    let cached_entry: u32 = u32::from_str_radix(parts[4].strip_prefix("entry=0x")?, 16).ok()?;
    let cached_count: usize = parts
        .get(5)
        .and_then(|s| s.strip_prefix("count="))?
        .parse()
        .ok()?;

    if cached_xdk != xdk_build || cached_entry != entry_point {
        debug_log(&format!(
            "[OOVPA] Cache version mismatch: xdk={} vs {}, entry=0x{:08X} vs 0x{:08X}",
            cached_xdk, xdk_build, cached_entry, entry_point
        ));
        return None;
    }

    // Set global state that scan() normally sets
    XDK_BUILD.store(xdk_build, std::sync::atomic::Ordering::Relaxed);
    XBE_ENTRY.store(entry_point, std::sync::atomic::Ordering::Relaxed);

    let mut matches = Vec::with_capacity(cached_count);
    for line in lines {
        let line = line.ok()?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Format: "0xADDR name argc hle_mode"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let addr = u32::from_str_radix(parts[0].strip_prefix("0x").unwrap_or(parts[0]), 16).ok()?;
        let name_str = parts[1];
        let cached_argc: u8 = parts[2].parse().ok()?;

        // Resolve name to a &'static str by finding the matching pattern
        let pattern_name = resolve_cached_name(name_str);
        let argc = oovpa_scan::known_argc(pattern_name).unwrap_or(cached_argc);
        if argc != cached_argc {
            debug_log(&format!(
                "[OOVPA] Cache argc override: {} {} -> {}",
                pattern_name, cached_argc, argc
            ));
        }

        matches.push(OovpaMatch {
            guest_addr: addr,
            pattern_name,
            argc,
            hle_mode: HleMode::Hle,
            host_offset: 0,
            original_byte: 0,
            is_cxbx: false,
            is_manual: false,
            active: false,
            pending_rearm: false,
            call_count: 0,
        });
    }

    if matches.len() != cached_count {
        debug_log(&format!(
            "[OOVPA] Cache count mismatch: expected {}, got {}",
            cached_count,
            matches.len()
        ));
        return None;
    }

    Some(matches)
}

/// Resolve a cached pattern name string back to a &'static str.
/// Falls back to a leaked String if the pattern isn't found in any table.
fn resolve_cached_name(name: &str) -> &'static str {
    use crate::xbox::aot::oovpa_patterns;

    // Search hand-tuned patterns
    for pat in oovpa_patterns::HAND_TUNED_PATTERNS {
        if pat.name == name {
            return pat.name;
        }
    }
    // Search CxbxDB patterns
    for pat in oovpa_patterns::CXBX_DB_PATTERNS {
        if pat.name == name {
            return pat.name;
        }
    }
    // Search XbSymbolDatabase-backed patterns through the XDK 4134 database lane.
    for table in [
        database::d3d8_ltcg_patterns_for_xdk(4134),
        database::d3d8_patterns_for_xdk(4134),
        database::dsound_patterns_for_xdk(4134),
        database::xapi_patterns_for_xdk(4134),
        database::xgraphic_patterns_for_xdk(4134),
    ] {
        for pat in table {
            if pat.name == name {
                return pat.name;
            }
        }
    }
    // Fallback: leak the string so it has 'static lifetime
    Box::leak(name.to_string().into_boxed_str())
}

/// Save OOVPA matches to disk cache.
pub fn save_cache(path: &str, matches: &[OovpaMatch], xdk_build: u16) {
    // Create cache directory
    if let Some(dir_end) = path.rfind('/').or_else(|| path.rfind('\\')) {
        let dir = &path[..dir_end];
        let _ = std::fs::create_dir_all(dir);
    }

    let entry = XBE_ENTRY.load(std::sync::atomic::Ordering::Relaxed);
    let profile = oovpa_scan::active_scan_profile_name();
    let mut content = format!(
        "OOVPA_CACHE v3 profile={} xdk={} entry=0x{:08X} count={}\n",
        profile,
        xdk_build,
        entry,
        matches.len()
    );

    for m in matches {
        content.push_str(&format!(
            "0x{:08X} {} {}\n",
            m.guest_addr, m.pattern_name, m.argc
        ));
    }

    match std::fs::write(path, &content) {
        Ok(_) => debug_log(&format!(
            "[OOVPA] Saved {} matches to cache: {}",
            matches.len(),
            path
        )),
        Err(e) => debug_log(&format!("[OOVPA] Failed to save cache: {}: {}", path, e)),
    }
}

/// Read-only diagnostic: compare the current XDK database scan/cache against the
/// previous Rustemu Spider-Man OOVPA cache backup. This answers whether the new
/// Cxbx-R-shaped database lane found the same SDK/API boundary symbols as the
/// older working-ish path without using the old cache to plant hooks.
pub fn log_diff_against_previous_cache(
    cache_path: &str,
    xdk_build: u16,
    entry_point: u32,
    current: &[OovpaMatch],
) {
    if !env_flag("RUSTEMU_OOVPA_DIFF") {
        return;
    }

    let Some(old_path) = latest_cache_backup(cache_path) else {
        debug_log(&format!(
            "[OOVPA-DIFF] [MISSING_FROM_4134] old_cache=<none> current={} count={}",
            cache_path,
            current.len()
        ));
        return;
    };

    let Some(old) = load_cache(&old_path, xdk_build, entry_point) else {
        debug_log(&format!(
            "[OOVPA-DIFF] [MISSING_FROM_4134] old_cache={} unreadable_or_mismatch current={} count={}",
            old_path,
            cache_path,
            current.len()
        ));
        return;
    };

    debug_log(&format!(
        "[OOVPA-DIFF] [MATCH] baseline old_cache={} old_count={} fresh_count={} xdk={} entry=0x{:08X}",
        old_path,
        old.len(),
        current.len(),
        xdk_build,
        entry_point
    ));

    log_watchlist_diff(&old, current);
    log_name_deltas(&old, current);
    log_address_conflicts(&old, current);
}

fn latest_cache_backup(cache_path: &str) -> Option<String> {
    let path = std::path::Path::new(cache_path);
    let dir = path.parent()?;
    let file_name = path.file_name()?.to_string_lossy();
    let prefix = format!("{}.bak_", file_name);
    let current_content = std::fs::read(cache_path).ok();

    let mut newest: Option<(std::time::SystemTime, String)> = None;
    for entry in std::fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.starts_with(&prefix) {
            continue;
        }
        let full_path = entry.path();
        if let Some(current) = &current_content {
            if let Ok(candidate) = std::fs::read(&full_path) {
                if candidate == *current {
                    continue;
                }
            }
        }
        let modified = entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let full = full_path.to_string_lossy().into_owned();
        match &newest {
            Some((best, _)) if modified <= *best => {}
            _ => newest = Some((modified, full)),
        }
    }
    newest.map(|(_, path)| path)
}

fn log_watchlist_diff(old: &[OovpaMatch], current: &[OovpaMatch]) {
    const WATCHLIST: &[&str] = &[
        "D3DDevice_CreateDevice",
        "D3DDevice_Present",
        "D3DDevice_Swap",
        "D3DDevice_SetRenderTarget",
        "D3DDevice_Clear",
        "D3DDevice_DrawIndexedVertices",
        "D3DDevice_SetVertexShader",
        "D3DDevice_SetStreamSource",
        "XInputGetState",
        "CreateEvent",
        "SetEvent",
        "WaitForSingleObject",
    ];

    for wanted in WATCHLIST {
        let old_hits = lookup_symbol_aliases(old, wanted);
        let new_hits = lookup_symbol_aliases(current, wanted);
        let category = match (old_hits.is_empty(), new_hits.is_empty()) {
            (false, false) => {
                if same_hit_set(&old_hits, &new_hits) {
                    "MATCH"
                } else {
                    "ADDRESS_SHIFT"
                }
            }
            (false, true) => "MISSING_FROM_4134",
            (true, false) => "NEW_IN_4134",
            (true, true) => "MISSING_FROM_4134",
        };
        debug_log(&format!(
            "[OOVPA-DIFF] [{}] watch={} old={} fresh={}",
            category,
            wanted,
            format_hits(&old_hits),
            format_hits(&new_hits)
        ));
    }
}

fn log_name_deltas(old: &[OovpaMatch], current: &[OovpaMatch]) {
    use std::collections::{BTreeSet, HashMap};

    let old_by_name = matches_by_name(old);
    let new_by_name = matches_by_name(current);
    let mut names = BTreeSet::new();
    names.extend(old_by_name.keys().copied());
    names.extend(new_by_name.keys().copied());

    for name in names {
        let old_hits = old_by_name.get(name).cloned().unwrap_or_default();
        let new_hits = new_by_name.get(name).cloned().unwrap_or_default();
        if old_hits.is_empty() {
            debug_log(&format!(
                "[OOVPA-DIFF] [NEW_IN_4134] {} old=<none> fresh={}",
                name,
                format_hits(&new_hits)
            ));
        } else if new_hits.is_empty() {
            debug_log(&format!(
                "[OOVPA-DIFF] [MISSING_FROM_4134] {} old={} fresh=<none>",
                name,
                format_hits(&old_hits)
            ));
        } else if !same_hit_set(&old_hits, &new_hits) {
            debug_log(&format!(
                "[OOVPA-DIFF] [ADDRESS_SHIFT] {} old={} fresh={}",
                name,
                format_hits(&old_hits),
                format_hits(&new_hits)
            ));
        }
    }

    fn matches_by_name(matches: &[OovpaMatch]) -> HashMap<&'static str, Vec<&OovpaMatch>> {
        let mut by_name: HashMap<&'static str, Vec<&OovpaMatch>> = HashMap::new();
        for m in matches {
            by_name
                .entry(normalize_cache_name(m.pattern_name))
                .or_default()
                .push(m);
        }
        by_name
    }
}

fn log_address_conflicts(old: &[OovpaMatch], current: &[OovpaMatch]) {
    use std::collections::HashMap;

    let mut old_by_addr: HashMap<u32, &'static str> = HashMap::new();
    for m in old {
        old_by_addr.insert(m.guest_addr, normalize_cache_name(m.pattern_name));
    }
    for m in current {
        let new_name = normalize_cache_name(m.pattern_name);
        if let Some(old_name) = old_by_addr.get(&m.guest_addr) {
            if *old_name != new_name {
                debug_log(&format!(
                    "[OOVPA-DIFF] [NAME_CONFLICT] addr=0x{:08X} old={} fresh={}",
                    m.guest_addr, old_name, new_name
                ));
            }
        }
    }
}

fn lookup_symbol_aliases<'a>(matches: &'a [OovpaMatch], wanted: &str) -> Vec<&'a OovpaMatch> {
    matches
        .iter()
        .filter(|m| symbol_alias_matches(wanted, normalize_cache_name(m.pattern_name)))
        .collect()
}

fn symbol_alias_matches(wanted: &str, actual: &str) -> bool {
    if wanted == actual {
        return true;
    }
    match wanted {
        // Cxbx-R and Rustemu logs commonly name this as Direct3D_CreateDevice.
        "D3DDevice_CreateDevice" => actual == "Direct3D_CreateDevice",
        // Many XDK 4134 paths expose Present behavior through Swap.
        "D3DDevice_Present" => actual == "D3DDevice_Present" || actual == "D3DDevice_Swap",
        "WaitForSingleObject" => {
            actual == "WaitForSingleObject"
                || actual == "WaitForSingleObjectEx"
                || actual == "NtWaitForSingleObject"
        }
        _ => false,
    }
}

fn same_hit_set(left: &[&OovpaMatch], right: &[&OovpaMatch]) -> bool {
    let mut left_keys: Vec<(u32, &'static str)> = left
        .iter()
        .map(|m| (m.guest_addr, normalize_cache_name(m.pattern_name)))
        .collect();
    let mut right_keys: Vec<(u32, &'static str)> = right
        .iter()
        .map(|m| (m.guest_addr, normalize_cache_name(m.pattern_name)))
        .collect();
    left_keys.sort_unstable();
    right_keys.sort_unstable();
    left_keys == right_keys
}

fn format_hits(matches: &[&OovpaMatch]) -> String {
    if matches.is_empty() {
        return "<none>".to_string();
    }
    matches
        .iter()
        .map(|m| format!("{}@0x{:08X}/argc{}", m.pattern_name, m.guest_addr, m.argc))
        .collect::<Vec<_>>()
        .join(",")
}

fn normalize_cache_name(name: &str) -> &'static str {
    let base = if let Some(idx) = name.find("__LTCG") {
        &name[..idx]
    } else {
        name
    };
    let stripped = if base.len() > 3 {
        if let Some(last_us) = base.rfind('_') {
            let suffix = &base[last_us + 1..];
            if suffix.len() <= 2 && !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
            {
                &base[..last_us]
            } else {
                base
            }
        } else {
            base
        }
    } else {
        base
    };
    resolve_cached_name(stripped)
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "off" || v == "no")
        })
        .unwrap_or(false)
}
