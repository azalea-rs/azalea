use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundContainerSetData {
    #[var]
    pub container_id: i32,
    pub id: u16,
    pub value: u16,
}
