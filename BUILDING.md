# Windows x64 build and reproduction

Public distribution is blocked: read LICENSE and docs/RELEASE_CHECKLIST.md.

## Prerequisites

* Windows 11 x64, DirectX 12-capable adapter/driver.
* Rust 1.99.1 x86_64-pc-windows-msvc, rustfmt and clippy (pinned in rust-toolchain.toml).
  egui 0.36 requires at least Rust 1.95.
* Visual Studio 2026 Community or Build Tools: Desktop development with C++,
  MSVC x64 compiler/librarian and Windows SDK including rc.exe.
  Candidate environment: MSVC 14.51.36231.
* Git, Windows PowerShell 5.1+, Python 3.11+ for audit/packaging scripts.
* Network for initial locked Cargo fetch, or the dependency-vendored source bundle.

SDL, bindgen and an external Doom executable are not needed. C is built by cc
and lib.exe, then statically linked into Rust. CRAFT_FONTS_DIR is unset; egui
still embeds its own default fonts, whose OFL/UFL notices are supplied.

## Build

In an x64 Developer PowerShell, from the source root:

```powershell
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup target add x86_64-pc-windows-msvc --toolchain 1.99.1
$env:CADCRAFT_REQUIRE_WINRES = '1'
Remove-Item Env:CRAFT_FONTS_DIR -ErrorAction SilentlyContinue
cargo build --locked --release --target x86_64-pc-windows-msvc -p cadcraft --features doom
```

Or run packaging/windows-doom/Build-Windows.ps1, which locates the C++ toolchain
with vswhere and initializes its x64 shell. Default EXE:
`target/x86_64-pc-windows-msvc/release/cadcraft.exe`. CARGO_TARGET_DIR overrides
its target root. The helper copies the EXE and launchers to an ignored local
candidate folder, without WAD files. Read README.md for launch instructions.

The initial verification used a clean source export and an empty target directory.
The host replacement was rebuilt from a refreshed source export using the same
pinned dependency cache; affected native and Rust crates were recompiled. Logs
and source manifests record inputs. To perform your own entirely fresh build,
select a new empty CARGO_TARGET_DIR.
Pinned-input reproducibility is intended; byte-for-byte identical EXEs are not claimed.

## Dependency source bundle

Cargo.lock pins exact registry checksums and git revisions. The corresponding
source bundle includes vendor-deps from `cargo vendor --locked --versioned-dirs`
and its generated Cargo source-replacement config. With those present,
`cargo build --offline --locked ...` uses the included dependency sources.
Keep all .cargo-checksum.json and notice files. Compiler/MSVC/SDK are separate
prerequisites. Do not use the replacement config without the matching vendor tree.

## Tests

```powershell
cargo fmt --all -- --check
cargo test --locked -p cadcraft-doom
cargo test --locked -p cadcraft-ui-egui --features doom --lib doom::
$env:CADCRAFT_DOOM_WAD = 'C:\Games\Doom\DOOM1.WAD'
$env:CADCRAFT_DOOM_TEST_OUTPUT = 'C:\Temp\cadcraft-doom-evidence'
cargo test --locked -p cadcraft-ui-egui --features doom --test doom_viewport -- --ignored --nocapture
cargo clippy --locked -p cadcraft-doom -p cadcraft-ui-egui -p cadcraft --features cadcraft/doom -- -D warnings
```

The opt-in test renders the complete CAD UI, exercises production raw-input
handling, compares drawing/view/revision/undo/redo after DOOMEXIT, then types a
CAD CIRCLE and undoes it. It consumes one engine session in its process.
It complements desktop testing; it is not a full playthrough.

## Runtime and packaging

Inspect the actual EXE with `dumpbin /headers` and `/dependents`; do not assume
its import set. Windows system DLLs and Microsoft's Visual C++ v14 x64 runtime
are required. No Microsoft runtime DLL is copied into the package. Obtain the
Redistributable from https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist .

Run `python scripts/package_release.py --exe <built-exe> --output <new-directory>`.
It creates an allowlisted, WAD-free ZIP and matching manifest. Packaging is refused
while licensing blockers remain; --candidate is only for explicitly blocked
local review artifacts. The complete source bundle also includes dependency
source. Do not publish blocked candidates. See docs/RELEASE_CHECKLIST.md.

Audit retained engine and packaging: `python scripts/audit_doom_vendor.py`,
`python -m unittest discover -s scripts -p "test_*.py"`.
Prepare matching complete source with `python scripts/prepare_source_bundle.py
--vendor <cargo-vendor-directory> --config <cargo-vendor-config> --output <new-directory>`.
