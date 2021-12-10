use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash, Clone, Debug)]
pub struct ServerboundStatusRequestPacket {}

#[async_trait]
impl PacketTrait for ServerboundStatusRequestPacket {
    fn get(self) -> Packet {
        Packet::ServerboundStatusRequestPacket(self)
    }
    fn write(&self, _buf: &mut Vec<u8>) {}

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        Err("ServerboundStatusRequestPacket::read not implemented".to_string())
    }
}
