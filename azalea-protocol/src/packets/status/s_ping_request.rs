use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundStatusPacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundStatusPacket)]
pub struct ServerboundPingRequest {
    pub time: u64,
}
