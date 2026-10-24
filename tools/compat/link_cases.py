#!/usr/bin/env python3
"""Rebuild both directions of the case graph, including hand-authored cases."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[2]
caps = json.loads((root/'spec/capabilities.json').read_text())
reqs = json.loads((root/'spec/requirements.json').read_text())
by_id = {cap['id']: cap for cap in caps}
for cap in caps:
    cap['case_ids'] = []
for path in sorted((root/'compat/cases').glob('*.json')):
    for case in json.loads(path.read_text()):
        for cid in case['capability_ids']:
            by_id[cid]['case_ids'].append(case['id'])
for cap in caps:
    cap['case_ids'] = sorted(set(cap['case_ids']))
for req in reqs:
    req['case_ids'] = sorted({case for cid in req['capability_ids'] for case in by_id[cid]['case_ids']})
for path, value in [('spec/capabilities.json', caps), ('spec/requirements.json', reqs)]:
    (root/path).write_text(json.dumps(value, indent=2, ensure_ascii=False)+'\n')
