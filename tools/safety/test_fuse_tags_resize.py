#!/usr/bin/env python3
"""Check that SIGWINCH during a stalled tags read does not consume a later read."""

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


def observe(binary: Path, resize: bool) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-fuse-tags-resize-") as directory:
        root = Path(directory)
        mountpoint = root / "mount"
        mountpoint.mkdir()
        marker = root / "read-count"
        (root / "input").write_bytes(b"first line\n")
        server_binary = root / "server"
        flags = subprocess.check_output(
            ["pkg-config", "--cflags", "--libs", "fuse3"], text=True
        ).split()
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE),
             "-o", str(server_binary), *flags],
            check=True, capture_output=True,
        )
        server = subprocess.Popen(
            [str(server_binary), "-f", "-s", str(mountpoint)],
            env={
                "PATH": "/usr/bin:/bin",
                "RLESS_FUSE_MARKER": str(marker),
                "RLESS_FUSE_TAGS": f"entry\t{root / 'input'}\t1\n",
                "RLESS_FUSE_STALL_KIND": "read",
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
                env={"PATH": "/usr/bin:/bin", "TERM": "xterm-256color",
                     "LC_ALL": "C", "HOME": directory, "LESSHISTFILE": "-"},
                start_new_session=True,
            )
            os.close(slave)
            os.set_blocking(master, False)
            deadline = time.monotonic() + 4
            while (not marker.exists() and pager.poll() is None
                   and time.monotonic() < deadline):
                time.sleep(0.01)
            read_hit = marker.exists()
            status_before_release = pager.poll()
            if resize and read_hit and status_before_release is None:
                os.kill(pager.pid, signal.SIGWINCH)
            # The FUSE callback remains blocked until this explicit release.
            # This interval allows the handler and stdio refill to observe the
            # notification before the worker can return any input bytes.
            if resize:
                time.sleep(0.05)
            os.kill(server.pid, signal.SIGUSR1)
            output = bytearray()
            deadline = time.monotonic() + 3
            while (b"first line" not in output and pager.poll() is None
                   and time.monotonic() < deadline):
                try:
                    output.extend(os.read(master, 65536))
                except (BlockingIOError, OSError):
                    pass
                time.sleep(0.01)
            if pager.poll() is None:
                os.write(master, b"q")
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            return {
                "resize_sent": resize,
                "read_hit": read_hit,
                "status_before_release": status_before_release,
                "read_count": marker.read_text().count("read") if read_hit else 0,
                "displayed_input": b"first line" in output,
                "exit_status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
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
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    normal = observe(binary, False)
    resized = observe(binary, True)
    passed = all(
        item["read_hit"] and item["status_before_release"] is None
        and item["displayed_input"] and item["exit_status"] == 0
        and item["terminal_restored"]
        for item in (normal, resized)
    ) and normal["read_count"] == resized["read_count"]
    result = {
        "binary_sha256": sha(binary), "fixture_sha256": sha(SOURCE),
        "normal": normal, "resized": resized,
        "expected_outcome_observed": bool(passed),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
