#!/usr/bin/env python3
"""Kill the handoff supervisor and verify shell-group and tty recovery."""

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

from test_shell_handoff_signal import alive, claim_terminal


def observe(binary: Path, ignored_sigchld: bool = False) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-shell-supervisor-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 200)
        marker = root / "helper-hit"
        helper = root / "helper"
        helper.write_text(
            '#!/bin/sh\ntrap "" HUP\n'
            'sleep 30 &\nchild=$!\n'
            'printf "hit %s %s" "$$" "$child" > "$RLESS_HELPER_TRACE"\n'
            'wait "$child"\n'
        )
        helper.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {"PATH": "/usr/bin:/bin", "HOME": directory,
               "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
               "RLESS_HELPER_TRACE": str(marker)}
        def prepare_terminal() -> None:
            claim_terminal()
            if ignored_sigchld:
                signal.signal(signal.SIGCHLD, signal.SIG_IGN)

        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            preexec_fn=prepare_terminal, close_fds=True, env=env,
        )
        os.close(slave)
        helper_pid = descendant_pid = shell_group = supervisor_pid = None
        output = bytearray()

        def read_output(timeout: float) -> None:
            ready, _, _ = select.select([master], [], [], timeout)
            if ready:
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    pass

        try:
            deadline = time.monotonic() + 5
            while b"line" not in output and time.monotonic() < deadline:
                read_output(0.05)
            if b"line" not in output:
                raise TimeoutError("pager did not display input")
            os.write(master, b"!" + os.fsencode(helper) + b"\n")
            deadline = time.monotonic() + 5
            while not marker.exists() and time.monotonic() < deadline:
                read_output(0.05)
            if not marker.exists():
                raise TimeoutError("shell helper did not start")
            fields = marker.read_bytes().split()
            helper_pid, descendant_pid = map(int, fields[1:3])
            shell_group = os.getpgid(helper_pid)
            children = Path(
                f"/proc/{pager.pid}/task/{pager.pid}/children"
            ).read_text().split()
            if len(children) != 1:
                raise AssertionError(f"expected one supervisor, got {children}")
            supervisor_pid = int(children[0])
            foreground_before = os.tcgetpgrp(master)
            live_before = alive(helper_pid) and alive(descendant_pid)
            os.kill(supervisor_pid, signal.SIGKILL)
            deadline = time.monotonic() + 2
            while os.tcgetpgrp(master) != pager.pid and time.monotonic() < deadline:
                read_output(0.01)
            foreground_recovered = os.tcgetpgrp(master) == pager.pid
            if foreground_recovered and pager.poll() is None:
                os.write(master, b"q")
                try:
                    pager.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
            deadline = time.monotonic() + 0.5
            while (alive(helper_pid) or alive(descendant_pid)) and time.monotonic() < deadline:
                time.sleep(0.01)
            return {
                "supervisor_pid": supervisor_pid,
                "helper_live_before": live_before,
                "shell_foreground_before": foreground_before == shell_group,
                "foreground_recovered": foreground_recovered,
                "helper_alive_after": alive(helper_pid),
                "descendant_alive_after": alive(descendant_pid),
                "pager_status": pager.poll(),
                "termios_restored": termios.tcgetattr(master) == original,
            }
        finally:
            if shell_group is not None:
                try:
                    os.killpg(shell_group, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            if supervisor_pid is not None:
                try:
                    os.kill(supervisor_pid, signal.SIGKILL)
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
    parser.add_argument("--ignored-sigchld", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    results = [observe(binary, args.ignored_sigchld) for _ in range(args.rounds)]
    passed = all(case["helper_live_before"]
                 and case["shell_foreground_before"]
                 and case["foreground_recovered"]
                 and not case["helper_alive_after"]
                 and not case["descendant_alive_after"]
                 and case["pager_status"] == 0
                 and case["termios_restored"] for case in results)
    report = {"binary": str(binary),
              "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "rounds": args.rounds, "ignored_sigchld": args.ignored_sigchld,
              "passed": passed, "results": results}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items()
                      if key != "results"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
