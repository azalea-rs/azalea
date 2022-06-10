from lib.utils import get_dir_location
from lib.utils import to_camel_case
from ..mappings import Mappings
import json

BLOCKS_RS_DIR = get_dir_location('../azalea-block/src/blocks.rs')

# Terminology:
# - Property: A property of a block, like "direction"
# - Variant: A potential state of a property, like "up"
# - State: A possible state of a block, a combination of variants
# - Block: Has properties and states.


def get_property_variants(data) -> list[str]:
    if 'values' in data:
        return list(map(str.lower, data['values']))
    if data['type'] == 'bool':
        return ['true', 'false']
    if data['type'] == 'int':
        # range between data['min'] and data['max']
        return [str(i) for i in range(data['min'], data['max'] + 1)]
    raise Exception('Unknown property type: ' + data['type'])


def generate_blocks(blocks: dict, mappings: Mappings):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    # Find properties
    properties = {}
    for block_data in blocks.values():
        block_properties = {}
        for property in block_data.get('states', []):
            property_name = mappings.get_field(
                property.get('declared_in', block_data['class']), property['field_name']).lower()
            property_variants = get_property_variants(property)
            block_properties[property_name] = property_variants
            # if property_name == 'eggs':
            #     print(property, property_name, property_variants)
        properties.update(block_properties)

    # Property codegen
    new_make_block_states_macro_code.append('    Properties => {')
    for property_name, property_variants in properties.items():
        new_make_block_states_macro_code.append(
            f'        {to_camel_case(property_name)} {{')

        for variant in property_variants:
            new_make_block_states_macro_code.append(
                f'            {to_camel_case(variant)},')

        new_make_block_states_macro_code.append(
            f'        }},')
    new_make_block_states_macro_code.append('    },')

    # Block codegen
    new_make_block_states_macro_code.append('    Blocks => {')
    for block_id, block_data in blocks.items():
        block_states = block_data['states']

        default_property_variants = {}
        for state in block_states:
            if state.get('default'):
                default_property_variants = state.get('properties', {})

        # TODO: use burger to generate the blockbehavior
        new_make_block_states_macro_code.append(
            f'        {block_id} => BlockBehavior::default(), {{')
        for property in block_data.get('properties', {}):
            property_default = default_property_variants.get(property)
            new_make_block_states_macro_code.append(
                f'            {to_camel_case(property)}={to_camel_case(property_default)},')
        new_make_block_states_macro_code.append('        },')
    new_make_block_states_macro_code.append('    }')
    new_make_block_states_macro_code.append('}')

    new_code = []
    in_macro = False
    for line in existing_code:
        if line == 'make_block_states! {':
            in_macro = True
        elif line == '}':
            if in_macro:
                in_macro = False
                new_code.extend(new_make_block_states_macro_code)
                continue
        if in_macro:
            continue
        new_code.append(line)

    with open(BLOCKS_RS_DIR, 'w') as f:
        f.write('\n'.join(new_code))
