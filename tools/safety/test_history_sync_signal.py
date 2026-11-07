#!/usr/bin/env python3
"""Stall history persistence during quit or autosave, then deliver a signal."""

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


ROOT = Path(__file__).resolve().parents[2]
SHIM = ROOT / "tools/safety/native/history_sync_stall.c"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary: Path, library: Path, mode: str, stage: str, delivery: str,
            signal_group: bool, no_tmpfile: bool) -> dict:
    with tempfile.TemporaryDirectory(prefix="rless-history-sync-") as directory:
        root = Path(directory)
        source = root / "input"
        source.write_bytes(b"needle\n" * 80)
        trace = root / "sync-hit"
        master, slave = pty.openpty()
        original = termios.tcgetattr(slave)
        env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
               "LC_ALL": "C", "LESSHISTFILE": str(root / "historyX"),
               "LD_PRELOAD": str(library), "RLESS_HISTORY_SYNC_TRACE": str(trace),
               "RLESS_HISTORY_STALL_STAGE": stage}
        if no_tmpfile:
            env["RLESS_HISTORY_NO_TMPFILE"] = "1"
        argv = [str(binary)]
        if delivery == "int":
            argv.append("-K")
        if mode == "autosave":
            argv.extend(("--autosave=m", "--save-marks"))
        argv.append(str(source))
        pager = subprocess.Popen(
            argv, stdin=slave, stdout=slave, stderr=slave,
            start_new_session=True, close_fds=True, env=env,
        )
        os.close(slave)
        try:
            output = bytearray()
            deadline = time.monotonic() + 2
            while b"needle" not in output and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.05)
                if ready:
                    output.extend(os.read(master, 65536))
            if pager.poll() is None:
                if mode == "quit":
                    os.write(master, b"/needle\n")
                    time.sleep(0.15)
                    os.write(master, b"q")
                else:
                    os.write(master, b"ma")
            deadline = time.monotonic() + 3
            while not trace.exists() and pager.poll() is None and time.monotonic() < deadline:
                ready, _, _ = select.select([master], [], [], 0.01)
                if ready:
                    output.extend(os.read(master, 65536))
            if trace.exists() and pager.poll() is None:
                delivered = {"int": signal.SIGINT, "term": signal.SIGTERM,
                             "winch": signal.SIGWINCH}[delivery]
                if signal_group:
                    os.killpg(pager.pid, delivered)
                else:
                    os.kill(pager.pid, delivered)
                try:
                    pager.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
            alive_after_winch = None
            if delivery == "winch":
                alive_after_winch = pager.poll() is None
                if alive_after_winch:
                    os.write(master, b"q")
                    try:
                        pager.wait(timeout=2)
                    except subprocess.TimeoutExpired:
                        pass
            return {
                "fsync_hit": trace.read_bytes() == b"hit\n" if trace.exists() else False,
                "status_after_signal": pager.poll(),
                "alive_after_winch": alive_after_winch,
                "wchan": (Path(f"/proc/{pager.pid}/wchan").read_text().strip()
                          if pager.poll() is None else None),
                "terminal_restored": termios.tcgetattr(master) == original,
                "temporary_names": sorted(path.name for path in root.iterdir()
                                          if path.name.startswith(".historyX.")),
                "history_committed": (root / "historyX").exists() and
                                     (root / "historyX").read_bytes().startswith(b".less-history-file:\n"),
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
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--mode", choices=("quit", "autosave"), default="quit")
    parser.add_argument("--stage", choices=("fsync", "created", "linked", "directory"), default="fsync")
    parser.add_argument("--signal", choices=("term", "int", "winch"), default="term")
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--signal-group", action="store_true")
    parser.add_argument("--no-tmpfile", action="store_true")
    parser.add_argument("--expect-blocked", action="store_true")
    parser.add_argument("--expect-restored-block", action="store_true")
    args = parser.parse_args()
    if args.expect_blocked and args.expect_restored_block:
        parser.error("choose one expected outcome")
    if args.rounds < 1:
        parser.error("--rounds must be positive")
    if args.signal == "winch" and args.mode != "autosave":
        parser.error("SIGWINCH control requires --mode autosave")
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="rless-history-shim-") as directory:
        library = Path(directory) / "history_sync_stall.so"
        subprocess.run(
            ["cc", "-shared", "-fPIC", "-Wall", "-Wextra", "-Werror",
             str(SHIM), "-o", str(library), "-ldl"],
            check=True, capture_output=True,
        )
        cases = [observe(binary, library, args.mode, args.stage, args.signal,
                         args.signal_group, args.no_tmpfile)
                 for _ in range(args.rounds)]
    def valid(case: dict) -> bool:
        if args.expect_blocked:
            return (case["fsync_hit"] and case["status_after_signal"] is None
                    and not case["terminal_restored"])
        if args.expect_restored_block:
            return (case["fsync_hit"] and case["status_after_signal"] is None
                    and case["terminal_restored"])
        if args.signal == "winch":
            return (case["fsync_hit"] and case["alive_after_winch"]
                    and case["status_after_signal"] == 0
                    and case["terminal_restored"] and not case["temporary_names"]
                    and case["history_committed"])
        return (case["fsync_hit"] and case["status_after_signal"] ==
                (2 if args.signal == "int" else 15)
                and case["terminal_restored"] and not case["temporary_names"]
                and case["history_committed"] == (args.stage == "directory"))
    passed_count = sum(valid(case) for case in cases)
    passed = passed_count == args.rounds
    result = {"binary_sha256": sha(binary), "shim_sha256": sha(SHIM),
              "mode": args.mode, "stage": args.stage, "signal": args.signal,
              "signal_group": args.signal_group, "no_tmpfile": args.no_tmpfile,
              "expect_blocked": args.expect_blocked,
              "expect_restored_block": args.expect_restored_block,
              "cases": cases, "passed_count": passed_count, "rounds": args.rounds,
              "passed": passed}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "cases"}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
