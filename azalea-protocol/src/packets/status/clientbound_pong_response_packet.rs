use azalea_protocol_macros::ClientboundStatusPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundStatusPacket)]
pub struct ClientboundPongResponsePacket {
#[var]
pub time: u64,
}