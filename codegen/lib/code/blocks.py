from lib.utils import upper_first_letter
from lib.utils import get_dir_location
from lib.utils import to_camel_case
from ..mappings import Mappings

BLOCKS_RS_DIR = get_dir_location('../azalea-block/src/blocks.rs')

# Terminology:
# - Property: A property of a block, like "direction"
# - Variant: A potential state of a property, like "up"
# - State: A possible state of a block, a combination of variants
# - Block: Has properties and states.

def generate_blocks(blocks_burger: dict, blocks_report: dict, mappings: Mappings):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    def get_property_struct_name(property: dict, block_data_burger: dict) -> str:
        property_name = None
        for class_name in [block_data_burger['class']] + block_data_burger['super']:
            property_name = mappings.get_field(class_name, property['field_name'])
            if property_name:
                break
        assert property_name
        property_name = to_camel_case(property_name.lower())
        if property['type'] == 'int':
            property_name = to_camel_case(block_data_burger['text_id']) + property_name
        return property_name

    # Find properties
    properties = {}
    for block_id, block_data_burger in blocks_burger.items():
        block_data_report = blocks_report[f'minecraft:{block_id}']

        block_properties = {}
        for property_name in list(block_data_report.get('properties', {}).keys()):
            property_burger = None
            for property in block_data_burger['states']:
                if property['name'] == property_name:
                    property_burger = property
                    break
            if property_burger is None:
                print('Error: The reports have states for a block, but Burger doesn\'t!', block_data_burger)
                continue
            # assert property_burger is not None
            property_variants = block_data_report['properties'][property_name]
            property_struct_name = get_property_struct_name(property_burger, block_data_burger)

            block_properties[property_struct_name] = property_variants
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
    for block_id, block_data_burger in blocks_burger.items():
        block_data_report = blocks_report['minecraft:' + block_id]

        block_properties_burger = block_data_burger['states']

        default_property_variants: dict[str, str] = {}
        for property in block_data_report['states']:
            if property.get('default'):
                default_property_variants = property.get('properties', {})

        # TODO: use burger to generate the blockbehavior
        new_make_block_states_macro_code.append(
            f'        {block_id} => BlockBehavior::default(), {{')
        for property in block_properties_burger:
            property_default = default_property_variants.get(property['name'])
            property_struct_name = get_property_struct_name(property, block_data_burger)
            assert property_default is not None
            new_make_block_states_macro_code.append(
                f'            {property_struct_name}={to_camel_case(property_default)},')
            # new_make_block_states_macro_code.append(
            #     f'            {to_camel_case(state)}=TODO,')
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
