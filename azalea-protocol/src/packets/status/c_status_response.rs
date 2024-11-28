use std::io::{Cursor, Write};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundStatusPacket;
use serde::{Deserialize, Serialize};
use serde_json::value::Serializer;

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
pub struct ClientboundStatusResponse {
    pub description: FormattedText,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
}

impl AzaleaRead for ClientboundStatusResponse {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<ClientboundStatusResponse, BufReadError> {
        let status_string = String::azalea_read(buf)?;
        let status_json: serde_json::Value = serde_json::from_str(status_string.as_str())?;

        Ok(ClientboundStatusResponse::deserialize(status_json)?)
    }
}

impl AzaleaWrite for ClientboundStatusResponse {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let status_string = ClientboundStatusResponse::serialize(self, Serializer)
            .unwrap()
            .to_string();
        status_string.azalea_write(buf)?;
        Ok(())
    }
}
