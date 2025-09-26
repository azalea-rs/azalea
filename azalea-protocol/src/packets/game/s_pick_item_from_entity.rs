use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPickItemFromEntity {
    #[var]
    pub id: MinecraftEntityId,
    pub include_data: bool,
}
