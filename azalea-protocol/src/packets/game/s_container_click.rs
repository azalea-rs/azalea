use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_inventory::{operations::ClickType, ItemStack};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundContainerClick {
    #[var]
    pub container_id: i32,
    #[var]
    pub state_id: u32,
    pub slot_num: i16,
    pub button_num: u8,
    pub click_type: ClickType,
    pub changed_slots: HashMap<u16, ItemStack>,
    pub carried_item: ItemStack,
}
