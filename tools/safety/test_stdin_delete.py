#!/usr/bin/env python3
"""Retained stdin :n/:d ownership, live fd0 identity and LSAN regression."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_pty


def sha(data):
    return hashlib.sha256(data).hexdigest()


def validate(result, output, binary):
    output, binary = Path(output), Path(binary)
    if result.get('binary_sha256') != sha(binary.read_bytes()):
        raise ValueError('changed stdin regression binary')
    if (output / 'positive.trace').read_bytes() != b'ready\nclose 0\n' or result.get('positive_status') != 0:
        raise ValueError('stdin close witness positive control did not detect close(0)')
    cases = result.get('cases', {})
    if set(cases) != {'retain', 'delete'}:
        raise ValueError('stdin ownership cases missing')
    for name, case in cases.items():
        raw = (output / (name + '.raw')).read_bytes()
        trace = (output / (name + '.close-trace')).read_bytes()
        if case.get('raw_sha256') != sha(raw) or case.get('close_trace_sha256') != sha(trace):
            raise ValueError('stdin witness differs from retained raw bytes')
        if trace != b'ready\n':
            raise ValueError('borrowed standard descriptor was closed')
        if case.get('status') != 0 or case.get('termios_restored') is not True:
            raise ValueError('stdin deletion status or terminal restoration failed')
        if re.search(rb'AddressSanitizer|LeakSanitizer|panicked|runtime error:', raw):
            raise ValueError('stdin ownership sanitizer diagnostic')
        if case.get('screens') != ['stdin', 'file']:
            raise ValueError('stdin/file screen transition missing')
        checkpoints = case.get('fd0_checkpoints')
        if (not isinstance(checkpoints, list) or len(checkpoints) != 2
                or [x.get('label') for x in checkpoints] != ['before', 'after']):
            raise ValueError('stdin descriptor checkpoints missing')
        first, second = checkpoints
        identity = ('target', 'dev', 'ino', 'mode')
        if (not re.fullmatch(r'pipe:\[\d+\]', first.get('target', ''))
                or not stat.S_ISFIFO(first.get('mode', 0))
                or any(first.get(key) != second.get(key) for key in identity)):
            raise ValueError('borrowed stdin pipe identity changed after navigation/deletion')
    return True


def run(binary, output):
    binary, output = Path(binary).resolve(strict=True), Path(output).resolve()
    output.mkdir(parents=True, exist_ok=False)
    shim = output / 'stdin_ownership.so'
    subprocess.run(['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
                    str(Path(__file__).parent / 'native/stdin_ownership.c'), '-ldl', '-o', str(shim)], check=True)
    positive = output / 'positive.trace'
    positive.touch()
    env = dict(PATH='/usr/bin:/bin', LD_PRELOAD=str(shim), RLESS_STDIN_CLOSE_TRACE=str(positive))
    control = subprocess.run([sys.executable, '-c', 'import os; os.close(0)'], env=env,
                             stdin=subprocess.DEVNULL, capture_output=True, timeout=5)
    safety = json.loads(Path(os.environ['RLESS_SAFETY_ENV']).read_text()) if os.environ.get('RLESS_SAFETY_ENV') else {}
    result = dict(binary_sha256=sha(binary.read_bytes()), safety_environment=safety, positive_status=control.returncode,
                  positive_trace_sha256=sha(positive.read_bytes()), shim_sha256=sha(shim.read_bytes()), cases={})
    for name, command in [('retain', ':n\n'), ('delete', ':d\n')]:
        with tempfile.TemporaryDirectory(prefix='rless-stdin-delete-', dir='/tmp') as directory:
            work = Path(directory)
            (work / 'i').write_text('file\n' * 80)
            trace = work / 'close-trace'; trace.touch()
            env = dict(PATH='/usr/bin:/bin', HOME=directory, TERM='xterm-256color', LC_ALL='C', LESSHISTFILE='-',
                       LD_PRELOAD=str(shim), RLESS_STDIN_CLOSE_TRACE=str(trace), **safety)
            case = dict(id='STDIN-' + name, mode='pty', size=[24, 80],
                        argv=['-PsREADY', '-PmREADY', '-PMREADY', '-', 'i'],
                        stream_b64=base64.b64encode(b'stdin\n' * 80).decode(),
                        events=[{'await': ['READY'], 'snapshot': True, 'heap_checkpoint': 'before'},
                                {'close_stream': True},
                                {'send': command, 'await': ['READY'], 'snapshot': True, 'heap_checkpoint': 'after'},
                                {'send': 'q'}], timeout=5)
            checkpoints = []
            def observe(pid, label):
                fd = Path('/proc') / str(pid) / 'fd/0'
                try:
                    info = fd.stat()
                    checkpoints.append(dict(label=label, target=os.readlink(fd), dev=info.st_dev, ino=info.st_ino, mode=info.st_mode))
                except FileNotFoundError:
                    checkpoints.append(dict(label=label, target='missing', dev=0, ino=0, mode=0))
            raw = bytearray(); original_read = os.read
            def capture(fd, count):
                content = original_read(fd, count); raw.extend(content); return content
            os.read = capture
            try:
                observed = run_pty([str(binary), *case['argv']], env, work, case,
                                   lambda data: data.replace(directory.encode(), b'{ROOT}'), checkpoint_observer=observe)
            finally:
                os.read = original_read
            (output / (name + '.raw')).write_bytes(raw)
            (output / (name + '.close-trace')).write_bytes(trace.read_bytes())
            result['cases'][name] = dict(status=observed['status'], termios_restored=observed['termios_restored'],
                screens=[x['rows'][0].strip() for x in observed['checkpoints']],
                fd0_checkpoints=checkpoints, raw_sha256=sha(raw), close_trace_sha256=sha(trace.read_bytes()))
            (output / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    try:
        validate(result, output, binary)
        result['oracle'] = dict(passed=True)
    except (ValueError, OSError) as error:
        result['oracle'] = dict(passed=False, error=str(error))
    (output / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path); parser.add_argument('output', type=Path)
    args = parser.parse_args(); result = run(args.binary, args.output)
    print(json.dumps(result['oracle']))
    return 0 if result['oracle']['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
