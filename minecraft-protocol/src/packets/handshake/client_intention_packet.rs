use std::hash::Hash;

use crate::{
    mc_buf,
    packets::{ConnectionProtocol, Packet, PacketTrait},
};

#[derive(Hash)]
pub struct ClientIntentionPacket<'a> {
    pub protocol_version: u32,
    pub hostname: &'a String,
    pub port: u16,
    /// 1 for status, 2 for login
    pub intention: ConnectionProtocol,
}

impl<'a> PacketTrait for ClientIntentionPacket<'a> {
    fn get(&self) -> Packet {
        Packet::ClientIntentionPacket(self)
    }

    fn write(&self, buf: &mut Vec<u8>) {
        mc_buf::write_varint(buf, self.protocol_version as i32);
        mc_buf::write_utf(buf, &self.hostname);
        mc_buf::write_short(buf, self.port);
        mc_buf::write_varint(buf, self.intention.clone() as i32);
    }

    fn parse<T: tokio::io::AsyncRead + std::marker::Unpin>(&self, buf: T) -> () {}
}
