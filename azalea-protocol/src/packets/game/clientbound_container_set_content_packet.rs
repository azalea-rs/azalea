use azalea_buf::McBuf;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetContentPacket {
    pub container_id: i8,
    #[var]
    pub state_id: u32,
    pub items: Vec<ItemSlot>,
    pub carried_item: ItemSlot,
}
