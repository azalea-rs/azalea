use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::movements::{PositionMoveRotation, RelativeMovements};


#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTeleportEntity {
    #[var]
    pub id: u32,
    pub change: PositionMoveRotation,
    pub relatives: RelativeMovements,
    pub on_ground: bool,
}
