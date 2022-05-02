use packet_macros::{GamePacket, McBufReadable, McBufWritable};

use super::clientbound_light_update_packet::ClientboundLightUpdatePacketData;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundLevelChunkWithLightPacket {
    pub x: i32,
    pub z: i32,
    pub chunk_data: ClientboundLevelChunkPacketData,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct ClientboundLevelChunkPacketData {
    heightmaps: azalea_nbt::Tag,
    // we can't parse the data in azalea-protocol because it dependso on context from other packets
    data: Vec<u8>,
    block_entities: Vec<BlockEntity>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct BlockEntity {
    packed_xz: u8,
    y: u16,
    #[varint]
    type_: i32,
    data: azalea_nbt::Tag,
}

