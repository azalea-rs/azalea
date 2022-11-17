use azalea_auth::game_profile::GameProfile;
use azalea_chat::Component;
use azalea_core::GameType;
use azalea_world::entity::EntityData;
use azalea_world::World;
use uuid::Uuid;

/// Something that has a world associated to it. Usually, this is a `Client`.
pub trait WorldHaver {
    fn world(&self) -> &World;
}

/// A player in the tab list.
#[derive(Debug)]
pub struct PlayerInfo {
    pub profile: GameProfile,
    /// The player's UUID.
    pub uuid: Uuid,
    pub gamemode: GameType,
    pub latency: i32,
    /// The player's display name in the tab list.
    pub display_name: Option<Component>,
}

impl PlayerInfo {
    /// Get a reference to the entity of the player in the world.
    pub fn entity<'d>(&'d self, world: &'d World) -> Option<&EntityData> {
        world.entity_by_uuid(&self.uuid)
    }

    /// Get a mutable reference to the entity of the player in the world.
    pub fn entity_mut<'d>(&'d mut self, world: &'d mut World) -> Option<&'d mut EntityData> {
        world.entity_mut_by_uuid(&self.uuid)
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}
