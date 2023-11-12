use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAdvancementsPacket {
    pub reset: bool,
    pub added: todo!(),
    pub removed: Vec<ResourceLocation>,
    pub progress: todo!(),
}
