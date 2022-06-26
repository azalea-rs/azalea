mod data;
mod physics;

use azalea_core::{PositionDelta, Vec3};
pub use data::*;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub uuid: Uuid,
    /// The position of the entity right now.
    pos: Vec3,
    /// The position of the entity last tick.
    pub old_pos: Vec3,
    pub delta: PositionDelta,

    pub x_rot: f32,
    pub y_rot: f32,
}

impl Entity {
    pub fn new(id: u32, uuid: Uuid, pos: Vec3) -> Self {
        Self {
            id,
            uuid,
            pos,
            old_pos: pos,
            delta: PositionDelta::default(),
            x_rot: 0.0,
            y_rot: 0.0,
        }
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    /// Sets the position of the entity. This doesn't update the cache in
    /// azalea-world, and should only be used within azalea-world!
    pub fn unsafe_move(&mut self, new_pos: Vec3) {
        self.pos = new_pos;
    }

    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        self.y_rot = y_rot.clamp(-90.0, 90.0) % 360.0;
        self.x_rot = x_rot % 360.0;
        // TODO: minecraft also sets yRotO and xRotO to xRot and yRot ... but idk what they're used for so
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
