#!/usr/bin/env python3
"""Exercise every single-byte percent suffix in a LESSMSG replacement."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    records = []
    with tempfile.TemporaryDirectory(prefix="rless-lessmsg-bytes-") as directory:
        root = Path(directory)
        message = root / "messages"
        env = {"PATH": "/usr/bin:/bin", "HOME": directory,
               "TERM": "xterm-256color", "LC_ALL": "C",
               "LESSHISTFILE": "-", "LESSMSG": str(message),
               "LESSKEY_CONTENT": "x definitely-not-an-action"}
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                env[name] = os.environ[name]
        for suffix in range(256):
            replacement = b"unknown_action_X byte-%" + bytes([suffix]) + b"-end\n"
            message.write_bytes(replacement)
            try:
                process = subprocess.run(
                    [str(binary), "--", "missing"], cwd=root, env=env,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                    timeout=2, check=False,
                )
                output = process.stdout + process.stderr
                record = {"suffix": suffix, "status": process.returncode,
                          "output_sha256": hashlib.sha256(output).hexdigest(),
                          "pointer_disclosed": bool(re.search(rb"0x[0-9a-fA-F]{6,}", output)),
                          "action_reached": b"lesskey-content: line 1:" in output}
            except subprocess.TimeoutExpired:
                record = {"suffix": suffix, "timeout": True}
            records.append(record)
    passed = all(record.get("status") == 1
                 and record.get("action_reached")
                 and not record.get("pointer_disclosed") for record in records)
    report = {"binary": str(binary),
              "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "executed": len(records), "passed": passed,
              "failures": [record for record in records
                           if record.get("status") != 1
                           or not record.get("action_reached")
                           or record.get("pointer_disclosed")],
              "records": records}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items()
                      if key != "records"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
