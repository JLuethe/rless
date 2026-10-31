#!/usr/bin/env python3
"""Freeze an explicit source manifest; never substitute a dirty tree for a commit."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[3]
SOURCE_ROOTS = {'src', 'tests', 'tools', 'vendor', 'compat', 'spec', 'docs', '.github'}
ROOT_FILES = {'.gitignore', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'README.md',
              'LICENSE', 'LICENSE-LESS', 'CHANGELOG.md', 'CONTRIBUTING.md', 'SECURITY.md',
              'rust-toolchain.toml', 'rust-toolchain'}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args])


def source_names(root):
    raw = git(root, 'ls-files', '-z', '--cached', '--others', '--exclude-standard')
    names = {os.fsdecode(name) for name in raw.split(b'\0') if name}
    generated = ('.git', '.venv', '__pycache__', '.runs', 'target')
    return sorted(name for name in names
                  if (name in ROOT_FILES or PurePosixPath(name).parts[0] in SOURCE_ROOTS)
                  if not any(part in generated for part in PurePosixPath(name).parts)
                  and not name.startswith(('tools/safety/core-fuzz/artifacts/',
                                           'tools/safety/core-fuzz/corpus/',
                                           'tools/safety/core-fuzz/coverage/')))


def checked_path(root, name):
    path = PurePosixPath(name)
    if path.is_absolute() or '..' in path.parts or not path.parts or str(path) != name:
        raise ValueError('unsafe manifest path: ' + name)
    if (name not in ROOT_FILES and path.parts[0] not in SOURCE_ROOTS) or any(
            part in ('.git', '.venv', '.aws', '.codex', '.agents', '__pycache__', '.runs', 'target')
            for part in path.parts):
        raise ValueError('build/private output cannot enter source snapshot: ' + name)
    result = root / name
    if result.is_symlink() or not result.is_file() or not result.resolve().is_relative_to(root.resolve()):
        raise ValueError('missing, linked or escaping source: ' + name)
    return result


def manifest(root=ROOT):
    root = Path(root).resolve()
    names = source_names(root)
    files = {}
    for name in names:
        path = checked_path(root, name)
        files[name] = dict(sha256=sha(path), mode=0o755 if path.stat().st_mode & 0o111 else 0o644)
    result = dict(schema_version=1, source_kind='explicit-working-tree-snapshot',
                  base_commit=git(root, 'rev-parse', 'HEAD').decode().strip(),
                  dirty_state=git(root, 'status', '--porcelain').decode(), files=files)
    result['files_sha256'] = hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest()
    validate(result, root)
    if names != source_names(root):
        raise ValueError('source selection changed while creating manifest')
    return result


def validate(value, root=ROOT, exact=False):
    if value.get('schema_version') != 1 or value.get('source_kind') != 'explicit-working-tree-snapshot':
        raise ValueError('unsupported snapshot schema')
    files = value.get('files')
    if not isinstance(files, dict) or not files or 'Cargo.toml' not in files:
        raise ValueError('empty or incomplete source manifest')
    digest = hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest()
    if value.get('files_sha256') != digest:
        raise ValueError('source manifest digest mismatch')
    for name, expected in files.items():
        path = checked_path(Path(root), name)
        if sha(path) != expected['sha256']:
            raise ValueError('source changed since manifest: ' + name)
        mode = 0o755 if path.stat().st_mode & 0o111 else 0o644
        if mode != expected['mode']:
            raise ValueError('source executable mode changed: ' + name)
    if exact:
        actual = {str(path.relative_to(root)) for path in Path(root).rglob('*') if path.is_file()}
        if actual != set(files):
            raise ValueError('extracted source contains missing or extra files')
    return value


def archive(value, destination, root=ROOT):
    root = Path(root).resolve()
    validate(value, root)
    if set(value['files']) != set(source_names(root)):
        raise ValueError('source selection changed since manifest')
    with tarfile.open(destination, 'x') as tar:
        for name, expected in sorted(value['files'].items()):
            content = checked_path(root, name).read_bytes()
            if hashlib.sha256(content).hexdigest() != expected['sha256']:
                raise ValueError('source changed during archive: ' + name)
            info = tarfile.TarInfo(name)
            info.mode = expected['mode']
            info.size = len(content)
            info.mtime = 0
            tar.addfile(info, io.BytesIO(content))
    validate(value, root)
    if set(value['files']) != set(source_names(root)):
        raise ValueError('source selection changed during archive')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    value = manifest()
    if args.output.resolve().is_relative_to(ROOT):
        relative = str(args.output.resolve().relative_to(ROOT))
        ignored = subprocess.run(['git', '-C', str(ROOT), 'check-ignore', '-q', relative]).returncode == 0
        if not ignored:
            parser.error('put the manifest outside the source tree or in an ignored run directory')
    with args.output.open('x') as out:
        json.dump(value, out, indent=2)
        out.write('\n')
    print(str(args.output.resolve()))


if __name__ == '__main__':
    main()
