#!/usr/bin/env python3
"""Regression for first-buffer ENOMEM and existing-buffer LRU fallback.

Usage: test_first_buffer.py LINKED_BINARY OUTPUT SHIM_OBJECT
Link native/first_buffer_fault.c with --wrap=calloc into the tested binary.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

binary=Path(sys.argv[1]).resolve(strict=True)
output=Path(sys.argv[2]).resolve()
output.mkdir(parents=True,exist_ok=True)
symbols=subprocess.check_output(['nm','-S','-C',str(binary)],text=True)
matches=re.findall(r'^([0-9a-f]+) ([0-9a-f]+) [tT] rless::ch::ch_addbuf$',symbols,re.M)
if len(matches)!=1: raise ValueError('ambiguous ch_addbuf symbol')
start,size=(int(x,16) for x in matches[0]); end=start+size
relocations=subprocess.check_output(['readelf','-rW',str(binary)],text=True)
if '__wrap_calloc' not in symbols: raise ValueError('wrapper not linked')
result=dict(binary=str(binary),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
            ch_addbuf=dict(start=hex(start),end=hex(end),size=size),
            wrapper_sha256=hashlib.sha256(Path(sys.argv[3]).read_bytes()).hexdigest(),
            cases={})
for name in ('control','miss','hit','reuse'):
    with tempfile.TemporaryDirectory(prefix='rless-firstbuf-',dir='/tmp') as directory:
        work=Path(directory)
        payload=b'line\n'*(4000 if name=='reuse' else 20)
        (work/'i').write_bytes(payload)
        (work/'hit').write_bytes(b'')
        env=dict(PATH='/usr/bin:/bin',HOME=directory,LC_ALL='C',TERM='dumb',
                 LESSHISTFILE='-',RUST_BACKTRACE='1')
        if os.environ.get('RLESS_SAFETY_ENV'):
            env.update(json.loads(Path(os.environ['RLESS_SAFETY_ENV']).read_text()))
        if name!='control':
            env.update(RLESS_BUFFER_TRACE=str(work/'hit'),
                RLESS_BUFFER_START=hex(0 if name=='miss' else start),
                RLESS_BUFFER_END=hex(1 if name=='miss' else end),
                RLESS_BUFFER_SKIP='1' if name=='reuse' else '0')
        try:
            process=subprocess.run([str(binary),'i'],cwd=work,env=env,
                                   stdin=subprocess.DEVNULL,capture_output=True,timeout=5)
            observed=dict(status=process.returncode,stdout_sha256=hashlib.sha256(process.stdout).hexdigest(),
                          stdout_bytes=len(process.stdout),stderr_sha256=hashlib.sha256(process.stderr).hexdigest(),
                          trace=(work/'hit').read_text(),trace_sha256=hashlib.sha256((work/'hit').read_bytes()).hexdigest())
            (output/(name+'.stdout')).write_bytes(process.stdout)
            (output/(name+'.stderr')).write_bytes(process.stderr)
        except Exception as error:
            observed=dict(error=f'{type(error).__name__}: {error}')
        result['cases'][name]=observed
        (output/'results.json').write_text(json.dumps(result,indent=2)+'\n')
payload=b'line\n'*20
control=result['cases']['control']; miss=result['cases']['miss']; hit=result['cases']['hit']
reuse=result['cases']['reuse']
def selected(case):
    match=re.fullmatch(r'ch_addbuf calloc\(1,8240\) caller_offset=([0-9a-f]+)\n',case.get('trace',''))
    return match is not None and start<=int(match[1],16)<end
stderr=(output/'hit.stderr').read_bytes() if (output/'hit.stderr').exists() else b''
result['oracle']=dict(passed=(control.get('status')==0 and miss.get('status')==0
    and control.get('stdout_sha256')==hashlib.sha256(payload).hexdigest()
    and miss.get('stdout_sha256')==hashlib.sha256(payload).hexdigest()
    and miss.get('trace')=='' and hit.get('status')==1 and hit.get('stdout_bytes')==0
    and selected(hit) and stderr.count(b'cannot allocate input buffer')==1
    and b'panicked' not in stderr and b'AddressSanitizer' not in stderr
    and reuse.get('status')==0 and selected(reuse)
    and reuse.get('stdout_sha256')==hashlib.sha256(b'line\n'*4000).hexdigest()),
    expected='control/miss copy exact bytes; first ENOMEM diagnoses once/status1; later ENOMEM reuses existing buffer without lost bytes')
(output/'results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result['oracle']))
raise SystemExit(0 if result['oracle']['passed'] else 1)
