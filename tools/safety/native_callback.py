"""Bind a test-only tputs failure to the pager's exact cost-count callback."""
import hashlib
from pathlib import Path
import re

from native_alloc import ElfTargets

SOURCE = Path(__file__).resolve().parent / 'native/io_fault.c'
SYMBOL = r'_ZN5rless6screen13inc_costcount17h[0-9a-f]{16}E'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def bind(binary):
    content = Path(binary).read_bytes()
    if (content[:6] != b'\x7fELF\x02\x01' or int.from_bytes(content[16:18], 'little') != 3
            or int.from_bytes(content[18:20], 'little') != 62):
        raise ValueError('callback counterpart requires little-endian x86_64 PIE')
    symbols = ElfTargets(content).symbols
    matches = [(name, address) for name, addresses in symbols.items()
               if re.fullmatch(SYMBOL, name) for address in addresses]
    if len(matches) != 1 or not 0 < matches[0][1] < (1 << 64):
        raise ValueError('missing or ambiguous cost-count callback symbol')
    return dict(schema_version=1, binary_sha256=hashlib.sha256(content).hexdigest(),
                symbol=matches[0][0], offset=matches[0][1],
                helper_sha256=sha(__file__), shim_source_sha256=sha(SOURCE))


def validate_shape(binding):
    if (not isinstance(binding, dict) or set(binding) != {'schema_version', 'binary_sha256',
            'symbol', 'offset', 'helper_sha256', 'shim_source_sha256'}
            or type(binding['schema_version']) is not int or binding['schema_version'] != 1
            or type(binding['offset']) is not int or not 0 < binding['offset'] < (1 << 64)
            or not isinstance(binding['symbol'], str) or not re.fullmatch(SYMBOL, binding['symbol'])
            or any(not isinstance(binding[key], str) or not re.fullmatch('[0-9a-f]{64}', binding[key])
                   for key in ('binary_sha256', 'helper_sha256', 'shim_source_sha256'))
            or binding['helper_sha256'] != sha(__file__)
            or binding['shim_source_sha256'] != sha(SOURCE)):
        raise ValueError('invalid native callback identity binding')


def validate_observations(report, resolve=Path):
    """Re-derive each binding from actual retained ELF bytes, never run the ELF."""
    checked = {}
    for record in report['records']:
        for observation in record['observations']:
            binding = observation.get('native_callback_binding')
            if binding is None:
                continue  # Marker-level validation rejects unbound callback hits.
            validate_shape(binding)
            binary = observation['binary']
            if binary not in checked:
                checked[binary] = bind(resolve(binary))
            if checked[binary] != binding:
                raise ValueError('native callback binding differs from retained pager ELF')
