//! Defines the [`Event`] enum and makes those events trigger when they're sent
//! in the ECS.

use std::sync::Arc;

use azalea_chat::FormattedText;
use azalea_core::tick::GameTick;
use azalea_protocol::packets::game::{
    c_player_combat_kill::ClientboundPlayerCombatKill, ClientboundGamePacket,
};
use azalea_world::{InstanceName, MinecraftEntityId};
use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_ecs::{
    component::Component,
    event::EventReader,
    query::{Added, With},
    schedule::IntoSystemConfigs,
    system::Query,
};
use derive_more::{Deref, DerefMut};
use tokio::sync::mpsc;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    disconnect::DisconnectEvent,
    packet_handling::game::{
        AddPlayerEvent, DeathEvent, KeepAliveEvent, PacketEvent, RemovePlayerEvent,
        UpdatePlayerEvent,
    },
    PlayerInfo,
};

// (for contributors):
// HOW TO ADD A NEW (packet based) EVENT:
// - make a struct that contains an entity field and a data field (look in
//   packet_handling.rs for examples, also you should end the struct name with
//   "Event")
// - the entity field is the local player entity that's receiving the event
// - in packet_handling, you always have a variable called player_entity that
//   you can use
// - add the event struct in the `impl Plugin for PacketHandlerPlugin`
// - to get the event writer, you have to get an
//   EventWriter<SomethingHappenedEvent> from the SystemState (the convention is
//   to end your variable with the word "events", like "something_events")
//
// - then here in this file, add it to the Event enum
// - and make an event listener system/function like the other ones and put the
//   function in the `impl Plugin for EventPlugin`

/// Something that happened in-game, such as a tick passing or chat message
/// being sent.
///
/// Note: Events are sent before they're processed, so for example game ticks
/// happen at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    ///
    /// You may want to use [`Event::Login`] instead to wait for the bot to be
    /// in the world.
    Init,
    /// The client is now in the world. Fired when we receive a login packet.
    Login,
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

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                chat_listener,
                login_listener,
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
            init_listener.before(crate::packet_handling::game::process_packet_events),
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
pub fn login_listener(query: Query<&LocalPlayerEvents, Added<MinecraftEntityId>>) {
    for local_player_events in &query {
        let _ = local_player_events.send(Event::Login);
    }
}

pub fn chat_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<ChatReceivedEvent>) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive chat events");
        let _ = local_player_events.send(Event::Chat(event.packet.clone()));
    }
}

// only tick if we're in a world
pub fn tick_listener(query: Query<&LocalPlayerEvents, With<InstanceName>>) {
    for local_player_events in &query {
        let _ = local_player_events.send(Event::Tick);
    }
}

pub fn packet_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<PacketEvent>) {
    for event in events.read() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-local entities shouldn't be able to receive packet events");
        let _ = local_player_events.send(Event::Packet(event.packet.clone()));
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
