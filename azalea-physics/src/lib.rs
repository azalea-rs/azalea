#![feature(trait_alias)]

pub mod collision;

use azalea_block::{Block, BlockState};
use azalea_core::{BlockPos, Vec3};
use azalea_world::{
    entity::{self},
    World,
};
use collision::{move_colliding, MoverType};

/// Move the entity with the given acceleration while handling friction,
/// gravity, collisions, and some other stuff.
fn travel(
    world: &World,
    physics: &mut entity::Physics,
    position: &mut entity::Position,
    attributes: &entity::Attributes,
    acceleration: &Vec3,
) {
    // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
    //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
    //     return;
    // }

    let gravity: f64 = 0.08;

    // TODO: slow falling effect
    // let is_falling = self.delta.y <= 0.;

    // TODO: fluids

    // TODO: elytra

    let block_pos_below = get_block_pos_below_that_affects_movement(position);

    let block_state_below = world
        .chunks
        .get_block_state(&block_pos_below)
        .unwrap_or(BlockState::Air);
    let block_below: Box<dyn Block> = block_state_below.into();
    let block_friction = block_below.behavior().friction;

    let inertia = if physics.on_ground {
        block_friction * 0.91
    } else {
        0.91
    };

    // this applies the current delta
    let mut movement = handle_relative_friction_and_calculate_movement(
        acceleration,
        block_friction,
        world,
        physics,
        position,
        attributes,
    );

    movement.y -= gravity;

    // if (this.shouldDiscardFriction()) {
    //     this.setDeltaMovement(movement.x, yMovement, movement.z);
    // } else {
    //     this.setDeltaMovement(movement.x * (double)inertia, yMovement *
    // 0.9800000190734863D, movement.z * (double)inertia); }

    // if should_discard_friction(self) {
    if false {
        physics.delta = movement;
    } else {
        physics.delta = Vec3 {
            x: movement.x * inertia as f64,
            y: movement.y * 0.98f64,
            z: movement.z * inertia as f64,
        };
    }
}

/// applies air resistance, calls self.travel(), and some other random
/// stuff.
pub fn ai_step(
    world: &World,
    physics: &mut entity::Physics,
    position: &mut entity::Position,
    sprinting: &entity::metadata::Sprinting,
    attributes: &entity::Attributes,
) {
    // vanilla does movement interpolation here, doesn't really matter much for a
    // bot though

    if physics.delta.x.abs() < 0.003 {
        physics.delta.x = 0.;
    }
    if physics.delta.y.abs() < 0.003 {
        physics.delta.y = 0.;
    }
    if physics.delta.z.abs() < 0.003 {
        physics.delta.z = 0.;
    }

    if physics.jumping {
        // TODO: jumping in liquids and jump delay

        if physics.on_ground {
            jump_from_ground(world, physics, position, sprinting);
        }
    }

    physics.xxa *= 0.98;
    physics.zza *= 0.98;

    travel(
        world,
        physics,
        position,
        attributes,
        &Vec3 {
            x: physics.xxa as f64,
            y: physics.yya as f64,
            z: physics.zza as f64,
        },
    );
    // freezing
    // pushEntities
    // drowning damage
}

fn jump_from_ground(
    world: &World,
    physics: &mut entity::Physics,
    position: &entity::Position,
    sprinting: &entity::metadata::Sprinting,
) {
    let jump_power: f64 = jump_power(world, position) as f64 + jump_boost_power();
    let old_delta_movement = physics.delta;
    physics.delta = Vec3 {
        x: old_delta_movement.x,
        y: jump_power,
        z: old_delta_movement.z,
    };
    if **sprinting {
        let y_rot = physics.y_rot * 0.017453292;
        physics.delta += Vec3 {
            x: (-f32::sin(y_rot) * 0.2) as f64,
            y: 0.,
            z: (f32::cos(y_rot) * 0.2) as f64,
        };
    }

    physics.has_impulse = true;
}

fn get_block_pos_below_that_affects_movement(position: &entity::Position) -> BlockPos {
    BlockPos::new(
        position.x.floor() as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (position.y - 0.5f64).floor() as i32,
        position.z.floor() as i32,
    )
}

fn handle_relative_friction_and_calculate_movement(
    acceleration: &Vec3,
    block_friction: f32,
    world: &World,
    physics: &mut entity::Physics,
    position: &mut entity::Position,
    attributes: &entity::Attributes,
) -> Vec3 {
    entity::move_relative(
        physics,
        get_friction_influenced_speed(physics, attributes, block_friction),
        acceleration,
    );
    // entity.delta = entity.handle_on_climbable(entity.delta);
    move_colliding(
        &MoverType::Own,
        &physics.delta.clone(),
        world,
        position,
        physics,
    )
    .expect("Entity should exist.");
    // let delta_movement = entity.delta;
    // ladders
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable()
    // || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) &&
    // PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {      var3 = new
    // Vec3(var3.x, 0.2D, var3.z);   }
    // TODO: powdered snow

    physics.delta
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction *
// friction * friction)) : this.flyingSpeed; }
fn get_friction_influenced_speed(
    physics: &entity::Physics,
    attributes: &entity::Attributes,
    friction: f32,
) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if physics.on_ground {
        let speed: f32 = attributes.speed.calculate() as f32;
        speed * (0.216f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        0.02
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor(world: &World, position: &entity::Position) -> f32 {
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
fn jump_power(world: &World, position: &entity::Position) -> f32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_core::ChunkPos;
    use azalea_world::{
        entity::{metadata, EntityMetadata},
        Chunk, PartialWorld,
    };
    use uuid::Uuid;

    #[test]
    fn test_gravity() {
        let mut world = PartialWorld::default();

        world.add_entity(
            0,
            EntityData::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.,
                    y: 70.,
                    z: 0.,
                },
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let mut entity = world.entity_mut(0).unwrap();
        // y should start at 70
        assert_eq!(entity.pos().y, 70.);
        entity.ai_step();
        // delta is applied before gravity, so the first tick only sets the delta
        assert_eq!(entity.pos().y, 70.);
        assert!(entity.delta.y < 0.);
        entity.ai_step();
        // the second tick applies the delta to the position, so now it should go down
        assert!(
            entity.pos().y < 70.,
            "Entity y ({}) didn't go down after physics steps",
            entity.pos().y
        );
    }
    #[test]
    fn test_collision() {
        let mut world = PartialWorld::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 70.,
                    z: 0.5,
                },
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let block_state = world.set_block_state(&BlockPos { x: 0, y: 69, z: 0 }, BlockState::Stone);
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        let mut entity = world.entity_mut(0).unwrap();
        entity.ai_step();
        // delta will change, but it won't move until next tick
        assert_eq!(entity.pos().y, 70.);
        assert!(entity.delta.y < 0.);
        entity.ai_step();
        // the second tick applies the delta to the position, but it also does collision
        assert_eq!(entity.pos().y, 70.);
    }

    #[test]
    fn test_slab_collision() {
        let mut world = PartialWorld::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 71.,
                    z: 0.5,
                },
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let block_state = world.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            BlockState::StoneSlab_BottomFalse,
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        let mut entity = world.entity_mut(0).unwrap();
        // do a few steps so we fall on the slab
        for _ in 0..20 {
            entity.ai_step();
        }
        assert_eq!(entity.pos().y, 69.5);
    }

    #[test]
    fn test_top_slab_collision() {
        let mut world = PartialWorld::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 71.,
                    z: 0.5,
                },
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let block_state = world.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            BlockState::StoneSlab_TopFalse,
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        let mut entity = world.entity_mut(0).unwrap();
        // do a few steps so we fall on the slab
        for _ in 0..20 {
            entity.ai_step();
        }
        assert_eq!(entity.pos().y, 70.);
    }

    #[test]
    fn test_weird_wall_collision() {
        let mut world = PartialWorld::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::nil(),
                Vec3 {
                    x: 0.5,
                    y: 73.,
                    z: 0.5,
                },
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let block_state = world.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            BlockState::CobblestoneWall_LowLowLowFalseFalseLow,
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        let mut entity = world.entity_mut(0).unwrap();
        // do a few steps so we fall on the slab
        for _ in 0..20 {
            entity.ai_step();
        }
        assert_eq!(entity.pos().y, 70.5);
    }
}
