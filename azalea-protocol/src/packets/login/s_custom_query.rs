use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQuery {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
