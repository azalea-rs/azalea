use azalea_buf::McBuf;
use azalea_core::Difficulty;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
