use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::NbtCompound;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundRegistryData {
    pub registry_id: ResourceLocation,
    pub entries: HashMap<ResourceLocation, Option<NbtCompound>>,
}
