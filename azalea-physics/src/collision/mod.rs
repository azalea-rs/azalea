mod blocks;
mod discrete_voxel_shape;
pub mod entity_collisions;
mod mergers;
mod shape;
pub mod world_collisions;

use std::{ops::Add, sync::LazyLock};

use azalea_block::{BlockState, fluid_state::FluidState};
use azalea_core::{
    aabb::Aabb,
    direction::Axis,
    math::{self, EPSILON},
    position::{BlockPos, Vec3},
};
use azalea_entity::{
    Attributes, Jumping, LookDirection, OnClimbable, Physics, PlayerAbilities, Pose, Position,
    metadata::Sprinting,
};
use azalea_world::{ChunkStorage, Instance, MoveEntityError};
use bevy_ecs::{entity::Entity, world::Mut};
pub use blocks::BlockWithShape;
pub use discrete_voxel_shape::*;
use entity_collisions::{CollidableEntityQuery, get_entity_collisions};
pub use shape::*;
use tracing::warn;

use self::world_collisions::get_block_collisions;
use crate::{
    collision::entity_collisions::AabbQuery, local_player::PhysicsState, travel::no_collision,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

// Entity.collide
fn collide(ctx: &MoveCtx, movement: Vec3) -> Vec3 {
    let entity_bounding_box = ctx.physics.bounding_box;
    let entity_collisions = get_entity_collisions(
        ctx.world,
        &entity_bounding_box.expand_towards(movement),
        Some(ctx.source_entity),
        ctx.aabb_query,
        ctx.collidable_entity_query,
    );
    let world = ctx.world;
    let collided_delta = if movement.length_squared() == 0.0 {
        movement
    } else {
        collide_bounding_box(movement, &entity_bounding_box, world, &entity_collisions)
    };

    let x_collision = movement.x != collided_delta.x;
    let y_collision = movement.y != collided_delta.y;
    let z_collision = movement.z != collided_delta.z;

    let on_ground = ctx.physics.on_ground() || y_collision && movement.y < 0.;

    let max_up_step = 0.6;
    if max_up_step > 0. && on_ground && (x_collision || z_collision) {
        let mut step_to_delta = collide_bounding_box(
            movement.with_y(max_up_step),
            &entity_bounding_box,
            world,
            &entity_collisions,
        );
        let directly_up_delta = collide_bounding_box(
            Vec3::ZERO.with_y(max_up_step),
            &entity_bounding_box.expand_towards(Vec3::new(movement.x, 0., movement.z)),
            world,
            &entity_collisions,
        );
        if directly_up_delta.y < max_up_step {
            let target_movement = collide_bounding_box(
                movement.with_y(0.),
                &entity_bounding_box.move_relative(directly_up_delta),
                world,
                &entity_collisions,
            )
            .add(directly_up_delta);
            if target_movement.horizontal_distance_squared()
                > step_to_delta.horizontal_distance_squared()
            {
                step_to_delta = target_movement;
            }
        }

        if step_to_delta.horizontal_distance_squared()
            > collided_delta.horizontal_distance_squared()
        {
            return step_to_delta.add(collide_bounding_box(
                Vec3::ZERO.with_y(-step_to_delta.y + movement.y),
                &entity_bounding_box.move_relative(step_to_delta),
                world,
                &entity_collisions,
            ));
        }
    }

    collided_delta
}

pub struct MoveCtx<'world, 'state, 'a, 'b> {
    pub mover_type: MoverType,
    pub world: &'a Instance,
    pub position: Mut<'a, Position>,
    pub physics: &'a mut Physics,
    pub source_entity: Entity,
    pub aabb_query: &'a AabbQuery<'world, 'state, 'b>,
    pub collidable_entity_query: &'a CollidableEntityQuery<'world, 'state>,
    pub physics_state: Option<&'a PhysicsState>,
    pub attributes: &'a Attributes,
    pub abilities: Option<&'a PlayerAbilities>,

    pub direction: LookDirection,
    pub sprinting: Sprinting,
    pub on_climbable: OnClimbable,
    pub pose: Option<Pose>,
    pub jumping: Jumping,
}

/// Move an entity by a given delta, checking for collisions.
///
/// In Mojmap, this is `Entity.move`.
#[allow(clippy::too_many_arguments)]
pub fn move_colliding(ctx: &mut MoveCtx, mut movement: Vec3) -> Result<(), MoveEntityError> {
    // TODO: do all these

    // if self.no_physics {
    //     return;
    // };

    // if (var1 == MoverType.PISTON) {
    //     var2 = this.limitPistonMovement(var2);
    //     if (var2.equals(Vec3.ZERO)) {
    //        return;
    //     }
    // }

    // if (this.stuckSpeedMultiplier.lengthSqr() > 1.0E-7D) {
    //     var2 = var2.multiply(this.stuckSpeedMultiplier);
    //     this.stuckSpeedMultiplier = Vec3.ZERO;
    //     this.setDeltaMovement(Vec3.ZERO);
    // }

    movement = maybe_back_off_from_edge(ctx, movement);
    let collide_result = collide(ctx, movement);

    let move_distance_sqr = collide_result.length_squared();

    let position = &mut ctx.position;
    let physics = &mut *ctx.physics;
    let world = ctx.world;

    if move_distance_sqr > EPSILON || movement.length_squared() - move_distance_sqr < EPSILON {
        // TODO: fall damage

        let new_pos = {
            Vec3 {
                x: position.x + collide_result.x,
                y: position.y + collide_result.y,
                z: position.z + collide_result.z,
            }
        };

        if new_pos != ***position {
            ***position = new_pos;
        }
    }

    let x_collision = !math::equal(movement.x, collide_result.x);
    let z_collision = !math::equal(movement.z, collide_result.z);
    let horizontal_collision = x_collision || z_collision;
    physics.horizontal_collision = horizontal_collision;

    let vertical_collision = movement.y != collide_result.y;
    physics.vertical_collision = vertical_collision;
    let on_ground = vertical_collision && movement.y < 0.;
    physics.set_on_ground(on_ground);

    // TODO: minecraft checks for a "minor" horizontal collision here

    let block_pos_below = azalea_entity::on_pos_legacy(&world.chunks, **position);
    let block_state_below = world.get_block_state(block_pos_below).unwrap_or_default();

    check_fall_damage(
        physics,
        collide_result.y,
        block_state_below,
        block_pos_below,
    );

    // if self.isRemoved() { return; }

    if horizontal_collision {
        let delta_movement = &physics.velocity;
        physics.velocity = Vec3 {
            x: if x_collision { 0. } else { delta_movement.x },
            y: delta_movement.y,
            z: if z_collision { 0. } else { delta_movement.z },
        }
    }

    if vertical_collision {
        // blockBelow.updateEntityAfterFallOn(this.level, this);
        // the default implementation of updateEntityAfterFallOn sets the y movement to
        // 0
        physics.velocity.y = 0.;
    }

    if on_ground {
        // blockBelow.stepOn(this.level, blockPosBelow, blockStateBelow,
        // this);
    }

    // sounds

    // this.tryCheckInsideBlocks();

    // float var25 = this.getBlockSpeedFactor();
    // this.setDeltaMovement(this.getDeltaMovement().multiply((double)var25, 1.0D,
    // (double)var25)); if (this.level.getBlockStatesIfLoaded(this.
    // getBoundingBox().deflate(1.0E-6D)).noneMatch((var0) -> {
    //    return var0.is(BlockTags.FIRE) || var0.is(Blocks.LAVA);
    // })) {
    //    if (this.remainingFireTicks <= 0) {
    //       this.setRemainingFireTicks(-this.getFireImmuneTicks());
    //    }

    //    if (this.wasOnFire && (this.isInPowderSnow ||
    // this.isInWaterRainOrBubble())) {       this.
    // playEntityOnFireExtinguishedSound();    }
    // }

    // if (this.isOnFire() && (this.isInPowderSnow || this.isInWaterRainOrBubble()))
    // {    this.setRemainingFireTicks(-this.getFireImmuneTicks());
    // }

    Ok(())
}

fn check_fall_damage(
    physics: &mut Physics,
    delta_y: f64,
    _block_state_below: BlockState,
    _block_pos_below: BlockPos,
) {
    if !physics.is_in_water() && delta_y < 0. {
        physics.fall_distance -= delta_y as f32 as f64;
    }

    if physics.on_ground() {
        // vanilla calls block.fallOn here but it's not relevant for us

        physics.fall_distance = 0.;
    }
}

fn maybe_back_off_from_edge(move_ctx: &mut MoveCtx, mut movement: Vec3) -> Vec3 {
    let is_staying_on_ground_surface = move_ctx.physics_state.is_some_and(|s| s.trying_to_crouch);
    let max_up_step = get_max_up_step(move_ctx.attributes);

    let fall_ctx = CanFallAtLeastCtx {
        physics: move_ctx.physics,
        world: move_ctx.world,
        source_entity: move_ctx.source_entity,
        aabb_query: move_ctx.aabb_query,
        collidable_entity_query: move_ctx.collidable_entity_query,
    };

    let Some(abilities) = move_ctx.abilities else {
        return movement;
    };

    let is_backing_off = !abilities.flying
        && movement.y <= 0.
        && matches!(move_ctx.mover_type, MoverType::Own | MoverType::Player)
        && is_staying_on_ground_surface
        && is_above_ground(&fall_ctx, max_up_step);
    if !is_backing_off {
        return movement;
    }

    let min_movement = 0.05;
    let min_movement_x = movement.x.signum() * min_movement;
    let min_movement_z = movement.z.signum() * min_movement;

    while movement.x != 0. && can_fall_at_least(&fall_ctx, movement.x, 0., max_up_step as f64) {
        if movement.x.abs() <= min_movement {
            movement.x = 0.;
            break;
        }

        movement.x -= min_movement_x
    }
    while movement.z != 0. && can_fall_at_least(&fall_ctx, 0., movement.z, max_up_step as f64) {
        if movement.z.abs() <= min_movement {
            movement.z = 0.;
            break;
        }

        movement.z -= min_movement_z
    }
    while movement.x != 0.0
        && movement.z != 0.0
        && can_fall_at_least(&fall_ctx, movement.x, movement.z, max_up_step as f64)
    {
        if movement.x.abs() <= min_movement {
            movement.x = 0.;
        } else {
            movement.x -= min_movement_x;
        }
        if movement.z.abs() <= min_movement {
            movement.z = 0.;
        } else {
            movement.z -= min_movement_z;
        }
    }

    movement
}

fn get_max_up_step(attributes: &Attributes) -> f32 {
    // this would be different if we were riding an entity
    attributes.step_height.calculate() as f32
}

fn is_above_ground(ctx: &CanFallAtLeastCtx, max_up_step: f32) -> bool {
    ctx.physics.on_ground()
        && ctx.physics.fall_distance < max_up_step as f64
        && !can_fall_at_least(ctx, 0., 0., max_up_step as f64 - ctx.physics.fall_distance)
}

pub struct CanFallAtLeastCtx<'world, 'state, 'a, 'b> {
    physics: &'a Physics,
    world: &'a Instance,
    source_entity: Entity,
    aabb_query: &'a AabbQuery<'world, 'state, 'b>,
    collidable_entity_query: &'a CollidableEntityQuery<'world, 'state>,
}

fn can_fall_at_least(
    ctx: &CanFallAtLeastCtx,
    delta_x: f64,
    delta_z: f64,
    max_up_step: f64,
) -> bool {
    let aabb = ctx.physics.bounding_box;
    let aabb = Aabb {
        min: Vec3 {
            x: aabb.min.x + EPSILON + delta_x,
            y: aabb.min.y - max_up_step - EPSILON,
            z: aabb.min.z + EPSILON + delta_z,
        },
        max: Vec3 {
            x: aabb.max.x - EPSILON + delta_x,
            y: aabb.min.y,
            z: aabb.max.z - EPSILON + delta_z,
        },
    };
    no_collision(
        ctx.world,
        Some(ctx.source_entity),
        ctx.aabb_query,
        ctx.collidable_entity_query,
        ctx.physics,
        &aabb,
        false,
    )
}

fn collide_bounding_box(
    movement: Vec3,
    entity_bounding_box: &Aabb,
    world: &Instance,
    entity_collisions: &[VoxelShape],
) -> Vec3 {
    let mut collision_boxes: Vec<VoxelShape> = Vec::with_capacity(entity_collisions.len() + 1);

    if !entity_collisions.is_empty() {
        collision_boxes.extend_from_slice(entity_collisions);
    }

    // TODO: world border

    let block_collisions =
        get_block_collisions(world, &entity_bounding_box.expand_towards(movement));
    collision_boxes.extend(block_collisions);
    collide_with_shapes(movement, *entity_bounding_box, &collision_boxes)
}

fn collide_with_shapes(
    mut movement: Vec3,
    mut entity_box: Aabb,
    collision_boxes: &[VoxelShape],
) -> Vec3 {
    if collision_boxes.is_empty() {
        return movement;
    }

    if movement.y != 0. {
        movement.y = Shapes::collide(Axis::Y, &entity_box, collision_boxes, movement.y);
        if movement.y != 0. {
            entity_box = entity_box.move_relative(Vec3::new(0., movement.y, 0.));
        }
    }

    // whether the player is moving more in the z axis than x
    // this is done to fix a movement bug, minecraft does this too
    let more_z_movement = movement.x.abs() < movement.z.abs();

    if more_z_movement && movement.z != 0. {
        movement.z = Shapes::collide(Axis::Z, &entity_box, collision_boxes, movement.z);
        if movement.z != 0. {
            entity_box = entity_box.move_relative(Vec3::new(0., 0., movement.z));
        }
    }

    if movement.x != 0. {
        movement.x = Shapes::collide(Axis::X, &entity_box, collision_boxes, movement.x);
        if movement.x != 0. {
            entity_box = entity_box.move_relative(Vec3::new(movement.x, 0., 0.));
        }
    }

    if !more_z_movement && movement.z != 0. {
        movement.z = Shapes::collide(Axis::Z, &entity_box, collision_boxes, movement.z);
    }

    movement
}

/// Get the [`VoxelShape`] for the given fluid state.
///
/// The instance and position are required so it can check if the block above is
/// also the same fluid type.
pub fn fluid_shape(fluid: &FluidState, world: &ChunkStorage, pos: BlockPos) -> &'static VoxelShape {
    if fluid.amount == 9 {
        let fluid_state_above = world.get_fluid_state(pos.up(1)).unwrap_or_default();
        if fluid_state_above.kind == fluid.kind {
            return &BLOCK_SHAPE;
        }
    }
    if fluid.amount > 9 {
        warn!("Tried to calculate shape for fluid with height > 9: {fluid:?} at {pos}");
        return &EMPTY_SHAPE;
    }

    // pre-calculate these in a LazyLock so this function can return a
    // reference instead

    static FLUID_SHAPES: LazyLock<[VoxelShape; 10]> = LazyLock::new(|| {
        [
            calculate_shape_for_fluid(0),
            calculate_shape_for_fluid(1),
            calculate_shape_for_fluid(2),
            calculate_shape_for_fluid(3),
            calculate_shape_for_fluid(4),
            calculate_shape_for_fluid(5),
            calculate_shape_for_fluid(6),
            calculate_shape_for_fluid(7),
            calculate_shape_for_fluid(8),
            calculate_shape_for_fluid(9),
        ]
    });

    &FLUID_SHAPES[fluid.amount as usize]
}
fn calculate_shape_for_fluid(amount: u8) -> VoxelShape {
    box_shape(0.0, 0.0, 0.0, 1.0, (f32::from(amount) / 9.0) as f64, 1.0)
}

/// Whether the block is treated as "motion blocking".
///
/// This is marked as deprecated in Minecraft.
pub fn legacy_blocks_motion(block: BlockState) -> bool {
    if block == BlockState::AIR {
        // fast path
        return false;
    }

    let registry_block = azalea_registry::Block::from(block);
    legacy_calculate_solid(block)
        && registry_block != azalea_registry::Block::Cobweb
        && registry_block != azalea_registry::Block::BambooSapling
}

pub fn legacy_calculate_solid(block: BlockState) -> bool {
    // force_solid has to be checked before anything else
    let block_trait = Box::<dyn azalea_block::BlockTrait>::from(block);
    if let Some(solid) = block_trait.behavior().force_solid {
        return solid;
    }

    let shape = block.collision_shape();
    if shape.is_empty() {
        return false;
    }
    let bounds = shape.bounds();
    bounds.size() >= 0.7291666666666666 || bounds.get_size(Axis::Y) >= 1.0
}
