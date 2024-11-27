use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundAwardStats {
    #[var]
    pub stats: HashMap<Stat, i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AzBuf)]
pub enum Stat {
    Mined(azalea_registry::Block),
    Crafted(azalea_registry::Item),
    Used(azalea_registry::Item),
    Broken(azalea_registry::Item),
    PickedUp(azalea_registry::Item),
    Dropped(azalea_registry::Item),
    Killed(azalea_registry::EntityKind),
    KilledBy(azalea_registry::EntityKind),
    Custom(azalea_registry::CustomStat),
}
