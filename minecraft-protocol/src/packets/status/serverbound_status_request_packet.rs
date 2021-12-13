use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::packets::{Packet, PacketTrait, ProtocolPacket};

use super::StatusPacket;

#[derive(Hash, Clone, Debug)]
pub struct ServerboundStatusRequestPacket {}

#[async_trait]
impl PacketTrait for ServerboundStatusRequestPacket {
    fn get(self) -> StatusPacket {
        StatusPacket::ServerboundStatusRequestPacket(self)
    }
    fn write(&self, _buf: &mut Vec<u8>) {
        panic!("ServerboundStatusRequestPacket::write not implemented")
    }

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        Err("ServerboundStatusRequestPacket::read not implemented".to_string())
    }
}
