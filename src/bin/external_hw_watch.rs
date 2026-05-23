#![cfg(windows)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{
    CloseHandle, DBG_CONTINUE, DBG_EXCEPTION_NOT_HANDLED, EXCEPTION_BREAKPOINT,
    EXCEPTION_SINGLE_STEP, HANDLE,
};
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Threading::*;

const CONTEXT_DEBUG_REGISTERS_AMD64_BITS: u32 = 0x0010_0010;
const CONTEXT_CONTROL_INTEGER_DEBUG_AMD64_BITS: u32 = 0x0010_0013;
const DR7_WRITE_4BYTE: u64 =
    (1 << 0) | (1 << 1) | (1 << 8) | (1 << 9) | (0b01 << 16) | (0b11 << 18);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct RemoteHashEntry {
    guest_addr: u32,
    host_offset: u32,
    tier: u8,
    _pad: [u8; 3],
}

#[derive(Default)]
struct RemoteLookup {
    code_base: u64,
    code_size: u32,
    host_off: u32,
    guest_pc: u32,
    hash_capacity: u32,
    hash_count: u32,
}

#[derive(Debug)]
struct Args {
    retroarch: PathBuf,
    core: PathBuf,
    xbe: PathBuf,
    log: PathBuf,
    target: u32,
    seconds: u64,
}

fn wide_null(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}

fn quote_arg(path: &Path) -> String {
    format!("\"{}\"", path.display())
}

fn parse_u32_auto(s: &str) -> u32 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).expect("bad hex u32")
    } else {
        s.parse::<u32>().expect("bad u32")
    }
}

fn parse_args() -> Args {
    let mut retroarch = PathBuf::from(r"./RetroArch-Win64/retroarch.exe");
    let mut core = PathBuf::from(r"./RetroArch-Win64/cores\rustemu_core.dll");
    let mut xbe = PathBuf::from(r"./games/spiderman/default.xbe");
    let mut log = PathBuf::from(r"./retroarch-external-watch.txt");
    let mut target = 0x003F_5BEC;
    let mut seconds = 120u64;

    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--retroarch" => retroarch = PathBuf::from(it.next().expect("--retroarch path")),
            "--core" => core = PathBuf::from(it.next().expect("--core path")),
            "--xbe" => xbe = PathBuf::from(it.next().expect("--xbe path")),
            "--log" => log = PathBuf::from(it.next().expect("--log path")),
            "--target" => target = parse_u32_auto(&it.next().expect("--target addr")),
            "--seconds" => seconds = it.next().expect("--seconds n").parse().expect("seconds"),
            _ => panic!("unknown arg {arg}"),
        }
    }

    Args {
        retroarch,
        core,
        xbe,
        log,
        target,
        seconds,
    }
}

fn read_guest_base(log_path: &Path) -> Option<u64> {
    let mut f = fs::File::open(log_path).ok()?;
    let mut s = String::new();
    f.read_to_string(&mut s).ok()?;
    for line in s.lines().rev() {
        if let Some(idx) = line.find("GuestMemory: 4GB reserved at 0x") {
            let rest = &line[idx + "GuestMemory: 4GB reserved at 0x".len()..];
            let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
            if !hex.is_empty() {
                if let Ok(v) = u64::from_str_radix(&hex, 16) {
                    return Some(v);
                }
            }
        }
    }
    None
}

unsafe fn set_thread_watchpoint(thread: HANDLE, watch_host: u64) -> bool {
    let mut ctx: CONTEXT = std::mem::zeroed();
    ctx.ContextFlags = CONTEXT_FLAGS(CONTEXT_DEBUG_REGISTERS_AMD64_BITS);
    if GetThreadContext(thread, &mut ctx).is_err() {
        return false;
    }
    ctx.Dr0 = watch_host;
    ctx.Dr1 = 0;
    ctx.Dr2 = 0;
    ctx.Dr3 = 0;
    ctx.Dr6 = 0;
    ctx.Dr7 = DR7_WRITE_4BYTE;
    SetThreadContext(thread, &ctx).is_ok()
}

unsafe fn read_remote<T: Copy>(process: HANDLE, addr: u64) -> Option<T> {
    let mut out: T = std::mem::zeroed();
    let mut bytes_read = 0usize;
    ReadProcessMemory(
        process,
        addr as *const _,
        &mut out as *mut T as *mut _,
        std::mem::size_of::<T>(),
        Some(&mut bytes_read),
    )
    .ok()?;
    if bytes_read == std::mem::size_of::<T>() {
        Some(out)
    } else {
        None
    }
}

unsafe fn read_remote_vec<T: Copy + Default>(
    process: HANDLE,
    addr: u64,
    count: usize,
) -> Option<Vec<T>> {
    let byte_len = count.checked_mul(std::mem::size_of::<T>())?;
    let mut out = vec![T::default(); count];
    let mut bytes_read = 0usize;
    ReadProcessMemory(
        process,
        addr as *const _,
        out.as_mut_ptr() as *mut _,
        byte_len,
        Some(&mut bytes_read),
    )
    .ok()?;
    if bytes_read == byte_len {
        Some(out)
    } else {
        None
    }
}

unsafe fn reverse_lookup_remote(process: HANDLE, ctx_ptr: u64, rip: u64) -> RemoteLookup {
    let mut out = RemoteLookup::default();
    if ctx_ptr == 0 {
        return out;
    }

    let Some(code_base) = read_remote::<u64>(process, ctx_ptr + 0x48) else {
        return out;
    };
    let Some(code_size) = read_remote::<u32>(process, ctx_ptr + 0x50) else {
        return out;
    };
    out.code_base = code_base;
    out.code_size = code_size;
    if code_base == 0 || rip < code_base || rip >= code_base.saturating_add(code_size as u64) {
        return out;
    }

    let host_off = (rip - code_base) as u32;
    out.host_off = host_off;

    let Some(entries_ptr) = read_remote::<u64>(process, ctx_ptr + 0x58) else {
        return out;
    };
    let Some(capacity) = read_remote::<u32>(process, ctx_ptr + 0x60) else {
        return out;
    };
    let Some(count) = read_remote::<u32>(process, ctx_ptr + 0x68) else {
        return out;
    };
    out.hash_capacity = capacity;
    out.hash_count = count;
    if entries_ptr == 0 || capacity == 0 || capacity > 8_388_608 {
        return out;
    }

    let Some(entries) = read_remote_vec::<RemoteHashEntry>(process, entries_ptr, capacity as usize)
    else {
        return out;
    };
    let mut best_guest = 0u32;
    let mut best_host = 0u32;
    for entry in entries {
        if entry.guest_addr != 0 && entry.host_offset <= host_off && entry.host_offset > best_host {
            best_host = entry.host_offset;
            best_guest = entry.guest_addr;
        }
    }
    out.guest_pc = best_guest;
    out
}

unsafe fn capture_thread_context(thread: HANDLE) -> Option<CONTEXT> {
    let mut ctx: CONTEXT = std::mem::zeroed();
    ctx.ContextFlags = CONTEXT_FLAGS(CONTEXT_CONTROL_INTEGER_DEBUG_AMD64_BITS);
    GetThreadContext(thread, &mut ctx).ok()?;
    Some(ctx)
}

fn main() -> windows::core::Result<()> {
    let args = parse_args();
    let _ = fs::remove_file(&args.log);

    let cmd = format!(
        "{} -v --log-file {} -L {} {}",
        quote_arg(&args.retroarch),
        quote_arg(&args.log),
        quote_arg(&args.core),
        quote_arg(&args.xbe)
    );
    let mut cmd_w = wide_null(OsStr::new(&cmd));
    let cwd_w = wide_null(
        args.retroarch
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .as_os_str(),
    );

    println!("launch: {cmd}");
    println!("target guest: 0x{:08X}", args.target);

    unsafe {
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        CreateProcessW(
            PCWSTR::null(),
            PWSTR(cmd_w.as_mut_ptr()),
            None,
            None,
            false,
            DEBUG_ONLY_THIS_PROCESS | CREATE_NEW_CONSOLE,
            None,
            PCWSTR(cwd_w.as_ptr()),
            &si,
            &mut pi,
        )?;

        let start = Instant::now();
        let deadline = start + Duration::from_secs(args.seconds);
        let mut event: DEBUG_EVENT = std::mem::zeroed();
        let mut threads: HashMap<u32, HANDLE> = HashMap::new();
        let mut armed_threads: HashSet<u32> = HashSet::new();
        let mut guest_base: Option<u64> = None;
        let mut initial_breakpoint_seen = false;

        while Instant::now() < deadline {
            if guest_base.is_none() {
                if let Some(base) = read_guest_base(&args.log) {
                    let watch_host = base + args.target as u64;
                    println!("guest_base=0x{base:016X} watch_host=0x{watch_host:016X}");
                    guest_base = Some(base);
                    for (&tid, &thread) in &threads {
                        if set_thread_watchpoint(thread, watch_host) {
                            armed_threads.insert(tid);
                            println!("armed existing tid={tid}");
                        }
                    }
                }
            }

            if WaitForDebugEvent(&mut event, 50).is_err() {
                continue;
            }

            let code = event.dwDebugEventCode;
            let pid = event.dwProcessId;
            let tid = event.dwThreadId;
            let mut continue_status = DBG_EXCEPTION_NOT_HANDLED;

            match code {
                CREATE_PROCESS_DEBUG_EVENT => {
                    let info = event.u.CreateProcessInfo;
                    threads.insert(tid, info.hThread);
                    if let Some(base) = guest_base {
                        let watch_host = base + args.target as u64;
                        if set_thread_watchpoint(info.hThread, watch_host) {
                            armed_threads.insert(tid);
                            println!("armed process tid={tid}");
                        }
                    }
                    continue_status = DBG_CONTINUE;
                }
                CREATE_THREAD_DEBUG_EVENT => {
                    let info = event.u.CreateThread;
                    threads.insert(tid, info.hThread);
                    if let Some(base) = guest_base {
                        let watch_host = base + args.target as u64;
                        if set_thread_watchpoint(info.hThread, watch_host) {
                            armed_threads.insert(tid);
                            println!("armed new tid={tid}");
                        }
                    }
                    continue_status = DBG_CONTINUE;
                }
                EXIT_THREAD_DEBUG_EVENT => {
                    threads.remove(&tid);
                    armed_threads.remove(&tid);
                    continue_status = DBG_CONTINUE;
                }
                EXIT_PROCESS_DEBUG_EVENT => {
                    println!("process exited");
                    let _ = ContinueDebugEvent(pid, tid, DBG_CONTINUE);
                    break;
                }
                EXCEPTION_DEBUG_EVENT => {
                    let ex = event.u.Exception;
                    let ex_code = ex.ExceptionRecord.ExceptionCode;
                    if ex_code == EXCEPTION_BREAKPOINT && !initial_breakpoint_seen {
                        initial_breakpoint_seen = true;
                        continue_status = DBG_CONTINUE;
                    } else if ex_code == EXCEPTION_SINGLE_STEP {
                        let ctx = threads
                            .get(&tid)
                            .and_then(|&thread| capture_thread_context(thread));
                        if let Some(ctx) = ctx {
                            let lookup = reverse_lookup_remote(pi.hProcess, ctx.R13, ctx.Rip);
                            let watch_host = guest_base
                                .map(|base| base + args.target as u64)
                                .unwrap_or(0);
                            println!(
                                "single-step tid={tid} first={} dr6=0x{:016X} dr7=0x{:016X} hit_dr0={} watch_host=0x{:016X} rip=0x{:016X} host_off=0x{:08X} guest_pc~=0x{:08X} r13=0x{:016X} r14=0x{:08X} r15=0x{:016X} rax=0x{:016X} rcx=0x{:016X} rdx=0x{:016X} rbp=0x{:016X} hash={}/{}",
                                ex.dwFirstChance,
                                ctx.Dr6,
                                ctx.Dr7,
                                (ctx.Dr6 & 1) != 0,
                                watch_host,
                                ctx.Rip,
                                lookup.host_off,
                                lookup.guest_pc,
                                ctx.R13,
                                ctx.R14 as u32,
                                ctx.R15,
                                ctx.Rax,
                                ctx.Rcx,
                                ctx.Rdx,
                                ctx.Rbp,
                                lookup.hash_count,
                                lookup.hash_capacity,
                            );
                        } else {
                            println!(
                                "single-step tid={tid} first={} context-unavailable",
                                ex.dwFirstChance
                            );
                        }
                        continue_status = DBG_EXCEPTION_NOT_HANDLED;
                    } else {
                        continue_status = DBG_EXCEPTION_NOT_HANDLED;
                    }
                }
                _ => {
                    continue_status = DBG_CONTINUE;
                }
            }

            let _ = ContinueDebugEvent(pid, tid, continue_status);
        }

        println!(
            "done: tracked_threads={} armed_threads={} guest_base={:?}",
            threads.len(),
            armed_threads.len(),
            guest_base
        );
        let _ = CloseHandle(pi.hThread);
        let _ = CloseHandle(pi.hProcess);
    }

    Ok(())
}
