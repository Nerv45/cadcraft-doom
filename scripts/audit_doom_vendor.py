"""Fail closed if removed/unlicensed engine files or stale source hashes return."""
from pathlib import Path
import csv
import hashlib
import json
import re

REMOVED_INTERFACES = {'config.h', 'doomgeneric.h', 'doomgeneric.c', 'dummy.c'}
# CRT header plus legacy headers in disabled ORIGCODE/platform branches.
EXPECTED_EXTERNAL_HEADERS = {'stdlib.h', 'SDL.h', 'SDL2/SDL.h', 'SDL2/SDL_cdrom.h', 'SDL_joystick.h', 'txt_main.h'}
ALLOWED_GRANTS = {'GPL-2.0-or-later', 'GPL-3.0-or-later'}


def notice_license(text):
    if re.search(r'either version\s+3', text[:6000]):
        return 'GPL-3.0-or-later'
    if re.search(r'either version\s+2', text[:6000]):
        return 'GPL-2.0-or-later'
    return None


def audit(root):
    crate = root / 'crates/doom'
    vendor = crate / 'vendor/doomgeneric'
    with (root / 'docs/licensing/doomgeneric-files.csv').open(encoding='utf-8') as manifest:
        rows = list(csv.DictReader(manifest))
    expected = {row['file']: row for row in rows}
    actual = {p.name: p for p in vendor.iterdir() if p.is_file()}
    if set(actual) != set(expected):
        raise ValueError(f'Vendor manifest mismatch: extra={set(actual)-set(expected)}, missing={set(expected)-set(actual)}')
    if REMOVED_INTERFACES.intersection(actual):
        raise ValueError('Removed upstream interface was reintroduced')
    counts = {}
    for name, file in actual.items():
        data = file.read_bytes()
        grant = notice_license(data.decode('utf-8'))
        if grant not in ALLOWED_GRANTS or grant != expected[name]['classification']:
            raise ValueError(f'Unverified licence notice: {name}')
        if hashlib.sha256(data).hexdigest() != expected[name]['sha256']:
            raise ValueError(f'Source changed without updating reviewed manifest: {name}')
        counts[grant] = counts.get(grant, 0) + 1
    units = (crate / 'native/sources.txt').read_text().split()
    if {'dummy', 'doomgeneric'}.intersection(units):
        raise ValueError('Removed bootstrap translation unit is selected')
    visited = set()
    todo = [vendor / (name + '.c') for name in units]
    todo += [crate / 'native/bridge.c', crate / 'native/engine_host.c', crate / 'native/embed_redirect.h']
    while todo:
        file = todo.pop()
        if file in visited:
            continue
        visited.add(file)
        text = file.read_text(encoding='utf-8-sig')
        for name in re.findall(r'^\s*#\s*include\s*"([^\"]+)"', text, re.M):
            if name in REMOVED_INTERFACES:
                raise ValueError(f'Old host interface included by {file.name}: {name}')
            choices = [file.parent / name, vendor / name, crate / 'native' / name]
            included = next((p for p in choices if p.is_file()), None)
            if included is None:
                if name not in EXPECTED_EXTERNAL_HEADERS:
                    raise ValueError(f'Missing local include: {file.name}: {name}')
                continue
            todo.append(included)
    extras = {p.name for p in (crate / 'vendor').iterdir() if p.name != 'doomgeneric'}
    if extras != {'LICENSE-DoomGeneric', 'PROVENANCE.md'}:
        raise ValueError(f'Unexpected upstream vendor documentation: {extras}')
    for name in ('cadcraft_platform.h', 'cadcraft_config.h', 'engine_host.c'):
        if 'SPDX-License-Identifier: GPL-3.0-or-later' not in (crate / 'native' / name).read_text(encoding='utf-8-sig')[:400]:
            raise ValueError(f'Missing host licence notice: {name}')
    return {'vendor_files': len(actual), 'licences': counts, 'engine_translation_units': len(units), 'local_closure_files': len(visited), 'unheaded_vendor_files': 0, 'removed_interfaces_present': False}


if __name__ == '__main__':
    print(json.dumps(audit(Path(__file__).resolve().parents[1]), indent=2))
