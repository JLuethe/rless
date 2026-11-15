#!/usr/bin/env python3
"""Verify secure history saving when O_TMPFILE is unavailable."""

import argparse
import hashlib
import json
from pathlib import Path
import stat
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "compat"))
from run_v2 import run_pty

SHIM = Path(__file__).with_name("native") / "history_sync_stall.c"


def observe(binary: Path, library: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-history-no-tmpfile-") as directory:
        root = Path(directory)
        (root / "input").write_bytes(b"needle\n" * 80)
        history = root / "history"
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": str(history), "LD_PRELOAD": str(library),
            "RLESS_HISTORY_NO_TMPFILE": "1",
        }
        scenario = {
            "id": "HISTORY-NO-TMPFILE", "mode": "pty",
            "events": [
                {"await": ["READY"]},
                {"send": "/needle\n", "await": ["READY"]},
                {"send": "q"},
            ],
        }
        result = run_pty(
            [str(binary), "-PsREADY", "-PmREADY", "-PMREADY", "input"],
            environment, root, scenario, lambda data: data,
        )
        contents = history.read_bytes() if history.exists() else b""
        return {
            "status": result["status"],
            "terminal_restored": result["termios_restored"],
            "history_has_search": b'"needle\n' in contents,
            "history_mode": stat.S_IMODE(history.stat().st_mode) if history.exists() else None,
            "temporary_names": sorted(path.name for path in root.iterdir()
                                      if path.name.startswith(".history.")),
        }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-failure", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-history-no-tmpfile-shim-") as directory:
        library = Path(directory) / "history_sync_stall.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(library), "-ldl"], check=True,
        )
        case = observe(binary, library)
    case["expected_outcome_observed"] = (
        case["terminal_restored"]
        and not case["temporary_names"]
        and (case["status"], case["history_has_search"])
        == ((1, False) if args.expect_failure else (0, True))
        and (args.expect_failure or case["history_mode"] == 0o600)
    )
    report = {
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "shim_sha256": hashlib.sha256(SHIM.read_bytes()).hexdigest(),
        "expect_failure": args.expect_failure,
        "case": case,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if case["expected_outcome_observed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
