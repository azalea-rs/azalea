use std::{
    collections::HashMap,
    error, io,
    sync::{Arc, PoisonError},
};

use azalea_core::game_type::GameMode;
use azalea_world::{Instance, PartialInstance};
use bevy_ecs::{component::Component, prelude::*};
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{ClientInformation, events::Event as AzaleaEvent, player::PlayerInfo};

/// A component that keeps strong references to our [`PartialInstance`] and
/// [`Instance`] for local players.
///
/// This can also act as a convenience for accessing the player's Instance since
/// the alternative is to look up the player's [`InstanceName`] in the
/// [`InstanceContainer`].
///
/// [`InstanceContainer`]: azalea_world::InstanceContainer
/// [`InstanceName`]: azalea_world::InstanceName
#[derive(Component, Clone)]
pub struct InstanceHolder {
    /// The partial instance is the world this client currently has loaded.
    ///
    /// It has a limited render distance.
    pub partial_instance: Arc<RwLock<PartialInstance>>,
    /// The combined [`PartialInstance`]s of all clients in the same instance
    /// (aka world/dimension).
    ///
    /// This is only relevant if you're using a shared world (i.e. a
    /// swarm).
    pub instance: Arc<RwLock<Instance>>,
}

/// The gamemode of a local player. For a non-local player, you can look up the
/// player in the [`TabList`].
#[derive(Component, Clone, Debug, Copy)]
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
#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct PermissionLevel(pub u8);

/// A component that contains a map of player UUIDs to their information in the
/// tab list.
///
/// ```
/// # use azalea_client::local_player::TabList;
/// # fn example(client: &azalea_client::Client) {
/// let tab_list = client.component::<TabList>();
/// println!("Online players:");
/// for (uuid, player_info) in tab_list.iter() {
///     println!("- {} ({}ms)", player_info.profile.name, player_info.latency);
/// }
/// # }
/// ```
///
/// For convenience, `TabList` is also a resource in the ECS.
/// It's set to be the same as the tab list for the last client whose tab list
/// was updated.
/// This means you should avoid using `TabList` as a resource unless you know
/// all of your clients will have the same tab list.
#[derive(Component, Resource, Clone, Debug, Deref, DerefMut, Default)]
pub struct TabList(HashMap<Uuid, PlayerInfo>);

#[derive(Component, Clone)]
pub struct Hunger {
    /// The main hunger bar. This is typically in the range `0..=20`.
    pub food: u32,
    /// The amount of saturation the player has.
    ///
    /// This isn't displayed in the vanilla Minecraft GUI, but it's used
    /// internally by the game. It's a decrementing counter, and the player's
    /// [`Hunger::food`] only starts decreasing when this reaches 0.
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

impl InstanceHolder {
    /// Create a new `InstanceHolder` for the given entity.
    ///
    /// The partial instance will be created for you. The render distance will
    /// be set to a default value, which you can change by creating a new
    /// partial_instance.
    pub fn new(entity: Entity, instance: Arc<RwLock<Instance>>) -> Self {
        let client_information = ClientInformation::default();

        InstanceHolder {
            instance,
            partial_instance: Arc::new(RwLock::new(PartialInstance::new(
                azalea_world::chunk_storage::calculate_chunk_storage_range(
                    client_information.view_distance.into(),
                ),
                Some(entity),
            ))),
        }
    }

    /// Reset the `Instance` to a new reference to an empty instance, but with
    /// the same registries as the current one.
    ///
    /// This is used by Azalea when entering the config state.
    pub fn reset(&mut self) {
        let registries = self.instance.read().registries.clone();

        let new_instance = Instance {
            registries,
            ..Default::default()
        };
        self.instance = Arc::new(RwLock::new(new_instance));

        self.partial_instance.write().reset();
    }
}

#[derive(Error, Debug)]
pub enum HandlePacketError {
    #[error("{0}")]
    Poison(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn error::Error + Send + Sync>),
    #[error("{0}")]
    Send(#[from] mpsc::error::SendError<AzaleaEvent>),
}

impl<T> From<PoisonError<T>> for HandlePacketError {
    fn from(e: PoisonError<T>) -> Self {
        HandlePacketError::Poison(e.to_string())
    }
}
