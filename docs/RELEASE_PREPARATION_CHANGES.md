# Release preparation changes (2026-10-26)

Every source-tree path changed or added during release preparation is listed
in RELEASE_PREPARATION_FILES.csv with its purpose. Pre-existing gameplay work
was preserved; it is not described as new work in this preparation pass.

* Audited/hash-compared all 203 DoomGeneric files. Original engine C sources and
  license headers remain unchanged in this pass; prior i_system.c host-exit
  adaptation remains. Native source list, worker, input and framebuffer integration
  were preserved. crates/doom/build.rs was formatted by rustfmt.
* Added truthful LICENSE map, third-party/license texts and 302-package Windows /
  484-package full dependency inventories. No unsupported GPL grant assigned.
* Removed eight restricted docs/brand PNG/SVG marks from the current tree. Private
  backups remain outside the source tree; LICENSE-brand.txt and developer credits
  remain. About/menu copy no longer presents this as an official upstream app.
  The separately MIT/Apache-attributed CADCraft drafting icon is retained.
* Replaced outdated delivery README text with English fork/build/control/WAD
  documentation, media placeholders, release gates and genuine-fork plan.
* Added pinned Rust toolchain, native build helper, portable launchers and tested
  allowlist packaging. Missing WAD/cancel permits CAD-only startup. No automatic
  game-data substitution is performed.
* Archived all seven upstream workflows under docs/upstream-workflows/*.disabled
  so this fork cannot accidentally run their original publishing/deployment jobs.
* Added .gitignore rules and package guards for WADs, saves, runtime profiles,
  tokens, logs, generated binaries/archives and Python caches.
* Prepared clean release-profile source/target directories and WAD-free local
  candidates, matching source/dependency bundle, manifests and checksums.

Outside the source checkout: the task-root .gitignore also excludes outputs WADs
and run-data; private work/verification/build material and blocked candidates are
local only. The original outputs/DOOM1.WAD and outputs/run-data/DOOM1.WAD were
retained, not deleted. The previous working EXE/launchers were not overwritten.

No commit, tag, repository creation, push, upstream message or publication was
performed. See RELEASE_VERIFICATION.md for tests actually executed and open gates.
