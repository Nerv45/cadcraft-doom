# Native host replacement and licensing review (2026-10-26)

## Scope and decision

The former blocker was four participating and 22 unused DoomGeneric files
without explicit version grants. All 26 have been removed, together with eight
unused licensed files and three redundant upstream README copies. No permission has been fabricated for them. Their hashes
and removal list are in adapter-removal-record.json; original source backups
remain private outside the release tree. No game WAD has been removed locally.

Selected vendor scope is 169 files, 78 C translation units: 168 files expressly
GPL-2.0-or-later, sha1.c expressly GPL-3.0-or-later. Current per-file hashes and
upstream hashes are in doomgeneric-files.csv. Native tests compile the actual
selected source list. scripts/audit_doom_vendor.py checks inventory hashes,
explicit grants, local include closure and rejects the removed interface files.
Legacy inactive SDL/system includes are explicitly recognized; SDL is not linked.

## New implementation

native/engine_host.c owns a static 320x200 RGBA-compatible pixel buffer, sets
licensed m_argv engine arguments, processes response files and calls D_DoomMain.
It defines the two disabled-network globals declared in retained net_client.h.
cadcraft_platform.h declares only the host contract needed by the engine;
cadcraft_config.h defines the four consumed package/path constants and rejects
unsupported sound/indexed-pixel modes. All three are GPL-3.0-or-later.
No dummy.c, doomgeneric.c, config.h or doomgeneric.h is shipped. Twelve retained
GPL files include the new contracts and identify their modifications; notices
remain. No Doom UI/gameplay Rust implementation was rewritten.

Existing bridge synchronization, key queue, worker, cooperative sleep and guarded
exit behavior remain. DG callbacks still copy frames under the mutex, and egui
still uploads owned bytes. There is no external executable/window or new thread
access to unguarded pixel memory. Restarting a native session still requires
restarting CADCraft. Gameplay/input semantics have not intentionally changed.

The implementation uses functional interfaces documented by the actual retained
GPL engine. The old upstream files were previously inspected. It would be
incorrect to call this a certified clean-room implementation or an independent
Doom engine; it is a newly written small host for the retained GPL Doom engine.

## Compatibility and obligations

GPL-3.0-or-later is selected for the linked program. GPL-2.0-or-later files permit
this choice; sha1.c already requires it. Apache-2.0 dependencies fit GPLv3
(https://www.apache.org/licenses/GPL-compatibility.html). MIT/BSD/ISC/Zlib/Boost
and Unicode notices remain. self_cell's Apache option is selected. No separate
upstream grant is required for files with an applicable existing license grant.

acadrust 0.6.5 and tooling-only colored 2.2.0 retain MPL-2.0 and source availability;
For linked acadrust, MPL section 3.3 permits additional GPL secondary distribution
because no attached Exhibit B incompatibility notice was found. Tooling-only
colored remains MPL-2.0 in the complete source bundle. See licenses/MPL-SECONDARY-NOTICE.md
and https://www.mozilla.org/en-US/MPL/2.0/ . Font assets retain OFL/UFL and their
other notices independently. Corrected Hack/emoji font notices must accompany
new artifacts; old blocked archives are not made publishable by this document.

The earlier dependency inventories cover 302 locked Windows metadata packages,
289 reachable normal/build packages conservatively, and 484 vendored packages
for the complete cross-platform source bundle. No dependency versions changed.
Retained component notices and source are included in matching release archives.
This engineering review establishes the stated source scope and selected grants;
it is not an assurance from a rights-holder or a legal opinion.

## Branding and publishing

Current artwork remains debranded; original credit and brand terms remain.
Git history can contain removed artwork: an ordinary upstream GitHub fork is
subject to GitHub platform terms, not a blanket off-platform artwork/trademark
license. Source archives exclude .git and restricted artwork. Do not redistribute
historical artwork in release assets. No WAD, saves, profiles or credentials may
be published. Use a genuine upstream fork, review staged changes and publish the
matching source with the binary only after the owner's final approval.

## Verification

See docs/ADAPTER_VERIFICATION.md for executed checks, hashes and limitations.
Historical pre-replacement reports and blocked archives are not current results.
