use std::io::Cursor;

use azalea_entity::indexing::EntityIdIndex;
use azalea_protocol::packets::config::s_finish_configuration::ServerboundFinishConfiguration;
use azalea_protocol::packets::config::s_keep_alive::ServerboundKeepAlive;
use azalea_protocol::packets::config::s_select_known_packs::ServerboundSelectKnownPacks;
use azalea_protocol::packets::config::{
    self, ClientboundConfigPacket, ServerboundConfigPacket, ServerboundCookieResponse,
    ServerboundResourcePack,
};
use azalea_protocol::packets::{ConnectionProtocol, Packet};
use azalea_protocol::read::deserialize_packet;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use tracing::{debug, error, warn};

use crate::client::InConfigurationState;
use crate::disconnect::DisconnectEvent;
use crate::local_player::Hunger;
use crate::packet_handling::game::KeepAliveEvent;
use crate::raw_connection::RawConnection;
use crate::InstanceHolder;

#[derive(Event, Debug, Clone)]
pub struct ConfigurationEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigPacket,
}

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<InConfigurationState>>,
    mut packet_events: ResMut<Events<ConfigurationEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_conn) in &query {
        let packets_lock = raw_conn.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            for raw_packet in packets.iter() {
                let packet = match deserialize_packet::<ClientboundConfigPacket>(&mut Cursor::new(
                    raw_packet,
                )) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!("failed to read packet: {:?}", err);
                        debug!("packet bytes: {:?}", raw_packet);
                        continue;
                    }
                };
                packet_events.send(ConfigurationEvent {
                    entity: player_entity,
                    packet,
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<ConfigurationEvent>> = SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for ConfigurationEvent {
        entity: player_entity,
        packet,
    } in events.read()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        match packet {
            ClientboundConfigPacket::RegistryData(p) => {
                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let instance_holder = query.get_mut(player_entity).unwrap();
                let mut instance = instance_holder.instance.write();

                // add the new registry data
                instance.registries.append(p.registry_id, p.entries);
            }

            ClientboundConfigPacket::CustomPayload(p) => {
                debug!("Got custom payload packet {p:?}");
            }
            ClientboundConfigPacket::Disconnect(p) => {
                warn!("Got disconnect packet {p:?}");
                let mut system_state: SystemState<EventWriter<DisconnectEvent>> =
                    SystemState::new(ecs);
                let mut disconnect_events = system_state.get_mut(ecs);
                disconnect_events.send(DisconnectEvent {
                    entity: player_entity,
                    reason: Some(p.reason.clone()),
                });
            }
            ClientboundConfigPacket::FinishConfiguration(p) => {
                debug!("got FinishConfiguration packet: {p:?}");

                let mut system_state: SystemState<Query<&mut RawConnection>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut raw_conn = query.get_mut(player_entity).unwrap();

                raw_conn
                    .write_packet(ServerboundFinishConfiguration {})
                    .expect(
                        "we should be in the right state and encoding this packet shouldn't fail",
                    );
                raw_conn.set_state(ConnectionProtocol::Game);

                // these components are added now that we're going to be in the Game state
                ecs.entity_mut(player_entity)
                    .remove::<InConfigurationState>()
                    .insert(crate::JoinedClientBundle {
                        physics_state: crate::PhysicsState::default(),
                        inventory: crate::inventory::Inventory::default(),
                        tab_list: crate::local_player::TabList::default(),
                        current_sequence_number: crate::interact::CurrentSequenceNumber::default(),
                        last_sent_direction: crate::movement::LastSentLookDirection::default(),
                        abilities: crate::local_player::PlayerAbilities::default(),
                        permission_level: crate::local_player::PermissionLevel::default(),
                        hunger: Hunger::default(),
                        chunk_batch_info: crate::chunks::ChunkBatchInfo::default(),

                        entity_id_index: EntityIdIndex::default(),

                        mining: crate::mining::MineBundle::default(),
                        attack: crate::attack::AttackBundle::default(),

                        _local_entity: azalea_entity::LocalEntity,
                    });
            }
            ClientboundConfigPacket::KeepAlive(p) => {
                debug!("Got keep alive packet (in configuration) {p:?} for {player_entity:?}");

                let mut system_state: SystemState<(
                    Query<&RawConnection>,
                    EventWriter<KeepAliveEvent>,
                )> = SystemState::new(ecs);
                let (query, mut keepalive_events) = system_state.get_mut(ecs);
                let raw_conn = query.get(player_entity).unwrap();

                keepalive_events.send(KeepAliveEvent {
                    entity: player_entity,
                    id: p.id,
                });
                raw_conn
                    .write_packet(ServerboundKeepAlive { id: p.id })
                    .unwrap();
            }
            ClientboundConfigPacket::Ping(p) => {
                debug!("Got ping packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_conn = query.get_mut(player_entity).unwrap();

                raw_conn
                    .write_packet(config::s_pong::ServerboundPong { id: p.id })
                    .unwrap();
            }
            ClientboundConfigPacket::ResourcePackPush(p) => {
                debug!("Got resource pack packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_conn = query.get_mut(player_entity).unwrap();

                // always accept resource pack
                raw_conn
                    .write_packet(ServerboundResourcePack {
                        id: p.id,
                        action: config::s_resource_pack::Action::Accepted,
                    })
                    .unwrap();
            }
            ClientboundConfigPacket::ResourcePackPop(_) => {
                // we can ignore this
            }
            ClientboundConfigPacket::UpdateEnabledFeatures(p) => {
                debug!("Got update enabled features packet {p:?}");
            }
            ClientboundConfigPacket::UpdateTags(_p) => {
                debug!("Got update tags packet");
            }
            ClientboundConfigPacket::CookieRequest(p) => {
                debug!("Got cookie request packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_conn = query.get_mut(player_entity).unwrap();

                raw_conn
                    .write_packet(ServerboundCookieResponse {
                        key: p.key,
                        // cookies aren't implemented
                        payload: None,
                    })
                    .unwrap();
            }
            ClientboundConfigPacket::ResetChat(p) => {
                debug!("Got reset chat packet {p:?}");
            }
            ClientboundConfigPacket::StoreCookie(p) => {
                debug!("Got store cookie packet {p:?}");
            }
            ClientboundConfigPacket::Transfer(p) => {
                debug!("Got transfer packet {p:?}");
            }
            ClientboundConfigPacket::SelectKnownPacks(p) => {
                debug!("Got select known packs packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_conn = query.get_mut(player_entity).unwrap();

                // resource pack management isn't implemented
                raw_conn
                    .write_packet(ServerboundSelectKnownPacks {
                        known_packs: vec![],
                    })
                    .unwrap();
            }
            ClientboundConfigPacket::ServerLinks(_) => {}
            ClientboundConfigPacket::CustomReportDetails(_) => {}
        }
    }
}

/// An event for sending a packet to the server while we're in the
/// `configuration` state.
#[derive(Event)]
pub struct SendConfigurationEvent {
    pub sent_by: Entity,
    pub packet: ServerboundConfigPacket,
}
impl SendConfigurationEvent {
    pub fn new(sent_by: Entity, packet: impl Packet<ServerboundConfigPacket>) -> Self {
        let packet = packet.into_variant();
        Self { sent_by, packet }
    }
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendConfigurationEvent>,
    mut query: Query<(&mut RawConnection, Option<&InConfigurationState>)>,
) {
    for event in send_packet_events.read() {
        if let Ok((raw_conn, in_configuration_state)) = query.get_mut(event.sent_by) {
            if in_configuration_state.is_none() {
                error!(
                    "Tried to send a configuration packet {:?} while not in configuration state",
                    event.packet
                );
                continue;
            }
            debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_conn.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}
