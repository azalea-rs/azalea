from lib.utils import get_dir_location
import re
import os

README_DIR = get_dir_location('../README.md')
VERSION_REGEX = r'\*Currently supported Minecraft version: `(.*)`.\*'


def get_version_id() -> str:
    with open(README_DIR, 'rb') as f:
        readme_text = f.read().decode()

    version_line_match = re.search(VERSION_REGEX, readme_text)
    if version_line_match:
        version_id = version_line_match.group(1)
        return version_id
    else:
        raise Exception('Could not find version id in README.md')


def set_version_id(version_id: str) -> None:
    with open(README_DIR, 'rb') as f:
        readme_text = f.read().decode()

    version_line_match = re.search(VERSION_REGEX, readme_text)
    if version_line_match:
        readme_text = readme_text.replace(
            version_line_match.group(1), version_id)
    else:
        raise Exception('Could not find version id in README.md')

    with open(README_DIR, 'wb') as f:
        f.write(readme_text.encode())


def get_protocol_version() -> str:
    # azalea-protocol/src/packets/mod.rs
    # pub const PROTOCOL_VERSION: u32 = 758;
    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'r') as f:
        mod_rs = f.read().splitlines()
    for line in mod_rs:
        if line.strip().startswith('pub const PROTOCOL_VERSION'):
            return line.strip().split(' ')[-1].strip(';')
    raise Exception(
        'Could not find protocol version in azalea-protocol/src/packets/mod.rs')


def set_protocol_version(protocol_version: str) -> None:
    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'r') as f:
        mod_rs = f.read().splitlines()
    for i, line in enumerate(mod_rs):
        if line.strip().startswith('pub const PROTOCOL_VERSION'):
            mod_rs[i] = f'pub const PROTOCOL_VERSION: u32 = {protocol_version};'
            break
    else:
        raise Exception(
            'Could not find protocol version in azalea-protocol/src/packets/mod.rs')

    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'w') as f:
        f.write('\n'.join(mod_rs))
