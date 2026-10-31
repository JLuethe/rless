"""Rebuild exact maintained mutations; submitted build records are not authority.

The private local compiler cache is not a portable receipt and never supplies gate
credit. It is keyed by actual source/compiler/flags/link-object bytes, and its
outputs are checked before use. A fresh verifier machine rebuilds it independently.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess

from common import PINS, ROOT, environment, sha, write_json
from native_alloc import CFLAGS, RUSTFLAGS as ALLOC_FLAGS


def require(value,message):
    if not value:raise ValueError(message)


def rederive(mutation, profile, objects, *, root=ROOT, tool_home=None):
    source=Path(mutation['source']).resolve();kind=mutation['kind']
    if profile=='ASAN':
        tool_home=tool_home or os.environ.get('RLESS_SAFETY_TOOL_HOME') or os.environ.get('RLESS_INVENTORY_TOOL_HOME')
        require(tool_home is not None,'ASAN mutation verification requires trusted --tool-home or RLESS_SAFETY_TOOL_HOME')
    env=environment(profile=='ASAN',tool_home)
    flags=((PINS['asan_rustflags']+' ') if profile=='ASAN' else '')+ALLOC_FLAGS
    env['RUSTFLAGS']=flags
    compiler=shutil.which('rustc',path=env['PATH']);cargo=shutil.which('cargo',path=env['PATH']);cc=shutil.which('cc',path=env['PATH'])
    require(all((compiler,cargo,cc)),'trusted mutation compiler tools missing')
    actual=subprocess.check_output([compiler,'-vV'],env=env,text=True,timeout=30)
    require((PINS['rustc_commit'] in actual) if profile=='ASAN' else ('rustc '+PINS['normal_rust_release']) in actual,'mutation compiler differs from maintained pin')
    # rustc may be a rustup proxy; bind both proxy bytes and the selected compiler identity.
    input_key=dict(kind=kind,source=str(source),source_files=mutation['source_files_sha256'],profile=profile,
                   rustflags=flags,compiler=sha(compiler),compiler_identity=actual,cargo=sha(cargo),cc=sha(cc))
    linked='first-buffer' if kind=='firstbuf' else 'live-heap' if kind=='heap-leak' else None
    if linked:
        input_key['object']=sha(objects[linked]);input_key['object_source']=sha(root/'tools/safety/native'/('first_buffer_fault.c' if kind=='firstbuf' else 'live_heap.c'))
    key=hashlib.sha256(json.dumps(input_key,sort_keys=True).encode()).hexdigest()
    cache_root=Path(os.environ.get('RLESS_MUTANT_CACHE',Path(root)/'compat/.runs/rederived-mutations'));cache_root.mkdir(parents=True,exist_ok=True,mode=0o700)
    require(not cache_root.is_symlink() and cache_root.stat().st_uid==os.getuid() and not cache_root.stat().st_mode&0o022,'unsafe private mutation cache')
    cache=cache_root/key;cache.mkdir(exist_ok=True,mode=0o700)
    lock=cache/'lock'
    with lock.open('a') as locked:
        fcntl.flock(locked,fcntl.LOCK_EX)
        manifest=cache/'proof.json'
        binary=cache/'target'/(PINS['target'] if profile=='ASAN' else '')/'debug/rless'
        if manifest.is_file():
            saved=json.loads(manifest.read_text())
            require(saved['inputs']==input_key and binary.is_file() and sha(binary)==saved['binary_sha256'],'private independent rebuild cache changed')
            return saved['binary_sha256']
        links=[]
        with (cache/'rebuild.log').open('wb') as log:
            if linked:
                current=root/'tools/safety/native'/('first_buffer_fault.c' if kind=='firstbuf' else 'live_heap.c')
                obj=cache/'checked-object.o'
                subprocess.run([cc,*CFLAGS,'-c',str(current),'-o',str(obj)],env=env,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=120)
                require(sha(obj)==sha(objects[linked]),'submitted mutation object differs from independently compiled maintained C source')
                links=[objects[linked],'-Wl,--wrap=calloc','-ldl'] if kind=='firstbuf' else [objects[linked],'-Wl,--wrap=read','-Wl,--wrap=ppoll']
            env['CARGO_TARGET_DIR']=str(cache/'target')
            command=[cargo,'rustc','--locked','--offline','--bin','rless',*(['--target',PINS['target']] if profile=='ASAN' else []),
                     *(['--features','safety-injection'] if kind=='heap-leak' else []),'--',*['-Clink-arg='+str(x) for x in links]]
            subprocess.run(command,cwd=source,env=env,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=600)
        proof=dict(inputs=input_key,binary_sha256=sha(binary))
        write_json(manifest,proof)
        return proof['binary_sha256']


def validate(mutation, profile, objects, binary, *, root=ROOT,tool_home=None):
    expected=rederive(mutation,profile,objects,root=root,tool_home=tool_home)
    require(sha(binary)==expected,'mutant binary differs from independently rebuilt exact mutated source')
    return expected
