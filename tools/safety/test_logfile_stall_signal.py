#!/usr/bin/env python3
"""Hold an interactive FUSE logfile write or close and verify cancellation."""

import argparse
import hashlib
import json
import os
import fcntl
from pathlib import Path
import pty
import select
import signal
import subprocess
import tempfile
import termios
import time

SOURCE = Path(__file__).with_name("native") / "fuse_stalled_log_write.c"


def proc_text(pid, name):
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError, PermissionError):
        return None


def wait_screen(master, pager, phrase, seconds):
    collected = bytearray()
    deadline = time.monotonic() + seconds
    while phrase not in collected and pager.poll() is None and time.monotonic() < deadline:
        readable, _, _ = select.select([master], [], [], 0.05)
        if readable:
            try:
                collected.extend(os.read(master, 65536))
            except OSError:
                break
    return phrase in collected, bytes(collected[-300:])


def observe(binary, delivery, stall):
    with tempfile.TemporaryDirectory(prefix="rless-fuse-log-") as name:
        root = Path(name)
        mount = root / "mount"
        mount.mkdir()
        marker = root / "write-hit"
        server_bin = root / "server"
        flags = subprocess.check_output(["pkg-config", "--cflags", "--libs", "fuse3"], text=True).split()
        subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", "-O2", str(SOURCE), "-o", str(server_bin), *flags], check=True)
        server = subprocess.Popen([str(server_bin), "-f", "-s", str(mount)],
                                  env={"PATH": "/usr/bin:/bin",
                                       "RLESS_FUSE_MARKER": str(marker),
                                       "RLESS_FUSE_STALL": stall},
                                  stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                                  start_new_session=True)
        lessecho = binary.with_name("lessecho")
        if not lessecho.exists():
            lessecho = Path(__file__).resolve().parents[2] / "target/debug/lessecho"
        master = None
        write_fd = None
        pager = None
        report = {"binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                  "source_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                  "server_sha256": hashlib.sha256(server_bin.read_bytes()).hexdigest(),
                  "stall": stall, "signal": delivery}
        try:
            deadline = time.monotonic() + 3
            while not os.path.ismount(mount) and server.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            report["mounted"] = os.path.ismount(mount)
            if not report["mounted"]:
                raise RuntimeError("FUSE did not mount")
            master, slave = pty.openpty()
            original = termios.tcgetattr(slave)
            read_fd, write_fd = os.pipe()
            def setup_tty():
                os.setsid()
                fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
            pager = subprocess.Popen([str(binary), "-PsREADY", "-PmREADY", "-PMREADY", "-"],
                                     stdin=read_fd, stdout=slave, stderr=slave,
                                     preexec_fn=setup_tty,
                                     env={"PATH": "/usr/bin:/bin", "HOME": str(root),
                                          "TERM": "xterm-256color", "LC_ALL": "C",
                                          "LESSHISTFILE": "-",
                                          "LESSECHO": str(lessecho)})
            os.close(slave)
            os.close(read_fd)
            os.write(write_fd, b"line\n" * 100)
            ready, tail = wait_screen(master, pager, b"READY", 4)
            report["ready"] = ready
            report["pre_command_tail"] = tail.decode("latin1")
            report["raw_before_command"] = termios.tcgetattr(master) != original
            report["log_prompt"] = False
            if ready:
                os.write(master, b"-o " + os.fsencode(mount / "log") + b"\n")
                if stall != "open":
                    prompted, prompt_tail = wait_screen(master, pager, b"Overwrite", 2)
                    report["log_prompt"] = prompted
                    report["log_prompt_tail"] = prompt_tail.decode("latin1")
                    if prompted:
                        os.write(master, b"O\n")
                        if stall == "flush":
                            resumed, resumed_tail = wait_screen(master, pager, b"READY", 2)
                            report["logging_ready"] = resumed
                            report["logging_ready_tail"] = resumed_tail.decode("latin1")
                            os.close(write_fd)
                            write_fd = None
                            os.write(master, b"q")
            deadline = time.monotonic() + 4
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            report["stall_hit"] = marker.exists()
            report["wchan_before_signal"] = proc_text(pager.pid, "wchan")
            report["worker_pids_before_signal"] = [int(pid) for pid in (
                proc_text(pager.pid, f"task/{pager.pid}/children") or "").split()]
            report["worker_wchans_before_signal"] = [proc_text(pid, "wchan") for pid in
                                                     report["worker_pids_before_signal"]]
            report["raw_at_stall"] = termios.tcgetattr(master) != original
            if pager.poll() is None:
                os.kill(pager.pid, signal.SIGWINCH if delivery == "winch" else signal.SIGTERM)
            try:
                pager.wait(timeout=0.75)
            except subprocess.TimeoutExpired:
                pass
            report["status_before_release"] = pager.poll()
            report["wchan_after_signal"] = proc_text(pager.pid, "wchan") if pager.poll() is None else None
            report["terminal_restored_before_release"] = termios.tcgetattr(master) == original
            if delivery == "winch" and pager.poll() is None:
                resumed, resumed_tail = wait_screen(master, pager, b"READY", 2)
                report["resumed_ready"] = resumed
                report["resumed_tail"] = resumed_tail.decode("latin1")
                os.write(master, b"q")
                try:
                    pager.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
                report["status_after_quit_before_release"] = pager.poll()
                report["terminal_restored_after_quit"] = termios.tcgetattr(master) == original
            os.kill(server.pid, signal.SIGUSR1)
            try:
                pager.wait(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            report["status_after_release"] = pager.poll()
            return report
        finally:
            if server.poll() is None:
                os.kill(server.pid, signal.SIGUSR1)
            if pager is not None:
                try:
                    os.killpg(pager.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                pager.wait(timeout=2)
            if write_fd is not None:
                os.close(write_fd)
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
    parser.add_argument("--signal", choices=("term", "winch"), default="term")
    parser.add_argument("--stall", choices=("open", "truncate", "write", "flush"), default="write")
    arguments = parser.parse_args()
    if arguments.stall == "flush" and arguments.signal != "term":
        parser.error("flush stall currently supports SIGTERM")
    binary = arguments.binary.resolve(strict=True)
    result = observe(binary, arguments.signal, arguments.stall)
    observed_stall = (result["mounted"] and result["ready"]
                      and result["raw_before_command"]
                      and (arguments.stall == "open" or result["log_prompt"])
                      and result["stall_hit"]
                      and (arguments.stall == "flush" or result["raw_at_stall"]))
    if arguments.expect_blocked:
        passed = (observed_stall and result["status_before_release"] is None
                  and (result["wchan_after_signal"] == "request_wait_answer"
                       or result["wchan_before_signal"] == "request_wait_answer"))
    elif arguments.signal == "winch":
        passed = (observed_stall and result["status_before_release"] is None
                  and result.get("resumed_ready")
                  and result.get("status_after_quit_before_release") == 1
                  and result.get("terminal_restored_after_quit"))
    else:
        passed = (observed_stall and result["status_before_release"] == 15
                  and result["terminal_restored_before_release"])
    result["expected_outcome_observed"] = bool(passed)
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    arguments.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items()
                      if not key.endswith("tail")}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
