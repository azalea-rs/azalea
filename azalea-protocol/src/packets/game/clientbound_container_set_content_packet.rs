use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetContentPacket {
#[var]
pub container_id: u32,
#[var]
pub state_id: u32,
}