use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::item::ItemStack;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetCreativeModeSlot {
    pub slot_num: u16,
    pub item_stack: ItemStack,
}
