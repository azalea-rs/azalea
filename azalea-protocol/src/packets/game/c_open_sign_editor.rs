use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::sign::SignTextSlot;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundOpenSignEditor {
    pub pos: BlockPos,
    pub slot: SignTextSlot,
}
