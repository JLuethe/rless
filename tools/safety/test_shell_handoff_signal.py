#!/usr/bin/env python3
"""Check shell-child conservation after pager SIGTERM against less 712."""

import argparse
import fcntl
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


def alive(pid: int) -> bool:
    try:
        status = Path(f"/proc/{pid}/status").read_text()
    except (FileNotFoundError, ProcessLookupError):
        return False
    return not any(
        line.startswith("State:") and line.split()[1] in ("Z", "X", "x")
        for line in status.splitlines()
    )


def claim_terminal() -> None:
    os.setsid()
    fcntl.ioctl(0, termios.TIOCSCTTY, 0)
    os.tcsetpgrp(0, os.getpgrp())


def observe(binary: Path, group_signal: bool = False,
            descendant: bool = False) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-shell-handoff-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 200)
        marker = root / "helper-hit"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\ntrap "" HUP\n'
            + ('sleep 30 &\nchild=$!\nprintf "hit %s %s" "$$" "$child" > "$RLESS_HELPER_TRACE"\nwait "$child"\n'
               if descendant else
               'printf "hit %s" "$$" > "$RLESS_HELPER_TRACE"\nexec sleep 30\n')
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "RLESS_HELPER_TRACE": str(marker),
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            preexec_fn=claim_terminal, close_fds=True, env=env,
        )
        os.close(slave)
        controlling_tty = os.tcgetpgrp(master) == os.getpgid(pager.pid)
        helper_pid = None
        descendant_pid = None
        try:
            output = bytearray()
            deadline = time.monotonic() + 2
            while time.monotonic() < deadline and pager.poll() is None:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
                if b"line" in output:
                    break
            if pager.poll() is None:
                os.write(master, b"!" + os.fsencode(helper) + b"\n")
            deadline = time.monotonic() + 2
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            hit = marker.read_bytes() if marker.exists() else b""
            if hit.startswith(b"hit "):
                identifiers = hit.split()
                helper_pid = int(identifiers[1])
                if descendant and len(identifiers) == 3:
                    descendant_pid = int(identifiers[2])
            live_before = helper_pid is not None and alive(helper_pid)
            descendant_live_before = descendant_pid is not None and alive(descendant_pid)
            handoff_foreground = (
                live_before and os.tcgetpgrp(master) == os.getpgid(helper_pid)
            )
            try:
                children = Path(f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().split()
            except (FileNotFoundError, ProcessLookupError):
                children = []
            if live_before and pager.poll() is None:
                if group_signal:
                    os.killpg(pager.pid, signal.SIGTERM)
                else:
                    os.kill(pager.pid, signal.SIGTERM)
                try:
                    pager.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            deadline = time.monotonic() + 0.5
            while helper_pid is not None and alive(helper_pid) and time.monotonic() < deadline:
                time.sleep(0.01)
            foreground_after_signal = os.tcgetpgrp(master)
            return {
                "controlling_tty": controlling_tty,
                "helper_hit": helper_pid is not None,
                "helper_live_before_signal": live_before,
                "handoff_foreground": handoff_foreground,
                "helper_alive_after_signal": alive(helper_pid) if helper_pid is not None else None,
                "descendant_live_before_signal": (
                    descendant_live_before if descendant else None
                ),
                "descendant_alive_after_signal": (
                    alive(descendant_pid) if descendant else None
                ),
                "pager_children_before_signal": children,
                "pager_status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "foreground_after_signal": foreground_after_signal,
                "pager_group": pager.pid,
            }
        finally:
            if helper_pid is not None:
                try:
                    os.kill(helper_pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            if descendant_pid is not None:
                try:
                    os.kill(descendant_pid, signal.SIGKILL)
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
    parser.add_argument("--reference", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--group", action="store_true",
                        help="send SIGTERM to the pager process group")
    parser.add_argument("--descendant", action="store_true",
                        help="make the shell wait for a background child")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    reference = args.reference.resolve(strict=True)
    cases = [observe(binary, args.group, args.descendant) for _ in range(args.rounds)]
    reference_cases = [observe(reference, args.group, args.descendant)
                       for _ in range(args.rounds)]
    def valid(case: dict) -> bool:
        return (case["controlling_tty"] and case["helper_hit"]
                and case["helper_live_before_signal"]
                and (not args.descendant or case["descendant_live_before_signal"])
                and case["handoff_foreground"]
                and case["pager_status"] == -signal.SIGTERM
                and case["terminal_restored"])
    matched = all(
        valid(candidate) and valid(expected)
        and candidate["helper_alive_after_signal"] == expected["helper_alive_after_signal"]
        for candidate, expected in zip(cases, reference_cases)
    )
    candidate_child_conserved = all(
        not case["helper_alive_after_signal"]
        and not case["descendant_alive_after_signal"]
        for case in cases
    )
    security_passed = all(map(valid, cases)) and candidate_child_conserved
    result = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "reference": str(reference),
        "reference_sha256": hashlib.sha256(reference.read_bytes()).hexdigest(),
        "rounds": args.rounds,
        "delivery": "process-group" if args.group else "direct",
        "descendant": args.descendant,
        "reference_matched": matched,
        "candidate_child_conserved": candidate_child_conserved,
        "security_passed": security_passed,
        "cases": cases,
        "reference_cases": reference_cases,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items()
                      if key not in ("cases", "reference_cases")}))
    # The C reference leaks this child. Its behavior remains in the report,
    # while the security gate requires the candidate to conserve its child.
    return 0 if security_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
