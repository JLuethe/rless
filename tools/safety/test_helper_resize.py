#!/usr/bin/env python3
"""Check that SIGWINCH does not discard a live LESSOPEN stream."""

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


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-helper-resize-") as directory:
        root = Path(directory)
        source = root / "source"
        source.write_bytes(b"ORIGINAL_CONTENT\n" * 50)
        marker = root / "helper-hit"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\necho hit > "$RLESS_HELPER_TRACE"\n'
            'sleep 0.3\nprintf "PREPROCESSED_CONTENT\\n"\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-",
            "LESSOPEN": f"|{helper} %s", "RLESS_HELPER_TRACE": str(marker),
        }
        process = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 2
            while not marker.exists() and process.poll() is None and time.monotonic() < deadline:
                time.sleep(0.005)
            hit = marker.exists()
            if hit and process.poll() is None:
                os.kill(process.pid, signal.SIGWINCH)
            deadline = time.monotonic() + 2
            while process.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
                if b"PREPROCESSED_CONTENT" in output or b"ORIGINAL_CONTENT" in output:
                    break
            if process.poll() is None:
                os.write(master, b"q")
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            return {
                "helper_hit": hit,
                "preprocessed": b"PREPROCESSED_CONTENT" in output,
                "original": b"ORIGINAL_CONTENT" in output,
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
    parser.add_argument("--rounds", type=int, default=20)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    results = [observe(binary) for _ in range(args.rounds)]
    report = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "rounds": args.rounds,
        "passed": sum(
            result["helper_hit"] and result["preprocessed"]
            and not result["original"] and result["status"] == 0
            and result["terminal_restored"] for result in results
        ),
        "results": results,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "results"}))
    return 0 if report["passed"] == args.rounds else 1


if __name__ == "__main__":
    raise SystemExit(main())
