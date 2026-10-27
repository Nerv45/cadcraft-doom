> Historical report: FreeDoom verification from 2026-10-24. Current original DOOM1.WAD and native Windows verification (2026-10-25): see the sibling output verification/WINDOWS-DOOM1-RESULTS.md.

# CADCraft DOOM verification

Verified on Windows x64 with Rust 1.99.1, MSVC 14.51, egui/eframe 0.36.2 and the
official FreeDoom 0.13.0 Phase 2 IWAD. Its SHA-256 is
`a8772e088847032510d97ba2312406a6998f21cbab44d4ff10696faa9c0ecd4b`.
No WAD is bundled in the source deliverable.

## End-to-end viewport verification: PASS

`cargo test --locked -p cadcraft-ui-egui --features doom --test doom_viewport -- --ignored --nocapture`

This opt-in test uses CADCraft's actual `CadApp::logic`, `CadApp::ui`, docking
layout, raw input hook, command line, texture upload and DoomGeneric worker. The
existing egui test renderer renders the full interface to images; no Doom game
window or external Doom executable is created.

The final run passed in 11.30 seconds:

- Opened and rendered CADCraft's sample Bracket drawing.
- Typed DOOM through the normal CAD command line and pressed Enter.
- Loaded the IWAD, displayed a title screen and started single-player gameplay.
- Sent Enter/menu and W/right-arrow movement through the raw input hook.
- Observed 459,639 changed pixels between the game view before and after movement.
  Visual inspection confirmed the player view and gun inside the central panel,
  with menus, tool sets, layers and properties surrounding it.
- Clicked the rendered Command field, typed DOOMEXIT and pressed Enter.
- Confirmed DOOM mode closed and the original drawing rendered again.
- Confirmed the identical drawing Arc, full Drawing equality, unchanged view,
  revision, undo count and redo count immediately after exit.
- Typed a new CAD CIRCLE command after exit; it edited the drawing normally.
  Undo restored the exact original drawing Arc.
- Confirmed a second native-engine launch was refused with the restart limitation.

Evidence is in the sibling `../verification/` output directory:
`01-cad-before.png`, `02-doom-title.png`, `03-doom-game.png`,
`04-doom-moved.png`, `05-cad-restored.png`.

## Other completed automated tests

- Native IWAD header/directory/range/required-lump validation: 2 tests passed.
- Missing and invalid WAD errors, simultaneous-worker rejection and validation
  retry: 1 integration test passed.
- Keyboard codes, aliases and release behavior: passed.
- Raw-input filtering, window-focus loss releasing keys, and document/undo/redo
  preservation on exit: passed.
- Native smoke: 168 observed frames, 116 frame changes in eight seconds;
  cooperative stop about 22 ms; second invocation rejected.
- DOOM's own internal Quit menu returned to the Rust host: passed.
- Repository layering check: 17 crates, no violations.

The checks/build results recorded in the final section below are the last checks
against the delivered source.

## Direct desktop verification limits

The desktop application compiled and opened a Windows window. Direct computer-use
inspection failed because the helper rejected its returned window identifier,
reporting that it no longer belonged to the same application identifier. Refresh
and the alternate native UI binding produced the same error. The local control
socket also failed to connect in this execution environment. Consequently, no
claim is made that physical keyboard/mouse input was driven in that desktop
window, or that the ten original scenarios all passed through desktop automation.

The rendered harness independently verifies the production UI, native engine,
input routing and exit/document preservation paths. Screenshots were inspected.
The native Windows event loop and physical input still merit a manual smoke test.

## Remaining limits

One engine session per CADCraft process; restart after exiting. Sound disabled.
No multiplayer, mouse look, macOS/Linux/Web DOOM support or release packaging.
Malformed WAD checks are structural; arbitrary hostile map/texture contents are
not comprehensively validated or sandboxed. Only trusted official FreeDoom data
was used. Other IWAD versions and maps beyond the starting area are untested.

The full upstream `cargo xtask ci`, cross-platform/Wasm builds and all unrelated
CAD tests have not been run. A full GPL/dependency license and trademark audit is
still required before public binary redistribution. See doom-architecture.md.

## Final checks

- `cargo build --locked -p cadcraft --features doom`: PASS (incremental, 44.68 s).
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --locked -p cadcraft-doom -p cadcraft-ui-egui -p cadcraft --features cadcraft/doom -- -D warnings`: PASS.
- `cargo test --locked -p cadcraft-doom`: PASS (2 unit + 1 integration tests).
- `cargo test --locked -p cadcraft-ui-egui --features doom --lib doom::`: PASS (2 tests).
- Opt-in `doom_viewport` render/input/exit test: PASS (1 test, 11.30 s).
- `cargo xtask layers`: PASS (17 crates).
- `cargo xtask assets`: PASS (28 assets).
- Local launcher PowerShell syntax parse: PASS.

The compiler reported that hard links are unavailable in this sandbox and copied
incremental-cache files instead. These environment warnings did not prevent the
strict lint or build from completing. No source-level Clippy warnings remain in
the checked production targets.

The final debug executable is built locally at `work/target/debug/cadcraft.exe`
relative to the generated project root. The sibling output launcher
`../Launch-CADCraft-DOOM.ps1` locates it and the separately downloaded test IWAD.


