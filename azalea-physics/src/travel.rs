use azalea_block::{Block, BlockState};
use azalea_core::{aabb::AABB, position::Vec3};
use azalea_entity::{
    metadata::Sprinting, move_relative, Attributes, InLoadedChunk, Jumping, LocalEntity,
    LookDirection, OnClimbable, Physics, Pose, Position,
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_ecs::prelude::*;

use crate::{
    collision::{move_colliding, MoverType},
    get_block_pos_below_that_affects_movement, handle_relative_friction_and_calculate_movement,
    HandleRelativeFrictionAndCalculateMovementOpts,
};

/// Move the entity with the given acceleration while handling friction,
/// gravity, collisions, and some other stuff.
#[allow(clippy::type_complexity)]
pub fn travel(
    mut query: Query<
        (
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
) {
    for (
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
                &mut physics,
                &direction,
                position,
                attributes,
                sprinting,
                on_climbable,
                &world,
            );
        } else {
            travel_in_air(
                &mut physics,
                &direction,
                position,
                &attributes,
                sprinting,
                &on_climbable,
                pose,
                &jumping,
                &world,
            );
        }
    }
}

/// The usual movement when we're not in water or using an elytra.
fn travel_in_air(
    physics: &mut Physics,
    direction: &LookDirection,
    position: Mut<Position>,
    attributes: &Attributes,
    sprinting: Sprinting,
    on_climbable: &OnClimbable,
    pose: Option<&Pose>,
    jumping: &Jumping,
    world: &Instance,
) {
    let gravity = get_effective_gravity();

    let block_pos_below = get_block_pos_below_that_affects_movement(&position);

    let block_state_below = world
        .chunks
        .get_block_state(&block_pos_below)
        .unwrap_or(BlockState::AIR);
    let block_below: Box<dyn Block> = block_state_below.into();
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
            world: &world,
            physics,
            direction: &direction,
            position,
            attributes,
            is_sprinting: *sprinting,
            on_climbable,
            pose,
            jumping,
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

fn travel_in_fluid(
    physics: &mut Physics,
    direction: &LookDirection,
    mut position: Mut<Position>,
    attributes: &Attributes,
    sprinting: Sprinting,
    on_climbable: &OnClimbable,
    world: &Instance,
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

        move_relative(physics, direction, speed, &acceleration);
        move_colliding(
            MoverType::Own,
            &physics.velocity.clone(),
            world,
            &mut position,
            physics,
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
        move_relative(physics, direction, 0.02, &acceleration);
        move_colliding(
            MoverType::Own,
            &physics.velocity.clone(),
            world,
            &mut position,
            physics,
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
            physics.bounding_box,
            world,
            velocity.x,
            velocity.y + 0.6 - position.y + y,
            velocity.z,
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
        let new_y_velocity;
        if moving_down
            && (new_velocity.y - 0.005).abs() >= 0.003
            && f64::abs(new_velocity.y - gravity / 16.0) < 0.003
        {
            new_y_velocity = -0.003;
        } else {
            new_y_velocity = new_velocity.y - gravity / 16.0;
        }

        Vec3 {
            x: new_velocity.x,
            y: new_y_velocity,
            z: new_velocity.z,
        }
    } else {
        new_velocity
    }
}

fn is_free(bounding_box: AABB, world: &Instance, x: f64, y: f64, z: f64) -> bool {
    // let bounding_box = bounding_box.move_relative(Vec3::new(x, y, z));

    let _ = (bounding_box, world, x, y, z);

    // TODO: implement this, see Entity.isFree

    true
}

fn get_effective_gravity() -> f64 {
    // TODO: slow falling effect
    0.08
}

fn fluid_jump_threshold() -> f64 {
    // this is 0.0 for entities with an eye height lower than 0.4, but that's not
    // implemented since it's usually not relevant for players (unless the player
    // was shrunk)
    0.4
}
