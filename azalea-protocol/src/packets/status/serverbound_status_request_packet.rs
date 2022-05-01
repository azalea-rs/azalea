use packet_macros::StatusPacket;

#[derive(Clone, Debug, StatusPacket)]
pub struct ServerboundStatusRequestPacket {}
