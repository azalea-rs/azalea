use std::{
    collections::HashSet,
    io::Cursor,
    ops::Add,
    sync::{Arc, Weak},
};

use azalea_chat::FormattedText;
use azalea_core::{
    game_type::GameMode,
    math,
    position::{ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::{
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::{apply_metadata, Health},
    Dead, EntityBundle, EntityKind, LastSentPosition, LoadedBy, LocalEntity, LookDirection,
    Physics, Position, RelativeEntityUpdate,
};
use azalea_protocol::{
    packets::{game::*, Packet},
    read::deserialize_packet,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemState},
};
use parking_lot::RwLock;
use tracing::{debug, error, trace, warn};
use uuid::Uuid;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    chunks,
    disconnect::DisconnectEvent,
    inventory::{
        ClientSideCloseContainerEvent, Inventory, MenuOpenedEvent, SetContainerContentEvent,
    },
    local_player::{
        GameProfileComponent, Hunger, InstanceHolder, LocalGameMode, PlayerAbilities, TabList,
    },
    movement::{KnockbackEvent, KnockbackType},
    raw_connection::RawConnection,
    ClientInformation, PlayerInfo,
};

/// An event that's sent when we receive a packet.
/// ```
/// # use azalea_client::packet_handling::game::PacketEvent;
/// # use azalea_protocol::packets::game::ClientboundGamePacket;
/// # use bevy_ecs::event::EventReader;
///
/// fn handle_packets(mut events: EventReader<PacketEvent>) {
///     for PacketEvent {
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
pub struct PacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundGamePacket>,
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
                            error!("failed to read packet: {err:?}");
                            debug!("packet bytes: {raw_packet:?}");
                            continue;
                        }
                    };
                packet_events.send(PacketEvent {
                    entity: player_entity,
                    packet: Arc::new(packet),
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::<(Entity, Arc<ClientboundGamePacket>)>::new();
    {
        let mut system_state = SystemState::<EventReader<PacketEvent>>::new(ecs);
        let mut events = system_state.get_mut(ecs);
        for PacketEvent {
            entity: player_entity,
            packet,
        } in events.read()
        {
            // we do this so `ecs` isn't borrowed for the whole loop
            events_owned.push((*player_entity, packet.clone()));
        }
    }
    for (player_entity, packet) in events_owned {
        let packet_clone = packet.clone();
        let packet_ref = packet_clone.as_ref();

        let mut handler = GamePacketHandler {
            player: player_entity,
            ecs,
        };

        match packet_ref {
            ClientboundGamePacket::Login(p) => handler.handle_login(p),
            ClientboundGamePacket::SetChunkCacheRadius(p) => {
                handler.handle_set_chunk_cache_radius(p)
            }
            ClientboundGamePacket::ChunkBatchStart(p) => handler.handle_chunk_batch_start(p),
            ClientboundGamePacket::ChunkBatchFinished(p) => handler.handle_chunk_batch_finished(p),
            ClientboundGamePacket::CustomPayload(p) => handler.handle_custom_payload(p),
            ClientboundGamePacket::ChangeDifficulty(p) => handler.handle_change_difficulty(p),
            ClientboundGamePacket::Commands(p) => handler.handle_commands(p),
            ClientboundGamePacket::PlayerAbilities(p) => handler.handle_player_abilities(p),
            ClientboundGamePacket::SetCursorItem(p) => handler.handle_set_cursor_item(p),
            ClientboundGamePacket::UpdateTags(p) => handler.handle_update_tags(p),
            ClientboundGamePacket::Disconnect(p) => handler.handle_disconnect(p),
            ClientboundGamePacket::UpdateRecipes(p) => handler.handle_update_recipes(p),
            ClientboundGamePacket::EntityEvent(p) => handler.handle_entity_event(p),
            ClientboundGamePacket::PlayerPosition(p) => handler.handle_player_position(p),
            ClientboundGamePacket::PlayerInfoUpdate(p) => handler.handle_player_info_update(p),
            ClientboundGamePacket::PlayerInfoRemove(p) => handler.handle_player_info_remove(p),
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                handler.handle_set_chunk_cache_center(p)
            }
            ClientboundGamePacket::ChunksBiomes(_) => {}
            ClientboundGamePacket::LightUpdate(p) => handler.handle_light_update(p),
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                handler.handle_level_chunk_with_light(p)
            }
            ClientboundGamePacket::AddEntity(p) => handler.handle_add_entity(p),
            ClientboundGamePacket::SetEntityData(p) => handler.handle_set_entity_data(p),
            ClientboundGamePacket::UpdateAttributes(p) => handler.handle_update_attributes(p),
            ClientboundGamePacket::SetEntityMotion(p) => handler.handle_set_entity_motion(p),
            ClientboundGamePacket::SetEntityLink(p) => handler.handle_set_entity_link(p),
            ClientboundGamePacket::InitializeBorder(p) => handler.handle_initialize_border(p),
            ClientboundGamePacket::SetTime(p) => handler.handle_set_time(p),
            ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                handler.handle_set_default_spawn_position(p)
            }
            ClientboundGamePacket::SetHealth(p) => handler.handle_set_health(p),
            ClientboundGamePacket::SetExperience(p) => handler.handle_set_experience(p),
            ClientboundGamePacket::TeleportEntity(p) => handler.handle_teleport_entity(p),
            ClientboundGamePacket::UpdateAdvancements(p) => handler.handle_update_advancements(p),
            ClientboundGamePacket::RotateHead(p) => handler.handle_rotate_head(p),
            ClientboundGamePacket::MoveEntityPos(p) => handler.handle_move_entity_pos(p),
            ClientboundGamePacket::MoveEntityPosRot(p) => handler.handle_move_entity_pos_rot(p),
            ClientboundGamePacket::MoveEntityRot(p) => handler.handle_move_entity_rot(p),
            ClientboundGamePacket::KeepAlive(p) => handler.handle_keep_alive(p),
            ClientboundGamePacket::RemoveEntities(p) => handler.handle_remove_entities(p),
            ClientboundGamePacket::PlayerChat(p) => handler.handle_player_chat(p),
            ClientboundGamePacket::SystemChat(p) => handler.handle_system_chat(p),
            ClientboundGamePacket::DisguisedChat(p) => handler.handle_disguised_chat(p),
            ClientboundGamePacket::Sound(_) => {}
            ClientboundGamePacket::LevelEvent(p) => handler.handle_level_event(p),
            ClientboundGamePacket::BlockUpdate(p) => handler.handle_block_update(p),
            ClientboundGamePacket::Animate(p) => handler.handle_animate(p),
            ClientboundGamePacket::SectionBlocksUpdate(p) => {
                handler.handle_section_blocks_update(p)
            }
            ClientboundGamePacket::GameEvent(p) => handler.handle_game_event(p),
            ClientboundGamePacket::LevelParticles(p) => handler.handle_level_particles(p),
            ClientboundGamePacket::ServerData(p) => handler.handle_server_data(p),
            ClientboundGamePacket::SetEquipment(p) => handler.handle_set_equipment(p),
            ClientboundGamePacket::UpdateMobEffect(p) => handler.handle_update_mob_effect(p),
            ClientboundGamePacket::AddExperienceOrb(_) => {}
            ClientboundGamePacket::AwardStats(_) => {}
            ClientboundGamePacket::BlockChangedAck(_) => {}
            ClientboundGamePacket::BlockDestruction(_) => {}
            ClientboundGamePacket::BlockEntityData(_) => {}
            ClientboundGamePacket::BlockEvent(p) => handler.handle_block_event(p),
            ClientboundGamePacket::BossEvent(_) => {}
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetContent(p) => {
                handler.handle_container_set_content(p)
            }
            ClientboundGamePacket::ContainerSetData(p) => handler.handle_container_set_data(p),
            ClientboundGamePacket::ContainerSetSlot(p) => handler.handle_container_set_slot(p),
            ClientboundGamePacket::ContainerClose(p) => handler.handle_container_close(p),
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
            ClientboundGamePacket::DeleteChat(_) => {}
            ClientboundGamePacket::Explode(p) => handler.handle_explode(p),
            ClientboundGamePacket::ForgetLevelChunk(p) => handler.handle_forget_level_chunk(p),
            ClientboundGamePacket::HorseScreenOpen(_) => {}
            ClientboundGamePacket::MapItemData(_) => {}
            ClientboundGamePacket::MerchantOffers(_) => {}
            ClientboundGamePacket::MoveVehicle(_) => {}
            ClientboundGamePacket::OpenBook(_) => {}
            ClientboundGamePacket::OpenScreen(p) => handler.handle_open_screen(p),
            ClientboundGamePacket::OpenSignEditor(_) => {}
            ClientboundGamePacket::Ping(p) => handler.handle_ping(p),
            ClientboundGamePacket::PlaceGhostRecipe(_) => {}
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(p) => handler.handle_player_combat_kill(p),
            ClientboundGamePacket::PlayerLookAt(_) => {}
            ClientboundGamePacket::RemoveMobEffect(_) => {}
            ClientboundGamePacket::ResourcePackPush(p) => handler.handle_resource_pack_push(p),
            ClientboundGamePacket::ResourcePackPop(_) => {}
            ClientboundGamePacket::Respawn(p) => handler.handle_respawn(p),
            ClientboundGamePacket::StartConfiguration(p) => handler.handle_start_configuration(p),
            ClientboundGamePacket::EntityPositionSync(p) => handler.handle_entity_position_sync(p),

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
            ClientboundGamePacket::BundleDelimiter(_) => {}
            ClientboundGamePacket::DamageEvent(_) => {}
            ClientboundGamePacket::HurtAnimation(_) => {}

            ClientboundGamePacket::TickingState(_) => {}
            ClientboundGamePacket::TickingStep(_) => {}

            ClientboundGamePacket::ResetScore(_) => {}
            ClientboundGamePacket::CookieRequest(_) => {}
            ClientboundGamePacket::DebugSample(_) => {}
            ClientboundGamePacket::PongResponse(_) => {}
            ClientboundGamePacket::StoreCookie(_) => {}
            ClientboundGamePacket::Transfer(_) => {}
            ClientboundGamePacket::MoveMinecartAlongTrack(_) => {}
            ClientboundGamePacket::SetHeldSlot(_) => {}
            ClientboundGamePacket::SetPlayerInventory(_) => {}
            ClientboundGamePacket::ProjectilePower(_) => {}
            ClientboundGamePacket::CustomReportDetails(_) => {}
            ClientboundGamePacket::ServerLinks(_) => {}
            ClientboundGamePacket::PlayerRotation(_) => {}
            ClientboundGamePacket::RecipeBookAdd(_) => {}
            ClientboundGamePacket::RecipeBookRemove(_) => {}
            ClientboundGamePacket::RecipeBookSettings(_) => {}
        }
    }
}

impl GamePacketHandler<'_> {
    pub fn handle_login(&mut self, p: &ClientboundLogin) {
        debug!("Got login packet");

        as_system::<(
            Commands,
            Query<(
                &GameProfileComponent,
                &ClientInformation,
                Option<&mut InstanceName>,
                Option<&mut LoadedBy>,
                &mut EntityIdIndex,
                &mut InstanceHolder,
            )>,
            EventWriter<InstanceLoadedEvent>,
            ResMut<InstanceContainer>,
            ResMut<EntityUuidIndex>,
            EventWriter<SendPacketEvent>,
        )>(
            self.ecs,
            |(
                mut commands,
                mut query,
                mut instance_loaded_events,
                mut instance_container,
                mut entity_uuid_index,
                mut send_packet_events,
            )| {
                let (
                    game_profile,
                    client_information,
                    instance_name,
                    loaded_by,
                    mut entity_id_index,
                    mut instance_holder,
                ) = query.get_mut(self.player).unwrap();

                {
                    let new_instance_name = p.common.dimension.clone();

                    if let Some(mut instance_name) = instance_name {
                        *instance_name = instance_name.clone();
                    } else {
                        commands
                            .entity(self.player)
                            .insert(InstanceName(new_instance_name.clone()));
                    }

                    let Some(dimension_type_element) =
                        instance_holder.instance.read().registries.dimension_type()
                    else {
                        error!("Server didn't send dimension type registry, can't log in");
                        return;
                    };

                    let dimension_name = ResourceLocation::new(&p.common.dimension.to_string());

                    let Some(dimension) = dimension_type_element.map.get(&dimension_name) else {
                        error!("No dimension_type with name {dimension_name}");
                        return;
                    };

                    // add this world to the instance_container (or don't if it's already
                    // there)
                    let weak_instance = instance_container.insert(
                        new_instance_name.clone(),
                        dimension.height,
                        dimension.min_y,
                    );
                    instance_loaded_events.send(InstanceLoadedEvent {
                        entity: self.player,
                        name: new_instance_name.clone(),
                        instance: Arc::downgrade(&weak_instance),
                    });

                    // set the partial_world to an empty world
                    // (when we add chunks or entities those will be in the
                    // instance_container)

                    *instance_holder.partial_instance.write() = PartialInstance::new(
                        azalea_world::chunk_storage::calculate_chunk_storage_range(
                            client_information.view_distance.into(),
                        ),
                        // this argument makes it so other clients don't update this player entity
                        // in a shared instance
                        Some(self.player),
                    );
                    {
                        let map = instance_holder.instance.read().registries.map.clone();
                        let new_registries = &mut weak_instance.write().registries;
                        // add the registries from this instance to the weak instance
                        for (registry_name, registry) in map {
                            new_registries.map.insert(registry_name, registry);
                        }
                    }
                    instance_holder.instance = weak_instance;

                    let entity_bundle = EntityBundle::new(
                        game_profile.uuid,
                        Vec3::default(),
                        azalea_registry::EntityKind::Player,
                        new_instance_name,
                    );
                    let entity_id = MinecraftEntityId(p.player_id);
                    // insert our components into the ecs :)
                    commands.entity(self.player).insert((
                        entity_id,
                        LocalGameMode {
                            current: p.common.game_type,
                            previous: p.common.previous_game_type.into(),
                        },
                        entity_bundle,
                    ));

                    azalea_entity::indexing::add_entity_to_indexes(
                        entity_id,
                        self.player,
                        Some(game_profile.uuid),
                        &mut entity_id_index,
                        &mut entity_uuid_index,
                        &mut instance_holder.instance.write(),
                    );

                    // update or insert loaded_by
                    if let Some(mut loaded_by) = loaded_by {
                        loaded_by.insert(self.player);
                    } else {
                        commands
                            .entity(self.player)
                            .insert(LoadedBy(HashSet::from_iter(vec![self.player])));
                    }
                }

                // send the client information that we have set
                debug!(
                    "Sending client information because login: {:?}",
                    client_information
                );
                send_packet_events.send(SendPacketEvent::new(
                    self.player,
                    ServerboundClientInformation {
                        information: client_information.clone(),
                    },
                ));
            },
        );
    }

    pub fn handle_set_chunk_cache_radius(&mut self, p: &ClientboundSetChunkCacheRadius) {
        debug!("Got set chunk cache radius packet {p:?}");
    }

    pub fn handle_chunk_batch_start(&mut self, _p: &ClientboundChunkBatchStart) {
        // the packet is empty, it's just a marker to tell us when the batch starts and
        // ends
        debug!("Got chunk batch start");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ChunkBatchStartEvent {
                entity: self.player,
            });
        });
    }

    pub fn handle_chunk_batch_finished(&mut self, p: &ClientboundChunkBatchFinished) {
        debug!("Got chunk batch finished {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ChunkBatchFinishedEvent {
                entity: self.player,
                batch_size: p.batch_size,
            });
        });
    }

    pub fn handle_custom_payload(&mut self, p: &ClientboundCustomPayload) {
        debug!("Got custom payload packet {p:?}");
    }

    pub fn handle_change_difficulty(&mut self, p: &ClientboundChangeDifficulty) {
        debug!("Got difficulty packet {p:?}");
    }

    pub fn handle_commands(&mut self, _p: &ClientboundCommands) {
        debug!("Got declare commands packet");
    }

    pub fn handle_player_abilities(&mut self, p: &ClientboundPlayerAbilities) {
        debug!("Got player abilities packet {p:?}");

        as_system::<Query<&mut PlayerAbilities>>(self.ecs, |mut query| {
            let mut player_abilities = query.get_mut(self.player).unwrap();

            *player_abilities = PlayerAbilities::from(p);
        });
    }

    pub fn handle_set_cursor_item(&mut self, p: &ClientboundSetCursorItem) {
        debug!("Got set cursor item packet {p:?}");
    }

    pub fn handle_update_tags(&mut self, _p: &ClientboundUpdateTags) {
        debug!("Got update tags packet");
    }

    pub fn handle_disconnect(&mut self, p: &ClientboundDisconnect) {
        warn!("Got disconnect packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason.clone()),
            });
        });
    }

    pub fn handle_update_recipes(&mut self, _p: &ClientboundUpdateRecipes) {
        debug!("Got update recipes packet");
    }

    pub fn handle_entity_event(&mut self, _p: &ClientboundEntityEvent) {
        // debug!("Got entity event packet {p:?}");
    }

    pub fn handle_player_position(&mut self, p: &ClientboundPlayerPosition) {
        debug!("Got player position packet {p:?}");

        as_system::<(
            Query<(
                &mut Physics,
                &mut LookDirection,
                &mut Position,
                &mut LastSentPosition,
            )>,
            EventWriter<SendPacketEvent>,
        )>(self.ecs, |(mut query, mut send_packet_events)| {
            let Ok((mut physics, mut direction, mut position, mut last_sent_position)) =
                query.get_mut(self.player)
            else {
                return;
            };

            **last_sent_position = **position;

            fn apply_change<T: Add<Output = T>>(base: T, condition: bool, change: T) -> T {
                if condition {
                    base + change
                } else {
                    change
                }
            }

            let new_x = apply_change(position.x, p.relative.x, p.change.pos.x);
            let new_y = apply_change(position.y, p.relative.y, p.change.pos.y);
            let new_z = apply_change(position.z, p.relative.z, p.change.pos.z);

            let new_y_rot = apply_change(
                direction.y_rot,
                p.relative.y_rot,
                p.change.look_direction.y_rot,
            );
            let new_x_rot = apply_change(
                direction.x_rot,
                p.relative.x_rot,
                p.change.look_direction.x_rot,
            );

            let mut new_delta_from_rotations = physics.velocity;
            if p.relative.rotate_delta {
                let y_rot_delta = direction.y_rot - new_y_rot;
                let x_rot_delta = direction.x_rot - new_x_rot;
                new_delta_from_rotations = new_delta_from_rotations
                    .x_rot(math::to_radians(x_rot_delta as f64) as f32)
                    .y_rot(math::to_radians(y_rot_delta as f64) as f32);
            }

            let new_delta = Vec3::new(
                apply_change(
                    new_delta_from_rotations.x,
                    p.relative.delta_x,
                    p.change.delta.x,
                ),
                apply_change(
                    new_delta_from_rotations.y,
                    p.relative.delta_y,
                    p.change.delta.y,
                ),
                apply_change(
                    new_delta_from_rotations.z,
                    p.relative.delta_z,
                    p.change.delta.z,
                ),
            );

            // apply the updates

            physics.velocity = new_delta;

            (direction.y_rot, direction.x_rot) = (new_y_rot, new_x_rot);

            let new_pos = Vec3::new(new_x, new_y, new_z);
            if new_pos != **position {
                **position = new_pos;
            }

            // old_pos is set to the current position when we're teleported
            physics.set_old_pos(&position);

            // send the relevant packets

            send_packet_events.send(SendPacketEvent::new(
                self.player,
                ServerboundAcceptTeleportation { id: p.id },
            ));
            send_packet_events.send(SendPacketEvent::new(
                self.player,
                ServerboundMovePlayerPosRot {
                    pos: new_pos,
                    look_direction: LookDirection::new(new_y_rot, new_x_rot),
                    // this is always false
                    on_ground: false,
                },
            ));
        });
    }

    pub fn handle_player_info_update(&mut self, p: &ClientboundPlayerInfoUpdate) {
        debug!("Got player info packet {p:?}");

        as_system::<(
            Query<&mut TabList>,
            EventWriter<AddPlayerEvent>,
            EventWriter<UpdatePlayerEvent>,
            ResMut<TabList>,
        )>(
            self.ecs,
            |(
                mut query,
                mut add_player_events,
                mut update_player_events,
                mut tab_list_resource,
            )| {
                let mut tab_list = query.get_mut(self.player).unwrap();

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
                            entity: self.player,
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
                            info.display_name.clone_from(&updated_info.display_name);
                        }
                        update_player_events.send(UpdatePlayerEvent {
                            entity: self.player,
                            info: info.clone(),
                        });
                    } else {
                        let uuid = updated_info.profile.uuid;
                        debug!("Ignoring PlayerInfoUpdate for unknown player {uuid}");
                    }
                }

                *tab_list_resource = tab_list.clone();
            },
        );
    }

    pub fn handle_player_info_remove(&mut self, p: &ClientboundPlayerInfoRemove) {
        debug!("Got chunk cache center packet {p:?}");

        as_system::<(
            Query<&mut TabList>,
            EventWriter<RemovePlayerEvent>,
            ResMut<TabList>,
        )>(
            self.ecs,
            |(mut query, mut remove_player_events, mut tab_list_resource)| {
                let mut tab_list = query.get_mut(self.player).unwrap();

                for uuid in &p.profile_ids {
                    if let Some(info) = tab_list.remove(uuid) {
                        remove_player_events.send(RemovePlayerEvent {
                            entity: self.player,
                            info,
                        });
                    }
                    tab_list_resource.remove(uuid);
                }
            },
        );
    }

    pub fn handle_set_chunk_cache_center(&mut self, p: &ClientboundSetChunkCacheCenter) {
        debug!("Got chunk cache center packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let instance_holder = query.get_mut(self.player).unwrap();
            let mut partial_world = instance_holder.partial_instance.write();

            partial_world
                .chunks
                .update_view_center(ChunkPos::new(p.x, p.z));
        });
    }

    pub fn handle_light_update(&mut self, _p: &ClientboundLightUpdate) {
        // debug!("Got light update packet {p:?}");
    }

    pub fn handle_level_chunk_with_light(&mut self, p: &ClientboundLevelChunkWithLight) {
        debug!("Got chunk with light packet {} {}", p.x, p.z);

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ReceiveChunkEvent {
                entity: self.player,
                packet: p.clone(),
            });
        });
    }

    pub fn handle_add_entity(&mut self, p: &ClientboundAddEntity) {
        debug!("Got add entity packet {p:?}");

        as_system::<(
            Commands,
            Query<(&mut EntityIdIndex, Option<&InstanceName>, Option<&TabList>)>,
            Query<&mut LoadedBy>,
            Query<Entity>,
            Res<InstanceContainer>,
            ResMut<EntityUuidIndex>,
        )>(
            self.ecs,
            |(
                mut commands,
                mut query,
                mut loaded_by_query,
                entity_query,
                instance_container,
                mut entity_uuid_index,
            )| {
                let (mut entity_id_index, instance_name, tab_list) =
                    query.get_mut(self.player).unwrap();

                let entity_id = MinecraftEntityId(p.id);

                let Some(instance_name) = instance_name else {
                    warn!("got add player packet but we haven't gotten a login packet yet");
                    return;
                };

                // check if the entity already exists, and if it does then only add to LoadedBy
                let instance = instance_container.get(instance_name).unwrap();
                if let Some(&ecs_entity) = instance.read().entity_by_id.get(&entity_id) {
                    // entity already exists
                    let Ok(mut loaded_by) = loaded_by_query.get_mut(ecs_entity) else {
                        // LoadedBy for this entity isn't in the ecs! figure out what went wrong
                        // and print an error

                        let entity_in_ecs = entity_query.get(ecs_entity).is_ok();

                        if entity_in_ecs {
                            error!("LoadedBy for entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id");
                        } else {
                            error!("Entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id");
                        }
                        return;
                    };
                    loaded_by.insert(self.player);

                    // per-client id index
                    entity_id_index.insert(entity_id, ecs_entity);

                    debug!("added to LoadedBy of entity {ecs_entity:?} with id {entity_id:?}");
                    return;
                };

                // entity doesn't exist in the global index!

                let bundle = p.as_entity_bundle((**instance_name).clone());
                let mut spawned =
                    commands.spawn((entity_id, LoadedBy(HashSet::from([self.player])), bundle));
                let ecs_entity: Entity = spawned.id();
                debug!("spawned entity {ecs_entity:?} with id {entity_id:?}");

                azalea_entity::indexing::add_entity_to_indexes(
                    entity_id,
                    ecs_entity,
                    Some(p.uuid),
                    &mut entity_id_index,
                    &mut entity_uuid_index,
                    &mut instance.write(),
                );

                // add the GameProfileComponent if the uuid is in the tab list
                if let Some(tab_list) = tab_list {
                    // (technically this makes it possible for non-player entities to have
                    // GameProfileComponents but the server would have to be doing something
                    // really weird)
                    if let Some(player_info) = tab_list.get(&p.uuid) {
                        spawned.insert(GameProfileComponent(player_info.profile.clone()));
                    }
                }

                // the bundle doesn't include the default entity metadata so we add that
                // separately
                p.apply_metadata(&mut spawned);
            },
        );
    }

    pub fn handle_set_entity_data(&mut self, p: &ClientboundSetEntityData) {
        debug!("Got set entity data packet {p:?}");

        as_system::<(
            Commands,
            Query<(&EntityIdIndex, &InstanceHolder, Option<&EntityKind>)>,
        )>(self.ecs, |(mut commands, query)| {
            let (entity_id_index, instance_holder, entity_kind) = query.get(self.player).unwrap();

            let entity = entity_id_index.get(MinecraftEntityId(p.id));

            let Some(entity) = entity else {
                // some servers like hypixel trigger this a lot :(
                debug!(
                    "Server sent an entity data packet for an entity id ({}) that we don't know about",
                    p.id
                );
                return;
            };
            let entity_kind =
                *entity_kind.expect("EntityKind component should always be present for entities");

            let packed_items = p.packed_items.clone().to_vec();

            // we use RelativeEntityUpdate because it makes sure changes aren't made
            // multiple times
            commands.entity(entity).queue(RelativeEntityUpdate {
                partial_world: instance_holder.partial_instance.clone(),
                update: Box::new(move |entity| {
                    let entity_id = entity.id();
                    entity.world_scope(|world| {
                        let mut commands_system_state = SystemState::<Commands>::new(world);
                        let mut commands = commands_system_state.get_mut(world);
                        let mut entity_commands = commands.entity(entity_id);
                        if let Err(e) =
                            apply_metadata(&mut entity_commands, *entity_kind, packed_items)
                        {
                            warn!("{e}");
                        }
                        commands_system_state.apply(world);
                    });
                }),
            });
        });
    }

    pub fn handle_update_attributes(&mut self, _p: &ClientboundUpdateAttributes) {
        // debug!("Got update attributes packet {p:?}");
    }

    pub fn handle_set_entity_motion(&mut self, p: &ClientboundSetEntityMotion) {
        // vanilla servers use this packet for knockback, but note that the Explode
        // packet is also sometimes used by servers for knockback

        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, query)| {
                let (entity_id_index, instance_holder) = query.get(self.player).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    // note that this log (and some other ones like the one in RemoveEntities)
                    // sometimes happens when killing mobs. it seems to be a vanilla bug, which is
                    // why it's a debug log instead of a warning
                    debug!(
                        "Got set entity motion packet for unknown entity id {}",
                        p.id
                    );
                    return;
                };

                // this is to make sure the same entity velocity update doesn't get sent
                // multiple times when in swarms

                let knockback = KnockbackType::Set(Vec3 {
                    x: p.xa as f64 / 8000.,
                    y: p.ya as f64 / 8000.,
                    z: p.za as f64 / 8000.,
                });

                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        entity_mut.world_scope(|world| {
                            world.send_event(KnockbackEvent { entity, knockback })
                        });
                    }),
                });
            },
        );
    }

    pub fn handle_set_entity_link(&mut self, p: &ClientboundSetEntityLink) {
        debug!("Got set entity link packet {p:?}");
    }

    pub fn handle_initialize_border(&mut self, p: &ClientboundInitializeBorder) {
        debug!("Got initialize border packet {p:?}");
    }

    pub fn handle_set_time(&mut self, _p: &ClientboundSetTime) {
        // debug!("Got set time packet {p:?}");
    }

    pub fn handle_set_default_spawn_position(&mut self, p: &ClientboundSetDefaultSpawnPosition) {
        debug!("Got set default spawn position packet {p:?}");
    }

    pub fn handle_set_health(&mut self, p: &ClientboundSetHealth) {
        debug!("Got set health packet {p:?}");

        as_system::<Query<(&mut Health, &mut Hunger)>>(self.ecs, |mut query| {
            let (mut health, mut hunger) = query.get_mut(self.player).unwrap();

            **health = p.health;
            (hunger.food, hunger.saturation) = (p.food, p.saturation);

            // the `Dead` component is added by the `update_dead` system
            // in azalea-world and then the `dead_event` system fires
            // the Death event.
        });
    }

    pub fn handle_set_experience(&mut self, p: &ClientboundSetExperience) {
        debug!("Got set experience packet {p:?}");
    }

    pub fn handle_teleport_entity(&mut self, p: &ClientboundTeleportEntity) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    warn!("Got teleport entity packet for unknown entity id {}", p.id);
                    return;
                };

                let new_pos = p.change.pos;
                let new_look_direction = LookDirection {
                    x_rot: (p.change.look_direction.x_rot as i32 * 360) as f32 / 256.,
                    y_rot: (p.change.look_direction.y_rot as i32 * 360) as f32 / 256.,
                };
                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity| {
                        let mut position = entity.get_mut::<Position>().unwrap();
                        if new_pos != **position {
                            **position = new_pos;
                        }
                        let position = *position;
                        let mut look_direction = entity.get_mut::<LookDirection>().unwrap();
                        if new_look_direction != *look_direction {
                            *look_direction = new_look_direction;
                        }
                        // old_pos is set to the current position when we're teleported
                        let mut physics = entity.get_mut::<Physics>().unwrap();
                        physics.set_old_pos(&position);
                    }),
                });
            },
        );
    }

    pub fn handle_update_advancements(&mut self, p: &ClientboundUpdateAdvancements) {
        debug!("Got update advancements packet {p:?}");
    }

    pub fn handle_rotate_head(&mut self, _p: &ClientboundRotateHead) {}

    pub fn handle_move_entity_pos(&mut self, p: &ClientboundMoveEntityPos) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                debug!("Got move entity pos packet {p:?}");

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.entity_id)) else {
                    debug!(
                        "Got move entity pos packet for unknown entity id {}",
                        p.entity_id
                    );
                    return;
                };

                let new_delta = p.delta.clone();
                let new_on_ground = p.on_ground;
                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                        let new_pos = physics.vec_delta_codec.decode(
                            new_delta.xa as i64,
                            new_delta.ya as i64,
                            new_delta.za as i64,
                        );
                        physics.vec_delta_codec.set_base(new_pos);
                        physics.set_on_ground(new_on_ground);

                        let mut position = entity_mut.get_mut::<Position>().unwrap();
                        if new_pos != **position {
                            **position = new_pos;
                        }
                    }),
                });
            },
        );
    }

    pub fn handle_move_entity_pos_rot(&mut self, p: &ClientboundMoveEntityPosRot) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                debug!("Got move entity pos rot packet {p:?}");

                let entity = entity_id_index.get(MinecraftEntityId(p.entity_id));

                let Some(entity) = entity else {
                    // often triggered by hypixel :(
                    debug!(
                        "Got move entity pos rot packet for unknown entity id {}",
                        p.entity_id
                    );
                    return;
                };

                let new_delta = p.delta.clone();
                let new_look_direction = LookDirection {
                    x_rot: (p.x_rot as i32 * 360) as f32 / 256.,
                    y_rot: (p.y_rot as i32 * 360) as f32 / 256.,
                };

                let new_on_ground = p.on_ground;

                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                        let new_pos = physics.vec_delta_codec.decode(
                            new_delta.xa as i64,
                            new_delta.ya as i64,
                            new_delta.za as i64,
                        );
                        physics.vec_delta_codec.set_base(new_pos);
                        physics.set_on_ground(new_on_ground);

                        let mut position = entity_mut.get_mut::<Position>().unwrap();
                        if new_pos != **position {
                            **position = new_pos;
                        }

                        let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                        if new_look_direction != *look_direction {
                            *look_direction = new_look_direction;
                        }
                    }),
                });
            },
        );
    }

    pub fn handle_move_entity_rot(&mut self, p: &ClientboundMoveEntityRot) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let entity = entity_id_index.get(MinecraftEntityId(p.entity_id));
                if let Some(entity) = entity {
                    let new_look_direction = LookDirection {
                        x_rot: (p.x_rot as i32 * 360) as f32 / 256.,
                        y_rot: (p.y_rot as i32 * 360) as f32 / 256.,
                    };
                    let new_on_ground = p.on_ground;

                    commands.entity(entity).queue(RelativeEntityUpdate {
                        partial_world: instance_holder.partial_instance.clone(),
                        update: Box::new(move |entity_mut| {
                            let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                            physics.set_on_ground(new_on_ground);

                            let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                            if new_look_direction != *look_direction {
                                *look_direction = new_look_direction;
                            }
                        }),
                    });
                } else {
                    warn!(
                        "Got move entity rot packet for unknown entity id {}",
                        p.entity_id
                    );
                }
            },
        );
    }
    pub fn handle_keep_alive(&mut self, p: &ClientboundKeepAlive) {
        /*
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
        send_packet_events.send(SendPacketEvent::new(
            player_entity,
            ServerboundKeepAlive { id: p.id },
        ));
        */

        debug!("Got keep alive packet {p:?} for {:?}", self.player);

        as_system::<(EventWriter<KeepAliveEvent>, EventWriter<SendPacketEvent>)>(
            self.ecs,
            |(mut keepalive_events, mut send_packet_events)| {
                keepalive_events.send(KeepAliveEvent {
                    entity: self.player,
                    id: p.id,
                });
                send_packet_events.send(SendPacketEvent::new(
                    self.player,
                    ServerboundKeepAlive { id: p.id },
                ));
            },
        );
    }

    pub fn handle_remove_entities(&mut self, p: &ClientboundRemoveEntities) {
        debug!("Got remove entities packet {p:?}");

        as_system::<(Query<&mut EntityIdIndex>, Query<&mut LoadedBy>)>(
            self.ecs,
            |(mut query, mut entity_query)| {
                let Ok(mut entity_id_index) = query.get_mut(self.player) else {
                    warn!("our local player doesn't have EntityIdIndex");
                    return;
                };

                for &id in &p.entity_ids {
                    let Some(entity) = entity_id_index.remove(MinecraftEntityId(id)) else {
                        debug!("Tried to remove entity with id {id} but it wasn't in the EntityIdIndex");
                        continue;
                    };
                    let Ok(mut loaded_by) = entity_query.get_mut(entity) else {
                        warn!(
                            "tried to despawn entity {id} but it doesn't have a LoadedBy component",
                        );
                        continue;
                    };

                    // the [`remove_despawned_entities_from_indexes`] system will despawn the entity
                    // if it's not loaded by anything anymore

                    // also we can't just ecs.despawn because if we're in a swarm then the entity
                    // might still be loaded by another client

                    loaded_by.remove(&self.player);
                }
            },
        );
    }
    pub fn handle_player_chat(&mut self, p: &ClientboundPlayerChat) {
        debug!("Got player chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::Player(Arc::new(p.clone())),
            });
        });
    }

    pub fn handle_system_chat(&mut self, p: &ClientboundSystemChat) {
        debug!("Got system chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::System(Arc::new(p.clone())),
            });
        });
    }

    pub fn handle_disguised_chat(&mut self, p: &ClientboundDisguisedChat) {
        debug!("Got disguised chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::Disguised(Arc::new(p.clone())),
            });
        });
    }

    pub fn handle_level_event(&mut self, p: &ClientboundLevelEvent) {
        debug!("Got level event packet {p:?}");
    }

    pub fn handle_block_update(&mut self, p: &ClientboundBlockUpdate) {
        debug!("Got block update packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let local_player = query.get_mut(self.player).unwrap();

            let world = local_player.instance.write();

            world.chunks.set_block_state(&p.pos, p.block_state);
        });
    }

    pub fn handle_animate(&mut self, p: &ClientboundAnimate) {
        debug!("Got animate packet {p:?}");
    }

    pub fn handle_section_blocks_update(&mut self, p: &ClientboundSectionBlocksUpdate) {
        debug!("Got section blocks update packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let local_player = query.get_mut(self.player).unwrap();
            let world = local_player.instance.write();
            for state in &p.states {
                world
                    .chunks
                    .set_block_state(&(p.section_pos + state.pos), state.state);
            }
        });
    }

    pub fn handle_game_event(&mut self, p: &ClientboundGameEvent) {
        use azalea_protocol::packets::game::c_game_event::EventType;

        debug!("Got game event packet {p:?}");

        #[allow(clippy::single_match)]
        match p.event {
            EventType::ChangeGameMode => {
                as_system::<Query<&mut LocalGameMode>>(self.ecs, |mut query| {
                    let mut local_game_mode = query.get_mut(self.player).unwrap();
                    if let Some(new_game_mode) = GameMode::from_id(p.param as u8) {
                        local_game_mode.current = new_game_mode;
                    }
                });
            }
            _ => {}
        }
    }

    pub fn handle_level_particles(&mut self, p: &ClientboundLevelParticles) {
        debug!("Got level particles packet {p:?}");
    }

    pub fn handle_server_data(&mut self, p: &ClientboundServerData) {
        debug!("Got server data packet {p:?}");
    }

    pub fn handle_set_equipment(&mut self, p: &ClientboundSetEquipment) {
        debug!("Got set equipment packet {p:?}");
    }

    pub fn handle_update_mob_effect(&mut self, p: &ClientboundUpdateMobEffect) {
        debug!("Got update mob effect packet {p:?}");
    }

    pub fn handle_block_event(&mut self, p: &ClientboundBlockEvent) {
        debug!("Got block event packet {p:?}");
    }

    pub fn handle_container_set_content(&mut self, p: &ClientboundContainerSetContent) {
        debug!("Got container set content packet {p:?}");

        as_system::<(Query<&mut Inventory>, EventWriter<_>)>(
            self.ecs,
            |(mut query, mut events)| {
                let mut inventory = query.get_mut(self.player).unwrap();

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
                        entity: self.player,
                        slots: p.items.clone(),
                        container_id: p.container_id,
                    });
                }
            },
        );
    }

    pub fn handle_container_set_data(&mut self, p: &ClientboundContainerSetData) {
        debug!("Got container set data packet {p:?}");

        // TODO: handle ContainerSetData packet
        // this is used for various things like the furnace progress
        // bar
        // see https://wiki.vg/Protocol#Set_Container_Property

        // as_system::<Query<&mut Inventory>>(self.ecs, |mut query| {
        //     let inventory = query.get_mut(self.player).unwrap();
        // });
    }

    pub fn handle_container_set_slot(&mut self, p: &ClientboundContainerSetSlot) {
        debug!("Got container set slot packet {p:?}");

        as_system::<Query<&mut Inventory>>(self.ecs, |mut query| {
            let mut inventory = query.get_mut(self.player).unwrap();

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
                if p.container_id == 0 && azalea_inventory::Player::is_hotbar_slot(p.slot.into()) {
                    // minecraft also sets a "pop time" here which is used for an animation
                    // but that's not really necessary
                    if let Some(slot) = inventory.inventory_menu.slot_mut(p.slot.into()) {
                        *slot = p.item_stack.clone();
                    }
                } else if p.container_id == inventory.id
                    && (p.container_id != 0 || !is_creative_mode_and_inventory_closed)
                {
                    // var2.containerMenu.setItem(var4, var1.getStateId(), var3);
                    if let Some(slot) = inventory.menu_mut().slot_mut(p.slot.into()) {
                        *slot = p.item_stack.clone();
                        inventory.state_id = p.state_id;
                    }
                }
            }
        });
    }

    pub fn handle_container_close(&mut self, p: &ClientboundContainerClose) {
        // there's a container_id field in the packet, but minecraft doesn't actually
        // check it

        debug!("Got container close packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ClientSideCloseContainerEvent {
                entity: self.player,
            });
        });
    }

    pub fn handle_explode(&mut self, p: &ClientboundExplode) {
        trace!("Got explode packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut knockback_events| {
            if let Some(knockback) = p.knockback {
                knockback_events.send(KnockbackEvent {
                    entity: self.player,
                    knockback: KnockbackType::Set(knockback),
                });
            }
        });
    }

    pub fn handle_forget_level_chunk(&mut self, p: &ClientboundForgetLevelChunk) {
        debug!("Got forget level chunk packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let local_player = query.get_mut(self.player).unwrap();

            let mut partial_instance = local_player.partial_instance.write();

            partial_instance.chunks.limited_set(&p.pos, None);
        });
    }

    pub fn handle_open_screen(&mut self, p: &ClientboundOpenScreen) {
        debug!("Got open screen packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(MenuOpenedEvent {
                entity: self.player,
                window_id: p.container_id,
                menu_type: p.menu_type,
                title: p.title.to_owned(),
            });
        });
    }

    pub fn handle_ping(&mut self, p: &ClientboundPing) {
        debug!("Got ping packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(SendPacketEvent::new(
                self.player,
                ServerboundPong { id: p.id },
            ));
        });
    }

    pub fn handle_player_combat_kill(&mut self, p: &ClientboundPlayerCombatKill) {
        debug!("Got player kill packet {p:?}");

        as_system::<(
            Commands,
            Query<(&MinecraftEntityId, Option<&Dead>)>,
            EventWriter<_>,
        )>(self.ecs, |(mut commands, mut query, mut events)| {
            let (entity_id, dead) = query.get_mut(self.player).unwrap();

            if **entity_id == p.player_id && dead.is_none() {
                commands.entity(self.player).insert(Dead);
                events.send(DeathEvent {
                    entity: self.player,
                    packet: Some(p.clone()),
                });
            }
        });
    }

    pub fn handle_resource_pack_push(&mut self, p: &ClientboundResourcePackPush) {
        debug!("Got resource pack packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ResourcePackEvent {
                entity: self.player,
                id: p.id,
                url: p.url.to_owned(),
                hash: p.hash.to_owned(),
                required: p.required,
                prompt: p.prompt.to_owned(),
            });
        });
    }

    pub fn handle_respawn(&mut self, p: &ClientboundRespawn) {
        debug!("Got respawn packet {p:?}");

        as_system::<(
            Commands,
            Query<(
                &mut InstanceHolder,
                &GameProfileComponent,
                &ClientInformation,
            )>,
            EventWriter<_>,
            ResMut<InstanceContainer>,
        )>(
            self.ecs,
            |(mut commands, mut query, mut events, mut instance_container)| {
                let (mut instance_holder, game_profile, client_information) =
                    query.get_mut(self.player).unwrap();

                {
                    let new_instance_name = p.common.dimension.clone();

                    let Some(dimension_type_element) =
                        instance_holder.instance.read().registries.dimension_type()
                    else {
                        error!("Server didn't send dimension type registry, can't log in.");
                        return;
                    };

                    let dimension_name = ResourceLocation::new(&p.common.dimension.to_string());

                    let Some(dimension) = dimension_type_element.map.get(&dimension_name) else {
                        error!("No dimension_type with name {dimension_name}");
                        return;
                    };

                    // add this world to the instance_container (or don't if it's already
                    // there)
                    let weak_instance = instance_container.insert(
                        new_instance_name.clone(),
                        dimension.height,
                        dimension.min_y,
                    );
                    events.send(InstanceLoadedEvent {
                        entity: self.player,
                        name: new_instance_name.clone(),
                        instance: Arc::downgrade(&weak_instance),
                    });

                    // set the partial_world to an empty world
                    // (when we add chunks or entities those will be in the
                    // instance_container)

                    *instance_holder.partial_instance.write() = PartialInstance::new(
                        azalea_world::chunk_storage::calculate_chunk_storage_range(
                            client_information.view_distance.into(),
                        ),
                        Some(self.player),
                    );
                    instance_holder.instance = weak_instance;

                    // this resets a bunch of our components like physics and stuff
                    let entity_bundle = EntityBundle::new(
                        game_profile.uuid,
                        Vec3::default(),
                        azalea_registry::EntityKind::Player,
                        new_instance_name,
                    );
                    // update the local gamemode and metadata things
                    commands.entity(self.player).insert((
                        LocalGameMode {
                            current: p.common.game_type,
                            previous: p.common.previous_game_type.into(),
                        },
                        entity_bundle,
                    ));
                }

                // Remove the Dead marker component from the player.
                commands.entity(self.player).remove::<Dead>();
            },
        )
    }

    pub fn handle_start_configuration(&mut self, _p: &ClientboundStartConfiguration) {
        as_system::<(Commands, EventWriter<_>)>(self.ecs, |(mut commands, mut events)| {
            events.send(SendPacketEvent::new(
                self.player,
                ServerboundConfigurationAcknowledged {},
            ));

            commands
                .entity(self.player)
                .insert(crate::client::InConfigState)
                .remove::<crate::JoinedClientBundle>();
        });
    }

    pub fn handle_entity_position_sync(&mut self, p: &ClientboundEntityPositionSync) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    debug!("Got teleport entity packet for unknown entity id {}", p.id);
                    return;
                };

                let new_position = p.values.pos;
                let new_on_ground = p.on_ground;
                let new_look_direction = p.values.look_direction;

                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        let is_local_entity = entity_mut.get::<LocalEntity>().is_some();
                        let mut physics = entity_mut.get_mut::<Physics>().unwrap();

                        physics.vec_delta_codec.set_base(new_position);

                        if is_local_entity {
                            debug!("Ignoring entity position sync packet for local player");
                            return;
                        }

                        physics.set_on_ground(new_on_ground);

                        let mut last_sent_position =
                            entity_mut.get_mut::<LastSentPosition>().unwrap();
                        **last_sent_position = new_position;
                        let mut position = entity_mut.get_mut::<Position>().unwrap();
                        **position = new_position;

                        let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                        *look_direction = new_look_direction;
                    }),
                });
            },
        );
    }
}

pub struct GamePacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
}

pub fn as_system<T>(ecs: &mut World, f: impl FnOnce(T::Item<'_, '_>))
where
    T: SystemParam + 'static,
{
    let mut system_state = SystemState::<T>::new(ecs);
    let values = system_state.get_mut(ecs);
    f(values);
    system_state.apply(ecs);
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

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendPacketEvent>,
    mut query: Query<&mut RawConnection>,
) {
    for event in send_packet_events.read() {
        if let Ok(raw_connection) = query.get_mut(event.sent_by) {
            // debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_connection.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}
