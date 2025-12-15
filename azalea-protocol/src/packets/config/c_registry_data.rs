use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::NbtCompound;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundRegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<(Identifier, Option<NbtCompound>)>,
}
