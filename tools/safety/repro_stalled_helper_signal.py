#!/usr/bin/env python3
"""Bounded reproducer for SIGTERM stuck waiting for a LESSOPEN helper."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import signal
import subprocess
import tempfile
import time


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-helper-signal-") as directory:
        work = Path(directory)
        data = work / "file"
        data.write_bytes(b"data\n" * 50)
        trace = work / "helper-hit"
        helper = work / "helper"
        helper.write_text(
            '#!/bin/sh\nprintf hit > "$RLESS_HELPER_TRACE"\nexec sleep 30\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        env = dict(
            PATH="/usr/bin:/bin", HOME=directory, TERM="xterm-256color",
            LC_ALL="C", LESSHISTFILE="-",
            LESSOPEN=f"|{helper} %s", RLESS_HELPER_TRACE=str(trace),
        )
        process = subprocess.Popen(
            [str(binary), str(data)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        try:
            deadline = time.monotonic() + 3
            while (not trace.exists() and time.monotonic() < deadline
                   and process.poll() is None):
                time.sleep(0.01)
            hit = trace.read_bytes() if trace.exists() else b""
            if hit == b"hit" and process.poll() is None:
                os.kill(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            try:
                wchan = (Path(f"/proc/{process.pid}/wchan").read_text().strip()
                         if process.poll() is None else None)
            except FileNotFoundError:
                wchan = None
            return dict(
                binary=str(binary), binary_sha256=sha(binary.read_bytes()),
                helper_sha256=sha(helper.read_bytes()), helper_hit=hit == b"hit",
                status=process.poll(), wchan=wchan,
            )
        finally:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    result = observe(args.binary.resolve())
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if result["helper_hit"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
