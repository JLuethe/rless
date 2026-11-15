"""Source-export controls for evidence inside the pinned non-Git containers."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from common import ROOT, git, fingerprint
import sys
sys.path.insert(0,str(ROOT/'tools'))
from release import file_identity


class ArchiveIdentity(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.root=Path(self.temp.name)
        (self.root/'src').mkdir()
        (self.root/'Cargo.toml').write_text('[package]\nname="fixture"\nversion="0.1.0"\n')
        (self.root/'src/lib.rs').write_text('pub static VALUE: usize = 1;\n')
        files={name:file_identity(self.root/name) for name in ('Cargo.toml','src/lib.rs')}
        marker=dict(schema_version=1,source_kind='explicit-working-tree-snapshot',base_commit='a'*40,
                    dirty_state=' M tools/safety/fixture.py\n',files=files,
                    files_sha256=hashlib.sha256(json.dumps(files,sort_keys=True).encode()).hexdigest())
        (self.root/'.rless-working-tree.json').write_text(json.dumps(marker))

    def tearDown(self):self.temp.cleanup()

    def test_export_queries_rederive_exact_source_identity(self):
        self.assertEqual(git('rev-parse','HEAD',root=self.root).strip(),b'a'*40)
        self.assertEqual(git('status','--porcelain',root=self.root),b' M tools/safety/fixture.py\n')
        self.assertEqual(set(fingerprint(self.root)['files']),{'Cargo.toml','src/lib.rs'})

    def test_export_changes_and_extra_inputs_reject(self):
        (self.root/'src/lib.rs').write_text('pub static VALUE: usize = 2;\n')
        with self.assertRaises(ValueError):fingerprint(self.root)
        (self.root/'src/lib.rs').write_text('pub static VALUE: usize = 1;\n')
        (self.root/'src/extra.rs').write_text('pub fn extra() {}\n')
        with self.assertRaises(ValueError):fingerprint(self.root)

    def test_archive_cannot_authorize_mutating_git_commands(self):
        with self.assertRaisesRegex(ValueError,'does not support'):git('add','.',root=self.root)


if __name__=='__main__':unittest.main()
