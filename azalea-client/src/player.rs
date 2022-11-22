use azalea_auth::game_profile::GameProfile;
use azalea_chat::Component;
use azalea_core::GameType;
use azalea_world::World;
use uuid::Uuid;

/// Something that has a world associated to it. this is usually a `Client`.
pub trait WorldHaver {
    fn world(&self) -> &World;
}

/// A player in the tab list.
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub profile: GameProfile,
    /// The player's UUID.
    pub uuid: Uuid,
    pub gamemode: GameType,
    pub latency: i32,
    /// The player's display name in the tab list.
    pub display_name: Option<Component>,
}
