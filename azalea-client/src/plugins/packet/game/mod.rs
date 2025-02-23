mod events;

use std::{collections::HashSet, ops::Add, sync::Arc};

use azalea_core::{
    game_type::GameMode,
    math,
    position::{ChunkPos, Vec3},
};
use azalea_entity::{
    Dead, EntityBundle, EntityKind, LastSentPosition, LoadedBy, LocalEntity, LookDirection,
    Physics, Position, RelativeEntityUpdate,
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::{Health, apply_metadata},
};
use azalea_protocol::packets::game::*;
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_ecs::{prelude::*, system::SystemState};
pub use events::*;
use tracing::{debug, error, trace, warn};

use crate::{
    ClientInformation, PlayerInfo,
    chat::{ChatPacket, ChatReceivedEvent},
    chunks, declare_packet_handlers,
    disconnect::DisconnectEvent,
    inventory::{
        ClientSideCloseContainerEvent, Inventory, MenuOpenedEvent, SetContainerContentEvent,
    },
    local_player::{
        GameProfileComponent, Hunger, InstanceHolder, LocalGameMode, PlayerAbilities, TabList,
    },
    movement::{KnockbackEvent, KnockbackType},
    packet::as_system,
};

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::<(Entity, Arc<ClientboundGamePacket>)>::new();

    {
        let mut system_state = SystemState::<EventReader<ReceivePacketEvent>>::new(ecs);
        let mut events = system_state.get_mut(ecs);
        for ReceivePacketEvent {
            entity: player_entity,
            packet,
        } in events.read()
        {
            // we do this so `ecs` isn't borrowed for the whole loop
            events_owned.push((*player_entity, packet.clone()));
        }
    }

    for (player_entity, packet) in events_owned {
        let mut handler = GamePacketHandler {
            player: player_entity,
            ecs,
        };

        declare_packet_handlers!(
            ClientboundGamePacket,
            packet.as_ref(),
            handler,
            [
                login,
                set_chunk_cache_radius,
                chunk_batch_start,
                chunk_batch_finished,
                custom_payload,
                change_difficulty,
                commands,
                player_abilities,
                set_cursor_item,
                update_tags,
                disconnect,
                update_recipes,
                entity_event,
                player_position,
                player_info_update,
                player_info_remove,
                set_chunk_cache_center,
                chunks_biomes,
                light_update,
                level_chunk_with_light,
                add_entity,
                set_entity_data,
                update_attributes,
                set_entity_motion,
                set_entity_link,
                initialize_border,
                set_time,
                set_default_spawn_position,
                set_health,
                set_experience,
                teleport_entity,
                update_advancements,
                rotate_head,
                move_entity_pos,
                move_entity_pos_rot,
                move_entity_rot,
                keep_alive,
                remove_entities,
                player_chat,
                system_chat,
                disguised_chat,
                sound,
                level_event,
                block_update,
                animate,
                section_blocks_update,
                game_event,
                level_particles,
                server_data,
                set_equipment,
                update_mob_effect,
                add_experience_orb,
                award_stats,
                block_changed_ack,
                block_destruction,
                block_entity_data,
                block_event,
                boss_event,
                command_suggestions,
                container_set_content,
                container_set_data,
                container_set_slot,
                container_close,
                cooldown,
                custom_chat_completions,
                delete_chat,
                explode,
                forget_level_chunk,
                horse_screen_open,
                map_item_data,
                merchant_offers,
                move_vehicle,
                open_book,
                open_screen,
                open_sign_editor,
                ping,
                place_ghost_recipe,
                player_combat_end,
                player_combat_enter,
                player_combat_kill,
                player_look_at,
                remove_mob_effect,
                resource_pack_push,
                resource_pack_pop,
                respawn,
                start_configuration,
                entity_position_sync,
                select_advancements_tab,
                set_action_bar_text,
                set_border_center,
                set_border_lerp_size,
                set_border_size,
                set_border_warning_delay,
                set_border_warning_distance,
                set_camera,
                set_display_objective,
                set_objective,
                set_passengers,
                set_player_team,
                set_score,
                set_simulation_distance,
                set_subtitle_text,
                set_title_text,
                set_titles_animation,
                clear_titles,
                sound_entity,
                stop_sound,
                tab_list,
                tag_query,
                take_item_entity,
                bundle_delimiter,
                damage_event,
                hurt_animation,
                ticking_state,
                ticking_step,
                reset_score,
                cookie_request,
                debug_sample,
                pong_response,
                store_cookie,
                transfer,
                move_minecart_along_track,
                set_held_slot,
                set_player_inventory,
                projectile_power,
                custom_report_details,
                server_links,
                player_rotation,
                recipe_book_add,
                recipe_book_remove,
                recipe_book_settings,
            ]
        );
    }
}

pub struct GamePacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
}
impl GamePacketHandler<'_> {
    pub fn login(&mut self, p: &ClientboundLogin) {
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

                let new_instance_name = p.common.dimension.clone();

                if let Some(mut instance_name) = instance_name {
                    *instance_name = instance_name.clone();
                } else {
                    commands
                        .entity(self.player)
                        .insert(InstanceName(new_instance_name.clone()));
                }

                let Some((_dimension_type, dimension_data)) = p
                    .common
                    .dimension_type(&instance_holder.instance.read().registries)
                else {
                    return;
                };

                // add this world to the instance_container (or don't if it's already
                // there)
                let weak_instance = instance_container.insert(
                    new_instance_name.clone(),
                    dimension_data.height,
                    dimension_data.min_y,
                    &instance_holder.instance.read().registries,
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
                let entity_id = p.player_id;
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

                // send the client information that we have set
                debug!(
                    "Sending client information because login: {:?}",
                    client_information
                );
                send_packet_events.send(SendPacketEvent::new(self.player,
                    azalea_protocol::packets::game::s_client_information::ServerboundClientInformation { information: client_information.clone() },
                ));
            },
        );
    }

    pub fn set_chunk_cache_radius(&mut self, p: &ClientboundSetChunkCacheRadius) {
        debug!("Got set chunk cache radius packet {p:?}");
    }

    pub fn chunk_batch_start(&mut self, _p: &ClientboundChunkBatchStart) {
        // the packet is empty, it's just a marker to tell us when the batch starts and
        // ends
        debug!("Got chunk batch start");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ChunkBatchStartEvent {
                entity: self.player,
            });
        });
    }

    pub fn chunk_batch_finished(&mut self, p: &ClientboundChunkBatchFinished) {
        debug!("Got chunk batch finished {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ChunkBatchFinishedEvent {
                entity: self.player,
                batch_size: p.batch_size,
            });
        });
    }

    pub fn custom_payload(&mut self, p: &ClientboundCustomPayload) {
        debug!("Got custom payload packet {p:?}");
    }

    pub fn change_difficulty(&mut self, p: &ClientboundChangeDifficulty) {
        debug!("Got difficulty packet {p:?}");
    }

    pub fn commands(&mut self, _p: &ClientboundCommands) {
        debug!("Got declare commands packet");
    }

    pub fn player_abilities(&mut self, p: &ClientboundPlayerAbilities) {
        debug!("Got player abilities packet {p:?}");

        as_system::<Query<&mut PlayerAbilities>>(self.ecs, |mut query| {
            let mut player_abilities = query.get_mut(self.player).unwrap();

            *player_abilities = PlayerAbilities::from(p);
        });
    }

    pub fn set_cursor_item(&mut self, p: &ClientboundSetCursorItem) {
        debug!("Got set cursor item packet {p:?}");
    }

    pub fn update_tags(&mut self, _p: &ClientboundUpdateTags) {
        debug!("Got update tags packet");
    }

    pub fn disconnect(&mut self, p: &ClientboundDisconnect) {
        warn!("Got disconnect packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason.clone()),
            });
        });
    }

    pub fn update_recipes(&mut self, _p: &ClientboundUpdateRecipes) {
        debug!("Got update recipes packet");
    }

    pub fn entity_event(&mut self, _p: &ClientboundEntityEvent) {
        // debug!("Got entity event packet {p:?}");
    }

    pub fn player_position(&mut self, p: &ClientboundPlayerPosition) {
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
                if condition { base + change } else { change }
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

    pub fn player_info_update(&mut self, p: &ClientboundPlayerInfoUpdate) {
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

    pub fn player_info_remove(&mut self, p: &ClientboundPlayerInfoRemove) {
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

    pub fn set_chunk_cache_center(&mut self, p: &ClientboundSetChunkCacheCenter) {
        debug!("Got chunk cache center packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let instance_holder = query.get_mut(self.player).unwrap();
            let mut partial_world = instance_holder.partial_instance.write();

            partial_world
                .chunks
                .update_view_center(ChunkPos::new(p.x, p.z));
        });
    }

    pub fn chunks_biomes(&mut self, _p: &ClientboundChunksBiomes) {}

    pub fn light_update(&mut self, _p: &ClientboundLightUpdate) {
        // debug!("Got light update packet {p:?}");
    }

    pub fn level_chunk_with_light(&mut self, p: &ClientboundLevelChunkWithLight) {
        debug!("Got chunk with light packet {} {}", p.x, p.z);

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(chunks::ReceiveChunkEvent {
                entity: self.player,
                packet: p.clone(),
            });
        });
    }

    pub fn add_entity(&mut self, p: &ClientboundAddEntity) {
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

                let entity_id = p.id;

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
                            error!(
                                "LoadedBy for entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id"
                            );
                        } else {
                            error!(
                                "Entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id"
                            );
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

    pub fn set_entity_data(&mut self, p: &ClientboundSetEntityData) {
        as_system::<(
            Commands,
            Query<(&EntityIdIndex, &InstanceHolder)>,
            // this is a separate query since it's applied on the entity id that's being updated
            // instead of the player that received the packet
            Query<&EntityKind>,
        )>(self.ecs, |(mut commands, query, entity_kind_query)| {
            let (entity_id_index, instance_holder) = query.get(self.player).unwrap();

            let entity = entity_id_index.get(p.id);

            let Some(entity) = entity else {
                // some servers like hypixel trigger this a lot :(
                debug!(
                    "Server sent an entity data packet for an entity id ({}) that we don't know about",
                    p.id
                );
                return;
            };

            let entity_kind = *entity_kind_query
                .get(entity)
                .expect("EntityKind component should always be present for entities");

            debug!("Got set entity data packet {p:?} for entity of kind {entity_kind:?}");

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

    pub fn update_attributes(&mut self, _p: &ClientboundUpdateAttributes) {
        // debug!("Got update attributes packet {p:?}");
    }

    pub fn set_entity_motion(&mut self, p: &ClientboundSetEntityMotion) {
        // vanilla servers use this packet for knockback, but note that the Explode
        // packet is also sometimes used by servers for knockback

        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, query)| {
                let (entity_id_index, instance_holder) = query.get(self.player).unwrap();

                let Some(entity) = entity_id_index.get(p.id) else {
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
                    x: p.delta.xa as f64 / 8000.,
                    y: p.delta.ya as f64 / 8000.,
                    z: p.delta.za as f64 / 8000.,
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

    pub fn set_entity_link(&mut self, p: &ClientboundSetEntityLink) {
        debug!("Got set entity link packet {p:?}");
    }

    pub fn initialize_border(&mut self, p: &ClientboundInitializeBorder) {
        debug!("Got initialize border packet {p:?}");
    }

    pub fn set_time(&mut self, _p: &ClientboundSetTime) {
        // debug!("Got set time packet {p:?}");
    }

    pub fn set_default_spawn_position(&mut self, p: &ClientboundSetDefaultSpawnPosition) {
        debug!("Got set default spawn position packet {p:?}");
    }

    pub fn set_health(&mut self, p: &ClientboundSetHealth) {
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

    pub fn set_experience(&mut self, p: &ClientboundSetExperience) {
        debug!("Got set experience packet {p:?}");
    }

    pub fn teleport_entity(&mut self, p: &ClientboundTeleportEntity) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let Some(entity) = entity_id_index.get(p.id) else {
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

    pub fn update_advancements(&mut self, p: &ClientboundUpdateAdvancements) {
        debug!("Got update advancements packet {p:?}");
    }

    pub fn rotate_head(&mut self, _p: &ClientboundRotateHead) {}

    pub fn move_entity_pos(&mut self, p: &ClientboundMoveEntityPos) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                debug!("Got move entity pos packet {p:?}");

                let Some(entity) = entity_id_index.get(p.entity_id) else {
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

    pub fn move_entity_pos_rot(&mut self, p: &ClientboundMoveEntityPosRot) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                debug!("Got move entity pos rot packet {p:?}");

                let entity = entity_id_index.get(p.entity_id);

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

    pub fn move_entity_rot(&mut self, p: &ClientboundMoveEntityRot) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let entity = entity_id_index.get(p.entity_id);
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
    pub fn keep_alive(&mut self, p: &ClientboundKeepAlive) {
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

    pub fn remove_entities(&mut self, p: &ClientboundRemoveEntities) {
        debug!("Got remove entities packet {p:?}");

        as_system::<(Query<&mut EntityIdIndex>, Query<&mut LoadedBy>)>(
            self.ecs,
            |(mut query, mut entity_query)| {
                let Ok(mut entity_id_index) = query.get_mut(self.player) else {
                    warn!("our local player doesn't have EntityIdIndex");
                    return;
                };

                for &id in &p.entity_ids {
                    let Some(entity) = entity_id_index.remove(id) else {
                        debug!(
                            "Tried to remove entity with id {id} but it wasn't in the EntityIdIndex"
                        );
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
    pub fn player_chat(&mut self, p: &ClientboundPlayerChat) {
        debug!("Got player chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::Player(Arc::new(p.clone())),
            });
        });
    }

    pub fn system_chat(&mut self, p: &ClientboundSystemChat) {
        debug!("Got system chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::System(Arc::new(p.clone())),
            });
        });
    }

    pub fn disguised_chat(&mut self, p: &ClientboundDisguisedChat) {
        debug!("Got disguised chat packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ChatReceivedEvent {
                entity: self.player,
                packet: ChatPacket::Disguised(Arc::new(p.clone())),
            });
        });
    }

    pub fn sound(&mut self, _p: &ClientboundSound) {}

    pub fn level_event(&mut self, p: &ClientboundLevelEvent) {
        debug!("Got level event packet {p:?}");
    }

    pub fn block_update(&mut self, p: &ClientboundBlockUpdate) {
        debug!("Got block update packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let local_player = query.get_mut(self.player).unwrap();

            let world = local_player.instance.write();

            world.chunks.set_block_state(&p.pos, p.block_state);
        });
    }

    pub fn animate(&mut self, p: &ClientboundAnimate) {
        debug!("Got animate packet {p:?}");
    }

    pub fn section_blocks_update(&mut self, p: &ClientboundSectionBlocksUpdate) {
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

    pub fn game_event(&mut self, p: &ClientboundGameEvent) {
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

    pub fn level_particles(&mut self, p: &ClientboundLevelParticles) {
        debug!("Got level particles packet {p:?}");
    }

    pub fn server_data(&mut self, p: &ClientboundServerData) {
        debug!("Got server data packet {p:?}");
    }

    pub fn set_equipment(&mut self, p: &ClientboundSetEquipment) {
        debug!("Got set equipment packet {p:?}");
    }

    pub fn update_mob_effect(&mut self, p: &ClientboundUpdateMobEffect) {
        debug!("Got update mob effect packet {p:?}");
    }

    pub fn add_experience_orb(&mut self, _p: &ClientboundAddExperienceOrb) {}

    pub fn award_stats(&mut self, _p: &ClientboundAwardStats) {}

    pub fn block_changed_ack(&mut self, _p: &ClientboundBlockChangedAck) {}

    pub fn block_destruction(&mut self, _p: &ClientboundBlockDestruction) {}

    pub fn block_entity_data(&mut self, _p: &ClientboundBlockEntityData) {}

    pub fn block_event(&mut self, p: &ClientboundBlockEvent) {
        debug!("Got block event packet {p:?}");
    }

    pub fn boss_event(&mut self, _p: &ClientboundBossEvent) {}

    pub fn command_suggestions(&mut self, _p: &ClientboundCommandSuggestions) {}

    pub fn container_set_content(&mut self, p: &ClientboundContainerSetContent) {
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

    pub fn container_set_data(&mut self, p: &ClientboundContainerSetData) {
        debug!("Got container set data packet {p:?}");

        // TODO: handle ContainerSetData packet
        // this is used for various things like the furnace progress
        // bar
        // see https://wiki.vg/Protocol#Set_Container_Property

        // as_system::<Query<&mut Inventory>>(self.ecs, |mut query| {
        //     let inventory = query.get_mut(self.player).unwrap();
        // });
    }

    pub fn container_set_slot(&mut self, p: &ClientboundContainerSetSlot) {
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

    pub fn container_close(&mut self, p: &ClientboundContainerClose) {
        // there's a container_id field in the packet, but minecraft doesn't actually
        // check it

        debug!("Got container close packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(ClientSideCloseContainerEvent {
                entity: self.player,
            });
        });
    }

    pub fn cooldown(&mut self, _p: &ClientboundCooldown) {}

    pub fn custom_chat_completions(&mut self, _p: &ClientboundCustomChatCompletions) {}

    pub fn delete_chat(&mut self, _p: &ClientboundDeleteChat) {}

    pub fn explode(&mut self, p: &ClientboundExplode) {
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

    pub fn forget_level_chunk(&mut self, p: &ClientboundForgetLevelChunk) {
        debug!("Got forget level chunk packet {p:?}");

        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let local_player = query.get_mut(self.player).unwrap();

            let mut partial_instance = local_player.partial_instance.write();

            partial_instance.chunks.limited_set(&p.pos, None);
        });
    }

    pub fn horse_screen_open(&mut self, _p: &ClientboundHorseScreenOpen) {}

    pub fn map_item_data(&mut self, _p: &ClientboundMapItemData) {}

    pub fn merchant_offers(&mut self, _p: &ClientboundMerchantOffers) {}

    pub fn move_vehicle(&mut self, _p: &ClientboundMoveVehicle) {}

    pub fn open_book(&mut self, _p: &ClientboundOpenBook) {}

    pub fn open_screen(&mut self, p: &ClientboundOpenScreen) {
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

    pub fn open_sign_editor(&mut self, _p: &ClientboundOpenSignEditor) {}

    pub fn ping(&mut self, p: &ClientboundPing) {
        debug!("Got ping packet {p:?}");

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(SendPacketEvent::new(
                self.player,
                ServerboundPong { id: p.id },
            ));
        });
    }

    pub fn place_ghost_recipe(&mut self, _p: &ClientboundPlaceGhostRecipe) {}

    pub fn player_combat_end(&mut self, _p: &ClientboundPlayerCombatEnd) {}

    pub fn player_combat_enter(&mut self, _p: &ClientboundPlayerCombatEnter) {}

    pub fn player_combat_kill(&mut self, p: &ClientboundPlayerCombatKill) {
        debug!("Got player kill packet {p:?}");

        as_system::<(
            Commands,
            Query<(&MinecraftEntityId, Option<&Dead>)>,
            EventWriter<_>,
        )>(self.ecs, |(mut commands, mut query, mut events)| {
            let (entity_id, dead) = query.get_mut(self.player).unwrap();

            if *entity_id == p.player_id && dead.is_none() {
                commands.entity(self.player).insert(Dead);
                events.send(DeathEvent {
                    entity: self.player,
                    packet: Some(p.clone()),
                });
            }
        });
    }

    pub fn player_look_at(&mut self, _p: &ClientboundPlayerLookAt) {}

    pub fn remove_mob_effect(&mut self, _p: &ClientboundRemoveMobEffect) {}

    pub fn resource_pack_push(&mut self, p: &ClientboundResourcePackPush) {
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

    pub fn resource_pack_pop(&mut self, _p: &ClientboundResourcePackPop) {}

    pub fn respawn(&mut self, p: &ClientboundRespawn) {
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

                let new_instance_name = p.common.dimension.clone();

                let Some((_dimension_type, dimension_data)) = p
                    .common
                    .dimension_type(&instance_holder.instance.read().registries)
                else {
                    return;
                };

                // add this world to the instance_container (or don't if it's already
                // there)
                let weak_instance = instance_container.insert(
                    new_instance_name.clone(),
                    dimension_data.height,
                    dimension_data.min_y,
                    &instance_holder.instance.read().registries,
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

                // Remove the Dead marker component from the player.
                commands.entity(self.player).remove::<Dead>();
            },
        )
    }

    pub fn start_configuration(&mut self, _p: &ClientboundStartConfiguration) {
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

    pub fn entity_position_sync(&mut self, p: &ClientboundEntityPositionSync) {
        as_system::<(Commands, Query<(&EntityIdIndex, &InstanceHolder)>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let (entity_id_index, instance_holder) = query.get_mut(self.player).unwrap();

                let Some(entity) = entity_id_index.get(p.id) else {
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

    pub fn select_advancements_tab(&mut self, _p: &ClientboundSelectAdvancementsTab) {}
    pub fn set_action_bar_text(&mut self, _p: &ClientboundSetActionBarText) {}
    pub fn set_border_center(&mut self, _p: &ClientboundSetBorderCenter) {}
    pub fn set_border_lerp_size(&mut self, _p: &ClientboundSetBorderLerpSize) {}
    pub fn set_border_size(&mut self, _p: &ClientboundSetBorderSize) {}
    pub fn set_border_warning_delay(&mut self, _p: &ClientboundSetBorderWarningDelay) {}
    pub fn set_border_warning_distance(&mut self, _p: &ClientboundSetBorderWarningDistance) {}
    pub fn set_camera(&mut self, _p: &ClientboundSetCamera) {}
    pub fn set_display_objective(&mut self, _p: &ClientboundSetDisplayObjective) {}
    pub fn set_objective(&mut self, _p: &ClientboundSetObjective) {}
    pub fn set_passengers(&mut self, _p: &ClientboundSetPassengers) {}
    pub fn set_player_team(&mut self, _p: &ClientboundSetPlayerTeam) {}
    pub fn set_score(&mut self, _p: &ClientboundSetScore) {}
    pub fn set_simulation_distance(&mut self, _p: &ClientboundSetSimulationDistance) {}
    pub fn set_subtitle_text(&mut self, _p: &ClientboundSetSubtitleText) {}
    pub fn set_title_text(&mut self, _p: &ClientboundSetTitleText) {}
    pub fn set_titles_animation(&mut self, _p: &ClientboundSetTitlesAnimation) {}
    pub fn clear_titles(&mut self, _p: &ClientboundClearTitles) {}
    pub fn sound_entity(&mut self, _p: &ClientboundSoundEntity) {}
    pub fn stop_sound(&mut self, _p: &ClientboundStopSound) {}
    pub fn tab_list(&mut self, _p: &ClientboundTabList) {}
    pub fn tag_query(&mut self, _p: &ClientboundTagQuery) {}
    pub fn take_item_entity(&mut self, _p: &ClientboundTakeItemEntity) {}
    pub fn bundle_delimiter(&mut self, _p: &ClientboundBundleDelimiter) {}
    pub fn damage_event(&mut self, _p: &ClientboundDamageEvent) {}
    pub fn hurt_animation(&mut self, _p: &ClientboundHurtAnimation) {}
    pub fn ticking_state(&mut self, _p: &ClientboundTickingState) {}
    pub fn ticking_step(&mut self, _p: &ClientboundTickingStep) {}
    pub fn reset_score(&mut self, _p: &ClientboundResetScore) {}
    pub fn cookie_request(&mut self, _p: &ClientboundCookieRequest) {}
    pub fn debug_sample(&mut self, _p: &ClientboundDebugSample) {}
    pub fn pong_response(&mut self, _p: &ClientboundPongResponse) {}
    pub fn store_cookie(&mut self, _p: &ClientboundStoreCookie) {}
    pub fn transfer(&mut self, _p: &ClientboundTransfer) {}
    pub fn move_minecart_along_track(&mut self, _p: &ClientboundMoveMinecartAlongTrack) {}
    pub fn set_held_slot(&mut self, _p: &ClientboundSetHeldSlot) {}
    pub fn set_player_inventory(&mut self, _p: &ClientboundSetPlayerInventory) {}
    pub fn projectile_power(&mut self, _p: &ClientboundProjectilePower) {}
    pub fn custom_report_details(&mut self, _p: &ClientboundCustomReportDetails) {}
    pub fn server_links(&mut self, _p: &ClientboundServerLinks) {}
    pub fn player_rotation(&mut self, _p: &ClientboundPlayerRotation) {}
    pub fn recipe_book_add(&mut self, _p: &ClientboundRecipeBookAdd) {}
    pub fn recipe_book_remove(&mut self, _p: &ClientboundRecipeBookRemove) {}
    pub fn recipe_book_settings(&mut self, _p: &ClientboundRecipeBookSettings) {}
}
