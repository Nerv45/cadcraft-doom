# Retained DoomGeneric engine and CADCraft-owned host

Upstream: https://github.com/ozkl/doomgeneric
Pinned revision: e257a09527062bfb986b6e5e0780839b6cbd5b71

The source directory now contains only the 169 files in the conservative
include/source closure of the selected Windows engine: 168 explicitly
GPL-2.0-or-later files and the explicitly GPL-3.0-or-later sha1.c.
Original copyright/license headers remain intact. The combined build selects
GPL-3.0-or-later; it does not revoke original component grants.

On 2026-10-26 all 26 unheaded upstream files were removed, including
config.h, doomgeneric.h, doomgeneric.c and dummy.c. Eight additional unused,
explicitly licensed files and three unneeded upstream readmes were also removed.
Private pre-change copies remain outside the repository and release artifacts.
The removal list/hashes are in docs/licensing/adapter-removal-record.json.
No upstream platform main function, window backend or old bootstrap is linked.

New host files, GPL-3.0-or-later:
- native/cadcraft_platform.h: typed callback/framebuffer contract for this host;
- native/cadcraft_config.h: minimal Windows single-player build choices;
- native/engine_host.c: host-owned static RGB32 surface and engine startup.

The host uses the documented DG callback interface and declarations/definitions
from retained GPL engine sources (m_argv.h, net_client.h and d_main.c).
It calls the retained argument processor and D_DoomMain directly, does not
implement doomgeneric_Create, and does not allocate a buffer through the removed
bootstrap. It supplies the single-player globals required by net_client.h.
The unused Timidity stub was not reproduced. No sound backend is linked.

Original ambiguous sources had been inspected during the prior licence audit.
This is newly authored host code, not a certified clean-room implementation.
Functional API names/types and necessary engine entry points are retained;
no blanket permission for the removed upstream files is asserted.

Retained-source modifications:
- i_system.c: existing host-return adaptation for I_Error/I_Quit (2026-10-24).
- Includes of old config.h/doomgeneric.h now use host-owned named headers.
- i_video.c uses the host frame dimensions instead of the old header's macros.
All modified retained files carry a separate change note and preserve upstream
notices. native/embed_redirect.h still routes engine exit calls into the C guard.
Rust worker lifecycle, key transition handling, locked RGBA mailbox, egui texture
rendering and document isolation remain as before; one session per process.

Audit: python scripts/audit_doom_vendor.py
See docs/licensing/ADAPTER_REPLACEMENT.md for scope, evidence and limitations.
