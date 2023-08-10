use std::{collections::HashMap, io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, GameMode};
use azalea_entity::{Dead, Position};
use azalea_protocol::packets::game::{
    clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket,
    serverbound_client_information_packet::ServerboundClientInformationPacket,
    ServerboundGamePacket,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, PartialInstance};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::*,
    query::Added,
    system::{Query, Res},
};
use derive_more::{Deref, DerefMut};
use log::error;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    events::{Event as AzaleaEvent, LocalPlayerEvents},
    raw_connection::RawConnection,
    PlayerInfo, WalkDirection,
};

/// A component that keeps strong references to our [`PartialInstance`] and
/// [`Instance`] for local players.
#[derive(Component)]
pub struct InstanceHolder {
    /// The partial instance is the world this client currently has loaded. It
    /// has a limited render distance.
    pub partial_instance: Arc<RwLock<PartialInstance>>,
    /// The world is the combined [`PartialInstance`]s of all clients in the
    /// same world. (Only relevant if you're using a shared world, i.e. a
    /// swarm)
    pub instance: Arc<RwLock<Instance>>,
}

/// Component for entities that can move and sprint. Usually only in
/// [`LocalEntity`]s.
#[derive(Default, Component)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    pub move_direction: WalkDirection,
    pub forward_impulse: f32,
    pub left_impulse: f32,
}

/// A component only present in players that contains the [`GameProfile`] (which
/// you can use to get a player's name).
///
/// Note that it's possible for this to be missing in a player if the server
/// never sent the player info for them (though this is uncommon).
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct GameProfileComponent(pub GameProfile);

/// Marks a [`LocalEntity`] that's in a loaded chunk. This is updated at the
/// beginning of every tick.
#[derive(Component, Clone, Debug, Copy)]
pub struct LocalEntityInLoadedChunk;

/// The gamemode of a local player. For a non-local player, you can look up the
/// player in the [`TabList`].
#[derive(Component, Clone, Debug, Copy)]
pub struct LocalGameMode {
    pub current: GameMode,
    pub previous: Option<GameMode>,
}

/// A component that contains some of the "settings" for this client that are
/// sent to the server, such as render distance. This is only present on local
/// players.
pub type ClientInformation = ServerboundClientInformationPacket;

/// A component that contains the abilities the player has, like flying
/// or instantly breaking blocks. This is only present on local players.
#[derive(Clone, Debug, Component, Default)]
pub struct PlayerAbilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub can_fly: bool,
    /// Whether the player can instantly break blocks and can duplicate blocks
    /// in their inventory.
    pub instant_break: bool,

    pub flying_speed: f32,
    /// Used for the fov
    pub walking_speed: f32,
}
impl From<ClientboundPlayerAbilitiesPacket> for PlayerAbilities {
    fn from(packet: ClientboundPlayerAbilitiesPacket) -> Self {
        Self {
            invulnerable: packet.flags.invulnerable,
            flying: packet.flags.flying,
            can_fly: packet.flags.can_fly,
            instant_break: packet.flags.instant_break,
            flying_speed: packet.flying_speed,
            walking_speed: packet.walking_speed,
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
/// # use azalea_client::TabList;
/// # fn example(client: &azalea_client::Client) {
/// let tab_list = client.component::<TabList>();
/// println!("Online players:");
/// for (uuid, player_info) in tab_list.iter() {
///     println!("- {} ({}ms)", player_info.profile.name, player_info.latency);
/// }
/// # }
#[derive(Component, Clone, Debug, Deref, DerefMut, Default)]
pub struct TabList(HashMap<Uuid, PlayerInfo>);

impl InstanceHolder {
    /// Create a new `InstanceHolder`.
    pub fn new(entity: Entity, world: Arc<RwLock<Instance>>) -> Self {
        let client_information = ClientInformation::default();

        InstanceHolder {
            instance: world,
            partial_instance: Arc::new(RwLock::new(PartialInstance::new(
                azalea_world::calculate_chunk_storage_range(
                    client_information.view_distance.into(),
                ),
                Some(entity),
            ))),
        }
    }
}

/// Update the [`LocalEntityInLoadedChunk`] component for all [`LocalEntity`]s.
pub fn update_in_loaded_chunk(
    mut commands: bevy_ecs::system::Commands,
    query: Query<(Entity, &InstanceName, &Position)>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, local_player, position) in &query {
        let player_chunk_pos = ChunkPos::from(position);
        let Some(instance_lock) = instance_container.get(local_player) else {
            continue;
        };

        let in_loaded_chunk = instance_lock.read().chunks.get(&player_chunk_pos).is_some();
        if in_loaded_chunk {
            commands.entity(entity).insert(LocalEntityInLoadedChunk);
        } else {
            commands.entity(entity).remove::<LocalEntityInLoadedChunk>();
        }
    }
}

/// Send the "Death" event for [`LocalEntity`]s that died with no reason.
pub fn death_event(query: Query<&LocalPlayerEvents, Added<Dead>>) {
    for local_player_events in &query {
        local_player_events.send(AzaleaEvent::Death(None)).unwrap();
    }
}

#[derive(Error, Debug)]
pub enum HandlePacketError {
    #[error("{0}")]
    Poison(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Send(#[from] mpsc::error::SendError<AzaleaEvent>),
}

impl<T> From<std::sync::PoisonError<T>> for HandlePacketError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandlePacketError::Poison(e.to_string())
    }
}

/// Event for sending a packet to the server.
#[derive(Event)]
pub struct SendPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundGamePacket,
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendPacketEvent>,
    mut query: Query<&mut RawConnection>,
) {
    for event in send_packet_events.iter() {
        if let Ok(raw_connection) = query.get_mut(event.entity) {
            // debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_connection.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}
