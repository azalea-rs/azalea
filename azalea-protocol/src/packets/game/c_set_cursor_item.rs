use azalea_buf::McBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetCursorItem {
    pub contents: Option<ItemStack>,
}
