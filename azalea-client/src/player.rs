use azalea_auth::game_profile::GameProfile;
use azalea_chat::FormattedText;
use azalea_core::GameType;
use azalea_world::entity::EntityInfos;
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Res},
};
use uuid::Uuid;

use crate::{packet_handling::AddPlayerEvent, GameProfileComponent};

/// A player in the tab list.
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    /// Information about the player's Minecraft account, including their
    /// username.
    pub profile: GameProfile,
    /// The player's UUID.
    pub uuid: Uuid,
    pub gamemode: GameType,
    pub latency: i32,
    /// The player's display name in the tab list, but only if it's different
    /// from the player's normal username. Use `player_info.profile.name` to get
    /// the player's actual username.
    pub display_name: Option<FormattedText>,
}

/// Add a [`GameProfileComponent`] when an [`AddPlayerEvent`] is received.
/// Usually the `GameProfileComponent` will be added from the
/// `ClientboundGamePacket::AddPlayer` handler though.
pub fn retroactively_add_game_profile_component(
    mut commands: Commands,
    mut events: EventReader<AddPlayerEvent>,
    entity_infos: Res<EntityInfos>,
) {
    for event in events.iter() {
        if let Some(entity) = entity_infos.get_entity_by_uuid(&event.info.uuid) {
            commands
                .entity(entity)
                .insert(GameProfileComponent(event.info.profile.clone()));
        }
    }
}
