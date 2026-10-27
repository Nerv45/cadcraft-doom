"""Allowlisted Windows package builder. Public releases are gated on licensing."""
from pathlib import Path
import argparse, hashlib, json, shutil, zipfile

def verify_files(folder):
    for p in folder.rglob('*'):
        if p.is_file() and (p.suffix.lower() in ('.wad','.dsg','.lmp','.token','.log') or any(x in ('run-data','profile','.git') for x in p.relative_to(folder).parts)):
            raise ValueError(f'Forbidden release file: {p}')

def archive(folder, dest):
    verify_files(folder)
    if dest.exists(): raise ValueError(f'Refusing to overwrite archive: {dest}')
    partial=dest.with_suffix(dest.suffix+'.partial')
    created=False
    try:
        with zipfile.ZipFile(partial,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=6,strict_timestamps=False) as z:
            created=True
            for p in sorted(folder.rglob('*')):
                if p.is_file(): z.write(p,folder.name+'/'+p.relative_to(folder).as_posix())
        with zipfile.ZipFile(partial) as z:
            bad=z.testzip()
            if bad: raise ValueError(f'Corrupt ZIP member: {bad}')
            if any(Path(n).suffix.lower()=='.wad' for n in z.namelist()): raise ValueError('WAD in archive')
        partial.rename(dest)
    except BaseException:
        if created and partial.exists(): partial.unlink()
        raise

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--candidate',action='store_true');p.add_argument('--exe',required=True,type=Path);p.add_argument('--output',required=True,type=Path);a=p.parse_args()
    root=Path(__file__).resolve().parents[1]
    status=json.loads((root/'docs/licensing/release-status.json').read_text())
    if not a.candidate and not status['public_release_allowed']:
        p.exit(2,'PUBLIC RELEASE BLOCKED: '+ '; '.join(status['blockers'])+'\n')
    if a.output.exists(): p.exit(2,'Refusing to overwrite an existing package directory\n')
    if not a.exe.is_file(): p.exit(2,f'Executable missing: {a.exe}\n')
    stem='CADCraft-DOOM-v0.1.0-windows-x64'+('-BLOCKED-CANDIDATE' if a.candidate else '')
    out=a.output/stem;out.mkdir(parents=True)
    shutil.copy2(a.exe,out/'CADCraft-DOOM.exe')
    for name in ('Launch-CADCraft-DOOM.bat','Launch-CADCraft-DOOM.ps1'):
        shutil.copy2(root/'packaging/windows-doom'/name,out/name)
    for name in ('README.md','BUILDING.md','LICENSE','LICENSE-MIT','LICENSE-APACHE','NOTICE','ATTRIBUTION.md','THIRD_PARTY_LICENSES.md','Cargo.lock'):
        shutil.copy2(root/name,out/name)
    for name in ('licenses','docs/licensing'):
        shutil.copytree(root/name,out/name)
    shutil.copy2(root/'docs/RELEASE_CHECKLIST.md',out/'RELEASE_CHECKLIST.md')
    shutil.copy2(root/'docs/ADAPTER_VERIFICATION.md',out/'docs/ADAPTER_VERIFICATION.md')
    shutil.copy2(root/'crates/doom/vendor/LICENSE-DoomGeneric',out/'licenses/LICENSE-DoomGeneric')
    if a.candidate:
        (out/'RELEASE-BLOCKED.txt').write_text('LOCAL REVIEW ONLY - DO NOT DISTRIBUTE\n'+ '\n'.join(status['blockers'])+'\nThe matching source ZIP and dependency sources must accompany any eventual cleared binary.\n',encoding='utf-8')
    hashes={x.relative_to(out).as_posix():hashlib.sha256(x.read_bytes()).hexdigest() for x in sorted(out.rglob('*')) if x.is_file()}
    (out/'PACKAGE-MANIFEST.json').write_text(json.dumps(hashes,indent=2)+'\n',encoding='utf-8')
    archive(out,a.output/(stem+'.zip'))
    print(out.resolve())
    print('ZIP verified; WAD-free. Public distribution '+('BLOCKED' if not status['public_release_allowed'] else 'requires human approval'))
