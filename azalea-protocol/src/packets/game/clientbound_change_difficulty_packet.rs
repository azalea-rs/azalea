use azalea_buf::McBuf;
use azalea_core::Difficulty;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
