#!/usr/bin/env python3
"""Verify that a shell helper inherits the pager's original SIGTERM disposition."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import tempfile
import time


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-signal-handoff-") as directory:
        work = Path(directory)
        source = work / "input"
        source.write_bytes(b"line\n" * 30)
        marker = work / "disposition"
        helper = work / "helper"
        helper.write_text(
            '#!/usr/bin/python3\n'
            'import os, signal\n'
            'open(os.environ["RLESS_SIGNAL_MARKER"], "w").write('
            'str(signal.getsignal(signal.SIGTERM)))\n'
        )
        helper.chmod(0o700)
        pid, master = pty.fork()
        if pid == 0:
            signal.signal(signal.SIGTERM, signal.SIG_IGN)
            environment = dict(
                HOME=directory,
                TERM="xterm",
                LC_ALL="C",
                PATH="/usr/bin:/bin",
                LESSHISTFILE="-",
                RLESS_SIGNAL_MARKER=str(marker),
            )
            os.execve(str(binary), [str(binary), "-P@@READY@@", str(source)], environment)
        transcript = bytearray()
        status = None
        reaped = False
        try:
            def read_for(seconds: float) -> None:
                ready, _, _ = select.select([master], [], [], seconds)
                if ready:
                    try:
                        transcript.extend(os.read(master, 65536))
                    except OSError:
                        pass

            deadline = time.monotonic() + 5
            while b"@@READY@@" not in transcript and time.monotonic() < deadline:
                read_for(0.05)
            if b"@@READY@@" not in transcript:
                raise TimeoutError("pager startup")
            os.write(master, ("!" + str(helper) + "\n").encode())
            deadline = time.monotonic() + 5
            while not marker.exists() and time.monotonic() < deadline:
                read_for(0.05)
            if not marker.exists():
                raise TimeoutError("shell helper")
            for _ in range(20):
                try:
                    os.write(master, b"\nq")
                except OSError:
                    pass
                read_for(0.05)
                done, status = os.waitpid(pid, os.WNOHANG)
                if done:
                    reaped = True
                    break
            if not reaped:
                raise TimeoutError("pager exit")
            return dict(
                disposition=int(marker.read_text()),
                status=os.waitstatus_to_exitcode(status),
                helper_ran=True,
            )
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
    parser.add_argument("--baseline", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    candidate = observe(binary)
    report = dict(
        binary=str(binary),
        binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
        candidate=candidate,
        passed=candidate == dict(disposition=1, status=0, helper_ran=True),
    )
    if args.baseline is not None:
        baseline_binary = args.baseline.resolve()
        baseline = observe(baseline_binary)
        report.update(
            baseline_binary=str(baseline_binary),
            baseline_sha256=hashlib.sha256(baseline_binary.read_bytes()).hexdigest(),
            baseline=baseline,
        )
        report["passed"] &= baseline == dict(disposition=0, status=0, helper_ran=True)
    if args.output is not None:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
