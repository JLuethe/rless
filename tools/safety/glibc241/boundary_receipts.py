"""Re-derive retained boundary receipts in their exact pinned reference runtime.

Evidence is mounted read-only at its original logical /work path. Only a private
compiler cache and normalized output are writable; original evidence is unchanged.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

from common import PINS, ROOT, fingerprint, sha


def require(value,message):
    if not value:raise ValueError(message)


def checked_mounts(command, tool_files):
    values={}
    for index,value in enumerate(command[:-1]):
        if value=='-v':
            source,target,mode=command[index+1].rsplit(':',2)
            values[target]=(Path(source).resolve(),mode)
    require('/opt/rust' in values and '/cache/registry' in values,'reference tool/cache mounts missing')
    toolchain=values['/opt/rust'][0];registry=values['/cache/registry'][0]
    for name in ('rustc','cargo'):
        path=toolchain/'bin'/name
        require(path.is_file() and str(path) in tool_files and sha(path)==tool_files[str(path)],'reference compiler mount differs from validated tool bytes')
    require(values['/opt/rust'][1]=='ro' and values['/cache/registry'][1]=='ro','reference compiler/cache mounts are not read-only')
    return toolchain,registry


def map_work(value,work):
    if isinstance(value,dict):return {map_work(k,work):map_work(v,work) for k,v in value.items()}
    if isinstance(value,list):return [map_work(x,work) for x in value]
    if isinstance(value,str):return re.sub(r'/work(?=/|$)',lambda _:str(work),value)
    return value


def rederive(outer,inner,work,profile,root=ROOT):
    root,work=Path(root).resolve(),Path(work).resolve()
    lock_path=root/'docs/verification/glibc241-environment.json';lock=json.loads(lock_path.read_text())
    require(outer['image_id'].removeprefix('sha256:')==lock['image_id'].removeprefix('sha256:')
            and outer['image_rootfs']==lock['rootfs'],'boundary reference image differs from maintained lock')
    actual=json.loads(subprocess.check_output(['podman','image','inspect',outer['image_id']],text=True,timeout=30))[0]
    require(actual['Id'].removeprefix('sha256:')==lock['image_id'].removeprefix('sha256:')
            and actual['RootFS']==lock['rootfs'] and actual['Architecture']=='amd64','actual boundary verifier image differs from lock')
    require(sha(root/'tools/safety/glibc241/Containerfile')==lock['containerfile_sha256'],'reference image procedure changed')
    reference=inner.get('boundary_controls')
    require(isinstance(reference,dict) and set(reference)=={'path','sha256'},'reference lane lacks retained supplemental boundary receipt')
    expected='/work/report/supplemental-boundaries/summary.json'
    require(reference['path']==expected,'unexpected reference boundary receipt path')
    summary=work/'report/supplemental-boundaries/summary.json'
    require(summary.is_file() and sha(summary)==reference['sha256'],'reference boundary receipt changed')
    child=json.loads(summary.read_text())
    require(child['profile']==profile and child['abi_profile']=='REFERENCE-GLIBC-2.41'
            and child['source_commit']==outer['source_commit'] and child['input_fingerprints']==fingerprint(root),'reference boundary source/profile mismatch')
    records=[x for x in inner['records'] if x['name']=='supplemental-boundary-controls']
    require(len(records)==1,'reference lane must execute supplemental controls exactly once')
    expected_command=['python3','tools/safety/boundary_controls.py','--profile',profile.lower(),
        '--output','/work/report/supplemental-boundaries','--reference-runtime']
    if profile=='ASAN':expected_command+=['--tool-home','/opt/rust']
    require(records[0]['command']==expected_command and records[0]['original_exit_status']==0,'reference supplemental command did not execute as specified')
    toolchain,registry=checked_mounts(outer['command'],outer['tool_files_sha256'])
    require(not toolchain.is_relative_to(work.parent) and not toolchain.is_relative_to(root),'reference compiler cannot be supplied by retained evidence')
    key=hashlib.sha256(json.dumps(dict(image=outer['image_id'],source=fingerprint(root),profile=profile,
        tools=outer['tool_files_sha256'],root=str(root)),sort_keys=True).encode()).hexdigest()
    cache=root/'compat/.runs/reference-boundary-verifier'/key
    cache.mkdir(parents=True,exist_ok=True,mode=0o700)
    require(not cache.is_symlink() and cache.stat().st_uid==os.getuid() and not cache.stat().st_mode&0o022,'unsafe reference verifier cache')
    cargo=cache/'cargo';cargo.mkdir(exist_ok=True,mode=0o700)
    link=cargo/'registry'
    if not link.exists() and not link.is_symlink():link.symlink_to('/cache/registry',target_is_directory=True)
    require(link.is_symlink() and os.readlink(link)=='/cache/registry','reference verifier registry mount changed')
    name='normalized-'+str(time.time_ns())+'.json';normalized=cache/name
    command=['podman','run','--rm','--network=none','--read-only','--cap-drop=all',
        '--userns=keep-id','--user',str(os.getuid())+':'+str(os.getgid()),'--tmpfs','/tmp:rw,nosuid,nodev',
        '-v',str(work)+':/work:ro','-v',str(cache)+':/verify-cache:rw','-v',str(cargo)+':/work/cargo:rw',
        '-v',str(toolchain)+':/opt/rust:ro','-v',str(registry)+':/cache/registry:ro',
        '--env','PATH=/opt/rless-python/bin:/opt/rust/bin:/usr/bin:/bin','--env','CARGO_HOME=/work/cargo',
        '--env','CARGO_NET_OFFLINE=true','--env','HOME=/work/home','--env','PYTHONDONTWRITEBYTECODE=1',
        '--env','RLESS_MUTANT_CACHE=/verify-cache/mutants',outer['image_id'],
        'python3','/work/source/tools/safety/boundary_controls.py','--verify','--output',expected,
        '--reference-runtime','--normalized-output','/verify-cache/'+name]
    if profile=='ASAN':command+=['--tool-home','/opt/rust']
    with (cache/(name+'.log')).open('wb') as log:
        result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,timeout=2400)
    require(result.returncode==0 and normalized.is_file(),'pinned reference boundary rederivation failed; see '+str(cache/(name+'.log')))
    result=json.loads(normalized.read_text())
    require(result['source_commit']==outer['source_commit'] and result['input_fingerprints']==fingerprint(root),'normalized reference boundary source changed')
    from boundary_controls import controls
    require({x['name'] for x in result['records']}==set(controls(profile)),'normalized reference controls incomplete')
    for item in result['records']:
        require(item['profile']==profile and item['oracle_metadata']['abi_profile']=='REFERENCE-GLIBC-2.41'
                and item['_validated_m0_controls']==[item['name']],'normalized reference control identity changed')
        item['name']='reference-boundary-'+item['name']
    return map_work(result['records'],work)
