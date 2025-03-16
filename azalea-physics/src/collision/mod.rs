mod blocks;
mod discrete_voxel_shape;
pub mod entity_collisions;
mod mergers;
mod shape;
pub mod world_collisions;

use std::{ops::Add, sync::LazyLock};

use azalea_block::{BlockState, fluid_state::FluidState};
use azalea_core::{
    aabb::AABB,
    direction::Axis,
    math::EPSILON,
    position::{BlockPos, Vec3},
};
use azalea_world::{ChunkStorage, Instance, MoveEntityError};
use bevy_ecs::{entity::Entity, world::Mut};
pub use blocks::BlockWithShape;
pub use discrete_voxel_shape::*;
use entity_collisions::{CollidableEntityQuery, PhysicsQuery, get_entity_collisions};
pub use shape::*;
use tracing::warn;

use self::world_collisions::get_block_collisions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

// Entity.collide
fn collide(
    movement: &Vec3,
    world: &Instance,
    physics: &azalea_entity::Physics,
    source_entity: Option<Entity>,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
) -> Vec3 {
    let entity_bounding_box = physics.bounding_box;
    let entity_collisions = get_entity_collisions(
        world,
        &entity_bounding_box.expand_towards(movement),
        source_entity,
        physics_query,
        collidable_entity_query,
    );
    let collided_delta = if movement.length_squared() == 0.0 {
        *movement
    } else {
        collide_bounding_box(movement, &entity_bounding_box, world, &entity_collisions)
    };

    let x_collision = movement.x != collided_delta.x;
    let y_collision = movement.y != collided_delta.y;
    let z_collision = movement.z != collided_delta.z;

    let on_ground = physics.on_ground() || y_collision && movement.y < 0.;

    let max_up_step = 0.6;
    if max_up_step > 0. && on_ground && (x_collision || z_collision) {
        let mut step_to_delta = collide_bounding_box(
            &movement.with_y(max_up_step),
            &entity_bounding_box,
            world,
            &entity_collisions,
        );
        let directly_up_delta = collide_bounding_box(
            &Vec3::ZERO.with_y(max_up_step),
            &entity_bounding_box.expand_towards(&Vec3::new(movement.x, 0., movement.z)),
            world,
            &entity_collisions,
        );
        if directly_up_delta.y < max_up_step {
            let target_movement = collide_bounding_box(
                &movement.with_y(0.),
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
                &Vec3::ZERO.with_y(-step_to_delta.y + movement.y),
                &entity_bounding_box.move_relative(step_to_delta),
                world,
                &entity_collisions,
            ));
        }
    }

    collided_delta
}

/// Move an entity by a given delta, checking for collisions.
///
/// In Mojmap, this is `Entity.move`.
#[allow(clippy::too_many_arguments)]
pub fn move_colliding(
    _mover_type: MoverType,
    movement: &Vec3,
    world: &Instance,
    position: &mut Mut<azalea_entity::Position>,
    physics: &mut azalea_entity::Physics,
    source_entity: Option<Entity>,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
) -> Result<(), MoveEntityError> {
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

    // movement = this.maybeBackOffFromEdge(movement, moverType);

    let collide_result = collide(
        movement,
        world,
        physics,
        source_entity,
        physics_query,
        collidable_entity_query,
    );

    let move_distance = collide_result.length_squared();

    if move_distance > EPSILON {
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

    let x_collision = movement.x != collide_result.x;
    let z_collision = movement.z != collide_result.z;
    let horizontal_collision = x_collision || z_collision;
    let vertical_collision = movement.y != collide_result.y;
    let on_ground = vertical_collision && movement.y < 0.;

    physics.horizontal_collision = horizontal_collision;
    physics.vertical_collision = vertical_collision;
    physics.set_on_ground(on_ground);

    // TODO: minecraft checks for a "minor" horizontal collision here

    let _block_pos_below = azalea_entity::on_pos_legacy(&world.chunks, position);
    // let _block_state_below = self
    //     .world
    //     .get_block_state(&block_pos_below)
    //     .expect("Couldn't get block state below");

    // self.check_fall_damage(collide_result.y, on_ground, block_state_below,
    // block_pos_below);

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

fn collide_bounding_box(
    movement: &Vec3,
    entity_bounding_box: &AABB,
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
    movement: &Vec3,
    mut entity_box: AABB,
    collision_boxes: &Vec<VoxelShape>,
) -> Vec3 {
    if collision_boxes.is_empty() {
        return *movement;
    }

    let mut x_movement = movement.x;
    let mut y_movement = movement.y;
    let mut z_movement = movement.z;
    if y_movement != 0. {
        y_movement = Shapes::collide(&Axis::Y, &entity_box, collision_boxes, y_movement);
        if y_movement != 0. {
            entity_box = entity_box.move_relative(Vec3 {
                x: 0.,
                y: y_movement,
                z: 0.,
            });
        }
    }

    // whether the player is moving more in the z axis than x
    // this is done to fix a movement bug, minecraft does this too
    let more_z_movement = x_movement.abs() < z_movement.abs();

    if more_z_movement && z_movement != 0. {
        z_movement = Shapes::collide(&Axis::Z, &entity_box, collision_boxes, z_movement);
        if z_movement != 0. {
            entity_box = entity_box.move_relative(Vec3 {
                x: 0.,
                y: 0.,
                z: z_movement,
            });
        }
    }

    if x_movement != 0. {
        x_movement = Shapes::collide(&Axis::X, &entity_box, collision_boxes, x_movement);
        if x_movement != 0. {
            entity_box = entity_box.move_relative(Vec3 {
                x: x_movement,
                y: 0.,
                z: 0.,
            });
        }
    }

    if !more_z_movement && z_movement != 0. {
        z_movement = Shapes::collide(&Axis::Z, &entity_box, collision_boxes, z_movement);
    }

    Vec3 {
        x: x_movement,
        y: y_movement,
        z: z_movement,
    }
}

/// Get the [`VoxelShape`] for the given fluid state.
///
/// The instance and position are required so it can check if the block above is
/// also the same fluid type.
pub fn fluid_shape(
    fluid: &FluidState,
    world: &ChunkStorage,
    pos: &BlockPos,
) -> &'static VoxelShape {
    if fluid.amount == 9 {
        let fluid_state_above = world.get_fluid_state(&pos.up(1)).unwrap_or_default();
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
    let block_trait = Box::<dyn azalea_block::Block>::from(block);
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
