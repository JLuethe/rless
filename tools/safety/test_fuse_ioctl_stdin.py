#!/usr/bin/env python3
"""Probe stalled FUSE stdin ioctl and metadata requests across SIGTERM."""

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


SOURCE = Path(__file__).with_name("native") / "fuse_ioctl_stdin.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare_terminal():
    os.setsid()
    fcntl.ioctl(1, termios.TIOCSCTTY, 0)
    os.tcsetpgrp(1, os.getpgrp())


def proc_text(pid, name):
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError):
        return None


def observe(binary, stall_getattr):
    with tempfile.TemporaryDirectory(prefix="rless-fuse-ioctl-stdin-") as directory:
        root = Path(directory)
        mount = root / "mount"
        mount.mkdir()
        marker = root / "events"
        server_binary = root / "server"
        flags = subprocess.check_output(
            ["pkg-config", "--cflags", "--libs", "fuse3"], text=True,
        ).split()
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE),
             "-o", str(server_binary), *flags], check=True, capture_output=True,
        )
        server_env = {"PATH": "/usr/bin:/bin", "RLESS_IOCTL_MARKER": str(marker)}
        if stall_getattr:
            server_env["RLESS_STALL_GETATTR"] = str(stall_getattr)
        server = subprocess.Popen(
            [str(server_binary), "-f", "-s", str(mount)],
            env=server_env,
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            start_new_session=True,
        )
        pager = None
        master = None
        try:
            deadline = time.monotonic() + 3
            while not os.path.ismount(mount) and server.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            if not os.path.ismount(mount):
                raise RuntimeError("FUSE mount did not become ready")
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            input_fd = os.open(mount / "input", os.O_RDONLY)
            try:
                pager = subprocess.Popen(
                    [str(binary), "-"], stdin=input_fd, stdout=slave, stderr=slave,
                    preexec_fn=prepare_terminal, close_fds=True,
                    env={"PATH": "/usr/bin:/bin", "HOME": directory,
                         "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
                )
            finally:
                os.close(input_fd)
                os.close(slave)
            deadline = time.monotonic() + 4
            events = []
            while pager.poll() is None and time.monotonic() < deadline:
                events = marker.read_text().splitlines() if marker.exists() else []
                trigger = (f"getattr {stall_getattr}" in events if stall_getattr else
                           "read" in events or any(item.startswith("ioctl 2 ") for item in events))
                if trigger:
                    break
                time.sleep(0.01)
            first_ioctl = any(item.startswith("ioctl 1 ") for item in events)
            second_ioctl = any(item.startswith("ioctl 2 ") for item in events)
            read_reached = "read" in events
            triggered = (f"getattr {stall_getattr}" in events if stall_getattr else
                         second_ioctl or read_reached)
            if first_ioctl and triggered and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            before = pager.poll()
            wchan = proc_text(pager.pid, "wchan") if before is None else None
            syscall = proc_text(pager.pid, "syscall") if before is None else None
            children = proc_text(pager.pid, f"task/{pager.pid}/children") if before is None else None
            restored_before = termios.tcgetattr(master) == original
            os.kill(server.pid, signal.SIGUSR1)
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            return {
                "binary_sha256": sha(binary),
                "source_sha256": sha(SOURCE),
                "server_sha256": sha(server_binary),
                "events": events, "first_ioctl": first_ioctl,
                "stall_getattr": stall_getattr, "triggered": triggered,
                "second_ioctl": second_ioctl, "read_reached": read_reached,
                "status_before_release": before,
                "wchan_before_release": wchan,
                "syscall_before_release": syscall,
                "children_before_release": children,
                "terminal_restored_before_release": restored_before,
                "status_after_release": pager.poll(),
                "terminal_restored_after_release": termios.tcgetattr(master) == original,
            }
        finally:
            try:
                os.kill(server.pid, signal.SIGUSR1)
            except ProcessLookupError:
                pass
            if pager is not None:
                try:
                    os.killpg(pager.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                pager.wait(timeout=2)
            if master is not None:
                os.close(master)
            if os.path.ismount(mount):
                subprocess.run(["fusermount3", "-u", str(mount)], timeout=3, check=False)
            if server.poll() is None:
                server.terminate()
            server.wait(timeout=3)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--stall-getattr", type=int, choices=(2, 3, 4), default=0)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    report = observe(binary, args.stall_getattr)
    common = (report["first_ioctl"] and report["triggered"]
              and report["terminal_restored_after_release"]
              and report["status_after_release"] == 15)
    if args.expect_blocked and args.stall_getattr:
        passed = (common and report["status_before_release"] is None
                  and report["wchan_before_release"] == "request_wait_answer"
                  and (report["syscall_before_release"] or "").startswith("5 0x0 ")
                  and report["children_before_release"] == "")
    elif args.expect_blocked:
        passed = (common and report["second_ioctl"] and not report["read_reached"]
                  and report["status_before_release"] is None
                  and report["wchan_before_release"] == "request_wait_answer"
                  and (report["syscall_before_release"] or "").startswith("16 0x0 "))
    elif args.stall_getattr:
        passed = (common and report["status_before_release"] == 15
                  and report["terminal_restored_before_release"])
    else:
        passed = (common and not report["second_ioctl"] and report["read_reached"]
                  and report["status_before_release"] == 15
                  and report["terminal_restored_before_release"])
    report["expect_blocked"] = args.expect_blocked
    report["passed"] = passed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
