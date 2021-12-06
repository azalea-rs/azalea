pub mod client_intention_packet;

use crate::friendly_byte_buf::FriendlyByteBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionProtocol {
    Handshaking = -1,
    Play = 0,
    Status = 1,
    Login = 2,
}

pub trait Packet {
    fn write(&self, friendly_byte_buf: &mut FriendlyByteBuf) -> ();
}

struct PacketSet<'a> {
    pub packets: Vec<&'a dyn Packet>,
}

impl<'a> PacketSet<'a> {
    fn add_packet(&mut self, packet: &'a dyn Packet) {
        self.packets.push(packet);
    }
}

// PacketSet
