use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundLoginAcknowledged;
