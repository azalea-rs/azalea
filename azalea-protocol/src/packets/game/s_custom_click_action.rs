use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;
use simdnbt::owned::Nbt;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundCustomClickAction {
    pub id: ResourceLocation,
    pub payload: Nbt,
}
