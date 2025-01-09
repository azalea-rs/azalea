use azalea_block::{fluid_state::FluidState, BlockState};
use azalea_core::{
    block_hit_result::BlockHitResult,
    direction::Direction,
    math::{self, lerp, EPSILON},
    position::{BlockPos, Vec3},
};
use azalea_inventory::ItemStack;
use azalea_world::ChunkStorage;
use bevy_ecs::entity::Entity;

use crate::collision::{BlockWithShape, VoxelShape, EMPTY_SHAPE};

#[derive(Debug, Clone)]
pub struct ClipContext {
    pub from: Vec3,
    pub to: Vec3,
    pub block_shape_type: BlockShapeType,
    pub fluid_pick_type: FluidPickType,
    // pub collision_context: EntityCollisionContext,
}
impl ClipContext {
    /// Get the shape of given block, using the type of shape set in
    /// [`Self::block_shape_type`].
    pub fn block_shape(&self, block_state: BlockState) -> &VoxelShape {
        // minecraft passes in the world and blockpos to this function but it's not
        // actually necessary. it is for fluid_shape though
        match self.block_shape_type {
            BlockShapeType::Collider => block_state.collision_shape(),
            BlockShapeType::Outline => block_state.outline_shape(),
            BlockShapeType::Visual => block_state.collision_shape(),
            BlockShapeType::FallDamageResetting => {
                if azalea_registry::tags::blocks::FALL_DAMAGE_RESETTING
                    .contains(&azalea_registry::Block::from(block_state))
                {
                    block_state.collision_shape()
                } else {
                    &EMPTY_SHAPE
                }
            }
        }
    }

    pub fn fluid_shape(
        &self,
        fluid_state: FluidState,
        world: &ChunkStorage,
        pos: &BlockPos,
    ) -> &VoxelShape {
        if self.fluid_pick_type.can_pick(&fluid_state) {
            crate::collision::fluid_shape(&fluid_state, world, pos)
        } else {
            &EMPTY_SHAPE
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlockShapeType {
    /// The shape that's used for collision.
    Collider,
    /// The block outline that renders when your cursor is over a block.
    Outline,
    /// Used by entities when considering their line of sight.
    ///
    /// TODO: visual block shape isn't implemented (it'll just return the
    /// collider shape), that's correct for most blocks though
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
impl FluidPickType {
    pub fn can_pick(&self, fluid_state: &FluidState) -> bool {
        match self {
            Self::None => false,
            Self::SourceOnly => fluid_state.amount == 8,
            Self::Any => fluid_state.fluid != azalea_registry::Fluid::Empty,
            Self::Water => fluid_state.fluid == azalea_registry::Fluid::Water,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityCollisionContext {
    pub descending: bool,
    pub entity_bottom: f64,
    pub held_item: ItemStack,
    // pub can_stand_on_fluid: Box<dyn Fn(&FluidState) -> bool>,
    pub entity: Entity,
}

pub fn clip(chunk_storage: &ChunkStorage, context: ClipContext) -> BlockHitResult {
    traverse_blocks(
        context.from,
        context.to,
        context,
        |ctx, block_pos| {
            let block_state = chunk_storage.get_block_state(block_pos).unwrap_or_default();
            let fluid_state = FluidState::from(block_state);

            let block_shape = ctx.block_shape(block_state);
            let interaction_clip = clip_with_interaction_override(
                &ctx.from,
                &ctx.to,
                block_pos,
                block_shape,
                &block_state,
            );
            let fluid_shape = ctx.fluid_shape(fluid_state, chunk_storage, block_pos);
            let fluid_clip = fluid_shape.clip(&ctx.from, &ctx.to, block_pos);

            let distance_to_interaction = interaction_clip
                .map(|hit| ctx.from.distance_squared_to(&hit.location))
                .unwrap_or(f64::MAX);
            let distance_to_fluid = fluid_clip
                .map(|hit| ctx.from.distance_squared_to(&hit.location))
                .unwrap_or(f64::MAX);

            if distance_to_interaction <= distance_to_fluid {
                interaction_clip
            } else {
                fluid_clip
            }
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

fn clip_with_interaction_override(
    from: &Vec3,
    to: &Vec3,
    block_pos: &BlockPos,
    block_shape: &VoxelShape,
    _block_state: &BlockState,
) -> Option<BlockHitResult> {
    let block_hit_result = block_shape.clip(from, to, block_pos);

    if let Some(block_hit_result) = block_hit_result {
        // TODO: minecraft calls .getInteractionShape here
        // getInteractionShape is empty for almost every shape except cauldons,
        // compostors, hoppers, and scaffolding.
        let interaction_shape = &*EMPTY_SHAPE;
        let interaction_hit_result = interaction_shape.clip(from, to, block_pos);
        if let Some(interaction_hit_result) = interaction_hit_result {
            if interaction_hit_result.location.distance_squared_to(from)
                < block_hit_result.location.distance_squared_to(from)
            {
                return Some(block_hit_result.with_direction(interaction_hit_result.direction));
            }
        }

        Some(block_hit_result)
    } else {
        None
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
                1. - math::fract(right_before_start.x)
            } else {
                math::fract(right_before_start.x)
            },
        y: percentage_step.y
            * if vec_sign.y > 0. {
                1. - math::fract(right_before_start.y)
            } else {
                math::fract(right_before_start.y)
            },
        z: percentage_step.z
            * if vec_sign.z > 0. {
                1. - math::fract(right_before_start.z)
            } else {
                math::fract(right_before_start.z)
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
