#!/usr/bin/env python3
"""Executable supplemental IO boundary checks outside the frozen corpus."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_case, run_pty


def sha(data):
    return hashlib.sha256(data).hexdigest()


def pager(text='line\n' * 80, events=None, size=(24, 80), env=None):
    return dict(id='SUPPLEMENTAL-IO', mode='pty', size=list(size),
                argv=['-PsREADY', '-PmREADY', '-PMREADY', 'i'],
                files={'i': {'text': text}}, env=env or {},
                events=events or [{'await': ['READY'], 'snapshot': True}, {'send': 'q'}])


def fault(case, shim, plan):
    case = json.loads(json.dumps(case))
    case['env'].update(LD_PRELOAD=str(shim), RLESS_BOUNDARY_PLAN=plan,
                       RLESS_BOUNDARY_TRACE='{ROOT}/hit')
    case['files']['hit'] = {'text': ''}
    return case


def check(value, message):
    if not value:
        raise AssertionError(message)


def run(binary, output, *, retained=None):
    shim = output / 'boundary_fault.so'
    if retained is None:
        output.mkdir(parents=True, exist_ok=False)
        subprocess.run(['cc', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC',
                        str(Path(__file__).parent / 'native/boundary_fault.c'),
                        '-ldl', '-o', str(shim)], check=True)
    results = {}
    def observe(name, case, verify):
        try:
            actual = run_case(binary, case) if retained is None else retained[name]['observed']
            verify(actual)
            results[name] = dict(passed=True, observed=actual)
        except Exception as error:
            results[name] = dict(passed=False, error=f'{type(error).__name__}: {error}')
        if retained is None:
            (output / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
    def healthy(o):
        check(o['status'] == 0 and o['termios_restored'], 'exit or terminal restore failed')
    def hit(o):
        check(o['files']['hit']['sha256'] == sha(b'hit\n'), 'fault did not fire exactly once')

    for name, content in [('empty', ''), ('small', 'a\n' * 8)]:
        observe('one-row-' + name, pager(content, size=(1, 80),
                env={'LESS_LINES': '1', 'LESS_SHELL_LINES': '-99'}), healthy)
    growth = pager('a\n' * 128, size=(1, 80), env={'LESS_LINES': '1'}, events=[
        {'await': ['READY'], 'snapshot': True}, {'send': 'g', 'await': ['READY'], 'snapshot': True},
        {'resize': [2, 80], 'await': ['READY'], 'snapshot': True},
        {'resize': [100, 80], 'await': ['READY'], 'snapshot': True}, {'send': 'q'}])
    def check_growth(o):
        healthy(o)
        check([len(x['rows']) for x in o['checkpoints']] == [1, 1, 2, 100], 'missing resize checkpoints')
    observe('resize-growth', growth, check_growth)

    payload = b'line 000\n' * 100
    log = dict(id='SUPPLEMENTAL-LOG', mode='pty',
        argv=['-o', 'capture', '-PsREADY', '-PmREADY', '-PMREADY', '-'],
        stream_b64=base64.b64encode(payload).decode(), files={}, env={},
        events=[{'await': ['READY']}, {'close_stream': True}, {'send': 'q'}])
    for plan in ('baseline', 'log-short', 'log-eintr', 'log-eio', 'log-enospc', 'log-close'):
        def verify(o, plan=plan):
            check(o['termios_restored'], 'log case did not restore terminal')
            saved = o['files'].get('capture')
            check(saved, 'log not created')
            if plan in ('baseline', 'log-short', 'log-eintr'):
                check(o['status'] == 0 and saved['sha256'] == sha(payload), 'log bytes lost or duplicated')
            elif plan == 'log-close':
                check(o['status'] == 1 and saved['sha256'] == sha(payload), 'close error not reported')
            else:
                check(o['status'] == 1 and saved['bytes'] < len(payload), 'write error not reported')
            if plan != 'baseline': hit(o)
        observe(plan, log if plan == 'baseline' else fault(log, shim, plan), verify)

    endless = fault(log, shim, 'log-eio')
    endless['events'] = [{'await': ['READY']}, {'send': 'q'}]
    def check_endless(o):
        hit(o)
        check(o['status'] == 1 and o['termios_restored'], 'failed log did not stop open-stream drain')
        check(o['files']['capture']['bytes'] < len(payload), 'failed log unexpectedly completed')
    observe('log-eio-open-stream-exit', endless, check_endless)

    # Activate logging after the first screen read. This writes preexisting
    # buffered blocks through sync_logfile, unlike startup -o above.
    sync = json.loads(json.dumps(log))
    sync['argv'] = ['-PsREADY', '-PmREADY', '-PMREADY', '-']
    sync['events'] = [{'await': ['READY']},
        {'send': '-o capture\n', 'await': ['Log file']},
        {'send': '\n', 'await': ['READY']}, {'close_stream': True}, {'send': 'q'}]
    for plan in ('baseline', 'log-short', 'log-eintr', 'log-eio'):
        case = sync if plan == 'baseline' else fault(sync, shim, plan)
        if plan == 'log-eio':
            case = json.loads(json.dumps(case))
            case['events'][1]['await'] = ['No log file']
        def verify_sync(o, plan=plan):
            check(o['termios_restored'], 'sync path did not restore terminal')
            saved = o['files'].get('capture')
            check(saved, 'sync path did not create log')
            if plan == 'log-eio':
                check(o['status'] == 1 and saved['bytes'] < len(payload), 'sync failure not retained')
            else:
                check(o['status'] == 0 and saved['sha256'] == sha(payload), 'sync lost bytes')
            if plan != 'baseline': hit(o)
        observe('sync-' + plan, case, verify_sync)

    def terminal_failed(o):
        hit(o)
        check(o['status'] == 1 and not o['termios_restored'], 'failed restore claimed success')
    observe('terminal-restore-failure', fault(pager(), shim, 'term-restore'), terminal_failed)

    shell = pager(events=[{'await': ['READY']},
        {'send': '!printf HELPER > helper.out\n', 'await': ['!done']},
        {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
    def shell_ok(o):
        healthy(o)
        check('helper.out' in o['files'], 'helper did not run')
    observe('shell-baseline', shell, shell_ok)
    for plan in ('fd-backup', 'fd-swap-eintr', 'fd-swap-eio', 'fd-restore-eio', 'fd-tty-fallback'):
        case = fault(shell, shim, plan)
        if plan == 'fd-backup':
            case['events'] = [{'await': ['READY']},
                {'send': '!printf HELPER > helper.out\n', 'await': ['cannot preserve standard input']},
                {'send': 'q'}]
        if plan == 'fd-swap-eio':
            case['events'] = [{'await': ['READY']},
                {'send': '!printf HELPER > helper.out\n', 'await': ['cannot set standard input']},
                {'send': 'q'}]
        if plan == 'fd-restore-eio':
            case['events'] = [{'await': ['READY']}, {'send': '!printf HELPER > helper.out\n'}]
        if plan == 'fd-tty-fallback':
            case['events'] = [{'await': ['READY']},
                {'send': '!printf STDERR >&2; printf HELPER > helper.out\n', 'await': ['STDERR']},
                {'await': ['!done']}, {'send': '\n', 'await': ['READY']}, {'send': 'q'}]
        def verify(o, plan=plan):
            hit(o)
            if plan == 'fd-backup':
                healthy(o)
                check('helper.out' not in o['files'], 'helper ran without backup')
            elif plan == 'fd-swap-eio':
                healthy(o)
                check('helper.out' not in o['files'], 'helper ran after stdin swap failed')
            elif plan == 'fd-restore-eio':
                check(o['status'] == 1 and o['termios_restored'], 'failed restore did not exit cooked')
            else: shell_ok(o)
        observe(plan, case, verify)

    low_tty = pager(events=[{'await': ['READY']},
        {'send': '!if [ -e /proc/self/fd/2 ]; then printf LEAK; else printf HELPER; fi > helper.out\n',
         'await': ['!done']}, {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
    def low_tty_ok(o):
        hit(o)
        shell_ok(o)
        check(o['files']['helper.out']['sha256'] == sha(b'HELPER'), 'owned tty in fd2 leaked into shell')
    observe('fd-tty-owned-2', fault(low_tty, shim, 'fd-tty-owned-2'), low_tty_ok)

    history = pager(events=[{'await': ['READY']}, {'send': '/line\n', 'await': ['READY']},
                            {'send': 'q'}], env={'LESSHISTFILE': '{ROOT}/history'})
    old = b'OLD HISTORY\n'
    history['files']['history'] = {'text': old.decode(), 'mode': 0o600}
    def history_ok(o):
        healthy(o)
        check(o['files']['history']['sha256'] != sha(old), 'history not rewritten')
        check(o['files']['history']['mode'] == 0o600, 'history not private')
    observe('history-baseline', history, history_ok)
    for plan in ('hist-fdopen', 'hist-write', 'hist-ferror', 'hist-fclose', 'hist-rename'):
        def verify(o):
            healthy(o)
            hit(o)
            check(o['files']['history']['sha256'] == sha(old), 'old history replaced')
            check(not any(x.startswith('history.') for x in o['files']), 'temporary history leaked')
        observe(plan, fault(history, shim, plan), verify)

    # run_v2 fixture files cannot express symlinks. Set up an isolated PTY
    # directly to verify both success and rollback against an old symlink.
    for plan in ('baseline', 'hist-rename'):
        name = 'history-symlink-' + plan
        def verify_symlink(o, plan=plan):
            healthy(o)
            effect = o['symlink_effects']
            check(effect['victim_sha256'] == sha(old), 'symlink target modified')
            check(effect['temporary_names'] == [], 'temporary file left')
            if plan == 'baseline':
                check(effect['history_is_symlink'] is False, 'successful rename followed symlink')
                check(effect['history_mode'] == 0o600, 'new history not private')
            else:
                check(effect['history_is_symlink'] is True, 'failed rename replaced symlink')
                check(effect['hit_sha256'] == sha(b'hit\n'), 'rename fault missed')
        if retained is not None:
            observe(name, None, verify_symlink)
            continue
        try:
            with tempfile.TemporaryDirectory(prefix='rless-history-link-') as directory:
                work = Path(directory)
                (work / 'i').write_text('line\n' * 80)
                (work / 'victim').write_bytes(old)
                (work / 'history').symlink_to('victim')
                (work / 'hit').write_bytes(b'')
                env = dict(PATH='/usr/bin:/bin', HOME=directory, TERM='xterm-256color',
                           LC_ALL='C', LESSHISTFILE=str(work / 'history'))
                if plan != 'baseline':
                    env.update(LD_PRELOAD=str(shim), RLESS_BOUNDARY_PLAN=plan,
                               RLESS_BOUNDARY_TRACE=str(work / 'hit'))
                safety = os.environ.get('RLESS_SAFETY_ENV')
                if safety:
                    env.update(json.loads(Path(safety).read_text()))
                argv = [str(binary), '-PsREADY', '-PmREADY', '-PMREADY', 'i']
                o = run_pty(argv, env, work, history, lambda data: data.replace(directory.encode(), b'{ROOT}'))
                healthy(o)
                check((work / 'victim').read_bytes() == old, 'symlink target modified')
                check(not any(x.name.startswith('history.') for x in work.iterdir()), 'temporary file left')
                if plan == 'baseline':
                    check(not (work / 'history').is_symlink(), 'successful rename followed symlink')
                    check((work / 'history').stat().st_mode & 0o777 == 0o600, 'new history not private')
                else:
                    check((work / 'history').is_symlink(), 'failed rename replaced symlink')
                    check((work / 'hit').read_bytes() == b'hit\n', 'rename fault missed')
                o['symlink_effects'] = dict(victim_sha256=sha((work / 'victim').read_bytes()),
                    temporary_names=sorted(x.name for x in work.iterdir() if x.name.startswith('history.')),
                    history_is_symlink=(work / 'history').is_symlink(),
                    history_mode=(work / 'history').stat().st_mode & 0o777,
                    hit_sha256=sha((work / 'hit').read_bytes()))
                verify_symlink(o)
                results[name] = dict(passed=True, observed=o)
        except Exception as error:
            results[name] = dict(passed=False, error=f'{type(error).__name__}: {error}')
        if retained is None:
            (output / 'results.json').write_text(json.dumps(results, indent=2) + '\n')

    summary = dict(executed=len(results), passed=sum(v['passed'] for v in results.values()),
                   failed=[k for k, v in results.items() if not v['passed']],
                   binary_sha256=sha(binary.read_bytes()),
                   test_sha256=sha(Path(__file__).read_bytes()),
                   shim_source_sha256=sha((Path(__file__).parent / 'native/boundary_fault.c').read_bytes()),
                   source_sha256={name: sha((Path(__file__).resolve().parents[2] / name).read_bytes())
                                  for name in ('src/ch.rs', 'src/cmdbuf.rs', 'src/lsystem.rs',
                                               'src/pager.rs', 'src/position.rs', 'src/screen.rs')},
                   asan_environment=bool(os.environ.get('RLESS_SAFETY_ENV')))
    if retained is not None:
        check(set(retained) == set(results) and len(results) == 30, 'missing or extra boundary cases')
    else:
        (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
        print(json.dumps(summary))
    return not summary['failed']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    return 0 if run(args.binary.resolve(), args.output.resolve()) else 1


if __name__ == '__main__':
    sys.exit(main())
