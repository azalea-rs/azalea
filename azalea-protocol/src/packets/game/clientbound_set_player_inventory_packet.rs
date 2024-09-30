use azalea_buf::McBuf;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetPlayerInventoryPacket {
    #[var]
    pub slot: i32,
    pub contents: Option<ItemSlot>,
}
