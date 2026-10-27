# CADCraft DOOM

An **unofficial community modification** of [CADCraft](https://github.com/storytold/cadcraft)
that runs [DoomGeneric](https://github.com/ozkl/doomgeneric) directly inside the drawing
canvas. Not endorsed by the original developers, ArtCraft Team or id Software.

**v0.1.0: GPL-3.0-or-later combined application.** Original component licenses
and notices remain. Read [LICENSE](LICENSE), [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md)
and the [replacement audit](docs/licensing/ADAPTER_REPLACEMENT.md).
Publication still requires the project owner's approval; nothing has been pushed.

Video launching the game is aviable:
https://youtu.be/eyfm4SWIQ5M?si=18avMHVJrybn2TBp

## Features

* Native DoomGeneric in the CADCraft process; no external Doom executable/window.
* CAD menus/panels stay visible; only the central drawing canvas changes.
* DOOMEXIT restores the drawing without changing document contents or undo/redo.
* Worker-thread framebuffer copies uploaded to an egui texture.
* User-supplied IWAD, keyboard controls and key release on focus loss.

## Windows quick start

Windows 11 x64 and a DirectX 12-capable adapter/driver are required. If Windows
reports missing VCRUNTIME140.dll, install Microsoft's official Visual C++ v14
x64 Redistributable. Rust/C++ build tools are not required to run a built EXE.

1. Extract the Windows package into a writable ASCII-only path, such as
   `C:\Games\CADCraft-DOOM`.
2. Double-click **Launch-CADCraft-DOOM.bat**. It starts the adjacent compiled
   **CADCraft-DOOM.exe**, without compiling or starting an external Doom process.
3. Select your own legally obtained IWAD. Cancel to use CADCraft alone.
4. Open a drawing (the launcher opens a synthetic sample). Type **DOOM**, Enter
   in the CADCraft command line. Click the game for keyboard focus.
5. Use Enter to select New Game, episode and skill.

Original shareware **DOOM1.WAD** is the verified game data. Place your local copy
beside the EXE, choose it in the launcher, set `CADCRAFT_DOOM_WAD` to its full
path, or pass `-WadPath` to the PowerShell launcher. No WAD is distributed,
downloaded or substituted. Other compatible IWADs may be selected explicitly;
only original shareware has been verified. Use trusted game data.
Without a WAD, CADCraft starts normally; DOOM reports a helpful error.

## Controls

| Input | Action |
|---|---|
| Up/Down or W/S | Move forward/backward |
| Left/Right or A/D | Turn |
| Ctrl | Fire |
| Space | Use/open |
| Shift | Run |
| 1-7 | Weapon selection |
| Enter | Confirm/menu selection |
| Escape | Doom internal menu |
| F12 / Exit DOOM button | Return to CADCraft |
| DOOMEXIT in the CAD Command field | Return to CADCraft |

Click another panel to release game focus. Focus loss releases held keys.
**One native game session per process:** after DOOMEXIT, close CADCraft and
launch again before starting another game. Save CAD work before closing.
Ordinary CAD interaction remains available after DOOMEXIT.

## Build and limitations

See [BUILDING.md](BUILDING.md). Cargo.lock pins checksums/git revisions.
DoomGeneric is vendored at `e257a09527062bfb986b6e5e0780839b6cbd5b71`.
Windows x64 MSVC only for the game; single player; sound disabled; keyboard first;
no multiplayer/mouse look; no engine reinitialization in one process. The 320x200
framebuffer scales with preserved aspect ratio. IWAD structural checks are not
a security sandbox for hostile maps/textures. All maps, other platforms and the
full upstream CAD suite are not claimed tested. Matching source and dependency
source must accompany binary distribution.

## Media placeholders

* Screenshot: TODO - reviewed image of Doom inside the full CADCraft UI.
* Demonstration video: TODO - public URL after publication approval.

These are placeholders. Private evidence is not automatically included; game
imagery has rights separate from the engine source license.

## Credits

Original CADCraft: ArtCraft Team and CADCraft contributors, MIT OR Apache-2.0;
base `41d9b5e957b0719c04b9687c4f29088d77153c31`.
Doom engine: id Software; Chocolate Doom contributions include Simon Howard;
DoomGeneric contributors provide its portable abstraction. GnuPG-derived SHA-1
credits the Free Software Foundation. Preserve all headers, NOTICE,
ATTRIBUTION.md and THIRD_PARTY_LICENSES.md. Game data is not included.

The About/Windows version metadata retains CADCraft base version 0.12.0;
v0.1.0 is the integration release version. Current executed checks are in
[docs/ADAPTER_VERIFICATION.md](docs/ADAPTER_VERIFICATION.md).
