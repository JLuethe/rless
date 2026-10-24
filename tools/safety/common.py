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
                or name in ('Cargo.toml', 'Cargo.lock', 'build.rs')):
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
            profile='NATIVE', artifacts=()):
        if not command:
            raise ValueError('empty command selection')
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
                      status='passed' if passed else 'failed', command=[str(x) for x in command],
                      original_exit_status=status, expected_exit_status=expected,
                      expected_diagnostic=diagnostic, diagnostic_found=has_finding,
                      timed_out=timed_out, stale_inputs=stale,
                      build_flags={k: env[k] for k in ('RUSTFLAGS', 'MIRIFLAGS', 'RUSTUP_TOOLCHAIN',
                                   'ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH') if env and k in env},
                      wall_seconds=time.monotonic() - started,
                      cpu_seconds=usage.ru_utime + usage.ru_stime - cpu.ru_utime - cpu.ru_stime,
                      log=str(log_path), log_sha256=sha(log_path), artifact_hashes=before)
        self.report['records'].append(record)
        self.save()
        print(f'{name}: {record["status"]} ({status})', flush=True)
        return record

    def unavailable(self, name, reason, profile, requirement_ids=()):
        self.report['records'].append(dict(name=name, status='unsupported', reason=reason,
                                           profile=profile, requirement_ids=list(requirement_ids)))
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


def validate_report(path, root=ROOT):
    """Re-derive a gate from logs, statuses, selections, artifacts and current inputs."""
    report = json.loads(Path(path).read_text())
    if report['input_fingerprints'] != fingerprint(root):
        raise ValueError('stale input fingerprint')
    records = report['records']
    if not records:
        raise ValueError('empty selection')
    names = set()
    for item in records:
        if item['name'] in names:
            raise ValueError('duplicate record')
        names.add(item['name'])
        if item['status'] != 'passed':
            raise ValueError('required record did not pass: ' + item['name'])
        log = Path(item['log'])
        if not log.is_file() or sha(log) != item['log_sha256']:
            raise ValueError('missing or altered diagnostic log')
        output = log.read_text(errors='replace')
        if item.get('timed_out') or item.get('stale_inputs'):
            raise ValueError('timed-out or stale record')
        if item['expected_diagnostic']:
            if item['original_exit_status'] == 0 or not re.search(item['expected_diagnostic'], output, re.I | re.S):
                raise ValueError('expected detector diagnostic missing')
        elif (item['original_exit_status'] != item['expected_exit_status'] or DIAGNOSTICS.search(output)):
            raise ValueError('unexpected exit or safety diagnostic')
        for artifact, value in item['artifact_hashes'].items():
            if not Path(artifact).is_file() or sha(artifact) != value:
                raise ValueError('stale binary artifact')
    return report
