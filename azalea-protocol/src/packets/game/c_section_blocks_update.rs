use std::io::{Cursor, Write};

use azalea_block::BlockState;
use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_core::position::{ChunkSectionBlockPos, ChunkSectionPos};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSectionBlocksUpdate {
    pub section_pos: ChunkSectionPos,
    pub states: Vec<BlockStateWithPosition>,
}

#[derive(Clone, Debug)]
pub struct BlockStateWithPosition {
    pub pos: ChunkSectionBlockPos,
    pub state: BlockState,
}

impl AzaleaRead for BlockStateWithPosition {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let data = u64::azalea_read_var(buf)?;
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

impl AzaleaWrite for BlockStateWithPosition {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let data = (self.state.id as u64) << 12
            | (u64::from(self.pos.x) << 8 | u64::from(self.pos.z) << 4 | u64::from(self.pos.y));
        u64::azalea_write_var(&data, buf)?;
        Ok(())
    }
}
