use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, BufReadError};
use azalea_chat::FormattedText;
use azalea_codec::Codec;
use azalea_protocol_macros::ClientboundLoginPacket;
use tracing::trace;

#[derive(ClientboundLoginPacket, Clone, Debug, PartialEq)]
pub struct ClientboundLoginDisconnect {
    pub reason: FormattedText,
}

impl AzBuf for ClientboundLoginDisconnect {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let string = String::azalea_read(ctx, buf)?;
        trace!("Got disconnect packet with string: {string:?}");
        let mut string = string.into_bytes();
        let json = match simd_json::from_slice::<simd_json::OwnedValue>(&mut string) {
            Ok(json) => json,
            Err(err) => {
                return Err(BufReadError::custom(format!(
                    "Failed to deserialize disconnect JSON {string:?}: {err}"
                )));
            }
        };

        let reason = FormattedText::codec_read(&json)?;
        Ok(ClientboundLoginDisconnect { reason })
    }
    fn azalea_write(&self, ctx: azalea_buf::Ctx, buf: &mut impl Write) -> io::Result<()> {
        let json: simd_json::OwnedValue = self.reason.codec_write()?;
        let string = serde_json::to_string(&json)?;
        string.azalea_write(ctx, buf)
    }
}
