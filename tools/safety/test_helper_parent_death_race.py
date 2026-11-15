#!/usr/bin/env python3
"""A LESSOPEN shell must reject reparenting between fork and child setup."""

import argparse
import fcntl
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


def alive(pid: int) -> bool:
    try:
        state = Path(f"/proc/{pid}/stat").read_text().split()[2]
    except (FileNotFoundError, ProcessLookupError):
        return False
    return state not in {"Z", "X", "x"}


def wait_for(predicate, seconds: float) -> bool:
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.005)
    return predicate()


def claim_terminal() -> None:
    os.setsid()
    fcntl.ioctl(0, termios.TIOCSCTTY, 0)


def observe(binary: Path, shim: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-helper-parent-race-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 80)
        gate = root / "gate"
        gate.mkdir()
        escaped = root / "escaped"
        helper = root / "helper"
        helper.write_text('#!/bin/sh\nprintf escaped > "$RLESS_TEST_ESCAPE"\n')
        helper.chmod(0o700)
        master, slave = pty.openpty()
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            preexec_fn=claim_terminal, close_fds=True,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                 "LESSOPEN": f"|{helper} %s", "LD_PRELOAD": str(shim),
                 "RLESS_TEST_SHELL_FORK_GATE": str(gate),
                 "RLESS_TEST_ESCAPE": str(escaped)},
        )
        os.close(slave)
        supervisor_pid = shell_pid = None
        try:
            if not wait_for(lambda: (gate / "child").exists() or pager.poll() is not None, 5):
                raise TimeoutError("helper shell never reached fork gate")
            if not (gate / "child").exists():
                raise AssertionError("pager exited before helper fork gate")
            shell_pid = int((gate / "child").read_text().strip())
            children = Path(f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().split()
            if len(children) != 1:
                raise AssertionError(f"expected one helper supervisor, got {children}")
            supervisor_pid = int(children[0])
            shell_live_before = alive(shell_pid)
            os.kill(pager.pid, signal.SIGSTOP)
            if not wait_for(lambda: Path(f"/proc/{pager.pid}/stat").read_text().split()[2] == "T", 2):
                raise TimeoutError("pager did not stop")
            os.kill(supervisor_pid, signal.SIGKILL)
            os.kill(pager.pid, signal.SIGKILL)
            pager.wait(timeout=2)
            (gate / "release").touch()
            wait_for(lambda: not alive(shell_pid) or escaped.exists(), 2)
            return {"shell_live_before": shell_live_before,
                    "pager_status": pager.returncode,
                    "shell_live_after": alive(shell_pid),
                    "escaped_command_ran": escaped.exists(),
                    "supervisor_pid": supervisor_pid, "shell_pid": shell_pid}
        finally:
            (gate / "release").touch()
            if supervisor_pid is not None:
                try:
                    os.killpg(supervisor_pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--shim", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--expect-escape", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    shim = args.shim.resolve(strict=True)
    cases = [observe(binary, shim) for _ in range(args.rounds)]
    passed = all(case["shell_live_before"] and case["pager_status"] == -signal.SIGKILL
                 and case["escaped_command_ran"] == args.expect_escape for case in cases)
    report = {"binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "shim_sha256": hashlib.sha256(shim.read_bytes()).hexdigest(),
              "rounds": args.rounds, "expect_escape": args.expect_escape,
              "passed": passed, "cases": cases}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "cases"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
