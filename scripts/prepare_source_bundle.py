"""Create a WAD-free complete source candidate with exact Cargo dependency sources."""
from pathlib import Path
import argparse,hashlib,json,shutil
from export_source import export
from package_release import archive
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--vendor',type=Path,required=True);p.add_argument('--config',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--prepare-only',action='store_true');p.add_argument('--candidate',action='store_true');a=p.parse_args()
    root=Path(__file__).resolve().parents[1]
    status=json.loads((root/'docs/licensing/release-status.json').read_text())
    if not a.candidate and not status['public_release_allowed']:
        p.exit(2, 'PUBLIC RELEASE BLOCKED: '+ '; '.join(status['blockers'])+'\n')
    dest=a.output/('CADCraft-DOOM-v0.1.0-corresponding-source'+('-BLOCKED-CANDIDATE' if a.candidate else ''))
    export(root,dest)
    shutil.copytree(a.vendor,dest/'vendor-deps')
    config=a.config.read_text(encoding='utf-8-sig')
    import re
    config=re.sub(r'directory = ".*"','directory = "vendor-deps"',config)
    with (dest/'.cargo/config.toml').open('a',encoding='utf-8') as out:out.write('\n'+config)
    if a.candidate: (dest/'RELEASE-BLOCKED.txt').write_text('LOCAL REVIEW ONLY - DO NOT DISTRIBUTE\n'+'\n'.join(status['blockers'])+'\n',encoding='utf-8')
    hashes={p.relative_to(dest).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(dest.rglob('*')) if p.is_file() and p.name!='SOURCE-MANIFEST.json'}
    (dest/'SOURCE-MANIFEST.json').write_text(json.dumps(hashes,indent=2)+'\n',encoding='utf-8')
    if not a.prepare_only: archive(dest,a.output/(dest.name+'.zip'))
    print('Complete corresponding source:',dest.resolve())
    print('Manifest files:',len(hashes),'vendored dependency directories:',len(list((dest/'vendor-deps').iterdir())))
