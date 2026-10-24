#!/usr/bin/env python3
"""Build a native binary archive, verify extraction and emit its checksum."""
import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import tarfile
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--verification', required=True, type=Path,
                        help='Successful tools/check.py summary for these exact release bytes')
    parser.add_argument('--output-dir', type=Path, default=ROOT/'dist')
    args = parser.parse_args()
    record = json.loads(args.verification.read_text())
    if record.get('passed') is not True:
        parser.error('verification report did not pass')
    subprocess.run(['cargo', 'build', '--release', '--locked'], cwd=ROOT, check=True)
    version = tomllib.loads((ROOT/'Cargo.toml').read_text())['package']['version']
    name = f'rless-{version}-linux-x86_64'
    data = {f'bin/{binary}': (ROOT/f'target/release/{binary}').read_bytes()
            for binary in ('rless', 'lessecho')}
    if sha(data['bin/rless']) != record['binaries']['release']['sha256']:
        parser.error('release binary changed since verification; run tools/check.py again')
    if sha(data['bin/lessecho']) != record['binaries']['release'].get('companion_sha256'):
        parser.error('companion verification is missing or stale; run tools/check.py again')
    helper_bytes = (ROOT/'vendor/less-helpers/less-osc8-open').read_bytes()
    if sha(helper_bytes) != record.get('helpers', {}).get('osc8_sha256'):
        parser.error('helper verification is missing or stale; run tools/check.py again')
    elf = subprocess.check_output(['readelf', '--version-info', str(ROOT/'target/release/rless')], text=True)
    versions = {value for value in re.findall(r'GLIBC_([0-9.]+)', elf)}
    dependencies = subprocess.check_output(['readelf', '--dynamic', str(ROOT/'target/release/rless')], text=True)
    manifest = dict(package='rless', version=version, platform=platform.system(), architecture=platform.machine(),
                    upstream_less='712', glibc_symbol_floor=max(versions, key=lambda v: tuple(map(int, v.split('.')))),
                    needed_libraries=re.findall(r'\(NEEDED\).*?\[([^\]]+)\]', dependencies),
                    binaries={path: sha(value) for path, value in data.items()},
                    helpers={'libexec/less-osc8-open': sha(helper_bytes)},
                    verification_sha256=sha(args.verification.read_bytes()),
                    scope='Native unsafe translation; finite compatibility tests are not a memory-safety or universal-equivalence proof.')
    data['manifest.json'] = (json.dumps(manifest, indent=2)+'\n').encode()
    data['verification.json'] = args.verification.read_bytes()
    data['libexec/less-osc8-open'] = helper_bytes
    data['activate.sh'] = b'''# Source with Bash to use this extracted package and its helpers.
_rless_package_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
export PATH="$_rless_package_dir/bin:$PATH"
if [[ ! ${LESSECHO+x} ]]; then
    printf -v LESSECHO '%q' "$_rless_package_dir/bin/lessecho"
    export LESSECHO
fi
if [[ ! ${LESS_OSC8_OPEN_ANY+x} ]]; then
    printf -v LESS_OSC8_OPEN_ANY '%q' "$_rless_package_dir/libexec/less-osc8-open"
    export LESS_OSC8_OPEN_ANY
fi
unset _rless_package_dir
'''
    data['README.txt'] = f'''rless {version}, native Linux x86_64 build

Source ./activate.sh with Bash, then run rless FILE or pipe bytes into rless.
This selects the extracted executables and their helpers without installing them.
The ./bin/lessecho companion is included. No C less executable is required.
The OSC 8 local-file/man helper is in libexec/less-osc8-open.
For a system /usr/local install, place rless in bin/ and the helpers in libexec/;
custom installations can set LESSECHO and LESS_OSC8_OPEN_ANY explicitly.

This build needs glibc symbols through {manifest['glibc_symbol_floor']} and the
shared libraries listed in manifest.json. It was tested on the build host;
other environments need their own verification.

The upstream less 712 version/help text is retained for compatibility comparisons.
This is an unsafe translated baseline. See verification.json for actual checks.
Project repository: https://github.com/JLuethe/rless
Licenses and attribution are in licenses/.
'''.encode()
    for destination, source in [('GPL-3.0.txt', ROOT/'LICENSE'),
                                ('Less.txt', ROOT/'LICENSE-LESS'),
                                ('C2Rust-BSD.txt', ROOT/'vendor/c2rust-bitfields/LICENSE')]:
        data[f'licenses/{destination}'] = source.read_bytes()
    directory = args.output_dir.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    archive = directory/f'{name}.tar.gz'
    epoch = int(os.environ.get('SOURCE_DATE_EPOCH', '0'))
    with tempfile.NamedTemporaryFile(dir=directory, prefix='.package-', suffix='.tmp') as temporary:
        with gzip.GzipFile(filename='', fileobj=temporary, mode='wb', mtime=epoch) as compressed:
            with tarfile.open(fileobj=compressed, mode='w', format=tarfile.PAX_FORMAT) as tar:
                for path, content in sorted(data.items()):
                    info = tarfile.TarInfo(f'{name}/{path}')
                    info.size = len(content)
                    info.mode = 0o755 if path.startswith(('bin/', 'libexec/')) else 0o644
                    info.mtime = epoch
                    tar.addfile(info, io.BytesIO(content))
        temporary.flush()
        # Test bytes read from the archive, including executable permissions.
        with tempfile.TemporaryDirectory(prefix='rless-package-test-') as extracted:
            with tarfile.open(temporary.name) as tar:
                tar.extractall(extracted, filter='data')
            package = Path(extracted)/name
            for path, content in data.items():
                assert (package/path).read_bytes() == content, path
            env = dict(PATH='/nonexistent', HOME=extracted, LC_ALL='C', TERM='dumb', LESSHISTFILE='-')
            payload = bytes(range(256))+b'last line'
            result = subprocess.run([str(package/'bin/rless')], input=payload, capture_output=True,
                                    env=env, timeout=5, check=True)
            assert result.stdout == payload and not result.stderr
            echo = subprocess.run([str(package/'bin/lessecho'), 'plain'], capture_output=True,
                                  env=env, timeout=5, check=True)
            assert echo.stdout == b'plain\n' and not echo.stderr
            local_file = Path(extracted)/'local-data'
            local_file.write_bytes(payload)
            linked = subprocess.run([str(package/'libexec/less-osc8-open'), local_file.as_uri()],
                                    capture_output=True, env={**env, 'PATH': '/usr/bin:/bin'},
                                    timeout=5, check=True)
            assert linked.stdout == payload and not linked.stderr
        archive.write_bytes(Path(temporary.name).read_bytes())
    checksum = sha(archive.read_bytes())
    archive.with_suffix(archive.suffix+'.sha256').write_text(f'{checksum}  {archive.name}\n')
    source_name = f'rless-{version}-source'
    source_archive = directory/f'{source_name}.tar.gz'
    source_files = [ROOT/name for name in ('.gitignore', 'Cargo.toml', 'Cargo.lock', 'build.rs',
                                          'README.md', 'LICENSE', 'LICENSE-LESS')]
    for top in ('src', 'tests', 'tools', 'vendor', 'docs', 'spec', 'compat'):
        for parent, directories, files in os.walk(ROOT/top):
            directories[:] = [d for d in directories if d not in ('.runs', '__pycache__', '.git')]
            source_files.extend(Path(parent)/file for file in files if not (Path(parent)/file).is_symlink())
    with source_archive.open('wb') as target:
        with gzip.GzipFile(filename='', fileobj=target, mode='wb', mtime=epoch) as compressed:
            with tarfile.open(fileobj=compressed, mode='w', format=tarfile.PAX_FORMAT) as tar:
                for path in sorted(source_files):
                    content = path.read_bytes()
                    info = tarfile.TarInfo(f'{source_name}/{path.relative_to(ROOT)}')
                    info.size = len(content)
                    info.mode = 0o755 if path.stat().st_mode & 0o111 else 0o644
                    info.mtime = epoch
                    tar.addfile(info, io.BytesIO(content))
    source_checksum = sha(source_archive.read_bytes())
    source_archive.with_suffix(source_archive.suffix+'.sha256').write_text(f'{source_checksum}  {source_archive.name}\n')
    print(json.dumps(dict(archive=str(archive), sha256=checksum,
                         source_archive=str(source_archive), source_sha256=source_checksum,
                         extracted_smoke_tests='passed')))


if __name__ == '__main__':
    main()
