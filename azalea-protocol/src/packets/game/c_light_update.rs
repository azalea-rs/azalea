use azalea_buf::AzBuf;
use azalea_core::bitset::BitSet;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundLightUpdate {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, AzBuf)]
pub struct ClientboundLightUpdatePacketData {
    pub sky_y_mask: BitSet,
    pub block_y_mask: BitSet,
    pub empty_sky_y_mask: BitSet,
    pub empty_block_y_mask: BitSet,
    pub sky_updates: Vec<Vec<u8>>,
    pub block_updates: Vec<Vec<u8>>,
}
