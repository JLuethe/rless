#!/usr/bin/env python3
"""Capture a pinned rootless image and rebuild/test committed source in glibc 2.41."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

sys.path.insert(0,str(Path(__file__).resolve().parents[2]))
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from common import ROOT, PINS, git, sha, utc, write_json
from snapshot import archive as snapshot_archive, validate as validate_snapshot
from release import commit_manifest, reference_policy, validate_reference_hashes


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--image',default='localhost/rless-glibc241:20261020')
    parser.add_argument('--reference-dir',type=Path,required=True)
    parser.add_argument('--rust-toolchain',type=Path,required=True)
    parser.add_argument('--registry-cache',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--commit',default='HEAD')
    parser.add_argument('--source-manifest',type=Path,
                        help='Explicit frozen working-tree manifest; otherwise only --commit is archived')
    args=parser.parse_args()
    os.umask(0o077)
    out=args.output.resolve();out.mkdir(parents=True,exist_ok=False)
    inputs=out/'input';inputs.mkdir();work=out/'work';work.mkdir()
    image=json.loads(subprocess.check_output(['podman','image','inspect',args.image],text=True))[0]
    image_id=image['Id']
    lock=json.loads((ROOT/'docs/verification/glibc241-environment.json').read_text())
    if (image_id.removeprefix('sha256:') != lock['image_id'].removeprefix('sha256:')
            or image.get('RootFS') != lock['rootfs']
            or image.get('Architecture') != 'amd64'
            or sha(ROOT/'tools/safety/glibc241/Containerfile') != lock['containerfile_sha256']):
        raise ValueError('reference environment differs from immutable lock')
    snapshot=None
    if args.source_manifest:
        if args.commit!='HEAD':
            parser.error('--source-manifest and a nondefault --commit are mutually exclusive')
        snapshot=validate_snapshot(json.loads(args.source_manifest.read_text()))
        source_commit=snapshot['base_commit']
        snapshot_archive(snapshot,inputs/'source.tar')
        write_json(inputs/'source-manifest.json',snapshot)
    else:
        source_commit=git('rev-parse',args.commit).decode().strip()
        (inputs/'source.tar').write_bytes(git('archive',source_commit))
        write_json(inputs/'release-source-manifest.json',commit_manifest(ROOT,source_commit))
    reference=out/'reference';reference.mkdir()
    for name in ('less','lessecho','less-osc8-open.sh'):
        source=args.reference_dir.resolve()/name
        if source.is_file():shutil.copy2(source,reference/name)
    if not all((reference/name).is_file() for name in ('less','lessecho','less-osc8-open.sh')):
        raise ValueError('pinned reference pager, companion and helper are required')
    record=dict(schema_version=1,procedure='REFERENCE-GLIBC-2.41-NATIVE-v1',tool_pins=PINS,
        environment_lock_sha256=sha(ROOT/'docs/verification/glibc241-environment.json'),started_at_utc=utc(),status='not-verified',source_commit=source_commit,
        source_tar_sha256=sha(inputs/'source.tar'),image_id=image_id,image_rootfs=image.get('RootFS'),
        containerfile_sha256=sha(ROOT/'tools/safety/glibc241/Containerfile'),
        references={p.name:sha(p) for p in reference.iterdir()},runtime_safety_verified=False)
    validate_reference_hashes(record['references'], ROOT)
    record['reference_policy']=reference_policy(ROOT)
    record['source_kind']='explicit-working-tree-snapshot' if snapshot else 'committed-source'
    if snapshot:
        record.update(source_manifest=snapshot,source_manifest_sha256=sha(inputs/'source-manifest.json'))
    toolchain=args.rust_toolchain.resolve()
    tool_files=[toolchain/'bin/rustc',toolchain/'bin/cargo']+sorted((toolchain/'lib').glob('*.so*'))
    record['tool_files_sha256']={str(path):sha(path) for path in tool_files}
    if not snapshot:
        record['release_source_manifest_sha256']=sha(inputs/'release-source-manifest.json')
    # Extract package payload/version/signed-metadata manifests from the exact image
    # ID. Every rebuild can compare these, even if image creation timestamps differ.
    for name in ('packages.tsv','packages.sha256','apt-metadata.sha256','runtime.txt','python-wheels.sha256','python-packages.txt'):
        value=subprocess.check_output(['podman','run','--rm','--network=none',image_id,'cat','/opt/rless-abi/'+name])
        (out/name).write_bytes(value)
    record['environment_manifests']={p.name:sha(p) for p in out.iterdir() if p.is_file()}
    write_json(out/'summary.json',record)
    command=['podman','run','--rm','--network=none','--read-only','--cap-drop=all',
        '--userns=keep-id','--user',str(os.getuid())+':'+str(os.getgid()),'--tmpfs','/tmp:rw,nosuid,nodev',
        '-v',str(inputs)+':/input:ro','-v',str(work)+':/work:rw','-v',str(reference)+':/reference:ro',
        '-v',str(args.rust_toolchain.resolve())+':/opt/rust:ro',
        '-v',str(args.registry_cache.resolve())+':/cache/registry:ro',
        '--env','PATH=/opt/rless-python/bin:/opt/rust/bin:/usr/bin:/bin','--env','CARGO_HOME=/work/cargo',
        '--env','CARGO_NET_OFFLINE=true','--env','HOME=/work/home',
        '--env','PYTHONDONTWRITEBYTECODE=1',image_id,
        'python3','-c',"import tarfile; t=tarfile.open('/input/source.tar'); s=t.extractfile('tools/safety/glibc241/inside.py').read(); exec(compile(s,'inside.py','exec'), {'__name__':'__main__'})"]
    record['command']=command
    with (out/'container.log').open('wb') as log:
        result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
    record['original_exit_status']=result.returncode
    result_path=work/'report/result.json'
    if result.returncode==0 and result_path.is_file():
        inner=json.loads(result_path.read_text())
        if inner['status']=='passed' and inner['runtime']=='glibc 2.41':
            record.update(status='passed',result=inner,inner_report_sha256=sha(result_path))
    record.update(finished_at_utc=utc(),container_log_sha256=sha(out/'container.log'))
    write_json(out/'summary.json',record)
    print('glibc 2.41 baseline: '+record['status']+'; '+str(out/'summary.json'))
    return 0 if record['status']=='passed' else 1


if __name__=='__main__':
    raise SystemExit(main())
