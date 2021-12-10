use async_trait::async_trait;
use std::hash::Hash;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash)]
pub struct ClientboundStatusRequestPacket {
    status: String,
}

impl PacketTrait for ClientboundStatusRequestPacket {
    fn get(&self) -> Packet {
        Packet::ClientboundStatusRequestPacket(self)
    }

    fn write(&self, _buf: &mut Vec<u8>) {}

    fn parse<T: tokio::io::AsyncRead + std::marker::Unpin>(
        buf: &mut T,
    ) -> Result<Packet<'_>, String> {
        let status = mc_buf::read_utf(&mut buf).await?;
        // this.status = GsonHelper.fromJson(GSON, friendlyByteBuf.readUtf(32767), ServerStatus.class);
        Ok(ClientboundStatusRequestPacket { status }.get())
    }
}
