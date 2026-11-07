#!/usr/bin/env python3
"""Force a partial sigaction installation failure and inspect rollback."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile


SOURCE = Path(__file__).with_name("native") / "signal_install_failure.c"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--signal", choices=("SIGINT", "SIGWINCH", "SIGUSR1"),
                        default="SIGWINCH")
    parser.add_argument("--inherit-ignored-winch", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shim = args.output.parent / (args.output.stem + ".so")
    subprocess.run(
        ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
         str(SOURCE), "-ldl", "-o", str(shim)], check=True,
    )
    with tempfile.TemporaryDirectory(prefix="rless-signal-install-") as directory:
        input_path = Path(directory) / "input"
        input_path.write_bytes(b"input must not be processed\n")
        reader, writer = os.pipe2(os.O_CLOEXEC)
        try:
            env = {
                "PATH": "/usr/bin:/bin", "HOME": directory,
                "TERM": "xterm-256color", "LC_ALL": "C", "LESSHISTFILE": "-",
                "LD_PRELOAD": str(shim.resolve()),
                "RLESS_SIGNAL_INSTALL_TRACE_FD": str(writer),
                "RLESS_SIGNAL_INSTALL_FAIL_SIGNAL": str(getattr(signal, args.signal)),
            }
            if args.inherit_ignored_winch:
                env["RLESS_SIGNAL_INSTALL_EXPECT_IGNORED"] = "1"
            process = subprocess.run(
                [str(binary), str(input_path)], stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env,
                pass_fds=(writer,), timeout=5,
                preexec_fn=(lambda: signal.signal(signal.SIGWINCH, signal.SIG_IGN))
                if args.inherit_ignored_winch else None,
            )
            os.close(writer)
            writer = -1
            trace = os.read(reader, 4096).decode("ascii", errors="replace")
        finally:
            os.close(reader)
            if writer >= 0:
                os.close(writer)
    report = {
        "binary_sha256": digest(binary), "test_sha256": digest(Path(__file__)),
        "shim_source_sha256": digest(SOURCE), "shim_sha256": digest(shim),
        "status": process.returncode, "trace": trace,
        "failed_signal": args.signal,
        "inherited_ignored_winch": args.inherit_ignored_winch,
        "stdout_sha256": hashlib.sha256(process.stdout).hexdigest(),
        "stdout_len": len(process.stdout),
        "stderr_tail": process.stderr[-200:].decode("latin1"),
    }
    expected_trace = ("inherited-winch-ignored\n" if args.inherit_ignored_winch else "")
    expected_trace += "hit\nactions-restored\nmask-restored\n"
    report["passed"] = (
        process.returncode == 1 and process.stdout == b"" and
        trace == expected_trace
    )
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
