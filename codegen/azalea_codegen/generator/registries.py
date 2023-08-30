"""Generates registries."""
import re

from azalea_codegen.utils import strip_suffix, to_camel_case, get_root_path

_REGISTRIES_PATH = get_root_path('azalea-registry/src/lib.rs')
_ENUM_MATCHER = re.compile(r'registry! \{.*?^enum (.+?) \{.*?}.*?}', re.DOTALL | re.MULTILINE)


def generate_registries(registries: dict):
    # Generate the code for all registries.
    registry_code = {}

    for registry_id, registry in registries.items():
        registry_name = registry_id.split(':', 1)[1]

        # Change _type to _kind because that's Rustier (and because _type is a reserved keyword).
        if registry_name.endswith('_type') or registry_name == 'menu':
            registry_name = strip_suffix(registry_name, '_type') + '_kind'

        # Generate registry code.
        struct_name = to_camel_case(registry_name)
        this_registry_code = [
            f'// registry id: {registry_id}',
            f'enum {struct_name} {{'
        ]

        for variant_id, _ in sorted(registry['entries'].items(), key=lambda x: x[1]['protocol_id']):
            variant_name = variant_id.split(':', 1)[1]
            variant_struct_name = to_camel_case(variant_name)

            this_registry_code.append(f'    {variant_struct_name} => "{variant_id}",')

        this_registry_code.append('}')
        registry_code[struct_name] = '\n'.join(this_registry_code)

    # Search through the original file and replace the registries.
    with open(_REGISTRIES_PATH, 'r') as f:
        registry_content = f.read()

    search_pos = 0
    while True:
        match = _ENUM_MATCHER.match(registry_content, search_pos)

        if match is None:
            break

        registry_struct_name = match.group(1)
        replacement = registry_code[registry_struct_name]

        registry_content = registry_content[:match.start()] + replacement + registry_content[match.end():]
        search_pos = match.start() + len(replacement)

    with open(_REGISTRIES_PATH, 'w') as f:
        f.write(registry_content)
