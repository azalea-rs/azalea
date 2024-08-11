use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigurationPacket;
use simdnbt::owned::NbtCompound;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundRegistryDataPacket {
    pub registry_id: ResourceLocation,
    pub entries: HashMap<ResourceLocation, Option<NbtCompound>>,
}
