"""Identity mutation controls; synthetic ELF fixtures are not pager evidence."""
import copy
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import native_callback as callback
from test_fault_paths import contracts, marker, validate_oracle_report


class CallbackBindingControls(unittest.TestCase):
    def binding(self):
        return dict(schema_version=1, binary_sha256='a'*64,
            symbol='_ZN5rless6screen13inc_costcount17h0123456789abcdefE', offset=0x1234,
            helper_sha256=callback.sha(callback.__file__), shim_source_sha256=callback.sha(callback.SOURCE))

    def test_shape_rejects_omitted_changed_or_ambiguous_identity(self):
        good = self.binding()
        callback.validate_shape(good)
        for value in (None, {}, dict(good, offset=True), dict(good, offset=0),
                      dict(good, symbol='putchr'), dict(good, helper_sha256='0'*64),
                      dict(good, shim_source_sha256='0'*64), dict(good, extra=True)):
            with self.subTest(value=value), self.assertRaises(ValueError):
                callback.validate_shape(value)

    def test_bind_requires_exact_callback_symbol_and_supported_elf(self):
        header = bytearray(64)
        header[:6] = b'\x7fELF\x02\x01'
        header[16:18] = (3).to_bytes(2, 'little')
        header[18:20] = (62).to_bytes(2, 'little')
        symbol = self.binding()['symbol']
        real_sha = callback.sha
        # Only the fake binary read is substituted; helper/source hashes remain real.
        with patch.object(Path, 'read_bytes', return_value=bytes(header)), \
                patch.object(callback, 'sha', return_value='a'*64), \
                patch.object(callback, 'ElfTargets', return_value=SimpleNamespace(symbols={symbol:{0x1234}})) as elf:
            self.assertEqual(callback.bind('/fixture')['offset'], 0x1234)
            for symbols in ({}, {symbol:{0}}, {symbol:{0x1234,0x2345}}, {'putchr':{0x1234}}):
                elf.return_value = SimpleNamespace(symbols=symbols)
                with self.assertRaises(ValueError):
                    callback.bind('/fixture')
        with patch.object(Path, 'read_bytes', return_value=b'not ELF'):
            with self.assertRaises(ValueError):
                callback.bind('/fixture')

    def test_retained_validation_rederives_mapped_elf(self):
        good = self.binding()
        report = dict(records=[dict(observations=[dict(binary='/work/pager', native_callback_binding=good)])])
        with patch.object(callback, 'bind', return_value=good) as bind:
            callback.validate_observations(report, lambda value: Path('/retained')/value.removeprefix('/'))
            bind.assert_called_once_with(Path('/retained/work/pager'))
        for field, value in [('offset', 0x4321), ('binary_sha256', 'b'*64)]:
            with patch.object(callback, 'bind', return_value=dict(good, **{field:value})):
                with self.assertRaisesRegex(ValueError, 'retained pager ELF'):
                    callback.validate_observations(report)

    def test_native_callback_marker_cannot_claim_an_unbound_target(self):
        method = 'test_native_regex_and_callback_counterparts_preserve_observation'
        selector = 'ActualFaultPaths.' + method
        observation = dict(kind='pty', status=0, termios_restored=True, native_plan='callback:1',
            native_callback_binding=self.binding(), trace_hex=marker(10, magic=b'RNT1').hex())
        report = dict(schema_version=2, passed=True, repetitions=1, tests_run=1,
            records=[dict(test_selector=selector, passed=True, repetition_mode='test-invocations',
                invocation_observation_counts=[1], contract=contracts()[method], observations=[observation])])
        self.assertEqual(validate_oracle_report(report, [selector])[selector][0]['site'], 'callback')
        for field, value in [('native_callback_binding', None), ('native_plan', 'callback:2')]:
            changed = copy.deepcopy(report)
            changed['records'][0]['observations'][0][field] = value
            with self.assertRaises(ValueError):
                validate_oracle_report(changed, [selector])


if __name__ == '__main__':
    unittest.main(verbosity=2)
