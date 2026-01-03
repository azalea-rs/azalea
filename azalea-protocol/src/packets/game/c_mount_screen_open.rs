use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMountScreenOpen {
    #[var]
    pub container_id: i32,
    #[var]
    pub inventory_columns: u32,
    pub entity_id: MinecraftEntityId,
}
