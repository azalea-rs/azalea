use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPaddleBoatPacket {
    pub left: bool,
    pub right: bool,
}
