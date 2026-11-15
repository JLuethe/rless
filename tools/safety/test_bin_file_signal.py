#!/usr/bin/env python3
"""Positive-controlled signal-before-read race in the binary-file probe."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import signal
import subprocess
import tempfile
import termios
import time


SHIM = Path(__file__).with_name('native') / 'bin_file_signal.c'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--expect-blocked', action='store_true')
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix='rless-bin-file-signal-') as directory:
        root = Path(directory)
        shim = root / 'bin_file_signal.so'
        subprocess.run(['cc', '-shared', '-fPIC', '-O2', '-Wall', '-Wextra', '-Werror',
                        str(SHIM), '-ldl', '-o', str(shim)], check=True,
                       capture_output=True)
        source = root / 'regular-input'
        source.write_bytes(b'line\n' * 200)
        marker = root / 'probe-hit'
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            'PATH': '/usr/bin:/bin', 'HOME': directory, 'TERM': 'xterm-256color',
            'LC_ALL': 'C', 'LESSHISTFILE': '-', 'LD_PRELOAD': str(shim),
            'RLESS_BIN_FILE_SIGNAL_TRACE': str(marker),
        }
        pager = subprocess.Popen([str(binary), str(source)], stdin=slave,
                                 stdout=slave, stderr=slave, start_new_session=True,
                                 close_fds=True, env=env)
        os.close(slave)
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            kind = marker.read_text() if marker.exists() else None
            deadline = time.monotonic() + 0.75
            while pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            status = pager.poll()
            try:
                wchan = Path(f'/proc/{pager.pid}/wchan').read_text().strip() if status is None else None
            except FileNotFoundError:
                wchan = None
            restored = termios.tcgetattr(master) == original
            passed = (kind == ('R' if args.expect_blocked else 'P')
                      and (status is None if args.expect_blocked else status == 15)
                      and (wchan == 'anon_pipe_read' if args.expect_blocked else restored))
            report = {
                'binary_sha256': sha(binary.read_bytes()),
                'shim_source_sha256': sha(SHIM.read_bytes()),
                'shim_sha256': sha(shim.read_bytes()),
                'hook': kind, 'pager_status': status,
                'wait_channel': wchan, 'terminal_restored': restored,
                'expect_blocked': args.expect_blocked,
                'expected_observed': passed,
            }
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(report, indent=2) + '\n')
            print(json.dumps(report))
            return 0 if passed else 1
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


if __name__ == '__main__':
    raise SystemExit(main())
