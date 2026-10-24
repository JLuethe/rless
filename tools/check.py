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

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--stress-rounds', type=int, default=20)
    parser.add_argument('--install-check', action='store_true')
    args = parser.parse_args()
    os.umask(0o077)
    (ROOT/'compat/.runs').mkdir(parents=True, exist_ok=True)
    directory = Path(tempfile.mkdtemp(prefix='verification-', dir=ROOT/'compat/.runs'))
    print(f'Verification reports: {directory}', flush=True)
    report = dict(started_at_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  passed=False, steps=[], binaries={})

    def fingerprint():
        paths = [ROOT/name for name in ('Cargo.toml', 'Cargo.lock', 'build.rs', 'compat/observer-v2.json')]
        for top in ('src', 'vendor', 'tools'):
            paths.extend(path for path in (ROOT/top).rglob('*')
                         if path.is_file() and '__pycache__' not in path.parts)
        digest = hashlib.sha256()
        for path in sorted(paths):
            digest.update(str(path.relative_to(ROOT)).encode()+b'\0'+path.read_bytes()+b'\0')
        return digest.hexdigest()

    report['build_and_test_inputs_sha256'] = fingerprint()

    def save():
        (directory/'summary.json').write_text(json.dumps(report, indent=2)+'\n')

    def run(name, command, expected_returncode=0):
        print(f'{name}...', flush=True)
        with (directory/f'{name}.log').open('w') as log:
            result = subprocess.run([str(arg) for arg in command], cwd=ROOT,
                                    stdout=log, stderr=subprocess.STDOUT)
        report['steps'].append(dict(name=name, returncode=result.returncode,
                                   expected_returncode=expected_returncode,
                                   passed=result.returncode == expected_returncode))
        save()
        if result.returncode != expected_returncode:
            print((directory/f'{name}.log').read_text()[-6000:], file=sys.stderr)
            raise RuntimeError(f'{name} failed; see {directory/name}.log')

    python = sys.executable
    try:
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
            run('clean-install', ['cargo', 'install', '--path', ROOT, '--locked',
                '--target-dir', directory/'clean-target', '--root', directory/'installed'])
            run('acceptance-installed', [python, 'tools/compat/check_candidate.py',
                '--binary', directory/'installed/bin/rless', '--results-dir', directory/'acceptance-installed'])
        if fingerprint() != report['build_and_test_inputs_sha256']:
            raise RuntimeError('build/test inputs changed during verification; rerun against a stable tree')
        helper = ROOT/'vendor/less-helpers/less-osc8-open'
        report['helpers'] = dict(osc8_sha256=hashlib.sha256(helper.read_bytes()).hexdigest())
        report['passed'] = True
    except (RuntimeError, OSError) as error:
        report['error'] = str(error)
        print(error, file=sys.stderr)
    report['finished_at_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    save()
    print(f'Passed: {report["passed"]}; summary: {directory/"summary.json"}', flush=True)
    return not report['passed']


if __name__ == '__main__':
    raise SystemExit(main())
