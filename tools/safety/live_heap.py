"""Supplemental live ASAN allocator balance at actual pager input barriers.

This is a scoped detector, not whole-program memory safety. It does not depend
on LSAN reachability and never substitutes FD counts for allocated bytes.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import struct

from common import PINS
from test_fault_paths import marker, pager, read_trace, trace_channel
from run_v2 import run_case


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


class Samples:
    def __init__(self, reader):
        self.reader = reader
        self.raw = bytearray()
        self.records = []
        self.checkpoints = []

    def drain(self):
        while True:
            data = self.reader.read(65536)
            if not data:
                break
            self.raw.extend(data)
            if len(self.raw) > 1048576:
                raise ValueError('heap observation exceeded bounded channel')
        if len(self.raw) % 48:
            raise ValueError('truncated heap sample')
        while len(self.records) * 48 < len(self.raw):
            offset = len(self.records) * 48
            magic, backend, sequence, allocated, fd, call, argument = struct.unpack('<4sIQQQQQ', self.raw[offset:offset+48])
            if (magic != b'RHP1' or backend != 1 or sequence != len(self.records)+1
                    or call not in (0,271) or fd > 0x7fffffff
                    or (call == 0 and argument != fd) or (call == 271 and argument == 0)):
                raise ValueError('invalid heap sample identity or sequence')
            self.records.append(dict(sequence=sequence, allocated_bytes=allocated, terminal_fd=fd,
                                     wait_syscall=call, wait_argument=argument))

    def checkpoint(self, pid, label):
        # run_case already established terminal input consumed and process asleep.
        # Match the actual blocking read/one-FD ppoll and argument identity; an
        # old sample from the previous key must not stand in for current state.
        syscall = (Path('/proc')/str(pid)/'syscall').read_text().split()
        if syscall[0] not in ('0', '271') or (syscall[0] == '271' and int(syscall[2],0) != 1):
            raise ValueError('heap checkpoint is not blocked in terminal read/ppoll')
        self.drain()
        if (not self.records or self.records[-1]['wait_syscall'] != int(syscall[0])
                or self.records[-1]['wait_argument'] != int(syscall[1], 0)):
            raise ValueError('heap sample does not identify current terminal wait')
        sample = self.records[-1]
        if self.checkpoints and sample['sequence'] <= self.checkpoints[-1]['sequence']:
            raise ValueError('heap checkpoint reused a stale sample')
        self.checkpoints.append(dict(sample, label=label, blocked_syscall=syscall))


def require_balance(checkpoints, cycles):
    labels = ['warmup'] + [f'cycle-{i}' for i in range(3, cycles+1)]
    if (not isinstance(checkpoints, list) or any(not isinstance(x,dict) for x in checkpoints)
            or [x.get('label') for x in checkpoints] != labels
            or any(type(x.get('allocated_bytes')) is not int or x['allocated_bytes'] <= 0 for x in checkpoints)):
        raise ValueError('missing or invalid live heap checkpoints')
    baseline = checkpoints[0]['allocated_bytes']
    deltas = [x['allocated_bytes']-baseline for x in checkpoints[1:]]
    if any(deltas):
        raise ValueError('live heap did not return to warmed baseline: ' + repr(deltas))
    return deltas


def scenario(site, cycles):
    if site not in (None, 'filter-before', 'filter-after', 'regex') or type(cycles) is not int or not 4 <= cycles <= 1000:
        raise ValueError('unsupported live heap scenario')
    events = [{'await':['READY']}]
    for cycle in range(1, cycles+1):
        failure = site is not None and cycle == 3
        prompt = ('Invalid pattern' if site == 'regex' else
                  'Injected post-commit failure' if site == 'filter-after' else 'Injected filter commit failure')
        events.append({'send':'&keep\n', 'await':[prompt if failure else 'READY']})
        if failure:
            events.append({'send':'\n', 'await':['READY']})
        event = {'send':'&\ng', 'await':['READY']}
        if cycle >= 2:
            event['heap_checkpoint'] = 'warmup' if cycle == 2 else f'cycle-{cycle}'
        events.append(event)
    events.append({'send':'q'})
    # A READY prompt can precede completion of an output-worker write. Heap
    # checkpoints must follow a real terminal-input wait, not its ack ppoll.
    for event in events:
        if 'await' in event:
            event['require_terminal_input'] = True
    return pager(events)


def observe(binary, shim, output, site=None, cycles=10):
    case = scenario(site, cycles)
    output.mkdir(parents=True, exist_ok=False)
    reports = output/'sanitizer'
    reports.mkdir()
    with trace_channel() as (heap_reader, heap_writer), trace_channel() as (fault_reader, fault_writer):
        for writer in (heap_writer, fault_writer):
            os.set_inheritable(writer.fileno(), True)
        case['env'] = dict(RLESS_HEAP_TRACE_FD=str(heap_writer.fileno()),
            ASAN_OPTIONS=PINS['asan_options']+':symbolize=0:log_path='+str(reports/'raw'),
            LSAN_OPTIONS=PINS['lsan_options'])
        if site:
            case['env'].update(RLESS_FAULT=site+':3', RLESS_FAULT_FD=str(fault_writer.fileno()))
        samples = Samples(heap_reader)
        observed = run_case(binary, case, checkpoint_observer=samples.checkpoint)
        fault_trace = read_trace(fault_reader, fault_writer)
        heap_writer.close()
        samples.drain()
    (output/'heap-trace.bin').write_bytes(samples.raw)
    result = dict(schema_version=1, scope='whole-ASAN-allocator live bytes at warmed input barriers; scoped filter/regex sessions only',
        binary=str(binary), binary_sha256=sha(binary), shim=str(shim), shim_sha256=sha(shim),
        helper_sha256=sha(__file__), shim_source_sha256=sha(Path(__file__).parent/'native/live_heap.c'),
        observer_sha256=sha(Path(__file__).parents[1]/'compat/run_v2.py'),
        site=site, ordinal=3 if site else None, cycles=cycles, case=case, observation=observed,
        fault_trace_hex=fault_trace.hex(), heap_trace_sha256=sha(output/'heap-trace.bin'),
        samples=samples.records, checkpoints=samples.checkpoints, passed=False)
    # Evaluate this detector independently of exit-time LSAN. In particular a
    # positive mutation must not obtain its rejection solely from exit 87.
    try:
        result['deltas'] = require_balance(samples.checkpoints, cycles)
        result['heap_balance_passed'] = True
    except ValueError as error:
        result['heap_balance_passed'] = False
        result['heap_balance_error'] = str(error)
    try:
        if not result['heap_balance_passed']:
            raise ValueError(result['heap_balance_error'])
        expected = b'' if site is None else marker({'filter-before':7,'filter-after':8,'regex':5}[site],3)
        if fault_trace != expected or observed['status'] != 0 or not observed['termios_restored']:
            raise ValueError('fault identity, process status or terminal restoration failed')
        if list(reports.iterdir()):
            raise ValueError('sanitizer diagnostic in live heap session')
        result['passed'] = True
    except ValueError as error:
        result['error'] = str(error)
    (output/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--shim', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--site', choices=('filter-before','filter-after','regex'))
    parser.add_argument('--cycles', type=int, default=10)
    args = parser.parse_args()
    result = observe(args.binary.resolve(), args.shim.resolve(), args.output.resolve(), args.site, args.cycles)
    print(json.dumps(dict(passed=result['passed'], error=result.get('error'), output=str(args.output))))
    raise SystemExit(0 if result['passed'] else 1)


if __name__ == '__main__':
    main()
