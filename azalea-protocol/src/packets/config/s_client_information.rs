use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::common::client_information::ClientInformation;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundConfigPacket)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}
