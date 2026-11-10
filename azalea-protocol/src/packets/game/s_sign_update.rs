use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::sign::SignTextSlot;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSignUpdate {
    pub pos: BlockPos,
    pub lines: [String; 4],
    /// Minecraft 26.3 serializes the side after the four lines, not as a
    /// boolean before them.
    pub slot: SignTextSlot,
}
