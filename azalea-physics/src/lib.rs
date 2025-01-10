#![doc = include_str!("../README.md")]
#![feature(trait_alias)]

pub mod clip;
pub mod collision;
pub mod fluids;
pub mod travel;

use std::collections::HashSet;

use azalea_block::{fluid_state::FluidState, properties, Block, BlockState};
use azalea_core::{
    math,
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{
    metadata::Sprinting, move_relative, Attributes, InLoadedChunk, Jumping, LocalEntity,
    LookDirection, OnClimbable, Physics, Pose, Position,
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_app::{App, Plugin};
use bevy_ecs::{
    query::With,
    schedule::{IntoSystemConfigs, SystemSet},
    system::{Query, Res},
    world::Mut,
};
use clip::box_traverse_blocks;
use collision::{move_colliding, BlockWithShape, MoverType, VoxelShape, BLOCK_SHAPE};

/// A Bevy [`SystemSet`] for running physics that makes entities do things.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PhysicsSet;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            (
                fluids::update_in_water_state_and_do_fluid_pushing
                    .before(azalea_entity::update_fluid_on_eyes),
                update_old_position,
                fluids::update_swimming.after(azalea_entity::update_fluid_on_eyes),
                ai_step,
                travel::travel,
                apply_effects_from_blocks,
            )
                .chain()
                .in_set(PhysicsSet)
                .after(azalea_entity::update_in_loaded_chunk),
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
            &InstanceName,
        ),
        (With<LocalEntity>, With<InLoadedChunk>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, jumping, position, look_direction, sprinting, instance_name) in &mut query {
        // vanilla does movement interpolation here, doesn't really matter much for a
        // bot though

        if physics.no_jump_delay > 0 {
            physics.no_jump_delay -= 1;
        }

        if physics.velocity.x.abs() < 0.003 {
            physics.velocity.x = 0.;
        }
        if physics.velocity.y.abs() < 0.003 {
            physics.velocity.y = 0.;
        }
        if physics.velocity.z.abs() < 0.003 {
            physics.velocity.z = 0.;
        }

        if let Some(jumping) = jumping {
            if **jumping {
                // TODO: jumping in liquids and jump delay

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
                        if physics.on_ground()
                            || in_water
                                && fluid_height <= fluid_jump_threshold
                                && physics.no_jump_delay == 0
                        {
                            jump_from_ground(
                                &mut physics,
                                position,
                                look_direction,
                                sprinting,
                                instance_name,
                                &instance_container,
                            );
                            physics.no_jump_delay = 10;
                        }
                    } else {
                        jump_in_liquid(&mut physics);
                    }
                } else {
                    jump_in_liquid(&mut physics);
                }
            }
        } else {
            physics.no_jump_delay = 0;
        }

        physics.x_acceleration *= 0.98;
        physics.z_acceleration *= 0.98;

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
        (&mut Physics, &Position, &InstanceName),
        (With<LocalEntity>, With<InLoadedChunk>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, position, world_name) in &mut query {
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

        check_inside_blocks(&mut physics, &world, &movement_this_tick);
    }
}

fn check_inside_blocks(
    physics: &mut Physics,
    world: &Instance,
    movements: &[EntityMovement],
) -> Vec<BlockState> {
    let mut blocks_inside = Vec::new();
    let mut visited_blocks = HashSet::<BlockState>::new();

    for movement in movements {
        let bounding_box_at_target = physics
            .dimensions
            .make_bounding_box(&movement.to)
            .deflate_all(1.0E-5);

        for traversed_block in
            box_traverse_blocks(&movement.from, &movement.to, &bounding_box_at_target)
        {
            // if (!this.isAlive()) {
            //     return;
            // }

            let traversed_block_state = world.get_block_state(&traversed_block).unwrap_or_default();
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
                    &movement.from,
                    &movement.to,
                    traversed_block,
                    entity_inside_collision_shape,
                    physics,
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
    from: &Vec3,
    to: &Vec3,
    traversed_block: BlockPos,
    entity_inside_collision_shape: &VoxelShape,
    physics: &Physics,
) -> bool {
    let bounding_box_from = physics.dimensions.make_bounding_box(from);
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
            let block_above = world.get_block_state(&block_pos.up(1)).unwrap_or_default();
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
    position: &Position,
    look_direction: &LookDirection,
    sprinting: &Sprinting,
    instance_name: &InstanceName,
    instance_container: &InstanceContainer,
) {
    let world_lock = instance_container
        .get(instance_name)
        .expect("All entities should be in a valid world");
    let world = world_lock.read();

    let jump_power: f64 = jump_power(&world, position) as f64 + jump_boost_power();
    let old_delta_movement = physics.velocity;
    physics.velocity = Vec3 {
        x: old_delta_movement.x,
        y: jump_power,
        z: old_delta_movement.z,
    };
    if **sprinting {
        // sprint jumping gives some extra velocity
        let y_rot = look_direction.y_rot * 0.017453292;
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
        physics.set_old_pos(position);
    }
}

fn get_block_pos_below_that_affects_movement(position: &Position) -> BlockPos {
    BlockPos::new(
        position.x.floor() as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (position.y - 0.5f64).floor() as i32,
        position.z.floor() as i32,
    )
}

/// Options for [`handle_relative_friction_and_calculate_movement`]
struct HandleRelativeFrictionAndCalculateMovementOpts<'a> {
    block_friction: f32,
    world: &'a Instance,
    physics: &'a mut Physics,
    direction: &'a LookDirection,
    position: Mut<'a, Position>,
    attributes: &'a Attributes,
    is_sprinting: bool,
    on_climbable: &'a OnClimbable,
    pose: Option<&'a Pose>,
    jumping: &'a Jumping,
}
fn handle_relative_friction_and_calculate_movement(
    HandleRelativeFrictionAndCalculateMovementOpts {
        block_friction,
        world,
        physics,
        direction,
        mut position,
        attributes,
        is_sprinting,
        on_climbable,
        pose,
        jumping,
    }: HandleRelativeFrictionAndCalculateMovementOpts<'_>,
) -> Vec3 {
    move_relative(
        physics,
        direction,
        get_friction_influenced_speed(physics, attributes, block_friction, is_sprinting),
        &Vec3 {
            x: physics.x_acceleration as f64,
            y: physics.y_acceleration as f64,
            z: physics.z_acceleration as f64,
        },
    );

    physics.velocity = handle_on_climbable(physics.velocity, on_climbable, &position, world, pose);

    move_colliding(
        MoverType::Own,
        &physics.velocity.clone(),
        world,
        &mut position,
        physics,
    )
    .expect("Entity should exist");
    // let delta_movement = entity.delta;
    // ladders
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable()
    // || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) &&
    // PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {      var3 = new
    // Vec3(var3.x, 0.2D, var3.z);   }

    if physics.horizontal_collision || **jumping {
        let block_at_feet: azalea_registry::Block = world
            .chunks
            .get_block_state(&(*position).into())
            .unwrap_or_default()
            .into();

        // TODO: powdered snow
        if **on_climbable || block_at_feet == azalea_registry::Block::PowderSnow {
            physics.velocity.y = 0.2;
        }
    }

    physics.velocity
}

fn handle_on_climbable(
    velocity: Vec3,
    on_climbable: &OnClimbable,
    position: &Position,
    world: &Instance,
    pose: Option<&Pose>,
) -> Vec3 {
    if !**on_climbable {
        return velocity;
    }

    // minecraft does resetFallDistance here

    const CLIMBING_SPEED: f64 = 0.15_f32 as f64;

    let x = f64::clamp(velocity.x, -CLIMBING_SPEED, CLIMBING_SPEED);
    let z = f64::clamp(velocity.z, -CLIMBING_SPEED, CLIMBING_SPEED);
    let mut y = f64::max(velocity.y, -CLIMBING_SPEED);

    // sneaking on ladders/vines
    if y < 0.0
        && pose.copied() == Some(Pose::Sneaking)
        && azalea_registry::Block::from(
            world
                .chunks
                .get_block_state(&position.into())
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
    is_sprinting: bool,
) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if physics.on_ground() {
        let speed: f32 = attributes.speed.calculate() as f32;
        speed * (0.216f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        if is_sprinting {
            0.025999999f32
        } else {
            0.02
        }
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor(world: &Instance, position: &Position) -> f32 {
    let block_at_pos = world.chunks.get_block_state(&position.into());
    let block_below = world
        .chunks
        .get_block_state(&get_block_pos_below_that_affects_movement(position));

    let block_at_pos_jump_factor = if let Some(block) = block_at_pos {
        Box::<dyn Block>::from(block).behavior().jump_factor
    } else {
        1.
    };
    if block_at_pos_jump_factor != 1. {
        return block_at_pos_jump_factor;
    }

    if let Some(block) = block_below {
        Box::<dyn Block>::from(block).behavior().jump_factor
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
fn jump_power(world: &Instance, position: &Position) -> f32 {
    0.42 * block_jump_factor(world, position)
}

fn jump_boost_power() -> f64 {
    // TODO: potion effects
    // if let Some(effects) = entity.effects() {
    //     if let Some(jump_effect) = effects.get(&Effect::Jump) {
    //         0.1 * (jump_effect.amplifier + 1) as f32
    //     } else {
    //         0.
    //     }
    // } else {
    //     0.
    // }
    0.
}
