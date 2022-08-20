use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundContainerButtonClickPacket {
    pub container_id: u8,
    pub button_id: u8,
}
