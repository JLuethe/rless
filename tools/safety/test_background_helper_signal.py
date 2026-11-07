#!/usr/bin/env python3
"""Check cancellation when a LESSOPEN shell exits but its child holds the pipe."""

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


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except (FileNotFoundError, ProcessLookupError):
        return False
    return not any(
        line.startswith("State:") and "Z" in line.split()[1]
        for line in status.splitlines()
    )


def process_group(pid: int) -> int | None:
    try:
        return os.getpgid(pid)
    except ProcessLookupError:
        return None


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-background-helper-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"data\n" * 50)
        marker = root / "background-pid"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\n/usr/bin/python3 -c '
            "'import os, signal, time; "
            'signal.signal(signal.SIGHUP, signal.SIG_IGN); '
            'open(os.environ["RLESS_HELPER_TRACE"], "w").write(str(os.getpid())); '
            "time.sleep(30)' &\nexit 0\n"
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LESSOPEN": f"|{helper} %s",
            "RLESS_HELPER_TRACE": str(marker),
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        child_pid = None
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            if marker.exists():
                child_pid = int(marker.read_text())
            live_before = child_pid is not None and alive(child_pid)
            supervisors = []
            deadline = time.monotonic() + 0.5
            while time.monotonic() < deadline and pager.poll() is None:
                try:
                    supervisors = Path(
                        f"/proc/{pager.pid}/task/{pager.pid}/children"
                    ).read_text().split()
                except (FileNotFoundError, ProcessLookupError):
                    supervisors = []
                if not supervisors:
                    break
                time.sleep(0.01)
            supervisor_states = {}
            supervisor_groups = {}
            for supervisor in supervisors:
                supervisor_groups[supervisor] = process_group(int(supervisor))
                try:
                    supervisor_states[supervisor] = next(
                        line for line in Path(f"/proc/{supervisor}/status").read_text().splitlines()
                        if line.startswith("State:")
                    )
                except (FileNotFoundError, ProcessLookupError, StopIteration):
                    supervisor_states[supervisor] = "gone"
            if live_before and pager.poll() is None:
                child_group_before = process_group(child_pid)
                os.kill(pager.pid, signal.SIGTERM)
                try:
                    pager.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            deadline = time.monotonic() + 0.5
            while child_pid is not None and alive(child_pid) and time.monotonic() < deadline:
                time.sleep(0.01)
            supervisor_states_after = {}
            for supervisor in supervisors:
                try:
                    supervisor_states_after[supervisor] = {
                        line.split(":", 1)[0]: line.split(":", 1)[1].strip()
                        for line in Path(f"/proc/{supervisor}/status").read_text().splitlines()
                        if line.startswith(("State:", "PPid:"))
                    }
                except (FileNotFoundError, ProcessLookupError):
                    supervisor_states_after[supervisor] = {"State": "gone"}
            child_state_after = {}
            if child_pid is not None:
                try:
                    child_state_after = {
                        line.split(":", 1)[0]: line.split(":", 1)[1].strip()
                        for line in Path(f"/proc/{child_pid}/status").read_text().splitlines()
                        if line.startswith(("State:", "PPid:", "SigPnd:", "ShdPnd:", "SigBlk:"))
                    }
                except (FileNotFoundError, ProcessLookupError):
                    child_state_after = {"State": "gone"}
            return {
                "helper_hit": child_pid is not None,
                "child_live_before_signal": live_before,
                "child_alive_after_signal": alive(child_pid) if child_pid is not None else None,
                "supervisor_children_before_signal": supervisors,
                "supervisor_states_before_signal": supervisor_states,
                "supervisor_groups_before_signal": supervisor_groups,
                "supervisor_states_after_signal": supervisor_states_after,
                "child_group_before_signal": child_group_before if live_before else None,
                "child_group_after_signal": process_group(child_pid) if child_pid is not None else None,
                "child_state_after_signal": child_state_after,
                "pager_status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
        finally:
            if child_pid is not None:
                try:
                    os.kill(child_pid, signal.SIGKILL)
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
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary) for _ in range(args.rounds)]
    passed = sum(
        case["helper_hit"] and case["child_live_before_signal"]
        and not case["child_alive_after_signal"]
        and case["pager_status"] == 15 and case["terminal_restored"]
        for case in cases
    )
    result = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "rounds": args.rounds,
        "passed": passed,
        "cases": cases,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "cases"}))
    return 0 if passed == args.rounds else 1


if __name__ == "__main__":
    raise SystemExit(main())
