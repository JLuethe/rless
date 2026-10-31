#!/usr/bin/env python3
"""First-buffer ENOMEM and LRU fallback, with retained linked allocator proof."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

from linked_object import verify


def sha(data):
    return hashlib.sha256(data).hexdigest()


def bounds(binary):
    symbols = subprocess.check_output(['nm', '-S', '-C', str(binary)], text=True)
    matches = re.findall(r'^([0-9a-f]+) ([0-9a-f]+) [tT] rless::ch::ch_addbuf$', symbols, re.M)
    if len(matches) != 1:
        raise ValueError('ambiguous ch_addbuf symbol')
    start, size = (int(x, 16) for x in matches[0])
    return start, start + size


def observation(output, name, status):
    stdout = (output / (name + '.stdout')).read_bytes()
    stderr = (output / (name + '.stderr')).read_bytes()
    trace = (output / (name + '.trace')).read_bytes()
    return dict(status=status, stdout_sha256=sha(stdout), stdout_bytes=len(stdout),
                stderr_sha256=sha(stderr), trace=trace.decode('ascii'), trace_sha256=sha(trace))


def validate_raw(result, output, binary, object_file):
    output, binary, object_file = Path(output), Path(binary), Path(object_file)
    if result.get('binary_sha256') != sha(binary.read_bytes()) or result.get('wrapper_sha256') != sha(object_file.read_bytes()):
        raise ValueError('changed first-buffer binary or allocator object')
    start, end = bounds(binary)
    if result.get('ch_addbuf') != dict(start=hex(start), end=hex(end), size=end-start):
        raise ValueError('changed first-buffer caller identity')
    cases = result.get('cases', {})
    if set(cases) != {'control', 'miss', 'hit', 'reuse'}:
        raise ValueError('missing first-buffer controls')
    for name, case in cases.items():
        if type(case.get('status')) is not int or case != observation(output, name, case['status']):
            raise ValueError('first-buffer observations differ from raw bytes')
        if name in ('hit', 'reuse'):
            match = re.fullmatch(r'ch_addbuf calloc\(1,8240\) caller_offset=([0-9a-f]+)\n', case['trace'])
            if match is None or not start <= int(match[1], 16) < end:
                raise ValueError('allocation did not hit exactly once inside ch_addbuf')
        elif case['trace']:
            raise ValueError('first-buffer control unexpectedly injected')
    return cases


def validate_controls(cases, output):
    for name in ('control', 'miss', 'reuse'):
        payload = b'line\n' * (4000 if name == 'reuse' else 20)
        if (cases[name]['status'] != 0 or cases[name]['stdout_sha256'] != sha(payload)
                or (Path(output)/(name+'.stderr')).read_bytes()):
            raise ValueError('first-buffer control or LRU reuse lost bytes')


def validate_allocator(result, binary, object_file):
    proof = verify(Path(binary), Path(object_file), '__wrap_calloc')
    if result.get('allocator_proof') != proof:
        raise ValueError('first-buffer allocator proof differs from actual linked object')
    return proof


def validate(result, output, binary, object_file):
    cases = validate_raw(result, output, binary, object_file)
    stderr = (Path(output)/'hit.stderr').read_bytes()
    if cases['hit']['status'] == -6 and b'null pointer dereference' in stderr:
        raise ValueError('first-buffer behavior: empty-list hit aborted with signal 6')
    validate_controls(cases, output)
    if (cases['hit']['status'] != 1 or cases['hit']['stdout_bytes'] != 0
            or stderr.count(b'cannot allocate input buffer') != 1
            or re.search(rb'panicked|AddressSanitizer|LeakSanitizer|runtime error:', stderr)):
        raise ValueError('first ENOMEM was not diagnosed once with status 1')
    return validate_allocator(result, binary, object_file)


def validate_expected_failure(result, output, binary, object_file):
    """Accept only the original empty-list SIGABRT, never an arbitrary error."""
    cases = validate_raw(result, output, binary, object_file)
    validate_controls(cases, output)
    stderr = (Path(output)/'hit.stderr').read_bytes()
    if (cases['hit']['status'] != -6 or cases['hit']['stdout_bytes'] != 0
            or re.search(rb'panicked at src/ch\.rs:\d+:\d+:\s*null pointer dereference', stderr) is None
            or b'cannot allocate input buffer' in stderr):
        raise ValueError('first-buffer mutant lacks the intended null-pointer SIGABRT')
    validate_allocator(result, binary, object_file)
    return {'reason':'first-buffer-empty-list-sigabrt','status':-6}


def run(binary, output, object_file):
    binary, output, object_file = Path(binary).resolve(strict=True), Path(output).resolve(), Path(object_file).resolve(strict=True)
    output.mkdir(parents=True, exist_ok=False)
    start, end = bounds(binary)
    result = dict(binary=str(binary), binary_sha256=sha(binary.read_bytes()),
                  ch_addbuf=dict(start=hex(start), end=hex(end), size=end-start),
                  wrapper_sha256=sha(object_file.read_bytes()), cases={})
    try:
        result['allocator_proof'] = verify(binary, object_file, '__wrap_calloc')
    except ValueError as error:
        result['allocator_proof'] = None
        result['allocator_proof_error'] = str(error)
    # Behavior still executes for historical wrong-object fixtures, so their
    # rejection records the crash rather than only a changed wrapper size.
    for name in ('control', 'miss', 'hit', 'reuse'):
        with tempfile.TemporaryDirectory(prefix='rless-firstbuf-', dir='/tmp') as directory:
            work = Path(directory)
            (work / 'i').write_bytes(b'line\n' * (4000 if name == 'reuse' else 20))
            (work / 'hit').write_bytes(b'')
            env = dict(PATH='/usr/bin:/bin', HOME=directory, LC_ALL='C', TERM='dumb', LESSHISTFILE='-', RUST_BACKTRACE='1')
            if os.environ.get('RLESS_SAFETY_ENV'):
                env.update(json.loads(Path(os.environ['RLESS_SAFETY_ENV']).read_text()))
            if name != 'control':
                env.update(RLESS_BUFFER_TRACE=str(work / 'hit'), RLESS_BUFFER_START=hex(0 if name == 'miss' else start),
                           RLESS_BUFFER_END=hex(1 if name == 'miss' else end), RLESS_BUFFER_SKIP='1' if name == 'reuse' else '0')
            process = subprocess.run([str(binary), 'i'], cwd=work, env=env, stdin=subprocess.DEVNULL, capture_output=True, timeout=5)
            (output / (name + '.stdout')).write_bytes(process.stdout)
            (output / (name + '.stderr')).write_bytes(process.stderr)
            (output / (name + '.trace')).write_bytes((work / 'hit').read_bytes())
            result['cases'][name] = observation(output, name, process.returncode)
            (output / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    try:
        validate(result, output, binary, object_file)
        result['oracle'] = dict(passed=True)
    except (ValueError, OSError) as error:
        result['oracle'] = dict(passed=False, error=str(error))
    (output / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('object_file', type=Path)
    args = parser.parse_args()
    result = run(args.binary, args.output, args.object_file)
    print(json.dumps(result['oracle']))
    return 0 if result['oracle']['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
