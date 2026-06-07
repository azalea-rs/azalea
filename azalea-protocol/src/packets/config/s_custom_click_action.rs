use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::packets::OptionalNbt;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundConfigPacket)]
pub struct ServerboundCustomClickAction {
    pub id: Identifier,
    pub payload: OptionalNbt,
}
