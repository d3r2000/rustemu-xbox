pub mod file;
mod hal;
mod memory;
/// Xbox kernel ordinal dispatcher.
/// Centralized dispatch for kernel calls (0xFFFF0000+ordinal).
///
/// Reads stdcall args from guest stack, dispatches to subsystem handlers,
/// performs stdcall cleanup (ESP += arg_count * 4), sets EAX return value.
/// Tracks per-ordinal hit counts for debugging.
pub mod ordinals;

// Re-export synthetic-injection helpers used by worker/emulator reinjection paths.
pub use memory::{pool_bump_alloc, va_bump_alloc_guest};

use crate::xbox::memory::guest_memory::GuestMemory;
use std::collections::HashMap;

pub const KERNEL_MAGIC_BASE: u32 = 0xFFFF0000;

/// Xbox kernel object types (matches DISPATCHER_HEADER.Type).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XboxObjectType {
    Event,     // NotificationEvent=0, SynchronizationEvent=1
    Mutant,    // Type=2
    Semaphore, // Type=5
    Timer,     // Type=8/9
    Thread,    // Pseudo — for NtCurrentThread handle
    Directory, // For NtCreateDirectoryObject
    IoCompletion,
}

/// A tracked kernel object backed by a real Win32 handle.
pub struct KernelObject {
    pub obj_type: XboxObjectType,
    /// Real Win32 HANDLE (from CreateEventW / CreateMutexW / etc.)
    pub native_handle: usize,
    /// Reference count (starts at 1).
    pub ref_count: i32,
    /// Guest memory address of the object body (for ObReferenceObjectByHandle).
    pub guest_obj_addr: u32,
}

impl Drop for KernelObject {
    fn drop(&mut self) {
        if self.native_handle != 0 {
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(
                    windows::Win32::Foundation::HANDLE(self.native_handle as _),
                );
            }
        }
    }
}

/// Result of a kernel call dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KernelResult {
    Handled,    // Normal return — continue execution
    Halt,       // Halt execution (HalReturnToFirmware, KeBugCheck)
    NotHandled, // Unknown ordinal — stub it
    SpawnThread {
        // Create real OS thread for worker
        entry: u32,         // Thread entry (SystemRoutine, e.g. XapiThreadStartup)
        start_routine: u32, // StartRoutine — actual game function (1st arg to SystemRoutine)
        context: u32,       // StartContext — passed as 2nd arg to SystemRoutine
        thread_handle: u32, // Guest handle for the created thread object
    },
    ThreadExit,  // PsTerminateSystemThread — stop current thread (guest_addr → 0)
    QuickReboot, // HalReturnToFirmware(2) — re-enter XBE from entry point (Cxbx-R style)
}

/// Mutable kernel state (allocators, counters, fake handles).
pub struct KernelState {
    pub bump_pool: u32, // Pool allocator cursor (ExAllocatePool, MmAllocateSystemMemory)
    pub bump_contig: u32, // Contiguous allocator cursor (MmAllocateContiguous*)
    pub bump_va: u32,   // Virtual memory allocator cursor
    pub next_handle: u32, // Fake handle counter
    pub perf_counter: u64, // Fake performance counter
    pub system_time: u64, // Fake system time
    /// Per-ordinal hit count for debugging.
    pub hit_counts: Vec<u32>,
    /// True once the first worker thread has been launched (only one worker supported).
    pub worker_launched: bool,
    /// Raw XBE file data — needed for XeLoadSection to copy section data on demand.
    pub xbe_file_data: Vec<u8>,
    /// Certificate metadata for EEPROM-style region queries.
    pub xbe_title_id: u32,
    pub xbe_allowed_media: u32,
    pub xbe_game_region: u32,
    /// File I/O state — handle table + path translation.
    pub file_state: Option<file::FileState>,
    /// Rolling log of last N kernel calls for panic diagnostics.
    pub call_log: Vec<String>,
    /// Max entries in call_log ring buffer.
    pub call_log_max: usize,
    /// MmAllocateContiguousMemoryEx call counter (for log throttling).
    pub mm_contig_count: u32,
    /// Kernel thunk table address range (for re-resolution after XeLoadSection).
    /// (thunk_addr, base_address, image_size)
    pub thunk_info: Option<(u32, u32, u32)>,
    /// Object manager: guest handle → KernelObject (backed by real Win32 handles).
    /// Cxbx-R pattern: NtCreateEvent→CreateEventW, NtWait→WaitForMultipleObjects.
    pub object_table: HashMap<u32, KernelObject>,
    /// Reverse map: guest object body address → guest handle.
    /// Used by Ke* functions that take object pointers instead of handles.
    pub obj_addr_to_handle: HashMap<u32, u32>,
    /// Bump allocator for guest object body memory (for ObReferenceObjectByHandle).
    /// Region 0x00F00000..0x00F80000 — 512KB for object headers in guest space.
    pub obj_body_cursor: u32,
    /// Queued thread entry points for cooperative single-threaded scheduling.
    /// When PsCreateSystemThreadEx is called beyond the first worker, the entry
    /// is queued here. The worker picks these up between reinject cycles.
    pub thread_queue: Vec<(u32, u32, u32)>, // (entry, start_routine, context)
    /// Total kernel calls (all ordinals). Used for CRT boot diagnostics.
    pub total_kernel_calls: u64,
    /// One-shot: true after we've logged the _crtheap probe.
    pub crtheap_probed: bool,
}

impl KernelState {
    pub fn new() -> Self {
        Self {
            bump_pool: 0x8400_0000,   // must match memory::BUMP_BASE
            bump_contig: 0x80D0_0000, // must match memory::CONTIG_BASE
            bump_va: 0x0400_0000,     // must match memory::VA_BUMP_BASE (above contiguous alias)
            next_handle: 0x1000,
            perf_counter: 0,
            system_time: 132_000_000_000_000_000,
            hit_counts: vec![0u32; ordinals::MAX_ORDINAL],
            worker_launched: false,
            xbe_file_data: Vec::new(),
            xbe_title_id: 0,
            xbe_allowed_media: 0,
            xbe_game_region: 1,
            file_state: None,
            call_log: Vec::with_capacity(64),
            call_log_max: 50,
            mm_contig_count: 0,
            thunk_info: None,
            object_table: HashMap::new(),
            obj_addr_to_handle: HashMap::new(),
            obj_body_cursor: 0x00F0_0000,
            thread_queue: Vec::new(),
            total_kernel_calls: 0,
            crtheap_probed: false,
        }
    }

    pub fn with_xbe_data(
        xbe_data: Vec<u8>,
        xbe_path: &str,
        system_dir: &str,
        title_id: u32,
        allowed_media: u32,
        game_region: u32,
    ) -> Self {
        let mut s = Self::new();
        s.xbe_file_data = xbe_data;
        s.xbe_title_id = title_id;
        s.xbe_allowed_media = allowed_media;
        s.xbe_game_region = if game_region == 0 { 1 } else { game_region };
        let mut fs = file::FileState::new(xbe_path, system_dir);
        // Pre-register standard Xbox drive symlinks.
        // Games mount title-specific user data drives through XAPI; do not
        // pre-map U: here or save scans hit the raw UDATA root.
        fs.add_symlink("\\??\\D:", "\\Device\\CdRom0");
        fs.add_symlink("\\??\\T:", "\\Device\\Harddisk0\\partition1\\TDATA");
        fs.add_symlink("\\??\\Z:", "\\Device\\Harddisk0\\partition6");
        // Create cache directories
        let sys_dir = fs.system_directory.clone();
        let _ = std::fs::create_dir_all(format!("{}xbox/EmuDisk/partition6/temp", sys_dir));
        s.file_state = Some(fs);
        s
    }

    /// Build ordinal hit summary — only ordinals that were actually called.
    /// Returns lines for the caller to write to their preferred log output.
    pub fn hit_summary_lines(&self) -> Vec<String> {
        let mut hits: Vec<(u32, u32)> = self
            .hit_counts
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .map(|(ord, &count)| (ord as u32, count))
            .collect();
        hits.sort_by(|a, b| b.1.cmp(&a.1)); // descending by count

        let mut lines = Vec::new();
        lines.push(format!(
            "=== Kernel Ordinal Hit Summary ({} unique ordinals) ===",
            hits.len()
        ));
        for (ord, count) in &hits {
            let name = ordinals::name(*ord);
            let argc = ordinals::arg_count(*ord);
            let data = if ordinals::is_data(*ord) {
                " [DATA]"
            } else {
                ""
            };
            lines.push(format!(
                "  #{:>3} {:40} hits={:<8} argc={}{}",
                ord, name, count, argc, data
            ));
        }
        lines.push("=== End Ordinal Summary ===".to_string());
        lines
    }

    /// Create a tracked kernel object backed by a real Win32 handle.
    /// Returns the guest handle value written to the output pointer.
    pub fn create_object(&mut self, obj_type: XboxObjectType, native_handle: usize) -> u32 {
        self.next_handle += 1;
        let guest_handle = self.next_handle;
        // Allocate a 0x20-byte guest object body for ObReferenceObjectByHandle
        let guest_addr = self.obj_body_cursor;
        self.obj_body_cursor += 0x20;
        self.object_table.insert(
            guest_handle,
            KernelObject {
                obj_type,
                native_handle,
                ref_count: 1,
                guest_obj_addr: guest_addr,
            },
        );
        self.obj_addr_to_handle.insert(guest_addr, guest_handle);
        guest_handle
    }

    /// Create a tracked object from a guest-memory pointer (for KeInitializeEvent etc.)
    /// The game allocates the object body; we just create a Win32 backing object.
    pub fn create_object_at_addr(
        &mut self,
        obj_type: XboxObjectType,
        native_handle: usize,
        guest_addr: u32,
    ) -> u32 {
        self.next_handle += 1;
        let guest_handle = self.next_handle;
        self.object_table.insert(
            guest_handle,
            KernelObject {
                obj_type,
                native_handle,
                ref_count: 1,
                guest_obj_addr: guest_addr,
            },
        );
        self.obj_addr_to_handle.insert(guest_addr, guest_handle);
        guest_handle
    }

    /// Look up native Win32 handle from guest handle.
    pub fn get_native_handle(&self, guest_handle: u32) -> Option<usize> {
        self.object_table
            .get(&guest_handle)
            .map(|o| o.native_handle)
    }

    /// Look up native Win32 handle from guest object body address (for Ke* functions).
    pub fn get_native_handle_by_addr(&self, guest_addr: u32) -> Option<usize> {
        self.obj_addr_to_handle
            .get(&guest_addr)
            .and_then(|h| self.object_table.get(h))
            .map(|o| o.native_handle)
    }

    /// Look up guest object body address from guest handle.
    pub fn get_object_addr(&self, guest_handle: u32) -> Option<u32> {
        self.object_table
            .get(&guest_handle)
            .map(|o| o.guest_obj_addr)
    }
}

/// Dispatch a kernel call. Reads args from guest stack, calls subsystem handler,
/// performs stdcall cleanup, returns (KernelResult, eax_value).
pub fn dispatch_kernel_call(
    ordinal: u32,
    memory: &GuestMemory,
    guest_esp: &mut u32,
    guest_eax: &mut u32,
    guest_edx: &mut u32,
    guest_ecx: &mut u32,
    state: &mut KernelState,
) -> KernelResult {
    // Reject bogus ordinals — 0xFFFF comes from corrupted jump targets
    // (e.g. game struct function pointers in uninitialized heap memory).
    // These are not real kernel calls.
    if ordinal >= 0x8000 {
        static BOGUS_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = BOGUS_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 5 {
            crate::xbox::emulator::debug_log(&format!(
                "[KERNEL-BOGUS] ordinal={} (0x{:X}) is not a valid Xbox kernel ordinal — corrupted jump target",
                ordinal, ordinal
            ));
        }
        return KernelResult::NotHandled;
    }

    let _cpu_phase = crate::xbox::profiler::guard(crate::xbox::profiler::CpuPhase::KernelDispatch);
    let _kernel_subphase = crate::xbox::profiler::kernel_ordinal_guard(ordinal);

    // Track hit
    if (ordinal as usize) < state.hit_counts.len() {
        state.hit_counts[ordinal as usize] += 1;
    }
    state.total_kernel_calls += 1;

    // CRT boot diagnostic — shared with dispatch_inline (same check points)
    if state.total_kernel_calls == 500
        || state.total_kernel_calls == 900
        || state.total_kernel_calls == 1500
    {
        let crtheap = memory.read_u32(0x007E_1584);
        let active_heap = memory.read_u32(0x007E_1C4C);
        let process_heap = memory.read_u32(0x007E_18B0);
        crate::xbox::emulator::debug_log(&format!(
            "[CRT-PROBE] @{} calls: _crtheap=0x{:08X} __active_heap=0x{:08X} process_heap=0x{:08X}",
            state.total_kernel_calls, crtheap, active_heap, process_heap
        ));
        if crtheap != 0 && crtheap != 0x0200_0000 {
            crate::xbox::emulator::debug_log(&format!(
                "[CRT-PROBE] SUCCESS: _crtheap=0x{:08X} — CRT heap initialized organically!",
                crtheap
            ));
        }
    }

    // Data exports — just return the address (no call convention)
    if ordinals::is_data(ordinal) {
        log::trace!(
            "kernel: data export #{} {} -> 0",
            ordinal,
            ordinals::name(ordinal)
        );
        *guest_eax = 0;
        return KernelResult::Handled;
    }

    // ESP-DRIFT INSTRUMENTATION: capture entry ESP for full-call drift check.
    let esp_before = *guest_esp;

    // Read up to 12 stdcall args from guest stack (after return address)
    let mut args = [0u32; 12];
    let arg_count = ordinals::arg_count(ordinal);
    for i in 0..arg_count.min(12) {
        args[i as usize] = memory.read_u32(*guest_esp + 4 + i * 4);
    }

    // ----------------------------------------------------------------------
    // PHASE 4 · Matched-transition push (Frida/Wine-style log-only).
    // If the top of the transition stack is already a Kernel frame for
    // this exact ordinal with matching entry_esp, then kernel_bridge
    // (the trampoline-exit path) already pushed — don't double-push.
    // Otherwise this is the interpreter path (worker.rs) which has no
    // bridge, so we push here.
    // ----------------------------------------------------------------------
    let entry_esp = *guest_esp;
    let ret_addr_on_stack = memory.read_u32(*guest_esp);
    let bridge_already_pushed = {
        use crate::xbox::aot::transition;
        matches!(
            transition::peek(),
            Some(f) if f.kind == transition::TransitionKind::Kernel
                && f.ordinal == ordinal as u16
                && f.entry_r14 == entry_esp
        )
    };
    if !bridge_already_pushed {
        use crate::xbox::aot::transition;
        transition::push(transition::TransitionFrame::new_kernel(
            ordinal as u16,
            ordinals::name(ordinal),
            entry_esp,
            0, // guest_entry_pc unknown at this layer (kernel is Rust code)
            ret_addr_on_stack,
            arg_count as u8,
        ));
    }

    // Log first few calls of each ordinal
    let hits = state.hit_counts.get(ordinal as usize).copied().unwrap_or(0);
    if hits <= 3 {
        log::debug!(
            "kernel: #{} {} argc={} args=[{:#X}, {:#X}, {:#X}, {:#X}]",
            ordinal,
            ordinals::name(ordinal),
            arg_count,
            args[0],
            args[1],
            args[2],
            args[3]
        );
    }

    // Diagnostic: dump raw stack for MmAllocateContiguousMemoryEx (ord 166)
    if ordinal == 166 {
        let esp = *guest_esp;
        let raw: Vec<u32> = (0..8).map(|i| memory.read_u32(esp + i * 4)).collect();
        crate::xbox::emulator::debug_log(&format!(
            "[DIAG-166] ESP=0x{:08X} raw_stack=[0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}] args=[0x{:X}, 0x{:X}, 0x{:X}, 0x{:X}, 0x{:X}]",
            esp, raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            args[0], args[1], args[2], args[3], args[4]
        ));
    }

    // ESP-DRIFT INSTRUMENTATION: capture before dispatch so we can detect
    // handlers that unexpectedly mutate guest_esp on top of our own cleanup.
    let esp_before_dispatch = *guest_esp;

    // Dispatch to subsystem handlers (chain: file → memory → hal)
    let result = file::dispatch(ordinal, &args, state, memory)
        .or_else(|| memory::dispatch(ordinal, &args, state, memory))
        .or_else(|| {
            hal::dispatch(
                ordinal, &args, state, memory, guest_eax, guest_edx, guest_ecx,
            )
        });

    // ESP-DRIFT INSTRUMENTATION: any delta here is a handler-side mutation
    // (our own stdcall cleanup happens AFTER this, below). Expected delta = 0.
    let handler_delta = (*guest_esp).wrapping_sub(esp_before_dispatch);
    if handler_delta != 0 {
        static HANDLER_DRIFT_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = HANDLER_DRIFT_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 100 {
            crate::xbox::emulator::debug_log(&format!(
                "[ESP-DRIFT-HANDLER #{}] kcall#{} ord={} ({}) argc={} handler_delta=0x{:08X} esp:0x{:08X}->0x{:08X}",
                n, state.total_kernel_calls, ordinal, ordinals::name(ordinal), arg_count,
                handler_delta, esp_before_dispatch, *guest_esp
            ));
        }
    }

    // Capture caller return address from top of guest stack for diagnostics
    let caller_ret = memory.read_u32(*guest_esp);

    let (kr, eax) = match result {
        Some((kr, eax)) => (kr, eax),
        None => {
            // Unhandled — always log with full context (ordinal NUMBER, ret addr, args)
            if hits <= 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[KERNEL-UNHANDLED] ordinal={} ({}) argc={} ret=0x{:08X} args=[0x{:X},0x{:X},0x{:X},0x{:X}]",
                    ordinal, ordinals::name(ordinal), arg_count,
                    caller_ret, args[0], args[1], args[2], args[3]
                ));
            }
            (KernelResult::NotHandled, 0)
        }
    };

    *guest_eax = eax;

    // Stdcall cleanup: callee pops args (not return address — caller handles that)
    if arg_count > 0 {
        *guest_esp += arg_count * 4;
    }

    // ----------------------------------------------------------------------
    // PHASE 4 · Matched-transition pop+verify.
    // Our expected_r14_on_leave = entry_esp + 4 + argc*4, which assumes
    // BOTH the args AND ret_addr have been popped. Inside this function we
    // only pop args (caller does +4 separately for ret_addr). So we pass
    // `*guest_esp + 4` as the observed R14 to match the frame's expectation.
    //
    // If delta != 0, it means something inside the handler chain mutated
    // guest_esp in an unexpected way (e.g., bad memory::dispatch code
    // that did its own cleanup, or a handler that over-popped args).
    //
    // Log-only — does NOT alter any execution state.
    // ----------------------------------------------------------------------
    {
        use crate::xbox::aot::transition;
        let _ = transition::pop_and_verify(
            Some(transition::TransitionKind::Kernel),
            guest_esp.wrapping_add(4),
        );
    }

    // ESP-DRIFT INSTRUMENTATION: full-call delta check.
    // Expected delta = arg_count * 4 (stdcall callee pops args only; caller pops ret addr).
    let expected_delta = (arg_count as u32) * 4;
    let esp_after = *guest_esp;
    let actual_delta = esp_after.wrapping_sub(esp_before);
    if actual_delta != expected_delta {
        static DRIFT_N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DRIFT_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 100 {
            crate::xbox::emulator::debug_log(&format!(
                "[ESP-DRIFT #{}] kcall#{} ord={} ({}) argc={} expected=+{} actual=+0x{:08X} esp:0x{:08X}->0x{:08X}",
                n, state.total_kernel_calls, ordinal, ordinals::name(ordinal), arg_count,
                expected_delta, actual_delta, esp_before, esp_after
            ));
        }
    }

    kr
}

/// Inline kernel dispatch for VEH — takes pre-read args, does NOT touch ESP.
/// Returns (KernelResult, eax_value). Caller handles stdcall cleanup in R14.
pub fn dispatch_inline(
    ordinal: u32,
    args: &[u32; 12],
    guest_eax: &mut u32,
    guest_edx: &mut u32,
    guest_ecx: &mut u32,
    state: &mut KernelState,
    mem_base: *mut u8,
) -> KernelResult {
    // Reject bogus ordinals (same guard as dispatch_kernel_call)
    if ordinal >= 0x8000 {
        return KernelResult::NotHandled;
    }

    let _cpu_phase = crate::xbox::profiler::guard(crate::xbox::profiler::CpuPhase::KernelDispatch);
    let _kernel_subphase = crate::xbox::profiler::kernel_ordinal_guard(ordinal);

    // Track hit
    if (ordinal as usize) < state.hit_counts.len() {
        state.hit_counts[ordinal as usize] += 1;
    }
    state.total_kernel_calls += 1;

    // CRT boot diagnostic — probe at 500 and 900 kernel calls
    if state.total_kernel_calls == 500
        || state.total_kernel_calls == 900
        || state.total_kernel_calls == 1500
    {
        let crtheap = unsafe { std::ptr::read_unaligned(mem_base.add(0x007E_1584) as *const u32) };
        let active_heap =
            unsafe { std::ptr::read_unaligned(mem_base.add(0x007E_1C4C) as *const u32) };
        let process_heap =
            unsafe { std::ptr::read_unaligned(mem_base.add(0x007E_18B0) as *const u32) };
        crate::xbox::emulator::debug_log(&format!(
            "[CRT-PROBE] @{} calls: _crtheap=0x{:08X} __active_heap=0x{:08X} process_heap=0x{:08X}",
            state.total_kernel_calls, crtheap, active_heap, process_heap
        ));
        if crtheap != 0 && crtheap != 0x0200_0000 {
            crate::xbox::emulator::debug_log(&format!(
                "[CRT-PROBE] SUCCESS: _crtheap=0x{:08X} — CRT heap initialized organically!",
                crtheap
            ));
        }
    }

    // Data exports — return 0
    if ordinals::is_data(ordinal) {
        *guest_eax = 0;
        return KernelResult::Handled;
    }

    let arg_count = ordinals::arg_count(ordinal);

    // Log first few calls
    let hits = state.hit_counts.get(ordinal as usize).copied().unwrap_or(0);
    if hits <= 3 {
        log::debug!(
            "kernel-inline: #{} {} argc={} args=[{:#X}, {:#X}, {:#X}, {:#X}]",
            ordinal,
            ordinals::name(ordinal),
            arg_count,
            args[0],
            args[1],
            args[2],
            args[3]
        );
    }

    // Build a GuestMemory wrapper for subsystem handlers that need it
    let memory = unsafe { crate::xbox::memory::guest_memory::GuestMemory::from_raw(mem_base) };

    // Dispatch to subsystem handlers (chain: file → memory → hal)
    let result = file::dispatch(ordinal, args, state, &memory)
        .or_else(|| memory::dispatch(ordinal, args, state, &memory))
        .or_else(|| {
            hal::dispatch(
                ordinal, args, state, &memory, guest_eax, guest_edx, guest_ecx,
            )
        });

    let (kr, eax) = match result {
        Some((kr, eax)) => (kr, eax),
        None => {
            if hits <= 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[KERNEL-INLINE-UNHANDLED] ordinal={} ({}) argc={} args=[0x{:X},0x{:X},0x{:X},0x{:X}]",
                    ordinal, ordinals::name(ordinal), arg_count,
                    args[0], args[1], args[2], args[3]
                ));
            }
            (KernelResult::NotHandled, 0)
        }
    };

    // Rolling call log for panic diagnostics
    {
        let entry = format!(
            "#{} {} eax=0x{:08X} args=[{:#X},{:#X},{:#X},{:#X}]",
            ordinal,
            ordinals::name(ordinal),
            eax,
            args[0],
            args[1],
            args[2],
            args[3]
        );
        if state.call_log.len() >= state.call_log_max {
            state.call_log.remove(0);
        }
        state.call_log.push(entry);
    }

    // PANIC DUMP: if HalReturnToFirmware was just called, dump the last N calls
    if ordinal == ordinals::HalReturnToFirmware {
        use crate::xbox::emulator::debug_log;
        debug_log(&format!(
            "=== PANIC: HalReturnToFirmware({}) — last {} kernel calls ===",
            args[0],
            state.call_log.len()
        ));
        for (i, entry) in state.call_log.iter().enumerate() {
            debug_log(&format!("  [{}] {}", i, entry));
        }
        debug_log("=== END PANIC DUMP ===");
    }

    *guest_eax = eax;

    // TARGETED DIAGNOSTICS: Spider-Man CRT state + final kernel calls.
    //
    // These probes use Spider-Man-specific global addresses and one of them
    // write-protects .text at a hardcoded kernel-call count. Leaving them active
    // globally breaks other canaries (Shenmue II reaches kcall #452 while
    // enumerating SCENE\01\STREAM), so keep them title-scoped unless explicitly
    // requested for a diagnostic repro.
    let spiderman_kernel_diag = state.xbe_title_id == 0x4156_0006
        || std::env::var_os("RUSTEMU_SPIDEY_KERNEL_DIAG").is_some();
    if spiderman_kernel_diag {
        let n = state.total_kernel_calls;

        // .text watchpoint: check if guest .text is being zeroed
        if n % 5000 == 0 || n == 452 {
            // reduced from 50 to 5000 to cut log noise
            let text_probe = unsafe { *(mem_base.add(0x11000) as *const u32) };
            let text_probe2 = unsafe { *(mem_base.add(0x20000) as *const u32) };
            let text_probe3 = unsafe { *(mem_base.add(0x100000) as *const u32) };
            if text_probe == 0 && text_probe2 == 0 && text_probe3 == 0 {
                crate::xbox::emulator::debug_log(&format!(
                    "[TEXT-ZERO] .text is ZEROED at kcall #{} ord={} ({})",
                    n,
                    ordinal,
                    ordinals::name(ordinal)
                ));
            } else if n >= 400 {
                let child2_ret = unsafe { *(mem_base.add(0x2AC551) as *const u32) };
                let child2_ret2 = unsafe { *(mem_base.add(0x2AC540) as *const u32) };
                // Only log the CHILD-2 area occasionally to reduce noise
                if n % 500_000 == 0 || n == 5000 {
                    let scene_ptr = unsafe { *(mem_base.add(0x3F5BEC) as *const u32) };
                    let scene_phys = if scene_ptr >= 0x8000_0000 && scene_ptr < 0xA000_0000 {
                        (scene_ptr & 0x1FFF_FFFF) as usize
                    } else {
                        scene_ptr as usize
                    };
                    let scene_118 = if scene_phys < 0x1000_0000 {
                        unsafe { *(mem_base.add(scene_phys + 0x118) as *const u32) }
                    } else {
                        0xDEAD
                    };
                    // Dump scene struct fields around +0x118
                    if n == 5000 && scene_phys < 0x1000_0000 {
                        crate::xbox::emulator::debug_log(&format!(
                            "[SCENE-DUMP] scene=0x{:08X} phys=0x{:08X}",
                            scene_ptr, scene_phys
                        ));
                        for off in (0x100u32..=0x130).step_by(4) {
                            let val =
                                unsafe { *(mem_base.add(scene_phys + off as usize) as *const u32) };
                            crate::xbox::emulator::debug_log(&format!(
                                "  [scene+0x{:03X}] = 0x{:08X}{}",
                                off,
                                val,
                                if off == 0x118 {
                                    " <<< LOADING STATE"
                                } else {
                                    ""
                                }
                            ));
                        }
                    }
                    crate::xbox::emulator::debug_log(&format!(
                        "[TEXT-LIVE] kcall #{}: scene+118=0x{:08X}",
                        n, scene_118
                    ));
                }
            }
        }

        // ARM .text write-protect trap after kcall #452 (last kcall before .text gets zeroed).
        // Makes guest .text (0x11000-0x2EBDB0) PAGE_READONLY so any write triggers an AV
        // caught by the CODE-WP handler in veh.rs, revealing the exact faulting instruction.
        if n == 452 {
            let text_size = 0x2EB000 - 0x11000; // ~2.9MB, page-aligned
                                                // Protect primary view (0x00011000) and mirror view (0x80011000)
            for &offset in &[0x11000usize, 0x8001_1000usize, 0xA001_1000usize] {
                let text_start = unsafe { mem_base.add(offset) };
                let mut old_prot = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
                let ok = unsafe {
                    windows::Win32::System::Memory::VirtualProtect(
                        text_start as *const core::ffi::c_void,
                        text_size,
                        windows::Win32::System::Memory::PAGE_READONLY,
                        &mut old_prot,
                    )
                };
                crate::xbox::emulator::debug_log(&format!(
                    "[TEXT-TRAP] Armed .text write-protect at kcall #{}: guest=0x{:08X} host=0x{:X} size=0x{:X} ok={:?} old_prot=0x{:X}",
                    n, offset as u32, text_start as u64, text_size, ok, old_prot.0
                ));
            }
        }
        // String pool diagnostics — check if pool was initialized
        if n == 452 {
            let pool_count = unsafe { *(mem_base.add(0x4B1DC0) as *const u32) };
            let pool_flag = unsafe { *(mem_base.add(0x4B1DC4) as *const u8) };
            let pool_entry0 = unsafe { *(mem_base.add(0x3F87B0) as *const u32) };
            let pool_buf0 = unsafe { *(mem_base.add(0x436FC0) as *const u32) };
            // Three pools: small (0x3F8C30), medium (0x42EFB8), large (0x4B1DC0)
            let small_count = unsafe { *(mem_base.add(0x3F8C30) as *const u32) };
            let medium_count = unsafe { *(mem_base.add(0x42EFB8) as *const u32) };
            crate::xbox::emulator::debug_log(&format!(
                "[POOL-DIAG] kcall #{}: small=[0x3F8C30]={} medium=[0x42EFB8]={} large=[0x4B1DC0]={} flag=[0x4B1DC4]=0x{:02X}",
                n, small_count, medium_count, pool_count, pool_flag
            ));
            // Trace the ACTUAL allocator path: [ecx*4 + 0x42EFBC] for small pool
            // The allocator pops from ecx=count down to ecx=0
            if small_count > 0 && small_count < 0x10000 {
                let top_idx = small_count;
                let entry_addr = 0x42EFBC + top_idx as usize * 4;
                let entry_ptr = unsafe { *(mem_base.add(entry_addr) as *const u32) };
                crate::xbox::emulator::debug_log(&format!(
                    "[SMALL-POOL] top: [{}*4+0x42EFBC]=[0x{:08X}]=0x{:08X}",
                    top_idx, entry_addr, entry_ptr
                ));
                if entry_ptr > 0 && entry_ptr < 0x1000_0000 {
                    let flag = unsafe { *(mem_base.add(entry_ptr as usize + 4) as *const i32) };
                    let buf = unsafe { *(mem_base.add(entry_ptr as usize) as *const u32) };
                    let max_sz =
                        unsafe { *(mem_base.add(entry_ptr as usize + 0x10) as *const u32) };
                    crate::xbox::emulator::debug_log(&format!(
                        "[SMALL-POOL] entry@0x{:08X}: buf=0x{:08X} flag={} maxsz={}",
                        entry_ptr, buf, flag, max_sz
                    ));
                }
                // Check a few entries below the top
                for off in 1..4u32 {
                    if top_idx > off {
                        let idx = top_idx - off;
                        let ea = 0x42EFBC + idx as usize * 4;
                        let ep = unsafe { *(mem_base.add(ea) as *const u32) };
                        crate::xbox::emulator::debug_log(&format!(
                            "[SMALL-POOL] [{}*4+0x42EFBC]=[0x{:08X}]=0x{:08X}",
                            idx, ea, ep
                        ));
                    }
                }
            }
            // Zero page state (what rep stosd reads if ESI=0)
            let zp_0 = unsafe { *(mem_base.add(0) as *const u32) };
            let zp_10 = unsafe { *(mem_base.add(0x10) as *const u32) };
            crate::xbox::emulator::debug_log(&format!(
                "[ZERO-PAGE] [0x00]=0x{:08X} [0x10]=0x{:08X}",
                zp_0, zp_10
            ));
            // Dump first 5 pool entry "in-use" flags at entry_ptr+4
            // Entries are at [i*4 + 0x3F87B0] for i=0..pool_count-1
            for i in 0..5u32.min(pool_count) {
                let entry_ptr = unsafe { *(mem_base.add(0x3F87B0 + i as usize * 4) as *const u32) };
                if entry_ptr > 0 && entry_ptr < 0x1000_0000 {
                    let flag_val = unsafe { *(mem_base.add(entry_ptr as usize + 4) as *const i32) };
                    let buf_ptr = unsafe { *(mem_base.add(entry_ptr as usize) as *const u32) };
                    crate::xbox::emulator::debug_log(&format!(
                        "[POOL-ENTRY] entry[{}] @0x{:08X}: buf=0x{:08X} inuse_flag={} (0x{:08X})",
                        i, entry_ptr, buf_ptr, flag_val, flag_val as u32
                    ));
                }
            }
            // Also check the last few entries (the allocator pops from the top)
            for i in (pool_count.saturating_sub(3))..pool_count {
                let entry_ptr = unsafe { *(mem_base.add(0x3F87B0 + i as usize * 4) as *const u32) };
                if entry_ptr > 0 && entry_ptr < 0x1000_0000 {
                    let flag_val = unsafe { *(mem_base.add(entry_ptr as usize + 4) as *const i32) };
                    let buf_ptr = unsafe { *(mem_base.add(entry_ptr as usize) as *const u32) };
                    crate::xbox::emulator::debug_log(&format!(
                        "[POOL-ENTRY] entry[{}] @0x{:08X}: buf=0x{:08X} inuse_flag={} (0x{:08X})",
                        i, entry_ptr, buf_ptr, flag_val, flag_val as u32
                    ));
                }
            }
        }
        // After GAME.INI close: check if INI was parsed correctly
        if n == 442 && ordinal == ordinals::NtClose {
            for (label, addr) in [
                ("ROOT_DIR @0x3F57F0", 0x3F57F0u32),
                ("BUF2 @0x3F58F0", 0x3F58F0u32),
            ] {
                let mut s = [0u8; 64];
                for i in 0..64 {
                    s[i] = unsafe { *mem_base.add(addr as usize + i) };
                }
                let ascii: String = s
                    .iter()
                    .take_while(|&&b| b != 0)
                    .map(|&b| {
                        if b >= 0x20 && b < 0x7F {
                            b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();
                crate::xbox::emulator::debug_log(&format!(
                    "[INI-DIAG] after GAME.INI close #{}: {}: '{}'",
                    n, label, ascii
                ));
            }
        }
        // After kcall 440+: log ALL kernel calls with full 8 args
        if n >= 440 && n <= 460 {
            crate::xbox::emulator::debug_log(&format!(
                "[FINAL-KCALL] #{} ord={} ({}) args=[{:08X},{:08X},{:08X},{:08X},{:08X},{:08X},{:08X},{:08X}] eax=0x{:08X}",
                n, ordinal, ordinals::name(ordinal),
                args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], eax
            ));
        }

        // One-shot CRT diagnostic at kcall 450 (right before sm_paths.txt close)
        if n == 450 {
            // __pctype — CRT locale character type table pointer
            let pctype = unsafe { *(mem_base.add(0x3F4EC4) as *const u32) };
            // __ptmbcinfo — multibyte codepage info
            let mbcinfo = unsafe { *(mem_base.add(0x3F4EC0) as *const u32) };
            // _crtheap
            let crtheap = unsafe { *(mem_base.add(0x007E1584) as *const u32) };
            // __lc_handle (locale handle array)
            let lc0 = unsafe { *(mem_base.add(0x3F4EA0) as *const u32) };
            crate::xbox::emulator::debug_log(&format!(
                "[CRT-DIAG] kcall #{}: __pctype=[0x3F4EC4]=0x{:08X} __mbcinfo=[0x3F4EC0]=0x{:08X} _crtheap=0x{:08X} __lc[0]=0x{:08X}",
                n, pctype, mbcinfo, crtheap, lc0
            ));
            // If pctype is valid, dump first 16 bytes of the table
            if pctype > 0x1000 && pctype < 0x1000_0000 {
                let mut tbl = [0u8; 16];
                for i in 0..16 {
                    tbl[i] = unsafe { *mem_base.add(pctype as usize + i) };
                }
                crate::xbox::emulator::debug_log(&format!(
                    "[CRT-DIAG] __pctype table @0x{:08X}: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                    pctype, tbl[0], tbl[1], tbl[2], tbl[3], tbl[4], tbl[5], tbl[6], tbl[7],
                    tbl[8], tbl[9], tbl[10], tbl[11], tbl[12], tbl[13], tbl[14], tbl[15]
                ));
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "[CRT-DIAG] WARNING: __pctype=0x{:08X} — INVALID/NULL! sprintf will read garbage from zero page!",
                    pctype
                ));
            }
        }

        // After NtReadFile for sm_paths.txt (kcall #453): dump the read buffer
        if n == 453 && ordinal == ordinals::NtReadFile {
            let buf_addr = args[5]; // NtReadFile arg[5] = Buffer
            let buf_len = args[6].min(128); // arg[6] = Length
            let mut buf_sample = [0u8; 128];
            for i in 0..buf_len as usize {
                buf_sample[i] = unsafe { *mem_base.add(buf_addr as usize + i) };
            }
            let ascii: String = buf_sample[..buf_len as usize]
                .iter()
                .map(|&b| {
                    if b >= 0x20 && b < 0x7F {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();
            crate::xbox::emulator::debug_log(&format!(
                "[FILE-DIAG] kcall #{}: NtReadFile buf@0x{:08X} len=0x{:X} first 128: {}",
                n, buf_addr, args[6], ascii
            ));
            // Check IO_STATUS_BLOCK
            let iosb = args[4]; // arg[4] = IoStatusBlock
            let iosb_status = memory.read_u32(iosb);
            let iosb_info = memory.read_u32(iosb + 4);
            crate::xbox::emulator::debug_log(&format!(
                "[FILE-DIAG] IO_STATUS_BLOCK @0x{:08X}: Status=0x{:08X} Information=0x{:08X}",
                iosb, iosb_status, iosb_info
            ));
        }

        // After NtClose (kcall #454): dump global path buffers + stack
        if ordinal == ordinals::NtClose && n >= 450 && n <= 460 {
            // Check ROOT_DIR at 0x3F57F0 and PRE_ROOT_DIR at 0x3F58F0
            for (label, addr) in [
                ("ROOT_DIR @0x3F57F0", 0x3F57F0u32),
                ("BUF2 @0x3F58F0", 0x3F58F0u32),
            ] {
                let mut s = [0u8; 64];
                for i in 0..64 {
                    s[i] = unsafe { *mem_base.add(addr as usize + i) };
                }
                let ascii: String = s
                    .iter()
                    .take_while(|&&b| b != 0)
                    .map(|&b| {
                        if b >= 0x20 && b < 0x7F {
                            b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();
                crate::xbox::emulator::debug_log(&format!("[PATH-DIAG] {}: '{}'", label, ascii));
            }
            // Also check some key game globals
            // SCENE_NAME might be at a known address, search near 0x3F5xxx
            for addr in [0x3F59F0u32, 0x3F5AF0u32, 0x3F5BF0u32, 0x3F5CF0u32] {
                let mut s = [0u8; 32];
                for i in 0..32 {
                    s[i] = unsafe { *mem_base.add(addr as usize + i) };
                }
                let ascii: String = s
                    .iter()
                    .take_while(|&&b| b != 0)
                    .map(|&b| {
                        if b >= 0x20 && b < 0x7F {
                            b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();
                if !ascii.is_empty() {
                    crate::xbox::emulator::debug_log(&format!(
                        "[PATH-DIAG] @0x{:08X}: '{}'",
                        addr, ascii
                    ));
                }
            }
        }
    }

    kr
}
