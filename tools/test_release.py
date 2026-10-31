"""Release provenance and policy controls; no historical pass is release approval."""
import base64
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import package
from release import (FAMILIES, PROFILES, PROVENANCE, REQUIRED_STEPS, SEEDS, WORKING_PROVENANCE,
                     HELPER_ERRORS, HELPER_INVOCATIONS, HELPER_URIS, INTEGRATIONS,
                     commit_manifest, file_identity, source_identity, validate_verification,
                     write_export_manifest)


class ReleaseControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        self.root = self.directory / 'repo'
        self.root.mkdir()
        files = {'.gitignore': 'target/\n/dist/\n__pycache__/\ndocs/private.md\n',
                 'Cargo.toml': '[package]\nname="fixture"\nversion="0.1.0"\n',
                 'README.md': 'Source fixture\n', 'src/lib.rs': 'pub fn value() {}\n',
                 'tests/boundary.rs': '// acceptance input\n',
                 'compat/cases/options.json': '{}\n', 'compat/expected-options.json': '{}\n',
                 'tools/compat/run_v2.py': '# fixture observer\n', 'compat/observer-v2.json': '{}\n',
                 'vendor/helper/run.sh': '#!/bin/sh\nexit 0\n'}
        for name, content in files.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        self.git('init', '-q')
        self.git('add', '.')
        self.git('-c', 'user.name=Release Fixture', '-c', 'user.email=fixture@invalid', 'commit', '-qm', 'fixture')
        self.source = source_identity(self.root, require_clean=True)
        self.evidence = self.directory / 'evidence'
        self.evidence.mkdir()
        self.report_path = self.evidence / 'summary.json'

    def tearDown(self):
        self.temp.cleanup()

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.STDOUT)

    def write_evidence(self, record, name, value):
        path = self.evidence / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value if isinstance(value, str) else json.dumps(value))
        record['evidence_files'][name] = hashlib.sha256(path.read_bytes()).hexdigest()

    def passing_report(self):
        digest = 'a' * 64
        record = dict(schema_version=2, passed=True, source=self.source,
            selection=dict(reference=True, install_check=True, stress_rounds=100),
            reference=dict(sha256=digest), helpers=dict(osc8_sha256=digest),
            binaries={profile: dict(sha256=digest, companion_sha256=digest) for profile in PROFILES},
            installed=dict(sha256=digest, companion_sha256=digest), evidence_files={}, steps=[])
        for name in sorted(REQUIRED_STEPS):
            status = 1 if name == 'negative-candidate' else 0
            record['steps'].append(dict(name=name, returncode=status, expected_returncode=status, passed=True))
            self.write_evidence(record, name + '.log', 'Fixture executed oracle\n')
        self.write_evidence(record, 'negative.json', dict(executed=2, passed=0, failed=2, errors=[]))
        self.write_evidence(record, 'coverage.json', dict(schema_version=1, acceptance_cases=809,
            reference_expectations=809, required_interfaces=1, required_interfaces_with_cases=1))
        acceptance = dict(executed=809, passed=809, failed=0, sha256=digest,
            observer_sha256=self.source['files']['tools/compat/run_v2.py']['sha256'],
            observer_revision_sha256=self.source['files']['compat/observer-v2.json']['sha256'],
            families={name: dict(executed=count, passed=count, failed=0, errors=[]) for name, count in FAMILIES.items()})
        for lane in ('acceptance-debug', 'acceptance-release', 'reference', 'acceptance-installed'):
            self.write_evidence(record, lane + '/summary.json', acceptance)
        def terminal(row='Fixture observation', modes=None):
            return dict(rows=[row], styles=[], cursor=[0, 0, False], private_modes=modes or [],
                        modes=[], bells=0, osc=[])
        def pty(status, row='Fixture observation'):
            return dict(status=status, termios_restored=True, final_terminal=terminal(), checkpoints=[terminal(row)])
        for profile in PROFILES:
            integrations = {name: dict(passed=True, actual=pty(status)) for name, status in INTEGRATIONS.items()}
            for name in ('git-diff', 'man-local'):
                integrations[name]['actual']['files'] = {'pager.log': dict(sha256=hashlib.sha256(b'pager\n').hexdigest())}
            integrations['repeated-resize']['actual']['checkpoints'] = [terminal(
                'line 002' if index % 3 == 1 else 'line 001') for index in range(60)]
            for name, rows in [('follow-append', ['APPENDED-TWO']), ('follow-rotate', ['ROTATED-TWO']),
                    ('truncate-recovery', ['Cannot seek', 'TRUNCATED-TWO']), ('follow-live-pipe', ['PIPE-APPEND']),
                    ('sparse-byte-position', ['beyond-4-GiB'])]:
                integrations[name]['actual']['checkpoints'] = [terminal(row) for row in rows]
            self.write_evidence(record, f'integrations-{profile}.json', integrations)
            helpers = {'companion-glob': dict(passed=True, actual=pty(0, 'FIRST MATCH')),
                       'helper_sha256': digest}
            for index, uri in enumerate(HELPER_URIS):
                files = {name: {} for name in ('bin/less', 'bin/rless', 'bin/man')}
                if index in HELPER_INVOCATIONS:
                    invocation = HELPER_INVOCATIONS[index]
                    files['invocation.log'] = dict(sha256=hashlib.sha256(invocation).hexdigest(), bytes=len(invocation))
                helpers[f'osc8-helper-{index}'] = dict(passed=True, uri=uri, actual=dict(
                    status=1 if index in HELPER_ERRORS else 0, files=files, stderr_b64='',
                    stdout_b64=base64.b64encode(HELPER_ERRORS.get(index, b'')).decode()))
            self.write_evidence(record, f'helpers-{profile}.json', helpers)
            job = dict(stopped=dict(stopped_by=20, restored=True), exited=dict(status=0, restored=True),
                       stopped_terminal=terminal(), resumed_terminal=terminal(modes=[1049]))
            self.write_evidence(record, f'job-control-{profile}.json', dict(passed=True, actual=job, reference=job))
            for seed in SEEDS:
                self.write_evidence(record, f'stress-{profile}-{seed}.json', dict(seed=seed, rounds=100,
                    executed=708, passed=708, failed=0, failures=[], binary_sha256=digest, reference_sha256=digest))
        self.report_path.write_text(json.dumps(record))
        return record

    def test_source_manifest_covers_tests_and_corpus_and_rejects_dirty_input(self):
        self.assertIn('tests/boundary.rs', self.source['files'])
        self.assertIn('compat/cases/options.json', self.source['files'])
        self.assertIn('compat/expected-options.json', self.source['files'])
        (self.root / 'tests/boundary.rs').write_text('// changed test\n')
        self.assertFalse(source_identity(self.root)['clean'])
        with self.assertRaisesRegex(ValueError, 'clean committed'):
            source_identity(self.root, require_clean=True)

    def test_nested_builds_are_excluded_but_ignored_private_notes_reject_release(self):
        output = self.root / 'vendor/helper/target/debug/generated.rs'
        output.parent.mkdir(parents=True)
        output.write_text('generated\n')
        self.assertEqual(source_identity(self.root, require_clean=True)['manifest_sha256'], self.source['manifest_sha256'])
        notes = self.root / 'docs/private.md'
        notes.parent.mkdir()
        notes.write_text('must never be packaged\n')
        with self.assertRaisesRegex(ValueError, 'unapproved/untracked'):
            source_identity(self.root, require_clean=True)

    def test_source_archive_contains_exact_committed_files_and_portable_provenance(self):
        archive = self.directory / 'candidate.tar.gz'
        package.source_archive_bytes(self.root, self.source, 'fixture-source', archive, 0)
        with tarfile.open(archive) as tar:
            self.assertEqual({member.name for member in tar.getmembers()},
                             {'fixture-source/' + name for name in self.source['files']} | {'fixture-source/' + PROVENANCE})
            tar.extractall(self.directory / 'extracted', filter='data')
        extracted = self.directory / 'extracted/fixture-source'
        self.assertEqual(source_identity(extracted, require_clean=True)['source_commit'], self.source['source_commit'])
        (extracted / 'tests/boundary.rs').write_text('changed\n')
        with self.assertRaisesRegex(ValueError, 'changed since committed export'):
            source_identity(extracted, require_clean=True)

    def test_exporter_rejects_unapproved_tracked_build_output(self):
        path = self.root / 'vendor/helper/target/private.txt'
        path.parent.mkdir(parents=True)
        path.write_text('not source\n')
        self.git('add', '-f', 'vendor/helper/target/private.txt')
        self.git('-c', 'user.name=Release Fixture', '-c', 'user.email=fixture@invalid', 'commit', '-qm', 'bad output')
        with self.assertRaisesRegex(ValueError, 'unapproved tracked'):
            commit_manifest(self.root)

    def test_working_snapshot_can_be_checked_but_never_released(self):
        exported = self.directory / 'working'
        exported.mkdir()
        for name in self.source['files']:
            path = exported / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((self.root / name).read_bytes())
        files = {name: file_identity(exported / name) for name in self.source['files']}
        manifest = dict(schema_version=1, source_kind='explicit-working-tree-snapshot',
            base_commit=self.source['source_commit'], dirty_state=' M src/lib.rs\n', files=files,
            files_sha256=hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest())
        (exported / WORKING_PROVENANCE).write_text(json.dumps(manifest))
        self.assertFalse(source_identity(exported)['clean'])
        with self.assertRaisesRegex(ValueError, 'cannot qualify a release'):
            source_identity(exported, require_clean=True)

    def test_complete_native_report_positive_and_partial_report_negative(self):
        report = self.passing_report()
        self.assertTrue(validate_verification(report, self.report_path, self.root)['clean'])
        for name in ('reference', 'clean-install', 'stress-release-712'):
            partial = copy.deepcopy(report)
            partial['steps'] = [step for step in partial['steps'] if step['name'] != name]
            with self.assertRaisesRegex(ValueError, 'verification steps'):
                validate_verification(partial, self.report_path, self.root)
        short = copy.deepcopy(report)
        short['selection']['stress_rounds'] = 20
        with self.assertRaisesRegex(ValueError, '100 stress rounds'):
            validate_verification(short, self.report_path, self.root)

    def test_changed_or_missing_acceptance_observer_identity_is_rejected(self):
        for lane in ('acceptance-debug', 'acceptance-release', 'reference', 'acceptance-installed'):
            for field in ('observer_sha256', 'observer_revision_sha256'):
                for replacement in (None, '0' * 64):
                    with self.subTest(lane=lane, field=field, replacement=replacement):
                        report = self.passing_report()
                        name = lane + '/summary.json'
                        data = json.loads((self.evidence / name).read_text())
                        if replacement is None:
                            data.pop(field)
                        else:
                            data[field] = replacement
                        self.write_evidence(report, name, data)
                        with self.assertRaisesRegex(ValueError, 'observer identity'):
                            validate_verification(report, self.report_path, self.root)

    def test_changed_logs_and_incomplete_acceptance_cannot_be_promoted(self):
        report = self.passing_report()
        (self.evidence / 'rust-debug.log').write_text('changed\n')
        with self.assertRaisesRegex(ValueError, 'stale verification evidence'):
            validate_verification(report, self.report_path, self.root)
        report = self.passing_report()
        bad = json.loads((self.evidence / 'acceptance-installed/summary.json').read_text())
        bad['families']['integration']['errors'] = ['observer failed']
        self.write_evidence(report, 'acceptance-installed/summary.json', bad)
        with self.assertRaisesRegex(ValueError, 'acceptance family'):
            validate_verification(report, self.report_path, self.root)

    def test_changed_source_commit_and_binary_identity_are_rejected(self):
        report = self.passing_report()
        report['source'] = dict(report['source'], source_commit='0' * 40)
        with self.assertRaisesRegex(ValueError, 'source identity'):
            validate_verification(report, self.report_path, self.root)
        report = self.passing_report()
        bad = json.loads((self.evidence / 'stress-debug-712.json').read_text())
        bad['binary_sha256'] = 'b' * 64
        self.write_evidence(report, 'stress-debug-712.json', bad)
        with self.assertRaisesRegex(ValueError, 'stress binary'):
            validate_verification(report, self.report_path, self.root)

    def test_missing_corpus_evidence_cannot_be_promoted(self):
        report = self.passing_report()
        del report['evidence_files']['coverage.json']
        with self.assertRaisesRegex(ValueError, 'stale verification evidence'):
            validate_verification(report, self.report_path, self.root)
        self.write_evidence(report, 'coverage.json', dict(schema_version=1, acceptance_cases=808,
            reference_expectations=809, required_interfaces=1, required_interfaces_with_cases=1))
        with self.assertRaisesRegex(ValueError, 'corpus coverage'):
            validate_verification(report, self.report_path, self.root)

    def test_boolean_counts_and_exit_statuses_are_not_numbers(self):
        for name in ('format', 'negative-candidate'):
            for field in ('returncode', 'expected_returncode'):
                with self.subTest(name=name, field=field):
                    report = self.passing_report()
                    step = next(step for step in report['steps'] if step['name'] == name)
                    step[field] = bool(step[field])
                    with self.assertRaisesRegex(ValueError, 'verification step'):
                        validate_verification(report, self.report_path, self.root)
        for filename, field in [('acceptance-debug/summary.json', 'failed'), ('negative.json', 'passed'),
                                ('stress-debug-712.json', 'failed'), ('coverage.json', 'schema_version'),
                                ('coverage.json', 'required_interfaces_with_cases')]:
            with self.subTest(filename=filename, field=field):
                report = self.passing_report()
                data = json.loads((self.evidence / filename).read_text())
                data[field] = bool(data[field])
                self.write_evidence(report, filename, data)
                with self.assertRaises(ValueError):
                    validate_verification(report, self.report_path, self.root)

    def test_case_renames_and_missing_observations_cannot_keep_pass_credit(self):
        for filename, case in [('integrations-debug.json', 'sparse-byte-position'),
                               ('helpers-debug.json', 'osc8-helper-5')]:
            for mutation in ('rename', 'omit-actual', 'boolean-status'):
                with self.subTest(filename=filename, mutation=mutation):
                    report = self.passing_report()
                    data = json.loads((self.evidence / filename).read_text())
                    if mutation == 'rename':
                        data['unrelated-passing-case'] = data.pop(case)
                    elif mutation == 'omit-actual':
                        del data[case]['actual']
                    else:
                        data[case]['actual']['status'] = bool(data[case]['actual']['status'])
                    self.write_evidence(report, filename, data)
                    with self.assertRaises(ValueError):
                        validate_verification(report, self.report_path, self.root)

    def test_helper_literal_arguments_and_uri_identity_are_checked(self):
        for mutation in ('uri', 'arguments', 'rejected-invoked'):
            with self.subTest(mutation=mutation):
                report = self.passing_report()
                data = json.loads((self.evidence / 'helpers-debug.json').read_text())
                if mutation == 'uri':
                    data['osc8-helper-5']['uri'] = 'file:///tmp/different'
                elif mutation == 'arguments':
                    data['osc8-helper-5']['actual']['files']['invocation.log']['sha256'] = '0' * 64
                else:
                    data['osc8-helper-3']['actual']['files']['invocation.log'] = dict(sha256='0' * 64, bytes=0)
                self.write_evidence(report, 'helpers-debug.json', data)
                with self.assertRaises(ValueError):
                    validate_verification(report, self.report_path, self.root)

    def test_integration_pass_flags_cannot_hide_missing_recovery_or_wrong_pager(self):
        for case in ('git-diff', 'repeated-resize', 'follow-rotate', 'truncate-recovery', 'sparse-byte-position'):
            with self.subTest(case=case):
                report = self.passing_report()
                data = json.loads((self.evidence / 'integrations-debug.json').read_text())
                if case == 'git-diff':
                    data[case]['actual']['files'] = {}
                else:
                    data[case]['actual']['checkpoints'] = []
                self.write_evidence(report, 'integrations-debug.json', data)
                with self.assertRaises(ValueError):
                    validate_verification(report, self.report_path, self.root)

    def test_job_control_cannot_pass_with_missing_different_or_failed_observations(self):
        for mutation in ('passed-only', 'different', 'failed-both', 'boolean-status', 'no-resume-mode'):
            with self.subTest(mutation=mutation):
                report = self.passing_report()
                data = json.loads((self.evidence / 'job-control-debug.json').read_text())
                if mutation == 'passed-only':
                    data = dict(passed=True)
                elif mutation == 'different':
                    data['actual']['stopped_terminal']['rows'] = ['Changed observation']
                else:
                    for side in ('actual', 'reference'):
                        if mutation == 'failed-both':
                            data[side]['stopped']['restored'] = False
                        elif mutation == 'boolean-status':
                            data[side]['exited']['status'] = False
                        else:
                            data[side]['resumed_terminal']['private_modes'] = []
                self.write_evidence(report, 'job-control-debug.json', data)
                with self.assertRaises(ValueError):
                    validate_verification(report, self.report_path, self.root)

    def test_failed_source_rebuild_has_no_success_attestation(self):
        archive = self.directory / 'candidate.tar.gz'
        package.source_archive_bytes(self.root, self.source, 'fixture-source', archive, 0)
        output = self.directory / 'qualification'
        def fail_build(command, **kwargs):
            return subprocess.CompletedProcess(command, 128 if command[0] == 'git' else 1, stdout='')
        with patch('package.subprocess.run', side_effect=fail_build) as runner:
            with self.assertRaisesRegex(ValueError, 'source rebuild failed'):
                package.qualify_source(archive, 'fixture-source', self.source, output)
        cargo = [call.args[0] for call in runner.call_args_list if call.args[0][0] == 'cargo']
        self.assertEqual(cargo, [['cargo', 'build', '--release', '--locked', '--offline', '--bins']])
        self.assertFalse(json.loads((output / 'result.json').read_text())['passed'])
        self.assertFalse(any(self.directory.glob('*-release.json')))


if __name__ == '__main__':
    unittest.main(verbosity=2)
