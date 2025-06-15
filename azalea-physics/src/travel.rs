use azalea_block::{BlockState, BlockTrait, fluid_state::FluidState};
use azalea_core::{
    aabb::AABB,
    position::{BlockPos, Vec3},
};
use azalea_entity::{
    Attributes, InLoadedChunk, Jumping, LocalEntity, LookDirection, OnClimbable, Physics, Pose,
    Position, metadata::Sprinting, move_relative,
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_ecs::prelude::*;

use crate::{
    HandleRelativeFrictionAndCalculateMovementOpts,
    collision::{
        MoverType, Shapes,
        entity_collisions::{CollidableEntityQuery, PhysicsQuery, get_entity_collisions},
        move_colliding,
    },
    get_block_pos_below_that_affects_movement, handle_relative_friction_and_calculate_movement,
};

/// Move the entity with the given acceleration while handling friction,
/// gravity, collisions, and some other stuff.
#[allow(clippy::type_complexity)]
pub fn travel(
    mut query: Query<
        (
            Entity,
            &mut Physics,
            &mut LookDirection,
            &mut Position,
            Option<&Sprinting>,
            Option<&Pose>,
            &Attributes,
            &InstanceName,
            &OnClimbable,
            &Jumping,
        ),
        (With<LocalEntity>, With<InLoadedChunk>),
    >,
    instance_container: Res<InstanceContainer>,
    physics_query: PhysicsQuery,
    collidable_entity_query: CollidableEntityQuery,
) {
    for (
        entity,
        mut physics,
        direction,
        position,
        sprinting,
        pose,
        attributes,
        world_name,
        on_climbable,
        jumping,
    ) in &mut query
    {
        let Some(world_lock) = instance_container.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        let sprinting = *sprinting.unwrap_or(&Sprinting(false));

        // TODO: elytras

        if physics.is_in_water() || physics.is_in_lava() {
            // minecraft also checks for `this.isAffectedByFluids() &&
            // !this.canStandOnFluid(fluidAtBlock)` here but it doesn't matter
            // for players
            travel_in_fluid(
                &world,
                entity,
                &mut physics,
                *direction,
                position,
                attributes,
                sprinting,
                on_climbable,
                &physics_query,
                &collidable_entity_query,
            );
        } else {
            travel_in_air(
                &world,
                entity,
                &mut physics,
                *direction,
                position,
                attributes,
                sprinting,
                *on_climbable,
                pose,
                *jumping,
                &physics_query,
                &collidable_entity_query,
            );
        }
    }
}

/// The usual movement when we're not in water or using an elytra.
#[allow(clippy::too_many_arguments)]
fn travel_in_air(
    world: &Instance,
    entity: Entity,
    physics: &mut Physics,
    direction: LookDirection,
    position: Mut<Position>,
    attributes: &Attributes,
    sprinting: Sprinting,
    on_climbable: OnClimbable,
    pose: Option<&Pose>,
    jumping: Jumping,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
) {
    let gravity = get_effective_gravity();

    let block_pos_below = get_block_pos_below_that_affects_movement(*position);

    let block_state_below = world
        .chunks
        .get_block_state(block_pos_below)
        .unwrap_or(BlockState::AIR);
    let block_below: Box<dyn BlockTrait> = block_state_below.into();
    let block_friction = block_below.behavior().friction;

    let inertia = if physics.on_ground() {
        block_friction * 0.91
    } else {
        0.91
    };

    // this applies the current delta
    let mut movement = handle_relative_friction_and_calculate_movement(
        HandleRelativeFrictionAndCalculateMovementOpts {
            block_friction,
            world,
            physics,
            direction,
            position,
            attributes,
            is_sprinting: *sprinting,
            on_climbable,
            pose: pose.copied(),
            jumping,
            entity,
            physics_query,
            collidable_entity_query,
        },
    );

    movement.y -= gravity;

    // if (this.shouldDiscardFriction()) {
    //     this.setDeltaMovement(movement.x, yMovement, movement.z);
    // } else {
    //     this.setDeltaMovement(movement.x * (double)inertia, yMovement *
    // 0.9800000190734863D, movement.z * (double)inertia); }

    // if should_discard_friction(self) {
    if false {
        physics.velocity = movement;
    } else {
        physics.velocity = Vec3 {
            x: movement.x * inertia as f64,
            y: movement.y * 0.9800000190734863f64,
            z: movement.z * inertia as f64,
        };
    }
}

#[allow(clippy::too_many_arguments)]
fn travel_in_fluid(
    world: &Instance,
    entity: Entity,
    physics: &mut Physics,
    direction: LookDirection,
    mut position: Mut<Position>,
    attributes: &Attributes,
    sprinting: Sprinting,
    on_climbable: &OnClimbable,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
) {
    let moving_down = physics.velocity.y <= 0.;
    let y = position.y;
    let gravity = get_effective_gravity();

    let acceleration = Vec3::new(
        physics.x_acceleration as f64,
        physics.y_acceleration as f64,
        physics.z_acceleration as f64,
    );

    if physics.was_touching_water {
        let mut water_movement_speed = if *sprinting { 0.9 } else { 0.8 };
        let mut speed = 0.02;
        let mut water_efficiency_modifier = attributes.water_movement_efficiency.calculate() as f32;
        if !physics.on_ground() {
            water_efficiency_modifier *= 0.5;
        }

        if water_efficiency_modifier > 0. {
            water_movement_speed += (0.54600006 - water_movement_speed) * water_efficiency_modifier;
            speed += (attributes.speed.calculate() as f32 - speed) * water_efficiency_modifier;
        }

        // if (this.hasEffect(MobEffects.DOLPHINS_GRACE)) {
        //     waterMovementSpeed = 0.96F;
        // }

        move_relative(physics, direction, speed, acceleration);
        move_colliding(
            MoverType::Own,
            physics.velocity,
            world,
            &mut position,
            physics,
            Some(entity),
            physics_query,
            collidable_entity_query,
        )
        .expect("Entity should exist");

        let mut new_velocity = physics.velocity;
        if physics.horizontal_collision && **on_climbable {
            // underwater ladders
            new_velocity.y = 0.2;
        }
        new_velocity.x *= water_movement_speed as f64;
        new_velocity.y *= 0.8;
        new_velocity.z *= water_movement_speed as f64;
        physics.velocity =
            get_fluid_falling_adjusted_movement(gravity, moving_down, new_velocity, sprinting);
    } else {
        move_relative(physics, direction, 0.02, acceleration);
        move_colliding(
            MoverType::Own,
            physics.velocity,
            world,
            &mut position,
            physics,
            Some(entity),
            physics_query,
            collidable_entity_query,
        )
        .expect("Entity should exist");

        if physics.lava_fluid_height <= fluid_jump_threshold() {
            physics.velocity.x *= 0.5;
            physics.velocity.y *= 0.8;
            physics.velocity.z *= 0.5;
            let new_velocity = get_fluid_falling_adjusted_movement(
                gravity,
                moving_down,
                physics.velocity,
                sprinting,
            );
            physics.velocity = new_velocity;
        } else {
            physics.velocity *= 0.5;
        }

        if gravity != 0.0 {
            physics.velocity.y -= gravity / 4.0;
        }
    }

    let velocity = physics.velocity;
    if physics.horizontal_collision
        && is_free(
            world,
            entity,
            physics_query,
            collidable_entity_query,
            physics,
            physics.bounding_box,
            velocity.up(0.6).down(position.y).up(y),
        )
    {
        physics.velocity.y = 0.3;
    }
}

fn get_fluid_falling_adjusted_movement(
    gravity: f64,
    moving_down: bool,
    new_velocity: Vec3,
    sprinting: Sprinting,
) -> Vec3 {
    if gravity != 0. && !*sprinting {
        let new_y_velocity = if moving_down
            && (new_velocity.y - 0.005).abs() >= 0.003
            && f64::abs(new_velocity.y - gravity / 16.0) < 0.003
        {
            -0.003
        } else {
            new_velocity.y - gravity / 16.0
        };

        Vec3 {
            x: new_velocity.x,
            y: new_y_velocity,
            z: new_velocity.z,
        }
    } else {
        new_velocity
    }
}

fn is_free(
    world: &Instance,
    source_entity: Entity,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
    entity_physics: &mut Physics,
    bounding_box: AABB,
    delta: Vec3,
) -> bool {
    let bounding_box = bounding_box.move_relative(delta);

    no_collision(
        world,
        Some(source_entity),
        physics_query,
        collidable_entity_query,
        entity_physics,
        &bounding_box,
        false,
    ) && !contains_any_liquid(world, bounding_box)
}

fn no_collision(
    world: &Instance,
    source_entity: Option<Entity>,
    physics_query: &PhysicsQuery,
    collidable_entity_query: &CollidableEntityQuery,
    entity_physics: &mut Physics,
    aabb: &AABB,
    include_liquid_collisions: bool,
) -> bool {
    let collisions = if include_liquid_collisions {
        crate::collision::world_collisions::get_block_and_liquid_collisions(world, aabb)
    } else {
        crate::collision::world_collisions::get_block_collisions(world, aabb)
    };

    for collision in collisions {
        if !collision.is_empty() {
            return false;
        }
    }

    if !get_entity_collisions(
        world,
        aabb,
        source_entity,
        physics_query,
        collidable_entity_query,
    )
    .is_empty()
    {
        false
    } else if source_entity.is_none() {
        true
    } else {
        let collision = border_collision(entity_physics, aabb);
        if let Some(collision) = collision {
            // !Shapes.joinIsNotEmpty(collision, Shapes.create(aabb), BooleanOp.AND);
            !Shapes::matches_anywhere(&collision.into(), &aabb.into(), |a, b| a && b)
        } else {
            true
        }
    }
}

fn border_collision(_entity_physics: &Physics, _aabb: &AABB) -> Option<AABB> {
    // TODO: implement world border, see CollisionGetter.borderCollision

    None
}

fn contains_any_liquid(world: &Instance, bounding_box: AABB) -> bool {
    let min = bounding_box.min.to_block_pos_floor();
    let max = bounding_box.max.to_block_pos_ceil();

    for x in min.x..max.x {
        for y in min.y..max.y {
            for z in min.z..max.z {
                let block_state = world
                    .chunks
                    .get_block_state(BlockPos::new(x, y, z))
                    .unwrap_or_default();
                if !FluidState::from(block_state).is_empty() {
                    return true;
                }
            }
        }
    }

    false
}

fn get_effective_gravity() -> f64 {
    // TODO: slow falling effect
    0.08
}

pub fn fluid_jump_threshold() -> f64 {
    // this is 0.0 for entities with an eye height lower than 0.4, but that's not
    // implemented since it's usually not relevant for players (unless the player
    // was shrunk)
    0.4
}
