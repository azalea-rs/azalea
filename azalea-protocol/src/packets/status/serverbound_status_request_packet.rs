use packet_macros::{McBuf, StatusPacket};

#[derive(Clone, Debug, McBuf, StatusPacket)]
pub struct ServerboundStatusRequestPacket {}
