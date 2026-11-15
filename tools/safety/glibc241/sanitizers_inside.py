"""Pinned ASan/LSan build and diagnostic controls inside the actual reference ABI."""
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import time

WORK = Path('/work')
SOURCE = WORK / 'source'
REPORT = WORK / 'report'
DIAGNOSTICS = re.compile(r'(?:ERROR: (?:AddressSanitizer|LeakSanitizer)|AddressSanitizer:DEADLYSIGNAL|runtime error:)', re.I)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def main():
    os.umask(0o077)
    SOURCE.mkdir()
    REPORT.mkdir()
    (WORK / 'home').mkdir()
    (WORK / 'cargo').mkdir()
    (WORK / 'cargo/registry').symlink_to('/cache/registry')
    with tarfile.open('/input/source.tar') as archive:
        archive.extractall(SOURCE, filter='data')
    sys.path.insert(0, str(SOURCE / 'tools/safety/glibc241'))
    from snapshot import validate
    snapshot = json.loads(Path('/input/source-manifest.json').read_text())
    validate(snapshot, SOURCE, exact=True)
    sys.path.insert(0, str(SOURCE / 'tools'))
    from release import capture_reference_bundle
    capture_reference_bundle('/reference/less', SOURCE)
    pins = json.loads((SOURCE / 'tools/safety/profiles.json').read_text())
    options = json.loads(Path('/input/options.json').read_text())
    records = []
    result = dict(schema_version=1, status='not-verified', runtime_safety_verified=False,
                  source_manifest_sha256=sha('/input/source-manifest.json'), records=records,
                  procedure='REFERENCE-GLIBC-2.41-ASAN-OFFLINE-v1',
                  scope='Reference-ABI detector and fault infrastructure; no whole-catalogue or M0 closure claim.')

    def persist():
        save(REPORT / 'result.json', result)

    def run(name, command, *, env=None, expected=0, diagnostic=None, timeout=600):
        before = time.monotonic()
        log = REPORT / (name + '.log')
        with log.open('wb') as stream:
            completed = subprocess.run([str(x) for x in command], cwd=SOURCE, env=env,
                                       stdout=stream, stderr=subprocess.STDOUT, timeout=timeout)
        output = log.read_text(errors='replace')
        passed = completed.returncode == expected
        if diagnostic:
            passed = completed.returncode != 0 and re.search(diagnostic, output, re.I | re.S) is not None
        elif DIAGNOSTICS.search(output):
            passed = False
        record = dict(name=name, command=[str(x) for x in command], status='passed' if passed else 'failed',
                      original_exit_status=completed.returncode, expected_exit_status=expected,
                      expected_diagnostic=diagnostic, log=str(log), log_sha256=sha(log),
                      wall_seconds=time.monotonic() - before,
                      build_flags={key:env[key] for key in ('RUSTFLAGS','ASAN_OPTIONS','LSAN_OPTIONS') if env and key in env})
        records.append(record)
        persist()
        if not passed:
            raise RuntimeError('required record failed: ' + name)
        return output

    persist()
    try:
        runtime = subprocess.check_output(['getconf', 'GNU_LIBC_VERSION'], text=True).strip()
        if runtime != 'glibc 2.41':
            raise ValueError('incorrect target runtime: ' + runtime)
        result.update(runtime=runtime, identity=dict(uid=os.getuid(), euid=os.geteuid(), gid=os.getgid()))
        dependency_versions = {}
        for line in (SOURCE / 'tools/compat/requirements.txt').read_text().splitlines():
            if not line.strip() or line.lstrip().startswith('#'):
                continue
            package, version = line.strip().split('==')
            actual = importlib.metadata.version(package)
            if actual != version:
                raise ValueError('observer dependency differs: ' + package)
            dependency_versions[package] = actual
        result['python_dependencies'] = dependency_versions
        compiler = run('tool-rustc', ['rustc', '-vV'])
        if pins['rustc_commit'] not in compiler or 'host: ' + pins['target'] not in compiler:
            raise ValueError('nightly compiler does not match the pinned identity')
        run('tool-cargo', ['cargo', '-V'])
        run('abi-build', ['cc', '-Wall', '-Wextra', '-Werror', SOURCE / 'tools/safety/abi_probe.c',
                          '-o', REPORT / 'abi-probe'])
        probe = json.loads(run('abi-probe', [REPORT / 'abi-probe']))
        if probe['glibc_headers'] != '2.41':
            raise ValueError('native headers do not match runtime')
        result['header_probe'] = probe
        result['native_versions'] = {name: run('tool-' + name, command) for name, command in
            [('cc', ['cc', '--version']), ('linker', ['ld', '--version'])]}
        result['environment_manifests'] = {}
        for name in ('packages.tsv', 'packages.sha256', 'apt-metadata.sha256', 'runtime.txt',
                     'python-wheels.sha256', 'python-packages.txt'):
            shutil.copyfile(Path('/opt/rless-abi') / name, REPORT / name)
            result['environment_manifests'][name] = sha(REPORT / name)
        (REPORT / 'terminfo.txt').write_bytes(subprocess.check_output(['infocmp', '-x', 'xterm-256color']))
        result['terminfo_sha256'] = sha(REPORT / 'terminfo.txt')
        result['instrumentation_scope'] = pins['instrumentation_scope']
        result['build_environment'] = {key: os.environ[key] for key in
            ('PATH', 'CARGO_HOME', 'CARGO_NET_OFFLINE', 'HOME')}
        asan = dict(os.environ, RUSTFLAGS=pins['asan_rustflags'],
                    ASAN_OPTIONS=pins['asan_options'] + ':symbolize=0', LSAN_OPTIONS=pins['lsan_options'])
        asan.pop('ASAN_SYMBOLIZER_PATH', None)
        bins = {}
        for name, features in [('normal', []), ('injected', ['--features', 'safety-injection'])]:
            target = WORK / name
            env = dict(asan, CARGO_TARGET_DIR=str(target))
            run('build-' + name, ['cargo', 'build', '--locked', '--offline', '--bins',
                                  '--target', pins['target'], *features], env=env)
            bins[name] = target / pins['target'] / 'debug/rless'
            nm = run('instrumentation-' + name, ['nm', bins[name]])
            if '__asan_' not in nm:
                raise ValueError('candidate lacks ASan instrumentation: ' + name)
        target = WORK / 'detectors'
        run('build-detectors', ['cargo', 'build', '--locked', '--offline', '--manifest-path',
            SOURCE / 'tools/safety/probes/Cargo.toml', '--target', pins['target'], '--bins'],
            env=dict(asan, CARGO_TARGET_DIR=str(target)))
        probes = target / pins['target'] / 'debug'
        # Retain target objects before any control can fail. Postprocessing must
        # remain possible for failed attempts as well as successful ones.
        objects = {}
        native = WORK / 'native-objects'
        native.mkdir()
        for binary in [*bins.values(), *(probes / x for x in ('safe', 'asan-uaf', 'lsan-leak'))]:
            objects[str(binary)] = dict(path=str(binary), sha256=sha(binary), kind='application-or-control')
            loaded = subprocess.check_output(['ldd', str(binary)], text=True)
            for name in re.findall(r'(/[^\s()]+)', loaded):
                path = Path(name)
                if not path.is_file() or name in objects:
                    continue
                destination = native / sha(path)
                shutil.copyfile(path, destination)
                objects[name] = dict(path=str(destination), sha256=sha(destination), kind='target-native-library')
        result['objects'] = objects
        persist()
        sys.path.insert(0, str(SOURCE / 'tools/safety'))
        import test_fault_paths as harness
        controls = []
        result['detector_controls'] = controls
        for transport in ('pipe', 'pty'):
            for binary, expected, pattern, source in (
                ('safe', 0, None, None),
                ('asan-uaf', 87, r'ERROR: AddressSanitizer: heap-use-after-free', 'asan_uaf.rs'),
                ('lsan-leak', 87, r'ERROR: LeakSanitizer: detected memory leaks.*8192 byte', 'lsan_leak.rs'),
            ):
                directory = REPORT / ('control-' + transport + '-' + binary)
                directory.mkdir()
                variables = dict(ASAN_OPTIONS=asan['ASAN_OPTIONS'] + ':log_path=' + str(directory / 'raw'),
                                 LSAN_OPTIONS=asan['LSAN_OPTIONS'])
                harness.INSTRUMENTATION = variables
                if transport == 'pipe':
                    observed, trace = harness.pipe_run(probes / binary, payload=b'')
                    status = observed.returncode
                    observation = dict(status=status, stdout_hex=observed.stdout.hex(),
                                       stderr_hex=observed.stderr.hex(), trace_hex=trace.hex())
                else:
                    observation, trace = harness.pty_run(probes / binary,
                        dict(id='SANITIZER-' + binary, mode='pty', argv=[], events=[], files={}, timeout=10))
                    status = observation['status']
                    observation['trace_hex'] = trace.hex()
                logs = sorted(directory.glob('raw.*'))
                text = '\n'.join(path.read_text(errors='replace') for path in logs)
                passed = status == expected and not trace
                if transport == 'pty':
                    passed = passed and observation.get('termios_restored') is True
                if pattern:
                    passed = passed and bool(logs) and re.search(pattern, text, re.I | re.S) is not None
                else:
                    passed = passed and not logs
                save(directory / 'observation.json', observation)
                controls.append(dict(name=transport + '-' + binary, status='passed' if passed else 'failed',
                    original_exit_status=status, expected_exit_status=expected,
                    expected_diagnostic=pattern, required_source_suffix=source,
                    object=str(probes / binary), object_sha256=sha(probes / binary),
                    raw_reports=[dict(path=str(path), sha256=sha(path)) for path in logs],
                    observation=str(directory / 'observation.json'),
                    observation_sha256=sha(directory / 'observation.json')))
                persist()
                if not passed:
                    raise RuntimeError('detector control failed: ' + transport + '-' + binary)
        shims = {}
        for name, filename in [('cleanup', 'cleanup_eintr.c'), ('short-write', 'short_write.c'),
                               ('native-io', 'io_fault.c')]:
            shims[name] = WORK / (name + '.so')
            run('build-' + name + '-counterpart', ['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
                SOURCE / 'tools/safety/native' / filename, '-ldl', '-o', shims[name]])
        for path in shims.values():
            objects[str(path)]=dict(path=str(path),sha256=sha(path),kind='native-counterpart-shim')
        from native_alloc import RUSTFLAGS as ALLOC_RUSTFLAGS
        alloc_object = WORK / 'alloc-fault.o'
        alloc_binding = REPORT / 'alloc-binding.json'
        alloc_target = WORK / 'alloc-counterpart'
        alloc_binary = alloc_target / pins['target'] / 'debug/rless'
        run('build-allocation-object', ['python3', SOURCE / 'tools/safety/native_alloc.py',
            '--build-object', alloc_object, '--output', REPORT / 'alloc-object.json'])
        run('build-allocation-counterpart', ['cargo', 'rustc', '--locked', '--offline', '--bin', 'rless',
            '--target', pins['target'], '--', '-Clink-arg=' + str(alloc_object), '-Clink-arg=-Wl,--wrap=calloc', '-Clink-arg=-Wl,--wrap=strdup'],
            env=dict(asan, CARGO_TARGET_DIR=str(alloc_target), RUSTFLAGS=asan['RUSTFLAGS']+' '+ALLOC_RUSTFLAGS))
        run('bind-allocation-counterpart', ['python3', SOURCE / 'tools/safety/native_alloc.py',
            '--binary', alloc_binary, '--profile', 'asan', '--output', alloc_binding])
        for path in (alloc_object, alloc_binary, alloc_binding, REPORT / 'alloc-object.json'):
            objects[str(path)] = dict(path=str(path),sha256=sha(path),kind='scoped-allocation-counterpart')
        result['allocation_binding'] = dict(path=str(alloc_binding),sha256=sha(alloc_binding))
        unexpected = REPORT / 'unexpected-sanitizer'
        unexpected.mkdir()
        variables = dict(ASAN_OPTIONS=asan['ASAN_OPTIONS'] + ':log_path=' + str(unexpected / 'raw'),
                         LSAN_OPTIONS=asan['LSAN_OPTIONS'])
        config = REPORT / 'instrumentation.json'
        save(config, variables)
        command = ['python3', SOURCE / 'tools/safety/test_fault_paths.py', '--normal', bins['normal'],
                   '--injected', bins['injected'], '--native-shim', shims['cleanup'],
                   '--short-write-shim', shims['short-write'], '--repetitions', str(options['repetitions']),
                   '--instrumentation-env', config, '--native-io-shim', shims['native-io'],
                   '--reference', '/reference/less', '--native-alloc-binding', alloc_binding,
                   '--oracle-report', REPORT / 'fault-oracles.json']
        run('fault-path-tests', command, timeout=max(300, options['repetitions'] * 8))
        reports = sorted(unexpected.glob('*'))
        result['unexpected_sanitizer_reports'] = [dict(path=str(path), sha256=sha(path)) for path in reports]
        if reports:
            raise RuntimeError('unexpected sanitizer reports retained; fault lane is failed')
        result['oracle_report_sha256'] = sha(REPORT / 'fault-oracles.json')
        validate(snapshot, SOURCE)
        result.update(status='passed', infrastructure_scope='Declared fault regression and detector controls only',
                      remaining_final_acceptance='Not inferred from this lane; inspect M0 bindings and final matrix.')
    except Exception as error:
        result['error'] = type(error).__name__ + ': ' + str(error)
        raise
    finally:
        persist()


if __name__ == '__main__':
    main()
