/// DXT5 texture decoder + framebuffer injection.
/// Loads a DDS file from disc, decodes DXT5 → XRGB8888, and stores
/// the result in the GPU readback buffer for display.

use crate::xbox::emulator::debug_log;

/// Decode a DXT5-compressed DDS file to XRGB8888 pixels.
/// Returns (width, height, pixels) on success.
pub fn decode_dds_file(path: &str) -> Option<(u32, u32, Vec<u32>)> {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            debug_log(&format!("[DDS] Failed to read {}: {}", path, e));
            return None;
        }
    };

    if data.len() < 128 || &data[0..4] != b"DDS " {
        debug_log("[DDS] Invalid DDS header");
        return None;
    }

    let height = u32::from_le_bytes(data[12..16].try_into().ok()?) as usize;
    let width = u32::from_le_bytes(data[16..20].try_into().ok()?) as usize;
    let fourcc = &data[84..88];

    if fourcc != b"DXT5" {
        debug_log(&format!("[DDS] Unsupported format: {:?}", fourcc));
        return None;
    }

    let block_data = &data[128..];
    let bw = (width + 3) / 4;
    let bh = (height + 3) / 4;
    let expected = bw * bh * 16;

    if block_data.len() < expected {
        debug_log(&format!("[DDS] Data too short: {} < {}", block_data.len(), expected));
        return None;
    }

    let mut pixels = vec![0u32; width * height];

    for by in 0..bh {
        for bx in 0..bw {
            let block_offset = (by * bw + bx) * 16;
            let block = &block_data[block_offset..block_offset + 16];
            decode_dxt5_block(block, &mut pixels, width, height, bx * 4, by * 4);
        }
    }

    debug_log(&format!("[DDS] Decoded {}x{} DXT5 → {} pixels", width, height, pixels.len()));
    Some((width as u32, height as u32, pixels))
}

/// Decode a single 4x4 DXT5 block into the pixel buffer.
fn decode_dxt5_block(block: &[u8], pixels: &mut [u32], stride: usize, height: usize, x0: usize, y0: usize) {
    // Alpha: bytes 0-7
    let a0 = block[0] as u32;
    let a1 = block[1] as u32;
    let mut alpha_bits: u64 = 0;
    for i in 0..6 {
        alpha_bits |= (block[2 + i] as u64) << (i * 8);
    }

    let mut alphas = [0u8; 16];
    for i in 0..16 {
        let code = ((alpha_bits >> (i * 3)) & 7) as u32;
        let a = if a0 > a1 {
            match code {
                0 => a0,
                1 => a1,
                2 => (6 * a0 + 1 * a1) / 7,
                3 => (5 * a0 + 2 * a1) / 7,
                4 => (4 * a0 + 3 * a1) / 7,
                5 => (3 * a0 + 4 * a1) / 7,
                6 => (2 * a0 + 5 * a1) / 7,
                _ => (1 * a0 + 6 * a1) / 7,
            }
        } else {
            match code {
                0 => a0,
                1 => a1,
                2 => (4 * a0 + 1 * a1) / 5,
                3 => (3 * a0 + 2 * a1) / 5,
                4 => (2 * a0 + 3 * a1) / 5,
                5 => (1 * a0 + 4 * a1) / 5,
                6 => 0,
                _ => 255,
            }
        };
        alphas[i] = a.min(255) as u8;
    }

    // Color: bytes 8-15
    let c0 = u16::from_le_bytes([block[8], block[9]]);
    let c1 = u16::from_le_bytes([block[10], block[11]]);

    let r0 = ((c0 >> 11) & 0x1F) as u32;
    let g0 = ((c0 >> 5) & 0x3F) as u32;
    let b0 = (c0 & 0x1F) as u32;
    let r1 = ((c1 >> 11) & 0x1F) as u32;
    let g1 = ((c1 >> 5) & 0x3F) as u32;
    let b1 = (c1 & 0x1F) as u32;

    // Expand to 8-bit
    let colors: [(u8, u8, u8); 4] = [
        ((r0 * 255 / 31) as u8, (g0 * 255 / 63) as u8, (b0 * 255 / 31) as u8),
        ((r1 * 255 / 31) as u8, (g1 * 255 / 63) as u8, (b1 * 255 / 31) as u8),
        (
            ((2 * r0 + r1) * 255 / (31 * 3)) as u8,
            ((2 * g0 + g1) * 255 / (63 * 3)) as u8,
            ((2 * b0 + b1) * 255 / (31 * 3)) as u8,
        ),
        (
            ((r0 + 2 * r1) * 255 / (31 * 3)) as u8,
            ((g0 + 2 * g1) * 255 / (63 * 3)) as u8,
            ((b0 + 2 * b1) * 255 / (31 * 3)) as u8,
        ),
    ];

    let color_indices = u32::from_le_bytes([block[12], block[13], block[14], block[15]]);

    for py in 0..4 {
        for px in 0..4 {
            let x = x0 + px;
            let y = y0 + py;
            if x >= stride || y >= height {
                continue;
            }
            let idx = (color_indices >> ((py * 4 + px) * 2)) & 3;
            let (r, g, b) = colors[idx as usize];
            // Use opaque RGB — DXT5 alpha in loading textures is often 0
            // (transparency for compositing), but we want visible pixels.
            pixels[y * stride + x] = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }
}

/// Load a Spider-Man DDS texture and inject it into the GPU readback buffer.
/// Scales/centers the image to fit 640x480.
pub fn inject_loading_screen(xbe_dir: &str) -> bool {
    let candidates = [
        "data/loadtex/smload01.dds",  // Spider-Man scene load (256x256)
        "data/loadtex/smload02.dds",  // Spider-Man scene load (256x256)
        "data/loadtex/rspiderc.dds",  // Spider-Man loading screen (512x512)
        "data/loadtex/rspider.dds",   // Alternative (256x512)
        "data/loadtex/m0load00.dds",  // Mission loading (128x256)
    ];

    let mut decoded = None;
    for candidate in &candidates {
        let path = format!("{}/{}", xbe_dir, candidate);
        if let Some(result) = decode_dds_file(&path) {
            decoded = Some((result, *candidate));
            break;
        }
    }

    let ((tex_w, tex_h, tex_pixels), name) = match decoded {
        Some(d) => d,
        None => {
            debug_log("[DDS] No loadable DDS texture found");
            return false;
        }
    };

    // Scale/center to 640x480 framebuffer
    const FB_W: usize = 640;
    const FB_H: usize = 480;
    let mut fb = vec![0u32; FB_W * FB_H];

    let tw = tex_w as usize;
    let th = tex_h as usize;

    // Scale to fill 640x480, preserving aspect ratio, center as needed.
    // Use integer math to avoid float precision artifacts (vertical line bug).
    let scale_x = FB_W * 1024 / tw; // fixed-point 10.10
    let scale_y = FB_H * 1024 / th;
    let scale = scale_x.min(scale_y); // fit inside
    let scaled_w = tw * scale / 1024;
    let scaled_h = th * scale / 1024;
    let x_offset = if scaled_w < FB_W { (FB_W - scaled_w) / 2 } else { 0 };
    let y_offset = if scaled_h < FB_H { (FB_H - scaled_h) / 2 } else { 0 };

    for dy in 0..scaled_h.min(FB_H) {
        let src_y = dy * th / scaled_h;
        if src_y >= th { continue; }
        for dx in 0..scaled_w.min(FB_W) {
            let src_x = dx * tw / scaled_w;
            if src_x >= tw { continue; }
            let dst_x = dx + x_offset;
            let dst_y = dy + y_offset;
            if dst_x < FB_W && dst_y < FB_H {
                fb[dst_y * FB_W + dst_x] = tex_pixels[src_y * tw + src_x];
            }
        }
    }

    // Store in GPU readback buffer
    crate::xbox::gpu::store_readback(&fb);
    debug_log(&format!("[DDS] Injected {} ({}x{}) → 640x480 framebuffer", name, tw, th));
    true
}

/// Try to load LEGAL_00.DDS from the tools directory and inject it into
/// the GPU readback buffer.  Returns `true` if a texture was loaded.
pub fn inject_legal_screen() -> bool {
    let paths = [
        "./tools/LEGAL_00.DDS",
        "tools/LEGAL_00.DDS",
    ];

    for path in &paths {
        if let Some((w, h, pixels)) = decode_dds_file(path) {
            const FB_W: usize = 640;
            const FB_H: usize = 480;
            let mut fb = vec![0u32; FB_W * FB_H];

            let tw = w as usize;
            let th = h as usize;
            let scale_x = FB_W * 1024 / tw;
            let scale_y = FB_H * 1024 / th;
            let scale = scale_x.min(scale_y);
            let scaled_w = tw * scale / 1024;
            let scaled_h = th * scale / 1024;
            let x_offset = if scaled_w < FB_W { (FB_W - scaled_w) / 2 } else { 0 };
            let y_offset = if scaled_h < FB_H { (FB_H - scaled_h) / 2 } else { 0 };

            for dy in 0..scaled_h.min(FB_H) {
                let src_y = dy * th / scaled_h;
                if src_y >= th { continue; }
                for dx in 0..scaled_w.min(FB_W) {
                    let src_x = dx * tw / scaled_w;
                    if src_x >= tw { continue; }
                    let dst_x = dx + x_offset;
                    let dst_y = dy + y_offset;
                    if dst_x < FB_W && dst_y < FB_H {
                        fb[dst_y * FB_W + dst_x] = pixels[src_y * tw + src_x];
                    }
                }
            }

            crate::xbox::gpu::store_readback(&fb);
            debug_log(&format!("[DDS] Injected LEGAL_00.DDS ({}x{}) → 640x480", tw, th));
            return true;
        }
    }

    false
}
