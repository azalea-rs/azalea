import logging

from azalea_codegen import utils
from azalea_codegen.utils import Mappings, get_root_path

_OUTPUT_PATH = get_root_path('azalea-block', 'src', 'generated.rs')
_LOGGER = logging.getLogger(__name__)


# Terminology:
# - Property: A property of a block, like "direction"
# - Variant: A potential state of a property, like "up"
# - State: A possible state of a block, a combination of variants
# - Block: Has properties and states.


def generate_blocks(mappings: Mappings, blocks_burger: dict, ordered_blocks: list[str], blocks_report: dict):
    generated_code = ['make_block_states! {']

    # Aggregate all block state properties.
    all_properties = {}
    property_struct_names = {}

    for block_id in ordered_blocks:
        block_burger = blocks_burger[block_id]
        block_report = blocks_report[f'minecraft:{block_id}']

        for property_name in block_report.get('properties', {}).keys():
            property_burger = utils.find(block_burger.get('states', []), lambda p: p['name'] == property_name)

            if property_burger is None:
                _LOGGER.warning(f'Data generator reports contain a property `{property_name}` for block `{block_id}`,'
                                f' but burger doesn\'t')

            property_variants = block_report['properties'][property_name]
            property_struct_name = _get_property_struct_name(mappings, property_burger, block_burger, property_variants)

            if property_struct_name in all_properties and \
                    all_properties[property_struct_name] != property_variants:
                raise Exception(f'Generated multiple property types with the same name ({property_struct_name}).'
                                f' Conflicting variants are {property_variants} vs '
                                f'{all_properties[property_struct_name]}. This issue can be solved by giving one'
                                f' property a hardcoded name.')

            all_properties[property_struct_name] = property_variants
            property_struct_names[property_struct_name] = _clean_property_name(property_name)

    # Generate code for properties.
    generated_code.append('    Properties => {')

    for property_struct_name, property_variants in all_properties.items():
        property_name = property_struct_names[property_struct_name]

        if property_variants == ['true', 'false']:
            property_type = ['bool']

        else:
            property_type = [f'{property_struct_name} {{']

            for variant in property_variants:
                property_type.append(f'            {utils.to_camel_case(variant)},')

            property_type.append('        }')

        property_type = '\n'.join(property_type)
        generated_code.append(f'        "{property_name}" => {property_type},')

    generated_code.append('    },')

    # Generate code for all block states.
    generated_code.append('    Blocks => {')

    for block_id in ordered_blocks:
        block_burger = blocks_burger[block_id]
        block_report = blocks_report[f'minecraft:{block_id}']

        # Get default property values from the block report.
        default_property_variants = {}

        for state in block_report['states']:
            if state.get('default'):
                default_property_variants = state.get('properties', {})

        # Generate code for all properties.
        properties_code = []

        for property_name in block_report.get('properties', {}).keys():
            property_burger = utils.find(block_burger.get('states', []), lambda p: p['name'] == property_name)
            property_default = default_property_variants[property_name]
            property_variants = block_report['properties'][property_name]

            property_struct_name = _get_property_struct_name(mappings, property_burger, block_burger, property_variants)
            property_default_type = property_default if property_variants == ['true', 'false'] else \
                f'{property_struct_name}::{utils.to_camel_case(property_default)}'

            properties_code.append(f'            {_clean_property_name(property_name)}: {property_default_type},')

        # If there's no properties put both braces on the same line.
        properties_code = '\n'.join(['{'] + properties_code + ['        }']) if properties_code else '{}'

        # Generate block behaviour.
        behaviour_ctor = ['BlockBehaviour::new()']

        # Strength (explosion resistance and hardness)
        hardness = block_burger.get('hardness')
        resistance = block_burger.get('resistance')

        if hardness is not None and resistance is not None:
            behaviour_ctor.append(f'strength({hardness}, {resistance})')

        elif hardness is not None:
            behaviour_ctor.append(f'destroy_time({hardness})')

        elif resistance is not None:
            behaviour_ctor.append(f'explosion_resistance({resistance})')

        # Friction.
        friction = block_burger.get('friction')

        if friction is not None:
            behaviour_ctor.append(f'friction({friction})')

        # Requires correct tool for drops.
        if block_burger.get('requires_correct_tool_for_drops'):
            behaviour_ctor.append('requires_correct_tool_for_drops()')

        behaviour_ctor = '.'.join(behaviour_ctor)
        generated_code.append(f'        {block_id} => {behaviour_ctor}, {properties_code},')

    generated_code.extend(['    }', '}'])

    # Write out to file.
    with open(_OUTPUT_PATH, 'r') as f:
        existing_code = f.read().splitlines()

    new_code = []
    in_macro = False

    for line in existing_code:
        if line == 'make_block_states! {':
            in_macro = True

        elif line == '}':
            if in_macro:
                in_macro = False
                new_code.extend(generated_code)
                continue

        if in_macro:
            continue

        new_code.append(line)

    new_code.append('')

    with open(_OUTPUT_PATH, 'w') as f:
        f.write('\n'.join(new_code))


def _clean_property_name(property_name: str) -> str:
    # if the name ends with _<number>, remove that part
    ending = property_name.split('_')[-1]
    if ending.isdigit():
        property_name = property_name[:-(len(ending) + 1)]

    # `type` is a reserved keyword, so we use kind instead ¯\_(ツ)_/¯
    if property_name == 'type':
        property_name = 'kind'

    return property_name


def _get_property_struct_name(mappings: Mappings, property_burger: dict | None, block_burger: dict,
                              property_variants: list[str]) -> str:
    # these are hardcoded because otherwise they cause conflicts
    # some names inspired by https://github.com/feather-rs/feather/blob/main/feather/blocks/src/generated/table.rs
    if property_variants == ['north', 'east', 'south', 'west', 'up', 'down']:
        return 'FacingCubic'

    if property_variants == ['north', 'south', 'west', 'east']:
        return 'FacingCardinal'

    if property_variants == ['top', 'bottom']:
        return 'TopBottom'

    if property_variants == ['north_south', 'east_west', 'ascending_east', 'ascending_west', 'ascending_north',
                             'ascending_south']:
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

    if 'harp' in property_variants and 'didgeridoo' in property_variants:
        return 'Sound'

    if property_burger is None:
        return ''.join(map(utils.to_camel_case, property_variants))

    property_name = None

    for class_name in [block_burger['class']] + block_burger['super']:
        property_name = mappings.get_field_name(class_name, property_burger['field_name'])

        if property_name:
            break

    if property_name is None:
        if 'declared_in' in property_burger:
            property_name = mappings.get_field_name(property_burger['declared_in'], property_burger['field_name'])

    if property_name is None:
        property_name = property_burger['name']

    assert property_name

    property_name = utils.to_camel_case(property_name.lower())
    if property_burger['type'] == 'int':
        property_name = utils.to_camel_case(block_burger['text_id']) + property_name

    if property_variants == ['up', 'side', 'none']:
        property_name = 'Wire' + utils.to_camel_case(property_name)

    return property_name
