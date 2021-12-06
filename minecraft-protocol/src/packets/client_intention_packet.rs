use std::hash::Hash;

use crate::friendly_byte_buf::FriendlyByteBuf;

use super::{ConnectionProtocol, Packet};

#[derive(Hash)]
pub struct ClientIntentionPacket {
    protocol_version: u32,
    hostname: String,
    port: u16,
    /// 1 for status, 2 for login
    intention: ConnectionProtocol,
}

// implement "Packet" for "ClientIntentionPacket"
impl Packet for ClientIntentionPacket {
    const ID: u8 = 0x00;

    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, buf: &mut FriendlyByteBuf) {
        buf.write_varint(self.protocol_version);
        buf.write_utf(&self.hostname);
        buf.write_short(self.port);
        buf.write_varint(self.intention.clone() as u32);
    }
}

