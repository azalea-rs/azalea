from lib.utils import to_camel_case
from lib.utils import get_dir_location
import json

BLOCKS_RS_DIR = get_dir_location('../azalea-block/src/blocks.rs')


def generate_blocks(blocks: dict):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    # Find properties
    properties = {}
    for block_data in blocks.values():
        block_properties = block_data.get('properties', {})
        properties.update(block_properties)

    # Property codegen
    new_make_block_states_macro_code.append('    Properties => {')
    for property_name, property_variants in properties.items():
        new_make_block_states_macro_code.append(
            f'        {to_camel_case(property_name)} => {{')

        for variant in property_variants:
            new_make_block_states_macro_code.append(
                f'            {to_camel_case(variant)},')

        new_make_block_states_macro_code.append(
            f'        }},')
    new_make_block_states_macro_code.append('    },')

    # Block codegen
    new_make_block_states_macro_code.append('    Blocks => {')
    for block_id, block_data in blocks.items():
        block_id = block_id.split(':')[1]
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
    new_make_block_states_macro_code.append('    },')

    print('\n'.join(new_make_block_states_macro_code))
