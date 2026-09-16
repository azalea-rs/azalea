use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use azalea_registry::identifier::Identifier;
use simdnbt::owned::NbtTag;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundRegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<(Identifier, Option<NbtTag>)>,
}
