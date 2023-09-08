use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::configuration::serverbound_client_information_packet::ClientInformation;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientInformationPacket {
    pub information: ClientInformation,
}
