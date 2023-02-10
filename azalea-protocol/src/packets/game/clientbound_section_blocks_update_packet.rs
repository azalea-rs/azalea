use azalea_block::BlockState;
use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::{ChunkSectionBlockPos, ChunkSectionPos};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSectionBlocksUpdatePacket {
    pub section_pos: ChunkSectionPos,
    pub suppress_light_updates: bool,
    pub states: Vec<BlockStateWithPosition>,
}

#[derive(Clone, Debug)]
pub struct BlockStateWithPosition {
    pub pos: ChunkSectionBlockPos,
    pub state: BlockState,
}

impl McBufReadable for BlockStateWithPosition {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let data = u64::var_read_from(buf)?;
        let position_part = data & 4095;
        let state = (data >> 12) as u32;
        let state = BlockState::try_from(state)
            .map_err(|_| BufReadError::UnexpectedEnumVariant { id: state as i32 })?;
        let pos = ChunkSectionBlockPos {
            x: (position_part >> 8 & 15) as u8,
            y: (position_part & 15) as u8,
            z: (position_part >> 4 & 15) as u8,
        };
        Ok(BlockStateWithPosition { pos, state })
    }
}

impl McBufWritable for BlockStateWithPosition {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let data = (self.state.id as u64) << 12
            | (u64::from(self.pos.x) << 8 | u64::from(self.pos.z) << 4 | u64::from(self.pos.y));
        u64::var_write_into(&data, buf)?;
        Ok(())
    }
}
