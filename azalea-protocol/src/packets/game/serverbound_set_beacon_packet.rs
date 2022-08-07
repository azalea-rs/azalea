use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetBeaconPacket {
    #[var]
    pub primary: Option<u32>,
    #[var]
    pub secondary: Option<u32>,
}
