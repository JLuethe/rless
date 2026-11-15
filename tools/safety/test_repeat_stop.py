#!/usr/bin/env python3
"""Check that two stop notifications before suspension cause one stop."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from test_suspend import observe  # noqa: E402

SHIM_SOURCE = Path(__file__).parent / 'native/repeat_stop.c'


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--pre-fix', type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--rounds', type=int, default=1)
    args = parser.parse_args()
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='rless-repeat-stop-shim-') as directory:
        shim = Path(directory)/'repeat_stop.so'
        subprocess.run(['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
                        str(SHIM_SOURCE), '-ldl', '-o', str(shim)], check=True)
        candidates = [observe(args.binary.resolve(), shim) for _ in range(args.rounds)]
        pre_fix = observe(args.pre_fix.resolve(), shim) if args.pre_fix else None
        passed = all(case['fault_hit'] and not case['second_stop']
                     and case['exited']['status'] == 0 for case in candidates)
        if pre_fix is not None:
            passed &= pre_fix['fault_hit'] and pre_fix['second_stop']
        report = dict(passed=passed, rounds=args.rounds,
                      candidate_binary_sha256=sha(args.binary.read_bytes()),
                      pre_fix_binary_sha256=sha(args.pre_fix.read_bytes()) if args.pre_fix else None,
                      shim_source_sha256=sha(SHIM_SOURCE.read_bytes()),
                      shim_sha256=sha(shim.read_bytes()),
                      candidate=candidates, pre_fix=pre_fix)
        output.write_text(json.dumps(report, indent=2)+'\n')
        successful = sum(case['fault_hit'] and not case['second_stop']
                         and case['exited']['status'] == 0 for case in candidates)
        print(f"Repeated stop: candidate={successful}/{len(candidates)}; pre-fix={pre_fix and pre_fix['fault_hit'] and pre_fix['second_stop']}")
        return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
