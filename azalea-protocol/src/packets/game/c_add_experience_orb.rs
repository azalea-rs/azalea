use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundAddExperienceOrb {
    #[var]
    pub id: MinecraftEntityId,
    pub pos: Vec3,
    pub value: u16,
}
