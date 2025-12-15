use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use indexmap::IndexMap;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundCustomReportDetails {
    pub details: IndexMap<String, String>,
}
