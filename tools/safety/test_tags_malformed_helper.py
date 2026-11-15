#!/usr/bin/env python3
"""Malformed GNU Global tags output must not wait for an idle helper."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import subprocess
import tempfile
import termios
import time


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except FileNotFoundError:
        return False
    return not any(line.startswith("State:") and "Z" in line.split()[1]
                   for line in status.splitlines())


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-malformed-tags-") as directory:
        root = Path(directory)
        marker = root / "helper-pid"
        helper = root / "global-helper"
        helper.write_text(
            '#!/bin/sh\nprintf "%s" "$$" > "$RLESS_HELPER_TRACE"\n'
            'printf "malformed\\n"\nexec sleep 60\n'
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
        pager = subprocess.Popen(
            [str(binary), "-TGTAGS", "-tentry"], stdin=slave, stdout=slave,
            stderr=slave, start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.02)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            helper_pid = int(marker.read_text()) if marker.exists() else None
            deadline = time.monotonic() + 1
            while pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.02)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            return {
                "helper_hit": helper_pid is not None,
                "helper_alive": alive(helper_pid) if helper_pid is not None else None,
                "status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "output_tail": bytes(output[-180:]).decode("latin-1"),
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    case = observe(binary)
    if args.expect_vulnerable:
        passed = case["helper_hit"] and case["helper_alive"] and case["status"] is None
    else:
        passed = (case["helper_hit"] and not case["helper_alive"]
                  and case["status"] is not None and case["terminal_restored"])
    result = {
        "binary_sha256": digest(binary), "expect_vulnerable": args.expect_vulnerable,
        "case": case, "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
