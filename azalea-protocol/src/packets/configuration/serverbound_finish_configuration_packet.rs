use azalea_protocol_macros::ServerboundConfigurationPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundFinishConfigurationPacket {
}