"""Focused boundary lane inside the exact reference image; outer supervisor only."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

SOURCE=Path('/work/source');REPORT=Path('/work/report')


def main():
    os.umask(0o077);SOURCE.mkdir();REPORT.mkdir();Path('/work/home').mkdir();Path('/work/cargo').mkdir()
    Path('/work/cargo/registry').symlink_to('/cache/registry',target_is_directory=True)
    with tarfile.open('/input/source.tar') as archive:archive.extractall(SOURCE,filter='data')
    sys.path.insert(0,str(SOURCE/'tools/safety'))
    from glibc241.snapshot import validate
    from common import sha,write_json
    snapshot=json.loads(Path('/input/source-manifest.json').read_text());validate(snapshot,SOURCE,exact=True)
    shutil.copyfile('/input/source-manifest.json',SOURCE/'.rless-working-tree.json')
    profile=json.loads(Path('/input/options.json').read_text())['profile']
    command=['python3','tools/safety/boundary_controls.py','--profile',profile.lower(),
        '--output','/work/report/supplemental-boundaries','--reference-runtime']
    if profile=='ASAN':command+=['--tool-home','/opt/rust']
    log=REPORT/'supplemental-boundary-controls.log'
    with log.open('wb') as stream:
        process=subprocess.run(command,cwd=SOURCE,stdout=stream,stderr=subprocess.STDOUT,timeout=2400)
    summary=REPORT/'supplemental-boundaries/summary.json'
    result=dict(status='passed' if process.returncode==0 else 'failed',records=[dict(name='supplemental-boundary-controls',
        command=command,original_exit_status=process.returncode,expected_exit_status=0,log=str(log),log_sha256=sha(log))])
    if summary.is_file():result['boundary_controls']=dict(path=str(summary),sha256=sha(summary))
    validate(snapshot,SOURCE)
    write_json(REPORT/'result.json',result)
    if process.returncode:raise RuntimeError('reference supplemental lane failed; inspect '+str(log))


if __name__=='__main__':main()
