/// 4GB guest address space with 512MB RAM mirroring.
/// Maps same physical section at 0x00000000 and 0x80000000.
/// NV2A register ranges set PAGE_NOACCESS for MMIO trapping.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Failed to create file mapping: {0}")]
    SectionCreateFailed(std::io::Error),
    #[error("Failed to reserve 4GB address space: {0}")]
    ReservationFailed(std::io::Error),
    #[error("Failed to map RAM view: {0}")]
    MapFailed(std::io::Error),
    #[error("Failed to set memory protection: {0}")]
    ProtectFailed(std::io::Error),
    #[error("Failed to commit memory region: {0}")]
    CommitFailed(std::io::Error),
    #[error("Failed to split placeholder: {0}")]
    SplitFailed(std::io::Error),
}

/// Guest address space constants
pub const GUEST_ADDR_SPACE_SIZE: u64 = 0x1_0000_0000; // 4GB
pub const RAM_SIZE: u32 = 0x2000_0000; // 512MB (standard Xbox + headroom)
pub const RAM_MIRROR_BASE: u32 = 0x8000_0000; // Cached physical mirror
pub const RAM_UNCACHED_BASE: u32 = 0xA000_0000; // Uncached/write-combine physical mirror
pub const NV2A_REG_BASE: u32 = 0xFD00_0000;
pub const NV2A_REG_SIZE: u32 = 0x0070_0000; // 7MB primary registers
pub const PRAMIN_BASE: u32 = 0xFD70_0000;
pub const PRAMIN_SIZE: u32 = 0x0010_0000; // 1MB
pub const NV2A_USER_BASE: u32 = 0xFD80_0000;
pub const NV2A_USER_SIZE: u32 = 0x0000_2000; // 8KB
pub const MCPX_BASE: u32 = 0xFE00_0000;
pub const FLASH_BASE: u32 = 0xFF00_0000;

pub struct GuestMemory {
    base: *mut u8,
    #[cfg(windows)]
    section_handle: windows::Win32::Foundation::HANDLE,
    ram_size: u32,
    owned: bool, // true = we own the allocation and will free on drop
}

fn range_overlaps(start: u32, len: usize, watch_start: u32, watch_len: u32) -> bool {
    if len == 0 || watch_len == 0 {
        return false;
    }
    let end = start as u64 + len as u64;
    let watch_end = watch_start as u64 + watch_len as u64;
    (start as u64) < watch_end && end > watch_start as u64
}

fn log_bulk_write_watch(op: &str, guest_addr: u32, len: usize, data: Option<&[u8]>) {
    const WATCHES: &[(&str, u32, u32)] = &[
        ("spidey_text_12706", 0x0001_2706, 1),
        ("shenmue_d3d_isr", 0x002E_AE50, 0x20),
        ("shenmue_d3d_dpc", 0x002E_B820, 0x20),
    ];

    for &(label, watch_start, watch_len) in WATCHES {
        if !range_overlaps(guest_addr, len, watch_start, watch_len) {
            continue;
        }

        static WATCH_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = WATCH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n >= 64 && !n.is_power_of_two() {
            continue;
        }

        let sample = if let Some(bytes) = data {
            let sample_off = watch_start.saturating_sub(guest_addr) as usize;
            bytes
                .get(sample_off..)
                .unwrap_or_default()
                .iter()
                .take(16)
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            "zero-fill".to_string()
        };

        crate::xbox::emulator::debug_log(&format!(
            "[MEM-WRITE-WATCH #{}] op={} range=0x{:08X}..0x{:08X} len=0x{:X} covers={}@0x{:08X} sample=[{}]",
            n,
            op,
            guest_addr,
            guest_addr.wrapping_add(len as u32),
            len,
            label,
            watch_start,
            sample
        ));
    }
}

// SAFETY: GuestMemory is conceptually a large allocation owned by one thread.
// The emulator ensures single-threaded access during setup and uses proper
// synchronization during guest execution.
unsafe impl Send for GuestMemory {}

impl GuestMemory {
    /// Allocate 4GB guest address space with RAM mirroring.
    #[cfg(windows)]
    pub fn new() -> Result<Self, MemoryError> {
        use windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
        use windows::Win32::System::Memory::*;

        unsafe {
            // Step 1: Create 512MB section object (physical RAM backing)
            let section_handle = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                RAM_SIZE,
                None,
            )
            .map_err(|e| {
                MemoryError::SectionCreateFailed(std::io::Error::from_raw_os_error(e.code().0))
            })?;

            // Step 2: Reserve 4GB contiguous placeholder
            let base = VirtualAlloc2(
                None,
                None,
                GUEST_ADDR_SPACE_SIZE as usize,
                MEM_RESERVE | MEM_RESERVE_PLACEHOLDER,
                PAGE_NOACCESS.0,
                None,
            );
            if base.is_null() {
                let _ = CloseHandle(section_handle);
                return Err(MemoryError::ReservationFailed(
                    std::io::Error::last_os_error(),
                ));
            }
            let base = base as *mut u8;

            // Step 3: Split placeholder into regions for mapping
            // We need to split at RAM_SIZE boundary, RAM_MIRROR_BASE, and after mirror
            let split_result = Self::split_placeholder_regions(base);
            if let Err(e) = split_result {
                let _ = VirtualFree(base as *mut _, 0, MEM_RELEASE);
                let _ = CloseHandle(section_handle);
                return Err(e);
            }

            // Step 4: Map RAM at 0x00000000
            let ram_view = MapViewOfFile3(
                section_handle,
                None,
                Some(base as *mut _),
                0,
                RAM_SIZE as usize,
                MEM_REPLACE_PLACEHOLDER,
                PAGE_READWRITE.0,
                None,
            );
            if ram_view.Value.is_null() {
                let _ = VirtualFree(base as *mut _, 0, MEM_RELEASE);
                let _ = CloseHandle(section_handle);
                return Err(MemoryError::MapFailed(std::io::Error::last_os_error()));
            }

            // Step 5: Map RAM mirror at 0x80000000
            let mirror_view = MapViewOfFile3(
                section_handle,
                None,
                Some(base.add(RAM_MIRROR_BASE as usize) as *mut _),
                0,
                RAM_SIZE as usize,
                MEM_REPLACE_PLACEHOLDER,
                PAGE_READWRITE.0,
                None,
            );
            if mirror_view.Value.is_null() {
                let _ = UnmapViewOfFile(ram_view);
                let _ = VirtualFree(base as *mut _, 0, MEM_RELEASE);
                let _ = CloseHandle(section_handle);
                return Err(MemoryError::MapFailed(std::io::Error::last_os_error()));
            }

            // Step 5b: Map uncached RAM mirror at 0xA0000000
            let uncached_view = MapViewOfFile3(
                section_handle,
                None,
                Some(base.add(RAM_UNCACHED_BASE as usize) as *mut _),
                0,
                RAM_SIZE as usize,
                MEM_REPLACE_PLACEHOLDER,
                PAGE_READWRITE.0,
                None,
            );
            if uncached_view.Value.is_null() {
                crate::xbox::emulator::debug_log(&format!(
                    "Guest memory: WARNING — uncached mirror at 0xA0000000 failed: {}",
                    std::io::Error::last_os_error()
                ));
                // Non-fatal: fall through, VEH will handle AV at 0xA0000000
            } else {
                crate::xbox::emulator::debug_log(
                    "Guest memory: uncached mirror mapped at 0xA0000000-0xBFFFFFFF",
                );
            }

            // Step 5.5: Commit the gap between RAM (512MB) and mirror (2GB)
            // The bump allocator grows beyond 512MB during game init (480MB+ in 14K kernel calls).
            // Release the entire gap placeholder, then commit it as private RW pages.
            let gap_addr = base.add(RAM_SIZE as usize);
            let gap_size = (RAM_MIRROR_BASE - RAM_SIZE) as usize; // 0x60000000 = 1.5GB
                                                                  // Release the placeholder (already split as one region by split_placeholder_regions)
            let _ = VirtualFree(gap_addr as *mut _, 0, MEM_RELEASE);
            let gap_region = VirtualAlloc(
                Some(gap_addr as *mut _),
                gap_size,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );
            if !gap_region.is_null() {
                crate::xbox::emulator::debug_log(&format!(
                    "Guest memory: committed gap region 0x{:08X}-0x{:08X} ({}MB)",
                    RAM_SIZE,
                    RAM_MIRROR_BASE,
                    gap_size / (1024 * 1024)
                ));
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "Guest memory: WARNING — gap region commit failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            // Step 5.6: Commit upper gap (0xC0000000-0xFCFFFFFF)
            let upper_gap_addr = base.add((RAM_UNCACHED_BASE + RAM_SIZE) as usize);
            let upper_gap_size = (NV2A_REG_BASE - (RAM_UNCACHED_BASE + RAM_SIZE)) as usize;
            let _ = VirtualFree(upper_gap_addr as *mut _, 0, MEM_RELEASE);
            let upper_gap_region = VirtualAlloc(
                Some(upper_gap_addr as *mut _),
                upper_gap_size,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );
            if !upper_gap_region.is_null() {
                crate::xbox::emulator::debug_log(&format!(
                    "Guest memory: committed upper gap 0x{:08X}-0x{:08X} ({}MB)",
                    RAM_UNCACHED_BASE + RAM_SIZE,
                    NV2A_REG_BASE,
                    upper_gap_size / (1024 * 1024)
                ));
            } else {
                crate::xbox::emulator::debug_log(&format!(
                    "Guest memory: WARNING — upper gap commit failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            // Step 6: Commit GPU/Flash regions (0xFD000000-0xFFFFFFFF)
            // Placeholder can't be committed directly — release it first, then reserve+commit
            let gpu_addr = base.add(NV2A_REG_BASE as usize);
            let gpu_size = (0x1_0000_0000u64 - NV2A_REG_BASE as u64) as usize;
            let _ = VirtualFree(gpu_addr as *mut _, 0, MEM_RELEASE);
            let gpu_region = VirtualAlloc(
                Some(gpu_addr as *mut _),
                gpu_size,
                MEM_RESERVE | MEM_COMMIT,
                PAGE_READWRITE,
            );
            if gpu_region.is_null() {
                let _ = UnmapViewOfFile(mirror_view);
                let _ = UnmapViewOfFile(ram_view);
                let _ = CloseHandle(section_handle);
                return Err(MemoryError::CommitFailed(std::io::Error::last_os_error()));
            }

            // Step 7: Set NV2A register ranges to PAGE_NOACCESS for MMIO trapping
            let mut old_protect = PAGE_PROTECTION_FLAGS(0);

            // Primary NV2A registers (0xFD000000 - 0xFD6FFFFF)
            VirtualProtect(
                base.add(NV2A_REG_BASE as usize) as *mut _,
                NV2A_REG_SIZE as usize,
                PAGE_NOACCESS,
                &mut old_protect,
            )
            .map_err(|_| MemoryError::ProtectFailed(std::io::Error::last_os_error()))?;

            // NV2A USER channel (0xFD800000 - 0xFD801FFF)
            VirtualProtect(
                base.add(NV2A_USER_BASE as usize) as *mut _,
                NV2A_USER_SIZE as usize,
                PAGE_NOACCESS,
                &mut old_protect,
            )
            .map_err(|_| MemoryError::ProtectFailed(std::io::Error::last_os_error()))?;

            log::info!(
                "GuestMemory: 4GB reserved at {:p}, 512MB RAM mirrored, NV2A ranges protected",
                base
            );

            Ok(GuestMemory {
                base,
                section_handle,
                ram_size: RAM_SIZE,
                owned: true,
            })
        }
    }

    #[cfg(windows)]
    unsafe fn split_placeholder_regions(base: *mut u8) -> Result<(), MemoryError> {
        use windows::Win32::System::Memory::*;

        // MEM_RELEASE | MEM_PRESERVE_PLACEHOLDER — windows crate types them differently,
        // so construct from raw bits: MEM_RELEASE=0x8000, MEM_PRESERVE_PLACEHOLDER=0x2
        let release_placeholder = VIRTUAL_FREE_TYPE(MEM_RELEASE.0 | 0x0000_0002);

        // Split at RAM_SIZE (0x20000000) — separates RAM from middle gap
        VirtualFree(base as *mut _, RAM_SIZE as usize, release_placeholder)
            .map_err(|_| MemoryError::SplitFailed(std::io::Error::last_os_error()))?;

        // Split at RAM_MIRROR_BASE (0x80000000) — separates middle gap from mirror
        VirtualFree(
            base.add(RAM_SIZE as usize) as *mut _,
            (RAM_MIRROR_BASE - RAM_SIZE) as usize,
            release_placeholder,
        )
        .map_err(|_| MemoryError::SplitFailed(std::io::Error::last_os_error()))?;

        // Split at end of mirror (0xA0000000) — separates mirror from upper region
        let mirror_end = RAM_MIRROR_BASE + RAM_SIZE;
        VirtualFree(
            base.add(RAM_MIRROR_BASE as usize) as *mut _,
            RAM_SIZE as usize,
            release_placeholder,
        )
        .map_err(|_| MemoryError::SplitFailed(std::io::Error::last_os_error()))?;

        // Split at end of uncached mirror (0xC0000000) — separates uncached mirror from upper gap
        let uncached_end = RAM_UNCACHED_BASE + RAM_SIZE;
        VirtualFree(
            base.add(mirror_end as usize) as *mut _,
            RAM_SIZE as usize,
            release_placeholder,
        )
        .map_err(|_| MemoryError::SplitFailed(std::io::Error::last_os_error()))?;

        // Split at NV2A_REG_BASE (0xFD000000) — separates upper gap from GPU region
        VirtualFree(
            base.add(uncached_end as usize) as *mut _,
            (NV2A_REG_BASE - uncached_end) as usize,
            release_placeholder,
        )
        .map_err(|_| MemoryError::SplitFailed(std::io::Error::last_os_error()))?;

        Ok(())
    }

    /// Base pointer for guest memory (equivalent to R15 in the AOT register contract).
    pub fn base(&self) -> *mut u8 {
        self.base
    }

    /// RAM size in bytes.
    pub fn ram_size(&self) -> u32 {
        self.ram_size
    }

    /// Read a u8 from guest address space.
    pub fn read_u8(&self, guest_addr: u32) -> u8 {
        unsafe { *self.base.add(guest_addr as usize) }
    }

    /// Read a u16 (little-endian) from guest address space.
    pub fn read_u16(&self, guest_addr: u32) -> u16 {
        unsafe {
            let ptr = self.base.add(guest_addr as usize) as *const u16;
            ptr.read_unaligned()
        }
    }

    /// Read a u32 (little-endian) from guest address space.
    pub fn read_u32(&self, guest_addr: u32) -> u32 {
        // NV2A MMIO range: route to shadow registers (interpreter path)
        if guest_addr >= 0xFD00_0000 && guest_addr < 0xFD90_0000 {
            let offset = guest_addr - 0xFD00_0000;
            return crate::xbox::aot::nv2a::shadow_read(offset);
        }
        unsafe {
            let ptr = self.base.add(guest_addr as usize) as *const u32;
            ptr.read_unaligned()
        }
    }

    /// Write a u8 to guest address space.
    pub fn write_u8(&self, guest_addr: u32, val: u8) {
        unsafe {
            *self.base.add(guest_addr as usize) = val;
        }
    }

    /// Write a u16 (little-endian) to guest address space.
    pub fn write_u16(&self, guest_addr: u32, val: u16) {
        unsafe {
            let ptr = self.base.add(guest_addr as usize) as *mut u16;
            ptr.write_unaligned(val);
        }
    }

    /// Write a u32 (little-endian) to guest address space.
    pub fn write_u32(&self, guest_addr: u32, val: u32) {
        // [STACK-SMASH-WATCH 2026-04-22] catch the writer that stores
        // 0x00476920 to [0x1EFFFEF4] (the utility function's ret_addr
        // slot). Captures caller backtrace via std::backtrace.
        if guest_addr == 0x1EFF_FEF4
            || (val == 0x0047_6920 && guest_addr >= 0x1EFF_0000 && guest_addr < 0x1F00_0000)
        {
            static SMASH_LOG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = SMASH_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 30 {
                let bt = std::backtrace::Backtrace::force_capture();
                crate::xbox::emulator::debug_log(&format!(
                    "[STACK-SMASH-WATCH #{}] write_u32(0x{:08X}, 0x{:08X})\n{}",
                    n, guest_addr, val, bt
                ));
            }
        }

        // NV2A MMIO range: route to shadow registers (interpreter path)
        if guest_addr >= 0xFD00_0000 && guest_addr < 0xFD90_0000 {
            let offset = guest_addr - 0xFD00_0000;
            crate::xbox::aot::nv2a::shadow_write(offset, val);
            return;
        }
        unsafe {
            let ptr = self.base.add(guest_addr as usize) as *mut u32;
            ptr.write_unaligned(val);
        }
    }

    /// Bulk copy data into guest address space.
    pub fn copy_into(&self, guest_addr: u32, data: &[u8]) {
        log_bulk_write_watch("copy_into", guest_addr, data.len(), Some(data));
        unsafe {
            let dst = self.base.add(guest_addr as usize);
            std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        }
    }

    /// Zero-fill a region of guest address space.
    pub fn zero_fill(&self, guest_addr: u32, len: usize) {
        log_bulk_write_watch("zero_fill", guest_addr, len, None);
        unsafe {
            let dst = self.base.add(guest_addr as usize);
            std::ptr::write_bytes(dst, 0, len);
        }
    }

    /// Get a slice view into guest address space.
    pub fn slice(&self, guest_addr: u32, len: usize) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.base.add(guest_addr as usize), len) }
    }

    /// Get a mutable slice view into guest address space.
    pub fn slice_mut(&mut self, guest_addr: u32, len: usize) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.base.add(guest_addr as usize), len) }
    }

    /// Create a non-owning GuestMemory from a raw base pointer.
    /// Used by VEH inline kernel dispatch where only the pointer is available.
    /// The returned object will NOT free memory on drop.
    pub unsafe fn from_raw(base: *mut u8) -> GuestMemory {
        GuestMemory {
            base,
            #[cfg(windows)]
            section_handle: windows::Win32::Foundation::HANDLE(std::ptr::null_mut()),
            ram_size: RAM_SIZE,
            owned: false,
        }
    }

    /// Create a non-owning clone that shares the same memory region.
    /// The clone will NOT free memory on drop — only the original owner does.
    /// Used for worker threads that need `&GuestMemory` for kernel dispatch.
    #[cfg(windows)]
    pub fn clone_shared(&self) -> GuestMemory {
        GuestMemory {
            base: self.base,
            section_handle: self.section_handle,
            ram_size: self.ram_size,
            owned: false,
        }
    }
}

#[cfg(windows)]
impl Drop for GuestMemory {
    fn drop(&mut self) {
        if !self.owned {
            return;
        } // non-owning clone — don't free

        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::Memory::*;

        unsafe {
            // Unmap RAM view
            let _ = UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: self.base as *mut _,
            });
            // Unmap mirror view
            let _ = UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: self.base.add(RAM_MIRROR_BASE as usize) as *mut _,
            });
            // Unmap uncached mirror view
            let _ = UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: self.base.add(RAM_UNCACHED_BASE as usize) as *mut _,
            });
            // Free remaining placeholder regions
            let _ = VirtualFree(self.base.add(RAM_SIZE as usize) as *mut _, 0, MEM_RELEASE);
            let _ = VirtualFree(
                self.base.add((RAM_UNCACHED_BASE + RAM_SIZE) as usize) as *mut _,
                0,
                MEM_RELEASE,
            );
            let _ = VirtualFree(
                self.base.add(NV2A_REG_BASE as usize) as *mut _,
                0,
                MEM_RELEASE,
            );
            // Close section handle
            let _ = CloseHandle(self.section_handle);
        }
        log::info!("GuestMemory: released 4GB reservation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_mirroring() {
        let mem = GuestMemory::new().expect("Failed to allocate guest memory");

        // Write at 0x1000
        mem.write_u32(0x1000, 0xDEADBEEF);
        // Read from mirror at 0x80001000
        let val = mem.read_u32(0x8000_1000);
        assert_eq!(
            val, 0xDEADBEEF,
            "RAM mirroring failed: write at 0x1000, read at 0x80001000"
        );

        // Write at mirror, read from main
        mem.write_u32(0x8000_2000, 0xCAFEBABE);
        let val = mem.read_u32(0x2000);
        assert_eq!(
            val, 0xCAFEBABE,
            "Reverse mirror failed: write at 0x80002000, read at 0x2000"
        );
    }

    #[test]
    fn test_bulk_copy() {
        let mem = GuestMemory::new().expect("Failed to allocate guest memory");
        let data = [0x41u8, 0x42, 0x43, 0x44];
        mem.copy_into(0x5000, &data);
        assert_eq!(mem.read_u32(0x5000), 0x44434241); // Little-endian
    }
}
