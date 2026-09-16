use std::sync::Arc;

use azalea_buf::AzBuf;
use azalea_core::heightmap_kind::HeightmapKind;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::BlockEntityKind;
use simdnbt::owned::NbtTag;

use super::c_light_update::ClientboundLightUpdatePacketData;

#[derive(ClientboundGamePacket, AzBuf, Clone, Debug, PartialEq)]
pub struct ClientboundLevelChunkWithLight {
    // this can't be a ChunkPos since that reads z first and then x
    pub x: i32,
    pub z: i32,
    pub chunk_data: ClientboundLevelChunkPacketData,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ClientboundLevelChunkPacketData {
    pub heightmaps: Vec<(HeightmapKind, Box<[u64]>)>,
    /// The raw chunk sections.
    ///
    /// We can't parse the data in `azalea-protocol` because sometimes we want
    /// to skip parsing this.
    ///
    /// This is an Arc because it's often very big and we want it to be cheap to
    /// clone.
    pub buffer: Arc<Box<[u8]>>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: u16,
    pub kind: BlockEntityKind,
    /// This should only ever be a compound tag or end tag. We're a bit more
    /// permissive than vanilla here because simdnbt doesn't have a type
    /// that's equivalent to OPTIONAL_COMPOUND_TAG right now.
    pub tag: NbtTag,
}
