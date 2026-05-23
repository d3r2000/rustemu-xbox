use rustemu_core::xbox::aot::oovpa::{scan_ranges, OovpaMatch, OovpaScanRange, OovpaScanRangeKind};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const DEFAULT_XBE: &str = r"./games/doom3/doomlauncher.xbe";
const DEFAULT_CXBX_CACHE: &str = r"./CLASSIC DOOM-18b9d373840aa50c.ini";
const DEFAULT_FIXTURE: &str = r"./classic_doom_5849_symbols.toml";
const DEFAULT_REPORT: &str = r"./classic_doom_oovpa_report.tsv";

#[derive(Clone, Debug)]
struct Section {
    name: String,
    va: u32,
    vsize: u32,
    raw: u32,
    raw_size: u32,
    executable: bool,
}

#[derive(Default)]
struct Args {
    xbe: PathBuf,
    cxbxr_cache: PathBuf,
    fixture: PathBuf,
    output: PathBuf,
    scanner_output: Option<PathBuf>,
    profile: String,
}

#[derive(Default)]
struct Sources {
    cxbxr: BTreeMap<String, BTreeSet<u32>>,
    fixture: BTreeMap<String, BTreeSet<u32>>,
    scanner: BTreeMap<String, BTreeSet<u32>>,
}

#[derive(Default)]
struct Summary {
    cxbxr_symbols: usize,
    fixture_symbols: usize,
    scanner_symbols: usize,
    agree: usize,
    disagree: usize,
    missing_scanner: usize,
    missing_cxbxr: usize,
    missing_fixture: usize,
    scanner_only: usize,
    fixture_cxbxr_agree_scanner_missing: usize,
}

fn main() -> Result<(), String> {
    let args = parse_args()?;

    // The scanner side of the three-way diff must be pure scanner output. Do
    // not let a runtime fixture env var quietly fold TOML entries into it.
    std::env::remove_var("RUSTEMU_OOVPA_SYMBOLS_TOML");
    std::env::remove_var("RUSTEMU_DOOM_5849_TOML");
    std::env::remove_var("RUSTEMU_SPIDEY_4134_TOML");
    std::env::set_var("RUSTEMU_OOVPA_PROFILE", &args.profile);

    let image = load_xbe_image(&args.xbe)?;
    let cxbxr = parse_cxbxr_cache(&args.cxbxr_cache)?;
    let fixture = parse_fixture_toml(&args.fixture)?;
    let scanner_matches = scan_ranges(
        image.guest.as_ptr(),
        &image.scan_ranges,
        image.xdk_build,
        image.entry,
        image.d3d8_is_ltcg,
        None,
    );
    let scanner = matches_to_map(&scanner_matches);

    let scanner_output = args.scanner_output.clone().unwrap_or_else(|| {
        let mut p = args.output.clone();
        let stem = p
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("classic_doom_oovpa_report")
            .to_string();
        p.set_file_name(format!("{stem}.scanner.toml"));
        p
    });
    write_scanner_output(&scanner_output, &image, &scanner_matches)?;

    let sources = Sources {
        cxbxr,
        fixture,
        scanner,
    };
    let (report, summary) = build_report(&sources, &image, &args, &scanner_output);
    std::fs::write(&args.output, report)
        .map_err(|e| format!("write {}: {e}", args.output.display()))?;

    println!("Classic Doom OOVPA three-way probe");
    println!("  XBE:            {}", args.xbe.display());
    println!("  Cxbx-R cache:   {}", args.cxbxr_cache.display());
    println!("  TOML fixture:   {}", args.fixture.display());
    println!("  Report:         {}", args.output.display());
    println!("  Scanner output: {}", scanner_output.display());
    println!(
        "  XDK build:      {} LTCG={}",
        image.xdk_build, image.d3d8_is_ltcg
    );
    println!("  Entry:          0x{:08X}", image.entry);
    println!("  Scan ranges:    {}", image.scan_ranges.len());
    println!("Summary:");
    println!("  Cxbx-R cache:    {} symbols", summary.cxbxr_symbols);
    println!("  TOML fixture:    {} symbols", summary.fixture_symbols);
    println!("  Scanner output:  {} symbols", summary.scanner_symbols);
    println!("  Agreement:       {}", summary.agree);
    println!("  Disagreements:   {}", summary.disagree);
    println!("  Missing scanner: {}", summary.missing_scanner);
    println!("  Missing Cxbx-R:  {}", summary.missing_cxbxr);
    println!("  Missing fixture: {}", summary.missing_fixture);
    println!("  Scanner only:    {}", summary.scanner_only);

    Ok(())
}

struct XbeImage {
    guest: Vec<u8>,
    sections: Vec<Section>,
    scan_ranges: Vec<OovpaScanRange>,
    entry: u32,
    xdk_build: u16,
    d3d8_is_ltcg: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        xbe: PathBuf::from(DEFAULT_XBE),
        cxbxr_cache: PathBuf::from(DEFAULT_CXBX_CACHE),
        fixture: PathBuf::from(DEFAULT_FIXTURE),
        output: PathBuf::from(DEFAULT_REPORT),
        scanner_output: None,
        profile: "full".to_string(),
    };

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--xbe" => args.xbe = PathBuf::from(next_value(&mut it, "--xbe")?),
            "--cxbxr-cache" => {
                args.cxbxr_cache = PathBuf::from(next_value(&mut it, "--cxbxr-cache")?)
            }
            "--fixture" => args.fixture = PathBuf::from(next_value(&mut it, "--fixture")?),
            "--output" | "--out" => args.output = PathBuf::from(next_value(&mut it, "--output")?),
            "--scanner-output" => {
                args.scanner_output = Some(PathBuf::from(next_value(&mut it, "--scanner-output")?))
            }
            "--profile" => args.profile = next_value(&mut it, "--profile")?,
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(args)
}

fn next_value(it: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    it.next().ok_or_else(|| format!("{name} requires a value"))
}

fn print_usage() {
    println!(
        "usage: cargo run --release --bin doom_oovpa_probe -- [options]\n\
         \n\
         Options:\n\
           --xbe <path>             Classic Doom XBE (default: {DEFAULT_XBE})\n\
           --cxbxr-cache <path>     Cxbx-R SymbolCache .ini\n\
           --fixture <path>         Rustemu TOML fixture\n\
           --output <path>          TSV report path\n\
           --scanner-output <path>  Scanner-only TOML artifact path\n\
           --profile <name>         OOVPA scan profile: full, graphics, fast_boot (default: full)"
    );
}

fn load_xbe_image(path: &Path) -> Result<XbeImage, String> {
    let data = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if data.len() < 0x180 || &data[0..4] != b"XBEH" {
        return Err(format!("{} is not a valid XBE", path.display()));
    }

    let base = read_u32(&data, 0x104)?;
    let image_size = read_u32(&data, 0x10C)?;
    let section_count = read_u32(&data, 0x11C)?;
    let section_headers_va = read_u32(&data, 0x120)?;
    let entry_enc = read_u32(&data, 0x128)?;
    let lib_versions_count = read_u32(&data, 0x160)?;
    let lib_versions_va = read_u32(&data, 0x164)?;

    let entry_retail = entry_enc ^ 0xA8FC_57AB;
    let entry_debug = entry_enc ^ 0x9485_9D4B;
    let entry = if (0x0001_0000..0x0200_0000).contains(&entry_retail) {
        entry_retail
    } else {
        entry_debug
    };

    let section_headers_off = va_to_file_off(base, section_headers_va)?;
    let mut sections = Vec::new();
    let mut max_guest = image_size.max(0x0100_0000);
    for i in 0..section_count.min(128) {
        let off = section_headers_off + i as usize * 56;
        if off + 56 > data.len() {
            break;
        }
        let flags = read_u32(&data, off)?;
        let va = read_u32(&data, off + 0x04)?;
        let vsize = read_u32(&data, off + 0x08)?;
        let raw = read_u32(&data, off + 0x0C)?;
        let raw_size = read_u32(&data, off + 0x10)?;
        let name_va = read_u32(&data, off + 0x14)?;
        let name = read_section_name(&data, base, name_va);
        max_guest = max_guest.max(
            va.saturating_add(vsize.max(raw_size))
                .saturating_add(0x1000),
        );
        sections.push(Section {
            name,
            va,
            vsize,
            raw,
            raw_size,
            executable: (flags & 0x04) != 0,
        });
    }

    let mut guest = vec![0u8; max_guest as usize];
    let header_copy = data.len().min(0x10000).min(guest.len());
    guest[..header_copy].copy_from_slice(&data[..header_copy]);
    for s in &sections {
        if s.raw_size == 0 {
            continue;
        }
        let src = s.raw as usize;
        let len = s.raw_size as usize;
        let dst = s.va as usize;
        if src + len > data.len() || dst + len > guest.len() {
            return Err(format!(
                "section {} out of bounds: raw=0x{:X} size=0x{:X} va=0x{:08X}",
                s.name, s.raw, s.raw_size, s.va
            ));
        }
        guest[dst..dst + len].copy_from_slice(&data[src..src + len]);
    }

    let (xdk_build, d3d8_is_ltcg) =
        parse_xdk_build(&data, base, lib_versions_va, lib_versions_count)?;
    let scan_ranges = sections
        .iter()
        .filter(|s| s.executable && s.vsize > 0)
        .map(|s| OovpaScanRange {
            start: s.va,
            end: s.va.saturating_add(s.vsize),
            kind: classify_section(&s.name),
        })
        .collect::<Vec<_>>();

    Ok(XbeImage {
        guest,
        sections,
        scan_ranges,
        entry,
        xdk_build,
        d3d8_is_ltcg,
    })
}

fn read_u32(data: &[u8], off: usize) -> Result<u32, String> {
    let bytes = data
        .get(off..off + 4)
        .ok_or_else(|| format!("u32 read out of range at 0x{off:X}"))?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn va_to_file_off(base: u32, va: u32) -> Result<usize, String> {
    va.checked_sub(base)
        .map(|v| v as usize)
        .ok_or_else(|| format!("VA 0x{va:08X} is below base 0x{base:08X}"))
}

fn read_section_name(data: &[u8], base: u32, name_va: u32) -> String {
    let Ok(off) = va_to_file_off(base, name_va) else {
        return String::new();
    };
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

fn parse_xdk_build(
    data: &[u8],
    base: u32,
    lib_versions_va: u32,
    lib_versions_count: u32,
) -> Result<(u16, bool), String> {
    if lib_versions_va == 0 || lib_versions_count == 0 {
        return Ok((0, false));
    }
    let base_off = va_to_file_off(base, lib_versions_va)?;
    let mut d3d8_build = 0u16;
    let mut d3d8_is_ltcg = false;
    let mut xapi_build = 0u16;

    for i in 0..lib_versions_count.min(128) as usize {
        let off = base_off + i * 16;
        if off + 16 > data.len() {
            break;
        }
        let raw_name = &data[off..off + 8];
        let name = String::from_utf8_lossy(raw_name)
            .trim_end_matches('\0')
            .trim()
            .to_string();
        let build = u16::from_le_bytes([data[off + 12], data[off + 13]]);
        let flags = u16::from_le_bytes([data[off + 14], data[off + 15]]);
        if name.starts_with("D3D8") {
            d3d8_build = build;
            let qfe_ltcg = (flags & 0x1FFF) >= 2;
            let name_ltcg = name.eq_ignore_ascii_case("D3D8LTCG");
            d3d8_is_ltcg |= name_ltcg || qfe_ltcg;
        } else if name.starts_with("XAPILIB") && xapi_build == 0 {
            xapi_build = build;
        }
    }

    Ok((
        if d3d8_build != 0 {
            d3d8_build
        } else {
            xapi_build
        },
        d3d8_is_ltcg,
    ))
}

fn classify_section(name: &str) -> OovpaScanRangeKind {
    let upper = name.to_ascii_uppercase();
    if upper.contains("D3DX") {
        OovpaScanRangeKind::D3dx
    } else if upper.contains("D3D") {
        OovpaScanRangeKind::D3d
    } else if upper.contains("XGRPH") || upper.contains("XGRAPH") {
        OovpaScanRangeKind::Xgrph
    } else if upper.contains("DSOUND") {
        OovpaScanRangeKind::Dsound
    } else if upper.contains("XPP") || upper.contains("XAPI") || upper.contains("XINPUT") {
        OovpaScanRangeKind::Xapi
    } else {
        OovpaScanRangeKind::OtherExec
    }
}

fn parse_cxbxr_cache(path: &Path) -> Result<BTreeMap<String, BTreeSet<u32>>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut in_symbols = false;
    let mut out: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.eq_ignore_ascii_case("[Symbols]") {
            in_symbols = true;
            continue;
        }
        if line.starts_with('[') {
            in_symbols = false;
            continue;
        }
        if !in_symbols {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if let Some(addr) = parse_hex_u32(value.trim()) {
            out.entry(name.trim().to_string()).or_default().insert(addr);
        }
    }
    Ok(out)
}

fn parse_fixture_toml(path: &Path) -> Result<BTreeMap<String, BTreeSet<u32>>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut in_symbols = false;
    let mut out: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("[symbols.") {
            in_symbols = true;
            continue;
        }
        if line.starts_with('[') {
            in_symbols = false;
            continue;
        }
        if !in_symbols {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        let value = value
            .split('#')
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('"');
        if let Some(addr) = parse_hex_u32(value) {
            out.entry(name.trim().trim_matches('"').to_string())
                .or_default()
                .insert(addr);
        }
    }
    Ok(out)
}

fn parse_hex_u32(value: &str) -> Option<u32> {
    let value = value.trim().trim_matches('"');
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u32::from_str_radix(value, 16).ok()
}

fn matches_to_map(matches: &[OovpaMatch]) -> BTreeMap<String, BTreeSet<u32>> {
    let mut out = BTreeMap::new();
    for m in matches {
        out.entry(m.pattern_name.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(m.guest_addr);
    }
    out
}

fn build_report(
    sources: &Sources,
    image: &XbeImage,
    args: &Args,
    scanner_output: &Path,
) -> (String, Summary) {
    let mut names = BTreeSet::new();
    names.extend(sources.cxbxr.keys().cloned());
    names.extend(sources.fixture.keys().cloned());
    names.extend(sources.scanner.keys().cloned());

    let mut summary = Summary {
        cxbxr_symbols: sources.cxbxr.len(),
        fixture_symbols: sources.fixture.len(),
        scanner_symbols: sources.scanner.len(),
        ..Default::default()
    };

    let mut out = String::new();
    writeln!(out, "# Classic Doom OOVPA three-way probe").unwrap();
    writeln!(out, "# xbe\t{}", args.xbe.display()).unwrap();
    writeln!(out, "# cxbxr_cache\t{}", args.cxbxr_cache.display()).unwrap();
    writeln!(out, "# fixture\t{}", args.fixture.display()).unwrap();
    writeln!(out, "# scanner_output\t{}", scanner_output.display()).unwrap();
    writeln!(out, "# entry\t0x{:08X}", image.entry).unwrap();
    writeln!(out, "# xdk_build\t{}", image.xdk_build).unwrap();
    writeln!(out, "# d3d8_ltcg\t{}", image.d3d8_is_ltcg).unwrap();
    writeln!(out, "# sections\t{}", image.sections.len()).unwrap();
    writeln!(out, "# scan_ranges\t{}", image.scan_ranges.len()).unwrap();
    writeln!(
        out,
        "status\tsymbol\tcxbxr_addr\tfixture_addr\tscanner_addr"
    )
    .unwrap();

    for name in names {
        let cxbxr = sources.cxbxr.get(&name);
        let fixture = sources.fixture.get(&name);
        let scanner = sources.scanner.get(&name);
        let status = classify_status(cxbxr, fixture, scanner);
        match status {
            "agree" => summary.agree += 1,
            "disagree" => summary.disagree += 1,
            "missing_scanner" => summary.missing_scanner += 1,
            "missing_cxbxr" => summary.missing_cxbxr += 1,
            "missing_fixture" => summary.missing_fixture += 1,
            "scanner_only" => summary.scanner_only += 1,
            _ => {}
        }
        if status == "missing_scanner" && cxbxr.is_some() && fixture.is_some() && cxbxr == fixture {
            summary.fixture_cxbxr_agree_scanner_missing += 1;
        }
        writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}",
            status,
            name,
            fmt_addrs(cxbxr),
            fmt_addrs(fixture),
            fmt_addrs(scanner)
        )
        .unwrap();
    }

    writeln!(out, "\n# Summary").unwrap();
    writeln!(out, "# cxbxr_symbols\t{}", summary.cxbxr_symbols).unwrap();
    writeln!(out, "# fixture_symbols\t{}", summary.fixture_symbols).unwrap();
    writeln!(out, "# scanner_symbols\t{}", summary.scanner_symbols).unwrap();
    writeln!(out, "# agree\t{}", summary.agree).unwrap();
    writeln!(out, "# disagree\t{}", summary.disagree).unwrap();
    writeln!(out, "# missing_scanner\t{}", summary.missing_scanner).unwrap();
    writeln!(out, "# missing_cxbxr\t{}", summary.missing_cxbxr).unwrap();
    writeln!(out, "# missing_fixture\t{}", summary.missing_fixture).unwrap();
    writeln!(out, "# scanner_only\t{}", summary.scanner_only).unwrap();
    writeln!(
        out,
        "# fixture_cxbxr_agree_scanner_missing\t{}",
        summary.fixture_cxbxr_agree_scanner_missing
    )
    .unwrap();

    (out, summary)
}

fn classify_status(
    cxbxr: Option<&BTreeSet<u32>>,
    fixture: Option<&BTreeSet<u32>>,
    scanner: Option<&BTreeSet<u32>>,
) -> &'static str {
    match (cxbxr, fixture, scanner) {
        (Some(c), Some(f), Some(s)) if c == f && f == s => "agree",
        (Some(_), Some(_), None) => "missing_scanner",
        (Some(_), None, Some(_)) => "missing_fixture",
        (None, Some(_), Some(_)) => "missing_cxbxr",
        (Some(_), None, None) => "missing_fixture_scanner",
        (None, Some(_), None) => "missing_cxbxr_scanner",
        (None, None, Some(_)) => "scanner_only",
        (Some(_), Some(_), Some(_)) => "disagree",
        (None, None, None) => unreachable!(),
    }
}

fn fmt_addrs(addrs: Option<&BTreeSet<u32>>) -> String {
    let Some(addrs) = addrs else {
        return "-".to_string();
    };
    addrs
        .iter()
        .map(|a| format!("0x{a:08X}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn write_scanner_output(
    path: &Path,
    image: &XbeImage,
    matches: &[OovpaMatch],
) -> Result<(), String> {
    let mut out = String::new();
    let unique = matches_to_map(matches);
    writeln!(out, "# Classic Doom scanner-only OOVPA output").unwrap();
    writeln!(
        out,
        "# Generated by doom_oovpa_probe from Rustemu scanner code."
    )
    .unwrap();
    writeln!(
        out,
        "# Duplicate scanner names are emitted with __scanner_dup_N suffixes."
    )
    .unwrap();
    writeln!(out, "[meta]").unwrap();
    writeln!(out, "title = \"CLASSIC DOOM\"").unwrap();
    writeln!(out, "entry_point = \"0x{:08X}\"", image.entry).unwrap();
    writeln!(out, "xdk_build = {}", image.xdk_build).unwrap();
    writeln!(out, "d3d8_ltcg = {}", image.d3d8_is_ltcg).unwrap();
    writeln!(out, "raw_match_count = {}", matches.len()).unwrap();
    writeln!(out, "unique_symbol_count = {}", unique.len()).unwrap();
    writeln!(out, "\n[symbols.\"scanner\"]").unwrap();
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for m in matches {
        let count = seen.entry(m.pattern_name).or_insert(0);
        *count += 1;
        let key = if *count == 1 {
            m.pattern_name.to_string()
        } else {
            format!("{}__scanner_dup_{}", m.pattern_name, count)
        };
        if *count == 1 {
            writeln!(out, "{} = \"0x{:X}\"", toml_key(&key), m.guest_addr).unwrap();
        } else {
            writeln!(
                out,
                "{} = \"0x{:X}\" # original={}",
                toml_key(&key),
                m.guest_addr,
                m.pattern_name
            )
            .unwrap();
        }
    }
    std::fs::write(path, out).map_err(|e| format!("write {}: {e}", path.display()))
}

fn toml_key(name: &str) -> String {
    if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
    }
}
