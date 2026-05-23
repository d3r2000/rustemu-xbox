//! Xbox D3D texture resource format helpers.
//!
//! X_D3DResource header is 20 bytes: Common, Data, Lock, Format, Size.
//! Format field bit layout (per Cxbx-R XbD3D8Types.h):
//!   bits [3:0]   = mipmap levels - 1
//!   bits [7:4]   = dimension
//!   bits [15:8]  = format code (DXT1=0x0C, A8R8G8B8=0x06, LIN_A8R8G8B8=0x12, DXT5=0x0F)
//!   bits [23:20] = log2(width)
//!   bits [27:24] = log2(height)
//!
//! Used by the IDirect3DResource8::Register hook to classify guest textures
//! before uploading to the D3D11 backend.

use crate::xbox::gpu::texture_format as tf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XboxTexHeader {
    pub format_code: u32,
    pub width: u32,
    pub height: u32,
    pub mip_count: u32,
    pub data_offset: u32, // bytes from pBase (the Data field value)
    pub is_swizzled: bool,
    pub is_compressed: bool,
    pub bytes_per_pixel: u32, // 0 for compressed, else 1/2/4
}

impl XboxTexHeader {
    /// Decode the 20-byte X_D3DResource header starting at `bytes[0]`.
    pub fn from_resource_header(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        let data = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
        let format_dw = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
        let format_code = (format_dw >> 8) & 0xFF;
        let mip_count = ((format_dw) & 0xF) + 1;
        let log2w = (format_dw >> 20) & 0xF;
        let log2h = (format_dw >> 24) & 0xF;
        let width = 1u32 << log2w;
        let height = 1u32 << log2h;
        let is_compressed = tf::is_block_compressed(format_code);
        let is_swizzled = tf::is_swizzled(format_code);
        let bytes_per_pixel = tf::bytes_per_pixel(format_code).unwrap_or(0);
        Some(Self {
            format_code,
            width,
            height,
            mip_count,
            data_offset: data,
            is_swizzled,
            is_compressed,
            bytes_per_pixel,
        })
    }

    /// Total bytes of compressed-or-raw pixel data for mip level 0.
    pub fn mip0_size(&self) -> usize {
        tf::texture_byte_len(self.width, self.height, self.format_code) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_a8r8g8b8_128x128() {
        // Format dw: format=0x06, log2w=7, log2h=7, levels=0 → mip_count=1
        let format_dw: u32 = (0x06 << 8) | (7 << 20) | (7 << 24);
        let mut hdr = [0u8; 20];
        hdr[4..8].copy_from_slice(&0x1000u32.to_le_bytes()); // Data
        hdr[12..16].copy_from_slice(&format_dw.to_le_bytes());
        let t = XboxTexHeader::from_resource_header(&hdr).unwrap();
        assert_eq!(t.format_code, 0x06);
        assert_eq!(t.width, 128);
        assert_eq!(t.height, 128);
        assert_eq!(t.mip_count, 1);
        assert!(t.is_swizzled);
        assert!(!t.is_compressed);
        assert_eq!(t.bytes_per_pixel, 4);
        assert_eq!(t.mip0_size(), 128 * 128 * 4);
    }

    #[test]
    fn decode_dxt1_256x256() {
        let format_dw: u32 = (0x0C << 8) | (8 << 20) | (8 << 24);
        let mut hdr = [0u8; 20];
        hdr[12..16].copy_from_slice(&format_dw.to_le_bytes());
        let t = XboxTexHeader::from_resource_header(&hdr).unwrap();
        assert_eq!(t.format_code, 0x0C);
        assert_eq!(t.width, 256);
        assert_eq!(t.height, 256);
        assert!(t.is_compressed);
        assert_eq!(t.mip0_size(), 256 * 256 / 2);
    }

    #[test]
    fn decode_dxt3_512x256_with_mips() {
        let format_dw: u32 = 2 | (0x0E << 8) | (9 << 20) | (8 << 24);
        let mut hdr = [0u8; 20];
        hdr[4..8].copy_from_slice(&0x4000u32.to_le_bytes());
        hdr[12..16].copy_from_slice(&format_dw.to_le_bytes());

        let t = XboxTexHeader::from_resource_header(&hdr).unwrap();
        assert_eq!(t.format_code, tf::X_D3DFMT_DXT3);
        assert_eq!(t.width, 512);
        assert_eq!(t.height, 256);
        assert_eq!(t.mip_count, 3);
        assert_eq!(t.data_offset, 0x4000);
        assert!(t.is_compressed);
        assert!(!t.is_swizzled);
        assert_eq!(t.bytes_per_pixel, 0);
        assert_eq!(t.mip0_size(), 512 * 256);
    }

    #[test]
    fn decode_linear_a8r8g8b8_is_not_swizzled() {
        let format_dw: u32 = 3 | (tf::X_D3DFMT_LIN_A8R8G8B8 << 8) | (7 << 20) | (6 << 24);
        let mut hdr = [0u8; 20];
        hdr[12..16].copy_from_slice(&format_dw.to_le_bytes());

        let t = XboxTexHeader::from_resource_header(&hdr).unwrap();
        assert_eq!(t.format_code, tf::X_D3DFMT_LIN_A8R8G8B8);
        assert_eq!(t.width, 128);
        assert_eq!(t.height, 64);
        assert_eq!(t.mip_count, 4);
        assert!(!t.is_swizzled);
        assert!(!t.is_compressed);
        assert_eq!(t.bytes_per_pixel, 4);
        assert_eq!(t.mip0_size(), 128 * 64 * 4);
    }

    #[test]
    fn short_resource_header_is_rejected() {
        assert!(XboxTexHeader::from_resource_header(&[0u8; 19]).is_none());
    }
}
