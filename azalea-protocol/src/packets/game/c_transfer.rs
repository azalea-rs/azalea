use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundTransfer {
    pub host: String,
    #[var]
    pub port: u32,
}
