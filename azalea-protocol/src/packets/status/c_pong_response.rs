use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundStatusPacket;

#[derive(Clone, Debug, AzBuf, ClientboundStatusPacket)]
pub struct ClientboundPongResponse {
    pub time: u64,
}
