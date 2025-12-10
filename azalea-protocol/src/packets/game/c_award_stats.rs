use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::{BlockKind, CustomStat, EntityKind, ItemKind};

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundAwardStats {
    #[var]
    pub stats: HashMap<Stat, i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AzBuf)]
pub enum Stat {
    Mined(BlockKind),
    Crafted(ItemKind),
    Used(ItemKind),
    Broken(ItemKind),
    PickedUp(ItemKind),
    Dropped(ItemKind),
    Killed(EntityKind),
    KilledBy(EntityKind),
    Custom(CustomStat),
}
