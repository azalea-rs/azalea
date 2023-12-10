use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundLoginPacket;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ClientboundLoginPacket)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: FormattedText,
}

impl McBufReadable for ClientboundLoginDisconnectPacket {
    fn read_from(
        buf: &mut Cursor<&[u8]>,
    ) -> Result<ClientboundLoginDisconnectPacket, BufReadError> {
        let disconnect_string = String::read_from(buf)?;
        let disconnect_json: serde_json::Value = serde_json::from_str(disconnect_string.as_str())?;

        Ok(ClientboundLoginDisconnectPacket {
            reason: FormattedText::deserialize(disconnect_json)?,
        })
    }
}

impl McBufWritable for ClientboundLoginDisconnectPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let status_string = FormattedText::serialize(&self.reason, serde_json::value::Serializer)
            .unwrap()
            .to_string();
        status_string.write_into(buf)?;
        Ok(())
    }
}
