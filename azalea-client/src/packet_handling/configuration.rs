use std::sync::Arc;

use azalea_protocol::connect::{Connection, ReadConnection, WriteConnection};
use azalea_protocol::packets::configuration::{
    ClientboundConfigurationPacket, ServerboundConfigurationPacket,
};
use azalea_protocol::read::ReadPacketError;
use azalea_world::Instance;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use log::error;
use parking_lot::{Mutex, RwLock};
use tokio::sync::mpsc;

use crate::configuration::ConfigurationLocalPlayer;
use crate::ReceivedRegistries;

#[derive(Event, Debug, Clone)]
pub struct PacketEvent {
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
    mut packet_events: ResMut<Events<PacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, packet_receiver) in &query {
        let mut packets = packet_receiver.packets.lock();
        if !packets.is_empty() {
            for packet in packets.iter() {
                packet_events.send(PacketEvent {
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
    let mut system_state: SystemState<EventReader<PacketEvent>> = SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for PacketEvent {
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

            ClientboundConfigurationPacket::CustomPayload(_) => {}
            ClientboundConfigurationPacket::Disconnect(_) => {}
            ClientboundConfigurationPacket::FinishConfiguration(p) => {
                println!("got FinishConfiguration packet: {p:?}");

                let mut system_state: SystemState<
                    Query<(&mut ConfigurationLocalPlayer, &PacketReceiver)>,
                > = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let (configuring_local_player, configuration_packet_receiver) =
                    query.get_mut(player_entity).unwrap();

                // abort the read/write packets tasks from ConfiguringLocalPlayer and then get
                // the Connection halves
                configuring_local_player.read_packets_task.abort();
                configuring_local_player.write_packets_task.abort();
                let (read_conn, write_conn) = (
                    configuring_local_player
                        .read_conn
                        .try_lock()
                        .expect("the tasks were aborted, so the lock should be free")
                        .take()
                        .unwrap(),
                    configuring_local_player
                        .write_conn
                        .try_lock()
                        .expect("the tasks were aborted, so the lock should be free")
                        .take()
                        .unwrap(),
                );
                let connection = Connection {
                    reader: read_conn,
                    writer: write_conn,
                };
                // transition from the Configuration state to Game
                let connection = connection.game();
                let (read_conn, write_conn) = connection.into_split();
                let (read_conn, write_conn) = (
                    Arc::new(tokio::sync::Mutex::new(Some(read_conn))),
                    Arc::new(tokio::sync::Mutex::new(Some(write_conn))),
                );

                let (packet_writer_sender, packet_writer_receiver) = mpsc::unbounded_channel();
                // start receiving packets
                let game_packet_receiver = super::game::PacketReceiver {
                    packets: Arc::new(Mutex::new(Vec::new())),
                    run_schedule_sender: configuration_packet_receiver.run_schedule_sender.clone(),
                };

                let read_packets_task =
                    tokio::spawn(game_packet_receiver.clone().read_task(read_conn));
                let write_packets_task = tokio::spawn(
                    game_packet_receiver
                        .clone()
                        .write_task(write_conn, packet_writer_receiver),
                );

                let local_player = crate::local_player::LocalPlayer::new(
                    player_entity,
                    packet_writer_sender,
                    // default to an empty world, it'll be set correctly later when we
                    // get the login packet
                    Arc::new(RwLock::new(Instance::default())),
                    read_packets_task,
                    write_packets_task,
                );

                // these components are added now that we're going to be in the Game state
                ecs.entity_mut(player_entity)
                    .remove::<crate::client::ConfigurationClientBundle>()
                    .insert(crate::JoinedClientBundle {
                        local_player,
                        packet_receiver: game_packet_receiver,
                        physics_state: crate::local_player::PhysicsState::default(),
                        inventory: crate::inventory::InventoryComponent::default(),
                        client_information: crate::ClientInformation::default(),
                        tab_list: crate::TabList::default(),
                        current_sequence_number: crate::interact::CurrentSequenceNumber::default(),
                        last_sent_direction: crate::movement::LastSentLookDirection::default(),
                        abilities: crate::local_player::PlayerAbilities::default(),
                        permission_level: crate::local_player::PermissionLevel::default(),
                        mining: crate::mining::MineBundle::default(),
                        attack: crate::attack::AttackBundle::default(),
                        chunk_batch_info: crate::chunk_batching::ChunkBatchInfo::default(),
                        _local: azalea_entity::Local,
                    });
            }
            ClientboundConfigurationPacket::KeepAlive(_) => {}
            ClientboundConfigurationPacket::Ping(_) => {}
            ClientboundConfigurationPacket::ResourcePack(_) => {}
            ClientboundConfigurationPacket::UpdateEnabledFeatures(_) => {}
            ClientboundConfigurationPacket::UpdateTags(_) => {}
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
        read_conn: Arc<tokio::sync::Mutex<Option<ReadConnection<ClientboundConfigurationPacket>>>>,
    ) {
        loop {
            match read_conn.try_lock().unwrap().as_mut().unwrap().read().await {
                Ok(packet) => {
                    println!("received packet: {packet:?}");
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
        write_conn: Arc<
            tokio::sync::Mutex<Option<WriteConnection<ServerboundConfigurationPacket>>>,
        >,
        mut write_receiver: mpsc::UnboundedReceiver<ServerboundConfigurationPacket>,
    ) {
        while let Some(packet) = write_receiver.recv().await {
            if let Err(err) = write_conn
                .try_lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .write(packet)
                .await
            {
                error!("Disconnecting because we couldn't write a packet: {err}.");
                break;
            };
        }
        // receiver is automatically closed when it's dropped
    }
}
