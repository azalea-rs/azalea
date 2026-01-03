use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundTakeItemEntity {
    #[var]
    pub item_id: u32,
    #[var]
    pub player_id: MinecraftEntityId,
    #[var]
    pub amount: u32,
}
