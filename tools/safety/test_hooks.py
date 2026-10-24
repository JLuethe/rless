"""Actual pager allocation faults; no allocator model substitutes for these tests."""
import argparse
import os
from pathlib import Path
import struct
import subprocess
import unittest

NORMAL = None
INJECTED = None


def execute(binary, plan=None, nonblocking=True, fill=False):
    flags = os.O_CLOEXEC | (os.O_NONBLOCK if nonblocking else 0)
    read_fd, write_fd = os.pipe2(flags)
    if fill:
        try:
            while True:
                os.write(write_fd, b'x' * 4096)
        except BlockingIOError:
            pass
    env = dict(PATH='/usr/bin:/bin', HOME='/nonexistent', LC_ALL='C', TERM='dumb', LESSHISTFILE='-')
    if plan is not None:
        env.update(RLESS_FAULT=plan, RLESS_FAULT_FD=str(write_fd))
    try:
        result = subprocess.run([str(binary)], input=b'payload\n', capture_output=True,
                                env=env, pass_fds=[write_fd], timeout=5)
    finally:
        os.close(write_fd)
    chunks = []
    try:
        while True:
            value = os.read(read_fd, 65536)
            if not value:
                break
            chunks.append(value)
    finally:
        os.close(read_fd)
    return result, b''.join(chunks)


def require_fault(result, marker, ordinal):
    if result.returncode != 1:
        raise ValueError('wrong controlled-failure status')
    if marker != struct.pack('<4sIQ', b'RLF1', 1, ordinal):
        raise ValueError('missing, wrong-site or wrong-ordinal hit evidence')
    if b'Cannot allocate memory' not in result.stdout + result.stderr:
        raise ValueError('missing allocation failure diagnostic')
    if b'ERROR: AddressSanitizer' in result.stderr or b'ERROR: LeakSanitizer' in result.stderr:
        raise ValueError('sanitizer finding hidden behind expected error')


class ActualAllocationHooks(unittest.TestCase):
    def test_normal_build_contains_no_configuration_or_hook_strings(self):
        self.assertNotIn(b'RLESS_FAULT', NORMAL.read_bytes())
        result, marker = execute(NORMAL, 'unknown:0')
        self.assertEqual((result.returncode,result.stdout,result.stderr,marker), (0,b'payload\n',b'',b''))

    def test_feature_build_without_plan_preserves_output(self):
        result, marker = execute(INJECTED)
        self.assertEqual((result.returncode,result.stdout,result.stderr,marker), (0,b'payload\n',b'',b''))

    def test_actual_first_and_second_allocation_fail_at_recorded_ordinals(self):
        for ordinal in (1,2):
            result, marker = execute(INJECTED, f'alloc:{ordinal}')
            require_fault(result, marker, ordinal)

    def test_unreached_ordinal_is_not_a_fault_pass(self):
        result, marker = execute(INJECTED, 'alloc:18446744073709551615')
        self.assertEqual((result.returncode,result.stdout,marker),(0,b'payload\n',b''))
        with self.assertRaises(ValueError):
            require_fault(result,marker,1)

    def test_wrong_status_site_ordinal_and_diagnostic_controls(self):
        result, marker = execute(INJECTED, 'alloc:1')
        require_fault(result,marker,1)
        for wrong in [b'', struct.pack('<4sIQ',b'RLF1',2,1),struct.pack('<4sIQ',b'RLF1',1,2)]:
            with self.assertRaises(ValueError):require_fault(result,wrong,1)
        for wrong in [subprocess.CompletedProcess([],0,result.stdout,result.stderr),
                      subprocess.CompletedProcess([],1,b'',b'wrong error')]:
            with self.assertRaises(ValueError):require_fault(wrong,marker,1)

    def test_invalid_plan_and_blocking_channel_fail_before_pager(self):
        for plan in ('alloc:0','alloc:-1','unknown:1','alloc:999999999999999999999999999999'):
            result, marker = execute(INJECTED,plan)
            self.assertEqual(result.returncode,2)
            self.assertIn(b'rless safety injection:',result.stderr)
            self.assertEqual(marker,b'')
        result, marker = execute(INJECTED,'alloc:1',nonblocking=False)
        self.assertEqual(result.returncode,2)
        self.assertIn(b'nonblocking',result.stderr)
        self.assertEqual(marker,b'')

    def test_full_trace_pipe_cannot_block_or_satisfy_hit_evidence(self):
        result, marker = execute(INJECTED,'alloc:1',fill=True)
        self.assertEqual(result.returncode,1)
        with self.assertRaises(ValueError):require_fault(result,marker,1)


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--normal',type=Path,required=True)
    parser.add_argument('--injected',type=Path,required=True)
    args=parser.parse_args()
    NORMAL=args.normal.resolve(strict=True);INJECTED=args.injected.resolve(strict=True)
    unittest.main(argv=['test_hooks'],verbosity=2)
