use azalea_buf::McBuf;
use azalea_core::Slot;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundContainerSetContentPacket {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub items: Vec<Slot>,
    pub carried_item: Slot,
}
