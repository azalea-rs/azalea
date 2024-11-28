use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundPong {
    pub id: u32,
}
