use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundStatusPacket;

#[derive(Clone, Debug, McBuf, ClientboundStatusPacket)]
pub struct ClientboundPongResponsePacket {
    #[var]
    pub time: u64,
}
