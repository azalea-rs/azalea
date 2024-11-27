use azalea_buf::AzBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerInventory {
    #[var]
    pub slot: i32,
    pub contents: Option<ItemStack>,
}
