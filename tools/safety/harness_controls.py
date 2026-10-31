#!/usr/bin/env python3
"""Prove real pipe and PTY diagnostic capture with known detector controls."""
import argparse
import json
from pathlib import Path
import re
import sys

from common import sha, write_json
import test_fault_paths as faults


EXPECTED = {'safe': None, 'asan-uaf': r'ERROR: AddressSanitizer: heap-use-after-free',
            'lsan-leak': r'ERROR: LeakSanitizer: detected memory leaks'}


def validate_capture(report):
    if type(report.get('schema_version')) is not int or report.get('schema_version') != 1:
        raise ValueError('unsupported detector capture schema')
    records = report.get('records', [])
    expected = {(probe, mode) for probe in EXPECTED for mode in ('pipe', 'pty')}
    if len(records) != len(expected):
        raise ValueError('missing or duplicate detector capture control')
    seen = set()
    for record in records:
        key = (record['probe'], record['mode'])
        if key not in expected or key in seen:
            raise ValueError('unknown or duplicated detector capture control')
        seen.add(key)
        if type(record.get('status')) is not int or record.get('trace_hex') != '':
            raise ValueError('invalid detector status or unexpected fault trace')
        if sha(record['binary']) != record['binary_sha256']:
            raise ValueError('changed detector binary')
        texts = []
        for item in record['diagnostics']:
            if sha(item['path']) != item['sha256']:
                raise ValueError('changed diagnostic capture')
            texts.append(Path(item['path']).read_text(errors='replace'))
        text = '\n'.join(texts)
        if record['probe'] == 'safe':
            if record['status'] != 0 or texts:
                raise ValueError('safe counterpart failed or reported a defect')
        else:
            # In the pinned combined runtime, LSAN_OPTIONS is parsed after
            # ASAN_OPTIONS and sets their shared exitcode flag to 87. Independent
            # 86/87/88 controls verify this precedence; diagnostics distinguish
            # UAF from leaks. Do not disable leak detection to force status 86.
            expected_status = 87
            if record['status'] != expected_status or not re.search(EXPECTED[record['probe']], text):
                raise ValueError('intended detector did not report through the actual harness')
            if record['probe'] == 'lsan-leak' and '8192 byte' not in text:
                raise ValueError('leak control detected the wrong allocation')
        if record['mode'] == 'pty' and record.get('termios_restored') is not True:
            raise ValueError('detector PTY did not restore terminal state')
    return True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--probe-dir', type=Path, required=True)
    parser.add_argument('--instrumentation-env', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    # Probe processes run in isolated temporary working directories. Their
    # sanitizer log_path must point back to the retained output directory.
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    config = json.loads(args.instrumentation_env.read_text())
    if set(config) - {'ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH'}:
        parser.error('unsupported instrumentation configuration')
    report = dict(schema_version=1, scope='Actual pipe/PTY sanitizer capture controls', records=[])
    for probe in EXPECTED:
        binary = (args.probe_dir / probe).resolve(strict=True)
        for mode in ('pipe', 'pty'):
            directory = args.output / f'{probe}-{mode}'
            directory.mkdir()
            # Separate positive reports from application diagnostics. A passing
            # control must never suppress an application report in another lane.
            faults.INSTRUMENTATION = dict(config)
            options = [part for part in config['ASAN_OPTIONS'].split(':') if not part.startswith('log_path=')]
            faults.INSTRUMENTATION['ASAN_OPTIONS'] = ':'.join(options) + ':log_path=' + str(directory / 'asan')
            if mode == 'pipe':
                actual, trace = faults.pipe_run(binary)
                record = dict(status=actual.returncode)
            else:
                case = dict(id='SANITIZER-CAPTURE-' + probe, mode='pty', argv=[], files={}, events=[])
                actual, trace = faults.pty_run(binary, case)
                record = dict(status=actual['status'], termios_restored=actual['termios_restored'])
            if trace:
                raise ValueError('detector unexpectedly wrote a fault marker')
            record.update(probe=probe, mode=mode, trace_hex=trace.hex(), binary=str(binary), binary_sha256=sha(binary),
                          diagnostics=[dict(path=str(p), sha256=sha(p)) for p in sorted(directory.iterdir())])
            report['records'].append(record)
            write_json(args.output / 'summary.json', report)
    validate_capture(report)
    report['capture_verified'] = True
    write_json(args.output / 'summary.json', report)
    print('Verified safe/UAF/leak controls through pipe and PTY diagnostic capture.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
