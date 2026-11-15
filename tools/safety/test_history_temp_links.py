#!/usr/bin/env python3
"""Reproduce the predictable historyQ temp-name clobber from issue #20."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_pty


def sha(data):
    return hashlib.sha256(data).hexdigest()


def case(binary, kind):
    with tempfile.TemporaryDirectory(prefix='rless-hist-temp-') as directory:
        root = Path(directory)
        (root / 'i').write_bytes(b'needle\n' * 80)
        victim = root / 'victim'
        original = b'protected\n'
        victim.write_bytes(original)
        victim.chmod(0o644)
        trap = root / 'historyQ'
        if kind == 'symlink':
            trap.symlink_to('victim')
        else:
            os.link(victim, trap)
        history = root / 'historyX'
        environment = {
            'PATH': '/usr/bin:/bin', 'HOME': directory, 'TERM': 'xterm-256color',
            'LC_ALL': 'C', 'LESSHISTFILE': str(history),
        }
        scenario = {
            'id': 'HISTORY-TEMP-' + kind, 'mode': 'pty', 'size': [24, 80],
            'argv': ['-PsREADY', '-PmREADY', '-PMREADY', 'i'], 'files': {},
            'events': [
                {'await': ['READY']},
                {'send': '/needle\n', 'await': ['READY']},
                {'send': 'q'},
            ],
            'timeout': 5,
        }
        observed = run_pty(
            [str(binary), *scenario['argv']], environment, root, scenario,
            lambda data: data.replace(directory.encode(), b'{ROOT}'),
        )
        victim_bytes = victim.read_bytes()
        history_bytes = history.read_bytes() if history.exists() else b''
        trap_stat = trap.lstat() if trap.exists() or trap.is_symlink() else None
        remaining_temp_names = sorted(
            entry.name for entry in root.iterdir()
            if entry.name.startswith('.historyX.')
        )
        return {
            'status': observed['status'],
            'terminal_restored': observed['termios_restored'],
            'victim_sha256': sha(victim_bytes),
            'victim_mode': stat.S_IMODE(victim.stat().st_mode),
            'victim_unchanged': victim_bytes == original,
            'trap_present': trap_stat is not None,
            'trap_is_symlink': trap.is_symlink(),
            'trap_is_hardlink': trap_stat is not None and not trap.is_symlink()
                                and trap_stat.st_ino == victim.stat().st_ino,
            'history_has_search': b'"needle\n' in history_bytes,
            'history_mode': stat.S_IMODE(history.stat().st_mode) if history.exists() else None,
            'remaining_random_temps': remaining_temp_names,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--expect-vulnerable', action='store_true')
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    cases = {kind: case(binary, kind) for kind in ('symlink', 'hardlink')}
    for kind, result in cases.items():
        common = (result['status'] == 0 and result['terminal_restored']
                  and result['history_has_search'])
        if args.expect_vulnerable:
            passed = (common and not result['victim_unchanged']
                      and result['victim_mode'] == 0o600 and not result['trap_present'])
        else:
            passed = (common and result['victim_unchanged']
                      and result['victim_mode'] == 0o644 and result['trap_present']
                      and result['history_mode'] == 0o600
                      and result['remaining_random_temps'] == []
                      and (result['trap_is_symlink'] if kind == 'symlink'
                           else result['trap_is_hardlink']))
        result['expected_observed'] = passed
    report = {
        'binary': str(binary), 'binary_sha256': sha(binary.read_bytes()),
        'test_sha256': sha(Path(__file__).read_bytes()),
        'expect_vulnerable': args.expect_vulnerable, 'cases': cases,
        'passed': all(result['expected_observed'] for result in cases.values()),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({kind: value['expected_observed'] for kind, value in cases.items()}))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
