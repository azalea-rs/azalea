use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::movements::{PositionMoveRotation, RelativeMovements};

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundPlayerPosition {
    /// The teleport ID.
    #[var]
    pub id: u32,
    pub change: PositionMoveRotation,
    pub relative: RelativeMovements,
}
