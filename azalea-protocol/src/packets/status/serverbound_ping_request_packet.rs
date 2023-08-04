use azalea_protocol_macros::ServerboundStatusPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ServerboundStatusPacket)]
pub struct ServerboundPingRequestPacket {
#[var]
pub time: u64,
}