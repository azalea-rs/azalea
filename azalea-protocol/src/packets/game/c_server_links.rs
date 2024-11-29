use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

use crate::common::server_links::ServerLinkEntry;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundServerLinks {
    pub links: Vec<ServerLinkEntry>,
}
