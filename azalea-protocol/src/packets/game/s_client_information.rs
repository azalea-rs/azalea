use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::client_information::ClientInformation;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundClientInformation {
    pub client_information: ClientInformation,
}
