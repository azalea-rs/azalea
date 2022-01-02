// i don't know the actual name of this packet, i couldn't find it in the source code!

use crate::mc_buf::{Readable, Writable};
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateViewDistancePacket {
    #[varint]
    pub view_distance: i32,
}
