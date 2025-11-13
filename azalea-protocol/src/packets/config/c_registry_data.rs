use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;
use simdnbt::owned::NbtCompound;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundRegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<(Identifier, Option<NbtCompound>)>,
}
