/// XBE file parser and section mapper.
/// Ports AOT_XBELoader.cpp — parses Xbox executables, maps sections into
/// guest memory, resolves kernel imports via magic address thunks.
use crate::xbox::memory::guest_memory::GuestMemory;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;

/// Guest address where KeTickCount (ordinal 156) data export lives.
/// Updated by retro_run each frame so I_GetTime sees advancing ticks.
pub static KE_TICK_COUNT_ADDR: AtomicU32 = AtomicU32::new(0);

/// Guest address where KeInterruptTime (ordinal 120) data lives (8-byte KSYSTEM_TIME).
/// Updated by retro_run each frame. Doom's I_GetTime reads this, not KeTickCount.
pub static KE_INTERRUPT_TIME_ADDR: AtomicU32 = AtomicU32::new(0);

/// Guest address where KeSystemTime (ordinal 154) data lives (8-byte KSYSTEM_TIME).
/// Updated by retro_run each frame alongside KeInterruptTime.
pub static KE_SYSTEM_TIME_ADDR: AtomicU32 = AtomicU32::new(0);

#[derive(Error, Debug)]
pub enum XbeError {
    #[error("Failed to read XBE file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid XBE magic: expected 0x48454258, got 0x{0:08X}")]
    BadMagic(u32),
    #[error("XBE file too small: {0} bytes (need at least 376 for header)")]
    FileTooSmall(usize),
    #[error("Too many sections: {0} (max {MAX_SECTIONS})")]
    TooManySections(u32),
    #[error("Section '{name}' raw data out of bounds: offset {offset} + size {size} > file size {file_size}")]
    SectionOutOfBounds {
        name: String,
        offset: u32,
        size: u32,
        file_size: u32,
    },
    #[error("Failed to decrypt entry point — no valid candidate found")]
    EntryPointDecryptFailed,
    #[error("Failed to find kernel thunk table")]
    ThunkTableNotFound,
}

pub const XBE_MAGIC: u32 = 0x48454258; // "XBEH"
pub const MAX_SECTIONS: u32 = 64;
pub const KERNEL_MAGIC_BASE: u32 = 0xFFFF0000;

// XBE certificate offset from base address (fixed in XBE format)
const CERT_TITLE_NAME_OFFSET: usize = 0x0C; // Unicode title starts at cert+0x0C
const CERT_TITLE_NAME_LEN: usize = 40; // 40 wide chars (80 bytes)

/// Parsed XBE information returned after successful loading.
#[derive(Debug, Clone)]
pub struct XbeInfo {
    pub entry_point: u32,
    pub base_address: u32,
    pub image_size: u32,
    pub is_retail: bool,
    pub title: String,
    pub title_id: u32,
    pub allowed_media: u32,
    pub game_region: u32,
    pub num_sections: u32,
    pub kernel_thunk_addr: u32,
    pub sections: Vec<XbeSectionInfo>,
    /// D3D8 library build version from XBE library headers (e.g., 4134, 5344, 5849).
    /// Used to select the correct OOVPA patterns. 0 = unknown.
    pub d3d8_build_version: u16,
    /// Whether D3D8 was compiled with LTCG (Link-Time Code Generation).
    /// LTCG changes function prologues/epilogues and calling conventions.
    pub d3d8_is_ltcg: bool,
}

#[derive(Debug, Clone)]
pub struct XbeSectionInfo {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub raw_address: u32,
    pub raw_size: u32,
    pub flags: u32,
    pub executable: bool,
}

/// XBE header layout (all fields little-endian, packed).
/// Matches HarnessXBE_Header in AOT_XBELoader.cpp.
struct XbeHeader {
    magic: u32,
    base_address: u32,
    headers_size: u32,
    image_size: u32,
    cert_address: u32,
    num_sections: u32,
    section_headers_addr: u32,
    entry_point: u32,       // XOR-encrypted
    kernel_thunk_addr: u32, // XOR-encrypted
    lib_versions_addr: u32,
    lib_versions_count: u32,
}

struct XbeSectionHeader {
    flags: u32,
    virtual_address: u32,
    virtual_size: u32,
    raw_address: u32,
    raw_size: u32,
    section_name_addr: u32,
}

impl XbeHeader {
    fn parse(data: &[u8]) -> Result<Self, XbeError> {
        if data.len() < 376 {
            return Err(XbeError::FileTooSmall(data.len()));
        }
        let magic = read_u32(data, 0x000);
        if magic != XBE_MAGIC {
            return Err(XbeError::BadMagic(magic));
        }
        Ok(XbeHeader {
            magic,
            base_address: read_u32(data, 0x104),
            headers_size: read_u32(data, 0x108),
            image_size: read_u32(data, 0x10C),
            cert_address: read_u32(data, 0x118),
            num_sections: read_u32(data, 0x11C),
            section_headers_addr: read_u32(data, 0x120),
            entry_point: read_u32(data, 0x128),
            kernel_thunk_addr: read_u32(data, 0x158),
            lib_versions_count: read_u32(data, 0x160),
            lib_versions_addr: read_u32(data, 0x164),
        })
    }
}

impl XbeSectionHeader {
    fn parse(data: &[u8], offset: usize) -> Self {
        XbeSectionHeader {
            flags: read_u32(data, offset + 0x00),
            virtual_address: read_u32(data, offset + 0x04),
            virtual_size: read_u32(data, offset + 0x08),
            raw_address: read_u32(data, offset + 0x0C),
            raw_size: read_u32(data, offset + 0x10),
            section_name_addr: read_u32(data, offset + 0x14),
        }
    }
}

/// Load an XBE file into guest memory.
pub fn load_xbe(path: &str, memory: &GuestMemory) -> Result<XbeInfo, XbeError> {
    let file_data = std::fs::read(path)?;
    let file_size = file_data.len() as u32;

    // Parse header
    let header = XbeHeader::parse(&file_data)?;
    if header.num_sections > MAX_SECTIONS {
        return Err(XbeError::TooManySections(header.num_sections));
    }

    log::info!(
        "XBE: base=0x{:08X} image_size=0x{:X} sections={} entry(enc)=0x{:08X} thunk(enc)=0x{:08X}",
        header.base_address,
        header.image_size,
        header.num_sections,
        header.entry_point,
        header.kernel_thunk_addr
    );

    // Copy XBE header into guest memory at base_address.
    let headers_size = header.headers_size as usize;
    let copy_size = headers_size.min(file_data.len());
    memory.copy_into(header.base_address, &file_data[..copy_size]);
    log::info!(
        "XBE: copied {} bytes of headers to guest 0x{:08X}",
        copy_size,
        header.base_address
    );

    // DO NOT set init_flags bit 3 ("don't set up hard disk").
    // Bit 3 tells the CRT to skip XapiValidateDiskPartition, but the CRT
    // then takes the "no valid disk" path → XLaunchNewImageA → reboot.
    // Instead, let XapiValidateDiskPartition run — it calls NtOpenFile on
    // \Device\Harddisk0\partition1\ which our file I/O handler returns SUCCESS for.
    // Cxbx-R does NOT set bit 3 and handles partition I/O properly.
    let init_flags_addr = header.base_address + 0x124;
    let flags = memory.read_u32(init_flags_addr);
    log::info!("XBE: init_flags 0x{:08X} (NOT setting bit 3)", flags);

    // Parse section headers
    let sect_hdr_file_offset = (header.section_headers_addr - header.base_address) as usize;
    let mut sections = Vec::with_capacity(header.num_sections as usize);
    let mut has_d3d_section = false;

    for i in 0..header.num_sections {
        let offset = sect_hdr_file_offset + (i as usize) * 56; // sizeof(XbeSectionHeader) = 56 bytes
        let sh = XbeSectionHeader::parse(&file_data, offset);

        // Read section name from file (name_addr is a VA, convert to file offset)
        let name = read_section_name(&file_data, sh.section_name_addr, header.base_address);
        let executable = (sh.flags & 0x00000004) != 0; // IMAGE_SCN_MEM_EXECUTE

        if name.contains("D3D") {
            has_d3d_section = true;
        }

        // Validate raw data bounds
        if sh.raw_size > 0 {
            let end = sh.raw_address.checked_add(sh.raw_size).unwrap_or(u32::MAX);
            if end > file_size {
                return Err(XbeError::SectionOutOfBounds {
                    name: name.clone(),
                    offset: sh.raw_address,
                    size: sh.raw_size,
                    file_size,
                });
            }
        }

        // Map section into guest memory
        if sh.raw_size > 0 {
            let raw_start = sh.raw_address as usize;
            let raw_end = raw_start + sh.raw_size as usize;
            memory.copy_into(sh.virtual_address, &file_data[raw_start..raw_end]);
        }

        // Zero-fill remaining virtual size
        if sh.virtual_size > sh.raw_size {
            let zero_start = sh.virtual_address + sh.raw_size;
            let zero_len = (sh.virtual_size - sh.raw_size) as usize;
            memory.zero_fill(zero_start, zero_len);
        }

        log::info!(
            "  Section {}: '{}' VA=0x{:08X} vsize=0x{:X} raw=0x{:X} {}",
            i,
            name,
            sh.virtual_address,
            sh.virtual_size,
            sh.raw_size,
            if executable { "[EXEC]" } else { "" }
        );

        sections.push(XbeSectionInfo {
            name,
            virtual_address: sh.virtual_address,
            virtual_size: sh.virtual_size,
            raw_address: sh.raw_address,
            raw_size: sh.raw_size,
            flags: sh.flags,
            executable,
        });

        // Set section ref_count to 1 in guest memory — tells the game the section
        // is already loaded. Without this, XeLoadSection callers think sections are
        // unloaded and the game's string pool / asset systems fail to initialize.
        // Section header layout: ref_count at offset +0x18 within the 56-byte header.
        let hdr_guest_addr = header.section_headers_addr + (i as u32) * 56;
        memory.write_u32(hdr_guest_addr + 0x18, 1); // ref_count = 1 (loaded)
        memory.write_u16(hdr_guest_addr + 0x1C, 1); // head_shared_page_ref
        memory.write_u16(hdr_guest_addr + 0x1E, 1); // tail_shared_page_ref
    }

    // Decrypt entry point and kernel thunk address
    let (entry_point, kernel_thunk_addr) = decrypt_xbe_addresses(&header, memory, &file_data)?;

    log::info!(
        "XBE: entry=0x{:08X} thunk=0x{:08X} retail={} d3d_section={}",
        entry_point,
        kernel_thunk_addr,
        has_d3d_section,
        has_d3d_section
    );

    // Resolve kernel imports
    let thunk_table = ThunkTable::resolve(
        memory,
        kernel_thunk_addr,
        header.base_address,
        header.image_size,
    );
    let import_count = thunk_table.len() as u32;
    log::info!("XBE: resolved {} kernel imports", import_count);

    // Verify a few thunk entries were resolved correctly
    if import_count > 0 {
        let v0 = memory.read_u32(kernel_thunk_addr);
        let v75 = memory.read_u32(kernel_thunk_addr + 75 * 4); // entry #75 = NtAllocateVirtualMemory
        crate::xbox::emulator::debug_log(&format!(
            "XBE: thunk verify: [0]={:#010X} [75]={:#010X} at VA 0x{:08X}+0x{:X} (expect 0xFFFF****)",
            v0, v75, kernel_thunk_addr, 75 * 4
        ));
    }

    // Extract title and title_id from certificate
    let title = extract_title(&file_data, header.cert_address, header.base_address);
    let cert_file_offset = (header.cert_address - header.base_address) as usize;
    let title_id = if cert_file_offset + 0x0C <= file_data.len() {
        read_u32(&file_data, cert_file_offset + 0x08)
    } else {
        0
    };
    let allowed_media = if cert_file_offset + 0xA0 <= file_data.len() {
        read_u32(&file_data, cert_file_offset + 0x9C)
    } else {
        0
    };
    let game_region = if cert_file_offset + 0xA4 <= file_data.len() {
        read_u32(&file_data, cert_file_offset + 0xA0)
    } else {
        0
    };
    log::info!("XBE: title = '{}', title_id = 0x{:08X}", title, title_id);
    crate::xbox::emulator::debug_log(&format!(
        "XBE: certificate allowed_media=0x{:08X} game_region=0x{:08X}",
        allowed_media, game_region
    ));

    // Patch LaunchDataPage.dwTitleId to match the XBE certificate.
    // The CRT compares LaunchDataPage->dwTitleId against the running XBE's title ID.
    // If they don't match → cold boot → XWriteTitleInfoAndRebootA → HalReturnToFirmware(2).
    const LAUNCH_PAGE_ADDR: u32 = 0x00F2_0000;
    memory.write_u32(LAUNCH_PAGE_ADDR + 4, title_id);
    log::info!(
        "XBE: LaunchDataPage.dwTitleId = 0x{:08X} (matches certificate)",
        title_id
    );

    // Parse library versions from XBE header to find D3D8 build version.
    // Each library entry: 8 bytes name (ASCII) + 2 major + 2 minor + 2 build + 2 flags = 16 bytes.
    let (d3d8_build_version, d3d8_is_ltcg) = parse_d3d8_build_version(
        &file_data,
        header.lib_versions_addr,
        header.lib_versions_count,
        header.base_address,
    );
    log::info!(
        "XBE: D3D8 build version = {}, LTCG = {}",
        d3d8_build_version,
        d3d8_is_ltcg
    );
    crate::xbox::emulator::debug_log(&format!(
        "XBE: D3D8 lib build version = {} LTCG={} (used for OOVPA pattern selection)",
        d3d8_build_version, d3d8_is_ltcg
    ));

    // ========================================================================
    // PROBE A · XBE constructor-table enumeration (2026-04-20, docs/xbe_ctor_reference.md)
    // Read-only diagnostic. Dumps:
    //   1. TLS directory (XBE header offset 0x178)
    //   2. Section list with executable + size (for spotting .CRT / .rdata)
    //   3. Spider-Man ctor-table addresses (0x3DB000, 0x3DBB50 per CLAUDE.md)
    //   4. First 8 entries of kernel thunk table
    // Goal: identify whether a constructor walk is being skipped. If the TLS
    // directory has AddressOfCallBacks != 0, or if .CRT sections exist that
    // aren't walked by static_ctors_1 / _2, that's the missing init.
    // ========================================================================
    {
        crate::xbox::emulator::debug_log(
            "[XBE-ENUM] === Probe A: constructor table enumeration ===",
        );

        // ---- (1) TLS directory ----
        // Per Cxbx-R Xbe.h, XBE header layout has dwTLSAddr at offset 0x12C
        // (between dwEntryAddr at 0x128 and dwPeStackCommit at 0x130).
        // If non-zero, it points to an IMAGE_TLS_DIRECTORY-compatible struct:
        //   +0x00 StartAddressOfRawData   +0x04 EndAddressOfRawData
        //   +0x08 AddressOfIndex          +0x0C AddressOfCallBacks
        //   +0x10 SizeOfZeroFill          +0x14 Characteristics
        // (Previously read 0x178 by mistake — that's inside the PE metadata
        // range, not the TLS field. Fixed 2026-04-20.)
        let tls_addr = read_u32(&file_data, 0x12C);
        crate::xbox::emulator::debug_log(&format!(
            "[XBE-ENUM] TLS directory address: 0x{:08X} {}",
            tls_addr,
            if tls_addr == 0 {
                "(no TLS — callbacks path inert)"
            } else {
                "(PRESENT — parsing)"
            }
        ));
        if tls_addr != 0
            && tls_addr >= header.base_address
            && tls_addr < header.base_address + header.image_size
        {
            let raw_start = memory.read_u32(tls_addr);
            let raw_end = memory.read_u32(tls_addr + 4);
            let idx_addr = memory.read_u32(tls_addr + 8);
            let cb_addr = memory.read_u32(tls_addr + 12);
            let zero_fill = memory.read_u32(tls_addr + 16);
            let chars = memory.read_u32(tls_addr + 20);
            crate::xbox::emulator::debug_log(&format!(
                "[XBE-ENUM]   TLS: raw=[{:#010X}..{:#010X}] idx=0x{:08X} callbacks=0x{:08X} zero_fill={} chars=0x{:08X}",
                raw_start, raw_end, idx_addr, cb_addr, zero_fill, chars
            ));
            if cb_addr != 0 {
                crate::xbox::emulator::debug_log(
                    "[XBE-ENUM]   TLS CALLBACKS PRESENT — game expects these to run before main!",
                );
                let mut i = 0u32;
                loop {
                    let cb = memory.read_u32(cb_addr + i * 4);
                    if cb == 0 || i >= 16 {
                        break;
                    }
                    crate::xbox::emulator::debug_log(&format!(
                        "[XBE-ENUM]     callback[{}] = 0x{:08X}",
                        i, cb
                    ));
                    i += 1;
                }
                if i == 0 {
                    crate::xbox::emulator::debug_log(
                        "[XBE-ENUM]     (callback array exists but first entry is 0 — empty)",
                    );
                }
            } else {
                crate::xbox::emulator::debug_log(
                    "[XBE-ENUM]   No TLS callbacks (AddressOfCallBacks = 0)",
                );
            }
        }

        // ---- (2) Section table — look for CRT / data / rdata ----
        // MSVC linker concatenates .CRT$XCA..XCZ into a contiguous array.
        // In retail XBEs the .CRT section is often merged into .rdata or .data.
        // We already logged per-section info above; here we highlight ones
        // likely holding constructors.
        crate::xbox::emulator::debug_log(&format!(
            "[XBE-ENUM] {} sections enumerated:",
            sections.len()
        ));
        for s in &sections {
            let name_u = s.name.to_uppercase();
            let interesting = name_u.contains("CRT")
                || name_u == ".DATA"
                || name_u == ".RDATA"
                || name_u == ".TLS"
                || name_u == "$$XTIMAGE"
                || name_u.contains("INIT");
            crate::xbox::emulator::debug_log(&format!(
                "[XBE-ENUM]   '{}' VA=0x{:08X} vsize=0x{:X} raw=0x{:X} flags=0x{:08X}{}{}",
                s.name,
                s.virtual_address,
                s.virtual_size,
                s.raw_size,
                s.flags,
                if s.executable { " [EXEC]" } else { "" },
                if interesting { "  <-- candidate" } else { "" }
            ));
        }

        // ---- (3) Spider-Man's static_ctors_1 / _2 tables ----
        // Per CLAUDE.md: static_ctors_1 @ 0x002AE296 walks [0x3DB000..0x3DB00C]
        //                static_ctors_2 @ 0x002AE23E walks [0x3DBB50..0x3DBB64]
        // Each entry is a 4-byte function pointer. Table size per CLAUDE.md is
        // 0xC and 0x14 bytes respectively (3 and 5 pointers). Dump them so we
        // can see what ctors the game WOULD call if those functions ran.
        let check_table = |label: &str, start: u32, end: u32| {
            if start < header.base_address || end > header.base_address + header.image_size {
                crate::xbox::emulator::debug_log(&format!(
                    "[XBE-ENUM] {}: table [0x{:08X}..0x{:08X}] OUTSIDE image bounds — not applicable",
                    label, start, end
                ));
                return;
            }
            crate::xbox::emulator::debug_log(&format!(
                "[XBE-ENUM] {}: walking [0x{:08X}..0x{:08X}] ({} entries)",
                label,
                start,
                end,
                (end.saturating_sub(start)) / 4
            ));
            let mut addr = start;
            let mut idx = 0u32;
            while addr + 4 <= end && idx < 16 {
                let ptr = memory.read_u32(addr);
                crate::xbox::emulator::debug_log(&format!(
                    "[XBE-ENUM]   [{}] @0x{:08X} = 0x{:08X}{}",
                    idx,
                    addr,
                    ptr,
                    if ptr == 0 {
                        " (NULL — skipped by _initterm)"
                    } else {
                        ""
                    }
                ));
                addr += 4;
                idx += 1;
            }
        };
        // Tables as documented in CLAUDE.md — XBE base for Spider-Man is 0x00010000,
        // so 0x3DB000 is inside the image (image_size typically ~0x500000).
        check_table("static_ctors_1 table", 0x003D_B000, 0x003D_B00C);
        check_table("static_ctors_2 table", 0x003D_BB50, 0x003D_BB64);

        // ---- (4) First 8 kernel thunk entries ----
        // These should now be resolved (XOR-decoded) per the decrypt step above.
        // High bytes 0xFFFF indicate successful kernel ordinal resolution.
        crate::xbox::emulator::debug_log(&format!(
            "[XBE-ENUM] Kernel thunk table @0x{:08X}, first 8 entries:",
            kernel_thunk_addr
        ));
        for i in 0..8 {
            let thunk = memory.read_u32(kernel_thunk_addr + i * 4);
            crate::xbox::emulator::debug_log(&format!(
                "[XBE-ENUM]   thunk[{}] = 0x{:08X}{}",
                i,
                thunk,
                if thunk >> 16 == 0xFFFF {
                    " (ordinal resolved)"
                } else {
                    ""
                }
            ));
        }
        // ---- (5) Tripwire installation ----
        // Verify the 6 constructors we found actually run by checking if
        // their sentinel memory remains zero after worker startup. Plant
        // known non-zero sentinel values at addresses just before each
        // ctor table — if the ctor runs and WRITES into .data, neighboring
        // addresses may change. If the ctors DON'T run, the whole .data
        // region stays at zero (except what we pre-loaded from raw_data).
        //
        // Simpler: expose the ctor addresses so downstream diagnostic code
        // can watch for execution. Just list them here.
        crate::xbox::emulator::debug_log(
            "[XBE-ENUM] Suspected ctor functions (watch for guest PC hitting these):",
        );
        let ctor_fns = [
            (0x002AE296u32, "static_ctors_1 iterator"),
            (0x002AE23Eu32, "static_ctors_2 iterator"),
            (0x002BA4A5u32, "ctors1[1]"),
            (0x002BB0F0u32, "ctors1[2]"),
            (0x002B65E2u32, "ctors2[1]"),
            (0x002BA862u32, "ctors2[2]"),
            (0x002BED7Bu32, "ctors2[3]"),
            (0x002BB464u32, "ctors2[4]"),
        ];
        for (addr, name) in ctor_fns.iter() {
            // Read first 4 bytes at the ctor to check it's actually code
            // (not zero or kernel-thunk-magic). A valid guest code byte
            // pattern starts with common x86 prologue: 0x55 (push ebp)
            // or 0x83 (sub esp imm) or 0x56 (push esi) etc.
            let first_byte = memory.read_u8(*addr);
            let kind = match first_byte {
                0x55 => "push ebp (standard prologue)",
                0x53 => "push ebx",
                0x56 => "push esi",
                0x83 => "sub esp, imm",
                0x8B => "mov reg, ...",
                0xE8 => "call rel32",
                0xE9 => "jmp rel32",
                0x00 => "ZERO — not code!",
                _ => "other",
            };
            crate::xbox::emulator::debug_log(&format!(
                "[XBE-ENUM]   0x{:08X} '{}' first byte=0x{:02X} ({})",
                addr, name, first_byte, kind
            ));
        }

        crate::xbox::emulator::debug_log("[XBE-ENUM] === end Probe A ===");
    }

    Ok(XbeInfo {
        entry_point,
        base_address: header.base_address,
        image_size: header.image_size,
        is_retail: has_d3d_section,
        title,
        title_id,
        allowed_media,
        game_region,
        num_sections: header.num_sections,
        kernel_thunk_addr,
        sections,
        d3d8_build_version,
        d3d8_is_ltcg,
    })
}

/// Decrypt entry point and thunk address.
/// Tries multiple strategies: unencrypted (homebrew), external keys, heuristic scan.
fn decrypt_xbe_addresses(
    header: &XbeHeader,
    memory: &GuestMemory,
    _file_data: &[u8],
) -> Result<(u32, u32), XbeError> {
    let base = header.base_address;
    let img_size = header.image_size;

    // Strategy 1: Try unencrypted (nxdk / homebrew XBEs)
    let raw_entry = header.entry_point;
    let raw_thunk = header.kernel_thunk_addr;
    if is_valid_entry(raw_entry, base, img_size)
        && is_valid_thunk(raw_thunk, base, img_size, memory)
    {
        log::info!("XBE: unencrypted (homebrew/nxdk)");
        return Ok((raw_entry, raw_thunk));
    }

    // Strategy 2: Try well-known XOR keys
    // Retail keys
    let entry_retail = raw_entry ^ 0xA8FC57AB;
    let thunk_retail = raw_thunk ^ 0x5B6D40B6;
    if is_valid_entry(entry_retail, base, img_size)
        && is_valid_thunk(thunk_retail, base, img_size, memory)
    {
        log::info!("XBE: decrypted with retail keys");
        return Ok((entry_retail, thunk_retail));
    }

    // Debug keys
    let entry_debug = raw_entry ^ 0x94859D4B;
    let thunk_debug = raw_thunk ^ 0xEFB1F152;
    if is_valid_entry(entry_debug, base, img_size)
        && is_valid_thunk(thunk_debug, base, img_size, memory)
    {
        log::info!("XBE: decrypted with debug keys");
        return Ok((entry_debug, thunk_debug));
    }

    // Strategy 3: Heuristic — scan mapped memory for thunk table signature
    if let Some(thunk_va) = find_thunk_table(memory, base, img_size) {
        let thunk_key = raw_thunk ^ thunk_va;
        // Derive entry key via key-family deltas (same as C++ KEY_DELTA_FAMILY_A/B)
        let delta_a: u32 = 0xF391171D;
        let delta_b: u32 = 0x7B346C19;

        for delta in [delta_a, delta_b] {
            let entry_key = thunk_key ^ delta;
            let entry_candidate = raw_entry ^ entry_key;
            if is_valid_entry(entry_candidate, base, img_size) {
                log::info!("XBE: decrypted via heuristic (thunk at 0x{:08X})", thunk_va);
                return Ok((entry_candidate, thunk_va));
            }
        }

        // Try same key for entry
        let entry_same = raw_entry ^ thunk_key;
        if is_valid_entry(entry_same, base, img_size) {
            log::info!("XBE: decrypted via heuristic (same key)");
            return Ok((entry_same, thunk_va));
        }
    }

    Err(XbeError::EntryPointDecryptFailed)
}

fn is_valid_entry(entry: u32, base: u32, img_size: u32) -> bool {
    entry >= base && entry < base + img_size
}

fn is_valid_thunk(thunk: u32, base: u32, img_size: u32, memory: &GuestMemory) -> bool {
    if thunk < base || thunk + 4 >= base + img_size {
        return false;
    }
    let first_val = memory.read_u32(thunk);
    if (first_val & 0x8000_0000) == 0 {
        return false;
    }
    let ord = first_val & 0x7FFF_FFFF;
    ord >= 1 && ord <= 378
}

/// Scan mapped guest memory for the kernel thunk table signature:
/// array of u32 values where (val & 0x80000000) and (val & 0x7FFFFFFF) in [1, 378].
fn find_thunk_table(memory: &GuestMemory, base: u32, img_size: u32) -> Option<u32> {
    let mut best_va = 0u32;
    let mut best_count = 0;

    let mut addr = base;
    while addr + 32 < base + img_size {
        let mut count = 0;
        for i in 0..400u32 {
            if addr + i * 4 + 4 >= base + img_size {
                break;
            }
            let val = memory.read_u32(addr + i * 4);
            if val == 0 {
                break;
            }
            if (val & 0x8000_0000) == 0 {
                break;
            }
            let ord = val & 0x7FFF_FFFF;
            if ord < 1 || ord > 378 {
                break;
            }
            count += 1;
        }
        if count >= 4 && count > best_count {
            best_va = addr;
            best_count = count;
        }
        addr += 4;
    }

    if best_count >= 4 {
        Some(best_va)
    } else {
        None
    }
}

/// Data export base in guest memory (matching C++ 0x00E00000).
/// Each data export gets 256 bytes.
const DATA_EXPORT_BASE: u32 = 0x00E0_0000;
const DATA_EXPORT_ENTRY_SIZE: u32 = 0x100; // 256 bytes per export

/// Check if an ordinal is a kernel data export (not a function).
fn is_data_export(ordinal: u32) -> bool {
    matches!(
        ordinal,
        16 | 22 | 30 | 31 |           // Ex*ObjectType
        40 | 41 | 42 |                 // HalDisk*
        64 | 70 | 71 |                 // Io*ObjectType
        88 | 89 |                       // KdDebugger*
        102 |                           // MmGlobalData
        120 | 154 | 156 | 157 |        // Ke*Time*
        162 | 164 |                     // KiBugCheckData, LaunchDataPage
        240 | 245 | 249 |              // Ob*
        259 |                           // PsThreadObjectType
        321 | 322 | 323 | 324 | 325 | 326 | // Xbox*, Xe*
        353 | 354 | 355 | 356 | 357 // XboxLANKey..IdexChannelObject
    )
}

/// DWORD-sized data exports whose value lives directly in the thunk slot.
/// The Xbox kernel writes these in-place; game code reads with a single
/// `mov eax, [thunk_slot]` (no `__declspec(dllimport)` indirection).
fn is_value_in_thunk(ordinal: u32) -> bool {
    matches!(
        ordinal,
        16 | 22 | 30 | 31 |           // Ex*ObjectType (opaque DWORD handles)
        64 | 70 | 71 |                 // Io*ObjectType
        88 | 89 |                       // KdDebugger* (BYTE-sized)
        156 | 157 |                     // KeTickCount, KeTimeIncrement
        240 | 245 | 249 |              // Ob*ObjectType
        259 // PsThreadObjectType
    )
}

/// Data exports that change at runtime. These must NOT be included in the
/// thunk table entries — the AOT emitter constant-folds thunk reads into
/// `MOV reg, imm32`, which would bake the initial value permanently.
fn is_mutable_data_export(ordinal: u32) -> bool {
    matches!(ordinal, 120 | 154 | 156 | 157)
    // 120=KeInterruptTime, 154=KeSystemTime, 156=KeTickCount, 157=KeTimeIncrement
}

/// Initialize a kernel data export at the given guest address.
fn init_data_export(memory: &GuestMemory, ordinal: u32, addr: u32) {
    match ordinal {
        // Object type pointers — non-zero magic tags so NULL checks pass
        16 => memory.write_u32(addr, 0x00F0_0100), // ExEventObjectType
        22 => memory.write_u32(addr, 0x00F0_0200), // ExMutantObjectType
        30 => memory.write_u32(addr, 0x00F0_0300), // ExSemaphoreObjectType
        31 => memory.write_u32(addr, 0x00F0_0400), // ExTimerObjectType
        64 => memory.write_u32(addr, 0x00F0_0500), // IoCompletionObjectType
        70 => memory.write_u32(addr, 0x00F0_0600), // IoDeviceObjectType
        71 => memory.write_u32(addr, 0x00F0_0700), // IoFileObjectType
        240 => memory.write_u32(addr, 0x00F0_0800), // ObDirectoryObjectType
        245 => memory.write_u32(addr, 0x00F0_0900), // ObpObjectHandleTable
        249 => memory.write_u32(addr, 0x00F0_0A00), // ObSymbolicLinkObjectType
        259 => memory.write_u32(addr, 0x00F0_0B00), // PsThreadObjectType

        // Timing
        120 | 154 => {
            // KeInterruptTime / KeSystemTime -> 8-byte KSYSTEM_TIME (100ns units)
            let t: u64 = 132_000_000_000_000_000;
            memory.write_u32(addr, t as u32);
            memory.write_u32(addr + 4, (t >> 32) as u32);
            if ordinal == 120 {
                KE_INTERRUPT_TIME_ADDR.store(addr, Ordering::Relaxed);
                crate::xbox::emulator::debug_log(&format!(
                    "KeInterruptTime data export at 0x{:08X}",
                    addr
                ));
            } else {
                KE_SYSTEM_TIME_ADDR.store(addr, Ordering::Relaxed);
                crate::xbox::emulator::debug_log(&format!(
                    "KeSystemTime data export at 0x{:08X}",
                    addr
                ));
            }
        }
        156 => {
            // KeTickCount: some XBEs use dllimport-style double dereference
            // (mov eax,[thunk]; mov eax,[eax]) so we store a POINTER at the thunk slot
            // pointing to a separate data location where the actual tick value lives.
            // The data location is at a fixed address below DATA_EXPORT_BASE.
            let data_addr: u32 = 0x00DF_F000; // fixed KeTickCount data address
            memory.write_u32(data_addr, 1); // initial tick value
            memory.write_u32(addr, data_addr); // thunk slot = pointer to data
            KE_TICK_COUNT_ADDR.store(data_addr, Ordering::Relaxed);
            crate::xbox::emulator::debug_log(&format!(
                "KeTickCount: thunk=0x{:08X} -> data=0x{:08X}",
                addr, data_addr
            ));
        }
        157 => memory.write_u32(addr, 10_000), // KeTimeIncrement

        // XboxKrnlVersion -> 4 ULONGs (Major, Minor, Build, Qfe)
        324 => {
            memory.write_u32(addr, 1); // Major
            memory.write_u32(addr + 4, 0); // Minor
            memory.write_u32(addr + 8, 5838); // Build
            memory.write_u32(addr + 12, 1); // Qfe
        }

        // XboxHardwareInfo
        322 => {
            memory.write_u32(addr, 0x0000_0020); // Flags: INTERNAL_USB_HUB
            memory.write_u8(addr + 4, 0xB1); // GPU revision (NV2A B01)
            memory.write_u8(addr + 5, 0xD4); // MCP revision
        }

        // KdDebuggerEnabled=FALSE, KdDebuggerNotPresent=TRUE
        88 => memory.write_u8(addr, 0),
        89 => memory.write_u8(addr, 1),

        // LaunchDataPage (ordinal 164) — pointer-to-pointer.
        // The data export slot holds a pointer to a LAUNCH_DATA_PAGE (0x3000 bytes).
        // When NULL, the CRT treats this as "first boot" and calls
        // XWriteTitleInfoAndReboot → HalReturnToFirmware(2) to reboot.
        // Pre-populating with LDT_FROM_DASHBOARD (1) simulates "launched from
        // dashboard" so the CRT takes the normal init path to _initterm.
        164 => {
            const LAUNCH_PAGE_ADDR: u32 = 0x00F2_0000;
            // Store pointer to the page in the data export slot
            memory.write_u32(addr, LAUNCH_PAGE_ADDR);
            // LAUNCH_DATA_PAGE layout (0x3000 bytes total):
            //   +0x000: dwLaunchDataType (DWORD)
            //   +0x004: dwTitleId (DWORD)
            //   +0x008: szLaunchPath[256] (char[])
            //   +0x108: dwFlags (DWORD)
            //   +0x10C: pad[0x2F4]
            //   +0x400: LaunchData[0x2C00]
            memory.write_u32(LAUNCH_PAGE_ADDR, 1); // LDT_FROM_DASHBOARD
                                                   // dwTitleId is set later in load_xbe() from the XBE certificate.
                                                   // Must match the running XBE's title ID or CRT takes cold-boot reboot path.
                                                   // szLaunchPath: standard Xbox boot path
            let path = b"\\Device\\CdRom0\\default.xbe\0";
            for (i, &b) in path.iter().enumerate() {
                memory.write_u8(LAUNCH_PAGE_ADDR + 8 + i as u32, b);
            }
            memory.write_u32(LAUNCH_PAGE_ADDR + 0x108, 0); // dwFlags = 0
        }

        // Everything else: leave zeroed (keys=zeroed, etc.)
        _ => {}
    }
}

/// Kernel thunk table: resolved import entries with snapshot for runtime repair.
///
/// After XBE loading, each thunk entry holds either:
/// - `0xFFFF0000 + ordinal` for function imports (VEH dispatch)
/// - A guest memory pointer for data exports
///
/// Guest code (D3D init) zeroes thunk entries at runtime. The snapshot
/// preserves original resolved values so the emitter can constant-fold
/// thunk reads and the worker can repair corrupted entries.
#[derive(Debug, Clone)]
pub struct ThunkTable {
    /// (guest_addr, resolved_value) pairs in table order.
    entries: Vec<(u32, u32)>,
    /// First thunk address in guest memory.
    start: u32,
    /// One past last thunk address (start + entries.len() * 4).
    end: u32,
}

impl ThunkTable {
    /// Build an empty thunk table (for games where thunk resolution hasn't run yet).
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            start: 0,
            end: 0,
        }
    }

    /// Resolve kernel thunks in guest memory and snapshot the results.
    /// Function ordinals: 0x80000000|ordinal → 0xFFFF0000+ordinal (VEH dispatch).
    /// Data exports: 0x80000000|ordinal → pointer to allocated guest memory.
    pub fn resolve(memory: &GuestMemory, thunk_addr: u32, base: u32, img_size: u32) -> Self {
        let mut entries = Vec::new();
        let mut data_count = 0u32;
        let mut data_bump = DATA_EXPORT_BASE;
        let mut addr = thunk_addr;

        loop {
            if addr + 4 >= base + img_size {
                break;
            }
            let val = memory.read_u32(addr);
            if val == 0 {
                break;
            }
            if (val & 0x8000_0000) == 0 {
                break;
            }
            let ordinal = val & 0x7FFF_FFFF;
            if ordinal < 1 || ordinal > 378 {
                break;
            }

            if is_data_export(ordinal) {
                if is_value_in_thunk(ordinal) {
                    // DWORD-sized data exports: write VALUE directly into thunk slot.
                    // Xbox kernel updates these in-place; game reads with one dereference.
                    init_data_export(memory, ordinal, addr);
                    // Mutable exports (KeTickCount etc) must NOT be in entries —
                    // the emitter constant-folds thunk reads, which would bake
                    // the initial value as an immediate forever.
                    if !is_mutable_data_export(ordinal) {
                        entries.push((addr, memory.read_u32(addr)));
                    }
                } else {
                    // Struct-sized data exports: allocate separate storage, write ADDRESS
                    // into thunk slot. Game dereferences pointer to reach the struct.
                    let data_addr = data_bump;
                    data_bump += DATA_EXPORT_ENTRY_SIZE;
                    memory.write_u32(addr, data_addr);
                    init_data_export(memory, ordinal, data_addr);
                    entries.push((addr, data_addr));
                }
                data_count += 1;
            } else {
                let magic_addr = KERNEL_MAGIC_BASE + ordinal;
                memory.write_u32(addr, magic_addr);
                entries.push((addr, magic_addr));
            }
            addr += 4;
        }

        let count = entries.len() as u32;
        let end = if entries.is_empty() {
            thunk_addr
        } else {
            entries.last().unwrap().0 + 4
        };

        log::info!(
            "XBE: thunk table: {} entries ({} functions, {} data exports)",
            count,
            count - data_count,
            data_count
        );

        Self {
            entries,
            start: thunk_addr,
            end,
        }
    }

    /// Snapshot thunk table from already-resolved guest memory.
    /// Used when RuntimeContext needs its own copy after resolve() was called elsewhere.
    pub fn snapshot(mem_base: *const u8, thunk_addr: u32, base: u32, img_size: u32) -> Self {
        let tick_addr = KE_TICK_COUNT_ADDR.load(Ordering::Relaxed);
        let mut entries = Vec::new();
        let mut addr = thunk_addr;
        loop {
            if addr + 4 >= base + img_size {
                break;
            }
            let val = unsafe { *(mem_base.add(addr as usize) as *const u32) };
            if val == 0 {
                break;
            }
            // Skip mutable data exports — emitter would constant-fold their
            // current value as an immediate, preventing runtime updates.
            if addr != tick_addr {
                entries.push((addr, val));
            }
            addr += 4;
        }
        let end = if entries.is_empty() {
            thunk_addr
        } else {
            entries.last().unwrap().0 + 4
        };
        Self {
            entries,
            start: thunk_addr,
            end,
        }
    }

    /// Look up the resolved value for a thunk guest address.
    /// Returns None if `addr` is not in the thunk table.
    pub fn lookup(&self, addr: u32) -> Option<u32> {
        if addr < self.start || addr >= self.end {
            return None;
        }
        // Thunk entries are contiguous 4-byte slots — direct index.
        let idx = ((addr - self.start) / 4) as usize;
        self.entries.get(idx).map(|&(a, v)| {
            debug_assert_eq!(a, addr);
            v
        })
    }

    /// Check if a guest address falls within the thunk table range.
    pub fn contains(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.end
    }

    /// Repair all zeroed thunk entries in guest memory from the snapshot.
    /// Returns the number of entries repaired.
    pub fn repair_all(&self, memory: &GuestMemory) -> u32 {
        // Skip KeTickCount — it's a live value updated every frame
        let tick_addr = KE_TICK_COUNT_ADDR.load(std::sync::atomic::Ordering::Relaxed);
        let mut repaired = 0;
        for &(addr, val) in &self.entries {
            if addr == tick_addr {
                continue;
            }
            let current = memory.read_u32(addr);
            if current != val {
                memory.write_u32(addr, val);
                repaired += 1;
            }
        }
        repaired
    }

    /// Repair all zeroed thunk entries via raw pointer (for VEH context where GuestMemory isn't available).
    pub fn repair_all_raw(&self, mem_base: *mut u8) -> u32 {
        let tick_addr = KE_TICK_COUNT_ADDR.load(std::sync::atomic::Ordering::Relaxed);
        let mut repaired = 0;
        for &(addr, val) in &self.entries {
            if addr == tick_addr {
                continue;
            }
            let current = unsafe { *(mem_base.add(addr as usize) as *const u32) };
            if current != val {
                unsafe {
                    *(mem_base.add(addr as usize) as *mut u32) = val;
                }
                repaired += 1;
            }
        }
        repaired
    }

    /// Scan backward from a trap site to find which thunk entry was loaded,
    /// then return the original resolved value from the snapshot.
    /// This replaces the old `repair_zeroed_thunk()` in veh_dispatch.rs.
    pub fn repair_at_site(&self, mem_base: *const u8, guest_addr: u32) -> Option<u32> {
        if self.entries.is_empty() {
            return None;
        }

        // Strategy 1: Scan backward up to 32 bytes to find `mov reg, [disp32]`
        // where disp32 falls in the thunk table range.
        let scan_start = guest_addr.saturating_sub(32);
        let scan_len = (guest_addr - scan_start + 15) as usize;
        if mem_base.is_null() {
            return None;
        }

        let bytes: Vec<u8> = (0..scan_len)
            .map(|i| unsafe { *mem_base.add((scan_start as usize) + i) })
            .collect();

        use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};
        let mut decoder = Decoder::with_ip(32, &bytes, scan_start as u64, DecoderOptions::NONE);
        let mut instr = Instruction::default();

        while decoder.can_decode() {
            let pos = decoder.position();
            decoder.decode_out(&mut instr);
            if instr.is_invalid() {
                continue;
            }
            let instr_addr = scan_start + pos as u32;
            if instr_addr >= guest_addr {
                break;
            }

            if instr.mnemonic() == Mnemonic::Mov
                && instr.op0_kind() == OpKind::Register
                && instr.op_count() >= 2
                && instr.op_kind(1) == OpKind::Memory
                && instr.memory_base() == Register::None
                && instr.memory_index() == Register::None
            {
                let disp = instr.memory_displacement32();
                if let Some(resolved) = self.lookup(disp) {
                    // Write repaired value back to guest memory
                    unsafe {
                        (mem_base as *mut u8)
                            .add(disp as usize)
                            .cast::<u32>()
                            .write_unaligned(resolved);
                    }
                    return Some(resolved);
                }
            }
        }

        // Strategy 2: Find any zeroed function thunk and repair it (brute force fallback).
        for &(addr, resolved) in &self.entries {
            let current = unsafe { *(mem_base.add(addr as usize) as *const u32) };
            if current == 0 && resolved >= KERNEL_MAGIC_BASE {
                unsafe {
                    (mem_base as *mut u8)
                        .add(addr as usize)
                        .cast::<u32>()
                        .write_unaligned(resolved);
                }
                return Some(resolved);
            }
        }

        None
    }

    /// Number of entries in the thunk table.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the thunk table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Start address of the thunk table in guest memory.
    pub fn start_addr(&self) -> u32 {
        self.start
    }

    /// End address (exclusive) of the thunk table in guest memory.
    pub fn end_addr(&self) -> u32 {
        self.end
    }

    /// Iterate over (guest_addr, resolved_value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(u32, u32)> {
        self.entries.iter()
    }

    /// Get the entries as a slice (for emitter constant folding).
    pub fn as_slice(&self) -> &[(u32, u32)] {
        &self.entries
    }
}

/// Extract game title from XBE certificate (UTF-16LE → UTF-8).
fn extract_title(file_data: &[u8], cert_address: u32, base_address: u32) -> String {
    let cert_offset = (cert_address - base_address) as usize;
    let title_offset = cert_offset + CERT_TITLE_NAME_OFFSET;
    let title_end = title_offset + CERT_TITLE_NAME_LEN * 2; // UTF-16LE = 2 bytes per char

    if title_end > file_data.len() {
        return "Unknown".to_string();
    }

    let title_bytes = &file_data[title_offset..title_end];
    let wide_chars: Vec<u16> = title_bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .take_while(|&c| c != 0) // Null-terminated
        .collect();

    String::from_utf16_lossy(&wide_chars)
}

/// Read a null-terminated ASCII section name from the XBE file.
fn read_section_name(file_data: &[u8], name_addr: u32, base_address: u32) -> String {
    let offset = (name_addr.wrapping_sub(base_address)) as usize;
    if offset >= file_data.len() {
        return "???".to_string();
    }
    let end = file_data[offset..]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(63.min(file_data.len() - offset));
    String::from_utf8_lossy(&file_data[offset..offset + end]).to_string()
}

/// Parse XBE library version table to find D3D8 build version.
/// Library entry: 8 bytes ASCII name + 2 major + 2 minor + 2 build + 2 flags = 16 bytes.
/// Returns build number for D3D8 (or XAPILIB/XBOXKRNL as fallback), 0 if not found.
fn parse_d3d8_build_version(
    file_data: &[u8],
    lib_addr: u32,
    lib_count: u32,
    base: u32,
) -> (u16, bool) {
    if lib_addr == 0 || lib_count == 0 || lib_count > 64 {
        return (0, false);
    }
    let base_off = lib_addr.wrapping_sub(base) as usize;
    let mut d3d8_build: u16 = 0;
    let mut d3d8_ltcg: bool = false;
    let mut xapi_build: u16 = 0;

    for i in 0..lib_count as usize {
        let off = base_off + i * 16;
        if off + 16 > file_data.len() {
            break;
        }

        // 8-byte ASCII name (space-padded)
        let name = &file_data[off..off + 8];
        let name_str = std::str::from_utf8(&name[..name.iter().position(|&b| b == 0).unwrap_or(8)])
            .unwrap_or("???");
        let build = u16::from_le_bytes([file_data[off + 12], file_data[off + 13]]);
        let flags = u16::from_le_bytes([file_data[off + 14], file_data[off + 15]]);

        log::info!(
            "XBE: lib[{}] = '{}' build={} flags=0x{:04X}",
            i,
            name_str.trim(),
            build,
            flags
        );

        if name_str.starts_with("D3D8") {
            d3d8_build = build;
            // QFEVersion field (bits 0-12): value of 2+ typically indicates LTCG build
            // Spider-Man: D3D8 flags=0x4002 (QFE=2, LTCG), test XBE: 0x4001 (QFE=1, standard).
            // Some late retail titles advertise LTCG by the 8-byte library name itself
            // (D3D8LTCG) while still using QFE=1, so the name is authoritative.
            let qfe = flags & 0x1FFF;
            let name_is_ltcg = name_str.trim().eq_ignore_ascii_case("D3D8LTCG");
            d3d8_ltcg |= name_is_ltcg || qfe >= 2;
            if name_is_ltcg && qfe < 2 {
                log::info!(
                    "XBE: D3D8LTCG detected by library name despite QFE={} flags=0x{:04X}",
                    qfe,
                    flags
                );
            }
        } else if name_str.starts_with("XAPILIB") && xapi_build == 0 {
            xapi_build = build;
        }
    }

    (
        if d3d8_build > 0 {
            d3d8_build
        } else {
            xapi_build
        },
        d3d8_ltcg,
    )
}

/// Helper: read a little-endian u32 from a byte slice.
fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}
