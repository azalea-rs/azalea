use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_inventory::{operations::ClickType, ItemSlot};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundContainerClick {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub slot_num: i16,
    pub button_num: u8,
    pub click_type: ClickType,
    pub changed_slots: HashMap<u16, ItemSlot>,
    pub carried_item: ItemSlot,
}
