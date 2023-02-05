use crate::packets::McBufWritable;
use azalea_buf::{BufReadError, McBuf, McBufReadable};
use azalea_core::{BlockPos, FixedBitSet};
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::Cursor;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundSetCommandBlockPacket {
    pub pos: BlockPos,
    pub command: String,
    pub mode: Mode,

    pub track_output: bool,
    pub conditional: bool,
    pub automatic: bool,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Mode {
    Sequence = 0,
    Auto = 1,
    Redstone = 2,
}

impl McBufReadable for ServerboundSetCommandBlockPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let pos = BlockPos::read_from(buf)?;
        let command = String::read_from(buf)?;
        let mode = Mode::read_from(buf)?;

        let set = FixedBitSet::<3>::read_from(buf)?;
        Ok(Self {
            pos,
            command,
            mode,
            track_output: set.index(0),
            conditional: set.index(1),
            automatic: set.index(2),
        })
    }
}

impl McBufWritable for ServerboundSetCommandBlockPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.pos.write_into(buf)?;
        self.command.write_into(buf)?;
        self.mode.write_into(buf)?;

        let mut set = FixedBitSet::<3>::new();
        if self.track_output {
            set.set(0);
        }
        if self.conditional {
            set.set(1);
        }
        if self.automatic {
            set.set(2);
        }
        set.write_into(buf)
    }
}
