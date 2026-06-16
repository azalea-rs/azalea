use azalea_buf::AzBuf;
use azalea_core::entity_id::OptionalEntityId;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSpectatorAction {
    pub spectate_entity_id: OptionalEntityId,
}
