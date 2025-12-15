use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundCodeOfConduct {
    pub code_of_conduct: String,
}
