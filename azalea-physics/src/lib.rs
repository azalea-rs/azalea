mod aabb;
mod block_hit_result;
mod dimension_collisions;

pub use aabb::AABB;
use azalea_core::{PositionDelta, PositionXYZ, Vec3};
use azalea_entity::Entity;
use azalea_world::Dimension;
pub use block_hit_result::BlockHitResult;

pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

trait HasPhysics {
    fn move_entity(&mut self, mover_type: &MoverType, movement: &PositionDelta);
    fn collide_bounding_box(
        entity: Option<&Self>,
        movement: &Vec3,
        entity_bounding_box: &AABB,
        dimension: &Dimension,
        // entity_collisions: Vec<VoxelShape>
    ) -> Vec3;
}

impl HasPhysics for Entity {
    /// Move an entity by a given delta, checking for collisions.
    fn move_entity(&mut self, mover_type: &MoverType, movement: &PositionDelta) {
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

        // TODO
    }

    // fn collide(movement: &Vec3, dimension: &Dimension) -> Vec3 {
    //     if movement.length_sqr() == 0.0 {
    //         *movement
    //     } else {
    //         // Self::collide_bounding_box(
    //         //     Some(self),
    //         //     movement,
    //         //     entityBoundingBox,
    //         //     this.level,
    //         //     entityCollisions,
    //         // )
    //     }
    // }

    fn collide_bounding_box(
        entity: Option<&Self>,
        movement: &Vec3,
        entity_bounding_box: &AABB,
        dimension: &Dimension,
        // entity_collisions: Vec<VoxelShape>
    ) -> Vec3 {
        let mut collision_boxes = Vec::with_capacity(1); // entity_collisions.len() + 1

        // if !entity_collisions.is_empty() { add to collision_boxes }

        // TODO: world border

        let block_collisions =
            dimension.get_block_collisions(entity, entity_bounding_box.expand_towards(movement));
        collision_boxes.extend(block_collisions);
        Self::collide_with_shapes(movement, &entity_bounding_box, &collision_boxes)
    }
}
