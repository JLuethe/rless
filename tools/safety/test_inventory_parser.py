"""Exercise the actual Rust inventory parser, including opaque shared static types."""
import argparse
from pathlib import Path
import subprocess
import tempfile
import unittest

from inventory_format import all_statics, parse_records


BINARY = None


class ActualInventoryParser(unittest.TestCase):
    def parse(self, source):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'fixture.rs'
            path.write_text(source)
            result = subprocess.run([str(BINARY), str(path)], capture_output=True, text=True,
                                    check=True, timeout=10)
            return parse_records(result.stdout)

    def test_atomic_alias_shared_and_mutable_definitions_are_distinct_from_externs(self):
        records = self.parse('''
type Shared = std::sync::atomic::AtomicUsize;
static SIGNAL: Shared = Shared::new(0);
static IMMUTABLE: u8 = 1;
static mut RAW: u8 = 0;
extern "C" { static mut FOREIGN: u8; }
''')
        statics = {item['symbol']: item for item in all_statics(records)}
        self.assertEqual(set(statics), {'SIGNAL', 'IMMUTABLE', 'RAW'})
        self.assertEqual(statics['SIGNAL']['declared_type'], 'Shared')
        self.assertEqual(statics['SIGNAL']['binding_mutability'], 'shared')
        self.assertEqual(statics['SIGNAL']['interior_mutability'], 'requires-review')
        self.assertEqual(statics['RAW']['binding_mutability'], 'mutable')
        self.assertEqual([item['symbol'] for item in records if item['kind'] == 'mutable_static_definition'], ['RAW'])
        self.assertEqual([item['symbol'] for item in records if item['kind'] == 'extern_static_declaration'], ['FOREIGN'])

    def test_callback_and_interior_mutable_wrapper_types_are_retained_without_guessing(self):
        records = self.parse('''
static CALLBACK: unsafe extern "C" fn() = callback;
static WRAPPED: UnknownWrapper = make();
static CELL: std::cell::UnsafeCell<u8> = std::cell::UnsafeCell::new(0);
''')
        statics = {item['symbol']: item for item in all_statics(records)}
        self.assertEqual(set(statics), {'CALLBACK', 'WRAPPED', 'CELL'})
        self.assertIn('extern "C"', statics['CALLBACK']['declared_type'])
        self.assertEqual(statics['WRAPPED']['declared_type'], 'UnknownWrapper')
        self.assertTrue(any(item['kind'] == 'function_pointer_type' for item in records))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    args = parser.parse_args()
    BINARY = args.binary.resolve(strict=True)
    unittest.main(argv=['test_inventory_parser'], verbosity=2)
