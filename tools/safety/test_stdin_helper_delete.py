#!/usr/bin/env python3
"""Deleting retained stdin must release its LESSOPEN helper and owned FDs."""

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


def read_until(master: int, process: subprocess.Popen, marker: bytes, seconds: float) -> bytes:
    data = bytearray()
    deadline = time.monotonic() + seconds
    while marker not in data and process.poll() is None and time.monotonic() < deadline:
        ready, _, _ = select.select([master], [], [], 0.05)
        if ready:
            try:
                data.extend(os.read(master, 65536))
            except OSError:
                break
    return bytes(data)


def descriptor_targets(pid: int) -> dict[str, str]:
    root = Path(f"/proc/{pid}/fd")
    try:
        entries = list(root.iterdir())
    except (FileNotFoundError, ProcessLookupError):
        return {}
    targets = {}
    for entry in entries:
        try:
            targets[entry.name] = os.readlink(entry)
        except (FileNotFoundError, ProcessLookupError):
            # A transient worker descriptor may close during enumeration.
            continue
    return targets


def process_state(pid: int) -> str:
    try:
        return Path(f"/proc/{pid}/stat").read_text().split()[2]
    except (FileNotFoundError, ProcessLookupError):
        return "gone"


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-stdin-helper-delete-") as directory:
        root = Path(directory)
        (root / "i").write_text("file\n" * 80)
        script = root / "helper"
        script.write_text(
            "#!/bin/sh\n"
            "[ \"$1\" = - ] || exit 1\n"
            f"echo $$ > {root}/helper.pid\n"
            "i=0; while [ \"$i\" -lt 80 ]; do echo processed; i=$((i+1)); done\n"
            "exec sleep 60\n"
        )
        script.chmod(0o700)
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        process = subprocess.Popen(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", "-", "i"],
            stdin=subprocess.PIPE, stdout=slave, stderr=slave,
            start_new_session=True, cwd=root,
            env={"PATH": "/usr/bin:/bin", "HOME": directory,
                 "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                 "LESSOPEN": f"|-{script} %s"},
        )
        os.close(slave)
        helper_pid = None
        try:
            assert process.stdin is not None
            process.stdin.write(b"input\n" * 80)
            process.stdin.close()
            first = read_until(master, process, b"READY", 4)
            if (root / "helper.pid").exists():
                helper_pid = int((root / "helper.pid").read_text())
            before = descriptor_targets(process.pid) if process.poll() is None else {}
            # The helper's stdout is the parent's retained LESSOPEN read end.
            # Compare the kernel pipe identity, since unrelated worker FDs may
            # replace the released FDs when the next file opens.
            helper_pipe = (descriptor_targets(helper_pid).get("1")
                           if helper_pid is not None else None)
            if b"READY" in first and process.poll() is None:
                os.write(master, b":d\n")
            second = read_until(master, process, b"READY", 4)
            deadline = time.monotonic() + 0.5
            while helper_pid is not None and process_state(helper_pid) not in {"gone", "Z"} \
                    and time.monotonic() < deadline:
                time.sleep(0.02)
            after = descriptor_targets(process.pid) if process.poll() is None else {}
            state = process_state(helper_pid) if helper_pid is not None else "missing-pid"
            if process.poll() is None:
                os.write(master, b"q")
                read_until(master, process, b"\x1b[?1049l", 1)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            return {
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "first_ready": b"READY" in first,
                "second_ready": b"READY" in second,
                "second_file_visible": b"file" in second,
                "fd0_before": before.get("0"),
                "fd0_after": after.get("0"),
                "fd_count_before": len(before),
                "fd_count_after": len(after),
                "helper_pipe_target": helper_pipe,
                "helper_pipe_owned_before": helper_pipe is not None and helper_pipe in before.values(),
                "helper_pipe_owned_after": helper_pipe is not None and helper_pipe in after.values(),
                "helper_state_after_delete": state,
                "status": process.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
            }
        finally:
            if process.poll() is None:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            process.wait(timeout=2)
            if helper_pid is not None and process_state(helper_pid) not in {"gone", "Z"}:
                try:
                    os.kill(helper_pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-leak", action="store_true")
    args = parser.parse_args()
    result = observe(args.binary.resolve(strict=True))
    common = (result["first_ready"] and result["second_ready"]
              and result["second_file_visible"] and result["status"] == 0
              and result["terminal_restored"]
              and result["fd0_before"] == result["fd0_after"])
    if args.expect_leak:
        passed = (common and result["helper_state_after_delete"] not in {"gone", "Z"}
                  and result["helper_pipe_owned_before"]
                  and result["helper_pipe_owned_after"])
    else:
        passed = (common and result["helper_state_after_delete"] in {"gone", "Z"}
                  and result["helper_pipe_owned_before"]
                  and not result["helper_pipe_owned_after"])
    result["expected_outcome_observed"] = bool(passed)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
