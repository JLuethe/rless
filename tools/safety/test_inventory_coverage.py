"""Module/type-only files still carry policy and must not vanish from review."""
from pathlib import Path
import tempfile
import unittest

from common import sha
from run import source_file_summaries


class SourceCoverage(unittest.TestCase):
    def test_empty_event_module_and_type_inputs_keep_hash_bound_rows(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'src').mkdir()
            (root / 'src/lib.rs').write_text('#![forbid(unsafe_code)]\npub mod types;\n')
            (root / 'src/types.rs').write_text('pub enum ResultKind { Complete, Incomplete }\n')
            (root / 'src/state.rs').write_text('static mut STATE: i32 = 0;\n')
            sources = ['src/lib.rs', 'src/types.rs', 'src/state.rs']
            result = source_file_summaries([dict(file='src/state.rs', kind='mutable_static_definition')], sources, root)
            self.assertEqual(set(result), set(sources))
            for name in sources:
                self.assertEqual(result[name]['sha256'], sha(root / name))
            self.assertEqual(result['src/lib.rs']['counts'], {})
            self.assertEqual(result['src/types.rs']['counts'], {})
            self.assertEqual(result['src/state.rs']['counts'], {'mutable_static_definition': 1})
            old = result['src/lib.rs']['sha256']
            (root / 'src/lib.rs').write_text('pub mod types;\n')
            self.assertNotEqual(source_file_summaries([], sources, root)['src/lib.rs']['sha256'], old)

    def test_missing_input_cannot_be_silently_omitted(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaises(FileNotFoundError):
                source_file_summaries([], ['missing.rs'], Path(directory))


if __name__ == '__main__':
    unittest.main(verbosity=2)
