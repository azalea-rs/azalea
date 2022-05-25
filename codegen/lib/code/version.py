import re

README_DIR = '../README.md'
VERSION_REGEX = r'\*Currently supported Minecraft version: `(.*)`.\*'


def get_version_id() -> str:
    with open(README_DIR, 'r') as f:
        readme_text = f.read()

    version_line_match = re.search(VERSION_REGEX, readme_text)
    if version_line_match:
        version_id = version_line_match.group(1)
        return version_id
    else:
        raise Exception('Could not find version id in README.md')


def set_version_id(version_id: str) -> None:
    with open(README_DIR, 'r') as f:
        readme_text = f.read()

    version_line_match = re.search(VERSION_REGEX, readme_text)
    if version_line_match:
        readme_text = readme_text.replace(
            version_line_match.group(1), version_id)
    else:
        raise Exception('Could not find version id in README.md')

    with open(README_DIR, 'w') as f:
        f.write(readme_text)
