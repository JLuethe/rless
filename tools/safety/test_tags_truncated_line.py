#!/usr/bin/env python3
"""Exercise ctags lines that fill the fixed input buffer without a delimiter."""

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


CASES = {
    "missing_filename_separator": b"entry\t" + b"A" * 1017,
    "trailing_pattern_escape": b"entry\tfile\t/" + b"A" * 1010 + b"\\",
}
assert all(len(value) == 1023 for value in CASES.values())


def observe(binary: Path, contents: bytes) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-tags-truncated-") as directory:
        root = Path(directory)
        tags = root / "tags"
        tags.write_bytes(contents)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                env[name] = os.environ[name]
        process = subprocess.Popen(
            [str(binary), "-T" + str(tags), "-tentry"], stdin=slave,
            stdout=slave, stderr=slave, start_new_session=True, close_fds=True,
            env=env,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 2
            while process.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            if process.poll() is None:
                os.write(master, b"q")
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            while True:
                ready, _, _ = select.select([master], [], [], 0)
                if not ready:
                    break
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    break
            return {
                "status": process.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "stack_buffer_overflow": b"stack-buffer-overflow" in output,
                "sanitizer_report": b"AddressSanitizer" in output,
                "output_sha256": hashlib.sha256(output).hexdigest(),
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
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    results = {name: observe(binary, contents) for name, contents in CASES.items()}
    report = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "cases": results,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if all(
        result["status"] == 1 and result["terminal_restored"]
        and not result["sanitizer_report"] for result in results.values()
    ) else 1


if __name__ == "__main__":
    raise SystemExit(main())
