use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash, Clone, Debug)]
pub struct ServerboundHelloPacket {
    pub username: String,
}

#[async_trait]
impl PacketTrait for ServerboundHelloPacket {
    fn get(self) -> Packet {
        Packet::ServerboundHelloPacket(self)
    }
    fn write(&self, buf: &mut Vec<u8>) {
        mc_buf::write_utf(buf, &self.username);
    }

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        Err("ServerboundHelloPacket::read not implemented".to_string())
    }
}
