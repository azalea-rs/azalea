use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::common::ClientInformation;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket, PartialEq, Eq)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}
