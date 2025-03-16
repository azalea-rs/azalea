use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundHurtAnimation {
    #[var]
    pub id: MinecraftEntityId,
    pub yaw: f32,
}
