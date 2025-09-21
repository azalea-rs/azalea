use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundEntityTagQuery {
    #[var]
    pub transaction_id: u32,
    #[var]
    pub entity_id: MinecraftEntityId,
}
