use std::hash::Hash;

use super::Packet;

#[derive(Hash)]
pub struct ServerboundStatusRequestPacket {}

// implement "Packet" for "ClientIntentionPacket"
impl Packet for ServerboundStatusRequestPacket {
    fn get_id(&self) -> u32 {
        0x00
    }

    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, _buf: &mut Vec<u8>) {}
}
