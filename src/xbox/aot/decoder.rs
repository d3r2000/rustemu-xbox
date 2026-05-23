/// x86-32 instruction decoder using iced-x86.
/// Linear sweep decode with instruction classification.
use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrCategory {
    Alu,
    Mov,
    Stack,
    Branch,
    Memory,
    Shift,
    MulDiv,
    String,
    Fpu,
    Sse,
    System,
    Other,
}

pub struct DecodedInstr {
    pub guest_addr: u32,
    pub length: u8,
    pub instruction: Instruction,
    pub category: InstrCategory,
    pub has_mmio_risk: bool,
    pub branch_target: Option<u32>,
}

#[derive(Debug, Default)]
pub struct DecodeStats {
    pub total: u32,
    pub decode_errors: u32,
    pub cat_alu: u32,
    pub cat_mov: u32,
    pub cat_stack: u32,
    pub cat_branch: u32,
    pub cat_memory: u32,
    pub cat_shift: u32,
    pub cat_mul_div: u32,
    pub cat_string: u32,
    pub cat_fpu: u32,
    pub cat_sse: u32,
    pub cat_system: u32,
    pub cat_other: u32,
}

/// Decode a section of x86-32 code. Linear sweep.
pub fn decode_section(code: &[u8], base_addr: u32) -> (Vec<DecodedInstr>, DecodeStats) {
    let mut decoder = Decoder::with_ip(32, code, base_addr as u64, DecoderOptions::NONE);
    let mut instrs = Vec::with_capacity(code.len() / 3); // ~3 bytes/instr average for x86
    let mut stats = DecodeStats::default();
    let mut instr = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instr);
        let guest_addr = instr.ip() as u32;
        let length = instr.len() as u8;

        if instr.is_invalid() {
            stats.decode_errors += 1;
            continue;
        }

        stats.total += 1;
        let category = classify_mnemonic(instr.mnemonic());
        match category {
            InstrCategory::Alu => stats.cat_alu += 1,
            InstrCategory::Mov => stats.cat_mov += 1,
            InstrCategory::Stack => stats.cat_stack += 1,
            InstrCategory::Branch => stats.cat_branch += 1,
            InstrCategory::Memory => stats.cat_memory += 1,
            InstrCategory::Shift => stats.cat_shift += 1,
            InstrCategory::MulDiv => stats.cat_mul_div += 1,
            InstrCategory::String => stats.cat_string += 1,
            InstrCategory::Fpu => stats.cat_fpu += 1,
            InstrCategory::Sse => stats.cat_sse += 1,
            InstrCategory::System => stats.cat_system += 1,
            InstrCategory::Other => stats.cat_other += 1,
        }

        let has_mmio_risk = check_mmio_risk(&instr);

        let branch_target = if instr.is_jcc_short_or_near()
            || instr.mnemonic() == Mnemonic::Jmp
            || instr.mnemonic() == Mnemonic::Call
        {
            let target = instr.near_branch_target();
            if target != 0 {
                Some(target as u32)
            } else {
                None
            }
        } else {
            None
        };

        instrs.push(DecodedInstr {
            guest_addr,
            length,
            instruction: instr,
            category,
            has_mmio_risk,
            branch_target,
        });
    }

    (instrs, stats)
}

/// Compute the set of guest addresses reachable from seed points via direct
/// control flow (E2 fix, 2026-04-22). Recursive descent: for each reachable
/// address, follow (a) the fallthrough successor unless the instruction is
/// a terminator (RET, unconditional JMP, INT), and (b) the direct branch
/// target if known.
///
/// Used by emulator.rs to filter which decoded-instruction addresses are
/// inserted into the trusted `addr_hash`. Addresses NOT in the returned set
/// are either dead bytes (padding between functions), data that happens to
/// live in an EXEC-flagged section (KeDpc structs, INIT tables), or code
/// reachable only via indirect dispatch (function-pointer tables, switch
/// jumps). For the first two classes, skipping the hash insert prevents
/// stack-scan / indirect-CALL resolution from trusting them. For the third,
/// the VEH rescue_emit path picks them up on first call.
///
/// `seeds`: entry point addresses to start the walk from. Typically the XBE
/// entry point, plus any addresses known from symbol/OOVPA scans. Addresses
/// that are in `instrs` serve as the anchor — seeds outside `instrs`'
/// range are ignored.
///
/// This does NOT discover jump-table targets (switch-statement dispatch
/// through `jmp [reg*4+disp]`). Those become rescue_emit first-hits at
/// runtime. If a future game proves that cost too high, extract jump
/// tables via a separate pre-scan pass.
pub fn compute_reachable(instrs: &[DecodedInstr], seeds: &[u32]) -> std::collections::HashSet<u32> {
    use iced_x86::FlowControl;
    use std::collections::{HashMap, HashSet};

    // Build addr -> index for fast lookup.
    let mut idx_by_addr: HashMap<u32, usize> = HashMap::with_capacity(instrs.len());
    for (i, di) in instrs.iter().enumerate() {
        idx_by_addr.insert(di.guest_addr, i);
    }

    let mut reachable: HashSet<u32> = HashSet::with_capacity(instrs.len());
    let mut worklist: Vec<u32> = seeds
        .iter()
        .copied()
        .filter(|a| idx_by_addr.contains_key(a))
        .collect();

    while let Some(addr) = worklist.pop() {
        if !reachable.insert(addr) {
            continue; // already visited
        }
        let Some(&i) = idx_by_addr.get(&addr) else {
            continue;
        };
        let di = &instrs[i];
        let fc = di.instruction.flow_control();

        // Fallthrough: everything except terminators continues to the next insn.
        let is_terminator = matches!(
            fc,
            FlowControl::Return
                | FlowControl::UnconditionalBranch
                | FlowControl::Interrupt
                | FlowControl::Exception
        );
        if !is_terminator {
            let next = addr.wrapping_add(di.length as u32);
            if idx_by_addr.contains_key(&next) {
                worklist.push(next);
            }
        }

        // Direct branch / call target (already extracted by decode_section).
        if let Some(target) = di.branch_target {
            if idx_by_addr.contains_key(&target) {
                worklist.push(target);
            }
        }
    }

    reachable
}

fn check_mmio_risk(instr: &Instruction) -> bool {
    for i in 0..instr.op_count() {
        if instr.op_kind(i) == iced_x86::OpKind::Memory {
            let disp = instr.memory_displacement64() as u32;
            // NV2A register ranges
            if (0xFD00_0000..=0xFDFF_FFFF).contains(&disp)
                || (0xF000_0000..=0xF100_0000).contains(&disp)
                || (0xFEC0_0000..=0xFED0_0000).contains(&disp)
            {
                return true;
            }
        }
    }
    false
}

fn classify_mnemonic(m: Mnemonic) -> InstrCategory {
    use Mnemonic::*;
    match m {
        // ALU
        Add | Sub | And | Or | Xor | Cmp | Test | Not | Neg | Inc | Dec | Adc | Sbb | Bt | Bts
        | Btr | Btc | Bsf | Bsr | Bswap => InstrCategory::Alu,

        // MOV
        Mov | Movzx | Movsx | Lea | Xchg | Cdq | Cwd | Cbw | Cwde | Cmova | Cmovae | Cmovb
        | Cmovbe | Cmove | Cmovg | Cmovge | Cmovl | Cmovle | Cmovne | Cmovno | Cmovnp | Cmovns
        | Cmovo | Cmovp | Cmovs => InstrCategory::Mov,

        // Stack
        Push | Pop | Pushfd | Popfd | Pushad | Popad => InstrCategory::Stack,

        // Branch
        Jmp | Call | Ret | Retf | Ja | Jae | Jb | Jbe | Je | Jg | Jge | Jl | Jle | Jne | Jno
        | Jnp | Jns | Jo | Jp | Js | Jcxz | Jecxz | Loop | Loope | Loopne => InstrCategory::Branch,

        // Shift
        Shl | Shr | Sar | Rol | Ror | Rcl | Rcr | Shld | Shrd => InstrCategory::Shift,

        // Mul/Div
        Mul | Imul | Div | Idiv => InstrCategory::MulDiv,

        // String
        Movsb | Movsw | Movsd | Movsq | Stosb | Stosw | Stosd | Stosq | Scasb | Scasw | Scasd
        | Cmpsb | Cmpsw | Cmpsd | Lodsb | Lodsw | Lodsd => InstrCategory::String,

        // System
        Int | Int3 | Sysenter | Sysexit | Cli | Sti | Hlt | Cpuid | Rdtsc | Wbinvd | Invlpg => {
            InstrCategory::System
        }

        // FPU (x87) — check range
        _ if is_fpu(m) => InstrCategory::Fpu,

        // SSE
        _ if is_sse(m) => InstrCategory::Sse,

        _ => InstrCategory::Other,
    }
}

fn is_fpu(m: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(
        m,
        Fld | Fst
            | Fstp
            | Fild
            | Fist
            | Fistp
            | Fisttp
            | Fadd
            | Faddp
            | Fiadd
            | Fsub
            | Fsubp
            | Fisub
            | Fsubr
            | Fsubrp
            | Fisubr
            | Fmul
            | Fmulp
            | Fimul
            | Fdiv
            | Fdivp
            | Fidiv
            | Fdivr
            | Fdivrp
            | Fidivr
            | Fchs
            | Fabs
            | Fsqrt
            | Fprem
            | Fprem1
            | Frndint
            | Fscale
            | Fsin
            | Fcos
            | Fsincos
            | Fptan
            | Fpatan
            | F2xm1
            | Fyl2x
            | Fyl2xp1
            | Fcom
            | Fcomp
            | Fcompp
            | Fucom
            | Fucomp
            | Fucompp
            | Fcomi
            | Fcomip
            | Fucomi
            | Fucomip
            | Ftst
            | Fxam
            | Fldz
            | Fld1
            | Fldpi
            | Fldl2e
            | Fldl2t
            | Fldlg2
            | Fldln2
            | Fxch
            | Fcmovb
            | Fcmovbe
            | Fcmove
            | Fcmovnb
            | Fcmovnbe
            | Fcmovne
            | Fcmovnu
            | Fcmovu
            | Fnop
            | Fnclex
            | Fclex
            | Fninit
            | Finit
            | Fnsave
            | Fsave
            | Fnstenv
            | Fstenv
            | Frstor
            | Fldenv
            | Fnstcw
            | Fstcw
            | Fldcw
            | Fnstsw
            | Fstsw
            | Fdecstp
            | Fincstp
            | Ffree
            | Wait
    )
}

fn is_sse(m: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(
        m,
        // SSE moves
        Movaps | Movups | Movss | Movapd | Movupd | Movsd | Movlps | Movhps |
        Movlpd | Movhpd | Movmskps | Movmskpd | Movdqa | Movdqu | Movq | Movd |
        Movlhps | Movhlps | Movntps | Movntpd | Movnti | Movntdq |
        // SSE arithmetic
        Addps | Addss | Addpd | Addsd | Subps | Subss | Subpd | Subsd |
        Mulps | Mulss | Mulpd | Mulsd | Divps | Divss | Divpd | Divsd |
        Sqrtps | Sqrtss | Sqrtpd | Sqrtsd | Rcpps | Rcpss | Rsqrtps | Rsqrtss |
        Maxps | Maxss | Maxpd | Maxsd | Minps | Minss | Minpd | Minsd |
        // SSE compare
        Cmpps | Cmpss | Cmppd | Cmpsd | Comiss | Comisd | Ucomiss | Ucomisd |
        // SSE logical
        Andps | Andpd | Andnps | Andnpd | Orps | Orpd | Xorps | Xorpd |
        // SSE shuffle/unpack
        Shufps | Shufpd | Unpckhps | Unpcklps | Unpckhpd | Unpcklpd |
        // SSE convert
        Cvtsi2ss | Cvtsi2sd | Cvtss2si | Cvtsd2si | Cvttss2si | Cvttsd2si |
        Cvtss2sd | Cvtsd2ss | Cvtps2pd | Cvtpd2ps |
        Cvtdq2ps | Cvtps2dq | Cvttps2dq | Cvtdq2pd | Cvtpd2dq | Cvttpd2dq |
        // SSE2 integer
        Paddb | Paddw | Paddd | Paddq | Psubb | Psubw | Psubd | Psubq |
        Pmullw | Pmulhw | Pmulhuw | Pmuludq |
        Pand | Pandn | Por | Pxor |
        Pcmpeqb | Pcmpeqw | Pcmpeqd | Pcmpgtb | Pcmpgtw | Pcmpgtd |
        Psllw | Pslld | Psllq | Psrlw | Psrld | Psrlq | Psraw | Psrad |
        Punpckhbw | Punpckhwd | Punpckhdq | Punpcklbw | Punpcklwd | Punpckldq |
        Packuswb | Packsswb | Packssdw |
        Pshufd | Pshufhw | Pshuflw | Pshufw |
        // SSE misc
        Ldmxcsr | Stmxcsr | Prefetchnta | Prefetcht0 | Prefetcht1 | Prefetcht2 |
        Sfence | Lfence | Mfence | Pause | Emms
    )
}
