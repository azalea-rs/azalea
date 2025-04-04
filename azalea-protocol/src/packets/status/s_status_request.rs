use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundStatusPacket;

#[derive(Clone, Debug, AzBuf, ServerboundStatusPacket)]
pub struct ServerboundStatusRequest;
