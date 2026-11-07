#!/usr/bin/env python3
"""Force a partial sigaction installation failure and inspect rollback."""

import argparse
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


SOURCE = Path(__file__).with_name("native") / "signal_install_failure.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def handoff_case(binary: Path, shim: Path, directory: str, output_prefix: Path) -> dict:
    root = Path(directory)
    source = root / "input"
    source.write_bytes(b"line\n" * 100)
    marker = root / "unexpected-shell-command"
    reader, writer = os.pipe2(os.O_CLOEXEC)
    master, slave = pty.openpty()
    original = termios.tcgetattr(slave)
    env = {
        "PATH": "/usr/bin:/bin", "HOME": directory,
        "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
        "LD_PRELOAD": str(shim.resolve()),
        "RLESS_SIGNAL_INSTALL_TRACE_FD": str(writer),
        "RLESS_SIGNAL_INSTALL_FAIL_SIGNAL": str(signal.SIGWINCH.value),
        "RLESS_SIGNAL_INSTALL_HANDOFF": "1",
    }
    pager = subprocess.Popen(
        [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
        env=env, pass_fds=(writer,), start_new_session=True,
    )
    os.close(slave)
    os.close(writer)
    try:
        output = bytearray()
        deadline = time.monotonic() + 3
        while b"line" not in output and pager.poll() is None and time.monotonic() < deadline:
            ready, _, _ = select.select([master], [], [], 0.05)
            if ready:
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    break
        displayed = b"line" in output
        if displayed and pager.poll() is None:
            os.write(master, b"!touch " + os.fsencode(marker) + b"\n")
        os.set_blocking(reader, False)
        trace = bytearray()
        deadline = time.monotonic() + 3
        while b"hit\n" not in trace and pager.poll() is None and time.monotonic() < deadline:
            ready, _, _ = select.select([reader, master], [], [], 0.05)
            if reader in ready:
                try:
                    trace.extend(os.read(reader, 4096))
                except BlockingIOError:
                    pass
            if master in ready:
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    pass
        if pager.poll() is None:
            os.write(master, b"q")
        try:
            pager.wait(timeout=2)
        except subprocess.TimeoutExpired:
            pass
        os.set_blocking(master, False)
        while True:
            try:
                data = os.read(master, 65536)
            except (BlockingIOError, OSError):
                break
            if not data:
                break
            output.extend(data)
        while True:
            try:
                data = os.read(reader, 4096)
            except BlockingIOError:
                break
            if not data:
                break
            trace.extend(data)
        trace_path = output_prefix.with_name(output_prefix.name + ".trace.raw")
        tty_path = output_prefix.with_name(output_prefix.name + ".tty.raw")
        trace_path.write_bytes(trace)
        tty_path.write_bytes(output)
        return {
            "status": pager.poll(), "displayed": displayed,
            "trace": trace.decode("ascii", errors="replace"),
            "trace_sha256": hashlib.sha256(trace).hexdigest(),
            "trace_path": str(trace_path),
            "shell_command_ran": marker.exists(),
            "terminal_restored": termios.tcgetattr(master) == original,
            "output_sha256": hashlib.sha256(output).hexdigest(),
            "output_len": len(output), "output_path": str(tty_path),
        }
    finally:
        try:
            os.killpg(pager.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        pager.wait(timeout=2)
        os.close(master)
        os.close(reader)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--signal", choices=("SIGINT", "SIGWINCH", "SIGUSR1"),
                        default="SIGWINCH")
    parser.add_argument("--inherit-ignored-winch", action="store_true")
    parser.add_argument("--handoff", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.parent / (args.output.stem + ".so")
    subprocess.run(
        ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    if args.handoff:
        if args.signal != "SIGWINCH" or args.inherit_ignored_winch:
            parser.error("handoff mode requires SIGWINCH and no inherited override")
        with tempfile.TemporaryDirectory(prefix="rless-signal-handoff-") as directory:
            case = handoff_case(binary, shim, directory, args.output.with_suffix(""))
        report = {
            "binary_sha256": digest(binary), "test_sha256": digest(Path(__file__)),
            "shim_source_sha256": digest(SOURCE), "shim_sha256": digest(shim),
            "mode": "shell-handoff", "case": case,
            "passed": (case["status"] == 0 and case["displayed"]
                       and case["trace"] == "startup-armed\nhit\nactions-restored\nmask-restored\n"
                       and not case["shell_command_ran"] and case["terminal_restored"]),
        }
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report))
        return 0 if report["passed"] else 1
    with tempfile.TemporaryDirectory(prefix="rless-signal-install-") as directory:
        input_path = Path(directory) / "input"
        input_path.write_bytes(b"input must not be processed\n")
        reader, writer = os.pipe2(os.O_CLOEXEC)
        try:
            env = {
                "PATH": "/usr/bin:/bin", "HOME": directory,
                "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                "LD_PRELOAD": str(shim.resolve()),
                "RLESS_SIGNAL_INSTALL_TRACE_FD": str(writer),
                "RLESS_SIGNAL_INSTALL_FAIL_SIGNAL": str(getattr(signal, args.signal)),
            }
            if args.inherit_ignored_winch:
                env["RLESS_SIGNAL_INSTALL_EXPECT_IGNORED"] = "1"
            process = subprocess.run(
                [str(binary), str(input_path)], stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env,
                pass_fds=(writer,), timeout=5,
                preexec_fn=(lambda: signal.signal(signal.SIGWINCH, signal.SIG_IGN))
                if args.inherit_ignored_winch else None,
            )
            os.close(writer)
            writer = -1
            trace = os.read(reader, 4096).decode("ascii", errors="replace")
        finally:
            os.close(reader)
            if writer >= 0:
                os.close(writer)
    report = {
        "binary_sha256": digest(binary), "test_sha256": digest(Path(__file__)),
        "shim_source_sha256": digest(SOURCE), "shim_sha256": digest(shim),
        "status": process.returncode, "trace": trace,
        "failed_signal": args.signal,
        "inherited_ignored_winch": args.inherit_ignored_winch,
        "stdout_sha256": hashlib.sha256(process.stdout).hexdigest(),
        "stdout_len": len(process.stdout),
        "stderr_tail": process.stderr[-200:].decode("latin1"),
    }
    expected_trace = ("inherited-winch-ignored\n" if args.inherit_ignored_winch else "")
    expected_trace += "hit\nactions-restored\nmask-restored\n"
    report["passed"] = (
        process.returncode == 1 and process.stdout == b"" and
        trace == expected_trace
    )
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
