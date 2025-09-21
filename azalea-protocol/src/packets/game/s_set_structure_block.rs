use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
use azalea_core::{bitset::FixedBitSet, position::BlockPos};
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetStructureBlock {
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

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct BytePosition {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum UpdateType {
    UpdateData = 0,
    SaveArea = 1,
    LoadArea = 2,
    ScanArea = 3,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum StructureMode {
    Save = 0,
    Load = 1,
    Corner = 2,
    Data = 3,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum Mirror {
    #[default]
    None = 0,
    LeftRight = 1,
    FrontBack = 2,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum Rotation {
    #[default]
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    Counterclockwise90 = 3,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags {
    pub ignore_entities: bool,
    pub show_air: bool,
    pub show_bounding_box: bool,
}

impl AzaleaRead for Flags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<3>::azalea_read(buf)?;
        Ok(Self {
            ignore_entities: set.index(0),
            show_air: set.index(1),
            show_bounding_box: set.index(2),
        })
    }
}

impl AzaleaWrite for Flags {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut set = FixedBitSet::<3>::new();
        if self.ignore_entities {
            set.set(0);
        }
        if self.show_air {
            set.set(1);
        }
        if self.show_bounding_box {
            set.set(2);
        }
        set.azalea_write(buf)?;
        Ok(())
    }
}
