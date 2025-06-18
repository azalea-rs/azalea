use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigPacket;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundCustomClickAction {
    pub id: ResourceLocation,
    pub payload: Nbt,
}
