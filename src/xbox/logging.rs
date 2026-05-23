/// Structured logging backend for rustemu-jit.
///
/// Replaces the old per-message file-opening `debug_log()` with:
/// - Buffered file output (desktop dev, grep-friendly)
/// - RetroArch log callback (works on Xbox UWP)
/// - Per-module level filtering (kernel, veh, gpu, jit, etc.)
/// - Kernel call ring buffer with ESP tracking (crash diagnostics)
/// - Shadow stack visualizer (R12 chain dump)
/// - Guest address symbolication from xbox_scout_v2.db
use std::io::Write;
use std::sync::Mutex;

// ============================================================================
// RetroArch log callback type
// ============================================================================

/// RetroArch log level (matches libretro.h enum retro_log_level)
#[repr(u32)]
#[allow(dead_code)]
enum RetroLogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

/// Function pointer type for retro_log_printf_t.
/// Signature: void (*)(enum retro_log_level level, const char *fmt, ...)
/// We call it with a single pre-formatted string (no varargs from Rust).
pub type RetroLogPrintfT = unsafe extern "C" fn(level: u32, fmt: *const i8);

// ============================================================================
// Logger backend state
// ============================================================================

struct LogBackend {
    /// Buffered file writer — opened once at init, kept alive.
    file: Option<std::io::BufWriter<std::fs::File>>,
    /// RetroArch log callback — set via `set_retro_callback`.
    retro_cb: Option<RetroLogPrintfT>,
}

static BACKEND: Mutex<Option<LogBackend>> = Mutex::new(None);

// ============================================================================
// Initialization
// ============================================================================

/// Initialize the logging backend. Call once at retro_load_game.
/// `log_path`: file path for debug log (None = no file output, e.g. UWP).
pub fn init(log_path: Option<&str>) {
    let file = log_path.and_then(|p| {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .ok()
            .map(|f| std::io::BufWriter::new(f))
    });

    let backend = LogBackend {
        file,
        retro_cb: None,
    };

    *BACKEND.lock().unwrap() = Some(backend);

    // Install our custom logger as the `log` crate backend.
    static LOGGER: EmuLogger = EmuLogger;
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
}

/// Set the RetroArch log callback (from RETRO_ENVIRONMENT_GET_LOG_INTERFACE).
pub fn set_retro_callback(cb: RetroLogPrintfT) {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(ref mut backend) = *guard {
            backend.retro_cb = Some(cb);
        }
    }
}

/// Flush the file backend. Call at retro_unload_game / retro_deinit.
pub fn flush() {
    if let Ok(mut guard) = BACKEND.lock() {
        if let Some(ref mut backend) = *guard {
            if let Some(ref mut f) = backend.file {
                let _ = f.flush();
            }
        }
    }
}

// ============================================================================
// log crate backend implementation
// ============================================================================

struct EmuLogger;

impl log::Log for EmuLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true // filtering done at write time
    }

    fn log(&self, record: &log::Record) {
        let level_tag = match record.level() {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN",
            log::Level::Info => "INFO",
            log::Level::Debug => "DEBUG",
            log::Level::Trace => "TRACE",
        };

        let target = record.target();
        // Format: [rustemu][MODULE][LEVEL] message
        let msg = format!("[rustemu][{}][{}] {}\n", target, level_tag, record.args());

        if let Ok(mut guard) = BACKEND.lock() {
            if let Some(ref mut backend) = *guard {
                // If RetroArch callback is available, prefer it and SKIP the file
                // backend entirely. The host-file path is a hot spot for NtWriteFile
                // backpressure — on a crash-recovery loop the worker can park 2+
                // seconds inside a syscall waiting for disk flush. The libretro
                // log callback is a simple ring-buffer push and does not block.
                if let Some(cb) = backend.retro_cb {
                    let retro_level = match record.level() {
                        log::Level::Error => RetroLogLevel::Error as u32,
                        log::Level::Warn => RetroLogLevel::Warn as u32,
                        log::Level::Info => RetroLogLevel::Info as u32,
                        _ => RetroLogLevel::Debug as u32,
                    };
                    // RetroArch expects null-terminated C string.
                    if let Ok(cstr) = std::ffi::CString::new(msg.as_str()) {
                        unsafe {
                            cb(retro_level, cstr.as_ptr());
                        }
                    }
                } else if let Some(ref mut f) = backend.file {
                    // No RetroArch callback — fall back to buffered file. Only
                    // active during pre-init and when the callback is absent.
                    let _ = f.write_all(msg.as_bytes());
                    // Flush on warn/error so crash logs aren't lost
                    if record.level() <= log::Level::Warn {
                        let _ = f.flush();
                    }
                }
            }
        } else {
            // Mutex poisoned — fallback to stderr
            eprint!("{}", msg);
        }
    }

    fn flush(&self) {
        self::flush();
    }
}

// ============================================================================
// Kernel call ring buffer (crash diagnostics)
// ============================================================================

/// Single entry in the kernel call ring buffer.
#[derive(Clone)]
pub struct KernelCallEntry {
    pub ordinal: u32,
    pub name: &'static str,
    pub esp_before: u32,
    pub esp_after: u32,
    pub eax_ret: u32,
    pub args: [u32; 4],
}

/// Fixed-size ring buffer of recent kernel calls.
pub struct KernelRingBuffer {
    entries: Vec<KernelCallEntry>,
    capacity: usize,
    write_pos: usize,
    count: usize,
}

impl KernelRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
            write_pos: 0,
            count: 0,
        }
    }

    /// Record a kernel call. O(1), no allocation after warmup.
    pub fn push(&mut self, entry: KernelCallEntry) {
        if self.entries.len() < self.capacity {
            self.entries.push(entry);
        } else {
            self.entries[self.write_pos] = entry;
        }
        self.write_pos = (self.write_pos + 1) % self.capacity;
        self.count += 1;
    }

    /// Iterate entries oldest-first.
    pub fn iter_oldest_first(&self) -> impl Iterator<Item = &KernelCallEntry> {
        let len = self.entries.len();
        let start = if len < self.capacity {
            0
        } else {
            self.write_pos
        };
        (0..len).map(move |i| &self.entries[(start + i) % len])
    }

    /// Dump the ring buffer contents as formatted lines.
    pub fn dump(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "=== Kernel Call Ring Buffer (last {} of {} total) ===",
            self.entries.len(),
            self.count
        ));
        for (i, e) in self.iter_oldest_first().enumerate() {
            let esp_delta = e.esp_after as i64 - e.esp_before as i64;
            let drift_tag = if esp_delta != 0 {
                format!(" *** DRIFT {:+}", esp_delta)
            } else {
                String::new()
            };
            lines.push(format!(
                "  [{}] #{} {} ESP:{:08X}->{:08X}{} EAX={:08X} args=[{:08X},{:08X},{:08X},{:08X}]",
                i,
                e.ordinal,
                e.name,
                e.esp_before,
                e.esp_after,
                drift_tag,
                e.eax_ret,
                e.args[0],
                e.args[1],
                e.args[2],
                e.args[3],
            ));
        }
        lines.push("=== End Ring Buffer ===".to_string());
        lines
    }
}

/// Global ring buffer — 256 entries, accessible from kernel dispatch + crash dump.
static KERNEL_RING: Mutex<Option<KernelRingBuffer>> = Mutex::new(None);

/// Initialize the kernel ring buffer. Call once at emulator init.
pub fn init_ring_buffer(capacity: usize) {
    *KERNEL_RING.lock().unwrap() = Some(KernelRingBuffer::new(capacity));
}

/// Record a kernel call in the ring buffer.
pub fn record_kernel_call(entry: KernelCallEntry) {
    if let Ok(mut guard) = KERNEL_RING.lock() {
        if let Some(ref mut ring) = *guard {
            ring.push(entry);
        }
    }
}

/// Dump the kernel ring buffer (for crash reports).
pub fn dump_kernel_ring() -> Vec<String> {
    if let Ok(guard) = KERNEL_RING.lock() {
        if let Some(ref ring) = *guard {
            return ring.dump();
        }
    }
    vec!["(ring buffer not initialized)".to_string()]
}

// ============================================================================
// Shadow stack visualizer (R12 chain dump)
// ============================================================================

/// Dump the R12 shadow stack as symbolicated entries.
/// `r12_ptr`: current R12 value (top of shadow stack, grows downward).
/// `r12_top`: allocation top (highest valid address).
/// `code_base`: host code buffer base for reverse_lookup.
/// `max_entries`: max entries to dump.
/// `symbolicate`: closure that maps guest_addr -> Option<function name>.
pub fn dump_shadow_chain<F>(
    r12_ptr: u64,
    r12_top: u64,
    code_base: u64,
    max_entries: usize,
    symbolicate: F,
) -> Vec<String>
where
    F: Fn(u32) -> Option<String>,
{
    let mut lines = Vec::new();
    let depth = if r12_top >= r12_ptr {
        ((r12_top - r12_ptr) / 8) as usize
    } else {
        0
    };
    lines.push(format!(
        "=== Shadow Stack (R12) — {} entries, showing up to {} ===",
        depth, max_entries
    ));

    let mut addr = r12_ptr;
    for i in 0..max_entries.min(depth) {
        let host_ret = unsafe { *(addr as *const u64) };
        // The shadow stack stores host return addresses.
        // We need to convert host -> guest for symbolication.
        let host_offset = if host_ret >= code_base {
            (host_ret - code_base) as u32
        } else {
            0
        };

        let sym = symbolicate(host_offset).unwrap_or_else(|| "???".to_string());
        lines.push(format!(
            "  [{}] host={:016X} offset={:08X} => {}",
            i, host_ret, host_offset, sym
        ));
        addr += 8;
    }

    if depth > max_entries {
        lines.push(format!("  ... ({} more entries)", depth - max_entries));
    }
    lines.push("=== End Shadow Stack ===".to_string());
    lines
}

// ============================================================================
// Guest address symbolication (from xbox_scout_v2.db)
// ============================================================================

/// In-memory symbol table loaded from a Scout export at init.
pub struct SymbolTable {
    /// Sorted by start address for binary search.
    symbols: Vec<Symbol>,
}

struct Symbol {
    start: u32,
    end: u32,
    name: String,
}

impl SymbolTable {
    /// Create an empty symbol table (no DB available).
    pub fn empty() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    /// Load function symbols from a Scout CSV export.
    /// Falls back to empty if DB doesn't exist or can't be read.
    pub fn from_csv(path: &str) -> Self {
        match Self::try_load(path) {
            Ok(table) => {
                log::info!(target: "symbols", "Loaded {} symbols from {}", table.symbols.len(), path);
                table
            }
            Err(e) => {
                log::warn!(target: "symbols", "Could not load symbol CSV {}: {}", path, e);
                Self::empty()
            }
        }
    }

    fn try_load(path: &str) -> Result<Self, String> {
        // Load from a pre-exported CSV so the emulator has no runtime SQLite
        // dependency. Supported rows:
        //   addr,name
        //   addr,end_addr,name
        // Address fields may be decimal or 0x-prefixed hex.
        let content = std::fs::read_to_string(path).map_err(|e| format!("read failed: {}", e))?;

        let mut symbols = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.splitn(3, ',').map(str::trim).collect();
            if parts.len() < 2 || parts[0].eq_ignore_ascii_case("addr") {
                continue;
            }

            let Some(start) = parse_symbol_addr(parts[0]) else {
                continue;
            };

            let (end, name) = if parts.len() >= 3 {
                let end = parse_symbol_addr(parts[1]).unwrap_or(0);
                (end, parts[2])
            } else {
                (0, parts[1])
            };

            if !name.is_empty() {
                symbols.push(Symbol {
                    start,
                    end,
                    name: name.to_string(),
                });
            }
        }

        symbols.sort_by_key(|sym| sym.start);
        symbols.dedup_by_key(|sym| sym.start);

        // Older symbols.csv exports only carried function starts. Infer a
        // conservative range from the next function start so lookup(addr)
        // means "containing function", not just "nearest lower label".
        for i in 0..symbols.len() {
            let next_start = symbols.get(i + 1).map(|sym| sym.start);
            let sym = &mut symbols[i];
            if sym.end <= sym.start {
                sym.end = next_start.unwrap_or_else(|| sym.start.saturating_add(0x1_0000));
            }
        }

        Ok(Self { symbols })
    }

    /// Look up the nearest symbol at or before `addr`.
    /// Returns "func_name+0xNN" or None if no symbols loaded.
    pub fn lookup(&self, addr: u32) -> Option<String> {
        if self.symbols.is_empty() {
            return None;
        }
        // Binary search for nearest-lower.
        let idx = match self.symbols.binary_search_by_key(&addr, |sym| sym.start) {
            Ok(i) => i,
            Err(0) => return None,
            Err(i) => i - 1,
        };
        let sym = &self.symbols[idx];
        if addr >= sym.end {
            return None;
        }
        let offset = addr - sym.start;
        if offset == 0 {
            Some(sym.name.clone())
        } else {
            Some(format!("{}+0x{:X}", sym.name, offset))
        }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }
}

fn parse_symbol_addr(s: &str) -> Option<u32> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

/// Global symbol table — loaded once at init.
static SYMBOL_TABLE: Mutex<Option<SymbolTable>> = Mutex::new(None);

/// Initialize the symbol table from a Scout CSV export.
pub fn init_symbols(path: &str) {
    let table = SymbolTable::from_csv(path);
    *SYMBOL_TABLE.lock().unwrap() = Some(table);
}

/// Disable guest symbolication for the current title.
pub fn clear_symbols() {
    *SYMBOL_TABLE.lock().unwrap() = Some(SymbolTable::empty());
}

/// Select a title-appropriate Scout CSV. `RUSTEMU_SYMBOLS_CSV` overrides the
/// automatic choice; use `0`, `none`, or `off` to force-disable it.
pub fn init_symbols_for_game(xbe_path: &str, title: &str) {
    if let Ok(path) = std::env::var("RUSTEMU_SYMBOLS_CSV") {
        let trimmed = path.trim();
        if trimmed.is_empty()
            || trimmed == "0"
            || trimmed.eq_ignore_ascii_case("none")
            || trimmed.eq_ignore_ascii_case("off")
            || trimmed.eq_ignore_ascii_case("false")
        {
            clear_symbols();
            log::info!(target: "symbols", "Guest symbolication disabled by RUSTEMU_SYMBOLS_CSV");
        } else {
            init_symbols(trimmed);
        }
        return;
    }

    let marker = format!("{} {}", xbe_path, title).to_ascii_lowercase();
    if marker.contains("doom") {
        init_symbols(r"./doom_symbols.csv");
    } else if marker.contains("spider") {
        init_symbols(r"./symbols.csv");
    } else {
        clear_symbols();
        log::info!(
            target: "symbols",
            "No Scout symbols selected for title '{}' ({})",
            title,
            xbe_path
        );
    }
}

/// Look up a guest address in the global symbol table.
pub fn symbolicate(addr: u32) -> Option<String> {
    if let Ok(guard) = SYMBOL_TABLE.lock() {
        if let Some(ref table) = *guard {
            return table.lookup(addr);
        }
    }
    None
}

/// Format a guest address with symbol annotation.
/// Returns "0x002A52A0 (game_main)" or "0x002A52A0" if no symbol.
pub fn fmt_guest_addr(addr: u32) -> String {
    match symbolicate(addr) {
        Some(sym) => format!("0x{:08X} ({})", addr, sym),
        None => format!("0x{:08X}", addr),
    }
}

// ============================================================================
// Crash dump reporter
// ============================================================================

/// Full crash dump: ring buffer + shadow stack + registers.
/// Call from VEH or dispatch loop on AOT_EXIT_ERROR.
/// `addr_hash`: optional — when provided, shadow-stack entries are annotated
///              with the guest PC they were compiled from (via reverse_lookup).
pub fn crash_dump(
    reason: &str,
    guest_addr: u32,
    r12_ptr: u64,
    r12_top: u64,
    code_base: u64,
    esp: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    addr_hash: Option<&crate::xbox::aot::runtime::AddrHash>,
) {
    log::error!(target: "crash", "========== CRASH DUMP ==========");
    log::error!(target: "crash", "Reason: {}", reason);
    log::error!(target: "crash", "Guest addr: {}", fmt_guest_addr(guest_addr));
    log::error!(target: "crash", "ESP={:08X} EAX={:08X} EBX={:08X} ECX={:08X} EDX={:08X}",
        esp, eax, ebx, ecx, edx);

    // Dump kernel ring buffer
    for line in dump_kernel_ring() {
        log::error!(target: "crash", "{}", line);
    }

    // Dump shadow stack — annotate with nearest guest PC when addr_hash available.
    let chain = dump_shadow_chain(
        r12_ptr,
        r12_top,
        code_base,
        32,
        |host_off| match addr_hash {
            Some(h) => {
                let gpc = h.reverse_lookup(host_off);
                if gpc != 0 {
                    Some(format!("host+0x{:08X} (guest≈0x{:08X})", host_off, gpc))
                } else {
                    Some(format!("host+0x{:08X}", host_off))
                }
            }
            None => Some(format!("host+0x{:08X}", host_off)),
        },
    );
    for line in chain {
        log::error!(target: "crash", "{}", line);
    }

    log::error!(target: "crash", "========== END CRASH DUMP ==========");

    // Force flush
    flush();
}
