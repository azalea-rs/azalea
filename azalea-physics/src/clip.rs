use azalea_block::BlockState;
use azalea_core::{math::lerp, BlockHitResult, BlockPos, Direction, Vec3, EPSILON};
use azalea_inventory::ItemSlot;
use azalea_world::ChunkStorage;
use bevy_ecs::entity::Entity;

use crate::collision::{BlockWithShape, VoxelShape};

#[derive(Debug, Clone)]
pub struct ClipContext {
    pub from: Vec3,
    pub to: Vec3,
    pub block_shape_type: BlockShapeType,
    pub fluid_pick_type: FluidPickType,
    // pub collision_context: EntityCollisionContext,
}
impl ClipContext {
    // minecraft passes in the world and blockpos here... but it doesn't actually
    // seem necessary?
    pub fn block_shape(&self, block_state: BlockState) -> &VoxelShape {
        // TODO: implement the other shape getters
        // (see the ClipContext.Block class in the vanilla source)
        match self.block_shape_type {
            BlockShapeType::Collider => block_state.shape(),
            BlockShapeType::Outline => block_state.shape(),
            BlockShapeType::Visual => block_state.shape(),
            BlockShapeType::FallDamageResetting => block_state.shape(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlockShapeType {
    Collider,
    Outline,
    Visual,
    FallDamageResetting,
}
#[derive(Debug, Copy, Clone)]
pub enum FluidPickType {
    None,
    SourceOnly,
    Any,
    Water,
}
#[derive(Debug, Clone)]
pub struct EntityCollisionContext {
    pub descending: bool,
    pub entity_bottom: f64,
    pub held_item: ItemSlot,
    // pub can_stand_on_fluid: Box<dyn Fn(&FluidState) -> bool>,
    pub entity: Entity,
}

pub fn clip(chunk_storage: &ChunkStorage, context: ClipContext) -> BlockHitResult {
    traverse_blocks(
        context.from,
        context.to,
        context,
        |context, block_pos| {
            let block_state = chunk_storage.get_block_state(block_pos).unwrap_or_default();
            // TODO: add fluid stuff to this (see getFluidState in vanilla source)
            let block_shape = context.block_shape(block_state);
            clip_with_interaction_override(
                &context.from,
                &context.to,
                block_pos,
                block_shape,
                &block_state,
            )
            // let block_distance = if let Some(block_hit_result) =
            // block_hit_result {     context.from.distance_to_sqr(&
            // block_hit_result.location) } else {
            //     f64::MAX
            // };
        },
        |context| {
            let vec = context.from - context.to;
            BlockHitResult::miss(
                context.to,
                Direction::nearest(vec),
                BlockPos::from(context.to),
            )
        },
    )
}

// default BlockHitResult clipWithInteractionOverride(Vec3 world, Vec3 from,
// BlockPos to, VoxelShape shape,     BlockState block) {
//  BlockHitResult blockHitResult = shape.clip(world, from, to);
//  if (blockHitResult != null) {
//     BlockHitResult var7 = block.getInteractionShape(this, to).clip(world,
// from, to);     if (var7 != null
//           && var7.getLocation().subtract(world).lengthSqr() <
// blockHitResult.getLocation().subtract(world).lengthSqr()) {        return
// blockHitResult.withDirection(var7.getDirection());     }
//  }

//  return blockHitResult;
// }
fn clip_with_interaction_override(
    from: &Vec3,
    to: &Vec3,
    block_pos: &BlockPos,
    block_shape: &VoxelShape,
    block_state: &BlockState,
) -> Option<BlockHitResult> {
    let block_hit_result = block_shape.clip(from, to, block_pos);
    if let Some(block_hit_result) = block_hit_result {
        // TODO: minecraft calls .getInteractionShape here
        // are there even any blocks that have a physics shape different from the
        // interaction shape???
        // (if not then you can delete this comment)
        // (if there are then you have to implement BlockState::interaction_shape, lol
        // have fun)
        let interaction_shape = block_state.shape();
        let interaction_hit_result = interaction_shape.clip(from, to, block_pos);
        if let Some(interaction_hit_result) = interaction_hit_result {
            if interaction_hit_result.location.distance_to_sqr(from)
                < block_hit_result.location.distance_to_sqr(from)
            {
                return Some(block_hit_result.with_direction(interaction_hit_result.direction));
            }
        }
        Some(block_hit_result)
    } else {
        block_hit_result
    }
}

pub fn traverse_blocks<C, T>(
    from: Vec3,
    to: Vec3,
    context: C,
    get_hit_result: impl Fn(&C, &BlockPos) -> Option<T>,
    get_miss_result: impl Fn(&C) -> T,
) -> T {
    if from == to {
        return get_miss_result(&context);
    }

    let right_after_end = Vec3 {
        x: lerp(-EPSILON, to.x, from.x),
        y: lerp(-EPSILON, to.y, from.y),
        z: lerp(-EPSILON, to.z, from.z),
    };

    let right_before_start = Vec3 {
        x: lerp(-EPSILON, from.x, to.x),
        y: lerp(-EPSILON, from.y, to.y),
        z: lerp(-EPSILON, from.z, to.z),
    };

    let mut current_block = BlockPos::from(right_before_start);
    if let Some(data) = get_hit_result(&context, &current_block) {
        return data;
    }

    let vec = right_after_end - right_before_start;

    /// Returns either -1, 0, or 1, depending on whether the number is negative,
    /// zero, or positive.
    ///
    /// This function exists because f64::signum doesn't check for 0.
    fn get_number_sign(num: f64) -> f64 {
        if num == 0. {
            0.
        } else {
            num.signum()
        }
    }

    let vec_sign = Vec3 {
        x: get_number_sign(vec.x),
        y: get_number_sign(vec.y),
        z: get_number_sign(vec.z),
    };

    #[rustfmt::skip]
    let percentage_step = Vec3 {
        x: if vec_sign.x == 0. { f64::MAX } else { vec_sign.x / vec.x },
        y: if vec_sign.y == 0. { f64::MAX } else { vec_sign.y / vec.y },
        z: if vec_sign.z == 0. { f64::MAX } else { vec_sign.z / vec.z },
    };

    let mut percentage = Vec3 {
        x: percentage_step.x
            * if vec_sign.x > 0. {
                1. - right_before_start.x.fract()
            } else {
                right_before_start.x.fract().abs()
            },
        y: percentage_step.y
            * if vec_sign.y > 0. {
                1. - right_before_start.y.fract()
            } else {
                right_before_start.y.fract().abs()
            },
        z: percentage_step.z
            * if vec_sign.z > 0. {
                1. - right_before_start.z.fract()
            } else {
                right_before_start.z.fract().abs()
            },
    };

    loop {
        if percentage.x > 1. && percentage.y > 1. && percentage.z > 1. {
            return get_miss_result(&context);
        }

        if percentage.x < percentage.y {
            if percentage.x < percentage.z {
                current_block.x += vec_sign.x as i32;
                percentage.x += percentage_step.x;
            } else {
                current_block.z += vec_sign.z as i32;
                percentage.z += percentage_step.z;
            }
        } else if percentage.y < percentage.z {
            current_block.y += vec_sign.y as i32;
            percentage.y += percentage_step.y;
        } else {
            current_block.z += vec_sign.z as i32;
            percentage.z += percentage_step.z;
        }

        if let Some(data) = get_hit_result(&context, &current_block) {
            return data;
        }
    }
}
