/// DirectSound HLE — fake IDirectSound8 / IDirectSoundBuffer / IDirectSoundStream.
///
/// Xbox DirectSound is stdcall COM with custom extensions (no IUnknown::QueryInterface
/// in most paths, but AddRef/Release exist). We create fake objects with no-op vtable
/// stubs in guest memory so game code can call methods without crashing.
///
/// ## Object layout in guest memory
///
/// Each fake COM object occupies a contiguous region:
///   +0x00: vtable pointer (points to stub function table)
///   +0x04: reference count (starts at 1)
///   +0x08: object type tag (0xD500 = dsound, 0xD5B0 = buffer, 0xD5C0 = stream)
///   +0x0C: buffer data pointer (for buffers: guest address of audio data region)
///   +0x10: buffer size in bytes
///   +0x14: play flags (0 = stopped, DSBPLAY_LOOPING = 1)
///   +0x18: volume (hundredths of dB, 0 = max, -10000 = silence)
///   +0x1C: frequency (Hz, e.g. 48000)
///   +0x20: play cursor position (bytes)
///   +0x24: write cursor position (bytes, always play + WRITE_LEAD)
///   +0x28: status flags (DSBSTATUS_*)
///
/// ## Vtable stub approach
///
/// Each vtable entry points to a `RET N` instruction (stdcall cleanup). The actual
/// HLE logic is handled by OOVPA pattern intercepts in the VEH handler, not by
/// executing these stubs. The stubs exist only as crash safety: if the game calls
/// a method we don't intercept via OOVPA, it hits a harmless RET instead of faulting.
///
/// ## Buffer registry
///
/// All allocated buffers/streams are tracked in a global registry keyed by guest
/// address. This lets GetCurrentPosition/GetStatus/Lock look up per-buffer state.
///
/// Reference: Cxbx-R CxbxKrnl/EmuDSound.cpp, C++ v1 AOT_DSound.cpp.
use crate::xbox::emulator::debug_log;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

// ============================================================================
// Constants
// ============================================================================

/// Xbox DSBPLAY flags.
pub const DSBPLAY_LOOPING: u32 = 0x0000_0001;

/// Xbox DSBSTATUS flags.
pub const DSBSTATUS_PLAYING: u32 = 0x0000_0001;
pub const DSBSTATUS_LOOPING: u32 = 0x0000_0004;

/// DirectSound notification offset fired when playback stops.
const DSBPN_OFFSETSTOP: u32 = 0xFFFF_FFFF;

/// Write cursor leads play cursor by this many bytes.
const WRITE_CURSOR_LEAD: u32 = 1024;

/// Maximum buffer size we allow (1 MB). Prevents insane allocations from bad args.
const MAX_BUFFER_SIZE: u32 = 1024 * 1024;

/// Default buffer size when game doesn't specify or specifies 0.
const DEFAULT_BUFFER_SIZE: u32 = 32768;

/// Bytes per "tick" for simulated play cursor advancement.
/// 48000 Hz * 2 channels * 2 bytes/sample = 192000 bytes/sec.
/// At 60 fps, one frame = 3200 bytes.
const BYTES_PER_TICK: u32 = 3200;

/// Object type tags written at +0x08 (diagnostic only).
const TAG_DSOUND: u32 = 0x0000_D500;
const TAG_BUFFER: u32 = 0x0000_D5B0;
const TAG_STREAM: u32 = 0x0000_D5C0;

// ============================================================================
// Guest memory regions for fake objects
// ============================================================================

/// Shared vtable/stub region for IDirectSound8.
const DS_VTABLE_ADDR: u32 = 0x00EA_1100;
const DS_STUBS_ADDR: u32 = 0x00EA_1200;

/// Shared vtable/stub region for IDirectSoundBuffer.
const DSB_VTABLE_ADDR: u32 = 0x00EA_2100;
const DSB_STUBS_ADDR: u32 = 0x00EA_2200;

/// Shared vtable/stub region for IDirectSoundStream.
const DSS_VTABLE_ADDR: u32 = 0x00EA_3100;
const DSS_STUBS_ADDR: u32 = 0x00EA_3200;

/// Scratch audio data region base — buffers share this.
/// Each buffer gets a 64 KB slice: buffer N uses 0x00EC0000 + N * 0x10000.
const SCRATCH_AUDIO_BASE: u32 = 0x00EC_0000;
const SCRATCH_AUDIO_STRIDE: u32 = 0x0001_0000; // 64 KB per buffer

// ============================================================================
// Per-buffer / per-stream state (host-side tracking)
// ============================================================================

/// State tracked for each DirectSound buffer or stream.
#[derive(Clone)]
pub struct DsBufferState {
    /// Guest address of this object (the "this" pointer).
    pub guest_addr: u32,
    /// Audio data region in guest memory (where Lock returns pointers to).
    pub data_addr: u32,
    /// Buffer size in bytes.
    pub buffer_size: u32,
    /// True if Play has been called and Stop has not.
    pub playing: bool,
    /// True if playing with DSBPLAY_LOOPING.
    pub looping: bool,
    /// Play cursor position (bytes into buffer).
    pub play_cursor: u32,
    /// Volume in hundredths of dB (0 = max, -10000 = silence).
    pub volume: i32,
    /// Frequency in Hz.
    pub frequency: u32,
    /// Number of channels.
    pub channels: u16,
    /// Bits per sample.
    pub bits_per_sample: u16,
    /// Tick counter for position advancement.
    pub tick_count: u64,
    /// True if this is a stream (vs buffer).
    pub is_stream: bool,
    /// DirectSound position notifications registered with SetNotificationPositions.
    notifications: Vec<DsNotifyPosition>,
}

#[derive(Clone)]
struct DsNotifyPosition {
    offset: u32,
    event: u32,
    fired: bool,
}

impl DsBufferState {
    fn new(guest_addr: u32, data_addr: u32, buffer_size: u32, is_stream: bool) -> Self {
        Self {
            guest_addr,
            data_addr,
            buffer_size,
            playing: false,
            looping: false,
            play_cursor: 0,
            volume: 0,
            frequency: 48000,
            channels: 2,
            bits_per_sample: 16,
            tick_count: 0,
            is_stream,
            notifications: Vec::new(),
        }
    }

    /// Advance the play cursor by one tick (called from DoWork / frame tick).
    fn advance_cursor(&mut self) {
        if !self.playing || self.buffer_size == 0 {
            return;
        }
        self.tick_count += 1;
        let next = self.play_cursor.saturating_add(BYTES_PER_TICK);
        if self.looping {
            self.play_cursor = next % self.buffer_size;
        } else if next >= self.buffer_size {
            // Non-looping buffers must eventually clear DSBSTATUS_PLAYING.
            // Spider-Man waits on this after the title-screen confirm blip.
            self.play_cursor = self.buffer_size;
            self.playing = false;
        } else {
            self.play_cursor = next;
        }
    }

    /// Current write cursor (always leads play cursor).
    fn write_cursor(&self) -> u32 {
        if self.buffer_size == 0 {
            return 0;
        }
        (self.play_cursor + WRITE_CURSOR_LEAD) % self.buffer_size
    }

    /// Status flags matching DSBSTATUS_*.
    fn status_flags(&self) -> u32 {
        if !self.playing {
            return 0;
        }
        let mut flags = DSBSTATUS_PLAYING;
        if self.looping {
            flags |= DSBSTATUS_LOOPING;
        }
        flags
    }
}

fn sync_buffer_header(guest_mem: *mut u8, buf: &DsBufferState) {
    guest_write_u32(guest_mem, buf.guest_addr + 0x0C, buf.data_addr);
    guest_write_u32(guest_mem, buf.guest_addr + 0x10, buf.buffer_size);
    guest_write_u32(
        guest_mem,
        buf.guest_addr + 0x14,
        if buf.looping { DSBPLAY_LOOPING } else { 0 },
    );
    guest_write_u32(guest_mem, buf.guest_addr + 0x18, buf.volume as u32);
    guest_write_u32(guest_mem, buf.guest_addr + 0x1C, buf.frequency);
    guest_write_u32(guest_mem, buf.guest_addr + 0x20, buf.play_cursor);
    guest_write_u32(guest_mem, buf.guest_addr + 0x24, buf.write_cursor());
    guest_write_u32(guest_mem, buf.guest_addr + 0x28, buf.status_flags());
}

fn notify_position_crossed(
    notify_offset: u32,
    old_cursor: u32,
    new_cursor: u32,
    wrapped: bool,
    stopped: bool,
    buffer_size: u32,
) -> bool {
    if notify_offset == DSBPN_OFFSETSTOP {
        return stopped;
    }

    if buffer_size == 0 || notify_offset >= buffer_size {
        return stopped;
    }

    if wrapped {
        notify_offset > old_cursor || notify_offset <= new_cursor
    } else {
        notify_offset > old_cursor && notify_offset <= new_cursor
    }
}

fn signal_guest_event_like(guest_mem: *mut u8, event: u32) -> bool {
    if event == 0 {
        return false;
    }

    // Some callers pass an initialized KEVENT body directly.
    if (0x0001_0000..0x1000_0000).contains(&event) {
        guest_write_u32(guest_mem, event + 0x04, 1);
        return true;
    }

    // NtCreateEvent returns a guest handle. The kernel object body allocator is
    // monotonic from 0x00F00000 in the same order as guest handles, so this is a
    // conservative fallback that only writes if the guessed body looks like an event.
    if (0x1001..0x9000).contains(&event) {
        let idx = event - 0x1001;
        let guessed = 0x00F0_0000 + idx.saturating_mul(0x20);
        let ty = guest_read_u8(guest_mem, guessed);
        let size = guest_read_u8(guest_mem, guessed + 0x02);
        if (ty == 0 || ty == 1) && size == 4 {
            guest_write_u32(guest_mem, guessed + 0x04, 1);
            return true;
        }
    }

    false
}

fn fire_buffer_notifications(
    guest_mem: *mut u8,
    buf: &mut DsBufferState,
    old_cursor: u32,
    new_cursor: u32,
    wrapped: bool,
    stopped: bool,
) {
    if buf.notifications.is_empty() {
        return;
    }

    for notify in &mut buf.notifications {
        if notify.fired && !buf.looping {
            continue;
        }

        if !notify_position_crossed(
            notify.offset,
            old_cursor,
            new_cursor,
            wrapped,
            stopped,
            buf.buffer_size,
        ) {
            continue;
        }

        let apu_prof = crate::xbox::profiler::start(crate::xbox::profiler::CpuPhase::ApuNotify);
        let signaled = signal_guest_event_like(guest_mem, notify.event);
        if !buf.looping {
            notify.fired = true;
        }

        let n = DSOUND_NOTIFY_LOGS.fetch_add(1, Ordering::Relaxed);
        if n < 64 || n.is_power_of_two() {
            debug_log(&format!(
                "[APU-NOTIFY] buf=0x{:08X} off=0x{:08X} event=0x{:08X} cursor={}->{} wrapped={} stopped={} signaled={}",
                buf.guest_addr,
                notify.offset,
                notify.event,
                old_cursor,
                new_cursor,
                wrapped,
                stopped,
                signaled
            ));
        }
        crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::ApuNotify, apu_prof);
    }
}

fn log_spiderman_sound_slots(guest_mem: *mut u8, tick: u64, playing_count: usize) {
    if !(tick <= 8 || tick.is_power_of_two()) {
        return;
    }

    const INSTANCE_BASE: u32 = 0x0073_4418;
    const INSTANCE_STRIDE: u32 = 0x38;
    const INSTANCE_COUNT: u32 = 256;
    const EMITTER_BASE: u32 = 0x0073_2418;

    let emitter_active = guest_read_u32(guest_mem, EMITTER_BASE + 0x08);
    let emitter_head = guest_read_u32(guest_mem, EMITTER_BASE + 0x18);
    let mut first_active = None;

    for i in 0..INSTANCE_COUNT {
        let slot = INSTANCE_BASE + i * INSTANCE_STRIDE;
        let handle = guest_read_u32(guest_mem, slot);
        let active = unsafe { *((guest_mem as u64 + slot as u64 + 0x18) as *const u8) };
        if handle != 0 || active != 0 {
            first_active = Some((i, slot, handle));
            break;
        }
    }

    if let Some((idx, slot, handle)) = first_active {
        let linked_emitter = guest_read_u32(guest_mem, slot + 0x04);
        let sample_ptr = guest_read_u32(guest_mem, slot + 0x08);
        let aux_ptr = guest_read_u32(guest_mem, slot + 0x0C);
        let ds_buf = guest_read_u32(guest_mem, slot + 0x14);
        let active = unsafe { *((guest_mem as u64 + slot as u64 + 0x18) as *const u8) };
        let pending = unsafe { *((guest_mem as u64 + slot as u64 + 0x19) as *const u8) };
        let playing = unsafe { *((guest_mem as u64 + slot as u64 + 0x1A) as *const u8) };
        let queued = unsafe { *((guest_mem as u64 + slot as u64 + 0x1B) as *const u8) };
        let stopped = unsafe { *((guest_mem as u64 + slot as u64 + 0x1E) as *const u8) };
        let ref_count = guest_read_u32(guest_mem, slot + 0x20);
        debug_log(&format!(
            "[APU-SLOT] tick={} hle_playing={} emitter_active=0x{:08X} emitter_head=0x{:08X} \
             slot#{}=0x{:08X} handle=0x{:08X} emitter=0x{:08X} sample=0x{:08X} aux=0x{:08X} \
             dsbuf=0x{:08X} f18={} f19={} f1A={} f1B={} f1E={} ref20={}",
            tick,
            playing_count,
            emitter_active,
            emitter_head,
            idx,
            slot,
            handle,
            linked_emitter,
            sample_ptr,
            aux_ptr,
            ds_buf,
            active,
            pending,
            playing,
            queued,
            stopped,
            ref_count
        ));
    } else {
        debug_log(&format!(
            "[APU-SLOT] tick={} hle_playing={} emitter_active=0x{:08X} emitter_head=0x{:08X} no active slots",
            tick, playing_count, emitter_active, emitter_head
        ));
    }
}

fn retire_spiderman_finished_slot_for_buffer(guest_mem: *mut u8, buf: &DsBufferState) {
    if buf.playing || buf.looping || buf.guest_addr == 0 {
        return;
    }

    const INSTANCE_BASE: u32 = 0x0073_4418;
    const INSTANCE_STRIDE: u32 = 0x38;
    const INSTANCE_COUNT: u32 = 256;

    for i in 0..INSTANCE_COUNT {
        let slot = INSTANCE_BASE + i * INSTANCE_STRIDE;
        if guest_read_u32(guest_mem, slot + 0x14) != buf.guest_addr {
            continue;
        }

        let active = unsafe { *((guest_mem as u64 + slot as u64 + 0x18) as *const u8) };
        let playing = unsafe { *((guest_mem as u64 + slot as u64 + 0x1A) as *const u8) };
        let stopped = unsafe { *((guest_mem as u64 + slot as u64 + 0x1E) as *const u8) };
        if active == 0 || playing == 0 || stopped != 0 {
            continue;
        }

        // Spider-Man's mixer normally reaches sub_002B2E00 after GetStatus
        // reports a stopped non-looping buffer. The default emitter remains
        // inactive in the current HLE path, so mirror only that completion bit.
        unsafe {
            *((guest_mem as u64 + slot as u64 + 0x1E) as *mut u8) = 1;
        }

        let n = SPIDEY_SLOT_RETIRE_LOGS.fetch_add(1, Ordering::Relaxed);
        if n < 16 {
            debug_log(&format!(
                "[APU-SLOT-RETIRE] slot#{}=0x{:08X} dsbuf=0x{:08X} cursor={}/{} status=0x{:08X}",
                i,
                slot,
                buf.guest_addr,
                buf.play_cursor,
                buf.buffer_size,
                buf.status_flags()
            ));
        }
    }
}

// ============================================================================
// Global registry
// ============================================================================

struct DsRegistry {
    /// IDirectSound8 object guest address (only one).
    dsound_obj: u32,
    /// Buffers keyed by guest address.
    buffers: HashMap<u32, DsBufferState>,
    /// Next buffer index (for scratch audio region assignment).
    next_buffer_index: u32,
    /// Whether vtable stubs have been written to guest memory.
    vtables_initialized: bool,
    /// Tick counter for periodic cursor advancement.
    global_tick: u64,
}

impl DsRegistry {
    fn new() -> Self {
        Self {
            dsound_obj: 0,
            buffers: HashMap::new(),
            next_buffer_index: 0,
            vtables_initialized: false,
            global_tick: 0,
        }
    }

    fn alloc_scratch_region(&mut self) -> u32 {
        let addr = SCRATCH_AUDIO_BASE + self.next_buffer_index * SCRATCH_AUDIO_STRIDE;
        self.next_buffer_index += 1;
        // Wrap around after 16 regions (1 MB total scratch space)
        if self.next_buffer_index >= 16 {
            self.next_buffer_index = 0;
        }
        addr
    }
}

static REGISTRY: Mutex<Option<DsRegistry>> = Mutex::new(None);
static AUDIO_OUTPUT_ACTIVE: AtomicBool = AtomicBool::new(false);
static SPIDEY_SLOT_RETIRE_LOGS: AtomicU32 = AtomicU32::new(0);
static DSOUND_NOTIFY_LOGS: AtomicU32 = AtomicU32::new(0);

fn with_registry<F, R>(f: F) -> R
where
    F: FnOnce(&mut DsRegistry) -> R,
{
    let mut guard = REGISTRY.lock().unwrap();
    if guard.is_none() {
        *guard = Some(DsRegistry::new());
    }
    f(guard.as_mut().unwrap())
}

pub fn set_audio_output_active(active: bool) {
    AUDIO_OUTPUT_ACTIVE.store(active, Ordering::Relaxed);
}

pub fn mix_audio_frame(guest_mem: *mut u8, frames: usize) -> Vec<i16> {
    if frames == 0 {
        return Vec::new();
    }

    let mut output = vec![0i16; frames * 2];
    let mut saw_playing = false;

    with_registry(|reg| {
        reg.global_tick += 1;
        let tick = reg.global_tick;
        let mut mix_l = vec![0i32; frames];
        let mut mix_r = vec![0i32; frames];

        for buf in reg.buffers.values_mut() {
            if !buf.playing || buf.buffer_size == 0 {
                continue;
            }

            saw_playing = true;
            let old_cursor = buf.play_cursor;
            if is_mixable_pcm(buf) {
                mix_one_buffer(guest_mem, buf, frames, &mut mix_l, &mut mix_r);
            } else {
                // We do not decode Xbox ADPCM/XADPCM yet, but DirectSound's
                // control state still advances on real hardware. Keep status,
                // notifications, and Spider-Man's sound-slot retirement moving
                // even when the host mixer cannot emit audible samples.
                buf.advance_cursor();
            }
            let new_cursor = buf.play_cursor;
            let wrapped = buf.looping && new_cursor < old_cursor;
            let stopped = !buf.playing;
            fire_buffer_notifications(guest_mem, buf, old_cursor, new_cursor, wrapped, stopped);
            sync_buffer_header(guest_mem, buf);
            retire_spiderman_finished_slot_for_buffer(guest_mem, buf);
        }

        if saw_playing {
            for i in 0..frames {
                output[i * 2] = mix_l[i].clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                output[i * 2 + 1] = mix_r[i].clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
        }

        let playing_count = reg.buffers.values().filter(|b| b.playing).count();
        log_spiderman_sound_slots(guest_mem, tick, playing_count);
    });

    if saw_playing {
        output
    } else {
        Vec::new()
    }
}

fn is_mixable_pcm(buf: &DsBufferState) -> bool {
    matches!(buf.bits_per_sample, 8 | 16) && matches!(buf.channels, 1 | 2)
}

fn mix_one_buffer(
    guest_mem: *mut u8,
    buf: &mut DsBufferState,
    out_frames: usize,
    mix_l: &mut [i32],
    mix_r: &mut [i32],
) {
    let bytes_per_sample = match buf.bits_per_sample {
        8 => 1u32,
        16 => 2u32,
        _ => return,
    };
    let channels = match buf.channels {
        1 | 2 => buf.channels as u32,
        _ => return,
    };
    let block_align = channels.saturating_mul(bytes_per_sample);
    if block_align == 0 || buf.buffer_size < block_align {
        return;
    }

    let freq = buf.frequency.max(1);
    let start_cursor = align_cursor(buf.play_cursor.min(buf.buffer_size), block_align);
    let volume = dsound_volume_gain(buf.volume);
    let source_frames_total = buf.buffer_size / block_align;
    if source_frames_total == 0 {
        return;
    }

    let mut last_source_frame = 0u32;
    for out_idx in 0..out_frames {
        let rel_source_frame = ((out_idx as u64).saturating_mul(freq as u64) / 48_000) as u32;
        last_source_frame = rel_source_frame;
        let base_frame = start_cursor / block_align;
        let mut source_frame = base_frame.saturating_add(rel_source_frame);
        if buf.looping {
            source_frame %= source_frames_total;
        } else if source_frame >= source_frames_total {
            buf.playing = false;
            break;
        }

        let byte_offset = source_frame.saturating_mul(block_align);
        let sample_addr = buf.data_addr.wrapping_add(byte_offset);
        let (left, right) = read_pcm_frame(guest_mem, sample_addr, channels, buf.bits_per_sample);
        mix_l[out_idx] = mix_l[out_idx].saturating_add(((left as f32) * volume) as i32);
        mix_r[out_idx] = mix_r[out_idx].saturating_add(((right as f32) * volume) as i32);
    }

    let advance = last_source_frame
        .saturating_add(1)
        .saturating_mul(block_align);
    let next = start_cursor.saturating_add(advance);
    if buf.looping {
        buf.play_cursor = next % buf.buffer_size;
    } else if next >= buf.buffer_size {
        buf.play_cursor = buf.buffer_size;
        buf.playing = false;
    } else {
        buf.play_cursor = next;
    }
}

fn align_cursor(cursor: u32, block_align: u32) -> u32 {
    if block_align == 0 {
        cursor
    } else {
        cursor - (cursor % block_align)
    }
}

fn read_pcm_frame(
    guest_mem: *mut u8,
    addr: u32,
    channels: u32,
    bits_per_sample: u16,
) -> (i16, i16) {
    match bits_per_sample {
        8 => {
            let l = pcm8_to_i16(guest_read_u8(guest_mem, addr));
            let r = if channels >= 2 {
                pcm8_to_i16(guest_read_u8(guest_mem, addr.wrapping_add(1)))
            } else {
                l
            };
            (l, r)
        }
        16 => {
            let l = guest_read_i16(guest_mem, addr);
            let r = if channels >= 2 {
                guest_read_i16(guest_mem, addr.wrapping_add(2))
            } else {
                l
            };
            (l, r)
        }
        _ => (0, 0),
    }
}

fn pcm8_to_i16(v: u8) -> i16 {
    ((v as i16) - 128) << 8
}

fn dsound_volume_gain(volume_hundredths_db: i32) -> f32 {
    if volume_hundredths_db <= -10_000 {
        0.0
    } else if volume_hundredths_db >= 0 {
        1.0
    } else {
        10.0f32.powf(volume_hundredths_db as f32 / 2000.0)
    }
}

// ============================================================================
// Vtable construction
// ============================================================================

/// Write a single RET or RET N stub to guest memory, return its guest address.
fn write_ret_stub(guest_mem: *mut u8, stub_addr: u32, cleanup_bytes: u16) -> u32 {
    unsafe {
        let p = (guest_mem as u64 + stub_addr as u64) as *mut u8;
        if cleanup_bytes == 0 {
            // RET (0xC3)
            *p = 0xC3;
        } else {
            // RET imm16 (0xC2 xx xx)
            *p = 0xC2;
            *(p.add(1) as *mut u16) = cleanup_bytes;
        }
    }
    stub_addr
}

/// Build a vtable: array of function pointers in guest memory, each pointing to a RET N stub.
fn build_vtable(guest_mem: *mut u8, vtable_addr: u32, stubs_addr: u32, cleanups: &[u16]) {
    for (i, &cleanup) in cleanups.iter().enumerate() {
        // Each stub gets 4 bytes (RET N is 3 bytes, RET is 1 byte, padded to 4)
        let stub_off = stubs_addr + (i as u32) * 4;
        let stub_guest = write_ret_stub(guest_mem, stub_off, cleanup);

        // Write vtable entry
        unsafe {
            let vt_entry = (guest_mem as u64 + vtable_addr as u64 + (i as u64) * 4) as *mut u32;
            *vt_entry = stub_guest;
        }
    }
}

/// Initialize all three vtables (IDirectSound8, IDirectSoundBuffer, IDirectSoundStream).
fn ensure_vtables(guest_mem: *mut u8) {
    with_registry(|reg| {
        if reg.vtables_initialized {
            return;
        }

        // IDirectSound8 vtable — stdcall cleanup sizes from Cxbx-R OOVPA analysis.
        //
        // Xbox IDirectSound8 methods (no QueryInterface on Xbox):
        //  [0]  AddRef              -> ret 0  (cdecl-like, 0 args after this)
        //  [1]  Release             -> ret 0
        //  [2]  GetCaps             -> ret 8  (this + pCaps)
        //  [3]  CreateSoundBuffer   -> ret 16 (this + pDesc + ppBuffer + pUnused)
        //  [4]  SetI3DL2Listener    -> ret 12 (this + pAll + dwApply)
        //  [5]  CommitDeferredSettings -> ret 4 (this)
        //  [6]  GetOutputLevels     -> ret 8
        //  [7]  SetAllParameters    -> ret 12
        //  [8]  SetDistanceFactor   -> ret 12
        //  [9]  SetDopplerFactor    -> ret 12
        //  [10] SetRolloffFactor    -> ret 12
        //  [11] EnableHeadphones    -> ret 8
        //  [12] SetMixBinHeadroom   -> ret 12
        //  [13] SetSpeakerConfig    -> ret 8
        //  [14] SetOrientation      -> ret 28 (this + 6 floats + dwApply)
        //  [15] SetPosition         -> ret 20 (this + 3 floats + dwApply)
        //  [16] SetVelocity         -> ret 20 (this + 3 floats + dwApply)
        //  [17] SynchPlayback       -> ret 4
        //  [18] GetTime             -> ret 8
        //  [19] CommitChanges       -> ret 4
        //  [20] CreateSoundStream   -> ret 16
        //  [21] DownloadEffectsImage -> ret 16
        //  [22] SetEffectData       -> ret 16
        //  [23] GetEffectData       -> ret 16
        //  [24] CommitEffectData    -> ret 4
        #[rustfmt::skip]
        let ds_cleanups: &[u16] = &[
            0, 0,       // AddRef, Release
            8, 16,      // GetCaps, CreateSoundBuffer
            12, 4,      // SetI3DL2Listener, CommitDeferredSettings
            8, 12,      // GetOutputLevels, SetAllParameters
            12, 12, 12, // SetDistanceFactor, SetDopplerFactor, SetRolloffFactor
            8, 12, 8,   // EnableHeadphones, SetMixBinHeadroom, SetSpeakerConfig
            28, 20, 20, // SetOrientation, SetPosition, SetVelocity
            4, 8, 4,    // SynchPlayback, GetTime, CommitChanges
            16, 16,     // CreateSoundStream, DownloadEffectsImage
            16, 16, 4,  // SetEffectData, GetEffectData, CommitEffectData
        ];
        build_vtable(guest_mem, DS_VTABLE_ADDR, DS_STUBS_ADDR, ds_cleanups);

        // IDirectSoundBuffer vtable — from Cxbx-R OOVPA `retn` analysis:
        //
        //  [0]  AddRef              -> ret 0
        //  [1]  Release             -> ret 0
        //  [2]  Lock                -> ret 32 (this + offset + bytes + pp1 + pb1 + pp2 + pb2 + flags)
        //  [3]  Unlock              -> ret 4
        //  [4]  SetBufferData       -> ret 12 (this + pData + dwBytes)
        //  [5]  SetCurrentPosition  -> ret 8
        //  [6]  GetCurrentPosition  -> ret 8 (CMcpxBuffer ret 8)
        //  [7]  GetStatus           -> ret 8
        //  [8]  Play                -> ret 16 (this + reserved + priority + flags)
        //  [9]  Stop                -> ret 4
        //  [10] SetVolume           -> ret 8
        //  [11] SetPitch            -> ret 8
        //  [12] SetFrequency        -> ret 8
        //  [13] SetLoopRegion       -> ret 12
        //  [14] SetFormat           -> ret 8
        //  [15] SetHeadroom         -> ret 8
        //  [16] SetI3DL2Source      -> ret 12
        //  [17] SetMaxDistance      -> ret 12
        //  [18] SetMinDistance      -> ret 12
        //  [19] SetConeAngles       -> ret 16
        //  [20] SetConeOrientation  -> ret 20
        //  [21] SetConeOutsideVolume-> ret 12
        //  [22] SetPosition         -> ret 20
        //  [23] SetVelocity         -> ret 20
        //  [24] SetAllParameters    -> ret 12
        //  [25] SetOutputBuffer     -> ret 8
        //  [26] SetMixBins          -> ret 8
        //  [27] SetMixBinVolumes    -> ret 8
        //  [28] SetFilter           -> ret 8
        //  [29] SetEG               -> ret 8
        //  [30] SetLFO              -> ret 8
        //  [31] SetMode             -> ret 12
        #[rustfmt::skip]
        let dsb_cleanups: &[u16] = &[
            0, 0,           // AddRef, Release
            32, 4,          // Lock, Unlock
            12, 8,          // SetBufferData, SetCurrentPosition
            8, 8,           // GetCurrentPosition, GetStatus
            16, 4,          // Play, Stop
            8, 8, 8,        // SetVolume, SetPitch, SetFrequency
            12, 8, 8,       // SetLoopRegion, SetFormat, SetHeadroom
            12, 12, 12,     // SetI3DL2Source, SetMaxDistance, SetMinDistance
            16, 20, 12,     // SetConeAngles, SetConeOrientation, SetConeOutsideVolume
            20, 20, 12,     // SetPosition, SetVelocity, SetAllParameters
            8, 8, 8,        // SetOutputBuffer, SetMixBins, SetMixBinVolumes
            8, 8, 8, 12,    // SetFilter, SetEG, SetLFO, SetMode
        ];
        build_vtable(guest_mem, DSB_VTABLE_ADDR, DSB_STUBS_ADDR, &dsb_cleanups);

        // IDirectSoundStream vtable — similar layout to buffer with streaming additions.
        //
        //  [0]  AddRef          -> ret 0
        //  [1]  Release         -> ret 0
        //  [2]  Process         -> ret 12 (this + pInputDesc + pOutputDesc)
        //  [3]  Discontinuity   -> ret 4
        //  [4]  Flush           -> ret 4
        //  [5]  FlushEx         -> ret 12
        //  [6]  Pause           -> ret 8
        //  [7]  GetStatus       -> ret 8
        //  [8]  GetInfo         -> ret 8
        //  [9]  SetVolume       -> ret 8
        //  [10] SetPitch        -> ret 8
        //  [11] SetFrequency    -> ret 8
        //  [12] SetHeadroom     -> ret 8
        //  [13] SetMixBins      -> ret 8
        //  [14] SetMixBinVolumes-> ret 8
        //  [15] SetFilter       -> ret 8
        //  [16] SetEG           -> ret 8
        //  [17] SetLFO          -> ret 8
        //  [18] SetOutputBuffer -> ret 8
        //  [19] SetI3DL2Source  -> ret 12
        //  [20] SetMaxDistance  -> ret 12
        //  [21] SetMinDistance  -> ret 12
        //  [22] SetRolloffFactor-> ret 12
        //  [23] SetDistanceFactor-> ret 12
        //  [24] SetDopplerFactor -> ret 12
        //  [25] SetConeAngles   -> ret 16
        //  [26] SetConeOrientation -> ret 20
        //  [27] SetConeOutsideVolume -> ret 12
        //  [28] SetPosition     -> ret 20
        //  [29] SetVelocity     -> ret 20
        //  [30] SetAllParameters-> ret 12
        //  [31] SetMode         -> ret 12
        #[rustfmt::skip]
        let dss_cleanups: &[u16] = &[
            0, 0,           // AddRef, Release
            12, 4, 4,       // Process, Discontinuity, Flush
            12, 8, 8, 8,    // FlushEx, Pause, GetStatus, GetInfo
            8, 8, 8, 8,     // SetVolume, SetPitch, SetFrequency, SetHeadroom
            8, 8, 8, 8, 8, 8, // SetMixBins..SetOutputBuffer
            12, 12, 12,     // SetI3DL2Source, SetMaxDistance, SetMinDistance
            12, 12, 12,     // SetRolloffFactor, SetDistanceFactor, SetDopplerFactor
            16, 20, 12,     // SetConeAngles, SetConeOrientation, SetConeOutsideVolume
            20, 20, 12, 12, // SetPosition, SetVelocity, SetAllParameters, SetMode
        ];
        build_vtable(guest_mem, DSS_VTABLE_ADDR, DSS_STUBS_ADDR, &dss_cleanups);

        reg.vtables_initialized = true;
        debug_log("[APU-HLE] Vtables initialized: IDirectSound8 / IDirectSoundBuffer / IDirectSoundStream");
    });
}

/// Write a COM object header at a guest address.
fn write_object_header(guest_mem: *mut u8, obj_addr: u32, vtable_addr: u32, tag: u32) {
    unsafe {
        let base = guest_mem as u64 + obj_addr as u64;
        // +0x00: vtable pointer
        *(base as *mut u32) = vtable_addr;
        // +0x04: refcount = 1
        *((base + 4) as *mut u32) = 1;
        // +0x08: type tag
        *((base + 8) as *mut u32) = tag;
    }
}

/// Write a u32 to a guest address (helper for output pointers).
fn guest_write_u32(guest_mem: *mut u8, guest_addr: u32, value: u32) {
    if guest_addr != 0 {
        unsafe {
            *((guest_mem as u64 + guest_addr as u64) as *mut u32) = value;
        }
    }
}

/// Read a u32 from a guest address.
fn guest_read_u32(guest_mem: *mut u8, guest_addr: u32) -> u32 {
    if guest_addr == 0 {
        return 0;
    }
    unsafe { *((guest_mem as u64 + guest_addr as u64) as *const u32) }
}

/// Read a u8 from a guest address.
fn guest_read_u8(guest_mem: *mut u8, guest_addr: u32) -> u8 {
    if guest_addr == 0 {
        return 0;
    }
    unsafe { *((guest_mem as u64 + guest_addr as u64) as *const u8) }
}

/// Read a u16 from a guest address.
fn guest_read_u16(guest_mem: *mut u8, guest_addr: u32) -> u16 {
    if guest_addr == 0 {
        return 0;
    }
    unsafe { *((guest_mem as u64 + guest_addr as u64) as *const u16) }
}

/// Read an i16 PCM sample from a guest address.
fn guest_read_i16(guest_mem: *mut u8, guest_addr: u32) -> i16 {
    if guest_addr == 0 {
        return 0;
    }
    unsafe { *((guest_mem as u64 + guest_addr as u64) as *const i16) }
}

// ============================================================================
// HLE entry points — called from mod.rs dispatcher
// ============================================================================

/// DirectSoundCreate HLE.
///
/// Xbox signature: `HRESULT DirectSoundCreate(LPVOID flags, LPDIRECTSOUND8* ppDS, LPUNKNOWN pUnused)`
/// In practice args vary by XDK. Our OOVPA hooks pass:
///   args[0..3] = first 4 stack args
///   args[3] = ppDirectSound (output pointer) for most XDK versions.
///
/// We look at all args and use the one that looks like a valid output pointer.
pub fn hle_dsound_create(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    ensure_vtables(guest_mem);

    // Allocate the IDirectSound8 object at a fixed guest address
    let obj_addr: u32 = 0x00EA_1000;
    write_object_header(guest_mem, obj_addr, DS_VTABLE_ADDR, TAG_DSOUND);

    with_registry(|reg| {
        reg.dsound_obj = obj_addr;
    });

    // Find the output pointer — ppDirectSound. Standard Xbox API has it at args[1],
    // but some XDK wrappers put it at args[2] or args[3]. Scan for a valid guest address.
    let output_ptr = [args[1], args[2], args[3], args[0]]
        .iter()
        .copied()
        .find(|&a| a >= 0x0001_0000 && a < 0x2000_0000)
        .unwrap_or(args[1]);

    if output_ptr != 0 {
        guest_write_u32(guest_mem, output_ptr, obj_addr);
    }

    debug_log(&format!(
        "[APU-HLE] DirectSoundCreate: obj=0x{:08X} -> *0x{:08X} args=[0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X}]",
        obj_addr, output_ptr, args[0], args[1], args[2], args[3]
    ));
    0 // DS_OK
}

/// CDirectSound::CreateSoundBuffer / DirectSoundCreateBuffer
///
/// Creates a fake IDirectSoundBuffer8 object with tracked state.
///
/// CDirectSound_CreateSoundBuffer args (stdcall, ret 0x10):
///   args[0] = this (IDirectSound8*)
///   args[1] = pdsbd (DSBUFFERDESC*)
///   args[2] = ppBuffer (IDirectSoundBuffer** output)
///   args[3] = pUnused
///
/// DirectSoundCreateBuffer args (stdcall, ret 0x08):
///   args[0] = pdsbd (DSBUFFERDESC*)
///   args[1] = ppBuffer (output)
pub fn hle_create_sound_buffer(args: &[u32; 8], guest_mem: *mut u8, is_direct_api: bool) -> u32 {
    ensure_vtables(guest_mem);

    let (pp_buffer, p_desc) = if is_direct_api {
        // DirectSoundCreateBuffer: args[0]=pdsbd, args[1]=ppBuffer
        (args[1], args[0])
    } else {
        // CDirectSound_CreateSoundBuffer: args[0]=this, args[1]=pdsbd, args[2]=ppBuffer
        (args[2], args[1])
    };

    // Parse DSBUFFERDESC if available. Xbox DSBUFFERDESC layout:
    //   +0x00: dwSize (DWORD)
    //   +0x04: dwFlags (DWORD)
    //   +0x08: dwBufferBytes (DWORD)
    //   +0x0C: lpwfxFormat (WAVEFORMATEX*)
    let (buffer_bytes, frequency, channels, bits) = if p_desc != 0 {
        let buf_bytes = guest_read_u32(guest_mem, p_desc + 0x08);
        let fmt_ptr = guest_read_u32(guest_mem, p_desc + 0x0C);
        if fmt_ptr != 0 {
            // WAVEFORMATEX: +0x04 = nSamplesPerSec, +0x02 = nChannels, +0x0E = wBitsPerSample
            let freq = guest_read_u32(guest_mem, fmt_ptr + 0x04);
            let ch = guest_read_u16(guest_mem, fmt_ptr + 0x02);
            let bps = guest_read_u16(guest_mem, fmt_ptr + 0x0E);
            (buf_bytes, freq, ch, bps)
        } else {
            (buf_bytes, 48000, 2, 16)
        }
    } else {
        (0, 48000, 2, 16)
    };

    let buffer_size = if buffer_bytes == 0 || buffer_bytes > MAX_BUFFER_SIZE {
        DEFAULT_BUFFER_SIZE
    } else {
        buffer_bytes
    };

    let (obj_addr, data_addr, buf_count) = with_registry(|reg| {
        let data_addr = reg.alloc_scratch_region();
        // Object address: use a range starting at 0x00EA4000 so we don't collide with vtables
        let obj_addr = 0x00EA_4000 + (reg.buffers.len() as u32) * 0x40;

        let mut state = DsBufferState::new(obj_addr, data_addr, buffer_size, false);
        state.frequency = frequency;
        state.channels = channels;
        state.bits_per_sample = bits;
        reg.buffers.insert(obj_addr, state);

        (obj_addr, data_addr, reg.buffers.len())
    });

    // Write object header to guest memory
    write_object_header(guest_mem, obj_addr, DSB_VTABLE_ADDR, TAG_BUFFER);
    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&obj_addr) {
            sync_buffer_header(guest_mem, buf);
        }
    });

    // Write output
    guest_write_u32(guest_mem, pp_buffer, obj_addr);

    debug_log(&format!(
        "[APU-HLE] CreateSoundBuffer #{}: obj=0x{:08X} data=0x{:08X} size={} freq={} ch={} bits={}",
        buf_count, obj_addr, data_addr, buffer_size, frequency, channels, bits
    ));
    0 // DS_OK
}

/// CDirectSound::CreateSoundStream / DirectSoundCreateStream
///
/// CDirectSound_CreateSoundStream args (stdcall, ret 0x10):
///   args[0] = this, args[1] = pdssd, args[2] = ppStream, args[3] = pUnused
///
/// DirectSoundCreateStream args (stdcall, ret 0x08):
///   args[0] = pdssd, args[1] = ppStream
pub fn hle_create_sound_stream(args: &[u32; 8], guest_mem: *mut u8, is_direct_api: bool) -> u32 {
    ensure_vtables(guest_mem);

    let (pp_stream, p_desc) = if is_direct_api {
        (args[1], args[0])
    } else {
        (args[2], args[1])
    };

    // Parse DSSTREAMDESC if available (similar layout to DSBUFFERDESC for our purposes).
    let (frequency, channels, bits) = if p_desc != 0 {
        let fmt_ptr = guest_read_u32(guest_mem, p_desc + 0x0C);
        if fmt_ptr != 0 {
            let freq = guest_read_u32(guest_mem, fmt_ptr + 0x04);
            let ch = guest_read_u16(guest_mem, fmt_ptr + 0x02);
            let bps = guest_read_u16(guest_mem, fmt_ptr + 0x0E);
            (freq, ch, bps)
        } else {
            (48000, 2, 16)
        }
    } else {
        (48000, 2, 16)
    };

    let (obj_addr, stream_count) = with_registry(|reg| {
        let data_addr = reg.alloc_scratch_region();
        let obj_addr = 0x00EA_5000 + (reg.buffers.len() as u32) * 0x40;

        let mut state = DsBufferState::new(obj_addr, data_addr, DEFAULT_BUFFER_SIZE, true);
        state.frequency = frequency;
        state.channels = channels;
        state.bits_per_sample = bits;
        reg.buffers.insert(obj_addr, state);

        let count = reg.buffers.values().filter(|b| b.is_stream).count();
        (obj_addr, count)
    });

    write_object_header(guest_mem, obj_addr, DSS_VTABLE_ADDR, TAG_STREAM);
    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&obj_addr) {
            sync_buffer_header(guest_mem, buf);
        }
    });

    guest_write_u32(guest_mem, pp_stream, obj_addr);

    debug_log(&format!(
        "[APU-HLE] CreateSoundStream #{}: obj=0x{:08X} freq={} ch={} bits={}",
        stream_count, obj_addr, frequency, channels, bits
    ));
    0 // DS_OK
}

/// IDirectSoundBuffer::Play
///
/// args[0] = this, args[1] = reserved, args[2] = priority, args[3] = flags
pub fn hle_play(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let flags = args[3];
    let looping = (flags & DSBPLAY_LOOPING) != 0;

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            if !looping {
                buf.play_cursor = 0;
            }
            buf.playing = true;
            buf.looping = looping;
            for notify in &mut buf.notifications {
                notify.fired = false;
            }
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer::Stop
///
/// args[0] = this
pub fn hle_stop(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            buf.playing = false;
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer::GetStatus
///
/// args[0] = this, args[1] = pdwStatus (output)
pub fn hle_get_status(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let status_ptr = args[1];

    let flags = with_registry(|reg| {
        reg.buffers
            .get(&this)
            .map(|b| b.status_flags())
            .unwrap_or(0)
    });

    guest_write_u32(guest_mem, status_ptr, flags);
    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&this) {
            sync_buffer_header(guest_mem, buf);
        }
    });
    0 // DS_OK
}

/// IDirectSoundBuffer::GetCurrentPosition
///
/// args[0] = this, args[1] = pdwPlayCursor, args[2] = pdwWriteCursor
pub fn hle_get_current_position(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let play_ptr = args[1];
    let write_ptr = args[2];

    let (play, write) = with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&this) {
            (buf.play_cursor, buf.write_cursor())
        } else {
            (0, 0)
        }
    });

    guest_write_u32(guest_mem, play_ptr, play);
    guest_write_u32(guest_mem, write_ptr, write);
    0 // DS_OK
}

/// IDirectSoundBuffer::SetCurrentPosition
///
/// args[0] = this, args[1] = dwPosition
pub fn hle_set_current_position(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let pos = args[1];

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            buf.play_cursor = if buf.buffer_size > 0 {
                pos % buf.buffer_size
            } else {
                0
            };
            for notify in &mut buf.notifications {
                notify.fired = false;
            }
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer::Lock
///
/// args[0] = this
/// args[1] = dwOffset
/// args[2] = dwBytes
/// args[3] = ppvAudioPtr1 (output)
/// args[4] = pdwAudioBytes1 (output)
/// args[5] = ppvAudioPtr2 (output)
/// args[6] = pdwAudioBytes2 (output)
/// args[7] = dwFlags
pub fn hle_buffer_lock(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let dw_offset = args[1];
    let dw_bytes = args[2];
    let ppv_audio1 = args[3];
    let pdw_bytes1 = args[4];
    let ppv_audio2 = args[5];
    let pdw_bytes2 = args[6];

    let (data_addr, buffer_size) = with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&this) {
            (buf.data_addr, buf.buffer_size)
        } else {
            // Unknown buffer — use a safe scratch region
            (SCRATCH_AUDIO_BASE, DEFAULT_BUFFER_SIZE)
        }
    });

    let lock_bytes = if dw_bytes == 0 {
        buffer_size
    } else {
        dw_bytes.min(buffer_size)
    };
    let offset = if buffer_size > 0 {
        dw_offset % buffer_size
    } else {
        0
    };

    // Region 1: from offset to min(offset + lock_bytes, buffer_size)
    let region1_start = data_addr + offset;
    let region1_bytes = if buffer_size > 0 {
        lock_bytes.min(buffer_size - offset)
    } else {
        lock_bytes
    };

    guest_write_u32(guest_mem, ppv_audio1, region1_start);
    guest_write_u32(guest_mem, pdw_bytes1, region1_bytes);

    // Region 2: wrap-around
    let remaining = lock_bytes.saturating_sub(region1_bytes);
    if remaining > 0 && ppv_audio2 != 0 {
        guest_write_u32(guest_mem, ppv_audio2, data_addr);
        guest_write_u32(guest_mem, pdw_bytes2, remaining);
    } else {
        guest_write_u32(guest_mem, ppv_audio2, 0);
        guest_write_u32(guest_mem, pdw_bytes2, 0);
    }

    0 // DS_OK
}

/// IDirectSoundBuffer::SetVolume
///
/// args[0] = this, args[1] = lVolume
pub fn hle_set_volume(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let volume = args[1] as i32;

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            buf.volume = volume;
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer::SetFrequency
///
/// args[0] = this, args[1] = dwFrequency
pub fn hle_set_frequency(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let freq = args[1];

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            if freq > 0 {
                buf.frequency = freq;
            }
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer::SetNotificationPositions
///
/// args[0] = this, args[1] = dwNotifyCount, args[2] = paNotifies
pub fn hle_set_notification_positions(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let notify_count = args[1].min(64);
    let notifies = args[2];

    let mut stored = 0;
    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            buf.notifications.clear();

            if notifies != 0 {
                for i in 0..notify_count {
                    let entry = notifies + i * 8;
                    let offset = guest_read_u32(guest_mem, entry);
                    let event = guest_read_u32(guest_mem, entry + 0x04);
                    buf.notifications.push(DsNotifyPosition {
                        offset,
                        event,
                        fired: false,
                    });
                    stored += 1;
                }
            }
        }
    });

    debug_log(&format!(
        "[APU-NOTIFY] SetNotificationPositions buf=0x{:08X} count={} ptr=0x{:08X} stored={}",
        this, notify_count, notifies, stored
    ));

    0 // DS_OK
}

/// IDirectSoundBuffer::SetBufferData
///
/// args[0] = this, args[1] = pData, args[2] = dwBytes
pub fn hle_set_buffer_data(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let data_ptr = args[1];
    let data_bytes = args[2];

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            if data_ptr != 0 {
                buf.data_addr = data_ptr;
            }
            if data_bytes > 0 && data_bytes <= MAX_BUFFER_SIZE {
                buf.buffer_size = data_bytes;
            }
            // Reset cursor if buffer data changes
            buf.play_cursor = 0;
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// DirectSoundDoWork / CDirectSound::DoWork
///
/// Advances play cursors for all playing buffers. Called once per game frame
/// (typically 60 Hz).
pub fn hle_do_work(guest_mem: *mut u8) -> u32 {
    with_registry(|reg| {
        reg.global_tick += 1;
        let tick = reg.global_tick;
        for buf in reg.buffers.values_mut() {
            let old_cursor = buf.play_cursor;
            let was_playing = buf.playing;
            buf.advance_cursor();
            let new_cursor = buf.play_cursor;
            let wrapped = was_playing && buf.looping && new_cursor < old_cursor;
            let stopped = was_playing && !buf.playing;
            fire_buffer_notifications(guest_mem, buf, old_cursor, new_cursor, wrapped, stopped);
            sync_buffer_header(guest_mem, buf);
            retire_spiderman_finished_slot_for_buffer(guest_mem, buf);
        }
        let playing_count = reg.buffers.values().filter(|b| b.playing).count();
        log_spiderman_sound_slots(guest_mem, tick, playing_count);
    });
    0 // DS_OK
}

/// IDirectSoundStream::Process
///
/// args[0] = this, args[1] = pInputDesc, args[2] = pOutputDesc
///
/// For streams, the game submits audio packets. We accept them silently.
/// The key requirement is that the completion callback (if any) fires,
/// but since we don't have async infrastructure, we mark the packet as
/// completed immediately by setting status to XMEDIAPACKET_STATUS_SUCCESS (0).
pub fn hle_stream_process(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let _this = args[0];
    let p_input = args[1];

    // XMEDIAPACKET layout:
    //   +0x00: pvBuffer (data pointer)
    //   +0x04: dwMaxSize
    //   +0x08: pdwCompletedSize (output)
    //   +0x0C: hCompletionEvent
    //   +0x10: pdwStatus (output, set to 0 = success)
    if p_input != 0 {
        let max_size = guest_read_u32(guest_mem, p_input + 0x04);
        let completed_size_ptr = guest_read_u32(guest_mem, p_input + 0x08);
        let completion_event = guest_read_u32(guest_mem, p_input + 0x0C);
        let status_ptr = guest_read_u32(guest_mem, p_input + 0x10);

        // Report all bytes consumed
        guest_write_u32(guest_mem, completed_size_ptr, max_size);
        // Status = success (0)
        guest_write_u32(guest_mem, status_ptr, 0);
        if completion_event != 0 {
            let apu_prof = crate::xbox::profiler::start(crate::xbox::profiler::CpuPhase::ApuNotify);
            let signaled = signal_guest_event_like(guest_mem, completion_event);
            debug_log(&format!(
                "[APU-NOTIFY] StreamProcess completion event=0x{:08X} signaled={}",
                completion_event, signaled
            ));
            crate::xbox::profiler::finish(crate::xbox::profiler::CpuPhase::ApuNotify, apu_prof);
        }
    }

    0 // DS_OK
}

/// IDirectSoundStream::GetStatus
///
/// args[0] = this, args[1] = pdwStatus
pub fn hle_stream_get_status(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let status_ptr = args[1];

    let flags = with_registry(|reg| {
        reg.buffers
            .get(&this)
            .map(|b| b.status_flags())
            .unwrap_or(0)
    });

    guest_write_u32(guest_mem, status_ptr, flags);
    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get(&this) {
            sync_buffer_header(guest_mem, buf);
        }
    });
    0 // DS_OK
}

/// IDirectSoundStream::Pause
///
/// args[0] = this, args[1] = dwPause (DSSPAUSE_PAUSE=1, DSSPAUSE_RESUME=2)
pub fn hle_stream_pause(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let this = args[0];
    let pause_flag = args[1];

    with_registry(|reg| {
        if let Some(buf) = reg.buffers.get_mut(&this) {
            if pause_flag == 1 {
                // Pause
                buf.playing = false;
            } else if pause_flag == 2 {
                // Resume
                buf.playing = true;
            }
            sync_buffer_header(guest_mem, buf);
        }
    });

    0 // DS_OK
}

/// IDirectSoundBuffer/Stream::Release
///
/// args[0] = this
pub fn hle_release(args: &[u32; 8], _guest_mem: *mut u8) -> u32 {
    let this = args[0];

    with_registry(|reg| {
        // Remove from registry (refcount not tracked in detail)
        reg.buffers.remove(&this);
    });

    0 // refcount 0
}

/// IDirectSoundBuffer/Stream::AddRef
///
/// args[0] = this
pub fn hle_addref(_args: &[u32; 8], _guest_mem: *mut u8) -> u32 {
    1 // refcount (fake)
}

// ============================================================================
// Legacy compatibility — keep setup_fake_buffer_at / setup_fake_stream_at
// for callers in mod.rs that still use the old API shape.
// ============================================================================

/// Set up a fake IDirectSoundBuffer at a given guest address.
/// Legacy wrapper used by mod.rs `hle_create_sound_buffer`.
pub fn setup_fake_buffer_at(guest_mem: *mut u8, handle_addr: u32) {
    ensure_vtables(guest_mem);
    write_object_header(guest_mem, handle_addr, DSB_VTABLE_ADDR, TAG_BUFFER);

    // Register in buffer state if not already present
    with_registry(|reg| {
        if !reg.buffers.contains_key(&handle_addr) {
            let data_addr = reg.alloc_scratch_region();
            reg.buffers.insert(
                handle_addr,
                DsBufferState::new(handle_addr, data_addr, DEFAULT_BUFFER_SIZE, false),
            );
        }
    });
}

/// Set up a fake IDirectSoundStream at a given guest address.
/// Legacy wrapper used by mod.rs `hle_create_sound_stream`.
pub fn setup_fake_stream_at(guest_mem: *mut u8, handle_addr: u32) {
    ensure_vtables(guest_mem);
    write_object_header(guest_mem, handle_addr, DSS_VTABLE_ADDR, TAG_STREAM);

    with_registry(|reg| {
        if !reg.buffers.contains_key(&handle_addr) {
            let data_addr = reg.alloc_scratch_region();
            reg.buffers.insert(
                handle_addr,
                DsBufferState::new(handle_addr, data_addr, DEFAULT_BUFFER_SIZE, true),
            );
        }
    });
}

/// Functions to skip (NOP/RET) in the DSOUND section.
/// These crash when they try to program APU hardware that we don't emulate.
pub const DSOUND_SKIP_FUNCS: &[(u32, &str)] = &[(0x0033_F17C, "DirectSoundCreate_real")];

// ============================================================================
// Diagnostics
// ============================================================================

/// Return a summary of the DirectSound HLE state for debug display.
pub fn dsound_status_summary() -> String {
    with_registry(|reg| {
        let total = reg.buffers.len();
        let playing = reg.buffers.values().filter(|b| b.playing).count();
        let streams = reg.buffers.values().filter(|b| b.is_stream).count();
        let buffers = total - streams;
        format!(
            "DSOUND: {} bufs ({} playing), {} streams, tick={}",
            buffers, playing, streams, reg.global_tick
        )
    })
}
