use azalea_buf::{BitSet, McBuf};
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLightUpdatePacket {
    pub x: i32,
    pub z: i32,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, McBuf)]
pub struct ClientboundLightUpdatePacketData {
    trust_edges: bool,
    sky_y_mask: BitSet,
    block_y_mask: BitSet,
    empty_sky_y_mask: BitSet,
    empty_block_y_mask: BitSet,
    sky_updates: Vec<Vec<u8>>,
    block_updates: Vec<Vec<u8>>,
}
