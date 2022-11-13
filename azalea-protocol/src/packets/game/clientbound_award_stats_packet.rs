use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use std::collections::HashMap;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAwardStatsPacket {
    #[var]
    pub stats: HashMap<Stat, i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, McBuf)]
pub enum Stat {
    Mined(azalea_registry::Block),
    Crafted(azalea_registry::Item),
    Used(azalea_registry::Item),
    Broken(azalea_registry::Item),
    PickedUp(azalea_registry::Item),
    Dropped(azalea_registry::Item),
    Killed(azalea_registry::EntityType),
    KilledBy(azalea_registry::EntityType),
    Custom(azalea_registry::CustomStat),
}
