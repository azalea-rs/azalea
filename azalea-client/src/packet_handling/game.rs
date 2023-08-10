use std::{collections::HashSet, io::Cursor, sync::Arc, time::Instant};

use azalea_buf::McBufWritable;
use azalea_core::{ChunkPos, GameMode, ResourceLocation, Vec3};
use azalea_entity::{
    indexing::EntityUuidIndex,
    metadata::{apply_metadata, Health, PlayerMetadataBundle},
    Dead, EntityBundle, EntityKind, EntityUpdateSet, LastSentPosition, LoadedBy, LocalEntity,
    LookDirection, Physics, PlayerBundle, Position, RelativeEntityUpdate,
};
use azalea_protocol::{
    connect::{ReadConnection, WriteConnection},
    packets::game::{
        clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket,
        serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
        serverbound_chunk_batch_received_packet::ServerboundChunkBatchReceivedPacket,
        serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
        serverbound_pong_packet::ServerboundPongPacket,
        serverbound_resource_pack_packet::ServerboundResourcePackPacket, ClientboundGamePacket,
        ServerboundGamePacket,
    },
    read::{deserialize_packet, ReadPacketError},
};
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_app::{App, First, Plugin, PreUpdate, Update};
use bevy_ecs::{prelude::*, system::SystemState};
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    chunk_batching,
    disconnect::DisconnectEvent,
    inventory::{
        ClientSideCloseContainerEvent, InventoryComponent, MenuOpenedEvent,
        SetContainerContentEvent,
    },
    local_player::{
        GameProfileComponent, InstanceHolder, LocalGameMode, PlayerAbilities, SendPacketEvent,
    },
    raw_connection::RawConnection,
    ClientInformation, PlayerInfo, ReceivedRegistries, TabList,
};

/// An event that's sent when we receive a packet.
/// ```
/// # use azalea_client::packet_handling::PacketEvent;
/// # use azalea_protocol::packets::game::ClientboundGamePacket;
/// # use bevy_ecs::event::EventReader;
///
/// fn handle_packets(mut events: EventReader<PacketEvent>) {
///     for PacketEvent {
///         entity,
///         packet,
///     } in events.iter() {
///         match packet {
///             ClientboundGamePacket::LevelParticles(p) => {
///                 // ...
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct PacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundGamePacket,
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
/// reason in the death screen, the [`ClientboundPlayerCombatKillPacket`] will
/// be included.
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub packet: Option<ClientboundPlayerCombatKillPacket>,
}

pub fn death_event_on_0_health(
    query: Query<(Entity, &Health), Changed<Health>>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if **health == 0. {
            death_events.send(DeathEvent {
                entity,
                packet: None,
            });
        }
    }
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

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<LocalEntity>>,
    mut packet_events: ResMut<Events<PacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_connection) in &query {
        let packets_lock = raw_connection.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            for raw_packet in packets.iter() {
                let packet =
                    match deserialize_packet::<ClientboundGamePacket>(&mut Cursor::new(raw_packet))
                    {
                        Ok(packet) => packet,
                        Err(err) => {
                            error!("failed to read packet: {:?}", err);
                            continue;
                        }
                    };
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
            ClientboundGamePacket::Login(p) => {
                debug!("Got login packet");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(
                        &GameProfileComponent,
                        &ClientInformation,
                        &ReceivedRegistries,
                        Option<&mut InstanceName>,
                        &mut InstanceHolder,
                    )>,
                    ResMut<InstanceContainer>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, mut instance_container, mut send_packet_events) =
                    system_state.get_mut(ecs);
                let (
                    game_profile,
                    client_information,
                    received_registries,
                    instance_name,
                    mut instance_holder,
                ) = query.get_mut(player_entity).unwrap();

                {
                    let new_instance_name = p.common.dimension.clone();

                    if let Some(mut instance_name) = instance_name {
                        *instance_name = instance_name.clone();
                    } else {
                        commands
                            .entity(player_entity)
                            .insert(InstanceName(new_instance_name.clone()));
                    }

                    let Some(dimension_type) = received_registries.dimension_type() else {
                        error!("Server didn't send dimension type registry, can't log in");
                        continue;
                    };
                    let dimension = &dimension_type
                        .value
                        .iter()
                        .find(|t| t.name == p.common.dimension_type)
                        .unwrap_or_else(|| {
                            panic!("No dimension_type with name {}", p.common.dimension_type)
                        })
                        .element;

                    // add this world to the instance_container (or don't if it's already
                    // there)
                    let weak_instance = instance_container.insert(
                        new_instance_name.clone(),
                        dimension.height,
                        dimension.min_y,
                    );
                    // set the partial_world to an empty world
                    // (when we add chunks or entities those will be in the
                    // instance_container)

                    *instance_holder.partial_instance.write() = PartialInstance::new(
                        azalea_world::calculate_chunk_storage_range(
                            client_information.view_distance.into(),
                        ),
                        // this argument makes it so other clients don't update this player entity
                        // in a shared instance
                        Some(player_entity),
                    );
                    instance_holder.instance = weak_instance;

                    let player_bundle = PlayerBundle {
                        entity: EntityBundle::new(
                            game_profile.uuid,
                            Vec3::default(),
                            azalea_registry::EntityKind::Player,
                            new_instance_name,
                        ),
                        metadata: PlayerMetadataBundle::default(),
                    };
                    // insert our components into the ecs :)
                    commands.entity(player_entity).insert((
                        MinecraftEntityId(p.player_id),
                        LocalGameMode {
                            current: p.common.game_type,
                            previous: p.common.previous_game_type.into(),
                        },
                        player_bundle,
                    ));
                }

                // send the client information that we have set
                debug!(
                    "Sending client information because login: {:?}",
                    client_information
                );
                send_packet_events.send(SendPacketEvent {
                    entity: player_entity,
                    packet: client_information.clone().get(),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::SetChunkCacheRadius(p) => {
                debug!("Got set chunk cache radius packet {p:?}");
            }

            ClientboundGamePacket::ChunkBatchStart(_p) => {
                // the packet is empty, just a marker to tell us when the batch starts and ends
                debug!("Got chunk batch start");
                let mut system_state: SystemState<
                    EventWriter<chunk_batching::ChunkBatchStartEvent>,
                > = SystemState::new(ecs);
                let mut chunk_batch_start_events = system_state.get_mut(ecs);

                chunk_batch_start_events.send(chunk_batching::ChunkBatchStartEvent {
                    entity: player_entity,
                });
            }
            ClientboundGamePacket::ChunkBatchFinished(p) => {
                debug!("Got chunk batch finished {p:?}");

                let mut system_state: SystemState<
                    EventWriter<chunk_batching::ChunkBatchFinishedEvent>,
                > = SystemState::new(ecs);
                let mut chunk_batch_start_events = system_state.get_mut(ecs);

                chunk_batch_start_events.send(chunk_batching::ChunkBatchFinishedEvent {
                    entity: player_entity,
                    batch_size: p.batch_size,
                });
            }

            ClientboundGamePacket::CustomPayload(p) => {
                debug!("Got custom payload packet {p:?}");
            }
            ClientboundGamePacket::ChangeDifficulty(p) => {
                debug!("Got difficulty packet {p:?}");
            }
            ClientboundGamePacket::Commands(_p) => {
                debug!("Got declare commands packet");
            }
            ClientboundGamePacket::PlayerAbilities(p) => {
                debug!("Got player abilities packet {p:?}");
                let mut system_state: SystemState<Query<&mut PlayerAbilities>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut player_abilities = query.get_mut(player_entity).unwrap();

                *player_abilities = PlayerAbilities::from(p);
            }
            ClientboundGamePacket::SetCarriedItem(p) => {
                debug!("Got set carried item packet {p:?}");
            }
            ClientboundGamePacket::UpdateTags(_p) => {
                debug!("Got update tags packet");
            }
            ClientboundGamePacket::Disconnect(p) => {
                warn!("Got disconnect packet {p:?}");
                let mut system_state: SystemState<EventWriter<DisconnectEvent>> =
                    SystemState::new(ecs);
                let mut disconnect_events = system_state.get_mut(ecs);
                disconnect_events.send(DisconnectEvent {
                    entity: player_entity,
                });
            }
            ClientboundGamePacket::UpdateRecipes(_p) => {
                debug!("Got update recipes packet");
            }
            ClientboundGamePacket::EntityEvent(_p) => {
                // debug!("Got entity event packet {p:?}");
            }
            ClientboundGamePacket::Recipe(_p) => {
                debug!("Got recipe packet");
            }
            ClientboundGamePacket::PlayerPosition(p) => {
                // TODO: reply with teleport confirm
                debug!("Got player position packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Query<(
                        &mut InstanceHolder,
                        &mut Physics,
                        &mut LookDirection,
                        &mut Position,
                        &mut LastSentPosition,
                    )>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut send_packet_events) = system_state.get_mut(ecs);
                let Ok((
                    instance_holder,
                    mut physics,
                    mut direction,
                    mut position,
                    mut last_sent_position,
                )) = query.get_mut(player_entity)
                else {
                    continue;
                };

                let delta_movement = physics.delta;

                let is_x_relative = p.relative_arguments.x;
                let is_y_relative = p.relative_arguments.y;
                let is_z_relative = p.relative_arguments.z;

                let (delta_x, new_pos_x) = if is_x_relative {
                    last_sent_position.x += p.x;
                    (delta_movement.x, position.x + p.x)
                } else {
                    last_sent_position.x = p.x;
                    (0.0, p.x)
                };
                let (delta_y, new_pos_y) = if is_y_relative {
                    last_sent_position.y += p.y;
                    (delta_movement.y, position.y + p.y)
                } else {
                    last_sent_position.y = p.y;
                    (0.0, p.y)
                };
                let (delta_z, new_pos_z) = if is_z_relative {
                    last_sent_position.z += p.z;
                    (delta_movement.z, position.z + p.z)
                } else {
                    last_sent_position.z = p.z;
                    (0.0, p.z)
                };

                let mut y_rot = p.y_rot;
                let mut x_rot = p.x_rot;
                if p.relative_arguments.x_rot {
                    x_rot += direction.x_rot;
                }
                if p.relative_arguments.y_rot {
                    y_rot += direction.y_rot;
                }

                physics.delta = Vec3 {
                    x: delta_x,
                    y: delta_y,
                    z: delta_z,
                };
                // we call a function instead of setting the fields ourself since the
                // function makes sure the rotations stay in their
                // ranges
                (direction.y_rot, direction.x_rot) = (y_rot, x_rot);
                // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                // so investigate that ig
                let new_pos = Vec3 {
                    x: new_pos_x,
                    y: new_pos_y,
                    z: new_pos_z,
                };

                **position = new_pos;

                send_packet_events.send(SendPacketEvent {
                    entity: player_entity,
                    packet: ServerboundAcceptTeleportationPacket { id: p.id }.get(),
                });
                send_packet_events.send(SendPacketEvent {
                    entity: player_entity,
                    packet: ServerboundMovePlayerPosRotPacket {
                        x: new_pos.x,
                        y: new_pos.y,
                        z: new_pos.z,
                        y_rot,
                        x_rot,
                        // this is always false
                        on_ground: false,
                    }
                    .get(),
                });
            }
            ClientboundGamePacket::PlayerInfoUpdate(p) => {
                debug!("Got player info packet {p:?}");

                let mut system_state: SystemState<(
                    Query<&mut TabList>,
                    EventWriter<AddPlayerEvent>,
                    EventWriter<UpdatePlayerEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut add_player_events, mut update_player_events) =
                    system_state.get_mut(ecs);
                let mut tab_list = query.get_mut(player_entity).unwrap();

                for updated_info in &p.entries {
                    // add the new player maybe
                    if p.actions.add_player {
                        let info = PlayerInfo {
                            profile: updated_info.profile.clone(),
                            uuid: updated_info.profile.uuid,
                            gamemode: updated_info.game_mode,
                            latency: updated_info.latency,
                            display_name: updated_info.display_name.clone(),
                        };
                        tab_list.insert(updated_info.profile.uuid, info.clone());
                        add_player_events.send(AddPlayerEvent {
                            entity: player_entity,
                            info: info.clone(),
                        });
                    } else if let Some(info) = tab_list.get_mut(&updated_info.profile.uuid) {
                        // `else if` because the block for add_player above
                        // already sets all the fields
                        if p.actions.update_game_mode {
                            info.gamemode = updated_info.game_mode;
                        }
                        if p.actions.update_latency {
                            info.latency = updated_info.latency;
                        }
                        if p.actions.update_display_name {
                            info.display_name = updated_info.display_name.clone();
                        }
                        update_player_events.send(UpdatePlayerEvent {
                            entity: player_entity,
                            info: info.clone(),
                        });
                    } else {
                        warn!(
                            "Ignoring PlayerInfoUpdate for unknown player {}",
                            updated_info.profile.uuid
                        );
                    }
                }
            }
            ClientboundGamePacket::PlayerInfoRemove(p) => {
                let mut system_state: SystemState<(
                    Query<&mut TabList>,
                    EventWriter<RemovePlayerEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut remove_player_events) = system_state.get_mut(ecs);
                let mut tab_list = query.get_mut(player_entity).unwrap();

                for uuid in &p.profile_ids {
                    if let Some(info) = tab_list.remove(uuid) {
                        remove_player_events.send(RemovePlayerEvent {
                            entity: player_entity,
                            info,
                        });
                    }
                }
            }
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                debug!("Got chunk cache center packet {p:?}");

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();
                let mut partial_world = local_player.partial_instance.write();

                partial_world.chunks.view_center = ChunkPos::new(p.x, p.z);
            }
            ClientboundGamePacket::ChunksBiomes(_) => {}
            ClientboundGamePacket::LightUpdate(_p) => {
                // debug!("Got light update packet {p:?}");
            }
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                debug!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                // OPTIMIZATION: if we already know about the chunk from the
                // shared world (and not ourselves), then we don't need to
                // parse it again. This is only used when we have a shared
                // world, since we check that the chunk isn't currently owned
                // by this client.
                let shared_chunk = local_player.instance.read().chunks.get(&pos);
                let this_client_has_chunk = local_player
                    .partial_instance
                    .read()
                    .chunks
                    .limited_get(&pos)
                    .is_some();

                let mut world = local_player.instance.write();
                let mut partial_world = local_player.partial_instance.write();

                if !this_client_has_chunk {
                    if let Some(shared_chunk) = shared_chunk {
                        trace!(
                            "Skipping parsing chunk {:?} because we already know about it",
                            pos
                        );
                        partial_world.chunks.set_with_shared_reference(
                            &pos,
                            Some(shared_chunk.clone()),
                            &mut world.chunks,
                        );
                        continue;
                    }
                }

                if let Err(e) = partial_world.chunks.replace_with_packet_data(
                    &pos,
                    &mut Cursor::new(&p.chunk_data.data),
                    &mut world.chunks,
                ) {
                    error!("Couldn't set chunk data: {}", e);
                }
            }
            ClientboundGamePacket::AddEntity(p) => {
                debug!("Got add entity packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<Option<&InstanceName>>,
                    Res<InstanceContainer>,
                    ResMut<EntityUuidIndex>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, instance_container, mut entity_uuid_index) =
                    system_state.get_mut(ecs);
                let instance_name = query.get_mut(player_entity).unwrap();

                if let Some(instance_name) = instance_name {
                    let bundle = p.as_entity_bundle((**instance_name).clone());
                    let mut entity_commands = commands.spawn((
                        MinecraftEntityId(p.id),
                        LoadedBy(HashSet::from([player_entity])),
                        bundle,
                    ));

                    {
                        // add it to the indexes immediately so if there's a packet that references
                        // it immediately after it still works
                        let instance = instance_container.get(instance_name).unwrap();
                        instance
                            .write()
                            .entity_by_id
                            .insert(MinecraftEntityId(p.id), entity_commands.id());
                        entity_uuid_index.insert(p.uuid, entity_commands.id());
                    }

                    // the bundle doesn't include the default entity metadata so we add that
                    // separately
                    p.apply_metadata(&mut entity_commands);
                } else {
                    warn!("got add player packet but we haven't gotten a login packet yet");
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::SetEntityData(p) => {
                debug!("Got set entity data packet {p:?}");

                let mut system_state: SystemState<(
                    Commands,
                    Query<&mut InstanceHolder>,
                    Query<&EntityKind>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, entity_kind_query) = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.read();
                let entity = world.entity_by_id(&MinecraftEntityId(p.id));
                drop(world);

                if let Some(entity) = entity {
                    let entity_kind = entity_kind_query.get(entity).unwrap();
                    let mut entity_commands = commands.entity(entity);
                    if let Err(e) = apply_metadata(
                        &mut entity_commands,
                        **entity_kind,
                        (*p.packed_items).clone(),
                    ) {
                        warn!("{e}");
                    }
                } else {
                    warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::UpdateAttributes(_p) => {
                // debug!("Got update attributes packet {p:?}");
            }
            ClientboundGamePacket::SetEntityMotion(_p) => {
                // debug!("Got entity velocity packet {p:?}");
            }
            ClientboundGamePacket::SetEntityLink(p) => {
                debug!("Got set entity link packet {p:?}");
            }
            ClientboundGamePacket::AddPlayer(p) => {
                debug!("Got add player packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&TabList, Option<&InstanceName>)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (tab_list, world_name) = query.get_mut(player_entity).unwrap();

                if let Some(InstanceName(world_name)) = world_name {
                    let bundle = p.as_player_bundle(world_name.clone());
                    let mut spawned = commands.spawn((
                        MinecraftEntityId(p.id),
                        LoadedBy(HashSet::from([player_entity])),
                        bundle,
                    ));

                    if let Some(player_info) = tab_list.get(&p.uuid) {
                        spawned.insert(GameProfileComponent(player_info.profile.clone()));
                    }
                } else {
                    warn!("got add player packet but we haven't gotten a login packet yet");
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::InitializeBorder(p) => {
                debug!("Got initialize border packet {p:?}");
            }
            ClientboundGamePacket::SetTime(_p) => {
                // debug!("Got set time packet {p:?}");
            }
            ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                debug!("Got set default spawn position packet {p:?}");
            }
            ClientboundGamePacket::SetHealth(p) => {
                debug!("Got set health packet {p:?}");

                let mut system_state: SystemState<Query<&mut Health>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut health = query.get_mut(player_entity).unwrap();

                **health = p.health;

                // the `Dead` component is added by the `update_dead` system
                // in azalea-world and then the `dead_event` system fires
                // the Death event.
            }
            ClientboundGamePacket::SetExperience(p) => {
                debug!("Got set experience packet {p:?}");
            }
            ClientboundGamePacket::TeleportEntity(p) => {
                let mut system_state: SystemState<(Commands, Query<&mut InstanceHolder>)> =
                    SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.read();
                let entity = world.entity_by_id(&MinecraftEntityId(p.id));
                drop(world);

                if let Some(entity) = entity {
                    let new_position = p.position;
                    commands.entity(entity).add(RelativeEntityUpdate {
                        partial_world: local_player.partial_instance.clone(),
                        update: Box::new(move |entity| {
                            let mut position = entity.get_mut::<Position>().unwrap();
                            **position = new_position;
                        }),
                    });
                } else {
                    warn!("Got teleport entity packet for unknown entity id {}", p.id);
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {p:?}");
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {p:?}");
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut system_state: SystemState<(Commands, Query<&InstanceHolder>)> =
                    SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.read();
                let entity = world.entity_by_id(&MinecraftEntityId(p.entity_id));
                drop(world);

                if let Some(entity) = entity {
                    let delta = p.delta.clone();
                    commands.entity(entity).add(RelativeEntityUpdate {
                        partial_world: local_player.partial_instance.clone(),
                        update: Box::new(move |entity_mut| {
                            let mut position = entity_mut.get_mut::<Position>().unwrap();
                            **position = position.with_delta(&delta);
                        }),
                    });
                } else {
                    warn!(
                        "Got move entity pos packet for unknown entity id {}",
                        p.entity_id
                    );
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut system_state: SystemState<(Commands, Query<&mut InstanceHolder>)> =
                    SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.read();
                let entity = world.entity_by_id(&MinecraftEntityId(p.entity_id));
                drop(world);

                if let Some(entity) = entity {
                    let delta = p.delta.clone();
                    commands.entity(entity).add(RelativeEntityUpdate {
                        partial_world: local_player.partial_instance.clone(),
                        update: Box::new(move |entity_mut| {
                            let mut position = entity_mut.get_mut::<Position>().unwrap();
                            **position = position.with_delta(&delta);
                        }),
                    });
                } else {
                    warn!(
                        "Got move entity pos rot packet for unknown entity id {}",
                        p.entity_id
                    );
                }

                system_state.apply(ecs);
            }

            ClientboundGamePacket::MoveEntityRot(_p) => {
                // debug!("Got move entity rot packet {p:?}");
            }
            ClientboundGamePacket::KeepAlive(p) => {
                debug!("Got keep alive packet {p:?} for {player_entity:?}");

                let mut system_state: SystemState<(
                    EventWriter<KeepAliveEvent>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (mut keepalive_events, mut send_packet_events) = system_state.get_mut(ecs);

                keepalive_events.send(KeepAliveEvent {
                    entity: player_entity,
                    id: p.id,
                });
                send_packet_events.send(SendPacketEvent {
                    entity: player_entity,
                    packet: ServerboundKeepAlivePacket { id: p.id }.get(),
                });
            }
            ClientboundGamePacket::RemoveEntities(p) => {
                debug!("Got remove entities packet {p:?}");
            }
            ClientboundGamePacket::PlayerChat(p) => {
                debug!("Got player chat packet {p:?}");

                let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                    SystemState::new(ecs);
                let mut chat_events = system_state.get_mut(ecs);

                chat_events.send(ChatReceivedEvent {
                    entity: player_entity,
                    packet: ChatPacket::Player(Arc::new(p.clone())),
                });
            }
            ClientboundGamePacket::SystemChat(p) => {
                debug!("Got system chat packet {p:?}");

                let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                    SystemState::new(ecs);
                let mut chat_events = system_state.get_mut(ecs);

                chat_events.send(ChatReceivedEvent {
                    entity: player_entity,
                    packet: ChatPacket::System(Arc::new(p.clone())),
                });
            }
            ClientboundGamePacket::Sound(_p) => {
                // debug!("Got sound packet {p:?}");
            }
            ClientboundGamePacket::LevelEvent(p) => {
                debug!("Got level event packet {p:?}");
            }
            ClientboundGamePacket::BlockUpdate(p) => {
                debug!("Got block update packet {p:?}");

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.write();

                world.chunks.set_block_state(&p.pos, p.block_state);
            }
            ClientboundGamePacket::Animate(p) => {
                debug!("Got animate packet {p:?}");
            }
            ClientboundGamePacket::SectionBlocksUpdate(p) => {
                debug!("Got section blocks update packet {p:?}");
                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.write();

                for state in &p.states {
                    world
                        .chunks
                        .set_block_state(&(p.section_pos + state.pos), state.state);
                }
            }
            ClientboundGamePacket::GameEvent(p) => {
                use azalea_protocol::packets::game::clientbound_game_event_packet::EventType;

                debug!("Got game event packet {p:?}");

                #[allow(clippy::single_match)]
                match p.event {
                    EventType::ChangeGameMode => {
                        let mut system_state: SystemState<Query<&mut LocalGameMode>> =
                            SystemState::new(ecs);
                        let mut query = system_state.get_mut(ecs);
                        let mut local_game_mode = query.get_mut(player_entity).unwrap();
                        if let Some(new_game_mode) = GameMode::from_id(p.param as u8) {
                            local_game_mode.current = new_game_mode;
                        }
                    }
                    _ => {}
                }
            }
            ClientboundGamePacket::LevelParticles(p) => {
                debug!("Got level particles packet {p:?}");
            }
            ClientboundGamePacket::ServerData(p) => {
                debug!("Got server data packet {p:?}");
            }
            ClientboundGamePacket::SetEquipment(p) => {
                debug!("Got set equipment packet {p:?}");
            }
            ClientboundGamePacket::UpdateMobEffect(p) => {
                debug!("Got update mob effect packet {p:?}");
            }
            ClientboundGamePacket::AddExperienceOrb(_) => {}
            ClientboundGamePacket::AwardStats(_) => {}
            ClientboundGamePacket::BlockChangedAck(_) => {}
            ClientboundGamePacket::BlockDestruction(_) => {}
            ClientboundGamePacket::BlockEntityData(_) => {}
            ClientboundGamePacket::BlockEvent(p) => {
                debug!("Got block event packet {p:?}");
            }
            ClientboundGamePacket::BossEvent(_) => {}
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetContent(p) => {
                debug!("Got container set content packet {p:?}");

                let mut system_state: SystemState<(
                    Query<&mut InventoryComponent>,
                    EventWriter<SetContainerContentEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut events) = system_state.get_mut(ecs);
                let mut inventory = query.get_mut(player_entity).unwrap();

                // container id 0 is always the player's inventory
                if p.container_id == 0 {
                    // this is just so it has the same type as the `else` block
                    for (i, slot) in p.items.iter().enumerate() {
                        if let Some(slot_mut) = inventory.inventory_menu.slot_mut(i) {
                            *slot_mut = slot.clone();
                        }
                    }
                } else {
                    events.send(SetContainerContentEvent {
                        entity: player_entity,
                        slots: p.items.clone(),
                        container_id: p.container_id as u8,
                    });
                }
            }
            ClientboundGamePacket::ContainerSetData(p) => {
                debug!("Got container set data packet {p:?}");
                // let mut system_state: SystemState<Query<&mut
                // InventoryComponent>> =
                //     SystemState::new(ecs);
                // let mut query = system_state.get_mut(ecs);
                // let mut inventory =
                // query.get_mut(player_entity).unwrap();

                // TODO: handle ContainerSetData packet
                // this is used for various things like the furnace progress
                // bar
                // see https://wiki.vg/Protocol#Set_Container_Property
            }
            ClientboundGamePacket::ContainerSetSlot(p) => {
                debug!("Got container set slot packet {p:?}");

                let mut system_state: SystemState<Query<&mut InventoryComponent>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut inventory = query.get_mut(player_entity).unwrap();

                if p.container_id == -1 {
                    // -1 means carried item
                    inventory.carried = p.item_stack.clone();
                } else if p.container_id == -2 {
                    if let Some(slot) = inventory.inventory_menu.slot_mut(p.slot.into()) {
                        *slot = p.item_stack.clone();
                    }
                } else {
                    let is_creative_mode_and_inventory_closed = false;
                    // technically minecraft has slightly different behavior here if you're in
                    // creative mode and have your inventory open
                    if p.container_id == 0
                        && azalea_inventory::Player::is_hotbar_slot(p.slot.into())
                    {
                        // minecraft also sets a "pop time" here which is used for an animation
                        // but that's not really necessary
                        if let Some(slot) = inventory.inventory_menu.slot_mut(p.slot.into()) {
                            *slot = p.item_stack.clone();
                        }
                    } else if p.container_id == (inventory.id as i8)
                        && (p.container_id != 0 || !is_creative_mode_and_inventory_closed)
                    {
                        // var2.containerMenu.setItem(var4, var1.getStateId(), var3);
                        if let Some(slot) = inventory.menu_mut().slot_mut(p.slot.into()) {
                            *slot = p.item_stack.clone();
                            inventory.state_id = p.state_id;
                        }
                    }
                }
            }
            ClientboundGamePacket::ContainerClose(_p) => {
                // there's p.container_id but minecraft doesn't actually check it
                let mut system_state: SystemState<EventWriter<ClientSideCloseContainerEvent>> =
                    SystemState::new(ecs);
                let mut client_side_close_container_events = system_state.get_mut(ecs);
                client_side_close_container_events.send(ClientSideCloseContainerEvent {
                    entity: player_entity,
                })
            }
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
            ClientboundGamePacket::DeleteChat(_) => {}
            ClientboundGamePacket::Explode(_) => {}
            ClientboundGamePacket::ForgetLevelChunk(_) => {}
            ClientboundGamePacket::HorseScreenOpen(_) => {}
            ClientboundGamePacket::MapItemData(_) => {}
            ClientboundGamePacket::MerchantOffers(_) => {}
            ClientboundGamePacket::MoveVehicle(_) => {}
            ClientboundGamePacket::OpenBook(_) => {}
            ClientboundGamePacket::OpenScreen(p) => {
                debug!("Got open screen packet {p:?}");
                let mut system_state: SystemState<EventWriter<MenuOpenedEvent>> =
                    SystemState::new(ecs);
                let mut menu_opened_events = system_state.get_mut(ecs);
                menu_opened_events.send(MenuOpenedEvent {
                    entity: player_entity,
                    window_id: p.container_id,
                    menu_type: p.menu_type,
                    title: p.title,
                })
            }
            ClientboundGamePacket::OpenSignEditor(_) => {}
            ClientboundGamePacket::Ping(p) => {
                debug!("Got ping packet {p:?}");

                let mut system_state: SystemState<EventWriter<SendPacketEvent>> =
                    SystemState::new(ecs);
                let mut send_packet_events = system_state.get_mut(ecs);

                send_packet_events.send(SendPacketEvent {
                    entity: player_entity,
                    packet: ServerboundPongPacket { id: p.id }.get(),
                });
            }
            ClientboundGamePacket::PlaceGhostRecipe(_) => {}
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(p) => {
                debug!("Got player kill packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&MinecraftEntityId, Option<&Dead>)>,
                    EventWriter<DeathEvent>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, mut death_events) = system_state.get_mut(ecs);
                let (entity_id, dead) = query.get_mut(player_entity).unwrap();

                if **entity_id == p.player_id && dead.is_none() {
                    commands.entity(player_entity).insert(Dead);
                    death_events.send(DeathEvent {
                        entity: player_entity,
                        packet: Some(p.clone()),
                    });
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::PlayerLookAt(_) => {}
            ClientboundGamePacket::RemoveMobEffect(_) => {}
            ClientboundGamePacket::ResourcePack(p) => {
                debug!("Got resource pack packet {p:?}");

                let mut system_state: SystemState<EventWriter<SendPacketEvent>> =
                    SystemState::new(ecs);
                let mut send_packet_events = system_state.get_mut(ecs);

                // always accept resource pack
                send_packet_events.send(SendPacketEvent { entity: player_entity, packet: ServerboundResourcePackPacket { action: azalea_protocol::packets::game::serverbound_resource_pack_packet::Action::Accepted }.get() });
            }
            ClientboundGamePacket::Respawn(p) => {
                debug!("Got respawn packet {p:?}");

                let mut system_state: SystemState<Commands> = SystemState::new(ecs);
                let mut commands = system_state.get(ecs);

                // Remove the Dead marker component from the player.
                commands.entity(player_entity).remove::<Dead>();

                system_state.apply(ecs);
            }

            ClientboundGamePacket::SelectAdvancementsTab(_) => {}
            ClientboundGamePacket::SetActionBarText(_) => {}
            ClientboundGamePacket::SetBorderCenter(_) => {}
            ClientboundGamePacket::SetBorderLerpSize(_) => {}
            ClientboundGamePacket::SetBorderSize(_) => {}
            ClientboundGamePacket::SetBorderWarningDelay(_) => {}
            ClientboundGamePacket::SetBorderWarningDistance(_) => {}
            ClientboundGamePacket::SetCamera(_) => {}
            ClientboundGamePacket::SetDisplayObjective(_) => {}
            ClientboundGamePacket::SetObjective(_) => {}
            ClientboundGamePacket::SetPassengers(_) => {}
            ClientboundGamePacket::SetPlayerTeam(_) => {}
            ClientboundGamePacket::SetScore(_) => {}
            ClientboundGamePacket::SetSimulationDistance(_) => {}
            ClientboundGamePacket::SetSubtitleText(_) => {}
            ClientboundGamePacket::SetTitleText(_) => {}
            ClientboundGamePacket::SetTitlesAnimation(_) => {}
            ClientboundGamePacket::ClearTitles(_) => {}
            ClientboundGamePacket::SoundEntity(_) => {}
            ClientboundGamePacket::StopSound(_) => {}
            ClientboundGamePacket::TabList(_) => {}
            ClientboundGamePacket::TagQuery(_) => {}
            ClientboundGamePacket::TakeItemEntity(_) => {}
            ClientboundGamePacket::DisguisedChat(_) => {}
            ClientboundGamePacket::Bundle(_) => {}
            ClientboundGamePacket::DamageEvent(_) => {}
            ClientboundGamePacket::HurtAnimation(_) => {}

            ClientboundGamePacket::StartConfiguration(_) => todo!(),
        }
    }
}