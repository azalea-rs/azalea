use crate::packets::BufReadError;
use azalea_buf::McBuf;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetStructureBlockPacket {
    pub pos: BlockPos,
    pub update_type: UpdateType,
    pub mode: StructureMode,
    pub name: String,
    pub offset: BytePosition,
    pub size: BytePosition,
    pub mirror: Mirror,
    pub rotation: Rotation,
    pub data: String,
    pub integrity: f32,
    #[var]
    pub seed: u64,
    pub flags: Flags,
}

#[derive(Clone, Debug, McBuf)]
pub struct BytePosition {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum UpdateType {
    UpdateData = 0,
    SaveArea = 1,
    LoadArea = 2,
    ScanArea = 3,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum StructureMode {
    Save = 0,
    Load = 1,
    Corner = 2,
    Data = 3,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Mirror {
    None = 0,
    LeftRight = 1,
    FrontBack = 2,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Rotation {
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    Counterclockwise90 = 3,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub ignore_entities: bool,
    pub show_air: bool,
    pub show_bounding_box: bool,
}

impl McBufReadable for Flags {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        Ok(Self {
            ignore_entities: byte & 1 != 0,
            show_air: byte & 2 != 0,
            show_bounding_box: byte & 4 != 0,
        })
    }
}

impl McBufWritable for Flags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.ignore_entities {
            byte |= 1;
        }
        if self.show_air {
            byte |= 2;
        }
        if self.show_bounding_box {
            byte |= 4;
        }
        u8::write_into(&byte, buf)?;
        Ok(())
    }
}
