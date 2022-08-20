use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundForgetLevelChunkPacket {
    pub x: i32,
    pub z: i32,
}
