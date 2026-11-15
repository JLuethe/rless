#!/usr/bin/env python3
"""Bound a stalled shell-supervisor handshake even with inherited SIGTERM ignored."""

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


SOURCE = Path(__file__).with_name("native") / "shell_setup_gate.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare_terminal():
    os.setsid()
    fcntl.ioctl(0, termios.TIOCSCTTY, 0)
    os.tcsetpgrp(0, os.getpgrp())
    signal.signal(signal.SIGTERM, signal.SIG_IGN)
    signal.signal(signal.SIGCHLD, signal.SIG_IGN)


def observe(binary, shim):
    with tempfile.TemporaryDirectory(prefix="rless-shell-setup-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"line\n" * 100)
        marker = root / "gate-hit"
        release = root / "release"
        command_ran = root / "command-ran"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory,
            "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
            "LD_PRELOAD": str(shim),
            "RLESS_SHELL_SETUP_MARKER": str(marker),
            "RLESS_SHELL_SETUP_RELEASE": str(release),
        }
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            preexec_fn=prepare_terminal, close_fds=True, env=env,
        )
        os.close(slave)
        output = bytearray()
        supervisor = None
        try:
            deadline = time.monotonic() + 3
            while b"line" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            displayed = b"line" in output
            if displayed and pager.poll() is None:
                os.write(master, b"!touch " + os.fsencode(command_ran) + b"\n")
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            if marker.exists():
                supervisor = int(marker.read_text())
            try:
                children = Path(f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().split()
            except FileNotFoundError:
                children = []
            gate_is_child = supervisor is not None and str(supervisor) in children
            if supervisor is not None and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            time.sleep(0.25)
            initially_waiting = pager.poll() is None and not command_ran.exists()
            initial_wchan = (Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                             if pager.poll() is None else None)
            time.sleep(5.4)
            supervisor_present = (supervisor is not None
                                  and Path(f"/proc/{supervisor}").exists())
            foreground_before_q = os.tcgetpgrp(master)
            if pager.poll() is None:
                os.write(master, b"q")
            try:
                pager.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            return {
                "displayed": displayed, "gate_hit": marker.exists(),
                "gate_is_child": gate_is_child,
                "initially_waiting_after_ignored_term": initially_waiting,
                "initial_wait_channel": initial_wchan,
                "supervisor_present_after_timeout": supervisor_present,
                "foreground_before_q": foreground_before_q,
                "pager_group": pager.pid,
                "command_ran": command_ran.exists(),
                "status_before_gate_release": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            if supervisor is not None:
                try:
                    os.killpg(supervisor, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            release.touch()
            pager.wait(timeout=2)
            os.close(master)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.with_suffix(".so")
    subprocess.run(
        ["cc", "-shared", "-fPIC", "-O2", "-Wall", "-Wextra", "-Werror",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    case = observe(binary, shim)
    report = {
        "binary_sha256": sha(binary), "shim_source_sha256": sha(SOURCE),
        "shim_sha256": sha(shim), "expect_blocked": args.expect_blocked,
        "case": case,
    }
    common = (case["displayed"] and case["gate_hit"] and case["gate_is_child"]
              and case["initially_waiting_after_ignored_term"]
              and not case["command_ran"])
    if args.expect_blocked:
        passed = (common and case["supervisor_present_after_timeout"]
                  and case["status_before_gate_release"] is None)
    else:
        passed = (common and not case["supervisor_present_after_timeout"]
                  and case["foreground_before_q"] == case["pager_group"]
                  and case["status_before_gate_release"] == 0
                  and case["terminal_restored"])
    report["passed"] = passed
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
