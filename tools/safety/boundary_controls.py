#!/usr/bin/env python3
"""Execute and rederive supplemental memory/IO controls for each required ABI.

Retained artifacts are evidence, never trusted pass bits. Production source is
unchanged; negative controls compile explicit mutations in isolated copies.
"""
import argparse
import copy
import io
import json
import os
from pathlib import Path
import platform
import re
import shutil
import sys

from common import Evidence, PINS, ROOT, environment, fingerprint, git, sha, validate_report, write_json
from linked_object import verify
from live_heap import Samples, require_balance
from native_alloc import CFLAGS, RUSTFLAGS as ALLOC_FLAGS
from test_first_buffer import validate as validate_first, validate_expected_failure as validate_first_failure
from test_stdin_delete import validate as validate_stdin, validate_expected_failure as validate_stdin_failure
from test_io_boundaries import run as replay_boundaries

PROCEDURE = 'SUPPLEMENTAL-BOUNDARY-CONTROLS-v1'
BASE_CONTROLS = ('boundary-suite', 'first-buffer', 'stdin-delete', 'linked-object', 'memory-mutations')
ABI_CONTROLS = {f'{name}-{profile.lower()}': ('C0-COVER-2', profile)
                for profile in ('NATIVE', 'ASAN') for name in BASE_CONTROLS}
ABI_CONTROLS.update({'live-heap-balance': ('C0-COVER-2', 'ASAN'), 'live-heap-mutant': ('C0-COVER-2', 'ASAN')})


def require(value, message):
    if not value:
        raise ValueError(message)


def ref(path):
    return dict(path=str(Path(path).resolve()), sha256=sha(path))


def checked(value):
    require(isinstance(value, dict) and set(value) == {'path', 'sha256'}, 'invalid boundary artifact reference')
    p = Path(value['path'])
    require(p.is_file() and sha(p) == value['sha256'], 'missing or changed boundary artifact')
    return p


def load(value):
    return json.loads(checked(value).read_text())


def controls(profile):
    result = [name + '-' + profile.lower() for name in BASE_CONTROLS]
    return result + (['live-heap-balance', 'live-heap-mutant'] if profile == 'ASAN' else [])


def mutant_text(kind, source):
    if kind == 'firstbuf':
        start = source.index('            // A failed first allocation')
        end = source.index('            bn = (*thisfile).buflist.prev;', start)
        return source[:start] + source[end:]
    if kind in ('stdin-leak', 'stdin-close'):
        old = '    crate::ch::dispose_ifile_state(h);'
        require(source.count(old) == 1, 'stdin mutation target changed')
        return source.replace(old, '    // retained-state release deliberately skipped' if kind == 'stdin-leak'
                              else '    ::libc::close(0);\n' + old)
    if kind == 'heap-leak':
        old = ('        if crate::fault::hit(crate::fault::Site::FilterBefore) {\n'
               '            clear_pattern(filter);\n            free(filter.cast());')
        require(source.count(old) == 1, 'heap cleanup mutation target changed')
        return source.replace(old, '        if crate::fault::hit(crate::fault::Site::FilterBefore) {')
    raise ValueError('unknown boundary mutant')


MUTANT_PATH = {'firstbuf': 'src/ch.rs', 'stdin-leak': 'src/ifile.rs',
               'stdin-close': 'src/ifile.rs', 'heap-leak': 'src/search.rs'}


def snapshot_mutant(kind, destination, root=ROOT):
    paths = fingerprint(root)['files']
    destination.mkdir(parents=True, exist_ok=False)
    for name in paths:
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / name, target)
    path = destination / MUTANT_PATH[kind]
    path.write_text(mutant_text(kind, path.read_text()))
    return dict(kind=kind, source=str(destination), changed_path=MUTANT_PATH[kind],
                changed_sha256=sha(path), source_files_sha256={name: sha(destination / name) for name in paths})


def validate_mutant(mutation, root):
    kind = mutation['kind']; source = Path(mutation['source'])
    require(kind in MUTANT_PATH and mutation['changed_path'] == MUTANT_PATH[kind], 'unknown mutation identity')
    current = fingerprint(root)['files']
    require(set(mutation['source_files_sha256']) == set(current), 'mutant source manifest incomplete')
    for name in current:
        path = source / name
        require(path.is_file() and sha(path) == mutation['source_files_sha256'][name], 'mutant source changed')
        if name == MUTANT_PATH[kind]:
            require(path.read_text() == mutant_text(kind, (root / name).read_text()), 'mutant does not implement exact recipe')
        else:
            require(sha(path) == current[name], 'unrelated source change in mutant')
    require(mutation['changed_sha256'] == sha(source / MUTANT_PATH[kind]), 'mutant hash changed')


def validate_heap(result, directory, binary, object_file, *, mutant=False):
    directory = Path(directory)
    require(result['helper_sha256'] == sha(ROOT/'tools/safety/live_heap.py')
            and result['shim_source_sha256'] == sha(ROOT/'tools/safety/native/live_heap.c')
            and result['observer_sha256'] == sha(ROOT/'tools/compat/run_v2.py'), 'heap observer/source changed')
    from live_heap import scenario
    expected_case = scenario(result['site'],10)
    require(all(result['case'][key] == expected_case[key] for key in ('argv','files','events')), 'heap scenario changed')
    require(result['case']['env']['ASAN_OPTIONS'] == PINS['asan_options']+':symbolize=0:log_path='+str(directory/'sanitizer/raw')
            and result['case']['env']['LSAN_OPTIONS'] == PINS['lsan_options'], 'heap detector environment changed')
    require(result['binary_sha256'] == sha(binary) and result['shim_sha256'] == sha(object_file), 'heap binary/object changed')
    raw = directory / 'heap-trace.bin'
    require(result['heap_trace_sha256'] == sha(raw), 'heap trace changed')
    samples = Samples(io.BytesIO(raw.read_bytes())); samples.drain()
    require(samples.records == result['samples'], 'heap samples differ from raw trace')
    checkpoints = result['checkpoints']
    require(len(checkpoints) == 9, 'heap warmup/cycle checkpoints incomplete')
    previous = 0
    for checkpoint in checkpoints:
        sequence = checkpoint['sequence']
        require(type(sequence) is int and previous < sequence <= len(samples.records), 'heap checkpoint sequence stale')
        require(all(checkpoint.get(k) == v for k, v in samples.records[sequence-1].items()), 'heap checkpoint differs from raw sample')
        syscall = checkpoint['blocked_syscall']
        require(syscall[0] in ('0', '271') and int(syscall[0]) == checkpoint['wait_syscall']
                and int(syscall[1], 0) == checkpoint['wait_argument']
                and (syscall[0] != '271' or int(syscall[2], 0) == 1), 'heap checkpoint is not the observed input barrier')
        previous = sequence
    require(result['cycles'] == 10 and result['ordinal'] == (3 if result['site'] else None), 'heap scenario changed')
    require([x['label'] for x in checkpoints] == ['warmup'] + [f'cycle-{i}' for i in range(3,11)], 'heap labels incomplete')
    from test_fault_paths import marker
    expected = b'' if result['site'] is None else marker({'filter-before':7, 'filter-after':8, 'regex':5}[result['site']],3)
    require(bytes.fromhex(result['fault_trace_hex']) == expected, 'heap fault identity changed')
    if mutant:
        require(result['site'] == 'filter-before', 'wrong heap mutant scenario')
        baseline = checkpoints[0]['allocated_bytes']
        deltas = [x['allocated_bytes']-baseline for x in checkpoints[1:]]
        require(all(type(x) is int and x > 0 for x in deltas) and len(set(deltas)) == 1,
                'cleanup mutant lacks independently observed retained live bytes')
        try:
            require_balance(checkpoints, 10)
        except ValueError:
            return deltas  # Deliberately independent of LSAN status or result pass bits.
        raise ValueError('heap cleanup mutant was accepted')
    require_balance(checkpoints,10)
    require(result['observation']['status'] == 0 and result['observation']['termios_restored'] is True,
            'heap status or terminal restoration failed')
    require(not list((directory / 'sanitizer').iterdir()), 'heap sanitizer diagnostic')
    return [0]*8


def validate_steps(outer, steps, output, root):
    profile = outer['profile']; target = ['--target', PINS['target']] if profile == 'ASAN' else []
    suffix = (Path(PINS['target']) if target else Path()) / 'debug/rless'
    objects = {name: output / (name + '.o') for name in outer['objects']}
    links = {'normal': [], 'allocation': [objects['allocation'], '-Wl,--wrap=calloc', '-Wl,--wrap=strdup'],
             'first-buffer': [objects['first-buffer'], '-Wl,--wrap=calloc', '-ldl']}
    if profile == 'ASAN':
        links['live-heap'] = [objects['live-heap'], '-Wl,--wrap=read', '-Wl,--wrap=ppoll']
    binaries = {name: output / ('target-' + name) / suffix for name in links}
    for kind in outer['mutants']:
        name = 'mutant-' + kind
        links[name] = links['first-buffer'] if kind == 'firstbuf' else links.get('live-heap', []) if kind == 'heap-leak' else []
        binaries[name] = output / ('target-' + name) / suffix
    expected = {'observed-host-abi': ['getconf', 'GNU_LIBC_VERSION']}
    for name, link in links.items():
        expected['build-' + name] = ['cargo', 'rustc', '--locked', '--offline', '--bin', 'rless', *target,
            *(['--features', 'safety-injection'] if name in ('live-heap', 'mutant-heap-leak') else []), '--',
            *['-Clink-arg=' + str(value) for value in link]]
    sources = {'allocation': 'alloc_fault.c', 'first-buffer': 'first_buffer_fault.c', 'live-heap': 'live_heap.c'}
    for name, path in objects.items():
        expected['compile-' + name] = ['cc', *CFLAGS, '-c', str(root / 'tools/safety/native' / sources[name]), '-o', str(path)]
    def script(name, file, arguments):
        expected[name] = [str(root / 'tools/safety' / file), *[str(x) for x in arguments]]
    script('runtime-identity', 'runtime_identity.py', ['--binary', binaries['normal'], '--output', output/'runtime-identity'])
    script('linked-object-mutations', 'linked_controls.py', ['--binary', binaries['allocation'], '--object', objects['allocation'], '--anchor', '__wrap_calloc'])
    script('first-buffer', 'test_first_buffer.py', [binaries['first-buffer'], output/'first-buffer', objects['first-buffer']])
    script('stdin-delete', 'test_stdin_delete.py', [binaries['normal'], output/'stdin-delete'])
    script('boundary-suite', 'test_io_boundaries.py', ['--binary', binaries['normal'], '--output', output/'boundary-suite'])
    for kind in outer['mutants']:
        name = 'mutant-' + kind
        if kind == 'firstbuf':
            script(name, 'test_first_buffer.py', [binaries[name], output/name, objects['first-buffer']])
        elif kind != 'heap-leak':
            script(name, 'test_stdin_delete.py', [binaries[name], output/name])
        else:
            script(name, 'live_heap.py', ['--binary', binaries[name], '--shim', objects['live-heap'],
                                         '--output', output/name, '--site', 'filter-before'])
    for site in outer['heaps']:
        script('heap-'+site, 'live_heap.py', ['--binary', binaries['live-heap'], '--shim', objects['live-heap'],
            '--output', output/('heap-'+site), *(['--site', site] if site != 'none' else [])])
    require(set(steps) == set(expected), 'missing or extra supplemental execution step')
    for name, command in expected.items():
        record = steps[name]; actual = record['command']
        is_script = name not in ('observed-host-abi',) and not name.startswith(('build-', 'compile-'))
        require((actual[1:] if is_script else actual) == command, 'boundary command identity changed: '+name)
        expected_status = 1 if name.startswith('mutant-') else 0
        require(record['expected_exit_status'] == expected_status, 'boundary expected exit changed')
        cwd = Path(outer['mutants'][name.removeprefix('build-mutant-')]['source']) if name.startswith('build-mutant-') else root
        require(record.get('cwd') == str(cwd.resolve()), 'boundary execution cwd changed')
        if name.startswith('build-'):
            flags = record['build_flags']
            rustflags = ((PINS['asan_rustflags']+' ') if profile == 'ASAN' else '') + ALLOC_FLAGS
            require(flags.get('RUSTFLAGS') == rustflags and flags.get('CARGO_TARGET_DIR') == str(output/('target-'+name[6:])),
                    'boundary compiler flags/target changed')
            if profile == 'ASAN':
                require(flags.get('RUSTUP_TOOLCHAIN') == PINS['nightly'], 'boundary compiler toolchain changed')
        if profile == 'ASAN' and not name.startswith('compile-'):
            require(record['build_flags'].get('ASAN_OPTIONS') == PINS['asan_options']+(':symbolize=0' if outer['abi_profile']=='REFERENCE-GLIBC-2.41' else '')
                    and record['build_flags'].get('LSAN_OPTIONS') == PINS['lsan_options'], 'boundary detector options changed')
    for name, reference in outer['binaries'].items():
        require(checked(reference) == binaries[name], 'boundary binary path not bound to build')
    for name, reference in outer['objects'].items():
        require(checked(reference) == objects[name], 'boundary object path not bound to compilation')
    for name, mutation in outer['mutants'].items():
        require(checked(mutation['binary']) == binaries['mutant-'+name], 'mutant binary not bound to mutated build')


def validate_external_report(path, root=ROOT, *, reference_context=None, tool_home=None):
    path, root = Path(path).resolve(), Path(root).resolve()
    outer = json.loads(path.read_text())
    require(outer.get('report_kind') == 'boundary-controls' and outer.get('procedure') == PROCEDURE
            and outer.get('schema_version') == 1, 'unknown supplemental boundary procedure')
    require(outer['source_commit'] == git('rev-parse','HEAD',root=root).decode().strip()
            and outer['input_fingerprints'] == fingerprint(root), 'stale boundary source inputs')
    profile = outer['profile']; abi = outer['abi_profile']
    require(profile in ('NATIVE','ASAN') and abi in ('BUILDER-GLIBC-2.44','REFERENCE-GLIBC-2.41'), 'unknown boundary ABI/profile')
    execution = validate_report(checked(outer['execution']), root)
    require(execution['source_commit'] == outer['source_commit'], 'boundary execution commit differs')
    steps = {x['name']: x for x in execution['records']}
    validate_steps(outer, steps, path.parent, root)
    if profile == 'ASAN' and tool_home is None:
        tool_home = Path(steps['build-normal']['build_flags']['RUSTUP_HOME']).parent.resolve()
        require(not tool_home.is_relative_to(path.parent) and not tool_home.is_relative_to(root),
                'compiler installation cannot be supplied by the retained receipt')
    abi_log = Path(steps['observed-host-abi']['log']).read_text().strip()
    require(steps['observed-host-abi']['command'] == ['getconf','GNU_LIBC_VERSION']
            and abi_log == 'glibc ' + ('2.41' if abi == 'REFERENCE-GLIBC-2.41' else '2.44'), 'boundary ABI not observed')
    binaries = {name: checked(value) for name,value in outer['binaries'].items()}
    objects = {name: checked(value) for name,value in outer['objects'].items()}
    from runtime_identity import validate as validate_runtime
    runtime_path=checked(outer['runtime_identity'])
    validate_runtime(json.loads(runtime_path.read_text()),runtime_path.parent,binaries['normal'],abi_profile=abi,reference_context=reference_context)
    required_pairs = {'allocation','first-buffer'} | ({'live-heap'} if profile == 'ASAN' else set())
    require(set(outer['linked_pairs']) == required_pairs, 'required linked counterpart pair missing')
    for name, reference in outer['linked_pairs'].items():
        anchor = '__wrap_read' if name == 'live-heap' else '__wrap_calloc'
        require(load(reference) == verify(binaries[name], objects[name], anchor), 'linked object proof changed')
        other = binaries['first-buffer' if name != 'first-buffer' else 'allocation']
        try:
            verify(other, objects[name], anchor)
        except ValueError:
            pass
        else:
            raise ValueError('mismatched retained object pair was accepted')
    from m0_controls import unittest_ids, validate_unittest
    validate_unittest(Path(steps['linked-object-mutations']['log']).read_text(), unittest_ids(['test_linked_object.LinkedObjectMutations'],root))
    first = load(outer['results']['first-buffer']); first_dir = checked(outer['results']['first-buffer']).parent
    validate_first(first, first_dir, binaries['first-buffer'], objects['first-buffer'])
    stdin = load(outer['results']['stdin-delete']); stdin_dir = checked(outer['results']['stdin-delete']).parent
    validate_stdin(stdin, stdin_dir, binaries['normal'])
    boundary_path = checked(outer['results']['boundary-suite'])
    boundary = json.loads(boundary_path.read_text())
    require(replay_boundaries(binaries['normal'], boundary_path.parent, retained=boundary), 'retained boundary assertions failed')
    # Every one of the thirty recorded cases must reject a wrong status. This
    # re-runs assertions, rather than trusting a negative-control count.
    for name in boundary:
        changed = copy.deepcopy(boundary)
        old = changed[name]['observed']['status']; changed[name]['observed']['status'] = 0 if old else 1
        require(not replay_boundaries(binaries['normal'], boundary_path.parent, retained=changed),
                'boundary negative control accepted: '+name)
    required_mutants = {'firstbuf','stdin-close'} | ({'stdin-leak','heap-leak'} if profile == 'ASAN' else set())
    require(set(outer['mutants']) == required_mutants, 'runtime mutation control missing')
    for name, mutation in outer['mutants'].items():
        validate_mutant(mutation, root)
        binary = checked(mutation['binary']); result_path = checked(mutation['result']); result = json.loads(result_path.read_text())
        require(result.get('binary_sha256') == sha(binary), 'mutant result belongs to a different binary')
        if name == 'heap-leak':
            validate_heap(result,result_path.parent,binary,objects['live-heap'],mutant=True)
        elif name == 'firstbuf':
            validate_first_failure(result,result_path.parent,binary,objects['first-buffer'])
        else:
            validate_stdin_failure(result,result_path.parent,binary,name)
        from mutation_provenance import validate as validate_mutant_binary
        validate_mutant_binary(mutation,profile,objects,binary,root=root,tool_home=tool_home)
    if profile == 'ASAN':
        require(set(outer['heaps']) == {'none','filter-before','filter-after','regex'}, 'heap session missing')
        for name,value in outer['heaps'].items():
            report = load(value)
            require(report['site'] == (None if name == 'none' else name), 'heap session identity changed')
            validate_heap(report,checked(value).parent,binaries['live-heap'],objects['live-heap'])
        require(load(outer['detector_environment']) == {k: stdin['safety_environment'][k]
                for k in ('ASAN_OPTIONS','LSAN_OPTIONS','ASAN_SYMBOLIZER_PATH') if k in stdin['safety_environment']}, 'detector environment differs')
        require(stdin['safety_environment']['ASAN_OPTIONS'] == PINS['asan_options']+(':symbolize=0' if abi=='REFERENCE-GLIBC-2.41' else '')
                and stdin['safety_environment']['LSAN_OPTIONS'] == PINS['lsan_options'], 'detector settings weakened')
    normalized = []
    mapping = {'boundary-suite':'boundary-suite','first-buffer':'first-buffer','stdin-delete':'stdin-delete',
               'linked-object':'linked-object-mutations','memory-mutations':'mutant-firstbuf',
               'live-heap-balance':'heap-none','live-heap-mutant':'mutant-heap-leak'}
    for control in controls(profile):
        base = control.removesuffix('-'+profile.lower())
        item = copy.deepcopy(steps[mapping[base]])
        item.update(name=control,profile=profile,finding_ids=[],_validated_m0_controls=[control],
            oracle_metadata=dict(abi_profile=abi,corpus_sha256=sha(root/'compat/corpus.json'),seed=None,
                test_selector=PROCEDURE+':'+control,hook_sites=[],hook_ordinals=[],m0_controls=[control]))
        normalized.append(item)
    return dict(schema_version=1,source_commit=outer['source_commit'],input_fingerprints=outer['input_fingerprints'],
                tool_pins=PINS,records=normalized,status='passed',runtime_safety_verified=False)


def run_campaign(args):
    args.output = args.output.resolve(); args.output.mkdir(parents=True,exist_ok=False)
    profile = args.profile.upper(); target = ['--target',PINS['target']] if profile == 'ASAN' else []
    abi = {'2.44':'BUILDER-GLIBC-2.44','2.41':'REFERENCE-GLIBC-2.41'}.get(platform.libc_ver()[1])
    require(platform.libc_ver()[0] == 'glibc' and abi is not None,'unsupported boundary execution ABI')
    execution = Evidence(args.output/'execution')
    env = environment(profile == 'ASAN', args.tool_home)
    reference=None
    if abi=='REFERENCE-GLIBC-2.41':
        require(args.reference_runtime,'reference campaign requires the pinned runtime context')
        from runtime_identity import reference_context as observed_reference
        reference=observed_reference()
    if profile == 'ASAN':
        env.update(RUSTFLAGS=PINS['asan_rustflags'],ASAN_OPTIONS=PINS['asan_options']+(':symbolize=0' if reference else ''),LSAN_OPTIONS=PINS['lsan_options'])
        if reference is None:
            symbolizer=shutil.which(PINS['symbolizer']['name'])
            require(symbolizer is not None and sha(symbolizer)==PINS['symbolizer']['sha256'],'symbolizer does not match pin')
            env['ASAN_SYMBOLIZER_PATH']=symbolizer
        else:
            env.pop('ASAN_SYMBOLIZER_PATH',None)
    env['RUSTFLAGS'] = (env.get('RUSTFLAGS','')+' '+ALLOC_FLAGS).strip()
    config = args.output/'detector-environment.json'
    if profile == 'ASAN':
        write_json(config,{k:env[k] for k in ('ASAN_OPTIONS','LSAN_OPTIONS','ASAN_SYMBOLIZER_PATH') if k in env})
    tests_env = {**environment(), **({k:env[k] for k in ('ASAN_OPTIONS','LSAN_OPTIONS','ASAN_SYMBOLIZER_PATH') if k in env} if profile=='ASAN' else {})}
    if profile == 'ASAN': tests_env['RLESS_SAFETY_ENV'] = str(config)
    outer = dict(schema_version=1,report_kind='boundary-controls',procedure=PROCEDURE,profile=profile,abi_profile=abi,
        source_commit=execution.report['source_commit'],input_fingerprints=fingerprint(),
        execution={},binaries={},objects={},linked_pairs={},results={},mutants={},heaps={})
    def invoke(name,command,*,selected_env=None,cwd=ROOT,expected=0,artifacts=()):
        record = execution.run(name,command,env=selected_env or tests_env,cwd=cwd,expected=expected,
                               profile=profile,timeout=600,artifacts=artifacts)
        require(record['status']=='passed','boundary command failed: '+name)
        return record
    def build(name,*,source=ROOT,links=(),features=False):
        destination=args.output/('target-'+name)
        selected={**env,'CARGO_TARGET_DIR':str(destination)}
        command=['cargo','rustc','--locked','--offline','--bin','rless',*target,
                 *(['--features','safety-injection'] if features else []),'--']
        command += ['-Clink-arg='+str(value) for value in links]
        invoke('build-'+name,command,selected_env=selected,cwd=source)
        binary=destination/(PINS['target'] if target else '')/'debug/rless'
        execution.report['records'][-1]['artifact_hashes'][str(binary)]=sha(binary); execution.save()
        return binary
    def compile_object(name,source):
        destination=args.output/(name+'.o')
        invoke('compile-'+name,['cc',*CFLAGS,'-c',source,'-o',destination],selected_env=environment())
        execution.report['records'][-1]['artifact_hashes'][str(destination)]=sha(destination);execution.save()
        return destination
    def first_run(name,binary,object_file,expected=0):
        output=args.output/name
        invoke(name,[sys.executable,ROOT/'tools/safety/test_first_buffer.py',binary,output,object_file],
               expected=expected,artifacts=(binary,object_file))
        return output/'results.json'
    def stdin_run(name,binary,expected=0):
        output=args.output/name
        invoke(name,[sys.executable,ROOT/'tools/safety/test_stdin_delete.py',binary,output],expected=expected,artifacts=(binary,))
        return output/'results.json'
    invoke('observed-host-abi',['getconf','GNU_LIBC_VERSION'])
    normal=build('normal'); outer['binaries']['normal']=ref(normal)
    runtime=args.output/'runtime-identity'
    invoke('runtime-identity',[sys.executable,ROOT/'tools/safety/runtime_identity.py','--binary',normal,'--output',runtime],artifacts=(normal,))
    outer['runtime_identity']=ref(runtime/'result.json')
    alloc=compile_object('allocation',ROOT/'tools/safety/native/alloc_fault.c')
    first=compile_object('first-buffer',ROOT/'tools/safety/native/first_buffer_fault.c')
    objects={'allocation':alloc,'first-buffer':first}
    pairs={'allocation':build('allocation',links=(alloc,'-Wl,--wrap=calloc','-Wl,--wrap=strdup')),
           'first-buffer':build('first-buffer',links=(first,'-Wl,--wrap=calloc','-ldl'))}
    if profile=='ASAN':
        objects['live-heap']=compile_object('live-heap',ROOT/'tools/safety/native/live_heap.c')
        pairs['live-heap']=build('live-heap',links=(objects['live-heap'],'-Wl,--wrap=read','-Wl,--wrap=ppoll'),features=True)
        outer['detector_environment']=ref(config)
    for name,binary in pairs.items():
        outer['binaries'][name]=ref(binary);outer['objects'][name]=ref(objects[name])
        proof=args.output/(name+'-linked.json');write_json(proof,verify(binary,objects[name],'__wrap_read' if name=='live-heap' else '__wrap_calloc'))
        outer['linked_pairs'][name]=ref(proof)
    invoke('linked-object-mutations',[sys.executable,ROOT/'tools/safety/linked_controls.py','--binary',pairs['allocation'],'--object',alloc,'--anchor','__wrap_calloc'],artifacts=(pairs['allocation'],alloc))
    outer['results']['first-buffer']=ref(first_run('first-buffer',pairs['first-buffer'],first))
    outer['results']['stdin-delete']=ref(stdin_run('stdin-delete',normal))
    boundary=args.output/'boundary-suite'
    invoke('boundary-suite',[sys.executable,ROOT/'tools/safety/test_io_boundaries.py','--binary',normal,'--output',boundary],artifacts=(normal,))
    outer['results']['boundary-suite']=ref(boundary/'results.json')
    kinds=['firstbuf','stdin-close']+(['stdin-leak','heap-leak'] if profile=='ASAN' else [])
    for kind in kinds:
        mutation=snapshot_mutant(kind,args.output/('source-'+kind))
        source=Path(mutation['source'])
        if kind=='firstbuf':
            binary=build('mutant-'+kind,source=source,links=(first,'-Wl,--wrap=calloc','-ldl'))
            result=first_run('mutant-'+kind,binary,first,expected=1)
        elif kind=='heap-leak':
            binary=build('mutant-'+kind,source=source,links=(objects['live-heap'],'-Wl,--wrap=read','-Wl,--wrap=ppoll'),features=True)
            output=args.output/('mutant-'+kind)
            invoke('mutant-'+kind,[sys.executable,ROOT/'tools/safety/live_heap.py','--binary',binary,'--shim',objects['live-heap'],
                '--output',output,'--site','filter-before'],expected=1,artifacts=(binary,objects['live-heap']))
            result=output/'result.json'
        else:
            binary=build('mutant-'+kind,source=source)
            result=stdin_run('mutant-'+kind,binary,expected=1)
        mutation.update(binary=ref(binary),result=ref(result));outer['mutants'][kind]=mutation
    if profile=='ASAN':
        for site in ('none','filter-before','filter-after','regex'):
            output=args.output/('heap-'+site)
            invoke('heap-'+site,[sys.executable,ROOT/'tools/safety/live_heap.py','--binary',pairs['live-heap'],
                '--shim',objects['live-heap'],'--output',output,*(['--site',site] if site!='none' else [])],
                artifacts=(pairs['live-heap'],objects['live-heap']))
            outer['heaps'][site]=ref(output/'result.json')
    require(execution.finish(),'boundary execution report failed')
    outer['execution']=ref(execution.directory/'summary.json')
    summary=args.output/'summary.json';write_json(summary,outer)
    validate_external_report(summary,tool_home=args.tool_home,reference_context=reference)
    print(json.dumps(dict(passed=True,controls=controls(profile),report=str(summary))))
    return 0


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile',choices=('native','asan'))
    parser.add_argument('--tool-home',type=Path)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--verify',action='store_true')
    parser.add_argument('--reference-runtime',action='store_true')
    parser.add_argument('--normalized-output',type=Path)
    args=parser.parse_args();os.umask(0o077)
    if args.verify:
        if args.tool_home is not None:os.environ['RLESS_SAFETY_TOOL_HOME']=str(args.tool_home.resolve())
        if args.reference_runtime:
            from runtime_identity import reference_context as observed_reference
            result=validate_external_report(args.output,reference_context=observed_reference(),tool_home=args.tool_home)
        else:
            result=validate_report(args.output)
        if args.normalized_output is not None:write_json(args.normalized_output,result)
        print('Supplemental controls rederived; M0 ownership review remains separate.');return 0
    if args.profile is None or args.profile=='asan' and args.tool_home is None:
        parser.error('--profile and ASAN --tool-home are required')
    return run_campaign(args)


if __name__=='__main__':
    raise SystemExit(main())
