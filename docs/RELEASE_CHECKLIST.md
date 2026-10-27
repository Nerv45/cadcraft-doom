# v0.1.0 publication plan

**Publication awaits project owner approval. Nothing has been pushed or published.**

The former DoomGeneric version-grant blocker was removed by excluding all
26 unheaded files and replacing the four participating host files. Current
scope and license: docs/licensing/ADAPTER_REPLACEMENT.md. Combined program:
GPL-3.0-or-later, with original component notices, MPL secondary notice and
complete matching source. New artifacts include the corrected font notices.
Old BLOCKED-CANDIDATE archives remain historical and must not be published.

## Upstream verification

Original repository: https://github.com/storytold/cadcraft . Local origin still
points there. HEAD/base is 41d9b5e957b0719c04b9687c4f29088d77153c31, a shallow
boundary with one available commit. No independent root was created or pushed.
DoomGeneric is pinned to e257a09527062bfb986b6e5e0780839b6cbd5b71.

Proposed repository name: **cadcraft-doom**. Recommend a genuine upstream GitHub
fork, renamed if necessary, explicitly described as an unofficial community
modification. Do not upload the whole task directory or initialize a new history.

## Publication sequence - not executed

1. Review the replacement audit, dependency/font notices, matching source bundle
   and current debranded tree. Preserve applicable brand terms without historical
   artwork in release assets.
2. Approve owner, genuine-fork plan and final license statement.
3. Fetch full ancestry locally (`git fetch --unshallow origin`); inspect changes.
4. Only after approval, create the GitHub fork. Configure upstream to the original
   and origin to the approved user's fork. Never push to the original repository.
5. Commit reviewed changes with meaningful licensing/provenance, community-branding
   and WAD-free packaging/docs messages. Exclude WAD, logs, profiles, private
   evidence, control tokens, binaries and local paths.
6. Rebuild/test the final commit with an empty target directory; inspect imports
   and verify the extracted package. Include exact source and dependency source.
7. After approval, tag v0.1.0 and prepare a DRAFT Release containing Windows ZIP,
   matching complete source ZIP, SHA256SUMS and honest test results/limitations.
   Publish only after a separate final approval. Automatic upstream publishing
   workflows were moved to docs/upstream-workflows/*.disabled.

Add reviewed screenshot/video links to README.md after approval. Private game
imagery is not automatically redistributed. Preserve the per-file modification
and verification records. Do not publish a blocked local candidate.
