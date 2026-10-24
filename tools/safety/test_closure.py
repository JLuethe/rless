"""Mutation controls for ownership review and acceptance evidence completeness."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from common import Evidence, sha
from coverage import bind, matrix
from ownership import CHECKLIST, seed, validate


class ReviewControls(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.root=Path(self.temp.name)
        (self.root/'src').mkdir();(self.root/'src/fixture.rs').write_text('static mut OBJECT: i32 = 0;')
        (self.root/'review.md').write_text('Fixture owner, callback and error-path review.')
        self.inventory=dict(files={'src/fixture.rs':{'sha256':sha(self.root/'src/fixture.rs')}},
            mutable_static_definitions=[dict(file='src/fixture.rs',context='',symbol='OBJECT')],
            candidate_external_symbols=['free'],
            indirect_call_sites=[dict(file='src/fixture.rs',context='callback',line=1,column=1,symbol='callback')])
        self.regions={'regions':[dict(id='OWN-TST',current_paths=['src/fixture.rs'],delivery_packet='M0.2')]}
        self.findings={'findings':[dict(id='FINDING-TST',status='open')]}
        self.review=seed(self.inventory,self.regions,self.root)

    def tearDown(self):self.temp.cleanup()

    def complete(self):
        for item in self.review['entries']:
            item.update(status='reviewed-clear',reviewer='Fixture Reviewer',packet='M0.2',evidence_paths=['review.md'],evidence_sha256={'review.md':sha(self.root/'review.md')},
                        checklist={key:'Fixture analysis with explicit owner/alias/release invariant.' for key in CHECKLIST})

    def check(self):return validate(self.review,self.inventory,self.regions,self.findings,self.root)

    def test_seed_is_not_review_and_full_fixture_has_positive_counterpart(self):
        with self.assertRaises(ValueError):self.check()
        self.assertEqual(len(self.review['entries']),5)
        self.complete();self.assertEqual(self.check()['status'],'reviewed')

    def test_omitted_or_duplicated_entry_rejects(self):
        self.complete();original=copy.deepcopy(self.review)
        self.review['entries'].pop()
        with self.assertRaises(ValueError):self.check()
        self.review=original;self.review['entries'].append(copy.deepcopy(self.review['entries'][0]))
        with self.assertRaises(ValueError):self.check()

    def test_stale_source_and_inventory_rejects(self):
        self.complete();(self.root/'src/fixture.rs').write_text('changed source')
        with self.assertRaises(ValueError):self.check()

    def test_missing_alias_callback_error_or_release_analysis_rejects(self):
        self.complete()
        for field in ('aliases_escapes','callback_targets','error_cleanup','release_pair'):
            saved=self.review['entries'][0]['checklist'][field]
            self.review['entries'][0]['checklist'][field]=''
            with self.assertRaises(ValueError):self.check()
            self.review['entries'][0]['checklist'][field]=saved

    def test_hazard_requires_live_finding_and_retains_unsafe_status(self):
        self.complete();item=self.review['entries'][0];item['status']='reviewed-hazard-recorded'
        with self.assertRaises(ValueError):self.check()
        item['finding_ids']=['FINDING-TST'];self.assertFalse(self.check()['runtime_safety_verified'])
        self.findings['findings'][0]['status']='closed'
        with self.assertRaises(ValueError):self.check()

    def test_empty_or_escaping_evidence_rejects(self):
        self.complete();self.review['entries'][0]['evidence_paths']=[]
        with self.assertRaises(ValueError):self.check()
        self.review['entries'][0]['evidence_paths']=['../outside.md']
        with self.assertRaises(ValueError):self.check()

    def test_changed_review_notes_and_unknown_packet_rejects(self):
        self.complete();self.review['entries'][0]['packet']='invented-packet'
        with self.assertRaises(ValueError):self.check()
        self.review['entries'][0]['packet']='M0.2'
        (self.root/'review.md').write_text('modified after review')
        with self.assertRaises(ValueError):self.check()


class AcceptanceControls(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.directory=Path(self.temp.name);self.root=self.directory/'repo'
        self.root.mkdir();(self.root/'src').mkdir();(self.root/'src/check.py').write_text('print("fixture")')
        subprocess.run(['git','init','-q',self.root],check=True)
        subprocess.run(['git','-C',self.root,'add','.'],check=True)
        subprocess.run(['git','-C',self.root,'-c','user.name=Safety Fixture','-c','user.email=fixture@invalid','commit','-qm','fixture'],check=True)
        self.catalogue=dict(profiles={'NATIVE':'Native','MIRI':'Miri'},scenarios=[dict(id='CASE-TST',
            variants=['fault before commit','fault after commit'],required_profiles=['NATIVE','MIRI'],requirement_ids=[])])
        self.rows=matrix(self.catalogue)
        self.evidence=Evidence(self.directory/'run',self.root)
        self.evidence.run('selected',[sys.executable,'-c','print("fixture oracle")'],scenario_id='CASE-TST',
                          variant='fault before commit',profile='NATIVE')
        self.evidence.finish()
        row=next(x for x in self.rows.values() if x['profile']=='NATIVE' and x['variant']=='fault before commit')
        self.binding=dict(row_id=row['id'],scenario_id=row['scenario_id'],variant=row['variant'],profile=row['profile'],
            implementation_path='src/check.py',coverage_scope='partial',scope_notes='Fixture covers only one site.',
            report=str(self.evidence.directory/'summary.json'),record_name='selected')

    def tearDown(self):self.temp.cleanup()

    def test_exact_matrix_is_stable_under_variant_order(self):
        self.assertEqual(len(self.rows),4)
        self.catalogue['scenarios'][0]['variants'].reverse()
        self.assertEqual(set(self.rows),set(matrix(self.catalogue)))

    def test_partial_diagnostic_never_closes_whole_variant(self):
        result=bind(self.rows,[self.binding],self.root)
        self.assertEqual(result[self.binding['row_id']]['status'],'not-verified')
        self.assertEqual(sum(x['status']=='not-run' for x in result.values()),3)
        with self.assertRaises(ValueError):
            bind(self.rows,[{**self.binding,'coverage_scope':'complete'}],self.root)

    def test_unknown_duplicate_and_wrong_variant_rejects(self):
        for binding in [{**self.binding,'row_id':'unknown'}, {**self.binding,'variant':'similar but wrong variant'}]:
            with self.assertRaises(ValueError):bind(self.rows,[binding],self.root)
        with self.assertRaises(ValueError):bind(self.rows,[self.binding,self.binding],self.root)

    def test_unexecuted_selector_and_stale_source_rejects(self):
        with self.assertRaises(ValueError):bind(self.rows,[{**self.binding,'record_name':'never-executed'}],self.root)
        (self.root/'src/check.py').write_text('changed')
        with self.assertRaises(ValueError):bind(self.rows,[self.binding],self.root)

    def test_unfingerprinted_implementation_cannot_borrow_another_tests_result(self):
        (self.root/'untracked-notes.md').write_text('not the tested implementation')
        with self.assertRaises(ValueError):
            bind(self.rows,[{**self.binding,'implementation_path':'untracked-notes.md'}],self.root)

    def test_duplicate_variants_and_unknown_profiles_rejects(self):
        self.catalogue['scenarios'][0]['variants'].append('fault before commit')
        with self.assertRaises(ValueError):matrix(self.catalogue)
        self.catalogue['scenarios'][0]['variants'].pop()
        self.catalogue['scenarios'][0]['required_profiles'].append('UNKNOWN')
        with self.assertRaises(ValueError):matrix(self.catalogue)


if __name__=='__main__':unittest.main()
