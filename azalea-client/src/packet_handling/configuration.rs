use std::sync::Arc;

use azalea_protocol::connect::{ReadConnection, WriteConnection};
use azalea_protocol::packets::configuration::{
    ClientboundConfigurationPacket, ServerboundConfigurationPacket,
};
use azalea_protocol::read::ReadPacketError;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use log::error;
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::configuration::ConfiguringLocalPlayer;
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
pub struct PacketReceiver {
    pub packets: Arc<Mutex<Vec<ClientboundConfigurationPacket>>>,
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

pub fn send_packet_events(
    query: Query<(Entity, &PacketReceiver)>,
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
            ClientboundConfigurationPacket::FinishConfiguration(p) => {
                let mut system_state: SystemState<Query<&mut ConfiguringLocalPlayer>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let configuring_local_player = query.get_mut(player_entity).unwrap();

                // abort the read/write packets tasks from ConfiguringLocalPlayer and then get
                // the Connection halves
                configuring_local_player.read_packets_task.abort();
                configuring_local_player.write_packets_task.abort();

                let local_player = crate::local_player::LocalPlayer::new(
                    player_entity,
                    packet_writer_sender,
                    // default to an empty world, it'll be set correctly later when we
                    // get the login packet
                    Arc::new(RwLock::new(Instance::default())),
                    read_packets_task,
                    write_packets_task,
                );

                ecs.entity_mut(entity).insert(JoinedClientBundle {
                    local_player,
                    packet_receiver,
                    game_profile: GameProfileComponent(game_profile),
                    physics_state: PhysicsState::default(),
                    inventory: InventoryComponent::default(),
                    client_information: ClientInformation::default(),
                    tab_list: TabList::default(),
                    current_sequence_number: CurrentSequenceNumber::default(),
                    last_sent_direction: LastSentLookDirection::default(),
                    abilities: PlayerAbilities::default(),
                    permission_level: PermissionLevel::default(),
                    mining: mining::MineBundle::default(),
                    attack: attack::AttackBundle::default(),
                    chunk_batch_info: chunk_batching::ChunkBatchInfo::default(),
                    _local: Local,
                });
            }
            ClientboundConfigurationPacket::KeepAlive(_) => todo!(),
            ClientboundConfigurationPacket::Ping(_) => todo!(),
            ClientboundConfigurationPacket::ResourcePack(_) => todo!(),
            ClientboundConfigurationPacket::UpdateEnabledFeatures(_) => todo!(),
            ClientboundConfigurationPacket::UpdateTags(_) => todo!(),
        }
    }
}

impl PacketReceiver {
    /// Loop that reads from the connection and adds the packets to the queue +
    /// runs the schedule.
    pub async fn read_task(
        self,
        // this is a mutex because we need to recover the ReadConnection when this task gets
        // aborted (because we're switching to the game state)
        read_conn: Arc<tokio::sync::Mutex<ReadConnection<ClientboundConfigurationPacket>>>,
    ) {
        loop {
            match read_conn.try_lock().unwrap().read().await {
                Ok(packet) => {
                    self.packets.lock().push(packet);
                    // tell the client to run all the systems
                    self.run_schedule_sender.send(()).unwrap();
                }
                Err(error) => {
                    if !matches!(*error, ReadPacketError::ConnectionClosed) {
                        error!("Error reading packet from Client: {error:?}");
                    }
                    break;
                }
            }
        }
    }

    /// Consume the [`ServerboundGamePacket`] queue and actually write the
    /// packets to the server. It's like this so writing packets doesn't need to
    /// be awaited.
    pub async fn write_task(
        self,
        write_conn: Arc<tokio::sync::Mutex<WriteConnection<ServerboundConfigurationPacket>>>,
        mut write_receiver: mpsc::UnboundedReceiver<ServerboundConfigurationPacket>,
    ) {
        while let Some(packet) = write_receiver.recv().await {
            if let Err(err) = write_conn.try_lock().unwrap().write(packet).await {
                error!("Disconnecting because we couldn't write a packet: {err}.");
                break;
            };
        }
        // receiver is automatically closed when it's dropped
    }
}
