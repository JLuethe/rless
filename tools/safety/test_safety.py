"""Negative controls for evidence gates and test-only fault/readiness fixtures."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from common import Evidence, validate_report, write_json
from fixtures import Barrier, FaultPlan, transactional_append


class EvidenceGate(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        self.root = self.directory / 'repo'
        self.root.mkdir()
        subprocess.run(['git', 'init', '-q', self.root], check=True)
        (self.root / 'src').mkdir()
        (self.root / 'src/input.rs').write_text('fn main() {}\n')
        subprocess.run(['git', '-C', self.root, 'add', '.'], check=True)
        subprocess.run(['git', '-C', self.root, '-c', 'user.name=Safety Fixture', '-c',
                        'user.email=fixture@invalid', 'commit', '-qm', 'fixture'], check=True)
        self.evidence = Evidence(self.directory / 'run', self.root)

    def tearDown(self):
        self.temp.cleanup()

    def run_python(self, source, **kwargs):
        return self.evidence.run('fixture', [sys.executable, '-c', source], **kwargs)

    def assert_rejected(self):
        with self.assertRaises((ValueError, KeyError, OSError)):
            validate_report(self.evidence.directory / 'summary.json', self.root)

    def test_valid_safe_run(self):
        self.run_python('print("safe")')
        self.assertTrue(self.evidence.finish())
        validate_report(self.evidence.directory / 'summary.json', self.root)

    def test_empty_selection_cannot_pass(self):
        self.assertFalse(self.evidence.finish())
        self.evidence.report['status'] = 'passed'
        self.evidence.save()
        self.assert_rejected()

    def test_expected_nonzero_does_not_hide_asan(self):
        record = self.run_python('print("ERROR: AddressSanitizer: heap-use-after-free"); raise SystemExit(2)', expected=2)
        self.assertEqual(record['status'], 'failed')
        self.assert_rejected()

    def test_expected_detector_needs_intended_diagnostic(self):
        record = self.run_python('print("tool not found"); raise SystemExit(1)', diagnostic='heap-use-after-free')
        self.assertEqual(record['status'], 'failed')
        self.assert_rejected()

    def test_detector_negative_control_is_identified(self):
        self.run_python('print("ERROR: AddressSanitizer: heap-use-after-free"); raise SystemExit(86)',
                        diagnostic='ERROR: AddressSanitizer: heap-use-after-free')
        self.assertTrue(self.evidence.finish())
        validate_report(self.evidence.directory / 'summary.json', self.root)

    def test_missing_log_rejects_even_manual_pass_boolean(self):
        self.run_python('print("safe")')
        self.evidence.finish()
        (self.evidence.directory / 'fixture.log').unlink()
        self.assert_rejected()

    def test_altered_log_rejects(self):
        self.run_python('print("safe")')
        self.evidence.finish()
        (self.evidence.directory / 'fixture.log').write_text('different')
        self.assert_rejected()

    def test_changed_source_rejects(self):
        self.run_python('print("safe")')
        self.evidence.finish()
        (self.root / 'src/input.rs').write_text('changed')
        self.assert_rejected()

    def test_changed_binary_rejects(self):
        binary = self.directory / 'candidate'
        binary.write_bytes(b'old bytes')
        self.run_python('print("safe")', artifacts=[binary])
        self.evidence.finish()
        binary.write_bytes(b'new bytes')
        self.assert_rejected()

    def test_unsupported_required_record_rejects(self):
        self.evidence.unavailable('miri', 'missing', 'MIRI')
        self.assertFalse(self.evidence.finish())
        self.assert_rejected()

    def test_timeout_rejects_and_reaps_process(self):
        record = self.run_python('import time; time.sleep(10)', timeout=0.05)
        self.assertTrue(record['timed_out'])
        self.assertNotEqual(record['original_exit_status'], 0)
        self.assert_rejected()

    def test_duplicate_or_empty_command_rejects(self):
        with self.assertRaises(ValueError):
            self.evidence.run('nothing', [])
        self.run_python('print("safe")')
        with self.assertRaises(ValueError):
            self.run_python('print("safe")')


class FaultFixtures(unittest.TestCase):
    def test_each_failure_precedes_commit(self):
        for index in (1, 2):
            storage = bytearray(b'old')
            plan = FaultPlan(index)
            with self.assertRaises(MemoryError):
                transactional_append(storage, b'\x00\xffnew', plan)
            self.assertEqual(storage, b'old')
            self.assertEqual(len(plan.trace), index)
        storage = bytearray(b'old')
        transactional_append(storage, b'\x00\xffnew', FaultPlan())
        self.assertEqual(storage, b'old\x00\xffnew')

    def test_real_child_readiness_without_sleep(self):
        with Barrier() as barrier:
            child = subprocess.Popen([sys.executable, '-c',
                'import os,sys; os.write(int(sys.argv[1]), b"R")', str(barrier.write_fd)],
                pass_fds=[barrier.write_fd])
            try:
                barrier.wait()
                self.assertEqual(child.wait(timeout=5), 0)
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait()

    def test_timeout_and_bad_readiness_fail(self):
        with Barrier() as barrier:
            with self.assertRaises(TimeoutError):
                barrier.wait(0)
            os.write(barrier.write_fd, b'X')
            with self.assertRaises(RuntimeError):
                barrier.wait()

    def test_fixture_cycles_release_descriptors(self):
        initial = len(list(Path('/proc/self/fd').iterdir()))
        for _ in range(100):
            with Barrier() as barrier:
                barrier.ready()
                barrier.wait()
        self.assertEqual(len(list(Path('/proc/self/fd').iterdir())), initial)


class ObserverEnvironment(unittest.TestCase):
    def test_explicit_instrumentation_and_unknown_key_rejection(self):
        sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
        from run_v2 import run_case
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / 'env.json'
            previous = os.environ.get('RLESS_SAFETY_ENV')
            try:
                write_json(config, {'ASAN_OPTIONS': 'detect_leaks=1'})
                os.environ['RLESS_SAFETY_ENV'] = str(config)
                case = dict(id='instrumentation-control', mode='pipe', argv=['-c',
                    'import os; print(os.environ.get("ASAN_OPTIONS")); print(os.environ.get("RLESS_SAFETY_ENV"))'])
                result = run_case(Path('/usr/bin/python3'), case)
                import base64
                self.assertEqual(base64.b64decode(result['stdout_b64']), b'detect_leaks=1\nNone\n')
                write_json(config, {'LD_PRELOAD': '/arbitrary/library'})
                with self.assertRaises(ValueError):
                    run_case(Path('/usr/bin/python3'), case)
            finally:
                if previous is None:
                    os.environ.pop('RLESS_SAFETY_ENV', None)
                else:
                    os.environ['RLESS_SAFETY_ENV'] = previous


if __name__ == '__main__':
    unittest.main()
