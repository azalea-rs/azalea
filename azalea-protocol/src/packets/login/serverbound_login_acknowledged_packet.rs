use azalea_protocol_macros::ServerboundLoginPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundLoginAcknowledgedPacket {
}