use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzBufLimited, BufReadError};
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ServerboundGamePacket;
use simdnbt::owned::{NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundCustomClickAction {
    pub id: Identifier,
    pub payload: Option<NbtCompound>,
}

impl AzBuf for ServerboundCustomClickAction {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        self.id.azalea_write(buf)?;

        let mut payload_bytes = Vec::new();
        match &self.payload {
            Some(c) => {
                NbtTag::Compound(c.clone()).write(&mut payload_bytes);
            }
            None => payload_bytes.push(0),
        }
        payload_bytes.azalea_write(buf)?;

        Ok(())
    }
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = Identifier::azalea_read(ctx, buf)?;

        let payload_bytes = Box::<[u8]>::azalea_read_limited(ctx, buf, 65536)?;
        let payload = simdnbt::owned::read_optional_tag(&mut Cursor::new(&payload_bytes))
            .map_err(simdnbt::Error::from)?;
        let payload = match payload {
            Some(NbtTag::Compound(c)) => Some(c),
            Some(tag) => return Err(simdnbt::Error::InvalidRootType(tag.id()).into()),
            None => None,
        };

        Ok(Self { id, payload })
    }
}
