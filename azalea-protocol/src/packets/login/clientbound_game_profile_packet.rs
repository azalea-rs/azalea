use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundLoginPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundGameProfilePacket {
pub game_profile: BlockPos,
}