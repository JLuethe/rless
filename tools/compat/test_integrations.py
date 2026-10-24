#!/usr/bin/env python3
"""Compare real pager integrations and a sparse >4 GiB seek with the reference."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import tempfile

from run_v2 import run_case, run_pty


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--reference', required=True, type=Path)
    parser.add_argument('--results', required=True, type=Path)
    args = parser.parse_args()
    binaries = [args.reference.resolve(), args.binary.resolve()]
    os.umask(0o077)
    report = {}
    for name, executable, argv, files, variables in [
        ('git-diff', '/usr/bin/git', ['-p', 'diff', '--no-index', 'old', 'new'],
         {'old': {'text': 'old line\n'}, 'new': {'text': 'new line\n'}},
         {'GIT_PAGER': '{ROOT}/pager.sh', 'GIT_CONFIG_NOSYSTEM': '1',
          'GIT_CONFIG_GLOBAL': '/dev/null'}),
        ('man-local', '/usr/bin/man', ['-l', 'sample.1'],
         {'sample.1': {'text': '.TH SAMPLE 1\n.SH NAME\nsample \\- pager test\n.SH DESCRIPTION\nA short manual.\n'}},
         {'MANPAGER': '{ROOT}/pager.sh', 'MANWIDTH': '80'}),
    ]:
        observations = []
        for binary in binaries:
            case = dict(id=name, mode='pty', argv=argv, events=[], files={
                **files, 'pager.sh': {'mode': 0o700, 'text':
                    '#!/bin/sh\nprintf "pager\\n" >> pager.log\nexec "$PAGER_UNDER_TEST" "$@"\n'}},
                env={**variables, 'PAGER_UNDER_TEST': str(binary), 'LESS': '-FRX'})
            actual = run_case(Path(executable), case)
            assert actual['termios_restored'], (name, 'terminal not restored')
            assert actual['files']['pager.log']['sha256'] == hashlib.sha256(b'pager\n').hexdigest()
            observations.append(actual)
        assert observations[0] == observations[1], (name, observations)
        report[name] = {'passed': True, 'actual': observations[1]}

    case = dict(id='stream-without-eof', mode='pty', argv=['-P@@READY@@'],
                stream_b64=base64.b64encode(b'line\n'*40).decode(),
                events=[{'await': ['@@READY@@'], 'snapshot': True}, {'send': 'q'}])
    observations = [run_case(binary, case) for binary in binaries]
    assert observations[0] == observations[1], ('stream-without-eof', observations)
    assert observations[1]['termios_restored']
    report['stream-without-eof'] = {'passed': True, 'actual': observations[1]}

    case = dict(id='terminate-restores-terminal', mode='pty', argv=['-P@@READY@@', 'data'],
                files={'data': {'text': 'line\n'*40}},
                events=[{'await': ['@@READY@@']}, {'signal': 'SIGTERM'}])
    observations = [run_case(binary, case) for binary in binaries]
    assert observations[0] == observations[1], ('terminate-restores-terminal', observations)
    assert observations[1]['termios_restored'] and observations[1]['status'] == 15
    report['terminate-restores-terminal'] = {'passed': True, 'actual': observations[1]}

    events = [{'await': ['@@READY@@']}]
    for _ in range(10):
        for size in ([18, 60], [24, 80]):
            events.append({'resize': size, 'await': ['@@READY@@'], 'snapshot': True})
            events.append({'send': 'j', 'await': ['@@READY@@'], 'snapshot': True})
            events.append({'send': 'k', 'await': ['@@READY@@'], 'snapshot': True})
    events.append({'send': 'q'})
    case = dict(id='repeated-resize', mode='pty', argv=['-P@@READY@@', 'data'],
                files={'data': {'text': ''.join(f'line {n:03d}\n' for n in range(1, 51))}}, events=events)
    observations = [run_case(binary, case) for binary in binaries]
    assert observations[0] == observations[1], ('repeated-resize', observations)
    assert len(observations[1]['checkpoints']) == 60
    for index, checkpoint in enumerate(observations[1]['checkpoints']):
        expected = 'line 002' if index % 3 == 1 else 'line 001'
        assert checkpoint['rows'][0].startswith(expected), (index, checkpoint['rows'][0])
    assert observations[1]['termios_restored']
    report['repeated-resize'] = {'passed': True, 'actual': observations[1]}

    for operation, change, marker, options in [
        ('append', {'append_file': {'path': 'data', 'text': 'APPENDED-ONE\nAPPENDED-TWO\n'}},
         'APPENDED-TWO', []),
        ('rotate', {'rotate_file': {'path': 'data', 'to': 'data.old',
                                   'text': 'ROTATED-ONE\nROTATED-TWO\n'}},
         'ROTATED-TWO', ['--follow-name']),
    ]:
        name = 'follow-'+operation
        case = dict(id=name, mode='pty', argv=['-f', '-X', '-P@@READY@@', *options, 'data'],
                    files={'data': {'text': 'line\n'*40}},
                    events=[{'await': ['@@READY@@']},
                            {'send': 'F', 'await': ['Waiting for data']},
                            {**change, 'await': ['Waiting for data'], 'snapshot': True},
                            {'send': '\x03', 'await': ['@@READY@@', '(press RETURN)']},
                            {'send': '\nq'}])
        observations = [run_case(binary, case) for binary in binaries]
        assert observations[0] == observations[1], (name, observations)
        assert any(marker in row for row in observations[1]['checkpoints'][0]['rows']), name
        assert observations[1]['termios_restored']
        report[name] = {'passed': True, 'actual': observations[1]}

    # less reports an unavailable saved position after a file shrinks. Verify
    # both that diagnostic and recovery by jumping back to the new beginning.
    case = dict(id='truncate-recovery', mode='pty', argv=['-f', '-X', '-P@@READY@@', 'data'],
                files={'data': {'text': 'line\n'*40}},
                events=[{'await': ['@@READY@@']},
                        {'send': 'F', 'await': ['Waiting for data']},
                        {'replace_file': {'path': 'data', 'text': 'TRUNCATED-ONE\nTRUNCATED-TWO\n'}},
                        {'send': '\x03', 'await': ['@@READY@@', '(press RETURN)']},
                        {'send': '\nR', 'await': ['Cannot seek to that file position'], 'snapshot': True},
                        {'send': '\ng', 'await': ['@@READY@@'], 'snapshot': True},
                        {'send': 'q'}])
    observations = [run_case(binary, case) for binary in binaries]
    assert observations[0] == observations[1], ('truncate-recovery', observations)
    assert any('TRUNCATED-TWO' in row for row in observations[1]['checkpoints'][1]['rows'])
    assert observations[1]['termios_restored']
    report['truncate-recovery'] = {'passed': True, 'actual': observations[1]}

    case = dict(id='follow-live-pipe', mode='pty', argv=['-n', '-X', '-P@@READY@@'],
                stream_b64=base64.b64encode(b'line\n'*40).decode(),
                env={'LESS_DATA_DELAY': '20'},
                events=[{'await': ['@@READY@@']},
                        {'send': 'F', 'await': ['Waiting for data']},
                        {'write_stream_b64': base64.b64encode(b'PIPE-APPEND\n').decode(),
                         'await': ['Waiting for data'], 'snapshot': True},
                        {'send': '\x03', 'await': ['@@READY@@']},
                        {'send': 'q'}])
    observations = [run_case(binary, case) for binary in binaries]
    assert observations[0] == observations[1], ('follow-live-pipe', observations)
    assert any('PIPE-APPEND' in row for row in observations[1]['checkpoints'][0]['rows'])
    assert observations[1]['termios_restored']
    report['follow-live-pipe'] = {'passed': True, 'actual': observations[1]}

    with tempfile.TemporaryDirectory(prefix='rless-sparse-') as directory:
        work = Path(directory)
        path = work/'sparse'
        with path.open('wb') as file:
            file.seek((1 << 32)-1)
            file.write(b'\nbeyond-4-GiB\n')
        env = dict(PATH='/usr/bin:/bin', HOME=directory, TERM='xterm-256color',
                   LC_ALL='C', LESSHISTFILE='-')
        case = dict(id='sparse-byte-position',
                    events=[{'await': ['@@READY@@'], 'snapshot': True}, {'send': 'q'}])
        observations = [run_pty([str(binary), '-f', '-n', '-S', '-P@@READY@@',
                                 '+4294967296P', str(path)], env, work, case,
                                lambda data: data.replace(directory.encode(), b'{ROOT}'))
                        for binary in binaries]
        assert observations[0] == observations[1], ('sparse-byte-position', observations)
        assert observations[1]['checkpoints'][0]['rows'][0].startswith('beyond-4-GiB')
        assert observations[1]['termios_restored']
        report['sparse-byte-position'] = {'passed': True, 'actual': observations[1]}
    args.results.write_text(json.dumps(report, indent=2)+'\n')
    print(f'{len(report)} supplemental integrations passed')


if __name__ == '__main__':
    main()
