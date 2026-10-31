"""Snapshot and diagnostic mutation controls; real detector execution is separate."""
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from offline_symbolize import FRAME, require_source, symbolize
from snapshot import archive, checked_path, source_names, validate


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class SnapshotControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        (self.root / 'src').mkdir()
        (self.root / 'Cargo.toml').write_text('[package]\nname="fixture"\n')
        (self.root / 'src/main.rs').write_text('fn main() {}\n')
        self.value = dict(schema_version=1, source_kind='explicit-working-tree-snapshot',
            files={name: dict(sha256=digest(self.root / name), mode=0o644)
                   for name in ('Cargo.toml', 'src/main.rs')})
        self.value['files_sha256'] = hashlib.sha256(json.dumps(self.value['files'], sort_keys=True).encode()).hexdigest()

    def tearDown(self):
        self.temp.cleanup()

    def test_archive_is_exact_and_excludes_build_and_private_inputs(self):
        names = b'Cargo.toml\0src/main.rs\0tools/safety/core/target/debug/build\0.aws/credentials\0.env\0'
        with patch('snapshot.git', return_value=names):
            self.assertEqual(source_names(self.root), ['Cargo.toml', 'src/main.rs'])
        path = self.root / 'snapshot.tar'
        with patch('snapshot.source_names', return_value=['Cargo.toml', 'src/main.rs']):
            archive(self.value, path, self.root)
        with patch('snapshot.source_names', return_value=['Cargo.toml', 'src/main.rs', 'src/new.rs']):
            with self.assertRaisesRegex(ValueError, 'selection changed'):
                archive(self.value, self.root / 'stale.tar', self.root)
        with tarfile.open(path) as saved:
            self.assertEqual(saved.getnames(), ['Cargo.toml', 'src/main.rs'])
            self.assertEqual(saved.extractfile('src/main.rs').read(), b'fn main() {}\n')

    def test_changed_source_manifest_and_modes_fail(self):
        validate(self.value, self.root, exact=True)
        self.value['files_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'digest'):
            validate(self.value, self.root)
        self.value['files_sha256'] = hashlib.sha256(json.dumps(self.value['files'], sort_keys=True).encode()).hexdigest()
        (self.root / 'src/main.rs').chmod(0o755)
        with self.assertRaisesRegex(ValueError, 'mode'):
            validate(self.value, self.root)
        (self.root / 'src/main.rs').chmod(0o644)
        (self.root / 'src/main.rs').write_text('changed')
        with self.assertRaisesRegex(ValueError, 'changed'):
            archive(self.value, self.root / 'bad.tar', self.root)

    def test_escaping_links_missing_and_extra_files_fail(self):
        for name in ('../secret', '/etc/passwd', '.aws/credentials', 'tools/x/target/object'):
            with self.assertRaises(ValueError):
                checked_path(self.root, name)
        (self.root / 'src/link').symlink_to('/etc/passwd')
        with self.assertRaises(ValueError):
            checked_path(self.root, 'src/link')
        with self.assertRaisesRegex(ValueError, 'extra'):
            validate(self.value, self.root, exact=True)


class OfflineControls(unittest.TestCase):
    def test_only_module_relative_frames_are_decoded(self):
        raw = '#0 0x1234 (/work/target/a+0xabcd) (BuildId: abc)\n#1 0x5678 in unknown\n'
        frames = list(FRAME.finditer(raw))
        self.assertEqual([(row['module'], row['offset']) for row in frames], [('/work/target/a', '0xabcd')])

    def test_tool_pin_object_hash_and_source_identity_are_required(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            tool, binary, raw = root / 'symbolizer', root / 'a', root / 'raw'
            tool.write_bytes(b'fixture tool')
            binary.write_bytes(b'fixture ELF')
            raw.write_text('#0 0x1234 (/work/target/a+0xabcd)\n')
            objects = {'/work/target/a': dict(host_path=str(binary), sha256=digest(binary))}
            pin = dict(sha256=digest(tool))
            output = json.dumps([dict(Symbol=[dict(FileName='/work/source/asan_uaf.rs', Line=7)])])
            with patch('offline_symbolize.subprocess.run', return_value=subprocess.CompletedProcess([], 0, output, '')) as run:
                result = symbolize(raw, objects, tool, pin)
                require_source(result, 'asan_uaf.rs')
                self.assertIn('--no-debuginfod', run.call_args.args[0])
                with self.assertRaisesRegex(ValueError, 'intended source'):
                    require_source(result, 'lsan_leak.rs')
                with self.assertRaisesRegex(ValueError, 'pin'):
                    symbolize(raw, objects, tool, dict(sha256='0' * 64))
                binary.write_bytes(b'altered')
                with self.assertRaisesRegex(ValueError, 'changed'):
                    symbolize(raw, objects, tool, pin)

    def test_missing_objects_or_unknown_source_cannot_pass_a_control(self):
        result = dict(frames=[dict(status='unresolved')])
        with self.assertRaises(ValueError):
            require_source(result, 'asan_uaf.rs')


if __name__ == '__main__':
    unittest.main(verbosity=2)
