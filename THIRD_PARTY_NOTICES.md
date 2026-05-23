# Third-Party Notices

This file documents the third-party source code that portions of Rustemu
are derived from, along with the original copyrights and licenses. It
exists to satisfy the GPL-2.0 attribution-preservation requirement and to
give clear credit to the upstream projects whose work this codebase
builds on.

All upstream sources listed here are GPL-2.0-compatible; Rustemu is
distributed under GPL-2.0-only.

---

## Cxbx-Reloaded

**Project**: Cxbx-Reloaded — original Xbox emulator
**Repository**: https://github.com/Cxbx-Reloaded/Cxbx-Reloaded
**License**: GPL-2.0-only
**Original copyright**: © Cxbx-Reloaded contributors

The following Rustemu files contain code or data ported from
Cxbx-Reloaded. Each file carries an in-file `Ported from`,
`Source:`, `Derived from`, or `Adapted from` comment pointing at
the specific upstream file (and where applicable, line range).

| Rustemu file | Cxbx-Reloaded source |
|---|---|
| `src/xbox/gpu/fvf_decode.rs` | `src/core/hle/D3D8/XbD3D8Types.h` (FVF bit constants, lines 1282–1312); `src/core/hle/D3D8/XbConvert.cpp` (primitive-type mappings, lines 1197–1224) |
| `src/xbox/gpu/render_state.rs` | `src/core/hle/D3D8/XbD3D8Types.h` (X_D3DRENDERSTATETYPE enum) |
| `src/xbox/gpu/mod.rs` | `src/core/hle/D3D8/XbD3D8Types.h` + `XbState.cpp` (render-state docstring/module structure) |
| `src/xbox/gpu/d3d11.rs` | `src/core/hle/D3D8/XbD3D8Types.h` (host extension VS constant register layout, snapshot `585c49a`); additional adapted snippets in surrounding code |
| `src/xbox/gpu/nv2a_vsh.rs` | `reverseScreenspaceTransform` helper (derived) |

The work in these files is GPL-2.0-only, both at the upstream and as
incorporated here. Modifications and Rust translation by d3r2000, 2026.

---

## XboxLibretro (project's own C++ predecessor)

**Project**: XboxLibretro — the C++ predecessor of this Rust port
**License**: GPL-2.0-only (same author, same copyright holder)

The following Rustemu files were ported from this project's prior C++
sources. Copyright is held by the same author (d3r2000); no third-party
licensing is involved here, but the lineage is documented for clarity.

| Rustemu file | XboxLibretro source |
|---|---|
| `src/xbox/aot/veh_dispatch.rs` | `AOT_VEH.cpp` (runtime rescue path) |
| `src/xbox/aot/veh_dpc.rs` | `AOT_VEH_DPC.cpp` |
| `src/xbox/kernel/file.rs` | `AOT_KrnlFile.cpp` |
| `src/xbox/kernel/hal.rs` | `AOT_KrnlHal.cpp` |
| `src/xbox/kernel/memory.rs` | `AOT_KrnlMem.cpp` |

---

## Xbox kernel ordinal tables

**Source**: Public Xbox kernel header structure (`XboxKernelList.h`,
`XboxKernelArgCount.h`)
**Type**: API spec / X-macro tables — function names, ordinal numbers,
argument counts

The following file mirrors the Xbox kernel's exported-function table.
This is API surface (factual ordinal numbers and signatures), not
copyrighted creative expression. The same information is independently
re-derived and openly published by Cxbx-Reloaded, xemu, ReactOS,
xboxdevwiki, and the OpenXDK / nxdk projects. Same legal category as
Wine documenting the NT API table.

| Rustemu file | Source |
|---|---|
| `src/xbox/kernel/ordinals.rs` | `XboxKernelList.h` + `XboxKernelArgCount.h` (X-macro tables) |

---

## Inspiration without direct code derivation

The remainder of the codebase is clean-room Rust written from public
documentation, behavioural observation, and the conceptual designs of
established emulator and interoperability projects. Those projects are
acknowledged in the README's "Acknowledgements" section. No code from
xemu, Xenia, Wine, Frida, ReactOS, box64, FEX-Emu, RPCS3, nxdk, or
pyxbe was copied; design ideas were studied and reimplemented.

---

## Reporting attribution gaps

If you maintain an upstream project listed above (or one not listed
here) and believe Rustemu has derived from your work without sufficient
attribution, please open an issue at
https://github.com/d3r2000/rustemu-xbox/issues. Attribution corrections
will be made promptly.
