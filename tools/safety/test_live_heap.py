"""Mutation-sensitive live-byte parsing controls; no pager evidence here."""
import copy
import io
import struct
import unittest
from unittest.mock import patch

from live_heap import Samples, require_balance


def record(sequence, call=271, argument=0x1234):
    return struct.pack('<4sIQQQQQ', b'RHP1', 1, sequence, 4096, 4, call, argument)


class LiveHeapControls(unittest.TestCase):
    def test_balance_rejects_one_retained_owner_and_cumulative_growth(self):
        checkpoints = [dict(label=label, allocated_bytes=4096) for label in
                       ('warmup', 'cycle-3', 'cycle-4', 'cycle-5')]
        self.assertEqual(require_balance(checkpoints, 5), [0,0,0])
        for values in ([4096,4128,4128,4128], [4096,4128,4160,4192], [4096,4096,4095,4096]):
            changed = copy.deepcopy(checkpoints)
            for item, value in zip(changed, values):
                item['allocated_bytes'] = value
            with self.assertRaisesRegex(ValueError, 'warmed baseline'):
                require_balance(changed, 5)
        with self.assertRaises(ValueError):
            require_balance(checkpoints[:-1], 5)
        checkpoints[1]['allocated_bytes'] = True
        with self.assertRaises(ValueError):
            require_balance(checkpoints, 5)

    def test_raw_records_require_unbroken_sequence_and_backend(self):
        samples = Samples(io.BytesIO(record(1)+record(2)))
        samples.drain()
        self.assertEqual([x['sequence'] for x in samples.records], [1,2])
        for raw in (record(1)[:-1], record(2), record(1)+record(1), b'NOPE'+record(1)[4:]):
            with self.assertRaises(ValueError):
                Samples(io.BytesIO(raw)).drain()

    def test_checkpoint_requires_fresh_sample_for_the_blocked_wait(self):
        for syscall in ('7 0x1234 1', '271 0x5678 1', '271 0x1234 2', '0 0x4'):
            samples = Samples(io.BytesIO(record(1)))
            with patch('live_heap.Path.read_text', return_value=syscall), self.assertRaises(ValueError):
                samples.checkpoint(123, 'warmup')
        samples = Samples(io.BytesIO(record(1)))
        with patch('live_heap.Path.read_text', return_value='271 0x1234 1'):
            samples.checkpoint(123, 'warmup')
            with self.assertRaisesRegex(ValueError, 'stale'):
                samples.checkpoint(123, 'cycle-3')
        samples = Samples(io.BytesIO(record(1, call=0, argument=4)))
        with patch('live_heap.Path.read_text', return_value='0 0x4'):
            samples.checkpoint(123, 'warmup')


if __name__ == '__main__':
    unittest.main(verbosity=2)
