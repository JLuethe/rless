#!/usr/bin/env python3
"""Exact acceptance variant/profile matrix; passing diagnostics are not full coverage."""
import argparse
import hashlib
import json
from pathlib import Path

from common import ROOT, fingerprint, sha, validate_report, write_json


def row_id(scenario, variant, profile):
    value = json.dumps([scenario, variant, profile], ensure_ascii=True, separators=(',', ':'))
    return scenario + ':' + profile + ':' + hashlib.sha256(value.encode()).hexdigest()[:16]


def matrix(catalogue):
    rows = {}
    scenarios = set()
    for scenario in catalogue['scenarios']:
        if scenario['id'] in scenarios:
            raise ValueError('duplicate scenario')
        scenarios.add(scenario['id'])
        variants, profiles = scenario['variants'], scenario['required_profiles']
        if not variants or len(set(variants)) != len(variants):
            raise ValueError('empty or duplicate variant selection')
        if not profiles or len(set(profiles)) != len(profiles):
            raise ValueError('empty or duplicate profile selection')
        for variant in variants:
            for profile in profiles:
                if profile not in catalogue['profiles']:
                    raise ValueError('unknown profile')
                key = row_id(scenario['id'], variant, profile)
                rows[key] = dict(id=key, scenario_id=scenario['id'], variant=variant,
                                 profile=profile, requirement_ids=scenario['requirement_ids'],
                                 status='not-run', implementation_path=None, evidence=None)
    if not rows:
        raise ValueError('empty catalogue')
    return rows


def bind(rows, bindings, root=ROOT):
    """Validate scoped evidence; partial site coverage never passes an entire variant."""
    seen = set()
    result = {key: dict(value) for key, value in rows.items()}
    for item in bindings:
        key = item['row_id']
        if key in seen or key not in rows:
            raise ValueError('duplicate or unknown acceptance row')
        seen.add(key)
        row = rows[key]
        for field in ('scenario_id', 'variant', 'profile'):
            if item[field] != row[field]:
                raise ValueError('row identity/variant/profile mismatch')
        path = (root / item['implementation_path']).resolve()
        if not path.is_relative_to(root.resolve()) or not path.is_file():
            raise ValueError('missing or escaping implementation path')
        if item['coverage_scope'] not in ('partial', 'complete') or not item.get('scope_notes'):
            raise ValueError('missing implementation scope')
        report = validate_report(item['report'], root)
        relative = str(path.relative_to(root.resolve()))
        if report['input_fingerprints']['files'].get(relative) != sha(path):
            raise ValueError('implementation is not bound to the executed source fingerprint')
        matches = [x for x in report['records'] if x['name'] == item['record_name']]
        if len(matches) != 1:
            raise ValueError('missing or duplicate evidence selector')
        executed = matches[0]
        for field in ('scenario_id', 'variant', 'profile'):
            if executed[field] != row[field]:
                raise ValueError('executed oracle does not match row')
        if item['coverage_scope']=='complete' and executed.get('coverage_scope')!='complete':
            raise ValueError('partial or undeclared executed coverage cannot be promoted by a binding')
        result[key].update(status='passed' if item['coverage_scope'] == 'complete' else 'not-verified',
                           implementation_path=item['implementation_path'], evidence=item)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--catalogue', type=Path, default=ROOT / 'spec/safety/acceptance.json')
    parser.add_argument('--bindings', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--require-complete', action='store_true')
    args = parser.parse_args()
    rows = matrix(json.loads(args.catalogue.read_text()))
    if args.bindings:
        rows = bind(rows, json.loads(args.bindings.read_text())['bindings'])
    passed = all(row['status'] == 'passed' for row in rows.values())
    write_json(args.output, dict(schema_version=1, scope='Exact final acceptance matrix; diagnostic setup and partial hooks do not close rows.',
                                catalogue_sha256=sha(args.catalogue), input_fingerprint=fingerprint()['sha256'],
                                status='passed' if passed else 'not-verified', runtime_safety_verified=False,
                                row_count=len(rows), rows=list(rows.values())))
    print(f'{sum(x["status"] == "passed" for x in rows.values())}/{len(rows)} rows passed')
    return 1 if args.require_complete and not passed else 0


if __name__ == '__main__':
    raise SystemExit(main())
