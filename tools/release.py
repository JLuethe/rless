"""Source identity and fail-closed release policy shared by check/package runners.

An exported manifest is provenance supplied by a trusted git-archive runner. It
is not a signature or an independent proof that a remote repository owns a commit.
"""
import base64
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import subprocess


PROVENANCE = '.rless-source.json'
WORKING_PROVENANCE = '.rless-working-tree.json'
SOURCE_DIRS = {'src', 'tests', 'tools', 'vendor', 'docs', 'spec', 'compat', '.github', '.cargo'}
GENERATED_DIRS = {'target', 'dist', '.git', '.venv', '__pycache__', '.runs', 'artifacts', 'corpus', 'coverage'}
LOCAL_CONFIG_DIRS = {'.agents', '.codex', '.aws'}
ROOT_FILES = {'.gitignore', '.gitattributes', 'Cargo.toml', 'Cargo.lock', 'build.rs',
              'rust-toolchain', 'rust-toolchain.toml', 'Makefile', 'Dockerfile'}
ROOT_PREFIXES = ('README', 'LICENSE', 'CONTRIBUTING', 'CHANGELOG', 'SECURITY', 'CODE_OF_CONDUCT')
SEEDS = (712, 20261023)
PROFILES = ('debug', 'release')
FAMILIES = {'options': 230, 'behavior': 467, 'interactions': 108, 'integration': 4}
BASE_STEPS = {'format', 'rust-debug', 'rust-release', 'observer', 'corpus', 'negative-candidate',
              'acceptance-debug', 'acceptance-release', 'reference', 'clean-install', 'acceptance-installed'}
REQUIRED_STEPS = BASE_STEPS | {f'{lane}-{profile}' for profile in PROFILES
    for lane in ('integrations', 'job-control', 'helpers')} | {
    f'stress-{profile}-{seed}' for profile in PROFILES for seed in SEEDS}
INTEGRATIONS = {'git-diff': 1, 'man-local': 0, 'stream-without-eof': 0,
    'terminate-restores-terminal': 15, 'repeated-resize': 0, 'follow-append': 0,
    'follow-rotate': 0, 'truncate-recovery': 0, 'follow-live-pipe': 0, 'sparse-byte-position': 0}
HELPER_URIS = ('file:///tmp/local', 'file://localhost/tmp/local', 'file://this-host/tmp/local',
    'file://remote-host/tmp/local', 'file:invalid', 'file:///tmp/$(should-stay-literal)',
    'man:printf', 'man:printf(3)', 'https://example.invalid/')
HELPER_INVOCATIONS = {0: b'--\n/tmp/local\n', 1: b'--\n/tmp/local\n', 2: b'--\n/tmp/local\n',
    5: b'--\n/tmp/$(should-stay-literal)\n', 6: b'--\nprintf\n', 7: b'--\n3\nprintf\n'}
HELPER_ERRORS = {3: b'cannot open remote file: file://remote-host/tmp/local\n',
    4: b'invalid file link: file:invalid\n', 8: b'unsupported link type: https://example.invalid/\n'}


def integer(value, expected):
    """JSON booleans must never stand in for exit statuses or counts."""
    return type(value) is int and value == expected


def terminal_shape(value):
    if not isinstance(value, dict) or not isinstance(value.get('rows'), list) or not value['rows'] or not all(
            isinstance(row, str) for row in value['rows']):
        raise ValueError('missing terminal observation rows')
    for key in ('modes', 'private_modes'):
        if not isinstance(value.get(key), list) or not all(type(mode) is int for mode in value[key]):
            raise ValueError('invalid terminal observation modes')
    cursor = value.get('cursor')
    if (not isinstance(cursor, list) or len(cursor) != 3 or
            any(type(item) is not int or item < 0 for item in cursor[:2]) or type(cursor[2]) is not bool or
            type(value.get('bells')) is not int or value['bells'] < 0 or
            not isinstance(value.get('styles'), list) or not isinstance(value.get('osc'), list)):
        raise ValueError('invalid terminal observation state')


def pty_observation(value, status):
    if not isinstance(value, dict) or not integer(value.get('status'), status) or value.get('termios_restored') is not True:
        raise ValueError('failed or missing PTY process/terminal outcome')
    terminal_shape(value.get('final_terminal'))
    if not isinstance(value.get('checkpoints'), list):
        raise ValueError('missing PTY checkpoints')
    for checkpoint in value['checkpoints']:
        terminal_shape(checkpoint)


def validate_job_control(report):
    if report.get('passed') is not True or not isinstance(report.get('actual'), dict) or report['actual'] != report.get('reference'):
        raise ValueError('job-control evidence failed or differs from reference')
    for side in ('actual', 'reference'):
        value = report[side]
        if set(value) != {'stopped', 'stopped_terminal', 'resumed_terminal', 'exited'}:
            raise ValueError('incomplete job-control observation')
        stopped, exited = value['stopped'], value['exited']
        if (not isinstance(stopped, dict) or not integer(stopped.get('stopped_by'), 20) or stopped.get('restored') is not True or
                not isinstance(exited, dict) or not integer(exited.get('status'), 0) or exited.get('restored') is not True):
            raise ValueError('job-control did not stop/exit with terminal restoration')
        terminal_shape(value['stopped_terminal'])
        terminal_shape(value['resumed_terminal'])
        if ({1, 1000, 1006, 1049, 2004} & set(value['stopped_terminal']['private_modes']) or
                1049 not in value['resumed_terminal']['private_modes']):
            raise ValueError('job-control terminal modes did not restore/resume')


def validate_supplemental(report, lane, helper_sha256):
    names = set(INTEGRATIONS) if lane == 'integrations' else {'companion-glob'} | {
        f'osc8-helper-{index}' for index in range(len(HELPER_URIS))}
    metadata = set() if lane == 'integrations' else {'helper_sha256'}
    if set(report) != names | metadata or any(not isinstance(report[name], dict) or
            report[name].get('passed') is not True for name in names):
        raise ValueError('incomplete ' + lane + ' case identities/outcomes')
    if lane == 'integrations':
        for name, status in INTEGRATIONS.items():
            pty_observation(report[name].get('actual'), status)
        for name in ('git-diff', 'man-local'):
            pager_log = report[name]['actual'].get('files', {}).get('pager.log', {})
            if pager_log.get('sha256') != hashlib.sha256(b'pager\n').hexdigest():
                raise ValueError('integration did not invoke the selected pager')
        resized = report['repeated-resize']['actual']['checkpoints']
        if len(resized) != 60 or any(not checkpoint['rows'][0].startswith(
                'line 002' if index % 3 == 1 else 'line 001') for index, checkpoint in enumerate(resized)):
            raise ValueError('integration resize/navigation checkpoints incomplete')
        for name, index, marker in [('follow-append', 0, 'APPENDED-TWO'), ('follow-rotate', 0, 'ROTATED-TWO'),
                ('truncate-recovery', 1, 'TRUNCATED-TWO'), ('follow-live-pipe', 0, 'PIPE-APPEND')]:
            checkpoints = report[name]['actual']['checkpoints']
            if len(checkpoints) <= index or not any(marker in row for row in checkpoints[index]['rows']):
                raise ValueError('integration recovery checkpoint missing: ' + name)
        sparse = report['sparse-byte-position']['actual']['checkpoints']
        if not sparse or not sparse[0]['rows'][0].startswith('beyond-4-GiB'):
            raise ValueError('integration sparse seek checkpoint missing')
        return
    if report.get('helper_sha256') != helper_sha256:
        raise ValueError('helper evidence identity mismatch')
    glob = report['companion-glob'].get('actual')
    pty_observation(glob, 0)
    if not glob['checkpoints'] or not glob['checkpoints'][0]['rows'][0].startswith('FIRST MATCH'):
        raise ValueError('companion glob did not reach expected file')
    for index, uri in enumerate(HELPER_URIS):
        case = report[f'osc8-helper-{index}']
        value = case.get('actual', {})
        expected_status = 1 if index in HELPER_ERRORS else 0
        if (case.get('uri') != uri or not isinstance(value, dict) or not integer(value.get('status'), expected_status) or
                value.get('stderr_b64') != '' or
                value.get('stdout_b64') != base64.b64encode(HELPER_ERRORS.get(index, b'')).decode()):
            raise ValueError('helper URI/process outcome mismatch')
        files = value.get('files', {})
        if not isinstance(files, dict) or not {'bin/less', 'bin/rless', 'bin/man'} <= set(files):
            raise ValueError('helper stub file evidence missing')
        invocation = files.get('invocation.log')
        if index in HELPER_INVOCATIONS:
            expected = HELPER_INVOCATIONS[index]
            if (not isinstance(invocation, dict) or invocation.get('sha256') != hashlib.sha256(expected).hexdigest() or
                    not integer(invocation.get('bytes'), len(expected))):
                raise ValueError('helper invocation differs from literal argument contract')
        elif invocation is not None:
            raise ValueError('rejected helper URI unexpectedly invoked a command')


def sha_file(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def manifest_digest(files):
    return hashlib.sha256(json.dumps(files, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def allowed_path(name):
    path = PurePosixPath(name)
    if path.is_absolute() or '..' in path.parts or not path.parts or '\\' in name:
        return False
    if any(part in GENERATED_DIRS | LOCAL_CONFIG_DIRS for part in path.parts):
        return False
    return (len(path.parts) > 1 and path.parts[0] in SOURCE_DIRS) or (
        len(path.parts) == 1 and (name in ROOT_FILES or name.startswith(ROOT_PREFIXES)))


def source_paths(root):
    """Only approved source locations; generated outputs never enter a manifest."""
    result = []
    for parent, directories, files in os.walk(root, followlinks=False):
        relative = Path(parent).relative_to(root)
        directories[:] = sorted(d for d in directories if d not in GENERATED_DIRS | LOCAL_CONFIG_DIRS)
        if relative == Path('.'):
            unknown = set(directories) - SOURCE_DIRS
            if unknown:
                raise ValueError('unapproved source directories: ' + ', '.join(sorted(unknown)))
        for directory in directories:
            if (Path(parent) / directory).is_symlink():
                raise ValueError('symlinked source directory is not supported')
        for filename in sorted(files):
            path = Path(parent) / filename
            name = path.relative_to(root).as_posix()
            if name in (PROVENANCE, WORKING_PROVENANCE, '.git'):
                continue
            if not allowed_path(name):
                raise ValueError('unapproved source file: ' + name)
            if path.is_symlink() or not path.is_file():
                raise ValueError('source must be a regular file: ' + name)
            result.append(name)
    return sorted(result)


def file_identity(path):
    return dict(sha256=sha_file(path), mode=0o755 if path.stat().st_mode & 0o111 else 0o644)


def commit_manifest(repository, commit='HEAD'):
    """Capture deliberate tracked files directly from immutable Git objects."""
    def git(*args):
        return subprocess.check_output(['git', '-C', str(repository), *args])
    resolved = git('rev-parse', commit + '^{commit}').decode().strip()
    files = {}
    for record in git('ls-tree', '-rz', '--full-tree', resolved).split(b'\0'):
        if not record:
            continue
        metadata, encoded_name = record.split(b'\t', 1)
        mode, kind, oid = metadata.decode().split()
        name = os.fsdecode(encoded_name)
        if kind != 'blob' or mode not in ('100644', '100755') or not allowed_path(name):
            raise ValueError('unapproved tracked release input: ' + name)
        content = git('cat-file', 'blob', oid)
        files[name] = dict(sha256=hashlib.sha256(content).hexdigest(), mode=0o755 if mode == '100755' else 0o644)
    if not files:
        raise ValueError('empty committed source manifest')
    return dict(schema_version=1, origin='trusted-git-export', source_commit=resolved,
                files=files, manifest_sha256=manifest_digest(files))


def verify_export(root, manifest):
    if manifest.get('schema_version') != 1 or manifest.get('origin') != 'trusted-git-export':
        raise ValueError('unsupported committed source provenance')
    commit = manifest.get('source_commit', '')
    if len(commit) not in (40, 64) or any(c not in '0123456789abcdef' for c in commit):
        raise ValueError('invalid source commit identity')
    files = manifest.get('files', {})
    if not files or manifest.get('manifest_sha256') != manifest_digest(files):
        raise ValueError('empty or changed source manifest')
    if set(source_paths(root)) != set(files):
        raise ValueError('source tree contains missing or unapproved/untracked files')
    for name, expected in files.items():
        if not allowed_path(name) or file_identity(root / name) != expected:
            raise ValueError('source file changed since committed export: ' + name)
    return manifest


def write_export_manifest(root, manifest):
    """Called by a trusted exporter after extracting the exact committed source."""
    verify_export(root, manifest)
    path = root / PROVENANCE
    path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n')
    return path


def source_identity(root, require_clean=False):
    root = Path(root).resolve()
    try:
        result = subprocess.run(['git', '-C', str(root), 'rev-parse', '--show-toplevel'],
                                stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        own_git = result.returncode == 0 and Path(result.stdout.strip()).resolve() == root
    except FileNotFoundError:
        own_git = False
    if own_git:
        dirty = subprocess.check_output(['git', '-C', str(root), 'status', '--porcelain=v1',
                                         '--untracked-files=all']).decode()
        if dirty and require_clean:
            raise ValueError('release requires a clean committed tree; untracked inputs are not approved')
        if not dirty:
            manifest = commit_manifest(root)
            verify_export(root, manifest)
            return dict(**manifest, clean=True, kind='committed-checkout')
        files = {name: file_identity(root / name) for name in source_paths(root)}
        commit = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD']).decode().strip()
        return dict(schema_version=1, kind='working-tree', clean=False, source_commit=commit,
                    dirty_state=dirty, files=files, manifest_sha256=manifest_digest(files))
    provenance = root / PROVENANCE
    working = root / WORKING_PROVENANCE
    if working.is_file():
        if require_clean or provenance.exists():
            raise ValueError('explicit working-tree snapshots cannot qualify a release')
        value = json.loads(working.read_text())
        files = value.get('files', {})
        if value.get('schema_version') != 1 or value.get('source_kind') != 'explicit-working-tree-snapshot' or not files:
            raise ValueError('unsupported working-tree snapshot provenance')
        expected = hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest()
        if value.get('files_sha256') != expected or set(source_paths(root)) != set(files):
            raise ValueError('working-tree snapshot manifest mismatch')
        for name, metadata in files.items():
            if not allowed_path(name) or file_identity(root / name) != metadata:
                raise ValueError('working-tree snapshot source changed: ' + name)
        return dict(schema_version=1, kind='working-tree-export', clean=False,
                    source_commit=value['base_commit'], dirty_state=value['dirty_state'],
                    files=files, manifest_sha256=manifest_digest(files))
    if not provenance.is_file():
        raise ValueError('non-Git checkouts require provenance from the trusted committed-source exporter')
    manifest = verify_export(root, json.loads(provenance.read_text()))
    return dict(**manifest, clean=True, kind='committed-export')


def validate_acceptance(data):
    if not all(integer(data.get(key), count) for key, count in [('executed', 809), ('passed', 809), ('failed', 0)]):
        raise ValueError('incomplete 809-case acceptance evidence')
    if set(data.get('families', {})) != set(FAMILIES):
        raise ValueError('missing acceptance family')
    for name, count in FAMILIES.items():
        family = data['families'][name]
        if not all(integer(family.get(key), number) for key, number in [('executed', count), ('passed', count), ('failed', 0)]) or family.get('errors') != []:
            raise ValueError('failed or incomplete acceptance family: ' + name)


def validate_acceptance_observer(data, source):
    source = Path(source)
    expected = {
        'observer_sha256': sha_file(source / 'tools/compat/run_v2.py'),
        'observer_revision_sha256': sha_file(source / 'compat/observer-v2.json'),
    }
    if any(data.get(field) != digest for field, digest in expected.items()):
        raise ValueError('acceptance observer identity mismatch')


def validate_verification(record, path, root, require_clean=True):
    """Re-derive complete native qualification; packaging always requires clean.

    ABI diagnostics may explicitly allow a hash-bound working-tree snapshot.
    That return value retains clean=False and cannot authorize packaging.
    """
    if not integer(record.get('schema_version'), 2) or record.get('passed') is not True:
        raise ValueError('a current successful schema-2 check report is required; historical reports remain historical')
    source = source_identity(root, require_clean=require_clean)
    recorded = record.get('source', {})
    if type(recorded.get('clean')) is not bool or (require_clean and recorded.get('clean') is not True) or any(recorded.get(key) != source.get(key)
            for key in ('source_commit', 'manifest_sha256', 'files', 'clean', 'dirty_state')):
        raise ValueError('verification does not match the required source identity')
    selection = record.get('selection', {})
    rounds = selection.get('stress_rounds')
    if selection.get('reference') is not True or selection.get('install_check') is not True or type(rounds) is not int or rounds < 100:
        raise ValueError('release requires reference, install and at least 100 stress rounds')
    identities = [record.get('reference', {}).get('sha256'), record.get('helpers', {}).get('osc8_sha256')]
    for profile in PROFILES:
        identities.extend(record.get('binaries', {}).get(profile, {}).get(field)
                          for field in ('sha256', 'companion_sha256'))
    identities.extend(record.get('installed', {}).get(field) for field in ('sha256', 'companion_sha256'))
    if any(not isinstance(value, str) or len(value) != 64 or any(c not in '0123456789abcdef' for c in value)
           for value in identities):
        raise ValueError('missing binary/reference/helper identity')
    steps = record.get('steps', [])
    by_name = {step['name']: step for step in steps}
    if len(by_name) != len(steps) or set(by_name) != REQUIRED_STEPS:
        raise ValueError('missing, duplicate or unexpected release verification steps')
    directory = Path(path).resolve().parent
    def evidence(name):
        expected = record.get('evidence_files', {}).get(name)
        selected = (directory / name).resolve()
        if not expected or not selected.is_relative_to(directory) or not selected.is_file() or sha_file(selected) != expected:
            raise ValueError('missing or stale verification evidence: ' + name)
        return selected
    for name, step in by_name.items():
        expected = 1 if name == 'negative-candidate' else 0
        if not integer(step.get('returncode'), expected) or not integer(step.get('expected_returncode'), expected) or step.get('passed') is not True:
            raise ValueError('failed verification step: ' + name)
        evidence(name + '.log')
    coverage = json.loads(evidence('coverage.json').read_text())
    if (not integer(coverage.get('schema_version'), 1) or not integer(coverage.get('acceptance_cases'), 809) or
            not integer(coverage.get('reference_expectations'), 809) or
            type(coverage.get('required_interfaces')) is not int or coverage['required_interfaces'] <= 0 or
            not integer(coverage.get('required_interfaces_with_cases'), coverage['required_interfaces'])):
        raise ValueError('incomplete corpus coverage evidence')
    negative = json.loads(evidence('negative.json').read_text())
    if not all(integer(negative.get(key), count) for key, count in [('executed', 2), ('passed', 0), ('failed', 2)]) or negative.get('errors') != []:
        raise ValueError('negative candidate control did not reject exactly two cases')
    reference_hash = record.get('reference', {}).get('sha256')
    for lane in ('acceptance-debug', 'acceptance-release', 'reference', 'acceptance-installed'):
        acceptance = json.loads(evidence(lane + '/summary.json').read_text())
        validate_acceptance(acceptance)
        validate_acceptance_observer(acceptance, root)
        expected = reference_hash if lane == 'reference' else (
            record['binaries'][lane.removeprefix('acceptance-')]['sha256'] if lane != 'acceptance-installed' else
            record['installed']['sha256'])
        if acceptance.get('sha256') != expected:
            raise ValueError('acceptance binary identity mismatch: ' + lane)
    for profile in PROFILES:
        for lane in ('integrations', 'helpers'):
            report = json.loads(evidence(f'{lane}-{profile}.json').read_text())
            validate_supplemental(report, lane, record['helpers']['osc8_sha256'])
        validate_job_control(json.loads(evidence(f'job-control-{profile}.json').read_text()))
        for seed in SEEDS:
            stress = json.loads(evidence(f'stress-{profile}-{seed}.json').read_text())
            rounds = selection['stress_rounds']
            if not all(integer(stress.get(key), count) for key, count in [('seed', seed), ('rounds', rounds),
                    ('executed', 108 + 6 * rounds), ('passed', 108 + 6 * rounds), ('failed', 0)]) or stress.get('failures') != []:
                raise ValueError('incomplete seeded stress evidence')
            if stress.get('binary_sha256') != record['binaries'][profile]['sha256'] or stress.get('reference_sha256') != reference_hash:
                raise ValueError('stress binary/reference identity mismatch')
    return source
