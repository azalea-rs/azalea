use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;
use indexmap::IndexMap;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundCustomReportDetails {
    pub details: IndexMap<String, String>,
}
