/// OOVPA D3D constants, texture object layout, and texture allocation.
/// Split from oovpa.rs for modularity.
use crate::xbox::emulator::debug_log;

// ============================================================================
// XDK 4134 constants (Spider-Man)
// ============================================================================

/// g_pDevice global address
pub const G_PDEVICE: u32 = 0x0030_38E0;
/// Dummy pushbuffer region (64KB, harmless writes)
pub const PB_DUMMY_BASE: u32 = 0x00EA_0000;
pub const PB_DUMMY_SIZE: u32 = 0x0001_0000;

/// Fake Bink video handle address (guest memory).
/// BinkOpen returns this; BinkDoFrame/BinkNextFrame/BinkClose consume it.
/// Field at handle+0x250 = total_frames (set to 0 so BinkNextFrame sees "done").
pub const BINK_FAKE_HANDLE: u32 = 0x00EA_A000;

// Device struct offsets — see oovpa.rs for the canonical definitions.
// 0x2070-0x2084 is the SURFACE TABLE, not push buffer!
pub const DEV_PB_PUT: u32 = 0x2900; // internal PB tracking (relocated from 0x2070)
pub const DEV_PB_BASE: u32 = 0x2904;
pub const DEV_PB_LIMIT: u32 = 0x290C;
pub const DEV_VSHADER: u32 = 0x0380;
pub const DEV_FRAME_CTR: u32 = 0x2ABC;

// ============================================================================
// Xbox D3D texture/surface object layout (from Cxbx-R / XDK)
// ============================================================================
//
// X_D3DResource (base for textures and surfaces):
//   Offset 0x00: Common  -- D3DCOMMON_TYPE flags | refcount
//   Offset 0x04: Data    -- GPU data pointer (guest address of pixel data)
//   Offset 0x08: Lock    -- lock count (0 = unlocked)
//   Offset 0x0C: Format  -- packed: dimensions, pixel format, mip levels
//   Offset 0x10: Size    -- total allocation size in bytes
//
// On Xbox, a texture IS its own level-0 surface (no separate surface object).

/// Size of an Xbox D3D resource header (Common + Data + Lock + Format + Size)
pub const X_D3D_RESOURCE_SIZE: u32 = 0x14; // 20 bytes

// D3DCOMMON_TYPE flags (bits 16..18 of Common field)
pub const X_D3DCOMMON_TYPE_SHIFT: u32 = 16;
pub const X_D3DCOMMON_TYPE_TEXTURE: u32 = 3 << X_D3DCOMMON_TYPE_SHIFT;
pub const X_D3DCOMMON_TYPE_SURFACE: u32 = 4 << X_D3DCOMMON_TYPE_SHIFT;

// Field offsets within X_D3DResource
pub const X_D3DRES_COMMON: u32 = 0x00;
pub const X_D3DRES_DATA: u32 = 0x04;
pub const X_D3DRES_LOCK: u32 = 0x08;
pub const X_D3DRES_FORMAT: u32 = 0x0C;
pub const X_D3DRES_SIZE: u32 = 0x10;

/// Xbox D3D pixel format codes (Format field bits 8..12)
pub const X_D3DFMT_A8R8G8B8: u32 = 0x06;
pub const X_D3DFMT_X8R8G8B8: u32 = 0x07;
pub const X_D3DFMT_LIN_A8R8G8B8: u32 = 0x12;
pub const X_D3DFMT_LIN_X8R8G8B8: u32 = 0x1E;

/// Texture heap: 0x00EB0000..0x00EC0000 (64KB region for texture objects).
/// Each texture object = 0x14 bytes header + scratch pixel data.
/// We allocate 0x100 bytes per texture (header + 236 bytes of pixel scratch).
pub const TEX_HEAP_BASE: u32 = 0x00EB_0000;
pub const TEX_HEAP_SIZE: u32 = 0x0001_0000;
pub const TEX_SLOT_SIZE: u32 = 0x100; // 256 bytes per texture slot

/// Scratch pixel data area: 0x00EC0000..0x00ED0000 (64KB).
/// CreateTexture points Data to here so pixel reads don't fault.
pub const TEX_DATA_BASE: u32 = 0x00EC_0000;
pub const TEX_DATA_SIZE: u32 = 0x0001_0000;

/// Atomic bump allocator for texture object slots.
pub static NEXT_TEX_SLOT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(TEX_HEAP_BASE);

/// Atomic bump allocator for texture pixel data.
pub static NEXT_TEX_DATA: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(TEX_DATA_BASE);

/// Current render target surface pointer (guest address, 0 = none).
pub static CURRENT_RENDER_TARGET: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Current depth-stencil surface pointer (guest address, 0 = none).
pub static CURRENT_DEPTH_STENCIL: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Encode an Xbox D3D Format field from width, height, and format code.
/// Layout (Xbox D3D8 swizzled format):
///   bits  3..0  : mip level count minus 1
///   bits 12..8  : pixel format code
///   bits 23..20 : log2(width)
///   bits 27..24 : log2(height)
pub fn encode_xbox_format(width: u32, height: u32, fmt: u32, levels: u32) -> u32 {
    let log2w = if width > 1 {
        (width as f32).log2() as u32
    } else {
        0
    };
    let log2h = if height > 1 {
        (height as f32).log2() as u32
    } else {
        0
    };
    (log2w & 0xF) << 20 | (log2h & 0xF) << 24 | (fmt & 0x3F) << 8 | (levels.saturating_sub(1) & 0xF)
}

/// Allocate an Xbox D3D texture object in guest memory.
/// Returns the guest address of the texture header, or 0 on failure.
///
/// # Safety
/// `guest_mem` must point to the 4GB guest memory reservation.
pub unsafe fn alloc_texture(
    guest_mem: *mut u8,
    width: u32,
    height: u32,
    levels: u32,
    _usage: u32,
    format: u32,
) -> u32 {
    // Bump-allocate a texture slot
    let tex_addr = NEXT_TEX_SLOT.fetch_add(TEX_SLOT_SIZE, std::sync::atomic::Ordering::Relaxed);
    if tex_addr + TEX_SLOT_SIZE > TEX_HEAP_BASE + TEX_HEAP_SIZE {
        debug_log("[OOVPA-HLE] alloc_texture: texture heap exhausted!");
        return 0;
    }

    // Calculate pixel data size and allocate scratch area
    let bpp: u32 = match format {
        X_D3DFMT_A8R8G8B8 | X_D3DFMT_X8R8G8B8 | X_D3DFMT_LIN_A8R8G8B8 | X_D3DFMT_LIN_X8R8G8B8 => 4,
        _ => 4, // default to 32bpp
    };
    let data_size = width.max(1) * height.max(1) * bpp;
    let data_aligned = (data_size + 0xFF) & !0xFF; // 256-byte aligned

    let data_addr = NEXT_TEX_DATA.fetch_add(data_aligned, std::sync::atomic::Ordering::Relaxed);
    // If we run out of pixel scratch, reuse TEX_DATA_BASE (still mapped, won't fault)
    let safe_data = if data_addr + data_aligned <= TEX_DATA_BASE + TEX_DATA_SIZE {
        data_addr
    } else {
        debug_log("[OOVPA-HLE] alloc_texture: pixel data area exhausted, reusing base");
        TEX_DATA_BASE
    };

    // Write the 20-byte X_D3DResource header
    let base = guest_mem as u64 + tex_addr as u64;
    let common = X_D3DCOMMON_TYPE_TEXTURE | 1; // type=texture, refcount=1
    *(base as *mut u32) = common; // +0x00 Common
    *((base + X_D3DRES_DATA as u64) as *mut u32) = safe_data; // +0x04 Data
    *((base + X_D3DRES_LOCK as u64) as *mut u32) = 0; // +0x08 Lock
    *((base + X_D3DRES_FORMAT as u64) as *mut u32) =
        encode_xbox_format(width, height, format, levels); // +0x0C Format
    *((base + X_D3DRES_SIZE as u64) as *mut u32) = data_size; // +0x10 Size

    // Zero the rest of the slot (safety for any fields the game probes)
    for off in (X_D3D_RESOURCE_SIZE..TEX_SLOT_SIZE).step_by(4) {
        *((base + off as u64) as *mut u32) = 0;
    }

    tex_addr
}

/// Get the current render target guest address.
pub fn get_render_target() -> u32 {
    CURRENT_RENDER_TARGET.load(std::sync::atomic::Ordering::Relaxed)
}

/// Get the current depth-stencil guest address.
pub fn get_depth_stencil() -> u32 {
    CURRENT_DEPTH_STENCIL.load(std::sync::atomic::Ordering::Relaxed)
}
