mod aabb;
mod block_hit_result;

use crate::Entity;
pub use aabb::AABB;
use azalea_core::PositionDelta;
pub use block_hit_result::BlockHitResult;

pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

impl Entity {
    pub fn move_entity(&mut self, mover_type: &MoverType, movement: &PositionDelta) {
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
}
