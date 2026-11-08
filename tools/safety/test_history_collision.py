#!/usr/bin/env python3
"""Reject a named history-temp collision without deleting the prior file."""
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

HERE = Path(__file__).resolve().parent
SHIM_SOURCE = HERE / "native/history_collision_probe.c"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(binary, output_path, expect_vulnerable):
    with tempfile.TemporaryDirectory(prefix="rless-history-collision-") as directory:
        root = Path(directory)
        shim = root / "history-collision.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             "-O2", str(SHIM_SOURCE), "-o", str(shim), "-ldl"],
            check=True,
        )
        source = root / "input"
        source.write_bytes(b"needle\n" * 80)
        existing = root / (".historyX." + "42" * 16)
        before = b"preexisting unrelated file\n"
        existing.write_bytes(before)
        existing.chmod(0o600)
        marker = root / "collision-hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)

        def setup_tty():
            os.setsid()
            fcntl.ioctl(slave, termios.TIOCSCTTY, 0)

        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory,
            "TERM": "xterm-256color", "LC_ALL": "C",
            "LESSHISTFILE": str(root / "historyX"),
            "LD_PRELOAD": str(shim),
            "RLESS_COLLISION_PROBE": "1",
            "RLESS_COLLISION_MARKER": str(marker),
        }
        pager = subprocess.Popen(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", str(source)],
            stdin=slave, stdout=slave, stderr=slave, preexec_fn=setup_tty,
            close_fds=True, env=environment,
        )
        os.close(slave)
        output = bytearray()
        try:
            deadline = time.monotonic() + 4
            while b"READY" not in output and pager.poll() is None and time.monotonic() < deadline:
                if select.select([master], [], [], 0.05)[0]:
                    output.extend(os.read(master, 65536))
            ready = b"READY" in output
            if ready:
                os.write(master, b"/needle\n")
                time.sleep(0.15)
                os.write(master, b"q")
            try:
                pager.wait(timeout=4)
            except subprocess.TimeoutExpired:
                pass
            markers = marker.read_text().splitlines() if marker.exists() else []
            result = {
                "binary_sha256": digest(binary),
                "shim_source_sha256": digest(SHIM_SOURCE),
                "shim_sha256": digest(shim),
                "probe_sha256": digest(Path(__file__)),
                "ready": ready,
                "markers": markers,
                "status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "preexisting_present": existing.exists(),
                "preexisting_unchanged": existing.exists() and existing.read_bytes() == before,
                "history_exists": (root / "historyX").exists(),
                "remaining_temp_names": sorted(p.name for p in root.glob(".historyX.*")),
                "expect_vulnerable": expect_vulnerable,
            }
            common = ready and "random" in markers and "fallback" in markers
            if expect_vulnerable:
                expected = (common and "open-collision" in markers
                            and result["status"] == 15
                            and result["terminal_restored"]
                            and not result["preexisting_present"])
            else:
                expected = (common and "stat-collision" in markers
                            and "open-collision" not in markers
                            and result["status"] == 1
                            and result["terminal_restored"]
                            and result["preexisting_unchanged"]
                            and not result["history_exists"]
                            and result["remaining_temp_names"] == [existing.name])
            result["expected_observed"] = expected
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(json.dumps(result, indent=2) + "\n")
            print(json.dumps(result))
            return 0 if expected else 1
        finally:
            if pager.poll() is None:
                os.killpg(pager.pid, signal.SIGKILL)
            pager.wait(timeout=2)
            os.close(master)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    raise SystemExit(run(args.binary.resolve(strict=True), args.output, args.expect_vulnerable))
