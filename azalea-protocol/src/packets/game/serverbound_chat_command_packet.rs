use std::collections::HashMap;

use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundChatCommandPacket {
    pub command: String,
    // TODO: Choose a real timestamp type
    pub timestamp: u64,
    pub argument_signatures: ArgumentSignatures,
    pub signed_preview: bool,
}

#[derive(Clone, Debug, McBuf)]
pub struct ArgumentSignatures {
    pub salt: u64,
    pub signatures: HashMap<String, Vec<u8>>,
}
