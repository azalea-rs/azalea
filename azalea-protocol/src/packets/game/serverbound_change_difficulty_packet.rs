use azalea_buf::McBuf;
use azalea_core::Difficulty;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChangeDifficultyPacket {
    pub difficulty: Difficulty,
}
