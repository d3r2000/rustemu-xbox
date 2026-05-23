use super::{KernelResult, KernelState};
/// Xbox kernel memory management — bump allocator.
/// Ported from AOT_KrnlMem.cpp.
///
/// Three bump regions:
/// - Contiguous (MmAllocateContiguous*): physical mirror 0x80D00000+ (phys 0x00D00000)
/// - Pool (ExAllocatePool): physical mirror 0x82000000+ (phys 0x02000000)
/// - Virtual (NtAllocateVirtualMemory): starts at 0x02000000
///
/// XDK docs: contiguous allocations are NOT zero-initialized.
/// XDK docs: VirtualAlloc MEM_COMMIT IS zero-initialized (unless MEM_NOZERO).
use crate::xbox::memory::guest_memory::GuestMemory;

// Contiguous bump: low physical addresses. Games pass HighestAcceptableAddress
// constraining where contiguous memory can live (e.g., Spider-Man: hi=0x11000FE).
// Physical mirror: 0x80D00000 = phys 0x00D00000 (above XBE sections at ~0x007F0000).
// Virtual alias: 0x00D00000-0x01DFFFFF. NOT zero-filled per XDK docs.
const CONTIG_BASE: u32 = 0x80D0_0000;
const CONTIG_CEIL: u32 = 0x8400_0000; // up to phys 0x04000000 (64MB) — Spider-Man needs 34MB+

// Pool bump (ExAllocatePool): above contiguous range in physical mirror.
const BUMP_BASE: u32 = 0x8400_0000;
const BUMP_CEIL: u32 = 0x8800_0000; // 64MB pool space

// VA bump: must be above contiguous alias end.
// Contiguous alias: 0x00D00000 to 0x03FFFFFF (phys mirror of 0x80D00000-0x83FFFFFF).
// Start VA bump above that.
const VA_BUMP_BASE: u32 = 0x0400_0000;
const PAGE_SIZE: u32 = 0x1000;
const STACK_SIZE: u32 = 0x10000; // 64KB kernel stacks

fn align_up(val: u32, align: u32) -> u32 {
    (val + align - 1) & !(align - 1)
}

fn seed_spiderman_crt_heap(memory: &GuestMemory, reason: &str) {
    const HEAP_BASE: u32 = 0x0400_0000;
    const FREE_ENTRY: u32 = HEAP_BASE + 0x0040;

    // Spider-Man's statically linked CRT exposes its process heap at 0x04000000.
    // The native RtlCreateHeap path can leave that page freshly zeroed under HLE,
    // so provide the minimal MSVC heap shape the native small-block allocator
    // expects when a cold/interpreter path misses the RtlAllocateHeap hook.
    memory.write_u32(HEAP_BASE + 0x00, 0x0000_0000);
    memory.write_u32(HEAP_BASE + 0x04, 0x0000_0000);
    memory.write_u32(HEAP_BASE + 0x08, 0xEEFF_EEFF);
    memory.write_u32(HEAP_BASE + 0x0C, 0x0000_2000);
    memory.write_u32(HEAP_BASE + 0x10, HEAP_BASE + 0x10);
    memory.write_u32(HEAP_BASE + 0x14, HEAP_BASE + 0x10);
    memory.write_u32(HEAP_BASE + 0x18, HEAP_BASE);
    memory.write_u32(HEAP_BASE + 0x1C, HEAP_BASE);
    memory.write_u32(HEAP_BASE + 0x20, 0x0000_0100);
    memory.write_u32(HEAP_BASE + 0x24, HEAP_BASE + 0x0040);
    memory.write_u32(HEAP_BASE + 0x28, HEAP_BASE + 0x1000);
    memory.write_u32(HEAP_BASE + 0x2C, 0x0000_00FF);

    memory.write_u16(FREE_ENTRY + 0x00, 0x01F8);
    memory.write_u16(FREE_ENTRY + 0x02, 0x0000);
    memory.write_u8(FREE_ENTRY + 0x04, 0x00);
    memory.write_u8(FREE_ENTRY + 0x05, 0x18);
    memory.write_u8(FREE_ENTRY + 0x06, 0x00);
    memory.write_u8(FREE_ENTRY + 0x07, 0x00);
    memory.write_u32(FREE_ENTRY + 0x08, FREE_ENTRY);
    memory.write_u32(FREE_ENTRY + 0x0C, FREE_ENTRY);

    crate::xbox::emulator::debug_log(&format!(
        "[SPIDEY-HEAP-SEED] {}: restored CRT heap header at 0x04000000",
        reason
    ));
}

/// Dispatch memory-related kernel ordinals.
/// args[] are the stdcall arguments read from the guest stack.
/// Returns (KernelResult, eax_value).
pub fn dispatch(
    ordinal: u32,
    args: &[u32; 12],
    state: &mut KernelState,
    memory: &GuestMemory,
) -> Option<(KernelResult, u32)> {
    use super::ordinals;
    match ordinal {
        // ExAllocatePool(size) — with 8-byte header for CRT heap compatibility
        ordinals::ExAllocatePool => {
            let user_size = args[0];
            let total = align_up(user_size + 8, 8); // 8-byte header + align
            let block = bump_alloc(&mut state.bump_pool, total, BUMP_CEIL, memory);
            if block != 0 {
                // Write header: [size:4][tag:4] then return ptr+8
                memory.write_u32(block, user_size);
                memory.write_u32(block + 4, 0x6C6F6F50); // 'Pool'
                Some((KernelResult::Handled, block + 8))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }

        // ExAllocatePoolWithTag(size, tag) — with 8-byte header
        ordinals::ExAllocatePoolWithTag => {
            let user_size = args[0];
            let tag = args[1];
            let total = align_up(user_size + 8, 8);
            let block = bump_alloc(&mut state.bump_pool, total, BUMP_CEIL, memory);
            if block != 0 {
                memory.write_u32(block, user_size);
                memory.write_u32(block + 4, tag);
                Some((KernelResult::Handled, block + 8))
            } else {
                Some((KernelResult::Handled, 0))
            }
        }

        // ExFreePool(ptr) — no-op but header at ptr-8 is valid
        ordinals::ExFreePool => {
            // Mark block as freed by zeroing the tag (optional)
            if args[0] >= 8 {
                memory.write_u32(args[0] - 4, 0); // clear tag = freed
            }
            Some((KernelResult::Handled, 0))
        }

        // ExQueryPoolBlockSize(ptr) — read size from header
        ordinals::ExQueryPoolBlockSize => {
            let ptr = args[0];
            let size = if ptr >= 8 {
                memory.read_u32(ptr - 8)
            } else {
                0
            };
            Some((KernelResult::Handled, size))
        }

        // MmAllocateContiguousMemory(size) — page-aligned, NOT zero-filled
        // XDK: "The allocated memory is not initialized to any known value."
        // Returns VIRTUAL address (low alias 0x00XXXXXX), not physical mirror (0x80XXXXXX).
        // The bump allocator works in the physical mirror range (0x80D00000+) for
        // internal tracking, but the returned pointer must be the virtual alias
        // because game code validates pointers against 0x10000000 bounds.
        // Memory is accessible through both aliases (MapViewOfFile3 mirror).
        ordinals::MmAllocateContiguousMemory => {
            let user_size = args[0].max(PAGE_SIZE);
            let total = align_up(user_size, PAGE_SIZE);
            let phys_addr = bump_alloc(&mut state.bump_contig, total, CONTIG_CEIL, memory);
            let virt_addr = phys_addr & 0x1FFF_FFFF; // strip 0x80000000 mirror bit
            Some((KernelResult::Handled, virt_addr))
        }

        // MmAllocateContiguousMemoryEx(size, lo, hi, align, prot) — page-aligned, NOT zero-filled
        // XDK: "The allocated memory is not initialized to any known value."
        // Xbox kernel ALWAYS returns page-aligned addresses (no header).
        // Returns VIRTUAL address (low alias), not physical mirror.
        ordinals::MmAllocateContiguousMemoryEx => {
            let user_size = args[0].max(PAGE_SIZE);
            let total = align_up(user_size, PAGE_SIZE);
            let phys_addr = bump_alloc(&mut state.bump_contig, total, CONTIG_CEIL, memory);
            let virt_addr = phys_addr & 0x1FFF_FFFF; // strip 0x80000000 mirror bit
            state.mm_contig_count += 1;
            crate::xbox::emulator::debug_log(&format!(
                "[KERNEL] MmAllocContiguousEx(size=0x{:X}, lo=0x{:X}, hi=0x{:X}, align=0x{:X}, prot=0x{:X}) -> 0x{:08X} (phys=0x{:08X}) bump=0x{:08X} (#{})",
                args[0], args[1], args[2], args[3], args[4], virt_addr, phys_addr, state.bump_contig, state.mm_contig_count
            ));
            Some((KernelResult::Handled, virt_addr))
        }

        // MmAllocateSystemMemory(size, prot) — zero-filled
        ordinals::MmAllocateSystemMemory => {
            let size = args[0].max(PAGE_SIZE);
            let size = align_up(size, PAGE_SIZE);
            let addr = bump_alloc(&mut state.bump_pool, size, BUMP_CEIL, memory);
            if addr != 0 {
                zero_fill(memory, addr, size);
            }
            Some((KernelResult::Handled, addr))
        }

        // MmFreeContiguousMemory(ptr) — no-op
        ordinals::MmFreeContiguousMemory => Some((KernelResult::Handled, 0)),
        // MmFreeSystemMemory(ptr, size) — no-op
        ordinals::MmFreeSystemMemory => Some((KernelResult::Handled, 0)),

        // MmCreateKernelStack(size, debug)
        ordinals::MmCreateKernelStack => {
            let size = if args[0] == 0 {
                STACK_SIZE
            } else {
                align_up(args[0], PAGE_SIZE)
            };
            let addr = bump_alloc(&mut state.bump_pool, size, BUMP_CEIL, memory);
            // Return top of stack (stack grows down)
            let top = if addr != 0 { addr + size } else { 0 };
            Some((KernelResult::Handled, top))
        }

        // MmDeleteKernelStack(stack_base, stack_limit) — no-op
        ordinals::MmDeleteKernelStack => Some((KernelResult::Handled, 0)),

        // MmGetPhysicalAddress(va) -> pa = va & 0x0FFFFFFF
        ordinals::MmGetPhysicalAddress => {
            let pa = args[0] & 0x0FFF_FFFF;
            Some((KernelResult::Handled, pa))
        }

        // MmIsAddressValid(addr) -> 1
        ordinals::MmIsAddressValid => Some((KernelResult::Handled, 1)),

        // MmMapIoSpace(phys, size, prot) -> phys | 0x80000000
        ordinals::MmMapIoSpace => {
            let mapped = args[0] | 0x8000_0000;
            Some((KernelResult::Handled, mapped))
        }
        // MmUnmapIoSpace — no-op
        ordinals::MmUnmapIoSpace => Some((KernelResult::Handled, 0)),

        // MmQueryAllocationSize(ptr) -> actual allocation size from header
        // Pool allocations (ExAllocatePool) have 8-byte header: [size:4][tag:4].
        // Contiguous allocations have NO header (page-aligned requirement).
        ordinals::MmQueryAllocationSize => {
            let ptr = args[0];
            if ptr >= 8 {
                let size = memory.read_u32(ptr - 8);
                let tag = memory.read_u32(ptr - 4);
                // Pool header: size must be sane, tag must be ASCII-ish
                if size > 0 && size < 0x1000_0000 && tag != 0 {
                    Some((KernelResult::Handled, size))
                } else {
                    // No valid header (likely contiguous) — return page-aligned estimate
                    Some((KernelResult::Handled, PAGE_SIZE))
                }
            } else {
                Some((KernelResult::Handled, PAGE_SIZE))
            }
        }

        // MmQueryStatistics(ptr) — write full MM_STATISTICS struct (Cxbx-R format)
        // 9 fields, 36 bytes total. Games validate Length == 36.
        ordinals::MmQueryStatistics => {
            let ptr = args[0];
            if ptr != 0 {
                memory.write_u32(ptr, 36); // +0x00: Length (36 bytes, NOT 28)
                memory.write_u32(ptr + 4, 16384); // +0x04: TotalPhysicalPages (64MB/4KB)
                memory.write_u32(ptr + 8, 14336); // +0x08: AvailablePages
                memory.write_u32(ptr + 12, 4096); // +0x0C: VirtualMemoryBytesCommitted
                memory.write_u32(ptr + 16, 8192); // +0x10: VirtualMemoryBytesReserved
                memory.write_u32(ptr + 20, 1024); // +0x14: CachePagesCommitted
                memory.write_u32(ptr + 24, 2048); // +0x18: PoolPagesCommitted
                memory.write_u32(ptr + 28, 512); // +0x1C: StackPagesCommitted
                memory.write_u32(ptr + 32, 4096); // +0x20: ImagePagesCommitted
            }
            Some((KernelResult::Handled, 0))
        }

        // MmQueryAddressProtect(addr) -> PAGE_READWRITE (0x04)
        ordinals::MmQueryAddressProtect => Some((KernelResult::Handled, 0x04)),
        // MmSetAddressProtect — no-op
        ordinals::MmSetAddressProtect => Some((KernelResult::Handled, 0)),
        // MmPersistContiguousMemory — no-op
        ordinals::MmPersistContiguousMemory => Some((KernelResult::Handled, 0)),
        // MmLockUnlockBufferPages — no-op
        ordinals::MmLockUnlockBufferPages => Some((KernelResult::Handled, 0)),
        // MmLockUnlockPhysicalPage — no-op
        ordinals::MmLockUnlockPhysicalPage => Some((KernelResult::Handled, 0)),

        // NtAllocateVirtualMemory(*base_ptr, zero, *size_ptr, alloc_type, prot)
        ordinals::NtAllocateVirtualMemory => {
            let base_ptr = args[0];
            let size_ptr = args[2]; // arg[1] is ZeroBits
            let alloc_type = args[3]; // MEM_COMMIT=0x1000, MEM_RESERVE=0x2000
            if base_ptr == 0 || size_ptr == 0 {
                return Some((KernelResult::Handled, 0xC000_0017)); // STATUS_NO_MEMORY
            }
            let req_base = memory.read_u32(base_ptr);
            let req_size = memory.read_u32(size_ptr);
            let size = align_up(req_size.max(PAGE_SIZE), PAGE_SIZE);

            let addr = if req_base != 0 {
                // Caller specifies address. Per XDK docs (memman_3elf.txt):
                //   MEM_COMMIT zero-fills returned memory unless MEM_NOZERO is set.
                //   MEM_RESERVE reserves address space (also typically zeroed).
                //
                // 2026-04-26: previous implementation only zero-filled on MEM_RESERVE,
                // skipping zero-fill on MEM_COMMIT-only calls. That violated XDK spec
                // and corrupted CRT heap descriptors written via the
                // NtAllocateVirtualMemory(MEM_RESERVE) → NtAllocateVirtualMemory(addr,
                // MEM_COMMIT) two-call pattern. Doom's `RtlCreateHeap`-style CRT init
                // hits exactly that pattern; with garbage in the committed pages the
                // heap descriptor's free-list sentinels were poisoned and every
                // subsequent malloc returned 0 → gamestate global at [0x101BB8] stayed
                // NULL → black screen.
                //
                // Match the XDK contract: zero-fill on MEM_COMMIT unless MEM_NOZERO.
                // MEM_RESERVE alone (without MEM_COMMIT) reserves address space; we
                // also zero it because our backing memory is always accessible and
                // the guest expects fresh-zero pages on first commit.
                let is_commit = (alloc_type & 0x1000) != 0; // MEM_COMMIT
                let is_reserve = (alloc_type & 0x2000) != 0; // MEM_RESERVE
                let is_nozero = (alloc_type & 0x0800) != 0; // MEM_NOZERO
                if (is_commit || is_reserve) && !is_nozero {
                    let end = req_base.wrapping_add(size);
                    let overlaps_xbe = if let Some((_, xbe_base, xbe_size)) = state.thunk_info {
                        let xbe_end = xbe_base.wrapping_add(xbe_size);
                        req_base < xbe_end && end > xbe_base
                    } else {
                        false
                    };
                    // Pre-AOT scaffolding carve-out: Spider-Man's
                    // `apply_pre_aot_patches` writes a fake _HEAP signature at
                    // 0x04000000 BEFORE guest code runs. Spider-Man's CRT then
                    // calls NtAllocateVirtualMemory(0x04000000, MEM_COMMIT) to
                    // commit those pages — the new zero-fill semantics would
                    // wipe the FAKE_HEAP signature mid-flight, breaking
                    // Spider-Man. Match by title id, not entrypoint: the worker
                    // startup thunk can enter at several nearby addresses.
                    const SPIDERMAN_FAKE_HEAP_BASE: u32 = 0x0400_0000;
                    const SPIDERMAN_FAKE_HEAP_END: u32 = 0x0400_1000;
                    let in_spiderman_fake_heap = state.xbe_title_id == 0x4156_0006
                        && req_base < SPIDERMAN_FAKE_HEAP_END
                        && end > SPIDERMAN_FAKE_HEAP_BASE;
                    if !overlaps_xbe
                        && !in_spiderman_fake_heap
                        && end > req_base
                        && req_base < 0x2000_0000
                    {
                        for i in (0..size).step_by(4) {
                            memory.write_u32(req_base + i, 0);
                        }
                    }
                }
                req_base
            } else {
                // No base address — fresh allocation (MEM_RESERVE or MEM_RESERVE|MEM_COMMIT)
                // Always zero-fill since this is new address space.
                va_bump_alloc(&mut state.bump_va, size, memory)
            };
            if addr == 0 {
                return Some((KernelResult::Handled, 0xC000_0017));
            }
            if state.xbe_title_id == 0x4156_0006
                && addr < 0x0400_1000
                && addr.wrapping_add(size) > 0x0400_0000
            {
                seed_spiderman_crt_heap(memory, "NtAllocateVirtualMemory");
            }
            memory.write_u32(base_ptr, addr);
            memory.write_u32(size_ptr, size);
            crate::xbox::emulator::debug_log(&format!(
                "NtAllocateVirtualMemory: req_base=0x{:08X} size=0x{:X} type=0x{:X} -> addr=0x{:08X}",
                req_base, size, alloc_type, addr
            ));
            Some((KernelResult::Handled, 0)) // STATUS_SUCCESS
        }

        // NtFreeVirtualMemory — no-op
        ordinals::NtFreeVirtualMemory => Some((KernelResult::Handled, 0)),

        // NtProtectVirtualMemory — no-op, return success
        ordinals::NtProtectVirtualMemory => Some((KernelResult::Handled, 0)),

        // NtQueryVirtualMemory
        ordinals::NtQueryVirtualMemory => {
            let info_ptr = args[1];
            if info_ptr != 0 {
                // MEMORY_BASIC_INFORMATION (28 bytes)
                // +0x10: State = MEM_COMMIT (0x1000)
                // +0x14: Protect = PAGE_READWRITE (0x04)
                memory.write_u32(info_ptr + 0x10, 0x1000);
                memory.write_u32(info_ptr + 0x14, 0x04);
            }
            Some((KernelResult::Handled, 0))
        }

        // MmClaimGpuInstanceMemory(size, *phys_out)
        ordinals::MmClaimGpuInstanceMemory => {
            // Return PRAMIN base (0xFD700000)
            if args[1] != 0 {
                memory.write_u32(args[1], 0x0070_0000); // physical offset
            }
            Some((KernelResult::Handled, 0xFD70_0000))
        }

        // Debug memory ordinals (374-378) — stubs
        ordinals::MmDbgAllocateMemory
        | ordinals::MmDbgFreeMemory
        | ordinals::MmDbgQueryAvailablePages
        | ordinals::MmDbgReleaseAddress
        | ordinals::MmDbgWriteCheck => Some((KernelResult::Handled, 0)),

        _ => None, // Not a memory ordinal
    }
}

/// Zero-fill a guest memory region (4 bytes at a time).
fn zero_fill(memory: &GuestMemory, addr: u32, size: u32) {
    for i in (0..size).step_by(4) {
        memory.write_u32(addr + i, 0);
    }
}

/// Bump allocate from the pool/contiguous allocator.
/// NO zero-fill — guest memory is initialized to zero at startup, and the physical
/// mirror (0x80000000+) maps to the same RAM as virtual addresses (0x00000000+).
/// Zero-filling through the mirror would destroy virtual allocations (e.g. the CRT
/// heap at 0x01100000) when the pool region grows past physical address 0x01100000.
fn bump_alloc(bump: &mut u32, size: u32, ceil: u32, _memory: &GuestMemory) -> u32 {
    let addr = *bump;
    let end = addr.wrapping_add(size);
    if end > ceil || end < addr {
        log::warn!(
            "kernel: bump alloc OOM (need 0x{:X}, at 0x{:08X})",
            size,
            addr
        );
        return 0;
    }
    *bump = end;
    addr
}

/// Public helper for callers outside this module that need a pool allocation
/// without going through the kernel ordinal dispatch path (e.g. synthetic
/// FrameContext injection from worker.rs). Aligns the request size up to
/// `align` and returns the guest address (physical-mirror) on success, or
/// `Err(())` on OOM.
pub fn pool_bump_alloc(
    state: &mut KernelState,
    memory: &GuestMemory,
    size: u32,
    align: u32,
) -> Result<u32, ()> {
    let effective_align = if align == 0 { 8 } else { align };
    let aligned_size = align_up(size.max(1), effective_align);
    let addr = bump_alloc(&mut state.bump_pool, aligned_size, BUMP_CEIL, memory);
    if addr == 0 {
        Err(())
    } else {
        Ok(addr)
    }
}

/// Public helper for callers that need a low guest virtual allocation
/// without going through the kernel ordinal dispatch path. This is suitable
/// for synthetic game-owned objects that downstream code expects below the
/// legacy 0x1000_0000 pointer cutoff.
pub fn va_bump_alloc_guest(
    state: &mut KernelState,
    memory: &GuestMemory,
    size: u32,
    align: u32,
) -> Result<u32, ()> {
    let effective_align = if align == 0 { 8 } else { align };
    let aligned_size = align_up(size.max(1), effective_align);
    let addr = va_bump_alloc(&mut state.bump_va, aligned_size, memory);
    if addr == 0 {
        Err(())
    } else {
        Ok(addr)
    }
}

/// Bump allocate from the virtual memory allocator. Zero-fills.
fn va_bump_alloc(bump: &mut u32, size: u32, memory: &GuestMemory) -> u32 {
    let addr = *bump;
    let end = addr.wrapping_add(size);
    // Xbox has 64MB RAM but our guest memory commits up to 512MB (gap region).
    // Halo allocates 240+ MB of 1MB chunks and retries infinitely on failure.
    if end > 0x2000_0000 || end < addr {
        log::warn!(
            "kernel: VA bump alloc OOM (need 0x{:X}, at 0x{:08X})",
            size,
            addr
        );
        return 0;
    }
    for i in (0..size).step_by(4) {
        memory.write_u32(addr + i, 0);
    }
    *bump = end;
    addr
}
