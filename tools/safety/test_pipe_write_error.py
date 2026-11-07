#!/usr/bin/env python3
"""Inject a permanent helper-pipe write failure and verify final status."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import subprocess
import tempfile
import termios
import time


ROOT = Path(__file__).resolve().parents[2]
SHIM = ROOT / "tools/safety/native/pipe_write_eio.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary: Path, library: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-pipe-eio-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 200)
        trace = root / "hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(library),
            "RLESS_PIPE_WRITE_EIO_TRACE": str(trace),
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env, cwd=root,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 2
            while b"line" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            if pager.poll() is None:
                os.write(master, b"|.cat > pipe.out\n")
            deadline = time.monotonic() + 3
            while not trace.exists() and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            if pager.poll() is None:
                os.write(master, b"\nq")
                try:
                    pager.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
            while True:
                ready, _, _ = select.select([master], [], [], 0)
                if not ready:
                    break
                try:
                    chunk = os.read(master, 65536)
                except OSError:
                    break
                if not chunk:
                    break
                output.extend(chunk)
            return {
                "fault_hit": trace.read_bytes() == b"hit\n" if trace.exists() else False,
                "status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "diagnostic": b"rless: pipe I/O error" in output,
                "pipe_bytes": (root / "pipe.out").stat().st_size if (root / "pipe.out").exists() else None,
            }
        finally:
            try:
                os.killpg(pager.pid, 9)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-pipe-shim-") as directory:
        library = Path(directory) / "pipe_write_eio.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(library), "-ldl"],
            check=True, capture_output=True,
        )
        case = observe(binary, library)
    if args.expect_vulnerable:
        passed = case["fault_hit"] and case["status"] == 0 and case["terminal_restored"]
    else:
        passed = (case["fault_hit"] and case["status"] == 1
                  and case["terminal_restored"] and case["diagnostic"])
    result = {
        "binary_sha256": digest(binary), "shim_sha256": digest(SHIM),
        "expect_vulnerable": args.expect_vulnerable, "case": case,
        "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
