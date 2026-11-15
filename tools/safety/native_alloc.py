#!/usr/bin/env python3
"""Bind test-only calloc/strdup wrappers to exact pager owners without allocator bypass.

No target binary executes here. Build with a standalone C object and link only a
separate normal-source test pager using --wrap=calloc and --wrap=strdup, frame pointers and opt-level
0. A retained binding is required before a harness supplies plan_environment().
Release/native/ASAN production binaries remain separate. Claude executes controls.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import struct

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'tools/safety/native/alloc_fault.c'
PROCEDURE = 'SCOPED-NATIVE-ALLOC-STRDUP-v2'
RUSTFLAGS = '-Cforce-frame-pointers=yes -Copt-level=0 -Ccodegen-units=1'
CFLAGS = ['-std=c11', '-Wall', '-Wextra', '-Werror', '-O0', '-fPIC',
          '-fno-omit-frame-pointer', '-fno-optimize-sibling-calls']


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def capture(command):
    return subprocess.check_output([str(value) for value in command], text=True, stderr=subprocess.STDOUT)


def build_object(output, cc='cc'):
    output = Path(output).resolve()
    if output.exists():
        raise ValueError('counterpart object output already exists')
    output.parent.mkdir(parents=True, exist_ok=True)
    command = [cc, *CFLAGS, '-c', str(SOURCE), '-o', str(output)]
    subprocess.run(command, check=True)
    return dict(path=str(output), sha256=sha(output), source_sha256=sha(SOURCE),
                command=command, compiler=capture([cc, '--version']))


def _instructions(disassembly):
    return [match.group(1).strip() for line in disassembly.splitlines()
            if (match := re.match(r'^\s*[0-9a-f]+:\s+(.+)$', line))]


def _frame_pointer(disassembly, symbol):
    instructions = _instructions(disassembly)
    if instructions and instructions[0] == 'endbr64':
        instructions = instructions[1:]
    if len(instructions) < 2 or not re.fullmatch(r'push\s+%rbp', instructions[0]) or not re.fullmatch(
            r'mov\s+%rsp,%rbp', instructions[1]):
        raise ValueError('verified frame-pointer prologue missing: ' + symbol)


class ElfTargets:
    """Resolve only explicit x86_64 ELF symbol/GOT relocations; never load ELF."""
    def __init__(self, content):
        self.content = content
        if len(content) < 64:
            raise ValueError('truncated ELF header')
        offset = struct.unpack_from('<Q', content, 40)[0]
        size, count = struct.unpack_from('<HH', content, 58)
        if size != 64 or not count or offset + size * count > len(content):
            raise ValueError('unsupported/truncated ELF section table')
        self.sections = [struct.unpack_from('<IIQQQQIIQQ', content, offset + size * i) for i in range(count)]
        self.symbols, tables = {}, {}
        for index, section in enumerate(self.sections):
            if section[1] not in (2, 11):
                continue
            if section[9] != 24 or section[5] % 24 or section[6] >= count:
                raise ValueError('invalid ELF symbol table')
            strings = self.bytes(self.sections[section[6]])
            table = []
            raw = self.bytes(section)
            for start in range(0, len(raw), 24):
                name, info, other, owner, value, length = struct.unpack_from('<IBBHQQ', raw, start)
                if name >= len(strings):
                    raise ValueError('invalid ELF symbol name')
                end = strings.find(b'\0', name)
                if end < 0:
                    raise ValueError('unterminated ELF symbol name')
                name = strings[name:end].decode('utf-8', errors='strict')
                symbol = dict(name=name, address=value, defined=owner != 0)
                table.append(symbol)
                if name and owner != 0:
                    self.symbols.setdefault(name, set()).add(value)
            tables[index] = table
        self.slots = {}
        for section in self.sections:
            if section[1] != 4:
                continue
            if section[9] != 24 or section[5] % 24 or section[6] not in tables:
                raise ValueError('invalid ELF RELA table')
            raw = self.bytes(section)
            for start in range(0, len(raw), 24):
                address, info, addend = struct.unpack_from('<QQq', raw, start)
                kind, symbol_index = info & 0xffffffff, info >> 32
                if symbol_index >= len(tables[section[6]]):
                    raise ValueError('invalid relocation symbol')
                symbol = tables[section[6]][symbol_index]
                if kind == 8:  # R_X86_64_RELATIVE: load base plus exact addend.
                    resolved = dict(address=addend, name=None, kind=kind)
                elif kind in (1, 6, 7):  # 64, GLOB_DAT, JUMP_SLOT.
                    resolved = dict(address=symbol['address'] + addend if symbol['defined'] else None,
                                    name=symbol['name'], kind=kind)
                else:
                    resolved = dict(address=None, name=None, kind=kind)
                if address in self.slots:
                    raise ValueError('ambiguous relocation for GOT slot')
                self.slots[address] = resolved

    def bytes(self, section):
        start, size = section[4], section[5]
        if start + size > len(self.content):
            raise ValueError('ELF section extends beyond file')
        return self.content[start:start + size]

    def matches_address(self, address, names):
        return any(address in self.symbols.get(name, set()) for name in names)

    def matches_slot(self, address, names):
        relocation = self.slots.get(address)
        if relocation is not None:
            if relocation['address'] is not None:
                return self.matches_address(relocation['address'], names)
            return relocation['kind'] in (1, 6, 7) and relocation['name'] in names
        # Non-PIE static GOT entries can contain the final address directly.
        for section in self.sections:
            if section[1] != 8 and section[3] <= address and address + 8 <= section[3] + section[5]:
                value = struct.unpack_from('<Q', self.bytes(section), address - section[3])[0]
                return self.matches_address(value, names)
        return False

    def calls(self, text, names):
        rows = _instructions(text)
        for index, instruction in enumerate(rows):
            direct = re.search(r'\bcall\s+([0-9a-f]+)\s+<([^>]+)>', instruction)
            if direct and (self.matches_address(int(direct[1], 16), names) or
                           (direct[2].removesuffix('@plt') in names and direct[2].endswith('@plt'))):
                # PLT annotations alone are not enough: prove their GOT jump.
                if self.matches_address(int(direct[1], 16), names):
                    return True
                continue
            indirect = re.search(r'\bcall\s+\*[^#]*\(%rip\)\s*#\s*([0-9a-f]+)', instruction)
            if indirect and self.matches_slot(int(indirect[1], 16), names):
                return True
            register = re.fullmatch(r'call\s+\*(%[a-z0-9]+)', instruction)
            if register and index:
                load = re.search(r'\b(mov|lea)\s+[^,]*\(%rip\),' + re.escape(register[1]) +
                                 r'\s*#\s*([0-9a-f]+)', rows[index - 1])
                if load and ((load[1] == 'mov' and self.matches_slot(int(load[2], 16), names)) or
                             (load[1] == 'lea' and self.matches_address(int(load[2], 16), names))):
                    return True
        return False


def inspect_binary(binary, profile):
    binary = Path(binary).resolve(strict=True)
    if profile not in ('native', 'asan'):
        raise ValueError('unknown counterpart instrumentation profile')
    content = binary.read_bytes()
    if content[:6] != b'\x7fELF\x02\x01' or int.from_bytes(content[18:20], 'little') != 62:
        raise ValueError('counterpart must be a little-endian x86_64 ELF')
    if b'RLESS_FAULT_FD' in content:
        raise ValueError('native counterpart must not contain production injection hooks')
    symbols = capture(['nm', '-S', '--defined-only', binary])
    selected = {}
    for name in ('ecalloc', 'xbuf_add_byte', 'parse_lesskey', '__wrap_calloc', '__wrap_strdup'):
        rows = re.findall(r'^([0-9a-f]+)\s+([0-9a-f]+)\s+[Tt]\s+' + re.escape(name) + r'$', symbols, re.M)
        if len(rows) != 1 or not 0 < int(rows[0][1], 16) <= 1048576:
            raise ValueError('missing or ambiguous nonempty scoped symbol: ' + name)
        selected[name] = dict(address=int(rows[0][0], 16), size=int(rows[0][1], 16))
    instrumented = '__asan_' in symbols
    if instrumented != (profile == 'asan'):
        raise ValueError('counterpart binary instrumentation differs from requested profile')
    disassembly = {}
    for name in selected:
        text = capture(['objdump', '-d', '--no-show-raw-insn', '--disassemble=' + name, binary])
        _frame_pointer(text, name)
        disassembly[name] = text
    targets = ElfTargets(content)
    if not targets.calls(disassembly['ecalloc'], {'__wrap_calloc'}):
        raise ValueError('ecalloc does not demonstrably call the linked calloc wrapper')
    if not targets.calls(disassembly['xbuf_add_byte'], {'ecalloc'}):
        raise ValueError('growth owner does not demonstrably call the verified ecalloc frame')
    if not targets.calls(disassembly['parse_lesskey'], {'__wrap_strdup'}):
        raise ValueError('parser pre-init does not demonstrably call the linked strdup wrapper')
    for function in ('calloc', 'strdup'):
        real_names = {function, '__interceptor_' + function, '___interceptor_' + function}
        if profile == 'asan' and not targets.symbols.get(function):
            raise ValueError('ASAN counterpart does not define the sanitizer ' + function + ' interceptor')
        wrapper = disassembly['__wrap_' + function]
        forwarding = targets.calls(wrapper, real_names)
        if not forwarding:
            # Prove each printed PLT candidate through its actual GOT relocation.
            for address, name in re.findall(r'\bcall\s+([0-9a-f]+)\s+<([^>]+)@plt>', wrapper):
                if name not in real_names:
                    continue
                plt = capture(['objdump', '-d', '--no-show-raw-insn', '--start-address=' + str(int(address,16)),
                               '--stop-address=' + str(int(address,16)+16), binary])
                for slot in re.findall(r'\bjmp\s+\*[^#]*\(%rip\)\s*#\s*([0-9a-f]+)', plt):
                    forwarding = forwarding or targets.matches_slot(int(slot,16), real_names)
        if not forwarding:
            raise ValueError('wrapper forwarding to the real/sanitizer ' + function + ' is not visible')
        if '__libc_' in wrapper:
            raise ValueError('counterpart would bypass the active allocator')
    return dict(schema_version=1, procedure=PROCEDURE, binary=str(binary), binary_sha256=sha(binary),
                profile=profile, source_sha256=sha(SOURCE), helper_sha256=sha(__file__),
                symbols=selected, disassembly=disassembly, rustflags_required=RUSTFLAGS,
                scope='ecalloc boundary; reserve is downstream xbuf allocation; parser is fixture-specific pre-init strdup failure')


def validate_binding(binding, binary=None):
    if binding.get('procedure') != PROCEDURE or binding.get('schema_version') != 1:
        raise ValueError('unsupported native allocation binding')
    if binary is not None and Path(binary).resolve() != Path(binding['binary']).resolve():
        raise ValueError('binding selects a different counterpart binary')
    if binding != inspect_binary(binding['binary'], binding['profile']):
        raise ValueError('native allocation counterpart/source/disassembly binding changed')
    return binding


def validate_retained_binding(binding, retained_binary):
    """Recheck a copied target ELF without executing it or loading target libs."""
    retained = Path(retained_binary).resolve(strict=True)
    current = inspect_binary(retained, binding['profile'])
    current['binary'] = binding['binary']
    current['disassembly'] = {name:text.replace(str(retained), binding['binary'])
                              for name,text in current['disassembly'].items()}
    if current != binding:
        raise ValueError('retained native allocation binding no longer reproduces')
    return binding


def plan_environment(binding, site, ordinal, trace_fd):
    validate_binding(binding)
    if site not in ('alloc', 'reserve', 'parser') or type(ordinal) is not int or not 0 < ordinal <= (1 << 64) - 1:
        raise ValueError('invalid scoped allocation plan')
    if type(trace_fd) is not int or not 0 < trace_fd <= (1 << 31) - 1:
        raise ValueError('native allocation marker requires a positive inherited pipe descriptor')
    if site == 'parser':
        if ordinal != 1:
            raise ValueError('pre-init parser counterpart supports only ordinal 1')
        return dict(RLESS_NATIVE_PARSER_FD=str(trace_fd))
    return dict(RLESS_NATIVE_ALLOC_SITE=site, RLESS_NATIVE_ALLOC_ORDINAL=str(ordinal),
                RLESS_NATIVE_ALLOC_FD=str(trace_fd),
                RLESS_NATIVE_ALLOC_ECALLOC_SIZE=str(binding['symbols']['ecalloc']['size']),
                RLESS_NATIVE_ALLOC_XBUF_SIZE=str(binding['symbols']['xbuf_add_byte']['size']))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--build-object', type=Path)
    group.add_argument('--binary', type=Path)
    parser.add_argument('--cc', default='cc')
    parser.add_argument('--profile', choices=('native', 'asan'), default='native')
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    if args.output.exists():
        parser.error('binding output already exists')
    value = build_object(args.build_object, args.cc) if args.build_object else inspect_binary(args.binary, args.profile)
    with args.output.open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
    print(str(args.output.resolve()))


if __name__ == '__main__':
    main()
