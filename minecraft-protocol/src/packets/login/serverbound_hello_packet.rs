use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::{mc_buf, packets::Packet};

use super::LoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ServerboundHelloPacket {
    pub username: String,
}

impl ServerboundHelloPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ServerboundHelloPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        mc_buf::write_utf(buf, &self.username);
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<LoginPacket, String> {
        Err("ServerboundHelloPacket::read not implemented".to_string())
    }
}
