use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use super::s_select_known_packs::KnownPack;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundSelectKnownPacks {
    pub known_packs: Vec<KnownPack>,
}
