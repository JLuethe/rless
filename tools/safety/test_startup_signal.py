#!/usr/bin/env python3
"""Require bounded startup termination while LESSMSG is stalled on a FIFO."""

import argparse
import errno
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time


def run_case(binary: Path, number: signal.Signals, ignored: bool) -> dict:
    with tempfile.TemporaryDirectory() as directory:
        directory = Path(directory)
        fifo = directory / "lessmsg"
        source = directory / "input"
        os.mkfifo(fifo)
        source.write_bytes(b"input\n")
        env = os.environ.copy()
        env.update({"LESSMSG": str(fifo), "LESSHISTFILE": "-", "TERM": "xterm-256color"})
        preexec = (lambda: signal.signal(number, signal.SIG_IGN)) if ignored else None
        child = subprocess.Popen(
            [str(binary), str(source)],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env,
            preexec_fn=preexec,
        )
        writer = None
        try:
            deadline = time.monotonic() + 3
            while time.monotonic() < deadline:
                try:
                    writer = os.open(fifo, os.O_WRONLY | os.O_NONBLOCK)
                    break
                except OSError as exc:
                    if exc.errno != errno.ENXIO:
                        raise
                    time.sleep(0.01)
            if writer is None:
                raise RuntimeError("LESSMSG reader never opened the FIFO")
            time.sleep(0.1)
            before = Path(f"/proc/{child.pid}/wchan").read_text().strip()
            child.send_signal(number)
            time.sleep(0.3)
            alive = child.poll() is None
            after = Path(f"/proc/{child.pid}/wchan").read_text().strip() if alive else None
            return {
                "inherited_signal_ignored": ignored,
                "signal": number.name,
                "wait_channel_before": before,
                "alive_300ms_after_signal": alive,
                "wait_channel_after": after,
                "returncode_after_signal": child.poll(),
            }
        finally:
            if writer is not None:
                os.close(writer)
            if child.poll() is None:
                child.kill()
            child.communicate(timeout=3)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    report = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "cases": [
            run_case(binary, number, ignored)
            for number in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP)
            for ignored in (False, True)
        ],
    }
    expected = {"SIGINT": 2, "SIGTERM": 15, "SIGHUP": 15}
    report["passed"] = all(
        not case["alive_300ms_after_signal"]
        and case["returncode_after_signal"] == expected[case["signal"]]
        for case in report["cases"]
    )
    rendered = json.dumps(report, indent=2) + "\n"
    if args.output:
        args.output.write_text(rendered)
    print(rendered, end="")
    if not report["passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
