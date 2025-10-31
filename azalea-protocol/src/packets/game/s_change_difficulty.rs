use azalea_buf::AzBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundGamePacket)]
pub struct ServerboundChangeDifficulty {
    pub difficulty: Difficulty,
}
