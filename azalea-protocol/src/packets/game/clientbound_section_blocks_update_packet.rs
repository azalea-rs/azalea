use azalea_buf::McBuf;
use azalea_buf::{McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_core::{ChunkSectionBlockPos, ChunkSectionPos};
use packet_macros::GamePacket;
use std::io::{Read, Write};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSectionBlocksUpdatePacket {
    pub section_pos: ChunkSectionPos,
    pub suppress_light_updates: bool,
    pub states: Vec<BlockStateWithPosition>,
}

#[derive(Clone, Debug)]
pub struct BlockStateWithPosition {
    pub pos: ChunkSectionBlockPos,
    pub state: u32,
}

impl McBufReadable for BlockStateWithPosition {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let data = u64::var_read_into(buf)?;
        let position_part = data & 4095;
        let state = (data >> 12) as u32;
        let position = ChunkSectionBlockPos {
            x: (position_part >> 8 & 15) as u8,
            y: (position_part >> 0 & 15) as u8,
            z: (position_part >> 4 & 15) as u8,
        };
        Ok(BlockStateWithPosition {
            pos: position,
            state: state,
        })
    }
}

impl McBufWritable for BlockStateWithPosition {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let data = (self.state as u64) << 12
            | ((self.pos.x as u64) << 8 | (self.pos.z as u64) << 4 | (self.pos.y as u64));
        u64::var_write_into(&data, buf)?;
        Ok(())
    }
}
