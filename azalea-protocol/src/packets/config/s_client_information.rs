use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

use crate::common::ClientInformation;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket, PartialEq, Eq)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}
