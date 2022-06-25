use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetEntityLinkPacket {
    pub source_id: u32,
    pub dest_id: u32,
}
