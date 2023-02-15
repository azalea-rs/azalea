use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddPaintingPacket {
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    #[var]
    pub motive: u32,
    pub pos: BlockPos,
    pub direction: u8, // TODO: Does Direction::get2DDataValue, may not be implemented
}
