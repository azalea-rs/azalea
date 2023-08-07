use std::sync::Arc;

use azalea_protocol::packets::configuration::ClientboundConfigurationPacket;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::ReceivedRegistries;

#[derive(Event, Debug, Clone)]
pub struct ConfigurationPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigurationPacket,
}

/// Something that receives packets from the server.
#[derive(Event, Component, Clone)]
pub struct ConfigurationPacketReceiver {
    pub packets: Arc<Mutex<Vec<ClientboundConfigurationPacket>>>,
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

pub fn send_packet_events(
    query: Query<(Entity, &ConfigurationPacketReceiver)>,
    mut packet_events: ResMut<Events<ConfigurationPacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, packet_receiver) in &query {
        let mut packets = packet_receiver.packets.lock();
        if !packets.is_empty() {
            for packet in packets.iter() {
                packet_events.send(ConfigurationPacketEvent {
                    entity: player_entity,
                    packet: packet.clone(),
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<ConfigurationPacketEvent>> =
        SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for ConfigurationPacketEvent {
        entity: player_entity,
        packet,
    } in events.iter()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        match packet {
            ClientboundConfigurationPacket::RegistryData(p) => {
                let mut system_state: SystemState<Query<&mut ReceivedRegistries>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut received_registries = query.get_mut(player_entity).unwrap();

                let new_received_registries = p.registry_holder.registries;
                // override the old registries with the new ones
                // but if a registry wasn't sent, keep the old one
                for (registry_name, registry) in new_received_registries {
                    received_registries
                        .registries
                        .insert(registry_name, registry);
                }
            }

            ClientboundConfigurationPacket::CustomPayload(_) => todo!(),
            ClientboundConfigurationPacket::Disconnect(_) => todo!(),
            ClientboundConfigurationPacket::FinishConfiguration(_) => todo!(),
            ClientboundConfigurationPacket::KeepAlive(_) => todo!(),
            ClientboundConfigurationPacket::Ping(_) => todo!(),
            ClientboundConfigurationPacket::ResourcePack(_) => todo!(),
            ClientboundConfigurationPacket::UpdateEnabledFeatures(_) => todo!(),
            ClientboundConfigurationPacket::UpdateTags(_) => todo!(),
        }
    }
}
