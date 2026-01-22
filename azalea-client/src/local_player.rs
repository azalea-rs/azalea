use std::{collections::HashMap, sync::Arc};

use azalea_core::game_type::GameMode;
use azalea_world::{PartialWorld, World};
use bevy_ecs::{component::Component, prelude::*};
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use uuid::Uuid;

use crate::{player::PlayerInfo, ClientInformation};

/// A component that keeps strong references to our [`PartialWorld`] and
/// [`World`] for local players.
///
/// This can also act as a convenient way to access the player's `World`, since
/// the alternative is to look up the player's [`WorldName`] in the [`Worlds`]
/// resource.
///
/// [`Worlds`]: azalea_world::Worlds
/// [`WorldName`]: azalea_world::WorldName
#[derive(Clone, Component)]
pub struct WorldHolder {
    /// The slice of the world that this client actually has loaded, based on
    /// its render distance.
    pub partial: Arc<RwLock<PartialWorld>>,
    /// The combined [`PartialWorld`]s of all clients in the same world.
    ///
    /// The distinction between this and `partial` is mostly only relevant if
    /// you're using a shared world (i.e. a swarm). If in doubt, prefer to use
    /// the shared world.
    pub shared: Arc<RwLock<World>>,
}
#[deprecated = "renamed to `WorldHolder`."]
pub type InstanceHolder = WorldHolder;

/// The gamemode of a local player. For a non-local player, you can look up the
/// player in the [`TabList`].
#[derive(Clone, Component, Copy, Debug)]
pub struct LocalGameMode {
    pub current: GameMode,
    pub previous: Option<GameMode>,
}
impl From<GameMode> for LocalGameMode {
    fn from(current: GameMode) -> Self {
        LocalGameMode {
            current,
            previous: None,
        }
    }
}

/// Level must be 0..=4
#[derive(Clone, Component, Default, Deref, DerefMut)]
pub struct PermissionLevel(pub u8);

/// A component that contains a map of player UUIDs to their information in the
/// tab list.
///
/// ```
/// # use azalea_client::local_player::TabList;
/// fn example(tab_list: &TabList) {
///     println!("Online players:");
///     for (uuid, player_info) in tab_list.iter() {
///         println!("- {} ({}ms)", player_info.profile.name, player_info.latency);
///     }
/// }
/// ```
///
/// For convenience, `TabList` is also a resource in the ECS.
/// It's set to be the same as the tab list for the last client whose tab list
/// was updated.
/// This means you should avoid using `TabList` as a resource unless you know
/// all of your clients will have the same tab list.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut, Resource)]
pub struct TabList(HashMap<Uuid, PlayerInfo>);

#[derive(Clone, Component, Debug)]
pub struct Hunger {
    /// The main hunger bar. This is typically in the range `0..=20`.
    pub food: u32,
    /// The amount of saturation the player has.
    ///
    /// This isn't displayed in the vanilla Minecraft GUI, but it's used
    /// internally by the game. It's a decrementing counter, and the player's
    /// [`Hunger::food`] only starts decreasing when their saturation reaches 0.
    pub saturation: f32,
}

impl Default for Hunger {
    fn default() -> Self {
        Hunger {
            food: 20,
            saturation: 5.,
        }
    }
}
impl Hunger {
    /// Returns true if we have enough food level to sprint.
    ///
    /// Note that this doesn't consider our gamemode or passenger status.
    pub fn is_enough_to_sprint(&self) -> bool {
        // hasEnoughFoodToSprint
        self.food >= 6
    }
}

/// The player's experience state.
#[derive(Clone, Component, Debug)]
pub struct Experience {
    /// Progress towards the next level, in the range 0.0..1.0.
    pub progress: f32,
    /// The current experience level. You'll mostly be using this.
    pub level: u32,
    /// Total experience points accumulated.
    pub total: u32,
}

impl Default for Experience {
    fn default() -> Self {
        Experience {
            progress: 0.0,
            level: 0,
            total: 0,
        }
    }
}

impl WorldHolder {
    /// Create a new `WorldHolder` for the given entity.
    ///
    /// The partial world will be created for you. The render distance will
    /// be set to a default value, which you can change by creating a new
    /// partial world.
    pub fn new(entity: Entity, shared: Arc<RwLock<World>>) -> Self {
        let client_information = ClientInformation::default();

        WorldHolder {
            shared,
            partial: Arc::new(RwLock::new(PartialWorld::new(
                azalea_world::chunk_storage::calculate_chunk_storage_range(
                    client_information.view_distance.into(),
                ),
                Some(entity),
            ))),
        }
    }

    /// Reset the [`World`] to be a reference to an empty world, but with
    /// the same registries as the current one.
    ///
    /// This is used by Azalea when entering the config state.
    pub fn reset(&mut self) {
        let registries = self.shared.read().registries.clone();

        let new_world = World {
            registries,
            ..Default::default()
        };
        self.shared = Arc::new(RwLock::new(new_world));

        self.partial.write().reset();
    }
}
