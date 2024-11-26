use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::common::ClientInformation;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientInformation {
    pub information: ClientInformation,
}
