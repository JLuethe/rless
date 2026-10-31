"""Mutation definitions for the retained linked-object body proof.

These tests read the retained liveheap fixture, then mutate temporary copies.
They never execute the pager or compile a counterpart.
"""

from pathlib import Path
import struct
import tempfile
import unittest

import linked_object


ROOT = Path(__file__).resolve().parents[2]
RUN = ROOT / 'compat/.runs/liveheap-link-20261031T081000Z'
OBJECT = RUN / 'live_heap.o'
BINARY = RUN / 'target/x86_64-unknown-linux-gnu/debug/rless'
ANCHOR = '__wrap_read'


class LinkedObjectMutations(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if not OBJECT.is_file() or not BINARY.is_file():
            raise ValueError('required retained liveheap ELF pair is unavailable; supply unit_tests.py fixture arguments')
        cls.proof = linked_object.verify(BINARY, OBJECT, ANCHOR)
        cls.binary = linked_object.Elf(BINARY.read_bytes(), {2, 3})
        cls.object = linked_object.Elf(OBJECT.read_bytes(), {1})

    def modified_binary(self, edit):
        data = bytearray(BINARY.read_bytes())
        edit(data)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'rless-mutated'
            path.write_bytes(data)
            with self.assertRaises(ValueError):
                linked_object.verify(path, OBJECT, ANCHOR)

    def test_retained_body_proves(self):
        self.assertEqual(self.proof['source_code_sha256'],
                         self.proof['normalized_code_sha256'])
        self.assertTrue(self.proof['relocations'])
        self.assertTrue(self.proof['constructors'])

    def test_nonrelocated_instruction_change_rejected(self):
        address = self.proof['section_bases']['.text']
        section = self.binary.only('.text')
        offset = section['offset'] + address - section['address']
        self.modified_binary(lambda data: data.__setitem__(offset, data[offset] ^ 1))

    def test_relocation_target_change_rejected(self):
        relocation = next(row for row in self.proof['relocations']
                          if row['symbol'] == ('__real_read' if ANCHOR == '__wrap_read' else '__real_calloc'))
        address = self.proof['section_bases']['.text'] + relocation['offset']
        section = self.binary.only('.text')
        offset = section['offset'] + address - section['address']

        def edit(data):
            displacement = struct.unpack_from('<i', data, offset)[0]
            struct.pack_into('<i', data, offset, displacement + 1)

        self.modified_binary(edit)

    def test_data_byte_change_rejected(self):
        address = self.proof['section_bases']['.data']
        section = self.binary.only('.data')
        offset = section['offset'] + address - section['address']
        self.modified_binary(lambda data: data.__setitem__(offset, data[offset] ^ 1))

    def test_constructor_registration_change_rejected(self):
        address = self.proof['constructors'][0]['slot']
        section = self.binary.only('.init_array')
        offset = section['offset'] + address - section['address']

        def edit(data):
            pointer = struct.unpack_from('<Q', data, offset)[0]
            struct.pack_into('<Q', data, offset, pointer ^ 1)

        self.modified_binary(edit)

    def test_unsupported_object_relocation_rejected(self):
        data = bytearray(OBJECT.read_bytes())
        section = self.object.only('.rela.text')
        info_offset = section['offset'] + 8
        info = struct.unpack_from('<Q', data, info_offset)[0]
        struct.pack_into('<Q', data, info_offset, (info & ~0xffffffff) | 99)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'unsupported.o'
            path.write_bytes(data)
            with self.assertRaises(ValueError):
                linked_object.verify(BINARY, path, ANCHOR)


if __name__ == '__main__':
    unittest.main()
