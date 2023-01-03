use std::{collections::HashMap, io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, ResourceLocation};
use azalea_protocol::{connect::WriteConnection, packets::game::ServerboundGamePacket};
use azalea_world::{
    entity::{self, Entity},
    EntityInfos, PartialWorld, World,
};
use bevy_ecs::{component::Component, system::Query};
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{ClientInformation, Event, PlayerInfo, WalkDirection};

/// A player that you control that is currently in a Minecraft server.
#[derive(Component)]
pub struct LocalPlayer {
    pub profile: GameProfile,

    pub packet_writer: mpsc::UnboundedSender<ServerboundGamePacket>,

    // pub world: Arc<RwLock<PartialWorld>>,
    pub physics_state: PhysicsState,
    pub client_information: ClientInformation,
    pub dead: bool,
    /// A map of player uuids to their information in the tab list
    pub players: HashMap<Uuid, PlayerInfo>,

    pub partial_world: Arc<RwLock<PartialWorld>>,
    pub world: Arc<RwLock<World>>,
    pub world_name: Option<ResourceLocation>,

    pub tx: mpsc::UnboundedSender<Event>,
}

#[derive(Default)]
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

/// Marks a [`LocalPlayer`] that's in a loaded chunk. This is updated at the
/// beginning of every tick.
#[derive(Component)]
pub struct LocalPlayerInLoadedChunk;

impl LocalPlayer {
    /// Create a new `LocalPlayer`.
    ///
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        entity: Entity,
        profile: GameProfile,
        packet_writer: mpsc::UnboundedSender<ServerboundGamePacket>,
        world: Arc<RwLock<World>>,
        entity_infos: &mut EntityInfos,
        tx: mpsc::UnboundedSender<Event>,
    ) -> Self {
        let client_information = ClientInformation::default();

        LocalPlayer {
            profile,

            packet_writer,

            physics_state: PhysicsState::default(),
            client_information: ClientInformation::default(),
            dead: false,
            players: HashMap::new(),

            world,
            partial_world: Arc::new(RwLock::new(PartialWorld::new(
                client_information.view_distance.into(),
                Some(entity),
                entity_infos,
            ))),
            world_name: None,

            tx,
        }
    }

    /// Spawn a task to write a packet directly to the server.
    pub fn write_packet(&mut self, packet: ServerboundGamePacket) {
        self.packet_writer.send(packet);
    }
}

pub fn send_tick_event(query: Query<&LocalPlayer>) {
    for local_player in &query {
        local_player.tx.send(Event::Tick).unwrap();
    }
}

/// Update the [`LocalPlayerInLoadedChunk`] component for all [`LocalPlayer`]s.
pub fn update_in_loaded_chunk(
    mut commands: bevy_ecs::system::Commands,
    query: Query<(Entity, &LocalPlayer, &entity::Position)>,
) {
    for (ecs_entity_id, local_player, position) in &query {
        let player_chunk_pos = ChunkPos::from(position);
        let in_loaded_chunk = local_player
            .world
            .read()
            .chunks
            .get(&player_chunk_pos)
            .is_some();
        if in_loaded_chunk {
            commands
                .entity(ecs_entity_id)
                .insert(LocalPlayerInLoadedChunk);
        } else {
            commands
                .entity(ecs_entity_id)
                .remove::<LocalPlayerInLoadedChunk>();
        }
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
    Send(#[from] mpsc::error::SendError<Event>),
}

impl<T> From<std::sync::PoisonError<T>> for HandlePacketError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandlePacketError::Poison(e.to_string())
    }
}
