#!/usr/bin/env python3
"""Reproduce or resume a committed native baseline; retain exact provenance."""
import argparse
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import subprocess
import sys
import tarfile
import tempfile

from common import ROOT, PINS, environment, git, sha, utc, write_json

PROCEDURE = 'BUILDER-GLIBC-2.44-NATIVE-v1'


def ref(path):
    path = Path(path).resolve(strict=True)
    return dict(path=str(path), sha256=sha(path))


def invoke(command, cwd, env, log, timeout=1200, records=None):
    print('Running: ' + ' '.join(str(x) for x in command), flush=True)
    with Path(log).open('wb') as out:
        result = subprocess.run([str(x) for x in command], cwd=cwd, env=env,
                                stdout=out, stderr=subprocess.STDOUT, timeout=timeout)
    if records is not None:
        records.append(dict(name=Path(log).stem, command=[str(x) for x in command], cwd=str(cwd),
                            original_exit_status=result.returncode, expected_exit_status=0,
                            log=ref(log)))
    if result.returncode:
        raise RuntimeError('command failed; see ' + str(log))
    return Path(log).read_text(errors='replace')


def verify_snapshot(source, commit):
    paths = git('ls-tree', '-rz', '--name-only', commit).split(b'\0')
    files = {}
    for raw in paths:
        if not raw:
            continue
        name = os.fsdecode(raw)
        path = source / name
        if not path.is_file() or path.read_bytes() != git('show', commit + ':' + name):
            raise ValueError('snapshot does not match commit: ' + name)
        files[name] = sha(path)
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--commit', default='HEAD')
    parser.add_argument('--reference', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--resume-source', type=Path, help='Previously created pristine git-archive snapshot')
    parser.add_argument('--resume-summary', type=Path, help='Completed tools/check.py summary in that snapshot')
    args = parser.parse_args()
    if sys.flags.optimize:
        parser.error('baseline verification requires Python assertions enabled')
    os.umask(0o077)
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    commit = git('rev-parse', args.commit).decode().strip()
    reference = args.reference.resolve(strict=True)
    env = environment()
    archive = out / 'committed-source.tar'
    archive.write_bytes(git('archive', commit))
    if args.resume_source:
        if not args.resume_summary:
            parser.error('--resume-summary is required with --resume-source')
        source = args.resume_source.resolve(strict=True)
    else:
        source = out / 'source'
        source.mkdir()
        with tarfile.open(archive) as tar:
            tar.extractall(source, filter='data')
    files = verify_snapshot(source, commit)
    # The check/packaging tools must not infer a snapshot's commit from a parent
    # checkout or accept arbitrary untracked files. This manifest is emitted by
    # the trusted Git exporter after every exported byte has been compared.
    sys.path.insert(0, str(ROOT / 'tools'))
    from release import commit_manifest, write_export_manifest, validate_verification, validate_acceptance
    manifest_path = write_export_manifest(source, commit_manifest(ROOT, commit))
    commands = []
    def run(command, cwd, env, log, timeout=1200):
        return invoke(command, cwd, env, log, timeout, commands)
    record = dict(schema_version=2, report_kind='host-native-baseline', procedure=PROCEDURE,
                  source_kind='committed-source', tool_pins=PINS, commands=commands,
                  started_at_utc=utc(), source_commit=commit, status='not-verified',
                  original_exit_status=1, finished_complete=False, runtime_safety_verified=False,
                  source_root=str(source), source_manifest=ref(manifest_path), source_archive=ref(archive),
                  scope='Native unsafe compatibility baseline; not safety migration completion.',
                  source_files_sha256=files, source_manifest_sha256=hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest(),
                  reference=dict(path=str(reference), sha256=sha(reference)),
                  observer_sha256=sha(source / 'tools/compat/run_v2.py'),
                  frozen_corpus_sha256=sha(source / 'compat/corpus.json'),
                  permissions=dict(uid=os.getuid(), euid=os.geteuid(), gid=os.getgid(),
                       resource_limits={name:resource.getrlimit(getattr(resource, name)) for name in ('RLIMIT_AS','RLIMIT_DATA','RLIMIT_STACK','RLIMIT_NOFILE')}),
                  host=dict(system=platform.platform(), machine=platform.machine()))
    write_json(out / 'summary.json', record)
    try:
        if args.resume_summary:
            summary_path = args.resume_summary.resolve(strict=True)
        else:
            run([sys.executable, source / 'tools/check.py', '--release', '--reference', reference,
                    '--stress-rounds', '100', '--install-check'], source, env, out / 'native.log')
            summaries = sorted((source / 'compat/.runs').glob('verification-*/summary.json'))
            if len(summaries) != 1:
                raise ValueError('expected exactly one new verification summary')
            summary_path = summaries[0]
        summary = json.loads(summary_path.read_text())
        validate_verification(summary, summary_path, source)
        record['native_summary'] = ref(summary_path)
        required = ['format','rust-debug','rust-release','observer','corpus','negative-candidate',
                    'acceptance-debug','acceptance-release','reference','clean-install','acceptance-installed']
        required += [f'{lane}-{profile}' for profile in ('debug','release') for lane in ('integrations','job-control','helpers')]
        required += [f'stress-{profile}-{seed}' for profile in ('debug','release') for seed in (712,20261023)]
        steps = {s['name']:s for s in summary['steps']}
        if summary.get('passed') is not True or set(steps) != set(required):
            raise ValueError('incomplete baseline verification step selection')
        for name in required:
            step = steps[name]
            if step['returncode'] != step['expected_returncode'] or not step['passed']:
                raise ValueError('failed native step: ' + name)
            if not (summary_path.parent / (name + '.log')).is_file():
                raise ValueError('missing native log: ' + name)
        acceptance = {}
        for name in ('acceptance-debug','acceptance-release','reference','acceptance-installed'):
            data = json.loads((summary_path.parent / name / 'summary.json').read_text())
            if data['executed'] != 809 or data['passed'] != 809 or data['failed'] or any(x['errors'] for x in data['families'].values()):
                raise ValueError('incomplete acceptance replay: ' + name)
            if sha(data['binary']) != data['sha256']:
                raise ValueError('stale tested native binary: ' + name)
            acceptance[name] = data
        for p, h in json.loads((source / 'compat/corpus.json').read_text())['files'].items():
            if sha(source / p) != h:
                raise ValueError('frozen corpus changed: ' + p)
        package_output = run([sys.executable, source / 'tools/package.py', '--verification', summary_path,
                       '--output-dir', out / 'packages'], source, env, out / 'package.log')
        package = json.loads(package_output.splitlines()[-1])
        for key, hashkey in [('archive','sha256'),('source_archive','source_sha256')]:
            if sha(package[key]) != package[hashkey]:
                raise ValueError('package hash mismatch')
        extracted = out / 'source-rebuild'
        extracted.mkdir()
        with tarfile.open(package['source_archive']) as tar:
            tar.extractall(extracted, filter='data')
        rebuilt = next(extracted.iterdir())
        verify_snapshot(rebuilt, commit)
        run(['cargo','build','--release','--locked','--offline','--bins'], rebuilt, env, out / 'source-build.log')
        run(['cargo','test','--release','--locked','--offline'], rebuilt, env, out / 'source-tests.log')
        run([sys.executable, rebuilt / 'tools/compat/check_candidate.py', '--binary', rebuilt / 'target/release/rless',
                '--results-dir', out / 'source-acceptance'], rebuilt, env, out / 'source-acceptance.log')
        source_cases = json.loads((out / 'source-acceptance/summary.json').read_text())
        validate_acceptance(source_cases)
        record.update(source_rebuild_root=str(rebuilt), source_acceptance_report=ref(out / 'source-acceptance/summary.json'),
                      source_rebuilt_binaries={name:ref(rebuilt/'target/release'/name) for name in ('rless','lessecho')})
        if source_cases['executed'] != 809 or source_cases['passed'] != 809 or source_cases['failed']:
            raise ValueError('source archive acceptance incomplete')
        probe = out / 'abi-probe'
        probe_source = source / 'tools/safety/abi_probe.c'
        run(['cc','-Wall','-Wextra','-Werror',probe_source,'-o',probe], source, env, out / 'abi-build.log')
        record['header_probe'] = json.loads(run([probe], source, env, out / 'abi-probe.log'))
        record['abi_probe_binary'] = ref(probe)
        headers = run(['cc','-M',probe_source], source, env, out / 'abi-headers.log')
        record['headers_sha256'] = {p:sha(p) for p in headers.replace('\\\n',' ').split()[1:] if Path(p).is_file()}
        record['headers'] = {p:ref(p) for p in record['headers_sha256']}
        record['tool_versions'] = {name:run(command, source, env, out / ('version-' + name + '.log'))
             for name,command in [('rustc',['rustc','-vV']),('cargo',['cargo','-V']),('cc',['cc','--version']),
                    ('linker',['ld','--version']),('glibc',['getconf','GNU_LIBC_VERSION'])]}
        record['tools'] = {name:ref(shutil.which(program, path=env['PATH'])) for name, program in
                          [('cc','cc'),('linker','ld'),('glibc','getconf'),('ldd','ldd'),('terminfo','infocmp'),('readelf','readelf')]}
        record['rust_tool_resolution'] = {}
        for name in ('rustc','cargo'):
            invoked = Path(shutil.which(name,path=env['PATH']))
            resolver = 'rustup' if invoked.resolve().name == 'rustup' else 'direct'
            selected = run(['rustup','which',name], source, env, out / ('path-' + name + '.log')).strip() if resolver == 'rustup' else invoked
            record['tools'][name] = ref(selected)
            record['rust_tool_resolution'][name] = dict(invoked_path=str(invoked), resolver=resolver)
        record['tools']['python'] = ref(sys.executable)
        record['tool_files_sha256'] = {item['path']:item['sha256'] for item in record['tools'].values()}
        record['python_dependencies'] = {name:importlib.metadata.version(name) for name in
            [line.split('==')[0] for line in (source/'tools/compat/requirements.txt').read_text().splitlines() if line.strip() and not line.startswith('#')]}
        record['reference_bundle'] = {name:ref(reference if name == 'less' else reference.parent/name)
                                      for name in ('less','lessecho','less-osc8-open.sh')}
        native_libraries = {}
        for name,binary in [('reference',reference),('debug',source / 'target/debug/rless'),('release',source / 'target/release/rless')]:
            text = run(['ldd',binary], source, env, out / ('ldd-' + name + '.log'))
            libs = {p:sha(p) for p in re.findall(r'(/[^\s()]+)',text) if Path(p).is_file()}
            native_libraries[name] = dict(ldd=text,libraries_sha256=libs,binary=ref(binary),objects={p:ref(p) for p in libs})
        terminfo = run(['infocmp','-x','xterm-256color'], source, env, out / 'terminfo.log').encode()
        record.update(status='passed', native_verification=summary, acceptance=acceptance,
              packages=package, source_acceptance=source_cases, native_libraries=native_libraries,
              terminfo_xterm256_sha256=hashlib.sha256(terminfo).hexdigest(),
              glibc_241_safety_evidence='not-verified; requires a separate environment',
              diagnostic_logs={str(p):sha(p) for p in sorted(out.glob('*.log'))},
              native_logs={str(p):sha(p) for p in sorted(summary_path.parent.glob('*.log'))},
              runtime_safety_verified=False)
        verify_snapshot(source, commit)
        record.update(original_exit_status=0, finished_complete=True, finished_at_utc=utc())
        candidate_path = out / '.baseline-candidate.json'
        write_json(candidate_path, record)
        from host_evidence import validate_external_report
        validate_external_report(candidate_path, root=ROOT)
        candidate_path.unlink()
    except (OSError, ValueError, RuntimeError, KeyError, TypeError, IndexError, subprocess.SubprocessError) as error:
        record['error'] = str(error)
        record.update(status='not-verified', original_exit_status=1, finished_complete=False)
        print(str(error), file=sys.stderr)
    record['finished_at_utc'] = utc()
    write_json(out / 'summary.json', record)
    print('Baseline: ' + record['status'] + '; ' + str(out / 'summary.json'), flush=True)
    return 0 if record['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
