use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundTakeItemEntity {
    #[var]
    pub item_id: u32,
    #[var]
    pub player_id: MinecraftEntityId,
    #[var]
    pub amount: u32,
}
