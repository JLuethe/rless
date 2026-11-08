#!/usr/bin/env python3
"""A failed binary-probe seek must affect the final status and be diagnosed."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "compat"))
from run_v2 import run_pty

SOURCE = Path(__file__).with_name("native") / "seek_error_probe.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, library, mode):
    with tempfile.TemporaryDirectory(prefix="rless-seek-status-") as directory:
        root = Path(directory)
        (root / "input").write_bytes(b"needle\n" * 80)
        marker = root / "seek-hit"
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory,
            "TERM": "xterm-256color", "LC_ALL": "C",
            "LD_PRELOAD": str(library),
        }
        if mode:
            environment.update(RLESS_SEEK_ERROR_PROBE=mode, RLESS_SEEK_ERROR_MARKER=str(marker))
        scenario = {
            "id": "SEEK-STATUS-" + (mode or "normal"),
            "mode": "pty", "size": [24, 80], "files": {},
            "argv": ["-PsREADY", "-PmREADY", "-PMREADY", "input"],
            "events": [{"await": ["READY"]}, {"send": "q"}],
            "timeout": 5, "include_transcript": True,
        }
        observed = run_pty(
            [str(binary), *scenario["argv"]], environment, root, scenario,
            lambda data: data.replace(directory.encode(), b"{ROOT}"),
        )
        transcript = base64.b64decode(observed["transcript_b64"])
        return {
            "status": observed["status"],
            "terminal_restored": observed["termios_restored"],
            "seek_hits": marker.read_text().count("hit\n") if marker.exists() else 0,
            "diagnostic_present": b"rless: seek error" in transcript,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-seek-shim-") as directory:
        library = Path(directory) / "seek.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             "-O2", str(SOURCE), "-o", str(library), "-ldl"],
            check=True,
        )
        normal = observe(binary, library, None)
        injected = {mode: observe(binary, library, mode) for mode in ("probe", "reset")}
        expected = normal == {
            "status": 0, "terminal_restored": True,
            "seek_hits": 0, "diagnostic_present": False,
        }
        for error in injected.values():
            expected &= (error["seek_hits"] == 1 and error["terminal_restored"]
                         and (error["status"] == 0 and not error["diagnostic_present"]
                              if args.expect_vulnerable else
                              error["status"] == 1 and error["diagnostic_present"]))
        report = {
            "binary_sha256": sha(binary),
            "shim_source_sha256": sha(SOURCE),
            "shim_sha256": sha(library),
            "test_sha256": sha(Path(__file__)),
            "expect_vulnerable": args.expect_vulnerable,
            "normal": normal, "injected": injected,
            "expected_observed": expected,
        }
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report))
        return 0 if expected else 1


if __name__ == "__main__":
    raise SystemExit(main())
