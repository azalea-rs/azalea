use std::io::Cursor;
use std::io::Write;

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::bitset::FixedBitSet;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundBossEvent {
    pub id: Uuid,
    pub operation: Operation,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add(AddOperation),
    Remove,
    UpdateProgress(f32),
    UpdateName(FormattedText),
    UpdateStyle(Style),
    UpdateProperties(Properties),
}

impl AzaleaRead for Operation {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let operation_id = u32::azalea_read_var(buf)?;
        Ok(match operation_id {
            0 => Operation::Add(AddOperation::azalea_read(buf)?),
            1 => Operation::Remove,
            2 => Operation::UpdateProgress(f32::azalea_read(buf)?),
            3 => Operation::UpdateName(FormattedText::azalea_read(buf)?),
            4 => Operation::UpdateStyle(Style::azalea_read(buf)?),
            5 => Operation::UpdateProperties(Properties::azalea_read(buf)?),
            _ => {
                return Err(BufReadError::UnexpectedEnumVariant {
                    id: operation_id as i32,
                })
            }
        })
    }
}

impl AzaleaWrite for Operation {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Operation::Add(add) => {
                0u32.azalea_write_var(buf)?;
                add.azalea_write(buf)?;
            }
            Operation::Remove => {
                1u32.azalea_write_var(buf)?;
            }
            Operation::UpdateProgress(progress) => {
                2u32.azalea_write_var(buf)?;
                progress.azalea_write(buf)?;
            }
            Operation::UpdateName(name) => {
                3u32.azalea_write_var(buf)?;
                name.azalea_write(buf)?;
            }
            Operation::UpdateStyle(style) => {
                4u32.azalea_write_var(buf)?;
                style.azalea_write(buf)?;
            }
            Operation::UpdateProperties(properties) => {
                5u32.azalea_write_var(buf)?;
                properties.azalea_write(buf)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, AzBuf)]
pub struct AddOperation {
    pub name: FormattedText,
    pub progress: f32,
    pub style: Style,
    pub properties: Properties,
}

#[derive(Clone, Debug, AzBuf)]
pub struct Style {
    pub color: BossBarColor,
    pub overlay: BossBarOverlay,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum BossBarOverlay {
    Progress = 0,
    Notched6 = 1,
    Notched10 = 2,
    Notched12 = 3,
    Notched20 = 4,
}

#[derive(Clone, Debug)]
pub struct Properties {
    pub darken_screen: bool,
    pub play_music: bool,
    pub create_world_fog: bool,
}

impl AzaleaRead for Properties {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<{ 3_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(Self {
            darken_screen: set.index(0),
            play_music: set.index(1),
            create_world_fog: set.index(2),
        })
    }
}

impl AzaleaWrite for Properties {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 3_usize.div_ceil(8) }>::new();
        if self.darken_screen {
            set.set(0);
        }
        if self.play_music {
            set.set(1);
        }
        if self.create_world_fog {
            set.set(2);
        }
        set.azalea_write(buf)?;
        Ok(())
    }
}
