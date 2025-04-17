from lib.utils import get_dir_location, to_camel_case
from ..mappings import Mappings

COLLISION_BLOCKS_RS_DIR = get_dir_location(
    '../azalea-physics/src/collision/blocks.rs')


def generate_block_shapes(blocks_pixlyzer: dict, shapes: dict, aabbs: dict, block_states_report):
    blocks, shapes = simplify_shapes(blocks_pixlyzer, shapes, aabbs)

    code = generate_block_shapes_code(blocks, shapes, block_states_report)
    with open(COLLISION_BLOCKS_RS_DIR, 'w') as f:
        f.write(code)


def simplify_shapes(blocks: dict, shapes: dict, aabbs: dict):
    new_id_increment = 0

    new_shapes = {}
    old_id_to_new_id = {}

    old_id_to_new_id[None] = 0
    new_shapes[0] = ()
    new_id_increment += 1

    used_shape_ids = set()
    # determine the used shape ids
    for _block_id, block_data in blocks.items():
        block_shapes = {state.get('collision_shape') for state in block_data['states'].values()}
        block_shapes.update({state.get('outline_shape') for state in block_data['states'].values()})
        for s in block_shapes:
            used_shape_ids.add(s)

    for shape_id, shape in enumerate(shapes):
        if shape_id not in used_shape_ids: continue
        # pixlyzer gives us shapes as an index or list of indexes into the
        # aabbs list
        # and aabbs look like { "from": number or [x, y, z], "to": (number or vec3) }
        # convert them to [x1, y1, z1, x2, y2, z2]
        shape = [shape] if isinstance(shape, int) else shape
        shape = [aabbs[shape_aabb] for shape_aabb in shape]
        shape = tuple([(
            (tuple(part['from']) if isinstance(
                part['from'], list) else ((part['from'],)*3))
            + (tuple(part['to']) if isinstance(part['to'], list)
               else ((part['to'],)*3))
        ) for part in shape])

        old_id_to_new_id[shape_id] = new_id_increment
        new_shapes[new_id_increment] = shape
        new_id_increment += 1

    # now map the blocks to the new shape ids
    new_blocks = {}
    for block_id, block_data in blocks.items():
        block_id = block_id.split(':')[-1]

        block_collision_shapes = [state.get('collision_shape') for state in block_data['states'].values()]
        block_outline_shapes = [state.get('outline_shape') for state in block_data['states'].values()]

        new_blocks[block_id] = {
            'collision': [old_id_to_new_id[shape_id] for shape_id in block_collision_shapes],
            'outline': [old_id_to_new_id[shape_id] for shape_id in block_outline_shapes]
        }

    return new_blocks, new_shapes


def generate_block_shapes_code(blocks: dict, shapes: dict, block_states_report):
    # look at __cache__/generator-mod-*/blockCollisionShapes.json for format of blocks and shapes

    generated_shape_code = ''
    for (shape_id, shape) in sorted(shapes.items(), key=lambda shape: int(shape[0])):
        generated_shape_code += generate_code_for_shape(shape_id, shape)


    # static COLLISION_SHAPES_MAP: [&LazyLock<VoxelShape>; 26644] = [&SHAPE0, &SHAPE1, &SHAPE1, ...]
    empty_shapes = []
    full_shapes = []

    # the index into this list is the block state id
    collision_shapes_map = []
    outline_shapes_map = []

    for block_id, shape_datas in blocks.items():
        collision_shapes = shape_datas['collision']
        outline_shapes = shape_datas['outline']

        if isinstance(collision_shapes, int): collision_shapes = [collision_shapes]
        if isinstance(outline_shapes, int): outline_shapes = [outline_shapes]

        block_report_data = block_states_report['minecraft:' + block_id]

        for possible_state, shape_id in zip(block_report_data['states'], collision_shapes):
            block_state_id = possible_state['id']
            if shape_id == 0: empty_shapes.append(block_state_id)
            elif shape_id == 1: full_shapes.append(block_state_id)
            while len(collision_shapes_map) <= block_state_id:
                # default to shape 1 for missing shapes (full block)
                collision_shapes_map.append(1)
            collision_shapes_map[block_state_id] = shape_id
        for possible_state, shape_id in zip(block_report_data['states'], outline_shapes):
            block_state_id = possible_state['id']
            while len(outline_shapes_map) <= block_state_id:
                # default to shape 1 for missing shapes (full block)
                outline_shapes_map.append(1)
            outline_shapes_map[block_state_id] = shape_id

    generated_map_code = f'static COLLISION_SHAPES_MAP: [&LazyLock<VoxelShape>; {len(collision_shapes_map)}] = ['
    empty_shape_match_code = convert_ints_to_rust_ranges(empty_shapes)
    block_shape_match_code = convert_ints_to_rust_ranges(full_shapes)
    for block_state_id, shape_id in enumerate(collision_shapes_map):
        generated_map_code += f'&SHAPE{shape_id},\n'
    generated_map_code += '];\n'

    generated_map_code += f'static OUTLINE_SHAPES_MAP: [&LazyLock<VoxelShape>; {len(outline_shapes_map)}] = ['
    for block_state_id, shape_id in enumerate(outline_shapes_map):
        generated_map_code += f'&SHAPE{shape_id},\n'
    generated_map_code += '];\n'

    if empty_shape_match_code == '':
        print('Error: shape 0 was not found')

    return f'''
//! Autogenerated block collisions for every block

// This file is generated from codegen/lib/code/shapes.py. If you want to
// modify it, change that file.

#![allow(clippy::explicit_auto_deref)]
#![allow(clippy::redundant_closure)]

use std::sync::LazyLock;

use super::VoxelShape;
use crate::collision::{{self, Shapes}};
use azalea_block::*;

pub trait BlockWithShape {{
    fn collision_shape(&self) -> &'static VoxelShape;
    fn outline_shape(&self) -> &'static VoxelShape;
    /// Tells you whether the block has an empty shape.
    ///
    /// This is slightly more efficient than calling `shape()` and comparing against `EMPTY_SHAPE`.
    fn is_collision_shape_empty(&self) -> bool;
    fn is_collision_shape_full(&self) -> bool;
}}

{generated_shape_code}


impl BlockWithShape for BlockState {{
    fn collision_shape(&self) -> &'static VoxelShape {{
        COLLISION_SHAPES_MAP.get(self.id() as usize).unwrap_or(&&SHAPE1)
    }}
    fn outline_shape(&self) -> &'static VoxelShape {{
        OUTLINE_SHAPES_MAP.get(self.id() as usize).unwrap_or(&&SHAPE1)
    }}

    fn is_collision_shape_empty(&self) -> bool {{
        matches!(self.id(), {empty_shape_match_code})
    }}

    fn is_collision_shape_full(&self) -> bool {{
        matches!(self.id(), {block_shape_match_code})
    }}
}}

{generated_map_code}
'''

    


def generate_code_for_shape(shape_id: str, parts: list[list[float]]):
    def make_arguments(part: list[float]):
        return ', '.join(map(lambda n: str(n).rstrip('0'), part))
    code = ''
    code += f'static SHAPE{shape_id}: LazyLock<VoxelShape> = LazyLock::new(|| {{'
    steps = []
    if parts == ():
        steps.append('collision::EMPTY_SHAPE.clone()')
    else:
        steps.append(f'collision::box_shape({make_arguments(parts[0])})')
        for part in parts[1:]:
            steps.append(
                f'Shapes::or(s, collision::box_shape({make_arguments(part)}))')

    if len(steps) == 1:
        code += steps[0]
    else:
        code += '{\n'
        for step in steps[:-1]:
            code += f'    let s = {step};\n'
        code += f'    {steps[-1]}\n'
        code += '}\n'
    code += '});\n'
    return code

def convert_ints_to_rust_ranges(block_state_ids: list[int]) -> str:
    # convert them into ranges (so like 1|2|3 is 1..=3 instead)
    block_state_ids_ranges = []
    range_start_block_state_id = None
    last_block_state_id = None
    for block_state_id in sorted(block_state_ids):
        if range_start_block_state_id is None:
            range_start_block_state_id = block_state_id

        if last_block_state_id is not None:
            # check if the range is done
            if block_state_id - 1 != last_block_state_id:
                block_state_ids_ranges.append(f'{range_start_block_state_id}..={last_block_state_id}' if range_start_block_state_id != last_block_state_id else str(range_start_block_state_id))
                range_start_block_state_id = block_state_id

        last_block_state_id = block_state_id

    block_state_ids_ranges.append(f'{range_start_block_state_id}..={last_block_state_id}' if range_start_block_state_id != last_block_state_id else str(range_start_block_state_id))
    return '|'.join(block_state_ids_ranges)
