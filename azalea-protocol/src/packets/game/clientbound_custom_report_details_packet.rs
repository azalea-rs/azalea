use std::collections::HashMap;

use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCustomReportDetailsPacket {
    // azalea doesn't implement max lengths yet

    // max length = 32
    // key string is limited to 128 bytes
    // value string is limited to 4096 bytes
    pub details: HashMap<String, String>,
}
