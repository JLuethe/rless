"""Native baseline inside the pinned rootfs. Called only by the outer runner."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

WORK = Path('/work')
SOURCE = WORK / 'source'
REPORT = WORK / 'report'


def run(name, command, cwd=SOURCE):
    print(name, flush=True)
    with (REPORT / (name + '.log')).open('wb') as log:
        subprocess.run(command, cwd=cwd, check=True, stdout=log, stderr=subprocess.STDOUT)


def main():
    os.umask(0o077)
    SOURCE.mkdir()
    REPORT.mkdir()
    (WORK / 'home').mkdir()
    (WORK / 'cargo').mkdir()
    (WORK / 'cargo/registry').symlink_to('/cache/registry')
    with tarfile.open('/input/source.tar') as archive:
        archive.extractall(SOURCE, filter='data')
    runtime = subprocess.check_output(['getconf','GNU_LIBC_VERSION'],text=True).strip()
    if runtime != 'glibc 2.41':
        raise ValueError('wrong runtime: ' + runtime)
    run('abi-build',['cc','-Wall','-Wextra','-Werror',str(SOURCE/'tools/safety/abi_probe.c'),'-o',str(REPORT/'abi-probe')])
    run('abi-probe',[str(REPORT/'abi-probe')])
    probe=json.loads((REPORT/'abi-probe.log').read_text())
    if probe['glibc_headers']!='2.41':
        raise ValueError('host/incorrect headers contaminated the build')
    versions={name:subprocess.check_output(command,text=True,stderr=subprocess.STDOUT)
       for name,command in [('rustc',['rustc','-vV']),('cargo',['cargo','-V']),
        ('cc',['cc','--version']),('linker',['ld','--version'])]}
    (REPORT/'environment.json').write_text(json.dumps(dict(runtime=runtime,header_probe=probe,
        versions=versions,packages=Path('/opt/rless-abi/packages.tsv').read_text()),indent=2)+'\n')
    run('native-baseline',['python3','tools/check.py','--reference','/reference/less','--stress-rounds','100','--install-check'])
    summaries=list((SOURCE/'compat/.runs').glob('verification-*/summary.json'))
    if len(summaries)!=1:
        raise ValueError('wrong baseline selection')
    summary=json.loads(summaries[0].read_text())
    if not summary['passed'] or not all(row['passed'] for row in summary['steps']):
        raise ValueError('baseline failed')
    shutil.copyfile(summaries[0],REPORT/'native-summary.json')
    run('injection-build',['cargo','build','--locked','--offline','--features','safety-injection','--bins','--target-dir',str(WORK/'injected')])
    run('injection-tests',['python3','tools/safety/test_hooks.py','--normal',str(SOURCE/'target/debug/rless'),
        '--injected',str(WORK/'injected/debug/rless')])
    run('packages',['python3','tools/package.py','--verification',str(summaries[0]),'--output-dir',str(WORK/'packages')])
    package=json.loads((REPORT/'packages.log').read_text().splitlines()[-1])
    rebuilt=WORK/'rebuild'
    rebuilt.mkdir()
    with tarfile.open(package['source_archive']) as archive:
        archive.extractall(rebuilt,filter='data')
    source=next(rebuilt.iterdir())
    run('source-tests',['cargo','test','--release','--locked','--offline'],cwd=source)
    run('source-acceptance',['python3','tools/compat/check_candidate.py','--binary',str(source/'target/release/rless'),
        '--results-dir',str(REPORT/'source-acceptance')],cwd=source)
    source_cases=json.loads((REPORT/'source-acceptance/summary.json').read_text())
    if source_cases['executed']!=809 or source_cases['passed']!=809 or source_cases['failed']:
        raise ValueError('source rebuild acceptance incomplete')
    libraries={}
    for name,binary in [('reference',Path('/reference/less')),('debug',SOURCE/'target/debug/rless'),('release',SOURCE/'target/release/rless')]:
        import re
        text=subprocess.check_output(['ldd',str(binary)],text=True)
        hashes={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in re.findall(r'(/[^\s()]+)',text) if Path(p).is_file()}
        libraries[name]=dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),ldd=text,libraries_sha256=hashes)
    (REPORT/'result.json').write_text(json.dumps(dict(status='passed',runtime_safety_verified=False,
        scope='Actual glibc 2.41 native baseline; no full safety campaign claim.',runtime=runtime,
        header_probe=probe,libraries=libraries,native_summary=summary,source_acceptance=source_cases,
        packages=package,allocation_hook_profile='NATIVE allocation hook controls passed; remaining sites unimplemented',
        safety_profile_status='ASan/LSan/Miri not-run here; native baseline/hooks do not establish full safety campaigns'),indent=2)+'\n')


if __name__=='__main__':
    main()
