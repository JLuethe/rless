#!/usr/bin/env python3
"""Hold a redirected Unix socket full and verify SIGTERM cancels output."""

import argparse
import array
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import tempfile
import termios
import time


def wchan(pid: int) -> str | None:
    try:
        return Path(f"/proc/{pid}/wchan").read_text().strip()
    except (FileNotFoundError, ProcessLookupError):
        return None


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-output-socket-") as directory:
        source = Path(directory) / "input"
        source.write_bytes(b"line 0123456789abcdef\n" * 100000)
        writer, reader = socket.socketpair()
        writer.setsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF, 4096)
        send_capacity = writer.getsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF)
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=subprocess.DEVNULL,
            stdout=writer, stderr=subprocess.DEVNULL, close_fds=True,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
        )
        try:
            deadline = time.monotonic() + 2
            queued = 0
            before = None
            while pager.poll() is None and time.monotonic() < deadline:
                before = wchan(pager.pid)
                count = array.array("i", [0])
                fcntl.ioctl(writer.fileno(), termios.TIOCOUTQ, count, True)
                queued = count[0]
                if queued >= send_capacity and (
                    "poll" in (before or "") or before == "sock_alloc_send_pskb"
                ):
                    break
                time.sleep(0.01)
            stalled = queued >= send_capacity and (
                "poll" in (before or "") or before == "sock_alloc_send_pskb"
            )
            if pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            status = pager.poll()
            return {
                "output_stalled": stalled,
                "socket_queued_bytes": queued,
                "socket_send_capacity": send_capacity,
                "wchan_before_term": before,
                "status_after_term": status,
                "wchan_after_term": wchan(pager.pid) if status is None else None,
                "stdout_flags": fcntl.fcntl(writer.fileno(), fcntl.F_GETFL),
            }
        finally:
            if pager.poll() is None:
                pager.kill()
                pager.wait(timeout=2)
            writer.close()
            reader.close()


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
                case["status_after_term"] == (None if args.expect_blocked else 15) and
                (not args.expect_blocked or
                 case["wchan_after_term"] == "sock_alloc_send_pskb") and
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
