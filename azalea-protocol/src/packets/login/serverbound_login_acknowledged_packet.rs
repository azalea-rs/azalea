use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundLoginAcknowledgedPacket {}
