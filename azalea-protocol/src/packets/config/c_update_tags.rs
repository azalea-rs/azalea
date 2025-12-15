use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use crate::common::tags::TagMap;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundUpdateTags {
    pub tags: TagMap,
}
