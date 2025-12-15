use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use crate::common::server_links::ServerLinkEntry;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundServerLinks {
    pub links: Vec<ServerLinkEntry>,
}
