use iced_x86::{Decoder, DecoderOptions, Formatter, IntelFormatter, Mnemonic};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
struct Section {
    name: String,
    va: u32,
    vsize: u32,
    raw: u32,
    raw_size: u32,
    executable: bool,
}

#[derive(Clone, Debug)]
struct RosettaRow {
    addr: u32,
    section: String,
    decoded: bool,
    direct_branch_target: bool,
    call_fallthrough: bool,
    log_target: bool,
    text: String,
}

fn read_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(data[off..off + 4].try_into().unwrap())
}

fn parse_hex(s: &str) -> Option<u32> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(s, 16).ok()
}

fn find_hex_after(line: &str, marker: &str) -> Option<u32> {
    let idx = line.find(marker)?;
    let rest = &line[idx + marker.len()..];
    let hex_idx = rest.find("0x")?;
    let hex = rest[hex_idx + 2..]
        .chars()
        .take_while(|c| c.is_ascii_hexdigit())
        .collect::<String>();
    parse_hex(&hex)
}

fn section_name(data: &[u8], base: u32, name_va: u32) -> String {
    let off = name_va.wrapping_sub(base) as usize;
    if off >= data.len() {
        return String::new();
    }
    let end = data[off..]
        .iter()
        .position(|b| *b == 0)
        .map(|n| off + n)
        .unwrap_or((off + 32).min(data.len()));
    String::from_utf8_lossy(&data[off..end]).to_string()
}

fn parse_sections(data: &[u8]) -> (u32, u32, Vec<Section>) {
    if data.len() < 0x130 || &data[0..4] != b"XBEH" {
        panic!("not an XBE");
    }
    let base = read_u32(data, 0x104);
    let entry_enc = read_u32(data, 0x128);
    let entry_retail = entry_enc ^ 0xA8FC_57AB;
    let entry_debug = entry_enc ^ 0x9485_9D4B;
    let entry = if (0x0001_0000..0x0200_0000).contains(&entry_retail) {
        entry_retail
    } else {
        entry_debug
    };
    let num_sections = read_u32(data, 0x11c);
    let sh_va = read_u32(data, 0x120);
    let sh_off = sh_va.wrapping_sub(base) as usize;
    let mut sections = Vec::new();
    for i in 0..num_sections.min(128) {
        let off = sh_off + i as usize * 56;
        if off + 56 > data.len() {
            break;
        }
        let flags = read_u32(data, off);
        let va = read_u32(data, off + 0x04);
        let vsize = read_u32(data, off + 0x08);
        let raw = read_u32(data, off + 0x0c);
        let raw_size = read_u32(data, off + 0x10);
        let name_va = read_u32(data, off + 0x14);
        sections.push(Section {
            name: section_name(data, base, name_va),
            va,
            vsize,
            raw,
            raw_size,
            executable: (flags & 0x04) != 0,
        });
    }
    (base, entry, sections)
}

fn section_for(sections: &[Section], addr: u32) -> Option<&Section> {
    sections
        .iter()
        .find(|s| addr >= s.va && addr < s.va.wrapping_add(s.vsize.max(s.raw_size)))
}

fn is_data_name(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == ".data" || n == ".rdata" || n == ".bss"
}

fn in_code(sections: &[Section], addr: u32) -> bool {
    section_for(sections, addr)
        .map(|s| s.executable && !is_data_name(&s.name))
        .unwrap_or(false)
}

fn bytes_for_va<'a>(
    data: &'a [u8],
    sections: &[Section],
    addr: u32,
    size: usize,
) -> Option<&'a [u8]> {
    let s = section_for(sections, addr)?;
    let off = s.raw.wrapping_add(addr.wrapping_sub(s.va)) as usize;
    let end = off.checked_add(size)?.min(data.len());
    if off >= data.len() || off >= end {
        return None;
    }
    Some(&data[off..end])
}

fn decode_one(data: &[u8], sections: &[Section], addr: u32) -> String {
    let Some(bytes) = bytes_for_va(data, sections, addr, 16) else {
        return "<no bytes>".to_string();
    };
    let mut decoder = Decoder::with_ip(32, bytes, addr as u64, DecoderOptions::NONE);
    if !decoder.can_decode() {
        return "<decode empty>".to_string();
    }
    let instr = decoder.decode();
    if instr.is_invalid() {
        return "<invalid>".to_string();
    }
    let mut fmt = IntelFormatter::new();
    let mut text = String::new();
    fmt.format(&instr, &mut text);
    text
}

fn parse_log_targets(path: &str) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return out;
    };
    for line in text.lines() {
        let marker = if line.contains("[RET-FASTMISS") || line.contains("[RET-MISS") {
            Some("guest_target=")
        } else if line.contains("[RET-UNRES") || line.contains("UNRESOLVED target=") {
            Some("target=")
        } else if line.contains("[RESCUE] Compiled guest") {
            Some("Compiled guest ")
        } else if line.contains("[OOVPA] Misaligned:") {
            Some("guest ")
        } else {
            None
        };
        if let Some(marker) = marker {
            if let Some(addr) = find_hex_after(line, marker) {
                out.insert(addr);
            }
        }
    }
    out
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: xbe_rosetta <default.xbe> [--log debug.log] [--out rosetta.csv]");
        std::process::exit(2);
    }

    let xbe_path = args.remove(0);
    let mut log_path: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--log" if i + 1 < args.len() => {
                log_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--out" if i + 1 < args.len() => {
                out_path = Some(args[i + 1].clone());
                i += 2;
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(2);
            }
        }
    }

    let data = std::fs::read(&xbe_path).expect("read XBE");
    let (base, entry, sections) = parse_sections(&data);
    println!(
        "XBE Rosetta: {} base=0x{:08X} entry=0x{:08X} sections={}",
        xbe_path,
        base,
        entry,
        sections.len()
    );

    let log_targets = log_path
        .as_deref()
        .map(parse_log_targets)
        .unwrap_or_default();

    let mut decoded = BTreeSet::new();
    let mut direct_branch_targets = BTreeSet::new();
    let mut call_fallthroughs = BTreeSet::new();
    let mut section_counts: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();

    for section in &sections {
        if !section.executable || section.vsize == 0 || is_data_name(&section.name) {
            continue;
        }
        let raw_len = section.raw_size.min(section.vsize) as usize;
        let raw = section.raw as usize;
        if raw >= data.len() {
            continue;
        }
        let end = (raw + raw_len).min(data.len());
        let mut decoder =
            Decoder::with_ip(32, &data[raw..end], section.va as u64, DecoderOptions::NONE);
        while decoder.can_decode() {
            let instr = decoder.decode();
            if instr.is_invalid() {
                continue;
            }
            let ip = instr.ip() as u32;
            decoded.insert(ip);
            if instr.mnemonic() == Mnemonic::Call {
                let fall = ip.wrapping_add(instr.len() as u32);
                if in_code(&sections, fall) {
                    call_fallthroughs.insert(fall);
                }
            }
            if instr.is_jcc_short_or_near()
                || instr.mnemonic() == Mnemonic::Jmp
                || instr.mnemonic() == Mnemonic::Call
            {
                let target = instr.near_branch_target();
                if target != 0 {
                    let target = target as u32;
                    if in_code(&sections, target) {
                        direct_branch_targets.insert(target);
                    }
                }
            }
        }
        section_counts.insert(section.name.clone(), (0, 0, 0));
    }

    for addr in &decoded {
        if let Some(s) = section_for(&sections, *addr) {
            if let Some(v) = section_counts.get_mut(&s.name) {
                v.0 += 1;
            }
        }
    }
    for addr in &direct_branch_targets {
        if let Some(s) = section_for(&sections, *addr) {
            if let Some(v) = section_counts.get_mut(&s.name) {
                v.1 += 1;
            }
        }
    }
    for addr in &call_fallthroughs {
        if let Some(s) = section_for(&sections, *addr) {
            if let Some(v) = section_counts.get_mut(&s.name) {
                v.2 += 1;
            }
        }
    }

    println!(
        "decoded_addrs={} direct_branch_targets={} call_fallthroughs={} log_targets={}",
        decoded.len(),
        direct_branch_targets.len(),
        call_fallthroughs.len(),
        log_targets.len()
    );
    for (name, (d, b, c)) in &section_counts {
        println!("{name:20} decoded={d:8} branch_targets={b:7} call_fallthroughs={c:7}");
    }

    if !log_targets.is_empty() {
        println!();
        println!("log target classification:");
        for addr in &log_targets {
            let section = section_for(&sections, *addr)
                .map(|s| s.name.as_str())
                .unwrap_or("?");
            println!(
                "0x{addr:08X} section={section:12} code={} decoded={} branch_target={} call_fallthrough={} insn={}",
                in_code(&sections, *addr),
                decoded.contains(addr),
                direct_branch_targets.contains(addr),
                call_fallthroughs.contains(addr),
                decode_one(&data, &sections, *addr)
            );
        }
    }

    if let Some(path) = out_path {
        let mut rows = BTreeMap::<u32, RosettaRow>::new();
        for addr in decoded
            .iter()
            .chain(direct_branch_targets.iter())
            .chain(call_fallthroughs.iter())
            .chain(log_targets.iter())
        {
            let section = section_for(&sections, *addr)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "?".to_string());
            rows.entry(*addr).or_insert_with(|| RosettaRow {
                addr: *addr,
                section,
                decoded: decoded.contains(addr),
                direct_branch_target: direct_branch_targets.contains(addr),
                call_fallthrough: call_fallthroughs.contains(addr),
                log_target: log_targets.contains(addr),
                text: decode_one(&data, &sections, *addr).replace('"', "'"),
            });
        }
        let mut csv = String::from(
            "addr,section,decoded,direct_branch_target,call_fallthrough,log_target,instruction\n",
        );
        for row in rows.values() {
            csv.push_str(&format!(
                "0x{:08X},{},{},{},{},{},\"{}\"\n",
                row.addr,
                row.section,
                row.decoded,
                row.direct_branch_target,
                row.call_fallthrough,
                row.log_target,
                row.text
            ));
        }
        std::fs::write(&path, csv).expect("write rosetta csv");
        println!("wrote {path}");
    }
}
