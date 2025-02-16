use std::sync::{Arc, Weak};

use azalea_chat::FormattedText;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::packets::{
    game::{ClientboundGamePacket, ClientboundPlayerCombatKill, ServerboundGamePacket},
    Packet,
};
use azalea_world::Instance;
use bevy_ecs::{entity::Entity, event::Event};
use parking_lot::RwLock;
use uuid::Uuid;

use crate::PlayerInfo;

/// An event that's sent when we receive a packet.
/// ```
/// # use azalea_client::packet::game::ReceivePacketEvent;
/// # use azalea_protocol::packets::game::ClientboundGamePacket;
/// # use bevy_ecs::event::EventReader;
///
/// fn handle_packets(mut events: EventReader<ReceivePacketEvent>) {
///     for ReceivePacketEvent {
///         entity,
///         packet,
///     } in events.read() {
///         match packet.as_ref() {
///             ClientboundGamePacket::LevelParticles(p) => {
///                 // ...
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct ReceivePacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundGamePacket>,
}

/// An event for sending a packet to the server while we're in the `game` state.
#[derive(Event)]
pub struct SendPacketEvent {
    pub sent_by: Entity,
    pub packet: ServerboundGamePacket,
}
impl SendPacketEvent {
    pub fn new(sent_by: Entity, packet: impl Packet<ServerboundGamePacket>) -> Self {
        let packet = packet.into_variant();
        Self { sent_by, packet }
    }
}

/// A player joined the game (or more specifically, was added to the tab
/// list of a local player).
#[derive(Event, Debug, Clone)]
pub struct AddPlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player left the game (or maybe is still in the game and was just
/// removed from the tab list of a local player).
#[derive(Event, Debug, Clone)]
pub struct RemovePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player was updated in the tab list of a local player (gamemode, display
/// name, or latency changed).
#[derive(Event, Debug, Clone)]
pub struct UpdatePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}

/// Event for when an entity dies. dies. If it's a local player and there's a
/// reason in the death screen, the [`ClientboundPlayerCombatKill`] will
/// be included.
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub packet: Option<ClientboundPlayerCombatKill>,
}

/// A KeepAlive packet is sent from the server to verify that the client is
/// still connected.
#[derive(Event, Debug, Clone)]
pub struct KeepAliveEvent {
    pub entity: Entity,
    /// The ID of the keepalive. This is an arbitrary number, but vanilla
    /// servers use the time to generate this.
    pub id: u64,
}

#[derive(Event, Debug, Clone)]
pub struct ResourcePackEvent {
    pub entity: Entity,
    /// The random ID for this request to download the resource pack. The packet
    /// for replying to a resource pack push must contain the same ID.
    pub id: Uuid,
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}

/// An instance (aka world, dimension) was loaded by a client.
///
/// Since the instance is given to you as a weak reference, it won't be able to
/// be `upgrade`d if all local players leave it.
#[derive(Event, Debug, Clone)]
pub struct InstanceLoadedEvent {
    pub entity: Entity,
    pub name: ResourceLocation,
    pub instance: Weak<RwLock<Instance>>,
}
