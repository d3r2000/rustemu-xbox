use super::{HleMode, OovpaMatch, XBE_ENTRY, XDK_BUILD};
/// OOVPA hook planting — writes INT3 bytes into the host code buffer at matched addresses.
use crate::xbox::emulator::debug_log;

/// Manual hooks for known-problematic D3D internal functions (hardcoded guest addresses).
/// These prevent infinite loops and crashes from uninitialized sort tables.
pub struct ManualHook {
    pub name: &'static str,
    pub guest_addr: u32,
    pub argc: u8,
    /// Required XBE entry point (0 = any game in the XDK version range).
    pub entry: u32,
}

const SPIDERMAN_ENTRY: u32 = 0x002A_9C38;
const DOOM_ENTRY: u32 = 0x0003_E5AB;
const SHENMUE2_ENTRY: u32 = 0x0013_2489;

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim();
            v == "1"
                || v.eq_ignore_ascii_case("true")
                || v.eq_ignore_ascii_case("yes")
                || v.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn title_probes_enabled() -> bool {
    env_flag("RUSTEMU_TITLE_PROBES")
        || env_flag("RUSTEMU_SPIDEY_TITLE_PROBES")
        || env_flag("RUSTEMU_SPIDEY_PROBES")
        || title_patches_enabled()
}

fn title_patches_enabled() -> bool {
    env_flag("RUSTEMU_TITLE_PATCHES")
        || env_flag("RUSTEMU_SPIDEY_TITLE_PATCHES")
        || env_flag("RUSTEMU_SPIDEY_PATCHES")
}

fn spidey_input_event_probes_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_INPUT_EVENT_PROBES")
        || env_flag("RUSTEMU_SPIDEY_EVENT_PROBES")
        || env_flag("RUSTEMU_SPIDEY_SELECT_EVENT_PROBES")
}

fn spidey_f8580_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_F8580_PROBE") || env_flag("RUSTEMU_SPIDEY_F8580_TRACE")
}

fn spidey_fun29c5a0_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_FUN29C5A0_PROBE") || env_flag("RUSTEMU_SPIDEY_FUN29C5A0_SVSC_CANARY")
}

fn spidey_aot_script_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_AOT_SCRIPT_PROBES") || env_flag("RUSTEMU_SPIDEY_SCRIPT_PROBES")
}

fn spidey_xbs_parse_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_XBS_PARSE_PROBE") || env_flag("RUSTEMU_SPIDEY_XBS_PARSE_VERBOSE")
}

fn spidey_xbs_lifecycle_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_XBS_LIFECYCLE_PROBE")
        || env_flag("RUSTEMU_SPIDEY_XBS_SCHED_PROBE")
        || env_flag("RUSTEMU_SPIDEY_XBS_SCRIPT_PROBE")
}

fn spidey_string_probe_enabled() -> bool {
    env_flag("RUSTEMU_SPIDEY_STRING_PROBE") || env_flag("RUSTEMU_SPIDEY_STRING_TRACE")
}

fn is_spidey_script_probe(name: &str) -> bool {
    name.starts_with("SPIDEY_SCRIPT_51C00")
        || name.starts_with("SPIDEY_SCRIPT_NATIVE_")
        || name.starts_with("SPIDEY_SCRIPT_VM_")
        || name.starts_with("SPIDEY_SCRIPT_EVENT_SLOT_GET")
}

fn is_spidey_input_event_probe(name: &str) -> bool {
    name.starts_with("SPIDEY_SELECT_PRESSED_")
        || name.starts_with("SPIDEY_KEYPRESS_TABLE_")
        || name.starts_with("SPIDEY_EVENT_DISPATCH_")
        || name.starts_with("SPIDEY_EVENT_LISTENER_")
        || name.starts_with("SPIDEY_SIGNAL_")
}

fn is_spidey_f8580_probe(name: &str) -> bool {
    name == "SPIDEY_BOOT_GATE_TAP" || name.starts_with("SPIDEY_F8580_")
}

fn is_spidey_scene_probe(name: &str) -> bool {
    name == "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP"
        || name.starts_with("SPIDEY_F0690_")
        || name.starts_with("SPIDEY_E9D00_")
}

fn manual_hook_mode(name: &str) -> HleMode {
    if name == "XGRPH_AssembleShader" && env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
        return HleMode::Tap;
    }
    if name == "XGRPH_AssembleShader_RET_TAP" && env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
        return HleMode::Tap;
    }
    if name == "XGRPH_Lexer_TAP" && env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
        return HleMode::Tap;
    }
    if name == "XGRPH_Error_TAP" && env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
        return HleMode::Tap;
    }
    if is_spidey_scene_probe(name) {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_F8580_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_FUN29C5A0_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SCENE_F27")
        || name.starts_with("SPIDEY_SCENE_F28")
        || name.starts_with("SPIDEY_SCENE_F29")
        || name.starts_with("SPIDEY_SCENE_ED")
        || name.starts_with("SPIDEY_SCENE_F10")
        || name.starts_with("SPIDEY_SCENE_F11")
        || name.starts_with("SPIDEY_SCENE_F12")
        || name.starts_with("SPIDEY_SCENE_F7D")
        || name.starts_with("SPIDEY_SCENE_F7F")
    {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SCRIPT_51C00") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SCRIPT_NATIVE_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SCRIPT_VM_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_STRING_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_NATIVE_METHOD_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SELECT_PRESSED_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_KEYPRESS_TABLE_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_EVENT_DISPATCH_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_EVENT_LISTENER_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_SIGNAL_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_XBS_PARSE_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_XBS_LIFECYCLE_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_FLAG17F_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_ROOT_WRITE_") {
        return HleMode::Tap;
    }
    if name.starts_with("SPIDEY_TRIGGER_") {
        return HleMode::Tap;
    }
    if name.starts_with("DOOM_RENDER_")
        || (name.starts_with("DOOM_COLUMN_") && env_flag("RUSTEMU_DOOM_COLUMN_PROBE"))
        || (name.starts_with("DOOM_SPAN_") && env_flag("RUSTEMU_DOOM_SPAN_PROBE"))
        || ((name.starts_with("DOOM_WALL_") || name.starts_with("DOOM_MASKED_"))
            && env_flag("RUSTEMU_DOOM_WALL_PROBE"))
    {
        return HleMode::Tap;
    }
    if name == "SPIDEY_SCENE_ADVANCE_FLAG_SET_TAP" {
        return HleMode::Tap;
    }
    if name == "SPIDEY_ORIGIN_MENU_ATTACH_TAP" {
        return HleMode::Tap;
    }
    match name {
        "SPIDEY_FSM_DISPATCH_TAP"
        | "SPIDEY_FRAME_DISPATCH_GATE_TAP"
        | "SPIDEY_BOOT_GATE_TAP"
        | "SPIDEY_BOOT_SEQUENCE_TAP"
        | "SPIDEY_SCENE_WARNING_TAP"
        | "SPIDEY_SCENE_DYNAMIC_TAP"
        | "SPIDEY_SCENE_LEGAL_TAP"
        | "SPIDEY_SCENE_START_TAP"
        | "SPIDEY_SCENE_LOADER_ENTRY_TAP"
        | "SPIDEY_LOADER_GATE1_CALL_TAP"
        | "SPIDEY_LOADER_GATE1_RET_TAP"
        | "SPIDEY_LOADER_GATE2_CALL_TAP"
        | "SPIDEY_LOADER_GATE2_RET_TAP"
        | "SPIDEY_LOADER_SYNC_FALLBACK_TAP"
        | "SPIDEY_LEGAL_GUARD_ENTRY_TAP"
        | "SPIDEY_AFTER_LEGAL_LOAD_TAP"
        | "SPIDEY_AFTER_LEGAL_CHECK_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_LEGALBOX_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_19B10_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_D3D_BATCH_TAP"
        | "SPIDEY_BOOT_EDI_LOOP_BODY_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_LEGAL_CLEANUP_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_MOVIE_CHAIN_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_ACTIVISION_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_LOGO_CALLS_TAP"
        | "SPIDEY_BOOT_EDI_PRE_START_BRANCH_TAP"
        | "SPIDEY_BOOT_EDI_PRE_START_LOAD_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_START_GUARD_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_SETTER_BATCH_TAP"
        | "SPIDEY_BOOT_EDI_AFTER_E9140_TAP"
        | "SPIDEY_MOVIE_HELPER_ENTRY_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_EA580_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_STRCMP1_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_STRCMP2_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_2B4B20_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_AUDIO_INIT_TAP"
        | "SPIDEY_MOVIE_HELPER_BEFORE_COPY_SAVE_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_STRING_COPY_TAP"
        | "SPIDEY_MOVIE_HELPER_BEFORE_UPDATE_TAP"
        | "SPIDEY_MOVIE_HELPER_AFTER_UPDATE_TAP"
        | "SPIDEY_MOVIE_HELPER_BEFORE_RET_TAP"
        | "SPIDEY_EA580_ENTRY_TAP"
        | "SPIDEY_EA580_RUNTIME_PATH_TAP"
        | "SPIDEY_EA580_BEFORE_STRCMP_TAP"
        | "SPIDEY_EA580_AFTER_STRCMP_TAP"
        | "SPIDEY_EA580_BEFORE_RET_TAP"
        | "SPIDEY_EA580_INNER_ENTRY_TAP"
        | "SPIDEY_EA580_INNER_BEFORE_POP_EDI_TAP"
        | "SPIDEY_EA580_INNER_AFTER_POP_EDI_TAP"
        | "SPIDEY_STRCMP_ENTRY_TAP"
        | "SPIDEY_STRCMP_FALLBACK_JMP_TAP"
        | "SPIDEY_STRCMP_FAST_PATH_TAP"
        | "SPIDEY_STRCMP_AFTER_PUSH_EDI_TAP"
        | "SPIDEY_STRCMP_BEFORE_POP_EDI_TAP"
        | "SPIDEY_STRCMP_AFTER_POP_EDI_TAP"
        | "SPIDEY_STRCMP_BEFORE_RET_TAP"
        | "SPIDEY_STRCMP_FB_ENTRY_TAP"
        | "SPIDEY_STRCMP_FB_AFTER_PUSH_EDI_TAP"
        | "SPIDEY_STRCMP_FB_BEFORE_POP_EBX_TAP"
        | "SPIDEY_STRCMP_FB_BEFORE_POP_ESI_TAP"
        | "SPIDEY_STRCMP_FB_BEFORE_POP_EDI_TAP"
        | "SPIDEY_STRCMP_FB_AFTER_POP_EDI_TAP"
        | "SPIDEY_STRCMP_FB_BEFORE_RET_TAP"
        | "SPIDEY_LEGALBOX_LOADER_TAP"
        | "SPIDEY_LEGALBOX_CACHE_RET_TAP"
        | "SPIDEY_LEGALBOX_LOAD_RET_TAP"
        | "SPIDEY_REGISTER_CALL1_TAP"
        | "SPIDEY_REGISTER_CALL2_TAP"
        | "SPIDEY_FSM_ADVANCE_ENTRY_TAP"
        | "SPIDEY_FSM_READY_STAGE_TAP"
        | "SPIDEY_SCENE_RENDER_ENABLE_WRITE_TAP"
        | "SPIDEY_SCENE_F7D30_ENTRY_TAP"
        | "SPIDEY_SCENE_F7EC0_ENTRY_TAP"
        | "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP"
        | "SPIDEY_SCENE_DD130_CALL_TAP"
        | "SPIDEY_SCENE_AFTER_DD130_TAP"
        | "SPIDEY_DD130_ENTRY_TAP"
        | "SPIDEY_DD130_AFTER_FIRST_STORE_TAP"
        | "SPIDEY_DD130_AFTER_SECOND_STORE_TAP"
        | "SPIDEY_DD130_RET_TAP"
        | "SPIDEY_SCENE_DIRTY_WRITE_TAP"
        | "SPIDEY_XGRAPH_DIRTY_PROMOTE_TAP"
        | "SPIDEY_FSM_WRITE_1_TAP"
        | "SPIDEY_FSM_WRITE_2_TAP"
        | "SPIDEY_GAMEMAIN_FRAME_CTOR_CALL_TAP"
        | "SPIDEY_FRAME_CTOR_ENTRY_TAP"
        | "SPIDEY_FRAME_BODY_ENTRY_TAP"
        | "SPIDEY_FRAME_BODY_RET_TAP"
        | "SPIDEY_FRAME_STORE_TAP"
        | "SPIDEY_FRAME_STORE_ZERO_TAP"
        | "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP"
        | "SPIDEY_FRAME_RENDER_PREP_TAP"
        | "SPIDEY_FRAME_RENDER_CALL_TAP"
        | "SPIDEY_FRAME_RENDER_ENTRY_TAP"
        | "SPIDEY_FRAME_RENDER_RET_TAP"
        | "SPIDEY_RENDER_BODY_READY_RET_TAP"
        | "SPIDEY_RENDER_BODY_SCENE_CALL_TAP"
        | "SPIDEY_RENDER_BODY_AFTER_SCENE_TAP"
        | "SPIDEY_RENDER_BODY_POST_SCENE_TAP"
        | "SPIDEY_RENDER_BODY_TAIL_CALL_TAP"
        | "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP"
        | "SPIDEY_INPUT_CB_ENTRY_TAP"
        | "SPIDEY_INPUT_CB_XINPUT_CALL_TAP"
        | "SPIDEY_SELECT_PRESSED_REGISTER_TAP"
        | "SPIDEY_SELECT_PRESSED_CHECK_TAP"
        | "SPIDEY_SELECT_PRESSED_EMIT_TAP"
        | "SPIDEY_SELECT_PRESSED_MISS_TAP"
        | "SPIDEY_ACTION_GATE_ENTRY_TAP"
        | "SPIDEY_ACTION_GATE_SELECTED_TAP"
        | "SPIDEY_ACTION_GATE_FALSE_TAP"
        | "SPIDEY_ACTION_GATE_TRUE_TAP"
        | "SPIDEY_VECTOR_GROW_TAP"
        | "DOOM_GAMESTATE_OUTER_ENTRY_TAP"
        | "DOOM_GAMESTATE_INNER_ENTRY_TAP"
        | "DOOM_ALLOC_RET_TAP"
        | "XGRPH_ParseDocument"
        | "XGRPH_ParseHelper"
        | "DPC_BODY_PROBE_ENTRY" => HleMode::Tap,
        // 2026-04-24 EOS: tried TAP for XGRPH_CreateTexture and it
        // caused an infinite loop — 4.2 million calls, guest globals
        // got zeroed (App=Scene=State=Engine=0), worker spun in VEH
        // (92% time non-JIT, guest_pc=0xFFFFFFFF). The guest body has
        // NV2A / pushbuffer dependencies we don't handle, which
        // cascade into RET-to-null on the TAP boundary.
        //
        // Keeping HLE. Next-session fix: plant a narrower TAP at the
        // Register call sites 0x00299325 / 0x00299359 themselves so
        // Register runs in isolation without the surrounding XGRPH
        // body, OR pre-compute what Register would have done inside
        // our XGRPH HLE (requires knowing the pBase arg, which we
        // don't currently have).
        _ => HleMode::Hle,
    }
}

pub static MANUAL_HOOKS: &[ManualHook] = &[
    // ---- Spider-Man (entry=0x002A9C38) manual hooks ----
    // D3D_RenderStateSort/Sort2/Init REMOVED — game-internal D3D functions not in Cxbx-R SymbolCache.
    // These are NOT SDK boundary functions. Hooking them breaks the "HLE the OS, not the game" rule.
    // ManualHook { name: "D3D_RenderStateSort",  guest_addr: 0x0029_5F50, argc: 2, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "D3D_RenderStateSort2", guest_addr: 0x0029_61D0, argc: 1, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "D3D_RenderStateInit",  guest_addr: 0x0029_D6C0, argc: 0, entry: SPIDERMAN_ENTRY },
    // --- Frame-body helper tripwires (2026-04-21, next-session kickoff) ---
    //
    // Frame body (sub_0xD67E0) has ONE ret path at 0x000D69E4 that returns
    // non-zero `this`. Our FRAME-PTR-TICK probe proved 0x004BC630 never
    // populates, therefore Frame body is called but HANGS before reaching
    // its ret. Engine ptr at 0x004BC614 DOES populate, proving execution
    // survives the LOD-allocation chain deep in Frame body. So the hang
    // is AFTER that point.
    //
    // The disasm shows 4 sequential `call` instructions immediately after
    // the 73KB alloc loop: sub_0xD60E0, 0xD63B0, 0xD6440, 0xD64D0. These
    // are the prime suspects. Stub them as log-on-entry + return 0 so we
    // can see WHICH fires LAST before the game's render loop locks into
    // its keep-alive state. Whichever is silent is the candidate hang
    // site.
    //
    // Safe to stub: Frame body continues past any single helper failure
    // (non-fatal return-value checks were confirmed in the surrounding
    // disasm). Worst case: Frame body completes with some state missing,
    // but we've already confirmed the non-body-completion is what's
    // blocking 0x004BC630 from populating.
    // All 10 Frame-body probe hooks DISABLED 2026-04-21 — collectively
    // they broke the game's init chain worse than no probes. Each
    // `return 0` stub replaced a real function and cascading dependency
    // failures reduced Swap count from 42K to 4. Data captured banked:
    //   - Frame body reaches HELPER_A/B/C/D
    //   - Frame body reaches 0xD6902 area (post-helpers)
    //   - Engine ptr at 0x4BC614 writes at frame ~426 (LOD body completes)
    //   - With 0xDD470 stubbed, Engine.Init2 (0xF23C0) fires
    //   - But Frame body's RET at 0x000D69E4 STILL never reached
    // Keeping the hooks in source as comments for future re-enablement.
    //
    // ManualHook { name: "FRAME_HELPER_A",       guest_addr: 0x000D_60E0, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "FRAME_HELPER_B",       guest_addr: 0x000D_63B0, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "FRAME_HELPER_C",       guest_addr: 0x000D_6440, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "FRAME_HELPER_D",       guest_addr: 0x000D_64D0, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "ENGINE_INIT2_PROBE",   guest_addr: 0x000F_23C0, argc: 1, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "F_EFDC0_PROBE",        guest_addr: 0x000E_FDC0, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "F_1C8C0_PROBE",        guest_addr: 0x0001_C8C0, argc: 2, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "LOD_4SETTER_PROBE",    guest_addr: 0x000D_D470, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "LOD_SETTER_2A3C20",    guest_addr: 0x002A_3C20, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "FRAME_TAIL_28F980",    guest_addr: 0x0028_F980, argc: 0, entry: SPIDERMAN_ENTRY },
    //
    // 0x0029C5A0 seeds Spider-Man's signed D3D constant-register globals
    // and should later issue four SetVertexShaderConstant calls for c[-96..-93].
    // These are TAP-only breadcrumbs, enabled by RUSTEMU_SPIDEY_FUN29C5A0_PROBE.
    ManualHook {
        name: "SPIDEY_FUN29C5A0_PARENT_CALLSITE_TAP",
        guest_addr: 0x002A_0B1B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_PARENT_ENTRY_TAP",
        guest_addr: 0x0029_CB70,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_PARENT_C5A0_CALL_TAP",
        guest_addr: 0x0029_CB93,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_PARENT_C5A0_RET_TAP",
        guest_addr: 0x0029_CB98,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_ENTRY_TAP",
        guest_addr: 0x0029_C5A0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C96_CALL_TAP",
        guest_addr: 0x0029_C734,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C96_RET_TAP",
        guest_addr: 0x0029_C739,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C95_CALL_TAP",
        guest_addr: 0x0029_C7C3,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C95_RET_TAP",
        guest_addr: 0x0029_C7C8,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C94_CALL_TAP",
        guest_addr: 0x0029_C839,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C94_RET_TAP",
        guest_addr: 0x0029_C83E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C93_CALL_TAP",
        guest_addr: 0x0029_C86C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FUN29C5A0_C93_RET_TAP",
        guest_addr: 0x0029_C871,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // STRATEGIC PIVOT (2026-04-21, late session):
    // Abandoning the Frame-subsystem investigation. The probe-cascade
    // technique proved to be a lobotomy — each stubbed function starved
    // its downstream callers, and we ended up at Swap=4 (from the normal
    // 42K). The hang is almost certainly an async-poll waiting for an
    // XDK callback (VBlank, file I/O completion, DirectSound) we don't
    // signal — solving that requires subsystem work beyond today's scope.
    //
    // Instead: the 42K-Swap state IS valid pushbuffer traffic. Pivoting
    // to wire those `SET_SURFACE_*`, `SET_CLEAR_*`, etc. method writes
    // into the D3D11 backend's Clear + Swap path so the game's real
    // per-frame state produces VISIBLE output — even if it's just the
    // Clear color changing. That proves the full host-to-guest graphics
    // pipeline is functional.

    // Boot FSM TAP sweep (2026-04-24): observe, then pass through.
    // These must stay TAP-only. Replacing any of these game functions masks
    // the native state machine we are trying to trace.
    ManualHook {
        name: "SPIDEY_FSM_DISPATCH_TAP",
        guest_addr: 0x000D_D500,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7D30_ENTRY_TAP",
        guest_addr: 0x000F_7D30,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7EC0_ENTRY_TAP",
        guest_addr: 0x000F_7EC0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2740_ENTRY_TAP",
        guest_addr: 0x000F_2740,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F1080_FINALIZE_ENTRY_TAP",
        guest_addr: 0x000F_1080,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F1057_SET17F_TAP",
        guest_addr: 0x000F_1057,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F10AA_EARLY_RET_TEST_TAP",
        guest_addr: 0x000F_10AA,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F10B7_SET9_ONE_TAP",
        guest_addr: 0x000F_10B7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F10C0_SETA_ONE_TAP",
        guest_addr: 0x000F_10C0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F10C4_18B_TEST_TAP",
        guest_addr: 0x000F_10C4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F10CC_18B_FALSE_TAP",
        guest_addr: 0x000F_10CC,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F111F_COPY_STATE_TAP",
        guest_addr: 0x000F_111F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F115F_ROOT_CHECK_TAP",
        guest_addr: 0x000F_115F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F116C_ROOT_NULL_TAP",
        guest_addr: 0x000F_116C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F11B9_READY_PATH_TAP",
        guest_addr: 0x000F_11B9,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F11CA_SET18B_TAP",
        guest_addr: 0x000F_11CA,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F11DC_INPUT_CHECK_TAP",
        guest_addr: 0x000F_11DC,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F11FD_SET2F0_TAP",
        guest_addr: 0x000F_11FD,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F121B_ABORT_TAP",
        guest_addr: 0x000F_121B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F121E_FINAL_RET_TAP",
        guest_addr: 0x000F_121E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2819_SET8_ZERO_TAP",
        guest_addr: 0x000F_2819,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2801_PRIMARY_TEST_TAP",
        guest_addr: 0x000F_2801,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2812_PRIMARY_ARM_TAP",
        guest_addr: 0x000F_2812,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F284A_SET9_ONE_TAP",
        guest_addr: 0x000F_284A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2853_SECONDARY_TEST_TAP",
        guest_addr: 0x000F_2853,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2864_SET8_ONE_TAP",
        guest_addr: 0x000F_2864,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2885_FLAG_TEST_TAP",
        guest_addr: 0x000F_2885,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F289B_17F_BRANCH_TAP",
        guest_addr: 0x000F_289B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F28A1_A_TEST_TAP",
        guest_addr: 0x000F_28A1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F28AE_ED320_CALL_TAP",
        guest_addr: 0x000F_28AE,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F28C5_SECONDARY_ELSE_TAP",
        guest_addr: 0x000F_28C5,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F28D6_B4_TEST_TAP",
        guest_addr: 0x000F_28D6,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F28E9_STATE_TEST_TAP",
        guest_addr: 0x000F_28E9,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2901_GLOBAL_TEST_TAP",
        guest_addr: 0x000F_2901,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2922_CLEAR9_TAP",
        guest_addr: 0x000F_2922,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2928_ADVANCE_PATH_TAP",
        guest_addr: 0x000F_2928,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2930_ED320_CALL_TAP",
        guest_addr: 0x000F_2930,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F2935_F1080_CALL_TAP",
        guest_addr: 0x000F_2935,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F293C_EXIT_TAP",
        guest_addr: 0x000F_293C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ED320_ENTRY_TAP",
        guest_addr: 0x000E_D320,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ED346_18B_TEST_TAP",
        guest_addr: 0x000E_D346,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ED3B8_STATE_RECHECK_TAP",
        guest_addr: 0x000E_D3B8,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ED3C4_ADVANCE_TAP",
        guest_addr: 0x000E_D3C4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ED43F_RET_TAP",
        guest_addr: 0x000E_D43F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7D10_ADVANCE_TAP",
        guest_addr: 0x000F_7D10,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7F2A_FOCUS_DRAIN_TAP",
        guest_addr: 0x000F_7F2A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7F38_CONFIRM_PATH_TAP",
        guest_addr: 0x000F_7F38,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_F7FC1_ED320_TAP",
        guest_addr: 0x000F_7FC1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_ENTRY_TAP",
        guest_addr: 0x0005_1C00,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_LOOP_OPCODE_TAP",
        guest_addr: 0x0005_1C54,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_BRANCH_HOT_TAP",
        guest_addr: 0x0005_1C80,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_RET_TAP",
        guest_addr: 0x0005_2859,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_OP2A_REGISTER_LOCAL_TAP",
        guest_addr: 0x0005_2B78,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_OP2B_REGISTER_GLOBAL_TAP",
        guest_addr: 0x0005_2B88,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_OP2C_REGISTER_LOCAL_ONCE_TAP",
        guest_addr: 0x0005_2B91,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_51C00_OP2D_REGISTER_GLOBAL_ONCE_TAP",
        guest_addr: 0x0005_2BA1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_NATIVE_CALL_TAP",
        guest_addr: 0x0005_0D30,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_NATIVE_WAIT_TAP",
        guest_addr: 0x0005_0D4E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_NATIVE_RETURN_TEST_TAP",
        guest_addr: 0x0005_0D4A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_NATIVE_PC_WRITE_TAP",
        guest_addr: 0x0005_0D56,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_NATIVE_DONE_TAP",
        guest_addr: 0x0005_0D64,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_VM_STORE8_TAP",
        guest_addr: 0x0005_2660,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_VM_LOAD8_TAP",
        guest_addr: 0x0005_2754,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCRIPT_EVENT_SLOT_GET_TAP",
        guest_addr: 0x0005_27C0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_GETLINE_ENTRY_TAP",
        guest_addr: 0x0002_77D0,
        argc: 4,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_ALLOC_ENTRY_TAP",
        guest_addr: 0x0004_6F10,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_GROW_ENTRY_TAP",
        guest_addr: 0x0004_6FB0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_APPEND_LITERAL_ENTRY_TAP",
        guest_addr: 0x0004_7150,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_APPEND_OBJECT_ENTRY_TAP",
        guest_addr: 0x0004_72A0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_FROM_CSTR_ENTRY_TAP",
        guest_addr: 0x0004_76A0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_STRING_PATH_JOIN_ENTRY_TAP",
        guest_addr: 0x0004_7A00,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_NATIVE_METHOD_61400_ENTRY_TAP",
        guest_addr: 0x0006_1400,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_NATIVE_METHOD_78F00_ENTRY_TAP",
        guest_addr: 0x0007_8F00,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_NATIVE_METHOD_78F00_RET_EMPTY_TAP",
        guest_addr: 0x0007_8F38,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_NATIVE_METHOD_78F00_RET_DONE_TAP",
        guest_addr: 0x0007_8FEB,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_NATIVE_METHOD_78F00_RET_WAIT_TAP",
        guest_addr: 0x0007_9140,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FLAG17F_SCRIPT_SETTER_TAP",
        guest_addr: 0x0006_6CE0,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FLAG17F_CLEAR_POST_TAP",
        guest_addr: 0x0006_6D0A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FLAG17F_SET_POST_TAP",
        guest_addr: 0x0006_6D1D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_INIT_F0690_ENTRY_TAP",
        guest_addr: 0x000F_0690,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_FIRST_WAIT_RET_TAP",
        guest_addr: 0x000F_0C0F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_FIRST_WAIT_FALSE_TAP",
        guest_addr: 0x000F_0C13,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_AFTER_DELAY_TAP",
        guest_addr: 0x000F_0C29,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_LOOP_WAIT_RET_TAP",
        guest_addr: 0x000F_0C35,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_FINAL_WAIT_RET_TAP",
        guest_addr: 0x000F_0C46,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_AFTER_E9D00_TAP",
        guest_addr: 0x000F_0C4D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_AFTER_SECOND_DELAY_TAP",
        guest_addr: 0x000F_0C56,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_ENTRY_TAP",
        guest_addr: 0x000E_9D00,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_AFTER_FIRST_PARSE_TAP",
        guest_addr: 0x000E_9DED,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_AFTER_FIRST_COMMIT_TAP",
        guest_addr: 0x000E_9EA3,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_AFTER_SECOND_PARSE_TAP",
        guest_addr: 0x000E_9F22,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_AFTER_SECOND_COMMIT_TAP",
        guest_addr: 0x000E_9FD4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_BEFORE_OPTIONAL_ATTACH_TAP",
        guest_addr: 0x000E_A075,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_OPTIONAL_BRANCH_TAP",
        guest_addr: 0x000E_A08B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_E9D00_EPILOGUE_TAP",
        guest_addr: 0x000E_A13E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_DD130_CALL_TAP",
        guest_addr: 0x000F_0DE4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_AFTER_DD130_TAP",
        guest_addr: 0x000F_0DE9,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ORIGIN_MENU_ATTACH_TAP",
        guest_addr: 0x000F_0E79,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_SET25_TAP",
        guest_addr: 0x000F_0F1D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_SET17F_TAP",
        guest_addr: 0x000F_1057,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F0690_RET_TAP",
        guest_addr: 0x000F_107D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_DD130_ENTRY_TAP",
        guest_addr: 0x000D_D130,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_DD130_AFTER_FIRST_STORE_TAP",
        guest_addr: 0x000D_D184,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_DD130_AFTER_SECOND_STORE_TAP",
        guest_addr: 0x000D_D1C9,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_DD130_RET_TAP",
        guest_addr: 0x000D_D1D8,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_DIRTY_WRITE_TAP",
        guest_addr: 0x000F_06C0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XGRAPH_DIRTY_PROMOTE_TAP",
        guest_addr: 0x0003_AB60,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_DISPATCH_GATE_TAP",
        guest_addr: 0x002A_29E0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // FrameContext constructor reachability sweep. TAP-only: lets the game
    // execute natively while logging the exact point where [0x004BC630] fails
    // to become non-zero.
    ManualHook {
        name: "SPIDEY_GAMEMAIN_FRAME_CTOR_CALL_TAP",
        guest_addr: 0x002A_53FA,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_CTOR_ENTRY_TAP",
        guest_addr: 0x002A_4A50,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_BODY_ENTRY_TAP",
        guest_addr: 0x000D_67E0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_BODY_RET_TAP",
        guest_addr: 0x000D_69E4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_STORE_TAP",
        guest_addr: 0x002A_4A87,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_STORE_ZERO_TAP",
        guest_addr: 0x002A_4AA1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Post-"Please Wait" frame dispatcher branch probes. TAP-only: these
    // tell us whether frame+0x20 is skipping the real render function, or
    // whether 0xF21F0 is entered and exits without submitting draw calls.
    ManualHook {
        name: "SPIDEY_FRAME_COUNTDOWN_CHECK_TAP",
        guest_addr: 0x000D_6C40,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_RENDER_PREP_TAP",
        guest_addr: 0x000D_6C5F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_RENDER_CALL_TAP",
        guest_addr: 0x000D_6C67,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_RENDER_ENTRY_TAP",
        guest_addr: 0x000F_21F0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FRAME_RENDER_RET_TAP",
        guest_addr: 0x000F_237C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_RENDER_BODY_READY_RET_TAP",
        guest_addr: 0x000F_22D6,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_RENDER_BODY_SCENE_CALL_TAP",
        guest_addr: 0x000F_22E3,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_RENDER_BODY_AFTER_SCENE_TAP",
        guest_addr: 0x000F_22E8,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_RENDER_BODY_POST_SCENE_TAP",
        guest_addr: 0x000F_22F4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_RENDER_BODY_TAIL_CALL_TAP",
        guest_addr: 0x000F_2372,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_BOOT_GATE_TAP",
        guest_addr: 0x000F_8580,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // xgraph+0x134 vtable +0x308 resolves to a tiny `xor al, al; ret` body.
    // Do not hook it by default: it is a bad-prologue thunk and changes the
    // frontend timing path. Re-enable only as a focused experiment.
    // ManualHook {
    //     name: "SPIDEY_XGRAPH_READY_CB_HLE",
    //     guest_addr: 0x0015_C2A0,
    //     argc: 0,
    //     entry: SPIDERMAN_ENTRY,
    // },
    ManualHook {
        name: "SPIDEY_F8580_ROOT_CB_RET_TAP",
        guest_addr: 0x000F_85A4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_MAIN_GATE_TAP",
        guest_addr: 0x000F_85C9,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_RESET_PATH_TAP",
        guest_addr: 0x000F_8606,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_NORMAL_PATH_TAP",
        guest_addr: 0x000F_8618,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_LIST_BRANCH_TAP",
        guest_addr: 0x000F_8642,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_ADVANCE_CALL_TAP",
        guest_addr: 0x000F_86A7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_POST_ADVANCE_TAP",
        guest_addr: 0x000F_86C0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_FINAL_CB_TAP",
        guest_addr: 0x000F_86E8,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_F8580_RET_TAP",
        guest_addr: 0x000F_86FC,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_ADVANCE_FLAG_SET_TAP",
        guest_addr: 0x000D_D600,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_BOOT_SEQUENCE_TAP",
        guest_addr: 0x000F_23C0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Four scene-loader callsite TAPs. These are mid-function hooks on the
    // `call 0x1C8C0` instructions, so the stack log reports the already-pushed
    // scene/stash arguments rather than a normal function-entry return address.
    ManualHook {
        name: "SPIDEY_SCENE_WARNING_TAP",
        guest_addr: 0x000E_FEEA,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_DYNAMIC_TAP",
        guest_addr: 0x000F_0BE4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_LEGAL_TAP",
        guest_addr: 0x000F_2402,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_START_TAP",
        guest_addr: 0x000F_253B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_LOADER_ENTRY_TAP",
        guest_addr: 0x0001_C8C0,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    // One-shot/sampled diagnostic: the boot loop calls the update dispatcher
    // while waiting at the start scene. We keep only the entry TAP so the
    // callback list can be inspected without trapping after every callback.
    ManualHook {
        name: "SPIDEY_UPDATE_DISPATCH_ENTRY_TAP",
        guest_addr: 0x0003_AC20,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Input callback called through the update list's vtable+0x20 slot. These
    // TAPs are diagnostic only: they verify whether the callback reaches the
    // SDK poll path organically, without forcing controller state.
    ManualHook {
        name: "SPIDEY_INPUT_CB_ENTRY_TAP",
        guest_addr: 0x0001_5430,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_INPUT_CB_XINPUT_CALL_TAP",
        guest_addr: 0x0001_5459,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // XBS script_controller::SELECT_PRESSED bridge. These are TAP-only
    // diagnostics around the controller update branch that should emit event
    // id 0x1A after the native input vtable accepts button/action 0x15.
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_REGISTER_TAP",
        guest_addr: 0x0007_47EF,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_CHECK_TAP",
        guest_addr: 0x0007_3CB5,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_CB10_RET_TAP",
        guest_addr: 0x0007_3CC0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_CB1C_RET_TAP",
        guest_addr: 0x0007_3CC6,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_PRE_EMIT_TAP",
        guest_addr: 0x0007_3CD1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_EMIT_TAP",
        guest_addr: 0x0007_3CD3,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SELECT_PRESSED_MISS_TAP",
        guest_addr: 0x0007_3CD7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // wait_keypress/key dispatch table. 0x73560 marks a {key, pending_flag}
    // slot at 0x003DCEC0 + EAX*8; 0x73170 drains pending slots and calls the
    // listener vtable with the slot key.
    ManualHook {
        name: "SPIDEY_KEYPRESS_TABLE_MARK_ENTRY_TAP",
        guest_addr: 0x0007_356E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_KEYPRESS_TABLE_DRAIN_ENTRY_TAP",
        guest_addr: 0x0007_3170,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // (Removed: SPIDEY_SELECT_FCOMP_TAP at 0x00073CC6. It was rejected by the
    // prologue validator — the byte sequence D8 1D 04 75 38 00 is `fcomp [imm32]`,
    // an x87 instruction the validator doesn't recognize. Worse, the host code at
    // that offset was already INT3 (existing hook overlap). The CB1C_RET_TAP already
    // captures ST(0) just before this fcomp via x87_st0_from_context — that's the
    // legitimate diagnostic source. See docs/gate1-select-emit-investigation.md.)
    // script/signaller event dispatch. SELECT_PRESSED reaches this through
    // the embedded frame-scene dispatcher at frame_scene+0x324. These TAPs
    // are read-only diagnostics for the listener table and event slot.
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_ENTRY_TAP",
        guest_addr: 0x0003_5160,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_BUILD_RET_TAP",
        guest_addr: 0x0003_5175,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_LOOKUP_TAP",
        guest_addr: 0x0003_5183,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_LISTENER_CALL_TAP",
        guest_addr: 0x0003_518A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_LISTENER_RET_TAP",
        guest_addr: 0x0003_5192,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_DISPATCH_POST_LISTENER_TAP",
        guest_addr: 0x0003_519B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_LISTENER_REGISTER_TAP",
        guest_addr: 0x0004_6100,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_EVENT_LISTENER_MATCH_ENTRY_TAP",
        guest_addr: 0x0004_6200,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SIGNAL_WRAPPER_FIRE_CALL_TAP",
        guest_addr: 0x0004_60F0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SIGNAL_SECONDARY_FIRE_CALL_TAP",
        guest_addr: 0x0004_622D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Title/menu action gate. The boot loop calls this with action 0x14 and
    // waits here for a confirm/start style action before advancing.
    ManualHook {
        name: "SPIDEY_ACTION_GATE_ENTRY_TAP",
        guest_addr: 0x000F_7DA0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ACTION_GATE_SELECTED_TAP",
        guest_addr: 0x000F_7E20,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ACTION_GATE_FALSE_TAP",
        guest_addr: 0x000F_7E7E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ACTION_GATE_TRUE_TAP",
        guest_addr: 0x000F_7E8A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Origin_Z trigger/vtable path. These taps are read-only probes for the
    // active locomotion/trigger classes; do not replace the methods or force
    // their result flags.
    ManualHook {
        name: "SPIDEY_TRIGGER_FACTORY_TAP",
        guest_addr: 0x0020_2980,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_CTOR_TAP",
        guest_addr: 0x0020_27A0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_PARENT_CHECK_TAP",
        guest_addr: 0x0021_F530,
        argc: 5,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_CRAWL_GATE_TAP",
        guest_addr: 0x0021_0BE0,
        argc: 5,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_CRAWL_UPDATE_TAP",
        guest_addr: 0x0021_0D50,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_GATE_TAP",
        guest_addr: 0x0020_1B60,
        argc: 5,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_TRIGGER_VIS_HELPER_TAP",
        guest_addr: 0x001F_9870,
        argc: 5,
        entry: SPIDERMAN_ENTRY,
    },
    // Post-start menu allocator probe. The game currently reaches MENU.xbs
    // then calls this vector-grow helper with a bogus 0x0CB00000 allocation.
    // TAP keeps native execution intact while logging the owner object fields
    // that produced the size.
    ManualHook {
        name: "SPIDEY_VECTOR_GROW_TAP",
        guest_addr: 0x002A_1C90,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    // XBS archive parser probes. `sub_00025110` opens MENU.xbs /
    // ORIGIN_Z.xbs, validates the 0x5AFE header, then allocates and
    // registers archive records. These taps are read-only snapshots of
    // native progress after the file boundary.
    ManualHook {
        name: "SPIDEY_XBS_PARSE_ENTRY_TAP",
        guest_addr: 0x0002_5110,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_OPEN_RET_TAP",
        guest_addr: 0x0002_51B6,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_HEADER_READ_RET_TAP",
        guest_addr: 0x0002_51CC,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_VALID_HEADER_TAP",
        guest_addr: 0x0002_5241,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_AFTER_HEADER_LOG_TAP",
        guest_addr: 0x0002_5250,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_SECTION_A_READ_RET_TAP",
        guest_addr: 0x0002_5301,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_SECTION_B_ALLOC_RET_TAP",
        guest_addr: 0x0002_534B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_SECTION_B_READ_RET_TAP",
        guest_addr: 0x0002_53BC,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_RECORDS_ALLOC_RET_TAP",
        guest_addr: 0x0002_540E,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_RECORDS_READ_RET_TAP",
        guest_addr: 0x0002_5466,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_RECORD_ALLOC_RET_TAP",
        guest_addr: 0x0002_54C4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_RECORD_REGISTER_RET_TAP",
        guest_addr: 0x0002_551F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_RECORD_LOOP_DONE_TAP",
        guest_addr: 0x0002_552D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_PARSE_FOOTER_READ_RET_TAP",
        guest_addr: 0x0002_5556,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Scene script lifecycle probes. `0x00130680` installs the active
    // script manager globals from scene/root +0x1A0/+0x1AC; `0x00132430`
    // tears them down. These probes are read-only and exist to diagnose why
    // ORIGIN_Z parses successfully but reaches the loader with an empty
    // script queue and no `0x2A..0x2D` listener-registration opcodes.
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCENE_START_CALL_TAP",
        guest_addr: 0x0013_6B6C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCENE_START_ENTRY_TAP",
        guest_addr: 0x0013_0680,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_GLOBALS_SET_TAP",
        guest_addr: 0x0013_0BD7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_GLOBALS_READY_TAP",
        guest_addr: 0x0013_0BEB,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_NUMBERS_INIT_INDEX_RET_TAP",
        guest_addr: 0x0013_0C02,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_NUMBERS_INIT_ENQUEUE_RET_TAP",
        guest_addr: 0x0013_0C0F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_CALL_TAP",
        guest_addr: 0x0013_0C33,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_ENTRY_TAP",
        guest_addr: 0x0012_4D90,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_SCAN_TAP",
        guest_addr: 0x0012_4DD2,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_SKIP_TAP",
        guest_addr: 0x0012_4DDD,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_ACCEPT_TAP",
        guest_addr: 0x0012_4DF7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_LOOKUP_RET_TAP",
        guest_addr: 0x0012_4E5F,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_ENQUEUE_CALL_TAP",
        guest_addr: 0x0012_4E70,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_ENQUEUE_RET_TAP",
        guest_addr: 0x0012_4E75,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_RUN_CALL_TAP",
        guest_addr: 0x0012_4E8B,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_AUTO_PRELOAD_RUN_RET_TAP",
        guest_addr: 0x0012_4E90,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_RUN_ENTRY_TAP",
        guest_addr: 0x0004_BE70,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_VM_CALL_TAP",
        guest_addr: 0x0004_BE88,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_VM_RET_TAP",
        guest_addr: 0x0004_BE8D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_NATIVE_50D30_ENTRY_TAP",
        guest_addr: 0x0005_0D30,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_NATIVE_50D30_RET_TAP",
        guest_addr: 0x0005_0D4A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_PRELOAD_APPEND_ENTRY_TAP",
        guest_addr: 0x0012_BFA0,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCENE_START_RET_TAP",
        guest_addr: 0x0013_6B71,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCENE_DESTROY_ENTRY_TAP",
        guest_addr: 0x0013_2430,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_XBS_LIFECYCLE_SCRIPT_GLOBALS_CLEAR_TAP",
        guest_addr: 0x0013_3283,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Candidate native writers for object/root +0x1A4/+0x1A8. TAP-only:
    // these identify which constructor actually populates the menu/XGraph
    // slots after MENU.xbs is opened.
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_367D4_TAP",
        guest_addr: 0x0003_67D4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_B81BF_TAP",
        guest_addr: 0x000B_81BF,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_1018CB_TAP",
        guest_addr: 0x0010_18CB,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_12CBDD_TAP",
        guest_addr: 0x0012_CBDD,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_130C7D_TAP",
        guest_addr: 0x0013_0C7D,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_130CCE_TAP",
        guest_addr: 0x0013_0CCE,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_138CD0_TAP",
        guest_addr: 0x0013_8CD0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_ROOT_WRITE_138F8A_TAP",
        guest_addr: 0x0013_8F8A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LOADER_GATE1_CALL_TAP",
        guest_addr: 0x0001_C8F1,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LOADER_GATE1_RET_TAP",
        guest_addr: 0x0001_C8F6,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LOADER_GATE2_CALL_TAP",
        guest_addr: 0x0001_C925,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LOADER_GATE2_RET_TAP",
        guest_addr: 0x0001_C92A,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LOADER_SYNC_FALLBACK_TAP",
        guest_addr: 0x0001_CA22,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LEGAL_GUARD_ENTRY_TAP",
        guest_addr: 0x0001_AE60,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_AFTER_LEGAL_LOAD_TAP",
        guest_addr: 0x000F_2407,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_AFTER_LEGAL_CHECK_TAP",
        guest_addr: 0x000F_2414,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // 2026-04-24: Legalbox loader entry probe. `sub_000EC0A0` is called
    // from `sub_000F23C0` (boot sequence) at 0x000F2416, between the
    // bonus\legal scene-ctor call and the bonus\start scene-ctor call.
    // This function is responsible for opening `legalbox.pig` (or a
    // localized variant), but our log shows no .pig file ever opens.
    //
    // Three outcome targets for this probe:
    //   - NEVER FIRES → sub_000F23C0 short-circuits before 0x000F2416
    //     (likely an SEH unwind through an earlier call)
    //   - FIRES, [0x4C20E4] == 0 → localization manager never init'd
    //     (hunt Xbox language-query path or missing static initializer)
    //   - FIRES, [0x4C20E4] != 0 → downstream skip in sub_0x001F7B0
    //     (legalbox cache-already-loaded check wrongly returns true)
    ManualHook {
        name: "SPIDEY_LEGALBOX_LOADER_TAP",
        guest_addr: 0x000E_C0A0,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // 2026-04-24 final: dual-probe to identify which call inside
    // sub_000EC0A0 causes the legalbox.pig skip:
    // (A) sub_0x001F7B0 at 0x000EC19F — "alreadyLoaded?" check
    //     Probe right after return at 0x000EC1A4; eax holds result
    // (B) sub_0x001F640 at 0x000EC1F2 — actualLoad call
    //     Probe right after return at 0x000EC1F7; eax holds result
    ManualHook {
        name: "SPIDEY_LEGALBOX_CACHE_RET_TAP",
        guest_addr: 0x000E_C1A4,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_LEGALBOX_LOAD_RET_TAP",
        guest_addr: 0x000E_C1F7,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // 2026-04-24 FSM ADVANCE PROBE: agent A04 found that FSM at 0x00726690
    // is written by sub_000F7450 — at 0x000F751C (writes ECX value) and
    // 0x000F75CD (writes literal 2). If sub_000F7450 never fires, FSM
    // stays at 0 and we know the boot chain doesn't reach it. If it DOES
    // fire but FSM stays 0, our reader is wrong. If FSM transitions, the
    // legal screen render path should fire.
    ManualHook {
        name: "SPIDEY_FSM_ADVANCE_ENTRY_TAP",
        guest_addr: 0x000F_7450,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FSM_WRITE_1_TAP",
        guest_addr: 0x000F_751C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FSM_WRITE_2_TAP",
        guest_addr: 0x000F_75CD,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_FSM_READY_STAGE_TAP",
        guest_addr: 0x000F_763C,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_SCENE_RENDER_ENABLE_WRITE_TAP",
        guest_addr: 0x000F_7683,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // 2026-04-24: Narrow TAP probes at the two Register call sites inside
    // the XGRPH enclosing function. The enclosing body has NV2A/pushbuffer
    // dependencies we don't handle (see manual_hook_mode comment above for
    // the 4.2M-loop crash) — but we DO want Register itself (0x002F33F0,
    // HLE-override) to fire. Tapping the call SITES (not entry, not after)
    // lets us:
    //   1. Log args at PRE-call (CALL1) — record pBase / resource pointer
    //      pushed onto the stack.
    //   2. Log result at POST-call (CALL2) — record EAX after Register
    //      returns. Confirms the HLE-override fired and what it returned.
    // TAP semantics: the original CALL instruction at this address still
    // executes, so Register IS reached, and our hle_register_resource
    // updates the texture registry organically. If both probes fire, the
    // texture registry should populate and pixels flow.
    ManualHook {
        name: "SPIDEY_REGISTER_CALL1_TAP",
        guest_addr: 0x0029_9325,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "SPIDEY_REGISTER_CALL2_TAP",
        guest_addr: 0x0029_9359,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Frame_CTOR_PROBE (2026-04-21) — DISABLED.
    // Probe proved master init reaches Frame ctor call (commit a53e7f2).
    // Now that the Wine-style SEH dispatcher is wired (commits ea57dcc,
    // 63a5062, + this commit's veh.rs integration), we need the REAL
    // Frame ctor body (sub_0xD67E0) to run so it throws the exception
    // that drives the dispatcher. Stubbing it would mask the very
    // exception we're trying to catch.
    // ManualHook { name: "Frame_CTOR_PROBE",     guest_addr: 0x002A_4A50, argc: 0, entry: SPIDERMAN_ENTRY },
    // CRT statically-linked RtlAllocateHeap core (after _SEH_prolog frame 0x178).
    // Despite the legacy name, sub_002AB360 IS the NT RtlAllocateHeap — its body
    // walks `[esi+edi*8+0x180]` (FreeLists bucket array) and `[esi+0x580]` (heap
    // spinlock) — classic MSVC 7.x _HEAP layout. All game malloc/new routes here
    // via sub_002B4FA0.
    // stdcall(3): RtlAllocateHeap(HANDLE hHeap, DWORD dwFlags, SIZE_T dwBytes) → LPVOID.  ret 12.
    ManualHook {
        name: "RtlAllocateHeap",
        guest_addr: 0x002A_B360,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    },
    // CRT RtlFreeHeap core — BOOL RtlFreeHeap(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem).
    // ret 12. Callers are sub_002A56B5 (public free wrapper) etc.
    // HLE stub returns TRUE (success) since our bump allocator never frees.
    ManualHook {
        name: "RtlFreeHeap",
        guest_addr: 0x002A_C3A2,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    },
    // SEH runtime — _SEH_prolog/epilog set up the fs:[0] exception chain.
    // Without these, __except blocks have corrupt scope tables → infinite loop in _except_handler3.
    ManualHook {
        name: "_SEH_prolog",
        guest_addr: 0x002B_7B28,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    ManualHook {
        name: "_SEH_epilog",
        guest_addr: 0x002B_7B61,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // D3D_MakeSpace REMOVED 2026-04-14 (cherry-picked from JIT commit 19a124b):
    // The HLE stub returned PB_DUMMY_BASE without updating the guest's PushBuffer
    // Put/Limit pointers, so the BeginPush inline macro re-called MakeSpace 150K+
    // times per session. Let the native D3D library's MakeSpace run organically —
    // it will update Put/Limit correctly, write real NV2A methods to guest PB memory,
    // and kick via USER_PUT (0xFD800040) which veh_mmio already handles.
    // Coupled with the PFIFO DMA_GET drain at 0xFD003244 which reports GET == sw PUT,
    // so the MakeSpace space-check always succeeds.
    // ManualHook { name: "D3D_MakeSpace",        guest_addr: 0x002F_2D70, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "D3D_MakeSpace",        guest_addr: 0x002F_2FD0, argc: 0, entry: SPIDERMAN_ENTRY },
    ManualHook {
        name: "CDevice_SetStateVB",
        guest_addr: 0x002F_91A0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4 (1 stack arg + thiscall ECX)
    // ---- Render loop hooks — addresses verified against Cxbx-R SymbolCache (spider-KrnlDebug.txt) ----
    ManualHook {
        name: "D3DDevice_SetTexture",
        guest_addr: 0x002F_0100,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8 = 2 args (Stage, pTexture)
    ManualHook {
        name: "D3DDevice_SetStreamSource",
        guest_addr: 0x002F_1680,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0xC = 3 args (StreamNumber, pStreamData, Stride)
    ManualHook {
        name: "D3DDevice_SetViewport",
        guest_addr: 0x002E_FC90,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2EFC90, ret 4
    ManualHook {
        name: "D3DDevice_CreateTexture",
        guest_addr: 0x002E_DF30,
        argc: 7,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2EDF30, ret 0x1C (7 args)
    ManualHook {
        name: "D3DDevice_BlockUntilVerticalBlank",
        guest_addr: 0x002E_E580,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R SymbolCache: 0x2EE580 (was 0x2F3340 = D3DResource_IsBusy — WRONG)
    ManualHook {
        name: "D3DDevice_SetPixelShaderConstant",
        guest_addr: 0x002F_5780,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2F5780, ret 0xC
    ManualHook {
        name: "D3DDevice_SetVertexShaderConstant",
        guest_addr: 0x002F_1D10,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // confirmed ret 0Ch (stdcall) — P0 root cause
    ManualHook {
        name: "D3DDevice_EndPush",
        guest_addr: 0x002F_2950,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0 = cdecl/thiscall, no stack args
    ManualHook {
        name: "D3DDevice_InsertFence",
        guest_addr: 0x002E_E880,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2EE880, ret 0
    ManualHook {
        name: "D3DDevice_DrawVerticesUP",
        guest_addr: 0x002E_D5C0,
        argc: 4,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2ED5C0, ret 0x10
    ManualHook {
        name: "D3DDevice_SetRenderState_FrontFace",
        guest_addr: 0x002E_C3B0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R: 0x2EC3B0, ret 4
    ManualHook {
        name: "D3DDevice_Swap",
        guest_addr: 0x002F_47D0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R SymbolCache: 0x2F47D0 (was 0x2F4B30 = D3DVertexBuffer_Lock — WRONG)
    // Surface lock/unlock — BIK video writes decoded frames directly to back buffer
    ManualHook {
        name: "Lock2DSurface",
        guest_addr: 0x002F_B020,
        argc: 6,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0x18 = 6 args: pPixelContainer, FaceType, Level, pLockedRect, pRect, Flags
    ManualHook {
        name: "D3DDevice_GetBackBuffer",
        guest_addr: 0x002E_F550,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // Cxbx-R SymbolCache: 0x2EF550 (was 0x2F4B60 = dead space — WRONG)
    // D3D_InternalSetState REMOVED — 0x2F5090 is actually D3DDevice_CreatePixelShader per Cxbx-R SymbolCache.
    // Was incorrectly NOP'ing pixel shader creation. CreatePixelShader will be handled by OOVPA scan.
    // ManualHook { name: "D3D_InternalSetState",          guest_addr: 0x002F_5090, argc: 2, entry: SPIDERMAN_ENTRY },
    ManualHook {
        name: "Get2DSurfaceDesc",
        guest_addr: 0x002F_AC70,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // stdcall(3): (pPixelContainer, Level, pDesc), ret 12
    // XGRPH CreateTexture wrapper — BIK video calls this instead of D3DDevice_CreateTexture @ 0x2EDF30.
    // stdcall(4): (device, width, format, ppTexture), ret 0x10. Wrapper calls 0x307BF9 internally.
    ManualHook {
        name: "XGRPH_CreateTexture",
        guest_addr: 0x0030_836D,
        argc: 4,
        entry: SPIDERMAN_ENTRY,
    },
    // XGRPH shader assembler / preprocessor wrapper. This is Xbox graphics
    // middleware, not Treyarch game logic. Native preprocessing currently
    // loops in the macro parser under AOT; HLE returns a tiny XGBuffer so the
    // D3D vertex-shader HLE can continue from the SDK boundary.
    ManualHook {
        name: "XGRPH_AssembleShader",
        guest_addr: 0x0033_022F,
        argc: 11,
        entry: SPIDERMAN_ENTRY,
    },
    // Post-call tap for the native XGRPH_AssembleShader canary. Planted only
    // when RUSTEMU_SPIDEY_XGRPH_LLE_CANARY is set; captures EAX and *ppCode.
    ManualHook {
        name: "XGRPH_AssembleShader_RET_TAP",
        guest_addr: 0x0029_C4B2,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Lexer entry inside XGRPH's generated parser. The native assembler returns
    // E_FAIL before recognizing "xvs.1.1"; this tap checks what bytes the lexer
    // is actually reading from its parser context.
    ManualHook {
        name: "XGRPH_Lexer_TAP",
        guest_addr: 0x0033_3696,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // XGRPH generated-parser error reporter. Planted only for the LLE canary;
    // captures the native error string and parser context/source position.
    ManualHook {
        name: "XGRPH_Error_TAP",
        guest_addr: 0x0033_4943,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // XGRPH parser main helper — thiscall + two stack args, ret 8.
    // The previous byte patch modified this function's epilogue at 0x003372A8
    // and corrupted the following helper. Hook the entry cleanly instead.
    ManualHook {
        name: "XGRPH_ParseDocument",
        guest_addr: 0x0033_7051,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    },
    // XGRPH parser helper — thiscall + one stack arg, ret 4. The old
    // pre-AOT byte patch at 0x003372A8 corrupted this function's prologue
    // at 0x003372AC. Handle the SDK parser seam explicitly instead.
    ManualHook {
        name: "XGRPH_ParseHelper",
        guest_addr: 0x0033_72AC,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    },
    // RAD/Bink middleware HLE seam. Native Bink decode is currently visible but
    // far too slow under AOT (~3 FPS), so the boot movie chain never reaches
    // TREYARCH/ORIGIN in normal verifier time. These are SDK/middleware
    // functions, not Treyarch game-state gates; HLE'ing them follows the same
    // boundary discipline as D3D/DSOUND.
    ManualHook {
        name: "BinkOpen",
        guest_addr: 0x0035_E880,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8: (filename, flags)
    ManualHook {
        name: "BinkPause",
        guest_addr: 0x0035_D7C0,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8: (handle, pause)
    ManualHook {
        name: "BinkWait",
        guest_addr: 0x0035_D5B0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (handle)
    ManualHook {
        name: "BinkDoFrame",
        guest_addr: 0x0035_CE50,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (handle)
    ManualHook {
        name: "BinkCopyToBuffer",
        guest_addr: 0x0035_C840,
        argc: 7,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0x1C: (handle, dest, pitch, height, x, y, flags)
    ManualHook {
        name: "BinkNextFrame",
        guest_addr: 0x0035_F760,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (handle)
    ManualHook {
        name: "BinkClose",
        guest_addr: 0x0035_FAB0,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (handle)
    // Reframed E (2026-04-26): Probe A — TAP-mode probe inside Spider-Man's
    // VBlank DPC body at 0x002FBF10 (registered by KeInitializeDpc per Agent
    // #6 caller analysis at site 0x002FC625 `push 0x002FBF10`). Default off,
    // gated on RUSTEMU_DPC_BODY_PROBE=1 in plant_manual_hooks below. When
    // active, dispatches through the TAP path in oovpa_dispatch.rs which
    // logs register state + dereferences [esi+0x100] (PMC_INTR MMIO) and
    // [esi+0x108] from the NV2A shadow. Tells us whether bit 0x01000000 is
    // visible to the body when it executes — the binary fork in the 35-agent
    // synthesis decision tree.
    ManualHook {
        name: "DPC_BODY_PROBE_ENTRY",
        guest_addr: 0x002F_BF10,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // SceneRender/SceneUpdate REMOVED — game code, not SDK boundary functions.
    // "HLE the OS, not the game" rule: let game logic run natively.
    // ManualHook { name: "SceneRender",  guest_addr: 0x000F_6C80, argc: 0, entry: SPIDERMAN_ENTRY },
    // ManualHook { name: "SceneUpdate", guest_addr: 0x000F_2740, argc: 0, entry: SPIDERMAN_ENTRY },
    // ---- Classic Doom (entry=0x3E5AB) manual hooks ----
    // I_FinishUpdate, I_WaitVBL, I_GetTime, I_InitSound removed — these are game code, not OS.
    // I_GetTime reads KeTickCount natively. I_FinishUpdate calls D3D Clear+Present (handled by D3D hooks).
    // I_InitSound is NOP'd by pre-AOT patches. All must run natively per "HLE the OS, not the game".
    // ---- XPP USB/OHCI driver stubs (Spider-Man specific) ----
    // Cxbx-R HLEs XInitDevices to skip the entire USB/OHCI driver init.
    // Without this, XapiInitProcess → XInitDevices → XPP driver → IoCreateDevice →
    // uninitialized vtable calls → spin bailout → game never reaches D3D.
    ManualHook {
        name: "XInitDevices",
        guest_addr: 0x0037_9FEC,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // WINAPI stdcall: (dwPreallocTypeCount, pPreallocTypes) = ret 8
    ManualHook {
        name: "XGetDevices",
        guest_addr: 0x0037_AD85,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (DeviceType)
    ManualHook {
        name: "XGetDeviceChanges",
        guest_addr: 0x0037_ADA7,
        argc: 3,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0xC: (DeviceType, pdwInsertions, pdwRemovals)
    ManualHook {
        name: "XInputOpen",
        guest_addr: 0x0037_A9E9,
        argc: 4,
        entry: SPIDERMAN_ENTRY,
    }, // ret 0x10: (DeviceType, dwPort, dwSlot, pPollingParams)
    ManualHook {
        name: "XInputClose",
        guest_addr: 0x0037_AA5E,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (hDevice)
    ManualHook {
        name: "XInputGetCapabilities",
        guest_addr: 0x0037_AA6A,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8: (hDevice, pCapabilities)
    ManualHook {
        name: "XInputGetState",
        guest_addr: 0x0037_AC5C,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8: (hDevice, pState)
    // ---- Shenmue II (entry=0x00132489) manual hooks ----
    // Optimized CRT memmove/memcpy helper. The native body uses REP MOVSD then
    // a computed tail-copy jump table. On Shenmue's first post-CreateDevice
    // path, the AOT indirect jump/table path repeatedly re-enters the tail and
    // walks ESP down to RET_TO_ZERO. This is cdecl, so keep argc=0 and let the
    // caller clean its three arguments.
    ManualHook {
        name: "Shenmue_CRT_memmove",
        guest_addr: 0x002C_CDD0,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    // Tiny matrix stack helpers used heavily during Shenmue's post-CreateDevice
    // setup. The native bodies are short SSE copies; current AOT occasionally
    // reaches them with poisoned base registers, so these are diagnostic HLE
    // probes rather than long-term game logic patches.
    ManualHook {
        name: "Shenmue_MatrixLoadCurrent",
        guest_addr: 0x0018_B26E,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "Shenmue_MatrixStoreCurrent",
        guest_addr: 0x0018_AF1E,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "Shenmue_MatrixLoadCurrent",
        guest_addr: 0x000F_7ABB,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "Shenmue_MatrixStoreCurrent",
        guest_addr: 0x000F_7AB1,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "Shenmue_MatrixMultiplyCurrent",
        guest_addr: 0x0018_B4DC,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "Shenmue_MatrixMultiplyCurrent",
        guest_addr: 0x0018_B5F4,
        argc: 0,
        entry: SHENMUE2_ENTRY,
    },
    ManualHook {
        name: "XInputSetState",
        guest_addr: 0x0037_ACCD,
        argc: 2,
        entry: SPIDERMAN_ENTRY,
    }, // ret 8: (hDevice, pFeedback)
    ManualHook {
        name: "XInputPoll",
        guest_addr: 0x0037_AD05,
        argc: 1,
        entry: SPIDERMAN_ENTRY,
    }, // ret 4: (hDevice)
    ManualHook {
        name: "MU_Init",
        guest_addr: 0x0037_9ED5,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    }, // void MU_Init(void) = ret 0
    // Game's assertion/fatal error handler: formats error message then hits INT3 (debug break).
    // 87 callers. cdecl varargs: (format_string, ...). argc=0 because cdecl = caller cleans stack.
    // HLE reads format string and logs it instead of crashing at INT3.
    ManualHook {
        name: "GameAssert",
        guest_addr: 0x0001_3E20,
        argc: 0,
        entry: SPIDERMAN_ENTRY,
    },
    // Doom_S_Init / Doom_S_Update removed — game code, not OS. S_Init must run natively
    // to set ticdup and sound state. DirectSoundCreate HLE provides the fake objects it needs.
    // ManualHook { name: "Doom_S_Init",           guest_addr: 0x0001_1AF0, argc: 0, entry: DOOM_ENTRY },
    // ManualHook { name: "Doom_S_Update",         guest_addr: 0x0001_1AF0, argc: 0, entry: DOOM_ENTRY },
    // TryRunTics removed — game code, not OS. Must run natively so G_Ticker processes demos.
    // ManualHook { name: "Doom_TryRunTics",    guest_addr: 0x0001_2C90, argc: 1, entry: DOOM_ENTRY },
    // R_Init_screen removed — game code that inits renderer vtable/callbacks. Must run natively
    // or call [ecx+0x15C] = NULL → crash + bogus indirect jump into heap.
    // ManualHook { name: "Doom_R_Init_screen",    guest_addr: 0x0001_1080, argc: 0, entry: DOOM_ENTRY },
    ManualHook {
        name: "Doom_DirectSoundCreate",
        guest_addr: 0x0008_4325,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // stdcall, 4 args, ret 16
    ManualHook {
        name: "Doom_DSoundRelease",
        guest_addr: 0x0004_F25D,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // one-arg cleanup/list release, ret 4; must hook the function entry, not 0x4F270 mid-body
    ManualHook {
        name: "CDirectSoundBuffer_Play",
        guest_addr: 0x0008_2B35,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x82B35, ret 16: (this, dwFlags, dwReserved1, dwReserved2)
    ManualHook {
        name: "CDirectSoundBuffer_Stop",
        guest_addr: 0x0008_2B86,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x82B86, ret 4: (this)
    ManualHook {
        name: "CDirectSoundBuffer_StopEx",
        guest_addr: 0x0008_2BD5,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x82BD5, ret 16: (this, rtTimeStamp, dwFlags, pReserved)
    ManualHook {
        name: "CDirectSoundBuffer_GetStatus",
        guest_addr: 0x0008_2C2E,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x82C2E, ret 8: (this, pdwStatus)
    ManualHook {
        name: "CDirectSoundBuffer_SetCurrentPosition",
        guest_addr: 0x0008_2C7F,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x82C7F, ret 8: (this, dwPlayCursor)
    ManualHook {
        name: "IDirectSoundBuffer_Play",
        guest_addr: 0x0008_3494,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x83494, ret 16
    ManualHook {
        name: "IDirectSoundBuffer_Stop",
        guest_addr: 0x0008_34B8,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x834B8, ret 4
    ManualHook {
        name: "IDirectSoundBuffer_StopEx",
        guest_addr: 0x0008_34D0,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x834D0, ret 16
    ManualHook {
        name: "IDirectSoundBuffer_GetStatus",
        guest_addr: 0x0008_34F4,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x834F4, ret 8: (this, pdwStatus)
    ManualHook {
        name: "IDirectSoundBuffer_SetCurrentPosition",
        guest_addr: 0x0008_3510,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x83510, ret 8
    ManualHook {
        name: "Doom_malloc",
        guest_addr: 0x0004_5325,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // _nh_malloc (cdecl: caller cleans stack)
    ManualHook {
        name: "Doom_malloc2",
        guest_addr: 0x0004_5652,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // malloc (cdecl: caller cleans stack)
    // D3D framework: addresses from Cxbx-R SymbolCache for Classic Doom (XDK 5849)
    ManualHook {
        name: "Direct3D_CreateDevice",
        guest_addr: 0x0005_4C10,
        argc: 6,
        entry: DOOM_ENTRY,
    }, // ret 24: Adapter, DeviceType, hFocusWindow, BehaviorFlags, pPresentationParameters, ppReturnedDeviceInterface
    ManualHook {
        name: "Doom_D3D_Swap",
        guest_addr: 0x0005_0920,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // ret 4: (Flags)
    ManualHook {
        name: "Doom_D3D_Clear",
        guest_addr: 0x0005_0BB0,
        argc: 6,
        entry: DOOM_ENTRY,
    }, // ret 24: (Count, pRects, Flags, Color, Z, Stencil)
    ManualHook {
        name: "Doom_D3D_DrawVerticesUP",
        guest_addr: 0x0005_1030,
        argc: 4,
        entry: DOOM_ENTRY,
    }, // ret 16: (PrimitiveType, VertexCount, pData, Stride)
    ManualHook {
        name: "Doom_D3D_Begin",
        guest_addr: 0x0005_11E0,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // ret 4: (PrimitiveType)
    ManualHook {
        name: "Doom_D3D_End",
        guest_addr: 0x0005_1220,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // bare ret
    ManualHook {
        name: "Doom_D3D_CopyRects",
        guest_addr: 0x0005_2F50,
        argc: 5,
        entry: DOOM_ENTRY,
    }, // ret 20: (pSource, pRects, cRects, pDest, pPoints)
    ManualHook {
        name: "Doom_D3D_SetTexture",
        guest_addr: 0x0005_34B0,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // ret 8: (Stage, pTexture)
    ManualHook {
        name: "Doom_D3D_BlockVBlank",
        guest_addr: 0x0005_2AB0,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // bare ret
    ManualHook {
        name: "Doom_D3D_IsBusy",
        guest_addr: 0x0005_3660,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // bare ret
    ManualHook {
        name: "Doom_D3D_SetRenderTarget",
        guest_addr: 0x0005_2C80,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // ret 8: (pRenderTarget, pDepthStencil)
    ManualHook {
        name: "Doom_D3D_GetBackBuffer2",
        guest_addr: 0x0005_2F00,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // ret 4: (BackBuffer)
    ManualHook {
        name: "Doom_D3D_PersistDisplay",
        guest_addr: 0x0005_3AF0,
        argc: 3,
        entry: DOOM_ENTRY,
    }, // ret 12 (stdcall, 3 args)
    ManualHook {
        name: "Doom_D3D_KickOff",
        guest_addr: 0x0005_5C40,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // ret 0 (thiscall, no pushed args)
    ManualHook {
        name: "Doom_D3D_MakeSpace",
        guest_addr: 0x0005_60D0,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // ret 0 (thiscall, no pushed args)
    ManualHook {
        name: "Doom_D3D_KickOffAndWait",
        guest_addr: 0x0005_6130,
        argc: 0,
        entry: DOOM_ENTRY,
    }, // bare ret
    ManualHook {
        name: "Doom_D3D_SetViewport",
        guest_addr: 0x0005_3280,
        argc: 1,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x53280, ret 4
    ManualHook {
        name: "Doom_D3D_LoadVertexShader",
        guest_addr: 0x0005_4500,
        argc: 2,
        entry: DOOM_ENTRY,
    }, // Cxbx SymbolCache 0x54500, ret 8
    // ---- Doom gamestate-init probe (2026-04-26) ----
    // [0x00101BB8] (gamestate ptr) is NULL → game shows black screen because
    // 1508 readers find no state. Three TAPs identify which step in the chain
    // breaks: outer-loop never runs, inner never reached, or allocator returns 0.
    // Tap-only — game runs natively, we just log register/global state on entry.
    ManualHook {
        name: "DOOM_GAMESTATE_OUTER_ENTRY_TAP",
        guest_addr: 0x0001_2C10,
        argc: 1,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_GAMESTATE_INNER_ENTRY_TAP",
        guest_addr: 0x0001_29F0,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    // 0x12A04 = first instruction *after* `call 0x45652` returns. EAX holds
    // the allocator's return value (gamestate buffer ptr or NULL).
    ManualHook {
        name: "DOOM_ALLOC_RET_TAP",
        guest_addr: 0x0001_2A04,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_RENDER_BEFORE_VIEW_TAP",
        guest_addr: 0x0001_2CE2,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_RENDER_AFTER_VIEW_TAP",
        guest_addr: 0x0001_2CE7,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_RENDER_AFTER_BLIT_TAP",
        guest_addr: 0x0001_2CF3,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_RENDER_AFTER_HUD_TAP",
        guest_addr: 0x0001_2D08,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_COLUMN_WRITE_TAP",
        guest_addr: 0x0002_6A62,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_COLUMN_EDX_TAP",
        guest_addr: 0x0002_69B0,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    // Do not plant DOOM_COLUMN_CALL_TAP here: 0x2F165 is already an AOT
    // indirect-call INT3 trap, so TAP mode would restore 0xCC and self-loop.
    // The non-invasive before-call probe lives in veh_dispatch's Indirect path.
    ManualHook {
        name: "DOOM_COLUMN_CALL_AFTER_TAP",
        guest_addr: 0x0002_F16B,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_SPAN_CALL_AFTER_TAP",
        guest_addr: 0x0002_D536,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_MASKED_SCALE_SOURCE_TAP",
        guest_addr: 0x0002_F225,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_VIEWMAP_ENTRY_TAP",
        guest_addr: 0x0001_9C40,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_SETVIEW_ENTRY_TAP",
        guest_addr: 0x0001_9E30,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_EXEC_SETVIEW_ENTRY_TAP",
        guest_addr: 0x0001_9E60,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_RENDER_INIT_TAP",
        guest_addr: 0x0001_A310,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_POINTANGLE1_RET_TAP",
        guest_addr: 0x0002_FFB1,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_POINTANGLE2_RET_TAP",
        guest_addr: 0x0002_FFC0,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_CLIP_INPUT_TAP",
        guest_addr: 0x0002_FFF1,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_VIEWMAP_BUILD_STORE_TAP",
        guest_addr: 0x0001_9CDD,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_VIEWMAP_CLAMP_INPUT_TAP",
        guest_addr: 0x0001_9D60,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_VIEWMAP_CLAMP_FINAL_TAP",
        guest_addr: 0x0001_9D87,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_CLIP_OUTPUT_TAP",
        guest_addr: 0x0003_003D,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_PRE_LOOKUP_TAP",
        guest_addr: 0x0003_0049,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_CLIP_SOLID_ENTRY_TAP",
        guest_addr: 0x0002_FD90,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_CLIP_PASS_ENTRY_TAP",
        guest_addr: 0x0002_FEB0,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_PROJECTED_X_TAP",
        guest_addr: 0x0003_005F,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_PASS_DECISION_TAP",
        guest_addr: 0x0003_00D4,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_SOLID_DECISION_TAP",
        guest_addr: 0x0003_00E9,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_RANGE_ENTRY_TAP",
        guest_addr: 0x0003_9D40,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    ManualHook {
        name: "DOOM_WALL_SCALE_SETUP_TAP",
        guest_addr: 0x0003_9F1C,
        argc: 0,
        entry: DOOM_ENTRY,
    },
    // ticdup diagnostic hooks removed — would prevent game init from running
    // Doom_D3D_Render removed — game render callback must run natively so it calls
    // DrawVerticesUP. Init chain is fixed (I_InitSound, S_Init run natively), VEH
    // div-by-zero handler catches ticdup=0 divisions.
    // ManualHook { name: "Doom_D3D_Render",             guest_addr: 0x0001_10E0, argc: 2, entry: DOOM_ENTRY },
    // ---- test_blue / test_fence / test_* manual hooks REMOVED ----
    // These were hardcoded from stale map files and don't match current builds.
    // Verified: XDK 5344 map has Clear@0x1A0A0, Swap@0x19EF0 (not 0x1AD80/0x1ABD0).
    // XDK 5849 map has Clear@0x19F70, Swap@0x19DC0 (not 0x1C680/0x1ACB0).
    // OOVPA hand-tuned + CxbxDB patterns find these functions dynamically — no manual hooks needed.
    // Having entry:0 caused false INT3 in Spider-Man at random mid-function addresses.
];

/// Plant INT3 hooks at all matched + manual addresses.
/// Requires mutable access to the code buffer (VirtualProtect).
/// Returns (planted_count, misaligned_guest_addrs) — misaligned entries need rescue-compile.
pub fn plant_hooks(
    matches: &mut Vec<OovpaMatch>,
    code_base: *mut u8,
    addr_hash: &crate::xbox::aot::runtime::AddrHash,
) -> (u32, Vec<(usize, u32)>) {
    if code_base.is_null() || matches.is_empty() {
        return (0, Vec::new());
    }

    let mut planted = 0u32;
    let mut misaligned: Vec<(usize, u32)> = Vec::new(); // (match_index, guest_addr)

    // Plant OOVPA-scanned hooks
    for (idx, m) in matches.iter_mut().enumerate() {
        if m.is_manual {
            continue;
        }
        match addr_hash.lookup(m.guest_addr) {
            Some(host_off) => {
                // Exact match — plant directly
                if plant_int3(code_base, host_off, m) {
                    planted += 1;
                }
            }
            None => {
                // No exact entry — record for rescue-compile pass
                misaligned.push((idx, m.guest_addr));
                debug_log(&format!(
                    "[OOVPA] Misaligned: {} at guest 0x{:08X} — needs rescue-compile",
                    m.pattern_name, m.guest_addr
                ));
            }
        }
    }

    // Plant manual hooks — game-specific hardcoded addresses.
    // Each hook has an `entry` field: non-zero = only plant for that XBE entry point,
    // zero = plant for any game in the XDK version range.
    let xdk_build = XDK_BUILD.load(std::sync::atomic::Ordering::Relaxed);
    let entry_point = XBE_ENTRY.load(std::sync::atomic::Ordering::Relaxed);
    let use_manual = (xdk_build >= 4034 && xdk_build <= 4361)
        || (xdk_build >= 5300 && xdk_build <= 5900)
        || entry_point == SHENMUE2_ENTRY;
    if !use_manual {
        debug_log(&format!(
            "[OOVPA] Skipping {} manual hooks (XDK build {} not in supported range)",
            MANUAL_HOOKS.len(),
            xdk_build
        ));
    }
    // Bink HLE feature gate (Codex 2026-04-26 plan, Step 3):
    // RUSTEMU_NATIVE_BINK=1 disables Bink* manual hooks so the AOT-compiled
    // native Bink decoder runs instead of the no-op HLE stubs in oovpa_hle.rs.
    // Default off — current synthetic-skip behavior preserved (the HLE stubs
    // exist because native Bink ran at ~3 FPS during early testing and trapped
    // the boot chain in ACTIVISN.bik). Step 4 will measure native perf with
    // timing logs to find the real first blocker.
    let native_bink = env_flag("RUSTEMU_NATIVE_BINK");
    if native_bink {
        debug_log(
            "[OOVPA] RUSTEMU_NATIVE_BINK=1 — skipping Bink* manual hooks; native AOT decode path active",
        );
    }
    // Reframed E Probe A: gate planting on env var so default behavior is
    // unchanged (smoke gate stays bit-for-bit identical when unset).
    let dpc_body_probe = env_flag("RUSTEMU_DPC_BODY_PROBE");
    if dpc_body_probe {
        debug_log("[OOVPA] RUSTEMU_DPC_BODY_PROBE=1 — planting TAP probe at DPC body 0x002FBF10");
    }
    let cxbx_oovpa_strict = env_flag("RUSTEMU_CXBX_OOVPA_STRICT");
    if cxbx_oovpa_strict {
        debug_log(
            "[OOVPA] RUSTEMU_CXBX_OOVPA_STRICT=1 — skipping manual hooks; scanner OOVPA matches only",
        );
    }
    let title_probes = title_probes_enabled();
    let title_patches = title_patches_enabled();
    if title_probes {
        debug_log("[OOVPA] RUSTEMU_TITLE_PROBES=1 — Spider-Man title TAP probes may be planted");
    }
    if title_patches {
        debug_log(
            "[OOVPA] RUSTEMU_TITLE_PATCHES=1 — Spider-Man title-specific state patches are allowed",
        );
    }
    let spidey_input_event_probe = spidey_input_event_probes_enabled();
    if spidey_input_event_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_INPUT_EVENT_PROBES=1 — planting Spider-Man input/event TAP probes only",
        );
    }
    let spidey_f8580_probe = spidey_f8580_probe_enabled();
    if spidey_f8580_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_F8580_PROBE=1 — planting Spider-Man F8580 TAP probes only",
        );
    }
    let spidey_fun29c5a0_probe = spidey_fun29c5a0_probe_enabled();
    if spidey_fun29c5a0_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_FUN29C5A0_PROBE=1 — planting Spider-Man c[-96..-93] init TAP probes",
        );
    }
    let spidey_aot_script_probe = spidey_aot_script_probe_enabled();
    if spidey_aot_script_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_AOT_SCRIPT_PROBES=1 — planting Spider-Man script VM TAP probes",
        );
    }
    let spidey_xbs_parse_probe = spidey_xbs_parse_probe_enabled();
    if spidey_xbs_parse_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_XBS_PARSE_PROBE=1 — planting Spider-Man XBS archive parser TAP probes",
        );
    }
    let spidey_xbs_lifecycle_probe = spidey_xbs_lifecycle_probe_enabled();
    if spidey_xbs_lifecycle_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_XBS_LIFECYCLE_PROBE=1 — planting Spider-Man XBS scene/script lifecycle TAP probes",
        );
    }
    let spidey_string_probe = spidey_string_probe_enabled();
    if spidey_string_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_STRING_PROBE=1 — planting Spider-Man string/font allocator TAP probes",
        );
    }
    // These interior Spider-Man scene probes are useful when chasing the
    // ORIGIN_Z handoff, but they are not part of the stable boot path.
    // Keep them opt-in so normal Spider-Man runs do not plant extra
    // mid-function INT3 taps while we preserve the known menu/training path.
    let spidey_scene_probe = env_flag("RUSTEMU_SPIDEY_SCENE_PROBE") || title_probes;
    if spidey_scene_probe {
        debug_log("[OOVPA] RUSTEMU_SPIDEY_SCENE_PROBE=1 — planting F0690/E9D00 scene TAP probes");
    }
    // Cxbx-R-style OOVPA patches identify SDK/API entrypoints. These
    // Spider-Man action-gate hooks are title-internal mid-function probes, so
    // keep them opt-in while isolating menu reparse/state-loop regressions.
    let spidey_action_probe = env_flag("RUSTEMU_SPIDEY_ACTION_PROBE") || title_probes;
    if spidey_action_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_ACTION_PROBE=1 — planting Spider-Man action-gate TAP probes",
        );
    }
    let spidey_trigger_probe = env_flag("RUSTEMU_SPIDEY_TRIGGER_PROBE") || title_probes;
    if spidey_trigger_probe {
        debug_log(
            "[OOVPA] RUSTEMU_SPIDEY_TRIGGER_PROBE=1 — planting Spider-Man trigger/vtable TAP probes",
        );
    }
    let mut doom_render_probe_planted = 0u32;
    let mut doom_column_probe_planted = 0u32;
    for mh in MANUAL_HOOKS {
        if !use_manual {
            break;
        }
        if cxbx_oovpa_strict {
            break;
        }
        if mh.name.starts_with("DOOM_COLUMN_") && !env_flag("RUSTEMU_DOOM_COLUMN_PROBE") {
            continue;
        }
        if mh.name.starts_with("DOOM_SPAN_") && !env_flag("RUSTEMU_DOOM_SPAN_PROBE") {
            continue;
        }
        if mh.name == "DOOM_SPAN_CALL_AFTER_TAP" && !env_flag("RUSTEMU_DOOM_SPAN_AFTER_TAP") {
            continue;
        }
        if (mh.name.starts_with("DOOM_WALL_") || mh.name.starts_with("DOOM_MASKED_"))
            && !env_flag("RUSTEMU_DOOM_WALL_PROBE")
        {
            continue;
        }

        // Skip hooks tagged for a different game's entry point
        if mh.entry != 0 && mh.entry != entry_point {
            continue;
        }

        if mh.entry == SPIDERMAN_ENTRY && mh.name.starts_with("SPIDEY_") && !title_probes {
            if spidey_input_event_probe && is_spidey_input_event_probe(mh.name) {
                // Plant only the focused input/event probes; keep the broader
                // scene/root-write probe pack off unless title probes are enabled.
            } else if spidey_trigger_probe && mh.name.starts_with("SPIDEY_TRIGGER_") {
                // Plant only the focused trigger/vtable probes.
            } else if spidey_f8580_probe && is_spidey_f8580_probe(mh.name) {
                // Plant only the focused F8580 state-machine probes.
            } else if spidey_fun29c5a0_probe && mh.name.starts_with("SPIDEY_FUN29C5A0_") {
                // Plant only the focused shader-constant init probes.
            } else if spidey_aot_script_probe && is_spidey_script_probe(mh.name) {
                // Plant only the focused AOT-side XBS/script VM probes.
            } else if spidey_xbs_parse_probe && mh.name.starts_with("SPIDEY_XBS_PARSE_") {
                // Plant only the focused XBS archive parser probes.
            } else if spidey_xbs_lifecycle_probe && mh.name.starts_with("SPIDEY_XBS_LIFECYCLE_") {
                // Plant only the focused scene/script lifecycle probes.
            } else if spidey_scene_probe && is_spidey_scene_probe(mh.name) {
                // Plant only the focused scene-init probes.
            } else if spidey_string_probe && mh.name.starts_with("SPIDEY_STRING_") {
                // Plant only the focused string/font allocator probes.
            } else {
                continue;
            }
        }

        // Bink HLE feature gate (Steps 3+4):
        //   default mode  → HLE stub (current ~3 FPS workaround)
        //   native_bink=1 → TAP mode (run native body, log entry timestamp)
        // TAP achieves "disable stubs" while keeping a single-byte INT3 at
        // function entry — every call hits VEH once, gets timestamped, then
        // resumes the native body. The timing log distinguishes
        // "DoFrame > 50 ms = AOT/perf" from "stuck in BinkWait = DSound" from
        // "frame decoded but not visible = D3D copy/surface".
        let bink_native_tap = native_bink && mh.name.starts_with("Bink");
        // Reframed E Probe A: skip planting unless the env var is set, so
        // default smoke gate is bit-identical to before this commit.
        if mh.name == "DPC_BODY_PROBE_ENTRY" && !dpc_body_probe {
            continue;
        }
        if is_spidey_scene_probe(mh.name) && !spidey_scene_probe {
            continue;
        }
        if mh.name.starts_with("SPIDEY_ACTION_GATE_") && !spidey_action_probe {
            continue;
        }
        if mh.name.starts_with("SPIDEY_TRIGGER_") && !spidey_trigger_probe {
            continue;
        }
        if mh.name.starts_with("SPIDEY_FUN29C5A0_") && !spidey_fun29c5a0_probe {
            continue;
        }
        if mh.name.starts_with("SPIDEY_XBS_LIFECYCLE_") && !spidey_xbs_lifecycle_probe {
            continue;
        }
        if is_spidey_script_probe(mh.name) && !spidey_aot_script_probe {
            continue;
        }
        if mh.name.starts_with("SPIDEY_STRING_") && !spidey_string_probe {
            continue;
        }
        if mh.name == "XGRPH_AssembleShader_RET_TAP" && !env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY")
        {
            continue;
        }
        if mh.name == "XGRPH_Lexer_TAP" && !env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
            continue;
        }
        if mh.name == "XGRPH_Error_TAP" && !env_flag("RUSTEMU_SPIDEY_XGRPH_LLE_CANARY") {
            continue;
        }
        let final_mode = if bink_native_tap {
            HleMode::Tap
        } else {
            manual_hook_mode(mh.name)
        };
        match addr_hash.lookup(mh.guest_addr) {
            Some(host_off) => {
                let mut m = OovpaMatch {
                    guest_addr: mh.guest_addr,
                    host_offset: 0,
                    original_byte: 0,
                    pattern_name: mh.name,
                    argc: mh.argc,
                    hle_mode: final_mode,
                    is_cxbx: false,
                    is_manual: true,
                    active: false,
                    pending_rearm: false,
                    call_count: 0,
                };
                if plant_int3(code_base, host_off, &mut m) {
                    planted += 1;
                    if mh.name.starts_with("DOOM_RENDER_") {
                        doom_render_probe_planted += 1;
                        debug_log(&format!(
                            "[DOOM-RENDER-PROBE-INSTALL] planted {} guest=0x{:08X} host=+0x{:X} mode={:?}",
                            mh.name, mh.guest_addr, host_off, final_mode
                        ));
                    } else if mh.name.starts_with("DOOM_COLUMN_") {
                        doom_column_probe_planted += 1;
                        debug_log(&format!(
                            "[DOOM-COLUMN-PROBE-INSTALL] planted {} guest=0x{:08X} host=+0x{:X} mode={:?}",
                            mh.name, mh.guest_addr, host_off, final_mode
                        ));
                    } else if mh.name.starts_with("DOOM_WALL_")
                        || mh.name.starts_with("DOOM_MASKED_")
                    {
                        debug_log(&format!(
                            "[DOOM-WALL-PROBE-INSTALL] planted {} guest=0x{:08X} host=+0x{:X} mode={:?}",
                            mh.name, mh.guest_addr, host_off, final_mode
                        ));
                    }
                    matches.push(m);
                }
            }
            None => {
                debug_log(&format!(
                    "[OOVPA] Manual hook: no host offset for {} at 0x{:08X}",
                    mh.name, mh.guest_addr
                ));
                if mh.name.starts_with("DOOM_RENDER_") {
                    debug_log(&format!(
                        "[DOOM-RENDER-PROBE-INSTALL] missing {} guest=0x{:08X}",
                        mh.name, mh.guest_addr
                    ));
                } else if mh.name.starts_with("DOOM_COLUMN_") {
                    debug_log(&format!(
                        "[DOOM-COLUMN-PROBE-INSTALL] missing {} guest=0x{:08X}",
                        mh.name, mh.guest_addr
                    ));
                } else if mh.name.starts_with("DOOM_WALL_") || mh.name.starts_with("DOOM_MASKED_") {
                    debug_log(&format!(
                        "[DOOM-WALL-PROBE-INSTALL] missing {} guest=0x{:08X}",
                        mh.name, mh.guest_addr
                    ));
                }
            }
        }
    }

    if entry_point == DOOM_ENTRY {
        debug_log(&format!(
            "[DOOM-RENDER-PROBE-INSTALL] summary planted={}/4 entry=0x{:08X} xdk={} use_manual={} strict={}",
            doom_render_probe_planted, entry_point, xdk_build, use_manual, cxbx_oovpa_strict
        ));
        debug_log(&format!(
            "[DOOM-COLUMN-PROBE-INSTALL] summary planted={}/4 entry=0x{:08X} xdk={} use_manual={} strict={}",
            doom_column_probe_planted, entry_point, xdk_build, use_manual, cxbx_oovpa_strict
        ));
    }

    debug_log(&format!(
        "[OOVPA] Hooks planted: {}/{} ({} need rescue-compile)",
        planted,
        matches.len(),
        misaligned.len()
    ));
    (planted, misaligned)
}

/// Public wrapper for rescue-compile hook planting from emulator.rs.
pub fn plant_int3_pub(code_base: *mut u8, host_off: u32, m: &mut OovpaMatch) -> bool {
    plant_int3(code_base, host_off, m)
}

/// Plant INT3 at a single host offset, saving the original byte.
pub(super) fn plant_int3(code_base: *mut u8, host_off: u32, m: &mut OovpaMatch) -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::System::Memory::*;
        let target = unsafe { code_base.add(host_off as usize) };
        let mut old_prot = PAGE_PROTECTION_FLAGS(0);
        let ok =
            unsafe { VirtualProtect(target as *const _, 1, PAGE_EXECUTE_READWRITE, &mut old_prot) };
        if ok.is_ok() {
            m.host_offset = host_off;
            m.original_byte = unsafe { *target };
            unsafe {
                *target = 0xCC;
            } // INT3
            let _ = unsafe { VirtualProtect(target as *const _, 1, old_prot, &mut old_prot) };
            m.active = true;
            debug_log(&format!(
                "[OOVPA] Hook: {} guest=0x{:08X} host=+0x{:X} orig=0x{:02X}",
                m.pattern_name, m.guest_addr, host_off, m.original_byte
            ));
            true
        } else {
            debug_log(&format!(
                "[OOVPA] VirtualProtect failed for {} at host +0x{:X}",
                m.pattern_name, host_off
            ));
            false
        }
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Re-plant INT3 for hooks with pending_rearm.
pub fn rearm_pending(matches: &mut [OovpaMatch], code_base: *mut u8) {
    if code_base.is_null() {
        return;
    }
    for m in matches.iter_mut() {
        if !m.pending_rearm {
            continue;
        }
        let target = unsafe { code_base.add(m.host_offset as usize) };
        #[cfg(windows)]
        {
            use windows::Win32::System::Memory::*;
            let mut old_prot = PAGE_PROTECTION_FLAGS(0);
            if unsafe {
                VirtualProtect(target as *const _, 1, PAGE_EXECUTE_READWRITE, &mut old_prot)
            }
            .is_ok()
            {
                m.original_byte = unsafe { *target };
                unsafe {
                    *target = 0xCC;
                }
                let _ = unsafe { VirtualProtect(target as *const _, 1, old_prot, &mut old_prot) };
            }
        }
        m.active = true;
        m.pending_rearm = false;
    }
}
