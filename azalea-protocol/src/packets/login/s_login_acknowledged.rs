use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ServerboundLoginPacket)]
pub struct ServerboundLoginAcknowledged;
