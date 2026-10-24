#!/usr/bin/env python3
"""Extract the pinned interface tables; generate a reviewable coverage catalogue."""
import argparse, ast, base64, curses, hashlib, json, re, subprocess
from collections import defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser(); p.add_argument('--source',type=Path,required=True); args=p.parse_args(); src=args.source
SHA='7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=src,text=True).strip()==SHA
url=f'https://github.com/gwsw/less/blob/{SHA}/'
def save(path,obj):
 if path.startswith('compat/cases/'):
  shared=json.loads((ROOT/'compat/fixtures/pager.json').read_text())
  for case in obj:
   case['fixture_set']='pager'; case['files']={k:v for k,v in case['files'].items() if v!=shared.get(k)}
 (ROOT/path).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
def ref(file,pos): return {'file':file,'line':(src/file).read_text().count('\n',0,pos)+1,'url':url+file+'#L'+str((src/file).read_text().count('\n',0,pos)+1)}
def text(value): return {'text':value}
def payload(value): return {'base64':base64.b64encode(value).decode()}
def split_tokens(expr): return re.findall(r"SK\([^)]*\)|CONTROL\('[^']+'\)|'(?:\\.|[^'\\])*'|\bESC\b|\b0\b",expr)
# Freeze the target's special-key expansion; absent capabilities are inactive bindings.
curses.setupterm('xterm-256color')
screen=(src/'screen.c').read_text(); linux=screen[screen.index('\ts = ltgetstr("kcuf1", "kr", &sp);',screen.index('public constant char * special_key_str')):]
linux=linux[:linux.index('#endif\n\tcase SK_CONTROL_K:')]
keynames={m[1]:m[2] for m in re.finditer(r'case (SK_\w+):\s*\n\s*s = ltgetstr\("([^"]+)"',linux)}
keynames['SK_RIGHT_ARROW']='kcuf1'
keys={k:curses.tigetstr(v) for k,v in keynames.items()}
keys.update(SK_CONTROL_K=b'\x0b')
if not keys.get('SK_BACKSPACE'): keys['SK_BACKSPACE']=b'\x08'
if not keys.get('SK_DELETE'): keys['SK_DELETE']=b'\x7f'
def decode(expr):
 out=bytearray()
 for token in split_tokens(expr):
  if token.startswith('SK('):
   key=keys.get(token[3:-1])
   if not key: return None
   out.extend(key)
  elif token.startswith('CONTROL'):
   out.append(ord(ast.literal_eval(token[8:-1]))&31)
  elif token=='ESC': out.append(27)
  elif token=='0': out.append(0)
  else: out.extend(ast.literal_eval(token).encode('latin1'))
 return bytes(out)
caps=[]; requirements=[]; cases=[]
numbered=''.join(f'{n:03d} alpha beta needle {n}\n' for n in range(1,201))
files={'input.txt':text(numbered),'second.txt':text(numbered.replace('alpha','second')),'tags':text('entry\tinput.txt\t1\n')}
ready='@@READY@@'
base=['-X','--no-keypad','--no-paste','-Ps'+ready,'-Pm'+ready,'-PM'+ready]
def case(id,cap,argv,mode='pipe',env=None,events=None,extra=None):
 data={'id':id,'capability_ids':[cap],'mode':mode,'argv':argv,'files':files.copy(),'env':env or {}}
 if events is not None: data['events']=events
 if extra: data.update(extra)
 cases.append(data); return id

def attach(cap,parent,doc,clause):
 cap['parent_requirement']=parent; cap['contract']=doc; cap['reference_clause']=clause
 cap['requirement_id']=parent+'.'+str(1+sum(x['parent']==parent for x in requirements)).zfill(3)
 requirements.append({'id':cap['requirement_id'],'parent':parent,'capability_ids':[cap['id']],'contract':doc,'clause':clause,'case_ids':cap.get('case_ids',[]),'evidence':'source-backed','status':'specified'})
 caps.append(cap)

values={'buffers':'128','max-back-scroll':'3','jump-target':'.5','color':'M+r','lesskey-file':'obsolete','lesskey-src':'keys','lesskey-content':'#env;LESS=-N','log-file':'log','LOG-FILE':'log','pattern':'needle','prompt':'CUSTOM','tabs':'4,8','max-forw-scroll':'3','window':'10','quotes':'""','shift':'.25','rscroll':'>','wheel-lines':'4','line-num-width':'5','status-col-width':'2','header':'1,2,1','emouse':'vmove,click','search-options':'W','modelines':'2','intr':'x','proc-backspace':None,'time-file-size':'20','cmd':'g','match-shift':'.25','autosave':'m/!','end-prompt':'END','tag':'entry','tag-file':'tags'}
s=(src/'opttbl.c').read_text()
names={m[1]:(m[2],m[3]) for m in re.finditer(r'static struct optname (\w+)\s*=\s*\{\s*"([^"]+)",\s*(NULL|&\w+)\s*\}',s)}
pat=r"\{\s*('(?:\\.|[^'])+'|OLETTER_NONE),\s*&([A-Za-z0-9_]+),\s*([^,]+),\s*([^,]+),\s*([^,]+),\s*([^,]+),"
options=[]
for m in re.finditer(pat,s):
 short,var,flags,default,state,handler=[x.strip() for x in m.groups()]; aliases=[]; v=var
 while v:
  name,nxt=names[v]; aliases.append(name); v=nxt[1:] if nxt.startswith('&') else None
 shorts=[] if short=='OLETTER_NONE' else [ast.literal_eval(short)]
 if 'O_TRIPLE' in flags:
  shorts += [x.upper() for x in shorts if x.upper()!=x]
  aliases += [x.upper() for x in aliases if x.upper()!=x]
 name=aliases[0]; id='option:'+name; spellings=['-'+x for x in shorts]+['--'+x for x in aliases]
 cap=dict(id=id,kind='option',spellings=spellings,flags=flags.split('|'),table_default=default,state=state,handler=handler,source=ref('opttbl.c',m.start()),disposition='test-only' if name=='tty' else 'rejected-legacy' if name=='lesskey-file' else 'production',case_ids=[])
 options.append(cap)
 if name!='tty':
  for i,spelling in enumerate(spellings):
   value=values.get(name,'3' if 'O_NUMBER' in flags else 'value')
   arg=spelling+('='+str(value) if spelling.startswith('--') else str(value)) if 'O_STRING' in flags or 'O_NUMBER' in flags else spelling
   argv=[arg,'input.txt']
   datafiles={}
   if name=='tag': argv=[arg]
   if name=='lesskey-src': datafiles['keys']=text('#env\nLESS = -N\n')
   cid='option-'+name+'-'+str(i)
   if name=='version': cap['case_ids'].append(case(cid,id,argv))
   elif name=='help': cap['case_ids'].append(case(cid,id,argv,'pty',events=[{'await':['HELP --'],'snapshot':True},{'send':'q'}]))
   elif name=='prompt': cap['case_ids'].append(case(cid,id,['-X',arg,'input.txt'],'pty',events=[{'await':['CUSTOM'],'snapshot':True},{'send':'q'}]))
   elif name=='end-prompt': cap['case_ids'].append(case(cid,id,[*base,arg,'input.txt'],'pty',events=[{'await':[ready],'snapshot':True},{'send':'q'}]))
   elif name in ('lesskey-file',): cap['case_ids'].append(case(cid,id,argv))
   else:
    argv=(['--use-color'] if name=='color' else [])+[arg,*base]+([] if name=='tag' else ['input.txt'])
    cap['case_ids'].append(case(cid,id,argv,'pty',events=[{'await':[ready],'snapshot':True},{'send':'q'}],extra={'files':{**files,**datafiles}}))
  # Defaults and valid runtime queries are separate from parser spelling cases.
  if 'O_NO_QUERY' not in flags and 'O_NOVAR' not in flags:
   query='__'+name+'\n'
   cid='query-default-'+name
   cap['case_ids'].append(case(cid,id,[*base,'input.txt'],'pty',events=[{'await':[ready]},{'send':query,'await':['(press RETURN)',ready],'snapshot':True},{'send':'\nq'}]))
 attach(cap,'CLI-01','spec/behavior/01-startup-options.md','OPTIONS: '+name)
assert len(options)==86
def action_parent(action):
 if action in ('A_QUIT','A_VERSION'):return 'EXIT-01'
 if action=='A_HELP' or action in ('A_EXAMINE','A_NEXT_FILE','A_PREV_FILE','A_INDEX_FILE','A_REMOVE_FILE'):return 'FILE-01'
 if action in ('A_F_FOREVER','A_F_FOREVER_BELL','A_F_UNTIL_HILITE'):return 'FOLLOW-01'
 if 'SEARCH' in action:return 'SEARCH-03' if action in ('A_UNDO_SEARCH','A_CLR_SEARCH') else 'SEARCH-01'
 if action=='A_FILTER':return 'SEARCH-03'
 if action in ('A_DISP_OPTION','A_OPT_TOGGLE'):return 'CLI-04'
 if action=='A_FIRSTCMD':return 'CLI-03'
 if action=='A_STAT':return 'PROMPT-01'
 if action in ('A_SETMARK','A_SETMARKBOT','A_CLRMARK','A_GOMARK'):return 'NAV-03'
 if action in ('A_NEXT_TAG','A_PREV_TAG'):return 'FILE-02'
 if action in ('A_REPAINT','A_FREPAINT'):return 'TERM-01'
 if action in ('A_SHELL','A_PSHELL','A_VISUAL'):return 'EXT-02'
 if action=='A_PIPE':return 'EXT-03'
 if action.startswith('A_OSC8'):return 'EXT-04'
 if action.startswith(('A_GO','A_PERCENT')) or 'BRACKET' in action:return 'NAV-02'
 return 'NAV-01'

# Keyboard binding records, retaining exact byte expansions and inactive capabilities.
s=(src/'decode.c').read_text()
for table,parent,doc in [('cmdtable','NAV-01','spec/behavior/04-layout-navigation.md'),('edittable','EDIT-01','spec/behavior/06-languages-state.md')]:
 m=re.search(r'static unsigned char '+table+r'\[\]\s*=\s*\{(.*?)\n\};',s,re.S)
 index=0; offset=m.start(1)
 for line in m[1].splitlines(keepends=True):
  clean=re.sub(r'/\*.*?\*/','',line).strip()
  if not clean or clean.startswith(('#','/*','*')): offset+=len(line); continue
  b=re.match(r'(.*?,\s*0,)\s*((?:A_|EC_)[A-Z0-9_]+)(\|A_EXTRA)?(.*)',clean); assert b,clean
  prefix,action,extra,tail=b.groups(); raw=decode(prefix); extra_bytes=decode(tail) if extra else b''
  if raw is not None: raw=raw[:-1]
  if extra_bytes: extra_bytes=extra_bytes.rstrip(b'\0')
  id=table+':'+str(index).zfill(3)
  cap=dict(id=id,kind='pager-binding' if table=='cmdtable' else 'editing-binding',action=action,key_expression=prefix,extra_expression=tail if extra else '',key_b64=base64.b64encode(raw).decode() if raw is not None else None,extra_b64=base64.b64encode(extra_bytes or b'').decode(),source=ref('decode.c',offset),disposition='production' if raw is not None else 'inactive-terminal-capability',case_ids=[])
  attach(cap,action_parent(action) if table=='cmdtable' else parent,doc,action); index+=1; offset+=len(line)
 assert index=={'cmdtable':182,'edittable':74}[table]
# lesskey action vocabulary.
s=(src/'lesskey_parse.c').read_text()
for table,section in [('cmdnames','command'),('editnames','line-edit')]:
 m=re.search(r'static constant struct lesskey_cmdname '+table+r'\[\]\s*=\s*\{(.*?)\n\};',s,re.S)
 for a in re.finditer(r'\{ "([^"]+)",\s+([A-Z0-9_]+)\s*\}',m[1]):
  cap=dict(id='lesskey:'+section+':'+a[1],kind='lesskey-action',section=section,name=a[1],action=a[2],source=ref('lesskey_parse.c',m.start(1)+a.start()),disposition='production',case_ids=[])
  attach(cap,'CONFIG-01','spec/behavior/06-languages-state.md',section+': '+a[1])
# Manual ENVIRONMENT labels and their complete rendered clauses.
s=(src/'less.nro.VER').read_text(); start=s.index('.SH ENVIRONMENT'); end=s.index('.SH "SEE ALSO"',start)
for m in re.finditer(r'^\.IP\s+(.*?)\s*$',s[start:end],re.M):
 name=m[1].strip('" ').strip()
 if not re.fullmatch(r'[A-Z][A-Za-z0-9_, ]*',name) or name=='Linux and Unix': continue
 disposition='other-platform' if name in ('HOMEDRIVE, HOMEPATH','INIT') else 'production'
 cap=dict(id='env:'+name,kind='environment',name=name,source=ref('less.nro.VER',start+m.start()),disposition=disposition,case_ids=[])
 attach(cap,'CONFIG-04','spec/behavior/01-startup-options.md','ENVIRONMENT: '+name)
# Literal lookups not separately described by the ENVIRONMENT headings.
for name,file,disposition in [('LC_ALL','charset.c','production'),('TERMCAP','screen.c','production'),('LESS_SCREENFILL_TIME','os.c','production'),('COMSPEC','lsystem.c','other-platform'),('WINDOWID','screen.c','other-platform'),('LESSKEY','lesskey.c','not-built-companion')]:
 raw=(src/file).read_text(); match=re.search(r'(?:lgetenv|getenv)\(\s*"'+name+r'"',raw)
 cap=dict(id='env:'+name,kind='environment',name=name,source=ref(file,match.start()),disposition=disposition,case_ids=[])
 attach(cap,'CONFIG-04','spec/behavior/01-startup-options.md','Additional environment: '+name)

# Explicit grammars, helper and system surfaces not enumerable from tables.
extra=[('cli-grammar','CLI-02','01-startup-options','OPTIONS'),('startup-order','CONFIG-02','01-startup-options','Startup sequence'),('secure-mask','SEC-01','01-startup-options','Secure policy'),('file-lifecycle','IO-01','02-input-files-errors','File lifecycle'),('redirection','IO-02','02-input-files-errors','Redirection'),('stream-buffers','IO-03','02-input-files-errors','Streams'),('file-size-scan','IO-03','02-input-files-errors','Explicit scans'),('logging','IO-04','07-integrations','Logging'),('signals','TERM-02','03-terminal-events','Signals'),('fd-topology','TERM-01','03-terminal-events','File descriptors'),('mouse-reports','TERM-03','03-terminal-events','Mouse'),('paste','EDIT-01','03-terminal-events','Paste'),('text-layout','DISPLAY-02','04-layout-navigation','Text'),('unicode-tables','DISPLAY-02','04-layout-navigation','Unicode'),('search-grammar','SEARCH-01','05-search-filters','Search modifiers'),('regex-gnu','SEARCH-02','05-search-filters','GNU regex'),('filter','SEARCH-03','05-search-filters','Filtering'),('prompt-language','PROMPT-01','06-languages-state','PROMPTS'),('lesskey-language','CONFIG-01','06-languages-state','lesskey grammar'),('modelines','CONFIG-04','06-languages-state','Modelines'),('history-format','CONFIG-03','06-languages-state','History'),('messages','CONFIG-03','06-languages-state','Messages'),('lessopen','EXT-01','07-integrations','Preprocessor'),('shell-editor','EXT-02','07-integrations','Shell/editor'),('pipe-ranges','EXT-03','07-integrations','Pipe ranges'),('osc8','EXT-04','07-integrations','OSC 8'),('lessecho','EXT-04','07-integrations','lessecho'),('tags','FILE-02','07-integrations','Tags'),('follow','FOLLOW-01','07-integrations','Follow'),('more-mode','MORE-01','07-integrations','More'),('limits-performance','IO-03','02-input-files-errors','Resource contract')]
for name,parent,doc,clause in extra:
 cap=dict(id='surface:'+name,kind='dynamic-surface',disposition='production',case_ids=[],source={'file':'less.nro.VER','url':url+'less.nro.VER'})
 attach(cap,parent,'spec/behavior/'+doc+'.md',clause)
by_id={c['id']:c for c in caps}
for r in requirements:
 cap=by_id[r['capability_ids'][0]]; related=[]
 if cap['kind']=='option':
  name=cap['id'].split(':',1)[1]
  if name in ('raw-control-chars',):related+=['DISPLAY-03']
  if name in ('color','use-color','status-line','quiet','hilite-unread','hilite-target'):related+=['DISPLAY-04']
  if name in ('tabs','squeeze-blank-lines','chop-long-lines','header','LINE-NUMBERS','line-num-width','status-col-width','form-feed','wordwrap','tilde'):related+=['DISPLAY-01']
  if name=='lesskey-file':related+=['LEGACY-01']
 if cap['id']=='surface:secure-mask':related+=['SEC-02']
 r['related_parents']=related

save('spec/capabilities.json',caps); save('spec/requirements.json',requirements); save('compat/cases/options.json',cases)
save('compat/special-keys.json',{k:{'terminfo_name':keynames.get(k),'bytes_b64':base64.b64encode(v).decode() if v else None} for k,v in sorted(keys.items())})
print('capabilities',len(caps),'requirements',len(requirements),'option cases',len(cases))
