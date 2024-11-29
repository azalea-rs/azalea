from lib.utils import get_dir_location
import re
import os

README_DIR = get_dir_location('../README.md')
VERSION_REGEX = r'\_Currently supported Minecraft version: `(.*)`.\_'


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
    
    # update the version in all Cargo.toml files
    # version = "0.10.3+mc1.21.1"
    for root, _, files in os.walk(get_dir_location('..')):
        for file in files:
            if file == 'Cargo.toml':
                with open(os.path.join(root, file), 'r') as f:
                    cargo_toml = f.read().splitlines()
                for i, line in enumerate(cargo_toml):
                    if line.strip().startswith('version = '):
                        replaced = re.sub(r'\+mc[^"]+?"', f'+mc{version_id}"', line)
                        cargo_toml[i] = replaced
                        break
                else:
                    # didn't have a version line
                    continue
                if cargo_toml[-1] != '':
                    # make sure there's always a trailing newline
                    cargo_toml.append('')
                with open(os.path.join(root, file), 'w') as f:
                    f.write('\n'.join(cargo_toml))
    print('Updated version in README.md and Cargo.toml files')

def get_protocol_version() -> str:
    # azalea-protocol/src/packets/mod.rs
    # pub const PROTOCOL_VERSION: i32 = 758;
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
        if line.strip().startswith('pub const PROTOCOL_VERSION:'):
            mod_rs[i] = f'pub const PROTOCOL_VERSION: i32 = {protocol_version};'
            break
    else:
        raise Exception(
            'Could not find protocol version in azalea-protocol/src/packets/mod.rs')

    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'w') as f:
        f.write('\n'.join(mod_rs))
def set_version_name(version_name: str) -> None:
    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'r') as f:
        mod_rs = f.read().splitlines()
    for i, line in enumerate(mod_rs):
        if line.strip().startswith('pub const VERSION_NAME:'):
            mod_rs[i] = f'pub const VERSION_NAME: &str = "{version_name}";'
            break
    else:
        raise Exception(
            'Could not find version name in azalea-protocol/src/packets/mod.rs')

    with open(get_dir_location('../azalea-protocol/src/packets/mod.rs'), 'w') as f:
        f.write('\n'.join(mod_rs))
