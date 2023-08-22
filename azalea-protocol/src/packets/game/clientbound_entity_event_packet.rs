use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundEntityEventPacket {
    pub entity_id: u32,
    pub event_id: u8,
}
