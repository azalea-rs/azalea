use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundStatusPacket;

#[derive(AzBuf, ClientboundStatusPacket, Clone, Debug, PartialEq)]
pub struct ClientboundPongResponse {
    pub time: u64,
}
