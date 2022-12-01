mod blocks;
mod discrete_voxel_shape;
mod mergers;
mod shape;
mod world_collisions;

use azalea_core::{Axis, Vec3, AABB, EPSILON};
use azalea_world::entity::{Entity, EntityData};
use azalea_world::{MoveEntityError, World};
pub use blocks::BlockWithShape;
pub use discrete_voxel_shape::*;
pub use shape::*;
use std::ops::DerefMut;
use world_collisions::CollisionGetter;

pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

pub trait HasCollision {
    fn collide(&self, movement: &Vec3, entity: &EntityData) -> Vec3;
}

pub trait MovableEntity {
    fn move_colliding(
        &mut self,
        mover_type: &MoverType,
        movement: &Vec3,
    ) -> Result<(), MoveEntityError>;
}

impl HasCollision for World {
    // private Vec3 collide(Vec3 var1) {
    //     AABB var2 = this.getBoundingBox();
    //     List var3 = this.level.getEntityCollisions(this, var2.expandTowards(var1));
    //     Vec3 var4 = var1.lengthSqr() == 0.0D ? var1 : collideBoundingBox(this, var1, var2, this.level, var3);
    //     boolean var5 = var1.x != var4.x;
    //     boolean var6 = var1.y != var4.y;
    //     boolean var7 = var1.z != var4.z;
    //     boolean var8 = this.onGround || var6 && var1.y < 0.0D;
    //     if (this.maxUpStep > 0.0F && var8 && (var5 || var7)) {
    //        Vec3 var9 = collideBoundingBox(this, new Vec3(var1.x, (double)this.maxUpStep, var1.z), var2, this.level, var3);
    //        Vec3 var10 = collideBoundingBox(this, new Vec3(0.0D, (double)this.maxUpStep, 0.0D), var2.expandTowards(var1.x, 0.0D, var1.z), this.level, var3);
    //        if (var10.y < (double)this.maxUpStep) {
    //           Vec3 var11 = collideBoundingBox(this, new Vec3(var1.x, 0.0D, var1.z), var2.move(var10), this.level, var3).add(var10);
    //           if (var11.horizontalDistanceSqr() > var9.horizontalDistanceSqr()) {
    //              var9 = var11;
    //           }
    //        }

    //        if (var9.horizontalDistanceSqr() > var4.horizontalDistanceSqr()) {
    //           return var9.add(collideBoundingBox(this, new Vec3(0.0D, -var9.y + var1.y, 0.0D), var2.move(var9), this.level, var3));
    //        }
    //     }

    //     return var4;
    // }
    fn collide(&self, movement: &Vec3, entity: &EntityData) -> Vec3 {
        let entity_bounding_box = entity.bounding_box;
        // TODO: get_entity_collisions
        // let entity_collisions = world.get_entity_collisions(self, entity_bounding_box.expand_towards(movement));
        let entity_collisions = Vec::new();
        if movement.length_sqr() == 0.0 {
            *movement
        } else {
            collide_bounding_box(
                Some(entity),
                movement,
                &entity_bounding_box,
                self,
                entity_collisions,
            )
        }

        // TODO: stepping (for stairs and stuff)

        // collided_movement
    }
}

impl<D: DerefMut<Target = World>> MovableEntity for Entity<'_, D> {
    /// Move an entity by a given delta, checking for collisions.
    fn move_colliding(
        &mut self,
        _mover_type: &MoverType,
        movement: &Vec3,
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

        let collide_result = { self.world.collide(movement, self) };

        let move_distance = collide_result.length_sqr();

        if move_distance > EPSILON {
            // TODO: fall damage

            let new_pos = {
                let entity_pos = self.pos();
                Vec3 {
                    x: entity_pos.x + collide_result.x,
                    y: entity_pos.y + collide_result.y,
                    z: entity_pos.z + collide_result.z,
                }
            };

            self.world.set_entity_pos(self.id, new_pos)?;
        }

        let x_collision = movement.x != collide_result.x;
        let z_collision = movement.z != collide_result.z;
        let horizontal_collision = x_collision || z_collision;
        let vertical_collision = movement.y != collide_result.y;
        let on_ground = vertical_collision && movement.y < 0.;
        self.on_ground = on_ground;

        // TODO: minecraft checks for a "minor" horizontal collision here

        let _block_pos_below = self.on_pos_legacy();
        // let _block_state_below = self
        //     .world
        //     .get_block_state(&block_pos_below)
        //     .expect("Couldn't get block state below");

        // self.check_fall_damage(collide_result.y, on_ground, block_state_below, block_pos_below);

        // if self.isRemoved() { return; }

        if horizontal_collision {
            let delta_movement = &self.delta;
            self.delta = Vec3 {
                x: if x_collision { 0. } else { delta_movement.x },
                y: delta_movement.y,
                z: if z_collision { 0. } else { delta_movement.z },
            }
        }

        if vertical_collision {
            // blockBelow.updateEntityAfterFallOn(this.level, this);
            // the default implementation of updateEntityAfterFallOn sets the y movement to 0
            self.delta.y = 0.;
        }

        if on_ground {
            // blockBelow.stepOn(this.level, blockPosBelow, blockStateBelow, this);
        }

        // sounds

        // this.tryCheckInsideBlocks();

        // float var25 = this.getBlockSpeedFactor();
        // this.setDeltaMovement(this.getDeltaMovement().multiply((double)var25, 1.0D, (double)var25));
        // if (this.level.getBlockStatesIfLoaded(this.getBoundingBox().deflate(1.0E-6D)).noneMatch((var0) -> {
        //    return var0.is(BlockTags.FIRE) || var0.is(Blocks.LAVA);
        // })) {
        //    if (this.remainingFireTicks <= 0) {
        //       this.setRemainingFireTicks(-this.getFireImmuneTicks());
        //    }

        //    if (this.wasOnFire && (this.isInPowderSnow || this.isInWaterRainOrBubble())) {
        //       this.playEntityOnFireExtinguishedSound();
        //    }
        // }

        // if (this.isOnFire() && (this.isInPowderSnow || this.isInWaterRainOrBubble())) {
        //    this.setRemainingFireTicks(-this.getFireImmuneTicks());
        // }

        Ok(())
    }
}

fn collide_bounding_box(
    entity: Option<&EntityData>,
    movement: &Vec3,
    entity_bounding_box: &AABB,
    world: &World,
    entity_collisions: Vec<VoxelShape>,
) -> Vec3 {
    let mut collision_boxes: Vec<VoxelShape> = Vec::with_capacity(entity_collisions.len() + 1);

    if !entity_collisions.is_empty() {
        collision_boxes.extend(entity_collisions);
    }

    // TODO: world border

    let block_collisions =
        world.get_block_collisions(entity, entity_bounding_box.expand_towards(movement));
    let block_collisions = block_collisions.collect::<Vec<_>>();
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
            entity_box = entity_box.move_relative(0., y_movement, 0.);
        }
    }

    // whether the player is moving more in the z axis than x
    // this is done to fix a movement bug, minecraft does this too
    let more_z_movement = x_movement.abs() < z_movement.abs();

    if more_z_movement && z_movement != 0. {
        z_movement = Shapes::collide(&Axis::Z, &entity_box, collision_boxes, z_movement);
        if z_movement != 0. {
            entity_box = entity_box.move_relative(0., 0., z_movement);
        }
    }

    if x_movement != 0. {
        x_movement = Shapes::collide(&Axis::X, &entity_box, collision_boxes, x_movement);
        if x_movement != 0. {
            entity_box = entity_box.move_relative(x_movement, 0., 0.);
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
