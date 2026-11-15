#!/usr/bin/env python3
"""Hold redirected stdout full and verify SIGTERM cancels the pager write."""

import argparse
import array
import fcntl
import hashlib
import json
import os
from pathlib import Path
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


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-output-pipe-") as directory:
        source = Path(directory) / "input"
        source.write_bytes(b"line 0123456789abcdef\n" * 100000)
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, close_fds=True,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
        )
        assert pager.stdout is not None and pager.stderr is not None
        try:
            fcntl.fcntl(pager.stdout.fileno(), fcntl.F_SETPIPE_SZ, 4096)
            capacity = fcntl.fcntl(pager.stdout.fileno(), fcntl.F_GETPIPE_SZ)
            deadline = time.monotonic() + 2
            wchan = None
            available = 0
            while pager.poll() is None and time.monotonic() < deadline:
                wchan = proc_text(pager.pid, "wchan")
                count = array.array('i', [0])
                fcntl.ioctl(pager.stdout.fileno(), termios.FIONREAD, count, True)
                available = count[0]
                if available > 0 and (wchan == "anon_pipe_write" or "poll" in (wchan or "")):
                    break
                time.sleep(0.01)
            output_stalled = available > 0 and (
                wchan == "anon_pipe_write" or "poll" in (wchan or "")
            )
            if pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            status = pager.poll()
            return {
                "output_stalled": output_stalled,
                "pipe_queued_bytes": available,
                "pipe_capacity": capacity,
                "wchan_before_term": wchan,
                "status_after_term": status,
                "wchan_after_term": proc_text(pager.pid, "wchan") if status is None else None,
                "stdout_flags": fcntl.fcntl(pager.stdout.fileno(), fcntl.F_GETFL),
            }
        finally:
            if pager.poll() is None:
                pager.kill()
                pager.wait(timeout=2)
            pager.stdout.close()
            pager.stderr.close()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary) for _ in range(args.rounds)]
    def valid(case: dict) -> bool:
        return (case["output_stalled"] and
                (not args.expect_blocked or case["wchan_before_term"] == "anon_pipe_write") and
                case["status_after_term"] == (None if args.expect_blocked else 15) and
                (not args.expect_blocked or case["wchan_after_term"] == "anon_pipe_write") and
                not (case["stdout_flags"] & os.O_NONBLOCK))
    passed_count = sum(valid(case) for case in cases)
    report = {
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "test_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "rounds": args.rounds, "expect_blocked": args.expect_blocked,
        "passed_count": passed_count, "passed": passed_count == args.rounds,
        "cases": cases,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "cases"}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
