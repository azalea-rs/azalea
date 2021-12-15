use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::packets::{Packet, ProtocolPacket};

use super::StatusPacket;

#[derive(Hash, Clone, Debug)]
pub struct ServerboundStatusRequestPacket {}

impl ServerboundStatusRequestPacket {
    pub fn get(self) -> StatusPacket {
        StatusPacket::ServerboundStatusRequestPacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) {
        panic!("ServerboundStatusRequestPacket::write not implemented")
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _buf: &mut BufReader<T>,
    ) -> Result<StatusPacket, String> {
        Err("ServerboundStatusRequestPacket::read not implemented".to_string())
    }
}
