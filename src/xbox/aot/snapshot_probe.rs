//! Env-gated, PC-gated snapshot probe.
//!
//! Emits .snap files in the recomp-spiderman binary format (92-byte header
//! + regions) when the interpreter enters and exits a target function PC.
//!
//! Inactive unless `RUSTEMU_SNAPSHOT_PC` is set. Costs one env-var lookup at
//! interpreter startup and one cheap PC compare per `interpret()` call when
//! configured.
//!
//! Configuration via environment:
//!   RUSTEMU_SNAPSHOT_PC        target function entry VA, hex with 0x prefix
//!                              (e.g. 0x002EC340).
//!   RUSTEMU_SNAPSHOT_DIR       output dir (default: ./snapshots).
//!   RUSTEMU_SNAPSHOT_REGIONS   space-separated VA:SIZE pairs to capture, e.g.
//!                                "0x3038E0:4 0x800000:8 0x900000:256"
//!                              Sizes are hex if 0x-prefixed, else decimal.

use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::xbox::aot::micro_interp::X86Regs;
use crate::xbox::memory::guest_memory::GuestMemory;

struct Probe {
    target_pc: u32,
    out_dir: PathBuf,
    regions: Vec<(u32, u32)>,
    active: bool,
    seq: u32,
}

thread_local! {
    static PROBE: RefCell<Option<Probe>> = RefCell::new(Probe::from_env());
}

fn parse_u32(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(h, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

impl Probe {
    fn from_env() -> Option<Self> {
        let pc = parse_u32(&std::env::var("RUSTEMU_SNAPSHOT_PC").ok()?)?;
        let out_dir = std::env::var("RUSTEMU_SNAPSHOT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("snapshots"));

        let mut regions = Vec::new();
        if let Ok(rstr) = std::env::var("RUSTEMU_SNAPSHOT_REGIONS") {
            for tok in rstr.split_whitespace() {
                if let Some((va, sz)) = tok.split_once(':') {
                    if let (Some(va), Some(sz)) = (parse_u32(va), parse_u32(sz)) {
                        regions.push((va, sz));
                    }
                }
            }
        }

        eprintln!(
            "[snapshot_probe] armed for PC=0x{:08X}  out={:?}  regions={}",
            pc,
            out_dir,
            regions.len(),
        );
        Some(Self {
            target_pc: pc,
            out_dir,
            regions,
            active: false,
            seq: 0,
        })
    }
}

/// Called at the top of `interpret()` after `regs.eip = entry`.
pub fn on_entry(regs: &X86Regs, mem: &GuestMemory, entry_pc: u32) {
    PROBE.with(|cell| {
        let mut p_opt = cell.borrow_mut();
        let Some(p) = p_opt.as_mut() else {
            return;
        };
        if p.active || p.target_pc != entry_pc {
            return;
        }
        if let Err(e) = std::fs::create_dir_all(&p.out_dir) {
            eprintln!("[snapshot_probe] cannot create {:?}: {}", p.out_dir, e);
            return;
        }
        let path = p
            .out_dir
            .join(format!("rustemu_pc_{:08X}_pre_{:03}.snap", entry_pc, p.seq));
        match write_snapshot(&path, regs, mem, &p.regions, 0, 0) {
            Ok(n) => eprintln!(
                "[snapshot_probe] PRE  PC=0x{:08X} -> {} ({} bytes)",
                entry_pc,
                path.display(),
                n
            ),
            Err(e) => eprintln!("[snapshot_probe] PRE write failed: {}", e),
        }
        p.active = true;
    });
}

/// Called before each `return InterpResult::...` in `interpret()`. `trapped`
/// is 1 when the return path is abnormal (Timeout, Unhandled, AccessViolation,
/// KernelCall, OovpaHook), 0 for a clean ReturnedTo.
pub fn on_exit(regs: &X86Regs, mem: &GuestMemory, trapped: u32, trap_pc: u32) {
    PROBE.with(|cell| {
        let mut p_opt = cell.borrow_mut();
        let Some(p) = p_opt.as_mut() else {
            return;
        };
        if !p.active {
            return;
        }
        let path = p.out_dir.join(format!(
            "rustemu_pc_{:08X}_post_{:03}.snap",
            p.target_pc, p.seq
        ));
        match write_snapshot(&path, regs, mem, &p.regions, trapped, trap_pc) {
            Ok(n) => eprintln!(
                "[snapshot_probe] POST PC=0x{:08X} -> {} ({} bytes, trapped={})",
                p.target_pc,
                path.display(),
                n,
                trapped,
            ),
            Err(e) => eprintln!("[snapshot_probe] POST write failed: {}", e),
        }
        p.active = false;
        p.seq += 1;
    });
}

/// Write a snapshot. Returns total bytes written.
fn write_snapshot(
    path: &std::path::Path,
    regs: &X86Regs,
    mem: &GuestMemory,
    regions: &[(u32, u32)],
    trapped: u32,
    trap_pc: u32,
) -> std::io::Result<usize> {
    let mut f = File::create(path)?;
    let mut written = 0usize;

    // Header: 92 bytes, layout exactly matches runtime/snapshot.h SnapshotHeader.
    let mut hdr = [0u8; 92];
    hdr[0..7].copy_from_slice(b"RCMPSPM");
    hdr[7] = 0;

    let put = |buf: &mut [u8], off: usize, v: u32| {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    };

    put(&mut hdr, 8, 1); // version
    put(&mut hdr, 12, regs.eip); // pc
    put(&mut hdr, 16, regs.eax);
    put(&mut hdr, 20, regs.ebx);
    put(&mut hdr, 24, regs.ecx);
    put(&mut hdr, 28, regs.edx);
    put(&mut hdr, 32, regs.esi);
    put(&mut hdr, 36, regs.edi);
    put(&mut hdr, 40, regs.esp);
    put(&mut hdr, 44, regs.ebp);

    // Flags block: 8 bytes (CF, ZF, SF, OF, PF, AF, DF, pad). x86 EFLAGS bits:
    // CF=0, PF=2, AF=4, ZF=6, SF=7, DF=10, OF=11.
    let ef = regs.eflags;
    hdr[48] = ((ef >> 0) & 1) as u8; // CF
    hdr[49] = ((ef >> 6) & 1) as u8; // ZF
    hdr[50] = ((ef >> 7) & 1) as u8; // SF
    hdr[51] = ((ef >> 11) & 1) as u8; // OF
    hdr[52] = ((ef >> 2) & 1) as u8; // PF
    hdr[53] = ((ef >> 4) & 1) as u8; // AF
    hdr[54] = ((ef >> 10) & 1) as u8; // DF
    hdr[55] = 0;

    put(&mut hdr, 56, regions.len() as u32); // num_regions
    put(&mut hdr, 60, 0); // num_calls (rustemu doesn't track these)
    put(&mut hdr, 64, trapped);
    put(&mut hdr, 68, trap_pc);
    // 72..92: 5 x u32 reserved (already zero from init)

    f.write_all(&hdr)?;
    written += 92;

    for (base, size) in regions {
        f.write_all(&base.to_le_bytes())?;
        f.write_all(&size.to_le_bytes())?;
        written += 8;
        let mut buf = vec![0u8; *size as usize];
        for i in 0..*size {
            buf[i as usize] = mem.read_u8(base.wrapping_add(i));
        }
        f.write_all(&buf)?;
        written += *size as usize;
    }

    Ok(written)
}
