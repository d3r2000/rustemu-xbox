use super::ordinals;
use super::{KernelResult, KernelState};
/// Xbox kernel file I/O dispatch — real Windows file handles.
/// Ported from AOT_KrnlFile.cpp.
///
/// Ordinals: 187 (NtClose), 190 (NtCreateFile), 195 (NtDeleteFile),
///           198 (NtFlushBuffersFile), 202 (NtOpenFile), 211 (NtQueryInformationFile),
///           219 (NtReadFile), 226 (NtSetInformationFile), 236 (NtWriteFile),
///           327 (XeLoadSection), 328 (XeUnloadSection), and stubs.
///
/// Xbox uses ANSI strings (not Unicode) in OBJECT_ATTRIBUTES.
/// Path translation: \Device\CdRom0\ and D:\ map to the XBE directory.
use crate::xbox::memory::guest_memory::GuestMemory;

#[cfg(windows)]
use std::os::windows::io::FromRawHandle;

fn spidey_norm_guest_addr(addr: u32) -> u32 {
    if (0x8000_0000..0xA000_0000).contains(&addr) {
        addr & 0x1FFF_FFFF
    } else {
        addr
    }
}

fn log_spidey_ntcreatefile_boundary(memory: &GuestMemory, args: &[u32; 12], xbox_path: &str) {
    let lower = xbox_path.to_ascii_lowercase();
    if !lower.contains("menu.xbs")
        && !lower.contains("origin_z")
        && !lower.contains("peterstu.xbs")
        && !lower.contains("vshader.key")
        && !lower.contains("m0menu")
    {
        return;
    }

    static SPIDEY_FILE_BOUNDARY_LOG: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0);
    let n = SPIDEY_FILE_BOUNDARY_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let mut guest_eip = 0u32;
    let mut ret_addr = 0u32;
    let mut esp = 0u32;
    let mut ebp = 0u32;
    let mut source = "file";

    if let Some(frame) = crate::xbox::aot::transition::peek() {
        if frame.kind == crate::xbox::aot::transition::TransitionKind::Kernel
            && frame.ordinal as u32 == ordinals::NtCreateFile
        {
            guest_eip = frame.guest_entry_pc;
            ret_addr = frame.guest_ret_addr;
            esp = frame.entry_r14;
            source = "transition";
        }
    }

    let ctx_ptr = crate::xbox::aot::veh::get_thread_context();
    if !ctx_ptr.is_null() {
        let ctx = unsafe { &*ctx_ptr };
        if guest_eip == 0 {
            guest_eip = ctx.guest.exit_guest_addr;
        }
        if esp == 0 {
            esp = ctx.guest.esp;
        }
        ebp = ctx.guest.ebp;
    }

    if ret_addr == 0 && esp != 0 {
        ret_addr = memory.read_u32(esp);
    }

    let mut stack32 = String::new();
    if esp != 0 {
        for off in (0..0x80u32).step_by(4) {
            let v = memory.read_u32(esp.wrapping_add(off));
            stack32.push_str(&format!(" +{:02X}=0x{:08X}", off, v));
        }
    }

    let mut chain = String::new();
    if esp != 0 {
        for off in (0..0x80u32).step_by(4) {
            let v = memory.read_u32(esp.wrapping_add(off));
            if (0x0001_0000..0x0040_0000).contains(&v) {
                chain.push_str(&format!(" +{:02X}=0x{:08X}", off, v));
            }
        }
    }

    let mut ebp_chain = String::new();
    let mut frame_ebp = spidey_norm_guest_addr(ebp);
    for depth in 0..8u32 {
        if !(0x1000..0x2000_0000).contains(&frame_ebp) {
            break;
        }
        let next = spidey_norm_guest_addr(memory.read_u32(frame_ebp));
        let frame_ret = memory.read_u32(frame_ebp.wrapping_add(4));
        ebp_chain.push_str(&format!(
            " #{} ebp=0x{:08X} ret=0x{:08X} next=0x{:08X}",
            depth, frame_ebp, frame_ret, next
        ));
        if next == 0 || next == frame_ebp {
            break;
        }
        frame_ebp = next;
    }

    crate::xbox::emulator::debug_log(&format!(
        "[SPIDEY-FILE-KCALL-BOUNDARY #{}] source={} path='{}' guest_eip=0x{:08X} ret=0x{:08X} esp=0x{:08X} ebp=0x{:08X} args=[0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X},0x{:08X}] chain:{} stack32:{} ebp_chain:{}",
        n,
        source,
        xbox_path,
        guest_eip,
        ret_addr,
        esp,
        ebp,
        args[0],
        args[1],
        args[2],
        args[3],
        args[4],
        args[5],
        args[6],
        args[7],
        args[8],
        chain,
        stack32,
        ebp_chain
    ));
}

fn d_drive_cache_leak_component(xbox_path: &str) -> Option<&'static str> {
    let normalized = xbox_path
        .trim_start_matches("\\??\\")
        .replace('/', "\\")
        .to_ascii_lowercase();
    let path = normalized.strip_prefix("d:\\")?;
    for part in path.split('\\') {
        match part {
            "interface" => return Some("INTERFACE"),
            "fx" => return Some("FX"),
            "gct" => return Some("GCT"),
            _ => {}
        }
    }
    None
}

fn has_create_or_write_intent(desired_access: u32, create_disp: u32) -> bool {
    matches!(create_disp, 0 | 2 | 3 | 4 | 5)
        || (desired_access & 0x4000_0000) != 0
        || (desired_access & 0x0001_0116) != 0
}

fn write_io_status(memory: &GuestMemory, io_status_block: u32, status: u32, info: u32) {
    if io_status_block != 0 {
        memory.write_u32(io_status_block, status);
        memory.write_u32(io_status_block + 4, info);
    }
}

const SPIDEY_PETERSTU_XBS_SIZE: u64 = 0x17C740;
const SPIDEY_PETERSTU_POKE_THRESHOLD: u64 = SPIDEY_PETERSTU_XBS_SIZE - 0x1000;
const SPIDEY_PETERSTU_XBS_HEADER_START: u64 = 0x0;
const SPIDEY_PETERSTU_XBS_HEADER_END: u64 = 0x40;
const SPIDEY_PETERSTU_MAIN_RAW_START: u64 = 0x40;
const SPIDEY_PETERSTU_MAIN_RAW_END: u64 = 0x174683;
const SPIDEY_PETERSTU_TEXT_RAW_START: u64 = 0x1746A0;
const SPIDEY_PETERSTU_TEXT_RAW_END: u64 = 0x17A351;
const SPIDEY_PETERSTU_RECORDS_RAW_START: u64 = 0x17A360;
const SPIDEY_PETERSTU_RECORDS_RAW_END: u64 = 0x17C71B;
const SPIDEY_PETERSTU_MAIN_INFLATED_SIZE: u64 = 0x3CB300;
const SPIDEY_PETERSTU_TEX26_MAIN_OFF: u64 = 0x081780;
const SPIDEY_PETERSTU_TEX26_MAIN_SIZE: u64 = 0x020080;
const SPIDEY_PETERSTU_MESH29_MAIN_OFF: u64 = 0x00F000;
const SPIDEY_PETERSTU_MESH29_MAIN_SIZE: u64 = 0x0178D0;

fn spidey_env_enabled(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on" | "force"
            )
        })
        .unwrap_or(false)
}

fn spidey_env_unsafe_enabled(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "unsafe" | "force-unsafe"
            )
        })
        .unwrap_or(false)
}

fn spidey_is_peterstu_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/").to_ascii_lowercase();
    normalized.ends_with("/peterstu.xbs") || normalized.ends_with("peterstu.xbs")
}

fn spidey_format_ascii_runs(bytes: &[u8], limit: usize) -> String {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() && out.len() < limit {
        let start = i;
        while i < bytes.len()
            && (bytes[i].is_ascii_graphic() || bytes[i] == b' ')
            && bytes[i] != b'['
            && bytes[i] != b']'
        {
            i += 1;
        }
        if i.saturating_sub(start) >= 4 {
            let slice = &bytes[start..i];
            if slice.iter().any(|b| b.is_ascii_alphabetic()) {
                let mut s = String::from_utf8_lossy(slice).to_string();
                if s.len() > 48 {
                    s.truncate(48);
                    s.push_str("...");
                }
                out.push(format!("+0x{:X}='{}'", start, s.replace('\'', "`")));
            }
        }
        i = i.saturating_add(1).max(start.saturating_add(1));
    }
    if out.is_empty() {
        "-".to_string()
    } else {
        out.join(" ")
    }
}

fn spidey_peterstu_trace_read(
    host_path: &str,
    file_start: u64,
    buffer: u32,
    requested_len: u32,
    bytes: &[u8],
) {
    if bytes.is_empty()
        || (!spidey_env_enabled("RUSTEMU_SPIDEY_PETERSTU_TRACE")
            && !spidey_env_enabled("RUSTEMU_SPIDEY_PETERSTU_XBS_TRACE"))
        || !spidey_is_peterstu_path(host_path)
    {
        return;
    }

    const TAGS: &[(&str, &[u8])] = &[
        ("newent", b"newent"),
        ("conglom", b"conglom"),
        ("meshfile", b"meshfile"),
        ("spideybox", b"spideybox"),
        ("collide", b"collide"),
        ("capsule", b"capsule"),
        ("nodename", b"nodename"),
        ("bonename", b"bonename"),
        ("nobones", b"nobones"),
        ("bone", b"bone"),
        ("bip01", b"bip01"),
        ("pelvis", b"pelvis"),
        ("spine", b"spine"),
        ("head", b"head"),
        ("finger", b"finger"),
        ("thigh", b"thigh"),
        ("calf", b"calf"),
        ("foot", b"foot"),
        ("toe", b"toe"),
        ("lodstart", b"lodstart"),
        ("detail", b"detail"),
        ("mesh", b"mesh"),
        ("tmesh", b"tmesh"),
        ("xbmesh", b"xbmesh"),
        ("anmb", b"anmb"),
        ("anml", b"anml"),
        ("peterstudent", b"peterstudent"),
        ("fakehero", b"fakehero"),
        ("spiderman", b"spiderman"),
    ];

    let lower: Vec<u8> = bytes.iter().map(|b| b.to_ascii_lowercase()).collect();
    let mut hits = Vec::new();
    for (label, needle) in TAGS {
        if needle.is_empty() || needle.len() > lower.len() {
            continue;
        }
        let mut pos = 0usize;
        let mut per_tag = 0usize;
        while pos + needle.len() <= lower.len() && per_tag < 8 && hits.len() < 64 {
            if &lower[pos..pos + needle.len()] == *needle {
                hits.push(format!("{}@0x{:X}", label, file_start + pos as u64));
                per_tag += 1;
                pos += needle.len();
            } else {
                pos += 1;
            }
        }
    }

    let stream_tags = spidey_peterstu_xbs_read_tags(file_start, bytes.len() as u64);

    static SPIDEY_PETERSTU_TRACE_LOG: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0);
    let n = SPIDEY_PETERSTU_TRACE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let should_log = !hits.is_empty() || !stream_tags.is_empty() || n < 24 || n.is_power_of_two();
    if !should_log {
        return;
    }

    crate::xbox::emulator::debug_log(&format!(
        "[SPIDEY-PETERSTU-READ-TRACE] #{} host='{}' file=[0x{:X}..0x{:X}) buf=0x{:08X} req=0x{:X} read=0x{:X} streams=[{}] tags=[{}] ascii=[{}]",
        n,
        host_path,
        file_start,
        file_start + bytes.len() as u64,
        buffer,
        requested_len,
        bytes.len(),
        if stream_tags.is_empty() { "-".to_string() } else { stream_tags.join(" ") },
        if hits.is_empty() { "-".to_string() } else { hits.join(" ") },
        spidey_format_ascii_runs(bytes, 16),
    ));
}

fn spidey_peterstu_xbs_read_tags(file_start: u64, len: u64) -> Vec<String> {
    let file_end = file_start.saturating_add(len);
    let mut tags = Vec::new();
    for (label, start, end) in [
        (
            "header",
            SPIDEY_PETERSTU_XBS_HEADER_START,
            SPIDEY_PETERSTU_XBS_HEADER_END,
        ),
        (
            "main.zlib",
            SPIDEY_PETERSTU_MAIN_RAW_START,
            SPIDEY_PETERSTU_MAIN_RAW_END,
        ),
        (
            "text.zlib",
            SPIDEY_PETERSTU_TEXT_RAW_START,
            SPIDEY_PETERSTU_TEXT_RAW_END,
        ),
        (
            "records.zlib",
            SPIDEY_PETERSTU_RECORDS_RAW_START,
            SPIDEY_PETERSTU_RECORDS_RAW_END,
        ),
    ] {
        let overlap_start = file_start.max(start);
        let overlap_end = file_end.min(end);
        if overlap_start < overlap_end {
            tags.push(format!(
                "{}+0x{:X}..0x{:X}/0x{:X}",
                label,
                overlap_start - start,
                overlap_end - start,
                end - start
            ));
        }
    }
    tags
}

fn spidey_peterstu_resource_map_note() -> String {
    format!(
        "main_inflated=0x{:X} tex26(ord30,peterstudent,DDS/DXT1)=main[0x{:X}..0x{:X}) mesh29(ord31,peterstudent000)=main[0x{:X}..0x{:X}) raw_file_only_proves_compressed_streams",
        SPIDEY_PETERSTU_MAIN_INFLATED_SIZE,
        SPIDEY_PETERSTU_TEX26_MAIN_OFF,
        SPIDEY_PETERSTU_TEX26_MAIN_OFF + SPIDEY_PETERSTU_TEX26_MAIN_SIZE,
        SPIDEY_PETERSTU_MESH29_MAIN_OFF,
        SPIDEY_PETERSTU_MESH29_MAIN_OFF + SPIDEY_PETERSTU_MESH29_MAIN_SIZE,
    )
}

fn spidey_valid_low_ptr(addr: u32) -> bool {
    (0x1000..0x2000_0000).contains(&spidey_norm_guest_addr(addr))
}

fn spidey_poke_scene_ready(
    memory: &GuestMemory,
    label: &str,
    ptr: u32,
    direct_state_advance: bool,
) -> String {
    let phys = spidey_norm_guest_addr(ptr);
    if !spidey_valid_low_ptr(phys) {
        return format!(" {}=0x{:08X}/invalid", label, ptr);
    }

    let old_183 = memory.read_u8(phys.wrapping_add(0x183));
    let old_186 = memory.read_u8(phys.wrapping_add(0x186));
    let old_18c = memory.read_u8(phys.wrapping_add(0x18C));
    memory.write_u8(phys.wrapping_add(0x183), 1);
    memory.write_u8(phys.wrapping_add(0x186), 1);
    memory.write_u8(phys.wrapping_add(0x18C), 1);

    let mut state_desc = String::new();
    let ssp = memory.read_u32(phys.wrapping_add(0xA0));
    let ssp_phys = spidey_norm_guest_addr(ssp);
    if ssp_phys >= 0x18 && spidey_valid_low_ptr(ssp_phys) {
        let idx = memory.read_u32(ssp_phys.wrapping_sub(0x10));
        let arr = memory.read_u32(ssp_phys.wrapping_sub(0x14));
        let arr_phys = spidey_norm_guest_addr(arr);
        if idx < 0x1000 && spidey_valid_low_ptr(arr_phys) {
            let slot = arr_phys.wrapping_add(idx.wrapping_mul(4));
            let old_state = memory.read_u32(slot);
            let mut new_state = old_state;
            if direct_state_advance && old_state == 3 {
                new_state = 4;
                memory.write_u32(slot, new_state);
            }
            state_desc = format!(
                " state(ssp=0x{:08X} idx={} arr=0x{:08X} slot=0x{:08X} {}->{})",
                ssp, idx, arr, slot, old_state, new_state
            );
        } else {
            state_desc = format!(
                " state(ssp=0x{:08X} idx={} arr=0x{:08X}/invalid)",
                ssp, idx, arr
            );
        }
    }

    format!(
        " {}=0x{:08X}/0x{:08X} b183:{}->1 b186:{}->1 b18c:{}->1{}",
        label, ptr, phys, old_183, old_186, old_18c, state_desc
    )
}

fn spidey_try_peterstu_ready_poke(memory: &GuestMemory, host_path: &str, total_read: u64) {
    let ready_seq = crate::xbox::emulator::mark_spidey_peterstu_read_ready();
    let ready_poke_enabled = spidey_env_enabled("RUSTEMU_SPIDEY_PETERSTU_READY_POKE");
    let scene_poke = spidey_env_unsafe_enabled("RUSTEMU_SPIDEY_PETERSTU_SCENE_POKE");
    if !ready_poke_enabled || !scene_poke {
        crate::xbox::emulator::debug_log(&format!(
            "[SPIDEY-PETERSTU-READY-MARK] seq={} host='{}' total=0x{:X}/0x{:X} threshold=0x{:X} poke_env={} scene_poke=0 xbs_map={}",
            ready_seq,
            host_path,
            total_read,
            SPIDEY_PETERSTU_XBS_SIZE,
            SPIDEY_PETERSTU_POKE_THRESHOLD,
            if ready_poke_enabled { 1 } else { 0 },
            spidey_peterstu_resource_map_note()
        ));
        return;
    }

    let direct_state_advance = !std::env::var("RUSTEMU_SPIDEY_PETERSTU_READY_POKE")
        .map(|v| v.trim().eq_ignore_ascii_case("signal"))
        .unwrap_or(false);

    let frame = spidey_norm_guest_addr(memory.read_u32(0x004B_C630));
    let frame_scene = if spidey_valid_low_ptr(frame) {
        memory.read_u32(frame.wrapping_add(0x18))
    } else {
        0
    };
    let global_scene = memory.read_u32(0x003F_5BEC);
    let xgraph_root = memory.read_u32(0x004C_06B8);

    let mut details = String::new();
    details.push_str(&spidey_poke_scene_ready(
        memory,
        "frame_scene",
        frame_scene,
        direct_state_advance,
    ));
    details.push_str(&spidey_poke_scene_ready(
        memory,
        "global_scene",
        global_scene,
        direct_state_advance,
    ));
    details.push_str(&spidey_poke_scene_ready(
        memory,
        "xgraph_root",
        xgraph_root,
        direct_state_advance,
    ));

    crate::xbox::emulator::debug_log(&format!(
        "[SPIDEY-PETERSTU-READY-POKE] seq={} host='{}' total=0x{:X}/0x{:X} threshold=0x{:X} frame=0x{:08X} direct_state_advance={} xbs_map={}{}",
        ready_seq,
        host_path,
        total_read,
        SPIDEY_PETERSTU_XBS_SIZE,
        SPIDEY_PETERSTU_POKE_THRESHOLD,
        frame,
        direct_state_advance,
        spidey_peterstu_resource_map_note(),
        details
    ));
}

// ============================================================================
// File handle table — maps Xbox handles to real OS file handles
// ============================================================================
const FILE_HANDLE_BASE: u32 = 0x8000_1000;
const FILE_HANDLE_MAX: usize = 64;

/// File I/O state — handle table + XBE directory for path translation.
/// Dummy handle range for virtual partitions (no backing file).
const DUMMY_HANDLE_BASE: u32 = 0x8000_2000;
const DUMMY_HANDLE_MAX: usize = 16;
const SYNTHETIC_HANDLE_BASE: u32 = 0x8000_3000;
const SYNTHETIC_HANDLE_MAX: usize = 8;

/// Block-level partition handle — backed by HDD image file at a byte offset.
struct BlockPartition {
    file: std::fs::File,
    base_offset: u64, // byte offset into HDD image where this partition starts
    size: u64,        // partition size in bytes
    cursor: u64,      // current read/write position (relative to base_offset)
}

struct SyntheticFile {
    data: Vec<u8>,
    cursor: usize,
}

/// Pre-enumerated directory entry for NtQueryDirectoryFile.
/// Xbox FILE_DIRECTORY_INFORMATION uses ANSI filenames (1 byte per char).
#[derive(Clone)]
pub struct DirEntry {
    pub name: String, // ANSI-safe UTF-8 (Xbox filenames are 7-bit ASCII)
    pub size: u64,
    pub is_dir: bool,
    pub write_time: u64, // FILETIME (100ns since 1601-01-01)
}

/// Directory enumeration state — NtQueryDirectoryFile is stateful per handle.
/// Populated when NtCreateFile opens a directory; consumed by successive
/// NtQueryDirectoryFile calls.
pub struct DirIterState {
    pub entries: Vec<DirEntry>,
    pub cursor: usize,
}

pub struct FileState {
    handles: Vec<Option<std::fs::File>>,
    handle_paths: Vec<Option<String>>,
    handle_spidey_data_sink: Vec<bool>,
    handle_read_totals: Vec<u64>,
    handle_peterstu_ready_poked: Vec<bool>,
    /// Handles that point to partition root directories (0-6).
    /// NtReadFile on these returns synthetic sector data instead of reading the directory.
    partition_handles: Vec<Option<u8>>, // index → Some(partition_num) if partition root
    dummy_handles: Vec<bool>, // true = allocated, no backing file
    /// Block-level partition handles backed by HDD image.
    /// Keyed by dummy handle index.
    block_handles: Vec<Option<BlockPartition>>,
    /// Small in-memory files used for narrow compatibility shims where the
    /// extracted disc image is missing an optional asset.
    synthetic_handles: Vec<Option<SyntheticFile>>,
    /// Per-handle directory enumeration state (only populated for dir handles).
    pub dir_states: std::collections::HashMap<u32, DirIterState>,
    pub xbe_directory: String,    // e.g. "./games/spiderman/"
    pub system_directory: String, // RetroArch system dir for EmuDisk
    /// Dynamic symlink table: populated by IoCreateSymbolicLink (ord 67).
    /// Key: symlink name (e.g. "\\??\\Z:"), Value: target (e.g. "\\Device\\Harddisk0\\Partition6")
    /// Resolved at the top of translate_xbox_path before hardcoded checks.
    pub symlinks: std::collections::HashMap<String, String>,
    /// Symlink handle → symlink name mapping (for NtQuerySymbolicLinkObject).
    pub symlink_handles: std::collections::HashMap<u32, String>,
}

impl FileState {
    pub fn new(xbe_path: &str, system_dir: &str) -> Self {
        // Extract directory from XBE file path
        let xbe_directory = if let Some(pos) = xbe_path.rfind('\\').or_else(|| xbe_path.rfind('/'))
        {
            let mut dir = xbe_path[..=pos].to_string();
            // Ensure trailing separator
            if !dir.ends_with('\\') && !dir.ends_with('/') {
                dir.push('\\');
            }
            dir
        } else {
            ".\\".to_string()
        };

        let system_directory = if system_dir.is_empty() {
            xbe_directory.clone()
        } else {
            let mut s = system_dir.replace('\\', "/");
            if !s.ends_with('/') {
                s.push('/');
            }
            s
        };

        crate::xbox::emulator::debug_log(&format!(
            "FileState: xbe_directory='{}' system_dir='{}'",
            xbe_directory, system_directory
        ));

        Self {
            handles: (0..FILE_HANDLE_MAX).map(|_| None).collect(),
            handle_paths: (0..FILE_HANDLE_MAX).map(|_| None).collect(),
            handle_spidey_data_sink: vec![false; FILE_HANDLE_MAX],
            handle_read_totals: vec![0; FILE_HANDLE_MAX],
            handle_peterstu_ready_poked: vec![false; FILE_HANDLE_MAX],
            partition_handles: (0..FILE_HANDLE_MAX).map(|_| None).collect(),
            dummy_handles: vec![false; DUMMY_HANDLE_MAX],
            block_handles: (0..DUMMY_HANDLE_MAX).map(|_| None).collect(),
            synthetic_handles: (0..SYNTHETIC_HANDLE_MAX).map(|_| None).collect(),
            dir_states: std::collections::HashMap::new(),
            xbe_directory,
            system_directory,
            symlinks: std::collections::HashMap::new(),
            symlink_handles: std::collections::HashMap::new(),
        }
    }

    fn is_spiderman_title(&self) -> bool {
        let dir = self.xbe_directory.replace('\\', "/").to_ascii_lowercase();
        dir.contains("/spiderman/") || dir.contains("/spider-man/") || dir.ends_with("/spiderman")
    }

    /// Register a symlink from IoCreateSymbolicLink.
    pub fn add_symlink(&mut self, name: &str, target: &str) {
        let key = name.to_lowercase();
        crate::xbox::emulator::debug_log(&format!("[SYMLINK] Register: '{}' → '{}'", name, target));
        self.symlinks.insert(key, target.to_string());
    }

    /// Resolve a path through the symlink table.
    /// Returns the resolved path if a symlink matched, or None.
    pub fn resolve_symlink(&self, path: &str) -> Option<String> {
        let lower = path.to_lowercase();
        // Try \\??\X: format
        for (sym_name, sym_target) in &self.symlinks {
            if lower.starts_with(sym_name) {
                let rest = &path[sym_name.len()..];
                return Some(format!("{}{}", sym_target, rest));
            }
            // sym_name is like "\\??\z:" or "\??\z:"
            // Extract the drive letter pattern
            let drive_prefix = if sym_name.contains("\\??\\") {
                // Extract "x:" from "\\??\x:"
                sym_name.rsplit("\\??\\").next().unwrap_or("")
            } else {
                sym_name.as_str()
            };
            // Check if path starts with this drive letter
            if !drive_prefix.is_empty() && lower.starts_with(drive_prefix) {
                let rest = &path[drive_prefix.len()..];
                let resolved = format!("{}{}", sym_target, rest);
                return Some(resolved);
            }
        }
        None
    }

    pub fn translate_path(&self, xbox_path: &str) -> Option<String> {
        let resolved = self
            .resolve_symlink(xbox_path)
            .unwrap_or_else(|| xbox_path.to_string());
        translate_xbox_path(&resolved, &self.xbe_directory, &self.system_directory)
    }

    pub fn ensure_xbox_directory(&self, xbox_path: &str) -> Option<String> {
        if let Some(component) = d_drive_cache_leak_component(xbox_path) {
            crate::xbox::emulator::debug_log(&format!(
                "[D-DRIVE-DIR-LEAK] blocked create_dir component={} path='{}'",
                component, xbox_path
            ));
            return None;
        }
        let host_path = self.translate_path(xbox_path)?;
        let _ = std::fs::create_dir_all(&host_path);
        Some(host_path)
    }

    /// Allocate a dummy handle (no backing file — for virtual partitions).
    fn alloc_dummy_handle(&mut self) -> u32 {
        for (i, slot) in self.dummy_handles.iter_mut().enumerate() {
            if !*slot {
                *slot = true;
                return DUMMY_HANDLE_BASE + i as u32;
            }
        }
        DUMMY_HANDLE_BASE // reuse slot 0 if full
    }

    fn is_dummy_handle(&self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(DUMMY_HANDLE_BASE) as usize;
        idx < DUMMY_HANDLE_MAX && self.dummy_handles[idx]
    }

    fn close_dummy_handle(&mut self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(DUMMY_HANDLE_BASE) as usize;
        if idx < DUMMY_HANDLE_MAX && self.dummy_handles[idx] {
            self.dummy_handles[idx] = false;
            // Also clean up any block partition on this handle
            if idx < self.block_handles.len() {
                self.block_handles[idx] = None;
            }
            true
        } else {
            false
        }
    }

    fn alloc_synthetic_handle(&mut self, data: Vec<u8>) -> u32 {
        for (i, slot) in self.synthetic_handles.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(SyntheticFile { data, cursor: 0 });
                return SYNTHETIC_HANDLE_BASE + i as u32;
            }
        }
        0
    }

    fn synthetic_handle_mut(&mut self, xbox_handle: u32) -> Option<&mut SyntheticFile> {
        let idx = xbox_handle.wrapping_sub(SYNTHETIC_HANDLE_BASE) as usize;
        if idx < SYNTHETIC_HANDLE_MAX {
            self.synthetic_handles[idx].as_mut()
        } else {
            None
        }
    }

    fn synthetic_handle(&self, xbox_handle: u32) -> Option<&SyntheticFile> {
        let idx = xbox_handle.wrapping_sub(SYNTHETIC_HANDLE_BASE) as usize;
        if idx < SYNTHETIC_HANDLE_MAX {
            self.synthetic_handles[idx].as_ref()
        } else {
            None
        }
    }

    fn close_synthetic_handle(&mut self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(SYNTHETIC_HANDLE_BASE) as usize;
        if idx < SYNTHETIC_HANDLE_MAX && self.synthetic_handles[idx].is_some() {
            self.synthetic_handles[idx] = None;
            true
        } else {
            false
        }
    }

    /// Mark a handle as a partition root directory (for synthetic NtReadFile).
    fn set_partition_handle(&mut self, xbox_handle: u32, partition_num: u8) {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx < FILE_HANDLE_MAX {
            self.partition_handles[idx] = Some(partition_num);
        }
    }

    /// Check if a handle is a partition root (returns partition number).
    fn get_partition_num(&self, xbox_handle: u32) -> Option<u8> {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx < FILE_HANDLE_MAX {
            self.partition_handles[idx]
        } else {
            None
        }
    }

    /// Open a block-level partition handle backed by the HDD image.
    /// Returns a dummy handle that routes NtReadFile/NtWriteFile to the HDD image.
    fn open_block_partition(&mut self, partition_num: u32) -> Option<u32> {
        // Standard Xbox 8GB partition layout (LBA → byte offset)
        let (lba_start, lba_size) = match partition_num {
            3 => (0x0000_0400u64, 0x0017_7000u64), // X: cache
            4 => (0x0017_7400u64, 0x0017_7000u64), // Y: cache
            5 => (0x002E_E400u64, 0x0017_7000u64), // Z: cache
            2 => (0x0046_5400u64, 0x000F_A000u64), // C: system
            1 => (0x0055_F400u64, 0x0098_96B0u64), // E: data
            // Partition 6/7 are extended — map to end of standard disk
            6 | 7 => (0x002E_E400u64, 0x0017_7000u64), // Alias to Z: cache area
            _ => return None,
        };

        let byte_offset = lba_start * 512;
        let byte_size = lba_size * 512;

        // Open HDD image file
        let hdd_path = format!("{}xbox/xbox_hdd.img", self.system_directory);
        let file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&hdd_path)
        {
            Ok(f) => f,
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!(
                    "Block partition {}: failed to open HDD image '{}': {}",
                    partition_num, hdd_path, e
                ));
                return None;
            }
        };

        // Allocate a dummy handle and store the block partition
        let handle = self.alloc_dummy_handle();
        let idx = (handle - DUMMY_HANDLE_BASE) as usize;
        if idx < DUMMY_HANDLE_MAX {
            self.block_handles[idx] = Some(BlockPartition {
                file,
                base_offset: byte_offset,
                size: byte_size,
                cursor: 0,
            });
            crate::xbox::emulator::debug_log(&format!(
                "Block partition {}: handle=0x{:08X} offset=0x{:X} size=0x{:X} ({}MB)",
                partition_num,
                handle,
                byte_offset,
                byte_size,
                byte_size / (1024 * 1024)
            ));
            Some(handle)
        } else {
            None
        }
    }

    /// Get mutable reference to a block partition by handle.
    fn get_block_partition(&mut self, xbox_handle: u32) -> Option<&mut BlockPartition> {
        let idx = (xbox_handle.wrapping_sub(DUMMY_HANDLE_BASE)) as usize;
        if idx < DUMMY_HANDLE_MAX {
            self.block_handles[idx].as_mut()
        } else {
            None
        }
    }

    /// Check if a handle is a block partition.
    fn is_block_handle(&self, xbox_handle: u32) -> bool {
        let idx = (xbox_handle.wrapping_sub(DUMMY_HANDLE_BASE)) as usize;
        idx < DUMMY_HANDLE_MAX && self.block_handles[idx].is_some()
    }

    fn alloc_handle(&mut self, file: std::fs::File) -> u32 {
        self.alloc_handle_with_path(file, "", false)
    }

    fn alloc_handle_with_path(
        &mut self,
        file: std::fs::File,
        path: &str,
        is_spidey_sink: bool,
    ) -> u32 {
        for (i, slot) in self.handles.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(file);
                self.handle_paths[i] = if path.is_empty() {
                    None
                } else {
                    Some(path.to_string())
                };
                self.handle_spidey_data_sink[i] = is_spidey_sink;
                self.handle_read_totals[i] = 0;
                self.handle_peterstu_ready_poked[i] = false;
                return FILE_HANDLE_BASE + i as u32;
            }
        }
        0 // out of handles
    }

    fn handle_path(&self, xbox_handle: u32) -> Option<&str> {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx < FILE_HANDLE_MAX {
            self.handle_paths[idx].as_deref()
        } else {
            None
        }
    }

    fn is_spidey_data_sink_handle(&self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        idx < FILE_HANDLE_MAX && self.handle_spidey_data_sink[idx]
    }

    fn record_peterstu_read_progress(
        &mut self,
        xbox_handle: u32,
        bytes_read: usize,
    ) -> Option<(String, u64)> {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx >= FILE_HANDLE_MAX || bytes_read == 0 {
            return None;
        }

        self.handle_read_totals[idx] =
            self.handle_read_totals[idx].saturating_add(bytes_read as u64);
        if self.handle_peterstu_ready_poked[idx]
            || self.handle_read_totals[idx] < SPIDEY_PETERSTU_POKE_THRESHOLD
        {
            return None;
        }

        let path = self.handle_paths[idx].as_deref().unwrap_or("");
        if !spidey_is_peterstu_path(path) {
            return None;
        }

        self.handle_peterstu_ready_poked[idx] = true;
        Some((path.to_string(), self.handle_read_totals[idx]))
    }

    /// Populate DirIterState for a directory handle by enumerating the host dir.
    /// Called from NtCreateFile after a successful directory open so NtQueryDirectoryFile
    /// can return real filenames (activisn.bik, treyarch.bik, etc.).
    pub fn register_directory(&mut self, handle: u32, host_path: &str) {
        let mut entries: Vec<DirEntry> = Vec::new();
        let norm_host = host_path.replace('\\', "/");
        let norm_trimmed = norm_host.trim_end_matches('/').to_ascii_lowercase();
        let is_udata_root = norm_trimmed.ends_with("/xbox/emudisk/partition1/udata");
        // XDK docs: Xbox FATX enumeration OMITS "." and "..". A Bink loader that
        // blindly iterates and tries to open each name would crash on those.
        // Explicitly skip them below via filter_name().
        fn is_dot_entry(name: &str) -> bool {
            name == "." || name == ".."
        }
        match std::fs::read_dir(host_path) {
            Ok(rd) => {
                for e in rd.flatten() {
                    let name = match e.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => continue, // non-UTF-8 name — skip (Xbox is ASCII)
                    };
                    // Xbox FATX only allows ASCII in filenames; filter anything exotic
                    // so we never hand the guest a byte > 0x7F.
                    if !name.is_ascii() {
                        continue;
                    }
                    // Per XDK docs: Xbox FATX does NOT enumerate "." or "..".
                    // read_dir on Windows already omits them, but guard defensively.
                    if is_dot_entry(&name) {
                        continue;
                    }
                    let (size, is_dir, write_time) = match e.metadata() {
                        Ok(m) => {
                            let sz = m.len();
                            let dir = m.is_dir();
                            let wt = file_time_from_meta(&m);
                            (sz, dir, wt)
                        }
                        Err(_) => (0, false, 0),
                    };
                    if is_udata_root {
                        // Xbox save enumeration should only expose real save
                        // directories. Earlier probes accidentally auto-created
                        // parent folders for failed SaveMeta.xbx opens (e.g.
                        // "%08lx", "kkkkkkkk"), and Spider-Man then fed those
                        // garbage names into its fixed-size menu string pool.
                        // Keep stale host junk out of the guest view.
                        if !is_dir {
                            continue;
                        }
                        let save_meta = std::path::Path::new(host_path)
                            .join(&name)
                            .join("SaveMeta.xbx");
                        if !save_meta.exists() {
                            continue;
                        }
                    }
                    entries.push(DirEntry {
                        name,
                        size,
                        is_dir,
                        write_time,
                    });
                }
            }
            Err(e) => {
                crate::xbox::emulator::debug_log(&format!(
                    "[DIR-ENUM] read_dir('{}') failed: {} — leaving just . and ..",
                    host_path, e
                ));
            }
        }
        crate::xbox::emulator::debug_log(&format!(
            "[DIR-ENUM] handle=0x{:08X} host='{}' entries={}",
            handle,
            host_path,
            entries.len()
        ));
        self.dir_states
            .insert(handle, DirIterState { entries, cursor: 0 });
    }

    fn lookup_handle(&mut self, xbox_handle: u32) -> Option<&mut std::fs::File> {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx < FILE_HANDLE_MAX {
            self.handles[idx].as_mut()
        } else {
            None
        }
    }

    fn close_handle(&mut self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        if idx < FILE_HANDLE_MAX && self.handles[idx].is_some() {
            self.handles[idx] = None; // Drop closes the file
            self.handle_paths[idx] = None;
            self.handle_spidey_data_sink[idx] = false;
            self.handle_read_totals[idx] = 0;
            self.handle_peterstu_ready_poked[idx] = false;
            // Release any directory enumeration state for this handle.
            self.dir_states.remove(&xbox_handle);
            true
        } else {
            false
        }
    }

    fn is_file_handle(&self, xbox_handle: u32) -> bool {
        let idx = xbox_handle.wrapping_sub(FILE_HANDLE_BASE) as usize;
        idx < FILE_HANDLE_MAX && self.handles[idx].is_some()
    }
}

/// MS-DOS wildcard matcher — supports '*' (any run of chars) and '?' (any one char).
/// Case-sensitive; callers lower-case both sides beforehand to match Xbox FATX semantics.
fn wildcard_match(pattern: &str, name: &str) -> bool {
    // Special-case: empty pattern or bare "*"/"*.*" match everything.
    if pattern.is_empty() || pattern == "*" || pattern == "*.*" {
        return true;
    }
    let p: Vec<char> = pattern.chars().collect();
    let n: Vec<char> = name.chars().collect();
    // Classic iterative DP with two pointers (O(P*N), fine for short filenames).
    let (mut pi, mut ni) = (0usize, 0usize);
    let (mut star_p, mut star_n) = (None::<usize>, 0usize);
    while ni < n.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == n[ni]) {
            pi += 1;
            ni += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star_p = Some(pi);
            star_n = ni;
            pi += 1;
        } else if let Some(sp) = star_p {
            pi = sp + 1;
            star_n += 1;
            ni = star_n;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

/// Convert a SystemTime (from std::fs::Metadata) to a Windows FILETIME
/// (100ns ticks since 1601-01-01 UTC). Xbox stores timestamps in FILETIME.
fn file_time_from_meta(m: &std::fs::Metadata) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = match m.modified() {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let secs_since_unix = match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return 0,
    };
    // 11644473600 seconds between 1601 and 1970
    let secs_since_1601 = secs_since_unix + 11_644_473_600;
    secs_since_1601.saturating_mul(10_000_000)
}

// ============================================================================
// Xbox path translation
// ============================================================================

// ============================================================================
// Path translation: Xbox NT object paths → host filesystem paths.
//
// Design notes (2026-04-21 refactor):
//   - No `to_lowercase()` allocations. We match prefixes via
//     `strip_prefix_ci` which does a zero-alloc case-insensitive compare.
//   - No magic byte-index slicing (`&path[15..]`). The matcher returns
//     the remainder directly, so adding/tweaking a prefix is just a string
//     edit — no recalculating offsets.
//   - All host-path construction goes through `xbox_join`, which splits
//     the Xbox-style remainder by both `\\` and `/`, drops empty segments
//     (double-separators, trailing slashes), and pushes each as a
//     `PathBuf` segment — inheriting the OS-native separator.
//   - Rule table instead of chain-of-ifs where possible.
// ============================================================================

/// Zero-allocation case-insensitive ASCII prefix strip.
/// Returns `Some(remainder)` if `s` starts with `prefix` ignoring ASCII case.
fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.get(..prefix.len()).and_then(|head| {
        if head.eq_ignore_ascii_case(prefix) {
            Some(&s[prefix.len()..])
        } else {
            None
        }
    })
}

/// Join a base host directory with an Xbox-style relative path.
/// Splits `rel` by both `\\` and `/`, filters empty segments (handles
/// trailing/double separators), and builds a `PathBuf` with native
/// separators. Returns a `String` for compatibility with existing callers.
fn xbox_join(base: &str, rel: &str) -> String {
    let mut out = std::path::PathBuf::from(base);
    for seg in rel
        .split(|c| c == '\\' || c == '/')
        .filter(|s| !s.is_empty())
    {
        out.push(seg);
    }
    // Callers expect forward-slash output on both platforms (RetroArch logs, comparisons).
    out.to_string_lossy().replace('\\', "/")
}

/// Strip any of several prefix alternatives (case-insensitive). First hit wins.
fn strip_any_prefix_ci<'a>(s: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    prefixes.iter().find_map(|p| strip_prefix_ci(s, p))
}

/// Build an EmuDisk partition path: `<system_dir>xbox/EmuDisk/partitionN/<rel>`.
/// Falls back to `<xbe_dir>/<rel>` when `system_dir` is empty.
fn emudisk(system_dir: &str, xbe_dir: &str, part: u32, rel: &str) -> String {
    if system_dir.is_empty() {
        xbox_join(xbe_dir, rel)
    } else {
        let base = format!("{}xbox/EmuDisk/partition{}", system_dir, part);
        xbox_join(&base, rel)
    }
}

fn normalized_host_dir(path: &str) -> String {
    path.replace('\\', "/")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

fn is_spiderman_xbe_dir(xbe_dir: &str) -> bool {
    let dir = normalized_host_dir(xbe_dir);
    dir.ends_with("/spiderman") || dir.contains("/spiderman/")
}

fn classic_doom_wad_redirect(xbe_dir: &str, rel: &str) -> Option<String> {
    let normalized_rel = rel
        .replace('\\', "/")
        .trim_start_matches('/')
        .to_ascii_lowercase();
    let wad_name = normalized_rel.strip_prefix("temp/")?;
    let host_name = match wad_name {
        "doom.wad" => "DOOM.WAD",
        "doom2.wad" => "DOOM2.WAD",
        _ => return None,
    };
    let candidate = xbox_join(xbe_dir, &format!("base/classic/w/{}", host_name));
    if std::path::Path::new(&candidate).exists() {
        crate::xbox::emulator::debug_log(&format!(
            "[PATH-RESCUE] Classic Doom WAD '{}' -> '{}'",
            rel, candidate
        ));
        Some(candidate)
    } else {
        None
    }
}

fn spiderman_data_fallback(xbe_dir: &str, rel: &str) -> Option<String> {
    if !is_spiderman_xbe_dir(xbe_dir) {
        return None;
    }
    let in_data = xbox_join(&format!("{}data", xbe_dir), rel);
    if std::path::Path::new(&in_data).exists() {
        Some(in_data)
    } else {
        None
    }
}

fn translate_xbox_path(xbox_path: &str, xbe_dir: &str, system_dir: &str) -> Option<String> {
    // 1. \Device\CdRom0[\...]  →  xbe_dir/...
    if let Some(rest) = strip_any_prefix_ci(xbox_path, &[r"\Device\CdRom0\", r"\Device\CdRom0/"]) {
        return Some(xbox_join(xbe_dir, rest));
    }
    if xbox_path.eq_ignore_ascii_case(r"\Device\CdRom0") {
        return Some(xbe_dir.trim_end_matches(['\\', '/']).to_string());
    }

    // 2. \Device\Harddisk0\Partition1[\...]  →  EmuDisk/partition1/...
    //    TDATA/UDATA sub-paths are preserved (handled by xbox_join splitter).
    if let Some(rest) = strip_any_prefix_ci(
        xbox_path,
        &[
            r"\Device\Harddisk0\Partition1\",
            r"\Device\Harddisk0\Partition1/",
        ],
    ) {
        return Some(emudisk(system_dir, xbe_dir, 1, rest));
    }
    if xbox_path.eq_ignore_ascii_case(r"\Device\Harddisk0\Partition1") {
        if !system_dir.is_empty() {
            return Some(format!("{}xbox/EmuDisk/partition1/", system_dir));
        }
        return Some(xbe_dir.trim_end_matches(['\\', '/']).to_string());
    }

    // 3. \Device\Harddisk0\PartitionN[\...]  →  EmuDisk/partitionN/...
    if let Some(rest) = strip_prefix_ci(xbox_path, r"\Device\Harddisk0\Partition") {
        // rest starts with digits optionally followed by \ and more path
        let (num_str, tail) = match rest.find('\\') {
            Some(pos) => (&rest[..pos], &rest[pos + 1..]),
            None => (rest, ""),
        };
        if let Ok(part_num) = num_str.parse::<u32>() {
            if (2..=7).contains(&part_num) && !system_dir.is_empty() {
                if part_num == 2 {
                    if let Some(redirect) = classic_doom_wad_redirect(xbe_dir, tail) {
                        return Some(redirect);
                    }
                }
                let dir = format!("{}xbox/EmuDisk/partition{}/", system_dir, part_num);
                let _ = std::fs::create_dir_all(&dir);
                if tail.is_empty() {
                    return Some(dir);
                }
                return Some(emudisk(system_dir, xbe_dir, part_num, tail));
            }
        }
        crate::xbox::emulator::debug_log(&format!(
            "Path not recognized (harddisk0): '{}'",
            xbox_path
        ));
        return None;
    }

    // 4. DOS drive letters — \??\X:\... and bare X:\... forms.
    //    X → disposition:
    //      D                  → xbe_dir (disc root)
    //      T                  → EmuDisk/partition1/TDATA
    //      U                  → EmuDisk/partition1/UDATA
    //      C (NT-prefixed)    → EmuDisk/partition2 (system)
    //      C (bare dev-PC)    → xbe_dir via `\data\` marker cut
    //      Z                  → EmuDisk/partition6 (cache/scratch, auto-mkdir)
    //      other              → xbe_dir (best-effort, data-marker cut)
    if let Some((letter, rest, nt_prefixed)) = match_drive_letter(xbox_path) {
        return Some(dispatch_drive(
            letter,
            rest,
            nt_prefixed,
            xbe_dir,
            system_dir,
        ));
    }

    // 5. Relative path — Xbox CWD is the title's D: root.  Spider-Man has a
    // known extracted-layout quirk where a few relative asset probes actually
    // need D:\data, but that fallback must not become a global disc layout rule.
    if !xbox_path.starts_with('\\') && !xbox_path.starts_with('/') {
        let direct = xbox_join(xbe_dir, xbox_path);
        if std::path::Path::new(&direct).exists() {
            return Some(direct);
        }
        if let Some(in_data) = spiderman_data_fallback(xbe_dir, xbox_path) {
            return Some(in_data);
        }
        return Some(direct); // return original for diagnostic error reporting
    }

    // 6. Leading backslash without device prefix (e.g. `\sm_paths.txt`, `\data\x`).
    //    Xbox CWD is D:\ — treat as disc-relative.
    if xbox_path.starts_with('\\') || xbox_path.starts_with('/') {
        let rest = &xbox_path[1..];
        let direct = xbox_join(xbe_dir, rest);
        if std::path::Path::new(&direct).exists() {
            return Some(direct);
        }
        if let Some(in_data) = spiderman_data_fallback(xbe_dir, rest) {
            return Some(in_data);
        }
        return Some(direct);
    }

    crate::xbox::emulator::debug_log(&format!("Path not recognized: '{}'", xbox_path));
    None
}

/// Post-translation rescue for known game-side path bugs.
///
/// Spider-Man (XDK 4134) builds font/asset paths via `sprintf("%s\\%s.fon", dir,
/// name)` where `name` is sometimes an empty std::string. The result is a path
/// like `./games/spiderman/data/LOADTEX/.FON` — directory exists, filename is
/// just `.FON`. Without rescue, the file open returns ERROR_FILE_NOT_FOUND,
/// the game's font-loader assertion fires, and the App constructor never
/// completes. App stays NULL → engine never boots → 0 Draw Calls forever.
///
/// Strategy: when we see this exact pathological pattern (host path ending in
/// `/.FON` or `\.FON`), substitute a real font that's known to exist on disk
/// (`<xbe_dir>/data/loadtex/chis22.fon`). The game gets back valid font data,
/// the constructor completes, engine boots.
///
/// This is a deliberate, narrow workaround for one specific game-side
/// `sprintf` bug. Returns None if no rescue applies.
pub(super) fn rescue_path(host_path: &str, xbe_dir: &str) -> Option<String> {
    if let Some(candidate) = collapse_duplicate_final_extension(host_path) {
        if std::path::Path::new(&candidate).exists() {
            crate::xbox::emulator::debug_log(&format!(
                "[PATH-RESCUE] duplicate extension '{}' -> '{}'",
                host_path, candidate
            ));
            return Some(candidate);
        }
    }

    if !is_spiderman_xbe_dir(xbe_dir) {
        return None;
    }

    let lower = host_path.replace('\\', "/").to_ascii_lowercase();
    if !std::path::Path::new(host_path).exists()
        && lower.ends_with("/data/tips/english/training/menu.txt")
    {
        let candidate = format!("{}data/tips/ENGLISH/menu.txt", xbe_dir).replace('\\', "/");
        if std::path::Path::new(&candidate).exists() {
            crate::xbox::emulator::debug_log(&format!(
                "[PATH-RESCUE] missing Spider-Man training menu tips '{}' -> '{}'",
                host_path, candidate
            ));
            return Some(candidate);
        }
    }

    // Match `*/[\\.]FON` (case-insensitive) — i.e. extension `.FON` with empty stem.
    if !(lower.ends_with("/.fon") || lower.ends_with("\\.fon")) {
        return None;
    }
    let candidate = format!("{}data/loadtex/chis22.fon", xbe_dir).replace('\\', "/");
    if std::path::Path::new(&candidate).exists() {
        crate::xbox::emulator::debug_log(&format!(
            "[PATH-RESCUE] empty-filename .FON at '{}' → substituting '{}'",
            host_path, candidate
        ));
        Some(candidate)
    } else {
        None
    }
}

fn collapse_duplicate_final_extension(host_path: &str) -> Option<String> {
    let start = match (host_path.rfind('/'), host_path.rfind('\\')) {
        (Some(a), Some(b)) => a.max(b) + 1,
        (Some(a), None) => a + 1,
        (None, Some(b)) => b + 1,
        (None, None) => 0,
    };
    let file_name = &host_path[start..];
    let final_dot = file_name.rfind('.')?;
    let stem_with_prev_ext = &file_name[..final_dot];
    let prev_dot = stem_with_prev_ext.rfind('.')?;
    let prev_ext = &stem_with_prev_ext[prev_dot + 1..];
    let final_ext = &file_name[final_dot + 1..];
    let redundant_pair = matches!(
        (
            prev_ext.to_ascii_lowercase().as_str(),
            final_ext.to_ascii_lowercase().as_str()
        ),
        ("xsh", "xsd") | ("xsd", "xsh")
    );
    if prev_ext.is_empty() && !redundant_pair {
        return None;
    }
    if !prev_ext.eq_ignore_ascii_case(final_ext) && !redundant_pair {
        return None;
    }

    let mut collapsed = String::with_capacity(host_path.len() - (final_dot - prev_dot));
    collapsed.push_str(&host_path[..start]);
    collapsed.push_str(&stem_with_prev_ext[..prev_dot]);
    collapsed.push_str(&file_name[final_dot..]);
    Some(collapsed)
}

/// Recognize `\??\X:\...` or `X:\...` or `X:/...`, returning (letter, remainder, nt_prefixed).
/// - letter: lowercased ASCII drive letter
/// - remainder: path after the separator (does NOT include `\` or `/`)
/// - nt_prefixed: true iff the original used `\??\` form (NT-native)
fn match_drive_letter(xbox_path: &str) -> Option<(u8, &str, bool)> {
    // \??\X:\... or \??\X:/...
    if let Some(after_nt) = strip_prefix_ci(xbox_path, r"\??\") {
        if after_nt.len() >= 3 {
            let bytes = after_nt.as_bytes();
            if bytes[1] == b':'
                && (bytes[2] == b'\\' || bytes[2] == b'/')
                && bytes[0].is_ascii_alphabetic()
            {
                return Some((bytes[0].to_ascii_lowercase(), &after_nt[3..], true));
            }
        }
    }
    // Bare X:\... or X:/...
    if xbox_path.len() >= 3 {
        let bytes = xbox_path.as_bytes();
        if bytes[1] == b':'
            && (bytes[2] == b'\\' || bytes[2] == b'/')
            && bytes[0].is_ascii_alphabetic()
        {
            return Some((bytes[0].to_ascii_lowercase(), &xbox_path[3..], false));
        }
    }
    None
}

/// Cut a dev-PC style path at its first `data\` or `data/` segment and
/// route the remainder to the XBE directory. Returns `None` if no marker.
fn dev_pc_data_cut(rest: &str, xbe_dir: &str) -> Option<String> {
    let lower = rest.to_ascii_lowercase();
    let idx = lower.find("data\\").or_else(|| lower.find("data/"))?;
    Some(xbox_join(xbe_dir, &rest[idx..]))
}

fn dispatch_drive(
    letter: u8,
    rest: &str,
    nt_prefixed: bool,
    xbe_dir: &str,
    system_dir: &str,
) -> String {
    match letter {
        b'd' => xbox_join(xbe_dir, rest),
        b't' => emudisk_subdir(system_dir, xbe_dir, 1, "TDATA", rest),
        b'u' => emudisk_subdir(system_dir, xbe_dir, 1, "UDATA", rest),
        // \??\C:\ is the system partition (NT-native); bare c:\ is a dev-PC
        // path and goes through the data-marker cut to xbe_dir.
        b'c' if nt_prefixed => emudisk(system_dir, xbe_dir, 2, rest),
        b'z' => {
            if let Some(redirect) = classic_doom_wad_redirect(xbe_dir, rest) {
                return redirect;
            }
            // Z: — scratch/cache. Auto-create the parent directory so the
            // game's writes don't fail on mkdir-before-open patterns.
            let host = emudisk(system_dir, xbe_dir, 6, rest);
            if let Some(parent) = std::path::Path::new(&host).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            host
        }
        _ => {
            // Bare `c:\sm\data\...` and other dev-PC paths — cut at `data`.
            if is_spiderman_xbe_dir(xbe_dir) {
                if let Some(cut) = dev_pc_data_cut(rest, xbe_dir) {
                    return cut;
                }
            }
            xbox_join(xbe_dir, rest)
        }
    }
}

fn emudisk_subdir(system_dir: &str, xbe_dir: &str, part: u32, sub: &str, rel: &str) -> String {
    let combined = if rel.is_empty() {
        sub.to_string()
    } else {
        format!("{}/{}", sub, rel)
    };
    emudisk(system_dir, xbe_dir, part, &combined)
}

#[cfg(test)]
mod path_translation_tests {
    use super::*;

    // Ground-truth paths observed in Spider-Man debug.log runs.
    // xbe_dir always ends with a separator to match the real runtime value.
    const XBE: &str = "./games/spiderman/";
    const SYS: &str = "C:/sys/";

    fn expect(input: &str, want: &str) {
        let got = translate_xbox_path(input, XBE, SYS);
        assert_eq!(got.as_deref(), Some(want), "input={:?}", input);
    }
    fn expect_none(input: &str) {
        assert!(
            translate_xbox_path(input, XBE, SYS).is_none(),
            "input={:?}",
            input
        );
    }

    #[test]
    fn disc_root_cdrom() {
        expect(r"\Device\CdRom0\GAME.INI", "./games/spiderman/GAME.INI");
    }
    #[test]
    fn disc_root_cdrom_rel() {
        expect(
            r"\Device\CdRom0\data\sm_paths.txt",
            "./games/spiderman/data/sm_paths.txt",
        );
    }
    #[test]
    fn disc_root_cdrom_nosep() {
        expect(r"\Device\CdRom0", "./games/spiderman");
    }
    #[test]
    fn part1_base() {
        expect(
            r"\Device\Harddisk0\Partition1",
            "C:/sys/xbox/EmuDisk/partition1/",
        );
    }
    #[test]
    fn part1_tdata() {
        expect(
            r"\Device\Harddisk0\Partition1\TDATA\xyz",
            "C:/sys/xbox/EmuDisk/partition1/TDATA/xyz",
        );
    }
    #[test]
    fn part1_udata() {
        expect(
            r"\Device\Harddisk0\Partition1\UDATA\abc\save.bin",
            "C:/sys/xbox/EmuDisk/partition1/UDATA/abc/save.bin",
        );
    }
    #[test]
    fn part6_z_drive() {
        expect(
            r"\Device\Harddisk0\Partition6\scratch",
            "C:/sys/xbox/EmuDisk/partition6/scratch",
        );
    }
    #[test]
    fn nt_drive_d() {
        expect(r"\??\D:\GAME.INI", "./games/spiderman/GAME.INI");
    }
    #[test]
    fn nt_drive_t() {
        expect(
            r"\??\T:\saved",
            "C:/sys/xbox/EmuDisk/partition1/TDATA/saved",
        );
    }
    #[test]
    fn nt_drive_u() {
        expect(
            r"\??\U:\profile",
            "C:/sys/xbox/EmuDisk/partition1/UDATA/profile",
        );
    }
    #[test]
    fn nt_drive_c() {
        expect(r"\??\C:\system", "C:/sys/xbox/EmuDisk/partition2/system");
    }
    #[test]
    fn bare_d_drive() {
        expect(r"D:\GAME.INI", "./games/spiderman/GAME.INI");
    }
    #[test]
    fn bare_d_dds() {
        expect(
            r"d:\data\DDS\CHIS22.DDS",
            "./games/spiderman/data/DDS/CHIS22.DDS",
        );
    }
    #[test]
    fn bare_d_mixed_case() {
        expect(
            r"d:\data\loadtex\chis22.fon",
            "./games/spiderman/data/loadtex/chis22.fon",
        );
    }
    #[test]
    fn bare_d_empty_file() {
        expect(
            r"d:\data\LOADTEX\.FON",
            "./games/spiderman/data/LOADTEX/.FON",
        );
    } // game bug; we still translate correctly
    #[test]
    fn bare_t_drive() {
        expect(
            r"T:\save.bin",
            "C:/sys/xbox/EmuDisk/partition1/TDATA/save.bin",
        );
    }
    #[test]
    fn bare_u_drive() {
        expect(
            r"U:\profile",
            "C:/sys/xbox/EmuDisk/partition1/UDATA/profile",
        );
    }
    #[test]
    fn dev_pc_c_data() {
        expect(
            r"c:\sm\data\sm_paths.txt",
            "./games/spiderman/data/sm_paths.txt",
        );
    }
    #[test]
    fn dev_pc_c_data_lng() {
        expect(
            r"c:\sm\data\locales\english.lng",
            "./games/spiderman/data/locales/english.lng",
        );
    }
    #[test]
    fn leading_backslash() {
        expect(r"\GAME.INI", "./games/spiderman/GAME.INI");
    }
    #[test]
    fn double_separator() {
        expect(
            r"D:\\data\\DDS\\chis22.dds",
            "./games/spiderman/data/DDS/chis22.dds",
        );
    }
    #[test]
    fn forward_slash_ok() {
        expect(
            r"D:/data/DDS/chis22.dds",
            "./games/spiderman/data/DDS/chis22.dds",
        );
    }
    #[test]
    fn weird_partition() {
        expect_none(r"\Device\Harddisk0\Partition99\foo");
    }

    // Strip-prefix helpers.
    #[test]
    fn strip_ci_match() {
        assert_eq!(strip_prefix_ci("\\DEVICE\\abc", "\\device\\"), Some("abc"));
    }
    #[test]
    fn strip_ci_nomatch() {
        assert_eq!(strip_prefix_ci("\\other\\abc", "\\device\\"), None);
    }
    #[test]
    fn strip_ci_short() {
        assert_eq!(strip_prefix_ci("\\dev", "\\device\\"), None);
    }
    #[test]
    fn xbox_join_empty_segs() {
        assert_eq!(xbox_join("D:/x", r"a\\b\\c"), "D:/x/a/b/c");
        assert_eq!(xbox_join("D:/x", "/a/b/"), "D:/x/a/b");
    }
    #[test]
    fn duplicate_final_extension_collapse() {
        assert_eq!(
            collapse_duplicate_final_extension("D:/x/menu.xsh.XSH").as_deref(),
            Some("D:/x/menu.XSH")
        );
        assert_eq!(
            collapse_duplicate_final_extension(r"D:\x\menu.xsd.XSD").as_deref(),
            Some(r"D:\x\menu.XSD")
        );
        assert_eq!(
            collapse_duplicate_final_extension("D:/x/menu.xsh.XSD").as_deref(),
            Some("D:/x/menu.XSD")
        );
        assert_eq!(collapse_duplicate_final_extension("D:/x/menu.xsh"), None);
        assert_eq!(
            collapse_duplicate_final_extension("D:/x/menu.foo.XSH"),
            None
        );
    }
}

/// Check if a guest address is valid for reading (low RAM or physical mirror).
#[inline]
fn valid_guest_addr(addr: u32) -> bool {
    (addr != 0) && (addr < 0x2000_0000 || (addr >= 0x8000_0000 && addr < 0xA000_0000))
}

/// Extract ANSI path from guest OBJECT_ATTRIBUTES.
/// Read a guest ANSI_STRING struct at the given address.
/// ANSI_STRING: +0=Length(2), +2=MaxLength(2), +4=Buffer(4)
pub fn guest_ansi_string(memory: &GuestMemory, ansi_string_addr: u32) -> String {
    if !valid_guest_addr(ansi_string_addr) {
        return "<null>".to_string();
    }
    let str_len = memory.read_u16(ansi_string_addr) as u32;
    let str_buf = memory.read_u32(ansi_string_addr + 4);
    if !valid_guest_addr(str_buf) || str_len == 0 {
        return "<empty>".to_string();
    }
    let mut bytes = Vec::with_capacity(str_len as usize);
    for i in 0..str_len.min(256) {
        bytes.push(memory.read_u8(str_buf + i));
    }
    String::from_utf8_lossy(&bytes).to_string()
}

/// Diagnostic: dump OBJECT_ATTRIBUTES + ANSI_STRING + buffer bytes for a call
/// that produced an empty path. Used to trace the empty-NtCreateFile retry loop
/// Spider-Man enters after TREYARCH.bik enumeration.
fn dump_empty_path_probe(memory: &GuestMemory, args: &[u32; 12], n: u32) {
    use crate::xbox::emulator::debug_log;
    let obj_attr_addr = args[2];
    debug_log(&format!(
        "[EMPTY-PATH #{}] args[0]={:08X} args[1]={:08X} obj_attr={:08X} args[3]={:08X}",
        n, args[0], args[1], obj_attr_addr, args[3]
    ));
    if !valid_guest_addr(obj_attr_addr) {
        debug_log(&format!("[EMPTY-PATH #{}] obj_attr ptr INVALID", n));
        return;
    }
    let root_dir = memory.read_u32(obj_attr_addr);
    let p_name = memory.read_u32(obj_attr_addr + 4);
    let attrs = memory.read_u32(obj_attr_addr + 8);
    debug_log(&format!(
        "[EMPTY-PATH #{}] OBJECT_ATTRIBUTES: RootDir={:08X} pName={:08X} Attrs={:08X}",
        n, root_dir, p_name, attrs
    ));
    if !valid_guest_addr(p_name) {
        debug_log(&format!(
            "[EMPTY-PATH #{}] pName INVALID — no ANSI_STRING to decode",
            n
        ));
        return;
    }
    let s_len = memory.read_u16(p_name);
    let s_max = memory.read_u16(p_name + 2);
    let s_buf = memory.read_u32(p_name + 4);
    debug_log(&format!(
        "[EMPTY-PATH #{}] ANSI_STRING @ {:08X}: Length={} MaxLength={} Buffer={:08X}",
        n, p_name, s_len, s_max, s_buf
    ));
    if valid_guest_addr(s_buf) {
        let dump_len = 64u32.min(s_max.max(s_len) as u32 + 16);
        let mut bytes = Vec::with_capacity(dump_len as usize);
        for i in 0..dump_len {
            bytes.push(memory.read_u8(s_buf + i));
        }
        let hex: String = bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let ascii: String = bytes
            .iter()
            .map(|&b| {
                if (0x20..0x7F).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        debug_log(&format!("[EMPTY-PATH #{}] buf bytes: {}", n, hex));
        debug_log(&format!("[EMPTY-PATH #{}] buf ascii: '{}'", n, ascii));
    } else {
        debug_log(&format!(
            "[EMPTY-PATH #{}] Buffer ptr {:08X} INVALID — guest built a nil filename",
            n, s_buf
        ));
    }
}

/// Guest OBJECT_ATTRIBUTES: +0=RootDirectory(4), +4=PANSI_STRING ObjectName(4), +8=Attributes(4)
/// Guest ANSI_STRING: +0=Length(2), +2=MaxLength(2), +4=Buffer(4)
pub fn extract_object_path(memory: &GuestMemory, obj_attr_addr: u32) -> Option<String> {
    if !valid_guest_addr(obj_attr_addr) {
        return None;
    }
    let p_object_name = memory.read_u32(obj_attr_addr + 4);
    if !valid_guest_addr(p_object_name) {
        return None;
    }
    let str_len = memory.read_u16(p_object_name) as u32;
    let str_buf = memory.read_u32(p_object_name + 4);
    if !valid_guest_addr(str_buf) || str_len == 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(str_len as usize);
    for i in 0..str_len {
        bytes.push(memory.read_u8(str_buf + i));
    }
    Some(String::from_utf8_lossy(&bytes).to_string())
}

// ============================================================================
// Dispatch
// ============================================================================

pub fn dispatch(
    ordinal: u32,
    args: &[u32; 12],
    state: &mut KernelState,
    memory: &GuestMemory,
) -> Option<(KernelResult, u32)> {
    match ordinal {
        // NtClose — try file handles first
        ordinals::NtClose => {
            let handle = args[0];
            if let Some(ref mut fs) = state.file_state {
                // Check dummy handles first (virtual partitions)
                if fs.close_dummy_handle(handle) {
                    return Some((KernelResult::Handled, 0));
                }
                if fs.close_synthetic_handle(handle) {
                    return Some((KernelResult::Handled, 0));
                }
                if fs.close_handle(handle) {
                    return Some((KernelResult::Handled, 0));
                }
            }
            // Not a file handle — return None so HAL handles it (sync objects, etc.)
            None
        }

        // NtCreateFile (190) — 9 args
        ordinals::NtCreateFile => {
            let status = nt_create_file(args, state, memory);
            let path = extract_object_path(memory, args[2]).unwrap_or_default();
            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile: '{}' → 0x{:08X}{}",
                path,
                status,
                if status != 0 { " FAILED" } else { "" }
            ));
            // 2026-04-20 diagnostic for empty-path calls. Spider-Man gets into
            // a retry loop hitting NtCreateFile('') → STATUS_INVALID_PARAMETER
            // after the TREYARCH.bik enum. Dump the OBJECT_ATTRIBUTES + string
            // buffer so we can see what the guest actually built.
            if path.is_empty() {
                static EMPTY_PATH_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = EMPTY_PATH_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 8 {
                    dump_empty_path_probe(memory, args, n);
                }
            }
            Some((KernelResult::Handled, status))
        }

        // NtOpenFile (202) — 6 args → remap to NtCreateFile
        ordinals::NtOpenFile => {
            let path = extract_object_path(memory, args[2]).unwrap_or_default();
            crate::xbox::emulator::debug_log(&format!(
                "NtOpenFile: '{}' pHandle=0x{:08X} access=0x{:08X} ioStatus=0x{:08X} share=0x{:X} opts=0x{:X}",
                path, args[0], args[1], args[3], args[4], args[5]
            ));
            let mut remap_args = [0u32; 12];
            remap_args[0] = args[0]; // PHANDLE
            remap_args[1] = args[1]; // DesiredAccess
            remap_args[2] = args[2]; // POBJECT_ATTRIBUTES
            remap_args[3] = args[3]; // PIO_STATUS_BLOCK
            remap_args[4] = 0; // AllocationSize = NULL
            remap_args[5] = 0; // FileAttributes = 0
            remap_args[6] = args[4]; // ShareAccess
            remap_args[7] = 1; // CreateDisposition = FILE_OPEN
            remap_args[8] = args[5]; // CreateOptions = OpenOptions
            let status = nt_create_file(&remap_args, state, memory);
            crate::xbox::emulator::debug_log(&format!(
                "NtOpenFile: '{}' → 0x{:08X}{}",
                path,
                status,
                if status != 0 { " FAILED" } else { "" }
            ));
            Some((KernelResult::Handled, status))
        }

        // NtReadFile (219) — 8 args
        ordinals::NtReadFile => {
            let status = nt_read_file(args, state, memory);
            if status != 0 {
                crate::xbox::emulator::debug_log(&format!(
                    "NtReadFile: handle={:#X} → 0x{:08X} FAILED",
                    args[0], status
                ));
            }
            Some((KernelResult::Handled, status))
        }

        // NtWriteFile (236) — 9 args
        ordinals::NtWriteFile => Some((KernelResult::Handled, nt_write_file(args, state, memory))),

        // NtQueryInformationFile (211) — 5 args
        ordinals::NtQueryInformationFile => Some((
            KernelResult::Handled,
            nt_query_information_file(args, state, memory),
        )),

        // NtSetInformationFile (226) — 5 args
        ordinals::NtSetInformationFile => Some((
            KernelResult::Handled,
            nt_set_information_file(args, state, memory),
        )),

        // NtDeleteFile (195)
        ordinals::NtDeleteFile => {
            let fs = match state.file_state.as_ref() {
                Some(fs) => fs,
                None => return Some((KernelResult::Handled, 0xC000_000D)),
            };
            if let Some(xbox_path) = extract_object_path(memory, args[0]) {
                if let Some(host_path) = fs.translate_path(&xbox_path) {
                    match std::fs::remove_file(&host_path) {
                        Ok(_) => return Some((KernelResult::Handled, 0)),
                        Err(_) => return Some((KernelResult::Handled, 0xC000_0034)),
                    }
                }
            }
            Some((KernelResult::Handled, 0xC000_000D))
        }

        // NtFlushBuffersFile (198)
        ordinals::NtFlushBuffersFile => {
            // Flush is a best-effort operation
            Some((KernelResult::Handled, 0))
        }

        // NtDeviceIoControlFile
        //
        // 2026-04-26: previous implementation was "liar mode" — returned
        // STATUS_SUCCESS with info=0 for any IOCTL not in the known list.
        // That LIED to the caller about a successful zero-byte read, which
        // poisoned Doom's CRT heap-init: after partition2 OPEN, Doom's CRT
        // issues an IOCTL we don't enumerate, gets back "success +
        // cluster_count=0", short-circuits its heap-init success path, and
        // never publishes the heap handle into [0x007E18B0]/[0x00106E34].
        // Result: every subsequent malloc returned 0 → gamestate global
        // stayed NULL → black screen.
        //
        // Fix: return STATUS_INVALID_DEVICE_REQUEST (0xC0000010) for
        // unhandled IOCTLs. The CRT's standard fallback path treats this as
        // "device doesn't support this IOCTL" and proceeds with the next
        // step of init, allowing the heap handle to be stored.
        //
        // Spider-Man doesn't call this ordinal during its tracked kernel
        // window (verified empirically: 0 NtDeviceIoControlFile entries in
        // its call profile), so this fix is Doom-relevant without Spider-Man
        // risk.
        ordinals::NtDeviceIoControlFile => {
            let io_status = args[4];
            let ioctl = args[5];
            let in_buf = args[6];
            let in_len = args[7];
            let out_buf = args[8];
            let out_len = args[9];

            let mut info: u32 = 0;
            let mut handled = true;
            match ioctl {
                0x0004_D014 => {
                    // IOCTL_SCSI_PASS_THROUGH_DIRECT. Retail XAPI uses this
                    // while validating Xbox DVD media. Cxbx-R only fills these
                    // three DVDX2_AUTHENTICATION flags, which is enough for
                    // XapiVerifyMediaInDrive to accept the mounted disc.
                    const SPTD_DATA_BUFFER_OFFSET: u32 = 0x14;
                    const DVDX2_AUTH_PAGE_OFFSET: u32 = 0x08;
                    const DVDX2_PARTITION_AREA_OFFSET: u32 = DVDX2_AUTH_PAGE_OFFSET + 0x02;
                    const DVDX2_CDF_VALID_OFFSET: u32 = DVDX2_AUTH_PAGE_OFFSET + 0x03;
                    const DVDX2_AUTHENTICATION_OFFSET: u32 = DVDX2_AUTH_PAGE_OFFSET + 0x04;

                    if valid_guest_addr(in_buf) && in_len >= 0x1C {
                        let auth = memory.read_u32(in_buf + SPTD_DATA_BUFFER_OFFSET);
                        if valid_guest_addr(auth) {
                            memory.write_u8(auth + DVDX2_PARTITION_AREA_OFFSET, 1);
                            memory.write_u8(auth + DVDX2_CDF_VALID_OFFSET, 1);
                            memory.write_u8(auth + DVDX2_AUTHENTICATION_OFFSET, 1);
                            crate::xbox::emulator::debug_log(&format!(
                                "NtDeviceIoControlFile: IOCTL_SCSI_PASS_THROUGH_DIRECT media auth input=0x{:08X} auth=0x{:08X}",
                                in_buf, auth
                            ));
                        } else {
                            crate::xbox::emulator::debug_log(&format!(
                                "NtDeviceIoControlFile: IOCTL_SCSI_PASS_THROUGH_DIRECT missing auth buffer input=0x{:08X} auth=0x{:08X}",
                                in_buf, auth
                            ));
                        }
                    } else {
                        crate::xbox::emulator::debug_log(&format!(
                            "NtDeviceIoControlFile: IOCTL_SCSI_PASS_THROUGH_DIRECT short/invalid input=0x{:08X} len=0x{:X}",
                            in_buf, in_len
                        ));
                    }
                }
                0x0007_0000 => {
                    // IOCTL_DISK_GET_DRIVE_GEOMETRY
                    if out_buf != 0 && out_len >= 24 {
                        memory.zero_fill(out_buf, out_len as usize);
                        memory.write_u32(out_buf + 0, 1024);
                        memory.write_u32(out_buf + 4, 0);
                        memory.write_u32(out_buf + 8, 12);
                        memory.write_u32(out_buf + 12, 255);
                        memory.write_u32(out_buf + 16, 63);
                        memory.write_u32(out_buf + 20, 512);
                        info = 24;
                    }
                }
                0x0007_4004 => {
                    // IOCTL_DISK_GET_PARTITION_INFO
                    if out_buf != 0 && out_len >= 32 {
                        memory.zero_fill(out_buf, out_len as usize);
                        memory.write_u32(out_buf + 0, 0);
                        memory.write_u32(out_buf + 4, 0);
                        memory.write_u32(out_buf + 8, 0);
                        memory.write_u32(out_buf + 12, 2);
                        memory.write_u32(out_buf + 16, 0);
                        memory.write_u32(out_buf + 20, 1);
                        info = 32;
                    }
                }
                _ => {
                    handled = false;
                    crate::xbox::emulator::debug_log(&format!(
                        "NtDeviceIoControlFile: UNHANDLED IOCTL=0x{:08X} → STATUS_INVALID_DEVICE_REQUEST",
                        ioctl
                    ));
                }
            }

            let status: u32 = if handled { 0 } else { 0xC000_0010 };
            if io_status != 0 {
                memory.write_u32(io_status, status);
                memory.write_u32(io_status + 4, info);
            }
            Some((KernelResult::Handled, status))
        }

        // NtQueryVolumeInformationFile
        ordinals::NtQueryVolumeInformationFile => {
            let handle = args[0];
            let io_status = args[1];
            let vol_info = args[2];
            let length = args[3];
            let info_class = args[4];

            crate::xbox::emulator::debug_log(&format!(
                "NtQueryVolumeInformationFile: handle=0x{:08X} class={} buf=0x{:08X} len={}",
                handle, info_class, vol_info, length
            ));

            if info_class == 3 && vol_info != 0 && length >= 24 {
                // FileFsSizeInformation
                memory.zero_fill(vol_info, 24);
                // Xbox HDD: 8GB, 512 bytes/sector, 32 sectors/cluster (16KB clusters)
                let total_clusters: u64 = 0x0017_7000 / 32; // ~750MB partition / 32 sectors
                let free_clusters: u64 = total_clusters / 2;
                memory.write_u32(vol_info + 0, total_clusters as u32);
                memory.write_u32(vol_info + 4, (total_clusters >> 32) as u32);
                memory.write_u32(vol_info + 8, free_clusters as u32);
                memory.write_u32(vol_info + 12, (free_clusters >> 32) as u32);
                memory.write_u32(vol_info + 16, 32); // SectorsPerAllocationUnit
                memory.write_u32(vol_info + 20, 512); // BytesPerSector (Xbox = 512)
                if io_status != 0 {
                    memory.write_u32(io_status, 0);
                    memory.write_u32(io_status + 4, 24);
                }
                Some((KernelResult::Handled, 0))
            } else if info_class == 4 && vol_info != 0 && length >= 8 {
                // FileFsDeviceInformation
                memory.write_u32(vol_info, 7); // FILE_DEVICE_DISK
                memory.write_u32(vol_info + 4, 0);
                if io_status != 0 {
                    memory.write_u32(io_status, 0);
                    memory.write_u32(io_status + 4, 8);
                }
                Some((KernelResult::Handled, 0))
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "NtQueryVolumeInformationFile: unhandled class={}",
                    info_class
                ));
                if io_status != 0 {
                    memory.write_u32(io_status, 0xC000_0003);
                    memory.write_u32(io_status + 4, 0);
                }
                Some((KernelResult::Handled, 0xC000_0003))
            }
        }

        // NtDuplicateObject
        ordinals::NtDuplicateObject => {
            if valid_guest_addr(args[1]) {
                memory.write_u32(args[1], args[0]); // Return same handle
            }
            Some((KernelResult::Handled, 0))
        }

        // NtFsControlFile
        ordinals::NtFsControlFile => {
            let handle = args[0];
            let io_status = args[4];
            // Block partition handles: return success (FATX is already formatted in HDD image)
            if let Some(fs) = state.file_state.as_ref() {
                if fs.is_block_handle(handle) {
                    if io_status != 0 {
                        memory.write_u32(io_status, 0);
                        memory.write_u32(io_status + 4, 0);
                    }
                    return Some((KernelResult::Handled, 0));
                }
            }
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0002);
                memory.write_u32(io_status + 4, 0);
            }
            Some((KernelResult::Handled, 0xC000_0002))
        }

        // NtQueryDirectoryFile — enumerate pre-populated DirIterState.
        // Xbox signature (per Cxbx-R EmuKrnlNt.cpp, 10 args after stdcall cleanup):
        //   args[0] FileHandle
        //   args[1] Event            (ignored — we're synchronous)
        //   args[2] ApcRoutine       (ignored)
        //   args[3] ApcContext       (ignored)
        //   args[4] IoStatusBlock
        //   args[5] FileInformation  (output buffer)
        //   args[6] Length           (buffer size in bytes)
        //   args[7] FileInformationClass  (must be FileDirectoryInformation = 1)
        //   args[8] FileMask         (POBJECT_STRING wildcard — "*.bik", "bonus", etc.
        //                             If non-null we filter entries by MS-DOS wildcards.
        //                             Matches Cxbx-R behavior — ignoring mask causes the
        //                             game's parser to treat "wrong file returned" as
        //                             corruption and issue KeBugCheck(0xC0000144).)
        //   args[9] RestartScan      (TRUE resets the per-handle enumeration cursor)
        //
        // Xbox FILE_DIRECTORY_INFORMATION layout (ANSI filename, NOT UTF-16):
        //   +0x00 ULONG NextEntryOffset   (0 for last entry)
        //   +0x04 ULONG FileIndex
        //   +0x08 LARGE_INTEGER CreationTime
        //   +0x10 LARGE_INTEGER LastAccessTime
        //   +0x18 LARGE_INTEGER LastWriteTime
        //   +0x20 LARGE_INTEGER ChangeTime
        //   +0x28 LARGE_INTEGER EndOfFile
        //   +0x30 LARGE_INTEGER AllocationSize
        //   +0x38 ULONG FileAttributes
        //   +0x3C ULONG FileNameLength   (in BYTES, not chars)
        //   +0x40 CHAR   FileName[1]     (ANSI, not WCHAR — Xbox FATX only)
        ordinals::NtQueryDirectoryFile => {
            let handle = args[0];
            let io_status = args[4];
            let buffer = args[5];
            let buf_length = args[6];
            let _info_class = args[7];
            let file_mask_ptr = args[8];
            let restart_scan = args[9] != 0;

            // Read the FileMask (POBJECT_STRING: [Length:u16][MaxLen:u16][Buffer:ptr]).
            // If NULL or malformed, default to wildcard "*" (match anything).
            let file_mask: String = if file_mask_ptr != 0 {
                let len = memory.read_u16(file_mask_ptr) as u32;
                let buf = memory.read_u32(file_mask_ptr + 4);
                if len > 0 && len < 256 && buf != 0 {
                    let mut s = String::with_capacity(len as usize);
                    for i in 0..len {
                        let b = memory.read_u8(buf + i);
                        if b == 0 {
                            break;
                        }
                        s.push(b as char);
                    }
                    s
                } else {
                    "*".to_string()
                }
            } else {
                "*".to_string()
            };

            let fs = match state.file_state.as_mut() {
                Some(fs) => fs,
                None => {
                    if io_status != 0 {
                        memory.write_u32(io_status, 0xC000_0008);
                        memory.write_u32(io_status + 4, 0);
                    }
                    return Some((KernelResult::Handled, 0xC000_0008));
                }
            };

            let dir = match fs.dir_states.get_mut(&handle) {
                Some(d) => d,
                None => {
                    // Handle isn't a directory we registered — report as not a dir.
                    if io_status != 0 {
                        memory.write_u32(io_status, 0xC000_0103); // STATUS_NOT_A_DIRECTORY
                        memory.write_u32(io_status + 4, 0);
                    }
                    crate::xbox::emulator::debug_log(&format!(
                        "NtQueryDirectoryFile: handle=0x{:08X} not a dir",
                        handle
                    ));
                    return Some((KernelResult::Handled, 0xC000_0103));
                }
            };

            if restart_scan {
                dir.cursor = 0;
            }

            // Advance cursor to the next entry matching file_mask (MS-DOS wildcard).
            // Case-insensitive per XDK FATX semantics.
            let mask_lower = file_mask.to_lowercase();
            while dir.cursor < dir.entries.len() {
                let name_lower = dir.entries[dir.cursor].name.to_lowercase();
                if wildcard_match(&mask_lower, &name_lower) {
                    break;
                }
                dir.cursor += 1;
            }

            // When returning the entry, present the filename with the CASE the
            // guest requested. Real Xbox FATX preserves case exactly as written,
            // and the XDK's 2002-era string-builder assumes the returned FileName
            // matches the pattern case. Our host dir often has lowercase names
            // (extraction tool quirk), so we overlay the mask's case onto the
            // concrete name whenever the mask is a literal (no wildcards).
            let returned_name: String = if !file_mask.contains('*')
                && !file_mask.contains('?')
                && dir.cursor < dir.entries.len()
                && dir.entries[dir.cursor].name.len() == file_mask.len()
            {
                file_mask.clone()
            } else {
                // Wildcard pattern — no safe way to reconstruct the disc's case.
                // Fall back to the host filename as-is.
                dir.entries
                    .get(dir.cursor)
                    .map(|e| e.name.clone())
                    .unwrap_or_default()
            };

            if dir.cursor >= dir.entries.len() {
                // Exhausted — STATUS_NO_MORE_FILES.
                if io_status != 0 {
                    memory.write_u32(io_status, 0x8000_0006);
                    memory.write_u32(io_status + 4, 0);
                }
                static EXHAUST_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = EXHAUST_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 80 {
                    crate::xbox::emulator::debug_log(&format!(
                        "NtQueryDirectoryFile #{} handle=0x{:08X} mask='{}' exhausted entries={} restart={} -> STATUS_NO_MORE_FILES",
                        n,
                        handle,
                        file_mask,
                        dir.entries.len(),
                        restart_scan
                    ));
                }
                return Some((KernelResult::Handled, 0x8000_0006));
            }

            // Emit ONE entry per call (Xbox games almost always pass ReturnSingleEntry=TRUE).
            // Using single-entry output keeps us safe regardless of the caller's intent.
            let entry = &dir.entries[dir.cursor];
            let name_bytes = returned_name.as_bytes();
            let name_len = name_bytes.len() as u32;
            let record_size: u32 = 0x40u32 + name_len;

            if record_size > buf_length || buffer == 0 {
                if io_status != 0 {
                    memory.write_u32(io_status, 0x8000_0005); // STATUS_BUFFER_OVERFLOW
                    memory.write_u32(io_status + 4, 0);
                }
                return Some((KernelResult::Handled, 0x8000_0005));
            }

            // Zero the header region (0x40 bytes)
            for off in (0..0x40u32).step_by(4) {
                memory.write_u32(buffer + off, 0);
            }

            // NextEntryOffset = 0 (single entry per call)
            memory.write_u32(buffer + 0x00, 0);
            // FileIndex
            memory.write_u32(buffer + 0x04, dir.cursor as u32);
            // LastWriteTime — reuse for creation/access/change times too.
            let wt = entry.write_time;
            let wt_lo = wt as u32;
            let wt_hi = (wt >> 32) as u32;
            for off in [0x08u32, 0x10, 0x18, 0x20] {
                memory.write_u32(buffer + off, wt_lo);
                memory.write_u32(buffer + off + 4, wt_hi);
            }
            // EndOfFile
            memory.write_u32(buffer + 0x28, entry.size as u32);
            memory.write_u32(buffer + 0x2C, (entry.size >> 32) as u32);
            // AllocationSize — round up to 4KB (Xbox cluster-equivalent)
            let alloc = (entry.size + 4095) & !4095;
            memory.write_u32(buffer + 0x30, alloc as u32);
            memory.write_u32(buffer + 0x34, (alloc >> 32) as u32);
            // FileAttributes (0x10 = directory, 0x80 = normal)
            memory.write_u32(buffer + 0x38, if entry.is_dir { 0x10 } else { 0x80 });
            // FileNameLength (in bytes)
            memory.write_u32(buffer + 0x3C, name_len);
            // FileName (ANSI bytes directly)
            for (i, b) in name_bytes.iter().enumerate() {
                memory.write_u8(buffer + 0x40 + i as u32, *b);
            }

            if io_status != 0 {
                memory.write_u32(io_status, 0); // STATUS_SUCCESS
                memory.write_u32(io_status + 4, record_size); // bytes returned
            }

            // Log the first dozen entries per handle so we can see MOVIES/ enum in debug.log.
            static LOG_CTR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = LOG_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 80 {
                crate::xbox::emulator::debug_log(&format!(
                    "NtQueryDirectoryFile #{} handle=0x{:08X} mask='{}' → idx={} name='{}' (disc='{}') size={} attr=0x{:X}",
                    n, handle, file_mask, dir.cursor, returned_name, entry.name, entry.size,
                    if entry.is_dir { 0x10 } else { 0x80 }
                ));
            }

            dir.cursor += 1;
            Some((KernelResult::Handled, 0))
        }

        // NtQueryFullAttributesFile
        ordinals::NtQueryFullAttributesFile => {
            let fs = match state.file_state.as_ref() {
                Some(fs) => fs,
                None => return Some((KernelResult::Handled, 0xC000_0034)),
            };
            if let Some(xbox_path) = extract_object_path(memory, args[0]) {
                crate::xbox::emulator::debug_log(&format!(
                    "[FILE] NtQueryFullAttributesFile xbox='{}' args0=0x{:08X}",
                    xbox_path, args[0]
                ));
                if let Some(host_path) = fs.translate_path(&xbox_path) {
                    let exists = std::path::Path::new(&host_path).exists();
                    crate::xbox::emulator::debug_log(&format!(
                        "[FILE] NtQueryFullAttributesFile '{}' → host '{}' exists={}",
                        xbox_path, host_path, exists
                    ));
                    if exists {
                        if valid_guest_addr(args[1]) {
                            // FILE_NETWORK_OPEN_INFORMATION (56 bytes):
                            //   +0x00 CreationTime      (8)
                            //   +0x08 LastAccessTime     (8)
                            //   +0x10 LastWriteTime      (8)
                            //   +0x18 ChangeTime         (8)
                            //   +0x20 AllocationSize     (8)
                            //   +0x28 EndOfFile          (8)
                            //   +0x30 FileAttributes     (4)
                            memory.zero_fill(args[1], 56);
                            let p = std::path::Path::new(&host_path);
                            let attrs = if p.is_dir() { 0x10u32 } else { 0x80u32 };
                            memory.write_u32(args[1] + 0x30, attrs);
                            // Fill in file size so the game doesn't think it's empty
                            if let Ok(meta) = std::fs::metadata(&host_path) {
                                let size = meta.len();
                                memory.write_u32(args[1] + 0x28, size as u32);
                                memory.write_u32(args[1] + 0x2C, (size >> 32) as u32);
                                // AllocationSize = round up to 4KB
                                let alloc = (size + 4095) & !4095;
                                memory.write_u32(args[1] + 0x20, alloc as u32);
                                memory.write_u32(args[1] + 0x24, (alloc >> 32) as u32);
                            }
                        }
                        return Some((KernelResult::Handled, 0));
                    }
                }
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "[FILE] NtQueryFullAttributesFile: extract_object_path FAILED args0=0x{:08X}",
                    args[0]
                ));
            }
            Some((KernelResult::Handled, 0xC000_0034))
        }

        // NtReadFileScatter / NtWriteFileGather — not implemented
        ordinals::NtReadFileScatter | ordinals::NtWriteFileGather => {
            Some((KernelResult::Handled, 0xC000_0002))
        }

        // XeLoadSection (327)
        ordinals::XeLoadSection => {
            let p_section = args[0];
            if !valid_guest_addr(p_section) {
                return Some((KernelResult::Handled, 0xC000_0008));
            }

            let ref_count = memory.read_u32(p_section + 0x18);

            if ref_count == 0 {
                let va = memory.read_u32(p_section + 0x04);
                let vsize = memory.read_u32(p_section + 0x08);
                let raw_addr = memory.read_u32(p_section + 0x0C);
                let raw_size = memory.read_u32(p_section + 0x10);

                if valid_guest_addr(va) && vsize != 0 {
                    // NOTE: Do NOT zero_fill here. All sections are already
                    // loaded by the XBE loader at parse time. Zero-filling
                    // would destroy live code/data (including SEH handlers),
                    // causing SEH chain corruption and crashes.

                    // Copy raw data from XBE file
                    let xbe_data = &state.xbe_file_data;
                    let xbe_len = xbe_data.len() as u32;
                    if raw_addr != 0
                        && raw_size != 0
                        && !xbe_data.is_empty()
                        && raw_addr.wrapping_add(raw_size) <= xbe_len
                    {
                        let copy = raw_size.min(vsize);
                        memory.copy_into(
                            va,
                            &xbe_data[raw_addr as usize..(raw_addr + copy) as usize],
                        );
                    }

                    // If this section overlaps the kernel thunk table, re-resolve thunks.
                    // XeLoadSection copies raw XBE data which contains unresolved
                    // 0x80000000|ordinal entries, overwriting our resolved 0xFFFF0000+ordinal values.
                    if let Some((thunk_addr, base, img_size)) = state.thunk_info {
                        let sect_end = va.wrapping_add(vsize);
                        if thunk_addr >= va && thunk_addr < sect_end {
                            let tt = crate::xbox::loader::xbe::ThunkTable::resolve(
                                memory, thunk_addr, base, img_size,
                            );
                            crate::xbox::emulator::debug_log(&format!(
                                "XeLoadSection: re-resolved {} kernel thunks (section overlaps thunk table)",
                                tt.len()
                            ));
                        }
                    }

                    crate::xbox::emulator::debug_log(&format!(
                        "XeLoadSection(0x{:08X}): VA=0x{:08X} VSize=0x{:X} loaded",
                        p_section, va, vsize
                    ));
                }
            }

            memory.write_u32(p_section + 0x18, ref_count + 1);
            let head = memory.read_u16(p_section + 0x1C);
            let tail = memory.read_u16(p_section + 0x1E);
            memory.write_u16(p_section + 0x1C, head.wrapping_add(1));
            memory.write_u16(p_section + 0x1E, tail.wrapping_add(1));

            Some((KernelResult::Handled, 0))
        }

        // XeUnloadSection (328)
        ordinals::XeUnloadSection => {
            let p_section = args[0];
            if !valid_guest_addr(p_section) {
                return Some((KernelResult::Handled, 0xC000_000D));
            }

            let ref_count = memory.read_u32(p_section + 0x18);
            if ref_count > 0 {
                memory.write_u32(p_section + 0x18, ref_count - 1);

                if ref_count == 1 {
                    let va = memory.read_u32(p_section + 0x04);
                    crate::xbox::emulator::debug_log(&format!(
                        "XeUnloadSection(0x{:08X}): VA=0x{:08X} unloaded",
                        p_section, va
                    ));
                }

                let head = memory.read_u16(p_section + 0x1C);
                let tail = memory.read_u16(p_section + 0x1E);
                if head > 0 {
                    memory.write_u16(p_section + 0x1C, head - 1);
                }
                if tail > 0 {
                    memory.write_u16(p_section + 0x1E, tail - 1);
                }
            }

            Some((KernelResult::Handled, 0))
        }

        _ => None,
    }
}

// ============================================================================
// NtCreateFile implementation
// ============================================================================
fn nt_create_file(args: &[u32; 12], state: &mut KernelState, memory: &GuestMemory) -> u32 {
    let p_handle = args[0];
    let desired_access = args[1];
    let obj_attr = args[2];
    let io_status_block = args[3];
    let _alloc_size = args[4];
    let file_attribs = args[5];
    let share_access = args[6];
    let create_disp = args[7];
    let create_options = args[8];

    let fs = match state.file_state.as_mut() {
        Some(fs) => fs,
        None => {
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0xC000_000D);
                memory.write_u32(io_status_block + 4, 0);
            }
            return 0xC000_000D;
        }
    };

    // Extract Xbox ANSI path
    let xbox_path = match extract_object_path(memory, obj_attr) {
        Some(p) => p,
        None => {
            // 2026-04-20 Path A: empty ObjectName + non-null RootDirectory.
            //
            // NT idiom: opening an empty path against a RootDirectory handle
            // means "duplicate this dir handle" or "query attrs of the dir
            // itself". Spider-Man's movie-loader hits this after we bypass the
            // stash gate — the hero pointer is NULL so the filename builder
            // produces an empty string, and it forwards a pseudo-handle
            // (0xFFFFFFFD) as RootDirectory.
            //
            // Returning STATUS_INVALID_PARAMETER (0xC000000D) was triggering a
            // tight retry loop because the game's file loader treats that as
            // "transient I/O error, try again". Returning STATUS_OBJECT_NAME_
            // NOT_FOUND (0xC0000034) is interpreted as "expected — the file is
            // genuinely absent on this disc" and the loader falls through to
            // the loose-file path. This mirrors the Xbox kernel's behavior for
            // empty-name opens on non-FATX drives.
            let root_dir = if valid_guest_addr(obj_attr) {
                memory.read_u32(obj_attr)
            } else {
                0
            };
            let status = if root_dir != 0 {
                0xC000_0034 // STATUS_OBJECT_NAME_NOT_FOUND
            } else {
                0xC000_000D // real INVALID_PARAMETER — bare null obj_attr
            };
            static EMPTY_NAME_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = EMPTY_NAME_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 8 {
                crate::xbox::emulator::debug_log(&format!(
                    "[NtCreate empty-name] RootDir=0x{:08X} → status=0x{:08X} (was 0xC000000D)",
                    root_dir, status
                ));
            }
            if io_status_block != 0 {
                memory.write_u32(io_status_block, status);
                memory.write_u32(io_status_block + 4, 0);
            }
            return status;
        }
    };
    log_spidey_ntcreatefile_boundary(memory, args, &xbox_path);

    if let Some(component) = d_drive_cache_leak_component(&xbox_path) {
        if has_create_or_write_intent(desired_access, create_disp) {
            const STATUS_ACCESS_DENIED: u32 = 0xC000_0022;
            if p_handle != 0 {
                memory.write_u32(p_handle, 0);
            }
            write_io_status(memory, io_status_block, STATUS_ACCESS_DENIED, 0);
            crate::xbox::emulator::debug_log(&format!(
                "[D-DRIVE-WRITE-LEAK] blocked component={} path='{}' desired=0x{:08X} attrs=0x{:08X} share=0x{:08X} disp={} opts=0x{:08X} obj=0x{:08X} pHandle=0x{:08X}",
                component,
                xbox_path,
                desired_access,
                file_attribs,
                share_access,
                create_disp,
                create_options,
                obj_attr,
                p_handle
            ));
            return STATUS_ACCESS_DENIED;
        }
    }

    // Partition0: serve synthetic PARTINFO (Xbox HDD partition table).
    // The game reads 512 bytes of partition layout data. Without this,
    // it calls HalReturnToFirmware(2) and never reaches CreateDevice.
    {
        let lower_check = xbox_path.to_lowercase();
        // Partition0 (raw HDD partition table): return SUCCESS with dummy handle.
        // Serves synthetic data via NtReadFile on the dummy handle.
        // The rep movsd crash (ECX=0x38307F03 at guest 0x002B7397) is caught by
        // VEH and skipped — it's a memcpy with uninitialized size from partition
        // data parsing. HalReturnToFirmware is also NOPed as safety net.
        // Only intercept BARE partition paths (no sub-path after partition number).
        // Paths like \Device\Harddisk0\Partition1\TDATA\file must go through normal file open.
        let is_bare_partition0 = {
            if let Some(pos) = lower_check.find("partition0") {
                let after = &lower_check[pos + "partition0".len()..];
                after.is_empty() || after == "\\" || after == "/"
            } else {
                false
            }
        };
        let is_bare_partition1 = {
            if let Some(pos) = lower_check.find("partition1") {
                let after = &lower_check[pos + "partition1".len()..];
                after.is_empty() || after == "\\" || after == "/"
            } else {
                false
            }
        };
        if is_bare_partition0 {
            // Spider-Man's known-good baseline was partition0 FAIL + handled.
            // The newer generic partition0 success path is useful for other
            // titles, but Spider-Man still follows the unstable success branch:
            // first 0x200 read at offset 0x800 is followed by a jump to
            // 0xFFFFFFFF before D3D CreateDevice. Keep this title-specific and
            // leave the generic Cxbx-R-style partition0 reader below intact.
            if fs.is_spiderman_title() {
                const STATUS_OBJECT_NAME_NOT_FOUND: u32 = 0xC000_0034;
                if p_handle != 0 {
                    memory.write_u32(p_handle, 0);
                }
                write_io_status(memory, io_status_block, STATUS_OBJECT_NAME_NOT_FOUND, 0);
                crate::xbox::emulator::debug_log(&format!(
                    "[SPIDEY-PARTITION0-FAIL-HANDLED] '{}' -> STATUS_OBJECT_NAME_NOT_FOUND (stable Spider-Man partition0 path)",
                    xbox_path
                ));
                return STATUS_OBJECT_NAME_NOT_FOUND;
            }

            // partition0 (config/dashboard): return SUCCESS with dummy handle.
            // Serves full partition0.bin data via NtReadFile. Cxbx-R does the same:
            // opens partition0.bin as a real file and forwards reads.
            let dummy_handle = fs.alloc_dummy_handle();
            if p_handle != 0 {
                memory.write_u32(p_handle, dummy_handle);
            }
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0); // STATUS_SUCCESS
                memory.write_u32(io_status_block + 4, 0);
            }
            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile: '{}' → SUCCESS dummy_handle=0x{:X} (partition0 — serves partition0.bin)",
                xbox_path, dummy_handle
            ));
            return 0; // STATUS_SUCCESS
        }
        // partition1 (TDATA/UDATA metadata): return dummy handle for reads.
        if is_bare_partition1 {
            let dummy_handle = fs.alloc_dummy_handle();
            if p_handle != 0 {
                memory.write_u32(p_handle, dummy_handle);
            }
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0); // STATUS_SUCCESS
                memory.write_u32(io_status_block + 4, 0);
            }
            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile: '{}' → SUCCESS (partition1 dummy handle=0x{:08X})",
                xbox_path, dummy_handle
            ));
            return 0; // STATUS_SUCCESS
        }
        // Partitions 2-7: open as block-level handle backed by HDD image.
        // Game needs raw FATX partition access for cache reads/writes.
        {
            use std::str::FromStr;
            // Extract partition number from path like "partition3" or "Partition7"
            let part_prefix = "partition";
            if let Some(pos) = lower_check.rfind(part_prefix) {
                let after = &lower_check[pos + part_prefix.len()..];
                let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(part_num) = u32::from_str(&num_str) {
                    if part_num >= 2 && part_num <= 7 {
                        // Check if path has a sub-path (e.g., partition1\TDATA\...)
                        let has_subpath = xbox_path.len()
                            > lower_check.rfind(part_prefix).unwrap()
                                + part_prefix.len()
                                + num_str.len()
                                + 1;
                        if !has_subpath {
                            // Root partition access — open block handle
                            if let Some(handle) = fs.open_block_partition(part_num) {
                                if p_handle != 0 {
                                    memory.write_u32(p_handle, handle);
                                }
                                if io_status_block != 0 {
                                    memory.write_u32(io_status_block, 0);
                                    memory.write_u32(io_status_block + 4, 0);
                                }
                                crate::xbox::emulator::debug_log(&format!(
                                    "NtCreateFile: '{}' → SUCCESS (block partition {} handle=0x{:08X})",
                                    xbox_path, part_num, handle
                                ));
                                return 0; // STATUS_SUCCESS
                            }
                        }
                    }
                }
            }
        }
    }

    // Translate to host path (normalize separators), then apply rescue
    // for known game-side path bugs (e.g. Spider-Man's empty-filename .FON).
    let host_path = match fs.translate_path(&xbox_path) {
        Some(p) => {
            let normalized = p.replace('\\', "/");
            rescue_path(&normalized, &fs.xbe_directory).unwrap_or(normalized)
        }
        None => {
            // Relative path (no \Device\ prefix) — resolve against XBE directory.
            // Games open files relative to their working directory (the disc root).
            // Example: game opens "sm_paths.txt" → ./games/spiderman/sm_paths.txt
            // Spider-Man has a narrow extracted-layout fallback into data/.
            if !xbox_path.starts_with('\\') {
                let candidate = format!("{}{}", fs.xbe_directory, xbox_path).replace('\\', "/");
                if std::path::Path::new(&candidate).exists() {
                    candidate
                } else if let Some(data_candidate) =
                    spiderman_data_fallback(&fs.xbe_directory, &xbox_path)
                {
                    data_candidate
                } else {
                    candidate // use original for error reporting
                }
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "NtCreateFile: cannot translate '{}'",
                    xbox_path
                ));
                if io_status_block != 0 {
                    memory.write_u32(io_status_block, 0xC000_003A);
                    memory.write_u32(io_status_block + 4, 0);
                }
                return 0xC000_003A; // STATUS_OBJECT_PATH_NOT_FOUND
            }
        }
    };

    // Check if directory open. FILE_NON_DIRECTORY_FILE must override host
    // filesystem shape: games may probe a path that happens to be an extracted
    // host directory but explicitly request a normal file handle.
    let wants_directory = (create_options & 0x0000_0001) != 0;
    let wants_non_directory = (create_options & 0x0000_0040) != 0;
    let host_is_directory = std::path::Path::new(&host_path).is_dir()
        || host_path.ends_with('\\')
        || host_path.ends_with('/');
    let is_directory = if wants_directory {
        true
    } else if wants_non_directory {
        false
    } else {
        host_is_directory
    };

    let host_norm = host_path.replace('\\', "/");
    let host_norm_trimmed = host_norm.trim_end_matches('/');
    let data_root = format!(
        "{}/data",
        fs.xbe_directory.trim_end_matches(&['/', '\\'][..])
    )
    .replace('\\', "/");
    let is_spiderman_data_root = is_spiderman_xbe_dir(&fs.xbe_directory)
        && (host_norm_trimmed.eq_ignore_ascii_case("./games/spiderman/data")
            || host_norm_trimmed.eq_ignore_ascii_case(data_root.trim_end_matches('/')));
    let spiderman_data_file_sink = wants_non_directory
        && host_is_directory
        && is_spiderman_data_root
        && matches!(create_disp, 2 | 3 | 5);
    if spiderman_data_file_sink {
        static SPIDEY_DATA_SINK_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = SPIDEY_DATA_SINK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 64 || n.is_power_of_two() {
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-DATA-SINK-PROBE #{}] xbox='{}' host='{}' desired=0x{:08X} attrs=0x{:08X} share=0x{:08X} disp={} opts=0x{:08X} wants_dir={} wants_non_dir={} host_is_dir={} pHandle=0x{:08X} objAttr=0x{:08X} ioStatus=0x{:08X}",
                n,
                xbox_path,
                host_path,
                desired_access,
                file_attribs,
                share_access,
                create_disp,
                create_options,
                wants_directory,
                wants_non_directory,
                host_is_directory,
                p_handle,
                obj_attr,
                io_status_block
            ));
        }

        if std::env::var_os("RUSTEMU_SPIDEY_DATA_ROOT_STRICT").is_some() {
            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile('{}') -> FAIL (SpideyDataRootStrict: FileIsDirectory) host='{}' create_options=0x{:X}",
                xbox_path, host_path, create_options
            ));
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0xC000_00BA);
                memory.write_u32(io_status_block + 4, 0);
            }
            return 0xC000_00BA; // STATUS_FILE_IS_A_DIRECTORY
        }
    }

    if wants_non_directory && host_is_directory && !spiderman_data_file_sink {
        crate::xbox::emulator::debug_log(&format!(
            "NtCreateFile('{}') -> FAIL (FileIsDirectory) host='{}' create_options=0x{:X}",
            xbox_path, host_path, create_options
        ));
        if io_status_block != 0 {
            memory.write_u32(io_status_block, 0xC000_00BA);
            memory.write_u32(io_status_block + 4, 0);
        }
        return 0xC000_00BA; // STATUS_FILE_IS_A_DIRECTORY
    }

    // Auto-create save/config directories (TDATA, UDATA) before open
    let is_save_path = {
        let lp = host_path.to_lowercase();
        lp.contains("/tdata") || lp.contains("/udata")
    };
    let create_may_create = matches!(create_disp, 0 | 2 | 3 | 5);
    let has_write_intent =
        (desired_access & 0x4000_0000) != 0 || (desired_access & 0x0001_0116) != 0;
    let is_directory = if is_save_path
        && is_directory
        && !std::path::Path::new(&host_path).exists()
        && create_may_create
    {
        // For save directory paths that don't exist, create as directory
        let _ = std::fs::create_dir_all(&host_path);
        crate::xbox::emulator::debug_log(&format!(
            "NtCreateFile: auto-created save directory '{}'",
            host_path
        ));
        true
    } else if is_save_path
        && !is_directory
        && !std::path::Path::new(&host_path).exists()
        && create_may_create
        && has_write_intent
    {
        // For save file creation paths — ensure parent directory exists.
        // Read-only probes such as U:\...\SaveMeta.xbx must not create junk
        // parent folders; those later get enumerated as bogus save games.
        if let Some(parent) = std::path::Path::new(&host_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        false
    } else {
        is_directory
    };

    // Spider-Man calls CreateDirectoryA("D:\\data\\") once the post-start
    // menu path is active. On an extracted disc layout this is the existing
    // asset root, so returning a hard collision traps the caller in a retry
    // path. Keep normal FILE_CREATE collision semantics everywhere else.
    let spiderman_existing_data_dir =
        matches!(create_disp, 2 | 5) && is_directory && is_spiderman_data_root;
    let spiderman_sink_path = if spiderman_data_file_sink {
        let sink = format!(
            "{}xbox/EmuDisk/partition6/rustemu_spiderman_data_sink.bin",
            fs.system_directory
        )
        .replace('\\', "/");
        if let Some(parent) = std::path::Path::new(&sink).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Some(sink)
    } else {
        None
    };
    let open_host_path = spiderman_sink_path.as_deref().unwrap_or(&host_path);
    let open_is_directory = if spiderman_sink_path.is_some() {
        false
    } else {
        is_directory
    };

    // Spider-Man post-START can request an optional attract/menu demo file
    // that is absent from some extracted disc layouts. A hard not-found sends
    // the loader into a tight D:\data\DEMO\menu1.dem / vshader.key retry loop.
    // Return a small EOF-only file so the game can take its normal "no demo"
    // path and continue building the menu. The scene/XGRAPH corruption that
    // made this unsafe previously came from an unrelated global overwrite.
    if !open_is_directory
        && is_spiderman_xbe_dir(&fs.xbe_directory)
        && !std::path::Path::new(open_host_path).exists()
        && open_host_path
            .replace('\\', "/")
            .eq_ignore_ascii_case(&format!(
                "{}data/DEMO/menu1.dem",
                fs.xbe_directory.replace('\\', "/")
            ))
    {
        let xbox_handle = fs.alloc_synthetic_handle(Vec::new());
        if xbox_handle != 0 {
            if p_handle != 0 {
                memory.write_u32(p_handle, xbox_handle);
            }
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0);
                memory.write_u32(io_status_block + 4, 1);
            }
            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile('{}') -> synthetic empty handle=0x{:08X} host='{}' [missing optional Spider-Man demo]",
                xbox_path, xbox_handle, open_host_path
            ));
            return 0;
        }
    }

    // Open the file — use platform-specific API for directory support.
    // For the extracted Spider-Man data root, downgrade FILE_CREATE to
    // FILE_OPEN after confirming the directory exists.
    let file_result = open_file_or_dir(
        open_host_path,
        if spiderman_existing_data_dir {
            1
        } else {
            create_disp
        },
        open_is_directory,
        desired_access,
    );

    match file_result {
        Ok(file) => {
            let xbox_handle =
                fs.alloc_handle_with_path(file, open_host_path, spiderman_data_file_sink);
            if xbox_handle == 0 {
                crate::xbox::emulator::debug_log(&format!(
                    "NtCreateFile('{}') -> out of handles!",
                    xbox_path
                ));
                if io_status_block != 0 {
                    memory.write_u32(io_status_block, 0xC000_009A);
                    memory.write_u32(io_status_block + 4, 0);
                }
                return 0xC000_009A; // STATUS_INSUFFICIENT_RESOURCES
            }

            if p_handle != 0 {
                memory.write_u32(p_handle, xbox_handle);
            }
            if io_status_block != 0 {
                memory.write_u32(io_status_block, 0);
                memory.write_u32(io_status_block + 4, if create_disp <= 1 { 1 } else { 2 });
            }

            // If this is a directory, pre-enumerate for later NtQueryDirectoryFile.
            // Guarded on is_directory so we don't read_dir() on a regular file handle.
            if open_is_directory {
                fs.register_directory(xbox_handle, open_host_path);
            }

            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile('{}') -> handle=0x{:08X} host='{}'{}{}{}",
                xbox_path,
                xbox_handle,
                open_host_path,
                if open_is_directory { " [dir]" } else { "" },
                if spiderman_existing_data_dir {
                    " [existing-data-dir-open]"
                } else {
                    ""
                },
                if spiderman_data_file_sink {
                    " [spiderman-data-file-sink]"
                } else {
                    ""
                }
            ));
            0 // STATUS_SUCCESS
        }
        Err(e) => {
            let retry_existing_data_dir = matches!(create_disp, 2 | 5)
                && is_directory
                && e.kind() == std::io::ErrorKind::AlreadyExists
                && is_spiderman_xbe_dir(&fs.xbe_directory)
                && host_path
                    .replace('\\', "/")
                    .to_ascii_lowercase()
                    .contains("d:/img/spiderman/data");
            if retry_existing_data_dir {
                if let Ok(file) = open_file_or_dir(&host_path, 1, true, desired_access) {
                    let xbox_handle = fs.alloc_handle_with_path(file, &host_path, false);
                    if xbox_handle != 0 {
                        if p_handle != 0 {
                            memory.write_u32(p_handle, xbox_handle);
                        }
                        if io_status_block != 0 {
                            memory.write_u32(io_status_block, 0);
                            memory.write_u32(io_status_block + 4, 1);
                        }
                        fs.register_directory(xbox_handle, &host_path);
                        crate::xbox::emulator::debug_log(&format!(
                            "NtCreateFile('{}') -> handle=0x{:08X} host='{}' [dir] [existing-data-dir-open-after-collision]",
                            xbox_path, xbox_handle, host_path
                        ));
                        return 0;
                    }
                }
            }

            let status = match e.kind() {
                std::io::ErrorKind::NotFound => 0xC000_0034, // STATUS_OBJECT_NAME_NOT_FOUND
                std::io::ErrorKind::PermissionDenied => 0xC000_0022, // STATUS_ACCESS_DENIED
                std::io::ErrorKind::AlreadyExists => 0xC000_0035, // STATUS_OBJECT_NAME_COLLISION
                _ => 0xC000_0001,                            // STATUS_UNSUCCESSFUL
            };

            crate::xbox::emulator::debug_log(&format!(
                "NtCreateFile('{}') -> FAIL ({:?}) host='{}'",
                xbox_path,
                e.kind(),
                host_path
            ));

            if io_status_block != 0 {
                memory.write_u32(io_status_block, status);
                memory.write_u32(io_status_block + 4, 0);
            }
            status
        }
    }
}

// ============================================================================
// NtReadFile implementation
// ============================================================================
fn nt_read_file(args: &[u32; 12], state: &mut KernelState, memory: &GuestMemory) -> u32 {
    let handle = args[0];
    let event = args[1];
    let apc_routine = args[2];
    let apc_context = args[3];
    let io_status = args[4];
    let buffer = args[5];
    let length = args[6];
    let p_byte_offset = args[7];

    if event != 0 || apc_routine != 0 || apc_context != 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[NtReadFile-ASYNC-ARGS] handle=0x{:08X} event=0x{:08X} apc=0x{:08X} ctx=0x{:08X} iosb=0x{:08X} buf=0x{:08X} len=0x{:X} offptr=0x{:08X}",
            handle, event, apc_routine, apc_context, io_status, buffer, length, p_byte_offset
        ));
    }

    // Diagnostic: dump all 8 args if buffer looks suspicious
    if buffer == 0 || buffer >= 0xF000_0000 {
        crate::xbox::emulator::debug_log(&format!(
            "[NtReadFile-DIAG] buf=0 SUSPICIOUS! all_args=[{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X},{:#X}]",
            args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7],
            args[8], args[9], args[10], args[11]
        ));
    }

    // Sprint 7 fix (audit-2026-04-23.md P2.8): Reject NULL-buffer reads.
    // Spider-Man's font loader calls NtReadFile with Buffer=NULL for the
    // .FON file (chis22.fon). Silently succeeding with bytes_read=N discards
    // the actual data to /dev/null, leaving the font parser to read the
    // uninitialized guest buffer and assert "Bad token in font file."
    // Real NT kernels return STATUS_ACCESS_VIOLATION for NULL buffer; the
    // font loader's error path should then re-issue the call with a proper
    // heap-allocated buffer.
    if buffer == 0 && length > 0 {
        crate::xbox::emulator::debug_log(&format!(
            "NtReadFile: handle=0x{:08X} len=0x{:X} Buffer=NULL → STATUS_ACCESS_VIOLATION",
            handle, length
        ));
        if io_status != 0 {
            memory.write_u32(io_status, 0xC000_0005);
            memory.write_u32(io_status + 4, 0);
        }
        return 0xC000_0005;
    }

    let fs = match state.file_state.as_mut() {
        Some(fs) => fs,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0008);
                memory.write_u32(io_status + 4, 0);
            }
            return 0xC000_0008;
        }
    };

    if let Some(synth) = fs.synthetic_handle_mut(handle) {
        let file_offset = if valid_guest_addr(p_byte_offset) {
            let lo = memory.read_u32(p_byte_offset) as u64;
            let hi = memory.read_u32(p_byte_offset + 4) as u64;
            (lo | (hi << 32)) as usize
        } else {
            synth.cursor
        };
        let avail = synth.data.len().saturating_sub(file_offset);
        let actual = (length as usize).min(avail);
        if buffer != 0 && actual > 0 {
            memory.copy_into(buffer, &synth.data[file_offset..file_offset + actual]);
        }
        synth.cursor = file_offset.saturating_add(actual);
        if io_status != 0 {
            memory.write_u32(io_status, 0);
            memory.write_u32(io_status + 4, actual as u32);
        }
        crate::xbox::emulator::debug_log(&format!(
            "NtReadFile: synthetic handle=0x{:08X} offset=0x{:X} len=0x{:X} read={} buf=0x{:08X}",
            handle, file_offset, length, actual, buffer
        ));
        return 0;
    }

    // Block partition handles: serve reads from HDD image at partition offset.
    if fs.is_block_handle(handle) {
        use std::io::{Read, Seek, SeekFrom};
        let file_offset = if valid_guest_addr(p_byte_offset) {
            let lo = memory.read_u32(p_byte_offset) as u64;
            let hi = memory.read_u32(p_byte_offset + 4) as u64;
            lo | (hi << 32)
        } else if let Some(bp) = fs.get_block_partition(handle) {
            bp.cursor
        } else {
            0
        };
        if let Some(bp) = fs.get_block_partition(handle) {
            let abs_offset = bp.base_offset + file_offset;
            let avail = if file_offset < bp.size {
                bp.size - file_offset
            } else {
                0
            };
            let read_len = (length as u64).min(avail) as usize;
            let mut buf = vec![0u8; read_len];
            let actual = if bp.file.seek(SeekFrom::Start(abs_offset)).is_ok() {
                bp.file.read(&mut buf).unwrap_or(0)
            } else {
                0
            };
            if buffer != 0 && actual > 0 {
                memory.copy_into(buffer, &buf[..actual]);
            }
            bp.cursor = file_offset + actual as u64;
            if io_status != 0 {
                memory.write_u32(io_status, 0);
                memory.write_u32(io_status + 4, actual as u32);
            }
            return 0;
        }
    }

    // Dummy handles: partition0 reads.
    // Serve full partition0.bin data (524KB Cxbx-R format) for any offset.
    if fs.is_dummy_handle(handle) {
        let p0_data = get_partition0_image();

        let file_offset = if valid_guest_addr(p_byte_offset) {
            let lo = memory.read_u32(p_byte_offset) as u64;
            let hi = memory.read_u32(p_byte_offset + 4) as u64;
            (lo | (hi << 32)) as usize
        } else {
            0usize
        };

        let read_len = length as usize;

        if file_offset >= p0_data.len() {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0011u32); // STATUS_END_OF_FILE
                memory.write_u32(io_status + 4, 0);
            }
            crate::xbox::emulator::debug_log(&format!(
                "NtReadFile: partition0 offset=0x{:X} len=0x{:X} → STATUS_END_OF_FILE (past image)",
                file_offset, length
            ));
            return 0xC000_0011u32; // STATUS_END_OF_FILE
        }

        // Serve data from partition0.bin
        let avail = p0_data.len() - file_offset;
        let actual_read = read_len.min(avail);
        if buffer != 0 && actual_read > 0 {
            memory.copy_into(buffer, &p0_data[file_offset..file_offset + actual_read]);
        }
        if io_status != 0 {
            memory.write_u32(io_status, 0); // STATUS_SUCCESS
            memory.write_u32(io_status + 4, actual_read as u32);
        }
        crate::xbox::emulator::debug_log(&format!(
            "NtReadFile: partition0 offset=0x{:X} len=0x{:X} read={} buf=0x{:08X}",
            file_offset, length, actual_read, buffer
        ));
        return 0;
    }

    use std::io::{Read, Seek, SeekFrom};

    // Read into a temporary buffer, then copy to guest memory
    let read_len = length.min(16 * 1024 * 1024) as usize; // Cap at 16MB
    let mut tmp = vec![0u8; read_len];
    let trace_host_path = fs.handle_path(handle).map(str::to_string);
    let mut trace_file_start = 0u64;
    let read_result = {
        let file = match fs.lookup_handle(handle) {
            Some(f) => f,
            None => {
                crate::xbox::emulator::debug_log(&format!(
                    "NtReadFile: bad handle 0x{:08X}",
                    handle
                ));
                if io_status != 0 {
                    memory.write_u32(io_status, 0xC000_0008);
                    memory.write_u32(io_status + 4, 0);
                }
                return 0xC000_0008;
            }
        };

        // Seek if byte offset provided
        if valid_guest_addr(p_byte_offset) {
            let offset_lo = memory.read_u32(p_byte_offset) as u64;
            let offset_hi = memory.read_u32(p_byte_offset + 4) as u64;
            let offset = offset_lo | (offset_hi << 32);
            trace_file_start = offset;
            let _ = file.seek(SeekFrom::Start(offset));
        } else {
            trace_file_start = file.stream_position().unwrap_or(0);
        }

        file.read(&mut tmp)
    };

    match read_result {
        Ok(bytes_read) => {
            // Copy to guest memory
            if buffer != 0 && bytes_read > 0 {
                memory.copy_into(buffer, &tmp[..bytes_read]);
            }
            if io_status != 0 {
                memory.write_u32(io_status, 0);
                memory.write_u32(io_status + 4, bytes_read as u32);
            }
            if let Some(host_path) = trace_host_path.as_deref() {
                crate::xbox::emulator::debug_log(&format!(
                    "NtReadFile(0x{:08X}, buf=0x{:08X}, len=0x{:X}) -> {} bytes host='{}'",
                    handle, buffer, length, bytes_read, host_path
                ));
                let normalized_host_path = host_path.replace('\\', "/").to_ascii_lowercase();
                if normalized_host_path.ends_with("/data/loadtex/chis22.fon")
                    || normalized_host_path.ends_with("/data/vshader.key")
                {
                    let sample_len = bytes_read.min(48);
                    let sample = &tmp[..sample_len];
                    let sample_hex = sample
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let sample_ascii: String = sample
                        .iter()
                        .map(|&b| {
                            if (0x20..0x7F).contains(&b) {
                                b as char
                            } else {
                                '.'
                            }
                        })
                        .collect();
                    let io_status_status = if io_status != 0 {
                        memory.read_u32(io_status)
                    } else {
                        0xFFFF_FFFF
                    };
                    let io_status_info = if io_status != 0 {
                        memory.read_u32(io_status + 4)
                    } else {
                        0
                    };
                    crate::xbox::emulator::debug_log(&format!(
                        "[SPIDEY-FILE-READ-TRACE] host='{}' handle=0x{:08X} off=0x{:X} buf=0x{:08X} len=0x{:X} read={} iosb=0x{:08X} iosb.status=0x{:08X} iosb.info=0x{:X} sample_hex=[{}] sample_ascii='{}'",
                        host_path,
                        handle,
                        trace_file_start,
                        buffer,
                        length,
                        bytes_read,
                        io_status,
                        io_status_status,
                        io_status_info,
                        sample_hex,
                        sample_ascii
                    ));
                }
                let is_spidey_training_asset = normalized_host_path
                    .ends_with("/data/levels/origin_z.xbs")
                    || normalized_host_path.ends_with("/data/heroes/peterstu.xbs");
                if bytes_read > 0 && normalized_host_path.ends_with("/data/vshader.key") {
                    crate::xbox::emulator::mark_spidey_post_peterstu_vshader_key_read(
                        host_path, bytes_read,
                    );
                }
                if is_spidey_training_asset || event != 0 || apc_routine != 0 || apc_context != 0 {
                    crate::xbox::emulator::debug_log(&format!(
                        "[NtReadFile-TARGET-ARGS] host='{}' handle=0x{:08X} event=0x{:08X} apc=0x{:08X} ctx=0x{:08X} iosb=0x{:08X} buf=0x{:08X} len=0x{:X} off=0x{:X} read={}",
                        host_path,
                        handle,
                        event,
                        apc_routine,
                        apc_context,
                        io_status,
                        buffer,
                        length,
                        trace_file_start,
                        bytes_read
                    ));
                }
                if host_path
                    .replace('\\', "/")
                    .to_ascii_lowercase()
                    .ends_with("/data/images/loadscrn_x.img.xbx")
                {
                    let sample_len = bytes_read.min(32);
                    let sample = tmp[..sample_len]
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    crate::xbox::emulator::debug_log(&format!(
                        "[TONY-LOADSCRN-READ] off=0x{:X} buf=0x{:08X} len=0x{:X} read={} io=0x{:08X} sample={}",
                        trace_file_start, buffer, length, bytes_read, io_status, sample
                    ));
                }
                spidey_peterstu_trace_read(
                    host_path,
                    trace_file_start,
                    buffer,
                    length,
                    &tmp[..bytes_read],
                );
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "NtReadFile(0x{:08X}, buf=0x{:08X}, len=0x{:X}) -> {} bytes",
                    handle, buffer, length, bytes_read
                ));
            }
            if let Some((host_path, total_read)) =
                fs.record_peterstu_read_progress(handle, bytes_read)
            {
                spidey_try_peterstu_ready_poke(memory, &host_path, total_read);
            }
            0
        }
        Err(_) => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0011);
                memory.write_u32(io_status + 4, 0);
            }
            crate::xbox::emulator::debug_log(&format!(
                "NtReadFile(0x{:08X}, buf=0x{:08X}, len=0x{:X}) -> read error (host file read failed)",
                handle, buffer, length
            ));
            0xC000_0011
        }
    }
}

// ============================================================================
// NtWriteFile implementation
// ============================================================================
fn nt_write_file(args: &[u32; 12], state: &mut KernelState, memory: &GuestMemory) -> u32 {
    let handle = args[0];
    let _event = args[1];
    let _apc_routine = args[2];
    let _apc_context = args[3];
    let io_status = args[4];
    let buffer = args[5];
    let length = args[6];
    let p_byte_offset = args[7];

    // Sprint 7 fix (agent S9-A23 audit): Mirror the NtReadFile NULL-buffer
    // protection for writes. Previously NtWriteFile would accept Buffer=NULL
    // and silently write zeros (read_u8 on NULL returns 0, loop produces a
    // length-N run of zero bytes). Real NT returns STATUS_ACCESS_VIOLATION
    // for NULL user-mode Buffer. Catching this at entry avoids HDD image
    // corruption from a buggy game or an emulator bug that zeroes args.
    if buffer == 0 && length > 0 {
        crate::xbox::emulator::debug_log(&format!(
            "NtWriteFile: handle=0x{:08X} len=0x{:X} Buffer=NULL → STATUS_ACCESS_VIOLATION",
            handle, length
        ));
        if io_status != 0 {
            memory.write_u32(io_status, 0xC000_0005);
            memory.write_u32(io_status + 4, 0);
        }
        return 0xC000_0005;
    }

    let fs = match state.file_state.as_mut() {
        Some(fs) => fs,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0008);
                memory.write_u32(io_status + 4, 0);
            }
            return 0xC000_0008;
        }
    };

    let is_spidey_data_sink = fs.is_spidey_data_sink_handle(handle);
    let handle_path = fs.handle_path(handle).unwrap_or("").to_string();
    if is_spidey_data_sink {
        static SPIDEY_DATA_WRITE_LOG: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        let n = SPIDEY_DATA_WRITE_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 64 || n.is_power_of_two() {
            let byte_offset = if valid_guest_addr(p_byte_offset) {
                let lo = memory.read_u32(p_byte_offset) as u64;
                let hi = memory.read_u32(p_byte_offset + 4) as u64;
                Some(lo | (hi << 32))
            } else {
                None
            };
            let sample_len = length.min(64) as usize;
            let mut bytes = Vec::with_capacity(sample_len);
            if buffer != 0 {
                for i in 0..sample_len {
                    bytes.push(memory.read_u8(buffer + i as u32));
                }
            }
            let hex = bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let ascii = bytes
                .iter()
                .map(|&b| {
                    if (0x20..=0x7E).contains(&b) {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-DATA-SINK-WRITE #{}] handle=0x{:08X} path='{}' io=0x{:08X} buf=0x{:08X} len=0x{:X} off={} sample_hex='{}' sample_ascii='{}'",
                n,
                handle,
                handle_path,
                io_status,
                buffer,
                length,
                byte_offset
                    .map(|o| format!("0x{:X}", o))
                    .unwrap_or_else(|| "cursor".to_string()),
                hex,
                ascii
            ));
        }
    }

    // Block partition handles: write to HDD image at partition offset.
    if fs.is_block_handle(handle) {
        use std::io::{Seek, SeekFrom, Write};
        let file_offset = if valid_guest_addr(p_byte_offset) {
            let lo = memory.read_u32(p_byte_offset) as u64;
            let hi = memory.read_u32(p_byte_offset + 4) as u64;
            lo | (hi << 32)
        } else if let Some(bp) = fs.get_block_partition(handle) {
            bp.cursor
        } else {
            0
        };
        if let Some(bp) = fs.get_block_partition(handle) {
            let abs_offset = bp.base_offset + file_offset;
            let write_len = (length as u64).min(bp.size - file_offset.min(bp.size)) as usize;
            let mut tmp = vec![0u8; write_len];
            for i in 0..write_len {
                tmp[i] = memory.read_u8(buffer + i as u32);
            }
            let actual = if bp.file.seek(SeekFrom::Start(abs_offset)).is_ok() {
                bp.file.write(&tmp).unwrap_or(0)
            } else {
                0
            };
            bp.cursor = file_offset + actual as u64;
            if io_status != 0 {
                memory.write_u32(io_status, 0);
                memory.write_u32(io_status + 4, actual as u32);
            }
            return 0;
        }
    }

    let file = match fs.lookup_handle(handle) {
        Some(f) => f,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0008);
                memory.write_u32(io_status + 4, 0);
            }
            return 0xC000_0008;
        }
    };

    use std::io::{Seek, SeekFrom, Write};

    if valid_guest_addr(p_byte_offset) {
        let offset_lo = memory.read_u32(p_byte_offset) as u64;
        let offset_hi = memory.read_u32(p_byte_offset + 4) as u64;
        let offset = offset_lo | (offset_hi << 32);
        let _ = file.seek(SeekFrom::Start(offset));
    }

    // Read from guest memory into temp buffer, then write
    let write_len = length.min(16 * 1024 * 1024) as usize;
    let mut tmp = vec![0u8; write_len];
    for i in 0..write_len {
        tmp[i] = memory.read_u8(buffer + i as u32);
    }

    match file.write(&tmp) {
        Ok(bytes_written) => {
            if io_status != 0 {
                memory.write_u32(io_status, 0);
                memory.write_u32(io_status + 4, bytes_written as u32);
            }
            0
        }
        Err(_) => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0001);
                memory.write_u32(io_status + 4, 0);
            }
            0xC000_0001
        }
    }
}

// ============================================================================
// NtQueryInformationFile implementation
// ============================================================================
fn nt_query_information_file(
    args: &[u32; 12],
    state: &mut KernelState,
    memory: &GuestMemory,
) -> u32 {
    let handle = args[0];
    let io_status = args[1];
    let file_info = args[2];
    let length = args[3];
    let info_class = args[4];

    let fs = match state.file_state.as_mut() {
        Some(fs) => fs,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0008);
                memory.write_u32(io_status + 4, 0);
            }
            return 0xC000_0008;
        }
    };

    // Dummy handles (virtual partitions): return directory-like info
    if fs.is_dummy_handle(handle) {
        if file_info != 0 && length >= 22 {
            memory.zero_fill(file_info, (length.min(64)) as usize);
        }
        if io_status != 0 {
            memory.write_u32(io_status, 0); // STATUS_SUCCESS
            memory.write_u32(io_status + 4, 0);
        }
        return 0;
    }

    // In-memory synthetic files should behave like ordinary host files for
    // size/attribute queries. Spider-Man's post-START menu path queries the
    // optional DEMO\menu1.dem handle before reading it; returning bad-handle
    // here traps the loader in a retry loop.
    if let Some(synth) = fs.synthetic_handle(handle) {
        let file_size = synth.data.len() as u64;
        let alloc = if file_size == 0 {
            0
        } else {
            (file_size + 0xFFF) & !0xFFF
        };
        let mut status = 0u32;
        let mut info_written = 0u32;

        match info_class {
            5 => {
                // FileStandardInformation
                if length >= 22 && file_info != 0 {
                    memory.zero_fill(file_info, 22);
                    memory.write_u32(file_info + 0, alloc as u32);
                    memory.write_u32(file_info + 4, (alloc >> 32) as u32);
                    memory.write_u32(file_info + 8, file_size as u32);
                    memory.write_u32(file_info + 12, (file_size >> 32) as u32);
                    memory.write_u32(file_info + 16, 1);
                    info_written = 22;
                } else {
                    status = 0xC000_0023;
                }
            }
            14 => {
                // FilePositionInformation
                if length >= 8 && file_info != 0 {
                    memory.zero_fill(file_info, 8);
                    memory.write_u32(file_info, synth.cursor as u32);
                    info_written = 8;
                } else {
                    status = 0xC000_0023;
                }
            }
            4 => {
                // FileBasicInformation
                if length >= 36 && file_info != 0 {
                    memory.zero_fill(file_info, 36);
                    memory.write_u32(file_info + 32, 0x80);
                    info_written = 36;
                } else {
                    status = 0xC000_0023;
                }
            }
            34 => {
                // FileNetworkOpenInformation
                if length >= 56 && file_info != 0 {
                    memory.zero_fill(file_info, 56);
                    memory.write_u32(file_info + 32, alloc as u32);
                    memory.write_u32(file_info + 36, (alloc >> 32) as u32);
                    memory.write_u32(file_info + 40, file_size as u32);
                    memory.write_u32(file_info + 44, (file_size >> 32) as u32);
                    memory.write_u32(file_info + 48, 0x80);
                    info_written = 56;
                } else {
                    status = 0xC000_0023;
                }
            }
            _ => {
                crate::xbox::emulator::debug_log(&format!(
                    "NtQueryInformationFile: synthetic handle=0x{:08X} unhandled class {}",
                    handle, info_class
                ));
                status = 0xC000_0003;
            }
        }

        if io_status != 0 {
            memory.write_u32(io_status, status);
            memory.write_u32(io_status + 4, info_written);
        }
        crate::xbox::emulator::debug_log(&format!(
            "NtQueryInformationFile: synthetic handle=0x{:08X} class={} size={} status=0x{:08X}",
            handle, info_class, file_size, status
        ));
        return status;
    }

    let trace_host_path = fs.handle_path(handle).map(str::to_string);
    let file = match fs.lookup_handle(handle) {
        Some(f) => f,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0xC000_0008);
                memory.write_u32(io_status + 4, 0);
            }
            return 0xC000_0008;
        }
    };

    use std::io::{Seek, SeekFrom};

    let mut status = 0u32;
    let mut info_written = 0u32;

    let trace_file_size = file.metadata().ok().map(|m| m.len()).unwrap_or(0);

    match info_class {
        5 => {
            // FileStandardInformation — file size (22 bytes)
            if length >= 22 && file_info != 0 {
                memory.zero_fill(file_info, 22);
                if let Ok(metadata) = file.metadata() {
                    let file_size = metadata.len();
                    let alloc = (file_size + 0xFFF) & !0xFFF;
                    memory.write_u32(file_info + 0, alloc as u32);
                    memory.write_u32(file_info + 4, (alloc >> 32) as u32);
                    memory.write_u32(file_info + 8, file_size as u32);
                    memory.write_u32(file_info + 12, (file_size >> 32) as u32);
                    memory.write_u32(file_info + 16, 1); // NumberOfLinks
                    info_written = 22;
                } else {
                    status = 0xC000_0001;
                }
            } else {
                status = 0xC000_0023;
            }
        }
        14 => {
            // FilePositionInformation (8 bytes)
            if length >= 8 && file_info != 0 {
                let pos = file.seek(SeekFrom::Current(0)).unwrap_or(0);
                memory.write_u32(file_info, pos as u32);
                memory.write_u32(file_info + 4, (pos >> 32) as u32);
                info_written = 8;
            } else {
                status = 0xC000_0023;
            }
        }
        4 => {
            // FileBasicInformation (36 bytes)
            if length >= 36 && file_info != 0 {
                memory.zero_fill(file_info, 36);
                // Just provide zeroed timestamps + normal file attributes
                memory.write_u32(file_info + 32, 0x80); // FILE_ATTRIBUTE_NORMAL
                info_written = 36;
            } else {
                status = 0xC000_0023;
            }
        }
        34 => {
            // FileNetworkOpenInformation (56 bytes)
            if length >= 56 && file_info != 0 {
                memory.zero_fill(file_info, 56);
                if let Ok(metadata) = file.metadata() {
                    let file_size = metadata.len();
                    let alloc = (file_size + 0xFFF) & !0xFFF;
                    memory.write_u32(file_info + 32, alloc as u32);
                    memory.write_u32(file_info + 36, (alloc >> 32) as u32);
                    memory.write_u32(file_info + 40, file_size as u32);
                    memory.write_u32(file_info + 44, (file_size >> 32) as u32);
                    memory.write_u32(file_info + 48, 0x80); // attributes
                }
                info_written = 56;
            } else {
                status = 0xC000_0023;
            }
        }
        _ => {
            crate::xbox::emulator::debug_log(&format!(
                "NtQueryInformationFile: unhandled class {}",
                info_class
            ));
            status = 0xC000_0003; // STATUS_INVALID_INFO_CLASS
        }
    }

    if io_status != 0 {
        memory.write_u32(io_status, status);
        memory.write_u32(io_status + 4, info_written);
    }
    if let Some(host_path) = trace_host_path.as_deref() {
        let normalized_host_path = host_path.replace('\\', "/").to_ascii_lowercase();
        if normalized_host_path.ends_with("/data/loadtex/chis22.fon")
            || normalized_host_path.ends_with("/data/vshader.key")
        {
            let mut file_info_words = String::new();
            if file_info != 0 {
                for i in 0..8u32 {
                    if i != 0 {
                        file_info_words.push(' ');
                    }
                    file_info_words.push_str(&format!(
                        "{:08X}",
                        memory.read_u32(file_info.wrapping_add(i * 4))
                    ));
                }
            }
            crate::xbox::emulator::debug_log(&format!(
                "[SPIDEY-FILE-QUERY-TRACE] host='{}' handle=0x{:08X} class={} len=0x{:X} file_info=0x{:08X} status=0x{:08X} info_written={} size=0x{:X} words=[{}]",
                host_path,
                handle,
                info_class,
                length,
                file_info,
                status,
                info_written,
                trace_file_size,
                file_info_words
            ));
        }
    }
    status
}

// ============================================================================
// NtSetInformationFile implementation
// ============================================================================
fn nt_set_information_file(args: &[u32; 12], state: &mut KernelState, memory: &GuestMemory) -> u32 {
    let handle = args[0];
    let io_status = args[1];
    let file_info = args[2];
    let length = args[3];
    let info_class = args[4];

    let fs = match state.file_state.as_mut() {
        Some(fs) => fs,
        None => {
            if io_status != 0 {
                memory.write_u32(io_status, 0);
                memory.write_u32(io_status + 4, 0);
            }
            return 0;
        }
    };

    if info_class == 14 {
        // FilePositionInformation — set file pointer
        if let Some(file) = fs.lookup_handle(handle) {
            if file_info != 0 && length >= 8 {
                use std::io::{Seek, SeekFrom};
                let pos_lo = memory.read_u32(file_info) as u64;
                let pos_hi = memory.read_u32(file_info + 4) as u64;
                let pos = pos_lo | (pos_hi << 32);
                // Diagnostic: log every seek with handle + position so we can
                // verify decompression-archive offsets match the file layout.
                // Rate-limited to first 40 seeks to avoid log flood.
                static SEEK_LOG: std::sync::atomic::AtomicU32 =
                    std::sync::atomic::AtomicU32::new(0);
                let n = SEEK_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n < 40 {
                    let actual = file.seek(SeekFrom::Start(pos)).unwrap_or(u64::MAX);
                    crate::xbox::emulator::debug_log(&format!(
                        "[FILE-SEEK] #{} handle=0x{:08X} requested_pos=0x{:X} actual_pos=0x{:X}",
                        n, handle, pos, actual
                    ));
                } else {
                    let _ = file.seek(SeekFrom::Start(pos));
                }
            }
        }
    }

    if io_status != 0 {
        memory.write_u32(io_status, 0);
        memory.write_u32(io_status + 4, 0);
    }
    0
}

// ============================================================================
// Platform-specific file open (supports directories on Windows)
// ============================================================================

/// Open a file or directory. On Windows, directories require FILE_FLAG_BACKUP_SEMANTICS.
#[cfg(windows)]
fn open_file_or_dir(
    host_path: &str,
    create_disp: u32,
    is_directory: bool,
    desired_access: u32,
) -> Result<std::fs::File, std::io::Error> {
    use std::os::windows::ffi::OsStrExt;

    // Map Xbox NT create disposition to Win32
    let win32_disp: u32 = match create_disp {
        0 => 1, // FILE_SUPERSEDE → CREATE_NEW
        1 => 3, // FILE_OPEN → OPEN_EXISTING
        2 => 2, // FILE_CREATE → CREATE_ALWAYS
        3 => 4, // FILE_OPEN_IF → OPEN_ALWAYS
        4 => 5, // FILE_OVERWRITE → TRUNCATE_EXISTING
        5 => 2, // FILE_OVERWRITE_IF → CREATE_ALWAYS
        _ => 3, // default OPEN_EXISTING
    };

    // Map Xbox desired access to Win32
    // Directories only need read access regardless of what the game requests
    // Do not treat SYNCHRONIZE (0x0010_0000) or READ_CONTROL
    // (0x0002_0000) as write access. Spider-Man opens read-only disc assets
    // with GENERIC_READ | SYNCHRONIZE | FILE_READ_ATTRIBUTES
    // (0x8010_0080); requesting GENERIC_WRITE on the host makes those valid
    // Xbox reads fail with AccessDenied on ordinary extracted files.
    let has_write = !is_directory
        && ((desired_access & 0x4000_0000) != 0  // GENERIC_WRITE
        || (desired_access & 0x0001_0116) != 0); // DELETE | FILE_WRITE_* bits
    let win32_access: u32 = if has_write {
        0xC000_0000 | 0x0000_0080 // GENERIC_READ | GENERIC_WRITE | FILE_READ_ATTRIBUTES
    } else {
        0x8000_0000 | 0x0000_0080 // GENERIC_READ | FILE_READ_ATTRIBUTES
    };
    let win32_share: u32 = 0x1 | 0x2 | 0x4; // FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE
    let win32_flags: u32 = if is_directory { 0x0200_0000 } else { 0x80 }; // FILE_FLAG_BACKUP_SEMANTICS or FILE_ATTRIBUTE_NORMAL

    // Use wide string for CreateFileW
    let wide_path: Vec<u16> = std::ffi::OsStr::new(host_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let handle = unsafe {
        windows::Win32::Storage::FileSystem::CreateFileW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            win32_access,
            windows::Win32::Storage::FileSystem::FILE_SHARE_MODE(win32_share),
            None,
            windows::Win32::Storage::FileSystem::FILE_CREATION_DISPOSITION(win32_disp),
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(win32_flags),
            None,
        )
    };

    match handle {
        Ok(h) => {
            if h.is_invalid() {
                let err = std::io::Error::last_os_error();
                crate::xbox::emulator::debug_log(&format!(
                    "open_file_or_dir: Ok but INVALID_HANDLE: {:?}",
                    err
                ));
                Err(err)
            } else {
                Ok(unsafe { std::fs::File::from_raw_handle(h.0 as *mut std::ffi::c_void) })
            }
        }
        Err(e) => {
            crate::xbox::emulator::debug_log(&format!(
                "open_file_or_dir: CreateFileW failed: '{}' disp={} dir={} access=0x{:X} err={:?} code=0x{:X}",
                host_path, win32_disp, is_directory, win32_access, e.message(), e.code().0
            ));
            // Retry with FILE_FLAG_BACKUP_SEMANTICS if not already set
            if !is_directory {
                let retry_flags: u32 = 0x0200_0000; // FILE_FLAG_BACKUP_SEMANTICS
                let retry = unsafe {
                    windows::Win32::Storage::FileSystem::CreateFileW(
                        windows::core::PCWSTR(wide_path.as_ptr()),
                        win32_access,
                        windows::Win32::Storage::FileSystem::FILE_SHARE_MODE(win32_share),
                        None,
                        windows::Win32::Storage::FileSystem::FILE_CREATION_DISPOSITION(win32_disp),
                        windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(retry_flags),
                        None,
                    )
                };
                match retry {
                    Ok(h) if !h.is_invalid() => {
                        return Ok(unsafe {
                            std::fs::File::from_raw_handle(h.0 as *mut std::ffi::c_void)
                        });
                    }
                    _ => {}
                }
            }
            // Convert HRESULT to Win32 error code
            let win32_err = e.code().0 & 0xFFFF;
            Err(std::io::Error::from_raw_os_error(win32_err as i32))
        }
    }
}

#[cfg(not(windows))]
fn open_file_or_dir(
    host_path: &str,
    create_disp: u32,
    _is_directory: bool,
    _desired_access: u32,
) -> Result<std::fs::File, std::io::Error> {
    use std::fs::OpenOptions;
    match create_disp {
        1 => OpenOptions::new().read(true).open(host_path),
        3 => OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(host_path),
        _ => OpenOptions::new().read(true).open(host_path),
    }
}

// ============================================================================
// Synthetic Xbox HDD partition table (partition0)
// ============================================================================

/// Cached partition0 image (524KB Cxbx-R format), wrapped in Arc so
/// callers share the backing store. Prior code returned `Vec<u8>` and
/// cloned 524KB on every access — at N calls per run that's N × 524KB
/// of wasted memcpy. `Arc<Vec<u8>>` clones the refcount only (8 bytes).
/// Agent S10-A23 audit.
static PARTITION0_IMAGE: std::sync::Mutex<Option<std::sync::Arc<Vec<u8>>>> =
    std::sync::Mutex::new(None);

fn get_partition0_image() -> std::sync::Arc<Vec<u8>> {
    let mut guard = PARTITION0_IMAGE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref data) = *guard {
        return std::sync::Arc::clone(data);
    }

    // Load partition0.bin — proper PARTINFO header + FATX partition data.
    // Priority: 1) RetroArch EmuDisk (pyfatx-generated), 2) Cxbx-R EmuDisk, 3) synthetic fallback.
    // The pyfatx image has clean zeroes at offset 0x800 (no stale game metadata).
    let p0_paths = [
        "./RetroArch-Win64/system/xbox/EmuDisk/partition0.bin".to_string(),
        "./CxbxReloaded/EmuDisk/partition0.bin".to_string(),
    ];
    let mut data: Option<Vec<u8>> = None;
    for p in &p0_paths {
        if let Ok(d) = std::fs::read(p) {
            crate::xbox::emulator::debug_log(&format!(
                "partition0: loaded {} bytes from '{}'",
                d.len(),
                p
            ));
            data = Some(d);
            break;
        }
    }
    let data = match data {
        Some(d) => d,
        None => {
            // Fallback: empty 512KB with PARTINFO magic (will likely crash but at least loads)
            crate::xbox::emulator::debug_log(
                "partition0: WARNING — no .bin found, using empty fallback",
            );
            let mut d = vec![0u8; 524288];
            d[..16].copy_from_slice(b"****PARTINFO****");
            d
        }
    };

    let arc = std::sync::Arc::new(data);
    *guard = Some(std::sync::Arc::clone(&arc));
    arc
}

/// Build the 512-byte Xbox HDD partition table (PARTINFO sector).
/// Uses the Cxbx-R partition0.bin format: 16-byte magic + partition entries
/// with 16-char names, LBA offsets, sizes, and flags.
/// If the Cxbx-R file exists in the system directory, use it directly.
/// Otherwise, generate a minimal table matching Cxbx-R's default layout.
fn build_xbox_partinfo() -> [u8; 512] {
    // Try to load from EmuDisk (pyfatx first, then Cxbx-R fallback)
    let cxbx_paths = [
        "./RetroArch-Win64/system/xbox/EmuDisk/partition0.bin",
        "./CxbxReloaded/EmuDisk/partition0.bin",
    ];
    for path in &cxbx_paths {
        if let Ok(data) = std::fs::read(path) {
            if data.len() >= 512 && &data[..16] == b"****PARTINFO****" {
                let mut buf = [0u8; 512];
                buf.copy_from_slice(&data[..512]);
                crate::xbox::emulator::debug_log(&format!(
                    "partition0: loaded PARTINFO from '{}'",
                    path
                ));
                return buf;
            }
        }
    }

    // Fallback: minimal PARTINFO with just magic (all partitions zeroed)
    // This tells the game "no partitions configured" — it should skip
    // partition-dependent init and continue to D3D setup.
    crate::xbox::emulator::debug_log(
        "partition0: using minimal synthetic PARTINFO (no Cxbx file found)",
    );
    let mut buf = [0u8; 512];
    buf[..16].copy_from_slice(b"****PARTINFO****");
    buf
}
