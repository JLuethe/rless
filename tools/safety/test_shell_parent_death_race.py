#!/usr/bin/env python3
"""Prove a shell child checks the pre-fork supervisor identity after reparenting."""

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
import time

from test_shell_handoff_signal import alive, claim_terminal


def observe(binary: Path, shim: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-shell-parent-race-") as directory:
        root = Path(directory)
        (root / "input").write_bytes(b"line\n" * 100)
        gate = root / "gate"
        gate.mkdir()
        victim = root / "escaped"
        master, slave = pty.openpty()
        env = {"PATH": "/usr/bin:/bin", "HOME": directory,
               "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
               "LD_PRELOAD": str(shim), "RLESS_TEST_SHELL_FORK_GATE": str(gate)}
        pager = subprocess.Popen(
            [str(binary), str(root / "input")], cwd=directory,
            stdin=slave, stdout=slave, stderr=slave,
            preexec_fn=claim_terminal, close_fds=True, env=env,
        )
        os.close(slave)
        supervisor_pid = child_pid = None
        output = bytearray()

        def drain(timeout: float) -> None:
            ready, _, _ = select.select([master], [], [], timeout)
            if ready:
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    pass

        try:
            deadline = time.monotonic() + 5
            while b"line" not in output and time.monotonic() < deadline:
                drain(0.05)
            if b"line" not in output:
                raise TimeoutError("pager did not display input")
            os.write(master, b"!printf ESCAPED > escaped\n")
            deadline = time.monotonic() + 5
            while not (gate / "child").exists() and time.monotonic() < deadline:
                drain(0.05)
            if not (gate / "child").exists():
                raise TimeoutError("shell fork gate did not fire")
            child_pid = int((gate / "child").read_text().strip())
            children = Path(
                f"/proc/{pager.pid}/task/{pager.pid}/children"
            ).read_text().split()
            if len(children) != 1:
                raise AssertionError(f"expected one supervisor, got {children}")
            supervisor_pid = int(children[0])
            child_live_before = alive(child_pid)
            shell_foreground = os.tcgetpgrp(master) == supervisor_pid
            # Stop the pager before the supervisor exits so it cannot run
            # the pidfd-ready group cleanup. Kill both parents while the
            # shell child is still gated before returning from fork.
            os.kill(pager.pid, signal.SIGSTOP)
            deadline = time.monotonic() + 2
            while True:
                stopped_state = Path(f"/proc/{pager.pid}/status").read_text().split("State:", 1)[1].splitlines()[0].strip()
                if stopped_state.startswith("T"):
                    break
                if time.monotonic() >= deadline:
                    raise TimeoutError("pager did not stop before supervisor kill")
                time.sleep(0.001)
            try:
                os.kill(supervisor_pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            os.kill(pager.pid, signal.SIGKILL)
            pager.wait(timeout=2)
            supervisor_alive_after_pager = alive(supervisor_pid)
            child_alive_after_pager = alive(child_pid)
            (gate / "release").touch()
            deadline = time.monotonic() + 2
            while alive(child_pid) and time.monotonic() < deadline:
                time.sleep(0.01)
            return {"child_live_before": child_live_before,
                    "shell_foreground": shell_foreground,
                    "pager_status": pager.returncode,
                    "child_alive_after": alive(child_pid),
                    "escaped_command_ran": victim.exists(),
                    "stopped_state": stopped_state,
                    "supervisor_alive_after_pager": supervisor_alive_after_pager,
                    "child_alive_after_pager": child_alive_after_pager,
                    "supervisor_pid": supervisor_pid, "shell_child_pid": child_pid}
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
    args = parser.parse_args()
    binary, shim = args.binary.resolve(strict=True), args.shim.resolve(strict=True)
    results = [observe(binary, shim) for _ in range(args.rounds)]
    passed = all(case["child_live_before"] and case["shell_foreground"]
                 and case["pager_status"] == -signal.SIGKILL
                 and not case["child_alive_after"]
                 and not case["escaped_command_ran"] for case in results)
    report = {"binary": str(binary),
              "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "shim_sha256": hashlib.sha256(shim.read_bytes()).hexdigest(),
              "rounds": args.rounds, "passed": passed, "results": results}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items()
                      if key != "results"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
