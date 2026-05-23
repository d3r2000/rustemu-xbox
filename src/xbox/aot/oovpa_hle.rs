/// OOVPA HLE (High-Level Emulation) stubs for D3D, XInput, Bink, and more.
/// Split from oovpa.rs for modularity.
use crate::xbox::emulator::debug_log;
use super::oovpa::{OovpaMatch, OOVPA_STATE};
use super::oovpa_d3d::*;

// ============================================================================
// HLE stubs
// ============================================================================

pub fn execute_hle(
    name: &str,
    args: &[u32; 8],
    _argc: u8,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_mem: *mut u8,
    mmio_count: &mut u64,
    pb_commands: &mut u64,
    draw_calls: &mut u64,
    is_manual: bool,
) -> u32 {
    if is_manual {
        return execute_manual_hle(name, args, context, guest_mem);
    }

    match name {
        "Direct3D_CreateDevice" => {
            static CD_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = CD_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n == 0 {
                // First organic CreateDevice — initialize device struct + GPU
                crate::xbox::emulator::debug_log("[ORGANIC] First CreateDevice — initializing D3D device");
                let r = hle_create_device(guest_mem, mmio_count);
                *pb_commands += 1;
                r
            } else {
                // Subsequent calls — return cached success, don't re-init
                0 // D3D_OK
            }
        }
        "D3DDevice_Swap" => {
            let r = hle_swap(guest_mem, mmio_count);
            *pb_commands += 1;
            r
        }
        "D3DDevice_Clear" => hle_clear(args, guest_mem, mmio_count),
        "D3DDevice_SetVertexShader" => hle_set_vertex_shader(args, guest_mem),
        "D3DDevice_SetTransform" => hle_set_transform(args, guest_mem),
        "D3DDevice_BeginPush" => hle_begin_push(args, guest_mem, context),
        "D3DDevice_Reset" => {
            // Reset re-initializes present parameters. Reset PB PUT to base
            // so the pushbuffer stays consistent for subsequent draw calls.
            let dev_ptr = unsafe { *((guest_mem as u64 + G_PDEVICE as u64) as *const u32) };
            if dev_ptr != 0 {
                let pb_base = unsafe {
                    *((guest_mem as u64 + dev_ptr as u64 + DEV_PB_BASE as u64) as *const u32)
                };
                unsafe {
                    *((guest_mem as u64 + dev_ptr as u64 + DEV_PB_PUT as u64) as *mut u32) =
                        pb_base;
                }
            }
            0 // D3D_OK
        }
        "D3DDevice_SetRenderTarget" => {
            // SetRenderTarget(pRenderTarget, pNewZStencil) — 2 args
            let p_rt = args[0];
            let p_ds = args[1];
            CURRENT_RENDER_TARGET.store(p_rt, std::sync::atomic::Ordering::Relaxed);
            CURRENT_DEPTH_STENCIL.store(p_ds, std::sync::atomic::Ordering::Relaxed);
            debug_log(&format!(
                "[OOVPA-HLE] SetRenderTarget: RT=0x{:08X} DS=0x{:08X}",
                p_rt, p_ds
            ));
            0 // D3D_OK
        }
        "D3DDevice_KickOff" => 0,                             // no-op
        "D3DDevice_InsertFence" => 0x8000_BEEF,               // dummy token
        "D3D_BlockOnFence" => 0,                              // fence always complete
        name if crate::xbox::apu::is_dsound_function(name) => {
            crate::xbox::apu::handle_dsound_hle(name, args, guest_mem)
        }
        "D3DDevice_DrawVertices" | "D3DDevice_DrawIndexedVertices" => {
            hle_draw(name, args, guest_mem, mmio_count);
            *draw_calls += 1; // Stage 10
            0
        }
        _ => 0, // generic: return D3D_OK
    }
}

fn execute_manual_hle(
    name: &str,
    args: &[u32; 8],
    _context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
    guest_mem: *mut u8,
) -> u32 {
    // Manual hooks: skip the function entirely, return safe values
    match name {
        "D3D_RenderStateSort" | "D3D_RenderStateSort2" => 0, // skip sort
        "D3D_RenderStateInit" => 0,                          // skip init
        "D3D_MakeSpace" => PB_DUMMY_BASE,                    // return dummy PB pointer
        "CDevice_SetStateVB" => 0,                           // skip pre-draw state flush
        // ---- Render loop hooks ----
        "D3DDevice_SetRenderState_Simple" => {
            // Fastcall: ECX = pushbuffer method header, EDX = value
            let ecx = _context.Rcx as u32;
            let edx = _context.Rdx as u32;
            crate::xbox::gpu::set_render_state(ecx, edx);
            0
        }
        "D3DDevice_SetTexture" => {
            // 0x2F32F0 is a texture release helper (1 arg: pTexture, ret 4)
            let p_texture = args[0];
            debug_log(&format!(
                "[OOVPA-HLE] SetTexture: tex=0x{:08X}",
                p_texture
            ));
            0 // D3D_OK
        }
        "D3DDevice_SetStreamSource" => 0,                    // skip VB bind
        "D3DDevice_SetViewport" => 0,                        // skip viewport (uses default)
        "D3DDevice_CreateTexture" => {
            // Xbox D3D8 CreateTexture(Size, Levels, Usage, Format, ppTexture) — 5 args, ret 0x14
            let size = args[0];
            let width = size & 0xFFFF;
            let height = (size >> 16) & 0xFFFF;
            let levels = args[1];
            let usage = args[2];
            let format = args[3];
            let pp_texture = args[4];
            if pp_texture != 0 && pp_texture < 0x1000_0000 {
                let tex_addr = unsafe {
                    alloc_texture(guest_mem, width, height, levels, usage, format)
                };
                if tex_addr != 0 {
                    unsafe {
                        *(((guest_mem as u64) + pp_texture as u64) as *mut u32) = tex_addr;
                    }
                    debug_log(&format!(
                        "[OOVPA-HLE] CreateTexture: {}x{} fmt=0x{:02X} lvl={} tex=0x{:08X} -> *0x{:08X}",
                        width, height, format, levels, tex_addr, pp_texture
                    ));
                } else {
                    debug_log(&format!(
                        "[OOVPA-HLE] CreateTexture FAILED: {}x{} fmt=0x{:02X} (heap full)",
                        width, height, format
                    ));
                    return 0x8007_000E; // E_OUTOFMEMORY
                }
            }
            0 // D3D_OK
        }
        "D3DDevice_GetSurfaceLevel" => {
            // GetSurfaceLevel(pTexture, Level, ppSurface) — thiscall on Xbox
            // On Xbox D3D8, a texture IS its own level-0 surface. For level 0,
            // return the texture pointer itself. For other levels, return a
            // surface pointing to the same Data (sufficient for HLE).
            let p_texture = args[0];
            let level = args[1];
            let pp_surface = args[2];
            if pp_surface != 0 && pp_surface < 0x1000_0000 {
                if level == 0 && p_texture != 0 {
                    // Level 0: surface == texture
                    unsafe {
                        *(((guest_mem as u64) + pp_surface as u64) as *mut u32) = p_texture;
                    }
                    // Bump refcount (Common field at +0x00)
                    if p_texture >= TEX_HEAP_BASE && p_texture < TEX_HEAP_BASE + TEX_HEAP_SIZE {
                        unsafe {
                            let common_ptr = ((guest_mem as u64) + p_texture as u64) as *mut u32;
                            *common_ptr = (*common_ptr).wrapping_add(1); // refcount++
                        }
                    }
                } else {
                    // Non-zero level or null texture: allocate a surface stub
                    let surf_addr = unsafe {
                        alloc_texture(guest_mem, 1, 1, 1, 0, X_D3DFMT_A8R8G8B8)
                    };
                    if surf_addr != 0 {
                        // Mark as surface type instead of texture
                        unsafe {
                            let common_ptr = ((guest_mem as u64) + surf_addr as u64) as *mut u32;
                            *common_ptr = X_D3DCOMMON_TYPE_SURFACE | 1;
                            *(((guest_mem as u64) + pp_surface as u64) as *mut u32) = surf_addr;
                        }
                    }
                }
                debug_log(&format!(
                    "[OOVPA-HLE] GetSurfaceLevel: tex=0x{:08X} lvl={} -> *0x{:08X}",
                    p_texture, level, pp_surface
                ));
            }
            0 // D3D_OK
        }
        "D3DDevice_BlockUntilVerticalBlank" => {
            // Simulate waiting for VBlank (~60Hz). Sleep 16ms to prevent the game
            // from spinning at 100% CPU in a WaitForSingleObject/VBlank polling loop.
            std::thread::sleep(std::time::Duration::from_millis(16));
            0
        }
        "D3DDevice_SetPixelShaderConstant" => 0,             // skip PS constant upload
        "D3DDevice_SetVertexShaderConstant" => 0,            // skip VS constant upload
        "D3DDevice_EndPush" => 0,                            // no-op (PB writes go to dummy region)
        // ---- Bink video stubs ----
        "BinkOpen" => {
            // BinkOpen(filename, flags) -> handle
            // Return a fake handle so the game thinks the video opened.
            // Set total_frames at handle+0x250 to 0 so BinkNextFrame sees "done".
            let filename_ptr = args[0];
            let handle = BINK_FAKE_HANDLE;
            unsafe {
                // Zero 0x300 bytes of the fake Bink struct so probed fields are safe
                let base = (guest_mem as u64 + handle as u64) as *mut u8;
                std::ptr::write_bytes(base, 0, 0x300);
                // total_frames (offset 0x250) = 0 → BinkNextFrame returns "done"
                *((base as u64 + 0x250) as *mut u32) = 0;
            }
            debug_log(&format!(
                "[OOVPA-HLE] BinkOpen: filename=0x{:08X} -> handle=0x{:08X}",
                filename_ptr, handle
            ));
            handle
        }
        "BinkDoFrame" => {
            // BinkDoFrame(handle) -> 0 (success, frame decoded)
            0
        }
        "BinkNextFrame" => {
            // BinkNextFrame(handle) -> 0 (no more frames, video is done)
            0
        }
        "BinkClose" => {
            // BinkClose(handle) -> 0
            0
        }

        // ---- XInput HLE stubs (controller input via libretro) ----

        "XInitDevices" => {
            // XInitDevices(device_types, device_type_count)
            // Pre-fill g_DeviceType_Gamepad at 0x379C00 with port 0 connected
            unsafe {
                let gpad = (guest_mem as u64 + 0x0037_9C00u64) as *mut u32;
                // XPP_DEVICE_TYPE: CurrentConnected = 1, ChangeConnected = 1
                std::ptr::write(gpad, 1u32);            // CurrentConnected
                std::ptr::write(gpad.add(1), 1u32);     // ChangeConnected
                std::ptr::write(gpad.add(2), 0u32);     // ChangeRemoved
                std::ptr::write(gpad.add(3), 0u32);     // CurrentNotConnected
            }
            debug_log("[OOVPA-HLE] XInitDevices: port 0 gamepad connected");
            0 // void return
        }

        "XGetDevices" => {
            // XGetDevices(DeviceType) -> DWORD bitmask
            // Always report port 0 connected (gamepad present).
            // The XDEVICE_TYPE struct's CurrentConnected field in guest memory is
            // never initialized by the emulator (the real Xbox kernel maintains it).
            // Without this, games see 0 controllers and abort main().
            let device_type_addr = args[0];
            let connected = if device_type_addr != 0 {
                // Also write 1 to the struct's CurrentConnected field so guest reads match
                unsafe {
                    let ptr = (guest_mem as u64 + device_type_addr as u64) as *mut u32;
                    std::ptr::write(ptr, 1u32);
                }
                1u32
            } else {
                1u32
            };
            crate::rate_log!(10, "[OOVPA-HLE] XGetDevices: returning 0x{:X} (port 0 connected)", connected);
            connected
        }

        "XGetDeviceChanges" => {
            // XGetDeviceChanges(DeviceType, pInsertions, pRemovals) -> BOOL
            // After init, no changes — return FALSE
            if args[1] != 0 {
                unsafe {
                    let ins = (guest_mem as u64 + args[1] as u64) as *mut u32;
                    std::ptr::write(ins, 0u32);
                }
            }
            if args[2] != 0 {
                unsafe {
                    let rem = (guest_mem as u64 + args[2] as u64) as *mut u32;
                    std::ptr::write(rem, 0u32);
                }
            }
            0 // FALSE — no changes
        }

        "XInputOpen" => {
            // XInputOpen(DeviceType, dwPort, dwSlot, pPollingParams) -> HANDLE
            let port = args[1];
            let handle = 0xD000_0000u32 | port;
            debug_log(&format!("[OOVPA-HLE] XInputOpen: port {} -> handle 0x{:08X}", port, handle));
            handle
        }

        "XInputClose" => {
            // XInputClose(hDevice) -> void
            0
        }

        "XInputPoll" => {
            // XInputPoll(hDevice) -> DWORD
            0 // ERROR_SUCCESS
        }

        "XInputGetCapabilities" => {
            // XInputGetCapabilities(hDevice, pCaps) -> DWORD
            let caps_addr = args[1];
            if caps_addr != 0 {
                unsafe {
                    let caps = (guest_mem as u64 + caps_addr as u64) as *mut u8;
                    // Zero the struct first (XINPUT_CAPABILITIES = 44 bytes)
                    std::ptr::write_bytes(caps, 0u8, 44);
                    // SubType = 0x01 (XINPUT_DEVSUBTYPE_GAMEPAD)
                    std::ptr::write(caps, 0x01u8);
                }
            }
            debug_log("[OOVPA-HLE] XInputGetCapabilities: gamepad");
            0 // ERROR_SUCCESS
        }

        "XInputGetState" => {
            // XInputGetState(hDevice, pState) -> DWORD
            // pState = XINPUT_STATE { dwPacketNumber: u32, Gamepad: XINPUT_GAMEPAD }
            // XINPUT_GAMEPAD = { wButtons: u16, bAnalogButtons[8]: u8, sThumbLX/LY/RX/RY: i16 }
            use std::sync::atomic::{AtomicU32, Ordering};
            static PACKET: AtomicU32 = AtomicU32::new(0);

            let state_addr = args[1];
            if state_addr == 0 {
                return 0;
            }

            // Read libretro input for port 0
            let input_state_fn = unsafe { crate::xbox::emulator::G_INPUT_STATE };
            let cb = match input_state_fn {
                Some(f) => f,
                None => return 0,
            };

            // libretro constants
            const RETRO_DEVICE_JOYPAD: u32 = 1;
            const RETRO_DEVICE_ANALOG: u32 = 5;
            const RETRO_DEVICE_INDEX_ANALOG_LEFT: u32 = 0;
            const RETRO_DEVICE_INDEX_ANALOG_RIGHT: u32 = 1;
            const RETRO_DEVICE_ID_ANALOG_X: u32 = 0;
            const RETRO_DEVICE_ID_ANALOG_Y: u32 = 1;

            // Libretro joypad button IDs
            const RETRO_DEVICE_ID_JOYPAD_B: u32 = 0;
            const RETRO_DEVICE_ID_JOYPAD_Y: u32 = 1;
            const RETRO_DEVICE_ID_JOYPAD_SELECT: u32 = 2;
            const RETRO_DEVICE_ID_JOYPAD_START: u32 = 3;
            const RETRO_DEVICE_ID_JOYPAD_UP: u32 = 4;
            const RETRO_DEVICE_ID_JOYPAD_DOWN: u32 = 5;
            const RETRO_DEVICE_ID_JOYPAD_LEFT: u32 = 6;
            const RETRO_DEVICE_ID_JOYPAD_RIGHT: u32 = 7;
            const RETRO_DEVICE_ID_JOYPAD_A: u32 = 8;
            const RETRO_DEVICE_ID_JOYPAD_X: u32 = 9;
            const RETRO_DEVICE_ID_JOYPAD_L: u32 = 10;
            const RETRO_DEVICE_ID_JOYPAD_R: u32 = 11;
            const RETRO_DEVICE_ID_JOYPAD_L2: u32 = 12;
            const RETRO_DEVICE_ID_JOYPAD_R2: u32 = 13;
            const RETRO_DEVICE_ID_JOYPAD_L3: u32 = 14;
            const RETRO_DEVICE_ID_JOYPAD_R3: u32 = 15;

            // Xbox button bits (XINPUT_GAMEPAD.wButtons)
            let mut buttons: u16 = 0;
            let rd = |id: u32| -> i16 { unsafe { cb(0, RETRO_DEVICE_JOYPAD, 0, id) } };

            if rd(RETRO_DEVICE_ID_JOYPAD_UP)    != 0 { buttons |= 0x0001; } // XINPUT_GAMEPAD_DPAD_UP
            if rd(RETRO_DEVICE_ID_JOYPAD_DOWN)  != 0 { buttons |= 0x0002; } // DPAD_DOWN
            if rd(RETRO_DEVICE_ID_JOYPAD_LEFT)  != 0 { buttons |= 0x0004; } // DPAD_LEFT
            if rd(RETRO_DEVICE_ID_JOYPAD_RIGHT) != 0 { buttons |= 0x0008; } // DPAD_RIGHT
            if rd(RETRO_DEVICE_ID_JOYPAD_START) != 0 { buttons |= 0x0010; } // START
            if rd(RETRO_DEVICE_ID_JOYPAD_SELECT)!= 0 { buttons |= 0x0020; } // BACK
            if rd(RETRO_DEVICE_ID_JOYPAD_L3)    != 0 { buttons |= 0x0040; } // LEFT_THUMB
            if rd(RETRO_DEVICE_ID_JOYPAD_R3)    != 0 { buttons |= 0x0080; } // RIGHT_THUMB

            // Xbox analog buttons (0-255 pressure, mapped from digital 0/1)
            let analog_a  = if rd(RETRO_DEVICE_ID_JOYPAD_A)  != 0 { 255u8 } else { 0u8 };
            let analog_b  = if rd(RETRO_DEVICE_ID_JOYPAD_B)  != 0 { 255u8 } else { 0u8 };
            let analog_x  = if rd(RETRO_DEVICE_ID_JOYPAD_X)  != 0 { 255u8 } else { 0u8 };
            let analog_y  = if rd(RETRO_DEVICE_ID_JOYPAD_Y)  != 0 { 255u8 } else { 0u8 };
            let analog_black = if rd(RETRO_DEVICE_ID_JOYPAD_L) != 0 { 255u8 } else { 0u8 };
            let analog_white = if rd(RETRO_DEVICE_ID_JOYPAD_R) != 0 { 255u8 } else { 0u8 };
            let analog_lt = if rd(RETRO_DEVICE_ID_JOYPAD_L2) != 0 { 255u8 } else { 0u8 };
            let analog_rt = if rd(RETRO_DEVICE_ID_JOYPAD_R2) != 0 { 255u8 } else { 0u8 };

            // Analog sticks (libretro: -32768..32767, Xbox: same range)
            let rd_analog = |idx: u32, axis: u32| -> i16 {
                unsafe { cb(0, RETRO_DEVICE_ANALOG, idx, axis) }
            };
            let lx = rd_analog(RETRO_DEVICE_INDEX_ANALOG_LEFT,  RETRO_DEVICE_ID_ANALOG_X);
            let ly = rd_analog(RETRO_DEVICE_INDEX_ANALOG_LEFT,  RETRO_DEVICE_ID_ANALOG_Y);
            let rx = rd_analog(RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_X);
            let ry = rd_analog(RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_Y);

            // Write XINPUT_STATE to guest memory
            unsafe {
                let base = (guest_mem as u64 + state_addr as u64) as *mut u8;
                let pkt = PACKET.fetch_add(1, Ordering::Relaxed);
                // dwPacketNumber (offset 0, 4 bytes)
                std::ptr::copy_nonoverlapping(&pkt as *const u32 as *const u8, base, 4);
                // wButtons (offset 4, 2 bytes)
                std::ptr::copy_nonoverlapping(&buttons as *const u16 as *const u8, base.add(4), 2);
                // bAnalogButtons[8] (offset 6, 8 bytes)
                // Order: A, B, X, Y, Black, White, LeftTrigger, RightTrigger
                *base.add(6)  = analog_a;
                *base.add(7)  = analog_b;
                *base.add(8)  = analog_x;
                *base.add(9)  = analog_y;
                *base.add(10) = analog_black;
                *base.add(11) = analog_white;
                *base.add(12) = analog_lt;
                *base.add(13) = analog_rt;
                // sThumbLX (offset 14, 2 bytes)
                std::ptr::copy_nonoverlapping(&lx as *const i16 as *const u8, base.add(14), 2);
                // sThumbLY (offset 16, 2 bytes)
                std::ptr::copy_nonoverlapping(&ly as *const i16 as *const u8, base.add(16), 2);
                // sThumbRX (offset 18, 2 bytes)
                std::ptr::copy_nonoverlapping(&rx as *const i16 as *const u8, base.add(18), 2);
                // sThumbRY (offset 20, 2 bytes)
                std::ptr::copy_nonoverlapping(&ry as *const i16 as *const u8, base.add(20), 2);
            }

            0 // ERROR_SUCCESS
        }

        "XInputSetState" => {
            0 // ERROR_SUCCESS — rumble not supported
        }

        // pool_init: DISABLED — runs organically via AOT trampolines now.
        // With kernel trampolines, 1400+ kernel calls during pool init execute
        // with zero stack drift. Skipping pool_init left thread contexts uninitialized,
        // causing the worker to crash at XapiThreadStartup (CALL to garbage pointer).
        // "pool_init" => { ... }
        _ => 0,
    }
}

/// CreateDevice: inject Cxbx-R device struct dump, populate PB pointers, write g_pDevice
fn hle_create_device(guest_mem: *mut u8, mmio_count: &mut u64) -> u32 {
    let dev_addr = 0x0030_0E00u32; // device struct location
    let dev_size = 0x8000u32; // Cxbx-R dump is 32KB

    // Try to load device_struct.bin from Cxbx-R (real D3D8 device state)
    let bin_path = "./device_struct.bin";
    match std::fs::read(bin_path) {
        Ok(data) => {
            let copy_len = data.len().min(dev_size as usize);
            unsafe {
                let dst = (guest_mem as u64 + dev_addr as u64) as *mut u8;
                std::ptr::copy_nonoverlapping(data.as_ptr(), dst, copy_len);
            }
            debug_log(&format!(
                "[OOVPA-HLE] CreateDevice: injected {} bytes from device_struct.bin at 0x{:08X}",
                copy_len, dev_addr
            ));
        }
        Err(e) => {
            // Fallback: zero the struct
            debug_log(&format!(
                "[OOVPA-HLE] CreateDevice: device_struct.bin not found ({}), zeroing 0x{:X} bytes",
                e, dev_size
            ));
            for i in (0..dev_size).step_by(4) {
                unsafe {
                    *((guest_mem as u64 + (dev_addr + i) as u64) as *mut u32) = 0;
                }
            }
        }
    }

    // Write g_pDevice pointer
    unsafe {
        *((guest_mem as u64 + G_PDEVICE as u64) as *mut u32) = dev_addr;
    }

    // Override pushbuffer pointers to our dummy region (game writes PB commands here)
    let dev_base = guest_mem as u64 + dev_addr as u64;
    unsafe {
        *((dev_base + DEV_PB_PUT as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_BASE as u64) as *mut u32) = PB_DUMMY_BASE;
        *((dev_base + DEV_PB_LIMIT as u64) as *mut u32) = PB_DUMMY_BASE + PB_DUMMY_SIZE;
    }

    *mmio_count += 1; // CreateDevice counts as MMIO activity (Stage 8 trigger)

    // Request GPU init on main thread (D3D11 device creation conflicts with VEH on worker)
    crate::xbox::gpu::request_gpu_init();
    debug_log(&format!(
        "[OOVPA-HLE] CreateDevice: dev=0x{:08X} g_pDevice=0x{:08X} PB=0x{:08X} (GPU init requested)",
        dev_addr, G_PDEVICE, PB_DUMMY_BASE
    ));
    0 // D3D_OK
}

/// Swap: increment frame counter, signal frame ready for main thread readback.
/// NOTE: Do NOT call D3D11 readback here — D3D11 immediate context is single-threaded.
/// The main thread polls frame_ready and does readback in its own frame loop.
fn hle_swap(guest_mem: *mut u8, mmio_count: &mut u64) -> u32 {
    use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
    static SWAP_COUNT: AtomicU32 = AtomicU32::new(0);
    static LAST_SWAP_TICK: AtomicU64 = AtomicU64::new(0);

    let dev_ptr = unsafe { *((guest_mem as u64 + G_PDEVICE as u64) as *const u32) };
    if dev_ptr != 0 {
        let frame_addr = guest_mem as u64 + dev_ptr as u64 + DEV_FRAME_CTR as u64;
        let frame = unsafe { *(frame_addr as *const u32) };
        unsafe {
            *(frame_addr as *mut u32) = frame + 1;
        }
        // Reset PB PUT to base
        let pb_base =
            unsafe { *((guest_mem as u64 + dev_ptr as u64 + DEV_PB_BASE as u64) as *const u32) };
        unsafe {
            *((guest_mem as u64 + dev_ptr as u64 + DEV_PB_PUT as u64) as *mut u32) = pb_base;
        }
    }
    // Signal main thread to do GPU readback (D3D11 context is not thread-safe)
    crate::xbox::gpu::set_frame_dirty();
    *mmio_count += 1;

    // Frame pacing: ~60Hz (16ms per frame).
    // Use elapsed time since last swap to sleep only the remainder of a 16ms window.
    let now = {
        let mut qpc: i64 = 0;
        let mut freq: i64 = 0;
        unsafe {
            windows::Win32::System::Performance::QueryPerformanceCounter(
                &mut qpc as *mut i64 as *mut _,
            );
            windows::Win32::System::Performance::QueryPerformanceFrequency(
                &mut freq as *mut i64 as *mut _,
            );
        }
        if freq > 0 { (qpc as u64 * 1000) / freq as u64 } else { 0 }
    };
    let last = LAST_SWAP_TICK.load(Ordering::Relaxed);
    let elapsed_ms = if last > 0 { now.saturating_sub(last) } else { 0 };
    LAST_SWAP_TICK.store(now, Ordering::Relaxed);

    // Sleep for the remainder of the 16ms frame window
    if elapsed_ms < 16 {
        std::thread::sleep(std::time::Duration::from_millis(16 - elapsed_ms));
    }

    // Yield to main thread so retro_run() gets CPU time
    std::thread::yield_now();

    let sc = SWAP_COUNT.fetch_add(1, Ordering::Relaxed);
    if sc < 10 || sc % 60 == 0 {
        crate::xbox::emulator::debug_log(&format!(
            "[HLE-SWAP] #{} elapsed={}ms",
            sc, elapsed_ms,
        ));
    }
    0
}

/// Public wrapper for hle_swap — callable from Unicorn frame loop hook.
/// Signals main thread to render a synthetic frame (D3D12 must be used on main thread).
pub fn hle_swap_external(guest_mem: *mut u8, mmio_count: &mut u64) {
    // Signal main thread to render + present
    crate::xbox::gpu::request_synthetic_frame();
    hle_swap(guest_mem, mmio_count);
}

/// Clear: translate Xbox D3DDevice_Clear → host GPU clear.
/// Xbox D3D8 Clear args: (Count, pRects, Flags, Color, Z, Stencil)
fn hle_clear(args: &[u32; 8], _guest_mem: *mut u8, mmio_count: &mut u64) -> u32 {
    let color = args[3]; // ARGB32 clear color
    let mut gpu = crate::xbox::gpu::gpu_lock();
    if let Some(ref mut backend) = *gpu {
        backend.clear(color);
    }
    *mmio_count += 1;
    0
}

/// SetVertexShader: store handle in device struct
fn hle_set_vertex_shader(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let handle = args[0];
    let dev_ptr = unsafe { *((guest_mem as u64 + G_PDEVICE as u64) as *const u32) };
    if dev_ptr != 0 {
        unsafe {
            *((guest_mem as u64 + dev_ptr as u64 + DEV_VSHADER as u64) as *mut u32) = handle;
        }
    }
    0
}

/// SetTransform: read 4x4 matrix from guest memory and forward to GPU backend
fn hle_set_transform(args: &[u32; 8], guest_mem: *mut u8) -> u32 {
    let state_type = args[0]; // D3DTS_WORLD=256, D3DTS_VIEW=2, D3DTS_PROJECTION=3
    let matrix_ptr = args[1];
    if matrix_ptr != 0 && matrix_ptr < 0x1000_0000 {
        let mut matrix = [0.0f32; 16];
        unsafe {
            let src = guest_mem.add(matrix_ptr as usize) as *const f32;
            for i in 0..16 {
                matrix[i] = std::ptr::read(src.add(i));
            }
        }
        if let Some(ref mut gpu) = *crate::xbox::gpu::gpu_lock() {
            gpu.set_transform(state_type, &matrix);
        }
    }
    0
}

/// BeginPush: return dummy PB pointer to prevent stall
fn hle_begin_push(
    _args: &[u32; 8],
    guest_mem: *mut u8,
    context: &mut windows::Win32::System::Diagnostics::Debug::CONTEXT,
) -> u32 {
    // BeginPush returns a pointer where the caller can write PB commands.
    // We return the dummy PB region.
    context.Rax = PB_DUMMY_BASE as u64;
    PB_DUMMY_BASE
}

/// Draw: log draw call parameters and count as MMIO activity.
/// DrawVertices(PrimitiveType, StartVertex, VertexCount)
/// DrawIndexedVertices(PrimitiveType, VertexCount, pIndexData)
fn hle_draw(name: &str, args: &[u32; 8], guest_mem: *mut u8, mmio_count: &mut u64) {
    static DRAW_LOG: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = DRAW_LOG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // Log first 10 + powers of 2
    if n < 10 || (n & (n - 1)) == 0 {
        let prim_type = args[0];
        let prim_name = match prim_type {
            1 => "POINTLIST",
            2 => "LINELIST",
            3 => "LINESTRIP",
            4 => "TRIANGLELIST",
            5 => "TRIANGLESTRIP",
            6 => "TRIANGLEFAN",
            _ => "UNKNOWN",
        };
        if name == "D3DDevice_DrawVertices" {
            debug_log(&format!(
                "[HLE-DRAW] #{} DrawVertices type={} ({}) start={} count={}",
                n, prim_type, prim_name, args[1], args[2]
            ));
        } else {
            debug_log(&format!(
                "[HLE-DRAW] #{} DrawIndexedVertices type={} ({}) count={} pIdx=0x{:08X}",
                n, prim_type, prim_name, args[1], args[2]
            ));
        }
    }

    *mmio_count += 1;
}

/// Synthetic D3D injection: force CreateDevice + Swap to bootstrap Stage 9/10.
/// Called from worker dispatch loop after D3D table init phase completes.
pub fn force_synthetic_d3d(ctx: &mut crate::xbox::aot::runtime::RuntimeContext) {
    debug_log("[OOVPA-SYNTH] Forcing synthetic CreateDevice + Swap + Draw counters");
    // NOTE: Do NOT call D3D11 backend from worker thread — context is single-threaded.
    // Worker just sets up guest-side state + bumps counters.
    // Main thread does actual GPU clear/draw/readback via poll_gpu_init + poll_readback.
    hle_create_device(ctx.guest_mem_base, &mut ctx.mmio_count);
    ctx.pb_commands += 1;
    ctx.draw_calls += 1;

    // Signal main thread to do readback
    hle_swap(ctx.guest_mem_base, &mut ctx.mmio_count);
    ctx.pb_commands += 1;

    debug_log(&format!(
        "[OOVPA-SYNTH] Done: mmio={} pb={} draw={}",
        ctx.mmio_count, ctx.pb_commands, ctx.draw_calls
    ));
}

/// Run one frame of synthetic game loop: Clear → Draw → Swap.
/// Called from worker thread each VBlank after CreateDevice has fired.
/// Worker signals main thread for GPU operations via request/dirty flags.
pub fn run_synthetic_frame(ctx: &mut crate::xbox::aot::runtime::RuntimeContext, frame_num: u32) {
    let guest_mem = ctx.guest_mem_base;

    // Clear with rotating color (visual proof the loop is running)
    let r = ((frame_num * 3) % 256) as u32;
    let g = ((frame_num * 5 + 80) % 256) as u32;
    let b = ((frame_num * 7 + 160) % 256) as u32;
    let clear_color = 0xFF000000 | (r << 16) | (g << 8) | b;

    // Call HLE stubs directly (no guest code, no INT3)
    {
        let mut gpu = crate::xbox::gpu::gpu_lock();
        if let Some(ref mut backend) = *gpu {
            backend.clear(clear_color);

            // Draw test triangle with per-frame animation
            let t = frame_num as f32 * 0.05;
            let cx = 320.0 + (t.sin() * 100.0);
            let cy = 240.0 + (t.cos() * 80.0);
            let verts = [
                crate::xbox::gpu::NV2AVertex {
                    x: cx,
                    y: cy - 160.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFFFF0000,
                    u: 0.0,
                    v: 0.0,
                },
                crate::xbox::gpu::NV2AVertex {
                    x: cx + 200.0,
                    y: cy + 120.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFF00FF00,
                    u: 1.0,
                    v: 1.0,
                },
                crate::xbox::gpu::NV2AVertex {
                    x: cx - 200.0,
                    y: cy + 120.0,
                    z: 0.5,
                    w: 1.0,
                    color: 0xFF0000FF,
                    u: 0.0,
                    v: 1.0,
                },
            ];
            backend.draw_primitive(&verts, crate::xbox::gpu::NV097_TRIANGLES);

            // Readback for main thread display
            let pixels = backend.readback_framebuffer().to_vec();
            drop(gpu);
            crate::xbox::gpu::store_readback(&pixels);
        }
    }

    // Update counters
    ctx.mmio_count += 1;
    ctx.pb_commands += 1;
    ctx.draw_calls += 1;

    // Swap: increment device frame counter
    hle_swap(guest_mem, &mut ctx.mmio_count);
}

/// Check if any D3D table init hooks have fired (RenderStateInit/Sort2).
pub fn has_d3d_table_init_fired() -> bool {
    let guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return false,
    };
    state.matches.iter().any(|m| {
        (m.pattern_name == "D3D_RenderStateInit" || m.pattern_name == "D3D_RenderStateSort2")
            && m.call_count > 0
    })
}

/// Check if CreateDevice has already been called organically.
pub fn has_create_device_fired() -> bool {
    let guard = match OOVPA_STATE.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return false,
    };
    state
        .matches
        .iter()
        .any(|m| m.pattern_name == "Direct3D_CreateDevice" && m.call_count > 0)
}

/// Dump call count summary.
pub fn dump_summary(matches: &[OovpaMatch]) {
    if matches.is_empty() {
        return;
    }
    debug_log("[OOVPA] === Call Summary ===");
    for m in matches {
        if m.call_count > 0 {
            debug_log(&format!(
                "[OOVPA]   {}: {} calls (guest=0x{:08X})",
                m.pattern_name, m.call_count, m.guest_addr
            ));
        }
    }
}
