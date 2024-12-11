use std::io::{Cursor, Write};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{bitset::FixedBitSet, resource_location::ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;

use super::c_sound::SoundSource;

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundStopSound {
    pub source: Option<SoundSource>,
    pub name: Option<ResourceLocation>,
}

impl AzaleaRead for ClientboundStopSound {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<{ 2_usize.div_ceil(8) }>::azalea_read(buf)?;
        let source = if set.index(0) {
            Some(SoundSource::azalea_read(buf)?)
        } else {
            None
        };
        let name = if set.index(1) {
            Some(ResourceLocation::azalea_read(buf)?)
        } else {
            None
        };

        Ok(Self { source, name })
    }
}

impl AzaleaWrite for ClientboundStopSound {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 2_usize.div_ceil(8) }>::new();
        if self.source.is_some() {
            set.set(0);
        }
        if self.name.is_some() {
            set.set(1);
        }
        set.azalea_write(buf)?;
        if let Some(source) = &self.source {
            source.azalea_write(buf)?;
        }
        if let Some(name) = &self.name {
            name.azalea_write(buf)?;
        }
        Ok(())
    }
}
