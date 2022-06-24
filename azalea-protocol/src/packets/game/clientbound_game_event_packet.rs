use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundGameEventPacket {
    pub event: EventType,
    pub param: f32,
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum EventType {
    NoRespawnBlockAvailable = 0,
    StartRaining = 1,
    StopRaining = 2,
    ChangeGameMode = 3,
    WinGame = 4,
    DemoEvent = 5,
    ArrowHitPlayer = 6,
    RainLevelChange = 7,
    ThunderLevelChange = 8,
    PufferFishSting = 9,
    GuardianElderEffect = 10,
    ImmediateRespawn = 11,
}
