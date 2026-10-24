#!/usr/bin/env python3
"""Version 2 semantic PTY checkpoints; version 1 observer/corpus stay frozen."""
import argparse
import base64
import codecs
from collections import namedtuple
import copy
import errno
import fcntl
import hashlib
import json
import os
from pathlib import Path
import pty
import re
import select
import signal
import struct
import subprocess
import tempfile
import termios
import time

import pyte

ROOT = Path(__file__).resolve().parents[2]
LinkedChar = namedtuple('LinkedChar', pyte.screens.Char._fields + ('link',),
                        defaults=pyte.screens.Char.__new__.__defaults__ + ('',))


def load(path):
    return json.loads(Path(path).read_text())


def b64(data):
    return base64.b64encode(data).decode()


def replay_expectations(path):
    expected = load(path)
    revision = load(ROOT/'compat/observer-v2.json')
    for case_id, checkpoint in revision['checkpoints'].items():
        if case_id not in expected:
            continue
        previous = json.dumps(expected[case_id], sort_keys=True, separators=(',', ':')).encode()
        if hashlib.sha256(previous).hexdigest() != checkpoint['previous_sha256']:
            raise ValueError('observer revision requires the unchanged version 1 record: '+case_id)
        expected[case_id] = checkpoint['expected']
    return expected


class Screen(pyte.Screen):
    """Terminal cells plus DEC modes, alternate screen, bells and OSC events."""
    def __init__(self, columns, lines):
        self.private_modes = set()
        self.bells = 0
        self.osc = []
        self.saved_main = None
        super().__init__(columns, lines)

    def bell(self, *args):
        self.bells += 1

    @property
    def default_char(self):
        return LinkedChar(**super().default_char._asdict())

    def select_graphic_rendition(self, *args):
        self.cursor.attrs = LinkedChar(**self.cursor.attrs._asdict())
        link = self.cursor.attrs.link
        super().select_graphic_rendition(*args)
        self.cursor.attrs = LinkedChar(**self.cursor.attrs._asdict())._replace(link=link)

    def draw(self, data):
        self.cursor.attrs = LinkedChar(**self.cursor.attrs._asdict())
        super().draw(data)

    def set_mode(self, *modes, **kwargs):
        if kwargs.get('private'):
            self.private_modes.update(modes)
            if any(m in (47, 1047, 1049) for m in modes) and self.saved_main is None:
                self.saved_main = (copy.deepcopy(self.buffer), copy.deepcopy(self.cursor))
                self.erase_in_display(2)
                self.cursor_position()
        super().set_mode(*modes, **kwargs)

    def reset_mode(self, *modes, **kwargs):
        if kwargs.get('private'):
            self.private_modes.difference_update(modes)
            if any(m in (47, 1047, 1049) for m in modes) and self.saved_main:
                self.buffer, self.cursor = self.saved_main
                self.saved_main = None
        super().reset_mode(*modes, **kwargs)

    def state(self):
        styles, links = [], []
        for y in range(self.lines):
            for x in range(self.columns):
                cell = self.buffer[y][x]
                attrs = cell._asdict()
                attrs.pop('data')
                link = attrs.pop('link', '')
                if link:
                    if links and links[-1][0] == y and links[-1][2] == x and links[-1][3] == link:
                        links[-1][2] = x+1
                    else:
                        links.append([y, x, x+1, link])
                if any(attrs[k] != v for k, v in pyte.screens.Char(' ')._asdict().items() if k != 'data'):
                    if styles and styles[-1][0] == y and styles[-1][2] == x and styles[-1][3] == attrs:
                        styles[-1][2] = x+1
                    else:
                        styles.append([y, x, x+1, attrs])
        result = dict(rows=self.display, styles=styles,
                    cursor=[self.cursor.y, self.cursor.x, self.cursor.hidden],
                    private_modes=sorted(self.private_modes), modes=sorted(self.mode),
                    bells=self.bells, osc=self.osc)
        if links:
            result['hyperlinks'] = links
        return result


def run_case(binary, case):
    if case.get('executable'):
        if case['executable'] not in ('lessecho',):
            raise ValueError('unsupported companion executable')
        binary = binary.parent/case['executable']
    with tempfile.TemporaryDirectory(prefix='rless-case-') as directory:
        work = Path(directory)
        def expand(value):
            return value.replace('{ROOT}', directory)
        fixtures = {}
        if case.get('fixture_set'):
            if case['fixture_set'] != 'pager':
                raise ValueError('unknown fixture set')
            fixtures.update(load(ROOT/'compat/fixtures/pager.json'))
        fixtures.update(case.get('files', {}))
        for name, value in fixtures.items():
            if Path(name).is_absolute() or '..' in Path(name).parts:
                raise ValueError('fixture path must stay within the case directory')
            path = work/name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(base64.b64decode(value['base64']) if 'base64' in value else value['text'].encode())
            path.chmod(value.get('mode', 0o600))
        env = dict(PATH='/usr/bin:/bin', HOME=directory, TERM=case.get('term', 'xterm-256color'),
                   LC_ALL=case.get('locale', 'C'), LESSHISTFILE='-')
        for key, value in case.get('env', {}).items():
            if value is None:
                env.pop(key, None)
            else:
                env[key] = expand(value)
        argv = [str(binary), *[expand(x) for x in case['argv']]]
        def norm(data):
            return data.replace(directory.encode(), b'{ROOT}')
        if case['mode'] == 'pipe':
            result = subprocess.run(argv, cwd=work, env=env, input=base64.b64decode(case.get('stdin_b64', '')),
                                    capture_output=True, timeout=case.get('timeout', 5))
            observed = dict(status=result.returncode, stdout_b64=b64(norm(result.stdout)),
                            stderr_b64=b64(norm(result.stderr)))
        else:
            observed = run_pty(argv, env, work, case, norm)
        observed['files'] = {}
        for path in sorted(work.rglob('*')):
            if path.is_file():
                data = norm(path.read_bytes())
                observed['files'][str(path.relative_to(work))] = dict(sha256=hashlib.sha256(data).hexdigest(),
                    mode=path.stat().st_mode & 0o777, bytes=len(data))
        return observed


def run_pty(argv, env, work, case, norm):
    master, slave = pty.openpty()
    rows, columns = case.get('size', [24, 80])
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack('HHHH', rows, columns, 0, 0))
    before = termios.tcgetattr(slave)
    read_fd = write_fd = None
    if case.get('stream_b64') is not None:
        read_fd, write_fd = os.pipe()
    pid = os.fork()
    if pid == 0:
        try:
            os.close(master)
            os.setsid()
            fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
            os.dup2(read_fd if read_fd is not None else slave, 0)
            os.dup2(slave, 1)
            os.dup2(slave, 2)
            if read_fd is not None:
                os.close(read_fd)
                os.close(write_fd)
            if slave > 2:
                os.close(slave)
            os.chdir(work)
            os.execve(argv[0], argv, env)
        finally:
            os._exit(127)
    if read_fd is not None:
        os.close(read_fd)
        os.write(write_fd, base64.b64decode(case['stream_b64']))
    screen = Screen(columns, rows)
    stream = pyte.Stream(screen)
    decoder = codecs.getincrementaldecoder('utf8')('replace')
    transcript = bytearray()
    checkpoints = []
    status = None
    pending_terminal = bytearray()
    def terminal_feed(data):
        pending_terminal.extend(data)
        while pending_terminal:
            start_osc = pending_terminal.find(b'\x1b]')
            if start_osc < 0:
                keep = 1 if pending_terminal.endswith(b'\x1b') else 0
                end = len(pending_terminal)-keep
                stream.feed(decoder.decode(bytes(pending_terminal[:end])))
                del pending_terminal[:end]
                return
            if start_osc:
                stream.feed(decoder.decode(bytes(pending_terminal[:start_osc])))
                del pending_terminal[:start_osc]
            match = re.search(rb'\x07|\x1b\\', pending_terminal[2:])
            if match is None:
                return
            end = 2+match.end()
            payload = norm(bytes(pending_terminal[2:2+match.start()]))
            screen.osc.append(b64(payload))
            if payload.startswith(b'8;'):
                fields = payload.split(b';', 2)
                if len(fields) == 3:
                    screen.cursor.attrs = LinkedChar(**screen.cursor.attrs._asdict())._replace(link=b64(payload) if fields[2] else '')
            stream.feed(decoder.decode(bytes(pending_terminal[:end])))
            del pending_terminal[:end]
    def receive(timeout):
        ready, _, _ = select.select([master], [], [], timeout)
        if not ready:
            return False
        try:
            data = os.read(master, 65536)
        except OSError as error:
            if error.errno != errno.EIO:
                raise
            return False
        if not data:
            return False
        transcript.extend(data)
        terminal_feed(data)
        return True
    def wait(patterns, start):
        def input_checkpoint():
            # A prompt can be printed between two queued commands, or followed
            # by a repaint split across writes. Wait until the Linux child is
            # blocked for input, with the terminal input queue consumed. This
            # is a semantic checkpoint, not an arbitrary idle/sleep duration.
            queued = struct.unpack('i', fcntl.ioctl(slave, termios.FIONREAD, struct.pack('i', 0)))[0]
            if queued:
                return False
            proc = Path('/proc')/str(pid)
            state = (proc/'stat').read_text().rsplit(')', 1)[1].split()[0]
            if state == 'Z':
                return True
            if state != 'S':
                return False
            call = (proc/'syscall').read_text().split()[0]
            # Linux x86_64 read, poll, select, pselect6 and ppoll.
            allowed = ('0', '7', '23', '270', '271')
            if 'Waiting for data' in patterns:
                # Follow mode also waits between probes with nanosleep.
                allowed += ('35', '230')
            return call in allowed

        deadline = time.monotonic() + case.get('timeout', 5)
        while time.monotonic() < deadline:
            tail = norm(bytes(transcript[start:]))
            if any(p.encode() in tail for p in patterns) and input_checkpoint():
                # Drain already queued bytes; no speculative sleep or idle readiness.
                while receive(0):
                    pass
                return
            receive(min(0.1, max(0, deadline-time.monotonic())))
        raise TimeoutError(f"{case['id']}: checkpoint {patterns!r}; tail={norm(bytes(transcript[-400:]))!r}")
    try:
        start = 0
        for event in case.get('events', []):
            def fixture_path(name):
                path = (work/name).resolve()
                if not path.is_relative_to(work.resolve()):
                    raise ValueError('event file must stay within the case directory')
                return path

            for operation, mode in (('append_file', 'ab'), ('replace_file', 'wb')):
                if operation in event:
                    start = len(transcript)
                    change = event[operation]
                    data = base64.b64decode(change['base64']) if 'base64' in change else change['text'].encode()
                    with fixture_path(change['path']).open(mode) as file:
                        file.write(data)
            if 'rotate_file' in event:
                start = len(transcript)
                change = event['rotate_file']
                fixture_path(change['path']).rename(fixture_path(change['to']))
                fixture_path(change['path']).write_bytes(change['text'].encode())
            if 'write_stream_b64' in event:
                start = len(transcript)
                data = base64.b64decode(event['write_stream_b64'])
                if write_fd is None or len(data) > os.fpathconf(write_fd, 'PC_PIPE_BUF'):
                    raise ValueError('stream update requires an open pipe and at most PIPE_BUF bytes')
                os.write(write_fd, data)
            if 'send' in event or 'send_b64' in event:
                start = len(transcript)
                data = base64.b64decode(event['send_b64']) if 'send_b64' in event else event['send'].replace('{ROOT}', str(work)).encode()
                os.write(master, data)
            if 'signal' in event:
                start = len(transcript)
                os.kill(pid, getattr(signal, event['signal']))
            if 'resize' in event:
                start = len(transcript)
                height, width = event['resize']
                old_size = fcntl.ioctl(slave, termios.TIOCGWINSZ, b'\0'*8)
                new_size = struct.pack('HHHH', height, width, 0, 0)
                fcntl.ioctl(slave, termios.TIOCSWINSZ, new_size)
                screen.resize(height, width)
                # Linux signals the foreground process group when winsize
                # changes. A second explicit signal can arrive after redraw
                # and consume the next command during another repaint.
                if old_size == new_size:
                    os.kill(pid, signal.SIGWINCH)
            if event.get('close_stream'):
                os.close(write_fd)
                write_fd = None
            if 'await' in event:
                wait(event['await'], start)
            if event.get('snapshot'):
                checkpoints.append(screen.state())
        deadline = time.monotonic() + case.get('timeout', 5)
        while time.monotonic() < deadline:
            receive(0.01)
            waited, value = os.waitpid(pid, os.WNOHANG)
            if waited:
                status = os.waitstatus_to_exitcode(value)
                while receive(0):
                    pass
                break
        if status is None:
            raise TimeoutError(f"{case['id']}: child did not exit")
        after = termios.tcgetattr(slave)
        # Paths printed in terminal rows require the same fixture normalization.
        for checkpoint in checkpoints:
            checkpoint['rows'] = [line.replace(str(work), '{ROOT}') for line in checkpoint['rows']]
        final = screen.state()
        final['rows'] = [line.replace(str(work), '{ROOT}') for line in final['rows']]
        return dict(status=status, checkpoints=checkpoints, final_terminal=final,
                    termios_restored=before == after)
    finally:
        if status is None:
            try:
                os.kill(pid, signal.SIGKILL)
                os.waitpid(pid, 0)
            except (ProcessLookupError, ChildProcessError):
                pass
        if write_fd is not None:
            os.close(write_fd)
        os.close(master)
        os.close(slave)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['capture', 'replay', 'self-check'])
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--cases', type=Path, default=ROOT/'compat/cases/options.json')
    parser.add_argument('--expected', type=Path, default=ROOT/'compat/expected-options.json')
    parser.add_argument('--results', type=Path, default=ROOT/'compat/results-local.json')
    parser.add_argument('--filter', default='')
    args = parser.parse_args()
    binary = args.binary.resolve()
    cases = [x for x in load(args.cases) if args.filter in x['id']]
    if not cases:
        parser.error('selection contains no cases')
    if args.operation == 'capture':
        ref = load(ROOT/'compat/reference.json')
        if hashlib.sha256(binary.read_bytes()).hexdigest() != ref['binary_sha256']:
            parser.error('capture requires the frozen reference executable')
        if args.expected.exists():
            parser.error('refusing to replace existing expectations; choose a new --expected path and review differences')
    expected = {} if args.operation == 'capture' else replay_expectations(args.expected)
    output = dict(executed=0, passed=0, failed=0, errors=[], cases={})
    for case in cases:
        try:
            actual = run_case(binary, case)
            if args.operation == 'capture':
                expected[case['id']] = actual
                ok = True
            else:
                ok = actual == expected.get(case['id'])
            if args.operation == 'self-check':
                mutated = copy.deepcopy(actual)
                mutated['status'] = 99 if actual['status'] != 99 else 98
                assert mutated != actual
                if actual.get('checkpoints'):
                    mutated = copy.deepcopy(actual)
                    mutated['checkpoints'][0]['rows'][0] = 'DELIBERATE DIFFERENCE'
                    assert mutated != actual
            output['passed' if ok else 'failed'] += 1
            output['cases'][case['id']] = dict(passed=ok, actual=actual if not ok else None)
        except Exception as error:
            output['failed'] += 1
            output['errors'].append(dict(case=case['id'], error=str(error)))
            print(json.dumps(output['errors'][-1]), flush=True)
        output['executed'] += 1
        if output['executed'] % 50 == 0:
            print(json.dumps({k: output[k] for k in ('executed', 'passed', 'failed')}), flush=True)
    if args.operation == 'capture' and output['failed'] == 0:
        args.expected.write_text(json.dumps(expected, indent=2, ensure_ascii=False)+'\n')
    args.results.write_text(json.dumps(output, indent=2, ensure_ascii=False)+'\n')
    print(json.dumps({k: v for k, v in output.items() if k != 'cases'}))
    return bool(output['failed'])


if __name__ == '__main__':
    raise SystemExit(main())
