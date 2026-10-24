#!/usr/bin/env python3
"""Verify relocated companion globbing and the packaged OSC 8 helper contract."""
import argparse
import hashlib
import json
from pathlib import Path

from run_v2 import ROOT, run_case


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--reference', required=True, type=Path)
    parser.add_argument('--results', required=True, type=Path)
    args = parser.parse_args()
    binaries = [args.reference.resolve(), args.binary.resolve()]
    report = {}
    observations = []
    for binary in binaries:
        case = dict(id='companion-glob', mode='pty',
                    argv=['-X', '-Ps@@READY@@', '-Pm@@READY@@', '-PM@@READY@@', 'start'],
                    files={'start': {'text': 'start\n'*40},
                           'match one.txt': {'text': 'FIRST MATCH\n'*40},
                           'match two.txt': {'text': 'SECOND MATCH\n'*40}},
                    env={'LESSECHO': str(binary.parent/'lessecho')},
                    events=[{'await': ['@@READY@@']},
                            {'send': ':e match*.txt\n', 'await': ['@@READY@@'], 'snapshot': True},
                            {'send': 'q'}])
        observations.append(run_case(binary, case))
    assert observations[0] == observations[1], ('companion-glob', observations)
    assert observations[1]['checkpoints'][0]['rows'][0].startswith('FIRST MATCH')
    assert observations[1]['termios_restored']
    report['companion-glob'] = {'passed': True, 'actual': observations[1]}

    reference_helper = binaries[0].parent/'less-osc8-open.sh'
    helper = ROOT/'vendor/less-helpers/less-osc8-open'
    stub = '#!/bin/sh\nprintf "%s\\n" "$@" > invocation.log\n'
    for index, uri in enumerate(['file:///tmp/local', 'file://localhost/tmp/local',
                                  'file://this-host/tmp/local', 'file://remote-host/tmp/local',
                                  'file:invalid', 'file:///tmp/$(should-stay-literal)',
                                  'man:printf', 'man:printf(3)', 'https://example.invalid/']):
        observations = []
        for program in (reference_helper, helper):
            case = dict(id=f'osc8-helper-{index}', mode='pipe', argv=[str(program), uri],
                        env={'PATH': '{ROOT}/bin:/usr/bin:/bin', 'HOSTNAME': 'this-host',
                             'MANPAGER': 'configured-pager'},
                        files={f'bin/{name}': {'text': stub, 'mode': 0o700}
                               for name in ('less', 'rless', 'man')})
            observations.append(run_case(Path('/bin/sh'), case))
        assert observations[0] == observations[1], (uri, observations)
        report[f'osc8-helper-{index}'] = {'passed': True, 'uri': uri, 'actual': observations[1]}
    report['helper_sha256'] = hashlib.sha256(helper.read_bytes()).hexdigest()
    args.results.parent.mkdir(parents=True, exist_ok=True)
    args.results.write_text(json.dumps(report, indent=2)+'\n')
    print('10 companion/OSC 8 helper cases passed')


if __name__ == '__main__':
    main()
