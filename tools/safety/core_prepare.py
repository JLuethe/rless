#!/usr/bin/env python3
"""Execute isolated M2 preparation evidence; never promote production gates."""
import argparse
from collections import Counter
import json
import os
from pathlib import Path
import re
import subprocess
import sys

from common import Evidence, PINS, ROOT, environment, sha
from run import detectors, nightly_environment

ACTIVE_EVIDENCE = None


def main():
    global ACTIVE_EVIDENCE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--tool-home', type=Path, required=True)
    parser.add_argument('--fuzz-runs', type=int, default=100000)
    args = parser.parse_args()
    args.output = args.output.resolve()
    args.tool_home = args.tool_home.resolve()
    if args.output.exists() or args.fuzz_runs < 10000:
        parser.error('choose a fresh directory and at least 10000 preparation fuzz runs')
    evidence = Evidence(args.output)
    ACTIVE_EVIDENCE = evidence
    evidence.report.update(scope='Isolated safe-core preparation; no production callers, M2–M5 closure, independent approval or final campaign budget credit.',
        production_integration=False, production_fuzz_budget_credit=0,
        original_entry_gates=['C0-GATE', 'C1-PROTO', 'M1.4'])
    manifest = ROOT / 'tools/safety/core/Cargo.toml'
    native = environment()
    native['CARGO_TARGET_DIR'] = str(args.output / 'native')
    nightly = environment(True, args.tool_home)
    nightly['CARGO_TARGET_DIR'] = str(args.output / 'nightly')
    nightly['CARGO_NET_OFFLINE'] = 'true'

    def run(name, command, **kwargs):
        result = evidence.run(name, command, **kwargs)
        if result['status'] != 'passed':
            evidence.finish()
            raise RuntimeError('required preparation check failed: ' + name)
        return result

    run('native-rustc', ['rustc', '-vV'], env=native)
    if ('release: ' + PINS['normal_rust_release']) not in (args.output / 'native-rustc.log').read_text():
        raise ValueError('normal compiler does not match pin')
    run('plan-structure', [sys.executable, ROOT / 'tools/safety/validate_m2_m5.py'], env=native)
    run('plan-mutation-controls', [sys.executable, ROOT / 'tools/safety/test_m2_m5_plan.py'], env=native)
    for name, flags in [('debug', []), ('release', ['--release'])]:
        run('core-' + name, ['cargo', 'test', '--locked', '--offline', '--manifest-path', manifest,
            '--features', 'fault-testing', *flags], env=native)
    run('core-default-build', ['cargo', 'build', '--locked', '--offline', '--manifest-path', manifest], env=native)

    compiler = args.output / 'compiler'
    compiler.mkdir()
    library = compiler / 'librless_safe_core.rlib'
    run('compiler-core-build', ['rustc', '--edition=2021', '--crate-name', 'rless_safe_core', '--crate-type', 'lib',
        ROOT / 'src/safe_core/mod.rs', '-o', library], env=native)
    cases = ROOT / 'tools/safety/core/compile'
    common = ['rustc', '--edition=2021', '--extern', 'rless_safe_core=' + str(library)]
    run('core-compiler-safe', [*common, cases / 'safe.rs', '-o', compiler / 'safe'], env=native, artifacts=[library])
    run('core-compiler-safe-run', [compiler / 'safe'], env=native, artifacts=[compiler / 'safe'])
    for name, diagnostic in [('borrow', r'error\[E0502\].*cannot borrow'),
        ('owner', r'error\[E0521\].*borrowed data escapes'),
        ('escape', r'error: lifetime may not live long enough'),
        ('domain', r'error\[E0308\].*mismatched types'),
        ('send', r'error\[E0277\].*cannot be sent between threads safely')]:
        run('compiler-' + name, [*common, cases / (name + '.rs'), '-o', compiler / name],
            env=native, diagnostic=diagnostic, artifacts=[library], profile='STATIC')
    # Inject into a copy of the actual crate root. Removing its forbid attribute
    # makes this negative control compile and therefore fail the evidence lane.
    source = (ROOT / 'src/safe_core/mod.rs').read_text()
    source = re.sub(r'(?m)^((?:pub )?mod (\w+));$',
        lambda m: '#[path = ' + json.dumps(str(ROOT / 'src/safe_core' / (m[2] + '.rs'))) + ']\n' + m[0], source)
    injected = compiler / 'injected-core.rs'
    injected.write_text(source + '\nfn probe() { unsafe { std::ptr::read(&0u8); } }\n')
    run('compiler-actual-core-forbids-unsafe', ['rustc', '--edition=2021', '--crate-type', 'lib',
        injected, '-o', compiler / 'forbidden.rlib'], env=native,
        diagnostic=r'error: usage of an `unsafe` block', profile='STATIC', artifacts=[injected])

    # Tool/detector controls precede instrumented component work. Their positive
    # counterparts and expected diagnostic failures retain individual records.
    detectors(args, evidence)
    if any(r['status'] != 'passed' for r in evidence.report['records']):
        evidence.finish()
        return 1
    asan = nightly_environment(args, 'ASAN')
    asan['CARGO_TARGET_DIR'] = str(args.output / 'asan-core')
    asan['CARGO_NET_OFFLINE'] = 'true'
    run('core-asan', ['cargo', 'test', '--locked', '--offline', '--manifest-path', manifest,
        '--target', PINS['target'], '--features', 'fault-testing', '--lib', '--tests'], env=asan, profile='ASAN')
    for seed in PINS['miri_seeds']:
        env = {**nightly, 'MIRIFLAGS': '-Zmiri-seed=' + str(seed)}
        record = run('core-miri-' + str(seed), ['cargo', 'miri', 'test', '--locked', '--offline',
            '--manifest-path', manifest, '--features', 'fault-testing', '--lib'], env=env, profile='MIRI', timeout=600)
        if '13 passed; 0 failed' not in Path(record['log']).read_text():
            raise ValueError('Miri selection did not run all 13 current unit tests')
    run('core-miri-action-model', ['cargo', 'miri', 'test', '--locked', '--offline', '--manifest-path', manifest,
        '--features', 'fault-testing', '--test', 'model'], env={**nightly, 'MIRIFLAGS': '-Zmiri-seed=712'},
        profile='MIRI', timeout=900)

    helper_env = {**nightly, 'CARGO_TARGET_DIR': str(args.output / 'inventory-target')}
    run('build-policy-inventory', ['cargo', 'build', '--locked', '--offline', '--manifest-path',
        ROOT / 'tools/safety/inventory/Cargo.toml'], env=helper_env, profile='STATIC')
    expanded = args.output / 'expanded-core.rs'
    command = ['cargo', 'rustc', '--locked', '--offline', '--manifest-path', str(manifest),
               '--features', 'fault-testing', '--lib', '--', '-Zunpretty=expanded']
    capture = 'import subprocess,sys; from pathlib import Path; p=subprocess.run(sys.argv[2:],capture_output=True); sys.stderr.buffer.write(p.stderr); Path(sys.argv[1]).write_bytes(p.stdout); raise SystemExit(p.returncode)'
    run('expand-core', [sys.executable, '-c', capture, expanded, *command], env=nightly, profile='STATIC')
    helper = args.output / 'inventory-target/debug/rless-safety-inventory'
    record = run('expanded-core-policy', [helper, expanded], env=nightly, artifacts=[helper, expanded], profile='STATIC')
    counts = Counter(line.split('\t')[0] for line in Path(record['log']).read_text().splitlines())
    prohibited = ('unsafe_block', 'unsafe_function', 'unsafe_impl', 'unsafe_trait', 'raw_pointer_type',
                  'extern_function_declaration', 'extern_static_declaration', 'mutable_static_definition', 'abi_function_definition')
    if any(counts[k] for k in prohibited) or counts['function_definition'] < 20:
        record['status'] = 'failed'
        record['oracle_error'] = 'unsafe/native/raw/mutable-global core or empty selection'
        evidence.finish()
        return 1
    evidence.report['expanded_core_policy'] = dict(counts=counts, prohibited=list(prohibited),
        scope='Actual isolated component expanded with fault-testing; not legacy pager core.')

    fuzz_env = {**nightly_environment(args, 'ASAN'), 'CARGO_NET_OFFLINE': 'true'}
    fuzz_env['PATH'] = str(args.tool_home / 'fuzz/bin') + os.pathsep + fuzz_env['PATH']
    run('fuzz-version', ['cargo', 'fuzz', '--version'], env=fuzz_env, profile='FUZZ')
    if PINS['cargo_fuzz_version'] not in (args.output / 'fuzz-version.log').read_text():
        raise ValueError('fuzz engine version mismatch')
    engine = args.tool_home / 'fuzz/bin/cargo-fuzz'
    evidence.report['coverage_engine'] = dict(path=str(engine), sha256=sha(engine), pins=PINS)
    corpus = args.output / 'corpus'
    artifacts = args.output / 'artifacts'
    corpus.mkdir(); artifacts.mkdir()
    for name, data in [('empty', b''), ('bytes', bytes(range(256))), ('oracle-safe-counterpart', b'RLESS'),
                       ('growth-reuse', bytes([0, 1, 2, 3, 17, 0, 3, 0, 20, 0, 0, 0]) * 32)]:
        (corpus / name).write_bytes(data)
    base = ['cargo', 'fuzz', 'run', '--fuzz-dir', ROOT / 'tools/safety/core-fuzz',
            '--target-dir', args.output / 'fuzz-target']
    record = run('core-coverage-fuzz', [*base, 'core_actions', corpus, '--', '-runs=' + str(args.fuzz_runs),
        '-seed=712', '-max_len=4096', '-timeout=5', '-print_final_stats=1',
        '-artifact_prefix=' + str(artifacts) + '/'], env=fuzz_env, profile='FUZZ', timeout=1200, artifacts=[engine])
    log = Path(record['log']).read_text()
    executed = re.findall(r'stat::number_of_executed_units:\s*(\d+)', log)
    coverage = re.findall(r'cov: (\d+)', log)
    if not executed or int(executed[-1]) < args.fuzz_runs or not coverage or max(map(int, coverage)) < 2:
        raise ValueError('missing actual coverage/execution counters')
    evidence.report['preparation_fuzz'] = dict(executions=int(executed[-1]), coverage_edges=max(map(int, coverage)),
        seed=712, scope='Actual isolated ByteBuf/arena/coordinate components; zero production campaign budget credit.',
        limits='4096 input bytes, 1024 ByteBuf actions, 256 arena actions; large/integer limits covered separately by unit tests.',
        final_corpus_sha256={p.name: sha(p) for p in corpus.iterdir() if p.is_file()})
    bad = args.output / 'oracle-seed'
    bad.write_bytes(b'RLESS')
    run('fuzz-oracle-safe', [*base, 'core_actions', bad, '--', '-runs=1'], env=fuzz_env, profile='FUZZ', artifacts=[bad])
    run('fuzz-oracle-negative', [*base, 'oracle_fault', bad, '--', '-runs=1',
        '-artifact_prefix=' + str(artifacts) + '/'], env=fuzz_env, profile='FUZZ', timeout=600,
        diagnostic=r'injected component oracle violation: detector control.*(?:libFuzzer|deadly signal)', artifacts=[bad])
    evidence.report['retained_fuzz_artifacts'] = [dict(path=str(p), sha256=sha(p), bytes=p.stat().st_size)
        for p in [bad, *artifacts.iterdir()] if p.is_file()]
    # Successful preparation is a scoped result. It cannot close the migration.
    evidence.report['migration_status'] = 'M0/M1 gates open; M2–M5 production migration not started.'
    return 0 if evidence.finish() else 1


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except Exception as error:
        if ACTIVE_EVIDENCE is not None:
            ACTIVE_EVIDENCE.unavailable('supervisor-failure', type(error).__name__ + ': ' + str(error), 'STATIC')
            ACTIVE_EVIDENCE.finish()
        raise
