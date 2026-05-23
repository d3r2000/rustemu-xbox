/// NV2A pushbuffer parser — walks a DMA pushbuffer command stream, translates
/// the subset of NV097 (Kelvin) methods that drive geometry submission into
/// host-side `queue_draw` calls.
///
/// Command word encoding (Xbox NV2A / Kelvin):
///   bit 30    : NON_INCREMENT (args all go to the same method offset)
///   bits 1:0  : 00=increment / 01=jump / 10=call / 11=return (NV2A extension)
///   bits 28:18: count (11 bits, number of arg dwords)
///   bits 15:13: subchannel (Kelvin = 0 on Xbox, clip subch/object-cache = 6/7)
///   bits 12:2 : method offset (byte offset / 4, masked 0x1FFC)
///
/// Spider-Man (XDK 4134) uses `BeginPush` → direct writes → `EndPush` for
/// draw submission. The sequence we care about:
///
///   PUSH( SET_BEGIN_END, 1 ); PUSH( prim_type );
///   PUSH( NOINC | INLINE_ARRAY, N ); PUSH( raw_dw0..raw_dwN-1 );
///   PUSH( SET_BEGIN_END, 1 ); PUSH( 0 );    // close batch
///
/// We maintain per-USER-PUT-write state because a single batch can span
/// multiple kicks if the game writes partial commands before kicking PUT.
use crate::xbox::gpu::{
    queue_draw, set_frame_dirty, NV2AVertex, NV097_LINES, NV097_LINE_LOOP, NV097_LINE_STRIP,
    NV097_POINTS, NV097_TRIANGLES, NV097_TRIANGLE_FAN, NV097_TRIANGLE_STRIP,
};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Mutex,
};

// ---------------------------------------------------------------------------
// Xbox D3DFVF flags we decode. This subset is what Spider-Man's vertex
// shaders resolve to; unknown handles fall through to stride-only heuristics.
// Owned here (not in oovpa.rs) because the parser is the primary consumer.
// ---------------------------------------------------------------------------
pub const FVF_POSITION_MASK: u32 = 0x00E;
pub const FVF_XYZ: u32 = 0x002; // 3 floats
pub const FVF_XYZRHW: u32 = 0x004; // 4 floats (screen-space)
pub const FVF_XYZB1: u32 = 0x006; // 3 floats + 1 weight
pub const FVF_XYZB2: u32 = 0x008; // 3 floats + 2 weights
pub const FVF_NORMAL: u32 = 0x010; // 3 floats
pub const FVF_DIFFUSE: u32 = 0x040; // u32 ARGB
pub const FVF_SPECULAR: u32 = 0x080; // u32 ARGB
pub const FVF_TEX1: u32 = 0x100; // 2 floats UV
pub const FVF_TEXCOUNT_MASK: u32 = 0xF00;
pub const FVF_TEXCOUNT_SHIFT: u32 = 8;

// ---------------------------------------------------------------------------
// Stream / shader state — written by D3DDevice_SetStreamSource and
// D3DDevice_SetVertexShader HLE handlers in oovpa.rs, read by the parser
// when translating DRAW_ARRAYS / INLINE_ARRAY payloads.
// ---------------------------------------------------------------------------

/// Stream 0 vertex buffer (guest pointer to raw vertex data or an
/// X_D3DVertexBuffer object). 0 = unbound.
pub static STREAM0_VB_ADDR: AtomicU32 = AtomicU32::new(0);

/// Stream 0 stride in bytes. 0 = unbound.
pub static STREAM0_STRIDE: AtomicU32 = AtomicU32::new(0);

/// Per-slot vertex-attribute format captured from
/// `NV097_SET_VERTEX_DATA_ARRAY_FORMAT` writes (method offsets 0x1760..0x179C,
/// one per slot). 16 slots total.
///
/// Format word layout (per xemu `NV097_SET_VERTEX_DATA_ARRAY_FORMAT_*` masks):
///   bits[0..3]   type   (NV2A attribute type enum — float, short, UB, etc.)
///   bits[4..7]   size   (number of components, 0 = disabled)
///   bits[8..31]  stride (24-bit stride in bytes of the WHOLE vertex as seen
///                        from this attribute; all attributes of a vertex
///                        share the same stride in practice, so slot-0's
///                        value is the per-vertex stride)
///
/// Per xemu: this is the authoritative vertex layout source when the game uses
/// a programmable vertex shader (fvf=0). Summing component sizes across enabled
/// slots gives the total vertex size in dwords.
pub static VS_SLOT_TYPE: [AtomicU32; 16] = {
    const INIT: AtomicU32 = AtomicU32::new(0);
    [INIT; 16]
};
pub static VS_SLOT_SIZE: [AtomicU32; 16] = {
    const INIT: AtomicU32 = AtomicU32::new(0);
    [INIT; 16]
};
pub static VS_SLOT_STRIDE: [AtomicU32; 16] = {
    const INIT: AtomicU32 = AtomicU32::new(0);
    [INIT; 16]
};
/// V04 fix: per-slot guest base pointer from NV097_SET_VERTEX_DATA_ARRAY_OFFSET.
/// Each of the 16 vertex attribute slots can have its own base address in guest
/// memory — the range 0x1720..=0x175C assigns one dword per slot. Previously we
/// only captured slot 0 (mirrored to STREAM0_VB_ADDR), losing slots 1..15 to
/// the catch-all histogram. For multi-stream / split-attribute layouts this
/// would collapse every attribute to slot 0's buffer.
pub static VS_SLOT_OFFSET: [AtomicU32; 16] = {
    const INIT: AtomicU32 = AtomicU32::new(0);
    [INIT; 16]
};

/// Current SetVertexShader value. Xbox D3D8 follows Cxbx-R's discriminator:
/// bit 0 set means a programmable vertex-shader handle; bit 0 clear means an
/// FVF bitfield. Only FVF-style values are decoded fully; shader handles fall
/// back to captured declaration state or stride heuristics.
pub static VERTEX_SHADER: AtomicU32 = AtomicU32::new(0);

/// Total NV2A draw-method commands parsed across all `drive`/`drive_range`
/// invocations (`NV097_DRAW_ARRAYS`, `NV097_INLINE_ARRAY`,
/// `NV097_ARRAY_ELEMENT16/32`, immediate `SET_VERTEX_DATA*`). This is the
/// **real** "draws submitted by the game" counter, distinct from:
///   - `D3DDevice_Swap` calls (frames presented, ~60Hz cadence)
///   - `D3DDevice_DrawVertices`/`DrawIndexedVertices` SDK helpers (Spider-Man
///     bypasses these via BeginPush macros, so they stay 0)
/// Read by the HUD as `Draws (NV2A)`.
pub static NV2A_DRAW_METHODS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Per-method-ID call count. Indexed by NV2A method address (low 14 bits, since
/// NV097 method addresses fit in 0x0000..0x1FFC). The parser increments
/// `METHOD_HIST[method]` on each parse. Use `dump_method_histogram(top_n)` to
/// retrieve the most-called method IDs.
///
/// 2026-04-21: added when investigating why Spider-Man submits 8K+ methods
/// per minute but ZERO of them are NV097_DRAW_ARRAYS. Histogram tells us
/// exactly what state the game IS configuring (texture binds? render states?
/// stream sources? viewport?) so we can identify the missing setter that
/// would gate the draw call.
pub static METHOD_HIST: std::sync::Mutex<Option<std::collections::HashMap<u32, u64>>> =
    std::sync::Mutex::new(None);

/// Same histogram, but keyed by `(subchannel << 16) | method`. This is useful
/// for titles that bind Kelvin on a non-zero subchannel; the old method-only
/// view could prove that a draw opcode existed without showing that our
/// `subch == 0` filter was dropping it.
pub static METHOD_SUBCH_HIST: std::sync::Mutex<Option<std::collections::HashMap<u32, u64>>> =
    std::sync::Mutex::new(None);

fn method_hist_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_NV2A_METHOD_HIST")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

/// Snapshot the top-N most-frequent NV2A methods seen so far.
/// Returns `Vec<(method_id, count)>` sorted by count descending.
pub fn dump_method_histogram(top_n: usize) -> Vec<(u32, u64)> {
    if !method_hist_enabled() {
        return Vec::new();
    }
    let guard = METHOD_HIST.lock().unwrap_or_else(|e| e.into_inner());
    let map = match guard.as_ref() {
        Some(m) => m,
        None => return Vec::new(),
    };
    let mut entries: Vec<(u32, u64)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(top_n);
    entries
}

fn dump_method_subch_histogram(top_n: usize) -> Vec<(u32, u32, u64)> {
    if !method_hist_enabled() {
        return Vec::new();
    }
    let guard = METHOD_SUBCH_HIST.lock().unwrap_or_else(|e| e.into_inner());
    let map = match guard.as_ref() {
        Some(m) => m,
        None => return Vec::new(),
    };
    let mut entries: Vec<(u32, u32, u64)> = map
        .iter()
        .map(|(&k, &v)| ((k >> 16) & 0x7, k & 0x1FFC, v))
        .collect();
    entries.sort_by(|a, b| b.2.cmp(&a.2));
    entries.truncate(top_n);
    entries
}

fn method_subch_count(method: u32, subch: u32) -> u64 {
    if !method_hist_enabled() {
        return 0;
    }
    let guard = METHOD_SUBCH_HIST.lock().unwrap_or_else(|e| e.into_inner());
    let Some(map) = guard.as_ref() else {
        return 0;
    };
    let key = ((subch & 0x7) << 16) | (method & 0x1FFC);
    map.get(&key).copied().unwrap_or(0)
}

fn log_method_summary(tag: &str, n: u32, methods_seen: u64, draws_emitted: u64) {
    let draw_methods = [
        ("BEGIN_END", NV097_SET_BEGIN_END),
        ("DRAW_ARRAYS", NV097_DRAW_ARRAYS),
        ("ARRAY16", NV097_ARRAY_ELEMENT16),
        ("ARRAY32", NV097_ARRAY_ELEMENT32),
        ("INLINE_ARRAY", NV097_INLINE_ARRAY),
        ("VERTEX2F_M", NV097_SET_VERTEX_DATA2F_M),
        ("VERTEX4F_M", NV097_SET_VERTEX_DATA4F_M),
    ];

    crate::xbox::aot::veh::veh_log(&format!(
        "[PB-METHOD-SUMMARY {} #{}] methods_total={} draws_total={}",
        tag, n, methods_seen, draws_emitted
    ));
    for (name, method) in draw_methods {
        let by_subch = (0..8u32)
            .map(|subch| format!("s{}={}", subch, method_subch_count(method, subch)))
            .collect::<Vec<_>>()
            .join(" ");
        crate::xbox::aot::veh::veh_log(&format!(
            "[PB-DRAW-METHOD {} #{}] {:<12} mth=0x{:04X} {}",
            tag, n, name, method, by_subch
        ));
    }

    crate::xbox::aot::veh::veh_log(&format!("[PB-METHOD-TOP {} #{}]", tag, n));
    for (subch, method, count) in dump_method_subch_histogram(20) {
        crate::xbox::aot::veh::veh_log(&format!(
            "  subch={} mth=0x{:04X} {:>28} hits={}",
            subch,
            method,
            method_name(method),
            count
        ));
    }
}

/// Human-readable name for an NV2A method ID. Returns "?" for unrecognised IDs.
/// Names use the standard NV097 nomenclature from xemu / nouveau / Xbox SDK.
pub fn method_name(method: u32) -> &'static str {
    match method {
        0x0000 => "SET_OBJECT",
        0x0100 => "NO_OP",
        0x0104 => "NOTIFY",
        0x0110 => "WAIT_FOR_IDLE",
        0x017C => "SET_CONTEXT_DMA_NOTIFIES",
        0x0180 => "SET_CONTEXT_DMA_A",
        0x0184 => "SET_CONTEXT_DMA_B",
        0x0190..=0x01A0 => "SET_CONTEXT_DMA_*",
        0x0200..=0x023C => "SET_SURFACE_*",
        0x0240..=0x0244 => "SET_TWO_SIDE_LIGHT_EN",
        0x0260..=0x027C => "SET_CULL_FACE",
        0x0290 => "SET_SHADE_MODEL",
        0x0294 => "SET_FRONT_FACE",
        0x0300 => "SET_ALPHA_TEST_ENABLE",
        0x0304 => "SET_BLEND_ENABLE",
        0x0308 => "SET_CULL_FACE_ENABLE",
        0x030C => "SET_DEPTH_TEST_ENABLE",
        0x0310 => "SET_DITHER_ENABLE",
        0x0314 => "SET_LIGHTING_ENABLE",
        0x0334 => "SET_POLY_OFFSET_LINE_ENABLE",
        0x0338 => "SET_POLY_OFFSET_FILL_ENABLE",
        0x033C => "SET_ALPHA_FUNC",
        0x0340 => "SET_ALPHA_REF",
        0x0344 => "SET_BLEND_FUNC_SFACTOR",
        0x0348 => "SET_BLEND_FUNC_DFACTOR",
        0x034C => "SET_BLEND_COLOR",
        0x0350 => "SET_BLEND_EQUATION",
        0x0354 => "SET_DEPTH_FUNC",
        0x0358 => "SET_COLOR_MASK",
        0x035C => "SET_DEPTH_MASK",
        0x0398 => "SET_CLIP_MAX",
        0x03B8 => "SET_SPECULAR_ENABLE",
        0x03BC => "SET_LIGHT_ENABLE_MASK",
        0x17BC => "SET_LOGIC_OP_ENABLE",
        0x17C0 => "SET_LOGIC_OP",
        0x0420 => "SET_VIEWPORT_OFFSET",
        0x0438 => "SET_LIGHT_ENABLE_MASK",
        0x0454 => "SET_VERTEX_DATA4F",
        0x0810 => "SET_TEXTURE_OFFSET",
        0x0818 => "SET_TEXTURE_FORMAT",
        0x081C => "SET_TEXTURE_ADDRESS",
        0x0820 => "SET_TEXTURE_CONTROL0",
        0x0824 => "SET_TEXTURE_CONTROL1",
        0x0828 => "SET_TEXTURE_FILTER",
        0x082C => "SET_TEXTURE_IMAGE_RECT",
        0x0830 => "SET_TEXTURE_PALETTE",
        0x0AC0..=0x0AFC => "SET_TEXTURE_MATRIX",
        0x0B00..=0x0B40 => "SET_FOG_*",
        0x0B80 => "SET_TRANSFORM_PROGRAM_LOAD",
        0x0B84 => "SET_TRANSFORM_PROGRAM_START",
        0x0B88 => "SET_TRANSFORM_CONSTANT_LOAD",
        0x0B8C..=0x0BBC => "SET_TRANSFORM_CONSTANT",
        0x1700 => "SET_BACK_END_WRITE_SEMAPHORE_RELEASE",
        0x1714 => "FLIP_INCREMENT_WRITE",
        0x1718 => "FLIP_STALL",
        0x17FC => "SET_BEGIN_END",
        0x1800 => "ARRAY_ELEMENT16",
        0x1808 => "ARRAY_ELEMENT32",
        0x1810 => "DRAW_ARRAYS",
        0x1818 => "INLINE_ARRAY",
        0x1880..=0x18FC => "SET_VERTEX_DATA2F_M",
        0x1900..=0x193C => "SET_VERTEX_DATA2S",
        0x1940..=0x197C => "SET_VERTEX_DATA4UB",
        0x1980..=0x19FC => "SET_VERTEX_DATA4S_M",
        0x1A00..=0x1AFC => "SET_VERTEX_DATA4F_M",
        0x1B00..=0x1B0C => "SET_TEXTURE_OFFSET[1-3]",
        _ => "?",
    }
}

fn nv097_blend_factor_to_xbox(value: u32) -> Option<u32> {
    Some(match value {
        0x0000 => 1,           // D3DBLEND_ZERO
        0x0001 => 2,           // D3DBLEND_ONE
        0x0300 => 3,           // D3DBLEND_SRCCOLOR
        0x0301 => 4,           // D3DBLEND_INVSRCCOLOR
        0x0302 => 5,           // D3DBLEND_SRCALPHA
        0x0303 => 6,           // D3DBLEND_INVSRCALPHA
        0x0304 => 7,           // D3DBLEND_DESTALPHA
        0x0305 => 8,           // D3DBLEND_INVDESTALPHA
        0x0306 => 9,           // D3DBLEND_DESTCOLOR
        0x0307 => 10,          // D3DBLEND_INVDESTCOLOR
        0x0308 => 11,          // D3DBLEND_SRCALPHASAT
        0x8001 | 0x8003 => 14, // D3DBLEND_BLENDFACTOR
        0x8002 | 0x8004 => 15, // D3DBLEND_INVBLENDFACTOR
        _ => return None,
    })
}

fn nv097_blend_equation_to_xbox(value: u32) -> Option<u32> {
    Some(match value {
        0x8006 | 0xF006 => 1, // D3DBLENDOP_ADD
        0x800A => 2,          // D3DBLENDOP_SUBTRACT
        0x800B | 0xF005 => 3, // D3DBLENDOP_REVSUBTRACT
        0x8007 => 4,          // D3DBLENDOP_MIN
        0x8008 => 5,          // D3DBLENDOP_MAX
        _ => return None,
    })
}

fn nv097_blend_window_maybe(method: u32, count: u32, non_inc: bool) -> bool {
    if count == 0 {
        return false;
    }
    if non_inc {
        return matches!(
            method,
            NV097_SET_BLEND_ENABLE
                | NV097_SET_BLEND_FUNC_SFACTOR
                | NV097_SET_BLEND_FUNC_DFACTOR
                | NV097_SET_BLEND_COLOR
                | NV097_SET_BLEND_EQUATION
        );
    }
    let last = method.saturating_add(count.saturating_sub(1).saturating_mul(4));
    method <= NV097_SET_BLEND_EQUATION && last >= NV097_SET_BLEND_ENABLE
}

fn apply_nv097_blend_method(method: u32, raw_value: u32) -> bool {
    let mapped = match method {
        NV097_SET_BLEND_ENABLE => Some((X_D3DRS_ALPHABLENDENABLE, u32::from(raw_value != 0))),
        NV097_SET_BLEND_FUNC_SFACTOR => {
            nv097_blend_factor_to_xbox(raw_value).map(|v| (X_D3DRS_SRCBLEND, v))
        }
        NV097_SET_BLEND_FUNC_DFACTOR => {
            nv097_blend_factor_to_xbox(raw_value).map(|v| (X_D3DRS_DESTBLEND, v))
        }
        NV097_SET_BLEND_COLOR => Some((X_D3DRS_BLENDCOLOR, raw_value)),
        NV097_SET_BLEND_EQUATION => {
            nv097_blend_equation_to_xbox(raw_value).map(|v| (X_D3DRS_BLENDOP, v))
        }
        _ => return false,
    };

    let Some((slot, value)) = mapped else {
        static UNMAPPED_LOG: AtomicU32 = AtomicU32::new(0);
        let n = UNMAPPED_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[PB-BLEND-UNMAPPED #{}] mth=0x{:04X} {} raw=0x{:08X}",
                n,
                method,
                method_name(method),
                raw_value
            ));
        }
        return true;
    };

    crate::xbox::gpu::drain_queued_draws_into_active_backend("PB blend state");

    if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
        gpu.set_render_state(slot, value);
    }

    static PB_BLEND_LOG: AtomicU32 = AtomicU32::new(0);
    let n = PB_BLEND_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[PB-BLEND-STATE #{}] mth=0x{:04X} {} raw=0x{:08X} -> slot={} value=0x{:08X}",
            n,
            method,
            method_name(method),
            raw_value,
            slot,
            value
        ));
    }
    true
}

fn log_nv097_pixel_combiner_window(
    guest_mem: *mut u8,
    method: u32,
    count: u32,
    non_inc: bool,
    args_start: u32,
    header_addr: u32,
) {
    let interesting = method == 0x0218
        || method == 0x0220
        || (0x0300..=0x03FC).contains(&method)
        || (0x0800..=0x0BFC).contains(&method)
        || (0x0C00..=0x0CFC).contains(&method);
    if !interesting || count == 0 {
        return;
    }

    static PB_PSH_LOG: AtomicU32 = AtomicU32::new(0);
    let seq = PB_PSH_LOG.fetch_add(1, Ordering::Relaxed);
    if seq >= 256 && !seq.is_power_of_two() {
        return;
    }

    let base = guest_mem as u64 + args_start as u64;
    let shown = count.min(4);
    let mut args = String::new();
    for i in 0..shown {
        let current_method = if non_inc { method } else { method + i * 4 };
        let value = unsafe { std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32) };
        if i > 0 {
            args.push(' ');
        }
        args.push_str(&format!("0x{:04X}:0x{:08X}", current_method, value));
    }

    crate::xbox::emulator::debug_log(&format!(
        "[PB-PSH-RAW] #{} hdr=0x{:08X} mth=0x{:04X} {} count={} non_inc={} shown={} args={}",
        seq,
        header_addr,
        method,
        method_name_hint(method),
        count,
        non_inc,
        shown,
        args
    ));
}

// ---------------------------------------------------------------------------
// Global singleton driver — called from two places:
//   1. veh_mmio::handle_mmio on USER_PUT MMIO write (organic kick path —
//      happens when a TAP'd KickOff body executes its own MMIO write).
//   2. oovpa::execute_hle on D3DDevice_KickOff (HLE hook path — runs
//      *instead* of the SDK's kick code, so we synthesize the PUT write).
// ---------------------------------------------------------------------------

static PARSER: Mutex<PbParser> = Mutex::new(PbParser::new());

/// Drive the parser for a new USER_PUT end pointer. Tracks the last PUT so
/// subsequent kicks parse only the new tail range [old..new).
pub fn drive(guest_mem: *mut u8, new_put: u32) {
    static LAST_PUT: AtomicU32 = AtomicU32::new(0);
    static CALL_N: AtomicU32 = AtomicU32::new(0);

    // Sanity bounds: PUT must be inside guest RAM (512 MB). The old hardcoded
    // PB_BASE/PB_TOP window (0x00EA0000..0x00EB0000) was the dummy-PB region
    // set up at D3D pre-init. Spider-Man writes to its OWN allocated
    // pushbuffer at runtime (e.g. 0x00D01000 observed), outside that window.
    // Splice-fix 2026-04-20: accept any guest-RAM PUT.
    if new_put < 0x0001_0000 || new_put >= 0x2000_0000 {
        return;
    }

    let n = CALL_N.fetch_add(1, Ordering::Relaxed);
    let old_put = LAST_PUT.swap(new_put, Ordering::Relaxed);
    // If LAST_PUT is stale/uninitialized, process only the last 4KB before
    // new_put (bounded, avoids scanning unrelated memory as pushbuffer).
    let start = if old_put != 0 && old_put < new_put && new_put.wrapping_sub(old_put) < 0x10_0000 {
        old_put
    } else {
        new_put.saturating_sub(0x1000)
    };
    if start >= new_put {
        return;
    }

    let verbose = n < 4 || n.is_power_of_two();
    let mut p = PARSER.lock().unwrap_or_else(|e| e.into_inner());
    let parsed = p.parse(guest_mem, start, new_put, verbose);
    if verbose {
        crate::xbox::aot::veh::veh_log(&format!(
            "[PB-DRIVE #{}] range=[0x{:08X}..0x{:08X}) parsed={} methods_total={} draws_total={}",
            n, start, new_put, parsed, p.methods_seen, p.draws_emitted
        ));
        log_method_summary("DRIVE", n, p.methods_seen, p.draws_emitted);
    }
}

/// Read total draws emitted by the parser across all kicks (diagnostic).
pub fn total_draws_emitted() -> u64 {
    PARSER
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .draws_emitted
}

/// Drive the parser across an explicit pushbuffer range. Used by hle_swap
/// which intercepts the SDK KickOff path (no MMIO USER_PUT write happens,
/// so `drive()`'s LAST_PUT tracking doesn't apply).
pub fn drive_range(guest_mem: *mut u8, start: u32, end: u32) {
    static CALL_N: AtomicU32 = AtomicU32::new(0);
    if start >= end {
        return;
    }
    if start < 0x0001_0000 || end >= 0x2000_0000 {
        return;
    }
    if end.wrapping_sub(start) > 0x10_0000 {
        return;
    } // max 1 MB batch
    let n = CALL_N.fetch_add(1, Ordering::Relaxed);
    let verbose = n < 4 || n.is_power_of_two();
    let mut p = PARSER.lock().unwrap_or_else(|e| e.into_inner());
    let methods_before = p.methods_seen;
    let draws_before = p.draws_emitted;
    if verbose {
        log_range_raw_words(guest_mem, "RANGE", n, start, end);
    }
    let parsed = p.parse(guest_mem, start, end, verbose);
    let methods_delta = p.methods_seen.saturating_sub(methods_before);
    let draws_delta = p.draws_emitted.saturating_sub(draws_before);
    if verbose {
        crate::xbox::aot::veh::veh_log(&format!(
            "[PB-DRIVE-RANGE #{}] range=[0x{:08X}..0x{:08X}) bytes={} parsed={} methods_delta={} draws_delta={} methods_total={} draws_total={}",
            n,
            start,
            end,
            end.wrapping_sub(start),
            parsed,
            methods_delta,
            draws_delta,
            p.methods_seen,
            p.draws_emitted
        ));
        log_method_summary("RANGE", n, p.methods_seen, p.draws_emitted);
    }
}

fn log_range_raw_words(guest_mem: *mut u8, tag: &str, n: u32, start: u32, end: u32) {
    if guest_mem.is_null() || end <= start || end > 0x2000_0000 {
        return;
    }
    let byte_count = end.wrapping_sub(start) as usize;
    let dword_count = (byte_count / 4).min(48);
    if dword_count == 0 {
        return;
    }

    let mut words = Vec::with_capacity(dword_count);
    for i in 0..dword_count {
        let addr = start.saturating_add((i as u32).saturating_mul(4));
        let word =
            unsafe { std::ptr::read_unaligned((guest_mem as u64 + addr as u64) as *const u32) };
        words.push(word);
    }
    let hex = words
        .iter()
        .map(|w| format!("{:08X}", w))
        .collect::<Vec<_>>()
        .join(" ");
    crate::xbox::emulator::debug_log(&format!(
        "[PB-{}-RAW #{}] range=[0x{:08X}..0x{:08X}) bytes={} dwords={} head={}",
        tag, n, start, end, byte_count, dword_count, hex
    ));

    let mut cur_words = 0usize;
    let mut tokens = 0u32;
    while cur_words < words.len() && tokens < 12 {
        let hdr_index = cur_words;
        let dw = words[cur_words];
        cur_words += 1;
        tokens += 1;

        if dw == 0 {
            let zero_start = hdr_index;
            while cur_words < words.len() && words[cur_words] == 0 {
                cur_words += 1;
            }
            crate::xbox::emulator::debug_log(&format!(
                "[PB-{}-DECODE #{}] +{:03X}: zero-run dwords={}",
                tag,
                n,
                zero_start * 4,
                cur_words - zero_start
            ));
            continue;
        }

        if (dw & 3) == 1 {
            crate::xbox::emulator::debug_log(&format!(
                "[PB-{}-DECODE #{}] +{:03X}: JUMP target=0x{:08X} hdr=0x{:08X}",
                tag,
                n,
                hdr_index * 4,
                dw & 0xFFFF_FFFC,
                dw
            ));
            continue;
        }

        if (dw & 3) != 0 {
            crate::xbox::emulator::debug_log(&format!(
                "[PB-{}-DECODE #{}] +{:03X}: CALL/RET-like hdr=0x{:08X}",
                tag,
                n,
                hdr_index * 4,
                dw
            ));
            break;
        }

        let non_inc = (dw >> 30) & 1 != 0;
        let count = ((dw >> 18) & 0x7FF) as usize;
        let subch = ((dw >> 13) & 7) as u32;
        let method = dw & 0x1FFC;
        let available = words.len().saturating_sub(cur_words);
        let sample_len = count.min(available).min(12);
        let args = words[cur_words..cur_words + sample_len]
            .iter()
            .map(|w| format!("0x{:08X}", w))
            .collect::<Vec<_>>()
            .join(",");
        crate::xbox::emulator::debug_log(&format!(
            "[PB-{}-DECODE #{}] +{:03X}: hdr=0x{:08X} mth=0x{:04X} subch={} count={} non_inc={} args=[{}]{}",
            tag,
            n,
            hdr_index * 4,
            dw,
            method,
            subch,
            count,
            non_inc,
            args,
            if count > sample_len { "..." } else { "" }
        ));
        cur_words = cur_words.saturating_add(count.min(available));
    }
}

fn submit_pb_draw(verts: Vec<NV2AVertex>, prim_type: i32) {
    if std::env::var_os("RUSTEMU_NV2A_QUEUE_DRAWS").is_some() {
        queue_draw(verts, prim_type);
        return;
    }

    static SUBMIT_N: AtomicU32 = AtomicU32::new(0);
    let n = SUBMIT_N.fetch_add(1, Ordering::Relaxed);
    let vert_count = verts.len();
    let mut gpu = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut backend) = *gpu {
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[PB-DRAW-IMMEDIATE #{}] prim={} verts={}",
                n, prim_type, vert_count
            ));
        }
        backend.draw_primitive(&verts, prim_type);
    } else {
        queue_draw(verts, prim_type);
    }
}

fn spidey_rt_trace_pb_enabled() -> bool {
    std::env::var_os("RUSTEMU_SPIDEY_RT_TRACE").is_some()
}

fn spidey_composite_vs(vs: u32) -> bool {
    spidey_rt_trace_pb_enabled() && matches!(vs, 0x0000_1002 | 0x0000_100B)
}

fn vertex_shader_value_is_programmable(vs: u32) -> bool {
    (vs & 1 != 0) || crate::xbox::gpu::nv2a_vsh::is_registered_shader_handle(vs)
}

pub fn fvf_from_vertex_shader_value(vs: u32) -> u32 {
    // Cxbx-R: VshHandleIsVertexShader(handle) iff X_D3DFVF_RESERVED0 (bit 0)
    // is set. A clear low bit is an FVF value, so handles like Shenmue's 0x1C4
    // must decode as XYZRHW|DIFFUSE|SPECULAR|TEX1 rather than a VS object.
    // Rustemu also tolerates legacy fake handles already registered in the
    // local shader table, because older CreateVertexShader HLE did not force
    // generated handles to be odd.
    if !vertex_shader_value_is_programmable(vs) {
        vs
    } else {
        0
    }
}

pub fn seed_fvf_vertex_slots(fvf: u32) -> u32 {
    let stride = fvf_stride(fvf);
    for slot in 0..16 {
        VS_SLOT_TYPE[slot].store(0, Ordering::Relaxed);
        VS_SLOT_SIZE[slot].store(0, Ordering::Relaxed);
        VS_SLOT_STRIDE[slot].store(0, Ordering::Relaxed);
        VS_SLOT_OFFSET[slot].store(0, Ordering::Relaxed);
    }

    if stride == 0 {
        return 0;
    }

    let mut enabled = Vec::new();
    let mut set_slot = |slot: usize, ty: u32, size: u32, name: &'static str| {
        VS_SLOT_TYPE[slot].store(ty, Ordering::Relaxed);
        VS_SLOT_SIZE[slot].store(size, Ordering::Relaxed);
        VS_SLOT_STRIDE[slot].store(stride, Ordering::Relaxed);
        enabled.push(format!("s{}:{}:t{}x{}", slot, name, ty, size));
    };

    match fvf & FVF_POSITION_MASK {
        p if p == FVF_XYZ => set_slot(0, 2, 3, "pos"),
        p if p == FVF_XYZRHW => set_slot(0, 2, 4, "rhw"),
        p if p == FVF_XYZB1 => {
            set_slot(0, 2, 3, "pos");
            set_slot(1, 2, 1, "blend");
        }
        p if p == FVF_XYZB2 => {
            set_slot(0, 2, 3, "pos");
            set_slot(1, 2, 2, "blend");
        }
        _ => {}
    }
    if fvf & FVF_NORMAL != 0 {
        set_slot(2, 2, 3, "normal");
    }
    if fvf & FVF_DIFFUSE != 0 {
        set_slot(3, 0, 4, "diffuse");
    }
    if fvf & FVF_SPECULAR != 0 {
        set_slot(4, 0, 4, "specular");
    }
    let tex_count = ((fvf & FVF_TEXCOUNT_MASK) >> FVF_TEXCOUNT_SHIFT).min(4);
    for tex in 0..tex_count {
        set_slot((9 + tex) as usize, 2, 2, "tex");
    }

    static FVF_SLOT_LOG: AtomicU32 = AtomicU32::new(0);
    let n = FVF_SLOT_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 32 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[FVF-SLOT-SEED] #{} fvf=0x{:X} stride={} {}",
            n,
            fvf,
            stride,
            enabled.join(" ")
        ));
    }

    stride
}

fn spidey_format_vertex_slots() -> String {
    let mut parts = String::new();
    for slot in [0usize, 1, 2, 3, 4, 9] {
        let sz = VS_SLOT_SIZE[slot].load(Ordering::Relaxed);
        let ty = VS_SLOT_TYPE[slot].load(Ordering::Relaxed);
        let stride = VS_SLOT_STRIDE[slot].load(Ordering::Relaxed);
        let offset = VS_SLOT_OFFSET[slot].load(Ordering::Relaxed);
        parts.push_str(&format!(
            " s{}={{off:0x{:08X},str:{},sz:{},ty:{}}}",
            slot, offset, stride, sz, ty
        ));
    }
    parts
}

// ---------------------------------------------------------------------------
// NV097 method offsets (bits 12:2 of command word, masked 0x1FFC).
// ---------------------------------------------------------------------------
const NV097_NO_OPERATION: u32 = 0x0100;
const NV097_WAIT_FOR_IDLE: u32 = 0x0110;
const NV097_SET_OBJECT: u32 = 0x0000;
const NV097_SET_BEGIN_END: u32 = 0x17FC;
const NV097_ARRAY_ELEMENT16: u32 = 0x1800; // 2 × 16-bit indices per dword
const NV097_ARRAY_ELEMENT32: u32 = 0x1808; // 1 × 32-bit index per dword
const NV097_DRAW_ARRAYS: u32 = 0x1810;
const NV097_INLINE_ARRAY: u32 = 0x1818;
const NV097_SET_VERTEX_DATA2F_M: u32 = 0x1880;
const NV097_SET_VERTEX_DATA2S: u32 = 0x1900;
const NV097_SET_VERTEX_DATA4UB: u32 = 0x1940;
const NV097_SET_VERTEX_DATA4S_M: u32 = 0x1980;
const NV097_SET_VERTEX_DATA4F_M: u32 = 0x1A00;

fn direct_vertex_submit_enabled() -> bool {
    std::env::var_os("RUSTEMU_NV097_DIRECT_VERTEX_SUBMIT").is_some()
}

// NV097 blend-state methods. xemu's NV097 table is the source of truth here:
// 0x0338 is polygon-offset fill enable, while alpha blending is 0x0304.
const NV097_SET_BLEND_ENABLE: u32 = 0x0304;
const NV097_SET_BLEND_FUNC_SFACTOR: u32 = 0x0344;
const NV097_SET_BLEND_FUNC_DFACTOR: u32 = 0x0348;
const NV097_SET_BLEND_COLOR: u32 = 0x034C;
const NV097_SET_BLEND_EQUATION: u32 = 0x0350;

// Existing backend render-state tracking uses Xbox D3D8 render-state slots.
const X_D3DRS_ALPHABLENDENABLE: u32 = 59;
const X_D3DRS_SRCBLEND: u32 = 62;
const X_D3DRS_DESTBLEND: u32 = 63;
const X_D3DRS_BLENDOP: u32 = 74;
const X_D3DRS_BLENDCOLOR: u32 = 75;

// Vertex attribute array setup methods. Each base has 16 slots (one per
// attribute: 0=POSITION, 1=WEIGHT, 2=NORMAL, 3=DIFFUSE, 4=SPECULAR, etc.),
// stride 4 bytes per slot.
const NV097_SET_VERTEX_DATA_ARRAY_OFFSET: u32 = 0x1720; // per-attribute VB base addr
const NV097_SET_VERTEX_DATA_ARRAY_FORMAT: u32 = 0x1760; // per-attribute format word:
                                                        // format word layout: bits[0..3]=type, bits[4..7]=size(#components),
                                                        // bits[8..15]=stride (bytes). Stride applies to the WHOLE vertex, not
                                                        // just this attribute — hardware says all attributes of a vertex share
                                                        // a stride, so attribute 0 carries the interleaved stride we use.

// ---------------------------------------------------------------------------
// NV097_SET_BEGIN_END primitive type (arg 0).
// Values match Xbox NV2A hardware (see xemu nv2a_pgraph_primitive_type).
// ---------------------------------------------------------------------------
const NV_PRIM_NONE: u32 = 0;
const NV_PRIM_POINTS: u32 = 1;
const NV_PRIM_LINES: u32 = 2;
const NV_PRIM_LINE_LOOP: u32 = 3;
const NV_PRIM_LINE_STRIP: u32 = 4;
const NV_PRIM_TRIANGLES: u32 = 5;
const NV_PRIM_TRIANGLE_STRIP: u32 = 6;
const NV_PRIM_TRIANGLE_FAN: u32 = 7;
const NV_PRIM_QUADS: u32 = 8;
const NV_PRIM_QUAD_STRIP: u32 = 9;
const NV_PRIM_POLYGON: u32 = 10;

// ---------------------------------------------------------------------------
// Parser state — persists across multiple USER_PUT kicks because a game can
// call BeginPush / write half a batch / KickOff / write more / KickOff before
// the closing SET_BEGIN_END(0).
// ---------------------------------------------------------------------------
pub struct PbParser {
    /// Current primitive (0 = no open batch).
    prim: u32,
    /// Accumulated inline vertex data (raw NV2A dwords).
    inline: Vec<u32>,
    /// Accumulated indices for ARRAY_ELEMENT16 / ARRAY_ELEMENT32 indexed-draw
    /// batches. Drained on SET_BEGIN_END(NONE).
    /// Added 2026-04-21 to wire the previously "not yet wired" indexed-draw
    /// path — Spider-Man submits 100+ of these per run.
    indices: Vec<u32>,
    /// Current immediate-mode vertex attribute values written through
    /// NV097_SET_VERTEX_DATA* methods.
    direct_attrs: [[f32; 4]; 16],
    direct_attr_valid: [bool; 16],
    /// Immediate-mode vertices completed by position-slot writes.
    direct_verts: Vec<NV2AVertex>,
    /// Total methods parsed since parser creation.
    pub methods_seen: u64,
    /// Total draw batches dispatched via queue_draw.
    pub draws_emitted: u64,
    /// Total jump commands followed (diagnostic).
    pub jumps_followed: u64,
}

impl PbParser {
    pub const fn new() -> Self {
        Self {
            prim: 0,
            inline: Vec::new(),
            indices: Vec::new(),
            direct_attrs: [[0.0; 4]; 16],
            direct_attr_valid: [false; 16],
            direct_verts: Vec::new(),
            methods_seen: 0,
            draws_emitted: 0,
            jumps_followed: 0,
        }
    }

    /// Walk `[start..end)` as NV097 command tokens. `guest_mem` is the host
    /// pointer to the guest 4 GB reservation (R15 equivalent). Returns the
    /// number of methods parsed in this pass (caller uses for diagnostics).
    ///
    /// Safety: Reads from `guest_mem + start..end`. Caller must guarantee that
    /// range is mapped (our 512 MB RAM is, but addresses above 0x2000_0000
    /// may fault). Range is clamped before dereferencing.
    pub fn parse(&mut self, guest_mem: *mut u8, mut cur: u32, end: u32, log_verbose: bool) -> u32 {
        // Hard bound — never walk beyond 256 MB into guest RAM.
        let end = end.min(0x1000_0000);
        if cur >= end {
            return 0;
        }
        let mut parsed = 0u32;
        // Per-call safety cap: never parse more than 64 K commands in one
        // USER_PUT event, even if the PB is longer.
        const MAX_TOKENS: u32 = 65_536;
        while cur + 4 <= end && parsed < MAX_TOKENS {
            let dw =
                unsafe { std::ptr::read_unaligned((guest_mem as u64 + cur as u64) as *const u32) };
            cur += 4;
            parsed += 1;

            // Zero is an NV2A NOP/filler dword. Do not count it as a real
            // method; otherwise an untouched pushbuffer page looks like
            // thousands of SET_OBJECT packets and hides the fact that the
            // parser was pointed at empty memory.
            if dw == 0 {
                continue;
            }

            // JUMP (bits 1:0 == 01): target = dw & 0xFFFFFFFC. Only follow
            // jumps that stay within the dummy PB region (0x00EA_0000 +
            // 64 KB). External jumps abort the parse — the game is using a
            // segmented pushbuffer we don't model yet.
            if (dw & 3) == 1 {
                let tgt = dw & 0xFFFF_FFFC;
                if tgt >= 0x00EA_0000 && tgt < 0x00EB_0000 && tgt < end {
                    cur = tgt;
                    self.jumps_followed += 1;
                    continue;
                }
                break;
            }
            // CALL / RETURN (bits 1:0 == 10/11): NV2A extension, rare in
            // SDK-generated PBs. Treat as end-of-stream.
            if (dw & 3) != 0 {
                break;
            }

            let non_inc = (dw >> 30) & 1 != 0;
            let count = ((dw >> 18) & 0x7FF) as u32;
            let subch = ((dw >> 13) & 7) as u32;
            let method = dw & 0x1FFC;

            let args_start = cur;
            let args_end = cur.saturating_add(count.saturating_mul(4));
            if args_end > end {
                break;
            }
            cur = args_end;

            self.methods_seen = self.methods_seen.saturating_add(1);

            // Diagnostic histogram only. It takes locks and touches HashMaps,
            // so keep it out of normal gameplay unless explicitly requested.
            if method_hist_enabled() {
                let mut g = METHOD_HIST.lock().unwrap_or_else(|e| e.into_inner());
                let map = g.get_or_insert_with(std::collections::HashMap::new);
                *map.entry(method).or_insert(0) += 1;

                let mut g = METHOD_SUBCH_HIST.lock().unwrap_or_else(|e| e.into_inner());
                let map = g.get_or_insert_with(std::collections::HashMap::new);
                let key = ((subch & 0x7) << 16) | (method & 0x1FFC);
                *map.entry(key).or_insert(0) += 1;
            }

            // [SURFACE-ARG] Log the FIRST argument of each surface-state
            // method on the first ~10 occurrences. This tells us whether
            // the game is sending meaningful pitches/offsets/formats or
            // just writing zeros (which would explain PGRAPH_SURFACE_*
            // shadow registers reading 0 despite 750 writes each).
            //
            // Methods:
            //   0x0208 SET_SURFACE_PITCH           (pitch in low 16, zeta in high)
            //   0x020C SET_SURFACE_COLOR_OFFSET    (render-target guest phys addr)
            //   0x0210 SET_SURFACE_ZETA_OFFSET     (depth-buffer guest phys addr)
            //   0x0214 SET_SURFACE_FORMAT          (color/zeta/swizzle bits)
            //   0x0218 SET_SURFACE_CLIP_HORIZONTAL
            //   0x021C SET_SURFACE_CLIP_VERTICAL
            //   0x019C SET_CONTEXT_DMA_COLOR        (DMA channel bind)
            //   0x01A0 SET_CONTEXT_DMA_ZETA
            const SURFACE_METHODS: &[u32] = &[
                0x0208, 0x020C, 0x0210, 0x0214, 0x0218, 0x021C, 0x019C, 0x01A0,
            ];
            if SURFACE_METHODS.contains(&method) && count > 0 {
                static SURFACE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SURFACE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 {
                    let arg0 = unsafe {
                        std::ptr::read_unaligned(
                            (guest_mem as u64 + args_start as u64) as *const u32,
                        )
                    };
                    crate::xbox::emulator::debug_log(&format!(
                        "[SURFACE-ARG #{}] mth=0x{:04X} count={} arg0=0x{:08X} @guest=0x{:08X}",
                        n,
                        method,
                        count,
                        arg0,
                        cur - 4
                    ));
                }
            }

            if log_verbose && parsed <= 96 {
                let mut arg_sample = Vec::new();
                let base = guest_mem as u64 + args_start as u64;
                for i in 0..count.min(8) {
                    let arg =
                        unsafe { std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32) };
                    arg_sample.push(format!("0x{:08X}", arg));
                }
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-PARSE #{:>4}] mth=0x{:04X} subch={} count={} non_inc={} dw=0x{:08X} args=[{}] @0x{:08X}",
                    self.methods_seen,
                    method,
                    subch,
                    count,
                    non_inc,
                    dw,
                    arg_sample.join(","),
                    cur - 4
                ));
            }

            // Kelvin uses subchannel 0 (Xbox default). Ignore commands on
            // other subchannels (object binding / 2D / etc).
            if subch != 0 {
                continue;
            }

            if nv097_blend_window_maybe(method, count, non_inc) {
                let base = guest_mem as u64 + args_start as u64;
                for i in 0..count {
                    let current_method = if non_inc { method } else { method + i * 4 };
                    let arg =
                        unsafe { std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32) };
                    apply_nv097_blend_method(current_method, arg);
                }
            }
            log_nv097_pixel_combiner_window(guest_mem, method, count, non_inc, args_start, cur - 4);

            // -----------------------------------------------------------------
            // [NV2A-RAW-DRAW] Temporary: dump the raw DWORD stream for the
            // first 3 draw batches so we can see exactly what the guest is
            // pushing for HUD glyphs (6-vertex degenerate strips).
            //
            // A "draw batch" = BEGIN_END(prim!=0) .. BEGIN_END(0). We open
            // the dump window on the first BEGIN_END with a non-zero primitive
            // and close it after 3 batches have been fully captured.
            // -----------------------------------------------------------------
            {
                use std::sync::atomic::{AtomicU32, Ordering};
                static RAW_OPEN: AtomicU32 = AtomicU32::new(0); // 1 = window open
                static RAW_BATCHES: AtomicU32 = AtomicU32::new(0); // completed batches
                const RAW_MAX_BATCHES: u32 = 3;
                // Peek arg0 (if any) for BEGIN_END window control.
                let arg0_peek: u32 = if count > 0 {
                    unsafe {
                        std::ptr::read_unaligned(
                            (guest_mem as u64 + args_start as u64) as *const u32,
                        )
                    }
                } else {
                    0
                };
                let was_open = RAW_OPEN.load(Ordering::Relaxed) == 1;
                let done = RAW_BATCHES.load(Ordering::Relaxed) >= RAW_MAX_BATCHES;
                // Open window on first BEGIN_END(prim != 0).
                if !was_open
                    && !done
                    && method == NV097_SET_BEGIN_END
                    && count > 0
                    && arg0_peek != 0
                {
                    RAW_OPEN.store(1, Ordering::Relaxed);
                    crate::xbox::emulator::debug_log(
                        "[NV2A-RAW-DRAW] === BEGIN capture window (first 3 batches) ===",
                    );
                }
                let now_open = RAW_OPEN.load(Ordering::Relaxed) == 1;
                if now_open {
                    let batch_idx = RAW_BATCHES.load(Ordering::Relaxed);
                    // Dump header + every arg dword for this command.
                    crate::xbox::emulator::debug_log(&format!(
                        "[NV2A-RAW-DRAW #{}] HDR dw=0x{:08X} mth=0x{:04X} subch={} count={} non_inc={} @guest=0x{:08X}",
                        batch_idx, dw, method, subch, count, non_inc, cur - 4
                    ));
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let v = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        crate::xbox::emulator::debug_log(&format!(
                            "[NV2A-RAW-DRAW #{}]   arg[{:>3}] mth=0x{:04X}+0x{:X} val=0x{:08X}",
                            batch_idx,
                            i,
                            method,
                            if non_inc { 0 } else { i * 4 },
                            v
                        ));
                    }
                    // Close the batch on BEGIN_END(0); advance counter.
                    if method == NV097_SET_BEGIN_END && count > 0 && arg0_peek == 0 {
                        let n = RAW_BATCHES.fetch_add(1, Ordering::Relaxed) + 1;
                        crate::xbox::emulator::debug_log(&format!(
                            "[NV2A-RAW-DRAW] === END batch #{} ===",
                            n - 1
                        ));
                        if n >= RAW_MAX_BATCHES {
                            RAW_OPEN.store(0, Ordering::Relaxed);
                            crate::xbox::emulator::debug_log(
                                "[NV2A-RAW-DRAW] === capture window CLOSED ===",
                            );
                        }
                    }
                }
            }

            match method {
                NV097_SET_BEGIN_END => {
                    if count == 0 {
                        continue;
                    }
                    let arg = unsafe {
                        std::ptr::read_unaligned(
                            (guest_mem as u64 + args_start as u64) as *const u32,
                        )
                    };
                    if arg == NV_PRIM_NONE {
                        // Close batch — emit whatever we accumulated.
                        // Inline, indexed, and immediate-vertex paths are
                        // mutually exclusive in normal NV2A batches.
                        if !self.indices.is_empty() {
                            self.emit_indexed(guest_mem, log_verbose);
                        } else if !self.direct_verts.is_empty() {
                            self.emit_direct(log_verbose);
                        } else {
                            self.emit_inline(log_verbose);
                        }
                    } else {
                        // Open new batch. Discard any stale state.
                        self.prim = arg;
                        self.inline.clear();
                        self.indices.clear();
                        self.direct_verts.clear();
                    }
                }

                NV097_INLINE_ARRAY => {
                    // All `count` dwords are vertex data.
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let dw_v = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.inline.push(dw_v);
                    }
                }

                NV097_DRAW_ARRAYS => {
                    // Each arg dword: (count-1) << 24 | start (24 bits).
                    // Multiple args = concatenated ranges. Emit each as a
                    // sequential draw from the bound stream source.
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let a = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        let vstart = a & 0x00FF_FFFF;
                        let vcount = ((a >> 24) & 0xFF) + 1;
                        self.emit_sequential(guest_mem, vstart, vcount, log_verbose);
                    }
                    NV2A_DRAW_METHODS.fetch_add(count as u64, std::sync::atomic::Ordering::Relaxed);
                }

                // Indexed draws — accumulate indices for emit_indexed at
                // SET_BEGIN_END(NONE) close.
                //
                // Wiring added 2026-04-21. Previously this arm only counted
                // and logged "not yet wired to draw". Spider-Man submits 100+
                // of these per run — wiring them is the last gap between
                // NV2A parser seeing draws and queue_draw receiving them.
                //
                // Wire formats:
                //   NV097_ARRAY_ELEMENT16 (0x1800): each dword = 2×u16 indices
                //     packed as (idx1 << 16) | idx0. With `count` dwords,
                //     contributes `count*2` indices.
                //   NV097_ARRAY_ELEMENT32 (0x1808): each dword = 1×u32 index.
                //     Contributes `count` indices.
                NV097_ARRAY_ELEMENT16 => {
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        // Lower 16 bits = first index, upper 16 bits = second.
                        self.indices.push(dw & 0xFFFF);
                        // Upper half may be 0xFFFF = "no index" sentinel on
                        // NV2A — clamp by skipping high bits that match that.
                        let hi = (dw >> 16) & 0xFFFF;
                        if hi != 0xFFFF {
                            self.indices.push(hi);
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                NV097_ARRAY_ELEMENT32 => {
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let idx = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.indices.push(idx);
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                // Immediate-mode vertex data. These methods write current
                // vertex attribute values directly into PGRAPH state; a
                // position-slot write completes one vertex for the open batch.
                // `0x1880` is SET_VERTEX_DATA2F_M, not an index stream.
                m if (NV097_SET_VERTEX_DATA2F_M..=NV097_SET_VERTEX_DATA2F_M + 0x7C)
                    .contains(&m) =>
                {
                    let base_method = ((m - NV097_SET_VERTEX_DATA2F_M) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let method_slot = if non_inc {
                            base_method
                        } else {
                            base_method + i as usize
                        };
                        let attr = method_slot / 2;
                        let part = method_slot % 2;
                        if attr >= 16 {
                            continue;
                        }
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.direct_attrs[attr][part] = f32::from_bits(dw);
                        self.direct_attrs[attr][2] = 0.0;
                        self.direct_attrs[attr][3] = 1.0;
                        self.direct_attr_valid[attr] = true;
                        if attr == 0 && part == 1 {
                            self.finish_direct_vertex();
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                m if (NV097_SET_VERTEX_DATA4F_M..=NV097_SET_VERTEX_DATA4F_M + 0xFC)
                    .contains(&m) =>
                {
                    let base_method = ((m - NV097_SET_VERTEX_DATA4F_M) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let method_slot = if non_inc {
                            base_method
                        } else {
                            base_method + i as usize
                        };
                        let attr = method_slot / 4;
                        let part = method_slot % 4;
                        if attr >= 16 {
                            continue;
                        }
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.direct_attrs[attr][part] = f32::from_bits(dw);
                        self.direct_attr_valid[attr] = true;
                        if attr == 0 && part == 3 {
                            self.finish_direct_vertex();
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                m if (NV097_SET_VERTEX_DATA2S..=NV097_SET_VERTEX_DATA2S + 0x3C).contains(&m) => {
                    let base_attr = ((m - NV097_SET_VERTEX_DATA2S) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let attr = if non_inc {
                            base_attr
                        } else {
                            base_attr + i as usize
                        };
                        if attr >= 16 {
                            continue;
                        }
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.direct_attrs[attr][0] = (dw as i16) as f32;
                        self.direct_attrs[attr][1] = ((dw >> 16) as i16) as f32;
                        self.direct_attrs[attr][2] = 0.0;
                        self.direct_attrs[attr][3] = 1.0;
                        self.direct_attr_valid[attr] = true;
                        if attr == 0 {
                            self.finish_direct_vertex();
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                m if (NV097_SET_VERTEX_DATA4UB..=NV097_SET_VERTEX_DATA4UB + 0x3C).contains(&m) => {
                    let base_attr = ((m - NV097_SET_VERTEX_DATA4UB) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let attr = if non_inc {
                            base_attr
                        } else {
                            base_attr + i as usize
                        };
                        if attr >= 16 {
                            continue;
                        }
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.direct_attrs[attr][0] = (dw & 0xFF) as f32 / 255.0;
                        self.direct_attrs[attr][1] = ((dw >> 8) & 0xFF) as f32 / 255.0;
                        self.direct_attrs[attr][2] = ((dw >> 16) & 0xFF) as f32 / 255.0;
                        self.direct_attrs[attr][3] = ((dw >> 24) & 0xFF) as f32 / 255.0;
                        self.direct_attr_valid[attr] = true;
                        if attr == 0 {
                            self.finish_direct_vertex();
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                m if (NV097_SET_VERTEX_DATA4S_M..=NV097_SET_VERTEX_DATA4S_M + 0x7C)
                    .contains(&m) =>
                {
                    let base_method = ((m - NV097_SET_VERTEX_DATA4S_M) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let method_slot = if non_inc {
                            base_method
                        } else {
                            base_method + i as usize
                        };
                        let attr = method_slot / 2;
                        let part = method_slot % 2;
                        if attr >= 16 {
                            continue;
                        }
                        let dw = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        self.direct_attrs[attr][part * 2] =
                            (((dw & 0xFFFF) as i16) as f32 * 2.0 + 1.0) / 65535.0;
                        self.direct_attrs[attr][part * 2 + 1] =
                            (((dw >> 16) as i16) as f32 * 2.0 + 1.0) / 65535.0;
                        self.direct_attr_valid[attr] = true;
                        if attr == 0 && part == 1 {
                            self.finish_direct_vertex();
                        }
                    }
                    NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                // Vertex attribute array setup via direct NV097 method writes.
                // Spider-Man sets the stream source this way rather than via
                // the D3DDevice_SetStreamSource HLE path (which is hooked at
                // a wrong offset in our tree — first bytes 8B 0D E0 =
                // mid-function). The game writes method 0x1720 to set each
                // attribute's base address in guest memory, and 0x1760 to
                // set the format word (type + size + stride).
                //
                // Attribute 0 = POSITION. We map that to STREAM0_VB_ADDR /
                // STREAM0_STRIDE so emit_sequential/emit_indexed can read
                // vertices via the existing decode_stream_vertex path.
                // V04 fix: range-match OFFSET for slots 0..15 (0x1720..=0x175C).
                // Each of the 16 attribute slots can have its own base pointer in
                // guest memory. Previously only 0x1720 matched and only slot 0
                // was stored — slots 1..15 fell through to the histogram and
                // split-stream / multi-offset layouts silently collapsed.
                //
                // Xbox D3D8's D3DVSD_STREAM parses the vertex declaration into
                // per-slot base addresses written here; xemu stores them as
                // `pgraph->vertex_attributes[slot].dma_offset` identically.
                m if (NV097_SET_VERTEX_DATA_ARRAY_OFFSET
                    ..=NV097_SET_VERTEX_DATA_ARRAY_OFFSET + 0x3C)
                    .contains(&m) =>
                {
                    let base_slot = ((m - NV097_SET_VERTEX_DATA_ARRAY_OFFSET) >> 2) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let arg = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        let slot = if non_inc {
                            base_slot
                        } else {
                            base_slot + i as usize
                        };
                        if slot >= 16 {
                            break;
                        }
                        // Strip cache-bypass bit (like hle_set_stream_source does).
                        let vb = arg & 0x1FFF_FFFF;
                        VS_SLOT_OFFSET[slot].store(vb, Ordering::Relaxed);
                        // Keep slot 0 mirrored to STREAM0_VB_ADDR for the legacy
                        // decode_stream_vertex path that only knows about slot 0.
                        if slot == 0 {
                            STREAM0_VB_ADDR.store(vb, Ordering::Relaxed);
                        }
                        static OFF_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = OFF_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 32 {
                            crate::xbox::aot::veh::veh_log(&format!(
                                "[PB-PARSE] VERTEX_DATA_ARRAY_OFFSET[{}]=0x{:08X} (vb=0x{:08X})",
                                slot, arg, vb
                            ));
                        }
                    }
                }

                // Per-slot VERTEX_DATA_ARRAY_FORMAT: one method-offset per slot,
                // 16 slots total at 0x1760, 0x1764, 0x1768, ..., 0x179C.
                // Each slot has its own (type, size, stride). Per xemu this is
                // the authoritative vertex layout — slot 0 is POSITION, slot 2
                // is NORMAL, slot 3 is DIFFUSE, etc. To compute the true total
                // vertex stride for a VS-handle draw (where FVF is unavailable),
                // we sum the per-slot component sizes across enabled slots.
                //
                // Format word layout: bits[0..3]=type, bits[4..7]=size (number
                // of components), bits[8..15]=stride-in-bytes of the WHOLE
                // vertex as seen from this attribute's point of view. xemu
                // documents the stride field as per-attribute but hardware
                // behavior is that all attributes of a vertex share a stride —
                // so we still mirror slot-0's stride to STREAM0_STRIDE for the
                // decode_stream_vertex path.
                m if (NV097_SET_VERTEX_DATA_ARRAY_FORMAT
                    ..=NV097_SET_VERTEX_DATA_ARRAY_FORMAT + 0x3C)
                    .contains(&m) =>
                {
                    let base_slot = ((m - NV097_SET_VERTEX_DATA_ARRAY_FORMAT) / 4) as usize;
                    let base = guest_mem as u64 + args_start as u64;
                    for i in 0..count {
                        let arg = unsafe {
                            std::ptr::read_unaligned((base + (i as u64) * 4) as *const u32)
                        };
                        // In inc-mode, each arg goes to the next slot starting
                        // at base_slot. In no-inc mode, all args go to base_slot.
                        let slot = if non_inc {
                            base_slot
                        } else {
                            base_slot + i as usize
                        };
                        if slot >= 16 {
                            continue;
                        }
                        let attr_type = arg & 0xF;
                        let attr_size = (arg >> 4) & 0xF;
                        // 24-bit stride (xemu NV097_SET_VERTEX_DATA_ARRAY_FORMAT_STRIDE = 0xFFFFFF00)
                        let stride = (arg >> 8) & 0xFF_FFFF;

                        // Record the per-slot format for later stride-sum.
                        VS_SLOT_TYPE[slot].store(attr_type, Ordering::Relaxed);
                        VS_SLOT_SIZE[slot].store(attr_size, Ordering::Relaxed);
                        VS_SLOT_STRIDE[slot].store(stride, Ordering::Relaxed);

                        // Mirror slot-0 stride to STREAM0_STRIDE for the
                        // existing decode path (unchanged behavior for slot 0).
                        if slot == 0 && stride > 0 && stride <= 128 {
                            STREAM0_STRIDE.store(stride, Ordering::Relaxed);
                        }

                        static FMT_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = FMT_LOG.fetch_add(1, Ordering::Relaxed);
                        if n < 256 {
                            crate::xbox::aot::veh::veh_log(&format!(
                                "[VS-SLOT-FORMAT] slot={} type={} size={} stride={} (raw=0x{:08X})",
                                slot, attr_type, attr_size, stride, arg
                            ));
                        }
                    }
                }

                // Catch-all: accumulate a full histogram of method-offset
                // frequencies. Replaces the older bitmap-throttled first-seen
                // log. Every 50K parsed methods, dumps the top-20 hottest
                // offsets so we can see what the game actually writes to.
                //
                // Goal (2026-04-21): find the NV097 offset(s) Spider-Man uses
                // to set stream-source / vertex attributes. With ~166K
                // methods/minute observed, the top-20 hottest offsets
                // should include the stream-source setup path even if
                // mixed in with state writes.
                _ => {
                    if method != 0 || count != 0 {
                        hist_record(method, count);
                    }
                }
            }
        }
        parsed
    }

    fn finish_direct_vertex(&mut self) {
        if !self.direct_attr_valid[0] {
            return;
        }

        let p = self.direct_attrs[0];
        let mut v = NV2AVertex {
            x: p[0],
            y: p[1],
            z: p[2],
            w: p[3],
            color: 0xFFFF_FFFF,
            u: 0.0,
            v: 0.0,
        };

        if self.direct_attr_valid[3] {
            v.color = pack_color_argb(self.direct_attrs[3]);
        }
        if self.direct_attr_valid[9] {
            v.u = self.direct_attrs[9][0];
            v.v = self.direct_attrs[9][1];
        } else if self.direct_attr_valid[8] {
            v.u = self.direct_attrs[8][0];
            v.v = self.direct_attrs[8][1];
        }

        self.direct_verts.push(v);
    }

    fn emit_direct(&mut self, log_verbose: bool) {
        let prim = self.prim;
        let mut verts = std::mem::take(&mut self.direct_verts);
        self.prim = 0;
        if prim == 0 || verts.is_empty() {
            return;
        }

        if !direct_vertex_submit_enabled() {
            static DIRECT_SKIP_LOG: AtomicU32 = AtomicU32::new(0);
            let n = DIRECT_SKIP_LOG.fetch_add(1, Ordering::Relaxed);
            if log_verbose || n < 16 || n.is_power_of_two() {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DIRECT-DRAW-SKIP #{}] prim={}({}) nverts={} reason=env_disabled",
                    n,
                    prim,
                    prim_name(prim),
                    verts.len()
                ));
            }
            return;
        }

        if !verts.iter().any(|v| v.x != 0.0 || v.y != 0.0 || v.z != 0.0) {
            return;
        }

        let final_prim: i32 = if prim == NV_PRIM_QUADS {
            verts = expand_quads_to_tris(&verts);
            NV097_TRIANGLES
        } else if prim == NV_PRIM_QUAD_STRIP {
            verts = expand_quad_strip_to_tris(&verts);
            NV097_TRIANGLES
        } else {
            map_nv_prim(prim)
        };

        static DIRECT_LOG: AtomicU32 = AtomicU32::new(0);
        let n = DIRECT_LOG.fetch_add(1, Ordering::Relaxed);
        if log_verbose || n < 16 || n.is_power_of_two() {
            let decoded = verts
                .iter()
                .take(4)
                .map(|v| {
                    format!(
                        "({:.3},{:.3},{:.3},{:.3};c=0x{:08X};uv={:.4},{:.4})",
                        v.x, v.y, v.z, v.w, v.color, v.u, v.v
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            crate::xbox::aot::veh::veh_log(&format!(
                "[PB-DIRECT-DRAW #{}] prim={}({}) nverts={} decoded=[{}]",
                n,
                prim,
                prim_name(prim),
                verts.len(),
                decoded
            ));
        }

        submit_pb_draw(verts, final_prim);
        set_frame_dirty();
        self.draws_emitted = self.draws_emitted.saturating_add(1);
    }

    /// Drain the inline accumulator into a queued draw. Called when we see
    /// SET_BEGIN_END(0). No-op if there's no open batch or no data.
    fn emit_inline(&mut self, log_verbose: bool) {
        let prim = self.prim;
        let buf = std::mem::take(&mut self.inline);
        self.prim = 0;
        if prim == 0 || buf.is_empty() {
            return;
        }
        // Count this as a real draw-method submission. emit_inline fires once
        // per `SET_BEGIN_END(NONE)` close — i.e. one batch of geometry.
        NV2A_DRAW_METHODS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let fvf_raw = VERTEX_SHADER.load(Ordering::Relaxed);
        let fvf = fvf_from_vertex_shader_value(fvf_raw);
        let stride_guess = STREAM0_STRIDE.load(Ordering::Relaxed);
        let inline_stride = infer_inline_stride(prim, buf.len());
        // Prefer FVF-derived stride (more authoritative); fall back to the
        // INLINE_ARRAY payload shape before any stream-source state. INLINE_ARRAY
        // is self-contained; it must not inherit STREAM0_STRIDE from a prior
        // streamed/indexed draw. Spider-Man's selector path leaves stream stride
        // at 44, while later 2D overlay quads still carry 24-byte inline vertices.
        let mut stride = {
            let s = fvf_stride(fvf);
            if s > 0 {
                s
            } else if inline_stride > 0 {
                inline_stride
            } else if stream_has_format_captured() {
                // Xbox-programmable VS: sum per-slot component bytes from
                // captured NV097_SET_VERTEX_DATA_ARRAY_FORMAT writes.
                let vs_sum = compute_stride_for_stream(0);
                if vs_sum > 0 {
                    vs_sum
                } else {
                    stride_guess
                }
            } else {
                stride_guess
            }
        };
        // 2026-04-24 fallback: when SetVertexShader names a programmable VS
        // object (bit 0 set) instead of an FVF value, fvf_stride() may be 0.
        // Derive stride from inline buffer size / primitive minimum vertex
        // count in that case.
        //
        // NOTE: V-06 "prefer smaller stride" heuristic was tried and
        // REVERTED — produced NaN/Inf in decode for Spider-Man glyphs
        // because picking stride=12 over the real stride=24 caused odd-
        // indexed reads to hit color/UV bits as float positions. The
        // original stride = buf.len()/min_v computation IS correct for
        // well-formed QUADS — it yields 4 verts (for one quad) or 8
        // verts (for two quads) based on buffer size. The real bug is
        // upstream: Spider-Man's format may need per-attribute parsing
        // per xemu's VERTEX_DATA_ARRAY_FORMAT table (V-05 finding).
        if stride == 0 && !buf.is_empty() {
            let min_v = primitive_min_verts(prim);
            if min_v > 0 && buf.len() % min_v == 0 {
                let guess_dw = buf.len() / min_v;
                let guess_bytes = (guess_dw * 4) as u32;
                if matches!(guess_bytes, 12 | 16 | 20 | 24 | 28 | 32 | 36 | 40) {
                    stride = guess_bytes;
                }
            }
        }
        if inline_stride > 0 && stride_guess > 0 && inline_stride != stride_guess {
            static INLINE_STRIDE_OVERRIDE_LOG: AtomicU32 = AtomicU32::new(0);
            let ov_n = INLINE_STRIDE_OVERRIDE_LOG.fetch_add(1, Ordering::Relaxed);
            if ov_n < 16 || ov_n.is_power_of_two() {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[INLINE-STRIDE-OVERRIDE #{}] prim={} buf_dw={} stream_stride={} inline_stride={} fvf=0x{:X}",
                    ov_n,
                    prim,
                    buf.len(),
                    stride_guess,
                    inline_stride,
                    fvf
                ));
            }
        }
        if stride == 0 || stride > 128 {
            crate::xbox::aot::veh::veh_log(&format!(
                "[PB-DRAW] skip prim={} buf_dw={} fvf=0x{:X}: no usable stride",
                prim,
                buf.len(),
                fvf
            ));
            return;
        }
        let stride_dw = (stride / 4) as usize;
        if stride_dw == 0 {
            return;
        }
        let nverts = buf.len() / stride_dw;
        if nverts < primitive_min_verts(prim) {
            return;
        }

        // 2026-04-24 INLINE-DBG: log first 10 batches' raw state so we can
        // tell whether batches 2..30 are identical (stuck source) or vary
        // (real geometry). Compares prim/fvf/stride/length and head/tail dwords.
        static INLINE_DBG_N: AtomicU32 = AtomicU32::new(0);
        let n = INLINE_DBG_N.fetch_add(1, Ordering::Relaxed);
        if n < 10 {
            let head: Vec<String> = buf.iter().take(4).map(|d| format!("0x{:08X}", d)).collect();
            let tail_start = buf.len().saturating_sub(4);
            let tail: Vec<String> = buf[tail_start..]
                .iter()
                .map(|d| format!("0x{:08X}", d))
                .collect();
            crate::xbox::aot::veh::veh_log(&format!(
                "[INLINE-DBG #{}] prim={} fvf=0x{:X} stride={} inline_dw={} first_dws=[{}] last_dws=[{}]",
                n, prim, fvf, stride, buf.len(), head.join(","), tail.join(",")
            ));
        }

        let mut verts: Vec<NV2AVertex> = (0..nverts)
            .map(|i| decode_inline_vertex(&buf, i * stride_dw, stride, fvf))
            .collect();

        {
            let (min_x, max_x, min_y, max_y) = verts.iter().fold(
                (
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                ),
                |(min_x, max_x, min_y, max_y), v| {
                    (
                        min_x.min(v.x),
                        max_x.max(v.x),
                        min_y.min(v.y),
                        max_y.max(v.y),
                    )
                },
            );
            if n >= 10 && (n.is_power_of_two() || n % 4096 == 0) {
                let raw: Vec<String> = buf
                    .iter()
                    .take(12)
                    .map(|d| format!("0x{:08X}", d))
                    .collect();
                let decoded = verts
                    .iter()
                    .take(6)
                    .map(|v| {
                        format!(
                            "({:.3},{:.3},{:.3};c=0x{:08X};uv={:.4},{:.4})",
                            v.x, v.y, v.z, v.color, v.u, v.v
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                crate::xbox::aot::veh::veh_log(&format!(
                    "[INLINE-DBG-SPARSE #{}] prim={} fvf=0x{:X} stride={} nverts={} bbox=[{:.3},{:.3}..{:.3},{:.3}] raw=[{}] decoded=[{}]",
                    n,
                    prim,
                    fvf,
                    stride,
                    nverts,
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    raw.join(","),
                    decoded
                ));
            }
            if stride == 24 && (max_x - min_x).abs() <= 4.0 && (max_y - min_y).abs() <= 4.0 {
                static TINY_INLINE_LOG: AtomicU32 = AtomicU32::new(0);
                let tiny_n = TINY_INLINE_LOG.fetch_add(1, Ordering::Relaxed);
                if tiny_n < 16 || tiny_n.is_power_of_two() {
                    let raw: Vec<String> = buf
                        .iter()
                        .take(24)
                        .map(|d| format!("0x{:08X}", d))
                        .collect();
                    let decoded = verts
                        .iter()
                        .take(8)
                        .map(|v| {
                            format!(
                                "({:.3},{:.3},{:.3};c=0x{:08X};uv={:.4},{:.4})",
                                v.x, v.y, v.z, v.color, v.u, v.v
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    crate::xbox::aot::veh::veh_log(&format!(
                        "[INLINE-TINY #{}] prim={} stride={} nverts={} bbox=[{:.3},{:.3}..{:.3},{:.3}] raw=[{}] decoded=[{}]",
                        tiny_n,
                        prim,
                        stride,
                        nverts,
                        min_x,
                        min_y,
                        max_x,
                        max_y,
                        raw.join(","),
                        decoded
                    ));
                }
            }
        }

        // Skip all-zero batches (sentinel writes from boot / VB init).
        if !verts.iter().any(|v| v.x != 0.0 || v.y != 0.0 || v.z != 0.0) {
            static ZERO_INLINE_LOG: AtomicU32 = AtomicU32::new(0);
            let zero_n = ZERO_INLINE_LOG.fetch_add(1, Ordering::Relaxed);
            if zero_n < 16 || n.is_power_of_two() {
                let raw: Vec<String> = buf
                    .iter()
                    .take(24)
                    .map(|d| format!("0x{:08X}", d))
                    .collect();
                crate::xbox::aot::veh::veh_log(&format!(
                    "[INLINE-ZERO-SKIP #{}] inline={} prim={} fvf=0x{:X} stride={} nverts={} raw=[{}]",
                    zero_n,
                    n,
                    prim,
                    fvf,
                    stride,
                    nverts,
                    raw.join(",")
                ));
            }
            return;
        }

        let final_prim: i32 = if prim == NV_PRIM_QUADS {
            verts = expand_quads_to_tris(&verts);
            NV097_TRIANGLES
        } else if prim == NV_PRIM_QUAD_STRIP {
            verts = expand_quad_strip_to_tris(&verts);
            NV097_TRIANGLES
        } else {
            map_nv_prim(prim)
        };

        if log_verbose {
            crate::xbox::aot::veh::veh_log(&format!(
                "[PB-DRAW] prim={}({}) nverts={} stride={} fvf=0x{:X} queue_draw",
                prim,
                prim_name(prim),
                verts.len(),
                stride,
                fvf
            ));
        }

        submit_pb_draw(verts, final_prim);
        set_frame_dirty();
        self.draws_emitted = self.draws_emitted.saturating_add(1);
    }

    /// DRAW_ARRAYS path — reads from the bound stream source.
    fn emit_sequential(&mut self, guest_mem: *mut u8, vstart: u32, vcount: u32, log_verbose: bool) {
        if self.prim == 0 {
            return;
        }
        // Mask 2026-04-20: defensive strip of 0x80000000 cache-bypass bit.
        // Publisher (hle_set_stream_source) also masks, this is belt+braces.
        let vb = STREAM0_VB_ADDR.load(Ordering::Relaxed) & 0x1FFF_FFFF;
        let stride = STREAM0_STRIDE.load(Ordering::Relaxed);
        let vs_raw = VERTEX_SHADER.load(Ordering::Relaxed);
        let fvf = fvf_from_vertex_shader_value(vs_raw);
        if vb == 0 || stride == 0 || stride > 128 {
            if log_verbose {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DRAW] DRAW_ARRAYS skip: vb=0x{:X} stride={}",
                    vb, stride
                ));
            }
            return;
        }
        const MAX_VERTS: u32 = 65_536;
        let vcount = vcount.min(MAX_VERTS);
        // V04 read-side gate: use multi-stream decoder iff slots ≥1 have
        // captured bases. Falls back to classic single-stream path for
        // every FVF / single-slot game (unchanged behavior).
        let use_multi = stream_has_format_captured() && any_slot_ge1_has_offset();
        if spidey_composite_vs(vs_raw) {
            static PATH_LOG: AtomicU32 = AtomicU32::new(0);
            let n = PATH_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 64 {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DRAW-DECODE-PATH] #{} kind=DRAW_ARRAYS vs=0x{:08X} use_multi={} start={} count={} stream0=0x{:08X} stride={}{}",
                    n,
                    vs_raw,
                    use_multi,
                    vstart,
                    vcount,
                    vb,
                    stride,
                    spidey_format_vertex_slots()
                ));
            }
        }
        let mut verts: Vec<NV2AVertex> = (0..vcount)
            .map(|i| {
                if use_multi {
                    decode_stream_vertex_multi(guest_mem, vstart + i)
                } else {
                    decode_stream_vertex(guest_mem, vb, stride, vstart + i, fvf)
                }
            })
            .collect();
        if !verts.iter().any(|v| v.x != 0.0 || v.y != 0.0 || v.z != 0.0) {
            return;
        }
        let final_prim: i32 = if self.prim == NV_PRIM_QUADS {
            verts = expand_quads_to_tris(&verts);
            NV097_TRIANGLES
        } else if self.prim == NV_PRIM_QUAD_STRIP {
            verts = expand_quad_strip_to_tris(&verts);
            NV097_TRIANGLES
        } else {
            map_nv_prim(self.prim)
        };
        if log_verbose {
            crate::xbox::aot::veh::veh_log(&format!(
                "[PB-DRAW] DRAW_ARRAYS prim={}({}) start={} count={} stride={}",
                self.prim,
                prim_name(self.prim),
                vstart,
                vcount,
                stride
            ));
        }
        submit_pb_draw(verts, final_prim);
        set_frame_dirty();
        self.draws_emitted = self.draws_emitted.saturating_add(1);
    }

    /// Indexed-draw emitter. Drains `self.indices`, reads each vertex from
    /// the bound stream source via `decode_stream_vertex`, applies primitive
    /// translation (quads → tris), and emits via `queue_draw`.
    ///
    /// Added 2026-04-21 to wire the previously-unused
    /// NV097_ARRAY_ELEMENT16/32 match arm. Spider-Man
    /// uses indexed draws almost exclusively for world geometry.
    fn emit_indexed(&mut self, guest_mem: *mut u8, log_verbose: bool) {
        let prim = self.prim;
        let indices = std::mem::take(&mut self.indices);
        self.prim = 0;
        if prim == 0 || indices.is_empty() {
            return;
        }

        let vb = STREAM0_VB_ADDR.load(Ordering::Relaxed) & 0x1FFF_FFFF;
        let stride = STREAM0_STRIDE.load(Ordering::Relaxed);
        let vs_raw = VERTEX_SHADER.load(Ordering::Relaxed);
        let fvf = fvf_from_vertex_shader_value(vs_raw);

        if vb == 0 || stride == 0 || stride > 128 {
            if log_verbose {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DRAW] INDEXED skip prim={} n_idx={}: vb=0x{:X} stride={}",
                    prim,
                    indices.len(),
                    vb,
                    stride
                ));
            }
            return;
        }

        // Cap pathologically-large batches.
        const MAX_IDX: usize = 128 * 1024;
        let indices = if indices.len() > MAX_IDX {
            if log_verbose {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DRAW] INDEXED capping {} → {}",
                    indices.len(),
                    MAX_IDX
                ));
            }
            indices[..MAX_IDX].to_vec()
        } else {
            indices
        };

        // V04 read-side gate: same logic as emit_sequential.
        let use_multi = stream_has_format_captured() && any_slot_ge1_has_offset();
        if spidey_composite_vs(vs_raw) {
            static PATH_LOG: AtomicU32 = AtomicU32::new(0);
            let n = PATH_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 64 {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[PB-DRAW-DECODE-PATH] #{} kind=INDEXED vs=0x{:08X} use_multi={} n_idx={} stream0=0x{:08X} stride={}{}",
                    n,
                    vs_raw,
                    use_multi,
                    indices.len(),
                    vb,
                    stride,
                    spidey_format_vertex_slots()
                ));
            }
        }
        let mut verts: Vec<NV2AVertex> = indices
            .iter()
            .map(|&idx| {
                if use_multi {
                    decode_stream_vertex_multi(guest_mem, idx)
                } else {
                    decode_stream_vertex(guest_mem, vb, stride, idx, fvf)
                }
            })
            .collect();

        // Skip all-zero batches (boot-time sentinel writes).
        if !verts.iter().any(|v| v.x != 0.0 || v.y != 0.0 || v.z != 0.0) {
            return;
        }

        let final_prim: i32 = if prim == NV_PRIM_QUADS {
            verts = expand_quads_to_tris(&verts);
            NV097_TRIANGLES
        } else if prim == NV_PRIM_QUAD_STRIP {
            verts = expand_quad_strip_to_tris(&verts);
            NV097_TRIANGLES
        } else {
            map_nv_prim(prim)
        };

        if log_verbose {
            crate::xbox::aot::veh::veh_log(&format!(
                "[PB-DRAW] INDEXED prim={}({}) n_idx={} stride={} fvf=0x{:X} queue_draw",
                prim,
                prim_name(prim),
                verts.len(),
                stride,
                fvf
            ));
        }

        submit_pb_draw(verts, final_prim);
        set_frame_dirty();
        self.draws_emitted = self.draws_emitted.saturating_add(1);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn fvf_stride(fvf: u32) -> u32 {
    let pos = fvf & FVF_POSITION_MASK;
    let mut s: u32 = match pos {
        p if p == FVF_XYZ => 12,
        p if p == FVF_XYZRHW => 16,
        p if p == FVF_XYZB1 => 16,
        p if p == FVF_XYZB2 => 20,
        _ => return 0,
    };
    if fvf & FVF_NORMAL != 0 {
        s += 12;
    }
    if fvf & FVF_DIFFUSE != 0 {
        s += 4;
    }
    if fvf & FVF_SPECULAR != 0 {
        s += 4;
    }
    let tex = (fvf & FVF_TEXCOUNT_MASK) >> FVF_TEXCOUNT_SHIFT;
    s + tex * 8 // each texcoord set = 2 floats
}

fn primitive_min_verts(prim: u32) -> usize {
    match prim {
        NV_PRIM_POINTS => 1,
        NV_PRIM_LINES | NV_PRIM_LINE_LOOP | NV_PRIM_LINE_STRIP => 2,
        NV_PRIM_TRIANGLES | NV_PRIM_TRIANGLE_STRIP | NV_PRIM_TRIANGLE_FAN => 3,
        NV_PRIM_QUADS | NV_PRIM_QUAD_STRIP => 4,
        NV_PRIM_POLYGON => 3,
        _ => 1,
    }
}

fn infer_inline_stride(prim: u32, buf_dwords: usize) -> u32 {
    let min_v = primitive_min_verts(prim);
    if min_v == 0 || buf_dwords == 0 {
        return 0;
    }

    // INLINE_ARRAY is raw vertex payload. Prefer larger common layouts first
    // so a 24-dword quad packet is decoded as 4 x 24-byte vertices, not as
    // 6 x 16-byte or 8 x 12-byte pseudo-vertices.
    const CANDIDATE_BYTES: &[u32] = &[40, 36, 32, 28, 24, 20, 16, 12];
    for &bytes in CANDIDATE_BYTES {
        let stride_dw = (bytes / 4) as usize;
        if stride_dw == 0 || buf_dwords % stride_dw != 0 {
            continue;
        }
        let nverts = buf_dwords / stride_dw;
        if nverts >= min_v {
            return bytes;
        }
    }
    0
}

fn prim_name(prim: u32) -> &'static str {
    match prim {
        NV_PRIM_POINTS => "POINTS",
        NV_PRIM_LINES => "LINES",
        NV_PRIM_LINE_LOOP => "LINE_LOOP",
        NV_PRIM_LINE_STRIP => "LINE_STRIP",
        NV_PRIM_TRIANGLES => "TRIS",
        NV_PRIM_TRIANGLE_STRIP => "TRI_STRIP",
        NV_PRIM_TRIANGLE_FAN => "TRI_FAN",
        NV_PRIM_QUADS => "QUADS",
        NV_PRIM_QUAD_STRIP => "QUAD_STRIP",
        NV_PRIM_POLYGON => "POLYGON",
        _ => "?",
    }
}

// ---------------------------------------------------------------------------
// NV2A per-slot format decode (xemu-style, LLE observation)
//
// When the guest uses a programmable vertex shader (VS handle, not FVF), the
// Xbox D3D8 runtime parses the D3DVSD_* declaration into a 16-slot attribute
// table and programs it into NV2A via NV097_SET_VERTEX_DATA_ARRAY_FORMAT
// (0x1760..0x179C) and NV097_SET_VERTEX_DATA_ARRAY_OFFSET (0x1720..0x175C).
// VS_SLOT_TYPE / VS_SLOT_SIZE capture that table. These helpers decode the
// table back into per-vertex stride and per-attribute floats.
//
// Type codes (xemu pgraph.c handle_vertex_data_array_format):
//   0 = UB_D3D  (u8, 1 byte per component, BGRA wire order, normalize /255)
//   1 = S1      (i16, 2 bytes, normalize /32767 clamped to [-1, 1])
//   2 = F       (f32, 4 bytes, raw)
//   4 = UB_OGL  (u8, 1 byte, RGBA wire order, normalize /255)
//   5 = S32K    (i16, 2 bytes, unnormalized raw)
//   6 = CMP     (packed 11/11/10 signed, fixed 4 bytes regardless of size)
// ---------------------------------------------------------------------------

/// Bytes per component for a given NV2A type code. Returns 0 for unknown
/// or special types (CMP — caller special-cases to 4 bytes fixed).
fn element_size_nv2a(type_byte: u32) -> u32 {
    match type_byte {
        0 | 4 => 1, // UB_D3D / UB_OGL
        1 | 5 => 2, // S1 / S32K
        2 => 4,     // F
        6 => 0,     // CMP — fixed 4 bytes, independent of size (callers special-case)
        _ => 0,
    }
}

/// Bytes consumed by one attribute of one vertex.
fn slot_byte_size_nv2a(type_byte: u32, size: u32) -> u32 {
    if type_byte == 6 {
        4 // CMP is always 4 bytes (packed 11/11/10)
    } else {
        element_size_nv2a(type_byte) * size
    }
}

/// True if any attribute slot has a captured non-NONE format.
/// When false, callers should fall back to FVF / stride heuristics.
pub fn stream_has_format_captured() -> bool {
    (0..16).any(|s| {
        let ty = VS_SLOT_TYPE[s].load(Ordering::Relaxed);
        let sz = VS_SLOT_SIZE[s].load(Ordering::Relaxed);
        // A slot is "enabled" if size > 0; type byte alone is ambiguous
        // since type=0 (UB_D3D) is a legitimate value but requires size>0.
        let _ = ty;
        sz > 0
    })
}

/// V03 fix: compute per-vertex stride preferring the AUTHORITATIVE
/// VS_SLOT_STRIDE field (bits 8..31 of NV097_SET_VERTEX_DATA_ARRAY_FORMAT)
/// over the sum-of-sizes heuristic.
///
/// Rationale: xemu stores `pgraph->vertex_attributes[slot].stride` from the
/// FORMAT word's high 24 bits; every enabled slot programs the same stride
/// value (= full vertex stride, since all attributes live in the same
/// interleaved buffer). Summing component sizes silently under-computes
/// when the layout has inter-attribute padding (e.g. the NGL 40-byte
/// vertex = XYZRHW+DIFFUSE+TEX0+TEX1+pad where sum-of-sizes = 36).
///
/// Strategy: use max(slot_stride[s] for s where slot_stride[s] > 0); fall
/// back to sum-of-sizes only if NO slot has a captured stride (early in
/// boot before FORMAT writes land). Returns 0 when no slots are enabled.
pub fn compute_stride_for_stream(stream_index: u32) -> u32 {
    let _ = stream_index;
    let mut total: u32 = 0;
    let mut max_slot_stride: u32 = 0;
    let mut enabled: u32 = 0;
    let mut slot_sizes = [0u32; 16];
    let mut slot_strides = [0u32; 16];
    for slot in 0..16 {
        let sz = VS_SLOT_SIZE[slot].load(Ordering::Relaxed);
        if sz == 0 {
            continue;
        }
        let ty = VS_SLOT_TYPE[slot].load(Ordering::Relaxed);
        let bytes = slot_byte_size_nv2a(ty, sz);
        if bytes == 0 {
            continue;
        }
        total += bytes;
        slot_sizes[slot] = bytes;
        let slot_stride = VS_SLOT_STRIDE[slot].load(Ordering::Relaxed);
        slot_strides[slot] = slot_stride;
        if slot_stride > max_slot_stride {
            max_slot_stride = slot_stride;
        }
        enabled += 1;
    }
    // Prefer authoritative stride when available (xemu-consistent).
    // Guard against implausible values (> 256 bytes/vert = anomalous).
    let chosen = if max_slot_stride > 0 && max_slot_stride <= 256 && max_slot_stride >= total {
        max_slot_stride
    } else {
        total
    };
    static STR_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = STR_LOG.fetch_add(1, Ordering::Relaxed);
    if n < 64 && enabled > 0 {
        let mut parts = String::new();
        for (i, &sz) in slot_sizes.iter().enumerate() {
            if sz > 0 {
                if !parts.is_empty() {
                    parts.push(' ');
                }
                parts.push_str(&format!("{}:sz{}/s{}", i, sz, slot_strides[i]));
            }
        }
        crate::xbox::aot::veh::veh_log(&format!(
            "[VS-STRIDE-COMPUTED] stride={} (sum={}, max_slot_stride={}) slots={} ({})",
            chosen, total, max_slot_stride, enabled, parts
        ));
    }
    chosen
}

/// Decode one attribute of one vertex to a 4-component float.
/// Missing components default to {0, 0, 0, 1}.
fn decode_attribute_nv2a(type_byte: u32, size: u32, bytes: &[u8]) -> [f32; 4] {
    let mut out = [0.0f32, 0.0, 0.0, 1.0];
    let need = slot_byte_size_nv2a(type_byte, size) as usize;
    if need == 0 || bytes.len() < need || size == 0 || size > 4 {
        return out;
    }
    match type_byte {
        2 => {
            // F32
            for i in 0..size as usize {
                let b = &bytes[i * 4..i * 4 + 4];
                out[i] = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
            }
        }
        0 => {
            // UB_D3D — wire bytes are B, G, R, A; shader wants R, G, B, A
            // (Xbox D3DCOLOR is ARGB in DWORD, little-endian = B,G,R,A memory)
            out[0] = bytes[2] as f32 / 255.0;
            out[1] = bytes[1] as f32 / 255.0;
            out[2] = bytes[0] as f32 / 255.0;
            out[3] = bytes[3] as f32 / 255.0;
        }
        4 => {
            // UB_OGL — straight RGBA order
            for i in 0..size as usize {
                out[i] = bytes[i] as f32 / 255.0;
            }
        }
        1 => {
            // S1 — normalized signed short
            for i in 0..size as usize {
                let s = i16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
                out[i] = (s as f32 / 32767.0).max(-1.0);
            }
        }
        5 => {
            // S32K — raw signed short (unnormalized)
            for i in 0..size as usize {
                let s = i16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
                out[i] = s as f32;
            }
        }
        6 => {
            // CMP — 11/11/10 signed packed normal
            let packed = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let sx = ((packed & 0x7FF) << 21) as i32 >> 21;
            let sy = (((packed >> 11) & 0x7FF) << 21) as i32 >> 21;
            let sz = (((packed >> 22) & 0x3FF) << 22) as i32 >> 22;
            out[0] = (sx as f32 / 1023.0).max(-1.0);
            out[1] = (sy as f32 / 1023.0).max(-1.0);
            out[2] = (sz as f32 / 511.0).max(-1.0);
        }
        _ => {
            // Unknown type — return default. Throttled log.
            static UNK_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = UNK_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 16 {
                crate::xbox::aot::veh::veh_log(&format!(
                    "[VS-ATTR-DECODED] unknown type={} size={}",
                    type_byte, size
                ));
            }
        }
    }
    out
}

/// Pack a 4-component float (RGBA in [0,1]) back into a u32 0xAARRGGBB.
fn pack_color_argb(rgba: [f32; 4]) -> u32 {
    let r = (rgba[0].clamp(0.0, 1.0) * 255.0) as u32;
    let g = (rgba[1].clamp(0.0, 1.0) * 255.0) as u32;
    let b = (rgba[2].clamp(0.0, 1.0) * 255.0) as u32;
    let a = (rgba[3].clamp(0.0, 1.0) * 255.0) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}

fn sanitize_inline_z(z: f32, stride_dw: usize, raw: u32) -> f32 {
    // Spider-Man's menu/HUD inline quads use a 24-byte screen-space layout
    // where dword 2 is not a clip-space Z. Passing values like 0x49CC9C63
    // through to the D3D11 passthrough shader leaves otherwise valid quads
    // clipped or depth-rejected, so keep implausible Z on the 2D plane.
    if z.is_finite() && z.abs() <= 1.0 {
        return z;
    }

    if stride_dw == 6 {
        use std::sync::atomic::{AtomicU32, Ordering};
        static CLAMP_LOG: AtomicU32 = AtomicU32::new(0);
        let n = CLAMP_LOG.fetch_add(1, Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[VDEC-Z-CLAMP #{}] stride_dw={} raw=0x{:08X} z={:.3} -> 0",
                n, stride_dw, raw, z
            ));
        }
    }

    0.0
}

/// Decode one vertex from a byte-oriented slice using the captured per-slot
/// attribute table. Fixed-function declarations use Xbox slots
/// 0=POSITION/3=DIFFUSE/9=TEX0; programmable declarations use shader input
/// registers, where Spider-Man's compositor is v0=POSITION/v1=COLOR/v3=TEX0.
/// Returns default NV2AVertex if no slots are enabled.
fn decode_vertex_from_slots(bytes: &[u8]) -> NV2AVertex {
    let mut v = NV2AVertex::default();
    v.color = 0xFFFF_FFFF;
    v.w = 1.0;
    let vs_raw = VERTEX_SHADER.load(Ordering::Relaxed);
    let programmable_vs = vertex_shader_value_is_programmable(vs_raw);

    let mut offset: usize = 0;
    for slot in 0..16 {
        let sz = VS_SLOT_SIZE[slot].load(Ordering::Relaxed);
        if sz == 0 {
            continue;
        }
        let ty = VS_SLOT_TYPE[slot].load(Ordering::Relaxed);
        let attr_bytes = slot_byte_size_nv2a(ty, sz) as usize;
        if attr_bytes == 0 || offset + attr_bytes > bytes.len() {
            break;
        }
        let slice = &bytes[offset..offset + attr_bytes];
        let vals = decode_attribute_nv2a(ty, sz, slice);
        match (programmable_vs, slot) {
            (_, 0) => {
                // POSITION / v0.
                v.x = vals[0];
                v.y = vals[1];
                v.z = sanitize_inline_z(vals[2], bytes.len() / 4, 0);
                if sz >= 4 {
                    v.w = vals[3];
                }
            }
            (true, 1) => {
                // Programmable vertex shaders read color from v1. Spider-Man's
                // compositor declaration is v0=pos, v1=color, v3=tex0, v4=tex1;
                // treating v3 as fixed-function DIFFUSE corrupts RT sampling.
                if ty == 0 || ty == 4 {
                    v.color = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]);
                } else {
                    v.color = pack_color_argb(vals);
                }
            }
            (true, 3) => {
                // TEX0 / v3 for shader-handle declarations.
                v.u = vals[0];
                v.v = if sz >= 2 { vals[1] } else { 0.0 };
            }
            (false, 3) => {
                // Fixed-function DIFFUSE.
                v.color = pack_color_argb(vals);
            }
            (false, 9) => {
                // Fixed-function TEX0.
                v.u = vals[0];
                v.v = if sz >= 2 { vals[1] } else { 0.0 };
            }
            _ => {
                // NORMAL, SPECULAR, TEX1+, and shader v2/v4+ are carried by
                // richer payload paths when needed. NV2AVertex only has tex0.
            }
        }
        offset += attr_bytes;
    }
    v
}

fn map_nv_prim(prim: u32) -> i32 {
    match prim {
        NV_PRIM_POINTS => NV097_POINTS,
        NV_PRIM_LINES => NV097_LINES,
        NV_PRIM_LINE_LOOP => NV097_LINE_LOOP,
        NV_PRIM_LINE_STRIP => NV097_LINE_STRIP,
        NV_PRIM_TRIANGLES => NV097_TRIANGLES,
        NV_PRIM_TRIANGLE_STRIP => NV097_TRIANGLE_STRIP,
        NV_PRIM_TRIANGLE_FAN => NV097_TRIANGLE_FAN,
        NV_PRIM_POLYGON => NV097_TRIANGLE_FAN, // approximate
        _ => NV097_TRIANGLES,
    }
}

fn decode_inline_vertex(buf: &[u32], start: usize, stride: u32, fvf: u32) -> NV2AVertex {
    // [VDEC] Diagnostic probe: rate-limited to first 32 calls. Dumps stride
    // decision, slot-format capture state, and raw dwords 0-3 at start. Lets
    // us pin down whether HUD draws take the VS-slot path or fallback, what
    // strides/types are captured per-slot, and what raw bytes feed the
    // decoder. Strip after V03/V04 fix confirmed.
    {
        use std::sync::atomic::{AtomicU32, Ordering};
        static VDEC_COUNT: AtomicU32 = AtomicU32::new(0);
        let n = VDEC_COUNT.fetch_add(1, Ordering::Relaxed);
        if n < 32 {
            let captured = stream_has_format_captured();
            let s0_type = VS_SLOT_TYPE[0].load(Ordering::Relaxed);
            let s0_size = VS_SLOT_SIZE[0].load(Ordering::Relaxed);
            let s0_stride = VS_SLOT_STRIDE[0].load(Ordering::Relaxed);
            let s3_enabled = VS_SLOT_SIZE[3].load(Ordering::Relaxed) > 0;
            let s9_enabled = VS_SLOT_SIZE[9].load(Ordering::Relaxed) > 0;
            let dw0 = buf.get(start).copied().unwrap_or(0);
            let dw1 = buf.get(start + 1).copied().unwrap_or(0);
            let dw2 = buf.get(start + 2).copied().unwrap_or(0);
            let dw3 = buf.get(start + 3).copied().unwrap_or(0);
            let stride_dw = (stride / 4) as usize;
            crate::xbox::emulator::debug_log(&format!(
                "[VDEC] #{} fvf=0x{:X} stride={} stride_dw={} captured={} s0={{t:{},sz:{},str:{}}} s3_en={} s9_en={} dw0=0x{:08X} dw1=0x{:08X} dw2=0x{:08X} dw3=0x{:08X}",
                n, fvf, stride, stride_dw, captured, s0_type, s0_size, s0_stride, s3_enabled, s9_enabled, dw0, dw1, dw2, dw3
            ));
        }
    }

    // VS-slot path: when FVF is zero (programmable VS) and the per-slot format
    // table has been populated by NV097_SET_VERTEX_DATA_ARRAY_FORMAT writes,
    // decode using the authoritative per-attribute types instead of the stride
    // heuristic. This fixes Spider-Man's garbage vertex color / NaN positions
    // that came from reading color/UV bytes as raw floats.
    if stream_has_format_captured() {
        let stride_dw = (stride / 4) as usize;
        if stride_dw > 0 && start + stride_dw <= buf.len() {
            let dw_slice = &buf[start..start + stride_dw];
            let byte_ptr = dw_slice.as_ptr() as *const u8;
            let byte_slice = unsafe { std::slice::from_raw_parts(byte_ptr, stride_dw * 4) };
            return decode_vertex_from_slots(byte_slice);
        }
    }

    let mut v = NV2AVertex::default();
    v.color = 0xFFFF_FFFF;
    v.w = 1.0;

    let stride_dw = (stride / 4) as usize;
    let read_f32 = |idx: usize| -> f32 { buf.get(idx).copied().map(f32::from_bits).unwrap_or(0.0) };
    let read_u32 = |idx: usize| -> u32 { buf.get(idx).copied().unwrap_or(0) };

    let pos = fvf & FVF_POSITION_MASK;
    if pos == FVF_XYZ || pos == FVF_XYZRHW || pos == FVF_XYZB1 || pos == FVF_XYZB2 {
        let mut cur = start;
        v.x = read_f32(cur);
        cur += 1;
        v.y = read_f32(cur);
        cur += 1;
        v.z = read_f32(cur);
        cur += 1;
        if pos == FVF_XYZRHW {
            v.w = read_f32(cur);
            cur += 1;
        }
        if pos == FVF_XYZB1 {
            cur += 1;
        }
        if pos == FVF_XYZB2 {
            cur += 2;
        }
        if fvf & FVF_NORMAL != 0 {
            cur += 3;
        }
        if fvf & FVF_DIFFUSE != 0 {
            v.color = read_u32(cur);
            cur += 1;
        }
        if fvf & FVF_SPECULAR != 0 {
            cur += 1;
        }
        let tex = (fvf & FVF_TEXCOUNT_MASK) >> FVF_TEXCOUNT_SHIFT;
        if tex >= 1 {
            v.u = read_f32(cur);
            cur += 1;
            v.v = read_f32(cur);
            let _ = cur;
        }
        return v;
    }

    // Stride-heuristic fallback (no FVF position bits set, e.g. vertex shader
    // handle). Read XY from first 2 floats; read Z only if stride is wide
    // enough to include a dedicated Z slot.
    //
    // 2026-04-24 fix: Spider-Man HUD uses 12-byte verts = XY + COLOR (no Z).
    // Previously we read the 3rd dword as Z, which put the color bytes
    // (e.g. 0x44BCBEDD = 1509.77 as f32) into the Z coordinate. D3D11
    // clip-space rejection killed every vertex. Now stride_dw==3 is
    // recognized as XY+color with Z defaulting to 0.
    if stride_dw >= 2 {
        v.x = read_f32(start);
        v.y = read_f32(start + 1);
    }
    if stride_dw >= 4 {
        let raw_z = read_u32(start + 2);
        v.z = sanitize_inline_z(f32::from_bits(raw_z), stride_dw, raw_z);
    }
    match stride_dw {
        3 => {
            // XY + COLOR (2D HUD / pre-transformed pixel-space quad)
            v.color = read_u32(start + 2);
        }
        4 => {
            v.color = read_u32(start + 3);
        }
        5 => {
            v.color = read_u32(start + 3);
            // stride=20: XYZ + color + 1-float extra (unclear, skip)
        }
        6 => {
            v.color = read_u32(start + 3);
            v.u = read_f32(start + 4);
            v.v = read_f32(start + 5);
        }
        7 => {
            v.w = read_f32(start + 3);
            v.color = read_u32(start + 4);
            v.u = read_f32(start + 5);
            v.v = read_f32(start + 6);
        }
        _ => {
            if stride_dw >= 4 {
                v.color = read_u32(start + 3);
            }
            if stride_dw >= 6 {
                v.u = read_f32(start + 4);
                v.v = read_f32(start + 5);
            }
        }
    }
    let _ = FVF_TEX1; // silence unused-const warning in stride path
    v
}

fn decode_stream_vertex(
    guest_mem: *mut u8,
    vb_base: u32,
    stride: u32,
    index: u32,
    fvf: u32,
) -> NV2AVertex {
    let mut v = NV2AVertex::default();
    v.color = 0xFFFF_FFFF;
    v.w = 1.0;
    let off = vb_base.wrapping_add(index.wrapping_mul(stride));
    if off >= 0x2000_0000 || off.wrapping_add(stride) > 0x2000_0000 {
        return v;
    }
    let base = (guest_mem as u64) + off as u64;
    let read_f32 = |o: u32| -> f32 {
        unsafe { f32::from_bits(std::ptr::read_unaligned((base + o as u64) as *const u32)) }
    };
    let read_u32 =
        |o: u32| -> u32 { unsafe { std::ptr::read_unaligned((base + o as u64) as *const u32) } };

    let pos = fvf & FVF_POSITION_MASK;
    if pos == FVF_XYZ || pos == FVF_XYZRHW || pos == FVF_XYZB1 || pos == FVF_XYZB2 {
        let mut cur = 0u32;
        v.x = read_f32(cur);
        cur += 4;
        v.y = read_f32(cur);
        cur += 4;
        v.z = read_f32(cur);
        cur += 4;
        if pos == FVF_XYZRHW {
            v.w = read_f32(cur);
            cur += 4;
        }
        if pos == FVF_XYZB1 {
            cur += 4;
        }
        if pos == FVF_XYZB2 {
            cur += 8;
        }
        if fvf & FVF_NORMAL != 0 {
            cur += 12;
        }
        if fvf & FVF_DIFFUSE != 0 {
            v.color = read_u32(cur);
            cur += 4;
        }
        if fvf & FVF_SPECULAR != 0 {
            cur += 4;
        }
        let tex = (fvf & FVF_TEXCOUNT_MASK) >> FVF_TEXCOUNT_SHIFT;
        if tex >= 1 {
            v.u = read_f32(cur);
            cur += 4;
            v.v = read_f32(cur);
            let _ = cur;
        }
        return v;
    }

    // Stride fallback.
    match stride {
        12 => {
            v.x = read_f32(0);
            v.y = read_f32(4);
            v.z = read_f32(8);
        }
        16 => {
            v.x = read_f32(0);
            v.y = read_f32(4);
            v.z = read_f32(8);
            v.color = read_u32(12);
        }
        24 => {
            v.x = read_f32(0);
            v.y = read_f32(4);
            v.z = read_f32(8);
            v.color = read_u32(12);
            v.u = read_f32(16);
            v.v = read_f32(20);
        }
        28 => {
            v.x = read_f32(0);
            v.y = read_f32(4);
            v.z = read_f32(8);
            v.w = read_f32(12);
            v.color = read_u32(16);
            v.u = read_f32(20);
            v.v = read_f32(24);
        }
        _ => {
            if stride >= 12 {
                v.x = read_f32(0);
                v.y = read_f32(4);
                v.z = read_f32(8);
            }
            if stride >= 16 {
                v.color = read_u32(12);
            }
            if stride >= 24 {
                v.u = read_f32(16);
                v.v = read_f32(20);
            }
        }
    }
    v
}

/// V04 read-side: multi-stream vertex decoder. Reads each NV2A attribute
/// slot from its OWN base pointer + stride, using authoritative per-slot
/// type/size captured from NV097_SET_VERTEX_DATA_ARRAY_FORMAT. Mirrors
/// xemu's pgraph_gl_draw_vertex_array semantics:
///   `attr_bytes[slot] = guest_mem + VS_SLOT_OFFSET[slot] + index * VS_SLOT_STRIDE[slot]`
///
/// Slot-to-NV2AVertex mapping:
///   programmable VS: slot 0=v0 position, slot 1=v1 color, slot 3=v3 tex0
///   fixed-function: slot 0=position, slot 3=diffuse, slot 9=tex0
///
/// Historical fixed-function mapping (Xbox D3D8 canonical, matches xemu):
///   slot 0 = D3DVSDE_POSITION   → v.x, v.y, v.z, v.w
///   slot 3 = D3DVSDE_DIFFUSE    → v.color
///   slot 9 = D3DVSDE_TEXCOORD0  → v.u, v.v
///   other slots read but discarded (no NV2AVertex field yet).
///
/// Gated by `any_slot_ge1_has_offset()` — only fires for multi-stream
/// layouts where slots ≥1 have captured bases. Single-stream games keep
/// using `decode_stream_vertex` unchanged.
fn decode_stream_vertex_multi(guest_mem: *mut u8, index: u32) -> NV2AVertex {
    let mut v = NV2AVertex::default();
    v.color = 0xFFFF_FFFF;
    v.w = 1.0;
    let vs_raw = VERTEX_SHADER.load(Ordering::Relaxed);
    let programmable_vs = vertex_shader_value_is_programmable(vs_raw);

    // One-shot diagnostic (first 8 decodes), plus a narrow compositor-window
    // trace so early boot/menu vertices do not consume the only useful logs.
    static MULTI_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static COMPOSITE_MULTI_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let log_n = MULTI_LOG.fetch_add(1, Ordering::Relaxed);
    let comp_n = if spidey_composite_vs(vs_raw) {
        COMPOSITE_MULTI_LOG.fetch_add(1, Ordering::Relaxed)
    } else {
        u32::MAX
    };
    let do_log = log_n < 8 || comp_n < 64;
    if do_log {
        let mut parts = String::new();
        for s in 0..16usize {
            let sz = VS_SLOT_SIZE[s].load(Ordering::Relaxed);
            if sz == 0 {
                continue;
            }
            let off = VS_SLOT_OFFSET[s].load(Ordering::Relaxed);
            let str_ = VS_SLOT_STRIDE[s].load(Ordering::Relaxed);
            let ty = VS_SLOT_TYPE[s].load(Ordering::Relaxed);
            parts.push_str(&format!(
                " s{}={{off:0x{:08X},str:{},sz:{},ty:{}}}",
                s, off, str_, sz, ty
            ));
        }
        crate::xbox::aot::veh::veh_log(&format!(
            "[PB-DECODE-MULTI] #{} comp#{} idx={} vs=0x{:08X} mode={}{}",
            log_n,
            if comp_n == u32::MAX {
                String::from("-")
            } else {
                comp_n.to_string()
            },
            index,
            vs_raw,
            if programmable_vs {
                "programmable"
            } else {
                "fixed"
            },
            parts
        ));
    }

    for slot in 0..16usize {
        let sz = VS_SLOT_SIZE[slot].load(Ordering::Relaxed);
        if sz == 0 {
            continue;
        }
        let slot_stride = VS_SLOT_STRIDE[slot].load(Ordering::Relaxed);
        if slot_stride == 0 || slot_stride > 256 {
            continue;
        }
        let slot_base = VS_SLOT_OFFSET[slot].load(Ordering::Relaxed) & 0x1FFF_FFFF;
        if slot_base == 0 {
            continue;
        }
        let vertex_off = slot_base.wrapping_add(index.wrapping_mul(slot_stride));
        let ty = VS_SLOT_TYPE[slot].load(Ordering::Relaxed);
        let attr_byte_size = slot_byte_size_nv2a(ty, sz);
        if attr_byte_size == 0 || attr_byte_size > 16 {
            continue;
        }
        // Guest RAM bound: attribute must fit in [0, 0x20000000).
        if vertex_off.saturating_add(attr_byte_size) > 0x2000_0000 {
            continue;
        }
        let base = (guest_mem as u64) + vertex_off as u64;
        let mut buf = [0u8; 16];
        let n = attr_byte_size as usize;
        unsafe {
            std::ptr::copy_nonoverlapping(base as *const u8, buf.as_mut_ptr(), n);
        }

        match (programmable_vs, slot) {
            (_, 0) => {
                // POSITION / v0.
                let vals = decode_attribute_nv2a(ty, sz, &buf[..n]);
                v.x = vals[0];
                v.y = vals[1];
                v.z = if sz >= 3 { vals[2] } else { 0.0 };
                v.w = if sz >= 4 { vals[3] } else { 1.0 };
            }
            (true, 1) => {
                // Programmable VS input v1 is color in Spider-Man's compositor
                // declarations. Do not treat v3 as fixed-function DIFFUSE.
                if ty == 0 || ty == 4 {
                    v.color = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                } else if n >= 4 {
                    let vals = decode_attribute_nv2a(ty, sz, &buf[..n]);
                    v.color = pack_color_argb(vals);
                }
            }
            (true, 3) => {
                // Programmable VS input v3 is TEXCOORD0 for this layout.
                let vals = decode_attribute_nv2a(ty, sz, &buf[..n]);
                v.u = vals[0];
                v.v = if sz >= 2 { vals[1] } else { 0.0 };
            }
            (false, 3) => {
                // Fixed-function DIFFUSE.
                if ty == 0 || ty == 4 {
                    v.color = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                } else if n >= 4 {
                    let vals = decode_attribute_nv2a(ty, sz, &buf[..n]);
                    v.color = pack_color_argb(vals);
                }
            }
            (false, 9) => {
                // Fixed-function TEXCOORD0.
                let vals = decode_attribute_nv2a(ty, sz, &buf[..n]);
                v.u = vals[0];
                v.v = if sz >= 2 { vals[1] } else { 0.0 };
            }
            _ => { /* slot read but not forwarded */ }
        }
    }
    v
}

/// Gate: true iff any slot ≥1 has a non-zero captured OFFSET — indicates
/// multi-stream layout. Single-stream games write only slot 0 → returns
/// false → callers keep the legacy `decode_stream_vertex` path.
fn any_slot_ge1_has_offset() -> bool {
    (1..16).any(|s| VS_SLOT_OFFSET[s].load(Ordering::Relaxed) != 0)
}

fn expand_quads_to_tris(src: &[NV2AVertex]) -> Vec<NV2AVertex> {
    let mut out = Vec::with_capacity(src.len() * 6 / 4);
    for quad in src.chunks_exact(4) {
        let q = normalize_inclusive_degenerate_quad(quad)
            .unwrap_or([quad[0], quad[1], quad[2], quad[3]]);
        out.push(q[0]);
        out.push(q[1]);
        out.push(q[2]);
        out.push(q[0]);
        out.push(q[2]);
        out.push(q[3]);
    }
    out
}

fn normalize_inclusive_degenerate_quad(quad: &[NV2AVertex]) -> Option<[NV2AVertex; 4]> {
    const EPS: f32 = 0.0001;
    if quad.len() != 4 {
        return None;
    }
    if !quad
        .iter()
        .all(|v| v.x.is_finite() && v.y.is_finite() && v.z.is_finite())
    {
        return None;
    }

    let (xmin, xmax, ymin, ymax) = quad.iter().fold(
        (
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ),
        |(xmin, xmax, ymin, ymax), v| (xmin.min(v.x), xmax.max(v.x), ymin.min(v.y), ymax.max(v.y)),
    );
    let width = xmax - xmin;
    let height = ymax - ymin;
    if width.abs() >= EPS && height.abs() >= EPS {
        return None;
    }

    let mut out = [quad[0], quad[1], quad[2], quad[3]];
    if width.abs() < EPS && height.abs() >= EPS {
        // Spider-Man's legalbox path emits one-pixel vertical image spans as
        // QUADLIST vertices with inclusive X coordinates (left == right).
        // D3D11 triangle rasterization treats those as zero-area; widen by
        // one pixel around the submitted pixel center so the span survives.
        let x = xmin;
        out[0].x = x - 0.5;
        out[1].x = x + 0.5;
        out[2].x = x + 0.5;
        out[3].x = x - 0.5;
    } else if height.abs() < EPS && width.abs() >= EPS {
        // Same inclusive-coordinate rule for horizontal one-pixel spans.
        let y = ymin;
        out[0].y = y - 0.5;
        out[1].y = y - 0.5;
        out[2].y = y + 0.5;
        out[3].y = y + 0.5;
    } else {
        return None;
    }

    static RESCUE_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = RESCUE_N.fetch_add(1, Ordering::Relaxed);
    if n < 16 || n.is_power_of_two() {
        crate::xbox::emulator::debug_log(&format!(
            "[QUAD-INCLUSIVE-RESCUE #{}] x=[{:.1}..{:.1}] y=[{:.1}..{:.1}] -> x=[{:.1}..{:.1}] y=[{:.1}..{:.1}]",
            n,
            xmin,
            xmax,
            ymin,
            ymax,
            out.iter().fold(f32::INFINITY, |a, v| a.min(v.x)),
            out.iter().fold(f32::NEG_INFINITY, |a, v| a.max(v.x)),
            out.iter().fold(f32::INFINITY, |a, v| a.min(v.y)),
            out.iter().fold(f32::NEG_INFINITY, |a, v| a.max(v.y))
        ));
    }
    Some(out)
}

fn expand_quad_strip_to_tris(src: &[NV2AVertex]) -> Vec<NV2AVertex> {
    // Quad strip: v0,v1,v2,v3,v4,v5,... → quads (0,1,3,2), (2,3,5,4), ...
    let mut out = Vec::with_capacity(src.len() * 3 / 2);
    let mut i = 0;
    while i + 3 < src.len() {
        let a = src[i];
        let b = src[i + 1];
        let c = src[i + 2];
        let d = src[i + 3];
        out.push(a);
        out.push(b);
        out.push(d);
        out.push(a);
        out.push(d);
        out.push(c);
        i += 2;
    }
    out
}

// ---------------------------------------------------------------------------
// NV097 method-offset histogram (2026-04-21).
//
// Fixed-size atomic array (one u32 count per 4-byte method offset, 2048
// entries covering 0x0000..0x2000). Every unknown-method hit records to
// the bin. Every 50K total hits we dump the top-20 hottest bins so we
// can identify which methods Spider-Man actually writes beyond the ones
// we explicitly handle.
//
// Goal: find the stream-source setup path. STREAM0_VB_ADDR stays 0 through
// 166K+ parsed methods. The hot hits that aren't DRAW_ARRAYS / INLINE_ARRAY /
// ARRAY_ELEMENT* / SET_VERTEX_DATA* must include where the game binds its
// vertex buffer.
// ---------------------------------------------------------------------------

const HIST_SLOTS: usize = 2048;
static HIST: [std::sync::atomic::AtomicU32; HIST_SLOTS] = {
    const Z: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    [Z; HIST_SLOTS]
};
static HIST_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn hist_record(method: u32, count: u32) {
    let slot = ((method >> 2) as usize) & (HIST_SLOTS - 1);
    let inc = count.max(1);
    HIST[slot].fetch_add(inc, std::sync::atomic::Ordering::Relaxed);
    let tot = HIST_TOTAL.fetch_add(inc as u64, std::sync::atomic::Ordering::Relaxed);
    // Dump at 50K, 200K, 1M, 5M boundaries — coarse progression so we don't
    // spam the log but still see early vs late method distributions.
    let new_tot = tot + inc as u64;
    for &threshold in &[50_000u64, 200_000, 1_000_000, 5_000_000] {
        if tot < threshold && new_tot >= threshold {
            hist_dump_top(20, threshold);
        }
    }
}

fn hist_dump_top(top_n: usize, total_at_dump: u64) {
    // Gather (slot, count) pairs, sort descending by count, print top N.
    let mut entries: Vec<(u32, u32)> = (0..HIST_SLOTS)
        .filter_map(|slot| {
            let c = HIST[slot].load(std::sync::atomic::Ordering::Relaxed);
            if c > 0 {
                Some((slot as u32 * 4, c))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    let shown = entries.iter().take(top_n);
    crate::xbox::aot::veh::veh_log(&format!(
        "[NV097-HIST] total={} unique_offsets={} top_{}:",
        total_at_dump,
        entries.len(),
        top_n
    ));
    for (offset, count) in shown {
        let name = method_name_hint(*offset);
        crate::xbox::aot::veh::veh_log(&format!(
            "  0x{:04X} ({:>14}) hits={}",
            offset, name, count
        ));
    }
}

/// Best-effort name hint for a given NV097 method offset.
/// Helps reading the histogram dump — we don't need full names, just enough
/// to recognize the hot offsets.
fn method_name_hint(offset: u32) -> &'static str {
    match offset {
        0x0000 => "SET_OBJECT",
        0x0100 => "NO_OP",
        0x0104 => "NOTIFY",
        0x0110 => "WAIT_FOR_IDLE",
        0x0140 => "PM_TRIGGER",
        0x0180 => "SET_CTX_DMA_A",
        0x0184 => "SET_CTX_DMA_B",
        0x018C => "SET_CTX_DMA_COLOR",
        0x0190 => "SET_CTX_DMA_ZETA",
        0x0194 => "SET_CTX_DMA_VS_A",
        0x0198 => "SET_CTX_DMA_VS_B",
        0x019C => "SET_CTX_DMA_SEMAPH",
        0x0200 => "SURFACE_CLIP_H",
        0x0204 => "SURFACE_CLIP_V",
        0x0208 => "SURFACE_FORMAT",
        0x020C => "SURFACE_PITCH",
        0x0210 => "SURFACE_COLOR_OFF",
        0x0214 => "SURFACE_ZETA_OFF",
        0x0218 => "COMBINER_CONTROL",
        0x0220 => "SHADER_STAGE_PROG",
        0x0300..=0x03FC => "PS_STATE",
        0x0400..=0x04FC => "VS_CONSTANT",
        0x0500..=0x05FC => "VS_PROGRAM",
        0x0800..=0x0BFC => "TEXTURE_STAGE",
        0x0C00..=0x0CFC => "COMBINER",
        0x1720..=0x175F => "VTX_DATA_OFFSET",
        0x1760..=0x179F => "VTX_DATA_FORMAT",
        0x17FC => "BEGIN_END",
        0x1800 => "ARRAY_ELEMENT16",
        0x1808 => "ARRAY_ELEMENT32",
        0x1810 => "DRAW_ARRAYS",
        0x1818 => "INLINE_ARRAY",
        0x1880..=0x18FC => "VTX_DATA2F_M",
        0x1900..=0x193C => "VTX_DATA2S",
        0x1940..=0x197C => "VTX_DATA4UB",
        0x1980..=0x19FC => "VTX_DATA4S_M",
        0x1A00..=0x1AFC => "VTX_DATA4F_M",
        0x1C00..=0x1CFC => "VTX_DATA_CONST",
        0x1D6C => "CLEAR_COLOR",
        0x1D70 => "CLEAR_DEPTH",
        0x1D78 => "CLEAR_STENCIL",
        0x1D8C => "CLEAR_RECT_H",
        0x1D90 => "CLEAR_RECT_V",
        0x1D94 => "CLEAR",
        0x1D98..=0x1D9C => "CLEAR_EXT",
        0x1E80..=0x1EFC => "TRANSFORM",
        0x1F00..=0x1F3C => "MATERIAL",
        0x1F80..=0x1FFC => "LIGHT",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pb_cmd(method: u32, count: u32, non_inc: bool) -> u32 {
        ((non_inc as u32) << 30) | ((count & 0x7FF) << 18) | (method & 0x1FFC)
    }

    fn parse_words(parser: &mut PbParser, words: &mut [u32]) -> u32 {
        let bytes = (words.len() * std::mem::size_of::<u32>()) as u32;
        parser.parse(words.as_mut_ptr().cast::<u8>(), 0, bytes, false)
    }

    #[test]
    fn nv097_array_element16_accumulates_two_16bit_indices_per_dword() {
        let mut parser = PbParser::new();
        let mut pb = [
            pb_cmd(NV097_SET_BEGIN_END, 1, false),
            NV_PRIM_TRIANGLES,
            pb_cmd(NV097_ARRAY_ELEMENT16, 2, true),
            0x0002_0001,
            0xFFFF_0003,
        ];

        let parsed = parse_words(&mut parser, &mut pb);

        assert_eq!(parsed, 2);
        assert_eq!(parser.indices, vec![1, 2, 3]);
        assert!(parser.direct_verts.is_empty());
    }

    #[test]
    fn nv097_array_element32_accumulates_one_32bit_index_per_dword() {
        let mut parser = PbParser::new();
        let mut pb = [
            pb_cmd(NV097_SET_BEGIN_END, 1, false),
            NV_PRIM_TRIANGLES,
            pb_cmd(NV097_ARRAY_ELEMENT32, 3, true),
            7,
            8,
            9,
        ];

        let parsed = parse_words(&mut parser, &mut pb);

        assert_eq!(parsed, 2);
        assert_eq!(parser.indices, vec![7, 8, 9]);
        assert!(parser.direct_verts.is_empty());
    }

    #[test]
    fn nv097_1880_is_vertex_data2f_m_not_index_stream() {
        let mut parser = PbParser::new();
        let mut pb = [
            pb_cmd(NV097_SET_BEGIN_END, 1, false),
            NV_PRIM_TRIANGLES,
            pb_cmd(NV097_SET_VERTEX_DATA2F_M, 2, false),
            1.25f32.to_bits(),
            (-2.5f32).to_bits(),
        ];

        let parsed = parse_words(&mut parser, &mut pb);

        assert_eq!(parsed, 2);
        assert!(parser.indices.is_empty());
        assert_eq!(parser.direct_verts.len(), 1);
        assert_eq!(parser.direct_verts[0].x, 1.25);
        assert_eq!(parser.direct_verts[0].y, -2.5);
        assert_eq!(parser.direct_verts[0].z, 0.0);
        assert_eq!(parser.direct_verts[0].w, 1.0);
    }

    #[test]
    fn nv097_regression_method_names_for_index_and_1880_offsets() {
        assert_eq!(method_name(0x1800), "ARRAY_ELEMENT16");
        assert_eq!(method_name(0x1808), "ARRAY_ELEMENT32");
        assert_eq!(method_name(0x1880), "SET_VERTEX_DATA2F_M");
        assert_eq!(method_name_hint(0x1800), "ARRAY_ELEMENT16");
        assert_eq!(method_name_hint(0x1808), "ARRAY_ELEMENT32");
        assert_eq!(method_name_hint(0x1880), "VTX_DATA2F_M");
    }
}
