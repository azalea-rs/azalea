use azalea_core::BlockPos;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetDefaultSpawnPositionPacket {
    pub pos: BlockPos,
    pub angle: f32,
}
