"""Ownership packet precision controls; mutations update every enclosing digest."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from common import sha
from ownership import CHECKLIST,validate_review_packet,validate_finding_mentions
from ownership_fixtures import packet

class PacketPrecision(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name);(self.root/'src').mkdir()
        self.source=self.root/'src/state.rs'
        self.source.write_text('static STATE: usize = 0;\n'+'// Fixture line.\n'*159)
        self.inventory={'files':{'src/state.rs':{'sha256':sha(self.source)}}}
        self.review={'inventory_sha256':'a'*64}
        self.item=dict(id='STATIC:fixture',kind='shared-static-definition',paths=['src/state.rs'],symbol='STATE',
                       reviewer='Fixture reviewer',evidence_paths=[],evidence_sha256={},status='reviewed-clear',finding_ids=[])
        packet(self.item,self.review,self.inventory,self.root)
        self.payload=json.loads((self.root/self.item['review_packet']['path']).read_text())
        self.check()
    def save(self):
        path=self.root/self.item['review_packet']['path'];self.payload['checklist']=copy.deepcopy(self.item['checklist'])
        self.payload['finding_context']=copy.deepcopy(self.item.get('finding_context',{}))
        path.write_text(json.dumps(self.payload))
        self.item['review_packet']['sha256']=sha(path);self.item['evidence_sha256'][self.item['review_packet']['path']]=sha(path)
    def check(self):validate_review_packet(self.item,self.review,self.inventory,self.root)
    def citation(self,path,a,b):
        lines=path.read_text().splitlines(keepends=True)
        return dict(target='source:'+str(path.relative_to(self.root)),sha256=sha(path),start_line=a,end_line=b,
                    excerpt_sha256=hashlib.sha256(''.join(lines[a-1:b]).encode()).hexdigest())
    def test_identical_topic_extents_rehashed_are_rejected(self):
        refs=copy.deepcopy(self.item['checklist']['owner_creation']['source_refs'])
        for topic in CHECKLIST:self.item['checklist'][topic]['source_refs']=copy.deepcopy(refs)
        self.save()
        with self.assertRaisesRegex(ValueError,'identical citation extents'):self.check()
    def test_duplicate_citations_do_not_create_topic_diversity(self):
        ref=copy.deepcopy(self.item['checklist']['owner_creation']['source_refs'])
        for index,topic in enumerate(CHECKLIST):
            self.item['checklist'][topic]['source_refs']=copy.deepcopy(ref)*(index+1)
        self.save()
        with self.assertRaisesRegex(ValueError,'identical citation extents'):self.check()
    def test_reordered_limitations_do_not_create_topic_diversity(self):
        notes=['First repeated fixture limitation.', 'Second repeated fixture limitation.']
        for index,topic in enumerate(CHECKLIST):
            self.item['checklist'][topic]['limitations']=notes if index%2 else list(reversed(notes))
        self.save()
        with self.assertRaisesRegex(ValueError,'identical limitations'):self.check()
    def test_150_lines_allowed_151_rejected_even_when_rehashed(self):
        field=self.item['checklist']['aliases_escapes']
        field['source_refs']=[self.citation(self.source,1,150)];self.save();self.check()
        field['source_refs']=[self.citation(self.source,1,151)];self.save()
        with self.assertRaisesRegex(ValueError,'150-line limit'):self.check()
    def test_target_label_alone_is_not_a_target_claim(self):
        self.item['checklist']['owner_creation']['analysis']='STATE: This generic paragraph asserts that all ownership and cleanup obligations have been considered.'
        self.save()
        with self.assertRaisesRegex(ValueError,'only in its label'):self.check()
    def test_identical_limitation_lists_rehashed_are_rejected(self):
        for topic in CHECKLIST:self.item['checklist'][topic]['limitations']=['The same generic limitation repeated on every topic.']
        self.save()
        with self.assertRaisesRegex(ValueError,'identical limitations'):self.check()
    def test_empty_limitations_on_all_topics_are_also_rejected(self):
        for topic in CHECKLIST:self.item['checklist'][topic]['limitations']=[]
        self.save()
        with self.assertRaisesRegex(ValueError,'identical limitations'):self.check()
    def test_related_source_needs_hash_and_explicit_relationship(self):
        caller=self.root/'src/caller.rs';caller.write_text('fn caller() { let copy = STATE; }\n')
        self.inventory['files']['src/caller.rs']={'sha256':sha(caller)}
        self.item['checklist']['aliases_escapes']['source_refs']=[self.citation(caller,1,1)];self.save()
        with self.assertRaisesRegex(ValueError,'unrelated'):self.check()
        self.payload['related_sources']={'src/caller.rs':dict(sha256=sha(caller),relationship='The caller reads STATE by value and retains no borrow of its storage.')}
        self.save();self.check()
        caller.write_text('fn caller() { }\n')
        with self.assertRaisesRegex(ValueError,'related source'):self.check()
    def test_unrelated_readme_is_never_a_source_citation(self):
        doc=self.root/'README.md';doc.write_text('STATE is described in an unrelated overview.\n')
        self.item['checklist']['aliases_escapes']['source_refs']=[self.citation(doc,1,1)];self.save()
        with self.assertRaisesRegex(ValueError,'unrelated'):self.check()
    def test_mentioned_finding_must_be_indexed_even_for_clear_storage(self):
        known={'findings':[{'id':'M0-SIG-002','status':'open'}]}
        self.item['checklist']['signals_handoff']['limitations']=['M0-SIG-002 remains open for the asynchronous caller, outside the atomic storage scope.']
        with self.assertRaisesRegex(ValueError,'missing from finding_ids'):validate_finding_mentions(self.item,known)
        self.item['finding_ids']=['M0-SIG-002']
        with self.assertRaisesRegex(ValueError,'context-only'):validate_finding_mentions(self.item,known)
        self.item['finding_context']={'M0-SIG-002':'Context only: the clear storage judgment does not approve the asynchronous caller or close its finding.'}
        self.save();self.check();validate_finding_mentions(self.item,known)
    def test_unknown_finding_is_rejected(self):
        self.item['finding_ids']=['UNKNOWN']
        with self.assertRaisesRegex(ValueError,'unknown'):validate_finding_mentions(self.item,{'findings':[]})

if __name__=='__main__':unittest.main()
