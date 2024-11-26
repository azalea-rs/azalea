use azalea_buf::McBuf;
use azalea_core::difficulty::Difficulty;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChangeDifficulty {
    pub difficulty: Difficulty,
    pub locked: bool,
}
