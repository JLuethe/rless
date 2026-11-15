"""Lossless syntax inventory identities; no inferred ownership approvals."""
import hashlib
import json
import os
from pathlib import Path


# Expand the library and both executable roots in each configuration actually
# used by production-path fault testing. These are separate compilation inputs.
EXPANDED_CONFIGURATIONS = {
    'lib': {'target': ['--lib'], 'features': []},
    'rless': {'target': ['--bin', 'rless'], 'features': []},
    'lessecho': {'target': ['--bin', 'lessecho'], 'features': []},
    'lib-safety-injection': {'target': ['--lib'], 'features': ['safety-injection']},
    'rless-safety-injection': {'target': ['--bin', 'rless'], 'features': ['safety-injection']},
    'lessecho-safety-injection': {'target': ['--bin', 'lessecho'], 'features': ['safety-injection']},
}


def source_inputs(root):
    """Discover maintained Rust inputs without reading generated Cargo output."""
    paths = []
    excluded = {'target', '.git', '.runs', '__pycache__', 'artifacts', 'corpus', 'coverage'}
    for top in ('src', 'vendor'):
        for parent, directories, files in os.walk(root / top, followlinks=False):
            kept = [d for d in directories if d not in excluded]
            if any((Path(parent) / d).is_symlink() for d in kept):
                raise ValueError('symlinked Rust inventory directory: ' + str(parent))
            directories[:] = sorted(kept)
            for name in sorted(files):
                path = Path(parent) / name
                if path.suffix == '.rs':
                    if path.is_symlink():
                        raise ValueError('symlinked Rust inventory input: ' + str(path))
                    paths.append(str(path.relative_to(root)))
    if (root / 'build.rs').is_file():
        paths.append('build.rs')
    return sorted(paths)


def parse_records(text):
    records = []
    for line in text.splitlines():
        kind, file, row, col, context, symbol = line.split('\t', 5)
        record = dict(kind=kind, file=file, line=int(row), column=int(col),
                      context=context, symbol=symbol)
        if record['line'] < 1 or record['column'] < 1:
            raise ValueError('invalid AST source location')
        records.append(record)
    return records


def location(item):
    return (item['file'], item['context'], item['line'], item['column'])


def all_statics(records):
    """Retain shared bindings even when their type's mutability is opaque."""
    types = {}
    mutable = set()
    definitions = []
    seen = set()
    for item in records:
        key = location(item)
        if item['kind'] == 'static_declared_type':
            if key in types:
                raise ValueError('duplicate static type location')
            types[key] = item['symbol']
        elif item['kind'] == 'mutable_static_definition':
            mutable.add((*key, item['symbol']))
        elif item['kind'] == 'static_definition':
            identity = (*key, item['symbol'])
            if identity in seen:
                raise ValueError('duplicate static definition')
            seen.add(identity)
            definitions.append(item)
    if not mutable <= seen:
        raise ValueError('mutable static omitted from all-static inventory')
    if set(types) != {location(item) for item in definitions}:
        raise ValueError('missing or orphan static type')
    return [{**item, 'declared_type': types[location(item)],
             'binding_mutability': 'mutable' if (*location(item), item['symbol']) in mutable else 'shared',
             'interior_mutability': 'requires-review'} for item in definitions]


def native_candidates(records):
    definitions = {r['symbol'] for r in records if r['kind'] == 'abi_function_definition'}
    externs = {r['symbol'] for r in records if r['kind'] == 'extern_function_declaration'}
    qualified = {r['symbol'].replace(' ', '').lstrip(':') for r in records
                 if r['kind'] == 'direct_call' and
                 r['symbol'].replace(' ', '').lstrip(':').startswith('libc::')}
    return sorted((externs - definitions) | qualified)


def expanded_review_sites(records):
    """Actual expanded boundary rows, including native aliases and callbacks.

    This intentionally over-approximates: a function-pointer type or foreign
    declaration is a classification obligation, not proof of an executed call.
    """
    kinds = {'extern_function_declaration', 'native_link_name', 'extern_static_declaration',
             'indirect_call', 'function_pointer_type', 'abi_function_definition'}
    sites = all_statics(records)
    for item in records:
        if item['kind'] in kinds or (item['kind'] == 'direct_call' and
                item['symbol'].replace(' ', '').lstrip(':').startswith('libc::')):
            sites.append(item)
    # Compiler output paths are carried by the configuration's artifact record.
    # Excluding that path from semantic rows keeps the identities independent of
    # the caller's output directory; line/column/context still bind the location.
    result = [{key: value for key, value in item.items() if key != 'file'} for item in sites]
    identities = [json.dumps(item, sort_keys=True) for item in result]
    if len(set(identities)) != len(identities):
        raise ValueError('duplicate expanded boundary identity')
    return [dict(id=hashlib.sha256(identity.encode()).hexdigest()[:24], **item)
            for identity, item in zip(identities, result)]


def boundary_comparison(base, candidate):
    """Map boundary identities across feature expansion; never imply code equivalence."""
    def groups(sites):
        result = {}
        for site in sites:
            identity = json.dumps({key: value for key, value in site.items()
                                   if key not in ('id', 'line', 'column')}, sort_keys=True)
            result.setdefault(identity, []).append(site['id'])
        return result
    before, after = groups(base), groups(candidate)
    return dict(scope='Boundary signatures only; matching signatures do not prove equivalent bodies or safety.',
        matched=[dict(base_site_ids=before[key], candidate_site_ids=after[key])
                 for key in sorted(before.keys() & after.keys())],
        removed_site_ids=[value for key in sorted(before.keys() - after.keys()) for value in before[key]],
        added_site_ids=[value for key in sorted(after.keys() - before.keys()) for value in after[key]])


# These are policy inputs, never chosen by a submitted inventory/review payload.
REGION_REGISTER = 'docs/safety/ownership-register.json'
_DERIVED_CACHE = {}


def derivation_inputs(root):
    """Hash every maintained build input and the parser source, not caller ASTs."""
    root = Path(root).resolve()
    files = {}
    excluded = {'target', '.git', '.runs', '__pycache__', 'artifacts', 'corpus', 'coverage'}
    for name in ('Cargo.toml', 'Cargo.lock', 'build.rs', 'rust-toolchain.toml', 'rust-toolchain'):
        path = root / name
        if path.is_file():
            if path.is_symlink():
                raise ValueError('symlinked inventory build input: ' + name)
            files[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    for top in ('src', 'vendor', 'tools/safety/inventory', '.cargo'):
        for parent, directories, names in os.walk(root / top, followlinks=False):
            directories[:] = sorted(d for d in directories if d not in excluded)
            if any((Path(parent) / d).is_symlink() for d in directories):
                raise ValueError('symlinked inventory build directory')
            for name in sorted(names):
                path = Path(parent) / name
                if path.is_symlink() or not path.is_file():
                    raise ValueError('nonregular inventory build input: ' + str(path))
                files[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()
    required = {'Cargo.toml', 'Cargo.lock', 'tools/safety/inventory/Cargo.toml',
                'tools/safety/inventory/Cargo.lock', 'tools/safety/inventory/src/main.rs'}
    if not required <= files.keys():
        raise ValueError('missing maintained project/parser build inputs')
    return files


def _trusted_inventory_tools(root):
    """The invoking operator supplies tool locations through environment, not JSON.

    A checkout next to rless-env needs no overrides. Isolated worktrees use
    RLESS_INVENTORY_TOOL_HOME and RLESS_INVENTORY_CARGO_HOME. No retained helper
    executable, compiler path or execution-success assertion is trusted.
    """
    import subprocess
    from common import PINS
    tool_home = Path(os.environ.get('RLESS_INVENTORY_TOOL_HOME', root.parent / 'rless-env/safety')).resolve()
    cache = Path(os.environ.get('RLESS_INVENTORY_CARGO_HOME', tool_home.parent / 'cargo-home')).resolve()
    toolchain = tool_home / 'rustup/toolchains' / (PINS['nightly'] + '-' + PINS['target'])
    rustc, cargo = toolchain / 'bin/rustc', toolchain / 'bin/cargo'
    if not rustc.is_file() or not cargo.is_file() or not (cache / 'registry').is_dir():
        raise ValueError('trusted inventory toolchain/cache unavailable; configure RLESS_INVENTORY_TOOL_HOME and RLESS_INVENTORY_CARGO_HOME')
    env = dict(PATH=str(toolchain / 'bin') + ':/usr/bin:/bin', LANG='C.UTF-8', LC_ALL='C.UTF-8',
               RUSTC=str(rustc), RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='',
               CARGO_ENCODED_RUSTFLAGS='', CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2')
    version = subprocess.run([str(rustc), '-vV'], env=env, text=True, capture_output=True, timeout=30, check=True).stdout
    if ('commit-hash: ' + PINS['rustc_commit']) not in version or ('host: ' + PINS['target']) not in version:
        raise ValueError('trusted inventory compiler differs from pinned nightly/target')
    files = [rustc, cargo]
    files.extend(sorted((toolchain / 'lib').glob('*.so*')))
    files.extend(sorted((toolchain / 'lib/rustlib' / PINS['target'] / 'lib').glob('*.rlib')))
    tools = {str(path): hashlib.sha256(path.read_bytes()).hexdigest() for path in files}
    return cargo, env, cache, tools


def derive_current_inventory(root):
    """Independently rebuild/execute the current pinned parser and expansions.

    This takes NO submitted AST/source artifact arguments. The process-local
    cache stores only these independently derived facts and is keyed by complete
    current input/tool content hashes. Returned values are deep copies, so a
    caller cannot poison subsequent validations. No on-disk claimed receipt can
    populate this cache. Missing build tools or compile/parse failures fail closed.
    """
    import copy
    import subprocess
    import tempfile
    root = Path(root).resolve()
    inputs = derivation_inputs(root)
    sources = source_inputs(root)
    cargo, environment, cache, tools = _trusted_inventory_tools(root)
    identity = dict(root=str(root), inputs=inputs, tools=tools, configurations=EXPANDED_CONFIGURATIONS)
    key = hashlib.sha256(json.dumps(identity, sort_keys=True).encode()).hexdigest()
    if key in _DERIVED_CACHE:
        return copy.deepcopy(_DERIVED_CACHE[key])
    temporary_root = Path(os.environ.get('RLESS_INVENTORY_TMPDIR', root / 'compat/.runs')).resolve()
    temporary_root.mkdir(parents=True, exist_ok=True)
    try:
        with tempfile.TemporaryDirectory(prefix='inventory-rederive-', dir=temporary_root) as directory:
            work = Path(directory)
            home, cargo_home = work / 'home', work / 'cargo'
            home.mkdir(); cargo_home.mkdir()
            (cargo_home / 'registry').symlink_to(cache / 'registry', target_is_directory=True)
            env = dict(environment, HOME=str(home), CARGO_HOME=str(cargo_home),
                       CARGO_TARGET_DIR=str(work / 'target'))
            def execute(command):
                result = subprocess.run([str(value) for value in command], cwd=root, env=env,
                                        text=True, capture_output=True, timeout=600)
                if result.returncode:
                    raise ValueError('fresh inventory derivation failed: ' + ' '.join(map(str, command)) + '\n' + result.stderr[-4000:])
                return result.stdout
            execute([cargo, 'build', '--locked', '--offline', '--manifest-path',
                     root / 'tools/safety/inventory/Cargo.toml'])
            helper = work / 'target/debug/rless-safety-inventory'
            raw = parse_records(execute([helper, *sources]))
            expanded = {}
            for name, configuration in EXPANDED_CONFIGURATIONS.items():
                feature = ['--features', ','.join(configuration['features'])] if configuration['features'] else []
                text = execute([cargo, 'rustc', '--locked', '--offline', *configuration['target'], *feature,
                                '--', '-Zunpretty=expanded'])
                path = work / (name + '.rs')
                path.write_text(text)
                records = parse_records(execute([helper, path]))
                # The parser's input path is incidental; all other syntax fields
                # and expanded bytes are exact and must match the submitted data.
                expanded[name] = dict(source=text, records=[{k:v for k,v in item.items() if k != 'file'} for item in records])
            if derivation_inputs(root) != inputs or source_inputs(root) != sources:
                raise ValueError('maintained inputs changed during fresh inventory derivation')
            if any(hashlib.sha256(Path(path).read_bytes()).hexdigest() != digest for path,digest in tools.items()):
                raise ValueError('compiler/toolchain changed during inventory derivation')
            result = dict(source_records=raw, expanded=expanded,
                          derivation_inputs_sha256=hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest(),
                          parser_binary_sha256=hashlib.sha256(helper.read_bytes()).hexdigest())
    except (OSError, subprocess.SubprocessError) as error:
        raise ValueError('fresh inventory derivation unavailable: ' + str(error)) from error
    _DERIVED_CACHE.clear()  # Keep at most one independently derived candidate in memory.
    _DERIVED_CACHE[key] = result
    return copy.deepcopy(result)
