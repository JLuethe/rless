#!/usr/bin/env python3
"""Reference glibc ASan/LSan with pinned offline symbolization and retained raw logs."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from release import reference_policy, validate_reference_hashes

from offline_symbolize import require_source, symbolize
from snapshot import ROOT, archive, sha, validate


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def host_path(target_path, work):
    target = Path(target_path)
    if not target.is_relative_to('/work') or '..' in target.parts:
        raise ValueError('target artifact escapes isolated work: ' + target_path)
    candidate = (work / target.relative_to('/work')).resolve(strict=True)
    if not candidate.is_relative_to(work.resolve()):
        raise ValueError('target artifact resolves outside work')
    return candidate


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--image', default='localhost/rless-glibc241:20261020')
    parser.add_argument('--environment-lock', type=Path, default=ROOT / 'docs/verification/glibc241-environment.json')
    parser.add_argument('--source-manifest', type=Path, required=True)
    parser.add_argument('--tool-home', type=Path, required=True)
    parser.add_argument('--registry-cache', type=Path, required=True)
    parser.add_argument('--reference-dir', type=Path, required=True)
    parser.add_argument('--symbolizer', type=Path, default=Path('/usr/bin/llvm-symbolizer'))
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--repetitions', type=int, default=100)
    args = parser.parse_args()
    if args.repetitions < 1:
        parser.error('repetitions must be positive')
    os.umask(0o077)
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    inputs = out / 'input'
    inputs.mkdir()
    work = out / 'work'
    work.mkdir()
    record = dict(schema_version=1, status='not-verified', runtime_safety_verified=False,
                  procedure='REFERENCE-GLIBC-2.41-ASAN-OFFLINE-v1',
                  abi_profile='REFERENCE-GLIBC-2.41',
                  scope='Reference ABI sanitizer infrastructure; not whole-catalogue or independent M0 approval.')
    save(out / 'summary.json', record)
    try:
        snapshot = validate(json.loads(args.source_manifest.read_text()))
        archive(snapshot, inputs / 'source.tar')
        save(inputs / 'source-manifest.json', snapshot)
        pins = json.loads((ROOT / 'tools/safety/profiles.json').read_text())
        procedure = ROOT / 'tools/safety/glibc241/offline-symbolization-v1.json'
        record.update(source_commit=snapshot['base_commit'], source_kind=snapshot['source_kind'],
                      dirty_state=snapshot['dirty_state'], source_manifest=snapshot,
                      source_manifest_sha256=sha(inputs / 'source-manifest.json'),
                      source_tar_sha256=sha(inputs / 'source.tar'), tool_pins=pins,
                      procedure_sha256=sha(procedure), repetitions=args.repetitions)
        symbolizer = args.symbolizer.resolve(strict=True)
        if sha(symbolizer) != pins['symbolizer']['sha256']:
            raise ValueError('host symbolizer does not match the unchanged exact pin')
        version = subprocess.check_output([symbolizer, '--version'], text=True)
        if pins['symbolizer']['version'] not in version:
            raise ValueError('host symbolizer version mismatch')
        record['symbolizer'] = dict(path=str(symbolizer), sha256=sha(symbolizer), version=version,
                                   target_mount=False, use='offline metadata decoding only')
        toolchain = (args.tool_home.resolve() / 'rustup/toolchains' /
                     (pins['nightly'] + '-' + pins['target']))
        libraries = toolchain / 'lib/rustlib' / pins['target'] / 'lib'
        tools = [toolchain / 'bin/rustc', toolchain / 'bin/cargo',
                 libraries / 'librustc-nightly_rt.asan.a']
        tools.extend(sorted((toolchain / 'lib').glob('*.so*')))
        tools.extend(sorted(libraries.glob('libstd-*.rlib')))
        record['tool_files_sha256'] = {str(path): sha(path) for path in tools}
        lock = json.loads(args.environment_lock.read_text())
        image = json.loads(subprocess.check_output(['podman', 'image', 'inspect', args.image], text=True))[0]
        if image['Id'].removeprefix('sha256:') != lock['image_id'].removeprefix('sha256:'):
            raise ValueError('image ID differs from the explicit environment lock')
        if image.get('Architecture') != 'amd64' or image.get('RootFS') != lock['rootfs']:
            raise ValueError('image architecture/rootfs differs from lock')
        if sha(ROOT / 'tools/safety/glibc241/Containerfile') != lock['containerfile_sha256']:
            raise ValueError('Containerfile differs from the pinned environment lock')
        image_id = image['Id']
        record.update(image_id=image_id, image_rootfs=image['RootFS'],
                      environment_lock_sha256=sha(args.environment_lock))
        reference = out / 'reference'
        reference.mkdir()
        for name in ('less', 'lessecho', 'less-osc8-open.sh'):
            shutil.copy2(args.reference_dir.resolve() / name, reference / name)
        record['references'] = {path.name: sha(path) for path in reference.iterdir()}
        validate_reference_hashes(record['references'], ROOT)
        record['reference_policy'] = reference_policy(ROOT)
        save(inputs / 'options.json', dict(repetitions=args.repetitions))
        command = ['podman', 'run', '--rm', '--network=none', '--read-only', '--cap-drop=all', '--ulimit', 'core=0:0',
                   '--userns=keep-id', '--user', str(os.getuid()) + ':' + str(os.getgid()),
                   '--tmpfs', '/tmp:rw,nosuid,nodev',
                   '-v', str(inputs) + ':/input:ro', '-v', str(work) + ':/work:rw',
                   '-v', str(reference) + ':/reference:ro', '-v', str(toolchain) + ':/opt/rust:ro',
                   '-v', str(args.registry_cache.resolve(strict=True)) + ':/cache/registry:ro',
                   '--env', 'PATH=/opt/rless-python/bin:/opt/rust/bin:/usr/bin:/bin',
                   '--env', 'CARGO_HOME=/work/cargo', '--env', 'CARGO_NET_OFFLINE=true',
                   '--env', 'HOME=/work/home', '--env', 'PYTHONDONTWRITEBYTECODE=1', image_id,
                   'python3', '-c', "import tarfile; t=tarfile.open('/input/source.tar'); "
                   "s=t.extractfile('tools/safety/glibc241/sanitizers_inside.py').read(); "
                   "exec(compile(s,'sanitizers_inside.py','exec'), {'__name__':'__main__'})"]
        record['command'] = command
        save(out / 'summary.json', record)
        with (out / 'container.log').open('wb') as stream:
            completed = subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT, timeout=7200)
        record['original_exit_status'] = completed.returncode
        record['container_log_sha256'] = sha(out / 'container.log')
        inner_path = work / 'report/result.json'
        if not inner_path.is_file():
            raise ValueError('container did not retain an inner report')
        inner = json.loads(inner_path.read_text())
        record['inner_report_sha256'] = sha(inner_path)
        record['result'] = inner
        if inner.get('runtime') != 'glibc 2.41' or inner.get('header_probe', {}).get('glibc_headers') != '2.41':
            raise ValueError('inner report did not prove matching runtime and headers')
        objects = {}
        for module, entry in inner.get('objects', {}).items():
            local = host_path(entry['path'], work)
            if sha(local) != entry['sha256']:
                raise ValueError('retained target object changed: ' + module)
            objects[module] = dict(host_path=str(local), sha256=entry['sha256'])
        symbolized = []
        record['offline_symbolization'] = symbolized
        for control in inner.get('detector_controls', []):
            for raw in control['raw_reports']:
                path = host_path(raw['path'], work)
                if sha(path) != raw['sha256']:
                    raise ValueError('raw sanitizer report changed')
                resolved = symbolize(path, objects, symbolizer, pins['symbolizer'])
                if control['required_source_suffix']:
                    require_source(resolved, control['required_source_suffix'])
                symbolized.append(dict(control=control['name'], **resolved))
        if completed.returncode != 0 or inner.get('status') != 'passed':
            raise ValueError('reference ABI sanitizer lane did not pass; retained original result')
        if len(inner.get('detector_controls', [])) != 6 or len(symbolized) < 4:
            raise ValueError('safe and positive pipe/PTY detector controls are incomplete')
        if inner['source_manifest_sha256'] != record['source_manifest_sha256']:
            raise ValueError('container tested a different source snapshot')
        for path, digest in record['tool_files_sha256'].items():
            if sha(path) != digest:
                raise ValueError('toolchain changed during campaign')
        if sha(symbolizer) != record['symbolizer']['sha256']:
            raise ValueError('symbolizer changed during campaign')
        record['status'] = 'passed'
    except Exception as error:
        record['error'] = type(error).__name__ + ': ' + str(error)
        print(record['error'], file=sys.stderr)
    finally:
        save(out / 'summary.json', record)
    print('glibc 2.41 sanitizer infrastructure: ' + record['status'] + '; ' + str(out / 'summary.json'))
    return 0 if record['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
