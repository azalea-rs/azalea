use azalea_buf::AzBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundChangeDifficulty {
    pub difficulty: Difficulty,
    pub locked: bool,
}
