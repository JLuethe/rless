#!/usr/bin/env python3
"""Check that hostile LESSMSG conversions cannot read native varargs."""

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
    with tempfile.TemporaryDirectory(prefix="rless-lessmsg-format-") as directory:
        root = Path(directory)
        message = root / "messages"
        env = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LC_ALL": "C", "LESSHISTFILE": "-", "LESSMSG": str(message),
            "LESSKEY_CONTENT": "x definitely-not-an-action",
        }
        for name in ("ASAN_OPTIONS", "LSAN_OPTIONS", "ASAN_SYMBOLIZER_PATH"):
            if name in os.environ:
                env[name] = os.environ[name]
        results = []
        for variant in (
            b"%p", b"%n", b"%s %s", b"%1$s", b"%*s",
            b"%999999999s", b"%s %n", b"%s %p",
        ):
            message.write_bytes(b"unknown_action_X " + variant + b"\n")
            process = subprocess.run(
                [str(binary), "--", "missing"], cwd=root, env=env,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=3,
                check=False,
            )
            output = process.stdout + process.stderr
            results.append({
                "variant": variant.decode("ascii"),
                "status": process.returncode,
                "action_reached": b"lesskey-content: line 1:" in output,
                "format_rejected": b"invalid message format placeholders" in output,
                "native_pointer_disclosed": bool(re.search(rb"0x[0-9a-fA-F]{6,}", output)),
                "output_sha256": hashlib.sha256(output).hexdigest(),
            })
        report = {
            "binary": str(binary),
            "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
            "results": results,
        }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if all(
        result["status"] == 1 and result["action_reached"]
        and result["format_rejected"] and not result["native_pointer_disclosed"]
        for result in report["results"]
    ) else 1


if __name__ == "__main__":
    raise SystemExit(main())
