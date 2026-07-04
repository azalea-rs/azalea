from lib.utils import get_dir_location

COLLISION_BLOCKS_RS_DIR = get_dir_location("../azalea-physics/src/collision/blocks.rs")


def generate_block_shapes(pumpkin_blocks_data: dict, block_states_report, burger_data):
    blocks, shapes = simplify_shapes(pumpkin_blocks_data)

    code = generate_block_shapes_code(blocks, shapes, block_states_report)
    with open(COLLISION_BLOCKS_RS_DIR, "w") as f:
        f.write(code)


def simplify_shapes(pumpkin_blocks: dict) -> tuple[dict, dict]:
    """
    Returns new_blocks and new_shapes,
    where new_blocks is like { grass_block: { collision: [1, 1], outline: [1, 1], offset_type: "XZ" } }
    and new_shapes is like { 1: [ [0, 0, 0, 1, 1, 1] ] }
    """
    new_blocks = {}
    new_shapes = {}

    all_shapes_ids = {}

    for pumpkin_block_data in pumpkin_blocks["blocks"]:
        new_block_collision_shapes = []
        new_block_outline_shapes = []

        block_id = pumpkin_block_data["name"]

        for state in pumpkin_block_data["states"]:
            collision_shape = []
            for box_id in state["collision_shapes"]:
                box = pumpkin_blocks["shapes"][box_id]
                collision_shape.append(tuple(box["min"] + box["max"]))
            outline_shape = []
            for box_id in state["outline_shapes"]:
                box = pumpkin_blocks["shapes"][box_id]
                outline_shape.append(tuple(box["min"] + box["max"]))

            collision_shape = tuple(collision_shape)
            outline_shape = tuple(outline_shape)

            if collision_shape in all_shapes_ids:
                collision_shape_id = all_shapes_ids[collision_shape]
            else:
                collision_shape_id = len(all_shapes_ids)
                all_shapes_ids[collision_shape] = collision_shape_id
                new_shapes[collision_shape_id] = collision_shape
            if outline_shape in all_shapes_ids:
                outline_shape_id = all_shapes_ids[outline_shape]
            else:
                outline_shape_id = len(all_shapes_ids)
                all_shapes_ids[outline_shape] = outline_shape_id
                new_shapes[outline_shape_id] = outline_shape

            new_block_collision_shapes.append(collision_shape_id)
            new_block_outline_shapes.append(outline_shape_id)

        new_data_for_block = {
            "collision": new_block_collision_shapes,
            "outline": new_block_outline_shapes,
        }

        if "offset_type" in pumpkin_block_data:
            # don't waste space in `new_data_for_block` if there's no offset_type
            new_data_for_block["offset_type"] = pumpkin_block_data["offset_type"]

        new_blocks[block_id] = new_data_for_block

    return new_blocks, new_shapes


def generate_block_shapes_code(blocks: dict, shapes: dict, block_states_report):
    # look at __cache__/generator-mod-*/blockCollisionShapes.json for format of blocks and shapes

    generated_shape_code = ""
    for shape_id, shape in sorted(shapes.items(), key=lambda shape: int(shape[0])):
        generated_shape_code += generate_code_for_shape(shape_id, shape)

    # static COLLISION_SHAPES_MAP: [&LazyLock<VoxelShape>; 26644] = [&SHAPE0, &SHAPE1, &SHAPE1, ...]
    empty_shapes = []
    full_shapes = []

    # the index into this list is the block state id
    collision_shapes_map = []
    outline_shapes_map = []
    random_shape_offsets_map = []

    for block_id, shapes_data in blocks.items():
        collision_shapes = shapes_data["collision"]
        outline_shapes = shapes_data["outline"]
        # None, XZ, or XYZ
        offset_type = shapes_data.get("offset_type")

        if isinstance(collision_shapes, int):
            collision_shapes = [collision_shapes]
        if isinstance(outline_shapes, int):
            outline_shapes = [outline_shapes]

        # these are ids that we semi-arbitrarily chose. we'll have to check these ids in
        # shape_offset.rs. btw, the reason we don't use an enum is because that'd generate too
        # much code -- numbers are shorter.
        if offset_type is None:
            offset_type_id = 0
        elif offset_type == "XZ":
            offset_type_id = 1
        elif offset_type == "XYZ":
            offset_type_id = 2
        else:
            raise Exception(f"Invalid offset type from Burger: {offset_type}")

        block_report_data = block_states_report["minecraft:" + block_id]

        for possible_state, shape_id in zip(
            block_report_data["states"], collision_shapes
        ):
            block_state_id = possible_state["id"]
            if shape_id == 0:
                empty_shapes.append(block_state_id)
            elif shape_id == 1:
                full_shapes.append(block_state_id)
            while len(collision_shapes_map) <= block_state_id:
                # default to shape 1 for missing shapes (full block)
                collision_shapes_map.append(1)
            collision_shapes_map[block_state_id] = shape_id
        for possible_state, shape_id in zip(
            block_report_data["states"], outline_shapes
        ):
            block_state_id = possible_state["id"]
            while len(outline_shapes_map) <= block_state_id:
                # default to shape 1 for missing shapes (full block)
                outline_shapes_map.append(1)

                # and default to random offset type 0. this is fine to put in the same loop because
                # we expect them to end up as the same length and we only ever append to them here.
                random_shape_offsets_map.append(0)

            outline_shapes_map[block_state_id] = shape_id
            # yes, despite offsetTypes being per-blockkind, we make the map based on blockstates.
            # this is because azalea usually keeps blocks as blockstates, and we'd like to avoid
            # the conversion cost whenever possible.
            random_shape_offsets_map[block_state_id] = offset_type_id

    random_shape_offsets_map = ",".join(map(str, random_shape_offsets_map))

    generated_map_code = f"static COLLISION_SHAPES_MAP: [&LazyLock<VoxelShape>; {len(collision_shapes_map)}] = ["
    empty_shape_match_code = convert_ints_to_rust_ranges(empty_shapes)

    simple_collision_shapes_map = [2] * len(collision_shapes_map)
    for block_state_id, shape_id in enumerate(collision_shapes_map):
        generated_map_code += f"&SHAPE{shape_id},\n"
        if shape_id == 0:
            simple_collision_shapes_map[block_state_id] = 0
        elif shape_id == 1:
            simple_collision_shapes_map[block_state_id] = 1
    simple_collision_shapes_map = ",".join(map(str, simple_collision_shapes_map))
    generated_map_code += "];\n"

    generated_map_code += f"static OUTLINE_SHAPES_MAP: [&LazyLock<VoxelShape>; {len(outline_shapes_map)}] = ["
    for block_state_id, shape_id in enumerate(outline_shapes_map):
        generated_map_code += f"&SHAPE{shape_id},\n"
    generated_map_code += "];\n"

    if empty_shape_match_code == "":
        print("Error: shape 0 was not found")

    return f"""
//! Autogenerated block collisions for every block

// This file is @generated from codegen/lib/code/shapes.py. If you want to
// modify it, change that file.

#![allow(
    clippy::explicit_auto_deref,
    clippy::redundant_closure,
    clippy::needless_borrow
)]

use std::{{borrow::Cow, sync::LazyLock}};

use azalea_block::*;
use azalea_core::position::BlockPos;

use super::VoxelShape;
use crate::collision::{{self, Shapes}};

pub trait BlockWithShape {{
    /// The hitbox for blocks that's used when simulating physics.
    fn collision_shape(&self, pos: BlockPos) -> Cow<'static, VoxelShape>;
    /// The hitbox for blocks that's used for determining whether we're looking
    /// at it.
    ///
    /// This is often but not always the same as the collision shape. For
    /// example, tall grass has a normal outline shape but an empty collision
    /// shape.
    fn outline_shape(&self, pos: BlockPos) -> Cow<'static, VoxelShape>;

    /// The collision shape of the block, before applying random coordinate
    /// offsets.
    ///
    /// This is almost always the same as [`Self::collision_shape`], except for
    /// a few blocks like bamboo.
    fn base_collision_shape(&self) -> &'static VoxelShape;
    /// The outline shape of the block, before applying random coordinate
    /// offsets.
    ///
    /// This is almost always the same as [`Self::outline_shape`], except for
    /// a few blocks like bamboo.
    fn base_outline_shape(&self) -> &'static VoxelShape;

    /// Tells you whether the block has an empty shape.
    ///
    /// This is slightly more efficient than calling [`Self::collision_shape`]
    /// and comparing against `EMPTY_SHAPE`.
    fn is_collision_shape_empty(&self) -> bool;
    /// Returns true if the block's shape is exactly 1×1×1.
    fn is_collision_shape_full(&self) -> bool;
}}

{generated_shape_code}

impl BlockWithShape for BlockState {{
    fn collision_shape(&self, pos: BlockPos) -> Cow<'static, VoxelShape> {{
        super::shape_offset::apply_shape_offset(*self, pos, self.base_collision_shape())
    }}
    fn outline_shape(&self, pos: BlockPos) -> Cow<'static, VoxelShape> {{
        super::shape_offset::apply_shape_offset(*self, pos, self.base_outline_shape())
    }}

    fn base_collision_shape(&self) -> &'static VoxelShape {{
        COLLISION_SHAPES_MAP
            .get(self.id() as usize)
            .unwrap_or(&&SHAPE1)
    }}
    fn base_outline_shape(&self) -> &'static VoxelShape {{
        OUTLINE_SHAPES_MAP
            .get(self.id() as usize)
            .unwrap_or(&&SHAPE1)
    }}

    fn is_collision_shape_empty(&self) -> bool {{
        BASIC_COLLISION_SHAPES_MAP[self.id() as usize] == 0
    }}

    fn is_collision_shape_full(&self) -> bool {{
        BASIC_COLLISION_SHAPES_MAP[self.id() as usize] == 1
    }}
}}

static BASIC_COLLISION_SHAPES_MAP: &[u8; {len(collision_shapes_map)}] = &[{simple_collision_shapes_map}];

pub static RANDOM_SHAPE_OFFSETS_MAP: &[u8; {len(collision_shapes_map)}] = &[{random_shape_offsets_map}];

{generated_map_code}
"""


def generate_code_for_shape(shape_id: str, parts: list[list[float]]):
    def make_arguments(part: list[float]):
        return ", ".join(map(lambda n: str(n).rstrip("0"), part))

    code = ""
    if parts == ():
        code += (
            f"static SHAPE{shape_id}: &LazyLock<VoxelShape> = &collision::EMPTY_SHAPE;"
        )
    else:
        code += f"static SHAPE{shape_id}: LazyLock<VoxelShape> = LazyLock::new(|| {{"

        steps = []

        steps.append(f"collision::box_shape({make_arguments(parts[0])})")
        for part in parts[1:]:
            steps.append(f"Shapes::or(s, collision::box_shape({make_arguments(part)}))")

        if len(steps) == 1:
            code += steps[0]
        else:
            code += "{\n"
            for step in steps[:-1]:
                code += f"    let s = {step};\n"
            code += f"    {steps[-1]}\n"
            code += "}\n"
        code += "});\n"
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
                block_state_ids_ranges.append(
                    f"{range_start_block_state_id}..={last_block_state_id}"
                    if range_start_block_state_id != last_block_state_id
                    else str(range_start_block_state_id)
                )
                range_start_block_state_id = block_state_id

        last_block_state_id = block_state_id

    block_state_ids_ranges.append(
        f"{range_start_block_state_id}..={last_block_state_id}"
        if range_start_block_state_id != last_block_state_id
        else str(range_start_block_state_id)
    )
    return "|".join(block_state_ids_ranges)
