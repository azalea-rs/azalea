use packet_macros::StatusPacket;
use std::hash::Hash;

#[derive(Clone, Debug, StatusPacket)]
pub struct ServerboundStatusRequestPacket {}
