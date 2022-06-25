use azalea_buf::McBuf;
use packet_macros::LoginPacket;

#[derive(Clone, Debug, McBuf, LoginPacket)]
pub struct ClientboundHelloPacket {
    // TODO: make this len thing work
    // #[len(20)]
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}
