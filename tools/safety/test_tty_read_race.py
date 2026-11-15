#!/usr/bin/env python3
"""Prove stolen terminal readiness cannot strand a masked pager read."""

import argparse
import errno
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import subprocess
import tempfile
import time


SHIM_SOURCE = Path(__file__).parent / "native/ppoll_tty_drain.c"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def process_state(pid: int) -> tuple[str | None, bool | None]:
    try:
        wchan = Path(f"/proc/{pid}/wchan").read_text().strip()
        status = Path(f"/proc/{pid}/status").read_text()
    except FileNotFoundError:
        return None, None
    blocked = next((line.split()[1] for line in status.splitlines()
                    if line.startswith("SigBlk:")), None)
    return wchan, (bool(int(blocked, 16) & (1 << (signal.SIGTERM - 1)))
                   if blocked is not None else None)


def observe(binary: Path, shim: Path, raw_output: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-tty-drain-") as directory:
        work = Path(directory)
        data = work / "data"
        data.write_bytes(b"line\n" * 50)
        trace = work / "hit"
        master, slave = pty.openpty()
        env = dict(
            PATH="/usr/bin:/bin", HOME=directory, TERM="xterm-256color",
            LC_ALL="C", LESSHISTFILE="-", LD_PRELOAD=str(shim),
            RLESS_TTY_DRAIN_TRACE=str(trace),
        )
        process = subprocess.Popen(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", str(data)],
            stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        transcript = bytearray()
        wchan = None
        sigterm_blocked = None
        try:
            deadline = time.monotonic() + 3
            while time.monotonic() < deadline and b"READY" not in transcript:
                readable, _, _ = select.select([master], [], [], 0.02)
                if readable:
                    try:
                        transcript.extend(os.read(master, 65536))
                    except OSError as error:
                        if error.errno != errno.EIO:
                            raise
                        break
                if process.poll() is not None:
                    break
            if b"READY" not in transcript:
                raise AssertionError("pager did not reach the command prompt")
            os.write(master, b"q")
            deadline = time.monotonic() + 1
            hit = b""
            while time.monotonic() < deadline:
                if trace.exists():
                    hit = trace.read_bytes()
                    break
                if process.poll() is not None:
                    break
                time.sleep(0.01)
            if hit == b"hit\n" and process.poll() is None:
                time.sleep(0.1)
                wchan, sigterm_blocked = process_state(process.pid)
                os.kill(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            raw_output.write_bytes(transcript)
            return dict(
                binary=str(binary), binary_sha256=sha(binary.read_bytes()),
                fault_hit=hit == b"hit\n", trace_sha256=sha(hit),
                status=process.poll(), pre_signal_wchan=wchan,
                pre_signal_sigterm_blocked=sigterm_blocked,
                transcript_sha256=sha(transcript), transcript_bytes=len(transcript),
            )
        finally:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--pre-fix", type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    args = parser.parse_args()
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="rless-tty-shim-") as directory:
        shim = Path(directory) / "ppoll_tty_drain.so"
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
             str(SHIM_SOURCE), "-ldl", "-o", str(shim)],
            check=True,
        )
        observations = [observe(args.binary.resolve(), shim, output.parent / f"candidate-{i}.raw")
                        for i in range(args.rounds)]
        pre_fix = (observe(args.pre_fix.resolve(), shim, output.parent / "pre-fix.raw")
                   if args.pre_fix else None)
        passed = all(case["fault_hit"] and case["status"] == 15 for case in observations)
        if pre_fix:
            passed &= (pre_fix["fault_hit"] and pre_fix["status"] is None
                       and pre_fix["pre_signal_sigterm_blocked"] is True)
        report = dict(
            passed=passed, rounds=args.rounds,
            shim_source_sha256=sha(SHIM_SOURCE.read_bytes()),
            shim_sha256=sha(shim.read_bytes()),
            candidate=observations, pre_fix=pre_fix,
        )
        output.write_text(json.dumps(report, indent=2) + "\n")
        print(f"TTY drain: {sum(c['fault_hit'] and c['status'] == 15 for c in observations)}/{len(observations)} candidate cases; pre-fix={pre_fix and pre_fix['fault_hit'] and pre_fix['status'] is None and pre_fix['pre_signal_sigterm_blocked'] is True}")
        return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
