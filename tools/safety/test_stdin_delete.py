#!/usr/bin/env python3
"""Retain raw ASan/LSan PTY output for stdin :n versus :d ownership."""
import base64
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'compat'))
from run_v2 import run_pty

binary=Path(sys.argv[1]).resolve(strict=True)
output=Path(sys.argv[2]).resolve()
output.mkdir(parents=True,exist_ok=True)
manifest=os.environ.get('RLESS_SAFETY_ENV')
safety=json.loads(Path(manifest).read_text()) if manifest else {}
results=dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
             safety_environment=safety,cases={})

for name,command in [('retain',':n\n'),('delete',':d\n')]:
    try:
        with tempfile.TemporaryDirectory(prefix='rless-stdin-delete-',dir='/tmp') as directory:
            work=Path(directory)
            (work/'i').write_text('file\n'*80)
            env=dict(PATH='/usr/bin:/bin',HOME=directory,TERM='xterm-256color',
                     LC_ALL='C',LESSHISTFILE='-',**safety)
            case=dict(id='STDIN-'+name,mode='pty',size=[24,80],
                argv=['-PsREADY','-PmREADY','-PMREADY','-','i'],
                stream_b64=base64.b64encode(b'stdin\n'*80).decode(),
                events=[{'await':['READY'],'snapshot':True},{'close_stream':True},
                        {'send':command,'await':['READY'],'snapshot':True},{'send':'q'}],timeout=5)
            raw=bytearray()
            original_read=os.read
            def capture(fd,count):
                content=original_read(fd,count)
                raw.extend(content)
                return content
            os.read=capture
            try:
                observed=run_pty([str(binary),*case['argv']],env,work,case,
                                 lambda data:data.replace(directory.encode(),b'{ROOT}'))
            finally:
                os.read=original_read
            (output/(name+'.raw')).write_bytes(raw)
            results['cases'][name]=dict(status=observed['status'],
                termios_restored=observed['termios_restored'],
                screens=[checkpoint['rows'][0].strip() for checkpoint in observed['checkpoints']],
                raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),
                leak_report=b'ERROR: LeakSanitizer: detected memory leaks' in raw)
    except Exception as error:
        results['cases'][name]=dict(error=f'{type(error).__name__}: {error}')
    (output/'results.json').write_text(json.dumps(results,indent=2)+'\n')
retain=results['cases']['retain']; delete=results['cases']['delete']
results['oracle']=dict(passed=(retain.get('status')==0 and retain.get('termios_restored') is True
    and retain.get('leak_report') is False and delete.get('status')==0
    and delete.get('termios_restored') is True and delete.get('leak_report') is False
    and retain.get('screens')==delete.get('screens')))
(output/'results.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results['oracle']))
raise SystemExit(0 if results['oracle']['passed'] else 1)
