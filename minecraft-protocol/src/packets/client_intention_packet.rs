use std::hash::Hash;

use crate::mc_buf;

use super::{ConnectionProtocol, Packet};

#[derive(Hash)]
pub struct ClientIntentionPacket<'a> {
    pub protocol_version: u32,
    pub hostname: &'a String,
    pub port: u16,
    /// 1 for status, 2 for login
    pub intention: ConnectionProtocol,
}

// implement "Packet" for "ClientIntentionPacket"
impl<'a> Packet for ClientIntentionPacket<'a> {
    fn get_id(&self) -> u32 {
        0x00
    }

    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, buf: &mut Vec<u8>) {
        mc_buf::write_varint(buf, self.protocol_version);
        mc_buf::write_utf(buf, &self.hostname);
        mc_buf::write_short(buf, self.port);
        mc_buf::write_varint(buf, self.intention.clone() as u32);
    }
}
