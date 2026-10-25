"""Real pager fault/recovery paths. Whole acceptance coverage remains partial."""
import argparse
import copy
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_case

NORMAL = INJECTED = SHIM = SHORT_SHIM = None
REPETITIONS = 1
INSTRUMENTATION = {}


def marker(site, ordinal=1, magic=b'RLF1'):
    return struct.pack('<4sIQ', magic, site, ordinal)


def pipe_run(binary, plan=None, argv=(), payload=b'payload\n', files=None, native_short=False, unreadable=False):
    read_fd, write_fd = os.pipe2(os.O_CLOEXEC | os.O_NONBLOCK)
    with tempfile.TemporaryDirectory(prefix='rless-fault-') as directory:
        for name, value in (files or {}).items():
            (Path(directory) / name).write_bytes(value)
            if unreadable:
                (Path(directory) / name).chmod(0)
        env = dict(PATH='/usr/bin:/bin', HOME=directory, LC_ALL='C', TERM='dumb', LESSHISTFILE='-',
                   **INSTRUMENTATION)
        if plan:
            env.update(RLESS_FAULT=plan, RLESS_FAULT_FD=str(write_fd))
        if native_short:
            env.update(LD_PRELOAD=str(SHORT_SHIM), RLESS_NATIVE_SHORT_WRITE_FD=str(write_fd))
        try:
            result = subprocess.run([str(binary), *argv], input=payload, env=env, cwd=directory,
                capture_output=True, pass_fds=[write_fd], timeout=5)
        finally:
            os.close(write_fd)
            trace = os.read(read_fd, 65536)
            os.close(read_fd)
    return result, trace


def pty_run(binary, case, plan=None, native=False):
    read_fd, write_fd = os.pipe2(os.O_CLOEXEC | os.O_NONBLOCK)
    case = copy.deepcopy(case)
    # run_v2 forks/execs PTY children. Lend only the dedicated write FD across
    # that exec; the production feature makes it close-on-exec before helpers.
    os.set_inheritable(write_fd, True)
    env = case.setdefault('env', {})
    env.update(INSTRUMENTATION)
    if plan:
        env.update(RLESS_FAULT=plan, RLESS_FAULT_FD=str(write_fd))
    if native:
        env.update(LD_PRELOAD=str(SHIM), RLESS_NATIVE_CLEANUP_FD=str(write_fd))
    try:
        observed = run_case(binary, case)
    finally:
        os.close(write_fd)
        trace = os.read(read_fd, 65536)
        os.close(read_fd)
    return observed, trace


def pager(events=None, options=()):
    return dict(id='FAULT-PATH', mode='pty',
        argv=[*options, '-PsREADY', '-PmREADY', '-PMREADY', 'i'],
        files={'i': {'text': ''.join(f'keep {i:03d}\nDROP {i:03d}\n' for i in range(40))}},
        events=events or [{'await': ['READY'], 'snapshot': True}, {'send': 'q'}])


def require_observation(actual, expected, trace, site, ordinal=1):
    if trace != marker(site, ordinal):
        raise ValueError('missing, wrong-site or wrong-ordinal hit evidence')
    if actual != expected:
        raise ValueError('wrong status, terminal state, screen, bells or file effects')


class ActualFaultPaths(unittest.TestCase):
    def test_read_eintr_and_short_reads_recover_exact_bytes(self):
        for site, site_id in [('read-eintr', 14), ('read-short', 15)]:
            for ordinal in (1, 2):
                payload = b'payload\n' * 4096
                result, trace = pipe_run(INJECTED, f'{site}:{ordinal}', payload=payload)
                self.assertEqual(trace, marker(site_id, ordinal))
                self.assertEqual((result.returncode, result.stdout, result.stderr), (0, payload, b''))

    def test_diagnostic_read_error_reproduces_unreported_legacy_failure(self):
        result, trace = pipe_run(INJECTED, 'read:1')
        self.assertEqual(trace, marker(2))
        self.assertEqual(result.stdout, b'')
        self.assertEqual((result.returncode, result.stderr), (0, b''))
        # This test reproduces a finding. It must never bind to a complete
        # read-error safety acceptance row or turn status zero into acceptance.
        print('FINDING M0-IO-001: read EIO silently exits 0 with unread data', flush=True)

    def test_diagnostic_write_error_reproduces_unreported_legacy_failure(self):
        result, trace = pipe_run(INJECTED, 'write:1')
        self.assertEqual(trace, marker(4))
        self.assertEqual((result.returncode, result.stdout, result.stderr), (0, b'', b''))
        print('FINDING M0-IO-002: write EIO silently exits 0 with lost output', flush=True)

    def test_short_write_retries_the_exact_remaining_bytes(self):
        for binary, plan, native in ((INJECTED, 'write-short:1', False), (NORMAL, None, True)):
            result, trace = pipe_run(binary, plan, native_short=native)
            self.assertEqual(trace, marker(16, magic=b'RNT1' if native else b'RLF1'))
            self.assertEqual((result.returncode, result.stdout, result.stderr), (0, b'payload\n', b''))

    def test_saved_command_allocation_failures_have_exact_hit_and_status(self):
        reached = 0
        for ordinal in range(1, 25):
            result, trace = pipe_run(INJECTED, f'alloc:{ordinal}',
                argv=['--cmd=12g', '--cmd=1g', 'i'], files={'i': b'payload\n'})
            if trace:
                reached += 1
                self.assertEqual(trace, marker(1, ordinal))
                self.assertEqual(result.returncode, 1)
                self.assertIn(b'Cannot allocate memory', result.stdout + result.stderr)
            else:
                self.assertEqual(result.returncode, 0)
        self.assertGreaterEqual(reached, 4)
        print('Saved-command startup allocation ordinals reached:', reached, flush=True)

    def test_open_permission_failure_matches_real_native_denial(self):
        actual, trace = pipe_run(INJECTED, 'open:1', argv=['i'], files={'i': b'payload\n'})
        native, _ = pipe_run(NORMAL, argv=['i'], files={'i': b'payload\n'}, unreadable=True)
        self.assertEqual(trace, marker(3))
        self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                         (native.returncode, native.stdout, native.stderr))
        self.assertEqual(actual.returncode, 1)
        self.assertIn(b'Permission denied', actual.stderr)

    def test_parser_failure_then_valid_content_is_usable(self):
        result, trace = pipe_run(INJECTED, 'parser:1', argv=[
            '--lesskey-content=#env\nLESS=-N', '--lesskey-content=#env\nLESS=-S'])
        self.assertEqual(trace, marker(6))
        self.assertEqual((result.returncode, result.stdout), (0, b'payload\n'))
        self.assertEqual(result.stderr, b'Error in lesskey content\n')

    def test_reserve_failure_before_parser_growth_has_controlled_oom(self):
        result, trace = pipe_run(INJECTED, 'reserve:1', argv=['--lesskey-content=#env\nLESS=-N'])
        self.assertEqual(trace, marker(12))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, b'')
        self.assertIn(b'Cannot allocate memory', result.stderr)

    def test_cleanup_retries_injected_and_native_eintr_and_restores_terminal(self):
        case = pager()
        baseline, trace = pty_run(NORMAL, case)
        self.assertEqual(trace, b'')
        self.assertTrue(baseline['termios_restored'])
        self.assertEqual(baseline['status'], 0)
        for _ in range(REPETITIONS):
            actual, trace = pty_run(INJECTED, case, 'cleanup:1')
            require_observation(actual, baseline, trace, 11)
            actual, trace = pty_run(NORMAL, case, native=True)
            self.assertEqual(trace, marker(11, magic=b'RNT1'))
            self.assertEqual(actual, baseline)

    def test_cleanup_preserves_primary_termination_status(self):
        case = pager([{'await': ['READY']}, {'signal': 'SIGTERM'}])
        baseline, _ = pty_run(NORMAL, case)
        self.assertEqual(baseline['status'], 15)
        for native in (False, True):
            actual, trace = pty_run(NORMAL if native else INJECTED, case,
                None if native else 'cleanup:1', native=native)
            self.assertEqual(trace, marker(11, magic=b'RNT1' if native else b'RLF1'))
            self.assertTrue(actual['termios_restored'])
            self.assertEqual(actual, baseline)

    def test_cleanup_unreached_ordinal_and_normal_build_absence(self):
        case = pager()
        baseline, _ = pty_run(NORMAL, case)
        for binary, plan in ((INJECTED, 'cleanup:999999'), (NORMAL, 'cleanup:1')):
            actual, trace = pty_run(binary, case, plan)
            self.assertEqual(trace, b'')
            self.assertEqual(actual, baseline)

    def test_normal_build_absence_for_every_site(self):
        self.assertNotIn(b'RLESS_FAULT', NORMAL.read_bytes())
        for site in ('alloc', 'read', 'open', 'write', 'regex', 'parser', 'filter-before',
                     'filter-after', 'handoff', 'callback', 'cleanup', 'reserve', 'close',
                     'read-eintr', 'read-short', 'write-short'):
            result, trace = pipe_run(NORMAL, site + ':1')
            self.assertEqual((result.returncode, result.stdout, result.stderr, trace),
                             (0, b'payload\n', b'', b''))

    def test_unreached_ordinal_cannot_supply_hit_evidence_at_any_site(self):
        for site, argv, files in (
            ('alloc', [], {}), ('read', [], {}), ('read-eintr', [], {}), ('read-short', [], {}),
            ('write', [], {}), ('write-short', [], {}), ('open', ['i'], {'i': b'payload\n'}),
            ('parser', ['--lesskey-content=#env\nLESS=-N'], {}),
            ('reserve', ['--lesskey-content=#env\nLESS=-N'], {}),
        ):
            baseline, _ = pipe_run(NORMAL, argv=argv, files=files)
            actual, trace = pipe_run(INJECTED, site + ':999999', argv=argv, files=files)
            self.assertEqual(trace, b'')
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (baseline.returncode, baseline.stdout, baseline.stderr))
        for site in ('regex', 'callback', 'cleanup', 'close', 'filter-before', 'filter-after', 'handoff'):
            if site.startswith('filter'):
                case = pager([{'await': ['READY']}, {'send': '&keep\n', 'await': ['READY']}, {'send': 'q'}])
            elif site == 'handoff':
                case = pager([{'await': ['READY']}, {'send': '!true\n', 'await': ['!done']},
                              {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
            else:
                case = pager(options=['-pkeep'] if site == 'regex' else [])
            baseline, _ = pty_run(NORMAL, case)
            actual, trace = pty_run(INJECTED, case, site + ':999999')
            self.assertEqual(trace, b'')
            self.assertEqual(actual, baseline)

    def test_real_cleanup_oracle_rejects_hit_status_screen_mode_bell_and_file_mutations(self):
        baseline, _ = pty_run(NORMAL, pager())
        actual, trace = pty_run(INJECTED, pager(), 'cleanup:1')
        require_observation(actual, baseline, trace, 11)
        for wrong in (b'', marker(10), marker(11, 2)):
            with self.assertRaises(ValueError):
                require_observation(actual, baseline, wrong, 11)
        for field, value in (('status', 99), ('termios_restored', False), ('files', {})):
            wrong = copy.deepcopy(actual)
            wrong[field] = value
            with self.assertRaises(ValueError):
                require_observation(wrong, baseline, trace, 11)
        for field in ('rows', 'bells', 'modes'):
            wrong = copy.deepcopy(actual)
            wrong['checkpoints'][0][field] = ['WRONG']
            with self.assertRaises(ValueError):
                require_observation(wrong, baseline, trace, 11)

    def test_regex_compile_failure_retains_recovery_behavior(self):
        def case(expression):
            return pager([{'await': ['READY']}, {'send': '/' + expression + '\n', 'await': ['Invalid pattern']},
                {'send': '\n', 'await': ['READY']}, {'send': '/keep\n', 'await': ['READY'], 'snapshot': True},
                {'send': 'q'}])
        baseline, _ = pty_run(NORMAL, case('['))
        actual, trace = pty_run(INJECTED, case('keep'), 'regex:1')
        self.assertEqual(trace, marker(5))
        self.assertEqual(actual, baseline)
        self.assertTrue(actual['termios_restored'])

    def test_close_failure_is_after_real_release_and_preserves_terminal(self):
        case = pager()
        baseline, _ = pty_run(NORMAL, case)
        actual, trace = pty_run(INJECTED, case, 'close:1')
        self.assertEqual(trace, marker(13))
        self.assertEqual(actual, baseline)
        # Exit reaps process FDs. In-process repeated FD conservation still needs
        # a separate lifetime oracle; a post-close event alone cannot prove it.

    def test_filter_failure_before_and_after_commit_has_distinct_state(self):
        for site, site_id, message, committed in (
            ('filter-before', 7, 'Injected filter commit failure', False),
            ('filter-after', 8, 'Injected post-commit failure', True),
        ):
            case = pager([{'await': ['READY']}, {'send': '&keep\n', 'await': [message]},
                {'send': '\n', 'await': ['READY']}, {'send': 'g', 'await': ['READY'], 'snapshot': True},
                {'send': '&\n', 'await': ['READY']}, {'send': 'g', 'await': ['READY'], 'snapshot': True},
                {'send': 'q'}])
            actual, trace = pty_run(INJECTED, case, site + ':1')
            self.assertEqual(trace, marker(site_id))
            first = '\n'.join(actual['checkpoints'][0]['rows'][:-1])
            recovered = '\n'.join(actual['checkpoints'][1]['rows'][:-1])
            self.assertIn('keep 000', first)
            self.assertEqual('DROP 000' not in first, committed)
            self.assertIn('DROP 000', recovered)
            self.assertEqual(actual['status'], 0)
            self.assertTrue(actual['termios_restored'])

    def test_handoff_failure_skips_child_then_recovers_and_closes_trace_on_exec(self):
        command = 'if test -e /proc/$$/fd/$RLESS_FAULT_FD; then printf LEAK; else printf CLOSED; fi > child.out'
        case = pager([{'await': ['READY']}, {'send': '!printf FIRST > skipped.out\n', 'await': ['!done']},
            {'send': '\n', 'await': ['READY']}, {'send': '!' + command + '\n', 'await': ['!done']},
            {'send': '\n', 'await': ['READY'], 'snapshot': True}, {'send': 'q'}])
        actual, trace = pty_run(INJECTED, case, 'handoff:1')
        self.assertEqual(trace, marker(9))
        self.assertNotIn('skipped.out', actual['files'])
        import hashlib
        self.assertEqual(actual['files']['child.out']['sha256'], hashlib.sha256(b'CLOSED').hexdigest())
        self.assertEqual(actual['status'], 0)
        self.assertTrue(actual['termios_restored'])

    def test_callback_reachability_is_not_native_error_propagation(self):
        baseline, _ = pty_run(NORMAL, pager())
        actual, trace = pty_run(INJECTED, pager(), 'callback:1')
        self.assertEqual(trace, marker(10))
        self.assertEqual(actual, baseline)
        print('GAP: callback sentinel reachability only; tputs/cost error propagation remains unverified', flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--normal', type=Path, required=True)
    parser.add_argument('--injected', type=Path, required=True)
    parser.add_argument('--native-shim', type=Path, required=True)
    parser.add_argument('--short-write-shim', type=Path, required=True)
    parser.add_argument('--repetitions', type=int, default=1)
    parser.add_argument('--instrumentation-env', type=Path)
    args, remaining = parser.parse_known_args()
    if args.repetitions < 1:
        parser.error('repetitions must be positive')
    NORMAL, INJECTED, SHIM = (p.resolve(strict=True) for p in (args.normal, args.injected, args.native_shim))
    SHORT_SHIM = args.short_write_shim.resolve(strict=True)
    REPETITIONS = args.repetitions
    if args.instrumentation_env:
        INSTRUMENTATION = json.loads(args.instrumentation_env.read_text())
        if set(INSTRUMENTATION) - {'ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH'}:
            parser.error('unsupported instrumentation variable')
    unittest.main(argv=[sys.argv[0], *remaining], verbosity=2)
