#!/usr/bin/env python3
"""Run and rederive bounded M0 controls. Only an independent runner executes this.

This is infrastructure evidence, not production fuzz coverage or whole-core Miri.
"""
import argparse
import ast
import copy
import json
import os
from pathlib import Path
import re
import sys

from common import Evidence, PINS, ROOT, environment, fingerprint, git, sha, validate_report, write_json

PROCEDURE = 'M0-CONTROLS-v1'
LANES = ('static', 'hooks', 'detectors', 'fuzz')
UNIT_GROUPS = {
    'inventory-coverage': ['test_inventory_coverage'],
    'ownership-mutations': ['test_closure.ReviewControls'],
    'binding-mutations': ['test_closure.AcceptanceControls', 'test_safety', 'test_boundary_controls', 'test_native_callback',
                          'test_m0_gate', 'test_oracle_controls', 'test_m0_controls'],
}
COMPONENT_TEST = 'signal_events::tests'
COMPONENT_CASES = (
    'command_counter_overflow_remains_visible_after_dispatch',
    'publication_racing_with_dispatch_is_seen_in_one_of_two_snapshots',
    'state_bits_coalesce_but_commands_keep_their_count',
)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def ref(path):
    return dict(path=str(Path(path).resolve()), sha256=sha(path))


def checked(reference):
    require(isinstance(reference, dict) and set(reference) == {'path', 'sha256'}, 'invalid retained control reference')
    path = Path(reference['path'])
    require(path.is_file() and sha(path) == reference['sha256'], 'missing or altered control artifact')
    return path


def unittest_ids(selections, root=ROOT, script=False):
    result = []
    for selection in selections:
        module, _, chosen_class = selection.partition('.')
        tree = ast.parse((root / 'tools/safety' / (module + '.py')).read_text())
        for node in tree.body:
            if isinstance(node, ast.ClassDef) and (not chosen_class or node.name == chosen_class):
                for method in node.body:
                    if isinstance(method, (ast.FunctionDef, ast.AsyncFunctionDef)) and method.name.startswith('test_'):
                        result.append((method.name, ('__main__' if script else module) + '.' + node.name + '.' + method.name))
    require(result and len(set(result)) == len(result), 'empty or duplicate current unit-test selection')
    return result


def validate_unittest(text, expected):
    require(re.search(r'^Ran ' + str(len(expected)) + r' tests? in ', text, re.M) is not None
            and re.search(r'^OK\s*$', text, re.M) is not None, 'missing exact unittest completion')
    require(not re.search(r'^(?:FAILED|ERROR:|FAIL:|OK \()', text, re.M), 'failed or skipped unittest controls')
    observed = re.findall(r'^(test_[A-Za-z0-9_]+) \(([^\n]+)\) \.\.\. ', text, re.M)
    require(sorted(observed) == sorted(expected), 'missing, duplicate or unexpected named unit method')
    for method, identity in expected:
        # The evidence fixture runner emits progress to stdout during methods.
        # It cannot replace each method's final unittest result with a boolean.
        block = re.search(r'^' + re.escape(method) + r' \(' + re.escape(identity)
                          + r'\) \.\.\. (.*?)(?=^test_[A-Za-z0-9_]+ \(|^-{3,}|\Z)', text, re.M | re.S)
        require(block is not None and re.search(r'\bok\s*\Z', block.group(1)) is not None
                and not re.search(r'^(?:FAIL|ERROR|skipped)\b', block.group(1), re.M),
                'selected unit method did not pass: ' + identity)


def stable_environment(target):
    return {**environment(), 'RUSTUP_TOOLCHAIN': PINS['normal_rust_release'],
            'CARGO_TARGET_DIR': str(target), 'PYTHONPATH': str(ROOT / 'tools/safety')}


def run_static(args, evidence):
    env = stable_environment(args.output / 'target')
    evidence.run('normal-rustc', ['rustc', '-vV'], env=env, profile='STATIC')
    manifest = ROOT / 'tools/safety/inventory/Cargo.toml'
    evidence.run('build-parser', ['cargo', 'build', '--locked', '--offline', '--manifest-path', manifest],
                 env=env, profile='STATIC')
    helper = args.output / 'target/debug/rless-safety-inventory'
    evidence.run('inventory-parser', [sys.executable, ROOT / 'tools/safety/test_inventory_parser.py',
                 '--binary', helper], env=env, profile='STATIC', artifacts=[helper, sys.executable])
    for name, selections in UNIT_GROUPS.items():
        evidence.run(name, [sys.executable, '-m', 'unittest', '-v', *selections],
                     env=env, profile='STATIC', artifacts=[sys.executable], timeout=600)


def run_hooks(args, evidence):
    env = stable_environment(args.output / 'normal-target')
    injected_env = stable_environment(args.output / 'injected-target')
    evidence.run('normal-rustc', ['rustc', '-vV'], env=env, profile='STATIC')
    for kind, switches in (('debug', []), ('release', ['--release'])):
        evidence.run('build-normal-' + kind, ['cargo', 'build', '--locked', '--offline', '--bin', 'rless', *switches],
                     env=env, profile='NATIVE', timeout=600)
    evidence.run('build-injected', ['cargo', 'build', '--locked', '--offline', '--bin', 'rless',
                 '--features', 'safety-injection'], env=injected_env, profile='NATIVE', timeout=600)
    injected = args.output / 'injected-target/debug/rless'
    for kind in ('debug', 'release'):
        normal = args.output / 'normal-target' / kind / 'rless'
        evidence.run('hooks-' + kind, [sys.executable, ROOT / 'tools/safety/test_hooks.py',
                     '--normal', normal, '--injected', injected], env=env, profile='NATIVE',
                     artifacts=[normal, injected, sys.executable], timeout=300)
    evidence.run('fault-unit-tests', ['cargo', 'test', '--locked', '--offline', '--lib',
                 '--features', 'safety-injection', 'fault::tests::', '--', '--nocapture'],
                 env=injected_env, profile='NATIVE', timeout=600)


def run_detectors(args, evidence):
    from safety_runner import detectors, nightly_environment
    detectors(args, evidence)
    env = nightly_environment(args)
    for seed in PINS['miri_seeds']:
        evidence.run('miri-component-' + str(seed), ['cargo', 'miri', 'test', '--locked', '--offline',
            '--manifest-path', ROOT / 'tools/safety/components/Cargo.toml', '--lib', COMPONENT_TEST, '--', '--test-threads=1'],
            env={**env, 'MIRIFLAGS': f'-Zmiri-seed={seed}'}, profile='MIRI', timeout=600)


def exact(record, command, *, profile=None, exit_status=0):
    require(record['command'] == [str(value) for value in command], 'changed executed control command: ' + record['name'])
    require(profile is None or record['profile'] == profile, 'changed control profile')
    require(type(record['original_exit_status']) is int and record['original_exit_status'] == exit_status,
            'wrong original control exit: ' + record['name'])
    return Path(record['log']).read_text(errors='replace')


def diagnostic(record, pattern):
    require(record.get('expected_diagnostic') == pattern and record['original_exit_status'] != 0
            and re.search(pattern, Path(record['log']).read_text(errors='replace'), re.I | re.S),
            'intended control diagnostic was not retained: ' + record['name'])


def stable_flags(record, target, root):
    flags = record['build_flags']
    require(flags.get('RUSTUP_TOOLCHAIN') == PINS['normal_rust_release']
            and flags.get('CARGO_TARGET_DIR') == str(target)
            and flags.get('PYTHONPATH') == str(root / 'tools/safety'), 'changed control toolchain/target/module path')
    require(not any(key in flags for key in ('RUSTFLAGS', 'MIRIFLAGS', 'ASAN_OPTIONS', 'LSAN_OPTIONS')),
            'unexpected flags in normal control configuration')


def normalized(record, control, profile, root, *, seed=None):
    result = copy.deepcopy(record)
    result.update(name='m0-' + control, profile=profile, scenario_id=None, variant=control,
                  coverage_scope='partial', finding_ids=[], _validated_m0_controls=[control],
                  oracle_metadata=dict(abi_profile='BUILDER-GLIBC-2.44', corpus_sha256=sha(root / 'compat/corpus.json'),
                    seed=seed, test_selector=PROCEDURE + ':' + control, hook_sites=[], hook_ordinals=[], m0_controls=[control]))
    return result


def validate_static(records, out, root):
    required = {'observed-host-abi', 'normal-rustc', 'build-parser', 'inventory-parser', *UNIT_GROUPS}
    require(set(records) == required, 'static control selection is missing or unexpected')
    for name, record in records.items():
        if name != 'observed-host-abi':
            stable_flags(record, out / 'target', root)
    compiler = exact(records['normal-rustc'], ['rustc', '-vV'], profile='STATIC')
    require(re.search(r'^release: ' + re.escape(PINS['normal_rust_release']) + r'$', compiler, re.M)
            and re.search(r'^host: ' + re.escape(PINS['target']) + r'$', compiler, re.M),
            'normal compiler profile changed')
    exact(records['build-parser'], ['cargo', 'build', '--locked', '--offline', '--manifest-path',
          root / 'tools/safety/inventory/Cargo.toml'], profile='STATIC')
    helper = out / 'target/debug/rless-safety-inventory'
    parser = records['inventory-parser']
    text = exact(parser, [sys.executable, root / 'tools/safety/test_inventory_parser.py', '--binary', helper], profile='STATIC')
    require(parser['artifact_hashes'].get(str(helper)) == sha(helper), 'inventory parser binary not retained')
    validate_unittest(text, unittest_ids(['test_inventory_parser'], root, script=True))
    for name, selections in UNIT_GROUPS.items():
        text = exact(records[name], [sys.executable, '-m', 'unittest', '-v', *selections], profile='STATIC')
        validate_unittest(text, unittest_ids(selections, root))
    return [normalized(records[name], label, 'STATIC', root) for name, label in (
        ('inventory-coverage', 'inventory-omission-controls'),
        ('ownership-mutations', 'ownership-omission-controls'), ('binding-mutations', 'binding-mutation-controls'))]


def validate_hooks(records, out, root):
    require(set(records) == {'observed-host-abi', 'normal-rustc', 'build-normal-debug', 'build-normal-release',
                             'build-injected', 'hooks-debug', 'hooks-release', 'fault-unit-tests'},
            'hook-control selection is missing or unexpected')
    for name, record in records.items():
        if name != 'observed-host-abi':
            target = 'injected-target' if name in ('build-injected', 'fault-unit-tests') else 'normal-target'
            stable_flags(record, out / target, root)
    compiler = exact(records['normal-rustc'], ['rustc', '-vV'], profile='STATIC')
    require(re.search(r'^release: ' + re.escape(PINS['normal_rust_release']) + r'$', compiler, re.M)
            and re.search(r'^host: ' + re.escape(PINS['target']) + r'$', compiler, re.M),
            'normal compiler profile changed')
    for kind, switches in (('debug', []), ('release', ['--release'])):
        exact(records['build-normal-' + kind], ['cargo', 'build', '--locked', '--offline', '--bin', 'rless', *switches])
    exact(records['build-injected'], ['cargo', 'build', '--locked', '--offline', '--bin', 'rless', '--features', 'safety-injection'])
    injected = out / 'injected-target/debug/rless'
    for kind in ('debug', 'release'):
        normal = out / 'normal-target' / kind / 'rless'
        record = records['hooks-' + kind]
        for binary in (normal, injected):
            require(record['artifact_hashes'].get(str(binary)) == sha(binary), 'hook binary not retained')
        require(b'RLESS_FAULT' not in normal.read_bytes(), 'normal build contains fault configuration strings')
        text = exact(record, [sys.executable, root / 'tools/safety/test_hooks.py', '--normal', normal,
                             '--injected', injected], profile='NATIVE')
        validate_unittest(text, unittest_ids(['test_hooks'], root, script=True))
    text = exact(records['fault-unit-tests'], ['cargo', 'test', '--locked', '--offline', '--lib',
            '--features', 'safety-injection', 'fault::tests::', '--', '--nocapture'], profile='NATIVE')
    methods = re.findall(r'#\[test\]\s*fn\s+([a-zA-Z0-9_]+)', (root / 'src/fault.rs').read_text())
    require(methods and 'highest_ordinal_fires_once_then_exhaustion_is_reported_once' in methods,
            'counter exhaustion control disappeared')
    require(re.search(r'test result: ok\. ' + str(len(methods)) + r' passed; 0 failed; 0 ignored;', text),
            'missing exact Rust fault-test completion')
    for method in methods:
        require(re.search(r'^test fault::tests::' + re.escape(method) + r' \.\.\. ok$', text, re.M),
                'required Rust fault test did not run: ' + method)
    return [normalized(records[name], control, 'NATIVE', root) for name, control in (
        ('hooks-debug', 'normal-debug-hook-absence'), ('hooks-release', 'normal-release-hook-absence'),
        ('hooks-debug', 'malformed-hook-plans'), ('hooks-debug', 'trace-channel-controls'),
        ('fault-unit-tests', 'counter-exhaustion-control'))]


def pinned_nightly(records, report):
    for name, command in (('rustc', ['rustc', '-vV']), ('cargo', ['cargo', '-V']),
                          ('miri', ['cargo', 'miri', '--version'])):
        text = exact(records['tool-' + name], command, profile='STATIC')
        if name == 'rustc':
            require(re.search(r'^commit-hash: ' + re.escape(PINS['rustc_commit']) + r'[0-9a-f]*$', text, re.M)
                    and re.search(r'^host: ' + re.escape(PINS['target']) + r'$', text, re.M), 'nightly toolchain changed')
    require(report.get('tool_pins') == PINS and report.get('tool_files_sha256'), 'missing pinned nightly tools')
    tool_files = report['tool_files_sha256']
    flags = records['tool-rustc']['build_flags']
    require(flags.get('RUSTUP_HOME') and flags.get('CARGO_TARGET_DIR'), 'missing isolated toolchain identity')
    toolchain = Path(flags['RUSTUP_HOME']) / 'toolchains' / (PINS['nightly'] + '-' + PINS['target'])
    required_tools = [toolchain / 'bin' / name for name in ('rustc', 'cargo', 'miri')]
    required_tools.append(toolchain / 'lib/rustlib/src/rust/library/Cargo.lock')
    require(all(tool_files.get(str(path)) == sha(path) for path in required_tools), 'missing exact nightly tool files')
    for name, record in records.items():
        if name != 'observed-host-abi':
            current = record['build_flags']
            require(current.get('RUSTUP_TOOLCHAIN') == PINS['nightly']
                    and current.get('RUSTUP_HOME') == flags['RUSTUP_HOME']
                    and current.get('CARGO_TARGET_DIR') == flags['CARGO_TARGET_DIR'], 'control escaped pinned nightly configuration')


def validate_detectors(records, out, root, report):
    seeds = PINS['miri_seeds']
    required = {'observed-host-abi', 'tool-rustc', 'tool-cargo', 'tool-miri', 'build-detectors',
                'asan-safe', 'asan-uaf', 'lsan-leak', 'miri-setup', 'miri-alias', 'compiler-forbidden', 'compiler-safe'}
    required.update('miri-safe-' + str(seed) for seed in seeds)
    required.update('miri-component-' + str(seed) for seed in seeds)
    require(set(records) == required, 'detector/component selection is missing or unexpected')
    pinned_nightly(records, report)
    manifest = root / 'tools/safety/probes/Cargo.toml'
    exact(records['build-detectors'], ['cargo', 'build', '--locked', '--manifest-path', manifest,
                                      '--target', PINS['target'], '--bins'], profile='ASAN')
    require(records['build-detectors']['build_flags'].get('RUSTFLAGS') == PINS['asan_rustflags'], 'detectors lack sanitizer instrumentation')
    bins = out / 'target' / PINS['target'] / 'debug'
    # The pinned combined ASan/LSan runtime shares the LSAN exitcode flag.
    for name, status in (('asan-safe', 0), ('asan-uaf', 87), ('lsan-leak', 87)):
        binary = bins / ('safe' if name == 'asan-safe' else name)
        record = records[name]
        exact(record, [binary], exit_status=status)
        require(record['artifact_hashes'].get(str(binary)) == sha(binary), 'detector binary missing')
        flags = record['build_flags']
        require(flags.get('ASAN_OPTIONS') == PINS['asan_options'] and flags.get('LSAN_OPTIONS') == PINS['lsan_options']
                and sha(flags.get('ASAN_SYMBOLIZER_PATH', '/missing')) == PINS['symbolizer']['sha256'],
                'detector options/symbolizer differ from pins')
    diagnostic(records['asan-uaf'], r'ERROR: AddressSanitizer: heap-use-after-free')
    diagnostic(records['lsan-leak'], r'ERROR: LeakSanitizer: detected memory leaks.*8192 byte')
    exact(records['miri-setup'], ['cargo', 'miri', 'setup'], profile='MIRI')
    for seed in seeds:
        safe = records['miri-safe-' + str(seed)]
        exact(safe, ['cargo', 'miri', 'run', '--locked', '--manifest-path', manifest, '--bin', 'safe'], profile='MIRI')
        component = records['miri-component-' + str(seed)]
        text = exact(component, ['cargo', 'miri', 'test', '--locked', '--offline', '--manifest-path',
            root / 'tools/safety/components/Cargo.toml', '--lib', COMPONENT_TEST, '--', '--test-threads=1'], profile='MIRI')
        for record in (safe, component):
            require(record['build_flags'].get('MIRIFLAGS') == f'-Zmiri-seed={seed}', 'wrong Miri seed')
        require('running 3 tests' in text and 'test result: ok. 3 passed; 0 failed; 0 ignored;' in text
                and all(re.search(r'^test ' + re.escape(COMPONENT_TEST + '::' + name) + r' \.\.\. ok$', text, re.M)
                        for name in COMPONENT_CASES),
                'actual component Miri test did not pass')
    alias = records['miri-alias']
    diagnostic(alias, r'error: Undefined Behavior:.*(?:tag|borrow|alloc)')
    require(alias['original_exit_status'] != 0, 'Miri alias detector did not fail')
    require(alias['command'] == [str(x) for x in ['cargo', 'miri', 'run', '--locked', '--manifest-path', manifest, '--bin', 'miri-alias']],
            'wrong Miri alias command')
    exact(records['compiler-safe'], ['rustc', root / 'tools/safety/probes/safe.rs', '--out-dir', out / 'compiler-probes'])
    forbidden = records['compiler-forbidden']
    diagnostic(forbidden, r'error: usage of an `unsafe` block')
    require(forbidden['original_exit_status'] != 0 and forbidden['command'] == [str(x) for x in
        ['rustc', root / 'tools/safety/probes/forbidden.rs', '--out-dir', out / 'compiler-probes']], 'wrong compiler negative control')
    return [normalized(records[name], control, profile, root) for name, control, profile in (
        ('asan-uaf', 'asan-detector-control', 'ASAN'), ('lsan-leak', 'lsan-detector-control', 'LSAN'),
        ('miri-component-' + str(seeds[-1]), 'component-miri-controls', 'MIRI'))]


def validate_fuzz(records, out, root, report):
    required = {'observed-host-abi', 'tool-rustc', 'tool-cargo', 'tool-miri', 'fuzz-version', 'fuzz-model',
                'fuzz-oracle-negative', 'fuzz-minimization', 'fuzz-minimized-replay'}
    require(set(records) == required, 'fuzz control selection is missing or unexpected')
    pinned_nightly(records, report)
    version = exact(records['fuzz-version'], ['cargo', 'fuzz', '--version'], profile='FUZZ')
    require(re.search(r'\bcargo-fuzz ' + re.escape(PINS['cargo_fuzz_version']) + r'\s*$', version), 'wrong coverage engine version')
    engine = report['coverage_engine']
    require(engine['cargo_fuzz_version'] == PINS['cargo_fuzz_version']
            and engine['libfuzzer_sys_version'] == PINS['libfuzzer_sys_version'], 'wrong fuzz engine identity')
    checked(dict(path=engine['path'], sha256=engine['sha256']))
    require(records['fuzz-version']['build_flags'].get('PATH', '').split(os.pathsep)[0]
            == str(Path(engine['path']).parent), 'coverage engine was not selected on executable path')
    project, artifacts = out / 'fuzz-project', out / 'artifacts'
    original = {str(p.relative_to(root / 'tools/safety/fuzz')): sha(p)
                for p in (root / 'tools/safety/fuzz').rglob('*') if p.is_file()}
    for name, digest in original.items():
        require(sha(project / name) == digest, 'fuzz fixture differs from fingerprinted source')
    base = ['cargo', 'fuzz', 'run', '--fuzz-dir', project, '--target-dir', out / 'target']
    model = records['fuzz-model']
    text = exact(model, [*base, 'rollback_model', out / 'corpus', '--', '-runs=10000', '-seed=712',
        '-max_len=4096', '-timeout=5', '-print_final_stats=1', '-artifact_prefix=' + str(artifacts) + '/'], profile='FUZZ')
    coverage, counts = re.findall(r'cov: (\d+)', text), re.findall(r'stat::number_of_executed_units:\s*(\d+)', text)
    require(coverage and max(map(int, coverage)) >= 2 and counts and int(counts[-1]) >= 10000,
            'fuzz engine lacks real coverage or executions')
    negative = records['fuzz-oracle-negative']
    diagnostic(negative, r'injected rollback violation: detector control.*(?:libFuzzer|deadly signal)')
    require(negative['original_exit_status'] != 0 and negative['command'] == [str(x) for x in
        [*base, 'rollback_fault', out / 'fault-corpus', '--', '-runs=1', '-seed=712',
         '-artifact_prefix=' + str(artifacts) + '/']], 'wrong fuzz negative-control command')
    require((out / 'fault-corpus/injected').read_bytes() == b'prefix-RLESS-suffix', 'negative fuzz input changed')
    crashes = sorted(artifacts.glob('crash-*'))
    require(crashes, 'no retained detector crash input')
    require(b'RLESS' in crashes[0].read_bytes(), 'retained crash no longer triggers intended detector')
    exact(records['fuzz-minimization'], ['cargo', 'fuzz', 'tmin', '--fuzz-dir', project, '--target-dir', out / 'target',
          'rollback_fault', crashes[0], '--', '-runs=1000', '-seed=712'], profile='FUZZ')
    minimized = checked({key: report['minimized_reproducer'][key] for key in ('path', 'sha256')})
    require(b'RLESS' in minimized.read_bytes(), 'minimized detector input no longer triggers intended condition')
    replay = records['fuzz-minimized-replay']
    diagnostic(replay, r'injected rollback violation: detector control')
    require(replay['original_exit_status'] != 0 and replay['command'] == [str(x) for x in
        [*base, 'rollback_fault', minimized, '--', '-runs=1']], 'wrong minimized replay command')
    require(report.get('retained_fuzz_artifacts'), 'fuzz artifacts not retained')
    retained = report['retained_fuzz_artifacts']
    actual = {str(p) for directory in (artifacts, project / 'artifacts') for p in directory.rglob('*') if p.is_file()}
    require(len(retained) == len(actual) and {item['path'] for item in retained} == actual,
            'retained fuzz artifact inventory changed')
    for artifact in report['retained_fuzz_artifacts']:
        checked({key: artifact[key] for key in ('path', 'sha256')})
    return [normalized(replay, 'fuzz-engine-negative-control', 'FUZZ', root, seed=712)]


def validate_external_report(path, root=ROOT):
    path, root = Path(path).resolve(), Path(root).resolve()
    outer = json.loads(path.read_text())
    require(outer.get('report_kind') == 'm0-controls' and outer.get('procedure') == PROCEDURE
            and outer.get('schema_version') == 1, 'unknown M0 control procedure')
    require(outer['input_fingerprints'] == fingerprint(root)
            and outer['source_commit'] == git('rev-parse', 'HEAD', root=root).decode().strip(), 'stale M0 control inputs')
    require(outer.get('lanes') and set(outer['lanes']) <= set(LANES), 'unknown or empty M0 control lanes')
    records, tool_files = [], {}
    for lane, reference in outer['lanes'].items():
        summary = checked(reference)
        raw = json.loads(summary.read_text())
        require('report_kind' not in raw, 'nested control report must be raw execution evidence')
        report = validate_report(summary, root)
        require(report['source_commit'] == outer['source_commit'], 'control source commit changed')
        selected = {item['name']: item for item in report['records']}
        host = exact(selected['observed-host-abi'], ['getconf', 'GNU_LIBC_VERSION'], profile='ABI')
        require(host.strip() == 'glibc 2.44', 'host controls were not executed on builder ABI')
        out = summary.parent
        if lane == 'static':
            records.extend(validate_static(selected, out, root))
        elif lane == 'hooks':
            records.extend(validate_hooks(selected, out, root))
        elif lane == 'detectors':
            records.extend(validate_detectors(selected, out, root, report))
        else:
            records.extend(validate_fuzz(selected, out, root, report))
        tool_files.update(report.get('tool_files_sha256', {}))
    return dict(schema_version=1, source_commit=outer['source_commit'], input_fingerprints=outer['input_fingerprints'],
                tool_pins=PINS, tool_files_sha256=tool_files, records=records, status='passed', runtime_safety_verified=False)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('run', 'verify'))
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--tool-home', type=Path)
    parser.add_argument('--lanes', nargs='+', choices=LANES, default=list(LANES))
    args = parser.parse_args()
    if args.action == 'verify':
        validate_report(args.output)
        print('M0 controls revalidated; independent M0 review and final safety remain separate.')
        return 0
    if set(args.lanes) & {'detectors', 'fuzz'} and args.tool_home is None:
        parser.error('--tool-home is required for detectors/Miri/fuzz')
    if len(set(args.lanes)) != len(args.lanes):
        parser.error('duplicate control lanes are not allowed')
    if args.tool_home is not None:
        args.tool_home = args.tool_home.resolve()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    outer = dict(schema_version=1, report_kind='m0-controls', procedure=PROCEDURE,
        source_commit=git('rev-parse', 'HEAD').decode().strip(), input_fingerprints=fingerprint(), lanes={})
    destination = args.output
    for lane in args.lanes:
        args.output = destination / lane
        evidence = Evidence(args.output)
        evidence.run('observed-host-abi', ['getconf', 'GNU_LIBC_VERSION'], profile='ABI')
        try:
            if lane == 'fuzz':
                from safety_runner import fuzz
                fuzz(args, evidence)
            else:
                globals()['run_' + lane](args, evidence)
        except (OSError, ValueError) as error:
            evidence.unavailable('runner-error', str(error), 'STATIC')
        evidence.finish()
        outer['lanes'][lane] = ref(args.output / 'summary.json')
        write_json(destination / 'summary.json', outer)
    validate_report(destination / 'summary.json')
    print('M0 control evidence: ' + str(destination / 'summary.json'))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
