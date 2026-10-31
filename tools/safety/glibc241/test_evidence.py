"""Retained reference-report selection/path controls; run by independent tester."""
import json
from pathlib import Path
import sys
import tempfile
import types
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from glibc241.evidence import checked, fault_records, validate_external_report, work_path
from common import ROOT, sha
from release import reference_policy, validate_reference_hashes
from native_alloc import ElfTargets


class EvidenceControls(unittest.TestCase):
    def test_reference_qualification_rejects_self_consistent_alternate_bundle(self):
        hashes = reference_policy(ROOT)['files_sha256']
        validate_reference_hashes(hashes, ROOT)
        for name in hashes:
            with self.subTest(name=name):
                changed = dict(hashes, **{name: 'a' * 64})
                with self.assertRaisesRegex(ValueError, 'maintained qualification pins'):
                    validate_reference_hashes(changed, ROOT)
        for changed in ({}, {**hashes, 'unreviewed': 'a' * 64}):
            with self.assertRaisesRegex(ValueError, 'maintained qualification pins'):
                validate_reference_hashes(changed, ROOT)


    def test_unsupported_report_cannot_claim_native_pass(self):
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'summary.json'
            path.write_text(json.dumps(dict(status='passed',records=[])))
            with self.assertRaisesRegex(ValueError,'unsupported'):
                validate_external_report(path)

    def test_missing_changed_linked_and_escaping_artifacts_fail(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory)
            artifact=root/'file'
            artifact.write_text('retained')
            digest=sha(artifact)
            self.assertEqual(checked(artifact,digest),artifact)
            for path in ('/etc/passwd','/work/../../etc/passwd'):
                with self.assertRaisesRegex(ValueError,'escaping'):
                    work_path(path,root)
            link=root/'link'
            link.symlink_to(artifact)
            with self.assertRaisesRegex(ValueError,'altered'):
                checked(link,digest)
            artifact.write_text('changed')
            with self.assertRaisesRegex(ValueError,'altered'):
                checked(artifact,digest)

    def test_finding_output_cannot_hide_failed_method_or_global_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory)
            (root/'compat').mkdir()
            (root/'compat/corpus.json').write_text('{}')
            method='test_diagnostic_read'
            selector='ActualFaultPaths.'+method
            contract=dict(hook_sites=['read'],hook_ordinals=[1],m0_controls=['fired'],
                scenario_id='SAFECASE-VER-005',variant='all declared fault injection hooks',finding_ids=['M0-IO-001'])
            oracle=root/'oracle.json'
            oracle.write_text(json.dumps(dict(tests_run=1,repetitions=100,
                records=[dict(test_selector=selector,contract=contract)])))
            log=root/'test.log'
            template=method+' (__main__.ActualFaultPaths.'+method+') ... FINDING M0-IO-001\n{}\n\n---\nRan 1 test in 0.1s\n\n{}\n'
            fake=types.SimpleNamespace(validate_oracle_report=lambda report:{selector:[dict(site='read',ordinal=1,magic='RLF1')]})
            with patch.dict(sys.modules,{'test_fault_paths':fake}):
                log.write_text(template.format('ok','OK'))
                result=fault_records(oracle,log,root,'ASAN')
                self.assertEqual(result[0]['status'],'finding-open')
                self.assertEqual(result[0]['oracle_report']['sha256'],sha(oracle))
                self.assertEqual(result[0]['observed_hooks'][0]['ordinal'],1)
                for method_outcome,global_outcome in [('FAIL','OK'),('ok','FAILED (failures=1)')]:
                    log.write_text(template.format(method_outcome,global_outcome))
                    with self.assertRaises(ValueError):
                        fault_records(oracle,log,root,'ASAN')


class ScopedCallControls(unittest.TestCase):
    def test_got_load_register_call_requires_exact_resolved_owner(self):
        # Actual Rust O0 pattern observed in the historical binary: a GOT load
        # followed by register-indirect call, with a generic _DYNAMIC label.
        target=ElfTargets.__new__(ElfTargets)
        target.symbols={'__wrap_calloc':{0x86090},'ecalloc':{0x90000}}
        target.slots={0x154918:dict(address=0x86090,name=None,kind=8)}
        target.sections=[]
        text='860c3: mov 0xce84e(%rip),%rax # 154918 <_DYNAMIC+0x1708>\n860ca: call *%rax\n'
        self.assertTrue(target.calls(text,{'__wrap_calloc'}))
        self.assertFalse(target.calls(text,{'ecalloc'}))
        target.slots[0x154918]['address']=0x86091
        self.assertFalse(target.calls(text,{'__wrap_calloc'}))
        target.slots.clear()
        self.assertFalse(target.calls(text,{'__wrap_calloc'}))

    def test_printed_symbol_label_is_not_proof_of_a_call_target(self):
        target=ElfTargets.__new__(ElfTargets)
        target.symbols={'ecalloc':{0x86090}}
        target.slots={0x1534c8:dict(address=0x86090,name=None,kind=8)}
        target.sections=[]
        self.assertTrue(target.calls('e9375: call *0x6a14d(%rip) # 1534c8 <_DYNAMIC+0x608>',{'ecalloc'}))
        self.assertFalse(target.calls('e9375: call 99999 <ecalloc>',{'ecalloc'}))
        self.assertFalse(target.calls('e9375: call 99999 <ecalloc@plt>',{'ecalloc'}))
        self.assertTrue(target.calls('e9375: call 86090 <ecalloc>',{'ecalloc'}))


if __name__=='__main__':
    unittest.main(verbosity=2)
