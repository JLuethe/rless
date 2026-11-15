"""Re-derive committed host baseline evidence before granting scoped M0 credit.

Historical schema-1 records remain historical. This adapter never executes a
pager or compiler and never turns native compatibility into runtime safety.
"""
import hashlib
import json
from pathlib import Path
import re
import sys
import tarfile

from common import ROOT, PINS, DIAGNOSTICS, fingerprint, sha, validate_oracle_metadata

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from release import (PROVENANCE, commit_manifest, integer, validate_acceptance,
                     validate_acceptance_observer as acceptance_observer,
                     validate_verification, verify_export, validate_reference_record)

PROCEDURE = 'BUILDER-GLIBC-2.44-NATIVE-v1'
ABI = 'BUILDER-GLIBC-2.44'
# Expected Linux x86_64 GNU ABI used by the translated declarations and libc.
LAYOUT = dict(pointer_bytes=8, off_t_bytes=8, regex_bytes=64, regex_alignment=8,
    regex_nsub_offset=48, regmatch_bytes=8, regs_fixed=2, regoff_bytes=4,
    registers_bytes=24, registers_start_offset=8, registers_end_offset=16,
    regs_flag_byte_offset=56, regs_fixed_mask=4, sigaction_bytes=152, sigset_bytes=128,
    termios_bytes=60, stat_bytes=144, sigint=2, sigwinch=28, sigpipe=13)
TOOLS = {'rustc', 'cargo', 'cc', 'linker', 'glibc', 'ldd', 'terminfo', 'readelf', 'python'}


def require(value, message):
    if not value:
        raise ValueError(message)


def checked(reference, *, within=None):
    require(isinstance(reference, dict) and set(reference) == {'path', 'sha256'}, 'invalid host artifact reference')
    path = Path(reference['path'])
    require(path.is_absolute() and '..' not in path.parts and path.is_file() and not path.is_symlink(),
            'missing or linked host artifact: ' + str(path))
    require(path.resolve() == path, 'host artifact path traverses a link: ' + str(path))
    if within is not None:
        require(path.is_relative_to(Path(within).resolve()), 'host artifact escaped retained root')
    require(sha(path) == reference['sha256'], 'changed host artifact: ' + str(path))
    return path


def read_ref(reference, **kwargs):
    return json.loads(checked(reference, **kwargs).read_text())


def executable(reference):
    path = checked(reference)
    require(path.stat().st_mode & 0o111, 'host executable permission was removed: ' + str(path))
    return path


def validate_source_archive(path, manifest, prefix=None):
    """Compare names, bytes and executable modes without extracting an archive."""
    with tarfile.open(path) as archive:
        members = [item for item in archive.getmembers() if not item.isdir()]
        names = [item.name for item in members]
        expected = {f'{prefix}/{name}' if prefix else name: value for name, value in manifest['files'].items()}
        provenance = f'{prefix}/{PROVENANCE}' if prefix else None
        require(len(names) == len(set(names)) and set(names) == set(expected) | ({provenance} if provenance else set()),
                'host source archive has omitted, duplicate or extra files')
        for member in members:
            require(member.isfile(), 'host source archive has linked/non-file input')
            content = archive.extractfile(member).read()
            if member.name == provenance:
                require(json.loads(content) == manifest and member.mode == 0o644, 'archived source provenance mismatch')
            else:
                value = expected[member.name]
                require((0o755 if member.mode & 0o111 else 0o644) == value['mode'] and
                        hashlib.sha256(content).hexdigest() == value['sha256'], 'archived source bytes/mode changed')


def validate_commands(record, out, source, rebuilt):
    commands = record.get('commands')
    require(isinstance(commands, list), 'host command records missing')
    by_name = {}
    for item in commands:
        require(item['name'] not in by_name, 'duplicate host command')
        require(integer(item.get('original_exit_status'), 0) and integer(item.get('expected_exit_status'), 0),
                'host command did not succeed')
        log = checked(item['log'], within=out)
        require(log.name == item['name'] + '.log' and not DIAGNOSTICS.search(log.read_text(errors='replace')),
                'host command log identity or safety diagnostic failure')
        require(isinstance(item.get('command'), list) and item['command'] and all(isinstance(v, str) for v in item['command']),
                'host command argv missing')
        by_name[item['name']] = item
    required = {'package', 'source-build', 'source-tests', 'source-acceptance', 'abi-build', 'abi-probe',
        'abi-headers', 'version-rustc', 'version-cargo', 'version-cc', 'version-linker', 'version-glibc',
        'ldd-reference', 'ldd-debug', 'ldd-release', 'terminfo'}
    resolution = record.get('rust_tool_resolution', {})
    require(set(resolution) == {'rustc','cargo'}, 'host Rust tool resolution missing')
    required |= {'path-'+name for name,value in resolution.items() if value['resolver']=='rustup'}
    require(required <= set(by_name) <= required | {'native'}, 'host baseline command selection incomplete')
    expected = {
        'source-build': ['cargo', 'build', '--release', '--locked', '--offline', '--bins'],
        'source-tests': ['cargo', 'test', '--release', '--locked', '--offline'],
        'abi-build': ['cc', '-Wall', '-Wextra', '-Werror', str(source/'tools/safety/abi_probe.c'), '-o', str(out/'abi-probe')],
        'abi-probe': [str(out/'abi-probe')], 'abi-headers': ['cc', '-M', str(source/'tools/safety/abi_probe.c')],
        'version-rustc': ['rustc', '-vV'], 'version-cargo': ['cargo', '-V'], 'version-cc': ['cc', '--version'],
        'version-linker': ['ld', '--version'], 'version-glibc': ['getconf', 'GNU_LIBC_VERSION'],
        'terminfo': ['infocmp', '-x', 'xterm-256color']}
    expected.update({'path-'+name:['rustup','which',name] for name,value in resolution.items() if value['resolver']=='rustup'})
    for name, command in expected.items():
        require(by_name[name]['command'] == command, 'unexpected host command selection: ' + name)
    for name, item in by_name.items():
        require(Path(item['cwd']).resolve() == (rebuilt if name.startswith('source-') else source),
                'host command used wrong source directory')
    return by_name


def validate_package(record, out, source, manifest, native):
    package = record['packages']
    archive = checked(dict(path=package['archive'], sha256=package['sha256']), within=out)
    source_archive = checked(dict(path=package['source_archive'], sha256=package['source_sha256']), within=out)
    attestation_path = checked(dict(path=package['release_attestation'], sha256=package['release_attestation_sha256']), within=out)
    attestation = json.loads(attestation_path.read_text())
    require(integer(attestation.get('schema_version'), 1) and attestation.get('status') == 'release-qualified' and
            attestation.get('runtime_safety_verified') is False and package.get('extracted_smoke_tests') == 'passed',
            'package final attestation missing or promoted to runtime safety')
    require(attestation['source_commit'] == manifest['source_commit'] and
            attestation['source_manifest_sha256'] == manifest['manifest_sha256'] and
            attestation['verification_sha256'] == record['native_summary']['sha256'] and
            attestation['archives'] == {archive.name:sha(archive), source_archive.name:sha(source_archive)} and
            attestation.get('native_checks') == 'complete' and attestation.get('source_rebuild') == '809/809' and
            attestation.get('extracted_smoke_tests') == 'passed', 'package attestation identities/policy changed')
    for path in (archive, source_archive):
        require(path.with_suffix(path.suffix+'.sha256').read_text() == f'{sha(path)}  {path.name}\n', 'package checksum file mismatch')
    source_name = source_archive.name.removesuffix('.tar.gz')
    validate_source_archive(source_archive, manifest, source_name)
    qualification = archive.parent / (archive.name.removesuffix('.tar.gz') + '-qualification')
    result_path = checked(dict(path=str(qualification/'result.json'), sha256=attestation['source_rebuild_sha256']), within=out)
    result = json.loads(result_path.read_text())
    require(result.get('passed') is True and result['source_commit'] == manifest['source_commit'] and
            result['source_manifest_sha256'] == manifest['manifest_sha256'] and result['source_archive_sha256'] == sha(source_archive),
            'package source qualification changed')
    steps = result.get('records', [])
    require([item['name'] for item in steps] == ['source-build','source-tests','source-acceptance'], 'package source rebuild incomplete')
    for item in steps:
        require(integer(item.get('returncode'), 0), 'package source rebuild command failed')
        log = checked(dict(path=str(qualification/item['log']), sha256=item['log_sha256']), within=qualification)
        require(not DIAGNOSTICS.search(log.read_text(errors='replace')), 'package source rebuild sanitizer diagnostic')
    require(steps[0]['command'] == ['cargo','build','--release','--locked','--offline','--bins'] and
            steps[1]['command'] == ['cargo','test','--release','--locked','--offline'], 'package source build policy changed')
    replay = json.loads((qualification/'acceptance/summary.json').read_text())
    require(replay == result['acceptance'], 'package source replay summary changed')
    validate_acceptance(replay)
    acceptance_observer(replay, source)
    require(set(result['rebuilt_binaries']) == {'rless','lessecho'} and replay['sha256'] == result['rebuilt_binaries']['rless'],
            'package rebuilt binary/replay identity mismatch')
    require(all(isinstance(value,str) and re.fullmatch('[0-9a-f]{64}',value) for value in result['rebuilt_binaries'].values()),
            'package source rebuilt binary hashes missing')
    # Validate the final binary archive bytes without running its executables.
    prefix = archive.name.removesuffix('.tar.gz') + '/'
    with tarfile.open(archive) as tar:
        members = tar.getmembers()
        require(all(item.isfile() and item.name.startswith(prefix) and '..' not in Path(item.name).parts for item in members) and
                len({item.name for item in members}) == len(members), 'binary package has ambiguous or linked members')
        contents = {item.name.removeprefix(prefix):tar.extractfile(item).read() for item in members}
        require(set(contents) == {'bin/rless','bin/lessecho','manifest.json','verification.json','source-rebuild.json',
            'libexec/less-osc8-open','activate.sh','README.txt','licenses/GPL-3.0.txt','licenses/Less.txt','licenses/C2Rust-BSD.txt'},
            'binary package file selection changed')
        require(contents['verification.json'] == checked(record['native_summary']).read_bytes() and
                contents['source-rebuild.json'] == result_path.read_bytes(), 'binary package embedded evidence differs')
        metadata = json.loads(contents['manifest.json'])
        require(metadata['source_commit'] == manifest['source_commit'] and metadata['source_manifest_sha256'] == manifest['manifest_sha256'] and
                metadata['verification_sha256'] == record['native_summary']['sha256'] and
                metadata['source_rebuild_sha256'] == sha(result_path), 'binary package provenance differs')
        for name, digest in [('rless',native['binaries']['release']['sha256']), ('lessecho',native['binaries']['release']['companion_sha256'])]:
            require(hashlib.sha256(contents['bin/'+name]).hexdigest() == digest == metadata['binaries']['bin/'+name] and
                    b'RLESS_FAULT' not in contents['bin/'+name], 'binary package differs from tested default binary')
        require(hashlib.sha256(contents['libexec/less-osc8-open']).hexdigest() == native['helpers']['osc8_sha256'] ==
                metadata['helpers']['libexec/less-osc8-open'], 'packaged helper differs from tested helper')
        for member in members:
            relative = member.name.removeprefix(prefix)
            require(member.mode == (0o755 if relative.startswith(('bin/','libexec/')) else 0o644), 'packaged file mode mismatch')
        for name, path in [('GPL-3.0.txt','LICENSE'), ('Less.txt','LICENSE-LESS'), ('C2Rust-BSD.txt','vendor/c2rust-bitfields/LICENSE')]:
            require(contents['licenses/'+name] == (source/path).read_bytes(), 'packaged license differs from source')
    return archive, source_archive


def normalize(control, log, root, *, profile='NATIVE', suffix='', artifacts=None):
    metadata = dict(abi_profile=ABI, corpus_sha256=sha(root/'compat/corpus.json'), seed=None,
        test_selector='host-baseline.' + control + suffix, hook_sites=[], hook_ordinals=[], m0_controls=[control])
    if control.startswith('acceptance-'):
        metadata.update(executed_cases=809, passed_cases=809, failed_cases=0)
    validate_oracle_metadata(metadata)
    return dict(name='host-' + control + suffix, status='passed', profile=profile, requirement_ids=[],
        scenario_id=None, variant=control + suffix, coverage_scope='partial', finding_ids=[],
        original_exit_status=0, expected_exit_status=0, expected_diagnostic=None, timed_out=False, stale_inputs=False,
        log=str(log), log_sha256=sha(log), artifact_hashes=artifacts or {}, oracle_metadata=metadata,
        _validated_m0_controls=[control])


def validate_external_report(path, root=ROOT):
    root, path = Path(root).resolve(), Path(path).resolve()
    record = json.loads(path.read_text())
    require(integer(record.get('schema_version'), 2) and record.get('report_kind') == 'host-native-baseline' and
            record.get('procedure') == PROCEDURE and record.get('source_kind') == 'committed-source', 'unsupported host baseline procedure')
    require(record.get('status') == 'passed' and record.get('finished_complete') is True and
            integer(record.get('original_exit_status'), 0) and record.get('runtime_safety_verified') is False and
            record.get('finished_at_utc') and record.get('started_at_utc'), 'host baseline did not finish successfully')
    require(record.get('tool_pins') == PINS, 'host tool profile changed')
    out = path.parent
    source, rebuilt = Path(record['source_root']), Path(record['source_rebuild_root'])
    require(source.is_absolute() and source.resolve() == source and rebuilt.resolve().is_relative_to(out), 'invalid host source roots')
    manifest = read_ref(record['source_manifest'])
    require(manifest == commit_manifest(root, record['source_commit']), 'host source manifest differs from committed Git inputs')
    verify_export(root, manifest)
    verify_export(source, manifest)
    verify_export(rebuilt, manifest)
    require(checked(record['source_manifest']) == source/PROVENANCE and
            json.loads((rebuilt/PROVENANCE).read_text()) == manifest, 'source export provenance differs')
    files = {name:value['sha256'] for name,value in manifest['files'].items()}
    require(record['source_files_sha256'] == files and record['source_manifest_sha256'] ==
            hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest(), 'legacy source manifest identity changed')
    validate_source_archive(checked(record['source_archive'], within=out), manifest)
    require(record['observer_sha256'] == sha(root/'tools/compat/run_v2.py') and
            record['frozen_corpus_sha256'] == sha(root/'compat/corpus.json'), 'host observer/corpus identity changed')
    for name, digest in json.loads((root/'compat/corpus.json').read_text())['files'].items():
        require(name in files and files[name] == digest, 'host frozen corpus content changed')
    native_path = checked(record['native_summary'])
    require(native_path.resolve().is_relative_to(source/'compat/.runs'), 'native report outside committed source run directory')
    native = json.loads(native_path.read_text())
    require(native == record['native_verification'], 'embedded native verification changed')
    validate_verification(native, native_path, source)
    for filename, digest in native['evidence_files'].items():
        if filename.endswith('.log'):
            log=checked(dict(path=str(native_path.parent/filename),sha256=digest),within=native_path.parent)
            require(not DIAGNOSTICS.search(log.read_text(errors='replace')), 'unexpected native diagnostic in retained log')
    binaries = {}
    for profile in ('debug','release'):
        value = native['binaries'][profile]
        require(Path(value['path']) == source/'target'/profile/'rless', 'host native binary path changed')
        binaries[profile] = executable(dict(path=value['path'], sha256=value['sha256']))
        executable(dict(path=str(binaries[profile].parent/'lessecho'),sha256=value['companion_sha256']))
    for name, field in [('rless','sha256'),('lessecho','companion_sha256')]:
        executable(dict(path=str(native_path.parent/'installed/bin'/name),sha256=native['installed'][field]))
    require(sha(source/'vendor/less-helpers/less-osc8-open') == native['helpers']['osc8_sha256'], 'host helper changed')
    validate_reference_record(record, root)
    require(native['reference_bundle'] == record['reference_bundle'], 'host reference bundle selection mismatch')
    reference = executable(record['reference'])
    require(set(record['reference_bundle']) == {'less','lessecho','less-osc8-open.sh'}, 'host reference bundle incomplete')
    for name, value in record['reference_bundle'].items():
        require(checked(value) == (reference if name == 'less' else (reference.parent/name).resolve()), 'host reference bundle relocated')
    require(native['reference'] == record['reference'], 'host reference selection mismatch')
    commands = validate_commands(record, out, source, rebuilt)
    require(commands['source-acceptance']['command'][1:] == [str(rebuilt/'tools/compat/check_candidate.py'),'--binary',
        str(rebuilt/'target/release/rless'),'--results-dir',str(out/'source-acceptance')], 'host source replay selection changed')
    require(commands['package']['command'][1:] == [str(source/'tools/package.py'),'--verification',str(native_path),
        '--output-dir',str(out/'packages')], 'host package command selection changed')
    source_cases_path = checked(record['source_acceptance_report'], within=out)
    source_cases = json.loads(source_cases_path.read_text())
    require(source_cases == record['source_acceptance'], 'host source acceptance changed')
    validate_acceptance(source_cases)
    acceptance_observer(source_cases,source)
    require(set(record['source_rebuilt_binaries']) == {'rless','lessecho'}, 'host source rebuilt companion missing')
    for name, value in record['source_rebuilt_binaries'].items():
        require(executable(value) == rebuilt/'target/release'/name, 'wrong host source rebuilt binary')
    require(source_cases['binary'] == str(rebuilt/'target/release/rless') and
            source_cases['sha256'] == record['source_rebuilt_binaries']['rless']['sha256'], 'host source replay binary changed')
    require(json.loads(checked(commands['package']['log']).read_text().splitlines()[-1]) == record['packages'], 'package output differs from retained result')
    validate_package(record, out, source, manifest, native)
    require(set(record['tools']) == TOOLS, 'host tool binary provenance incomplete')
    for name, value in record['tools'].items():
        executable(value)
        if name in ('rustc','cargo'):
            resolution=record['rust_tool_resolution'][name]
            invoked=Path(resolution['invoked_path'])
            require(invoked.is_absolute() and invoked.is_file(), 'host Rust invocation path missing')
            if resolution['resolver']=='rustup':
                require(invoked.resolve().name=='rustup' and
                        Path(checked(commands['path-'+name]['log']).read_text().strip()).resolve() == Path(value['path']), 'host rustup tool selection changed')
            else:
                require(resolution['resolver']=='direct' and invoked.resolve()==Path(value['path']), 'host direct Rust tool selection changed')
    require(record['tool_files_sha256'] == {value['path']:value['sha256'] for value in record['tools'].values()}, 'host tool hash map incomplete')
    for name in ('package','source-acceptance'):
        require(Path(commands[name]['command'][0]).resolve() == Path(record['tools']['python']['path']), 'host Python interpreter changed')
    versions = record['tool_versions']
    require(set(versions) == {'rustc','cargo','cc','linker','glibc'}, 'host tool versions incomplete')
    for name, value in versions.items():
        require(value.strip() and checked(commands['version-'+name]['log']).read_text() == value, 'host tool version log mismatch')
    require(versions['glibc'].strip() == 'glibc 2.44' and 'rustc '+PINS['normal_rust_release'] in versions['rustc'] and
            'host: '+PINS['target'] in versions['rustc'] and 'cargo '+PINS['normal_rust_release'] in versions['cargo'], 'host ABI/compiler profile mismatch')
    require(record['host']['machine'] == 'x86_64' and 'Linux' in record['host']['system'], 'host platform mismatch')
    requirements = dict(line.split('==') for line in (root/'tools/compat/requirements.txt').read_text().splitlines() if line.strip() and not line.startswith('#'))
    require(record['python_dependencies'] == requirements, 'host observer Python dependencies changed')
    probe = json.loads(checked(commands['abi-probe']['log']).read_text())
    require(probe == record['header_probe'] and probe.get('glibc_headers') == '2.44' and probe.get('fixed_register_probe') is True and
            isinstance(probe.get('ncurses_headers'), str) and probe['ncurses_headers'] and
            all(integer(probe.get(name), value) for name,value in LAYOUT.items()), 'host ABI layout/probe mismatch')
    require(executable(record['abi_probe_binary']) == out/'abi-probe', 'host probe binary changed')
    header_text = checked(commands['abi-headers']['log']).read_text()
    header_names = set(header_text.replace('\\\n',' ').split()[1:])
    require(header_names == set(record['headers']) == set(record['headers_sha256']) and
            str(source/'tools/safety/abi_probe.c') in header_names and len(header_names) > 10, 'host header dependency set incomplete')
    for requested, value in record['headers'].items():
        require(checked(value) == Path(requested).resolve() and value['sha256'] == record['headers_sha256'][requested], 'host header content/target changed')
    require(set(record['native_libraries']) == {'reference','debug','release'}, 'host native library manifest incomplete')
    for name, item in record['native_libraries'].items():
        binary = reference if name == 'reference' else binaries[name]
        require(checked(item['binary']) == binary and commands['ldd-'+name]['command'] == ['ldd',str(binary)], 'host loaded-object binary mismatch')
        text = checked(commands['ldd-'+name]['log']).read_text()
        requested = set(re.findall(r'(/[^\s()]+)',text))
        require(text == item['ldd'] and requested == set(item['objects']) == set(item['libraries_sha256']) and
                any('/libc.so.' in value for value in requested) and any('/libtinfo.so.' in value for value in requested), 'host loaded object set incomplete')
        for target, value in item['objects'].items():
            require(checked(value) == Path(target).resolve() and item['libraries_sha256'][target] == value['sha256'], 'host loaded native object changed')
    terminal_log=checked(commands['terminfo']['log'])
    terminal_text=terminal_log.read_text()
    require(sha(terminal_log) == record['terminfo_xterm256_sha256'] and all(value in terminal_text for value in
            ('xterm-256color','cup=','setaf=')), 'host terminal database capture missing or changed')
    require(record['diagnostic_logs'] == {str(p):sha(p) for p in sorted(out.glob('*.log'))} and
            record['native_logs'] == {str(p):sha(p) for p in sorted(native_path.parent.glob('*.log'))}, 'host diagnostic/native log manifest changed')
    require(set(record['acceptance']) == {'acceptance-debug','acceptance-release','reference','acceptance-installed'}, 'host acceptance summary set incomplete')
    records = [normalize('native-provenance', checked(commands['version-glibc']['log']), root, profile='ABI'),
               normalize('abi-layout-probes', checked(commands['abi-probe']['log']), root, profile='ABI',
                         artifacts={record['abi_probe_binary']['path']:record['abi_probe_binary']['sha256']})]
    for lane in ('acceptance-debug','acceptance-release','reference','acceptance-installed'):
        data = json.loads((native_path.parent/lane/'summary.json').read_text())
        acceptance_observer(data,source)
        require(data == record['acceptance'][lane] and sha(data['binary']) == data['sha256'], 'host acceptance summary/binary mismatch')
        control = 'acceptance-reference' if lane == 'reference' else lane
        records.append(normalize(control, native_path.parent/(lane+'.log'), root))
    records.append(normalize('acceptance-source-rebuild',checked(commands['source-acceptance']['log']),root))
    for lane in ('helpers','integrations','job-control'):
        for profile in ('debug','release'):
            records.append(normalize(lane,native_path.parent/(lane+'-'+profile+'.log'),root,suffix='-'+profile))
    return dict(schema_version=1, source_commit=record['source_commit'], input_fingerprints=fingerprint(root),
        tool_pins=PINS, tool_files_sha256=record['tool_files_sha256'], records=records,
        status='passed', runtime_safety_verified=False)
