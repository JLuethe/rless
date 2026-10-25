#!/usr/bin/env python3
"""Validate refinement traceability; no runtime gate or approval is inferred."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def validate(plan, original, requirements, acceptance, campaigns):
    expected = {p['id']: p for p in original['work_packages'] if p['id'].startswith(('M2.', 'M3.', 'M4.', 'M5.'))}
    actual = {p['id']: p for p in plan['packets']}
    if len(actual) != len(plan['packets']) or set(actual) != set(expected):
        raise ValueError('missing, duplicate or unknown M2–M5 packet')
    if plan['production_integration_authorized'] is not False:
        raise ValueError('refinement cannot authorize gated production integration')
    if plan['entry_gates'] != ['C0-GATE', 'C1-PROTO', 'M1.4']:
        raise ValueError('entry prerequisites changed')
    prep = plan['preparation']
    if (prep['forbid_unsafe'] is not True or prep['direct_native_imports'] is not False
            or prep['counts_as_production_migration'] is not False
            or prep['counts_toward_final_campaign_floors'] is not False):
        raise ValueError('preparation scope bypasses migration or safety gates')
    for key, packet in actual.items():
        base = expected[key]
        if packet['integration_prerequisites'] != base['depends_on']:
            raise ValueError('packet prerequisite weakened: ' + key)
        if packet['requirement_ids'] != base['requirement_ids'] or packet['exit_gate'] != base['exit_gate']:
            raise ValueError('requirement or exit gate changed: ' + key)
        if not packet['implementation_units'] or any(not x.strip() for x in packet['implementation_units']):
            raise ValueError('missing implementation detail: ' + key)
        if packet['status'] != 'planned' or packet['evidence_status'] != 'not-verified' or packet['completion_record'] is not None:
            raise ValueError('specification cannot claim implementation completion: ' + key)
    final = next(p for p in campaigns['lanes'] if p['id'] == 'FINAL')
    rows = sum(len(p['variants']) * len(p['required_profiles']) for p in acceptance['scenarios'])
    preserved = plan['preserved_contract']
    values = dict(requirements=len(requirements['requirements']), acceptance_families=len(acceptance['scenarios']),
        named_variants=sum(len(p['variants']) for p in acceptance['scenarios']), profile_rows=rows,
        production_fuzz_targets=len(campaigns['in_process_targets']),
        final_inputs_per_target_min=final['inputs_per_fuzz_target_min'],
        final_cpu_minutes_per_target_min=final['cpu_minutes_per_fuzz_target_min'],
        final_native_scenarios_min=final['bounded_native_scenarios_min'],
        final_repetitions_per_race_recovery_min=final['repeats_per_required_race_recovery_min'],
        merge_authorized=False, release_authorized=False)
    if preserved != values:
        raise ValueError('contract, campaign floor or authorization changed')
    return dict(specification_valid=True, packets=len(actual), runtime_safety_verified=False,
                production_integration_authorized=False)


def main():
    def read(name):
        return json.loads((ROOT / 'spec/safety' / name).read_text())
    result = validate(read('m2-m5-plan.json'), read('work-packages.json'), read('requirements.json'),
                      read('acceptance.json'), read('campaigns.json'))
    print(json.dumps(result))


if __name__ == '__main__':
    main()
