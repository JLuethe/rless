#!/usr/bin/env python3
"""Reproducible M0 safety inventory, detector controls and initial diagnostics.

These lanes are infrastructure evidence, not implementation of all 175 safety
variants. Required unavailable work leaves the aggregate not-verified.
"""
import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parent))
from common import Evidence, PINS, ROOT, environment, git, sha, utc, validate_report, write_json


def nightly_environment(args, profile=None):
    env = environment(True, args.tool_home)
    env['CARGO_TARGET_DIR'] = str(args.output / 'target')
    if profile in ('ASAN', 'LSAN'):
        env['RUSTFLAGS'] = PINS['asan_rustflags']
        env['ASAN_OPTIONS'] = PINS['asan_options']
        env['LSAN_OPTIONS'] = PINS['lsan_options']
        # The host symbolizer is pinned separately from Rust's LLVM runtime.
        symbolizer = shutil.which(PINS['symbolizer']['name'])
        if symbolizer is None or sha(symbolizer) != PINS['symbolizer']['sha256']:
            raise ValueError('symbolizer binary does not match pin')
        env['ASAN_SYMBOLIZER_PATH'] = symbolizer
    return env


def check_tools(args, evidence):
    env = nightly_environment(args)
    for name, command in [('rustc', ['rustc', '-vV']), ('cargo', ['cargo', '-V']),
                          ('miri', ['cargo', 'miri', '--version'])]:
        record = evidence.run('tool-' + name, command, env=env, profile='STATIC', timeout=30)
        if record['status'] != 'passed':
            raise ValueError('required tool unavailable: ' + name)
    compiler = (args.output / 'tool-rustc.log').read_text()
    if PINS['rustc_commit'] not in compiler or ('host: ' + PINS['target']) not in compiler:
        raise ValueError('nightly compiler does not match exact pin/target')
    tc = Path(args.tool_home) / 'rustup/toolchains' / (PINS['nightly'] + '-' + PINS['target'])
    files = [tc / 'bin/rustc', tc / 'bin/cargo', tc / 'bin/miri',
             Path(shutil.which(PINS['symbolizer']['name']) or '/missing-symbolizer'),
             tc / 'lib/rustlib/src/rust/library/Cargo.lock']
    files.extend((tc / 'lib').glob('librustc_driver*.so'))
    files.extend((tc / 'lib/rustlib' / PINS['target'] / 'lib').glob('libstd-*.rlib'))
    for suffix in ('*.rs', '*.toml'):
        files.extend((tc / 'lib/rustlib/src/rust/library').rglob(suffix))
    evidence.report['tool_pins'] = PINS
    evidence.report['tool_files_sha256'] = {str(p): sha(p) for p in files}
    evidence.save()
    return env


def detectors(args, evidence):
    env = check_tools(args, evidence)
    instrumented = nightly_environment(args, 'ASAN')
    manifest = ROOT / 'tools/safety/probes/Cargo.toml'
    built = evidence.run('build-detectors', ['cargo', 'build', '--locked', '--manifest-path', manifest,
                         '--target', PINS['target'], '--bins'], env=instrumented, timeout=300, profile='ASAN')
    if built['status'] != 'passed':
        return
    binaries = args.output / 'target' / PINS['target'] / 'debug'
    reqs = ['SAFE-VER-002', 'SAFE-VER-010']
    evidence.run('asan-safe', [binaries / 'safe'], env=instrumented, artifacts=[binaries / 'safe'],
                 profile='ASAN', requirement_ids=reqs)
    evidence.run('asan-uaf', [binaries / 'asan-uaf'], env=instrumented,
                 diagnostic=r'ERROR: AddressSanitizer: heap-use-after-free',
                 artifacts=[binaries / 'asan-uaf'], profile='ASAN', requirement_ids=reqs)
    evidence.run('lsan-leak', [binaries / 'lsan-leak'], env=instrumented,
                 diagnostic=r'ERROR: LeakSanitizer: detected memory leaks.*8192 byte',
                 artifacts=[binaries / 'lsan-leak'], profile='LSAN', requirement_ids=reqs)
    if evidence.run('miri-setup', ['cargo', 'miri', 'setup'], env=env,
                    timeout=600, profile='MIRI')['status'] != 'passed':
        return
    for seed in PINS['miri_seeds']:
        seeded = {**env, 'MIRIFLAGS': f'-Zmiri-seed={seed}'}
        evidence.run('miri-safe-' + str(seed), ['cargo', 'miri', 'run', '--locked', '--manifest-path',
                     manifest, '--bin', 'safe'], env=seeded, timeout=180,
                     profile='MIRI', requirement_ids=['SAFE-VER-003', 'SAFE-VER-010'])
    evidence.run('miri-alias', ['cargo', 'miri', 'run', '--locked', '--manifest-path', manifest,
                 '--bin', 'miri-alias'], env=env, timeout=180,
                 diagnostic=r'error: Undefined Behavior:.*(?:tag|borrow|alloc)', profile='MIRI',
                 requirement_ids=['SAFE-VER-003', 'SAFE-VER-010'])
    (args.output / 'compiler-probes').mkdir(exist_ok=True)
    evidence.run('compiler-forbidden', ['rustc', ROOT / 'tools/safety/probes/forbidden.rs',
                 '--out-dir', args.output / 'compiler-probes'], env=env,
                 diagnostic=r'error: usage of an `unsafe` block', profile='STATIC',
                 requirement_ids=['SAFE-FFI-006', 'SAFE-VER-010'])
    evidence.run('compiler-safe', ['rustc', ROOT / 'tools/safety/probes/safe.rs',
                 '--out-dir', args.output / 'compiler-probes'], env=env, profile='STATIC',
                 requirement_ids=['SAFE-FFI-006', 'SAFE-VER-010'])


def inventory(args, evidence):
    env = check_tools(args, evidence)
    helper_manifest = ROOT / 'tools/safety/inventory/Cargo.toml'
    if evidence.run('build-inventory', ['cargo', 'build', '--locked', '--offline', '--manifest-path',
                     helper_manifest], env=env, profile='STATIC')['status'] != 'passed':
        return
    helper = args.output / 'target/debug/rless-safety-inventory'
    sources = sorted(str(p.relative_to(ROOT)) for top in ('src', 'vendor')
                     for p in (ROOT / top).rglob('*.rs')) + ['build.rs']
    if evidence.run('source-ast', [helper, *sources], env=env, profile='STATIC',
                    artifacts=[helper])['status'] != 'passed':
        return
    fixture = args.output / 'inventory-fixture.rs'
    fixture.write_text('static mut OBJECT: i32 = 0; extern "C" { static mut ALIAS: i32; fn native(); }\n'
                       'unsafe fn caller(p: *mut i32) { unsafe { *p = 1; native(); } }\n')
    checked = evidence.run('inventory-detector-control', [helper, fixture], env=env, profile='STATIC')
    text = (args.output / 'inventory-detector-control.log').read_text()
    counts = Counter(line.split('\t', 1)[0] for line in text.splitlines())
    if counts != Counter({'mutable_static_definition':1, 'extern_static_declaration':1,
                         'extern_function_declaration':1, 'function_definition':1,
                         'unsafe_function':1, 'raw_pointer_type':1, 'unsafe_block':1, 'direct_call':1}):
        checked['status'] = 'failed'
        checked['oracle_error'] = 'AST failed to separate a mutable definition from an extern declaration'
    for name, target in [('lib', ['--lib']), ('lessecho', ['--bin', 'lessecho'])]:
        record = evidence.run('expand-' + name, ['cargo', 'rustc', '--locked', '--offline', *target,
                              '--', '-Zunpretty=expanded'], env=env, profile='STATIC')
        if record['status'] != 'passed':
            continue
        # Cargo messages are on stderr; reroute only rustc stdout to a source file.
        source = args.output / ('expanded-' + name + '.rs')
        with source.open('wb') as out:
            subprocess.run(['cargo', 'rustc', '--locked', '--offline', *target, '--', '-Zunpretty=expanded'],
                           cwd=ROOT, env=env, stdout=out, stderr=subprocess.DEVNULL, check=True)
        evidence.run('expanded-ast-' + name, [helper, source], env=env, profile='STATIC', artifacts=[helper, source])
    records = []
    for line in (args.output / 'source-ast.log').read_text().splitlines():
        kind, file, row, col, context, symbol = line.split('\t', 5)
        records.append(dict(kind=kind, file=file, line=int(row), column=int(col), context=context, symbol=symbol))
    raw_path = args.output / 'syntax-inventory.json'
    write_json(raw_path, records)
    definitions_by_name = {}
    for item in records:
        if item['kind'] == 'function_definition':
            key = item['file'] + '::' + '::'.join(x for x in (item['context'], item['symbol']) if x)
            definitions_by_name.setdefault(item['symbol'], []).append(key)
    calls = {}
    for item in records:
        if item['kind'] not in ('direct_call', 'indirect_call'):
            continue
        caller = item['file'] + '::' + item['context']
        symbol = item['symbol'].replace(' ', '').split('::')[-1]
        calls.setdefault(caller, []).append(dict(symbol=item['symbol'], line=item['line'],
              resolution='indirect/unresolved' if item['kind']=='indirect_call' else 'syntactic candidate only',
              candidate_definitions=definitions_by_name.get(symbol, [])))
    write_json(args.output / 'callgraph.json', dict(scope='Syntactic direct-call candidates; indirect/alias/link-name resolution requires manual review.', calls=calls))
    definitions = {r['symbol'] for r in records if r['kind'] == 'abi_function_definition'}
    externs = {r['symbol'] for r in records if r['kind'] == 'extern_function_declaration'}
    # Canonical libc calls do not have a local extern declaration. Keep their
    # qualified identities until review resolves linkage; do not silently omit
    # them or conflate a declaration with an actual native object.
    qualified_native = [r for r in records if r['kind']=='direct_call' and
                        r['symbol'].replace(' ','').lstrip(':').startswith('libc::')]
    native_candidates = (externs - definitions) | {r['symbol'].replace(' ','').lstrip(':') for r in qualified_native}
    files = {}
    for file in sources:
        rows = [r for r in records if r['file'] == file]
        if not rows:
            continue
        files[file] = dict(sha256=sha(ROOT / file), counts=dict(Counter(r['kind'] for r in rows)),
                           classification='manual ownership/indirect-call review pending')
    expanded_counts = {}
    for name in ('lib', 'lessecho'):
        log = args.output / ('expanded-ast-' + name + '.log')
        if log.exists():
            expanded_counts[name] = dict(Counter(line.split('\t', 1)[0] for line in log.read_text().splitlines()))
    summary = dict(schema_version=1, recorded_at_utc=utc(), status='partial', evidence_status='not-verified',
                   scope='AST and compiler-expanded syntax inventory; not a complete ownership/alias or indirect-call proof.',
                   files=files, counts=dict(Counter(r['kind'] for r in records)),
                   expanded_counts=expanded_counts, candidate_external_symbols=sorted(native_candidates),
                   qualified_native_calls=qualified_native,
                   application_c_abi_symbols=sorted(definitions),
                   mutable_static_definitions=[r for r in records if r['kind'] == 'mutable_static_definition'],
                   raw_inventory=dict(path=str(raw_path), sha256=sha(raw_path)),
                   indirect_call_sites=[r for r in records if r['kind'] == 'indirect_call'],
                   source_fingerprint=evidence.report['input_fingerprints']['sha256'],
                   manual_register='docs/safety/ownership-register.json')
    write_json(args.output / 'inventory.json', summary)
    evidence.report['inventory'] = dict(path=str(args.output / 'inventory.json'), sha256=sha(args.output / 'inventory.json'))
    evidence.save()


def initial(args, evidence):
    env = check_tools(args, evidence)
    instrumented = nightly_environment(args, 'ASAN')
    if evidence.run('build-asan-candidate', ['cargo', 'build', '--locked', '--offline', '--bins',
                     '--target', PINS['target']], env=instrumented, timeout=600,
                     profile='ASAN')['status'] != 'passed':
        return
    bins = args.output / 'target' / PINS['target'] / 'debug'
    binary = bins / 'rless'
    evidence.report['instrumented_binaries'] = {name: dict(path=str(bins / name), sha256=sha(bins / name))
                                                for name in ('rless', 'lessecho')}
    evidence.run('asan-rust-tests', ['cargo', 'test', '--locked', '--offline', '--target', PINS['target']],
                 env=instrumented, timeout=300, profile='ASAN', requirement_ids=['SAFE-VER-002'])
    # Separate diagnostics from PTY output while preserving every sanitizer report.
    diagnostics = args.output / 'sanitizer'
    diagnostics.mkdir()
    variables = {k: instrumented[k] for k in ('ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH')}
    variables['ASAN_OPTIONS'] += ':log_path=' + str(diagnostics / 'asan')
    config = args.output / 'instrumentation-env.json'
    write_json(config, variables)
    observer_env = {**environment(), 'RLESS_SAFETY_ENV': str(config)}
    evidence.run('asan-acceptance', [sys.executable, ROOT / 'tools/compat/check_candidate.py',
                 '--binary', binary, '--results-dir', args.output / 'acceptance'],
                 env=observer_env, timeout=900, profile='ASAN', artifacts=[binary, config],
                 requirement_ids=['SAFE-VER-001', 'SAFE-VER-002'])
    reports = []
    for path in sorted(diagnostics.glob('*')):
        reports.append(dict(path=str(path), sha256=sha(path), diagnostic=path.read_text(errors='replace')[:20000]))
    if reports:
        evidence.report['findings'].append(dict(id='M0-INSTRUMENTED-001', status='open',
             classification='instrumented diagnostic; triage each report before assigning application/native/tool ownership',
             blocks_final_safety_gate=True, reports=reports))
        evidence.unavailable('sanitizer-findings-open', 'Sanitizer reports require triage and fixes; expected application exits cannot hide them.', 'ASAN')
    # Full-program Miri stops in the native ELF sigaction constructor before any
    # test runs. Compile the real runtime module in isolation, never a stub.
    evidence.report['boundary_exclusions'] = [dict(profile='MIRI',
        boundary='src/process.rs::remember_sigpipe ELF constructor calls native sigaction',
        reason='Pinned Miri reports unsupported foreign function sigaction on Linux.',
        status='unsupported', native_alternative=dict(record='asan-rust-tests',
          test_names=['a_closed_output_pipe_preserves_default_sigpipe',
                      'an_inherited_ignored_sigpipe_stays_ignored']),
        review_status='independent boundary review pending')]
    for seed in PINS['miri_seeds']:
        seeded = {**env, 'MIRIFLAGS': f'-Zmiri-seed={seed}'}
        record = evidence.run('miri-runtime-' + str(seed), ['cargo', 'miri', 'test', '--locked', '--offline',
                     '--manifest-path', ROOT / 'tools/safety/components/Cargo.toml', '--lib',
                     'runtime::tests::nested_key_read_keeps_the_file_read_interruptible', '--', '--exact'],
                     env=seeded, timeout=300, profile='MIRI', requirement_ids=['SAFE-VER-003'])
        output = (args.output / ('miri-runtime-' + str(seed) + '.log')).read_text()
        if 'running 1 test' not in output or '1 passed; 0 failed' not in output:
            record['status'] = 'failed'
            record['oracle_error'] = 'Empty or unexpected component test selection'
    evidence.unavailable('miri-unconverted-core', 'Owned buffer/parser/state components are not migrated yet; one atomic regression is not whole-core Miri coverage.', 'MIRI')
    current = subprocess.check_output(['getconf', 'GNU_LIBC_VERSION'], text=True).strip()
    evidence.report['observed_glibc'] = current
    if current != 'glibc 2.41':
        evidence.unavailable('reference-abi-2.41', 'This host is ' + current + '; no glibc 2.41 test environment was supplied.', 'ABI')
    evidence.report['acceptance_matrix'] = [dict(scenario_id=x['id'], variant=v, profile=p,
           status='not-run', reason='M0 diagnostics do not implement this acceptance variant.')
           for x in json.loads((ROOT / 'spec/safety/acceptance.json').read_text())['scenarios']
           for v in x['variants'] for p in x['required_profiles']]
    evidence.save()


def fuzz(args, evidence):
    env = check_tools(args, evidence)
    env['PATH'] = str(Path(args.tool_home) / 'fuzz/bin') + os.pathsep + env['PATH']
    env['ASAN_SYMBOLIZER_PATH'] = nightly_environment(args, 'ASAN')['ASAN_SYMBOLIZER_PATH']
    version = evidence.run('fuzz-version', ['cargo', 'fuzz', '--version'], env=env, profile='FUZZ')
    if version['status'] != 'passed' or PINS['cargo_fuzz_version'] not in (args.output / 'fuzz-version.log').read_text():
        raise ValueError('coverage engine does not match pin')
    engine = Path(args.tool_home) / 'fuzz/bin/cargo-fuzz'
    evidence.report['coverage_engine'] = dict(path=str(engine), sha256=sha(engine),
          cargo_fuzz_version=PINS['cargo_fuzz_version'], libfuzzer_sys_version=PINS['libfuzzer_sys_version'])
    corpus = args.output / 'corpus'
    corpus.mkdir()
    (corpus / 'bytes').write_bytes(bytes(range(256)))
    (corpus / 'empty').write_bytes(b'')
    artifacts = args.output / 'artifacts'
    artifacts.mkdir()
    # cargo-fuzz creates artifacts beside its manifest. Keep every generated file
    # in the ignored evidence directory, including minimization outputs.
    project = args.output / 'fuzz-project'
    shutil.copytree(ROOT / 'tools/safety/fuzz', project)
    project_inputs = [p for p in project.rglob('*') if p.is_file()]
    base = ['cargo', 'fuzz', 'run', '--fuzz-dir', project, '--target-dir', args.output / 'target']
    record = evidence.run('fuzz-model', [*base, 'rollback_model', corpus, '--', '-runs=10000',
                 '-seed=712', '-max_len=4096', '-timeout=5', '-print_final_stats=1',
                 '-artifact_prefix=' + str(artifacts) + '/'], env=env, timeout=600,
                 profile='FUZZ', requirement_ids=['SAFE-VER-004', 'SAFE-VER-010'], artifacts=[engine, *project_inputs])
    output = (args.output / 'fuzz-model.log').read_text()
    coverage = re.findall(r'cov: (\d+)', output)
    counts = re.findall(r'stat::number_of_executed_units:\s*(\d+)', output)
    if record['status'] == 'passed' and (not coverage or max(map(int, coverage)) < 2 or not counts or int(counts[-1]) < 10000):
        record['status'] = 'failed'
        record['coverage_error'] = 'Missing coverage or required execution counts'
    evidence.report['fuzz_model'] = dict(scope='Test-only rollback model, not any of the 15 production targets.',
          executions=int(counts[-1]) if counts else 0, coverage_edges=max(map(int, coverage)) if coverage else 0,
          seed=712, corpus={p.name: sha(p) for p in corpus.iterdir() if p.is_file()})
    bad_corpus = args.output / 'fault-corpus'
    bad_corpus.mkdir()
    (bad_corpus / 'injected').write_bytes(b'prefix-RLESS-suffix')
    evidence.run('fuzz-oracle-negative', [*base, 'rollback_fault', bad_corpus, '--', '-runs=1',
                 '-seed=712', '-artifact_prefix=' + str(artifacts) + '/'], env=env, timeout=600,
                 diagnostic=r'injected rollback violation: detector control.*(?:libFuzzer|deadly signal)',
                 profile='FUZZ', requirement_ids=['SAFE-VER-010'])
    crashes = sorted(artifacts.glob('crash-*'))
    if not crashes:
        evidence.unavailable('fuzz-minimization', 'No retained crash input produced by the negative control.', 'FUZZ')
    else:
        evidence.run('fuzz-minimization', ['cargo', 'fuzz', 'tmin', '--fuzz-dir', project, '--target-dir', args.output / 'target',
                     'rollback_fault', crashes[0], '--', '-runs=1000', '-seed=712'], env=env,
                     timeout=300, profile='FUZZ')
        minimized = sorted((project / 'artifacts').rglob('minimized-from-*'), key=lambda p: p.stat().st_size)
        if not minimized:
            evidence.unavailable('minimized-reproducer', 'Minimizer produced no retained input.', 'FUZZ')
        else:
            evidence.run('fuzz-minimized-replay', [*base, 'rollback_fault', minimized[0], '--', '-runs=1'],
                         env=env, timeout=300, diagnostic=r'injected rollback violation: detector control',
                         profile='FUZZ', artifacts=[minimized[0]])
            evidence.report['minimized_reproducer'] = dict(path=str(minimized[0]), sha256=sha(minimized[0]), bytes=minimized[0].stat().st_size)
        evidence.report['retained_fuzz_artifacts'] = [dict(path=str(p), sha256=sha(p), bytes=p.stat().st_size)
                  for directory in (artifacts, project / 'artifacts') for p in directory.rglob('*') if p.is_file()]
    evidence.save()


def hooks(args, evidence):
    if args.normal_binary is None or args.injected_binary is None:
        raise ValueError('hooks lane requires --normal-binary and --injected-binary')
    evidence.report['coverage_scope'] = 'Partial: real ecalloc allocation hook only; not every reserve/create/native/cleanup boundary.'
    evidence.report['remaining_fault_families'] = ['other logical allocation sites', 'I/O', 'commit', 'callback', 'handoff', 'cleanup']
    evidence.run('allocation-hook-native', [sys.executable, ROOT / 'tools/safety/test_hooks.py',
                 '--normal', args.normal_binary, '--injected', args.injected_binary],
                 profile='NATIVE', requirement_ids=['SAFE-MEM-007','SAFE-VER-005','SAFE-VER-010'],
                 scenario_id='SAFECASE-MEM-007', variant='fail each logical reserve/create boundary',
                 artifacts=[args.normal_binary,args.injected_binary])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('lane', choices=('detectors', 'inventory', 'initial', 'fuzz', 'hooks', 'verify'))
    parser.add_argument('--normal-binary', type=Path)
    parser.add_argument('--injected-binary', type=Path)
    parser.add_argument('--tool-home', type=Path, help='Isolated rustup/cargo installation root')
    parser.add_argument('--output', type=Path, help='New ignored report directory, or summary JSON for verify')
    args = parser.parse_args()
    os.umask(0o077)
    if args.lane == 'verify':
        if args.output is None:
            parser.error('--output SUMMARY.json is required')
        try:
            validate_report(args.output)
        except (ValueError, KeyError, OSError) as error:
            print(str(error), file=sys.stderr)
            return 1
        print('Infrastructure report verified; this is not final safety acceptance.')
        return 0
    if args.tool_home is None and args.lane != 'hooks':
        parser.error('--tool-home is required')
    if args.output is None:
        (ROOT / 'compat/.runs').mkdir(parents=True, exist_ok=True)
        args.output = Path(tempfile.mkdtemp(prefix='safety-' + args.lane + '-', dir=ROOT / 'compat/.runs'))
    else:
        args.output = args.output.resolve()
        args.output.mkdir(parents=True, exist_ok=False)
    evidence = Evidence(args.output)
    print('Evidence: ' + str(args.output), flush=True)
    try:
        globals()[args.lane](args, evidence)
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        evidence.unavailable('runner-error', str(error), 'STATIC')
    passed = evidence.finish()
    print('Result: ' + evidence.report['status'], flush=True)
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
