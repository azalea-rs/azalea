use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundStatusPacket;

#[derive(Clone, Debug, McBuf, ServerboundStatusPacket)]
pub struct ServerboundPingRequestPacket {
    pub time: u64,
}
