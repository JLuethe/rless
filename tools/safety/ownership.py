#!/usr/bin/env python3
"""Seed and validate source-bound ownership reviews, never infer review from counts."""
import argparse
import hashlib
import json
from pathlib import Path

from common import ROOT, sha, write_json

CHECKLIST = ('owner_creation', 'aliases_escapes', 'callback_targets', 'invalidation',
             'mutation_commit', 'release_pair', 'error_cleanup', 'signals_handoff')
PACKETS = {x['id'] for x in json.loads((ROOT/'spec/safety/work-packages.json').read_text())['work_packages']}


def expected_entries(inventory, regions, root=ROOT):
    """File-level review covers raw sites; objects/indirect/native entries stay distinct."""
    entries = {}
    covered = set()
    for region in regions['regions']:
        key = region['id']
        paths = region['current_paths']
        covered.update(paths)
        entries[key] = dict(kind='ownership-cycle', paths=paths, packet=region['delivery_packet'])
    for path in inventory['files']:
        if not (root / path).is_file() or sha(root / path) != inventory['files'][path]['sha256']:
            raise ValueError('stale inventory source: ' + path)
        key = 'FILE:' + path
        owners = [r['id'] for r in regions['regions'] if path in r['current_paths']]
        entries[key] = dict(kind='source-file', paths=[path], regions=owners, packet=None,
                            scope='source AST including feature/test-guarded items; expanded production review also required')
    for item in inventory['mutable_static_definitions']:
        identity = json.dumps([item['file'], item['context'], item['symbol'],item.get('line',0),item.get('column',0)], separators=(',', ':'))
        key = 'STATIC:' + hashlib.sha256(identity.encode()).hexdigest()[:24]
        if key in entries:
            raise ValueError('ambiguous mutable definition identity')
        entries[key] = dict(kind='mutable-definition', paths=[item['file']], symbol=item['symbol'],
                            context=item['context'], packet=None)
    for symbol in inventory['candidate_external_symbols']:
        entries['NATIVE:' + symbol] = dict(kind='native-symbol-candidate', paths=[], symbol=symbol,
                                          packet=None, scope='Requires actual link/alias/target classification')
    for item in inventory['indirect_call_sites']:
        identity = json.dumps([item['file'],item['context'],item['line'],item['column'],item['symbol']], separators=(',', ':'))
        key = 'INDIRECT:' + hashlib.sha256(identity.encode()).hexdigest()[:24]
        if key in entries:
            raise ValueError('duplicate indirect-call identity')
        entries[key] = dict(kind='indirect-call', paths=[item['file']], symbol=item['symbol'],
                            context=item['context'], packet=None)
    return entries


def seed(inventory, regions, root=ROOT):
    entries = expected_entries(inventory, regions, root)
    return dict(schema_version=1, scope='Pending review entries; seeding is not review completion.',
                inventory_sha256=hashlib.sha256(json.dumps(inventory, sort_keys=True).encode()).hexdigest(),
                region_register_sha256=hashlib.sha256(json.dumps(regions, sort_keys=True).encode()).hexdigest(),
                source_files_sha256={p:sha(root / p) for p in inventory['files']},
                status='not-verified', entries=[dict(id=key, **item, status='pending', reviewer=None,
                    evidence_paths=[], evidence_sha256={}, finding_ids=[], checklist={name:None for name in CHECKLIST})
                    for key,item in entries.items()])


def validate(review, inventory, regions, findings, root=ROOT, complete=True):
    expected = seed(inventory, regions, root)
    for field in ('inventory_sha256', 'region_register_sha256', 'source_files_sha256'):
        if review[field] != expected[field]:
            raise ValueError('stale review ' + field)
    entries = {x['id']:x for x in review['entries']}
    if len(entries) != len(review['entries']) or set(entries) != {x['id'] for x in expected['entries']}:
        raise ValueError('missing, extra or duplicate review entry')
    live_findings = {x['id'] for x in findings['findings'] if x['status'] == 'open'}
    missing = []
    for base in expected['entries']:
        item = entries[base['id']]
        for field in ('kind', 'paths'):
            if item[field] != base[field]:
                raise ValueError('review target changed')
        if item['status'] not in ('pending', 'in-progress', 'reviewed-clear', 'reviewed-hazard-recorded'):
            raise ValueError('unknown review state')
        if item['status'] in ('pending', 'in-progress'):
            missing.append(item['id'])
            continue
        if item.get('packet') not in PACKETS or not item.get('reviewer') or not item.get('evidence_paths'):
            raise ValueError('review metadata/evidence missing')
        for field in CHECKLIST:
            if not isinstance(item['checklist'].get(field), str) or not item['checklist'][field].strip():
                raise ValueError('missing owner/alias/callback/release analysis')
        for path in item['evidence_paths']:
            resolved = (root / path).resolve()
            if not resolved.is_relative_to(root.resolve()) or not resolved.is_file():
                raise ValueError('missing or escaping review evidence')
            if item.get('evidence_sha256',{}).get(path)!=sha(resolved):
                raise ValueError('changed or unbound review evidence')
        if item['status'] == 'reviewed-hazard-recorded' and (not item['finding_ids'] or not set(item['finding_ids']) <= live_findings):
            raise ValueError('hazard has no live finding')
    if complete and missing:
        raise ValueError(f'{len(missing)} review entries remain incomplete')
    return dict(status='not-verified' if missing else 'reviewed', entries=len(entries), pending=len(missing),
                runtime_safety_verified=False)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=('seed','check'))
    parser.add_argument('--inventory', type=Path, required=True)
    parser.add_argument('--regions', type=Path, default=ROOT/'docs/safety/ownership-register.json')
    parser.add_argument('--findings', type=Path, default=ROOT/'docs/safety/findings.json')
    parser.add_argument('--review', type=Path, required=True)
    parser.add_argument('--allow-pending', action='store_true')
    args = parser.parse_args()
    inventory=json.loads(args.inventory.read_text()); regions=json.loads(args.regions.read_text())
    if args.operation=='seed':
        if args.review.exists():
            parser.error('refusing to overwrite existing review')
        write_json(args.review,seed(inventory,regions))
        print('Seeded pending review; no entry was marked reviewed.')
    else:
        result=validate(json.loads(args.review.read_text()),inventory,regions,
                        json.loads(args.findings.read_text()),complete=not args.allow_pending)
        print(json.dumps(result))


if __name__=='__main__':
    main()
