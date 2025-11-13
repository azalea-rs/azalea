use std::sync::{Arc, Weak};

use azalea_chat::FormattedText;
use azalea_core::identifier::Identifier;
use azalea_protocol::packets::{
    Packet,
    game::{ClientboundGamePacket, ClientboundPlayerCombatKill, ServerboundGamePacket},
};
use azalea_world::Instance;
use bevy_ecs::prelude::*;
use parking_lot::RwLock;
use tracing::{error, trace};
use uuid::Uuid;

use crate::{client::InGameState, connection::RawConnection, player::PlayerInfo};

/// An event that's sent when we receive a packet.
/// ```
/// # use azalea_client::packet::game::ReceiveGamePacketEvent;
/// # use azalea_protocol::packets::game::ClientboundGamePacket;
/// # use bevy_ecs::message::MessageReader;
///
/// fn handle_packets(mut events: MessageReader<ReceiveGamePacketEvent>) {
///     for ReceiveGamePacketEvent { entity, packet } in events.read() {
///         match packet.as_ref() {
///             ClientboundGamePacket::LevelParticles(p) => {
///                 // ...
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Message, Debug, Clone)]
pub struct ReceiveGamePacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundGamePacket>,
}

/// An event for sending a packet to the server while we're in the `game` state.
#[derive(EntityEvent, Clone, Debug)]
pub struct SendGamePacketEvent {
    #[event_target]
    pub sent_by: Entity,
    pub packet: ServerboundGamePacket,
}
impl SendGamePacketEvent {
    pub fn new(sent_by: Entity, packet: impl Packet<ServerboundGamePacket>) -> Self {
        let packet = packet.into_variant();
        Self { sent_by, packet }
    }
}

pub fn handle_outgoing_packets_observer(
    trigger: On<SendGamePacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InGameState>)>,
) {
    let event = trigger.event();

    if let Ok((mut raw_connection, in_game_state)) = query.get_mut(event.sent_by) {
        if in_game_state.is_none() {
            error!(
                "Tried to send a game packet {:?} while not in game state",
                event.packet
            );
            return;
        }

        trace!("Sending game packet: {:?}", event.packet);
        if let Err(e) = raw_connection.write(event.packet.clone()) {
            error!("Failed to send packet: {e}");
        }
    } else {
        trace!("Not sending game packet: {:?}", event.packet);
    }
}

/// A player joined the game (or more specifically, was added to the tab
/// list of a local player).
#[derive(Message, Debug, Clone)]
pub struct AddPlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player left the game (or maybe is still in the game and was just
/// removed from the tab list of a local player).
#[derive(Message, Debug, Clone)]
pub struct RemovePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player was updated in the tab list of a local player (gamemode, display
/// name, or latency changed).
#[derive(Message, Debug, Clone)]
pub struct UpdatePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}

/// Event for when an entity dies.
///
/// If it's a local player and there's a reason in the death screen, the
/// [`ClientboundPlayerCombatKill`] will be included.
#[derive(Message, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub packet: Option<ClientboundPlayerCombatKill>,
}

/// A KeepAlive packet is sent from the server to verify that the client is
/// still connected.
#[derive(Message, Debug, Clone)]
pub struct KeepAliveEvent {
    pub entity: Entity,
    /// The ID of the keepalive.
    ///
    /// This is an arbitrary number, but vanilla servers use the current time to
    /// generate this.
    pub id: u64,
}

#[derive(Message, Debug, Clone)]
pub struct ResourcePackEvent {
    pub entity: Entity,
    /// The random ID for this request to download the resource pack.
    ///
    /// The packet for replying to a resource pack push must contain the same
    /// ID.
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
#[derive(Message, Debug, Clone)]
pub struct InstanceLoadedEvent {
    pub entity: Entity,
    pub name: Identifier,
    pub instance: Weak<RwLock<Instance>>,
}

/// A Bevy trigger that's sent when our client receives a [`ClientboundPing`]
/// packet in the game state.
///
/// Also see [`ConfigPingEvent`] which is used for the config state.
///
/// [`ClientboundPing`]: azalea_protocol::packets::game::ClientboundPing
/// [`ConfigPingEvent`]: crate::packet::config::ConfigPingEvent
#[derive(EntityEvent, Debug, Clone)]
pub struct GamePingEvent {
    pub entity: Entity,
    pub packet: azalea_protocol::packets::game::ClientboundPing,
}
