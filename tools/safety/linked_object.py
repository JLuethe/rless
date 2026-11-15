#!/usr/bin/env python3
"""Prove that a retained x86_64 ELF object body is present in a linked pager.

This is a read-only, fail-closed verifier for the test-only O0 C wrappers. It
does not execute either ELF, trust objdump labels, or reconstruct a build. The
build/source record must independently bind the retained object to its source.
"""

import argparse
import hashlib
import json
from pathlib import Path
import struct

from native_alloc import ElfTargets


ELF_HEADER = struct.Struct('<16sHHIQQQIHHHHHH')
SECTION = struct.Struct('<IIQQQQIIQQ')
SYMBOL = struct.Struct('<IBBHQQ')
RELA = struct.Struct('<QQq')
SHT_PROGBITS = 1
SHT_SYMTAB = 2
SHT_NOBITS = 8
SHT_INIT_ARRAY = 14
SHT_RELA = 4
SHF_ALLOC = 2
SHF_EXECINSTR = 4
STT_FUNC = 2
STT_OBJECT = 1
STT_SECTION = 3
R_64 = 1
R_PC32 = 2
R_PLT32 = 4
R_GOTPCREL = 9
R_GOTPCRELX = 41
R_REX_GOTPCRELX = 42
TEXT_RELOCATIONS = {R_PC32, R_PLT32, R_GOTPCREL, R_GOTPCRELX, R_REX_GOTPCRELX}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


class Elf:
    def __init__(self, content, expected_type):
        self.content = content
        require(len(content) >= ELF_HEADER.size, 'truncated ELF header')
        (identity, elf_type, machine, version, _entry, _phoff, shoff, _flags,
         ehsize, _phentsize, _phnum, shentsize, shnum, shstrndx) = ELF_HEADER.unpack_from(content)
        require(identity[:7] == b'\x7fELF\x02\x01\x01' and machine == 62 and version == 1,
                'requires ELF64 little-endian x86_64')
        require(elf_type in expected_type and ehsize == 64 and shentsize == SECTION.size,
                'unsupported ELF type/header')
        require(0 < shnum <= 65535 and shoff + shnum * shentsize <= len(content)
                and 0 < shstrndx < shnum, 'invalid section table')
        self.sections = []
        for index in range(shnum):
            fields = SECTION.unpack_from(content, shoff + index * shentsize)
            name, kind, flags, address, offset, size, link, info, align, entsize = fields
            if kind != SHT_NOBITS:
                require(offset + size <= len(content), 'section extends beyond ELF')
            self.sections.append(dict(index=index, name_offset=name, kind=kind,
                                      flags=flags, address=address, offset=offset,
                                      size=size, link=link, info=info, align=align,
                                      entsize=entsize))
        names = self.section_bytes(self.sections[shstrndx])
        for section in self.sections:
            section['name'] = self.string(names, section['name_offset'])
        self.by_name = {}
        for section in self.sections:
            self.by_name.setdefault(section['name'], []).append(section)
        self.symbol_tables = {}
        self.symbols = []
        for section in self.sections:
            if section['kind'] != SHT_SYMTAB:
                continue
            require(section['entsize'] == SYMBOL.size and section['size'] % SYMBOL.size == 0
                    and section['link'] < shnum, 'invalid symbol table')
            strings = self.section_bytes(self.sections[section['link']])
            symbols = []
            raw = self.section_bytes(section)
            for offset in range(0, len(raw), SYMBOL.size):
                name, info, other, owner, value, size = SYMBOL.unpack_from(raw, offset)
                require(owner < shnum or owner in (0xfff1, 0xfff2), 'invalid symbol section')
                symbols.append(dict(name=self.string(strings, name), type=info & 15,
                                    bind=info >> 4, other=other, owner=owner,
                                    value=value, size=size))
            self.symbol_tables[section['index']] = symbols
            self.symbols.extend(symbols)

    @staticmethod
    def string(table, offset):
        require(offset < len(table), 'string offset outside table')
        end = table.find(b'\0', offset)
        require(end >= 0, 'unterminated ELF string')
        return table[offset:end].decode('utf-8', 'strict')

    def only(self, name):
        matches = self.by_name.get(name, [])
        require(len(matches) == 1, 'missing or ambiguous section: ' + name)
        return matches[0]

    def section_bytes(self, section):
        require(section['kind'] != SHT_NOBITS, 'NOBITS has no file bytes')
        start = section['offset']
        return self.content[start:start + section['size']]

    def address_bytes(self, address, size, expected_section=None):
        candidates = [section for section in self.sections
                      if section['kind'] != SHT_NOBITS and section['flags'] & SHF_ALLOC
                      and section['address'] <= address
                      and address + size <= section['address'] + section['size']
                      and (expected_section is None or section['name'] == expected_section)]
        require(len(candidates) == 1, 'address is not uniquely mapped in ' + str(expected_section))
        section = candidates[0]
        start = section['offset'] + address - section['address']
        return self.content[start:start + size]

    def relocations(self, target):
        matching = [section for section in self.sections if section['kind'] == SHT_RELA
                    and section['info'] == target['index']]
        require(len(matching) <= 1, 'ambiguous relocation section for ' + target['name'])
        if not matching:
            return []
        section = matching[0]
        require(section['entsize'] == RELA.size and section['size'] % RELA.size == 0
                and section['link'] in self.symbol_tables, 'invalid RELA table')
        symbols = self.symbol_tables[section['link']]
        result = []
        for offset in range(0, section['size'], RELA.size):
            place, info, addend = RELA.unpack_from(self.content, section['offset'] + offset)
            index, kind = info >> 32, info & 0xffffffff
            require(index < len(symbols), 'relocation symbol index outside table')
            result.append(dict(offset=place, kind=kind, addend=addend,
                               symbol=symbols[index]))
        return result


def symbol_addresses(elf, name, kind=None):
    return {(symbol['value'], symbol['size']) for symbol in elf.symbols
            if symbol['name'] == name and symbol['owner'] not in (0, 0xfff1, 0xfff2)
            and (kind is None or symbol['type'] == kind)}


def mapped_symbol_names(name):
    if name.startswith('__real_'):
        actual = name[len('__real_'):]
        return {actual, '__interceptor_' + actual, '___interceptor_' + actual}
    return {name}


def plt_target(binary, targets, address, names):
    # x86_64 PLT: optional endbr64 followed by FF 25 RIP-relative GOT jump.
    try:
        code = binary.address_bytes(address, 10)
    except ValueError:
        return False
    offset = 4 if code.startswith(b'\xf3\x0f\x1e\xfa') else 0
    if code[offset:offset + 2] != b'\xff\x25':
        return False
    displacement = struct.unpack_from('<i', code, offset + 2)[0]
    slot = address + offset + 6 + displacement
    return targets.matches_slot(slot, names)


def external_target(binary, targets, address, symbol, got):
    names = mapped_symbol_names(symbol['name'])
    require(names != {''}, 'anonymous undefined relocation target')
    if got:
        return targets.matches_slot(address, names)
    return any(address in targets.symbols.get(name, ()) for name in names) or \
        plt_target(binary, targets, address, names)


def object_section_name(obj, symbol):
    if symbol['owner'] in (0, 0xfff1, 0xfff2):
        return None
    return obj.sections[symbol['owner']]['name']


def verify_text(obj, binary, targets, base, anchor):
    source = obj.section_bytes(obj.only('.text'))
    linked = bytearray(binary.address_bytes(base, len(source), '.text'))
    normalized = bytearray(linked)
    section_bases = {'.text': base}
    fields = set()
    relaxations = set()
    proof = []
    for relocation in obj.relocations(obj.only('.text')):
        offset, kind, addend, symbol = (relocation[key] for key in
                                        ('offset', 'kind', 'addend', 'symbol'))
        require(kind in TEXT_RELOCATIONS, 'unsupported .text relocation kind: ' + str(kind))
        require(offset + 4 <= len(source) and all(i not in fields for i in range(offset, offset + 4)),
                'overlapping or out-of-bounds text relocation')
        fields.update(range(offset, offset + 4))
        place = base + offset
        displacement = struct.unpack_from('<i', linked, offset)[0]
        target = place + displacement - addend
        owner_name = object_section_name(obj, symbol)
        opcode_relaxed = False
        if kind in (R_GOTPCRELX, R_REX_GOTPCRELX):
            opcode = offset - 2
            require(opcode >= 0, 'GOT relaxation has no opcode')
            if linked[opcode] != source[opcode]:
                require(source[opcode] == 0x8b and linked[opcode] == 0x8d
                        and source[offset - 1] & 0xc7 == 0x05
                        and (kind != R_REX_GOTPCRELX or
                             offset >= 3 and 0x40 <= source[offset - 3] <= 0x4f),
                        'unsupported or forged GOT relaxation')
                opcode_relaxed = True
                normalized[opcode] = source[opcode]
                relaxations.add(opcode)
        if owner_name is not None:
            require(kind in (R_PC32, R_PLT32), 'unsupported internal GOT relocation')
            inferred = target - symbol['value']
            if owner_name in section_bases:
                require(section_bases[owner_name] == inferred,
                        'object section references disagree: ' + owner_name)
            else:
                section_bases[owner_name] = inferred
        else:
            require(bool(symbol['name']), 'anonymous external target')
            got = kind in (R_GOTPCREL, R_GOTPCRELX, R_REX_GOTPCRELX) and not opcode_relaxed
            require(external_target(binary, targets, target, symbol, got),
                    'relocation target mismatch: ' + symbol['name'])
        normalized[offset:offset + 4] = source[offset:offset + 4]
        proof.append(dict(offset=offset, kind=kind, symbol=symbol['name'] or owner_name,
                          target=target, relaxed=opcode_relaxed))
    require(normalized == source, 'linked .text differs outside verified relocation fields')
    require(len(source) > 0 and proof, 'object has no verifiable text body')
    for section in obj.sections:
        if section['name'] not in ('.rodata', '.data', '.bss') or not section['size']:
            continue
        name = section['name']
        require(name in section_bases, 'unanchored object section: ' + name)
        address = section_bases[name]
        require(address >= 0, 'negative section base')
        if name == '.bss':
            candidates = [part for part in binary.sections if part['name'] == '.bss'
                          and part['kind'] == SHT_NOBITS and part['address'] <= address
                          and address + section['size'] <= part['address'] + part['size']]
            require(len(candidates) == 1, 'object BSS not inside linked NOBITS range')
        else:
            require(binary.address_bytes(address, section['size'], name) == obj.section_bytes(section),
                    'linked ' + name + ' bytes differ from object')
    for symbol in obj.symbols:
        owner_name = object_section_name(obj, symbol)
        if owner_name is None or symbol['type'] not in (STT_FUNC, STT_OBJECT):
            continue
        require(bool(symbol['name']) and symbol['size'] > 0,
                'anonymous or empty defined function/data symbol')
        require(symbol['value'] + symbol['size'] <= obj.sections[symbol['owner']]['size'],
                'object symbol extends outside its section')
        if symbol['type'] == STT_FUNC:
            require(owner_name == '.text', 'object function outside contiguous .text')
        require(owner_name in section_bases, 'named symbol in unanchored section: ' + symbol['name'])
        expected = section_bases[owner_name] + symbol['value']
        require((expected, symbol['size']) in symbol_addresses(binary, symbol['name'], symbol['type']),
                'linked symbol position/size mismatch: ' + symbol['name'])
    require(anchor in {symbol['name'] for symbol in obj.symbols if symbol['type'] == STT_FUNC},
            'anchor is not a source function')
    return source, normalized, section_bases, proof


def verify_constructors(obj, binary, targets, section_bases):
    if '.init_array' not in obj.by_name:
        return []
    section = obj.only('.init_array')
    require(section['kind'] == SHT_INIT_ARRAY and section['size'] % 8 == 0,
            'invalid object constructor array')
    relocations = obj.relocations(section)
    require(len(relocations) == section['size'] // 8, 'constructor entries lack relocation proof')
    linked = binary.only('.init_array')
    require(linked['kind'] == SHT_INIT_ARRAY, 'linked constructor array missing')
    entries = []
    for relocation in relocations:
        require(relocation['kind'] == R_64 and relocation['offset'] % 8 == 0
                and relocation['offset'] + 8 <= section['size'], 'unsupported constructor relocation')
        symbol = relocation['symbol']
        owner_name = object_section_name(obj, symbol)
        require(owner_name in section_bases, 'unanchored constructor target')
        target = section_bases[owner_name] + symbol['value'] + relocation['addend']
        found = []
        for offset in range(0, linked['size'], 8):
            address = linked['address'] + offset
            value = struct.unpack_from('<Q', binary.content, linked['offset'] + offset)[0]
            slot = targets.slots.get(address)
            if (slot is None and value == target) or \
                    (slot is not None and slot['kind'] == 8 and
                     slot['address'] == target and value == 0):
                found.append(address)
        require(len(found) == 1, 'constructor target not unique in linked init_array')
        entries.append(dict(target=target, slot=found[0]))
    return entries


def inspect_linked_object(object_path, binary_path, anchor):
    object_path = Path(object_path).resolve(strict=True)
    binary_path = Path(binary_path).resolve(strict=True)
    object_content = object_path.read_bytes()
    binary_content = binary_path.read_bytes()
    obj = Elf(object_content, {1})
    binary = Elf(binary_content, {2, 3})
    require(all(not section['flags'] & SHF_EXECINSTR or section['name'] == '.text'
                for section in obj.sections), 'object has executable code outside .text')
    allowed_relocation_targets = {obj.only('.text')['index']}
    for name in ('.init_array', '.eh_frame'):
        if name in obj.by_name:
            allowed_relocation_targets.add(obj.only(name)['index'])
    require(all(section['info'] in allowed_relocation_targets
                for section in obj.sections if section['kind'] == SHT_RELA),
            'unsupported object relocation section')
    anchors = [symbol for symbol in obj.symbols if symbol['name'] == anchor
               and symbol['type'] == STT_FUNC and symbol['owner'] == obj.only('.text')['index']
               and symbol['size'] > 0]
    require(len(anchors) == 1, 'missing/ambiguous object anchor')
    matches = symbol_addresses(binary, anchor, STT_FUNC)
    require(len(matches) == 1, 'missing/ambiguous linked anchor')
    address, size = next(iter(matches))
    require(size == anchors[0]['size'], 'linked anchor size differs')
    base = address - anchors[0]['value']
    require(base >= 0, 'invalid linked text base')
    targets = ElfTargets(binary_content)
    source, normalized, bases, relocations = verify_text(obj, binary, targets, base, anchor)
    constructors = verify_constructors(obj, binary, targets, bases)
    return dict(schema_version=1, procedure='ELF64-X86_64-LINKED-OBJECT-BODY-v1',
                object=str(object_path), object_sha256=digest(object_content),
                binary=str(binary_path), binary_sha256=digest(binary_content),
                helper_sha256=digest(Path(__file__).read_bytes()), anchor=anchor,
                source_code_sha256=digest(source), normalized_code_sha256=digest(normalized),
                section_bases=bases, relocations=relocations, constructors=constructors,
                scope='contiguous O0 .text body, relocations, .rodata/.data/.bss and constructors; '
                      'source-to-object build provenance is checked separately')


def verify(binary: Path, object_file: Path, anchor: str):
    """Return a JSON-serializable proof or raise ValueError; never execute ELF."""
    return inspect_linked_object(object_file, binary, anchor)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--object', required=True, type=Path)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--anchor', required=True)
    args = parser.parse_args()
    print(json.dumps(inspect_linked_object(args.object, args.binary, args.anchor), indent=2))


if __name__ == '__main__':
    main()
