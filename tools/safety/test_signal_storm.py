#!/usr/bin/env python3
"""Exercise mixed signal delivery while the pager waits for a terminal command."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "compat"))
from run_v2 import run_pty

STORM_SIGNALS = (signal.SIGUSR1, signal.SIGWINCH, signal.SIGALRM, signal.SIGINT)


def observe(binary: Path, burst: int) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-signal-storm-") as directory:
        root = Path(directory)
        (root / "input").write_bytes(b"line\n" * 200)
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LESS_SIGUSR1": "g",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                environment[name] = os.environ[name]
        case = {
            "id": "SIGNAL-STORM", "mode": "pty",
            "events": [{"await": ["READY"], "heap_checkpoint": "storm"}],
        }
        sent = 0

        def storm(pid: int, label: str) -> None:
            nonlocal sent
            if label != "storm":
                raise ValueError("wrong signal barrier")
            for _ in range(burst):
                for number in STORM_SIGNALS:
                    os.kill(pid, number)
                    sent += 1
            os.kill(pid, signal.SIGTERM)
            sent += 1

        result = run_pty(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", "input"],
            environment, root, case, lambda data: data,
            checkpoint_observer=storm,
        )
        return {
            "signals_sent": sent, "status": result["status"],
            "terminal_restored": result["termios_restored"],
            "passed": sent == burst * len(STORM_SIGNALS) + 1
                and result["status"] == 15 and result["termios_restored"],
        }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--rounds", type=int, default=20)
    parser.add_argument("--burst", type=int, default=200)
    args = parser.parse_args()
    if args.rounds < 1 or args.burst < 1:
        parser.error("rounds and burst must be positive")
    binary = args.binary.resolve(strict=True)
    cases = [observe(binary, args.burst) for _ in range(args.rounds)]
    report = {
        "binary": str(binary), "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "rounds": args.rounds, "burst": args.burst,
        "passed": sum(case["passed"] for case in cases), "cases": cases,
    }
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"signal storm: {report['passed']}/{args.rounds} passed, {args.burst} mixed bursts each")
    return 0 if report["passed"] == args.rounds else 1


if __name__ == "__main__":
    raise SystemExit(main())
