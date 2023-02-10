from lib.utils import to_snake_case, upper_first_letter, get_dir_location, to_camel_case
from ..mappings import Mappings
from typing import Optional
import re

REGISTRIES_DIR = get_dir_location('../azalea-registry/src/lib.rs')


def generate_registries(registries: dict):
    with open(REGISTRIES_DIR, 'r') as f:
        code = f.read().split('\n')

    for registry_name, registry in registries.items():
        # registry!(Block, {
        #     Air => "minecraft:air",
        #     Stone => "minecraft:stone"
        # });

        if registry_name.endswith('_type'):
            # change _type to _kind because that's Rustier (and because _type
            # is a reserved keyword)
            registry_name = registry_name[:-5] + '_kind'

        registry_struct_name = to_camel_case(registry_name.split(':')[1])

        registry_code = []
        registry_code.append(f'enum {registry_struct_name} {{')
        registry_entries = sorted(
            registry['entries'].items(), key=lambda x: x[1]['protocol_id'])
        for variant_name, _variant in registry_entries:
            variant_struct_name = to_camel_case(
                variant_name.split(':')[1])
            registry_code.append(f'\t{variant_struct_name} => "{variant_name}",')
        registry_code.append('}')

        # when we find a "registry! {" line, find the next line that starts
        # with "enum <name>" and replace that until we find a line that's "}"
        found = False
        in_registry_macro = False
        for i, line in enumerate(list(code)):
            if not in_registry_macro and line == "registry! {":
                in_registry_macro = True
            elif in_registry_macro and line == registry_code[0]:
                # found it, now delete until we get to "}"
                while code[i] != '}':
                    code.pop(i)
                code[i] = '\n'.join(registry_code)
                found = True
                break
        if not found:
            code.append('registry! {')
            code.append('\n'.join(registry_code))
            code.append('}')
            code.append('')

    with open(REGISTRIES_DIR, 'w') as f:
        f.write('\n'.join(code))