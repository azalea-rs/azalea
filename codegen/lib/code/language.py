from lib.utils import get_dir_location
import json

LANGUAGE_DIR = get_dir_location("../azalea-language/src/en_us.json")


def write_language(contents: dict):
    with open(LANGUAGE_DIR, "w") as f:
        f.write(json.dumps(contents, indent="  "))
