use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundStatusPacket;

#[derive(Clone, Debug, McBuf, ClientboundStatusPacket)]
pub struct ClientboundPongResponsePacket {
    pub time: u64,
}
