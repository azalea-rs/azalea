use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundTransferPacket {
    pub host: String,
    #[var]
    pub port: u32,
}
