use std::{
    io::Cursor,
    sync::{Arc, Weak},
};

use azalea_chat::FormattedText;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::{
    packets::{
        Packet,
        game::{ClientboundGamePacket, ClientboundPlayerCombatKill, ServerboundGamePacket},
    },
    read::deserialize_packet,
};
use azalea_world::Instance;
use bevy_ecs::prelude::*;
use parking_lot::RwLock;
use tracing::{debug, error};
use uuid::Uuid;

use crate::{PlayerInfo, client::InGameState, raw_connection::RawConnection};

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
#[derive(Event, Clone, Debug)]
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

pub fn handle_outgoing_packets_observer(
    trigger: Trigger<SendPacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InGameState>)>,
) {
    let event = trigger.event();

    if let Ok((raw_connection, in_game_state)) = query.get_mut(event.sent_by) {
        if in_game_state.is_none() {
            error!(
                "Tried to send a game packet {:?} while not in game state",
                event.packet
            );
            return;
        }

        // debug!("Sending packet: {:?}", event.packet);
        if let Err(e) = raw_connection.write_packet(event.packet.clone()) {
            error!("Failed to send packet: {e}");
        }
    }
}

/// A system that converts [`SendPacketEvent`] events into triggers so they get
/// received by [`handle_outgoing_packets_observer`].
pub fn handle_outgoing_packets(mut commands: Commands, mut events: EventReader<SendPacketEvent>) {
    for event in events.read() {
        commands.trigger(event.clone());
    }
}

pub fn emit_receive_packet_events(
    query: Query<(Entity, &RawConnection), With<InGameState>>,
    mut packet_events: ResMut<Events<ReceivePacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_connection) in &query {
        let packets_lock = raw_connection.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            let mut packets_read = 0;
            for raw_packet in packets.iter() {
                packets_read += 1;
                let packet =
                    match deserialize_packet::<ClientboundGamePacket>(&mut Cursor::new(raw_packet))
                    {
                        Ok(packet) => packet,
                        Err(err) => {
                            error!("failed to read packet: {err:?}");
                            debug!("packet bytes: {raw_packet:?}");
                            continue;
                        }
                    };

                let should_interrupt = packet_interrupts(&packet);

                packet_events.send(ReceivePacketEvent {
                    entity: player_entity,
                    packet: Arc::new(packet),
                });

                if should_interrupt {
                    break;
                }
            }
            packets.drain(0..packets_read);
        }
    }
}

/// Whether the given packet should make us stop deserializing the received
/// packets until next update.
///
/// This is used for packets that can switch the client state.
fn packet_interrupts(packet: &ClientboundGamePacket) -> bool {
    matches!(
        packet,
        ClientboundGamePacket::StartConfiguration(_)
            | ClientboundGamePacket::Disconnect(_)
            | ClientboundGamePacket::Transfer(_)
    )
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

/// A Bevy trigger that's sent when our client receives a [`ClientboundPing`]
/// packet in the game state.
///
/// Also see [`ConfigPingEvent`] which is used for the config state.
///
/// This is not an event and can't be listened to from a normal system,
///so  `EventReader<PingEvent>` will not work.
///
/// To use it, add your "system" with `add_observer` instead of `add_systems`
/// and use `Trigger<PingEvent>` instead of `EventReader`.
///
/// The client Entity that received the packet will be attached to the trigger.
///
/// [`ClientboundPing`]: azalea_protocol::packets::game::ClientboundPing
/// [`ConfigPingEvent`]: crate::packet::config::ConfigPingEvent
#[derive(Event, Debug, Clone)]
pub struct PingEvent(pub azalea_protocol::packets::game::ClientboundPing);
