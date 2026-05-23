/// 12-stage boot display system.
/// Ports AOT_Display.h/.cpp — shows emulator progress with colored backgrounds
/// and bitmap font text overlay. Stage 11 adds a diagnostic HUD overlaid on the
/// GPU triangle. Stage 12 = actual game content rendering.

pub const FB_WIDTH: usize = 640;
pub const FB_HEIGHT: usize = 480;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum BootStage {
    XbeLoading = 1,   // XBE file opened
    AotCompiling = 2, // AOT compiler running
    CacheReady = 3,   // AOT done/cached
    CrtStartup = 4,   // Event: first kernel call
    StaticCtors = 5,  // Event: worker thread alive
    GameMain = 6,     // Event: first OOVPA hook fired
    GameLogic = 7,    // Event: CreateDevice HLE fired
    MmioActive = 8,   // mmio_count > 0 — first NV2A register
    PushBuffer = 9,   // pb_commands > 0 — pushbuffer command
    GameRunning = 10, // draw_calls > 0 — geometry drawn
    HudOverlay = 11,  // Diagnostic HUD overlaid on GPU triangle
    GameContent = 12, // Actual game content (Spider-Man logo, gameplay)
}

impl BootStage {
    /// Background color (R, G, B) for each stage.
    pub fn color(self) -> (u8, u8, u8) {
        match self {
            Self::XbeLoading => (0x0A, 0x0A, 0x0A),   // Near black
            Self::AotCompiling => (0x1A, 0x00, 0x00), // Dark red
            Self::CacheReady => (0x1A, 0x08, 0x00),   // Dark orange
            Self::CrtStartup => (0x1A, 0x1A, 0x00),   // Dark yellow
            Self::StaticCtors => (0x0A, 0x1A, 0x00),  // Olive green
            Self::GameMain => (0x00, 0x1A, 0x1A),     // Teal
            Self::GameLogic => (0x64, 0x95, 0xED),    // Cornflower blue
            Self::MmioActive => (0xFF, 0x00, 0xFF),   // Magenta
            Self::PushBuffer => (0x00, 0xFF, 0x40),   // Bright green
            Self::GameRunning => (0x00, 0x00, 0x00),  // Black
            Self::HudOverlay => (0x00, 0x00, 0x00),   // Black (GPU draws behind)
            Self::GameContent => (0x00, 0x00, 0x00),  // Black (game drives display)
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::XbeLoading => "STAGE 1/12 - LOADING XBE",
            Self::AotCompiling => "STAGE 2/12 - AOT COMPILATION",
            Self::CacheReady => "STAGE 3/12 - AOT DONE",
            Self::CrtStartup => "STAGE 4/12 - CRT STARTUP",
            Self::StaticCtors => "STAGE 5/12 - STATIC CONSTRUCTORS",
            Self::GameMain => "STAGE 6/12 - GAME MAIN",
            Self::GameLogic => "STAGE 7/12 - GAME LOGIC",
            Self::MmioActive => "STAGE 8/12 - NV2A MMIO",
            Self::PushBuffer => "STAGE 9/12 - PUSHBUFFER",
            Self::GameRunning => "STAGE 10/12 - GAME RUNNING",
            Self::HudOverlay => "STAGE 11/12 - HUD DIAGNOSTIC",
            Self::GameContent => "STAGE 12/12 - GAME CONTENT",
        }
    }

    pub fn meaning(self) -> &'static str {
        match self {
            Self::XbeLoading => "Reading XBE executable, mapping sections to guest RAM",
            Self::AotCompiling => "Translating x86 guest code to x64 host code",
            Self::CacheReady => "Translation complete - starting execution",
            Self::CrtStartup => "First kernel call - guest code executing",
            Self::StaticCtors => "Worker thread alive - CRT init running",
            Self::GameMain => "OOVPA hooks firing - D3D/SDK intercepted",
            Self::GameLogic => "CreateDevice done - game logic running",
            Self::MmioActive => "First NV2A GPU register touched",
            Self::PushBuffer => "Pushbuffer command processed",
            Self::GameRunning => "Geometry drawn to screen",
            Self::HudOverlay => "HLE diagnostic overlay on GPU output",
            Self::GameContent => "Game content rendering",
        }
    }
}

/// Milestone events for game-agnostic stage advancement.
/// Each event maps to one stage transition. Fire once when the real event occurs.
#[derive(Debug, Clone, Copy)]
pub enum BootEvent {
    /// First kernel call dispatched (any game) → Stage 4 CrtStartup
    FirstKernelCall,
    /// Worker thread confirmed alive → Stage 5 StaticCtors
    WorkerAlive,
    /// Any OOVPA hook fired (D3D/DSOUND intercepted) → Stage 6 GameMain
    OovpaHooksFired,
    /// CreateDevice HLE completed → Stage 7 GameLogic
    CreateDeviceFired,
}

/// Convert 8-bit RGB to XRGB8888.
fn rgb_to_xrgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Stage display renderer. Owns the framebuffer and renders boot progress.
pub struct StageDisplay {
    stage: BootStage,
    framebuffer: Vec<u32>, // 640x480 XRGB8888
    game_title: String,
    pub dispatches: u32,
    pub mmio_count: u32,
    pub pb_commands: u32,
    pub draw_calls: u32,
    // Pipeline status for display
    pub kernel_calls: u32,
    pub veh_events: u32,
    pub recoveries: u32,
    pub oovpa_hooks_fired: u32,
    pub hle_create_device: bool,
    pub hle_swap_count: u32,
    pub hle_draw_count: u32,
    /// NV2A draw-method submissions seen by the pushbuffer parser (DRAW_ARRAYS,
    /// INLINE_ARRAY/SET_BEGIN_END close, INLINE_ELEMENTS). The **real** measure
    /// of how many draws the game submitted; `hle_draw_count` only counts SDK
    /// helpers (DrawVertices/DrawIndexedVertices), which Spider-Man bypasses
    /// via BeginPush macros.
    pub nv2a_draws: u64,
    pub worker_alive: bool,
    pub gpu_backend: &'static str,
    pub prev_kernel_calls: u32,
    pub stall_frames: u32,
    // Stage 11 HUD: last known ordinal name + last HLE hook name
    pub last_kernel_ord: u32,
    pub last_hle_name: &'static str,
    pub worker_esp: u32,
    pub worker_initial_esp: u32,
    pub stack_scans: u32,
}

impl StageDisplay {
    pub fn new() -> Self {
        Self {
            stage: BootStage::XbeLoading,
            framebuffer: vec![0u32; FB_WIDTH * FB_HEIGHT],
            game_title: String::new(),
            dispatches: 0,
            mmio_count: 0,
            pb_commands: 0,
            draw_calls: 0,
            kernel_calls: 0,
            veh_events: 0,
            recoveries: 0,
            oovpa_hooks_fired: 0,
            hle_create_device: false,
            hle_swap_count: 0,
            hle_draw_count: 0,
            nv2a_draws: 0,
            worker_alive: false,
            gpu_backend: "none",
            prev_kernel_calls: 0,
            stall_frames: 0,
            last_kernel_ord: 0,
            last_hle_name: "",
            worker_esp: 0,
            worker_initial_esp: 0,
            stack_scans: 0,
        }
    }

    pub fn stage(&self) -> BootStage {
        self.stage
    }

    pub fn set_stage(&mut self, stage: BootStage) {
        if stage > self.stage {
            log::info!("Stage: {} -> {}", self.stage.title(), stage.title());
            self.stage = stage;
        }
    }

    pub fn set_game_title(&mut self, title: &str) {
        self.game_title = title.to_string();
    }

    /// Update stage from live counters. Stages 4-7 are event-driven (set via
    /// `signal_event`), not threshold-driven. Stages 8-10 use GPU counter checks.
    pub fn update_from_counters(&mut self) {
        // Stages 4-7: event-driven — advanced by signal_event(), not thresholds.
        // Stages 8-10: GPU counters (real hardware events, game-agnostic)
        if self.stage < BootStage::MmioActive && self.mmio_count > 0 {
            self.set_stage(BootStage::MmioActive);
        }
        if self.stage < BootStage::PushBuffer && self.pb_commands > 0 {
            self.set_stage(BootStage::PushBuffer);
        }
        if self.stage < BootStage::GameRunning && self.draw_calls > 0 {
            self.set_stage(BootStage::GameRunning);
        }
        // Stage 11: auto-advance once draw calls arrive
        if self.stage == BootStage::GameRunning && self.draw_calls > 0 {
            self.set_stage(BootStage::HudOverlay);
        }
        // Stall detection: if kernel_calls haven't changed in 60 frames (~1 sec)
        if self.kernel_calls == self.prev_kernel_calls && self.kernel_calls > 0 {
            self.stall_frames += 1;
        } else {
            self.stall_frames = 0;
        }
        self.prev_kernel_calls = self.kernel_calls;
    }

    /// Signal a milestone event. Game-agnostic — callers fire these when real
    /// events happen (first kernel call, worker alive, OOVPA hooks fire, etc.)
    /// rather than relying on counter thresholds.
    pub fn signal_event(&mut self, event: BootEvent) {
        match event {
            BootEvent::FirstKernelCall => {
                self.set_stage(BootStage::CrtStartup);
            }
            BootEvent::WorkerAlive => {
                self.set_stage(BootStage::StaticCtors);
            }
            BootEvent::OovpaHooksFired => {
                self.set_stage(BootStage::GameMain);
            }
            BootEvent::CreateDeviceFired => {
                self.set_stage(BootStage::GameLogic);
            }
        }
    }

    /// Render the stage display and return the framebuffer as bytes (XRGB8888).
    pub fn render(&mut self) -> &[u8] {
        let (r, g, b) = self.stage.color();
        let bg = rgb_to_xrgb(r, g, b);

        // Fill background
        for pixel in self.framebuffer.iter_mut() {
            *pixel = bg;
        }

        let white: u32 = rgb_to_xrgb(0xFF, 0xFF, 0xFF);
        let yellow: u32 = rgb_to_xrgb(0xFF, 0xFF, 0x00);
        // Use dark text on bright backgrounds (Stage 9 green, Stage 10 black)
        let is_bright_bg = matches!(self.stage, BootStage::PushBuffer);
        let grey: u32 = if is_bright_bg {
            rgb_to_xrgb(0x10, 0x10, 0x10) // Dark grey on bright green
        } else {
            rgb_to_xrgb(0xAA, 0xAA, 0xAA)
        };
        let mx = 30; // Left margin

        // Header
        self.draw_string("RUSTEMU", mx, 10, 3, white);
        if !self.game_title.is_empty() {
            let title = self.game_title.clone();
            self.draw_string(&title, mx + 250, 18, 1, grey);
        }

        // Stage bar
        let stage_str = format!("{} | {}", self.stage.title(), self.stage.meaning());
        self.draw_string(&stage_str, mx, 50, 1, yellow);

        // === CPU PIPELINE ===
        let cpu_color = if self.worker_alive {
            if is_bright_bg {
                rgb_to_xrgb(0x00, 0x60, 0x00)
            } else {
                rgb_to_xrgb(0x00, 0xFF, 0x00)
            }
        } else {
            rgb_to_xrgb(0xFF, 0x40, 0x40)
        };
        self.draw_string("-- CPU --", mx, 80, 2, white);
        let cpu1 = format!(
            "Worker: {}  Dispatches: {}  VEH: {}",
            if self.worker_alive { "ALIVE" } else { "DEAD" },
            self.dispatches,
            self.veh_events
        );
        self.draw_string(&cpu1, mx, 105, 1, cpu_color);
        let cpu2 = format!(
            "Kernel calls: {}  Recoveries: {}",
            self.kernel_calls, self.recoveries
        );
        self.draw_string(&cpu2, mx, 120, 1, grey);

        // === HLE PIPELINE ===
        let hle_color = if self.oovpa_hooks_fired > 0 {
            if is_bright_bg {
                rgb_to_xrgb(0x00, 0x60, 0x00)
            } else {
                rgb_to_xrgb(0x00, 0xFF, 0x00)
            }
        } else {
            if is_bright_bg {
                rgb_to_xrgb(0x40, 0x40, 0x40)
            } else {
                rgb_to_xrgb(0x80, 0x80, 0x80)
            }
        };
        self.draw_string("-- HLE --", mx, 155, 2, white);
        let hle1 = format!(
            "OOVPA hooks fired: {}  CreateDevice: {}",
            self.oovpa_hooks_fired,
            if self.hle_create_device { "YES" } else { "no" }
        );
        self.draw_string(&hle1, mx, 180, 1, hle_color);
        let hle2 = format!(
            "Swap: {}  Draw: {}  MMIO: {}",
            self.hle_swap_count, self.hle_draw_count, self.mmio_count
        );
        self.draw_string(&hle2, mx, 195, 1, hle_color);

        // === GPU PIPELINE ===
        let gpu_color = if self.pb_commands > 0 {
            if is_bright_bg {
                rgb_to_xrgb(0x00, 0x60, 0x00)
            } else {
                rgb_to_xrgb(0x00, 0xFF, 0x00)
            }
        } else {
            if is_bright_bg {
                rgb_to_xrgb(0x40, 0x40, 0x40)
            } else {
                rgb_to_xrgb(0x80, 0x80, 0x80)
            }
        };
        self.draw_string("-- GPU --", mx, 230, 2, white);
        let gpu1 = format!(
            "Backend: {}  PB cmds: {}  Draws: {}",
            self.gpu_backend, self.pb_commands, self.draw_calls
        );
        self.draw_string(&gpu1, mx, 255, 1, gpu_color);

        // === PIPELINE STATUS ===
        self.draw_string("-- PIPELINE --", mx, 290, 2, white);
        // Show what's working and what's blocked
        let aot_status = if self.kernel_calls > 0 {
            "[OK]"
        } else {
            "[...]"
        };
        let hle_status = if self.hle_create_device {
            "[OK]"
        } else if self.oovpa_hooks_fired > 0 {
            "[PARTIAL]"
        } else {
            "[WAITING]"
        };
        let pfifo_status = if self.pb_commands > 0 {
            "[OK]"
        } else {
            "[WAITING]"
        };
        let render_status = if self.draw_calls > 0 {
            "[OK]"
        } else {
            "[WAITING]"
        };
        let pipe = format!(
            "AOT {} > HLE {} > PFIFO {} > RENDER {}",
            aot_status, hle_status, pfifo_status, render_status
        );
        self.draw_string(&pipe, mx, 315, 1, yellow);

        // Blocker hint with stall detection
        let blocker = if self.stall_frames > 60 {
            "STALLED - kernel calls not advancing"
        } else if self.draw_calls > 0 {
            "Game rendering active"
        } else if self.pb_commands > 0 {
            "Waiting for draw calls"
        } else if self.hle_create_device {
            "Loading assets..."
        } else if self.oovpa_hooks_fired > 0 {
            "HLE active, waiting for CreateDevice"
        } else if self.mmio_count > 0 {
            "MMIO active, OOVPA hooks not yet fired"
        } else if self.kernel_calls > 0 {
            "CPU running, waiting for D3D init"
        } else {
            "Booting..."
        };
        let blocker_color = if self.stall_frames > 60 {
            rgb_to_xrgb(0xFF, 0x00, 0x00) // Red for stall
        } else {
            rgb_to_xrgb(0xFF, 0xAA, 0x00) // Orange normal
        };
        self.draw_string(blocker, mx, 340, 1, blocker_color);

        // Stall counter
        if self.stall_frames > 0 {
            let stall_str = format!("Stall: {} frames", self.stall_frames);
            self.draw_string(&stall_str, mx, 360, 1, rgb_to_xrgb(0xFF, 0x60, 0x60));
        }

        // Footer
        self.draw_string("github.com/d3r2000/rustemu", mx, 440, 1, grey);

        // Return as byte slice
        unsafe {
            std::slice::from_raw_parts(
                self.framebuffer.as_ptr() as *const u8,
                self.framebuffer.len() * 4,
            )
        }
    }

    /// Draw a text string with a 1-pixel black drop-shadow behind each glyph.
    ///
    /// Rationale (2026-04-21): `render_hud_overlay` previously relied on
    /// `draw_panel` (50% darken rectangle) to make HUD text readable over
    /// GPU readback pixels. That panel obscures any real rendered content
    /// — including the diagnostic pulsing Clear color we use to prove the
    /// readback pipeline is alive. Drop-shadow text reads cleanly over any
    /// background without hiding what's underneath, so the panel can be
    /// removed entirely.
    ///
    /// Cost: 2x the glyph blits vs a single pass. Negligible at 640×480
    /// for the handful of HUD strings drawn per frame.
    fn draw_string(&mut self, text: &str, x: i32, y: i32, scale: i32, color: u32) {
        const SHADOW: u32 = 0; // pure black, opaque in XRGB8888
        let char_w = 8 * scale + scale;

        // Shadow pass — offset (+1, +1)
        let mut cx = x;
        for ch in text.chars() {
            if cx + 8 * scale > FB_WIDTH as i32 {
                break;
            }
            self.draw_char(ch, cx + 1, y + 1, scale, SHADOW);
            cx += char_w;
        }

        // Foreground pass
        let mut cx = x;
        for ch in text.chars() {
            if cx + 8 * scale > FB_WIDTH as i32 {
                break;
            }
            self.draw_char(ch, cx, y, scale, color);
            cx += char_w;
        }
    }

    /// Render the Stage 11 HUD overlay on top of existing GPU pixels (triangle stays behind).
    /// Draws semi-transparent panels with diagnostic text + progress bar.
    pub fn render_hud_overlay(&mut self, pixels: &mut [u32]) {
        // Copy GPU pixels into our framebuffer so we can draw on top
        assert!(pixels.len() >= FB_WIDTH * FB_HEIGHT);
        self.framebuffer
            .copy_from_slice(&pixels[..FB_WIDTH * FB_HEIGHT]);

        let white: u32 = rgb_to_xrgb(0xFF, 0xFF, 0xFF);
        let yellow: u32 = rgb_to_xrgb(0xFF, 0xFF, 0x00);
        let green: u32 = rgb_to_xrgb(0x00, 0xFF, 0x00);
        let red: u32 = rgb_to_xrgb(0xFF, 0x40, 0x40);
        let orange: u32 = rgb_to_xrgb(0xFF, 0xAA, 0x00);
        let grey: u32 = rgb_to_xrgb(0xAA, 0xAA, 0xAA);
        let panel: u32 = rgb_to_xrgb(0x00, 0x00, 0x00); // black panel bg
        let bar_bg: u32 = rgb_to_xrgb(0x30, 0x30, 0x30);
        let bar_fg: u32 = rgb_to_xrgb(0x00, 0xCC, 0x44);
        let mx = 16;

        // Panel removed (2026-04-21). Previously drew a 612×472 50%-darken
        // rectangle behind the HUD for readability; later shrunk to 400×280
        // as a half-measure. Both approaches hide GPU-rendered pixels
        // underneath.
        //
        // Replaced by drop-shadow text rendering inside `draw_string` —
        // each glyph gets a 1-pixel black shadow offset (+1, +1) that
        // makes it readable over any base color without darkening a
        // rectangle that obscures the underlying render.
        //
        // If you ever need the panel back (e.g. for a very bright or
        // textured background that even shadow text can't read over),
        // call `self.draw_panel(mx - 4, 4, 400, 280);` here — or better,
        // clip it to the actual text bounding box.

        // Header
        self.draw_string("RUSTEMU", mx, 10, 2, green);
        if !self.game_title.is_empty() {
            let title = self.game_title.clone();
            self.draw_string(&title, mx + 180, 14, 1, grey);
        }
        let stage_str = format!("{}", self.stage.title());
        self.draw_string(&stage_str, mx, 32, 1, yellow);

        // === PROGRESS BAR ===
        // Milestones: kernel_calls target 874K (from d2bc1e3), show how far we are
        let target = 874_000u32;
        let progress = if self.kernel_calls >= target {
            100
        } else {
            ((self.kernel_calls as u64 * 100) / target as u64) as u32
        };
        let bar_x = mx;
        let bar_y = 48;
        let bar_w = 580;
        let bar_h = 14;
        // Bar background
        self.draw_rect(bar_x, bar_y, bar_w, bar_h, bar_bg);
        // Bar fill
        let fill_w = ((bar_w as u64 * progress as u64) / 100) as i32;
        if fill_w > 0 {
            self.draw_rect(bar_x, bar_y, fill_w, bar_h, bar_fg);
        }
        // Bar text
        let bar_text = format!(
            "{}%  ({}/{}K kernel calls)",
            progress,
            self.kernel_calls,
            target / 1000
        );
        self.draw_string(&bar_text, bar_x + 4, bar_y + 3, 1, white);

        // === COUNTERS (2 columns) ===
        let y0 = 72;
        let col2 = mx + 300;

        let c1 = format!("Kernel Calls: {}", self.kernel_calls);
        self.draw_string(&c1, mx, y0, 1, white);
        let c2 = format!("MMIO: {}", self.mmio_count);
        self.draw_string(&c2, col2, y0, 1, white);

        let c3 = format!("VEH Events: {}", self.veh_events);
        self.draw_string(&c3, mx, y0 + 14, 1, white);
        let c4 = format!("Dispatches: {}", self.dispatches);
        self.draw_string(&c4, col2, y0 + 14, 1, white);

        let c5 = format!("PB Commands: {}", self.pb_commands);
        self.draw_string(&c5, mx, y0 + 28, 1, white);
        // c6 was "Draw Calls" — but it actually counts D3DDevice_Swap (frames
        // presented), not draws. Renamed to "Frames" for accuracy.
        let c6 = format!("Frames: {}", self.draw_calls);
        self.draw_string(&c6, col2, y0 + 28, 1, white);

        let c7 = format!("Stack Scans: {}", self.stack_scans);
        self.draw_string(&c7, mx, y0 + 42, 1, white);
        let c8 = format!("OOVPA Hooks: {}", self.oovpa_hooks_fired);
        self.draw_string(&c8, col2, y0 + 42, 1, white);

        // === HLE STATUS ===
        let hy = y0 + 64;
        self.draw_string("-- HLE --", mx, hy, 1, yellow);
        // First HLE row: CreateDevice + Swap counter (frames presented via D3D).
        let hle1 = format!(
            "CreateDevice: {}  Swap: {}",
            if self.hle_create_device { "YES" } else { "no" },
            self.hle_swap_count,
        );
        let hle_color = if self.hle_create_device { green } else { grey };
        self.draw_string(&hle1, mx, hy + 12, 1, hle_color);

        // Second HLE row: the two distinct draw counters, side-by-side so
        // their semantics are obvious at a glance.
        //   SDK    = D3DDevice_DrawVertices/DrawIndexedVertices OOVPA hits
        //            (Spider-Man uses BeginPush macros instead → stays 0)
        //   NV2A   = pushbuffer parser draw-method submissions
        //            (the **real** geometry-submitted-to-GPU count)
        let draw_row = format!(
            "Draws (SDK): {}  Draws (NV2A): {}",
            self.hle_draw_count, self.nv2a_draws
        );
        let draws_color = if self.nv2a_draws > 0 { green } else { grey };
        self.draw_string(&draw_row, mx, hy + 24, 1, draws_color);

        let gpu1 = format!(
            "Backend: {}  Worker: {}",
            self.gpu_backend,
            if self.worker_alive { "ALIVE" } else { "DEAD" }
        );
        let worker_color = if self.worker_alive { green } else { red };
        self.draw_string(&gpu1, mx, hy + 36, 1, worker_color);

        // === PIPELINE ===
        let py = hy + 56;
        self.draw_string("-- PIPELINE --", mx, py, 1, yellow);
        let aot_ok = self.dispatches > 0;
        let hle_ok = self.hle_create_device;
        let pfifo_ok = self.pb_commands > 0;
        // RENDER step is OK only when we've actually parsed real NV2A draw
        // methods from the pushbuffer — not when the SDK helpers fire (which
        // Spider-Man never calls) and not when D3DDevice_Swap fires (which
        // happens regardless of geometry).
        let render_ok = self.nv2a_draws > 0;
        let pipe = format!(
            "AOT [{}] > HLE [{}] > PFIFO [{}] > RENDER [{}]",
            if aot_ok { "OK" } else { ".." },
            if hle_ok { "OK" } else { ".." },
            if pfifo_ok { "OK" } else { ".." },
            if render_ok { "OK" } else { ".." },
        );
        self.draw_string(
            &pipe,
            mx,
            py + 12,
            1,
            if render_ok { green } else { orange },
        );

        // === BLOCKERS ===
        let by = py + 32;
        self.draw_string("-- BLOCKERS --", mx, by, 1, red);
        let mut bi = 0;

        if self.stall_frames > 60 {
            let stall_s = format!(
                "[!] STALLED {} frames - kernel calls frozen",
                self.stall_frames
            );
            self.draw_string(&stall_s, mx, by + 12 + bi * 12, 1, red);
            bi += 1;
        }
        if !self.hle_create_device && self.oovpa_hooks_fired > 0 {
            self.draw_string(
                "[!] CreateDevice HLE not fired yet",
                mx,
                by + 12 + bi * 12,
                1,
                orange,
            );
            bi += 1;
        }
        if self.pb_commands == 0 && self.hle_create_device {
            self.draw_string(
                "[!] Zero pushbuffer commands",
                mx,
                by + 12 + bi * 12,
                1,
                orange,
            );
            bi += 1;
        }
        if bi == 0 {
            self.draw_string("No blockers detected", mx, by + 12, 1, green);
        }

        // === LAST ACTIVITY ===
        let ly = by + 12 + (bi.max(1)) * 12 + 8;
        self.draw_string("-- LAST ACTIVITY --", mx, ly, 1, yellow);
        if self.last_kernel_ord > 0 {
            let ord_s = format!("Last kernel: ord #{}", self.last_kernel_ord);
            self.draw_string(&ord_s, mx, ly + 12, 1, grey);
        }
        if !self.last_hle_name.is_empty() {
            let hle_s = format!("Last HLE: {}", self.last_hle_name);
            self.draw_string(&hle_s, mx, ly + 24, 1, grey);
        }
        if self.worker_esp != 0 {
            let drift = self.worker_esp as i64 - self.worker_initial_esp as i64;
            let esp_s = format!("Worker ESP: 0x{:08X} (drift: {})", self.worker_esp, drift);
            self.draw_string(
                &esp_s,
                mx,
                ly + 36,
                1,
                if drift.abs() > 100 { red } else { grey },
            );
        }

        // Footer
        self.draw_string("github.com/d3r2000/rustemu", mx, 456, 1, grey);

        // Copy back to caller's buffer
        pixels[..FB_WIDTH * FB_HEIGHT].copy_from_slice(&self.framebuffer);
    }

    /// Draw a filled rectangle.
    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for py in y..(y + h) {
            for px in x..(x + w) {
                if px >= 0 && px < FB_WIDTH as i32 && py >= 0 && py < FB_HEIGHT as i32 {
                    self.framebuffer[py as usize * FB_WIDTH + px as usize] = color;
                }
            }
        }
    }

    /// Draw a semi-transparent dark panel (50% darken existing pixels).
    fn draw_panel(&mut self, x: i32, y: i32, w: i32, h: i32) {
        for py in y..(y + h) {
            for px in x..(x + w) {
                if px >= 0 && px < FB_WIDTH as i32 && py >= 0 && py < FB_HEIGHT as i32 {
                    let idx = py as usize * FB_WIDTH + px as usize;
                    let p = self.framebuffer[idx];
                    // 50% darken: shift each channel right by 1
                    self.framebuffer[idx] = (p >> 1) & 0x007F7F7F;
                }
            }
        }
    }

    fn draw_char(&mut self, c: char, x: i32, y: i32, scale: i32, color: u32) {
        let c = c as u32;
        if c < 32 || c > 127 {
            return;
        }
        let glyph = &FONT_8X8[(c - 32) as usize];
        for row in 0..8 {
            for col in 0..8 {
                if glyph[row] & (1 << col) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let px = x + col as i32 * scale + sx;
                            let py = y + row as i32 * scale + sy;
                            if px >= 0 && px < FB_WIDTH as i32 && py >= 0 && py < FB_HEIGHT as i32 {
                                self.framebuffer[py as usize * FB_WIDTH + px as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }
}

// 8x8 bitmap font (printable ASCII 32-127, 96 glyphs).
// Ported directly from AOT_Display.cpp g_font8x8.
#[rustfmt::skip]
const FONT_8X8: [[u8; 8]; 96] = [
    // Space
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],
    // ! " # $ % & ' ( ) * + , - . /
    [0x18,0x3C,0x3C,0x18,0x18,0x00,0x18,0x00],
    [0x36,0x36,0x00,0x00,0x00,0x00,0x00,0x00],
    [0x36,0x36,0x7F,0x36,0x7F,0x36,0x36,0x00],
    [0x0C,0x3E,0x03,0x1E,0x30,0x1F,0x0C,0x00],
    [0x00,0x63,0x33,0x18,0x0C,0x66,0x63,0x00],
    [0x1C,0x36,0x1C,0x6E,0x3B,0x33,0x6E,0x00],
    [0x06,0x06,0x03,0x00,0x00,0x00,0x00,0x00],
    [0x18,0x0C,0x06,0x06,0x06,0x0C,0x18,0x00],
    [0x06,0x0C,0x18,0x18,0x18,0x0C,0x06,0x00],
    [0x00,0x66,0x3C,0xFF,0x3C,0x66,0x00,0x00],
    [0x00,0x0C,0x0C,0x3F,0x0C,0x0C,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00,0x0C,0x0C,0x06],
    [0x00,0x00,0x00,0x3F,0x00,0x00,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00,0x0C,0x0C,0x00],
    [0x60,0x30,0x18,0x0C,0x06,0x03,0x01,0x00],
    // 0-9
    [0x3E,0x63,0x73,0x7B,0x6F,0x67,0x3E,0x00],
    [0x0C,0x0E,0x0C,0x0C,0x0C,0x0C,0x3F,0x00],
    [0x1E,0x33,0x30,0x1C,0x06,0x33,0x3F,0x00],
    [0x1E,0x33,0x30,0x1C,0x30,0x33,0x1E,0x00],
    [0x38,0x3C,0x36,0x33,0x7F,0x30,0x78,0x00],
    [0x3F,0x03,0x1F,0x30,0x30,0x33,0x1E,0x00],
    [0x1C,0x06,0x03,0x1F,0x33,0x33,0x1E,0x00],
    [0x3F,0x33,0x30,0x18,0x0C,0x0C,0x0C,0x00],
    [0x1E,0x33,0x33,0x1E,0x33,0x33,0x1E,0x00],
    [0x1E,0x33,0x33,0x3E,0x30,0x18,0x0E,0x00],
    // : ; < = > ? @
    [0x00,0x0C,0x0C,0x00,0x00,0x0C,0x0C,0x00],
    [0x00,0x0C,0x0C,0x00,0x00,0x0C,0x0C,0x06],
    [0x18,0x0C,0x06,0x03,0x06,0x0C,0x18,0x00],
    [0x00,0x00,0x3F,0x00,0x00,0x3F,0x00,0x00],
    [0x06,0x0C,0x18,0x30,0x18,0x0C,0x06,0x00],
    [0x1E,0x33,0x30,0x18,0x0C,0x00,0x0C,0x00],
    [0x3E,0x63,0x7B,0x7B,0x7B,0x03,0x1E,0x00],
    // A-Z
    [0x0C,0x1E,0x33,0x33,0x3F,0x33,0x33,0x00],
    [0x3F,0x66,0x66,0x3E,0x66,0x66,0x3F,0x00],
    [0x3C,0x66,0x03,0x03,0x03,0x66,0x3C,0x00],
    [0x1F,0x36,0x66,0x66,0x66,0x36,0x1F,0x00],
    [0x7F,0x46,0x16,0x1E,0x16,0x46,0x7F,0x00],
    [0x7F,0x46,0x16,0x1E,0x16,0x06,0x0F,0x00],
    [0x3C,0x66,0x03,0x03,0x73,0x66,0x7C,0x00],
    [0x33,0x33,0x33,0x3F,0x33,0x33,0x33,0x00],
    [0x1E,0x0C,0x0C,0x0C,0x0C,0x0C,0x1E,0x00],
    [0x78,0x30,0x30,0x30,0x33,0x33,0x1E,0x00],
    [0x67,0x66,0x36,0x1E,0x36,0x66,0x67,0x00],
    [0x0F,0x06,0x06,0x06,0x46,0x66,0x7F,0x00],
    [0x63,0x77,0x7F,0x7F,0x6B,0x63,0x63,0x00],
    [0x63,0x67,0x6F,0x7B,0x73,0x63,0x63,0x00],
    [0x1C,0x36,0x63,0x63,0x63,0x36,0x1C,0x00],
    [0x3F,0x66,0x66,0x3E,0x06,0x06,0x0F,0x00],
    [0x1E,0x33,0x33,0x33,0x3B,0x1E,0x38,0x00],
    [0x3F,0x66,0x66,0x3E,0x36,0x66,0x67,0x00],
    [0x1E,0x33,0x07,0x0E,0x38,0x33,0x1E,0x00],
    [0x3F,0x2D,0x0C,0x0C,0x0C,0x0C,0x1E,0x00],
    [0x33,0x33,0x33,0x33,0x33,0x33,0x3F,0x00],
    [0x33,0x33,0x33,0x33,0x33,0x1E,0x0C,0x00],
    [0x63,0x63,0x63,0x6B,0x7F,0x77,0x63,0x00],
    [0x63,0x63,0x36,0x1C,0x1C,0x36,0x63,0x00],
    [0x33,0x33,0x33,0x1E,0x0C,0x0C,0x1E,0x00],
    [0x7F,0x63,0x31,0x18,0x4C,0x66,0x7F,0x00],
    // [ \ ] ^ _ `
    [0x1E,0x06,0x06,0x06,0x06,0x06,0x1E,0x00],
    [0x03,0x06,0x0C,0x18,0x30,0x60,0x40,0x00],
    [0x1E,0x18,0x18,0x18,0x18,0x18,0x1E,0x00],
    [0x08,0x1C,0x36,0x63,0x00,0x00,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xFF],
    [0x0C,0x0C,0x18,0x00,0x00,0x00,0x00,0x00],
    // a-z
    [0x00,0x00,0x1E,0x30,0x3E,0x33,0x6E,0x00],
    [0x07,0x06,0x06,0x3E,0x66,0x66,0x3B,0x00],
    [0x00,0x00,0x1E,0x33,0x03,0x33,0x1E,0x00],
    [0x38,0x30,0x30,0x3E,0x33,0x33,0x6E,0x00],
    [0x00,0x00,0x1E,0x33,0x3F,0x03,0x1E,0x00],
    [0x1C,0x36,0x06,0x0F,0x06,0x06,0x0F,0x00],
    [0x00,0x00,0x6E,0x33,0x33,0x3E,0x30,0x1F],
    [0x07,0x06,0x36,0x6E,0x66,0x66,0x67,0x00],
    [0x0C,0x00,0x0E,0x0C,0x0C,0x0C,0x1E,0x00],
    [0x30,0x00,0x30,0x30,0x30,0x33,0x33,0x1E],
    [0x07,0x06,0x66,0x36,0x1E,0x36,0x67,0x00],
    [0x0E,0x0C,0x0C,0x0C,0x0C,0x0C,0x1E,0x00],
    [0x00,0x00,0x33,0x7F,0x7F,0x6B,0x63,0x00],
    [0x00,0x00,0x1F,0x33,0x33,0x33,0x33,0x00],
    [0x00,0x00,0x1E,0x33,0x33,0x33,0x1E,0x00],
    [0x00,0x00,0x3B,0x66,0x66,0x3E,0x06,0x0F],
    [0x00,0x00,0x6E,0x33,0x33,0x3E,0x30,0x78],
    [0x00,0x00,0x3B,0x6E,0x66,0x06,0x0F,0x00],
    [0x00,0x00,0x3E,0x03,0x1E,0x30,0x1F,0x00],
    [0x08,0x0C,0x3E,0x0C,0x0C,0x2C,0x18,0x00],
    [0x00,0x00,0x33,0x33,0x33,0x33,0x6E,0x00],
    [0x00,0x00,0x33,0x33,0x33,0x1E,0x0C,0x00],
    [0x00,0x00,0x63,0x6B,0x7F,0x7F,0x36,0x00],
    [0x00,0x00,0x63,0x36,0x1C,0x36,0x63,0x00],
    [0x00,0x00,0x33,0x33,0x33,0x3E,0x30,0x1F],
    [0x00,0x00,0x3F,0x19,0x0C,0x26,0x3F,0x00],
    // { | } ~ DEL
    [0x38,0x0C,0x0C,0x07,0x0C,0x0C,0x38,0x00],
    [0x18,0x18,0x18,0x00,0x18,0x18,0x18,0x00],
    [0x07,0x0C,0x0C,0x38,0x0C,0x0C,0x07,0x00],
    [0x6E,0x3B,0x00,0x00,0x00,0x00,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],
];
