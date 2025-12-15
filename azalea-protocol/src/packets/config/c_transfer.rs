use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundTransfer {
    pub host: String,
    #[var]
    pub port: u32,
}
