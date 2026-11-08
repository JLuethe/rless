#!/usr/bin/env python3
"""A permanent LESSOPEN stdout read error must affect the final status."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile


SOURCE = Path(__file__).with_name("native") / "lessopen_read_error.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, library, mode, inject, empty=False, close_error=False):
    with tempfile.TemporaryDirectory(prefix="rless-lessopen-read-") as directory:
        root = Path(directory)
        original = root / "original"
        alternate = root / "alternate"
        helper = root / "helper"
        ready = root / "ready"
        marker = root / "hit"
        original.write_bytes(b"ORIGINAL\n")
        alternate.write_bytes(b"ALTERNATE\n")
        helper.write_text(
            '#!/bin/sh\n'
            'touch "$RLESS_LESSOPEN_READY"\n'
            'if [ "$RLESS_LESSOPEN_EMPTY" = 1 ]; then exit 0; fi\n'
            'if [ "$RLESS_LESSOPEN_MODE" = filename ]; then\n'
            '    printf "%s\\n" "$RLESS_LESSOPEN_ALT"\n'
            'else\n'
            '    printf "ALTERNATE\\n"\n'
            'fi\n'
        )
        helper.chmod(0o700)
        prefix = {"filename": "", "pipe": "|", "double-pipe": "||"}[mode]
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "LC_ALL": "C",
            "TERM": "xterm-256color", "LESSHISTFILE": "-",
            "LESSOPEN": f"{prefix}{helper} %s",
            "RLESS_LESSOPEN_READY": str(ready),
            "RLESS_LESSOPEN_READ_HIT": str(marker),
            "RLESS_LESSOPEN_READ_LENGTH": "4096" if mode == "filename" else "1",
            "RLESS_LESSOPEN_MODE": mode, "RLESS_LESSOPEN_ALT": str(alternate),
            "RLESS_LESSOPEN_EMPTY": "1" if empty else "0",
        }
        if inject:
            environment["LD_PRELOAD"] = str(library)
        if close_error:
            environment.pop("RLESS_LESSOPEN_READ_LENGTH")
            environment["RLESS_LESSOPEN_CLOSE_ERROR"] = "1"
        result = subprocess.run(
            [str(binary), str(original)], env=environment, cwd=root,
            stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            timeout=5, check=False,
        )
        return {
            "status": result.returncode,
            "stdout": result.stdout.decode("ascii", "replace"),
            "stderr": result.stderr.decode("ascii", "replace"),
            "ready": ready.exists(), "hit": marker.read_bytes() == b"hit\n" if marker.exists() else False,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expect-vulnerable", action="store_true")
    parser.add_argument("--expect-vulnerable-close", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-lessopen-shim-") as directory:
        library = Path(directory) / "read_error.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror", "-O2",
             str(SOURCE), "-o", str(library), "-ldl"], check=True,
        )
        cases = {}
        passed = True
        for mode in ("filename", "pipe", "double-pipe"):
            normal = observe(binary, library, mode, False)
            empty = observe(binary, library, mode, False, empty=True)
            fault = observe(binary, library, mode, True)
            cases[mode] = {"normal": normal, "empty": empty, "fault": fault}
            passed &= normal["status"] == 0 and normal["stdout"] == "ALTERNATE\n"
            passed &= normal["ready"] and not normal["hit"]
            passed &= empty["status"] == 0 and empty["ready"] and not empty["hit"]
            passed &= empty["stdout"] == ("" if mode == "double-pipe" else "ORIGINAL\n")
            passed &= empty["stderr"] == ""
            passed &= fault["ready"] and fault["hit"]
            if args.expect_vulnerable:
                passed &= fault["status"] == 0 and fault["stderr"] == ""
            else:
                passed &= fault["status"] == 1
                passed &= "input preprocessor I/O error" in fault["stderr"]
        close_fault = observe(binary, library, "pipe", True, close_error=True)
        cases["pipe-close"] = close_fault
        passed &= close_fault["ready"] and close_fault["hit"]
        passed &= close_fault["stdout"].startswith("ALTERNATE\n")
        passed &= "pclose: Input/output error" in close_fault["stdout"]
        if args.expect_vulnerable or args.expect_vulnerable_close:
            passed &= close_fault["status"] == 0
        else:
            passed &= close_fault["status"] == 1
            passed &= "input preprocessor I/O error" in close_fault["stderr"]
        report = {
            "binary_sha256": sha(binary), "shim_source_sha256": sha(SOURCE),
            "shim_sha256": sha(library), "test_sha256": sha(Path(__file__)),
            "expect_vulnerable": args.expect_vulnerable,
            "expect_vulnerable_close": args.expect_vulnerable_close,
            "cases": cases, "expected_observed": bool(passed),
        }
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report))
        return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
