#!/usr/bin/env python3
"""Run and rederive focused supplemental controls in the locked reference ABI."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile

from common import ROOT,PINS,fingerprint,git,sha,write_json
from glibc241.snapshot import manifest,archive,validate,source_names
from glibc241.boundary_receipts import rederive

PROCEDURE='REFERENCE-GLIBC-2.41-BOUNDARIES-v1'


def require(value,message):
    if not value:raise ValueError(message)


def validate_external_report(path,root=ROOT):
    path,root=Path(path).resolve(),Path(root).resolve();out=path.parent
    outer=json.loads(path.read_text())
    require(outer.get('report_kind')=='reference-boundary-controls' and outer.get('procedure')==PROCEDURE,'unknown reference boundary procedure')
    require(outer['source_commit']==git('rev-parse','HEAD',root=root).decode().strip()
            and outer['input_fingerprints']==fingerprint(root),'stale reference boundary source')
    snapshot=validate(outer['source_manifest'],root)
    require(set(source_names(root))==set(snapshot['files']),'reference boundary source membership changed')
    require(sha(out/'input/source-manifest.json')==outer['source_manifest_sha256']
            and json.loads((out/'input/source-manifest.json').read_text())==snapshot,'reference boundary snapshot changed')
    require(sha(out/'input/source.tar')==outer['source_tar_sha256'],'reference boundary archive changed')
    with tarfile.open(out/'input/source.tar') as tar:
        members=tar.getmembers()
        require(len(members)==len(snapshot['files']) and {m.name for m in members}==set(snapshot['files']),'reference boundary archive membership changed')
        for item in members:
            expected=snapshot['files'][item.name]
            require(item.isfile() and item.mode==expected['mode']
                    and hashlib.sha256(tar.extractfile(item).read()).hexdigest()==expected['sha256'],'reference boundary archive content changed')
    validate(snapshot,out/'work/source')
    require(sha(out/'work/report/result.json')==outer['inner_report_sha256'],'reference boundary result changed')
    inner=json.loads((out/'work/report/result.json').read_text())
    require(inner.get('status')=='passed' and outer['original_exit_status']==0,'reference boundary command did not pass')
    for record in inner['records']:
        relative=Path(record['log']).relative_to('/work')
        require(record['original_exit_status']==record['expected_exit_status']==0 and sha(out/'work'/relative)==record['log_sha256'],'reference boundary execution log changed')
    require(outer['tool_pins']==PINS,'reference boundary tool pins changed')
    for name,digest in outer['tool_files_sha256'].items():require(Path(name).is_file() and sha(name)==digest,'reference boundary compiler changed')
    records=rederive(outer,inner,out/'work',outer['profile'],root)
    return dict(schema_version=1,source_commit=outer['source_commit'],input_fingerprints=fingerprint(root),
        tool_pins=PINS,tool_files_sha256=outer['tool_files_sha256'],records=records,status='passed',runtime_safety_verified=False)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile',choices=('native','asan'))
    parser.add_argument('--image',default='localhost/rless-glibc241:20261020')
    parser.add_argument('--rust-toolchain',type=Path)
    parser.add_argument('--registry-cache',type=Path)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--verify',action='store_true')
    args=parser.parse_args();os.umask(0o077)
    if args.verify:
        validate_external_report(args.output);print('Reference boundary controls rederived.');return 0
    if args.profile is None or args.rust_toolchain is None or args.registry_cache is None:parser.error('profile, rust-toolchain and registry-cache required')
    out=args.output.resolve();out.mkdir(parents=True,exist_ok=False);inputs=out/'input';inputs.mkdir();work=out/'work';work.mkdir()
    snapshot=manifest(ROOT);archive(snapshot,inputs/'source.tar');write_json(inputs/'source-manifest.json',snapshot)
    write_json(inputs/'options.json',dict(profile=args.profile.upper()))
    lock=json.loads((ROOT/'docs/verification/glibc241-environment.json').read_text())
    image=json.loads(subprocess.check_output(['podman','image','inspect',args.image],text=True,timeout=30))[0]
    require(image['Id'].removeprefix('sha256:')==lock['image_id'].removeprefix('sha256:')
            and image['RootFS']==lock['rootfs'] and image['Architecture']=='amd64','reference boundary image differs from lock')
    toolchain=args.rust_toolchain.resolve(strict=True);registry=args.registry_cache.resolve(strict=True)
    tool_files=[toolchain/'bin/rustc',toolchain/'bin/cargo']+sorted((toolchain/'lib').glob('*.so*'))
    tool_files+=sorted((toolchain/'lib/rustlib'/PINS['target']/'lib').glob('librustc-nightly_rt.asan.a'))
    command=['podman','run','--rm','--network=none','--read-only','--cap-drop=all', '--ulimit', 'core=0:0','--userns=keep-id',
        '--user',str(os.getuid())+':'+str(os.getgid()),'--tmpfs','/tmp:rw,nosuid,nodev',
        '-v',str(inputs)+':/input:ro','-v',str(work)+':/work:rw','-v',str(toolchain)+':/opt/rust:ro',
        '-v',str(registry)+':/cache/registry:ro','--env','PATH=/opt/rless-python/bin:/opt/rust/bin:/usr/bin:/bin',
        '--env','CARGO_HOME=/work/cargo','--env','CARGO_NET_OFFLINE=true','--env','HOME=/work/home',
        '--env','PYTHONDONTWRITEBYTECODE=1',image['Id'],'python3','-c',
        "import tarfile;t=tarfile.open('/input/source.tar');s=t.extractfile('tools/safety/glibc241/boundary_inside.py').read();exec(compile(s,'boundary_inside.py','exec'),{'__name__':'__main__'})"]
    outer=dict(schema_version=1,report_kind='reference-boundary-controls',procedure=PROCEDURE,profile=args.profile.upper(),
        source_commit=snapshot['base_commit'],input_fingerprints=fingerprint(),source_manifest=snapshot,
        source_manifest_sha256=sha(inputs/'source-manifest.json'),source_tar_sha256=sha(inputs/'source.tar'),
        image_id=image['Id'],image_rootfs=image['RootFS'],tool_pins=PINS,
        tool_files_sha256={str(p):sha(p) for p in tool_files},command=command,status='not-verified',runtime_safety_verified=False)
    write_json(out/'summary.json',outer)
    with (out/'container.log').open('wb') as log:
        process=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,timeout=3000)
    outer['original_exit_status']=process.returncode
    result=out/'work/report/result.json'
    if result.is_file():outer['inner_report_sha256']=sha(result)
    outer['status']='passed' if process.returncode==0 else 'failed';write_json(out/'summary.json',outer)
    require(process.returncode==0,'reference boundary container failed; inspect '+str(out/'container.log'))
    validate_external_report(out/'summary.json')
    print(json.dumps(dict(passed=True,profile=args.profile,report=str(out/'summary.json'))))
    return 0


if __name__=='__main__':raise SystemExit(main())
