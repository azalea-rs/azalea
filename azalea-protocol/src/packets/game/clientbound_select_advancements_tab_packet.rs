use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSelectAdvancementsTabPacket {
    pub tab: Option<ResourceLocation>,
}
