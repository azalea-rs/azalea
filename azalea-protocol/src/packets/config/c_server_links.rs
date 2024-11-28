use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use crate::common::server_links::ServerLinkEntry;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundServerLinks {
    pub links: Vec<ServerLinkEntry>,
}
