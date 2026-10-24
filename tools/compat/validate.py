#!/usr/bin/env python3
"""Validate the specification graph and report measured acceptance coverage."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import jsonschema

ROOT = Path(__file__).resolve().parents[2]


def read(name):
    return json.loads((ROOT/name).read_text())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT/'spec/coverage.json')
    args = parser.parse_args()
    caps = read('spec/capabilities.json')
    reqs = read('spec/requirements.json')
    cap_schema = read('compat/schemas/capability.schema.json')
    req_schema = read('compat/schemas/requirement.schema.json')
    case_schema = read('compat/schemas/case.schema.json')
    cap_validator = jsonschema.Draft202012Validator(cap_schema)
    req_validator = jsonschema.Draft202012Validator(req_schema)
    case_validator = jsonschema.Draft202012Validator(case_schema)
    for capability in caps:
        cap_validator.validate(capability)
    for requirement in reqs:
        req_validator.validate(requirement)
    capmap = {x['id']: x for x in caps}
    assert len(capmap) == len(caps), 'duplicate capability ID'
    assert len({x['id'] for x in reqs}) == len(reqs), 'duplicate requirement ID'
    parent_ids = set(re.findall(r'^\| ([A-Z]+-\d+) \|', (ROOT/'docs/rless-compatibility-spec.md').read_text(), re.M))
    mapped_parents = {x['parent'] for x in reqs} | {p for x in reqs for p in x.get('related_parents', [])}
    assert parent_ids <= mapped_parents, 'unmapped parent requirements: '+str(parent_ids-mapped_parents)
    allcases, expected, replayed = {}, {}, {}
    for casefile in sorted((ROOT/'compat/cases').glob('*.json')):
        family = casefile.stem
        cases = read(f'compat/cases/{family}.json')
        for case in cases:
            case_validator.validate(case)
            assert case['id'] not in allcases, 'duplicate case'
            assert set(case['capability_ids']) <= capmap.keys(), 'unknown capability'
            allcases[case['id']] = case
        records = read(f'compat/expected-{family}.json')
        assert set(records) == {x['id'] for x in cases}, f'orphan or missing {family} expectations'
        expected.update(records)
        replaypath = ROOT/f'compat/replay-{family}.json'
        if replaypath.exists():
            replay = json.loads(replaypath.read_text())
            assert not replay['failed'] and not replay['errors'], 'failing replay '+family
            assert set(replay['cases']) == {x['id'] for x in cases}, 'stale replay IDs '+family
            replayed.update(replay['cases'])
    for cap in caps:
        assert (ROOT/cap['contract']).is_file(), 'missing contract'
        assert set(cap['case_ids']) <= allcases.keys(), 'unknown case link'
        for cid in cap['case_ids']:
            assert cap['id'] in allcases[cid]['capability_ids'], 'one-sided case link'
        if cap['disposition'] in ('production', 'rejected-legacy'):
            assert cap['case_ids'], 'uncased required interface '+cap['id']
    for req in reqs:
        assert req['parent'] in parent_ids, 'unknown parent'
        assert set(req['capability_ids']) <= capmap.keys(), 'unknown requirement capability'
        wanted = {cid for cap in req['capability_ids'] for cid in capmap[cap]['case_ids']}
        assert set(req['case_ids']) == wanted, 'stale requirement mapping'
    profile = read('compat/reference.json')
    for name, digest in profile['source_data_sha256'].items():
        assert hashlib.sha256((ROOT/'spec/reference'/name).read_bytes()).hexdigest() == digest
    for name, digest in profile.get('normative_annex_sha256', {}).items():
        assert hashlib.sha256((ROOT/'spec/reference'/name).read_bytes()).hexdigest() == digest
    corpus = read('compat/corpus.json')
    for name, digest in corpus['files'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == digest, 'changed acceptance corpus: '+name
    report = dict(schema_version=1, capabilities=len(caps), detailed_requirements=len(reqs),
                  parent_requirements=len(parent_ids), dispositions=dict(Counter(x['disposition'] for x in caps)),
                  kinds=dict(Counter(x['kind'] for x in caps)), acceptance_cases=len(allcases),
                  reference_expectations=len(expected), replayed=len(replayed),
                  replay_passed=sum(x['passed'] for x in replayed.values()),
                  required_interfaces_with_cases=sum(bool(x['case_ids']) for x in caps if x['disposition'] in ('production','rejected-legacy')),
                  required_interfaces=sum(x['disposition'] in ('production','rejected-legacy') for x in caps),
                  interpretation='Interface-to-case coverage, not a proof of all behavioral paths or rless conformance.')
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
