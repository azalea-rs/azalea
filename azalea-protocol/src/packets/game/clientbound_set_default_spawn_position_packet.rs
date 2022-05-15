use azalea_core::BlockPos;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetDefaultSpawnPositionPacket {
    pub pos: BlockPos,
    pub angle: f32,
}
