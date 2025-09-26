use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundLoginAcknowledged;
