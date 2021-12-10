use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundStatusResponsePacket {
    status: String,
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
        let status = mc_buf::read_utf(buf).await?;
        // this.status = GsonHelper.fromJson(GSON, friendlyByteBuf.readUtf(32767), ServerStatus.class);
        Ok(ClientboundStatusResponsePacket { status }.get())
    }
}
