use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

use crate::common::tags::TagMap;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundConfigPacket)]
pub struct ClientboundUpdateTags {
    pub tags: TagMap,
}
