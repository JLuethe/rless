#!/usr/bin/env python3
"""Hold an inherited regular descriptor's lseek across SIGTERM."""

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


SOURCE = Path(__file__).with_name("native") / "stall_inherited_seek.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare_terminal():
    os.setsid()
    fcntl.ioctl(1, termios.TIOCSCTTY, 0)
    os.tcsetpgrp(1, os.getpgrp())


def observe(binary):
    with tempfile.TemporaryDirectory(prefix="rless-stall-seek-") as directory:
        root = Path(directory)
        shim = root / "shim.so"
        marker = root / "marker"
        release = root / "release"
        source = root / "input"
        source.write_bytes(b"input line\n" * 100)
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC", "-O2",
             str(SOURCE), "-o", str(shim), "-ldl"],
            check=True, capture_output=True,
        )
        input_fd = os.open(source, os.O_RDONLY)
        output_master, output_slave = pty.openpty()
        original = termios.tcgetattr(output_slave)
        pager = None
        try:
            identity = os.fstat(input_fd)
            env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
                   "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                   "RLESS_STALL_SEEK_DEV": str(identity.st_dev),
                   "RLESS_STALL_SEEK_INO": str(identity.st_ino),
                   "RLESS_STALL_SEEK_MARKER": str(marker),
                   "RLESS_STALL_SEEK_RELEASE": str(release)}
            pager = subprocess.Popen(
                [str(binary), "-"], stdin=input_fd, stdout=output_slave,
                stderr=output_slave, env=env, preexec_fn=prepare_terminal,
                close_fds=True,
            )
            os.close(output_slave)
            output_slave = -1
            deadline = time.monotonic() + 4
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            entries = marker.read_text().splitlines() if marker.exists() else []
            owners = [int(entry.split()[0]) for entry in entries]
            children = []
            try:
                children = [int(pid) for pid in Path(
                    f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().split()]
            except FileNotFoundError:
                pass
            if owners and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            before = pager.poll()
            result = {
                "binary_sha256": sha(binary), "source_sha256": sha(SOURCE),
                "shim_sha256": sha(shim), "marker_entries": entries,
                "marker_owners": owners, "children_at_trigger": children,
                "pager_pid": pager.pid, "status_before_release": before,
                "wchan_before_release": Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                if before is None else None,
                "terminal_restored_before_release": termios.tcgetattr(output_master) == original,
            }
            release.touch()
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            result["status_after_release"] = pager.poll()
            result["terminal_restored_after_release"] = termios.tcgetattr(output_master) == original
            child_owners = [pid for pid in owners if pid != pager.pid]
            deadline = time.monotonic() + 1
            while any(Path(f"/proc/{pid}").exists() for pid in child_owners) and time.monotonic() < deadline:
                time.sleep(0.01)
            result["child_owners_gone"] = all(not Path(f"/proc/{pid}").exists()
                                             for pid in child_owners)
            return result
        finally:
            release.touch()
            if pager is not None:
                try:
                    os.killpg(pager.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                pager.wait(timeout=2)
            for fd in (input_fd, output_master, output_slave):
                if fd >= 0:
                    os.close(fd)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    report = observe(args.binary.resolve(strict=True))
    common = bool(report["marker_entries"]) and report["terminal_restored_after_release"]
    if args.expect_blocked:
        passed = (common and report["pager_pid"] in report["marker_owners"]
                  and report["status_before_release"] is None
                  and report["status_after_release"] == 15)
    else:
        passed = (common and report["pager_pid"] not in report["marker_owners"]
                  and any(pid in report["children_at_trigger"] for pid in report["marker_owners"])
                  and report["status_before_release"] == 15
                  and report["status_after_release"] == 15
                  and report["terminal_restored_before_release"]
                  and report["child_owners_gone"])
    report["expect_blocked"] = args.expect_blocked
    report["passed"] = passed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
