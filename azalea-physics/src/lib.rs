#![doc = include_str!("../README.md")]
#![feature(trait_alias)]

pub mod client_movement;
pub mod clip;
pub mod collision;
pub mod fluids;
pub mod support;
pub mod travel;

use std::{collections::HashSet, ops::Add};

use azalea_block::{BlockState, fluid_state::FluidState, properties};
use azalea_core::{
    math,
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    ActiveEffects, Attributes, EntityGeometryUpdateSystems, EntityKindComponent, GroundContact,
    HasClientLoaded, Jumping, LocalEntity, LookDirection, MovementResult, OnClimbable, Physics,
    Pose, Position, StuckSpeedMultiplier,
    dimensions::EntityDimensions,
    metadata::{AbstractLiving, Sprinting},
    move_relative, on_pos, on_pos_legacy,
};
use azalea_registry::{
    builtin::{BlockKind, EntityKind, MobEffect},
    tags::blocks::SUPPRESSES_BOUNCE,
};
use azalea_world::{ChunkStorage, World, WorldName, Worlds};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use clip::box_traverse_blocks;
use collision::{BLOCK_SHAPE, BlockWithShape, VoxelShape, move_colliding};

use crate::{
    client_movement::ClientMovementState,
    collision::{MoveCtx, entity_collisions::update_last_bounding_box},
    support::{
        clear_server_update_flag, update_main_supporting_block_pos_from_server,
        update_main_supporting_block_pos_local,
    },
    travel::{get_effective_gravity, travel_post_move},
};

/// A Bevy [`SystemSet`] for running physics that makes entities do things.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PhysicsSystems;

/// A Bevy [`SystemSet`] for running the original travel function (now broken
/// into multiple systems)
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct TravelSystems;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            (
                fluids::update_in_water_state_and_do_fluid_pushing,
                update_old_position,
                fluids::update_swimming,
                ai_step,
                (
                    travel::travel_until_moved.before(EntityGeometryUpdateSystems),
                    update_main_supporting_block_pos_local.after(EntityGeometryUpdateSystems),
                    update_falling_distance,
                    bounce_on_block,
                    apply_speed_factor,
                    travel_post_move,
                )
                    .chain()
                    .in_set(TravelSystems),
                apply_effects_from_blocks,
            )
                .chain()
                .in_set(PhysicsSystems)
                .after(azalea_entity::update_in_loaded_chunk),
        )
        // we want this to happen after packets are handled but before physics
        .add_systems(
            Update,
            (
                update_main_supporting_block_pos_from_server,
                clear_server_update_flag,
                update_last_bounding_box,
            )
                .chain()
                .after(azalea_entity::update_bounding_box),
        );
    }
}

/// Applies air resistance and handles jumping.
///
/// Happens before [`travel::travel`].
#[allow(clippy::type_complexity)]
pub fn ai_step(
    mut query: Query<
        (
            &mut Physics,
            Option<&Jumping>,
            &GroundContact,
            &Position,
            &LookDirection,
            &Sprinting,
            &ActiveEffects,
            &WorldName,
            &EntityKindComponent,
            &ClientMovementState,
        ),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (
        mut physics,
        jumping,
        ground_contact,
        position,
        look_direction,
        sprinting,
        active_effects,
        world_name,
        entity_kind,
        client_movement,
    ) in &mut query
    {
        let is_player = **entity_kind == EntityKind::Player;

        // vanilla does movement interpolation here, doesn't really matter much for a
        // bot though

        if physics.no_jump_delay > 0 {
            physics.no_jump_delay -= 1;
        }

        if is_player {
            if physics.velocity.horizontal_distance_squared() < 9.0e-6 {
                physics.velocity.x = 0.;
                physics.velocity.z = 0.;
            }
        } else {
            if physics.velocity.x.abs() < 0.003 {
                physics.velocity.x = 0.;
            }
            if physics.velocity.z.abs() < 0.003 {
                physics.velocity.z = 0.;
            }
        }

        if physics.velocity.y.abs() < 0.003 {
            physics.velocity.y = 0.;
        }

        if is_player {
            // handled in local_player_ai_step
        } else {
            physics.x_acceleration *= 0.98;
            physics.z_acceleration *= 0.98;
        }

        if client_movement.trying_to_crouch && physics.is_in_water() {
            go_down_in_water(&mut physics);
        }

        if jumping == Some(&Jumping(true)) {
            let fluid_height = if physics.is_in_lava() {
                physics.lava_fluid_height
            } else if physics.is_in_water() {
                physics.water_fluid_height
            } else {
                0.
            };

            let in_water = physics.is_in_water() && fluid_height > 0.;
            let fluid_jump_threshold = travel::fluid_jump_threshold();

            if !in_water || ground_contact.on_ground() && fluid_height <= fluid_jump_threshold {
                if !physics.is_in_lava()
                    || ground_contact.on_ground() && fluid_height <= fluid_jump_threshold
                {
                    if (ground_contact.on_ground()
                        || in_water && fluid_height <= fluid_jump_threshold)
                        && physics.no_jump_delay == 0
                    {
                        jump_from_ground(
                            &mut physics,
                            ground_contact,
                            *position,
                            *look_direction,
                            *sprinting,
                            world_name,
                            &worlds,
                            active_effects,
                        );
                        physics.no_jump_delay = 10;
                    }
                } else {
                    jump_in_liquid(&mut physics);
                }
            } else {
                jump_in_liquid(&mut physics);
            }
        } else {
            physics.no_jump_delay = 0;
        }

        // TODO: freezing, pushEntities, drowning damage (in their own systems,
        // after `travel`)
    }
}

fn jump_in_liquid(physics: &mut Physics) {
    physics.velocity.y += 0.04f32 as f64;
}

fn go_down_in_water(physics: &mut Physics) {
    physics.velocity.y -= 0.04f32 as f64;
}

// in minecraft, this is done as part of aiStep immediately after travel
#[allow(clippy::type_complexity)]
pub fn apply_effects_from_blocks(
    mut query: Query<
        (
            &mut Physics,
            &mut StuckSpeedMultiplier,
            &ActiveEffects,
            &GroundContact,
            &Position,
            &EntityDimensions,
            &WorldName,
            Option<&ClientMovementState>,
        ),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (
        mut physics,
        mut stuck_speed_multiplier,
        effects,
        ground_contact,
        position,
        dimensions,
        world_name,
        client_movement,
    ) in &mut query
    {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        // if !is_affected_by_blocks {
        //     continue
        // }

        if ground_contact.on_ground() {
            let block_pos = on_pos_legacy(&world.chunks, *position, ground_contact);
            if let Some(state) = world.chunks.get_block_state(block_pos)
                && let Some(client_movement) = client_movement
            {
                handle_entity_step_on(state.as_block_kind(), client_movement, &mut physics);
            }
        }

        // minecraft adds more entries to the list when the code is running on the
        // server
        let movement_this_tick = [EntityMovement {
            from: physics.old_position,
            to: **position,
        }];

        check_inside_blocks(
            &mut physics,
            &mut stuck_speed_multiplier,
            effects,
            ground_contact,
            position,
            dimensions,
            &world,
            &movement_this_tick,
        );
    }
}

/// Entity.restituteMovementAfterCollisions
/// restitute is not as straight forward as bounce, right
#[allow(clippy::type_complexity)]
pub fn bounce_on_block(
    mut query: Query<
        (
            &MovementResult,
            &ClientMovementState,
            &Attributes,
            &Position,
            &GroundContact,
            &WorldName,
            Option<&AbstractLiving>,
            &mut Physics,
        ),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (
        movement_result,
        client_movement,
        attributes,
        position,
        ground_contact,
        world_name,
        living,
        mut physics,
    ) in &mut query
    {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        let block_pos_below =
            azalea_entity::on_pos_legacy(&world.chunks, *position, ground_contact);
        let block_state_below = world.get_block_state(block_pos_below).unwrap_or_default();

        let mut restitution = if client_movement.trying_to_crouch {
            0.0
        } else {
            attributes.bounciness.calculate()
        };
        let mut velocity = physics.velocity;
        if movement_result.x_collision() {
            velocity = velocity.with_x(-physics.velocity.x * restitution);
        }

        if movement_result.z_collision() {
            velocity = velocity.with_z(-physics.velocity.z * restitution);
        }

        let velocity = if movement_result.vertical_collision() {
            if movement_result.vertical_collision_below() {
                restitution = if -physics.velocity.y >= get_effective_gravity()
                    && !client_movement.trying_to_crouch
                    && !SUPPRESSES_BOUNCE.contains(&block_state_below.as_block_kind())
                {
                    let bounciness = block_state_below.as_block_state().behavior().bounciness;
                    restitution.max(if living.is_some() {
                        bounciness as f64
                    } else {
                        (bounciness * 0.8f32) as f64
                    })
                } else {
                    0.0
                };
            }

            let (gravity_compensation, effective_drag) = if restitution > 0.0 {
                let portion_with_movement = movement_result.actual.y / physics.velocity.y;

                (
                    portion_with_movement * get_effective_gravity(),
                    azalea_core::math::lerp(
                        portion_with_movement,
                        1.0,
                        attributes.air_drag.calculate(),
                    ),
                )
            } else {
                (0.0, 1.0)
            };

            velocity
                .with_y((gravity_compensation - physics.velocity.y) * effective_drag * restitution)
        } else {
            velocity
        };

        physics.velocity = velocity;
    }
}

#[allow(clippy::type_complexity)]
pub fn apply_speed_factor(
    mut query: Query<
        (&mut Physics, &Position, &GroundContact, &WorldName),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (mut physics, position, ground_contact, world_name) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        if let Some(block_state) = world.chunks.get_block_state(BlockPos::from(position)) {
            let speed_factor = block_state.behavior().speed_factor;

            let speed_factor = if block_state.as_block_kind() == BlockKind::BubbleColumn
                || block_state.as_block_kind() == BlockKind::Water
            {
                speed_factor
            } else if speed_factor == 1.0f32 {
                world
                    .chunks
                    .get_block_state(get_block_pos_below_that_affects_movement(
                        &world.chunks,
                        *position,
                        ground_contact,
                    ))
                    .unwrap_or(BlockState::from(BlockKind::VoidAir))
                    .behavior()
                    .speed_factor
            } else {
                speed_factor
            };

            physics.velocity =
                physics
                    .velocity
                    .multiply(speed_factor as f64, 1.0, speed_factor as f64)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn check_inside_blocks(
    physics: &mut Physics,
    stuck_speed_multipler: &mut StuckSpeedMultiplier,
    effect: &ActiveEffects,
    ground_contact: &GroundContact,
    position: &Position,
    dimensions: &EntityDimensions,
    world: &World,
    movements: &[EntityMovement],
) -> Vec<BlockPos> {
    let mut blocks_inside = Vec::new();
    let mut visited_blocks = HashSet::<BlockPos>::new();

    for movement in movements {
        let bounding_box_at_target = dimensions
            .make_bounding_box(movement.to)
            .deflate_all(1.0E-5);

        let moved_far = movement.from.distance_squared_to(movement.to)
            > 0.9999900000002526 * 0.9999900000002526;

        for traversed_block in
            box_traverse_blocks(movement.from, movement.to, &bounding_box_at_target)
        {
            // if (!this.isAlive()) {
            //     return;
            // }

            let traversed_block_state = world.get_block_state(traversed_block).unwrap_or_default();
            if traversed_block_state.is_air() {
                continue;
            }
            if !visited_blocks.insert(traversed_block) {
                continue;
            }

            /*
            VoxelShape var12 = traversedBlockState.getEntityInsideCollisionShape(this.level(), traversedBlock);
            if (var12 != Shapes.block() && !this.collidedWithShapeMovingFrom(from, to, traversedBlock, var12)) {
               continue;
            }

            traversedBlockState.entityInside(this.level(), traversedBlock, this);
            this.onInsideBlock(traversedBlockState);
            */

            // this is different for end portal frames and tripwire hooks, i don't think it
            // actually matters for a client though
            let entity_inside_collision_shape = &*BLOCK_SHAPE;

            if entity_inside_collision_shape != &*BLOCK_SHAPE
                && !collided_with_shape_moving_from(
                    movement.from,
                    movement.to,
                    traversed_block,
                    entity_inside_collision_shape,
                    dimensions,
                )
            {
                continue;
            }

            handle_entity_inside_block(
                moved_far
                    || bounding_box_at_target.intersects_vec3(
                        traversed_block.to_vec3_floored(),
                        traversed_block.to_vec3_floored().add(Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        }),
                    ),
                world,
                traversed_block_state,
                effect,
                traversed_block,
                ground_contact,
                position,
                dimensions,
                stuck_speed_multipler,
                physics,
            );

            blocks_inside.push(traversed_block);
        }
    }

    blocks_inside
}

fn collided_with_shape_moving_from(
    from: Vec3,
    to: Vec3,
    traversed_block: BlockPos,
    entity_inside_collision_shape: &VoxelShape,
    dimensions: &EntityDimensions,
) -> bool {
    let bounding_box_from = dimensions.make_bounding_box(from);
    let delta = to - from;
    bounding_box_from.collided_along_vector(
        delta,
        &entity_inside_collision_shape
            .move_relative(traversed_block.to_vec3_floored())
            .to_aabbs(),
    )
}

// BlockBehavior.entityInside
#[allow(clippy::too_many_arguments)]
fn handle_entity_inside_block(
    precise: bool,
    world: &World,
    block: BlockState,
    effect: &ActiveEffects,
    block_pos: BlockPos,
    ground_contact: &GroundContact,
    position: &Position,
    dimensions: &EntityDimensions,
    stuck_speed_multipler: &mut StuckSpeedMultiplier,
    physics: &mut Physics,
) {
    let registry_block = BlockKind::from(block);
    #[allow(clippy::single_match)]
    match registry_block {
        BlockKind::BubbleColumn => {
            if !precise {
                return;
            }

            let block_above = world.get_block_state(block_pos.up(1)).unwrap_or_default();
            let is_block_above_empty =
                block_above.is_collision_shape_empty() && FluidState::from(block_above).is_empty();
            let drag_down = block
                .property::<properties::Drag>()
                .expect("drag property should always be present on bubble columns");
            let velocity = &mut physics.velocity;

            if is_block_above_empty {
                let new_y = if drag_down {
                    f64::max(-0.9, velocity.y - 0.03)
                } else {
                    f64::min(1.8, velocity.y + 0.1)
                };
                velocity.y = new_y;
            } else {
                let new_y = if drag_down {
                    f64::max(-0.3, velocity.y - 0.03)
                } else {
                    f64::min(0.7, velocity.y + 0.06)
                };
                velocity.y = new_y;
                physics.reset_fall_distance();
            }
        }
        BlockKind::Cobweb => {
            stuck_speed_multipler.modifier = if effect.get(MobEffect::Weaving).is_some() {
                Vec3 {
                    x: 0.5,
                    y: 0.25,
                    z: 0.5,
                }
            } else {
                Vec3 {
                    x: 0.25,
                    y: 0.05f32 as f64,
                    z: 0.25,
                }
            }
        }
        BlockKind::SweetBerryBush => {
            stuck_speed_multipler.modifier = Vec3 {
                x: 0.8f32 as f64,
                y: 0.75,
                z: 0.8f32 as f64,
            }
        }
        BlockKind::HoneyBlock => {
            let dx = (block_pos.x as f64 + 0.5 - position.x).abs();
            let dz = (block_pos.z as f64 + 0.5 - position.z).abs();
            let overlapping_dist = 0.4375f64 + (dimensions.width / 2.0f32) as f64;

            let old_velocity_y = physics.velocity.y / 0.98f32 as f64 + 0.08;
            const NEW_VELOCITY_Y: f64 = (-0.05f64 - 0.08) * 0.98f32 as f64;

            let is_sliding_down = !ground_contact.on_ground()
                && position.y <= block_pos.y as f64 + 0.9375 - 1.0e-7
                && old_velocity_y < -0.08
                && (dx + 1.0e-7 > overlapping_dist || dz + 1.0e-7 > overlapping_dist);

            if is_sliding_down {
                if old_velocity_y < -0.13 {
                    let reduction_factor_horizontal = -0.05 / old_velocity_y;
                    physics.velocity.x *= reduction_factor_horizontal;
                    physics.velocity.y = NEW_VELOCITY_Y;
                    physics.velocity.z *= reduction_factor_horizontal;
                } else {
                    physics.velocity.y = NEW_VELOCITY_Y;
                }

                physics.reset_fall_distance();
            }
        }
        _ => {}
    }
}

// can't imagine that's the only block that currently has client side effect
// when stepping on, therefore minimal args
fn handle_entity_step_on(
    block: BlockKind,
    client_movement: &ClientMovementState,
    physics: &mut Physics,
) {
    if BlockKind::SlimeBlock == block {
        let y_absolute = physics.velocity.y.abs();
        if y_absolute > 0.1 && !client_movement.trying_to_crouch {
            let scale = 0.4 + y_absolute * 0.2;
            physics.velocity.x *= scale;
            physics.velocity.z *= scale;
        }
    }
}

pub struct EntityMovement {
    pub from: Vec3,
    pub to: Vec3,
}

#[allow(clippy::too_many_arguments)]
pub fn jump_from_ground(
    physics: &mut Physics,
    ground_contact: &GroundContact,
    position: Position,
    look_direction: LookDirection,
    sprinting: Sprinting,
    world_name: &WorldName,
    worlds: &Worlds,
    active_effects: &ActiveEffects,
) {
    let world_lock = worlds
        .get(world_name)
        .expect("All entities should be in a valid world");
    let world = world_lock.read();

    let base_jump = jump_power(&world, position, ground_contact);
    let jump_power = base_jump + jump_boost_power(active_effects);
    if jump_power <= 1.0E-5 {
        return;
    }

    let old_delta_movement = physics.velocity;
    physics.velocity = Vec3 {
        x: old_delta_movement.x,
        y: f64::max(jump_power as f64, old_delta_movement.y),
        z: old_delta_movement.z,
    };
    if *sprinting {
        // sprint jumping gives some extra velocity
        let y_rot = look_direction.y_rot() * 0.017453292;
        physics.velocity += Vec3 {
            x: (-math::sin(y_rot) * 0.2) as f64,
            y: 0.,
            z: (math::cos(y_rot) * 0.2) as f64,
        };
    }

    physics.has_impulse = true;
}

pub fn update_old_position(mut query: Query<(&mut Physics, &Position)>) {
    for (mut physics, position) in &mut query {
        physics.set_old_pos(*position);
    }
}

pub fn get_block_pos_below_that_affects_movement(
    chunk_storage: &ChunkStorage,
    position: Position,
    ground_contact: &GroundContact,
) -> BlockPos {
    on_pos(0.500001f32, chunk_storage, position, ground_contact)
}

fn handle_relative_friction_and_calculate_movement(ctx: &mut MoveCtx, block_friction: f32) {
    move_relative(
        ctx.physics,
        ctx.direction,
        get_friction_influenced_speed(
            ctx.ground_contact,
            ctx.attributes,
            block_friction,
            ctx.sprinting,
        ),
        Vec3::new(
            ctx.physics.x_acceleration as f64,
            ctx.physics.y_acceleration as f64,
            ctx.physics.z_acceleration as f64,
        ),
    );

    ctx.physics.velocity = handle_on_climbable(
        ctx.physics.velocity,
        ctx.on_climbable,
        *ctx.position,
        ctx.world,
        ctx.pose,
    );

    move_colliding(ctx, ctx.physics.velocity);
}

fn handle_on_climbable(
    velocity: Vec3,
    on_climbable: OnClimbable,
    position: Position,
    world: &World,
    pose: Option<Pose>,
) -> Vec3 {
    if !*on_climbable {
        return velocity;
    }

    // minecraft does resetFallDistance here

    const CLIMBING_SPEED: f64 = 0.15_f32 as f64;

    let x = f64::clamp(velocity.x, -CLIMBING_SPEED, CLIMBING_SPEED);
    let z = f64::clamp(velocity.z, -CLIMBING_SPEED, CLIMBING_SPEED);
    let mut y = f64::max(velocity.y, -CLIMBING_SPEED);

    // sneaking on ladders/vines
    if y < 0.0
        && pose == Some(Pose::Crouching)
        && BlockKind::from(
            world
                .chunks
                .get_block_state(position.into())
                .unwrap_or_default(),
        ) != BlockKind::Scaffolding
    {
        y = 0.;
    }

    Vec3 { x, y, z }
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction *
// friction * friction)) : this.flyingSpeed; }
fn get_friction_influenced_speed(
    ground_contact: &GroundContact,
    attributes: &Attributes,
    friction: f32,
    sprinting: Sprinting,
) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if ground_contact.on_ground() {
        let speed = attributes.movement_speed.calculate() as f32;
        speed * (0.21600002f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        if *sprinting { 0.025999999f32 } else { 0.02 }
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor(world: &World, position: Position, ground_contact: &GroundContact) -> f32 {
    let block_at_pos = world.chunks.get_block_state(position.into());
    let block_below = world
        .chunks
        .get_block_state(get_block_pos_below_that_affects_movement(
            &world.chunks,
            position,
            ground_contact,
        ));

    let block_at_pos_jump_factor = if let Some(block) = block_at_pos {
        block.behavior().jump_factor
    } else {
        1.
    };
    if block_at_pos_jump_factor != 1. {
        return block_at_pos_jump_factor;
    }

    if let Some(block) = block_below {
        block.behavior().jump_factor
    } else {
        1.
    }
}

// protected float getJumpPower() {
//     return 0.42F * this.getBlockJumpFactor();
// }
// public double getJumpBoostPower() {
//     return this.hasEffect(MobEffects.JUMP) ? (double)(0.1F *
// (float)(this.getEffect(MobEffects.JUMP).getAmplifier() + 1)) : 0.0D; }
fn jump_power(world: &World, position: Position, ground_contact: &GroundContact) -> f32 {
    0.42 * block_jump_factor(world, position, ground_contact)
}

fn jump_boost_power(active_effects: &ActiveEffects) -> f32 {
    active_effects
        .get_level(MobEffect::JumpBoost)
        .map(|level| 0.1 * (level + 1) as f32)
        .unwrap_or(0.)
}

#[allow(clippy::type_complexity)]
pub fn update_falling_distance(
    mut query: Query<
        (&Position, &WorldName, &GroundContact, &mut Physics),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    worlds: Res<Worlds>,
) {
    for (position, world_name, ground_contact, mut physics) in &mut query {
        let Some(world_lock) = worlds.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        let block_pos_below =
            azalea_entity::on_pos_legacy(&world.chunks, *position, ground_contact);
        let block_state_below = world.get_block_state(block_pos_below).unwrap_or_default();

        let old_position = physics.old_position;
        check_fall_damage(
            &mut physics,
            ground_contact,
            (**position - old_position).y,
            block_state_below,
            block_pos_below,
        );
    }
}

fn check_fall_damage(
    physics: &mut Physics,
    ground_contact: &GroundContact,
    delta_y: f64,
    _block_state_below: BlockState,
    _block_pos_below: BlockPos,
) {
    if !physics.is_in_water() && delta_y < 0. {
        physics.fall_distance -= delta_y as f32 as f64;
    }

    if ground_contact.on_ground() {
        // vanilla calls block.fallOn here but it's not relevant for us

        physics.fall_distance = 0.;
    }
}
