use async_trait::async_trait;
use minecraft_chat::component::Component;
use serde_json::Value;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Clone, Debug)]
struct Version {
    name: String,
    protocol: u32,
}

#[derive(Clone, Debug)]
struct SamplePlayer {
    id: String,
    name: String,
}

#[derive(Clone, Debug)]
struct Players {
    max: u32,
    online: u32,
    sample: Vec<SamplePlayer>,
}

#[derive(Clone, Debug)]
pub struct ClientboundStatusResponsePacket {
    // version: Version,
    description: Component,
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
        let description_string: &Value = status_json.get("description").unwrap();

        // this.status = GsonHelper.fromJson(GSON, friendlyByteBuf.readUtf(32767), ServerStatus.class);
        Ok(ClientboundStatusResponsePacket {
            // version: status_json.get("version"),
            description: Component::new(description_string)?,
        }
        .get())
    }
}
