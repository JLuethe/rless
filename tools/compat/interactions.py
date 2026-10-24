#!/usr/bin/env python3
import copy,itertools,json
from pathlib import Path
R=Path(__file__).resolve().parents[2]
caps=json.loads((R/'spec/capabilities.json').read_text()); by={c['id']:c for c in caps}
files=json.loads((R/'compat/fixtures/pager.json').read_text())
ready='@@READY@@'; base=['-X','--no-keypad','--no-paste','-Ps'+ready,'-Pm'+ready,'-PM'+ready]
cases=[]
def add(id,ids,argv,mode='pipe',events=None,env=None,patch=None):
 c={'id':id,'capability_ids':ids,'argv':argv,'mode':mode,'files':copy.deepcopy(files),'env':env or {}}
 if events is not None:c['events']=events
 if patch:c['files'].update(patch)
 cases.append(c)
 for cid in ids:by[cid]['case_ids'].append(id)
# Numeric/string parsing must distinguish missing, negative, zero and overflow.
for cap in caps:
 if cap['kind']!='option' or cap['disposition']!='production':continue
 name=cap['spellings'][-1]
 if 'O_STRING' in cap['flags'] or 'O_NUMBER' in cap['flags']:
  add('missing-value-'+cap['id'],[cap['id']],[name])
 if 'O_NUMBER' in cap['flags']:
  for label,value in [('zero','0'),('negative','-2'),('overflow','999999999999999999999999')]:
   add(label+'-'+cap['id'],[cap['id']],[name+'='+value,'input.txt'])
# Pairwise matrix: all combinations of each selected pair, not a claim of all 85-option combinations.
pairs=[('number-chop','-N','-S','line-numbers','chop-long-lines'),('header-number','--header=1,2,1','-N','header','line-numbers'),('search-chop','-i','-S','ignore-case','chop-long-lines'),('search-header','--header=1,0,1','--no-search-headers','header','no-search-headers'),('raw-proc','-R','--proc-backspace','raw-control-chars','proc-backspace'),('size-time','--file-size','--time-file-size=20','file-size','time-file-size'),('quiet-status','-Q','-J','quiet','status-column')]
matrix=[]
for name,a,b,aid,bid in pairs:
 row={'id':name,'dimensions':[[None,a],[None,b]],'case_ids':[]}
 for va,vb in itertools.product([0,1],repeat=2):
  id='pair-'+name+f'-{va}{vb}'; options=([a] if va else [])+([b] if vb else [])
  add(id,['option:'+aid,'option:'+bid],[*options,*base,'input.txt'],mode='pty',events=[{'await':[ready],'snapshot':True},{'send':'/needle 120\n','await':[ready,'(press RETURN)'],'snapshot':True},{'send':'\nq'}]);row['case_ids'].append(id)
 matrix.append(row)
for flag in ['-X','-E']:
 row={'id':'one-screen-F'+flag,'dimensions':[[None,'-F'],[None,flag]],'case_ids':[]}
 for f,x in itertools.product([0,1],repeat=2):
  id='onescreen-'+flag+f'-{f}{x}'; options=(['-F'] if f else [])+([flag] if x else [])
  expect_exit=f or (flag=='-E' and x)
  add(id,['option:quit-if-one-screen','option:no-init' if flag=='-X' else 'option:quit-at-eof'],[*options,'-Ps'+ready,'input.txt'],mode='pty',events=[] if expect_exit else [{'await':[ready],'snapshot':True},{'send':'q'}],patch={'input.txt':{'text':'one\ntwo\nthree\n'}});row['case_ids'].append(id)
 matrix.append(row)
# Configuration source order, empty values, version gates, reset and literal handling.
for name,env,keys in [('local-over-env',{'LESS':'-S'},'#env\nLESS = -N\n'),('system-under-env',{'LESSKEY_CONTENT_SYSTEM':'#env;LESS=-N','LESS':'-S'},''),('inline-over-file',{'LESSKEY_CONTENT':'#env;LESS=-S'},'#env\nLESS = -N\n'),('version-gate',{},'#env\n#version >= 712 LESS = -N\n#version > 712 LESS = -S\n'),('append-expand',{'BASE':'-N'},'#env\nLESS = ${BASE}\nLESS += S\n'),('noconfig',{'LESSNOCONFIG':'-'},'#env\nLESS = -N\n')]:
 add('config-'+name,['surface:startup-order','surface:lesskey-language'],[*base,'input.txt'],mode='pty',events=[{'await':[ready],'snapshot':True},{'send':'q'}],env=env,patch={'.lesskey':{'text':keys}})
for name,env,options in [('raw-env',{'LESS':'-r'},[]),('raw-argv',{},['-r']),('raw-reset',{},['-+R']),('raw-repeat',{},['-r','-r'])]:
 add(name,['option:raw-control-chars'],[*options,*base,'input.txt'],mode='pty',env=env,events=[{'await':[ready]},{'send':'_r','await':['(press RETURN)'],'snapshot':True},{'send':'\nq'}])
# Explicit zero-byte preprocessor distinctions.
for prefix in ['|','||']:
 for status in [0,1]:
  add('empty-preproc-'+str(len(prefix))+'-'+str(status),['surface:lessopen'],['input.txt'],env={'LESSOPEN':prefix+'exit '+str(status)+'; : %s'})
for cap in caps:cap['case_ids']=sorted(set(cap['case_ids']))
reqs=json.loads((R/'spec/requirements.json').read_text())
for req in reqs:req['case_ids']=sorted({id for c in req['capability_ids'] for id in by[c]['case_ids']})
for c in cases:c['fixture_set']='pager'; c['files']={k:v for k,v in c['files'].items() if v!=files.get(k)}
for path,obj in [('compat/cases/interactions.json',cases),('compat/interactions.json',matrix),('spec/capabilities.json',caps),('spec/requirements.json',reqs)]: (R/path).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
print('interaction/boundary cases',len(cases))
