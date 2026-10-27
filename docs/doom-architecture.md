# CADCraft DOOM: pre-implementation architecture report

Inspected CADCraft 41d9b5e957b0719c04b9687c4f29088d77153c31 and DoomGeneric e257a09527062bfb986b6e5e0780839b6cbd5b71 on 2026-10-24.

- Cargo workspace: crates/*, apps/*, xtask. Rust 2024; desktop/UI require >=1.95; egui/eframe 0.36.
- Desktop entry: apps/cadcraft/src/main.rs, eframe App with logic/ui/raw_input_hook.
- CadApp in crates/ui-egui/src/lib.rs owns Session and CanvasState. draw creates CentralPanel; docking::show calls canvas::show for Panel::Canvas. Replace only that body and retain Session/CanvasState.
- Commands: engine CommandSpec registry; UI-only menus::UI_COMMANDS and menus::run_ui_command, reached by CadApp::run/start/cmdline. DOOM belongs in the UI registry, outside document transactions.
- Input: menus::shortcuts runs before rendering; canvas calls cmdline::keyboard. Raw input passes CadApp::raw_input_hook then cmdline::capture_tab. Intercept game input before these consumers.
- Textures: exchange.rs uses Context::load_texture(ColorImage). Keep a TextureHandle and update only when a new frame sequence arrives. Letterbox 320x200 pixels at classic corrected 4:3 display aspect.
- docs/customization.md: native plugins unsupported; Rust/Wasm plugin API planned. LISP/scripts/control channel cannot embed a native framebuffer. Implement an optional source feature.
- Existing build: cargo run --release -p cadcraft -- --sample. Gate: cargo xtask ci (fmt, clippy, tests, assets, layers, wasm).

## Planned integration

Dedicated Windows x64 MSVC crate isolates unsafe FFI; UI stays safe Rust. One Rust worker enters a C-only guarded loop; all engine globals/framebuffer remain on that thread. SRW locks protect a latest-frame copy and key-state mailbox. UI polls at 35 Hz while active; worker sleeps between ticks and implements DG_SleepMs. No OS game window or external game executable.

I_Error and I_Quit must bypass process exit and upstream cleanup; C-only setjmp/longjmp handles error, stop and quit without crossing Rust frames. Other exit calls are redirected. A Windows SEH boundary catches worker exceptions as a last resort, not as a security sandbox. Permit one invocation per process; retain engine resources until process exit because upstream lacks a complete teardown/reset API. DOOMEXIT restores CAD immediately and requests cooperative worker stop.

Validate IWAD header, bounded file/count/directory/lump ranges and essential map/render lumps before entering C. This does not validate every internal texture/map structure: use trusted FreeDoom releases only. Malformed native data is not a security-safe input format.

## Licensing decision before integration

CADCraft: MIT OR Apache-2.0. DoomGeneric: GPLv2 license and GPL-2.0-or-later source headers. A statically linked executable is a GPL-covered combined work; a separate FFI crate does not avoid copyleft. Select CADCraft's MIT option for GPLv2 compatibility. Apache-2.0-only dependencies require considering GPLv3, contingent on every included file allowing that version. Audit the full dependency/file inventory before redistribution; that audit is not complete here.

Preserve all copyright/license/NOTICE/attribution notices, identify modifications, provide the applicable GPL text and complete corresponding source/build scripts for the binary, and meet applicable source-offer/installation-information requirements. Do not label the combined executable MIT/Apache-only. Game data is separately licensed; no commercial WAD is included. FreeDoom redistribution requires its own notices. ArtCraft trademarks have separate restrictions requiring removal from modified distributions.

References: repository LICENSE-MIT, LICENSE-APACHE, NOTICE, ATTRIBUTION.md; DoomGeneric LICENSE and C file headers; https://www.gnu.org/licenses/gpl-faq.html#GPLStaticVsDynamic ; https://www.gnu.org/licenses/license-list.html#apache2 .

## Environment

MSVC 14.51 exists in Visual Studio 18 Community; workspace-local Rust 1.99.1 installed without PATH changes. Gitignored plan/ files are absent. Shared craftrules retrieval failed (authentication/404); not reviewed. User explicitly authorizes GPL/C/FFI for this fork, overriding upstream pure-Rust/no-GPL rules within this scope. Build/test evidence will be documented separately.
