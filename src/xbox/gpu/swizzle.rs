//! Xbox texture swizzle / unswizzle (Morton / Z-order interleave).
//!
//! Xbox textures stored by the NV2A in "swizzled" form are laid out in Morton
//! order: each output pixel's x and y bits are interleaved to produce the
//! swizzled offset. For square power-of-2 textures this is the classic
//! Z-order curve. For non-square power-of-2 textures (e.g. 256x128) the
//! interleave only covers `min(log2 w, log2 h)` bits of each axis; the
//! leftover high bits of the larger axis are appended linearly at the top
//! of the swizzled offset.
//!
//! Supported `bytes_per_pixel` values: 1, 2, 4 (L8/A8/P8, R5G6B5/ARGB1555/
//! ARGB4444/L8A8, ARGB8888/XRGB8888). DXT compressed formats use the same
//! Morton addressing at 4x4 block granularity; use [`unswizzle_blocks`] for
//! those instead of treating compressed payloads as linear rows.
//!
//! `D3DFMT_LIN_*` formats are already linear and must bypass this module —
//! format identification lives in the sibling `d3dfmt` module.

/// Convert a swizzled texture to linear row-major layout.
///
/// `swizzled`: source bytes in Xbox swizzled format. Length must equal
/// `width * height * bytes_per_pixel`.
/// `width`, `height`: power-of-2 pixel dimensions.
/// `bytes_per_pixel`: 1, 2, or 4.
///
/// Returns a new linear buffer of the same size, or `None` on invalid input
/// (non-power-of-2 dimensions, unsupported bpp, or length mismatch).
pub fn unswizzle(
    swizzled: &[u8],
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
) -> Option<Vec<u8>> {
    let (w, h, bpp) = validate(swizzled, width, height, bytes_per_pixel)?;
    let bpp = bpp as usize;
    let len = swizzled.len();
    let mut out = vec![0u8; len];

    let (x_mask, y_mask) = build_masks(w, h);

    // Iterate destination (linear) pixels. For each (x, y) compute source
    // (swizzled) offset via bit-scatter into x_mask / y_mask.
    for y in 0..h {
        let ys = spread(y, y_mask);
        let dst_row = (y as usize) * (w as usize) * bpp;
        for x in 0..w {
            let src_pixel = (spread(x, x_mask) | ys) as usize;
            let src_byte = src_pixel * bpp;
            let dst_byte = dst_row + (x as usize) * bpp;
            out[dst_byte..dst_byte + bpp].copy_from_slice(&swizzled[src_byte..src_byte + bpp]);
        }
    }
    Some(out)
}

/// Reverse of [`unswizzle`]: pack a linear row-major texture into Xbox
/// swizzled layout. Used when the host writes to a CPU-mapped texture that
/// the guest will read back through the NV2A texture unit.
pub fn swizzle(linear: &[u8], width: u32, height: u32, bytes_per_pixel: u32) -> Option<Vec<u8>> {
    let (w, h, bpp) = validate(linear, width, height, bytes_per_pixel)?;
    let bpp = bpp as usize;
    let len = linear.len();
    let mut out = vec![0u8; len];

    let (x_mask, y_mask) = build_masks(w, h);

    for y in 0..h {
        let ys = spread(y, y_mask);
        let src_row = (y as usize) * (w as usize) * bpp;
        for x in 0..w {
            let dst_pixel = (spread(x, x_mask) | ys) as usize;
            let dst_byte = dst_pixel * bpp;
            let src_byte = src_row + (x as usize) * bpp;
            out[dst_byte..dst_byte + bpp].copy_from_slice(&linear[src_byte..src_byte + bpp]);
        }
    }
    Some(out)
}

/// Convert a swizzled block-compressed texture to linear block rows.
///
/// `block_width` and `block_height` are counts of compressed 4x4 blocks, not
/// pixel dimensions. They do not need to be powers of two; some swizzled
/// layouts address a padded Morton container, so `swizzled` may be larger than
/// the returned linear top-level block payload. `bytes_per_block` is 8 for
/// DXT1/BC1 and 16 for DXT3/5.
pub fn unswizzle_blocks(
    swizzled: &[u8],
    block_width: u32,
    block_height: u32,
    bytes_per_block: u32,
) -> Option<Vec<u8>> {
    let (w, h, bpb) = validate_blocks(swizzled, block_width, block_height, bytes_per_block)?;
    let bpb = bpb as usize;
    let len = (w as usize).checked_mul(h as usize)?.checked_mul(bpb)?;
    let mut out = vec![0u8; len];

    let (x_mask, y_mask) = build_masks(w, h);

    for y in 0..h {
        let ys = spread(y, y_mask);
        let dst_row = (y as usize) * (w as usize) * bpb;
        for x in 0..w {
            let src_block = (spread(x, x_mask) | ys) as usize;
            let src_byte = src_block * bpb;
            let dst_byte = dst_row + (x as usize) * bpb;
            if src_byte.checked_add(bpb)? > swizzled.len() {
                return None;
            }
            out[dst_byte..dst_byte + bpb].copy_from_slice(&swizzled[src_byte..src_byte + bpb]);
        }
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

/// Validate inputs and return `(width, height, bpp)` if all checks pass.
fn validate(buf: &[u8], width: u32, height: u32, bytes_per_pixel: u32) -> Option<(u32, u32, u32)> {
    if width == 0 || height == 0 {
        return None;
    }
    if !width.is_power_of_two() || !height.is_power_of_two() {
        return None;
    }
    if !matches!(bytes_per_pixel, 1 | 2 | 4) {
        return None;
    }
    let expected = (width as usize)
        .checked_mul(height as usize)?
        .checked_mul(bytes_per_pixel as usize)?;
    if buf.len() != expected {
        return None;
    }
    Some((width, height, bytes_per_pixel))
}

fn validate_blocks(
    buf: &[u8],
    block_width: u32,
    block_height: u32,
    bytes_per_block: u32,
) -> Option<(u32, u32, u32)> {
    if block_width == 0 || block_height == 0 {
        return None;
    }
    if !matches!(bytes_per_block, 8 | 16) {
        return None;
    }
    let expected = (block_width as usize)
        .checked_mul(block_height as usize)?
        .checked_mul(bytes_per_block as usize)?;
    if buf.len() < expected {
        return None;
    }
    Some((block_width, block_height, bytes_per_block))
}

/// Build `(x_mask, y_mask)` such that the swizzled offset of pixel (x, y) is
/// `spread(x, x_mask) | spread(y, y_mask)`.
///
/// The masks are disjoint and cover enough bits to address the supplied width
/// and height. In the interleaved region (low bits) x and y alternate. Beyond
/// `min(log2 w, log2 h)` bits, the leftover high bits of the larger axis are
/// packed linearly at the top. This mirrors xemu's mask-generation loop and
/// also works for non-power-of-two block counts.
fn build_masks(width: u32, height: u32) -> (u32, u32) {
    let mut x_mask: u32 = 0;
    let mut y_mask: u32 = 0;
    let mut axis_bit: u32 = 1;
    let mut mask_bit: u32 = 1;

    // Walk bit positions from LSB up. At each position, allocate the bit
    // to x if x still has bits left, then to y if y still has bits left.
    // This produces the classic Morton interleave in the overlapping
    // range and a linear tail for the larger dimension.
    while axis_bit < width || axis_bit < height {
        if axis_bit < width {
            x_mask |= mask_bit;
            mask_bit <<= 1;
        }
        if axis_bit < height {
            y_mask |= mask_bit;
            mask_bit <<= 1;
        }
        axis_bit <<= 1;
    }
    (x_mask, y_mask)
}

/// Scatter the low bits of `value` into the positions indicated by `mask`.
///
/// Bit 0 of `value` lands at the lowest set bit of `mask`, bit 1 at the
/// next set bit of `mask`, and so on. Unused high bits of `value` are
/// ignored.
#[inline]
fn spread(value: u32, mask: u32) -> u32 {
    let mut result: u32 = 0;
    let mut m = mask;
    let mut v = value;
    while m != 0 {
        let low = m & m.wrapping_neg(); // isolate lowest set bit
        if v & 1 != 0 {
            result |= low;
        }
        v >>= 1;
        m &= m - 1; // clear lowest set bit
    }
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// 2x2 is the trivial Morton case: linear == swizzled in pixel order.
    #[test]
    fn unswizzle_2x2_identity() {
        let swizzled = vec![b'A', b'B', b'C', b'D'];
        let out = unswizzle(&swizzled, 2, 2, 1).expect("2x2 bpp=1");
        // Linear layout of pixels: row 0 = A B, row 1 = C D.
        assert_eq!(out, vec![b'A', b'B', b'C', b'D']);
    }

    /// 4x4 Morton order visits pixels in this sequence:
    /// (0,0) (1,0) (0,1) (1,1) (2,0) (3,0) (2,1) (3,1)
    /// (0,2) (1,2) (0,3) (1,3) (2,2) (3,2) (2,3) (3,3)
    ///
    /// We hand-construct swizzled bytes where byte i = swizzle order i and
    /// verify the linear output places them at the expected (x,y) positions.
    #[test]
    fn unswizzle_4x4_known_pattern() {
        // Swizzled buffer: byte[i] = i (so we can read the Morton order directly).
        let swizzled: Vec<u8> = (0..16u8).collect();

        // Expected linear[y*4 + x] = Morton index of (x, y).
        // Morton index: interleave x and y bits where x owns even bits,
        // y owns odd bits of the swizzled offset.
        let mut expected = vec![0u8; 16];
        for y in 0..4u32 {
            for x in 0..4u32 {
                // interleave x bits at positions 0,2 and y bits at 1,3.
                let m = (x & 1) | ((y & 1) << 1) | ((x & 2) << 1) | ((y & 2) << 2);
                expected[(y as usize) * 4 + x as usize] = m as u8;
            }
        }
        let out = unswizzle(&swizzled, 4, 4, 1).expect("4x4 bpp=1");
        assert_eq!(out, expected);

        // Spot-check the order stated in the task description.
        // Swizzle index -> (x, y):
        // 0 -> (0,0), 1 -> (1,0), 2 -> (0,1), 3 -> (1,1),
        // 4 -> (2,0), 5 -> (3,0), 6 -> (2,1), 7 -> (3,1),
        // 8 -> (0,2), 9 -> (1,2), 10 -> (0,3), 11 -> (1,3),
        // 12 -> (2,2), 13 -> (3,2), 14 -> (2,3), 15 -> (3,3).
        let at = |x: u32, y: u32| out[(y * 4 + x) as usize];
        assert_eq!(at(0, 0), 0);
        assert_eq!(at(1, 0), 1);
        assert_eq!(at(0, 1), 2);
        assert_eq!(at(1, 1), 3);
        assert_eq!(at(2, 0), 4);
        assert_eq!(at(3, 0), 5);
        assert_eq!(at(2, 1), 6);
        assert_eq!(at(3, 1), 7);
        assert_eq!(at(0, 2), 8);
        assert_eq!(at(1, 2), 9);
        assert_eq!(at(0, 3), 10);
        assert_eq!(at(1, 3), 11);
        assert_eq!(at(2, 2), 12);
        assert_eq!(at(3, 2), 13);
        assert_eq!(at(2, 3), 14);
        assert_eq!(at(3, 3), 15);
    }

    /// Round-trip: linear -> swizzle -> unswizzle should recover the original
    /// for an 8x8 texture with 1 bpp.
    #[test]
    fn roundtrip_8x8_bpp1() {
        let linear: Vec<u8> = (0..64u8).collect();
        let sw = swizzle(&linear, 8, 8, 1).expect("swizzle 8x8");
        let back = unswizzle(&sw, 8, 8, 1).expect("unswizzle 8x8");
        assert_eq!(back, linear);
    }

    /// Round-trip 16x16 with bpp=4 (XRGB8888).
    #[test]
    fn roundtrip_16x16_bpp4() {
        // Use a distinctive per-pixel 32-bit value so any byte swap shows up.
        let mut linear = Vec::with_capacity(16 * 16 * 4);
        for i in 0..16u32 * 16 {
            let v = 0xDEAD0000u32 | i;
            linear.extend_from_slice(&v.to_le_bytes());
        }
        let sw = swizzle(&linear, 16, 16, 4).expect("swizzle 16x16 bpp=4");
        let back = unswizzle(&sw, 16, 16, 4).expect("unswizzle 16x16 bpp=4");
        assert_eq!(back, linear);
    }

    /// Round-trip 16x16 with bpp=2 (R5G6B5-sized elements).
    #[test]
    fn roundtrip_16x16_bpp2() {
        let mut linear = Vec::with_capacity(16 * 16 * 2);
        for i in 0..16u16 * 16 {
            linear.extend_from_slice(&(i ^ 0xAAAA).to_le_bytes());
        }
        let sw = swizzle(&linear, 16, 16, 2).expect("swizzle 16x16 bpp=2");
        let back = unswizzle(&sw, 16, 16, 2).expect("unswizzle 16x16 bpp=2");
        assert_eq!(back, linear);
    }

    #[test]
    fn unswizzle_blocks_4x4_known_pattern() {
        let bytes_per_block = 8usize;
        let mut swizzled = Vec::with_capacity(16 * bytes_per_block);
        for block_id in 0..16u8 {
            swizzled.extend(std::iter::repeat(block_id).take(bytes_per_block));
        }

        let out = unswizzle_blocks(&swizzled, 4, 4, bytes_per_block as u32)
            .expect("4x4 DXT1 block unswizzle");
        let linear_block_ids: Vec<u8> = out
            .chunks_exact(bytes_per_block)
            .map(|block| block[0])
            .collect();

        assert_eq!(
            linear_block_ids,
            vec![0, 1, 4, 5, 2, 3, 6, 7, 8, 9, 12, 13, 10, 11, 14, 15]
        );
    }

    #[test]
    fn unswizzle_blocks_4x2_preserves_16_byte_blocks() {
        let bytes_per_block = 16usize;
        let mut swizzled = Vec::with_capacity(8 * bytes_per_block);
        for block_id in 0..8u8 {
            for byte in 0..bytes_per_block as u8 {
                swizzled.push((block_id << 4) | byte);
            }
        }

        let out = unswizzle_blocks(&swizzled, 4, 2, bytes_per_block as u32)
            .expect("4x2 DXT5 block unswizzle");
        let linear_block_ids: Vec<u8> = out
            .chunks_exact(bytes_per_block)
            .map(|block| block[0] >> 4)
            .collect();

        assert_eq!(linear_block_ids, vec![0, 1, 4, 5, 2, 3, 6, 7]);
        assert_eq!(&out[0..16], &(0u8..16u8).collect::<Vec<_>>()[..]);
        assert_eq!(&out[32..48], &(0x40u8..0x50u8).collect::<Vec<_>>()[..]);
    }

    #[test]
    fn unswizzle_blocks_non_power_of_two_uses_padded_morton_container() {
        let bytes_per_block = 8usize;
        let block_width = 3u32;
        let block_height = 2u32;
        let (x_mask, y_mask) = build_masks(block_width, block_height);

        let mut max_src_block = 0usize;
        for y in 0..block_height {
            let ys = spread(y, y_mask);
            for x in 0..block_width {
                max_src_block = max_src_block.max((spread(x, x_mask) | ys) as usize);
            }
        }

        let mut swizzled = Vec::with_capacity((max_src_block + 1) * bytes_per_block);
        for block_id in 0..=max_src_block as u8 {
            swizzled.extend(std::iter::repeat(block_id).take(bytes_per_block));
        }

        let out = unswizzle_blocks(&swizzled, block_width, block_height, bytes_per_block as u32)
            .expect("3x2 DXT1 block unswizzle with padded source");
        let linear_block_ids: Vec<u8> = out
            .chunks_exact(bytes_per_block)
            .map(|block| block[0])
            .collect();

        assert_eq!(linear_block_ids, vec![0, 1, 4, 2, 3, 6]);
        assert_eq!(
            out.len(),
            (block_width * block_height) as usize * bytes_per_block
        );
    }

    /// Non-square 4x2: width=4, height=2. min(log2 w, log2 h) = 1.
    /// Interleave 1 bit of each axis (bits 0=x0, 1=y0), then the leftover
    /// high x bit (bit 2=x1) is appended linearly at the top.
    ///
    /// So x_mask = 0b101 (bits 0 and 2), y_mask = 0b010 (bit 1).
    /// Swizzle order: offset i -> (x, y) where
    ///   y = (i >> 1) & 1
    ///   x = (i & 1) | ((i >> 1) & 2)  // bit0=x0, bit2=x1
    ///
    /// i=0: x=0,y=0;  i=1: x=1,y=0;  i=2: x=0,y=1;  i=3: x=1,y=1;
    /// i=4: x=2,y=0;  i=5: x=3,y=0;  i=6: x=2,y=1;  i=7: x=3,y=1.
    #[test]
    fn non_square_4x2_leftover_bits() {
        let swizzled: Vec<u8> = (0..8u8).collect();
        let out = unswizzle(&swizzled, 4, 2, 1).expect("4x2 bpp=1");

        // Expected linear[y*4 + x] = morton index as above.
        // row 0 (y=0): (0,0)=0, (1,0)=1, (2,0)=4, (3,0)=5
        // row 1 (y=1): (0,1)=2, (1,1)=3, (2,1)=6, (3,1)=7
        let expected = vec![0, 1, 4, 5, 2, 3, 6, 7];
        assert_eq!(out, expected);

        // Also verify the masks we generate match the rule.
        let (xm, ym) = build_masks(4, 2);
        assert_eq!(xm, 0b101, "x_mask for 4x2 should be 0b101");
        assert_eq!(ym, 0b010, "y_mask for 4x2 should be 0b010");
    }

    #[test]
    fn non_square_4x2_bpp4_swizzle_preserves_texel_bytes() {
        let mut linear = Vec::new();
        for p in 0..8u8 {
            linear.extend_from_slice(&[p, p.wrapping_add(0x10), p.wrapping_add(0x20), 0xA0 | p]);
        }

        let sw = swizzle(&linear, 4, 2, 4).expect("4x2 bpp=4");
        let swizzled_texel_ids: Vec<u8> = sw.chunks_exact(4).map(|texel| texel[0]).collect();

        assert_eq!(swizzled_texel_ids, vec![0, 1, 4, 5, 2, 3, 6, 7]);
        assert_eq!(&sw[0..4], &[0x00, 0x10, 0x20, 0xA0]);
        assert_eq!(&sw[8..12], &[0x04, 0x14, 0x24, 0xA4]);
    }

    /// Tall non-square 2x8: height > width.
    /// log2 w = 1, log2 h = 3. The interleaved region is 1 bit of each
    /// (bit 0 = x0, bit 1 = y0), then the two leftover y bits (y1, y2)
    /// are packed linearly on top at bits 2, 3.
    ///
    /// x_mask = 0b0001 = 1
    /// y_mask = 0b1110 = 14
    #[test]
    fn non_square_2x8_roundtrip() {
        let linear: Vec<u8> = (0..16u8).collect();
        let sw = swizzle(&linear, 2, 8, 1).expect("2x8 swizzle");
        let back = unswizzle(&sw, 2, 8, 1).expect("2x8 unswizzle");
        assert_eq!(back, linear);

        let (xm, ym) = build_masks(2, 8);
        assert_eq!(xm, 0b0001);
        assert_eq!(ym, 0b1110);
    }

    /// Non-power-of-2 dimensions must be rejected.
    #[test]
    fn invalid_non_power_of_two_rejected() {
        // Width 3 is not a power of 2.
        let buf = vec![0u8; 3 * 4];
        assert!(unswizzle(&buf, 3, 4, 1).is_none());
        assert!(swizzle(&buf, 3, 4, 1).is_none());

        // Height 6 is not a power of 2.
        let buf = vec![0u8; 4 * 6];
        assert!(unswizzle(&buf, 4, 6, 1).is_none());

        // Zero dimensions rejected.
        assert!(unswizzle(&[], 0, 4, 1).is_none());
        assert!(unswizzle(&[], 4, 0, 1).is_none());
    }

    /// Invalid bytes_per_pixel values rejected.
    #[test]
    fn invalid_bpp_rejected() {
        let buf = vec![0u8; 4 * 4 * 3];
        assert!(unswizzle(&buf, 4, 4, 3).is_none());
        assert!(unswizzle(&buf, 4, 4, 0).is_none());
        assert!(unswizzle(&buf, 4, 4, 8).is_none());
    }

    /// Buffer length mismatch rejected.
    #[test]
    fn invalid_length_rejected() {
        let buf = vec![0u8; 15]; // expected 16 for 4x4x1
        assert!(unswizzle(&buf, 4, 4, 1).is_none());
    }

    /// build_masks for a square power-of-2 produces the classic interleave.
    #[test]
    fn build_masks_square() {
        // 4x4: bits 0,2 -> x; bits 1,3 -> y.
        let (xm, ym) = build_masks(4, 4);
        assert_eq!(xm, 0b0101);
        assert_eq!(ym, 0b1010);

        // 8x8: bits 0,2,4 -> x; bits 1,3,5 -> y.
        let (xm, ym) = build_masks(8, 8);
        assert_eq!(xm, 0b010101);
        assert_eq!(ym, 0b101010);
    }
}
