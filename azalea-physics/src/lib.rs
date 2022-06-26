mod aabb;
mod block_hit_result;

pub use aabb::AABB;
use azalea_core::{PositionDelta, PositionXYZ, Vec3};
use azalea_entity::Entity;
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

    // fn collide_bounding_box(self: )
}
