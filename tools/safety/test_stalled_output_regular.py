#!/usr/bin/env python3
"""Hold an inherited regular stdout write across SIGTERM."""

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time


SOURCE = Path(__file__).with_name("native") / "stall_output_write.c"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def proc_text(pid, name):
    try:
        return Path(f"/proc/{pid}/{name}").read_text().strip()
    except (FileNotFoundError, ProcessLookupError, PermissionError):
        return None


def observe(binary):
    with tempfile.TemporaryDirectory(prefix="rless-stall-regular-output-") as directory:
        root = Path(directory)
        shim = root / "shim.so"
        marker = root / "marker"
        release = root / "release"
        data = root / "data"
        data.write_bytes(b"line\n" * 1000)
        subprocess.run(
            ["cc", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC", "-O2",
             str(SOURCE), "-o", str(shim), "-ldl"],
            check=True, capture_output=True,
        )
        output_path = root / "output"
        pager = None
        with output_path.open("wb") as output:
            original_flags = fcntl.fcntl(output, fcntl.F_GETFL)
            identity = os.fstat(output.fileno())
            env = {"PATH": "/usr/bin:/bin", "HOME": directory, "TERM": "xterm-256color",
                   "LC_ALL": "C", "LESSHISTFILE": "-", "LD_PRELOAD": str(shim),
                   "RLESS_STALL_OUTPUT_KIND": "regular",
                   "RLESS_STALL_OUTPUT_DEV": str(identity.st_dev),
                   "RLESS_STALL_OUTPUT_INO": str(identity.st_ino),
                   "RLESS_STALL_OUTPUT_MARKER": str(marker),
                   "RLESS_STALL_OUTPUT_RELEASE": str(release)}
            try:
                pager = subprocess.Popen(
                    [str(binary), str(data)], stdin=subprocess.DEVNULL,
                    stdout=output, stderr=subprocess.PIPE, env=env,
                    preexec_fn=os.setsid, close_fds=True,
                )
                deadline = time.monotonic() + 4
                while not marker.exists() and pager.poll() is None and time.monotonic() < deadline:
                    time.sleep(0.01)
                owners = [int(line) for line in marker.read_text().splitlines()] if marker.exists() else []
                children = [int(pid) for pid in
                            (proc_text(pager.pid, f"task/{pager.pid}/children") or "").split()]
                if owners and pager.poll() is None:
                    os.kill(pager.pid, signal.SIGTERM)
                try:
                    pager.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
                before = pager.poll()
                result = {
                    "binary_sha256": sha(binary), "source_sha256": sha(SOURCE),
                    "shim_sha256": sha(shim), "pager_pid": pager.pid,
                    "marker_owners": owners, "children_at_trigger": children,
                    "status_before_release": before,
                    "wchan_before_release": proc_text(pager.pid, "wchan") if before is None else None,
                    "output_bytes_before_release": output_path.stat().st_size,
                    "output_flags_after_signal": fcntl.fcntl(output, fcntl.F_GETFL),
                    "original_output_flags": original_flags,
                }
                release.touch()
                try:
                    pager.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
                result["status_after_release"] = pager.poll()
                result["output_bytes_after_release"] = output_path.stat().st_size
                child_owners = [pid for pid in owners if pid != pager.pid]
                deadline = time.monotonic() + 1
                while any(Path(f"/proc/{pid}").exists() for pid in child_owners) and time.monotonic() < deadline:
                    time.sleep(0.01)
                result["child_owners_gone"] = all(
                    not Path(f"/proc/{pid}").exists() for pid in child_owners
                )
                return result
            finally:
                release.touch()
                if pager is not None:
                    try:
                        os.killpg(pager.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    pager.wait(timeout=2)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expect-blocked", action="store_true")
    args = parser.parse_args()
    report = observe(args.binary.resolve(strict=True))
    common = bool(report["marker_owners"]) and report["status_after_release"] == 15
    if args.expect_blocked:
        passed = (common and report["pager_pid"] in report["marker_owners"]
                  and report["status_before_release"] is None)
    else:
        passed = (common and report["pager_pid"] not in report["marker_owners"]
                  and any(pid in report["children_at_trigger"] for pid in report["marker_owners"])
                  and report["status_before_release"] == 15
                  and report["output_bytes_before_release"] == 0
                  and report["output_bytes_after_release"] == 0
                  and report["output_flags_after_signal"] == report["original_output_flags"]
                  and report["child_owners_gone"])
    report["expect_blocked"] = args.expect_blocked
    report["passed"] = passed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
