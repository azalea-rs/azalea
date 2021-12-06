pub mod client_intention_packet;

use std::collections::HashMap;

use crate::friendly_byte_buf::FriendlyByteBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshaking = -1,
    Play = 0,
    Status = 1,
    Login = 2,
}

pub trait Packet {
    /// The id of the packet, this is always a byte in vanilla.
    /// This might be bigger than a u8 if using modpacks with lots of custom packets?
    const ID: u8;
 
    fn write(&self, friendly_byte_buf: &mut FriendlyByteBuf) -> ();
}
