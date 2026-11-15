#!/usr/bin/env python3
"""Check command dispatch and nested prompts after terminal signals."""

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import subprocess
import tempfile
import termios
import time


SOURCE = Path(__file__).with_name("native") / "tty_command_signal.c"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, shim: Path, mode: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-tty-command-signal-") as directory:
        root = Path(directory)
        input_path = root / "input"
        marker = root / "helper-ran"
        input_path.write_bytes(b"line\n")
        master, slave = pty.openpty()
        trace_read, trace_write = os.pipe2(os.O_CLOEXEC)
        before = termios.tcgetattr(slave)
        output = bytearray()
        def prepare_tty() -> None:
            os.setsid()
            fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory,
            "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
            "LD_PRELOAD": str(shim),
            "RLESS_TTY_COMMAND_TRACE_FD": str(trace_write),
            "RLESS_TTY_COMMAND_MODE": mode,
        }
        if mode == "mark_user":
            env["LESS_SIGUSR1"] = "q"
        pager = subprocess.Popen(
            [str(binary), str(input_path)], stdin=slave, stdout=slave,
            stderr=slave, env=env, preexec_fn=prepare_tty,
            pass_fds=(trace_write,),
        )
        os.close(trace_write)
        trace_write = -1
        try:
            deadline = time.monotonic() + 3
            ready = False
            while time.monotonic() < deadline:
                if select.select([master], [], [], 0.05)[0]:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
                    if b"(END)" in output:
                        ready = True
                        break
            if ready:
                command = (b"q" if mode == "resize" else
                           b"|\x0e1" if mode in ("pipe_prompt", "pipe_resize") else
                           b"ma" if mode in ("mark_prompt", "mark_resize", "mark_user") else
                           b"!printf x > " + os.fsencode(marker) + b"\n")
                os.write(master, command)
            trace = bytearray()
            deadline = time.monotonic() + 3
            while time.monotonic() < deadline and b"\n" not in trace:
                if select.select([trace_read], [], [], 0.05)[0]:
                    trace.extend(os.read(trace_read, 128))
                    break
                if select.select([master], [], [], 0)[0]:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            if trace and mode == "terminate":
                # The old pager may wait at the shell's done prompt. The
                # corrected pager exits directly on the pending termination.
                try:
                    os.write(master, b"\nq")
                except OSError:
                    pass
            if trace and mode == "mark_resize":
                os.write(master, b"'aq")
            if trace and mode == "pipe_resize":
                # Complete the line-number prompt, then the empty pipe
                # command, then quit the pager.
                os.write(master, b"\n\nq")
            if trace and mode in ("pipe_prompt", "pipe_resize", "mark_prompt", "mark_resize", "mark_user"):
                # Capture the nested prompt's output after the signal. The
                # pipe prompt may otherwise ring the bell indefinitely.
                until = time.monotonic() + 0.1
                while time.monotonic() < until and pager.poll() is None:
                    if select.select([master], [], [], 0.01)[0]:
                        try:
                            output.extend(os.read(master, 65536))
                        except OSError:
                            break
            try:
                pager.wait(timeout=2 if mode in ("pipe_prompt", "pipe_resize", "mark_prompt", "mark_resize", "mark_user") else 3)
                timed_out = False
            except subprocess.TimeoutExpired:
                pager.kill()
                pager.wait(timeout=3)
                timed_out = True
            after = termios.tcgetattr(slave)
            return {
                "mode": mode, "ready": ready,
                "trace": trace.decode("ascii", errors="replace"),
                "status": pager.returncode, "timed_out": timed_out,
                "helper_ran": marker.exists(),
                "terminal_restored": before == after,
                "invalid_mark_seen": b"Invalid mark letter" in output,
                "bell_count": output.count(7),
                "output_sha256": digest(output),
                "output_tail": bytes(output[-200:]).decode("latin1"),
            }
        finally:
            if pager.poll() is None:
                pager.kill()
                pager.wait()
            os.close(master)
            os.close(slave)
            os.close(trace_read)
            if trace_write >= 0:
                os.close(trace_write)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--modes", nargs="+", choices=("terminate", "resize", "pipe_prompt", "pipe_resize", "mark_prompt", "mark_resize", "mark_user"),
                        default=("terminate", "resize", "pipe_prompt", "pipe_resize", "mark_prompt", "mark_resize", "mark_user"))
    parser.add_argument("--expect-helper-after-signal", action="store_true")
    parser.add_argument("--expect-prompt-hang", action="store_true")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.parent / (args.output.stem + ".so")
    subprocess.run(
        ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    cases = [observe(binary, shim.resolve(), mode)
             for mode in args.modes for _ in range(args.rounds)]
    def valid(case: dict) -> bool:
        if not case["ready"] or case["trace"] != "hit-blocked\n":
            return False
        if case["mode"] == "pipe_prompt" and args.expect_prompt_hang:
            return (case["timed_out"] and case["bell_count"] > 0
                    and not case["helper_ran"])
        return (case["status"] == (0 if case["mode"] in ("resize", "pipe_resize", "mark_resize", "mark_user") else 15)
                and not case["timed_out"]
                and not (case["mode"] in ("mark_prompt", "mark_resize", "mark_user") and case["invalid_mark_seen"])
                and case["helper_ran"] == (args.expect_helper_after_signal
                                           if case["mode"] == "terminate" else False)
                and case["terminal_restored"])
    passed_count = sum(valid(case) for case in cases)
    report = {
        "binary_sha256": digest(binary.read_bytes()),
        "test_sha256": digest(Path(__file__).read_bytes()),
        "shim_source_sha256": digest(SOURCE.read_bytes()),
        "shim_sha256": digest(shim.read_bytes()),
        "expect_helper_after_signal": args.expect_helper_after_signal,
        "expect_prompt_hang": args.expect_prompt_hang,
        "modes": args.modes,
        "rounds_per_mode": args.rounds, "passed_count": passed_count,
        "passed": passed_count == len(cases), "cases": cases,
    }
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "cases"}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
