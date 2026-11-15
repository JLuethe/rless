#!/usr/bin/env python3
"""Execute retained-object mutation controls against an explicit fresh ELF pair."""
import argparse
from pathlib import Path
import unittest
import test_linked_object as controls


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',type=Path,required=True)
    parser.add_argument('--object',type=Path,required=True)
    parser.add_argument('--anchor',choices=('__wrap_read','__wrap_calloc'),required=True)
    args=parser.parse_args()
    controls.BINARY=args.binary.resolve(strict=True);controls.OBJECT=args.object.resolve(strict=True)
    controls.ANCHOR=args.anchor
    suite=unittest.defaultTestLoader.loadTestsFromTestCase(controls.LinkedObjectMutations)
    result=unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() and not result.skipped and result.testsRun==6 else 1


if __name__=='__main__':
    raise SystemExit(main())
