use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_core::{FixedBitSet, ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

use super::clientbound_sound_packet::SoundSource;

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundStopSoundPacket {
    pub source: Option<SoundSource>,
    pub name: Option<ResourceLocation>,
}

impl McBufReadable for ClientboundStopSoundPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<2>::read_from(buf)?;
        let source = if set.index(0) {
            Some(SoundSource::read_from(buf)?)
        } else {
            None
        };
        let name = if set.index(1) {
            Some(ResourceLocation::read_from(buf)?)
        } else {
            None
        };

        Ok(Self { source, name })
    }
}

impl McBufWritable for ClientboundStopSoundPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<2>::new();
        if self.source.is_some() {
            set.set(0);
        }
        if self.name.is_some() {
            set.set(1);
        }
        set.write_into(buf)?;
        if let Some(source) = &self.source {
            source.write_into(buf)?;
        }
        if let Some(name) = &self.name {
            name.write_into(buf)?;
        }
        Ok(())
    }
}
