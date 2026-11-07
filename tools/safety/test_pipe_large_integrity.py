#!/usr/bin/env python3
"""Compare multi-megabyte range-pipe bytes with the pinned less reference."""

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


PAYLOAD = b"line 0123456789abcdef\n" * 100000


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def observe(binary: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-large-pipe-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(PAYLOAD)
        capture = root / "capture"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
               "LC_ALL": "C", "LESSHISTFILE": "-"}
        pager = subprocess.Popen(
            [str(binary), str(source)], stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        try:
            time.sleep(0.2)
            os.write(master, b"Gmag|acat > " + os.fsencode(capture) + b"\n")
            output = bytearray()
            deadline = time.monotonic() + 8
            while b"press RETURN" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    try:
                        output.extend(os.read(master, 65536))
                    except OSError:
                        break
            prompt = b"press RETURN" in output
            if pager.poll() is None:
                os.write(master, b"\nq")
                try:
                    pager.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    pass
            content = capture.read_bytes() if capture.exists() else b""
            return {
                "binary_sha256": sha(binary.read_bytes()),
                "prompt": prompt,
                "status": pager.poll(),
                "terminal_restored": termios.tcgetattr(master) == original,
                "capture_bytes": len(content),
                "capture_sha256": sha(content),
                "source_prefix": PAYLOAD.startswith(content),
            }
        finally:
            try:
                os.killpg(pager.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pager.wait(timeout=2)
            os.close(master)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--reference", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    candidate = observe(args.binary.resolve(strict=True))
    reference = observe(args.reference.resolve(strict=True))
    common = lambda case: (case["prompt"] and case["status"] == 0
                           and case["terminal_restored"] and case["source_prefix"]
                           and case["capture_bytes"] > 2_000_000)
    passed = (common(candidate) and common(reference)
              and candidate["capture_bytes"] == reference["capture_bytes"]
              and candidate["capture_sha256"] == reference["capture_sha256"])
    result = {"payload_sha256": sha(PAYLOAD), "candidate": candidate,
              "reference": reference, "passed": passed}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
