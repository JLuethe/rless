"""Deliver SIGTERM at both sides of the pager's self-stop boundary."""

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import struct
import subprocess
import tempfile
import termios
import time


def receive_line(fd, timeout):
    end = time.monotonic() + timeout
    data = bytearray()
    while b"\n" not in data and time.monotonic() < end:
        if select.select([fd], [], [], 0.02)[0]:
            chunk = os.read(fd, 4096)
            if not chunk:
                break
            data.extend(chunk)
    return json.loads(data.split(b"\n", 1)[0]) if b"\n" in data else {"timeout": True}


def run(binary, shim, phase):
    with tempfile.TemporaryDirectory(prefix="rless-stop-term-") as directory:
        root = Path(directory)
        (root / "data").write_text("line\n" * 50)
        trace = root / "hit"
        master, slave = pty.openpty()
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
        original = termios.tcgetattr(slave)
        control_read, control_write = os.pipe()
        report_read, report_write = os.pipe()
        supervisor = os.fork()
        if supervisor == 0:
            os.close(master)
            os.close(control_write)
            os.close(report_read)
            child = None
            try:
                os.setsid()
                fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
                signal.signal(signal.SIGTTOU, signal.SIG_IGN)
                gate_r, gate_w = os.pipe()
                child = os.fork()
                if child == 0:
                    os.close(gate_w)
                    os.close(control_read)
                    os.close(report_write)
                    os.setpgid(0, 0)
                    os.read(gate_r, 1)
                    os.close(gate_r)
                    for fd in (0, 1, 2):
                        os.dup2(slave, fd)
                    if slave > 2:
                        os.close(slave)
                    env = {"PATH": "/usr/bin:/bin", "HOME": directory,
                           "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                           "LD_PRELOAD": str(shim), "RLESS_STOP_TERM_TRACE": str(trace),
                           "RLESS_STOP_TERM_PHASE": phase}
                    os.chdir(root)
                    os.execve(str(binary), [str(binary), "-P@@READY@@", "data"], env)
                os.close(gate_r)
                os.setpgid(child, child)
                os.tcsetpgrp(slave, child)
                os.write(gate_w, b"g")
                os.close(gate_w)
                os.write(report_write, (json.dumps({"child": child}) + "\n").encode())
                if os.read(control_read, 1) != b"s":
                    raise RuntimeError("missing start command")
                os.kill(child, signal.SIGTSTP)
                outcome = None
                end = time.monotonic() + 2
                while time.monotonic() < end:
                    found, status = os.waitpid(child, os.WNOHANG | os.WUNTRACED)
                    if found:
                        outcome = "stopped" if os.WIFSTOPPED(status) else "exited"
                        break
                    time.sleep(0.01)
                if outcome == "stopped":
                    time.sleep(0.2)
                    still_stopped = os.waitpid(child, os.WNOHANG)[0] == 0
                    restored = termios.tcgetattr(slave) == original
                    os.kill(child, signal.SIGCONT)
                    end = time.monotonic() + 2
                    exit_status = None
                    while time.monotonic() < end:
                        found, status = os.waitpid(child, os.WNOHANG)
                        if found:
                            exit_status = os.waitstatus_to_exitcode(status)
                            break
                        time.sleep(0.01)
                elif outcome == "exited":
                    still_stopped = False
                    restored = termios.tcgetattr(slave) == original
                    exit_status = os.waitstatus_to_exitcode(status)
                else:
                    still_stopped = False
                    restored = termios.tcgetattr(slave) == original
                    exit_status = None
                record = {"outcome": outcome, "still_stopped_after_200ms": still_stopped,
                          "terminal_restored": restored, "status": exit_status,
                          "shim_hit": trace.read_bytes() == b"hit\n" if trace.exists() else False}
                os.write(report_write, (json.dumps(record) + "\n").encode())
            except BaseException as error:
                os.write(report_write, (json.dumps({"error": repr(error)}) + "\n").encode())
            finally:
                if child is not None:
                    try:
                        os.killpg(child, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                os._exit(0)
        os.close(control_read)
        os.close(report_write)
        first = receive_line(report_read, 2)
        transcript = bytearray()
        end = time.monotonic() + 5
        while b"@@READY@@" not in transcript and time.monotonic() < end:
            if select.select([master], [], [], 0.02)[0]:
                try:
                    transcript.extend(os.read(master, 65536))
                except OSError:
                    break
        if b"@@READY@@" in transcript:
            os.write(control_write, b"s")
            result = receive_line(report_read, 5)
        else:
            result = {"error": "pager not ready"}
        os.close(control_write)
        os.waitpid(supervisor, 0)
        os.close(report_read)
        os.close(master)
        os.close(slave)
        return {"ready": b"@@READY@@" in transcript, "first": first,
                "phase": phase, **result}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--pre-fix", type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    baseline = args.pre_fix.resolve(strict=True) if args.pre_fix else None
    source = Path(__file__).with_name("native") / "stop_term.c"
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="rless-stop-term-shim-") as directory:
        shim = Path(directory) / "stop_term.so"
        subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
                        str(source), "-ldl", "-o", str(shim)], check=True)
        cases = [dict(round=round_number, **run(binary, shim, phase))
                 for round_number in range(args.rounds)
                 for phase in ("kill", "unmask")]
        prior = [run(baseline, shim, phase) for phase in ("kill", "unmask")] if baseline else None
        passed = all(case["ready"] and case["shim_hit"]
                     and case["outcome"] == "exited" and case["status"] == 15
                     and case["terminal_restored"] for case in cases)
        if prior is not None:
            passed &= all(case["ready"] and case["shim_hit"]
                          and case["outcome"] == "stopped"
                          and case["still_stopped_after_200ms"]
                          and case["terminal_restored"]
                          and case["status"] == 15 for case in prior)
        report = {"passed": passed, "rounds": args.rounds,
                  "candidate_binary_sha256": sha(binary),
                  "baseline_binary_sha256": sha(baseline) if baseline else None,
                  "shim_source_sha256": sha(source), "shim_sha256": sha(shim),
                  "candidate": cases, "baseline": prior}
        output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({"passed": passed, "output": str(output)}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
