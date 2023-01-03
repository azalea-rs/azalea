use azalea_auth::game_profile::GameProfile;
use azalea_chat::FormattedText;
use azalea_core::GameType;
use uuid::Uuid;

/// A player in the tab list.
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub profile: GameProfile,
    /// The player's UUID.
    pub uuid: Uuid,
    pub gamemode: GameType,
    pub latency: i32,
    /// The player's display name in the tab list.
    pub display_name: Option<FormattedText>,
}
