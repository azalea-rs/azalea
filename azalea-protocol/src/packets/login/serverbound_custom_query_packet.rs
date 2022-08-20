use azalea_buf::{McBuf, UnsizedByteArray};
use packet_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundCustomQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub data: Option<UnsizedByteArray>,
}
