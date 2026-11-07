#!/usr/bin/env python3
"""Interrupt a regular-file stdout write and check SIGTERM stops output."""

import argparse
import hashlib
import json
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
        marker = root / "marker"
        with output_path.open("wb") as output:
            pager = subprocess.Popen(
                [str(binary), str(root / "input")],
                stdin=subprocess.DEVNULL, stdout=output,
                stderr=subprocess.PIPE,
                env={"PATH": "/usr/bin:/bin", "HOME": directory,
                     "TERM": "xterm-256color", "LC_ALL": "C",
                     "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                     "RLESS_REGULAR_SIGNAL_MARKER": str(marker)},
            )
            try:
                _, stderr = pager.communicate(timeout=2)
            except subprocess.TimeoutExpired:
                pager.kill()
                _, stderr = pager.communicate(timeout=2)
        written = output_path.read_bytes()
        return {
            "status": pager.returncode, "trace_hit": marker.exists() and marker.read_bytes() == b"hit\n",
            "output_bytes": len(written), "input_bytes": len(payload),
            "output_sha256": sha(written), "stderr": stderr.decode("latin1"),
        }


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
        # A child may finish bytes already in flight when it notifies the
        # pager. The pager must stop well before the complete input is copied.
        if args.expect_continued:
            return (case["trace_hit"] and case["status"] == 15
                    and 0 < case["output_bytes"] <= case["input_bytes"])
        return (case["trace_hit"] and case["status"] == 15
                and 0 <= case["output_bytes"] <= 8192
                and case["stderr"] in ("", "rless: write error\n"))
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
