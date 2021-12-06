use std::hash::Hash;

use crate::friendly_byte_buf::FriendlyByteBuf;

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
    fn get_id(&self) -> u8 {
        0x00
    }

    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, buf: &mut FriendlyByteBuf) {
        buf.write_varint(self.protocol_version);
        buf.write_utf(&self.hostname);
        buf.write_short(self.port);
        buf.write_varint(self.intention.clone() as u32);
    }
}
