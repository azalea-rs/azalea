use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::movements::PositionMoveRotation;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundEntityPositionSync {
    #[var]
    pub id: u32,
    pub values: PositionMoveRotation,
    pub on_ground: bool,
}
