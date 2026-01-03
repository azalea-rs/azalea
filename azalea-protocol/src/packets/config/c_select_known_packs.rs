use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use super::s_select_known_packs::KnownPack;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundSelectKnownPacks {
    pub known_packs: Vec<KnownPack>,
}
