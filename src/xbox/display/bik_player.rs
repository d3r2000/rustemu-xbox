/// Bink video player — pre-extracted raw BGRA frames + PCM audio from activisn.bik.
///
/// Video: ffmpeg -i activisn.bik -pix_fmt bgra -f rawvideo activisn_frames.raw
///   313 frames at 30fps, each 640x480x4 = 1,228,800 bytes. Total ~384MB.
///
/// Audio: ffmpeg -i activisn.bik -vn -f s16le -ar 48000 -ac 2 activisn_audio.raw
///   Stereo 48kHz signed 16-bit PCM. Total ~2MB.
///
/// RetroArch runs at 60Hz. We advance video every other frame (→30fps).
/// Audio is fed via retro_audio_sample_batch at 48000/60 = 800 samples/frame.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

const FRAME_WIDTH: usize = 640;
const FRAME_HEIGHT: usize = 480;
const FRAME_BYTES: usize = FRAME_WIDTH * FRAME_HEIGHT * 4;
const TOTAL_FRAMES: u32 = 313;

/// Audio: 48kHz stereo s16le, 4 bytes per sample-pair (L+R).
const AUDIO_SAMPLE_RATE: usize = 48000;
const AUDIO_CHANNELS: usize = 2;
/// Samples per retro_run call: 48000 / 60 = 800 stereo frames.
const AUDIO_FRAMES_PER_TICK: usize = AUDIO_SAMPLE_RATE / 60;

static VIDEO_DATA: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static AUDIO_DATA: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static CURRENT_FRAME: AtomicU32 = AtomicU32::new(0);
static AUDIO_CURSOR: AtomicU32 = AtomicU32::new(0);
static PLAYING: AtomicBool = AtomicBool::new(false);
static FINISHED: AtomicBool = AtomicBool::new(false);
static FRAME_SKIP: AtomicU32 = AtomicU32::new(0);

/// Load the raw frame blob from disk. Call once at init.
pub fn load_video(path: &str) -> bool {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            crate::xbox::emulator::debug_log(&format!("[BIK] Failed to read {}: {}", path, e));
            return false;
        }
    };

    let expected = FRAME_BYTES * TOTAL_FRAMES as usize;
    if data.len() < expected {
        crate::xbox::emulator::debug_log(&format!(
            "[BIK] File too small: {} < {} (need {} frames)",
            data.len(), expected, TOTAL_FRAMES
        ));
        return false;
    }

    crate::xbox::emulator::debug_log(&format!(
        "[BIK] Loaded {} bytes ({} frames, {}x{})",
        data.len(), TOTAL_FRAMES, FRAME_WIDTH, FRAME_HEIGHT
    ));

    *VIDEO_DATA.lock().unwrap_or_else(|e| e.into_inner()) = Some(data);
    CURRENT_FRAME.store(0, Ordering::Relaxed);
    FINISHED.store(false, Ordering::Relaxed);
    true
}

/// Load audio track (raw PCM s16le stereo 48kHz).
pub fn load_audio(path: &str) -> bool {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            crate::xbox::emulator::debug_log(&format!("[BIK] Failed to read audio {}: {}", path, e));
            return false;
        }
    };
    let total_samples = data.len() / (AUDIO_CHANNELS * 2); // 2 bytes per sample
    crate::xbox::emulator::debug_log(&format!(
        "[BIK] Audio loaded: {} bytes ({} stereo samples, {:.1}s)",
        data.len(), total_samples, total_samples as f64 / AUDIO_SAMPLE_RATE as f64
    ));
    *AUDIO_DATA.lock().unwrap_or_else(|e| e.into_inner()) = Some(data);
    AUDIO_CURSOR.store(0, Ordering::Relaxed);
    true
}

/// Start playback.
pub fn play() {
    CURRENT_FRAME.store(0, Ordering::Relaxed);
    AUDIO_CURSOR.store(0, Ordering::Relaxed);
    FRAME_SKIP.store(0, Ordering::Relaxed);
    FINISHED.store(false, Ordering::Relaxed);
    PLAYING.store(true, Ordering::Relaxed);
    crate::xbox::emulator::debug_log("[BIK] Playback started");
}

pub fn is_finished() -> bool {
    FINISHED.load(Ordering::Relaxed)
}

pub fn is_playing() -> bool {
    PLAYING.load(Ordering::Relaxed)
}

/// Advance one video frame and store into GPU readback buffer.
/// Call once per retro_run(). Returns true if a frame was displayed.
pub fn tick() -> bool {
    if !PLAYING.load(Ordering::Relaxed) {
        return false;
    }

    // Skip every other frame to match ~30fps source at 60Hz retro_run
    let skip = FRAME_SKIP.fetch_add(1, Ordering::Relaxed);
    if skip % 2 != 0 {
        return true; // Keep showing previous frame
    }

    let frame_idx = CURRENT_FRAME.load(Ordering::Relaxed);
    if frame_idx >= TOTAL_FRAMES {
        PLAYING.store(false, Ordering::Relaxed);
        FINISHED.store(true, Ordering::Relaxed);
        crate::xbox::emulator::debug_log("[BIK] Playback finished");
        return false;
    }

    let guard = VIDEO_DATA.lock().unwrap_or_else(|e| e.into_inner());
    let data = match guard.as_ref() {
        Some(d) => d,
        None => return false,
    };

    let offset = frame_idx as usize * FRAME_BYTES;
    if offset + FRAME_BYTES > data.len() {
        return false;
    }

    let frame_data = &data[offset..offset + FRAME_BYTES];

    // Convert BGRA → XRGB8888 (0xFFRRGGBB)
    let mut pixels = vec![0u32; FRAME_WIDTH * FRAME_HEIGHT];
    for i in 0..pixels.len() {
        let b = frame_data[i * 4] as u32;
        let g = frame_data[i * 4 + 1] as u32;
        let r = frame_data[i * 4 + 2] as u32;
        pixels[i] = 0xFF000000 | (r << 16) | (g << 8) | b;
    }

    crate::xbox::gpu::store_readback(&pixels);

    if frame_idx % 30 == 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[BIK] Frame {}/{}", frame_idx + 1, TOTAL_FRAMES
        ));
    }

    CURRENT_FRAME.store(frame_idx + 1, Ordering::Relaxed);
    true
}

/// Get audio samples for this frame. Returns a slice of interleaved s16le stereo
/// samples (800 frames = 1600 i16 values per retro_run call at 60Hz).
/// Returns None if not playing or no audio loaded.
pub fn get_audio_samples() -> Option<Vec<i16>> {
    if !PLAYING.load(Ordering::Relaxed) {
        return None;
    }

    let guard = AUDIO_DATA.lock().unwrap_or_else(|e| e.into_inner());
    let data = match guard.as_ref() {
        Some(d) => d,
        None => return None,
    };

    let cursor = AUDIO_CURSOR.load(Ordering::Relaxed) as usize;
    let bytes_per_tick = AUDIO_FRAMES_PER_TICK * AUDIO_CHANNELS * 2; // 800 * 2 * 2 = 3200 bytes
    let end = (cursor + bytes_per_tick).min(data.len());

    if cursor >= data.len() {
        return None;
    }

    let chunk = &data[cursor..end];
    // Convert bytes to i16 samples
    let samples: Vec<i16> = chunk
        .chunks_exact(2)
        .map(|pair| i16::from_le_bytes([pair[0], pair[1]]))
        .collect();

    AUDIO_CURSOR.store(end as u32, Ordering::Relaxed);
    Some(samples)
}
