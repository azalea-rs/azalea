use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEventPacket {
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
#[var]
pub b0: u32,
#[var]
pub b1: u32,
}