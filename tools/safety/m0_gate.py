#!/usr/bin/env python3
"""Derive M0 infrastructure closure without pretending to close the M5 catalogue.

Receipt paths refer to retained evidence. No status boolean supplied by a caller
can substitute for a current source inventory, a completed review or an oracle.
"""
import argparse
import json
from pathlib import Path
import re

from common import ROOT, PINS, fingerprint, git, is_external_report, sha, validate_report, write_json
from coverage import bind, matrix
from ownership import validate as validate_ownership

POLICY_VERSION = '1.0.0'
REQUIRED_ABIS = ('REFERENCE-GLIBC-2.41', 'BUILDER-GLIBC-2.44')
HOOK_PROFILES = ('NATIVE', 'ASAN')
HOOK_CONTROLS = ('fired', 'unfaulted', 'unreached', 'ordinal-separation',
                 'native-counterpart', 'resource-conservation')
ALTERNATIVE_CONTROLS = {'native-counterpart', 'resource-conservation'}
MINIMUM_SITES = {'alloc', 'read', 'open', 'write', 'regex', 'parser', 'filter-before',
                 'filter-after', 'handoff', 'callback', 'cleanup', 'reserve', 'close',
                 'read-eintr', 'read-short', 'write-short'}
COMMON_CONTROLS = {
    'inventory-omission-controls': ('C0-OWN-1', 'STATIC'),
    'ownership-omission-controls': ('C0-OWN-2', 'STATIC'),
    'binding-mutation-controls': ('C0-COVER-1', 'STATIC'),
    'normal-debug-hook-absence': ('C0-HOOK-1', 'NATIVE'),
    'normal-release-hook-absence': ('C0-HOOK-1', 'NATIVE'),
    'malformed-hook-plans': ('C0-HOOK-1', 'NATIVE'),
    'trace-channel-controls': ('C0-HOOK-1', 'NATIVE'),
    'counter-exhaustion-control': ('C0-HOOK-1', 'NATIVE'),
    'component-miri-controls': ('C0-COVER-2', 'MIRI'),
    'fuzz-engine-negative-control': ('C0-COVER-2', 'FUZZ'),
}
ABI_CONTROLS = {
    'native-provenance': ('C0-ABI-1', 'ABI'),
    'abi-layout-probes': ('C0-ABI-2', 'ABI'),
    'acceptance-debug': ('C0-ABI-2', 'NATIVE'),
    'acceptance-release': ('C0-ABI-2', 'NATIVE'),
    'acceptance-reference': ('C0-ABI-2', 'NATIVE'),
    'acceptance-installed': ('C0-ABI-2', 'NATIVE'),
    'acceptance-source-rebuild': ('C0-ABI-2', 'NATIVE'),
    'helpers': ('C0-ABI-2', 'NATIVE'),
    'integrations': ('C0-ABI-2', 'NATIVE'),
    'job-control': ('C0-ABI-2', 'NATIVE'),
    'asan-detector-control': ('C0-COVER-2', 'ASAN'),
    'lsan-detector-control': ('C0-COVER-2', 'LSAN'),
    'harness-sanitizer-capture-control': ('C0-COVER-2', 'ASAN'),
}


def declared_sites(root):
    source = (root / 'src/fault.rs').read_text()
    sites = set(re.findall(r'b"([a-z][a-z0-9-]*)"\s*=>\s*Site::', source))
    if not MINIMUM_SITES <= sites:
        raise ValueError('fault-site catalogue lost a required M0 site')
    return sites


def obligations(root=ROOT):
    """Versioned C0 controls; site-dependent alternatives need reviewed evidence."""
    if not set(REQUIRED_ABIS) <= {p['id'] for p in PINS['abi_profiles'] if p['required']}:
        raise ValueError('required ABI profile was removed')
    result = {}
    for control, (packet, profile) in COMMON_CONTROLS.items():
        result[control] = dict(control=control, packet=packet, profile=profile,
                               abi_profile='BUILDER-GLIBC-2.44', site=None)
    for abi in REQUIRED_ABIS:
        for control, (packet, profile) in ABI_CONTROLS.items():
            result[f'{control}:{abi}'] = dict(control=control, packet=packet,
                profile=profile, abi_profile=abi, site=None)
        for site in sorted(declared_sites(root)):
            for profile in HOOK_PROFILES:
                for control in HOOK_CONTROLS:
                    result[f'hook:{site}:{control}:{abi}:{profile}'] = dict(
                        control=control, packet='C0-HOOK-2', profile=profile,
                        abi_profile=abi, site=site)
    return result


def checked_ref(value, root=ROOT):
    if not isinstance(value, dict) or set(value) != {'path', 'sha256'}:
        raise ValueError('evidence references require path and sha256')
    path = Path(value['path'])
    if not path.is_absolute():
        path = root / path
    path = path.resolve()
    if not path.is_file() or sha(path) != value['sha256']:
        raise ValueError('missing or changed evidence: ' + str(path))
    return path


def source_bound_review(review, current_files, root, *, decision=None):
    if not isinstance(review.get('reviewer'), str) or not review['reviewer'].strip():
        raise ValueError('independent reviewer identity missing')
    if not review.get('implementation_author') or review['reviewer'] == review['implementation_author']:
        raise ValueError('implementation author cannot provide independent approval')
    if decision is not None and review.get('decision') != decision:
        raise ValueError('independent review does not approve the declared scope')
    sources = review.get('source_files_sha256')
    if not sources or any(current_files.get(path) != digest for path, digest in sources.items()):
        raise ValueError('review has missing or stale source scope')
    note = checked_ref(review['evidence'], root)
    if not note.read_text().strip():
        raise ValueError('empty independent review notes')


def validated_evidence(path, root):
    raw = json.loads(path.read_text())
    if 'input_fingerprints' in raw or is_external_report(raw):
        return validate_report(path, root, allow_open_findings=True)
    # No generic status-only fallback is permitted for unknown report shapes.
    raise ValueError('unknown M0 evidence report format')


def validate_candidate(candidate, root=ROOT):
    root = root.resolve()
    current = fingerprint(root)
    if (type(candidate.get('schema_version')) is not int or candidate.get('schema_version') != 1
            or candidate.get('policy_version') != POLICY_VERSION):
        raise ValueError('unknown M0 gate policy')
    if candidate.get('source_commit') != git('rev-parse', 'HEAD', root=root).decode().strip():
        raise ValueError('candidate commit changed')
    if candidate.get('input_fingerprints') != current:
        raise ValueError('M0 candidate inputs changed')
    own = {name: json.loads(checked_ref(candidate['ownership'][name], root).read_text())
           for name in ('inventory', 'regions', 'review', 'findings')}
    if own['inventory'].get('schema_version', 0) < 2:
        raise ValueError('M0 needs the all-static and feature-expanded inventory')
    ownership = validate_ownership(own['review'], own['inventory'], own['regions'],
                                   own['findings'], root=root, complete=True)

    expected = obligations(root)
    selectors = candidate.get('selectors', {})
    alternatives = candidate.get('applicability', {})
    if set(selectors) & set(alternatives) or set(selectors) | set(alternatives) != set(expected):
        raise ValueError('missing, extra or doubly selected M0 obligations')
    reports = {}
    records = {}
    for ref in candidate.get('reports', []):
        path = checked_ref(ref, root)
        if str(path) in reports:
            raise ValueError('duplicate M0 evidence report')
        # Full-report validation prevents omitted unsupported infrastructure from
        # being hidden behind one passing selector. Final validation stays stricter.
        report = validated_evidence(path, root)
        if report['source_commit'] != candidate['source_commit']:
            raise ValueError('M0 evidence was executed at a different commit')
        reports[str(path)] = report
        records.update({(str(path), item['name']): item for item in report['records']})
    if not reports:
        raise ValueError('no executed M0 evidence')

    selected = set()

    def select(value, obligation, control):
        if not isinstance(value, dict) or set(value) != {'report', 'record_name'}:
            raise ValueError('invalid M0 evidence selector')
        path = Path(value['report'])
        path = path if path.is_absolute() else root / path
        key = (str(path.resolve()), value['record_name'])
        item = records.get(key)
        if item is None:
            raise ValueError('unexecuted M0 evidence selector')
        metadata = item.get('oracle_metadata') or {}
        if item['profile'] != obligation['profile'] or metadata.get('abi_profile') != obligation['abi_profile']:
            raise ValueError('M0 evidence uses the wrong profile or ABI')
        controls = metadata.get('m0_controls', [])
        if control not in controls and not (control == 'ordinal-separation' and 'fired' in controls):
            raise ValueError('executed oracle does not implement the selected M0 control')
        if obligation['site'] is None and control not in item.get('_validated_m0_controls', []):
            raise ValueError('M0 infrastructure labels need rederived command/probe evidence')
        if obligation['site'] is not None and obligation['site'] not in metadata.get('hook_sites', []):
            raise ValueError('M0 hook evidence has the wrong site')
        if obligation['site'] is not None and (
                not item.get('oracle_report') or not isinstance(item.get('observed_hooks'), list)):
            raise ValueError('M0 hook controls need validated retained observations, not metadata alone')
        if control == 'harness-sanitizer-capture-control' and item.get('_validated_detector_capture') is not True:
            raise ValueError('M0 detector capture needs revalidated pipe/PTY diagnostics')
        if control.startswith('acceptance-'):
            if (any(type(metadata.get(field)) is not int for field in
                    ('executed_cases', 'passed_cases', 'failed_cases'))
                    or metadata.get('executed_cases') != 809 or metadata.get('passed_cases') != 809
                    or metadata.get('failed_cases') != 0):
                raise ValueError('M0 compatibility evidence is not the full frozen corpus')
        selected.add(key)
        return item

    def select_many(value, obligation, control):
        values = value if isinstance(value, list) else [value]
        if not values:
            raise ValueError('empty M0 oracle selection')
        identities = [json.dumps(item, sort_keys=True) for item in values]
        if len(set(identities)) != len(identities):
            raise ValueError('duplicate M0 oracle selection')
        items = [select(item, obligation, control) for item in values]
        if obligation['site'] is not None:
            events = [event for item in items for event in item.get('observed_hooks', [])
                      if event['site'] == obligation['site']]
            fired = {event['ordinal'] for event in events if event['magic'] == 'RLF1'}
            native = {event['ordinal'] for event in events if event['magic'] == 'RNT1'}
            if control in ('fired', 'termination-cleanup') and not fired:
                raise ValueError('fired hook control has no actual matching RLF1 marker')
            if control == 'ordinal-separation' and len(fired) < 2:
                raise ValueError('hook ordinal separation lacks two actual observed ordinals')
            if control == 'native-counterpart' and not native:
                raise ValueError('native counterpart has no actual matching RNT1 marker')
        return items

    for key, value in selectors.items():
        select_many(value, expected[key], expected[key]['control'])
    for key, value in alternatives.items():
        obligation = expected[key]
        if obligation['control'] not in ALTERNATIVE_CONTROLS or obligation['site'] is None:
            raise ValueError('this M0 obligation cannot be waived by applicability')
        if not value.get('rationale') or value.get('decision') != 'alternate-required':
            raise ValueError('site applicability needs a reviewed rationale and alternate')
        source_bound_review(value, current['files'], root)
        if 'src/fault.rs' not in value['source_files_sha256']:
            raise ValueError('hook applicability is not bound to the hook implementation')
        alternate = value.get('alternative_control')
        if not alternate or alternate == obligation['control']:
            raise ValueError('missing specific alternate oracle')
        choices = value.get('alternative_selectors')
        if not isinstance(choices, list) or not choices:
            raise ValueError('applicability cannot replace an executed alternate test')
        select_many(choices, obligation, alternate)

    live = {item['id']: item for item in own['findings']['findings'] if item['status'] == 'open'}
    dispositions = candidate.get('legacy_dispositions', [])
    by_id = {item['finding_id']: item for item in dispositions}
    if len(by_id) != len(dispositions):
        raise ValueError('duplicate legacy finding disposition')
    observed = {finding for item in records.values() for finding in item.get('finding_ids', [])}
    reviewed_hazards = {finding for item in own['review']['entries'] for finding in item.get('finding_ids', [])}
    infrastructure = {key for key, item in live.items() if item.get('delivery_packet', '').startswith('M0.')}
    if infrastructure:
        raise ValueError('open M0 infrastructure gap: ' + ', '.join(sorted(infrastructure)))
    if set(by_id) != set(live) | observed | reviewed_hazards:
        raise ValueError('legacy finding disposition missing or unrelated')
    for key, value in by_id.items():
        finding = live.get(key)
        if finding is None or finding.get('blocks_final_safety_gate') is not True:
            raise ValueError('legacy finding is not live or no longer blocks final safety')
        packet = value.get('delivery_packet', '')
        if packet != finding.get('delivery_packet') or not re.fullmatch(r'M[1-4]\.\d+', packet):
            raise ValueError('an M0 infrastructure gap cannot be deferred as a legacy finding')
        if not all(isinstance(value.get(field), str) and value[field].strip()
                   for field in ('owner', 'reason', 'regression')):
            raise ValueError('legacy finding lacks an owner, rationale or regression')
        sources = value.get('source_files_sha256')
        if not sources or any(current['files'].get(path) != digest for path, digest in sources.items()):
            raise ValueError('legacy disposition is not source-bound')
        if not checked_ref(value['evidence'], root).read_text().strip():
            raise ValueError('legacy finding has no retained evidence')

    catalogue = json.loads((root / 'spec/safety/acceptance.json').read_text())
    binding_file = checked_ref(candidate['bindings'], root)
    bindings = json.loads(binding_file.read_text())['bindings']
    rows = bind(matrix(catalogue), bindings, root)
    bound = {item['row_id'] for item in bindings}
    deferred = candidate.get('row_dispositions', {})
    if set(deferred) != set(rows) - bound:
        raise ValueError('every unbound final row needs an explicit later disposition')
    packets = {item['id'] for item in json.loads((root / 'spec/safety/work-packages.json').read_text())['work_packages']}
    for value in deferred.values():
        if value.get('status') not in ('not-run', 'blocked') or value.get('delivery_packet') not in packets or not value.get('reason'):
            raise ValueError('invalid final-row disposition')
    for (report_path, record_name), item in records.items():
        if item.get('scenario_id') is not None:
            matches = []
            for binding in bindings:
                for execution in binding.get('executions', [binding]):
                    path = Path(execution['report'])
                    path = path if path.is_absolute() else root / path
                    if execution['record_name'] == record_name and str(path.resolve()) == report_path:
                        matches.append(binding)
            if not matches:
                raise ValueError('executed acceptance oracle is not bound to an exact row')

    review = candidate['independent_review']
    source_bound_review(review, current['files'], root, decision='approve-m0-infrastructure')
    if review['source_files_sha256'] != current['files']:
        raise ValueError('M0 closure review does not cover all current inputs')
    return dict(schema_version=1, policy_version=POLICY_VERSION, status='m0-verified',
        source_commit=candidate['source_commit'], input_fingerprints=current,
        ownership=ownership, obligations=len(expected), alternate_obligations=len(alternatives),
        legacy_findings=sorted(by_id), final_acceptance_rows=len(rows),
        final_rows_passed=sum(row['status'] == 'passed' for row in rows.values()),
        runtime_safety_verified=False, m5_verified=False,
        scope='M0 infrastructure and review only; legacy findings and later acceptance remain open.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path)
    parser.add_argument('--describe', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.describe:
        write_json(args.output, dict(policy_version=POLICY_VERSION, obligations=obligations()))
        return 0
    if args.candidate is None:
        parser.error('--candidate is required unless --describe is selected')
    try:
        result = validate_candidate(json.loads(args.candidate.read_text()))
    except (ValueError, KeyError, OSError, TypeError) as error:
        write_json(args.output, dict(schema_version=1, policy_version=POLICY_VERSION,
            status='not-verified', runtime_safety_verified=False, m5_verified=False, error=str(error)))
        print('M0 OPEN: ' + str(error))
        return 1
    write_json(args.output, result)
    print('M0 infrastructure verified; M1–M5 and runtime safety remain separate.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
