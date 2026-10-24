#!/usr/bin/env python3
"""Build and record the bounded real fault/recovery regression slice."""
import argparse
from pathlib import Path
import sys

from common import Evidence, PINS, ROOT, environment, sha, write_json
from run import check_tools, nightly_environment


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile', choices=('native', 'asan'), required=True)
    parser.add_argument('--tool-home', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--repetitions', type=int, default=1)
    args = parser.parse_args()
    args.output = args.output.resolve()
    if args.output.exists() or args.repetitions < 1:
        parser.error('choose a fresh output directory and positive repetition count')
    evidence = Evidence(args.output)
    evidence.report['scope'] = 'Partial real cleanup/read/regex/close/allocation regressions; not complete site/variant/profile acceptance or independent review.'
    env = environment()
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
    diagnostics.mkdir()
    extra = []
    if args.profile == 'asan':
        config = args.output / 'instrumentation.json'
        variables = {k: env[k] for k in ('ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH')}
        variables['ASAN_OPTIONS'] += ':log_path=' + str(diagnostics / 'asan')
        write_json(config, variables)
        extra = ['--instrumentation-env', config]
    record = evidence.run('real-path-tests', [sys.executable, ROOT / 'tools/safety/test_fault_paths.py',
        '--normal', bins['normal'], '--injected', bins['injected'], '--native-shim', shim,
        '--short-write-shim', short_shim, '--repetitions', str(args.repetitions), *extra], env=environment(), profile=profile,
        artifacts=[*bins.values(), shim, short_shim], timeout=max(180, args.repetitions * 4))
    evidence.report['findings'].extend([
        dict(id='M0-IO-001', status='open', reason='Injected read EIO loses input without diagnostic and returns zero.'),
        dict(id='M0-IO-002', status='open', reason='Injected write EIO loses output without diagnostic and returns zero.')])
    reports = [dict(path=str(p), sha256=sha(p)) for p in sorted(diagnostics.iterdir())]
    evidence.report['sanitizer_reports'] = reports
    if reports:
        record['status'] = 'failed'
        evidence.report['findings'].append(dict(status='open', reports=reports))
    evidence.unavailable('permanent-io-error-acceptance',
        'Diagnostic reproduction is not a passing safety oracle; M0-IO-001/002 remain open.', profile)
    evidence.unavailable('remaining-site-acceptance',
        'All 16 site IDs have real hits, but callback error propagation, parser partial commits, native counterparts for every modeled failure, per-site ordinal separation and repeated in-process resource conservation remain incomplete.', profile)
    return 0 if evidence.finish() else 1


if __name__ == '__main__':
    raise SystemExit(main())
