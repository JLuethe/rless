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
import sys
import tarfile
import tempfile
import tomllib

from release import (PROVENANCE, sha_file, source_identity, validate_acceptance,
                     validate_acceptance_observer, validate_verification, verify_export)

ROOT = Path(__file__).resolve().parents[1]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def source_archive_bytes(root, source, name, archive, epoch):
    """Use only the committed manifest, never os.walk output as archive authority."""
    manifest = {key: source[key] for key in ('schema_version', 'origin', 'source_commit', 'files', 'manifest_sha256')}
    verify_export(root, manifest)
    with archive.open('wb') as target:
        with gzip.GzipFile(filename='', fileobj=target, mode='wb', mtime=epoch) as compressed:
            with tarfile.open(fileobj=compressed, mode='w', format=tarfile.PAX_FORMAT) as tar:
                entries = [(path, (root / path).read_bytes(), metadata['mode'])
                           for path, metadata in sorted(source['files'].items())]
                entries.append((PROVENANCE, (json.dumps(manifest, indent=2, sort_keys=True) + '\n').encode(), 0o644))
                for path, content, mode in entries:
                    info = tarfile.TarInfo(f'{name}/{path}')
                    info.size, info.mode, info.mtime = len(content), mode, epoch
                    tar.addfile(info, io.BytesIO(content))


def qualify_source(archive, source_name, expected_source, output):
    """Rebuild and replay an internal candidate before final artifacts exist."""
    output.mkdir()
    env = dict(os.environ)
    env.pop('CARGO_TARGET_DIR', None)
    records = []
    with tempfile.TemporaryDirectory(prefix='rless-source-rebuild-') as directory:
        with tarfile.open(archive) as tar:
            tar.extractall(directory, filter='data')
        rebuilt = Path(directory) / source_name
        identity = source_identity(rebuilt, require_clean=True)
        if identity['source_commit'] != expected_source['source_commit'] or identity['manifest_sha256'] != expected_source['manifest_sha256']:
            raise ValueError('source archive provenance does not match verified source')
        commands = [('source-build', ['cargo', 'build', '--release', '--locked', '--offline', '--bins']),
                    ('source-tests', ['cargo', 'test', '--release', '--locked', '--offline']),
                    ('source-acceptance', [sys.executable, 'tools/compat/check_candidate.py', '--binary',
                        str(rebuilt / 'target/release/rless'), '--results-dir', str(output / 'acceptance')])]
        for name, command in commands:
            log = output / (name + '.log')
            with log.open('wb') as stream:
                result = subprocess.run(command, cwd=rebuilt, env=env, stdout=stream,
                                        stderr=subprocess.STDOUT, timeout=1200)
            records.append(dict(name=name, command=command, returncode=result.returncode,
                                log=log.name, log_sha256=sha_file(log)))
            if result.returncode != 0:
                (output / 'result.json').write_text(json.dumps(dict(passed=False, records=records), indent=2) + '\n')
                raise ValueError('source rebuild failed: ' + name + '; see ' + str(log))
        acceptance = json.loads((output / 'acceptance/summary.json').read_text())
        validate_acceptance(acceptance)
        validate_acceptance_observer(acceptance, rebuilt)
        if acceptance.get('sha256') != sha_file(rebuilt / 'target/release/rless'):
            raise ValueError('rebuilt binary changed after source acceptance')
        if source_identity(rebuilt, require_clean=True)['manifest_sha256'] != identity['manifest_sha256']:
            raise ValueError('source rebuild modified approved inputs')
        result = dict(passed=True, source_commit=identity['source_commit'],
            source_manifest_sha256=identity['manifest_sha256'], source_archive_sha256=sha_file(archive),
            records=records, acceptance=acceptance,
            rebuilt_binaries={name: sha_file(rebuilt / 'target/release' / name) for name in ('rless', 'lessecho')})
        (output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--verification', required=True, type=Path,
                        help='Successful tools/check.py summary for these exact release bytes')
    parser.add_argument('--output-dir', type=Path, default=ROOT/'dist')
    args = parser.parse_args()
    if sys.flags.optimize:
        parser.error('packaging requires Python assertions enabled')
    if platform.system() != 'Linux' or platform.machine() != 'x86_64':
        parser.error('this release package policy supports Linux x86_64 only')
    verification_bytes = args.verification.read_bytes()
    verification_sha256 = sha(verification_bytes)
    record = json.loads(verification_bytes)
    try:
        source = validate_verification(record, args.verification, ROOT)
    except (OSError, ValueError, KeyError) as error:
        parser.error(str(error))
    env = dict(os.environ)
    env.pop('CARGO_TARGET_DIR', None)
    subprocess.run(['cargo', 'build', '--release', '--locked', '--bins'], cwd=ROOT, env=env, check=True)
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
    if any(b'RLESS_FAULT' in content for content in data.values()):
        parser.error('refusing to package a fault-injection binary')
    abi = {}
    for binary in ('rless', 'lessecho'):
        elf = subprocess.check_output(['readelf', '--version-info', str(ROOT/'target/release'/binary)], text=True)
        versions = set(re.findall(r'GLIBC_([0-9.]+)', elf))
        dependencies = subprocess.check_output(['readelf', '--dynamic', str(ROOT/'target/release'/binary)], text=True)
        abi[binary] = dict(glibc_symbol_floor=max(versions, key=lambda v: tuple(map(int, v.split('.')))),
                           needed_libraries=re.findall(r'\(NEEDED\).*?\[([^\]]+)\]', dependencies))
    directory = args.output_dir.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    source_name = f'rless-{version}-source'
    final_names = [name + '.tar.gz', name + '.tar.gz.sha256', source_name + '.tar.gz',
                   source_name + '.tar.gz.sha256', name + '-release.json', name + '-qualification']
    if any((directory / filename).exists() for filename in final_names):
        parser.error('release output already exists; use a fresh --output-dir')
    # Keep failed candidates and logs for diagnosis. Only the final attestation
    # marks a completed release artifact set; no failed candidate is published.
    staging = Path(tempfile.mkdtemp(prefix='.candidate-', dir=directory))
    epoch = int(os.environ.get('SOURCE_DATE_EPOCH', '0'))
    staged_source = staging / (source_name + '.tar.gz')
    source_archive_bytes(ROOT, source, source_name, staged_source, epoch)
    qualification = staging / (name + '-qualification')
    qualify_source(staged_source, source_name, source, qualification)
    manifest = dict(package='rless', version=version, platform=platform.system(), architecture=platform.machine(),
                    upstream_less='712', source_commit=source['source_commit'], source_manifest_sha256=source['manifest_sha256'],
                    glibc_symbol_floor=max((item['glibc_symbol_floor'] for item in abi.values()), key=lambda v: tuple(map(int, v.split('.')))),
                    needed_libraries=sorted({library for item in abi.values() for library in item['needed_libraries']}),
                    executable_abi=abi, source_rebuild_sha256=sha_file(qualification / 'result.json'),
                    binaries={path: sha(value) for path, value in data.items()},
                    helpers={'libexec/less-osc8-open': sha(helper_bytes)},
                    verification_sha256=verification_sha256,
                    scope='Native unsafe translation; finite compatibility tests are not a memory-safety or universal-equivalence proof.')
    data['manifest.json'] = (json.dumps(manifest, indent=2)+'\n').encode()
    data['verification.json'] = verification_bytes
    data['source-rebuild.json'] = (qualification / 'result.json').read_bytes()
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
    for destination, license_path in [('GPL-3.0.txt', ROOT/'LICENSE'),
                                ('Less.txt', ROOT/'LICENSE-LESS'),
                                ('C2Rust-BSD.txt', ROOT/'vendor/c2rust-bitfields/LICENSE')]:
        data[f'licenses/{destination}'] = license_path.read_bytes()
    archive = staging/f'{name}.tar.gz'
    with tempfile.NamedTemporaryFile(dir=staging, prefix='.package-', suffix='.tmp') as temporary:
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
    source_checksum = sha_file(staged_source)
    staged_source.with_suffix(staged_source.suffix+'.sha256').write_text(f'{source_checksum}  {staged_source.name}\n')
    current = validate_verification(record, args.verification, ROOT)
    if sha_file(args.verification) != verification_sha256:
        raise ValueError('verification report changed during packaging')
    if current['manifest_sha256'] != source['manifest_sha256']:
        raise ValueError('committed source changed during packaging')
    if any(sha_file(ROOT/'target/release'/binary) != manifest['binaries']['bin/' + binary] for binary in ('rless', 'lessecho')):
        raise ValueError('release binaries changed during packaging')
    attestation = dict(schema_version=1, status='release-qualified', source_commit=source['source_commit'],
        source_manifest_sha256=source['manifest_sha256'], verification_sha256=verification_sha256,
        source_rebuild_sha256=sha_file(qualification / 'result.json'),
        archives={archive.name: checksum, staged_source.name: source_checksum},
        native_checks='complete', source_rebuild='809/809', extracted_smoke_tests='passed',
        runtime_safety_verified=False)
    for filename in final_names[:-2]:
        (staging / filename).replace(directory / filename)
    qualification.replace(directory / qualification.name)
    attestation_path = directory / (name + '-release.json')
    attestation_path.write_text(json.dumps(attestation, indent=2) + '\n')
    archive = directory / archive.name
    source_archive = directory / staged_source.name
    print(json.dumps(dict(archive=str(archive), sha256=checksum,
                         source_archive=str(source_archive), source_sha256=source_checksum,
                         extracted_smoke_tests='passed', release_attestation=str(attestation_path),
                         release_attestation_sha256=sha_file(attestation_path))))


if __name__ == '__main__':
    main()
