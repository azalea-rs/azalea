use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use super::s_select_known_packs::KnownPack;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundSelectKnownPacks {
    pub known_packs: Vec<KnownPack>,
}
