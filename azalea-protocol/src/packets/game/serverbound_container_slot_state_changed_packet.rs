use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundContainerSlotStateChangedPacket {
    #[var]
    pub slot_id: u32,
    #[var]
    pub container_id: u32,
    pub new_state: bool,
}
