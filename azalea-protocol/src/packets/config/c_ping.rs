use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundPing {
    pub id: u32,
}
