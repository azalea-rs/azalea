use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetDataPacket {
    pub container_id: u8,
    pub id: u16,
    pub value: u16,
}
