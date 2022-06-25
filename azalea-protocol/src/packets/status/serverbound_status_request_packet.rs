use azalea_buf::McBuf;
use packet_macros::StatusPacket;

#[derive(Clone, Debug, McBuf, StatusPacket)]
pub struct ServerboundStatusRequestPacket {}
