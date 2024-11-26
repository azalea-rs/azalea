use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket)]
pub struct ServerboundFinishConfiguration {}
