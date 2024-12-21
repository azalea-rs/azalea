use std::io::{Cursor, Write};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundLoginPacket;
use serde::{Deserialize, Serialize};
use tracing::trace;

#[derive(Clone, Debug, ClientboundLoginPacket)]
pub struct ClientboundLoginDisconnect {
    pub reason: FormattedText,
}

impl AzaleaRead for ClientboundLoginDisconnect {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<ClientboundLoginDisconnect, BufReadError> {
        let disconnect_string = String::azalea_read(buf)?;
        trace!("Got disconnect packet with string: {disconnect_string:?}");
        let disconnect_json =
            match serde_json::from_str::<serde_json::Value>(disconnect_string.as_str()) {
                Ok(json) => json,
                Err(err) => {
                    return Err(BufReadError::Custom(format!(
                        "Failed to deserialize disconnect JSON {disconnect_string:?}: {err}"
                    )))
                }
            };

        Ok(ClientboundLoginDisconnect {
            reason: FormattedText::deserialize(disconnect_json)?,
        })
    }
}

impl AzaleaWrite for ClientboundLoginDisconnect {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let status_string = FormattedText::serialize(&self.reason, serde_json::value::Serializer)
            .unwrap()
            .to_string();
        status_string.azalea_write(buf)?;
        Ok(())
    }
}
