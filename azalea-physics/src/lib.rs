pub mod collision;

use azalea_core::{BlockPos, Vec3};
use azalea_world::{entity::Entity, Dimension};

trait HasPhysics {
    fn travel(&self, acceleration: &Vec3, dimension: &Dimension) -> Result<(), String>;

    fn get_block_pos_below_that_affects_movement(&self) -> BlockPos;
}

// impl HasPhysics for Entity {
//     fn travel(&self, acceleration: &Vec3, dimension: &Dimension) -> Result<(), String> {
//         // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
//         //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
//         //     return;
//         // }

//         let mut gravity: f64 = 0.08;

//         let is_falling = self.delta.ya <= 0.;

//         // TODO: slow falling effect

//         // TODO: fluids

//         // TODO: elytra

//         let block_pos_below = self.get_block_pos_below_that_affects_movement();
//         let block_friction = if let Some(block_state_below) = dimension.get_block_state(&block_pos_below)
//         {
// 			let block_below = block_state_below;
//             block_below.into().friction();
//         } else {
//             panic!();
//         };

//         Ok(())
//     }
// }
