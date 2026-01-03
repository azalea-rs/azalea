mod container;

#[cfg(test)]
mod tests;

use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
pub use container::*;

/// A representation of the different types of chunk palettes Minecraft uses.
#[derive(Clone, Debug)]
pub enum Palette<S: PalletedContainerKind> {
    /// ID of the corresponding entry in its global palette
    SingleValue(S),
    // in vanilla this keeps a `size` field that might be less than the length, but i'm not sure
    // it's actually needed?
    Linear(Vec<S>),
    Hashmap(Vec<S>),
    Global,
}

impl<S: PalletedContainerKind> Palette<S> {
    pub fn value_for(&self, id: usize) -> S {
        match self {
            Palette::SingleValue(v) => *v,
            Palette::Linear(v) => v.get(id).copied().unwrap_or_default(),
            Palette::Hashmap(v) => v.get(id).copied().unwrap_or_default(),
            Palette::Global => S::try_from(id as u32).unwrap_or_default(),
        }
    }
}

impl<S: PalletedContainerKind> AzaleaWrite for Palette<S> {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            Palette::SingleValue(value) => {
                (*value).into().azalea_write_var(buf)?;
            }
            Palette::Linear(values) => {
                (values.len() as u32).azalea_write_var(buf)?;
                for value in values {
                    (*value).into().azalea_write_var(buf)?;
                }
            }
            Palette::Hashmap(values) => {
                (values.len() as u32).azalea_write_var(buf)?;
                for value in values {
                    (*value).into().azalea_write_var(buf)?;
                }
            }
            Palette::Global => {}
        }
        Ok(())
    }
}

impl PaletteKind {
    pub fn read<S: PalletedContainerKind>(
        &self,
        buf: &mut Cursor<&[u8]>,
    ) -> Result<Palette<S>, BufReadError> {
        Ok(match self {
            // since they're read as varints it's actually fine to just use BlockStateIntegerRepr
            // instead of the correct type (u32)
            PaletteKind::SingleValue => {
                Palette::SingleValue(S::try_from(u32::azalea_read_var(buf)?).unwrap_or_default())
            }
            PaletteKind::Linear => Palette::Linear(
                Vec::<u32>::azalea_read_var(buf)?
                    .into_iter()
                    .map(|v| S::try_from(v).unwrap_or_default())
                    .collect(),
            ),
            PaletteKind::Hashmap => Palette::Hashmap(
                Vec::<u32>::azalea_read_var(buf)?
                    .into_iter()
                    .map(|v| S::try_from(v).unwrap_or_default())
                    .collect(),
            ),
            PaletteKind::Global => Palette::Global,
        })
    }

    pub fn as_empty_palette<S: PalletedContainerKind>(&self) -> Palette<S> {
        match self {
            PaletteKind::SingleValue => Palette::SingleValue(S::default()),
            PaletteKind::Linear => Palette::Linear(Vec::new()),
            PaletteKind::Hashmap => Palette::Hashmap(Vec::new()),
            PaletteKind::Global => Palette::Global,
        }
    }
}

impl<S: PalletedContainerKind> From<&Palette<S>> for PaletteKind {
    fn from(palette: &Palette<S>) -> Self {
        match palette {
            Palette::SingleValue(_) => PaletteKind::SingleValue,
            Palette::Linear(_) => PaletteKind::Linear,
            Palette::Hashmap(_) => PaletteKind::Hashmap,
            Palette::Global => PaletteKind::Global,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaletteKind {
    SingleValue,
    Linear,
    Hashmap,
    Global,
}
