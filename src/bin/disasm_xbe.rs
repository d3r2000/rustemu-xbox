use iced_x86::{Decoder, DecoderOptions, Formatter, IntelFormatter};

fn read_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(data[off..off + 4].try_into().unwrap())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: disasm_xbe <xbe> <va_hex> [bytes]");
        std::process::exit(2);
    }
    let path = &args[1];
    let va = u32::from_str_radix(args[2].trim_start_matches("0x"), 16).unwrap();
    let count = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(128);
    let data = std::fs::read(path).unwrap();
    let base = read_u32(&data, 0x104);
    let num_sections = read_u32(&data, 0x11c);
    let sh_va = read_u32(&data, 0x120);
    let sh_off = sh_va.wrapping_sub(base) as usize;

    let mut file_off = None;
    for i in 0..num_sections {
        let off = sh_off + i as usize * 56;
        let sect_va = read_u32(&data, off + 0x04);
        let vsize = read_u32(&data, off + 0x08);
        let raw = read_u32(&data, off + 0x0c);
        let raw_size = read_u32(&data, off + 0x10);
        let span = vsize.max(raw_size);
        if va >= sect_va && va < sect_va.wrapping_add(span) {
            file_off = Some(raw.wrapping_add(va.wrapping_sub(sect_va)) as usize);
            break;
        }
    }
    let file_off = file_off.unwrap_or_else(|| va.wrapping_sub(base) as usize);
    let end = (file_off + count).min(data.len());
    println!(
        "path={} va=0x{:08X} file_off=0x{:X} bytes={}",
        path,
        va,
        file_off,
        end.saturating_sub(file_off)
    );
    let mut decoder = Decoder::with_ip(32, &data[file_off..end], va as u64, DecoderOptions::NONE);
    let mut fmt = IntelFormatter::new();
    let mut out = String::new();
    while decoder.can_decode() {
        let instr = decoder.decode();
        out.clear();
        fmt.format(&instr, &mut out);
        println!("{:08X}: {:<32} ; {}", instr.ip() as u32, out, instr.len());
    }
}
