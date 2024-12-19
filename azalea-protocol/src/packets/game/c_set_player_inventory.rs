use azalea_buf::AzBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerInventory {
    #[var]
    pub slot: u32,
    pub contents: ItemStack,
}
