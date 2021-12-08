use std::hash::Hash;

use super::Packet;

#[derive(Hash)]
pub struct ServerboundStatusRequestPacket {
    // status: ServerStatus,
    status: String,
}

// implement "Packet" for "ClientIntentionPacket"
impl Packet for ServerboundStatusRequestPacket {
    fn get_id(&self) -> u32 {
        0x00
    }

    // implement "from_reader" for "ClientIntentionPacket"
    fn write(&self, _buf: &mut Vec<u8>) {}
    fn parse<T: tokio::io::AsyncRead + std::marker::Unpin>(&self, buf: T) -> () {
        mc_buf::read_utf;
        // this.status = GsonHelper.fromJson(GSON, friendlyByteBuf.readUtf(32767), ServerStatus.class);
    }
}
