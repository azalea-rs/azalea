use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket)]
pub struct ServerboundSelectKnownPacks {
    pub known_packs: Vec<KnownPack>,
}

#[derive(Clone, Debug, McBuf)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
