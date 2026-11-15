"""Negative controls for evidence gates and test-only fault/readiness fixtures."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from common import (Evidence, sha, validate_detector_capture_evidence,
                    validate_fault_oracle_evidence, validate_fault_reference_bundle, validate_report, write_json)
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

    def test_report_level_sanitizer_diagnostics_cannot_hide_behind_passed_records(self):
        self.run_python('print("safe")')
        diagnostic = self.directory / 'late-sanitizer.log'
        diagnostic.write_text('ERROR: AddressSanitizer: heap-use-after-free\n')
        self.evidence.report['sanitizer_reports'] = [dict(path=str(diagnostic), sha256=sha(diagnostic))]
        self.assertFalse(self.evidence.finish())
        # Even a manually promoted aggregate and permission for assigned legacy
        # findings cannot hide these unclassified application diagnostics.
        self.evidence.report['status'] = 'passed'
        self.evidence.save()
        with self.assertRaisesRegex(ValueError, 'report-level sanitizer'):
            validate_report(self.evidence.directory / 'summary.json', self.root, allow_open_findings=True)

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

    def test_changed_tool_rejects(self):
        from common import sha
        tool = self.directory / 'tool'
        tool.write_bytes(b'original tool')
        self.evidence.report['tool_files_sha256'] = {str(tool): sha(tool)}
        self.run_python('print("safe")')
        self.evidence.finish()
        tool.write_bytes(b'changed tool')
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

    def test_scoped_record_does_not_approve_incomplete_campaign(self):
        self.run_python('print("safe")')
        self.evidence.unavailable('unfinished', 'missing native counterpart', 'ASAN')
        self.assertFalse(self.evidence.finish())
        self.assert_rejected()
        validate_report(self.evidence.directory / 'summary.json', self.root,
                        record_names=['fixture'])
        for names in ([], ['not-run'], ['fixture', 'fixture'], ['unfinished']):
            with self.assertRaises(ValueError):
                validate_report(self.evidence.directory / 'summary.json', self.root,
                                record_names=names)

    def test_legacy_diagnostic_is_never_a_final_pass(self):
        record = self.run_python('print("observed known legacy failure")', finding_ids=['LEGACY-1'])
        self.assertEqual(record['status'], 'finding-open')
        self.assertFalse(self.evidence.finish())
        self.assert_rejected()
        validate_report(self.evidence.directory / 'summary.json', self.root,
                        allow_open_findings=True)
        record['status'] = 'passed'
        self.evidence.save()
        self.assert_rejected()

    def test_a_failed_oracle_cannot_be_saved_as_a_legacy_diagnostic(self):
        record = self.run_python('raise SystemExit(3)', finding_ids=['LEGACY-1'])
        self.assertEqual(record['status'], 'failed')
        with self.assertRaises(ValueError):
            validate_report(self.evidence.directory / 'summary.json', self.root,
                            allow_open_findings=True)

    def test_unexecuted_infrastructure_cannot_be_declared_a_legacy_finding(self):
        with self.assertRaises(ValueError):
            self.evidence.unavailable('missing', 'tool unavailable', 'ASAN', kind='legacy')

    def test_oracle_metadata_requires_current_corpus_and_exact_domains(self):
        (self.root / 'compat').mkdir()
        corpus = self.root / 'compat/corpus.json'
        corpus.write_text('{}')
        evidence = Evidence(self.directory / 'metadata-run', self.root)
        metadata = dict(abi_profile='BUILDER-GLIBC-2.44', corpus_sha256=sha(corpus),
            seed=712, test_selector='Fixture.test_read_error', hook_sites=['read'], hook_ordinals=[1, 2])
        for changes in ({'hook_ordinals': [0]}, {'hook_ordinals': [True]},
                        {'hook_sites': ['read', 'read']}, {'abi_profile': 'ANY'}, {'seed': -1}):
            with self.assertRaises(ValueError):
                evidence.run('bad', [sys.executable, '-c', 'print("safe")'],
                             oracle_metadata={**metadata, **changes})
        item = evidence.run('valid', [sys.executable, '-c', 'print("safe")'], oracle_metadata=metadata)
        validate_report(evidence.directory / 'summary.json', self.root)
        item['oracle_metadata']['corpus_sha256'] = '0' * 64
        evidence.save()
        with self.assertRaises(ValueError):
            validate_report(evidence.directory / 'summary.json', self.root)

    def test_declarative_fault_metadata_needs_a_retained_observation_report(self):
        item = dict(oracle_metadata={'test_selector': 'ActualFaultPaths.test_fixture'},
                    observed_hooks=[dict(site='read', ordinal=1, magic='RLF1')])
        with self.assertRaisesRegex(ValueError, 'retained observation report'):
            validate_fault_oracle_evidence(item, self.root)

    def test_detector_capture_rederives_raw_diagnostics_not_capture_verified(self):
        binary = self.directory / 'detector-fixture'
        binary.write_bytes(b'synthetic detector fixture')
        report = {'schema_version': 1, 'capture_verified': True, 'records': []}
        for probe, status, diagnostic in (
                ('safe', 0, None),
                ('asan-uaf', 87, 'ERROR: AddressSanitizer: heap-use-after-free'),
                ('lsan-leak', 87, 'ERROR: LeakSanitizer: detected memory leaks: 8192 byte')):
            for mode in ('pipe', 'pty'):
                logs = []
                if diagnostic:
                    path = self.directory / (probe + '-' + mode + '.log')
                    path.write_text(diagnostic)
                    logs = [dict(path=str(path), sha256=sha(path))]
                report['records'].append(dict(probe=probe, mode=mode, status=status,
                    binary=str(binary), binary_sha256=sha(binary), diagnostics=logs,
                    termios_restored=True, trace_hex=''))
        path = self.directory / 'capture.json'
        write_json(path, report)
        item = dict(oracle_metadata={'test_selector': 'harness_controls.validate_capture'},
                    detector_capture_report=dict(path=str(path), sha256=sha(path)))
        self.assertTrue(validate_detector_capture_evidence(item, self.root))
        report['records'][-1]['diagnostics'] = []
        write_json(path, report)
        item['detector_capture_report']['sha256'] = sha(path)
        with self.assertRaisesRegex(ValueError, 'intended detector'):
            validate_detector_capture_evidence(item, self.root)

    def test_host_report_cannot_supply_a_derived_capture_proof_bit(self):
        item = self.run_python('print("safe")')
        item['_validated_detector_capture'] = True
        item['_validated_m0_controls'] = ['inventory-omission-controls']
        self.evidence.save()
        validated = validate_report(self.evidence.directory / 'summary.json', self.root)
        self.assertIs(validated['records'][0]['_validated_detector_capture'], False)
        self.assertEqual(validated['records'][0]['_validated_m0_controls'], [])

    def test_boolean_exit_status_cannot_impersonate_success(self):
        item = self.run_python('print("safe")')
        item['original_exit_status'] = False
        self.evidence.save()
        with self.assertRaisesRegex(ValueError, 'exit statuses must be integers'):
            validate_report(self.evidence.directory / 'summary.json', self.root)

    def test_fault_reference_bundle_rejects_substitution_and_changed_files(self):
        import copy
        import sys
        sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
        from release import capture_reference_bundle, reference_policy
        bundle_dir = self.directory / 'reference'
        bundle_dir.mkdir()
        names = ('less', 'lessecho', 'less-osc8-open.sh')
        for name in names:
            (bundle_dir / name).write_text('qualified fixture ' + name)
        policy = dict(upstream=dict(repository='https://example.invalid/less.git',
                                   version='712', commit='a' * 40), regex='gnu',
                      files_sha256={name: sha(bundle_dir / name) for name in names})
        authority = self.root / 'tools/safety/profiles.json'
        write_json(authority, dict(reference_bundle=policy))
        bundle = capture_reference_bundle(bundle_dir / 'less', self.root)
        report = dict(reference_policy=reference_policy(self.root), reference_bundle=bundle,
                      reference=bundle['less'], records=[dict(oracle_metadata={
                          'test_selector': 'ActualFaultPaths.test_fixture'})])
        self.assertTrue(validate_fault_reference_bundle(report, self.root))
        for field in ('reference_policy', 'reference_bundle', 'reference'):
            incomplete = copy.deepcopy(report)
            incomplete.pop(field)
            with self.assertRaises(ValueError):
                validate_fault_reference_bundle(incomplete, self.root)
        changed = copy.deepcopy(report)
        changed['reference_bundle']['less']['sha256'] = 'b' * 64
        changed['reference_policy']['files_sha256']['less'] = 'b' * 64
        with self.assertRaises(ValueError):
            validate_fault_reference_bundle(changed, self.root)
        (bundle_dir / 'lessecho').write_text('substituted helper')
        with self.assertRaises(ValueError):
            validate_fault_reference_bundle(report, self.root)

    def test_fault_reference_and_copied_marker_claims_are_checked(self):
        from test_fault_paths import contracts
        name = 'test_diagnostic_read_error_reproduces_unreported_legacy_failure'
        selector = 'ActualFaultPaths.' + name
        declaration = contracts()[name]
        oracle = self.directory / 'oracle.json'
        write_json(oracle, {'repetitions': 1})
        observed = [dict(site='read', ordinal=1, magic='RLF1')]
        item = dict(command=['fixture', selector], scenario_id=declaration['scenario_id'],
            variant=declaration['variant'], finding_ids=declaration['finding_ids'],
            oracle_metadata=dict(test_selector=selector, repetitions=1,
                **{key: declaration[key] for key in ('hook_sites', 'hook_ordinals', 'm0_controls')}),
            oracle_report=dict(path=str(oracle), sha256=sha(oracle)),
            observed_hooks=observed.copy())
        # The parser has its own marker mutation controls; this checks binding
        # of its rederived output to the common evidence record.
        with patch('test_fault_paths.validate_oracle_report', return_value={selector: observed}):
            self.assertEqual(validate_fault_oracle_evidence(item, self.root), observed)
            item['observed_hooks'] = [dict(site='read', ordinal=2, magic='RLF1')]
            with self.assertRaisesRegex(ValueError, 'retained marker bytes'):
                validate_fault_oracle_evidence(item, self.root)
            item['observed_hooks'] = observed
            item['oracle_metadata']['hook_sites'] = ['write']
            with self.assertRaisesRegex(ValueError, 'current contract'):
                validate_fault_oracle_evidence(item, self.root)
            item['oracle_metadata']['hook_sites'] = declaration['hook_sites']
            oracle.write_text('{"repetitions":2}')
            with self.assertRaisesRegex(ValueError, 'changed fault observation report'):
                validate_fault_oracle_evidence(item, self.root)


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
