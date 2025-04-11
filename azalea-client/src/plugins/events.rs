//! Defines the [`Event`] enum and makes those events trigger when they're sent
//! in the ECS.

use std::sync::Arc;

use azalea_chat::FormattedText;
use azalea_core::tick::GameTick;
use azalea_entity::{Dead, InLoadedChunk};
use azalea_protocol::packets::game::{
    ClientboundGamePacket, c_player_combat_kill::ClientboundPlayerCombatKill,
};
use azalea_world::{InstanceName, MinecraftEntityId};
use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    query::{Added, With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query},
};
use derive_more::{Deref, DerefMut};
use tokio::sync::mpsc;

use crate::{
    PlayerInfo,
    chat::{ChatPacket, ChatReceivedEvent},
    disconnect::DisconnectEvent,
    packet::game::{
        AddPlayerEvent, DeathEvent, KeepAliveEvent, ReceiveGamePacketEvent, RemovePlayerEvent,
        UpdatePlayerEvent,
    },
};

// (for contributors):
// HOW TO ADD A NEW (packet based) EVENT:
// - Add it as an ECS event first:
//     - Make a struct that contains an entity field and some data fields (look
//       in packet/game/events.rs for examples. These structs should always have
//       their names end with "Event".
//         - (the `entity` field is the local player entity that's receiving the
//           event)
//     - In the GamePacketHandler, you always have a `player` field that you can
//       use.
//     - Add the event struct in PacketPlugin::build
//         - (in the `impl Plugin for PacketPlugin`)
//     - To get the event writer, you have to get an EventWriter<ThingEvent>.
//       Look at other packets in packet/game/mod.rs for examples.
//
// At this point, you've created a new ECS event. That's annoying for bots to
// use though, so you might wanna add it to the Event enum too:
//     - In this file, add a new variant to that Event enum with the same name
//       as your event (without the "Event" suffix).
//     - Create a new system function like the other ones here, and put that
//       system function in the `impl Plugin for EventsPlugin`

/// Something that happened in-game, such as a tick passing or chat message
/// being sent.
///
/// Note: Events are sent before they're processed, so for example game ticks
/// happen at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    ///
    /// You may want to use [`Event::Spawn`] instead to wait for the bot to be
    /// in the world.
    Init,
    /// Fired when we receive a login packet, which is after [`Event::Init`] but
    /// before [`Event::Spawn`]. You usually want [`Event::Spawn`] instead.
    ///
    /// Your position may be [`Vec3::ZERO`] immediately after you receive this
    /// event, but it'll be ready by the time you get [`Event::Spawn`].
    ///
    /// It's possible for this event to be sent multiple times per client if a
    /// server sends multiple login packets (like when switching worlds).
    ///
    /// [`Vec3::ZERO`]: azalea_core::position::Vec3::ZERO
    Login,
    /// Fired when the player fully spawns into the world (is in a loaded chunk)
    /// and is ready to interact with it.
    ///
    /// This is usually the event you should listen for when waiting for the bot
    /// to be ready.
    ///
    /// This event will be sent every time the client respawns or switches
    /// worlds, as long as the server sends chunks to the client.
    Spawn,
    /// A chat message was sent in the game chat.
    Chat(ChatPacket),
    /// Happens 20 times per second, but only when the world is loaded.
    Tick,
    /// We received a packet from the server.
    ///
    /// ```
    /// # use azalea_client::Event;
    /// # use azalea_protocol::packets::game::ClientboundGamePacket;
    /// # async fn example(event: Event) {
    /// # match event {
    /// Event::Packet(packet) => match *packet {
    ///     ClientboundGamePacket::Login(_) => {
    ///         println!("login packet");
    ///     }
    ///     _ => {}
    /// },
    /// # _ => {}
    /// # }
    /// # }
    /// ```
    Packet(Arc<ClientboundGamePacket>),
    /// A player joined the game (or more specifically, was added to the tab
    /// list).
    AddPlayer(PlayerInfo),
    /// A player left the game (or maybe is still in the game and was just
    /// removed from the tab list).
    RemovePlayer(PlayerInfo),
    /// A player was updated in the tab list (gamemode, display
    /// name, or latency changed).
    UpdatePlayer(PlayerInfo),
    /// The client player died in-game.
    Death(Option<Arc<ClientboundPlayerCombatKill>>),
    /// A `KeepAlive` packet was sent by the server.
    KeepAlive(u64),
    /// The client disconnected from the server.
    Disconnect(Option<FormattedText>),
}

/// A component that contains an event sender for events that are only
/// received by local players. The receiver for this is returned by
/// [`Client::start_client`].
///
/// [`Client::start_client`]: crate::Client::start_client
#[derive(Component, Deref, DerefMut)]
pub struct LocalPlayerEvents(pub mpsc::UnboundedSender<Event>);

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                chat_listener,
                login_listener,
                spawn_listener,
                packet_listener,
                add_player_listener,
                update_player_listener,
                remove_player_listener,
                keepalive_listener,
                death_listener,
                disconnect_listener,
            ),
        )
        .add_systems(
            PreUpdate,
            init_listener.before(super::connection::read_packets),
        )
        .add_systems(GameTick, tick_listener);
    }
}

// when LocalPlayerEvents is added, it means the client just started
pub fn init_listener(query: Query<&LocalPlayerEvents, Added<LocalPlayerEvents>>) {
    for local_player_events in &query {
        let _ = local_player_events.send(Event::Init);
    }
}

// when MinecraftEntityId is added, it means the player is now in the world
pub fn login_listener(
    query: Query<(Entity, &LocalPlayerEvents), Added<MinecraftEntityId>>,
    mut commands: Commands,
) {
    for (entity, local_player_events) in &query {
        let _ = local_player_events.send(Event::Login);
        commands.entity(entity).remove::<SentSpawnEvent>();
    }
}

/// A unit struct component that indicates that the entity has sent
/// [`Event::Spawn`].
///
/// This is just used internally by the [`spawn_listener`] system to avoid
/// sending the event twice if we stop being in an unloaded chunk. It's removed
/// when we receive a login packet.
#[derive(Component)]
pub struct SentSpawnEvent;
#[allow(clippy::type_complexity)]
pub fn spawn_listener(
    query: Query<(Entity, &LocalPlayerEvents), (Added<InLoadedChunk>, Without<SentSpawnEvent>)>,
    mut commands: Commands,
) {
    for (entity, local_player_events) in &query {
        let _ = local_player_events.send(Event::Spawn);
        commands.entity(entity).insert(SentSpawnEvent);
    }
}

pub fn chat_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<ChatReceivedEvent>) {
    for event in events.read() {
        if let Ok(local_player_events) = query.get(event.entity) {
            let _ = local_player_events.send(Event::Chat(event.packet.clone()));
        }
    }
}

// only tick if we're in a world
pub fn tick_listener(query: Query<&LocalPlayerEvents, With<InstanceName>>) {
    for local_player_events in &query {
        let _ = local_player_events.send(Event::Tick);
    }
}

pub fn packet_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<ReceiveGamePacketEvent>,
) {
    for event in events.read() {
        if let Ok(local_player_events) = query.get(event.entity) {
            let _ = local_player_events.send(Event::Packet(event.packet.clone()));
        }
    }
}

pub fn add_player_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<AddPlayerEvent>,
) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive add player events");
        let _ = local_player_events.send(Event::AddPlayer(event.info.clone()));
    }
}

pub fn update_player_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<UpdatePlayerEvent>,
) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive update player events");
        let _ = local_player_events.send(Event::UpdatePlayer(event.info.clone()));
    }
}

pub fn remove_player_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<RemovePlayerEvent>,
) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive remove player events");
        let _ = local_player_events.send(Event::RemovePlayer(event.info.clone()));
    }
}

pub fn death_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<DeathEvent>) {
    for event in events.read() {
        if let Ok(local_player_events) = query.get(event.entity) {
            let _ = local_player_events.send(Event::Death(event.packet.clone().map(|p| p.into())));
        }
    }
}

/// Send the "Death" event for [`LocalEntity`]s that died with no reason.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
pub fn dead_component_listener(query: Query<&LocalPlayerEvents, Added<Dead>>) {
    for local_player_events in &query {
        local_player_events.send(Event::Death(None)).unwrap();
    }
}

pub fn keepalive_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<KeepAliveEvent>,
) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive keepalive events");
        let _ = local_player_events.send(Event::KeepAlive(event.id));
    }
}

pub fn disconnect_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<DisconnectEvent>,
) {
    for event in events.read() {
        if let Ok(local_player_events) = query.get(event.entity) {
            let _ = local_player_events.send(Event::Disconnect(event.reason.clone()));
        }
    }
}
