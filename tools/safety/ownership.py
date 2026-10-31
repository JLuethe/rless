#!/usr/bin/env python3
"""Seed and validate source-bound ownership reviews, never infer review from counts."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

from common import ROOT, sha, write_json
from inventory_format import (EXPANDED_CONFIGURATIONS, all_statics, boundary_comparison, expanded_review_sites,
                              native_candidates, parse_records, source_inputs, derive_current_inventory, REGION_REGISTER)

CHECKLIST = ('owner_creation', 'aliases_escapes', 'callback_targets', 'invalidation',
             'mutation_commit', 'release_pair', 'error_cleanup', 'signals_handoff')
PACKETS = {x['id'] for x in json.loads((ROOT/'spec/safety/work-packages.json').read_text())['work_packages']}


def checked_artifact(artifact, root):
    path = root / artifact['path']
    if not path.is_file() or sha(path) != artifact['sha256']:
        raise ValueError('missing or stale inventory artifact: ' + str(path))
    return path


def validate_inventory_v2(inventory, root):
    """Derive coverage from retained syntax, not editable summary counts."""
    if set(inventory['files']) != set(source_inputs(root)):
        raise ValueError('missing or extra maintained source input')
    # A retained JSON/TSV artifact and an index derived from that SAME artifact
    # can both omit a site consistently. Rebuild the maintained parser and derive
    # the expected source facts without passing it any submitted artifact.
    current = derive_current_inventory(root)
    if inventory.get('derivation_inputs_sha256') != current['derivation_inputs_sha256']:
        raise ValueError('stale inventory derivation inputs')
    records = json.loads(checked_artifact(inventory['raw_inventory'], root).read_text())
    if records != current['source_records']:
        raise ValueError('raw inventory is not the complete current-source AST')
    if not {item['file'] for item in records} <= set(inventory['files']):
        raise ValueError('syntax record outside inventoried source inputs')
    if inventory['static_definitions'] != all_statics(records):
        raise ValueError('missing, extra or changed all-static definition')
    for field, kind in [('mutable_static_definitions', 'mutable_static_definition'),
                        ('indirect_call_sites', 'indirect_call')]:
        if inventory[field] != [item for item in records if item['kind'] == kind]:
            raise ValueError('changed source syntax coverage: ' + field)
    if inventory['candidate_external_symbols'] != native_candidates(records):
        raise ValueError('changed native-symbol coverage')
    configurations = inventory['expanded_configurations']
    if set(configurations) != set(EXPANDED_CONFIGURATIONS):
        raise ValueError('missing or extra expanded build configuration')
    for name, configuration in configurations.items():
        for field in ('target', 'features'):
            if configuration[field] != EXPANDED_CONFIGURATIONS[name][field]:
                raise ValueError('changed expanded configuration: ' + name)
        source = checked_artifact(configuration['source'], root)
        syntax = checked_artifact(configuration['syntax'], root)
        expanded = parse_records(syntax.read_text())
        expected = current['expanded'][name]
        if source.read_text() != expected['source']:
            raise ValueError('expanded source does not match current compiler/configuration: ' + name)
        if [{k:v for k,v in item.items() if k != 'file'} for item in expanded] != expected['records']:
            raise ValueError('expanded AST is not the complete current compiler output: ' + name)
        if any((root / item['file']).resolve() != source.resolve() for item in expanded):
            raise ValueError('expanded syntax belongs to a different artifact')
        if configuration['review_sites'] != expanded_review_sites(expanded):
            raise ValueError('missing, extra or changed expanded review site: ' + name)
        if name.endswith('-safety-injection'):
            baseline = name.removesuffix('-safety-injection')
            if configuration.get('baseline_configuration') != baseline or configuration.get('boundary_comparison') != boundary_comparison(
                    configurations[baseline]['review_sites'], configuration['review_sites']):
                raise ValueError('changed feature boundary comparison: ' + name)


def validate_region_register(inventory, regions, root=ROOT):
    """Review input must be the current canonical register, not an alternate set."""
    path = Path(root) / REGION_REGISTER
    if (inventory.get('manual_register') != REGION_REGISTER or not path.is_file() or path.is_symlink()
            or not path.resolve().is_relative_to(Path(root).resolve())):
        raise ValueError('inventory must bind the canonical current region register')
    if inventory.get('manual_register_sha256') != sha(path):
        raise ValueError('stale canonical region register binding')
    canonical = json.loads(path.read_text())
    if regions != canonical:
        raise ValueError('alternate region register differs from the canonical current register')
    rows = canonical.get('regions')
    if not isinstance(rows, list) or not rows:
        raise ValueError('canonical region register is empty')
    seen = set()
    for region in rows:
        identity, paths = region.get('id'), region.get('current_paths')
        if not isinstance(identity, str) or not identity or identity in seen:
            raise ValueError('missing or duplicate canonical region identity')
        seen.add(identity)
        if (not isinstance(paths, list) or not paths or len(paths) != len(set(paths))
                or any(path not in inventory['files'] for path in paths)):
            raise ValueError('canonical region refers to missing or duplicate maintained inputs')
        if region.get('delivery_packet') not in PACKETS:
            raise ValueError('canonical region has no current delivery packet')
    return canonical


def expected_entries(inventory, regions, root=ROOT):
    """File-level review covers raw sites; objects/indirect/native entries stay distinct."""
    validate_region_register(inventory, regions, root)
    entries = {}
    version = inventory.get('schema_version', 1)
    if type(version) is not int or version not in (1, 2):
        raise ValueError('unsupported inventory schema')
    if version == 2:
        validate_inventory_v2(inventory, root)
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
    statics = inventory['static_definitions'] if version == 2 else inventory['mutable_static_definitions']
    for item in statics:
        identity = json.dumps([item['file'], item['context'], item['symbol'],item.get('line',0),item.get('column',0)], separators=(',', ':'))
        key = 'STATIC:' + hashlib.sha256(identity.encode()).hexdigest()[:24]
        if key in entries:
            raise ValueError('ambiguous static definition identity')
        mutable = version == 1 or item['binding_mutability'] == 'mutable'
        entries[key] = dict(kind='mutable-definition' if mutable else 'shared-static-definition',
                            paths=[item['file']], symbol=item['symbol'],
                            context=item['context'], packet=None)
        if version == 2:
            entries[key].update(declared_type=item['declared_type'],
                                binding_mutability=item['binding_mutability'],
                                scope='Every static is reviewed; shared binding does not imply immutable contents.')
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
    if version == 2:
        for name, configuration in inventory['expanded_configurations'].items():
            entries['EXPANDED:' + name] = dict(kind='expanded-source', paths=[], packet=None,
                configuration=name, source_artifact={'sha256': configuration['source']['sha256']},
                syntax_artifact={'sha256': configuration['syntax']['sha256']},
                site_ids=[site['id'] for site in configuration['review_sites']],
                review_sites_sha256=hashlib.sha256(json.dumps(configuration['review_sites'], sort_keys=True).encode()).hexdigest(),
                scope='Review compiler-generated code and configuration-specific owner/native/callback reachability.')
    return entries


INVENTORY_BINDING = 'source-inventory-content-v1'


def inventory_digest(inventory):
    """Bind complete inventory content, independently of run timestamp/storage.

    Artifact hashes, compiler configurations, build derivation hashes and every
    source/static/native/indirect/expanded site remain bound. validate_inventory_v2
    still checks actual retained artifacts against a fresh source derivation.
    """
    value = copy.deepcopy(inventory)
    value.pop('recorded_at_utc', None)
    if isinstance(value.get('raw_inventory'), dict):
        value['raw_inventory'].pop('path', None)
    for config in value.get('expanded_configurations', {}).values():
        for kind in ('source', 'syntax'):
            if isinstance(config.get(kind), dict):
                config[kind].pop('path', None)
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def seed(inventory, regions, root=ROOT):
    entries = expected_entries(inventory, regions, root)
    return dict(schema_version=2, inventory_binding=INVENTORY_BINDING, scope='Pending review entries; seeding is not review completion.',
                inventory_sha256=inventory_digest(inventory),
                region_register_sha256=hashlib.sha256(json.dumps(regions, sort_keys=True).encode()).hexdigest(),
                source_files_sha256={p:sha(root / p) for p in inventory['files']},
                status='not-verified', entries=[dict(id=key, **item, status='pending', reviewer=None,
                    evidence_paths=[], evidence_sha256={}, finding_ids=[], checklist={name:None for name in CHECKLIST})
                    for key,item in entries.items()])


def validate_expanded_review(item, entries, configuration, root):
    """A configuration approval must account for every retained expanded boundary."""
    mapping = item.get('expanded_review_map')
    if not isinstance(mapping, dict):
        raise ValueError('missing expanded per-site review map')
    path = (root / mapping['path']).resolve()
    if not path.is_relative_to(root.resolve()) or not path.is_file() or sha(path) != mapping['sha256']:
        raise ValueError('missing, escaping or stale expanded review map')
    if mapping['path'] not in item['evidence_paths']:
        raise ValueError('expanded review map not listed as review evidence')
    review = json.loads(path.read_text())
    if review.get('configuration') != item['configuration'] or review.get('source_sha256') != configuration['source']['sha256']:
        raise ValueError('expanded review map configuration/source mismatch')
    sites = {site['id']: site for site in configuration['review_sites']}
    seen = set()
    for group in review.get('site_reviews', []):
        ids = group.get('site_ids', [])
        if not ids or len(ids) != len(set(ids)) or not set(ids) <= sites.keys() or seen & set(ids):
            raise ValueError('missing, duplicate or unknown expanded review site')
        seen.update(ids)
        if not isinstance(group.get('analysis'), str) or not group['analysis'].strip():
            raise ValueError('empty expanded-site analysis')
        if group.get('disposition') == 'source-covered':
            references = group.get('source_review_ids', [])
            if not references or any(key not in entries or entries[key]['kind'] not in (
                    'mutable-definition', 'shared-static-definition', 'native-symbol-candidate', 'indirect-call')
                    or entries[key]['status'] not in ('reviewed-clear', 'reviewed-hazard-recorded') for key in references):
                raise ValueError('expanded source coverage cites unreviewed or imprecise source entries')
            for site_id in ids:
                site = sites[site_id]
                symbol = site['symbol'].replace(' ', '').lstrip(':')
                if not any(entries[key].get('symbol', '').replace(' ', '').lstrip(':') == symbol
                        and ('declared_type' not in site or entries[key].get('declared_type', '').replace(' ', '')
                             == site['declared_type'].replace(' ', '')) for key in references):
                    raise ValueError('expanded source coverage has no matching source symbol')
            inherited_findings = {finding for key in references
                                  if entries[key]['status'] == 'reviewed-hazard-recorded'
                                  for finding in entries[key]['finding_ids']}
            if inherited_findings and (item['status'] != 'reviewed-hazard-recorded'
                    or not inherited_findings <= set(item['finding_ids'])):
                raise ValueError('expanded review hides a referenced source hazard')
        elif group.get('disposition') == 'configuration-reviewed':
            # Macro-only boundaries and signatures that cannot map precisely to
            # source rows need explicit reviewer analysis rather than an assumed
            # match to a similar source symbol.
            section, evidence_path = group.get('evidence_section'), group.get('evidence_path')
            if not isinstance(section, str) or not section.strip() or evidence_path not in item['evidence_paths']:
                raise ValueError('configuration-specific review has no evidence section')
            if section not in (root / evidence_path).read_text():
                raise ValueError('configuration-specific evidence section is missing')
        else:
            raise ValueError('unknown expanded-site review disposition')
    if seen != sites.keys():
        raise ValueError('unreviewed expanded sites remain')


def validate(review, inventory, regions, findings, root=ROOT, complete=True):
    if complete and inventory.get('schema_version', 1) != 2:
        raise ValueError('complete ownership review requires source-derived schema-2 inventory')
    expected = seed(inventory, regions, root)
    if review.get('schema_version') != 2 or review.get('inventory_binding') != INVENTORY_BINDING:
        raise ValueError('ownership review requires explicit migration to source-inventory-content-v1')
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
        for field in ('kind', 'paths', 'symbol', 'context', 'configuration', 'declared_type',
                      'binding_mutability', 'source_artifact', 'syntax_artifact', 'site_ids', 'review_sites_sha256'):
            if field in base and item.get(field) != base[field]:
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
        if item['kind'] == 'expanded-source':
            validate_expanded_review(item, entries, inventory['expanded_configurations'][item['configuration']], root)
    if complete and missing:
        raise ValueError(f'{len(missing)} review entries remain incomplete')
    return dict(status='not-verified' if missing or inventory.get('schema_version', 1) != 2 else 'reviewed', entries=len(entries), pending=len(missing),
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
