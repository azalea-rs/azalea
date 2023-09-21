use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundFinishConfigurationPacket {}
