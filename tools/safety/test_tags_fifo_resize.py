#!/usr/bin/env python3
"""Prove a resize is dispatched while a partial tags line waits in a FIFO."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import signal
import subprocess
import tempfile
import termios
import time


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def wait_for(predicate, seconds: float) -> bool:
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.01)
    return predicate()


def wchan(pid: int) -> str:
    try:
        return Path(f"/proc/{pid}/wchan").read_text().strip()
    except (FileNotFoundError, ProcessLookupError):
        return "gone"


def observe(binary: Path, shim_source: Path, compiler: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-tags-resize-") as directory:
        root = Path(directory)
        shim = root / "resize.so"
        subprocess.run(
            [compiler, "-shared", "-fPIC", "-O2", "-o", str(shim), str(shim_source), "-ldl"],
            check=True, capture_output=True,
        )
        fifo = root / "tags"
        os.mkfifo(fifo)
        target = root / "target"
        target.write_bytes(b"tagged line\n")
        gate = root / "gate"
        hit = root / "hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        writer = os.open(fifo, os.O_RDWR | os.O_NONBLOCK)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
            "RLESS_RESIZE_GATE": str(gate), "RLESS_RESIZE_HIT": str(hit),
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                env[name] = os.environ[name]
        process = subprocess.Popen(
            [str(binary), "-T" + str(fifo), "-tentry"],
            stdin=slave, stdout=slave, stderr=slave, env=env,
            start_new_session=True, close_fds=True,
        )
        os.close(slave)
        try:
            initial_wait = wait_for(
                lambda: process.poll() is None and "poll" in wchan(process.pid), 3,
            )
            os.write(writer, b"entry\t")
            partial_wait = wait_for(
                lambda: process.poll() is None and "poll" in wchan(process.pid), 2,
            )
            gate.touch()
            if process.poll() is None:
                os.kill(process.pid, signal.SIGWINCH)
            first_dispatch = wait_for(hit.exists, 1)
            if process.poll() is None:
                os.kill(process.pid, signal.SIGWINCH)
            second_dispatch = wait_for(
                lambda: hit.exists() and len(hit.read_bytes()) >= 8, 1,
            )
            os.write(writer, str(target).encode() + b"\t1\n")
            os.close(writer)
            writer = -1
            time.sleep(0.15)
            if process.poll() is None:
                os.write(master, b"q")
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                os.kill(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            result = {
                "binary_sha256": digest(binary),
                "shim_source_sha256": digest(shim_source),
                "initial_wait": initial_wait,
                "partial_line_wait": partial_wait,
                "first_dispatch_before_more_input": first_dispatch,
                "second_dispatch_before_more_input": second_dispatch,
                "status": process.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
            result["passed"] = (
                initial_wait and partial_wait and first_dispatch and second_dispatch
                and result["status"] == 0 and result["terminal_restored"]
            )
            return result
        finally:
            if writer >= 0:
                os.close(writer)
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
    parser.add_argument("--compiler", default="cc")
    args = parser.parse_args()
    source = Path(__file__).resolve().parent / "native/resize_ioctl_probe.c"
    result = observe(args.binary.resolve(strict=True), source, args.compiler)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
