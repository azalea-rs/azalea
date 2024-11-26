use azalea_buf::McBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerInventory {
    #[var]
    pub slot: i32,
    pub contents: Option<ItemStack>,
}
