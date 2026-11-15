#!/usr/bin/env python3
"""Signal after a worker commits the shared offset but before its reply."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time


SOURCE = Path(__file__).with_name("native") / "stall_inherited_seek.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, release_delay):
    with tempfile.TemporaryDirectory(prefix="rless-seek-commit-") as directory:
        root = Path(directory)
        shim = root / "shim.so"
        marker = root / "marker"
        release = root / "release"
        source = root / "input"
        output = root / "output"
        source.write_bytes(b"line-%06d\n" % 0 + b"".join(
            b"line-%06d\n" % index for index in range(1, 4000)))
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC", "-O2",
             str(SOURCE), "-o", str(shim), "-ldl"],
            check=True, capture_output=True,
        )
        input_fd = os.open(source, os.O_RDONLY)
        output_fd = os.open(output, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
        pager = None
        try:
            identity = os.fstat(input_fd)
            env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "dumb",
                   "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                   "RLESS_STALL_SEEK_DEV": str(identity.st_dev),
                   "RLESS_STALL_SEEK_INO": str(identity.st_ino),
                   "RLESS_STALL_SEEK_MARKER": str(marker),
                   "RLESS_STALL_SEEK_RELEASE": str(release),
                   "RLESS_STALL_SEEK_AFTER_COMMIT": "1"}
            pager = subprocess.Popen(
                [str(binary), "-"], stdin=input_fd, stdout=output_fd,
                stderr=subprocess.PIPE, env=env, close_fds=True,
            )
            deadline = time.monotonic() + 4
            while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                time.sleep(0.005)
            entries = marker.read_text().splitlines() if marker.exists() else []
            offset_at_marker = os.lseek(input_fd, 0, os.SEEK_CUR)
            if entries and pager.poll() is None:
                os.kill(pager.pid, signal.SIGWINCH)
            # A corrected commit wait may finish during this short interval.
            time.sleep(release_delay)
            release.touch()
            try:
                _, stderr = pager.communicate(timeout=4)
            except subprocess.TimeoutExpired:
                stderr = b""
            output_bytes = output.read_bytes()
            source_bytes = source.read_bytes()
            return {
                "binary_sha256": sha(binary), "source_sha256": sha(SOURCE),
                "shim_sha256": sha(shim), "marker_entries": entries,
                "offset_at_marker": offset_at_marker,
                "release_delay_seconds": release_delay,
                "status": pager.poll(),
                "output_sha256": hashlib.sha256(output_bytes).hexdigest(),
                "expected_sha256": hashlib.sha256(source_bytes).hexdigest(),
                "output_bytes": len(output_bytes), "expected_bytes": len(source_bytes),
                "stderr_tail": stderr[-200:].decode("utf-8", "replace"),
            }
        finally:
            release.touch()
            if pager is not None and pager.poll() is None:
                pager.kill()
                pager.wait(timeout=2)
            os.close(input_fd)
            os.close(output_fd)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expect-loss", action="store_true")
    parser.add_argument("--expect-error", action="store_true")
    parser.add_argument("--release-delay", type=float, default=0.03)
    args = parser.parse_args()
    report = observe(args.binary.resolve(strict=True), args.release_delay)
    common = bool(report["marker_entries"]) and report["offset_at_marker"] >= 8192
    if args.expect_error:
        passed = (common and report["status"] == 1
                  and report["output_sha256"] != report["expected_sha256"])
    elif args.expect_loss:
        passed = (common and report["status"] == 0
                  and report["output_sha256"] != report["expected_sha256"])
    else:
        passed = (common and report["status"] == 0
                  and report["output_sha256"] == report["expected_sha256"])
    report["expect_loss"] = args.expect_loss
    report["expect_error"] = args.expect_error
    report["passed"] = passed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
