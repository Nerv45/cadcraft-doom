"""Export the current source tree without Git metadata, WADs or private runtime data."""
from pathlib import Path
import argparse, hashlib, json, shutil, subprocess

def export(root, dest):
    root=root.resolve();dest=dest.resolve()
    if dest.exists(): raise ValueError(f'Refusing to overwrite source export: {dest}')
    if dest.is_relative_to(root): raise ValueError('Export must be outside the source checkout')
    names=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','-z'],cwd=root).decode().split('\0')
    dest.mkdir(parents=True)
    hashes={}
    for name in sorted(set(names)):
        if not name: continue
        src=root/name;rel=Path(name)
        if not src.is_file(): continue
        if src.is_symlink(): raise ValueError(f'Review symlink before export: {name}')
        if rel.is_absolute() or '..' in rel.parts: raise ValueError(name)
        if rel.suffix.lower() in ('.wad','.exe','.pdb','.log','.token','.dsg','.lmp','.zip'): raise ValueError(f'Private/binary file in source selection: {name}')
        if any(p in ('.git','run-data','profile','vendor-deps','release-artifacts') for p in rel.parts): raise ValueError(name)
        out=dest/rel;out.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,out)
        hashes[rel.as_posix()]=hashlib.sha256(out.read_bytes()).hexdigest()
    (dest/'SOURCE-MANIFEST.json').write_text(json.dumps(hashes,indent=2)+'\n',encoding='utf-8')
    return hashes

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('destination',type=Path);a=p.parse_args()
    result=export(Path(__file__).resolve().parents[1],a.destination)
    print(f'Exported {len(result)} source files to {a.destination}; no Git history or game data copied')
