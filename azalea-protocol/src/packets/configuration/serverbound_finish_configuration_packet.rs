use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundFinishConfigurationPacket {}
