"""M0 gate controls. Fixtures are synthetic; no fixture is product evidence."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from common import Evidence, fingerprint, sha, write_json
from coverage import matrix
import m0_gate


class M0GateControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.directory = Path(self.temporary.name)
        self.root = self.directory / 'repo'
        (self.root / 'src').mkdir(parents=True)
        (self.root / 'compat').mkdir()
        (self.root / 'spec/safety').mkdir(parents=True)
        (self.root / 'docs/safety').mkdir(parents=True)
        write_json(self.root / 'docs/safety/findings.json', {'findings': []})
        write_json(self.root / 'docs/safety/ownership-register.json', {'regions': []})
        (self.root / 'src/fault.rs').write_text('\n'.join(
            f'b"{site}" => Site::Fixture,' for site in sorted(m0_gate.MINIMUM_SITES)))
        (self.root / 'compat/corpus.json').write_text('{}')
        self.catalogue = dict(profiles={'NATIVE': 'native'}, scenarios=[dict(
            id='CASE-FIXTURE', variants=['future production acceptance'], required_profiles=['NATIVE'],
            requirement_ids=['SAFE-VER-001'])])
        write_json(self.root / 'spec/safety/acceptance.json', self.catalogue)
        write_json(self.root / 'spec/safety/work-packages.json',
                   {'work_packages': [{'id': 'M1.1'}, {'id': 'M3.2'}]})
        subprocess.run(['git', 'init', '-q', self.root], check=True)
        subprocess.run(['git', '-C', self.root, 'add', '.'], check=True)
        subprocess.run(['git', '-C', self.root, '-c', 'user.name=Gate Fixture', '-c',
                        'user.email=fixture@invalid', 'commit', '-qm', 'fixture'], check=True)
        self.note = self.directory / 'independent-review.txt'
        self.note.write_text('Independent fixture review of all fixture controls and applicability.')
        self.evidence = Evidence(self.directory / 'evidence', self.root)
        self.required = m0_gate.obligations(self.root)
        controls = sorted({value['control'] for value in self.required.values()} |
                          {'termination-cleanup', 'natural-error-counterpart'})
        contexts = sorted({(value['profile'], value['abi_profile']) for value in self.required.values()})
        self.names = {}
        for index, (profile, abi) in enumerate(contexts):
            name = f'fixture-{index}'
            self.names[(profile, abi)] = name
            self.evidence.run(name, [sys.executable, '-c', 'print("fixture oracle")'], profile=profile,
                oracle_metadata=dict(abi_profile=abi, corpus_sha256=sha(self.root / 'compat/corpus.json'),
                    seed=712, test_selector='Fixture.test_controls', hook_sites=sorted(m0_gate.MINIMUM_SITES),
                    hook_ordinals=[1, 2], m0_controls=controls, executed_cases=809, passed_cases=809,
                    failed_cases=0))
        self.evidence.finish()
        self.report = self.evidence.directory / 'summary.json'
        own = {}
        for name, data in [('inventory', {'schema_version': 2}), ('regions', {'regions': []}),
                           ('review', {'entries': []}), ('findings', {'findings': []})]:
            path = {'findings': self.root / 'docs/safety/findings.json',
                    'regions': self.root / 'docs/safety/ownership-register.json'}.get(
                        name, self.directory / (name + '.json'))
            write_json(path, data)
            own[name] = self.ref(path)
        bindings = self.directory / 'bindings.json'
        write_json(bindings, {'bindings': []})
        current = fingerprint(self.root)
        self.candidate = dict(schema_version=1, policy_version=m0_gate.POLICY_VERSION,
            source_commit=self.evidence.report['source_commit'], input_fingerprints=current,
            ownership=own, reports=[self.ref(self.report)], selectors={}, applicability={},
            bindings=self.ref(bindings), row_dispositions={key: dict(status='not-run',
                delivery_packet='M1.1', reason='Final application migration follows M0.')
                for key in matrix(self.catalogue)}, legacy_dispositions=[],
            independent_review=dict(reviewer='Independent Fixture Reviewer',
                implementation_author='Fixture Author', source_files_sha256=current['files'],
                evidence=self.ref(self.note), decision='approve-m0-infrastructure'))
        for key, value in self.required.items():
            self.candidate['selectors'][key] = dict(report=str(self.report),
                record_name=self.names[(value['profile'], value['abi_profile'])])
        # Ownership's own tests exercise actual inventories/ledgers. Here isolate
        # gate composition, including that pending ownership is not suppressed.
        self.ownership_patch = patch('m0_gate.validate_ownership', return_value=dict(
            status='reviewed', pending=0, runtime_safety_verified=False))
        self.ownership = self.ownership_patch.start()
        # Marker parsing/current-contract validation is tested by the fault
        # oracle suite. Supply normalized fixture observations here to exercise
        # gate composition independently of the real pager's contracts.
        self.real_validate_evidence = m0_gate.validated_evidence
        self.capture_validated = True
        self.controls_validated = True
        self.observed = {name: [dict(site=site, ordinal=ordinal, magic=magic)
            for site in sorted(m0_gate.MINIMUM_SITES)
            for magic, ordinal in (('RLF1', 1), ('RLF1', 2), ('RNT1', 1))]
            for name in self.names.values()}
        self.evidence_patch = patch('m0_gate.validated_evidence', side_effect=self.fixture_evidence)
        self.evidence_patch.start()
        self.seal_fixture_review()

    def tearDown(self):
        self.evidence_patch.stop()
        self.ownership_patch.stop()
        self.temporary.cleanup()

    def ref(self, path):
        return dict(path=str(path), sha256=sha(path))

    def fixture_evidence(self, path, root):
        report = self.real_validate_evidence(path, root)
        for item in report['records']:
            item['oracle_report'] = self.ref(self.note)
            item['observed_hooks'] = copy.deepcopy(self.observed.get(item['name'], []))
            item['_validated_detector_capture'] = self.capture_validated
            item['_validated_m0_controls'] = (
                (item.get('oracle_metadata') or {}).get('m0_controls', []).copy()
                if self.controls_validated else [])
        return report

    def check(self):
        return m0_gate.validate_candidate(self.candidate, self.root)

    def refresh_report(self):
        self.evidence.save()
        self.candidate['reports'] = [self.ref(self.report)]
        self.seal_fixture_review()

    def seal_fixture_review(self):
        # A synthetic reviewer approves this fixture's exact inputs only.
        self.candidate['independent_review']['candidate_inputs_sha256'] = m0_gate.candidate_inputs_sha256(self.candidate)

    def test_review_digest_rejects_post_review_equivalent_selector_or_findings_ref(self):
        original = copy.deepcopy(self.candidate)
        key = next(iter(self.candidate['selectors']))
        self.candidate['selectors'][key] = [self.candidate['selectors'][key]]
        with self.assertRaisesRegex(ValueError, 'current candidate receipt'):
            self.check()
        self.candidate = original
        self.candidate['ownership']['findings']['path'] = 'docs/safety/findings.json'
        with self.assertRaisesRegex(ValueError, 'current candidate receipt'):
            self.check()

    def test_review_digest_is_required(self):
        del self.candidate['independent_review']['candidate_inputs_sha256']
        with self.assertRaisesRegex(ValueError, 'current candidate receipt'):
            self.check()

    def rebind_fixture_inputs(self):
        # Synthetic composition fixtures only; real execution receipts must
        # never be rewritten to conceal changed inputs.
        current = fingerprint(self.root)
        self.candidate['input_fingerprints'] = current
        self.candidate['independent_review']['source_files_sha256'] = current['files']
        self.evidence.report['input_fingerprints'] = current
        self.refresh_report()

    def test_substitute_findings_file_cannot_hide_authoritative_blockers(self):
        authoritative = self.root / 'docs/safety/findings.json'
        write_json(authoritative, {'findings': [dict(id='M0-COV-001', status='open',
            delivery_packet='M0.1', blocks_final_safety_gate=True)]})
        self.rebind_fixture_inputs()
        alternate = self.directory / 'omitted-findings.json'
        write_json(alternate, {'findings': []})
        self.candidate['ownership']['findings'] = self.ref(alternate)
        with self.assertRaisesRegex(ValueError, 'authoritative maintained register'):
            self.check()

    def test_maintained_findings_change_invalidates_existing_receipts(self):
        authoritative = self.root / 'docs/safety/findings.json'
        self.assertIn('docs/safety/findings.json', self.candidate['input_fingerprints']['files'])
        write_json(authoritative, {'findings': [dict(id='M0-NEW', status='open')]})
        self.candidate['ownership']['findings'] = self.ref(authoritative)
        with self.assertRaisesRegex(ValueError, 'inputs changed'):
            self.check()

    def test_substitute_region_register_cannot_drop_owner_cycles(self):
        alternate = self.directory / 'omitted-regions.json'
        write_json(alternate, {'regions': []})
        self.candidate['ownership']['regions'] = self.ref(alternate)
        with self.assertRaisesRegex(ValueError, 'ownership regions must use the authoritative'):
            self.check()

    def test_region_register_change_invalidates_existing_receipts(self):
        authoritative = self.root / 'docs/safety/ownership-register.json'
        self.assertIn('docs/safety/ownership-register.json', self.candidate['input_fingerprints']['files'])
        write_json(authoritative, {'regions': [{'id': 'OWN-NEW'}]})
        self.candidate['ownership']['regions'] = self.ref(authoritative)
        with self.assertRaisesRegex(ValueError, 'inputs changed'):
            self.check()

    def test_m0_positive_fixture_does_not_close_final_acceptance(self):
        result = self.check()
        self.assertEqual(result['status'], 'm0-verified')
        self.assertFalse(result['runtime_safety_verified'])
        self.assertFalse(result['m5_verified'])
        self.assertEqual(result['final_rows_passed'], 0)
        self.assertTrue(self.ownership.call_args.kwargs['complete'])

    def test_pending_ownership_prevents_m0_closure(self):
        self.ownership.side_effect = ValueError('687 review entries remain incomplete')
        with self.assertRaisesRegex(ValueError, '687'):
            self.check()

    def test_removed_hook_or_abi_obligation_rejects(self):
        for key in ('hook:alloc:fired:REFERENCE-GLIBC-2.41:ASAN',
                    'acceptance-source-rebuild:REFERENCE-GLIBC-2.41'):
            saved = self.candidate['selectors'].pop(key)
            with self.assertRaisesRegex(ValueError, 'M0 obligations'):
                self.check()
            self.candidate['selectors'][key] = saved

    def test_declared_hook_coverage_without_observations_rejects(self):
        def without_markers(path, root):
            report = self.fixture_evidence(path, root)
            for item in report['records']:
                item.pop('oracle_report')
                item.pop('observed_hooks')
            return report
        with patch('m0_gate.validated_evidence', side_effect=without_markers):
            with self.assertRaisesRegex(ValueError, 'not metadata alone'):
                self.check()

    def test_claiming_two_ordinals_does_not_supply_two_observed_events(self):
        name = self.names[('NATIVE', 'REFERENCE-GLIBC-2.41')]
        self.observed[name] = [event for event in self.observed[name]
                               if not (event['site'] == 'alloc' and event['ordinal'] == 2)]
        with self.assertRaisesRegex(ValueError, 'two actual observed ordinals'):
            self.check()

    def test_capture_metadata_cannot_substitute_for_retained_diagnostics(self):
        self.capture_validated = False
        with self.assertRaisesRegex(ValueError, 'revalidated pipe/PTY diagnostics'):
            self.check()

    def test_common_control_metadata_without_derived_proof_rejects(self):
        self.controls_validated = False
        with self.assertRaisesRegex(ValueError, 'rederived command/probe evidence'):
            self.check()

    def test_native_counterpart_requires_a_native_marker_at_that_site(self):
        name = self.names[('NATIVE', 'REFERENCE-GLIBC-2.41')]
        self.observed[name] = [event for event in self.observed[name]
                               if not (event['site'] == 'alloc' and event['magic'] == 'RNT1')]
        with self.assertRaisesRegex(ValueError, 'matching RNT1'):
            self.check()

    def test_distinct_observed_ordinals_can_come_from_two_oracles(self):
        context = ('NATIVE', 'REFERENCE-GLIBC-2.41')
        first = self.names[context]
        original = next(item for item in self.evidence.report['records'] if item['name'] == first)
        metadata = copy.deepcopy(original['oracle_metadata'])
        metadata['hook_ordinals'] = [2]
        metadata['m0_controls'] = ['fired']
        self.evidence.run('second-ordinal', [sys.executable, '-c', 'print("ordinal 2")'],
                          profile=context[0], oracle_metadata=metadata)
        self.observed['second-ordinal'] = [dict(site='alloc', ordinal=2, magic='RLF1')]
        self.observed[first] = [event for event in self.observed[first]
                                if not (event['site'] == 'alloc' and event['ordinal'] == 2)]
        key = 'hook:alloc:ordinal-separation:REFERENCE-GLIBC-2.41:NATIVE'
        self.candidate['selectors'][key] = [self.candidate['selectors'][key],
            dict(report=str(self.report), record_name='second-ordinal')]
        self.refresh_report()
        self.assertEqual(self.check()['status'], 'm0-verified')

    def test_host_evidence_cannot_supply_reference_abi(self):
        self.candidate['selectors']['acceptance-debug:REFERENCE-GLIBC-2.41']['record_name'] = \
            self.names[('NATIVE', 'BUILDER-GLIBC-2.44')]
        with self.assertRaisesRegex(ValueError, 'wrong profile or ABI'):
            self.check()

    def test_extra_unsupported_record_cannot_hide_behind_passed_selectors(self):
        self.evidence.unavailable('missing-control', 'required native counterpart missing', 'NATIVE')
        self.refresh_report()
        with self.assertRaisesRegex(ValueError, 'required record did not pass'):
            self.check()

    def test_missing_log_and_changed_metadata_cannot_pass(self):
        item = self.evidence.report['records'][0]
        Path(item['log']).unlink()
        with self.assertRaisesRegex(ValueError, 'diagnostic log'):
            self.check()

    def test_partial_corpus_cannot_be_called_full_replay(self):
        for item in self.evidence.report['records']:
            if item['profile'] == 'NATIVE':
                item['oracle_metadata']['passed_cases'] = 808
        self.refresh_report()
        with self.assertRaisesRegex(ValueError, 'full frozen corpus'):
            self.check()

    def test_boolean_failed_case_count_is_not_zero_executed_failures(self):
        for item in self.evidence.report['records']:
            if item['profile'] == 'NATIVE':
                item['oracle_metadata']['failed_cases'] = False
        self.refresh_report()
        with self.assertRaisesRegex(ValueError, 'full frozen corpus'):
            self.check()

    def test_applicability_needs_review_and_executed_alternate(self):
        key = 'hook:alloc:resource-conservation:REFERENCE-GLIBC-2.41:NATIVE'
        original = self.candidate['selectors'].pop(key)
        self.candidate['applicability'][key] = dict(
            decision='alternate-required', rationale='Fixture allocation failure terminates its process.',
            reviewer='Independent Fixture Reviewer', implementation_author='Fixture Author',
            source_files_sha256={'src/fault.rs': sha(self.root / 'src/fault.rs')},
            evidence=self.ref(self.note), alternative_control='termination-cleanup',
            alternative_selectors=[original])
        self.seal_fixture_review()
        self.assertEqual(self.check()['alternate_obligations'], 1)
        self.candidate['applicability'][key]['alternative_selectors'] = []
        with self.assertRaisesRegex(ValueError, 'executed alternate'):
            self.check()

    def test_termination_alternate_needs_a_matching_site_marker(self):
        key = 'hook:alloc:resource-conservation:REFERENCE-GLIBC-2.41:NATIVE'
        original = self.candidate['selectors'].pop(key)
        self.candidate['applicability'][key] = dict(
            decision='alternate-required', rationale='Fixture allocation failure terminates its process.',
            reviewer='Independent Fixture Reviewer', implementation_author='Fixture Author',
            source_files_sha256={'src/fault.rs': sha(self.root / 'src/fault.rs')},
            evidence=self.ref(self.note), alternative_control='termination-cleanup',
            alternative_selectors=[original])
        name = original['record_name']
        # Keep fired/ordinal obligations independently supplied by another
        # record while removing only this alternate's matching termination hit.
        duplicate = copy.deepcopy(next(item for item in self.evidence.report['records'] if item['name'] == name))
        duplicate['name'] = 'termination-without-matching-hit'
        self.evidence.report['records'].append(duplicate)
        self.observed[duplicate['name']] = [event for event in self.observed[name] if event['site'] != 'alloc']
        self.candidate['applicability'][key]['alternative_selectors'] = [
            dict(report=str(self.report), record_name=duplicate['name'])]
        self.refresh_report()
        with self.assertRaisesRegex(ValueError, 'matching RLF1'):
            self.check()

    def test_fired_fault_evidence_cannot_be_waived(self):
        key = 'hook:alloc:fired:REFERENCE-GLIBC-2.41:NATIVE'
        original = self.candidate['selectors'].pop(key)
        self.candidate['applicability'][key] = dict(alternative_selectors=[original])
        with self.assertRaisesRegex(ValueError, 'cannot be waived'):
            self.check()

    def test_legacy_diagnostic_needs_live_source_bound_disposition(self):
        self.evidence.run('legacy', [sys.executable, '-c', 'print("legacy behavior observed")'],
            finding_ids=['M0-IO-001'])
        self.refresh_report()
        with self.assertRaisesRegex(ValueError, 'disposition missing'):
            self.check()
        finding_path = Path(self.candidate['ownership']['findings']['path'])
        write_json(finding_path, {'findings': [dict(id='M0-IO-001', status='open',
            delivery_packet='M3.2', blocks_final_safety_gate=True)]})
        self.candidate['ownership']['findings'] = self.ref(finding_path)
        self.rebind_fixture_inputs()
        self.candidate['legacy_dispositions'] = [dict(finding_id='M0-IO-001', owner='Codex',
            delivery_packet='M3.2', reason='Inherited reference behavior reproduced.',
            regression='Fixture.test_native_read_error',
            source_files_sha256={'src/fault.rs': sha(self.root / 'src/fault.rs')}, evidence=self.ref(self.note))]
        self.seal_fixture_review()
        self.assertEqual(self.check()['legacy_findings'], ['M0-IO-001'])
        self.candidate['legacy_dispositions'][0]['reason'] = 'Changed after review.'
        with self.assertRaisesRegex(ValueError, 'current candidate receipt'):
            self.check()
        finding = json.loads(finding_path.read_text())
        finding['findings'][0]['delivery_packet'] = 'M0.4'
        write_json(finding_path, finding)
        self.candidate['ownership']['findings'] = self.ref(finding_path)
        self.rebind_fixture_inputs()
        self.candidate['legacy_dispositions'][0]['delivery_packet'] = 'M0.4'
        with self.assertRaisesRegex(ValueError, 'infrastructure gap'):
            self.check()

    def test_changed_source_and_self_approval_reject(self):
        self.candidate['independent_review']['reviewer'] = 'Fixture Author'
        with self.assertRaisesRegex(ValueError, 'independent approval'):
            self.check()
        self.candidate['independent_review']['reviewer'] = 'Independent Fixture Reviewer'
        (self.root / 'src/fault.rs').write_text('changed')
        with self.assertRaisesRegex(ValueError, 'inputs changed'):
            self.check()

    def test_every_final_row_needs_honest_disposition(self):
        self.candidate['row_dispositions'].clear()
        with self.assertRaisesRegex(ValueError, 'every unbound final row'):
            self.check()


if __name__ == '__main__':
    unittest.main()
