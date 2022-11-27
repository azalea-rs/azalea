#![feature(trait_alias)]

pub mod collision;

use std::ops::DerefMut;

use azalea_block::{Block, BlockState};
use azalea_core::{BlockPos, Vec3};
use azalea_world::{
    entity::{Entity, EntityData},
    World,
};
use collision::{MovableEntity, MoverType};

pub trait HasPhysics {
    fn travel(&mut self, acceleration: &Vec3);
    fn ai_step(&mut self);

    fn jump_from_ground(&mut self);
}

impl<D: DerefMut<Target = World>> HasPhysics for Entity<'_, D> {
    /// Move the entity with the given acceleration while handling friction,
    /// gravity, collisions, and some other stuff.
    fn travel(&mut self, acceleration: &Vec3) {
        // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
        //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
        //     return;
        // }

        let gravity: f64 = 0.08;

        // TODO: slow falling effect
        // let is_falling = self.delta.y <= 0.;

        // TODO: fluids

        // TODO: elytra

        let block_pos_below = get_block_pos_below_that_affects_movement(self);

        let block_state_below = self
            .world
            .get_block_state(&block_pos_below)
            .unwrap_or(BlockState::Air);
        let block_below: Box<dyn Block> = block_state_below.into();
        let block_friction = block_below.behavior().friction;

        let inertia = if self.on_ground {
            block_friction * 0.91
        } else {
            0.91
        };

        // this applies the current delta
        let mut movement =
            handle_relative_friction_and_calculate_movement(self, acceleration, block_friction);

        movement.y -= gravity;

        // if (this.shouldDiscardFriction()) {
        //     this.setDeltaMovement(movement.x, yMovement, movement.z);
        // } else {
        //     this.setDeltaMovement(movement.x * (double)inertia, yMovement * 0.9800000190734863D, movement.z * (double)inertia);
        // }

        // if should_discard_friction(self) {
        if false {
            self.delta = movement;
        } else {
            self.delta = Vec3 {
                x: movement.x * inertia as f64,
                y: movement.y * 0.98f64,
                z: movement.z * inertia as f64,
            };
        }
    }

    /// applies air resistance, calls self.travel(), and some other random
    /// stuff.
    fn ai_step(&mut self) {
        // vanilla does movement interpolation here, doesn't really matter much for a bot though

        if self.delta.x.abs() < 0.003 {
            self.delta.x = 0.;
        }
        if self.delta.y.abs() < 0.003 {
            self.delta.y = 0.;
        }
        if self.delta.z.abs() < 0.003 {
            self.delta.z = 0.;
        }

        if self.jumping {
            // TODO: jumping in liquids and jump delay

            if self.on_ground {
                self.jump_from_ground();
            }
        }

        self.xxa *= 0.98;
        self.zza *= 0.98;

        self.travel(&Vec3 {
            x: self.xxa as f64,
            y: self.yya as f64,
            z: self.zza as f64,
        });
        // freezing
        // pushEntities
        // drowning damage
    }

    fn jump_from_ground(&mut self) {
        let jump_power: f64 = jump_power(self) as f64 + jump_boost_power(self);
        let old_delta_movement = self.delta;
        self.delta = Vec3 {
            x: old_delta_movement.x,
            y: jump_power,
            z: old_delta_movement.z,
        };
        if self.metadata.sprinting {
            let y_rot = self.y_rot * 0.017453292;
            self.delta += Vec3 {
                x: (-f32::sin(y_rot) * 0.2) as f64,
                y: 0.,
                z: (f32::cos(y_rot) * 0.2) as f64,
            };
        }

        self.has_impulse = true;
    }
}

fn get_block_pos_below_that_affects_movement(entity: &EntityData) -> BlockPos {
    BlockPos::new(
        entity.pos().x.floor() as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (entity.pos().y - 0.5f64).floor() as i32,
        entity.pos().z.floor() as i32,
    )
}

fn handle_relative_friction_and_calculate_movement<D: DerefMut<Target = World>>(
    entity: &mut Entity<D>,
    acceleration: &Vec3,
    block_friction: f32,
) -> Vec3 {
    entity.move_relative(
        get_friction_influenced_speed(&*entity, block_friction),
        acceleration,
    );
    // entity.delta = entity.handle_on_climbable(entity.delta);
    entity
        .move_colliding(&MoverType::Own, &entity.delta.clone())
        .expect("Entity should exist.");
    // let delta_movement = entity.delta;
    // ladders
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable() || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) && PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {
    //      var3 = new Vec3(var3.x, 0.2D, var3.z);
    //   }
    // TODO: powdered snow

    entity.delta
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction * friction * friction)) : this.flyingSpeed;
// }
fn get_friction_influenced_speed(entity: &EntityData, friction: f32) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if entity.on_ground {
        let speed: f32 = entity.attributes.speed.calculate() as f32;
        speed * (0.216f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        0.02
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor<D: DerefMut<Target = World>>(entity: &Entity<D>) -> f32 {
    let block_at_pos = entity.world.get_block_state(&entity.pos().into());
    let block_below = entity
        .world
        .get_block_state(&get_block_pos_below_that_affects_movement(entity));

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
//     return this.hasEffect(MobEffects.JUMP) ? (double)(0.1F * (float)(this.getEffect(MobEffects.JUMP).getAmplifier() + 1)) : 0.0D;
// }
fn jump_power<D: DerefMut<Target = World>>(entity: &Entity<D>) -> f32 {
    0.42 * block_jump_factor(entity)
}

fn jump_boost_power<D: DerefMut<Target = World>>(_entity: &Entity<D>) -> f64 {
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
        Chunk, World,
    };
    use uuid::Uuid;

    #[test]
    fn test_gravity() {
        let mut world = World::default();

        world.add_entity(
            0,
            EntityData::new(
                Uuid::from_u128(0),
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
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::from_u128(0),
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
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::from_u128(0),
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
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::from_u128(0),
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
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.add_entity(
            0,
            EntityData::new(
                Uuid::from_u128(0),
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
