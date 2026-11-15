#!/usr/bin/env python3
"""Fill PTY output, deliver SIGTERM and verify bounded terminal cleanup."""

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


def proc_text(pid: int, name: str) -> str | None:
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError):
        return None


def observe(binary: Path, delivery: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-pty-backpressure-") as directory:
        source = Path(directory) / "input"
        source.write_bytes(b"line 0123456789abcdef\n" * 100000)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        pager = subprocess.Popen(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", str(source)],
            stdin=slave, stdout=slave, stderr=slave, start_new_session=True,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
        )
        os.close(slave)
        try:
            output = bytearray()
            deadline = time.monotonic() + 3
            while b"READY" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            if pager.poll() is None:
                os.write(master, b"j" * 1000)
            time.sleep(0.25)
            before = proc_text(pager.pid, "syscall") if pager.poll() is None else None
            before_wait = proc_text(pager.pid, "wchan") if pager.poll() is None else None
            fdinfo = proc_text(pager.pid, "fdinfo/1") if pager.poll() is None else None
            flags_line = next((line for line in fdinfo.splitlines()
                               if line.startswith("flags:")), None) if fdinfo else None
            original_output_nonblocking = (bool(int(flags_line.split()[1], 8) & os.O_NONBLOCK)
                                           if flags_line else None)
            if pager.poll() is None:
                os.kill(pager.pid, signal.SIGWINCH if delivery == "winch" else signal.SIGTERM)
            try:
                pager.wait(timeout=0.25 if delivery == "winch" else 1)
            except subprocess.TimeoutExpired:
                pass
            after_signal = pager.poll()
            resumed_output = bytearray()
            if delivery == "winch" and pager.poll() is None:
                os.write(master, b"q")
                deadline = time.monotonic() + 4
                while pager.poll() is None and time.monotonic() < deadline:
                    readable, _, _ = select.select([master], [], [], 0.05)
                    if readable:
                        try:
                            resumed_output.extend(os.read(master, 65536))
                        except OSError:
                            break
                if pager.poll() is None:
                    os.write(master, b"q")
                    try:
                        pager.wait(timeout=1)
                    except subprocess.TimeoutExpired:
                        pass
            result = {
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "signal": delivery,
                "ready": b"READY" in output,
                "syscall_before_signal": before,
                "wchan_before_signal": before_wait,
                "original_output_nonblocking": original_output_nonblocking,
                "status_after_signal": after_signal,
                "status_after_resume": pager.poll(),
                "resumed_output_bytes": len(resumed_output),
                "syscall_after_signal": proc_text(pager.pid, "syscall")
                    if pager.poll() is None else None,
                "terminal_restored": termios.tcgetattr(master) == original,
            }
            return result
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
    parser.add_argument("--signal", choices=("term", "winch"), default="term")
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    case = observe(binary, args.signal)
    if args.signal == "winch":
        passed = (case["ready"] and case["status_after_signal"] is None
                  and case["status_after_resume"] == 0
                  and case["resumed_output_bytes"] > 0
                  and case["original_output_nonblocking"] is False
                  and case["terminal_restored"])
    elif args.expect_blocked:
        passed = (case["ready"] and case["syscall_before_signal"] is not None
                  and case["syscall_before_signal"].startswith("1 ")
                  and case["status_after_signal"] is None
                  and case["original_output_nonblocking"] is False
                  and not case["terminal_restored"])
    else:
        passed = (case["ready"] and case["status_after_signal"] == 15
                  and case["original_output_nonblocking"] is False
                  and case["terminal_restored"])
    case["expected_outcome_observed"] = bool(passed)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(case, indent=2) + "\n")
    print(json.dumps(case))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
