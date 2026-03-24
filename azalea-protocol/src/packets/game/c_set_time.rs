use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::data::WorldClock;
use indexmap::IndexMap;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetTime {
    pub game_time: u64,
    pub clock_updates: IndexMap<WorldClock, ClockState>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct ClockState {
    #[var]
    pub total_ticks: u64,
    pub partial_tick: f32,
    pub rate: f32,
}
