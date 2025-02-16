use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityMotion {
    #[var]
    pub id: MinecraftEntityId,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}
