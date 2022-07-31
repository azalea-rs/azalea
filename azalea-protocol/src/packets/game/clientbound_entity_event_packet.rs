use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

// we can't identify the status in azalea-protocol since they vary depending on the entity
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundEntityEventPacket {
    pub entity_id: u32,
    pub event_id: u8,
}
