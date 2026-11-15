#!/usr/bin/env python3
"""Force a logfile hard link and verify its victim is never truncated."""

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
VICTIM = b"VICTIM\n"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path, expect_clobber: bool) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-log-hardlink-") as directory:
        root = Path(directory)
        victim = root / "victim"
        capture = root / "capture"
        victim.write_bytes(VICTIM)
        os.link(victim, capture)
        events = ([{"await": ["READY"]}] if expect_clobber else [
            {"await": ["Cannot write"]},
            {"send": "\n", "await": ["READY"]},
        ])
        events.extend([{"close_stream": True}, {"send": "q"}])
        environment = {
            "PATH": "/usr/bin:/bin", "HOME": directory,
            "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                environment[name] = os.environ[name]
        case = {
            "id": "LOG-HARDLINK", "mode": "pty",
            "stream_b64": base64.b64encode(PAYLOAD).decode(), "events": events,
        }
        result = run_pty(
            [str(binary), "-O", "capture", "-PsREADY", "-PmREADY", "-PMREADY", "-"],
            environment, root, case, lambda data: data,
        )
        contents = victim.read_bytes()
        same_inode = os.stat(victim).st_ino == os.stat(capture).st_ino
        expected = PAYLOAD if expect_clobber else VICTIM
        return {
            "status": result["status"],
            "terminal_restored": result["termios_restored"],
            "victim_sha256": sha(contents),
            "same_inode": same_inode,
            "link_count": os.stat(victim).st_nlink,
            "passed": (result["status"] == (0 if expect_clobber else 1)
                       and result["termios_restored"]
                       and contents == expected and same_inode
                       and os.stat(victim).st_nlink == 2),
        }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-clobber", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    case = observe(binary, args.expect_clobber)
    report = {
        "binary_sha256": sha(binary.read_bytes()),
        "test_sha256": sha(Path(__file__).read_bytes()),
        "expect_clobber": args.expect_clobber,
        "case": case, "passed": case["passed"],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
