/// NV2A GPU state — shadow registers + PRAMIN bootstrap.
/// Port of C++ AOT_NV2A_Bootstrap.cpp + NV2A_State.h.
///
/// The Xbox kernel initializes the NV2A GPU before launching any game.
/// Without this setup, games (or their statically-linked D3D8 library) that
/// probe GPU state during early init may spin forever or take wrong paths.
///
/// Shadow registers are served by VEH on MMIO reads (0xFD000000+).
/// PRAMIN (0xFD700000+) is PAGE_READWRITE — written directly to guest RAM.
use crate::xbox::emulator::debug_log;
use std::sync::Mutex;

// ============================================================================
// NV2A register block offsets (absolute from 0xFD000000)
// ============================================================================

const PMC_BASE: u32 = 0x000000;
const PBUS_BASE: u32 = 0x001000;
const PFIFO_BASE: u32 = 0x002000;
const PVIDEO_BASE: u32 = 0x008000;
const PTIMER_BASE: u32 = 0x009000;
const PFB_BASE: u32 = 0x100000;
const PGRAPH_BASE: u32 = 0x400000;
const PCRTC_BASE: u32 = 0x600000;
const USER_BASE: u32 = 0x800000;

// ============================================================================
// Shadow register arrays
// ============================================================================

pub struct Nv2aState {
    pub pmc: [u32; 512],      // 0x000000 - 0x0007FF
    pub pbus: [u32; 512],     // 0x001000 - 0x0017FF
    pub pfifo: [u32; 2048],   // 0x002000 - 0x003FFF
    pub pvideo: [u32; 1024],  // 0x008000 - 0x008FFF
    pub ptimer: [u32; 256],   // 0x009000 - 0x0093FF
    pub pfb: [u32; 1024],     // 0x100000 - 0x100FFF
    pub pgraph: [u32; 8192],  // 0x400000 - 0x407FFF
    pub pcrtc: [u32; 1024],   // 0x600000 - 0x600FFF
    pub pramin: [u32; 65536], // 0x700000 - 0x73FFFF (PRAMIN instance memory)
    pub user: [u32; 512],     // 0x800000 - 0x8007FF
    pub pb_put: u32,
    pub pb_get: u32,
}

impl Nv2aState {
    fn new() -> Self {
        Self {
            pmc: [0; 512],
            pbus: [0; 512],
            pfifo: [0; 2048],
            pvideo: [0; 1024],
            ptimer: [0; 256],
            pfb: [0; 1024],
            pgraph: [0; 8192],
            pcrtc: [0; 1024],
            pramin: [0; 65536],
            user: [0; 512],
            pb_put: 0,
            pb_get: 0,
        }
    }

    /// Read a shadow register by absolute NV2A offset (relative to 0xFD000000).
    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            // PMC
            0x000000..=0x0007FF => {
                let idx = (offset - PMC_BASE) / 4;
                self.pmc.get(idx as usize).copied().unwrap_or(0)
            }
            // PBUS
            0x001000..=0x0017FF => {
                let idx = (offset - PBUS_BASE) / 4;
                self.pbus.get(idx as usize).copied().unwrap_or(0)
            }
            // PFIFO
            0x002000..=0x003FFF => {
                let idx = (offset - PFIFO_BASE) / 4;
                self.pfifo.get(idx as usize).copied().unwrap_or(0)
            }
            // PVIDEO
            0x008000..=0x008FFF => {
                let idx = (offset - PVIDEO_BASE) / 4;
                self.pvideo.get(idx as usize).copied().unwrap_or(0)
            }
            // PTIMER
            0x009000..=0x0093FF => {
                let idx = (offset - PTIMER_BASE) / 4;
                self.ptimer.get(idx as usize).copied().unwrap_or(0)
            }
            // PFB
            0x100000..=0x100FFF => {
                let idx = (offset - PFB_BASE) / 4;
                self.pfb.get(idx as usize).copied().unwrap_or(0)
            }
            // PGRAPH
            0x400000..=0x407FFF => {
                let idx = (offset - PGRAPH_BASE) / 4;
                self.pgraph.get(idx as usize).copied().unwrap_or(0)
            }
            // PCRTC
            0x600000..=0x600FFF => {
                let idx = (offset - PCRTC_BASE) / 4;
                self.pcrtc.get(idx as usize).copied().unwrap_or(0)
            }
            // PRAMIN (instance memory — also accessible via guest RAM at 0xFD700000+)
            0x700000..=0x73FFFF => {
                let idx = (offset - 0x700000) / 4;
                self.pramin.get(idx as usize).copied().unwrap_or(0)
            }
            // USER channel
            0x800000..=0x8007FF => {
                let idx = (offset - USER_BASE) / 4;
                self.user.get(idx as usize).copied().unwrap_or(0)
            }
            _ => 0,
        }
    }

    /// Write a shadow register by absolute NV2A offset.
    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x000000..=0x0007FF => {
                let idx = (offset - PMC_BASE) / 4;
                if let Some(r) = self.pmc.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x001000..=0x0017FF => {
                let idx = (offset - PBUS_BASE) / 4;
                if let Some(r) = self.pbus.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x002000..=0x003FFF => {
                let idx = (offset - PFIFO_BASE) / 4;
                if let Some(r) = self.pfifo.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x008000..=0x008FFF => {
                let idx = (offset - PVIDEO_BASE) / 4;
                if let Some(r) = self.pvideo.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x009000..=0x0093FF => {
                let idx = (offset - PTIMER_BASE) / 4;
                if let Some(r) = self.ptimer.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x100000..=0x100FFF => {
                let idx = (offset - PFB_BASE) / 4;
                if let Some(r) = self.pfb.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x400000..=0x407FFF => {
                let idx = (offset - PGRAPH_BASE) / 4;
                if let Some(r) = self.pgraph.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x600000..=0x600FFF => {
                let idx = (offset - PCRTC_BASE) / 4;
                if let Some(r) = self.pcrtc.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x700000..=0x73FFFF => {
                let idx = (offset - 0x700000) / 4;
                if let Some(r) = self.pramin.get_mut(idx as usize) {
                    *r = value;
                }
            }
            0x800000..=0x8007FF => {
                let idx = (offset - USER_BASE) / 4;
                if let Some(r) = self.user.get_mut(idx as usize) {
                    *r = value;
                }
            }
            _ => {}
        }
    }

    // Shadow write helpers (match C++ w_pfifo, w_pgraph, etc.)
    fn w_pfifo(&mut self, abs: u32, val: u32) {
        let idx = (abs - 0x2000) / 4;
        if let Some(r) = self.pfifo.get_mut(idx as usize) {
            *r = val;
        }
    }
    fn w_pvideo(&mut self, abs: u32, val: u32) {
        let idx = (abs - 0x8000) / 4;
        if let Some(r) = self.pvideo.get_mut(idx as usize) {
            *r = val;
        }
    }
    fn w_ptimer(&mut self, abs: u32, val: u32) {
        let idx = (abs - 0x9000) / 4;
        if let Some(r) = self.ptimer.get_mut(idx as usize) {
            *r = val;
        }
    }
    fn w_pfb(&mut self, abs: u32, val: u32) {
        let idx = (abs - 0x100000) / 4;
        if let Some(r) = self.pfb.get_mut(idx as usize) {
            *r = val;
        }
    }
    fn w_pgraph(&mut self, abs: u32, val: u32) {
        let idx = (abs - 0x400000) / 4;
        if let Some(r) = self.pgraph.get_mut(idx as usize) {
            *r = val;
        }
    }
}

// ============================================================================
// Global NV2A state (accessed from VEH handler)
// ============================================================================

static NV2A_STATE: Mutex<Option<Box<Nv2aState>>> = Mutex::new(None);

/// Install NV2A state for VEH access.
pub fn install_state(state: Box<Nv2aState>) {
    *NV2A_STATE.lock().unwrap() = Some(state);
}

/// Read a shadow register from VEH context.
/// Returns the shadow value, or 0 if state not initialized.
pub fn shadow_read(offset: u32) -> u32 {
    if let Ok(guard) = NV2A_STATE.lock() {
        if let Some(ref state) = *guard {
            return state.read(offset);
        }
    }
    0
}

/// Write a shadow register from VEH context.
pub fn shadow_write(offset: u32, value: u32) {
    let pcrtc_start_old = if offset == 0x60_0800 {
        Some(shadow_read(offset))
    } else {
        None
    };

    // Log non-PFIFO NV2A writes (PGRAPH, PCRTC, PFB — framebuffer setup)
    // Skip PFIFO (0x2000-0x3FFF) — too noisy (DMA_GET polling)
    if offset >= 0x100000 || (offset >= 0x8000 && offset < 0x100000) || offset >= 0x400000 {
        static NV2A_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = NV2A_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 100 {
            let block = if offset >= 0x600000 && offset < 0x700000 {
                "PCRTC"
            } else if offset >= 0x400000 && offset < 0x500000 {
                "PGRAPH"
            } else if offset >= 0x100000 && offset < 0x200000 {
                "PFB"
            } else if offset >= 0x800000 {
                "USER"
            } else {
                "OTHER"
            };
            crate::xbox::emulator::debug_log(&format!(
                "[NV2A-REG] #{} {} off=0x{:06X} val=0x{:08X}",
                n, block, offset, value
            ));
        }
    }

    if let Some(old) = pcrtc_start_old {
        static PCRTC_START_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = PCRTC_START_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let verbose = std::env::var("RUSTEMU_PCRTC_DIAG")
            .map(|v| {
                let v = v.trim();
                v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false);
        if verbose || n < 32 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[NV2A-PCRTC-START-WRITE] #{} old=0x{:08X} new=0x{:08X} masked=0x{:08X}",
                n,
                old,
                value,
                value & 0x07FF_FFFF
            ));
        }
    }

    if let Ok(mut guard) = NV2A_STATE.lock() {
        if let Some(ref mut state) = *guard {
            state.write(offset, value);
        }
    }
}

/// Remove NV2A state (called on unload).
pub fn remove_state() {
    *NV2A_STATE.lock().unwrap() = None;
}

// ============================================================================
// PRAMIN helpers
// ============================================================================

struct PraminCtx {
    free_inst: u32,
}

impl PraminCtx {
    fn alloc(&mut self, blocks: u32) -> u32 {
        let inst = self.free_inst;
        self.free_inst += blocks;
        inst
    }
}

fn pramin_w32(state: &mut Nv2aState, byte_offset: u32, val: u32) {
    let idx = byte_offset / 4;
    if let Some(r) = state.pramin.get_mut(idx as usize) {
        *r = val;
    }
}

fn pramin_dma_ctx(
    state: &mut Nv2aState,
    inst: u32,
    dma_class: u32,
    base_addr: u32,
    limit: u32,
    sysmem: bool,
) {
    let off = inst << 4;
    let mut flags = dma_class | 0x0000_3000 | 0x0000_8000;
    if sysmem {
        flags |= 0x0002_0000;
    }
    pramin_w32(state, off + 0x00, flags);
    pramin_w32(state, off + 0x04, limit);
    pramin_w32(state, off + 0x08, base_addr | 3);
    pramin_w32(state, off + 0x0C, base_addr | 3);
}

// ============================================================================
// RAMHT hash + insert (matching C++ AOT_NV2A_Bootstrap.cpp)
// ============================================================================

fn ramht_ctz32(v: u32) -> u32 {
    if v == 0 {
        return 32;
    }
    v.trailing_zeros()
}

fn ramht_hash(handle: u32, channel_id: u32, ramht_size: u32) -> u32 {
    if ramht_size < 8 {
        return 0;
    }
    let mut bits = ramht_ctz32(ramht_size);
    if bits == 0 || bits == 32 {
        return 0;
    }
    bits -= 1;
    if bits < 5 {
        bits = 5;
    }
    if bits > 31 {
        bits = 31;
    }

    let mask = if bits >= 31 {
        0x7FFF_FFFF
    } else {
        (1u32 << bits) - 1
    };
    let mut hash = 0u32;
    let mut h = handle;
    while h != 0 {
        hash ^= h & mask;
        h >>= bits;
    }
    hash ^= (channel_id & 0x1F) << (bits - 4);
    let entry_count = ramht_size / 8;
    if entry_count != 0 {
        hash %= entry_count;
    }
    hash
}

fn ramht_insert(
    state: &mut Nv2aState,
    ramht_base: u32,
    ramht_size: u32,
    channel_id: u32,
    handle: u32,
    inst: u32,
) {
    if ramht_size < 8 || ramht_base >= 0x40000 {
        return;
    }
    if (ramht_base + ramht_size) > 0x40000 {
        return;
    }

    let entry_count = ramht_size / 8;
    if entry_count == 0 {
        return;
    }

    let slot = ramht_hash(handle, channel_id, ramht_size);
    let entry_ctx = 0x8000_0000 | ((channel_id & 0x1F) << 24) | (inst & 0xFFFF);

    for probe in 0..entry_count {
        let idx = (slot + probe) % entry_count;
        let entry_off = ramht_base + idx * 8;

        let prev_h = state
            .pramin
            .get((entry_off / 4) as usize)
            .copied()
            .unwrap_or(0);
        let prev_d = state
            .pramin
            .get((entry_off / 4 + 1) as usize)
            .copied()
            .unwrap_or(0);

        let slot_empty = prev_h == 0 && prev_d == 0;
        if !slot_empty && prev_h != handle {
            continue;
        }

        pramin_w32(state, entry_off, handle);
        pramin_w32(state, entry_off + 4, entry_ctx);
        return;
    }
}

/// Insert RAMHT entry with aliases (handle, handle|0x100, handle|0x200, handle|0x300)
fn ramht_insert_aliases(state: &mut Nv2aState, handle: u32, inst: u32, chid: u32) {
    let ramht_base = 0x10000u32;
    let ramht_size = 0x1000u32; // 4KB
    ramht_insert(state, ramht_base, ramht_size, chid, handle, inst);
    ramht_insert(state, ramht_base, ramht_size, chid, handle | 0x100, inst);
    ramht_insert(state, ramht_base, ramht_size, chid, handle | 0x200, inst);
    ramht_insert(state, ramht_base, ramht_size, chid, handle | 0x300, inst);
}

// ============================================================================
// Bootstrap — port of C++ AOT_D3D8Init()
// ============================================================================

/// Notifier physical base — reserved high-RAM page.
const NOTIFIER_PHYS_BASE: u32 = 0x01F0_0000;

/// Initialize NV2A shadow registers and PRAMIN to match Xbox BIOS state.
/// Must be called before the game runs. Writes PRAMIN data to guest RAM.
pub fn bootstrap(guest_mem: &crate::xbox::memory::guest_memory::GuestMemory) -> Box<Nv2aState> {
    let mut state = Box::new(Nv2aState::new());
    let mut pctx = PraminCtx { free_inst: 0x110A }; // After RAMHT + RAMFC + RAMRO

    // --- PMC ---
    state.pmc[0] = 0x02A0_00A1; // PMC_ID: NV2A (Xbox GPU)
    state.pmc[0x100 / 4] = 0; // PMC_INTR: no pending interrupts
    state.pmc[0x140 / 4] = 0x0000_0001; // PMC_INTR_EN: master interrupt enable
    state.pmc[0x200 / 4] = 0xFFFF_FFFF; // PMC_ENABLE: all engines enabled

    // --- PBUS ---
    state.pbus[0] = 0x02A0_00A1; // Same ID on PBUS

    // --- PTIMER --- 31.25 MHz from 233 MHz GPU PLL
    state.w_ptimer(0x9200, 56968); // NUMERATOR
    state.w_ptimer(0x9210, 7629); // DENOMINATOR
    state.w_ptimer(0x9140, 0); // INTR_EN: disabled

    // --- PFB --- Memory configuration
    state.w_pfb(0x10020C, 0x03FF_FFFF); // CSTATUS: 64MB RAM
    state.w_pfb(0x100214, 0); // NVM: disabled
                              // Clear tile config (8 tiles × 4 regs)
    for i in 0..8u32 {
        state.w_pfb(0x100240 + i * 16 + 0, 0);
        state.w_pfb(0x100240 + i * 16 + 4, 0);
        state.w_pfb(0x100240 + i * 16 + 8, 0);
        state.w_pfb(0x100240 + i * 16 + 12, 0);
    }

    // --- PVIDEO --- Video overlay defaults (unity scale/contrast)
    state.w_pvideo(0x8910, 0x0000_1000); // LUMINANCE_0: unity
    state.w_pvideo(0x8914, 0x0000_1000); // LUMINANCE_1
    state.w_pvideo(0x8918, 0x0000_1000); // CHROMINANCE_0
    state.w_pvideo(0x891C, 0x0000_1000); // CHROMINANCE_1
    state.w_pvideo(0x8928, 0xFFFF_FFFF); // SIZE_IN_0
    state.w_pvideo(0x892C, 0xFFFF_FFFF); // SIZE_IN_1
    state.w_pvideo(0x8938, 0x0010_0000); // DS_DX_0: unity
    state.w_pvideo(0x893C, 0x0010_0000); // DS_DX_1
    state.w_pvideo(0x8940, 0x0010_0000); // DT_DY_0: unity
    state.w_pvideo(0x8944, 0x0010_0000); // DT_DY_1

    // --- PCRTC ---
    state.pcrtc[0x800 / 4] = 0x03C0_0000; // PCRTC_START: 60MB (64MB - 4MB GPU reserved)
    state.pcrtc[0x100 / 4] = 0; // INTR: clear
    state.pcrtc[0x140 / 4] = 0x0000_0001; // INTR_EN: VBlank enabled

    // Zero RAMHT + RAMFC + RAMRO area in PRAMIN
    for i in (0x10000u32..0x110C0).step_by(4) {
        pramin_w32(&mut state, i, 0);
    }

    // --- PFIFO --- Command FIFO + DMA engine
    state.w_pfifo(0x2210, 0x0300_0100); // RAMHT: base=0x10000, search=128
    state.w_pfifo(0x2214, 0x0089_0110); // RAMFC: base=0x11000
    state.w_pfifo(0x3210, 0); // CACHE1_PUT
    state.w_pfifo(0x3270, 0); // CACHE1_GET
    state.w_pfifo(0x3240, 0); // CACHE1_DMA_PUT
    state.w_pfifo(0x3244, 0); // CACHE1_DMA_GET
    state.w_pfifo(0x3228, 0); // CACHE1_DMA_STATE
    state.w_pfifo(0x2504, 0); // MODE: all PIO initially
    state.w_pfifo(0x2508, 0); // DMA: not pending
    state.w_pfifo(0x250C, 0); // SIZE
    state.w_pfifo(0x2410, 0); // RUNOUT_PUT
    state.w_pfifo(0x2420, 0); // RUNOUT_GET
    state.w_pfifo(0x3224, 0x000F_0078); // CACHE1_DMA_FETCH
    state.w_pfifo(0x2044, 0x0101_FFFF); // DMA_TIMESLICE
    state.w_pfifo(0x2040, 0x0000_00FF); // DELAY_0
                                        // DMA_PUSH: enabled + idle (bit4=0 for D3D to proceed)
    state.w_pfifo(0x3214, 0x0000_0001);
    // CACHE1_STATUS: low-mark empty (bit4=1), high-mark empty (bit8=0) → FIFO idle
    state.w_pfifo(0x3218, 0x0000_0010);
    // INTR_0: clear (W1C pattern — store 0 directly)
    state.pfifo[(0x2100 - 0x2000) / 4] = 0;
    state.w_pfifo(0x2140, 0x0111_1111); // INTR_EN_0: enable all

    // --- PGRAPH --- Graphics engine status: idle
    // PGRAPH_STATUS (0x400700): 0 = all idle, no busy bits
    state.pgraph[(0x700) / 4] = 0; // All engines idle
                                   // PGRAPH_INTR (0x400100): no pending interrupts
    state.pgraph[(0x100) / 4] = 0;

    // --- PRAMIN DMA contexts ---
    let vram_limit = 0x03FF_FFFFu32;
    let dma3 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma3, 0x3D, 0, vram_limit, false);
    let dma4 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma4, 0x03, 0, vram_limit, false);
    let dma5 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma5, 0x02, 0, vram_limit, false);
    let dma6 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma6, 0x02, 0, vram_limit, false);
    let dma9 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma9, 0x3D, 0, vram_limit, false);
    let dma10 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma10, 0x3D, 0, vram_limit, false);
    let dma11 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma11, 0x3D, 0, vram_limit, false);
    // SYSMEM notification DMA contexts
    let dma2 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma2, 0x03, NOTIFIER_PHYS_BASE, 0x1F, true);
    let dma7 = pctx.alloc(1);
    pramin_dma_ctx(
        &mut state,
        dma7,
        0x3D,
        NOTIFIER_PHYS_BASE + 0x20,
        0x1F,
        true,
    );
    let dma8 = pctx.alloc(1);
    pramin_dma_ctx(
        &mut state,
        dma8,
        0x3D,
        NOTIFIER_PHYS_BASE + 0x40,
        0x20,
        true,
    );
    let dma12 = pctx.alloc(1);
    pramin_dma_ctx(&mut state, dma12, 0x3D, 0x8000_0000, 0x1000_0000, true);

    // --- Graphics context table + big FIFO context ---
    let gr_ctx_table = pctx.alloc(8);
    let fifo_big = pctx.alloc(0x37F);
    // Zero big FIFO context
    for i in 0..0x37F0u32 / 4 {
        pramin_w32(&mut state, (fifo_big << 4) + i * 4, 0);
    }
    // Channel 0 → big context, channel 1 → unused
    pramin_w32(&mut state, (gr_ctx_table << 4), fifo_big);
    pramin_w32(&mut state, (gr_ctx_table << 4) + 4, 0);

    // Graphics class objects
    let gr_kelvin = pctx.alloc(1);
    pramin_w32(&mut state, gr_kelvin << 4, 0x0000_0097); // NV097 Kelvin 3D
    let gr_m2m = pctx.alloc(1);
    pramin_w32(&mut state, gr_m2m << 4, 0x0000_0039); // MemToMem
    let gr_mtex = pctx.alloc(1);
    pramin_w32(&mut state, gr_mtex << 4, 0x0000_009F); // MultiTex
    let gr_surf = pctx.alloc(1);
    pramin_w32(&mut state, gr_surf << 4, 0x0000_0062); // Surfaces3D

    // RAMHT: insert bootstrap object aliases
    ramht_insert_aliases(&mut state, 0x0000_000D, gr_kelvin, 0); // Kelvin
    ramht_insert_aliases(&mut state, 0x0000_000E, gr_m2m, 0); // M2M
    ramht_insert_aliases(&mut state, 0x0000_0010, gr_mtex, 0); // MultiTex
    ramht_insert_aliases(&mut state, 0x0000_0011, gr_surf, 0); // Surfaces3D
                                                               // Notifier DMA handles
    ramht_insert_aliases(&mut state, 0x0010_0001, dma2, 0);
    ramht_insert_aliases(&mut state, 0x0011_0001, dma7, 0);
    ramht_insert_aliases(&mut state, 0x0012_0001, dma8, 0);

    // --- RAMFC --- Channel 0 FIFO context
    let ramfc_base = 0x11000u32;
    pramin_w32(&mut state, ramfc_base + 0, 0); // DMA_PUT
    pramin_w32(&mut state, ramfc_base + 4, 0); // DMA_GET
    pramin_w32(&mut state, ramfc_base + 8, 0); // REF
    pramin_w32(&mut state, ramfc_base + 12, dma6); // DMA_INSTANCE
    pramin_w32(&mut state, ramfc_base + 16, 0); // DMA_STATE
    pramin_w32(&mut state, ramfc_base + 20, 0x000F_0078); // DMA_FETCH config
    pramin_w32(&mut state, ramfc_base + 24, 0); // ENGINE
    pramin_w32(&mut state, ramfc_base + 28, 0); // PULL1

    // PFIFO: set channel 0 DMA mode + DMA instance
    state.w_pfifo(0x3204, 0x0000_0100); // CACHE1_PUSH1: ch0, DMA mode
    state.w_pfifo(0x322C, dma6); // CACHE1_DMA_INSTANCE
    state.w_pfifo(0x3230, 0); // CACHE1_DMA_CTL
    state.w_pfifo(0x3280, 0); // CACHE1_ENGINE: all software
    state.w_pfifo(0x2504, 1); // MODE: channel 0 = DMA
    state.w_pfifo(0x3200, 1); // CACHE1_PUSH0: enabled
    state.w_pfifo(0x3220, 1); // CACHE1_DMA_PUSH: enabled
    state.w_pfifo(0x3250, 1); // CACHE1_PULL0: enabled
    state.w_pfifo(0x2500, 1); // CACHES: enabled

    // --- PGRAPH --- Graphics engine
    state.w_pgraph(0x400720, 0); // FIFO: disable during setup
    state.w_pgraph(0x400080, 0x0000_0000); // DEBUG_0
    state.w_pgraph(0x400084, 0x0011_8700); // DEBUG_1
    state.w_pgraph(0x400880, 0x0008_CFFF); // DEBUG_7
    state.w_pgraph(0x40008C, 0xF3EE_0479); // DEBUG_3
    state.w_pgraph(0x400090, 0x0000_0000); // DEBUG_4
    state.w_pgraph(0x400094, 0x0000_0004); // DEBUG_5
    state.w_pgraph(0x400098, 0x0000_0078);
    state.w_pgraph(0x40009C, 0x0000_0040);
    state.w_pgraph(0x400B80, 0x45EA_D10F);
    state.w_pgraph(0x400B84, 0);
    state.w_pgraph(0x400B88, 0);
    state.w_pgraph(0x400780, gr_ctx_table); // CHANNEL_CTX_TABLE
    state.w_pgraph(0x40014C, 0); // CTX_SWITCH1-4
    state.w_pgraph(0x400150, 0);
    state.w_pgraph(0x400154, 0);
    state.w_pgraph(0x400158, 0);
    state.w_pgraph(0x400144, 0x1000_0000); // CTX_CONTROL: DEVICE_ENABLED
    state.w_pgraph(0x400764, 0x0800_0000); // FFINTFC_ST2: CHID_STATUS_VALID
                                           // Tile synchronization
    for i in 0..8u32 {
        state.w_pgraph(0x400900 + i * 16, 0);
        state.w_pgraph(0x400980 + i * 4, 0);
    }
    state.w_pgraph(0x4009A0, 0);
    state.w_pgraph(0x4009A4, state.pfb[0x200 / 4]); // CFG0 = PFB_CFG0
    state.w_pgraph(0x4009A8, state.pfb[0x204 / 4]); // CFG1 = PFB_CFG1
                                                    // PGRAPH INTR: clear + enable all
    state.pgraph[(0x400100 - 0x400000) / 4] = 0;
    state.w_pgraph(0x400140, 0xFFFF_FFFF); // INTR_EN: all
    state.w_pgraph(0x400720, 1); // FIFO: re-enable

    // --- USER channel ---
    state.user[0x40 / 4] = 0; // DMA_PUT
    state.user[0x44 / 4] = 0; // DMA_GET
    state.pb_put = 0;
    state.pb_get = 0;

    // Write PRAMIN data to guest RAM (0xFD700000+ is PAGE_READWRITE)
    let pramin_guest_base = 0xFD70_0000u64;
    let mem_base = guest_mem.base();
    for i in 0..state.pramin.len() {
        let val = state.pramin[i];
        if val != 0 {
            let addr = pramin_guest_base + (i as u64 * 4);
            unsafe {
                *((mem_base as u64 + addr) as *mut u32) = val;
            }
        }
    }

    debug_log(&format!(
        "NV2A bootstrap: PRAMIN {} blocks used, PFIFO ch0 DMA, PGRAPH on, PMC_ID=0x{:08X}",
        pctx.free_inst - 0x110A,
        state.pmc[0]
    ));

    state
}

// ============================================================================
// Pushbuffer consumer — drain NV2A commands so D3D driver doesn't stall
// ============================================================================

/// NV2A pushbuffer command header format:
/// bits [31:18] = method count (number of data dwords)
/// bits [17:2]  = method offset (NV2A register)
/// bits [1:0]   = subchannel / flags
///
/// Special: if header == 0, it's a NOP/fence marker.
/// If bit 30 set (non-incrementing), method stays the same for all data.

/// Consume pushbuffer commands from GET to PUT.
/// Returns the new GET value (= PUT, everything consumed).
/// Also returns counts of recognized methods for diagnostics.
pub fn consume_pushbuffer(mem_base: *const u8, get: u32, put: u32) -> PushbufferResult {
    let mut pos = get;
    let mut result = PushbufferResult::default();

    // Safety: don't process if pointers look wrong
    if get == put {
        return result;
    }
    if get > 0x1000_0000 || put > 0x1000_0000 {
        return result;
    }
    if put < get && (get - put) > 0x100000 {
        return result;
    } // wrap guard

    // Limit processing to prevent infinite loops on garbage data
    let max_commands = 10_000u32;
    let mut cmd_count = 0u32;

    while pos < put && cmd_count < max_commands {
        let header = unsafe { *((mem_base as u64 + pos as u64) as *const u32) };

        // NOP / zero header = skip
        if header == 0 {
            pos += 4;
            cmd_count += 1;
            continue;
        }

        // Jump command (old-style DMA transfer)
        if header & 0xE0000003 == 0x20000000 {
            // Jump to address in header
            pos = header & 0x1FFFFFFC;
            cmd_count += 1;
            continue;
        }

        // Call command
        if header & 0xE0000003 == 0x00000002 {
            pos += 4;
            cmd_count += 1;
            continue;
        }

        // Return command
        if header & 0xE0030003 == 0x00020000 {
            break; // end of subroutine
        }

        // Normal method: extract count and method
        let count = ((header >> 18) & 0x7FF) as u32;
        let method = ((header >> 2) & 0x1FFF) as u32;
        let _subchannel = header & 0x3;
        let non_inc = (header & 0x40000000) != 0;

        // Categorize methods
        match method {
            0x0100 => result.nops += count,            // NOP
            0x0110 => result.notifies += 1,            // NOTIFY
            0x012C => result.syncs += 1,               // WAIT_FOR_IDLE
            0x0130 => result.pm_triggers += 1,         // PM_TRIGGER
            0x0180..=0x01FC => result.dma_setups += 1, // DMA context binds
            0x0200..=0x0210 => result.clears += 1,     // CLEAR_SURFACE variants
            0x017C => {
                // SET_BEGIN_END
                if count >= 1 {
                    let data = unsafe { *((mem_base as u64 + pos as u64 + 4) as *const u32) };
                    if data != 0 {
                        result.begin_end += 1; // begin primitive
                    }
                }
            }
            0x1800..=0x1900 => result.vertex_data += count, // inline vertex data
            0x1D70 => result.draw_arrays += 1,              // DRAW_ARRAYS
            0x1D7C => result.draw_elements += 1,            // DRAW_ELEMENTS (indexed)
            0x1D90 => result.draw_inline += 1,              // INLINE_ARRAY
            _ => result.other += 1,
        }

        // Advance past header + count data dwords
        pos += 4 + count * 4;
        cmd_count += 1;
        result.total_commands += 1;
    }

    result.new_get = pos;
    result
}

#[derive(Default, Debug)]
pub struct PushbufferResult {
    pub new_get: u32,
    pub total_commands: u32,
    pub nops: u32,
    pub notifies: u32,
    pub syncs: u32,
    pub pm_triggers: u32,
    pub dma_setups: u32,
    pub clears: u32,
    pub begin_end: u32,
    pub vertex_data: u32,
    pub draw_arrays: u32,
    pub draw_elements: u32,
    pub draw_inline: u32,
    pub other: u32,
}
