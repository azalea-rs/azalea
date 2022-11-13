use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
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
        let byte = u8::read_from(buf)?;
        let source = if byte & 1 != 0 {
            Some(SoundSource::read_from(buf)?)
        } else {
            None
        };
        let name = if byte & 2 != 0 {
            Some(ResourceLocation::read_from(buf)?)
        } else {
            None
        };

        Ok(Self { source, name })
    }
}

impl McBufWritable for ClientboundStopSoundPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut byte = 0u8;
        if self.source.is_some() {
            byte |= 1;
        }
        if self.name.is_some() {
            byte |= 2;
        }
        byte.write_into(buf)?;
        if let Some(source) = &self.source {
            source.write_into(buf)?;
        }
        if let Some(name) = &self.name {
            name.write_into(buf)?;
        }
        Ok(())
    }
}
