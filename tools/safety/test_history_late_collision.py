#!/usr/bin/env python3
"""Force a collision after the named history temp precheck and before create."""
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
SHIM_SOURCE = HERE / "native/history_late_collision_probe.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-history-late-") as directory:
        root = Path(directory)
        shim = root / "late.so"
        subprocess.run(["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
                        "-O2", str(SHIM_SOURCE), "-o", str(shim), "-ldl"],
                       check=True)
        source = root / "input"
        source.write_bytes(b"needle\n" * 80)
        marker = root / "marker"
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
            "RLESS_LATE_COLLISION_MARKER": str(marker),
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
            names = sorted(root.glob(".historyX.*"))
            markers = marker.read_text().splitlines() if marker.exists() else []
            report = {
                "binary_sha256": sha(binary),
                "shim_source_sha256": sha(SHIM_SOURCE),
                "shim_sha256": sha(shim),
                "test_sha256": sha(Path(__file__)),
                "ready": ready,
                "markers": markers,
                "status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "remaining_temp_names": [p.name for p in names],
                "remaining_contents": [p.read_bytes().decode("ascii", "replace") for p in names],
                "expect_vulnerable": args.expect_vulnerable,
            }
            expected = (ready and "fallback" in markers and "late-created" in markers
                        and "open-collision" in markers and report["status"] == 15
                        and report["terminal_restored"])
            if args.expect_vulnerable:
                expected &= names == []
            else:
                expected &= (len(names) == 1 and report["remaining_contents"] == ["unrelated\n"])
            report["expected_observed"] = expected
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(report, indent=2) + "\n")
            print(json.dumps(report))
            return 0 if expected else 1
        finally:
            if pager.poll() is None:
                os.killpg(pager.pid, signal.SIGKILL)
            pager.wait(timeout=2)
            os.close(master)


if __name__ == "__main__":
    raise SystemExit(main())
