#!/usr/bin/env python3
"""Create behavior-distinguishing sessions for active bindings and language surfaces."""
import base64,copy,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
caps=json.loads((ROOT/'spec/capabilities.json').read_text()); reqs=json.loads((ROOT/'spec/requirements.json').read_text())
for cap in caps: cap['case_ids']=[x for x in cap['case_ids'] if x.startswith(('option-','query-default-'))]
ready='@@READY@@'; end=['(press RETURN)',ready,'HELP --','Press RETURN']
base=['-X','--no-keypad','--no-paste','-Ps'+ready,'-Pm'+ready,'-PM'+ready]
numbered=''.join(f'{n:03d} alpha beta needle {n}\n' for n in range(1,201))
files={'input.txt':{'text':numbered},'second.txt':{'text':numbered.replace('alpha','second')},'tags':{'text':'entry\tinput.txt\t1\n'}}
cases=[]
def add(id,ids,argv=None,events=None,env=None,replace_files=None,mode='pty',**extra):
 c=dict(id=id,capability_ids=ids,mode=mode,argv=argv or [*base,'input.txt','second.txt'],files=copy.deepcopy(files),env=env or {})
 if mode=='pty': c['events']=events or [{'await':[ready],'snapshot':True},{'send':'\nqq'}]
 if replace_files: c['files'].update(replace_files)
 c.update(extra); cases.append(c)
 for cap in caps:
  if cap['id'] in ids: cap['case_ids'].append(id)
 return c

def action_case(cap,key,extra=b'',mapping=None):
 action=cap['action']; id='binding-'+cap['id'].replace(':','-'); event=[{'await':[ready]}]; env={'VISUAL':'/bin/true','EDITOR':'/bin/true'}
 view_files={}; argv=[*base,'input.txt','second.txt']; data=key; done=False
 if cap['kind']=='editing-binding' or cap.get('section')=='line-edit':
  data=b'/alpha beta'+key+(bytes([32,33,33]) if action=='EC_X11MOUSE' else b'0;1;1M' if action=='EC_X116MOUSE' else b'needle\x1b[201~' if action=='A_START_PASTE' else b'')+b'\n'
 elif action=='A_QUIT': done=True
 elif action in ('A_F_FOREVER','A_F_FOREVER_BELL','A_F_UNTIL_HILITE'):
  event += [{'send':'/needle\n','await':end}]
  event += [{'send_b64':base64.b64encode(key).decode(),'await':['Waiting for data'],'snapshot':True},{'send':'\x03','await':end,'snapshot':True},{'send':'\nqq'}]
  data=None
 elif action in ('A_F_SEARCH','A_B_SEARCH'): data+=b'needle\n'
 elif action=='A_FILTER': data+=b'needle\n'
 elif action=='A_SETMARK' or action=='A_SETMARKBOT' or action=='A_CLRMARK': data+=b'a'
 elif action=='A_GOMARK':
  event += [{'send':'ma20g','await':end}]; data+=b'a'
 elif action=='A_DIGIT': data+=b'5g' if key==b'.' else b'g'
 elif action=='A_EXAMINE': data+=b'second.txt\n'
 elif action=='A_DISP_OPTION': data+=b'r'
 elif action=='A_OPT_TOGGLE':
  if extra==b't': data+=b'entry\n'
  elif extra==b'o': data+=b'log\n'
  else: data+=b'N'
 elif action in ('A_SHELL','A_PSHELL'): data+=b'printf CHILD\n'
 elif action=='A_PIPE': data+=b'.cat > pipe.out\n'
 elif action=='A_FIRSTCMD': data+=b'g\n'
 elif action in ('A_F_BRACKET','A_B_BRACKET'):
  view_files['input.txt']={'text':'001 (alpha) [beta] {needle}\n'+numbered}
  if not extra: data+=b'()'
 elif action in ('A_AGAIN_SEARCH','A_REVERSE_SEARCH','A_T_AGAIN_SEARCH','A_T_REVERSE_SEARCH','A_UNDO_SEARCH','A_CLR_SEARCH'):
  event += [{'send':'/needle\n','await':end}]
 elif action in ('A_NEXT_TAG','A_PREV_TAG'):
  argv=[*base,'-tentry']
 elif action=='A_NOACTION': data+=(b'needle\n' if extra==b'/' else b'g\n' if extra==b'+' else b'N' if extra==b'-' else b'5p' if extra==b'.' else b'g') if extra else b'r'
 elif action in ('A_X11MOUSE_IN','A_X116MOUSE_IN'):
  data+=bytes([32,33,33]) if action=='A_X11MOUSE_IN' else b'0;1;1M'
 elif action in ('EC_X11MOUSE','EC_X116MOUSE'): pass
 elif action in ('A_START_PASTE','A_END_PASTE'): data+=b'\x1b[201~r'
 elif action=='A_DEBUG': data+=b'r'
 if mapping is not None:
  view_files['.lesskey']={'text':mapping}
 if data is not None:
  barrier=action=='A_NOACTION' and cap['kind']!='editing-binding' and cap.get('section')!='line-edit'
  if barrier:data+=b'-Ps@@BARRIER-%f@@\n'
  if done: event.append({'send_b64':base64.b64encode(data).decode()})
  else:
   event.append({'send_b64':base64.b64encode(data).decode(),'await':['Prompt (short): @@BARRIER-%f@@  (press RETURN)'] if barrier else end,'snapshot':not barrier})
   if barrier:event.append({'send':'\n','await':['@@BARRIER-input.txt@@'],'snapshot':True})
   event.append({'send':'\nqq'})
 add(id,[cap['id']],argv=argv,events=event,env=env,replace_files=view_files)

for cap in caps:
 if cap['kind'] in ('pager-binding','editing-binding') and cap['disposition']=='production':
  raw=base64.b64decode(cap['key_b64']); extra=base64.b64decode(cap['extra_b64'])
  action_case(cap,raw,extra)
 elif cap['kind']=='lesskey-action':
  action_case(cap,b'9' if cap['action']=='A_DIGIT' else b'Q',mapping='#'+cap['section']+'\n'+('9' if cap['action']=='A_DIGIT' else 'Q')+' '+cap['name']+'\n')

# Diagnostic/copy/parser boundary cases.
pipe_specs=[('unknown-option',['--does-not-exist','input.txt'],{},['surface:cli-grammar']),('missing-P',['-P'],{},['surface:cli-grammar']),('missing-file',['missing'],{},['surface:file-lifecycle']),('missing-first',['missing','input.txt'],{},['surface:redirection']),('missing-last',['input.txt','missing'],{},['surface:redirection']),('interspersed',['input.txt','-N'],{},['surface:cli-grammar']),('posix-empty',['input.txt','-N'],{'POSIXLY_CORRECT':''},['surface:cli-grammar']),('invalid-number',['-b9999999999999999999999999','input.txt'],{},['surface:cli-grammar']),('missing-number',['-b'],{},['surface:cli-grammar']),('binary-copy',['-N','-S','-R','bytes'],{},['surface:redirection']),('copy-stdin',[],{},['surface:redirection']),('secure-zero',['-tentry'],{'LESSSECURE':'0'},['surface:secure-mask']),('secure-allow',['-tentry'],{'LESSSECURE':'0','LESSSECURE_ALLOW':'tags'},['surface:secure-mask']),('ambiguous-prefix',['--mo','input.txt'],{},['surface:cli-grammar']),('unknown-lesskey-action',['--lesskey-content=#command;Q nonexistent','input.txt'],{},['surface:lesskey-language'])]
for name,argv,env,ids in pipe_specs:
 add('io-'+name,ids,argv=argv,env=env,mode='pipe',replace_files={'bytes':{'base64':base64.b64encode(bytes(range(256))+b'\r\nno final newline').decode()}},stdin_b64=base64.b64encode(b'standard\x00input').decode())
# Every variable is associated with a standalone lookup/interpretation session.
env_values={'LC_ALL':'C.UTF-8','TERMCAP':'xterm:co#80:li#24:','LESS_SCREENFILL_TIME':'100','LESS':'-N','LESSKEY_CONTENT':'#env;LESS=-N','LESSKEY_CONTENT_SYSTEM':'#env;LESS=-S','LESSCHARSET':'utf-8','LESSHISTSIZE':'2','LESSHISTFILE':'{ROOT}/history','LESSSECURE':'0','LESSSECURE_ALLOW':'history','LESSSECURE_DISALLOW':'shell','LESSNOCONFIG':'TERM,LESSHISTFILE','LESS_LINES':'12','LESS_COLUMNS':'60','LINES':'10','COLUMNS':'30','LC_CTYPE':'C','LANG':'C','LESS_DATA_DELAY':'10','LESS_IS_MORE':'0','POSIXLY_CORRECT':'','SHELL':'/bin/sh','EDITOR':'/bin/true','VISUAL':'/bin/true','HOME':'{ROOT}','PATH':'/usr/bin:/bin','TERM':'xterm-256color','LESSBINFMT':'*s<%02X>','LESSUTFBINFMT':'<U+%04lX>','LESSCHARDEF':'8bcccbcc18b95.b','LESSUTFCHARDEF':'E000-E0FF:p','LESSANSIENDCHARS':'m','LESSANSIMIDCHARS':'0123456789;[','LESSANSIOSCALLOW':'8','LESSANSIOSCCHARS':']','LESSMETACHARS':'*? ','LESSMETAESCAPE':'\\','LESSEDIT':'%E +%lm %f','LESSGLOBALTAGS':'global','LESSSEPARATOR':':','LESS_SHELL_COPTION':'-c','LESS_SHELL_LINES':'3','LESS_SIGUSR1':'','MORE':'-N','XDG_CONFIG_HOME':'{ROOT}/config','XDG_STATE_HOME':'{ROOT}/state','XDG_DATA_HOME':'{ROOT}/data','LESSKEYIN':'keys','LESSKEYIN_SYSTEM':'keys','LESSMSG':'messages','LESSOPEN':'|cat %s','LESSCLOSE':'printf closed >> close.log','LESSECHO':'/bin/echo'}
for cap in caps:
 if cap['kind']!='environment' or cap['disposition']!='production': continue
 name=cap['name']
 if name.startswith('LESS_TERMCAP_'): env={'LESS_TERMCAP_so':'\x1b[7m'}
 elif name.startswith('LESS_TERMINFO_'): env={'LESS_TERMINFO_smso':'\x1b[7m'}
 elif name.startswith('LESS_OSC8_OPEN_'): env={'LESS_OSC8_OPEN_ANY':'/bin/true'}
 else: env={name:env_values.get(name,'1')}
 add('env-'+name,[cap['id']],env=env,replace_files={'keys':{'text':'#env\nLESS = -N\n'},'messages':{'text':'Invalid_pattern BAD_PATTERN\n'}})
# State sequences exercise semantic effects beyond single-option parsing.
sequences=[('motion',['20g','5j','2b','50%','G','g'],['surface:text-layout']),('marks',['ma','30g',"'a"],['surface:text-layout']),('search',['/needle 120\n','n','N'],['surface:search-grammar']),('regex-backreference',['/([0-9])\\1\n'],['surface:regex-gnu']),('filter',['&needle 1\n','&\n'],['surface:filter']),('files',[':n\n',':p\n',':d\n'],['surface:file-lifecycle']),('query-raw',['_r','-r','_r','-+R','_r'],['surface:cli-grammar']),('pipe',['|.cat > range.out\n'],['surface:pipe-ranges']),('shell',['!printf CHILD > child.out\n'],['surface:shell-editor']),('modeline',['__tabs\n'],['surface:modelines']),('prompt',['='],['surface:prompt-language']),('history',['/needle 90\n','ma'],['surface:history-format']),('translation',['/[\n'],['surface:messages']),('osc',['\x0fn','\x0fl'],['surface:osc8'])]
for name,commands,ids in sequences:
 events=[{'await':[ready]}]
 for command in commands:
  events += [{'send':command,'await':end,'snapshot':True},{'send':'\n'}] if command.startswith(('_','-','!','|')) or name=='translation' else [{'send':command,'await':end,'snapshot':True}]
 events.append({'send':'q'})
 kwargs={}
 if name=='history': kwargs['env']={'LESSHISTFILE':'{ROOT}/history'}
 if name=='translation': kwargs.update(env={'LESSMSG':'messages'},replace_files={'messages':{'text':'Invalid_pattern BAD_PATTERN\n'}})
 if name=='modeline': kwargs.update(argv=['--modelines=1',*base,'input.txt'],replace_files={'input.txt':{'text':'less: set ts=4:\n'+numbered}})
 if name=='osc': kwargs['replace_files']={'input.txt':{'text':'\x1b]8;;file:///tmp/fixture\x1b\\LINK\x1b]8;;\x1b\\\n'+numbered}}
 add('sequence-'+name,ids,events=events,**kwargs)
for term in ['vt100','dumb']:
 add('term-'+term,['surface:fd-topology'],term=term,argv=['-d',*base,'input.txt'])
for locale in ['C','C.UTF-8']:
 add('unicode-'+locale,['surface:unicode-tables'],locale=locale,replace_files={'input.txt':{'text':'宽 é \t column\n'+numbered}})
add('resize',['surface:signals'],events=[{'await':[ready]},{'resize':[12,60],'await':[ready],'snapshot':True},{'send':'\nqq'}])
add('signal-term',['surface:signals'],events=[{'await':[ready]},{'signal':'SIGTERM'}])
add('stream-close',['surface:stream-buffers'],argv=base,stream_b64=base64.b64encode(numbered.encode()).decode(),events=[{'await':[ready],'snapshot':True},{'close_stream':True},{'send':'G','await':end,'snapshot':True},{'send':'\nqq'}])
add('stream-size',['surface:file-size-scan'],argv=['--file-size',*base],stream_b64=base64.b64encode(numbered.encode()).decode(),events=[{'close_stream':True},{'await':[ready],'snapshot':True},{'send':'\nqq'}])
add('preprocessor',['surface:lessopen'],env={'LESSOPEN':'|printf PREPROCESSED; printf invoked > hook.log; : %s'},replace_files={'input.txt':{'text':'original\n'}})
add('log',['surface:logging'],argv=['-olog',*base],stream_b64=base64.b64encode(numbered.encode()).decode(),events=[{'await':[ready]},{'close_stream':True},{'send':'G','await':end,'snapshot':True},{'send':'\nqq'}])
add('more',['surface:more-mode'],env={'LESS_IS_MORE':'1'})
add('tags',['surface:tags'],argv=[*base,'-tentry'])
add('lessecho',['surface:lessecho'],argv=['-V'],mode='pipe',executable='lessecho')
add('startup-lookup',['surface:startup-order'],env={'LESS':'-S'},replace_files={'.lesskey':{'text':'#env\nLESS = -N\nLESSSECURE = 1\n'}},events=[{'await':[ready]},{'send':'!printf allowed > security.log\n','await':end,'snapshot':True},{'send':'\nq'}])
add('mouse-scroll',['surface:mouse-reports'],argv=['--mouse',*base,'input.txt'],events=[{'await':[ready]},{'send':'\x1b[<65;5;5M'+'r','await':end,'snapshot':True},{'send':'q'}])
add('bracketed-paste',['surface:paste'],argv=[x for x in base if x!='--no-paste']+['input.txt'],events=[{'await':[ready]},{'send':'/\x1b[200~needle\x1b[201~\n','await':end,'snapshot':True},{'send':'q'}])
add('follow-interrupt',['surface:follow'],events=[{'await':[ready]},{'send':'F','await':['Waiting for data'],'snapshot':True},{'signal':'SIGINT','await':end,'snapshot':True},{'send':'\nqq'}])
add('giant-line',['surface:limits-performance'],replace_files={'input.txt':{'text':'X'*100000+'\n'+numbered}})
# Persist case membership for every capability and requirement.
bycap={c['id']:c for c in caps}
for req in reqs: req['case_ids']=sorted({cid for cap in req['capability_ids'] for cid in bycap[cap]['case_ids']})
shared=json.loads((ROOT/'compat/fixtures/pager.json').read_text())
for c in cases:c['fixture_set']='pager'; c['files']={k:v for k,v in c['files'].items() if v!=shared.get(k)}
for path,obj in [('compat/cases/behavior.json',cases),('spec/capabilities.json',caps),('spec/requirements.json',reqs)]: (ROOT/path).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
print('behavior cases',len(cases),'uncased production',[c['id'] for c in caps if c['disposition']=='production' and not c['case_ids']])
