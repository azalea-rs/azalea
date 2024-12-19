use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundContainerSlotStateChanged {
    #[var]
    pub slot_id: u32,
    #[var]
    pub container_id: i32,
    pub new_state: bool,
}
