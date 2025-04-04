use azalea_buf::AzBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundChangeDifficulty {
    pub difficulty: Difficulty,
}
