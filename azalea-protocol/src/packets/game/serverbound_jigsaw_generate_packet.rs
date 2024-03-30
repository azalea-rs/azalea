use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundJigsawGeneratePacket {
    pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
    #[var]
    pub levels: u32,
    pub keep_jigsaws: bool,
}
