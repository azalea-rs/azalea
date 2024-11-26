use azalea_buf::McBuf;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetCreativeModeSlot {
    pub slot_num: u16,
    pub item_stack: ItemStack,
}
