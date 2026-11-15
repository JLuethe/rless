#!/usr/bin/env python3
"""Positive-controlled SIGTERM reproducer for a stalled GNU Global tags helper."""

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


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except FileNotFoundError:
        return False
    return not any(
        line.startswith("State:") and "Z" in line.split()[1]
        for line in status.splitlines()
    )


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-tags-signal-") as directory:
        root = Path(directory)
        marker = root / "helper-hit"
        helper = root / "global-helper"
        helper.write_text(
            '#!/bin/sh\nprintf "hit %s" "$$" > "$RLESS_HELPER_TRACE"\nexec sleep 30\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LESSGLOBALTAGS": str(helper),
            "RLESS_HELPER_TRACE": str(marker),
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                env[name] = os.environ[name]
        process = subprocess.Popen(
            [str(binary), "-TGTAGS", "-tentry"], stdin=slave, stdout=slave,
            stderr=slave, start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and process.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            hit = marker.read_bytes() if marker.exists() else b""
            helper_pid = int(hit.split()[1]) if hit.startswith(b"hit ") else None
            if helper_pid is not None and process.poll() is None:
                os.kill(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            return {
                "binary": str(binary),
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "helper_sha256": hashlib.sha256(helper.read_bytes()).hexdigest(),
                "helper_hit": helper_pid is not None,
                "helper_pid": helper_pid,
                "helper_alive": alive(helper_pid) if helper_pid is not None else None,
                "status": process.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
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
    return 0 if (
        result["helper_hit"] and result["status"] == 15
        and not result["helper_alive"] and result["terminal_restored"]
    ) else 1


if __name__ == "__main__":
    raise SystemExit(main())
