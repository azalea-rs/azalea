use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_core::entity_id::MinecraftEntityId;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPickItemFromEntity {
    #[var]
    pub id: MinecraftEntityId,
    pub include_data: bool,
}
