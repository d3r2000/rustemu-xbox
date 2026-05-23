/// VEH MMIO sub-handler — NV2A GPU register decode [NV2A-DATA]
/// Handles read/write access violations in the NV2A MMIO range (0xFD000000+).
/// Decodes the faulting x64 instruction to determine operand size and direction,
/// then emulates the register read/write and advances RIP.

#[cfg(windows)]
use super::veh::{read_reg32, veh_log, write_reg64, VehResult};
#[cfg(windows)]
use crate::xbox::aot::runtime::RuntimeContext;

/// Check if a guest address is in any Xbox MMIO region.
#[cfg(windows)]
pub fn is_nv2a_mmio(guest_addr: u32) -> bool {
    // NV2A registers: 0xFD000000 - 0xFD7FFFFF
    // NV2A USER channel: 0xFD800000 - 0xFD801FFF
    // APU (MCPX): 0xFE800000 - 0xFE87FFFF
    // AC97 (audio): 0xFEC00000 - 0xFEC00FFF
    // NIC (network): 0xFEF00000 - 0xFEF003FF
    (0xFD00_0000..=0xFD7F_FFFF).contains(&guest_addr)
        || (0xFD80_0000..=0xFD80_1FFF).contains(&guest_addr)
        || (0xFE80_0000..=0xFE87_FFFF).contains(&guest_addr)
        || (0xFEC0_0000..=0xFEC0_0FFF).contains(&guest_addr)
        || (0xFEF0_0000..=0xFEF0_03FF).contains(&guest_addr)
}

/// NV2A USER PUT register — writing here submits pushbuffer commands.
const NV2A_USER_PUT: u32 = 0xFD80_0040;
/// NV2A USER GET register — reading here checks GPU consumption pointer.
const NV2A_USER_GET: u32 = 0xFD80_0044;

#[cfg(windows)]
fn valid_guest_ptr(ptr: u32) -> bool {
    ptr >= 0x0001_0000 && ptr < 0x1000_0000 && (ptr & 3) == 0
}

#[cfg(windows)]
fn plausible_pushbuffer_ptr(ptr: u32) -> bool {
    ptr >= 0x0001_0000 && ptr < 0x2000_0000 && (ptr & 3) == 0
}

#[cfg(windows)]
fn read_device_pb_put(guest_mem: *mut u8) -> u32 {
    let dev_ptr = super::oovpa::read_dev_ptr(guest_mem);
    if !valid_guest_ptr(dev_ptr) {
        return 0;
    }
    let dev_base = guest_mem as u64 + dev_ptr as u64;
    let fence_put = unsafe { std::ptr::read_unaligned((dev_base + 0x30) as *const u32) };
    let hle_put = unsafe {
        std::ptr::read_unaligned(
            (guest_mem as u64 + dev_ptr as u64 + super::oovpa::DEV_PB_PUT as u64) as *const u32,
        )
    };
    static PB_PUT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = PB_PUT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 8 {
        veh_log(&format!(
            "[NV2A-DRAIN-SOURCE] #{} dev=0x{:08X} fence_put[+30]=0x{:08X} hle_put[+{:04X}]=0x{:08X}",
            n,
            dev_ptr,
            fence_put,
            super::oovpa::DEV_PB_PUT,
            hle_put
        ));
    }
    if plausible_pushbuffer_ptr(hle_put) {
        hle_put
    } else {
        0
    }
}

#[cfg(windows)]
fn sync_device_pb_get_put(guest_mem: *mut u8, value: u32) {
    let dev_ptr = super::oovpa::read_dev_ptr(guest_mem);
    if !valid_guest_ptr(dev_ptr) {
        return;
    }
    let dev_base = guest_mem as u64 + dev_ptr as u64;
    unsafe {
        use super::oovpa::{DEV_PB_GET, DEV_PB_PUT};
        std::ptr::write_unaligned((dev_base + DEV_PB_PUT as u64) as *mut u32, value);
        std::ptr::write_unaligned((dev_base + DEV_PB_GET as u64) as *mut u32, value);
    }
}

#[cfg(windows)]
fn sync_d3d_fence_completion(guest_mem: *mut u8, reason: &str) {
    let dev_ptr = super::oovpa::read_dev_ptr(guest_mem);
    if !valid_guest_ptr(dev_ptr) {
        return;
    }

    let dev_base = guest_mem as u64 + dev_ptr as u64;
    let fence_put = unsafe { std::ptr::read_unaligned((dev_base + 0x30) as *const u32) };
    let fence_get_ptr = unsafe { std::ptr::read_unaligned((dev_base + 0x34) as *const u32) };

    if fence_put == 0 || fence_put > 0x0000_FFFF || !valid_guest_ptr(fence_get_ptr) {
        return;
    }

    let get_addr = guest_mem as u64 + fence_get_ptr as u64;
    let old = unsafe { std::ptr::read_unaligned(get_addr as *const u32) };
    if old == fence_put {
        return;
    }

    unsafe {
        std::ptr::write_unaligned(get_addr as *mut u32, fence_put);
    }

    static FENCE_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = FENCE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n < 16 {
        veh_log(&format!(
            "[NV2A-FENCE-DRAIN] #{} {} dev=0x{:08X} put=0x{:08X} get_ptr=0x{:08X} old=0x{:08X} new=0x{:08X}",
            n, reason, dev_ptr, fence_put, fence_get_ptr, old, fence_put
        ));
    }
}

/// Handle an MMIO access violation.
/// Decodes the faulting x64 instruction, emulates the GPU register access,
/// and advances RIP past the instruction.
#[cfg(windows)]
pub fn handle_mmio(
    rip: u64,
    code_end: u64,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_addr: u32,
    is_write: bool,
    ctx: &mut RuntimeContext,
) -> VehResult {
    use iced_x86::{Decoder, DecoderOptions, Instruction, OpKind};

    // 2026-04-21 Layer 5 probe: record MMIO access frequency by address +
    // direction. Dumps top-20 at thresholds so we can identify registers
    // polled in tight loops during stuck periods (CreateDevice internal
    // wait, etc.). Focuses the search — the hot register is the one
    // blocking progress.
    mmio_hist_record(guest_addr, is_write);

    let max_len = ((code_end - rip) as usize).min(15);
    if max_len == 0 {
        return VehResult::NotHandled;
    }

    let code_slice = unsafe { std::slice::from_raw_parts(rip as *const u8, max_len) };

    let mut decoder = Decoder::with_ip(64, code_slice, rip, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    if !decoder.can_decode() {
        return VehResult::NotHandled;
    }
    decoder.decode_out(&mut instr);
    if instr.is_invalid() {
        return VehResult::NotHandled;
    }

    let instr_len = instr.len();

    // Non-NV2A MMIO (APU, AC97, NIC): return 0 for reads, ignore writes.
    // These peripherals are HLE'd at the API level (DirectSound, network).
    if guest_addr >= 0xFE80_0000 {
        if !is_write {
            set_read_result(&instr, context, 0);
        }
        ctx.mmio_count += 1;
        context.Rip = rip + instr_len as u64;
        return VehResult::Handled;
    }

    // NV2A offset relative to 0xFD000000
    let nv2a_offset = guest_addr - 0xFD00_0000;

    if is_write {
        let value = extract_write_value(&instr, context);
        // Intercept writes to USER GET (0xFD800044): the game resets GET=0
        // expecting the DMA engine to advance it. Since we have no DMA engine,
        // override GET with current PUT so the game always sees "consumed."
        if guest_addr == NV2A_USER_GET {
            let put = super::nv2a::shadow_read(0x80_0040);
            let override_val = if put != 0 { put } else { value };
            super::nv2a::shadow_write(nv2a_offset, override_val);
            // Also sync the dynamically discovered D3D device PB bookkeeping.
            let guest_mem = ctx.guest_mem_base;
            sync_device_pb_get_put(guest_mem, override_val);
            // Write override to guest memory NV2A region too
            unsafe {
                let nv2a_user = guest_mem as u64 + 0xFD800000u64;
                *((nv2a_user + 0x44) as *mut u32) = override_val;
            }
        } else if nv2a_offset == 0x0100
            || nv2a_offset == 0x2100
            || nv2a_offset == 0x3220
            || nv2a_offset == 0x60_0100
            || nv2a_offset == 0x40_0100
        {
            // NV2A interrupt registers use write-1-to-clear semantics:
            // writing a 1 bit CLEARS the corresponding pending bit.
            // PMC_INTR (0x100), PFIFO_INTR (0x2100), PFIFO_DMA_STATE
            // (0x3220), PCRTC_INTR (0x600100), PGRAPH_INTR (0x400100)
            let current = super::nv2a::shadow_read(nv2a_offset);
            super::nv2a::shadow_write(nv2a_offset, current & !value);
            trace_nv2a_intr_access(guest_addr, nv2a_offset, true, value, current & !value);

            // PMC_INTR is an aggregate register: each bit reflects whether the
            // corresponding engine's interrupt register has any pending bits.
            // When PFIFO_INTR, PCRTC_INTR or PGRAPH_INTR is fully cleared, auto-clear
            // the corresponding PMC_INTR bit.
            if nv2a_offset == 0x2100 {
                // PFIFO_INTR cleared → clear PMC_INTR bit 8
                let new_pfifo = super::nv2a::shadow_read(0x2100);
                if new_pfifo == 0 {
                    let pmc = super::nv2a::shadow_read(0x0100);
                    super::nv2a::shadow_write(0x0100, pmc & !0x0000_0100);
                }
            } else if nv2a_offset == 0x60_0100 {
                // PCRTC_INTR cleared → clear PMC_INTR bit 24
                let new_pcrtc = super::nv2a::shadow_read(0x60_0100);
                if new_pcrtc == 0 {
                    let pmc = super::nv2a::shadow_read(0x0100);
                    super::nv2a::shadow_write(0x0100, pmc & !0x0100_0000);
                }
            } else if nv2a_offset == 0x40_0100 {
                // PGRAPH_INTR cleared → clear PMC_INTR bit 12
                let new_pgraph = super::nv2a::shadow_read(0x40_0100);
                if new_pgraph == 0 {
                    let pmc = super::nv2a::shadow_read(0x0100);
                    super::nv2a::shadow_write(0x0100, pmc & !0x0000_1000);
                }
            }
        } else {
            // Write to shadow registers normally
            super::nv2a::shadow_write(nv2a_offset, value);
            trace_nv2a_intr_access(
                guest_addr,
                nv2a_offset,
                true,
                value,
                super::nv2a::shadow_read(nv2a_offset),
            );
        }
        if guest_addr == NV2A_USER_PUT {
            ctx.pb_commands += 1;
            // NV2A pushbuffer geometry extraction (2026-04-20, splice from JIT).
            // Drive the Kelvin/NV097 parser through the DMA stream from the
            // last-seen GET up to the new PUT. The parser queues completed
            // vertex batches via gpu::queue_draw; hle_swap drains the queue
            // into the backend before readback.
            super::nv2a_pb::drive(ctx.guest_mem_base, value);
            sync_d3d_fence_completion(ctx.guest_mem_base, "user-put");
            // PB spinlock breaker: when PUT is written, immediately set GET=PUT.
            // Sync ALL locations the game might read GET from:
            // 1. NV2A shadow registers (MMIO read path)
            super::nv2a::shadow_write(0x80_0044, value); // USER GET = PUT
            super::nv2a::shadow_write(0x3244, value); // PFIFO DMA GET = PUT
                                                      // 2. Device struct (read by D3D SDK code / KickOff)
            let guest_mem = ctx.guest_mem_base;
            // NOTE: do NOT write dev+0x00 — that's the NV2A register base
            // pointer (0xFD000000), used by CMiniport/PGRAPH polling code.
            // Overwriting it with PUT corrupts [dev[0]+0x400700] reads.
            sync_device_pb_get_put(guest_mem, value);
            // 3. Guest memory NV2A USER channel (direct reads)
            unsafe {
                let nv2a_user = guest_mem as u64 + 0xFD800000u64;
                *((nv2a_user + 0x44) as *mut u32) = value; // USER GET in guest mem
            }
            // 4. Zero-page offset 0x44 — fallback for when pChannel=0
            //    (CMiniport_CreateCtxDmaObject HLE leaves pChannel NULL,
            //    so native D3D polling reads [0+0x44] instead of MMIO)
            unsafe {
                *((guest_mem as u64 + 0x44) as *mut u32) = value;
            }
            if ctx.pb_commands <= 5 {
                veh_log(&format!(
                    "[NV2A-DATA] PB PUT write value=0x{:08X} → GET=PUT synced everywhere (pb_commands={})",
                    value, ctx.pb_commands
                ));
            }
        }
    } else {
        // (MMIO-HIST probe removed — worker may be spinning on RAM, not MMIO.)
        // Read from shadow registers — with HLE overrides for status polling
        let value = if guest_addr == NV2A_USER_GET {
            // USER GET is the hardware-consumed pointer. When PUT is submitted,
            // emulate an infinitely fast PFIFO by reporting GET == PUT. Use the
            // dynamically inferred g_pDevice for titles beyond Spider-Man.
            let shadow_put = super::nv2a::shadow_read(0x80_0040);
            let dev_put = read_device_pb_put(ctx.guest_mem_base);
            let consumed = if dev_put != 0 {
                dev_put
            } else if shadow_put != 0 {
                shadow_put
            } else {
                super::nv2a::shadow_read(nv2a_offset)
            };
            super::nv2a::shadow_write(0x80_0044, consumed);
            sync_d3d_fence_completion(ctx.guest_mem_base, "user-get-read");
            unsafe {
                let nv2a_user = ctx.guest_mem_base as u64 + 0xFD800000u64;
                std::ptr::write_unaligned((nv2a_user + 0x44) as *mut u32, consumed);
            }
            consumed
        } else if guest_addr >= 0xFD40_0000 && guest_addr < 0xFD50_0000 {
            // PGRAPH registers: return 0 for all status/interrupt registers.
            // The native D3D runtime polls these to check GPU idle:
            // - 0xFD400100 = PGRAPH_INTR (interrupt pending) → 0 = no interrupts
            // - 0xFD400108 = PGRAPH_NSOURCE (error source) → 0 = no errors
            // - 0xFD400700 = PGRAPH_STATUS (pipeline busy) → 0 = idle
            // - 0xFD400704 = PGRAPH register → 0 = idle
            // Returning 0 simulates D3D__NullHardware (infinitely fast GPU).
            0
        } else if guest_addr >= 0xFD00_2400 && guest_addr < 0xFD00_2800 {
            // PFIFO interrupt/status region (0x2400-0x27FF): return 0x10 for all
            // The ISR at 0x002FC807 checks [esi+2400h] bit 4 to decide idle.
            // Bit4=1 = idle/acknowledged. Without this, the ISR loops forever.
            0x0000_0010
        } else if guest_addr == 0xFD00_3244 {
            // PFIFO DMA_GET (CACHE1_DMA_GET). Cherry-picked from JIT commit 19a124b.
            // The native D3D MakeSpace routine polls this register to measure how
            // much of its pushbuffer the GPU has consumed. If GET doesn't advance,
            // MakeSpace spins infinitely (observed: 150K+ iterations).
            // We emulate "infinitely fast GPU consumption" by reporting GET == guest's
            // software PUT (g_pDevice[0]), so the ring always looks fully drained.
            let sw_put = read_device_pb_put(ctx.guest_mem_base);
            let shadow_put = super::nv2a::shadow_read(0x80_0040);
            sync_d3d_fence_completion(ctx.guest_mem_base, "pfifo-dma-get-read");
            let value = if sw_put != 0 {
                super::nv2a::shadow_write(0x3244, sw_put);
                sw_put
            } else if shadow_put != 0 {
                super::nv2a::shadow_write(0x3244, shadow_put);
                shadow_put
            } else {
                super::nv2a::shadow_read(nv2a_offset)
            };
            static DRAIN_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = DRAIN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 8 {
                veh_log(&format!(
                    "[NV2A-DRAIN] PFIFO DMA_GET poll #{} → 0x{:08X} (drained to sw PUT)",
                    n, value
                ));
            }
            value
        } else if guest_addr == 0xFD00_3218 {
            // PFIFO CACHE1_STATUS: always return 0x10 (low-mark empty = FIFO idle)
            0x0000_0010
        } else if guest_addr == 0xFD00_3220 {
            // PFIFO CACHE1_DMA_FETCH: bit4=0 means not fetching (idle)
            // ISR at 0x002FC810 checks this, and exit at 0x002FC817 requires bit4=0.
            // Return 0 so the function exits instead of looping.
            0
        } else if guest_addr == 0xFD00_3228 {
            // PFIFO CACHE1_DMA_STATE: 0 = idle
            0
        } else if guest_addr == 0xFD00_3214 {
            // PFIFO CACHE1_DMA_PUSH: bit0=access(1), bit4=status(1=suspended/idle)
            // The ISR at 0x002FC0D6/002FC10E loops until bit4 is set.
            // Bit4=1 means DMA pusher is idle/suspended — GPU finished processing.
            0x0000_0011
        } else if guest_addr >= 0xFD10_0000 && guest_addr < 0xFD10_1000 {
            // PFB (Frame Buffer controller): return 0 for tile/status registers.
            // Game writes config, then polls waiting for GPU to clear status bits.
            0
        } else if guest_addr >= 0xFD80_0000 && guest_addr <= 0xFD80_1FFF {
            // NV2A USER channel: game polls GET/status to check GPU progress.
            // Return current PUT value so GET == PUT (GPU consumed everything).
            // This breaks "wait for GPU" spin loops like the one at 0x00345780
            // which polls [0xFE820010] waiting for GET >= 0x20.
            let put = super::nv2a::shadow_read(0x80_0040);
            if put >= 0x20 {
                put
            } else {
                0x1000
            }
        } else {
            super::nv2a::shadow_read(nv2a_offset)
        };
        trace_nv2a_intr_access(guest_addr, nv2a_offset, false, value, value);
        set_read_result(&instr, context, value);

        // Diagnostic: log the first few GET reads after KickOff to debug polling
        if guest_addr == NV2A_USER_GET && ctx.mmio_count >= 120 && ctx.mmio_count <= 135 {
            let dev_ptr = super::oovpa::read_dev_ptr(ctx.guest_mem_base);
            let dev_pb_put = read_device_pb_put(ctx.guest_mem_base);
            let dev0 = if valid_guest_ptr(dev_ptr) {
                unsafe {
                    std::ptr::read_unaligned(
                        (ctx.guest_mem_base as u64 + dev_ptr as u64) as *const u32,
                    )
                }
            } else {
                0xDEAD
            };
            let ch_ptr = if valid_guest_ptr(dev_ptr) {
                unsafe {
                    std::ptr::read_unaligned(
                        (ctx.guest_mem_base as u64 + dev_ptr as u64 + 0x2264) as *const u32,
                    )
                }
            } else {
                0xDEAD
            };
            veh_log(&format!(
                "[GET-PROBE] mmio={} GET_returned=0x{:08X} dev=0x{:08X} dev[0]=0x{:08X} dev_pb_put=0x{:08X} pCh=0x{:08X} shadow_PUT=0x{:08X}",
                ctx.mmio_count, value, dev_ptr, dev0, dev_pb_put, ch_ptr,
                super::nv2a::shadow_read(0x80_0040)
            ));
        }
    }

    ctx.mmio_count += 1;

    // Log MMIO 500-550 to capture the ISR loop pattern
    if ctx.mmio_count >= 500 && ctx.mmio_count <= 550 {
        let value_str = if is_write {
            format!("val=0x{:08X}", extract_write_value(&instr, context))
        } else {
            String::from("")
        };
        veh_log(&format!(
            "[NV2A-DATA] {} guest=0x{:08X} off=0x{:06X} {} (total={})",
            if is_write { "WR" } else { "RD" },
            guest_addr,
            nv2a_offset,
            value_str,
            ctx.mmio_count
        ));
    }

    context.Rip = rip + instr_len as u64;
    VehResult::Handled
}

/// Extract the value being written to MMIO from the instruction's source operand.
#[cfg(windows)]
pub(crate) fn extract_write_value(
    instr: &iced_x86::Instruction,
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> u32 {
    use iced_x86::OpKind;

    for i in 0..instr.op_count() {
        match instr.op_kind(i) {
            OpKind::Register => {
                if i > 0 || instr.op_count() == 1 {
                    return read_reg32(instr.op_register(i), context);
                }
            }
            OpKind::Immediate32 | OpKind::Immediate32to64 => {
                return instr.immediate32();
            }
            OpKind::Immediate16 => {
                return instr.immediate16() as u32;
            }
            OpKind::Immediate8 | OpKind::Immediate8to32 | OpKind::Immediate8to64 => {
                return instr.immediate8() as u32;
            }
            _ => {}
        }
    }
    0
}

/// Set the result of an MMIO read into the instruction's destination register,
/// or update EFLAGS for instructions that read memory without a register destination
/// (TEST, CMP, OR/AND/XOR with memory source).
#[cfg(windows)]
pub(crate) fn set_read_result(
    instr: &iced_x86::Instruction,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    value: u32,
) {
    use iced_x86::{Mnemonic, OpKind};

    let mnemonic = instr.mnemonic();

    // TEST mem, imm  or  TEST mem, reg — no register destination, only sets flags
    if mnemonic == Mnemonic::Test {
        // op0 = memory (the MMIO address), op1 = immediate or register
        let operand = if instr.op_count() >= 2 {
            match instr.op_kind(1) {
                OpKind::Immediate8 | OpKind::Immediate8to32 | OpKind::Immediate8to64 => {
                    instr.immediate8() as u32
                }
                OpKind::Immediate32 | OpKind::Immediate32to64 => instr.immediate32(),
                OpKind::Immediate16 => instr.immediate16() as u32,
                OpKind::Register => read_reg32(instr.op_register(1), context),
                _ => 0,
            }
        } else {
            0
        };

        // Determine operand size from the memory operand
        let size = instr.memory_size();
        let result = match size {
            iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => {
                (value as u8 as u32) & (operand as u8 as u32)
            }
            iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => {
                (value as u16 as u32) & (operand as u16 as u32)
            }
            _ => value & operand,
        };
        let bit_size = match size {
            iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => 8,
            iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => 16,
            _ => 32,
        };
        set_flags_logical(context, result, bit_size);
        return;
    }

    // CMP mem, imm  or  CMP mem, reg — no register destination, only sets flags
    if mnemonic == Mnemonic::Cmp {
        // Could be CMP [mem], imm/reg  OR  CMP reg, [mem]
        if instr.op_count() >= 2 && instr.op_kind(0) == OpKind::Register {
            // CMP reg, [mem] — reg is op0, memory value is the source
            let reg_val = read_reg32(instr.op_register(0), context);
            let size = instr.memory_size();
            let bit_size = match size {
                iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => 8,
                iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => 16,
                _ => 32,
            };
            set_flags_sub(context, reg_val, value, bit_size);
        } else if instr.op_count() >= 2 {
            // CMP [mem], imm/reg — memory is op0
            let operand = match instr.op_kind(1) {
                OpKind::Immediate8 | OpKind::Immediate8to32 | OpKind::Immediate8to64 => {
                    instr.immediate8() as i8 as i32 as u32
                }
                OpKind::Immediate32 | OpKind::Immediate32to64 => instr.immediate32(),
                OpKind::Immediate16 => instr.immediate16() as u32,
                OpKind::Register => read_reg32(instr.op_register(1), context),
                _ => 0,
            };
            let size = instr.memory_size();
            let bit_size = match size {
                iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => 8,
                iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => 16,
                _ => 32,
            };
            set_flags_sub(context, value, operand, bit_size);
        }
        return;
    }

    // MOV reg, [mem] — standard register destination
    if instr.op_count() >= 1 && instr.op_kind(0) == OpKind::Register {
        write_reg64(instr.op_register(0), context, value as u64);
    }
}

/// Compute and set EFLAGS for logical operations (TEST, AND, OR, XOR).
/// CF=0, OF=0, ZF/SF/PF computed from result.
#[cfg(windows)]
fn set_flags_logical(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    result: u32,
    bit_size: u32,
) {
    let mask = if bit_size == 8 {
        0xFF
    } else if bit_size == 16 {
        0xFFFF
    } else {
        0xFFFF_FFFF
    };
    let masked = result & mask;
    let sf_bit = 1u32 << (bit_size - 1);

    let zf = if masked == 0 { 1u32 } else { 0u32 };
    let sf = if (masked & sf_bit) != 0 { 1u32 } else { 0u32 };
    let pf = parity(masked as u8);

    // EFLAGS bit positions: CF=0, PF=2, ZF=6, SF=7, OF=11
    let mut eflags = context.EFlags as u32;
    eflags &= !(1 << 0); // CF = 0
    eflags &= !(1 << 11); // OF = 0
    eflags = (eflags & !(1 << 6)) | (zf << 6); // ZF
    eflags = (eflags & !(1 << 7)) | (sf << 7); // SF
    eflags = (eflags & !(1 << 2)) | (pf << 2); // PF
    context.EFlags = eflags as u32;
}

/// Compute and set EFLAGS for subtraction (CMP, SUB).
/// CF, ZF, SF, OF, PF computed from left - right.
#[cfg(windows)]
fn set_flags_sub(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    left: u32,
    right: u32,
    bit_size: u32,
) {
    let mask = if bit_size == 8 {
        0xFFu64
    } else if bit_size == 16 {
        0xFFFFu64
    } else {
        0xFFFF_FFFFu64
    };
    let l = (left as u64) & mask;
    let r = (right as u64) & mask;
    let result = l.wrapping_sub(r);
    let masked = (result & mask) as u32;
    let sf_bit = 1u32 << (bit_size - 1);

    let cf = if l < r { 1u32 } else { 0u32 };
    let zf = if masked == 0 { 1u32 } else { 0u32 };
    let sf = if (masked & sf_bit) != 0 { 1u32 } else { 0u32 };
    // OF: sign of (left ^ right) & (left ^ result) — overflow when signs disagree
    let of = if (((left ^ right) & (left ^ masked)) & sf_bit) != 0 {
        1u32
    } else {
        0u32
    };
    let pf = parity(masked as u8);

    let mut eflags = context.EFlags as u32;
    eflags = (eflags & !(1 << 0)) | (cf << 0); // CF
    eflags = (eflags & !(1 << 6)) | (zf << 6); // ZF
    eflags = (eflags & !(1 << 7)) | (sf << 7); // SF
    eflags = (eflags & !(1 << 11)) | (of << 11); // OF
    eflags = (eflags & !(1 << 2)) | (pf << 2); // PF
    context.EFlags = eflags as u32;
}

/// Compute x86 parity flag: 1 if low byte has even number of set bits.
#[cfg(windows)]
fn parity(byte: u8) -> u32 {
    if byte.count_ones() % 2 == 0 {
        1
    } else {
        0
    }
}

#[cfg(windows)]
fn trace_nv2a_intr_access(
    guest_addr: u32,
    nv2a_offset: u32,
    is_write: bool,
    value: u32,
    shadow_after: u32,
) {
    use std::sync::atomic::{AtomicU32, Ordering};

    fn name(offset: u32) -> Option<&'static str> {
        match offset {
            0x0001_00 => Some("PMC_INTR"),
            0x0001_40 => Some("PMC_INTR_EN"),
            0x0021_00 => Some("PFIFO_INTR"),
            0x0021_40 => Some("PFIFO_INTR_EN"),
            0x0032_14 => Some("PFIFO_DMA_PUSH"),
            0x0032_18 => Some("PFIFO_CACHE1_STATUS"),
            0x0032_20 => Some("PFIFO_DMA_FETCH"),
            0x0032_28 => Some("PFIFO_DMA_STATE"),
            0x0081_00 => Some("PBUS_INTR"),
            0x0091_00 => Some("PTIMER_INTR"),
            0x4001_00 => Some("PGRAPH_INTR"),
            0x4001_40 => Some("PGRAPH_INTR_EN"),
            0x6001_00 => Some("PCRTC_INTR"),
            0x6001_40 => Some("PCRTC_INTR_EN"),
            _ => None,
        }
    }

    let Some(reg_name) = name(nv2a_offset) else {
        return;
    };

    // Keep this probe bounded. We need the interrupt bit pattern around the
    // first DPC abort, not a forever trace of every frame.
    static INTR_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);
    let n = INTR_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if n >= 256 {
        return;
    }

    let pmc = super::nv2a::shadow_read(0x0100);
    let pfifo = super::nv2a::shadow_read(0x2100);
    let pgraph = super::nv2a::shadow_read(0x40_0100);
    let pcrtc = super::nv2a::shadow_read(0x60_0100);
    veh_log(&format!(
        "[NV2A-INTR] #{} {} {} guest=0x{:08X} off=0x{:06X} value=0x{:08X} after=0x{:08X} pmc=0x{:08X} pfifo=0x{:08X} pgraph=0x{:08X} pcrtc=0x{:08X}",
        n,
        if is_write { "WR" } else { "RD" },
        reg_name,
        guest_addr,
        nv2a_offset,
        value,
        shadow_after,
        pmc,
        pfifo,
        pgraph,
        pcrtc
    ));
}

// ============================================================================
// MMIO histogram (Layer 5 probe, 2026-04-21)
//
// Per-address hit counter with direction split. Dumps top-20 hot addresses
// at 10K / 50K / 200K / 1M thresholds. The register(s) dominating the
// hotlist during a stuck period are what the guest is polling — that's
// what we need to make return "ready" for CreateDevice's internal wait
// loop (or any other MMIO spin) to progress.
// ============================================================================

const MMIO_HIST_BUCKETS: usize = 4096;
static MMIO_READS: [std::sync::atomic::AtomicU32; MMIO_HIST_BUCKETS] = {
    const Z: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    [Z; MMIO_HIST_BUCKETS]
};
static MMIO_WRITES: [std::sync::atomic::AtomicU32; MMIO_HIST_BUCKETS] = {
    const Z: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    [Z; MMIO_HIST_BUCKETS]
};
static MMIO_HIST_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Hash `guest_addr` into `MMIO_HIST_BUCKETS` slots. 4-byte aligned bucketing
/// then modular reduction — handles the whole 0xFD000000..0xFE800000 range.
fn mmio_bucket(guest_addr: u32) -> usize {
    ((guest_addr >> 2) as usize) & (MMIO_HIST_BUCKETS - 1)
}

fn mmio_hist_record(guest_addr: u32, is_write: bool) {
    use std::sync::atomic::Ordering;
    let slot = mmio_bucket(guest_addr);
    let bin = if is_write {
        &MMIO_WRITES[slot]
    } else {
        &MMIO_READS[slot]
    };
    bin.fetch_add(1, Ordering::Relaxed);
    let tot = MMIO_HIST_TOTAL.fetch_add(1, Ordering::Relaxed);
    let new_tot = tot + 1;
    // Dump at progressively rarer thresholds so we see pattern evolution
    // without spamming the log.
    for &threshold in &[10_000u64, 50_000, 200_000, 1_000_000, 5_000_000] {
        if tot < threshold && new_tot >= threshold {
            mmio_hist_dump_top(20, threshold);
        }
    }
}

fn mmio_hist_dump_top(top_n: usize, total_at_dump: u64) {
    use std::sync::atomic::Ordering;
    // Gather (slot, reads, writes) triples; sort by reads desc then writes desc.
    let mut entries: Vec<(usize, u32, u32)> = (0..MMIO_HIST_BUCKETS)
        .filter_map(|slot| {
            let r = MMIO_READS[slot].load(Ordering::Relaxed);
            let w = MMIO_WRITES[slot].load(Ordering::Relaxed);
            if r == 0 && w == 0 {
                None
            } else {
                Some((slot, r, w))
            }
        })
        .collect();
    entries.sort_by(|a, b| (b.1 + b.2).cmp(&(a.1 + a.2)));
    veh_log(&format!(
        "[MMIO-HIST] total={} unique_bins={} top_{}:",
        total_at_dump,
        entries.len(),
        top_n
    ));
    for (slot, reads, writes) in entries.iter().take(top_n) {
        // We can't recover exact addresses (bucket collisions possible) —
        // but bucket index × 4 + NV2A_BASE gives the primary candidate.
        // For NV2A range, slot × 4 lands within 0x00_0000..0x00_4000 offset.
        let primary = ((*slot as u32) << 2) | 0xFD00_0000;
        veh_log(&format!(
            "  bin=0x{:04X} (primary addr ≈ 0x{:08X}) R={} W={}",
            slot, primary, reads, writes
        ));
    }
}
