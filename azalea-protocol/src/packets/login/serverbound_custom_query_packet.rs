use azalea_buf::{McBuf, UnsizedByteArray};
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
