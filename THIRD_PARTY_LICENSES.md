# CADCraft DOOM third-party licenses

The combined application is distributed under **GPL-3.0-or-later**. Individual
components retain the grants below. This is an unofficial community modification,
not endorsed by ArtCraft Team, CADCraft upstream, DoomGeneric or id Software.

## CADCraft and integration

Original CADCraft 0.12.0, base commit
`41d9b5e957b0719c04b9687c4f29088d77153c31`:
https://github.com/storytold/cadcraft . Copyright (c) 2026 ArtCraft Team and
CADCraft contributors. MIT OR Apache-2.0; preserve LICENSE-MIT, LICENSE-APACHE,
NOTICE, ATTRIBUTION.md and original per-file notices. Integration files with
GPL-2.0-or-later headers keep that grant; the combined doom crate's metadata and
new native host files specify GPL-3.0-or-later.

## Retained Doom engine and replacement host

Source pin: https://github.com/ozkl/doomgeneric/tree/e257a09527062bfb986b6e5e0780839b6cbd5b71

The release retains 169 engine files: **168 explicitly GPL-2.0-or-later and
sha1.c explicitly GPL-3.0-or-later**. 78 C translation units are selected.
Original headers, authors and upstream GPLv2 license text remain intact.
The 12 modified retained files identify their host-integration changes.
Copyright remains with id Software, Simon Howard, Free Software Foundation
and the other authors named in those headers.

All 26 files without explicit per-file version grants and eight unused licensed
files have been removed. The ambiguous upstream config.h, doomgeneric.h,
doomgeneric.c and dummy.c are neither shipped nor compiled. The replacement
native/cadcraft_config.h, cadcraft_platform.h and engine_host.c implement the
minimal host interface and startup using retained, explicitly licensed engine
declarations. They carry a new GPL-3.0-or-later grant; this does not relicense
removed upstream files. Earlier versions of those files were visible during
investigation: this is not a certified clean-room development claim.

See docs/licensing/ADAPTER_REPLACEMENT.md, adapter-removal-record.json and
current doomgeneric-files.csv for the exact audit and hashes. The earlier
203-file inventory is retained as historical metadata, not removed source.
The former version-grant blocker no longer applies to the selected sources.
No additional upstream permission was obtained or assumed.

GPL-2.0-or-later grants permit choosing GPLv3+, satisfying compiled sha1.c.
Apache-2.0 dependencies are compatible with GPLv3, not GPLv2-only:
https://www.apache.org/licenses/GPL-compatibility.html . Original permissive
notices remain. GPL-3.0-or-later is the selected license for the combined work.

## Rust dependencies and bundled fonts

The generated `docs/licensing/cargo-dependencies.csv` lists exact versions,
source identifiers and declared licenses from the locked Windows metadata
resolution, including workspace/test/build packages. It is a conservative
inventory, not a claim that every package is in the executable.
Full copied dependency license/copyright/notice files are in
`licenses/dependencies/<name-version>/`. Cargo.lock pins crates.io checksums
and git commit identifiers. Use the matching source archive and vendored
Cargo dependencies when reproducing a release.

Notable components:

| Component | Version | License |
|---|---|---|
| egui, eframe, egui-wgpu | 0.36.2 | MIT OR Apache-2.0 |
| wgpu | 30.0.1 | MIT OR Apache-2.0 |
| winit | 0.30.13 | Apache-2.0 |
| acadrust | 0.6.5 | MPL-2.0 |
| colored (tooling graph) | 2.2.0 | MPL-2.0 |
| rfd | 0.17.2 | MIT |
| hidapi (Windows-native backend) | 2.6.7 | MIT |
| self_cell | 1.3.0 | Apache-2.0 OR GPL-2.0-only; select Apache-2.0 |
| cc (build tool) | 1.6.0 | MIT OR Apache-2.0 |
| epaint_default_fonts | 0.36.2 | (MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0 |

For dual MIT/Apache components the MIT option is available; retain supplied
notices regardless. Retain required Unicode, BSD, ISC, Zlib, Boost and font
notices too. egui's bundled default fonts are separate assets: their OFL/UFL
licenses remain applicable even without the optional craft-fonts input.
This build does not enable CRAFT_FONTS_DIR.

## MPL-2.0 combination

acadrust is used unmodified as a separate DWG library. Preserve its MPL license
and source, including any future changes. The inspected acadrust and colored
source trees have no Exhibit B incompatibility marking in source files.
The generic Exhibit B text in a license copy is not itself such a marking.
For acadrust in this GPL larger work, MPL section 3.3 is exercised: its covered
source is additionally available under GPL-3.0-or-later, while MPL availability
and notices remain. Tooling-only colored stays MPL-2.0 in the source bundle. See licenses/MPL-SECONDARY-NOTICE.md. Do not simply replace the MPL headers.
Providing the exact source alongside the executable also makes source access
clear to recipients. [Mozilla FAQ Q8/Q14/Q25](https://www.mozilla.org/en-US/MPL/2.0/FAQ/).

## Branding and game data

Restricted ArtCraft logos/marks were removed from this modified source tree.
The original `docs/brand/LICENSE-brand.txt` is retained. Copyright attribution
is factual credit, not endorsement. The existing CADCraft app/drafting icon
is separately attributed MIT OR Apache-2.0 in ATTRIBUTION.md.

No WAD belongs in public source or binary archives, including DOOM1.WAD,
run-data copies, savegames or privately downloaded FreeDoom. Doom's engine
GPL does not license id Software's game data. Users supply a compatible IWAD
obtained under its own terms. The private local originals were not deleted.

## Binary/source distribution

Distribute the whole linked executable under the selected
compatible GPL terms, with all applicable component notices and the exact
Corresponding Source: integration, modified engine, CADCraft, dependency source,
Cargo.lock and build/packaging scripts. A link to an arbitrary upstream HEAD
is insufficient. Publish a matching source archive with every binary asset.
Do not add restrictions contradicting recipients' GPL/MPL rights. System
libraries and build tools are documented in BUILDING.md, not bundled here.

The complete offline dependency bundle also contains non-Windows packages.
All 484 vendored Cargo package versions/licenses are listed separately in
`docs/licensing/cargo-vendored-packages.csv`. The Windows metadata inventory
contains 302 packages including local workspace/test/build packages. Neither
inventory replaces review of per-file notices in the corresponding source.

