#!/usr/bin/env python3
"""Swap a logfile name at its prompt and verify descriptor-bound writes."""

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

PAYLOAD = b"line\n" * 100
OLD_LOG = b"OLD\n"
VICTIM = b"VICTIM\n"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, answer: str, swap_kind: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-log-race-") as directory:
        root = Path(directory)
        parent = root / "folder" if swap_kind == "directory" else root
        parent.mkdir(exist_ok=True)
        (parent / "capture").write_bytes(OLD_LOG)
        (root / "victim").write_bytes(VICTIM)
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                environment[name] = os.environ[name]
        case = {
            "id": "LOG-RACE-" + swap_kind + "-" + answer,
            "mode": "pty",
            "stream_b64": base64.b64encode(PAYLOAD).decode(),
            "events": [
                {"await": ["exists"], "heap_checkpoint": "swap"},
                {"send": answer + "\n", "await": ["READY"]},
                {"close_stream": True},
                {"send": "q"},
            ],
        }
        swap_hits = 0

        def swap(_pid: int, label: str) -> None:
            nonlocal swap_hits
            if label != "swap":
                raise ValueError("wrong race barrier")
            swap_hits += 1
            if swap_kind == "directory":
                parent.rename(root / "displaced")
                parent.mkdir()
                (parent / "capture").symlink_to("../victim")
            else:
                (parent / "capture").rename(root / "displaced")
                (parent / "capture").symlink_to("victim")

        result = run_pty(
            [str(binary), "-o", "folder/capture" if swap_kind == "directory"
             else "capture", "-PsREADY", "-PmREADY", "-PMREADY", "-"],
            environment, root, case, lambda data: data, checkpoint_observer=swap,
        )
        victim = (root / "victim").read_bytes()
        displaced = (root / "displaced" / "capture" if swap_kind == "directory"
                     else root / "displaced").read_bytes()
        expected = PAYLOAD if answer == "O" else OLD_LOG + PAYLOAD
        return {
            "answer": answer, "swap_kind": swap_kind, "swap_hits": swap_hits,
            "status": result["status"],
            "terminal_restored": result["termios_restored"],
            "victim_sha256": sha(victim),
            "displaced_sha256": sha(displaced),
            "expected_displaced_sha256": sha(expected),
            "capture_is_symlink": (parent / "capture").is_symlink(),
            "passed": (
                swap_hits == 1 and result["status"] == 0
                and result["termios_restored"]
                and victim == VICTIM and displaced == expected
                and (parent / "capture").is_symlink()
            ),
        }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    arguments = parser.parse_args()
    binary = arguments.binary.resolve(strict=True)
    observations = [observe(binary, answer, swap_kind)
                    for swap_kind in ("basename", "directory")
                    for answer in ("O", "A")]
    report = {
        "binary": str(binary), "binary_sha256": sha(binary.read_bytes()),
        "observer_sha256": sha((Path(__file__).resolve().parents[1] / "compat/run_v2.py").read_bytes()),
        "cases": observations,
        "passed": all(item["passed"] for item in observations),
    }
    output = arguments.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"passed": report["passed"], "cases": observations}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
