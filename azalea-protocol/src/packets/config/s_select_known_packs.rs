use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundSelectKnownPacks {
    pub known_packs: Vec<KnownPack>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
