use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundTransfer {
    pub host: String,
    #[var]
    pub port: u32,
}
