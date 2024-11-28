use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ClientboundLoginPacket)]
pub struct ClientboundHello {
    #[limit(20)]
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub challenge: Vec<u8>,
    pub should_authenticate: bool,
}
