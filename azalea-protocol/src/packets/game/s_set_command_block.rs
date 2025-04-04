use std::io::Cursor;

use azalea_buf::{AzBuf, AzaleaRead, BufReadError};
use azalea_core::{bitset::FixedBitSet, position::BlockPos};
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::AzaleaWrite;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundSetCommandBlock {
    pub pos: BlockPos,
    pub command: String,
    pub mode: Mode,

    pub track_output: bool,
    pub conditional: bool,
    pub automatic: bool,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum Mode {
    Sequence = 0,
    Auto = 1,
    Redstone = 2,
}

impl AzaleaRead for ServerboundSetCommandBlock {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let pos = BlockPos::azalea_read(buf)?;
        let command = String::azalea_read(buf)?;
        let mode = Mode::azalea_read(buf)?;

        let set = FixedBitSet::<{ 3_usize.div_ceil(8) }>::azalea_read(buf)?;
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

impl AzaleaWrite for ServerboundSetCommandBlock {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.pos.azalea_write(buf)?;
        self.command.azalea_write(buf)?;
        self.mode.azalea_write(buf)?;

        let mut set = FixedBitSet::<{ 3_usize.div_ceil(8) }>::new();
        if self.track_output {
            set.set(0);
        }
        if self.conditional {
            set.set(1);
        }
        if self.automatic {
            set.set(2);
        }
        set.azalea_write(buf)
    }
}
