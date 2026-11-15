#!/usr/bin/env python3
"""Prove helper cancellation does not depend on an unbounded clone waitpid."""

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


ROOT = Path(__file__).resolve().parents[2]
SHIM = ROOT / "tools/safety/native/helper_reap_delay.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary: Path, library: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-helper-reap-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"original\n" * 50)
        helper = root / "helper"
        helper.write_text("#!/bin/sh\nprintf 'replaced\\n'\n")
        helper.chmod(0o700)
        trace = root / "reap-trace"
        master, slave = pty.openpty()
        original_termios = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LESSOPEN": f"|{helper} %s",
            "LD_PRELOAD": str(library), "RLESS_REAP_DELAY_TRACE": str(trace),
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        try:
            deadline = time.monotonic() + 3
            while not trace.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.005)
            marker = trace.read_text() if trace.exists() else ""
            live_before_signal = pager.poll() is None
            if live_before_signal:
                os.kill(pager.pid, signal.SIGTERM)
                try:
                    pager.wait(timeout=0.8)
                except subprocess.TimeoutExpired:
                    pass
            try:
                wchan = (Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                         if pager.poll() is None else None)
            except FileNotFoundError:
                wchan = None
            return {
                "trace": marker,
                "live_before_signal": live_before_signal,
                "status_after_signal": pager.poll(),
                "wchan": wchan,
                "terminal_restored": termios.tcgetattr(master) == original_termios,
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--pre-fix", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="rless-reap-shim-") as directory:
        library = Path(directory) / "helper_reap_delay.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(library), "-ldl"],
            check=True, capture_output=True,
        )
        baseline = observe(args.pre_fix.resolve(strict=True), library)
        candidate = observe(args.binary.resolve(strict=True), library)
    passed = (
        baseline["live_before_signal"] and "B" in baseline["trace"]
        and baseline["status_after_signal"] is None
        and candidate["live_before_signal"] and "N" in candidate["trace"]
        and "B" not in candidate["trace"]
        and candidate["status_after_signal"] == 15
        and candidate["terminal_restored"]
    )
    result = {
        "baseline_binary_sha256": digest(args.pre_fix.resolve(strict=True)),
        "candidate_binary_sha256": digest(args.binary.resolve(strict=True)),
        "shim_sha256": digest(SHIM),
        "baseline": baseline,
        "candidate": candidate,
        "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
