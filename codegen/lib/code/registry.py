from lib.utils import to_snake_case, upper_first_letter, get_dir_location, to_camel_case
from ..mappings import Mappings
from typing import Optional
import re

REGISTRIES_DIR = get_dir_location('../azalea-registry/src/lib.rs')


def generate_registries(registries: dict):
    code = []

    code.append('use azalea_registry_macros::registry;')
    code.append('')

    for registry_name, registry in registries.items():
        # registry!(Block, {
        #     Air => "minecraft:air",
        #     Stone => "minecraft:stone"
        # });
        registry_struct_name = to_camel_case(registry_name.split(':')[1])
        code.append(f'registry!({registry_struct_name}, {{')
        registry_entries = sorted(
            registry['entries'].items(), key=lambda x: x[1]['protocol_id'])
        for variant_name, _variant in registry_entries:
            variant_struct_name = to_camel_case(
                variant_name.split(':')[1])
            code.append(f'\t{variant_struct_name} => "{variant_name}",')
        code.append('});')
        code.append('')

    with open(REGISTRIES_DIR, 'w') as f:
        f.write('\n'.join(code))
