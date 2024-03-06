use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundContainerClickPacket {
#[var]
pub container_id: u32,
#[var]
pub state_id: u32,
#[var]
pub slot_num: u32,
#[var]
pub button_num: u32,
pub click_type: ClickType,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ClickType {
    Pickup=0,
    QuickMove=1,
    Swap=2,
    Clone=3,
    Throw=4,
    QuickCraft=5,
    PickupAll=6,
}