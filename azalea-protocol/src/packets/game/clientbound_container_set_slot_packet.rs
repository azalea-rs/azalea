use azalea_buf::McBuf;
use azalea_core::Slot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetSlotPacket {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub slot: u16,
    pub item_stack: Slot,
}
