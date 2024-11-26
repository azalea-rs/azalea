use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::NbtCompound;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundRegistryData {
    pub registry_id: ResourceLocation,
    pub entries: HashMap<ResourceLocation, Option<NbtCompound>>,
}
