#!/usr/bin/env python3
"""Check SIGTERM during a stalled regular-file stderr diagnostic."""
import argparse
import hashlib
import fcntl
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
SOURCE = ROOT / "tools/safety/native/fuse_stalled_log_write.c"


def wait_for(predicate, seconds):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.01)
    return predicate()


def observe(binary, signal_group):
    with tempfile.TemporaryDirectory(prefix="rless-diagnostic-regular-") as directory:
        root = Path(directory)
        mount = root / "mount"
        mount.mkdir()
        marker = root / "write-hit"
        server_bin = root / "server"
        flags = subprocess.check_output(["pkg-config", "--cflags", "--libs", "fuse3"], text=True).split()
        subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE), "-o", str(server_bin), *flags], check=True)
        server = subprocess.Popen(
            [str(server_bin), "-f", "-s", str(mount)], start_new_session=True,
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            env={"PATH": "/usr/bin:/bin", "RLESS_FUSE_MARKER": str(marker), "RLESS_FUSE_STALL": "write"},
        )
        pager = None
        master = None
        diagnostic_fd = None
        try:
            if not wait_for(lambda: os.path.ismount(mount) or server.poll() is not None, 3):
                raise RuntimeError("FUSE mount timed out")
            if not os.path.ismount(mount):
                raise RuntimeError("FUSE mount failed")
            diagnostic_fd = os.open(mount / "log", os.O_WRONLY)
            source = root / "input"
            source.write_bytes(b"needle\n" * 80)
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            def setup_tty():
                os.setsid()
                fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
            pager = subprocess.Popen(
                [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", str(source)],
                stdin=slave, stdout=slave, stderr=diagnostic_fd,
                preexec_fn=setup_tty, close_fds=True,
                env={"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
                     "LC_ALL": "C", "LESSHISTFILE": str(root / "missing" / "history")},
            )
            os.close(slave)
            output = bytearray()
            deadline = time.monotonic() + 3
            while b"READY" not in output and pager.poll() is None and time.monotonic() < deadline:
                if select.select([master], [], [], 0.05)[0]:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            ready = b"READY" in output
            marker_before_search = marker.exists()
            raw_at_ready = termios.tcgetattr(master) != original
            if ready:
                os.write(master, b"/needle\n")
                time.sleep(0.3)
            after_search = bytearray()
            while select.select([master], [], [], 0)[0]:
                try:
                    after_search.extend(os.read(master, 65536))
                except OSError:
                    break
            if ready and pager.poll() is None:
                os.write(master, b"q")
            hit = wait_for(lambda: marker.exists() or pager.poll() is not None, 4) and marker.exists()
            raw_at_hit = termios.tcgetattr(master) != original
            wchan_before = Path(f"/proc/{pager.pid}/wchan").read_text().strip() if pager.poll() is None else None
            children_before = Path(f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().strip() if pager.poll() is None else None
            fd2_before = os.readlink(f"/proc/{pager.pid}/fd/2") if pager.poll() is None else None
            if hit and pager.poll() is None:
                if signal_group:
                    os.killpg(pager.pid, signal.SIGTERM)
                else:
                    os.kill(pager.pid, signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            after_term = bytearray()
            while select.select([master], [], [], 0)[0]:
                try:
                    after_term.extend(os.read(master, 65536))
                except OSError:
                    break
            status_before = pager.poll()
            terminal_before = termios.tcgetattr(master) == original
            wchan_after = Path(f"/proc/{pager.pid}/wchan").read_text().strip() if status_before is None else None
            syscall_after = Path(f"/proc/{pager.pid}/syscall").read_text().strip() if status_before is None else None
            children_after = Path(f"/proc/{pager.pid}/task/{pager.pid}/children").read_text().strip() if status_before is None else None
            os.kill(server.pid, signal.SIGUSR1)
            try:
                pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                pass
            result = {
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "server_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                "signal_group": signal_group,
                "ready": ready, "diagnostic_write_hit": hit, "raw_at_hit": raw_at_hit,
                "marker_before_search": marker_before_search,
                "raw_at_ready": raw_at_ready,
                "output_tail": bytes(output[-250:]).decode("latin1"),
                "after_search_tail": bytes(after_search[-400:]).decode("latin1"),
                "after_term_tail": bytes(after_term[-400:]).decode("latin1"),
                "children_before_term": children_before,
                "fd2_before_term": fd2_before,
                "wchan_before_term": wchan_before, "status_before_release": status_before,
                "terminal_restored_before_release": terminal_before,
                "wchan_after_term": wchan_after, "status_after_release": pager.poll(),
                "syscall_after_term": syscall_after, "children_after_term": children_after,
            }
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
            if diagnostic_fd is not None:
                os.close(diagnostic_fd)
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
    parser.add_argument("--signal-group", action="store_true")
    args = parser.parse_args()
    result = observe(args.binary.resolve(strict=True), args.signal_group)
    expected_status = None if args.expect_blocked else 15
    result["expected_outcome_observed"] = (
        result["ready"] is True
        and result["marker_before_search"] is False
        and result["raw_at_ready"] is True
        and result["diagnostic_write_hit"] is True
        and result["raw_at_hit"] is False
        and result["terminal_restored_before_release"] is True
        and result["status_before_release"] == expected_status
        and result["status_after_release"] == 15
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if result["expected_outcome_observed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
