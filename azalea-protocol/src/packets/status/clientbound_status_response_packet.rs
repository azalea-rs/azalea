use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_chat::Component;
use azalea_protocol_macros::ClientboundStatusPacket;
use serde::Deserialize;
use serde_json::Value;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub name: Component,
    pub protocol: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SamplePlayer {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<SamplePlayer>,
}

// the entire packet is just json, which is why it has deserialize
#[derive(Clone, Debug, Deserialize, ClientboundStatusPacket)]
pub struct ClientboundStatusResponsePacket {
    pub description: Component,
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}

impl McBufReadable for ClientboundStatusResponsePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<ClientboundStatusResponsePacket, BufReadError> {
        let status_string = String::read_from(buf)?;
        let status_json: Value = serde_json::from_str(status_string.as_str())?;

        Ok(ClientboundStatusResponsePacket::deserialize(status_json)?)
    }
}

impl McBufWritable for ClientboundStatusResponsePacket {
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}
