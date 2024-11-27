use std::hash::Hash;

use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Hash, Clone, Debug, AzBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryAnswer {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
