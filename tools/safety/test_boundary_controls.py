"""Synthetic adversarial controls; real retained campaigns run separately."""
import copy
import io
from pathlib import Path
import stat
import struct
import tempfile
import unittest
from unittest.mock import patch

import boundary_controls as boundary
from common import PINS, ROOT, sha
from live_heap import Samples, scenario
import m0_gate
from test_stdin_delete import validate as validate_stdin
from test_first_buffer import validate as validate_first


class SupplementalControls(unittest.TestCase):
    def test_safety_runner_api_is_not_compat_run_module(self):
        import safety_runner
        self.assertEqual(Path(safety_runner.source_file_summaries.__code__.co_filename).resolve(),
                         (ROOT/'tools/safety/run.py').resolve())

    def test_every_boundary_control_is_required_on_both_abis(self):
        obligations=m0_gate.obligations()
        for name,(_,profile) in boundary.ABI_CONTROLS.items():
            for abi in m0_gate.REQUIRED_ABIS:
                item=obligations[name+':'+abi]
                self.assertEqual((item['control'],item['profile'],item['abi_profile'],item['site']),(name,profile,abi,None))

    def test_status_only_or_unknown_receipt_gets_no_credit(self):
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'receipt.json'
            for text in ('{}','{"passed":true}','{"report_kind":"boundary-controls","procedure":"copied","schema_version":1}'):
                path.write_text(text)
                with self.assertRaises(ValueError): boundary.validate_external_report(path)

    def test_mutation_targets_fail_when_source_shape_changes(self):
        for kind,path in boundary.MUTANT_PATH.items():
            original=(ROOT/path).read_text()
            changed=boundary.mutant_text(kind,original)
            self.assertNotEqual(original,changed)
            if kind in ('stdin-leak','stdin-close','heap-leak'):
                with self.assertRaises(ValueError): boundary.mutant_text(kind,'no mutation target')

    def test_first_buffer_requires_actual_allocator_object_not_symbol_claim(self):
        with patch('test_first_buffer.verify',side_effect=ValueError('wrong linked allocator')) as verify:
            with self.assertRaisesRegex(ValueError,'linked allocator'):
                validate_first({'oracle':{'passed':True}},Path('/result'),Path('/binary'),Path('/object'))
            verify.assert_called_once_with(Path('/binary'),Path('/object'),'__wrap_calloc')

    def test_stdin_witness_rejects_closed_or_replaced_borrowed_pipe(self):
        from test_stdin_delete import sha as digest
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);binary=out/'binary';binary.write_bytes(b'synthetic')
            (out/'positive.trace').write_bytes(b'ready\nclose 0\n')
            identity=dict(target='pipe:[123]',dev=1,ino=123,mode=stat.S_IFIFO|0o600)
            result=dict(binary_sha256=sha(binary),positive_status=0,cases={})
            for name in ('retain','delete'):
                (out/(name+'.raw')).write_bytes(b'')
                (out/(name+'.close-trace')).write_bytes(b'ready\n')
                result['cases'][name]=dict(status=0,termios_restored=True,screens=['stdin','file'],
                    raw_sha256=digest(b''),close_trace_sha256=digest(b'ready\n'),
                    fd0_checkpoints=[dict(identity,label='before'),dict(identity,label='after')])
            self.assertTrue(validate_stdin(result,out,binary))
            changed=copy.deepcopy(result);changed['cases']['delete']['fd0_checkpoints'][1]['ino']=999
            with self.assertRaisesRegex(ValueError,'identity changed'): validate_stdin(changed,out,binary)
            (out/'delete.close-trace').write_bytes(b'ready\nclose 0\n')
            result['cases']['delete']['close_trace_sha256']=digest(b'ready\nclose 0\n')
            with self.assertRaisesRegex(ValueError,'was closed'): validate_stdin(result,out,binary)

    def heap_fixture(self,out,delta):
        binary=out/'binary';binary.write_bytes(b'synthetic binary')
        obj=out/'object';obj.write_bytes(b'synthetic object')
        raw=b''.join(struct.pack('<4sIQQQQQ',b'RHP1',1,index,4096+(delta if index>1 else 0),4,271,0x1234) for index in range(1,10))
        (out/'heap-trace.bin').write_bytes(raw)
        samples=Samples(io.BytesIO(raw));samples.drain()
        checkpoints=[dict(x,label='warmup' if index==0 else f'cycle-{index+2}',blocked_syscall=['271','0x1234','1'])
                     for index,x in enumerate(samples.records)]
        case=scenario('filter-before',10)
        case['env']=dict(ASAN_OPTIONS=PINS['asan_options']+':symbolize=0:log_path='+str(out/'sanitizer/raw'),LSAN_OPTIONS=PINS['lsan_options'])
        from test_fault_paths import marker
        result=dict(binary_sha256=sha(binary),shim_sha256=sha(obj),heap_trace_sha256=sha(out/'heap-trace.bin'),
            helper_sha256=sha(ROOT/'tools/safety/live_heap.py'),shim_source_sha256=sha(ROOT/'tools/safety/native/live_heap.c'),
            observer_sha256=sha(ROOT/'tools/compat/run_v2.py'),samples=samples.records,checkpoints=checkpoints,
            site='filter-before',ordinal=3,cycles=10,case=case,fault_trace_hex=marker(7,3).hex(),
            observation=dict(status=0,termios_restored=True),passed=False,heap_balance_passed=False)
        return result,binary,obj

    def test_cleanup_mutant_rejection_is_independent_of_lsan_exit(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);result,binary,obj=self.heap_fixture(out,845)
            for status in (0,87):
                result['observation']['status']=status
                self.assertEqual(boundary.validate_heap(result,out,binary,obj,mutant=True),[845]*8)

    def test_false_heap_failure_bit_without_live_growth_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);result,binary,obj=self.heap_fixture(out,0)
            result['observation']['status']=87
            with self.assertRaisesRegex(ValueError,'retained live bytes'):
                boundary.validate_heap(result,out,binary,obj,mutant=True)

    def test_heap_checkpoint_cannot_replace_raw_allocated_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);result,binary,obj=self.heap_fixture(out,845)
            result['checkpoints'][1]['allocated_bytes']=4096
            with self.assertRaisesRegex(ValueError,'differs from raw'):
                boundary.validate_heap(result,out,binary,obj,mutant=True)


if __name__=='__main__':
    unittest.main()
