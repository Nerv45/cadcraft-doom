# Host replacement verification - 2026-10-26

This report supersedes the old blocked-candidate results for the host replacement.
No public repository, push, tag or GitHub Release was created.

| Check actually executed | Result |
|---|---|
| Windows x64 MSVC release build from refreshed source export | PASS, exit 0, 5m54s |
| cargo fmt --all -- --check | PASS |
| Clippy selected application/DOOM/UI crates, -D warnings | PASS, exit 0 |
| Native DOOM tests: 2 unit + 1 lifecycle/error integration | PASS |
| DOOM UI unit tests | PASS, 2 tests |
| Real engine opt-in playable_viewport_exit_and_cad_resume | PASS, 14.26s |
| Python packaging/audit tests | PASS, 8 tests |
| Selected vendor inventory/hash/grant/include audit | PASS:169 files,168 GPL2+,1 GPL3+,78 C units |
| EXE PE headers/imports/version resources | PASS: AMD64 GUI, current GPL3+ metadata |
| Windows ZIP full CRC/no-WAD check | PASS |
| Extracted EXE desktop starts and displays sample drawing | PASS |
| Extracted package normal closure and relaunch | PASS |
| Launcher validation without WAD and with original DOOM1.WAD | PASS |
| Launcher malformed IWAD rejection | PASS: clear error, exit 1, no app launch |
| New EXE desktop Doom gameplay through physical keyboard | NOT VERIFIED in this run |

The real-engine test uses the actual CADCraft egui shell and production raw-input
hook, original local shareware DOOM1.WAD (SHA256
1d7d43be501e67d927e415e0b8f3e29c3bf33075e859721816f652a526cac771), and the new
native host. It types DOOM, starts New Game, renders E1M1, exercises W movement,
turning, Ctrl fire and internal menu, then types DOOMEXIT. It checks document Arc
identity, complete document value, revision, view and undo/redo lengths remain
unchanged, and creates/undoes a CAD circle afterward. Moving changed 422701 game
pixels. Private screenshots were visually reviewed; the game is within the full
CAD UI. Multiple game windows/external Doom executables are not involved.

Native tests cover malformed and missing IWADs and worker/lifecycle safeguards;
UI tests cover focus loss and key release. All maps, sustained native desktop key
holding, all controls/maps and a clean-machine installation were not tested.

The final EXE desktop starts normally without a WAD; keyboard injection was
limited by the current Cyrillic keyboard layout, and sky.type_text did not insert
Latin text into the custom command field. Attempts to use the built-in loopback
control channel were blocked by the execution environment (WinError10013,
connection timeout and EACCES). Therefore the new EXE's desktop DOOM command,
missing-WAD error display and interactive game were not marked PASS. Those code
paths passed the separate native/UI tests described above. Previous release
manual gameplay results do not substitute for current EXE verification.

The first combined Python run failed because its default sandbox temporary
folder denied creating fixture subdirectories. Re-running all eight with TEMP
and TMP set to the task's writable work/test-temp succeeded. No product code
change was needed. Clippy/build emitted filesystem hard-link fallback warnings
in the sandbox; no source diagnostics or compile errors remained.

The replacement was rebuilt incrementally using the pinned existing dependency
cache; affected C/Rust crates were recompiled. This was not a second completely
empty-cache rebuild. Cargo.lock and dependency versions are unchanged. The
source ZIP includes all 484 vendored dependency sources and their notices.
Runtime imports require Windows system DLLs and Microsoft's VCRUNTIME140.dll;
no Doom/SDL DLL or external Doom process is required. Sound remains disabled.
Only one native session per CADCraft process is supported; restart the app after
DOOMEXIT to begin another game.

EXE SHA256: 8547c61f717c2b3b13cf7ebf1fc82a88a2fd76c6536a03f6f3e16a503bfd75aa
EXE bytes: 55175680

Private evidence/logs are retained outside public source in work/replacement-*,
work/release-pe-* and work/release-version-info.log. Local WADs are unchanged.

The working Rust input/rendering implementation, FFI lifecycle facade, viewport
test and Cargo.lock match their pre-replacement SHA256 hashes. All compiled
C/Rust/config inputs match the release build source snapshot. The changed paths
versus the previous release snapshot are listed in ADAPTER_CHANGED_FILES.csv.
Both private WAD copies retained their original SHA256. The fire screenshot
shows ammunition decreasing from 50 to 49 after production Ctrl input.
