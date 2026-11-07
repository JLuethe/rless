#!/usr/bin/env python3
"""Hold a FUSE read in the kernel and verify SIGTERM exits without waiting."""

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


SOURCE = Path(__file__).with_name("native") / "fuse_stalled_read.c"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def proc_text(pid: int, name: str) -> str | None:
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError, PermissionError):
        return None


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-fuse-stall-") as directory:
        root = Path(directory)
        mountpoint = root / "mount"
        mountpoint.mkdir()
        marker = root / "hit"
        server_binary = root / "server"
        fuse_flags = subprocess.check_output(["pkg-config", "--cflags", "--libs", "fuse3"],
                                             text=True).split()
        subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE),
                        "-o", str(server_binary), *fuse_flags], check=True, capture_output=True)
        server = subprocess.Popen(
            [str(server_binary), "-f", "-s", str(mountpoint)],
            env={"PATH": "/usr/bin:/bin", "RLESS_FUSE_MARKER": str(marker)},
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, start_new_session=True,
        )
        pager = None
        master = None
        try:
            deadline = time.monotonic() + 3
            while not os.path.ismount(mountpoint) and server.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            if not os.path.ismount(mountpoint):
                raise RuntimeError("FUSE mount did not become ready")
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            pager = subprocess.Popen(
                [str(binary), str(mountpoint / "input")],
                stdin=slave, stdout=slave, stderr=slave, start_new_session=True,
                env={"PATH": "/usr/bin:/bin", "HOME": directory,
                     "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-"},
            )
            os.close(slave)
            deadline = time.monotonic() + 4
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            children = []
            if marker.exists() and pager.poll() is None:
                children = [int(pid) for pid in Path(
                    f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().split()]
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            result = {
                "binary_sha256": sha(binary), "source_sha256": sha(SOURCE),
                "server_sha256": sha(server_binary), "mount_success": True,
                "read_hit": marker.read_bytes() == b"hit\n" if marker.exists() else False,
                "worker_pids": children,
                "worker_states_before_release": [proc_text(pid, "status") for pid in children],
                "pager_status_before_release": pager.poll(),
                "pager_wchan_before_release": proc_text(pager.pid, "wchan"),
                "pager_syscall_before_release": proc_text(pager.pid, "syscall"),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
            os.kill(server.pid, signal.SIGUSR1)
            try:
                pager.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            result["pager_status_after_release"] = pager.poll()
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
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    result = observe(binary)
    if args.expect_blocked:
        passed = (result["read_hit"] and result["worker_pids"]
                  and result["pager_status_before_release"] is None
                  and result["pager_wchan_before_release"] == "request_wait_answer"
                  and result["pager_status_after_release"] == 15
                  and result["terminal_restored"])
    else:
        passed = (result["read_hit"] and result["worker_pids"]
                  and result["pager_status_before_release"] == 15
                  and result["pager_status_after_release"] == 15
                  and result["terminal_restored"])
    result["expected_outcome_observed"] = bool(passed)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: v for k, v in result.items() if k != "worker_states_before_release"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
