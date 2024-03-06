use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetDefaultSpawnPositionPacket {
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
pub angle: f32,
}