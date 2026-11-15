"""Native baseline inside the pinned rootfs. Called only by the outer runner."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

WORK = Path('/work')
SOURCE = WORK / 'source'
REPORT = WORK / 'report'
RECORDS = []


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def run(name, command, cwd=SOURCE, env=None, timeout=None):
    print(name, flush=True)
    with (REPORT / (name + '.log')).open('wb') as log:
        result=subprocess.run(command, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT,timeout=timeout)
    RECORDS.append(dict(name=name,command=command,original_exit_status=result.returncode,
        expected_exit_status=0,log=str(REPORT/(name+'.log')),log_sha256=sha(REPORT/(name+'.log')),
        build_flags={key:env[key] for key in ('RUSTFLAGS',) if env and key in env}))
    if result.returncode:
        raise RuntimeError('native command failed: '+name)


def main():
    os.umask(0o077)
    SOURCE.mkdir()
    REPORT.mkdir()
    (WORK / 'home').mkdir()
    (WORK / 'cargo').mkdir()
    (WORK / 'cargo/registry').symlink_to('/cache/registry')
    with tarfile.open('/input/source.tar') as archive:
        archive.extractall(SOURCE, filter='data')
    snapshot=None
    if Path('/input/source-manifest.json').is_file():
        sys.path.insert(0,str(SOURCE/'tools/safety/glibc241'))
        from snapshot import validate
        snapshot=json.loads(Path('/input/source-manifest.json').read_text())
        validate(snapshot,SOURCE,exact=True)
        shutil.copyfile('/input/source-manifest.json',SOURCE/'.rless-working-tree.json')
    else:
        sys.path.insert(0,str(SOURCE/'tools'))
        from release import write_export_manifest
        write_export_manifest(SOURCE,json.loads(Path('/input/release-source-manifest.json').read_text()))
    sys.path.insert(0,str(SOURCE/'tools'))
    from release import capture_reference_bundle
    capture_reference_bundle('/reference/less', SOURCE)
    import importlib.metadata
    import pyte
    python_dependencies={}
    for line in (SOURCE/'tools/compat/requirements.txt').read_text().splitlines():
        if not line.strip() or line.lstrip().startswith('#'):
            continue
        name,expected=line.strip().split('==')
        actual=importlib.metadata.version(name)
        if actual!=expected:
            raise ValueError('observer dependency mismatch: '+name+' '+actual+' != '+expected)
        python_dependencies[name]=actual
    if 'blink' not in pyte.screens.Char._fields:
        raise ValueError('terminal observer schema differs from the frozen profile')
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
        identity=dict(uid=os.getuid(),euid=os.geteuid(),gid=os.getgid()),
        versions=versions,python_dependencies=python_dependencies,
        packages=Path('/opt/rless-abi/packages.tsv').read_text()),indent=2)+'\n')
    (REPORT/'terminfo.txt').write_bytes(subprocess.check_output(['infocmp','-x','xterm-256color']))
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
    shims={}
    for name,filename in [('cleanup','cleanup_eintr.c'),('short-write','short_write.c'),('native-io','io_fault.c')]:
        shims[name]=WORK/(name+'.so')
        run('build-'+name+'-counterpart',['cc','-Wall','-Wextra','-Werror','-shared','-fPIC',
            str(SOURCE/'tools/safety/native'/filename),'-ldl','-o',str(shims[name])])
    sys.path.insert(0,str(SOURCE/'tools/safety'))
    from native_alloc import RUSTFLAGS as ALLOC_RUSTFLAGS
    alloc_object=WORK/'alloc-fault.o'
    alloc_binding=REPORT/'alloc-binding.json'
    alloc_target=WORK/'alloc-counterpart'
    alloc_binary=alloc_target/'debug/rless'
    run('build-allocation-object',['python3','tools/safety/native_alloc.py','--build-object',str(alloc_object),
        '--output',str(REPORT/'alloc-object.json')])
    alloc_environment=dict(os.environ,RUSTFLAGS=ALLOC_RUSTFLAGS,CARGO_TARGET_DIR=str(alloc_target))
    run('build-allocation-counterpart',['cargo','rustc','--locked','--offline','--bin','rless','--',
        '-Clink-arg='+str(alloc_object),'-Clink-arg=-Wl,--wrap=calloc', '-Clink-arg=-Wl,--wrap=strdup'],env=alloc_environment)
    run('bind-allocation-counterpart',['python3','tools/safety/native_alloc.py','--binary',str(alloc_binary),
        '--profile','native','--output',str(alloc_binding)])
    test_binaries={str(path):sha(path) for path in [SOURCE/'target/debug/rless',WORK/'injected/debug/rless',alloc_binary,alloc_object,*shims.values()]}
    run('fault-path-tests',['python3','tools/safety/test_fault_paths.py',
        '--normal',str(SOURCE/'target/debug/rless'),'--injected',str(WORK/'injected/debug/rless'),
        '--native-shim',str(shims['cleanup']),'--short-write-shim',str(shims['short-write']),
        '--native-io-shim',str(shims['native-io']),'--reference','/reference/less',
        '--native-alloc-binding',str(alloc_binding),
        '--repetitions','100','--oracle-report',str(REPORT/'fault-oracles.json')])
    package=None
    source_cases=None
    if snapshot is None:
        run('packages',['python3','tools/package.py','--verification',str(summaries[0]),'--output-dir',str(WORK/'packages')])
        package=json.loads((REPORT/'packages.log').read_text().splitlines()[-1])
        rebuilt=WORK/'rebuild'
        rebuilt.mkdir()
        with tarfile.open(package['source_archive']) as archive:
            archive.extractall(rebuilt,filter='data')
        source=next(rebuilt.iterdir())
        run('source-tests',['cargo','test','--release','--locked','--offline'],cwd=source)
        run('source-build',['cargo','build','--release','--bins','--locked','--offline'],cwd=source)
        run('source-acceptance',['python3','tools/compat/check_candidate.py','--binary',str(source/'target/release/rless'),
            '--results-dir',str(REPORT/'source-acceptance')],cwd=source)
        source_cases=json.loads((REPORT/'source-acceptance/summary.json').read_text())
        if source_cases['executed']!=809 or source_cases['passed']!=809 or source_cases['failed']:
            raise ValueError('source rebuild acceptance incomplete')
    libraries={}
    native_objects=WORK/'native-objects'
    native_objects.mkdir()
    objects={}
    for name,binary in [('reference',Path('/reference/less')),('debug',SOURCE/'target/debug/rless'),('release',SOURCE/'target/release/rless')]:
        import re
        text=subprocess.check_output(['ldd',str(binary)],text=True)
        hashes={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in re.findall(r'(/[^\s()]+)',text) if Path(p).is_file()}
        for native_path,digest in hashes.items():
            retained=native_objects/digest
            if not retained.exists():
                shutil.copyfile(native_path,retained)
            objects[native_path]=dict(path=str(retained),sha256=digest)
        libraries[name]=dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),ldd=text,libraries_sha256=hashes)
    run('supplemental-boundary-controls',['python3','tools/safety/boundary_controls.py',
        '--profile','native','--output','/work/report/supplemental-boundaries','--reference-runtime'],timeout=2400)
    boundary_summary=REPORT/'supplemental-boundaries/summary.json'
    if snapshot:
        validate(snapshot,SOURCE)
    evidence_files={str(path):sha(path) for path in REPORT.rglob('*') if path.is_file()}
    for name,digest in summary['evidence_files'].items():
        evidence_files[str(summaries[0].parent/name)]=digest
    if package:
        for key in ('archive','source_archive'):
            evidence_files[package[key]]=sha(package[key])
    (REPORT/'result.json').write_text(json.dumps(dict(status='passed',runtime_safety_verified=False,
        procedure='REFERENCE-GLIBC-2.41-NATIVE-v1',records=RECORDS,
        evidence_files=evidence_files,objects=objects,test_binaries=test_binaries,
        boundary_controls=dict(path=str(boundary_summary),sha256=sha(boundary_summary)),
        allocation_binding=dict(path=str(alloc_binding),sha256=sha(alloc_binding)),native_summary_path=str(summaries[0]),
        source_manifest_sha256=sha('/input/source-manifest.json') if snapshot else None,
        scope='Actual glibc 2.41 native baseline; no full safety campaign claim.',runtime=runtime,
        identity=dict(uid=os.getuid(),euid=os.geteuid(),gid=os.getgid()),
        header_probe=probe,libraries=libraries,native_summary=summary,source_acceptance=source_cases,
        packages=package,package_status='not-run-uncommitted-snapshot' if snapshot else 'passed',allocation_hook_profile='NATIVE allocation hook controls passed; remaining sites unimplemented',
        safety_profile_status='ASan/LSan/Miri not-run here; native baseline/hooks do not establish full safety campaigns'),indent=2)+'\n')


if __name__=='__main__':
    main()
