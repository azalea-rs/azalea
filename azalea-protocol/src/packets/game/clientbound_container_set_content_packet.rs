use azalea_buf::McBuf;
use azalea_core::Slot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetContentPacket {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub items: Vec<Slot>,
    pub carried_item: Slot,
}
