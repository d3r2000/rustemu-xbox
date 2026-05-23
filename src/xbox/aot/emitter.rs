/// x86-32 → x64 AOT emitter.
/// Two-pass architecture: sizing → emission → fixup.
/// Direct addressing for branches; INT3 fallback only for unresolved/kernel.
///
/// Memory safety: uses iced-x86 Encoder for all memory operand rewrites,
/// eliminating manual REX/ModRM/SIB byte construction.
use crate::xbox::aot::decoder::{DecodedInstr, InstrCategory};
use crate::xbox::aot::runtime::{AddrEntry, BranchFixup, TrapInfo, TrapType};
use crate::xbox::memory::guest_memory::GuestMemory;
use iced_x86::{Encoder, Instruction, Mnemonic, OpKind, Register};

// ============================================================================
// Emission strategy
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmitStrategy {
    Identity,       // no memory operand, no ESP — copy verbatim
    RewriteMem,     // ANY memory operand → iced-x86 rewrite with R15 base
    RewriteEspReg,  // ESP as register operand (no memory) → replace ESP with R14D
    RewritePushReg, // PUSH reg → LEA R14D,[R14-4]; MOV [R15+R14],reg
    RewritePopReg,  // POP reg → MOV reg,[R15+R14]; LEA R14D,[R14+4]
    RewritePushEsp, // PUSH ESP → push PRE-decrement R14 (Intel 286+ semantics)
    RewritePopEsp,  // POP ESP → MOV R14D,[R15+R14] (load overrides +4)
    RewritePushImm, // PUSH imm → LEA R14D,[R14-4]; MOV DWORD [R15+R14],imm32
    RewriteCallRel, // CALL rel32 → push ret addr + shadow push + JMP
    RewriteRet,     // RET → shadow stack pop + JMP R10
    RewriteLeave,   // LEAVE → MOV R14D,EBP; MOV EBP,[R15+R14]; LEA R14D,[R14+4]
    RewritePushMem, // PUSH [mem] → read via R15, push to R14 guest stack
    RewritePopMem,  // POP [mem] → pop from R14 guest stack, write via R15
    RewriteIncDec,  // 0x40-0x4F → FF /0 or FF /1 (long form)
    RewriteString,  // string ops → wrap with R15 add/sub
    TrapIndirect,   // indirect JMP/CALL → INT3
    TrapSystem,     // system instruction → INT3
    Skip,           // NOP → emit nothing
}

// ============================================================================
// Emitted block output
// ============================================================================

#[derive(Debug, Default)]
pub struct EmitStats {
    pub identity: u32,
    pub rewrite_mem: u32,
    pub rewrite_esp_reg: u32,
    pub rewrite_push: u32,
    pub rewrite_pop: u32,
    pub rewrite_call: u32,
    pub rewrite_ret: u32,
    pub rewrite_leave: u32,
    pub rewrite_incdec: u32,
    pub rewrite_string: u32,
    pub trap_indirect: u32,
    pub trap_system: u32,
    pub skip: u32,
    pub total: u32,
}

pub struct EmittedBlock {
    pub code: Vec<u8>,
    pub addr_map: Vec<AddrEntry>,
    pub traps: Vec<TrapInfo>,
    pub unresolved: Vec<BranchFixup>,
    pub stats: EmitStats,
    pub encode_fail_count: u32,
    pub encode_fail_samples: Vec<(u32, String)>,
}

// ============================================================================
// Main emission function
// ============================================================================

/// Emit x64 code for a decoded section.
/// `guest_code` is the raw bytes of the guest section (for identity copies).
/// `memory` is used to resolve indirect CALL targets through kernel thunk tables.
pub fn emit_section(
    instrs: &[DecodedInstr],
    guest_code: &[u8],
    section_base: u32,
    memory: &GuestMemory,
    hash_shift: u8,
    thunk_entries: &[(u32, u32)],
) -> EmittedBlock {
    let estimated = guest_code.len() * 3 + 65536; // 3x for inline RET (86 bytes vs 1)
    let mut code = Vec::with_capacity(estimated);
    let mut addr_map = Vec::with_capacity(instrs.len());
    let mut traps = Vec::new();
    let mut fixups: Vec<BranchFixup> = Vec::new();
    let mut stats = EmitStats::default();
    let mut failures = EncodeFailures::new();

    for di in instrs {
        let strategy = classify_for_emit(di);
        let host_offset = code.len() as u32;

        addr_map.push(AddrEntry {
            guest_addr: di.guest_addr,
            host_offset,
        });

        stats.total += 1;

        let file_offset = (di.guest_addr - section_base) as usize;
        let guest_bytes = &guest_code[file_offset..file_offset + di.length as usize];

        // Debug: trace _alloca_probe (0x2B6C00-0x2B6C10) and string allocator (0x46F10-0x46FA0)
        if (di.guest_addr >= 0x002B6C00 && di.guest_addr <= 0x002B6C10)
            || (di.guest_addr >= 0x00046F10 && di.guest_addr <= 0x00046F31)
        {
            crate::xbox::emulator::debug_log(&format!(
                "EMIT-ALLOCA: guest=0x{:08X} mnemonic={:?} strategy={:?} bytes={:02X?} esp_op={}",
                di.guest_addr,
                di.instruction.mnemonic(),
                strategy,
                guest_bytes,
                has_esp_register_operand(&di.instruction)
            ));
        }
        // Debug: trace the crash address area
        if di.guest_addr >= 0x002BA8E0 && di.guest_addr <= 0x002BA8F0 {
            let pre_len = code.len();
            crate::xbox::emulator::debug_log(&format!(
                "EMIT-TRACE: guest=0x{:08X} len={} bytes={:02X?} strategy={:?} mnemonic={:?} has_mem={} base={:?} idx={:?} scale={}",
                di.guest_addr, di.length, guest_bytes, strategy,
                di.instruction.mnemonic(), has_memory_operand(&di.instruction),
                di.instruction.memory_base(), di.instruction.memory_index(),
                di.instruction.memory_index_scale()
            ));
            // After emission, log the emitted bytes
            // (We'll add a post-check after the match block)
        }
        let _pre_emit_len = code.len();

        // PUSHFD/POPFD — special handling before strategy dispatch
        // PUSHFD: LEA R14D,[R14-4]; PUSHFQ; POP R11; MOV [R15+R14],R11D (11 bytes)
        // LEA preserves flags so PUSHFQ captures correct pre-PUSHFD flags
        if di.instruction.mnemonic() == Mnemonic::Pushfd {
            code.extend_from_slice(&[
                0x45, 0x8D, 0x76, 0xFC, // LEA R14D, [R14-4]
                0x9C, // PUSHFQ (to host stack)
                0x41, 0x5B, // POP R11
                0x47, 0x89, 0x1C, 0x37, // MOV [R15+R14], R11D
            ]);
            stats.rewrite_push += 1;
            continue;
        }
        // POPFD: MOV R11D,[R15+R14]; LEA R14D,[R14+4]; PUSH R11; POPFQ (11 bytes)
        if di.instruction.mnemonic() == Mnemonic::Popfd {
            code.extend_from_slice(&[
                0x47, 0x8B, 0x1C, 0x37, // MOV R11D, [R15+R14]
                0x45, 0x8D, 0x76, 0x04, // LEA R14D, [R14+4]
                0x41, 0x53, // PUSH R11
                0x9D, // POPFQ
            ]);
            stats.rewrite_pop += 1;
            continue;
        }

        // P0 decode dump: DISABLED — was flooding log with ~10K lines during .text AOT.
        // Re-enable for targeted debugging only.

        // FONT-LOADER PROBES (2026-04-22): 22-call block exonerated
        // (commit 711aab0). New target: documented R14-drift chain in
        // CHILD-2 font loader:
        //   sub_2CDA0 (font loader entry, SEH-wrapped)
        //   sub_26220 (intermediate, thiscall-ish, calls sub_13DA0)
        //   sub_42580 (malloc thunk, reads [esp+4] and [esp+0xc])
        // Hypothesis: stdcall/cdecl mismatch in ret-N cleanup drops 4 bytes
        // from R14 on round-trip.
        //
        // Plant probes at each function's entry + first RET. Log full
        // state via VEH handler (same one as 22-call trace): ESP, EBP,
        // R12, [ESP]. Bounded 300 events.
        const CALL_BOUNDARY_SITES: &[u32] = &[];
        if CALL_BOUNDARY_SITES.contains(&di.guest_addr) {
            traps.push(TrapInfo {
                host_offset: code.len() as u32,
                trap_type: TrapType::StackTrace,
                ret_cleanup: 0,
                guest_addr: di.guest_addr,
            });
            code.push(0xCC); // CALL-BOUNDARY probe (one-shot-capped at 300)
        }

        // Thunk constant folding: when instruction reads from the kernel thunk table
        // (e.g. `mov esi, [0xB534C]`), emit `mov esi, imm32` with the resolved value.
        // This prevents runtime corruption from guest code zeroing .rdata thunk entries.
        if !thunk_entries.is_empty()
            && di.instruction.mnemonic() == Mnemonic::Mov
            && di.instruction.op0_kind() == OpKind::Register
            && di.instruction.op_count() >= 2
            && di.instruction.op_kind(1) == OpKind::Memory
            && di.instruction.memory_base() == Register::None
            && di.instruction.memory_index() == Register::None
        {
            let disp = di.instruction.memory_displacement32();
            // Thunk entries are contiguous 4-byte slots — direct index instead of linear scan
            let thunk_start = thunk_entries.first().map(|e| e.0).unwrap_or(0);
            let resolved_opt = if disp >= thunk_start && !thunk_entries.is_empty() {
                let idx = ((disp - thunk_start) / 4) as usize;
                thunk_entries.get(idx).filter(|e| e.0 == disp).map(|e| e.1)
            } else {
                None
            };
            if let Some(resolved) = resolved_opt {
                // Emit MOV reg32, imm32 instead of MOV reg32, [R15+disp32]
                let dst = di.instruction.op_register(0);
                let reg_code = match dst {
                    Register::EAX => 0xB8u8,
                    Register::ECX => 0xB9,
                    Register::EDX => 0xBA,
                    Register::EBX => 0xBB,
                    Register::ESP => 0xBC, // unlikely but complete
                    Register::EBP => 0xBD,
                    Register::ESI => 0xBE,
                    Register::EDI => 0xBF,
                    _ => 0, // unsupported — fall through to normal rewrite
                };
                if reg_code != 0 {
                    code.push(reg_code);
                    code.extend_from_slice(&resolved.to_le_bytes());
                    stats.rewrite_mem += 1;
                    continue;
                }
            }
        }

        match strategy {
            EmitStrategy::Identity => {
                code.extend_from_slice(guest_bytes);
                stats.identity += 1;
            }
            EmitStrategy::RewriteMem => {
                if !emit_memory_rewrite(&mut code, &di.instruction, guest_bytes) {
                    failures.record(di.guest_addr, di.instruction.mnemonic());
                    // Rescue trap: VEH will decode original guest bytes at runtime
                    traps.push(TrapInfo {
                        host_offset: code.len() as u32,
                        trap_type: TrapType::Rescue,
                        ret_cleanup: di.length as u16,
                        guest_addr: di.guest_addr,
                    });
                    code.push(0xCC);
                }
                stats.rewrite_mem += 1;
            }
            EmitStrategy::RewriteEspReg => {
                if !emit_esp_reg_rewrite(&mut code, &di.instruction) {
                    failures.record(di.guest_addr, di.instruction.mnemonic());
                    traps.push(TrapInfo {
                        host_offset: code.len() as u32,
                        trap_type: TrapType::Rescue,
                        ret_cleanup: di.length as u16,
                        guest_addr: di.guest_addr,
                    });
                    code.push(0xCC);
                }
                stats.rewrite_esp_reg += 1;
            }
            EmitStrategy::RewritePushReg => {
                emit_push_reg(&mut code, guest_bytes[0]);
                stats.rewrite_push += 1;
            }
            EmitStrategy::RewritePopReg => {
                emit_pop_reg(&mut code, guest_bytes[0]);
                stats.rewrite_pop += 1;
            }
            EmitStrategy::RewritePushEsp => {
                emit_push_esp(&mut code);
                stats.rewrite_push += 1;
            }
            EmitStrategy::RewritePopEsp => {
                emit_pop_esp(&mut code);
                stats.rewrite_pop += 1;
            }
            EmitStrategy::RewritePushImm => {
                emit_push_imm(&mut code, di);
                stats.rewrite_push += 1;
            }
            EmitStrategy::RewritePushMem => {
                if !emit_push_mem(&mut code, &di.instruction, guest_bytes) {
                    failures.record(di.guest_addr, di.instruction.mnemonic());
                    traps.push(TrapInfo {
                        host_offset: code.len() as u32,
                        trap_type: TrapType::Rescue,
                        ret_cleanup: di.length as u16,
                        guest_addr: di.guest_addr,
                    });
                    code.push(0xCC);
                }
                stats.rewrite_push += 1;
            }
            EmitStrategy::RewritePopMem => {
                if !emit_pop_mem(&mut code, &di.instruction, guest_bytes) {
                    failures.record(di.guest_addr, di.instruction.mnemonic());
                    traps.push(TrapInfo {
                        host_offset: code.len() as u32,
                        trap_type: TrapType::Rescue,
                        ret_cleanup: di.length as u16,
                        guest_addr: di.guest_addr,
                    });
                    code.push(0xCC);
                }
                stats.rewrite_pop += 1;
            }
            EmitStrategy::RewriteCallRel => {
                let ret_addr = di.guest_addr + di.length as u32;
                // HLE: intercept calls to _SEH_prolog (0x002B7B28) and
                // _SEH_epilog (0x002B7B61). Instead of JMPing to compiled
                // code (which causes R14 drift from broken stack accounting),
                // emit INT3 that fires the Rust HLE handler. Also intercept
                // XapiCallThreadNotifyRoutines (0x002A966D) which crashes
                // on uninitialized TLS callback list.
                // _SEH_prolog/epilog: NOT intercepted at emit time.
                // The compiled code works (1430 kernel calls, Stage 10).
                // OOVPA manual hooks handle epilog via host-code INT3 planting.
                let target = di
                    .branch_target
                    .map(|target| fold_direct_jmp_thunk(memory, target));
                emit_call_rel(&mut code, ret_addr, &mut fixups, target);
                stats.rewrite_call += 1;
            }
            EmitStrategy::RewriteRet => {
                let cleanup = get_ret_cleanup(di);
                // Inline RET: FULL COVERAGE — all sections use inline guest-hash dispatch.
                // The P0 batched cdecl crash, the 0x00046693 ref-count ret crash, and all
                // other RET corruption stems from INT3 breaking the atomic ret→cleanup flow.
                // XGRPH data-in-code: handled by RetMiss fallback (VEH stack scan).
                let use_inline = true;
                if use_inline {
                    emit_inline_guest_ret(
                        &mut code,
                        &mut traps,
                        cleanup,
                        di.guest_addr,
                        hash_shift,
                    );
                } else {
                    traps.push(TrapInfo {
                        host_offset: code.len() as u32,
                        trap_type: TrapType::Ret,
                        ret_cleanup: cleanup,
                        guest_addr: di.guest_addr,
                    });
                    code.push(0xCC);
                }
                stats.rewrite_ret += 1;
            }
            EmitStrategy::RewriteLeave => {
                let before_len = code.len();
                emit_leave(&mut code, di, before_len);
                stats.rewrite_leave += 1;
            }
            EmitStrategy::RewriteIncDec => {
                emit_incdec_long(&mut code, guest_bytes[0]);
                stats.rewrite_incdec += 1;
            }
            EmitStrategy::RewriteString => {
                emit_string_op(&mut code, di, guest_bytes);
                stats.rewrite_string += 1;
            }
            EmitStrategy::TrapIndirect => {
                // Try to resolve kernel thunk: CALL/JMP [disp32] where target is kernel ordinal.
                // Two formats:
                //   1. 0xFFFF0000+ordinal (standard thunk table after XBE decryption)
                //   2. 0x80000000|ordinal (XPP section function pointers, mirror-mapped)
                let resolved_target = resolve_indirect_target(&di.instruction, memory);
                if let Some(target) = resolved_target {
                    let is_kernel =
                        target >= 0xFFFF_0000 || (target >= 0x8000_0000 && target < 0x8000_0200);
                    if is_kernel {
                        // Normalize mirror thunks to 0xFFFF0000+ordinal
                        let kernel_addr = if target >= 0xFFFF_0000 {
                            target
                        } else {
                            0xFFFF_0000 + (target & 0xFFFF)
                        };
                        let is_call = di.instruction.mnemonic() == Mnemonic::Call;
                        traps.push(TrapInfo {
                            host_offset: code.len() as u32,
                            trap_type: TrapType::System,
                            ret_cleanup: if is_call { 1 } else { 0 },
                            guest_addr: kernel_addr,
                        });
                        code.push(0xCC); // INT3
                        stats.trap_system += 1;
                        continue;
                    }
                }
                // Unresolved indirect — plain trap
                // Store is_call in ret_cleanup (1=CALL, 0=JMP) so VEH doesn't
                // need to read live guest bytes (which may be overwritten by game).
                let is_call_flag = if di.instruction.mnemonic() == Mnemonic::Call {
                    1u16
                } else {
                    0u16
                };
                traps.push(TrapInfo {
                    host_offset: code.len() as u32,
                    trap_type: TrapType::Indirect,
                    ret_cleanup: is_call_flag,
                    guest_addr: di.guest_addr,
                });
                code.push(0xCC); // INT3
                stats.trap_indirect += 1;
            }
            EmitStrategy::TrapSystem => {
                traps.push(TrapInfo {
                    host_offset: code.len() as u32,
                    trap_type: TrapType::System,
                    ret_cleanup: 0,
                    guest_addr: di.guest_addr,
                });
                code.push(0xCC); // INT3
                stats.trap_system += 1;
            }
            EmitStrategy::Skip => {
                stats.skip += 1;
            }
        }

        // Post-emit trace for _alloca_probe and string allocator
        if (di.guest_addr >= 0x002B6C00 && di.guest_addr <= 0x002B6C10)
            || (di.guest_addr >= 0x00046F10 && di.guest_addr <= 0x00046F31)
        {
            let emitted = &code[_pre_emit_len..];
            crate::xbox::emulator::debug_log(&format!(
                "EMIT-ALLOCA: guest=0x{:08X} -> {} host bytes: {:02X?}",
                di.guest_addr,
                emitted.len(),
                emitted
            ));
        }

        // Handle direct branch fixups (JMP/Jcc)
        if strategy == EmitStrategy::Identity && di.category == InstrCategory::Branch {
            if let Some(target) = di.branch_target {
                let m = di.instruction.mnemonic();
                if m != Mnemonic::Call && m != Mnemonic::Ret && m != Mnemonic::Retf {
                    if is_short_branch(guest_bytes) {
                        // Short Jcc/JMP (2 bytes) — undo identity copy, emit near form
                        let identity_start = code.len() - di.length as usize;
                        code.truncate(identity_start);

                        if guest_bytes[0] == 0xEB {
                            code.push(0xE9);
                        } else {
                            // Short Jcc → near Jcc (0x0F 0x80-0x8F)
                            code.push(0x0F);
                            code.push(0x80 + (guest_bytes[0] - 0x70));
                        }
                        let rel32_pos = code.len() as u32;
                        code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
                        fixups.push(BranchFixup {
                            host_offset: rel32_pos,
                            guest_target: target,
                            instr_end_delta: 4,
                        });
                    } else if di.length >= 5 {
                        let rel32_pos = code.len() as u32 - 4;
                        let pos = rel32_pos as usize;
                        if pos + 4 <= code.len() {
                            // Identity-copying guest-relative branch displacements is
                            // unsafe in the x64 code buffer. If a target is rescued
                            // later this placeholder is patched; until then it should
                            // fall through instead of jumping to a guest-relative
                            // offset interpreted as a host-relative offset.
                            code[pos..pos + 4].copy_from_slice(&0i32.to_le_bytes());
                        }
                        fixups.push(BranchFixup {
                            host_offset: rel32_pos,
                            guest_target: target,
                            instr_end_delta: 4,
                        });
                    }
                }
            }
        }
    }

    // Resolve branch fixups
    resolve_fixups(&mut code, &fixups, &addr_map);

    traps.sort_by_key(|t| t.host_offset);
    let unresolved = find_unresolved(&fixups, &addr_map);

    // Post-emit verification: scan emitted x64 for memory accesses missing R15 base
    verify_r15_base(&code, &addr_map, &traps);

    EmittedBlock {
        code,
        addr_map,
        traps,
        unresolved,
        stats,
        encode_fail_count: failures.count,
        encode_fail_samples: failures.first_few,
    }
}

// ============================================================================
// Classification
// ============================================================================

fn classify_for_emit(di: &DecodedInstr) -> EmitStrategy {
    let instr = &di.instruction;
    let m = instr.mnemonic();

    match m {
        Mnemonic::Nop => EmitStrategy::Skip,
        Mnemonic::Int3 => EmitStrategy::TrapSystem,

        Mnemonic::Push => {
            if instr.op0_kind() == OpKind::Register {
                if instr.op_register(0) == Register::ESP {
                    // PUSH ESP: Intel SDM 286+ contract = push the PRE-decrement
                    // value of ESP. Old routing to RewriteEspReg emitted native
                    // PUSH R14D (host stack), which is wrong on two counts.
                    EmitStrategy::RewritePushEsp
                } else {
                    EmitStrategy::RewritePushReg
                }
            } else if instr.op0_kind() == OpKind::Memory {
                EmitStrategy::RewritePushMem
            } else {
                EmitStrategy::RewritePushImm
            }
        }
        Mnemonic::Pop => {
            if instr.op0_kind() == OpKind::Register {
                if instr.op_register(0) == Register::ESP {
                    // POP ESP: Intel SDM = DEST := [SS:ESP]; ESP := ESP + 4.
                    // Since DEST == ESP, the +4 increment is clobbered by the
                    // load. Net semantics: R14D := [R15+R14]. Old routing
                    // emitted native POP R14D (host stack), wrong on both counts.
                    EmitStrategy::RewritePopEsp
                } else {
                    EmitStrategy::RewritePopReg
                }
            } else if instr.op0_kind() == OpKind::Memory {
                EmitStrategy::RewritePopMem
            } else {
                EmitStrategy::RewriteMem
            }
        }

        Mnemonic::Call => {
            if di.branch_target.is_some() {
                EmitStrategy::RewriteCallRel
            } else {
                EmitStrategy::TrapIndirect
            }
        }

        Mnemonic::Ret | Mnemonic::Retf => EmitStrategy::RewriteRet,
        Mnemonic::Leave => EmitStrategy::RewriteLeave,

        // INC/DEC short form (0x40-0x4F conflict with REX in 64-bit mode)
        Mnemonic::Inc | Mnemonic::Dec => {
            if instr.op0_kind() == OpKind::Register && instr.len() == 1 {
                if instr.op_register(0) == Register::ESP {
                    EmitStrategy::RewriteEspReg
                } else {
                    EmitStrategy::RewriteIncDec
                }
            } else if has_memory_operand(instr) {
                EmitStrategy::RewriteMem
            } else if has_esp_register_operand(instr) {
                EmitStrategy::RewriteEspReg
            } else {
                EmitStrategy::Identity
            }
        }

        // String operations — need R15 wrap for ESI/EDI
        Mnemonic::Movsb
        | Mnemonic::Movsw
        | Mnemonic::Movsd
        | Mnemonic::Movsq
        | Mnemonic::Stosb
        | Mnemonic::Stosw
        | Mnemonic::Stosd
        | Mnemonic::Stosq
        | Mnemonic::Scasb
        | Mnemonic::Scasw
        | Mnemonic::Scasd
        | Mnemonic::Cmpsb
        | Mnemonic::Cmpsw
        | Mnemonic::Cmpsd
        | Mnemonic::Lodsb
        | Mnemonic::Lodsw
        | Mnemonic::Lodsd => EmitStrategy::RewriteString,

        // JMP (direct)
        Mnemonic::Jmp if di.branch_target.is_some() => EmitStrategy::Identity,
        // JMP (indirect)
        Mnemonic::Jmp => EmitStrategy::TrapIndirect,

        // System instructions
        Mnemonic::Hlt
        | Mnemonic::Cpuid
        | Mnemonic::Rdtsc
        | Mnemonic::Wbinvd
        | Mnemonic::Invlpg
        | Mnemonic::Sysenter
        | Mnemonic::Sysexit
        | Mnemonic::Cli
        | Mnemonic::Sti => EmitStrategy::TrapSystem,

        Mnemonic::Int => EmitStrategy::TrapSystem,

        // Conditional branches — identity (short ones expanded in fixup pass)
        _ if instr.is_jcc_short_or_near() => EmitStrategy::Identity,

        // LEA — computes a guest address, does NOT access memory.
        // Must NOT add R15 — the result is a guest address that will
        // get R15 added when it's actually dereferenced later.
        // But if it references ESP (register OR memory base), we need ESP→R14D rewrite.
        Mnemonic::Lea => {
            if has_esp_register_operand(instr) || has_esp_in_memory(instr) {
                EmitStrategy::RewriteEspReg
            } else {
                EmitStrategy::Identity
            }
        }

        // Everything else — check for memory operands and ESP register
        _ => {
            if has_memory_operand(instr) {
                EmitStrategy::RewriteMem
            } else if has_esp_register_operand(instr) {
                EmitStrategy::RewriteEspReg
            } else {
                EmitStrategy::Identity
            }
        }
    }
}

fn has_memory_operand(instr: &Instruction) -> bool {
    for i in 0..instr.op_count() {
        if instr.op_kind(i) == OpKind::Memory {
            return true;
        }
    }
    false
}

/// Check if any explicit register operand is ESP/RSP.
/// This catches `MOV EBP, ESP`, `SUB ESP, N`, `AND ESP, -16`, etc.
fn has_esp_register_operand(instr: &Instruction) -> bool {
    for i in 0..instr.op_count() {
        if instr.op_kind(i) == OpKind::Register {
            let reg = instr.op_register(i);
            if reg == Register::ESP || reg == Register::RSP || reg == Register::SP {
                return true;
            }
        }
    }
    false
}

/// Check if the memory operand's base or index register is ESP/RSP.
/// This catches `LEA EBP, [ESP+N]`, `LEA EAX, [ESP+ECX*4]`, etc.
/// where ESP is in the addressing mode, not as a standalone register operand.
fn has_esp_in_memory(instr: &Instruction) -> bool {
    let base = instr.memory_base();
    let index = instr.memory_index();
    matches!(base, Register::ESP | Register::RSP | Register::SP)
        || matches!(index, Register::ESP | Register::RSP | Register::SP)
}

fn is_short_branch(bytes: &[u8]) -> bool {
    matches!(bytes[0], 0x70..=0x7F | 0xEB)
}

/// Try to resolve an indirect CALL/JMP target through guest memory.
/// For `CALL [disp32]` or `JMP [disp32]` (absolute indirect), reads the
/// thunk table entry. Returns `Some(target)` if resolvable.
fn resolve_indirect_target(instr: &Instruction, memory: &GuestMemory) -> Option<u32> {
    // Only handle memory-indirect with no base/index (absolute addressing)
    if instr.op0_kind() != OpKind::Memory {
        return None;
    }
    if instr.memory_base() != Register::None || instr.memory_index() != Register::None {
        return None;
    }
    // disp32 = thunk table entry address
    let thunk_addr = instr.memory_displacement32();
    if thunk_addr == 0 {
        return None;
    }
    let target = memory.read_u32(thunk_addr);
    if target != 0 {
        Some(target)
    } else {
        None
    }
}

/// Fold tiny direct-jump thunks used as callable stubs.
///
/// If the AOT linear pass misses the thunk address, a direct CALL fixup can
/// remain as `push ret; jmp +0`, which falls through with the return address
/// still on the guest stack. Replacing `call thunk` with `call real_target`
/// preserves guest semantics because the thunk is only an unconditional jump.
fn fold_direct_jmp_thunk(memory: &GuestMemory, target: u32) -> u32 {
    let mut current = target;
    for _ in 0..4 {
        let opcode = memory.read_u8(current);
        let next = match opcode {
            0xE9 => {
                let disp = memory.read_u32(current.wrapping_add(1)) as i32;
                current.wrapping_add(5).wrapping_add(disp as u32)
            }
            0xEB => {
                let disp = memory.read_u8(current.wrapping_add(1)) as i8 as i32;
                current.wrapping_add(2).wrapping_add(disp as u32)
            }
            _ => break,
        };

        // Stay in the normal XBE code range; do not chase wild/data targets.
        if !(0x0001_0000..0x0080_0000).contains(&next) || next == current {
            break;
        }
        current = next;
    }
    current
}

// ============================================================================
// Memory operand rewrite using iced-x86 Encoder (type-safe)
// ============================================================================

/// Map 32-bit register to 64-bit counterpart for use as SIB index.
fn to_64bit(reg: Register) -> Register {
    match reg {
        Register::EAX => Register::RAX,
        Register::ECX => Register::RCX,
        Register::EDX => Register::RDX,
        Register::EBX => Register::RBX,
        Register::ESP | Register::R14D | Register::R14 => Register::R14, // guest ESP lives in R14
        Register::EBP => Register::RBP,
        Register::ESI => Register::RSI,
        Register::EDI => Register::RDI,
        // If already 64-bit or unknown, pass through
        other => other,
    }
}

/// Map x86-32 register to its 32-bit form for LEA address-size override.
/// ESP → R14D (guest ESP), others stay as their 32-bit names.
fn to_32bit(reg: Register) -> Register {
    match reg {
        Register::EAX | Register::RAX => Register::EAX,
        Register::ECX | Register::RCX => Register::ECX,
        Register::EDX | Register::RDX => Register::EDX,
        Register::EBX | Register::RBX => Register::EBX,
        Register::ESP | Register::R14 | Register::R14D => Register::R14D, // guest ESP
        Register::EBP | Register::RBP => Register::EBP,
        Register::ESI | Register::RSI => Register::ESI,
        Register::EDI | Register::RDI => Register::EDI,
        other => other,
    }
}

/// Rewrite a memory operand to use R15 as base (guest memory base).
/// Tries iced-x86 encoder first, falls back to manual byte construction.
/// Returns true on success, false if both paths failed (NOP emitted).
fn emit_memory_rewrite(code: &mut Vec<u8>, instr: &Instruction, guest_bytes: &[u8]) -> bool {
    let saved_len = code.len();
    let trace = guest_bytes.len() >= 3
        && guest_bytes[0] == 0x8B
        && guest_bytes[1] == 0x34
        && guest_bytes[2] == 0xB5;

    // In x64, any REX prefix disables AH/CH/DH/BH encodings and turns those
    // ModRM fields into SPL/BPL/SIL/DIL. R15-relative guest memory always needs
    // REX, so high-byte MOVs must be expanded instead of re-encoded directly.
    if instr.mnemonic() == Mnemonic::Mov {
        if instr.op0_kind() == OpKind::Register
            && instr.op1_kind() == OpKind::Memory
            && high_byte_parent(instr.op_register(0)).is_some()
        {
            return emit_high_byte_load_from_mem(code, instr);
        }
        if instr.op0_kind() == OpKind::Memory
            && instr.op1_kind() == OpKind::Register
            && high_byte_parent(instr.op_register(1)).is_some()
        {
            return emit_high_byte_store_to_mem(code, instr);
        }
    }

    // Try iced-x86 encoder first
    if try_iced_memory_rewrite(code, instr) {
        if trace {
            let emitted = &code[saved_len..];
            crate::xbox::emulator::debug_log(&format!(
                "EMIT-REWRITE: iced OK for MOV ESI,[ESI*4+disp32] → {:02X?} ({}b)",
                emitted,
                emitted.len()
            ));
        }
        return true;
    }

    // Iced failed — revert and try manual byte-level rewrite
    code.truncate(saved_len);
    if trace {
        crate::xbox::emulator::debug_log(
            "EMIT-REWRITE: iced FAILED for MOV ESI,[ESI*4+disp32], trying manual",
        );
    }
    if manual_memory_rewrite(code, instr, guest_bytes) {
        if trace {
            let emitted = &code[saved_len..];
            crate::xbox::emulator::debug_log(&format!(
                "EMIT-REWRITE: manual OK → {:02X?} ({}b)",
                emitted,
                emitted.len()
            ));
        }
        return true;
    }

    // Both failed — caller will emit rescue trap
    code.truncate(saved_len);
    if trace {
        crate::xbox::emulator::debug_log("EMIT-REWRITE: BOTH FAILED for MOV ESI,[ESI*4+disp32]");
    }
    false
}

/// KPCR base address in guest memory — FS: reads are redirected here.
/// On x64 Windows FS.base=0, so our emitter converts FS:[offset] → [R15+KPCR+offset].
/// This keeps the zero page zeroed (NULL deref = 0) while FS reads hit valid KPCR data.
const FAKE_KPCR_BASE_ADDR: u32 = 0x0C00_0000;

fn high_byte_parent(reg: Register) -> Option<Register> {
    match reg {
        Register::AH => Some(Register::EAX),
        Register::CH => Some(Register::ECX),
        Register::DH => Some(Register::EDX),
        Register::BH => Some(Register::EBX),
        _ => None,
    }
}

fn emit_guest_ea_to_r10d(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let base = instr.memory_base();
    let index = instr.memory_index();
    let scale = instr.memory_index_scale();
    let fs_adj = if instr.segment_prefix() == Register::FS || instr.segment_prefix() == Register::GS
    {
        FAKE_KPCR_BASE_ADDR
    } else {
        0
    };
    let disp = instr.memory_displacement32().wrapping_add(fs_adj);

    if base == Register::None && index == Register::None {
        // MOV R10D, imm32
        code.extend_from_slice(&[0x41, 0xBA]);
        code.extend_from_slice(&disp.to_le_bytes());
        return true;
    }

    let base32 = if base == Register::None {
        Register::None
    } else {
        to_32bit(base)
    };
    let index32 = if index == Register::None {
        Register::None
    } else {
        to_32bit(index)
    };

    let mem_op = if base32 != Register::None && index32 != Register::None {
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            base32,
            index32,
            scale as u32,
            disp as i32 as i64,
            4,
        )
    } else if base32 != Register::None {
        iced_x86::MemoryOperand::with_base_displ_size(base32, disp as i32 as i64, 4)
    } else {
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            Register::None,
            index32,
            scale as u32,
            disp as i32 as i64,
            4,
        )
    };

    let lea = match Instruction::with2(iced_x86::Code::Lea_r32_m, Register::R10D, mem_op) {
        Ok(lea) => lea,
        Err(_) => return false,
    };
    encode_to(code, &lea)
}

fn emit_high_byte_load_from_mem(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let saved_len = code.len();
    let Some(parent) = high_byte_parent(instr.op_register(0)) else {
        return false;
    };
    let (parent_enc, parent_rex) = reg32_to_enc(parent);
    if parent_rex {
        return false;
    }

    code.push(0x9C); // PUSHFQ
    if !emit_guest_ea_to_r10d(code, instr) {
        code.truncate(saved_len);
        return false;
    }

    let mem_op = iced_x86::MemoryOperand::with_base_index(Register::R15, Register::R10);
    let movzx = match Instruction::with2(iced_x86::Code::Movzx_r32_rm8, Register::R11D, mem_op) {
        Ok(movzx) => movzx,
        Err(_) => {
            code.truncate(saved_len);
            return false;
        }
    };
    if !encode_to(code, &movzx) {
        code.truncate(saved_len);
        return false;
    }

    // Clear bits 8..15 in the parent register, then OR in scratch << 8.
    code.extend_from_slice(&[0x81, 0xE0 | parent_enc]);
    code.extend_from_slice(&0xFFFF_00FFu32.to_le_bytes());
    code.extend_from_slice(&[0x41, 0xC1, 0xE3, 0x08]); // SHL R11D,8
    code.extend_from_slice(&[0x44, 0x09, 0xD8 | parent_enc]); // OR parent,R11D
    code.push(0x9D); // POPFQ
    true
}

fn emit_high_byte_store_to_mem(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let saved_len = code.len();
    let Some(parent) = high_byte_parent(instr.op_register(1)) else {
        return false;
    };
    let (parent_enc, parent_rex) = reg32_to_enc(parent);
    if parent_rex {
        return false;
    }

    code.push(0x9C); // PUSHFQ
    if !emit_guest_ea_to_r10d(code, instr) {
        code.truncate(saved_len);
        return false;
    }

    // MOV R11D,parent; SHR R11D,8; MOV [R15+R10],R11B
    code.extend_from_slice(&[0x41, 0x89, 0xC3 | (parent_enc << 3)]);
    code.extend_from_slice(&[0x41, 0xC1, 0xEB, 0x08]);
    code.extend_from_slice(&[0x47, 0x88, 0x1C, 0x17]);
    code.push(0x9D); // POPFQ
    true
}

/// Try iced-x86 Instruction mutation + Encoder approach.
fn try_iced_memory_rewrite(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let base = instr.memory_base();
    let index = instr.memory_index();
    let scale = instr.memory_index_scale();

    let mut new_instr = *instr;
    replace_esp_in_operands(&mut new_instr);

    // FS: segment prefix → redirect to KPCR base (0x0C000000 + offset)
    // Strip segment prefix (invalid in 64-bit mode with R15 base)
    if instr.segment_prefix() == Register::FS || instr.segment_prefix() == Register::GS {
        let old_disp = new_instr.memory_displacement32();
        new_instr.set_memory_displacement32(old_disp.wrapping_add(FAKE_KPCR_BASE_ADDR));
        new_instr.set_segment_prefix(Register::None);
    }

    match (base, index) {
        (Register::None, Register::None) => {
            new_instr.set_memory_base(Register::R15);
        }
        (reg, Register::None) if reg != Register::None => {
            let disp = new_instr.memory_displacement32();
            let has_disp = disp != 0 || instr.memory_displ_size() > 0;
            if has_disp {
                // When base+disp can wrap in 32-bit (e.g. ESI=0xFFFFFFC4, disp=0x60
                // → 0x24 in 32-bit but overflows to 0x100000024 in 64-bit),
                // use LEA R11D to compute 32-bit EA with wrapping, then [R15+R11].
                let reg32 = to_32bit(reg);
                let mem_op = iced_x86::MemoryOperand::with_base_displ_size(reg32, disp as i64, 4);
                let lea = Instruction::with2(iced_x86::Code::Lea_r32_m, Register::R11D, mem_op);
                let lea = match lea {
                    Ok(l) => l,
                    Err(_) => return false,
                };
                let ok1 = encode_to(code, &lea);
                new_instr.set_memory_base(Register::R15);
                new_instr.set_memory_index(Register::R11);
                new_instr.set_memory_index_scale(1);
                new_instr.set_memory_displacement32(0);
                if new_instr.memory_displ_size() > 0 {
                    new_instr.set_memory_displ_size(0);
                }
                let ok2 = encode_to(code, &new_instr);
                return ok1 && ok2;
            } else {
                let reg64 = to_64bit(reg);
                new_instr.set_memory_base(Register::R15);
                new_instr.set_memory_index(reg64);
                new_instr.set_memory_index_scale(1);
            }
        }
        (reg, idx) if reg != Register::None && idx != Register::None => {
            // C++ approach: LEA R11D computes 32-bit EA (wraps at 4GB),
            // then actual access uses [R15+R11] so VEH R15-fixup still works.
            // Old approach (LEA R11,[R15+base]) broke fixup because R15
            // wasn't in the faulting instruction, and 64-bit addition overflowed.
            let base32 = to_32bit(reg);
            let idx32 = to_32bit(idx);
            let disp = new_instr.memory_displacement32();
            let has_disp = disp != 0 || instr.memory_displ_size() > 0;
            let mem_op = if has_disp {
                iced_x86::MemoryOperand::with_base_index_scale_displ_size(
                    base32,
                    idx32,
                    scale as u32,
                    disp as i64,
                    4,
                )
            } else {
                iced_x86::MemoryOperand::with_base_index_scale(base32, idx32, scale as u32)
            };
            let lea = Instruction::with2(iced_x86::Code::Lea_r32_m, Register::R11D, mem_op);
            let lea = match lea {
                Ok(l) => l,
                Err(_) => return false,
            };
            let ok1 = encode_to(code, &lea);
            // Now replace original memory operand with [R15+R11] (no disp)
            new_instr.set_memory_base(Register::R15);
            new_instr.set_memory_index(Register::R11);
            new_instr.set_memory_index_scale(1);
            new_instr.set_memory_displacement32(0);
            if new_instr.memory_displ_size() > 0 {
                new_instr.set_memory_displ_size(0);
            }
            let ok2 = encode_to(code, &new_instr);
            return ok1 && ok2;
        }
        (Register::None, idx) if idx != Register::None => {
            let disp = new_instr.memory_displacement32();
            let has_disp = disp != 0 || instr.memory_displ_size() > 0;
            if has_disp {
                // Same 32-bit wrapping fix as base-only path above.
                let idx32 = to_32bit(idx);
                let mem_op = iced_x86::MemoryOperand::with_base_index_scale_displ_size(
                    Register::None,
                    idx32,
                    scale as u32,
                    disp as i64,
                    4,
                );
                let lea = Instruction::with2(iced_x86::Code::Lea_r32_m, Register::R11D, mem_op);
                let lea = match lea {
                    Ok(l) => l,
                    Err(_) => return false,
                };
                let ok1 = encode_to(code, &lea);
                new_instr.set_memory_base(Register::R15);
                new_instr.set_memory_index(Register::R11);
                new_instr.set_memory_index_scale(1);
                new_instr.set_memory_displacement32(0);
                if new_instr.memory_displ_size() > 0 {
                    new_instr.set_memory_displ_size(0);
                }
                let ok2 = encode_to(code, &new_instr);
                return ok1 && ok2;
            } else {
                let idx64 = to_64bit(idx);
                new_instr.set_memory_base(Register::R15);
                new_instr.set_memory_index(idx64);
                new_instr.set_memory_index_scale(scale);
            }
        }
        _ => {}
    }

    encode_to(code, &new_instr)
}

// ============================================================================
// Manual byte-level memory rewrite (fallback when iced-x86 encoder fails)
// Matches C++ approach: raw REX/ModRM/SIB byte construction.
// ============================================================================

/// Manual byte-level memory operand rewrite. Parses the x86-32 instruction
/// bytes and reconstructs with R15 as base register, matching C++ emitter.
fn manual_memory_rewrite(code: &mut Vec<u8>, instr: &Instruction, guest_bytes: &[u8]) -> bool {
    let len = guest_bytes.len();
    if len == 0 {
        return false;
    }

    // Check for FS/GS segment prefix — add KPCR base to displacement
    let fs_disp_adjust: u32 =
        if instr.segment_prefix() == Register::FS || instr.segment_prefix() == Register::GS {
            FAKE_KPCR_BASE_ADDR
        } else {
            0
        };

    // --- Handle MOV moffs special cases (A0-A3) ---
    // These have no ModRM byte: MOV AL/AX/EAX,[moffs32] and MOV [moffs32],AL/AX/EAX
    let first_non_prefix = skip_prefixes(guest_bytes);
    if first_non_prefix < len {
        let opcode = guest_bytes[first_non_prefix];
        if matches!(opcode, 0xA0 | 0xA1 | 0xA2 | 0xA3) {
            return emit_moffs_rewrite(code, guest_bytes, first_non_prefix);
        }
    }

    // --- Parse instruction structure ---
    let parsed = match parse_x86_instr(guest_bytes) {
        Some(p) => p,
        None => return false,
    };

    let modrm = guest_bytes[parsed.modrm_pos];
    let mod_field = (modrm >> 6) & 3;
    let reg_field = (modrm >> 3) & 7;
    let rm_field = modrm & 7;

    // Check if reg field is ESP and represents a register operand (not opcode extension)
    let esp_in_reg = reg_field == 4 && has_gpr_in_reg_field(instr);

    // Adjusted reg field: ESP(4) → R14D needs reg=6 + REX.R
    let new_reg = if esp_in_reg { 6 } else { reg_field };
    let rex_r = if esp_in_reg { 0x04u8 } else { 0 };

    // --- Emit prefixes (skip 0x67 address-size, 0x64/0x65 segment overrides) ---
    for i in 0..parsed.opcode_pos {
        let b = guest_bytes[i];
        if b != 0x67 && b != 0x64 && b != 0x65 {
            code.push(b);
        }
    }

    match (mod_field, rm_field) {
        // ---- [disp32] (mod=0, rm=5) → [R15 + disp32] ----
        (0, 5) => {
            if parsed.disp_pos + 4 > len {
                return false;
            }
            code.push(0x41 | rex_r);
            code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
            code.push(0x80 | (new_reg << 3) | 0x07);
            // Adjust displacement for FS: segment → KPCR base
            let disp = u32::from_le_bytes([
                guest_bytes[parsed.disp_pos],
                guest_bytes[parsed.disp_pos + 1],
                guest_bytes[parsed.disp_pos + 2],
                guest_bytes[parsed.disp_pos + 3],
            ])
            .wrapping_add(fs_disp_adjust);
            code.extend_from_slice(&disp.to_le_bytes());
            copy_tail(code, guest_bytes, parsed.imm_pos);
        }

        // ---- [reg] (mod=0, rm≠4,5) → [R15 + reg*1] ----
        (0, rm) if rm != 4 => {
            let rex_x = if rm == 4 { 0x02u8 } else { 0 }; // REX.X if index needs extension (won't hit since rm≠4)
            code.push(0x41 | rex_r | rex_x); // REX.B for R15
            code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
            // ModRM: mod=00, reg, rm=100 (SIB follows)
            code.push((new_reg << 3) | 0x04);
            // SIB: scale=0, index=rm, base=111 (R15)
            code.push((rm << 3) | 0x07);
            copy_tail(code, guest_bytes, parsed.imm_pos);
        }

        // ---- [reg+disp8] (mod=1, rm≠4) → [R15 + reg*1 + disp32] ----
        (1, rm) if rm != 4 => {
            if parsed.disp_pos >= len {
                return false;
            }
            code.push(0x41 | rex_r);
            code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
            code.push(0x80 | (new_reg << 3) | 0x04);
            code.push((rm << 3) | 0x07);
            let disp8 = guest_bytes[parsed.disp_pos] as i8;
            let disp32 = (disp8 as i32 as u32).wrapping_add(fs_disp_adjust);
            code.extend_from_slice(&disp32.to_le_bytes());
            copy_tail(code, guest_bytes, parsed.imm_pos);
        }

        // ---- [reg+disp32] (mod=2, rm≠4) → [R15 + reg*1 + disp32] ----
        (2, rm) if rm != 4 => {
            if parsed.disp_pos + 4 > len {
                return false;
            }
            code.push(0x41 | rex_r);
            code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
            code.push(0x80 | (new_reg << 3) | 0x04);
            code.push((rm << 3) | 0x07);
            code.extend_from_slice(&guest_bytes[parsed.disp_pos..parsed.disp_pos + 4]);
            copy_tail(code, guest_bytes, parsed.imm_pos);
        }

        // ---- SIB cases (rm=4) ----
        (_, 4) => {
            let sib_pos = parsed.modrm_pos + 1;
            if sib_pos >= len {
                return false;
            }
            let sib = guest_bytes[sib_pos];
            let sib_scale = (sib >> 6) & 3;
            let sib_index = (sib >> 3) & 7;
            let sib_base = sib & 7;

            let disp_end = parsed.disp_pos + parsed.disp_size as usize;
            if parsed.disp_size > 0 && disp_end > len {
                return false;
            }

            if sib_index == 4 && sib_base == 4 {
                // [ESP+disp] (base=ESP, index=none) → [R15 + R14 + disp]
                code.push(0x43 | rex_r);
                code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
                code.push((mod_field << 6) | (new_reg << 3) | 0x04);
                code.push(0x37); // SIB: index=R14(110), base=R15(111)
                if parsed.disp_size > 0 {
                    code.extend_from_slice(&guest_bytes[parsed.disp_pos..disp_end]);
                }
                copy_tail(code, guest_bytes, parsed.imm_pos);
            } else if sib_index == 4 && mod_field == 0 && sib_base == 5 {
                // [disp32] via SIB (index=none, base=5, mod=0) → [R15 + disp32]
                code.push(0x41 | rex_r);
                code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
                code.push(0x80 | (new_reg << 3) | 0x07);
                code.extend_from_slice(&guest_bytes[parsed.disp_pos..disp_end]);
                copy_tail(code, guest_bytes, parsed.imm_pos);
            } else if sib_index == 4 {
                // [base+disp] via SIB with no index → [R15 + base*1 + disp]
                code.push(0x41 | rex_r);
                code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
                code.push((mod_field << 6) | (new_reg << 3) | 0x04);
                code.push((sib_base << 3) | 0x07);
                if parsed.disp_size > 0 {
                    code.extend_from_slice(&guest_bytes[parsed.disp_pos..disp_end]);
                }
                copy_tail(code, guest_bytes, parsed.imm_pos);
            } else if mod_field == 0 && sib_base == 5 {
                // [index*scale + disp32] (no base, SIB special: mod=0, base=5)
                // → [R15 + index*scale + disp32]
                let idx64 = to_64bit(reg32_from_index(sib_index));
                let rex_x = if matches!(
                    idx64,
                    Register::R8
                        | Register::R9
                        | Register::R10
                        | Register::R11
                        | Register::R12
                        | Register::R13
                        | Register::R14
                        | Register::R15
                ) {
                    0x02u8
                } else {
                    0
                };
                code.push(0x41 | rex_r | rex_x); // REX.B for R15
                code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
                // ModRM: mod=10 (disp32), reg, rm=100 (SIB follows)
                code.push(0x80 | (new_reg << 3) | 0x04);
                // SIB: original scale, index=idx64, base=R15(111)
                let new_sib = (sib_scale << 6) | (reg_encoding(idx64) << 3) | 0x07;
                code.push(new_sib);
                // Copy original disp32
                if disp_end <= len {
                    code.extend_from_slice(&guest_bytes[parsed.disp_pos..disp_end]);
                }
                copy_tail(code, guest_bytes, parsed.imm_pos);
            } else {
                // [base + index*scale + disp] → need scratch register
                // LEA R11, [R15 + base64]; then [R11 + index*scale + disp]
                let base64 = to_64bit(reg32_from_index(sib_base));
                let idx64 = to_64bit(reg32_from_index(sib_index));

                // Use iced-x86 for the LEA (reliable for simple LEA)
                let lea = Instruction::with2(
                    iced_x86::Code::Lea_r64_m,
                    Register::R11,
                    iced_x86::MemoryOperand::with_base_index(Register::R15, base64),
                );
                match lea {
                    Ok(lea) => {
                        if !encode_to(code, &lea) {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }

                // Emit original instruction with base=R11 instead of original base
                let rex_x = if matches!(
                    idx64,
                    Register::R8
                        | Register::R9
                        | Register::R10
                        | Register::R11
                        | Register::R12
                        | Register::R13
                        | Register::R14
                        | Register::R15
                ) {
                    0x02
                } else {
                    0
                };
                code.push(0x41 | rex_r | rex_x); // REX.B for R11 in SIB base
                code.extend_from_slice(&guest_bytes[parsed.opcode_pos..parsed.modrm_pos]);
                code.push((mod_field << 6) | (new_reg << 3) | 0x04); // ModRM with SIB
                                                                     // SIB: original scale, index=idx64, base=R11(011 via REX.B)
                let new_sib = (sib_scale << 6) | (reg_encoding(idx64) << 3) | 0x03;
                code.push(new_sib);
                // Copy displacement (bounds-checked)
                let disp_end = parsed.disp_pos + parsed.disp_size as usize;
                if parsed.disp_size > 0 && disp_end <= len {
                    code.extend_from_slice(&guest_bytes[parsed.disp_pos..disp_end]);
                }
                // Copy immediates (bounds-checked)
                if parsed.imm_pos < len {
                    code.extend_from_slice(&guest_bytes[parsed.imm_pos..]);
                }
            }
        }

        _ => return false,
    }

    true
}

/// Handle MOV moffs (A0-A3): no ModRM byte, just opcode + disp32.
/// Convert to ModRM form with R15 base.
fn emit_moffs_rewrite(code: &mut Vec<u8>, guest_bytes: &[u8], opcode_pos: usize) -> bool {
    let opcode = guest_bytes[opcode_pos];
    let disp_pos = opcode_pos + 1;
    if disp_pos + 4 > guest_bytes.len() {
        return false;
    }

    // Check for FS/GS segment prefix → add KPCR base to displacement
    let has_fs = guest_bytes[..opcode_pos]
        .iter()
        .any(|&b| b == 0x64 || b == 0x65);

    // Copy prefixes (skip 0x67 address-size and 0x64/0x65 segment overrides)
    for i in 0..opcode_pos {
        let b = guest_bytes[i];
        if b != 0x67 && b != 0x64 && b != 0x65 {
            code.push(b);
        }
    }

    // REX.B for R15
    code.push(0x41);

    // Convert opcode: A0→8A, A1→8B (load); A2→88, A3→89 (store)
    let new_opcode = match opcode {
        0xA0 => 0x8A, // MOV AL,[moffs] → MOV AL,[R15+disp32]
        0xA1 => 0x8B, // MOV EAX,[moffs] → MOV EAX,[R15+disp32]
        0xA2 => 0x88, // MOV [moffs],AL → MOV [R15+disp32],AL
        0xA3 => 0x89, // MOV [moffs],EAX → MOV [R15+disp32],EAX
        _ => return false,
    };
    code.push(new_opcode);

    // ModRM: mod=10, reg=000 (AL/EAX), rm=111 (R15)
    code.push(0x87); // 10_000_111

    // Adjust displacement for FS: segment → KPCR base
    let disp = u32::from_le_bytes([
        guest_bytes[disp_pos],
        guest_bytes[disp_pos + 1],
        guest_bytes[disp_pos + 2],
        guest_bytes[disp_pos + 3],
    ])
    .wrapping_add(if has_fs { FAKE_KPCR_BASE_ADDR } else { 0 });
    code.extend_from_slice(&disp.to_le_bytes());

    true
}

/// Parse x86-32 instruction to find byte boundaries.
struct ParsedX86 {
    opcode_pos: usize, // start of opcode byte(s)
    modrm_pos: usize,  // position of ModRM byte
    disp_pos: usize,   // start of displacement
    disp_size: u8,     // 0, 1, or 4
    imm_pos: usize,    // start of immediates (everything after displacement)
}

fn parse_x86_instr(bytes: &[u8]) -> Option<ParsedX86> {
    let len = bytes.len();
    let mut pos = skip_prefixes(bytes);
    if pos >= len {
        return None;
    }

    let opcode_pos = pos;

    // Determine opcode length
    if bytes[pos] == 0x0F {
        pos += 1;
        if pos >= len {
            return None;
        }
        if bytes[pos] == 0x38 || bytes[pos] == 0x3A {
            pos += 1; // 3-byte opcode
        }
        pos += 1; // final opcode byte
    } else {
        pos += 1;
    }

    // ModRM should be at pos now
    if pos >= len {
        return None;
    }
    let modrm_pos = pos;
    let modrm = bytes[modrm_pos];
    let mod_field = (modrm >> 6) & 3;
    let rm_field = modrm & 7;
    pos += 1;

    // SIB byte if rm=4 and mod≠3
    let has_sib = rm_field == 4 && mod_field != 3;
    let sib = if has_sib {
        if pos >= len {
            return None;
        }
        let s = bytes[pos];
        pos += 1;
        Some(s)
    } else {
        None
    };

    // Displacement size
    let disp_pos = pos;
    let disp_size: u8 = match mod_field {
        0 => {
            if rm_field == 5 {
                4 // [disp32]
            } else if has_sib && (sib.unwrap_or(0) & 7) == 5 {
                4 // SIB with base=5, mod=0 → disp32
            } else {
                0
            }
        }
        1 => 1,
        2 => 4,
        _ => 0, // mod=3 means register, no memory
    };
    pos += disp_size as usize;

    Some(ParsedX86 {
        opcode_pos,
        modrm_pos,
        disp_pos,
        disp_size,
        imm_pos: pos,
    })
}

fn skip_prefixes(bytes: &[u8]) -> usize {
    let mut pos = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            0x66 | 0x67 | 0xF0 | 0xF2 | 0xF3 | 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 => pos += 1,
            _ => break,
        }
    }
    pos
}

fn copy_tail(code: &mut Vec<u8>, guest_bytes: &[u8], from: usize) {
    if from < guest_bytes.len() {
        code.extend_from_slice(&guest_bytes[from..]);
    }
}

/// Check if the ModRM reg field represents a GPR (not an opcode extension).
/// Uses iced-x86's decoded info: if a non-memory operand is a register, reg field is a GPR.
fn has_gpr_in_reg_field(instr: &Instruction) -> bool {
    for i in 0..instr.op_count() {
        if instr.op_kind(i) == OpKind::Register {
            let reg = instr.op_register(i);
            // Filter out segment registers, control registers, etc.
            if matches!(
                reg,
                Register::EAX
                    | Register::ECX
                    | Register::EDX
                    | Register::EBX
                    | Register::ESP
                    | Register::EBP
                    | Register::ESI
                    | Register::EDI
                    | Register::AX
                    | Register::CX
                    | Register::DX
                    | Register::BX
                    | Register::SP
                    | Register::BP
                    | Register::SI
                    | Register::DI
                    | Register::AL
                    | Register::CL
                    | Register::DL
                    | Register::BL
                    | Register::AH
                    | Register::CH
                    | Register::DH
                    | Register::BH
            ) {
                return true;
            }
        }
    }
    false
}

/// Map register index (0-7) to iced-x86 32-bit Register.
fn reg32_from_index(idx: u8) -> Register {
    match idx {
        0 => Register::EAX,
        1 => Register::ECX,
        2 => Register::EDX,
        3 => Register::EBX,
        4 => Register::ESP,
        5 => Register::EBP,
        6 => Register::ESI,
        7 => Register::EDI,
        _ => Register::EAX,
    }
}

/// Get the 3-bit encoding for a 64-bit register (low 3 bits, REX extends the 4th).
fn reg_encoding(reg: Register) -> u8 {
    match reg {
        Register::RAX => 0,
        Register::RCX => 1,
        Register::RDX => 2,
        Register::RBX => 3,
        Register::RSP => 4,
        Register::RBP => 5,
        Register::RSI => 6,
        Register::RDI => 7,
        Register::R8 => 0,
        Register::R9 => 1,
        Register::R10 => 2,
        Register::R11 => 3,
        Register::R12 => 4,
        Register::R13 => 5,
        Register::R14 => 6,
        Register::R15 => 7,
        _ => 0,
    }
}

/// Rewrite ESP register operands to R14D, then encode in 64-bit mode.
/// Handles: MOV EBP,ESP → MOV EBP,R14D; SUB ESP,N → SUB R14D,N;
///          LEA EBP,[ESP+N] → LEA EBP,[R14+N]; etc.
fn emit_esp_reg_rewrite(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    // LEA with ESP in memory base/index needs special handling:
    // the 32-bit decoded instruction can't be re-encoded in 64-bit after
    // patching the base to R14. Build from scratch using iced-x86 builder.
    if instr.mnemonic() == Mnemonic::Lea && has_esp_in_memory(instr) {
        return emit_lea_esp_rewrite(code, instr);
    }
    let mut new_instr = *instr;
    replace_esp_in_operands(&mut new_instr);
    encode_to(code, &new_instr)
}

/// Manually emit LEA with ESP→R14 in memory operand.
/// Can't re-encode the 32-bit decoded instruction in 64-bit mode after patching
/// the base to R14, so we build the bytes by hand.
///
/// Handles: LEA reg, [ESP+disp] → LEA reg, [R14+disp]
///          LEA reg, [ESP+index*scale+disp] → LEA reg, [R14+index*scale+disp]
///          LEA ESP, [base+disp] → LEA R14D, [base+disp] (dest=ESP case)
fn emit_lea_esp_rewrite(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    // Get destination register and its 3-bit encoding
    let dest = if instr.op0_kind() == OpKind::Register {
        instr.op_register(0)
    } else {
        return false;
    };

    // Map 32-bit dest to encoding. If dest is ESP, we want R14D.
    let (dest_enc, dest_rex_r) = match dest {
        Register::EAX => (0u8, false),
        Register::ECX => (1, false),
        Register::EDX => (2, false),
        Register::EBX => (3, false),
        Register::ESP => (6, true), // ESP dest → R14D (enc=6, REX.R for R14)
        Register::EBP => (5, false),
        Register::ESI => (6, false),
        Register::EDI => (7, false),
        _ => return false,
    };

    // Get memory operand components, replacing ESP with R14
    let orig_base = instr.memory_base();
    let orig_index = instr.memory_index();
    let scale = instr.memory_index_scale();
    let disp = instr.memory_displacement32() as i32;

    // Is ESP in base or index?
    let base_is_esp = matches!(orig_base, Register::ESP | Register::RSP);
    let index_is_esp = matches!(orig_index, Register::ESP | Register::RSP);

    // Map base register (ESP→R14, others→their x64 3-bit encoding)
    let (base_enc, base_rex_b) = if base_is_esp {
        (6u8, true) // R14 = enc 6, REX.B
    } else {
        reg32_to_enc(orig_base)
    };

    let has_index = orig_index != Register::None;
    let (index_enc, index_rex_x) = if has_index {
        if index_is_esp {
            (6u8, true) // R14 = enc 6, REX.X
        } else {
            reg32_to_enc(orig_index)
        }
    } else {
        (4, false) // 4 = no index in SIB
    };

    // Determine displacement size
    let (mod_bits, disp_bytes): (u8, &[u8]) = if disp == 0 && base_enc != 5 {
        // mod=00, no displacement (but EBP/R13 base always needs disp8)
        (0b00, &[])
    } else if disp >= -128 && disp <= 127 {
        (0b01, &[]) // placeholder — we'll push disp8 below
    } else {
        (0b10, &[]) // disp32
    };

    // Build REX prefix: 0x40 | (R<<2) | (X<<1) | B
    let rex = 0x40
        | if dest_rex_r { 0x04 } else { 0 }
        | if index_rex_x { 0x02 } else { 0 }
        | if base_rex_b { 0x01 } else { 0 };

    // R14 as base always needs SIB (base enc 6 doesn't conflict, but
    // if no index we still need SIB when base is R14 and encoding requires it)
    // Actually: base=R14(enc 6) with mod!=11 and rm=6 → no SIB needed (only rm=4 forces SIB)
    // But if there IS an index register, we need SIB.
    let need_sib = has_index || base_enc == 4; // rm=4 → SIB; or explicit index

    if rex != 0x40 {
        code.push(rex);
    }
    code.push(0x8D); // LEA opcode

    if need_sib {
        // ModRM: mod=XX reg=dest rm=4 (SIB follows)
        let modrm = (mod_bits << 6) | (dest_enc << 3) | 0x04;
        code.push(modrm);
        // SIB: scale | index | base
        let scale_bits = match scale {
            1 => 0b00,
            2 => 0b01,
            4 => 0b10,
            8 => 0b11,
            _ => 0b00,
        };
        let sib = (scale_bits << 6) | (index_enc << 3) | base_enc;
        code.push(sib);
    } else {
        // ModRM: mod=XX reg=dest rm=base
        let modrm = (mod_bits << 6) | (dest_enc << 3) | base_enc;
        code.push(modrm);
    }

    // Displacement
    match mod_bits {
        0b01 => code.push(disp as u8),                                // disp8
        0b10 => code.extend_from_slice(&(disp as i32).to_le_bytes()), // disp32
        _ => {}                                                       // no displacement
    }

    true
}

/// Map a 32-bit x86 register to its 3-bit encoding and REX.B/X flag.
fn reg32_to_enc(reg: Register) -> (u8, bool) {
    match reg {
        Register::EAX | Register::RAX => (0, false),
        Register::ECX | Register::RCX => (1, false),
        Register::EDX | Register::RDX => (2, false),
        Register::EBX | Register::RBX => (3, false),
        Register::ESP | Register::RSP => (4, false),
        Register::EBP | Register::RBP => (5, false),
        Register::ESI | Register::RSI => (6, false),
        Register::EDI | Register::RDI => (7, false),
        Register::R14 => (6, true),
        Register::None => (0, false),
        _ => (0, false),
    }
}

/// Replace ESP/RSP/SP with R14/R14D in ALL operand positions:
/// register operands, memory base, and memory index.
fn replace_esp_in_operands(instr: &mut Instruction) {
    // Register operands (MOV EBP,ESP → MOV EBP,R14D)
    for i in 0..instr.op_count() {
        if instr.op_kind(i) == OpKind::Register {
            match instr.op_register(i) {
                Register::ESP => {
                    instr.set_op_register(i, Register::R14D);
                }
                Register::RSP => {
                    instr.set_op_register(i, Register::R14D);
                }
                Register::SP => {
                    instr.set_op_register(i, Register::R14W);
                }
                _ => {}
            }
        }
    }
    // Memory base register (LEA EBP,[ESP+8] → LEA EBP,[R14+8])
    match instr.memory_base() {
        Register::ESP | Register::RSP => {
            instr.set_memory_base(Register::R14);
        }
        _ => {}
    }
    // Memory index register (rare but possible: [EAX+ESP*1])
    match instr.memory_index() {
        Register::ESP | Register::RSP => {
            instr.set_memory_index(Register::R14);
        }
        _ => {}
    }
}

/// Encode a single instruction using iced-x86's 64-bit Encoder.
/// Returns true on success, false if it emitted a NOP fallback.
fn encode_to(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let mut encoder = Encoder::new(64);
    match encoder.encode(instr, 0) {
        Ok(_len) => {
            code.extend_from_slice(&encoder.take_buffer());
            true
        }
        Err(_e) => {
            // Emit NOP so execution flows through (skip failed instruction)
            code.push(0x90);
            false
        }
    }
}

/// Track encode failures during emission for logging.
struct EncodeFailures {
    count: u32,
    first_few: Vec<(u32, String)>, // (guest_addr, mnemonic)
}

impl EncodeFailures {
    fn new() -> Self {
        Self {
            count: 0,
            first_few: Vec::new(),
        }
    }
    fn record(&mut self, guest_addr: u32, mnemonic: Mnemonic) {
        self.count += 1;
        if self.first_few.len() < 10 {
            self.first_few.push((guest_addr, format!("{:?}", mnemonic)));
        }
    }
}

// ============================================================================
// Emission helpers — stack operations (manual, since these are always the same)
// ============================================================================

/// PUSH reg (0x50-0x57) → 8 bytes
fn emit_push_reg(code: &mut Vec<u8>, opcode: u8) {
    let reg_idx = opcode & 0x07;
    // LEA R14D, [R14-4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0xFC]);
    // MOV [R15+R14], reg32
    let modrm = 0x04 | (reg_idx << 3);
    code.extend_from_slice(&[0x43, 0x89, modrm, 0x37]);
}

/// POP reg (0x58-0x5F) → 8 bytes
fn emit_pop_reg(code: &mut Vec<u8>, opcode: u8) {
    let reg_idx = opcode & 0x07;
    // MOV reg32, [R15+R14]
    let modrm = 0x04 | (reg_idx << 3);
    code.extend_from_slice(&[0x43, 0x8B, modrm, 0x37]);
    // LEA R14D, [R14+4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0x04]);
}

/// PUSH ESP (0x54) → 9 bytes
/// Intel SDM contract (286+): push the PRE-decrement value of ESP.
/// Store-first pattern: write old R14 to [R15+R14-4], then drop R14 by 4.
///   MOV [R15+R14-4], R14D    ; 47 89 74 37 FC  (disp8 = -4)
///   LEA R14D, [R14-4]        ; 45 8D 76 FC
fn emit_push_esp(code: &mut Vec<u8>) {
    // MOV [R15+R14-4], R14D
    //   REX.WRXB = 0111 (R=R14 src, X=R14 index, B=R15 base)
    //   opcode 89 (MOV r/m32, r32)
    //   mod=01 (disp8), reg=R14&7=6, rm=100 (SIB) → ModR/M = 0x74
    //   SIB: scale=00, index=R14&7=6, base=R15&7=7 → 0x37
    //   disp8 = -4 = 0xFC
    code.extend_from_slice(&[0x47, 0x89, 0x74, 0x37, 0xFC]);
    // LEA R14D, [R14-4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0xFC]);
}

/// POP ESP (0x5C) → 4 bytes
/// Intel SDM: DEST := [SS:ESP]; ESP += 4. Since DEST==ESP, the +4 is clobbered
/// by the load. Net semantics: R14D := [R15+R14]. No post-load increment.
///   MOV R14D, [R15+R14]      ; 47 8B 34 37
fn emit_pop_esp(code: &mut Vec<u8>) {
    // MOV R14D, [R15+R14]
    //   REX.WRXB = 0111 (R=R14 dst, X=R14 index, B=R15 base)
    //   opcode 8B (MOV r32, r/m32)
    //   mod=00, reg=R14&7=6, rm=100 (SIB) → ModR/M = 0x34
    //   SIB: scale=00, index=R14&7=6, base=R15&7=7 → 0x37
    code.extend_from_slice(&[0x47, 0x8B, 0x34, 0x37]);
}

/// PUSH imm32/imm8 → 12 bytes
/// CRITICAL: For `push imm8` (opcode 0x6A), x86 sign-extends the 8-bit immediate to 32 bits.
/// `push -1` (0x6A 0xFF) must push 0xFFFFFFFF, NOT 0x000000FF.
/// iced-x86's `immediate32to64()` does NOT sign-extend Immediate8to32 operands —
/// it returns the raw byte value. Use `immediate8to32()` which correctly sign-extends.
fn emit_push_imm(code: &mut Vec<u8>, di: &DecodedInstr) {
    let imm = match di.instruction.op_kind(0) {
        iced_x86::OpKind::Immediate8to32 => {
            // Sign-extend 8-bit to 32-bit (e.g., 0xFF → 0xFFFFFFFF = -1)
            di.instruction.immediate8to32() as u32
        }
        _ => {
            // Already 32-bit immediate
            di.instruction.immediate32() as u32
        }
    };
    // LEA R14D, [R14-4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0xFC]);
    // MOV DWORD [R15+R14], imm32
    code.extend_from_slice(&[0x43, 0xC7, 0x04, 0x37]);
    code.extend_from_slice(&imm.to_le_bytes());
}

/// PUSH [mem] → MOV R11D, [R15+base+disp]; LEA R14D, [R14-4]; MOV [R15+R14], R11D
/// Rewrites memory operand to use R15, reads into R11D, then pushes to guest stack.
fn emit_push_mem(code: &mut Vec<u8>, instr: &Instruction, _guest_bytes: &[u8]) -> bool {
    let saved = code.len();

    // Step 1: Emit MOV R11D, [R15-rebased memory] to read the value
    if !emit_mov_r11d_from_guest_mem(code, instr) {
        code.truncate(saved);
        return false;
    }

    // Step 2: LEA R14D, [R14-4] — decrement guest ESP
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0xFC]);
    // Step 3: MOV [R15+R14], R11D — write to guest stack
    code.extend_from_slice(&[0x47, 0x89, 0x1C, 0x37]);
    true
}

/// POP [mem] → MOV R11D, [R15+R14]; LEA R14D, [R14+4]; MOV [R15+base+disp], R11D
/// Pops from guest stack into R11D, then writes to guest memory with R15 rebase.
fn emit_pop_mem(code: &mut Vec<u8>, instr: &Instruction, _guest_bytes: &[u8]) -> bool {
    let saved = code.len();

    // Step 1: MOV R11D, [R15+R14] — pop from guest stack
    code.extend_from_slice(&[0x47, 0x8B, 0x1C, 0x37]);
    // Step 2: LEA R14D, [R14+4] — increment guest ESP
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0x04]);

    // Step 3: Write R11D to guest memory with R15 rebase
    if !emit_mov_to_guest_mem_r11d(code, instr) {
        code.truncate(saved);
        return false;
    }

    true
}

/// Emit MOV R11D, [R15-rebased memory operand from instruction]
fn emit_mov_r11d_from_guest_mem(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let base = instr.memory_base();
    let index = instr.memory_index();
    let scale = instr.memory_index_scale();
    let fs_adj = if instr.segment_prefix() == Register::FS || instr.segment_prefix() == Register::GS
    {
        FAKE_KPCR_BASE_ADDR as i64
    } else {
        0
    };
    let disp = instr.memory_displacement64() as i64 + fs_adj;

    // Replace ESP references with R14
    let base = if base == Register::ESP {
        Register::R14D
    } else {
        base
    };
    let index = if index == Register::ESP {
        Register::R14D
    } else {
        index
    };

    let mem_op = if base == Register::None && index == Register::None {
        iced_x86::MemoryOperand::with_base_displ(Register::R15, disp)
    } else if index == Register::None {
        let base64 = to_64bit(base);
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(Register::R15, base64, 1, disp, 8)
    } else if base == Register::None {
        let idx64 = to_64bit(index);
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            Register::R15,
            idx64,
            scale,
            disp,
            8,
        )
    } else {
        // [base+index*scale+disp]: LEA R11,[R15+base64] then MOV R11D,[R11+idx64*scale+disp]
        let base64 = to_64bit(base);
        let idx64 = to_64bit(index);
        let lea = Instruction::with2(
            iced_x86::Code::Lea_r64_m,
            Register::R11,
            iced_x86::MemoryOperand::with_base_index(Register::R15, base64),
        )
        .unwrap();
        if !encode_to(code, &lea) {
            return false;
        }
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            Register::R11,
            idx64,
            scale,
            disp,
            8,
        )
    };

    match Instruction::with2(iced_x86::Code::Mov_r32_rm32, Register::R11D, mem_op) {
        Ok(mov) => encode_to(code, &mov),
        Err(_) => false,
    }
}

/// Emit MOV [R15-rebased memory operand from instruction], R11D
fn emit_mov_to_guest_mem_r11d(code: &mut Vec<u8>, instr: &Instruction) -> bool {
    let base = instr.memory_base();
    let index = instr.memory_index();
    let scale = instr.memory_index_scale();
    let fs_adj = if instr.segment_prefix() == Register::FS || instr.segment_prefix() == Register::GS
    {
        FAKE_KPCR_BASE_ADDR as i64
    } else {
        0
    };
    let disp = instr.memory_displacement64() as i64 + fs_adj;

    // Replace ESP references with R14
    let base = if base == Register::ESP {
        Register::R14D
    } else {
        base
    };
    let index = if index == Register::ESP {
        Register::R14D
    } else {
        index
    };

    let mem_op = if base == Register::None && index == Register::None {
        iced_x86::MemoryOperand::with_base_displ(Register::R15, disp)
    } else if index == Register::None {
        let base64 = to_64bit(base);
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(Register::R15, base64, 1, disp, 8)
    } else if base == Register::None {
        let idx64 = to_64bit(index);
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            Register::R15,
            idx64,
            scale,
            disp,
            8,
        )
    } else {
        // R11 is in use (holds value), use R10 as temp for address
        let base64 = to_64bit(base);
        let idx64 = to_64bit(index);
        let lea = Instruction::with2(
            iced_x86::Code::Lea_r64_m,
            Register::R10,
            iced_x86::MemoryOperand::with_base_index(Register::R15, base64),
        )
        .unwrap();
        if !encode_to(code, &lea) {
            return false;
        }
        iced_x86::MemoryOperand::with_base_index_scale_displ_size(
            Register::R10,
            idx64,
            scale,
            disp,
            8,
        )
    };

    match Instruction::with2(iced_x86::Code::Mov_rm32_r32, mem_op, Register::R11D) {
        Ok(mov) => encode_to(code, &mov),
        Err(_) => false,
    }
}

/// CALL rel32 → ~17 bytes (push ret addr to guest stack + JMP)
/// R12 shadow stack push DISABLED — GPS-RET is disabled, RET uses inline
/// guest-hash dispatch (emit_inline_guest_ret). Unbalanced R12 push without
/// pop caused shadow stack overflow (17M+ VEH exceptions in frame loops).
/// x86 CALL preserves EFLAGS, so the guest-stack decrement must use LEA rather
/// than SUB to avoid poisoning a caller's pending Jcc.
fn emit_call_rel(
    code: &mut Vec<u8>,
    ret_addr: u32,
    fixups: &mut Vec<BranchFixup>,
    target: Option<u32>,
) {
    // LEA R14D, [R14-4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0xFC]);
    // MOV DWORD [R15+R14], ret_addr
    code.extend_from_slice(&[0x43, 0xC7, 0x04, 0x37]);
    code.extend_from_slice(&ret_addr.to_le_bytes());
    // JMP rel32 (placeholder)
    let fixup_pos = code.len() as u32 + 1;
    code.push(0xE9);
    code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    if let Some(target_addr) = target {
        fixups.push(BranchFixup {
            host_offset: fixup_pos,
            guest_target: target_addr,
            instr_end_delta: 4,
        });
    }
}

/// RET → shadow stack pop + bounds check + cleanup + JMP R10.
/// Falls back to INT3 if R10 is 0 (underflow) OR outside code buffer.
/// Uses R13 (RuntimeContext pointer) to read code_base and code_size.
/// RET with inline shadow stack pop. Zero check → INT3 fallback.
/// ADD R14 is AFTER the JZ so fallback doesn't double-clean.
fn emit_ret_checked(code: &mut Vec<u8>, cleanup: u16) {
    // MOV R10, [R12]          ; 4 bytes
    code.extend_from_slice(&[0x4D, 0x8B, 0x14, 0x24]);
    // ADD R12, 8              ; 4 bytes
    code.extend_from_slice(&[0x49, 0x83, 0xC4, 0x08]);
    // TEST R10, R10           ; 3 bytes
    code.extend_from_slice(&[0x4D, 0x85, 0xD2]);
    // JZ to INT3 fallback     ; 2 bytes — skip ADD + JMP
    let total_cleanup = 4u32 + cleanup as u32;
    let add_size: u8 = if total_cleanup <= 127 { 4 } else { 7 };
    let jz_offset = add_size + 3; // ADD R14 + JMP R10
    code.extend_from_slice(&[0x74, jz_offset]);
    // ADD R14D, 4+cleanup     ; 4 or 7 bytes (normal path only)
    if total_cleanup <= 127 {
        code.extend_from_slice(&[0x41, 0x83, 0xC6, total_cleanup as u8]);
    } else {
        code.extend_from_slice(&[0x41, 0x81, 0xC6]);
        code.extend_from_slice(&total_cleanup.to_le_bytes());
    }
    // JMP R10                 ; 3 bytes
    code.extend_from_slice(&[0x41, 0xFF, 0xE2]);
    // INT3 fallback           ; 1 byte (R14 NOT adjusted)
    code.push(0xCC);
}

/// RET → shadow stack pop + underflow check + cleanup + JMP R10.
/// On underflow (R10=0), INT3 fallback lets VEH handle via guest stack.
/// CRITICAL: ADD R14 must be AFTER the underflow check, otherwise the
/// VEH fallback double-cleans R14 (emit_ret + VEH handler both add).
fn emit_ret(code: &mut Vec<u8>, cleanup: u16) {
    // MOV R10, [R12]
    code.extend_from_slice(&[0x4D, 0x8B, 0x14, 0x24]);
    // ADD R12, 8
    code.extend_from_slice(&[0x49, 0x83, 0xC4, 0x08]);
    // TEST R10, R10 — check for shadow stack underflow (0 = no entry)
    code.extend_from_slice(&[0x4D, 0x85, 0xD2]);
    // JNZ over INT3 — calculate jump offset based on cleanup size
    // INT3 = 1 byte, then ADD R14 + JMP R10 follow
    code.extend_from_slice(&[0x75, 0x01]);
    // INT3 — fallback: VEH reads guest stack (R14 NOT yet adjusted)
    code.push(0xCC);
    // Normal path: ADD R14D, 4+cleanup (only reached if R10 != 0)
    let total_cleanup = 4u32 + cleanup as u32;
    if total_cleanup <= 127 {
        code.extend_from_slice(&[0x41, 0x83, 0xC6, total_cleanup as u8]);
    } else {
        code.extend_from_slice(&[0x41, 0x81, 0xC6]);
        code.extend_from_slice(&total_cleanup.to_le_bytes());
    }
    // JMP R10 — jump to shadow stack host address
    code.extend_from_slice(&[0x41, 0xFF, 0xE2]);
}

/// Inline guest-hash RET: read [R15+R14], adjust R14, probe addr_hash, jump.
/// Fallback to INT3 (TrapType::RetMiss) on hash miss. ~86 bytes per RET site.
/// Registers used as scratch: R8, R9, R10, R11. Does NOT clobber R15/R14/R13/R12/RAX-RDI.
///
/// Layout offsets (verified by compile-time assertions in runtime.rs):
///   [R13+0x48] = code_base (ptr)     [R13+0x58] = addr_hash.entries (ptr)
///   [R13+0x64] = addr_hash.mask (u32)
///   HashEntry: +0 guest_addr (u32), +4 host_offset (u32), size=12 bytes
fn emit_inline_guest_ret(
    code: &mut Vec<u8>,
    traps: &mut Vec<TrapInfo>,
    cleanup: u16,
    guest_addr: u32,
    hash_shift: u8,
) {
    let total_cleanup = 4u32 + cleanup as u32;

    // Step 1: MOV R10D, [R15+R14] — read guest return address (4 bytes)
    code.extend_from_slice(&[0x47, 0x8B, 0x14, 0x37]);

    // Step 2: ADD R14D, 4+cleanup — adjust guest ESP atomically with read
    if total_cleanup <= 127 {
        code.extend_from_slice(&[0x41, 0x83, 0xC6, total_cleanup as u8]);
    } else {
        code.extend_from_slice(&[0x41, 0x81, 0xC6]);
        code.extend_from_slice(&total_cleanup.to_le_bytes());
    }

    // Step 3: R12 pop DISABLED — GPS-RET is disabled (C++ v1 never used R12 for RET).
    // Removing the pop prevents shadow stack overflow when game_main loops internally
    // (each call/kernel-return cycle leaked 8 bytes from unbalanced push/pop).
    // code.extend_from_slice(&[0x49, 0x83, 0xC4, 0x08]);

    // Step 4: TEST R10D, R10D — RET-to-zero check
    code.extend_from_slice(&[0x45, 0x85, 0xD2]);
    let jz_pos = code.len();
    code.extend_from_slice(&[0x74, 0x00]); // JZ .fallback (placeholder)

    // Step 5: Mirror normalization (0x80000000-0x9FFFFFFF only)
    // MOV R11D, R10D
    code.extend_from_slice(&[0x45, 0x89, 0xD3]);
    // SUB R11D, 0x80000000
    code.extend_from_slice(&[0x41, 0x81, 0xEB, 0x00, 0x00, 0x00, 0x80]);
    // CMP R11D, 0x20000000
    code.extend_from_slice(&[0x41, 0x81, 0xFB, 0x00, 0x00, 0x00, 0x20]);
    // JAE .no_mirror (+7 bytes to skip AND)
    code.extend_from_slice(&[0x73, 0x07]);
    // AND R10D, 0x1FFFFFFF
    code.extend_from_slice(&[0x41, 0x81, 0xE2, 0xFF, 0xFF, 0xFF, 0x1F]);

    // Step 6: Hash computation
    // IMUL R11D, R10D, 0x9E3779B1
    code.extend_from_slice(&[0x45, 0x69, 0xDA, 0xB1, 0x79, 0x37, 0x9E]);
    // SHR R11D, hash_shift
    code.extend_from_slice(&[0x41, 0xC1, 0xEB, hash_shift]);
    // AND R11D, [R13+0x64] (mask)
    code.extend_from_slice(&[0x45, 0x23, 0x5D, 0x64]);

    // Step 7: Load entries pointer + compute entry address
    // MOV R9, [R13+0x58] (entries ptr)
    code.extend_from_slice(&[0x4D, 0x8B, 0x4D, 0x58]);
    // IMUL R8D, R11D, 12 (entry offset = idx * sizeof(HashEntry))
    code.extend_from_slice(&[0x45, 0x6B, 0xC3, 0x0C]);
    // ADD R9, R8 (R9 = &entries[idx])
    code.extend_from_slice(&[0x4D, 0x01, 0xC1]);

    // Step 8: Probe 0 compare
    // CMP [R9], R10D (entry.guest_addr vs normalized return address)
    code.extend_from_slice(&[0x45, 0x39, 0x11]);
    let jne_pos = code.len();
    code.extend_from_slice(&[0x75, 0x00]); // JNE .fallback (placeholder)

    // Step 9: Hit — store last RET target for watchdog, then jump
    // MOV [R15+0x3F1D40], R10D — write guest RET target to probe address
    // (0x3F1D40 is unused guest memory near VBlank flag at 0x3F1D48)
    code.extend_from_slice(&[0x45, 0x89, 0x97, 0x40, 0x1D, 0x3F, 0x00]);
    // MOV R11D, [R9+4] (host_offset)
    code.extend_from_slice(&[0x45, 0x8B, 0x59, 0x04]);
    // ADD R11, [R13+0x48] (code_base, 64-bit pointer add)
    code.extend_from_slice(&[0x4D, 0x03, 0x5D, 0x48]);
    // JMP R11
    code.extend_from_slice(&[0x41, 0xFF, 0xE3]);

    // .fallback: INT3 — TrapType::RetMiss (R14+R12 already adjusted)
    let fallback_pos = code.len();
    traps.push(TrapInfo {
        host_offset: fallback_pos as u32,
        trap_type: TrapType::RetMiss,
        ret_cleanup: 0, // R14 ALREADY adjusted — VEH must NOT re-adjust
        guest_addr,
    });
    code.push(0xCC);

    // Patch jump offsets
    code[jz_pos + 1] = (fallback_pos - (jz_pos + 2)) as u8;
    code[jne_pos + 1] = (fallback_pos - (jne_pos + 2)) as u8;
}

/// LEAVE → 11 bytes
fn emit_leave(code: &mut Vec<u8>, _di: &DecodedInstr, _before_len: usize) {
    // MOV R14D, EBP
    code.extend_from_slice(&[0x41, 0x89, 0xEE]);
    // MOV EBP, [R15+R14]
    code.extend_from_slice(&[0x43, 0x8B, 0x2C, 0x37]);
    // LEA R14D, [R14+4]
    code.extend_from_slice(&[0x45, 0x8D, 0x76, 0x04]);
}

/// INC/DEC short form → long form (2 bytes)
fn emit_incdec_long(code: &mut Vec<u8>, opcode: u8) {
    let reg = opcode & 0x07;
    if opcode < 0x48 {
        code.extend_from_slice(&[0xFF, 0xC0 | reg]);
    } else {
        code.extend_from_slice(&[0xFF, 0xC8 | reg]);
    }
}

/// String ops → wrap with R15 add/sub.
///
/// Flag preservation (E1 fix, 2026-04-22):
///   - `Cmpsb/w/d` and `Scasb/w/d` produce meaningful flag output (ZF, CF,
///     etc.) that guest code reads via a following Jcc. `Movs/Stos/Lods`
///     don't set flags but may run between a guest CMP and a Jcc and must
///     not clobber the incoming flags.
///   - Previous version wrapped the whole sequence in PUSHFQ/POPFQ. That
///     saved the flags *before* the string op, then restored those same
///     flags *after*, wiping any output from cmps/scas. Broke every
///     `rep cmpsd; je label` idiom (memcmp, strcmp, XBE section lookup).
///   - New version uses LEA for the `+R15` (LEA doesn't touch flags), so
///     no guard is needed on the entry side. After the string op, PUSHFQ
///     preserves whatever flags the op produced (its output for cmps/scas,
///     or the pre-op flags for movs/stos/lods which don't write), then
///     SUB/MOV reverse the base adjustment (SUB does clobber flags), then
///     POPFQ restores the string-op flags for the following Jcc.
fn emit_string_op(code: &mut Vec<u8>, di: &DecodedInstr, guest_bytes: &[u8]) {
    let m = di.instruction.mnemonic();
    let uses_esi = matches!(
        m,
        Mnemonic::Movsb
            | Mnemonic::Movsw
            | Mnemonic::Movsd
            | Mnemonic::Movsq
            | Mnemonic::Cmpsb
            | Mnemonic::Cmpsw
            | Mnemonic::Cmpsd
            | Mnemonic::Lodsb
            | Mnemonic::Lodsw
            | Mnemonic::Lodsd
    );

    // 2026-04-24 fix (phase50/51 + stack_writer_at_EE482): zero-extend
    // EDI/ESI/ECX to 64-bit BEFORE the LEA composes guest+R15 host addr.
    // Guest x86 treats these as 32-bit; host x64 uses the full 64-bit RDI.
    // If prior code left dirty high bits in any of these, the LEA (REX.W=1)
    // generates a wildly wrong host address and `rep movsd`/`stosd` writes
    // to random memory — including the outer frame's saved RET slot,
    // causing the 0x1EFFFDC0 / 0xDEADBEEF / 0xE8CB8B52 stack-smash pattern.
    // MOV r32,r32 zero-extends on x64 and doesn't touch flags.
    code.extend_from_slice(&[0x89, 0xFF]); // mov edi, edi (zero-extend RDI)
    if uses_esi {
        code.extend_from_slice(&[0x89, 0xF6]); // mov esi, esi (zero-extend RSI)
    }
    code.extend_from_slice(&[0x89, 0xC9]); // mov ecx, ecx (zero-extend RCX for REP count)

    // LEA RDI, [RDI + R15*1]  — no flag impact (ADD would clobber).
    //   REX.WX = 0x4A (W=1 for 64-bit, X=1 for R15 index extension)
    //   opcode 0x8D (LEA)
    //   ModRM 0x3C (mod=00, reg=RDI=111, rm=100 → SIB follows)
    //   SIB 0x3F (scale=00, index=R15=111, base=RDI=111)
    code.extend_from_slice(&[0x4A, 0x8D, 0x3C, 0x3F]);
    if uses_esi {
        // LEA RSI, [RSI + R15*1]
        //   ModRM 0x34 (reg=RSI=110)
        //   SIB 0x3E (base=RSI=110)
        code.extend_from_slice(&[0x4A, 0x8D, 0x34, 0x3E]);
    }

    // Original guest instruction (with any REP prefix).
    // For cmps/scas this SETS ZF/CF/SF/OF/AF/PF; for movs/stos/lods
    // it leaves flags untouched.
    code.extend_from_slice(guest_bytes);

    // Save the string op's output flags (or the caller's incoming flags
    // if movs/stos/lods passed them through unchanged).
    code.push(0x9C); // PUSHFQ

    // SUB RDI, R15 — clobbers flags, but we just saved them.
    code.extend_from_slice(&[0x4C, 0x29, 0xFF]);
    if uses_esi {
        // SUB RSI, R15
        code.extend_from_slice(&[0x4C, 0x29, 0xFE]);
    }
    // MOV EDI, EDI / MOV ESI, ESI — zero-extend to 64-bit (no flag impact).
    code.extend_from_slice(&[0x89, 0xFF]);
    if uses_esi {
        code.extend_from_slice(&[0x89, 0xF6]);
    }

    // POPFQ — restore string op's output flags for the following Jcc.
    code.push(0x9D);
}

fn get_ret_cleanup(di: &DecodedInstr) -> u16 {
    if di.instruction.mnemonic() == Mnemonic::Ret && di.length > 1 {
        di.instruction.immediate16() as u16
    } else {
        0
    }
}

// ============================================================================
// Branch fixup resolution
// ============================================================================

fn resolve_fixups(code: &mut [u8], fixups: &[BranchFixup], addr_map: &[AddrEntry]) {
    for fixup in fixups {
        if let Some(target_offset) = lookup_addr_map(addr_map, fixup.guest_target) {
            let rel32 =
                target_offset as i64 - (fixup.host_offset as i64 + fixup.instr_end_delta as i64);
            let rel32 = rel32 as i32;
            let pos = fixup.host_offset as usize;
            if pos + 4 <= code.len() {
                code[pos..pos + 4].copy_from_slice(&rel32.to_le_bytes());
            }
        }
    }
}

fn find_unresolved(fixups: &[BranchFixup], addr_map: &[AddrEntry]) -> Vec<BranchFixup> {
    fixups
        .iter()
        .filter(|f| lookup_addr_map(addr_map, f.guest_target).is_none())
        .copied()
        .collect()
}

fn lookup_addr_map(addr_map: &[AddrEntry], guest_addr: u32) -> Option<u32> {
    addr_map
        .binary_search_by_key(&guest_addr, |e| e.guest_addr)
        .ok()
        .map(|i| addr_map[i].host_offset)
}

// ============================================================================
// Post-emit verification: scan x64 code for memory accesses without R15 base
// ============================================================================

/// Decode the emitted x64 code and flag any memory access that doesn't use R15 base.
/// These are potential crash sources — the instruction will access host memory instead of
/// guest memory via R15.
fn verify_r15_base(
    code: &[u8],
    addr_map: &[AddrEntry],
    _traps: &[crate::xbox::aot::runtime::TrapInfo],
) {
    use iced_x86::{Decoder, DecoderOptions, Register};

    // Scan at each addr_map boundary — decode one x64 instruction per guest instruction
    // to avoid misalignment from the linear x64 decoder.
    let mut bad_count = 0u32;
    let mut bad_samples: Vec<String> = Vec::new();

    for (i, entry) in addr_map.iter().enumerate() {
        let start = entry.host_offset as usize;
        let end = if i + 1 < addr_map.len() {
            (addr_map[i + 1].host_offset as usize).min(code.len())
        } else {
            code.len()
        };
        if start >= end || start >= code.len() {
            continue;
        }

        let chunk = &code[start..end];
        let mut decoder = Decoder::with_ip(64, chunk, start as u64, DecoderOptions::NONE);

        // Decode all instructions in this chunk (one guest instr may emit multiple x64 instrs)
        while decoder.can_decode() {
            let offset = decoder.position() as u32 + entry.host_offset;
            let instr = decoder.decode();
            if instr.is_invalid() {
                break;
            }

            if instr.mnemonic() == iced_x86::Mnemonic::Int3 {
                continue;
            }
            if instr.mnemonic() == iced_x86::Mnemonic::Nop {
                continue;
            }
            if instr.mnemonic() == iced_x86::Mnemonic::Lea {
                continue;
            }

            let has_mem =
                (0..instr.op_count()).any(|i| instr.op_kind(i) == iced_x86::OpKind::Memory);
            if !has_mem {
                continue;
            }

            let base = instr.memory_base();
            let index = instr.memory_index();

            // R11 is scratch for base+index rewrites — LEA R11,[R15+base] precedes
            let ok = base == Register::R15 || index == Register::R15
                || base == Register::R11 || index == Register::R11
                || base == Register::RSP || base == Register::R12
                || base == Register::R13
                || base == Register::R9  // inline RET hash table probe (host memory)
                || base == Register::RIP
                || instr.is_ip_rel_memory_operand();

            if !ok {
                // Filter out decoder misalignment artifacts (00 00 = add [rax],al etc.)
                let mnem = instr.mnemonic();
                if mnem == iced_x86::Mnemonic::Add && instr.len() <= 2 {
                    continue;
                }
                if mnem == iced_x86::Mnemonic::Cmp && instr.len() <= 2 {
                    continue;
                }
                if mnem == iced_x86::Mnemonic::Xor && instr.len() <= 2 {
                    continue;
                }
                if mnem == iced_x86::Mnemonic::Mov && base == Register::RDX {
                    continue;
                } // MOV [rdx+N],seg
                bad_count += 1;
                if bad_samples.len() < 50 {
                    let mut fmt = iced_x86::IntelFormatter::new();
                    let mut output = String::new();
                    iced_x86::Formatter::format(&mut fmt, &instr, &mut output);
                    bad_samples.push(format!(
                        "host=0x{:X} guest=0x{:08X} base={:?} idx={:?} {}",
                        offset, entry.guest_addr, base, index, output
                    ));
                }
            }
        }
    }

    if bad_count > 0 {
        crate::xbox::emulator::debug_log(&format!(
            "VERIFY: {} emitted instructions access memory without R15 base!",
            bad_count
        ));
        for s in &bad_samples {
            crate::xbox::emulator::debug_log(&format!("  BAD: {}", s));
        }
    } else {
        crate::xbox::emulator::debug_log("VERIFY: all emitted memory accesses use R15 base (OK)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xbox::aot::decoder;

    /// Synthetic stdcall test: verify that `ret 0xC` produces cleanup=12 in the trap table.
    /// This is the P0 theory: if get_ret_cleanup returns 0 instead of 12 for `C2 0C 00`,
    /// the VEH won't clean the caller's stack after a stdcall return.
    #[test]
    fn ret_0xc_cleanup_value() {
        // x86 bytes: ret 0xC (C2 0C 00)
        let bytes: &[u8] = &[0xC2, 0x0C, 0x00];
        let (instrs, _) = decoder::decode_section(bytes, 0x1000);
        assert_eq!(instrs.len(), 1, "should decode 1 instruction");
        assert_eq!(instrs[0].instruction.mnemonic(), Mnemonic::Ret);
        let cleanup = get_ret_cleanup(&instrs[0]);
        assert_eq!(
            cleanup, 12,
            "ret 0xC should have cleanup=12, got {}",
            cleanup
        );
    }

    /// Verify bare `ret` (C3) produces cleanup=0.
    #[test]
    fn bare_ret_cleanup_value() {
        let bytes: &[u8] = &[0xC3];
        let (instrs, _) = decoder::decode_section(bytes, 0x1000);
        assert_eq!(instrs.len(), 1);
        let cleanup = get_ret_cleanup(&instrs[0]);
        assert_eq!(
            cleanup, 0,
            "bare ret should have cleanup=0, got {}",
            cleanup
        );
    }

    fn decode_x64_regs(bytes: &[u8]) -> Vec<Register> {
        let mut regs = Vec::new();
        let mut decoder = iced_x86::Decoder::new(64, bytes, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            let instr = decoder.decode();
            for i in 0..instr.op_count() {
                let reg = instr.op_register(i);
                if reg != Register::None {
                    regs.push(reg);
                }
            }
            let base = instr.memory_base();
            if base != Register::None {
                regs.push(base);
            }
            let index = instr.memory_index();
            if index != Register::None {
                regs.push(index);
            }
        }
        regs
    }

    fn assert_no_rex_high_byte_aliases(bytes: &[u8]) {
        let bad = [Register::SPL, Register::BPL, Register::SIL, Register::DIL];
        let regs = decode_x64_regs(bytes);
        assert!(
            !regs.iter().any(|r| bad.contains(r)),
            "emitted x64 must not accidentally encode AH/CH/DH/BH as SPL/BPL/SIL/DIL: {:02X?} regs={:?}",
            bytes,
            regs
        );
    }

    #[test]
    fn high_byte_mem_load_rewrite_avoids_rex_alias() {
        // x86: mov ah, byte ptr [edi]
        let bytes = [0x8A, 0x27];
        let (instrs, _) = decoder::decode_section(&bytes, 0x1000);
        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].instruction.mnemonic(), Mnemonic::Mov);
        assert_eq!(instrs[0].instruction.op0_register(), Register::AH);
        assert_eq!(classify_for_emit(&instrs[0]), EmitStrategy::RewriteMem);

        let mut code = Vec::new();
        assert!(
            emit_memory_rewrite(&mut code, &instrs[0].instruction, &bytes),
            "high-byte memory load should be expanded successfully"
        );
        assert_eq!(code.first(), Some(&0x9C), "rewrite should preserve flags");
        assert_eq!(code.last(), Some(&0x9D), "rewrite should restore flags");
        assert_no_rex_high_byte_aliases(&code);

        let mut decoder = iced_x86::Decoder::new(64, &code, iced_x86::DecoderOptions::NONE);
        assert_eq!(decoder.decode().mnemonic(), Mnemonic::Pushfq);
        assert!(
            (0..8).any(|_| decoder.decode().mnemonic() == Mnemonic::Movzx),
            "expanded load should include a byte load via MOVZX: {:02X?}",
            code
        );
    }

    #[test]
    fn high_byte_mem_store_rewrite_avoids_rex_alias() {
        // x86: mov byte ptr [esi], ah
        let bytes = [0x88, 0x26];
        let (instrs, _) = decoder::decode_section(&bytes, 0x1000);
        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].instruction.mnemonic(), Mnemonic::Mov);
        assert_eq!(instrs[0].instruction.op1_register(), Register::AH);
        assert_eq!(classify_for_emit(&instrs[0]), EmitStrategy::RewriteMem);

        let mut code = Vec::new();
        assert!(
            emit_memory_rewrite(&mut code, &instrs[0].instruction, &bytes),
            "high-byte memory store should be expanded successfully"
        );
        assert_eq!(code.first(), Some(&0x9C), "rewrite should preserve flags");
        assert_eq!(code.last(), Some(&0x9D), "rewrite should restore flags");
        assert_no_rex_high_byte_aliases(&code);

        let regs = decode_x64_regs(&code);
        assert!(
            regs.contains(&Register::R11L),
            "expanded store should write the extracted high byte through R11L: {:02X?} regs={:?}",
            code,
            regs
        );
    }

    /// Verify `ret 0x14` (5 args stdcall) produces cleanup=20.
    #[test]
    fn ret_0x14_cleanup_value() {
        let bytes: &[u8] = &[0xC2, 0x14, 0x00];
        let (instrs, _) = decoder::decode_section(bytes, 0x1000);
        assert_eq!(instrs.len(), 1);
        let cleanup = get_ret_cleanup(&instrs[0]);
        assert_eq!(
            cleanup, 20,
            "ret 0x14 should have cleanup=20, got {}",
            cleanup
        );
    }

    /// Synthetic P0 scenario: decode the exact call pattern and verify cleanup extraction.
    #[test]
    fn synthetic_stdcall_decode() {
        let base = 0x1000u32;
        let mut code = vec![0x90u8; 0x30];

        // 0x1000: sub esp, 0x10
        code[0] = 0x83;
        code[1] = 0xEC;
        code[2] = 0x10;
        // 0x1003: push 1; push 2; push 3
        code[3] = 0x6A;
        code[4] = 0x01;
        code[5] = 0x6A;
        code[6] = 0x02;
        code[7] = 0x6A;
        code[8] = 0x03;
        // 0x1009: call 0x1020
        code[9] = 0xE8;
        let rel = 0x1020i32 - 0x100Ei32;
        code[10..14].copy_from_slice(&rel.to_le_bytes());
        // 0x100E: add esp, 0x10
        code[14] = 0x83;
        code[15] = 0xC4;
        code[16] = 0x10;
        // 0x1011: ret
        code[17] = 0xC3;
        // 0x1020: push ebp; mov ebp,esp; pop ebp; ret 0xC
        code[0x20] = 0x55;
        code[0x21] = 0x8B;
        code[0x22] = 0xEC;
        code[0x23] = 0x5D;
        code[0x24] = 0xC2;
        code[0x25] = 0x0C;
        code[0x26] = 0x00;

        let (instrs, _stats) = decoder::decode_section(&code, base);

        // Find both ret instructions
        let rets: Vec<_> = instrs
            .iter()
            .filter(|i| i.instruction.mnemonic() == Mnemonic::Ret)
            .collect();
        assert!(rets.len() >= 2, "need 2 rets, got {}", rets.len());

        let bare = rets
            .iter()
            .find(|i| i.guest_addr == 0x1011)
            .expect("bare ret at 0x1011");
        let stdcall = rets
            .iter()
            .find(|i| i.guest_addr == 0x1024)
            .expect("stdcall ret at 0x1024");

        assert_eq!(get_ret_cleanup(bare), 0, "bare ret cleanup should be 0");
        assert_eq!(get_ret_cleanup(stdcall), 12, "ret 0xC cleanup should be 12");

        // Compute expected stack balance through the caller (0x1000-0x1011)
        // sub esp,16 = -16; push×3 = -12; call = -4; add esp,16 = +16; ret = +4
        // With stdcall callee (ret 12): call net effect = -4 + 4 + 12 = +12
        // Total: -16 - 12 + 12 + 16 + 4 = +4 (balanced: +4 = return address pop)
        let mut balance: i32 = 0;
        for di in &instrs {
            if di.guest_addr < 0x1000 || di.guest_addr > 0x1011 {
                continue;
            }
            let m = di.instruction.mnemonic();
            let delta: i32 = match m {
                Mnemonic::Push => -4,
                Mnemonic::Pop => 4,
                Mnemonic::Ret if di.length > 1 => 4 + di.instruction.immediate16() as i32,
                Mnemonic::Ret => 4,
                Mnemonic::Call => {
                    // cdecl call: push ret addr, callee returns with ret 0xC
                    // net = -4 (push ret) + 4 (pop ret) + 12 (cleanup) = +12
                    12 // stdcall callee cleans args
                }
                Mnemonic::Sub if di.instruction.op0_register() == Register::ESP => {
                    -(di.instruction.immediate32() as i32)
                }
                Mnemonic::Add if di.instruction.op0_register() == Register::ESP => {
                    di.instruction.immediate32() as i32
                }
                _ => 0,
            };
            if delta != 0 {
                balance += delta;
            }
        }
        assert_eq!(
            balance, 4,
            "caller function balance should be +4 (ret addr pop), got {}",
            balance
        );
    }

    /// Spider-Man font-loader helper sub_26220 relies on `add esp, 4`
    /// before `pop esi; ret`. If this rewrite is skipped, RET sees the
    /// saved ESI as the return address and lands in RET-ONELOW recovery.
    #[test]
    fn add_esp_imm8_rewrites_to_r14d() {
        let bytes: &[u8] = &[0x83, 0xC4, 0x04]; // add esp, 4
        let (instrs, _) = decoder::decode_section(bytes, 0x26238);
        assert_eq!(instrs.len(), 1);
        assert_eq!(classify_for_emit(&instrs[0]), EmitStrategy::RewriteEspReg);

        let mut emitted = Vec::new();
        assert!(
            emit_esp_reg_rewrite(&mut emitted, &instrs[0].instruction),
            "add esp,4 must encode successfully instead of becoming a rescue/NOP"
        );

        assert_eq!(
            emitted,
            [0x41, 0x83, 0xC6, 0x04],
            "add esp,4 should become add r14d,4"
        );
    }

    /// The malloc thunk under sub_26220 uses `push dword ptr [esp+8]`.
    /// This must read from the guest stack via [R15+R14+8], not fall into
    /// rescue/NOP due to a mixed R14D/R14 memory operand.
    #[test]
    fn push_esp_disp8_memory_rewrites_to_guest_stack() {
        let bytes: &[u8] = &[0xFF, 0x74, 0x24, 0x08]; // push dword ptr [esp+8]
        let (instrs, _) = decoder::decode_section(bytes, 0x2B4FF9);
        assert_eq!(instrs.len(), 1);
        assert_eq!(classify_for_emit(&instrs[0]), EmitStrategy::RewritePushMem);

        let mut emitted = Vec::new();
        assert!(
            emit_push_mem(&mut emitted, &instrs[0].instruction, bytes),
            "push [esp+8] must encode successfully instead of becoming rescue/NOP"
        );

        assert_eq!(
            emitted,
            [
                0x47, 0x8B, 0x9C, 0x37, 0x08, 0x00, 0x00, 0x00, // mov r11d, [r15+r14+8]
                0x45, 0x8D, 0x76, 0xFC, // lea r14d, [r14-4]
                0x47, 0x89, 0x1C, 0x37 // mov [r15+r14], r11d
            ],
            "push [esp+8] should use the guest stack base"
        );
    }

    /// Extreme test: verify cleanup extraction for every possible ret imm16 value.
    /// Catches any edge cases in get_ret_cleanup for unusual cleanup sizes.
    #[test]
    fn ret_cleanup_all_common_values() {
        // Test every stdcall cleanup from 0 to 64 bytes (0 to 16 args)
        for cleanup in (0..=64u16).step_by(4) {
            let bytes: Vec<u8> = if cleanup == 0 {
                vec![0xC3] // bare ret
            } else {
                vec![0xC2, (cleanup & 0xFF) as u8, (cleanup >> 8) as u8] // ret imm16
            };
            let (instrs, _) = decoder::decode_section(&bytes, 0x1000);
            assert!(
                !instrs.is_empty(),
                "should decode ret with cleanup={}",
                cleanup
            );
            let got = get_ret_cleanup(&instrs[0]);
            assert_eq!(
                got, cleanup,
                "ret cleanup mismatch: bytes={:02X?} expected={} got={}",
                bytes, cleanup, got
            );
        }
    }

    /// Extreme test: verify cleanup for the EXACT Spider-Man D3D functions.
    /// Uses real guest bytes extracted from the P0 decode dump.
    #[test]
    fn spiderman_d3d_ret_cleanup() {
        // 0x002F1DA8: ret 0Ch — D3DDevice_SetVertexShaderConstant
        let (instrs, _) = decoder::decode_section(&[0xC2, 0x0C, 0x00], 0x002F1DA8);
        assert_eq!(
            get_ret_cleanup(&instrs[0]),
            12,
            "SetVertexShaderConstant ret 0xC"
        );

        // 0x002F1DCE: ret 0Ch — second exit point
        let (instrs, _) = decoder::decode_section(&[0xC2, 0x0C, 0x00], 0x002F1DCE);
        assert_eq!(
            get_ret_cleanup(&instrs[0]),
            12,
            "SetVertexShaderConstant alt ret 0xC"
        );

        // 0x002F0E0C: ret (bare) — the ONLY D3D ret that actually fired in runtime
        let (instrs, _) = decoder::decode_section(&[0xC3], 0x002F0E0C);
        assert_eq!(get_ret_cleanup(&instrs[0]), 0, "D3D bare ret");

        // 0x0029C874: ret (bare) — P0 crash site
        let (instrs, _) = decoder::decode_section(&[0xC3], 0x0029C874);
        assert_eq!(get_ret_cleanup(&instrs[0]), 0, "P0 crash site bare ret");

        // 0x0029C880: ret (bare) — function 0x0029C880
        let (instrs, _) = decoder::decode_section(&[0xC3], 0x0029C880);
        assert_eq!(get_ret_cleanup(&instrs[0]), 0, "0x0029C880 bare ret");

        // 0x0029CBCF: ret (bare) — the looping cdecl ret
        let (instrs, _) = decoder::decode_section(&[0xC3], 0x0029CBCF);
        assert_eq!(get_ret_cleanup(&instrs[0]), 0, "0x0029CBCF bare ret");
    }

    /// Extreme test: verify VEH R14 delta math for stdcall cleanup.
    /// Simulates what veh_dispatch.rs line 106 does.
    #[test]
    fn veh_r14_delta_math() {
        // Simulate: new_r14 = r14 + 4 + ret_cleanup
        // For bare ret: +4 (pop return address only)
        let r14: u32 = 0x00C1FE1C;
        assert_eq!(
            r14.wrapping_add(4).wrapping_add(0),
            0x00C1FE20,
            "bare ret: +4"
        );

        // For ret 4: +8 (pop ret + 1 arg)
        assert_eq!(r14.wrapping_add(4).wrapping_add(4), 0x00C1FE24, "ret 4: +8");

        // For ret 0xC: +16 (pop ret + 3 args)
        assert_eq!(
            r14.wrapping_add(4).wrapping_add(12),
            0x00C1FE2C,
            "ret 0xC: +16"
        );

        // For ret 0x14: +24 (pop ret + 5 args)
        assert_eq!(
            r14.wrapping_add(4).wrapping_add(20),
            0x00C1FE34,
            "ret 0x14: +24"
        );

        // Edge: ret 0xFFFF (max imm16) — should still be correct
        let r14_low: u32 = 0x00010000;
        assert_eq!(
            r14_low.wrapping_add(4).wrapping_add(0xFFFF),
            0x00020003,
            "ret 0xFFFF: +65539"
        );

        // Edge: R14 near wrap — wrapping_add handles overflow
        let r14_high: u32 = 0xFFFFFFF0;
        assert_eq!(
            r14_high.wrapping_add(4).wrapping_add(12),
            0x00000000,
            "wrap around"
        );
    }

    /// Extreme test: verify the OOVPA HLE cleanup math matches ret N cleanup.
    /// OOVPA does: R14 += 4 + argc * 4
    /// VEH does:   R14 += 4 + ret_cleanup
    /// For stdcall: ret_cleanup == argc * 4
    /// These MUST match or the caller sees different R14 depending on whether
    /// the function was HLE'd or compiled.
    #[test]
    fn oovpa_vs_veh_cleanup_parity() {
        struct Case {
            name: &'static str,
            argc: u8,
            ret_cleanup: u16,
        }
        let cases = [
            Case {
                name: "SetVertexShaderConstant",
                argc: 3,
                ret_cleanup: 12,
            },
            Case {
                name: "SetPixelShaderConstant",
                argc: 3,
                ret_cleanup: 12,
            },
            Case {
                name: "CreateTexture",
                argc: 5,
                ret_cleanup: 20,
            },
            Case {
                name: "SetViewport",
                argc: 1,
                ret_cleanup: 4,
            },
            Case {
                name: "BlockUntilVerticalBlank",
                argc: 1,
                ret_cleanup: 4,
            },
            Case {
                name: "SetVertexShader",
                argc: 1,
                ret_cleanup: 4,
            },
            Case {
                name: "SetTransform",
                argc: 2,
                ret_cleanup: 8,
            },
            Case {
                name: "Clear",
                argc: 6,
                ret_cleanup: 24,
            },
            Case {
                name: "Swap",
                argc: 1,
                ret_cleanup: 4,
            },
        ];

        for c in &cases {
            let oovpa_delta = 4u32 + c.argc as u32 * 4; // OOVPA: R14 += 4 + argc*4
            let veh_delta = 4u32 + c.ret_cleanup as u32; // VEH:  R14 += 4 + cleanup
            assert_eq!(
                oovpa_delta, veh_delta,
                "{}: OOVPA delta {} != VEH delta {} (argc={}, ret_cleanup={})",
                c.name, oovpa_delta, veh_delta, c.argc, c.ret_cleanup
            );
        }
    }

    /// Extreme test: verify emit_call_rel writes the correct GUEST return address.
    /// The guest return address is the instruction AFTER the CALL.
    /// If this is wrong, the called function's ret returns to the wrong place.
    #[test]
    fn emit_call_rel_guest_return_addr() {
        // call at 0x1000 (5 bytes: E8 xx xx xx xx) → target 0x2000
        // guest return address should be 0x1005
        let base = 0x1000u32;
        let mut code = vec![0x90u8; 0x10];
        // 0x1000: call 0x2000 (rel32 = 0x2000 - 0x1005 = 0x0FFB)
        code[0] = 0xE8;
        let rel = 0x2000i32 - 0x1005i32;
        code[1..5].copy_from_slice(&rel.to_le_bytes());
        // 0x1005: nop (this is where ret should return to)
        code[5] = 0x90;

        let (instrs, _) = decoder::decode_section(&code, base);
        let call_instr = instrs
            .iter()
            .find(|i| i.instruction.mnemonic() == Mnemonic::Call);
        assert!(call_instr.is_some(), "should find CALL instruction");

        let call = call_instr.unwrap();
        assert_eq!(call.guest_addr, 0x1000, "call should be at 0x1000");
        // Guest return address = call_addr + call_length
        let guest_ret = call.guest_addr + call.length as u32;
        assert_eq!(
            guest_ret, 0x1005,
            "guest return addr should be 0x1005, got 0x{:08X}",
            guest_ret
        );

        // Now test emit_call_rel writes this value into [R15+R14]
        let mut emit_code = Vec::new();
        let mut fixups = Vec::new();
        emit_call_rel(&mut emit_code, guest_ret, &mut fixups, Some(0x2000));

        // The emitted code layout:
        //   [0-3]: LEA R14D,[R14-4] (45 8D 76 FC)
        //   [4-7]: MOV [R15+R14],  (43 C7 04 37) — opcode
        //   [8-11]: imm32          (guest return address)
        assert_eq!(
            &emit_code[0..4],
            &[0x45, 0x8D, 0x76, 0xFC],
            "CALL rewrite must use LEA to preserve EFLAGS"
        );
        let ret_in_code =
            u32::from_le_bytes([emit_code[8], emit_code[9], emit_code[10], emit_code[11]]);
        assert_eq!(
            ret_in_code, 0x1005,
            "emit_call_rel should embed guest ret addr 0x1005, got 0x{:08X}",
            ret_in_code
        );
    }

    /// Full emit test: verify the trap table entries have correct cleanup
    /// and that find_trap can locate every RET trap by host_offset.
    /// This tests the end-to-end path: decode → emit → trap table → lookup.
    #[test]
    fn full_emit_ret_cleanup_in_trap_table() {
        // Simple guest code: just ret (C3) and ret 0xC (C2 0C 00)
        // placed at different addresses to avoid any P0-zone special handling
        let base = 0x80000u32; // well outside any P0/quarantine zone
        let mut code = vec![0x90u8; 0x20]; // nops
                                           // 0x80000: nop (padding)
                                           // 0x80010: ret
        code[0x10] = 0xC3;
        // 0x80011: nop padding
        // 0x80018: ret 0xC
        code[0x18] = 0xC2;
        code[0x19] = 0x0C;
        code[0x1A] = 0x00;

        let (instrs, _) = decoder::decode_section(&code, base);

        // Need a GuestMemory for emit_section — create one
        let memory = match crate::xbox::memory::guest_memory::GuestMemory::new() {
            Ok(m) => m,
            Err(_) => {
                eprintln!("SKIP: GuestMemory::new() failed (needs VirtualAlloc2)");
                return;
            }
        };

        let block = emit_section(&instrs, &code, base, &memory, 10, &[]);

        // Find RET traps
        let ret_traps: Vec<_> = block
            .traps
            .iter()
            .filter(|t| matches!(t.trap_type, TrapType::Ret | TrapType::RetMiss))
            .collect();

        println!("RET traps found: {}", ret_traps.len());
        for t in &ret_traps {
            println!(
                "  guest=0x{:08X} host_off=0x{:X} cleanup={}",
                t.guest_addr, t.host_offset, t.ret_cleanup
            );
        }

        // Verify bare ret has cleanup=0
        let bare = ret_traps.iter().find(|t| t.guest_addr == 0x80010);
        assert!(bare.is_some(), "should find trap for bare ret at 0x80010");
        assert_eq!(bare.unwrap().ret_cleanup, 0);

        // Verify stdcall ret trap exists
        let stdcall = ret_traps.iter().find(|t| t.guest_addr == 0x80018);
        assert!(stdcall.is_some(), "should find trap for ret 0xC at 0x80018");
        // RetMiss: cleanup=0 (inline code already applied 4+12)
        // Ret: cleanup=12 (VEH must apply)
        if stdcall.unwrap().trap_type == TrapType::RetMiss {
            assert_eq!(
                stdcall.unwrap().ret_cleanup,
                0,
                "RetMiss cleanup should be 0"
            );
        } else {
            assert_eq!(stdcall.unwrap().ret_cleanup, 12, "Ret cleanup should be 12");
        }

        // Verify find_trap works for both (simulates VEH lookup)
        // Build a minimal RuntimeContext-like trap table for binary search
        let sorted_traps = &block.traps;
        for trap in sorted_traps {
            if matches!(trap.trap_type, TrapType::Ret | TrapType::RetMiss) {
                let found = sorted_traps.binary_search_by_key(&trap.host_offset, |t| t.host_offset);
                assert!(
                    found.is_ok(),
                    "find_trap should find RET at host_off=0x{:X} guest=0x{:08X}",
                    trap.host_offset,
                    trap.guest_addr
                );
                let idx = found.unwrap();
                assert_eq!(
                    sorted_traps[idx].ret_cleanup, trap.ret_cleanup,
                    "found trap cleanup should match: expected {} got {}",
                    trap.ret_cleanup, sorted_traps[idx].ret_cleanup
                );
            }
        }
    }

    /// Test that RET trap lookup still works when emitting in the old P0 zone.
    /// StackTrace probes are now only injected at targeted CALL_BOUNDARY sites,
    /// so generic P0-zone code should not assume any extra probes.
    #[test]
    fn p0_zone_ret_with_stacktrace_probes() {
        // Place a ret 0xC inside the P0 zone
        let base = 0x0029_C800u32;
        let mut code = vec![0x90u8; 0x20];
        // 0x0029C800: push 1 (6A 01)
        code[0] = 0x6A;
        code[1] = 0x01;
        // 0x0029C802: push 2 (6A 02)
        code[2] = 0x6A;
        code[3] = 0x02;
        // 0x0029C804: ret 0x08 (C2 08 00) — stdcall, 2 args
        code[4] = 0xC2;
        code[5] = 0x08;
        code[6] = 0x00;
        // 0x0029C807: ret (C3) — bare ret
        code[7] = 0xC3;

        let (instrs, _) = decoder::decode_section(&code, base);

        let memory = match crate::xbox::memory::guest_memory::GuestMemory::new() {
            Ok(m) => m,
            Err(_) => {
                eprintln!("SKIP: GuestMemory::new() failed");
                return;
            }
        };

        let block = emit_section(&instrs, &code, base, &memory, 10, &[]);

        // CALL_BOUNDARY probes are now address-specific. This synthetic P0 snippet
        // should still emit correct RET traps even without any StackTrace probes.

        let ret_traps: Vec<_> = block
            .traps
            .iter()
            .filter(|t| matches!(t.trap_type, TrapType::Ret | TrapType::RetMiss))
            .collect();
        let trace_traps: Vec<_> = block
            .traps
            .iter()
            .filter(|t| t.trap_type == TrapType::StackTrace)
            .collect();

        println!(
            "In P0 zone: {} RET traps, {} StackTrace probes",
            ret_traps.len(),
            trace_traps.len()
        );

        assert_eq!(
            trace_traps.len(),
            0,
            "generic P0-zone snippet should not get CALL_BOUNDARY probes"
        );

        // Verify ret 0x08 trap exists at guest 0x0029C804
        let stdcall = ret_traps.iter().find(|t| t.guest_addr == 0x0029_C804);
        assert!(stdcall.is_some(), "should find ret 0x08 trap");
        // RetMiss: cleanup=0 because inline code already applied it
        // Ret: cleanup=8 because VEH needs to apply it
        if stdcall.unwrap().trap_type == TrapType::RetMiss {
            assert_eq!(
                stdcall.unwrap().ret_cleanup,
                0,
                "RetMiss should have cleanup=0 (already applied)"
            );
        } else {
            assert_eq!(stdcall.unwrap().ret_cleanup, 8, "Ret should have cleanup=8");
        }

        // Verify bare ret trap exists at guest 0x0029C807
        let bare = ret_traps.iter().find(|t| t.guest_addr == 0x0029_C807);
        assert!(bare.is_some(), "should find bare ret trap");
        assert_eq!(bare.unwrap().ret_cleanup, 0);

        // Verify binary search finds both despite interleaved StackTrace traps
        for trap in &block.traps {
            let found = block
                .traps
                .binary_search_by_key(&trap.host_offset, |t| t.host_offset);
            assert!(
                found.is_ok(),
                "should find trap type={:?} at host_off=0x{:X} guest=0x{:08X}",
                trap.trap_type,
                trap.host_offset,
                trap.guest_addr
            );
        }
    }

    /// Verify `emit_leave` produces the correct 3-step x64 emission for guest LEAVE (0xC9).
    ///
    /// Guest LEAVE semantics: ESP <- EBP; EBP <- pop (i.e. [ESP], then ESP+=4).
    /// In our register contract R14 = guest ESP, R15 = guest memory base, RBP = guest EBP.
    /// Correct emission must therefore be:
    ///   1. mov r14d, ebp          (ESP <- EBP) — zero-extends into R14
    ///   2. mov ebp, [r15+r14]     (EBP <- [ESP])
    ///   3. lea r14d, [r14+4]      (ESP += 4) — zero-extends into R14
    /// Total: 11 bytes.
    #[test]
    fn test_emit_leave_bytes() {
        // Decode a single LEAVE (0xC9) so this test documents usage even though
        // emit_leave itself doesn't consume the DecodedInstr.
        let (instrs, _) = decoder::decode_section(&[0xC9u8], 0x1000);
        assert_eq!(instrs.len(), 1, "should decode 1 instruction");
        assert_eq!(
            instrs[0].instruction.mnemonic(),
            Mnemonic::Leave,
            "decoded mnemonic should be LEAVE, got {:?}",
            instrs[0].instruction.mnemonic()
        );
        assert_eq!(instrs[0].length, 1, "LEAVE is 1 byte");

        let mut code = Vec::new();
        // emit_leave takes (code, di, before_len) for diagnostic logging; before_len=0
        // just means "this is the first thing in the emitted buffer".
        emit_leave(&mut code, &instrs[0], 0);

        // Expected correct emission (11 bytes total):
        //   41 89 EE           mov r14d, ebp       (MOV r/m32,r32 form with REX.B)
        //   43 8B 2C 37        mov ebp, [r15+r14]  (SIB: base=r15, index=r14, REX.X+REX.B)
        //   45 8D 76 04        lea r14d, [r14+4]   (REX.R+REX.B for r14 as both dest & base)
        let expected: Vec<u8> = vec![
            0x41, 0x89, 0xEE, 0x43, 0x8B, 0x2C, 0x37, 0x45, 0x8D, 0x76, 0x04,
        ];

        assert_eq!(
            code.len(),
            expected.len(),
            "emit_leave should emit {} bytes, got {}: {:02X?}",
            expected.len(),
            code.len(),
            code
        );
        assert_eq!(
            code, expected,
            "emit_leave byte mismatch:\nexpected: {:02X?}\n   actual: {:02X?}",
            expected, code
        );

        // Round-trip the emitted host bytes through iced-x86 in 64-bit mode to
        // catch cases where the bytes assemble but mean something else.
        let mut host_decoder = iced_x86::Decoder::new(64, &code, iced_x86::DecoderOptions::NONE);
        let i1 = host_decoder.decode();
        let i2 = host_decoder.decode();
        let i3 = host_decoder.decode();
        assert_eq!(
            i1.mnemonic(),
            Mnemonic::Mov,
            "1st emitted instr should be MOV"
        );
        assert_eq!(
            i2.mnemonic(),
            Mnemonic::Mov,
            "2nd emitted instr should be MOV (memory load)"
        );
        assert_eq!(
            i3.mnemonic(),
            Mnemonic::Lea,
            "3rd emitted instr should be LEA"
        );
        assert_eq!(
            i1.op0_register(),
            Register::R14D,
            "1st MOV dest must be R14D (guest ESP low)"
        );
        assert_eq!(
            i1.op1_register(),
            Register::EBP,
            "1st MOV src must be EBP (guest EBP)"
        );
        assert_eq!(i2.op0_register(), Register::EBP, "2nd MOV dest must be EBP");
        assert_eq!(
            i2.memory_base(),
            Register::R15,
            "2nd MOV base must be R15 (guest mem)"
        );
        assert_eq!(
            i2.memory_index(),
            Register::R14,
            "2nd MOV index must be R14 (guest ESP)"
        );
        assert_eq!(
            i3.op0_register(),
            Register::R14D,
            "3rd LEA dest must be R14D"
        );
        assert_eq!(i3.memory_base(), Register::R14, "3rd LEA base must be R14");
        assert_eq!(
            i3.memory_displacement64(),
            4,
            "3rd LEA disp must be +4 (ESP += 4)"
        );
    }

    /// Ground-truth encoding of the 3-step LEAVE equivalent via the iced-x86 Encoder.
    ///
    /// Feeds three `Instruction`s (mov r14d,ebp ; mov ebp,[r15+r14] ; lea r14d,[r14+4])
    /// into the encoder and prints + asserts the resulting bytes. This is the
    /// authoritative reference the hand-coded `emit_leave` table should match.
    #[test]
    fn ground_truth_leave_encoding() {
        use iced_x86::{Code, Encoder, Instruction, MemoryOperand, Register};

        // Step 1: mov r14d, ebp  (MOV r/m32, r32 form — REX.B for r14d)
        let i1 = Instruction::with2::<Register, Register>(
            Code::Mov_rm32_r32,
            Register::R14D,
            Register::EBP,
        )
        .unwrap();

        // Step 2: mov ebp, dword ptr [r15+r14]  (SIB base=r15, index=r14, scale=1)
        let i2 = Instruction::with2::<Register, MemoryOperand>(
            Code::Mov_r32_rm32,
            Register::EBP,
            MemoryOperand::with_base_index(Register::R15, Register::R14),
        )
        .unwrap();

        // Step 3: lea r14d, [r14+4]
        let i3 = Instruction::with2::<Register, MemoryOperand>(
            Code::Lea_r32_m,
            Register::R14D,
            MemoryOperand::with_base_displ(Register::R14, 4),
        )
        .unwrap();

        let mut encoder = Encoder::new(64);
        // `rip` is the target instruction address; use 0 because we want pure
        // encoding (no RIP-relative operands here).
        encoder.encode(&i1, 0).expect("encode i1");
        encoder.encode(&i2, 0).expect("encode i2");
        encoder.encode(&i3, 0).expect("encode i3");

        let bytes = encoder.take_buffer();
        println!("LEAVE equivalent: {:02X?}", bytes);
        println!("LEAVE equivalent byte count: {}", bytes.len());

        // Known-good hand-coded sequence (what emit_leave emits).
        let expected: Vec<u8> = vec![
            0x41, 0x89, 0xEE, // mov r14d, ebp
            0x43, 0x8B, 0x2C, 0x37, // mov ebp, [r15+r14]
            0x45, 0x8D, 0x76, 0x04, // lea r14d, [r14+4]
        ];

        assert_eq!(
            bytes.len(),
            11,
            "iced-x86 ground truth should be 11 bytes, got {}",
            bytes.len()
        );
        assert_eq!(
            bytes, expected,
            "iced-x86 ground truth mismatch:\nexpected: {:02X?}\n  encoder: {:02X?}",
            expected, bytes
        );

        // Cross-check: emit_leave output == iced-x86 ground truth.
        let (instrs, _) = decoder::decode_section(&[0xC9u8], 0x1000);
        let mut hand = Vec::new();
        emit_leave(&mut hand, &instrs[0], 0);
        assert_eq!(
            hand, bytes,
            "emit_leave != iced-x86 ground truth:\nemit_leave: {:02X?}\n  ground : {:02X?}",
            hand, bytes
        );
    }
}
