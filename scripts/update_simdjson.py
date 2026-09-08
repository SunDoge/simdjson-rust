#!/usr/bin/env python3
"""Update vendored simdjson from an explicit upstream release tag (Python 3 + Git)."""
import argparse
import hashlib
import os
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parents[1]
UPSTREAM = 'https://github.com/simdjson/simdjson.git'
FILES = {'simdjson.h': 'singleheader/simdjson.h',
         'simdjson.cpp': 'singleheader/simdjson.cpp', 'LICENSE': 'LICENSE'}


def release_version(value):
    version = value.removeprefix('v')
    if not re.fullmatch(r'\d+\.\d+\.\d+(?:-[A-Za-z0-9.-]+)?', version):
        raise argparse.ArgumentTypeError('expected an explicit release version, e.g. 4.6.4')
    return version


def resolve_commit(version):
    ref = f'refs/tags/v{version}'
    result = subprocess.run(['git', 'ls-remote', '--tags', UPSTREAM, ref, ref + '^{}'],
                            check=True, text=True, capture_output=True, timeout=60)
    refs = dict((name, sha) for sha, name in
                (line.split() for line in result.stdout.splitlines()))
    commit = refs.get(ref + '^{}', refs.get(ref, ''))
    if not re.fullmatch(r'[0-9a-f]{40}', commit):
        raise ValueError(f'upstream tag v{version} was not found')
    return commit


def download(commit, path):
    # Resolve the tag once; every file is fetched from the same immutable commit.
    url = f'https://raw.githubusercontent.com/simdjson/simdjson/{commit}/{path}'
    with urlopen(url, timeout=60) as response:
        data = response.read()
    if not data:
        raise ValueError(f'empty upstream file: {path}')
    return data


def prepare(root, version, commit):
    vendor = root / 'simdjson-sys/vendor/simdjson'
    old_readme = (vendor / 'README.md').read_text(encoding="utf-8")
    match = re.search(r'simdjson v([^\s]+)\.', old_readme)
    if not match:
        raise ValueError('cannot read the current version from vendor README')
    old_version = match[1]
    sources = {name: download(commit, path) for name, path in FILES.items()}
    declared = re.search(rb'#define SIMDJSON_VERSION "([^"]+)"', sources['simdjson.h'])
    if not declared or declared[1].decode() != version:
        raise ValueError('downloaded header version does not match requested release')
    changes = {vendor / name: data for name, data in sources.items()}
    checksums = ''.join(f'| `{name}` | `{hashlib.sha256(data).hexdigest()}` |\n'
                        for name, data in sources.items())
    changes[vendor / 'README.md'] = (
        f'# Vendored simdjson\n\nOfficial singleheader sources from simdjson v{version}.\n'
        f'Upstream: https://github.com/simdjson/simdjson/tree/{commit}\n\n'
        'Files are unmodified; retain LICENSE when updating both files together.\n\n'
        'Update with `mise run update-simdjson <version>` from the repository root.\n\n'
        '| File | SHA-256 |\n| --- | --- |\n' + checksums
    ).encode()
    for name in ['README.md', 'simdjson-sys/README.md']:
        path = root / name
        text = path.read_text(encoding="utf-8")
        pattern = rf'\bv{re.escape(old_version)}(?![\w.-])'
        updated, count = re.subn(pattern, f'v{version}', text)
        if not count:
            raise ValueError(f'cannot find current simdjson version in {name}')
        changes[path] = updated.encode()
    if version != old_version:
        path = root / 'CHANGELOG.md'
        text = path.read_text(encoding="utf-8")
        marker = '## [Unreleased]\n'
        if text.count(marker) != 1:
            raise ValueError('expected one Unreleased section in CHANGELOG.md')
        entry = f'\n- Update bundled simdjson from v{old_version} to v{version}.\n'
        changes[path] = text.replace(marker, marker + entry, 1).encode()
    return changes


def apply(changes):
    # Download and validate everything before touching the working tree.
    # Replace individual files atomically and avoid rewriting unchanged sources.
    for path, data in changes.items():
        if path.exists() and path.read_bytes() == data:
            continue
        with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as file:
            temporary = Path(file.name)
            file.write(data)
        try:
            temporary.chmod(path.stat().st_mode & 0o777 if path.exists() else 0o644)
            os.replace(temporary, path)
        finally:
            temporary.unlink(missing_ok=True)
        print(f'Updated {path.relative_to(ROOT)}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('version', type=release_version, help='upstream release, e.g. 4.6.4 or v4.6.4')
    args = parser.parse_args()
    try:
        commit = resolve_commit(args.version)
        print(f'simdjson v{args.version}: {commit}', flush=True)
        apply(prepare(ROOT, args.version, commit))
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        parser.exit(1, f'update failed: {error}\n')
    print('Review error codes and DOM tape layout, then run mise run release-check and platform CI.')


if __name__ == '__main__':
    main()
