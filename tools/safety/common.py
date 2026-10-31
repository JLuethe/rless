"""Shared fingerprints and supervised evidence capture (Python standard library)."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
PINS = json.loads((Path(__file__).parent / 'profiles.json').read_text())
DIAGNOSTICS = re.compile(
    r'(?:ERROR: (?:AddressSanitizer|LeakSanitizer)|SUMMARY: (?:AddressSanitizer|LeakSanitizer)|'
    r'AddressSanitizer:DEADLYSIGNAL|error: Undefined Behavior|runtime error:)', re.I)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode='w', dir=path.parent, delete=False) as out:
        json.dump(value, out, indent=2)
        out.write('\n')
        name = out.name
    os.replace(name, path)


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def git(*args, root=ROOT):
    return subprocess.check_output(['git', '-C', str(root), *args])


def fingerprint(root=ROOT):
    """Include tracked and newly added inputs; ignored run outputs cannot enter it."""
    paths = git('ls-files', '-z', '--cached', '--others', '--exclude-standard', root=root).split(b'\0')
    files = {}
    for raw in sorted(set(paths)):
        if not raw:
            continue
        name = os.fsdecode(raw)
        if name in ('spec/safety/validation.json', 'spec/safety/github-tracking.json'):
            continue  # Generated structure report and external tracking references.
        if not (name.startswith(('src/', 'tests/', 'tools/', 'vendor/', 'compat/', 'spec/'))
                or name in ('.gitignore', 'Cargo.toml', 'Cargo.lock', 'build.rs',
                            'docs/safety/findings.json')):
            continue
        path = root / name
        if path.is_file():
            files[name] = sha(path)
        else:
            files[name] = 'MISSING'
    digest = hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest()
    return {'sha256': digest, 'files': files}


def environment(nightly=False, tool_home=None):
    """Do not inherit arbitrary Rust, sanitizer, Miri, pager or loader settings."""
    env = {k: os.environ[k] for k in ('HOME', 'PATH', 'TMPDIR') if k in os.environ}
    env.update(LANG='C.UTF-8', LC_ALL='C.UTF-8', TERM='xterm-256color',
               CARGO_HOME=os.environ.get('CARGO_HOME', str(Path.home() / '.cargo')),
               CARGO_BUILD_JOBS='2')
    if nightly:
        if tool_home is None:
            raise ValueError('--tool-home is required for the isolated pinned nightly')
        tool_home = Path(tool_home).resolve()
        env['RUSTUP_HOME'] = str(tool_home / 'rustup')
        env['RUSTUP_TOOLCHAIN'] = PINS['nightly']
        env['PATH'] = str(tool_home / 'cargo/bin') + os.pathsep + env.get('PATH', '')
    return env


def validate_oracle_metadata(value):
    """Check the executed oracle identity, not an assertion that its coverage is complete."""
    if value is None:
        return
    required = {'abi_profile', 'corpus_sha256', 'seed', 'test_selector',
                'hook_sites', 'hook_ordinals'}
    if not isinstance(value, dict) or not required <= value.keys():
        raise ValueError('incomplete executed oracle metadata')
    if value['abi_profile'] not in {x['id'] for x in PINS['abi_profiles']}:
        raise ValueError('unknown executed ABI profile')
    if not isinstance(value['corpus_sha256'], str) or not re.fullmatch(r'[0-9a-f]{64}', value['corpus_sha256']):
        raise ValueError('missing executed corpus hash')
    if value['seed'] is not None and (type(value['seed']) is not int or value['seed'] < 0):
        raise ValueError('invalid executed seed')
    if not isinstance(value['test_selector'], str) or not value['test_selector'].strip():
        raise ValueError('missing executed test selector')
    sites, ordinals = value['hook_sites'], value['hook_ordinals']
    if not isinstance(sites, list) or any(
            not isinstance(s, str) or not re.fullmatch(r'[a-z][a-z0-9-]*', s) for s in sites) or len(set(sites)) != len(sites):
        raise ValueError('invalid executed hook sites')
    if not isinstance(ordinals, list) or any(
            type(n) is not int or n < 1 for n in ordinals) or len(set(ordinals)) != len(ordinals):
        raise ValueError('invalid executed hook ordinals')
    # Reject values that cannot round-trip through the stored JSON evidence.
    if json.loads(json.dumps(value, sort_keys=True)) != value:
        raise ValueError('oracle metadata is not JSON stable')


def validate_fault_oracle_evidence(item, root=ROOT, *, external=False):
    """Re-derive actual marker observations; declaration metadata is not hit proof."""
    metadata = item.get('oracle_metadata') or {}
    selector = metadata.get('test_selector', '')
    reference = item.get('oracle_report')
    if reference is None:
        if selector.startswith('ActualFaultPaths.') or 'observed_hooks' in item:
            raise ValueError('fault oracle lacks a retained observation report')
        return None
    if not isinstance(reference, dict) or set(reference) != {'path', 'sha256'}:
        raise ValueError('fault observation reference requires path and sha256')
    path = Path(reference['path'])
    path = path if path.is_absolute() else root / path
    if not path.is_file() or sha(path) != reference['sha256']:
        raise ValueError('missing or changed fault observation report')
    if not selector.startswith('ActualFaultPaths.'):
        raise ValueError('unknown fault oracle test selector')
    # This module performs pure JSON/trace validation here; no tests execute.
    from test_fault_paths import contracts, validate_oracle_report
    report = json.loads(path.read_text())
    validated = validate_oracle_report(report, expected_selectors=None if external else [selector])
    if selector not in validated:
        raise ValueError('selected fault oracle was not executed')
    declaration = contracts().get(selector.removeprefix('ActualFaultPaths.'))
    if declaration is None:
        raise ValueError('fault oracle contract is no longer declared')
    for field in ('hook_sites', 'hook_ordinals', 'm0_controls'):
        if metadata.get(field) != declaration[field]:
            raise ValueError('fault metadata differs from the executed current contract: ' + field)
    for field in ('scenario_id', 'variant'):
        if item.get(field) != declaration[field]:
            raise ValueError('fault evidence changed its acceptance identity')
    if item.get('finding_ids', []) != declaration['finding_ids']:
        raise ValueError('fault evidence changed its retained legacy findings')
    if metadata.get('repetitions') != report.get('repetitions'):
        raise ValueError('fault repetition count differs from executed observations')
    if not external and selector not in item.get('command', []):
        raise ValueError('per-test command did not select the declared oracle')
    observed = validated[selector]
    if item.get('observed_hooks') != observed:
        raise ValueError('copied hook observations differ from retained marker bytes')
    return observed


def validate_detector_capture_evidence(item, root=ROOT):
    """Validate actual retained pipe/PTY detector output, never its status flag."""
    reference = item.get('detector_capture_report')
    selector = (item.get('oracle_metadata') or {}).get('test_selector')
    if reference is None:
        if selector == 'harness_controls.validate_capture':
            raise ValueError('detector capture oracle lacks its retained report')
        return False
    if not isinstance(reference, dict) or set(reference) != {'path', 'sha256'}:
        raise ValueError('detector capture reference requires path and sha256')
    path = Path(reference['path'])
    path = path if path.is_absolute() else root / path
    if not path.is_file() or sha(path) != reference['sha256']:
        raise ValueError('missing or changed detector capture report')
    if selector != 'harness_controls.validate_capture':
        raise ValueError('unknown detector capture selector')
    from harness_controls import validate_capture
    validate_capture(json.loads(path.read_text()))
    return True


def is_external_report(report):
    if report.get('report_kind') in ('m0-controls', 'host-native-baseline'):
        return True
    return ('input_fingerprints' not in report and (
        report.get('source_kind') == 'explicit-working-tree-snapshot' or
        (report.get('source_kind') == 'committed-source' and
         report.get('procedure') == 'REFERENCE-GLIBC-2.41-NATIVE-v1')))


class Evidence:
    def __init__(self, directory, root=ROOT):
        self.directory = Path(directory).resolve()
        self.directory.mkdir(parents=True, exist_ok=True)
        self.root = root
        self.report = dict(schema_version=1, started_at_utc=utc(), status='not-verified',
                           scope='M0 infrastructure/diagnostics; does not close migration requirements.',
                           source_commit=git('rev-parse', 'HEAD', root=root).decode().strip(),
                           dirty_state=git('status', '--porcelain', root=root).decode(),
                           input_fingerprints=fingerprint(root), records=[], findings=[])
        self.save()

    def save(self):
        write_json(self.directory / 'summary.json', self.report)

    def run(self, name, command, *, env=None, cwd=None, timeout=300, expected=0,
            diagnostic=None, requirement_ids=(), scenario_id=None, variant=None,
            profile='NATIVE', artifacts=(), coverage_scope=None, oracle_metadata=None,
            finding_ids=()):
        if coverage_scope not in (None,'partial','complete'):
            raise ValueError('unknown executed coverage scope')
        if not command:
            raise ValueError('empty command selection')
        validate_oracle_metadata(oracle_metadata)
        if oracle_metadata is not None:
            oracle_metadata = json.loads(json.dumps(oracle_metadata))
        finding_ids = list(finding_ids)
        if any(not isinstance(value, str) or not value.strip() for value in finding_ids) or len(set(finding_ids)) != len(finding_ids):
            raise ValueError('invalid diagnostic finding identities')
        if finding_ids and diagnostic is not None:
            raise ValueError('detector controls cannot be legacy application findings')
        if not re.fullmatch(r'[A-Za-z0-9_.-]+', name):
            raise ValueError('unsafe evidence record name')
        if any(x['name'] == name for x in self.report['records']):
            raise ValueError('duplicate evidence record name')
        if fingerprint(self.root) != self.report['input_fingerprints']:
            raise ValueError('source/tool/spec inputs changed before execution')
        before = {str(Path(p).resolve()): sha(p) for p in artifacts}
        started = time.monotonic()
        cpu = resource.getrusage(resource.RUSAGE_CHILDREN)
        log_path = self.directory / (name + '.log')
        timed_out = False
        with log_path.open('wb') as log:
            process = subprocess.Popen([str(x) for x in command], cwd=cwd or self.root,
                                       env=env or environment(), stdout=log,
                                       stderr=subprocess.STDOUT, start_new_session=True)
            try:
                status = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                timed_out = True
                os.killpg(process.pid, signal.SIGKILL)
                status = process.wait()
        usage = resource.getrusage(resource.RUSAGE_CHILDREN)
        output = log_path.read_text(errors='replace')
        has_finding = bool(DIAGNOSTICS.search(output))
        # An expected nonzero application exit can never conceal a sanitizer report.
        passed = not timed_out and status == expected and not has_finding
        if diagnostic is not None:
            passed = (not timed_out and status != 0 and
                      re.search(diagnostic, output, re.I | re.S) is not None)
        stale = (fingerprint(self.root) != self.report['input_fingerprints'] or
                 any(not Path(p).is_file() or sha(p) != h for p, h in before.items()))
        if stale:
            passed = False
        record = dict(name=name, requirement_ids=list(requirement_ids), scenario_id=scenario_id,
                      variant=variant or name, profile=profile,
                      coverage_scope=coverage_scope,
                      status=('finding-open' if finding_ids else 'passed') if passed else 'failed',
                      finding_ids=finding_ids, oracle_metadata=oracle_metadata,
                      command=[str(x) for x in command],
                      original_exit_status=status, expected_exit_status=expected,
                      expected_diagnostic=diagnostic, diagnostic_found=has_finding,
                      timed_out=timed_out, stale_inputs=stale,
                      build_flags={k: env[k] for k in ('RUSTFLAGS', 'MIRIFLAGS', 'RUSTUP_TOOLCHAIN',
                                   'RUSTUP_HOME', 'CARGO_HOME', 'CARGO_TARGET_DIR', 'PYTHONPATH', 'PATH',
                                   'ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH') if env and k in env},
                      wall_seconds=time.monotonic() - started,
                      cpu_seconds=usage.ru_utime + usage.ru_stime - cpu.ru_utime - cpu.ru_stime,
                      log=str(log_path), log_sha256=sha(log_path), artifact_hashes=before)
        self.report['records'].append(record)
        self.save()
        print(f'{name}: {record["status"]} ({status})', flush=True)
        return record

    def unavailable(self, name, reason, profile, requirement_ids=(), *, kind='infrastructure'):
        if kind != 'infrastructure':
            raise ValueError('unexecuted work is infrastructure, not an observed legacy finding')
        if any(x['name'] == name for x in self.report['records']):
            raise ValueError('duplicate evidence record name')
        self.report['records'].append(dict(name=name, status='unsupported', reason=reason,
                                           gap_kind=kind, profile=profile,
                                           requirement_ids=list(requirement_ids)))
        self.save()

    def finish(self):
        records = self.report['records']
        passed = bool(records) and all(x['status'] == 'passed' for x in records)
        if fingerprint(self.root) != self.report['input_fingerprints']:
            passed = False
            self.report['input_change_error'] = 'source/tool/spec changed during campaign'
        self.report.update(status='passed' if passed else 'not-verified', finished_at_utc=utc(),
                           runtime_safety_verified=False)
        self.save()
        return passed


def validate_report(path, root=ROOT, *, record_names=None, allow_open_findings=False):
    """Re-derive a gate from logs, statuses, selections, artifacts and current inputs."""
    report = json.loads(Path(path).read_text())
    external = is_external_report(report)
    if external:
        # Container evidence has separate pipe/PTY observations and offline raw
        # diagnostics. The dedicated validator rederives them before normalization.
        if report.get('report_kind') == 'm0-controls':
            from m0_controls import validate_external_report
        elif report.get('report_kind') == 'host-native-baseline':
            from host_evidence import validate_external_report
        else:
            from glibc241.evidence import validate_external_report
        report = validate_external_report(Path(path), root=root)
    if report['input_fingerprints'] != fingerprint(root):
        raise ValueError('stale input fingerprint')
    if report.get('tool_pins', PINS) != PINS:
        raise ValueError('tool profile changed')
    for tool, value in report.get('tool_files_sha256', {}).items():
        if not Path(tool).is_file() or sha(tool) != value:
            raise ValueError('stale tool or toolchain source')
    records = report['records']
    if not records:
        raise ValueError('empty selection')
    names = [item['name'] for item in records]
    if len(set(names)) != len(names):
        raise ValueError('duplicate record')
    if record_names is not None:
        record_names = list(record_names)
        if not record_names or len(set(record_names)) != len(record_names) or not set(record_names) <= set(names):
            raise ValueError('empty, duplicate or unknown evidence selection')
        records = [item for item in records if item['name'] in record_names]
    for item in records:
        finding_ids = item.get('finding_ids', [])
        is_finding = item['status'] == 'finding-open'
        if not isinstance(finding_ids, list) or any(not isinstance(value, str) or not value.strip() for value in finding_ids):
            raise ValueError('invalid diagnostic finding identities')
        if is_finding and (not allow_open_findings or not finding_ids):
            raise ValueError('open application finding cannot pass the final safety gate')
        if finding_ids and (not is_finding or len(set(finding_ids)) != len(finding_ids)):
            raise ValueError('finding outcome was promoted or has duplicate identities')
        if item['status'] != 'passed' and not (allow_open_findings and is_finding):
            raise ValueError('required record did not pass: ' + item['name'])
        if (type(item.get('original_exit_status')) is not int
                or type(item.get('expected_exit_status')) is not int):
            raise ValueError('evidence exit statuses must be integers')
        validate_oracle_metadata(item.get('oracle_metadata'))
        metadata = item.get('oracle_metadata')
        if metadata is not None:
            corpus = root / 'compat/corpus.json'
            if not corpus.is_file() or sha(corpus) != metadata['corpus_sha256']:
                raise ValueError('changed executed corpus')
        validate_fault_oracle_evidence(item, root, external=external)
        # Host JSON cannot supply a trusted proof bit. External normalized values
        # originate only from the specialized validator above, which rederives all
        # six detector controls and their retained observations/diagnostics.
        if not external:
            item['_validated_detector_capture'] = validate_detector_capture_evidence(item, root)
            item['_validated_m0_controls'] = (
                ['harness-sanitizer-capture-control'] if item['_validated_detector_capture'] else [])
        if external:
            continue
        log = Path(item['log'])
        if not log.is_file() or sha(log) != item['log_sha256']:
            raise ValueError('missing or altered diagnostic log')
        output = log.read_text(errors='replace')
        if item.get('timed_out') or item.get('stale_inputs'):
            raise ValueError('timed-out or stale record')
        if item['expected_diagnostic']:
            if is_finding:
                raise ValueError('detector control mislabeled as an application finding')
            if item['original_exit_status'] == 0 or not re.search(item['expected_diagnostic'], output, re.I | re.S):
                raise ValueError('expected detector diagnostic missing')
        elif (item['original_exit_status'] != item['expected_exit_status'] or DIAGNOSTICS.search(output)):
            raise ValueError('unexpected exit or safety diagnostic')
        for artifact, value in item['artifact_hashes'].items():
            if not Path(artifact).is_file() or sha(artifact) != value:
                raise ValueError('stale binary artifact')
    return report
