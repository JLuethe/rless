#!/usr/bin/env python3
"""Run every safety unit module with required executable/ELF fixtures supplied.

ActualFaultPaths and HookPaths are process campaigns, invoked by fault_paths.py
and run.py hooks; they require their supervisors and are not unit tests.
"""
import argparse
import importlib
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parents[2]
SUPERVISED={'test_fault_paths','test_hooks'}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inventory-binary',type=Path,required=True)
    parser.add_argument('--linked-binary',type=Path,required=True)
    parser.add_argument('--linked-object',type=Path,required=True)
    args=parser.parse_args()
    for path in (args.inventory_binary,args.linked_binary,args.linked_object):
        if not path.is_file(): parser.error('required unit fixture missing: '+str(path))
    inventory=importlib.import_module('test_inventory_parser')
    inventory.BINARY=args.inventory_binary.resolve(strict=True)
    linked=importlib.import_module('test_linked_object')
    linked.BINARY=args.linked_binary.resolve(strict=True)
    linked.OBJECT=args.linked_object.resolve(strict=True)
    modules=[p.stem for p in sorted((ROOT/'tools/safety').glob('test_*.py')) if p.stem not in SUPERVISED]
    suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromName(name) for name in modules)
    result=unittest.TextTestRunner(verbosity=2).run(suite)
    if result.skipped:
        print('Required unit fixture/control was skipped:',result.skipped,file=sys.stderr)
    return 0 if result.wasSuccessful() and not result.skipped else 1


if __name__=='__main__':
    raise SystemExit(main())
