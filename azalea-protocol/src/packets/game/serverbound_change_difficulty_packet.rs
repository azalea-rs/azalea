use azalea_buf::McBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChangeDifficultyPacket {
    pub difficulty: Difficulty,
}
