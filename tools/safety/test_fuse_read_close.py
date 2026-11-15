#!/usr/bin/env python3
"""Hold tags-file FUSE open or close and test SIGTERM cancellation."""

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


SOURCE = Path(__file__).with_name("native") / "fuse_stalled_read_close.c"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def proc_text(pid: int, name: str) -> str | None:
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError, PermissionError):
        return None


def observe(binary: Path, close_ordinal: int, stall_open: bool) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-fuse-read-close-") as directory:
        root = Path(directory)
        mountpoint = root / "mount"
        mountpoint.mkdir()
        marker = root / "flush-hit"
        (root / "input").write_bytes(b"first line\n")
        server_binary = root / "server"
        fuse_flags = subprocess.check_output(
            ["pkg-config", "--cflags", "--libs", "fuse3"], text=True
        ).split()
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE),
             "-o", str(server_binary), *fuse_flags],
            check=True, capture_output=True,
        )
        server = subprocess.Popen(
            [str(server_binary), "-f", "-s", str(mountpoint)],
            env={
                "PATH": "/usr/bin:/bin",
                "RLESS_FUSE_MARKER": str(marker),
                "RLESS_FUSE_TAGS": f"entry\t{root / 'input'}\t1\n",
                "RLESS_FUSE_STALL_ORDINAL": str(close_ordinal),
                "RLESS_FUSE_STALL_KIND": "open" if stall_open else "close",
            },
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            start_new_session=True,
        )
        pager = None
        master = None
        try:
            deadline = time.monotonic() + 3
            while (not os.path.ismount(mountpoint) and server.poll() is None
                   and time.monotonic() < deadline):
                time.sleep(0.01)
            if not os.path.ismount(mountpoint):
                raise RuntimeError("FUSE mount did not become ready")
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            pager = subprocess.Popen(
                [str(binary), "-T" + str(mountpoint / "tags"), "-tentry"],
                stdin=slave, stdout=slave, stderr=slave,
                env={
                    "PATH": "/usr/bin:/bin", "TERM": "xterm-256color",
                    "LC_ALL": "C", "HOME": directory, "LESSHISTFILE": "-",
                },
                start_new_session=True,
            )
            os.close(slave)
            deadline = time.monotonic() + 4
            while (not marker.exists() and pager.poll() is None
                   and time.monotonic() < deadline):
                time.sleep(0.01)
            hit = marker.exists() and marker.read_bytes() == b"hit\n"
            before_signal = pager.poll()
            if hit and before_signal is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            result = {
                "binary_sha256": sha(binary),
                "fixture_sha256": sha(SOURCE),
                "close_ordinal": close_ordinal,
                "stall_kind": "open" if stall_open else "close",
                "server_sha256": sha(server_binary),
                "operation_hit": hit,
                "status_before_signal": before_signal,
                "status_before_release": pager.poll(),
                "wchan_before_release": proc_text(pager.pid, "wchan"),
                "syscall_before_release": proc_text(pager.pid, "syscall"),
                "fd_targets_before_release": {
                    entry.name: os.readlink(entry)
                    for entry in Path(f"/proc/{pager.pid}/fd").iterdir()
                    if entry.is_symlink()
                } if pager.poll() is None else {},
                "children_before_release": proc_text(
                    pager.pid, f"task/{pager.pid}/children"
                ),
                "terminal_restored_before_release": termios.tcgetattr(master) == original,
            }
            os.kill(server.pid, signal.SIGUSR1)
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            result["status_after_release"] = pager.poll()
            result["terminal_restored_after_release"] = (
                termios.tcgetattr(master) == original
            )
            return result
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
            if os.path.ismount(mountpoint):
                subprocess.run(["fusermount3", "-u", str(mountpoint)],
                               timeout=3, check=False)
            if server.poll() is None:
                server.terminate()
            server.wait(timeout=3)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--close-ordinal", type=int, choices=(1, 2), default=1)
    parser.add_argument("--stall-open", action="store_true")
    args = parser.parse_args()
    result = observe(args.binary.resolve(strict=True), args.close_ordinal, args.stall_open)
    common = (result["operation_hit"] and result["status_before_signal"] is None
              and result["status_after_release"] == 15
              and result["terminal_restored_after_release"])
    if args.expect_blocked:
        passed = (common and result["status_before_release"] is None
                  and result["wchan_before_release"] == "request_wait_answer")
    else:
        passed = (common and result["status_before_release"] == 15
                  and result["terminal_restored_before_release"])
    result["expected_outcome_observed"] = bool(passed)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: v for k, v in result.items()
                      if k != "syscall_before_release"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
