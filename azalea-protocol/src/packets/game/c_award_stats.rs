use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::{Block, CustomStat, EntityKind, Item};

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundAwardStats {
    #[var]
    pub stats: HashMap<Stat, i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AzBuf)]
pub enum Stat {
    Mined(Block),
    Crafted(Item),
    Used(Item),
    Broken(Item),
    PickedUp(Item),
    Dropped(Item),
    Killed(EntityKind),
    KilledBy(EntityKind),
    Custom(CustomStat),
}
