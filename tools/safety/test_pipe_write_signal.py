#!/usr/bin/env python3
"""Stall a pipe handoff write with inherited SIGTERM ignored."""

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
    return not any(line.startswith("State:") and "Z" in line.split()[1]
                   for line in status.splitlines())


def observe(binary: Path, delivered: signal.Signals) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-pipe-write-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line 0123456789abcdef\n" * 100000)
        marker = root / "helper-hit"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\ntrap "" HUP TERM\n'
            'printf "%s" "$$" > "$RLESS_HELPER_TRACE"\nexec sleep 30\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "RLESS_HELPER_TRACE": str(marker),
        }

        def inherited_ignore() -> None:
            signal.signal(signal.SIGTERM, signal.SIG_IGN)

        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
            preexec_fn=inherited_ignore,
        )
        os.close(slave)
        helper_pid = None
        try:
            time.sleep(0.2)
            os.write(master, b"Gmag|a" + os.fsencode(helper) + b"\n")
            deadline = time.monotonic() + 5
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                readable, _, _ = select.select([master], [], [], 0.05)
                if readable:
                    try:
                        os.read(master, 65536)
                    except OSError:
                        break
            if marker.exists():
                helper_pid = int(marker.read_text())
            wchan = None
            deadline = time.monotonic() + 3
            while pager.poll() is None and time.monotonic() < deadline:
                wchan = Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                if wchan == "anon_pipe_write":
                    break
                time.sleep(0.01)
            live_before = pager.poll() is None and helper_pid is not None and alive(helper_pid)
            if live_before:
                os.kill(pager.pid, delivered)
                try:
                    if delivered == signal.SIGTERM:
                        pager.wait(timeout=1)
                    else:
                        time.sleep(0.5)
                except subprocess.TimeoutExpired:
                    pass
            after = (Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                     if pager.poll() is None else None)
            status_after_signal = pager.poll()
            helper_after_signal = alive(helper_pid) if helper_pid else None
            status_after_quit = None
            if delivered == signal.SIGWINCH and status_after_signal is None:
                os.write(master, b"q")
                try:
                    pager.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
                status_after_quit = pager.poll()
            return {
                "helper_hit": helper_pid is not None,
                "helper_alive_before_signal": live_before,
                "wait_channel_before_signal": wchan,
                "status_after_signal": status_after_signal,
                "wait_channel_after_signal": after,
                "helper_alive_after_signal": helper_after_signal,
                "status_after_quit": status_after_quit,
                "terminal_restored": termios.tcgetattr(master) == original,
            }
        finally:
            if helper_pid is not None:
                try:
                    os.kill(helper_pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
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
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--expect-reference", action="store_true")
    parser.add_argument("--expect-resize", action="store_true")
    parser.add_argument("--signal", choices=("SIGTERM", "SIGWINCH"), default="SIGTERM")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    delivered = getattr(signal, args.signal)
    case = observe(binary, delivered)
    common = case["helper_hit"] and case["helper_alive_before_signal"]
    if sum((args.expect_blocked, args.expect_reference, args.expect_resize)) > 1:
        parser.error("choose one expected outcome")
    if args.expect_blocked:
        passed = (common and case["wait_channel_before_signal"] == "anon_pipe_write"
                  and case["status_after_signal"] is None
                  and case["wait_channel_after_signal"] == "anon_pipe_write")
    elif args.expect_reference:
        passed = (common and case["wait_channel_before_signal"] == "anon_pipe_write"
                  and case["status_after_signal"] == -signal.SIGTERM
                  and case["helper_alive_after_signal"]
                  and case["terminal_restored"])
    elif args.expect_resize:
        passed = (common and case["status_after_signal"] is None
                  and not case["helper_alive_after_signal"]
                  and case["status_after_quit"] == 0
                  and case["terminal_restored"])
    else:
        passed = (common and case["status_after_signal"] == 15
                  and not case["helper_alive_after_signal"]
                  and case["terminal_restored"])
    result = {
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "expect_blocked": args.expect_blocked,
        "expect_reference": args.expect_reference,
        "expect_resize": args.expect_resize,
        "signal": args.signal,
        "case": case,
        "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
