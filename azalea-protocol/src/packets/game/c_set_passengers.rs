use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetPassengers {
    #[var]
    pub vehicle: MinecraftEntityId,
    #[var]
    pub passengers: Vec<MinecraftEntityId>,
}
