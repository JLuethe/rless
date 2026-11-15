#!/usr/bin/env python3
"""Check owned descriptor cleanup when a file changes into a tty before open."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_pty


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--expect-leak', action='store_true')
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    output = args.output.resolve()
    source = Path(__file__).with_name('native') / 'rejected_tty_stat.c'
    output.mkdir(parents=True, exist_ok=False)
    shim = output / 'rejected_tty_stat.so'
    subprocess.run(['cc', '-shared', '-fPIC', '-Wall', '-Wextra', '-Werror',
                    str(source), '-ldl', '-o', str(shim)], check=True)
    with tempfile.TemporaryDirectory(prefix='rless-rejected-tty-') as directory:
        work = Path(directory)
        (work / 'i').write_bytes(b'line\n' * 80)
        trace = work / 'stat-trace'
        trace.touch()
        env = {
            'PATH': '/usr/bin:/bin', 'HOME': directory, 'TERM': 'xterm-256color',
            'LC_ALL': 'C', 'LESSHISTFILE': '-', 'LD_PRELOAD': str(shim),
            'RLESS_REJECTED_TTY_STAT_TRACE': str(trace),
        }
        scenario = {
            'id': 'REJECTED-TTY-FD', 'mode': 'pty', 'size': [24, 80],
            'argv': ['-PsREADY', '-PmREADY', '-PMREADY', 'i'],
            'events': [
                {'await': ['READY'], 'resources': True},
                {'send': ':e /dev/tty\n', 'await': ['is a terminal'], 'resources': True},
                {'send': '\n', 'await': ['READY'], 'resources': True},
                {'send': ':e /dev/tty\n', 'await': ['is a terminal'], 'resources': True},
                {'send': '\n', 'await': ['READY'], 'resources': True},
                {'send': 'q'},
            ],
            'timeout': 5,
        }
        observed = run_pty([str(binary), *scenario['argv']], env, work, scenario,
                           lambda data: data.replace(directory.encode(), b'{ROOT}'))
        resources = observed['resource_checkpoints']
        counts = [len(point['descriptors']) for point in resources]
        trace_bytes = trace.read_bytes()
        steady = (len(counts) == 5 and counts[1:] == [counts[0]] * 4)
        leaking = (len(counts) == 5 and counts[1:] ==
                   [counts[0] + 1, counts[0] + 1, counts[0] + 2, counts[0] + 2])
        passed = (observed['status'] == 0 and observed['termios_restored']
                  and trace_bytes == b'regular-stat\n' and
                  (leaking if args.expect_leak else steady))
        report = {
            'binary_sha256': sha(binary.read_bytes()),
            'shim_source_sha256': sha(source.read_bytes()),
            'shim_sha256': sha(shim.read_bytes()),
            'test_sha256': sha(Path(__file__).read_bytes()),
            'status': observed['status'],
            'terminal_restored': observed['termios_restored'],
            'trace_sha256': sha(trace_bytes),
            'descriptor_counts': counts,
            'descriptor_snapshots': resources,
            'expected_observed': passed,
        }
        (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps({key: report[key] for key in
                          ('status', 'terminal_restored', 'descriptor_counts', 'expected_observed')}))
        return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
