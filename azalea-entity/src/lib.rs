use azalea_core::EntityPos;
#[cfg(feature = "protocol")]
use azalea_protocol::packets::game::clientbound_add_entity_packet::ClientboundAddEntityPacket;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Entity {
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub uuid: Uuid,
    pos: EntityPos,
}

impl Entity {
    pub fn pos(&self) -> &EntityPos {
        &self.pos
    }

    pub fn set_pos(&mut self, pos: EntityPos) {
        // TODO: check if it moved to another chunk
        self.pos = pos;
    }
}

#[cfg(feature = "protocol")]
impl From<&azalea_protocol::packets::game::clientbound_add_entity_packet::ClientboundAddEntityPacket>
    for Entity
{
    fn from(
        p: &azalea_protocol::packets::game::clientbound_add_entity_packet::ClientboundAddEntityPacket,
    ) -> Self {
        Self {
            id: p.id,
            uuid: p.uuid,
            pos: EntityPos {
                x: p.x,
                y: p.y,
                z: p.z,
            },
        }
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
