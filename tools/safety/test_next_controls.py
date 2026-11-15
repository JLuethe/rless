"""Independent QA controls for the reported NEXT findings."""
import copy
import hashlib
import json
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch

from common import sha, validate_fault_supplement
from mutation_provenance import validate as validate_binary
from test_first_buffer import observation, validate_expected_failure as first_failure
from test_stdin_delete import sha as digest, validate_expected_failure as stdin_failure
import runtime_identity


class NextControls(unittest.TestCase):
    def first_fixture(self,out):
        binary=out/'binary';binary.write_bytes(b'synthetic binary')
        obj=out/'object';obj.write_bytes(b'synthetic object')
        result=dict(binary_sha256=sha(binary),wrapper_sha256=sha(obj),allocator_proof={},
                    ch_addbuf=dict(start='0x100',end='0x200',size=256),cases={})
        for name in ('control','miss','hit','reuse'):
            data=b'' if name=='hit' else b'line\n'*(4000 if name=='reuse' else 20)
            error=b'thread main panicked at src/ch.rs:597:13:\nnull pointer dereference occurred\n' if name=='hit' else b''
            trace=b'ch_addbuf calloc(1,8240) caller_offset=150\n' if name in ('hit','reuse') else b''
            (out/(name+'.stdout')).write_bytes(data);(out/(name+'.stderr')).write_bytes(error);(out/(name+'.trace')).write_bytes(trace)
            result['cases'][name]=observation(out,name,-6 if name=='hit' else 0)
        return result,binary,obj

    def test_first_mutant_requires_specific_crash_not_forced_passing_statuses(self):
        with tempfile.TemporaryDirectory() as directory, patch('test_first_buffer.bounds',return_value=(0x100,0x200)), patch('test_first_buffer.verify',return_value={}):
            out=Path(directory);result,binary,obj=self.first_fixture(out)
            self.assertEqual(first_failure(result,out,binary,obj)['reason'],'first-buffer-empty-list-sigabrt')
            result['cases']['hit']['status']=0
            with self.assertRaisesRegex(ValueError,'intended null-pointer SIGABRT'):first_failure(result,out,binary,obj)

    def test_first_mutant_rejects_unrelated_panic_even_after_rehash(self):
        with tempfile.TemporaryDirectory() as directory, patch('test_first_buffer.bounds',return_value=(0x100,0x200)), patch('test_first_buffer.verify',return_value={}):
            out=Path(directory);result,binary,obj=self.first_fixture(out)
            (out/'hit.stderr').write_bytes(b'panicked at src/other.rs:1:1: invalid test data')
            result['cases']['hit']=observation(out,'hit',-6)
            with self.assertRaisesRegex(ValueError,'intended null-pointer SIGABRT'):first_failure(result,out,binary,obj)

    def test_mutant_binary_must_match_independent_source_build(self):
        with tempfile.TemporaryDirectory() as directory:
            binary=Path(directory)/'fixed';binary.write_bytes(b'fixed binary')
            with patch('mutation_provenance.rederive',return_value='0'*64):
                with self.assertRaisesRegex(ValueError,'independently rebuilt exact mutated source'):
                    validate_binary({'kind':'stdin-close'},'NATIVE',{},binary)

    def stdin_fixture(self,out):
        binary=out/'binary';binary.write_bytes(b'synthetic mutant')
        positive=b'ready\nclose 0\n';(out/'positive.trace').write_bytes(positive)
        identity=dict(target='pipe:[123]',dev=1,ino=123,mode=stat.S_IFIFO|0o600)
        result=dict(binary_sha256=sha(binary),positive_status=0,positive_trace_sha256=digest(positive),cases={})
        for name in ('retain','delete'):
            trace=b'ready\n'+(b'close 0\n' if name=='delete' else b'')
            (out/(name+'.raw')).write_bytes(b'');(out/(name+'.close-trace')).write_bytes(trace)
            result['cases'][name]=dict(status=0,termios_restored=True,screens=['stdin','file'],raw_sha256=digest(b''),close_trace_sha256=digest(trace),
                fd0_checkpoints=[dict(identity,label='before'),dict(label='after',target='missing',dev=0,ino=0,mode=0) if name=='delete' else dict(identity,label='after')])
        return result,binary

    def test_stdin_mutant_identity_error_cannot_count_as_expected_close(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);result,binary=self.stdin_fixture(out)
            self.assertEqual(stdin_failure(result,out,binary,'stdin-close')['reason'],'stdin-close')
            binary.write_bytes(b'fixed replacement')
            with self.assertRaisesRegex(ValueError,'changed stdin regression binary'):stdin_failure(result,out,binary,'stdin-close')

    def test_stdin_close_requires_missing_fd_at_the_input_barrier(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);result,binary=self.stdin_fixture(out)
            result['cases']['delete']['fd0_checkpoints'][1]=dict(result['cases']['retain']['fd0_checkpoints'][1])
            with self.assertRaisesRegex(ValueError,'fd0 absent'):stdin_failure(result,out,binary,'stdin-close')

    def test_fault_summary_cannot_omit_or_null_its_supplement(self):
        for report in ({'report_kind':'fault-campaign','records':[]},
                       {'native_allocation_binding':{},'records':[]},
                       {'records':[{'oracle_metadata':{'test_selector':'ActualFaultPaths.case'}}]},
                       {'report_kind':'fault-campaign','records':[{'name':'supplemental-boundary-controls','boundary_controls_report':None}]}):
            with self.subTest(report=report),self.assertRaisesRegex(ValueError,'exactly one retained supplemental'):
                validate_fault_supplement(report)

    def test_reference_context_uses_the_actual_runtime_lock_schema(self):
        from common import ROOT
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory)
            (root/'docs/verification').mkdir(parents=True)
            (root/'tools/safety/glibc241').mkdir(parents=True)
            containerfile=root/'tools/safety/glibc241/Containerfile';containerfile.write_text('Synthetic image procedure fixture.')
            provider=root/'libc.so.6';provider.write_bytes(b'Synthetic libc fixture.')
            lock=json.loads((ROOT/'docs/verification/glibc241-environment.json').read_text())
            self.assertIn('runtime',lock);self.assertNotIn('runtime.txt',lock)
            lock['containerfile_sha256']=sha(containerfile)
            path=root/'docs/verification/glibc241-environment.json';path.write_text(json.dumps(lock))
            runtime=Path('/opt/rless-abi/runtime.txt')
            real_read=Path.read_text;real_file=Path.is_file;real_bytes=Path.read_bytes
            def read(source,*args,**kwargs):
                return 'glibc 2.41\n' if source==runtime else real_read(source,*args,**kwargs)
            def is_file(source):return True if source==runtime else real_file(source)
            def read_bytes(source):return b'glibc 2.41\n' if source==runtime else real_bytes(source)
            with patch.object(Path,'read_text',read),patch.object(Path,'is_file',is_file),patch.object(Path,'read_bytes',read_bytes), \
                    patch('runtime_identity.libc_mappings',return_value=[{'path':str(provider)}]), \
                    patch('runtime_identity.libc_version',return_value={'version':'2.41'}):
                result=runtime_identity.reference_context(root)
                self.assertEqual(result['image_id'],lock['image_id'])
                self.assertEqual(result['libc_sha256'],sha(provider))
                self.assertEqual(result['environment_lock_sha256'],sha(path))

    def test_runtime_abi_comes_from_actual_libc_elf_not_log_label(self):
        with tempfile.TemporaryDirectory() as directory:
            out=Path(directory);binary=out/'binary';binary.write_bytes(b'synthetic ELF')
            (out/'maps.raw').write_text('100-200 r-xp 0 00:01 123 /usr/lib/libc.so.6\n')
            (out/'runtime-libc.so.6').write_bytes(b'libc fixture')
            proof={'version':'2.44'}
            result=dict(binary_sha256=sha(binary),maps_sha256=sha(out/'maps.raw'),libc_sha256=sha(out/'runtime-libc.so.6'),
                        mappings=runtime_identity.libc_mappings((out/'maps.raw').read_text()),libc_version_proof=proof)
            with patch('runtime_identity.libc_version',return_value=proof):
                with self.assertRaisesRegex(ValueError,'ABI label differs from mapped libc'):
                    runtime_identity.validate(result,out,binary,abi_profile='REFERENCE-GLIBC-2.41')


if __name__=='__main__':unittest.main()
