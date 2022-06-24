mod data;

use azalea_core::EntityPos;
#[cfg(feature = "protocol")]
use azalea_protocol::packets::game::{
    clientbound_add_entity_packet::ClientboundAddEntityPacket,
    clientbound_add_player_packet::ClientboundAddPlayerPacket,
};
pub use data::*;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub uuid: Uuid,
    pub pos: EntityPos,
}

impl Entity {
    pub fn new(id: u32, uuid: Uuid, pos: EntityPos) -> Self {
        Self { id, uuid, pos }
    }

    pub fn pos(&self) -> &EntityPos {
        &self.pos
    }

    /// Sets the position of the entity. This doesn't update the cache in
    /// azalea-world, and should only be used within azalea-world!
    pub fn unsafe_move(&mut self, new_pos: EntityPos) {
        self.pos = new_pos;
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
