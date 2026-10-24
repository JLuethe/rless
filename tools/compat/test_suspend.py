#!/usr/bin/env python3
"""Exercise real job-control suspension with a parent in the pager's session."""
import argparse
import codecs
import fcntl
import json
import os
from pathlib import Path
import pty
import select
import signal
import struct
import tempfile
import termios
import time

import pyte
from run_v2 import Screen


def observe(binary):
    with tempfile.TemporaryDirectory(prefix='rless-job-control-') as directory:
        work = Path(directory)
        (work/'data').write_text('line\n'*50)
        master, slave = pty.openpty()
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack('HHHH', 24, 80, 0, 0))
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
                child = os.fork()
                if child == 0:
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
                    os.execve(str(binary), [str(binary), '-P@@READY@@', 'data'],
                              dict(PATH='/usr/bin:/bin', HOME=directory, LC_ALL='C',
                                   TERM='xterm-256color', LESSHISTFILE='-'))
                os.close(gate_read)
                os.setpgid(child, child)
                os.tcsetpgrp(slave, child)
                os.write(gate_write, b'g')
                os.close(gate_write)

                def send(record):
                    os.write(report_write, (json.dumps(record)+'\n').encode())

                send({'child': child})
                assert os.read(control_read, 1) == b's'
                os.kill(child, signal.SIGTSTP)
                _, status = os.waitpid(child, os.WUNTRACED)
                assert os.WIFSTOPPED(status)
                send(dict(stopped_by=os.WSTOPSIG(status), restored=termios.tcgetattr(slave) == original))
                assert os.read(control_read, 1) == b'c'
                os.kill(child, signal.SIGCONT)
                _, status = os.waitpid(child, os.WCONTINUED)
                assert os.WIFCONTINUED(status)
                send({'continued': True})
                assert os.read(control_read, 1) == b'e'
                _, status = os.waitpid(child, 0)
                send(dict(status=os.waitstatus_to_exitcode(status), restored=termios.tcgetattr(slave) == original))
                os._exit(0)
            except BaseException as error:
                os.write(report_write, (json.dumps({'error': repr(error)})+'\n').encode())
                os._exit(1)
        os.close(control_read)
        os.close(report_write)
        screen = Screen(80, 24)
        stream = pyte.Stream(screen)
        decoder = codecs.getincrementaldecoder('utf8')('replace')
        transcript = bytearray()
        reports = bytearray()
        child = None
        reaped = False

        def receive(timeout):
            ready, _, _ = select.select([master, report_read], [], [], timeout)
            if master in ready:
                chunk = os.read(master, 65536)
                transcript.extend(chunk)
                stream.feed(decoder.decode(chunk))
            if report_read in ready:
                reports.extend(os.read(report_read, 65536))

        def report():
            deadline = time.monotonic()+5
            while b'\n' not in reports:
                if time.monotonic() >= deadline:
                    raise TimeoutError('job-control supervisor report')
                receive(0.01)
            line, _, rest = reports.partition(b'\n')
            reports[:] = rest
            record = json.loads(line)
            assert 'error' not in record, record
            return record

        def prompt(start):
            deadline = time.monotonic()+5
            while b'@@READY@@' not in transcript[start:]:
                if time.monotonic() >= deadline:
                    raise TimeoutError('pager did not redraw after job-control event')
                receive(0.01)
            while select.select([master], [], [], 0)[0]:
                receive(0)

        try:
            child = report()['child']
            prompt(0)
            os.write(control_write, b's')
            stopped = report()
            while select.select([master], [], [], 0)[0]:
                receive(0)
            stopped_screen = screen.state()
            assert stopped == {'stopped_by': int(signal.SIGTSTP), 'restored': True}, stopped
            assert not ({1, 1000, 1006, 1049, 2004} & set(stopped_screen['private_modes']))
            start = len(transcript)
            os.write(control_write, b'c')
            assert report() == {'continued': True}
            prompt(start)
            resumed = screen.state()
            assert 1049 in resumed['private_modes']
            os.write(master, b'q')
            os.write(control_write, b'e')
            exited = report()
            assert exited == {'status': 0, 'restored': True}, exited
            _, status = os.waitpid(supervisor, 0)
            reaped = True
            assert os.waitstatus_to_exitcode(status) == 0
            return dict(stopped=stopped, stopped_terminal=stopped_screen,
                        resumed_terminal=resumed, exited=exited)
        finally:
            if not reaped:
                if child is not None:
                    try:
                        os.killpg(child, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                try:
                    os.kill(supervisor, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                os.waitpid(supervisor, 0)
            for fd in (master, slave, control_write, report_read):
                os.close(fd)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--reference', required=True, type=Path)
    parser.add_argument('--results', required=True, type=Path)
    args = parser.parse_args()
    expected = observe(args.reference.resolve())
    actual = observe(args.binary.resolve())
    report = dict(passed=actual == expected, reference=expected, actual=actual)
    args.results.parent.mkdir(parents=True, exist_ok=True)
    args.results.write_text(json.dumps(report, indent=2)+'\n')
    print('Real SIGTSTP/SIGCONT job-control check:', report['passed'])
    return not report['passed']


if __name__ == '__main__':
    raise SystemExit(main())
