/// Micro-interpreter for x86 guest code.
/// Inspired by nesrust's cpu.rs — fetch/decode/execute in safe Rust.
/// Used as a fallback for code regions where AOT inline RET has hash
/// collision issues (the P0 batched cdecl zone).
///
/// This interpreter:
/// - Reads x86 instructions from guest memory via iced-x86 decoder
/// - Executes them by updating a simple register file + guest memory
/// - Handles CALL/RET with proper stack semantics (no R14/VEH)
/// - Routes kernel calls (0xFFFF0000+) to the kernel dispatcher
/// - Returns when it hits a RET that returns to the caller's address
use crate::xbox::emulator::debug_log;
use crate::xbox::memory::guest_memory::GuestMemory;

/// Simple x86 register file
#[derive(Debug, Clone)]
pub struct X86Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub eip: u32,
    pub eflags: u32,
    // x87 FPU stack (8 entries, top-of-stack index)
    pub fpu_stack: [f64; 8],
    pub fpu_top: usize,
    // SSE XMM registers (8 registers × 128 bits = 16 bytes each)
    pub xmm: [[u8; 16]; 8],
}

impl X86Regs {
    pub fn new() -> Self {
        Self {
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            eip: 0,
            eflags: 0,
            fpu_stack: [0.0; 8],
            fpu_top: 0,
            xmm: [[0u8; 16]; 8],
        }
    }

    fn get(&self, reg: iced_x86::Register) -> u32 {
        use iced_x86::Register::*;
        match reg {
            EAX => self.eax,
            ECX => self.ecx,
            EDX => self.edx,
            EBX => self.ebx,
            ESP => self.esp,
            EBP => self.ebp,
            ESI => self.esi,
            EDI => self.edi,
            // 8-bit
            AL => self.eax & 0xFF,
            CL => self.ecx & 0xFF,
            DL => self.edx & 0xFF,
            BL => self.ebx & 0xFF,
            AH => (self.eax >> 8) & 0xFF,
            CH => (self.ecx >> 8) & 0xFF,
            DH => (self.edx >> 8) & 0xFF,
            BH => (self.ebx >> 8) & 0xFF,
            // 16-bit
            AX => self.eax & 0xFFFF,
            CX => self.ecx & 0xFFFF,
            DX => self.edx & 0xFFFF,
            BX => self.ebx & 0xFFFF,
            SP => self.esp & 0xFFFF,
            BP => self.ebp & 0xFFFF,
            SI => self.esi & 0xFFFF,
            DI => self.edi & 0xFFFF,
            _ => 0,
        }
    }

    fn set(&mut self, reg: iced_x86::Register, val: u32) {
        use iced_x86::Register::*;
        match reg {
            EAX => self.eax = val,
            ECX => self.ecx = val,
            EDX => self.edx = val,
            EBX => self.ebx = val,
            ESP => self.esp = val,
            EBP => self.ebp = val,
            ESI => self.esi = val,
            EDI => self.edi = val,
            AL => self.eax = (self.eax & 0xFFFFFF00) | (val & 0xFF),
            CL => self.ecx = (self.ecx & 0xFFFFFF00) | (val & 0xFF),
            DL => self.edx = (self.edx & 0xFFFFFF00) | (val & 0xFF),
            BL => self.ebx = (self.ebx & 0xFFFFFF00) | (val & 0xFF),
            AH => self.eax = (self.eax & 0xFFFF00FF) | ((val & 0xFF) << 8),
            CH => self.ecx = (self.ecx & 0xFFFF00FF) | ((val & 0xFF) << 8),
            DH => self.edx = (self.edx & 0xFFFF00FF) | ((val & 0xFF) << 8),
            BH => self.ebx = (self.ebx & 0xFFFF00FF) | ((val & 0xFF) << 8),
            AX => self.eax = (self.eax & 0xFFFF0000) | (val & 0xFFFF),
            CX => self.ecx = (self.ecx & 0xFFFF0000) | (val & 0xFFFF),
            DX => self.edx = (self.edx & 0xFFFF0000) | (val & 0xFFFF),
            BX => self.ebx = (self.ebx & 0xFFFF0000) | (val & 0xFFFF),
            SP => self.esp = (self.esp & 0xFFFF0000) | (val & 0xFFFF),
            BP => self.ebp = (self.ebp & 0xFFFF0000) | (val & 0xFFFF),
            SI => self.esi = (self.esi & 0xFFFF0000) | (val & 0xFFFF),
            DI => self.edi = (self.edi & 0xFFFF0000) | (val & 0xFFFF),
            _ => {}
        }
    }

    fn update_flags_add(&mut self, a: u32, b: u32, result: u32) {
        let zf = if result == 0 { 0x40 } else { 0 };
        let sf = if result & 0x8000_0000 != 0 { 0x80 } else { 0 };
        let cf = if result < a || result < b { 1 } else { 0 }; // carry on overflow
        let of = if ((!(a ^ b)) & (a ^ result)) & 0x8000_0000 != 0 {
            0x800
        } else {
            0
        };
        self.eflags = (self.eflags & !0x8C1) | zf | sf | cf | of;
    }

    fn update_flags_sub(&mut self, a: u32, b: u32, result: u32) {
        let zf = if result == 0 { 0x40 } else { 0 };
        let sf = if result & 0x8000_0000 != 0 { 0x80 } else { 0 };
        let cf = if a < b { 1 } else { 0 };
        // OF: overflow if signs of a and b differ AND result sign differs from a
        let of = if ((a ^ b) & (a ^ result)) & 0x8000_0000 != 0 {
            0x800
        } else {
            0
        };
        self.eflags = (self.eflags & !0x8C1) | zf | sf | cf | of;
    }

    fn update_flags_test(&mut self, result: u32) {
        let zf = if result == 0 { 0x40 } else { 0 };
        let sf = if result & 0x8000_0000 != 0 { 0x80 } else { 0 };
        self.eflags = (self.eflags & !0x8C1) | zf | sf;
    }

    fn flag_zf(&self) -> bool {
        self.eflags & 0x40 != 0
    }
    fn flag_cf(&self) -> bool {
        self.eflags & 0x01 != 0
    }
    fn flag_sf(&self) -> bool {
        self.eflags & 0x80 != 0
    }
    fn flag_of(&self) -> bool {
        self.eflags & 0x800 != 0
    }
}

/// Result of interpreting a block
#[derive(Debug)]
pub enum InterpResult {
    /// Returned to the expected caller address
    ReturnedTo(u32),
    /// Hit a kernel call (ordinal)
    KernelCall { ordinal: u32, ret_addr: u32 },
    /// Hit an OOVPA-hooked function
    OovpaHook { guest_addr: u32, ret_addr: u32 },
    /// Exceeded instruction limit
    Timeout,
    /// Hit unhandled instruction
    Unhandled { eip: u32, mnemonic: String },
    /// Access violation
    AccessViolation { eip: u32, addr: u32 },
}

/// Doom has legitimate long counted init loops that exceed the generic
/// backward-branch spin breaker. These pairs are allowed to keep taking the
/// branch instead of being forced to fall through.
fn preserves_doom_init_loop(eip: u32, target: u32) -> bool {
    matches!(
        (eip, target),
        (0x0001_9CE0, 0x0001_9C70)
            | (0x0001_9D1B, 0x0001_9D12)
            | (0x0001_9D3B, 0x0001_9D00)
            | (0x0001_9D90, 0x0001_9D50)
            | (0x0002_EEA7, 0x0002_EE30)
    )
}

fn taken_backward_jump_is_generic_spin_candidate(eip: u32, target: u32) -> bool {
    target < eip && !preserves_doom_init_loop(eip, target)
}

fn spidey_fun29c5a0_probe_enabled() -> bool {
    std::env::var("RUSTEMU_SPIDEY_FUN29C5A0_PROBE")
        .or_else(|_| std::env::var("RUSTEMU_SPIDEY_FUN29C5A0_SVSC_CANARY"))
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn spidey_fun29c5a0_probe_name(eip: u32) -> Option<&'static str> {
    match eip {
        0x002A_0B1B => Some("PARENT_CALLSITE"),
        0x0029_CB70 => Some("PARENT_ENTRY"),
        0x0029_CB93 => Some("PARENT_C5A0_CALL"),
        0x0029_CB98 => Some("PARENT_C5A0_RET"),
        0x0029_C5A0 => Some("ENTRY"),
        0x0029_C734 => Some("C96_CALL"),
        0x0029_C739 => Some("C96_RET"),
        0x0029_C7C3 => Some("C95_CALL"),
        0x0029_C7C8 => Some("C95_RET"),
        0x0029_C839 => Some("C94_CALL"),
        0x0029_C83E => Some("C94_RET"),
        0x0029_C86C => Some("C93_CALL"),
        0x0029_C871 => Some("C93_RET"),
        _ => None,
    }
}

fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn spidey_xbs_script_probe_enabled() -> bool {
    env_truthy("RUSTEMU_SPIDEY_XBS_SCRIPT_PROBE")
}

fn valid_spidey_guest_ptr(ptr: u32) -> bool {
    (0x1000..0x2000_0000).contains(&ptr)
}

fn spidey_xbs_script_probe(memory: &GuestMemory, regs: &X86Regs, eip: u32) {
    let label = match eip {
        0x0005_1C00 => "51C00_ENTRY",
        0x0005_1C40 => "51C00_LOOP_FETCH",
        0x0005_1C54 => "51C00_LOOP_DECODE",
        0x0005_1C80 => "51C00_BRANCH",
        0x0005_1CC0 => "51C00_SWITCH",
        0x0005_2680 => "51C00_SCRIPT_HOT",
        0x0005_2B26 => "OP28_EVENT_LOCAL",
        0x0005_2B52 => "OP29_EVENT_GLOBAL",
        0x0005_2B78 => "OP2A_REGISTER_LOCAL",
        0x0005_2B88 => "OP2B_REGISTER_GLOBAL",
        0x0005_2B91 => "OP2C_REGISTER_LOCAL_ONCE",
        0x0005_2BA1 => "OP2D_REGISTER_GLOBAL_ONCE",
        0x0004_6480 => "event_local_entry",
        0x0004_6530 => "event_global_entry",
        0x0005_1600 => "register_local_wrapper",
        0x0005_0F30 => "register_global_wrapper",
        0x0004_6100 => "listener_register",
        _ => return,
    };

    static SCRIPT_XBS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    static SCRIPT_XBS_LOOP_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    let script = if eip == 0x0005_1C00 {
        regs.ecx
    } else {
        regs.ebp
    };
    let script_pc = if valid_spidey_guest_ptr(script) {
        memory.read_u32(script.wrapping_add(0x1C))
    } else {
        0
    };
    let op_word = if valid_spidey_guest_ptr(script_pc) {
        memory.read_u16(script_pc)
    } else {
        0
    };
    let major = op_word >> 8;
    let op = op_word & 0x7F;
    let stack14 = if valid_spidey_guest_ptr(script) {
        memory.read_u32(script.wrapping_add(0x14))
    } else {
        0
    };
    let base18 = if valid_spidey_guest_ptr(script) {
        memory.read_u32(script.wrapping_add(0x18))
    } else {
        0
    };
    let ctr44 = if valid_spidey_guest_ptr(script) {
        memory.read_u32(script.wrapping_add(0x44))
    } else {
        0
    };
    let timer40 = if valid_spidey_guest_ptr(script) {
        memory.read_u32(script.wrapping_add(0x40))
    } else {
        0
    };

    let loop_like = matches!(
        eip,
        0x0005_1C40 | 0x0005_1C54 | 0x0005_1C80 | 0x0005_1CC0 | 0x0005_2680
    );
    if loop_like {
        let n = SCRIPT_XBS_LOOP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n >= 192 && !n.is_power_of_two() && !(0x28..=0x2D).contains(&major) {
            return;
        }
    } else {
        let n = SCRIPT_XBS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n >= 256 && !n.is_power_of_two() {
            return;
        }
    }

    let ret = memory.read_u32(regs.esp);
    let arg0 = memory.read_u32(regs.esp.wrapping_add(4));
    let arg1 = memory.read_u32(regs.esp.wrapping_add(8));
    let arg2 = memory.read_u32(regs.esp.wrapping_add(0x0C));
    let arg3 = memory.read_u32(regs.esp.wrapping_add(0x10));
    let listener_slot = if eip == 0x0004_6100 { arg3 & 0xFF } else { 0 };

    debug_log(&format!(
        "[SPIDEY-XBS-INTERP] {} eip=0x{:08X} script=0x{:08X} pc=0x{:08X} word=0x{:04X} major=0x{:02X} op=0x{:02X} stack14=0x{:08X} base18=0x{:08X} ctr44={} timer40=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esi=0x{:08X} edi=0x{:08X} ebp=0x{:08X} esp=0x{:08X} ret=0x{:08X} args=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] slot_arg=0x{:02X}",
        label,
        eip,
        script,
        script_pc,
        op_word,
        major,
        op,
        stack14,
        base18,
        ctr44,
        timer40,
        regs.eax,
        regs.ecx,
        regs.edx,
        regs.ebx,
        regs.esi,
        regs.edi,
        regs.ebp,
        regs.esp,
        ret,
        arg0,
        arg1,
        arg2,
        arg3,
        listener_slot
    ));
}

/// Interpret x86 guest code starting at `entry` until it returns to `expected_ret`.
/// `memory` provides guest RAM access. `regs` holds the initial register state.
/// `exec_ranges` lists (start, end) of executable sections; EIP outside all ranges
/// triggers a simulated RET to prevent executing data bytes as code.
/// Returns when RET pops `expected_ret` from the stack, or after `max_insns` instructions.
pub fn interpret(
    memory: &GuestMemory,
    regs: &mut X86Regs,
    entry: u32,
    expected_ret: u32,
    max_insns: u64,
    oovpa_addrs: &[u32],
    exec_ranges: &[(u32, u32)],
) -> InterpResult {
    regs.eip = entry;
    crate::xbox::aot::bink_profile::note_interp_entry(entry);
    crate::xbox::aot::snapshot_probe::on_entry(regs, memory, entry);
    macro_rules! return_with_snapshot {
        ($result:expr, $trapped:expr, $trap_pc:expr) => {{
            let result = $result;
            crate::xbox::aot::snapshot_probe::on_exit(regs, memory, $trapped, $trap_pc);
            return result;
        }};
    }
    let mut count: u64 = 0;
    // Spin detector: break tight loops (NV2A register polling, delay loops).
    // Track backward jumps, but preserve known finite Doom renderer setup loops.
    let mut last_backward_target: u32 = 0;
    let mut backward_count: u32 = 0;
    let mut last_eip: u32 = 0;
    let mut spin_count: u32 = 0;

    loop {
        // Spin detection: if same EIP seen 2000+ times, force exit the loop
        if regs.eip == last_eip {
            spin_count += 1;
            if spin_count >= 2000 {
                // Skip the backward jump by advancing past it
                // The backward jump is typically 2 bytes (short) or 6 bytes (near)
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        (memory.base() as u64 + regs.eip as u64) as *const u8,
                        8,
                    )
                };
                // Decode the instruction at EIP to find its length
                let mut dec = iced_x86::Decoder::with_ip(
                    32,
                    bytes,
                    regs.eip as u64,
                    iced_x86::DecoderOptions::NONE,
                );
                if dec.can_decode() {
                    let instr = dec.decode();
                    // If it's a backward conditional jump, skip past it
                    if instr.is_jcc_short_or_near() || instr.mnemonic() == iced_x86::Mnemonic::Jmp {
                        static SPIN_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let n = SPIN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if n < 20 {
                            crate::xbox::emulator::debug_log(&format!(
                                "[SPIN-BREAK] #{} at 0x{:08X}: {} — forced fall-through after {} iterations",
                                n, regs.eip, instr, spin_count
                            ));
                        }
                        regs.eip += instr.len() as u32; // skip past the jump
                        spin_count = 0;
                        last_eip = 0;
                        continue;
                    }
                }
                spin_count = 0;
            }
        } else {
            last_eip = regs.eip;
            spin_count = 0;
        }

        if count >= max_insns {
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("./interp.log")
            {
                use std::io::Write;
                let _ = writeln!(
                    f,
                    "[INTERP] Timeout at 0x{:08X} after {} insns ESP=0x{:08X}",
                    regs.eip, count, regs.esp
                );
            }
            return_with_snapshot!(InterpResult::Timeout, 1, regs.eip);
        }
        // Periodic progress log every 100K instructions
        if count % 100_000 == 0 && count > 0 {
            static PROG_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = PROG_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 50 {
                debug_log(&format!(
                    "[INTERP] {}K insns EIP=0x{:08X} ESP=0x{:08X}",
                    count / 1000,
                    regs.eip,
                    regs.esp
                ));
            }
        }
        count += 1;

        let eip = regs.eip;

        if spidey_xbs_script_probe_enabled() {
            spidey_xbs_script_probe(memory, regs, eip);
        }

        if spidey_fun29c5a0_probe_enabled() {
            if let Some(label) = spidey_fun29c5a0_probe_name(eip) {
                static FUN29C5A0_INTERP_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = FUN29C5A0_INTERP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 64 || n.is_power_of_two() {
                    let esp0 = memory.read_u32(regs.esp);
                    let esp4 = memory.read_u32(regs.esp.wrapping_add(4));
                    let esp8 = memory.read_u32(regs.esp.wrapping_add(8));
                    let esp_c = memory.read_u32(regs.esp.wrapping_add(0x0C));
                    let b0 = memory.read_u8(eip);
                    let b1 = memory.read_u8(eip.wrapping_add(1));
                    let b2 = memory.read_u8(eip.wrapping_add(2));
                    let b3 = memory.read_u8(eip.wrapping_add(3));
                    let b4 = memory.read_u8(eip.wrapping_add(4));
                    let call_target = if b0 == 0xE8 {
                        let rel = (b1 as u32)
                            | ((b2 as u32) << 8)
                            | ((b3 as u32) << 16)
                            | ((b4 as u32) << 24);
                        eip.wrapping_add(5).wrapping_add(rel)
                    } else {
                        0
                    };
                    let c_m96 = memory.read_u32(0x005F_2C68);
                    let c_m95 = memory.read_u32(0x006F_2C98);
                    let c_m94 = memory.read_u32(0x004C_E0F0);
                    let c_m93 = memory.read_u32(0x004D_2AA8);
                    let bone_base = memory.read_u32(0x004D_2ABC);
                    let scratch = [
                        memory.read_u32(0x004D_1520),
                        memory.read_u32(0x004D_1524),
                        memory.read_u32(0x004D_1528),
                        memory.read_u32(0x004D_152C),
                    ];
                    debug_log(&format!(
                        "[SPIDEY-FUN29C5A0-INTERP] #{} {} eip=0x{:08X} esp=0x{:08X} stack=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] regs eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X} bytes=[{:02X} {:02X} {:02X} {:02X} {:02X}] call_target=0x{:08X} globals m96=0x{:08X} m95=0x{:08X} m94=0x{:08X} m93=0x{:08X} bone_base=0x{:08X} scratch=[0x{:08X},0x{:08X},0x{:08X},0x{:08X}] scratch_f=[{:.6},{:.6},{:.6},{:.6}]",
                        n,
                        label,
                        eip,
                        regs.esp,
                        esp0,
                        esp4,
                        esp8,
                        esp_c,
                        regs.eax,
                        regs.ecx,
                        regs.edx,
                        regs.esi,
                        regs.edi,
                        b0,
                        b1,
                        b2,
                        b3,
                        b4,
                        call_target,
                        c_m96,
                        c_m95,
                        c_m94,
                        c_m93,
                        bone_base,
                        scratch[0],
                        scratch[1],
                        scratch[2],
                        scratch[3],
                        f32::from_bits(scratch[0]),
                        f32::from_bits(scratch[1]),
                        f32::from_bits(scratch[2]),
                        f32::from_bits(scratch[3]),
                    ));
                }
            }
        }

        // Trace the strcpy infinite loop caller
        if eip == 0x451A3 {
            static STRCPY_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = STRCPY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 5 {
                // pop edi reads [ESP], ret reads [ESP+4]
                let ret_addr = memory.read_u32(regs.esp.wrapping_add(4));
                let caller_esp = regs.esp.wrapping_add(8); // ESP after pop+ret
                debug_log(&format!(
                    "[STRCPY-TRACE] #{} ret_addr=0x{:08X} ESP=0x{:08X} caller_ESP=0x{:08X} EDI=0x{:08X} ESI=0x{:08X} EBX=0x{:08X}",
                    n, ret_addr, regs.esp, caller_esp, regs.edi, regs.esi, regs.ebx
                ));
            }
        }

        // I_GetTime (0x1B500): reads [gs+0x3968] which is never updated in the
        // game loop. Increment it before each read so TryRunTics sees advancing
        // ticks. Only after D_DoomMain init is done (gs cached = game started).
        if eip == 0x1B500 {
            let gs = crate::xbox::worker::DOOM_GAME_STATE_CACHE
                .load(std::sync::atomic::Ordering::Relaxed);
            if gs >= 0x0100_0000 && gs < 0x1000_0000 {
                let old_tick = memory.read_u32(gs.wrapping_add(0x3968));
                memory.write_u32(gs.wrapping_add(0x3968), old_tick.wrapping_add(1));
            }
        }

        // D_CheckNetGame + ticdup tracking
        if eip == 0x2CFA0 {
            // D_CheckNetGame entry
            let gs = memory.read_u32(0x101BB8);
            let argc = if gs > 0 && gs < 0x2000_0000 {
                memory.read_u32(gs.wrapping_add(0x3D78))
            } else {
                0xDEAD
            };
            debug_log(&format!(
                "[TICDUP-TRACE] D_CheckNetGame ENTERED at 0x2CFA0 gs=0x{:08X} argc={} ESP=0x{:08X}",
                gs, argc, regs.esp
            ));
        }
        if eip == 0x2D036 {
            // mov word ptr [ecx + 0x1272], di — ticdup=1 write
            debug_log(&format!(
                "[TICDUP-TRACE] ticdup WRITE at 0x2D036 ecx=0x{:08X} di=0x{:04X} ESP=0x{:08X}",
                regs.ecx,
                regs.edi & 0xFFFF,
                regs.esp
            ));
        }
        if eip == 0x19202 {
            // call D_CheckNetGame from D_DoomMain
            debug_log(&format!(
                "[TICDUP-TRACE] D_DoomMain about to call D_CheckNetGame at 0x19202 ESP=0x{:08X}",
                regs.esp
            ));
        }
        // 0x192AA: movsx ecx, word ptr [eax+0x1272] — copy ticdup from net struct
        // 0x192B1: mov dword ptr [eax+0x1F50], ecx  — store to game loop field
        if eip == 0x192AA || eip == 0x192B1 {
            let gs = regs.eax; // eax = doom globals base at this point
            let ticdup_src = if gs > 0 && gs < 0x2000_0000 {
                memory.read_u16(gs.wrapping_add(0x1272))
            } else {
                0xBEEF
            };
            let ticdup_dst = if gs > 0 && gs < 0x2000_0000 {
                memory.read_u32(gs.wrapping_add(0x1F50))
            } else {
                0xDEADBEEF
            };
            debug_log(&format!(
                "[TICDUP-COPY] 0x{:05X} EAX(gs)=0x{:08X} ECX=0x{:08X} [gs+0x1272]={} [gs+0x1F50]={} ESP=0x{:08X}",
                eip, gs, regs.ecx, ticdup_src, ticdup_dst, regs.esp
            ));
        }
        // div-by-zero sites: idiv dword ptr [ecx+0x1F50]
        if eip == 0x18C41 || eip == 0x19461 || eip == 0x1950F || eip == 0x19688 {
            let gs = regs.ecx;
            let divisor = if gs > 0 && gs < 0x2000_0000 {
                memory.read_u32(gs.wrapping_add(0x1F50))
            } else {
                0xDEADBEEF
            };
            let ticdup_src = if gs > 0 && gs < 0x2000_0000 {
                memory.read_u16(gs.wrapping_add(0x1272))
            } else {
                0xBEEF
            };
            debug_log(&format!(
                "[TICDUP-DIV] 0x{:05X} ECX(gs)=0x{:08X} [gs+0x1F50]={} [gs+0x1272]={} EAX=0x{:08X} EDX=0x{:08X} ESP=0x{:08X}",
                eip, gs, divisor, ticdup_src, regs.eax, regs.edx, regs.esp
            ));
        }

        // CRT heap free list loop diagnostic (0x43972-0x43989)
        if eip == 0x43970 {
            static HEAP_DIAG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let hd = HEAP_DIAG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if hd < 10 {
                let ecx = regs.ecx;
                let first = if ecx > 0 && ecx < 0x2000_0000 {
                    memory.read_u32(ecx)
                } else {
                    0xDEAD
                };
                // Walk up to 5 nodes to see list structure
                let mut nodes = Vec::new();
                let mut ptr = first;
                for _ in 0..5 {
                    if ptr == 0 || ptr == ecx || ptr >= 0x2000_0000 {
                        break;
                    }
                    let node_data = ptr.wrapping_sub(8);
                    let node_size = if node_data < 0x2000_0000 {
                        memory.read_u16(node_data)
                    } else {
                        0xBEEF
                    };
                    nodes.push(format!("0x{:08X}(sz={})", ptr, node_size));
                    ptr = if ptr < 0x2000_0000 {
                        memory.read_u32(ptr)
                    } else {
                        0
                    };
                }
                debug_log(&format!(
                    "[HEAP-DIAG] #{} DX=0x{:04X} ECX(head)=0x{:08X} first=0x{:08X} list=[{}]",
                    hd,
                    regs.edx & 0xFFFF,
                    ecx,
                    first,
                    nodes.join(" → ")
                ));
            }
        }

        // D_DoomMain + R_Init milestones (temporary tracing)
        if eip == 0x126AD || eip == 0x126B3 || eip == 0x126B8 || eip == 0x12706
            || eip == 0x115E0 || eip == 0x11947 || eip == 0x11950 || eip == 0x119A8
            || eip == 0x119FB || eip == 0x11A02
            || eip == 0x50AD0 || eip == 0x54CC0 // CreateSurface, D3D funcs
            || eip == 0x11652
        // after CreateDevice
        {
            static MILE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let m = MILE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if m < 50 {
                let b0 = memory.read_u8(eip);
                let b1 = memory.read_u8(eip + 1);
                let b2 = memory.read_u8(eip + 2);
                let b3 = memory.read_u8(eip + 3);
                let b4 = memory.read_u8(eip + 4);
                debug_log(&format!(
                    "[DOOM-MILE] 0x{:08X} ESP=0x{:08X} EAX=0x{:08X} ECX=0x{:08X} EBX=0x{:08X} [ESP]=0x{:08X} bytes=[{:02X} {:02X} {:02X} {:02X} {:02X}]",
                    eip, regs.esp, regs.eax, regs.ecx, regs.ebx,
                    memory.read_u32(regs.esp),
                    b0, b1, b2, b3, b4
                ));
            }
        }

        // CRT _mtinitlocknum diagnostic: trace lock init flow
        // 0x47F0B = first NULL check (cmp [esi], ebx)
        // 0x47F11 = malloc(0x1C) for critsec
        // 0x47F2C = _lock(10) call
        // 0x47F3F = InitCSAndSpin wrapper call
        // 0x47F46 = test eax, eax (check wrapper result)
        // 0x47F6D = mov [esi], edi (store to lock_table)
        // 0x47F93 = _lock(n) entry
        if eip == 0x47F0B
            || eip == 0x47F46
            || eip == 0x47F6D
            || eip == 0x47F93
            || eip == 0x47F11
            || eip == 0x47F3F
            || eip == 0x47F2C
        {
            static LOCK_DIAG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let ld = LOCK_DIAG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if ld < 80 {
                let lock10 = memory.read_u32(0x000D_FA50); // lock_table[10]
                let lock18 = memory.read_u32(0x000D_FA90); // lock_table[18]
                debug_log(&format!(
                    "[LOCK-DIAG] #{} EIP=0x{:05X} EAX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X} [ESI]=0x{:08X} ESP=0x{:08X} EBP=0x{:08X} lock10=0x{:08X} lock18=0x{:08X}",
                    ld, eip, regs.eax, regs.esi, regs.edi,
                    if regs.esi > 0 && regs.esi < 0x2000_0000 { memory.read_u32(regs.esi) } else { 0xDEAD },
                    regs.esp, regs.ebp, lock10, lock18
                ));
            }
        }

        // Sanity check: if EIP is 0, it's a RET to sentinel
        if eip == 0 {
            return_with_snapshot!(InterpResult::ReturnedTo(0), 0, 0);
        }

        // Check for kernel thunk FIRST (before null-bytes check, because
        // kernel addresses 0xFFFF0000+ read as null bytes in guest memory)
        if eip >= 0xFFFF_0000 {
            let ordinal = eip & 0xFFFF;
            return_with_snapshot!(
                InterpResult::KernelCall {
                    ordinal,
                    ret_addr: memory.read_u32(regs.esp),
                },
                1,
                eip
            );
        }

        // Also check 0x80000000|ordinal (mirror thunk format)
        if eip >= 0x8000_0000 && eip < 0x8000_FFFF {
            let ordinal = eip & 0xFFFF;
            return_with_snapshot!(
                InterpResult::KernelCall {
                    ordinal,
                    ret_addr: memory.read_u32(regs.esp),
                },
                1,
                eip
            );
        }

        // Check for OOVPA hooks
        if oovpa_addrs.contains(&eip) {
            if crate::xbox::aot::oovpa::is_tap_hook(eip) {
                static TAP_PASS_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = TAP_PASS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 40 {
                    debug_log(&format!(
                        "[INTERP-TAP] pass-through at 0x{:08X} ESP=0x{:08X}",
                        eip, regs.esp
                    ));
                }
                if let Some(name) = match eip {
                    0x0001_9C40 => Some("DOOM_WALL_VIEWMAP_ENTRY_TAP"),
                    0x0001_9E30 => Some("DOOM_WALL_SETVIEW_ENTRY_TAP"),
                    0x0001_9E60 => Some("DOOM_WALL_EXEC_SETVIEW_ENTRY_TAP"),
                    0x0001_A310 => Some("DOOM_WALL_RENDER_INIT_TAP"),
                    0x0001_9CDD => Some("DOOM_WALL_VIEWMAP_BUILD_STORE_TAP"),
                    0x0001_9D60 => Some("DOOM_WALL_VIEWMAP_CLAMP_INPUT_TAP"),
                    0x0001_9D87 => Some("DOOM_WALL_VIEWMAP_CLAMP_FINAL_TAP"),
                    _ => None,
                } {
                    crate::xbox::aot::oovpa::oovpa_hle::doom_wall_viewmap_tap(
                        name,
                        memory.base(),
                        regs.eax,
                        regs.ecx,
                        regs.edx,
                        regs.ebx,
                        regs.esi,
                        regs.edi,
                        regs.esp,
                    );
                }
                if matches!(eip, 0x0001_2C10 | 0x0001_29F0 | 0x0001_2A04) {
                    static DOOM_TAP_LOG: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let dn = DOOM_TAP_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if dn < 32 || dn.is_power_of_two() {
                        let name = match eip {
                            0x0001_2C10 => "OUTER",
                            0x0001_29F0 => "INNER",
                            0x0001_2A04 => "ALLOC_RET",
                            _ => "UNKNOWN",
                        };
                        let ret = memory.read_u32(regs.esp);
                        let arg0 = memory.read_u32(regs.esp.wrapping_add(4));
                        let arg1 = memory.read_u32(regs.esp.wrapping_add(8));
                        let init_flag = memory.read_u32(0x0010_1BB4);
                        let curr_slot = memory.read_u32(0x0010_1B9C);
                        let gamestate = memory.read_u32(0x0010_1BB8);
                        let gs_array0 = memory.read_u32(0x0010_1BA4);
                        debug_log(&format!(
                            "[DOOM-PROBE-INTERP #{}] {} eip=0x{:08X} ret=0x{:08X} esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edi=0x{:08X} arg0=0x{:08X} arg1=0x{:08X} gamestate=0x{:08X} init_flag=0x{:08X} curr_slot=0x{:08X} gs_array[0]=0x{:08X}",
                            dn,
                            name,
                            eip,
                            ret,
                            regs.esp,
                            regs.eax,
                            regs.ecx,
                            regs.edi,
                            arg0,
                            arg1,
                            gamestate,
                            init_flag,
                            curr_slot,
                            gs_array0
                        ));
                    }
                }
            } else {
                return_with_snapshot!(
                    InterpResult::OovpaHook {
                        guest_addr: eip,
                        ret_addr: memory.read_u32(regs.esp),
                    },
                    1,
                    eip
                );
            }
        }

        // DSOUND section skip: DSOUND does APU hardware init with infinite polling loops.
        // Without real APU hardware, these loops spin forever. Skip all DSOUND code.
        // Scan the function body for `ret N` (C2 nn nn) to determine stdcall cleanup,
        // then simulate the return inline instead of yielding as an unknown OOVPA hook.
        if eip >= 0x0008_2460 && eip < 0x0008_B444 {
            // If this address is a known OOVPA hook, yield to the handler (it knows the cleanup)
            if oovpa_addrs.contains(&eip) {
                return_with_snapshot!(
                    InterpResult::OovpaHook {
                        guest_addr: eip,
                        ret_addr: memory.read_u32(regs.esp),
                    },
                    1,
                    eip
                );
            }
            // Unknown DSOUND internal function — scan for ret N to find stdcall cleanup
            let mut cleanup: u32 = 0;
            for off in 0..256u32 {
                let addr = eip.wrapping_add(off);
                if addr >= 0x0008_B444 {
                    break;
                }
                let b = memory.read_u8(addr);
                if b == 0xC2 {
                    // ret imm16 — stdcall cleanup
                    cleanup =
                        memory.read_u8(addr + 1) as u32 | ((memory.read_u8(addr + 2) as u32) << 8);
                    break;
                }
                if b == 0xC3 {
                    // ret — no cleanup (cdecl or 0-arg)
                    break;
                }
            }
            // Simulate the return: pop ret addr, clean up stdcall args, set EAX=0
            let ret_addr = memory.read_u32(regs.esp);
            regs.esp = regs.esp.wrapping_add(4 + cleanup);
            regs.eax = 0;
            regs.eip = ret_addr;
            count += 1;
            continue;
        }

        // Null-bytes / bump allocator address — simulate RET (return 0 to caller)
        let first_byte = memory.read_u8(eip);
        if (first_byte == 0x00
            && memory.read_u8(eip + 1) == 0x00
            && memory.read_u8(eip + 2) == 0x00
            && memory.read_u8(eip + 3) == 0x00)
            || (eip >= 0x02000000 && eip < 0x10000000)
        // bump allocator region (above 32MB)
        {
            // Jumped into uninitialized memory (null vtable or bump allocator).
            // Simulate a bare RET back to the caller: pop [ESP] → EIP, EAX = 0
            let ret_addr = memory.read_u32(regs.esp);
            regs.esp = regs.esp.wrapping_add(4);
            regs.eax = 0;
            regs.eip = ret_addr;
            continue; // re-enter the main loop at the return address
        }

        // Non-executable section guard: if EIP is in a data section (e.g. .rdata/.data),
        // don't execute random data bytes. Scan stack for the nearest valid code address
        // to unwind to, preventing cascading through garbage addresses.
        if !exec_ranges.is_empty() && eip < 0x0100_0000 {
            let in_code = exec_ranges
                .iter()
                .any(|&(start, end)| eip >= start && eip < end);
            if !in_code {
                // Scan stack for nearest valid code return address
                let mut recovered = false;
                for slot in 0..32u32 {
                    let addr = regs.esp.wrapping_add(slot * 4);
                    if addr >= 0x1F00_0000 {
                        break;
                    }
                    let candidate = memory.read_u32(addr);
                    if candidate >= 0x10000 && candidate < 0x0100_0000 {
                        let valid = exec_ranges
                            .iter()
                            .any(|&(s, e)| candidate >= s && candidate < e);
                        if valid {
                            static NON_EXEC_LOG: std::sync::atomic::AtomicU32 =
                                std::sync::atomic::AtomicU32::new(0);
                            let n = NON_EXEC_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n < 20 {
                                debug_log(&format!(
                                    "[INTERP] Non-exec EIP 0x{:08X} → stack scan found 0x{:08X} at ESP+{} (ESP=0x{:08X})",
                                    eip, candidate, slot * 4, regs.esp
                                ));
                            }
                            regs.esp = addr.wrapping_add(4);
                            regs.eax = 0;
                            regs.eip = candidate;
                            recovered = true;
                            break;
                        }
                    }
                }
                if !recovered {
                    // No valid code address found — simulate bare RET
                    let ret_addr = memory.read_u32(regs.esp);
                    regs.esp = regs.esp.wrapping_add(4);
                    regs.eax = 0;
                    regs.eip = ret_addr;
                    static NON_EXEC_FAIL: std::sync::atomic::AtomicU32 =
                        std::sync::atomic::AtomicU32::new(0);
                    let n = NON_EXEC_FAIL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if n < 10 {
                        debug_log(&format!(
                            "[INTERP] Non-exec EIP 0x{:08X} → no valid code found, bare RET to 0x{:08X}",
                            eip, ret_addr
                        ));
                    }
                }
                continue;
            }
        }

        // Read up to 15 bytes at EIP
        let mut code = [0u8; 15];
        for i in 0..15u32 {
            code[i as usize] = memory.read_u8(eip + i);
        }

        // Decode one instruction
        let mut decoder =
            iced_x86::Decoder::with_ip(32, &code, eip as u64, iced_x86::DecoderOptions::NONE);
        if !decoder.can_decode() {
            debug_log(&format!(
                "[INTERP-DECODE-FAIL] can't decode at 0x{:08X} bytes=[{:02X} {:02X} {:02X} {:02X}]",
                eip, code[0], code[1], code[2], code[3]
            ));
            return_with_snapshot!(
                InterpResult::Unhandled {
                    eip,
                    mnemonic: "decode_fail".into(),
                },
                1,
                eip
            );
        }
        let instr = decoder.decode();
        if instr.is_invalid() {
            debug_log(&format!(
                "[INTERP-DECODE-INVALID] at 0x{:08X} bytes=[{:02X} {:02X} {:02X} {:02X} {:02X} {:02X}]",
                eip, code[0], code[1], code[2], code[3], code[4], code[5]
            ));
            return_with_snapshot!(
                InterpResult::Unhandled {
                    eip,
                    mnemonic: "INVALID".into(),
                },
                1,
                eip
            );
        }
        let len = instr.len() as u32;

        use iced_x86::Mnemonic::*;
        use iced_x86::OpKind;

        match instr.mnemonic() {
            // --- Data movement ---
            Mov => {
                let val = eval_operand(memory, regs, &instr, 1);
                // Log FS: segment writes (SEH chain modifications)
                if instr.segment_prefix() == iced_x86::Register::FS {
                    // Log ALL FS writes that target 0x0C000000 (SEH chain)
                    let dst_mem = instr.op0_kind() == iced_x86::OpKind::Memory;
                    let src_mem = instr.op1_kind() == iced_x86::OpKind::Memory;
                    let mem_addr = if dst_mem || src_mem {
                        eval_mem_addr(regs, &instr, if dst_mem { 0 } else { 1 })
                    } else {
                        0
                    };
                    // Only log first 20 writes to 0x0C000000 (fs:[0] = SEH chain head)
                    if dst_mem && (mem_addr == 0x0C00_0000 || mem_addr == 0) {
                        static FS_LOG: std::sync::atomic::AtomicU32 =
                            std::sync::atomic::AtomicU32::new(0);
                        let fl = FS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if fl < 20 {
                            crate::xbox::emulator::debug_log(&format!(
                                "[INTERP-FS] EIP=0x{:08X} MOV fs:[0]@{:08X} = 0x{:08X} ESP=0x{:08X}",
                                regs.eip, mem_addr, val, regs.esp
                            ));
                        }
                    }
                }
                store_operand(memory, regs, &instr, 0, val);
                regs.eip += len;
            }
            Movzx => {
                let val = eval_operand(memory, regs, &instr, 1)
                    & match instr.memory_size() {
                        iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => 0xFF,
                        iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => 0xFFFF,
                        _ => 0xFFFFFFFF,
                    };
                store_operand(memory, regs, &instr, 0, val);
                regs.eip += len;
            }
            Movsx => {
                let raw = eval_operand(memory, regs, &instr, 1);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int8 | iced_x86::MemorySize::UInt8 => {
                        (raw as u8 as i8 as i32) as u32
                    }
                    iced_x86::MemorySize::Int16 | iced_x86::MemorySize::UInt16 => {
                        (raw as u16 as i16 as i32) as u32
                    }
                    _ => raw,
                };
                store_operand(memory, regs, &instr, 0, val);
                regs.eip += len;
            }
            Lea => {
                let addr = eval_mem_addr(regs, &instr, 1);
                store_operand(memory, regs, &instr, 0, addr);
                regs.eip += len;
            }
            Xor => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let result = a ^ b;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            And => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let result = a & b;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            Or => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let result = a | b;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            Sbb => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let carry_in = if regs.flag_cf() { 1u32 } else { 0 };
                let result = a.wrapping_sub(b).wrapping_sub(carry_in);
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_sub(a, b.wrapping_add(carry_in), result);
                regs.eip += len;
            }
            Add | Adc => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let carry_in = if instr.mnemonic() == Adc && regs.flag_cf() {
                    1u32
                } else {
                    0
                };
                let result = a.wrapping_add(b).wrapping_add(carry_in);
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_add(a, b, result);
                regs.eip += len;
            }
            Sub => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let result = a.wrapping_sub(b);
                store_operand(memory, regs, &instr, 0, result);
                let r = a.wrapping_sub(b);
                regs.update_flags_sub(a, b, r);
                regs.eip += len;
            }
            Cmp => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                let r = a.wrapping_sub(b);
                regs.update_flags_sub(a, b, r);
                regs.eip += len;
            }
            Test => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                regs.update_flags_test(a & b);
                regs.eip += len;
            }
            Inc => {
                let val = eval_operand(memory, regs, &instr, 0);
                let result = val.wrapping_add(1);
                store_operand(memory, regs, &instr, 0, result);
                // INC preserves CF, only updates ZF/SF/OF
                let old_cf = regs.flag_cf();
                regs.update_flags_add(val, 1, result);
                if old_cf {
                    regs.eflags |= 1;
                } else {
                    regs.eflags &= !1;
                }
                regs.eip += len;
            }
            Dec => {
                let val = eval_operand(memory, regs, &instr, 0);
                let result = val.wrapping_sub(1);
                store_operand(memory, regs, &instr, 0, result);
                // DEC preserves CF, only updates ZF/SF/OF
                let old_cf = regs.flag_cf();
                regs.update_flags_sub(val, 1, result);
                if old_cf {
                    regs.eflags |= 1;
                } else {
                    regs.eflags &= !1;
                }
                regs.eip += len;
            }
            Shl => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 0x1F;
                let result = val << shift;
                store_operand(memory, regs, &instr, 0, result);
                regs.eip += len;
            }
            Shr => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 0x1F;
                let result = val >> shift;
                store_operand(memory, regs, &instr, 0, result);
                regs.eip += len;
            }
            Nop => {
                regs.eip += len;
            }

            // --- Stack ---
            Push => {
                let val = eval_operand(memory, regs, &instr, 0);
                regs.esp = regs.esp.wrapping_sub(4);
                memory.write_u32(regs.esp, val);
                regs.eip += len;
            }
            Pop => {
                let val = memory.read_u32(regs.esp);
                regs.esp = regs.esp.wrapping_add(4);
                store_operand(memory, regs, &instr, 0, val);
                regs.eip += len;
            }

            // --- Control flow ---
            Call => {
                let target = if instr.op0_kind() == OpKind::NearBranch32 {
                    instr.near_branch32()
                } else {
                    eval_operand(memory, regs, &instr, 0)
                };
                // HLE: skip known CRT boot functions that take 50M+ instructions.
                // Instead of calling them, just advance EIP past the call.
                // These functions set up drives and CRT state — we do that in Rust.
                let is_hle_skip = match target {
                    // _initterm (C++ static constructors) — was previously skipped but
                    // _ioinit is needed for CRT file I/O (open/read/lseek/close).
                    // Without _ioinit, fopen/open return -1 → WAD loading fails.
                    // Lock #10 pre-init + fclose HLE compensate for _mtinit skip.
                    // t if t >= 0x00040E00 && t <= 0x00040EFF => true,  // REMOVED
                    // XapiInitProcess-internal heavy functions (partition enum)
                    t if t >= 0x00040500 && t <= 0x00040510 => true,
                    // NOTE: 0x43100-0x43200 (SEH D3D pre-init) was previously HLE-skipped here.
                    // Removed: EAX=0 return caused XapiInitProcess to take error path →
                    // XWriteTitleInfoAndReboot → HalReturnToFirmware(2) infinite loop.
                    // The function must either run or return a success value.
                    // XGRPH shader compiler (Doom): crashes deep in shader bytecode parse.
                    // HLE D3D doesn't need the shader compiler to run.
                    t if t >= 0x00062320 && t < 0x00082460 => true,
                    // Bump allocator region — uninitialized vtable pointers
                    // Exclude 0x01E00000-0x01E10000 (our pre-AOT trampoline/data area)
                    t if t >= 0x01000000
                        && t < 0x10000000
                        && !(t >= 0x01E0_0000 && t < 0x01E1_0000) =>
                    {
                        true
                    }
                    // Zero/null function pointer
                    0 => true,
                    _ => false,
                };
                if is_hle_skip {
                    // Skip the call — just advance past it, set EAX=0 (success)
                    regs.eax = 0;
                    regs.eip = eip + len; // skip the call instruction
                } else {
                    // Normal call: push return address and jump
                    let ret = eip + len;
                    regs.esp = regs.esp.wrapping_sub(4);
                    memory.write_u32(regs.esp, ret);
                    // R_Init ESP trace: log every call inside R_Init (0x115E0-0x11A10)
                    if eip >= 0x115E0 && eip <= 0x11A10 {
                        debug_log(&format!(
                            "[R_INIT-CALL] 0x{:05X}: call 0x{:05X} ESP=0x{:08X} ret=0x{:05X}",
                            eip, target, regs.esp, ret
                        ));
                    }
                    regs.eip = target;
                }
            }
            Ret => {
                let ret_addr = memory.read_u32(regs.esp);
                let cleanup = if instr.op_count() > 0 {
                    match instr.op0_kind() {
                        OpKind::Immediate16 => instr.immediate16() as u32,
                        _ => 0,
                    }
                } else {
                    0
                };
                regs.esp = regs.esp.wrapping_add(4 + cleanup);
                // R_Init ESP trace: log RET returning into R_Init range
                if ret_addr >= 0x115E0 && ret_addr <= 0x11A10 {
                    debug_log(&format!(
                        "[R_INIT-RET] from 0x{:05X} to 0x{:05X} cleanup={} ESP=0x{:08X}",
                        eip, ret_addr, cleanup, regs.esp
                    ));
                }
                // Log RETs for debugging
                static RET_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let n = RET_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 100 {
                    debug_log(&format!(
                        "[INTERP-RET] #{} from 0x{:08X} to 0x{:08X} cleanup={} ESP=0x{:08X}",
                        n, eip, ret_addr, cleanup, regs.esp
                    ));
                }
                if ret_addr == expected_ret {
                    regs.eip = ret_addr;
                    return_with_snapshot!(InterpResult::ReturnedTo(ret_addr), 0, 0);
                }
                regs.eip = ret_addr;
            }
            Jmp => {
                let target = if instr.op0_kind() == OpKind::NearBranch32 {
                    instr.near_branch32()
                } else {
                    eval_operand(memory, regs, &instr, 0)
                };
                // Detect vtable JMP to wild address
                if target == 0 || (target >= 0x80000000 && target < 0xFFFF0000) {
                    debug_log(&format!(
                        "[INTERP-WILD-JMP] at 0x{:08X} target=0x{:08X} EAX=0x{:08X} ECX=0x{:08X} ESI=0x{:08X} ESP=0x{:08X}",
                        eip, target, regs.eax, regs.ecx, regs.esi, regs.esp
                    ));
                }
                regs.eip = target;
            }
            Je | Jne | Jb | Jae | Jl | Jge | Jle | Jg | Js | Jns | Jp | Jnp | Jbe | Ja | Jo
            | Jno => {
                let target = instr.near_branch32();
                let taken = match instr.mnemonic() {
                    Je => regs.eflags & 0x40 != 0,
                    Jne => regs.eflags & 0x40 == 0,
                    Jb => regs.eflags & 1 != 0,
                    Jae => regs.eflags & 1 == 0,
                    Js => regs.eflags & 0x80 != 0,
                    Jns => regs.eflags & 0x80 == 0,
                    Jo => regs.eflags & 0x800 != 0,
                    Jno => regs.eflags & 0x800 == 0,
                    Jl => (regs.eflags & 0x80 != 0) != (regs.eflags & 0x800 != 0),
                    Jge => (regs.eflags & 0x80 != 0) == (regs.eflags & 0x800 != 0),
                    Jle => {
                        (regs.eflags & 0x40 != 0)
                            || ((regs.eflags & 0x80 != 0) != (regs.eflags & 0x800 != 0))
                    }
                    Jg => {
                        (regs.eflags & 0x40 == 0)
                            && ((regs.eflags & 0x80 != 0) == (regs.eflags & 0x800 != 0))
                    }
                    Jbe => (regs.eflags & 1 != 0) || (regs.eflags & 0x40 != 0),
                    Ja => (regs.eflags & 1 == 0) && (regs.eflags & 0x40 == 0),
                    Jp => regs.eflags & 4 != 0,
                    Jnp => regs.eflags & 4 == 0,
                    _ => false,
                };
                if taken && target < eip {
                    if taken_backward_jump_is_generic_spin_candidate(eip, target) {
                        // Backward jump taken — potential spin loop.
                        if target == last_backward_target {
                            backward_count += 1;
                            if backward_count >= 500 {
                                // Break the spin: DON'T take the backward jump.
                                static SPIN_LOG: std::sync::atomic::AtomicU32 =
                                    std::sync::atomic::AtomicU32::new(0);
                                let n = SPIN_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if n < 20 {
                                    crate::xbox::emulator::debug_log(&format!(
                                        "[SPIN-BREAK] #{} backward jump 0x{:08X}->0x{:08X} after {} iterations",
                                        n, eip, target, backward_count
                                    ));
                                }
                                regs.eip = eip + len; // fall through
                                backward_count = 0;
                                last_backward_target = 0;
                                count += 1;
                                continue;
                            }
                        } else {
                            last_backward_target = target;
                            backward_count = 1;
                        }
                    }
                }
                regs.eip = if taken { target } else { eip + len };
            }

            // JECXZ: jump if ECX == 0 (used by CRT strlen)
            Jecxz => {
                let target = instr.near_branch32();
                if regs.ecx == 0 {
                    regs.eip = target;
                } else {
                    regs.eip = eip + len;
                }
            }
            Loopne => {
                regs.ecx = regs.ecx.wrapping_sub(1);
                let zf = (regs.eflags >> 6) & 1;
                if regs.ecx != 0 && zf == 0 {
                    regs.eip = instr.near_branch32();
                } else {
                    regs.eip = eip + len;
                }
            }
            Loope => {
                regs.ecx = regs.ecx.wrapping_sub(1);
                let zf = (regs.eflags >> 6) & 1;
                if regs.ecx != 0 && zf == 1 {
                    regs.eip = instr.near_branch32();
                } else {
                    regs.eip = eip + len;
                }
            }
            Loop => {
                regs.ecx = regs.ecx.wrapping_sub(1);
                if regs.ecx != 0 {
                    regs.eip = instr.near_branch32();
                } else {
                    regs.eip = eip + len;
                }
            }

            // --- FS segment (SEH) ---
            // mov eax, fs:[0] → read from KPCR at 0x0C000000
            // mov fs:[0], esp → write to KPCR at 0x0C000000
            // eval_mem_addr() adds 0x0C000000 for FS-prefix, matching emitter

            // --- Bitwise NOT / NEG ---
            Not => {
                let val = eval_operand(memory, regs, &instr, 0);
                store_operand(memory, regs, &instr, 0, !val);
                regs.eip += len;
            }
            Neg => {
                let val = eval_operand(memory, regs, &instr, 0);
                let result = 0u32.wrapping_sub(val);
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_sub(0, val, result);
                regs.eip += len;
            }
            Shr => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 0x1F;
                let result = val >> shift;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            Shl | Sal => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 0x1F;
                let result = val << shift;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            Sar => {
                let val = eval_operand(memory, regs, &instr, 0) as i32;
                let shift = eval_operand(memory, regs, &instr, 1) & 0x1F;
                let result = (val >> shift) as u32;
                store_operand(memory, regs, &instr, 0, result);
                regs.update_flags_test(result);
                regs.eip += len;
            }
            Setne | Sete | Setb | Setae | Setl | Setge | Setle | Setg | Sets | Setns => {
                let taken = match instr.mnemonic() {
                    Setne => !regs.flag_zf(),
                    Sete => regs.flag_zf(),
                    Setb => regs.flag_cf(),
                    Setae => !regs.flag_cf(),
                    Setl => regs.flag_sf() != regs.flag_of(),
                    Setge => regs.flag_sf() == regs.flag_of(),
                    Setle => regs.flag_zf() || (regs.flag_sf() != regs.flag_of()),
                    Setg => !regs.flag_zf() && (regs.flag_sf() == regs.flag_of()),
                    Sets => regs.flag_sf(),
                    Setns => !regs.flag_sf(),
                    _ => false,
                };
                store_operand(memory, regs, &instr, 0, if taken { 1 } else { 0 });
                regs.eip += len;
            }
            Cdq => {
                // Sign-extend EAX into EDX:EAX
                regs.edx = if (regs.eax as i32) < 0 { 0xFFFFFFFF } else { 0 };
                regs.eip += len;
            }
            // Port I/O — NV2A PCI configuration space
            // IN reads from port, OUT writes to port. Stub: return 0 for reads, ignore writes.
            In => {
                // IN EAX, DX — read from port DX
                regs.eax = 0; // stub: no data
                regs.eip += len;
            }
            Out => {
                // OUT DX, EAX — write to port DX (ignore)
                regs.eip += len;
            }
            // System/fence/barrier instructions — NOP stubs
            Wbinvd | Invd | Cli | Sti | Hlt | Invlpg | Sfence | Lfence | Mfence | Pause => {
                regs.eip += len;
            }
            Rdtsc => {
                // Return a monotonically increasing timestamp
                static TSC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                let t = TSC.fetch_add(1000, std::sync::atomic::Ordering::Relaxed);
                regs.eax = t as u32;
                regs.edx = (t >> 32) as u32;
                regs.eip += len;
            }
            Rdmsr | Wrmsr => {
                // Model-specific registers — return 0
                regs.eax = 0;
                regs.edx = 0;
                regs.eip += len;
            }
            // x87 control instructions (NOPs in our context — no FPU exceptions)
            Wait | Fnclex | Fclex => {
                regs.eip += len;
            }

            // SCASB/SCASD: repne/repe scasb/scasd (string scan, used by CRT strlen)
            Scasb => {
                let df = regs.eflags & (1 << 10) != 0;
                let step: u32 = if df { 0u32.wrapping_sub(1) } else { 1 };
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    while regs.ecx != 0 {
                        let val = memory.read_u8(regs.edi) as u32;
                        regs.ecx = regs.ecx.wrapping_sub(1);
                        regs.edi = regs.edi.wrapping_add(step);
                        let al = regs.eax & 0xFF;
                        let r = al.wrapping_sub(val);
                        regs.update_flags_sub(al, val, r);
                        // For 8-bit: fix SF to check bit 7, not bit 31
                        let sf8 = if (r as u8) & 0x80 != 0 { 0x80 } else { 0 };
                        regs.eflags = (regs.eflags & !0x80) | sf8;
                        if instr.has_repne_prefix() && regs.flag_zf() {
                            break;
                        }
                        if instr.has_rep_prefix() && !regs.flag_zf() {
                            break;
                        }
                    }
                } else {
                    let val = memory.read_u8(regs.edi) as u32;
                    regs.edi = regs.edi.wrapping_add(step);
                    let al = regs.eax & 0xFF;
                    let r = al.wrapping_sub(val);
                    regs.update_flags_sub(al, val, r);
                    let sf8 = if (r as u8) & 0x80 != 0 { 0x80 } else { 0 };
                    regs.eflags = (regs.eflags & !0x80) | sf8;
                }
                regs.eip += len;
            }
            Scasd => {
                let df = regs.eflags & (1 << 10) != 0;
                let step: u32 = if df { 0u32.wrapping_sub(4) } else { 4 };
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    while regs.ecx != 0 {
                        let val = memory.read_u32(regs.edi);
                        regs.ecx = regs.ecx.wrapping_sub(1);
                        regs.edi = regs.edi.wrapping_add(step);
                        let r = regs.eax.wrapping_sub(val);
                        regs.update_flags_sub(regs.eax, val, r);
                        if instr.has_repne_prefix() && regs.flag_zf() {
                            break;
                        }
                        if instr.has_rep_prefix() && !regs.flag_zf() {
                            break;
                        }
                    }
                } else {
                    let val = memory.read_u32(regs.edi);
                    regs.edi = regs.edi.wrapping_add(step);
                    let r = regs.eax.wrapping_sub(val);
                    regs.update_flags_sub(regs.eax, val, r);
                }
                regs.eip += len;
            }
            // CMPSB/CMPSD: string compare instructions
            Cmpsb => {
                let df = regs.eflags & (1 << 10) != 0;
                let step: u32 = if df { 0u32.wrapping_sub(1) } else { 1 };
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    while regs.ecx != 0 {
                        let a = memory.read_u8(regs.esi) as u32;
                        let b = memory.read_u8(regs.edi) as u32;
                        regs.ecx = regs.ecx.wrapping_sub(1);
                        regs.esi = regs.esi.wrapping_add(step);
                        regs.edi = regs.edi.wrapping_add(step);
                        let r = a.wrapping_sub(b);
                        regs.update_flags_sub(a, b, r);
                        let sf8 = if (r as u8) & 0x80 != 0 { 0x80 } else { 0 };
                        regs.eflags = (regs.eflags & !0x80) | sf8;
                        if instr.has_repne_prefix() && regs.flag_zf() {
                            break;
                        }
                        if instr.has_rep_prefix() && !regs.flag_zf() {
                            break;
                        }
                    }
                } else {
                    let a = memory.read_u8(regs.esi) as u32;
                    let b = memory.read_u8(regs.edi) as u32;
                    regs.esi = regs.esi.wrapping_add(step);
                    regs.edi = regs.edi.wrapping_add(step);
                    let r = a.wrapping_sub(b);
                    regs.update_flags_sub(a, b, r);
                    let sf8 = if (r as u8) & 0x80 != 0 { 0x80 } else { 0 };
                    regs.eflags = (regs.eflags & !0x80) | sf8;
                }
                regs.eip += len;
            }
            Cmpsd => {
                let df = regs.eflags & (1 << 10) != 0;
                let step: u32 = if df { 0u32.wrapping_sub(4) } else { 4 };
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    while regs.ecx != 0 {
                        let a = memory.read_u32(regs.esi);
                        let b = memory.read_u32(regs.edi);
                        regs.ecx = regs.ecx.wrapping_sub(1);
                        regs.esi = regs.esi.wrapping_add(step);
                        regs.edi = regs.edi.wrapping_add(step);
                        let r = a.wrapping_sub(b);
                        regs.update_flags_sub(a, b, r);
                        if instr.has_repne_prefix() && regs.flag_zf() {
                            break;
                        }
                        if instr.has_rep_prefix() && !regs.flag_zf() {
                            break;
                        }
                    }
                } else {
                    let a = memory.read_u32(regs.esi);
                    let b = memory.read_u32(regs.edi);
                    regs.esi = regs.esi.wrapping_add(step);
                    regs.edi = regs.edi.wrapping_add(step);
                    let r = a.wrapping_sub(b);
                    regs.update_flags_sub(a, b, r);
                }
                regs.eip += len;
            }

            // --- x87 FPU instructions ---
            Fild => {
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.read_u16(addr) as i16 as f64,
                    iced_x86::MemorySize::Int64 => {
                        let lo = memory.read_u32(addr) as u64;
                        let hi = memory.read_u32(addr + 4) as u64;
                        (lo | (hi << 32)) as i64 as f64
                    }
                    _ => memory.read_u32(addr) as i32 as f64, // Int32 default
                };
                regs.fpu_top = (regs.fpu_top + 7) % 8; // push
                regs.fpu_stack[regs.fpu_top] = val;
                regs.eip += len;
            }
            Fld => {
                let val = if instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    fpu_read_float(memory, addr, instr.memory_size())
                } else {
                    // FLD ST(i)
                    let idx = fpu_reg_index(regs, instr.op0_register()).unwrap_or(regs.fpu_top);
                    regs.fpu_stack[idx]
                };
                regs.fpu_top = (regs.fpu_top + 7) % 8;
                regs.fpu_stack[regs.fpu_top] = val;
                regs.eip += len;
            }
            Fst | Fstp => {
                let val = regs.fpu_stack[regs.fpu_top];
                if instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    match instr.memory_size() {
                        iced_x86::MemorySize::Float64 => {
                            let bits = val.to_bits();
                            memory.write_u32(addr, bits as u32);
                            memory.write_u32(addr + 4, (bits >> 32) as u32);
                        }
                        _ => memory.write_u32(addr, (val as f32).to_bits()),
                    }
                }
                if instr.mnemonic() == Fstp {
                    regs.fpu_top = (regs.fpu_top + 1) % 8; // pop
                }
                regs.eip += len;
            }
            Fiadd => {
                // FIADD m16int/m32int: ST(0) += (int)mem
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.read_u16(addr) as i16 as f64,
                    _ => memory.read_u32(addr) as i32 as f64,
                };
                regs.fpu_stack[regs.fpu_top] += val;
                regs.eip += len;
            }
            Fisub => {
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.read_u16(addr) as i16 as f64,
                    _ => memory.read_u32(addr) as i32 as f64,
                };
                regs.fpu_stack[regs.fpu_top] -= val;
                regs.eip += len;
            }
            Fimul => {
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.read_u16(addr) as i16 as f64,
                    _ => memory.read_u32(addr) as i32 as f64,
                };
                regs.fpu_stack[regs.fpu_top] *= val;
                regs.eip += len;
            }
            Fidiv => {
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.read_u16(addr) as i16 as f64,
                    _ => memory.read_u32(addr) as i32 as f64,
                };
                if val != 0.0 {
                    regs.fpu_stack[regs.fpu_top] /= val;
                }
                regs.eip += len;
            }
            Fist | Fistp => {
                // FIST/FISTP: store ST(0) as integer to memory
                let addr = eval_mem_addr(regs, &instr, 0);
                let val = regs.fpu_stack[regs.fpu_top] as i32;
                match instr.memory_size() {
                    iced_x86::MemorySize::Int16 => memory.write_u16(addr, val as u16),
                    _ => memory.write_u32(addr, val as u32),
                }
                if instr.mnemonic() == Fistp {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fadd | Faddp => {
                if instr.op_count() == 1 && instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    let val = fpu_read_float(memory, addr, instr.memory_size());
                    regs.fpu_stack[regs.fpu_top] += val;
                } else {
                    let dst = if instr.mnemonic() == Faddp {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else if instr.op_count() >= 1 && instr.op0_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or(regs.fpu_top)
                    } else {
                        regs.fpu_top
                    };
                    let src = if instr.mnemonic() == Faddp {
                        regs.fpu_top
                    } else if instr.op_count() >= 2 && instr.op1_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op1_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else {
                        (regs.fpu_top + 1) % 8
                    };
                    regs.fpu_stack[dst] += regs.fpu_stack[src];
                }
                if instr.mnemonic() == Faddp {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fsub | Fsubp | Fsubr | Fsubrp => {
                if instr.op_count() == 1 && instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    let val = fpu_read_float(memory, addr, instr.memory_size());
                    if instr.mnemonic() == Fsubr || instr.mnemonic() == Fsubrp {
                        regs.fpu_stack[regs.fpu_top] = val - regs.fpu_stack[regs.fpu_top];
                    } else {
                        regs.fpu_stack[regs.fpu_top] -= val;
                    }
                } else {
                    let is_pop = matches!(instr.mnemonic(), Fsubp | Fsubrp);
                    let dst = if is_pop {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else if instr.op_count() >= 1 && instr.op0_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or(regs.fpu_top)
                    } else {
                        regs.fpu_top
                    };
                    let src = if is_pop {
                        regs.fpu_top
                    } else if instr.op_count() >= 2 && instr.op1_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op1_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else {
                        (regs.fpu_top + 1) % 8
                    };
                    let a = regs.fpu_stack[dst];
                    let b = regs.fpu_stack[src];
                    regs.fpu_stack[dst] = if matches!(instr.mnemonic(), Fsubr | Fsubrp) {
                        b - a
                    } else {
                        a - b
                    };
                }
                if matches!(instr.mnemonic(), Fsubp | Fsubrp) {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fmul | Fmulp => {
                if instr.op_count() == 1 && instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    let val = fpu_read_float(memory, addr, instr.memory_size());
                    regs.fpu_stack[regs.fpu_top] *= val;
                } else {
                    let dst = if instr.mnemonic() == Fmulp {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else if instr.op_count() >= 1 && instr.op0_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or(regs.fpu_top)
                    } else {
                        regs.fpu_top
                    };
                    let src = if instr.mnemonic() == Fmulp {
                        regs.fpu_top
                    } else if instr.op_count() >= 2 && instr.op1_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op1_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else {
                        (regs.fpu_top + 1) % 8
                    };
                    regs.fpu_stack[dst] *= regs.fpu_stack[src];
                }
                if instr.mnemonic() == Fmulp {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fdiv | Fdivp | Fdivr | Fdivrp => {
                if instr.op_count() == 1 && instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    let val = fpu_read_float(memory, addr, instr.memory_size());
                    if instr.mnemonic() == Fdivr || instr.mnemonic() == Fdivrp {
                        if regs.fpu_stack[regs.fpu_top] != 0.0 {
                            regs.fpu_stack[regs.fpu_top] = val / regs.fpu_stack[regs.fpu_top];
                        }
                    } else if val != 0.0 {
                        regs.fpu_stack[regs.fpu_top] /= val;
                    }
                } else {
                    let is_pop = matches!(instr.mnemonic(), Fdivp | Fdivrp);
                    let dst = if is_pop {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else if instr.op_count() >= 1 && instr.op0_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or(regs.fpu_top)
                    } else {
                        regs.fpu_top
                    };
                    let src = if is_pop {
                        regs.fpu_top
                    } else if instr.op_count() >= 2 && instr.op1_kind() == OpKind::Register {
                        fpu_reg_index(regs, instr.op1_register()).unwrap_or((regs.fpu_top + 1) % 8)
                    } else {
                        (regs.fpu_top + 1) % 8
                    };
                    let a = regs.fpu_stack[dst];
                    let b = regs.fpu_stack[src];
                    if matches!(instr.mnemonic(), Fdivr | Fdivrp) {
                        if a != 0.0 {
                            regs.fpu_stack[dst] = b / a;
                        }
                    } else if b != 0.0 {
                        regs.fpu_stack[dst] = a / b;
                    }
                }
                if matches!(instr.mnemonic(), Fdivp | Fdivrp) {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fcomp | Fcom => {
                let st0 = regs.fpu_stack[regs.fpu_top];
                let val = if instr.op0_kind() == OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    fpu_read_float(memory, addr, instr.memory_size())
                } else {
                    let idx =
                        fpu_reg_index(regs, instr.op0_register()).unwrap_or((regs.fpu_top + 1) % 8);
                    regs.fpu_stack[idx]
                };
                // Set FPU status word for FNSTSW
                let mut sw = 0u16;
                if st0 < val {
                    sw |= 0x0100;
                } // C0
                if st0 == val {
                    sw |= 0x4000;
                } // C3
                regs.eflags = (regs.eflags & 0xFFFF0000) | sw as u32;
                if instr.mnemonic() == Fcomp {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fcomip | Fucomi | Fucomip | Fcomi => {
                // Compare ST(0) with ST(i), set EFLAGS directly (not FPU SW)
                let st0 = regs.fpu_stack[regs.fpu_top];
                let idx = if instr.op_count() >= 2 {
                    // op1 is STi
                    let reg = instr.op1_register();
                    let i = match reg {
                        iced_x86::Register::ST0 => 0,
                        iced_x86::Register::ST1 => 1,
                        iced_x86::Register::ST2 => 2,
                        iced_x86::Register::ST3 => 3,
                        iced_x86::Register::ST4 => 4,
                        iced_x86::Register::ST5 => 5,
                        iced_x86::Register::ST6 => 6,
                        iced_x86::Register::ST7 => 7,
                        _ => 1,
                    };
                    (regs.fpu_top + i) % 8
                } else {
                    (regs.fpu_top + 1) % 8
                };
                let val = regs.fpu_stack[idx];
                // Set EFLAGS: ZF (0x40), PF (0x04), CF (0x01)
                let mut flags = regs.eflags & !(0x40 | 0x04 | 0x01);
                if st0.is_nan() || val.is_nan() {
                    flags |= 0x40 | 0x04 | 0x01; // unordered
                } else if st0 < val {
                    flags |= 0x01; // CF=1
                } else if st0 == val {
                    flags |= 0x40; // ZF=1
                }
                // else: st0 > val, all clear
                regs.eflags = flags;
                // Pop if Fcomip or Fucomip
                if instr.mnemonic() == Fcomip || instr.mnemonic() == Fucomip {
                    regs.fpu_top = (regs.fpu_top + 1) % 8;
                }
                regs.eip += len;
            }
            Fnstsw => {
                // Store FPU status word to AX or memory
                let sw = (regs.eflags & 0xFFFF) as u16;
                if instr.op0_kind() == OpKind::Register {
                    regs.eax = (regs.eax & 0xFFFF0000) | sw as u32;
                } else {
                    let addr = eval_mem_addr(regs, &instr, 0);
                    memory.write_u16(addr, sw);
                }
                regs.eip += len;
            }
            Fldz => {
                regs.fpu_top = (regs.fpu_top + 7) % 8;
                regs.fpu_stack[regs.fpu_top] = 0.0;
                regs.eip += len;
            }
            Fld1 => {
                regs.fpu_top = (regs.fpu_top + 7) % 8;
                regs.fpu_stack[regs.fpu_top] = 1.0;
                regs.eip += len;
            }
            Fchs => {
                regs.fpu_stack[regs.fpu_top] = -regs.fpu_stack[regs.fpu_top];
                regs.eip += len;
            }
            Fabs => {
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top].abs();
                regs.eip += len;
            }
            Fsqrt => {
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top].sqrt();
                regs.eip += len;
            }
            Fsin => {
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top].sin();
                regs.eip += len;
            }
            Fcos => {
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top].cos();
                regs.eip += len;
            }
            Fpatan => {
                // ST(1) = atan2(ST(1), ST(0)), pop ST(0)
                let st0 = regs.fpu_stack[regs.fpu_top];
                let st1_idx = (regs.fpu_top + 1) % 8;
                regs.fpu_stack[st1_idx] = regs.fpu_stack[st1_idx].atan2(st0);
                regs.fpu_top = (regs.fpu_top + 1) % 8;
                regs.eip += len;
            }
            Frndint => {
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top].round();
                regs.eip += len;
            }
            Fscale => {
                // ST(0) = ST(0) * 2^trunc(ST(1))
                let st1 = regs.fpu_stack[(regs.fpu_top + 1) % 8];
                let scale = 2.0f64.powi(st1 as i32);
                regs.fpu_stack[regs.fpu_top] *= scale;
                regs.eip += len;
            }
            Fyl2x => {
                // ST(1) = ST(1) * log2(ST(0)), pop
                let st0 = regs.fpu_stack[regs.fpu_top];
                let st1_idx = (regs.fpu_top + 1) % 8;
                regs.fpu_stack[st1_idx] = regs.fpu_stack[st1_idx] * st0.log2();
                regs.fpu_top = (regs.fpu_top + 1) % 8;
                regs.eip += len;
            }
            F2xm1 => {
                // ST(0) = 2^ST(0) - 1
                regs.fpu_stack[regs.fpu_top] = (2.0f64).powf(regs.fpu_stack[regs.fpu_top]) - 1.0;
                regs.eip += len;
            }
            Fprem | Fprem1 => {
                let st1 = regs.fpu_stack[(regs.fpu_top + 1) % 8];
                if st1 != 0.0 {
                    regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[regs.fpu_top] % st1;
                }
                // Clear C2 (bit 10 in FPU SW) to indicate reduction complete
                regs.eflags &= !(1 << 10);
                regs.eip += len;
            }
            Fsincos => {
                let val = regs.fpu_stack[regs.fpu_top];
                regs.fpu_stack[regs.fpu_top] = val.sin();
                // Push cos
                regs.fpu_top = (regs.fpu_top + 7) % 8; // push
                regs.fpu_stack[regs.fpu_top] = val.cos();
                regs.eip += len;
            }
            Fptan => {
                let val = regs.fpu_stack[regs.fpu_top];
                regs.fpu_stack[regs.fpu_top] = val.tan();
                // Push 1.0
                regs.fpu_top = (regs.fpu_top + 7) % 8;
                regs.fpu_stack[regs.fpu_top] = 1.0;
                regs.eip += len;
            }
            Ftst => {
                // Compare ST(0) with 0.0, set FPU status word
                let st0 = regs.fpu_stack[regs.fpu_top];
                let mut sw = 0u16;
                if st0 < 0.0 {
                    sw |= 0x0100;
                } // C0
                if st0 == 0.0 {
                    sw |= 0x4000;
                } // C3
                if st0.is_nan() {
                    sw |= 0x4500;
                } // unordered
                regs.eflags = (regs.eflags & 0xFFFF0000) | sw as u32;
                regs.eip += len;
            }
            Fxch => {
                let idx = if instr.op_count() > 0 {
                    (regs.fpu_top + instr.op0_register() as usize
                        - iced_x86::Register::ST0 as usize)
                        % 8
                } else {
                    (regs.fpu_top + 1) % 8
                };
                let tmp = regs.fpu_stack[regs.fpu_top];
                regs.fpu_stack[regs.fpu_top] = regs.fpu_stack[idx];
                regs.fpu_stack[idx] = tmp;
                regs.eip += len;
            }
            Fistp => {
                let val = regs.fpu_stack[regs.fpu_top] as i32;
                let addr = eval_mem_addr(regs, &instr, 0);
                memory.write_u32(addr, val as u32);
                regs.fpu_top = (regs.fpu_top + 1) % 8;
                regs.eip += len;
            }
            Fnstcw | Fstcw => {
                let addr = eval_mem_addr(regs, &instr, 0);
                memory.write_u16(addr, 0x037F); // default control word
                regs.eip += len;
            }
            Fldcw => {
                // Load control word — ignore (we don't track rounding mode)
                regs.eip += len;
            }
            Cpuid => {
                // Return Pentium III identity
                match regs.eax {
                    0 => {
                        regs.eax = 2;
                        regs.ebx = 0x756E6547;
                        regs.edx = 0x49656E69;
                        regs.ecx = 0x6C65746E;
                    } // "GenuineIntel"
                    1 => {
                        regs.eax = 0x673;
                        regs.ebx = 0;
                        regs.ecx = 0;
                        regs.edx = 0x0383F9FF;
                    } // P3
                    _ => {
                        regs.eax = 0;
                        regs.ebx = 0;
                        regs.ecx = 0;
                        regs.edx = 0;
                    }
                }
                regs.eip += len;
            }
            Imul => {
                if instr.op_count() == 3 {
                    let a = eval_operand(memory, regs, &instr, 1);
                    let b = eval_operand(memory, regs, &instr, 2);
                    store_operand(memory, regs, &instr, 0, a.wrapping_mul(b));
                } else if instr.op_count() == 2 {
                    let a = eval_operand(memory, regs, &instr, 0);
                    let b = eval_operand(memory, regs, &instr, 1);
                    store_operand(memory, regs, &instr, 0, a.wrapping_mul(b));
                } else {
                    let val = eval_operand(memory, regs, &instr, 0);
                    let result = (regs.eax as i32 as i64).wrapping_mul(val as i32 as i64);
                    regs.eax = result as u32;
                    regs.edx = (result >> 32) as u32;
                }
                regs.eip += len;
            }
            Idiv => {
                let divisor = eval_operand(memory, regs, &instr, 0) as i32;
                if divisor == 0 {
                    // Division by zero — on real x86, raises INT 0.
                    // For game compat: set EAX=0, EDX=0, skip instruction.
                    regs.eax = 0;
                    regs.edx = 0;
                    regs.eip += len;
                } else {
                    let dividend = (((regs.edx as u64) << 32) | regs.eax as u64) as i64;
                    regs.eax = (dividend / divisor as i64) as u32;
                    regs.edx = (dividend % divisor as i64) as u32;
                    regs.eip += len;
                }
            }

            // --- String operations (rep movsd, rep movsb, rep stosd, rep stosb) ---
            Movsd => {
                let total = if instr.has_rep_prefix() { regs.ecx } else { 1 };
                // Sanity: if rep count > 4MB of dwords (16MB), it's uninitialized garbage.
                // Trigger immediate AV instead of writing through all guest memory.
                if total > 0x40_0000 && instr.has_rep_prefix() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[MOVSD-SKIP] rep movsd ECX=0x{:08X} ESI=0x{:08X} EDI=0x{:08X} EIP=0x{:08X} — skipped (D3D surface copy with garbage count)",
                        total, regs.esi, regs.edi, regs.eip
                    ));
                    // Skip instead of AV — D3D surface copies with uninitialized count
                    // are caused by missing surface descriptor in our CreateDevice HLE.
                    // The framebuffer is already captured via I_FinishUpdate.
                    regs.ecx = 0;
                    regs.eip += len;
                    continue;
                }
                let cap = total;
                for i in 0..cap {
                    if !is_safe_addr(regs.esi) || !is_safe_addr(regs.edi) {
                        if instr.has_rep_prefix() {
                            regs.ecx = total - i;
                        }
                        return_with_snapshot!(
                            InterpResult::AccessViolation {
                                eip: regs.eip,
                                addr: regs.edi,
                            },
                            1,
                            regs.eip
                        );
                    }
                    let val = memory.read_u32(regs.esi);
                    memory.write_u32(regs.edi, val);
                    regs.esi = regs.esi.wrapping_add(4);
                    regs.edi = regs.edi.wrapping_add(4);
                }
                count += cap as u64;
                if instr.has_rep_prefix() {
                    regs.ecx = total - cap;
                    if regs.ecx == 0 {
                        regs.eip += len;
                    }
                } else {
                    regs.eip += len;
                }
            }
            Movsb => {
                let total = if instr.has_rep_prefix() { regs.ecx } else { 1 };
                if total > 0x100_0000 && instr.has_rep_prefix() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[MOVSB-INSANE] rep movsb ECX=0x{:08X} EIP=0x{:08X} — immediate AV",
                        total, regs.eip
                    ));
                    return_with_snapshot!(
                        InterpResult::AccessViolation {
                            eip: regs.eip,
                            addr: 0xDEAD_DEAD,
                        },
                        1,
                        regs.eip
                    );
                }
                let cap = total;
                for i in 0..cap {
                    if !is_safe_addr(regs.esi) || !is_safe_addr(regs.edi) {
                        if instr.has_rep_prefix() {
                            regs.ecx = total - i;
                        }
                        return_with_snapshot!(
                            InterpResult::AccessViolation {
                                eip: regs.eip,
                                addr: regs.edi,
                            },
                            1,
                            regs.eip
                        );
                    }
                    let val = memory.read_u8(regs.esi);
                    memory.write_u8(regs.edi, val);
                    regs.esi = regs.esi.wrapping_add(1);
                    regs.edi = regs.edi.wrapping_add(1);
                }
                count += cap as u64;
                if instr.has_rep_prefix() {
                    regs.ecx = total - cap;
                    if regs.ecx == 0 {
                        regs.eip += len;
                    }
                } else {
                    regs.eip += len;
                }
            }
            Stosd => {
                let total = if instr.has_rep_prefix() { regs.ecx } else { 1 };
                if total > 0x40_0000 && instr.has_rep_prefix() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[STOSD-INSANE] rep stosd ECX=0x{:08X} EIP=0x{:08X} — immediate AV",
                        total, regs.eip
                    ));
                    return_with_snapshot!(
                        InterpResult::AccessViolation {
                            eip: regs.eip,
                            addr: 0xDEAD_DEAD,
                        },
                        1,
                        regs.eip
                    );
                }
                // Watchpoint: detect rep stosd covering 0x12706
                {
                    use std::sync::atomic::{AtomicBool, Ordering};
                    static STOSD_LOGGED: AtomicBool = AtomicBool::new(false);
                    let start = regs.edi;
                    let end = start.wrapping_add(total.wrapping_mul(4));
                    if start <= 0x0001_2706
                        && end > 0x0001_2706
                        && !STOSD_LOGGED.swap(true, Ordering::Relaxed)
                    {
                        crate::xbox::emulator::debug_log(&format!(
                            "[WATCHPOINT] rep stosd EDI=0x{:08X} ECX=0x{:08X} EAX=0x{:08X} EIP=0x{:08X} COVERS 0x12706!",
                            regs.edi, total, regs.eax, regs.eip
                        ));
                    }
                }
                let cap = total;
                for _ in 0..cap {
                    if !is_safe_addr(regs.edi) {
                        return_with_snapshot!(
                            InterpResult::AccessViolation {
                                eip: regs.eip,
                                addr: regs.edi,
                            },
                            1,
                            regs.eip
                        );
                    }
                    memory.write_u32(regs.edi, regs.eax);
                    regs.edi = regs.edi.wrapping_add(4);
                }
                count += cap as u64;
                if instr.has_rep_prefix() {
                    regs.ecx = total - cap;
                    if regs.ecx == 0 {
                        regs.eip += len;
                    }
                } else {
                    regs.eip += len;
                }
            }
            Stosw => {
                let total = if instr.has_rep_prefix() { regs.ecx } else { 1 };
                for _ in 0..total {
                    memory.write_u16(regs.edi, regs.eax as u16);
                    regs.edi = regs.edi.wrapping_add(2);
                }
                count += total as u64;
                if instr.has_rep_prefix() {
                    regs.ecx = 0;
                    regs.eip += len;
                } else {
                    regs.eip += len;
                }
            }
            Stosb => {
                let total = if instr.has_rep_prefix() { regs.ecx } else { 1 };
                // Watchpoint: detect rep stosb covering 0x12706
                {
                    use std::sync::atomic::{AtomicBool, Ordering};
                    static STOSB_LOGGED: AtomicBool = AtomicBool::new(false);
                    let start = regs.edi;
                    let end = start.wrapping_add(total);
                    if start <= 0x0001_2706
                        && end > 0x0001_2706
                        && !STOSB_LOGGED.swap(true, Ordering::Relaxed)
                    {
                        crate::xbox::emulator::debug_log(&format!(
                            "[WATCHPOINT] rep stosb EDI=0x{:08X} ECX=0x{:08X} EAX=0x{:08X} EIP=0x{:08X} COVERS 0x12706!",
                            regs.edi, total, regs.eax, regs.eip
                        ));
                    }
                }
                let cap = total; // No cap
                for _ in 0..cap {
                    memory.write_u8(regs.edi, regs.eax as u8);
                    regs.edi = regs.edi.wrapping_add(1);
                }
                count += cap as u64;
                if instr.has_rep_prefix() {
                    regs.ecx = total - cap;
                    if regs.ecx == 0 {
                        regs.eip += len;
                    }
                } else {
                    regs.eip += len;
                }
            }

            Aam => {
                // AAM imm8: AH = AL / imm8, AL = AL % imm8
                let base = instr.immediate(0) as u8;
                if base == 0 {
                    regs.eip += len;
                    continue;
                } // div by zero guard
                let al = (regs.eax & 0xFF) as u8;
                let ah = al / base;
                let new_al = al % base;
                regs.eax = (regs.eax & 0xFFFF_0000) | ((ah as u32) << 8) | (new_al as u32);
                regs.eip += len;
            }
            Aad => {
                // AAD imm8: AL = AH * imm8 + AL, AH = 0
                let base = instr.immediate(0) as u8;
                let al = (regs.eax & 0xFF) as u8;
                let ah = ((regs.eax >> 8) & 0xFF) as u8;
                let result = (ah as u16)
                    .wrapping_mul(base as u16)
                    .wrapping_add(al as u16) as u8;
                regs.eax = (regs.eax & 0xFFFF_0000) | (result as u32);
                regs.eip += len;
            }
            Salc => {
                // SALC (0xD6): undocumented. AL = 0xFF if CF set, 0x00 if CF clear.
                let cf = regs.eflags & 1;
                let al = if cf != 0 { 0xFF } else { 0x00 };
                regs.eax = (regs.eax & 0xFFFF_FF00) | al;
                regs.eip += len;
            }
            Xlatb => {
                // XLATB: AL = [EBX + unsigned AL]
                let addr = regs.ebx.wrapping_add(regs.eax & 0xFF);
                let val = memory.read_u8(addr);
                regs.eax = (regs.eax & 0xFFFF_FF00) | (val as u32);
                regs.eip += len;
            }
            Leave => {
                // LEAVE = MOV ESP, EBP; POP EBP
                regs.esp = regs.ebp;
                regs.ebp = memory.read_u32(regs.esp);
                regs.esp = regs.esp.wrapping_add(4);
                regs.eip += len;
            }
            Xchg => {
                let a = eval_operand(memory, regs, &instr, 0);
                let b = eval_operand(memory, regs, &instr, 1);
                store_operand(memory, regs, &instr, 0, b);
                store_operand(memory, regs, &instr, 1, a);
                regs.eip += len;
            }

            Div => {
                let divisor = eval_operand(memory, regs, &instr, 0);
                if divisor == 0 {
                    regs.eax = 0;
                    regs.edx = 0;
                    regs.eip += len;
                } else {
                    let dividend = ((regs.edx as u64) << 32) | (regs.eax as u64);
                    regs.eax = (dividend / divisor as u64) as u32;
                    regs.edx = (dividend % divisor as u64) as u32;
                    regs.eip += len;
                }
            }
            Mul => {
                let val = eval_operand(memory, regs, &instr, 0);
                let result = (regs.eax as u64).wrapping_mul(val as u64);
                regs.eax = result as u32;
                regs.edx = (result >> 32) as u32;
                regs.eip += len;
            }
            Shrd => {
                // SHRD dst, src, count: double-precision right shift
                let dst = eval_operand(memory, regs, &instr, 0);
                let src = eval_operand(memory, regs, &instr, 1);
                let count = (eval_operand(memory, regs, &instr, 2) & 31) as u32;
                if count > 0 {
                    let combined = ((src as u64) << 32) | (dst as u64);
                    let result = (combined >> count) as u32;
                    store_operand(memory, regs, &instr, 0, result);
                    // CF = last bit shifted out
                    if (dst >> (count - 1)) & 1 != 0 {
                        regs.eflags |= 1;
                    } else {
                        regs.eflags &= !1;
                    }
                    let zf = if result == 0 { 0x40 } else { 0 };
                    let sf = if result & 0x8000_0000 != 0 { 0x80 } else { 0 };
                    regs.eflags = (regs.eflags & !0xC0) | zf | sf;
                }
                regs.eip += len;
            }
            Shld => {
                // SHLD dst, src, count: double-precision left shift
                let dst = eval_operand(memory, regs, &instr, 0);
                let src = eval_operand(memory, regs, &instr, 1);
                let count = (eval_operand(memory, regs, &instr, 2) & 31) as u32;
                if count > 0 {
                    let combined = ((dst as u64) << 32) | (src as u64);
                    let result = (combined << count >> 32) as u32;
                    store_operand(memory, regs, &instr, 0, result);
                    if (dst >> (32 - count)) & 1 != 0 {
                        regs.eflags |= 1;
                    } else {
                        regs.eflags &= !1;
                    }
                    let zf = if result == 0 { 0x40 } else { 0 };
                    let sf = if result & 0x8000_0000 != 0 { 0x80 } else { 0 };
                    regs.eflags = (regs.eflags & !0xC0) | zf | sf;
                }
                regs.eip += len;
            }
            Bt => {
                let base_val = eval_operand(memory, regs, &instr, 0);
                let bit = eval_operand(memory, regs, &instr, 1) & 31;
                if base_val & (1 << bit) != 0 {
                    regs.eflags |= 1;
                } else {
                    regs.eflags &= !1;
                }
                regs.eip += len;
            }
            Bsr => {
                let val = eval_operand(memory, regs, &instr, 1);
                if val == 0 {
                    regs.eflags |= 0x40; // ZF=1
                } else {
                    let bit = 31 - val.leading_zeros();
                    store_operand(memory, regs, &instr, 0, bit);
                    regs.eflags &= !0x40; // ZF=0
                }
                regs.eip += len;
            }
            Bsf => {
                let val = eval_operand(memory, regs, &instr, 1);
                if val == 0 {
                    regs.eflags |= 0x40;
                } else {
                    store_operand(memory, regs, &instr, 0, val.trailing_zeros());
                    regs.eflags &= !0x40;
                }
                regs.eip += len;
            }
            Ror => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 31;
                let result = val.rotate_right(shift);
                store_operand(memory, regs, &instr, 0, result);
                regs.eip += len;
            }
            Rol => {
                let val = eval_operand(memory, regs, &instr, 0);
                let shift = eval_operand(memory, regs, &instr, 1) & 31;
                let result = val.rotate_left(shift);
                store_operand(memory, regs, &instr, 0, result);
                regs.eip += len;
            }

            // --- SSE float instructions ---
            Movss => {
                // movss xmm, [mem]: load 32-bit float, zero upper 96 bits
                // movss [mem], xmm: store low 32-bit float
                // movss xmm, xmm: copy low 32 bits (upper dest preserved)
                let op0_kind = instr.op_kind(0);
                let op1_kind = instr.op_kind(1);
                if op0_kind == iced_x86::OpKind::Register && op1_kind == iced_x86::OpKind::Memory {
                    // movss xmm, [mem]
                    let xmm_idx = xmm_index(instr.op_register(0));
                    let addr = eval_mem_addr(regs, &instr, 1);
                    let val = sse_read_f32(memory, addr);
                    regs.xmm[xmm_idx] = [0u8; 16]; // zero all 128 bits
                    regs.xmm[xmm_idx][..4].copy_from_slice(&val.to_le_bytes());
                } else if op0_kind == iced_x86::OpKind::Memory
                    && op1_kind == iced_x86::OpKind::Register
                {
                    // movss [mem], xmm
                    let xmm_idx = xmm_index(instr.op_register(1));
                    let addr = eval_mem_addr(regs, &instr, 0);
                    let val = f32::from_le_bytes(regs.xmm[xmm_idx][..4].try_into().unwrap());
                    sse_write_f32(memory, addr, val);
                } else if op0_kind == iced_x86::OpKind::Register
                    && op1_kind == iced_x86::OpKind::Register
                {
                    // movss xmm, xmm — copy low 32 bits only
                    let dst = xmm_index(instr.op_register(0));
                    let src = xmm_index(instr.op_register(1));
                    let src_bytes: [u8; 4] = regs.xmm[src][..4].try_into().unwrap();
                    regs.xmm[dst][..4].copy_from_slice(&src_bytes);
                }
                regs.eip += len;
            }
            Movaps | Movups => {
                // movaps/movups xmm, xmm/[mem] or [mem], xmm — 128-bit move
                let op0_kind = instr.op_kind(0);
                let op1_kind = instr.op_kind(1);
                if op0_kind == iced_x86::OpKind::Register && op1_kind == iced_x86::OpKind::Register
                {
                    let dst = xmm_index(instr.op_register(0));
                    let src = xmm_index(instr.op_register(1));
                    let data = regs.xmm[src];
                    regs.xmm[dst] = data;
                } else if op0_kind == iced_x86::OpKind::Register
                    && op1_kind == iced_x86::OpKind::Memory
                {
                    let dst = xmm_index(instr.op_register(0));
                    let addr = eval_mem_addr(regs, &instr, 1);
                    regs.xmm[dst] = sse_read_128(memory, addr);
                } else if op0_kind == iced_x86::OpKind::Memory
                    && op1_kind == iced_x86::OpKind::Register
                {
                    let src = xmm_index(instr.op_register(1));
                    let addr = eval_mem_addr(regs, &instr, 0);
                    sse_write_128(memory, addr, &regs.xmm[src]);
                }
                regs.eip += len;
            }
            Movlps => {
                // movlps xmm, [mem] — load 64 bits into low qword (upper preserved)
                // movlps [mem], xmm — store low 64 bits
                let op0_kind = instr.op_kind(0);
                let op1_kind = instr.op_kind(1);
                if op0_kind == iced_x86::OpKind::Register && op1_kind == iced_x86::OpKind::Memory {
                    let dst = xmm_index(instr.op_register(0));
                    let addr = eval_mem_addr(regs, &instr, 1);
                    let data = sse_read_64(memory, addr);
                    regs.xmm[dst][..8].copy_from_slice(&data);
                } else if op0_kind == iced_x86::OpKind::Memory
                    && op1_kind == iced_x86::OpKind::Register
                {
                    let src = xmm_index(instr.op_register(1));
                    let addr = eval_mem_addr(regs, &instr, 0);
                    sse_write_64(memory, addr, &regs.xmm[src][..8]);
                }
                regs.eip += len;
            }
            Movhps => {
                // movhps xmm, [mem] — load 64 bits into high qword (low preserved)
                // movhps [mem], xmm — store high 64 bits
                let op0_kind = instr.op_kind(0);
                let op1_kind = instr.op_kind(1);
                if op0_kind == iced_x86::OpKind::Register && op1_kind == iced_x86::OpKind::Memory {
                    let dst = xmm_index(instr.op_register(0));
                    let addr = eval_mem_addr(regs, &instr, 1);
                    let data = sse_read_64(memory, addr);
                    regs.xmm[dst][8..16].copy_from_slice(&data);
                } else if op0_kind == iced_x86::OpKind::Memory
                    && op1_kind == iced_x86::OpKind::Register
                {
                    let src = xmm_index(instr.op_register(1));
                    let addr = eval_mem_addr(regs, &instr, 0);
                    sse_write_64(memory, addr, &regs.xmm[src][8..16]);
                }
                regs.eip += len;
            }

            // --- SSE scalar arithmetic (low 32-bit float, upper preserved) ---
            Addss | Subss | Mulss | Divss | Sqrtss | Minss | Maxss => {
                let dst = xmm_index(instr.op_register(0));
                let a = f32::from_le_bytes(regs.xmm[dst][..4].try_into().unwrap());
                let b = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_f32(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f32::from_le_bytes(regs.xmm[src][..4].try_into().unwrap())
                };
                let result = match instr.mnemonic() {
                    Addss => a + b,
                    Subss => a - b,
                    Mulss => a * b,
                    Divss => {
                        if b != 0.0 {
                            a / b
                        } else {
                            0.0
                        }
                    }
                    Sqrtss => b.sqrt(),
                    Minss => a.min(b),
                    Maxss => a.max(b),
                    _ => unreachable!(),
                };
                regs.xmm[dst][..4].copy_from_slice(&result.to_le_bytes());
                regs.eip += len;
            }

            // --- SSE packed arithmetic (all four 32-bit floats) ---
            Addps | Subps | Mulps | Divps => {
                let dst = xmm_index(instr.op_register(0));
                let src_bytes = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_128(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    regs.xmm[src]
                };
                for i in 0..4 {
                    let off = i * 4;
                    let a = f32::from_le_bytes(regs.xmm[dst][off..off + 4].try_into().unwrap());
                    let b = f32::from_le_bytes(src_bytes[off..off + 4].try_into().unwrap());
                    let r = match instr.mnemonic() {
                        Addps => a + b,
                        Subps => a - b,
                        Mulps => a * b,
                        Divps => {
                            if b != 0.0 {
                                a / b
                            } else {
                                0.0
                            }
                        }
                        _ => unreachable!(),
                    };
                    regs.xmm[dst][off..off + 4].copy_from_slice(&r.to_le_bytes());
                }
                regs.eip += len;
            }

            // --- SSE compare (set EFLAGS from float compare) ---
            Comiss | Ucomiss => {
                let a_idx = xmm_index(instr.op_register(0));
                let a = f32::from_le_bytes(regs.xmm[a_idx][..4].try_into().unwrap());
                let b = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_f32(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f32::from_le_bytes(regs.xmm[src][..4].try_into().unwrap())
                };
                // comiss sets ZF, PF, CF (like x87 FCOM → SAHF)
                let (zf, pf, cf) = if a.is_nan() || b.is_nan() {
                    (1u32, 1u32, 1u32) // unordered
                } else if a > b {
                    (0, 0, 0)
                } else if a < b {
                    (0, 0, 1)
                } else {
                    (1, 0, 0) // equal
                };
                regs.eflags = (regs.eflags & !0x45) | (zf << 6) | (pf << 2) | cf;
                regs.eip += len;
            }

            // --- SSE conversion ---
            Cvtsi2ss => {
                // cvtsi2ss xmm, r/m32 — int to float
                let dst = xmm_index(instr.op_register(0));
                let val = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    memory.read_u32(addr) as i32
                } else {
                    regs.get(instr.op_register(1)) as i32
                };
                let f = val as f32;
                regs.xmm[dst][..4].copy_from_slice(&f.to_le_bytes());
                regs.eip += len;
            }
            Cvtss2si => {
                // cvtss2si r32, xmm/m32 — float to int (round)
                let f = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_f32(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f32::from_le_bytes(regs.xmm[src][..4].try_into().unwrap())
                };
                regs.set(instr.op_register(0), f.round() as i32 as u32);
                regs.eip += len;
            }
            Cvttss2si => {
                // cvttss2si r32, xmm/m32 — float to int (truncate)
                let f = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_f32(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f32::from_le_bytes(regs.xmm[src][..4].try_into().unwrap())
                };
                regs.set(instr.op_register(0), f as i32 as u32);
                regs.eip += len;
            }
            Cvtsi2sd => {
                // cvtsi2sd xmm, r/m32 — int to double
                let dst = xmm_index(instr.op_register(0));
                let val = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    memory.read_u32(addr) as i32
                } else {
                    regs.get(instr.op_register(1)) as i32
                };
                let d = val as f64;
                regs.xmm[dst][..8].copy_from_slice(&d.to_le_bytes());
                regs.eip += len;
            }
            Cvtsd2si => {
                let d = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    f64::from_le_bytes(sse_read_64(memory, addr).try_into().unwrap())
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f64::from_le_bytes(regs.xmm[src][..8].try_into().unwrap())
                };
                regs.set(instr.op_register(0), d.round() as i32 as u32);
                regs.eip += len;
            }
            Cvttsd2si => {
                let d = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    f64::from_le_bytes(sse_read_64(memory, addr).try_into().unwrap())
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f64::from_le_bytes(regs.xmm[src][..8].try_into().unwrap())
                };
                regs.set(instr.op_register(0), d as i32 as u32);
                regs.eip += len;
            }
            Cvtss2sd => {
                // cvtss2sd xmm, xmm/m32 — float to double
                let dst = xmm_index(instr.op_register(0));
                let f = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_f32(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f32::from_le_bytes(regs.xmm[src][..4].try_into().unwrap())
                };
                let d = f as f64;
                regs.xmm[dst][..8].copy_from_slice(&d.to_le_bytes());
                regs.eip += len;
            }
            Cvtsd2ss => {
                // cvtsd2ss xmm, xmm/m64 — double to float
                let dst = xmm_index(instr.op_register(0));
                let d = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    f64::from_le_bytes(sse_read_64(memory, addr).try_into().unwrap())
                } else {
                    let src = xmm_index(instr.op_register(1));
                    f64::from_le_bytes(regs.xmm[src][..8].try_into().unwrap())
                };
                let f = d as f32;
                regs.xmm[dst][..4].copy_from_slice(&f.to_le_bytes());
                regs.eip += len;
            }

            // --- SSE bitwise ---
            Xorps | Andps | Orps => {
                let dst = xmm_index(instr.op_register(0));
                let src_bytes = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_128(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    regs.xmm[src]
                };
                for i in 0..16 {
                    regs.xmm[dst][i] = match instr.mnemonic() {
                        Xorps => regs.xmm[dst][i] ^ src_bytes[i],
                        Andps => regs.xmm[dst][i] & src_bytes[i],
                        Orps => regs.xmm[dst][i] | src_bytes[i],
                        _ => unreachable!(),
                    };
                }
                regs.eip += len;
            }

            // --- SSE shuffle/unpack ---
            Shufps => {
                let dst_idx = xmm_index(instr.op_register(0));
                let src_bytes = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_128(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    regs.xmm[src]
                };
                let imm = instr.immediate8();
                let dst_copy = regs.xmm[dst_idx];
                // Result[0] = dst[(imm>>0)&3], Result[1] = dst[(imm>>2)&3]
                // Result[2] = src[(imm>>4)&3], Result[3] = src[(imm>>6)&3]
                for i in 0..2 {
                    let sel = ((imm >> (i * 2)) & 3) as usize;
                    regs.xmm[dst_idx][i * 4..i * 4 + 4]
                        .copy_from_slice(&dst_copy[sel * 4..sel * 4 + 4]);
                }
                for i in 2..4 {
                    let sel = ((imm >> (i * 2)) & 3) as usize;
                    regs.xmm[dst_idx][i * 4..i * 4 + 4]
                        .copy_from_slice(&src_bytes[sel * 4..sel * 4 + 4]);
                }
                regs.eip += len;
            }
            Unpcklps => {
                let dst_idx = xmm_index(instr.op_register(0));
                let src_bytes = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_128(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    regs.xmm[src]
                };
                let d = regs.xmm[dst_idx];
                // [dst0, src0, dst1, src1]
                regs.xmm[dst_idx][0..4].copy_from_slice(&d[0..4]);
                regs.xmm[dst_idx][4..8].copy_from_slice(&src_bytes[0..4]);
                regs.xmm[dst_idx][8..12].copy_from_slice(&d[4..8]);
                regs.xmm[dst_idx][12..16].copy_from_slice(&src_bytes[4..8]);
                regs.eip += len;
            }
            Unpckhps => {
                let dst_idx = xmm_index(instr.op_register(0));
                let src_bytes = if instr.op_kind(1) == iced_x86::OpKind::Memory {
                    let addr = eval_mem_addr(regs, &instr, 1);
                    sse_read_128(memory, addr)
                } else {
                    let src = xmm_index(instr.op_register(1));
                    regs.xmm[src]
                };
                let d = regs.xmm[dst_idx];
                // [dst2, src2, dst3, src3]
                regs.xmm[dst_idx][0..4].copy_from_slice(&d[8..12]);
                regs.xmm[dst_idx][4..8].copy_from_slice(&src_bytes[8..12]);
                regs.xmm[dst_idx][8..12].copy_from_slice(&d[12..16]);
                regs.xmm[dst_idx][12..16].copy_from_slice(&src_bytes[12..16]);
                regs.eip += len;
            }

            // --- CMOVcc (conditional move) ---
            Cmove | Cmovne | Cmovb | Cmovae | Cmovl | Cmovge | Cmovle | Cmovg | Cmovs | Cmovns
            | Cmovbe | Cmova | Cmovo | Cmovno | Cmovp | Cmovnp => {
                let cond = match instr.mnemonic() {
                    Cmove => regs.flag_zf(),
                    Cmovne => !regs.flag_zf(),
                    Cmovb => regs.flag_cf(),
                    Cmovae => !regs.flag_cf(),
                    Cmovs => regs.flag_sf(),
                    Cmovns => !regs.flag_sf(),
                    Cmovo => regs.flag_of(),
                    Cmovno => !regs.flag_of(),
                    Cmovl => regs.flag_sf() != regs.flag_of(),
                    Cmovge => regs.flag_sf() == regs.flag_of(),
                    Cmovle => regs.flag_zf() || (regs.flag_sf() != regs.flag_of()),
                    Cmovg => !regs.flag_zf() && (regs.flag_sf() == regs.flag_of()),
                    Cmovbe => regs.flag_cf() || regs.flag_zf(),
                    Cmova => !regs.flag_cf() && !regs.flag_zf(),
                    Cmovp => regs.eflags & 4 != 0,
                    Cmovnp => regs.eflags & 4 == 0,
                    _ => false,
                };
                if cond {
                    let val = eval_operand(memory, regs, &instr, 1);
                    store_operand(memory, regs, &instr, 0, val);
                }
                regs.eip += len;
            }

            // --- Debug breakpoint (INT3 / 0xCC) ---
            // Retail Xbox games may contain debug INT3 left by developers.
            // On real Xbox, the kernel debugger catches these. We skip them.
            Int3 => {
                regs.eip += len;
            }

            // --- Bswap (byte swap) ---
            Bswap => {
                let val = eval_operand(memory, regs, &instr, 0);
                let swapped = val.swap_bytes();
                store_operand(memory, regs, &instr, 0, swapped);
                regs.eip += len;
            }

            // --- Unhandled ---
            _ => {
                return_with_snapshot!(
                    InterpResult::Unhandled {
                        eip,
                        mnemonic: format!("{:?}", instr.mnemonic()),
                    },
                    1,
                    eip
                );
            }
        }
    }
}

fn fpu_reg_index(regs: &X86Regs, reg: iced_x86::Register) -> Option<usize> {
    let reg = reg as usize;
    let st0 = iced_x86::Register::ST0 as usize;
    let st7 = iced_x86::Register::ST7 as usize;
    if (st0..=st7).contains(&reg) {
        Some((regs.fpu_top + reg - st0) % 8)
    } else {
        None
    }
}

fn fpu_read_float(memory: &GuestMemory, addr: u32, size: iced_x86::MemorySize) -> f64 {
    if !is_safe_addr(addr) {
        return 0.0;
    }
    match size {
        iced_x86::MemorySize::Float64 => {
            let lo = memory.read_u32(addr) as u64;
            let hi = memory.read_u32(addr.wrapping_add(4)) as u64;
            f64::from_bits(lo | (hi << 32))
        }
        _ => f32::from_bits(memory.read_u32(addr)) as f64,
    }
}

/// Evaluate an operand value
fn eval_operand(
    memory: &GuestMemory,
    regs: &X86Regs,
    instr: &iced_x86::Instruction,
    op_idx: u32,
) -> u32 {
    use iced_x86::OpKind;
    match instr.op_kind(op_idx) {
        OpKind::Register => regs.get(instr.op_register(op_idx)),
        OpKind::Immediate8 | OpKind::Immediate8to32 | OpKind::Immediate8to64 => {
            instr.immediate8to32() as u32
        }
        OpKind::Immediate16 => instr.immediate16() as u32,
        OpKind::Immediate32 | OpKind::Immediate32to64 => instr.immediate32(),
        OpKind::Memory => {
            let addr = eval_mem_addr(regs, instr, op_idx);
            // Bounds check: only access committed guest memory (0-0x1FFFFFFF or 0x80000000-0x9FFFFFFF)
            let safe = is_safe_addr(addr);
            if !safe {
                return 0;
            } // return 0 for unmapped reads
            match instr.memory_size() {
                iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => {
                    memory.read_u8(addr) as u32
                }
                iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => {
                    memory.read_u16(addr) as u32
                }
                _ => memory.read_u32(addr),
            }
        }
        OpKind::NearBranch32 => instr.near_branch32(),
        _ => 0,
    }
}

/// Store a value to an operand
fn store_operand(
    memory: &GuestMemory,
    regs: &mut X86Regs,
    instr: &iced_x86::Instruction,
    op_idx: u32,
    val: u32,
) {
    use iced_x86::OpKind;
    match instr.op_kind(op_idx) {
        OpKind::Register => regs.set(instr.op_register(op_idx), val),
        OpKind::Memory => {
            let addr = eval_mem_addr(regs, instr, op_idx);
            let safe = addr < 0x2000_0000
                || (addr >= 0x8000_0000 && addr < 0xA000_0000)
                || (addr >= 0x0C00_0000 && addr < 0x0C01_0000);
            if !safe {
                return;
            } // silently drop writes to unmapped memory
            match instr.memory_size() {
                iced_x86::MemorySize::UInt8 | iced_x86::MemorySize::Int8 => {
                    memory.write_u8(addr, val as u8)
                }
                iced_x86::MemorySize::UInt16 | iced_x86::MemorySize::Int16 => {
                    memory.write_u16(addr, val as u16)
                }
                _ => memory.write_u32(addr, val),
            }
        }
        _ => {}
    }
}

/// Calculate effective address for a memory operand
fn eval_mem_addr(regs: &X86Regs, instr: &iced_x86::Instruction, op_idx: u32) -> u32 {
    let mut addr: u32 = 0;

    // Base register
    if instr.memory_base() != iced_x86::Register::None {
        addr = addr.wrapping_add(regs.get(instr.memory_base()));
    }

    // Index register * scale
    if instr.memory_index() != iced_x86::Register::None {
        let idx = regs.get(instr.memory_index());
        addr = addr.wrapping_add(idx.wrapping_mul(instr.memory_index_scale() as u32));
    }

    // Displacement
    let disp = instr.memory_displacement32();
    addr = addr.wrapping_add(disp);

    // FS segment → add KPCR base (0x0C000000), matching AOT emitter's FAKE_KPCR_BASE_ADDR
    if instr.segment_prefix() == iced_x86::Register::FS
        || instr.segment_prefix() == iced_x86::Register::GS
    {
        addr = addr.wrapping_add(0x0C00_0000);
    }

    addr
}

/// Check if guest address is in committed memory (safe to read/write)
fn is_safe_addr(addr: u32) -> bool {
    addr < 0x8000_0000 // 0-2GB: RAM (0-512MB) + gap (512MB-2GB, committed)
        || (addr >= 0x8000_0000 && addr < 0xA000_0000) // mirror
        || (addr >= 0x0C00_0000 && addr < 0x0C01_0000) // KPCR
}

/// Get XMM register index (0-7) from iced_x86 Register
fn xmm_index(reg: iced_x86::Register) -> usize {
    use iced_x86::Register::*;
    match reg {
        XMM0 => 0,
        XMM1 => 1,
        XMM2 => 2,
        XMM3 => 3,
        XMM4 => 4,
        XMM5 => 5,
        XMM6 => 6,
        XMM7 => 7,
        _ => 0,
    }
}

/// Read 32-bit float from guest memory
fn sse_read_f32(memory: &GuestMemory, addr: u32) -> f32 {
    if !is_safe_addr(addr) {
        return 0.0;
    }
    f32::from_le_bytes(memory.read_u32(addr).to_le_bytes())
}

/// Write 32-bit float to guest memory
fn sse_write_f32(memory: &GuestMemory, addr: u32, val: f32) {
    if !is_safe_addr(addr) {
        return;
    }
    memory.write_u32(addr, u32::from_le_bytes(val.to_le_bytes()));
}

/// Read 128 bits from guest memory
fn sse_read_128(memory: &GuestMemory, addr: u32) -> [u8; 16] {
    if !is_safe_addr(addr) {
        return [0u8; 16];
    }
    let mut buf = [0u8; 16];
    for i in 0..4 {
        let w = memory.read_u32(addr.wrapping_add(i as u32 * 4));
        buf[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    buf
}

/// Write 128 bits to guest memory
fn sse_write_128(memory: &GuestMemory, addr: u32, data: &[u8; 16]) {
    if !is_safe_addr(addr) {
        return;
    }
    for i in 0..4 {
        let w = u32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
        memory.write_u32(addr.wrapping_add(i as u32 * 4), w);
    }
}

/// Read 64 bits from guest memory
fn sse_read_64(memory: &GuestMemory, addr: u32) -> [u8; 8] {
    if !is_safe_addr(addr) {
        return [0u8; 8];
    }
    let mut buf = [0u8; 8];
    for i in 0..2 {
        let w = memory.read_u32(addr.wrapping_add(i as u32 * 4));
        buf[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    buf
}

/// Write 64 bits to guest memory
fn sse_write_64(memory: &GuestMemory, addr: u32, data: &[u8]) {
    if !is_safe_addr(addr) {
        return;
    }
    for i in 0..2 {
        if i * 4 + 4 <= data.len() {
            let w = u32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
            memory.write_u32(addr.wrapping_add(i as u32 * 4), w);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{preserves_doom_init_loop, taken_backward_jump_is_generic_spin_candidate};

    #[test]
    fn infra_regression_doom_long_init_loops_are_spin_break_exempt() {
        let exempt_pairs = [
            (0x0001_9CE0, 0x0001_9C70),
            (0x0001_9D1B, 0x0001_9D12),
            (0x0001_9D3B, 0x0001_9D00),
            (0x0001_9D90, 0x0001_9D50),
            (0x0002_EEA7, 0x0002_EE30),
        ];

        for (eip, target) in exempt_pairs {
            assert!(preserves_doom_init_loop(eip, target));
            assert!(!taken_backward_jump_is_generic_spin_candidate(eip, target));
        }
    }

    #[test]
    fn infra_regression_nearby_or_unknown_backward_jumps_still_count_as_spin_candidates() {
        assert!(!preserves_doom_init_loop(0x0001_9CE1, 0x0001_9C70));
        assert!(taken_backward_jump_is_generic_spin_candidate(
            0x0001_9CE1,
            0x0001_9C70
        ));

        assert!(!preserves_doom_init_loop(0x0003_0000, 0x0002_FFF0));
        assert!(taken_backward_jump_is_generic_spin_candidate(
            0x0003_0000,
            0x0002_FFF0
        ));

        assert!(!taken_backward_jump_is_generic_spin_candidate(
            0x0002_0000,
            0x0002_0010
        ));
    }
}
