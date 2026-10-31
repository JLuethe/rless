#!/usr/bin/env python3
"""Build and record the bounded real fault/recovery regression slice."""
import argparse
import json
import os
from pathlib import Path
import platform
import sys
import unittest

from common import Evidence, PINS, ROOT, environment, sha, write_json
from run import check_tools, nightly_environment
from native_alloc import RUSTFLAGS as ALLOC_RUSTFLAGS
from test_fault_paths import ActualFaultPaths, contracts, validate_oracle_report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile', choices=('native', 'asan'), required=True)
    parser.add_argument('--tool-home', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--repetitions', type=int, default=1)
    parser.add_argument('--reference', type=Path, default=os.environ.get('RLESS_REFERENCE'))
    parser.add_argument('--abi-profile', choices=('BUILDER-GLIBC-2.44', 'REFERENCE-GLIBC-2.41'))
    args = parser.parse_args()
    args.output = args.output.resolve()
    if args.output.exists() or args.repetitions < 1:
        parser.error('choose a fresh output directory and positive repetition count')
    evidence = Evidence(args.output)
    evidence.report['scope'] = 'Partial real cleanup/read/regex/close/allocation regressions; not complete site/variant/profile acceptance or independent review.'
    env = environment()
    abi_profile = args.abi_profile or {'2.44': 'BUILDER-GLIBC-2.44', '2.41': 'REFERENCE-GLIBC-2.41'}.get(platform.libc_ver()[1])
    if abi_profile is None:
        parser.error('unrecognized runtime ABI; declare and verify a supported --abi-profile')
    expected_glibc = '2.41' if abi_profile == 'REFERENCE-GLIBC-2.41' else '2.44'
    if platform.libc_ver() != ('glibc', expected_glibc):
        parser.error('declared ABI does not match the running glibc')
    if args.reference is None:
        parser.error('--reference is required for native error comparison')
    reference = args.reference.resolve(strict=True)
    evidence.report['abi_profile'] = abi_profile
    evidence.report['reference'] = dict(path=str(reference), sha256=sha(reference))
    profile, target = 'NATIVE', []
    if args.profile == 'asan':
        if not args.tool_home:
            parser.error('--tool-home is required for ASan')
        check_tools(args, evidence)
        env = nightly_environment(args, 'ASAN')
        profile, target = 'ASAN', ['--target', PINS['target']]
    bins = {}
    for name, features in [('normal', []), ('injected', ['--features', 'safety-injection'])]:
        build_env = {**env, 'CARGO_TARGET_DIR': str(args.output / name)}
        record = evidence.run('build-' + name, ['cargo', 'build', '--locked', '--offline', '--bins', *target, *features],
            env=build_env, profile=profile, timeout=600)
        if record['status'] != 'passed':
            evidence.finish()
            return 1
        bins[name] = args.output / name / (PINS['target'] if target else '') / 'debug/rless'
    alloc_object = args.output / 'alloc-fault.o'
    alloc_object_record = args.output / 'alloc-object.json'
    alloc_binding = args.output / 'alloc-binding.json'
    alloc_target = args.output / 'alloc-counterpart'
    alloc_binary = alloc_target / (PINS['target'] if target else '') / 'debug/rless'
    alloc_env = {**env, 'CARGO_TARGET_DIR': str(alloc_target),
                 'RUSTFLAGS': (env.get('RUSTFLAGS', '') + ' ' + ALLOC_RUSTFLAGS).strip()}
    for name, command, selected_env in [
        ('build-allocation-object', [sys.executable, ROOT / 'tools/safety/native_alloc.py',
            '--build-object', alloc_object, '--output', alloc_object_record], environment()),
        ('build-allocation-counterpart', ['cargo', 'rustc', '--locked', '--offline', '--bin', 'rless',
            *target, '--', '-Clink-arg=' + str(alloc_object), '-Clink-arg=-Wl,--wrap=calloc', '-Clink-arg=-Wl,--wrap=strdup'], alloc_env),
        ('bind-allocation-counterpart', [sys.executable, ROOT / 'tools/safety/native_alloc.py',
            '--binary', alloc_binary, '--profile', args.profile, '--output', alloc_binding], environment()),
    ]:
        record = evidence.run(name, command, env=selected_env, profile=profile, timeout=600)
        if record['status'] != 'passed':
            evidence.finish()
            return 1
    alloc_artifacts = [alloc_object, alloc_object_record, alloc_binary, alloc_binding]
    record['artifact_hashes'].update({str(path):sha(path) for path in alloc_artifacts})
    evidence.report['native_allocation_binding'] = dict(path=str(alloc_binding), sha256=sha(alloc_binding))
    evidence.save()
    shim = args.output / 'cleanup-eintr.so'
    if evidence.run('build-native-counterpart', ['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
        ROOT / 'tools/safety/native/cleanup_eintr.c', '-ldl', '-o', shim], env=environment())['status'] != 'passed':
        evidence.finish()
        return 1
    diagnostics = args.output / 'sanitizer'
    short_shim = args.output / 'short-write.so'
    if evidence.run('build-short-write-counterpart', ['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
        ROOT / 'tools/safety/native/short_write.c', '-ldl', '-o', short_shim], env=environment())['status'] != 'passed':
        evidence.finish()
        return 1
    io_shim = args.output / 'io-fault.so'
    if evidence.run('build-io-counterpart', ['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
        ROOT / 'tools/safety/native/io_fault.c', '-ldl', '-o', io_shim], env=environment())['status'] != 'passed':
        evidence.finish()
        return 1
    diagnostics.mkdir()
    extra = []
    if args.profile == 'asan':
        config = args.output / 'instrumentation.json'
        variables = {k: env[k] for k in ('ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH')}
        variables['ASAN_OPTIONS'] += ':log_path=' + str(diagnostics / 'asan')
        write_json(config, variables)
        extra = ['--instrumentation-env', config]
        probe_env = {**env, 'CARGO_TARGET_DIR': str(args.output / 'probes')}
        build = evidence.run('build-harness-detectors', ['cargo', 'build', '--locked', '--offline',
            '--manifest-path', ROOT / 'tools/safety/probes/Cargo.toml', '--target', PINS['target'],
            '--bin', 'safe', '--bin', 'asan-uaf', '--bin', 'lsan-leak'],
            env=probe_env, profile=profile, timeout=600)
        if build['status'] != 'passed':
            evidence.finish()
            return 1
        capture = args.output / 'harness-controls'
        probes = args.output / 'probes' / PINS['target'] / 'debug'
        controlled = evidence.run('harness-sanitizer-capture-control', [sys.executable,
            ROOT / 'tools/safety/harness_controls.py', '--probe-dir', probes,
            '--instrumentation-env', config, '--output', capture], env=environment(), profile=profile,
            artifacts=[probes / name for name in ('safe', 'asan-uaf', 'lsan-leak')],
            scenario_id='SAFECASE-VER-002', variant='expected-nonzero case with injected sanitizer finding',
            coverage_scope='partial', requirement_ids=['SAFE-VER-002', 'SAFE-VER-010'],
            oracle_metadata=dict(abi_profile=abi_profile, corpus_sha256=sha(ROOT / 'compat/corpus.json'),
                seed=None, test_selector='harness_controls.validate_capture', hook_sites=[], hook_ordinals=[],
                m0_controls=['harness-sanitizer-capture-control']))
        capture_path = capture / 'summary.json'
        if capture_path.is_file():
            from harness_controls import validate_capture
            try:
                capture_report = json.loads(capture_path.read_text())
                validate_capture(capture_report)
                controlled['artifact_hashes'][str(capture_path)] = sha(capture_path)
                for control in capture_report['records']:
                    for diagnostic in control['diagnostics']:
                        controlled['artifact_hashes'][diagnostic['path']] = diagnostic['sha256']
                controlled['detector_capture_report'] = dict(path=str(capture_path), sha256=sha(capture_path))
            except ValueError as error:
                controlled.update(status='failed', oracle_error=str(error))
        else:
            controlled.update(status='failed', oracle_error='missing harness detector proof')
        evidence.save()
    declarations = contracts()
    selected = unittest.defaultTestLoader.getTestCaseNames(ActualFaultPaths)
    if set(selected) != set(declarations):
        raise ValueError('every actual test needs exactly one catalogue binding')
    for name in selected:
        declaration = declarations[name]
        oracle = args.output / (name + '.json')
        metadata = dict(abi_profile=abi_profile, corpus_sha256=sha(ROOT / 'compat/corpus.json'),
            seed=None, test_selector='ActualFaultPaths.' + name,
            hook_sites=declaration['hook_sites'], hook_ordinals=declaration['hook_ordinals'],
            m0_controls=declaration['m0_controls'], repetitions=args.repetitions)
        before_reports = set(diagnostics.iterdir())
        record = evidence.run(name, [sys.executable, ROOT / 'tools/safety/test_fault_paths.py',
            '--normal', bins['normal'], '--injected', bins['injected'], '--native-shim', shim,
            '--short-write-shim', short_shim, '--native-io-shim', io_shim, '--reference', reference,
            '--native-alloc-binding', alloc_binding,
            '--repetitions', str(args.repetitions), '--oracle-report', oracle, *extra,
            'ActualFaultPaths.' + name], env=environment(), profile=profile,
            artifacts=[*bins.values(), shim, short_shim, io_shim, reference, *alloc_artifacts], timeout=max(180, args.repetitions * 5),
            scenario_id=declaration['scenario_id'], variant=declaration['variant'],
            requirement_ids=[declaration['scenario_id'].replace('SAFECASE-', 'SAFE-')],
            coverage_scope='partial', oracle_metadata=metadata, finding_ids=declaration['finding_ids'])
        if oracle.is_file():
            detail = json.loads(oracle.read_text())
            record['artifact_hashes'][str(oracle)] = sha(oracle)
            record['oracle_report'] = dict(path=str(oracle), sha256=sha(oracle))
            try:
                record['observed_hooks'] = validate_oracle_report(detail, [metadata['test_selector']])[metadata['test_selector']]
            except ValueError as error:
                record.update(status='failed', oracle_error=str(error))
        else:
            record.update(status='failed', oracle_error='missing actual per-test oracle')
        new_reports = set(diagnostics.iterdir()) - before_reports
        if new_reports:
            record.update(status='failed', sanitizer_reports=[dict(path=str(p), sha256=sha(p)) for p in sorted(new_reports)])
        evidence.save()
    evidence.report['findings'].extend([
        dict(id='M0-IO-001', status='open', reason='Injected read EIO loses input without diagnostic and returns zero.'),
        dict(id='M0-IO-002', status='open', reason='Injected write EIO loses output without diagnostic and returns zero.')])
    reports = [dict(path=str(p), sha256=sha(p)) for p in sorted(diagnostics.iterdir())]
    evidence.report['sanitizer_reports'] = reports
    if reports:
        evidence.report['findings'].append(dict(status='open', reports=reports))
    # This bounded slice does not derive M0 closure. The gate independently
    # requires every site/profile/ABI control and source-bound review. An
    # unconditional synthetic "unsupported" command is not executed evidence.
    evidence.report['remaining_scope'] = {
        'status': 'not-assessed-by-this-runner',
        'gate': 'tools/safety/m0_gate.py',
        'requires': ['complete ownership/native review', 'all required site/profile/ABI controls',
                     'source-reviewed applicability and executed alternatives',
                     'independent source-bound M0 approval'],
    }
    return 0 if evidence.finish() else 1


if __name__ == '__main__':
    raise SystemExit(main())
