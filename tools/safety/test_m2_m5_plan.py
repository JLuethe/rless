"""Negative controls for weakened prerequisites and false completion claims."""
import copy
import json
from pathlib import Path
import unittest

from validate_m2_m5 import validate

SPEC = Path(__file__).resolve().parents[2] / 'spec/safety'


class RefinementControls(unittest.TestCase):
    def setUp(self):
        def read(name):
            return json.loads((SPEC / name).read_text())
        self.plan = read('m2-m5-plan.json')
        self.context = [read(n) for n in ('work-packages.json', 'requirements.json', 'acceptance.json', 'campaigns.json')]

    def test_original_counterpart_validates_without_runtime_approval(self):
        result = validate(self.plan, *self.context)
        self.assertEqual(result['packets'], 19)
        self.assertFalse(result['runtime_safety_verified'])
        self.assertFalse(result['production_integration_authorized'])

    def test_omitted_duplicated_or_weakened_packet_fails(self):
        for mutation in ('omit', 'duplicate', 'prerequisite', 'requirement', 'exit'):
            plan = copy.deepcopy(self.plan)
            if mutation == 'omit': plan['packets'].pop()
            elif mutation == 'duplicate': plan['packets'].append(plan['packets'][0])
            elif mutation == 'prerequisite': plan['packets'][0]['integration_prerequisites'] = []
            elif mutation == 'requirement': plan['packets'][0]['requirement_ids'] = []
            else: plan['packets'][0]['exit_gate'] = 'documentation is enough'
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                validate(plan, *self.context)

    def test_preparation_cannot_bypass_gates_or_pay_final_budgets(self):
        for field in ('production_integration_authorized', 'entry_gates', 'counts_as_production_migration',
                      'counts_toward_final_campaign_floors', 'forbid_unsafe', 'direct_native_imports'):
            plan = copy.deepcopy(self.plan)
            if field == 'production_integration_authorized': plan[field] = True
            elif field == 'entry_gates': plan[field] = []
            else: plan['preparation'][field] = not plan['preparation'][field]
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate(plan, *self.context)

    def test_false_completion_lowered_floor_and_release_fail(self):
        for mutation in ('completion', 'floor', 'release'):
            plan = copy.deepcopy(self.plan)
            if mutation == 'completion': plan['packets'][0]['status'] = 'passed'
            elif mutation == 'floor': plan['preserved_contract']['final_cpu_minutes_per_target_min'] = 0
            else: plan['preserved_contract']['release_authorized'] = True
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                validate(plan, *self.context)


if __name__ == '__main__':
    unittest.main(verbosity=2)
