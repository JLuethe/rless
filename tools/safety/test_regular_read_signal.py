#!/usr/bin/env python3
"""Inject a post-readiness regular-file stall and test SIGTERM dispatch."""

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


SHIM = Path(__file__).with_name("native") / "stall_regular_read.c"


def observe(binary: Path, mode: str, kind: str = "regular") -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-regular-read-") as directory:
        root = Path(directory)
        shim = root / "stall.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-O2", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(shim)], check=True, capture_output=True,
        )
        source = root / "regular-input"
        source.write_bytes(b"line\n" * 200)
        marker = root / "read-hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
            "RLESS_REGULAR_READ_TRACE": str(marker),
            "RLESS_REGULAR_READ_MODE": mode,
            "RLESS_REGULAR_READ_TARGET": "null" if kind == "character" else "regular-input",
        }
        env.update({key: os.environ[key] for key in ("ASAN_OPTIONS", "LSAN_OPTIONS") if key in os.environ})
        input_stream = open(os.devnull, "rb") if kind == "character" else None
        pager = subprocess.Popen(
            [str(binary), "-" if kind == "character" else str(source)],
            stdin=input_stream if input_stream is not None else slave,
            stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        if input_stream is not None:
            input_stream.close()
        os.close(slave)
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            state = marker.read_text() if marker.exists() else None
            try:
                children = [int(pid) for pid in Path(
                    f"/proc/{pager.pid}/task/{pager.pid}/children"
                ).read_text().split()] if state is not None else []
            except FileNotFoundError:
                children = []
            if state is not None and pager.poll() is None:
                os.kill(pager.pid, signal.SIGTERM)
            deadline = time.monotonic() + 0.75
            while pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            status = pager.poll()
            try:
                wchan = Path(f"/proc/{pager.pid}/wchan").read_text().strip() if status is None else None
            except (FileNotFoundError, ProcessLookupError):
                wchan = None
            restored = termios.tcgetattr(master) == original
            child_deadline = time.monotonic() + 0.5
            while any(Path(f"/proc/{pid}").exists() for pid in children) and time.monotonic() < child_deadline:
                time.sleep(0.01)
            return {
                "shim_hit": state is not None,
                "sigterm_masked_at_read": state == "B",
                "pager_status_after_sigterm": status,
                "wait_channel": wchan,
                "terminal_restored": restored,
                "child_pids_at_stall": children,
                "children_gone_after_exit": all(not Path(f"/proc/{pid}").exists() for pid in children),
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def observe_resize(binary: Path, path_swap: bool = False) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-regular-resize-") as directory:
        root = Path(directory)
        shim = root / "stall.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-O2", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(shim)], check=True, capture_output=True,
        )
        source = root / "regular-input"
        source.write_bytes(b"line\n" * 200)
        marker = root / "read-hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
            "RLESS_REGULAR_READ_TRACE": str(marker),
            "RLESS_REGULAR_READ_MODE": "once",
        }
        env.update({key: os.environ[key] for key in ("ASAN_OPTIONS", "LSAN_OPTIONS") if key in os.environ})
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 3
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            try:
                read_children = [int(pid) for pid in Path(
                    f"/proc/{pager.pid}/task/{pager.pid}/children"
                ).read_text().split()] if marker.exists() else []
            except FileNotFoundError:
                read_children = []
            if path_swap and marker.exists() and pager.poll() is None:
                source.rename(root / "original-moved")
                source.write_bytes(b"ATTACK\n" * 200)
            if marker.exists() and pager.poll() is None:
                os.kill(pager.pid, signal.SIGWINCH)
            deadline = time.monotonic() + 2
            while b"line" not in output and pager.poll() is None and time.monotonic() < deadline:
                readable, _, _ = select.select([master], [], [], 0.05)
                if readable:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            displayed = b"line" in output
            if pager.poll() is None:
                os.write(master, b"q")
            try:
                status = pager.wait(timeout=2)
            except subprocess.TimeoutExpired:
                status = None
            child_deadline = time.monotonic() + 0.5
            while any(Path(f"/proc/{pid}").exists() for pid in read_children) and time.monotonic() < child_deadline:
                time.sleep(0.01)
            original_preserved = not path_swap or b"ATTACK" not in output
            children_gone = all(not Path(f"/proc/{pid}").exists() for pid in read_children)
            return {
                "shim_hit": marker.exists(),
                "sigterm_masked_at_read": marker.read_text() == "B" if marker.exists() else None,
                "path_swapped": path_swap,
                "content_displayed_after_resize": displayed,
                "replacement_content_absent": original_preserved,
                "pager_status": status,
                "terminal_restored": termios.tcgetattr(master) == original,
                "read_children_at_stall": read_children,
                "read_children_gone": children_gone,
                "passed": marker.exists() and displayed and original_preserved and children_gone
                          and status == 0
                          and termios.tcgetattr(master) == original,
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--kind", choices=("regular", "character"), default="regular")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    case = observe(binary, "masked", args.kind)
    control = observe(binary, "unmasked", args.kind)
    resize = observe_resize(binary) if args.kind == "regular" and not args.expect_blocked else None
    swapped = observe_resize(binary, path_swap=True) if resize is not None else None
    control_valid = (control["shim_hit"] and not control["sigterm_masked_at_read"]
                     and control["pager_status_after_sigterm"] == 15
                     and control["terminal_restored"])
    blocked = (case["shim_hit"] and case["sigterm_masked_at_read"]
               and case["pager_status_after_sigterm"] is None and control_valid)
    fixed = (case["shim_hit"] and case["pager_status_after_sigterm"] == 15
             and bool(case["child_pids_at_stall"]) and case["children_gone_after_exit"]
             and case["terminal_restored"] and control_valid
             and (args.kind == "character" or resize is not None and resize["passed"]
                  and swapped is not None and swapped["passed"]))
    report = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "shim_sha256": hashlib.sha256(SHIM.read_bytes()).hexdigest(),
        "kind": args.kind,
        "case": case,
        "unmasked_control": control,
        "resize_cancel_preserves_input": resize,
        "resize_after_path_swap_preserves_inode": swapped,
        "blocked": blocked,
        "fixed": fixed,
        "expected_outcome_observed": blocked if args.expect_blocked else fixed,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["expected_outcome_observed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
