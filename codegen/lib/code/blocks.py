from typing import Optional
from lib.utils import to_snake_case, upper_first_letter, get_dir_location, to_camel_case
from ..mappings import Mappings
import re

BLOCKS_RS_DIR = get_dir_location('../azalea-block/src/blocks.rs')

# Terminology:
# - Property: A property of a block, like "direction"
# - Variant: A potential state of a property, like "up"
# - State: A possible state of a block, a combination of variants
# - Block: Has properties and states.


def generate_blocks(blocks_burger: dict, blocks_report: dict, ordered_blocks: list[str], mappings: Mappings):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    def get_property_struct_name(property: Optional[dict], block_data_burger: dict, property_variants: list[str]) -> str:
        if property is None:
            return '_'.join(map(to_camel_case, property_variants))
            
        property_name = None
        for class_name in [block_data_burger['class']] + block_data_burger['super']:
            property_name = mappings.get_field(
                class_name, property['field_name'])
            if property_name:
                break
        assert property_name
        property_name = to_camel_case(property_name.lower())
        if property['type'] == 'int':
            property_name = to_camel_case(
                block_data_burger['text_id']) + property_name
        return property_name

    # Find properties
    properties = {}

    # This dict looks like { 'FloweringAzaleaLeavesDistance': 'distance' }
    property_struct_names_to_names = {}
    for block_id in ordered_blocks:
        block_data_burger = blocks_burger[block_id]
        block_data_report = blocks_report[f'minecraft:{block_id}']

        block_properties = {}
        for property_name in list(block_data_report.get('properties', {}).keys()):
            property_burger = None
            for property in block_data_burger.get('states', []):
                if property['name'] == property_name:
                    property_burger = property
                    break

            property_variants = block_data_report['properties'][property_name]

            if property_burger is None:
                print(
                    'Warning: The reports have states for a block, but Burger doesn\'t!', block_data_burger)

            property_struct_name = get_property_struct_name(
                property_burger, block_data_burger, property_variants)
            # assert property_name == property_burger['name']

            block_properties[property_struct_name] = property_variants

            # if the name ends with _<number>, remove that part
            ending = property_name.split('_')[-1]
            if ending.isdigit():
                property_name = property_name[:-(len(ending) + 1)]
            property_struct_names_to_names[property_struct_name] = property_name

        properties.update(block_properties)

    # Property codegen
    new_make_block_states_macro_code.append('    Properties => {')
    for property_struct_name, property_variants in properties.items():
        # "face" => Face {
        #     Floor,
        #     Wall,
        #     Ceiling,
        # },
        property_name = property_struct_names_to_names[property_struct_name]
        new_make_block_states_macro_code.append(
            f'        "{property_name}" => {property_struct_name} {{')

        for variant in property_variants:
            new_make_block_states_macro_code.append(
                f'            {to_camel_case(variant)},')

        new_make_block_states_macro_code.append(
            f'        }},')
    new_make_block_states_macro_code.append('    },')

    # Block codegen
    new_make_block_states_macro_code.append('    Blocks => {')
    for block_id in ordered_blocks:
        block_data_burger = blocks_burger[block_id]
        block_data_report = blocks_report['minecraft:' + block_id]

        block_properties = block_data_burger.get('states', [])
        block_properties_burger = block_data_burger.get('states', [])

        default_property_variants: dict[str, str] = {}
        for state in block_data_report['states']:
            if state.get('default'):
                default_property_variants = state.get('properties', {})

        # TODO: use burger to generate the blockbehavior
        new_make_block_states_macro_code.append(
            f'        {block_id} => BlockBehavior::default(), {{')
        for property_name in list(block_data_report.get('properties', {}).keys()):
            property_burger = None
            for property in block_data_burger.get('states', []):
                if property['name'] == property_name:
                    property_burger = property
                    break

            property_default = default_property_variants.get(property_name)
            property_variants = block_data_report['properties'][property_name]

            property_struct_name = get_property_struct_name(
                property_burger, block_data_burger, property_variants)
            assert property_default is not None
            new_make_block_states_macro_code.append(
                f'            {property_struct_name}={to_camel_case(property_default)},')
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
