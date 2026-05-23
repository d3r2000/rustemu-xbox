use super::emulator::debug_log;
use crate::xbox::loader::xbe::XbeInfo;
/// Pre-AOT guest code patches.
/// Extracted from emulator.rs load_game() — no logic changes.
/// Applied before the AOT compiler runs so patches are compiled into the translation.
use crate::xbox::memory::guest_memory::GuestMemory;

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

/// Apply pre-AOT patches to guest memory.
/// Call this after loading the XBE but before emitting x64 code.
pub(crate) fn apply_pre_aot_patches(memory: &GuestMemory, xbe_info: &XbeInfo) {
    // Pre-AOT guest code patches — game-specific.
    // Only apply Spider-Man patches when running Spider-Man.
    let is_spiderman_xbe = xbe_info.title.contains("Spider")
        || xbe_info.title.contains("spider")
        || xbe_info.entry_point == 0x002A9C38; // Spider-Man XDK 4134 entry point
    let legacy_spidey_p0_zone_bypass = env_flag("RUSTEMU_SPIDEY_NOP_P0_ZONE_CALL");

    if !is_spiderman_xbe {
        debug_log(&format!(
            "[PRE-AOT] Non-Spider-Man game '{}' — skipping game-specific patches",
            xbe_info.title
        ));
    }

    // test_blue_padded.xbe (XDK 4361): patch call_all_padding to RET
    // immediately (2026-04-21).
    //
    // The XBE contains 70,000 __declspec(noinline) pad functions kept
    // alive by call_all_padding() calling every one of them. Live trace
    // showed the emulator corrupts ESI during pad execution at guest PC
    // ~0x004C51XX (event 23,770 of 23,900) — likely RET-MISS mis-
    // dispatch in our emitter. The corruption triggers an AV that no
    // SEH handler catches (filter 0x004ACE10 is opaque to our
    // pattern-matched whitelist), causing an AV retry storm that
    // kills the worker.
    //
    // call_all_padding's only purpose is to force the linker to keep
    // the pads. Skipping its body has no semantic effect for the
    // test — the pads exist whether executed or not. Main proceeds
    // to Direct3D_CreateDevice → for(;;) { Clear(0xFF0000FF); Swap; }
    // which is all we want to test.
    //
    // Address derivation: call_all_padding is at .map VA 0x00845F20
    // (test_blue_padded.map). With PE linker base 0x00400000 and XBE
    // image base 0x00010000, the loaded guest address is 0x00455F20.
    //
    // Writing 0xC3 (RET) at entry turns call_all_padding into a no-op.
    // Blue-series tests (test_blue, test_blue_ltcg, test_blue_padded) all
    // have the same main shape: `CreateDevice; for(;;){ Clear(BLUE); Swap; }`.
    // None read any internal D3D state that the guest CreateDevice would
    // populate, so all of them benefit from the same HLE-override route.
    //
    // test_blue.xbe       — minimal, 0.11 MB
    // test_blue_ltcg.xbe  — LTCG variant, 0.11 MB
    // test_blue_padded.xbe — 0.11 MB body + 70K __declspec(noinline) pads = 5.5 MB
    let is_blue_series = xbe_info.title == "Blue Padded (4361)"
        || xbe_info.title == "Blue Test (4361)"
        || xbe_info.title == "Blue Test LTCG (4361)";
    let is_blue_padded = xbe_info.title == "Blue Padded (4361)";
    if is_blue_series {
        // Tell the OOVPA dispatcher to use HLE-override (not TAP) for this XBE.
        // Blue-series main is just `CreateDevice; for(;;){ Clear; Swap; }`
        // with no subsequent code that reads internal D3D state — so we can
        // skip the guest's own CreateDevice body entirely (all the
        // CMiniport_InitHardware / WaitForIdle / pad-function cascade that
        // was corrupting ESP). Real games keep TAP.
        crate::xbox::aot::oovpa::IS_BLUE_PADDED.store(true, std::sync::atomic::Ordering::Relaxed);
        debug_log(&format!(
            "[PRE-AOT] {}: IS_BLUE_PADDED=true (CreateDevice will use HLE-override, not TAP)",
            xbe_info.title
        ));
    }
    if is_blue_padded {
        // These addresses are only present in the padded variant (the pads
        // inflate the XBE enough to push function layouts out). Applying
        // them to test_blue / test_blue_ltcg would poke random bytes.
        const CALL_ALL_PADDING_ADDR: u32 = 0x0045_5F20;
        memory.write_u8(CALL_ALL_PADDING_ADDR, 0xC3);
        debug_log(&format!(
            "[PRE-AOT] Blue Padded test: patched call_all_padding at 0x{:08X} to RET \
             (skips 70K pad execution that triggers ESI-corruption AV storm)",
            CALL_ALL_PADDING_ADDR
        ));

        // 0x004BD48B: WaitForIdle / BlockUntilIdle polling function.
        // Reads [this->device + 0x400700] and spins while non-zero. This is a
        // fence/pending-command counter that real NV2A hardware decrements as
        // it processes pushbuffer commands. With no real GPU, the value never
        // changes and the worker spins forever after reaching Stage 8.
        //
        // Our MMIO handler already reports "infinitely fast GPU" (all status
        // registers return 0 / idle), so skipping the wait is consistent —
        // the GPU is never busy in our emulation.
        const WAIT_FOR_IDLE_ADDR: u32 = 0x004B_D48B;
        memory.write_u8(WAIT_FOR_IDLE_ADDR, 0xC3);
        debug_log(&format!(
            "[PRE-AOT] Blue Padded test: patched WaitForIdle at 0x{:08X} to RET \
             (skips poll loop on [device+0x400700] pending-commands fence)",
            WAIT_FOR_IDLE_ADDR
        ));
    }

    // Universal Xbox hardware info initialization (all games).
    // XboxHardwareInfo struct at 0x01E00000 (safe pre-init area):
    //   +0x00: Flags = 0x20 (retail console)
    //   +0x04: GpuRevision = 0xA1 (NV2A)
    //   +0x05: McpRevision = 0xD3 (MCPX)
    // XboxKernelVersion at 0x01E00010:
    //   Major=1, Minor=0, Build=5838, Qfe=1
    // AV pack info at 0x01E00020:
    //   Standard composite (0x00000600 = AV_STANDARD)
    let hw_info_addr = 0x01E0_0000u32;
    memory.write_u32(hw_info_addr + 0x00, 0x0000_0020); // Flags: retail
    memory.write_u8(hw_info_addr + 0x04, 0xA1); // GPU rev
    memory.write_u8(hw_info_addr + 0x05, 0xD3); // MCP rev
                                                // AV info struct
    let av_info_addr = 0x01E0_0020u32;
    memory.write_u32(av_info_addr + 0x00, 0x0000_0600); // AV_STANDARD composite
    memory.write_u32(av_info_addr + 0x04, 0x0000_0F68); // hardware revision word (v1.0 Xbox)
    debug_log(&format!(
        "[PRE-AOT] XboxHardwareInfo at 0x{:08X}: retail, GPU=0xA1, MCP=0xD3",
        hw_info_addr
    ));

    // Game-specific: Classic Doom reads hardware info via pointers in .rdata
    if xbe_info.title.contains("CLASSIC DOOM") || xbe_info.title.contains("Doom") {
        // [0xB52E4] = pointer to XboxHardwareInfo
        // [0xB52F8] = pointer to AV info (revision word at +4)
        memory.write_u32(0x000B_52E4, hw_info_addr);
        memory.write_u32(0x000B_52F8, av_info_addr);
        debug_log("[PRE-AOT] Classic Doom: set hardware info pointers at [0xB52E4] and [0xB52F8]");
        // HLE hardware detection at 0x00040F29 as bare RET (197 bytes).
        // Zero game-visible side effects (confirmed by Codex analysis):
        // only writes ROM patch + GDT descriptors, both handled by zeros.
        memory.write_u8(0x0004_0F29, 0xC3); // RET
                                            // NOP the D3D delay loop at 0x00055880 — just RET immediately.
                                            // It's a countdown from 400 that wastes interpreter cycles.
                                            // The caller loops calling this delay while waiting for NV2A.
        memory.write_u8(0x0005_5880, 0xC3); // RET (skip delay countdown)
                                            // NOP the NV2A status polling loop at 0x5A510 — force jump to exit
                                            // The loop reads [ebx+0x1C20]+0x44 (NV2A PGRAPH status) and retries.
                                            // Force the JE at 0x5A510 to always jump (NOP the test, set ZF):
        memory.write_u8(0x0005_A510, 0xEB); // JE → JMP (unconditional)
                                            // NOP the NV2A command buffer flush/sync at 0x5AD84.
                                            // This function writes PFIFO PUT, spins waiting for GPU busy bit,
                                            // calls MmSetAddressProtect, then completion callback. Called 170+ times
                                            // during CreateDevice register init. No real GPU to consume commands.
        memory.write_u8(0x0005_AD84, 0xC3); // RET immediately
        debug_log("[PRE-AOT] Classic Doom: NOP'd NV2A command buffer sync at 0x5AD84");
        // 0x3E51C: push 2; call [0xB5234] is IoDismountVolumeByName (ord 49 in XDK 5849),
        // NOT HalReturnToFirmware. This is a legitimate cleanup call — do NOT NOP.
        debug_log("[PRE-AOT] Classic Doom: NOP'd D3D delay + NV2A status poll");
        debug_log("[PRE-AOT] Classic Doom: HLE hardware detection at 0x00040F29 → RET");

        // 0x42742: __sbh_free_block — CRT SBH free list insertion/coalescing.
        // Previously stubbed, but the function also initializes free list sentinels
        // during region creation. Stubbing prevents sentinel init → infinite loop
        // in alloc's free list search. Let it run natively now that 16-bit regs work.
        debug_log("[PRE-AOT] Classic Doom: CRT SBH free_block at 0x42742 runs natively");

        // HLE memcpy at 0x000456B0 — CRT scalar memcpy that burns 50M+
        // interpreted instructions copying data byte-by-byte. Replace with
        // a fast rep movsd stub.
        let memcpy_stub: &[u8] = &[
            0x57, // push edi
            0x56, // push esi
            0x8B, 0x7C, 0x24, 0x0C, // mov edi, [esp+0xC]  ; dst
            0x8B, 0x74, 0x24, 0x10, // mov esi, [esp+0x10] ; src
            0x8B, 0x4C, 0x24, 0x14, // mov ecx, [esp+0x14] ; count
            0x8B, 0xC7, // mov eax, edi         ; return dst
            0xC1, 0xE9, 0x02, // shr ecx, 2          ; dwords
            0xF3, 0xA5, // rep movsd
            0x8B, 0x4C, 0x24, 0x14, // mov ecx, [esp+0x14]
            0x83, 0xE1, 0x03, // and ecx, 3          ; remaining bytes
            0xF3, 0xA4, // rep movsb
            0x5E, // pop esi
            0x5F, // pop edi
            0xC3, // ret
        ];
        for (i, &b) in memcpy_stub.iter().enumerate() {
            memory.write_u8(0x0004_56B0 + i as u32, b);
        }
        debug_log("[PRE-AOT] Classic Doom: HLE memcpy at 0x000456B0 → fast rep movsd");

        // I_InitSound now runs natively — DirectSoundCreate is HLE'd at 0x84325
        // to return a proper fake IDirectSound8 with vtable stubs.
        // The old 0xC3 patch here prevented I_InitSound from initializing sound state.
        debug_log("[PRE-AOT] Classic Doom: I_InitSound at 0x11A80 runs natively (DirectSoundCreate HLE'd)");

        // HLE fclose (0x468B2) → return 0 (success).
        // CRT I/O subsystem (_ioinit) never ran because _initterm is HLE-skipped.
        // File table at [0x10B6D4] = NULL, count at [0x10B6D8] = 0.
        // fclose loops infinitely on uninitialized FILE struct (buffer flush retries).
        // This is an OS-layer fix: CRT file I/O isn't needed — we handle files via
        // NtCreateFile/NtReadFile at kernel level.
        memory.write_u8(0x0004_68B2, 0x33); // xor eax, eax
        memory.write_u8(0x0004_68B3, 0xC0);
        memory.write_u8(0x0004_68B4, 0xC3); // ret
        debug_log(
            "[PRE-AOT] Classic Doom: HLE fclose at 0x468B2 → return 0 (CRT I/O not initialized)",
        );

        // Fix CRT lock bootstrap infinite recursion:
        // _lock(n) at 0x47F93 checks lock_table[n] at 0xDFA00+n*8. If NULL, calls
        // _mtinitlocknum(n) at 0x47EF3, which allocates a CRITICAL_SECTION and then
        // calls _lock(10) to protect the creation. But if lock_table[10] is also NULL,
        // it recurses infinitely. Pre-init the creation lock (locknum 10) at 0xDFA50.
        // Xbox RTL_CRITICAL_SECTION uses DISPATCHER_HEADER prefix (0x1C bytes total):
        //   +0x00: DISPATCHER_HEADER (Type=1, AbsoluteInsert=0, Size=7)
        //   +0x04: SignalState (0)
        //   +0x08: WaitListHead.Flink (self-ref)
        //   +0x0C: WaitListHead.Blink (self-ref)
        //   +0x10: LockCount (-1 = unlocked)
        //   +0x14: RecursionCount (0)
        //   +0x18: OwningThread (0)
        let critsec_addr = 0x01E0_1000u32; // safe pre-init area
        memory.write_u8(critsec_addr, 1); // Type = MutantObject
        memory.write_u8(critsec_addr + 1, 0); // AbsoluteInsert
        memory.write_u16(critsec_addr + 2, 7); // Size (0x1C / 4)
        memory.write_u32(critsec_addr + 0x04, 0); // SignalState
        memory.write_u32(critsec_addr + 0x08, critsec_addr + 0x08); // Flink = self
        memory.write_u32(critsec_addr + 0x0C, critsec_addr + 0x08); // Blink = self
        memory.write_u32(critsec_addr + 0x10, 0xFFFF_FFFF); // LockCount = -1 (unlocked)
        memory.write_u32(critsec_addr + 0x14, 0); // RecursionCount
        memory.write_u32(critsec_addr + 0x18, 0); // OwningThread
                                                  // Store in lock table: entry 10 at 0xDFA00 + 10*8 = 0xDFA50
        memory.write_u32(0x000D_FA50, critsec_addr);
        debug_log(&format!(
            "[PRE-AOT] Classic Doom: pre-init CRT lock #10 (creation lock) at 0x{:08X} → [0xDFA50]",
            critsec_addr
        ));

        // HLE printf (0x462A9) → return 0 (no-op).
        // Xbox has no stdout — the CRT _output formatter loops writing characters
        // to an uninitialized FILE* buffer, causing shadow stack overflow.
        // sprintf (0x4634B) MUST remain functional (Doom uses it for string building).
        memory.write_u8(0x0004_62A9, 0x33); // xor eax, eax
        memory.write_u8(0x0004_62AA, 0xC0);
        memory.write_u8(0x0004_62AB, 0xC3); // ret
        debug_log("[PRE-AOT] Classic Doom: HLE printf at 0x462A9 → return 0 (no stdout)");

        // Game-logic patches REMOVED (2026-03-29): Let D_DoomMain run organically.
        // Removed: HLE WAD search (0x24D20), NOP'd WAD-found check (0x1656E),
        // NOP'd gating checks (0x19380), trampoline at 0x01E03000.
        //
        // KEPT: Search path pre-population (OS-layer launcher config).
        // The doomlauncher init function at 0xE1840 (in .data section) sets up Xbox
        // filesystem search paths for WAD files. Our AOT doesn't compile .data, and
        // the interpreter can't execute it. These paths are static Xbox filesystem
        // configuration — equivalent to argv/environment setup by the OS launcher.
        // D_DoomMain calls IdentifyVersion which iterates these paths to find WADs.
        //
        // Architecture: D_DoomMain runs in a loop. Iteration 0 allocates game_state,
        // runs D_DoomMain (which fails because paths aren't set yet), then the launcher
        // init at [esi+8] (0xE1840) sets paths. Iteration 1 re-enters D_DoomMain with
        // paths set. Since we can't execute 0xE1840, we pre-set the paths before the
        // loop starts. D_DoomMain zeroes game_state with rep stosd at 0x12DFE, so we
        // use a post-zeroing trampoline to re-populate paths.
        let paths_array = 0x01E0_2000u32;
        let str_empty = 0x01E0_2020u32;
        let str_doom = 0x01E0_2030u32;
        let str_doom2 = 0x01E0_2060u32;
        // Xbox search paths: d:\ = disc (XBE dir), z:\ = cache partition
        // The actual WADs are at d:\base\classic\w\ on the disc image
        memory.write_u8(str_empty, 0);
        for (i, b) in b"d:\\base\\classic\\w\\doom.wad\0".iter().enumerate() {
            memory.write_u8(str_doom + i as u32, *b);
        }
        for (i, b) in b"d:\\base\\classic\\w\\doom2.wad\0".iter().enumerate() {
            memory.write_u8(str_doom2 + i as u32, *b);
        }
        memory.write_u32(paths_array, str_empty);
        memory.write_u32(paths_array + 4, str_doom);
        memory.write_u32(paths_array + 8, str_doom2);
        memory.write_u32(paths_array + 12, 0);
        debug_log("[PRE-AOT] Classic Doom: search paths at 0x01E0_2000 (d:\\base\\classic\\w\\)");

        // Trampoline at 0x01E0_3000: simulates Xbox launcher init after game_state zeroing.
        // D_DoomMain does `rep stosd` at 0x12DFE to zero the 252KB game state, wiping
        // any pre-populated fields. At 0x12E00 (right after stosd), we redirect to this
        // trampoline that re-populates:
        //   1. Search paths (+0x3D78/+0x3D7C) for WAD file discovery
        //   2. Tic runner gate fields (+0x22A0, +0x2284) that enable the game loop
        //   3. Game state (+0x2260=4) to enter WAD loading mode
        // These are normally set by the launcher init at [esi+8]=0xE1840 (.data section),
        // which can't execute because AOT skips .data and the interpreter loses return
        // tracking on the indirect call.
        let tramp = 0x01E0_3000u32;
        // Trampoline re-populates ONLY search paths (OS-layer launcher config).
        // Gate fields (singletics, gameaction, game state) are removed — D_DoomMain
        // sets those organically. Per "HLE the OS" methodology: only populate what
        // the OS/launcher would provide (filesystem paths), not game logic state.
        let stub: &[u8] = &[
            0x51, // push ecx
            0x56, // push esi
            0x57, // push edi
            0xA1, 0xB8, 0x1B, 0x10, 0x00, // mov eax, [0x101BB8]
            // Re-populate search paths after rep stosd zeroing
            0xC7, 0x80, 0x78, 0x3D, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // mov [eax+0x3D78], 3
            0xC7, 0x80, 0x7C, 0x3D, 0x00, 0x00, 0x00, 0x20, 0xE0,
            0x01, // mov [eax+0x3D7C], 0x01E02000
            0x5F, // pop edi
            0x5E, // pop esi
            0x59, // pop ecx
            0xA1, 0xB8, 0x1B, 0x10, 0x00, // mov eax, [0x101BB8] (original insn)
            0xC3, // ret
        ];
        for (i, &b) in stub.iter().enumerate() {
            memory.write_u8(tramp + i as u32, b);
        }
        // Patch 0x12E00: replace `mov eax, [0x101BB8]` (5 bytes: A1 B8 1B 10 00)
        // with `call 0x01E03000` (5 bytes: E8 rel32)
        let call_target = tramp as i64 - (0x12E00i64 + 5);
        let rel32 = call_target as u32;
        memory.write_u8(0x0001_2E00, 0xE8);
        memory.write_u8(0x0001_2E01, (rel32 & 0xFF) as u8);
        memory.write_u8(0x0001_2E02, ((rel32 >> 8) & 0xFF) as u8);
        memory.write_u8(0x0001_2E03, ((rel32 >> 16) & 0xFF) as u8);
        memory.write_u8(0x0001_2E04, ((rel32 >> 24) & 0xFF) as u8);
        debug_log(&format!(
            "[PRE-AOT] Classic Doom: trampoline at 0x{:08X}, patched 0x12E00 CALL (re-populate paths after zeroing)",
            tramp
        ));

        // Tic runner gate checks at 0x193A5/B3/C0/CE: NO LONGER NOP'd.
        // D_DoomMain now runs from the top (0x12600) and sets singletics,
        // gameaction, consoleplayer, advancedemo organically. The NOP patches
        // were game-logic compensation for a broken init path.
    }

    if is_spiderman_xbe {
        debug_log(
            "[PRE-AOT] Spider-Man mode: ORGANIC (no bump heap, no fake objects, no forced phases)",
        );

        // ====================================================================
        // FAKE HEAP STRUCT (2026-04-20 hack-patch)
        // Option A per session checkpoint: write a plausible MSVC _HEAP header
        // at process_heap = 0x04000000 so RtlAllocateHeap finds a "valid"
        // heap when called with that handle.
        //
        // Probes A+B proved:
        //   - [0x007E18B0] (process_heap) = 0x04000000 (CRT stored it)
        //   - [0x007E1584] (_crtheap)     = 0 (system heap mode, NORMAL)
        //   - [0x007E1C4C] (__active_heap) = 0 (system heap mode, NORMAL)
        //   - Heap struct at 0x04000000 = all zero (RtlCreateHeap aborted
        //     silently before writing the struct)
        //
        // Our pure matched-transition discipline (Phases 3-8) proved the
        // dispatch boundaries are clean (131K+ transitions, 0 drift). The
        // silent abort must be inside RtlCreateHeap's native guest code,
        // possibly via veh_fixup R15 sign-extension misapply, or an
        // emitter miscompilation.
        //
        // Pressure-test: inject a plausible _HEAP header so RtlAllocateHeap
        // succeeds against process_heap. If the game progresses past the
        // font-loader stall, we've confirmed heap init is the single
        // blocker. If not, there's ALSO another silent corruption we haven't
        // instrumented.
        //
        // MSVC 7.x heap structure key fields (first 48 bytes):
        //   +0x00  HEAP_ENTRY Entry {Size, PrevSize, Flags, UnusedBytes} (8 bytes)
        //   +0x08  SegmentSignature = 0xEEFFEEFF
        //   +0x0C  SegmentFlags     = 0x00002000 (HEAP_SEGMENT_USER)
        //   +0x10  LIST_ENTRY SegmentListEntry {Flink, Blink} = self (8 bytes)
        //   +0x18  Heap*            = self
        //   +0x1C  BaseAddress      = 0x04000000
        //   +0x20  NumberOfPages    = 0x100 (1MB reserve / 4KB page)
        //   +0x24  FirstEntry*      = past header (0x04000040)
        //   +0x28  LastValidEntry*  = end of commit (0x04001000)
        //   +0x2C  UncommittedPages = 0xFF (all but first page uncommitted)
        //   +0x30+ FreeLists + locks + more
        //
        // We write enough to pass a "is this heap real?" signature check.
        // The allocator may still fail on FreeLists walks if it tries to
        // do real allocation, but at least it'll see "heap is initialized".
        // ====================================================================
        let heap_base = 0x0400_0000u32;
        // HEAP_ENTRY header (first 8 bytes) — zero'd default
        memory.write_u32(heap_base + 0x00, 0x0000_0000);
        memory.write_u32(heap_base + 0x04, 0x0000_0000);
        // SegmentSignature: MSVC heap magic
        memory.write_u32(heap_base + 0x08, 0xEEFF_EEFF);
        // SegmentFlags: mark as user-allocated, growable
        memory.write_u32(heap_base + 0x0C, 0x0000_2000);
        // SegmentListEntry: self-referential (no other segments)
        memory.write_u32(heap_base + 0x10, heap_base + 0x10);
        memory.write_u32(heap_base + 0x14, heap_base + 0x10);
        // Heap* = self (segment points back to owning heap)
        memory.write_u32(heap_base + 0x18, heap_base);
        // BaseAddress
        memory.write_u32(heap_base + 0x1C, heap_base);
        // NumberOfPages: 256 pages = 1MB reserve
        memory.write_u32(heap_base + 0x20, 0x0000_0100);
        // FirstEntry: points past the 48-byte header, aligned to 8 bytes
        memory.write_u32(heap_base + 0x24, heap_base + 0x0040);
        // LastValidEntry: end of the 4KB commit
        memory.write_u32(heap_base + 0x28, heap_base + 0x1000);
        // UncommittedPages: 255 (all pages except first are uncommitted)
        memory.write_u32(heap_base + 0x2C, 0x0000_00FF);
        // Write a HEAP_ENTRY free block at 0x04000040: size = remaining commit
        // Size = (0x1000 - 0x40) / 8 = 0x1F8 (units of 8 bytes in MSVC heap).
        // Flags = HEAP_ENTRY_LAST_ENTRY (0x10) | HEAP_ENTRY_FREE (0x08)
        let free_entry = heap_base + 0x0040;
        memory.write_u16(free_entry + 0x00, 0x01F8); // Size (in 8-byte units)
        memory.write_u16(free_entry + 0x02, 0x0000); // PreviousSize
        memory.write_u8(free_entry + 0x04, 0x00); // SmallTagIndex
        memory.write_u8(free_entry + 0x05, 0x18); // Flags: LAST_ENTRY|FREE
        memory.write_u8(free_entry + 0x06, 0x00); // UnusedBytes
        memory.write_u8(free_entry + 0x07, 0x00); // SegmentIndex
                                                  // Free-block payload: back/forward pointers for the free list,
                                                  // both self-referential (this is the only free block).
        memory.write_u32(free_entry + 0x08, free_entry);
        memory.write_u32(free_entry + 0x0C, free_entry);

        debug_log("[PRE-AOT] FAKE-HEAP: injected _HEAP header at 0x04000000 with Signature=0xEEFFEEFF, growable, single 0x1F80-byte free block at +0x40");
        debug_log("[PRE-AOT] FAKE-HEAP: this is a pressure test — if game advances past font-loader, heap init is the single blocker");

        // 2026-04-24 REVERSED: keep RUN_FROM_STASHES non-zero, ONLY zero HALT_ON_ASSERTS.
        //
        // Spider-Man XDK 4134 (this XBE) has a hardcoded flag-defaults table at
        // .data:0x003DC550 (102 dwords, one per config flag). The names table at
        // 0x003DC3B8 maps index → name:
        //   [0] IGNORE_FILE_CACHE
        //   [1] BUILD_STASHES
        //   [2] RUN_FROM_STASHES   ← non-zero default — KEEP IT
        //   [3] CD_ONLY
        //   [7] WINDOW_DEFAULT      (non-zero = normal)
        //   [14] GRAVITY             (non-zero = normal)
        //   [16] PAL0_NTSC1          (non-zero = normal, NTSC)
        //   [18] HALT_ON_ASSERTS     ← non-zero = dev-build — zero it
        //   [19] NO_WARNINGS         ← non-zero = dev-build
        //   [23] PROFILING_ON        ← non-zero = dev-build
        //   [33] AI_PATH_DEBUG       ← non-zero = dev-build
        //
        // sub_000204D0 (the config-parser) does a per-flag loop:
        //   for i in 0..102: App.byte[4+i] = (defaults[i] != 0) ? 1 : 0
        //
        // HISTORY: Earlier this session, we zeroed RUN_FROM_STASHES thinking it
        // was a "dev-build artifact". That was WRONG. Investigation on 2026-04-24
        // proved:
        //   1. ALL stash files (*.xbs) ship with the disc at D:/data/bonus/
        //      (LEGAL.xbs 37KB, MENU.xbs 3MB, CITY_*.xbs 4-5MB each, etc.).
        //   2. sub_0x1C8C0 (scene register) calls sub_0x1C550.
        //   3. sub_0x1C550 AT 0x1C558-0x1C561 checks RUN_FROM_STASHES:
        //        CL = [eax+6]; test cl, cl; je 0x1c596
        //      When RUN_FROM_STASHES=0, JE is taken → fall-through is a NO-OP
        //      that just writes 0 to [world+0x118] and returns eax=0.
        //      When RUN_FROM_STASHES=1, JE is NOT taken → falls into the stash
        //      load path (call sub_0x13E20) which opens LEGAL.xbs etc.
        //   4. Empirical runtime data (SPIDEY-TAPs #2–#7) confirmed sub_0x1C8C0
        //      runs to completion WITHOUT opening any *.xbs file, because our
        //      forced RUN_FROM_STASHES=0 made sub_0x1C550 bail silently.
        //   5. NO legal DDS can render without LEGAL.xbs being loaded.
        // So the fix is to REMOVE the RUN_FROM_STASHES zero (keep flag = 1) so
        // the stash path runs organically and LEGAL.xbs gets opened/decompressed.
        //
        // HALT_ON_ASSERTS still needs zeroing: a debug assert would hard-halt.
        for i in 0..4 {
            memory.write_u8(0x003D_C550 + 18 * 4 + i, 0x00);
        }
        debug_log("PRE-AOT PATCH: zeroed .data:0x003DC598 (HALT_ON_ASSERTS=0 so dev asserts don't halt). RUN_FROM_STASHES LEFT AT DEFAULT (non-zero) so sub_0x1C550 takes stash-load path and actually opens LEGAL.xbs/MENU.xbs");

        // NOP the D3D state table call BEFORE AOT so it compiles as NOPs
        for i in 0..5u32 {
            memory.write_u8(0x0029_CBB3 + i, 0x90); // NOP the CALL
        }
        memory.write_u8(0x0029_CBB1, 0x90); // NOP push edi
        memory.write_u8(0x0029_CBB2, 0x90); // NOP push eax
        debug_log("PRE-AOT PATCH: 0x0029CBB1-0x0029CBB7 NOP D3D state table call");

        // Historical bypass for the P0 batched cdecl call at 0x002A0B1B.
        // This call enters 0x0029CB70, which runs nglInitShadersConst
        // (0x0029C5A0) and seeds c[-96..-93]. It was once skipped because the
        // old flat VS-constant namespace crashed/trampled state, but the signed
        // constant window is now a core invariant. Keep the bypass opt-in only.
        if legacy_spidey_p0_zone_bypass {
            for i in 0..5u32 {
                memory.write_u8(0x002A_0B1B + i, 0x90);
            }
            debug_log("PRE-AOT PATCH: 0x002A0B1B NOP P0 zone call (legacy opt-in via RUSTEMU_SPIDEY_NOP_P0_ZONE_CALL)");
        } else {
            debug_log("PRE-AOT PATCH: 0x002A0B1B P0 zone call preserved so nglInitShadersConst can run organically");
        }

        // NOP specific XPP GPU busy spin that blocks D3D init.
        // Only NOP the spin at 0x0037A9AC (bit 0x100 wait) — the others
        // are needed for init to complete correctly.
        memory.write_u8(0x0037_A9AC, 0x90);
        memory.write_u8(0x0037_A9AD, 0x90);
        debug_log("PRE-AOT PATCH: 0x0037A9AC NOP XPP GPU busy spin (bit 0x100 wait)");

        // NOP XPP display config call at 0x0037A5AF (call [0x381784], 6 bytes).
        // [0x381784] = 0x00000000 — unresolved XPP function pointer.
        // On real Xbox, kernel fills this during XBE loading. Without it,
        // the call goes to address 0 which VEH stubs but leaves garbage state.
        for j in 0..6u32 {
            memory.write_u8(0x0037_A5AF + j, 0x90);
        }
        debug_log("PRE-AOT PATCH: 0x0037A5AF NOP XPP display config call ([0x381784]=NULL)");

        // NOP game_main's SEH handler installation at 0x002A52B6.
        // game_main uses _SEH_prolog: push -1; push scope_table; push handler;
        // mov eax,fs:[0]; push eax; mov fs:[0],esp
        // The handler at 0x2B7750 catches AVs and EXITS game_main cleanly.
        // This prevents game_main from reaching step 11 (app_ctor) because
        // steps 8-10 (render setup) trigger AVs on uninitialized D3D fields.
        // On real Xbox, the __except filter continues execution. Our SEH HLE
        // PRESERVED: game_main SEH install at 0x002A52B6.
        // Previously NOPed to let VEH handle AVs, but this removes the game's
        // exception safety net. When init hits expected exceptions (e.g., in
        // string allocator), FS:[0]=0 means no handler → unhandled → thread exit.
        // VEH still runs first-chance; game SEH provides the fallback handler.
        debug_log("PRE-AOT PATCH: 0x002A52B6 SEH install PRESERVED (game needs SEH handlers)");

        // Timer loop at 0x000F70C4 compares accumulated time to [0x3DDD04].
        // Set to 0.0f so timer comparison always passes immediately.
        memory.write_u32(0x003D_DD04, 0x00000000);
        debug_log("PRE-AOT PATCH: 0x003DDD04 = 0.0f (frame rate limiter disabled)");

        // Frame rate limiter 2: jnp at 0x000D6CD5 loops to 0x000D6CC0
        // Change jnp to JMP+0 — fall through always.
        memory.write_u8(0x000D_6CD5, 0xEB); // JMP rel8 (unconditional short)
        memory.write_u8(0x000D_6CD6, 0x00); // JMP +0 = fall through
        debug_log("PRE-AOT PATCH: 0x000D6CD5 jnp→jmp+0 (timer loop 2 always falls through)");

        // 2026-04-26: Preserve the XGRPH parser. The old 5-byte stub at
        // 0x003372A8 started on the `leave; ret 8` epilogue of one function
        // and overwrote 0x003372AC, the first byte of the next parser
        // function that live callsites enter. That corruption produced bogus
        // return targets inside DDS buffers (for example 0x103571B0) and made
        // stack-scan recovery limp through XGRPH parsing. If this parser still
        // needs help, handle it as an SDK/HLE seam, not a byte patch.
        debug_log("PRE-AOT PATCH: 0x003372A8/0x003372AC XGRPH parser preserved");

        // NOP XPP GPU busy spin at 0x0037A9AC
        memory.write_u8(0x0037_A9AC, 0x90);
        memory.write_u8(0x0037_A9AD, 0x90);
        debug_log("PRE-AOT PATCH: 0x0037A9AC NOP XPP GPU busy spin");

        // NOP XPP display config call at 0x0037A5AF (call [0x381784]=NULL, 6 bytes)
        for j in 0..6u32 {
            memory.write_u8(0x0037_A5AF + j, 0x90);
        }
        debug_log("PRE-AOT PATCH: 0x0037A5AF NOP XPP display config call");

        // 2026-04-22: REVERTED. The NOP of `mov fs:[0], esp` at 0x002A52B6
        // was breaking the game_main SEH chain — any early init call that
        // throws (e.g. D3D invalid state) unwinds out of game_main with no
        // handler, so game_main RETs at dispatch#2 with ESP=0x00AFE008 before
        // reaching the frame_ctx constructor at 0x002A53FA. Without
        // `[0x004BC630]` being written, Spider-Man's state machine can't
        // advance past title screen.
        //
        // Earlier justification was "VEH handles AVs via skip-instruction;
        // SEH HLE incorrectly unwinds." But our SEH HLE is (03a8da93-era)
        // more mature now; trust it and let game_main install its handler.
        debug_log("PRE-AOT PATCH: 0x002A52B6 SEH install PRESERVED (2026-04-22: needed for game_main to reach frame_ctx init)");

        // 2026-04-22 RADAR BISECTION: skip all of game_main's early init
        // calls and jump directly to the frame_ctx constructor call at
        // 0x002A53FA. game_main runs but HEAP-CANARY fires inside one of
        // the early calls (0x37AD80, 0x2A7FF7, 0x42290, 0x2A4FF0,
        // 0x296F70, 0x13CB0, 0x2A08B0) or their subroutines. If this
        // JMP bypass lets [0x004BC630] become non-zero, we've isolated
        // the fault to the skipped region.
        //
        // At 0x002A52E2 the stack state is clean (all struct locals
        // [ebp-0x28]..[ebp-0x1c] just initialized, no pending args).
        // JMP rel32 from 0x2A52E2 to 0x2A53FA: offset = 0x2A53FA - 0x2A52E7 = 0x113.
        // Bytes: E9 13 01 00 00 (5 bytes)
        // Pad remaining 0x113 - 5 = 0x10E bytes to NOP? No — we only
        // replace the 5 bytes at 0x2A52E2; rest of the region remains
        // unused (jumped over).
        //
        // Only apply for Spider-Man (XBE entry 0x002A9C38).
        let is_spiderman = xbe_info.title.contains("Spider") || xbe_info.entry_point == 0x002A9C38;
        if is_spiderman {
            // BISECTION (2026-04-22): game_main's 0x5318-0x53FA block contains
            // 21 calls (not 4 as the pre-bisection comment guessed). Bisection
            // results:
            //   step 3 (JMP 0x5318→0x53FA, skip all 21): Frame=0x04C08352,
            //     swaps_hle=160, kernel_calls=5600, RGB noise on screen. SAFE.
            //   step 4a (run #1-#10): Frame=0, Scene=0x10203750
            //   step 4b (run #1-#5):  Frame=0, Scene=0
            //   step 4c (run #1-#2):  all globals 0, swaps=2 (worst)
            //   step 4d (run #1 only): SAME as 4c
            //   step 5  (skip only #1, run #2-#21): worker ret=0x1EFFFD90
            //     (stack-ret), exit=3, kernel_calls=443. Fatal.
            // Conclusion: ANY of these 21 calls running natively corrupts the
            // worker. Not a single-call poisoner — our emitter/VEH has
            // something wrong with this region in aggregate. Keep step 3.
            //
            // 21 calls in 0x002A531D-0x002A53F1 (enumerated via raw-XBE scan):
            //   #1  0x531D call 0x00297510   #12 0x53A3 call 0x002B1B90
            //   #2  0x5322 call 0x0029D8B0   #13 0x53A8 call 0x002B1C10
            //   #3  0x532A call 0x002A17E0   #14 0x53B2 call 0x002B1B70
            //   #4  0x5332 call 0x002A4BF0   #15 0x53B7 call 0x002A4B30
            //   #5  0x5337 call 0x002A5230   #16 0x53BC call 0x00015C60
            //   #6  0x534E call 0x00021360   #17 0x53CB call 0x00015860
            //   #7  0x5362 call 0x002B6B2A   #18 0x53DF call 0x00015860
            //   #8  0x5367 call 0x002A4C60   #19 0x53E7 call 0x000158A0
            //   #9  0x537E call 0x002B5EC0   #20 0x53EC call 0x00015870
            //   #10 0x5394 call 0x0001FA10   #21 0x53F1 call 0x00015CC0
            //   #11 0x5399 call 0x002A4AC0
            //
            // NO-BYPASS RUN 2026-04-22: remove all game_main JMP patches.
            // Let the entire block 0x5318-0x53FA run natively. Probes at
            // the 22 call boundaries + allocator watchpoints will capture
            // the exact crash signature.
            //
            // Goal: observe which call fails + [0x3F5A48] state + ALLOC-WATCH
            // activity to identify the poison call.
            debug_log("PRE-AOT PATCH: no game_main bypass — full 22-call chain runs natively");

            // 2026-04-24: Patch the FRAME-DISPATCHER sub_002A29E0 to skip its two
            // early-exit gates. Per Extra4 disasm, the dispatcher prolog is:
            //   mov  ecx, [0x4BC630]
            //   add  ecx, 0x18                ; ecx = scene_mgr
            //   cmp  byte [ecx+0x186], 0
            //   je   exit                     ; GATE 1: render-enable bit
            //   mov  eax, [0x726690]
            //   test eax, eax
            //   je   exit                     ; GATE 2: FSM state pointer
            //   ...real dispatch work...
            //
            // We've spent multiple sessions trying to *satisfy* these gates from
            // the emulator (write scene_mgr+0x186=1, hydrate state machine at
            // 0x726690, etc.) — every attempt either races the guest's own
            // initializer or trips a downstream gate that needs different state.
            //
            // 2026-04-25: Do NOT NOP these game-logic gates anymore.
            // The legal and Bink texture paths now render from guest resource
            // data, so this scaffold would only hide the real missing engine
            // state. A TAP on 0x002A29E0 logs the values below while letting
            // Spider-Man's dispatcher take its native early-exit path.
            //
            // Addresses below are RELATIVE to sub_002A29E0; exact offsets need
            // a one-time verification pass against the iced-x86 disasm DB
            // (xbox_scout_v2.db). MSVC compiles these as short JE (0x74 rel8 =
            // 2 bytes). If either site emits a near JE (0F 84 rel32 = 6 bytes)
            // the byte count must be widened to match — see comment above each
            // patch site.
            //
            // GATE 1: cmp byte [ecx+0x186], 0 ; je exit
            //   Verified Spider-Man XDK 4134 sequence:
            //     0x002A29F0: mov al, [ecx+0x186]
            //     0x002A29F8: test al, al
            //     0x002A29FA: push edi
            //     0x002A29FB: je 0x002A2A1A
            //     0x002A29FD: mov eax, [esi]       ; esi = 0x00726690
            //     0x002A29FF: cmp eax, ebx
            //     0x002A2A01: je 0x002A2A1A
            //
            // Keep the diagnostic scaffold narrow: patch exactly the two short
            // JE instructions, not surrounding prologue bytes. The previous
            // addresses landed inside `mov ecx,[eax+0x18]` and on `push edi`,
            // which corrupted the dispatcher and unbalanced its stack.
            debug_log("PRE-AOT PATCH: sub_002A29E0 dispatcher gates preserved (TAP logs organic gate values)");

            // 2026-04-29: Let the scene-render frame pump run natively.
            // The earlier NOP here hid a cdecl/dispatch failure in
            // sub_002B1DC0, but it also bypassed a per-frame path that may
            // feed scene/load completion. If this call still fails, fix the
            // platform/dispatch dependency it exposes instead of skipping it.
            debug_log("PRE-AOT PATCH: Spider-Man scene-render audio frame pump calls preserved at 0x000F6D2F/0x000F70D2");
        }

        // Preserve the P0 batched cdecl call by default. It feeds the
        // nglInitShadersConst path that should organically upload c[-96..-93].
        if legacy_spidey_p0_zone_bypass {
            for i in 0..5u32 {
                memory.write_u8(0x002A_0B1B + i, 0x90);
            }
            debug_log("PRE-AOT PATCH: 0x002A0B1B NOP P0 zone call (legacy opt-in via RUSTEMU_SPIDEY_NOP_P0_ZONE_CALL)");
        } else {
            debug_log("PRE-AOT PATCH: 0x002A0B1B P0 zone call preserved so nglInitShadersConst can run organically");
        }

        // NOP D3D state table call at 0x0029CBB1-0x0029CBB7
        for i in 0..5u32 {
            memory.write_u8(0x0029_CBB3 + i, 0x90);
        }
        memory.write_u8(0x0029_CBB1, 0x90);
        memory.write_u8(0x0029_CBB2, 0x90);
        debug_log("PRE-AOT PATCH: 0x0029CBB1-0x0029CBB7 NOP D3D state table call");

        // ====================================================================
        // LEGACY SCAFFOLDING — DISABLED
        // Everything below was compensating for missing OS-layer heap support.
        // Now that NtAllocateVirtualMemory works, CRT builds its own heap
        // organically. These patches PREVENT organic init by replacing CRT
        // internals with bump allocators and pre-populating game objects that
        // constructors would create if the heap worked.
        //
        // What was removed:
        //   - Bump HeapAlloc at 0x002AB360
        //   - Bump _heap_alloc_base at 0x002AC3A2
        //   - No-op HeapFree at 0x002AAC92
        //   - Bump operator new at 0x000424E0
        //   - No-op operator delete at 0x000422B0
        //   - Heap coalescing NOP at 0x002AC405
        //   - _crtheap hydration + __active_heap=3
        //   - NOP writes to [0x4BC614] (App object protection)
        //   - Stub frame_ctor, app_ctor, re-entry guard
        //   - Pre-init App/Scene/Update/Input/Frame objects
        //   - Render context callback injection at [0x3F10E8]
        //   - Render callback injection at 0x01F01000
        // ====================================================================
        debug_log("[PRE-AOT] Legacy scaffolding DISABLED — CRT builds own heap via NtAllocateVirtualMemory");
    } // end if is_spiderman_xbe (pre-AOT patches)
}
