use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundKeepAlive {
    pub id: u64,
}
