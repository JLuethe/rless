#!/usr/bin/env python3
"""A failed inherited-SIGPIPE query or restoration must stop startup."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile


SOURCE = Path(__file__).with_name("native") / "sigpipe_startup_failure.c"
PAYLOAD = b"sigpipe startup control\n"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, shim: Path, mode: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-sigpipe-startup-") as directory:
        root = Path(directory)
        input_path = root / "input"
        input_path.write_bytes(PAYLOAD)
        read_fd, write_fd = os.pipe2(os.O_CLOEXEC)
        try:
            env = {
                "PATH": "/usr/bin:/bin", "HOME": directory,
                "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                "LD_PRELOAD": str(shim), "RLESS_SIGPIPE_FAILURE": mode,
                "RLESS_SIGPIPE_TRACE_FD": str(write_fd),
            }
            pager = subprocess.Popen(
                [str(binary), str(input_path)], stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env,
                pass_fds=(write_fd,),
            )
            os.close(write_fd)
            write_fd = -1
            try:
                stdout, stderr = pager.communicate(timeout=3)
                timed_out = False
            except subprocess.TimeoutExpired:
                pager.kill()
                stdout, stderr = pager.communicate(timeout=3)
                timed_out = True
            trace = os.read(read_fd, 128)
            return {
                "mode": mode, "status": pager.returncode,
                "timed_out": timed_out, "trace": trace.decode("ascii"),
                "stdout_sha256": digest(stdout), "stdout_bytes": len(stdout),
                "stderr": stderr.decode("latin1"),
            }
        finally:
            os.close(read_fd)
            if write_fd >= 0:
                os.close(write_fd)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.parent / (args.output.stem + ".so")
    subprocess.run(
        ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    cases = [observe(binary, shim.resolve(), mode)
             for mode in ("none", "query", "restore")]
    def valid(case: dict) -> bool:
        if case["timed_out"] or case["stderr"]:
            return False
        if case["mode"] == "none" or args.expect_vulnerable:
            return (case["status"] == 0 and case["stdout_bytes"] == len(PAYLOAD)
                    and case["stdout_sha256"] == digest(PAYLOAD)
                    and case["trace"] == ("" if case["mode"] == "none"
                                          else case["mode"] + "\n"))
        return (case["status"] == 1 and case["stdout_bytes"] == 0
                and case["trace"] == case["mode"] + "\n")
    report = {
        "binary_sha256": digest(binary.read_bytes()),
        "test_sha256": digest(Path(__file__).read_bytes()),
        "shim_source_sha256": digest(SOURCE.read_bytes()),
        "shim_sha256": digest(shim.read_bytes()),
        "expect_vulnerable": args.expect_vulnerable,
        "passed": all(map(valid, cases)), "cases": cases,
    }
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "cases"}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
