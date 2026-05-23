/// VEH Fixup sub-handler — R15 address fixups [L2-TAG]
/// Handles: sign-extension, overflow, string instructions, 64MB RAM wrap.
///
/// Strategy (matching C++ AOT_VEH_Fixup.cpp):
///   Instead of decoding + emulating the faulting instruction, we temporarily
///   shift R15 by ±4GB and set the Trap Flag. The CPU re-executes the original
///   instruction with the corrected R15. On the single-step trap, we restore R15.
///   This handles ALL instruction types (MOV, PUSH, POP, LEA, REP STOSD, etc.)
///   without needing per-instruction decode logic.

#[cfg(windows)]
use super::veh::{veh_log, VehResult};

#[cfg(windows)]
use std::collections::HashMap;
#[cfg(windows)]
use std::sync::{Mutex, OnceLock};

#[cfg(windows)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct R15FixupSite {
    host_rip: u64,
    host_offset: u32,
    guest_pc: u32,
    opcode_bytes: [u8; 16],
    delta_kind: i8,
}

#[cfg(windows)]
#[derive(Clone, Copy, Default)]
struct R15FixupStats {
    count: u64,
    last_fault_addr: u64,
    last_guest_addr: u32,
    last_r14: u32,
    last_rax: u32,
    last_rbx: u32,
    last_rcx: u32,
    last_rdx: u32,
    last_rsi: u32,
    last_rdi: u32,
}

#[cfg(windows)]
static R15_FIXUP_PROFILE: OnceLock<Mutex<HashMap<R15FixupSite, R15FixupStats>>> = OnceLock::new();
#[cfg(windows)]
static R15_FIXUP_PROFILE_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(windows)]
fn r15_fixup_profile_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("RUSTEMU_R15_FIXUP_PROF")
            .map(|value| {
                let value = value.trim();
                value == "1"
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
                    || value.eq_ignore_ascii_case("on")
            })
            .unwrap_or(false)
    })
}

#[cfg(windows)]
pub fn r15_fixup_profile_active() -> bool {
    r15_fixup_profile_enabled()
}

#[cfg(windows)]
fn r15_fixup_profile_map() -> &'static Mutex<HashMap<R15FixupSite, R15FixupStats>> {
    R15_FIXUP_PROFILE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(windows)]
fn decode_profile_bytes(site: &R15FixupSite) -> String {
    let mut dec = iced_x86::Decoder::with_ip(
        64,
        &site.opcode_bytes,
        site.host_rip,
        iced_x86::DecoderOptions::NONE,
    );
    if !dec.can_decode() {
        return "??".to_string();
    }
    let instr = dec.decode();
    let mut formatter = iced_x86::IntelFormatter::new();
    let mut output = String::new();
    iced_x86::Formatter::format(&mut formatter, &instr, &mut output);
    output
}

#[cfg(windows)]
pub fn reset_r15_fixup_profile() {
    if let Some(map) = R15_FIXUP_PROFILE.get() {
        if let Ok(mut map) = map.lock() {
            map.clear();
        }
    }
    R15_FIXUP_PROFILE_COUNT.store(0, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(windows)]
pub fn dump_r15_fixup_profile(reason: &str) {
    if !r15_fixup_profile_enabled() {
        return;
    }
    let rows = {
        let Ok(map) = r15_fixup_profile_map().lock() else {
            veh_log("[R15-FIXUP-PROF] unable to lock histogram");
            return;
        };
        let mut rows: Vec<(R15FixupSite, R15FixupStats)> =
            map.iter().map(|(site, stats)| (*site, *stats)).collect();
        rows.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        rows
    };
    let total = R15_FIXUP_PROFILE_COUNT.load(std::sync::atomic::Ordering::Relaxed);
    veh_log(&format!(
        "[R15-FIXUP-PROF] reason={} total={} unique={}",
        reason,
        total,
        rows.len()
    ));
    for (rank, (site, stats)) in rows.iter().take(16).enumerate() {
        let bytes = site
            .opcode_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        veh_log(&format!(
            "[R15-FIXUP-PROF] #{:02} hits={} guest_pc=0x{:08X} host_off=0x{:X} rip=0x{:016X} kind={} guest=0x{:08X} fault=0x{:016X} instr='{}' bytes=[{}] r14=0x{:08X} eax=0x{:08X} ebx=0x{:08X} ecx=0x{:08X} edx=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
            rank + 1,
            stats.count,
            site.guest_pc,
            site.host_offset,
            site.host_rip,
            if site.delta_kind > 0 { "signext+4g" } else { "overflow-4g" },
            stats.last_guest_addr,
            stats.last_fault_addr,
            decode_profile_bytes(site),
            bytes,
            stats.last_r14,
            stats.last_rax,
            stats.last_rbx,
            stats.last_rcx,
            stats.last_rdx,
            stats.last_rsi,
            stats.last_rdi
        ));
    }
}

#[cfg(windows)]
fn record_r15_fixup_profile(
    host_rip: u64,
    host_offset: u32,
    guest_pc: u32,
    fault_addr: u64,
    guest_addr: u32,
    delta: i64,
    context: &windows::Win32::System::Diagnostics::Debug::CONTEXT,
) {
    if !r15_fixup_profile_enabled() {
        return;
    }
    let mut opcode_bytes = [0u8; 16];
    unsafe {
        std::ptr::copy_nonoverlapping(host_rip as *const u8, opcode_bytes.as_mut_ptr(), 16);
    }
    let site = R15FixupSite {
        host_rip,
        host_offset,
        guest_pc,
        opcode_bytes,
        delta_kind: if delta >= 0 { 1 } else { -1 },
    };
    let mut should_dump = false;
    let total = R15_FIXUP_PROFILE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    if let Ok(mut map) = r15_fixup_profile_map().lock() {
        let stats = map.entry(site).or_default();
        stats.count = stats.count.saturating_add(1);
        stats.last_fault_addr = fault_addr;
        stats.last_guest_addr = guest_addr;
        stats.last_r14 = context.R14 as u32;
        stats.last_rax = context.Rax as u32;
        stats.last_rbx = context.Rbx as u32;
        stats.last_rcx = context.Rcx as u32;
        stats.last_rdx = context.Rdx as u32;
        stats.last_rsi = context.Rsi as u32;
        stats.last_rdi = context.Rdi as u32;
        should_dump = total <= 16 || total == 100 || total == 1_000 || total % 5_000 == 0;
    }
    if should_dump {
        dump_r15_fixup_profile("periodic");
    }
}

/// Per-thread flag: true while R15 is temporarily shifted for a fixup.
/// Prevents DPC injection and nested fixups while R15 is wrong.
#[cfg(windows)]
thread_local! {
    pub(crate) static R15_FIXUP_PENDING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The original R15 value to restore on single-step.
    pub(crate) static R15_ORIGINAL: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    /// Consecutive null-deref counter. Reset on non-null-deref VEH events.
    pub(crate) static NULL_DEREF_STREAK: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Reset the null-deref streak counter (call from non-null-deref VEH paths).
#[cfg(windows)]
pub fn reset_null_deref_streak() {
    NULL_DEREF_STREAK.with(|c| c.set(0));
}

/// Handle an access violation that may need an R15 fixup.
/// Called when fault_addr is outside the 4GB guest address space relative to R15.
///
/// Returns VehResult::Handled if the fixup was applied (instruction will re-execute),
/// or VehResult::NotHandled if this isn't a fixup case.
#[cfg(windows)]
pub fn handle_r15_fixup(
    fault_addr: u64,
    r15_base: u64,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    host_offset: u32,
    guest_pc: u32,
    sign_ext_fixups: &mut u64,
) -> VehResult {
    let offset64 = fault_addr.wrapping_sub(r15_base);

    // Check if the fault is outside the 4GB guest address space
    if offset64 <= 0xFFFF_FFFF {
        return VehResult::NotHandled; // Normal fault within guest space
    }

    // Xbox has NO null page guard — address 0 is the start of 64MB RAM (TEB zero page).
    // Guest code accessing address 0 (NULL pointer) reads/writes real mapped memory.
    // The sign-extension fixup applies normally: shift R15 by ±4GB, re-execute, restore.
    // This matches real Xbox hardware where NULL dereferences don't fault.
    let guest_addr_approx = offset64 as u32;
    if guest_addr_approx < 0x1000 {
        crate::rate_log!(
            50,
            "[L2-TAG] LOW-ADDR fixup: guest=0x{:08X} fault=0x{:016X} (Xbox zero-page is mapped)",
            guest_addr_approx,
            fault_addr
        );

        // Sprint 8 fix (audit-2026-04-23.md P1.6 — Zero page DEADBEEF):
        // Guest writes `mov dword [r15-0x80000000], 0xDEADBEEFh` which wraps
        // to guest address 0 and corrupts NT_TIB (the zero page holds TEB,
        // StackBase, StackLimit, Self pointers). Subsequent SEH chain walks
        // then find a bogus handler at address 0xDEADBEEF → fatal crash.
        // DEADBEEF is a canonical poison sentinel; no legitimate title writes
        // exactly that value to exactly address 0. Block the write and skip
        // the instruction. Scope is narrow: guest_addr == 0, R15-based write,
        // imm32 == 0xDEADBEEF.
        if guest_addr_approx == 0 {
            let rip = context.Rip as u64;
            let bytes = unsafe { std::slice::from_raw_parts(rip as *const u8, 16) };
            let mut dec =
                iced_x86::Decoder::with_ip(64, bytes, rip, iced_x86::DecoderOptions::NONE);
            if dec.can_decode() {
                let instr = dec.decode();
                let is_r15_base = instr.memory_base() == iced_x86::Register::R15;
                let is_mov_imm = instr.mnemonic() == iced_x86::Mnemonic::Mov
                    && instr.op1_kind() == iced_x86::OpKind::Immediate32;
                if is_r15_base && is_mov_imm && instr.immediate32() == 0xDEADBEEF {
                    static BLOCKED: std::sync::atomic::AtomicU64 =
                        std::sync::atomic::AtomicU64::new(0);
                    let n = BLOCKED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if n <= 5 || n.is_power_of_two() {
                        veh_log(&format!(
                            "[DEADBEEF-GUARD] blocked zero-page poison write #{} RIP=0x{:016X} disp=0x{:X}",
                            n, rip, instr.memory_displacement64()
                        ));
                    }
                    // Skip the faulting instruction; NT_TIB remains intact.
                    context.Rip = rip.wrapping_add(instr.len() as u64);
                    return VehResult::Handled;
                }
            }
        }
        // Fall through to normal R15 fixup below — Xbox has no null page guard
    }

    // Guard: high-negative addresses (>= 0xF0000000) are real guest AVs,
    // NOT sign-extension errors. These come from null pointer dereferences
    // with negative offsets (e.g. mov eax,[esi-8] where ESI=0 → 0xFFFFFFF8).
    // Without this guard, the fixup loops infinitely (add +4GB, still wraps).
    if guest_addr_approx >= 0xF000_0000
        && !(guest_addr_approx >= 0xFD00_0000 && guest_addr_approx < 0xFD90_0000)  // NV2A MMIO OK
        && !(guest_addr_approx >= 0xFE80_0000 && guest_addr_approx < 0xFED0_0000)
    // APU/AC97 OK
    {
        crate::rate_log!(
            10,
            "[L2-TAG] R15-fixup SKIP: guest=0x{:08X} high-negative (real AV), fault=0x{:016X}",
            guest_addr_approx,
            fault_addr
        );
        return VehResult::NotHandled;
    }

    // Only apply R15 fixup if the faulting instruction actually uses R15 as base/index.
    // Shadow stack (R12), host stack (RSP), or other non-R15 accesses should NOT be fixed.
    {
        let rip = context.Rip as u64;
        let bytes = unsafe { std::slice::from_raw_parts(rip as *const u8, 16) };
        let mut dec = iced_x86::Decoder::with_ip(64, bytes, rip, iced_x86::DecoderOptions::NONE);
        if dec.can_decode() {
            let instr = dec.decode();
            let base = instr.memory_base();
            let index = instr.memory_index();
            if base != iced_x86::Register::R15 && index != iced_x86::Register::R15 {
                // Not an R15-based access — don't apply fixup
                return VehResult::NotHandled;
            }
            // One-shot decode log for the first fixup at each unique RIP
            static DECODED_RIP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let prev = DECODED_RIP.load(std::sync::atomic::Ordering::Relaxed);
            if prev != rip {
                DECODED_RIP.store(rip, std::sync::atomic::Ordering::Relaxed);
                let mut formatter = iced_x86::IntelFormatter::new();
                let mut output = String::new();
                iced_x86::Formatter::format(&mut formatter, &instr, &mut output);
                veh_log(&format!(
                    "[R15-DECODE] RIP=0x{:016X} instr='{}' base={:?} idx={:?} disp=0x{:X} bytes={:02X?} RAX=0x{:X} RBX=0x{:X} RCX=0x{:X} RDX=0x{:X} RSI=0x{:X} RDI=0x{:X}",
                    rip, output, base, index, instr.memory_displacement64(),
                    &bytes[..instr.len()],
                    context.Rax, context.Rbx, context.Rcx, context.Rdx, context.Rsi, context.Rdi
                ));
            }
        }
    }

    // Safety: don't nest fixups
    let already_pending = R15_FIXUP_PENDING.with(|f| f.get());
    if already_pending {
        // Fixup during fixup — safety net: restore R15 and bail
        let orig = R15_ORIGINAL.with(|r| r.get());
        context.R15 = orig;
        context.EFlags &= !0x100; // Clear trap flag
        R15_FIXUP_PENDING.with(|f| f.set(false));
        veh_log("[L2-TAG] SAFETY-NET: AV during pending fixup, R15 restored");
        return VehResult::NotHandled; // Let crash handler deal with it
    }

    // Determine the delta to apply to R15:
    // Sign-extension case: guest_addr >= 0x80000000 (negative disp32) → R15 needs +4GB
    // Overflow case: guest_addr < 0x80000000 (positive disp overflowed) → R15 needs -4GB
    // Using guest_addr instead of fault_addr comparison because when
    // fault_addr wraps below zero to a very large u64 (e.g. base - 0x80000000 wraps to
    // 0xFFFFFFFE7FFF0000), the unsigned comparison `fault_addr < r15_base` gives the
    // wrong answer.
    let delta: i64 = if guest_addr_approx >= 0x8000_0000 {
        0x1_0000_0000i64 // +4GB
    } else {
        -0x1_0000_0000i64 // -4GB
    };

    let guest_addr = offset64 as u32;
    record_r15_fixup_profile(
        context.Rip,
        host_offset,
        guest_pc,
        fault_addr,
        guest_addr,
        delta,
        context,
    );

    if *sign_ext_fixups < 10 {
        veh_log(&format!(
            "[L2-TAG] R15-fixup: fault=0x{:016X} guest=0x{:08X} delta={:+} RIP=0x{:016X}",
            fault_addr, guest_addr, delta, context.Rip
        ));
    }

    // Save original R15 and apply the shift
    R15_ORIGINAL.with(|r| r.set(r15_base));
    R15_FIXUP_PENDING.with(|f| f.set(true));
    crate::xbox::aot::bink_profile::note_av_fixup(guest_addr);

    context.R15 = (r15_base as i64 + delta) as u64;
    context.EFlags |= 0x100; // Set Trap Flag → single-step after re-execution

    *sign_ext_fixups += 1;

    // .text zeroing detector
    if *sign_ext_fixups % 200 == 0 {
        let text_p = unsafe { *((r15_base as usize + 0x11000) as *const u32) };
        if text_p == 0 {
            veh_log(&format!(
                "[TEXT-ZERO-FIXUP] .text zeroed at fixup #{} guest=0x{:08X} fault=0x{:016X} RIP=0x{:016X}",
                *sign_ext_fixups, guest_addr_approx, fault_addr, context.Rip
            ));
        }
    }

    // Periodic RIP/guest sampling to detect where the worker is stuck
    if *sign_ext_fixups % 500_000 == 0 {
        veh_log(&format!(
            "[R15-SAMPLE] fixups={} guest=0x{:08X} RIP=0x{:016X} R14(esp)=0x{:08X}",
            *sign_ext_fixups, guest_addr, context.Rip, context.R14 as u32
        ));
    }

    VehResult::Handled
}

/// Handle a single-step trap that follows an R15 fixup.
/// Restores R15 to its original value and clears the trap flag.
///
/// Returns VehResult::Handled if this was our single-step (R15 restored),
/// or VehResult::NotHandled if no fixup was pending.
#[cfg(windows)]
pub fn handle_single_step(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> VehResult {
    let pending = R15_FIXUP_PENDING.with(|f| f.get());
    if !pending {
        return VehResult::NotHandled;
    }

    // Restore R15 to original guest memory base
    let orig = R15_ORIGINAL.with(|r| r.get());
    context.R15 = orig;
    context.EFlags &= !0x100; // Clear Trap Flag
    R15_FIXUP_PENDING.with(|f| f.set(false));

    VehResult::Handled
}

/// Check if an R15 fixup is currently pending (R15 is temporarily shifted).
#[cfg(windows)]
pub fn is_fixup_pending() -> bool {
    R15_FIXUP_PENDING.with(|f| f.get())
}

/// Clear a pending R15 fixup and restore R15 to the saved original value.
/// Used when a non-single-step code path (e.g. kernel exit diverting RIP
/// to the dispatcher) abandons the would-be re-execution, so the TF
/// single-step trap will never arrive to run `handle_single_step`.
/// Safe to call when no fixup is pending: returns false, does nothing.
/// When true is returned, context.R15 has been restored to the pre-shift
/// value (the guest memory base).
#[cfg(windows)]
pub fn clear_pending_fixup(
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> bool {
    if !R15_FIXUP_PENDING.with(|f| f.get()) {
        return false;
    }
    // Mirror handle_single_step's full state reset.
    let orig = R15_ORIGINAL.with(|r| r.get());
    context.R15 = orig;
    context.EFlags &= !0x100; // Clear Trap Flag (was set by handle_r15_fixup)
    R15_FIXUP_PENDING.with(|f| f.set(false));
    R15_ORIGINAL.with(|r| r.set(0));
    true
}
