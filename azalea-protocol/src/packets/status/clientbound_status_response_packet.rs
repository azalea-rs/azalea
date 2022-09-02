use super::ClientboundStatusPacket;
use azalea_buf::{BufReadError, McBufReadable};
use azalea_chat::component::Component;
use serde::Deserialize;
use serde_json::Value;
use std::io::{Read, Write};

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub name: Component,
    pub protocol: u32,
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
#[derive(Clone, Debug, Deserialize)]
pub struct ClientboundStatusResponsePacket {
    pub description: Component,
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}

impl ClientboundStatusResponsePacket {
    pub fn get(self) -> ClientboundStatusPacket {
        ClientboundStatusPacket::ClientboundStatusResponsePacket(self)
    }

    pub fn write(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<ClientboundStatusPacket, BufReadError> {
        let status_string = String::read_from(buf)?;
        let status_json: Value = serde_json::from_str(status_string.as_str())?;

        let packet = ClientboundStatusResponsePacket::deserialize(status_json)?.get();

        Ok(packet)
    }
}
