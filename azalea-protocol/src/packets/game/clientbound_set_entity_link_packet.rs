use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetEntityLinkPacket {
    pub source_id: u32,
    pub dest_id: u32,
}
