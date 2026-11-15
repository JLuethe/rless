#!/usr/bin/env python3
"""Focused real-pager leak regressions; partial evidence, not M0 closure."""
import argparse
import json
from pathlib import Path
import sys

from common import Evidence, PINS, ROOT, environment, sha, write_json
from run import check_tools, nightly_environment


def cases():
    result = []
    # These include the original minimized leak reproducers without replacing
    # their historical files or any frozen compatibility inputs/observations.
    for name, options in (
        ('regex-original', ['-px']),
        ('command-original', ['--cmd=g']),
        ('command-replaced', ['--cmd=G', '--cmd=1g'] * 32),
        ('regex-backreference', ['-p((a)(b))\\1']),
        ('regex-empty', ['-pa*']),
    ):
        result.append(dict(id='LEAK-' + name, mode='pty',
            argv=[*options, '-PsREADY', '-PmREADY', '-PMREADY', 'i'],
            files={'i': {'text': 'x\nzabab\nbbb\n'}},
            events=[{'await': ['READY'], 'snapshot': True}, {'send': 'q'}]))
    events = [{'await': ['READY'], 'snapshot': True}]
    for _ in range(32):
        for command in ('g', '/((a)(b))\\1\n', '/a*\n'):
            events.append(dict(send=command, await_=['READY'], snapshot=True))
    # "await" is a Python keyword, so spell it explicitly in generated cases.
    for event in events:
        if 'await_' in event:
            event['await'] = event.pop('await_')
    events.append({'send': 'q'})
    result.append(dict(id='LEAK-regex-repeated', mode='pty',
        argv=['-PsREADY', '-PmREADY', '-PMREADY', 'i'],
        files={'i': {'text': 'x\nzabab\nbbb\n'}}, events=events))
    result.append(dict(id='LEAK-lesskey-file-then-inline', mode='pty',
        argv=['-PsREADY', '-PmREADY', '-PMREADY', 'i'],
        files={'i': {'text': 'one\ntwo\n'}, '.lesskey': {'text': '#env\nLESS=-N\n'}},
        env={'LESSKEY_CONTENT': '#env;LESS=-S'},
        events=[{'await': ['READY'], 'snapshot': True}, {'send': 'q'}]))
    result.append(dict(id='LEAK-lesskey-repeated-variables', mode='pty',
        argv=[*['--lesskey-content=#env\nLESS=-N'] * 32, '-PsREADY', '-PmREADY', '-PMREADY', 'i'],
        files={'i': {'text': 'one\ntwo\n'}},
        events=[{'await': ['READY'], 'snapshot': True}, {'send': 'q'}]))
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile', choices=('native', 'asan'), default='native')
    parser.add_argument('--reference', type=Path, required=True)
    parser.add_argument('--tool-home', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--capture-reference', action='store_true', help=argparse.SUPPRESS)
    args = parser.parse_args()
    args.output = args.output.resolve()
    if args.output.exists():
        parser.error('choose a new evidence directory; earlier failures are retained')
    reference_manifest = ROOT / 'docs/verification/glibc241-baseline.json'
    baseline = json.loads(reference_manifest.read_text())
    if baseline['status'] != 'passed' or sha(args.reference) != baseline['references']['less']:
        parser.error('reference must match the documented rebuilt reference identity')
    if args.capture_reference:
        sys.path.insert(0, str(ROOT / 'tools/compat'))
        from run_v2 import run_case
        observed = {case['id']: run_case(args.reference.resolve(), case) for case in cases()}
        if any(x['status'] != 0 for x in observed.values()):
            raise ValueError('reference regression did not exit successfully')
        write_json(args.output, observed)
        return 0
    if args.profile == 'asan' and args.tool_home is None:
        parser.error('--tool-home is required for pinned ASan/LSan')
    evidence = Evidence(args.output)
    evidence.report['scope'] = 'Regex/saved-command regressions only; native library internals are not instrumented; no whole-core or M0 approval.'
    evidence.report['reference_identity'] = dict(manifest=str(reference_manifest),
        manifest_sha256=sha(reference_manifest), binary_sha256=sha(args.reference),
        scope='Documented rebuilt reference, distinct from the original frozen capture binary.')
    env = environment()
    env['CARGO_TARGET_DIR'] = str(args.output / 'target')
    target = []
    profile = 'NATIVE'
    if args.profile == 'asan':
        check_tools(args, evidence)
        env = nightly_environment(args, 'ASAN')
        target = ['--target', PINS['target']]
        profile = 'ASAN'
    build = evidence.run('build', ['cargo', 'build', '--locked', '--offline', '--bins', *target],
                         env=env, profile=profile, timeout=600)
    if build['status'] != 'passed':
        evidence.finish()
        return 1
    evidence.run('ownership-unit-tests', ['cargo', 'test', '--locked', '--offline', '--lib', *target,
                 'ownership_tests::', '--', '--test-threads=1'], env=env, profile=profile)
    binary = args.output / 'target' / (PINS['target'] if target else '') / 'debug/rless'
    corpus = args.output / 'cases.json'
    write_json(corpus, cases())
    observer = ROOT / 'tools/compat/run_v2.py'
    expected = args.output / 'reference.json'
    # New differential cases use the explicitly pinned rebuilt reference. The
    # frozen observer/corpus/expectations and its capture guard stay unchanged.
    capture = evidence.run('reference', [sys.executable, Path(__file__).resolve(), '--capture-reference',
        '--reference', args.reference.resolve(), '--output', expected],
        env=environment(), artifacts=[args.reference, corpus, observer, reference_manifest], timeout=180)
    if capture['status'] == 'passed':
        diagnostics = args.output / 'sanitizer'
        diagnostics.mkdir()
        observer_env = environment()
        if args.profile == 'asan':
            config = args.output / 'instrumentation.json'
            variables = {k: env[k] for k in ('ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH')}
            variables['ASAN_OPTIONS'] += ':log_path=' + str(diagnostics / 'asan')
            write_json(config, variables)
            observer_env['RLESS_SAFETY_ENV'] = str(config)
        record = evidence.run('pager-regressions', [sys.executable, observer, 'replay', '--binary', binary,
            '--cases', corpus, '--expected', expected, '--results', args.output / 'candidate-result.json'],
            env=observer_env, artifacts=[binary, corpus, observer, expected], profile=profile, timeout=180)
        reports = [dict(path=str(p), sha256=sha(p)) for p in sorted(diagnostics.iterdir())]
        evidence.report['sanitizer_reports'] = reports
        if reports:
            record['status'] = 'failed'
            evidence.report['findings'].append(dict(status='open', reports=reports,
                reason='Every sanitizer report requires triage, including expected application failures.'))
        data = json.loads((args.output / 'candidate-result.json').read_text())
        if data['executed'] != len(cases()) or data['passed'] != len(cases()) or data['errors']:
            record['status'] = 'failed'
    return 0 if evidence.finish() else 1


if __name__ == '__main__':
    raise SystemExit(main())
