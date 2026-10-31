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
