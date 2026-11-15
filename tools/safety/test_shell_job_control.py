#!/usr/bin/env python3
"""Check shell foreground ownership with a real parent in the pager session."""

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import tempfile
import termios
import time


def alive(pid: int) -> bool:
    try:
        state = Path(f'/proc/{pid}/status').read_text()
    except (FileNotFoundError, ProcessLookupError):
        return False
    return not any(line.startswith('State:') and line.split()[1] in ('Z', 'X', 'x')
                   for line in state.splitlines())


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix='rless-shell-job-') as directory:
        root = Path(directory)
        source = root / 'input'
        source.write_text('line\n' * 80)
        marker = root / 'helper-hit'
        helper = root / 'helper'
        helper.write_text('#!/bin/sh\ntrap "" HUP\n'
                          'printf "hit %s" "$$" > "$RLESS_HELPER_TRACE"\n'
                          'exec sleep 30\n')
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        control_read, control_write = os.pipe()
        report_read, report_write = os.pipe()
        supervisor = os.fork()
        if supervisor == 0:
            try:
                os.close(master)
                os.close(control_write)
                os.close(report_read)
                os.setsid()
                fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
                signal.signal(signal.SIGTTOU, signal.SIG_IGN)
                gate_read, gate_write = os.pipe()
                pager = os.fork()
                if pager == 0:
                    os.close(gate_write)
                    os.close(control_read)
                    os.close(report_write)
                    os.setpgid(0, 0)
                    os.read(gate_read, 1)
                    os.close(gate_read)
                    for fd in (0, 1, 2):
                        os.dup2(slave, fd)
                    if slave > 2:
                        os.close(slave)
                    os.chdir(directory)
                    env = dict(PATH='/usr/bin:/bin', HOME=directory,
                               TERM='xterm-256color', LC_ALL='C',
                               LESSHISTFILE='-', RLESS_HELPER_TRACE=str(marker))
                    os.execve(str(binary), [str(binary), '-PREADY', str(source)], env)
                os.close(gate_read)
                os.setpgid(pager, pager)
                os.tcsetpgrp(slave, pager)
                os.write(gate_write, b'g')
                os.close(gate_write)

                def send(value):
                    os.write(report_write, (json.dumps(value) + '\n').encode())

                send({'pager': pager})
                assert os.read(control_read, 1) == b's'
                send({'foreground_during_shell': os.tcgetpgrp(slave)})
                assert os.read(control_read, 1) == b't'
                os.kill(pager, signal.SIGTERM)
                _, status = os.waitpid(pager, 0)
                send({'pager_status': os.waitstatus_to_exitcode(status),
                      'termios_restored': termios.tcgetattr(slave) == original,
                      'foreground_after_exit': os.tcgetpgrp(slave)})
                os.tcsetpgrp(slave, os.getpgrp())
                send({'parent_reclaimed_terminal': os.tcgetpgrp(slave) == os.getpgrp()})
                os._exit(0)
            except BaseException as error:
                os.write(report_write, (json.dumps({'error': repr(error)}) + '\n').encode())
                os._exit(1)

        os.close(slave)
        os.close(control_read)
        os.close(report_write)
        pending = bytearray()
        transcript = bytearray()
        pager = None
        helper_pid = None
        helper_group = None
        reaped = False

        def read_ready(timeout=5):
            ready, _, _ = select.select([master, report_read], [], [], timeout)
            if master in ready:
                try:
                    transcript.extend(os.read(master, 65536))
                except OSError:
                    pass
            if report_read in ready:
                pending.extend(os.read(report_read, 65536))

        def report():
            deadline = time.monotonic() + 5
            while b'\n' not in pending:
                if time.monotonic() >= deadline:
                    raise TimeoutError('job-control supervisor report')
                read_ready(0.05)
            line, _, rest = pending.partition(b'\n')
            pending[:] = rest
            value = json.loads(line)
            if 'error' in value:
                raise RuntimeError(value['error'])
            return value

        try:
            pager = report()['pager']
            deadline = time.monotonic() + 5
            while b'READY' not in transcript:
                if time.monotonic() >= deadline:
                    raise TimeoutError('pager startup')
                read_ready(0.05)
            os.write(master, b'!' + os.fsencode(helper) + b'\n')
            deadline = time.monotonic() + 5
            while not marker.exists():
                if time.monotonic() >= deadline:
                    raise TimeoutError('shell helper start')
                read_ready(0.05)
            helper_pid = int(marker.read_bytes().split()[1])
            helper_group = os.getpgid(helper_pid)
            was_live = alive(helper_pid)
            os.write(control_write, b's')
            foreground = report()['foreground_during_shell']
            os.write(control_write, b't')
            exited = report()
            reclaimed = report()['parent_reclaimed_terminal']
            deadline = time.monotonic() + 0.5
            while alive(helper_pid) and time.monotonic() < deadline:
                time.sleep(0.01)
            helper_status = None
            if alive(helper_pid):
                try:
                    helper_status = {
                        line.split(':', 1)[0]: line.split(':', 1)[1].strip()
                        for line in Path(f'/proc/{helper_pid}/status').read_text().splitlines()
                        if line.startswith(('Name:', 'State:', 'Pid:', 'PPid:'))
                    }
                    helper_status['group'] = os.getpgid(helper_pid)
                    helper_status['wchan'] = Path(f'/proc/{helper_pid}/wchan').read_text().strip()
                except (FileNotFoundError, ProcessLookupError):
                    helper_status = {'disappeared_before_capture': True}
            result = {
                'pager_status': exited['pager_status'],
                'helper_live_before_signal': was_live,
                'helper_alive_after_signal': alive(helper_pid),
                'helper_status_after_signal': helper_status,
                'shell_foreground': foreground == helper_group,
                'parent_reclaimed_terminal': reclaimed,
                'termios_restored': exited['termios_restored'],
                'foreground_after_exit': exited['foreground_after_exit'],
                'pager_group': pager,
            }
            _, status = os.waitpid(supervisor, 0)
            reaped = True
            result['supervisor_status'] = os.waitstatus_to_exitcode(status)
            return result
        finally:
            if helper_group is not None:
                try:
                    os.killpg(helper_group, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            if pager is not None:
                try:
                    os.killpg(pager, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            if not reaped:
                try:
                    os.kill(supervisor, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                os.waitpid(supervisor, 0)
            os.close(control_write)
            os.close(report_read)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--rounds', type=int, default=1)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    results = [observe(binary) for _ in range(args.rounds)]
    passed = all(result['pager_status'] == -signal.SIGTERM
                 and result['helper_live_before_signal']
                 and not result['helper_alive_after_signal']
                 and result['shell_foreground']
                 and result['parent_reclaimed_terminal']
                 and result['termios_restored']
                 and result['supervisor_status'] == 0
                 for result in results)
    report = {'binary': str(binary),
              'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
              'rounds': args.rounds, 'passed': passed, 'results': results}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({key: value for key, value in report.items()
                      if key != 'results'}))
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
