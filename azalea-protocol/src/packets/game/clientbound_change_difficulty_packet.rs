use azalea_core::difficulty::Difficulty;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
