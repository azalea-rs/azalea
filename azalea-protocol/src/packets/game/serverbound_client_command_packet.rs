use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientCommandPacket {
    pub action: Action,
}
#[derive(McBuf, Copy, Debug)]
enum Action {
    PerformRespawn = 0,
    RequestStats = 1,
}
