use std::sync::Arc;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::heightmap::HeightmapKind;
use simdnbt::owned::Nbt;

use super::c_light_update::ClientboundLightUpdatePacketData;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundLevelChunkWithLight {
    // this can't be a ChunkPos since that reads z first and then x
    pub x: i32,
    pub z: i32,
    pub chunk_data: ClientboundLevelChunkPacketData,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct ClientboundLevelChunkPacketData {
    pub heightmaps: Vec<(HeightmapKind, Box<[u64]>)>,
    /// The raw chunk sections.
    ///
    /// We can't parse the data in azalea-protocol because it depends on context
    /// from other packets
    ///
    /// This is an Arc because it's often very big and we want it to be cheap to
    /// clone.
    pub data: Arc<Box<[u8]>>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: u16,
    pub kind: azalea_registry::BlockEntityKind,
    pub data: Nbt,
}
