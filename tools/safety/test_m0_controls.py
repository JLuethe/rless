"""Negative controls for exact M0 command/log selection. Never product evidence."""
from pathlib import Path
import tempfile
import unittest

from common import sha
from m0_controls import checked, diagnostic, exact, unittest_ids, validate_unittest


class ControlSelection(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.log = self.root / 'command.log'
        self.log.write_text('selected result\n')
        self.record = dict(name='fixture', command=['cargo', 'miri', 'test'],
            profile='MIRI', original_exit_status=0, expected_diagnostic=None, log=str(self.log))

    def tearDown(self):
        self.temporary.cleanup()

    def test_same_output_from_different_command_cannot_supply_credit(self):
        self.assertEqual(exact(self.record, ['cargo', 'miri', 'test'], profile='MIRI'), 'selected result\n')
        for change in ({'command': ['python', '-c', 'print("selected result")']},
                       {'profile': 'NATIVE'}, {'original_exit_status': False}, {'original_exit_status': 1}):
            with self.subTest(change=change), self.assertRaises(ValueError):
                exact({**self.record, **change}, ['cargo', 'miri', 'test'], profile='MIRI')

    def test_rehashed_wrong_diagnostic_or_broadened_expected_pattern_rejects(self):
        intended = r'ERROR: AddressSanitizer: heap-use-after-free'
        self.record.update(original_exit_status=86, expected_diagnostic=intended)
        self.log.write_text(intended)
        diagnostic(self.record, intended)
        self.record['expected_diagnostic'] = '.*'
        with self.assertRaises(ValueError):
            diagnostic(self.record, intended)
        self.record['expected_diagnostic'] = intended
        self.log.write_text('unrelated failure')
        with self.assertRaises(ValueError):
            diagnostic(self.record, intended)

    def test_retained_child_report_hash_is_mandatory(self):
        reference = dict(path=str(self.log), sha256=sha(self.log))
        self.assertEqual(checked(reference), self.log)
        self.log.write_text('changed')
        with self.assertRaises(ValueError):
            checked(reference)


class NamedUnitCompletion(unittest.TestCase):
    def setUp(self):
        self.expected = [('test_one', 'fixture.Controls.test_one'), ('test_two', 'fixture.Controls.test_two')]
        self.text = ('test_one (fixture.Controls.test_one) ... ok\n'
                     'test_two (fixture.Controls.test_two) ... nested evidence progress\nok\n'
                     '----------------------------------------------------------------------\n'
                     'Ran 2 tests in 0.100s\n\nOK\n')

    def test_all_source_selected_methods_and_exact_count_pass(self):
        validate_unittest(self.text, self.expected)

    def test_success_footer_does_not_hide_missing_duplicate_or_failed_method(self):
        mutations = [self.text.replace('test_one (fixture.Controls.test_one) ... ok\n', ''),
                     self.text.replace('test_two (fixture.Controls.test_two)', 'test_one (fixture.Controls.test_one)'),
                     self.text.replace('... ok\n', '... skipped "unsupported"\n'),
                     self.text.replace('Ran 2 tests', 'Ran 1 test'),
                     self.text.replace('nested evidence progress\nok', 'ERROR'),
                     self.text + 'test_extra (fixture.Controls.test_extra) ... ok\n']
        for value in mutations:
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate_unittest(value, self.expected)

    def test_current_source_selection_cannot_be_silently_empty(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'tools/safety').mkdir(parents=True)
            (root / 'tools/safety/example.py').write_text(
                'class Wanted:\n    def test_one(self): pass\nclass Other:\n    def test_two(self): pass\n')
            self.assertEqual(unittest_ids(['example.Wanted'], root), [('test_one', 'example.Wanted.test_one')])
            with self.assertRaises(ValueError):
                unittest_ids(['example.Missing'], root)


if __name__ == '__main__':
    unittest.main()
