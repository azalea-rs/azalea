use std::hash::Hash;

use azalea_buf::{McBuf, UnsizedByteArray};
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Hash, Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryAnswerPacket {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
