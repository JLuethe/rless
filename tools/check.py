#!/usr/bin/env python3
"""Build and verify the native pager, saving a complete report in compat/.runs/."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

from release import source_identity, sha_file

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--stress-rounds', type=int, default=20)
    parser.add_argument('--install-check', action='store_true')
    parser.add_argument('--release', action='store_true',
                        help='Require committed source, reference, install checks and at least 100 stress rounds')
    args = parser.parse_args()
    if sys.flags.optimize:
        parser.error('verification requires Python assertions enabled')
    if args.stress_rounds < 1:
        parser.error('--stress-rounds must be positive')
    if args.release:
        if not args.reference:
            parser.error('--release requires --reference')
        args.install_check = True
        args.stress_rounds = max(100, args.stress_rounds)
    os.umask(0o077)
    (ROOT/'compat/.runs').mkdir(parents=True, exist_ok=True)
    directory = Path(tempfile.mkdtemp(prefix='verification-', dir=ROOT/'compat/.runs'))
    print(f'Verification reports: {directory}', flush=True)
    report = dict(schema_version=2, started_at_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  passed=False, steps=[], binaries={}, evidence_files={},
                  selection=dict(reference=bool(args.reference), install_check=args.install_check,
                                 stress_rounds=args.stress_rounds), release_prerequisites_complete=False)

    def save():
        (directory/'summary.json').write_text(json.dumps(report, indent=2)+'\n')

    def run(name, command, expected_returncode=0):
        print(f'{name}...', flush=True)
        with (directory/f'{name}.log').open('w') as log:
            result = subprocess.run([str(arg) for arg in command], cwd=ROOT,
                                    stdout=log, stderr=subprocess.STDOUT)
        report['steps'].append(dict(name=name, returncode=result.returncode,
                                   expected_returncode=expected_returncode,
                                   command=[str(arg) for arg in command], cwd=str(ROOT),
                                   passed=result.returncode == expected_returncode))
        report['evidence_files'][name + '.log'] = sha_file(directory / (name + '.log'))
        save()
        if result.returncode != expected_returncode:
            print((directory/f'{name}.log').read_text()[-6000:], file=sys.stderr)
            raise RuntimeError(f'{name} failed; see {directory/name}.log')

    python = sys.executable
    try:
        report['source'] = source_identity(ROOT, require_clean=args.release)
        report['build_and_test_inputs_sha256'] = report['source']['manifest_sha256']
        save()
        run('format', ['cargo', 'fmt', '--all', '--', '--check'])
        run('rust-debug', ['cargo', 'test', '--locked'])
        run('rust-release', ['cargo', 'test', '--release', '--locked'])
        run('observer', [python, 'tools/compat/test_runner.py'])
        run('corpus', [python, 'tools/compat/validate.py', '--output', directory/'coverage.json'])
        run('negative-candidate', [python, 'tools/compat/run_v2.py', 'replay', '--binary', '/bin/cat',
            '--filter', 'version', '--cases', 'compat/cases/options.json',
            '--expected', 'compat/expected-options.json', '--results', directory/'negative.json'], 1)
        negative = json.loads((directory/'negative.json').read_text())
        if negative['executed'] != 2 or negative['failed'] != 2 or negative['errors']:
            raise RuntimeError('negative candidate was not rejected on exactly the two selected cases')
        for profile in ('debug', 'release'):
            binary = ROOT/f'target/{profile}/rless'
            run(f'acceptance-{profile}', [python, 'tools/compat/check_candidate.py',
                '--binary', binary, '--results-dir', directory/f'acceptance-{profile}'])
            report['binaries'][profile] = dict(path=str(binary), sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                companion_sha256=hashlib.sha256((binary.parent/'lessecho').read_bytes()).hexdigest())
        if args.reference:
            reference = args.reference.resolve(strict=True)
            report['reference'] = dict(path=str(reference), sha256=sha_file(reference))
            run('reference', [python, 'tools/compat/check_candidate.py', '--binary', reference,
                             '--results-dir', directory/'reference'])
            for profile in ('debug', 'release'):
                binary = ROOT/f'target/{profile}/rless'
                run(f'integrations-{profile}', [python, 'tools/compat/test_integrations.py',
                    '--binary', binary, '--reference', reference, '--results', directory/f'integrations-{profile}.json'])
                run(f'job-control-{profile}', [python, 'tools/compat/test_suspend.py',
                    '--binary', binary, '--reference', reference, '--results', directory/f'job-control-{profile}.json'])
                run(f'helpers-{profile}', [python, 'tools/compat/test_helpers.py',
                    '--binary', binary, '--reference', reference, '--results', directory/f'helpers-{profile}.json'])
                for seed in (712, 20261023):
                    run(f'stress-{profile}-{seed}', [python, 'tools/compat/stress.py',
                        '--binary', binary, '--reference', reference, '--rounds', args.stress_rounds,
                        '--seed', seed, '--results', directory/f'stress-{profile}-{seed}.json'])
        else:
            report['reference_checks'] = 'Not requested; supply --reference for differential integration/stress tests.'
        if args.install_check:
            run('clean-install', ['cargo', 'install', '--path', ROOT, '--locked', '--offline',
                '--target-dir', directory/'clean-target', '--root', directory/'installed'])
            run('acceptance-installed', [python, 'tools/compat/check_candidate.py',
                '--binary', directory/'installed/bin/rless', '--results-dir', directory/'acceptance-installed'])
            report['installed'] = dict(sha256=sha_file(directory/'installed/bin/rless'),
                                       companion_sha256=sha_file(directory/'installed/bin/lessecho'))
        current = source_identity(ROOT, require_clean=args.release)
        if any(current.get(field) != report['source'].get(field)
               for field in ('source_commit', 'manifest_sha256', 'files', 'clean', 'dirty_state')):
            raise RuntimeError('build/test inputs changed during verification; rerun against a stable tree')
        helper = ROOT/'vendor/less-helpers/less-osc8-open'
        report['helpers'] = dict(osc8_sha256=hashlib.sha256(helper.read_bytes()).hexdigest())
        evidence_names = ['negative.json', 'coverage.json', 'acceptance-debug/summary.json', 'acceptance-release/summary.json']
        if args.reference:
            evidence_names += ['reference/summary.json']
            evidence_names += [f'{lane}-{profile}.json' for profile in ('debug', 'release')
                               for lane in ('integrations', 'job-control', 'helpers')]
            evidence_names += [f'stress-{profile}-{seed}.json' for profile in ('debug', 'release') for seed in (712, 20261023)]
        if args.install_check:
            evidence_names += ['acceptance-installed/summary.json']
        report['evidence_files'].update({name: sha_file(directory / name) for name in evidence_names})
        report['release_prerequisites_complete'] = bool(current['clean'] and args.reference and
                                                       args.install_check and args.stress_rounds >= 100)
        report['passed'] = True
    except (RuntimeError, OSError, ValueError, subprocess.SubprocessError) as error:
        report['error'] = str(error)
        print(error, file=sys.stderr)
    report['finished_at_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    save()
    print(f'Passed: {report["passed"]}; summary: {directory/"summary.json"}', flush=True)
    return not report['passed']


if __name__ == '__main__':
    raise SystemExit(main())
