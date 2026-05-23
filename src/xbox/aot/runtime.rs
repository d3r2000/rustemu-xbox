/// AOT runtime types: GuestState, RuntimeContext, AddrHash, TrapInfo.
/// All asm!-visible structs are #[repr(C)] with documented offsets.
use std::alloc;
use std::ptr;

// ============================================================================
// GuardedStack — VirtualAlloc'd region with guard pages on both sides.
// Prevents heap corruption from shadow stack overflow/underflow.
// Layout: [GUARD_PAGE | usable region | GUARD_PAGE]
// ============================================================================

#[cfg(windows)]
pub struct GuardedStack {
    /// Base of the entire VirtualAlloc reservation (including guard pages)
    alloc_base: *mut u8,
    /// Total allocation size (guard + usable + guard)
    alloc_size: usize,
    /// Pointer to start of usable region (after bottom guard)
    pub usable_base: *mut u8,
    /// Size of usable region in bytes
    pub usable_size: usize,
}

#[cfg(windows)]
impl GuardedStack {
    pub fn new(usable_entries: usize) -> Self {
        use windows::Win32::System::Memory::*;

        let page_size = 4096usize;
        let usable_size = usable_entries * 8; // u64 entries
                                              // Round up to page boundary
        let usable_pages = (usable_size + page_size - 1) / page_size;
        let usable_size_aligned = usable_pages * page_size;
        let total_size = page_size + usable_size_aligned + page_size; // guard + usable + guard

        unsafe {
            // Reserve entire region
            let base = VirtualAlloc(None, total_size, MEM_RESERVE, PAGE_NOACCESS);
            assert!(!base.is_null(), "GuardedStack: VirtualAlloc reserve failed");

            // Commit usable region (middle) as PAGE_READWRITE
            let usable_base = (base as *mut u8).add(page_size);
            let committed = VirtualAlloc(
                Some(usable_base as *const _),
                usable_size_aligned,
                MEM_COMMIT,
                PAGE_READWRITE,
            );
            assert!(
                !committed.is_null(),
                "GuardedStack: VirtualAlloc commit failed"
            );

            // Guard pages remain reserved-only (PAGE_NOACCESS) — any access = AV

            Self {
                alloc_base: base as *mut u8,
                alloc_size: total_size,
                usable_base,
                usable_size: usable_size_aligned,
            }
        }
    }

    /// Get pointer to top of usable region (for full-descending stack)
    pub fn top(&self) -> *mut u8 {
        unsafe { self.usable_base.add(self.usable_size) }
    }

    /// Check if a fault address is within our guard pages
    pub fn is_guard_fault(&self, addr: u64) -> bool {
        let addr = addr as usize;
        let base = self.alloc_base as usize;
        let page_size = 4096usize;
        // Bottom guard: [alloc_base, alloc_base + page_size)
        if addr >= base && addr < base + page_size {
            return true;
        }
        // Top guard: [usable_base + usable_size, alloc_base + alloc_size)
        let top_guard_start = self.usable_base as usize + self.usable_size;
        if addr >= top_guard_start && addr < base + self.alloc_size {
            return true;
        }
        false
    }
}

#[cfg(windows)]
impl Drop for GuardedStack {
    fn drop(&mut self) {
        if !self.alloc_base.is_null() {
            unsafe {
                let _ = windows::Win32::System::Memory::VirtualFree(
                    self.alloc_base as *mut _,
                    0,
                    windows::Win32::System::Memory::MEM_RELEASE,
                );
            }
        }
    }
}

#[cfg(windows)]
unsafe impl Send for GuardedStack {}

// ============================================================================
// Exit reasons
// ============================================================================

pub const AOT_EXIT_RUNNING: u32 = 0;
pub const AOT_EXIT_RET_TO_ZERO: u32 = 1;
pub const AOT_EXIT_SYSTEM_TRAP: u32 = 2;
pub const AOT_EXIT_UNRESOLVED: u32 = 3;
pub const AOT_EXIT_ERROR: u32 = 4;
pub const AOT_EXIT_SPAWN_THREAD: u32 = 5;
pub const AOT_EXIT_THREAD_EXIT: u32 = 6;
pub const AOT_EXIT_HALT: u32 = 7;
pub const AOT_EXIT_KERNEL_CALL: u32 = 8;

// ============================================================================
// GuestState — exact layout for asm! trampoline (R13 points here)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GuestState {
    pub host_rsp: u64,         // +0x00: saved host RSP for exit path
    pub exit_addr: u64,        // +0x08: address of trampoline exit label
    pub eax: u32,              // +0x10
    pub ecx: u32,              // +0x14
    pub edx: u32,              // +0x18
    pub ebx: u32,              // +0x1C
    pub esp: u32,              // +0x20: loaded into R14
    pub ebp: u32,              // +0x24
    pub esi: u32,              // +0x28
    pub edi: u32,              // +0x2C
    pub exit_reason: u32,      // +0x30: AOT_EXIT_*
    pub exit_guest_addr: u32,  // +0x34: guest address at exit
    pub r15_base: u64,         // +0x38: guest memory base pointer
    pub shadow_stack_top: u64, // +0x40: R12 value (shadow stack pointer)
}
// Total: 0x48 = 72 bytes

// Compile-time offset verification
const _: () = {
    assert!(std::mem::size_of::<GuestState>() == 0x48);
    assert!(std::mem::offset_of!(GuestState, host_rsp) == 0x00);
    assert!(std::mem::offset_of!(GuestState, exit_addr) == 0x08);
    assert!(std::mem::offset_of!(GuestState, eax) == 0x10);
    assert!(std::mem::offset_of!(GuestState, ecx) == 0x14);
    assert!(std::mem::offset_of!(GuestState, edx) == 0x18);
    assert!(std::mem::offset_of!(GuestState, ebx) == 0x1C);
    assert!(std::mem::offset_of!(GuestState, esp) == 0x20);
    assert!(std::mem::offset_of!(GuestState, ebp) == 0x24);
    assert!(std::mem::offset_of!(GuestState, esi) == 0x28);
    assert!(std::mem::offset_of!(GuestState, edi) == 0x2C);
    assert!(std::mem::offset_of!(GuestState, exit_reason) == 0x30);
    assert!(std::mem::offset_of!(GuestState, exit_guest_addr) == 0x34);
    assert!(std::mem::offset_of!(GuestState, r15_base) == 0x38);
    assert!(std::mem::offset_of!(GuestState, shadow_stack_top) == 0x40);
};

// RuntimeContext layout assertions — HARD REQUIREMENT for inline RET x64 code.
// The emitted code uses [R13+offset] to access these fields directly.
// If any assertion fails, the inline hash probe reads wrong data → crash.
const _: () = {
    assert!(std::mem::offset_of!(RuntimeContext, code_base) == 0x48);
    assert!(std::mem::offset_of!(RuntimeContext, code_size) == 0x50);
    assert!(std::mem::offset_of!(RuntimeContext, hash_shift) == 0x54);
    assert!(std::mem::offset_of!(RuntimeContext, addr_hash) == 0x58);
};

impl Default for GuestState {
    fn default() -> Self {
        Self {
            host_rsp: 0,
            exit_addr: 0,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            exit_reason: AOT_EXIT_RUNNING,
            exit_guest_addr: 0,
            r15_base: 0,
            shadow_stack_top: 0,
        }
    }
}

// ============================================================================
// Trap types and info
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrapType {
    Ret = 0,
    Indirect = 1,
    System = 2,
    Error = 3,
    Rescue = 4,     // Encode failure → VEH decodes original guest bytes at runtime
    StackTrace = 5, // P0 diagnostic: log R14 and continue (no state change)
    RetMiss = 6,    // Inline RET hash miss — R14+R12 already adjusted, R10D has guest_ret
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapInfo {
    pub host_offset: u32,
    pub trap_type: TrapType,
    pub ret_cleanup: u16, // for RET imm16 (stdcall cleanup bytes)
    pub guest_addr: u32,
}

// ============================================================================
// Address hash table — Fibonacci hashing, open addressing, tier system
// Assembly-friendly: #[repr(C)] so emitter can inline lookups
// ============================================================================

pub const FIB_HASH_CONST: u32 = 0x9E3779B1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AddrTier {
    Main = 0,    // first-pass code, never overwritten
    Harvest = 1, // pointer harvest phase
    Rescue = 2,  // runtime on-demand, lowest priority
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HashEntry {
    pub guest_addr: u32, // 0 = empty slot
    pub host_offset: u32,
    pub tier: u8,
    pub _pad: [u8; 3],
}

#[repr(C)]
pub struct AddrHash {
    pub entries: *mut HashEntry,
    pub capacity: u32,
    pub mask: u32, // capacity - 1
    pub count: u32,
    pub owned: bool, // true = we allocated entries, drop will dealloc
}

impl AddrHash {
    /// Create a new hash table with 2^bits capacity.
    pub fn new(bits: u32) -> Self {
        let capacity = 1u32 << bits;
        let layout = alloc::Layout::array::<HashEntry>(capacity as usize).unwrap();
        let entries = unsafe { alloc::alloc_zeroed(layout) as *mut HashEntry };
        assert!(!entries.is_null(), "AddrHash allocation failed");
        Self {
            entries,
            capacity,
            mask: capacity - 1,
            count: 0,
            owned: true,
        }
    }

    /// Insert a guest→host mapping. Returns true if inserted, false if table full
    /// or existing entry has higher priority (lower tier).
    pub fn insert(&mut self, guest_addr: u32, host_offset: u32, tier: AddrTier) -> bool {
        if guest_addr == 0 {
            return false;
        } // 0 is sentinel for empty
        let start = self.hash(guest_addr);
        let mut idx = start;
        loop {
            let entry = unsafe { &mut *self.entries.add(idx as usize) };
            if entry.guest_addr == 0 {
                // Empty slot — insert
                entry.guest_addr = guest_addr;
                entry.host_offset = host_offset;
                entry.tier = tier as u8;
                self.count += 1;
                return true;
            }
            if entry.guest_addr == guest_addr {
                // Existing entry — only overwrite if new tier < existing tier
                if (tier as u8) < entry.tier {
                    entry.host_offset = host_offset;
                    entry.tier = tier as u8;
                    return true;
                }
                return false; // existing has priority
            }
            idx = (idx + 1) & self.mask;
            if idx == start {
                return false;
            } // table full
        }
    }

    /// Lookup host offset for a guest address. Returns None if not found.
    #[inline]
    pub fn lookup(&self, guest_addr: u32) -> Option<u32> {
        if guest_addr == 0 {
            return None;
        } // 0 is sentinel
        let start = self.hash(guest_addr);
        let mut idx = start;
        loop {
            let entry = unsafe { &*self.entries.add(idx as usize) };
            if entry.guest_addr == guest_addr {
                return Some(entry.host_offset);
            }
            if entry.guest_addr == 0 {
                return None;
            }
            idx = (idx + 1) & self.mask;
            if idx == start {
                return None;
            }
        }
    }

    /// Lookup host offset for a guest address, trying nearby addresses if exact miss.
    /// Returns (actual_guest_addr, host_offset) or None. Checks ±1..±max_delta.
    pub fn lookup_nearby(&self, guest_addr: u32, max_delta: u32) -> Option<(u32, u32)> {
        if let Some(off) = self.lookup(guest_addr) {
            return Some((guest_addr, off));
        }
        for delta in 1..=max_delta {
            if let Some(off) = self.lookup(guest_addr.wrapping_sub(delta)) {
                return Some((guest_addr.wrapping_sub(delta), off));
            }
            if let Some(off) = self.lookup(guest_addr.wrapping_add(delta)) {
                return Some((guest_addr.wrapping_add(delta), off));
            }
        }
        None
    }

    /// Remove a guest address from the hash table (for rescue recompilation).
    pub fn remove(&mut self, guest_addr: u32) {
        if guest_addr == 0 {
            return;
        }
        let start = self.hash(guest_addr);
        let mut idx = start;
        loop {
            let entry = unsafe { &mut *self.entries.add(idx as usize) };
            if entry.guest_addr == guest_addr {
                entry.guest_addr = 0;
                entry.host_offset = 0;
                entry.tier = 0;
                if self.count > 0 {
                    self.count -= 1;
                }
                return;
            }
            if entry.guest_addr == 0 {
                return;
            }
            idx = (idx + 1) & self.mask;
            if idx == start {
                return;
            }
        }
    }

    /// Reverse lookup: find guest addr closest to (but ≤) a host offset.
    /// Linear scan — only use for diagnostics, not hot paths.
    pub fn reverse_lookup(&self, host_offset: u32) -> u32 {
        let mut best_guest = 0u32;
        let mut best_host = 0u32;
        let cap = self.capacity as usize;
        for i in 0..cap {
            let entry = unsafe { &*self.entries.add(i) };
            if entry.guest_addr != 0
                && entry.host_offset <= host_offset
                && entry.host_offset > best_host
            {
                best_host = entry.host_offset;
                best_guest = entry.guest_addr;
            }
        }
        best_guest
    }

    #[inline]
    fn hash(&self, guest_addr: u32) -> u32 {
        guest_addr
            .wrapping_mul(FIB_HASH_CONST)
            .wrapping_shr(32u32.saturating_sub(self.capacity.trailing_zeros()))
            & self.mask
    }
}

impl Drop for AddrHash {
    fn drop(&mut self) {
        if self.owned && !self.entries.is_null() {
            let layout = alloc::Layout::array::<HashEntry>(self.capacity as usize).unwrap();
            unsafe {
                alloc::dealloc(self.entries as *mut u8, layout);
            }
            self.entries = ptr::null_mut();
        }
    }
}

// ============================================================================
// Address map entry (for branch fixups)
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct AddrEntry {
    pub guest_addr: u32,
    pub host_offset: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct BranchFixup {
    pub host_offset: u32,    // where rel32 placeholder is in output
    pub guest_target: u32,   // target guest address
    pub instr_end_delta: u8, // bytes from fixup to end of host instruction
}

// ============================================================================
// RuntimeContext — full AOT engine state
// ============================================================================

#[repr(C)]
pub struct RuntimeContext {
    pub guest: GuestState,         // +0x00 (MUST be first for R13 access)
    pub code_base: *mut u8,        // +0x48
    pub code_size: u32,            // +0x50
    pub hash_shift: u32,           // +0x54: 32 - log2(hash_capacity), for inline RET probe
    pub addr_hash: AddrHash,       // +0x58
    pub trap_table: Vec<TrapInfo>, // sorted by host_offset
    pub shadow_stack: GuardedStack,
    pub host_stack: Vec<u8>, // 1MB host stack
    pub guest_mem_base: *mut u8,
    // Rescue pool for on-demand JIT compilation of missing functions.
    pub rescue_pool: *mut u8,
    pub rescue_pool_offset: u32, // Offset from code_base to rescue pool start
    pub rescue_pool_size: u32,
    pub rescue_pool_used: u32,
    // Rescue code ranges (dynamically emitted for on-demand compilation)
    pub rescue_ranges: Vec<(*const u8, usize)>,
    // stats
    pub dispatch_count: u64,
    pub ret_shadow_hits: u64,
    pub ret_hash_lookups: u64,
    pub indirect_dispatches: u64,
    pub system_traps: u64,
    /// Return address for the last indirect CALL (0 if JMP or no pending CALL).
    /// Set by VEH, consumed by dispatch loop. Avoids pushing to guest stack in VEH
    /// (which would corrupt ESP if the target is unresolvable).
    pub indirect_call_ret_addr: u32,
    pub mmio_count: u64,
    pub pb_commands: u64,
    pub draw_calls: u64,
    pub sign_ext_fixups: u64,
    pub rescue_count: u64,
    /// Kernel state pointer — set by dispatch loop owner, used by VEH for inline dispatch.
    pub kernel_state: *mut u8,
    /// For AOT_EXIT_SPAWN_THREAD: entry point and context for new worker thread.
    pub spawn_entry: u32, // SystemRoutine (e.g. XapiThreadStartup)
    pub spawn_start_routine: u32, // StartRoutine (actual game function)
    pub spawn_context: u32,       // StartContext (passed to StartRoutine)
    pub spawn_thread_handle: u32,
    /// For AOT_EXIT_KERNEL_CALL: kernel call details for safe dispatch outside VEH.
    pub kernel_ordinal: u32,
    pub kernel_args: [u32; 12],
    pub kernel_arg_count: u32,
    pub kernel_is_call: bool,
    pub kernel_jmp_ret_addr: u32, // for JMP: guest return addr from stack
    pub kernel_host_resume: u64,  // host RIP to resume after kernel dispatch
    pub kernel_r14_pre_cleanup: u32, // R14 BEFORE stdcall cleanup (for drift tracking)
    /// Resolved kernel thunk table with snapshot for runtime repair.
    pub thunk_table: crate::xbox::loader::xbe::ThunkTable,
    /// Executable section ranges (guest VA start, guest VA end) for interpreter guard.
    /// If EIP falls outside all ranges, interpreter simulates RET instead of executing data.
    pub exec_ranges: Vec<(u32, u32)>,
    /// Data-only section ranges (guest VA start, guest VA end) identified by name
    /// (.data / .rdata / .bss) regardless of their XBE EXEC flag. Spider-Man's .data
    /// is XBE-flagged 0x7 (RWX) but contains string tables and static data, not code.
    /// rescue_emit rejects addresses in these ranges so indirect CALLs / RETs that
    /// resolve to a stale pointer into a data section don't compile ASCII bytes as
    /// x86 instructions (cascades into `outsb gs:[rsi]` / `gs: popad` illegal-instr
    /// faults at runtime).
    pub data_ranges: Vec<(u32, u32)>,
    /// Unresolved branch fixups from AOT compilation — can be re-resolved after rescue-compile.
    pub unresolved_fixups: Vec<(BranchFixup, u32)>, // (fixup, global_host_base_offset)
}

// RuntimeContext contains raw pointers but is only mutated by the thread that owns it.
// Guest memory is a shared VirtualAlloc region — thread safety is managed at the HLE level.
unsafe impl Send for RuntimeContext {}

impl RuntimeContext {
    pub fn new(guest_mem_base: *mut u8, r15_base: u64, addr_hash_bits: u32) -> Self {
        let shadow_cap = 16 * 1024 * 1024usize; // 128MB — must survive Doom's 85K kernel calls with R12 leak
        let host_stack_size = 128 * 1024 * 1024; // 128MB — Classic Doom needs deep CRT init chains

        let mut ctx = Self {
            guest: GuestState::default(),
            code_base: ptr::null_mut(),
            code_size: 0,
            hash_shift: 32u32.saturating_sub(addr_hash_bits),
            addr_hash: AddrHash::new(addr_hash_bits),
            trap_table: Vec::new(),
            shadow_stack: GuardedStack::new(shadow_cap),
            host_stack: vec![0u8; host_stack_size],
            guest_mem_base,
            rescue_pool: ptr::null_mut(),
            rescue_pool_offset: 0,
            rescue_pool_size: 0,
            rescue_pool_used: 0,
            rescue_ranges: Vec::new(),
            dispatch_count: 0,
            ret_shadow_hits: 0,
            ret_hash_lookups: 0,
            indirect_dispatches: 0,
            system_traps: 0,
            indirect_call_ret_addr: 0,
            mmio_count: 0,
            pb_commands: 0,
            draw_calls: 0,
            sign_ext_fixups: 0,
            rescue_count: 0,
            kernel_state: std::ptr::null_mut(),
            spawn_entry: 0,
            spawn_start_routine: 0,
            spawn_context: 0,
            spawn_thread_handle: 0,
            kernel_ordinal: 0,
            kernel_args: [0u32; 12],
            kernel_arg_count: 0,
            kernel_is_call: false,
            kernel_jmp_ret_addr: 0,
            kernel_host_resume: 0,
            kernel_r14_pre_cleanup: 0,
            thunk_table: crate::xbox::loader::xbe::ThunkTable::empty(),
            exec_ranges: Vec::new(),
            data_ranges: Vec::new(),
            unresolved_fixups: Vec::new(),
        };

        ctx.guest.r15_base = r15_base;
        // Shadow stack pointer starts at the top (full-descending)
        ctx.guest.shadow_stack_top = ctx.shadow_stack.top() as u64;

        ctx
    }

    /// Snapshot resolved kernel thunk table from guest memory.
    pub fn snapshot_thunks(&mut self, thunk_addr: u32, base: u32, img_size: u32) {
        self.thunk_table = crate::xbox::loader::xbe::ThunkTable::snapshot(
            self.guest_mem_base,
            thunk_addr,
            base,
            img_size,
        );
        crate::xbox::emulator::debug_log(&format!(
            "Thunk snapshot: {} entries at 0x{:08X}",
            self.thunk_table.len(),
            thunk_addr
        ));
    }

    /// Create a worker context sharing code/hash/traps with the main context.
    /// Own GuestState, shadow stack, and host stack — matching C++ AOT_LaunchWorkerThread.
    pub fn new_worker(main: &RuntimeContext) -> Self {
        let shadow_cap = 16 * 1024 * 1024usize; // 128MB — must survive Doom's 85K kernel calls with R12 leak
        let host_stack_size = 128 * 1024 * 1024; // 128MB — Classic Doom needs deep CRT init chains

        let shadow_stack = GuardedStack::new(shadow_cap);
        let top = shadow_stack.top() as u64;

        // Share code_base, code_size, addr_hash (read-only), trap_table, guest_mem
        // by copying the raw pointers. AddrHash entries pointer is shared (read-only).
        Self {
            guest: GuestState {
                r15_base: main.guest.r15_base,
                shadow_stack_top: top,
                ..GuestState::default()
            },
            code_base: main.code_base,
            code_size: main.code_size,
            hash_shift: main.hash_shift,
            // Share the hash table pointer (read-only after compilation)
            addr_hash: AddrHash {
                entries: main.addr_hash.entries,
                capacity: main.addr_hash.capacity,
                mask: main.addr_hash.mask,
                count: main.addr_hash.count,
                owned: false, // shared pointer — main owns the allocation
            },
            // Clone trap table (Vec of small structs, cheap)
            trap_table: main.trap_table.clone(),
            shadow_stack,
            host_stack: vec![0u8; host_stack_size],
            guest_mem_base: main.guest_mem_base,
            rescue_pool: main.rescue_pool,
            rescue_pool_offset: main.rescue_pool_offset,
            rescue_pool_size: main.rescue_pool_size,
            rescue_pool_used: main.rescue_pool_used,
            rescue_ranges: main.rescue_ranges.clone(),
            dispatch_count: 0,
            ret_shadow_hits: 0,
            ret_hash_lookups: 0,
            indirect_dispatches: 0,
            system_traps: 0,
            indirect_call_ret_addr: 0,
            mmio_count: 0,
            pb_commands: 0,
            draw_calls: 0,
            sign_ext_fixups: 0,
            rescue_count: 0,
            kernel_state: std::ptr::null_mut(),
            spawn_entry: 0,
            spawn_start_routine: 0,
            spawn_context: 0,
            spawn_thread_handle: 0,
            kernel_ordinal: 0,
            kernel_args: [0u32; 12],
            kernel_arg_count: 0,
            kernel_is_call: false,
            kernel_jmp_ret_addr: 0,
            kernel_host_resume: 0,
            kernel_r14_pre_cleanup: 0,
            thunk_table: main.thunk_table.clone(),
            exec_ranges: main.exec_ranges.clone(),
            data_ranges: main.data_ranges.clone(),
            unresolved_fixups: Vec::new(), // worker doesn't need fixups
        }
    }

    /// Reset shadow stack pointer to the top (for thread switching).
    pub fn reset_shadow_stack(&mut self) {
        let top = self.shadow_stack.top() as u64;
        self.guest.shadow_stack_top = top;
        // Zero the top 64KB of shadow stack to prevent stale entries from
        // previous dispatches causing inline RET to jump to old host addresses.
        // 64KB = 8192 entries, enough for any call depth within a single dispatch.
        // Pool init's deepest nesting is ~500 calls = 4KB of shadow stack.
        let zero_bytes = 65536usize.min(self.shadow_stack.usable_size);
        unsafe {
            let zero_start = (top as usize - zero_bytes) as *mut u8;
            std::ptr::write_bytes(zero_start, 0, zero_bytes);
        }
    }

    /// Check if a host RIP is in any of our code regions (main buffer or rescue ranges).
    pub fn is_our_code(&self, rip: u64) -> bool {
        let code_base = self.code_base as u64;
        let code_end = code_base + self.code_size as u64;
        if rip >= code_base && rip < code_end {
            return true;
        }
        self.rescue_ranges.iter().any(|&(base, size)| {
            let b = base as u64;
            rip >= b && rip < b + size as u64
        })
    }

    /// Compute the host offset for any RIP in our code (main AOT or rescue pool).
    /// Returns 0xFFFF_FFFF if RIP is not in our code.
    /// Rescue pool lives at code_base + rescue_pool_offset, so (rip - code_base)
    /// gives the correct offset for both main and rescue code.
    pub fn host_offset_for_rip(&self, rip: u64) -> u32 {
        if self.is_our_code(rip) {
            (rip - self.code_base as u64) as u32
        } else {
            0xFFFF_FFFF
        }
    }

    /// Re-resolve unresolved branch fixups in the main code buffer.
    /// Called after rescue-compile adds new addr_hash entries.
    /// Returns count of newly resolved fixups.
    pub fn resolve_pending_fixups(&mut self) -> u32 {
        if self.unresolved_fixups.is_empty() || self.code_base.is_null() {
            return 0;
        }
        let mut resolved = 0u32;
        let mut still_unresolved = Vec::new();
        for &(ref fixup, base_offset) in &self.unresolved_fixups {
            let global_host_offset = fixup.host_offset + base_offset;
            if let Some(target_host) = self.addr_hash.lookup(fixup.guest_target) {
                let rel32 =
                    target_host as i64 - (global_host_offset as i64 + fixup.instr_end_delta as i64);
                let rel32 = rel32 as i32;
                // Patch the main code buffer (already RWX or needs VirtualProtect)
                #[cfg(windows)]
                unsafe {
                    use windows::Win32::System::Memory::*;
                    let ptr = self.code_base.add(global_host_offset as usize);
                    let mut old_prot = PAGE_PROTECTION_FLAGS(0);
                    if VirtualProtect(ptr as *const _, 4, PAGE_EXECUTE_READWRITE, &mut old_prot)
                        .is_ok()
                    {
                        std::ptr::copy_nonoverlapping(rel32.to_le_bytes().as_ptr(), ptr, 4);
                        let _ = VirtualProtect(ptr as *const _, 4, old_prot, &mut old_prot);
                        resolved += 1;
                        crate::xbox::emulator::debug_log(&format!(
                            "[FIXUP] Resolved call to guest 0x{:08X} at host+0x{:X} → target host+0x{:X}",
                            fixup.guest_target, global_host_offset, target_host
                        ));
                    }
                }
                #[cfg(not(windows))]
                {
                    resolved += 1;
                }
            } else {
                still_unresolved.push((fixup.clone(), base_offset));
            }
        }
        self.unresolved_fixups = still_unresolved;
        resolved
    }

    /// Look up a trap by host offset (binary search, trap_table must be sorted).
    pub fn find_trap(&self, host_offset: u32) -> Option<&TrapInfo> {
        self.trap_table
            .binary_search_by_key(&host_offset, |t| t.host_offset)
            .ok()
            .map(|i| &self.trap_table[i])
    }

    /// Rescue-compile a guest function on-demand (lazy JIT).
    /// Called when UNRESOLVED fires for a valid guest address.
    pub fn rescue_emit(&mut self, guest_addr: u32) -> Option<u32> {
        use crate::xbox::aot::{decoder, emitter};
        use crate::xbox::memory::guest_memory::GuestMemory;

        crate::xbox::aot::jit::note_unresolved_rescue_candidate(
            guest_addr,
            self.guest.esp,
            self.guest.eax,
        );

        // NULL-page guard (2026-04-23): reject any rescue target below 0x10000.
        // The first 64KB of the Xbox address space is unused / NT_TIB / zero page
        // and never contains legitimate code. A corrupted function pointer, stale
        // RET target, or uninitialized indirect CALL can resolve into this range;
        // rescue-compiling zero-filled bytes decodes to long `add [eax],al` runs
        // that fault immediately and spin the worker in AV retry loops.
        if guest_addr < 0x10000 {
            static NULL_REJECT_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = NULL_REJECT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[RESCUE] REJECT NULL-page target 0x{:08X} (< 0x10000) — not compiling",
                    guest_addr
                ));
            }
            return None;
        }

        // Stack-range guard (H03 agent finding, 2026-04-24; widened same day):
        // reject rescue targets that land in the guest stack region. The top
        // ~128MB of the 512MB RAM reservation ([0x18000000, 0x20000000)) is
        // conventionally reserved for thread stacks and TLS — no legitimate
        // Xbox code lives there. A corrupted return address or stale stack
        // value can slip past `looks_like_ret_addr` (worker.rs) and reach
        // rescue_emit, where without this guard we'd dutifully compile guest
        // stack bytes as x86 instructions.
        //
        // Observed on Spider-Man:
        // - 0x1EFFFDC0 (main thread stack) — caught by initial 0x1E000000 cutoff
        // - 0x1CFFFFE0 (worker thread stack) — SLIPPED THROUGH initial cutoff,
        //   widened threshold to 0x18000000 to cover secondary thread stacks
        //
        // XBE sections max out at ~0x007FFFFF; VA heap bumps from 0x02000000
        // (limit much lower than 0x18000000 in practice). Physical-alias range
        // starts at 0x80000000 (handled by the post-guard < 0x2000_0000 check).
        if guest_addr >= 0x1800_0000 && guest_addr < 0x2000_0000 {
            static STACK_REJECT_LOG: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
            let n = STACK_REJECT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[RESCUE] REJECT stack-range target 0x{:08X} (in [0x1E000000, 0x20000000)) — not compiling",
                    guest_addr
                ));
            }
            return None;
        }

        if self.rescue_pool.is_null() || self.rescue_pool_used >= self.rescue_pool_size {
            return None;
        }

        // E2 consumer-side guard (2026-04-22): reject targets that fall in a
        // data-only section (.data / .rdata / .bss) even if the XBE header
        // flags the section as EXEC. Spider-Man's .data is flagged 0x7 (RWX)
        // and contains string tables + static globals, not code. A corrupted
        // function pointer or stale RET target can resolve into that range;
        // without this guard, rescue_emit would dutifully compile 1000+
        // ASCII bytes as x86 instructions, which decode to `gs: outsb`
        // (privileged) and `gs: popad` (invalid in x64) that fault at
        // runtime and spin the worker in AV retry loops.
        //
        // Observed on Spider-Man: guest 0x00476920 → rescue compiled 1978
        // instrs → fault at compiled 0x00476949 (inside the same block).
        // Returning None here makes the caller take the "bogus target" exit
        // path, which is correctly handled by the stack-scan recovery guard
        // already in worker.rs (commit 1c3eb54).
        if self
            .data_ranges
            .iter()
            .any(|&(s, e)| guest_addr >= s && guest_addr < e)
        {
            static REJECT_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = REJECT_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 5 {
                crate::xbox::emulator::debug_log(&format!(
                    "[RESCUE] REJECT data-section target 0x{:08X} (in .data/.rdata/.bss) — not compiling",
                    guest_addr
                ));
            }
            return None;
        }

        // Remove stale entry if it exists (broken compiled code from AOT pass)
        self.addr_hash.remove(guest_addr);

        // Read guest bytes (up to 4KB from target address)
        let decode_size = 4096u32;
        let guest_bytes = unsafe {
            if guest_addr < 0x2000_0000 {
                std::slice::from_raw_parts(
                    self.guest_mem_base.add(guest_addr as usize),
                    decode_size as usize,
                )
            } else {
                return None;
            }
        };

        let (decoded, _stats) = decoder::decode_section(guest_bytes, guest_addr);
        if decoded.is_empty() {
            crate::xbox::emulator::debug_log(&format!(
                "[RESCUE] decode empty for guest 0x{:08X} (bytes: {:02X} {:02X} {:02X} {:02X})",
                guest_addr, guest_bytes[0], guest_bytes[1], guest_bytes[2], guest_bytes[3]
            ));
            return None;
        }

        let memory = unsafe { GuestMemory::from_raw(self.guest_mem_base) };
        let block = emitter::emit_section(
            &decoded,
            guest_bytes,
            guest_addr,
            &memory,
            self.hash_shift as u8,
            self.thunk_table.as_slice(),
        );
        if block.code.is_empty() {
            crate::xbox::emulator::debug_log(&format!(
                "[RESCUE] emit empty for guest 0x{:08X} ({} decoded)",
                guest_addr,
                decoded.len()
            ));
            return None;
        }

        let needed = block.code.len() as u32;
        if self.rescue_pool_used + needed > self.rescue_pool_size {
            return None;
        }

        // Make rescue pool writable
        #[cfg(windows)]
        unsafe {
            use windows::Win32::System::Memory::*;
            let mut old = PAGE_PROTECTION_FLAGS(0);
            let _ = VirtualProtect(
                self.rescue_pool as *const _,
                self.rescue_pool_size as usize,
                PAGE_READWRITE,
                &mut old,
            );
        }

        let pool_offset = self.rescue_pool_used;
        let host_base_offset = self.rescue_pool_offset + pool_offset;
        unsafe {
            let dest = self.rescue_pool.add(pool_offset as usize);
            std::ptr::copy_nonoverlapping(block.code.as_ptr(), dest, block.code.len());
        }
        self.rescue_pool_used += needed;

        // Add addr_map entries to hash
        let mut first_offset = None;
        let mut inserted = 0u32;
        for entry in &block.addr_map {
            let ho = entry.host_offset + host_base_offset;
            if self.addr_hash.insert(entry.guest_addr, ho, AddrTier::Main) {
                inserted += 1;
            }
            if entry.guest_addr == guest_addr && first_offset.is_none() {
                first_offset = Some(ho);
            }
        }

        // Add trap table entries
        for trap in &block.traps {
            let mut adjusted = *trap;
            adjusted.host_offset += host_base_offset;
            self.trap_table.push(adjusted);
        }
        self.trap_table.sort_by_key(|t| t.host_offset);

        // Resolve branch fixups
        for fixup in &block.unresolved {
            if let Some(target_host) = self.addr_hash.lookup(fixup.guest_target) {
                let fixup_abs = host_base_offset + fixup.host_offset;
                let rel32 =
                    (target_host as i64 - (fixup_abs as i64 + fixup.instr_end_delta as i64)) as i32;
                unsafe {
                    let ptr = self
                        .rescue_pool
                        .add((pool_offset + fixup.host_offset) as usize);
                    *(ptr as *mut i32) = rel32;
                }
            }
        }

        // Make rescue pool executable
        #[cfg(windows)]
        unsafe {
            use windows::Win32::System::Memory::*;
            let mut old = PAGE_PROTECTION_FLAGS(0);
            let _ = VirtualProtect(
                self.rescue_pool as *const _,
                self.rescue_pool_used as usize,
                PAGE_EXECUTE_READ,
                &mut old,
            );
        }

        self.rescue_ranges.push((
            unsafe { self.rescue_pool as *const u8 },
            self.rescue_pool_used as usize,
        ));
        self.rescue_count += 1;

        let result = self.addr_hash.lookup(guest_addr).or(first_offset);
        crate::xbox::aot::jit::note_rescue_result(guest_addr, result, decoded.len(), needed);
        crate::xbox::emulator::debug_log(&format!(
            "[RESCUE] Compiled guest 0x{:08X}: {} instrs, {} bytes, {}/{} inserted -> host+0x{:X} (pool {}/{})",
            guest_addr, decoded.len(), needed, inserted, block.addr_map.len(),
            result.unwrap_or(0xFFFFFFFF), self.rescue_pool_used, self.rescue_pool_size
        ));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_state_size_and_offsets() {
        assert_eq!(std::mem::size_of::<GuestState>(), 0x48);
    }

    #[test]
    fn addr_hash_insert_lookup() {
        let mut hash = AddrHash::new(8); // 256 entries
        assert!(hash.insert(0x00010000, 0, AddrTier::Main));
        assert!(hash.insert(0x00010010, 100, AddrTier::Main));
        assert!(hash.insert(0x00010020, 200, AddrTier::Main));

        assert_eq!(hash.lookup(0x00010000), Some(0));
        assert_eq!(hash.lookup(0x00010010), Some(100));
        assert_eq!(hash.lookup(0x00010020), Some(200));
        assert_eq!(hash.lookup(0x00010030), None);
        assert_eq!(hash.count, 3);
    }

    #[test]
    fn addr_hash_tier_priority() {
        let mut hash = AddrHash::new(8);
        // Insert at tier Rescue
        assert!(hash.insert(0x1000, 100, AddrTier::Rescue));
        assert_eq!(hash.lookup(0x1000), Some(100));

        // Overwrite with Main (lower tier = higher priority)
        assert!(hash.insert(0x1000, 200, AddrTier::Main));
        assert_eq!(hash.lookup(0x1000), Some(200));

        // Try to overwrite Main with Rescue — should fail
        assert!(!hash.insert(0x1000, 300, AddrTier::Rescue));
        assert_eq!(hash.lookup(0x1000), Some(200));
    }

    #[test]
    fn addr_hash_zero_sentinel() {
        let mut hash = AddrHash::new(8);
        // guest_addr 0 is reserved as empty sentinel
        assert!(!hash.insert(0, 100, AddrTier::Main));
        assert_eq!(hash.lookup(0), None);
    }
}
