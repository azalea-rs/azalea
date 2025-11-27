#![doc = include_str!("../README.md")]
#![feature(trait_alias)]

pub mod clip;
pub mod collision;
pub mod fluids;
pub mod local_player;
pub mod travel;

use std::collections::HashSet;

use azalea_block::{BlockState, BlockTrait, fluid_state::FluidState, properties};
use azalea_core::{
    math,
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    ActiveEffects, Attributes, EntityKindComponent, HasClientLoaded, Jumping, LocalEntity,
    LookDirection, OnClimbable, Physics, Pose, Position, dimensions::EntityDimensions,
    metadata::Sprinting, move_relative,
};
use azalea_registry::{Block, EntityKind, MobEffect};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use clip::box_traverse_blocks;
use collision::{BLOCK_SHAPE, BlockWithShape, VoxelShape, move_colliding};

use crate::collision::{MoveCtx, entity_collisions::update_last_bounding_box};

/// A Bevy [`SystemSet`] for running physics that makes entities do things.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PhysicsSystems;

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
                travel::travel,
                apply_effects_from_blocks,
            )
                .chain()
                .in_set(PhysicsSystems)
                .after(azalea_entity::update_in_loaded_chunk),
        )
        // we want this to happen after packets are handled but before physics
        .add_systems(
            Update,
            update_last_bounding_box.after(azalea_entity::update_bounding_box),
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
            &Position,
            &LookDirection,
            &Sprinting,
            &ActiveEffects,
            &InstanceName,
            &EntityKindComponent,
        ),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (
        mut physics,
        jumping,
        position,
        look_direction,
        sprinting,
        active_effects,
        instance_name,
        entity_kind,
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

            if !in_water || physics.on_ground() && fluid_height <= fluid_jump_threshold {
                if !physics.is_in_lava()
                    || physics.on_ground() && fluid_height <= fluid_jump_threshold
                {
                    if (physics.on_ground() || in_water && fluid_height <= fluid_jump_threshold)
                        && physics.no_jump_delay == 0
                    {
                        jump_from_ground(
                            &mut physics,
                            *position,
                            *look_direction,
                            *sprinting,
                            instance_name,
                            &instance_container,
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
    physics.velocity.y += 0.04;
}

// in minecraft, this is done as part of aiStep immediately after travel
#[allow(clippy::type_complexity)]
pub fn apply_effects_from_blocks(
    mut query: Query<
        (&mut Physics, &Position, &EntityDimensions, &InstanceName),
        (With<LocalEntity>, With<HasClientLoaded>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, position, dimensions, world_name) in &mut query {
        let Some(world_lock) = instance_container.get(world_name) else {
            continue;
        };
        let world = world_lock.read();

        // if !is_affected_by_blocks {
        //     continue
        // }

        // if (this.onGround()) {
        //     BlockPos var3 = this.getOnPosLegacy();
        //     BlockState var4 = this.level().getBlockState(var3);
        //     var4.getBlock().stepOn(this.level(), var3, var4, this);
        //  }

        // minecraft adds more entries to the list when the code is running on the
        // server
        let movement_this_tick = [EntityMovement {
            from: physics.old_position,
            to: **position,
        }];

        check_inside_blocks(&mut physics, dimensions, &world, &movement_this_tick);
    }
}

fn check_inside_blocks(
    physics: &mut Physics,
    dimensions: &EntityDimensions,
    world: &Instance,
    movements: &[EntityMovement],
) -> Vec<BlockState> {
    let mut blocks_inside = Vec::new();
    let mut visited_blocks = HashSet::<BlockState>::new();

    for movement in movements {
        let bounding_box_at_target = dimensions
            .make_bounding_box(movement.to)
            .deflate_all(1.0E-5);

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
            if !visited_blocks.insert(traversed_block_state) {
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

            handle_entity_inside_block(world, traversed_block_state, traversed_block, physics);

            blocks_inside.push(traversed_block_state);
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
fn handle_entity_inside_block(
    world: &Instance,
    block: BlockState,
    block_pos: BlockPos,
    physics: &mut Physics,
) {
    let registry_block = azalea_registry::Block::from(block);
    #[allow(clippy::single_match)]
    match registry_block {
        azalea_registry::Block::BubbleColumn => {
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
        _ => {}
    }
}

pub struct EntityMovement {
    pub from: Vec3,
    pub to: Vec3,
}

pub fn jump_from_ground(
    physics: &mut Physics,
    position: Position,
    look_direction: LookDirection,
    sprinting: Sprinting,
    instance_name: &InstanceName,
    instance_container: &InstanceContainer,
    active_effects: &ActiveEffects,
) {
    let world_lock = instance_container
        .get(instance_name)
        .expect("All entities should be in a valid world");
    let world = world_lock.read();

    let base_jump = jump_power(&world, position);
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

pub fn get_block_pos_below_that_affects_movement(position: Position) -> BlockPos {
    BlockPos::new(
        position.x.floor() as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (position.y - 0.5f64).floor() as i32,
        position.z.floor() as i32,
    )
}

fn handle_relative_friction_and_calculate_movement(ctx: &mut MoveCtx, block_friction: f32) -> Vec3 {
    move_relative(
        ctx.physics,
        ctx.direction,
        get_friction_influenced_speed(ctx.physics, ctx.attributes, block_friction, ctx.sprinting),
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
    // let delta_movement = entity.delta;
    // ladders
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable()
    // || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) &&
    // PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {      var3 = new
    // Vec3(var3.x, 0.2D, var3.z);   }

    if ctx.physics.horizontal_collision || *ctx.jumping {
        let block_at_feet: Block = ctx
            .world
            .chunks
            .get_block_state(BlockPos::from(*ctx.position))
            .unwrap_or_default()
            .into();

        if *ctx.on_climbable || block_at_feet == Block::PowderSnow {
            ctx.physics.velocity.y = 0.2;
        }
    }

    ctx.physics.velocity
}

fn handle_on_climbable(
    velocity: Vec3,
    on_climbable: OnClimbable,
    position: Position,
    world: &Instance,
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
        && azalea_registry::Block::from(
            world
                .chunks
                .get_block_state(position.into())
                .unwrap_or_default(),
        ) != azalea_registry::Block::Scaffolding
    {
        y = 0.;
    }

    Vec3 { x, y, z }
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction *
// friction * friction)) : this.flyingSpeed; }
fn get_friction_influenced_speed(
    physics: &Physics,
    attributes: &Attributes,
    friction: f32,
    sprinting: Sprinting,
) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if physics.on_ground() {
        let speed = attributes.movement_speed.calculate() as f32;
        speed * (0.21600002f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        if *sprinting { 0.025999999f32 } else { 0.02 }
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor(world: &Instance, position: Position) -> f32 {
    let block_at_pos = world.chunks.get_block_state(position.into());
    let block_below = world
        .chunks
        .get_block_state(get_block_pos_below_that_affects_movement(position));

    let block_at_pos_jump_factor = if let Some(block) = block_at_pos {
        Box::<dyn BlockTrait>::from(block).behavior().jump_factor
    } else {
        1.
    };
    if block_at_pos_jump_factor != 1. {
        return block_at_pos_jump_factor;
    }

    if let Some(block) = block_below {
        Box::<dyn BlockTrait>::from(block).behavior().jump_factor
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
fn jump_power(world: &Instance, position: Position) -> f32 {
    0.42 * block_jump_factor(world, position)
}

fn jump_boost_power(active_effects: &ActiveEffects) -> f32 {
    active_effects
        .get_level(MobEffect::JumpBoost)
        .map(|level| 0.1 * (level + 1) as f32)
        .unwrap_or(0.)
}
