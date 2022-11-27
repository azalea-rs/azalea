use azalea_buf::McBuf;
use azalea_core::Slot;
use azalea_protocol_macros::ServerboundGamePacket;
use std::collections::HashMap;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundContainerClickPacket {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub slot_num: u16,
    pub button_num: u8,
    pub click_type: ClickType,
    pub changed_slots: HashMap<u16, Slot>,
    pub carried_item: Slot,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ClickType {
    Pickup = 0,
    QuickMove = 1,
    Swap = 2,
    Clone = 3,
    Throw = 4,
    QuickCraft = 5,
    PickupAll = 6,
}
