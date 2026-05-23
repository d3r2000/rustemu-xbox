//! D3D11 graphics-system responsibilities.
//!
//! This module is the landing zone for Cxbx-R/Xenia-style render-target and
//! presentation ownership. It starts with the readback present cache so the
//! behavior change stays isolated from the larger backend split.

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{
    ID3D11DepthStencilView, ID3D11RenderTargetView, ID3D11ShaderResourceView, ID3D11Texture2D,
    D3D11_BIND_DEPTH_STENCIL, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_BOX,
    D3D11_CLEAR_DEPTH, D3D11_CLEAR_STENCIL, D3D11_CPU_ACCESS_READ, D3D11_MAPPED_SUBRESOURCE,
    D3D11_MAP_READ, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT, D3D11_USAGE_STAGING,
};
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_SAMPLE_DESC,
};

use super::{
    argb_fnv1a64, d3d11_frame_prof_enabled, d3d11_log_frame_prof, d3d11_prof_add_us,
    spidey_rt_diag_key, spidey_rt_trace_enabled, D3D11Backend, D3D11_HLE_RT_COMMIT,
    D3D11_HLE_RT_DATA, D3D11_HLE_RT_FORMAT, D3D11_HLE_RT_KEY, D3D11_HLE_RT_PITCH,
    D3D11_HLE_RT_SIZE, D3D11_PROF_READBACK_CALLS, D3D11_PROF_READBACK_COPY_US,
    D3D11_PROF_READBACK_CPU_US, D3D11_PROF_READBACK_MAP_US, D3D11_PROF_READBACK_TOTAL_US,
};
use crate::xbox::gpu::GpuBackend;

#[cfg(windows)]
const DEFAULT_BACKBUFFER_RT_KEY: u32 = 0xFFFF_FF00;

#[cfg(windows)]
#[derive(Clone, Copy, Default)]
pub(super) struct PendingRenderTarget {
    pub(super) key: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) format: u32,
    pub(super) data: u32,
    pub(super) pitch: u32,
}

#[cfg(windows)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NormalizedRenderTarget {
    key: u32,
    width: u32,
    height: u32,
    format: u32,
    data: u32,
    pitch: u32,
}

#[cfg(windows)]
fn normalize_pending_render_target(
    pending: PendingRenderTarget,
    host_width: i32,
    host_height: i32,
) -> Option<NormalizedRenderTarget> {
    let key = pending.key;
    if key == 0 {
        return None;
    }

    let width = pending.width.clamp(1, host_width.max(1) as u32).max(1);
    let height = pending.height.clamp(1, host_height.max(1) as u32).max(1);
    let data = if pending.data != 0 {
        pending.data & !0x3
    } else {
        key & !0x3
    };
    let pitch = if pending.pitch != 0 {
        pending.pitch
    } else {
        width.saturating_mul(4)
    };

    Some(NormalizedRenderTarget {
        key,
        width,
        height,
        format: pending.format,
        data,
        pitch,
    })
}

#[cfg(windows)]
pub(super) struct D3D11RenderTargetSurface {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) format: u32,
    pub(super) data: u32,
    pub(super) pitch: u32,
    pub(super) tex: ID3D11Texture2D,
    pub(super) srv: ID3D11ShaderResourceView,
    pub(super) rtv: ID3D11RenderTargetView,
    pub(super) ds_tex: ID3D11Texture2D,
    pub(super) dsv: ID3D11DepthStencilView,
    pub(super) staging_tex: ID3D11Texture2D,
}

impl D3D11Backend {
    #[cfg(windows)]
    pub(super) fn cached_render_target_resources(
        &self,
        key: u32,
    ) -> Option<(
        ID3D11Texture2D,
        ID3D11RenderTargetView,
        ID3D11DepthStencilView,
        ID3D11Texture2D,
        u32,
    )> {
        let surface = self.rt_cache.get(&key)?;
        Some((
            surface.tex.clone(),
            surface.rtv.clone(),
            surface.dsv.clone(),
            surface.staging_tex.clone(),
            key,
        ))
    }

    #[cfg(windows)]
    pub(super) fn active_render_target_resources(
        &self,
    ) -> Option<(
        ID3D11Texture2D,
        ID3D11RenderTargetView,
        ID3D11DepthStencilView,
        ID3D11Texture2D,
        u32,
    )> {
        let logical_backbuffer_key = self.backbuffer_rt_info.key;
        if self.active_rt_key != 0 {
            if let Some(resources) = self.cached_render_target_resources(self.active_rt_key) {
                return Some(resources);
            }
            if self.active_rt_key == logical_backbuffer_key {
                if let Some(resources) = self.cached_render_target_resources(logical_backbuffer_key)
                {
                    return Some(resources);
                }
                return Some((
                    self.rt_tex.as_ref()?.clone(),
                    self.rtv.as_ref()?.clone(),
                    self.dsv.as_ref()?.clone(),
                    self.staging_tex.as_ref()?.clone(),
                    logical_backbuffer_key,
                ));
            }
        }

        // Prefer the cached surface for the Xbox logical backbuffer. Spider-Man
        // creates an explicit render target for key 0x01231000; treating the
        // default D3D11 texture as that key makes Swap copy stale black even
        // while mid-frame readback shows the menu art in the cached RT.
        if let Some(resources) = self.cached_render_target_resources(logical_backbuffer_key) {
            return Some(resources);
        }

        // Fallback for simple demos that really do render into the default RT.
        Some((
            self.rt_tex.as_ref()?.clone(),
            self.rtv.as_ref()?.clone(),
            self.dsv.as_ref()?.clone(),
            self.staging_tex.as_ref()?.clone(),
            logical_backbuffer_key,
        ))
    }

    #[cfg(windows)]
    pub(super) fn backbuffer_render_target_resources(
        &self,
    ) -> Option<(
        ID3D11Texture2D,
        ID3D11RenderTargetView,
        ID3D11DepthStencilView,
        ID3D11Texture2D,
        u32,
    )> {
        let logical_backbuffer_key = self.backbuffer_rt_info.key;
        if let Some(resources) = self.cached_render_target_resources(logical_backbuffer_key) {
            return Some(resources);
        }
        Some((
            self.rt_tex.as_ref()?.clone(),
            self.rtv.as_ref()?.clone(),
            self.dsv.as_ref()?.clone(),
            self.staging_tex.as_ref()?.clone(),
            logical_backbuffer_key,
        ))
    }

    #[cfg(windows)]
    pub(super) fn scanout_render_target_resources(
        &self,
        pcrtc_start: u32,
    ) -> Option<(
        ID3D11Texture2D,
        ID3D11RenderTargetView,
        ID3D11DepthStencilView,
        ID3D11Texture2D,
        u32,
    )> {
        let scanout = pcrtc_start & !0x3;
        if scanout == 0 {
            return None;
        }

        let mut best: Option<(u32, u64)> = None;
        for (key, surface) in &self.rt_cache {
            if *key == 0 {
                continue;
            }
            let base = Self::render_target_alias_base(*key, surface);
            let len = Self::render_target_alias_len(surface);
            if base == 0 || len == 0 {
                continue;
            }

            let start = base as u64;
            let end = start.saturating_add(len);
            let scanout64 = scanout as u64;
            let distance = if scanout64 >= start && scanout64 < end {
                0
            } else if scanout64 < start && start - scanout64 <= 0x1000 {
                start - scanout64
            } else {
                continue;
            };

            match best {
                Some((_best_key, best_distance)) if best_distance <= distance => {}
                _ => best = Some((*key, distance)),
            }
        }

        let (key, distance) = best?;
        let surface = self.rt_cache.get(&key)?;
        static SCANOUT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = SCANOUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-SCANOUT-RT] #{} pcrtc_start=0x{:08X} -> key=0x{:08X} data=0x{:08X} {}x{} pitch={} distance=0x{:X} active_rt=0x{:08X}",
                n,
                pcrtc_start,
                key,
                surface.data,
                surface.width,
                surface.height,
                surface.pitch,
                distance,
                self.active_rt_key
            ));
        }
        Some((
            surface.tex.clone(),
            surface.rtv.clone(),
            surface.dsv.clone(),
            surface.staging_tex.clone(),
            key,
        ))
    }

    #[cfg(windows)]
    pub(super) fn ensure_scanout_render_target(
        &mut self,
        key: u32,
        width: u32,
        height: u32,
    ) -> bool {
        let key = key & !0x3;
        if key == 0 {
            return false;
        }
        let width = width.clamp(1, self.width.max(1) as u32).max(1);
        let height = height.clamp(1, self.height.max(1) as u32).max(1);

        let needs_create = self
            .rt_cache
            .get(&key)
            .map(|surface| surface.width != width || surface.height != height)
            .unwrap_or(true);
        if !needs_create {
            return true;
        }

        if self.rt_cache.len() >= 64 && !self.rt_cache.contains_key(&key) {
            self.rt_cache.clear();
        }
        let Some(surface) =
            self.create_render_target_surface(key, width, height, 0x12, key, width * 4)
        else {
            return false;
        };
        self.rt_cache.insert(key, surface);
        static RT_SCANOUT_CACHE_WRITE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let write_n = RT_SCANOUT_CACHE_WRITE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if write_n < 32 || write_n.is_power_of_two() || spidey_rt_diag_key(key) {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RT-CACHE-WRITE] #{} source=scanout key=0x{:08X} data=0x{:08X} {}x{} fmt=0x00000012 pitch={} cache={} active=0x{:08X} backbuffer=0x{:08X}",
                write_n,
                key,
                key,
                width,
                height,
                width * 4,
                self.rt_cache.len(),
                self.active_rt_key,
                self.backbuffer_rt_key
            ));
        }

        static SCANOUT_CREATE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = SCANOUT_CREATE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-SCANOUT-CREATE] #{} key=0x{:08X} {}x{} pitch={}",
                n,
                key,
                width,
                height,
                width * 4
            ));
        }
        true
    }

    #[cfg(windows)]
    pub(super) fn readback_resources(
        &mut self,
        rt: ID3D11Texture2D,
        stag: ID3D11Texture2D,
        active_rt_key: u32,
        pcrtc_start: u32,
        source: &'static str,
    ) -> &[u32] {
        let Some(ctx) = &self.ctx else {
            return &self.readback_buf;
        };
        let frame_prof = d3d11_frame_prof_enabled();
        let readback_start = if frame_prof {
            D3D11_PROF_READBACK_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(std::time::Instant::now())
        } else {
            None
        };

        unsafe {
            let out_w = self.width.max(0) as usize;
            let out_h = self.height.max(0) as usize;
            if out_w == 0 || out_h == 0 {
                return &self.readback_buf;
            }

            let copy_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            ctx.CopyResource(&stag, &rt);
            if let Some(start) = copy_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_COPY_US, start);
            }

            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            let map_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };
            if ctx
                .Map(&stag, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
                .is_err()
            {
                return &self.readback_buf;
            }
            if let Some(start) = map_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_MAP_US, start);
            }
            let cpu_start = if frame_prof {
                Some(std::time::Instant::now())
            } else {
                None
            };

            let needed = out_w.saturating_mul(out_h);
            if self.readback_buf.len() != needed {
                self.readback_buf.resize(needed, 0xFF00_0000);
            }
            self.readback_buf.fill(0xFF00_0000);

            let mut rt_desc = D3D11_TEXTURE2D_DESC::default();
            rt.GetDesc(&mut rt_desc);
            let rt_w = rt_desc.Width as usize;
            let rt_h = rt_desc.Height as usize;
            let copy_w = rt_w.min(out_w);
            let copy_h = rt_h.min(out_h);

            static PRESENT_READBACK_LOG_N: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let log_n = PRESENT_READBACK_LOG_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if log_n < 16 || log_n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PRESENT-READBACK] #{} source={} rt={}x{} out={}x{} key=0x{:08X} pcrtc_start=0x{:08X} copy={}x{} rowpitch={}",
                    log_n,
                    source,
                    rt_w,
                    rt_h,
                    out_w,
                    out_h,
                    active_rt_key,
                    pcrtc_start,
                    copy_w,
                    copy_h,
                    mapped.RowPitch
                ));
            }

            let src = mapped.pData as *const u8;
            let dst = self.readback_buf.as_mut_ptr() as *mut u8;
            let dst_pitch = out_w * 4;
            let row_bytes = copy_w * 4;
            if row_bytes > 0 && copy_h > 0 {
                for y in 0..copy_h {
                    std::ptr::copy_nonoverlapping(
                        src.add(y * mapped.RowPitch as usize),
                        dst.add(y * dst_pitch),
                        row_bytes,
                    );
                }
            }
            ctx.Unmap(&stag, 0);

            let (rgb_nonblack, alpha_nonzero, sampled, _, _, _) =
                sample_pixel_stats(&self.readback_buf);
            self.present_cache.reconcile_after_readback(
                &mut self.readback_buf,
                self.black_clear_without_draw_pending,
                self.draws_since_clear,
                source,
                active_rt_key,
                rgb_nonblack,
                alpha_nonzero,
                sampled,
            );
            self.apply_peter_candidate_overlay(out_w, out_h);
            let (rgb_nonblack, alpha_nonzero, sampled, first, mid, last) =
                sample_pixel_stats(&self.readback_buf);
            if log_n < 16 || log_n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PRESENT-STATS] #{} source={} key=0x{:08X} rgb_nonblack={}/{} alpha_nonzero={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X}",
                    log_n,
                    source,
                    active_rt_key,
                    rgb_nonblack,
                    sampled,
                    alpha_nonzero,
                    sampled,
                    first,
                    mid,
                    last
                ));
            }

            maybe_record_present_frame(source, out_w as u32, out_h as u32, &self.readback_buf);
            if let Some(start) = cpu_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_CPU_US, start);
            }
            if let Some(start) = readback_start {
                d3d11_prof_add_us(&D3D11_PROF_READBACK_TOTAL_US, start);
                d3d11_log_frame_prof(source, active_rt_key, out_w, out_h);
            }
        }

        self.pump_debug_messages();
        &self.readback_buf
    }

    #[cfg(windows)]
    pub(super) fn bind_active_render_target(&mut self) {
        let Some(ctx) = self.ctx.clone() else {
            return;
        };
        let Some((_tex, rtv, dsv, _staging, key)) = self.active_render_target_resources() else {
            return;
        };
        if let Some((tex_key, tex_base, tex_len, rt_base, rt_len)) =
            self.active_texture_conflicts_with_render_target(key)
        {
            unsafe {
                ctx.PSSetShaderResources(0, Some(&[None]));
            }
            self.has_active_texture = false;
            self.texture_v_flip = false;
            self.active_tex_width = 0;
            self.active_tex_height = 0;
            self.active_tex_handle = 0;
            self.active_tex_format_code = 0;
            self.active_tex_source = "rt-srv-unbound";
            self.active_tex_byte_len = 0;
            self.active_tex_srv_bound = false;
            self.active_tex_guest_raw = 0;
            self.active_tex_guest_norm = 0;
            self.active_tex_data = 0;
            self.active_tex_guest_format_code = 0;
            self.active_tex_swizzled_blocks = false;
            self.active_tex_alpha_min = 0;
            self.active_tex_alpha_max = 0;
            self.active_tex_alpha_nonzero_sample = 0;
            self.active_tex_rgb_nonzero_sample = 0;
            self.active_tex_sample_count = 0;

            static RT_SRV_UNBIND_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RT_SRV_UNBIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() || spidey_rt_diag_key(key) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-SRV-UNBIND] #{} action=unbind-stage0 rt=0x{:08X} rt_alias=[0x{:08X}..0x{:08X}) tex_key=0x{:08X} tex_alias=[0x{:08X}..0x{:08X})",
                    n,
                    key,
                    rt_base,
                    (rt_base as u64).saturating_add(rt_len) as u32,
                    tex_key,
                    tex_base,
                    (tex_base as u64).saturating_add(tex_len) as u32
                ));
            }
        }
        unsafe {
            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), &dsv);
        }
    }

    #[cfg(windows)]
    fn active_texture_conflicts_with_render_target(
        &self,
        rt_key: u32,
    ) -> Option<(u32, u32, u64, u32, u64)> {
        if !self.has_active_texture
            || !self.active_tex_srv_bound
            || !self.active_tex_source.starts_with("render-target")
        {
            return None;
        }
        let rt_surface = self.rt_cache.get(&rt_key)?;
        let rt_base = Self::render_target_alias_base(rt_key, rt_surface);
        let rt_len = Self::render_target_alias_len(rt_surface);
        if rt_base == 0 || rt_len == 0 {
            return None;
        }

        let tex_key = self.active_tex_guest_norm & !0x3;
        if let Some(tex_surface) = self.rt_cache.get(&tex_key) {
            let tex_base = Self::render_target_alias_base(tex_key, tex_surface);
            let tex_len = Self::render_target_alias_len(tex_surface);
            if Self::render_target_ranges_overlap(rt_base, rt_len, tex_base, tex_len) {
                return Some((tex_key, tex_base, tex_len, rt_base, rt_len));
            }
        }

        let tex_base = (if self.active_tex_data != 0 {
            self.active_tex_data
        } else {
            tex_key
        }) & !0x3;
        let tex_len = self.active_tex_byte_len.max(
            (self.active_tex_width as usize)
                .saturating_mul(self.active_tex_height as usize)
                .saturating_mul(4),
        ) as u64;
        if Self::render_target_ranges_overlap(rt_base, rt_len, tex_base, tex_len) {
            return Some((tex_key, tex_base, tex_len, rt_base, rt_len));
        }
        None
    }

    #[cfg(windows)]
    fn render_target_ranges_overlap(a_base: u32, a_len: u64, b_base: u32, b_len: u64) -> bool {
        if a_base == 0 || b_base == 0 || a_len == 0 || b_len == 0 {
            return false;
        }
        let a0 = a_base as u64;
        let b0 = b_base as u64;
        let a1 = a0.saturating_add(a_len);
        let b1 = b0.saturating_add(b_len);
        a0 < b1 && b0 < a1
    }

    #[cfg(windows)]
    pub(super) fn create_render_target_surface(
        &self,
        key: u32,
        width: u32,
        height: u32,
        format: u32,
        data: u32,
        pitch: u32,
    ) -> Option<D3D11RenderTargetSurface> {
        let Some(device) = &self.device else {
            return None;
        };
        let width = width.clamp(1, 4096);
        let height = height.clamp(1, 4096);

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
                BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };
            let mut tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&tex_desc, None, Some(&mut tex)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateTexture2D {}",
                    key, width, height, format, data, e
                ));
                return None;
            }
            let tex = tex?;

            let mut rtv: Option<ID3D11RenderTargetView> = None;
            if let Err(e) = device.CreateRenderTargetView(&tex, None, Some(&mut rtv)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateRenderTargetView {}",
                    key, width, height, format, data, e
                ));
                return None;
            }
            let rtv = rtv?;

            let mut srv: Option<ID3D11ShaderResourceView> = None;
            if let Err(e) = device.CreateShaderResourceView(&tex, None, Some(&mut srv)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateShaderResourceView {}",
                    key, width, height, format, data, e
                ));
                return None;
            }
            let srv = srv?;

            let ds_desc = D3D11_TEXTURE2D_DESC {
                Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
                BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32,
                ..tex_desc
            };
            let mut ds_tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&ds_desc, None, Some(&mut ds_tex)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateDepthTexture {}",
                    key, width, height, format, data, e
                ));
                return None;
            }
            let ds_tex = ds_tex?;

            let mut dsv: Option<ID3D11DepthStencilView> = None;
            if let Err(e) = device.CreateDepthStencilView(&ds_tex, None, Some(&mut dsv)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateDepthStencilView {}",
                    key, width, height, format, data, e
                ));
                return None;
            }
            let dsv = dsv?;

            if let Some(ctx) = self.ctx.as_ref() {
                ctx.ClearRenderTargetView(&rtv, &[0.0, 0.0, 0.0, 0.0]);
                ctx.ClearDepthStencilView(
                    &dsv,
                    (D3D11_CLEAR_DEPTH.0 | D3D11_CLEAR_STENCIL.0) as u32,
                    1.0,
                    0,
                );
                static INIT_CLEAR_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = INIT_CLEAR_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-RT-INIT-CLEAR] #{} key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} pitch={}",
                        n, key, width, height, format, data, pitch
                    ));
                }
            }

            let staging_desc = D3D11_TEXTURE2D_DESC {
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                ..tex_desc
            };
            let mut staging_tex: Option<ID3D11Texture2D> = None;
            if let Err(e) = device.CreateTexture2D(&staging_desc, None, Some(&mut staging_tex)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-FAIL] key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} reason=CreateStagingTexture {}",
                    key, width, height, format, data, e
                ));
                return None;
            }

            Some(D3D11RenderTargetSurface {
                width,
                height,
                format,
                data,
                pitch,
                tex,
                srv,
                rtv,
                ds_tex,
                dsv,
                staging_tex: staging_tex?,
            })
        }
    }

    #[cfg(windows)]
    pub(super) fn commit_hle_render_target(&mut self) {
        let key = self.pending_rt.key;
        if key == 0 {
            static NULL_RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = NULL_RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT] retain color target on NULL key active=0x{:08X}",
                    self.active_rt_key
                ));
            }
            self.bind_active_render_target();
            return;
        }

        let Some(normalized) =
            normalize_pending_render_target(self.pending_rt, self.width, self.height)
        else {
            return;
        };
        let width = normalized.width;
        let height = normalized.height;
        let format = normalized.format;
        let data = normalized.data;
        let pitch = normalized.pitch;
        let expected_width = self.width.max(1) as u32;
        let expected_height = self.height.max(1) as u32;
        let replaces_synthetic_backbuffer = self.backbuffer_rt_key == DEFAULT_BACKBUFFER_RT_KEY
            && key != DEFAULT_BACKBUFFER_RT_KEY
            && width == expected_width
            && height == expected_height;

        if self.backbuffer_rt_key == 0 || replaces_synthetic_backbuffer {
            let old_key = self.backbuffer_rt_key;
            self.backbuffer_rt_key = key;
            if replaces_synthetic_backbuffer {
                static ADOPT_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = ADOPT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 16 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-BACKBUFFER-ADOPT] #{} old=0x{:08X} new=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                        n, old_key, key, width, height, data, pitch
                    ));
                }
            }
        }
        if key == self.backbuffer_rt_key {
            self.backbuffer_rt_info = PendingRenderTarget {
                key,
                width,
                height,
                format,
                data,
                pitch,
            };

            let scanout = (if data != 0 { data } else { key }) & !0x3;
            let old_scanout = crate::xbox::aot::nv2a::shadow_read(0x60_0800);
            if width == expected_width && height == expected_height {
                if scanout != 0 && old_scanout != scanout {
                    crate::xbox::aot::nv2a::shadow_write(0x60_0800, scanout);
                    static SCANOUT_LINK_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = SCANOUT_LINK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if env_flag("RUSTEMU_PCRTC_DIAG") || n < 16 || n.is_power_of_two() {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-SCANOUT-LINK] #{} pcrtc_start 0x{:08X}->0x{:08X} backbuffer_key=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={}",
                            n, old_scanout, scanout, key, data, width, height, format, pitch
                        ));
                    }
                } else {
                    static SCANOUT_LINK_SKIP_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n =
                        SCANOUT_LINK_SKIP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if env_flag("RUSTEMU_PCRTC_DIAG") || n < 16 || n.is_power_of_two() {
                        let reason = if scanout == 0 {
                            "zero-scanout"
                        } else {
                            "already-linked"
                        };
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D11-SCANOUT-LINK-SKIP] #{} reason={} pcrtc_start=0x{:08X} scanout=0x{:08X} backbuffer_key=0x{:08X} data=0x{:08X} {}x{} expected={}x{} fmt=0x{:08X} pitch={}",
                            n,
                            reason,
                            old_scanout,
                            scanout,
                            key,
                            data,
                            width,
                            height,
                            expected_width,
                            expected_height,
                            format,
                            pitch
                        ));
                    }
                }
            } else {
                static SCANOUT_LINK_SKIP_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SCANOUT_LINK_SKIP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if env_flag("RUSTEMU_PCRTC_DIAG") || n < 16 || n.is_power_of_two() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D11-SCANOUT-LINK-SKIP] #{} reason=size pcrtc_start=0x{:08X} scanout=0x{:08X} backbuffer_key=0x{:08X} data=0x{:08X} {}x{} expected={}x{} fmt=0x{:08X} pitch={}",
                        n,
                        old_scanout,
                        scanout,
                        key,
                        data,
                        width,
                        height,
                        expected_width,
                        expected_height,
                        format,
                        pitch
                    ));
                }
            }
        }

        let needs_create = self
            .rt_cache
            .get(&key)
            .map(|surface| {
                surface.width != width || surface.height != height || surface.format != format
            })
            .unwrap_or(true);
        if needs_create {
            if self.rt_cache.len() >= 64 {
                self.rt_cache.clear();
            }
            let Some(surface) =
                self.create_render_target_surface(key, width, height, format, data, pitch)
            else {
                return;
            };
            self.rt_cache.insert(key, surface);
            static RT_CACHE_WRITE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RT_CACHE_WRITE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let rt_trace = spidey_rt_trace_enabled();
            if n < 64 || n.is_power_of_two() || (rt_trace && spidey_rt_diag_key(key)) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-CACHE-WRITE] #{} source=commit key=0x{:08X} data=0x{:08X} {}x{} fmt=0x{:08X} pitch={} cache={} active=0x{:08X} backbuffer=0x{:08X}",
                    n,
                    key,
                    data,
                    width,
                    height,
                    format,
                    pitch,
                    self.rt_cache.len(),
                    self.active_rt_key,
                    self.backbuffer_rt_key
                ));
            }
        }

        self.active_rt_key = key;
        self.bind_active_render_target();
        static RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let rt_trace = spidey_rt_trace_enabled();
        if n < 32 || n.is_power_of_two() || (rt_trace && spidey_rt_diag_key(key)) {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RT] bind key=0x{:08X} {}x{} fmt=0x{:08X} data=0x{:08X} pitch={} cache={} created={} backbuffer=0x{:08X} active=0x{:08X}",
                key,
                width,
                height,
                format,
                data,
                pitch,
                self.rt_cache.len(),
                needs_create,
                self.backbuffer_rt_key,
                self.active_rt_key
            ));
        }
    }

    #[cfg(windows)]
    pub(super) fn bind_default_render_target_impl(&mut self, reason: &'static str) {
        if self.backbuffer_rt_key == 0 {
            self.backbuffer_rt_key = DEFAULT_BACKBUFFER_RT_KEY;
        }

        if self.backbuffer_rt_info.key == 0 {
            let width = self.width.max(1) as u32;
            let height = self.height.max(1) as u32;
            self.backbuffer_rt_info = PendingRenderTarget {
                key: self.backbuffer_rt_key,
                width,
                height,
                format: crate::xbox::gpu::texture_format::X_D3DFMT_LIN_A8R8G8B8,
                data: 0,
                pitch: width.saturating_mul(4),
            };
        }

        self.active_rt_key = self.backbuffer_rt_key;
        self.bind_active_render_target();

        static DEFAULT_RT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DEFAULT_RT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-DEFAULT-RT] #{} reason={} active=0x{:08X} backbuffer=0x{:08X} {}x{} data=0x{:08X} pitch={}",
                n,
                reason,
                self.active_rt_key,
                self.backbuffer_rt_info.key,
                self.backbuffer_rt_info.width,
                self.backbuffer_rt_info.height,
                self.backbuffer_rt_info.data,
                self.backbuffer_rt_info.pitch
            ));
        }
    }

    #[cfg(windows)]
    pub(super) fn handle_hle_render_target_control(&mut self, state: u32, value: u32) -> bool {
        match state {
            D3D11_HLE_RT_KEY => self.pending_rt.key = value,
            D3D11_HLE_RT_SIZE => {
                self.pending_rt.width = value & 0xFFFF;
                self.pending_rt.height = (value >> 16) & 0xFFFF;
            }
            D3D11_HLE_RT_FORMAT => self.pending_rt.format = value,
            D3D11_HLE_RT_DATA => self.pending_rt.data = value,
            D3D11_HLE_RT_PITCH => self.pending_rt.pitch = value,
            D3D11_HLE_RT_COMMIT => self.commit_hle_render_target(),
            _ => return false,
        }
        true
    }

    #[cfg(windows)]
    pub(super) fn bind_render_target_texture_impl(
        &mut self,
        stage: u32,
        key: u32,
        source: &'static str,
        tex_identity: u32,
        data_identity: u32,
    ) -> bool {
        if stage >= 4 || key == 0 {
            return false;
        }
        if key == self.active_rt_key {
            static RT_SELF_BIND_IMPL_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RT_SELF_BIND_IMPL_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() || spidey_rt_diag_key(key) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-TEX-HAZARD] #{} action=reject-active-rt-impl stage={} key=0x{:08X} active_rt=0x{:08X} source={}",
                    n, stage, key, self.active_rt_key, source
                ));
            }
            return false;
        }
        let Some(ctx) = self.ctx.clone() else {
            return false;
        };
        let Some(surface) = self.rt_cache.get(&key) else {
            return false;
        };
        let srv = surface.srv.clone();
        let width = surface.width;
        let height = surface.height;
        let format = surface.format;
        let pitch = surface.pitch.max(width.saturating_mul(4)).max(4);
        let byte_len = pitch.saturating_mul(height) as usize;
        let alias_base = Self::render_target_alias_base(key, surface);
        let alias_len = Self::render_target_alias_len(surface);
        let active_alias = self.rt_cache.get(&self.active_rt_key).map(|active| {
            (
                Self::render_target_alias_base(self.active_rt_key, active),
                Self::render_target_alias_len(active),
            )
        });
        let active_alias_overlap = active_alias
            .map(|(base, len)| Self::render_target_ranges_overlap(alias_base, alias_len, base, len))
            .unwrap_or(false);
        self.maybe_capture_spidey_rt_sample_readback(
            &ctx,
            surface,
            stage,
            key,
            source,
            tex_identity,
            data_identity,
            active_alias_overlap,
        );
        if active_alias_overlap {
            static RT_ALIAS_HAZARD_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = RT_ALIAS_HAZARD_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 64 || n.is_power_of_two() || spidey_rt_diag_key(key) {
                let (active_base, active_len) = active_alias.unwrap_or((0, 0));
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RT-TEX-HAZARD] #{} action=bind-alias-overlap stage={} key=0x{:08X} alias=[0x{:08X}..0x{:08X}) active_rt=0x{:08X} active_alias=[0x{:08X}..0x{:08X}) source={}",
                    n,
                    stage,
                    key,
                    alias_base,
                    (alias_base as u64).saturating_add(alias_len) as u32,
                    self.active_rt_key,
                    active_base,
                    (active_base as u64).saturating_add(active_len) as u32,
                    source
                ));
            }
        }
        unsafe {
            ctx.PSSetShaderResources(stage, Some(&[Some(srv)]));
        }
        self.bind_sampler_for_stage(stage);
        if stage == 0 {
            self.has_active_texture = true;
            self.active_tex_width = width;
            self.active_tex_height = height;
            self.active_tex_handle = self.next_texture_handle();
            self.active_tex_format_code = format;
            self.active_tex_source = source;
            self.active_tex_byte_len = byte_len;
            self.active_tex_srv_bound = true;
            self.active_tex_guest_raw = tex_identity;
            self.active_tex_guest_norm = key;
            self.active_tex_data = data_identity;
            self.active_tex_guest_format_code = format;
            self.active_tex_alpha_min = 0;
            self.active_tex_alpha_max = 0;
            self.active_tex_alpha_nonzero_sample = 0;
            self.active_tex_rgb_nonzero_sample = 0;
            self.active_tex_sample_count = 0;
            self.texture_v_flip = false;
        }

        static RT_TEX_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = RT_TEX_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let rt_trace = spidey_rt_trace_enabled();
        if n < 32
            || n.is_power_of_two()
            || source == "render-target-alias"
            || (rt_trace && spidey_rt_diag_key(key))
        {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RT-TEX] #{} stage={} key=0x{:08X} source={} tex=0x{:08X} data=0x{:08X} alias=[0x{:08X}..0x{:08X}) {}x{} fmt=0x{:08X} pitch={} active_rt=0x{:08X} active_alias_overlap={} active_tex_source={} stage0={}",
                n,
                stage,
                key,
                source,
                tex_identity,
                data_identity,
                alias_base,
                (alias_base as u64).saturating_add(alias_len) as u32,
                width,
                height,
                format,
                pitch,
                self.active_rt_key,
                active_alias_overlap,
                self.active_tex_source,
                stage == 0
            ));
        }
        true
    }

    #[cfg(windows)]
    fn maybe_capture_spidey_rt_sample_readback(
        &self,
        ctx: &windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
        surface: &D3D11RenderTargetSurface,
        stage: u32,
        key: u32,
        source: &'static str,
        tex_identity: u32,
        data_identity: u32,
        active_alias_overlap: bool,
    ) {
        if !spidey_rt_sample_readback_enabled() || stage != 0 {
            return;
        }
        let keys = spidey_rt_sample_readback_keys();
        if !keys.is_empty() && !keys.contains(&(key & !0x3)) && !keys.contains(&key) {
            return;
        }

        static SPIDEY_RT_SAMPLE_READBACK_N: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let seq = SPIDEY_RT_SAMPLE_READBACK_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let limit = spidey_rt_sample_readback_limit();
        if seq >= limit && !seq.is_power_of_two() {
            return;
        }

        unsafe {
            ctx.CopyResource(&surface.staging_tex, &surface.tex);
            ctx.Flush();

            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            if let Err(err) = ctx.Map(
                &surface.staging_tex,
                0,
                D3D11_MAP_READ,
                0,
                Some(&mut mapped),
            ) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-SPIDEY-RT-SAMPLE-READBACK] #{} stage={} key=0x{:08X} source={} reason=map-failed err={}",
                    seq, stage, key, source, err
                ));
                return;
            }

            let width = surface.width as usize;
            let height = surface.height as usize;
            let mut pixels = vec![0xFF00_0000u32; width.saturating_mul(height)];
            let src = mapped.pData as *const u8;
            let dst = pixels.as_mut_ptr() as *mut u8;
            let row_bytes = width.saturating_mul(4);
            for y in 0..height {
                std::ptr::copy_nonoverlapping(
                    src.add(y.saturating_mul(mapped.RowPitch as usize)),
                    dst.add(y.saturating_mul(row_bytes)),
                    row_bytes,
                );
            }
            ctx.Unmap(&surface.staging_tex, 0);

            let full_rgb_nonblack = pixels.iter().filter(|px| (**px & 0x00FF_FFFF) != 0).count();
            let full_alpha_nonzero = pixels.iter().filter(|px| (**px & 0xFF00_0000) != 0).count();
            let (sample_rgb, sample_alpha, sample_count, first, mid, last) =
                sample_pixel_stats(&pixels);
            let path = format!(
                r"./spidey_rt_sample_{:05}_stage{}_key{:08X}_{}x{}.bmp",
                seq, stage, key, surface.width, surface.height
            );
            write_d3d11_readback_bmp(&path, surface.width, surface.height, &pixels);
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-SPIDEY-RT-SAMPLE-READBACK] #{} stage={} key=0x{:08X} source={} tex=0x{:08X} data=0x{:08X} size={}x{} fmt=0x{:08X} pitch={} rowpitch={} active_rt=0x{:08X} active_alias_overlap={} full_hash=0x{:016X} full_rgb_nonblack={}/{} full_alpha_nonzero={}/{} sample_rgb={}/{} sample_alpha={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X} path={}",
                seq,
                stage,
                key,
                source,
                tex_identity,
                data_identity,
                surface.width,
                surface.height,
                surface.format,
                surface.pitch,
                mapped.RowPitch,
                self.active_rt_key,
                active_alias_overlap,
                argb_fnv1a64(&pixels),
                full_rgb_nonblack,
                pixels.len(),
                full_alpha_nonzero,
                pixels.len(),
                sample_rgb,
                sample_count,
                sample_alpha,
                sample_count,
                first,
                mid,
                last,
                path
            ));
        }
    }

    #[cfg(windows)]
    pub(super) fn resolve_scanout_impl(&mut self, pcrtc_start: u32) -> bool {
        let scanout_key = pcrtc_start & !0x3;
        if scanout_key == 0 {
            return false;
        }
        let Some(ctx) = self.ctx.as_ref().cloned() else {
            return false;
        };
        let (src_rt, _src_rtv, _src_dsv, _src_stag, src_key, src_kind) =
            if let Some((rt, rtv, dsv, stag, key)) = self.backbuffer_render_target_resources() {
                (rt, rtv, dsv, stag, key, "backbuffer")
            } else if let Some((rt, rtv, dsv, stag, key)) = self.active_render_target_resources() {
                (rt, rtv, dsv, stag, key, "active")
            } else {
                return false;
            };
        if src_key == scanout_key {
            return true;
        }

        let mut src_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe {
            src_rt.GetDesc(&mut src_desc);
        }
        if src_desc.Width == 0 || src_desc.Height == 0 {
            return false;
        }
        if !self.ensure_scanout_render_target(scanout_key, src_desc.Width, src_desc.Height) {
            return false;
        }

        let Some((dst_rt, dst_w, dst_h)) = self
            .rt_cache
            .get(&scanout_key)
            .map(|surface| (surface.tex.clone(), surface.width, surface.height))
        else {
            return false;
        };

        unsafe {
            if dst_w == src_desc.Width && dst_h == src_desc.Height {
                ctx.CopyResource(&dst_rt, &src_rt);
            } else {
                let copy_w = dst_w.min(src_desc.Width);
                let copy_h = dst_h.min(src_desc.Height);
                let src_box = D3D11_BOX {
                    left: 0,
                    top: 0,
                    front: 0,
                    right: copy_w,
                    bottom: copy_h,
                    back: 1,
                };
                ctx.CopySubresourceRegion(&dst_rt, 0, 0, 0, 0, &src_rt, 0, Some(&src_box));
            }
            ctx.Flush();
        }

        static RESOLVE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = RESOLVE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 32 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-SWAP-RESOLVE] #{} src_kind={} src=0x{:08X} {}x{} -> scanout=0x{:08X} {}x{} pcrtc=0x{:08X}",
                n,
                src_kind,
                src_key,
                src_desc.Width,
                src_desc.Height,
                scanout_key,
                dst_w,
                dst_h,
                pcrtc_start
            ));
        }
        true
    }

    #[cfg(windows)]
    pub(super) fn bind_display_as_render_target_impl(&mut self, pcrtc_start: u32) -> bool {
        let scanout_key = pcrtc_start & !0x3;
        if scanout_key == 0 {
            return false;
        }

        let Some((src_rt, _src_rtv, _src_dsv, _src_stag, _src_key)) = self
            .backbuffer_render_target_resources()
            .or_else(|| self.active_render_target_resources())
        else {
            return false;
        };
        let mut src_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe {
            src_rt.GetDesc(&mut src_desc);
        }
        if src_desc.Width == 0 || src_desc.Height == 0 {
            return false;
        }
        if !self.ensure_scanout_render_target(scanout_key, src_desc.Width, src_desc.Height) {
            return false;
        }

        self.active_rt_key = scanout_key;
        self.bind_active_render_target();
        static FRONT_BIND_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = FRONT_BIND_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 16 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-FRONT-BIND] #{} pcrtc=0x{:08X} active_rt=0x{:08X} {}x{}",
                n, pcrtc_start, scanout_key, src_desc.Width, src_desc.Height
            ));
        }
        true
    }

    #[cfg(windows)]
    pub(super) fn restore_backbuffer_render_target_impl(&mut self) {
        if self.active_rt_key != 0 {
            static FRONT_RESTORE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = FRONT_RESTORE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-FRONT-RESTORE] #{} old_active_rt=0x{:08X}",
                    n, self.active_rt_key
                ));
            }
        }
        self.bind_default_render_target_impl("restore");
    }

    #[cfg(windows)]
    pub(super) fn present_display_impl(&mut self, pcrtc_start: u32) -> &[u32] {
        if let Some((rt, _rtv, _dsv, stag, key)) = self.scanout_render_target_resources(pcrtc_start)
        {
            self.readback_resources(rt, stag, key, pcrtc_start, "scanout")
        } else {
            self.present_framebuffer()
        }
    }

    #[cfg(windows)]
    pub(super) fn present_framebuffer_impl(&mut self) -> &[u32] {
        let force_active = std::env::var("RUSTEMU_PRESENT_ACTIVE")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false);
        if force_active {
            return self.readback_framebuffer();
        }

        let pcrtc_start = crate::xbox::aot::nv2a::shadow_read(0x60_0800);
        if let Some((rt, _rtv, _dsv, stag, key)) = self.scanout_render_target_resources(pcrtc_start)
        {
            self.readback_resources(rt, stag, key, pcrtc_start, "scanout")
        } else {
            self.readback_framebuffer()
        }
    }

    #[cfg(windows)]
    pub(super) fn render_target_alias_base(key: u32, surface: &D3D11RenderTargetSurface) -> u32 {
        if surface.data != 0 {
            surface.data & !0x3
        } else {
            key & !0x3
        }
    }

    #[cfg(windows)]
    pub(super) fn render_target_alias_len(surface: &D3D11RenderTargetSurface) -> u64 {
        let pitch = surface.pitch.max(surface.width.saturating_mul(4)).max(4);
        (pitch as u64).saturating_mul(surface.height.max(1) as u64)
    }

    #[cfg(windows)]
    pub(super) fn find_render_target_alias_by_data(
        &self,
        data_addr: u32,
    ) -> Option<(u32, u32, u64)> {
        let addr = data_addr & !0x3;
        if addr == 0 {
            return None;
        }

        let mut best: Option<(u32, u32, u64)> = None;
        for (key, surface) in &self.rt_cache {
            if *key == 0 || *key == self.active_rt_key {
                continue;
            }
            let base = Self::render_target_alias_base(*key, surface);
            let len = Self::render_target_alias_len(surface);
            if base == 0 || len == 0 {
                continue;
            }
            // A D3D texture's data pointer can live in the same physical heap
            // as scanout/front-buffer surfaces without being a render target.
            // Spider-Man stores ordinary DXT menu tiles at 0x03C00420 etc.;
            // treating any address inside the scanout allocation as an RT
            // alias makes those tiles sample stale framebuffer pixels, which
            // appears as repeated vertical strips. Only alias when the guest
            // pointer names the render-target base itself.
            if addr == base {
                match best {
                    Some((_best_key, _best_base, best_len)) if best_len <= len => {}
                    _ => best = Some((*key, base, len)),
                }
            }
        }
        best
    }

    pub(super) fn debug_active_render_target(&self) -> Option<(u32, u32, u32, u32, u32)> {
        #[cfg(windows)]
        {
            if self.active_rt_key != 0 {
                if let Some(surface) = self.rt_cache.get(&self.active_rt_key) {
                    return Some((
                        self.active_rt_key,
                        surface.width,
                        surface.height,
                        surface.data,
                        surface.pitch,
                    ));
                }
            }

            if self.backbuffer_rt_info.key != 0 {
                Some((
                    self.backbuffer_rt_info.key,
                    self.backbuffer_rt_info.width,
                    self.backbuffer_rt_info.height,
                    self.backbuffer_rt_info.data,
                    self.backbuffer_rt_info.pitch,
                ))
            } else {
                Some((
                    0,
                    self.width.max(0) as u32,
                    self.height.max(0) as u32,
                    0,
                    self.width.max(0) as u32 * 4,
                ))
            }
        }

        #[cfg(not(windows))]
        {
            None
        }
    }
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
        })
        .unwrap_or(false)
}

#[cfg(windows)]
fn parse_u32_env_list(name: &str) -> Vec<u32> {
    std::env::var(name)
        .ok()
        .map(|raw| {
            raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
                .filter_map(|part| {
                    let part = part.trim();
                    if part.is_empty() {
                        return None;
                    }
                    if let Some(hex) = part.strip_prefix("0x").or_else(|| part.strip_prefix("0X")) {
                        u32::from_str_radix(hex, 16).ok()
                    } else {
                        part.parse::<u32>().ok()
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(windows)]
fn spidey_rt_sample_readback_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_D3D11_RT_SAMPLE_READBACK")
        || env_flag("RUSTEMU_SPIDEY_D3D11_RT_READBACK")
}

#[cfg(windows)]
fn spidey_rt_sample_readback_keys() -> &'static Vec<u32> {
    static KEYS: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
    KEYS.get_or_init(|| {
        let mut keys = parse_u32_env_list("RUSTEMU_SPIDEY_D3D11_RT_READBACK_KEYS");
        if keys.is_empty() {
            keys.push(0x03C0_0150);
        }
        keys
    })
}

#[cfg(windows)]
fn spidey_rt_sample_readback_limit() -> u32 {
    static LIMIT: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *LIMIT.get_or_init(|| {
        std::env::var("RUSTEMU_SPIDEY_D3D11_RT_READBACK_LIMIT")
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok())
            .unwrap_or(8)
            .max(1)
    })
}

pub(super) fn sample_pixel_stats(pixels: &[u32]) -> (usize, usize, usize, u32, u32, u32) {
    if pixels.is_empty() {
        return (0, 0, 0, 0, 0, 0);
    }

    let max_samples = pixels.len().min(4096);
    let step = (pixels.len() / max_samples).max(1);
    let mut sampled = 0usize;
    let mut rgb_nonblack = 0usize;
    let mut alpha_nonzero = 0usize;

    for p in pixels.iter().step_by(step).take(max_samples) {
        sampled += 1;
        rgb_nonblack += usize::from((p & 0x00FF_FFFF) != 0);
        alpha_nonzero += usize::from((p & 0xFF00_0000) != 0);
    }

    (
        rgb_nonblack,
        alpha_nonzero,
        sampled,
        pixels[0],
        pixels[pixels.len() / 2],
        pixels[pixels.len() - 1],
    )
}

pub(super) fn write_d3d11_readback_bmp(path: &str, width: u32, height: u32, pixels: &[u32]) {
    let row_stride = ((width * 3 + 3) / 4) * 4;
    let image_size = row_stride * height;
    let file_size = 54 + image_size;
    let mut bmp = Vec::with_capacity(file_size as usize);

    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(file_size as u32).to_le_bytes());
    bmp.extend_from_slice(&[0u8; 4]);
    bmp.extend_from_slice(&54u32.to_le_bytes());
    bmp.extend_from_slice(&40u32.to_le_bytes());
    bmp.extend_from_slice(&(width as i32).to_le_bytes());
    bmp.extend_from_slice(&(height as i32).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());
    bmp.extend_from_slice(&24u16.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&(image_size as u32).to_le_bytes());
    bmp.extend_from_slice(&[0u8; 16]);

    for y in (0..height as usize).rev() {
        let row_start = y * width as usize;
        for x in 0..width as usize {
            let p = pixels.get(row_start + x).copied().unwrap_or(0);
            bmp.push((p & 0xFF) as u8);
            bmp.push(((p >> 8) & 0xFF) as u8);
            bmp.push(((p >> 16) & 0xFF) as u8);
        }
        while (bmp.len() - 54) % row_stride as usize != 0 {
            bmp.push(0);
        }
    }

    if let Err(e) = std::fs::write(path, bmp) {
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-BMP-WRITE] failed to write {}: {}",
            path, e
        ));
    }
}

#[cfg(windows)]
struct ReadbackRecordConfig {
    dir: String,
    every: u32,
    limit: u32,
}

#[cfg(windows)]
pub(super) fn maybe_record_present_frame(source: &str, width: u32, height: u32, pixels: &[u32]) {
    static CFG: std::sync::OnceLock<Option<ReadbackRecordConfig>> = std::sync::OnceLock::new();
    static FRAME_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let Some(cfg) = CFG
        .get_or_init(|| {
            if !env_flag("RUSTEMU_RECORD_READBACK_FRAMES") {
                return None;
            }
            let dir = std::env::var("RUSTEMU_RECORD_READBACK_DIR").unwrap_or_else(|_| {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                format!(r"./recordings\readback_{}", stamp)
            });
            let every = std::env::var("RUSTEMU_RECORD_READBACK_EVERY")
                .ok()
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(1)
                .max(1);
            let limit = std::env::var("RUSTEMU_RECORD_READBACK_LIMIT")
                .ok()
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(300);
            if let Err(e) = std::fs::create_dir_all(&dir) {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-RECORD-CONFIG] disabled failed_to_create dir={} err={}",
                    dir, e
                ));
                return None;
            }
            crate::xbox::emulator::debug_log(&format!(
                "[D3D11-RECORD-CONFIG] dir={} every={} limit={}",
                dir, every, limit
            ));
            Some(ReadbackRecordConfig { dir, every, limit })
        })
        .as_ref()
    else {
        return;
    };

    let n = FRAME_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n % cfg.every != 0 {
        return;
    }
    let frame = n / cfg.every;
    if frame >= cfg.limit {
        return;
    }

    let path = std::path::Path::new(&cfg.dir).join(format!("frame_{:05}.bmp", frame));
    write_d3d11_readback_bmp(&path.to_string_lossy(), width, height, pixels);
    if frame < 8 || frame.is_power_of_two() {
        let (rgb_nonblack, alpha_nonzero, sampled, first, mid, last) = sample_pixel_stats(pixels);
        crate::xbox::emulator::debug_log(&format!(
            "[D3D11-RECORD-FRAME] frame={} source={} path={} rgb_nonblack={}/{} alpha_nonzero={}/{} first=0x{:08X} mid=0x{:08X} last=0x{:08X}",
            frame,
            source,
            path.display(),
            rgb_nonblack,
            sampled,
            alpha_nonzero,
            sampled,
            first,
            mid,
            last
        ));
    }
}

pub(super) fn sampled_argb_stats(pixels: &[u32]) -> (usize, usize, usize, u32, u32) {
    let sample_step = (pixels.len() / 1024).max(1);
    let mut nonzero = 0usize;
    let mut alpha = 0usize;
    let mut rgb = 0usize;
    for px in pixels.iter().step_by(sample_step).take(1024) {
        if *px != 0 {
            nonzero += 1;
        }
        if (*px & 0xFF00_0000) != 0 {
            alpha += 1;
        }
        if (*px & 0x00FF_FFFF) != 0 {
            rgb += 1;
        }
    }
    let first = pixels.first().copied().unwrap_or(0);
    let mid = pixels
        .get(pixels.len().saturating_div(2))
        .copied()
        .unwrap_or(0);
    (nonzero, alpha, rgb, first, mid)
}

#[derive(Default)]
pub(super) struct PresentReadbackCache {
    buf: Vec<u32>,
    valid: bool,
    rt_key: u32,
    rgb_sample: usize,
    alpha_sample: usize,
}

fn composite_sparse_readback_over_cache(readback_buf: &mut [u32], cached: &[u32]) -> usize {
    let mut overlaid = 0usize;
    for (dst, &base) in readback_buf.iter_mut().zip(cached.iter()) {
        let src = *dst;
        let src_a = (src >> 24) & 0xFF;
        let src_rgb = src & 0x00FF_FFFF;
        if src_a == 0 && src_rgb == 0 {
            *dst = base;
            continue;
        }

        overlaid += 1;
        if src_a == 0xFF {
            continue;
        }

        let inv_a = 255 - src_a;
        let sr = (src >> 16) & 0xFF;
        let sg = (src >> 8) & 0xFF;
        let sb = src & 0xFF;
        let br = (base >> 16) & 0xFF;
        let bg = (base >> 8) & 0xFF;
        let bb = base & 0xFF;
        let r = (sr * src_a + br * inv_a + 127) / 255;
        let g = (sg * src_a + bg * inv_a + 127) / 255;
        let b = (sb * src_a + bb * inv_a + 127) / 255;
        *dst = 0xFF00_0000 | (r << 16) | (g << 8) | b;
    }
    overlaid
}

impl PresentReadbackCache {
    pub(super) fn clear(&mut self) {
        self.buf.clear();
        self.valid = false;
        self.rt_key = 0;
        self.rgb_sample = 0;
        self.alpha_sample = 0;
    }

    pub(super) fn reconcile_after_readback(
        &mut self,
        readback_buf: &mut Vec<u32>,
        black_clear_without_draw_pending: bool,
        draws_since_clear: u32,
        source: &'static str,
        rt_key: u32,
        rgb_nonblack: usize,
        alpha_nonzero: usize,
        sampled: usize,
    ) {
        if sampled == 0 || env_flag("RUSTEMU_PRESENT_CACHE_OFF") {
            return;
        }

        let useful_threshold = (sampled / 64).max(8);
        let useful_frame = rgb_nonblack >= useful_threshold && alpha_nonzero > 0;
        let same_cached_rt =
            self.valid && self.rt_key == rt_key && self.buf.len() == readback_buf.len();
        let sparse_regression = same_cached_rt
            && self.rgb_sample >= useful_threshold.saturating_mul(4)
            && rgb_nonblack > 0
            && rgb_nonblack.saturating_mul(4) < self.rgb_sample
            && alpha_nonzero <= self.alpha_sample;
        if sparse_regression && !env_flag("RUSTEMU_PRESENT_CACHE_SPARSE_OFF") {
            let overlaid = composite_sparse_readback_over_cache(readback_buf, &self.buf);
            static PRESENT_CACHE_SPARSE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PRESENT_CACHE_SPARSE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PRESENT-CACHE-COMPOSITE] #{} source={} rt=0x{:08X} cached_rt=0x{:08X} cached_rgb={}/{} cached_alpha={}/{} reason=sparse-regression current_rgb={}/{} current_alpha={}/{} overlaid={}",
                    n,
                    source,
                    rt_key,
                    self.rt_key,
                    self.rgb_sample,
                    sampled,
                    self.alpha_sample,
                    sampled,
                    rgb_nonblack,
                    sampled,
                    alpha_nonzero,
                    sampled,
                    overlaid
                ));
            }
            return;
        }
        if useful_frame {
            if self.buf.len() != readback_buf.len() {
                self.buf.resize(readback_buf.len(), 0xFF00_0000);
            }
            self.buf.copy_from_slice(readback_buf);
            self.valid = true;
            self.rt_key = rt_key;
            self.rgb_sample = rgb_nonblack;
            self.alpha_sample = alpha_nonzero;

            static PRESENT_CACHE_SAVE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PRESENT_CACHE_SAVE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 16 || n.is_power_of_two() {
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PRESENT-CACHE-SAVE] #{} source={} rt=0x{:08X} rgb_nonblack={}/{} alpha_nonzero={}/{}",
                    n, source, rt_key, rgb_nonblack, sampled, alpha_nonzero, sampled
                ));
            }
            return;
        }

        let empty_frame = rgb_nonblack == 0;
        let black_sync_clear = black_clear_without_draw_pending && draws_since_clear == 0;
        if self.valid && self.buf.len() == readback_buf.len() && (empty_frame || black_sync_clear) {
            readback_buf.copy_from_slice(&self.buf);
            static PRESENT_CACHE_USE_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = PRESENT_CACHE_USE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 || n.is_power_of_two() {
                let reason = if empty_frame {
                    "empty-readback"
                } else {
                    "black-sync-clear"
                };
                crate::xbox::emulator::debug_log(&format!(
                    "[D3D11-PRESENT-CACHE-USE] #{} source={} rt=0x{:08X} cached_rt=0x{:08X} cached_rgb={}/{} cached_alpha={}/{} reason={}",
                    n,
                    source,
                    rt_key,
                    self.rt_key,
                    self.rgb_sample,
                    sampled,
                    self.alpha_sample,
                    sampled,
                    reason
                ));
            }
        }
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    fn pending_rt(
        key: u32,
        width: u32,
        height: u32,
        format: u32,
        data: u32,
        pitch: u32,
    ) -> PendingRenderTarget {
        PendingRenderTarget {
            key,
            width,
            height,
            format,
            data,
            pitch,
        }
    }

    #[test]
    fn d3d11_rt_normalization_rejects_zero_key() {
        assert_eq!(
            normalize_pending_render_target(pending_rt(0, 640, 480, 0x12, 0, 0), 640, 480),
            None
        );
    }

    #[test]
    fn d3d11_rt_normalization_clamps_guest_size_to_host_viewport() {
        let rt = normalize_pending_render_target(
            pending_rt(0x0123_1000, 2048, 0, 0x12, 0x03C0_0150, 0),
            640,
            480,
        )
        .unwrap();

        assert_eq!(rt.width, 640);
        assert_eq!(rt.height, 1);
        assert_eq!(rt.pitch, 640 * 4);
    }

    #[test]
    fn d3d11_rt_normalization_aligns_guest_data_pointer() {
        let rt = normalize_pending_render_target(
            pending_rt(0x0123_1000, 256, 254, 0x12, 0x03C0_0153, 0),
            640,
            480,
        )
        .unwrap();

        assert_eq!(rt.key, 0x0123_1000);
        assert_eq!(rt.data, 0x03C0_0150);
        assert_eq!(rt.pitch, 256 * 4);
    }

    #[test]
    fn d3d11_rt_normalization_uses_key_as_data_when_guest_data_is_zero() {
        let rt = normalize_pending_render_target(
            pending_rt(0x0123_1003, 320, 240, 0x12, 0, 0x900),
            640,
            480,
        )
        .unwrap();

        assert_eq!(rt.key, 0x0123_1003);
        assert_eq!(rt.data, 0x0123_1000);
        assert_eq!(rt.pitch, 0x900);
    }
}
