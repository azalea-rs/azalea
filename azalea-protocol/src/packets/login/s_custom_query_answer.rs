use std::hash::Hash;

use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(AzBuf, Clone, Debug, Hash, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryAnswer {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
