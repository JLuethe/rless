#!/usr/bin/env python3
"""Hold an inherited PTY stdout native boundary across SIGTERM."""

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


SOURCE = Path(__file__).with_name("native") / "stall_reopened_output.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare_terminal():
    os.setsid()
    fcntl.ioctl(1, termios.TIOCSCTTY, 0)
    os.tcsetpgrp(1, os.getpgrp())


def proc_text(pid, name):
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError, PermissionError):
        return None


def observe(binary, source, expect_elided):
    with tempfile.TemporaryDirectory(prefix="rless-stall-output-") as directory:
        root = Path(directory)
        shim = root / "shim.so"
        marker = root / "marker"
        release = root / "release"
        data = root / "data"
        data.write_bytes((b"line\n" * 1000))
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC", "-O2",
             str(source), "-o", str(shim), "-ldl"],
            check=True, capture_output=True,
        )
        master, slave = pty.openpty()
        control_fd = os.dup(slave)
        original_flags = fcntl.fcntl(control_fd, fcntl.F_GETFL)
        original = termios.tcgetattr(slave)
        identity = os.fstat(slave)
        pager = None
        try:
            env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
                   "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                   "RLESS_STALL_OUTPUT_DEV": str(identity.st_dev),
                   "RLESS_STALL_OUTPUT_INO": str(identity.st_ino),
                   "RLESS_STALL_OUTPUT_MARKER": str(marker),
                   "RLESS_STALL_OUTPUT_RELEASE": str(release)}
            pager = subprocess.Popen(
                [str(binary), str(data)], stdin=slave, stdout=slave, stderr=slave,
                env=env, preexec_fn=prepare_terminal, close_fds=True,
            )
            os.close(slave)
            slave = -1
            deadline = time.monotonic() + 4
            screen_seen = False
            while pager.poll() is None and time.monotonic() < deadline:
                if marker.exists():
                    break
                if expect_elided:
                    ready, _, _ = select.select([master], [], [], 0.05)
                    if ready:
                        screen_seen |= b"line" in os.read(master, 65536)
                    if screen_seen:
                        break
                else:
                    time.sleep(0.01)
            owners = [int(line) for line in marker.read_text().splitlines()] if marker.exists() else []
            children = [int(pid) for pid in
                        (proc_text(pager.pid, f"task/{pager.pid}/children") or "").split()]
            if (owners or (expect_elided and screen_seen)) and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=2.0)
            except subprocess.TimeoutExpired:
                pass
            before = pager.poll()
            result = {
                "binary_sha256": sha(binary), "source_sha256": sha(source),
                "shim_sha256": sha(shim), "marker_owners": owners,
                "children_at_trigger": children, "pager_pid": pager.pid,
                "screen_seen": screen_seen,
                "status_before_release": before,
                "wchan_before_release": proc_text(pager.pid, "wchan") if before is None else None,
                "children_before_release": [int(pid) for pid in
                                            (proc_text(pager.pid, f"task/{pager.pid}/children") or "").split()] if before is None else [],
                "owner_wchan_before_release": {str(pid): proc_text(pid, "wchan") for pid in owners},
                "terminal_restored_before_release": termios.tcgetattr(master) == original,
                "output_flags_after_signal": fcntl.fcntl(control_fd, fcntl.F_GETFL),
                "original_output_flags": original_flags,
            }
            release.touch()
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            result["status_after_release"] = pager.poll()
            result["terminal_restored_after_release"] = termios.tcgetattr(master) == original
            deadline = time.monotonic() + 1
            child_owners = [pid for pid in owners if pid != pager.pid]
            while any(Path(f"/proc/{pid}").exists() for pid in child_owners) and time.monotonic() < deadline:
                time.sleep(0.01)
            result["child_owners_gone"] = all(not Path(f"/proc/{pid}").exists() for pid in child_owners)
            return result
        finally:
            release.touch()
            if pager is not None:
                try:
                    os.killpg(pager.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                pager.wait(timeout=2)
            for fd in (master, slave, control_fd):
                if fd >= 0:
                    os.close(fd)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--shim-source", type=Path, default=SOURCE)
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--expect-elided", action="store_true")
    args = parser.parse_args()
    if args.expect_blocked and args.expect_elided:
        parser.error("choose only one expectation")
    report = observe(args.binary.resolve(strict=True), args.shim_source.resolve(strict=True), args.expect_elided)
    common = report["terminal_restored_after_release"]
    if args.expect_blocked:
        passed = (common and report["pager_pid"] in report["marker_owners"]
                  and report["status_before_release"] is None
                  and report["status_after_release"] == 15)
    elif args.expect_elided:
        passed = (common and report["screen_seen"] and not report["marker_owners"]
                  and report["status_before_release"] == 15
                  and report["terminal_restored_before_release"]
                  and report["output_flags_after_signal"] == report["original_output_flags"])
    else:
        passed = (common and report["marker_owners"]
                  and report["pager_pid"] not in report["marker_owners"]
                  and any(pid in report["children_at_trigger"] for pid in report["marker_owners"])
                  and report["status_before_release"] == 15
                  and report["status_after_release"] == 15
                  and report["terminal_restored_before_release"]
                  and report["output_flags_after_signal"] == report["original_output_flags"]
                  and report["child_owners_gone"])
    report["expect_blocked"] = args.expect_blocked
    report["expect_elided"] = args.expect_elided
    report["passed"] = passed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
