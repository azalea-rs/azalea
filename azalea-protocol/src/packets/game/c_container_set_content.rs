use azalea_buf::McBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetContent {
    pub container_id: i8,
    #[var]
    pub state_id: u32,
    pub items: Vec<ItemStack>,
    pub carried_item: ItemStack,
}
