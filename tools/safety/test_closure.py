"""Mutation controls for ownership review and acceptance evidence completeness."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from common import Evidence, sha
from coverage import bind, matrix
from ownership import CHECKLIST, seed, validate
from inventory_format import REGION_REGISTER


class ReviewControls(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.root=Path(self.temp.name)
        (self.root/'src').mkdir();(self.root/'src/fixture.rs').write_text('static mut OBJECT: i32 = 0;')
        (self.root/'review.md').write_text('Fixture owner, callback and error-path review.')
        self.inventory=dict(files={'src/fixture.rs':{'sha256':sha(self.root/'src/fixture.rs')}},
            mutable_static_definitions=[dict(file='src/fixture.rs',context='',symbol='OBJECT')],
            candidate_external_symbols=['free'],
            indirect_call_sites=[dict(file='src/fixture.rs',context='callback',line=1,column=1,symbol='callback')])
        # These fixtures test review metadata only. Source completeness has its
        # own unmocked integration and mutation tests in test_inventory_coverage.
        self.inventory.update(schema_version=2,expanded_configurations={},
            static_definitions=[dict(**row,declared_type='i32',binding_mutability='mutable')
                                for row in self.inventory['mutable_static_definitions']])
        completeness=patch('ownership.validate_inventory_v2',return_value=None)
        completeness.start()
        self.addCleanup(completeness.stop)
        self.regions={'regions':[dict(id='OWN-TST',current_paths=['src/fixture.rs'],delivery_packet='M0.2')]}
        canonical=self.root/REGION_REGISTER
        canonical.parent.mkdir(parents=True)
        canonical.write_text(json.dumps(self.regions))
        self.inventory.update(manual_register=REGION_REGISTER,manual_register_sha256=sha(canonical))
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

    def test_historical_inventory_cannot_approve_complete_review(self):
        self.inventory['schema_version']=1
        self.review=seed(self.inventory,self.regions,self.root)
        self.complete()
        with self.assertRaisesRegex(ValueError,'source-derived schema-2'):
            self.check()
        result=validate(self.review,self.inventory,self.regions,self.findings,self.root,complete=False)
        self.assertEqual(result['status'],'not-verified')

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

    def test_multiple_partial_oracles_cannot_be_promoted_to_complete(self):
        self.evidence.run('second',[sys.executable,'-c','print("second fixture oracle")'],
            scenario_id='CASE-TST', variant='fault before commit',profile='NATIVE')
        item={key:value for key,value in self.binding.items() if key not in ('report','record_name')}
        item['executions']=[dict(report=self.binding['report'],record_name=name)
                            for name in ('selected','second')]
        self.assertEqual(bind(self.rows,[item],self.root)[item['row_id']]['status'],'not-verified')
        with self.assertRaises(ValueError):
            bind(self.rows,[{**item,'coverage_scope':'complete'}],self.root)
        item['executions'].append(item['executions'][0])
        with self.assertRaises(ValueError):bind(self.rows,[item],self.root)

    def test_binding_preserves_legacy_findings_without_closing_row(self):
        self.evidence.run('legacy',[sys.executable,'-c','print("known behavior reproduced")'],
            scenario_id='CASE-TST',variant='fault before commit',profile='NATIVE',finding_ids=['LEGACY-1'])
        item={**self.binding,'record_name':'legacy'}
        row=bind(self.rows,[item],self.root)[item['row_id']]
        self.assertEqual(row['status'],'finding-open')
        self.assertEqual(row['finding_ids'],['LEGACY-1'])

    def test_binding_cannot_change_hook_ordinal_abi_seed_or_corpus(self):
        (self.root/'compat').mkdir()
        corpus=self.root/'compat/corpus.json';corpus.write_text('{}')
        evidence=Evidence(self.directory/'metadata',self.root)
        metadata=dict(abi_profile='BUILDER-GLIBC-2.44',corpus_sha256=sha(corpus),
            seed=712,test_selector='Fixture.selected',hook_sites=['read'],hook_ordinals=[1])
        evidence.run('selected',[sys.executable,'-c','print("oracle")'],scenario_id='CASE-TST',
            variant='fault before commit',profile='NATIVE',oracle_metadata=metadata)
        item={**self.binding,'report':str(evidence.directory/'summary.json'),'oracle_metadata':metadata}
        bind(self.rows,[item],self.root)
        for field,value in [('hook_ordinals',[2]),('abi_profile','REFERENCE-GLIBC-2.41'),
                            ('seed',713),('corpus_sha256','0'*64)]:
            with self.assertRaises(ValueError):
                bind(self.rows,[{**item,'oracle_metadata':{**metadata,field:value}}],self.root)


if __name__=='__main__':unittest.main()
