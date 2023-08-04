use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundEntityEventPacket {
#[var]
pub entity_id: u32,
#[var]
pub event_id: u32,
}