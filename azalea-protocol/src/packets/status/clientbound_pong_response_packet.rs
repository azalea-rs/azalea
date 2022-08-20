use azalea_buf::McBuf;
use packet_macros::ClientboundStatusPacket;

#[derive(Clone, Debug, McBuf, ClientboundStatusPacket)]
pub struct ClientboundPongResponsePacket {
    pub time: u64,
}
