#!/usr/bin/env python3
"""Replay every frozen family against a binary; keep all reports outside Git."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--results-dir', type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    os.umask(0o077)
    runs = ROOT/'compat/.runs'
    runs.mkdir(exist_ok=True)
    if args.results_dir is None:
        directory = Path(tempfile.mkdtemp(prefix='candidate-', dir=runs))
    else:
        directory = args.results_dir.resolve()
        directory.mkdir(parents=True, exist_ok=False)
    print(f'Reports: {directory}', flush=True)
    summary = dict(binary=str(binary), sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                   platform=platform.platform(),
                   observer_sha256=hashlib.sha256((ROOT/'tools/compat/run_v2.py').read_bytes()).hexdigest(),
                   observer_revision_sha256=hashlib.sha256((ROOT/'compat/observer-v2.json').read_bytes()).hexdigest(),
                   families={}, executed=0, passed=0, failed=0)
    for family in ('options', 'behavior', 'interactions', 'integration'):
        report = directory/f'{family}.json'
        with (directory/f'{family}.log').open('w') as log:
            result = subprocess.run([
                sys.executable, str(ROOT/'tools/compat/run_v2.py'), 'replay',
                '--binary', str(binary), '--cases', str(ROOT/f'compat/cases/{family}.json'),
                '--expected', str(ROOT/f'compat/expected-{family}.json'),
                '--results', str(report)], stdout=log, stderr=subprocess.STDOUT)
        if not report.exists():
            print((directory/f'{family}.log').read_text(), file=sys.stderr)
            return 1
        data = json.loads(report.read_text())
        summary['families'][family] = {k: data[k] for k in ('executed', 'passed', 'failed', 'errors')}
        for key in ('executed', 'passed', 'failed'):
            summary[key] += data[key]
        print(f'{family}: {data["passed"]}/{data["executed"]} passed', flush=True)
        if result.returncode and not data['failed']:
            return result.returncode
    (directory/'summary.json').write_text(json.dumps(summary, indent=2)+'\n')
    print(f'Total: {summary["passed"]}/{summary["executed"]} passed')
    return bool(summary['failed'])


if __name__ == '__main__':
    raise SystemExit(main())
