#!/usr/bin/env python3
"""Install the exact M0 tools into an isolated directory without shell edits."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tomllib

PINS = json.loads((Path(__file__).parent / 'profiles.json').read_text())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--tool-home', type=Path, required=True)
    args = parser.parse_args()
    home = args.tool_home.resolve()
    download = home / 'downloads'
    download.mkdir(parents=True, exist_ok=True)
    init = download / ('rustup-init-' + PINS['rustup_version'])
    url = 'https://static.rust-lang.org/rustup/archive/' + PINS['rustup_version'] + '/' + PINS['target'] + '/rustup-init'
    manifest = download / ('channel-rust-' + PINS['nightly'] + '.toml')
    for path, address, expected in [(init, url, PINS['rustup_init_sha256']),
           (manifest, 'https://static.rust-lang.org/dist/' + PINS['nightly'].removeprefix('nightly-') + '/channel-rust-nightly.toml', PINS['channel_manifest_sha256'])]:
        if not path.exists():
            subprocess.run(['curl', '--proto', '=https', '--tlsv1.2', '-fsSL', address, '-o', path], check=True)
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError('download does not match pin: ' + str(path))
    channel = tomllib.loads(manifest.read_text())
    for component in ('miri-preview', 'rust-src', 'llvm-tools-preview'):
        target = '*' if component == 'rust-src' else PINS['target']
        if not channel['pkg'][component]['target'][target]['available']:
            raise ValueError('required component unavailable in pinned manifest')
    env = {**os.environ, 'RUSTUP_HOME': str(home / 'rustup'), 'CARGO_HOME': str(home / 'cargo')}
    init.chmod(0o755)
    rustup = home / 'cargo/bin/rustup'
    if not rustup.exists():
        subprocess.run([init, '-y', '--no-modify-path', '--default-toolchain', 'none', '--profile', 'minimal'], env=env, check=True)
    subprocess.run([rustup, 'set', 'auto-self-update', 'disable'], env=env, check=True)
    subprocess.run([rustup, 'toolchain', 'install', PINS['nightly'], '--profile', 'minimal',
                    '--component', ','.join(PINS['components'])], env=env, check=True)
    # Reuse an explicitly supplied Cargo cache; do not change the normal compiler.
    env['CARGO_HOME'] = os.environ.get('CARGO_HOME', str(home / 'cargo'))
    env['RUSTUP_TOOLCHAIN'] = PINS['nightly']
    env['PATH'] = str(home / 'cargo/bin') + os.pathsep + env.get('PATH', '')
    subprocess.run(['cargo', 'install', 'cargo-fuzz', '--version', PINS['cargo_fuzz_version'],
                    '--locked', '--root', home / 'fuzz'], env=env, check=True)
    subprocess.run(['cargo', 'miri', 'setup'], env=env, check=True)
    print('Installed isolated tools at ' + str(home))


if __name__ == '__main__':
    main()
