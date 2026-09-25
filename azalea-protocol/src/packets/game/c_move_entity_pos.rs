use std::io::{self, Cursor, Write};

use azalea_buf::AzBuf;
use azalea_core::entity_id::MinecraftEntityId;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::packets::game::{ClientboundMoveEntityPosRot, c_move_entity_pos_rot};

#[derive(ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveEntityPos {
    pub entity_id: MinecraftEntityId,
    pub properties: c_move_entity_pos_rot::Properties,
    pub delta: c_move_entity_pos_rot::VecDelta,
}
impl AzBuf for ClientboundMoveEntityPos {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let p = c_move_entity_pos_rot::read_move_entity_packet(buf, false)?;
        Ok(Self {
            entity_id: p.entity_id,
            delta: p.delta,
            properties: p.properties,
        })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let p = ClientboundMoveEntityPosRot {
            entity_id: self.entity_id,
            delta: self.delta.clone(),
            look_direction: Default::default(),
            properties: self.properties,
        };
        c_move_entity_pos_rot::write_move_entity_packet(buf, &p, false)
    }
}
