"""Revalidate reference ABI evidence; status booleans never grant coverage."""
import hashlib
import json
from pathlib import Path
import re
import tarfile
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from release import reference_policy, validate_reference_hashes

from common import ROOT, PINS, DIAGNOSTICS, fingerprint, sha, validate_oracle_metadata
from .snapshot import validate, source_names
from .offline_symbolize import symbolize, require_source
from native_alloc import RUSTFLAGS as ALLOC_RUSTFLAGS, CFLAGS as ALLOC_CFLAGS, validate_retained_binding

PROCEDURE = 'REFERENCE-GLIBC-2.41-ASAN-OFFLINE-v1'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def checked(path, digest):
    path = Path(path)
    require(path.is_file() and not path.is_symlink() and sha(path) == digest,
            'missing or altered evidence: ' + str(path))
    return path


def work_path(target, work):
    target = Path(target)
    require(target.is_relative_to('/work') and '..' not in target.parts, 'escaping target path')
    path = work / target.relative_to('/work')
    require(path.resolve().is_relative_to(work.resolve()), 'escaping retained artifact')
    return path


def metadata(root, selector, controls=()):
    value = dict(abi_profile='REFERENCE-GLIBC-2.41', corpus_sha256=sha(root / 'compat/corpus.json'),
                 seed=None, test_selector=selector, hook_sites=[], hook_ordinals=[],
                 m0_controls=list(controls))
    validate_oracle_metadata(value)
    return value


def normalize(name, log, profile, root, *, expected=0, original=0, diagnostic=None,
              controls=(), artifacts=None):
    return dict(name=name, status='passed', profile=profile, requirement_ids=[],
                scenario_id=None, variant=name, coverage_scope='partial', finding_ids=[],
                original_exit_status=original, expected_exit_status=expected,
                expected_diagnostic=diagnostic, timed_out=False, stale_inputs=False,
                log=str(log), log_sha256=sha(log), artifact_hashes=artifacts or {},
                oracle_metadata=metadata(root, name, controls))


def validate_asan_report(path, root=ROOT):
    root, path = Path(root).resolve(), Path(path).resolve()
    outer = json.loads(path.read_text())
    require(outer.get('procedure') == PROCEDURE, 'unsupported reference ABI report procedure')
    require(outer.get('status') == 'passed' and outer.get('original_exit_status') == 0,
            'reference ABI campaign did not pass')
    out, work = path.parent, path.parent / 'work'
    snapshot = validate(outer['source_manifest'], root)
    require(set(source_names(root)) == set(snapshot['files']), 'current source selection changed')
    require(outer['source_commit'] == snapshot['base_commit'], 'source commit mismatch')
    manifest = checked(out / 'input/source-manifest.json', outer['source_manifest_sha256'])
    require(json.loads(manifest.read_text()) == snapshot, 'source manifest identity mismatch')
    source_tar = checked(out / 'input/source.tar', outer['source_tar_sha256'])
    with tarfile.open(source_tar) as archive:
        members = archive.getmembers()
        require(len(members) == len(snapshot['files']) and {m.name for m in members} == set(snapshot['files']),
                'source archive selection changed')
        for member in members:
            expected = snapshot['files'][member.name]
            require(member.isfile() and member.mode == expected['mode'], 'source archive type/mode mismatch')
            require(hashlib.sha256(archive.extractfile(member).read()).hexdigest() == expected['sha256'],
                    'source archive bytes differ from frozen inputs')
    validate(snapshot, work / 'source')
    require(outer['tool_pins'] == PINS, 'reference tool pins changed')
    checked(root / 'tools/safety/glibc241/offline-symbolization-v1.json', outer['procedure_sha256'])
    lock_path = checked(root / 'docs/verification/glibc241-environment.json', outer['environment_lock_sha256'])
    lock = json.loads(lock_path.read_text())
    require(outer['image_id'].removeprefix('sha256:') == lock['image_id'].removeprefix('sha256:')
            and outer['image_rootfs'] == lock['rootfs'], 'immutable image lock mismatch')
    checked(root / 'tools/safety/glibc241/Containerfile', lock['containerfile_sha256'])
    require(set(outer['references']) == {'less', 'lessecho', 'less-osc8-open.sh'}, 'incomplete reference bundle')
    validate_reference_hashes(outer['references'], root)
    require(outer.get('reference_policy') == reference_policy(root), 'reference qualification policy mismatch')
    for name, digest in outer['references'].items():
        checked(out / 'reference' / name, digest)
    tools = outer['tool_files_sha256']
    require(any(p.endswith('/bin/rustc') for p in tools) and any(p.endswith('/bin/cargo') for p in tools)
            and any(p.endswith('/librustc-nightly_rt.asan.a') for p in tools), 'missing exact sanitizer toolchain')
    for tool, digest in tools.items():
        checked(tool, digest)
    symbolizer = checked(outer['symbolizer']['path'], PINS['symbolizer']['sha256'])
    require(outer['symbolizer']['sha256'] == PINS['symbolizer']['sha256']
            and outer['symbolizer']['target_mount'] is False, 'invalid offline symbolizer binding')
    command = outer['command']
    for argument in ('--network=none', '--read-only', '--cap-drop=all', 'CARGO_NET_OFFLINE=true'):
        require(argument in command, 'container isolation missing: ' + argument)
    mounts = [command[i + 1] for i, value in enumerate(command[:-1]) if value == '-v']
    require(len(mounts) == 5 and {m.rsplit(':', 2)[1] for m in mounts} ==
            {'/input', '/work', '/reference', '/opt/rust', '/cache/registry'}, 'unexpected target mount')
    require(outer['image_id'] in command, 'container command did not use immutable image ID')
    checked(out / 'container.log', outer['container_log_sha256'])
    inner_path = checked(work / 'report/result.json', outer['inner_report_sha256'])
    inner = json.loads(inner_path.read_text())
    require(inner == outer['result'], 'altered nested report')
    require(inner['status'] == 'passed' and inner['procedure'] == PROCEDURE and
            inner['source_manifest_sha256'] == outer['source_manifest_sha256'], 'wrong inner snapshot/procedure')
    require(inner['runtime'] == 'glibc 2.41' and inner['header_probe'] == lock['header_probe'], 'wrong runtime or ABI layout')
    require(inner['instrumentation_scope'] == PINS['instrumentation_scope'], 'instrumentation scope changed')
    for name, digest in inner['environment_manifests'].items():
        file = checked(work / 'report' / name, digest)
        if name in lock:
            require(file.read_text() == lock[name], 'image environment differs from locked provenance')
    checked(work / 'report/terminfo.txt', inner['terminfo_sha256'])
    require(inner['terminfo_sha256'] == lock['terminfo_xterm256_sha256'], 'terminal profile changed')
    expected_dependencies = dict(line.split('==') for line in
        (root / 'tools/compat/requirements.txt').read_text().splitlines() if line.strip() and not line.startswith('#'))
    require(inner['python_dependencies'] == expected_dependencies, 'observer dependency mismatch')
    records, names = [], set()
    for item in inner['records']:
        require(item['name'] not in names, 'duplicate inner record')
        names.add(item['name'])
        log = checked(work_path(item['log'], work), item['log_sha256'])
        require(item['status'] == 'passed' and item['original_exit_status'] == item['expected_exit_status'] == 0
                and item['expected_diagnostic'] is None and not DIAGNOSTICS.search(log.read_text(errors='replace')),
                'inner command did not pass cleanly: ' + item['name'])
        if item['name'] == 'build-allocation-counterpart':
            require(item['build_flags'] == dict(RUSTFLAGS=PINS['asan_rustflags']+' '+ALLOC_RUSTFLAGS,
                ASAN_OPTIONS=PINS['asan_options']+':symbolize=0',LSAN_OPTIONS=PINS['lsan_options']),
                'scoped allocation counterpart changed sanitizer flags')
        if item['name'] in ('build-normal','build-injected','build-detectors'):
            require(item['build_flags'] == dict(RUSTFLAGS=PINS['asan_rustflags'],
                ASAN_OPTIONS=PINS['asan_options']+':symbolize=0',LSAN_OPTIONS=PINS['lsan_options']),
                'sanitizer compiler/runtime flags changed')
        records.append(normalize('reference-' + item['name'], log, 'ASAN', root))
    require({'tool-rustc', 'abi-build', 'abi-probe', 'build-normal', 'build-injected', 'build-detectors',
             'instrumentation-normal', 'instrumentation-injected', 'fault-path-tests', 'build-allocation-object',
             'build-allocation-counterpart','bind-allocation-counterpart'} <= names,
            'required sanitizer build or regression command missing')
    compiler = (work / 'report/tool-rustc.log').read_text()
    require(PINS['rustc_commit'] in compiler and 'host: ' + PINS['target'] in compiler, 'nightly compiler mismatch')
    for name in ('normal', 'injected'):
        require('__asan_' in (work / ('report/instrumentation-' + name + '.log')).read_text(), 'ASAN instrumentation absent')
    objects = {}
    for module, entry in inner['objects'].items():
        local = checked(work_path(entry['path'], work), entry['sha256'])
        objects[module] = dict(host_path=str(local), sha256=entry['sha256'])
    controls = inner['detector_controls']
    require(len(controls) == 6 and {c['name'] for c in controls} ==
            {t + '-' + b for t in ('pipe', 'pty') for b in ('safe', 'asan-uaf', 'lsan-leak')},
            'pipe/PTY detector control selection incomplete')
    observed_symbolization = []
    for control in controls:
        name = control['name']
        binary = name.split('-', 1)[1]
        expected = {'safe': 0, 'asan-uaf': 87, 'lsan-leak': 87}[binary]
        suffix = {'safe': None, 'asan-uaf': 'asan_uaf.rs', 'lsan-leak': 'lsan_leak.rs'}[binary]
        pattern = {'safe': None, 'asan-uaf': r'ERROR: AddressSanitizer: heap-use-after-free',
                   'lsan-leak': r'ERROR: LeakSanitizer: detected memory leaks.*8192 byte'}[binary]
        observation_path = checked(work_path(control['observation'], work), control['observation_sha256'])
        observation = json.loads(observation_path.read_text())
        require(control['status'] == 'passed' and control['original_exit_status'] == expected and
                control['expected_exit_status'] == expected and observation['status'] == expected and
                observation['trace_hex'] == '' and control['expected_diagnostic'] == pattern and
                control['required_source_suffix'] == suffix, 'detector observation or contract mismatch')
        if name.startswith('pty-'):
            require(observation.get('termios_restored') is True, 'detector PTY terminal state not restored')
        require(control['object'] in objects and control['object_sha256'] == objects[control['object']]['sha256'],
                'detector object identity mismatch')
        raw_text, artifacts = [], {str(observation_path): sha(observation_path)}
        for raw in control['raw_reports']:
            require(suffix is not None, 'safe control produced a raw sanitizer log')
            log = checked(work_path(raw['path'], work), raw['sha256'])
            artifacts[str(log)] = sha(log)
            raw_text.append(log.read_text(errors='replace'))
            resolved = symbolize(log, objects, symbolizer, PINS['symbolizer'])
            require_source(resolved, suffix)
            observed_symbolization.append(dict(control=name, **resolved))
        require(bool(raw_text) == bool(pattern), 'safe/positive detector raw log selection mismatch')
        require(not pattern or re.search(pattern, '\n'.join(raw_text), re.I | re.S), 'positive detector diagnostic missing')
        labels = ['harness-sanitizer-capture-control']
        if binary != 'safe':
            labels += ['asan-detector-control' if binary == 'asan-uaf' else 'lsan-detector-control']
        log = work_path(control['raw_reports'][0]['path'], work) if raw_text else observation_path
        records.append(normalize('reference-control-' + name, log, 'LSAN' if binary == 'lsan-leak' else 'ASAN', root,
            expected=expected, original=expected, diagnostic=pattern, controls=labels, artifacts=artifacts))
    require(observed_symbolization == outer['offline_symbolization'], 'offline symbolization no longer reproduces')
    # This bit is derived only after every safe/UAF/leak pipe and PTY record has
    # passed raw diagnostic, terminal restoration, binary and symbolization checks.
    for record in records:
        if 'harness-sanitizer-capture-control' in record['oracle_metadata']['m0_controls']:
            record['_validated_detector_capture'] = True
    require(not inner['unexpected_sanitizer_reports'] and not list((work / 'report/unexpected-sanitizer').iterdir()),
            'unexpected sanitizer output exists')
    configuration = json.loads((work / 'report/instrumentation.json').read_text())
    require(configuration == dict(ASAN_OPTIONS=PINS['asan_options'] + ':symbolize=0:log_path=/work/report/unexpected-sanitizer/raw',
                                  LSAN_OPTIONS=PINS['lsan_options']), 'fault instrumentation options changed')
    oracle_path = checked(work / 'report/fault-oracles.json', inner['oracle_report_sha256'])
    alloc_binary=validate_allocation_binding(inner,work,root,'asan')
    fault_oracles=json.loads(oracle_path.read_text())
    from native_callback import validate_observations
    validate_observations(fault_oracles, lambda binary: work_path(binary, work))
    allowed_binaries={'/work/normal/'+PINS['target']+'/debug/rless',
                      '/work/injected/'+PINS['target']+'/debug/rless','/reference/less',alloc_binary}
    require(all(observation['binary'] in allowed_binaries for row in fault_oracles['records']
                for observation in row['observations']), 'fault oracle selected an unexpected binary')
    require(json.loads(oracle_path.read_text())['repetitions']==outer['repetitions'],
            'executed fault repetitions differ from selected campaign')
    records.extend(fault_records(oracle_path, work / 'report/fault-path-tests.log', root, 'ASAN'))
    for record in records:
        record['_validated_m0_controls']=list(record['oracle_metadata']['m0_controls'])
    return dict(schema_version=1, source_commit=outer['source_commit'], input_fingerprints=fingerprint(root),
                tool_pins=PINS, tool_files_sha256=tools, records=records,
                status='passed', runtime_safety_verified=False)


def fault_records(oracle_path, log, root, profile):
    records=[]
    oracles = json.loads(oracle_path.read_text())
    from test_fault_paths import validate_oracle_report
    validated = validate_oracle_report(oracles)
    text = log.read_text(errors='replace')
    require(re.search(r'Ran ' + str(oracles['tests_run']) + r' tests? in ', text)
            and re.search(r'^OK\s*$', text, re.M), 'unittest completion not retained')
    for selector, observed in validated.items():
        method = selector.rsplit('.', 1)[-1]
        block = re.search(r'^' + re.escape(method) + r' \([^\n]*\) \.\.\. (.*?)(?=^test_[a-zA-Z0-9_]+ \(|^-{3,}|\Z)', text, re.M | re.S)
        require(block is not None and re.search(r'\bok\s*\Z', block.group(1))
                and not re.search(r'^(?:FAIL|ERROR|skipped)\b', block.group(1), re.M),
                'test method did not pass in original output: ' + method)
        record = normalize('reference-fault-' + method, log, profile, root,
                           artifacts={str(oracle_path): sha(oracle_path)})
        record['observed_hooks'] = observed
        record['oracle_metadata']['test_selector'] = selector
        record['oracle_metadata']['repetitions'] = oracles['repetitions']
        record['oracle_report'] = dict(path=str(oracle_path), sha256=sha(oracle_path))
        validate_oracle_metadata(record['oracle_metadata'])
        contract = next(item['contract'] for item in oracles['records'] if item['test_selector'] == selector)
        record['oracle_metadata'].update(hook_sites=contract['hook_sites'],
            hook_ordinals=contract['hook_ordinals'], m0_controls=contract['m0_controls'])
        record.update(scenario_id=contract['scenario_id'], variant=contract['variant'],
                      finding_ids=contract.get('finding_ids', []))
        if record['finding_ids']:
            record['status'] = 'finding-open'
        records.append(record)
    return records


def validate_native_report(path, root=ROOT):
    import sys
    sys.path.insert(0, str(Path(root) / 'tools'))
    from release import validate_verification, validate_acceptance, commit_manifest, verify_export
    root, path = Path(root).resolve(), Path(path).resolve()
    outer = json.loads(path.read_text())
    out, work = path.parent, path.parent / 'work'
    require(outer['status'] == 'passed' and outer['original_exit_status'] == 0, 'native campaign failed')
    snapshot = outer.get('source_manifest')
    if snapshot:
        validate(snapshot, root)
        require(set(snapshot['files']) == set(source_names(root)), 'native current source selection changed')
        require(snapshot['base_commit'] == outer['source_commit'], 'native base commit mismatch')
        manifest_path = checked(out / 'input/source-manifest.json', outer['source_manifest_sha256'])
        require(json.loads(manifest_path.read_text()) == snapshot, 'native snapshot manifest mismatch')
        files = snapshot['files']
    else:
        manifest_path = checked(out / 'input/release-source-manifest.json', outer['release_source_manifest_sha256'])
        committed = json.loads(manifest_path.read_text())
        require(committed == commit_manifest(root, outer['source_commit']), 'committed export differs from Git objects')
        verify_export(root, committed)
        files = committed['files']
    archive_path = checked(out / 'input/source.tar', outer['source_tar_sha256'])
    with tarfile.open(archive_path) as archive:
        members = [m for m in archive.getmembers() if not m.isdir()]
        require(len(members) == len(files) and {m.name for m in members} == set(files), 'native source archive selection mismatch')
        for member in members:
            require(member.isfile() and (0o755 if member.mode & 0o111 else 0o644) == files[member.name]['mode'] and
                    hashlib.sha256(archive.extractfile(member).read()).hexdigest() == files[member.name]['sha256'],
                    'native source archive bytes/mode mismatch')
    lock_path = checked(root / 'docs/verification/glibc241-environment.json', outer['environment_lock_sha256'])
    lock = json.loads(lock_path.read_text())
    require(outer['image_id'].removeprefix('sha256:') == lock['image_id'].removeprefix('sha256:')
            and outer['image_rootfs'] == lock['rootfs'], 'native image lock mismatch')
    checked(root / 'tools/safety/glibc241/Containerfile', lock['containerfile_sha256'])
    require(outer['tool_pins'] == PINS, 'native tool pins changed')
    for tool, digest in outer['tool_files_sha256'].items():
        checked(tool, digest)
    require(any(name.endswith('/bin/rustc') for name in outer['tool_files_sha256']) and
            any(name.endswith('/bin/cargo') for name in outer['tool_files_sha256']), 'native toolchain identity missing')
    for name, digest in outer['environment_manifests'].items():
        item = checked(out / name, digest)
        if name in lock:
            require(item.read_text() == lock[name], 'native environment changed')
    require(set(outer['references']) == {'less', 'lessecho', 'less-osc8-open.sh'}, 'incomplete native reference bundle')
    validate_reference_hashes(outer['references'], root)
    require(outer.get('reference_policy') == reference_policy(root), 'reference qualification policy mismatch')
    for name, digest in outer['references'].items():
        checked(out / 'reference' / name, digest)
    for flag in ('--network=none', '--read-only', '--cap-drop=all', 'CARGO_NET_OFFLINE=true'):
        require(flag in outer['command'], 'missing native container isolation')
    mounts = [outer['command'][i+1] for i, value in enumerate(outer['command'][:-1]) if value == '-v']
    require(len(mounts) == 5 and {m.rsplit(':', 2)[1] for m in mounts} ==
            {'/input', '/work', '/reference', '/opt/rust', '/cache/registry'}, 'unexpected native target mount')
    require(outer['image_id'] in outer['command'], 'native container command lacks immutable image')
    container_log = checked(out / 'container.log', outer['container_log_sha256'])
    inner_path = checked(work / 'report/result.json', outer['inner_report_sha256'])
    inner = json.loads(inner_path.read_text())
    require(inner == outer['result'] and inner['status'] == 'passed' and inner['runtime'] == 'glibc 2.41'
            and inner['header_probe'] == lock['header_probe'], 'native target identity mismatch')
    for target, digest in inner['evidence_files'].items():
        checked(work_path(target, work), digest)
    environment = json.loads((work / 'report/environment.json').read_text())
    require('rustc ' + PINS['normal_rust_release'] in environment['versions']['rustc']
            and 'host: ' + PINS['target'] in environment['versions']['rustc'], 'native compiler identity mismatch')
    require(sha(work/'report/terminfo.txt') == lock['terminfo_xterm256_sha256'], 'native terminal profile changed')
    dependencies=dict(line.split('==') for line in (root/'tools/compat/requirements.txt').read_text().splitlines()
                      if line.strip() and not line.startswith('#'))
    require(environment['python_dependencies']==dependencies,'native observer dependency mismatch')
    require(environment['packages']==lock['packages.tsv'],'native package manifest mismatch')
    require(environment['runtime'] == inner['runtime'] and environment['header_probe'] == inner['header_probe'],
            'native environment observation mismatch')
    for module, item in inner['objects'].items():
        checked(work_path(item['path'], work), item['sha256'])
    for name, item in inner['libraries'].items():
        binary = out / 'reference/less' if name == 'reference' else work / ('source/target/' + name + '/rless')
        checked(binary, item['binary_sha256'])
        require(item['libraries_sha256'] and all(inner['objects'][module]['sha256'] == digest
                for module, digest in item['libraries_sha256'].items()), 'native loaded object identity mismatch')
    records, by_name = [], {}
    for item in inner['records']:
        require(item['name'] not in by_name, 'duplicate native command')
        by_name[item['name']] = item
        log = checked(work_path(item['log'], work), item['log_sha256'])
        require(item['original_exit_status'] == item['expected_exit_status'] == 0
                and not DIAGNOSTICS.search(log.read_text(errors='replace')), 'native command failed or diagnosed')
    require({'abi-build', 'abi-probe', 'native-baseline', 'injection-build', 'injection-tests', 'fault-path-tests',
             'build-allocation-object','build-allocation-counterpart','bind-allocation-counterpart'} <= set(by_name),
            'incomplete native command selection')
    require(by_name['build-allocation-counterpart']['build_flags']==dict(RUSTFLAGS=ALLOC_RUSTFLAGS),
            'native allocation counterpart compiler flags changed')
    alloc_binary=validate_allocation_binding(inner,work,root,'native')
    summary_path = work_path(inner['native_summary_path'], work)
    summary = json.loads(summary_path.read_text())
    require(summary == inner['native_summary'], 'native verification summary changed')
    require(sha(summary_path) == inner['evidence_files']['/work/report/native-summary.json'],
            'native verification summary hash mismatch')
    validate_verification(summary, summary_path, work / 'source', require_clean=snapshot is None)
    for profile,item in summary['binaries'].items():
        binary=checked(work_path(item['path'],work),item['sha256'])
        checked(binary.parent/'lessecho',item['companion_sha256'])
    checked(summary_path.parent/'installed/bin/rless',summary['installed']['sha256'])
    checked(summary_path.parent/'installed/bin/lessecho',summary['installed']['companion_sha256'])
    checked(work/'source/vendor/less-helpers/less-osc8-open',summary['helpers']['osc8_sha256'])
    require(summary['reference']['sha256']==outer['references']['less'], 'native reference selection mismatch')
    records.append(normalize('reference-native-provenance', work / 'report/environment.json', 'ABI', root,
                             controls=['native-provenance']))
    records.append(normalize('reference-abi-layout-probes', work / 'report/abi-probe.log', 'ABI', root,
                             controls=['abi-layout-probes']))
    for lane in ('acceptance-debug', 'acceptance-release', 'reference', 'acceptance-installed'):
        control = 'acceptance-reference' if lane == 'reference' else lane
        data = json.loads((summary_path.parent / lane / 'summary.json').read_text())
        validate_acceptance(data)
        record = normalize('reference-' + control, summary_path.parent / (lane + '.log'), 'NATIVE', root, controls=[control])
        record['oracle_metadata'].update(executed_cases=809, passed_cases=809, failed_cases=0)
        records.append(record)
    for lane in ('helpers', 'integrations', 'job-control'):
        for profile in ('debug', 'release'):
            records.append(normalize('reference-' + lane + '-' + profile,
                summary_path.parent / (lane + '-' + profile + '.log'), 'NATIVE', root, controls=[lane]))
    if snapshot:
        require(inner['packages'] is None and inner['source_acceptance'] is None and
                inner['package_status'] == 'not-run-uncommitted-snapshot', 'dirty snapshot misrepresented as release')
    else:
        require({'packages', 'source-tests', 'source-build', 'source-acceptance'} <= set(by_name) and inner['package_status'] == 'passed',
                'committed source rebuild commands missing')
        # Relocate retained path references, preserving all embedded evidence
        # bytes. Require the same archive/attestation checks as the host lane.
        from host_evidence import validate_package, acceptance_observer
        package = dict(inner['packages'])
        for field in ('archive', 'source_archive', 'release_attestation'):
            package[field] = str(work_path(package[field], work))
        validate_package(dict(packages=package,
            native_summary=dict(path=str(summary_path), sha256=sha(summary_path))),
            work, work / 'source', committed, summary)
        source_cases = json.loads((work / 'report/source-acceptance/summary.json').read_text())
        require(source_cases == inner['source_acceptance'], 'source acceptance changed')
        validate_acceptance(source_cases)
        acceptance_observer(source_cases, work / 'source')
        rebuilt=list((work/'rebuild').iterdir())
        require(len(rebuilt)==1,'ambiguous source rebuild root')
        checked(rebuilt[0]/'target/release/rless',source_cases['sha256'])
        record = normalize('reference-acceptance-source-rebuild', work / 'report/source-acceptance.log', 'NATIVE', root,
                           controls=['acceptance-source-rebuild'])
        record['oracle_metadata'].update(executed_cases=809, passed_cases=809, failed_cases=0)
        records.append(record)
    for target,digest in inner['test_binaries'].items():
        checked(work_path(target,work),digest)
    expected_test_binaries={'/work/source/target/debug/rless','/work/injected/debug/rless',
                            '/work/cleanup.so','/work/short-write.so','/work/native-io.so',alloc_binary,'/work/alloc-fault.o'}
    require(set(inner['test_binaries'])==expected_test_binaries,'native fault binary manifest incomplete')
    fault_oracles=json.loads((work/'report/fault-oracles.json').read_text())
    from native_callback import validate_observations
    validate_observations(fault_oracles, lambda binary: work_path(binary, work))
    require(all(observation['binary'] in {'/work/source/target/debug/rless','/work/injected/debug/rless','/reference/less',alloc_binary}
                for row in fault_oracles['records'] for observation in row['observations']),
            'native fault oracle selected an unexpected binary')
    require(fault_oracles['repetitions']==100,
            'native actual fault suite did not run the declared 100 repetitions')
    records.extend(fault_records(work / 'report/fault-oracles.json', work / 'report/fault-path-tests.log', root, 'NATIVE'))
    for record in records:
        record['_validated_m0_controls']=list(record['oracle_metadata']['m0_controls'])
    return dict(schema_version=1, source_commit=outer['source_commit'], input_fingerprints=fingerprint(root),
                tool_pins=PINS, tool_files_sha256=outer['tool_files_sha256'], records=records,
                status='passed', runtime_safety_verified=False)


def validate_allocation_binding(inner, work, root, profile):
    builds=[record for record in inner['records'] if record['name']=='build-allocation-counterpart']
    expected_command=['cargo','rustc','--locked','--offline','--bin','rless']
    if profile=='asan':
        expected_command += ['--target',PINS['target']]
    expected_command += ['--','-Clink-arg=/work/alloc-fault.o',
                         '-Clink-arg=-Wl,--wrap=calloc','-Clink-arg=-Wl,--wrap=strdup']
    require(len(builds)==1 and builds[0]['command']==expected_command,
            'allocation/parser counterpart link command changed')
    reference=inner['allocation_binding']
    binding_path=checked(work_path(reference['path'],work),reference['sha256'])
    binding=json.loads(binding_path.read_text())
    expected='/work/alloc-counterpart/'+(PINS['target']+'/' if profile=='asan' else '')+'debug/rless'
    require(binding['binary']==expected and binding['profile']==profile,'wrong allocation counterpart selection')
    validate_retained_binding(binding,work_path(expected,work))
    object_record=json.loads((work/'report/alloc-object.json').read_text())
    require(object_record['path']=='/work/alloc-fault.o' and
            object_record['source_sha256']==sha(root/'tools/safety/native/alloc_fault.c'),
            'native allocation object/source identity mismatch')
    checked(work/'alloc-fault.o',object_record['sha256'])
    require(object_record['command']==['cc',*ALLOC_CFLAGS,'-c','/work/source/tools/safety/native/alloc_fault.c',
                                       '-o','/work/alloc-fault.o'], 'native allocation object flags changed')
    return expected


def validate_external_report(path, root=ROOT):
    procedure = json.loads(Path(path).read_text()).get('procedure')
    if procedure == PROCEDURE:
        return validate_asan_report(path, root)
    if procedure == 'REFERENCE-GLIBC-2.41-NATIVE-v1':
        return validate_native_report(path, root)
    raise ValueError('unsupported reference ABI report procedure')
