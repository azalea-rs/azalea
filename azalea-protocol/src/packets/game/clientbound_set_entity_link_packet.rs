use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetEntityLinkPacket {
    pub source_id: u32,
    pub dest_id: u32,
}
