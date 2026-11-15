#!/usr/bin/env python3
"""Exercise cancellable FIFO open with real signals and a blocked worker."""

import argparse
import json
import os
from pathlib import Path
import pty
import select
import signal
import tempfile
import time


def children(pid: int) -> list[int]:
    try:
        return [int(value) for value in Path(f"/proc/{pid}/task/{pid}/children").read_text().split()]
    except FileNotFoundError:
        return []


def run_case(binary: Path, mode: str, ignore_sigchld: bool = False) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-fifo-open-") as directory:
        fifo = Path(directory, "input")
        os.mkfifo(fifo)
        pid, master = pty.fork()
        if pid == 0:
            if ignore_sigchld:
                signal.signal(signal.SIGCHLD, signal.SIG_IGN)
            env = dict(os.environ, TERM="xterm", LESSHISTFILE="-", LC_ALL="C")
            os.execve(str(binary), [str(binary), "-f", str(fifo)], env)
        reaped = False
        try:
            # A child blocked in the FIFO open proves that the signal hits the
            # intended path. The original implementation blocks in the pager.
            deadline = time.monotonic() + 3
            worker = None
            blocked_in_open = False
            while time.monotonic() < deadline:
                found = children(pid)
                if found:
                    worker = found[0]
                    try:
                        if Path(f"/proc/{worker}/wchan").read_text().strip() == "wait_for_partner":
                            blocked_in_open = True
                            break
                    except FileNotFoundError:
                        pass
                done, status = os.waitpid(pid, os.WNOHANG)
                if done:
                    reaped = True
                    raise AssertionError(f"pager exited before FIFO open: {status}")
                time.sleep(0.01)
            if worker is None or not blocked_in_open:
                raise AssertionError("no worker blocked in FIFO open")
            worker_fds = os.listdir(f"/proc/{worker}/fd")
            if set(worker_fds) != {"3", "4"}:
                raise AssertionError(f"worker retained unexpected descriptors: {worker_fds}")
            if mode == "stopped":
                os.kill(worker, signal.SIGSTOP)
            if mode == "resize":
                os.kill(pid, signal.SIGWINCH)
                time.sleep(0.02)
                writer = os.open(fifo, os.O_WRONLY | os.O_NONBLOCK)
                try:
                    os.write(writer, b"fifo payload\n")
                finally:
                    os.close(writer)
                time.sleep(0.03)
                os.write(master, b"q")
            else:
                os.kill(pid, signal.SIGTERM)
            deadline = time.monotonic() + 3
            transcript = bytearray()
            while time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.01)
                if ready:
                    try:
                        transcript.extend(os.read(master, 65536))
                    except OSError:
                        pass
                done, status = os.waitpid(pid, os.WNOHANG)
                if done:
                    reaped = True
                    break
            if not reaped:
                raise AssertionError(f"pager hung in {mode} case")
            code = os.waitstatus_to_exitcode(status)
            expected = 0 if mode == "resize" else 15
            if code != expected:
                raise AssertionError(f"{mode}: status {code}, expected {expected}")
            if mode == "resize" and b"fifo payload" not in transcript:
                raise AssertionError("resize case lost FIFO data")
            if Path(f"/proc/{worker}").exists():
                raise AssertionError("FIFO worker survived pager exit")
            return dict(mode=mode, status=code, worker_seen=True,
                        worker_fds=worker_fds, payload_seen=b"fifo payload" in transcript)
        finally:
            if not reaped:
                try:
                    os.kill(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                os.waitpid(pid, 0)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    results = []
    for _ in range(args.rounds):
        results.append(run_case(binary, "term"))
        results.append(run_case(binary, "term", ignore_sigchld=True))
        results.append(run_case(binary, "stopped"))
        results.append(run_case(binary, "resize"))
    report = dict(binary=str(binary), rounds=args.rounds, cases=results, passed=True)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"FIFO open: {len(results)}/{len(results)} cases passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
