use azalea_core::ChunkSectionPos;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSectionBlocksUpdatePacket {
    pub section_pos: ChunkSectionPos,
    pub suppress_light_updates: bool,
    #[var]
    pub states: Vec<u64>,
}
