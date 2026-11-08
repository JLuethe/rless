#!/usr/bin/env python3
"""A LESSOPEN read error must cancel a helper that stays alive."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time


SOURCE = Path(__file__).with_name("native") / "lessopen_read_error.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except FileNotFoundError:
        return False
    return not any(line.startswith("State:") and "Z" in line.split()[1]
                   for line in status.splitlines())


def observe(binary: Path, library: Path, mode: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-lessopen-stall-") as directory:
        root = Path(directory)
        original = root / "original"
        alternate = root / "alternate"
        helper = root / "helper"
        ready = root / "ready"
        hit = root / "hit"
        original.write_bytes(b"ORIGINAL\n")
        alternate.write_bytes(b"ALTERNATE\n")
        helper.write_text(
            '#!/bin/sh\nprintf "%s" "$$" > "$RLESS_LESSOPEN_READY"\n'
            'printf "%s\\n" "$RLESS_LESSOPEN_ALT"\nexec sleep 60\n'
        )
        helper.chmod(0o700)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(library),
            "LESSOPEN": ("|" if mode == "pipe" else "") + str(helper) + " %s",
            "RLESS_LESSOPEN_READY": str(ready),
            "RLESS_LESSOPEN_READ_HIT": str(hit),
            "RLESS_LESSOPEN_READ_LENGTH": "1" if mode == "pipe" else "4096",
            "RLESS_LESSOPEN_ALT": str(alternate),
        }
        pager = subprocess.Popen(
            [str(binary), str(original)], env=env, cwd=root,
            stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, start_new_session=True,
        )
        helper_pid = None
        try:
            try:
                output, error = pager.communicate(timeout=1)
            except subprocess.TimeoutExpired:
                output, error = b"", b""
            helper_pid = int(ready.read_text()) if ready.exists() else None
            if pager.poll() is not None and helper_pid is not None:
                deadline = time.monotonic() + 1
                while alive(helper_pid) and time.monotonic() < deadline:
                    time.sleep(0.01)
            return {
                "fault_hit": hit.exists() and hit.read_bytes() == b"hit\n",
                "helper_alive": alive(helper_pid) if helper_pid is not None else None,
                "status": pager.poll(),
                "diagnostic": b"input preprocessor I/O error" in error,
                "stdout": output.decode("latin-1"),
            }
        finally:
            if pager.poll() is None:
                os.killpg(pager.pid, signal.SIGKILL)
            pager.communicate(timeout=2)
            if helper_pid is not None and alive(helper_pid):
                os.kill(helper_pid, signal.SIGKILL)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-lessopen-stall-shim-") as directory:
        library = Path(directory) / "read_error.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror", "-O2",
             str(SOURCE), "-o", str(library), "-ldl"], check=True,
            capture_output=True,
        )
        cases = {mode: observe(binary, library, mode)
                 for mode in ("filename", "pipe")}
    if args.expect_vulnerable:
        passed = all(case["fault_hit"] and case["helper_alive"]
                     and case["status"] is None for case in cases.values())
    else:
        passed = all(case["fault_hit"] and not case["helper_alive"]
                     and case["status"] == 1 and case["diagnostic"]
                     for case in cases.values())
    result = {
        "binary_sha256": digest(binary), "shim_sha256": digest(SOURCE),
        "expect_vulnerable": args.expect_vulnerable,
        "cases": cases, "passed": passed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
