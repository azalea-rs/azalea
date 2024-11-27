use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundSetBeacon {
    #[var]
    pub primary: Option<u32>,
    #[var]
    pub secondary: Option<u32>,
}
