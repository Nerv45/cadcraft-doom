from pathlib import Path
import unittest,tempfile,zipfile,subprocess,sys
sys.path.insert(0,str(Path(__file__).resolve().parent))
from package_release import archive
class PackagingTests(unittest.TestCase):
    def test_wad_rejected_case_insensitively(self):
        with tempfile.TemporaryDirectory() as t:
            p=Path(t)/'package';p.mkdir();(p/'DOOM1.WaD').write_bytes(b'IWAD')
            with self.assertRaises(ValueError):archive(p,Path(t)/'bad.zip')
    def test_runtime_data_rejected(self):
        with tempfile.TemporaryDirectory() as t:
            p=Path(t)/'package';(p/'run-data').mkdir(parents=True);(p/'run-data/game.cfg').write_text('private')
            with self.assertRaises(ValueError):archive(p,Path(t)/'bad.zip')
    def test_allowlisted_zip_integrity(self):
        with tempfile.TemporaryDirectory() as t:
            p=Path(t)/'package';p.mkdir();(p/'README.md').write_text('test');z=Path(t)/'good.zip';archive(p,z)
            with zipfile.ZipFile(z) as f:self.assertEqual(f.namelist(),['package/README.md'])
    def test_pre_1980_dependency_timestamps(self):
        import os
        with tempfile.TemporaryDirectory() as t:
            p=Path(t)/'package';p.mkdir();f=p/'LICENSE';f.write_text('test notice');os.utime(f,(0,0))
            z=Path(t)/'old-time.zip';archive(p,z)
            with zipfile.ZipFile(z) as a:self.assertEqual(a.getinfo('package/LICENSE').date_time[0],1980)
    def test_public_packaging_blocked_before_output(self):
        with tempfile.TemporaryDirectory() as t:
            out=Path(t)/'not-created'
            import shutil,json
            root=Path(t)/'fixture';(root/'scripts').mkdir(parents=True);(root/'docs/licensing').mkdir(parents=True)
            script=root/'scripts/package_release.py';shutil.copy2(Path(__file__).with_name('package_release.py'),script)
            (root/'docs/licensing/release-status.json').write_text(json.dumps({'public_release_allowed':False,'blockers':['test blocker']}))
            result=subprocess.run([sys.executable,str(script),'--exe',str(Path(t)/'absent.exe'),'--output',str(out)],capture_output=True,text=True)
            self.assertEqual(result.returncode,2);self.assertIn('PUBLIC RELEASE BLOCKED',result.stderr);self.assertFalse(out.exists())
if __name__=='__main__':unittest.main()
