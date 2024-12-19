use azalea_buf::AzBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundContainerSetSlot {
    #[var]
    pub container_id: i32,
    #[var]
    pub state_id: u32,
    pub slot: u16,
    pub item_stack: ItemStack,
}
