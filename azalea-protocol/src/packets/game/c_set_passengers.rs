use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetPassengers {
    #[var]
    pub vehicle: MinecraftEntityId,
    #[var]
    pub passengers: Vec<MinecraftEntityId>,
}
