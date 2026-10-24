#!/usr/bin/env python3
"""Seeded differential process/PTY probes; never modify frozen expectations."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import random
import subprocess
import tempfile
import time

from run_v2 import run_case


def cases(seed, rounds):
    rng = random.Random(seed)
    values = ['', '0', '-1', '1', '2147483647', '2147483648',
              '9223372036854775807', '9223372036854775808', '9'*80, '1.5', 'abc', '+1']
    for option in ['-b', '-h', '-j', '-x', '-y', '-z', '--shift=', '--header=', '--wheel-lines=']:
        for value in values:
            yield dict(id=f'number-{option}-{value}', mode='pipe', argv=[option+value],
                       stdin_b64=base64.b64encode(b'bytes\x00\xff\r\n').decode())

    options = ['-N', '-R', '-S', '-U', '-s', '-i', '-I', '-f', '-x4', '-X', '-F']
    lengths = [0, 1, 2, 127, 4095, 4096, 4097, 65536]
    for index in range(rounds*3):
        data = rng.randbytes(rng.choice(lengths))
        yield dict(id=f'bytes-{index}', mode='pipe',
                   argv=rng.sample(options, rng.randrange(5)),
                   locale=rng.choice(['C', 'C.UTF-8']),
                   env={'LESS': rng.choice(['', '-R -N', '-+R -x4', '-S -i'])},
                   stdin_b64=base64.b64encode(data).decode())

    for index in range(rounds):
        # Exercise diagnostics and recovery from malformed user configuration.
        lines = ['#command', 'k forw-line', '#line-edit', r'\ek up', '#env', 'LESS = -R']
        lines.insert(rng.randrange(len(lines)+1), rng.choice([
            'unknown-action nonexistent-action', '#unknown', '#env',
            'LESS += -N', 'LESS = "unterminated', '#if version >= 99999']))
        yield dict(id=f'lesskey-{index}', mode='pipe', argv=['data'],
                   env={'LESSKEYIN': '{ROOT}/keys'},
                   files={'keys': {'text': '\n'.join(lines)+'\n'},
                          'data': {'text': 'input\n'}})

    commands = ['j', 'k', '3j', '2k', 'G', '5g', 'd', 'u', ' ', 'b', '50%',
                'ma', "'a", '/needle\n', 'n', 'N', '?alpha\n']
    text = ''.join(f'{n:03d} alpha {"needle" if n % 7 == 0 else "plain"}\t'
                   f'界 é \x1b[31mred\x1b[m\n' for n in range(1, 151))
    for index in range(rounds):
        events = [{'await': ['@@READY@@']}]
        for command in rng.choices(commands, k=8):
            events.append({'send': command, 'await': ['@@READY@@', '(press RETURN)'], 'snapshot': True})
            events.append({'send': '\n', 'await': ['@@READY@@']})
        events.append({'send': 'q'})
        yield dict(id=f'sequence-{index}', mode='pty',
                   argv=['-f', '-X', '--no-keypad', '--no-paste', '-Ps@@READY@@',
                         '-Pm@@READY@@', '-PM@@READY@@',
                         *rng.sample(['-N', '-R', '-S', '-x4', '-J'], rng.randrange(4)), 'data'],
                   locale=rng.choice(['C', 'C.UTF-8']), size=rng.choice([[8, 30], [12, 40], [24, 80]]),
                   files={'data': {'text': text}}, events=events)

    for index in range(rounds):
        words = ['plain', 'with space', "quote'word", 'double"quote', '$dollar',
                 'semi;colon', 'tab\tword', 'newline\nword', '界']
        yield dict(id=f'lessecho-{index}', mode='pipe', executable='lessecho',
                   argv=[*rng.choice([[], ['-a'], ['-ox', '-cy'], ['-e!']]),
                         *rng.choices(words, k=rng.randrange(1, 8))])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--reference', required=True, type=Path)
    parser.add_argument('--results', required=True, type=Path)
    parser.add_argument('--seed', type=int, default=712)
    parser.add_argument('--rounds', type=int, default=20)
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error('--rounds must be positive')
    candidate = args.binary.resolve(strict=True)
    reference = args.reference.resolve(strict=True)
    os.umask(0o077)
    args.results.parent.mkdir(parents=True, exist_ok=True)
    clock_source = Path(__file__).resolve().parent/'probes/fixed_clock.c'
    clock_directory = Path(tempfile.mkdtemp(prefix='fixed-clock-', dir=args.results.parent.resolve()))
    clock = clock_directory/'clock.so'
    subprocess.run(['cc', '-shared', '-fPIC', str(clock_source), '-o', str(clock)], check=True)
    report = dict(seed=args.seed, rounds=args.rounds, executed=0, passed=0, failed=0,
                  binary=str(candidate), binary_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest(),
                  reference_sha256=hashlib.sha256(reference.read_bytes()).hexdigest(), failures=[],
                  clock=dict(unix_seconds=1700000000, library=str(clock),
                             source_sha256=hashlib.sha256(clock_source.read_bytes()).hexdigest(),
                             scope='time() only; monotonic clocks and signal timers remain real'))
    started = time.monotonic()
    for case in cases(args.seed, args.rounds):
        case.setdefault('env', {})['LD_PRELOAD'] = str(clock)
        try:
            expected = run_case(reference, case)
            actual = run_case(candidate, case)
            if actual != expected:
                report['failures'].append(dict(case=case, expected=expected, actual=actual))
                report['failed'] += 1
            else:
                report['passed'] += 1
        except Exception as error:
            report['failed'] += 1
            report['failures'].append(dict(case=case, error=str(error)))
        report['executed'] += 1
        if report['executed'] % 50 == 0:
            print({key: report[key] for key in ('executed', 'passed', 'failed')}, flush=True)
    report['elapsed_seconds'] = time.monotonic()-started
    args.results.parent.mkdir(parents=True, exist_ok=True)
    args.results.write_text(json.dumps(report, indent=2, ensure_ascii=False)+'\n')
    print({key: value for key, value in report.items() if key != 'failures'}, flush=True)
    return bool(report['failed'])


if __name__ == '__main__':
    raise SystemExit(main())
