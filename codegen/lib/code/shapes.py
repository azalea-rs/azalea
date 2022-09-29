from lib.utils import get_dir_location, to_camel_case
from lib.code.utils import clean_property_name
from .blocks import get_property_struct_name
from ..mappings import Mappings

COLLISION_BLOCKS_RS_DIR = get_dir_location(
    '../azalea-physics/src/collision/blocks.rs')


def generate_block_shapes(blocks: dict, shapes: dict, block_states_report, block_datas_burger, mappings: Mappings):
    code = generate_block_shapes_code(blocks, shapes, block_states_report, block_datas_burger, mappings)
    with open(COLLISION_BLOCKS_RS_DIR, 'w') as f:
        f.write(code)


def generate_block_shapes_code(blocks: dict, shapes: dict, block_states_report, block_datas_burger, mappings: Mappings):
    # look at downloads/generator-mod-*/blockCollisionShapes.json for format of blocks and shapes

    generated_shape_code = ''
    # we make several lazy_static! blocks so it doesn't complain about
    # recursion and hopefully the compiler can paralleize it?
    generated_shape_code += 'lazy_static! {'
    for i, (shape_id, shape) in enumerate(sorted(shapes.items(), key=lambda shape: int(shape[0]))):
        if i > 0 and i % 10 == 0:
            generated_shape_code += '}\nlazy_static! {'
        generated_shape_code += generate_code_for_shape(shape_id, shape)
    generated_shape_code += '}'

    generated_impl_code = ''
    for block_id, shape_ids in blocks.items():
        if isinstance(shape_ids, int):
            shape_ids = [shape_ids]
        block_report_data = block_states_report['minecraft:' + block_id]
        block_data_burger = block_datas_burger[block_id]
        generated_impl_code += generate_code_for_impl(
            block_id,
            shape_ids,
            block_report_data,
            block_data_burger,
            mappings
        )

    return f'''
//! Autogenerated block collisions for every block

// This file is generated from codegen/lib/code/block_shapes.py. If you want to
// modify it, change that file.

#![allow(clippy::explicit_auto_deref)]

use super::VoxelShape;
use crate::collision::{{self, Shapes}};
use azalea_block::*;
use lazy_static::lazy_static;

trait BlockWithShape {{
    fn shape(&self) -> &'static VoxelShape;
}}

{generated_shape_code}

{generated_impl_code}
'''


def generate_code_for_shape(shape_id: str, parts: list[list[float]]):
    def make_arguments(part: list[float]):
        return ', '.join(map(lambda n: str(n).rstrip('0'), part))
    code = ''
    code += f'static ref SHAPE{shape_id}: VoxelShape = {{\n'
    steps = []
    if parts == []:
        steps.append('collision::empty_shape()')
    else:
        steps.append(f'collision::box_shape({make_arguments(parts[0])})')
        for part in parts[1:]:
            steps.append(
                f'Shapes::or(s, collision::box_shape({make_arguments(part)}))')

    for step in steps[:-1]:
        code += f'    let s = {step};\n'
    code += f'    {steps[-1]}\n'
    code += f'}};\n'
    return code


def generate_code_for_impl(block_id: str, shape_ids: list[int], block_report_data, block_data_burger, mappings: Mappings):
    # if block_id != 'spruce_fence':
    #     return ''

    # match self {
    #     StoneSlabBlock {
    #         kind: Type::Top,
    #         waterlogged: Waterlogged::True,
    #     } => &SHAPE0,
    #     _ => &SHAPE1,
    # }

    block_struct_name = to_camel_case(block_id) + 'Block'

    property_names = tuple(block_report_data.get('properties', {}).keys())

    # { (tuple of property values): shape_id }
    possible_states = {}
    for possible_state, shape_id in zip(block_report_data['states'], shape_ids):
        possible_states[tuple(
            possible_state.get('properties', {}).values())] = shape_id

    should_also_ignore_other_fields = False

    # detect if a property actually makes a difference, and if it doesn't then
    # replace the name with None
    for property_index, property_name in enumerate(property_names):
        changes_shape = False
        # { waterlogged: false, thing: false }: 0
        # { waterlogged: false, thing: true }: 1
        # { waterlogged: true, thing: false }: 0
        # { waterlogged: true, thing: true }: 1
        similar_properties_shape = {}
        for property_values, shape_id in possible_states.items():
            # switch out our property for None
            property_values_modified = list(property_values)
            property_values_modified[property_index] = None
            property_values_modified = tuple(property_values_modified)
            # we haven't seen this combination before, add it to our dict
            if property_values_modified not in similar_properties_shape:
                similar_properties_shape[property_values_modified] = shape_id
            # if we've seen this combination before and it's a different shape
            # id, that means we can't ignore this property
            elif similar_properties_shape[property_values_modified] != shape_id:
                # we know it changes the shape now, so there's nothing else to do
                changes_shape = True
                should_also_ignore_other_fields = True
                break
            property_value = property_values[property_index]
        # if this property doesn't change the shape, we can update
        # possible_states
        if not changes_shape:
            for property_values, shape_id in list(possible_states.items()):
                property_values_copy = list(property_values)
                property_values_copy[property_index] = None
                del possible_states[property_values]
                possible_states[tuple(property_values_copy)] = shape_id

    if len(possible_states) == 1:
        function_inner = f'&SHAPE{list(possible_states.values())[0]}'
    else:
        match_inner = ''
        for property_values, shape_id in possible_states.items():
            match_inner += f'    {block_struct_name} {{\n'
            for property_name, property_value in zip(property_names, property_values):
                # get the burger data for this property
                property_burger = None
                for property_burger_iter in block_data_burger['states']:
                    if property_burger_iter['name'] == property_name:
                        property_burger = property_burger_iter
                assert property_burger
                actual_property_variants = block_report_data['properties'][property_name]
                property_type_name = get_property_struct_name(
                    property_burger,
                    block_data_burger,
                    actual_property_variants,
                    mappings
                )

                if property_value is not None:
                    if actual_property_variants == ['true', 'false']:
                        default_code = property_value # it'll be either true or false
                    else:
                        default_code = f'{property_type_name}::{to_camel_case(property_value)}'
                    match_inner += f'        {clean_property_name(property_name)}: {default_code},\n'
            if should_also_ignore_other_fields:
                match_inner += '        ..\n'
            match_inner += f'    }} => &SHAPE{shape_id},\n'

        function_inner = 'match self {\n' + match_inner + '\n}'

    code = ''
    code += f'impl BlockWithShape for {block_struct_name} {{\n'
    code += f'    fn shape(&self) -> &\'static VoxelShape {{\n'
    code += f'        {function_inner}\n'
    code += f'    }}\n'
    code += '}'

    return code
