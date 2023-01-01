use std::{collections::HashMap, io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{Connection, ReadConnection, WriteConnection},
    packets::game::{
        serverbound_keep_alive_packet::ServerboundKeepAlivePacket, ClientboundGamePacket,
        ServerboundGamePacket,
    },
};
use azalea_world::{
    entity::{self, metadata::PlayerMetadataBundle, EntityId},
    PartialWorld, WeakWorldContainer,
};
use bevy_ecs::{
    component::Component,
    event::EventReader,
    system::{Query, Res, ResMut},
};
use log::debug;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{ChatPacket, ClientInformation, Event, PlayerInfo, WalkDirection};

/// A player that you control that is currently in a Minecraft server.
#[derive(Component)]
pub struct LocalPlayer {
    pub profile: GameProfile,

    pub write_conn: WriteConnection<ServerboundGamePacket>,
    // pub world: Arc<RwLock<PartialWorld>>,
    pub physics_state: PhysicsState,
    pub client_information: ClientInformation,
    pub dead: bool,
    /// A map of player uuids to their information in the tab list
    pub players: HashMap<Uuid, PlayerInfo>,

    pub world: Arc<RwLock<PartialWorld>>,
    pub world_name: Option<ResourceLocation>,

    pub tx: mpsc::Sender<Event>,
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
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        write_conn: WriteConnection<ServerboundGamePacket>,
        world: Arc<RwLock<PartialWorld>>,
        tx: mpsc::Sender<Event>,
    ) -> Self {
        LocalPlayer {
            profile,
            write_conn,
            physics_state: PhysicsState::default(),
            client_information: ClientInformation::default(),
            dead: false,
            players: HashMap::new(),

            world,
            world_name: Arc::new(RwLock::new(None)),

            tx,
            world_name: None,
        }
    }

    /// Write a packet directly to the server.
    pub async fn write_packet_async(
        &mut self,
        packet: ServerboundGamePacket,
    ) -> Result<(), std::io::Error> {
        self.write_conn.write(packet).await?;
        Ok(())
    }

    /// Spawn a task to write a packet directly to the server.
    pub fn write_packet(&mut self, packet: ServerboundGamePacket) {
        tokio::spawn(self.write_packet_async(packet));
    }

    /// Update the [`LocalPlayerInLoadedChunk`] component for all
    /// [`LocalPlayer`]s.
    fn update_in_loaded_chunk(
        mut commands: bevy_ecs::system::Commands,
        query: Query<(entity::EcsEntityId, &LocalPlayer, &entity::Position)>,
    ) {
        for (ecs_entity_id, local_player, position) in &query {
            let player_chunk_pos = ChunkPos::from(position);
            let in_loaded_chunk = local_player
                .world
                .read()
                .get_chunk(&player_chunk_pos)
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

    pub(crate) fn send_event(event: Event, tx: &mpsc::Sender<Event>) {
        tokio::spawn(tx.send(event));
    }

    fn send_tick_event(query: Query<&LocalPlayer>) {
        for local_player in &query {
            let tx = local_player.tx.clone();
            Self::send_event(Event::Tick, &tx);
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
