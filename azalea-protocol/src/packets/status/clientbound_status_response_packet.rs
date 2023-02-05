use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundStatusPacket;
use serde::{Deserialize, Serialize};
use serde_json::{value::Serializer, Value};
use std::io::{Cursor, Write};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SamplePlayer {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<SamplePlayer>,
}

// the entire packet is just json, which is why it has deserialize
#[derive(Clone, Debug, Serialize, Deserialize, ClientboundStatusPacket)]
pub struct ClientboundStatusResponsePacket {
    pub description: FormattedText,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previewsChat")]
    pub previews_chat: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
}

impl McBufReadable for ClientboundStatusResponsePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<ClientboundStatusResponsePacket, BufReadError> {
        let status_string = String::read_from(buf)?;
        let status_json: Value = serde_json::from_str(status_string.as_str())?;

        Ok(ClientboundStatusResponsePacket::deserialize(status_json)?)
    }
}

impl McBufWritable for ClientboundStatusResponsePacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let status_string = ClientboundStatusResponsePacket::serialize(self, Serializer)
            .unwrap()
            .to_string();
        status_string.write_into(buf)?;
        Ok(())
    }
}
