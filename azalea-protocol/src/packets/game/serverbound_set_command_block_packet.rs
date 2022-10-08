use crate::packets::McBufWritable;
use azalea_buf::{BufReadError, McBuf, McBufReadable};
use azalea_core::BlockPos;
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

        let byte = u8::read_from(buf)?;
        let track_output = byte & 1 != 0;
        let conditional = byte & 2 != 0;
        let automatic = byte & 4 != 0;
        Ok(Self {
            pos,
            command,
            mode,
            track_output,
            conditional,
            automatic,
        })
    }
}

impl McBufWritable for ServerboundSetCommandBlockPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.pos.write_into(buf)?;
        self.command.write_into(buf)?;
        self.mode.write_into(buf)?;

        let mut byte: u8 = 0;
        if self.track_output {
            byte |= 1;
        }
        if self.conditional {
            byte |= 2;
        }
        if self.automatic {
            byte |= 4;
        }
        byte.write_into(buf)?;
        Ok(())
    }
}
