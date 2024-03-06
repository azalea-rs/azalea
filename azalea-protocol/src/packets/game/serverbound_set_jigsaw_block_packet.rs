use azalea_core::resource_location::ResourceLocation;
use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetJigsawBlockPacket {
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
pub name: ResourceLocation,
pub target: ResourceLocation,
pub pool: ResourceLocation,
pub final_state: String,
pub joint: String, // TODO: Does JigsawBlockEntity$JointType::getSerializedName, may not be implemented
#[var]
pub selection_priority: u32,
#[var]
pub placement_priority: u32,
}