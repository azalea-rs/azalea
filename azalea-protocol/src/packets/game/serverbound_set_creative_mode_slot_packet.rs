use azalea_buf::McBuf;
use azalea_inventory::ItemSlot;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetCreativeModeSlotPacket {
    pub slot_num: u16,
    pub item_stack: ItemSlot,
}
