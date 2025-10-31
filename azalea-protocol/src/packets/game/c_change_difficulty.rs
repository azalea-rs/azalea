use azalea_buf::AzBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundChangeDifficulty {
    pub difficulty: Difficulty,
    pub locked: bool,
}
