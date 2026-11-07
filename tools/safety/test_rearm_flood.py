#!/usr/bin/env python3
"""Check that a continuing external SIGALRM stream cannot starve timer rearm."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import time


TEST = "signal::notification_tests::continuing_external_alarm_stream_refuses_rearm"
BASELINE_TEST = "signal::notification_tests::pending_external_alarm_is_drained_before_a_new_timer"
SOURCE = Path(__file__).resolve().parent / "native/rearm_flood.c"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary: Path, shim: Path, root: Path, timeout: float,
            test: str, mode: str) -> dict:
    trace = root / "hit"
    environment = {
        "PATH": "/usr/bin:/bin",
        "HOME": str(root),
        "LD_PRELOAD": str(shim),
        "RLESS_REARM_TRACE": str(trace),
        "RLESS_REARM_MODE": mode,
        "RUST_TEST_THREADS": "1",
    }
    if mode == "all":
        environment["RLESS_TEST_REARM_FLOOD"] = "1"
    started = time.monotonic()
    try:
        process = subprocess.run(
            [str(binary), test, "--exact", "--nocapture"],
            env=environment, capture_output=True, timeout=timeout, check=False,
        )
        status = process.returncode
        timed_out = False
        stdout = process.stdout
        stderr = process.stderr
    except subprocess.TimeoutExpired as error:
        status = None
        timed_out = True
        stdout = error.stdout or b""
        stderr = error.stderr or b""
    return {
        "binary": str(binary), "binary_sha256": sha(binary), "test": test,
        "shim_hit": trace.read_bytes() == b"hit\n" if trace.exists() else False,
        "status": status, "timed_out": timed_out,
        "elapsed_seconds": round(time.monotonic() - started, 3),
        "stdout": stdout.decode("utf-8", "replace")[-2048:],
        "stderr": stderr.decode("utf-8", "replace")[-2048:],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--test-binary", required=True, type=Path)
    parser.add_argument("--pre-fix-test-binary", type=Path)
    parser.add_argument("--timeout", type=float, default=0.25)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    if args.timeout <= 0:
        parser.error("timeout must be positive")
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="rless-rearm-flood-") as directory:
        root = Path(directory)
        shim = root / "rearm_flood.so"
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
             str(SOURCE), "-ldl", "-o", str(shim)], check=True,
        )
        candidate_dir = root / "candidate"
        candidate_dir.mkdir()
        candidate = observe(args.test_binary.resolve(strict=True), shim,
                            candidate_dir, args.timeout, TEST, "all")
        baseline = None
        if args.pre_fix_test_binary:
            baseline_dir = root / "baseline"
            baseline_dir.mkdir()
            baseline = observe(args.pre_fix_test_binary.resolve(strict=True), shim,
                               baseline_dir, args.timeout, BASELINE_TEST, "after-real")
        passed = (candidate["shim_hit"] and candidate["status"] == 0
                  and not candidate["timed_out"] and "1 passed" in candidate["stdout"])
        if baseline is not None:
            passed &= baseline["shim_hit"] and baseline["timed_out"]
        report = {
            "passed": passed, "timeout_seconds": args.timeout,
            "source_sha256": sha(SOURCE), "candidate": candidate,
            "baseline": baseline,
        }
        output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({"passed": passed, "candidate": candidate["status"],
                          "baseline_timed_out": baseline["timed_out"] if baseline else None,
                          "output": str(output)}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
