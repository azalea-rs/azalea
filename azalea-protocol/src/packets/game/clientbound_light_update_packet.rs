use azalea_buf::McBuf;
use azalea_core::bitset::BitSet;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLightUpdatePacket {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, McBuf)]
pub struct ClientboundLightUpdatePacketData {
    pub sky_y_mask: BitSet,
    pub block_y_mask: BitSet,
    pub empty_sky_y_mask: BitSet,
    pub empty_block_y_mask: BitSet,
    pub sky_updates: Vec<Vec<u8>>,
    pub block_updates: Vec<Vec<u8>>,
}
