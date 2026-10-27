"""Regression checks for the pinned engine licensing and removal boundary."""
from pathlib import Path
import csv
import tempfile
import unittest
import shutil
from audit_doom_vendor import audit, notice_license

class VendorAuditTests(unittest.TestCase):
    def test_selected_engine_and_host_are_reviewed(self):
        report = audit(Path(__file__).resolve().parents[1])
        self.assertEqual(report['unheaded_vendor_files'], 0)
        self.assertEqual(report['licences'], {'GPL-2.0-or-later': 168, 'GPL-3.0-or-later': 1})

    def test_unheaded_file_never_gets_an_invented_grant(self):
        self.assertIsNone(notice_license('void example(void) {}'))
        self.assertIsNone(notice_license('GNU GENERAL PUBLIC LICENSE Version 2, June 1991'))

    def test_removed_interface_reintroduction_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root/'crates/doom/vendor/doomgeneric').mkdir(parents=True)
            (root/'docs/licensing').mkdir(parents=True)
            (root/'crates/doom/vendor/doomgeneric/config.h').write_text('/* old interface */')
            (root/'docs/licensing/doomgeneric-files.csv').write_text('file,classification,sha256\nconfig.h,GPL-2.0-or-later,invalid\n')
            with self.assertRaisesRegex(ValueError, 'reintroduced'):
                audit(root)

if __name__ == '__main__':
    unittest.main()
