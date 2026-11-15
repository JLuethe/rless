#!/usr/bin/env python3
"""A failed pipe-helper setup must cancel its running child without waiting."""

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
SHIM = ROOT / "tools/safety/native/pipe_setup_bad_fd.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary: Path, library: Path, failure: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-pipe-setup-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 200)
        trace = root / "hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(library),
            "RLESS_PIPE_SETUP_BAD_FD_TRACE": str(trace),
            "RLESS_PIPE_SETUP_MODE": failure,
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env, cwd=root,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 3
            while b"line" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            if pager.poll() is None:
                os.write(master, b"|.sleep 60\n")
            deadline = time.monotonic() + 3
            while not trace.exists() and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            hit = trace.exists() and trace.read_bytes() == b"hit\n"
            diagnostic = (b"Cannot create pipe" if failure == "bad_fd"
                          else b"cannot ignore SIGPIPE for pipe")
            # The injection marker precedes the error prompt. Sending q at
            # that marker can be consumed by the prompt rather than by the
            # resumed command loop, especially in the optimized build.
            sent_return = False
            next_quit = 0.0
            deadline = time.monotonic() + 2
            while pager.poll() is None and time.monotonic() < deadline:
                if hit and diagnostic in output:
                    if not sent_return:
                        os.write(master, b"\n")
                        sent_return = True
                        next_quit = time.monotonic() + 0.05
                    elif time.monotonic() >= next_quit:
                        os.write(master, b"q")
                        next_quit = time.monotonic() + 0.15
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            return {
                "fault_hit": hit,
                "status": pager.poll(),
                "stalled_after_setup_failure": hit and pager.poll() is None,
                "terminal_restored": termios.tcgetattr(master) == original,
                "diagnostic": diagnostic in output,
                "output_tail": bytes(output[-400:]).decode("latin-1"),
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
    parser.add_argument("--failure", choices=("bad_fd", "sigpipe_install"),
                        default="bad_fd")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-pipe-setup-shim-") as directory:
        library = Path(directory) / "pipe_setup_bad_fd.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(library), "-ldl"],
            check=True, capture_output=True,
        )
        case = observe(binary, library, args.failure)
    if args.expect_vulnerable:
        passed = case["fault_hit"] and case["stalled_after_setup_failure"]
    else:
        passed = (case["fault_hit"] and case["status"] == 1
                  and case["terminal_restored"] and case["diagnostic"])
    result = {
        "binary_sha256": digest(binary), "shim_sha256": digest(SHIM),
        "failure": args.failure, "expect_vulnerable": args.expect_vulnerable,
        "case": case, "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
