#!/usr/bin/env python3
"""Interrupt a regular-file stdout write and check SIGTERM stops output."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile


SOURCE = Path(__file__).with_name("native") / "output_regular_signal.c"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, shim: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-regular-output-signal-") as directory:
        root = Path(directory)
        payload = b"line 0123456789abcdef\n" * 10000
        (root / "input").write_bytes(payload)
        output_path = root / "output"
        trace_read, trace_write = os.pipe2(os.O_CLOEXEC)
        try:
            with output_path.open("wb") as output:
                pager = subprocess.Popen(
                    [str(binary), str(root / "input")],
                    stdin=subprocess.DEVNULL, stdout=output,
                    stderr=subprocess.PIPE, pass_fds=(trace_write,),
                    env={"PATH": "/usr/bin:/bin", "HOME": directory,
                         "TERM": "xterm-256color", "LC_ALL": "C",
                         "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                         "RLESS_REGULAR_SIGNAL_TRACE_FD": str(trace_write)},
                )
                os.close(trace_write)
                trace_write = -1
                try:
                    _, stderr = pager.communicate(timeout=2)
                except subprocess.TimeoutExpired:
                    pager.kill()
                    _, stderr = pager.communicate(timeout=2)
            trace = os.read(trace_read, 64)
            written = output_path.read_bytes()
            return {
                "status": pager.returncode, "trace_hit": trace == b"hit\n",
                "output_bytes": len(written), "input_bytes": len(payload),
                "output_sha256": sha(written), "stderr": stderr.decode("latin1"),
            }
        finally:
            os.close(trace_read)
            if trace_write >= 0:
                os.close(trace_write)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--expect-continued", action="store_true")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("rounds must be positive")
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.parent / (args.output.stem + ".so")
    subprocess.run(
        ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    cases = [observe(binary, shim.resolve()) for _ in range(args.rounds)]
    def valid(case: dict) -> bool:
        return (case["trace_hit"] and case["status"] == 15
                and ((0 < case["output_bytes"] <= case["input_bytes"])
                     if args.expect_continued else case["output_bytes"] == 0)
                and case["stderr"] == ("" if args.expect_continued
                                       else "rless: write error\n"))
    passed_count = sum(valid(case) for case in cases)
    report = {
        "binary_sha256": sha(binary.read_bytes()),
        "test_sha256": sha(Path(__file__).read_bytes()),
        "shim_source_sha256": sha(SOURCE.read_bytes()),
        "shim_sha256": sha(shim.read_bytes()),
        "expect_continued": args.expect_continued,
        "rounds": args.rounds, "passed_count": passed_count,
        "passed": passed_count == args.rounds, "cases": cases,
    }
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "cases"}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
