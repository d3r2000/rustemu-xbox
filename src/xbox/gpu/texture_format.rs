//! Xbox D3DFORMAT decoding helpers for host texture upload.
//!
//! Xbox format enum values differ from PC Direct3D. Keep the canonical
//! constants and software decode rules here so D3D11, XPR registration, and
//! D3D HLE texture binding all agree on byte counts and component order.

pub const X_D3DFMT_L8: u32 = 0x00;
pub const X_D3DFMT_AL8: u32 = 0x01;
pub const X_D3DFMT_A1R5G5B5: u32 = 0x02;
pub const X_D3DFMT_X1R5G5B5: u32 = 0x03;
pub const X_D3DFMT_A4R4G4B4: u32 = 0x04;
pub const X_D3DFMT_R5G6B5: u32 = 0x05;
pub const X_D3DFMT_A8R8G8B8: u32 = 0x06;
pub const X_D3DFMT_X8R8G8B8: u32 = 0x07;
pub const X_D3DFMT_P8: u32 = 0x0B;
pub const X_D3DFMT_DXT1: u32 = 0x0C;
pub const X_D3DFMT_DXT3: u32 = 0x0E;
pub const X_D3DFMT_DXT5: u32 = 0x0F;
pub const X_D3DFMT_LIN_A1R5G5B5: u32 = 0x10;
pub const X_D3DFMT_LIN_R5G6B5: u32 = 0x11;
pub const X_D3DFMT_LIN_A8R8G8B8: u32 = 0x12;
pub const X_D3DFMT_LIN_L8: u32 = 0x13;
pub const X_D3DFMT_LIN_R8B8: u32 = 0x16;
pub const X_D3DFMT_LIN_G8B8: u32 = 0x17;
pub const X_D3DFMT_A8: u32 = 0x19;
pub const X_D3DFMT_A8L8: u32 = 0x1A;
pub const X_D3DFMT_LIN_AL8: u32 = 0x1B;
pub const X_D3DFMT_LIN_X1R5G5B5: u32 = 0x1C;
pub const X_D3DFMT_LIN_A4R4G4B4: u32 = 0x1D;
pub const X_D3DFMT_LIN_X8R8G8B8: u32 = 0x1E;
pub const X_D3DFMT_LIN_A8: u32 = 0x1F;
pub const X_D3DFMT_LIN_A8L8: u32 = 0x20;
pub const X_D3DFMT_YUY2: u32 = 0x24;
pub const X_D3DFMT_UYVY: u32 = 0x25;
pub const X_D3DFMT_R6G5B5: u32 = 0x27;
pub const X_D3DFMT_G8B8: u32 = 0x28;
pub const X_D3DFMT_R8B8: u32 = 0x29;
pub const X_D3DFMT_D24S8: u32 = 0x2A;
pub const X_D3DFMT_F24S8: u32 = 0x2B;
pub const X_D3DFMT_D16: u32 = 0x2C;
pub const X_D3DFMT_F16: u32 = 0x2D;
pub const X_D3DFMT_LIN_D24S8: u32 = 0x2E;
pub const X_D3DFMT_LIN_F24S8: u32 = 0x2F;
pub const X_D3DFMT_LIN_D16: u32 = 0x30;
pub const X_D3DFMT_LIN_F16: u32 = 0x31;
pub const X_D3DFMT_L16: u32 = 0x32;
pub const X_D3DFMT_V16U16: u32 = 0x33;
pub const X_D3DFMT_LIN_L16: u32 = 0x35;
pub const X_D3DFMT_LIN_V16U16: u32 = 0x36;
pub const X_D3DFMT_LIN_R6G5B5: u32 = 0x37;
pub const X_D3DFMT_R5G5B5A1: u32 = 0x38;
pub const X_D3DFMT_R4G4B4A4: u32 = 0x39;
pub const X_D3DFMT_A8B8G8R8: u32 = 0x3A;
pub const X_D3DFMT_B8G8R8A8: u32 = 0x3B;
pub const X_D3DFMT_R8G8B8A8: u32 = 0x3C;
pub const X_D3DFMT_LIN_R5G5B5A1: u32 = 0x3D;
pub const X_D3DFMT_LIN_R4G4B4A4: u32 = 0x3E;
pub const X_D3DFMT_LIN_A8B8G8R8: u32 = 0x3F;
pub const X_D3DFMT_LIN_B8G8R8A8: u32 = 0x40;
pub const X_D3DFMT_LIN_R8G8B8A8: u32 = 0x41;

#[inline]
pub fn format_code(format: u32) -> u32 {
    let shifted = (format >> 8) & 0xFF;
    if shifted != 0 {
        shifted
    } else {
        format & 0xFF
    }
}

pub fn format_name(format_code: u32) -> &'static str {
    match format_code {
        X_D3DFMT_L8 => "L8",
        X_D3DFMT_AL8 => "AL8",
        X_D3DFMT_A1R5G5B5 => "A1R5G5B5",
        X_D3DFMT_X1R5G5B5 => "X1R5G5B5",
        X_D3DFMT_A4R4G4B4 => "A4R4G4B4",
        X_D3DFMT_R5G6B5 => "R5G6B5",
        X_D3DFMT_A8R8G8B8 => "A8R8G8B8",
        X_D3DFMT_X8R8G8B8 => "X8R8G8B8",
        X_D3DFMT_P8 => "P8",
        X_D3DFMT_DXT1 => "DXT1",
        X_D3DFMT_DXT3 => "DXT3",
        X_D3DFMT_DXT5 => "DXT5",
        X_D3DFMT_LIN_A1R5G5B5 => "LIN_A1R5G5B5",
        X_D3DFMT_LIN_R5G6B5 => "LIN_R5G6B5",
        X_D3DFMT_LIN_A8R8G8B8 => "LIN_A8R8G8B8",
        X_D3DFMT_LIN_L8 => "LIN_L8",
        X_D3DFMT_LIN_R8B8 => "LIN_R8B8",
        X_D3DFMT_LIN_G8B8 => "LIN_G8B8/V8U8",
        X_D3DFMT_A8 => "A8",
        X_D3DFMT_A8L8 => "A8L8",
        X_D3DFMT_LIN_AL8 => "LIN_AL8",
        X_D3DFMT_LIN_X1R5G5B5 => "LIN_X1R5G5B5",
        X_D3DFMT_LIN_A4R4G4B4 => "LIN_A4R4G4B4",
        X_D3DFMT_LIN_X8R8G8B8 => "LIN_X8R8G8B8",
        X_D3DFMT_LIN_A8 => "LIN_A8",
        X_D3DFMT_LIN_A8L8 => "LIN_A8L8",
        X_D3DFMT_YUY2 => "YUY2",
        X_D3DFMT_UYVY => "UYVY",
        X_D3DFMT_R6G5B5 => "R6G5B5",
        X_D3DFMT_G8B8 => "G8B8/V8U8",
        X_D3DFMT_R8B8 => "R8B8",
        X_D3DFMT_L16 => "L16",
        X_D3DFMT_LIN_L16 => "LIN_L16",
        X_D3DFMT_B8G8R8A8 => "B8G8R8A8",
        X_D3DFMT_R8G8B8A8 => "R8G8B8A8",
        X_D3DFMT_LIN_B8G8R8A8 => "LIN_B8G8R8A8",
        X_D3DFMT_LIN_R8G8B8A8 => "LIN_R8G8B8A8",
        _ => "UNKNOWN",
    }
}

pub fn is_block_compressed(format_code: u32) -> bool {
    matches!(format_code, X_D3DFMT_DXT1 | X_D3DFMT_DXT3 | X_D3DFMT_DXT5)
}

pub fn is_swizzled_block_compressed(format_code: u32) -> bool {
    // Spider-Man's current corruption evidence points at the DXT1/BC1 fallback
    // lane. DXT5/BC3 menu uploads already render through the native path; do
    // not deswizzle them without a separate per-format proof.
    matches!(format_code, X_D3DFMT_DXT1)
}

pub fn block_bytes(format_code: u32) -> Option<u32> {
    match format_code {
        X_D3DFMT_DXT1 => Some(8),
        X_D3DFMT_DXT3 | X_D3DFMT_DXT5 => Some(16),
        _ => None,
    }
}

pub fn is_depth_format(format_code: u32) -> bool {
    matches!(
        format_code,
        X_D3DFMT_D24S8
            | X_D3DFMT_F24S8
            | X_D3DFMT_D16
            | X_D3DFMT_F16
            | X_D3DFMT_LIN_D24S8
            | X_D3DFMT_LIN_F24S8
            | X_D3DFMT_LIN_D16
            | X_D3DFMT_LIN_F16
    )
}

pub fn is_linear(format_code: u32) -> bool {
    matches!(
        format_code,
        X_D3DFMT_LIN_A1R5G5B5
            | X_D3DFMT_LIN_R5G6B5
            | X_D3DFMT_LIN_A8R8G8B8
            | X_D3DFMT_LIN_L8
            | X_D3DFMT_LIN_R8B8
            | X_D3DFMT_LIN_G8B8
            | X_D3DFMT_LIN_AL8
            | X_D3DFMT_LIN_X1R5G5B5
            | X_D3DFMT_LIN_A4R4G4B4
            | X_D3DFMT_LIN_X8R8G8B8
            | X_D3DFMT_LIN_A8
            | X_D3DFMT_LIN_A8L8
            | X_D3DFMT_LIN_L16
            | X_D3DFMT_LIN_V16U16
            | X_D3DFMT_LIN_R6G5B5
            | X_D3DFMT_LIN_R5G5B5A1
            | X_D3DFMT_LIN_R4G4B4A4
            | X_D3DFMT_LIN_A8B8G8R8
            | X_D3DFMT_LIN_B8G8R8A8
            | X_D3DFMT_LIN_R8G8B8A8
    )
}

pub fn is_swizzled(format_code: u32) -> bool {
    !is_linear(format_code) && !is_block_compressed(format_code)
}

pub fn bytes_per_pixel(format_code: u32) -> Option<u32> {
    match format_code {
        X_D3DFMT_L8 | X_D3DFMT_AL8 | X_D3DFMT_P8 | X_D3DFMT_A8 | X_D3DFMT_LIN_L8
        | X_D3DFMT_LIN_AL8 | X_D3DFMT_LIN_A8 => Some(1),
        X_D3DFMT_A1R5G5B5
        | X_D3DFMT_X1R5G5B5
        | X_D3DFMT_A4R4G4B4
        | X_D3DFMT_R5G6B5
        | X_D3DFMT_LIN_A1R5G5B5
        | X_D3DFMT_LIN_R5G6B5
        | X_D3DFMT_A8L8
        | X_D3DFMT_LIN_X1R5G5B5
        | X_D3DFMT_LIN_A4R4G4B4
        | X_D3DFMT_LIN_A8L8
        | X_D3DFMT_R6G5B5
        | X_D3DFMT_G8B8
        | X_D3DFMT_R8B8
        | X_D3DFMT_L16
        | X_D3DFMT_LIN_L16
        | X_D3DFMT_LIN_R6G5B5
        | X_D3DFMT_R5G5B5A1
        | X_D3DFMT_R4G4B4A4
        | X_D3DFMT_LIN_R5G5B5A1
        | X_D3DFMT_LIN_R4G4B4A4 => Some(2),
        X_D3DFMT_A8R8G8B8
        | X_D3DFMT_X8R8G8B8
        | X_D3DFMT_LIN_A8R8G8B8
        | X_D3DFMT_LIN_X8R8G8B8
        | X_D3DFMT_V16U16
        | X_D3DFMT_LIN_V16U16
        | X_D3DFMT_A8B8G8R8
        | X_D3DFMT_B8G8R8A8
        | X_D3DFMT_R8G8B8A8
        | X_D3DFMT_LIN_A8B8G8R8
        | X_D3DFMT_LIN_B8G8R8A8
        | X_D3DFMT_LIN_R8G8B8A8 => Some(4),
        _ if is_block_compressed(format_code) => Some(0),
        _ => None,
    }
}

pub fn texture_pitch_bytes(width: u32, format_code: u32) -> u32 {
    match format_code {
        X_D3DFMT_DXT1 => ((width + 3) / 4).saturating_mul(8),
        X_D3DFMT_DXT3 | X_D3DFMT_DXT5 => ((width + 3) / 4).saturating_mul(16),
        _ => width.saturating_mul(bytes_per_pixel(format_code).unwrap_or(4)),
    }
}

pub fn texture_byte_len(width: u32, height: u32, format_code: u32) -> u32 {
    let pitch = texture_pitch_bytes(width, format_code);
    match format_code {
        X_D3DFMT_DXT1 | X_D3DFMT_DXT3 | X_D3DFMT_DXT5 => pitch.saturating_mul((height + 3) / 4),
        _ => pitch.saturating_mul(height),
    }
}

#[inline]
fn expand5(v: u16) -> u32 {
    (v as u32 * 255 + 15) / 31
}

#[inline]
fn expand6(v: u16) -> u32 {
    (v as u32 * 255 + 31) / 63
}

#[inline]
fn expand4(v: u16) -> u32 {
    (v as u32) * 17
}

#[inline]
fn argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    ((a & 0xFF) << 24) | ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)
}

#[inline]
fn rgb565(v: u16) -> (u32, u32, u32) {
    (
        expand5((v >> 11) & 0x1F),
        expand6((v >> 5) & 0x3F),
        expand5(v & 0x1F),
    )
}

#[inline]
fn mix2(a: u32, b: u32) -> u32 {
    (a + b) / 2
}

#[inline]
fn mix_2_1(a: u32, b: u32) -> u32 {
    (2 * a + b) / 3
}

#[inline]
fn mix_1_2(a: u32, b: u32) -> u32 {
    (a + 2 * b) / 3
}

fn dxt_color_table(block: &[u8], force_four_color: bool) -> [u32; 4] {
    let c0 = u16::from_le_bytes([block[0], block[1]]);
    let c1 = u16::from_le_bytes([block[2], block[3]]);
    let (r0, g0, b0) = rgb565(c0);
    let (r1, g1, b1) = rgb565(c1);

    if force_four_color || c0 > c1 {
        [
            argb(0xFF, r0, g0, b0),
            argb(0xFF, r1, g1, b1),
            argb(0xFF, mix_2_1(r0, r1), mix_2_1(g0, g1), mix_2_1(b0, b1)),
            argb(0xFF, mix_1_2(r0, r1), mix_1_2(g0, g1), mix_1_2(b0, b1)),
        ]
    } else {
        [
            argb(0xFF, r0, g0, b0),
            argb(0xFF, r1, g1, b1),
            argb(0xFF, mix2(r0, r1), mix2(g0, g1), mix2(b0, b1)),
            0,
        ]
    }
}

fn decode_dxt_color_block(
    color_block: &[u8],
    alpha: &[u8; 16],
    out: &mut [u32],
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
    force_four_color: bool,
) {
    let colors = dxt_color_table(color_block, force_four_color);
    let indices = u32::from_le_bytes([
        color_block[4],
        color_block[5],
        color_block[6],
        color_block[7],
    ]);
    for py in 0..4 {
        for px in 0..4 {
            let x = x0 + px;
            let y = y0 + py;
            if x >= width || y >= height {
                continue;
            }
            let i = py * 4 + px;
            let idx = ((indices >> (i * 2)) & 3) as usize;
            let rgb = colors[idx];
            out[y * width + x] = (rgb & 0x00FF_FFFF) | ((alpha[i] as u32) << 24);
        }
    }
}

/// Decode one block-compressed mip level to host ARGB32 values.
pub fn decode_block_compressed_to_argb(
    format_code: u32,
    width: u32,
    height: u32,
    bytes: &[u8],
) -> Option<Vec<u32>> {
    if !is_block_compressed(format_code) || width == 0 || height == 0 {
        return None;
    }
    let width_usize = width as usize;
    let height_usize = height as usize;
    let block_width = ((width + 3) / 4) as usize;
    let block_height = ((height + 3) / 4) as usize;
    let bytes_per_block = block_bytes(format_code)? as usize;
    let expected = block_width
        .checked_mul(block_height)?
        .checked_mul(bytes_per_block)?;
    if bytes.len() < expected {
        return None;
    }

    let mut out = vec![0u32; width_usize.checked_mul(height_usize)?];
    for by in 0..block_height {
        for bx in 0..block_width {
            let block_offset = (by * block_width + bx) * bytes_per_block;
            let block = &bytes[block_offset..block_offset + bytes_per_block];
            let mut alpha = [0xFFu8; 16];
            let color_block = match format_code {
                X_D3DFMT_DXT1 => block,
                X_D3DFMT_DXT3 => {
                    let alpha_bits = u64::from_le_bytes([
                        block[0], block[1], block[2], block[3], block[4], block[5], block[6],
                        block[7],
                    ]);
                    for (i, a) in alpha.iter_mut().enumerate() {
                        *a = (((alpha_bits >> (i * 4)) & 0xF) as u8) * 17;
                    }
                    &block[8..16]
                }
                X_D3DFMT_DXT5 => {
                    let a0 = block[0] as u32;
                    let a1 = block[1] as u32;
                    let mut alpha_bits = 0u64;
                    for i in 0..6 {
                        alpha_bits |= (block[2 + i] as u64) << (i * 8);
                    }
                    let mut table = [0u8; 8];
                    table[0] = a0 as u8;
                    table[1] = a1 as u8;
                    if a0 > a1 {
                        table[2] = ((6 * a0 + a1) / 7) as u8;
                        table[3] = ((5 * a0 + 2 * a1) / 7) as u8;
                        table[4] = ((4 * a0 + 3 * a1) / 7) as u8;
                        table[5] = ((3 * a0 + 4 * a1) / 7) as u8;
                        table[6] = ((2 * a0 + 5 * a1) / 7) as u8;
                        table[7] = ((a0 + 6 * a1) / 7) as u8;
                    } else {
                        table[2] = ((4 * a0 + a1) / 5) as u8;
                        table[3] = ((3 * a0 + 2 * a1) / 5) as u8;
                        table[4] = ((2 * a0 + 3 * a1) / 5) as u8;
                        table[5] = ((a0 + 4 * a1) / 5) as u8;
                        table[6] = 0;
                        table[7] = 255;
                    }
                    for (i, a) in alpha.iter_mut().enumerate() {
                        let idx = ((alpha_bits >> (i * 3)) & 7) as usize;
                        *a = table[idx];
                    }
                    &block[8..16]
                }
                _ => return None,
            };
            decode_dxt_color_block(
                color_block,
                &alpha,
                &mut out,
                width_usize,
                height_usize,
                bx * 4,
                by * 4,
                format_code != X_D3DFMT_DXT1,
            );
        }
    }
    Some(out)
}

/// Promote alpha-only masks to white RGB while preserving alpha. Xbox D3D8
/// titles commonly pair these masks with texture-stage modulation; if the host
/// samples them as literal black RGB, text/UI masks disappear.
pub fn promote_alpha_mask_rgb(pixels: &mut [u32]) -> bool {
    let has_alpha = pixels.iter().any(|px| (*px & 0xFF00_0000) != 0);
    let has_rgb = pixels.iter().any(|px| (*px & 0x00FF_FFFF) != 0);
    if !has_alpha {
        return false;
    }

    if has_rgb {
        // Some Xbox font atlases are DXT5 alpha masks with a constant gray RGB
        // payload in every block. DXT1 masks can instead decode transparent
        // texels as black RGB, so measure variance only across visible texels.
        // Real UI art has a much wider visible color range and must keep its
        // texture color.
        let mut min_r = u8::MAX;
        let mut min_g = u8::MAX;
        let mut min_b = u8::MAX;
        let mut max_r = 0u8;
        let mut max_g = 0u8;
        let mut max_b = 0u8;
        let mut samples = 0usize;
        for px in pixels
            .iter()
            .step_by((pixels.len() / 4096).max(1))
            .take(4096)
        {
            if (*px & 0xFF00_0000) == 0 {
                continue;
            }
            let r = ((px >> 16) & 0xFF) as u8;
            let g = ((px >> 8) & 0xFF) as u8;
            let b = (px & 0xFF) as u8;
            min_r = min_r.min(r);
            min_g = min_g.min(g);
            min_b = min_b.min(b);
            max_r = max_r.max(r);
            max_g = max_g.max(g);
            max_b = max_b.max(b);
            samples += 1;
        }
        if samples == 0
            || max_r.saturating_sub(min_r) > 12
            || max_g.saturating_sub(min_g) > 12
            || max_b.saturating_sub(min_b) > 12
        {
            return false;
        }
    }

    for px in pixels {
        if (*px & 0xFF00_0000) != 0 {
            *px |= 0x00FF_FFFF;
        } else {
            *px &= 0xFF00_0000;
        }
    }
    true
}

/// Decode one linear mip level to host ARGB32 values. P8 uses the supplied
/// Xbox D3DCOLOR palette; without a palette, it degrades to opaque grayscale
/// rather than failing the whole draw.
pub fn decode_linear_to_argb(
    format_code: u32,
    width: u32,
    height: u32,
    bytes: &[u8],
    palette: Option<&[u32; 256]>,
) -> Option<Vec<u32>> {
    if is_block_compressed(format_code) || is_depth_format(format_code) {
        return None;
    }
    let bpp = bytes_per_pixel(format_code)?;
    if bpp == 0 {
        return None;
    }
    let pixel_count = (width as usize).checked_mul(height as usize)?;
    let expected = pixel_count.checked_mul(bpp as usize)?;
    if bytes.len() < expected {
        return None;
    }

    let mut out = vec![0u32; pixel_count];
    match format_code {
        X_D3DFMT_L8 | X_D3DFMT_LIN_L8 => {
            for (dst, &l) in out.iter_mut().zip(bytes.iter()) {
                let l = l as u32;
                *dst = argb(0xFF, l, l, l);
            }
        }
        X_D3DFMT_AL8 | X_D3DFMT_LIN_AL8 => {
            for (dst, &v) in out.iter_mut().zip(bytes.iter()) {
                let l = expand4((v & 0x0F) as u16);
                let a = expand4((v >> 4) as u16);
                *dst = argb(a, l, l, l);
            }
        }
        X_D3DFMT_P8 => {
            if let Some(palette) = palette {
                for (dst, &idx) in out.iter_mut().zip(bytes.iter()) {
                    *dst = palette[idx as usize];
                }
            } else {
                for (dst, &idx) in out.iter_mut().zip(bytes.iter()) {
                    let v = idx as u32;
                    *dst = argb(0xFF, v, v, v);
                }
            }
        }
        X_D3DFMT_A8 | X_D3DFMT_LIN_A8 => {
            for (dst, &a) in out.iter_mut().zip(bytes.iter()) {
                *dst = argb(a as u32, 0xFF, 0xFF, 0xFF);
            }
        }
        X_D3DFMT_A1R5G5B5 | X_D3DFMT_LIN_A1R5G5B5 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                let a = if (v & 0x8000) != 0 { 0xFF } else { 0x00 };
                *dst = argb(
                    a,
                    expand5((v >> 10) & 0x1F),
                    expand5((v >> 5) & 0x1F),
                    expand5(v & 0x1F),
                );
            }
        }
        X_D3DFMT_X1R5G5B5 | X_D3DFMT_LIN_X1R5G5B5 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                *dst = argb(
                    0xFF,
                    expand5((v >> 10) & 0x1F),
                    expand5((v >> 5) & 0x1F),
                    expand5(v & 0x1F),
                );
            }
        }
        X_D3DFMT_A4R4G4B4 | X_D3DFMT_LIN_A4R4G4B4 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                *dst = argb(
                    expand4((v >> 12) & 0xF),
                    expand4((v >> 8) & 0xF),
                    expand4((v >> 4) & 0xF),
                    expand4(v & 0xF),
                );
            }
        }
        X_D3DFMT_R5G6B5 | X_D3DFMT_LIN_R5G6B5 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                *dst = argb(
                    0xFF,
                    expand5((v >> 11) & 0x1F),
                    expand6((v >> 5) & 0x3F),
                    expand5(v & 0x1F),
                );
            }
        }
        X_D3DFMT_A8L8 | X_D3DFMT_LIN_A8L8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let l = src[0] as u32;
                let a = src[1] as u32;
                *dst = argb(a, l, l, l);
            }
        }
        X_D3DFMT_R6G5B5 | X_D3DFMT_LIN_R6G5B5 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                let r = ((v >> 10) & 0x3F) as u32 * 255 / 63;
                let g = ((v >> 5) & 0x1F) as u32 * 255 / 31;
                let b = (v & 0x1F) as u32 * 255 / 31;
                *dst = argb(0xFF, r, g, b);
            }
        }
        X_D3DFMT_R5G5B5A1 | X_D3DFMT_LIN_R5G5B5A1 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                let a = if (v & 0x0001) != 0 { 0xFF } else { 0x00 };
                *dst = argb(
                    a,
                    expand5((v >> 11) & 0x1F),
                    expand5((v >> 6) & 0x1F),
                    expand5((v >> 1) & 0x1F),
                );
            }
        }
        X_D3DFMT_R4G4B4A4 | X_D3DFMT_LIN_R4G4B4A4 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let v = u16::from_le_bytes([src[0], src[1]]);
                *dst = argb(
                    expand4(v & 0xF),
                    expand4((v >> 12) & 0xF),
                    expand4((v >> 8) & 0xF),
                    expand4((v >> 4) & 0xF),
                );
            }
        }
        X_D3DFMT_L16 | X_D3DFMT_LIN_L16 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                let l = (u16::from_le_bytes([src[0], src[1]]) >> 8) as u32;
                *dst = argb(0xFF, l, l, l);
            }
        }
        X_D3DFMT_G8B8 | X_D3DFMT_R8B8 | X_D3DFMT_LIN_R8B8 | X_D3DFMT_LIN_G8B8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(2)) {
                *dst = argb(0xFF, src[1] as u32, src[0] as u32, 0);
            }
        }
        X_D3DFMT_A8R8G8B8 | X_D3DFMT_LIN_A8R8G8B8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(4)) {
                *dst = u32::from_le_bytes([src[0], src[1], src[2], src[3]]);
            }
        }
        X_D3DFMT_X8R8G8B8 | X_D3DFMT_LIN_X8R8G8B8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(4)) {
                *dst = u32::from_le_bytes([src[0], src[1], src[2], src[3]]) | 0xFF00_0000;
            }
        }
        X_D3DFMT_A8B8G8R8 | X_D3DFMT_LIN_A8B8G8R8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(4)) {
                *dst = argb(src[3] as u32, src[0] as u32, src[1] as u32, src[2] as u32);
            }
        }
        X_D3DFMT_B8G8R8A8 | X_D3DFMT_LIN_B8G8R8A8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(4)) {
                *dst = argb(src[0] as u32, src[1] as u32, src[2] as u32, src[3] as u32);
            }
        }
        X_D3DFMT_R8G8B8A8 | X_D3DFMT_LIN_R8G8B8A8 => {
            for (dst, src) in out.iter_mut().zip(bytes.chunks_exact(4)) {
                *dst = argb(src[0] as u32, src[3] as u32, src[2] as u32, src[1] as u32);
            }
        }
        _ => return None,
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_dxt5_alpha_indices(indices: [u8; 16]) -> [u8; 6] {
        let mut bits = 0u64;
        for (i, idx) in indices.iter().enumerate() {
            bits |= ((*idx as u64) & 7) << (i * 3);
        }
        let le = bits.to_le_bytes();
        [le[0], le[1], le[2], le[3], le[4], le[5]]
    }

    fn pack_dxt_color_indices(indices: [u8; 16]) -> [u8; 4] {
        let mut bits = 0u32;
        for (i, idx) in indices.iter().enumerate() {
            bits |= ((*idx as u32) & 3) << (i * 2);
        }
        bits.to_le_bytes()
    }

    #[test]
    fn p8_uses_palette() {
        let mut pal = [0u32; 256];
        pal[2] = 0xFF11_2233;
        pal[7] = 0x8044_5566;
        let out = decode_linear_to_argb(X_D3DFMT_P8, 2, 1, &[2, 7], Some(&pal)).unwrap();
        assert_eq!(out, vec![0xFF11_2233, 0x8044_5566]);
    }

    #[test]
    fn x8_forces_opaque_alpha() {
        let out =
            decode_linear_to_argb(X_D3DFMT_X8R8G8B8, 1, 1, &[0x33, 0x22, 0x11, 0], None).unwrap();
        assert_eq!(out[0], 0xFF11_2233);
    }

    #[test]
    fn rgb565_expands_components() {
        let out =
            decode_linear_to_argb(X_D3DFMT_R5G6B5, 1, 1, &0xF800u16.to_le_bytes(), None).unwrap();
        assert_eq!(out[0], 0xFFFF_0000);
    }

    #[test]
    fn a8_decodes_as_white_alpha_mask() {
        let out = decode_linear_to_argb(X_D3DFMT_A8, 2, 1, &[0x00, 0x80], None).unwrap();
        assert_eq!(out, vec![0x00FF_FFFF, 0x80FF_FFFF]);
    }

    #[test]
    fn dxt3_decodes_explicit_alpha_and_crops_partial_block() {
        let mut block = [0u8; 16];
        let mut alpha_bits = 0u64;
        for i in 0..16 {
            alpha_bits |= ((i as u64) & 0xF) << (i * 4);
        }
        block[0..8].copy_from_slice(&alpha_bits.to_le_bytes());
        block[8..10].copy_from_slice(&0xF800u16.to_le_bytes()); // red
        block[10..12].copy_from_slice(&0x07E0u16.to_le_bytes()); // green
        block[12..16].copy_from_slice(&pack_dxt_color_indices([0; 16]));

        let out = decode_block_compressed_to_argb(X_D3DFMT_DXT3, 2, 2, &block).unwrap();
        assert_eq!(
            out,
            vec![
                0x00FF_0000, // texel 0 alpha nibble 0
                0x11FF_0000, // texel 1 alpha nibble 1
                0x44FF_0000, // texel 4 alpha nibble 4
                0x55FF_0000, // texel 5 alpha nibble 5
            ]
        );
    }

    #[test]
    fn dxt5_decodes_alpha_table_and_forces_four_color_mode() {
        let alpha_indices = [0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7];
        let color_indices = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

        let mut block = [0u8; 16];
        block[0] = 255;
        block[1] = 0;
        block[2..8].copy_from_slice(&pack_dxt5_alpha_indices(alpha_indices));

        // Deliberately use color0 < color1. DXT1 would switch to three-color
        // mode here, but DXT5 must always decode a four-color color block.
        block[8..10].copy_from_slice(&0x001Fu16.to_le_bytes()); // blue
        block[10..12].copy_from_slice(&0xF800u16.to_le_bytes()); // red
        block[12..16].copy_from_slice(&pack_dxt_color_indices(color_indices));

        let out = decode_block_compressed_to_argb(X_D3DFMT_DXT5, 4, 4, &block).unwrap();
        let expected_alpha = [
            255, 0, 218, 182, 145, 109, 72, 36, 255, 0, 218, 182, 145, 109, 72, 36,
        ];
        let expected_rgb = [0x0000_00FF, 0x00FF_0000, 0x0055_00AA, 0x00AA_0055];

        for i in 0..16 {
            assert_eq!(
                out[i],
                ((expected_alpha[i] as u32) << 24) | expected_rgb[i % 4],
                "texel {i}"
            );
        }
    }

    #[test]
    fn alpha_mask_promotion_keeps_varied_rgb_art() {
        let mut pixels = vec![0xFFFF_0000, 0xFF00_FF00, 0xFF00_00FF, 0x8080_4020];

        assert!(!promote_alpha_mask_rgb(&mut pixels));
        assert_eq!(
            pixels,
            vec![0xFFFF_0000, 0xFF00_FF00, 0xFF00_00FF, 0x8080_4020]
        );
    }

    #[test]
    fn alpha_mask_promotion_whitens_low_variance_masks() {
        let mut pixels = vec![0x800A_0B0C, 0x000A_0B0C, 0x400B_0B0C];

        assert!(promote_alpha_mask_rgb(&mut pixels));
        assert_eq!(pixels, vec![0x80FF_FFFF, 0x0000_0000, 0x40FF_FFFF]);
    }

    #[test]
    fn dxt_formats_are_native_compressed_not_swizzled() {
        for (format, block_size) in [(X_D3DFMT_DXT1, 8), (X_D3DFMT_DXT3, 16), (X_D3DFMT_DXT5, 16)] {
            assert!(is_block_compressed(format), "fmt=0x{format:02X}");
            assert!(!is_swizzled(format), "fmt=0x{format:02X}");
            assert_eq!(block_bytes(format), Some(block_size), "fmt=0x{format:02X}");
        }
        assert!(is_swizzled_block_compressed(X_D3DFMT_DXT1));
        assert!(!is_swizzled_block_compressed(X_D3DFMT_DXT3));
        assert!(!is_swizzled_block_compressed(X_D3DFMT_DXT5));
    }

    #[test]
    fn dxt_byte_lengths_round_up_to_whole_blocks() {
        assert_eq!(texture_byte_len(1, 1, X_D3DFMT_DXT1), 8);
        assert_eq!(texture_byte_len(5, 5, X_D3DFMT_DXT3), 64);
        assert_eq!(texture_pitch_bytes(5, X_D3DFMT_DXT5), 32);
    }
}
