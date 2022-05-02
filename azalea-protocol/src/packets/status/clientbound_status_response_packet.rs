use std::io::Read;

use azalea_chat::component::Component;
use serde::Deserialize;
use serde_json::Value;

use crate::mc_buf::Readable;

use super::StatusPacket;

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
    pub max: u32,
    pub online: u32,
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
    pub fn get(self) -> StatusPacket {
        StatusPacket::ClientboundStatusResponsePacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<StatusPacket, String> {
        let status_string = buf.read_utf()?;
        let status_json: Value =
            serde_json::from_str(status_string.as_str()).expect("Server status isn't valid JSON");

        let packet = ClientboundStatusResponsePacket::deserialize(status_json)
            .map_err(|e| e.to_string())?
            .get();

        Ok(packet)
    }
}
