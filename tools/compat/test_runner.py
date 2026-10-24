"""Regression checks for semantic PTY checkpoints; no frozen data is rewritten."""
from pathlib import Path
import unittest

from run_v2 import ROOT, load, replay_expectations, run_case


class Checkpoints(unittest.TestCase):
    def test_one_resize_event_delivers_one_signal(self):
        import hashlib
        helper = '''import os, signal, termios, tty
saved = termios.tcgetattr(0)
tty.setraw(0)
count = 0
def resized(signum, frame):
    global count
    count += 1
    os.write(1, b"\\r\\nRESIZED\\r\\n")
signal.signal(signal.SIGWINCH, resized)
try:
    os.write(1, b"READY")
    assert os.read(0, 1) == b"q"
    with open("count", "w") as file:
        file.write(str(count))
finally:
    termios.tcsetattr(0, termios.TCSANOW, saved)
'''
        actual = run_case(Path('/usr/bin/python3'), dict(
            id='observer-single-resize-signal', mode='pty', argv=['helper.py'],
            files={'helper.py': {'text': helper}}, events=[{'await': ['READY']},
                {'resize': [12, 60], 'await': ['RESIZED']},
                {'resize': [12, 60], 'await': ['RESIZED']},
                {'resize': [24, 80], 'await': ['RESIZED']}, {'send': 'q'}]))
        self.assertEqual(actual['status'], 0)
        self.assertEqual(actual['files']['count']['sha256'], hashlib.sha256(b'3').hexdigest())
        self.assertTrue(actual['termios_restored'])

    def test_stress_clock_keeps_monotonic_timers_real(self):
        import os
        import subprocess
        import sys
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            library = Path(directory)/'clock.so'
            subprocess.run(['cc', '-shared', '-fPIC', str(ROOT/'tools/compat/probes/fixed_clock.c'),
                            '-o', str(library)], check=True, capture_output=True)
            code = '''import ctypes, time
libc = ctypes.CDLL(None)
libc.time.argtypes = [ctypes.POINTER(ctypes.c_long)]
libc.time.restype = ctypes.c_long
value = ctypes.c_long()
assert libc.time(ctypes.byref(value)) == value.value == 1700000000
before = time.monotonic()
time.sleep(0.01)
assert time.monotonic() - before >= 0.005
'''
            result = subprocess.run([sys.executable, '-c', code],
                                    env={**os.environ, 'LD_PRELOAD': str(library)},
                                    capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 0, result.stderr)

    def test_stream_can_grow_without_closing_the_pipe(self):
        import base64
        helper = '''import os
assert os.read(0, 1) == b"a"
os.write(1, b"READY\\n")
assert os.read(0, 1) == b"b"
os.write(1, b"DONE\\n")
'''
        actual = run_case(Path('/usr/bin/python3'), dict(
            id='observer-growing-pipe', mode='pty', argv=['helper.py'],
            files={'helper.py': {'text': helper}},
            stream_b64=base64.b64encode(b'a').decode(),
            events=[{'await': ['READY']},
                    {'write_stream_b64': base64.b64encode(b'b').decode(),
                     'await': ['DONE'], 'snapshot': True}]))
        self.assertEqual(actual['status'], 0)
        self.assertEqual(actual['checkpoints'][0]['rows'][1].rstrip(), 'DONE')

    def test_file_events_cannot_escape_the_fixture_directory(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            protected = Path(directory)/'protected'
            protected.write_text('unchanged')
            case = dict(id='observer-confined-files', mode='pty', argv=['helper.py'],
                        files={'helper.py': {'text': 'import os\nprint("READY", flush=True)\nos.read(0, 1)\n'}},
                        events=[{'await': ['READY']},
                                {'append_file': {'path': str(protected), 'text': 'changed'}}])
            with self.assertRaisesRegex(ValueError, 'within the case directory'):
                run_case(Path('/usr/bin/python3'), case)
            self.assertEqual(protected.read_text(), 'unchanged')

    def test_versioned_correction_is_bound_to_the_original_record(self):
        import json
        import tempfile
        frozen = load(ROOT/'compat/expected-integration.json')
        revised = replay_expectations(ROOT/'compat/expected-integration.json')
        self.assertNotEqual(frozen['uri-handler'], revised['uri-handler'])
        for name in ('editor-argv', 'global-provider', 'history-load'):
            self.assertEqual(frozen[name], revised[name])
        frozen['uri-handler']['status'] = 99
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'changed.json'
            path.write_text(json.dumps(frozen))
            with self.assertRaisesRegex(ValueError, 'unchanged version 1 record'):
                replay_expectations(path)

    def test_waits_for_queued_commands_and_complete_repaint(self):
        helper = '''import os, termios, tty
saved = termios.tcgetattr(0)
tty.setraw(0)
try:
    os.write(1, b"READY")
    assert os.read(0, 1) == b"a"
    os.write(1, b"\\rREADY")
    assert os.read(0, 1) == b"b"
    os.write(1, b"\\rREADY")
    assert os.read(0, 1) == b"c"
    # Simulate work between the interim prompt and its repaint. The observer
    # must wait for the input syscall, regardless of how writes are scheduled.
    for i in range(200000):
        value = i * i
    os.write(1, b"\\x1b[H\\x1b[2Jcomplete\\r\\nREADY")
    assert os.read(0, 1) == b"q"
finally:
    termios.tcsetattr(0, termios.TCSANOW, saved)
'''
        case = dict(id='observer-queued-repaint', mode='pty',
                    argv=['helper.py'], files={'helper.py': {'text': helper}},
                    events=[{'await': ['READY']},
                            {'send': 'abc', 'await': ['READY'], 'snapshot': True},
                            {'send': 'q'}])
        for _ in range(5):
            actual = run_case(Path('/usr/bin/python3'), case)
            self.assertEqual(actual['status'], 0)
            self.assertEqual(actual['checkpoints'][0]['rows'][0].rstrip(), 'complete')
            self.assertEqual(actual['checkpoints'][0]['rows'][1].rstrip(), 'READY')
            self.assertTrue(actual['termios_restored'])


if __name__ == '__main__':
    unittest.main()
