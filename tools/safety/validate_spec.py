from pathlib import Path
from collections import Counter
import json,re,hashlib,datetime
root=Path(__file__).resolve().parents[2]; s=root/'spec/safety'
def read(n): return json.loads((s/n).read_text())
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def slug(t): return re.sub(r'\s','-',re.sub(r'[^\w\- ]','',t.lower()))
def anchors(p): return {slug(re.sub(r'[*`]', '', x)) for x in re.findall(r'^#{1,6} (.+)$',p.read_text(),re.M)}
r=read('requirements.json'); a=read('acceptance.json'); w=read('work-packages.json'); c=read('campaigns.json'); g=read('github-tracking.json')
for d in [r,a,w,c,g]: assert d['specification_version']=='0.2.0'
req={x['id']:x for x in r['requirements']}; cases={x['id']:x for x in a['scenarios']}; packets={x['id']:x for x in w['work_packages']}
assert len(req)==len(r['requirements'])==43
assert len(cases)==43 and sum(len(x['variants']) for x in cases.values())==175
assert len(packets)==28 and len(w['phases'])==6
parents=set(re.findall(r'^\| ([A-Z]+-\d+) \|',(root/'docs/rless-compatibility-spec.md').read_text(),re.M)); assert len(parents)==39
for x in req.values():
 p=root/x['contract']; assert p.is_file(),p
 assert x['anchor'] in anchors(p),(x['id'],x['anchor'])
 assert re.search(r'^## '+re.escape(x['id'])+' '+re.escape(x['title'])+'$',p.read_text(),re.M),x['id']
 assert set(x['compatibility_parents'])<=parents
 assert x['status']=='specified' and x['evidence_status']=='not-verified'
 for t in x['acceptance_ids']: assert t in cases and x['id'] in cases[t]['requirement_ids']
for x in cases.values():
 assert x['status']=='planned' and x['evidence_status']=='not-verified' and x['implementation_path'] is None
 assert len(x['variants'])==len(set(x['variants'])) and x['variants']
 assert set(x['required_profiles'])<=set(a['profiles'])
 for t in x['requirement_ids']: assert t in req and x['id'] in req[t]['acceptance_ids']
def dag(items):
 seen=set(); active=set()
 def visit(i):
  assert i in items,i
  if i in seen:return
  assert i not in active,i
  active.add(i)
  for dep in items[i]['depends_on']: visit(dep)
  active.remove(i); seen.add(i)
 for i in items:visit(i)
 return 'acyclic'
dag(req);dag(packets)
assert {r for x in packets.values() for r in x['requirement_ids']}==set(req)
phaseids={x['id'] for x in w['phases']}; assert phaseids=={'M'+str(i) for i in range(6)}
issues={x['phase']:x for x in g['phases']}; assert set(issues)==phaseids
for x in list(packets.values())+w['phases']:
 assert x['status'] in ('planned','in-progress') and x['evidence_status']=='not-verified'
 phase=x.get('phase',x['id']); assert phase in phaseids
 assert (x['status']=='in-progress') == (phase=='M0')
 assert x['github_issue']==issues[phase]['url']
for x in packets.values():
 assert set(x['requirement_ids'])<=set(req)
 for p in x['current_paths']: assert (root/p).exists(),p
 assert x['implementation_steps'] and x['deliverables'] and x['exit_gate']
assert c['status']=='planned' and c['evidence_status']=='not-verified'
assert len(c['in_process_targets'])==15 and len({x['id'] for x in c['in_process_targets']})==15
assert len(c['native_event_scenarios'])==12 and len({x['id'] for x in c['native_event_scenarios']})==12
assert set(c['profiles'])==set(a['profiles'])
for x in c['in_process_targets']+c['native_event_scenarios']:
 assert x['status']=='planned' and x['evidence_status']=='not-verified'
 assert set(x['requirement_ids'])<=set(req)
 for p in x.get('current_paths',[]): assert (root/p).exists(),p
 if x['id'].startswith('EVENT'): assert x['release_repeats_min']>=100
final=next(x for x in c['lanes'] if x['id']=='FINAL')
assert final['inputs_per_fuzz_target_min']==100000 and final['cpu_minutes_per_fuzz_target_min']==60 and final['both_fuzz_floors_required']
assert final['bounded_native_scenarios_min']==10000 and final['repeats_per_required_race_recovery_min']==100
assert next(x for x in c['lanes'] if x['id']=='INTEGRATION')['scheduled_now'] is False
assert (root/'tools/safety/run.py').is_file()
closure=read('closure-plan.json')
assert closure['plan_version']=='1.0.0' and closure['safety_specification_version']==r['specification_version']
closure_packets={x['id']:x for x in closure['packets']}
assert len(closure_packets)==len(closure['packets'])==11
dag(closure_packets)
for x in closure_packets.values():
 assert x['parent_packet'] in packets and set(x['requirement_ids'])<=set(req)
 assert set(x['acceptance_ids'])<=set(cases)
 assert x['status'] in ('planned','in-progress') and x['evidence_status']=='not-verified'
 assert x['deliverable_and_exit_gate']
assert 'C0-GATE' in closure_packets['C1-PROTO']['depends_on']
links=0
for p in list(s.glob('*.md'))+[root/'README.md',root/'docs/current-state.md',root/'spec/README.md']:
 for href in re.findall(r'\[[^\]]+\]\(([^)]+)\)',p.read_text()):
  if re.match(r'^(https?://|mailto:)',href):continue
  file,_,anchor=href.partition('#'); dest=(p.parent/file).resolve() if file else p
  assert dest.exists(),(str(p),href)
  if anchor and dest.suffix=='.md': assert anchor in anchors(dest),(str(p),href)
  links+=1
corpus=json.loads((root/'compat/corpus.json').read_text())
for p,h in corpus['files'].items(): assert digest(root/p)==h,p
report={'schema_version':1,'specification_version':'0.2.0','recorded_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'Specification structure and traceability only; no safety implementation or runtime test result is asserted.','specification_checks_passed':True,'runtime_safety_verified':False,'implementation_tests_executed':0,'requirements':43,'planned_acceptance_families':43,'named_variants':175,'workstreams':dict(sorted(Counter(str(x['workstream']) for x in req.values()).items())),'delivery_packets':28,'phases':6,'in_progress_phases':['M0'],'planned_in_process_fuzz_targets':15,'planned_native_event_scenario_groups':12,'checked_local_links':links,'dependency_graph':'acyclic (requirements and delivery packets)','packet_requirement_coverage':'43 of 43','closure_refinement_packets':11,'closure_dependency_graph':'acyclic','original_frozen_corpus':'hashes unchanged','github_tracking':'six phase issues/milestones plus parent; M0 in progress/not-verified','inputs_sha256':{str(p.relative_to(root)):digest(p) for p in sorted(s.iterdir()) if p.suffix in {'.json','.md'} and p.name!='validation.json'}}
(s/'validation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='inputs_sha256'},indent=2))
