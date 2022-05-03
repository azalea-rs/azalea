use crate::mc_buf::BitSet;
use packet_macros::{GamePacket, McBufReadable, McBufWritable};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundLightUpdatePacket {
    pub x: i32,
    pub z: i32,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct ClientboundLightUpdatePacketData {
    trust_edges: bool,
    sky_y_mask: BitSet,
    block_y_mask: BitSet,
    empty_sky_y_mask: BitSet,
    empty_block_y_mask: BitSet,
    sky_updates: Vec<Vec<u8>>,
    block_updates: Vec<Vec<u8>>,
}
