"""Mutation controls for retained oracle parsing; fixtures are not product evidence."""
import copy
import io
import json
from pathlib import Path
import signal
import tempfile
import unittest
from unittest.mock import call, patch

from common import ROOT, sha
from harness_controls import validate_capture
import test_fault_paths as faults
from test_fault_paths import INTERNAL_REPEAT_METHODS, SITE_IDS, contracts, finish_process_group, marker, validate_oracle_report


class RepetitionExecutionControls(unittest.TestCase):
    def test_noncycle_selector_is_actually_invoked_for_every_repetition(self):
        class Fixture(unittest.TestCase):
            def test_read_error_reports_failure_without_discarding_status(self):
                faults.OBSERVATIONS.append(dict(kind='pipe', status=0, trace_hex=marker(2).hex()))
        with patch.object(faults, 'REPETITIONS', 3):
            runner = faults.RepeatingOracleRunner(stream=io.StringIO(), verbosity=0, resultclass=faults.OracleResult)
            result = runner.run(unittest.defaultTestLoader.loadTestsFromTestCase(Fixture))
        self.assertEqual(result.testsRun, 3)
        self.assertEqual(len(result.oracles), 1)
        self.assertEqual(result.oracles[0]['invocation_observation_counts'], [1, 1, 1])
        report = dict(schema_version=2, repetitions=3, tests_run=result.testsRun,
                      passed=result.wasSuccessful(), records=result.oracles)
        validate_oracle_report(report, [result.oracles[0]['test_selector']])

    def test_internal_cycle_selector_is_not_multiplied_twice(self):
        class Fixture(unittest.TestCase):
            def test_callback_startup_failure_preserves_later_session_lifecycle(self):
                faults.OBSERVATIONS.append({'fixture': 'internal cycle body owns its repetition'})
        with patch.object(faults, 'REPETITIONS', 3):
            runner = faults.RepeatingOracleRunner(stream=io.StringIO(), verbosity=0, resultclass=faults.OracleResult)
            result = runner.run(unittest.defaultTestLoader.loadTestsFromTestCase(Fixture))
        self.assertEqual(result.testsRun, 1)
        self.assertEqual(result.oracles[0]['invocation_observation_counts'], [1])
        self.assertEqual(result.oracles[0]['repetition_mode'], 'internal-cycles')


class ProcessGroupControls(unittest.TestCase):
    def test_transient_group_is_allowed_to_finish_reaping(self):
        with patch('test_fault_paths.os.killpg', side_effect=[None, ProcessLookupError]) as kill, \
                patch('test_fault_paths.time.sleep'):
            finish_process_group(123)
        self.assertEqual(kill.call_args_list, [call(123, 0), call(123, 0)])

    def test_group_disappearing_at_cleanup_boundary_is_empty(self):
        with patch('test_fault_paths.os.killpg', side_effect=[None, ProcessLookupError]) as kill:
            finish_process_group(123, grace_seconds=0)
        self.assertEqual(kill.call_args_list, [call(123, 0), call(123, signal.SIGKILL)])

    def test_live_group_at_deadline_is_killed_and_rejected(self):
        with patch('test_fault_paths.os.killpg') as kill:
            with self.assertRaisesRegex(ValueError, 'survived pager termination'):
                finish_process_group(123, grace_seconds=0)
        self.assertEqual(kill.call_args_list, [call(123, 0), call(123, signal.SIGKILL)])


class RetainedFaultOracleControls(unittest.TestCase):
    def test_every_production_oracle_has_an_actual_catalogue_row_in_both_lanes(self):
        catalogue = json.loads((ROOT / 'spec/safety/acceptance.json').read_text())
        scenarios = {item['id']: item for item in catalogue['scenarios']}
        for name, contract in contracts().items():
            with self.subTest(test=name):
                scenario = scenarios[contract['scenario_id']]
                self.assertIn(contract['variant'], scenario['variants'])
                self.assertTrue({'NATIVE', 'ASAN'} <= set(scenario['required_profiles']))

    def setUp(self):
        self.method = 'test_read_error_reports_failure_without_discarding_status'
        self.selector = 'ActualFaultPaths.' + self.method
        self.report = dict(schema_version=2, passed=True, repetitions=1, tests_run=1,
            records=[dict(test_selector=self.selector, passed=True,
                repetition_mode='test-invocations', invocation_observation_counts=[1],
                contract=contracts()[self.method], observations=[dict(kind='pipe', status=0,
                    trace_hex=marker(SITE_IDS['read']).hex())])])

    def validate(self, report=None, selectors=None):
        return validate_oracle_report(self.report if report is None else report,
            expected_selectors=[self.selector] if selectors is None else selectors)

    def test_actual_bytes_determine_event_identity(self):
        self.assertEqual(self.validate(), {self.selector: [dict(site='read', ordinal=1, magic='RLF1')]})
        self.report['records'][0]['observations'][0]['trace_hex'] = marker(SITE_IDS['read'], magic=b'RNT1').hex()
        self.assertEqual(self.validate()[self.selector][0]['magic'], 'RNT1')

    def test_declared_repetition_count_cannot_replace_executed_invocations(self):
        self.report.update(repetitions=3, tests_run=3)
        with self.assertRaisesRegex(ValueError, 'invocation observations'):
            self.validate()
        self.report['tests_run'] = 1
        with self.assertRaisesRegex(ValueError, 'repetitions were not executed'):
            self.validate()

    def test_unbounded_write_evidence_cannot_substitute_a_finite_or_live_producer(self):
        name = 'test_write_failure_stops_unbounded_input_and_broken_stderr'
        selector = 'ActualFaultPaths.' + name
        observations = [dict(kind='pipe', status=1, process_reaped=True, process_group_empty=True,
            input_kind='unbounded-pipe', producer_stopped=True, producer_broken_pipe=True,
            producer_bytes=8192, stdout_full=i >= 2, stderr_full=i == 3,
            trace_hex=(marker(4, magic=b'RLF1' if i == 0 else b'RNT1').hex() if i < 2 else ''))
            for i in range(4)]
        report = dict(schema_version=2, passed=True, repetitions=1, tests_run=1,
            records=[dict(test_selector=selector, passed=True, contract=contracts()[name],
                repetition_mode='test-invocations', invocation_observation_counts=[4], observations=observations)])
        validate_oracle_report(report, [selector])
        for field, value in [('input_kind', 'pipe'), ('producer_stopped', False),
                             ('producer_broken_pipe', False), ('producer_bytes', 0),
                             ('producer_bytes', True), ('producer_bytes', 1024 * 1024),
                             ('status', 0), ('stdout_full', True)]:
            changed = copy.deepcopy(report)
            changed['records'][0]['observations'][0][field] = value
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                validate_oracle_report(changed, [selector])
        changed = copy.deepcopy(report)
        changed['records'][0]['observations'][-1]['stderr_full'] = False
        with self.assertRaises(ValueError):
            validate_oracle_report(changed, [selector])

    def test_missing_false_or_inconsistent_invocation_counts_reject(self):
        for counts in (None, [], [True], [0], [2], [1, 1]):
            report = copy.deepcopy(self.report)
            report['records'][0]['invocation_observation_counts'] = counts
            with self.subTest(counts=counts), self.assertRaises(ValueError):
                self.validate(report)

    def test_empty_trace_and_a_declared_plan_supply_no_hit_credit(self):
        observation = self.report['records'][0]['observations'][0]
        observation.update(trace_hex='', plan='read:1', observed_hooks=[dict(site='read', ordinal=1, magic='RLF1')])
        self.assertEqual(self.validate(), {self.selector: []})

    def test_invalid_trace_events_reject(self):
        for trace in ('not hex', '00', marker(2, magic=b'FAKE').hex(),
                      marker(999).hex(), marker(2, ordinal=0).hex(),
                      marker(SITE_IDS['write']).hex(), marker(2, ordinal=2).hex()):
            with self.subTest(trace=trace):
                report = copy.deepcopy(self.report)
                report['records'][0]['observations'][0]['trace_hex'] = trace
                with self.assertRaises(ValueError):
                    self.validate(report)
        del self.report['records'][0]['observations'][0]['trace_hex']
        with self.assertRaises(ValueError):
            self.validate()

    def test_changing_declared_ordinal_cannot_authorize_a_new_hit(self):
        self.report['records'][0]['contract']['hook_ordinals'] = [2]
        self.report['records'][0]['observations'][0]['trace_hex'] = marker(2, ordinal=2).hex()
        with self.assertRaisesRegex(ValueError, 'contract changed'):
            self.validate()

    def test_counts_schema_and_process_exit_are_not_booleans(self):
        for field in ('schema_version', 'repetitions', 'tests_run'):
            report = copy.deepcopy(self.report)
            report[field] = True
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.validate(report)
        self.report['records'][0]['observations'][0]['status'] = False
        with self.assertRaises(ValueError):
            self.validate()

    def test_missing_duplicate_or_unknown_selectors_reject(self):
        for selectors in ([], [self.selector, self.selector], ['ActualFaultPaths.missing']):
            with self.subTest(selectors=selectors), self.assertRaises(ValueError):
                self.validate(selectors=selectors)
        for records in ([], self.report['records'] * 2):
            report = {**self.report, 'records': records}
            with self.subTest(records=len(records)), self.assertRaises(ValueError):
                self.validate(report)
        other = 'ActualFaultPaths.test_write_error_reports_failure_without_discarding_status'
        report = copy.deepcopy(self.report)
        report.update(tests_run=2, records=report['records'] * 2)
        with self.assertRaises(ValueError):
            self.validate(report, [self.selector, other])

    def test_failed_or_unobserved_test_cannot_be_reported_as_passed(self):
        for changes in ({'passed': False}, {'observations': []},
                        {'test_selector': 'ActualFaultPaths.missing'}):
            report = copy.deepcopy(self.report)
            report['records'][0].update(changes)
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                self.validate(report)

    def test_terminal_restoration_must_be_observed_true(self):
        for restored in (None, False, 1, 'true'):
            report = copy.deepcopy(self.report)
            report['records'][0]['observations'][0].update(kind='pty', termios_restored=restored)
            with self.subTest(restored=restored), self.assertRaises(ValueError):
                self.validate(report)

    def retained_control(self, method, observation):
        selector = 'ActualFaultPaths.' + method
        internal = method in INTERNAL_REPEAT_METHODS
        observations = [observation, copy.deepcopy(observation)] if internal else [observation]
        report = dict(schema_version=2, passed=True, repetitions=1, tests_run=1,
            records=[dict(test_selector=selector, passed=True, contract=contracts()[method],
                          repetition_mode='internal-cycles' if internal else 'test-invocations',
                          invocation_observation_counts=[len(observations)], observations=observations)])
        return report, selector

    def test_termination_requires_actual_pipe_reaping_and_empty_group(self):
        observation = dict(kind='pipe', status=1, trace_hex=marker(SITE_IDS['alloc']).hex(),
                           process_reaped=True, process_group_empty=True)
        report, selector = self.retained_control(
            'test_pipe_failure_termination_reaps_process_and_child_group', observation)
        self.assertEqual(self.validate(report, [selector])[selector],
                         [dict(site='alloc', ordinal=1, magic='RLF1')])
        for field in ('process_reaped', 'process_group_empty'):
            for value in (None, False, 1, 'true'):
                changed = copy.deepcopy(report)
                changed['records'][0]['observations'][0][field] = value
                with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                    self.validate(changed, [selector])
        observation.update(kind='pty', termios_restored=True)
        with self.assertRaises(ValueError):
            self.validate(report, [selector])

    def test_resource_conservation_requires_live_comparable_checkpoints(self):
        checkpoints = [dict(descriptors=['/dev/pts/fixture', '/dev/pts/fixture', '/input/i'], children=[])
                       for _ in range(3)]
        observation = dict(kind='pty', status=0, termios_restored=True, trace_hex='', resources=checkpoints)
        report, selector = self.retained_control(
            'test_repeated_file_and_child_failures_conserve_live_process_resources', observation)
        self.assertEqual(self.validate(report, [selector]), {selector: []})
        invalid = [None, [], checkpoints[:1],
                   [checkpoints[0], dict(descriptors=[], children=[])],
                   [checkpoints[0], dict(descriptors=[42, '/input/i', '/dev/pts/fixture'], children=[])],
                   [checkpoints[0], dict(descriptors=['/input/i'], children=[])],
                   [checkpoints[0], dict(descriptors=checkpoints[0]['descriptors'], children=['12345'])],
                   [checkpoints[0], dict(descriptors=checkpoints[0]['descriptors'])]]
        for resources in invalid:
            changed = copy.deepcopy(report)
            changed['records'][0]['observations'][0]['resources'] = resources
            with self.subTest(resources=resources), self.assertRaises(ValueError):
                self.validate(changed, [selector])
        observation['kind'] = 'pipe'
        with self.assertRaises(ValueError):
            self.validate(report, [selector])

    def test_failed_extra_observation_cannot_hide_behind_a_good_checkpoint(self):
        good = dict(kind='pty', status=0, termios_restored=True, trace_hex='',
                    resources=[dict(descriptors=['/input/i'], children=[]) for _ in range(3)])
        report, selector = self.retained_control(
            'test_repeated_file_and_child_failures_conserve_live_process_resources', good)
        bad = copy.deepcopy(good)
        bad['resources'][-1]['children'] = ['12345']
        report['records'][0]['observations'][1] = bad
        with self.assertRaises(ValueError):
            self.validate(report, [selector])


class DetectorCaptureControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        binary = self.root / 'synthetic-binary'
        binary.write_bytes(b'fixture, never executed')
        self.report = dict(schema_version=1, capture_verified=True, records=[])
        for probe, status, text in (
                ('safe', 0, None),
                ('asan-uaf', 87, 'ERROR: AddressSanitizer: heap-use-after-free'),
                ('lsan-leak', 87, 'ERROR: LeakSanitizer: detected memory leaks: 8192 byte')):
            for mode in ('pipe', 'pty'):
                diagnostics = []
                if text:
                    path = self.root / (probe + '-' + mode)
                    path.write_text(text)
                    diagnostics.append(dict(path=str(path), sha256=sha(path)))
                self.report['records'].append(dict(probe=probe, mode=mode, status=status,
                    binary=str(binary), binary_sha256=sha(binary), trace_hex='',
                    termios_restored=True, diagnostics=diagnostics))

    def tearDown(self):
        self.temporary.cleanup()

    def test_all_six_real_shapes_are_required(self):
        self.assertTrue(validate_capture(self.report))
        for records in (self.report['records'][:-1], self.report['records'][:-1] + self.report['records'][:1]):
            with self.subTest(count=len(records)), self.assertRaises(ValueError):
                validate_capture({**self.report, 'records': records})

    def test_schema_and_trace_cannot_be_omitted_or_changed(self):
        for schema in (None, True, 2):
            with self.subTest(schema=schema), self.assertRaises(ValueError):
                validate_capture({**self.report, 'schema_version': schema})
        for trace in (None, marker(1).hex(), 'not hex'):
            report = copy.deepcopy(self.report)
            report['records'][0]['trace_hex'] = trace
            with self.subTest(trace=trace), self.assertRaises(ValueError):
                validate_capture(report)

    def test_probe_specific_exit_and_terminal_restoration_are_required(self):
        for index, status in ((0, False), (0, 86), (2, 86), (4, 86)):
            report = copy.deepcopy(self.report)
            report['records'][index]['status'] = status
            with self.subTest(index=index, status=status), self.assertRaises(ValueError):
                validate_capture(report)
        self.report['records'][1]['termios_restored'] = False
        with self.assertRaises(ValueError):
            validate_capture(self.report)

    def test_raw_log_and_binary_hash_tampering_reject(self):
        item = self.report['records'][2]['diagnostics'][0]
        path = Path(item['path'])
        old = path.read_bytes()
        path.write_bytes(old + b' changed')
        with self.assertRaises(ValueError):
            validate_capture(self.report)
        path.write_bytes(old)
        Path(self.report['records'][0]['binary']).write_bytes(b'changed binary')
        with self.assertRaises(ValueError):
            validate_capture(self.report)

    def test_rehashed_wrong_diagnostic_still_rejects(self):
        for index, text in ((2, 'ERROR: LeakSanitizer: detected memory leaks: 8192 byte'),
                            (4, 'ERROR: LeakSanitizer: detected memory leaks: 8 byte')):
            report = copy.deepcopy(self.report)
            path = self.root / ('wrong-' + str(index))
            path.write_text(text)
            report['records'][index]['diagnostics'] = [dict(path=str(path), sha256=sha(path))]
            with self.subTest(index=index), self.assertRaises(ValueError):
                validate_capture(report)

    def test_success_flag_cannot_replace_missing_positive_output(self):
        self.report['records'][2]['diagnostics'] = []
        with self.assertRaises(ValueError):
            validate_capture(self.report)


if __name__ == '__main__':
    unittest.main()
