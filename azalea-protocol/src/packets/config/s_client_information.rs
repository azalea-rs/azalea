use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::common::client_information::ClientInformation;

#[derive(AzBuf, Clone, Debug, Eq, PartialEq, ServerboundConfigPacket)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}
