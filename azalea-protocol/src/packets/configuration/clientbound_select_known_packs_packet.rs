use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigurationPacket;

use super::serverbound_select_known_packs_packet::KnownPack;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundSelectKnownPacksPacket {
    pub known_packs: Vec<KnownPack>,
}
