use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundCustomReportDetails {
    pub details: HashMap<String, String>,
}
