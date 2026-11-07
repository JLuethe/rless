"""Real pager fault/recovery paths. Whole acceptance coverage remains partial."""
import argparse
import base64
import copy
from contextlib import ExitStack, contextmanager
import hashlib
import json
import os
from pathlib import Path
import signal
import struct
import subprocess
import sys
import tempfile
import threading
import time
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'compat'))
from run_v2 import run_case

NORMAL = INJECTED = SHIM = SHORT_SHIM = IO_SHIM = REFERENCE = ALLOC_BINDING = None
REPETITIONS = 1
INSTRUMENTATION = {}
OBSERVATIONS = []
# These methods repeat recovery inside their own session/lifetime oracle.
# Every other method is invoked once per requested repetition.
INTERNAL_REPEAT_METHODS = {
    'test_repeated_file_and_child_failures_conserve_live_process_resources',
    'test_injected_recovery_conserves_descriptors_and_children_in_live_pager',
    'test_callback_startup_failure_preserves_later_session_lifecycle',
    'test_cleanup_retries_injected_and_native_eintr_and_restores_terminal',
}


def marker(site, ordinal=1, magic=b'RLF1'):
    return struct.pack('<4sIQ', magic, site, ordinal)


@contextmanager
def trace_channel():
    read_fd, write_fd = os.pipe2(os.O_CLOEXEC | os.O_NONBLOCK)
    with os.fdopen(read_fd, 'rb', buffering=0) as reader, os.fdopen(write_fd, 'wb', buffering=0) as writer:
        yield reader, writer


def read_trace(reader, writer):
    writer.close()
    chunks = []
    length = 0
    while True:
        data = reader.read(65536)
        if data is None:
            raise ValueError('trace writer remains open after process completion')
        if not data:
            return b''.join(chunks)
        length += len(data)
        if length > 65536:
            raise ValueError('trace exceeds bounded evidence channel')
        chunks.append(data)


def finish_process_group(pid, grace_seconds=1.0):
    """Allow bounded asynchronous reaping; reject and kill a surviving group."""
    deadline = time.monotonic() + grace_seconds
    while True:
        try:
            os.killpg(pid, 0)
        except ProcessLookupError:
            return
        if time.monotonic() >= deadline:
            try:
                os.killpg(pid, signal.SIGKILL)
            except ProcessLookupError:
                return
            raise ValueError('child process group survived pager termination')
        time.sleep(0.01)


def pipe_run(binary, plan=None, argv=(), payload=b'payload\n', files=None, native_short=False,
             unreadable=False, native_plan=None, native_alloc_plan=None, seekable_stdin=False,
             lesskey_env=None, message_content=None, more_mode=False,
             unbounded_stdin=False, stdout_full=False, stderr_full=False):
    with ExitStack() as handles, trace_channel() as (reader, writer), tempfile.TemporaryDirectory(prefix='rless-fault-') as directory:
        write_fd = writer.fileno()
        for name, value in (files or {}).items():
            (Path(directory) / name).write_bytes(value)
            if unreadable:
                (Path(directory) / name).chmod(0)
        env = dict(PATH='/usr/bin:/bin', HOME=directory, LC_ALL='C', TERM='dumb', LESSHISTFILE='-',
                   **INSTRUMENTATION)
        if set(lesskey_env or {}) - {'LESSKEY_CONTENT', 'LESSKEY_CONTENT_SYSTEM'}:
            raise ValueError('unexpected lesskey environment input')
        env.update(lesskey_env or {})
        if more_mode:
            env['LESS_IS_MORE'] = '1'
        if message_content is not None:
            message_path = Path(directory) / 'messages'
            message_path.write_text(message_content)
            env['LESSMSG'] = str(message_path)
        if plan:
            env.update(RLESS_FAULT=plan, RLESS_FAULT_FD=str(write_fd))
        if native_short:
            env.update(LD_PRELOAD=str(SHORT_SHIM), RLESS_NATIVE_SHORT_WRITE_FD=str(write_fd),
                       RLESS_NATIVE_SHORT_WRITE_AT=str(int(native_short)))
        if native_plan:
            if IO_SHIM is None:
                raise RuntimeError('native I/O counterpart requires --native-io-shim')
            site, ordinal = native_plan.split(':')
            env.update(LD_PRELOAD=str(IO_SHIM), RLESS_NATIVE_IO_SITE=site,
                       RLESS_NATIVE_IO_ORDINAL=ordinal, RLESS_NATIVE_IO_FD=str(write_fd))
        if native_alloc_plan:
            if ALLOC_BINDING is None:
                raise RuntimeError('native allocation counterpart requires --native-alloc-binding')
            from native_alloc import plan_environment
            site, ordinal = native_alloc_plan.split(':')
            env.update(plan_environment(ALLOC_BINDING, site, int(ordinal), write_fd))
            binary = Path(ALLOC_BINDING['binary'])
        command = [str(binary), *argv]
        input_file = None
        if seekable_stdin:
            input_file = handles.enter_context(tempfile.TemporaryFile())
            input_file.write(payload)
            input_file.seek(0)
        if unbounded_stdin and (seekable_stdin or not payload):
            raise ValueError('unbounded input needs a nonempty pipe payload')
        full_output = handles.enter_context(open('/dev/full', 'wb', buffering=0)) if stdout_full else None
        full_error = handles.enter_context(open('/dev/full', 'wb', buffering=0)) if stderr_full else None
        process = subprocess.Popen(command, env=env, cwd=directory,
            stdin=input_file if seekable_stdin else subprocess.PIPE,
            stdout=full_output or subprocess.PIPE, stderr=full_error or subprocess.PIPE,
            pass_fds=[write_fd], start_new_session=True)
        produced = [0]
        producer_broken_pipe = [False]
        producer = None
        if unbounded_stdin:
            # This producer never sends EOF voluntarily. Pager failure must close
            # its input; a finite communicate(payload) could hide an endless read.
            def produce():
                try:
                    with process.stdin:
                        while True:
                            produced[0] += os.write(process.stdin.fileno(), payload)
                except BrokenPipeError:
                    producer_broken_pipe[0] = True
            producer = threading.Thread(target=produce, daemon=True)
            producer.start()
        try:
            if producer is not None:
                process.wait(timeout=5)
                producer.join(timeout=2)
                if producer.is_alive():
                    raise ValueError('unbounded input producer survived pager exit')
                process.stdin = None
            stdout, stderr = process.communicate(None if seekable_stdin or unbounded_stdin else payload, timeout=5)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            if process.poll() is None:
                process.kill()
            if producer is not None:
                producer.join(timeout=2)
                process.stdin = None
            try:
                process.communicate(timeout=2)
            except subprocess.TimeoutExpired:
                # A detached helper must not make cleanup wait forever on pipes.
                for stream in (process.stdin, process.stdout, process.stderr):
                    if stream is not None:
                        stream.close()
                process.wait(timeout=2)
            raise
        result = subprocess.CompletedProcess(command, process.returncode, stdout or b'', stderr or b'')
        finish_process_group(process.pid)
        group_empty = True
        trace = read_trace(reader, writer)
    OBSERVATIONS.append(dict(kind='pipe', binary=str(binary), plan=plan, native_plan=native_plan,
                             native_alloc_plan=native_alloc_plan,
                             lesskey_env=lesskey_env or {}, more_mode=more_mode,
                             message_content_sha256=(hashlib.sha256(message_content.encode()).hexdigest()
                                                     if message_content is not None else None),
                             input_kind='unbounded-pipe' if unbounded_stdin else ('seekable-stdin' if seekable_stdin else 'pipe'),
                             producer_bytes=produced[0], producer_stopped=producer is not None and not producer.is_alive(),
                             producer_broken_pipe=producer_broken_pipe[0],
                             stdout_full=stdout_full, stderr_full=stderr_full,
                             trace_hex=trace.hex(), status=result.returncode,
                             process_reaped=process.returncode is not None, process_group_empty=group_empty,
                             input_sha256=hashlib.sha256(payload).hexdigest(),
                             stdout_sha256=hashlib.sha256(result.stdout).hexdigest(),
                             stderr_hex=result.stderr.hex()))
    return result, trace


def pty_run(binary, case, plan=None, native=False, native_plan=None):
    callback_binding = None
    with trace_channel() as (reader, writer):
        write_fd = writer.fileno()
        case = copy.deepcopy(case)
        # run_v2 forks/execs PTY children. Only this borrowed writer crosses exec.
        os.set_inheritable(write_fd, True)
        env = case.setdefault('env', {})
        env.update(INSTRUMENTATION)
        for event in case.get('events', []):
            if 'send' in event:
                event['send'] = event['send'].replace('{TRACE_FD}', str(write_fd))
        if plan:
            env.update(RLESS_FAULT=plan, RLESS_FAULT_FD=str(write_fd))
        if native:
            env.update(LD_PRELOAD=str(SHIM), RLESS_NATIVE_CLEANUP_FD=str(write_fd),
                       RLESS_NATIVE_CLEANUP_AT=str(int(native)))
        if native_plan:
            if IO_SHIM is None:
                raise RuntimeError('native I/O counterpart requires --native-io-shim')
            site, ordinal = native_plan.split(':')
            env.update(LD_PRELOAD=str(IO_SHIM), RLESS_NATIVE_IO_SITE=site,
                       RLESS_NATIVE_IO_ORDINAL=ordinal, RLESS_NATIVE_IO_FD=str(write_fd))
            if site == 'callback':
                from native_callback import bind
                callback_binding = bind(binary)
                env['RLESS_NATIVE_CALLBACK_OFFSET'] = str(callback_binding['offset'])
        observed = run_case(binary, case)
        trace = read_trace(reader, writer)
    OBSERVATIONS.append(dict(kind='pty', binary=str(binary), plan=plan, native_plan=native_plan,
                             native_callback_binding=callback_binding,
                             trace_hex=trace.hex(), status=observed['status'],
                             termios_restored=observed['termios_restored'],
                             resources=observed.get('resource_checkpoints', []),
                             observation_sha256=hashlib.sha256(json.dumps(observed, sort_keys=True).encode()).hexdigest()))
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


def resource_identity(checkpoint, allow_input_switch=False):
    """Keep descriptor kinds and multiplicity across input worker restarts."""
    descriptors = checkpoint['descriptors']
    if sum(name.startswith('socket:[') for name in descriptors) != 1:
        raise ValueError('expected one active input-worker socket')
    return sorted(
        '{INPUT}' if allow_input_switch and name in ('{ROOT}/i', '{ROOT}/j')
        else '{INPUT_SOCKET}' if name.startswith('socket:[')
        else name for name in descriptors
    )


def input_worker_children(checkpoint):
    """One active path-backed input owns one worker; no helper may accumulate."""
    children = checkpoint.get('children')
    return (isinstance(children, list) and len(children) == 1
            and isinstance(children[0], str) and children[0].isdigit())


class ActualFaultPaths(unittest.TestCase):
    def test_read_eintr_and_short_reads_recover_exact_bytes(self):
        for site, site_id in [('read-eintr', 14), ('read-short', 15)]:
            for ordinal in (1, 2):
                payload = b'payload\n' * 4096
                result, trace = pipe_run(INJECTED, f'{site}:{ordinal}', payload=payload)
                self.assertEqual(trace, marker(site_id, ordinal))
                self.assertEqual((result.returncode, result.stdout, result.stderr), (0, payload, b''))
                native, native_trace = pipe_run(NORMAL, native_plan=f'{site}:{ordinal}', payload=payload)
                self.assertEqual(native_trace, marker(site_id, ordinal, b'RNT1'))
                self.assertEqual((native.returncode, native.stdout, native.stderr), (0, payload, b''))

    def test_read_error_reports_failure_without_discarding_status(self):
        result, trace = pipe_run(INJECTED, 'read:1')
        self.assertEqual(trace, marker(2))
        self.assertEqual(result.stdout, b'')
        self.assertEqual((result.returncode, result.stderr), (1, b'rless: read error\n'))

    def test_native_io_errors_match_rust_hooks_and_document_legacy_divergence(self):
        if REFERENCE is None:
            self.fail('native error oracle requires --reference')
        for site, site_id in [('read', 2), ('write', 4)]:
            observed = []
            for binary in (NORMAL, REFERENCE):
                control, trace = pipe_run(binary)
                self.assertEqual((control.returncode, control.stdout, control.stderr, trace),
                                 (0, b'payload\n', b'', b''))
                failed, trace = pipe_run(binary, native_plan=f'{site}:1')
                self.assertEqual(trace, marker(site_id, magic=b'RNT1'))
                observed.append((failed.returncode, failed.stdout, failed.stderr))
            self.assertEqual(observed[1], (0, b'', b''))  # retained C defect
            self.assertEqual(observed[0], (1, b'', f'rless: {site} error\n'.encode()))
            injected, trace = pipe_run(INJECTED, f'{site}:1')
            self.assertEqual(trace, marker(site_id))
            self.assertEqual((injected.returncode, injected.stdout, injected.stderr), observed[0])

    def test_second_read_and_write_ordinals_are_distinct_native_errors(self):
        payload = bytes(range(256)) * 1024
        # Named files undergo a direct binary-detection read before the runtime
        # boundary. A regular-file stdin skips that probe and makes both the
        # syscall and injected ordinals count the same deterministic reads.
        for site, site_id in [('read', 2), ('write', 4)]:
            for ordinal in (1, 2):
                actual, trace = pipe_run(INJECTED, f'{site}:{ordinal}', payload=payload, seekable_stdin=True)
                native, native_trace = pipe_run(NORMAL, payload=payload, seekable_stdin=True,
                                               native_plan=f'{site}:{ordinal}')
                self.assertEqual(trace, marker(site_id, ordinal))
                self.assertEqual(native_trace, marker(site_id, ordinal, b'RNT1'))
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                                 (native.returncode, native.stdout, native.stderr))
                self.assertEqual((actual.returncode, actual.stderr),
                                 (1, f'rless: {site} error\n'.encode()))
                width = 8192 if site == 'read' else 1023
                self.assertEqual(actual.stdout, payload[:width * (ordinal - 1)])

    def test_named_input_probe_read_error_is_not_lost_by_later_success(self):
        payload = b'line\n' * 400
        for ordinal in (1, 2):
            actual, trace = pipe_run(NORMAL, argv=['i'], files={'i': payload},
                                     native_plan=f'read:{ordinal}')
            self.assertEqual(trace, marker(2, ordinal, b'RNT1'))
            self.assertEqual(actual.returncode, 1)
            self.assertEqual(actual.stderr, b'rless: read error\n')
            self.assertEqual(actual.stdout, payload if ordinal == 1 else b'')
        case = pager()
        case['include_transcript'] = True
        interactive, trace = pty_run(NORMAL, case, native_plan='read:1')
        self.assertEqual(trace, marker(2, 1, b'RNT1'))
        self.assertEqual(interactive['status'], 1)
        self.assertTrue(interactive['termios_restored'])
        self.assertIn(b'rless: read error', base64.b64decode(interactive['transcript_b64']))

    def test_write_error_reports_failure_without_discarding_status(self):
        result, trace = pipe_run(INJECTED, 'write:1')
        self.assertEqual(trace, marker(4))
        self.assertEqual((result.returncode, result.stdout, result.stderr),
                         (1, b'', b'rless: write error\n'))

    def test_read_failure_survives_later_successful_file(self):
        for binary, plan, native in ((INJECTED, 'read:1', None), (NORMAL, None, 'read:1')):
            actual, trace = pipe_run(binary, plan, native_plan=native,
                argv=['-', 'j'], payload=b'first input\n', files={'j': b'later input\n'}, seekable_stdin=True)
            self.assertEqual(trace, marker(2, magic=b'RNT1' if native else b'RLF1'))
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (1, b'later input\n', b'rless: read error\n'))

    def test_write_failure_stops_unbounded_input_and_broken_stderr(self):
        for binary, plan, native in ((INJECTED, 'write:1', None), (NORMAL, None, 'write:1')):
            actual, trace = pipe_run(binary, plan, native_plan=native,
                                    payload=b'x' * 4096, unbounded_stdin=True)
            self.assertEqual(trace, marker(4, magic=b'RNT1' if native else b'RLF1'))
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (1, b'', b'rless: write error\n'))
            self.assertTrue(OBSERVATIONS[-1]['producer_stopped'])
            self.assertLess(OBSERVATIONS[-1]['producer_bytes'], 1024 * 1024)
        # Natural permanent ENOSPC, including an independently broken diagnostic
        # stream. Neither case relies on a one-shot injection becoming writable.
        for broken_stderr in (False, True):
            actual, trace = pipe_run(NORMAL, payload=b'x' * 4096, unbounded_stdin=True,
                                    stdout_full=True, stderr_full=broken_stderr)
            self.assertEqual(trace, b'')
            self.assertEqual(actual.returncode, 1)
            self.assertEqual(actual.stderr, b'' if broken_stderr else b'rless: write error\n')
            self.assertTrue(OBSERVATIONS[-1]['producer_stopped'])
            self.assertLess(OBSERVATIONS[-1]['producer_bytes'], 1024 * 1024)

    def test_final_flush_failure_changes_success_and_preserves_primary_status(self):
        # --version buffers less than 1023 bytes then calls quit(0); its first
        # actual write is quit's flush, with no cat_file loop to propagate it.
        control, trace = pipe_run(NORMAL, argv=['--version'], payload=b'')
        self.assertEqual((control.returncode, control.stderr, trace), (0, b'', b''))
        self.assertLess(len(control.stdout), 1023)
        self.assertGreater(len(control.stdout), 0)
        for binary, plan, native in ((INJECTED, 'write:1', None), (NORMAL, None, 'write:1')):
            actual, trace = pipe_run(binary, plan, native_plan=native, argv=['--version'], payload=b'')
            self.assertEqual(trace, marker(4, magic=b'RNT1' if native else b'RLF1'))
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (1, b'', b'rless: write error\n'))
        # The initial prompt flush succeeds. The next nonempty flush is terminal
        # teardown; verify both the primary status and real termios restoration.
        case = pager([{'await': ['READY'], 'snapshot': True}, {'signal': 'SIGTERM'}])
        baseline, trace = pty_run(NORMAL, case)
        self.assertEqual((baseline['status'], trace), (15, b''))
        for binary, plan, native in ((INJECTED, 'write:2', None), (NORMAL, None, 'write:2')):
            actual, trace = pty_run(binary, case, plan, native_plan=native)
            self.assertEqual(trace, marker(4, 2, b'RNT1' if native else b'RLF1'))
            self.assertEqual(actual['status'], 15)
            self.assertTrue(actual['termios_restored'])
            self.assertEqual(actual['checkpoints'][0], baseline['checkpoints'][0])

    def test_pipe_failure_termination_reaps_process_and_child_group(self):
        payload = b'payload\n' * 4096
        for site, site_id in [('alloc', 1), ('read', 2), ('write', 4), ('reserve', 12),
                              ('read-eintr', 14), ('read-short', 15), ('write-short', 16)]:
            argv = ['--lesskey-content=#env\nLESS=-N'] if site == 'reserve' else []
            actual, trace = pipe_run(INJECTED, site + ':1', argv=argv, payload=payload)
            self.assertEqual(trace, marker(site_id))
            observation = OBSERVATIONS[-1]
            self.assertTrue(observation['process_reaped'])
            self.assertTrue(observation['process_group_empty'])
            if site in ('alloc', 'reserve'):
                self.assertEqual(actual.returncode, 1)
                self.assertIn(b'Cannot allocate memory', actual.stdout + actual.stderr)
            elif site in ('read', 'write'):
                native, native_trace = pipe_run(NORMAL, native_plan=site + ':1', payload=payload)
                self.assertEqual(native_trace, marker(site_id, magic=b'RNT1'))
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                                 (native.returncode, native.stdout, native.stderr))
                self.assertEqual((actual.returncode, actual.stderr),
                                 (1, f'rless: {site} error\n'.encode()))
                self.assertLess(len(actual.stdout), len(payload))
            else:
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr), (0, payload, b''))

    def test_native_write_eintr_retries_exact_remaining_bytes(self):
        payload = b'interrupted output\n' * 8192
        for ordinal in (1, 2):
            actual, trace = pipe_run(NORMAL, native_plan=f'write-eintr:{ordinal}', payload=payload)
            self.assertEqual(trace, marker(4, ordinal, b'RNT1'))
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr), (0, payload, b''))

    def test_short_write_retries_the_exact_remaining_bytes(self):
        payload = b'payload\n' * 16384
        for ordinal in (1, 2):
            for binary, plan, native in ((INJECTED, f'write-short:{ordinal}', False), (NORMAL, None, ordinal)):
                result, trace = pipe_run(binary, plan, argv=['i'], files={'i': payload}, native_short=native)
                self.assertEqual(trace, marker(16, ordinal, magic=b'RNT1' if native else b'RLF1'))
                self.assertEqual((result.returncode, result.stdout, result.stderr), (0, payload, b''))

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

    def test_scoped_native_allocation_and_growth_failures_match_pager_oom(self):
        if ALLOC_BINDING is None:
            self.fail('scoped native allocation test requires --native-alloc-binding')
        counterpart = Path(ALLOC_BINDING['binary'])
        for site, site_id in [('alloc', 1), ('reserve', 12)]:
            argv = (['--cmd=12g', '--cmd=1g', 'i'] if site == 'alloc' else
                    ['--lesskey-content=#env\nA=' + 'x' * 256 + '\nB=' + 'y' * 256, 'i'])
            files = {'i': b'payload\n'}
            baseline, control_trace = pipe_run(NORMAL, argv=argv, files=files)
            unfaulted, absent_trace = pipe_run(counterpart, argv=argv, files=files)
            self.assertEqual((control_trace, absent_trace), (b'', b''))
            self.assertEqual((unfaulted.returncode, unfaulted.stdout, unfaulted.stderr),
                             (baseline.returncode, baseline.stdout, baseline.stderr))
            for ordinal in (1, 2):
                actual, trace = pipe_run(INJECTED, f'{site}:{ordinal}', argv=argv, files=files)
                native, native_trace = pipe_run(counterpart, native_alloc_plan=f'{site}:{ordinal}',
                                                argv=argv, files=files)
                self.assertEqual(trace, marker(site_id, ordinal))
                self.assertEqual(native_trace, marker(site_id, ordinal, b'RNT1'))
                self.assertEqual(actual.returncode, 1)
                self.assertIn(b'Cannot allocate memory', actual.stdout + actual.stderr)
                self.assertEqual((native.returncode, native.stdout, native.stderr),
                                 (actual.returncode, actual.stdout, actual.stderr))
                self.assertTrue(OBSERVATIONS[-1]['process_reaped'])
                self.assertTrue(OBSERVATIONS[-1]['process_group_empty'])

    def test_open_permission_failure_matches_real_native_denial(self):
        actual, trace = pipe_run(INJECTED, 'open:1', argv=['i'], files={'i': b'payload\n'})
        native, _ = pipe_run(NORMAL, argv=['i'], files={'i': b'payload\n'}, unreadable=True)
        self.assertEqual(trace, marker(3))
        self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                         (native.returncode, native.stdout, native.stderr))
        self.assertEqual(actual.returncode, 1)
        self.assertIn(b'Permission denied', actual.stderr)

    def test_native_open_and_close_counterparts_have_exact_observed_ordinals(self):
        for ordinal in (1, 2):
            actual, trace = pipe_run(INJECTED, f'open:{ordinal}', argv=['i', 'j'],
                                     files={'i': b'first\n', 'j': b'second\n'})
            native, native_trace = pipe_run(NORMAL, native_plan=f'open:{ordinal}', argv=['i', 'j'],
                                            files={'i': b'first\n', 'j': b'second\n'})
            self.assertEqual(trace, marker(3, ordinal))
            self.assertEqual(native_trace, marker(3, ordinal, b'RNT1'))
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (native.returncode, native.stdout, native.stderr))
        case = pager()
        actual, trace = pty_run(INJECTED, case, 'close:1')
        native, native_trace = pty_run(NORMAL, case, native_plan='close:1')
        self.assertEqual(trace, marker(13))
        self.assertEqual(native_trace, marker(13, magic=b'RNT1'))
        self.assertEqual(actual, native)

    def test_native_regex_and_callback_counterparts_preserve_observation(self):
        case = pager([{'await': ['READY']}, {'send': '/keep\n', 'await': ['Invalid pattern']},
                      {'send': '\n', 'await': ['READY']}, {'send': '/keep\n', 'await': ['READY'], 'snapshot': True},
                      {'send': 'q'}])
        actual, trace = pty_run(INJECTED, case, 'regex:1')
        native, native_trace = pty_run(NORMAL, case, native_plan='regex:1')
        self.assertEqual(trace, marker(5))
        self.assertEqual(native_trace, marker(5, magic=b'RNT1'))
        self.assertEqual(actual, native)
        for ordinal in (1, 2):
            actual, trace = pty_run(INJECTED, pager(), f'callback:{ordinal}')
            native, native_trace = pty_run(NORMAL, pager(), native_plan=f'callback:{ordinal}')
            self.assertEqual(trace, marker(10, ordinal))
            self.assertEqual(native_trace, marker(10, ordinal, b'RNT1'))
            self.assertEqual(actual, native)

    def test_duplicate_message_and_help_definitions_release_previous_overrides(self):
        for content, argv in (
            ('No_such_file \n', []),
            ('No_such_file FIRST\n', []),
            ('No_such_file FIRST\nNo_such_file SECOND\n', []),
            ('HELP \n', ['--help']),
            ('HELP FIRST\nHELP SECOND\n', ['--help']),
        ):
            baseline, trace = pipe_run(REFERENCE, argv=argv, message_content=content)
            self.assertEqual((baseline.returncode, baseline.stderr, trace), (0, b'', b''))
            if content == 'HELP \n':
                self.assertEqual(baseline.stdout, b'')
            elif argv:
                self.assertIn(b'SECOND', baseline.stdout)
                self.assertNotIn(b'FIRST', baseline.stdout)
            for binary in (NORMAL, INJECTED):
                actual, trace = pipe_run(binary, argv=argv, message_content=content)
                self.assertEqual(trace, b'')
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                                 (baseline.returncode, baseline.stdout, baseline.stderr))

    def test_duplicate_startup_options_release_previous_saved_strings(self):
        cases = [
            (False, ['-oone']), (False, ['-oone', '-otwo']),
            (False, ['-tone']), (False, ['-tone', '-ttwo']),
            (False, ['-Tone']), (False, ['-Tone', '-Ttwo']),
            (False, ['--header=1']), (False, ['--header=1', '--header=2']),
            (True, ['-pone']), (True, ['-pone', '-ptwo']),
            (True, ['++one', '-ptwo']), (True, ['-pone', '++two']),
        ]
        for more_mode, argv in cases:
            baseline, trace = pipe_run(REFERENCE, argv=argv, more_mode=more_mode)
            self.assertEqual((baseline.returncode, baseline.stdout, baseline.stderr, trace),
                             (0, b'payload\n', b'', b''))
            for binary in (NORMAL, INJECTED):
                actual, trace = pipe_run(binary, argv=argv, more_mode=more_mode)
                self.assertEqual(trace, b'')
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                                 (baseline.returncode, baseline.stdout, baseline.stderr))

    def test_stdin_tags_reset_releases_saved_filename(self):
        for argv in (['-tfoo'], ['-T-', '-tfoo']):
            case = dict(id='TAGS-OWNER-RESET', mode='pty', argv=argv,
                        stream_b64='', events=[{'close_stream': True}])
            baseline, trace = pty_run(REFERENCE, case)
            self.assertEqual((baseline['status'], baseline['termios_restored'], trace),
                             (1, True, b''))
            for binary in (NORMAL, INJECTED):
                actual, trace = pty_run(binary, case)
                self.assertEqual(trace, b'')
                self.assertEqual(actual, baseline)

    def test_stop_directive_releases_discarded_command_table_owners(self):
        # Exercise the default borrowed table, a parsed predecessor and a
        # parsed successor to the first stop table. ASAN/LSAN diagnostics are
        # captured and rejected by the runner even when the bytes match.
        cases = [
            {},
            {'LESSKEY_CONTENT': '#stop'},
            {'LESSKEY_CONTENT_SYSTEM': 'x quit\n', 'LESSKEY_CONTENT': '#stop'},
            {'LESSKEY_CONTENT_SYSTEM': '#stop', 'LESSKEY_CONTENT': 'x quit\n'},
        ]
        for config in cases:
            for binary in (NORMAL, INJECTED):
                actual, trace = pipe_run(binary, lesskey_env=config)
                self.assertEqual(trace, b'')
                self.assertEqual((actual.returncode, actual.stdout, actual.stderr), (0, b'payload\n', b''))

    def test_parser_failure_then_valid_content_is_usable(self):
        result, trace = pipe_run(INJECTED, 'parser:1', argv=[
            '--lesskey-content=#env\nLESS=-N', '--lesskey-content=#env\nLESS=-S'])
        self.assertEqual(trace, marker(6))
        self.assertEqual((result.returncode, result.stdout), (0, b'payload\n'))
        self.assertEqual(result.stderr, b'Error in lesskey content\n')

    def test_native_handoff_failure_skips_exact_child_and_recovers(self):
        for ordinal in (1, 2):
            events = [{'await': ['READY'], 'resources': True}]
            for number in range(1, 4):
                events.extend([{'send': f'!printf child{number} > child{number}.out\n', 'await': ['!done']},
                               {'send': '\n', 'await': ['READY'], 'resources': True}])
            events.append({'send': 'q'})
            case = pager(events)
            actual, trace = pty_run(INJECTED, case, f'handoff:{ordinal}')
            native, native_trace = pty_run(NORMAL, case, native_plan=f'handoff:{ordinal}')
            self.assertEqual(trace, marker(9, ordinal))
            self.assertEqual(native_trace, marker(9, ordinal, b'RNT1'))
            # Descriptor path strings include per-process pipe identities. The
            # effects/screens/status compare exactly; resources compare within
            # each live process and by cardinality between the two runs.
            for observed in (actual, native):
                checkpoints = observed.pop('resource_checkpoints')
                self.assertTrue(checkpoints)
                for checkpoint in checkpoints:
                    self.assertTrue(input_worker_children(checkpoint))
                    self.assertEqual(resource_identity(checkpoint), resource_identity(checkpoints[0]))
                self.assertNotIn(f'child{ordinal}.out', observed['files'])
                for number in set(range(1, 4)) - {ordinal}:
                    self.assertEqual(observed['files'][f'child{number}.out']['sha256'],
                                     hashlib.sha256(f'child{number}'.encode()).hexdigest())
            self.assertEqual(actual, native)

    def test_parser_partial_staging_failure_releases_each_table_before_retry(self):
        staged = 'q quit\n#line-edit\n\\b backspace\n#env\nLESS=-N\nLAST=value\n'
        for ordinal in (2, 4, 6):
            result, trace = pipe_run(INJECTED, f'parser:{ordinal}', argv=[
                '--lesskey-content=' + staged, '--lesskey-content=#env\nLESS=-S'])
            self.assertEqual(trace, marker(6, ordinal))
            self.assertEqual((result.returncode, result.stdout), (0, b'payload\n'))
            self.assertEqual(result.stderr, b'Error in lesskey content\n')

    def test_natural_parser_rejection_after_staging_then_valid_retry(self):
        rejected = ('z quit\n#line-edit\n\\b backspace\n#env\nRLESS_STAGED=value\n'
                    '#command\nq nonexistent-rless-test-action\n')
        for binary in (NORMAL, INJECTED):
            result, trace = pipe_run(binary, argv=[
                '--lesskey-content=' + rejected, '--lesskey-content=#env\nLESS=-S'])
            self.assertEqual(trace, b'')
            self.assertEqual((result.returncode, result.stdout), (0, b'payload\n'))
            self.assertIn(b'Error in lesskey content', result.stderr)

        # Make non-publication and the later valid parse observable in the same
        # live pager. Rejected z->quit must be inert; accepted x->quit must work.
        rejected_pty = ('z quit\n#line-edit\n\\b backspace\n#env\nLESS=-N\n'
                        '#command\nq nonexistent-rless-test-action\n')
        case = pager([{'await': ['Press RETURN']},
            {'send': '\n', 'await': ['READY'], 'snapshot': True},
            {'send': 'z', 'await': ['READY'], 'snapshot': True}, {'send': 'x'}],
            options=['--lesskey-content=' + rejected_pty, '--lesskey-content=x quit\n'])
        for binary in (NORMAL, INJECTED):
            actual, trace = pty_run(binary, case)
            self.assertEqual((actual['status'], actual['termios_restored'], trace), (0, True, b''))
            self.assertEqual(len(actual['checkpoints']), 2)
            self.assertTrue(actual['checkpoints'][0]['rows'][0].startswith('keep 000'))

    def test_parser_preinit_failure_preserves_previously_published_buffers(self):
        # The later expand_cmd_tables traverses the first published buffer.
        # ASAN must reject freeing it from a subsequent pre-init parse failure.
        if ALLOC_BINDING is None:
            self.fail('pre-init parser failure requires the scoped native binding')
        argv = ['--lesskey-content=z repaint', '--lesskey-src=rless-early-config-failure',
                '--lesskey-content=q quit']
        baseline, absent = pipe_run(NORMAL, argv=argv)
        result, trace = pipe_run(Path(ALLOC_BINDING['binary']), native_alloc_plan='parser:1', argv=argv)
        self.assertEqual(absent, b'')
        self.assertEqual(trace, marker(6, magic=b'RNT1'))
        self.assertEqual((result.returncode, result.stdout), (0, b'payload\n'))
        self.assertIn(b'Cannot use lesskey source file', result.stderr)
        self.assertEqual((result.returncode, result.stdout, result.stderr),
                         (baseline.returncode, baseline.stdout, baseline.stderr))

    def test_natural_filter_rejection_preserves_committed_filter_then_clear(self):
        case = pager([{'await': ['READY'], 'resources': True},
            {'send': '&keep\n', 'await': ['READY']}, {'send': 'g', 'await': ['READY'], 'snapshot': True},
            {'send': '&[\n', 'await': ['Invalid pattern']},
            {'send': '\n', 'await': ['READY']}, {'send': 'g', 'await': ['READY'], 'snapshot': True, 'resources': True},
            {'send': '&\n', 'await': ['READY']}, {'send': 'g', 'await': ['READY'], 'snapshot': True, 'resources': True},
            {'send': 'q'}])
        for binary in (NORMAL, INJECTED):
            actual, trace = pty_run(binary, case)
            self.assertEqual(trace, b'')
            self.assertEqual(actual['status'], 0)
            self.assertTrue(actual['termios_restored'])
            before, rejected, cleared = ['\n'.join(item['rows'][:-1]) for item in actual['checkpoints']]
            self.assertIn('keep 000', before)
            self.assertNotIn('DROP 000', before)
            self.assertIn('keep 000', rejected)
            self.assertNotIn('DROP 000', rejected)
            self.assertIn('DROP 000', cleared)
            resources = actual['resource_checkpoints']
            for checkpoint in resources:
                self.assertEqual(resource_identity(checkpoint), resource_identity(resources[0]))
                self.assertTrue(input_worker_children(checkpoint))

    def test_repeated_file_and_child_failures_conserve_live_process_resources(self):
        events = [{'await': ['READY'], 'resources': True}]
        for _ in range(REPETITIONS):
            events.extend([
                {'send': ':e missing-input\n', 'await': ['No such file or directory']},
                {'send': '\n', 'await': ['READY'], 'resources': True},
                {'send': '!/nonexistent-rless-test-command\n', 'await': ['!done']},
                {'send': '\n', 'await': ['READY'], 'resources': True},
            ])
        events.append({'send': 'q'})
        for binary in (NORMAL, INJECTED):
            actual, trace = pty_run(binary, pager(events))
            self.assertEqual(trace, b'')
            self.assertTrue(actual['termios_restored'])
            self.assertEqual(actual['status'], 0)
            checkpoints = actual['resource_checkpoints']
            self.assertEqual(len(checkpoints), 1 + 2 * REPETITIONS)
            expected_fds = len(checkpoints[0]['descriptors'])
            for checkpoint in checkpoints:
                self.assertEqual(len(checkpoint['descriptors']), expected_fds)
                self.assertEqual(resource_identity(checkpoint), resource_identity(checkpoints[0]))
                self.assertTrue(input_worker_children(checkpoint))

    def test_reserve_failure_before_parser_growth_has_controlled_oom(self):
        result, trace = pipe_run(INJECTED, 'reserve:1', argv=['--lesskey-content=#env\nLESS=-N'])
        self.assertEqual(trace, marker(12))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, b'')
        self.assertIn(b'Cannot allocate memory', result.stderr)

    def test_injected_recovery_conserves_descriptors_and_children_in_live_pager(self):
        for site, site_id in [('regex', 5), ('filter-before', 7), ('filter-after', 8),
                              ('handoff', 9), ('cleanup', 11), ('close', 13)]:
            def make_case(injected):
                events = [{'await': ['READY'], 'resources': True}]
                if site == 'regex':
                    events.append({'send': '/keep\n', 'await': ['Invalid pattern' if injected else 'READY']})
                    if injected:
                        events.append({'send': '\n', 'await': ['READY'], 'resources': True})
                    cycle = [{'send': '/keep\n', 'await': ['READY'], 'resources': True}]
                elif site.startswith('filter'):
                    message = 'Injected filter commit failure' if site == 'filter-before' else 'Injected post-commit failure'
                    events.append({'send': '&keep\n', 'await': [message if injected else 'READY']})
                    if injected:
                        events.append({'send': '\n', 'await': ['READY'], 'resources': True})
                    cycle = [{'send': '&\n', 'await': ['READY'], 'resources': True},
                             {'send': '&keep\n', 'await': ['READY'], 'resources': True}]
                elif site == 'close':
                    cycle = [{'send': ':n\n', 'await': ['READY'], 'resources': True},
                             {'send': ':p\n', 'await': ['READY'], 'resources': True}]
                else:
                    cycle = [{'send': '!true\n', 'await': ['!done']},
                             {'send': '\n', 'await': ['READY'], 'resources': True}]
                for _ in range(REPETITIONS):
                    events.extend(copy.deepcopy(cycle))
                events.append({'send': 'q'})
                case = pager(events)
                if site == 'close':
                    case['argv'].append('j')
                    case['files']['j'] = {'text': 'second\n' * 100}
                return case
            baseline, control_trace = pty_run(NORMAL, make_case(False))
            actual, trace = pty_run(INJECTED, make_case(True), site + ':1')
            self.assertEqual(control_trace, b'')
            self.assertEqual(trace, marker(site_id))
            expected_count = len(baseline['resource_checkpoints'][0]['descriptors'])
            for observed in (baseline, actual):
                self.assertEqual(observed['status'], 0)
                self.assertTrue(observed['termios_restored'])
                self.assertGreaterEqual(len(observed['resource_checkpoints']), REPETITIONS + 1)
                for checkpoint in observed['resource_checkpoints']:
                    self.assertEqual(len(checkpoint['descriptors']), expected_count)
                    self.assertTrue(input_worker_children(checkpoint))
                    self.assertEqual(resource_identity(checkpoint, site == 'close'),
                                     resource_identity(observed['resource_checkpoints'][0], site == 'close'))

    def test_callback_startup_failure_preserves_later_session_lifecycle(self):
        # cost() runs during terminal setup, before the first input barrier.
        # This is a startup/later-use alternate, not a before/after-fault sample.
        events = [{'await': ['READY'], 'resources': True}]
        for _ in range(REPETITIONS):
            events.extend([{'send': '!true\n', 'await': ['!done']},
                           {'send': '\n', 'await': ['READY'], 'resources': True}])
        events.append({'send': 'q'})
        baseline, control_trace = pty_run(NORMAL, pager(events))
        actual, trace = pty_run(INJECTED, pager(events), 'callback:1')
        self.assertEqual(control_trace, b'')
        self.assertEqual(trace, marker(10))
        for observed in (baseline, actual):
            checkpoints = observed.pop('resource_checkpoints')
            self.assertEqual(len(checkpoints), REPETITIONS + 1)
            for checkpoint in checkpoints:
                self.assertTrue(input_worker_children(checkpoint))
                self.assertEqual(resource_identity(checkpoint), resource_identity(checkpoints[0]))
            self.assertTrue(observed['termios_restored'])
            self.assertEqual(observed['status'], 0)
        self.assertEqual(actual, baseline)

    def test_reserve_second_growth_ordinal_has_distinct_controlled_failure(self):
        content = '#env\nA=' + 'x' * 256 + '\nB=' + 'y' * 256
        for ordinal in (1, 2):
            result, trace = pipe_run(INJECTED, f'reserve:{ordinal}', argv=['--lesskey-content=' + content])
            self.assertEqual(trace, marker(12, ordinal))
            self.assertEqual(result.returncode, 1)
            self.assertIn(b'Cannot allocate memory', result.stdout + result.stderr)

    def test_filter_handoff_close_cleanup_and_regex_second_ordinals(self):
        for site, site_id in [('filter-before', 7), ('filter-after', 8)]:
            message = 'Injected filter commit failure' if site_id == 7 else 'Injected post-commit failure'
            case = pager([{'await': ['READY']}, {'send': '&keep\n', 'await': ['READY']},
                          {'send': '&DROP\n', 'await': [message]},
                          {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
            actual, trace = pty_run(INJECTED, case, f'{site}:2')
            self.assertEqual(trace, marker(site_id, 2))
            self.assertTrue(actual['termios_restored'])
            self.assertEqual(actual['status'], 0)
        handoff = pager([{'await': ['READY']}, {'send': '!true\n', 'await': ['!done']},
                         {'send': '\n', 'await': ['READY']}, {'send': '!printf BAD > unexpected\n', 'await': ['!done']},
                         {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
        actual, trace = pty_run(INJECTED, handoff, 'handoff:2')
        self.assertEqual(trace, marker(9, 2))
        self.assertNotIn('unexpected', actual['files'])
        self.assertTrue(actual['termios_restored'])
        cleanup = pager([{'await': ['READY']}, {'send': '!true\n', 'await': ['!done']},
                         {'send': '\n', 'await': ['READY']}, {'send': 'q'}])
        baseline, _ = pty_run(NORMAL, cleanup)
        actual, trace = pty_run(INJECTED, cleanup, 'cleanup:2')
        self.assertEqual(trace, marker(11, 2))
        self.assertEqual(actual, baseline)
        native, native_trace = pty_run(NORMAL, cleanup, native=2)
        self.assertEqual(native_trace, marker(11, 2, b'RNT1'))
        self.assertEqual(native, baseline)
        switching = pager([{'await': ['READY']}, {'send': ':n\n', 'await': ['READY']}, {'send': 'q'}])
        switching['argv'].append('j')
        switching['files']['j'] = {'text': 'second\n' * 100}
        baseline, _ = pty_run(NORMAL, switching)
        actual, trace = pty_run(INJECTED, switching, 'close:2')
        self.assertEqual(trace, marker(13, 2))
        self.assertEqual(actual, baseline)
        regex = pager([{'await': ['READY']}, {'send': '/keep\n', 'await': ['READY']},
                       {'send': '/DROP\n', 'await': ['Invalid pattern']},
                       {'send': '\n', 'await': ['READY']}, {'send': '/keep\n', 'await': ['READY']}, {'send': 'q'}])
        actual, trace = pty_run(INJECTED, regex, 'regex:2')
        native, native_trace = pty_run(NORMAL, regex, native_plan='regex:2')
        self.assertEqual(trace, marker(5, 2))
        self.assertEqual(native_trace, marker(5, 2, b'RNT1'))
        self.assertEqual(actual, native)

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
        natural = pager([{'await': ['READY']}, {'send': '&keep\n', 'await': ['READY']},
            {'send': 'g', 'await': ['READY'], 'snapshot': True},
            {'send': '&\n', 'await': ['READY']},
            {'send': 'g', 'await': ['READY'], 'snapshot': True}, {'send': 'q'}])
        baseline, absent = pty_run(NORMAL, natural)
        self.assertEqual((baseline['status'], baseline['termios_restored'], absent), (0, True, b''))
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
            expected_first = baseline['checkpoints'][0 if committed else 1]['rows'][:-1]
            self.assertEqual(actual['checkpoints'][0]['rows'][:-1], expected_first)
            self.assertEqual(actual['checkpoints'][1]['rows'][:-1], baseline['checkpoints'][1]['rows'][:-1])
            self.assertEqual(actual['status'], 0)
            self.assertTrue(actual['termios_restored'])

    def test_handoff_failure_skips_child_then_recovers_and_closes_trace_on_exec(self):
        command = 'if test -e /proc/$$/fd/{TRACE_FD}; then printf LEAK; else printf CLOSED; fi > child.out'
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


def contracts():
    """Exact catalogue selectors for this bounded production-path suite.

    Scope is always partial: a concrete site/oracle does not cover every path in
    an entire acceptance variant. Metadata records what each test really checks.
    """
    def entry(sites=(), ordinals=(1,), controls=(), scenario='SAFECASE-VER-005',
              variant='all declared fault injection hooks', findings=()):
        return dict(scenario_id=scenario, variant=variant, hook_sites=list(sites),
                    hook_ordinals=list(ordinals), m0_controls=list(controls), finding_ids=list(findings))
    return {
        'test_read_eintr_and_short_reads_recover_exact_bytes': entry(('read-eintr', 'read-short'), (1, 2), ('fired', 'ordinal-separation', 'native-counterpart')),
        'test_read_error_reports_failure_without_discarding_status': entry(('read',), findings=('M0-IO-001',), controls=('fired',)),
        'test_write_error_reports_failure_without_discarding_status': entry(('write',), findings=('M0-IO-002',), controls=('fired',)),
        'test_pipe_failure_termination_reaps_process_and_child_group': entry(('alloc', 'read', 'write', 'reserve', 'read-eintr', 'read-short', 'write-short'), controls=('fired', 'termination-cleanup'), findings=('M0-IO-001', 'M0-IO-002')),
        'test_native_io_errors_match_rust_hooks_and_document_legacy_divergence': entry(('read', 'write'), controls=('fired', 'native-counterpart', 'unfaulted'), findings=('M0-IO-001', 'M0-IO-002')),
        'test_second_read_and_write_ordinals_are_distinct_native_errors': entry(('read', 'write'), (1, 2), ('fired', 'native-counterpart', 'ordinal-separation'), findings=('M0-IO-001', 'M0-IO-002')),
        'test_named_input_probe_read_error_is_not_lost_by_later_success': entry(('read',), (1, 2), ('fired', 'native-counterpart', 'ordinal-separation'), findings=('M0-IO-001',)),
        'test_read_failure_survives_later_successful_file': entry(('read',), controls=('fired', 'native-counterpart'), findings=('M0-IO-001',)),
        'test_write_failure_stops_unbounded_input_and_broken_stderr': entry(('write',), controls=('fired', 'native-counterpart', 'termination-cleanup'), findings=('M0-IO-002',)),
        'test_final_flush_failure_changes_success_and_preserves_primary_status': entry(('write',), (1, 2), ('fired', 'native-counterpart'), findings=('M0-IO-002',)),
        'test_native_write_eintr_retries_exact_remaining_bytes': entry(('write',), (1, 2)),
        'test_short_write_retries_the_exact_remaining_bytes': entry(('write-short',), (1, 2), ('fired', 'native-counterpart', 'ordinal-separation')),
        'test_saved_command_allocation_failures_have_exact_hit_and_status': entry(('alloc',), tuple(range(1, 25)), ('fired', 'ordinal-separation')),
        'test_scoped_native_allocation_and_growth_failures_match_pager_oom': entry(('alloc', 'reserve'), (1, 2), ('fired', 'native-counterpart', 'ordinal-separation', 'unfaulted', 'termination-cleanup')),
        'test_open_permission_failure_matches_real_native_denial': entry(('open',), controls=('fired', 'native-counterpart')),
        'test_native_open_and_close_counterparts_have_exact_observed_ordinals': entry(('open', 'close'), (1, 2), ('fired', 'native-counterpart')),
        'test_native_regex_and_callback_counterparts_preserve_observation': entry(('regex', 'callback'), (1, 2), ('fired', 'native-counterpart')),
        'test_native_handoff_failure_skips_exact_child_and_recovers': entry(('handoff',), (1, 2), ('fired', 'native-counterpart', 'resource-conservation', 'ordinal-separation')),
        'test_parser_failure_then_valid_content_is_usable': entry(('parser',), controls=('fired',)),
        'test_duplicate_startup_options_release_previous_saved_strings': entry((), (), ('startup-option-owner-regression',), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_stdin_tags_reset_releases_saved_filename': entry((), (), ('startup-option-owner-regression',), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_stop_directive_releases_discarded_command_table_owners': entry((), (), ('command-table-owner-regression',), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_duplicate_message_and_help_definitions_release_previous_overrides': entry((), (), ('message-owner-regression',), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_parser_partial_staging_failure_releases_each_table_before_retry': entry(('parser',), (2, 4, 6), ('fired', 'ordinal-separation'), scenario='SAFECASE-MEM-007', variant='failure before and after commit staging'),
        'test_natural_parser_rejection_after_staging_then_valid_retry': entry(('parser',), (), ('natural-parser-rejection',), scenario='SAFECASE-MEM-007', variant='failure before and after commit staging'),
        'test_parser_preinit_failure_preserves_previously_published_buffers': entry(('parser',), controls=('early-parser-owner-regression',), scenario='SAFECASE-MEM-007', variant='failure before and after commit staging'),
        'test_natural_filter_rejection_preserves_committed_filter_then_clear': entry(('filter-before', 'filter-after'), (), ('natural-filter-rejection', 'resource-conservation')),
        'test_repeated_file_and_child_failures_conserve_live_process_resources': entry(('open', 'handoff'), (), ('resource-conservation',), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_injected_recovery_conserves_descriptors_and_children_in_live_pager': entry(('regex', 'filter-before', 'filter-after', 'handoff', 'cleanup', 'close'), controls=('fired', 'resource-conservation'), scenario='SAFECASE-VER-006', variant='FD/worker/child/live allocation accounting'),
        'test_callback_startup_failure_preserves_later_session_lifecycle': entry(('callback',), controls=('fired', 'callback-startup-lifecycle')),
        'test_reserve_failure_before_parser_growth_has_controlled_oom': entry(('reserve',), controls=('fired',)),
        'test_reserve_second_growth_ordinal_has_distinct_controlled_failure': entry(('reserve',), (1, 2), ('fired', 'ordinal-separation')),
        'test_filter_handoff_close_cleanup_and_regex_second_ordinals': entry(('filter-before', 'filter-after', 'handoff', 'close', 'cleanup', 'regex'), (2,), ('fired', 'ordinal-separation')),
        'test_cleanup_retries_injected_and_native_eintr_and_restores_terminal': entry(('cleanup',), controls=('fired', 'native-counterpart', 'unfaulted')),
        'test_cleanup_preserves_primary_termination_status': entry(('cleanup',), controls=('fired', 'native-counterpart')),
        'test_cleanup_unreached_ordinal_and_normal_build_absence': entry(('cleanup',), (1, 999999), ('unreached', 'unfaulted')),
        'test_normal_build_absence_for_every_site': entry(('alloc', 'read', 'open', 'write', 'regex', 'parser', 'filter-before', 'filter-after', 'handoff', 'callback', 'cleanup', 'reserve', 'close', 'read-eintr', 'read-short', 'write-short'), controls=('unfaulted',)),
        'test_unreached_ordinal_cannot_supply_hit_evidence_at_any_site': entry(('alloc', 'read', 'open', 'write', 'regex', 'parser', 'filter-before', 'filter-after', 'handoff', 'callback', 'cleanup', 'reserve', 'close', 'read-eintr', 'read-short', 'write-short'), (999999,), ('unreached', 'unfaulted')),
        'test_real_cleanup_oracle_rejects_hit_status_screen_mode_bell_and_file_mutations': entry(('cleanup',), controls=('fired',), scenario='SAFECASE-VER-010', variant='wrong screen/mode/status/bell/effect/helper output'),
        'test_regex_compile_failure_retains_recovery_behavior': entry(('regex',), controls=('fired', 'native-counterpart')),
        'test_close_failure_is_after_real_release_and_preserves_terminal': entry(('close',), controls=('fired', 'unfaulted')),
        'test_filter_failure_before_and_after_commit_has_distinct_state': entry(('filter-before', 'filter-after'), controls=('fired', 'committed-filter-recovery')),
        'test_handoff_failure_skips_child_then_recovers_and_closes_trace_on_exec': entry(('handoff',), controls=('fired',)),
        'test_callback_reachability_is_not_native_error_propagation': entry(('callback',), controls=('fired', 'unfaulted')),
    }


SITE_IDS = dict(zip(('alloc', 'read', 'open', 'write', 'regex', 'parser', 'filter-before',
                    'filter-after', 'handoff', 'callback', 'cleanup', 'reserve', 'close',
                    'read-eintr', 'read-short', 'write-short'), range(1, 17)))


def validate_oracle_report(report, expected_selectors=None):
    """Validate retained observations and derive actual hits, never plan credit.

    The supervising runner separately verifies source/binary/log hashes and the
    original process status. This function is shared by host and ABI validators.
    """
    known = {'ActualFaultPaths.' + name: value for name, value in contracts().items()}
    expected = set(known) if expected_selectors is None else set(expected_selectors)
    if not expected or not expected <= set(known):
        raise ValueError('unknown or empty actual oracle selection')
    if expected_selectors is not None and len(expected) != len(expected_selectors):
        raise ValueError('duplicate expected oracle selector')
    records = report.get('records')
    if (type(report.get('schema_version')) is not int or report.get('schema_version') != 2 or report.get('passed') is not True
            or type(report.get('repetitions')) is not int or report['repetitions'] < 1
            or not isinstance(records, list) or type(report.get('tests_run')) is not int
            or len(records) != len(expected)):
        raise ValueError('failed or incomplete actual oracle report')
    expected_runs = sum(1 if selector.split('.')[-1] in INTERNAL_REPEAT_METHODS else report['repetitions']
                        for selector in expected)
    if report['tests_run'] != expected_runs:
        raise ValueError('requested repetitions were not executed')
    result = {}
    names = {value: name for name, value in SITE_IDS.items()}
    for record in records:
        selector = record.get('test_selector')
        if selector not in expected or selector in result or record.get('passed') is not True:
            raise ValueError('missing, duplicate or failed test oracle')
        contract = known[selector]
        if record.get('contract') != contract:
            raise ValueError('actual oracle contract changed')
        observations = record.get('observations')
        if not isinstance(observations, list) or not observations:
            raise ValueError('test has no executed process observations')
        internal = selector.split('.')[-1] in INTERNAL_REPEAT_METHODS
        counts = record.get('invocation_observation_counts')
        if (record.get('repetition_mode') != ('internal-cycles' if internal else 'test-invocations')
                or not isinstance(counts, list) or len(counts) != (1 if internal else report['repetitions'])
                or any(type(count) is not int or count < 1 for count in counts)
                or sum(counts) != len(observations)):
            raise ValueError('missing or inconsistent executed invocation observations')
        method = selector.split('.')[-1]
        if internal:
            if method == 'test_cleanup_retries_injected_and_native_eintr_and_restores_terminal':
                if len(observations) != 1 + 2 * report['repetitions']:
                    raise ValueError('cleanup repetitions lack actual process observations')
            else:
                expected_observations = 12 if method == 'test_injected_recovery_conserves_descriptors_and_children_in_live_pager' else 2
                if len(observations) != expected_observations:
                    raise ValueError('lifecycle control has incomplete process observations')
                for observation in observations:
                    checkpoints = observation.get('resources', [])
                    minimum = report['repetitions'] + 1
                    if method == 'test_repeated_file_and_child_failures_conserve_live_process_resources':
                        minimum = 2 * report['repetitions'] + 1
                    if not isinstance(checkpoints, list) or len(checkpoints) < minimum:
                        raise ValueError('lifecycle repetitions lack actual resource checkpoints')
        if method == 'test_write_failure_stops_unbounded_input_and_broken_stderr':
            if any(count != 4 for count in counts):
                raise ValueError('unbounded-write oracle lacks all four executed shapes')
            for index, observation in enumerate(observations):
                natural = index % 4 >= 2
                if (observation.get('kind') != 'pipe' or observation.get('input_kind') != 'unbounded-pipe'
                        or observation.get('status') != 1 or observation.get('producer_stopped') is not True
                        or observation.get('producer_broken_pipe') is not True
                        or type(observation.get('producer_bytes')) is not int
                        or not 0 < observation['producer_bytes'] < 1024 * 1024
                        or observation.get('stdout_full') is not natural
                        or observation.get('stderr_full') is not (index % 4 == 3)
                        or (natural and observation.get('trace_hex') != '')):
                    raise ValueError('unbounded-write oracle lacks bounded producer/stream evidence')
        observed = set()
        for observation in observations:
            if observation.get('kind') not in ('pipe', 'pty') or type(observation.get('status')) is not int:
                raise ValueError('invalid process observation')
            if observation['kind'] == 'pty' and observation.get('termios_restored') is not True:
                raise ValueError('missing or failed terminal restoration observation')
            if 'termination-cleanup' in contract['m0_controls'] and (
                    observation['kind'] != 'pipe' or observation.get('process_reaped') is not True
                    or observation.get('process_group_empty') is not True):
                raise ValueError('missing or failed process termination observation')
            if 'resource-conservation' in contract['m0_controls']:
                checkpoints = observation.get('resources')
                if observation['kind'] != 'pty' or not isinstance(checkpoints, list) or len(checkpoints) < 2:
                    raise ValueError('missing live process resource observations')
                counts = []
                for checkpoint in checkpoints:
                    descriptors = checkpoint.get('descriptors')
                    if (not isinstance(descriptors, list) or not descriptors
                            or not all(isinstance(item, str) for item in descriptors)
                            or not input_worker_children(checkpoint)):
                        raise ValueError('invalid descriptor observation or unreaped child')
                    counts.append(len(descriptors))
                if len(set(counts)) != 1:
                    raise ValueError('live process descriptors were not conserved')
                identities = [resource_identity(item, 'close' in contract['hook_sites']) for item in checkpoints]
                if any(identity != identities[0] for identity in identities):
                    raise ValueError('live descriptor identity changed unexpectedly')
            try:
                trace = bytes.fromhex(observation['trace_hex'])
            except (ValueError, TypeError, KeyError) as error:
                raise ValueError('invalid trace encoding') from error
            if len(trace) % 16:
                raise ValueError('truncated trace event')
            for offset in range(0, len(trace), 16):
                magic, site_id, ordinal = struct.unpack('<4sIQ', trace[offset:offset+16])
                if magic not in (b'RLF1', b'RNT1') or site_id not in names or not ordinal:
                    raise ValueError('unknown, exhausted or malformed actual event')
                site = names[site_id]
                if magic == b'RNT1' and site == 'callback':
                    from native_callback import validate_shape
                    validate_shape(observation.get('native_callback_binding'))
                    if observation.get('native_plan') != f'callback:{ordinal}':
                        raise ValueError('native callback event differs from bound plan')
                if site not in contract['hook_sites'] or ordinal not in contract['hook_ordinals']:
                    raise ValueError('actual hook differs from executed selector')
                observed.add((site, ordinal, magic.decode('ascii')))
        result[selector] = [dict(site=site, ordinal=ordinal, magic=magic)
                            for site, ordinal, magic in sorted(observed)]
    if set(result) != expected:
        raise ValueError('missing selected process oracle')
    return result


class OracleResult(unittest.TextTestResult):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.oracles = []

    def startTest(self, test):
        OBSERVATIONS.clear()
        super().startTest(test)

    def stopTest(self, test):
        name = test._testMethodName
        bad = {item.id() for item, _ in [*self.failures, *self.errors, *self.skipped]}
        selector = 'ActualFaultPaths.' + name
        record = next((item for item in self.oracles if item['test_selector'] == selector), None)
        if record is None:
            record = dict(test_selector=selector, passed=True, contract=contracts().get(name),
                repetition_mode='internal-cycles' if name in INTERNAL_REPEAT_METHODS else 'test-invocations',
                invocation_observation_counts=[], observations=[])
            self.oracles.append(record)
        record['passed'] &= test.id() not in bad
        record['invocation_observation_counts'].append(len(OBSERVATIONS))
        record['observations'].extend(copy.deepcopy(OBSERVATIONS))
        super().stopTest(test)


class RepeatingOracleRunner(unittest.TextTestRunner):
    def run(self, test):
        def repetitions(suite):
            for selected in suite:
                if isinstance(selected, unittest.TestSuite):
                    yield from repetitions(selected)
                else:
                    count = 1 if selected._testMethodName in INTERNAL_REPEAT_METHODS else REPETITIONS
                    for _ in range(count):
                        yield type(selected)(selected._testMethodName)
        return super().run(unittest.TestSuite(repetitions(test)))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--normal', type=Path, required=True)
    parser.add_argument('--injected', type=Path, required=True)
    parser.add_argument('--native-shim', type=Path, required=True)
    parser.add_argument('--short-write-shim', type=Path, required=True)
    parser.add_argument('--native-io-shim', type=Path)
    parser.add_argument('--native-alloc-binding', type=Path)
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--oracle-report', type=Path)
    parser.add_argument('--repetitions', type=int, default=1)
    parser.add_argument('--instrumentation-env', type=Path)
    args, remaining = parser.parse_known_args()
    if args.repetitions < 1:
        parser.error('repetitions must be positive')
    NORMAL, INJECTED, SHIM = (p.resolve(strict=True) for p in (args.normal, args.injected, args.native_shim))
    SHORT_SHIM = args.short_write_shim.resolve(strict=True)
    IO_SHIM = args.native_io_shim.resolve(strict=True) if args.native_io_shim else None
    REFERENCE = args.reference.resolve(strict=True) if args.reference else None
    if args.native_alloc_binding:
        from native_alloc import validate_binding
        ALLOC_BINDING = validate_binding(json.loads(args.native_alloc_binding.read_text()))
    REPETITIONS = args.repetitions
    if args.instrumentation_env:
        INSTRUMENTATION = json.loads(args.instrumentation_env.read_text())
        if set(INSTRUMENTATION) - {'ASAN_OPTIONS', 'LSAN_OPTIONS', 'ASAN_SYMBOLIZER_PATH'}:
            parser.error('unsupported instrumentation variable')
    runner = RepeatingOracleRunner(verbosity=2, resultclass=OracleResult)
    program = unittest.main(argv=[sys.argv[0], *remaining], testRunner=runner, exit=False)
    if args.oracle_report:
        args.oracle_report.parent.mkdir(parents=True, exist_ok=True)
        args.oracle_report.write_text(json.dumps(dict(schema_version=2, repetitions=REPETITIONS,
            tests_run=program.result.testsRun, passed=program.result.wasSuccessful(),
            records=program.result.oracles), indent=2) + '\n')
    raise SystemExit(0 if program.result.wasSuccessful() else 1)
