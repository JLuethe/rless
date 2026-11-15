#!/usr/bin/env python3
"""Check that SIGTERM during a pipe handoff does not orphan its helper."""

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


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except (FileNotFoundError, ProcessLookupError):
        return False
    return not any(
        line.startswith("State:") and "Z" in line.split()[1]
        for line in status.splitlines()
    )


def process_group(pid: int) -> int | None:
    try:
        return os.getpgid(pid)
    except ProcessLookupError:
        return None


def observe(binary: Path, termination: signal.Signals) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-pipe-handoff-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 200)
        marker = root / "helper-hit"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\ntrap "" HUP\n'
            'printf "hit %s" "$$" > "$RLESS_HELPER_TRACE"\nexec sleep 30\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "RLESS_HELPER_TRACE": str(marker),
        }
        process = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        helper_pid = None
        try:
            deadline = time.monotonic() + 2
            output = bytearray()
            while time.monotonic() < deadline and process.poll() is None:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
                if b"line" in output:
                    break
            if process.poll() is None:
                os.write(master, b"|." + os.fsencode(helper) + b"\n")
            deadline = time.monotonic() + 2
            while not marker.exists() and process.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            hit = marker.read_bytes() if marker.exists() else b""
            if hit.startswith(b"hit "):
                helper_pid = int(hit.split()[1])
            live_before = helper_pid is not None and alive(helper_pid)
            helper_group_before = process_group(helper_pid) if helper_pid is not None else None
            try:
                supervisor_children = (Path(f"/proc/{process.pid}/task/{process.pid}/children")
                                       .read_text().split())
            except (FileNotFoundError, ProcessLookupError):
                supervisor_children = []
            if live_before and process.poll() is None:
                os.kill(process.pid, termination)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            deadline = time.monotonic() + 0.5
            while helper_pid is not None and alive(helper_pid) and time.monotonic() < deadline:
                time.sleep(0.01)
            return {
                "helper_hit": helper_pid is not None,
                "helper_live_before_signal": live_before,
                "helper_alive_after_signal": alive(helper_pid) if helper_pid is not None else None,
                "helper_group_before_signal": helper_group_before,
                "helper_group_after_signal": process_group(helper_pid) if helper_pid is not None else None,
                "supervisor_children_before_signal": supervisor_children,
                "pager_status": process.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
        finally:
            if helper_pid is not None:
                try:
                    os.kill(helper_pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
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
    parser.add_argument("--rounds", type=int, default=20)
    parser.add_argument("--signal", choices=("SIGTERM", "SIGKILL"), default="SIGTERM")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary, getattr(signal, args.signal)) for _ in range(args.rounds)]
    passed = sum(
        case["helper_hit"] and case["helper_live_before_signal"]
        and not case["helper_alive_after_signal"]
        and case["pager_status"] is not None and case["terminal_restored"]
        for case in cases
    )
    result = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "signal": args.signal,
        "rounds": args.rounds,
        "passed": passed,
        "cases": cases,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "cases"}))
    return 0 if passed == args.rounds else 1


if __name__ == "__main__":
    raise SystemExit(main())
