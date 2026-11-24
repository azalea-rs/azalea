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
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let string = String::azalea_read(buf)?;
        trace!("Got disconnect packet with string: {string:?}");
        let json = match serde_json::from_str::<serde_json::Value>(string.as_str()) {
            Ok(json) => json,
            Err(err) => {
                return Err(BufReadError::custom(format!(
                    "Failed to deserialize disconnect JSON {string:?}: {err}"
                )));
            }
        };

        let reason = FormattedText::codec_read(&json, Default::default())?;
        Ok(ClientboundLoginDisconnect { reason })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let json: serde_json::Value = self.reason.codec_write(Default::default())?;
        let string = serde_json::to_string(&json)?;
        string.azalea_write(buf)
    }
}
