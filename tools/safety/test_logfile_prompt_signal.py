#!/usr/bin/env python3
"""Interrupt an existing-logfile prompt before terminal initialization."""

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "compat"))
from run_v2 import run_pty


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, signal_name: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-log-prompt-signal-") as directory:
        root = Path(directory)
        original = b"OLD\n"
        log = root / "capture"
        log.write_bytes(original)
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                environment[name] = os.environ[name]
        events = [{"await": ["exists"]}]
        if signal_name == "SIGTERM":
            events.append({"signal": signal_name})
        else:
            events.extend([{"signal": signal_name, "await": ["READY"]}, {"send": "q"}])
        case = {
            "id": "LOG-PROMPT-" + signal_name,
            "mode": "pty",
            "stream_b64": base64.b64encode(b"input\n" * 100).decode(),
            "events": events,
        }
        try:
            result = run_pty(
                [str(binary), "-o", "capture", "-PsREADY", "-PmREADY", "-PMREADY", "-"],
                environment, root, case, lambda data: data,
            )
        except TimeoutError as error:
            return {"signal": signal_name, "passed": False, "error": str(error)}
        expected_status = 15 if signal_name == "SIGTERM" else 0
        passed = (
            result["status"] == expected_status and result["termios_restored"]
            and log.read_bytes() == original
        )
        return {
            "signal": signal_name, "status": result["status"],
            "terminal_restored": result["termios_restored"],
            "log_sha256": sha(log.read_bytes()), "passed": passed,
        }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary, signal_name) for signal_name in ("SIGTERM", "SIGWINCH")]
    report = {
        "binary": str(binary), "binary_sha256": sha(binary.read_bytes()),
        "observer_sha256": sha((Path(__file__).resolve().parents[1] / "compat/run_v2.py").read_bytes()),
        "cases": cases, "passed": all(case["passed"] for case in cases),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
