#!/usr/bin/env python3
"""Check that an output error cannot block forever on a full stderr transport."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import socket
import subprocess
import tempfile
import time


def observe(binary: Path, transport: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-diagnostic-backpressure-") as directory:
        source = Path(directory) / "input"
        source.write_bytes(b"x" * 100000)
        reader = writer = None
        if transport == "pipe":
            reader, writer = os.pipe2(os.O_CLOEXEC | os.O_NONBLOCK)
            while True:
                try:
                    os.write(writer, b"E" * 4096)
                except BlockingIOError:
                    break
            os.set_blocking(writer, True)
        else:
            reader, writer = socket.socketpair()
            writer.setblocking(False)
            while True:
                try:
                    writer.send(b"E" * 4096)
                except BlockingIOError:
                    break
            writer.setblocking(True)
        read_fd = reader if transport == "pipe" else reader.fileno()
        write_fd = writer if transport == "pipe" else writer.fileno()
        original_nonblocking = not os.get_blocking(write_fd)
        full = os.open("/dev/full", os.O_WRONLY)
        process = subprocess.Popen(
            [str(binary), str(source)], stdin=subprocess.DEVNULL,
            stdout=full, stderr=write_fd,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
        )
        os.close(full)
        try:
            try:
                process.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            return {
                "transport": transport,
                "status": process.poll(),
                "wchan": Path(f"/proc/{process.pid}/wchan").read_text().strip()
                    if process.poll() is None else None,
                "original_nonblocking_before": original_nonblocking,
                "original_nonblocking_after": not os.get_blocking(write_fd),
            }
        finally:
            if process.poll() is None:
                process.kill()
            process.wait(timeout=2)
            if transport == "pipe":
                os.close(read_fd)
                os.close(write_fd)
            else:
                reader.close()
                writer.close()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary, kind) for kind in ("pipe", "socket")]
    expected = None if args.expect_blocked else 1
    passed = all(
        case["status"] == expected
        and case["original_nonblocking_before"] is False
        and case["original_nonblocking_after"] is False
        for case in cases
    )
    result = {
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "cases": cases,
        "expected_outcome_observed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
