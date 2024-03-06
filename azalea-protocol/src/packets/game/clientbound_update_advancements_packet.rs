use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAdvancementsPacket {
pub reset: bool,
pub removed: Vec<ResourceLocation>,
pub progress: todo!(),
}