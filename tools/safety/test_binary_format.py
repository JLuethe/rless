#!/usr/bin/env python3
"""Exercise hostile and valid binary-character formats through the real pager."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import signal
import subprocess
import tempfile
import termios
import time


def exercise(binary, source, env, timeout=3.0):
    master, slave = pty.openpty()
    original = termios.tcgetattr(slave)
    pager = subprocess.Popen(
        [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
        start_new_session=True, close_fds=True, env=env,
    )
    os.close(slave)
    output = bytearray()
    prompted = False
    accepted = False
    quit_sent = False
    start = time.monotonic()
    try:
        while time.monotonic() - start < timeout:
            ready, _, _ = select.select([master], [], [], 0.05)
            data = b""
            if ready:
                try:
                    data = os.read(master, 65536)
                except OSError:
                    data = b""
                output.extend(data)
            if not accepted and b"binary file" in output.lower():
                prompted = True
                os.write(master, b"y\n")
                accepted = True
            if accepted and not quit_sent and time.monotonic() - start > 0.6:
                if pager.poll() is None:
                    try:
                        os.write(master, b"q")
                    except OSError:
                        pass
                quit_sent = True
            if pager.poll() is not None and (not ready or not data):
                break
        status = pager.poll()
        restored = termios.tcgetattr(master) == original
        return {
            "status": status, "binary_prompt": prompted, "accepted": accepted,
            "terminal_restored": restored,
            "output_sha256": hashlib.sha256(output).hexdigest(),
            "has_default_byte": b"<80>" in output,
            "has_valid_hex": b"[0x80]" in output,
            "has_default_unicode": b"<U+0080>" in output,
            "has_pointer_text": b"0x80" in output,
        }
    finally:
        try:
            os.killpg(pager.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        pager.wait(timeout=2)
        os.close(master)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--expect-vulnerable", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-binary-format-") as directory:
        root = Path(directory)
        raw = root / "raw"
        raw.write_bytes(b"\x80\n" * 80)
        unicode = root / "unicode"
        unicode.write_bytes(b"\xc2\x80\n" * 80)
        base = {
            "PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
            "LESSHISTFILE": "-", "LC_ALL": "C",
        }
        variants = (
            ("byte_string", raw, {"LESSBINFMT": "%s"}),
            ("byte_pointer", raw, {"LESSBINFMT": "%p"}),
            ("byte_numeric", raw, {"LESSBINFMT": "[%#04x]"}),
            ("unicode_string", unicode, {"LESSUTFBINFMT": "%s", "LESSCHARSET": "utf-8"}),
        )
        results = {}
        for name, source, extra in variants:
            results[name] = exercise(binary, source, base | extra)
    report = {
        "binary": str(binary),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "expect_vulnerable": args.expect_vulnerable,
        "results": results,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    if args.expect_vulnerable:
        passed = (results["byte_string"]["status"] == -signal.SIGSEGV
                  and results["byte_string"]["binary_prompt"]
                  and results["unicode_string"]["status"] == -signal.SIGSEGV
                  and results["unicode_string"]["binary_prompt"]
                  and results["byte_pointer"]["has_pointer_text"]
                  and results["byte_numeric"]["status"] == 0
                  and results["byte_numeric"]["has_valid_hex"])
    else:
        passed = (all(result["status"] == 0 and result["terminal_restored"]
                      and result["binary_prompt"] for result in results.values())
                  and results["byte_string"]["has_default_byte"]
                  and not results["byte_pointer"]["has_pointer_text"]
                  and results["byte_numeric"]["has_valid_hex"]
                  and results["unicode_string"]["has_default_unicode"])
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
