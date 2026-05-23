//! Texture cache layer.
//!
//! Owns Xbox texture identity, compressed format decode, host SRV upload, and
//! texture-stage sampler binding for the D3D11 backend.

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::*;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::*;

use super::D3D11Backend;

impl D3D11Backend {
    fn texture_readback_probe_enabled() -> bool {
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ENABLED.get_or_init(|| {
            std::env::var("RUSTEMU_D3D11_TEXTURE_READBACK")
                .or_else(|_| std::env::var("RUSTEMU_DOOM_D3D11_TEX_READBACK"))
                .map(|v| {
                    let v = v.trim();
                    v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
                })
                .unwrap_or(false)
        })
    }

    fn argb_hash(pixels: &[u32]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for &px in pixels {
            for byte in px.to_le_bytes() {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
            }
        }
        hash
    }

    fn crop_argb(
        pixels: &[u32],
        width: u32,
        height: u32,
        crop_w: u32,
        crop_h: u32,
    ) -> Option<Vec<u32>> {
        if crop_w == 0 || crop_h == 0 || crop_w > width || crop_h > height {
            return None;
        }
        let src_w = width as usize;
        let crop_w = crop_w as usize;
        let crop_h = crop_h as usize;
        if pixels.len() < (width as usize).saturating_mul(height as usize) {
            return None;
        }
        let mut out = Vec::with_capacity(crop_w.saturating_mul(crop_h));
        for y in 0..crop_h {
            let start = y.saturating_mul(src_w);
            out.extend_from_slice(&pixels[start..start + crop_w]);
        }
        Some(out)
    }

    #[cfg(windows)]
    fn maybe_dump_uploaded_texture_readback(
        &self,
        stage: u32,
        width: u32,
        height: u32,
        pixels: &[u32],
        format_code: u32,
        source: &'static str,
        byte_len: usize,
        texture: &ID3D11Texture2D,
    ) {
        let doom_present_argb_upload = format_code == 0xFFFF_FFFE
            && self.active_tex_guest_format_code
                == crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8;
        let doom_present_native_upload =
            format_code == crate::xbox::gpu::texture_format::X_D3DFMT_A8R8G8B8;

        if !Self::texture_readback_probe_enabled()
            || stage != 0
            || width != 512
            || height != 256
            || !(doom_present_argb_upload || doom_present_native_upload)
            || self.active_tex_guest_norm != 0x03C0_01A0
            || self.active_tex_data != 0x0400_0000
        {
            return;
        }

        static DID_DUMP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if DID_DUMP.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }

        let Some(device) = &self.device else {
            return;
        };
        let Some(ctx) = &self.ctx else {
            return;
        };

        unsafe {
            let staging_desc = D3D11_TEXTURE2D_DESC {
                Width: width,
                Height: height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                MiscFlags: 0,
            };

            let mut staging: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&staging_desc, None, Some(&mut staging)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TEX-READBACK] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} source={} bytes={} reason=CreateStagingTexture err={}",
                    stage,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    width,
                    height,
                    source,
                    byte_len,
                    e
                ));
                return;
            }
            let Some(staging) = staging else {
                return;
            };

            ctx.CopyResource(&staging, texture);
            ctx.Flush();

            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            if let Err(e) = ctx.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TEX-READBACK] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} {}x{} source={} bytes={} reason=Map err={}",
                    stage,
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data,
                    width,
                    height,
                    source,
                    byte_len,
                    e
                ));
                return;
            }

            let mut readback = vec![0u32; (width as usize).saturating_mul(height as usize)];
            let src = mapped.pData as *const u8;
            let dst = readback.as_mut_ptr() as *mut u8;
            let row_bytes = (width as usize).saturating_mul(4);
            for y in 0..height as usize {
                std::ptr::copy_nonoverlapping(
                    src.add(y.saturating_mul(mapped.RowPitch as usize)),
                    dst.add(y.saturating_mul(row_bytes)),
                    row_bytes,
                );
            }
            ctx.Unmap(&staging, 0);

            let mismatches = pixels
                .iter()
                .zip(readback.iter())
                .filter(|(a, b)| a != b)
                .count();
            let cpu_hash = Self::argb_hash(pixels);
            let gpu_hash = Self::argb_hash(&readback);
            let full_path = format!(
                r"./doom_d3d11_tex_readback_tex{:08X}_src{:08X}_{}x{}.bmp",
                self.active_tex_guest_norm, self.active_tex_data, width, height
            );
            super::graphics_system::write_d3d11_readback_bmp(&full_path, width, height, &readback);

            let crop_320_200_path = format!(
                r"./doom_d3d11_tex_readback_tex{:08X}_src{:08X}_crop320x200.bmp",
                self.active_tex_guest_norm, self.active_tex_data
            );
            let crop_320_200_line =
                if let Some(crop) = Self::crop_argb(&readback, width, height, 320, 200) {
                    super::graphics_system::write_d3d11_readback_bmp(
                        &crop_320_200_path,
                        320,
                        200,
                        &crop,
                    );
                    format!(
                        "crop320x200 hash=0x{:016X} path={}",
                        Self::argb_hash(&crop),
                        crop_320_200_path
                    )
                } else {
                    "crop320x200 unavailable".to_string()
                };

            let crop_320_168_path = format!(
                r"./doom_d3d11_tex_readback_tex{:08X}_src{:08X}_crop320x168.bmp",
                self.active_tex_guest_norm, self.active_tex_data
            );
            let crop_320_168_line =
                if let Some(crop) = Self::crop_argb(&readback, width, height, 320, 168) {
                    super::graphics_system::write_d3d11_readback_bmp(
                        &crop_320_168_path,
                        320,
                        168,
                        &crop,
                    );
                    format!(
                        "crop320x168 hash=0x{:016X} path={}",
                        Self::argb_hash(&crop),
                        crop_320_168_path
                    )
                } else {
                    "crop320x168 unavailable".to_string()
                };

            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-TEX-READBACK] stage={} tex=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} source={} upload_fmt=0x{:08X}/{} {}x{} bytes={} rowpitch={} cpu_hash=0x{:016X} gpu_hash=0x{:016X} mismatches={} full_path={} | {} | {}",
                stage,
                self.active_tex_guest_raw,
                self.active_tex_guest_norm,
                self.active_tex_data,
                self.active_tex_guest_format_code,
                crate::xbox::gpu::texture_format::format_name(self.active_tex_guest_format_code),
                source,
                format_code,
                if format_code == 0xFFFF_FFFE {
                    "ARG32(decoded)"
                } else {
                    crate::xbox::gpu::texture_format::format_name(format_code)
                },
                width,
                height,
                byte_len,
                mapped.RowPitch,
                cpu_hash,
                gpu_hash,
                mismatches,
                full_path,
                crop_320_200_line,
                crop_320_168_line
            ));
        }
    }

    pub(super) fn active_texture_format_name(&self) -> &'static str {
        match self.active_tex_format_code {
            0xFFFF_FFFE => "ARG32(decoded)",
            0 => self.active_tex_source,
            fmt => crate::xbox::gpu::texture_format::format_name(fmt),
        }
    }

    pub(super) fn argb_sample_stats(pixels: &[u32]) -> (u8, u8, usize, usize, usize) {
        if pixels.is_empty() {
            return (0, 0, 0, 0, 0);
        }
        let sample_step = (pixels.len() / 1024).max(1);
        let mut min_alpha = u8::MAX;
        let mut max_alpha = u8::MIN;
        let mut alpha_nonzero = 0usize;
        let mut rgb_nonzero = 0usize;
        let mut samples = 0usize;
        for px in pixels.iter().step_by(sample_step).take(1024) {
            let alpha = (px >> 24) as u8;
            min_alpha = min_alpha.min(alpha);
            max_alpha = max_alpha.max(alpha);
            alpha_nonzero += usize::from((*px & 0xFF00_0000) != 0);
            rgb_nonzero += usize::from((*px & 0x00FF_FFFF) != 0);
            samples += 1;
        }
        if samples == 0 {
            (0, 0, 0, 0, 0)
        } else {
            (min_alpha, max_alpha, alpha_nonzero, rgb_nonzero, samples)
        }
    }

    pub(super) fn active_texture_is_spidey_select_circle(&self) -> bool {
        self.active_tex_guest_raw == 0x83E0_8570 || self.active_tex_guest_norm == 0x03E0_8570
    }

    pub(super) fn active_texture_is_spidey_chis22(&self) -> bool {
        if self.active_tex_guest_raw == 0x83C2_9310 || self.active_tex_guest_norm == 0x03C2_9310 {
            return true;
        }

        let is_dxt_alpha_atlas = matches!(
            self.active_tex_guest_format_code,
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT3
                | crate::xbox::gpu::texture_format::X_D3DFMT_DXT5
        ) && self.active_tex_width == 512
            && self.active_tex_height == 512
            && self.active_tex_sample_count != 0
            && self.active_tex_alpha_nonzero_sample != 0
            && self.active_tex_alpha_nonzero_sample <= 128;

        is_dxt_alpha_atlas
            && (self.active_tex_source == "bc-cpu-decode"
                || self.active_tex_format_code == self.active_tex_guest_format_code)
    }

    #[cfg(windows)]
    pub(super) fn upload_argb_texture(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        pixels: &[u32],
        format_code: u32,
        source: &'static str,
        byte_len: usize,
    ) -> bool {
        let device = match &self.device {
            Some(d) => d,
            None => return false,
        };
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return false,
        };
        if width == 0 || height == 0 || pixels.len() < (width * height) as usize {
            static ZERO_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = ZERO_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TEX-DIAG-FAIL #{}] stage={} source={} fmt=0x{:08X}/{} {}x{} pixels={} bytes={} reason=zero-or-short",
                    n,
                    stage,
                    source,
                    format_code,
                    if format_code == 0xFFFF_FFFE {
                        "ARG32(decoded)"
                    } else {
                        crate::xbox::gpu::texture_format::format_name(format_code)
                    },
                    width,
                    height,
                    pixels.len(),
                    byte_len
                ));
            }
            return false;
        }

        let (tex_alpha_min, tex_alpha_max, tex_alpha_nonzero, tex_rgb_nonzero, tex_sample_count) =
            Self::argb_sample_stats(pixels);

        unsafe {
            let tex_desc = D3D11_TEXTURE2D_DESC {
                Width: width,
                Height: height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };
            let init_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: pixels.as_ptr() as *const _,
                SysMemPitch: width * 4,
                SysMemSlicePitch: 0,
            };
            let mut texture: Option<ID3D11Texture2D> = None;
            if device
                .CreateTexture2D(&tex_desc, Some(&init_data), Some(&mut texture))
                .is_err()
            {
                static TEX_FAIL_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = TEX_FAIL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-TEX-DIAG-FAIL #{}] stage={} source={} fmt=0x{:08X}/{} {}x{} pixels={} bytes={} reason=CreateTexture2D",
                        n,
                        stage,
                        source,
                        format_code,
                        if format_code == 0xFFFF_FFFE {
                            "ARG32(decoded)"
                        } else {
                            crate::xbox::gpu::texture_format::format_name(format_code)
                        },
                        width,
                        height,
                        pixels.len(),
                        byte_len
                    ));
                }
                return false;
            }
            let texture = match texture {
                Some(t) => t,
                None => return false,
            };

            let mut srv: Option<ID3D11ShaderResourceView> = None;
            if device
                .CreateShaderResourceView(&texture, None, Some(&mut srv))
                .is_err()
            {
                static SRV_FAIL_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SRV_FAIL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-TEX-DIAG-FAIL #{}] stage={} source={} fmt=0x{:08X}/{} {}x{} pixels={} bytes={} reason=CreateShaderResourceView",
                        n,
                        stage,
                        source,
                        format_code,
                        if format_code == 0xFFFF_FFFE {
                            "ARG32(decoded)"
                        } else {
                            crate::xbox::gpu::texture_format::format_name(format_code)
                        },
                        width,
                        height,
                        pixels.len(),
                        byte_len
                    ));
                }
                return false;
            }

            if let Some(ref srv) = srv {
                ctx.PSSetShaderResources(stage, Some(&[Some(srv.clone())]));
            }

            self.bind_sampler_for_stage(stage);
            if stage == 0 {
                self.has_active_texture = true;
                self.active_tex_width = width;
                self.active_tex_height = height;
                self.active_tex_handle = self.next_texture_handle();
                self.active_tex_format_code = format_code;
                self.active_tex_source = source;
                self.active_tex_byte_len = byte_len;
                self.active_tex_srv_bound = srv.is_some();
                self.active_tex_alpha_min = tex_alpha_min;
                self.active_tex_alpha_max = tex_alpha_max;
                self.active_tex_alpha_nonzero_sample = tex_alpha_nonzero;
                self.active_tex_rgb_nonzero_sample = tex_rgb_nonzero;
                self.active_tex_sample_count = tex_sample_count;
                self.texture_v_flip = false;
                if super::spidey_peter_texel_probe_enabled() {
                    self.active_tex_probe_pixels.clear();
                    self.active_tex_probe_pixels.extend_from_slice(pixels);
                    self.active_tex_probe_width = width;
                    self.active_tex_probe_height = height;
                } else {
                    self.active_tex_probe_pixels.clear();
                    self.active_tex_probe_width = 0;
                    self.active_tex_probe_height = 0;
                }

                static TEX_DIAG_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = TEX_DIAG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 48 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-TEX-DIAG #{}] tex={} stage={} guest=0x{:08X}->0x{:08X} data=0x{:08X} guest_fmt=0x{:02X}/{} source={} fmt=0x{:08X}/{} {}x{} bytes={} srv_bound={} tex_alpha=[{}..{}] tex_alpha_nonzero={}/{} tex_rgb_nonzero={}/{}",
                        n,
                        self.active_tex_handle,
                        stage,
                        self.active_tex_guest_raw,
                        self.active_tex_guest_norm,
                        self.active_tex_data,
                        self.active_tex_guest_format_code,
                        crate::xbox::gpu::texture_format::format_name(
                            self.active_tex_guest_format_code
                        ),
                        source,
                        format_code,
                        self.active_texture_format_name(),
                        width,
                        height,
                        byte_len,
                        self.active_tex_srv_bound,
                        self.active_tex_alpha_min,
                        self.active_tex_alpha_max,
                        self.active_tex_alpha_nonzero_sample,
                        self.active_tex_sample_count,
                        self.active_tex_rgb_nonzero_sample,
                        self.active_tex_sample_count
                    ));
                }
            }

            self.maybe_dump_uploaded_texture_readback(
                stage,
                width,
                height,
                pixels,
                format_code,
                source,
                byte_len,
                &texture,
            );
        }

        true
    }

    #[cfg(windows)]
    fn sampler_address_mode(value: u32) -> D3D11_TEXTURE_ADDRESS_MODE {
        match value {
            2 => D3D11_TEXTURE_ADDRESS_MIRROR,
            3 => D3D11_TEXTURE_ADDRESS_CLAMP,
            4 => D3D11_TEXTURE_ADDRESS_BORDER,
            5 => D3D11_TEXTURE_ADDRESS_CLAMP,
            _ => D3D11_TEXTURE_ADDRESS_WRAP,
        }
    }

    #[cfg(windows)]
    fn sampler_filter(states: &[u32; 32]) -> D3D11_FILTER {
        let mag = states[super::super::X_D3DTSS_MAGFILTER];
        let min = states[super::super::X_D3DTSS_MINFILTER];
        let mip = states[super::super::X_D3DTSS_MIPFILTER];
        if matches!(mag, 3..=5) || matches!(min, 3..=5) || matches!(mip, 3..=5) {
            return D3D11_FILTER_ANISOTROPIC;
        }

        match (min == 2, mag == 2, mip == 2) {
            (false, false, false) => D3D11_FILTER_MIN_MAG_MIP_POINT,
            (false, false, true) => D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR,
            (false, true, false) => D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
            (false, true, true) => D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR,
            (true, false, false) => D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT,
            (true, false, true) => D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
            (true, true, false) => D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT,
            (true, true, true) => D3D11_FILTER_MIN_MAG_MIP_LINEAR,
        }
    }

    #[cfg(windows)]
    pub(super) fn bind_sampler_for_stage(&mut self, stage: u32) {
        if stage >= 4 {
            return;
        }
        let device = match &self.device {
            Some(d) => d,
            None => return,
        };
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return,
        };
        let states = self.texture_stage_states[stage as usize];
        let anisotropic = matches!(states[super::super::X_D3DTSS_MAGFILTER], 3..=5)
            || matches!(states[super::super::X_D3DTSS_MINFILTER], 3..=5)
            || matches!(states[super::super::X_D3DTSS_MIPFILTER], 3..=5);
        let sampler_desc = D3D11_SAMPLER_DESC {
            Filter: Self::sampler_filter(&states),
            AddressU: Self::sampler_address_mode(states[super::super::X_D3DTSS_ADDRESSU]),
            AddressV: Self::sampler_address_mode(states[super::super::X_D3DTSS_ADDRESSV]),
            AddressW: Self::sampler_address_mode(states[super::super::X_D3DTSS_ADDRESSW]),
            MipLODBias: 0.0,
            MaxAnisotropy: if anisotropic { 4 } else { 1 },
            ComparisonFunc: D3D11_COMPARISON_NEVER,
            BorderColor: [0.0; 4],
            MinLOD: 0.0,
            MaxLOD: f32::MAX,
        };
        let mut sampler: Option<ID3D11SamplerState> = None;
        unsafe {
            if device
                .CreateSamplerState(&sampler_desc, Some(&mut sampler))
                .is_ok()
            {
                if let Some(ref sampler) = sampler {
                    ctx.PSSetSamplers(stage, Some(&[Some(sampler.clone())]));
                }
            }
        }
    }

    #[cfg(windows)]
    fn upload_bc_native_texture(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        format_code: u32,
        bytes: &[u8],
    ) -> bool {
        let block_width = (width + 3) / 4;
        let block_height = (height + 3) / 4;
        let bytes_per_block =
            crate::xbox::gpu::texture_format::block_bytes(format_code).unwrap_or(16);
        let expected_size = (block_width * block_height * bytes_per_block) as usize;
        if bytes.len() < expected_size {
            return false;
        }

        let ctx = match self.ctx.as_ref() {
            Some(c) => c.clone(),
            None => return false,
        };
        let device = match self.device.as_ref() {
            Some(d) => d.clone(),
            None => return false,
        };
        let dxgi_format = match format_code {
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT1 => DXGI_FORMAT_BC1_UNORM,
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT3 => DXGI_FORMAT_BC2_UNORM,
            crate::xbox::gpu::texture_format::X_D3DFMT_DXT5 => DXGI_FORMAT_BC3_UNORM,
            _ => return false,
        };

        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: dxgi_format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let init_data = D3D11_SUBRESOURCE_DATA {
            pSysMem: bytes.as_ptr() as *const _,
            SysMemPitch: block_width * bytes_per_block,
            SysMemSlicePitch: 0,
        };
        let mut tex: Option<ID3D11Texture2D> = None;
        unsafe {
            if device
                .CreateTexture2D(&desc, Some(&init_data), Some(&mut tex))
                .is_err()
            {
                static BC_FAIL_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = BC_FAIL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-BC-FAIL] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} reason=CreateTexture2D",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len()
                    ));
                }
                return false;
            }
        }
        let Some(tex) = tex else { return false };

        let mut srv: Option<ID3D11ShaderResourceView> = None;
        unsafe {
            if device
                .CreateShaderResourceView(&tex, None, Some(&mut srv))
                .is_err()
            {
                return false;
            }
        }
        let Some(srv) = srv else { return false };

        unsafe {
            ctx.PSSetShaderResources(stage, Some(&[Some(srv)]));
        }
        self.bind_sampler_for_stage(stage);
        if stage == 0 {
            self.has_active_texture = true;
            self.active_tex_width = width;
            self.active_tex_height = height;
            self.active_tex_handle = self.next_texture_handle();
            self.active_tex_format_code = format_code;
            self.active_tex_source = "bc-native";
            self.active_tex_byte_len = bytes.len();
            self.active_tex_srv_bound = true;
            self.active_tex_alpha_min = 0;
            self.active_tex_alpha_max = 0;
            self.active_tex_alpha_nonzero_sample = 0;
            self.active_tex_rgb_nonzero_sample = 0;
            self.active_tex_sample_count = 0;
            self.texture_v_flip = false;
            if super::spidey_peter_texel_probe_enabled() {
                if let Some(pixels) =
                    crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                        format_code,
                        width,
                        height,
                        bytes,
                    )
                {
                    self.active_tex_probe_pixels = pixels;
                    self.active_tex_probe_width = width;
                    self.active_tex_probe_height = height;
                } else {
                    self.active_tex_probe_pixels.clear();
                    self.active_tex_probe_width = 0;
                    self.active_tex_probe_height = 0;
                }
            } else {
                self.active_tex_probe_pixels.clear();
                self.active_tex_probe_width = 0;
                self.active_tex_probe_height = 0;
            }
        }

        static BC_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = BC_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 20 {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-BC] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} tex=0x{:08X}->0x{:08X} data=0x{:08X}",
                n,
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                bytes.len(),
                self.active_tex_guest_raw,
                self.active_tex_guest_norm,
                self.active_tex_data
            ));
        }
        if stage == 0 {
            static BC_TEX_DIAG_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = BC_TEX_DIAG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 24 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-TEX-DIAG #{}] tex={} stage={} source=bc-native fmt=0x{:08X}/{} {}x{} bytes={} srv_bound={}",
                    n,
                    self.active_tex_handle,
                    stage,
                    format_code,
                    self.active_texture_format_name(),
                    width,
                    height,
                    bytes.len(),
                    self.active_tex_srv_bound
                ));
            }
        }

        true
    }

    #[cfg(windows)]
    pub(super) fn set_texture_raw(
        &mut self,
        stage: u32,
        width: u32,
        height: u32,
        format_code: u32,
        bytes: &[u8],
    ) {
        let format_code = crate::xbox::gpu::texture_format::format_code(format_code);
        if !crate::xbox::gpu::texture_format::is_block_compressed(format_code) {
            if let Some(pixels) = crate::xbox::gpu::texture_format::decode_linear_to_argb(
                format_code,
                width,
                height,
                bytes,
                None,
            ) {
                let _ = self.upload_argb_texture(
                    stage,
                    width,
                    height,
                    &pixels,
                    format_code,
                    "raw-decode",
                    bytes.len(),
                );
                static RAW_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let n = RAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 20 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-RAW] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} decoded",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len()
                    ));
                }
            } else {
                static UNSUP_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = UNSUP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 20 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-TEX-UNSUPPORTED] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={}",
                        n,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        bytes.len()
                    ));
                }
            }
            return;
        }
        let linear_block_storage = if self.active_tex_swizzled_blocks
            && crate::xbox::gpu::texture_format::is_swizzled_block_compressed(format_code)
        {
            let block_width = (width + 3) / 4;
            let block_height = (height + 3) / 4;
            let bytes_per_block =
                crate::xbox::gpu::texture_format::block_bytes(format_code).unwrap_or(0);
            let expected = (block_width as usize)
                .saturating_mul(block_height as usize)
                .saturating_mul(bytes_per_block as usize);
            if bytes.len() >= expected && expected != 0 {
                match crate::xbox::gpu::swizzle::unswizzle_blocks(
                    bytes,
                    block_width,
                    block_height,
                    bytes_per_block,
                ) {
                    Some(linear_blocks) => {
                        static BC_UNSWIZZLE_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = BC_UNSWIZZLE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 32 || n.is_power_of_two() {
                            crate::xbox::emulator::debug_log(&format!(
                                "[D3D11-BC-UNSWIZZLE] #{} stage={} fmt=0x{:02X}/{} {}x{} blocks={}x{} bpb={} bytes={} tex=0x{:08X}->0x{:08X} data=0x{:08X}",
                                n,
                                stage,
                                format_code,
                                crate::xbox::gpu::texture_format::format_name(format_code),
                                width,
                                height,
                                block_width,
                                block_height,
                                bytes_per_block,
                                expected,
                                self.active_tex_guest_raw,
                                self.active_tex_guest_norm,
                                self.active_tex_data
                            ));
                        }
                        Some(linear_blocks)
                    }
                    None => {
                        static BC_UNSWIZZLE_FAIL_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = BC_UNSWIZZLE_FAIL_LOG
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 32 || n.is_power_of_two() {
                            crate::xbox::emulator::debug_log(&format!(
                                "[D3D11-BC-UNSWIZZLE-FAIL] #{} stage={} fmt=0x{:02X}/{} {}x{} blocks={}x{} bpb={} bytes={}",
                                n,
                                stage,
                                format_code,
                                crate::xbox::gpu::texture_format::format_name(format_code),
                                width,
                                height,
                                block_width,
                                block_height,
                                bytes_per_block,
                                bytes.len()
                            ));
                        }
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };
        let bc_bytes = linear_block_storage.as_deref().unwrap_or(bytes);

        if stage == 0 && width <= 512 && height <= 512 {
            if let Some(mut pixels) =
                crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
                    format_code,
                    width,
                    height,
                    bc_bytes,
                )
            {
                let sample_step = (pixels.len() / 1024).max(1);
                let alpha_sample = pixels
                    .iter()
                    .step_by(sample_step)
                    .take(1024)
                    .filter(|px| (**px & 0xFF00_0000) != 0)
                    .count();
                let rgb_before = pixels
                    .iter()
                    .step_by(sample_step)
                    .take(1024)
                    .filter(|px| (**px & 0x00FF_FFFF) != 0)
                    .count();
                if crate::xbox::gpu::texture_format::promote_alpha_mask_rgb(&mut pixels) {
                    let rgb_after = pixels
                        .iter()
                        .step_by(sample_step)
                        .take(1024)
                        .filter(|px| (**px & 0x00FF_FFFF) != 0)
                        .count();
                    let _ = self.upload_argb_texture(
                        stage,
                        width,
                        height,
                        &pixels,
                        format_code,
                        "bc-alpha-mask",
                        bytes.len(),
                    );
                    static BC_MASK_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = BC_MASK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 32 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-BC-ALPHA-MASK] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} alpha_sample={} rgb_sample={}=>{} tex=0x{:08X}->0x{:08X} data=0x{:08X}",
                            n,
                            stage,
                            format_code,
                            crate::xbox::gpu::texture_format::format_name(format_code),
                            width,
                            height,
                            bytes.len(),
                            alpha_sample,
                            rgb_before,
                            rgb_after,
                            self.active_tex_guest_raw,
                            self.active_tex_guest_norm,
                            self.active_tex_data
                        ));
                    }
                    return;
                }
            }
        }

        if self.upload_bc_native_texture(stage, width, height, format_code, bc_bytes) {
            return;
        }
        if let Some(mut pixels) = crate::xbox::gpu::texture_format::decode_block_compressed_to_argb(
            format_code,
            width,
            height,
            bc_bytes,
        ) {
            let sample_step = (pixels.len() / 1024).max(1);
            let alpha_sample = pixels
                .iter()
                .step_by(sample_step)
                .take(1024)
                .filter(|px| (**px & 0xFF00_0000) != 0)
                .count();
            let rgb_before = pixels
                .iter()
                .step_by(sample_step)
                .take(1024)
                .filter(|px| (**px & 0x00FF_FFFF) != 0)
                .count();
            let explicit_spidey_chis22 = self.active_texture_is_spidey_chis22();
            let promoted = if explicit_spidey_chis22 {
                crate::xbox::gpu::texture_format::promote_alpha_mask_rgb(&mut pixels)
            } else {
                false
            };
            let rgb_after = pixels
                .iter()
                .step_by(sample_step)
                .take(1024)
                .filter(|px| (**px & 0x00FF_FFFF) != 0)
                .count();
            let _ = self.upload_argb_texture(
                stage,
                width,
                height,
                &pixels,
                format_code,
                "bc-cpu-decode",
                bytes.len(),
            );
            if promoted {
                static ALPHA_MASK_PROMOTE_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let pn = ALPHA_MASK_PROMOTE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if pn < 32 || pn.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-ALPHA-MASK-PROMOTE] #{} stage={} fmt=0x{:02X}/{} {}x{} alpha_sample={} rgb_sample={}=>{} tex=0x{:08X}->0x{:08X} data=0x{:08X} explicit_chis22={}",
                        pn,
                        stage,
                        format_code,
                        crate::xbox::gpu::texture_format::format_name(format_code),
                        width,
                        height,
                        alpha_sample,
                        rgb_before,
                        rgb_after,
                        self.active_tex_guest_raw,
                        self.active_tex_guest_norm,
                        self.active_tex_data,
                        explicit_spidey_chis22
                    ));
                }
            }
            static BC_CPU_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = BC_CPU_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 24 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-BC-CPU] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} alpha_sample={} rgb_sample={}=>{} promoted={} promote_scope={} tex=0x{:08X}->0x{:08X} data=0x{:08X}",
                    n,
                    stage,
                    format_code,
                    crate::xbox::gpu::texture_format::format_name(format_code),
                    width,
                    height,
                    bytes.len(),
                    alpha_sample,
                    rgb_before,
                    rgb_after,
                    promoted,
                    if explicit_spidey_chis22 {
                        "chis22"
                    } else if promoted {
                        "alpha-mask"
                    } else {
                        "raw"
                    },
                    self.active_tex_guest_raw,
                    self.active_tex_guest_norm,
                    self.active_tex_data
                ));
            }
            return;
        }
        static BC_FALLBACK_FAIL_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = BC_FALLBACK_FAIL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-BC-FALLBACK-FAIL] #{} stage={} fmt=0x{:02X}/{} {}x{} bytes={} tex=0x{:08X}->0x{:08X} data=0x{:08X}",
                n,
                stage,
                format_code,
                crate::xbox::gpu::texture_format::format_name(format_code),
                width,
                height,
                bytes.len(),
                self.active_tex_guest_raw,
                self.active_tex_guest_norm,
                self.active_tex_data
            ));
        }
    }
}
