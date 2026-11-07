#!/usr/bin/env python3
"""Hold a FUSE history create until after SIGTERM, then check delayed cleanup."""
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

ROOT = Path(__file__).resolve().parents[2]
SERVER_SOURCE = Path(__file__).with_name("native") / "history_late_create.c"
SHIM_SOURCE = Path(__file__).with_name("native") / "history_late_create_probe.c"


def wait_for(predicate, seconds):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.01)
    return predicate()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-leak", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-late-create-") as directory:
        root = Path(directory)
        mount = root / "mount"
        mount.mkdir()
        marker = root / "create-hit"
        created = root / "created"
        unlinked = root / "unlinked"
        skip_marker = root / "skip-wait"
        skip_wait = args.expect_leak
        server_bin = root / "server"
        shim = root / "no_tmpfile.so"
        flags = subprocess.check_output(["pkg-config", "--cflags", "--libs", "fuse3"], text=True).split()
        subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SERVER_SOURCE),
                        "-o", str(server_bin), *flags], check=True)
        subprocess.run(["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror", "-O2",
                        str(SHIM_SOURCE), "-o", str(shim)], check=True)
        server = subprocess.Popen(
            [str(server_bin), "-f", str(mount)], start_new_session=True,
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            env={"PATH": "/usr/bin:/bin", "RLESS_FUSE_MARKER": str(marker),
                 "RLESS_FUSE_CREATED": str(created), "RLESS_FUSE_UNLINKED": str(unlinked)},
        )
        pager = None
        master = None
        try:
            if not wait_for(lambda: os.path.ismount(mount) or server.poll() is not None, 3):
                raise RuntimeError("FUSE mount timed out")
            if not os.path.ismount(mount):
                raise RuntimeError("FUSE mount failed")
            source = root / "input"
            source.write_bytes(b"needle\n" * 80)
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            def setup_tty():
                os.setsid()
                fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
            pager_env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
                         "LC_ALL": "C", "LESSHISTFILE": str(mount / "historyX"),
                         "LD_PRELOAD": str(shim)}
            if skip_wait:
                pager_env["RLESS_SKIP_CLEANUP_WAIT"] = "1"
                pager_env["RLESS_SKIP_MARKER"] = str(skip_marker)
            pager = subprocess.Popen(
                [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", str(source)],
                stdin=slave, stdout=slave, stderr=slave,
                preexec_fn=setup_tty, close_fds=True,
                env=pager_env,
            )
            os.close(slave)
            output = bytearray()
            deadline = time.monotonic() + 3
            while b"READY" not in output and pager.poll() is None and time.monotonic() < deadline:
                if select.select([master], [], [], 0.05)[0]:
                    output.extend(os.read(master, 65536))
            ready = b"READY" in output
            if ready:
                os.write(master, b"/needle\n")
                time.sleep(0.2)
                os.write(master, b"q")
            hit = wait_for(lambda: marker.exists() or pager.poll() is not None, 4) and marker.exists()
            if hit and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            status_before = pager.poll()
            terminal_restored = termios.tcgetattr(master) == original
            os.kill(server.pid, signal.SIGUSR1)
            created_seen = wait_for(lambda: created.exists(), 2)
            unlinked_seen = wait_for(lambda: unlinked.exists(), 2)
            result = {
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "server_sha256": hashlib.sha256(SERVER_SOURCE.read_bytes()).hexdigest(),
                "shim_sha256": hashlib.sha256(SHIM_SOURCE.read_bytes()).hexdigest(),
                "ready": ready, "create_hit": hit,
                "status_before_release": status_before,
                "terminal_restored_before_release": terminal_restored,
                "late_create_completed": created_seen,
                "late_name_unlinked": unlinked_seen,
                "skip_wait_mutation": skip_wait,
                "skip_wait_hit": skip_marker.exists(),
            }
            result["expected_outcome_observed"] = (
                ready and hit and status_before == 15 and terminal_restored
                and created_seen and unlinked_seen == (not skip_wait)
                and skip_marker.exists() == skip_wait
            )
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(result, indent=2) + "\n")
            print(json.dumps(result))
            return 0 if result["expected_outcome_observed"] else 1
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


if __name__ == "__main__":
    raise SystemExit(main())
