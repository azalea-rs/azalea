use azalea_buf::AzBuf;
use azalea_core::delta::LpVec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetEntityMotion {
    #[var]
    pub id: MinecraftEntityId,
    pub delta: LpVec3,
}
