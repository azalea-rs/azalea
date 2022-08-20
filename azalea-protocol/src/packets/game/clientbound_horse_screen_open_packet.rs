use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundHorseScreenOpenPacket {
    pub container_id: u8,
    #[var]
    pub size: u32,
    pub entity_id: u32,
}
