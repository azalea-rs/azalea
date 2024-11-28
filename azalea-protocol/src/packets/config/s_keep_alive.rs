use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundKeepAlive {
    pub id: u64,
}
