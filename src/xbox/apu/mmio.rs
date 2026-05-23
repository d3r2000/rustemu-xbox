/// APU MMIO handler — MCPX southbridge audio register emulation.
///
/// Implements shadow registers for the APU so that DSOUND code polling
/// hardware status bits gets sensible "idle/ready" values instead of
/// blanket 0xFFFFFFFF.  Writes to command/control registers auto-update
/// the corresponding status registers so spin-loops break immediately.
///
/// Register semantics derived from:
///   - xemu  hw/xbox/mcpx/apu/apu.c  (mcpx_apu_read / mcpx_apu_write)
///   - xemu  hw/xbox/mcpx/apu/apu_regs.h
///   - C++ v1  AOT_DSound.h  (Spider-Man hotspot offsets)
///
/// Two callers:
///   1. VEH handler (handle_apu_mmio) — native AOT path
///   2. Unicorn hook (apu_shadow_read / apu_shadow_write) — sniper path
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use super::regs;
use super::{AC97_BASE, AC97_END, APU_BASE, APU_END, NIC_BASE, NIC_END};

// ---------------------------------------------------------------------------
// Shadow register file — 128 KB covers the "hot" 0x00000-0x1FFFF range.
// Accesses above 0x20000 (GP DSP scratch, EP scratch) are handled specially.
// ---------------------------------------------------------------------------

const SHADOW_SIZE: usize = 0x2_0000; // 128 KB — covers all control regs

/// Lazy-initialized shadow register backing store.
/// Access only through `with_shadow()`.
static SHADOW: Mutex<Option<Box<[u32]>>> = Mutex::new(None);

/// Global APU MMIO counter.
static GLOBAL_APU_MMIO_COUNT: AtomicU64 = AtomicU64::new(0);

/// Initialise the shadow register file with power-on defaults.
/// Called once on first access.
fn init_shadow() -> Box<[u32]> {
    let count = SHADOW_SIZE / 4;
    let mut regs = vec![0u32; count].into_boxed_slice();

    // FE control: start in FREE_RUNNING mode (not trapped/halted) so that
    // any code checking FECTL doesn't think the FE is stuck.
    let fectl_idx = (super::regs::NV_PAPU_FECTL as usize) / 4;
    if fectl_idx < regs.len() {
        regs[fectl_idx] = super::regs::NV_PAPU_FECTL_FEMETHMODE_FREE_RUNNING;
    }

    // SE control: XCNTMODE = OFF (no frame counter running)
    let sectl_idx = (super::regs::NV_PAPU_SECTL as usize) / 4;
    if sectl_idx < regs.len() {
        regs[sectl_idx] = super::regs::NV_PAPU_SECTL_XCNTMODE_OFF;
    }

    // Interrupt status: clear (no pending interrupts)
    // NV_PAPU_ISTS = 0  → no GINTSTS, no FETINTSTS

    // FETFORCE1: SE2FE_IDLE_VOICE set → voice processor reports idle
    let fetforce1_idx = (super::regs::NV_PAPU_FETFORCE1 as usize) / 4;
    if fetforce1_idx < regs.len() {
        regs[fetforce1_idx] = super::regs::NV_PAPU_FETFORCE1_SE2FE_IDLE_VOICE;
    }

    regs
}

/// Run `f` with mutable access to the shadow register file.
fn with_shadow<F, R>(f: F) -> R
where
    F: FnOnce(&mut [u32]) -> R,
{
    let mut guard = SHADOW.lock().unwrap();
    if guard.is_none() {
        *guard = Some(init_shadow());
    }
    f(guard.as_mut().unwrap())
}

// ---------------------------------------------------------------------------
// Public read / write — used by both VEH and Unicorn paths.
// ---------------------------------------------------------------------------

/// Read an APU shadow register.
/// `offset` is relative to APU_BASE (0xFE800000).
///
/// Returns a value that satisfies DSOUND polling loops:
///   - GP_DSP_SCRATCH: 0 (idle)
///   - EP_STATUS / EP_DSP_STATUS: 0 (idle)
///   - NV_PAPU_XGSCNT: monotonic tick counter
///   - NV_PAPU_ISTS: 0 (no pending interrupts)
///   - FECTL: current shadow value (FREE_RUNNING after init)
///   - Everything else: shadow value (default 0)
pub fn apu_shadow_read(offset: u32) -> u32 {
    // GP DSP scratch RAM (0x020000+) — always return 0 ("idle")
    if offset >= 0x02_0000 && offset < 0x03_0000 {
        return 0;
    }
    // EP control area (0x030000-0x03FFFF) — status = 0 (idle)
    if offset >= 0x03_0000 && offset < 0x04_0000 {
        return 0;
    }
    // EP DSP area (0x050000-0x05FFFF) — status = 0 (idle)
    if offset >= 0x05_0000 && offset < 0x06_0000 {
        return 0;
    }

    // XGSCNT — global sample counter, return monotonic tick
    if offset == regs::NV_PAPU_XGSCNT {
        // 48 kHz * 32 samples/frame; approximate with a cheap counter
        static XGSCNT: AtomicU64 = AtomicU64::new(0);
        return XGSCNT.fetch_add(32, Ordering::Relaxed) as u32;
    }

    // Within shadow range: return stored value
    let idx = (offset as usize) / 4;
    if idx < SHADOW_SIZE / 4 {
        with_shadow(|regs| regs[idx])
    } else {
        0
    }
}

/// Write an APU shadow register.
/// `offset` is relative to APU_BASE (0xFE800000).
///
/// Special semantics:
///   - NV_PAPU_ISTS: write-1-to-clear (W1C) — clears the written bits
///   - NV_PAPU_FECTL: store value, then auto-trap the FE so that any
///     subsequent poll of FECTL sees TRAPPED (matching xemu behavior
///     where the FE thread traps itself after processing a command).
///   - NV_PAPU_SECTL: store directly (xemu broadcasts a condvar; we
///     just record the value so reads see the game's intent).
///   - GP/EP DSP scratch: absorb silently.
pub fn apu_shadow_write(offset: u32, value: u32) {
    // GP/EP DSP scratch writes — absorb
    if offset >= 0x02_0000 {
        return;
    }

    let idx = (offset as usize) / 4;
    if idx >= SHADOW_SIZE / 4 {
        return;
    }

    with_shadow(|regs| {
        match offset {
            // W1C: clear the bits that the guest wrote 1 to
            o if o == super::regs::NV_PAPU_ISTS => {
                regs[idx] &= !value;
            }
            // FECTL: store the value.  In xemu the FE thread runs commands
            // then sets FEMETHMODE = TRAPPED.  We short-circuit: if the game
            // writes FREE_RUNNING (to kick the FE), we immediately flip to
            // TRAPPED so the next read sees "done processing".
            o if o == super::regs::NV_PAPU_FECTL => {
                regs[idx] = value;
                let mode = value & super::regs::NV_PAPU_FECTL_FEMETHMODE;
                if mode == super::regs::NV_PAPU_FECTL_FEMETHMODE_FREE_RUNNING {
                    // Auto-trap: FE processed the command instantly
                    regs[idx] = (regs[idx] & !super::regs::NV_PAPU_FECTL_FEMETHMODE)
                        | super::regs::NV_PAPU_FECTL_FEMETHMODE_TRAPPED;
                    // Also set FETINTSTS so interrupt-driven code sees completion
                    let ists_idx = (super::regs::NV_PAPU_ISTS as usize) / 4;
                    regs[ists_idx] |= super::regs::NV_PAPU_ISTS_FETINTSTS;
                }
            }
            // SECTL: store directly
            o if o == super::regs::NV_PAPU_SECTL => {
                regs[idx] = value;
            }
            // FEMEMDATA: "magic write" — in xemu this writes to the address in
            // FEMEMADDR.  We just record the value; the guest memory write would
            // need guest_mem access which VEH doesn't provide here.  The store
            // to the shadow is enough to unblock polling.
            o if o == super::regs::NV_PAPU_FEMEMDATA => {
                regs[idx] = value;
            }
            // Default: store directly
            _ => {
                regs[idx] = value;
            }
        }
    });
}

// ---------------------------------------------------------------------------
// AC97 — codec registers (0xFEC00000+)
// ---------------------------------------------------------------------------

/// Read an AC97 register.  `offset` is relative to AC97_BASE.
/// Returns "codec ready" bits so polling loops don't spin.
pub fn ac97_shadow_read(offset: u32) -> u32 {
    // AC97 global status register (offset 0x30): bit 8 = "codec ready"
    // AC97 powerdown/status (offset 0x26): 0x000F = all sections ready
    match offset {
        0x30 => 0x0000_0100, // Global Status: Codec Ready (bit 8)
        0x26 => 0x0000_000F, // Powerdown: all ready
        _ => 0,
    }
}

/// Write an AC97 register — silently absorbed.
pub fn ac97_shadow_write(_offset: u32, _value: u32) {
    // No state to track for AC97
}

// ---------------------------------------------------------------------------
// Convenience: range check
// ---------------------------------------------------------------------------

/// Check if a guest address is in a southbridge MMIO region.
/// Covers: APU (0xFE800000), AC97 (0xFEC00000), NIC (0xFEF00000).
pub fn is_apu_mmio(guest_addr: u32) -> bool {
    (APU_BASE..=APU_END).contains(&guest_addr)
        || (AC97_BASE..=AC97_END).contains(&guest_addr)
        || (NIC_BASE..=NIC_END).contains(&guest_addr)
}

/// Read the global APU MMIO count.
pub fn global_apu_mmio_count() -> u64 {
    GLOBAL_APU_MMIO_COUNT.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// VEH entry point — called from veh.rs on access violation in APU/AC97 range
// ---------------------------------------------------------------------------

/// Handle an APU/AC97/NIC MMIO access violation.
/// Decodes the faulting x64 instruction, reads or writes the shadow
/// register, and advances RIP.
#[cfg(windows)]
pub fn handle_apu_mmio(
    rip: u64,
    code_end: u64,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_addr: u32,
    is_write: bool,
) -> crate::xbox::aot::veh::VehResult {
    use crate::xbox::aot::veh::veh_log;
    use crate::xbox::aot::veh::VehResult;
    use crate::xbox::aot::veh_mmio::{extract_write_value, set_read_result};
    use iced_x86::{Decoder, DecoderOptions, Instruction};

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

    if (APU_BASE..=APU_END).contains(&guest_addr) {
        let offset = guest_addr - APU_BASE;
        if is_write {
            let value = extract_write_value(&instr, context);
            apu_shadow_write(offset, value);
        } else {
            let value = apu_shadow_read(offset);
            set_read_result(&instr, context, value);
        }
    } else if (AC97_BASE..=AC97_END).contains(&guest_addr) {
        let offset = guest_addr - AC97_BASE;
        if is_write {
            ac97_shadow_write(offset, 0); // value not critical for AC97
        } else {
            let value = ac97_shadow_read(offset);
            set_read_result(&instr, context, value);
        }
    } else {
        // NIC range — reads return 0, writes absorbed
        if !is_write {
            set_read_result(&instr, context, 0);
        }
    }

    let count = GLOBAL_APU_MMIO_COUNT.fetch_add(1, Ordering::Relaxed);
    if count < 10 {
        let offset = guest_addr.wrapping_sub(APU_BASE);
        veh_log(&format!(
            "[APU-MMIO] {} guest=0x{:08X} offset=0x{:06X} len={} (total={})",
            if is_write { "WRITE" } else { "READ" },
            guest_addr,
            offset,
            instr_len,
            count + 1
        ));
    }

    context.Rip = rip + instr_len as u64;
    VehResult::Handled
}
