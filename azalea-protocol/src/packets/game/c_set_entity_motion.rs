use azalea_buf::AzBuf;
use azalea_core::delta::PositionDelta8;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetEntityMotion {
    #[var]
    pub id: MinecraftEntityId,
    pub delta: PositionDelta8,
}
