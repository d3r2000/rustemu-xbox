use super::{database, HleMode, OovpaMatch, OovpaPattern, G_PDEVICE_DYNAMIC, XBE_ENTRY, XDK_BUILD};
/// OOVPA pattern scanner — scans guest memory for known SDK function signatures.
use crate::xbox::emulator::debug_log;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OovpaScanRangeKind {
    D3d,
    D3dx,
    Xgrph,
    Dsound,
    Xapi,
    OtherExec,
}

#[derive(Debug, Clone, Copy)]
pub struct OovpaScanRange {
    pub start: u32,
    pub end: u32,
    pub kind: OovpaScanRangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OovpaScanProfile {
    FastBoot,
    Graphics,
    Full,
}

pub(crate) fn active_scan_profile_name() -> &'static str {
    match (active_scan_profile(), symbol_fixture_toml_enabled()) {
        (OovpaScanProfile::FastBoot, true) => "fast_boot_toml_only_trusted",
        (OovpaScanProfile::Graphics, true) => "graphics_toml_only_trusted",
        (OovpaScanProfile::Full, true) => "full_toml_only_trusted",
        (profile, false) => profile.name(),
    }
}

fn oovpa_diff_enabled() -> bool {
    std::env::var("RUSTEMU_OOVPA_DIFF")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "off" || v == "no")
        })
        .unwrap_or(false)
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "off" || v == "no")
        })
        .unwrap_or(false)
}

fn symbol_fixture_toml_enabled() -> bool {
    std::env::var("RUSTEMU_OOVPA_SYMBOLS_TOML")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
        || env_flag("RUSTEMU_SPIDEY_4134_TOML")
        || env_flag("RUSTEMU_DOOM_5849_TOML")
}

pub(crate) fn symbol_cache_input_enabled(
    entry_point: u32,
    title_id: Option<u32>,
    xdk_build: u16,
    d3d8_is_ltcg: bool,
) -> bool {
    map_symbol_fixture_enabled()
        || (!symbol_fixture_toml_enabled()
            && find_symbol_cache_path(entry_point, title_id, xdk_build, d3d8_is_ltcg).is_some())
}

pub(crate) fn map_symbol_fixture_enabled() -> bool {
    find_msvc_map_symbol_path().is_some()
}

fn active_scan_profile() -> OovpaScanProfile {
    match std::env::var("RUSTEMU_OOVPA_PROFILE") {
        Ok(v) => match v.trim().to_ascii_lowercase().as_str() {
            "full" | "all" | "legacy" => OovpaScanProfile::Full,
            "graphics" | "gfx" | "d3d" => OovpaScanProfile::Graphics,
            _ => OovpaScanProfile::FastBoot,
        },
        Err(_) => OovpaScanProfile::FastBoot,
    }
}

impl OovpaScanProfile {
    fn name(self) -> &'static str {
        match self {
            OovpaScanProfile::FastBoot => "fast_boot",
            OovpaScanProfile::Graphics => "graphics",
            OovpaScanProfile::Full => "full",
        }
    }
}

/// Scan guest memory for OOVPA patterns. Returns matches found.
pub fn scan(
    guest_base: *const u8,
    scan_start: u32,
    scan_end: u32,
    xdk_build: u16,
    entry_point: u32,
    d3d8_is_ltcg: bool,
) -> Vec<OovpaMatch> {
    let range = OovpaScanRange {
        start: scan_start,
        end: scan_end,
        kind: OovpaScanRangeKind::OtherExec,
    };
    scan_ranges(
        guest_base,
        &[range],
        xdk_build,
        entry_point,
        d3d8_is_ltcg,
        None,
    )
}

/// Scan guest memory for OOVPA patterns across already-classified XBE section ranges.
pub fn scan_ranges(
    guest_base: *const u8,
    scan_ranges: &[OovpaScanRange],
    xdk_build: u16,
    entry_point: u32,
    d3d8_is_ltcg: bool,
    title_id: Option<u32>,
) -> Vec<OovpaMatch> {
    let scan_ranges: Vec<OovpaScanRange> = scan_ranges
        .iter()
        .copied()
        .filter(|r| r.start < r.end)
        .collect();
    let scan_start = scan_ranges.iter().map(|r| r.start).min().unwrap_or(0);
    let scan_end = scan_ranges.iter().map(|r| r.end).max().unwrap_or(0);
    if guest_base.is_null() || scan_start >= scan_end {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let profile = active_scan_profile();
    let all_ranges = ranges_for_kinds(
        &scan_ranges,
        &[
            OovpaScanRangeKind::D3d,
            OovpaScanRangeKind::D3dx,
            OovpaScanRangeKind::Xgrph,
            OovpaScanRangeKind::Dsound,
            OovpaScanRangeKind::Xapi,
            OovpaScanRangeKind::OtherExec,
        ],
    );
    let d3d_ranges = ranges_for_kinds(
        &scan_ranges,
        &[
            OovpaScanRangeKind::D3d,
            OovpaScanRangeKind::D3dx,
            OovpaScanRangeKind::Xgrph,
        ],
    );
    let sdk_ranges = ranges_for_kinds(
        &scan_ranges,
        &[
            OovpaScanRangeKind::D3d,
            OovpaScanRangeKind::D3dx,
            OovpaScanRangeKind::Xgrph,
            OovpaScanRangeKind::Dsound,
            OovpaScanRangeKind::Xapi,
        ],
    );
    let dsound_ranges = ranges_for_kinds(&scan_ranges, &[OovpaScanRangeKind::Dsound]);
    let xapi_ranges = ranges_for_kinds(&scan_ranges, &[OovpaScanRangeKind::Xapi]);
    let xgraphic_ranges = ranges_for_kinds(&scan_ranges, &[OovpaScanRangeKind::Xgrph]);
    let d3d_ranges = fallback_ranges(d3d_ranges, &all_ranges);
    let sdk_ranges = fallback_ranges(sdk_ranges, &all_ranges);
    let dsound_ranges = fallback_ranges(dsound_ranges, &all_ranges);
    let xapi_ranges = fallback_ranges(xapi_ranges, &all_ranges);
    let xgraphic_ranges = fallback_ranges(xgraphic_ranges, &all_ranges);

    XDK_BUILD.store(xdk_build, std::sync::atomic::Ordering::Relaxed);
    XBE_ENTRY.store(entry_point, std::sync::atomic::Ordering::Relaxed);
    G_PDEVICE_DYNAMIC.store(0, std::sync::atomic::Ordering::Relaxed);

    // Select CxbxDB pattern set based on XDK build version.
    // Each XDK version ships different D3D8 library code with different byte patterns.
    // Scanning wrong-version patterns causes false positives (e.g., TwoSidedLighting
    // matching CRT code on XDK 5344 when the pattern is from XDK 1036).
    use crate::xbox::aot::oovpa_patterns;
    let cxbx_patterns: &[OovpaPattern] = match xdk_build {
        0 => {
            // Unknown version — use full DB but will rely on min_version filter
            debug_log("[OOVPA] XDK build unknown — scanning all CxbxDB patterns");
            oovpa_patterns::CXBX_DB_PATTERNS
        }
        1..=3911 => {
            debug_log(&format!(
                "[OOVPA] XDK build {} — early SDK (≤3911)",
                xdk_build
            ));
            oovpa_patterns::CXBX_DB_PATTERNS
        }
        3912..=4361 => {
            // XDK 4034-4361: Spider-Man (4134), early retail titles
            debug_log(&format!(
                "[OOVPA] XDK build {} — mid SDK (3912-4361, Spider-Man era)",
                xdk_build
            ));
            oovpa_patterns::CXBX_DB_PATTERNS
        }
        4362..=5344 => {
            // XDK 5344: test_blue, mid-lifecycle titles
            debug_log(&format!(
                "[OOVPA] XDK build {} — late SDK (4362-5344, test_blue era)",
                xdk_build
            ));
            oovpa_patterns::CXBX_DB_PATTERNS
        }
        5345..=5849 => {
            // XDK 5849: Classic Doom, late-lifecycle titles
            debug_log(&format!(
                "[OOVPA] XDK build {} — final SDK (5345-5849, Doom era)",
                xdk_build
            ));
            oovpa_patterns::CXBX_DB_PATTERNS
        }
        _ => {
            debug_log(&format!(
                "[OOVPA] XDK build {} — unknown range, scanning all",
                xdk_build
            ));
            oovpa_patterns::CXBX_DB_PATTERNS
        }
    };

    debug_log(&format!(
        "[OOVPA] Scanning profile={} ranges={} total={} bytes d3d={} bytes sdk={} bytes xapi={} bytes dsound={} bytes xgraphic={} bytes (0x{:08X}-0x{:08X}) — {} hand-tuned + {} CxbxDB patterns, XDK build={}",
        profile.name(),
        scan_ranges.len(),
        total_range_bytes(&all_ranges),
        total_range_bytes(&d3d_ranges),
        total_range_bytes(&sdk_ranges),
        total_range_bytes(&xapi_ranges),
        total_range_bytes(&dsound_ranges),
        total_range_bytes(&xgraphic_ranges),
        scan_start,
        scan_end,
        oovpa_patterns::HAND_TUNED_PATTERNS.len(),
        cxbx_patterns.len(),
        xdk_build
    ));

    // Phase 1: hand-tuned patterns — gated by XDK version.
    // These are manually tuned for specific XDK versions and WILL false-match
    // if applied to the wrong version.
    let hand_tuned: &[OovpaPattern] = match xdk_build {
        0 => {
            debug_log("[OOVPA] XDK build unknown — hand-tuned fallback");
            oovpa_patterns::HAND_TUNED_PATTERNS
        }
        3911..=5849 => {
            // XDK 3911-5849: hand-tuned patterns cover core D3D8 functions
            // (CreateDevice, Clear, Swap, etc.) that are stable across XDK versions.
            // CxbxDB adds version-specific patterns on top.
            debug_log(&format!(
                "[OOVPA] XDK build {} — hand-tuned + CxbxDB",
                xdk_build
            ));
            oovpa_patterns::HAND_TUNED_PATTERNS
        }
        _ => {
            debug_log(&format!(
                "[OOVPA] XDK build {} — CxbxDB only (no hand-tuned)",
                xdk_build
            ));
            &[]
        }
    };
    scan_table(
        guest_base,
        &sdk_ranges,
        scan_end,
        hand_tuned,
        false,
        xdk_build,
        profile,
        &mut matches,
    );

    let ht_count = matches.len();
    debug_log(&format!(
        "[OOVPA] Phase 1 (hand-tuned): {} matches",
        ht_count
    ));

    // Phase 2: CxbxDB patterns — filtered by XDK build version.
    // scan_table skips patterns whose min_version > xdk_build.
    let phase1_count = matches.len();
    scan_table(
        guest_base,
        &d3d_ranges,
        scan_end,
        cxbx_patterns,
        true,
        xdk_build,
        profile,
        &mut matches,
    );
    // Dedup: if a CxbxDB pattern matches the same guest address as a hand-tuned one, keep the hand-tuned one
    matches.dedup_by(|a, b| {
        if a.guest_addr == b.guest_addr {
            true // Keep b (the earlier hand-tuned entry)
        } else {
            false
        }
    });
    let phase2_count = matches.len() - phase1_count;
    debug_log(&format!(
        "[OOVPA] Phase 2 (CxbxDB, version-filtered): {} new matches (total {})",
        phase2_count,
        matches.len()
    ));

    // Phase 2b: XbSymbolDatabase patterns — the full Cxbx-R pattern database.
    // Select LTCG or standard patterns based on XBE's D3D8 library flags.
    // LTCG changes function prologues/epilogues — scanning wrong set causes massive false positives.
    let phase2b_start = matches.len();
    let d3d8_patterns = if d3d8_is_ltcg {
        debug_log(&format!(
            "[OOVPA] D3D8 is LTCG — scanning database::d3d8_ltcg::xdk{} patterns",
            xdk_build
        ));
        database::d3d8_ltcg_patterns_for_xdk(xdk_build as u32)
    } else {
        debug_log(&format!(
            "[OOVPA] D3D8 is standard — scanning database::d3d8::xdk{} patterns",
            xdk_build
        ));
        database::d3d8_patterns_for_xdk(xdk_build as u32)
    };
    scan_table(
        guest_base,
        &d3d_ranges,
        scan_end,
        d3d8_patterns,
        true,
        xdk_build,
        profile,
        &mut matches,
    );
    // Dedup: keep earlier matches (hand-tuned > CxbxDB > XbSymDB)
    matches.sort_by_key(|m| m.guest_addr);
    matches.dedup_by(|a, b| {
        if a.guest_addr == b.guest_addr {
            true
        } else {
            false
        }
    });
    let phase2b_count = matches.len() - phase2b_start;
    debug_log(&format!(
        "[OOVPA] Phase 2b (XbSymDB {}): {} new matches (total {})",
        if d3d8_is_ltcg { "LTCG" } else { "Standard" },
        phase2b_count,
        matches.len()
    ));

    // Phase 2c: DSound patterns — mass-stub all DirectSound functions.
    // Every DSound function returns DS_OK (0) through the generic HLE dispatcher.
    let phase2c_start = matches.len();
    if profile == OovpaScanProfile::Full {
        let dsound_patterns = database::dsound_patterns_for_xdk(xdk_build as u32);
        scan_table(
            guest_base,
            &dsound_ranges,
            scan_end,
            dsound_patterns,
            true,
            xdk_build,
            profile,
            &mut matches,
        );
    } else {
        debug_log(&format!(
            "[OOVPA] Phase 2c (DSound): deferred in profile={}",
            profile.name()
        ));
    }
    matches.sort_by_key(|m| m.guest_addr);
    matches.dedup_by(|a, b| a.guest_addr == b.guest_addr);
    let phase2c_count = matches.len() - phase2c_start;
    debug_log(&format!(
        "[OOVPA] Phase 2c (DSound): {} new matches (total {})",
        phase2c_count,
        matches.len()
    ));

    // Phase 2d: Xapi patterns — stub XAPI functions.
    let phase2d_start = matches.len();
    let xapi_patterns = database::xapi_patterns_for_xdk(xdk_build as u32);
    scan_table(
        guest_base,
        &xapi_ranges,
        scan_end,
        xapi_patterns,
        true,
        xdk_build,
        profile,
        &mut matches,
    );
    matches.sort_by_key(|m| m.guest_addr);
    matches.dedup_by(|a, b| a.guest_addr == b.guest_addr);
    let phase2d_count = matches.len() - phase2d_start;
    debug_log(&format!(
        "[OOVPA] Phase 2d (Xapi): {} new matches (total {})",
        phase2d_count,
        matches.len()
    ));

    // Phase 2e: XGraphic/XGRPH helper patterns.
    let phase2e_start = matches.len();
    let xgraphic_patterns = database::xgraphic_patterns_for_xdk(xdk_build as u32);
    scan_table(
        guest_base,
        &xgraphic_ranges,
        scan_end,
        xgraphic_patterns,
        true,
        xdk_build,
        profile,
        &mut matches,
    );
    matches.sort_by_key(|m| m.guest_addr);
    matches.dedup_by(|a, b| a.guest_addr == b.guest_addr);
    let phase2e_count = matches.len() - phase2e_start;
    debug_log(&format!(
        "[OOVPA] Phase 2e (XGraphic): {} new matches (total {})",
        phase2e_count,
        matches.len()
    ));

    // Phase 3: Universal _SEH_prolog/_SEH_epilog detection — DISABLED.
    // Was matching hundreds of individual SEH call sites, not just the helper functions.
    // The HLE handler assumes it's the single _SEH_prolog at 0x002B7B28, not inline code.
    // Re-enable after HLE handler is generalized for inline SEH sites.
    let _seh_prolog_sig: &[u8] = &[
        0x64, 0xA1, 0x00, 0x00, 0x00, 0x00, // mov eax, fs:[0]
        0x50, // push eax
        0x8B, 0x44, 0x24, 0x10, // mov eax, [esp+0x10]
        0x89, 0x6C, 0x24, 0x10, // mov [esp+0x10], ebp
        0x8D, 0x6C, 0x24, 0x10, // lea ebp, [esp+0x10]
    ];
    let _seh_epilog_sig: &[u8] = &[
        0x8B, 0x4D, 0xF0, // mov ecx, [ebp-0x10]
        0x64, 0x89, 0x0D, 0x00, 0x00, 0x00, 0x00, // mov fs:[0], ecx
        0x59, // pop ecx
        0x5F, // pop edi
        0x5E, // pop esi
        0x5B, // pop ebx
    ];
    let _seh_count = 0u32;
    // Phase 3 scanning disabled — see comment above.

    // Phase 4a: Remove functions that must NEVER be HLE'd.
    // These are CRT startup, kernel wrappers, and threading functions that must run
    // natively through AOT. Stubbing them with return 0 prevents the game from booting.
    // Cxbx-R has real HLE implementations for these; we don't, so blocking is correct.
    let must_run_natively: &[&str] = &[
        // CRT startup chain — game never boots if these are intercepted
        "mainCRTStartup",
        "mainXapiStartup",
        "XapiInitProcess",
        "_cinit",
        "_rtinit",
        "XapiThreadStartup",
        "XapiFiberStartup",
        // Kernel call wrappers — these forward to ordinals, must not be stubbed
        "CreateThread",
        "CreateEventA",
        "OpenEventA",
        "SetEvent",
        "ResetEvent",
        "PulseEvent",
        "CloseHandle",
        "ExitThread",
        "GetExitCodeThread",
        "CreateMutexA",
        "SetThreadPriority",
        "SetThreadPriorityBoost",
        "GetThreadPriority",
        "CreateFiber",
        "DeleteFiber",
        "SwitchToFiber",
        "ConvertThreadToFiber",
        "RaiseException",
        "QueryPerformanceCounter",
        "SignalObjectAndWait",
        "QueueUserAPC",
        "SwitchToThread",
        // Error handling — needed for kernel error propagation
        "GetLastError",
        "SetLastError",
        "XapiSetLastNTError",
        "UnhandledExceptionFilter",
        "XapiFormatObjectAttributes",
        "XapiCallThreadNotifyRoutines",
        "XRegisterThreadNotifyRoutine",
        // File I/O wrappers — must go through kernel file ordinals
        "ReadFileEx",
        "WriteFileEx",
        "GetOverlappedResult",
        "MoveFileA",
    ];
    let before_native = matches.len();
    matches.retain(|m| !must_run_natively.contains(&m.pattern_name));
    let native_removed = before_native - matches.len();
    if native_removed > 0 {
        debug_log(&format!(
            "[OOVPA] Removed {} must-run-natively functions (CRT/kernel wrappers)",
            native_removed
        ));
    }

    // Phase 4b: Remove known false positives from CxbxDB scan.
    // These are game-code addresses that happen to match D3D byte patterns
    // (verified by SymbolCache comparison). Hooking them causes wrong HLE dispatch.
    let false_positives: &[(u32, &str)] = &[
        (0x000476A0, "D3DDevice_Swap"), // Spider-Man: game code, not D3D Swap
        (0x002EC78C, "D3DDevice_SetRenderState_FrontFace"), // Spider-Man XDK 4134: mid-function false match; Cxbx-R labels nearby 0x002EC780 as VertexBlend
    ];
    let before_fp = matches.len();
    matches.retain(|m| {
        !false_positives
            .iter()
            .any(|(addr, name)| m.guest_addr == *addr && m.pattern_name == *name)
    });
    let removed = before_fp - matches.len();
    if removed > 0 {
        debug_log(&format!(
            "[OOVPA] Removed {} known false-positive match(es)",
            removed
        ));
    }

    // Phase 5: SymbolCache map file — fill gaps with known-good addresses.
    // Cxbx-R's SymbolCache provides ground-truth function addresses for this XBE.
    // Any symbol not already matched by pattern scanning gets added as a manual hook.
    // This gives us 100% coverage for games that have been analyzed by Cxbx-R.
    let map_fixture_path = find_msvc_map_symbol_path();
    let map_fixture_active = map_fixture_path.is_some();
    let toml_fixture_active = symbol_fixture_toml_enabled();
    let symbol_cache_path = if map_fixture_active {
        debug_log(
            "[OOVPA] Phase 5: legacy text SymbolCache disabled because MSVC .map fixture is active",
        );
        None
    } else if toml_fixture_active {
        debug_log(
            "[OOVPA] Phase 5: legacy text SymbolCache disabled because TOML fixture is active",
        );
        None
    } else {
        find_symbol_cache_path(entry_point, title_id, xdk_build, d3d8_is_ltcg)
    };
    if let Some(cache_path) = symbol_cache_path {
        let phase5_start = matches.len();
        load_symbol_cache(
            &cache_path,
            guest_base,
            scan_end,
            &must_run_natively,
            &mut matches,
        );
        let phase5_count = matches.len() - phase5_start;
        if phase5_count > 0 {
            debug_log(&format!(
                "[OOVPA] Phase 5 (SymbolCache map): {} new hooks from map file (total {})",
                phase5_count,
                matches.len()
            ));
        }
    }

    if let Some(map_path) = map_fixture_path {
        let phase5_start = matches.len();
        load_msvc_map_symbol_fixture(
            &map_path,
            guest_base,
            scan_end,
            entry_point,
            &scan_ranges,
            &must_run_natively,
            &mut matches,
        );
        let phase5_count = matches.len() - phase5_start;
        debug_log(&format!(
            "[OOVPA] Phase 5M (MSVC .map fixture): {} net new hooks from {} (total {})",
            phase5_count,
            map_path,
            matches.len()
        ));
    }

    if let Some(toml_path) = find_symbol_fixture_toml_path(entry_point) {
        let phase5_start = matches.len();
        load_symbol_fixture_toml(
            &toml_path,
            guest_base,
            scan_end,
            &must_run_natively,
            &mut matches,
        );
        let phase5_count = matches.len() - phase5_start;
        debug_log(&format!(
            "[OOVPA] Phase 5T (TOML SymbolCache fixture): {} new hooks from {} (total {})",
            phase5_count,
            toml_path,
            matches.len()
        ));
    }

    // Phase 6: Prologue validation — reject any match whose first byte is
    // not a plausible function entry. OOVPA patterns can legitimately match
    // mid-function instructions (e.g. `8B 4C 24 04 = mov ecx, [esp+4]`) when
    // the signature bytes happen to align there. Planting INT3 on a
    // mid-function byte never fires (guest execution arrives only at the
    // real entry) and consumes a hook slot that should have gone to the
    // correct address. Filter matches here so plant_hooks never sees them.
    //
    // Keep the byte-class whitelist in lock-step with the validator in
    // `validate_against_symbol_cache` (see has_prologue check around line
    // 1028) and the SymbolCache warn-only check at line 481.
    let before_prologue = matches.len();
    matches.retain(|m| {
        // Bounds check — defensive, matches should already be in-range.
        if m.guest_addr < 0x1000 || m.guest_addr >= scan_end {
            return true; // let other passes decide
        }
        let b0 = unsafe { *guest_base.add(m.guest_addr as usize) };
        let b1 = unsafe { *guest_base.add((m.guest_addr + 1) as usize) };
        let has_prologue = b0 == 0x55                     // push ebp
            || (b0 >= 0x50 && b0 <= 0x57)                 // push reg
            || (b0 == 0x83 && b1 == 0xEC)                 // sub esp, imm8
            || (b0 == 0x81 && b1 == 0xEC)                 // sub esp, imm32
            || (b0 == 0x8B && b1 == 0xFF)                 // mov edi, edi (hotpatch)
            || b0 == 0xA1                                 // mov eax, [imm32]
            || (b0 == 0x8B && b1 == 0x44)                 // mov eax, [esp+disp8]
            || (b0 == 0x8B && b1 == 0x4C)                 // mov ecx, [esp+disp8] — stdcall thunk
            || (b0 == 0x8B && b1 == 0x54)                 // mov edx, [esp+disp8] — stdcall thunk (Spider-Man D3D_CreateStandAloneSurface)
            || (b0 == 0x8B && b1 == 0x0D)                 // mov ecx, [imm32] — LTCG entry
            || (b0 == 0xFF && b1 == 0x74)                 // push [esp+N] — stdcall thunk
            || b0 == 0x68                                 // push imm32 — SEH prologue / thunk
            || b0 == 0xE8                                 // call rel32 — forwarder thunk
            || b0 == 0x6A                                 // push imm8
            || (b0 == 0x33 && b1 == 0xC0)                 // xor eax, eax
            || b0 == 0x64                                 // fs: prefix
            || b0 == 0xCC                                 // INT3 (already hooked)
            || b0 == 0xB8                                 // mov eax, imm32 (common entry)
            || b0 == 0xE9                                 // jmp rel32 (thunk)
            || b0 == 0xEB;                                // jmp rel8 (thunk)
        if !has_prologue {
            if m.is_cxbx {
                debug_log(&format!(
                    "[OOVPA] WARN_PROLOGUE(trusted): {} at 0x{:08X} — first bytes: {:02X} {:02X} (keeping SymbolCache/TOML entry)",
                    m.pattern_name, m.guest_addr, b0, b1
                ));
                return true;
            }
            debug_log(&format!(
                "[VALIDATE] SKIPPED_BAD_PROLOGUE: {} at 0x{:08X} — first bytes: {:02X} {:02X} (not a function entry)",
                m.pattern_name, m.guest_addr, b0, b1
            ));
            if oovpa_diff_enabled() {
                debug_log(&format!(
                    "[OOVPA-DIFF] [PROLOGUE_REJECTED] {} at 0x{:08X} bytes={:02X} {:02X}",
                    m.pattern_name, m.guest_addr, b0, b1
                ));
            }
        }
        has_prologue
    });
    let prologue_removed = before_prologue - matches.len();
    if prologue_removed > 0 {
        debug_log(&format!(
            "[OOVPA] Prologue filter: removed {} hook(s) matching mid-function bytes (of {} matches before filter)",
            prologue_removed, before_prologue
        ));
    }

    // Phase 7: LTCG SDK-bounds filter (Spider-Man-era D3D LTCG hooks).
    //
    // LTCG-variant D3D/D3DX patterns (names containing "__LTCG_") live only
    // inside Microsoft's statically-linked SDK sections — D3D, D3DX, and
    // XGRPH. Their byte prologs (`83 EC <imm8>; ...; 81 C1 00 00 F8 FF 89`)
    // are short and use very common x86 opcodes, so a raw scan across the
    // entire XBE produces many false-positive hits inside game code
    // (platformer physics, string builders, cache management — anywhere a
    // stack frame happens to be followed by `add reg, const`).
    //
    // Drop LTCG matches that land outside the union of SDK section bounds.
    // Non-LTCG patterns keep full scan coverage — they have stronger
    // prologue signatures (more bytes, more specific byte values) and
    // legitimately can appear anywhere the SDK linker placed them.
    //
    // Ranges are half-open `[start, end)`. The match is kept iff its
    // guest_addr is inside any D3D/D3DX/XGRPH range from the current XBE.
    let before_ltcg = matches.len();
    let mut kept_count = 0u32;
    let mut dropped_count = 0u32;
    matches.retain(|m| {
        if !m.pattern_name.contains("__LTCG_") {
            return true; // non-LTCG hooks untouched
        }
        let in_sdk = d3d_ranges
            .iter()
            .any(|&(start, end)| m.guest_addr >= start && m.guest_addr < end);
        if in_sdk {
            debug_log(&format!(
                "[OOVPA-LTCG-FILTER] kept {} @ guest=0x{:08X} (in SDK range)",
                m.pattern_name, m.guest_addr
            ));
            kept_count += 1;
            true
        } else {
            debug_log(&format!(
                "[OOVPA-LTCG-FILTER] dropped {} @ guest=0x{:08X} (out of SDK bounds)",
                m.pattern_name, m.guest_addr
            ));
            dropped_count += 1;
            false
        }
    });
    let ltcg_removed = before_ltcg - matches.len();
    if before_ltcg > 0 && (kept_count > 0 || dropped_count > 0) {
        debug_log(&format!(
            "[OOVPA-LTCG-FILTER] summary: {} kept, {} dropped, {} non-LTCG untouched",
            kept_count,
            dropped_count,
            matches.len() - kept_count as usize
        ));
    }
    let _ = ltcg_removed;

    debug_log(&format!(
        "[OOVPA] Scan complete: {} functions matched",
        matches.len()
    ));
    infer_g_pdevice_global(guest_base, scan_end, &matches);
    matches
}

fn valid_guest_global(addr: u32, scan_end: u32) -> bool {
    (0x0001_0000..0x0080_0000).contains(&addr) && addr < scan_end && (addr & 3) == 0
}

fn read_u32_le(guest_base: *const u8, addr: u32, scan_end: u32) -> Option<u32> {
    if addr.checked_add(4)? > scan_end {
        return None;
    }
    Some(unsafe {
        u32::from_le_bytes([
            *guest_base.add(addr as usize),
            *guest_base.add(addr as usize + 1),
            *guest_base.add(addr as usize + 2),
            *guest_base.add(addr as usize + 3),
        ])
    })
}

fn extract_abs_from_mov_load(
    guest_base: *const u8,
    scan_end: u32,
    start: u32,
    len: u32,
) -> Option<(u32, u32)> {
    let end = start.saturating_add(len).min(scan_end);
    let mut off = start;
    while off.saturating_add(6) <= end {
        let b0 = unsafe { *guest_base.add(off as usize) };
        let b1 = unsafe { *guest_base.add(off as usize + 1) };
        if b0 == 0x8B && matches!(b1, 0x0D | 0x1D | 0x2D | 0x35 | 0x3D) {
            if let Some(addr) = read_u32_le(guest_base, off + 2, scan_end) {
                if valid_guest_global(addr, scan_end) {
                    return Some((addr, off - start));
                }
            }
        }
        off = off.saturating_add(1);
    }
    None
}

pub(crate) fn infer_g_pdevice_global(guest_base: *const u8, scan_end: u32, matches: &[OovpaMatch]) {
    fn by_name<'a>(
        matches: &'a [OovpaMatch],
        wanted: &'static str,
    ) -> impl Iterator<Item = &'a OovpaMatch> {
        matches
            .iter()
            .filter(move |m| normalize_func_name(m.pattern_name) == wanted)
    }

    let mut found: Option<(u32, u32, &'static str, u32)> = None;

    for m in by_name(matches, "D3DDevice_Clear") {
        if m.guest_addr.saturating_add(11) <= scan_end {
            let b5 = unsafe { *guest_base.add(m.guest_addr as usize + 5) };
            let b6 = unsafe { *guest_base.add(m.guest_addr as usize + 6) };
            if b5 == 0x8B && b6 == 0x2D {
                if let Some(addr) = read_u32_le(guest_base, m.guest_addr + 7, scan_end) {
                    if valid_guest_global(addr, scan_end) {
                        found = Some((addr, m.guest_addr, m.pattern_name, 7));
                        break;
                    }
                }
            }
        }
    }

    if found.is_none() {
        for m in by_name(matches, "D3DDevice_Swap") {
            if let Some((addr, off)) =
                extract_abs_from_mov_load(guest_base, scan_end, m.guest_addr, 24)
            {
                found = Some((addr, m.guest_addr, m.pattern_name, off + 2));
                break;
            }
        }
    }

    if found.is_none() {
        for m in matches
            .iter()
            .filter(|m| normalize_func_name(m.pattern_name).starts_with("D3DDevice_"))
        {
            if let Some((addr, off)) =
                extract_abs_from_mov_load(guest_base, scan_end, m.guest_addr, 32)
            {
                found = Some((addr, m.guest_addr, m.pattern_name, off + 2));
                break;
            }
        }
    }

    if let Some((addr, source, name, imm_off)) = found {
        G_PDEVICE_DYNAMIC.store(addr, std::sync::atomic::Ordering::Relaxed);
        debug_log(&format!(
            "[OOVPA] Dynamic g_pDevice inferred: global=0x{:08X} from {} guest=0x{:08X}+0x{:X}",
            addr, name, source, imm_off
        ));
    } else {
        debug_log("[OOVPA] Dynamic g_pDevice not inferred during scan; CreateDevice TAP/HLE will discover it later");
    }
}

#[derive(Debug, Default)]
struct SymbolCacheMeta {
    title_id: Option<u32>,
    build_version: Option<u16>,
    has_d3d8_ltcg: bool,
}

fn parse_hex_u32(s: &str) -> Option<u32> {
    let s = s.trim().trim_matches('"');
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    u32::from_str_radix(s, 16).ok()
}

fn parse_symbol_cache_meta(content: &str) -> SymbolCacheMeta {
    let mut meta = SymbolCacheMeta::default();
    let mut section = "";

    for raw in content.lines() {
        let line = raw.split(';').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = &line[1..line.len() - 1];
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"');

        if section.eq_ignore_ascii_case("Certificate") && key.eq_ignore_ascii_case("TitleIDHex") {
            meta.title_id = parse_hex_u32(value);
        } else if section.eq_ignore_ascii_case("Libs") {
            if key.eq_ignore_ascii_case("BuildVersion") {
                meta.build_version = value.parse::<u16>().ok();
            } else if key.eq_ignore_ascii_case("D3D8LTCG") {
                meta.has_d3d8_ltcg = true;
                if meta.build_version.is_none() {
                    meta.build_version = value.parse::<u16>().ok();
                }
            }
        }
    }

    meta
}

fn find_cxbxr_symbol_cache_by_title(
    title_id: u32,
    xdk_build: u16,
    d3d8_is_ltcg: bool,
) -> Option<String> {
    let mut dirs = Vec::new();
    if let Ok(dir) = std::env::var("RUSTEMU_CXBXR_SYMBOLCACHE_DIR") {
        let dir = dir.trim();
        if !dir.is_empty() {
            dirs.push(dir.to_string());
        }
    }
    dirs.push(r"./cxbxr-docker-build\SymbolCache".to_string());
    dirs.push(r"./CxbxReloaded/SymbolCache".to_string());
    dirs.sort();
    dirs.dedup();

    let mut paths = Vec::new();
    for dir in dirs {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        paths.extend(
            read_dir
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e.eq_ignore_ascii_case("ini"))
                        .unwrap_or(false)
                }),
        );
    }
    paths.sort();

    let mut fallback = None;
    for path in paths {
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let meta = parse_symbol_cache_meta(&content);
        if meta.title_id != Some(title_id) {
            continue;
        }

        let build_matches = xdk_build == 0 || meta.build_version == Some(xdk_build);
        let ltcg_matches = meta.has_d3d8_ltcg == d3d8_is_ltcg;
        if build_matches && ltcg_matches {
            return Some(path.to_string_lossy().to_string());
        }

        // Keep a title-only fallback for older cache files with incomplete
        // metadata, but prefer exact build/LTCG matches whenever available.
        if fallback.is_none() {
            fallback = Some(path.to_string_lossy().to_string());
        }
    }

    fallback
}

/// Find the SymbolCache file for this game.
fn find_symbol_cache_path(
    entry_point: u32,
    title_id: Option<u32>,
    xdk_build: u16,
    d3d8_is_ltcg: bool,
) -> Option<String> {
    for env_name in ["RUSTEMU_OOVPA_SYMBOLS_INI", "RUSTEMU_OOVPA_SYMBOLS_CACHE"] {
        if let Ok(path) = std::env::var(env_name) {
            let path = path.trim();
            if !path.is_empty() && std::path::Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
    }

    if env_flag("RUSTEMU_OOVPA_DISABLE_SYMBOLCACHE") {
        return None;
    }

    if let Some(title_id) = title_id {
        if let Some(path) = find_cxbxr_symbol_cache_by_title(title_id, xdk_build, d3d8_is_ltcg) {
            return Some(path);
        }
    }

    // Legacy Spider-Man export. Entry point alone is not unique across retail
    // games, so only use this fallback for non-LTCG XDK 4134 runs.
    if entry_point == 0x002A_9C38 && xdk_build == 4134 && !d3d8_is_ltcg {
        let path = r"./xdk/build\out\spiderman_symbols.txt";
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}

fn find_symbol_fixture_toml_path(entry_point: u32) -> Option<String> {
    if let Ok(path) = std::env::var("RUSTEMU_OOVPA_SYMBOLS_TOML") {
        let path = path.trim();
        if !path.is_empty() && std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    if entry_point == 0x002A_9C38 && env_flag("RUSTEMU_SPIDEY_4134_TOML") {
        let path = r"./spiderman_4134_symbols.toml";
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
        debug_log(&format!(
            "[OOVPA] RUSTEMU_SPIDEY_4134_TOML set but fixture missing: {}",
            path
        ));
    }

    if entry_point == 0x0003_E5AB && env_flag("RUSTEMU_DOOM_5849_TOML") {
        let path = r"./classic_doom_5849_symbols.toml";
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
        debug_log(&format!(
            "[OOVPA] RUSTEMU_DOOM_5849_TOML set but fixture missing: {}",
            path
        ));
    }

    None
}

fn find_msvc_map_symbol_path() -> Option<String> {
    for env_name in ["RUSTEMU_OOVPA_MAP_SYMBOLS", "RUSTEMU_OOVPA_SYMBOLS_MAP"] {
        if let Ok(path) = std::env::var(env_name) {
            let path = path.trim();
            if !path.is_empty() && std::path::Path::new(path).exists() {
                return Some(path.to_string());
            }
            if !path.is_empty() {
                debug_log(&format!(
                    "[OOVPA] {} set but MSVC .map file is missing: {}",
                    env_name, path
                ));
            }
        }
    }

    None
}

fn parse_symbol_fixture_toml(content: &str) -> Vec<(u32, String)> {
    let mut in_symbols = false;
    let mut entries = Vec::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("[symbols.") {
            in_symbols = true;
            continue;
        }
        if line.starts_with('[') {
            in_symbols = false;
            continue;
        }
        if !in_symbols {
            continue;
        }

        let Some((name, value_and_comment)) = line.split_once('=') else {
            continue;
        };
        let name = name.trim().trim_matches('"');
        if name.is_empty() {
            continue;
        }
        let value = value_and_comment
            .split('#')
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('"');
        let Some(hex) = value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
        else {
            continue;
        };
        if let Ok(addr) = u32::from_str_radix(hex, 16) {
            entries.push((addr, name.to_string()));
        }
    }

    entries
}

fn load_msvc_map_symbol_fixture(
    path: &str,
    guest_base: *const u8,
    scan_end: u32,
    entry_point: u32,
    scan_ranges: &[OovpaScanRange],
    must_run_natively: &[&str],
    matches: &mut Vec<OovpaMatch>,
) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            debug_log(&format!("[OOVPA] Cannot read MSVC .map {}: {}", path, e));
            return;
        }
    };

    let entries = parse_msvc_map_entries(&content, entry_point, scan_ranges);
    debug_log(&format!(
        "[OOVPA-MAP] Parsed {} D3D public symbol(s) from {}",
        entries.len(),
        path
    ));
    load_symbol_entries(
        path,
        entries,
        guest_base,
        scan_end,
        must_run_natively,
        matches,
    );
}

#[derive(Debug, Clone)]
struct MsvcMapPublic {
    segment: u32,
    offset: u32,
    name: String,
    rva_base: u32,
    object: String,
}

fn parse_msvc_map_entries(
    content: &str,
    entry_point: u32,
    scan_ranges: &[OovpaScanRange],
) -> Vec<(u32, String)> {
    let publics = parse_msvc_map_publics(content);
    if publics.is_empty() {
        debug_log("[OOVPA-MAP] No public symbols found in MSVC .map file");
        return Vec::new();
    }

    let Some(delta) = infer_msvc_map_delta(&publics, entry_point, scan_ranges) else {
        debug_log("[OOVPA-MAP] Could not infer Rva+Base to XBE-VA delta; map fixture ignored");
        return Vec::new();
    };

    let mut entries = Vec::new();
    let mut skipped_non_d3d = 0u32;
    let mut skipped_cpp = 0u32;
    let mut skipped_underflow = 0u32;
    for public in publics {
        if !is_d3d8_map_public(&public) {
            skipped_non_d3d += 1;
            continue;
        }
        let Some(name) = undecorate_msvc_c_symbol(&public.name) else {
            skipped_cpp += 1;
            continue;
        };
        let Some(guest_addr) = public.rva_base.checked_sub(delta) else {
            skipped_underflow += 1;
            continue;
        };
        entries.push((guest_addr, name));
    }

    entries.sort_by_key(|(addr, name)| (*addr, name.clone()));
    entries.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    debug_log(&format!(
        "[OOVPA-MAP] Normalized MSVC .map delta=0x{:08X}; kept={} skipped_non_d3d={} skipped_cpp={} skipped_underflow={}",
        delta,
        entries.len(),
        skipped_non_d3d,
        skipped_cpp,
        skipped_underflow
    ));
    entries
}

fn parse_msvc_map_publics(content: &str) -> Vec<MsvcMapPublic> {
    let mut publics = Vec::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 || !parts[0].contains(':') {
            continue;
        }

        let Some((seg, off)) = parts[0].split_once(':') else {
            continue;
        };
        let Ok(segment) = u32::from_str_radix(seg, 16) else {
            continue;
        };
        let Ok(offset) = u32::from_str_radix(off, 16) else {
            continue;
        };
        let Ok(rva_base) = u32::from_str_radix(parts[2], 16) else {
            continue;
        };
        let object_idx = if parts.get(3) == Some(&"f") { 4 } else { 3 };
        let object = parts.get(object_idx).copied().unwrap_or("").to_string();

        publics.push(MsvcMapPublic {
            segment,
            offset,
            name: parts[1].to_string(),
            rva_base,
            object,
        });
    }

    publics
}

fn infer_msvc_map_delta(
    publics: &[MsvcMapPublic],
    entry_point: u32,
    scan_ranges: &[OovpaScanRange],
) -> Option<u32> {
    if entry_point != 0 {
        if let Some(entry_public) = publics
            .iter()
            .find(|p| undecorate_msvc_c_symbol(&p.name).as_deref() == Some("mainCRTStartup"))
        {
            if let Some(delta) = entry_public.rva_base.checked_sub(entry_point) {
                debug_log(&format!(
                    "[OOVPA-MAP] Delta inferred from mainCRTStartup: map=0x{:08X} entry=0x{:08X}",
                    entry_public.rva_base, entry_point
                ));
                return Some(delta);
            }
        }
    }

    let first_exec_start = scan_ranges
        .iter()
        .map(|r| r.start)
        .min()
        .filter(|start| *start != 0)?;
    let first_section_public = publics
        .iter()
        .filter(|p| p.segment != 0 && p.offset == 0)
        .min_by_key(|p| (p.segment, p.rva_base))?;
    let delta = first_section_public
        .rva_base
        .checked_sub(first_exec_start)?;
    debug_log(&format!(
        "[OOVPA-MAP] Delta inferred from first public: {} map=0x{:08X} xbe_start=0x{:08X}",
        first_section_public.name, first_section_public.rva_base, first_exec_start
    ));
    Some(delta)
}

fn is_d3d8_map_public(public: &MsvcMapPublic) -> bool {
    let object_is_d3d8 = public.object.to_ascii_lowercase().starts_with("d3d8:");
    let name = public.name.as_str();
    object_is_d3d8
        && (name.starts_with("_D3D") || name.starts_with("_Direct3D") || name.starts_with("@D3D"))
        && !name.starts_with("__imp__")
}

fn undecorate_msvc_c_symbol(raw: &str) -> Option<String> {
    if raw.starts_with('?') || raw.starts_with("__imp__") {
        return None;
    }

    let mut name = raw.trim();
    if let Some(stripped) = name.strip_prefix('_') {
        name = stripped;
    } else if let Some(stripped) = name.strip_prefix('@') {
        name = stripped;
    }

    if let Some(pos) = name.rfind('@') {
        let suffix = &name[pos + 1..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            name = &name[..pos];
        }
    }

    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn load_symbol_fixture_toml(
    path: &str,
    guest_base: *const u8,
    scan_end: u32,
    must_run_natively: &[&str],
    matches: &mut Vec<OovpaMatch>,
) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            debug_log(&format!(
                "[OOVPA] Cannot read TOML symbol fixture {}: {}",
                path, e
            ));
            return;
        }
    };

    let entries = parse_symbol_fixture_toml(&content);
    load_symbol_entries(
        path,
        entries,
        guest_base,
        scan_end,
        must_run_natively,
        matches,
    );
}

/// Load SymbolCache and create manual hooks for functions not already matched.
fn load_symbol_cache(
    path: &str,
    guest_base: *const u8,
    scan_end: u32,
    must_run_natively: &[&str],
    matches: &mut Vec<OovpaMatch>,
) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            debug_log(&format!("[OOVPA] Cannot read SymbolCache {}: {}", path, e));
            return;
        }
    };

    let entries = parse_symbol_cache_entries(&content);
    load_symbol_entries(
        path,
        entries,
        guest_base,
        scan_end,
        must_run_natively,
        matches,
    );
}

fn parse_symbol_cache_entries(content: &str) -> Vec<(u32, String)> {
    let mut entries = Vec::new();
    let mut in_symbols = false;

    for raw in content.lines() {
        let line = raw.split(';').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_symbols = line[1..line.len() - 1].eq_ignore_ascii_case("Symbols");
            continue;
        }

        if in_symbols {
            if let Some((name, value_and_comment)) = line.split_once('=') {
                let name = name.trim().trim_matches('"');
                let value = value_and_comment
                    .split('#')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"');
                if !name.is_empty() {
                    if let Some(addr) = parse_hex_u32(value) {
                        entries.push((addr, name.to_string()));
                    }
                }
                continue;
            }
        }

        if let Some((addr_str, name)) = line.split_once(" -> ") {
            let addr_str = addr_str
                .trim()
                .trim_start_matches("0x")
                .trim_start_matches("0X");
            if let Ok(addr) = u32::from_str_radix(addr_str, 16) {
                entries.push((addr, name.trim().to_string()));
            }
        }
    }

    entries
}

fn load_symbol_entries(
    label: &str,
    entries: Vec<(u32, String)>,
    guest_base: *const u8,
    scan_end: u32,
    must_run_natively: &[&str],
    matches: &mut Vec<OovpaMatch>,
) {
    let mut added = 0u32;
    let mut replaced = 0u32;
    let mut skipped_native = 0u32;
    let mut skipped_data = 0u32;
    let mut skipped_existing = 0u32;

    for (addr, name) in entries {
        let name = name.trim();
        // Skip data symbols (offsets, globals, low addresses)
        if addr < 0x1000
            || name.contains("_OFFSET")
            || name.starts_with("D3DRS_")
            || name.starts_with("D3D_g_")
            || name.starts_with("g_")
            || name.starts_with("_tls")
        {
            skipped_data += 1;
            continue;
        }

        // Skip addresses in kernel range (above 0x80000000) or obviously invalid
        if addr >= 0x8000_0000 {
            continue;
        }

        if addr + 1 >= scan_end {
            skipped_data += 1;
            continue;
        }

        // Skip must-run-natively functions
        if must_run_natively.contains(&name) {
            skipped_native += 1;
            continue;
        }

        // SymbolCache is authoritative by address too. Pattern matching can
        // land on the correct entry point with the wrong SDK name, especially
        // across dense DSOUND COM wrappers where byte signatures are similar.
        // If the exact address is already matched, correct the name/argc in
        // place instead of keeping the stale OOVPA label from the cache.
        if let Some(idx) = matches.iter().position(|m| m.guest_addr == addr) {
            let argc = super::oovpa_scan::known_argc(name)
                .unwrap_or_else(|| super::oovpa_scan::auto_detect_argc(guest_base, addr, scan_end));
            let current = normalize_func_name(matches[idx].pattern_name);
            let desired = normalize_func_name(name);
            if current != desired || matches[idx].argc != argc {
                let old_name = matches[idx].pattern_name;
                let old_argc = matches[idx].argc;
                let static_name: &'static str = Box::leak(name.to_string().into_boxed_str());
                matches[idx].pattern_name = static_name;
                matches[idx].argc = argc;
                debug_log(&format!(
                    "[OOVPA] SymbolCache RENAME: 0x{:08X} {}({}) → {}({})",
                    addr, old_name, old_argc, name, argc
                ));
                replaced += 1;
            } else {
                skipped_existing += 1;
            }
            continue;
        }

        // SymbolCache is authoritative: if an OOVPA pattern matched the same
        // function name at a DIFFERENT address, the pattern match is a false
        // positive. Remove it and use the SymbolCache address instead.
        // This fixes LTCG false positives (e.g. CreateDevice at 0x19CE0 vs real 0x2F3BD0).
        let oovpa_idx = matches.iter().position(|m| {
            m.guest_addr != addr && {
                // Normalize both names for comparison (strip LTCG suffixes, trailing _N)
                let m_base = normalize_func_name(m.pattern_name);
                let sc_base = normalize_func_name(name);
                m_base == sc_base
            }
        });
        if let Some(idx) = oovpa_idx {
            let old_addr = matches[idx].guest_addr;
            debug_log(&format!(
                "[OOVPA] SymbolCache OVERRIDE: {} — OOVPA 0x{:08X} → SymCache 0x{:08X} (OOVPA was false positive)",
                name, old_addr, addr
            ));
            matches.remove(idx);
            replaced += 1;
        }

        // Prologue validation: warn but don't reject — SymbolCache is ground truth.
        // OOVPA pattern matches get rejected on bad prologue, but Cxbx-R's SymbolCache
        // addresses are verified by their analysis, so we trust them.
        let b0 = unsafe { *guest_base.add(addr as usize) };
        let b1 = unsafe { *guest_base.add((addr + 1) as usize) };
        let has_prologue = b0 == 0x55 || (b0 >= 0x50 && b0 <= 0x57)
            || (b0 == 0x83 && b1 == 0xEC) || (b0 == 0x81 && b1 == 0xEC)
            || (b0 == 0x8B && b1 == 0xFF) || b0 == 0xA1
            || (b0 == 0x8B && b1 == 0x44) || (b0 == 0x8B && b1 == 0x4C)
            || (b0 == 0x8B && b1 == 0x0D) // mov ecx, [imm32] — LTCG entry
            || b0 == 0x6A || (b0 == 0x33 && b1 == 0xC0)
            || b0 == 0x64 || b0 == 0xCC;
        if !has_prologue {
            debug_log(&format!(
                "[OOVPA] WARN_PROLOGUE(SymCache): {} at 0x{:08X} — {:02X} {:02X} (trusting SymbolCache)",
                name, addr, b0, b1
            ));
            // Don't reject — SymbolCache is authoritative
        }

        // Use known argc override if available, else auto-detect from ret N
        let argc = super::oovpa_scan::known_argc(name)
            .unwrap_or_else(|| super::oovpa_scan::auto_detect_argc(guest_base, addr, scan_end));

        // Leak the name string so it has 'static lifetime (these are loaded once at startup)
        let static_name: &'static str = Box::leak(name.to_string().into_boxed_str());

        matches.push(OovpaMatch {
            guest_addr: addr,
            host_offset: 0,
            original_byte: 0,
            pattern_name: static_name,
            argc,
            hle_mode: HleMode::Hle,
            // SymbolCache entries are Cxbx-R-derived SDK symbols, not Rustemu
            // hardcoded title hooks. Keep them in the normal planting path;
            // `is_manual=true` is reserved for MANUAL_HOOKS below plant_hooks().
            is_cxbx: true,
            is_manual: false,
            active: false,
            pending_rearm: false,
            call_count: 0,
        });
        added += 1;
    }

    debug_log(&format!(
        "[OOVPA] Symbol entries from {}: {} added, {} replaced (false positives/renames), {} already matched, {} native-blocked, {} data-skipped",
        label, added, replaced, skipped_existing, skipped_native, skipped_data
    ));
}

/// Normalize a function name for comparison: strip LTCG suffixes and trailing _N size suffix.
/// "Direct3D_CreateDevice_16__LTCG_eax4_ebx6" → "Direct3D_CreateDevice"
fn normalize_func_name(name: &str) -> &str {
    // Strip __LTCG suffix
    let base = if let Some(idx) = name.find("__LTCG") {
        &name[..idx]
    } else {
        name
    };
    // Strip trailing _N (1-2 digit number) size suffix
    if base.len() > 3 {
        if let Some(last_us) = base.rfind('_') {
            let suffix = &base[last_us + 1..];
            if suffix.len() <= 2 && !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
            {
                return &base[..last_us];
            }
        }
    }
    base
}

fn symbol_names_equivalent(a: &str, b: &str) -> bool {
    normalize_func_name(a) == normalize_func_name(b)
}

fn ranges_for_kinds(ranges: &[OovpaScanRange], kinds: &[OovpaScanRangeKind]) -> Vec<(u32, u32)> {
    ranges
        .iter()
        .filter(|r| kinds.contains(&r.kind) && r.start < r.end)
        .map(|r| (r.start, r.end))
        .collect()
}

fn fallback_ranges(primary: Vec<(u32, u32)>, fallback: &[(u32, u32)]) -> Vec<(u32, u32)> {
    if primary.is_empty() {
        fallback.to_vec()
    } else {
        primary
    }
}

fn total_range_bytes(ranges: &[(u32, u32)]) -> u32 {
    ranges
        .iter()
        .fold(0u32, |acc, (s, e)| acc.saturating_add(e.saturating_sub(*s)))
}

fn pattern_enabled_for_profile(name: &str, profile: OovpaScanProfile) -> bool {
    match profile {
        OovpaScanProfile::Full => true,
        OovpaScanProfile::Graphics => {
            is_graphics_symbol(name) || is_input_or_boot_xapi_symbol(name)
        }
        OovpaScanProfile::FastBoot => {
            is_fast_boot_d3d_symbol(name)
                || is_input_or_boot_xapi_symbol(name)
                || normalize_func_name(name) == "DirectSoundCreate"
        }
    }
}

fn is_graphics_symbol(name: &str) -> bool {
    let base = normalize_func_name(name);
    base.starts_with("D3D") || base.starts_with("Direct3D") || base.starts_with("CDevice")
}

fn is_fast_boot_d3d_symbol(name: &str) -> bool {
    let base = normalize_func_name(name);
    matches!(
        base,
        "Direct3D_CreateDevice"
            | "D3DDevice_Clear"
            | "D3DDevice_Swap"
            | "D3DDevice_Present"
            | "D3DDevice_Reset"
            | "D3DDevice_KickOff"
            | "CDevice_KickOff"
            | "D3D_KickOffAndWaitForIdle"
            | "D3D_KickOffAndWaitForIdle2"
            | "D3D_SetFence"
            | "D3D_BlockOnTime"
            | "D3DDevice_Begin"
            | "D3DDevice_BeginPush"
            | "D3DDevice_BeginPushBuffer"
            | "D3DDevice_EndPush"
            | "D3DDevice_DrawVertices"
            | "D3DDevice_DrawVerticesUP"
            | "D3DDevice_DrawIndexedVertices"
            | "D3DDevice_DrawIndexedVerticesUP"
            | "D3DDevice_SetVertexShader"
            | "D3DDevice_SetPixelShader"
            | "D3DDevice_SetTexture"
            | "D3DDevice_SetTexture_4"
            | "D3DDevice_SetRenderTarget"
            | "D3DDevice_SetStreamSource"
            | "D3DDevice_SetIndices"
            | "D3DDevice_SetTransform"
            | "D3DDevice_SetViewport"
            | "D3DDevice_SetScreenSpaceOffset"
            | "D3DDevice_SetVertexShaderConstant"
            | "D3DDevice_LoadVertexShader"
            | "D3DDevice_LoadVertexShaderProgram"
            | "D3DDevice_SelectVertexShader"
            | "D3DDevice_CreateVertexShader"
            | "D3DDevice_CreatePixelShader"
            | "D3DDevice_CreateTexture"
            | "D3D_CreateTexture"
            | "D3DDevice_CopyRects"
            | "D3DDevice_InsertFence"
            | "D3DDevice_IsFencePending"
            | "D3D_BlockOnFence"
            | "D3D_BlockOnResource"
            | "D3DDevice_BlockUntilVerticalBlank"
            | "D3DDevice_SetRenderState_ZEnable"
            | "D3DDevice_SetRenderState_CullMode"
            | "D3DDevice_SetRenderState_FillMode"
            | "D3DDevice_SetRenderState_TextureFactor"
            | "D3DDevice_SetRenderState_BackFillMode"
            | "D3DDevice_SetRenderState_FogColor"
            | "D3DDevice_SetRenderState_FrontFace"
            | "D3DDevice_SetRenderState_LineWidth"
            | "D3DDevice_SetRenderState_ZBias"
            | "D3DDevice_SetRenderStateInline"
            | "D3DDevice_SetRenderStateInline__GenericFragment"
            | "D3DDevice_SetTextureStageStateNotInline"
            | "CDevice_SetStateUP"
            | "CDevice_SetStateVB"
            | "CDevice_InitializeFrameBuffers"
            | "CDevice_FreeFrameBuffers"
    )
}

fn is_input_or_boot_xapi_symbol(name: &str) -> bool {
    let base = normalize_func_name(name);
    matches!(
        base,
        "XInitDevices"
            | "XGetDevices"
            | "XGetDeviceChanges"
            | "XInputOpen"
            | "XInputClose"
            | "XInputPoll"
            | "XInputGetState"
            | "XInputSetState"
            | "XInputGetCapabilities"
            | "XInputGetDeviceDescription"
            | "XGetDeviceEnumerationStatus"
            | "XapiMapLetterToDirectory"
            | "XapiSetupPerTitleDriveLetters"
            | "XapiSelectCachePartition"
            | "XMountUtilityDrive"
            | "XMountAlternateTitleA"
            | "XUnmountAlternateTitleA"
            | "XMountMUA"
            | "XMountMURootA"
            | "XUnmountMU"
            | "MU_Init"
            | "XLaunchNewImageA"
            | "XGetLaunchInfo"
            | "XGetSectionHandleA"
            | "XLoadSectionByHandle"
            | "XFreeSectionByHandle"
            | "XAutoPowerDownResetTimer"
            | "OutputDebugStringA"
            | "GetTimeZoneInformation"
            | "XCalculateSignatureBegin"
            | "timeSetEvent"
            | "timeKillEvent"
    )
}

fn xref_target_matches_pattern(target: &str, pattern_name: &str) -> bool {
    let target = normalize_func_name(target.strip_prefix("XREF_").unwrap_or(target));
    let pattern = normalize_func_name(pattern_name);
    if target == pattern {
        return true;
    }
    if let Some(stripped) = target.strip_prefix("D3D_") {
        if stripped == pattern {
            return true;
        }
    }
    if let Some(stripped) = pattern.strip_prefix("D3D_") {
        if stripped == target {
            return true;
        }
    }
    false
}

fn resolve_xref_target(matches: &[OovpaMatch], target: &str) -> Option<u32> {
    matches
        .iter()
        .find(|m| xref_target_matches_pattern(target, m.pattern_name))
        .map(|m| m.guest_addr)
}

fn candidate_xrefs_match(
    guest_base: *const u8,
    scan_end: u32,
    candidate_addr: u32,
    xrefs: &[super::OovpaXref],
    matches: &[OovpaMatch],
) -> (bool, u32, u32) {
    let mut checked = 0u32;
    let mut matched = 0u32;
    for xref in xrefs {
        let Some(target_addr) = resolve_xref_target(matches, xref.target) else {
            continue;
        };
        checked += 1;
        let operand_addr = candidate_addr.saturating_add(xref.offset as u32);
        let Some(operand) = read_u32_le(guest_base, operand_addr, scan_end) else {
            return (false, matched, checked);
        };
        let rel_target = operand.wrapping_add(operand_addr.wrapping_add(4));
        if operand != target_addr && rel_target != target_addr {
            return (false, matched, checked);
        }
        matched += 1;
    }
    (true, matched, checked)
}

fn scan_table(
    guest_base: *const u8,
    ranges: &[(u32, u32)],
    scan_end: u32,
    patterns: &[OovpaPattern],
    is_cxbx: bool,
    xdk_build: u16,
    profile: OovpaScanProfile,
    matches: &mut Vec<OovpaMatch>,
) {
    // Best-version filter: for each function name, only scan the pattern with the
    // highest min_version that's still ≤ xdk_build. This prevents old-version patterns
    // (e.g. SetVertexShader from XDK 1024) from false-matching against newer XBEs
    // (e.g. Spider-Man XDK 4134). Mirrors Cxbx-R's version selection logic.
    let mut best_version: std::collections::HashMap<&str, u16> = std::collections::HashMap::new();
    if xdk_build > 0 {
        for pat in patterns {
            if !pattern_enabled_for_profile(pat.name, profile) {
                continue;
            }
            if pat.min_version > 0 && pat.min_version > xdk_build {
                continue; // too new for this XBE
            }
            let entry = best_version
                .entry(normalize_func_name(pat.name))
                .or_insert(0);
            if pat.min_version > *entry {
                *entry = pat.min_version;
            }
        }
    }

    let mut skipped_old = 0u32;
    let mut skipped_new = 0u32;
    let mut skipped_small = 0u32;
    let mut skipped_profile = 0u32;
    let total = patterns.len();

    let mut ordered: Vec<&OovpaPattern> = Vec::with_capacity(patterns.len());
    ordered.extend(
        patterns
            .iter()
            .filter(|pat| database::xrefs_for_pattern(pat, xdk_build as u32).is_empty()),
    );
    ordered.extend(
        patterns
            .iter()
            .filter(|pat| !database::xrefs_for_pattern(pat, xdk_build as u32).is_empty()),
    );

    for pat in ordered {
        if !pattern_enabled_for_profile(pat.name, profile) {
            skipped_profile += 1;
            continue;
        }
        // Version filter: skip patterns that require a newer XDK than this XBE
        if pat.min_version > 0 && xdk_build > 0 && xdk_build < pat.min_version {
            skipped_new += 1;
            continue;
        }
        // Best-version ceiling: if a newer pattern exists for this function that still
        // fits the XBE's XDK version, skip older patterns. Only the closest-version
        // pattern should match — older ones have stale bytes that cause false positives.
        if xdk_build > 0 {
            if let Some(&best) = best_version.get(normalize_func_name(pat.name)) {
                if pat.min_version < best {
                    skipped_old += 1;
                    continue;
                }
            }
        }
        let xrefs = database::xrefs_for_pattern(pat, xdk_build as u32);
        let has_resolved_xref = xrefs
            .iter()
            .any(|xref| resolve_xref_target(matches, xref.target).is_some());

        // Skip byte-only patterns with too few entries. Cxbx-R permits tiny
        // patterns when an XREF_ENTRY resolves to another known SDK symbol; we
        // mirror that by scanning them only after their function xref is known.
        if pat.entries.len() < 7 && !has_resolved_xref {
            skipped_small += 1;
            continue;
        }
        let mut best_matched = 0u32;
        let mut best_addr = 0u32;
        let mut found = false;

        for &(range_start, range_end) in ranges {
            if range_start >= range_end {
                continue;
            }
            let end = range_end.saturating_sub(pat.detect_size as u32);
            let mut addr = range_start;
            while addr < end {
                let mut matched = 0u32;
                for entry in pat.entries {
                    let byte = unsafe { *guest_base.add((addr + entry.offset as u32) as usize) };
                    if byte == entry.value {
                        matched += 1;
                    }
                }

                // Exact match only — 1-byte tolerance removed (caused 27% false-positive rate).
                // Patterns must match ALL entries. If a pattern doesn't match, add more
                // version-specific entries or a separate pattern for that XDK version.
                let total = pat.entries.len() as u32;
                let is_match = matched == total;
                if is_match {
                    let (xrefs_ok, xrefs_matched, xrefs_checked) =
                        candidate_xrefs_match(guest_base, scan_end, addr, xrefs, matches);
                    if !xrefs_ok {
                        if oovpa_diff_enabled() {
                            debug_log(&format!(
                                "[OOVPA-DIFF] [XREF_REJECTED] {} at 0x{:08X} matched_bytes={}/{} xrefs={}/{}",
                                pat.name,
                                addr,
                                matched,
                                total,
                                xrefs_matched,
                                xrefs_checked
                            ));
                        }
                        addr += 1;
                        continue;
                    }

                    // Reject if within 16 bytes of an existing match
                    if matches
                        .iter()
                        .any(|m| (addr as i64 - m.guest_addr as i64).unsigned_abs() < 16)
                    {
                        addr += 1;
                        continue;
                    }
                    // Check dedup in merged mode
                    if is_cxbx && matches.iter().any(|m| m.guest_addr == addr) {
                        addr += 1;
                        continue;
                    }

                    // Prologue validation: reject matches that land inside another
                    // function's body. LTCG patterns are especially prone to this.
                    let b0 = unsafe { *guest_base.add(addr as usize) };
                    let b1 = unsafe { *guest_base.add((addr + 1) as usize) };
                    // Preceding byte check (2026-04-21): functions in MSVC-linked
                    // binaries are typically padded to 16-byte boundaries with
                    // INT3 (0xCC), preceded by the previous function's RET
                    // (0xC3 or 0xC2 imm16), or nop-aligned. If our candidate
                    // address isn't preceded by one of these, it's very likely
                    // a mid-function match. This catches false positives where
                    // a valid-looking prologue opcode occurs inside another
                    // function's body.
                    let prev = if addr > 0 {
                        unsafe { *guest_base.add((addr - 1) as usize) }
                    } else {
                        0
                    };
                    let prev_is_function_boundary = prev == 0xCC   // INT3 padding
                    || prev == 0xC3                             // ret
                    || prev == 0x90                             // nop
                    || prev == 0xE9 || prev == 0xEB             // jmp (end of prev fn)
                    // For ret N (C2 imm16), the last byte is the hi imm byte.
                    // Hard to detect without deeper analysis; accept for now.
                    || addr == 0; // Very start of section.

                    let has_prologue = b0 == 0x55  // push ebp
                    || (b0 >= 0x50 && b0 <= 0x57)  // push reg
                    || (b0 == 0x83 && b1 == 0xEC)  // sub esp, imm8
                    || (b0 == 0x81 && b1 == 0xEC)  // sub esp, imm32
                    || (b0 == 0x8B && b1 == 0xFF)  // mov edi,edi (hotpatch)
                    || b0 == 0xA1   // mov eax, [imm32] — e.g. CreateDevice
                    || (b0 == 0x8B && b1 == 0x44)  // mov eax, [esp+...]
                    || (b0 == 0x8B && b1 == 0x4C)  // mov ecx, [esp+...]
                    || (b0 == 0x8B && b1 == 0x54)  // mov edx, [esp+...] — stdcall thunk
                    || (b0 == 0xFF && b1 == 0x74)  // push [esp+N] — stdcall thunk
                    || b0 == 0x68   // push imm32 — SEH prologue
                    || b0 == 0xE8   // call rel32 — forwarder
                    || b0 == 0xB8   // mov eax, imm32 (common entry)
                    || b0 == 0xE9   // jmp rel32 (thunk)
                    || b0 == 0xEB   // jmp rel8 (thunk)
                    || (b0 == 0x6A)  // push imm8
                    || (b0 == 0x33 && b1 == 0xC0)  // xor eax, eax
                    || (b0 == 0x64)  // fs: prefix (TLS access)
                    || b0 == 0xCC; // INT3 (already hooked)

                    // `8B 0D <disp32>` (mov ecx, [global]) removed from the
                    // base whitelist (2026-04-21). This opcode is frequent
                    // inside function bodies, and the bare pattern matched
                    // mid-function in test_blue_ltcg at guest 0x00019CE0 —
                    // causing CreateDevice to fire 7× on an internal loop
                    // iteration instead of once on function entry. Accept
                    // it ONLY when the preceding byte is a function boundary.
                    let has_prologue =
                        has_prologue || (b0 == 0x8B && b1 == 0x0D && prev_is_function_boundary);

                    if !has_prologue {
                        debug_log(&format!(
                            "[OOVPA] REJECT_PROLOGUE: {} at 0x{:08X} — first bytes {:02X} {:02X} prev={:02X} not a function entry",
                            pat.name, addr, b0, b1, prev
                        ));
                        if oovpa_diff_enabled() {
                            debug_log(&format!(
                                "[OOVPA-DIFF] [PROLOGUE_REJECTED] {} at 0x{:08X} bytes={:02X} {:02X} prev={:02X}",
                                pat.name, addr, b0, b1, prev
                            ));
                        }
                        addr += 1;
                        continue;
                    }

                    // Determine argc: known override > pattern > auto-detect
                    let argc = known_argc(pat.name).unwrap_or_else(|| {
                        if pat.argc > 0 {
                            pat.argc
                        } else {
                            auto_detect_argc(guest_base, addr, scan_end)
                        }
                    });

                    // Log first 16 bytes
                    let mut sig = String::with_capacity(48);
                    for b in 0..16u32 {
                        if addr + b < scan_end {
                            let byte = unsafe { *guest_base.add((addr + b) as usize) };
                            sig.push_str(&format!("{:02X} ", byte));
                        }
                    }

                    let source = database::source_for_pattern(pat, xdk_build as u32);
                    let xref_suffix = if xrefs_checked > 0 {
                        format!(
                            " xrefs={}/{} source={}",
                            xrefs_matched,
                            xrefs_checked,
                            source.unwrap_or("unknown")
                        )
                    } else if let Some(source) = source {
                        format!(" source={}", source)
                    } else {
                        String::new()
                    };

                    debug_log(&format!(
                        "[OOVPA] MATCH: {} at 0x{:08X} argc={}{} sig={}",
                        pat.name,
                        addr,
                        argc,
                        xref_suffix,
                        sig.trim()
                    ));

                    matches.push(OovpaMatch {
                        guest_addr: addr,
                        host_offset: 0, // set during hook planting
                        original_byte: 0,
                        pattern_name: pat.name,
                        argc,
                        hle_mode: pat.hle_mode,
                        is_cxbx,
                        is_manual: false,
                        active: false,
                        pending_rearm: false,
                        call_count: 0,
                    });
                    found = true;
                    // For CxbxDB patterns: continue scanning for LTCG copies.
                    // Hand-tuned: stop at first match.
                    if !is_cxbx {
                        break;
                    }
                }

                if matched > best_matched {
                    best_matched = matched;
                    best_addr = addr;
                }
                addr += 1;
            }
            if found && !is_cxbx {
                break;
            }
        }

        if !found {
            let pct = if pat.entries.len() > 0 {
                (best_matched * 100) / pat.entries.len() as u32
            } else {
                0
            };
            if pct >= 70 {
                let mut miss_info = String::new();
                if best_addr > 0 {
                    for entry in pat.entries {
                        let byte =
                            unsafe { *guest_base.add((best_addr + entry.offset as u32) as usize) };
                        if byte != entry.value {
                            miss_info.push_str(&format!(
                                " [0x{:02X}: exp=0x{:02X} got=0x{:02X}]",
                                entry.offset, entry.value, byte
                            ));
                        }
                    }
                }
                debug_log(&format!(
                    "[OOVPA] NEAR_MISS: {} at 0x{:08X} ({}% = {}/{}){}",
                    pat.name,
                    best_addr,
                    pct,
                    best_matched,
                    pat.entries.len(),
                    miss_info
                ));
            } else {
                debug_log(&format!(
                    "[OOVPA] NO_MATCH: {} (best {}/{} = {}% at 0x{:08X})",
                    pat.name,
                    best_matched,
                    pat.entries.len(),
                    pct,
                    best_addr
                ));
            }
        }
    }

    let scanned = total as u32 - skipped_old - skipped_new - skipped_small - skipped_profile;
    debug_log(&format!(
        "[OOVPA] scan_table: {} total, {} scanned, {} skipped ({}=profile, {}=too_new, {}=superseded, {}=too_small)",
        total,
        scanned,
        skipped_profile + skipped_old + skipped_new + skipped_small,
        skipped_profile,
        skipped_new,
        skipped_old,
        skipped_small
    ));
}

// ============================================================================
// Validation: SymbolCache comparison (Option A) + ret N prologue check (Option B)
// ============================================================================

/// Load Cxbx-R SymbolCache from file and compare against our matches + manual hooks.
/// Logs every match, mismatch, and missing symbol.
pub fn validate_against_symbol_cache(
    matches: &[super::OovpaMatch],
    guest_base: *const u8,
    scan_end: u32,
    entry_point: u32,
    title_id: Option<u32>,
    xdk_build: u16,
    d3d8_is_ltcg: bool,
) {
    debug_log(&format!(
        "[VALIDATE] Starting SymbolCache validation ({} matches to check)...",
        matches.len()
    ));

    let Some(symbols_path) = find_symbol_cache_path(entry_point, title_id, xdk_build, d3d8_is_ltcg)
    else {
        debug_log("[VALIDATE] No SymbolCache selected for this title — skipping comparison");
        return;
    };
    let content = match std::fs::read_to_string(&symbols_path) {
        Ok(c) => c,
        Err(e) => {
            debug_log(&format!(
                "[VALIDATE] Cannot read {}: {} — skipping SymbolCache comparison",
                symbols_path, e
            ));
            return;
        }
    };

    let cxbx_symbols = parse_symbol_cache_entries(&content);

    debug_log(&format!(
        "[VALIDATE] Loaded {} Cxbx-R symbols from SymbolCache {}",
        cxbx_symbols.len(),
        symbols_path
    ));

    // Build lookup from our matches (guest_addr → name)
    let mut our_addrs: std::collections::HashMap<u32, &str> = std::collections::HashMap::new();
    for m in matches {
        our_addrs.insert(m.guest_addr, m.pattern_name);
    }

    // === Option A: Compare every Cxbx-R symbol against our matches ===
    let mut matched = 0u32;
    let mut misnamed = 0u32;
    let mut missing = 0u32;
    let mut missing_d3d = Vec::new();
    let mut missing_dsound = Vec::new();
    let mut missing_xapi = Vec::new();
    let mut missing_xinput = Vec::new();

    for (cxbx_addr, cxbx_name) in &cxbx_symbols {
        // Skip data offsets (< 0x1000 or data section addresses)
        if *cxbx_addr < 0x1000
            || cxbx_name.contains("_OFFSET")
            || cxbx_name.starts_with("D3DRS_")
            || cxbx_name.starts_with("D3D_g_")
            || cxbx_name.starts_with("g_")
            || cxbx_name.starts_with("_tls")
        {
            continue;
        }

        match our_addrs.get(cxbx_addr) {
            Some(our_name) => {
                if symbol_names_equivalent(our_name, cxbx_name.as_str()) {
                    matched += 1;
                } else {
                    // Same address, different normalized name.
                    misnamed += 1;
                    debug_log(&format!(
                        "[VALIDATE] NAME_MISMATCH: 0x{:08X} — us='{}' cxbxr='{}'",
                        cxbx_addr, our_name, cxbx_name
                    ));
                }
            }
            None => {
                missing += 1;
                // Categorize by address range / name prefix
                if cxbx_name.starts_with("D3D")
                    || cxbx_name.starts_with("Direct3D")
                    || cxbx_name.starts_with("CDevice")
                    || cxbx_name.starts_with("Lock")
                    || cxbx_name.starts_with("Get2D")
                    || cxbx_name.starts_with("CMiniport")
                {
                    missing_d3d.push((*cxbx_addr, cxbx_name.as_str()));
                } else if cxbx_name.contains("Sound")
                    || cxbx_name.contains("Mcpx")
                    || cxbx_name.contains("HRTF")
                    || cxbx_name.starts_with("XAudio")
                    || cxbx_name.starts_with("DSound")
                    || cxbx_name.starts_with("IsValid")
                {
                    missing_dsound.push((*cxbx_addr, cxbx_name.as_str()));
                } else if cxbx_name.starts_with("XInput")
                    || cxbx_name.starts_with("XGet")
                    || cxbx_name.starts_with("XInit")
                    || cxbx_name.contains("MU_")
                    || cxbx_name.contains("XID_")
                    || cxbx_name.starts_with("XMount")
                    || cxbx_name.starts_with("XUnmount")
                {
                    missing_xinput.push((*cxbx_addr, cxbx_name.as_str()));
                } else {
                    missing_xapi.push((*cxbx_addr, cxbx_name.as_str()));
                }
            }
        }
    }

    // === Log summary ===
    debug_log(&format!(
        "[VALIDATE] SymbolCache: {} matched, {} misnamed, {} missing (of {} function symbols)",
        matched,
        misnamed,
        missing,
        matched + misnamed + missing
    ));

    // Log missing D3D functions (most critical for rendering)
    if !missing_d3d.is_empty() {
        debug_log(&format!("[VALIDATE] MISSING D3D ({}):", missing_d3d.len()));
        for (addr, name) in &missing_d3d {
            debug_log(&format!("[VALIDATE]   0x{:08X} -> {}", addr, name));
        }
    }

    // Log missing DSound functions
    if !missing_dsound.is_empty() {
        debug_log(&format!(
            "[VALIDATE] MISSING DSound ({}):",
            missing_dsound.len()
        ));
        // Just count — too many to list individually
        for (addr, name) in missing_dsound.iter().take(10) {
            debug_log(&format!("[VALIDATE]   0x{:08X} -> {}", addr, name));
        }
        if missing_dsound.len() > 10 {
            debug_log(&format!(
                "[VALIDATE]   ... and {} more DSound functions",
                missing_dsound.len() - 10
            ));
        }
    }

    // Log missing XInput/USB
    if !missing_xinput.is_empty() {
        debug_log(&format!(
            "[VALIDATE] MISSING XInput/USB ({}):",
            missing_xinput.len()
        ));
        for (addr, name) in &missing_xinput {
            debug_log(&format!("[VALIDATE]   0x{:08X} -> {}", addr, name));
        }
    }

    // Log missing XAPI
    if !missing_xapi.is_empty() {
        debug_log(&format!(
            "[VALIDATE] MISSING XAPI ({}):",
            missing_xapi.len()
        ));
        for (addr, name) in &missing_xapi {
            debug_log(&format!("[VALIDATE]   0x{:08X} -> {}", addr, name));
        }
    }

    // === Check our matches that are NOT in Cxbx-R's SymbolCache ===
    let cxbx_addrs: std::collections::HashSet<u32> = cxbx_symbols.iter().map(|(a, _)| *a).collect();
    let mut extra = 0u32;
    for m in matches {
        if !cxbx_addrs.contains(&m.guest_addr) && m.guest_addr >= 0x1000 {
            extra += 1;
            debug_log(&format!(
                "[VALIDATE] EXTRA (not in CxbxR): 0x{:08X} -> {} ({})",
                m.guest_addr,
                m.pattern_name,
                if m.is_manual {
                    "manual"
                } else if m.is_cxbx {
                    "cxbxdb"
                } else {
                    "hand-tuned"
                }
            ));
        }
    }
    if extra > 0 {
        debug_log(&format!(
            "[VALIDATE] {} hooks in our matches but NOT in Cxbx-R SymbolCache",
            extra
        ));
    }

    // === Option B: ret N / prologue validation for every match ===
    debug_log("[VALIDATE] === Prologue + ret N validation ===");
    let mut valid = 0u32;
    let mut invalid = 0u32;
    for m in matches {
        if m.guest_addr < 0x1000 || m.guest_addr >= scan_end {
            continue;
        }
        let addr = m.guest_addr;
        let b0 = unsafe { *guest_base.add(addr as usize) };
        let b1 = unsafe { *guest_base.add((addr + 1) as usize) };
        let b2 = if addr + 2 < scan_end {
            unsafe { *guest_base.add((addr + 2) as usize) }
        } else {
            0
        };

        // Check for valid function prologue patterns.
        // MUST stay in lock-step with the scanner's retain() filter around
        // line 382-396. When these drift, the validator produces BAD_PROLOGUE
        // entries for matches that the scanner already accepted — noise, not
        // signal. All byte classes below are the ones scan() considers valid
        // function entries, in the same order.
        let has_prologue = b0 == 0x55                      // push ebp
            || (b0 >= 0x50 && b0 <= 0x57)                 // push reg
            || (b0 == 0x83 && b1 == 0xEC)                 // sub esp, imm8
            || (b0 == 0x81 && b1 == 0xEC)                 // sub esp, imm32
            || (b0 == 0x8B && b1 == 0xFF)                 // mov edi, edi (hotpatch)
            || b0 == 0xA1                                 // mov eax, [imm32]
            || (b0 == 0x8B && b1 == 0x44)                 // mov eax, [esp+disp8]
            || (b0 == 0x8B && b1 == 0x4C)                 // mov ecx, [esp+disp8] — stdcall thunk
            || (b0 == 0x8B && b1 == 0x54)                 // mov edx, [esp+disp8] — stdcall thunk
            || (b0 == 0x8B && b1 == 0x0D)                 // mov ecx, [imm32] — LTCG entry
            || (b0 == 0xFF && b1 == 0x74)                 // push [esp+N] — stdcall thunk
            || b0 == 0x68                                 // push imm32 — SEH prologue / thunk
            || b0 == 0xE8                                 // call rel32 — forwarder thunk
            || b0 == 0x6A                                 // push imm8
            || (b0 == 0x33 && b1 == 0xC0)                 // xor eax, eax
            || b0 == 0x64                                 // fs: prefix
            || b0 == 0xCC                                 // INT3 (already hooked)
            || b0 == 0xB8                                 // mov eax, imm32 (common entry)
            || b0 == 0xE9                                 // jmp rel32 (thunk)
            || b0 == 0xEB; // jmp rel8 (thunk)

        // Find ret N — auto_detect_argc scans first 512 bytes
        let detected_argc = auto_detect_argc(guest_base, addr, scan_end);

        if !has_prologue && b0 != 0xCC {
            invalid += 1;
            debug_log(&format!(
                "[VALIDATE] BAD_PROLOGUE: {} at 0x{:08X} — first bytes: {:02X} {:02X} {:02X} (expected function entry)",
                m.pattern_name, addr, b0, b1, b2
            ));
        } else {
            valid += 1;
        }

        // Check argc mismatch
        if m.argc > 0 && detected_argc > 0 && m.argc != detected_argc {
            debug_log(&format!(
                "[VALIDATE] ARGC_MISMATCH: {} at 0x{:08X} — declared={} detected={} (ret {})",
                m.pattern_name,
                addr,
                m.argc,
                detected_argc,
                detected_argc * 4
            ));
        }
    }

    debug_log(&format!(
        "[VALIDATE] Prologue check: {} valid, {} invalid (of {} hooks)",
        valid,
        invalid,
        valid + invalid
    ));
}

/// Known argc overrides for D3D/kernel functions whose public API argc is fixed.
/// LTCG can restructure functions so auto_detect_argc finds an internal sub-return
/// (e.g. `ret 4` at +0xBB) instead of the true stdcall cleanup.
/// These values are from the XDK API documentation.
pub(super) fn known_argc(name: &str) -> Option<u8> {
    // LTCG SetTexture specializations — BOTH variants pass ONE arg via the
    // stack (`ret 4` in Cxbx-R), the other via EAX. Apply the override before
    // normalization so the LTCG suffix isn't stripped. For the canonical
    // non-LTCG `D3DDevice_SetTexture` argc remains 2 (stdcall Stage, pTexture).
    match name {
        "D3DDevice_SetTexture_4__LTCG_eax1" => return Some(1), // Stage in EAX, pTexture on stack
        "D3DDevice_SetTexture_4__LTCG_eax2" => return Some(1), // pTexture in EAX, Stage on stack
        _ => {}
    }

    // Normalize: strip __LTCG suffix, trailing _N size
    let base = if let Some(i) = name.find("__LTCG") {
        &name[..i]
    } else {
        name
    };
    let base = if base.len() > 3 {
        if let Some(u) = base.rfind('_') {
            let suffix = &base[u + 1..];
            if suffix.len() <= 2 && suffix.chars().all(|c| c.is_ascii_digit()) {
                &base[..u]
            } else {
                base
            }
        } else {
            base
        }
    } else {
        base
    };

    match base {
        // Direct3D device creation / lifecycle
        "Direct3D_CreateDevice" => Some(6), // Adapter, DeviceType, hFocusWindow, BehaviorFlags, pPresentationParameters, ppReturnedDeviceInterface
        // Swap / Present
        "D3DDevice_Swap" => Some(1),    // Flags
        "D3DDevice_Present" => Some(4), // pSourceRect, pDestRect, hDestWindowOverride, pDirtyRegion
        // Clear
        "D3DDevice_Clear" => Some(6), // Count, pRects, Flags, Color, Z, Stencil
        // Draw calls
        "D3DDevice_DrawVertices" => Some(3), // PrimitiveType, StartVertex, VertexCount
        "D3DDevice_DrawIndexedVertices" => Some(3),
        "D3DDevice_DrawVerticesUP" => Some(4),
        // Public Xbox D3D8 export is _D3DDevice_DrawIndexedVerticesUP@20:
        // (PrimitiveType, IndexCount, pIndexData, pVertexStreamZeroData,
        // VertexStreamZeroStride). The older argc=8 over-popped 12 bytes per
        // call and corrupted the harvester's stack locals before SetStreamSource.
        "D3DDevice_DrawIndexedVerticesUP" => Some(5),
        // State setters
        "D3DDevice_SetRenderState_Simple" => Some(0), // __fastcall: args in ECX/EDX, not on stack
        "D3DDevice_SetTransform" => Some(2),
        "D3DDevice_SetViewport" => Some(1),
        "D3DDevice_SetRenderTarget" => Some(2),
        "D3DDevice_SetTexture" => Some(2),
        "D3DDevice_SetTextureStageState" => Some(3),
        "D3DDevice_SetTextureStageStateNotInline" => Some(3),
        "D3DDevice_SetTextureStageStateNotInline2" => Some(3),
        "D3D_CDevice_SetTextureStageStateNotInline" => Some(3),
        // SetTextureState_* variants: stdcall (Stage, Value) — ret 8 = 2 args.
        // Validator flagged auto_detect_argc returning 3 (noise — likely scanned past
        // the true epilogue into an internal helper's ret 0xC). XDK API signature is
        // D3DDevice_SetTextureState_<Field>(DWORD Stage, DWORD Value).
        "D3DDevice_SetTextureState_TexCoordIndex" => Some(2), // ret 8: (Stage, Value)
        "D3DDevice_SetTextureState_BorderColor" => Some(2),   // ret 8: (Stage, Value)
        "D3DDevice_SetTextureState_ColorKeyColor" => Some(2), // ret 8: (Stage, Value)
        "D3DDevice_SetTextureState_BumpEnv" => Some(3),       // ret 12: (Stage, State, Value)
        "D3DDevice_SetVertexShader" => Some(1),
        "D3DDevice_SetPixelShader" => Some(1),
        "D3DDevice_SetShaderConstantMode" => Some(1),
        "D3DDevice_GetShaderConstantMode" => Some(1),
        "D3DDevice_SetVertexShaderConstant" => Some(3),
        "D3DDevice_SetPixelShaderConstant" => Some(3),
        // D3DDevice_CreateVertexShader(pDeclaration, pFunction, pHandle, Usage) — ret 0x10
        "D3DDevice_CreateVertexShader" => Some(4),
        // D3DDevice_CreatePixelShader(pFunction, pHandle) — ret 0x8
        "D3DDevice_CreatePixelShader" => Some(2),
        "D3DDevice_SetStreamSource" => Some(3),
        "D3DDevice_SetIndices" => Some(2),
        "D3DDevice_SetPalette" => Some(2),
        "D3DDevice_CreatePalette" => Some(2),
        "D3DDevice_CreatePalette2" => Some(1),
        // Texture/Resource creation
        "D3DDevice_CreateTexture" => Some(7),
        "D3DDevice_CreateTexture2" => Some(7),
        "XGRPH_CreateTexture" => Some(4), // (device, width, format, ppTexture) ret 0x10
        "D3DDevice_CreateImageSurface" => Some(4), // stdcall (Width, Height, Format, ppSurf) ret 16
        // Previous override to Some(1) was WRONG: it was based on auto-detected
        // "ret 4" at 0x002EE000, but that address is a JMP thunk (E9 rel32) into
        // the real body at 0x002F39C0 (= D3D_CreateStandAloneSurface). The
        // detected "ret 4" was noise from scanning past the JMP. With argc=1,
        // our handler at oovpa_hle.rs:1261 read args[3] as ppSurf but args[3]
        // was never populated (stays 0 from the [0u32;8] init), producing a
        // phantom NULL. 10-agent audit + runtime validator both confirmed.
        "D3D_CreateStandAloneSurface" => Some(4), // same args as CreateImageSurface
        "D3DDevice_CreateVertexBuffer" => Some(5),
        "D3DDevice_CreateVertexBuffer2" => Some(1), // Length only
        "D3DDevice_CreateIndexBuffer2" => Some(2),
        // Surface functions
        "D3DTexture_GetSurfaceLevel" => Some(3), // pTexture, Level, ppSurface
        "D3DTexture_LockRect" => Some(5),        // pTexture, Level, pLockedRect, pRect, Flags
        "D3DSurface_LockRect" => Some(4), // guest ret 0x10: (pSurface, pLockedRect, pRect, Flags)
        "D3DCubeTexture_LockRect" => Some(3), // guest ret 0xC: LTCG variant
        "Get2DSurfaceDesc" => Some(3),    // pPixelContainer, Level, pDesc
        "Lock2DSurface" => Some(6), // pPixelContainer, FaceType, Level, pLockedRect, pRect, Flags
        // Begin/End. `D3DDevice_BeginPush` is ABI-ambiguous across SDK/cache
        // names: some bodies are ret 4 and return the cursor in eax, while
        // Spider-Man XDK 4134's same base symbol is ret 8 and writes ppPush.
        // Let the epilogue scanner choose for the unsuffixed name.
        "D3DDevice_BeginPush" => None,
        "D3DDevice_BeginPush_8" => Some(2), // guest ret 8: (Count, ppPush)
        "D3DDevice_EndPush" => Some(1),
        "D3DDevice_BeginVisibilityTest" => Some(0),
        "D3DDevice_EndVisibilityTest" => Some(1),
        // Begin/End scene
        "D3DDevice_BeginScene" => Some(0),
        "D3DDevice_EndScene" => Some(0),
        // Misc
        "D3DDevice_SetScissors" => Some(3),
        "D3DDevice_SetBackBufferScale" => Some(2),
        "D3DDevice_GetBackBuffer2" => Some(1),
        "D3DDevice_SetFlickerFilter" => Some(1),
        "D3DDevice_SetSoftDisplayFilter" => Some(1),
        "D3DDevice_EnableOverlay" => Some(1),
        "D3DDevice_UpdateOverlay" => Some(7),
        "D3DDevice_BlockOnFence" => Some(1), // guest ret 4: (FenceToken)
        "D3D_SetFence" => Some(1),           // guest ret 4: (FenceToken/Time)
        "D3D_BlockOnTime" => Some(2),        // guest ret 8: (Time, MakeSpace)
        "D3D_BlockOnResource" => Some(1), // Spider-Man XDK 4134 body reads [esp+4] and returns ret 4
        "D3DDevice_BlockUntilIdle" => Some(0), // guest bare ret: no args
        "D3DDevice_BlockUntilVerticalBlank" => Some(0),
        "D3DDevice_SetVerticalBlankCallback" => Some(1),
        "D3DDevice_SetSwapCallback" => Some(1),
        "D3DDevice_SetGammaRamp" => Some(2),
        "D3DDevice_GetVisibilityTestResult" => Some(3),
        "D3DDevice_LoadVertexShader" => Some(2), // guest ret 8: (Handle, Address)
        "D3DDevice_BeginStateBig" => Some(1), // guest ret 4: reserves Count dwords and returns PB cursor
        "D3DDevice_SelectVertexShader" => Some(2),
        "D3DDevice_SetVertexShaderInput" => Some(3),
        "D3DDevice_RunVertexStateShader" => Some(2),
        "D3DDevice_GetProjectionViewportMatrix" => Some(1),
        "D3DDevice_SetModelView" => Some(3),
        "D3DDevice_SetScreenSpaceOffset" => Some(2),
        "D3DDevice_GetDisplayFieldStatus" => Some(1),
        // Xbox D3D8 Reset is exported as _D3DDevice_Reset@4 in the 4361
        // harvester map: one stack argument, pPresentationParameters. Treating
        // it as two args advances ESP by one extra dword and corrupts locals
        // after Reset, which made the harvester's pVB become a return address.
        "D3DDevice_Reset" => Some(1),
        "D3DDevice_IsBusy" => Some(0), // guest bare ret: no stack args
        // Resource / buffer functions
        "D3DResource_IsBusy" => Some(1),  // guest ret 4: (pResource)
        "D3DResource_Release" => Some(1), // ret 4: IDirect3DResource8* on stack
        "D3DResource_Register" => Some(2), // ret 8: (pResource, pBase); ECX-this variants are handled too
        "D3DVertexBuffer_Lock" => Some(5), // guest ret 0x14: (pVB, OffsetToLock, SizeToLock, ppbData, Flags)
        "D3DTexture_GetSurfaceLevel2" => Some(1),
        "D3DSurface_GetDesc" => Some(2), // guest ret 8: (pSurface, pDesc)
        "D3DDevice_GetBackBuffer" => Some(3), // BackBuffer, Type, ppBackBuffer
        "D3DDevice_CopyRects" => Some(5),
        // Palette
        "D3DPalette_Lock" => Some(3),
        "D3DPalette_Lock2" => Some(2), // guest ret 8: (pPalette, Flags)
        // XGRPH helpers. XGSwizzleRect's epilogue is far beyond the generic
        // 512-byte ret scan in XDK 4134, but the real function returns `ret 0x20`.
        "XGSwizzleRect" => Some(8),
        // DirectSound
        "DirectSoundCreate" => Some(3),
        "CDirectSound_CreateSoundBuffer" | "IDirectSound_CreateSoundBuffer" => Some(4),
        "CDirectSound_CreateSoundStream" | "IDirectSound_CreateSoundStream" => Some(4),
        "DirectSoundCreateBuffer" | "DirectSoundCreateStream" => Some(2),
        "CDirectSound_SetMixBinHeadroom" => Some(3),
        "CDirectSound_SetPosition" => Some(5),
        "CDirectSound_SetVelocity" => Some(5),
        "CDirectSound_SynchPlayback" => Some(0),
        "CDirectSoundBuffer_SetBufferData" | "IDirectSoundBuffer_SetBufferData" => Some(3),
        "CDirectSoundBuffer_SetPlayRegion" | "IDirectSoundBuffer_SetPlayRegion" => Some(3),
        "CDirectSoundBuffer_SetLoopRegion" | "IDirectSoundBuffer_SetLoopRegion" => Some(3),
        "CDirectSoundBuffer_SetCurrentPosition" | "IDirectSoundBuffer_SetCurrentPosition" => {
            Some(2)
        }
        "CDirectSoundBuffer_GetCurrentPosition" | "IDirectSoundBuffer_GetCurrentPosition" => {
            Some(3)
        }
        "CDirectSoundBuffer_GetStatus" | "IDirectSoundBuffer_GetStatus" => Some(2),
        "CDirectSoundBuffer_Lock" | "IDirectSoundBuffer_Lock" => Some(8),
        "CDirectSoundBuffer_Play" | "IDirectSoundBuffer_Play" => Some(4),
        "CDirectSoundBuffer_PlayEx" | "IDirectSoundBuffer_PlayEx" => Some(4),
        "CDirectSoundBuffer_Stop" | "IDirectSoundBuffer_Stop" => Some(1),
        "CDirectSoundBuffer_StopEx" | "IDirectSoundBuffer_StopEx" => Some(4),
        // XAPI input/device functions
        "XGetDevices" => Some(1),           // ret 4: (DeviceType)
        "XGetDeviceChanges" => Some(3),     // ret 0xC: (DeviceType, pdwInsertions, pdwRemovals)
        "XInputOpen" => Some(4), // ret 0x10: (DeviceType, dwPort, dwSlot, pPollingParams)
        "XInputGetState" => Some(2), // ret 8: (hDevice, pState)
        "XInputGetCapabilities" => Some(2), // ret 8: (hDevice, pCapabilities)
        "XInputClose" => Some(1), // ret 4: (hDevice)
        "XInputPoll" => Some(1), // ret 4: (hDevice)
        "XInputSetState" => Some(2), // ret 8: (hDevice, pFeedback)
        // XAPI mount/filesystem functions
        "XMountMUA" => Some(3), // ret 0xC: (dwPort, dwSlot, lpDrivePath) — stdcall
        "XMountMURootA" => Some(3), // ret 0xC: (dwPort, dwSlot, lpDrivePath) — stdcall
        "XUnmountMU" => Some(2), // ret 8: (dwPort, dwSlot) — stdcall
        "XMountUtilityDrive" => Some(1), // ret 4: (fFormatClean) — stdcall
        "XMountAlternateTitle" => Some(3), // ret 0xC: (lpTitleName, dwAltTitleId, lpDrivePath)
        "XapiMapLetterToDirectory" => Some(6), // ret 0x18: (drive, device, title_id, create_dir, title_name, update_ts)
        // XAPI misc
        "XPP_Init" => Some(0),           // ret 0: no args (internal init)
        "XapiBootToDash" => Some(0),     // ret 0: no args
        "OutputDebugStringA" => Some(1), // ret 4: (lpOutputString) — stdcall variant
        // XAPI save game functions
        "XCreateSaveGame" => Some(6), // ret 0x18: (lpRootPathName, lpSaveGameName, dwCreationDisposition, dwCreateFlags, lpPathBuffer, uSize)
        "XFindFirstSaveGame" => Some(2), // ret 8: (lpRootPathName, pFindGameData)
        "XFindNextSaveGame" => Some(2), // ret 8: (hFindGame, pFindGameData)
        "XFindClose" => Some(1),      // ret 4: (hFind)
        "XDeleteSaveGame" => Some(2), // ret 8: (lpRootPathName, lpSaveGameName)
        // XAPI device/filesystem misc
        "XInitDevices" => Some(2), // ret 8: (dwPreallocTypeCount, pPreallocTypes)
        "XGetDeviceEnumerationStatus" => Some(0), // ret 0: no args
        "XFormatUtilityDrive" => Some(0), // ret 0: no args
        "XGetLaunchInfo" => Some(2), // ret 8: (pdwLaunchDataType, pLaunchData)
        "XLaunchNewImageA" => Some(2), // ret 8: (lpTitlePath, pLaunchData)
        "XRegisterThreadNotifyRoutine" => Some(2), // ret 8: (pThreadNotification, fRegister)
        "XSetProcessQuantumLength" => Some(1), // ret 4: (dwMilliseconds)
        "XMUNameFromDriveLetter" => Some(2), // ret 8: (chDriveLetter, lpName)
        "XMUPortFromDriveLetterA" => Some(1), // ret 4: (chDriveLetter)
        "XMUSlotFromDriveLetterA" => Some(1), // ret 4: (chDriveLetter)
        "XMUWriteNameToDriveLetter" => Some(2), // ret 8: (chDriveLetter, lpName)
        "XReadMUMetaData" => Some(4), // ret 0x10: (chDriveLetter, lpGameName, pXGAME_FIND_DATA, dwFlags)
        // Win32 thread functions (XAPI wrappers)
        "CreateThread" => Some(6), // ret 0x18: (lpAttr, dwStackSize, lpStartAddress, lpParam, dwFlags, lpThreadId)
        "SetThreadPriority" => Some(2), // ret 8: (hThread, nPriority)
        "GetThreadPriority" => Some(1), // ret 4: (hThread)
        "GetExitCodeThread" => Some(2), // ret 8: (hThread, lpExitCode)
        "SetThreadPriorityBoost" => Some(2), // ret 8: (hThread, bDisablePriorityBoost)
        // Win32 sync functions (XAPI wrappers)
        "CreateMutexA" => Some(3), // ret 0xC: (lpMutexAttributes, bInitialOwner, lpName)
        "CreateEventA" => Some(4), // ret 0x10: (lpEventAttributes, bManualReset, bInitialState, lpName)
        "OpenEventA" => Some(3),   // ret 0xC: (dwDesiredAccess, bInheritHandle, lpName)
        "SignalObjectAndWait" => Some(4), // ret 0x10: (hObjectToSignal, hObjectToWaitOn, dwMilliseconds, bAlertable)
        "QueueUserAPC" => Some(3),        // ret 0xC: (pfnAPC, hThread, dwData)
        "RaiseException" => Some(4), // ret 0x10: (dwExceptionCode, dwExceptionFlags, nNumberOfArguments, lpArguments)
        // Win32 misc (XAPI wrappers)
        "SetLastError" => Some(1),              // ret 4: (dwErrCode)
        "GetLastError" => Some(0),              // ret 0: no args
        "QueryPerformanceCounter" => Some(1),   // ret 4: (lpPerformanceCount)
        "QueryPerformanceFrequency" => Some(1), // ret 4: (lpFrequency)
        "GetTimeZoneInformation" => Some(1),    // ret 4: (lpTimeZoneInformation)
        // Win32 I/O (XAPI wrappers)
        "ReadFileEx" => Some(5), // ret 0x14: (hFile, lpBuffer, nNumberOfBytesToRead, lpOverlapped, lpCompletionRoutine)
        "WriteFileEx" => Some(5), // ret 0x14: (hFile, lpBuffer, nNumberOfBytesToWrite, lpOverlapped, lpCompletionRoutine)
        "MoveFileA" => Some(2),   // ret 8: (lpExistingFileName, lpNewFileName)
        _ => None,
    }
}

/// Auto-detect stdcall argc from ret N epilogue (scan first 512 bytes).
pub(super) fn auto_detect_argc(guest_base: *const u8, addr: u32, scan_end: u32) -> u8 {
    for i in 0..512u32 {
        if addr + i + 2 >= scan_end {
            break;
        }
        let b0 = unsafe { *guest_base.add((addr + i) as usize) };
        let b1 = unsafe { *guest_base.add((addr + i + 1) as usize) };
        let b2 = unsafe { *guest_base.add((addr + i + 2) as usize) };
        if b0 == 0xC2 && b2 == 0x00 {
            return b1 / 4; // ret imm16 → argc = imm16/4
        }
        if b0 == 0xC3 {
            return 0; // ret → cdecl/thiscall with 0 stack args
        }
    }
    0
}
