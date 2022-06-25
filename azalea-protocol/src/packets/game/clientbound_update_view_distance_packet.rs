use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundUpdateViewDistancePacket {
    #[var]
    pub view_distance: i32,
}
