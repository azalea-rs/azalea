from lib.utils import get_dir_location, to_camel_case
from ..mappings import Mappings
from typing import Optional
import re

BLOCKS_RS_DIR = get_dir_location('../azalea-block/src/generated.rs')

# Terminology:
# - Property: A property of a block, like "direction"
# - Variant: A potential state of a property, like "up"
# - State: A possible state of a block, a combination of variants
# - Block: Has properties and states.


def generate_blocks(blocks_report: dict, pixlyzer_block_datas: dict, ordered_blocks: list[str]):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    # Find properties
    properties = {}

    # This dict looks like { 'FloweringAzaleaLeavesDistance': 'distance' }
    property_struct_names_to_names = {}
    for block_id in ordered_blocks:
        block_data_report = blocks_report[f'minecraft:{block_id}']

        block_properties = {}
        for property_id in list(block_data_report.get('properties', {}).keys()):
            property_variants = block_data_report['properties'][property_id]

            property_struct_name = get_property_struct_name(
                block_id, property_id, property_variants)

            if property_struct_name in properties:
                if not properties[property_struct_name] == property_variants:
                    raise Exception(
                        'There are multiple enums with the same name! '
                        f'Name: {property_struct_name}, variants: {property_variants}/{properties[property_struct_name]}. '
                        'This can be fixed by hardcoding a name in the get_property_struct_name function.'
                    )

            block_properties[property_struct_name] = property_variants

            property_struct_names_to_names[property_struct_name] = property_id

        properties.update(block_properties)

    # Property codegen
    new_make_block_states_macro_code.append('    Properties => {')
    for property_struct_name, property_variants in properties.items():
        # "face" => Face {
        #     Floor,
        #     Wall,
        #     Ceiling,
        # },
        property_id = property_struct_names_to_names[property_struct_name]

        # if the only variants are true and false, we make it unit struct with a boolean instead of an enum
        if property_variants == ['true', 'false']:
            property_shape_code = f'{property_struct_name}(bool)'
        else:
            property_shape_code = f'{property_struct_name} {{\n'
            for variant in property_variants:
                property_shape_code += f'            {to_camel_case(variant)},\n'
            property_shape_code += '        }'

        new_make_block_states_macro_code.append(
            f'        "{property_id}" => {property_shape_code},')

    new_make_block_states_macro_code.append('    },')

    # Block codegen
    new_make_block_states_macro_code.append('    Blocks => {')
    for block_id in ordered_blocks:
        block_data_report = blocks_report['minecraft:' + block_id]
        block_data_pixlyzer = pixlyzer_block_datas.get(f'minecraft:{block_id}', {})

        default_property_variants: dict[str, str] = {}
        for state in block_data_report['states']:
            if state.get('default'):
                default_property_variants = state.get('properties', {})

        properties_code = '{'
        for property_id in list(block_data_report.get('properties', {}).keys()):
            property_default = default_property_variants.get(property_id)
            property_variants = block_data_report['properties'][property_id]

            property_struct_name = get_property_struct_name(
                block_id, property_id, property_variants)

            is_boolean_property = property_variants == ['true', 'false']

            if is_boolean_property:
                # if it's a boolean, keep the type lowercase
                # (so it's either `true` or `false`)
                property_default_type = f'{property_struct_name}({property_default})'
            else:
                property_default_type = f'{property_struct_name}::{to_camel_case(property_default)}'

            assert property_default is not None

            this_property_code = f'"{property_id}": {property_default_type}'

            properties_code += f'\n            {this_property_code},'
        # if there's nothing inside the properties, keep it in one line
        if properties_code == '{':
            properties_code += '}'
        else:
            properties_code += '\n        }'

        # make the block behavior
        behavior_constructor = 'BlockBehavior::new()'
        # requires tool
        if block_data_pixlyzer.get('requires_tool'):
            behavior_constructor += '.requires_correct_tool_for_drops()'
        # strength
        destroy_time = block_data_pixlyzer.get('hardness')
        explosion_resistance = block_data_pixlyzer.get('explosion_resistance')
        if destroy_time and explosion_resistance:
            behavior_constructor += f'.strength({destroy_time}, {explosion_resistance})'
        elif destroy_time:
            behavior_constructor += f'.destroy_time({destroy_time})'
        elif explosion_resistance:
            behavior_constructor += f'.explosion_resistance({explosion_resistance})'
        # friction
        friction = block_data_pixlyzer.get('friction')
        if friction != None:
            behavior_constructor += f'.friction({friction})'

        # TODO: use burger to generate the blockbehavior
        new_make_block_states_macro_code.append(
            f'        {block_id} => {behavior_constructor}, {properties_code},')

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
    # empty line at the end
    new_code.append('')

    with open(BLOCKS_RS_DIR, 'w') as f:
        f.write('\n'.join(new_code))

def get_property_struct_name(block_id: str, property_id: str, property_variants: list[str]) -> str:
    # these are hardcoded because otherwise they cause conflicts
    # some names inspired by https://github.com/feather-rs/feather/blob/main/feather/blocks/src/generated/table.rs
    if property_variants == ['north', 'east', 'south', 'west', 'up', 'down']:
        return 'FacingCubic'
    if property_variants == ['north', 'south', 'west', 'east']:
        return 'FacingCardinal'
    if property_variants == ['top', 'bottom']:
        return 'TopBottom'
    if property_variants == ['north_south', 'east_west', 'ascending_east', 'ascending_west', 'ascending_north', 'ascending_south']:
        return 'RailShape'
    if property_variants == ['straight', 'inner_left', 'inner_right', 'outer_left', 'outer_right']:
        return 'StairShape'
    if property_variants == ['normal', 'sticky']:
        return 'PistonType'
    if property_variants == ['x', 'z']:
        return 'AxisXZ'
    if property_variants == ['single', 'left', 'right']:
        return 'ChestType'
    if property_variants == ['compare', 'subtract']:
        return 'ComparatorType'
    if property_variants == ['inactive', 'waiting_for_players', 'active', 'waiting_for_reward_ejection', 'ejecting_reward', 'cooldown']:
        return 'TrialSpawnerState'
    if property_variants == ['inactive', 'active', 'unlocking', 'ejecting']:
        return 'VaultState'
    if 'harp' in property_variants and 'didgeridoo' in property_variants:
        return 'Sound'
    if is_list_of_string_integers(property_variants):
        # if the values are all integers, then prepend the block name
        return to_camel_case(block_id) + to_camel_case(property_id)
    if property_variants == ['up', 'side', 'none']:
        return 'Wire' + to_camel_case(property_id)
    if property_variants == ['none', 'low', 'tall']:
        return 'Wall' + to_camel_case(property_id)

    return to_camel_case(property_id)

def is_list_of_string_integers(l: list[str]) -> bool:
    return all(map(str.isdigit, l))

def get_ordered_blocks(registries_report: dict[str, dict]) -> list[str]:
    '''
    Returns a list of block ids (like ['air', 'stone', ...]) ordered by their protocol id.
    '''
    blocks_registry = registries_report['minecraft:block']

    blocks_to_ids = {} 
    for block_id, value in blocks_registry['entries'].items():
        prefix = 'minecraft:'
        assert block_id.startswith(prefix)
        block_id = block_id[len(prefix):]
        protocol_id = value['protocol_id']
        blocks_to_ids[block_id] = protocol_id
    
    ordered_blocks = []
    for block_id in sorted(blocks_to_ids, key=blocks_to_ids.get):
        ordered_blocks.append(block_id)
    return ordered_blocks
