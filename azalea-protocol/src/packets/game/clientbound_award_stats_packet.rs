use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;
use std::collections::HashMap;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAwardStatsPacket {
    #[var]
    pub stats: HashMap<Stat, i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, McBuf)]
pub struct Stat {
    // TODO: make these good enums and stuff
    #[var]
    pub stat_type: u32,
    #[var]
    pub statistic_id: u32,
}
