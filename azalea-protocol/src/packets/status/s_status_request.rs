use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundStatusPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundStatusPacket)]
pub struct ServerboundStatusRequest;
