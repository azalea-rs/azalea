use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundHelloPacket {
    // TODO: make this len thing work
    // #[len(20)]
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}
