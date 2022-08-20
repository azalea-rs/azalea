mod data;
mod dimensions;

use crate::Dimension;
use azalea_core::{BlockPos, PositionDelta, Vec3, AABB};
pub use data::*;
pub use dimensions::*;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub uuid: Uuid,
    /// The position of the entity right now.
    /// This can be changde with unsafe_move, but the correct way is with dimension.move_entity
    pos: Vec3,
    /// The position of the entity last tick.
    pub last_pos: Vec3,
    pub delta: PositionDelta,

    pub x_rot: f32,
    pub y_rot: f32,

    pub x_rot_last: f32,
    pub y_rot_last: f32,

    pub on_ground: bool,
    pub last_on_ground: bool,

    /// The width and height of the entity.
    pub dimensions: EntityDimensions,
    /// The bounding box of the entity. This is more than just width and height, unlike dimensions.
    pub bounding_box: AABB,
}
impl Entity {
    pub fn new(id: u32, uuid: Uuid, pos: Vec3) -> Self {
        // TODO: have this be based on the entity type
        let dimensions = EntityDimensions {
            width: 0.8,
            height: 1.8,
        };
        Self {
            id,
            uuid,
            pos,
            last_pos: pos,
            delta: PositionDelta::default(),

            x_rot: 0.0,
            y_rot: 0.0,

            y_rot_last: 0.0,
            x_rot_last: 0.0,

            on_ground: false,
            last_on_ground: false,

            // TODO: have this be based on the entity type
            bounding_box: dimensions.make_bounding_box(&pos),
            dimensions,
        }
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    /// Sets the position of the entity. This doesn't update the cache in
    /// azalea-world, and should only be used within azalea-world!
    pub fn unsafe_move(&mut self, new_pos: Vec3) {
        self.pos = new_pos;
        self.bounding_box = self.make_bounding_box();
    }

    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        self.y_rot = y_rot.clamp(-90.0, 90.0) % 360.0;
        self.x_rot = x_rot % 360.0;
        // TODO: minecraft also sets yRotO and xRotO to xRot and yRot ... but idk what they're used for so
    }

    fn make_bounding_box(&self) -> AABB {
        self.dimensions.make_bounding_box(&self.pos)
    }

    /// Get the position of the block below the entity, but a little lower.
    pub fn on_pos_legacy(&self, dimension: &Dimension) -> BlockPos {
        self.on_pos(0.2, dimension)
    }

    // int x = Mth.floor(this.position.x);
    // int y = Mth.floor(this.position.y - (double)var1);
    // int z = Mth.floor(this.position.z);
    // BlockPos var5 = new BlockPos(x, y, z);
    // if (this.level.getBlockState(var5).isAir()) {
    //    BlockPos var6 = var5.below();
    //    BlockState var7 = this.level.getBlockState(var6);
    //    if (var7.is(BlockTags.FENCES) || var7.is(BlockTags.WALLS) || var7.getBlock() instanceof FenceGateBlock) {
    //       return var6;
    //    }
    // }
    // return var5;
    pub fn on_pos(&self, offset: f32, dimension: &Dimension) -> BlockPos {
        let x = self.pos.x.floor() as i32;
        let y = (self.pos.y - offset as f64).floor() as i32;
        let z = self.pos.z.floor() as i32;
        let pos = BlockPos { x, y, z };
        let block_pos = pos.below();
        let block_state = dimension.get_block_state(&block_pos);

        // TODO: check if block below is a fence, wall, or fence gate
        // if block_state == Some(BlockState::Air) {
        //     let block_pos_below = block_pos.below();
        //     let block_state_below = dimension.get_block_state(&block_pos_below);
        //     if let Some(block_state_below) = block_state_below {
        //         if block_state_below.is_fence()
        //             || block_state_below.is_wall()
        //             || block_state_below.is_fence_gate()
        //         {
        //             return block_pos_below;
        //         }
        //     }
        // }

        pos
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
