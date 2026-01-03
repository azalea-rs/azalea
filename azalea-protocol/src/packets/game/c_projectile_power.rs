use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundProjectilePower {
    #[var]
    pub id: MinecraftEntityId,
    pub acceleration_power: f64,
}
