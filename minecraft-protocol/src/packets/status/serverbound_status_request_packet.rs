use std::hash::Hash;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash)]
pub struct ServerboundStatusRequestPacket {}

// implement "Packet" for "ClientIntentionPacket"
impl PacketTrait for ServerboundStatusRequestPacket {
    fn get(&self) -> Packet {
        Packet::ServerboundStatusRequestPacket(self)
    }
    fn write(&self, _buf: &mut Vec<u8>) {}
    fn parse<T: tokio::io::AsyncRead + std::marker::Unpin>(&self, buf: T) -> () {}
}
