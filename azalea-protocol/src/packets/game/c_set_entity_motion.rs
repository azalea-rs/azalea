use azalea_buf::AzBuf;
use azalea_core::delta::LpVec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetEntityMotion {
    #[var]
    pub id: MinecraftEntityId,
    pub delta: LpVec3,
}
