use crate::friendly_byte_buf::FriendlyByteBuf;

use super::{ConnectionProtocol, Packet};

pub struct ClientIntentionPacket {
    protocol_version: u32,
    hostname: String,
    port: u16,
    intention: ConnectionProtocol,
}

// implement "Packet" for "ClientIntentionPacket"
impl Packet for ClientIntentionPacket {
    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, buf: &mut FriendlyByteBuf) {
        buf.write_varint(self.protocol_version);
        buf.write_utf(&self.hostname);
        buf.write_short(self.port);
        buf.write_varint(self.intention.clone() as u32);
    }
}
