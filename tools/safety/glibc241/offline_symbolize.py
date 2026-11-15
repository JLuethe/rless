"""Resolve retained ASan module-relative PCs without loading target libraries."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

FRAME = re.compile(r'^\s*#(?P<index>\d+)\s+0x[0-9a-fA-F]+.*?\((?P<module>/[^\n]+?)\+(?P<offset>0x[0-9a-fA-F]+)\)', re.M)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def symbolize(raw_path, objects, symbolizer, pin):
    """Objects map original target paths to immutable host copies and SHA-256s."""
    symbolizer = Path(symbolizer).resolve(strict=True)
    if sha(symbolizer) != pin['sha256']:
        raise ValueError('offline symbolizer differs from exact pin')
    frames = []
    for match in FRAME.finditer(Path(raw_path).read_text(errors='replace')):
        module, offset = match['module'], match['offset']
        frame = dict(index=int(match['index']), target_object=module, offset=offset)
        binding = objects.get(module)
        if binding is None:
            frame.update(status='unresolved', reason='object absent from target object manifest')
        else:
            path = Path(binding['host_path']).resolve(strict=True)
            if sha(path) != binding['sha256']:
                raise ValueError('target object changed before symbolization: ' + module)
            command = [str(symbolizer), '--no-debuginfod', '--output-style=JSON',
                       '--inlines', '--demangle', '--obj=' + str(path), offset]
            env = dict(PATH='/usr/bin:/bin', LANG='C', LC_ALL='C', DEBUGINFOD_URLS='')
            completed = subprocess.run(command, capture_output=True, text=True, env=env,
                                       timeout=30, check=True)
            value = json.loads(completed.stdout)
            if not isinstance(value, list) or len(value) != 1 or 'Symbol' not in value[0]:
                raise ValueError('unexpected llvm-symbolizer JSON')
            symbols = value[0]['Symbol']
            resolved = any(x.get('FileName') not in (None, '', '??') and x.get('Line', 0) > 0 for x in symbols)
            frame.update(status='resolved' if resolved else 'unresolved', symbols=symbols,
                         object_sha256=binding['sha256'], command=command,
                         stderr=completed.stderr)
        frames.append(frame)
    return dict(raw_path=str(raw_path), raw_sha256=sha(raw_path), frames=frames,
                symbolizer_sha256=pin['sha256'], symbolizer_path=str(symbolizer))


def require_source(result, suffix):
    """A successful tool exit or native frame cannot satisfy an application control."""
    if not any(symbol.get('FileName', '').endswith(suffix) and symbol.get('Line', 0) > 0
               for frame in result['frames'] if frame['status'] == 'resolved'
               for symbol in frame['symbols']):
        raise ValueError('positive detector did not resolve intended source: ' + suffix)
