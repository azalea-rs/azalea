use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundSelectKnownPacksPacket {
    pub known_packs: Vec<KnownPack>,
}

#[derive(Clone, Debug, McBuf)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
