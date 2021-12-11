use async_trait::async_trait;
use minecraft_chat::component::Component;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub name: String,
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

#[derive(Clone, Debug, Deserialize)]
pub struct ClientboundStatusResponsePacket {
    pub version: Version,
    pub description: Component,
}

#[async_trait]
impl PacketTrait for ClientboundStatusResponsePacket {
    fn get(self) -> Packet {
        Packet::ClientboundStatusResponsePacket(self)
    }

    fn write(&self, _buf: &mut Vec<u8>) {}

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        let status_string = mc_buf::read_utf(buf).await?;
        let status_json: Value =
            serde_json::from_str(status_string.as_str()).expect("Server status isn't valid JSON");

        Ok(ClientboundStatusResponsePacket::deserialize(status_json)
            .map_err(|e| e.to_string())?
            .get())
    }
}
