use azalea_buf::McBuf;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetSlotPacket {
    pub container_id: i8,
    #[var]
    pub state_id: u32,
    pub slot: u16,
    pub item_stack: ItemSlot,
}
