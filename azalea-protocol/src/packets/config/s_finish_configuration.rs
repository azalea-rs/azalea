use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundConfigPacket)]
pub struct ServerboundFinishConfiguration;
