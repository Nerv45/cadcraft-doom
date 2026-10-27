# Draft upstream license clarification - NOT SENT

For ozkl/doomgeneric at e257a09527062bfb986b6e5e0780839b6cbd5b71:

The root LICENSE is GPLv2. Most source headers grant GPL-2.0-or-later,
but compiled sha1.c explicitly grants GPL-3.0-or-later. 26 files lack explicit
per-file grants; the complete list/hashes are in doomgeneric-files.csv.

Can the relevant rights holders confirm and document whether all contributions
in those files may be used under GPL-2.0-or-later or GPL-3.0-or-later? In particular,
please cover doomgeneric.c, dummy.c and their unheaded headers, and identify any
contributions for which the maintainer cannot grant or confirm these permissions.
A root-license badge alone does not resolve this for our statically linked GPLv3
larger work with Apache-2.0 dependencies.

No message/issue was posted. Maintainer authority over every contribution must
be established; an unsupported assertion does not clear the release blocker.
If rights holders confirm GPLv2-only, this combined executable cannot simply be
relabeled GPLv3. Any replacement/rework must be separately reviewed and retested.

## History inspected locally

Full DoomGeneric history was fetched read-only in the private work checkout.
LICENSE was introduced by 0b085de3725d60d24ed4dac4dfed68a3aca86140
(Create LICENSE, 2019-08-18), containing the standard GPLv2 text. README/license
history did not establish a separate explicit or-later grant for all unheaded
contributions. The old 1997 non-profit README is retained only as historical
provenance, not applied as a new restriction on explicitly GPL-licensed files.
DoomGeneric Git history is not included in source bundles, which contain only
the reviewed pinned source files and notices, without historical binary/game data.
